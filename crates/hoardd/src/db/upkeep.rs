//! Keeping the database quick and its files small as a library grows and changes. All of it is done a little at a
//! time, each step its own short write, so nothing else waits more than a few milliseconds for it: it runs on its
//! own every hour (see `maintenance::schedule_upkeep`).

use std::time::Duration;

use anyhow::Result;
use rusqlite::Connection;

/// Finished jobs are kept this long, for the lists of recent work; after that they only take room.
const KEEP_DONE_JOBS: time::Duration = time::Duration::days(30);
/// Finished jobs deleted per write.
const JOBS_STEP: usize = 500;
/// Pages of free room given back to the disk per write: a megabyte, with SQLite's usual 4 KB pages.
const ROOM_STEP: i64 = 256;
/// The gap between steps, for other writes to go first.
const PAUSE: Duration = Duration::from_millis(20);

/// SQLite's `auto_vacuum` setting that lets free room be given back a step at a time.
const INCREMENTAL: i64 = 2;

/// How big the database's files are, in bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Sizes {
    /// The database file.
    pub(crate) file: u64,
    /// Room inside it left by deleted rows, reused by new ones until it's given back to the disk.
    pub(crate) free: u64,
}

pub(crate) fn sizes(c: &Connection) -> Result<Sizes> {
    let pragma = |name: &str| c.query_row(&format!("PRAGMA {name}"), [], |r| r.get::<_, i64>(0));
    let page = u64::try_from(pragma("page_size")?).unwrap_or(0);
    let pages = |n: i64| u64::try_from(n).unwrap_or(0) * page;
    Ok(Sizes { file: pages(pragma("page_count")?), free: pages(pragma("freelist_count")?) })
}

/// What a tidy did.
#[derive(Debug, Default)]
pub(crate) struct Tidied {
    /// Finished jobs old enough to go.
    pub(crate) jobs: usize,
    /// Bytes of free room given back to the disk.
    pub(crate) freed: u64,
}

/// Deletes finished jobs older than a month, then gives back to the disk the room deleted rows left (deleting a
/// big pack can leave a hundred megabytes), each a step at a time with a pause between, and lets the write-ahead log
/// be folded into the database without waiting for anyone reading.
///
/// Measured on a library of 300,000 assets and left out: statistics for SQLite's planner (`ANALYZE`), since Hoard's
/// queries are written for the plans they get without them, and with them a few got slower and none faster; and
/// merging the search index, which SQLite already keeps in few enough pieces that searches by words took under a
/// millisecond however much it had been edited.
pub(crate) fn tidy(c: &Connection) -> Result<Tidied> {
    let before = time::OffsetDateTime::now_utc() - KEEP_DONE_JOBS;
    let before = before.format(&time::format_description::well_known::Rfc3339).unwrap_or_default();
    let mut jobs = 0;
    loop {
        let gone = c.execute(
            &format!(
                "DELETE FROM jobs WHERE id IN
                   (SELECT id FROM jobs WHERE status = 'done' AND updated_at < ?1 LIMIT {JOBS_STEP})"
            ),
            [&before],
        )?;
        jobs += gone;
        if gone < JOBS_STEP {
            break;
        }
        std::thread::sleep(PAUSE);
    }
    let start = sizes(c)?;
    if c.query_row("PRAGMA auto_vacuum", [], |r| r.get::<_, i64>(0))? == INCREMENTAL {
        let mut free = start.free;
        while free > 0 {
            give_back(c, ROOM_STEP)?;
            let now = sizes(c)?.free;
            // Nothing given back (another connection holding the room, say): leave it for the next tidy.
            if now >= free {
                break;
            }
            free = now;
            std::thread::sleep(PAUSE);
        }
    }
    // The checkpoints after each write already do this; one more in case the last of them found someone reading.
    // Never one that waits for readers (TRUNCATE, RESTART, FULL): it holds up every write while it waits.
    c.query_row("PRAGMA wal_checkpoint(PASSIVE)", [], |_| Ok(()))?;
    Ok(Tidied { jobs, freed: start.file.saturating_sub(sizes(c)?.file) })
}

/// Gives `pages` of free room back to the disk, in one write. SQLite gives back a page each time the pragma is
/// stepped, so it's stepped to the end: run once, as by `execute_batch`, it gave back 4 KB, and a library with 88 MB
/// to give back took eight minutes of pauses and grew the write-ahead log to 400 MB with a write per page.
fn give_back(c: &Connection, pages: i64) -> Result<()> {
    let mut vacuum = c.prepare(&format!("PRAGMA incremental_vacuum({pages})"))?;
    let mut rows = vacuum.query([])?;
    while rows.next()?.is_some() {}
    Ok(())
}

/// Lets the database give free room back to the disk a step at a time (see [`tidy`]). A setting SQLite only
/// changes by rewriting the whole database, so a library made before it is rewritten once, here, as the server
/// starts and before it takes requests: a few seconds for a big library. One that can't be (no room on the disk for
/// the copy, say) goes on as it is, reusing its free room but never giving it back.
pub(crate) fn give_back_room_over_time(c: &Connection) {
    if c.query_row("PRAGMA auto_vacuum", [], |r| r.get::<_, i64>(0)).is_ok_and(|v| v == INCREMENTAL) {
        return;
    }
    let started = std::time::Instant::now();
    let before = sizes(c).unwrap_or_default();
    // The copy VACUUM makes goes to a file, not memory: it's as big as the database.
    let rewritten = c.execute_batch(
        "PRAGMA auto_vacuum = INCREMENTAL;
         PRAGMA temp_store = FILE;
         VACUUM;",
    );
    c.execute_batch("PRAGMA temp_store = MEMORY;").ok();
    match rewritten {
        Ok(()) => {
            // VACUUM wrote the whole database through the log, and nothing's reading yet: empty it.
            c.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(())).ok();
            let after = sizes(c).unwrap_or_default();
            tracing::info!(
                before_bytes = before.file,
                after_bytes = after.file,
                took = ?started.elapsed(),
                "set the database to give back free room over time"
            );
        }
        Err(e) => tracing::warn!(error = %e, "could not set the database to give back free room over time"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::open;

    fn library() -> (tempfile::TempDir, crate::db::Pool) {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        (dir, pool)
    }

    #[test]
    fn a_tidy_deletes_only_finished_jobs_a_month_old_a_step_at_a_time() {
        let (_dir, pool) = library();
        let c = pool.get().unwrap();
        c.execute_batch(&format!(
            "INSERT INTO jobs (kind, status, run_after, created_at, updated_at) VALUES
               ('scan', 'failed', 'x', '2020-01-01T00:00:00Z', '2020-01-01T00:00:00Z'),
               ('scan', 'queued', 'x', '2020-01-01T00:00:00Z', '2020-01-01T00:00:00Z'),
               ('scan', 'done', 'x', '2020-01-01T00:00:00Z', '2999-01-01T00:00:00Z');
             WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < {})
             INSERT INTO jobs (kind, status, run_after, created_at, updated_at)
             SELECT 'scan', 'done', 'x', '2020-01-01T00:00:00Z', '2020-01-01T00:00:00Z' FROM k;",
            JOBS_STEP * 2
        ))
        .unwrap();
        let tidied = tidy(&c).unwrap();
        assert_eq!(tidied.jobs, JOBS_STEP * 2 + 1, "more than one step's worth");
        let left: Vec<String> = c
            .prepare("SELECT status || ' ' || updated_at FROM jobs ORDER BY id")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(left, ["failed 2020-01-01T00:00:00Z", "queued 2020-01-01T00:00:00Z", "done 2999-01-01T00:00:00Z"]);
    }

    #[test]
    fn a_new_library_gives_back_free_room_over_time() {
        let (_dir, pool) = library();
        let c = pool.get().unwrap();
        assert_eq!(c.query_row("PRAGMA auto_vacuum", [], |r| r.get::<_, i64>(0)).unwrap(), INCREMENTAL);
        c.execute_batch(
            "INSERT INTO packs (id, name, status, created_at) VALUES ('keep', 'Keep', 'ready', 'x'), ('go', 'Go', 'ready', 'x');
             WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < 1999)
             INSERT INTO blobs (hash, size, created_at) SELECT 'b' || i, i, 'x' FROM k;
             WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < 1999)
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, words, created_at, updated_at)
             SELECT 'a' || i, CASE WHEN i < 100 THEN 'keep' ELSE 'go' END, 'b' || i, 'file' || i || '.png',
                    'mossy stone ' || i, 'png', 'texture', printf('%.2000c', 'w'), 'x', 'x' FROM k;",
        )
        .unwrap();
        for i in 0..2000 {
            crate::db::reindex_asset(&c, &format!("a{i}")).unwrap();
        }
        drop(c);
        let mut c = pool.get().unwrap();
        assert!(crate::db::delete_pack(&mut c, "go").unwrap());
        let before = sizes(&c).unwrap();
        assert!(
            before.free > (ROOM_STEP as u64) * 4096 * 2,
            "the deleted pack left more than a step's room: {before:?}"
        );
        give_back(&c, ROOM_STEP).unwrap();
        let stepped = sizes(&c).unwrap();
        assert_eq!(before.free - stepped.free, (ROOM_STEP as u64) * 4096, "a whole step's room in one go");
        let tidied = tidy(&c).unwrap();
        let after = sizes(&c).unwrap();
        assert_eq!(after.free, 0);
        assert_eq!(tidied.freed, stepped.file - after.file);
        assert!(after.file < before.file, "{before:?} to {after:?}");
        let found: i64 =
            c.query_row("SELECT count(*) FROM assets_fts WHERE assets_fts MATCH 'mossy'", [], |r| r.get(0)).unwrap();
        assert_eq!(found, 100, "the rest is still found");
    }

    #[test]
    fn a_library_made_before_is_switched_once_as_it_opens() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("hoard.db");
        drop(open(&path, &dir.path().join("backups")).unwrap());
        // As every library made before: room never given back.
        Connection::open(&path).unwrap().execute_batch("PRAGMA auto_vacuum = NONE; VACUUM;").unwrap();
        let mode = |c: &Connection| c.query_row("PRAGMA auto_vacuum", [], |r| r.get::<_, i64>(0)).unwrap();
        assert_eq!(mode(&Connection::open(&path).unwrap()), 0);
        let pool = open(&path, &dir.path().join("backups")).unwrap();
        let c = pool.get().unwrap();
        assert_eq!(mode(&c), INCREMENTAL);
        assert_eq!(c.query_row("PRAGMA temp_store", [], |r| r.get::<_, i64>(0)).unwrap(), 2, "back to memory");
    }
}
