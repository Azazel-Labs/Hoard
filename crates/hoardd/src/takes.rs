//! A file of many takes (an FBX of a character's every clip: `Idle`, `Walk`, `Run`…) has each take as an animation of
//! its own, so searching "run" finds the run, named for it, drawn mid-run, and played, fitted and downloaded alone.
//! Each is a GLB of the file's nodes and that one clip ([`crate::meshes::take_glbs`]), kept beside the file as
//! `<file>@<take>.glb`, the way Unity names a character's clips, and marked with the file it came from (`take_of`). A
//! file of clips with no mesh is hidden behind its takes; a character stays as it is, with its clips.

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use hoard_core::{AssetKind, TagSource};
use rusqlite::{OptionalExtension, params};
use serde_json::json;

use crate::animations::{POSE_SECS, Rig};
use crate::{AppState, db};

/// An asset that is one take of a file of many.
pub(crate) const IS_TAKE: &str = "(json_extract(a.meta, '$.take_of') IS NOT NULL)";

/// A take's path: beside its file, named for the file and the take (`Clips/hero@Run.glb`).
fn take_path(file: &str, take: &str) -> String {
    let (dir, name) = file.rsplit_once('/').map_or(("", file), |(d, n)| (d, n));
    let stem = name.rsplit_once('.').map_or(name, |(s, _)| s);
    let take: String =
        take.chars().map(|c| if "/\\:*?\"<>|".contains(c) || c.is_control() { '_' } else { c }).collect();
    let file = format!("{stem}@{}.glb", take.trim());
    if dir.is_empty() { file } else { format!("{dir}/{file}") }
}

/// A file's takes, named as they'll be (a name twice gets a number): none for a file with fewer than two.
fn takes(rig: &Rig) -> Vec<String> {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let names: Vec<String> = rig
        .clips
        .iter()
        .filter(|c| c.secs >= POSE_SECS)
        .map(|c| {
            let n = seen.entry(c.name.to_lowercase()).or_default();
            *n += 1;
            if *n == 1 { c.name.clone() } else { format!("{} {n}", c.name) }
        })
        .collect();
    if names.len() >= 2 { names } else { Vec::new() }
}

/// A file with takes, as found in the pack.
struct File {
    id: String,
    path: String,
    blob: String,
    thumb: Option<String>,
    takes: Vec<String>,
}

/// A take already made: its id, blob, the file and the file's contents it was made from.
struct Made {
    id: String,
    blob: String,
    of: String,
    from: String,
}

/// A take to write: the file it's of and that file's contents, its name and path, the file's picture to show until
/// it's drawn, its rig and its GLB.
struct Take {
    of: String,
    from: String,
    name: String,
    path: String,
    thumb: Option<String>,
    rig: Option<Rig>,
    blob: crate::blobs::StoredBlob,
}

/// A pack's files with takes, the takes made already (by path), and the paths of its other assets.
type Found = (Vec<File>, HashMap<String, Made>, HashSet<String>);

fn find(c: &rusqlite::Connection, pack_id: &str) -> Result<Found> {
    // Another format's copy of a file (the FBX beside its GLB) would only make the same takes again.
    let mut stmt = c.prepare(&format!(
        "SELECT a.id, a.path, a.blob_hash, a.thumb_hash, r.rig FROM assets a JOIN rigs r ON r.asset_id = a.id
          WHERE a.pack_id = ?1 AND a.kind IN ('model', 'animation') AND NOT {IS_TAKE}
            AND NOT EXISTS (SELECT 1 FROM asset_links l WHERE l.to_id = a.id AND l.rel = 'format')
          ORDER BY a.path"
    ))?;
    let rows = stmt.query_map([pack_id], |r| {
        Ok((r.get::<_, String>(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get::<_, String>(4)?))
    })?;
    let mut files = Vec::new();
    for row in rows {
        let (id, path, blob, thumb, rig) = row?;
        let Ok(rig) = serde_json::from_str::<Rig>(&rig) else { continue };
        let takes = takes(&rig);
        if !takes.is_empty() {
            files.push(File { id, path, blob, thumb, takes });
        }
    }
    let made = c
        .prepare(&format!(
            "SELECT a.path, a.id, a.blob_hash, json_extract(a.meta, '$.take_of'),
                    COALESCE(json_extract(a.meta, '$.take_from'), '')
               FROM assets a WHERE a.pack_id = ?1 AND {IS_TAKE}"
        ))?
        .query_map([pack_id], |r| {
            Ok((r.get(0)?, Made { id: r.get(1)?, blob: r.get(2)?, of: r.get(3)?, from: r.get(4)? }))
        })?
        .collect::<rusqlite::Result<_>>()?;
    let paths = c
        .prepare(&format!("SELECT a.path FROM assets a WHERE a.pack_id = ?1 AND NOT {IS_TAKE}"))?
        .query_map([pack_id], |r| r.get(0))?
        .collect::<rusqlite::Result<_>>()?;
    Ok((files, made, paths))
}

/// Make a pack's files of many takes into an animation each, again for a file that changed, and drop the takes of
/// files gone or down to one clip. Returns whether anything changed.
pub(crate) async fn make(state: &AppState, pack_id: &str) -> Result<bool> {
    let pid = pack_id.to_owned();
    let (files, made, paths) = db::call(&state.pool, move |c| find(c, &pid)).await?;
    if files.is_empty() && made.is_empty() {
        return Ok(false);
    }

    // The takes to keep as they are, and those to make (again).
    let mut keep: HashSet<String> = HashSet::new();
    let mut fresh: Vec<Take> = Vec::new();
    for f in &files {
        let wanted: Vec<String> =
            f.takes.iter().map(|t| take_path(&f.path, t)).filter(|p| !paths.contains(p)).collect();
        if wanted.iter().all(|p| made.get(p).is_some_and(|m| m.of == f.id && m.from == f.blob)) {
            keep.extend(wanted);
            continue;
        }
        let work = state.cfg.tmp_dir().join(format!("takes-{}", uuid::Uuid::now_v7()));
        let glbs = async {
            let local = crate::motion::fetch(state, &f.id, &work).await?;
            crate::heavy::run("takes", move || crate::meshes::take_glbs(&local.path, &local.ext, &local.root)).await?
        }
        .await;
        tokio::fs::remove_dir_all(&work).await.ok();
        let glbs = match glbs {
            Ok(g) => g,
            Err(e) => {
                // Its takes from before stay until it can be read.
                tracing::info!(path = %f.path, error = %format!("{e:#}"), "couldn't make a file's takes");
                keep.extend(made.iter().filter(|(_, m)| m.of == f.id).map(|(p, _)| p.clone()));
                continue;
            }
        };
        // Named as the rig names them, which numbers a name that comes twice.
        for (name, (_, glb)) in f.takes.iter().zip(glbs) {
            let path = take_path(&f.path, name);
            if paths.contains(&path) {
                continue;
            }
            let rig = crate::animations::rig_of_glb(&glb);
            let blob = state.blobs.put_bytes(glb, AssetKind::Animation).await?;
            keep.insert(path.clone());
            let (of, from, name, thumb) = (f.id.clone(), f.blob.clone(), name.clone(), f.thumb.clone());
            fresh.push(Take { of, from, name, path, thumb, rig, blob });
        }
    }
    let gone: Vec<(String, String)> =
        made.iter().filter(|(p, _)| !keep.contains(*p)).map(|(_, m)| (m.id.clone(), m.blob.clone())).collect();
    if fresh.is_empty() && gone.is_empty() {
        return Ok(false);
    }
    let pid = pack_id.to_owned();
    let replaced =
        db::call(&state.pool, move |c| write(c, &pid, &fresh, &gone)).await.context("writing a pack's takes")?;
    crate::jobs::delete_unused(state, replaced).await?;
    Ok(true)
}

/// Write a pack's takes made (again), and delete those gone, returning the blobs no longer theirs.
fn write(
    c: &mut rusqlite::Connection,
    pack_id: &str,
    fresh: &[Take],
    gone: &[(String, String)],
) -> Result<Vec<(String, i64)>> {
    let now = db::now();
    let tx = c.transaction()?;
    let mut replaced: Vec<(String, i64)> = Vec::new();
    let size_of = |tx: &rusqlite::Transaction<'_>, hash: &str| -> rusqlite::Result<i64> {
        tx.query_row("SELECT COALESCE((SELECT size FROM blobs WHERE hash = ?1), 0)", [hash], |r| r.get(0))
    };
    for t in fresh {
        tx.execute(
            "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
            params![t.blob.hash, i64::try_from(t.blob.size).unwrap_or(i64::MAX), now],
        )?;
        let old: Option<(String, String)> = tx
            .query_row(
                "SELECT id, blob_hash FROM assets WHERE pack_id = ?1 AND path = ?2",
                params![pack_id, t.path],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let id = old.as_ref().map_or_else(|| uuid::Uuid::now_v7().to_string(), |(id, _)| id.clone());
        let meta = json!({ "take_of": t.of, "take": t.name, "take_from": t.from });
        tx.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, category, meta, words, thumb_hash,
                                 created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 'glb', 'animation', 'animations', ?6, ?7, ?8, ?9, ?9)
             ON CONFLICT (pack_id, path) DO UPDATE SET
                 blob_hash = excluded.blob_hash, meta = excluded.meta, words = excluded.words,
                 updated_at = excluded.updated_at,
                 name = CASE WHEN human_edited THEN name ELSE excluded.name END",
            params![
                id,
                pack_id,
                t.blob.hash,
                t.path,
                crate::tagger::display_name(&t.name),
                meta.to_string(),
                crate::tagger::fts_words(&t.path),
                t.thumb,
                now
            ],
        )?;
        if let Some((_, was)) = old.filter(|(_, was)| *was != t.blob.hash) {
            // Made again from changed contents: what was drawn of the old take goes, its file's picture standing in
            // until it's drawn again.
            let drawn: Vec<(String, i64)> = tx
                .prepare(
                    "SELECT d.blob_hash, b.size FROM derivatives d JOIN blobs b ON b.hash = d.blob_hash
                      WHERE d.asset_id = ?1",
                )?
                .query_map([&id], |r| Ok((r.get(0)?, r.get(1)?)))?
                .collect::<rusqlite::Result<_>>()?;
            replaced.extend(drawn);
            tx.execute("DELETE FROM derivatives WHERE asset_id = ?1", [&id])?;
            tx.execute(
                "UPDATE assets SET thumb_hash = ?2 WHERE id = ?1 AND thumb_source IS NULL",
                params![id, t.thumb],
            )?;
            replaced.push((was.clone(), size_of(&tx, &was)?));
        }
        // Its tags are its file's, but for the file's kind.
        tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND source <> 'human'", [&id])?;
        tx.execute(
            "INSERT INTO asset_tags (asset_id, tag, source, confidence)
             SELECT ?1, tag, source, confidence FROM asset_tags WHERE asset_id = ?2 AND tag NOT LIKE 'kind:%'
             ON CONFLICT DO NOTHING",
            params![id, t.of],
        )?;
        db::upsert_tag(&tx, &id, "kind:animation", TagSource::Meta, 1.0)?;
        db::reindex_asset(&tx, &id)?;
        crate::animations::store(&tx, &id, t.rig.as_ref())?;
    }
    for (id, blob) in gone {
        tx.execute("DELETE FROM assets WHERE id = ?1", [id])?;
        replaced.push((blob.clone(), size_of(&tx, blob)?));
    }
    tx.commit()?;
    Ok(replaced)
}

/// [`make`], with a failure logged rather than failing what called it.
pub(crate) async fn make_logged(state: &AppState, pack_id: &str) {
    if let Err(e) = make(state, pack_id).await {
        tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "couldn't make the pack's takes");
    }
}

/// Make the takes of every pack that has files of many takes and none made yet: the library as it was before takes
/// were made, once, in the background.
pub(crate) async fn backfill(state: &AppState) {
    let packs = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(&format!(
            "SELECT DISTINCT a.pack_id FROM assets a JOIN rigs r ON r.asset_id = a.id
              WHERE a.kind IN ('model', 'animation') AND NOT {IS_TAKE}
                AND (SELECT COUNT(*) FROM json_each(r.rig, '$.clips') c WHERE json_extract(c.value, '$.secs') >= ?1) >= 2
                AND NOT EXISTS (SELECT 1 FROM assets t WHERE json_extract(t.meta, '$.take_of') = +a.id)
                AND a.pack_id IN (SELECT id FROM packs WHERE status = 'ready')"
        ))?;
        Ok(stmt.query_map([POSE_SECS], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?)
    })
    .await;
    let packs = match packs {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(error = %format!("{e:#}"), "couldn't look for files of many takes");
            return;
        }
    };
    let mut any = false;
    for p in &packs {
        any |= make(state, p).await.unwrap_or_else(|e| {
            tracing::warn!(pack = %p, error = %format!("{e:#}"), "couldn't make the pack's takes");
            false
        });
    }
    if any {
        crate::motion::kick(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animations::Clip;

    #[test]
    fn takes_are_named_beside_their_file() {
        assert_eq!(take_path("Clips/hero.fbx", "Run"), "Clips/hero@Run.glb");
        assert_eq!(take_path("hero.fbx", "a/b:c"), "hero@a_b_c.glb");
        let clip = |name: &str, secs: f64| Clip { name: name.into(), secs, bones: Vec::new() };
        let rig = |clips: Vec<Clip>| Rig { clips, ..Rig::default() };
        // One take, or one and a pose, is the file itself.
        assert_eq!(takes(&rig(vec![clip("Run", 1.0)])), Vec::<String>::new());
        assert_eq!(takes(&rig(vec![clip("Run", 1.0), clip("TPose", 0.0)])), Vec::<String>::new());
        assert_eq!(
            takes(&rig(vec![clip("Idle", 2.0), clip("TPose", 0.0), clip("Run", 1.0), clip("run", 0.8)])),
            ["Idle", "Run", "run 2"]
        );
    }
}
