//! The database under the load of an import: many writers at once, searches and the job workers alongside.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
        tx.prepare_cached("INSERT INTO blobs (hash, size, created_at) VALUES (?1, 1, 'x')")?.execute([&id])?;
        tx.prepare_cached(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, words, created_at, updated_at)
             VALUES (?1, 'p', ?1, ?2, ?3, 'png', 'texture', 'rock wall', 'x', 'x')",
        )?
        .execute(params![id, format!("Dir{writer}/{id}.png"), format!("rock {id}")])?;
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
    let written = AtomicUsize::new(0);
    let started = Instant::now();
    let ((slowest, passed_over), searches) = std::thread::scope(|s| {
        let writers: Vec<_> = (0..WRITERS)
            .map(|w| {
                let (pool, written) = (&pool, &written);
                s.spawn(move || {
                    let (mut slowest, mut passed_over) = (Duration::ZERO, 0);
                    for b in 0..BATCHES {
                        let (at, before) = (Instant::now(), written.load(Ordering::Acquire));
                        write_batch(pool, w, b).unwrap_or_else(|e| {
                            panic!(
                                "writer {w}, batch {b}: {e:#} after {:?}, {} written meanwhile",
                                at.elapsed(),
                                written.load(Ordering::Acquire) - before
                            )
                        });
                        slowest = slowest.max(at.elapsed());
                        passed_over = passed_over.max(written.fetch_add(1, Ordering::AcqRel) - before);
                    }
                    (slowest, passed_over)
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
                        super::claim_job(&c, usize::MAX).unwrap();
                        n += 1;
                        drop(c);
                        // Searches come one after another, not in a spin that takes cores from the writers: on a
                        // busy machine that alone kept a batch waiting past `WAIT_FOR_WRITE`.
                        std::thread::sleep(Duration::from_millis(1));
                    }
                    n
                })
            })
            .collect();
        // The searches stop however the writers ended: a writer that failed must fail the test, not leave the
        // searches going forever.
        let ended: Vec<_> = writers.into_iter().map(std::thread::ScopedJoinHandle::join).collect();
        done.store(true, Ordering::Release);
        let searches = readers.into_iter().map(|r| r.join().unwrap()).sum::<usize>();
        let (slowest, passed_over) = ended
            .into_iter()
            .map(|w| w.unwrap_or_else(|panic| std::panic::resume_unwind(panic)))
            .fold((Duration::ZERO, 0), |(t, n), (t2, n2)| (t.max(t2), n.max(n2)));
        ((slowest, passed_over), searches)
    });
    let c = pool.get().unwrap();
    let total = WRITERS * BATCHES * PER_BATCH;
    let rows: i64 = c.query_row("SELECT count(*) FROM assets_fts", [], |r| r.get(0)).unwrap();
    assert_eq!(rows as usize, total);
    let found: i64 =
        c.query_row("SELECT count(*) FROM assets_fts WHERE assets_fts MATCH 'stone'", [], |r| r.get(0)).unwrap();
    assert_eq!(found as usize, total);
    eprintln!(
        "{total} assets by {WRITERS} writers in {:.1?}, the slowest batch (waiting included) {slowest:.1?}, the most \
         written while one waited {passed_over}; {searches} searches meanwhile",
        started.elapsed()
    );
    // How long a batch waits grows with how busy the machine is, so it's no measure on a loaded CI runner; how many
    // others are written while it waits doesn't. Taking turns strictly, that's one each of the other writers; a waiter
    // that kept missing its turn would see a good share of the whole import go past it.
    assert!(passed_over < WRITERS * BATCHES / 2, "{passed_over} batches were written while one waited for its turn");
    assert!(searches > 0);
}

/// A long run of writes in slices leaves room between them: settings saved while it runs each get their turn, not
/// only once the whole run is done.
#[test]
fn a_long_run_in_slices_lets_others_write_meanwhile() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let (begun, shorts_done) = (AtomicBool::new(false), AtomicBool::new(false));
    // The run goes on until the short writes are all in, or until this if they can't get in while it does.
    let most = Duration::from_secs(20);
    let (outlasted, longest) = std::thread::scope(|s| {
        let long = s.spawn(|| {
            let c = pool.get().unwrap();
            let mut slices = super::Slices::new(&c);
            let started = Instant::now();
            let mut i = 0;
            while !shorts_done.load(Ordering::Acquire) && started.elapsed() < most {
                let tx = slices.tx().unwrap();
                tx.execute("INSERT INTO blobs (hash, size, created_at) VALUES (?1, 1, 'x')", [format!("long{i}")])
                    .unwrap();
                begun.store(true, Ordering::Release);
                std::thread::sleep(Duration::from_millis(2));
                i += 1;
            }
            let outlasted = shorts_done.load(Ordering::Acquire);
            slices.commit().unwrap();
            outlasted
        });
        while !begun.load(Ordering::Acquire) {
            std::thread::sleep(Duration::from_millis(5));
        }
        let mut longest = Duration::ZERO;
        for i in 0..10 {
            let at = Instant::now();
            pool.get()
                .unwrap()
                .execute("INSERT INTO blobs (hash, size, created_at) VALUES (?1, 1, 'x')", [format!("short{i}")])
                .unwrap();
            longest = longest.max(at.elapsed());
            std::thread::sleep(Duration::from_millis(90));
        }
        shorts_done.store(true, Ordering::Release);
        (long.join().unwrap(), longest)
    });
    eprintln!("the longest a short write waited: {longest:.1?}");
    // Timed, a wait of a slice or so is no measure on a loaded machine; what is, is that they all got in meanwhile.
    assert!(outlasted, "the short writes only got in once the long run was over; one waited {longest:?}");
}
