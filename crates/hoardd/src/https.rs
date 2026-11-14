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

use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, PoisonError, RwLock};
use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, Method, Request, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use rusqlite::{Connection, OptionalExtension};
use rustls::ServerConfig;
use rustls_pki_types::pem::PemObject;
use rustls_pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::Digest as _;
use time::OffsetDateTime;

use crate::AppState;
use crate::error::{ApiResult, AppError};

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

// ---------------------------------------------------------------------------------------------
// What the server asks

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

// ---------------------------------------------------------------------------------------------
// Starting up

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
        (true, Some(tls), Some(ep)) => crate::server::quic_config(tls).map(|q| ep.set_server_config(Some(q))),
        (true, Some(tls), None) => match h.app.get() {
            Some(app) => crate::server::start_h3(h.listen, tls, app.clone()).map(|ep| *slot = Some(ep)),
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

// ---------------------------------------------------------------------------------------------
// Certificates

/// The certificate `saved` asks for: Hoard's own (made or renewed as needed), the uploaded one (`upload`, or the
/// one kept from before), or none.
fn build(dir: &Path, saved: &Saved, public_url: Option<&str>, upload: Option<&Upload>) -> Result<Option<Built>> {
    match saved.mode {
        Mode::Off => Ok(None),
        Mode::Own => {
            let names = wanted_names(saved, public_url);
            if own_needs_issue(dir, &names) {
                let (_, issuer) = load_or_make_ca(dir)?;
                issue(dir, &names, &issuer)?;
            }
            let built = read_pair(&dir.join("server.crt"), &dir.join("server.key"))?;
            Ok(Some(built))
        }
        Mode::Custom => {
            let built = if let Some(u) = upload {
                u.parse()?
            } else {
                let (c, k) = (dir.join("custom.crt"), dir.join("custom.key"));
                if !c.is_file() || !k.is_file() {
                    bail!("Paste or upload your certificate and its key first");
                }
                read_pair(&c, &k)?
            };
            check_dates(&built.info)?;
            Ok(Some(built))
        }
    }
}

fn read_pair(cert: &Path, key: &Path) -> Result<Built> {
    let upload = Upload {
        cert: std::fs::read_to_string(cert).with_context(|| format!("read {}", cert.display()))?,
        key: std::fs::read_to_string(key).with_context(|| format!("read {}", key.display()))?,
    };
    upload.parse()
}

/// A certificate chain and key as pasted or uploaded (in either box, or both in one).
#[derive(Debug)]
struct Upload {
    cert: String,
    key: String,
}

impl Upload {
    /// The PEM blocks: certificates (in order) and the private key.
    fn split(&self) -> Result<(String, String)> {
        let blocks = pem_blocks(&format!("{}\n{}", self.cert, self.key));
        let certs: Vec<&str> =
            blocks.iter().filter(|(label, _)| label == "CERTIFICATE").map(|(_, b)| b.as_str()).collect();
        let keys: Vec<&str> =
            blocks.iter().filter(|(label, _)| label.ends_with("PRIVATE KEY")).map(|(_, b)| b.as_str()).collect();
        if certs.is_empty() {
            bail!("No certificate found: paste the text that starts with -----BEGIN CERTIFICATE-----");
        }
        match keys.len() {
            0 => bail!("No private key found: paste the text that starts with -----BEGIN PRIVATE KEY-----"),
            1 => {}
            _ => bail!("There's more than one private key: paste only the one that goes with the certificate"),
        }
        if blocks.iter().any(|(label, _)| label == "ENCRYPTED PRIVATE KEY") {
            bail!(
                "The key is protected with a password: give it without one (openssl pkey -in key.pem -out plain.pem)"
            );
        }
        Ok((certs.join(""), keys.concat()))
    }

    fn parse(&self) -> Result<Built> {
        let (certs, key) = self.split()?;
        let chain = CertificateDer::pem_slice_iter(certs.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| anyhow!("The certificate isn't valid PEM"))?;
        let key =
            PrivateKeyDer::from_pem_slice(key.as_bytes()).map_err(|_| anyhow!("The private key isn't valid PEM"))?;
        let info = cert_info(&chain[0])?;
        server_config(&chain, &key, &[b"h2"]).map_err(|e| match e.downcast_ref::<rustls::Error>() {
            Some(rustls::Error::InconsistentKeys(_)) => anyhow!(
                "The key doesn't belong to the certificate. If you gave a chain, put your server's certificate first"
            ),
            _ => e.context("The certificate or key can't be used"),
        })?;
        Ok(Built { chain, key, info })
    }

    /// The certificate and key as they're kept (`custom.crt`, `custom.key`).
    fn store(&self, dir: &Path) -> Result<()> {
        let (certs, key) = self.split()?;
        std::fs::create_dir_all(dir)?;
        write_private(&dir.join("custom.key"), &key)?;
        std::fs::write(dir.join("custom.crt"), certs)?;
        Ok(())
    }
}

/// `-----BEGIN <label>-----` … `-----END <label>-----` blocks, as (label, text with a newline at the end).
fn pem_blocks(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut open: Option<(String, String)> = None;
    for line in text.lines().map(str::trim) {
        if let Some(label) = line.strip_prefix("-----BEGIN ").and_then(|l| l.strip_suffix("-----")) {
            open = Some((label.to_owned(), format!("{line}\n")));
        } else if let Some((label, mut body)) = open.take() {
            body.push_str(line);
            body.push('\n');
            if line.strip_prefix("-----END ").and_then(|l| l.strip_suffix("-----")) == Some(label.as_str()) {
                out.push((label, body));
            } else {
                open = Some((label, body));
            }
        }
    }
    out
}

fn check_dates(info: &CertInfo) -> Result<()> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    if info.not_after < now {
        bail!("This certificate expired on {}", day(info.not_after));
    }
    if info.not_before > now {
        bail!("This certificate isn't valid until {}", day(info.not_before));
    }
    Ok(())
}

fn day(unix: i64) -> String {
    OffsetDateTime::from_unix_timestamp(unix)
        .ok()
        .and_then(|t| t.format(time::macros::format_description!("[year]-[month]-[day]")).ok())
        .unwrap_or_else(|| unix.to_string())
}

pub(crate) fn server_config(
    chain: &[CertificateDer<'static>],
    key: &PrivateKeyDer<'static>,
    alpn: &[&[u8]],
) -> Result<ServerConfig> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut config = ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(chain.to_vec(), key.clone_key())?;
    config.alpn_protocols = alpn.iter().map(|p| p.to_vec()).collect();
    Ok(config)
}

fn cert_info(der: &[u8]) -> Result<CertInfo> {
    use x509_parser::extensions::GeneralName;
    let (_, x) =
        x509_parser::parse_x509_certificate(der).map_err(|_| anyhow!("That isn't a certificate Hoard can read"))?;
    let cn = |n: &x509_parser::x509::X509Name<'_>| {
        n.iter_common_name().next().and_then(|a| a.as_str().ok()).map_or_else(|| n.to_string(), str::to_owned)
    };
    let mut names = Vec::new();
    if let Ok(Some(san)) = x.subject_alternative_name() {
        for g in &san.value.general_names {
            match g {
                GeneralName::DNSName(d) => names.push((*d).to_owned()),
                GeneralName::IPAddress(b) => {
                    if let Ok(v4) = <[u8; 4]>::try_from(*b) {
                        names.push(IpAddr::from(v4).to_string());
                    } else if let Ok(v6) = <[u8; 16]>::try_from(*b) {
                        names.push(IpAddr::from(v6).to_string());
                    }
                }
                _ => {}
            }
        }
    }
    let fingerprint = sha2::Sha256::digest(der).iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(":");
    Ok(CertInfo {
        names,
        subject: cn(x.subject()),
        issuer: cn(x.issuer()),
        not_before: x.validity().not_before.timestamp(),
        not_after: x.validity().not_after.timestamp(),
        self_signed: x.subject().as_raw() == x.issuer().as_raw(),
        fingerprint,
    })
}

/// Whether a certificate for `names` covers `host` (`*.example.com` covers one label).
fn covers(names: &[String], host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    names.iter().any(|n| {
        let n = n.to_ascii_lowercase();
        n == host
            || n.strip_prefix("*.")
                .is_some_and(|base| host.split_once('.').is_some_and(|(label, rest)| !label.is_empty() && rest == base))
    })
}

/// The CA (made on first use) as PEM, and as an issuer.
fn load_or_make_ca(dir: &Path) -> Result<(String, Issuer<'static, KeyPair>)> {
    let (c, k) = (dir.join("ca.crt"), dir.join("ca.key"));
    if !c.is_file() || !k.is_file() {
        std::fs::create_dir_all(dir)?;
        let key = KeyPair::generate()?;
        let tag = crate::tools::hex(&crate::auth::random_bytes::<3>()?);
        let mut p = CertificateParams::default();
        p.distinguished_name = DistinguishedName::new();
        p.distinguished_name.push(DnType::CommonName, format!("Hoard local authority {tag}"));
        p.distinguished_name.push(DnType::OrganizationName, "Hoard");
        p.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        p.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign, KeyUsagePurpose::DigitalSignature];
        let now = OffsetDateTime::now_utc();
        p.not_before = now - time::Duration::days(1);
        p.not_after = now + time::Duration::days(3650);
        let cert = p.self_signed(&key)?;
        write_private(&k, &key.serialize_pem())?;
        std::fs::write(&c, cert.pem())?;
        tracing::info!(cert = %c.display(), "made Hoard's certificate authority");
    }
    let pem = std::fs::read_to_string(&c).with_context(|| format!("read {}", c.display()))?;
    let key = KeyPair::from_pem(&std::fs::read_to_string(&k).with_context(|| format!("read {}", k.display()))?)
        .context("Hoard's certificate authority key")?;
    let issuer = Issuer::from_ca_cert_pem(&pem, key).context("Hoard's certificate authority")?;
    Ok((pem, issuer))
}

/// The CA certificate, if there is one.
fn ca_info(dir: &Path) -> Option<CertInfo> {
    let pem = std::fs::read_to_string(dir.join("ca.crt")).ok()?;
    let der = CertificateDer::from_pem_slice(pem.as_bytes()).ok()?;
    cert_info(&der).ok()
}

/// Whether the server certificate has to be (re)issued: missing, not for exactly `names`, not from the CA there is
/// now, or running out.
fn own_needs_issue(dir: &Path, names: &[String]) -> bool {
    let Some(ca) = ca_info(dir) else { return true };
    let Ok(pem) = std::fs::read_to_string(dir.join("server.crt")) else { return true };
    let Some(cert) = CertificateDer::from_pem_slice(pem.as_bytes()).ok().and_then(|d| cert_info(&d).ok()) else {
        return true;
    };
    let mut have = cert.names.clone();
    sort_names(&mut have);
    let left = cert.not_after - OffsetDateTime::now_utc().unix_timestamp();
    have != names || cert.issuer != ca.subject || left < RENEW_DAYS * 86_400 || !dir.join("server.key").is_file()
}

fn issue(dir: &Path, names: &[String], issuer: &Issuer<'_, KeyPair>) -> Result<()> {
    let key = KeyPair::generate()?;
    let mut p = CertificateParams::new(names.to_vec()).context("certificate addresses")?;
    p.distinguished_name = DistinguishedName::new();
    p.distinguished_name.push(DnType::CommonName, "Hoard");
    p.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    p.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    p.use_authority_key_identifier_extension = true;
    let now = OffsetDateTime::now_utc();
    p.not_before = now - time::Duration::days(1);
    p.not_after = now + time::Duration::days(ISSUE_DAYS);
    let cert = p.signed_by(&key, issuer)?;
    write_private(&dir.join("server.key"), &key.serialize_pem())?;
    std::fs::write(dir.join("server.crt"), cert.pem())?;
    tracing::info!(names = ?names, "issued Hoard's HTTPS certificate");
    Ok(())
}

/// A private key, readable only by this user (on Unix).
fn write_private(path: &Path, text: &str) -> Result<()> {
    use std::io::Write as _;
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut opts, 0o600);
    let mut f = opts.open(path).with_context(|| format!("write {}", path.display()))?;
    f.write_all(text.as_bytes())?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Addresses

/// The addresses Hoard's own certificate covers: the ones found, and the admin's.
fn wanted_names(saved: &Saved, public_url: Option<&str>) -> Vec<String> {
    let mut names = found_names(public_url);
    names.extend(saved.names.iter().cloned());
    sort_names(&mut names);
    names
}

/// This computer's name and LAN addresses, `localhost`, and the public address in Settings. In a container the
/// computer's name is the container's (no use to anyone), so it's left out: the address you type into the browser
/// is added in Settings instead.
fn found_names(public_url: Option<&str>) -> Vec<String> {
    let mut out = vec!["localhost".to_owned(), "127.0.0.1".to_owned(), "::1".to_owned()];
    if !Path::new("/.dockerenv").exists()
        && let Some(host) = gethostname::gethostname().to_str().and_then(|h| clean_name(h).ok())
    {
        if !host.contains('.') {
            out.push(format!("{host}.local"));
        }
        out.push(host);
    }
    for iface in if_addrs::get_if_addrs().unwrap_or_default() {
        if let IpAddr::V4(v4) = iface.ip()
            && !v4.is_loopback()
            && !v4.is_link_local()
            && !v4.is_unspecified()
        {
            out.push(v4.to_string());
        }
    }
    if let Some(host) = public_url.and_then(|u| clean_name(u).ok()) {
        out.push(host);
    }
    sort_names(&mut out);
    out
}

/// Names first, then addresses; no repeats.
fn sort_names(names: &mut Vec<String>) {
    names.sort_by(|a, b| (a.parse::<IpAddr>().is_ok(), a).cmp(&(b.parse::<IpAddr>().is_ok(), b)));
    names.dedup();
}

/// An address as typed (`https://NAS.local:4627/`, `[::1]`, `192.168.1.5`) as a certificate holds it.
fn clean_name(raw: &str) -> Result<String, String> {
    let bad = || format!("“{}” isn't an address a certificate can hold", raw.trim());
    let mut s = raw.trim();
    if let Some((_, rest)) = s.split_once("://") {
        s = rest;
    }
    s = s.split(['/', '?', '#']).next().unwrap_or("");
    if let Some(rest) = s.strip_prefix('[') {
        s = rest.split(']').next().unwrap_or("");
    } else if s.matches(':').count() == 1 {
        s = s.split(':').next().unwrap_or("");
    }
    if let Ok(ip) = s.parse::<IpAddr>() {
        return Ok(ip.to_string());
    }
    let s = s.trim_end_matches('.').to_ascii_lowercase();
    let label_ok = |l: &str| {
        (1..=63).contains(&l.len())
            && l.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            && !l.starts_with('-')
            && !l.ends_with('-')
    };
    let mut labels = s.split('.');
    let first = labels.next().unwrap_or("");
    let rest: Vec<&str> = labels.collect();
    let ok =
        s.len() <= 253 && (label_ok(first) || (first == "*" && rest.len() >= 2)) && rest.iter().all(|l| label_ok(l));
    if ok { Ok(s) } else { Err(bad()) }
}

/// The host (without the port) a request was made to.
fn request_host(headers: &HeaderMap) -> Option<String> {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok())?;
    clean_name(host).ok()
}

// ---------------------------------------------------------------------------------------------
// Trying a certificate before switching to it

/// A TLS handshake with `config`, in memory: proves the key works for real before anyone is switched over.
async fn try_handshake(config: Arc<ServerConfig>) -> Result<()> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut client = rustls::ClientConfig::builder_with_provider(Arc::clone(&provider))
        .with_safe_default_protocol_versions()?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(SignatureOnly(provider)))
        .with_no_client_auth();
    client.alpn_protocols = vec![b"h2".to_vec()];
    let (a, b) = tokio::io::duplex(64 << 10);
    let server = tokio_rustls::TlsAcceptor::from(config).accept(a);
    let client = tokio_rustls::TlsConnector::from(Arc::new(client)).connect(ServerName::try_from("hoard.test")?, b);
    let (s, c) = tokio::time::timeout(Duration::from_secs(10), async { tokio::join!(server, client) })
        .await
        .context("the test connection timed out")?;
    s.context("the test connection failed")?;
    c.context("the test connection failed")?;
    Ok(())
}

/// Checks the server's signature in the handshake, and nothing about who it is (that's not the question here).
#[derive(Debug)]
pub(crate) struct SignatureOnly(pub(crate) Arc<rustls::crypto::CryptoProvider>);

impl rustls::client::danger::ServerCertVerifier for SignatureOnly {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls_pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(message, cert, dss, &self.0.signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(message, cert, dss, &self.0.signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}

// ---------------------------------------------------------------------------------------------
// Saved in the database

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

// ---------------------------------------------------------------------------------------------
// API

/// Open: the CA certificate, for installing on devices (phones download it before they've signed in).
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/api/https/ca.crt", get(ca_cert))
}

/// Settings → Connection.
pub(crate) fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/https", get(get_status).put(put_change))
        .route("/api/admin/https/check", post(check))
}

async fn ca_cert() -> Response {
    let Some(pem) = HTTPS.get().and_then(|h| std::fs::read(h.dir.join("ca.crt")).ok()) else {
        return (StatusCode::NOT_FOUND, "Hoard hasn't made its certificate authority yet").into_response();
    };
    (
        [
            (header::CONTENT_TYPE, "application/x-x509-ca-cert"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"hoard-ca.crt\""),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        pem,
    )
        .into_response()
}

fn status(state: &AppState, headers: &HeaderMap) -> Value {
    let Some(h) = HTTPS.get() else { return json!({ "mode": "off", "available": false }) };
    let public_url = state.settings.get().public_url.clone();
    let host = request_host(headers);
    let live = h.live();
    let cert = live.cert.clone();
    let custom = read_pair(&h.dir.join("custom.crt"), &h.dir.join("custom.key")).ok().map(|b| b.info);
    json!({
        "available": true,
        "mode": live.saved.mode,
        "names": live.saved.names,
        "http3": live.saved.http3,
        "redirect": live.saved.redirect,
        "on": live.tls.is_some(),
        "error": live.error,
        "http3_running": h.h3.lock().unwrap_or_else(PoisonError::into_inner).is_some(),
        "http3_error": live.h3_error,
        "found": found_names(public_url.as_deref()),
        "covers_this_address": match (&cert, &host) {
            (Some(c), Some(host)) => Some(covers(&c.names, host)),
            _ => None,
        },
        "cert": cert,
        "custom": custom,
        "ca": ca_info(&h.dir),
        "port": h.listen.port(),
        "this_address": host,
    })
}

async fn get_status(State(state): State<AppState>, headers: HeaderMap) -> Json<Value> {
    Json(status(&state, &headers))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Change {
    mode: Option<Mode>,
    names: Option<Vec<String>>,
    http3: Option<bool>,
    redirect: Option<bool>,
    /// A new certificate for "my certificate" (with `key`).
    cert: Option<String>,
    key: Option<String>,
}

fn upload_of(cert: Option<String>, key: Option<String>) -> Option<Upload> {
    let blank = |s: &Option<String>| s.as_deref().is_none_or(|s| s.trim().is_empty());
    if blank(&cert) && blank(&key) {
        return None;
    }
    Some(Upload { cert: cert.unwrap_or_default(), key: key.unwrap_or_default() })
}

/// Change HTTPS. The new setup is built and tried before it's saved and switched to; one that doesn't work is
/// refused and what was running stays.
async fn put_change(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(change): Json<Change>,
) -> ApiResult<Json<Value>> {
    let h = HTTPS.get().ok_or_else(|| AppError::unavailable("HTTPS isn't available on this server"))?;
    let _one = h.change.lock().await;
    let mut next = h.live().saved.clone();
    if let Some(m) = change.mode {
        next.mode = m;
    }
    if let Some(names) = change.names {
        let mut clean = names
            .iter()
            .filter(|n| !n.trim().is_empty())
            .map(|n| clean_name(n))
            .collect::<Result<Vec<_>, _>>()
            .map_err(AppError::bad_request)?;
        sort_names(&mut clean);
        next.names = clean;
    }
    if let Some(v) = change.http3 {
        next.http3 = v;
    }
    if let Some(v) = change.redirect {
        next.redirect = v;
    }
    let upload = upload_of(change.cert, change.key);
    if upload.is_some() && next.mode != Mode::Custom {
        return Err(AppError::bad_request("A certificate goes with “My certificate”"));
    }

    let public_url = state.settings.get().public_url.clone();
    let dir = h.dir.clone();
    let (to_save, upload) = (next.clone(), upload);
    let (built, upload) = tokio::task::spawn_blocking(move || {
        let built = build(&dir, &to_save, public_url.as_deref(), upload.as_ref());
        (built, upload)
    })
    .await?;
    let built = built.map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    if let Some(b) = &built {
        let config = server_config(&b.chain, &b.key, &[b"h2", b"http/1.1"])?;
        try_handshake(Arc::new(config))
            .await
            .map_err(|e| AppError::bad_request(format!("The certificate didn't work in a test connection: {e:#}")))?;
    }
    if let Some(u) = &upload {
        u.store(&h.dir)?;
    }
    let to_save = next.clone();
    crate::db::call(&state.pool, move |c| save(c, &to_save)).await?;
    activate(h, next, built)?;
    let live = h.live();
    tracing::info!(mode = ?live.saved.mode, http3 = live.saved.http3, redirect = live.saved.redirect, "HTTPS changed");
    drop(live);
    Ok(Json(status(&state, &headers)))
}

#[derive(Debug, Deserialize)]
struct CheckBody {
    cert: String,
    key: String,
}

/// What a pasted certificate is (names, dates, who issued it) and whether it can be used, without saving it.
async fn check(headers: HeaderMap, Json(body): Json<CheckBody>) -> ApiResult<Json<Value>> {
    let upload = Upload { cert: body.cert, key: body.key };
    let built = upload.parse().map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    let problem = check_dates(&built.info).err().map(|e| e.to_string());
    let host = request_host(&headers);
    Ok(Json(json!({
        "cert": built.info,
        "problem": problem,
        "covers_this_address": host.as_deref().map(|host| covers(&built.info.names, host)),
        "this_address": host,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_are_cleaned_up() {
        assert_eq!(clean_name("https://NAS.local:4627/x").unwrap(), "nas.local");
        assert_eq!(clean_name("192.168.0.22:4627").unwrap(), "192.168.0.22");
        assert_eq!(clean_name("[::1]:4627").unwrap(), "::1");
        assert_eq!(clean_name("fe80::1").unwrap(), "fe80::1");
        assert_eq!(clean_name("*.example.com").unwrap(), "*.example.com");
        assert_eq!(clean_name("hoard.tail1234.ts.net.").unwrap(), "hoard.tail1234.ts.net");
        assert!(clean_name("not a name").is_err());
        assert!(clean_name("*.com").is_err());
        assert!(clean_name("-bad.example").is_err());
    }

    #[test]
    fn wildcards_cover_one_label() {
        let names = vec!["*.example.com".to_owned(), "192.168.0.22".to_owned()];
        assert!(covers(&names, "hoard.example.com"));
        assert!(!covers(&names, "a.b.example.com"));
        assert!(!covers(&names, "example.com"));
        assert!(covers(&names, "192.168.0.22"));
    }

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

    #[tokio::test]
    async fn own_certificate_is_issued_from_the_authority_and_reissued_when_addresses_change() {
        let dir = tempfile::tempdir().unwrap();
        let saved = Saved { mode: Mode::Own, names: vec!["nas.example".into()], ..Saved::default() };
        let built = build(dir.path(), &saved, None, None).unwrap().unwrap();
        assert!(built.info.names.contains(&"nas.example".to_owned()));
        assert!(built.info.names.contains(&"127.0.0.1".to_owned()));
        let ca = ca_info(dir.path()).unwrap();
        assert_eq!(built.info.issuer, ca.subject);
        assert!(!built.info.self_signed);
        try_handshake(Arc::new(server_config(&built.chain, &built.key, &[b"h2"]).unwrap())).await.unwrap();

        // Same addresses: kept. A new one: re-issued from the same authority.
        let names = wanted_names(&saved, None);
        assert!(!own_needs_issue(dir.path(), &names));
        let more = Saved { names: vec!["nas.example".into(), "hoard.example".into()], ..saved };
        let again = build(dir.path(), &more, Some("https://public.example"), None).unwrap().unwrap();
        assert!(again.info.names.contains(&"hoard.example".to_owned()));
        assert!(again.info.names.contains(&"public.example".to_owned()));
        assert_ne!(again.info.fingerprint, built.info.fingerprint);
        assert_eq!(ca_info(dir.path()).unwrap().fingerprint, ca.fingerprint, "the authority stays");
    }

    fn self_signed(names: &[&str], days: (i64, i64)) -> (String, String) {
        let key = KeyPair::generate().unwrap();
        let mut p = CertificateParams::new(names.iter().map(|n| (*n).to_owned()).collect::<Vec<_>>()).unwrap();
        let now = OffsetDateTime::now_utc();
        p.not_before = now + time::Duration::days(days.0);
        p.not_after = now + time::Duration::days(days.1);
        (p.self_signed(&key).unwrap().pem(), key.serialize_pem())
    }

    #[test]
    fn uploaded_certificates_are_checked() {
        let (cert, key) = self_signed(&["hoard.example"], (-1, 90));
        let ok = Upload { cert: cert.clone(), key: key.clone() }.parse().unwrap();
        assert_eq!(ok.info.names, ["hoard.example"]);
        assert!(check_dates(&ok.info).is_ok());

        // Both in one box works too.
        assert!(Upload { cert: format!("{cert}{key}"), key: String::new() }.parse().is_ok());

        let (_, other_key) = self_signed(&["x.example"], (-1, 90));
        let msg = |u: Upload| format!("{:#}", u.parse().err().unwrap());
        assert!(msg(Upload { cert: cert.clone(), key: other_key }).contains("doesn't belong"));
        assert!(msg(Upload { cert: cert.clone(), key: String::new() }).contains("No private key"));
        assert!(msg(Upload { cert: String::new(), key: key.clone() }).contains("No certificate"));

        let (old, old_key) = self_signed(&["hoard.example"], (-90, -1));
        let expired = Upload { cert: old, key: old_key }.parse().unwrap();
        assert!(check_dates(&expired.info).unwrap_err().to_string().contains("expired"));
        let dir = tempfile::tempdir().unwrap();
        let saved = Saved { mode: Mode::Custom, ..Saved::default() };
        let (old, old_key) = self_signed(&["hoard.example"], (-90, -1));
        let err = build(dir.path(), &saved, None, Some(&Upload { cert: old, key: old_key })).err().unwrap();
        assert!(err.to_string().contains("expired"));
        assert!(build(dir.path(), &saved, None, None).err().unwrap().to_string().contains("Paste or upload"));
    }
}
