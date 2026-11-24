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
mod changes;
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
pub(crate) use changes::library_writes;
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
    // Upgraded (and rewritten if need be) on a connection of its own first: the pool's each note what's changed with triggers on the tables
    // there are as they open (see `changes::watch`), and r2d2 opens them all at once.
    {
        let mut conn = Connection::open(path).with_context(|| format!("open {}", path.display()))?;
        set_up(&mut conn)?;
        migrate(&mut conn, backups, MIGRATIONS)?;
        // (It sets `temp_store`, which would throw away a pool connection's triggers.)
        upkeep::give_back_room_over_time(&conn);
    }
    let manager = SqliteConnectionManager::file(path).with_init(|c| {
        // First: setting `temp_store` throws away temporary triggers.
        set_up(c)?;
        // What each transaction changes, for what's kept from the library to be put right (see `changes`).
        let pending = Arc::new(Mutex::new(changes::Pending::default()));
        // (Its lock is only ever free: a connection is used by one thread at a time.)
        changes::watch(c, &pending)?;
        let (rolled, committed) = (Arc::clone(&pending), pending);
        c.rollback_hook(Some(move || {
            changes::rolled_back(&mut rolled.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
            turn_over();
        }))?;
        // Every write counts, on whichever connection (see `writes`), once it's there for others to read: unlike a
        // commit hook, which runs just before. Taking the place of SQLite's own hook, this checkpoints the log the
        // way that does (at 1000 pages, without waiting on readers).
        c.wal_hook(Some(move |wal: &mut rusqlite::hooks::Wal, pages| {
            changes::committed(&mut committed.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
            WRITES.fetch_add(1, Ordering::AcqRel);
            if pages >= 1000 {
                // As SQLite's own: a checkpoint that can't finish now is done by a later one.
                wal.checkpoint().ok();
            }
            finished();
            Ok(())
        }))
    });
    // Readers don't wait for each other or for the writer (WAL), so there's room for many: searches (at most
    // READ_LANE of them, see `read`), the job workers and every other request at once.
    let pool = r2d2::Pool::builder().max_size(24).build(manager).context("open sqlite pool")?;
    Ok(pool)
}

/// How every connection to the library behaves.
fn set_up(c: &mut Connection) -> rusqlite::Result<()> {
    // Transactions take the write lock when they begin. A deferred one that reads and then writes can't wait for
    // another writer (SQLite would deadlock), so it fails at once with "database is locked", busy_timeout or not:
    // two tag edits at the same moment did.
    c.set_transaction_behavior(TransactionBehavior::Immediate);
    // Waiting first: switching to WAL takes a lock, and connections opened at once must wait for it rather than
    // fail with "database is locked".
    c.busy_handler(Some(wait_turn))?;
    // The log is used again from the start once everything in it is in the database, but never made smaller: after
    // a big import, or writes while a long search kept it from being reused, it stayed hundreds of megabytes.
    // `journal_size_limit` cuts it back to 64 MB each time it's reused.
    c.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;
         PRAGMA temp_store = MEMORY;
         PRAGMA journal_size_limit = 67108864;",
    )
}

/// How long a write waits for the one before it to finish before giving up with "database is locked".
const WAIT_FOR_WRITE: Duration = Duration::from_secs(30);

/// How often a waiting write looks again, should nobody wake it.
const LOOK_AGAIN: Duration = Duration::from_millis(20);

/// Whose turn it is to write.
struct Turn {
    /// Writes since the server started, bumped as each finishes.
    writes: u64,
    /// When a write last waited for another to finish: one still waiting looks again at least every [`LOOK_AGAIN`].
    waited: Option<Instant>,
    /// The write under way, once it has changed the library.
    holder: Option<Holder>,
}

/// A write under way: since it first changed the library, and what it's for (its request's or job's span), so a write
/// kept waiting can say what it waits for, and one held long says so (see [`SLOW`]). (Since its first change, not its
/// beginning: work done in a transaction before it changes anything isn't counted.)
struct Holder {
    since: Instant,
    span: tracing::Span,
}

/// A write began changing the library.
pub(crate) fn began_writing() {
    turn().holder = Some(Holder { since: Instant::now(), span: tracing::Span::current() });
}

/// The write under way ended: one that held the database long says so, in the log of whatever it was for.
fn let_go(holder: Option<Holder>) {
    if let Some(h) = holder
        && h.since.elapsed() >= SLOW
    {
        tracing::warn!(parent: &h.span, secs = h.since.elapsed().as_secs_f32(), "held the database for writing");
    }
}

/// [`Turn`], and a signal for the writers waiting theirs.
static TURN: (Mutex<Turn>, Condvar) = (Mutex::new(Turn { writes: 0, waited: None, holder: None }), Condvar::new());

fn turn() -> std::sync::MutexGuard<'static, Turn> {
    TURN.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

thread_local! {
    /// When the write this thread is trying to start began waiting.
    static WAITING_SINCE: Cell<Option<Instant>> = const { Cell::new(None) };
    /// Whether the write this thread is trying to start has said in the log that it's been kept waiting.
    static SAID_WAITING: Cell<bool> = const { Cell::new(false) };
}

/// Waiting for another connection's write to finish (SQLite's busy handler; `tries` counts from 0 per write). A
/// write that finishes wakes the waiters at once, where SQLite's busy timeout sleeps up to 100 ms between looks, so
/// under a steady run of short writes a waiter could miss every gap between them.
fn wait_turn(tries: i32) -> bool {
    let now = Instant::now();
    let since = WAITING_SINCE.with(|w| {
        if tries == 0 || w.get().is_none() {
            w.set(Some(now));
            SAID_WAITING.set(false);
        }
        w.get().unwrap_or(now)
    });
    let waited = now - since;
    let gave_up = waited >= WAIT_FOR_WRITE;
    let mut turn = turn();
    if gave_up || (waited >= SLOW && !SAID_WAITING.replace(true)) {
        say_waiting(&turn, waited, gave_up);
    }
    if gave_up {
        return false;
    }
    turn.waited = Some(now);
    // A write that finished just before this began waiting, or one by another process, wakes nobody: look again
    // soon regardless.
    drop(TURN.1.wait_timeout(turn, LOOK_AGAIN));
    true
}

/// A write kept waiting long: in its own log (the request's or job's), and in the log of the write it waits for.
fn say_waiting(turn: &Turn, waited: Duration, gave_up: bool) {
    let secs = waited.as_secs_f32();
    if let Some(h) = &turn.holder {
        let held = h.since.elapsed().as_secs_f32();
        tracing::warn!(secs, gave_up, held_secs = held, "a write waits for the database, which another holds");
        tracing::warn!(parent: &h.span, secs = held, waiting_secs = secs, "holding the database while a write waits");
    } else {
        // Held by a write that hasn't changed anything yet, or another program.
        tracing::warn!(secs, gave_up, "a write waits for the database");
    }
}

/// A write was undone: whoever's waiting can go.
fn turn_over() {
    let mut turn = turn();
    turn.writes += 1;
    let holder = turn.holder.take();
    drop(turn);
    TURN.1.notify_all();
    let_go(holder);
}

/// A write finished, and if another is waiting, it goes next. Woken, a waiter still has to be given a core before it
/// can begin, while the writer that just finished is running already and usually begins its next first: on a busy
/// machine, writers going one after another (an import's) kept a tag edit waiting until it gave up. So this one
/// steps aside until another write finishes, or a waiter has had time to look again. Whether one is waiting is told
/// by when one last looked, which can't be left behind by a write that ended some way no hook sees.
fn finished() {
    let mut turn = turn();
    turn.writes += 1;
    let holder = turn.holder.take();
    TURN.1.notify_all();
    if turn.waited.is_some_and(|at| at.elapsed() < 2 * LOOK_AGAIN) {
        let at = turn.writes;
        drop(TURN.1.wait_timeout_while(turn, LOOK_AGAIN, |t| t.writes == at));
    } else {
        drop(turn);
    }
    let_go(holder);
}

/// How long one slice of a long run of writes holds the database (see [`Slices`]).
const SLICE: Duration = Duration::from_millis(100);

/// A long run of writes (a big pack's materials, say) in short transactions, so the other writers waiting meanwhile
/// (someone saving a setting) get a turn every [`SLICE`] rather than once it's all done. Each slice is all or
/// nothing; the run as a whole isn't, so it must be fine for others to see it partly done.
pub(crate) struct Slices<'c> {
    conn: &'c Connection,
    tx: Option<rusqlite::Transaction<'c>>,
    since: Instant,
}

impl<'c> Slices<'c> {
    pub(crate) fn new(conn: &'c Connection) -> Self {
        Self { conn, tx: None, since: Instant::now() }
    }

    /// The transaction to write in next: a new one once the last has held the database for a slice.
    pub(crate) fn tx(&mut self) -> Result<&rusqlite::Transaction<'c>> {
        if self.since.elapsed() >= SLICE {
            self.turn()?;
        }
        if self.tx.is_none() {
            self.tx = Some(rusqlite::Transaction::new_unchecked(self.conn, TransactionBehavior::Immediate)?);
            self.since = Instant::now();
        }
        Ok(self.tx.as_ref().expect("just begun"))
    }

    /// Commit what's written so far, and give any writer waiting a moment to go first.
    pub(crate) fn turn(&mut self) -> Result<()> {
        let Some(tx) = self.tx.take() else { return Ok(()) };
        tx.commit()?;
        // The commit woke the waiters (see `turn_over`), but this thread would usually begin again before they look:
        // wait for one of them to finish, or a few milliseconds if nobody's waiting.
        let seen = turn();
        let at = seen.writes;
        drop(TURN.1.wait_timeout_while(seen, Duration::from_millis(5), |t| t.writes == at));
        Ok(())
    }

    /// Commit the last slice.
    pub(crate) fn commit(mut self) -> Result<()> {
        match self.tx.take() {
            Some(tx) => Ok(tx.commit()?),
            None => Ok(()),
        }
    }
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
    let asked = Instant::now();
    crate::profiler::db(tokio::task::spawn_blocking(move || {
        span.in_scope(|| {
            crate::i18n::blocking(language, || {
                let mut conn = connection(&pool, asked)?;
                timed(&mut conn, f)
            })
        })
    }))
    .await?
}

/// A connection from the pool, warning in the log when the work asking for it (at `asked`) waited long for a thread to
/// run on or a connection to run with: then the server has more to do than it has room for, and everything waits.
fn connection(pool: &Pool, asked: Instant) -> Result<Conn> {
    let began = Instant::now();
    let conn = pool.get();
    let (thread, connection) = (began - asked, began.elapsed());
    if thread + connection >= WAITED {
        let state = pool.state();
        tracing::warn!(
            thread_secs = thread.as_secs_f32(),
            connection_secs = connection.as_secs_f32(),
            connections = state.connections,
            idle = state.idle_connections,
            "database work waited to begin"
        );
    }
    Ok(conn?)
}

/// See [`connection`].
const WAITED: Duration = Duration::from_secs(2);

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
    let asked = Instant::now();
    let out = crate::profiler::db(tokio::task::spawn_blocking(move || {
        span.in_scope(|| {
            crate::i18n::blocking(language, || {
                let mut conn = connection(&pool, asked)?;
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
