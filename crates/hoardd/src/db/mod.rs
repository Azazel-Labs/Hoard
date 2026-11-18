//! SQLite (WAL) metadata store with FTS5 search.
//!
//! The database only holds metadata. File bytes live in the blob store, so the DB stays small and
//! can be snapshotted with `VACUUM INTO` at any time (see the backup endpoint).
//!
//! The schema and its upgrades are in `schema`; the queries are in a module per table they mostly read.

use std::path::Path;

use anyhow::{Context, Result};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, TransactionBehavior};

mod assets;
mod collections;
mod jobs;
mod packs;
mod schema;
mod search;
mod tags;

pub(crate) use assets::*;
pub(crate) use collections::*;
pub(crate) use jobs::*;
pub(crate) use packs::*;
use schema::{MIGRATIONS, migrate};
pub(crate) use search::*;
pub(crate) use tags::*;

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
        // busy_timeout first: switching to WAL takes a lock, and connections opened at once must wait for it
        // rather than fail with "database is locked".
        c.execute_batch(
            "PRAGMA busy_timeout = 10000;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;",
        )
    });
    let pool = r2d2::Pool::builder().max_size(12).build(manager).context("open sqlite pool")?;
    let mut conn = pool.get()?;
    migrate(&mut conn, backups, MIGRATIONS)?;
    Ok(pool)
}

/// Run blocking DB work off the async runtime, in the language of the request it's for.
pub(crate) async fn call<T, F>(pool: &Pool, f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce(&mut Conn) -> Result<T> + Send + 'static,
{
    let pool = pool.clone();
    let language = crate::i18n::current();
    crate::profiler::db(tokio::task::spawn_blocking(move || {
        crate::i18n::blocking(language, || {
            let mut conn = pool.get()?;
            f(&mut conn)
        })
    }))
    .await?
}

/// An empty library with the whole schema, for other modules' tests.
#[cfg(test)]
pub(crate) fn test_library() -> Connection {
    let dir = tempfile::tempdir().expect("temp dir");
    let mut conn = Connection::open_in_memory().expect("in-memory database");
    migrate(&mut conn, dir.path(), MIGRATIONS).expect("schema");
    conn
}
