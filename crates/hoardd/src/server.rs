//! Listening: HTTP/1.1, HTTP/2 and (opt-in) HTTP/3.
//!
//! Browsers only speak HTTP/2 over TLS, and HTTP/2 is what makes a grid of hundreds of thumbnails fast: one
//! connection, every request multiplexed, headers compressed. Over plain HTTP they fall back to HTTP/1.1 with about six
//! connections per host. So:
//!
//! - no TLS configured: HTTP/1.1, plus cleartext HTTP/2 ("h2c", prior knowledge) for the CLI and reverse proxies;
//! - `--tls-cert/--tls-key` or `--tls-self-signed`: HTTPS with HTTP/2 negotiated via ALPN, HTTP/1.1 as fallback;
//! - `--http3` (needs TLS): HTTP/3 over QUIC on the same port number (UDP), advertised with `Alt-Svc` so browsers
//!   switch to it after their first request. The Rust HTTP/3 stack (`h3`) is still pre-1.0, hence opt-in.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use axum::Router;
use axum::body::Body;
use bytes::{Buf, Bytes};
use futures_util::StreamExt;
use rustls::ServerConfig;
use rustls_pki_types::pem::PemObject;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use tower::ServiceExt;

use crate::config::Config;

type CertChain = (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>);

/// Load the configured certificate, or create (once) and reuse a self-signed one under the data dir.
pub(crate) fn load_tls(cfg: &Config) -> Result<Option<CertChain>> {
    let (cert_path, key_path) = match (&cfg.tls_cert, &cfg.tls_key) {
        (Some(c), Some(k)) => (c.clone(), k.clone()),
        (None, None) if cfg.tls_self_signed => {
            let dir = cfg.data_dir.join("tls");
            let (c, k) = (dir.join("self-signed.crt"), dir.join("self-signed.key"));
            if !c.is_file() || !k.is_file() {
                std::fs::create_dir_all(&dir)?;
                let mut names = vec!["localhost".to_owned(), "127.0.0.1".to_owned(), "::1".to_owned()];
                names.extend(cfg.tls_names.iter().cloned());
                if !cfg.listen.ip().is_unspecified() {
                    names.push(cfg.listen.ip().to_string());
                }
                names.sort();
                names.dedup();
                let generated = rcgen::generate_simple_self_signed(names.clone()).context("generate certificate")?;
                std::fs::write(&c, generated.cert.pem())?;
                std::fs::write(&k, generated.signing_key.serialize_pem())?;
                tracing::info!(names = ?names, cert = %c.display(), "generated a self-signed TLS certificate");
            }
            (c, k)
        }
        (None, None) => return Ok(None),
        _ => bail!("--tls-cert and --tls-key go together"),
    };
    let certs = CertificateDer::pem_file_iter(&cert_path)
        .with_context(|| format!("read {}", cert_path.display()))?
        .collect::<Result<Vec<_>, _>>()?;
    let key = PrivateKeyDer::from_pem_file(&key_path).with_context(|| format!("read {}", key_path.display()))?;
    Ok(Some((certs, key)))
}

fn rustls_config(chain: &CertChain, alpn: &[&[u8]]) -> Result<ServerConfig> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut config = ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(chain.0.clone(), chain.1.clone_key())?;
    config.alpn_protocols = alpn.iter().map(|p| p.to_vec()).collect();
    Ok(config)
}

/// `Alt-Svc` value advertising HTTP/3 on `port`.
pub(crate) fn alt_svc(port: u16) -> String {
    format!("h3=\":{port}\"; ma=86400")
}

pub(crate) async fn serve(cfg: &Config, app: Router) -> Result<()> {
    let addr = cfg.listen;
    let Some(chain) = load_tls(cfg)? else {
        if cfg.http3 {
            bail!("--http3 needs TLS: set --tls-cert/--tls-key or --tls-self-signed");
        }
        // axum::serve speaks HTTP/1.1 and, with the `http2` feature, h2c with prior knowledge.
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!(%addr, "listening: http/1.1 + h2c (no TLS, so browsers will use HTTP/1.1)");
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        return Ok(());
    };

    if cfg.http3 {
        let h3_config = rustls_config(&chain, &[b"h3"])?;
        let h3_app = app.clone();
        tokio::spawn(async move {
            if let Err(e) = serve_h3(addr, h3_config, h3_app).await {
                tracing::error!(error = %format!("{e:#}"), "HTTP/3 listener stopped");
            }
        });
    }

    let tls =
        axum_server::tls_rustls::RustlsConfig::from_config(Arc::new(rustls_config(&chain, &[b"h2", b"http/1.1"])?));
    let handle = axum_server::Handle::new();
    let shutdown = handle.clone();
    tokio::spawn(async move {
        shutdown_signal().await;
        shutdown.graceful_shutdown(Some(Duration::from_secs(10)));
    });
    tracing::info!(%addr, http3 = cfg.http3, "listening: https with http/2 (ALPN), http/1.1 fallback");
    axum_server::bind_rustls(addr, tls)
        .handle(handle)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.ok();
    tracing::info!("shutting down");
}

// ---------------------------------------------------------------------------------------------
// HTTP/3

async fn serve_h3(addr: SocketAddr, tls: ServerConfig, app: Router) -> Result<()> {
    let quic = quinn::crypto::rustls::QuicServerConfig::try_from(tls).context("QUIC TLS config")?;
    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(quic));
    // Notice vanished clients quickly (closed laptop lid), and keep healthy idle connections alive.
    let mut transport = quinn::TransportConfig::default();
    transport.max_idle_timeout(Some(Duration::from_secs(30).try_into()?));
    transport.keep_alive_interval(Some(Duration::from_secs(10)));
    server_config.transport_config(Arc::new(transport));
    let endpoint =
        quinn::Endpoint::server(server_config, addr).with_context(|| format!("bind UDP {addr} for HTTP/3"))?;
    tracing::info!(%addr, "listening: http/3 (QUIC)");
    while let Some(incoming) = endpoint.accept().await {
        let app = app.clone();
        tokio::spawn(async move {
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
        });
    }
    Ok(())
}

/// Bridge one HTTP/3 request into the axum router and stream the response back.
async fn handle_h3<S>(
    app: Router,
    req: http::Request<()>,
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
    let mut req = req.map(|()| body);
    req.extensions_mut().insert(axum::extract::ConnectInfo(remote));

    let resp = app.oneshot(req).await.unwrap_or_else(|never| match never {});
    let (parts, body) = resp.into_parts();
    send.send_response(http::Response::from_parts(parts, ())).await?;
    let mut data = body.into_data_stream();
    while let Some(chunk) = data.next().await {
        send.send_data(chunk?).await?;
    }
    send.finish().await?;
    Ok(())
}
