//! Watched folders: each folder inside a watched location is a pack, indexed where it sits. A scan adds a pack for
//! each new folder, drops the packs whose folder has been gone for [`FORGET_MISSING_AFTER`], and catches up packs
//! whose files changed. Every watched location is scanned at startup and every [`RESCAN_EVERY`] after, and from
//! Settings on demand. Most rescans are quick: a pack whose folders all have the modified time they had is left
//! alone, as nothing in it was added, removed or renamed. A full one, at startup, on demand and every
//! [`FULL_RESCAN_EVERY`], also looks at each file for ones edited where they sit (which leaves its folder's time be).

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::time::{Duration, Instant, UNIX_EPOCH};

use anyhow::Result;
use rusqlite::{Connection, params};
use serde_json::json;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::AppState;
use crate::locations::{self, Mode};
use crate::{db, ingest, issues, jobs};

/// How often every watched folder is looked at again. Folder-change notifications don't reach a container
/// through most network shares, so this is a plain timer.
const RESCAN_EVERY: Duration = Duration::from_mins(15);

/// How often a rescan looks at every file, not just the folders' modified times.
const FULL_RESCAN_EVERY: Duration = Duration::from_hours(24);

/// How long a pack whose folder has gone missing is kept, with its tags and notes, before it's dropped. A folder
/// that's missing is more often on a disk or share that's offline than deleted.
const FORGET_MISSING_AFTER: Duration = Duration::from_hours(72);

/// Queue a scan of a watched location (a full one looks at every file), unless one is already waiting or running
/// (two at once would both add the same new folders). A full scan asked for while a quick one waits makes it full.
pub(crate) fn enqueue(c: &Connection, state: &AppState, location: i64, full: bool) -> Result<()> {
    if !db::job_pending(c, jobs::SCAN, None, Some(("location", location)))? {
        db::enqueue(c, jobs::SCAN, None, &json!({ "location": location, "full": full }))?;
        state.wake.notify_one();
    } else if full {
        c.execute(
            "UPDATE jobs SET payload = json_set(payload, '$.full', json('true'))
              WHERE kind = ?1 AND status = 'queued' AND json_extract(payload, '$.location') = ?2",
            params![jobs::SCAN, location],
        )?;
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
        let mut last_full: Option<Instant> = None;
        loop {
            tick.tick().await;
            let full = last_full.is_none_or(|t| t.elapsed() >= FULL_RESCAN_EVERY);
            if full {
                last_full = Some(Instant::now());
            }
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
                    enqueue(c, &s, id, full)?;
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
    /// Packs whose folder was renamed, found under the new name.
    pub(crate) renamed: usize,
}

pub(crate) fn scan(state: &AppState, location: i64, full: bool) -> Result<Changes> {
    let changes = sweep(&mut *state.pool.get()?, location, full)?;
    if changes != Changes::default() {
        state.wake.notify_one();
    }
    Ok(changes)
}

fn sweep(conn: &mut Connection, location: i64, full: bool) -> Result<Changes> {
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
            issues::note(
                conn,
                issues::FOLDER,
                issues::At::location(location),
                &format!("{} can’t be read: {e}", loc.path),
            );
            return Ok(Changes::default());
        }
    };
    let existing: HashMap<String, Existing> = {
        let mut stmt = conn.prepare(
            "SELECT folder, id, status, missing_since, dir_times FROM packs
              WHERE location_id = ?1 AND folder IS NOT NULL",
        )?;
        stmt.query_map([location], |r| {
            let (id, status, missing_since, dir_times) = (r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?);
            Ok((r.get(0)?, Existing { id, status, missing_since, dir_times }))
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    // An empty folder where there were packs is far more often a share that didn't mount than a library someone
    // emptied: leave its packs alone.
    if folders.is_empty() && !existing.is_empty() {
        tracing::warn!(location = %loc.name, path = %loc.path, "watched folder is empty; keeping its packs");
        let why = format!("{} is empty, but it had packs: it may not be mounted.", loc.path);
        issues::note(conn, issues::FOLDER, issues::At::location(location), &why);
        return Ok(Changes::default());
    }
    issues::clear(conn, &[issues::FOLDER], issues::At::location(location));
    // Folders whose pack was deleted for good stay out.
    let skipped: Vec<String> = {
        let mut stmt = conn.prepare("SELECT folder FROM skipped_folders WHERE location_id = ?1")?;
        stmt.query_map([location], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
    };
    for f in &skipped {
        folders.remove(f);
    }

    let mut changes = Changes::default();
    let mut new: Vec<&String> = folders.iter().filter(|f| !existing.contains_key(*f)).collect();
    new.sort();
    // A new folder holding the files of a pack whose folder is gone is that folder renamed: the pack (with its tags
    // and notes) follows it rather than being dropped for a new one.
    let mut renamed: HashSet<&str> = HashSet::new();
    let mut gone: Vec<(&String, &Existing)> =
        existing.iter().filter(|(f, p)| !folders.contains(*f) && p.status == "ready").collect();
    gone.sort_by_key(|(f, _)| f.as_str());
    new.retain(|folder| {
        if gone.is_empty() {
            return true;
        }
        let Some(at) = renamed_from(conn, &root.join(folder), &gone) else { return true };
        let (old, pack) = gone.remove(at);
        match follow_rename(conn, &pack.id, old, folder, &root.join(folder)) {
            Ok(()) => {
                tracing::info!(location = %loc.name, from = %old, to = %folder, "watched pack folder renamed");
                renamed.insert(pack.id.as_str());
                changes.renamed += 1;
                false
            }
            Err(e) => {
                tracing::warn!(from = %old, to = %folder, error = %format!("{e:#}"), "couldn't follow a renamed folder");
                true
            }
        }
    });
    let now = OffsetDateTime::now_utc();
    for (folder, pack) in &existing {
        if renamed.contains(pack.id.as_str()) {
            continue;
        }
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
            "ready" if !full && unchanged(pack.dir_times.as_deref(), &root.join(folder)) => false,
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
        tracing::info!(location = %loc.name, added = changes.added, removed = changes.removed, updated = changes.updated, renamed = changes.renamed, "watched folder scanned");
    }
    Ok(changes)
}

/// Of the packs whose folder is gone, the one this new folder is a rename of: the one whose files it mostly holds
/// (at least [`SAME_FILES`] of the two lists of files together are in both).
fn renamed_from(c: &Connection, dir: &Path, gone: &[(&String, &Existing)]) -> Option<usize> {
    let on_disk = ingest::pack_rels(dir).ok().filter(|f| !f.is_empty())?;
    let mut best = None;
    for (i, (_, pack)) in gone.iter().enumerate() {
        let Ok(known) = db::known_paths(c, &pack.id) else { continue };
        let both = on_disk.iter().filter(|p| known.contains(*p)).count();
        let either = on_disk.len() + known.len() - both;
        let share = both as f64 / either.max(1) as f64;
        if share >= SAME_FILES && best.is_none_or(|(_, b)| share > b) {
            best = Some((i, share));
        }
    }
    best.map(|(i, _)| i)
}

/// How much of two folders' files must be the same for one to be taken for the other renamed.
const SAME_FILES: f64 = 0.8;

/// Point a pack at its folder's new name (and name it after it, unless it was named otherwise), then catch it up
/// with whatever else changed in the folder.
fn follow_rename(c: &Connection, pack_id: &str, old: &str, new: &str, dir: &Path) -> Result<()> {
    c.execute(
        "UPDATE packs SET folder = ?3, name = CASE WHEN name = ?2 THEN ?3 ELSE name END, missing_since = NULL
          WHERE id = ?1",
        params![pack_id, old, new],
    )?;
    catch_up(c, pack_id, dir)?;
    Ok(())
}

/// A pack of a watched location, as a scan finds it in the database.
struct Existing {
    id: String,
    status: String,
    missing_since: Option<String>,
    /// The modified time of each of its folders when it was last caught up (see [`dir_times`]).
    dir_times: Option<String>,
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

/// Bring a ready pack in line with its folder: assets whose file is gone are dropped, new files are queued for
/// ingest, and files changed where they sit are analyzed again. Returns whether anything changed.
fn catch_up(c: &Connection, pack_id: &str, dir: &Path) -> Result<bool> {
    // Taken before the files are listed, so a change made while they are is seen next time.
    let times = dir_times(dir).ok().map(|t| serde_json::to_string(&t)).transpose()?;
    let Ok(on_disk) = ingest::pack_rels(dir) else { return Ok(false) };
    // An asset converted from one of the pack's files (an Unreal package's texture, say) has no file of its own in
    // the folder: it stays as long as the file it came from does.
    let assets: Vec<(String, String, Option<String>)> = {
        let mut stmt = c.prepare(&format!(
            "SELECT a.id, a.path, o.path FROM assets a LEFT JOIN asset_origin o ON o.asset_id = a.id
                  WHERE a.pack_id = ?1 AND NOT {} AND NOT {}",
            crate::materials::IS_MATERIAL,
            crate::takes::IS_TAKE
        ))?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    let gone: Vec<&str> = assets
        .iter()
        .filter(|(_, path, origin)| !on_disk.contains(path) && origin.as_ref().is_none_or(|o| !on_disk.contains(o)))
        .map(|(id, _, _)| id.as_str())
        .collect();
    let known = db::known_paths(c, pack_id)?;
    let added = on_disk.iter().any(|p| !known.contains(p));
    if !gone.is_empty() {
        // Prepared once: a whole folder can go at once, and deleting from assets fires its triggers.
        let mut takes = c.prepare("DELETE FROM assets WHERE json_extract(meta, '$.take_of') = ?1")?;
        let mut asset = c.prepare("DELETE FROM assets WHERE id = ?1")?;
        for id in &gone {
            // With its takes, if it had them.
            takes.execute([id])?;
            asset.execute([id])?;
        }
    }
    if added {
        db::enqueue(c, jobs::INGEST, Some(pack_id), &json!({ "auto_name": false }))?;
    } else if !gone.is_empty() {
        // Ingesting looks again for materials; with nothing to ingest, a material may have lost a map.
        db::enqueue(c, jobs::MATERIALS, Some(pack_id), &json!({}))?;
    }
    let changed = changed_in_place(c, pack_id, dir)?;
    if !changed.is_empty() {
        tracing::info!(
            pack = pack_id,
            files = changed.len(),
            "watched files changed where they sit; reading them again"
        );
        db::enqueue(c, jobs::RETAG, Some(pack_id), &json!({ "assets": changed, "then_llm": false }))?;
    }
    c.prepare_cached("UPDATE packs SET dir_times = ?2 WHERE id = ?1")?.execute(params![pack_id, times])?;
    Ok(added || !gone.is_empty() || !changed.is_empty())
}

/// The modified time (in nanoseconds) of each folder in a pack, the pack's own as `""`. Adding, removing or renaming
/// a file changes its folder's time; editing one where it sits doesn't.
fn dir_times(dir: &Path) -> Result<BTreeMap<String, i64>> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, i64>) -> Result<()> {
        out.insert(rel_of(root, dir), modified_ns(dir)?);
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() && !ingest::is_junk(&rel_of(root, &path)) {
                walk(root, &path, out)?;
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    walk(dir, dir, &mut out)?;
    Ok(out)
}

fn rel_of(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    rel.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/")
}

fn modified_ns(path: &Path) -> Result<i64> {
    let t = std::fs::metadata(path)?.modified()?;
    Ok(t.duration_since(UNIX_EPOCH).map_or(0, |d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX)))
}

/// Whether each folder of a pack still has the modified time it had when the pack was last caught up: if so, no file
/// in it was added, removed or renamed since. Only its folders are looked at, not their files.
fn unchanged(stored: Option<&str>, dir: &Path) -> bool {
    let Some(times) = stored.and_then(|s| serde_json::from_str::<BTreeMap<String, i64>>(s).ok()) else { return false };
    !times.is_empty()
        && times.iter().all(|(rel, t)| {
            let path = if rel.is_empty() { dir.to_path_buf() } else { dir.join(rel) };
            modified_ns(&path).is_ok_and(|m| m == *t)
        })
}

/// The assets of a pack whose file was changed where it sits: a size other than the one stored, or a modified time
/// after the asset was last read, with contents that hash differently. Each is pointed at its new contents, and
/// what was made from the old ones (its thumbnail, previews and conversions) is let go, to be made again.
fn changed_in_place(c: &Connection, pack_id: &str, dir: &Path) -> Result<Vec<String>> {
    let assets: Vec<(String, String, String, i64, String)> = {
        let mut stmt = c.prepare(&format!(
            "SELECT a.id, a.path, a.blob_hash, b.size, a.updated_at FROM assets a JOIN blobs b ON b.hash = a.blob_hash
              WHERE a.pack_id = ?1 AND NOT {} AND NOT {}
                AND NOT EXISTS (SELECT 1 FROM asset_origin o WHERE o.asset_id = a.id)",
            crate::materials::IS_MATERIAL,
            crate::takes::IS_TAKE
        ))?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)))?
            .collect::<rusqlite::Result<_>>()?
    };
    let mut changed = Vec::new();
    for (id, rel, hash, size, read_at) in assets {
        let file = locations::watched_path(&dir.display().to_string(), None, &rel);
        let Ok(meta) = std::fs::metadata(&file) else { continue };
        let read_at = OffsetDateTime::parse(&read_at, &Rfc3339).ok();
        let modified = meta.modified().ok().map(OffsetDateTime::from);
        let newer = matches!((modified, read_at), (Some(m), Some(r)) if m > r);
        if i64::try_from(meta.len()).ok() == Some(size) && !newer {
            continue;
        }
        let blob = match crate::blobs::BlobStore::hash_file(&file) {
            Ok(b) => b,
            Err(e) => {
                tracing::debug!(file = %file.display(), error = %format!("{e:#}"), "changed watched file can't be read");
                continue;
            }
        };
        let now = db::now();
        if blob.hash == hash {
            // Touched but the same: not looked at again until it's touched again.
            c.prepare_cached("UPDATE assets SET updated_at = ?2 WHERE id = ?1")?.execute(params![id, now])?;
            continue;
        }
        // (Cached, as a folder's files can all change at once.)
        c.prepare_cached("INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING")?
            .execute(params![blob.hash, blob.size as i64, now])?;
        c.prepare_cached(
            "UPDATE assets SET blob_hash = ?2, thumb_hash = NULL, thumb_source = NULL, updated_at = ?3 WHERE id = ?1",
        )?
        .execute(params![id, blob.hash, now])?;
        c.prepare_cached("DELETE FROM derivatives WHERE asset_id = ?1")?.execute([&id])?;
        changed.push(id);
    }
    Ok(changed)
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

    /// An asset read just now from a file holding its own path (as the files in these tests do).
    fn asset(c: &Connection, pack: &str, id: &str, path: &str) {
        c.execute(
            "INSERT OR IGNORE INTO blobs (hash, size, created_at) VALUES (?1, ?2, '')",
            params![id, path.len() as i64],
        )
        .unwrap();
        c.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
             VALUES (?1, ?2, ?1, ?3, ?3, 'x', 'other', '', ?4)",
            params![id, pack, path, db::now()],
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
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { added: 1, ..Changes::default() });
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

        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes::default(), "nothing new: no ingest");
        assert!(!db::job_pending(&c, jobs::INGEST, Some(&pack), None).unwrap());

        std::fs::remove_file(kit.join("a.png")).unwrap();
        std::fs::write(kit.join("c.png"), "new").unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { updated: 1, ..Changes::default() });
        assert_eq!(ids(&c, &pack), ["conv"], "the converted asset stays while its package is there");
        assert!(db::job_pending(&c, jobs::INGEST, Some(&pack), None).unwrap(), "c.png is queued");

        c.execute("DELETE FROM jobs", []).unwrap();
        std::fs::remove_file(kit.join("Foo.uasset")).unwrap();
        sweep(&mut c, loc, true).unwrap();
        assert!(ids(&c, &pack).is_empty(), "and goes with it");
    }

    #[test]
    fn a_file_changed_where_it_sits_is_read_again_and_one_only_touched_is_not() {
        let dir = tempfile::tempdir().unwrap();
        let (_cfg, pool) = setup(dir.path());
        let loc = watched(&pool, dir.path(), &["Kit"]);
        let kit = dir.path().join("packs/Kit");
        for f in ["a.png", "b.png"] {
            std::fs::write(kit.join(f), f).unwrap();
        }
        let mut c = pool.get().unwrap();
        sweep(&mut c, loc, true).unwrap();
        let pack = ingested(&c, "Kit");
        asset(&c, &pack, "a", "a.png");
        let b_hash = crate::blobs::BlobStore::hash_file(&kit.join("b.png")).unwrap().hash;
        asset(&c, &pack, &b_hash, "b.png");
        c.execute("INSERT INTO blobs (hash, size, created_at) VALUES ('thumb', 1, '')", []).unwrap();
        c.execute("UPDATE assets SET thumb_hash = 'thumb', thumb_source = 'browser'", []).unwrap();
        c.execute(
            "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES ('a', 'views', 'thumb', '')",
            [],
        )
        .unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes::default(), "as they were read");

        // a.png is painted over; b.png is saved again as it was, and only looks newer. Both were read an hour ago.
        c.execute(
            "UPDATE assets SET updated_at = ?1",
            [(OffsetDateTime::now_utc() - Duration::from_hours(1)).format(&Rfc3339).unwrap()],
        )
        .unwrap();
        std::fs::write(kit.join("a.png"), "a.png, painted over").unwrap();
        std::fs::write(kit.join("b.png"), "b.png").unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { updated: 1, ..Changes::default() });
        let job: String = c
            .query_row("SELECT payload FROM jobs WHERE kind = ?1 AND pack_id = ?2", [jobs::RETAG, &pack], |r| r.get(0))
            .unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&job).unwrap(),
            json!({ "assets": ["a"], "then_llm": false })
        );
        let (hash, thumb, source): (String, Option<String>, Option<String>) = c
            .query_row("SELECT blob_hash, thumb_hash, thumb_source FROM assets WHERE id = 'a'", [], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .unwrap();
        assert_eq!(hash, crate::blobs::BlobStore::hash_file(&kit.join("a.png")).unwrap().hash);
        assert_eq!((thumb, source), (None, None), "its old pictures are let go");
        let views: i64 =
            c.query_row("SELECT COUNT(*) FROM derivatives WHERE asset_id = 'a'", [], |r| r.get(0)).unwrap();
        assert_eq!(views, 0);
        let b: (String, Option<String>) = c
            .query_row("SELECT blob_hash, thumb_hash FROM assets WHERE id = ?1", [&b_hash], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .unwrap();
        assert_eq!(b, (b_hash.clone(), Some("thumb".to_owned())), "b.png is as it was");

        c.execute("DELETE FROM jobs", []).unwrap();
        assert_eq!(
            sweep(&mut c, loc, true).unwrap(),
            Changes::default(),
            "and neither is read again until it changes again"
        );
    }

    #[test]
    fn a_renamed_folder_keeps_its_pack_and_one_with_other_files_is_new() {
        let dir = tempfile::tempdir().unwrap();
        let (_cfg, pool) = setup(dir.path());
        let loc = watched(&pool, dir.path(), &["Kit"]);
        let packs = dir.path().join("packs");
        let files = ["a.png", "b.png", "c.png", "d.png", "e.png"];
        for f in files {
            std::fs::write(packs.join("Kit").join(f), f).unwrap();
        }
        let mut c = pool.get().unwrap();
        sweep(&mut c, loc, true).unwrap();
        let pack = ingested(&c, "Kit");
        for f in files {
            asset(&c, &pack, f, f);
        }
        let folder_and_name = |c: &Connection| -> (String, String, Option<String>) {
            c.query_row("SELECT folder, name, missing_since FROM packs WHERE id = ?1", [&pack], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .unwrap()
        };
        let pack_count =
            |c: &Connection| -> i64 { c.query_row("SELECT COUNT(*) FROM packs", [], |r| r.get(0)).unwrap() };

        // Renamed, with one file gone on the way: still mostly the same files.
        std::fs::rename(packs.join("Kit"), packs.join("Kit 2")).unwrap();
        std::fs::remove_file(packs.join("Kit 2/e.png")).unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { renamed: 1, ..Changes::default() });
        assert_eq!(folder_and_name(&c), ("Kit 2".to_owned(), "Kit 2".to_owned(), None));
        assert_eq!(pack_count(&c), 1);
        assert_eq!(ids(&c, &pack), ["a.png", "b.png", "c.png", "d.png"], "the missing file goes with the catch-up");

        // A pack named otherwise keeps its name.
        c.execute("UPDATE packs SET name = 'My kit' WHERE id = ?1", [&pack]).unwrap();
        std::fs::rename(packs.join("Kit 2"), packs.join("Kit 3")).unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { renamed: 1, ..Changes::default() });
        assert_eq!(folder_and_name(&c), ("Kit 3".to_owned(), "My kit".to_owned(), None));

        // A folder gone while another with other files turns up: the first is missing, the second a pack of its own.
        std::fs::remove_dir_all(packs.join("Kit 3")).unwrap();
        std::fs::create_dir_all(packs.join("Other")).unwrap();
        for f in ["a.png", "x.png", "y.png"] {
            std::fs::write(packs.join("Other").join(f), f).unwrap();
        }
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { added: 1, ..Changes::default() });
        assert_eq!(pack_count(&c), 2);
        assert!(folder_and_name(&c).2.is_some(), "the old pack waits in case its folder comes back");
    }

    #[test]
    fn a_quick_rescan_skips_packs_whose_folders_are_as_they_were() {
        let dir = tempfile::tempdir().unwrap();
        let (_cfg, pool) = setup(dir.path());
        let loc = watched(&pool, dir.path(), &["Kit"]);
        let kit = dir.path().join("packs/Kit");
        std::fs::create_dir_all(kit.join("sub")).unwrap();
        for f in ["a.png", "sub/s.png"] {
            std::fs::write(kit.join(f), f).unwrap();
        }
        let mut c = pool.get().unwrap();
        sweep(&mut c, loc, true).unwrap();
        let pack = ingested(&c, "Kit");
        asset(&c, &pack, "a", "a.png");
        asset(&c, &pack, "s", "sub/s.png");
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes::default());
        let times: String = c.query_row("SELECT dir_times FROM packs WHERE id = ?1", [&pack], |r| r.get(0)).unwrap();
        let folders: Vec<String> = serde_json::from_str::<BTreeMap<String, i64>>(&times).unwrap().into_keys().collect();
        assert_eq!(folders, ["", "sub"]);

        // Edited where it sits: its folder's time stays, so only a full scan sees it.
        std::fs::write(kit.join("a.png"), "a.png, painted over").unwrap();
        assert_eq!(sweep(&mut c, loc, false).unwrap(), Changes::default());
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { updated: 1, ..Changes::default() });
        c.execute("DELETE FROM jobs", []).unwrap();
        assert_eq!(sweep(&mut c, loc, false).unwrap(), Changes::default());

        // A file added deep in the pack changes its folder's time, which a quick scan sees.
        std::fs::write(kit.join("sub/t.png"), "new").unwrap();
        assert_eq!(sweep(&mut c, loc, false).unwrap(), Changes { updated: 1, ..Changes::default() });
        assert!(db::job_pending(&c, jobs::INGEST, Some(&pack), None).unwrap());
    }

    #[test]
    fn a_missing_folder_keeps_its_pack_for_a_while_and_a_deleted_pack_stays_deleted() {
        let dir = tempfile::tempdir().unwrap();
        let (_cfg, pool) = setup(dir.path());
        let loc = watched(&pool, dir.path(), &["Kit", "Sounds", "Junk"]);
        let mut c = pool.get().unwrap();
        sweep(&mut c, loc, true).unwrap();
        let kit = ingested(&c, "Kit");
        ingested(&c, "Sounds");
        let junk = ingested(&c, "Junk");
        let since = |c: &Connection| -> Option<String> {
            c.query_row("SELECT missing_since FROM packs WHERE id = ?1", [&kit], |r| r.get(0)).unwrap()
        };

        let packs = dir.path().join("packs");
        std::fs::rename(packs.join("Kit"), dir.path().join("away")).unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes::default());
        assert!(since(&c).is_some(), "kept, marked missing");
        std::fs::rename(dir.path().join("away"), packs.join("Kit")).unwrap();
        sweep(&mut c, loc, true).unwrap();
        assert!(since(&c).is_none(), "back again");

        std::fs::rename(packs.join("Kit"), dir.path().join("away")).unwrap();
        sweep(&mut c, loc, true).unwrap();
        c.execute("UPDATE packs SET missing_since = '2020-01-01T00:00:00Z' WHERE id = ?1", [&kit]).unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes { removed: 1, ..Changes::default() });

        c.execute("UPDATE packs SET archived_at = 'x' WHERE id = ?1", [&junk]).unwrap();
        db::purge_archived(&mut c, &[junk], &[], false).unwrap();
        assert_eq!(sweep(&mut c, loc, true).unwrap(), Changes::default(), "a pack deleted for good isn't added back");
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
