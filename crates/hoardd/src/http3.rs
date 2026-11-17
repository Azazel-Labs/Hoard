//! HTTP/3 (QUIC, UDP on the `--listen` port number), started by `https.rs` while it's on in Settings.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::Router;
use axum::body::Body;
use axum::http::{Request, Response};
use bytes::{Buf, Bytes};
use futures_util::StreamExt;
use rustls::ServerConfig;

use crate::server::handle;

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
