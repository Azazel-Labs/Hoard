//! AI tagging on a ChatGPT plan, through OpenAI's "Sign in with ChatGPT" for open-source and locally hosted apps
//! (<https://developers.openai.com/siwc/token-sharing-open-source>). The server registers itself as an app on the
//! user's ChatGPT account, keeps the tokens in its database, and sends tagging requests to the Responses API, where
//! they count against the plan's included usage.
//!
//! Signing in: OpenAI only sends the browser back to `http://127.0.0.1:<port>/auth/callback`. When the browser is
//! on the server's own machine that's this server, which finishes the sign-in itself ([`callback`]). When it isn't
//! (a NAS), that page doesn't load, and the person pastes its address into Settings instead ([`finish`]).

use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow, bail};
use axum::Json;
use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::Digest as _;

use crate::AppState;
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::llm::ModelInfo;

const AUTHORIZE: &str = "https://auth.openai.com/api/accounts/authorize";
const TOKEN: &str = "https://auth.openai.com/api/accounts/oauth/token";
const DISCOVERY: &str = "https://auth.openai.com/.well-known/openid-configuration";
const ISSUER: &str = "https://auth.openai.com";
const API: &str = "https://api.openai.com/v1";
const SCOPES: &str = "openid profile email offline_access resource.invoke chatgpt.tokens.use.direct";
/// The scope that lets requests use the plan; signing in without it only identifies the account.
const PLAN_SCOPE: &str = "chatgpt.tokens.use.direct";
/// The name the app is registered under on the ChatGPT account (the same on every install).
const APP_NAME: &str = "Hoard";
/// Where people set a weekly limit for Hoard, or see what it has used.
pub(crate) const MANAGE_USAGE: &str = "https://chatgpt.com/settings/usage";
/// How long a sign-in started from Settings can be finished.
const SIGN_IN_TTL: Duration = Duration::from_mins(15);
/// Tokens are refreshed this long before they run out (they last an hour).
const REFRESH_AHEAD: i64 = 300;
const SETTINGS_KEY: &str = "chatgpt";

/// The ChatGPT account this server is registered with, kept in the database. The host ID is made once and kept
/// for good; the client ID is OpenAI's registration of this server on the account, reused for every later
/// sign-in; signing out clears only the tokens.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Account {
    host_id: String,
    client_id: Option<String>,
    email: Option<String>,
    sub: Option<String>,
    id_token: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    /// Unix seconds.
    expires_at: Option<i64>,
    scopes: Option<String>,
    /// Why tagging can't use the account right now, in words for Settings.
    problem: Option<String>,
}

impl Account {
    fn signed_in(&self) -> bool {
        self.refresh_token.is_some() || self.access_token.is_some()
    }

    fn clear_tokens(&mut self, problem: Option<String>) {
        self.id_token = None;
        self.access_token = None;
        self.refresh_token = None;
        self.expires_at = None;
        self.scopes = None;
        self.problem = problem;
    }
}

fn load(c: &rusqlite::Connection) -> Result<Account> {
    let text: Option<String> =
        c.query_row("SELECT value FROM settings WHERE key = ?1", [SETTINGS_KEY], |r| r.get(0)).optional()?;
    let mut account: Account = text.and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    if account.host_id.is_empty() {
        account.host_id = format!("urn:uuid:{}", uuid::Uuid::new_v4());
        save(c, &account)?;
    }
    Ok(account)
}

fn save(c: &rusqlite::Connection, account: &Account) -> Result<()> {
    c.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT (key) DO UPDATE SET value = excluded.value",
        [SETTINGS_KEY, &serde_json::to_string(account)?],
    )?;
    Ok(())
}

async fn account(state: &AppState) -> Result<Account> {
    db::call(&state.pool, |c| load(c)).await
}

async fn store(state: &AppState, account: Account) -> Result<()> {
    db::call(&state.pool, move |c| save(c, &account)).await
}

/// A sign-in started from Settings and not finished yet.
struct Pending {
    state: String,
    nonce: String,
    verifier: String,
    redirect_uri: String,
    started: Instant,
}

static PENDING: Mutex<Vec<Pending>> = Mutex::new(Vec::new());
/// One refresh at a time: the refresh token rotates, so two at once would spend it twice.
static REFRESHING: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn pending() -> std::sync::MutexGuard<'static, Vec<Pending>> {
    let mut list = PENDING.lock().unwrap_or_else(PoisonError::into_inner);
    list.retain(|p| p.started.elapsed() < SIGN_IN_TTL);
    list
}

/// 32 random bytes, base64url: for state, nonce and the PKCE verifier.
fn random() -> String {
    let mut bytes = [0u8; 32];
    bytes[..16].copy_from_slice(uuid::Uuid::new_v4().as_bytes());
    bytes[16..].copy_from_slice(uuid::Uuid::new_v4().as_bytes());
    URL_SAFE_NO_PAD.encode(bytes)
}

fn form(pairs: &[(&str, &str)]) -> String {
    let mut url = reqwest::Url::parse("http://form/").expect("static URL");
    url.query_pairs_mut().extend_pairs(pairs);
    url.query().unwrap_or_default().to_owned()
}

fn now() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}

/// The address to send the browser to, to sign in (or, for a server registered before, to sign in again).
fn authorize_url(account: &Account, p: &Pending) -> Result<String> {
    let challenge = URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(p.verifier.as_bytes()));
    let mut params = vec![
        ("response_type", "code"),
        ("redirect_uri", p.redirect_uri.as_str()),
        ("scope", SCOPES),
        ("resource", API),
        ("state", p.state.as_str()),
        ("nonce", p.nonce.as_str()),
        ("code_challenge_method", "S256"),
        ("code_challenge", challenge.as_str()),
        ("ext_agent_host_id", account.host_id.as_str()),
    ];
    match &account.client_id {
        Some(id) => {
            params.push(("client_id", id));
            if let Some(email) = &account.email {
                params.push(("login_hint", email));
            }
        }
        None => params.extend([("client_id", "dynamic_agent_client"), ("agent_name_hint", APP_NAME)]),
    }
    Ok(reqwest::Url::parse_with_params(AUTHORIZE, &params)?.to_string())
}

/// What OpenAI sends back to the redirect address.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct Callback {
    code: Option<String>,
    state: Option<String>,
    client_id: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Tokens {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    id_token: Option<String>,
    #[serde(default)]
    expires_in: Option<i64>,
    #[serde(default)]
    scope: Option<String>,
}

/// POST a form to the token endpoint. `Err(Ok(code))` is OpenAI refusing it, with its error code.
async fn token_request(state: &AppState, body: String) -> Result<Tokens, Result<String, anyhow::Error>> {
    let resp = state
        .http
        .post(TOKEN)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| Err(anyhow!(e).context(tr!("error-openai-auth-unreachable"))))?;
    let status = resp.status();
    let text = resp.text().await.map_err(|e| Err(e.into()))?;
    if status.is_success() {
        return serde_json::from_str(&text).map_err(|e| Err(anyhow!(e).context(tr!("error-openai-unexpected"))));
    }
    let v: Value = serde_json::from_str(&text).unwrap_or_default();
    match v["error"].as_str() {
        Some(code) => Err(Ok(code.to_owned())),
        None => Err(Err(anyhow!(tr!(
            "error-openai-answered",
            status = status.to_string(),
            reason = text.chars().take(300).collect::<String>()
        )))),
    }
}

/// The claims of an ID token. It comes straight from OpenAI's token endpoint over TLS, which OpenID Connect
/// accepts in place of checking its signature (Core 1.0, 3.1.3.7); the issuer, audience, expiry and nonce are
/// checked here.
fn id_claims(token: &str, client_id: &str, nonce: &str) -> Result<Value> {
    let payload = token.split('.').nth(1).with_context(|| tr!("error-chatgpt-id-token-not-jwt"))?;
    let claims: Value = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(payload.trim_end_matches('='))?)?;
    if claims["iss"].as_str().map(|i| i.trim_end_matches('/')) != Some(ISSUER) {
        bail!(tr!("error-chatgpt-id-token-issuer", issuer = claims["iss"].to_string(), expected = ISSUER));
    }
    let aud_ok = match &claims["aud"] {
        Value::String(a) => a == client_id,
        Value::Array(list) => list.iter().any(|a| a.as_str() == Some(client_id)),
        _ => false,
    };
    if !aud_ok {
        bail!(tr!("error-chatgpt-id-token-audience"));
    }
    if claims["exp"].as_i64().is_none_or(|exp| exp < now() - 60) {
        bail!(tr!("error-chatgpt-id-token-expired"));
    }
    if claims["nonce"].as_str() != Some(nonce) {
        bail!(tr!("error-chatgpt-id-token-nonce"));
    }
    Ok(claims)
}

/// Finish a sign-in: check what came back, swap the code for tokens, and keep them.
async fn complete(state: &AppState, cb: Callback) -> Result<Account> {
    let sent = cb.state.as_deref().with_context(|| tr!("error-chatgpt-no-state"))?;
    let p = {
        let mut list = pending();
        let i = list.iter().position(|p| p.state == sent).with_context(|| tr!("error-chatgpt-not-started"))?;
        list.remove(i)
    };
    if let Some(e) = &cb.error {
        if e == "access_denied" {
            bail!(tr!("error-chatgpt-cancelled"));
        }
        bail!(tr!("error-chatgpt-refused", reason = cb.error_description.as_deref().unwrap_or(e)));
    }
    let code = cb.code.with_context(|| tr!("error-chatgpt-no-code"))?;
    let mut account = account(state).await?;
    let client_id = match (&account.client_id, cb.client_id) {
        (Some(saved), Some(sent)) if *saved != sent => {
            bail!(tr!("error-chatgpt-other-registration"))
        }
        (Some(saved), _) => saved.clone(),
        (None, Some(sent)) => sent,
        (None, None) => bail!(tr!("error-chatgpt-not-registered")),
    };
    let body = form(&[
        ("grant_type", "authorization_code"),
        ("client_id", &client_id),
        ("code", &code),
        ("code_verifier", &p.verifier),
        ("redirect_uri", &p.redirect_uri),
        ("resource", API),
    ]);
    let tokens = match token_request(state, body).await {
        Ok(t) => t,
        Err(Ok(code)) if code == "invalid_grant" => {
            bail!(tr!("error-chatgpt-code-used"))
        }
        Err(Ok(code)) => bail!(tr!("error-chatgpt-refused-code", code = code)),
        Err(Err(e)) => return Err(e),
    };
    let id_token = tokens.id_token.as_deref().with_context(|| tr!("error-chatgpt-no-id-token"))?;
    let claims = id_claims(id_token, &client_id, &p.nonce)?;
    let plan = tokens.scope.as_deref().is_some_and(|s| s.split_whitespace().any(|s| s == PLAN_SCOPE));
    account.client_id = Some(client_id);
    account.sub = claims["sub"].as_str().map(str::to_owned);
    account.email = claims["email"].as_str().map(str::to_owned).or(account.email);
    account.id_token = Some(id_token.to_owned());
    account.access_token = Some(tokens.access_token);
    account.refresh_token = tokens.refresh_token;
    account.expires_at = Some(now() + tokens.expires_in.unwrap_or(3600));
    account.scopes = tokens.scope;
    account.problem = (!plan).then(|| tr!("error-chatgpt-plan-not-allowed"));
    store(state, account.clone()).await?;
    tracing::info!(email = account.email.as_deref().unwrap_or("?"), "signed in to ChatGPT");
    Ok(account)
}

/// A current access token, refreshed first when it's about to run out. `force` refreshes it regardless (after the
/// API turned it down).
async fn access_token(state: &AppState, force: bool) -> Result<String> {
    let _one = REFRESHING.lock().await;
    let mut account = account(state).await?;
    if let Some(problem) = &account.problem {
        bail!(tr!("error-chatgpt-problem", problem = problem));
    }
    let fresh = account.expires_at.is_some_and(|t| t - REFRESH_AHEAD > now());
    if let (Some(token), true, false) = (&account.access_token, fresh, force) {
        return Ok(token.clone());
    }
    let (Some(client_id), Some(refresh)) = (account.client_id.clone(), account.refresh_token.clone()) else {
        bail!(tr!("error-chatgpt-not-signed-in"));
    };
    let body = form(&[
        ("grant_type", "refresh_token"),
        ("client_id", &client_id),
        ("refresh_token", &refresh),
        ("resource", API),
    ]);
    match token_request(state, body).await {
        Ok(t) => {
            account.access_token = Some(t.access_token.clone());
            account.refresh_token = t.refresh_token.or(account.refresh_token);
            account.expires_at = Some(now() + t.expires_in.unwrap_or(3600));
            if t.scope.is_some() {
                account.scopes = t.scope;
            }
            if t.id_token.is_some() {
                account.id_token = t.id_token;
            }
            store(state, account).await?;
            Ok(t.access_token)
        }
        // OpenAI having a moment: keep the tokens and try again next time.
        Err(Ok(code)) if matches!(code.as_str(), "server_error" | "temporarily_unavailable" | "slow_down") => {
            bail!(tr!("error-chatgpt-refresh-later", code = code))
        }
        // A refresh token that's no good any more (revoked, expired, disconnected in ChatGPT): sign in again.
        Err(Ok(code)) => {
            account.clear_tokens(Some(tr!("error-chatgpt-signed-out-problem", code = code)));
            store(state, account).await?;
            bail!(tr!("error-chatgpt-signed-out", code = code))
        }
        // A network problem: keep the tokens and try again next time.
        Err(Err(e)) => Err(e),
    }
}

/// Send one request to the Responses API and return the reply's text. `messages` is the chat-completions form the
/// tagger builds (a system message and a user message of text and image parts).
pub(crate) async fn chat(state: &AppState, model: &str, messages: &Value) -> Result<String> {
    let mut instructions = String::new();
    let mut input = Vec::new();
    for m in messages.as_array().into_iter().flatten() {
        let role = m["role"].as_str().unwrap_or("user");
        if role == "system" {
            instructions.push_str(m["content"].as_str().unwrap_or_default());
            continue;
        }
        let content: Vec<Value> = match &m["content"] {
            Value::String(t) => vec![json!({ "type": "input_text", "text": t })],
            Value::Array(parts) => parts
                .iter()
                .filter_map(|p| match p["type"].as_str()? {
                    "text" => Some(json!({ "type": "input_text", "text": p["text"] })),
                    "image_url" => Some(json!({ "type": "input_image", "image_url": p["image_url"]["url"] })),
                    _ => None,
                })
                .collect(),
            _ => continue,
        };
        input.push(json!({ "role": role, "content": content }));
    }
    let body = json!({
        "model": model,
        "instructions": instructions,
        "input": input,
        "store": false,
        "stream": true,
    });
    for retry in [false, true] {
        let token = access_token(state, retry).await?;
        let resp = state
            .http
            .post(format!("{API}/responses"))
            .bearer_auth(&token)
            .json(&body)
            .timeout(Duration::from_mins(5))
            .send()
            .await
            .with_context(|| tr!("error-openai-api-unreachable"))?;
        let status = resp.status();
        let text = resp.text().await.with_context(|| tr!("error-openai-broke-off"))?;
        // Once, a turned-down token gets refreshed and the request sent again.
        if status == reqwest::StatusCode::UNAUTHORIZED && !retry {
            continue;
        }
        if !status.is_success() {
            let v: Value = serde_json::from_str(&text).unwrap_or_default();
            bail!(explain(&v["error"], v["detail"].as_str().unwrap_or(&text), status.as_u16()));
        }
        return read_stream(&text);
    }
    bail!(tr!("error-chatgpt-kept-refusing"))
}

/// The reply's text from a Responses event stream, or why there isn't one.
fn read_stream(sse: &str) -> Result<String> {
    let mut out = String::new();
    for line in sse.lines() {
        let Some(data) = line.strip_prefix("data:").map(str::trim) else { continue };
        let Ok(event) = serde_json::from_str::<Value>(data) else { continue };
        match event["type"].as_str().unwrap_or_default() {
            "response.output_text.delta" => out.push_str(event["delta"].as_str().unwrap_or_default()),
            "response.completed" => return Ok(out),
            "response.failed" => bail!(explain(&event["response"]["error"], &tr!("error-chatgpt-request-failed"), 0)),
            "response.incomplete" => match event["response"]["incomplete_details"]["reason"].as_str() {
                Some(reason) => bail!(tr!("error-chatgpt-cut-off", reason = reason)),
                None => bail!(tr!("error-chatgpt-cut-off-no-reason")),
            },
            "error" => bail!(explain(&event["error"], &tr!("error-chatgpt-request-failed"), 0)),
            _ => {}
        }
    }
    bail!(tr!("error-openai-ended-early"))
}

/// An OpenAI error in words, with what to do about it for the ones the docs single out.
fn explain(error: &Value, fallback: &str, status: u16) -> String {
    let code = error["code"].as_str().unwrap_or_default();
    let message = error["message"].as_str().unwrap_or(fallback);
    match code {
        "subscription_sharing_usage_limit_exceeded" => tr!("error-chatgpt-usage-limit", url = MANAGE_USAGE),
        "subscription_sharing_user_not_eligible" => tr!("error-chatgpt-not-eligible"),
        "subscription_sharing_usage_unavailable" | "subscription_sharing_user_unavailable" => {
            tr!("error-chatgpt-usage-unavailable")
        }
        "subscription_sharing_unsupported_capability" => match error["param"].as_str() {
            Some(param) if param.contains("image") => tr!("error-chatgpt-unsupported-images", param = param),
            Some(param) => tr!("error-chatgpt-unsupported", param = param),
            None => tr!("error-chatgpt-unsupported-part"),
        },
        "" if status != 0 => tr!(
            "error-openai-answered",
            status = status.to_string(),
            reason = message.chars().take(300).collect::<String>()
        ),
        "" => message.to_owned(),
        _ => format!("{message} ({code})"),
    }
}

/// The models the signed-in plan can use, in OpenAI's order.
async fn models(state: &AppState) -> Result<Vec<ModelInfo>> {
    let token = access_token(state, false).await?;
    let resp = state
        .http
        .get(format!("{API}/models"))
        .bearer_auth(token)
        .timeout(Duration::from_secs(20))
        .send()
        .await
        .with_context(|| tr!("error-openai-api-unreachable"))?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        let v: Value = serde_json::from_str(&text).unwrap_or_default();
        bail!(explain(&v["error"], v["detail"].as_str().unwrap_or(&text), status.as_u16()));
    }
    Ok(parse_models(&serde_json::from_str(&text)?))
}

fn parse_models(v: &Value) -> Vec<ModelInfo> {
    let list = v["models"].as_array().or_else(|| v["data"].as_array()).cloned().unwrap_or_default();
    list.iter()
        .filter(|m| m["visibility"].as_str().is_none_or(|vis| vis == "list"))
        .filter_map(|m| {
            let id = m["slug"].as_str().or_else(|| m["id"].as_str())?.to_owned();
            let name = m["display_name"].as_str().map(str::to_owned);
            let vision = m["input_modalities"].as_array().map(|mods| mods.iter().any(|x| x.as_str() == Some("image")));
            Some(ModelInfo { id, name, vision })
        })
        .collect()
}

/// Tell OpenAI to forget the refresh token, then forget the tokens here. The registration (client and host IDs)
/// is kept, so signing in again reuses it.
async fn sign_out(state: &AppState) -> Result<Option<String>> {
    let _one = REFRESHING.lock().await;
    let mut account = account(state).await?;
    let mut note = None;
    if let (Some(token), Some(client_id)) = (&account.refresh_token, &account.client_id) {
        let revoked = async {
            let doc: Value = state.http.get(DISCOVERY).send().await?.error_for_status()?.json().await?;
            let url = doc["revocation_endpoint"].as_str().context("no revocation endpoint")?;
            let body = form(&[("token", token), ("token_type_hint", "refresh_token"), ("client_id", client_id)]);
            state
                .http
                .post(url)
                .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(body)
                .timeout(Duration::from_secs(20))
                .send()
                .await?
                .error_for_status()?;
            anyhow::Ok(())
        }
        .await;
        if let Err(e) = revoked {
            tracing::warn!("couldn't revoke the ChatGPT sign-in: {e:#}");
            note = Some(tr!("server-chatgpt-revoke-failed"));
        }
    }
    account.clear_tokens(None);
    store(state, account).await?;
    Ok(note)
}

fn status_json(account: &Account) -> Value {
    json!({
        "signed_in": account.signed_in(),
        "email": account.email,
        "problem": account.problem,
        "waiting": !pending().is_empty(),
        "manage_url": MANAGE_USAGE,
    })
}

/// `GET /api/admin/chatgpt`: whether the server is signed in, and as whom.
pub(crate) async fn get_status(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    Ok(Json(status_json(&account(&state).await?)))
}

#[derive(Deserialize)]
pub(crate) struct StartBody {
    /// The port the browser reaches Hoard on, so a browser on the server's own machine comes straight back.
    port: u16,
}

/// `POST /api/admin/chatgpt/start`: the address to open to sign in.
pub(crate) async fn start(State(state): State<AppState>, Json(body): Json<StartBody>) -> ApiResult<Json<Value>> {
    if body.port == 0 {
        return Err(AppError::bad_request(tr!("error-chatgpt-port")));
    }
    let account = account(&state).await?;
    let p = Pending {
        state: random(),
        nonce: random(),
        verifier: random(),
        redirect_uri: format!("http://127.0.0.1:{}/auth/callback", body.port),
        started: Instant::now(),
    };
    let url = authorize_url(&account, &p)?;
    let mut list = pending();
    // Only the latest few: each "Continue with ChatGPT" starts over.
    if list.len() >= 4 {
        list.remove(0);
    }
    list.push(p);
    Ok(Json(json!({ "url": url })))
}

#[derive(Deserialize)]
pub(crate) struct FinishBody {
    /// The address the browser was sent to after signing in, pasted from its address bar.
    url: String,
}

/// `POST /api/admin/chatgpt/finish`: finish a sign-in from the address pasted in Settings.
pub(crate) async fn finish(State(state): State<AppState>, Json(body): Json<FinishBody>) -> ApiResult<Json<Value>> {
    let url =
        reqwest::Url::parse(body.url.trim()).map_err(|_| AppError::bad_request(tr!("error-chatgpt-paste-address")))?;
    let cb: Callback = serde_json::from_value(Value::Object(
        url.query_pairs().map(|(k, v)| (k.into_owned(), Value::String(v.into_owned()))).collect(),
    ))
    .map_err(AppError::bad_request)?;
    let account = complete(&state, cb).await.map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    Ok(Json(status_json(&account)))
}

/// `POST /api/admin/chatgpt/sign-out`.
pub(crate) async fn sign_out_route(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let note = sign_out(&state).await?;
    let mut v = status_json(&account(&state).await?);
    v["note"] = json!(note);
    Ok(Json(v))
}

/// `GET /api/admin/chatgpt/models`.
pub(crate) async fn list_models(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let models = models(&state).await.map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    Ok(Json(json!({ "models": models })))
}

/// `GET /auth/callback`: where OpenAI sends a browser on the server's own machine after signing in. No API token:
/// the one-time state, which only Settings has seen, is what ties it to a sign-in.
pub(crate) async fn callback(State(state): State<AppState>, Query(cb): Query<Callback>) -> impl IntoResponse {
    let back = html_escape(&tr!("server-back-to-settings"));
    let (title, body) = match complete(&state, cb).await {
        Ok(a) => (
            html_escape(&tr!("server-chatgpt-signed-in-title")),
            format!(
                "<p>{}</p><p><a href=\"/#settings/ai\">{back}</a></p>",
                html_escape(&match a.email {
                    Some(email) => tr!("server-chatgpt-signed-in-as", email = email),
                    None => tr!("server-chatgpt-signed-in"),
                })
            ),
        ),
        Err(e) => (
            html_escape(&tr!("server-chatgpt-not-finished-title")),
            format!(
                "<p>{}</p><p>{}</p><p><a href=\"/#settings/ai\">{back}</a></p>",
                html_escape(&sentence(&format!("{e:#}"))),
                html_escape(&tr!("server-chatgpt-other-computer"))
            ),
        ),
    };
    Html(format!(
        "<!doctype html><meta charset=utf-8><meta name=viewport content=\"width=device-width\"><title>{title}</title>\
         <style>body{{font:16px/1.5 system-ui,sans-serif;max-width:34rem;margin:15vh auto;padding:0 16px}}\
         @media(prefers-color-scheme:dark){{body{{background:#16181c;color:#e6e6e6}}a{{color:#8ab4ff}}}}</style>\
         <h1>{title}</h1>{body}"
    ))
}

/// An error message as a sentence, for a page of its own.
fn sentence(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jwt(claims: &Value) -> String {
        format!("e30.{}.sig", URL_SAFE_NO_PAD.encode(claims.to_string()))
    }

    #[test]
    fn id_tokens_are_checked() {
        let exp = now() + 600;
        let good = json!({ "iss": ISSUER, "aud": "oaiapp_1", "exp": exp, "nonce": "n", "sub": "u", "email": "a@b" });
        assert_eq!(id_claims(&jwt(&good), "oaiapp_1", "n").unwrap()["sub"], "u");
        let list_aud = json!({ "iss": "https://auth.openai.com/", "aud": ["x", "oaiapp_1"], "exp": exp, "nonce": "n" });
        assert!(id_claims(&jwt(&list_aud), "oaiapp_1", "n").is_ok());
        for (bad, why) in [
            (json!({ "iss": "https://evil", "aud": "oaiapp_1", "exp": exp, "nonce": "n" }), "issuer"),
            (json!({ "iss": ISSUER, "aud": "other", "exp": exp, "nonce": "n" }), "audience"),
            (json!({ "iss": ISSUER, "aud": "oaiapp_1", "exp": 1, "nonce": "n" }), "expiry"),
            (json!({ "iss": ISSUER, "aud": "oaiapp_1", "exp": exp, "nonce": "other" }), "nonce"),
        ] {
            assert!(id_claims(&jwt(&bad), "oaiapp_1", "n").is_err(), "{why}");
        }
    }

    #[test]
    fn reads_the_reply_out_of_the_event_stream() {
        let sse = "event: response.created\ndata: {\"type\":\"response.created\"}\n\n\
                   data: {\"type\":\"response.output_text.delta\",\"delta\":\"{\\\"a\\\":\"}\n\n\
                   data: {\"type\":\"response.output_text.delta\",\"delta\":\"1}\"}\n\n\
                   data: {\"type\":\"response.completed\",\"response\":{}}\n\n";
        assert_eq!(read_stream(sse).unwrap(), "{\"a\":1}");

        let limit = "data: {\"type\":\"response.failed\",\"response\":{\"error\":{\"code\":\"subscription_sharing_usage_limit_exceeded\",\"message\":\"x\"}}}\n";
        assert!(read_stream(limit).unwrap_err().to_string().contains(MANAGE_USAGE));
        assert!(
            read_stream("data: {\"type\":\"response.output_text.delta\",\"delta\":\"x\"}\n").is_err(),
            "no completion"
        );
    }

    #[test]
    fn sign_in_addresses_register_once_then_reuse_the_registration() {
        let p = Pending {
            state: "s".into(),
            nonce: "n".into(),
            verifier: "v".into(),
            redirect_uri: "http://127.0.0.1:4627/auth/callback".into(),
            started: Instant::now(),
        };
        let mut account = Account { host_id: "urn:uuid:h".into(), ..Account::default() };
        let first = reqwest::Url::parse(&authorize_url(&account, &p).unwrap()).unwrap();
        let q: std::collections::HashMap<_, _> = first.query_pairs().into_owned().collect();
        assert_eq!(q["client_id"], "dynamic_agent_client");
        assert_eq!(q["agent_name_hint"], APP_NAME);
        assert_eq!(q["redirect_uri"], "http://127.0.0.1:4627/auth/callback");
        assert_eq!(q["code_challenge"], URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(b"v")));
        assert!(q["scope"].contains(PLAN_SCOPE));

        account.client_id = Some("oaiapp_1".into());
        account.email = Some("a@b".into());
        let again = reqwest::Url::parse(&authorize_url(&account, &p).unwrap()).unwrap();
        let q: std::collections::HashMap<_, _> = again.query_pairs().into_owned().collect();
        assert_eq!(q["client_id"], "oaiapp_1");
        assert_eq!(q["login_hint"], "a@b");
        assert!(!q.contains_key("agent_name_hint"));
    }

    #[test]
    fn lists_only_the_models_meant_to_be_shown() {
        let v = json!({ "models": [
            { "slug": "b-model", "display_name": "B", "visibility": "list", "input_modalities": ["text", "image"] },
            { "slug": "hidden", "visibility": "hide" },
            { "slug": "a-model", "display_name": "A", "visibility": "list" },
        ]});
        let m = parse_models(&v);
        assert_eq!(m.iter().map(|m| m.id.as_str()).collect::<Vec<_>>(), ["b-model", "a-model"], "OpenAI's order");
        assert_eq!(m[0].vision, Some(true));
        assert_eq!(m[1].vision, None);
    }
}
