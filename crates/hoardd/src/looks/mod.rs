//! Search by look: a dominant-color palette and a visual descriptor per thumbnail, for "find by color" and
//! "more like this".
//!
//! Everything is computed from an asset's thumbnail, so textures, models (their renders), materials, PSDs and videos
//! are covered the same way. Features are stored per thumbnail blob (`looks`, keyed by its hash), so a changed
//! thumbnail is simply a new row, and the same picture shared by several assets is done once.
//!
//! - **Palette**: up to eight colors found by k-means in `OKLab` (a perceptual color space), each with the share of
//!   the picture it covers. Transparent pixels (a model render's background) don't count.
//! - **Descriptor**: 127 signed bytes, four blocks each scaled so that the dot product of two descriptors is a
//!   weighted cosine similarity: a soft color histogram (lightness × chroma × hue), a rotation-normalized edge
//!   orientation histogram, surface statistics (contrast, detail at four scales, lightness spread, coverage), and
//!   rotation-invariant local binary patterns at two radii (the "grain" of a texture).
//! - **Perceptual hash**: 64 bits from the low frequencies of a 32×32 DCT, so near-copies rank first.
//! - **Print**: a coarse 12×12 copy of the picture (color and opacity per cell), kept in the table only. Pictures that
//!   look alike by the rest are near copies (see `nearcopies`) when their prints match too: a recolor or a moved part
//!   changes the print where it's changed, a smaller size or a lossy format doesn't.
//!
//! Searching is brute force over an in-memory index (a few hundred bytes per thumbnail).

mod features;
mod index;

use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::Result;
use hoard_core::SearchQuery;
use rusqlite::{OptionalExtension, params};

use crate::{AppState, db};
pub(crate) use features::{analyze, prints, same_print};
pub(crate) use index::Index;

/// Bump when the features change; rows from another version are recomputed by the next looks job.
pub(crate) const VERSION: i64 = 1;

/// Kinds whose thumbnails are pictures of the thing (not a waveform, a type specimen or a page).
pub(crate) const KINDS_SQL: &str =
    "a.kind IN ('texture', 'vector', 'model', 'material', 'animation', 'video', 'other')";

/// One color of a palette, in `OKLab`, with the share of the (opaque part of the) picture it covers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Swatch {
    pub(crate) lab: [f32; 3],
    pub(crate) share: f32,
}

/// What's stored for one thumbnail.
#[derive(Clone, Debug)]
pub(crate) struct Features {
    pub(crate) palette: Vec<Swatch>,
    pub(crate) descriptor: Vec<i8>,
    pub(crate) phash: u64,
    /// See [`print`].
    pub(crate) print: Vec<u8>,
}

static SRGB_TO_LINEAR: LazyLock<[f64; 256]> = LazyLock::new(|| {
    std::array::from_fn(|i| {
        let c = i as f64 / 255.0;
        if c <= 0.040_45 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    })
});

/// sRGB bytes to `OKLab` (L in 0..=1, a and b roughly -0.3..=0.3).
#[expect(clippy::many_single_char_names, reason = "the names color science uses")]
pub(crate) fn oklab(rgb: [u8; 3]) -> [f32; 3] {
    let [r, g, b] = rgb.map(|c| SRGB_TO_LINEAR[usize::from(c)]);
    let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).cbrt();
    let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).cbrt();
    let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).cbrt();
    [
        (0.210_454_255_3 * l + 0.793_617_785 * m - 0.004_072_046_8 * s) as f32,
        (1.977_998_495_1 * l - 2.428_592_205 * m + 0.450_593_709_9 * s) as f32,
        (0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766 * s) as f32,
    ]
}

/// `OKLab` back to sRGB bytes, clipped to the gamut.
#[expect(clippy::many_single_char_names, reason = "the names color science uses")]
pub(crate) fn srgb(lab: [f32; 3]) -> [u8; 3] {
    let [l, a, b] = lab.map(f64::from);
    let l_ = (l + 0.396_337_777_4 * a + 0.215_803_757_3 * b).powi(3);
    let m_ = (l - 0.105_561_345_8 * a - 0.063_854_172_8 * b).powi(3);
    let s_ = (l - 0.089_484_177_5 * a - 1.291_485_548 * b).powi(3);
    let lin = [
        4.076_741_662_1 * l_ - 3.307_711_591_3 * m_ + 0.230_969_929_2 * s_,
        -1.268_438_004_6 * l_ + 2.609_757_401_1 * m_ - 0.341_319_396_5 * s_,
        -0.004_196_086_3 * l_ - 0.703_418_614_7 * m_ + 1.707_614_701 * s_,
    ];
    lin.map(|c| {
        let c = c.clamp(0.0, 1.0);
        let v = if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
        (v * 255.0).round() as u8
    })
}

pub(crate) fn hex(lab: [f32; 3]) -> String {
    let [r, g, b] = srgb(lab);
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// `#rrggbb`, `rrggbb` or `#rgb` to `OKLab`.
pub(crate) fn parse_hex(s: &str) -> Option<[f32; 3]> {
    let s = s.trim().trim_start_matches('#');
    let digits: Vec<u8> = s.chars().map(|c| c.to_digit(16).map(|d| d as u8)).collect::<Option<_>>()?;
    let rgb = match digits.as_slice() {
        [r, g, b] => [r * 17, g * 17, b * 17],
        [r1, r2, g1, g2, b1, b2] => [r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2],
        _ => return None,
    };
    Some(oklab(rgb))
}

fn dist2(x: [f32; 3], y: [f32; 3]) -> f32 {
    (x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2) + (x[2] - y[2]).powi(2)
}

/// Four bytes a color: L × 255, a × 400, b × 400 (signed), share × 255.
pub(crate) fn encode_palette(palette: &[Swatch]) -> Vec<u8> {
    palette
        .iter()
        .flat_map(|s| {
            [
                (s.lab[0] * 255.0).round().clamp(0.0, 255.0) as u8,
                (s.lab[1] * 400.0).round().clamp(-127.0, 127.0) as i8 as u8,
                (s.lab[2] * 400.0).round().clamp(-127.0, 127.0) as i8 as u8,
                (s.share * 255.0).round().clamp(0.0, 255.0) as u8,
            ]
        })
        .collect()
}

pub(crate) fn decode_palette(bytes: &[u8]) -> Vec<Swatch> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| Swatch {
            lab: [f32::from(c[0]) / 255.0, f32::from(c[1] as i8) / 400.0, f32::from(c[2] as i8) / 400.0],
            share: f32::from(c[3]) / 255.0,
        })
        .collect()
}

/// The palette of a thumbnail as shown on an asset's page, most of the picture first.
pub(crate) fn palette_of(conn: &rusqlite::Connection, thumb_hash: &str) -> Result<Vec<hoard_core::Swatch>> {
    let bytes: Option<Vec<u8>> =
        conn.query_row("SELECT palette FROM looks WHERE thumb_hash = ?1", [thumb_hash], |r| r.get(0)).optional()?;
    Ok(shown(&bytes.unwrap_or_default()))
}

/// A stored palette as an asset's page shows it.
fn shown(stored: &[u8]) -> Vec<hoard_core::Swatch> {
    decode_palette(stored)
        .into_iter()
        .map(|s| hoard_core::Swatch { hex: hex(s.lab), share: (s.share * 100.0).round() / 100.0 })
        .collect()
}

/// A library search, ranked by look when it asks for "more like this" (`similar`) or a color (`color`), or by sound
/// for a word imitating one (`imitation`).
pub(crate) async fn search(state: &AppState, q: SearchQuery, facets: bool) -> Result<db::SearchPage> {
    let rank = rank(state, &q).await?;
    db::call(&state.pool, move |c| db::search_ranked(c, &q, facets, rank.as_ref())).await
}

async fn rank(state: &AppState, q: &SearchQuery) -> Result<Option<db::LookRank>> {
    // A word imitating a sound: the sounds shaped like it.
    if let Some(word) = q.imitation.as_deref().filter(|w| !w.trim().is_empty()) {
        let scores = crate::imitation::rank(state, word).await?;
        return Ok(Some(db::LookRank { scores, exclude: None, blend: false, keys: db::Keys::Files }));
    }
    let similar = q.similar.clone().filter(|s| !s.is_empty());
    let color = q.color.as_deref().filter(|c| !c.is_empty()).map(parse_hex);
    if similar.is_none() && color.is_none() {
        // Words alone: with image search on, pictures that show them match too, beside names that have them.
        let text = q.q.as_deref().unwrap_or("").trim();
        if text.is_empty() {
            return Ok(None);
        }
        // And with sound search on, sounds that sound like them.
        let (pictures, sounds) =
            tokio::join!(crate::clip::search_text(state, text), crate::clap::search_text(state, text));
        let pictures = pictures.unwrap_or_else(|e| {
            tracing::warn!(error = %format!("{e:#}"), "image search failed; searching names only");
            Vec::new()
        });
        let mut sounds = sounds.unwrap_or_else(|e| {
            tracing::warn!(error = %format!("{e:#}"), "sound search failed; searching names only");
            Vec::new()
        });
        // Searching sounds, a single word may be a written noise ("whooom"): sounds shaped like it match too.
        if q.kind.as_deref() == Some("audio") && !text.contains(char::is_whitespace) {
            let shaped = crate::imitation::rank(state, text).await?;
            sounds = with_shapes(sounds, shaped);
        }
        let keys = match (pictures.is_empty(), sounds.is_empty()) {
            (true, true) => return Ok(None),
            (false, true) => db::Keys::Thumbs,
            (true, false) => db::Keys::Files,
            (false, false) => db::Keys::Mixed,
        };
        let mut scores = pictures;
        scores.extend(sounds);
        return Ok(Some(db::LookRank { scores, exclude: None, blend: true, keys }));
    }
    let mut scores: Option<Vec<(String, f32)>> = None;
    if let Some(id) = &similar {
        let id = id.clone();
        let asset: Option<(String, String, Option<String>)> = db::call(&state.pool, move |c| {
            Ok(c.query_row("SELECT kind, blob_hash, thumb_hash FROM assets WHERE id = ?1", [id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .optional()?)
        })
        .await?;
        let (kind, file, thumb) = asset.unwrap_or_default();
        // A sound: what sounds like it (a waveform's colors say nothing).
        if kind == "audio" {
            state.sounds.ensure(state).await?;
            let scores = state.sounds.similar(&file);
            return Ok(Some(db::LookRank { scores, exclude: similar, blend: false, keys: db::Keys::Files }));
        }
        state.looks.ensure(state).await?;
        scores = Some(match thumb {
            Some(h) => {
                let meaning = crate::clip::similar(state, &h).await;
                state.looks.similar(&h, meaning.as_ref())
            }
            None => Vec::new(),
        });
    }
    if let Some(color) = color {
        state.looks.ensure(state).await?;
        // A color that doesn't parse matches nothing rather than everything.
        let by_color = color.map_or_default(|lab| state.looks.by_color(lab));
        scores = Some(match scores {
            // Both: what looks alike and has the color, by both scores.
            Some(similar) => {
                let by_color: HashMap<String, f32> = by_color.into_iter().collect();
                similar.into_iter().filter_map(|(h, s)| by_color.get(&h).map(|c| (h, s + c))).collect()
            }
            None => by_color,
        });
    }
    Ok(Some(db::LookRank {
        scores: scores.unwrap_or_default(),
        exclude: similar,
        blend: false,
        keys: db::Keys::Thumbs,
    }))
}

/// Sound search scores (by file) with how well each sound fits a written noise added. A fit (from the floor to 1)
/// counts for at most 0.2, under a word in the name (0.25) or a close sound match, so the noise sorts the sounds the
/// words found and brings in the rest after them.
fn with_shapes(sounds: Vec<(String, f32)>, shaped: Vec<(String, f32)>) -> Vec<(String, f32)> {
    use crate::imitation::FLOOR;
    let mut by_file: HashMap<String, f32> = sounds.into_iter().collect();
    for (file, fit) in shaped {
        let add = (fit - FLOOR) / (1.0 - FLOOR) * 0.2;
        if add > 0.0 {
            *by_file.entry(file).or_default() += add;
        }
    }
    let mut out: Vec<(String, f32)> = by_file.into_iter().collect();
    out.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    out
}

const MISSING_SQL: &str =
    "NOT EXISTS (SELECT 1 FROM looks l WHERE l.thumb_hash = a.thumb_hash AND l.version = ?1 AND l.print IS NOT NULL)";
const ORPHAN_SQL: &str = "NOT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash = l.thumb_hash)";

/// Queue a looks job when some thumbnail has no look yet (or one no longer belongs to any asset).
pub(crate) async fn schedule(state: &AppState) {
    let queued = db::call(&state.pool, |c| {
        let waiting: bool = c.query_row(
            "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued')",
            [crate::jobs::LOOKS],
            |r| r.get(0),
        )?;
        if waiting {
            return Ok(false);
        }
        let work: bool = c.query_row(
            &format!(
                "SELECT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash IS NOT NULL AND {KINDS_SQL} AND {MISSING_SQL})
                     OR EXISTS (SELECT 1 FROM looks l WHERE {ORPHAN_SQL})"
            ),
            [VERSION],
            |r| r.get(0),
        )?;
        if work {
            db::enqueue(c, crate::jobs::LOOKS, None, &serde_json::json!({}))?;
        }
        Ok(work)
    })
    .await;
    match queued {
        Ok(true) => state.wake.notify_one(),
        Ok(false) => {}
        Err(e) => tracing::warn!(error = ?e, "could not queue the looks job"),
    }
    crate::clip::schedule(state).await;
    crate::sounds::schedule(state).await;
    crate::clap::schedule(state).await;
}

/// Thumbnails done per round (read, analyzed, written).
const BATCH: usize = 96;

/// Work out the look of every thumbnail that has none, and forget the looks of thumbnails nothing uses.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    state.looks.ensure(state).await?;
    let gone: Vec<String> = db::call(&state.pool, |c| {
        let gone = c
            .prepare(&format!("SELECT thumb_hash FROM looks l WHERE {ORPHAN_SQL}"))?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        c.execute(&format!("DELETE FROM looks AS l WHERE {ORPHAN_SQL}"), [])?;
        Ok(gone)
    })
    .await?;
    state.looks.remove(&gone);

    // Thumbnails that couldn't be read this time (storage unreachable): left for a later run.
    let mut unreadable = std::collections::HashSet::new();
    let mut done = 0usize;
    loop {
        let skip = unreadable.clone();
        let todo: Vec<String> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT DISTINCT a.thumb_hash FROM assets a
                      WHERE a.thumb_hash IS NOT NULL AND {KINDS_SQL} AND {MISSING_SQL}"
                ))?
                .query_map([VERSION], |r| r.get(0))?
                .collect::<rusqlite::Result<Vec<String>>>()?;
            Ok(rows.into_iter().filter(|h| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return Ok(());
        }
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            crate::activity::job_progress(crate::jobs::LOOKS, None, done, total, "pictures");
            let mut pictures = Vec::with_capacity(chunk.len());
            for hash in chunk {
                match state.blobs.read_all(hash).await {
                    Ok(bytes) => pictures.push((hash.clone(), bytes)),
                    Err(e) => {
                        tracing::debug!(hash, error = %e, "thumbnail unreadable for looks");
                        unreadable.insert(hash.clone());
                    }
                }
            }
            let computed = crate::heavy::run("looks", move || analyze_all(&pictures)).await?;
            save(state, &computed).await?;
            done += chunk.len();
            for (hash, f) in computed {
                if let Some(f) = f {
                    state.looks.insert(hash, f);
                }
            }
        }
        crate::activity::job_progress(crate::jobs::LOOKS, None, done, total, "pictures");
    }
}

/// The look of a thumbnail that just arrived (one a browser drew), at once rather than with the next looks job, so
/// its palette is there when the asset is shown again. The job still runs for what else waits (image search, the old
/// thumbnail's look). Returns its palette.
pub(crate) async fn analyze_now(state: &AppState, hash: String, bytes: Vec<u8>) -> Result<Vec<hoard_core::Swatch>> {
    let features = crate::heavy::run("looks", move || analyze(&bytes)).await?;
    let computed = [(hash, features)];
    save(state, &computed).await?;
    let [(hash, features)] = computed;
    let palette = features.as_ref().map_or_default(|f| shown(&encode_palette(&f.palette)));
    if let Some(f) = features {
        state.looks.insert(hash, f);
    }
    schedule(state).await;
    Ok(palette)
}

async fn save(state: &AppState, computed: &[(String, Option<Features>)]) -> Result<()> {
    /// A thumbnail's hash, palette, descriptor, phash and print, as stored.
    type Row = (String, Vec<u8>, Vec<u8>, i64, Vec<u8>);
    let rows: Vec<Row> = computed
        .iter()
        .map(|(hash, f)| match f {
            Some(f) => (
                hash.clone(),
                encode_palette(&f.palette),
                f.descriptor.iter().map(|&v| v as u8).collect(),
                f.phash as i64,
                f.print.clone(),
            ),
            // Not a picture after all: an empty row, so it isn't tried again.
            None => (hash.clone(), Vec::new(), Vec::new(), 0, Vec::new()),
        })
        .collect();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO looks (thumb_hash, version, palette, descriptor, phash, print)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?;
            for (hash, palette, descriptor, phash, print) in &rows {
                stmt.execute(params![hash, VERSION, palette, descriptor, phash, print])?;
            }
        }
        tx.commit()?;
        Ok(())
    })
    .await
}

/// Analyze pictures on a few threads (half the cores: this is background work).
fn analyze_all(pictures: &[(String, bytes::Bytes)]) -> Vec<(String, Option<Features>)> {
    let threads = std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1));
    let per = pictures.len().div_ceil(threads).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = pictures
            .chunks(per)
            .map(|chunk| {
                scope.spawn(move || {
                    crate::heavy::lower_priority();
                    chunk.iter().map(|(h, b)| (h.clone(), analyze(b))).collect::<Vec<_>>()
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looks::features::print_distance;
    use crate::looks::index::{Entry, SIMILAR_FLOOR, color_score, similarity};

    fn png(img: &image::RgbaImage) -> Vec<u8> {
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }

    /// Cheap deterministic value noise in 0..=1.
    fn noise(seed: u32, x: u32, y: u32, scale: f32) -> f32 {
        let hash = |ix: i64, iy: i64| {
            let mut h =
                (ix as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (iy as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
            h ^= u64::from(seed).wrapping_mul(0x1656_67B1_9E37_79F9);
            h ^= h >> 29;
            h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
            h ^= h >> 32;
            (h & 0xFFFF) as f32 / 65535.0
        };
        let (fx, fy) = (x as f32 / scale, y as f32 / scale);
        let (ix, iy) = (fx.floor() as i64, fy.floor() as i64);
        let (tx, ty) = (fx - fx.floor(), fy - fy.floor());
        let top = hash(ix, iy) * (1.0 - tx) + hash(ix + 1, iy) * tx;
        let bottom = hash(ix, iy + 1) * (1.0 - tx) + hash(ix + 1, iy + 1) * tx;
        top * (1.0 - ty) + bottom * ty
    }

    fn rock(seed: u32, tint: [f32; 3]) -> image::RgbaImage {
        image::RgbaImage::from_fn(256, 256, |x, y| {
            let v = 0.5 * noise(seed, x, y, 24.0) + 0.3 * noise(seed + 1, x, y, 8.0) + 0.2 * noise(seed + 2, x, y, 3.0);
            let c = tint.map(|t| (t * (0.4 + v) * 255.0).clamp(0.0, 255.0) as u8);
            image::Rgba([c[0], c[1], c[2], 255])
        })
    }

    fn stripes(period: u32, vertical: bool, a: [u8; 3], b: [u8; 3]) -> image::RgbaImage {
        image::RgbaImage::from_fn(256, 256, |x, y| {
            let t = if vertical { x } else { y };
            let c = if (t / period).is_multiple_of(2) { a } else { b };
            image::Rgba([c[0], c[1], c[2], 255])
        })
    }

    fn entry(img: &image::RgbaImage) -> Entry {
        Entry::from(analyze(&png(img)).unwrap()).unwrap()
    }

    #[test]
    fn a_written_noise_sorts_the_sounds_the_words_found_then_adds_the_rest() {
        let found = vec![("name.wav".to_owned(), 0.25), ("close.wav".to_owned(), 0.3)];
        let shaped = vec![("name.wav".to_owned(), 1.0), ("shape.wav".to_owned(), 0.9), ("poor.wav".to_owned(), 0.2)];
        let out = with_shapes(found, shaped);
        let order: Vec<&str> = out.iter().map(|(f, _)| f.as_str()).collect();
        assert_eq!(order, ["name.wav", "close.wav", "shape.wav"]);
        assert!((out[0].1 - 0.45).abs() < 1e-6);
    }

    #[test]
    fn colors_round_trip_through_oklab() {
        for hex_in in ["#3a7d2c", "#000000", "#ffffff", "#ff0000", "#1e90ff"] {
            assert_eq!(hex(parse_hex(hex_in).unwrap()), hex_in);
        }
        assert_eq!(parse_hex("f00").map(hex).as_deref(), Some("#ff0000"));
        assert!(parse_hex("#12345").is_none());
        assert!(parse_hex("red").is_none());
    }

    #[test]
    fn palette_finds_the_colors_and_their_shares() {
        let img = image::RgbaImage::from_fn(200, 100, |x, _| {
            if x < 150 { image::Rgba([200, 30, 30, 255]) } else { image::Rgba([30, 60, 200, 255]) }
        });
        let f = analyze(&png(&img)).unwrap();
        assert_eq!(f.palette.len(), 2, "{:?}", f.palette);
        assert!((f.palette[0].share - 0.75).abs() < 0.05, "{:?}", f.palette);
        assert!(dist2(f.palette[0].lab, oklab([200, 30, 30])) < 0.001);
        assert!(dist2(f.palette[1].lab, oklab([30, 60, 200])) < 0.001);

        let stored = decode_palette(&encode_palette(&f.palette));
        assert_eq!(stored.len(), 2);
        assert!(dist2(stored[0].lab, f.palette[0].lab) < 0.0001);
        assert!((stored[0].share - f.palette[0].share).abs() < 0.01);
    }

    #[test]
    fn transparent_background_is_left_out() {
        let img = image::RgbaImage::from_fn(128, 128, |x, y| {
            if (32..96).contains(&x) && (32..96).contains(&y) {
                image::Rgba([40, 160, 60, 255])
            } else {
                image::Rgba([0, 0, 0, 0])
            }
        });
        let f = analyze(&png(&img)).unwrap();
        assert_eq!(f.palette.len(), 1, "{:?}", f.palette);
        assert!(f.palette[0].share > 0.95);
        assert!(analyze(&png(&image::RgbaImage::new(16, 16))).is_none(), "nothing opaque");
        assert!(analyze(b"not a picture").is_none());
    }

    #[test]
    fn color_search_scores_by_how_much_of_the_picture_is_that_color() {
        let green = oklab([40, 160, 60]);
        let palette = [Swatch { lab: green, share: 0.6 }, Swatch { lab: oklab([200, 200, 200]), share: 0.4 }];
        let exact = color_score(&palette, green);
        assert!((exact - 0.6).abs() < 0.05, "{exact}");
        assert!(color_score(&palette, oklab([60, 170, 70])) > 0.4, "a nearby shade still counts");
        assert!(color_score(&palette, oklab([200, 30, 30])) < 0.02, "red isn't green");
    }

    #[test]
    fn like_looks_like_like() {
        let brown = [0.55, 0.42, 0.3];
        let gray = [0.5, 0.5, 0.5];
        let rock_a = entry(&rock(1, brown));
        let rock_b = entry(&rock(7, brown));
        let gray_rock = entry(&rock(3, gray));
        let planks = entry(&stripes(16, false, [150, 110, 70], [110, 75, 45]));
        let planks_turned = entry(&stripes(16, true, [150, 110, 70], [110, 75, 45]));
        let blue_tiles = entry(&stripes(8, true, [30, 60, 200], [220, 220, 240]));

        let same_kind = similarity(&rock_a, &rock_b);
        assert!(same_kind > SIMILAR_FLOOR, "{same_kind}");
        assert!(same_kind > similarity(&rock_a, &gray_rock), "color counts");
        assert!(similarity(&rock_a, &gray_rock) > similarity(&rock_a, &blue_tiles), "and so does surface");
        assert!(same_kind > similarity(&rock_a, &planks));
        // Only the perceptual hash (which is for near-copies) sees the turn.
        assert!(similarity(&planks, &planks_turned) > 0.75, "turning a texture doesn't change what it looks like");
        assert!(similarity(&rock_a, &blue_tiles) < SIMILAR_FLOOR);
    }

    /// Save as JPEG and read it back.
    fn jpeg(img: &image::RgbaImage, quality: u8) -> image::RgbaImage {
        let rgb = image::DynamicImage::ImageRgba8(img.clone()).to_rgb8();
        let mut out = Vec::new();
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality).encode_image(&rgb).unwrap();
        image::load_from_memory(&out).unwrap().to_rgba8()
    }

    #[test]
    fn prints_tell_a_copy_from_a_variant() {
        let img = rock(5, [0.6, 0.5, 0.4]);
        let print_of = |i: &image::RgbaImage| analyze(&png(i)).unwrap().print;
        let at = |i: &image::RgbaImage, w: u32| image::imageops::resize(i, w, w, image::imageops::FilterType::Lanczos3);
        let original = print_of(&img);
        let dist = |other: &image::RgbaImage| print_distance(&original, &print_of(other)).unwrap();

        // The same picture smaller, bigger or saved lossily is a copy.
        for (what, copy) in [("smaller", at(&img, 64)), ("bigger", at(&img, 1024)), ("jpeg", jpeg(&img, 70))] {
            assert!(same_print(&original, &print_of(&copy), false), "{what}: {:?}", dist(&copy));
        }

        // A recolor, a brighter version, a part painted over, the picture turned, or another rock is not.
        let mut recolored = img.clone();
        for p in recolored.pixels_mut() {
            p.0 = [p[1], p[2], p[0], p[3]];
        }
        let mut brighter = img.clone();
        for p in brighter.pixels_mut() {
            p.0 = [p[0].saturating_add(30), p[1].saturating_add(30), p[2].saturating_add(30), p[3]];
        }
        let mut patched = img.clone();
        for y in 100..140 {
            for x in 100..140 {
                patched.put_pixel(x, y, image::Rgba([200, 40, 40, 255]));
            }
        }
        let turned = image::imageops::rotate90(&img);
        let other = rock(6, [0.6, 0.5, 0.4]);
        for (what, variant) in [
            ("recolored", recolored),
            ("brighter", brighter),
            ("patched", patched),
            ("turned", turned),
            ("other", other),
        ] {
            assert!(!same_print(&original, &print_of(&variant), false), "{what}: {:?}", dist(&variant));
        }

        // A see-through picture against the same picture made solid is not.
        let mut cutout = img.clone();
        for (x, _, p) in cutout.enumerate_pixels_mut() {
            if x < 128 {
                p.0[3] = 0;
            }
        }
        assert!(!same_print(&original, &print_of(&cutout), false));
    }

    #[test]
    fn near_pairs_finds_what_near_finds() {
        let index = Index::default();
        *index.0.write().unwrap() = Some(HashMap::new());
        let img = rock(5, [0.6, 0.5, 0.4]);
        let small = image::imageops::resize(&img, 100, 100, image::imageops::FilterType::Triangle);
        index.insert("a".into(), analyze(&png(&img)).unwrap());
        index.insert("b".into(), analyze(&png(&small)).unwrap());
        index.insert("c".into(), analyze(&png(&rock(6, [0.6, 0.5, 0.4]))).unwrap());
        index.insert("d".into(), analyze(&png(&stripes(16, false, [150, 110, 70], [110, 75, 45]))).unwrap());
        assert_eq!(index.near("a", 6, 0.9), vec!["b".to_owned()]);
        assert_eq!(index.near_pairs(6, 0.9), vec![("a".to_owned(), "b".to_owned())]);
        assert!(index.close("b", "a", 6, 0.9));
        assert!(!index.close("a", "c", 6, 0.9));
    }

    #[test]
    fn near_copies_share_a_perceptual_hash() {
        let img = rock(5, [0.6, 0.5, 0.4]);
        let small = image::imageops::resize(&img, 100, 100, image::imageops::FilterType::Triangle);
        let a = analyze(&png(&img)).unwrap();
        let b = analyze(&png(&small)).unwrap();
        let c = analyze(&png(&rock(6, [0.6, 0.5, 0.4]))).unwrap();
        assert!((a.phash ^ b.phash).count_ones() <= 6, "{}", (a.phash ^ b.phash).count_ones());
        assert!((a.phash ^ c.phash).count_ones() >= 16, "{}", (a.phash ^ c.phash).count_ones());
    }
}
