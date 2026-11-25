//! Textures Hoard's reader converted before `hoard-unreal` r2 came out with red and blue swapped wherever Unreal
//! stored the source as an 8-bit PNG (migration 91 lists them). Each is put right once, at start: made again from
//! the original package where that's kept, which gives exactly what converting it now gives; otherwise from the PNG
//! itself, when what's known of it says Unreal wrote it. The PNGs Hoard wrote from raw pixels were right all along.

use std::collections::BTreeSet;

use anyhow::Result;
use hoard_core::AssetKind;
use hoard_unreal::convert::{self, Outcome};
use hoard_unreal::package::{recolor_png, written_here};
use rusqlite::params;
use serde_json::json;

use crate::{AppState, db, jobs};

/// What made the files on the list, for `asset_origin.converter`.
const BEFORE: &str = "hoard-unreal r1";

struct Todo {
    asset: String,
    /// The file the asset has: only that one is put right.
    blob: String,
    /// The original package, if kept.
    original: Option<String>,
    notes: String,
    pack: String,
}

pub(crate) async fn run(state: AppState) {
    match recolor(&state).await {
        Ok(0) => {}
        Ok(n) => tracing::info!(textures = n, "put red and blue right in textures converted from Unreal packages"),
        Err(e) => tracing::warn!(error = %format!("{e:#}"), "couldn't put right the colors of old Unreal textures"),
    }
}

async fn recolor(state: &AppState) -> Result<usize> {
    let todo = db::call(&state.pool, |c| {
        // One whose asset has since gone or changed has nothing left to put right.
        c.execute(
            "DELETE FROM unreal_recolor WHERE NOT EXISTS (
               SELECT 1 FROM assets a JOIN asset_origin o ON o.asset_id = a.id
                WHERE a.id = unreal_recolor.asset_id AND a.blob_hash = unreal_recolor.blob_hash)",
            [],
        )?;
        let mut stmt = c.prepare(
            "SELECT r.asset_id, r.blob_hash, o.blob_hash, COALESCE(o.notes, ''), a.pack_id
               FROM unreal_recolor r JOIN assets a ON a.id = r.asset_id JOIN asset_origin o ON o.asset_id = a.id",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(Todo { asset: r.get(0)?, blob: r.get(1)?, original: r.get(2)?, notes: r.get(3)?, pack: r.get(4)? })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    let mut packs = BTreeSet::new();
    let mut fixed = 0;
    for t in todo {
        // One that can't be read now (its file missing, the store offline) stays listed for the next start.
        let (bytes, converter) = match fix(state, &t).await {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(asset = %t.asset, error = %format!("{e:#}"), "couldn't check an old Unreal texture's colors");
                continue;
            }
        };
        let blob = match bytes {
            Some(b) => Some(state.blobs.put_bytes(b, AssetKind::from_ext("png")).await?),
            None => None,
        };
        let Todo { asset, blob: was, pack, .. } = t;
        let changed = db::call(&state.pool, move |c| {
            let now = db::now();
            let tx = c.transaction()?;
            let still: bool = tx.prepare_cached("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1 AND blob_hash = ?2)")?.query_row(params![asset, was],
                |r| r.get(0),
            )?;
            if still {
                if let Some(b) = &blob {
                    tx.prepare_cached("INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING")?.execute(params![b.hash, b.size as i64, now],
                    )?;
                    tx.prepare_cached("UPDATE assets SET blob_hash = ?2, updated_at = ?3 WHERE id = ?1")?.execute(params![asset, b.hash, now],
                    )?;
                    tx.prepare_cached("DELETE FROM derivatives WHERE asset_id = ?1 AND (kind IN ('source', 'views') OR kind LIKE 'as:%')")?.execute([&asset],
                    )?;
                }
                tx.prepare_cached("UPDATE asset_origin SET converter = ?2 WHERE asset_id = ?1")?.execute(params![asset, converter])?;
            }
            tx.prepare_cached("DELETE FROM unreal_recolor WHERE asset_id = ?1")?.execute([&asset])?;
            tx.commit()?;
            Ok(still && blob.is_some())
        })
        .await?;
        if changed {
            fixed += 1;
            packs.insert(pack);
        }
    }
    if fixed > 0 {
        // Read again for their thumbnails, which were drawn from the swapped files. One waiting to start already (as
        // migration 89 queued for every pack with Unreal packages) reads the files as they are now; one under way may
        // have read them before.
        db::call(&state.pool, move |c| {
            for p in &packs {
                let waiting: bool = c
                    .prepare_cached(
                        "SELECT EXISTS (SELECT 1 FROM jobs WHERE pack_id = ?1 AND kind = ?2 AND status = 'queued')",
                    )?
                    .query_row(params![p, jobs::RETAG], |r| r.get(0))?;
                if !waiting {
                    db::enqueue(c, jobs::RETAG, Some(p), &json!({ "then_llm": false }))?;
                }
            }
            Ok(())
        })
        .await?;
        state.wake.notify_one();
        jobs::gc(state).await?;
    }
    Ok(fixed)
}

/// The asset's file put right, if it needs to be, and what made it.
async fn fix(state: &AppState, t: &Todo) -> Result<(Option<Vec<u8>>, String)> {
    let png = state.blobs.read_all(&t.blob).await?.to_vec();
    let original = match &t.original {
        Some(h) => Some(state.blobs.read_all(h).await?.to_vec()),
        None => None,
    };
    let notes = t.notes.clone();
    crate::heavy::run("unreal", move || decide(&png, original.as_deref(), &notes)).await?
}

fn decide(png: &[u8], original: Option<&[u8]>, notes: &str) -> Result<(Option<Vec<u8>>, String)> {
    // The package is kept: convert it again, as it's converted now.
    if let Some(data) = original
        && let Ok(pkg) = hoard_unreal::parse(data)
        && let Outcome::Converted { file, .. } = convert::convert(data, &pkg)
        && file.ext == "png"
    {
        let converter = crate::extract::native_converter(file.how);
        return Ok(((file.bytes != png).then_some(file.bytes), converter));
    }
    let now = convert::REVISION;
    if !unreal_wrote(notes, png) {
        return Ok((None, format!("{BEFORE}: raw source pixels (r{now} found its colors right)")));
    }
    Ok(match recolor_png(png)? {
        Some(fixed) => (Some(fixed), format!("{BEFORE}: PNG source as stored; then r{now}: red and blue put right")),
        None => (None, format!("{BEFORE}: PNG source as stored (16-bit or gray)")),
    })
}

/// Whether a PNG made before r2 is the source as Unreal stored it, red and blue swapped, rather than one Hoard wrote
/// from raw pixels, which is right already. The notes kept with it say so where the package did; where it didn't
/// (UE4 marks a PNG source with a flag that wasn't noted), the PNG itself does.
fn unreal_wrote(notes: &str, png: &[u8]) -> bool {
    // "64x64 TSF_BGRA8 TSCF_PNG": size, format, then how Unreal compressed it.
    let first = notes.split(';').next().unwrap_or_default();
    if first.contains("TSF_RGBA8_DEPRECATED") {
        return false;
    }
    if first.contains("from inline bulk data") {
        return true;
    }
    let mut words = first.split_whitespace();
    let size = words
        .next()
        .and_then(|s| s.split_once('x'))
        .and_then(|(w, h)| Some((w.parse::<u64>().ok()?, h.parse::<u64>().ok()?)));
    match words.nth(1) {
        Some("TSCF_PNG" | "1") => true,
        // A PNG anyway, where the source was found by its signature.
        Some(_) => !written_here(png),
        // UE4 compresses only a source larger than 4x4.
        None => !size.is_some_and(|(w, h)| w <= 4 || h <= 4) && !written_here(png),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(img: &image::DynamicImage) -> Vec<u8> {
        let mut out = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png).unwrap();
        out
    }

    /// As libpng might have: same pixels, other bytes.
    fn unreals(img: &image::RgbaImage) -> Vec<u8> {
        use image::ImageEncoder;
        let mut out = Vec::new();
        image::codecs::png::PngEncoder::new_with_quality(
            &mut out,
            image::codecs::png::CompressionType::Best,
            image::codecs::png::FilterType::Paeth,
        )
        .write_image(img.as_raw(), img.width(), img.height(), image::ExtendedColorType::Rgba8)
        .unwrap();
        out
    }

    fn pixel(png: &[u8]) -> [u8; 4] {
        image::load_from_memory(png).unwrap().to_rgba8().get_pixel(0, 0).0
    }

    #[test]
    fn what_unreal_stored_is_put_right_and_what_hoard_wrote_is_left() {
        // Red, as Unreal stores it: blue.
        let stored = image::RgbaImage::from_pixel(16, 16, image::Rgba([0, 0, 255, 255]));
        let theirs = unreals(&stored);
        let ours = png(&stored.clone().into());

        for notes in ["16x16 TSF_BGRA8 TSCF_PNG", "16x16 TSF_BGRA8 1", "16x16 TSF_BGRA8", "from inline bulk data"] {
            let (fixed, by) = decide(&theirs, None, notes).unwrap();
            assert_eq!(pixel(&fixed.expect(notes)), [255, 0, 0, 255], "{notes}");
            assert!(by.ends_with("red and blue put right"), "{by}");
        }
        // Written from raw pixels: right already, whatever the notes leave unsaid.
        for notes in ["16x16 TSF_BGRA8", "16x16 TSF_BGRA8 TSCF_None", "16x16 TSF_BGRA8 0"] {
            let (fixed, by) = decide(&ours, None, notes).unwrap();
            assert!(fixed.is_none(), "{notes}");
            assert!(by.contains("found its colors right"), "{by}");
        }
        // Formats Unreal didn't swap.
        assert!(decide(&theirs, None, "16x16 TSF_RGBA8_DEPRECATED").unwrap().0.is_none());
        let gray = png(&image::DynamicImage::ImageLuma8(image::GrayImage::new(16, 16)));
        assert!(decide(&gray, None, "16x16 TSF_G8 TSCF_PNG").unwrap().0.is_none());
        // UE4 never compressed a source this small, so it's Hoard's.
        let tiny = unreals(&image::RgbaImage::from_pixel(4, 4, image::Rgba([0, 0, 255, 255])));
        assert!(decide(&tiny, None, "4x4 TSF_BGRA8").unwrap().0.is_none());
    }

    /// An original that isn't a package Hoard can read leaves the PNG to decide.
    #[test]
    fn an_unreadable_original_falls_back_to_the_png() {
        let theirs = unreals(&image::RgbaImage::from_pixel(8, 8, image::Rgba([0, 0, 255, 255])));
        let (fixed, _) = decide(&theirs, Some(b"not a package"), "8x8 TSF_BGRA8 TSCF_PNG").unwrap();
        assert_eq!(pixel(&fixed.unwrap()), [255, 0, 0, 255]);
    }
}
