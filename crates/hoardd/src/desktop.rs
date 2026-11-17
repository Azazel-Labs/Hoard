//! The desktop app: Hoard for one person, in a window on their own computer.
//!
//! The app runs this server inside itself on a loopback address and shows the portal in its window, so the engine
//! plugins, the `hoard` CLI and MCP clients reach it the same way they reach a server (with an access token made in
//! Settings → Account). What's different:
//!
//! - **One account, made for you.** The first start makes the owner's account (named after the person signed in to
//!   the computer) with a password nobody knows: nobody signs in by typing one.
//! - **The window signs itself in.** Each time the app opens a window it asks [`launch_url`] for an address with a
//!   one-time code; loading it gives the window a session as the owner and goes on to the library.
//! - **This computer only.** It listens on a loopback address, and turns away requests whose `Host` isn't one, so a
//!   web page can't reach it by pointing a name of its own at 127.0.0.1 (DNS rebinding).

use std::net::SocketAddr;
use std::sync::{Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use anyhow::Result;
use axum::Router;
use axum::extract::{Query, Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use rusqlite::{Connection, OptionalExtension as _, params};
use serde::Deserialize;
use tokio::sync::Notify;

use crate::auth::{digest, hash_password, new_session, random_bytes, session_cookie};
use crate::{AppState, db, tools};

/// How long a launch address works for, once.
const LAUNCH_CODE_LIFE: Duration = Duration::from_mins(2);

/// Where the server listens, once it does.
static LISTENING: OnceLock<SocketAddr> = OnceLock::new();
static LISTENING_NOW: Notify = Notify::const_new();
/// Launch codes not used yet, by digest.
static CODES: Mutex<Vec<(String, Instant)>> = Mutex::new(Vec::new());

/// The server is listening on `addr`.
pub(crate) fn listening_on(addr: SocketAddr) {
    LISTENING.set(addr).ok();
    LISTENING_NOW.notify_waiters();
}

/// Where the server listens, once it does.
pub async fn listening() -> SocketAddr {
    loop {
        let notified = LISTENING_NOW.notified();
        if let Some(addr) = LISTENING.get() {
            return *addr;
        }
        notified.await;
    }
}

/// An address that signs a window in as the owner and opens the library: it works once, for two minutes. `None`
/// until the server is listening.
pub fn launch_url() -> Option<String> {
    let addr = LISTENING.get()?;
    let code = tools::hex(&random_bytes::<32>().ok()?);
    let mut codes = CODES.lock().unwrap_or_else(PoisonError::into_inner);
    codes.retain(|(_, made)| made.elapsed() < LAUNCH_CODE_LIFE);
    codes.push((digest(&code), Instant::now()));
    Some(format!("http://{addr}/api/desktop/launch?code={code}"))
}

/// Whether `code` is one [`launch_url`] gave out, still unused and in time. Using it uses it up.
fn take_code(code: &str) -> bool {
    let wanted = digest(code);
    let mut codes = CODES.lock().unwrap_or_else(PoisonError::into_inner);
    codes.retain(|(_, made)| made.elapsed() < LAUNCH_CODE_LIFE);
    let before = codes.len();
    codes.retain(|(d, _)| *d != wanted);
    codes.len() < before
}

/// Make the owner's account on the first start.
pub(crate) fn ensure_owner(c: &Connection) -> Result<()> {
    let any: Option<i64> = c.query_row("SELECT id FROM users LIMIT 1", [], |r| r.get(0)).optional()?;
    if any.is_some() {
        return Ok(());
    }
    let name = owner_name();
    // A password nobody is told: the window signs in with launch codes.
    let hash = hash_password(&tools::hex(&random_bytes::<32>()?))?;
    c.execute(
        "INSERT INTO users (username, password_hash, role, created_at) VALUES (?1, ?2, 'admin', ?3)",
        params![name, hash, db::now()],
    )?;
    tracing::info!(user = %name, "made the owner's account");
    Ok(())
}

/// The name of whoever is signed in to this computer, as an account name.
fn owner_name() -> String {
    ["USERNAME", "USER", "LOGNAME"]
        .iter()
        .filter_map(|k| std::env::var(k).ok())
        .map(|n| n.trim().chars().filter(|c| !c.is_control()).take(64).collect::<String>())
        .find(|n| !n.is_empty())
        .unwrap_or_else(|| "Owner".to_owned())
}

/// Open to everyone: the launch address.
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/api/desktop/launch", get(launch))
}

#[derive(Deserialize)]
struct Launch {
    code: String,
}

/// Sign the window in as the owner (the first admin), then show the library.
async fn launch(State(state): State<AppState>, headers: HeaderMap, Query(q): Query<Launch>) -> Response {
    if !take_code(&q.code) {
        // An old address (a reload, or a window left open past two minutes): the session from before still works.
        return Redirect::to("/").into_response();
    }
    let token = db::call(&state.pool, |c| {
        let owner: Option<i64> = c
            .query_row("SELECT id FROM users WHERE role = 'admin' ORDER BY id LIMIT 1", [], |r| r.get(0))
            .optional()?;
        owner.map(|id| new_session(c, id)).transpose()
    })
    .await;
    match token.and_then(|t| {
        let t = t.ok_or_else(|| anyhow::anyhow!("no owner account"))?;
        session_cookie(&headers, &t)
    }) {
        Ok(cookie) => ([(header::SET_COOKIE, cookie)], Redirect::to("/")).into_response(),
        Err(e) => {
            tracing::warn!(error = %format!("{e:#}"), "couldn't sign the window in");
            Redirect::to("/").into_response()
        }
    }
}

/// Turn away requests that name a host other than this computer (DNS rebinding).
pub(crate) async fn loopback_only(req: Request, next: Next) -> Response {
    let host = req.headers().get(header::HOST).and_then(|h| h.to_str().ok()).unwrap_or_default();
    if is_loopback_host(host) { next.run(req).await } else { StatusCode::MISDIRECTED_REQUEST.into_response() }
}

/// `127.0.0.1`, `[::1]` or `localhost`, with or without a port.
fn is_loopback_host(host: &str) -> bool {
    let name = if let Some(rest) = host.strip_prefix('[') {
        rest.split_once(']').map_or("", |(name, _)| name)
    } else {
        host.rsplit_once(':').map_or(host, |(name, _)| name)
    };
    name.eq_ignore_ascii_case("localhost") || name.parse::<std::net::IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

/// Ask the server to stop, as Ctrl+C would: [`crate::run`] returns once open requests finish.
pub fn shut_down() {
    crate::server::STOP.notify_one();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_hosts() {
        for ok in ["127.0.0.1:4627", "localhost:4627", "LOCALHOST", "[::1]:80", "127.0.0.1", "127.1.2.3:9"] {
            assert!(is_loopback_host(ok), "{ok}");
        }
        for no in ["", "evil.example:4627", "192.168.0.2:4627", "localhost.evil.example", "[::2]:1"] {
            assert!(!is_loopback_host(no), "{no}");
        }
    }

    #[test]
    fn launch_codes_work_once() {
        LISTENING.set("127.0.0.1:1".parse().unwrap()).ok();
        let url = launch_url().unwrap();
        let code = url.rsplit_once("code=").unwrap().1;
        assert!(take_code(code));
        assert!(!take_code(code));
        assert!(!take_code("made up"));
    }

    #[test]
    fn owner_is_made_once() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE,
               password_hash TEXT NOT NULL, role TEXT NOT NULL, created_at TEXT NOT NULL)",
        )
        .unwrap();
        ensure_owner(&c).unwrap();
        ensure_owner(&c).unwrap();
        let (n, role): (i64, String) =
            c.query_row("SELECT COUNT(*), MAX(role) FROM users", [], |r| Ok((r.get(0)?, r.get(1)?))).unwrap();
        assert_eq!((n, role.as_str()), (1, "admin"));
    }
}
