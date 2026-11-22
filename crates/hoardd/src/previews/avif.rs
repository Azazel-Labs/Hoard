//! Thumbnails sent as AVIF (Settings → Performance → Thumbnails). The stored thumbnails stay JPEG or PNG, since the
//! server reads them back (tagging, image search, looks, material spheres) and has nothing to read AVIF with. A
//! browser that says it reads AVIF gets one made from the stored thumbnail the first time it asks, kept with the
//! conversions (so it's trimmed with them, least recently used first) and sent from then on. Until it's made the stored
//! picture goes out, to be checked again next time, so a page never waits for one. Pixel art, Lossless, and pictures
//! AVIF hardly shrinks (flat colors, line art) stay as they are.

use std::collections::HashSet;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::Response;
use rusqlite::{OptionalExtension as _, params};

use crate::AppState;
use crate::db;
use crate::error::ApiResult;

/// AVIFs made at once: each takes a core for about 50 ms, and a page of new tiles asks for dozens.
static SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(2);
/// Thumbnails an AVIF is being made from, so a picture asked for again meanwhile isn't made twice.
static MAKING: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);
/// Thumbnails AVIF didn't shrink enough to be worth it, not tried again until the server restarts.
static NOT_WORTH_IT: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);
/// Most thumbnails [`NOT_WORTH_IT`] remembers before it starts over.
const REMEMBERED: usize = 100_000;
/// The conversion cache is trimmed after this many AVIFs are added to it.
const TRIM_EVERY: usize = 200;
static MADE: AtomicUsize = AtomicUsize::new(0);

/// The derivative an AVIF made from this thumbnail is kept as. Named for the thumbnail, so a new one is made when the
/// thumbnail changes (the old one is left for the trim); `as:` puts it with the conversions.
fn kind(thumb: &str) -> String {
    format!("as:thumb.avif:{}", thumb.get(..16).unwrap_or(thumb))
}

/// Whether the browser reads AVIF: browsers list it in `Accept` when they ask for a picture.
pub(crate) fn accepts_avif(headers: &HeaderMap) -> bool {
    headers
        .get_all(header::ACCEPT)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .any(|t| t.split(';').next().is_some_and(|t| t.trim().eq_ignore_ascii_case("image/avif")))
}

/// What to send for a thumbnail, AVIF or not.
pub(crate) enum AvifThumb {
    /// The AVIF made before.
    Made(Response),
    /// The stored picture for now: an AVIF is being made in the background.
    Making,
    /// The stored picture.
    Stored,
}

/// The asset's thumbnail (stored as `thumb`) as AVIF, when Settings asks for it and the browser reads it.
pub(crate) async fn avif_thumb(state: &AppState, id: &str, thumb: &str, headers: &HeaderMap) -> ApiResult<AvifThumb> {
    let s = state.settings.get();
    let Some(quality) = s.thumb_quality.avif_quality().filter(|_| s.thumb_avif && accepts_avif(headers)) else {
        return Ok(AvifThumb::Stored);
    };
    if lock(&NOT_WORTH_IT).contains(thumb) {
        return Ok(AvifThumb::Stored);
    }
    let (id2, kind) = (id.to_owned(), kind(thumb));
    let kind2 = kind.clone();
    let row = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT (SELECT blob_hash FROM derivatives WHERE asset_id = ?1 AND kind = ?2),
                    COALESCE(json_extract(meta, '$.\"pixel-art-likely\"'), 0)
               FROM assets WHERE id = ?1",
            params![id2, kind2],
            |r| Ok((r.get::<_, Option<String>>(0)?, r.get::<_, i64>(1)? != 0)),
        )
        .optional()?)
    })
    .await?;
    let Some((made, pixel_art)) = row else { return Ok(AvifThumb::Stored) };
    if pixel_art {
        return Ok(AvifThumb::Stored);
    }
    if let Some(avif) = made {
        crate::formats::touch_conversion(state, id, &kind);
        return crate::api::serve_blob(state, &avif, headers, "image/avif", None).await.map(AvifThumb::Made);
    }
    start(state, id, thumb, kind, quality);
    Ok(AvifThumb::Making)
}

/// Mark a thumbnail response as one that differs by what the browser reads.
pub(crate) fn vary(mut resp: Response) -> Response {
    resp.headers_mut().insert(header::VARY, HeaderValue::from_static("Accept"));
    resp
}

fn lock(set: &Mutex<HashSet<String>>) -> std::sync::MutexGuard<'_, HashSet<String>> {
    set.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Make the AVIF in the background, unless it's being made already.
fn start(state: &AppState, id: &str, thumb: &str, kind: String, quality: u8) {
    if !lock(&MAKING).insert(thumb.to_owned()) {
        return;
    }
    let (state, id, thumb) = (std::sync::Arc::clone(state), id.to_owned(), thumb.to_owned());
    tokio::spawn(async move {
        let made = match SLOTS.acquire().await {
            Ok(_slot) => make(&state, &id, &thumb, kind, quality).await,
            Err(e) => Err(e.into()),
        };
        lock(&MAKING).remove(&thumb);
        match made {
            Ok(true) => {
                if MADE.fetch_add(1, Ordering::Relaxed) % TRIM_EVERY == TRIM_EVERY - 1 {
                    crate::formats::log_trim(&state).await;
                }
            }
            Ok(false) => {
                let mut skip = lock(&NOT_WORTH_IT);
                if skip.len() >= REMEMBERED {
                    skip.clear();
                }
                skip.insert(thumb);
            }
            Err(e) => tracing::debug!(asset = %id, error = %format!("{e:#}"), "couldn't make an AVIF thumbnail"),
        }
    });
}

/// Make the AVIF of every thumbnail that hasn't one yet, instead of as each is first shown (Settings → Maintenance,
/// and the button under the thumbnail choices). It takes the same two slots as those made while browsing, so pages
/// aren't kept waiting, and stops if thumbnails stop being sent as AVIF.
pub(crate) async fn convert_all(state: &AppState, progress: impl Fn(usize, usize)) -> anyhow::Result<String> {
    let quality = || {
        let s = state.settings.get();
        s.thumb_quality.avif_quality().filter(|_| s.thumb_avif)
    };
    let Some(mut q) = quality() else { return Ok("Thumbnails aren't sent as AVIF".to_owned()) };
    let todo: Vec<(String, String)> = db::call(&state.pool, |c| {
        let rows = c
            .prepare(
                "SELECT a.id, a.thumb_hash FROM assets a
                  WHERE a.thumb_hash IS NOT NULL
                    AND COALESCE(json_extract(a.meta, '$.\"pixel-art-likely\"'), 0) = 0
                    AND NOT EXISTS (SELECT 1 FROM derivatives d
                                     WHERE d.asset_id = a.id AND d.kind = 'as:thumb.avif:' || substr(a.thumb_hash, 1, 16))",
            )?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    let total = todo.len();
    let (mut made, mut as_is, mut failed) = (0usize, 0usize, 0usize);
    let mut tally = |done: Result<(String, anyhow::Result<bool>), tokio::task::JoinError>| match done {
        Ok((_, Ok(true))) => made += 1,
        Ok((thumb, Ok(false))) => {
            as_is += 1;
            lock(&NOT_WORTH_IT).insert(thumb);
        }
        Ok((_, Err(e))) => {
            failed += 1;
            tracing::debug!(error = %format!("{e:#}"), "couldn't make an AVIF thumbnail");
        }
        Err(_) => failed += 1,
    };
    let mut making = tokio::task::JoinSet::new();
    let mut stopped = false;
    for (i, (id, thumb)) in todo.into_iter().enumerate() {
        if i % 50 == 0 {
            progress(i, total);
            let Some(now) = quality() else {
                stopped = true;
                break;
            };
            q = now;
        }
        if lock(&NOT_WORTH_IT).contains(&thumb) || !lock(&MAKING).insert(thumb.clone()) {
            continue;
        }
        let slot = SLOTS.acquire().await?;
        let state = std::sync::Arc::clone(state);
        making.spawn(async move {
            let _slot = slot;
            let made = make(&state, &id, &thumb, kind(&thumb), q).await;
            lock(&MAKING).remove(&thumb);
            if matches!(made, Ok(true)) && MADE.fetch_add(1, Ordering::Relaxed) % TRIM_EVERY == TRIM_EVERY - 1 {
                crate::formats::log_trim(&state).await;
            }
            (thumb, made)
        });
        while let Some(done) = making.try_join_next() {
            tally(done);
        }
    }
    while let Some(done) = making.join_next().await {
        tally(done);
    }
    progress(total, total);
    let mut summary = if stopped {
        "Stopped: thumbnails aren't sent as AVIF any more".to_owned()
    } else if made + as_is + failed == 0 {
        "Every thumbnail was already AVIF".to_owned()
    } else {
        format!("Converted {made} of {total} thumbnails")
    };
    if as_is > 0 {
        let _ = write!(summary, "; {as_is} left as they are, AVIF being no smaller");
    }
    if failed > 0 {
        let _ = write!(summary, "; {failed} couldn't be converted");
    }
    Ok(summary)
}

/// Make and keep the AVIF; `false` when it isn't a fifth smaller than the stored picture.
async fn make(state: &AppState, id: &str, thumb: &str, kind: String, quality: u8) -> anyhow::Result<bool> {
    let bytes = state.blobs.read_all(thumb).await?;
    let avif = crate::heavy::run("thumb-avif", move || -> anyhow::Result<Option<Vec<u8>>> {
        let img = image::load_from_memory(&bytes)?.to_rgba8();
        let avif = crate::analyze::avif_bytes(&img, quality)?;
        Ok(worth_it(avif.len(), bytes.len()).then_some(avif))
    })
    .await??;
    let Some(avif) = avif else { return Ok(false) };
    super::save_derivative(state, id.to_owned(), &kind, avif).await.map_err(|e| e.err)?;
    Ok(true)
}

fn worth_it(avif: usize, stored: usize) -> bool {
    avif * 10 < stored * 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browsers_that_read_avif_say_so() {
        let mut h = HeaderMap::new();
        assert!(!accepts_avif(&h));
        h.insert(header::ACCEPT, HeaderValue::from_static("image/avif,image/webp,image/apng,image/*,*/*;q=0.8"));
        assert!(accepts_avif(&h));
        h.insert(header::ACCEPT, HeaderValue::from_static("image/webp,*/*"));
        assert!(!accepts_avif(&h));
        h.insert(header::ACCEPT, HeaderValue::from_static("image/png, Image/AVIF;q=0.9"));
        assert!(accepts_avif(&h));
    }

    #[test]
    fn a_photo_thumbnail_is_smaller_as_avif_and_a_cutout_keeps_its_transparency() {
        // Smooth color with some detail, like a render or a photo: what JPEG thumbnails are.
        let photo = image::RgbaImage::from_fn(256, 256, |x, y| {
            let (fx, fy) = (x as f32 / 256.0, y as f32 / 256.0);
            let ripple = ((fx * 23.0).sin() * (fy * 17.0).cos() * 40.0) as i32;
            let c = |v: f32| (v * 200.0) as i32 + ripple;
            image::Rgba([
                c(fx).clamp(0, 255) as u8,
                c(fy).clamp(0, 255) as u8,
                c(1.0 - fx * fy).clamp(0, 255) as u8,
                255,
            ])
        });
        let jpeg = crate::analyze::thumb_bytes(photo.clone(), false).unwrap();
        assert_eq!(crate::analyze::thumb_mime(&jpeg), "image/jpeg");
        let avif = crate::analyze::avif_bytes(&photo, 68).unwrap();
        assert_eq!(&avif[4..12], b"ftypavif");
        assert!(worth_it(avif.len(), jpeg.len()), "AVIF {} bytes, JPEG {}", avif.len(), jpeg.len());

        let mut cutout = photo;
        for p in cutout.pixels_mut().take(256 * 100) {
            p[3] = 0;
        }
        let png = crate::analyze::thumb_bytes(cutout.clone(), false).unwrap();
        assert_eq!(crate::analyze::thumb_mime(&png), "image/png");
        let avif = crate::analyze::avif_bytes(&cutout, 68).unwrap();
        assert!(worth_it(avif.len(), png.len()), "AVIF {} bytes, PNG {}", avif.len(), png.len());
    }

    #[test]
    fn each_thumbnail_has_its_own_avif() {
        assert_eq!(kind("0123456789abcdef0123"), "as:thumb.avif:0123456789abcdef");
        assert!(kind("0123456789abcdef0123").starts_with("as:"), "kept and trimmed with the conversions");
        assert_eq!(kind("short"), "as:thumb.avif:short");
    }
}
