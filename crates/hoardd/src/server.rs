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
//! See `https.rs` for what plain HTTP does once HTTPS is on.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::Router;
use axum::body::Body;
use axum::http::{HeaderValue, Request, Response, StatusCode, header};
use bytes::{Buf, Bytes};
use futures_util::StreamExt;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::graceful::{GracefulShutdown, Watcher};
use rustls::ServerConfig;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tower::ServiceExt;

use crate::config::Config;

/// A client that connects and says nothing is dropped after this long; a TLS handshake gets this long.
const FIRST_BYTE: Duration = Duration::from_secs(30);
const HANDSHAKE: Duration = Duration::from_secs(15);

pub(crate) async fn serve(cfg: &Config, app: Router) -> Result<()> {
    let addr = cfg.listen;
    let listener = tokio::net::TcpListener::bind(addr).await.with_context(|| format!("listen on {addr}"))?;
    let https = crate::https::tls_config().is_some();
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
    if !tls {
        return serve_io(TokioIo::new(stream), remote, app, false, watcher).await;
    }
    // HTTPS is off: nothing to answer a handshake with.
    let Some(config) = crate::https::tls_config() else { return };
    match tokio::time::timeout(HANDSHAKE, tokio_rustls::TlsAcceptor::from(config).accept(stream)).await {
        Ok(Ok(tls)) => serve_io(TokioIo::new(tls), remote, app, true, watcher).await,
        Ok(Err(e)) => tracing::debug!(%remote, error = %e, "TLS handshake failed"),
        Err(_) => tracing::debug!(%remote, "TLS handshake timed out"),
    }
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
async fn handle(app: Router, mut req: Request<Body>, remote: SocketAddr, secure: bool) -> Response<Body> {
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

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.ok();
    tracing::info!("shutting down");
}

// ---------------------------------------------------------------------------------------------
// HTTP/3

/// QUIC settings around `tls` (ALPN `h3`).
pub(crate) fn quic_config(tls: Arc<ServerConfig>) -> Result<quinn::ServerConfig> {
    let quic = quinn::crypto::rustls::QuicServerConfig::try_from(tls).context("QUIC TLS config")?;
    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(quic));
    // Notice vanished clients quickly (closed laptop lid), and keep healthy idle connections alive.
    let mut transport = quinn::TransportConfig::default();
    transport.max_idle_timeout(Some(Duration::from_secs(30).try_into()?));
    transport.keep_alive_interval(Some(Duration::from_secs(10)));
    server_config.transport_config(Arc::new(transport));
    Ok(server_config)
}

/// Start the HTTP/3 listener on UDP `addr`. It runs until the endpoint is closed.
pub(crate) fn start_h3(addr: SocketAddr, tls: Arc<ServerConfig>, app: Router) -> Result<quinn::Endpoint> {
    let endpoint =
        quinn::Endpoint::server(quic_config(tls)?, addr).with_context(|| format!("listen on UDP {addr} for HTTP/3"))?;
    tracing::info!(%addr, "listening: http/3 (QUIC)");
    let accepting = endpoint.clone();
    tokio::spawn(async move {
        while let Some(incoming) = accepting.accept().await {
            tokio::spawn(h3_connection(incoming, app.clone()));
        }
    });
    Ok(endpoint)
}

async fn h3_connection(incoming: quinn::Incoming, app: Router) {
    let remote = incoming.remote_address();
    let conn = match incoming.await {
        Ok(c) => c,
        Err(e) => return tracing::debug!(%remote, error = %e, "QUIC handshake failed"),
    };
    let mut h3 = match h3::server::Connection::<_, Bytes>::new(h3_quinn::Connection::new(conn)).await {
        Ok(c) => c,
        Err(e) => return tracing::debug!(%remote, error = %e, "HTTP/3 setup failed"),
    };
    loop {
        match h3.accept().await {
            Ok(Some(resolver)) => {
                let app = app.clone();
                tokio::spawn(async move {
                    match resolver.resolve_request().await {
                        Ok((req, stream)) => {
                            if let Err(e) = handle_h3(app, req, stream, remote).await {
                                tracing::debug!(error = %format!("{e:#}"), "HTTP/3 request failed");
                            }
                        }
                        Err(e) => tracing::debug!(error = %e, "bad HTTP/3 request"),
                    }
                });
            }
            Ok(None) => break,
            Err(e) => {
                tracing::debug!(%remote, error = %e, "HTTP/3 connection closed");
                break;
            }
        }
    }
}

/// Bridge one HTTP/3 request into the axum router and stream the response back.
async fn handle_h3<S>(
    app: Router,
    req: Request<()>,
    stream: h3::server::RequestStream<S, Bytes>,
    remote: SocketAddr,
) -> Result<()>
where
    S: h3::quic::BidiStream<Bytes> + Send + 'static,
    <S as h3::quic::BidiStream<Bytes>>::RecvStream: Send + 'static,
    <S as h3::quic::BidiStream<Bytes>>::SendStream: Send + 'static,
{
    let (mut send, mut recv) = stream.split();

    // Request body: pump QUIC data frames into a channel the router reads as a stream.
    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Bytes, std::io::Error>>(8);
    tokio::spawn(async move {
        loop {
            match recv.recv_data().await {
                Ok(Some(mut chunk)) => {
                    let bytes = chunk.copy_to_bytes(chunk.remaining());
                    if tx.send(Ok(bytes)).await.is_err() {
                        break;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let _ = tx.send(Err(std::io::Error::other(e.to_string()))).await;
                    break;
                }
            }
        }
    });
    let body =
        Body::from_stream(futures_util::stream::unfold(rx, |mut rx| async move { rx.recv().await.map(|v| (v, rx)) }));
    let resp = handle(app, req.map(|()| body), remote, true).await;
    let (parts, body) = resp.into_parts();
    send.send_response(Response::from_parts(parts, ())).await?;
    let mut data = body.into_data_stream();
    while let Some(chunk) = data.next().await {
        send.send_data(chunk?).await?;
    }
    send.finish().await?;
    Ok(())
}
