//! OAuth: the trip to the provider and back (authorization code with PKCE), and who the provider says signed in.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, bail};
use axum::Router;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use base64::Engine as _;
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::Digest as _;

use super::accounts::{Source, account_for};
use super::{Method, OAuth};
use crate::AppState;
use crate::db;
use crate::i18n::tr;

/// How long someone has to finish signing in at the OAuth provider.
const OAUTH_WAIT: Duration = Duration::from_secs(600);
const OAUTH_COOKIE: &str = "hoard_oauth";
const CALLBACK: &str = "/api/auth/oauth/callback";

struct Pending {
    verifier: String,
    redirect: String,
    since: Instant,
}

/// Sign-ins started and not yet back from the provider, by `state`.
static PENDING: LazyLock<Mutex<HashMap<String, Pending>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn random() -> Result<String> {
    Ok(b64(&crate::auth::random_bytes::<32>()?))
}

/// Where the provider sends people back to. Give the provider this address.
pub(super) fn callback_url(state: &AppState, headers: &HeaderMap) -> String {
    format!("{}{CALLBACK}", crate::mcp::base_url(state, headers))
}

fn state_cookie(headers: &HeaderMap, value: &str, max_age: u64) -> Result<HeaderValue> {
    let secure = if crate::auth::over_https(headers) { "; Secure" } else { "" };
    Ok(HeaderValue::from_str(&format!(
        "{OAUTH_COOKIE}={value}; Path=/api/auth/oauth; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}"
    ))?)
}

/// Back to the portal, showing why signing in didn't work.
fn back_with_error(headers: &HeaderMap, msg: &str) -> Response {
    tracing::info!(reason = msg, "OAuth sign-in failed");
    let mut url = reqwest::Url::parse("http://hoard/").expect("valid");
    url.query_pairs_mut().append_pair("signin_error", msg);
    let location = format!("/?{}", url.query().unwrap_or_default());
    let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, location)]).into_response();
    if let Ok(clear) = state_cookie(headers, "", 0) {
        res.headers_mut().append(header::SET_COOKIE, clear);
    }
    res
}

/// Off to the provider's sign-in page.
async fn oauth_start(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let cfg = state.settings.get().auth.clone();
    if cfg.method != Method::Oauth {
        return back_with_error(&headers, &tr!("error-oauth-off"));
    }
    match start_url(&state, &headers, &cfg.oauth) {
        Ok((location, st)) => {
            let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, location)]).into_response();
            if let Ok(c) = state_cookie(&headers, &st, OAUTH_WAIT.as_secs()) {
                res.headers_mut().append(header::SET_COOKIE, c);
            }
            res
        }
        Err(e) => back_with_error(&headers, &format!("{e:#}")),
    }
}

fn start_url(state: &AppState, headers: &HeaderMap, o: &OAuth) -> Result<(String, String)> {
    let st = random()?;
    let verifier = random()?;
    let challenge = b64(&sha2::Sha256::digest(verifier.as_bytes()));
    let redirect = callback_url(state, headers);
    let mut url = reqwest::Url::parse(o.authorization_url.trim())
        .with_context(|| tr!("error-oauth-bad-authorization-address"))?;
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", o.client_id.trim())
        .append_pair("redirect_uri", &redirect)
        .append_pair("scope", o.scopes.trim())
        .append_pair("state", &st)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256");
    let mut pending = PENDING.lock().unwrap_or_else(PoisonError::into_inner);
    pending.retain(|_, p| p.since.elapsed() < OAUTH_WAIT);
    pending.insert(st.clone(), Pending { verifier, redirect, since: Instant::now() });
    Ok((url.into(), st))
}

#[derive(Deserialize)]
struct Callback {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

/// Back from the provider: trade the code for tokens, find out who it is, and sign them in.
async fn oauth_callback(State(state): State<AppState>, headers: HeaderMap, Query(q): Query<Callback>) -> Response {
    if let Some(err) = q.error {
        let detail = q.error_description.unwrap_or(err);
        return back_with_error(&headers, &tr!("error-oauth-refused", reason = detail));
    }
    let (Some(code), Some(st)) = (q.code, q.state) else {
        return back_with_error(&headers, &tr!("error-oauth-no-code"));
    };
    let same_browser = crate::auth::cookie_value(&headers, OAUTH_COOKIE).is_some_and(|c| c == st);
    let pending = PENDING.lock().unwrap_or_else(PoisonError::into_inner).remove(&st);
    let Some(pending) = pending.filter(|p| same_browser && p.since.elapsed() < OAUTH_WAIT) else {
        return back_with_error(&headers, &tr!("error-oauth-expired"));
    };
    let cfg = state.settings.get().auth.clone();
    if cfg.method != Method::Oauth {
        return back_with_error(&headers, &tr!("error-oauth-off"));
    }
    let claims = match who_signed_in(&state.http, &cfg.oauth, &code, &pending).await {
        Ok(c) => c,
        Err(e) => return back_with_error(&headers, &format!("{e:#}")),
    };
    let Some(name) = claim_string(&claims, cfg.oauth.username_claim.trim()) else {
        return back_with_error(&headers, &tr!("error-oauth-no-claim", claim = cfg.oauth.username_claim.trim()));
    };
    let Some(sub) = claim_string(&claims, "sub").or_else(|| claim_string(&claims, "id")) else {
        return back_with_error(&headers, &tr!("error-oauth-no-sub"));
    };
    let issuer = claim_string(&claims, "iss").unwrap_or_else(|| cfg.oauth.token_url.trim().to_owned());
    let subject = format!("{issuer} {sub}");
    let groups = claim_strings(&claims, cfg.oauth.groups_claim.trim());
    let made = db::call(&state.pool, move |c| {
        Ok(match account_for(c, &cfg, Source::Oauth, &name, Some(&subject), &groups)? {
            Ok((id, username, _)) => Ok((crate::auth::new_session(c, id)?, username)),
            Err(msg) => Err(msg),
        })
    })
    .await;
    match made {
        Ok(Ok((token, username))) => {
            tracing::info!(user = %username, "signed in with OAuth");
            let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, "/")]).into_response();
            if let Ok(c) = crate::auth::session_cookie(&headers, &token) {
                res.headers_mut().append(header::SET_COOKIE, c);
            }
            if let Ok(c) = state_cookie(&headers, "", 0) {
                res.headers_mut().append(header::SET_COOKIE, c);
            }
            res
        }
        Ok(Err(msg)) => back_with_error(&headers, &msg),
        Err(e) => back_with_error(&headers, &format!("{e:#}")),
    }
}

/// The claims about who signed in: the user-info answer over the ID token's. (The ID token came straight from the
/// token endpoint over TLS, which OpenID Connect accepts in place of checking its signature.)
async fn who_signed_in(http: &reqwest::Client, o: &OAuth, code: &str, pending: &Pending) -> Result<Value> {
    let mut form = reqwest::Url::parse("http://hoard/").expect("valid");
    form.query_pairs_mut()
        .append_pair("grant_type", "authorization_code")
        .append_pair("code", code)
        .append_pair("redirect_uri", &pending.redirect)
        .append_pair("client_id", o.client_id.trim())
        .append_pair("client_secret", &o.client_secret.0)
        .append_pair("code_verifier", &pending.verifier);
    let res = http
        .post(o.token_url.trim())
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .timeout(Duration::from_secs(30))
        .body(form.query().unwrap_or_default().to_owned())
        .send()
        .await
        .with_context(|| tr!("error-oauth-token-unreachable"))?;
    let status = res.status();
    let body: Value = res.json().await.unwrap_or(Value::Null);
    if !status.is_success() || body.get("error").is_some() {
        let why = body
            .get("error_description")
            .or_else(|| body.get("error"))
            .and_then(Value::as_str)
            .map_or_else(|| status.to_string(), str::to_owned);
        bail!(tr!("error-oauth-token-refused", reason = why));
    }
    let mut claims = body.get("id_token").and_then(Value::as_str).and_then(jwt_claims).unwrap_or_else(|| json!({}));
    if !o.userinfo_url.trim().is_empty() {
        let token =
            body.get("access_token").and_then(Value::as_str).with_context(|| tr!("error-oauth-no-access-token"))?;
        let res = http
            .get(o.userinfo_url.trim())
            .bearer_auth(token)
            .header(header::ACCEPT, "application/json")
            .header(header::USER_AGENT, hoard_core::user_agent())
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .with_context(|| tr!("error-oauth-userinfo-unreachable"))?;
        if !res.status().is_success() {
            bail!(tr!("error-oauth-userinfo-refused", status = res.status().to_string()));
        }
        let info: Value = res.json().await.with_context(|| tr!("error-oauth-userinfo-not-json"))?;
        if let (Some(all), Some(more)) = (claims.as_object_mut(), info.as_object()) {
            all.extend(more.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
    }
    Ok(claims)
}

/// A JWT's claims, unchecked.
fn jwt_claims(jwt: &str) -> Option<Value> {
    let payload = jwt.split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload.trim_end_matches('=')).ok()?;
    serde_json::from_slice(&bytes).ok().filter(Value::is_object)
}

fn claim<'a>(claims: &'a Value, path: &str) -> Option<&'a Value> {
    if path.is_empty() {
        return None;
    }
    claims.get(path).or_else(|| path.split('.').try_fold(claims, |v, k| v.get(k)))
}

fn claim_string(claims: &Value, path: &str) -> Option<String> {
    match claim(claims, path)? {
        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_owned()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn claim_strings(claims: &Value, path: &str) -> Vec<String> {
    match claim(claims, path) {
        Some(Value::Array(a)) => a.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect(),
        Some(Value::String(s)) => s.split([',', ' ']).filter(|s| !s.is_empty()).map(str::to_owned).collect(),
        _ => Vec::new(),
    }
}

/// Open to everyone: the trip to the OAuth provider and back.
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/api/auth/oauth/start", get(oauth_start)).route(CALLBACK, get(oauth_callback))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claims() {
        let c = json!({ "preferred_username": "ada", "id": 42, "realm_access": { "roles": ["admins", "x"] }, "groups": "a, b" });
        assert_eq!(claim_string(&c, "preferred_username").as_deref(), Some("ada"));
        assert_eq!(claim_string(&c, "id").as_deref(), Some("42"));
        assert_eq!(claim_strings(&c, "realm_access.roles"), ["admins", "x"]);
        assert_eq!(claim_strings(&c, "groups"), ["a", "b"]);
        let jwt = format!("x.{}.y", b64(br#"{"sub":"1","email":"a@b.c"}"#));
        assert_eq!(jwt_claims(&jwt).unwrap()["email"], "a@b.c");
    }
}
