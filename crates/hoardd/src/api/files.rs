//! Files: an asset's own, its thumbnail, a blob, a file in a pack, a derivative and the original, with ranges
//! and a sandbox for anything that could run scripts.

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use object_store::GetRange;
use rusqlite::OptionalExtension as _;
use serde::Deserialize;

use crate::AppState;
use crate::blobs::is_valid_hash;
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::previews::AvifThumb;

#[derive(Debug, Deserialize)]
pub(super) struct FileParams {
    #[serde(default)]
    download: Option<u8>,
}

pub(super) async fn asset_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<FileParams>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let names = state.settings.get();
    let download = p.download.is_some_and(|d| d != 0);
    let (hash, path, ext, renamed) = db::call(&state.pool, move |c| {
        let Some((hash, path, ext)) = db::asset_file_info(c, &id2)? else { return Ok(None) };
        // A texture map goes out named by the convention in Settings.
        let renamed = if download { crate::naming::file_name(c, &names.texture_names, &id2, &path)? } else { None };
        Ok(Some((hash, path, ext, renamed)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    let mime = mime_guess::from_ext(&ext).first_or_octet_stream().to_string();
    let disposition = download.then(|| {
        let file = renamed.as_deref().unwrap_or_else(|| path.rsplit('/').next().unwrap_or(&path));
        format!("attachment; filename=\"{}\"", file.replace(['"', '\\'], "_"))
    });
    serve_blob(&state, &hash, &headers, &mime, disposition).await
}

#[derive(Debug, Deserialize)]
pub(super) struct ThumbQuery {
    /// The thumbnail's revision (`Asset::thumb_rev`). With it the URL names one picture, cached for good; without
    /// it the picture may change, so browsers check again.
    v: Option<String>,
}

pub(super) async fn asset_thumb(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(q): Query<ThumbQuery>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let hash = db::call(&state.pool, move |c| db::asset_thumb(c, &id2))
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-thumbnail-not-found", id = id)))?;
    let (resp, versioned) = match crate::previews::avif_thumb(&state, &id, &hash, &headers).await? {
        AvifThumb::Made(resp) => (resp, q.v.is_some()),
        // Checked again next time, when the AVIF may be there.
        AvifThumb::Making => (stored_thumb(&state, &hash, &headers).await?, false),
        AvifThumb::Stored => (stored_thumb(&state, &hash, &headers).await?, q.v.is_some()),
    };
    let resp = if versioned { resp } else { crate::previews::unversioned(resp) };
    // With AVIF on, what's sent depends on what the browser says it reads.
    Ok(if state.settings.get().thumb_avif { crate::previews::vary(resp) } else { resp })
}

/// A thumbnail as it's stored.
async fn stored_thumb(state: &AppState, hash: &str, headers: &HeaderMap) -> ApiResult<Response> {
    // Thumbnails are JPEG or PNG; the first bytes say which. A browser revalidating its copy gets a 304 without them.
    let revalidating =
        headers.get(header::IF_NONE_MATCH).is_some_and(|v| v.as_bytes() == format!("\"{hash}\"").as_bytes());
    let mime = if revalidating {
        "image/png"
    } else {
        crate::analyze::thumb_mime(&state.blobs.head(hash, 4).await.unwrap_or_default())
    };
    serve_blob(state, hash, headers, mime, None).await
}

pub(super) async fn blob(
    State(state): State<AppState>,
    Path(hash): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    if !is_valid_hash(&hash) {
        return Err(AppError::bad_request(tr!("error-not-a-hash")));
    }
    serve_blob(&state, &hash, &headers, "application/octet-stream", None).await
}

/// Parse a single-range `Range: bytes=…` header. Multi-range requests get the full body.
fn parse_range(headers: &HeaderMap) -> Option<GetRange> {
    let v = headers.get(header::RANGE)?.to_str().ok()?.strip_prefix("bytes=")?;
    if v.contains(',') {
        return None;
    }
    let (start, end) = v.split_once('-')?;
    match (start.trim(), end.trim()) {
        ("", n) => Some(GetRange::Suffix(n.parse().ok()?)),
        (s, "") => Some(GetRange::Offset(s.parse().ok()?)),
        (s, e) => {
            let (s, e): (u64, u64) = (s.parse().ok()?, e.parse().ok()?);
            (e >= s).then(|| GetRange::Bounded(s..e + 1))
        }
    }
}

/// What a file of this type may do when it's opened on its own. Files are whatever people uploaded, served from the
/// portal's origin, so an SVG or HTML page with a script in it would run as whoever opened it: sandboxed with
/// nothing allowed, it can't. Pictures, sound, video and PDFs are left alone, since browsers show them with their own
/// viewers (a sandboxed PDF isn't shown at all).
fn file_policy(content_type: &str) -> Option<&'static str> {
    let ct = content_type.split(';').next().unwrap_or("").trim().to_ascii_lowercase();
    let viewer = (ct.starts_with("image/") && !ct.contains("svg"))
        || ct.starts_with("audio/")
        || ct.starts_with("video/")
        || ct == "application/pdf";
    (!viewer).then_some("default-src 'none'; style-src 'unsafe-inline'; img-src data:; sandbox")
}

/// Stream a blob from storage to the client. This is the proxy path: when blobs live on a NAS, bytes flow
/// NAS -> this server -> client. Blobs are immutable, so the browser may keep them forever, and so may anything in
/// front (Cloudflare, nginx) when guest browsing is on: otherwise they're private, since a shared cache would hand
/// them to anyone with the address.
pub(crate) async fn serve_blob(
    state: &AppState,
    hash: &str,
    headers: &HeaderMap,
    content_type: &str,
    disposition: Option<String>,
) -> ApiResult<Response> {
    let etag = format!("\"{hash}\"");
    if headers.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()) == Some(etag.as_str()) {
        return Ok((StatusCode::NOT_MODIFIED, [(header::ETAG, etag)]).into_response());
    }
    let range = parse_range(headers);
    let partial = range.is_some();
    let read = match crate::profiler::disk(state.blobs.read(hash, range)).await {
        Ok(Some(r)) => r,
        Ok(None) => return Err(AppError::not_found(tr!("error-blob-not-found", hash = hash))),
        Err(_) if partial => return Ok(StatusCode::RANGE_NOT_SATISFIABLE.into_response()),
        Err(e) => return Err(e.into()),
    };
    let len = read.range.end - read.range.start;
    let mut resp = Response::builder()
        .status(if partial { StatusCode::PARTIAL_CONTENT } else { StatusCode::OK })
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, len)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::ETAG, &etag)
        .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
        .header(
            header::CACHE_CONTROL,
            if state.settings.get().guest_browsing {
                "public, max-age=31536000, immutable"
            } else {
                "private, max-age=31536000, immutable"
            },
        );
    if let Some(policy) = file_policy(content_type) {
        resp = resp.header(header::CONTENT_SECURITY_POLICY, policy);
    }
    if partial {
        resp = resp.header(
            header::CONTENT_RANGE,
            format!("bytes {}-{}/{}", read.range.start, read.range.end.saturating_sub(1), read.total_size),
        );
    }
    if let Some(d) = disposition
        && let Ok(v) = HeaderValue::from_str(&d)
    {
        resp = resp.header(header::CONTENT_DISPOSITION, v);
    }
    Ok(resp.body(Body::from_stream(read.stream))?)
}

/// Serve a pack's file by its original path. Browser model loaders resolve relative references
/// (`textures/wood.png` next to `crate.gltf`) against the model's URL, so this makes them just work.
pub(super) async fn pack_file(
    State(state): State<AppState>,
    Path((id, path)): Path<(String, String)>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let p2 = path.clone();
    let (hash, ext) = db::call(&state.pool, move |c| db::pack_file(c, &id2, &p2))
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-file-in-pack-not-found", path = path, id = id)))?;
    let mime = mime_guess::from_ext(&ext).first_or_octet_stream().to_string();
    serve_blob(&state, &hash, &headers, &mime, None).await
}

pub(super) async fn asset_derivative(
    State(state): State<AppState>,
    Path((id, kind)): Path<(String, String)>,
    Query(p): Query<FileParams>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let (id2, k2) = (id.clone(), kind.clone());
    let (hash, path, source_ext) = db::call(&state.pool, move |c| {
        let Some(hash) = db::derivative(c, &id2, &k2)? else { return Ok(None) };
        let (path, ext): (String, Option<String>) =
            c.query_row("SELECT path, json_extract(meta, '$.source_ext') FROM assets WHERE id = ?1", [&id2], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })?;
        Ok(Some((hash, path, ext)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-derivative-not-found", kind = kind, id = id)))?;
    // A recovered source file is served as what it is (a .wav out of a .uasset).
    let ext = match kind.as_str() {
        "glb" => "glb".to_owned(),
        "views" => "png".to_owned(),
        "motion" => "webp".to_owned(),
        "source" => source_ext.unwrap_or_else(|| "bin".to_owned()),
        "preview" => "mp3".to_owned(),
        "sheet" => "json".to_owned(),
        _ => "bin".to_owned(),
    };
    let mime = match ext.as_str() {
        "glb" => "model/gltf-binary".to_owned(),
        e => mime_guess::from_ext(e).first_or_octet_stream().to_string(),
    };
    let disposition = p.download.filter(|d| *d != 0).map(|_| {
        let file = path.rsplit('/').next().unwrap_or(&path);
        let stem = file.rsplit_once('.').map_or(file, |(s, _)| s).replace(['"', '\\'], "_");
        format!("attachment; filename=\"{stem}.{ext}\"")
    });
    serve_blob(&state, &hash, &headers, &mime, disposition).await
}

/// The original file of a converted asset, when it was kept.
pub(super) async fn asset_original(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let kept = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT blob_hash, path FROM asset_origin WHERE asset_id = ?1 AND blob_hash IS NOT NULL",
            [&id2],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        )
        .optional()?)
    })
    .await?;
    let attachment = |path: &str| {
        let file = path.rsplit('/').next().unwrap_or(path).replace(['"', '\\'], "_");
        format!("attachment; filename=\"{file}\"")
    };
    if let Some((hash, path)) = kept {
        return serve_blob(&state, &hash, &headers, "application/octet-stream", Some(attachment(&path))).await;
    }
    // A WAV made FLAC, made again.
    let (wav, path) = crate::reformat::rebuilt_original(&state, &id)
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-original-not-found", id = id)))?;
    let disposition = HeaderValue::from_str(&attachment(&path)).map_err(anyhow::Error::from)?;
    Ok((
        [(header::CONTENT_TYPE, HeaderValue::from_static("audio/wav")), (header::CONTENT_DISPOSITION, disposition)],
        wav,
    )
        .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        let mut h = HeaderMap::new();
        h.insert(header::RANGE, HeaderValue::from_static("bytes=0-99"));
        assert_eq!(parse_range(&h), Some(GetRange::Bounded(0..100)));
        h.insert(header::RANGE, HeaderValue::from_static("bytes=500-"));
        assert_eq!(parse_range(&h), Some(GetRange::Offset(500)));
        h.insert(header::RANGE, HeaderValue::from_static("bytes=-200"));
        assert_eq!(parse_range(&h), Some(GetRange::Suffix(200)));
        h.insert(header::RANGE, HeaderValue::from_static("bytes=0-1,5-6"));
        assert_eq!(parse_range(&h), None);
    }

    #[test]
    fn files_that_could_run_scripts_are_sandboxed() {
        for ct in [
            "image/svg+xml",
            "text/html",
            "application/xhtml+xml",
            "text/xml",
            "application/octet-stream",
            "text/plain",
        ] {
            assert!(file_policy(ct).is_some_and(|p| p.contains("sandbox")), "{ct}");
        }
        for ct in ["image/png", "image/jpeg", "audio/ogg", "video/mp4", "application/pdf"] {
            assert_eq!(file_policy(ct), None, "{ct}");
        }
    }
}
