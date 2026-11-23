//! Settings → Connection's routes: the authority's certificate to install, the status, turning HTTPS on or off or
//! changing the certificate, and checking one before it's saved.

use std::sync::{Arc, PoisonError};

use anyhow::Result;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};

use super::certs::{Upload, build, ca_info, check_dates, covers, read_pair, server_config};
use super::names::{clean_name, found_names, request_host, sort_names};
use super::{HTTPS, Mode, activate, save, try_handshake};
use crate::AppState;
use crate::catalog::{Access, Operation};
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;

/// Open: the CA certificate, for installing on devices (phones download it before they've signed in).
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/api/https/ca.crt", get(ca_cert))
}

/// Settings → Connection.
pub(crate) fn operations() -> Vec<Operation> {
    use Access::Admin;
    vec![
        Admin.get("/api/admin/https", get_status).named(
            "get_https",
            "How Hoard serves HTTPS: off, its own certificate authority, or a certificate given it, with the names \
             and dates it covers.",
        ),
        Admin
            .put("/api/admin/https", put_change)
            .named(
                "change_https",
                "Change how Hoard serves HTTPS. The new setup is tried before it's switched to.",
            )
            .for_people("a wrong change can cut off every device, the one asking included"),
        Admin.post("/api/admin/https/check", check).named(
            "check_certificate",
            "What a pasted certificate is (names, dates, who issued it) and whether it can be used, without saving it.",
        ),
    ]
}

async fn ca_cert() -> Response {
    let Some(pem) = HTTPS.get().and_then(|h| std::fs::read(h.dir.join("ca.crt")).ok()) else {
        return (StatusCode::NOT_FOUND, tr!("error-https-no-ca")).into_response();
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
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
    let h = HTTPS.get().ok_or_else(|| AppError::unavailable(tr!("error-https-unavailable")))?;
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
        return Err(AppError::bad_request(tr!("error-https-certificate-mode")));
    }

    let public_url = state.settings.get().public_url.clone();
    let dir = h.dir.clone();
    let (to_save, upload, language) = (next.clone(), upload, crate::i18n::current());
    let (built, upload) = tokio::task::spawn_blocking(move || {
        let built = crate::i18n::blocking(language, || build(&dir, &to_save, public_url.as_deref(), upload.as_ref()));
        (built, upload)
    })
    .await?;
    let built = built.map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    if let Some(b) = &built {
        let config = server_config(&b.chain, &b.key, &[b"h2", b"http/1.1"])?;
        try_handshake(Arc::new(config))
            .await
            .map_err(|e| AppError::bad_request(tr!("error-https-test", reason = format!("{e:#}"))))?;
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
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
