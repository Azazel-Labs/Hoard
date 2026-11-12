//! Watched folders: each folder inside a watched location is a pack, indexed where it sits. A scan adds a pack for
//! each new folder, drops the packs whose folder has been gone for [`FORGET_MISSING_AFTER`], and catches up packs
//! whose files changed. Every watched location is scanned at startup and every [`RESCAN_EVERY`] after, and from
//! Settings on demand.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use rusqlite::{Connection, params};
use serde_json::json;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::AppState;
use crate::locations::{self, Mode};
use crate::{db, ingest, jobs};

/// How often every watched folder is looked at again. Folder-change notifications don't reach a container
/// through most network shares, so this is a plain timer.
const RESCAN_EVERY: Duration = Duration::from_mins(15);

/// How long a pack whose folder has gone missing is kept, with its tags and notes, before it's dropped. A folder
/// that's missing is more often on a disk or share that's offline than deleted.
const FORGET_MISSING_AFTER: Duration = Duration::from_hours(72);

/// Queue a scan of a watched location, unless one is already waiting or running (two at once would both add the
/// same new folders).
pub(crate) fn enqueue(c: &Connection, state: &AppState, location: i64) -> Result<()> {
    if !db::job_pending(c, jobs::SCAN, None, Some(("location", location)))? {
        db::enqueue(c, jobs::SCAN, None, &json!({ "location": location }))?;
        state.wake.notify_one();
    }
    Ok(())
}

/// Scan every watched location now, and again every [`RESCAN_EVERY`]. Each time, managed locations that couldn't
/// be opened (a disk that wasn't mounted yet) are tried again too.
pub(crate) fn spawn_rescans(state: &AppState) {
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(RESCAN_EVERY);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tick.tick().await;
            if let Err(e) = state.blobs.refresh().await {
                tracing::warn!(error = %format!("{e:#}"), "couldn't reopen the library locations");
            }
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

pub(crate) fn scan(state: &AppState, location: i64) -> Result<Changes> {
    let changes = sweep(&mut *state.pool.get()?, location)?;
    if changes != Changes::default() {
        state.wake.notify_one();
    }
    Ok(changes)
}

fn sweep(conn: &mut Connection, location: i64) -> Result<Changes> {
    let Some(loc) = locations::get(conn, location)? else { return Ok(Changes::default()) };
    if loc.mode != Mode::Watched {
        return Ok(Changes::default());
    }
    let root = Path::new(&loc.path);
    let mut folders = match pack_folders(root) {
        Ok(f) => f,
        // Offline (a disk that isn't mounted, a NAS that's asleep): its packs stay as they are until it's back.
        Err(e) => {
            tracing::warn!(location = %loc.name, path = %loc.path, "watched folder can't be read: {e}");
            return Ok(Changes::default());
        }
    };
    let existing: HashMap<String, Existing> = {
        let mut stmt = conn.prepare(
            "SELECT folder, id, status, missing_since FROM packs WHERE location_id = ?1 AND folder IS NOT NULL",
        )?;
        stmt.query_map([location], |r| {
            Ok((r.get(0)?, Existing { id: r.get(1)?, status: r.get(2)?, missing_since: r.get(3)? }))
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    // An empty folder where there were packs is far more often a share that didn't mount than a library someone
    // emptied: leave its packs alone.
    if folders.is_empty() && !existing.is_empty() {
        tracing::warn!(location = %loc.name, path = %loc.path, "watched folder is empty; keeping its packs");
        return Ok(Changes::default());
    }
    // Folders whose pack was deleted for good stay out.
    let skipped: Vec<String> = {
        let mut stmt = conn.prepare("SELECT folder FROM skipped_folders WHERE location_id = ?1")?;
        stmt.query_map([location], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
    };
    for f in &skipped {
        folders.remove(f);
    }

    let mut changes = Changes::default();
    let now = OffsetDateTime::now_utc();
    for (folder, pack) in &existing {
        if folders.contains(folder) {
            if pack.missing_since.is_some() {
                conn.execute("UPDATE packs SET missing_since = NULL WHERE id = ?1", [&pack.id])?;
            }
            continue;
        }
        let since = pack.missing_since.as_deref().and_then(|t| OffsetDateTime::parse(t, &Rfc3339).ok());
        match since {
            Some(t) if (now - t).unsigned_abs() >= FORGET_MISSING_AFTER => {
                db::delete_pack(conn, &pack.id)?;
                tracing::info!(location = %loc.name, folder = %folder, "watched pack folder is gone; pack removed");
                changes.removed += 1;
            }
            Some(_) => {}
            None => {
                conn.execute("UPDATE packs SET missing_since = ?2 WHERE id = ?1", params![pack.id, db::now()])?;
                tracing::warn!(location = %loc.name, folder = %folder, "watched pack folder is missing; the pack is kept for now in case the folder is only offline");
            }
        }
    }
    let mut new: Vec<&String> = folders.iter().filter(|f| !existing.contains_key(*f)).collect();
    new.sort();
    for folder in new {
        if add_pack(conn, location, folder)? {
            changes.added += 1;
        }
    }
    for (folder, pack) in &existing {
        if !folders.contains(folder) || db::job_pending(conn, jobs::INGEST, Some(&pack.id), None)? {
            continue;
        }
        let touched = match pack.status.as_str() {
            "ready" => catch_up(conn, &pack.id, &root.join(folder))?,
            // A pack whose ingest failed (a share that dropped out halfway, say) is tried again.
            "failed" => {
                db::enqueue(conn, jobs::INGEST, Some(&pack.id), &json!({ "auto_name": false }))?;
                true
            }
            _ => false,
        };
        if touched {
            changes.updated += 1;
        }
    }
    conn.execute("UPDATE locations SET scanned_at = ?2 WHERE id = ?1", params![location, db::now()])?;
    if changes != Changes::default() {
        tracing::info!(location = %loc.name, added = changes.added, removed = changes.removed, updated = changes.updated, "watched folder scanned");
    }
    Ok(changes)
}

/// A pack of a watched location, as a scan finds it in the database.
struct Existing {
    id: String,
    status: String,
    missing_since: Option<String>,
}

/// A pack for a new folder, queued for ingest (which reads the folder in place). False if the folder already has
/// one.
fn add_pack(c: &Connection, location: i64, folder: &str) -> Result<bool> {
    let id = uuid::Uuid::now_v7().to_string();
    let added = c.execute(
        "INSERT INTO packs (id, name, status, source, files_total, created_at, location_id, folder)
         SELECT ?1, ?2, 'ingesting', 'watched folder', 0, ?3, ?4, ?2
          WHERE NOT EXISTS (SELECT 1 FROM packs WHERE location_id = ?4 AND folder = ?2)",
        params![id, folder, db::now(), location],
    )?;
    if added > 0 {
        db::enqueue(c, jobs::INGEST, Some(&id), &json!({ "auto_name": false }))?;
    }
    Ok(added > 0)
}

/// Bring a ready pack in line with its folder: assets whose file is gone are dropped, and new files are queued for
/// ingest. Returns whether anything changed.
fn catch_up(c: &Connection, pack_id: &str, dir: &Path) -> Result<bool> {
    let Ok(on_disk) = ingest::pack_rels(dir) else { return Ok(false) };
    // An asset converted from one of the pack's files (an Unreal package's texture, say) has no file of its own in
    // the folder: it stays as long as the file it came from does.
    let assets: Vec<(String, String, Option<String>)> = {
        let mut stmt = c.prepare(
            "SELECT a.id, a.path, o.path FROM assets a LEFT JOIN asset_origin o ON o.asset_id = a.id
              WHERE a.pack_id = ?1",
        )?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    let gone: Vec<&str> = assets
        .iter()
        .filter(|(_, path, origin)| !on_disk.contains(path) && origin.as_ref().is_none_or(|o| !on_disk.contains(o)))
        .map(|(id, _, _)| id.as_str())
        .collect();
    let known = db::known_paths(c, pack_id)?;
    let added = on_disk.iter().any(|p| !known.contains(p));
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
    use crate::blobs::tests::setup;

    /// A watched location over `dir/packs`, with one folder per name.
    fn watched(pool: &db::Pool, dir: &Path, folders: &[&str]) -> i64 {
        for f in folders {
            std::fs::create_dir_all(dir.join("packs").join(f)).unwrap();
        }
        let new = locations::NewLocation {
            name: None,
            mode: Mode::Watched,
            path: dir.join("packs").display().to_string(),
            kinds: vec![],
            catch_all: false,
        };
        locations::insert(&pool.get().unwrap(), &new).unwrap().id
    }

    /// Pretend the ingest queued for a pack has run.
    fn ingested(c: &Connection, folder: &str) -> String {
        let id: String = c.query_row("SELECT id FROM packs WHERE folder = ?1", [folder], |r| r.get(0)).unwrap();
        c.execute("UPDATE packs SET status = 'ready' WHERE id = ?1", [&id]).unwrap();
        c.execute("DELETE FROM jobs", []).unwrap();
        id
    }

    fn asset(c: &Connection, pack: &str, id: &str, path: &str) {
        c.execute("INSERT OR IGNORE INTO blobs (hash, size, created_at) VALUES (?1, 1, '')", [id]).unwrap();
        c.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
             VALUES (?1, ?2, ?1, ?3, ?3, 'x', 'other', '', '')",
            [id, pack, path],
        )
        .unwrap();
    }

    fn ids(c: &Connection, pack: &str) -> Vec<String> {
        let mut stmt = c.prepare("SELECT id FROM assets WHERE pack_id = ?1 ORDER BY id").unwrap();
        stmt.query_map([pack], |r| r.get(0)).unwrap().collect::<rusqlite::Result<_>>().unwrap()
    }

    #[test]
    fn a_rescan_follows_the_folder_without_undoing_conversions_or_deletions() {
        let dir = tempfile::tempdir().unwrap();
        let (_cfg, pool) = setup(dir.path());
        let loc = watched(&pool, dir.path(), &["Kit"]);
        let kit = dir.path().join("packs/Kit");
        for f in ["Foo.uasset", "a.png", "b.png"] {
            std::fs::write(kit.join(f), f).unwrap();
        }
        let mut c = pool.get().unwrap();
        assert_eq!(sweep(&mut c, loc).unwrap(), Changes { added: 1, ..Changes::default() });
        let pack = ingested(&c, "Kit");
        // Foo.uasset was converted to the model it held; b.png was deleted for good.
        asset(&c, &pack, "conv", "Foo.fbx");
        c.execute(
            "INSERT INTO asset_origin (asset_id, path, ext, method, converted_at)
             VALUES ('conv', 'Foo.uasset', 'uasset', 'native', '')",
            [],
        )
        .unwrap();
        asset(&c, &pack, "png", "a.png");
        asset(&c, &pack, "gone", "b.png");
        c.execute("UPDATE assets SET archived_at = 'x' WHERE id = 'gone'", []).unwrap();
        db::purge_archived(&mut c, &[], &["gone".to_owned()], false).unwrap();

        assert_eq!(sweep(&mut c, loc).unwrap(), Changes::default(), "nothing new: no ingest");
        assert!(!db::job_pending(&c, jobs::INGEST, Some(&pack), None).unwrap());

        std::fs::remove_file(kit.join("a.png")).unwrap();
        std::fs::write(kit.join("c.png"), "new").unwrap();
        assert_eq!(sweep(&mut c, loc).unwrap(), Changes { updated: 1, ..Changes::default() });
        assert_eq!(ids(&c, &pack), ["conv"], "the converted asset stays while its package is there");
        assert!(db::job_pending(&c, jobs::INGEST, Some(&pack), None).unwrap(), "c.png is queued");

        c.execute("DELETE FROM jobs", []).unwrap();
        std::fs::remove_file(kit.join("Foo.uasset")).unwrap();
        sweep(&mut c, loc).unwrap();
        assert!(ids(&c, &pack).is_empty(), "and goes with it");
    }

    #[test]
    fn a_missing_folder_keeps_its_pack_for_a_while_and_a_deleted_pack_stays_deleted() {
        let dir = tempfile::tempdir().unwrap();
        let (_cfg, pool) = setup(dir.path());
        let loc = watched(&pool, dir.path(), &["Kit", "Sounds", "Junk"]);
        let mut c = pool.get().unwrap();
        sweep(&mut c, loc).unwrap();
        let kit = ingested(&c, "Kit");
        ingested(&c, "Sounds");
        let junk = ingested(&c, "Junk");
        let since = |c: &Connection| -> Option<String> {
            c.query_row("SELECT missing_since FROM packs WHERE id = ?1", [&kit], |r| r.get(0)).unwrap()
        };

        let packs = dir.path().join("packs");
        std::fs::rename(packs.join("Kit"), dir.path().join("away")).unwrap();
        assert_eq!(sweep(&mut c, loc).unwrap(), Changes::default());
        assert!(since(&c).is_some(), "kept, marked missing");
        std::fs::rename(dir.path().join("away"), packs.join("Kit")).unwrap();
        sweep(&mut c, loc).unwrap();
        assert!(since(&c).is_none(), "back again");

        std::fs::rename(packs.join("Kit"), dir.path().join("away")).unwrap();
        sweep(&mut c, loc).unwrap();
        c.execute("UPDATE packs SET missing_since = '2020-01-01T00:00:00Z' WHERE id = ?1", [&kit]).unwrap();
        assert_eq!(sweep(&mut c, loc).unwrap(), Changes { removed: 1, ..Changes::default() });

        c.execute("UPDATE packs SET archived_at = 'x' WHERE id = ?1", [&junk]).unwrap();
        db::purge_archived(&mut c, &[junk], &[], false).unwrap();
        assert_eq!(sweep(&mut c, loc).unwrap(), Changes::default(), "a pack deleted for good isn't added back");
    }

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
