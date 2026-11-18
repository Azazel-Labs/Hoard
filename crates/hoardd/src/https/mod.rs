//! HTTPS, set up in Settings → Connection.
//!
//! Hoard answers `https://` and `http://` on the same port: the first byte of a connection says which (a TLS
//! handshake starts with 0x16, an HTTP request with a letter; see `server.rs`). So turning HTTPS on or off, or
//! changing the certificate, takes effect from the next connection, without a restart.
//!
//! - **Hoard's own certificate**: Hoard makes a certificate authority once (`<data dir>/tls/ca.crt` and `ca.key`)
//!   and issues the server's certificate from it (`server.crt`, `server.key`) for the addresses it finds (this
//!   computer's name and LAN addresses, the public address in Settings) plus any the admin adds. Install the
//!   authority on a device once and its browsers trust Hoard. The server certificate is renewed, and re-issued when
//!   the addresses change, without anyone noticing.
//! - **My certificate**: a PEM chain and key pasted or uploaded in Settings (`custom.crt`, `custom.key`).
//!
//! Once HTTPS is on, plain `http://` keeps serving everything except page loads by browsers on other devices, which
//! are sent to `https://` (the admin can turn that off). The `hoard` CLI, the Unreal worker and scripts keep working
//! with the address they were given, and `http://localhost` always works, so nobody is locked out by a certificate
//! their browser doesn't trust. `hoardd --https-off` is the last resort.

mod certs;
mod names;
mod routes;

use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock, PoisonError, RwLock};
use std::time::Duration;

use anyhow::{Context, Result};
use axum::Router;
use axum::http::{HeaderMap, HeaderValue, Method, Request, header};
use rusqlite::{Connection, OptionalExtension};
use rustls::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::AppState;
use crate::i18n::tr;
pub(crate) use certs::server_config;
use certs::{build, own_needs_issue};
use names::{clean_name, wanted_names};
pub(crate) use routes::{admin_router, router};

/// The row in `settings` that holds [`Saved`].
const KEY: &str = "https";
/// Hoard's server certificate lasts this long, and is renewed when it has [`RENEW_DAYS`] left.
const ISSUE_DAYS: i64 = 397;
const RENEW_DAYS: i64 = 30;
/// How often addresses and expiry dates are looked at again.
const REFRESH: Duration = Duration::from_hours(6);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Mode {
    #[default]
    Off,
    /// Issued by Hoard's own certificate authority.
    Own,
    /// Pasted or uploaded in Settings.
    Custom,
}

/// What's chosen in Settings → Connection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct Saved {
    pub(crate) mode: Mode,
    /// Addresses the admin added for Hoard's own certificate, on top of the ones found.
    pub(crate) names: Vec<String>,
    /// Also serve HTTP/3 (QUIC, UDP on the same port).
    pub(crate) http3: bool,
    /// Send browsers that load a page over `http://` from another device to `https://`.
    pub(crate) redirect: bool,
}

impl Default for Saved {
    fn default() -> Self {
        Self { mode: Mode::Off, names: Vec::new(), http3: false, redirect: true }
    }
}

/// A certificate, as Settings shows it.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct CertInfo {
    pub(crate) names: Vec<String>,
    pub(crate) subject: String,
    pub(crate) issuer: String,
    /// Unix seconds.
    pub(crate) not_before: i64,
    pub(crate) not_after: i64,
    pub(crate) self_signed: bool,
    /// SHA-256, as browsers and `openssl x509 -fingerprint -sha256` show it.
    pub(crate) fingerprint: String,
}

/// A certificate chain and its key, checked and ready to serve.
struct Built {
    chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
    info: CertInfo,
}

/// What's being served now.
#[derive(Debug, Default)]
struct Live {
    saved: Saved,
    /// For TCP connections (ALPN `h2`, `http/1.1`); `None` while HTTPS is off or couldn't start.
    tls: Option<Arc<ServerConfig>>,
    /// The same certificate for QUIC (ALPN `h3`).
    h3_tls: Option<Arc<ServerConfig>>,
    cert: Option<CertInfo>,
    /// Why HTTPS isn't on although it's chosen (only at startup: a change that doesn't work is refused).
    error: Option<String>,
    h3_error: Option<String>,
}

#[derive(Debug)]
struct Https {
    dir: PathBuf,
    listen: SocketAddr,
    live: RwLock<Live>,
    /// The HTTP/3 listener while it runs.
    h3: Mutex<Option<quinn::Endpoint>>,
    /// The app, for the HTTP/3 listener (set once the server is listening).
    app: OnceLock<Router>,
    /// One change (or renewal) at a time.
    change: tokio::sync::Mutex<()>,
}

static HTTPS: OnceLock<Https> = OnceLock::new();

impl Https {
    fn live(&self) -> std::sync::RwLockReadGuard<'_, Live> {
        self.live.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn live_mut(&self) -> std::sync::RwLockWriteGuard<'_, Live> {
        self.live.write().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Whether HTTPS is on.
pub(crate) fn on() -> bool {
    tls_config().is_some()
}

/// The TLS config for a new TCP connection, or `None` while HTTPS is off.
pub(crate) fn tls_config() -> Option<Arc<ServerConfig>> {
    HTTPS.get()?.live().tls.clone()
}

/// Where to send a browser that loads a page over plain HTTP, when HTTPS is on and set to do that. Never for
/// this computer (`localhost`), API clients (the CLI, the Unreal worker, scripts, MCP), or requests through a
/// reverse proxy (it decides).
pub(crate) fn redirect_for<B>(req: &Request<B>, remote: SocketAddr) -> Option<String> {
    let h = HTTPS.get()?;
    {
        let live = h.live();
        if live.tls.is_none() || !live.saved.redirect {
            return None;
        }
    }
    if is_loopback(remote.ip()) || !is_page_load(req.method(), req.headers()) {
        return None;
    }
    let headers = req.headers();
    if ["x-forwarded-proto", "x-forwarded-for", "forwarded"].iter().any(|n| headers.contains_key(*n)) {
        return None;
    }
    let path = req.uri().path();
    let api = ["/api/", "/mcp", "/oauth/", "/auth/", "/.well-known/"].iter().any(|p| path.starts_with(p));
    if api {
        return None;
    }
    let host = authority(req)?;
    // http://localhost reached through Docker's port mapping comes from the bridge's address, not loopback.
    let name = clean_name(&host).ok()?;
    if name == "localhost" || name.ends_with(".localhost") || name.parse::<IpAddr>().is_ok_and(is_loopback) {
        return None;
    }
    let pq = req.uri().path_and_query().map_or("/", |p| p.as_str());
    Some(format!("https://{host}{pq}"))
}

/// `Alt-Svc` advertising HTTP/3 while it runs, on the port the browser used (with Docker that can differ from
/// the one Hoard listens on; the UDP port is expected to be mapped the same way).
pub(crate) fn alt_svc<B>(req: &Request<B>) -> Option<HeaderValue> {
    let h = HTTPS.get()?;
    let running = h.h3.lock().unwrap_or_else(PoisonError::into_inner).is_some();
    if !running {
        return None;
    }
    let port =
        authority(req).and_then(|a| a.parse::<http::uri::Authority>().ok()).and_then(|a| a.port_u16()).unwrap_or(443);
    HeaderValue::from_str(&format!("h3=\":{port}\"; ma=86400")).ok()
}

/// The host (and port) the client asked for: the URI's authority for HTTP/2 and 3, the Host header for HTTP/1.1.
fn authority<B>(req: &Request<B>) -> Option<String> {
    if let Some(a) = req.uri().authority() {
        return Some(a.as_str().to_owned());
    }
    req.headers().get(header::HOST).and_then(|v| v.to_str().ok()).map(str::to_owned)
}

fn is_loopback(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_loopback(),
        IpAddr::V6(v6) => v6.is_loopback() || v6.to_ipv4_mapped().is_some_and(|v4| v4.is_loopback()),
    }
}

/// A browser loading a page (not a script, an image or an API call).
fn is_page_load(method: &Method, headers: &HeaderMap) -> bool {
    if method != Method::GET && method != Method::HEAD {
        return false;
    }
    let get = |n: &str| headers.get(n).and_then(|v| v.to_str().ok());
    match get("sec-fetch-mode") {
        Some(mode) => mode.eq_ignore_ascii_case("navigate"),
        None => get("accept").is_some_and(|a| a.contains("text/html")),
    }
}

/// Read what's saved and put it in effect. A certificate that can't be loaded leaves HTTPS off (with the reason
/// in Settings) rather than stopping the server.
pub(crate) fn init(state: &AppState) -> Result<()> {
    let saved = load(&*state.pool.get()?)?;
    let h = HTTPS.get_or_init(|| Https {
        dir: state.cfg.data_dir.join("tls"),
        listen: state.cfg.listen,
        live: RwLock::default(),
        h3: Mutex::new(None),
        app: OnceLock::new(),
        change: tokio::sync::Mutex::new(()),
    });
    let public_url = state.settings.get().public_url.clone();
    let result = build(&h.dir, &saved, public_url.as_deref(), None).and_then(|b| activate(h, saved.clone(), b));
    match result {
        Ok(()) => {
            if let Some(cert) = &h.live().cert {
                tracing::info!(names = ?cert.names, http3 = saved.http3, "HTTPS is on (Settings → Connection)");
            }
        }
        Err(e) => {
            let msg = format!("{e:#}");
            tracing::error!(error = %msg, "HTTPS is set up but couldn't start; serving plain http:// meanwhile");
            let mut live = h.live_mut();
            live.saved = saved;
            live.error = Some(msg);
        }
    }
    let st = Arc::clone(state);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(REFRESH).await;
            refresh(&st).await;
        }
    });
    Ok(())
}

/// The server is listening: remember the app (for HTTP/3) and start HTTP/3 if it's on.
pub(crate) fn serving(app: Router) {
    if let Some(h) = HTTPS.get() {
        h.app.set(app).ok();
        apply_h3(h);
    }
}

/// Stop the HTTP/3 listener (shutting down).
pub(crate) fn stop() {
    if let Some(ep) = HTTPS.get().and_then(|h| h.h3.lock().unwrap_or_else(PoisonError::into_inner).take()) {
        ep.close(0u32.into(), b"shutting down");
    }
}

/// `hoardd --https-off`: back to plain HTTP at the next start.
pub(crate) fn turn_off(conn: &Connection) -> Result<()> {
    let saved = Saved { mode: Mode::Off, http3: false, ..load(conn)? };
    save(conn, &saved)
}

/// Renew Hoard's own certificate when it's due or the addresses changed; warn about an uploaded one running out.
async fn refresh(state: &AppState) {
    let Some(h) = HTTPS.get() else { return };
    let _one = h.change.lock().await;
    let saved = h.live().saved.clone();
    match saved.mode {
        Mode::Off => {}
        Mode::Custom => {
            if let Some(cert) = &h.live().cert {
                let days = (cert.not_after - OffsetDateTime::now_utc().unix_timestamp()) / 86_400;
                if days < 14 {
                    tracing::warn!(
                        days,
                        "your HTTPS certificate runs out soon: upload a new one in Settings → Connection"
                    );
                }
            }
        }
        Mode::Own => {
            let public_url = state.settings.get().public_url.clone();
            let dir = h.dir.clone();
            let wanted = wanted_names(&saved, public_url.as_deref());
            let fresh = tokio::task::spawn_blocking(move || own_needs_issue(&dir, &wanted)).await;
            if matches!(fresh, Ok(false)) && h.live().tls.is_some() {
                return;
            }
            let (dir, wanted) = (h.dir.clone(), saved.clone());
            let built = tokio::task::spawn_blocking(move || build(&dir, &wanted, public_url.as_deref(), None)).await;
            match built.map_err(anyhow::Error::from).and_then(|b| b).and_then(|b| activate(h, saved, b)) {
                Ok(()) => tracing::info!("renewed Hoard's HTTPS certificate"),
                Err(e) => tracing::warn!(error = %format!("{e:#}"), "couldn't renew Hoard's HTTPS certificate"),
            }
        }
    }
}

/// Serve `built` (already checked) for `saved`.
fn activate(h: &Https, saved: Saved, built: Option<Built>) -> Result<()> {
    let (tls, h3_tls, cert) = match built {
        Some(b) => (
            Some(Arc::new(server_config(&b.chain, &b.key, &[b"h2", b"http/1.1"])?)),
            Some(Arc::new(server_config(&b.chain, &b.key, &[b"h3"])?)),
            Some(b.info),
        ),
        None => (None, None, None),
    };
    {
        let mut live = h.live_mut();
        live.saved = saved;
        live.tls = tls;
        live.h3_tls = h3_tls;
        live.cert = cert;
        live.error = None;
    }
    apply_h3(h);
    Ok(())
}

/// Start, update or stop the HTTP/3 listener to match what's chosen.
fn apply_h3(h: &Https) {
    let (want, tls) = {
        let live = h.live();
        (live.saved.http3 && live.h3_tls.is_some(), live.h3_tls.clone())
    };
    let mut slot = h.h3.lock().unwrap_or_else(PoisonError::into_inner);
    let result = match (want, tls, slot.as_ref()) {
        (true, Some(tls), Some(ep)) => crate::http3::quic_config(tls).map(|q| ep.set_server_config(Some(q))),
        (true, Some(tls), None) => match h.app.get() {
            Some(app) => crate::http3::start_h3(h.listen, tls, app.clone()).map(|ep| *slot = Some(ep)),
            None => Ok(()),
        },
        _ => {
            if let Some(ep) = slot.take() {
                ep.close(0u32.into(), b"HTTP/3 turned off");
                tracing::info!("HTTP/3 stopped");
            }
            Ok(())
        }
    };
    drop(slot);
    h.live_mut().h3_error = result.err().map(|e| format!("{e:#}"));
}

/// A TLS handshake with `config`, in memory: proves the key works for real before anyone is switched over.
async fn try_handshake(config: Arc<ServerConfig>) -> Result<()> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut client = rustls::ClientConfig::builder_with_provider(Arc::clone(&provider))
        .with_safe_default_protocol_versions()?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(crate::signature_only::SignatureOnly(provider)))
        .with_no_client_auth();
    client.alpn_protocols = vec![b"h2".to_vec()];
    let (a, b) = tokio::io::duplex(64 << 10);
    let server = tokio_rustls::TlsAcceptor::from(config).accept(a);
    let client = tokio_rustls::TlsConnector::from(Arc::new(client)).connect(ServerName::try_from("hoard.test")?, b);
    let (s, c) = tokio::time::timeout(Duration::from_secs(10), async { tokio::join!(server, client) })
        .await
        .with_context(|| tr!("error-https-test-timed-out"))?;
    s.with_context(|| tr!("error-https-test-failed"))?;
    c.with_context(|| tr!("error-https-test-failed"))?;
    Ok(())
}

fn load(conn: &Connection) -> Result<Saved> {
    let text: Option<String> =
        conn.query_row("SELECT value FROM settings WHERE key = ?1", [KEY], |r| r.get(0)).optional()?;
    Ok(text.and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default())
}

fn save(conn: &Connection, saved: &Saved) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT (key) DO UPDATE SET value = excluded.value",
        [KEY, serde_json::to_string(saved)?.as_str()],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_loads_are_told_apart_from_api_calls() {
        let mut h = HeaderMap::new();
        h.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
        assert!(is_page_load(&Method::GET, &h));
        assert!(!is_page_load(&Method::POST, &h));
        h.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        assert!(!is_page_load(&Method::GET, &h));
        let mut old = HeaderMap::new();
        old.insert(header::ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml"));
        assert!(is_page_load(&Method::GET, &old));
        assert!(!is_page_load(&Method::GET, &HeaderMap::new()), "curl and the CLI aren't sent anywhere");
    }
}
