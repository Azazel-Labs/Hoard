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
    let req =
        Request { discard_originals: mode == UnrealConvert::Replace, drop_unsupported: false, then_llm: Some(true) };
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
    // Converting twice keeps the first origin (the package, not an intermediate file).
    let prior: Option<(String, String, Option<String>)> = tx
        .query_row("SELECT path, ext, blob_hash FROM asset_origin WHERE asset_id = ?1", [asset_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .optional()?;
    let (orig_path, orig_ext, orig_hash) = prior.unwrap_or_else(|| (path.clone(), ext.clone(), Some(old_hash)));
    tx.execute(
        "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
        params![new.hash, new.size as i64, now],
    )?;
    let deps = json!(origin.dependencies).to_string();
    let notes = origin.notes.join("; ");
    let kept = orig_hash.filter(|_| keep_original);
    let record = |target_id: &str| -> rusqlite::Result<usize> {
        tx.execute(
            "INSERT INTO asset_origin (asset_id, path, ext, blob_hash, class, engine, dependencies, method, notes,
                                       converted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT (asset_id) DO UPDATE SET blob_hash = CASE WHEN ?4 IS NULL THEN NULL ELSE blob_hash END,
                 method = excluded.method, notes = excluded.notes, converted_at = excluded.converted_at",
            params![target_id, orig_path, orig_ext, kept, origin.class, origin.engine, deps, origin.method, notes, now],
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

pub(crate) async fn convert_pack(state: &AppState, pack_id: &str, req: Request) -> Result<Summary> {
    let pid = pack_id.to_owned();
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
        state.blobs.download_to(hash, &local).await?;
        let l = local.clone();
        let (outcome, mut origin) = crate::heavy::run("unreal", move || convert_file(&l)).await?;
        tokio::fs::remove_file(&local).await.ok();
        let class = origin.class.clone();
        let id2 = id.clone();
        match outcome {
            Outcome::Converted { file, notes } => {
                let blob = state.blobs.put_bytes(file.bytes, AssetKind::from_ext(file.ext)).await?;
                origin.notes = notes;
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
                db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM assets WHERE id = ?1", [&id2])?)).await?;
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
    after_changes(state, pack_id, sum.converted + sum.merged + sum.dropped > 0, req.then_llm).await?;
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
            "SELECT path, ext, blob_hash IS NOT NULL, class, engine, dependencies, method, notes, converted_at
             FROM asset_origin WHERE asset_id = ?1",
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
