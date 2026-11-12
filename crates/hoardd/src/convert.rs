//! Optional conversion of authoring formats to GLB with headless Blender.
//!
//! Rust can't read .blend at all, and FBX/USD/DAE support is patchy, while Blender imports all of them and
//! has an excellent glTF exporter. So for those formats Hoard materializes the asset plus its linked files
//! (so textures resolve) in a temp folder, runs `blender -b` with a tiny export script, and stores the GLB
//! as a *derivative* of the original. The derivative feeds the thumbnail renderer and the browser preview,
//! and is downloadable for engines that want glTF. The original is never touched.

use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, params};

use crate::{AppState, analyze, db, jobs};

/// Extensions worth sending through Blender.
pub(crate) const CONVERTIBLE: &[&str] =
    &["blend", "fbx", "dae", "3ds", "usd", "usda", "usdc", "usdz", "ply", "stl", "abc", "x3d"];
const TIMEOUT: Duration = Duration::from_secs(180);

/// Blender-side script: import by extension, export everything as one GLB.
const EXPORT_SCRIPT: &str = r#"
import sys, bpy
src, dst = sys.argv[sys.argv.index("--") + 1:][:2]
ext = src.rsplit(".", 1)[-1].lower()
if ext == "blend":
    bpy.ops.wm.open_mainfile(filepath=src)
else:
    bpy.ops.wm.read_factory_settings(use_empty=True)
    importers = {
        "fbx": lambda: bpy.ops.import_scene.fbx(filepath=src),
        "dae": lambda: bpy.ops.wm.collada_import(filepath=src),
        "3ds": lambda: bpy.ops.import_scene.max3ds(filepath=src),
        "ply": lambda: bpy.ops.wm.ply_import(filepath=src),
        "stl": lambda: bpy.ops.wm.stl_import(filepath=src),
        "abc": lambda: bpy.ops.wm.alembic_import(filepath=src),
        "x3d": lambda: bpy.ops.import_scene.x3d(filepath=src),
    }
    importers.get(ext, lambda: bpy.ops.wm.usd_import(filepath=src))()
bpy.ops.export_scene.gltf(filepath=dst, export_format="GLB", export_animations=True, export_apply=True)
"#;

/// Find or fetch Blender for the current setting. When it becomes ready because the setting was just changed
/// (`changed`) or because it was just downloaded, packs that were ingested without it get their conversions queued.
/// A plain restart doesn't: those packs were either converted already or failed for reasons a rerun won't fix.
pub(crate) fn start_blender(state: &AppState, changed: bool) {
    let st = std::sync::Arc::clone(state);
    let setting = state.settings.get().blender.clone();
    crate::blender::start(&setting, state.cfg.tools_dir(), move |downloaded| {
        if !(changed || downloaded) {
            return;
        }
        tokio::spawn(async move {
            match db::call(&st.pool, |c| backfill(c)).await {
                Ok(0) => {}
                Ok(n) => {
                    tracing::info!(packs = n, "queued Blender conversions for packs that waited for it");
                    st.wake.notify_one();
                }
                Err(e) => tracing::warn!(error = %format!("{e:#}"), "could not queue Blender conversions"),
            }
        });
    });
}

/// Queue a conversion for every ready pack with a convertible file that has no GLB yet and no conversion pending.
fn backfill(conn: &Connection) -> Result<usize> {
    let placeholders = vec!["?"; CONVERTIBLE.len()].join(",");
    let args: Vec<&dyn rusqlite::ToSql> = CONVERTIBLE.iter().map(|e| e as &dyn rusqlite::ToSql).collect();
    let packs: Vec<String> = conn
        .prepare(&format!(
            "SELECT DISTINCT a.pack_id FROM assets a JOIN packs p ON p.id = a.pack_id
              WHERE p.status = 'ready' AND a.ext IN ({placeholders})
                AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'glb')
                AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = a.pack_id AND j.kind = '{kind}'
                                  AND j.status IN ('queued', 'running'))",
            kind = jobs::CONVERT,
        ))?
        .query_map(args.as_slice(), |r| r.get(0))?
        .collect::<rusqlite::Result<_>>()?;
    for p in &packs {
        db::enqueue(conn, jobs::CONVERT, Some(p), &serde_json::json!({}))?;
    }
    Ok(packs.len())
}

/// After ingest: queue a conversion job if Blender is ready and the pack has convertible files.
pub(crate) fn maybe_enqueue(state: &AppState, conn: &Connection, pack_id: &str) -> Result<()> {
    if crate::blender::tool().is_none() {
        return Ok(());
    }
    let placeholders = vec!["?"; CONVERTIBLE.len()].join(",");
    let mut args: Vec<&dyn rusqlite::ToSql> = vec![&pack_id];
    args.extend(CONVERTIBLE.iter().map(|e| e as &dyn rusqlite::ToSql));
    let n: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM assets WHERE pack_id = ? AND ext IN ({placeholders})"),
        args.as_slice(),
        |r| r.get(0),
    )?;
    if n > 0 {
        db::enqueue(conn, jobs::CONVERT, Some(pack_id), &serde_json::json!({}))?;
        state.wake.notify_one();
    }
    Ok(())
}

struct Todo {
    id: String,
    path: String,
    has_thumb: bool,
    deps: Vec<db::BundleFile>,
}

pub(crate) async fn convert_pack(state: &AppState, pack_id: &str) -> Result<()> {
    let blender = crate::blender::tool().context("Blender isn't available; turn it on in Settings")?.path;
    let pid = pack_id.to_owned();
    let todo: Vec<Todo> = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare(
            "SELECT id, path, ext, thumb_hash IS NOT NULL FROM assets a WHERE pack_id = ?1
               AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'glb')",
        )?;
        let rows: Vec<(String, String, String, bool)> = stmt
            .query_map([&pid], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut todo = Vec::new();
        for (id, path, ext, has_thumb) in rows {
            if CONVERTIBLE.contains(&ext.as_str()) {
                let ids = db::dependency_closure(c, &id, false)?;
                todo.push(Todo { deps: db::bundle_files(c, &ids)?, id, path, has_thumb });
            }
        }
        Ok(todo)
    })
    .await?;

    let mut failures = 0usize;
    for (n, t) in todo.iter().enumerate() {
        crate::activity::job_progress(jobs::CONVERT, Some(pack_id), n, todo.len(), "models");
        if let Err(e) = convert_one(state, &blender, t).await {
            failures += 1;
            tracing::warn!(asset = %t.path, error = %format!("{e:#}"), "Blender conversion failed");
        }
    }
    tracing::info!(pack = pack_id, converted = todo.len() - failures, failures, "conversion finished");
    Ok(())
}

async fn convert_one(state: &AppState, blender: &Path, t: &Todo) -> Result<()> {
    let work = state.cfg.tmp_dir().join(format!("convert-{}", t.id));
    tokio::fs::create_dir_all(&work).await?;
    let result = async {
        // Recreate the files' original layout so relative texture paths resolve inside Blender.
        for f in &t.deps {
            let dest = work.join(&f.path);
            if let Some(parent) = dest.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            state.blobs.download_to(&f.hash, &dest).await?;
        }
        let src = work.join(&t.path);
        let out = work.join("__hoard_out.glb");
        let mut cmd = crate::heavy::command(blender);
        cmd.arg("-b")
            .arg("--factory-startup")
            .arg("--python-exit-code")
            .arg("1")
            .arg("--python-expr")
            .arg(EXPORT_SCRIPT)
            .arg("--")
            .arg(&src)
            .arg(&out)
            .kill_on_drop(true);
        let output = tokio::time::timeout(TIMEOUT, cmd.output()).await.context("Blender timed out")??;
        if !output.status.success() || !out.is_file() {
            let tail: String =
                String::from_utf8_lossy(&output.stderr).lines().rev().take(8).collect::<Vec<_>>().join(" | ");
            bail!("Blender exited with {}: {tail}", output.status);
        }
        let glb = tokio::fs::read(&out).await?;

        // Thumbnail from the converted model, if the original had none.
        let thumb = if t.has_thumb {
            None
        } else {
            let bytes = glb.clone();
            crate::heavy::run("thumbnail", move || -> Option<Vec<u8>> {
                let r = analyze::renderer()?;
                let img = r.lock().unwrap_or_else(std::sync::PoisonError::into_inner).render_slice(&bytes, 256).ok()?;
                analyze::thumb_bytes(img, false).ok()
            })
            .await?
        };
        let glb_blob = state.blobs.put_bytes(glb, hoard_core::AssetKind::Model).await?;
        let thumb_blob = match thumb {
            Some(bytes) => Some(state.blobs.put_cache_bytes(bytes).await?),
            None => None,
        };
        let id = t.id.clone();
        db::call(&state.pool, move |c| {
            let now = db::now();
            let tx = c.transaction()?;
            for b in std::iter::once(&glb_blob).chain(thumb_blob.as_ref()) {
                tx.execute(
                    "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
                    params![b.hash, b.size as i64, now],
                )?;
            }
            tx.execute(
                "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, 'glb', ?2, ?3)
                 ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash",
                params![id, glb_blob.hash, now],
            )?;
            if let Some(tb) = &thumb_blob {
                tx.execute(
                    "UPDATE assets SET thumb_hash = ?2 WHERE id = ?1 AND thumb_hash IS NULL",
                    params![id, tb.hash],
                )?;
            }
            tx.commit()?;
            Ok(())
        })
        .await
    }
    .await;
    tokio::fs::remove_dir_all(&work).await.ok();
    result
}
