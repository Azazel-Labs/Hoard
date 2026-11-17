//! Built without the `https` feature (the desktop app, which only answers this computer): plain HTTP only, no
//! certificates and no Settings → Connection.

use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use axum::http::{HeaderValue, Request};
use rusqlite::Connection;

use crate::AppState;

pub(crate) const fn on() -> bool {
    false
}

pub(crate) const fn redirect_for<B>(_req: &Request<B>, _remote: SocketAddr) -> Option<String> {
    None
}

pub(crate) const fn alt_svc<B>(_req: &Request<B>) -> Option<HeaderValue> {
    None
}

#[expect(clippy::unnecessary_wraps, reason = "the same signature as with the `https` feature")]
pub(crate) const fn init(_state: &AppState) -> Result<()> {
    Ok(())
}

pub(crate) fn serving(_app: Router) {}

pub(crate) const fn stop() {}

#[expect(clippy::unnecessary_wraps, reason = "the same signature as with the `https` feature")]
pub(crate) const fn turn_off(_conn: &Connection) -> Result<()> {
    Ok(())
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
}

pub(crate) fn admin_router() -> Router<AppState> {
    Router::new()
}
