//! Pictures of an asset made beyond its analysis:
//!
//! - **Light previews** of images: the picture at most 2048 px on its longer side, so an 8K texture opens (and
//!   textures a model, in the 3D viewer) without fetching tens of megabytes first. Made at ingest for big images,
//!   or on first request; stored as the `lite` derivative. Images small enough and readable by browsers are their own
//!   preview.
//! - **Frame sheets** of Aseprite files: every frame side by side, for the portal to play (see
//!   `textures::aseprite_sheet`). Made on first request and stored as the `frames` derivative.
//! - **Thumbnails drawn by a browser**: the 3D viewer draws a model with all its textures, lit as it shows it, and a
//!   signed-in person's browser sends that picture to replace the server's. Where a thumbnail came from is kept, so
//!   a worse one never replaces a better one and re-analysis leaves it be.

use crate::i18n::tr;
use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::Response;
use image::{DynamicImage, GenericImageView as _};
use rusqlite::{OptionalExtension as _, params};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::db;
use crate::error::{ApiResult, AppError};

/// The longer side of a light preview.
pub(crate) const LITE_MAX: u32 = 2048;
/// The derivative kind a light preview is stored as.
pub(crate) const LITE: &str = "lite";
/// What browsers draw themselves. GIF and SVG are left whole however big (animation, vectors).
const NATIVE: &[&str] = &["png", "jpg", "jpeg", "webp", "avif", "bmp", "gif", "svg", "ico"];

/// Whether an image needs a light preview made: it's bigger than one, or browsers can't show it as it is.
pub(crate) fn wants_lite(ext: &str, width: u32, height: u32) -> bool {
    match ext {
        "gif" | "svg" | "svgz" | "ico" => false,
        e if NATIVE.contains(&e) => width.max(height) > LITE_MAX,
        _ => true,
    }
}

/// A light preview of decoded pixels: JPEG when opaque, lossless WebP when any pixel is see-through.
pub(crate) fn lite_bytes(img: &DynamicImage) -> anyhow::Result<Vec<u8>> {
    let img = if crate::analyze::is_float(img) {
        crate::analyze::tonemap(&img.thumbnail(LITE_MAX, LITE_MAX))
    } else if img.width().max(img.height()) > LITE_MAX {
        img.thumbnail(LITE_MAX, LITE_MAX)
    } else {
        img.clone()
    };
    let rgba = img.to_rgba8();
    let mut out = Vec::new();
    if rgba.pixels().all(|p| p[3] == 255) {
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 90)
            .encode_image(&DynamicImage::ImageRgba8(rgba).to_rgb8())?;
    } else {
        use image::ImageEncoder as _;
        let (w, h) = rgba.dimensions();
        image::codecs::webp::WebPEncoder::new_lossless(&mut out).write_image(
            rgba.as_raw(),
            w,
            h,
            image::ExtendedColorType::Rgba8,
        )?;
    }
    Ok(out)
}

fn lite_mime(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) { "image/jpeg" } else { "image/webp" }
}

struct ImageRow {
    hash: String,
    ext: String,
    kind: String,
    width: Option<u32>,
    height: Option<u32>,
    size: u64,
    lite: Option<String>,
}

/// `GET /api/assets/{id}/lite`: the image's light preview, made now if it has none yet (the image itself, when it's
/// small enough and browsers read it). The URL stays the same when the asset's file changes, so browsers check back
/// (a 304 by hash) rather than keep it for good.
pub(crate) async fn asset_lite(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    lite_response(state, id, headers).await.map(unversioned)
}

async fn lite_response(state: AppState, id: String, headers: HeaderMap) -> ApiResult<Response> {
    let id2 = id.clone();
    let row = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT a.blob_hash, a.ext, a.kind, json_extract(a.meta, '$.width'), json_extract(a.meta, '$.height'),
                    (SELECT blob_hash FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'lite'),
                    (SELECT size FROM blobs b WHERE b.hash = a.blob_hash)
               FROM assets a WHERE a.id = ?1",
            [&id2],
            |r| {
                Ok(ImageRow {
                    hash: r.get(0)?,
                    ext: r.get::<_, String>(1)?.to_lowercase(),
                    kind: r.get(2)?,
                    width: r.get::<_, Option<f64>>(3)?.map(|v| v as u32),
                    height: r.get::<_, Option<f64>>(4)?.map(|v| v as u32),
                    lite: r.get(5)?,
                    size: r.get::<_, Option<i64>>(6)?.and_then(|v| u64::try_from(v).ok()).unwrap_or(0),
                })
            },
        )
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    if row.kind != "texture" {
        return Err(AppError::bad_request(tr!("error-light-preview-images")));
    }
    if let Some(hash) = &row.lite {
        let mime = lite_mime(&state.blobs.head(hash, 4).await.unwrap_or_default());
        return crate::api::serve_blob(&state, hash, &headers, mime, None).await;
    }
    let size_known = row.width.zip(row.height);
    if size_known.is_some_and(|(w, h)| !wants_lite(&row.ext, w, h)) {
        let mime = mime_guess::from_ext(&row.ext).first_or_octet_stream().to_string();
        return crate::api::serve_blob(&state, &row.hash, &headers, &mime, None).await;
    }
    if row.size > crate::analyze::MAX_IMAGE_DECODE_BYTES {
        return Err(AppError::not_found(tr!("error-preview-too-big")));
    }
    // Made once, now: fetch the original, decode it and keep the preview.
    let tmp = state.cfg.tmp_dir();
    tokio::fs::create_dir_all(&tmp).await?;
    let work = tempfile::Builder::new().prefix("lite-").tempdir_in(&tmp)?;
    let local = work.path().join(format!("original.{}", row.ext));
    let made = async {
        state.blobs.download_to(&row.hash, &local).await?;
        let (ext, path) = (row.ext.clone(), local.clone());
        crate::heavy::run("light preview", move || -> anyhow::Result<Option<Vec<u8>>> {
            let img = crate::formats::load_image(&path, &ext)?;
            let (w, h) = img.dimensions();
            if !wants_lite(&ext, w, h) {
                return Ok(None);
            }
            lite_bytes(&img).map(Some)
        })
        .await?
    }
    .await;
    drop(work);
    let Some(bytes) =
        made.map_err(|e| AppError::bad_request(tr!("error-no-light-preview", reason = format!("{e:#}"))))?
    else {
        let mime = mime_guess::from_ext(&row.ext).first_or_octet_stream().to_string();
        return crate::api::serve_blob(&state, &row.hash, &headers, &mime, None).await;
    };
    let mime = lite_mime(&bytes);
    let blob = save_derivative(&state, id, LITE, bytes).await?;
    crate::api::serve_blob(&state, &blob, &headers, mime, None).await
}

/// `GET /api/assets/{id}/frames.png`: an Aseprite file's frames side by side, as many rows as columns, each the
/// asset's `width`×`height`, to play with its `frame_ms`. 404 for a single frame or one too big for a sheet.
pub(crate) async fn asset_frames(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let (hash, ext, sheet) = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT a.blob_hash, lower(a.ext),
                    (SELECT blob_hash FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'frames')
               FROM assets a WHERE a.id = ?1",
            [&id2],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, Option<String>>(2)?)),
        )
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    if let Some(sheet) = sheet {
        return crate::api::serve_blob(&state, &sheet, &headers, "image/png", None).await.map(unversioned);
    }
    if !matches!(ext.as_str(), "aseprite" | "ase") {
        return Err(AppError::not_found(tr!("error-frame-sheet-aseprite-only")));
    }
    let tmp = state.cfg.tmp_dir();
    tokio::fs::create_dir_all(&tmp).await?;
    let work = tempfile::Builder::new().prefix("frames-").tempdir_in(&tmp)?;
    let local = work.path().join(format!("original.{ext}"));
    state.blobs.download_to(&hash, &local).await?;
    let made = crate::heavy::run("frame sheet", move || crate::textures::aseprite_sheet(&local)).await?;
    drop(work);
    let bytes = made
        .map_err(|e| AppError::bad_request(tr!("error-no-frame-sheet", reason = format!("{e:#}"))))?
        .ok_or_else(|| AppError::not_found(tr!("error-frame-sheet-frames")))?;
    let blob = save_derivative(&state, id, "frames", bytes).await?;
    crate::api::serve_blob(&state, &blob, &headers, "image/png", None).await.map(unversioned)
}

/// Keep a picture made from an asset as its derivative of this kind; returns the picture's hash.
async fn save_derivative(state: &AppState, id: String, kind: &'static str, bytes: Vec<u8>) -> ApiResult<String> {
    let blob = state.blobs.put_cache_bytes(bytes).await?;
    let (hash, size) = (blob.hash.clone(), i64::try_from(blob.size).unwrap_or(i64::MAX));
    db::call(&state.pool, move |c| {
        let now = db::now();
        c.execute(
            "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
            params![hash, size, now],
        )?;
        c.execute(
            "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, ?4, ?2, ?3)
             ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash, created_at = excluded.created_at",
            params![id, hash, now, kind],
        )?;
        Ok(())
    })
    .await?;
    Ok(blob.hash)
}

/// How good a thumbnail's source is: the server's own drawing, a browser's with textures missing, a browser's
/// with everything.
fn rank(source: Option<&str>) -> u8 {
    match source {
        Some("browser") => 2,
        Some("browser-partial") => 1,
        _ => 0,
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ThumbParams {
    /// `browser` (default) or `browser-partial`.
    #[serde(default)]
    source: Option<String>,
}

/// Biggest picture a browser may send for a thumbnail.
pub(crate) const MAX_DRAWN_BYTES: usize = 8 << 20;

/// `PUT /api/assets/{id}/thumb`: a model's thumbnail as a signed-in person's browser drew it (any image format the
/// server reads; it's scaled to thumbnail size). Kept unless the thumbnail there now came from a better source.
pub(crate) async fn put_thumb(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<ThumbParams>,
    body: Bytes,
) -> ApiResult<Json<Value>> {
    let source = match p.source.as_deref() {
        None | Some("browser") => "browser",
        Some("browser-partial") => "browser-partial",
        Some(other) => return Err(AppError::bad_request(tr!("error-thumbnail-source", source = other))),
    };
    if body.len() > MAX_DRAWN_BYTES {
        return Err(AppError::bad_request(tr!("error-thumbnail-too-big")));
    }
    let id2 = id.clone();
    let (kind, current, old) = db::call(&state.pool, move |c| {
        Ok(c.query_row("SELECT kind, thumb_source, thumb_hash FROM assets WHERE id = ?1", [&id2], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?, r.get::<_, Option<String>>(2)?))
        })
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    if !matches!(kind.as_str(), "model" | "animation") {
        return Err(AppError::bad_request(tr!("error-thumbnail-models-only")));
    }
    if rank(current.as_deref()) > rank(Some(source)) {
        return Ok(Json(json!({ "replaced": false, "thumb_source": current })));
    }
    let bytes = crate::heavy::run("drawn thumbnail", move || -> anyhow::Result<Vec<u8>> {
        let img = image::load_from_memory(&body)?;
        let (w, h) = img.dimensions();
        anyhow::ensure!(w.max(h) <= 4096 && w.min(h) >= 16, "a {w}×{h} picture isn't a thumbnail");
        crate::analyze::thumb_bytes(img.thumbnail(256, 256).to_rgba8(), false)
    })
    .await?
    .map_err(|e| AppError::bad_request(tr!("error-not-a-picture", reason = format!("{e:#}"))))?;
    let blob = state.blobs.put_cache_bytes(bytes).await?;
    let (hash, size) = (blob.hash.clone(), i64::try_from(blob.size).unwrap_or(i64::MAX));
    let (id2, src) = (id.clone(), source.to_owned());
    let replaced = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let now = db::now();
        tx.execute(
            "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
            params![hash, size, now],
        )?;
        // Checked again under the write: another browser may have sent a better one meanwhile.
        let n = tx.execute(
            "UPDATE assets SET thumb_hash = ?2, thumb_source = ?3 WHERE id = ?1
               AND (CASE thumb_source WHEN 'browser' THEN 2 WHEN 'browser-partial' THEN 1 ELSE 0 END) <= ?4",
            params![id2, hash, src, rank(Some(&src))],
        )?;
        tx.commit()?;
        Ok(n > 0)
    })
    .await?;
    if let Some(old) = old.filter(|o| replaced && *o != blob.hash) {
        let old_size = db::call(&state.pool, {
            let old = old.clone();
            move |c| Ok(c.query_row("SELECT size FROM blobs WHERE hash = ?1", [&old], |r| r.get(0)).optional()?)
        })
        .await?;
        if let Some(size) = old_size {
            crate::jobs::delete_unused(&state, vec![(old, size)]).await.ok();
        }
    }
    Ok(Json(json!({
        "replaced": replaced,
        "thumb_source": if replaced { Some(source.to_owned()) } else { current },
        "thumb_rev": blob.hash.get(..12),
    })))
}

/// Thumbnails asked for without their revision (`/thumb` rather than `/thumb?v=…`) may change under the same URL,
/// so they're revalidated rather than cached for good.
pub(crate) fn unversioned(mut resp: Response) -> Response {
    resp.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn which_images_get_a_light_preview() {
        assert!(wants_lite("jpg", 8192, 8192));
        assert!(!wants_lite("jpg", 2048, 1024));
        assert!(wants_lite("tga", 512, 512), "browsers can't show TGA");
        assert!(!wants_lite("gif", 4096, 4096), "animated");
        assert!(!wants_lite("svg", 9000, 9000), "vector");
    }

    #[test]
    fn light_preview_is_small_and_keeps_alpha() {
        let big = DynamicImage::ImageRgb8(image::RgbImage::from_pixel(4096, 1024, image::Rgb([10, 200, 30])));
        let out = lite_bytes(&big).unwrap();
        assert_eq!(lite_mime(&out), "image/jpeg");
        let img = image::load_from_memory(&out).unwrap();
        assert_eq!(img.dimensions(), (2048, 512));

        let clear = DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(3000, 3000, image::Rgba([0, 0, 0, 0])));
        let out = lite_bytes(&clear).unwrap();
        assert_eq!(lite_mime(&out), "image/webp");
        assert_eq!(image::load_from_memory(&out).unwrap().dimensions(), (2048, 2048));
    }

    #[test]
    fn better_sources_rank_higher() {
        assert!(rank(Some("browser")) > rank(Some("browser-partial")));
        assert!(rank(Some("browser-partial")) > rank(None));
    }
}
