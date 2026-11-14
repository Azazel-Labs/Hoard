//! Accounts. People sign in to the portal with a name and password and get a session cookie; scripts, the `hoard`
//! CLI, MCP clients and the Unreal worker send an access token made in Settings → Account, as
//! `Authorization: Bearer <token>`.
//!
//! Everything needs one or the other, except reads when Settings → Account lets anyone browse. Until the first
//! account exists the portal asks for one, and whoever opens it first makes it. A forgotten password is reset on
//! the server: `hoardd --reset-password <name>` (in Docker, `docker compose exec hoard hoardd --reset-password
//! <name>`) prints a new one.
//!
//! Each account has a [`Role`]: it can look, change the library as well, or administer the server too (Settings,
//! accounts, maintenance). The first account is an admin. An access token has a role of its own, chosen when it's made
//! and never more than its maker's: a token made by someone who is later given a lesser role has that role too.
//! Tokens can't manage accounts or tokens, so a leaked one can be revoked and can't mint more. Apps that sign in over
//! OAuth (`oauth.rs`) act with the role of the person who let them in.

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
use crate::signin::{Gate, Source};
use crate::{db, tools};

/// The session cookie over plain `http://`.
const COOKIE: &str = "hoard_session";
/// The session cookie over HTTPS. A different name, because a browser won't let a plain `http://` page replace a
/// `Secure` cookie: with one name, signing in over `http://` after HTTPS was turned off would silently fail.
const SECURE_COOKIE: &str = "__Host-hoard_session";
/// Sent by the portal with every request. Another site's page can't add a header to a request here (there is no
/// CORS), so a request that carries the session cookie without it didn't come from the portal.
pub(crate) const CSRF_HEADER: &str = "x-hoard";
/// A session nobody has used for this long is signed out.
const SESSION_DAYS: i64 = 30;
const MIN_PASSWORD: usize = 8;
/// Failed sign-ins in a row before each name is held to one try per [`THROTTLE`].
const FREE_TRIES: u32 = 5;
const THROTTLE: Duration = Duration::from_secs(30);

/// What an account or token may do. Each can do everything the ones before it can.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Role {
    /// Look around, search and download.
    View,
    /// Also add, change, tag, collect and archive.
    Edit,
    /// Also Settings, accounts, maintenance, and deleting for good.
    Admin,
}

impl Role {
    pub(crate) fn parse(s: &str) -> Option<Self> {
        match s {
            "view" => Some(Self::View),
            "edit" => Some(Self::Edit),
            "admin" => Some(Self::Admin),
            _ => None,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::View => "view",
            Self::Edit => "edit",
            Self::Admin => "admin",
        }
    }

    /// As stored. Anything else (there shouldn't be) can only look.
    pub(crate) fn stored(s: &str) -> Self {
        Self::parse(s).unwrap_or(Self::View)
    }

    fn requested(s: Option<&str>, default: Self) -> ApiResult<Self> {
        match s {
            None => Ok(default),
            Some(s) => Self::parse(s).ok_or_else(|| AppError::bad_request("A role is view, edit or admin")),
        }
    }
}

/// Who a request is from.
#[derive(Clone, Debug)]
pub(crate) enum Who {
    User {
        id: i64,
        name: String,
        role: Role,
    },
    /// An access token or connected app, acting for `owner` (gone if that account was removed).
    Token {
        name: String,
        role: Role,
        owner: Option<i64>,
    },
}

impl Who {
    fn json(&self) -> Value {
        match self {
            Self::User { name, role, .. } => json!({ "name": name, "kind": "user", "role": role.as_str() }),
            Self::Token { name, role, .. } => json!({ "name": name, "kind": "token", "role": role.as_str() }),
        }
    }

    pub(crate) const fn role(&self) -> Role {
        match self {
            Self::User { role, .. } | Self::Token { role, .. } => *role,
        }
    }

    /// Refused unless this can do what `role` can.
    pub(crate) fn needs(&self, role: Role) -> ApiResult<()> {
        if self.role() >= role {
            return Ok(());
        }
        Err(AppError::forbidden(match role {
            Role::Admin => "only an admin can do this",
            _ => "this account can look but not change anything: an admin can change that in Settings → Account",
        }))
    }

    /// The account this is or acts for: whose favourites and saved searches are meant.
    pub(crate) const fn owner(&self) -> Option<i64> {
        match self {
            Self::User { id, .. } => Some(*id),
            Self::Token { owner, .. } => *owner,
        }
    }

    /// The signed-in person, for things access tokens can't do.
    pub(crate) fn person(&self) -> ApiResult<i64> {
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

/// The session from the cookie: over HTTPS the secure one if it's there, else the plain one (also sent over HTTPS).
fn session_token(headers: &HeaderMap) -> Option<String> {
    over_https(headers)
        .then(|| cookie_value(headers, SECURE_COOKIE))
        .flatten()
        .or_else(|| cookie_value(headers, COOKIE))
}

pub(crate) fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .find_map(|kv| kv.trim().strip_prefix(name)?.strip_prefix('='))
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

pub(crate) fn digest(secret: &str) -> String {
    tools::hex(&sha2::Sha256::digest(secret.as_bytes()))
}

pub(crate) fn random_bytes<const N: usize>() -> Result<[u8; N]> {
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

/// An access token's id, name and role, and its maker's role, id and where they sign in.
type TokenRow = (i64, String, String, String, i64, String);

/// Who `cred` belongs to, if anyone. Notes the use (at most once a minute for tokens, once an hour for sessions).
/// Sessions end after [`SESSION_DAYS`] unused, or when `gate` says so. Nothing gets in for an account whose way of
/// signing in is turned off (a directory account once the directory is), and a token goes with the account that made
/// it.
fn lookup(c: &Connection, cred: &Credential, gate: Gate) -> Result<Option<Who>> {
    match cred {
        Credential::Bearer(token) => {
            let hash = digest(token);
            let row: Option<TokenRow> = c
                .query_row(
                    "SELECT t.id, t.name, t.role, u.role, u.id, u.source
                       FROM access_tokens t JOIN users u ON u.id = t.created_by WHERE t.token_hash = ?1",
                    [&hash],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
                )
                .optional()?;
            let Some((id, name, own, maker, owner, source)) = row else {
                let who = crate::oauth::lookup(c, &hash)?;
                if let Some(Who::Token { owner: Some(owner), .. }) = &who {
                    let source: Option<String> =
                        c.query_row("SELECT source FROM users WHERE id = ?1", [owner], |r| r.get(0)).optional()?;
                    if !source.is_some_and(|s| gate.lets_in(&s)) {
                        return Ok(None);
                    }
                }
                return Ok(who);
            };
            if !gate.lets_in(&source) {
                return Ok(None);
            }
            c.execute(
                "UPDATE access_tokens SET last_used_at = ?2
                  WHERE id = ?1 AND (last_used_at IS NULL OR unixepoch(last_used_at) < unixepoch('now') - 60)",
                params![id, db::now()],
            )?;
            let role = Role::stored(&own).min(Role::stored(&maker));
            Ok(Some(Who::Token { name, role, owner: Some(owner) }))
        }
        Credential::Session(token) => {
            let hash = digest(token);
            let row: Option<(i64, String, String, i64, i64, String)> = c
                .query_row(
                    "SELECT u.id, u.username, u.role, unixepoch('now') - unixepoch(s.last_seen_at),
                            unixepoch('now') - unixepoch(s.created_at), u.source
                       FROM sessions s JOIN users u ON u.id = s.user_id WHERE s.token_hash = ?1",
                    [&hash],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
                )
                .optional()?;
            let Some((id, name, role, idle, age, source)) = row else { return Ok(None) };
            let hours = gate.session_hours(&source);
            if !gate.lets_in(&source)
                || idle > SESSION_DAYS * 86_400
                || hours.is_some_and(|h| age > i64::from(h) * 3600)
            {
                c.execute("DELETE FROM sessions WHERE token_hash = ?1", [&hash])?;
                return Ok(None);
            }
            if idle > 3600 {
                c.execute("UPDATE sessions SET last_seen_at = ?2 WHERE token_hash = ?1", params![hash, db::now()])?;
            }
            Ok(Some(Who::User { id, name, role: Role::stored(&role) }))
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
    let gate = state.settings.get().auth.gate();
    Ok(db::call(&state.pool, move |c| lookup(c, &cred, gate)).await?)
}

async fn require(state: &AppState, role: Role, mut req: Request, next: Next) -> ApiResult<Response> {
    let who = authenticate(state, req.method(), req.headers()).await?.ok_or_else(AppError::unauthorized)?;
    who.needs(role)?;
    req.extensions_mut().insert(who);
    Ok(next.run(req).await)
}

/// Middleware for what's anyone's own (their password, tokens and connected apps): a signed-in person or a token.
pub(crate) async fn require_user(State(state): State<AppState>, req: Request, next: Next) -> ApiResult<Response> {
    require(&state, Role::View, req, next).await
}

/// Middleware for changes to the library.
pub(crate) async fn require_editor(State(state): State<AppState>, req: Request, next: Next) -> ApiResult<Response> {
    require(&state, Role::Edit, req, next).await
}

/// Middleware for Settings, accounts and maintenance.
pub(crate) async fn require_admin(State(state): State<AppState>, req: Request, next: Next) -> ApiResult<Response> {
    require(&state, Role::Admin, req, next).await
}

/// Middleware for reads: anyone signed in, unless Settings lets anyone browse.
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

/// Behind [`require_user`]: everyone's own password and tokens. An admin sees and can revoke everyone's tokens.
pub(crate) fn account_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/password", post(change_password))
        .route("/api/admin/tokens", get(list_tokens).post(create_token))
        .route("/api/admin/tokens/{id}", axum::routing::delete(delete_token))
}

/// Behind [`require_admin`].
pub(crate) fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/users", get(list_users).post(create_user))
        .route("/api/admin/users/{id}", axum::routing::delete(delete_user).patch(patch_user))
}

async fn status(State(state): State<AppState>, headers: HeaderMap) -> ApiResult<Json<Value>> {
    let cred = credential(&headers);
    let settings = state.settings.get();
    let gate = settings.auth.gate();
    let (accounts, who) = db::call(&state.pool, move |c| {
        let n: i64 = c.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))?;
        let who = match cred {
            Some(cred) => lookup(c, &cred, gate)?,
            None => None,
        };
        let source: Option<String> = match &who {
            Some(Who::User { id, .. }) => {
                c.query_row("SELECT source FROM users WHERE id = ?1", [id], |r| r.get(0)).optional()?
            }
            _ => None,
        };
        Ok((n, who.map(|w| (w, source))))
    })
    .await?;
    let user = who.as_ref().map(|(w, source)| {
        let mut j = w.json();
        if let (Some(source), Some(o)) = (source, j.as_object_mut()) {
            o.insert("source".into(), json!(source));
        }
        j
    });
    Ok(Json(json!({
        "setup_needed": accounts == 0,
        "signed_in": who.is_some(),
        "user": user,
        "guest_browsing": settings.guest_browsing,
        "name": settings.name,
        "sign_in": crate::signin::sign_in_options(&settings.auth),
    })))
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct SetupBody {
    username: String,
    password: String,
    #[serde(default)]
    code: String,
}

/// The code that makes the first account while there's none, from the server's log (or `HOARD_SETUP_CODE`): whoever
/// reaches a fresh server's address first can't claim it, only someone who can see the server.
static SETUP_CODE: Mutex<Option<String>> = Mutex::new(None);

/// A setup code as typed: any case, with or without its dash.
fn normalized_code(s: &str) -> String {
    s.chars().filter(|c| !matches!(c, '-' | ' ')).flat_map(char::to_uppercase).collect()
}

/// When there's no account yet, choose the setup code and say it in the log.
pub(crate) async fn prepare_setup(state: &AppState) -> Result<()> {
    let accounts: i64 =
        db::call(&state.pool, |c| Ok(c.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))?)).await?;
    if accounts > 0 {
        return Ok(());
    }
    let given = state.cfg.setup_code.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let code = if let Some(code) = given {
        tracing::warn!(
            "This server has no accounts yet: open the portal and give the setup code from HOARD_SETUP_CODE"
        );
        code.to_owned()
    } else {
        const ALPHABET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
        let s: String =
            random_bytes::<10>()?.iter().map(|b| char::from(ALPHABET[usize::from(*b) % ALPHABET.len()])).collect();
        let code = format!("{}-{}", &s[..5], &s[5..]);
        tracing::warn!("This server has no accounts yet: open the portal and give the setup code {code}");
        code
    };
    *SETUP_CODE.lock().unwrap_or_else(PoisonError::into_inner) = Some(normalized_code(&code));
    Ok(())
}

/// A name and password fit to save, or what's wrong with them.
fn check_new(username: &str, password: &str) -> ApiResult<String> {
    let name = check_name(username)?;
    check_password(password)?;
    Ok(name)
}

fn check_name(username: &str) -> ApiResult<String> {
    let name = username.trim();
    if name.is_empty() || name.chars().count() > 64 || name.chars().any(char::is_control) {
        return Err(AppError::bad_request("Choose a name of up to 64 characters"));
    }
    Ok(name.to_owned())
}

fn check_password(password: &str) -> ApiResult<()> {
    if password.chars().count() < MIN_PASSWORD {
        return Err(AppError::bad_request(format!("Use a password of at least {MIN_PASSWORD} characters")));
    }
    Ok(())
}

/// Whether to mark the cookie `Secure`: when this request came over HTTPS, directly (the server says so in the same
/// header) or through a proxy that says so. (Not from the public address alone: someone on plain `http://` at home
/// would never get the cookie back.)
pub(crate) fn over_https(headers: &HeaderMap) -> bool {
    headers.get("x-forwarded-proto").and_then(|v| v.to_str().ok()).is_some_and(|p| p.eq_ignore_ascii_case("https"))
}

/// The cookie lasts as long as browsers allow; the session itself ends after [`SESSION_DAYS`] unused.
fn cookie(headers: &HeaderMap, token: &str, max_age: i64) -> Result<HeaderValue> {
    set_cookie(over_https(headers), token, max_age)
}

fn set_cookie(secure: bool, token: &str, max_age: i64) -> Result<HeaderValue> {
    let (name, flag) = if secure { (SECURE_COOKIE, "; Secure") } else { (COOKIE, "") };
    Ok(HeaderValue::from_str(&format!("{name}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age}{flag}"))?)
}

pub(crate) fn new_session(c: &Connection, user_id: i64) -> Result<String> {
    let token = tools::hex(&random_bytes::<32>()?);
    c.execute(
        "INSERT INTO sessions (token_hash, user_id, created_at, last_seen_at) VALUES (?1, ?2, ?3, ?3)",
        params![digest(&token), user_id, db::now()],
    )?;
    c.execute("DELETE FROM sessions WHERE unixepoch(last_seen_at) < unixepoch('now') - ?1", [SESSION_DAYS * 86_400])?;
    Ok(token)
}

/// The session cookie for a new sign-in.
pub(crate) fn session_cookie(headers: &HeaderMap, token: &str) -> Result<HeaderValue> {
    cookie(headers, token, 400 * 86_400)
}

fn signed_in(headers: &HeaderMap, token: &str, name: &str, role: Role) -> ApiResult<Response> {
    let set = session_cookie(headers, token)?;
    let user = json!({ "name": name, "kind": "user", "role": role.as_str() });
    Ok(([(header::SET_COOKIE, set)], Json(json!({ "user": user }))).into_response())
}

/// The first account, with the setup code from the server's log. Refused once there is one.
async fn setup(State(state): State<AppState>, headers: HeaderMap, Json(body): Json<SetupBody>) -> ApiResult<Response> {
    const TRIES: &str = "\u{1}setup";
    if let Some(wait) = wait_for(TRIES) {
        return Err(AppError::too_many(format!("Too many wrong codes: wait {} seconds", wait.as_secs().max(1))));
    }
    let name = check_new(&body.username, &body.password)?;
    let expected = SETUP_CODE.lock().unwrap_or_else(PoisonError::into_inner).clone();
    let Some(expected) = expected else { return Err(AppError::conflict("There's already an account: sign in")) };
    // Compared as digests, so how long it takes says nothing about how much of the code was right.
    let right = digest(&normalized_code(&body.code)) == digest(&expected);
    note_attempt(TRIES, right);
    if !right {
        return Err(AppError {
            status: StatusCode::FORBIDDEN,
            err: anyhow!("That isn't the setup code: the server prints it in its log when it starts"),
        });
    }
    let made = name.clone();
    let token = db::call(&state.pool, move |c| {
        let hash = hash_password(&body.password)?;
        let added = c.execute(
            "INSERT INTO users (username, password_hash, role, created_at)
             SELECT ?1, ?2, 'admin', ?3 WHERE NOT EXISTS (SELECT 1 FROM users)",
            params![made, hash, db::now()],
        )?;
        if added == 0 {
            return Ok(None);
        }
        new_session(c, c.last_insert_rowid()).map(Some)
    })
    .await?
    .ok_or_else(|| AppError::conflict("There's already an account: sign in"))?;
    *SETUP_CODE.lock().unwrap_or_else(PoisonError::into_inner) = None;
    tracing::info!(user = %name, "first account made");
    signed_in(&headers, &token, &name, Role::Admin)
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
    let auth = state.settings.get().auth.clone();
    let directory = auth.directory().is_some();
    let (lookup_name, password) = (name.clone(), body.password.clone());
    // Hoard's own accounts always sign in with their own password, whatever Settings → Authentication says, so a
    // directory that's down or set up wrong can't lock an admin out. Anyone else goes to the directory, if there is one.
    let found = db::call(&state.pool, move |c| {
        let row: Option<(i64, String, String, String, String)> = c
            .query_row(
                "SELECT id, username, password_hash, role, source FROM users WHERE username = ?1",
                [&lookup_name],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .optional()?;
        if directory && row.as_ref().is_none_or(|r| r.4 == "ldap") {
            return Ok(Err(()));
        }
        let internal = row.filter(|r| r.4 == "internal");
        let hash = internal.as_ref().map_or(DUMMY_HASH.as_str(), |r| r.2.as_str());
        if !password_matches(&password, hash) {
            return Ok(Ok(None));
        }
        let Some((id, username, _, role, _)) = internal else { return Ok(Ok(None)) };
        Ok(Ok(Some((new_session(c, id)?, username, Role::stored(&role)))))
    })
    .await?;
    let found = match found {
        Ok(found) => found,
        Err(()) => directory_login(&state, &auth, &name, &body.password).await?,
    };
    note_attempt(&name, found.is_some());
    let Some((token, username, role)) = found else {
        tracing::info!(user = %name, "failed sign-in");
        return Err(AppError { status: StatusCode::UNAUTHORIZED, err: anyhow!("That name and password don't match") });
    };
    signed_in(&headers, &token, &username, role)
}

/// Sign in through LDAP or Active Directory. `None`: not someone the directory knows with that password.
async fn directory_login(
    state: &AppState,
    auth: &crate::signin::AuthConfig,
    name: &str,
    password: &str,
) -> ApiResult<Option<(String, String, Role)>> {
    let Some(dir) = auth.directory() else { return Ok(None) };
    let person = crate::signin::directory_sign_in(dir, name, password).await.map_err(|e| {
        tracing::warn!(user = %name, "the directory couldn't check a sign-in: {e:#}");
        AppError::unavailable(format!("The directory couldn't check your password: {e:#}"))
    })?;
    let Some(person) = person else { return Ok(None) };
    let auth = auth.clone();
    let made = db::call(&state.pool, move |c| {
        Ok(match crate::signin::account_for(c, &auth, Source::Ldap, &person.name, None, &person.groups)? {
            Ok((id, username, role)) => Ok((new_session(c, id)?, username, role)),
            Err(msg) => Err(msg),
        })
    })
    .await?;
    match made {
        Ok(signed_in) => Ok(Some(signed_in)),
        Err(msg) => Err(AppError::forbidden(msg)),
    }
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> ApiResult<Response> {
    // Over HTTPS the browser may hold both cookies (from before and after HTTPS was turned on): end both.
    let secure = over_https(&headers);
    let tokens: Vec<String> =
        [secure.then(|| cookie_value(&headers, SECURE_COOKIE)).flatten(), cookie_value(&headers, COOKIE)]
            .into_iter()
            .flatten()
            .collect();
    db::call(&state.pool, move |c| {
        for token in &tokens {
            c.execute("DELETE FROM sessions WHERE token_hash = ?1", [digest(token)])?;
        }
        Ok(())
    })
    .await?;
    let mut res = StatusCode::NO_CONTENT.into_response();
    res.headers_mut().append(header::SET_COOKIE, set_cookie(false, "", 0)?);
    if secure {
        res.headers_mut().append(header::SET_COOKIE, set_cookie(true, "", 0)?);
    }
    Ok(res)
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
        let (hash, source): (String, String) =
            c.query_row("SELECT password_hash, source FROM users WHERE id = ?1", [id], |r| Ok((r.get(0)?, r.get(1)?)))?;
        if source != "internal" {
            return Ok(None);
        }
        if !password_matches(&body.current, &hash) {
            return Ok(Some(false));
        }
        c.execute("UPDATE users SET password_hash = ?2 WHERE id = ?1", params![id, hash_password(&body.new)?])?;
        c.execute("DELETE FROM sessions WHERE user_id = ?1 AND token_hash != ?2", params![id, keep])?;
        Ok(Some(true))
    })
    .await?;
    match changed {
        None => Err(AppError::bad_request("Your password is kept by the directory or provider you sign in with")),
        Some(false) => Err(AppError::bad_request("Your current password isn't right")),
        Some(true) => Ok(StatusCode::NO_CONTENT),
    }
}

async fn list_users(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let me = who.person()?;
    let users = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare("SELECT id, username, created_at, role, source FROM users ORDER BY username")?;
        let rows = stmt
            .query_map([], |r| {
                let id: i64 = r.get(0)?;
                Ok(json!({
                    "id": id,
                    "username": r.get::<_, String>(1)?,
                    "created_at": r.get::<_, String>(2)?,
                    "role": Role::stored(&r.get::<_, String>(3)?).as_str(),
                    "source": r.get::<_, String>(4)?,
                    "you": id == me,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!({ "users": users })))
}

#[derive(Deserialize)]
struct NewAccount {
    username: String,
    /// Not for accounts that sign in through a directory or OAuth.
    #[serde(default)]
    password: String,
    /// `internal` (the default), `ldap` or `oauth`: who checks the account's sign-in.
    source: Option<String>,
    /// `view`, `edit` (the default) or `admin`.
    role: Option<String>,
}

async fn create_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(body): Json<NewAccount>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    who.person()?;
    let source = match body.source.as_deref() {
        None => Source::Internal,
        Some(s) => Source::parse(s).ok_or_else(|| {
            AppError::bad_request("An account signs in with Hoard (internal), a directory (ldap) or OAuth")
        })?,
    };
    let internal = source == Source::Internal;
    let name = if internal { check_new(&body.username, &body.password)? } else { check_name(&body.username)? };
    let role = Role::requested(body.role.as_deref(), Role::Edit)?;
    let username = name.clone();
    let added = db::call(&state.pool, move |c| {
        let hash = if internal { hash_password(&body.password)? } else { String::new() };
        let n = c.execute(
            "INSERT INTO users (username, password_hash, role, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT DO NOTHING",
            params![username, hash, role.as_str(), source.as_str(), db::now()],
        )?;
        Ok((n == 1).then(|| c.last_insert_rowid()))
    })
    .await?;
    let Some(id) = added else {
        return Err(AppError::conflict("There's already an account with that name"));
    };
    let made = json!({ "id": id, "username": name, "role": role.as_str(), "source": source.as_str() });
    Ok((StatusCode::CREATED, Json(made)))
}

#[derive(Deserialize)]
struct AccountChange {
    role: String,
}

/// Give an account another role. Not your own: there's always an admin, because the one doing this is one.
async fn patch_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
    Json(body): Json<AccountChange>,
) -> ApiResult<StatusCode> {
    if who.person()? == id {
        return Err(AppError::bad_request("Another admin can change your role"));
    }
    let role = Role::requested(Some(&body.role), Role::View)?;
    let n = db::call(&state.pool, move |c| {
        Ok(c.execute("UPDATE users SET role = ?2 WHERE id = ?1", params![id, role.as_str()])?)
    })
    .await?;
    if n == 0 {
        return Err(AppError::not_found("account"));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    if who.person()? == id {
        return Err(AppError::bad_request("You can't remove your own account while you're signed in to it"));
    }
    // Their tokens and connected apps go with them.
    let n = db::call(&state.pool, move |c| {
        sign_out_everywhere(c, id)?;
        Ok(c.execute("DELETE FROM users WHERE id = ?1", [id])?)
    })
    .await?;
    if n == 0 {
        return Err(AppError::not_found("account"));
    }
    Ok(StatusCode::NO_CONTENT)
}

/// End everything `user_id` is signed in with: sessions, access tokens and connected apps.
pub(crate) fn sign_out_everywhere(c: &Connection, user_id: i64) -> rusqlite::Result<()> {
    c.execute("DELETE FROM sessions WHERE user_id = ?1", [user_id])?;
    c.execute("DELETE FROM access_tokens WHERE created_by = ?1", [user_id])?;
    c.execute("DELETE FROM oauth_grants WHERE user_id = ?1", [user_id])?;
    Ok(())
}

/// Whose tokens and connected apps `who` may see and revoke: everyone's for an admin (`None`), else their own.
pub(crate) fn owner_filter(who: &Who) -> ApiResult<Option<i64>> {
    let me = who.person()?;
    Ok((who.role() < Role::Admin).then_some(me))
}

async fn list_tokens(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let only = owner_filter(&who)?;
    let tokens = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare(
            "SELECT t.id, t.name, t.created_at, t.last_used_at, u.username, t.role
               FROM access_tokens t LEFT JOIN users u ON u.id = t.created_by
              WHERE ?1 IS NULL OR t.created_by = ?1 ORDER BY t.created_at DESC",
        )?;
        let rows = stmt
            .query_map([only], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "created_at": r.get::<_, String>(2)?,
                    "last_used_at": r.get::<_, Option<String>>(3)?,
                    "created_by": r.get::<_, Option<String>>(4)?,
                    "role": Role::stored(&r.get::<_, String>(5)?).as_str(),
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
    /// Up to the maker's own role, which is the default.
    role: Option<String>,
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
    let role = Role::requested(body.role.as_deref(), who.role())?;
    if role > who.role() {
        return Err(AppError::forbidden("A token can't do more than the account that makes it"));
    }
    let token = format!("hoard_{}", tools::hex(&random_bytes::<32>()?));
    let (hash, label) = (digest(&token), name.clone());
    let id = db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO access_tokens (name, token_hash, created_by, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![label, hash, me, role.as_str(), db::now()],
        )?;
        Ok(c.last_insert_rowid())
    })
    .await?;
    Ok(Json(json!({ "id": id, "name": name, "token": token, "role": role.as_str() })))
}

async fn delete_token(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    let only = owner_filter(&who)?;
    let n = db::call(&state.pool, move |c| {
        Ok(c.execute("DELETE FROM access_tokens WHERE id = ?1 AND (?2 IS NULL OR created_by = ?2)", params![id, only])?)
    })
    .await?;
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
    // From then on it signs in with this password, even if a directory or provider checked it before.
    c.execute(
        "UPDATE users SET password_hash = ?2, source = 'internal' WHERE id = ?1",
        params![id, hash_password(&password)?],
    )?;
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
        // Over HTTPS the secure cookie wins; over http:// it's never read.
        let mut h = HeaderMap::new();
        h.insert(header::COOKIE, HeaderValue::from_static("hoard_session=plain; __Host-hoard_session=secure"));
        assert_eq!(session_token(&h).as_deref(), Some("plain"));
        h.insert("x-forwarded-proto", HeaderValue::from_static("https"));
        assert_eq!(session_token(&h).as_deref(), Some("secure"));
        assert!(cookie(&h, "t", 1).unwrap().to_str().unwrap().starts_with("__Host-hoard_session=t; Path=/;"));
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
