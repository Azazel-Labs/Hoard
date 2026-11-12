//! OAuth 2.1 sign-in for MCP clients, so Claude (claude.ai, Claude Desktop, Cowork, Claude Code) and other apps can
//! connect to `/mcp` by signing in to Hoard instead of being handed an access token.
//!
//! The flow, as MCP clients expect it:
//! 1. A request to `/mcp` without a valid token gets 401 with `WWW-Authenticate: Bearer resource_metadata="…"`.
//! 2. The client reads that protected-resource metadata (RFC 9728), then this server's authorization-server metadata
//!    (RFC 8414), and registers itself (dynamic client registration, RFC 7591).
//! 3. It sends the person's browser to `/oauth/authorize`, which is a page of the portal (`ConnectApp.vue`): sign in
//!    if need be, then allow or cancel. Allowing hands the browser back to the app with a one-time code.
//! 4. The app trades the code and its PKCE verifier for an access token (good for an hour) and a refresh token at
//!    `/oauth/token`, and refreshes from then on. Refresh tokens rotate; a connection unused for 90 days ends.
//!
//! A connected app acts for the person who allowed it and can do what an access token can: everything except manage
//! accounts and tokens. Settings → Account lists connected apps and disconnects them.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration, Instant};

use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Form, Json, Router};
use base64::Engine as _;
use rusqlite::{Connection, OptionalExtension as _, params};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::Digest as _;

use crate::auth::{Who, digest, random_bytes};
use crate::error::{ApiResult, AppError};
use crate::{AppState, db, tools};

const ACCESS_SECS: i64 = 3600;
const IDLE_DAYS: i64 = 90;
const CODE_LIFETIME: Duration = Duration::from_secs(600);
/// Registered apps kept before unused ones are dropped (anyone can register one).
const MAX_CLIENTS: i64 = 500;

/// Open to everyone: discovery, registration and the token endpoint.
pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/.well-known/oauth-protected-resource", get(resource_metadata))
        .route("/.well-known/oauth-protected-resource/mcp", get(resource_metadata))
        .route("/.well-known/oauth-authorization-server", get(server_metadata))
        .route("/oauth/register", post(register))
        .route("/oauth/token", post(token))
}

/// Behind `require_user`: the portal's consent page and Settings → Account.
pub(crate) fn portal_router() -> Router<AppState> {
    Router::new()
        .route("/api/oauth/request", get(describe_request))
        .route("/api/oauth/approve", post(approve))
        .route("/api/admin/connections", get(list_connections))
        .route("/api/admin/connections/{id}", axum::routing::delete(disconnect))
}

/// `WWW-Authenticate` for a 401 from `/mcp`: where to find out how to sign in.
pub(crate) fn challenge(state: &AppState, headers: &HeaderMap, bad_token: bool) -> HeaderValue {
    let base = crate::mcp::base_url(state, headers);
    let error = if bad_token { r#", error="invalid_token""# } else { "" };
    HeaderValue::from_str(&format!(
        r#"Bearer resource_metadata="{base}/.well-known/oauth-protected-resource/mcp", scope="hoard"{error}"#
    ))
    .unwrap_or_else(|_| HeaderValue::from_static("Bearer"))
}

/// Who an OAuth access token (by its hash) belongs to, if it's live.
pub(crate) fn lookup(c: &Connection, hash: &str) -> rusqlite::Result<Option<Who>> {
    let row: Option<(i64, String, String)> = c
        .query_row(
            "SELECT g.id, cl.name, u.username FROM oauth_grants g
               JOIN oauth_clients cl ON cl.id = g.client_id JOIN users u ON u.id = g.user_id
              WHERE g.access_hash = ?1 AND g.access_expires_at > unixepoch('now')",
            [hash],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .optional()?;
    if let Some((id, ..)) = &row {
        c.execute(
            "UPDATE oauth_grants SET last_used_at = ?2
              WHERE id = ?1 AND (last_used_at IS NULL OR unixepoch(last_used_at) < unixepoch('now') - 60)",
            params![id, db::now()],
        )?;
    }
    Ok(row.map(|(_, client, user)| Who::Token { name: format!("{client} for {user}") }))
}

// ---------------------------------------------------------------------------------------------
// Discovery

async fn resource_metadata(State(state): State<AppState>, headers: HeaderMap) -> Json<Value> {
    let base = crate::mcp::base_url(&state, &headers);
    Json(json!({
        "resource": format!("{base}/mcp"),
        "authorization_servers": [base],
        "bearer_methods_supported": ["header"],
        "scopes_supported": ["hoard"],
        "resource_name": "Hoard",
    }))
}

async fn server_metadata(State(state): State<AppState>, headers: HeaderMap) -> Json<Value> {
    let base = crate::mcp::base_url(&state, &headers);
    Json(json!({
        "issuer": base,
        "authorization_endpoint": format!("{base}/oauth/authorize"),
        "token_endpoint": format!("{base}/oauth/token"),
        "registration_endpoint": format!("{base}/oauth/register"),
        "response_types_supported": ["code"],
        "response_modes_supported": ["query"],
        "grant_types_supported": ["authorization_code", "refresh_token"],
        "code_challenge_methods_supported": ["S256"],
        "token_endpoint_auth_methods_supported": ["none", "client_secret_post", "client_secret_basic"],
        "scopes_supported": ["hoard"],
        "authorization_response_iss_parameter_supported": true,
    }))
}

// ---------------------------------------------------------------------------------------------
// Errors in OAuth's own shape

struct OAuthError {
    status: StatusCode,
    code: &'static str,
    desc: String,
}

impl OAuthError {
    fn new(status: StatusCode, code: &'static str, desc: impl Into<String>) -> Self {
        Self { status, code, desc: desc.into() }
    }
    fn grant(desc: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "invalid_grant", desc)
    }
    fn client() -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "invalid_client", "unknown app, or wrong client secret")
    }
}

impl<E: Into<anyhow::Error>> From<E> for OAuthError {
    fn from(e: E) -> Self {
        tracing::error!(error = ?e.into(), "OAuth request failed");
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "server_error", "something went wrong on the server")
    }
}

impl IntoResponse for OAuthError {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": self.code, "error_description": self.desc }));
        (self.status, [(header::CACHE_CONTROL, "no-store")], body).into_response()
    }
}

// ---------------------------------------------------------------------------------------------
// Registration

#[derive(Deserialize)]
struct Registration {
    #[serde(default)]
    redirect_uris: Vec<String>,
    client_name: Option<String>,
    token_endpoint_auth_method: Option<String>,
}

async fn register(State(state): State<AppState>, body: Result<Json<Registration>, JsonRejection>) -> Response {
    match register_client(&state, body).await {
        Ok(v) => (StatusCode::CREATED, [(header::CACHE_CONTROL, "no-store")], Json(v)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn register_client(
    state: &AppState,
    body: Result<Json<Registration>, JsonRejection>,
) -> Result<Value, OAuthError> {
    let bad = |code, desc: &str| OAuthError::new(StatusCode::BAD_REQUEST, code, desc);
    let Ok(Json(reg)) = body else { return Err(bad("invalid_client_metadata", "expected a JSON registration")) };
    if reg.redirect_uris.is_empty() || reg.redirect_uris.len() > 10 || !reg.redirect_uris.iter().all(|u| allowed(u)) {
        return Err(bad("invalid_redirect_uri", "redirect URIs must be https://, or http:// to localhost"));
    }
    let method = match reg.token_endpoint_auth_method.as_deref() {
        None | Some("none") => "none",
        Some(m @ ("client_secret_post" | "client_secret_basic")) => m,
        Some(_) => {
            return Err(bad(
                "invalid_client_metadata",
                "token_endpoint_auth_method must be none, client_secret_post or client_secret_basic",
            ));
        }
    };
    let name: String =
        reg.client_name.unwrap_or_default().trim().chars().filter(|c| !c.is_control()).take(100).collect();
    let name = if name.is_empty() { "An MCP app".to_owned() } else { name };
    let id = tools::hex(&random_bytes::<16>()?);
    let secret = if method == "none" { None } else { Some(format!("hoard_cs_{}", tools::hex(&random_bytes::<32>()?))) };
    let (row_id, row_name, uris, secret_hash) =
        (id.clone(), name.clone(), serde_json::to_string(&reg.redirect_uris)?, secret.as_deref().map(digest));
    let added = db::call(&state.pool, move |c| {
        let n: i64 = c.query_row("SELECT COUNT(*) FROM oauth_clients", [], |r| r.get(0))?;
        if n >= MAX_CLIENTS {
            c.execute(
                "DELETE FROM oauth_clients WHERE unixepoch(created_at) < unixepoch('now') - 86400
                   AND NOT EXISTS (SELECT 1 FROM oauth_grants g WHERE g.client_id = oauth_clients.id)",
                [],
            )?;
            let n: i64 = c.query_row("SELECT COUNT(*) FROM oauth_clients", [], |r| r.get(0))?;
            if n >= MAX_CLIENTS {
                return Ok(false);
            }
        }
        c.execute(
            "INSERT INTO oauth_clients (id, name, redirect_uris, secret_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![row_id, row_name, uris, secret_hash, db::now()],
        )?;
        Ok(true)
    })
    .await?;
    if !added {
        return Err(OAuthError::new(StatusCode::SERVICE_UNAVAILABLE, "temporarily_unavailable", "too many apps"));
    }
    tracing::info!(app = %name, "MCP app registered");
    let mut out = json!({
        "client_id": id,
        "client_name": name,
        "redirect_uris": reg.redirect_uris,
        "grant_types": ["authorization_code", "refresh_token"],
        "response_types": ["code"],
        "token_endpoint_auth_method": method,
        "client_id_issued_at": time::OffsetDateTime::now_utc().unix_timestamp(),
    });
    if let Some(s) = secret {
        out["client_secret"] = json!(s);
        out["client_secret_expires_at"] = json!(0);
    }
    Ok(out)
}

/// A redirect URI an app may register: `https://`, or `http://` to this machine (RFC 8252).
fn allowed(u: &str) -> bool {
    if u.contains('#') || u.len() > 2000 || u.chars().any(|c| c.is_control() || c == ' ') {
        return false;
    }
    match u.strip_prefix("https://") {
        Some(rest) => !rest.is_empty() && !rest.starts_with('/'),
        None => loopback(u).is_some(),
    }
}

/// An `http://` address on this machine without its port: a native app may pick any port when it signs in.
fn loopback(u: &str) -> Option<String> {
    let rest = u.strip_prefix("http://")?;
    let (authority, path) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    let host = if authority.starts_with("[::1]") { "[::1]" } else { authority.split(':').next()? };
    matches!(host, "localhost" | "127.0.0.1" | "[::1]").then(|| format!("http://{host}{path}"))
}

fn redirect_matches(registered: &[String], given: &str) -> bool {
    let given_loop = loopback(given);
    registered.iter().any(|r| r == given || (given_loop.is_some() && loopback(r) == given_loop))
}

// ---------------------------------------------------------------------------------------------
// Authorization (the portal's consent page)

#[derive(Deserialize, Default)]
struct AuthRequest {
    response_type: Option<String>,
    client_id: Option<String>,
    redirect_uri: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
    state: Option<String>,
}

struct Checked {
    client_id: String,
    client_name: String,
    redirect_uri: String,
    challenge: String,
}

/// The app and where to send the browser back, or why the request can't go ahead (shown on the page: with a bad
/// redirect there's nowhere safe to send an error).
async fn check(state: &AppState, r: &AuthRequest) -> ApiResult<Checked> {
    let client_id = r.client_id.clone().ok_or_else(|| AppError::bad_request("This link has no client_id."))?;
    let id = client_id.clone();
    let client: Option<(String, String)> = db::call(&state.pool, move |c| {
        Ok(c.query_row("SELECT name, redirect_uris FROM oauth_clients WHERE id = ?1", [&id], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .optional()?)
    })
    .await?;
    let Some((client_name, uris)) = client else {
        return Err(AppError::bad_request(
            "This app isn't registered here (any more). Start connecting again from the app.",
        ));
    };
    let uris: Vec<String> = serde_json::from_str(&uris)?;
    let redirect_uri = match (&r.redirect_uri, uris.as_slice()) {
        (Some(u), _) if redirect_matches(&uris, u) => u.clone(),
        (None, [only]) => only.clone(),
        _ => return Err(AppError::bad_request("The app asked to be sent back to an address it didn't register.")),
    };
    if r.response_type.as_deref() != Some("code") {
        return Err(AppError::bad_request("Hoard only hands out sign-in codes (response_type=code)."));
    }
    let challenge = r.code_challenge.clone().unwrap_or_default();
    if r.code_challenge_method.as_deref() != Some("S256") || !(43..=128).contains(&challenge.len()) {
        return Err(AppError::bad_request("The app has to use PKCE with S256 to sign in to Hoard."));
    }
    Ok(Checked { client_id, client_name, redirect_uri, challenge })
}

async fn describe_request(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Query(r): Query<AuthRequest>,
) -> ApiResult<Json<Value>> {
    who.person()?;
    let ok = check(&state, &r).await?;
    let host = ok.redirect_uri.split("://").nth(1).unwrap_or_default().split(['/', '?']).next().unwrap_or_default();
    Ok(Json(json!({ "client": ok.client_name, "redirect_host": host })))
}

#[derive(Deserialize)]
struct Approval {
    allow: bool,
    #[serde(flatten)]
    request: AuthRequest,
}

/// A code the app hasn't traded in yet.
struct Pending {
    client_id: String,
    user_id: i64,
    redirect_uri: String,
    challenge: String,
    at: Instant,
}

/// By the code's hash. Codes live minutes, so a restart losing them only means signing in again.
static PENDING: LazyLock<Mutex<HashMap<String, Pending>>> = LazyLock::new(Mutex::default);

async fn approve(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    headers: HeaderMap,
    Json(body): Json<Approval>,
) -> ApiResult<Json<Value>> {
    let user_id = who.person()?;
    let ok = check(&state, &body.request).await?;
    let iss = crate::mcp::base_url(&state, &headers);
    let st = body.request.state.as_deref();
    let redirect = if body.allow {
        let code = tools::hex(&random_bytes::<32>()?);
        let mut pending = PENDING.lock().unwrap_or_else(PoisonError::into_inner);
        pending.retain(|_, p| p.at.elapsed() < CODE_LIFETIME);
        pending.insert(
            digest(&code),
            Pending {
                client_id: ok.client_id,
                user_id,
                redirect_uri: ok.redirect_uri.clone(),
                challenge: ok.challenge,
                at: Instant::now(),
            },
        );
        with_params(&ok.redirect_uri, &[("code", Some(&code)), ("state", st), ("iss", Some(&iss))])
    } else {
        with_params(&ok.redirect_uri, &[("error", Some("access_denied")), ("state", st), ("iss", Some(&iss))])
    };
    Ok(Json(json!({ "redirect": redirect })))
}

fn with_params(uri: &str, pairs: &[(&str, Option<&str>)]) -> String {
    let mut out = uri.to_owned();
    let mut sep = if uri.contains('?') { '&' } else { '?' };
    for (k, v) in pairs {
        let Some(v) = v else { continue };
        out.push(sep);
        out.push_str(k);
        out.push('=');
        for b in v.bytes() {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                out.push(char::from(b));
            } else {
                let _ = write!(out, "%{b:02X}");
            }
        }
        sep = '&';
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Tokens

async fn token(
    State(state): State<AppState>,
    headers: HeaderMap,
    form: Result<Form<HashMap<String, String>>, FormRejection>,
) -> Response {
    let out = match form {
        Ok(Form(f)) => issue(&state, &headers, &f).await,
        Err(_) => Err(OAuthError::new(StatusCode::BAD_REQUEST, "invalid_request", "send a form-encoded token request")),
    };
    match out {
        Ok(v) => ([(header::CACHE_CONTROL, "no-store"), (header::PRAGMA, "no-cache")], Json(v)).into_response(),
        Err(e) => e.into_response(),
    }
}

/// The client id and secret, from HTTP Basic or the form.
fn client_credentials(headers: &HeaderMap, f: &HashMap<String, String>) -> (Option<String>, Option<String>) {
    let basic = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Basic "))
        .and_then(|b| base64::engine::general_purpose::STANDARD.decode(b.trim()).ok())
        .and_then(|b| String::from_utf8(b).ok());
    if let Some((id, secret)) = basic.as_deref().and_then(|b| b.split_once(':')) {
        return (Some(id.to_owned()), Some(secret.to_owned()));
    }
    (f.get("client_id").cloned(), f.get("client_secret").cloned())
}

fn s256(verifier: &str) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(verifier.as_bytes()))
}

async fn issue(state: &AppState, headers: &HeaderMap, f: &HashMap<String, String>) -> Result<Value, OAuthError> {
    let (client_id, secret) = client_credentials(headers, f);
    let client_id = client_id.ok_or_else(OAuthError::client)?;
    let id = client_id.clone();
    let known: Option<Option<String>> = db::call(&state.pool, move |c| {
        Ok(c.query_row("SELECT secret_hash FROM oauth_clients WHERE id = ?1", [&id], |r| r.get(0)).optional()?)
    })
    .await?;
    match known {
        None => return Err(OAuthError::client()),
        Some(Some(hash)) if secret.as_deref().map(digest).as_ref() != Some(&hash) => return Err(OAuthError::client()),
        Some(_) => {}
    }
    let access = format!("hoard_at_{}", tools::hex(&random_bytes::<32>()?));
    let refresh = format!("hoard_rt_{}", tools::hex(&random_bytes::<32>()?));
    let (access_hash, refresh_hash) = (digest(&access), digest(&refresh));
    match f.get("grant_type").map(String::as_str) {
        Some("authorization_code") => {
            let code = f.get("code").map(|c| digest(c)).unwrap_or_default();
            let p = PENDING.lock().unwrap_or_else(PoisonError::into_inner).remove(&code);
            let Some(p) = p.filter(|p| p.at.elapsed() < CODE_LIFETIME && p.client_id == client_id) else {
                return Err(OAuthError::grant("the code is unknown, used or expired"));
            };
            if f.get("redirect_uri").is_some_and(|u| *u != p.redirect_uri) {
                return Err(OAuthError::grant("redirect_uri doesn't match the one the code was made for"));
            }
            if f.get("code_verifier").map(|v| s256(v)) != Some(p.challenge) {
                return Err(OAuthError::grant("the PKCE code_verifier doesn't match"));
            }
            db::call(&state.pool, move |c| {
                c.execute(
                    "DELETE FROM oauth_grants WHERE unixepoch(coalesce(last_used_at, created_at)) < unixepoch('now') - ?1",
                    [IDLE_DAYS * 86_400],
                )?;
                c.execute(
                    "INSERT INTO oauth_grants (client_id, user_id, access_hash, access_expires_at, refresh_hash, created_at)
                     VALUES (?1, ?2, ?3, unixepoch('now') + ?4, ?5, ?6)",
                    params![client_id, p.user_id, access_hash, ACCESS_SECS, refresh_hash, db::now()],
                )?;
                Ok(())
            })
            .await?;
        }
        Some("refresh_token") => {
            let old = f.get("refresh_token").map(|t| digest(t)).unwrap_or_default();
            let n = db::call(&state.pool, move |c| {
                Ok(c.execute(
                    "UPDATE oauth_grants SET access_hash = ?3, access_expires_at = unixepoch('now') + ?4,
                            refresh_hash = ?5, last_used_at = ?6
                      WHERE refresh_hash = ?1 AND client_id = ?2
                        AND unixepoch(coalesce(last_used_at, created_at)) >= unixepoch('now') - ?7",
                    params![old, client_id, access_hash, ACCESS_SECS, refresh_hash, db::now(), IDLE_DAYS * 86_400],
                )?)
            })
            .await?;
            if n == 0 {
                return Err(OAuthError::grant("the refresh token is unknown, used, expired or disconnected"));
            }
        }
        _ => {
            return Err(OAuthError::new(
                StatusCode::BAD_REQUEST,
                "unsupported_grant_type",
                "grant_type must be authorization_code or refresh_token",
            ));
        }
    }
    Ok(json!({
        "access_token": access,
        "token_type": "Bearer",
        "expires_in": ACCESS_SECS,
        "refresh_token": refresh,
        "scope": "hoard",
    }))
}

// ---------------------------------------------------------------------------------------------
// Settings → Account

async fn list_connections(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    who.person()?;
    let rows = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT g.id, cl.name, u.username, g.created_at, g.last_used_at FROM oauth_grants g
               JOIN oauth_clients cl ON cl.id = g.client_id JOIN users u ON u.id = g.user_id
              ORDER BY g.created_at DESC",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?,
                    "app": r.get::<_, String>(1)?,
                    "user": r.get::<_, String>(2)?,
                    "created_at": r.get::<_, String>(3)?,
                    "last_used_at": r.get::<_, Option<String>>(4)?,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!({ "connections": rows })))
}

async fn disconnect(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    who.person()?;
    let n = db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM oauth_grants WHERE id = ?1", [id])?)).await?;
    if n == 0 {
        return Err(AppError::not_found("connection"));
    }
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirects() {
        assert!(allowed("https://claude.ai/api/mcp/auth_callback"));
        assert!(allowed("http://localhost:33418/callback"));
        assert!(allowed("http://127.0.0.1/cb"));
        assert!(allowed("http://[::1]:9000/cb"));
        assert!(!allowed("http://example.com/cb"));
        assert!(!allowed("http://localhost.evil.com/cb"));
        assert!(!allowed("http://evil@localhost/cb") || loopback("http://evil@localhost/cb").is_none());
        assert!(!allowed("https://x.example/cb#frag"));
        assert!(!allowed("javascript:alert(1)"));
        let reg = vec!["http://localhost:1234/callback".to_owned(), "https://claude.ai/cb".to_owned()];
        assert!(redirect_matches(&reg, "http://localhost:5555/callback"));
        assert!(!redirect_matches(&reg, "http://localhost:5555/other"));
        assert!(redirect_matches(&reg, "https://claude.ai/cb"));
        assert!(!redirect_matches(&reg, "https://claude.ai/cb2"));
    }

    #[test]
    fn pkce_and_params() {
        // RFC 7636, appendix B.
        assert_eq!(s256("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"), "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
        assert_eq!(
            with_params("https://a.example/cb?x=1", &[("code", Some("ab")), ("state", Some("a b&c")), ("none", None)]),
            "https://a.example/cb?x=1&code=ab&state=a%20b%26c"
        );
    }
}
