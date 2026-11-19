//! The database under the load of an import: many writers at once, searches and the job workers alongside.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use rusqlite::params;

use super::{Pool, open};

const WRITERS: usize = 8;
const READERS: usize = 4;
const BATCHES: usize = 30;
const PER_BATCH: usize = 64;

/// A batch of new assets, as ingest writes them: rows, tags and their search rows, in one transaction.
fn write_batch(pool: &Pool, writer: usize, batch: usize) -> anyhow::Result<()> {
    let mut c = pool.get()?;
    let tx = c.transaction()?;
    for i in 0..PER_BATCH {
        let id = format!("a{writer}-{batch}-{i}");
        tx.execute("INSERT INTO blobs (hash, size, created_at) VALUES (?1, 1, 'x')", [&id])?;
        tx.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, words, created_at, updated_at)
             VALUES (?1, 'p', ?1, ?2, ?3, 'png', 'texture', 'rock wall', 'x', 'x')",
            params![id, format!("Dir{writer}/{id}.png"), format!("rock {id}")],
        )?;
        super::upsert_tag(&tx, &id, "stone", hoard_core::TagSource::Rule, 0.6)?;
        super::reindex_asset(&tx, &id)?;
    }
    tx.commit()?;
    Ok(())
}

/// Writers queue for the lock in turn and none gives up with "database is locked", however many there are, while
/// searches and the job workers' polling carry on alongside. Each write is short, so none waits long for its turn.
#[test]
fn an_import_with_searches_alongside_is_never_locked_out() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    pool.get()
        .unwrap()
        .execute("INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'Rocks', 'ingesting', 'x')", [])
        .unwrap();
    let done = AtomicBool::new(false);
    let started = Instant::now();
    let (slowest, searches) = std::thread::scope(|s| {
        let writers: Vec<_> = (0..WRITERS)
            .map(|w| {
                let pool = &pool;
                s.spawn(move || {
                    let mut slowest = Duration::ZERO;
                    for b in 0..BATCHES {
                        let at = Instant::now();
                        write_batch(pool, w, b).unwrap_or_else(|e| panic!("writer {w}, batch {b}: {e:#}"));
                        slowest = slowest.max(at.elapsed());
                    }
                    slowest
                })
            })
            .collect();
        let readers: Vec<_> = (0..READERS)
            .map(|_| {
                let (pool, done) = (&pool, &done);
                s.spawn(move || {
                    let mut n = 0;
                    while !done.load(Ordering::Acquire) {
                        let c = pool.get().unwrap();
                        let _: i64 = c
                            .query_row(
                                "SELECT count(*) FROM assets_fts f JOIN assets a ON a.id = f.asset_id
                                 WHERE assets_fts MATCH 'rock'",
                                [],
                                |r| r.get(0),
                            )
                            .unwrap();
                        // A worker looking for a job.
                        super::claim_job(&c).unwrap();
                        n += 1;
                    }
                    n
                })
            })
            .collect();
        let slowest = writers.into_iter().map(|w| w.join().unwrap()).max().unwrap_or_default();
        done.store(true, Ordering::Release);
        (slowest, readers.into_iter().map(|r| r.join().unwrap()).sum::<usize>())
    });
    let c = pool.get().unwrap();
    let total = WRITERS * BATCHES * PER_BATCH;
    let rows: i64 = c.query_row("SELECT count(*) FROM assets_fts", [], |r| r.get(0)).unwrap();
    assert_eq!(rows as usize, total);
    let found: i64 =
        c.query_row("SELECT count(*) FROM assets_fts WHERE assets_fts MATCH 'stone'", [], |r| r.get(0)).unwrap();
    assert_eq!(found as usize, total);
    eprintln!(
        "{total} assets by {WRITERS} writers in {:.1?}, the slowest batch (waiting included) {slowest:.1?}; \
         {searches} searches meanwhile",
        started.elapsed()
    );
    // Generous for a loaded CI machine in a debug build; a batch that held the lock for long, or a waiter that kept
    // missing its turn, takes far longer.
    assert!(slowest < Duration::from_secs(10), "a batch took {slowest:?}");
    assert!(searches > 0);
}
