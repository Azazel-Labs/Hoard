//! Conversions: an asset in another format, Unreal packages, and the conversion rules run over the library.

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use hoard_core::Asset;
use rusqlite::OptionalExtension as _;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{db, jobs};

use super::files::serve_blob;

#[derive(Deserialize)]
pub(super) struct AsParams {
    /// Scale images down so neither side exceeds this (a power of two, 64-16384).
    #[serde(default)]
    max: Option<u32>,
    /// `0` to serve inline (for previews) instead of as an attachment.
    #[serde(default)]
    download: Option<u8>,
    /// Seconds to wait for a conversion that isn't cached yet before answering 202 with its progress (0-60,
    /// default 20).
    #[serde(default)]
    wait: Option<u64>,
}

/// How long `/as/` waits for a conversion by default. Waiting holds no thread, only the connection.
const AS_WAIT_DEFAULT: u64 = 20;

/// Where a conversion stands, as JSON: `ready`, `queued`, `converting` (with `progress`) or `failed` (with `error`).
fn stage_json(stage: &crate::formats::Stage) -> Value {
    use crate::formats::Stage;
    match stage {
        Stage::Done(_) => json!({ "state": "ready" }),
        Stage::Queued => json!({ "state": "queued" }),
        Stage::Working(p) => json!({ "state": "converting", "progress": (f64::from(*p) * 100.0).round() / 100.0 }),
        Stage::Failed(e) => json!({ "state": "failed", "error": e }),
    }
}

fn refused(format: &str, r: crate::formats::Refused) -> AppError {
    use crate::formats::Refused;
    match r {
        Refused::NotFound => AppError::not_found(tr!("error-asset-gone")),
        Refused::Unsupported(m) => AppError::bad_request(m),
        Refused::Failed(m) => AppError::unprocessable(tr!("error-conversion-failed", format = format, reason = m)),
    }
}

fn check_max(max: Option<u32>) -> ApiResult<()> {
    if let Some(m) = max
        && !crate::formats::valid_max(m)
    {
        return Err(AppError::bad_request(tr!("error-max-size")));
    }
    Ok(())
}

/// The asset converted to another format (images, audio, models, fonts; see `formats`), made on first request and
/// cached. A conversion that isn't cached is started (or joined, if another request started it) and waited for up
/// to `wait` seconds; if it's still running then, the answer is 202 with its progress and `Retry-After`. The
/// conversion carries on either way.
pub(super) async fn asset_as(
    State(state): State<AppState>,
    Path((id, format)): Path<(String, String)>,
    Query(p): Query<AsParams>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    use crate::formats::{Lookup, Stage};
    check_max(p.max)?;
    let format = format.to_ascii_lowercase();
    let ready = match crate::formats::request(&state, &id, &format, p.max).await.map_err(|r| refused(&format, r))? {
        Lookup::Ready(r) => r,
        Lookup::Pending(rx) => {
            let limit = std::time::Duration::from_secs(p.wait.unwrap_or(AS_WAIT_DEFAULT).min(60));
            match crate::formats::wait(rx, limit).await {
                Stage::Done(r) => r,
                Stage::Failed(m) => {
                    return Err(AppError::unprocessable(tr!("error-conversion-failed", format = format, reason = m)));
                }
                pending => {
                    let mut resp = (StatusCode::ACCEPTED, Json(stage_json(&pending))).into_response();
                    resp.headers_mut().insert(header::RETRY_AFTER, HeaderValue::from_static("2"));
                    return Ok(resp);
                }
            }
        }
    };
    // A texture map converted (a PNG of a TGA, a smaller copy) is named by the convention too.
    let names = state.settings.get();
    let id2 = id.clone();
    let map_stem = db::call(&state.pool, move |c| crate::naming::stem_of(c, &names.texture_names, &id2)).await?;
    let stem = map_stem.as_deref().unwrap_or(&ready.stem);
    let name = match (ready.ext.as_str(), p.max) {
        ("zip", _) => format!("{stem}.{format}.zip"),
        (e, Some(m)) => format!("{stem}_{m}.{e}"),
        (e, None) => format!("{stem}.{e}"),
    };
    let mime = match ready.ext.as_str() {
        "glb" => "model/gltf-binary".to_owned(),
        "exr" => "image/x-exr".to_owned(),
        "opus" => "audio/ogg".to_owned(),
        e => mime_guess::from_ext(e).first_or_octet_stream().to_string(),
    };
    let disposition =
        (p.download != Some(0)).then(|| format!("attachment; filename=\"{}\"", name.replace(['"', '\\'], "_")));
    let mut resp = serve_blob(&state, &ready.hash, &headers, &mime, disposition).await?;
    // The address names the asset, not the bytes, which are made again when the way Hoard makes them changes (a
    // pack's models turned, a newer reader): the browser asks each time and is told when its copy still stands.
    let cache = if state.settings.get().guest_browsing { "public, no-cache" } else { "private, no-cache" };
    resp.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(cache));
    Ok(resp)
}

/// Start (or look in on) a conversion without downloading it: `{"state": "ready" | "queued" | "converting" |
/// "failed", "progress"?, "error"?}`. With `wait`, answers as soon as anything changes or after that many seconds
/// (at most 10). The portal polls this, then downloads from `/as/{format}`.
pub(super) async fn asset_as_status(
    State(state): State<AppState>,
    Path((id, format)): Path<(String, String)>,
    Query(p): Query<AsParams>,
) -> ApiResult<Json<Value>> {
    use crate::formats::{Lookup, Stage};
    check_max(p.max)?;
    let format = format.to_ascii_lowercase();
    Ok(Json(match crate::formats::request(&state, &id, &format, p.max).await.map_err(|r| refused(&format, r))? {
        Lookup::Ready(_) => stage_json(&Stage::Done(crate::formats::Ready {
            hash: String::new(),
            ext: String::new(),
            stem: String::new(),
        })),
        Lookup::Pending(rx) => {
            let limit = std::time::Duration::from_secs(p.wait.unwrap_or(0).min(10));
            stage_json(&crate::formats::wait_for_change(rx, limit).await)
        }
    }))
}

pub(super) async fn unreal_convert(
    State(state): State<AppState>,
    Path(id): Path<String>,
    body: Option<Json<hoard_core::UnrealConvertRequest>>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let req = body.map_or_default(|Json(b)| b);
    let job = crate::extract::Request {
        discard_originals: req.discard_originals,
        drop_unsupported: req.drop_unsupported,
        then_llm: None,
    };
    let id2 = id.clone();
    let queued = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(None);
        }
        Ok(Some(db::enqueue(c, jobs::UNREAL, Some(&id2), &serde_json::to_value(job)?)?))
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))?;
    state.wake.notify_one();
    Ok((StatusCode::ACCEPTED, Json(json!({ "job": queued }))))
}

pub(super) async fn unreal_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<hoard_core::UnrealPackStatus>> {
    let id2 = id.clone();
    db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(None);
        }
        crate::extract::pack_status(c, &id2).map(Some)
    })
    .await?
    .map(Json)
    .ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))
}

#[derive(Deserialize)]
pub(super) struct PendingParams {
    #[serde(default)]
    limit: Option<u32>,
}

pub(super) async fn unreal_pending(
    State(state): State<AppState>,
    Query(p): Query<PendingParams>,
) -> ApiResult<Json<Vec<hoard_core::PendingPackage>>> {
    let limit = p.limit.unwrap_or(500).clamp(1, 5000);
    Ok(Json(db::call(&state.pool, move |c| crate::extract::pending(c, limit)).await?))
}

#[derive(Deserialize)]
pub(super) struct ReplaceParams {
    /// Extension of the uploaded file (`fbx`, `png`, …).
    ext: String,
    /// `editor` for files exported by Unreal Editor, `native` otherwise.
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    keep_original: Option<u8>,
    #[serde(default)]
    notes: Option<String>,
    /// What made the file: the tool, its version and how (kept with the asset's origin).
    #[serde(default)]
    converter: Option<String>,
}

/// Swap an asset's file for a converted one (`hoard unreal-worker` uploads editor exports here).
pub(super) async fn replace_asset(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<ReplaceParams>,
    body: axum::body::Bytes,
) -> ApiResult<Json<Asset>> {
    let ext = p.ext.trim_start_matches('.').to_ascii_lowercase();
    if ext.is_empty() || ext.len() > 12 || !ext.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AppError::bad_request(tr!("error-replace-ext")));
    }
    if body.is_empty() {
        return Err(AppError::bad_request(tr!("error-empty-file")));
    }
    let id2 = id.clone();
    let class = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT COALESCE(u.class, json_extract(a.meta, '$.ue_class')) FROM assets a
             LEFT JOIN unreal_status u ON u.asset_id = a.id WHERE a.id = ?1",
            [&id2],
            |r| r.get::<_, Option<String>>(0),
        )
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    let blob = state.blobs.put_bytes(body.to_vec(), hoard_core::AssetKind::from_ext(&ext)).await?;
    let origin = crate::extract::Origin {
        class,
        engine: None,
        dependencies: Vec::new(),
        method: if p.method.as_deref() == Some("native") { "native" } else { "editor" },
        notes: p.notes.into_iter().collect(),
        converter: p.converter.map_or_default(|c| c.chars().take(500).collect()),
    };
    let keep = p.keep_original.unwrap_or(1) != 0;
    let id2 = id.clone();
    let asset = db::call(&state.pool, move |c| {
        // Engine and dependencies come from the package, while we still have its analysis.
        let mut origin = origin;
        let meta: Option<String> =
            c.query_row("SELECT meta FROM assets WHERE id = ?1", [&id2], |r| r.get(0)).optional()?;
        if let Some(m) = meta.and_then(|m| serde_json::from_str::<Value>(&m).ok()) {
            origin.engine = m.get("ue_engine").and_then(Value::as_str).map(str::to_owned);
            origin.dependencies = m
                .get("ue_dependencies")
                .and_then(Value::as_array)
                .map_or_default(|a| a.iter().filter_map(|d| d.as_str().map(str::to_owned)).collect());
        }
        let tx = c.transaction()?;
        let kept = crate::extract::replace_asset(&tx, &id2, &blob, &ext, &origin, keep)?;
        tx.commit()?;
        if kept { db::get_asset(c, &id2) } else { Ok(None) }
    })
    .await?
    .ok_or_else(|| AppError::conflict(tr!("error-replace-duplicate")))?;
    Ok(Json(asset))
}

/// After a batch of replacements: retag the pack, queue previews (Blender) and free discarded originals.
pub(super) async fn unreal_finish(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let id2 = id.clone();
    let st = std::sync::Arc::clone(&state);
    let found = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(false);
        }
        crate::convert::maybe_enqueue(&st, c, &id2)?;
        Ok(true)
    })
    .await?;
    if !found {
        return Err(AppError::not_found(tr!("error-pack-not-found", id = id)));
    }
    crate::extract::after_changes(&state, &id, true, None).await?;
    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
pub(super) struct PreviewBody {
    rules: crate::reformat::Rules,
}

/// What a change to the conversion rules would do to files already in the library, before it's saved.
pub(super) async fn conversions_preview(
    State(state): State<AppState>,
    Json(body): Json<PreviewBody>,
) -> ApiResult<Json<Value>> {
    body.rules.check().map_err(AppError::bad_request)?;
    Ok(Json(db::call(&state.pool, move |c| crate::reformat::preview(c, &body.rules)).await?))
}

/// Convert the files already in the library that the saved rules match.
pub(super) async fn conversions_run(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let st = std::sync::Arc::clone(&state);
    let packs = db::call(&state.pool, move |c| crate::reformat::run_existing(&st, c)).await?;
    Ok(Json(json!({ "packs": packs })))
}

/// Put back the file a conversion rule replaced, while its original is kept.
pub(super) async fn unconvert_asset(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    // A WAV made FLAC is made again first.
    crate::reformat::restore_original(&state, &id).await?;
    let pack = db::call(&state.pool, move |c| {
        Ok(match crate::reformat::put_back(c, &id) {
            Ok(p) => Ok(p),
            Err(Ok(msg)) => Err(msg),
            Err(Err(e)) => return Err(e),
        })
    })
    .await?
    .map_err(AppError::bad_request)?;
    state.wake.notify_one();
    Ok(Json(json!({ "pack_id": pack })))
}
