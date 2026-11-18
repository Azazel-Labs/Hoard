//! Listening: HTTP/1.1, HTTP/2 and HTTP/3, plain and HTTPS on one port.
//!
//! Browsers only speak HTTP/2 over TLS, and HTTP/2 is what makes a grid of hundreds of thumbnails fast: one
//! connection, every request multiplexed, headers compressed. Over plain HTTP they fall back to HTTP/1.1 with about six
//! connections per host.
//!
//! Hoard answers both on the `--listen` port and tells them apart by the first byte a client sends (a TLS handshake
//! starts with 0x16, an HTTP request with a letter, an HTTP/2 preface with `P`):
//!
//! - plain: HTTP/1.1, and cleartext HTTP/2 ("h2c", prior knowledge) for the CLI and reverse proxies;
//! - TLS, while HTTPS is on in Settings: HTTP/2 negotiated via ALPN, HTTP/1.1 as fallback, with the certificate
//!   chosen there (looked up per connection, so a change applies at once);
//! - HTTP/3 (QUIC, UDP on the same port number), when turned on in Settings, advertised with `Alt-Svc` so browsers
//!   switch to it after their first request.
//!
//! See `https` for what plain HTTP does once HTTPS is on, and `http3.rs` for HTTP/3. Built without the `https`
//! feature (the desktop app), only plain HTTP is answered.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::Router;
use axum::body::Body;
use axum::http::{HeaderValue, Request, Response, StatusCode, header};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::graceful::{GracefulShutdown, Watcher};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::sync::Notify;
use tower::ServiceExt;

use crate::config::Config;

/// A client that connects and says nothing is dropped after this long; a TLS handshake gets this long.
const FIRST_BYTE: Duration = Duration::from_secs(30);
#[cfg(feature = "https")]
const HANDSHAKE: Duration = Duration::from_secs(15);

pub(crate) async fn serve(cfg: &Config, app: Router) -> Result<()> {
    let addr = cfg.listen;
    let listener = tokio::net::TcpListener::bind(addr).await.with_context(|| format!("listen on {addr}"))?;
    crate::desktop::listening_on(listener.local_addr().unwrap_or(addr));
    let https = crate::https::on();
    tracing::info!(%addr, https, "listening: http/1.1 + h2c, and https with http/2 when it's on in Settings");
    crate::https::serving(app.clone());

    let graceful = GracefulShutdown::new();
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);
    loop {
        tokio::select! {
            accepted = listener.accept() => match accepted {
                Ok((stream, remote)) => {
                    tokio::spawn(connection(stream, remote, app.clone(), graceful.watcher()));
                }
                Err(e) => {
                    // Out of file handles and the like: don't spin.
                    tracing::warn!(error = %e, "accept failed");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            },
            () = &mut shutdown => break,
        }
    }
    drop(listener);
    crate::https::stop();
    tokio::select! {
        () = graceful.shutdown() => {}
        () = tokio::time::sleep(Duration::from_secs(10)) => tracing::info!("stopped waiting for open connections"),
    }
    Ok(())
}

/// One TCP connection: TLS or plain, by its first byte.
async fn connection(stream: TcpStream, remote: SocketAddr, app: Router, watcher: Watcher) {
    stream.set_nodelay(true).ok();
    let mut first = [0u8; 1];
    let tls = match tokio::time::timeout(FIRST_BYTE, stream.peek(&mut first)).await {
        Ok(Ok(1)) => first[0] == 0x16,
        _ => return,
    };
    if tls {
        serve_tls(stream, remote, app, watcher).await;
    } else {
        serve_io(TokioIo::new(stream), remote, app, false, watcher).await;
    }
}

#[cfg(feature = "https")]
async fn serve_tls(stream: TcpStream, remote: SocketAddr, app: Router, watcher: Watcher) {
    // HTTPS is off: nothing to answer a handshake with.
    let Some(config) = crate::https::tls_config() else { return };
    match tokio::time::timeout(HANDSHAKE, tokio_rustls::TlsAcceptor::from(config).accept(stream)).await {
        Ok(Ok(tls)) => serve_io(TokioIo::new(tls), remote, app, true, watcher).await,
        Ok(Err(e)) => tracing::debug!(%remote, error = %e, "TLS handshake failed"),
        Err(_) => tracing::debug!(%remote, "TLS handshake timed out"),
    }
}

/// Built without HTTPS: nothing to answer a handshake with.
#[cfg(not(feature = "https"))]
#[expect(clippy::unused_async, reason = "the same signature as with the `https` feature")]
async fn serve_tls(_stream: TcpStream, remote: SocketAddr, _app: Router, _watcher: Watcher) {
    tracing::debug!(%remote, "TLS handshake refused: built without HTTPS");
}

async fn serve_io<I>(io: TokioIo<I>, remote: SocketAddr, app: Router, secure: bool, watcher: Watcher)
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let service = hyper::service::service_fn(move |req: Request<hyper::body::Incoming>| {
        let app = app.clone();
        async move { Ok::<_, Infallible>(handle(app, req.map(Body::new), remote, secure).await) }
    });
    let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
    if let Err(e) = watcher.watch(builder.serve_connection_with_upgrades(io, service)).await {
        tracing::debug!(%remote, error = %e, "connection ended with an error");
    }
}

/// One request, from any of the listeners, into the app.
pub(crate) async fn handle(app: Router, mut req: Request<Body>, remote: SocketAddr, secure: bool) -> Response<Body> {
    if secure {
        // What a reverse proxy would say: the cookie gets `Secure`, and MCP builds `https://` addresses.
        req.headers_mut().insert("x-forwarded-proto", HeaderValue::from_static("https"));
    } else if let Some(to) = crate::https::redirect_for(&req, remote) {
        // 307 rather than 308, and no HSTS: browsers don't remember it, so turning HTTPS off again just works.
        return Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header(header::LOCATION, to)
            .header(header::CACHE_CONTROL, "no-store")
            .body(Body::empty())
            .unwrap_or_default();
    }
    let alt_svc = if secure { crate::https::alt_svc(&req) } else { None };
    req.extensions_mut().insert(axum::extract::ConnectInfo(remote));
    let mut resp = app.oneshot(req).await.unwrap_or_else(|never| match never {});
    if let Some(v) = alt_svc {
        resp.headers_mut().insert(header::ALT_SVC, v);
    }
    resp
}

/// Stops the server as Ctrl+C does: the desktop app's way to quit.
pub(crate) static STOP: Notify = Notify::const_new();

async fn shutdown_signal() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        () = STOP.notified() => {}
    }
    tracing::info!("shutting down");
}
