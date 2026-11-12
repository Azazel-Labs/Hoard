//! Watched folders: each folder inside a watched location is a pack, indexed where it sits. A scan adds a pack for
//! each new folder, drops the packs whose folder is gone, and catches up packs whose files changed. Every watched
//! location is scanned at startup and every [`RESCAN_EVERY`] after, and from Settings on demand.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use rusqlite::{Connection, params};
use serde_json::json;
use tokio::runtime::Handle;

use crate::AppState;
use crate::locations::{self, Mode};
use crate::{db, ingest, jobs};

/// How often every watched folder is looked at again. Folder-change notifications don't reach a container
/// through most network shares, so this is a plain timer.
const RESCAN_EVERY: Duration = Duration::from_mins(15);

/// Queue a scan of a watched location, unless one is already waiting.
pub(crate) fn enqueue(c: &Connection, state: &AppState, location: i64) -> Result<()> {
    let waiting: bool = c.query_row(
        "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued'
                          AND json_extract(payload, '$.location') = ?2)",
        params![jobs::SCAN, location],
        |r| r.get(0),
    )?;
    if !waiting {
        db::enqueue(c, jobs::SCAN, None, &json!({ "location": location }))?;
        state.wake.notify_one();
    }
    Ok(())
}

/// Scan every watched location now, and again every [`RESCAN_EVERY`].
pub(crate) fn spawn_rescans(state: &AppState) {
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(RESCAN_EVERY);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tick.tick().await;
            let s = std::sync::Arc::clone(&state);
            let res = db::call(&state.pool, move |c| {
                let ids: Vec<i64> = {
                    let mut stmt = c.prepare("SELECT id FROM locations WHERE mode = 'watched'")?;
                    stmt.query_map([], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
                };
                for id in ids {
                    enqueue(c, &s, id)?;
                }
                Ok(())
            })
            .await;
            if let Err(e) = res {
                tracing::warn!(error = %format!("{e:#}"), "couldn't queue the watched-folder scans");
            }
        }
    });
}

/// Folders that are never packs: hidden ones, and what operating systems and NASes leave lying around.
fn is_litter(name: &str) -> bool {
    name.starts_with(['.', '@', '#', '$'])
        || matches!(name, "System Volume Information" | "__MACOSX" | "lost+found" | "Thumbs.db")
}

/// The pack folders directly inside a watched location.
fn pack_folders(root: &Path) -> Result<HashSet<String>> {
    let mut out = HashSet::new();
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !is_litter(&name) && entry.path().is_dir() {
            out.insert(name);
        }
    }
    Ok(out)
}

/// What a scan changed.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Changes {
    pub(crate) added: usize,
    pub(crate) removed: usize,
    pub(crate) updated: usize,
}

pub(crate) fn scan(state: &AppState, rt: &Handle, location: i64) -> Result<Changes> {
    let mut conn = state.pool.get()?;
    let Some(loc) = locations::get(&conn, location)? else { return Ok(Changes::default()) };
    if loc.mode != Mode::Watched {
        return Ok(Changes::default());
    }
    let root = Path::new(&loc.path);
    let folders = match pack_folders(root) {
        Ok(f) => f,
        // Offline (a disk that isn't mounted, a NAS that's asleep): its packs stay as they are until it's back.
        Err(e) => {
            tracing::warn!(location = %loc.name, path = %loc.path, "watched folder can't be read: {e}");
            return Ok(Changes::default());
        }
    };
    let existing: HashMap<String, (String, String)> = {
        let mut stmt =
            conn.prepare("SELECT folder, id, status FROM packs WHERE location_id = ?1 AND folder IS NOT NULL")?;
        stmt.query_map([location], |r| Ok((r.get(0)?, (r.get(1)?, r.get(2)?))))?.collect::<rusqlite::Result<_>>()?
    };
    // An empty folder where there were packs is far more often a share that didn't mount than a library someone
    // emptied: losing every pack's tags and notes to that would be the worst outcome, so leave them.
    if folders.is_empty() && !existing.is_empty() {
        tracing::warn!(location = %loc.name, path = %loc.path, "watched folder is empty; keeping its packs");
        return Ok(Changes::default());
    }

    let mut changes = Changes::default();
    for (folder, (pack_id, _)) in &existing {
        if !folders.contains(folder) {
            db::delete_pack(&mut conn, pack_id)?;
            tracing::info!(location = %loc.name, folder = %folder, "watched pack folder is gone; pack removed");
            changes.removed += 1;
        }
    }
    let mut new: Vec<&String> = folders.iter().filter(|f| !existing.contains_key(*f)).collect();
    new.sort();
    for folder in new {
        add_pack(&conn, location, folder)?;
        changes.added += 1;
    }
    for (folder, (pack_id, status)) in &existing {
        if status == "ready" && folders.contains(folder) && catch_up(state, rt, &conn, pack_id, &root.join(folder))? {
            changes.updated += 1;
        }
    }
    conn.execute("UPDATE locations SET scanned_at = ?2 WHERE id = ?1", params![location, db::now()])?;
    if changes != Changes::default() {
        tracing::info!(location = %loc.name, added = changes.added, removed = changes.removed, updated = changes.updated, "watched folder scanned");
        state.wake.notify_one();
    }
    Ok(changes)
}

/// A pack for a new folder, queued for ingest (which reads the folder in place).
fn add_pack(c: &Connection, location: i64, folder: &str) -> Result<()> {
    let id = uuid::Uuid::now_v7().to_string();
    c.execute(
        "INSERT INTO packs (id, name, status, source, files_total, created_at, location_id, folder)
         VALUES (?1, ?2, 'ingesting', 'watched folder', 0, ?3, ?4, ?2)",
        params![id, folder, db::now(), location],
    )?;
    db::enqueue(c, jobs::INGEST, Some(&id), &json!({ "auto_name": false }))?;
    Ok(())
}

/// Bring a ready pack in line with its folder: assets whose file is gone are dropped, and new files are queued for
/// ingest. Returns whether anything changed.
fn catch_up(state: &AppState, rt: &Handle, c: &Connection, pack_id: &str, dir: &Path) -> Result<bool> {
    let busy: bool = c.query_row(
        "SELECT EXISTS (SELECT 1 FROM jobs WHERE pack_id = ?1 AND status IN ('queued', 'running'))",
        [pack_id],
        |r| r.get(0),
    )?;
    if busy {
        return Ok(false);
    }
    let Ok(on_disk) = ingest::pack_rels(dir) else { return Ok(false) };
    let known: Vec<(String, String, String)> = {
        let mut stmt = c.prepare("SELECT id, path, blob_hash FROM assets WHERE pack_id = ?1")?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    // Assets made from the pack's files (converted Unreal packages, say) have no file of their own in the folder;
    // they're kept in Hoard's own storage, which is how they're told apart from files that were deleted.
    let mut gone = Vec::new();
    for (id, path, hash) in &known {
        if !on_disk.contains(path) && !rt.block_on(state.blobs.stored(hash))? {
            gone.push(id.as_str());
        }
    }
    let paths: HashSet<&str> = known.iter().map(|(_, p, _)| p.as_str()).collect();
    let added = on_disk.iter().any(|p| !paths.contains(p.as_str()));
    for id in &gone {
        c.execute("DELETE FROM assets_fts WHERE asset_id = ?1", [id])?;
        c.execute("DELETE FROM assets WHERE id = ?1", [id])?;
    }
    if added {
        db::enqueue(c, jobs::INGEST, Some(pack_id), &json!({ "auto_name": false }))?;
    }
    Ok(added || !gone.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn litter_is_not_a_pack() {
        for name in [".git", "@eaDir", "#recycle", "$RECYCLE.BIN", "System Volume Information", "__MACOSX"] {
            assert!(is_litter(name), "{name}");
        }
        assert!(!is_litter("Forest Kit"));
    }

    #[test]
    fn pack_folders_are_the_folders_directly_inside() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("Forest Kit/Textures")).unwrap();
        std::fs::create_dir_all(dir.path().join("Sci-Fi Sounds")).unwrap();
        std::fs::create_dir_all(dir.path().join(".trash")).unwrap();
        std::fs::write(dir.path().join("readme.txt"), "loose files aren't packs").unwrap();
        let mut got: Vec<_> = pack_folders(dir.path()).unwrap().into_iter().collect();
        got.sort();
        assert_eq!(got, ["Forest Kit", "Sci-Fi Sounds"]);
    }
}
