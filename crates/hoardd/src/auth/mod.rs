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

mod accounts;
mod signing_in;

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher as _, PasswordVerifier as _};
use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, Method, header};
use axum::middleware::Next;
use axum::response::Response;
use rusqlite::{Connection, OptionalExtension as _, params};
use serde_json::{Value, json};
use sha2::Digest as _;

use crate::AppState;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::signin::Gate;
use crate::{db, tools};

pub(crate) use accounts::operations;
pub(crate) use signing_in::router;

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
            Some(s) => Self::parse(s).ok_or_else(|| AppError::bad_request(tr!("error-role"))),
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
            Role::Admin => tr!("error-admin-only"),
            _ => tr!("error-view-only"),
        }))
    }

    /// The account's name, or the access token's.
    pub(crate) fn name(&self) -> &str {
        match self {
            Self::User { name, .. } | Self::Token { name, .. } => name,
        }
    }

    /// Whether this is an access token or connected app rather than someone signed in.
    pub(crate) const fn is_token(&self) -> bool {
        matches!(self, Self::Token { .. })
    }

    /// The account this is or acts for: whose favorites and saved searches are meant.
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
            Self::Token { .. } => Err(AppError::forbidden(tr!("error-portal-only"))),
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

pub(crate) fn hash_password(password: &str) -> Result<String> {
    Ok(Argon2::default().hash_password(password.as_bytes()).map_err(|e| anyhow!("hash password: {e}"))?.to_string())
}

fn password_matches(password: &str, hash: &str) -> bool {
    Argon2::default().verify_password(password.as_bytes(), hash).is_ok()
}

/// Checked against when there's no such name, so a wrong name takes as long as a wrong password.
static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| hash_password("not a password").unwrap_or_default());

/// An access token's id, name and role, its maker's role, id and where they sign in, and whether its use is due to
/// be noted.
type TokenRow = (i64, String, String, String, i64, String, bool);

/// Whether a use noted in `column` is old enough to note again: at most once a minute. Asked in the `SELECT`,
/// because any `UPDATE`, even one whose `WHERE` matches nothing, waits for the write lock, so a request that only
/// reads would wait out an import's write.
pub(crate) fn use_due(column: &str) -> String {
    format!("({column} IS NULL OR unixepoch({column}) < unixepoch('now') - 60)")
}

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
                    &format!(
                        "SELECT t.id, t.name, t.role, u.role, u.id, u.source, {}
                           FROM access_tokens t JOIN users u ON u.id = t.created_by WHERE t.token_hash = ?1",
                        use_due("t.last_used_at")
                    ),
                    [&hash],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
                )
                .optional()?;
            let Some((id, name, own, maker, owner, source, due)) = row else {
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
            if due {
                c.execute("UPDATE access_tokens SET last_used_at = ?2 WHERE id = ?1", params![id, db::now()])?;
            }
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
        return Err(AppError::forbidden(tr!("error-not-from-portal")));
    }
    let gate = state.settings.get().auth.gate();
    Ok(db::call(&state.pool, move |c| lookup(c, &cred, gate)).await?)
}

async fn require(state: &AppState, role: Role, mut req: Request, next: Next) -> ApiResult<Response> {
    let who = match authenticate(state, req.method(), req.headers()).await? {
        Some(who) => who,
        None => crate::uploads::keyed(state, req.method(), req.uri().path(), req.headers())
            .await?
            .ok_or_else(AppError::unauthorized)?,
    };
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

/// A name and password fit to save, or what's wrong with them.
fn check_new(username: &str, password: &str) -> ApiResult<String> {
    let name = check_name(username)?;
    check_password(password)?;
    Ok(name)
}

fn check_name(username: &str) -> ApiResult<String> {
    let name = username.trim();
    if name.is_empty() || name.chars().count() > 64 || name.chars().any(char::is_control) {
        return Err(AppError::bad_request(tr!("error-account-name")));
    }
    Ok(name.to_owned())
}

fn check_password(password: &str) -> ApiResult<()> {
    if password.chars().count() < MIN_PASSWORD {
        return Err(AppError::bad_request(tr!("error-password-length", min = MIN_PASSWORD)));
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

    /// A token used a moment ago signs in without writing, so it doesn't wait for an import holding the write lock.
    #[test]
    fn a_token_signs_in_while_something_else_writes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("hoard.db");
        let pool = db::open(&path, &dir.path().join("backups")).unwrap();
        let token = "hoard_0123456789abcdef0123456789abcdef";
        pool.get()
            .unwrap()
            .execute_batch(&format!(
                "INSERT INTO users (id, username, password_hash, created_at, role) VALUES (1, 'a', 'x', 'x', 'admin');
                 INSERT INTO access_tokens (name, token_hash, created_by, created_at, last_used_at, role)
                 VALUES ('engine', '{}', 1, 'x', '{}', 'view');",
                digest(token),
                db::now()
            ))
            .unwrap();
        let writer = Connection::open(&path).unwrap();
        writer.execute_batch("BEGIN IMMEDIATE").unwrap();
        let started = Instant::now();
        let gate = crate::signin::AuthConfig::default().gate();
        let who = lookup(&pool.get().unwrap(), &Credential::Bearer(token.into()), gate).unwrap();
        assert!(matches!(who, Some(Who::Token { ref name, .. }) if name == "engine"));
        assert!(started.elapsed() < Duration::from_secs(1), "waited {:?}", started.elapsed());
        writer.execute_batch("ROLLBACK").unwrap();
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
