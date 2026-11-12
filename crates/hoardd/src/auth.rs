//! Accounts. People sign in to the portal with a name and password and get a session cookie; scripts, the `hoard`
//! CLI, MCP clients and the Unreal worker send an access token made in Settings → Account, as
//! `Authorization: Bearer <token>`.
//!
//! Everything needs one or the other, except reads when Settings → Account lets anyone browse. Until the first
//! account exists the portal asks for one, and whoever opens it first makes it. A forgotten password is reset on
//! the server: `hoardd --reset-password <name>` (in Docker, `docker compose exec hoard hoardd --reset-password
//! <name>`) prints a new one.
//!
//! Every account can do everything; there are no roles. Access tokens can do everything except manage accounts and
//! tokens, so a leaked one can be revoked and can't mint more.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher as _, PasswordVerifier as _};
use axum::extract::{Path, Request, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use rusqlite::{Connection, OptionalExtension as _, params};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::Digest as _;

use crate::AppState;
use crate::error::{ApiResult, AppError};
use crate::{db, tools};

pub(crate) const COOKIE: &str = "hoard_session";
/// Sent by the portal with every request. Another site's page can't add a header to a request here (there is no
/// CORS), so a request that carries the session cookie without it didn't come from the portal.
pub(crate) const CSRF_HEADER: &str = "x-hoard";
/// A session nobody has used for this long is signed out.
const SESSION_DAYS: i64 = 30;
const MIN_PASSWORD: usize = 8;
/// Failed sign-ins in a row before each name is held to one try per [`THROTTLE`].
const FREE_TRIES: u32 = 5;
const THROTTLE: Duration = Duration::from_secs(30);

/// Who a request is from.
#[derive(Clone, Debug)]
pub(crate) enum Who {
    User { id: i64, name: String },
    Token { name: String },
}

impl Who {
    fn json(&self) -> Value {
        match self {
            Self::User { name, .. } => json!({ "name": name, "kind": "user" }),
            Self::Token { name, .. } => json!({ "name": name, "kind": "token" }),
        }
    }

    /// The signed-in person, for things access tokens can't do.
    fn person(&self) -> ApiResult<i64> {
        match self {
            Self::User { id, .. } => Ok(*id),
            Self::Token { .. } => Err(AppError::forbidden("sign in to the portal to manage accounts and tokens")),
        }
    }
}

enum Credential {
    Session(String),
    Bearer(String),
}

fn credential(headers: &HeaderMap) -> Option<Credential> {
    let bearer =
        headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok()).and_then(|v| v.strip_prefix("Bearer "));
    match bearer {
        Some(t) => Some(Credential::Bearer(t.trim().to_owned())),
        None => session_token(headers).map(Credential::Session),
    }
}

fn session_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .find_map(|kv| kv.trim().strip_prefix(COOKIE)?.strip_prefix('='))
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

fn digest(secret: &str) -> String {
    tools::hex(&sha2::Sha256::digest(secret.as_bytes()))
}

fn random_bytes<const N: usize>() -> Result<[u8; N]> {
    let mut b = [0u8; N];
    getrandom::fill(&mut b).map_err(|e| anyhow!("no randomness from the OS: {e}"))?;
    Ok(b)
}

fn hash_password(password: &str) -> Result<String> {
    Ok(Argon2::default().hash_password(password.as_bytes()).map_err(|e| anyhow!("hash password: {e}"))?.to_string())
}

fn password_matches(password: &str, hash: &str) -> bool {
    Argon2::default().verify_password(password.as_bytes(), hash).is_ok()
}

/// Checked against when there's no such name, so a wrong name takes as long as a wrong password.
static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| hash_password("not a password").unwrap_or_default());

/// Who `cred` belongs to, if anyone. Notes the use (at most once a minute for tokens, once an hour for sessions).
fn lookup(c: &Connection, cred: &Credential) -> Result<Option<Who>> {
    match cred {
        Credential::Bearer(token) => {
            let hash = digest(token);
            let row: Option<(i64, String)> = c
                .query_row("SELECT id, name FROM access_tokens WHERE token_hash = ?1", [&hash], |r| {
                    Ok((r.get(0)?, r.get(1)?))
                })
                .optional()?;
            if let Some((id, _)) = &row {
                c.execute(
                    "UPDATE access_tokens SET last_used_at = ?2
                      WHERE id = ?1 AND (last_used_at IS NULL OR unixepoch(last_used_at) < unixepoch('now') - 60)",
                    params![id, db::now()],
                )?;
            }
            Ok(row.map(|(_, name)| Who::Token { name }))
        }
        Credential::Session(token) => {
            let hash = digest(token);
            let row: Option<(i64, String, i64)> = c
                .query_row(
                    "SELECT u.id, u.username, unixepoch('now') - unixepoch(s.last_seen_at)
                       FROM sessions s JOIN users u ON u.id = s.user_id WHERE s.token_hash = ?1",
                    [&hash],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
                )
                .optional()?;
            let Some((id, name, idle)) = row else { return Ok(None) };
            if idle > SESSION_DAYS * 86_400 {
                c.execute("DELETE FROM sessions WHERE token_hash = ?1", [&hash])?;
                return Ok(None);
            }
            if idle > 3600 {
                c.execute("UPDATE sessions SET last_seen_at = ?2 WHERE token_hash = ?1", params![hash, db::now()])?;
            }
            Ok(Some(Who::User { id, name }))
        }
    }
}

/// Who sent this request, if they sent a valid session or token. A cookie-signed change that the portal didn't
/// send (no [`CSRF_HEADER`]) is refused outright.
pub(crate) async fn authenticate(state: &AppState, method: &Method, headers: &HeaderMap) -> ApiResult<Option<Who>> {
    let Some(cred) = credential(headers) else { return Ok(None) };
    let safe = matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS);
    if matches!(cred, Credential::Session(_)) && !safe && !headers.contains_key(CSRF_HEADER) {
        return Err(AppError::forbidden("this request didn't come from the Hoard portal"));
    }
    Ok(db::call(&state.pool, move |c| lookup(c, &cred)).await?)
}

/// Middleware for changes and admin pages: a signed-in person or an access token.
pub(crate) async fn require_user(State(state): State<AppState>, mut req: Request, next: Next) -> ApiResult<Response> {
    let who = authenticate(&state, req.method(), req.headers()).await?.ok_or_else(AppError::unauthorized)?;
    req.extensions_mut().insert(who);
    Ok(next.run(req).await)
}

/// Middleware for reads: the same, unless Settings lets anyone browse.
pub(crate) async fn require_reader(State(state): State<AppState>, mut req: Request, next: Next) -> ApiResult<Response> {
    match authenticate(&state, req.method(), req.headers()).await? {
        Some(who) => {
            req.extensions_mut().insert(who);
        }
        None if state.settings.get().guest_browsing => {}
        None => return Err(AppError::unauthorized()),
    }
    Ok(next.run(req).await)
}

// ---------------------------------------------------------------------------------------------
// Sign-in throttling

/// Failed sign-ins in a row per name (lowercased), and when the last one was.
static FAILURES: LazyLock<Mutex<HashMap<String, (u32, Instant)>>> = LazyLock::new(Mutex::default);

/// How long `name` must wait before its next try.
fn wait_for(name: &str) -> Option<Duration> {
    let failures = FAILURES.lock().unwrap_or_else(PoisonError::into_inner);
    let &(n, last) = failures.get(&name.to_lowercase())?;
    (n >= FREE_TRIES).then(|| THROTTLE.saturating_sub(last.elapsed())).filter(|d| !d.is_zero())
}

fn note_attempt(name: &str, ok: bool) {
    let mut failures = FAILURES.lock().unwrap_or_else(PoisonError::into_inner);
    if ok {
        failures.remove(&name.to_lowercase());
        return;
    }
    if failures.len() > 10_000 {
        failures.retain(|_, (_, last)| last.elapsed() < Duration::from_hours(1));
    }
    let entry = failures.entry(name.to_lowercase()).or_insert((0, Instant::now()));
    *entry = (entry.0 + 1, Instant::now());
}

// ---------------------------------------------------------------------------------------------
// Routes

/// Open to everyone: what the portal needs to show the sign-in or first-account screen.
pub(crate) fn router() -> Router<AppState> {
    let posts = Router::new()
        .route("/api/auth/setup", post(setup))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route_layer(axum::middleware::from_fn(from_portal));
    Router::new().route("/api/auth/status", get(status)).merge(posts)
}

/// Signing in, making the first account and signing out only come from the portal: another site can't make a browser
/// claim a fresh server, or sign it in as someone else.
async fn from_portal(req: Request, next: Next) -> Response {
    if req.headers().contains_key(CSRF_HEADER) {
        next.run(req).await
    } else {
        AppError::forbidden("this request didn't come from the Hoard portal").into_response()
    }
}

/// Behind [`require_user`].
pub(crate) fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/password", post(change_password))
        .route("/api/admin/users", get(list_users).post(create_user))
        .route("/api/admin/users/{id}", axum::routing::delete(delete_user))
        .route("/api/admin/tokens", get(list_tokens).post(create_token))
        .route("/api/admin/tokens/{id}", axum::routing::delete(delete_token))
}

async fn status(State(state): State<AppState>, headers: HeaderMap) -> ApiResult<Json<Value>> {
    let cred = credential(&headers);
    let (accounts, who) = db::call(&state.pool, move |c| {
        let n: i64 = c.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))?;
        let who = match cred {
            Some(cred) => lookup(c, &cred)?,
            None => None,
        };
        Ok((n, who))
    })
    .await?;
    Ok(Json(json!({
        "setup_needed": accounts == 0,
        "signed_in": who.is_some(),
        "user": who.as_ref().map(Who::json),
        "guest_browsing": state.settings.get().guest_browsing,
    })))
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

/// A name and password fit to save, or what's wrong with them.
fn check_new(username: &str, password: &str) -> ApiResult<String> {
    let name = username.trim();
    if name.is_empty() || name.chars().count() > 64 || name.chars().any(char::is_control) {
        return Err(AppError::bad_request("Choose a name of up to 64 characters"));
    }
    check_password(password)?;
    Ok(name.to_owned())
}

fn check_password(password: &str) -> ApiResult<()> {
    if password.chars().count() < MIN_PASSWORD {
        return Err(AppError::bad_request(format!("Use a password of at least {MIN_PASSWORD} characters")));
    }
    Ok(())
}

/// Whether to mark the cookie `Secure`: when this request came over HTTPS, directly or through a proxy that says so.
/// (Not from the public address alone: someone on plain `http://` at home would never get the cookie back.)
fn over_https(state: &AppState, headers: &HeaderMap) -> bool {
    state.cfg.tls_cert.is_some()
        || state.cfg.tls_self_signed
        || headers
            .get("x-forwarded-proto")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|p| p.eq_ignore_ascii_case("https"))
}

/// The cookie lasts as long as browsers allow; the session itself ends after [`SESSION_DAYS`] unused.
fn cookie(state: &AppState, headers: &HeaderMap, token: &str, max_age: i64) -> Result<HeaderValue> {
    let secure = if over_https(state, headers) { "; Secure" } else { "" };
    Ok(HeaderValue::from_str(&format!("{COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}"))?)
}

fn new_session(c: &Connection, user_id: i64) -> Result<String> {
    let token = tools::hex(&random_bytes::<32>()?);
    c.execute(
        "INSERT INTO sessions (token_hash, user_id, created_at, last_seen_at) VALUES (?1, ?2, ?3, ?3)",
        params![digest(&token), user_id, db::now()],
    )?;
    c.execute("DELETE FROM sessions WHERE unixepoch(last_seen_at) < unixepoch('now') - ?1", [SESSION_DAYS * 86_400])?;
    Ok(token)
}

fn signed_in(state: &AppState, headers: &HeaderMap, token: &str, name: &str) -> ApiResult<Response> {
    let set = cookie(state, headers, token, 400 * 86_400)?;
    Ok(([(header::SET_COOKIE, set)], Json(json!({ "user": { "name": name, "kind": "user" } }))).into_response())
}

/// The first account. Refused once there is one.
async fn setup(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<Credentials>,
) -> ApiResult<Response> {
    let name = check_new(&body.username, &body.password)?;
    let made = name.clone();
    let token = db::call(&state.pool, move |c| {
        let hash = hash_password(&body.password)?;
        let added = c.execute(
            "INSERT INTO users (username, password_hash, created_at)
             SELECT ?1, ?2, ?3 WHERE NOT EXISTS (SELECT 1 FROM users)",
            params![made, hash, db::now()],
        )?;
        if added == 0 {
            return Ok(None);
        }
        new_session(c, c.last_insert_rowid()).map(Some)
    })
    .await?
    .ok_or_else(|| AppError::conflict("There's already an account: sign in"))?;
    tracing::info!(user = %name, "first account made");
    signed_in(&state, &headers, &token, &name)
}

async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<Credentials>,
) -> ApiResult<Response> {
    let name = body.username.trim().to_owned();
    if let Some(wait) = wait_for(&name) {
        return Err(AppError::too_many(format!("Too many wrong tries: wait {} seconds", wait.as_secs().max(1))));
    }
    let lookup_name = name.clone();
    let found = db::call(&state.pool, move |c| {
        let row: Option<(i64, String, String)> = c
            .query_row("SELECT id, username, password_hash FROM users WHERE username = ?1", [&lookup_name], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .optional()?;
        let hash = row.as_ref().map_or(DUMMY_HASH.as_str(), |r| r.2.as_str());
        if !password_matches(&body.password, hash) {
            return Ok(None);
        }
        let Some((id, username, _)) = row else { return Ok(None) };
        Ok(Some((new_session(c, id)?, username)))
    })
    .await?;
    note_attempt(&name, found.is_some());
    let Some((token, username)) = found else {
        tracing::info!(user = %name, "failed sign-in");
        return Err(AppError { status: StatusCode::UNAUTHORIZED, err: anyhow!("That name and password don't match") });
    };
    signed_in(&state, &headers, &token, &username)
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> ApiResult<Response> {
    if let Some(token) = session_token(&headers) {
        db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM sessions WHERE token_hash = ?1", [digest(&token)])?))
            .await?;
    }
    let gone = cookie(&state, &headers, "", 0)?;
    Ok(([(header::SET_COOKIE, gone)], StatusCode::NO_CONTENT).into_response())
}

#[derive(Deserialize)]
struct PasswordChange {
    current: String,
    new: String,
}

/// Change your own password. Signs out your other sessions.
async fn change_password(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    headers: HeaderMap,
    Json(body): Json<PasswordChange>,
) -> ApiResult<StatusCode> {
    let id = who.person()?;
    check_password(&body.new)?;
    let keep = session_token(&headers).map(|t| digest(&t)).unwrap_or_default();
    let changed = db::call(&state.pool, move |c| {
        let hash: String = c.query_row("SELECT password_hash FROM users WHERE id = ?1", [id], |r| r.get(0))?;
        if !password_matches(&body.current, &hash) {
            return Ok(false);
        }
        c.execute("UPDATE users SET password_hash = ?2 WHERE id = ?1", params![id, hash_password(&body.new)?])?;
        c.execute("DELETE FROM sessions WHERE user_id = ?1 AND token_hash != ?2", params![id, keep])?;
        Ok(true)
    })
    .await?;
    if !changed {
        return Err(AppError::bad_request("Your current password isn't right"));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_users(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let me = who.person()?;
    let users = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare("SELECT id, username, created_at FROM users ORDER BY username")?;
        let rows = stmt
            .query_map([], |r| {
                let id: i64 = r.get(0)?;
                Ok(json!({ "id": id, "username": r.get::<_, String>(1)?, "created_at": r.get::<_, String>(2)?, "you": id == me }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!({ "users": users })))
}

async fn create_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(body): Json<Credentials>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    who.person()?;
    let name = check_new(&body.username, &body.password)?;
    let username = name.clone();
    let added = db::call(&state.pool, move |c| {
        let hash = hash_password(&body.password)?;
        let n = c.execute(
            "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
            params![username, hash, db::now()],
        )?;
        Ok((n == 1).then(|| c.last_insert_rowid()))
    })
    .await?;
    let Some(id) = added else {
        return Err(AppError::conflict("There's already an account with that name"));
    };
    Ok((StatusCode::CREATED, Json(json!({ "id": id, "username": name }))))
}

async fn delete_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    if who.person()? == id {
        return Err(AppError::bad_request("You can't remove your own account while you're signed in to it"));
    }
    let n = db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM users WHERE id = ?1", [id])?)).await?;
    if n == 0 {
        return Err(AppError::not_found("account"));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_tokens(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    who.person()?;
    let tokens = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT t.id, t.name, t.created_at, t.last_used_at, u.username
               FROM access_tokens t LEFT JOIN users u ON u.id = t.created_by ORDER BY t.created_at DESC",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "created_at": r.get::<_, String>(2)?,
                    "last_used_at": r.get::<_, Option<String>>(3)?,
                    "created_by": r.get::<_, Option<String>>(4)?,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!({ "tokens": tokens })))
}

#[derive(Deserialize)]
struct NewToken {
    name: String,
}

/// A new access token. The token itself is in this response and nowhere else: only its hash is kept.
async fn create_token(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(body): Json<NewToken>,
) -> ApiResult<Json<Value>> {
    let me = who.person()?;
    let name = body.name.trim().to_owned();
    if name.is_empty() || name.chars().count() > 80 {
        return Err(AppError::bad_request("Name the token after what will use it, in up to 80 characters"));
    }
    let token = format!("hoard_{}", tools::hex(&random_bytes::<32>()?));
    let (hash, label) = (digest(&token), name.clone());
    let id = db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO access_tokens (name, token_hash, created_by, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![label, hash, me, db::now()],
        )?;
        Ok(c.last_insert_rowid())
    })
    .await?;
    Ok(Json(json!({ "id": id, "name": name, "token": token })))
}

async fn delete_token(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    who.person()?;
    let n = db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM access_tokens WHERE id = ?1", [id])?)).await?;
    if n == 0 {
        return Err(AppError::not_found("token"));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------------------------
// Password reset from the server's shell

/// For reset passwords: no look-alikes (0/O, 1/l/I). 16 of them is about 90 bits.
const ALPHABET: &[u8] = b"abcdefghijkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";

/// Give `name` a new random password, sign them out everywhere, and return the password.
/// Returns the account's name as stored, and its new password.
pub(crate) fn reset_password(c: &Connection, name: &str) -> Result<(String, String)> {
    let found: Option<(i64, String)> = c
        .query_row("SELECT id, username FROM users WHERE username = ?1", [name.trim()], |r| Ok((r.get(0)?, r.get(1)?)))
        .optional()?;
    let Some((id, username)) = found else {
        let mut stmt = c.prepare("SELECT username FROM users ORDER BY username")?;
        let names = stmt.query_map([], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        if names.is_empty() {
            anyhow::bail!("there are no accounts yet: open the portal to make one");
        }
        anyhow::bail!("there's no account called {name:?}. The accounts are: {}", names.join(", "));
    };
    let password: String =
        random_bytes::<16>()?.iter().map(|b| char::from(ALPHABET[usize::from(*b) % ALPHABET.len()])).collect();
    c.execute("UPDATE users SET password_hash = ?2 WHERE id = ?1", params![id, hash_password(&password)?])?;
    c.execute("DELETE FROM sessions WHERE user_id = ?1", [id])?;
    Ok((username, password))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookies_and_bearers() {
        let mut h = HeaderMap::new();
        h.insert(header::COOKIE, HeaderValue::from_static("theme=dark; hoard_session=abc123; other=1"));
        assert_eq!(session_token(&h).as_deref(), Some("abc123"));
        assert!(matches!(credential(&h), Some(Credential::Session(t)) if t == "abc123"));
        h.insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer hoard_xyz"));
        assert!(matches!(credential(&h), Some(Credential::Bearer(t)) if t == "hoard_xyz"));
        let mut h = HeaderMap::new();
        h.insert(header::COOKIE, HeaderValue::from_static("hoard_session_old=1; hoard_session="));
        assert_eq!(session_token(&h), None);
    }

    #[test]
    fn passwords() {
        let hash = hash_password("correct horse").unwrap();
        assert!(password_matches("correct horse", &hash));
        assert!(!password_matches("wrong horse", &hash));
        assert!(!password_matches("anything", "not a hash"));
    }

    #[test]
    fn throttles_after_five_misses() {
        let name = "Throttle-Test";
        for _ in 0..FREE_TRIES {
            assert!(wait_for(name).is_none());
            note_attempt(name, false);
        }
        assert!(wait_for("throttle-test").is_some());
        note_attempt(name, true);
        assert!(wait_for(name).is_none());
    }
}
