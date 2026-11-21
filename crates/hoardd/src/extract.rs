//! Get content out of Unreal packages and into common formats, with Hoard's own reader (`hoard-unreal`):
//! textures to PNG/EXR, sounds to WAV, materials and data tables to JSON. The converted file takes over the
//! asset (same id, links, collections and tags), and `asset_origin` remembers what it was. The original package is
//! kept or discarded per the job's mode.
//!
//! Packages only Unreal Editor can convert (meshes, animations, raw sources Unreal stored Oodle-compressed) are
//! recorded in
//! `unreal_status`; `hoard unreal-worker` picks those up, exports them headless, and uploads the results through
//! [`replace_asset`].

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use hoard_core::{AssetKind, AssetOrigin, PendingPackage, UnrealPackStatus, UnrealStatus};
use hoard_unreal::convert::{self, Outcome};
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::blobs::StoredBlob;
use crate::config::UnrealConvert;
use crate::{AppState, db, jobs};

/// Job payload for [`jobs::UNREAL`].
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub(crate) struct Request {
    /// Discard the original packages once converted.
    #[serde(default)]
    pub(crate) discard_originals: bool,
    /// Delete packages that have no common format at all (Blueprints, levels, …).
    #[serde(default)]
    pub(crate) drop_unsupported: bool,
    /// Set when queued by ingest: finish the ingest pipeline afterwards (with or without the LLM stage).
    #[serde(default)]
    pub(crate) then_llm: Option<bool>,
}

/// After ingest: queue conversion if the pack has Unreal packages and conversion is on. Returns whether it did;
/// the conversion job then finishes the ingest pipeline.
pub(crate) fn maybe_enqueue(state: &AppState, conn: &Connection, pack_id: &str) -> Result<bool> {
    let mode = state.settings.get().unreal_convert;
    if mode == UnrealConvert::Off || count_packages(conn, pack_id)? == 0 {
        return Ok(false);
    }
    // A watched folder's packages stay: they're files Hoard doesn't own, and a rescan would only find them again.
    let discard = mode == UnrealConvert::Replace && crate::locations::watched_root(conn, pack_id)?.is_none();
    let req = Request { discard_originals: discard, drop_unsupported: false, then_llm: Some(true) };
    db::enqueue(conn, jobs::UNREAL, Some(pack_id), &serde_json::to_value(req)?)?;
    state.wake.notify_one();
    Ok(true)
}

fn count_packages(conn: &Connection, pack_id: &str) -> Result<i64> {
    Ok(conn.query_row(
        "SELECT COUNT(*) FROM assets WHERE pack_id = ?1 AND ext IN ('uasset', 'umap')",
        [pack_id],
        |r| r.get(0),
    )?)
}

/// Where a converted file goes.
enum Target {
    /// A new path: same folder and name, new extension.
    Path(String),
    /// The pack already ships this exact file (the PNG next to its .uasset): this asset id.
    Twin(String),
}

fn target(conn: &Connection, pack_id: &str, path: &str, ext: &str, hash: &str) -> Result<Target> {
    let stem = path.rsplit_once('.').map_or(path, |(s, _)| s);
    for candidate in [format!("{stem}.{ext}"), format!("{stem}.unreal.{ext}")] {
        let existing: Option<(String, String)> = conn
            .query_row(
                "SELECT id, blob_hash FROM assets WHERE pack_id = ?1 AND path = ?2",
                params![pack_id, candidate],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        match existing {
            None => return Ok(Target::Path(candidate)),
            Some((id, h)) if h == hash => return Ok(Target::Twin(id)),
            Some(_) => {}
        }
    }
    Ok(Target::Path(format!("{stem}.{}.{ext}", &hash[..8])))
}

/// What a converted asset came from.
#[derive(Debug, Clone, Default)]
pub(crate) struct Origin {
    pub(crate) class: Option<String>,
    pub(crate) engine: Option<String>,
    pub(crate) dependencies: Vec<String>,
    pub(crate) method: &'static str,
    pub(crate) notes: Vec<String>,
    /// Which code, at which revision, made the file and how ("hoard-unreal r2: PNG source, red and blue put right").
    /// Empty says only Hoard's version and the method.
    pub(crate) converter: String,
}

/// What [`Origin::converter`] says for a file made by `hoard-unreal` the way `how` says.
pub(crate) fn native_converter(how: &str) -> String {
    format!("hoard-unreal r{}: {how}", convert::REVISION)
}

/// Swap an asset's file for a converted one, recording where it came from. Returns false when the converted file
/// duplicated one already in the pack and the asset was folded into it instead.
pub(crate) fn replace_asset(
    tx: &Transaction<'_>,
    asset_id: &str,
    new: &StoredBlob,
    new_ext: &str,
    origin: &Origin,
    keep_original: bool,
) -> Result<bool> {
    let now = db::now();
    let (pack_id, path, ext, old_hash): (String, String, String, String) = tx
        .query_row("SELECT pack_id, path, ext, blob_hash FROM assets WHERE id = ?1", [asset_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .optional()?
        .context("asset is gone")?;
    // Converting twice keeps the first origin (the package, not an intermediate file), and its hash for good.
    let prior: Option<(String, String, Option<String>, Option<String>)> = tx
        .query_row("SELECT path, ext, blob_hash, orig_hash FROM asset_origin WHERE asset_id = ?1", [asset_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .optional()?;
    let (orig_path, orig_ext, orig_kept, orig_hash) = match prior {
        Some((p, e, kept, hash)) => {
            let hash = hash.or_else(|| kept.clone());
            (p, e, kept, hash)
        }
        None => (path.clone(), ext.clone(), Some(old_hash.clone()), Some(old_hash)),
    };
    tx.execute(
        "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
        params![new.hash, new.size as i64, now],
    )?;
    let deps = json!(origin.dependencies).to_string();
    let notes = origin.notes.join("; ");
    let kept = orig_kept.filter(|_| keep_original);
    let converter = if origin.converter.is_empty() {
        format!("hoard {} {}", env!("CARGO_PKG_VERSION"), origin.method)
    } else {
        origin.converter.clone()
    };
    // A second conversion adds to the record of the first rather than replacing it.
    let record = |target_id: &str| -> rusqlite::Result<usize> {
        tx.execute(
            "INSERT INTO asset_origin (asset_id, path, ext, blob_hash, orig_hash, class, engine, dependencies, method,
                                       notes, converter, converted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
             ON CONFLICT (asset_id) DO UPDATE SET blob_hash = CASE WHEN ?4 IS NULL THEN NULL ELSE blob_hash END,
                 orig_hash = COALESCE(asset_origin.orig_hash, excluded.orig_hash),
                 method = excluded.method, notes = excluded.notes,
                 converter = COALESCE(asset_origin.converter, 'unrecorded') || '; then ' || excluded.converter,
                 converted_at = excluded.converted_at",
            params![
                target_id,
                orig_path,
                orig_ext,
                kept,
                orig_hash,
                origin.class,
                origin.engine,
                deps,
                origin.method,
                notes,
                converter,
                now
            ],
        )
    };
    let new_path = match target(tx, &pack_id, &path, new_ext, &new.hash)? {
        Target::Path(p) => p,
        Target::Twin(twin) => {
            // The twin stands in for the package from now on, unless it already has an origin of its own.
            let has_origin: bool =
                tx.query_row("SELECT EXISTS (SELECT 1 FROM asset_origin WHERE asset_id = ?1)", [&twin], |r| r.get(0))?;
            if !has_origin {
                record(&twin)?;
            }
            tx.execute("DELETE FROM assets WHERE id = ?1", [asset_id])?;
            return Ok(false);
        }
    };
    let kind = AssetKind::from_ext(new_ext);
    tx.execute(
        "UPDATE assets SET blob_hash = ?2, path = ?3, ext = ?4, kind = ?5, words = ?6, updated_at = ?7 WHERE id = ?1",
        params![asset_id, new.hash, new_path, new_ext, kind.as_str(), crate::tagger::fts_words(&new_path), now],
    )?;
    // Previews made from the package (the recovered source, the multi-view sheet) are superseded.
    tx.execute(
        "DELETE FROM derivatives WHERE asset_id = ?1 AND (kind IN ('source', 'views') OR kind LIKE 'as:%')",
        [asset_id],
    )?;
    tx.execute("DELETE FROM unreal_status WHERE asset_id = ?1", [asset_id])?;
    record(asset_id)?;
    Ok(true)
}

/// Record why a package wasn't converted.
pub(crate) fn set_status(
    conn: &Connection,
    asset_id: &str,
    status: &str,
    class: Option<&str>,
    reason: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO unreal_status (asset_id, status, class, reason, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT (asset_id) DO UPDATE SET status = excluded.status, class = excluded.class,
             reason = excluded.reason, updated_at = excluded.updated_at",
        params![asset_id, status, class, reason, db::now()],
    )?;
    Ok(())
}

/// Convert one package's bytes, off the async runtime.
fn convert_file(local: &Path) -> (Outcome, Origin) {
    let data = match std::fs::read(local) {
        Ok(d) => d,
        Err(e) => return (Outcome::Failed(format!("read: {e}")), Origin::default()),
    };
    let pkg = match hoard_unreal::parse(&data) {
        Ok(p) => p,
        Err(e) => return (Outcome::Failed(format!("{e:#}")), Origin::default()),
    };
    let origin = Origin {
        class: pkg.class.clone(),
        engine: Some(pkg.engine.clone()).filter(|e| !e.is_empty()),
        dependencies: pkg.dependencies.clone(),
        method: "native",
        notes: Vec::new(),
        converter: String::new(),
    };
    (convert::convert(&data, &pkg), origin)
}

/// Counts for one run, reported in the log and the job result.
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub(crate) struct Summary {
    pub(crate) converted: usize,
    pub(crate) merged: usize,
    pub(crate) needs_editor: usize,
    pub(crate) unsupported: usize,
    pub(crate) dropped: usize,
    pub(crate) failed: usize,
}

pub(crate) async fn convert_pack(state: &AppState, pack_id: &str, mut req: Request) -> Result<Summary> {
    let pid = pack_id.to_owned();
    if req.discard_originals {
        let pid = pid.clone();
        req.discard_originals =
            db::call(&state.pool, move |c| crate::locations::watched_root(c, &pid)).await?.is_none();
    }
    let todo: Vec<(String, String, String)> = db::call(&state.pool, move |c| {
        let mut stmt =
            c.prepare("SELECT id, path, blob_hash FROM assets WHERE pack_id = ?1 AND ext IN ('uasset', 'umap')")?;
        let rows =
            stmt.query_map([&pid], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?;
        Ok(rows)
    })
    .await?;
    // Discarding applies to originals kept by earlier runs too.
    let released = if req.discard_originals {
        let pid = pack_id.to_owned();
        db::call(&state.pool, move |c| {
            Ok(c.execute(
                "UPDATE asset_origin SET blob_hash = NULL
                 WHERE blob_hash IS NOT NULL AND asset_id IN (SELECT id FROM assets WHERE pack_id = ?1)",
                [&pid],
            )?)
        })
        .await?
    } else {
        0
    };
    let work = state.cfg.tmp_dir().join(format!("unreal-{pack_id}"));
    tokio::fs::create_dir_all(&work).await?;
    let mut sum = Summary::default();
    for (n, (id, path, hash)) in todo.iter().enumerate() {
        crate::activity::job_progress(jobs::UNREAL, Some(pack_id), n, todo.len(), "packages");
        let local = work.join(format!("{n}.uasset"));
        // A package whose file is missing, or that the reader trips over, is that one package failed: the rest of the
        // pack is still converted, and the pack carries on to its tagging.
        let converted = match state.blobs.download_to(hash, &local).await {
            Ok(()) => {
                let l = local.clone();
                crate::heavy::run("unreal", move || convert_file(&l)).await
            }
            Err(e) => Err(e),
        };
        tokio::fs::remove_file(&local).await.ok();
        let (outcome, mut origin) =
            converted.unwrap_or_else(|e| (Outcome::Failed(format!("{e:#}")), Origin::default()));
        let class = origin.class.clone();
        let id2 = id.clone();
        match outcome {
            Outcome::Converted { file, notes } => {
                let blob = state.blobs.put_bytes(file.bytes, AssetKind::from_ext(file.ext)).await?;
                origin.notes = notes;
                origin.converter = native_converter(file.how);
                let ext = file.ext;
                let replaced = db::call(&state.pool, move |c| {
                    let tx = c.transaction()?;
                    let r = replace_asset(&tx, &id2, &blob, ext, &origin, !req.discard_originals)?;
                    tx.commit()?;
                    Ok(r)
                })
                .await?;
                if replaced {
                    sum.converted += 1;
                } else {
                    sum.merged += 1;
                }
            }
            Outcome::Unsupported(why) if req.drop_unsupported => {
                tracing::info!(asset = %path, reason = %why, "dropping package with no common format");
                db::call(&state.pool, move |c| {
                    // A watched folder still has the file: remember it's dropped, or the next rescan adds it back.
                    db::skip_on_rescan(c, &id2)?;
                    Ok(c.execute("DELETE FROM assets WHERE id = ?1", [&id2])?)
                })
                .await?;
                sum.dropped += 1;
            }
            other => {
                let (status, why) = match other {
                    Outcome::NeedsEditor(w) => {
                        sum.needs_editor += 1;
                        ("needs_editor", w)
                    }
                    Outcome::Unsupported(w) => {
                        sum.unsupported += 1;
                        ("unsupported", w)
                    }
                    Outcome::Failed(w) => {
                        sum.failed += 1;
                        tracing::warn!(asset = %path, error = %w, "Unreal conversion failed");
                        ("failed", w)
                    }
                    Outcome::Converted { .. } => unreachable!("handled above"),
                };
                db::call(&state.pool, move |c| set_status(c, &id2, status, class.as_deref(), &why)).await?;
            }
        }
    }
    tokio::fs::remove_dir_all(&work).await.ok();
    if released > 0 {
        let (n, bytes) = jobs::gc(state).await?;
        tracing::info!(released, blobs = n, bytes, "discarded kept originals");
    }
    let changed = sum.converted + sum.merged + sum.dropped > 0;
    // What Unreal packages became may be what a conversion rule converts; that job reads the pack again after.
    let then = crate::reformat::Request { then_llm: req.then_llm, changed };
    let (s, pid) = (std::sync::Arc::clone(state), pack_id.to_owned());
    if !db::call(&state.pool, move |c| crate::reformat::maybe_enqueue(&s, c, &pid, then)).await? {
        after_changes(state, pack_id, changed, req.then_llm).await?;
    }
    tracing::info!(pack = pack_id, ?sum, "Unreal conversion finished");
    Ok(sum)
}

/// Re-analyze the pack (new kinds, thumbnails, tags, links) and free discarded originals. With `then_llm`, the
/// ingest pipeline continues from here.
pub(crate) async fn after_changes(
    state: &AppState,
    pack_id: &str,
    changed: bool,
    then_llm: Option<bool>,
) -> Result<()> {
    let pid = pack_id.to_owned();
    if !changed {
        if let Some(llm) = then_llm {
            let s = std::sync::Arc::clone(state);
            db::call(&state.pool, move |c| crate::ingest::finish_rules_stage(&s, c, &pid, llm)).await?;
        }
        return Ok(());
    }
    db::call(&state.pool, move |c| {
        db::enqueue(c, jobs::RETAG, Some(&pid), &json!({ "then_llm": then_llm.unwrap_or(false) }))?;
        Ok(())
    })
    .await?;
    state.wake.notify_one();
    let (n, bytes) = jobs::gc(state).await?;
    if n > 0 {
        tracing::info!(blobs = n, bytes, "freed discarded originals");
    }
    Ok(())
}

/// Per-pack status for the portal: how many packages are converted, waiting for the editor, or unsupported.
pub(crate) fn pack_status(conn: &Connection, pack_id: &str) -> Result<UnrealPackStatus> {
    let one = |sql: &str| -> Result<i64> { Ok(conn.query_row(sql, [pack_id], |r| r.get(0))?) };
    let mut s = UnrealPackStatus {
        packages: count_packages(conn, pack_id)?,
        converted: one("SELECT COUNT(*) FROM asset_origin o JOIN assets a ON a.id = o.asset_id WHERE a.pack_id = ?1")?,
        originals_kept: one("SELECT COUNT(*) FROM asset_origin o JOIN assets a ON a.id = o.asset_id
             WHERE a.pack_id = ?1 AND o.blob_hash IS NOT NULL")?,
        ..UnrealPackStatus::default()
    };
    let mut stmt = conn.prepare(
        "SELECT u.status, COUNT(*) FROM unreal_status u JOIN assets a ON a.id = u.asset_id WHERE a.pack_id = ?1
         GROUP BY u.status",
    )?;
    let mut counted = 0;
    for row in stmt.query_map([pack_id], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))? {
        let (status, n) = row?;
        counted += n;
        match status.as_str() {
            "needs_editor" => s.needs_editor = n,
            "unsupported" => s.unsupported = n,
            _ => s.failed += n,
        }
    }
    s.pending = s.packages - counted;
    let mut stmt = conn.prepare(
        "SELECT COALESCE(u.class, '?') || ': ' || u.reason, COUNT(*) AS n FROM unreal_status u
         JOIN assets a ON a.id = u.asset_id WHERE a.pack_id = ?1 GROUP BY 1 ORDER BY n DESC LIMIT 12",
    )?;
    s.reasons = stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
    Ok(s)
}

/// Work for `hoard unreal-worker`: packages that need Unreal Editor, in pack order.
pub(crate) fn pending(conn: &Connection, limit: u32) -> Result<Vec<PendingPackage>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.pack_id, a.path, u.class, u.reason FROM unreal_status u JOIN assets a ON a.id = u.asset_id
         WHERE u.status = 'needs_editor' ORDER BY a.pack_id, a.path LIMIT ?1",
    )?;
    let rows = stmt
        .query_map([limit], |r| {
            Ok(PendingPackage {
                asset_id: r.get(0)?,
                pack_id: r.get(1)?,
                path: r.get(2)?,
                class: r.get(3)?,
                reason: r.get(4)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(rows)
}

/// What a converted asset came from, for the asset detail view.
pub(crate) fn origin_of(conn: &Connection, asset_id: &str) -> Result<Option<AssetOrigin>> {
    Ok(conn
        .query_row(
            &format!(
                "SELECT o.path, o.ext, o.blob_hash IS NOT NULL OR ({}), o.class, o.engine, o.dependencies, o.method,
                        o.notes, o.converted_at
                   FROM asset_origin o JOIN assets a ON a.id = o.asset_id WHERE o.asset_id = ?1",
                crate::reformat::REBUILDS
            ),
            [asset_id],
            |r| {
                let deps: String = r.get(5)?;
                Ok(AssetOrigin {
                    path: r.get(0)?,
                    ext: r.get(1)?,
                    original_kept: r.get(2)?,
                    class: r.get(3)?,
                    engine: r.get(4)?,
                    dependencies: serde_json::from_str(&deps).unwrap_or_default(),
                    method: r.get(6)?,
                    notes: r.get::<_, Option<String>>(7)?.filter(|n| !n.is_empty()),
                    converted_at: r.get(8)?,
                    kept_until: None,
                })
            },
        )
        .optional()?)
}

/// Unreal conversion status of one asset still in package form.
pub(crate) fn status_of(conn: &Connection, asset_id: &str) -> Result<Option<UnrealStatus>> {
    Ok(conn
        .query_row("SELECT status, class, reason FROM unreal_status WHERE asset_id = ?1", [asset_id], |r| {
            Ok(UnrealStatus { status: r.get(0)?, class: r.get(1)?, reason: r.get(2)? })
        })
        .optional()?)
}

/// Package facts carried onto a converted asset when the pack is retagged.
#[derive(Debug, Clone, Default)]
pub(crate) struct OriginFacts {
    pub(crate) path: String,
    pub(crate) class: Option<String>,
    pub(crate) engine: Option<String>,
    pub(crate) dependencies: Vec<String>,
}

/// Origins for a pack, keyed by asset id.
pub(crate) fn pack_origins(conn: &Connection, pack_id: &str) -> Result<HashMap<String, OriginFacts>> {
    let mut stmt = conn.prepare(
        "SELECT o.asset_id, o.path, o.class, o.engine, o.dependencies FROM asset_origin o
         JOIN assets a ON a.id = o.asset_id WHERE a.pack_id = ?1",
    )?;
    let rows = stmt
        .query_map([pack_id], |r| {
            let deps: String = r.get(4)?;
            let facts = OriginFacts {
                path: r.get(1)?,
                class: r.get(2)?,
                engine: r.get(3)?,
                dependencies: serde_json::from_str(&deps).unwrap_or_default(),
            };
            Ok((r.get::<_, String>(0)?, facts))
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Row = (Option<String>, Option<String>, String, String);

    fn origin_row(c: &Connection) -> Row {
        c.query_row("SELECT blob_hash, orig_hash, converter, path FROM asset_origin WHERE asset_id = 't'", [], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .unwrap()
    }

    /// What a converted file came from, and what made it, outlive the original and a second conversion.
    #[test]
    fn where_a_file_came_from_outlives_the_original() {
        let mut conn = db::test_library();
        conn.execute_batch(
            "INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'Props', 'ready', 'x');
             INSERT INTO blobs (hash, size, created_at) VALUES ('pkg', 1, 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('t', 'p', 'pkg', 'T_Jar.uasset', 'T_Jar', 'uasset', 'texture', '{}', 'x', 'x');",
        )
        .unwrap();
        let made = native_converter("PNG source, red and blue put right");
        let origin =
            Origin { class: Some("Texture2D".into()), method: "native", converter: made.clone(), ..Origin::default() };
        let tx = conn.transaction().unwrap();
        assert!(replace_asset(&tx, "t", &StoredBlob { hash: "png".into(), size: 2 }, "png", &origin, false).unwrap());
        tx.commit().unwrap();
        let (kept, orig, by, path) = origin_row(&conn);
        assert_eq!(kept, None, "the package is discarded");
        assert_eq!(orig.as_deref(), Some("pkg"), "but what it was is remembered");
        assert_eq!(by, format!("hoard-unreal r{}: PNG source, red and blue put right", convert::REVISION));
        assert_eq!(path, "T_Jar.uasset");

        // Converted again: the package is still what it came from, and both steps are told.
        let tx = conn.transaction().unwrap();
        let again = Origin { method: "rule", ..Origin::default() };
        replace_asset(&tx, "t", &StoredBlob { hash: "jpg".into(), size: 3 }, "jpg", &again, true).unwrap();
        tx.commit().unwrap();
        let (kept, orig, by, path) = origin_row(&conn);
        assert_eq!(kept, None);
        assert_eq!(orig.as_deref(), Some("pkg"));
        assert_eq!(by, format!("{made}; then hoard {} rule", env!("CARGO_PKG_VERSION")));
        assert_eq!(path, "T_Jar.uasset");
    }
}
