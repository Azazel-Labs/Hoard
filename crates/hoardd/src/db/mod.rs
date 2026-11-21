//! SQLite (WAL) metadata store with FTS5 search.
//!
//! The database only holds metadata. File bytes live in the blob store, so the DB stays small and
//! can be snapshotted with `VACUUM INTO` at any time (see the backup endpoint). It looks after itself (see `upkeep`).
//!
//! The schema and its upgrades are in `schema`; the queries are in a module per table they mostly read.

use std::cell::Cell;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, InterruptHandle, TransactionBehavior};

mod assets;
mod collections;
mod edits;
mod jobs;
mod kept;
mod packs;
mod schema;
mod search;
#[cfg(test)]
mod stress;
mod tags;
mod upkeep;

pub(crate) use assets::*;
pub(crate) use collections::*;
pub(crate) use edits::*;
pub(crate) use jobs::*;
pub(crate) use kept::{Kept, save_in as keep_in, save_later, saved_later};
pub(crate) use packs::*;
use schema::{MIGRATIONS, migrate};
pub(crate) use search::*;
pub(crate) use tags::*;
pub(crate) use upkeep::tidy;

pub(crate) type Pool = r2d2::Pool<SqliteConnectionManager>;
pub(crate) type Conn = r2d2::PooledConnection<SqliteConnectionManager>;

pub(crate) fn now() -> String {
    time::OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap_or_default()
}

/// Open the database, upgrading its schema first if this build is newer (backing it up into `backups` before
/// touching it). Refuses a database written by a newer build rather than guess at a schema it doesn't know.
pub(crate) fn open(path: &Path, backups: &Path) -> Result<Pool> {
    // WAL is a property of the file, so set it once here, alone: on a new database the pool's connections would all
    // try to switch it at the same moment, and SQLite fails that race with "database is locked" (busy_timeout
    // doesn't cover it). After this, each connection's `journal_mode = WAL` below finds it set and is a no-op.
    Connection::open(path)
        .and_then(|c| c.execute_batch("PRAGMA journal_mode = WAL;"))
        .with_context(|| format!("open {}", path.display()))?;
    let manager = SqliteConnectionManager::file(path).with_init(|c| {
        // Transactions take the write lock when they begin. A deferred one that reads and then writes can't wait for
        // another writer (SQLite would deadlock), so it fails at once with "database is locked", busy_timeout or not:
        // two tag edits at the same moment did.
        c.set_transaction_behavior(TransactionBehavior::Immediate);
        // Waiting first: switching to WAL takes a lock, and connections opened at once must wait for it rather than
        // fail with "database is locked".
        c.busy_handler(Some(wait_turn))?;
        c.rollback_hook(Some(turn_over))?;
        // The log is used again from the start once everything in it is in the database, but never made smaller: after
        // a big import, or writes while a long search kept it from being reused, it stayed hundreds of megabytes.
        // `journal_size_limit` cuts it back to 64 MB each time it's reused.
        c.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA journal_size_limit = 67108864;",
        )?;
        // Every write counts, on whichever connection (see `writes`), once it's there for others to read: unlike a
        // commit hook, which runs just before. Taking the place of SQLite's own hook, this checkpoints the log the
        // way that does (at 1000 pages, without waiting on readers).
        c.wal_hook(Some(|wal: &mut rusqlite::hooks::Wal, pages| {
            WRITES.fetch_add(1, Ordering::AcqRel);
            turn_over();
            if pages >= 1000 {
                // As SQLite's own: a checkpoint that can't finish now is done by a later one.
                wal.checkpoint().ok();
            }
            Ok(())
        }))
    });
    // Readers don't wait for each other or for the writer (WAL), so there's room for many: searches (at most
    // READ_LANE of them, see `read`), the job workers and every other request at once.
    let pool = r2d2::Pool::builder().max_size(24).build(manager).context("open sqlite pool")?;
    let mut conn = pool.get()?;
    migrate(&mut conn, backups, MIGRATIONS)?;
    upkeep::give_back_room_over_time(&conn);
    Ok(pool)
}

/// How long a write waits for the one before it to finish before giving up with "database is locked".
const WAIT_FOR_WRITE: Duration = Duration::from_secs(30);

/// Writes since the server started (bumped as each finishes), and a signal for the writers waiting their turn.
static TURN: (Mutex<u64>, Condvar) = (Mutex::new(0), Condvar::new());

thread_local! {
    /// When the write this thread is trying to start began waiting.
    static WAITING_SINCE: Cell<Option<Instant>> = const { Cell::new(None) };
}

/// Waiting for another connection's write to finish (SQLite's busy handler; `tries` counts from 0 per write). A
/// write that finishes wakes the waiters at once, where SQLite's busy timeout sleeps up to 100 ms between looks, so
/// under a steady run of short writes a waiter could miss every gap between them.
fn wait_turn(tries: i32) -> bool {
    let now = Instant::now();
    let since = WAITING_SINCE.with(|w| {
        if tries == 0 || w.get().is_none() {
            w.set(Some(now));
        }
        w.get().unwrap_or(now)
    });
    if now - since >= WAIT_FOR_WRITE {
        return false;
    }
    let (writes, freed) = &TURN;
    let seen = writes.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // A write that finished just before this began waiting, or one by another process, wakes nobody: look again
    // soon regardless.
    drop(freed.wait_timeout(seen, Duration::from_millis(20)));
    true
}

/// A write finished (or was undone): whoever's waiting can go.
fn turn_over() {
    let (writes, freed) = &TURN;
    *writes.lock().unwrap_or_else(std::sync::PoisonError::into_inner) += 1;
    freed.notify_all();
}

/// How many times the library has been written to since the server started.
static WRITES: AtomicU64 = AtomicU64::new(0);

/// [`WRITES`]: what's worked out from the library and kept (a search's counts, say) is good while this stays the same.
pub(crate) fn writes() -> u64 {
    WRITES.load(Ordering::Acquire)
}

/// Database work that takes this long gets a warning in the log naming it.
const SLOW: Duration = Duration::from_secs(5);

/// Run blocking DB work off the async runtime, in the language of the request it's for.
pub(crate) async fn call<T, F>(pool: &Pool, f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce(&mut Conn) -> Result<T> + Send + 'static,
{
    let pool = pool.clone();
    let language = crate::i18n::current();
    // Logs from here carry the request that asked, and work that holds the database for seconds says where it's
    // from (its closure's name is its caller's), so a "database is locked" can be traced to whoever held the lock.
    let span = tracing::Span::current();
    crate::profiler::db(tokio::task::spawn_blocking(move || {
        span.in_scope(|| {
            crate::i18n::blocking(language, || {
                let mut conn = pool.get()?;
                timed(&mut conn, f)
            })
        })
    }))
    .await?
}

/// Run `f`, warning in the log when it held the database for [`SLOW`] or more.
fn timed<T, F: FnOnce(&mut Conn) -> Result<T>>(conn: &mut Conn, f: F) -> Result<T> {
    let started = Instant::now();
    let out = f(conn);
    let took = started.elapsed();
    if took >= SLOW {
        tracing::warn!(work = std::any::type_name::<F>(), secs = took.as_secs_f32(), "slow database work");
    }
    out
}

/// How many searches run at once. Past that they wait their turn, so however many a browser asks for (scrolling a
/// big library asks for a page of tiles at a time) they never take every connection: the other requests and the
/// imports always find one.
const READ_LANE: usize = 6;
static READS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(READ_LANE);

/// [`call`] for reading that a request can walk away from, like a search: it waits its turn (see [`READ_LANE`]),
/// and once the request is gone (the browser moved on to another search, or was closed) it's stopped where it is,
/// or never started. Only for reading: work stopped halfway is undone.
pub(crate) async fn read<T, F>(pool: &Pool, f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce(&mut Conn) -> Result<T> + Send + 'static,
{
    let _turn = READS.acquire().await?;
    let pool = pool.clone();
    let language = crate::i18n::current();
    let span = tracing::Span::current();
    let mut walked = Walked(Arc::new(Watch::default()), false);
    let watch = Arc::clone(&walked.0);
    let out = crate::profiler::db(tokio::task::spawn_blocking(move || {
        span.in_scope(|| {
            crate::i18n::blocking(language, || {
                let mut conn = pool.get()?;
                *watch.handle.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
                    Some(conn.get_interrupt_handle());
                // Gone before the handle was there to stop it with: don't start.
                let out = if watch.gone.load(Ordering::Acquire) {
                    bail!("the request was dropped")
                } else {
                    timed(&mut conn, f)
                };
                // Taken back before the connection goes back to the pool, so a late stop can't land on someone
                // else's work.
                watch.handle.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                out
            })
        })
    }))
    .await;
    walked.1 = true;
    out?
}

/// Whether the request a [`read`] is for went away, and how to stop the work it was doing.
#[derive(Default)]
struct Watch {
    gone: AtomicBool,
    handle: Mutex<Option<InterruptHandle>>,
}

/// Dropped with its request: unless the work finished (`.1`), it's stopped.
struct Walked(Arc<Watch>, bool);

impl Drop for Walked {
    fn drop(&mut self) {
        if self.1 {
            return;
        }
        self.0.gone.store(true, Ordering::Release);
        if let Some(h) = self.0.handle.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref() {
            h.interrupt();
        }
    }
}

/// An empty library with the whole schema, for other modules' tests.
#[cfg(test)]
pub(crate) fn test_library() -> Connection {
    let dir = tempfile::tempdir().expect("temp dir");
    let mut conn = Connection::open_in_memory().expect("in-memory database");
    migrate(&mut conn, dir.path(), MIGRATIONS).expect("schema");
    conn
}
