//! The schema: the baseline every database starts from, the steps since, and the upgrade that runs them,
//! backing the database up first.

use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::Connection;

use super::now;

mod baseline;
mod migrations;

use baseline::{BASELINE, BASELINE_VERSION};
pub(in crate::db) use migrations::MIGRATIONS;

/// How many automatic pre-upgrade backups to keep in `<data dir>/backups`.
const KEEP_UPGRADE_BACKUPS: usize = 5;

fn user_version(conn: &Connection) -> Result<i64> {
    Ok(conn.query_row("PRAGMA user_version", [], |r| r.get(0))?)
}

pub(in crate::db) fn migrate(conn: &mut Connection, backups: &Path, steps: &[&str]) -> Result<()> {
    let target = BASELINE_VERSION + steps.len() as i64;
    let version = user_version(conn)?;
    if version > target {
        anyhow::bail!(
            "the database is at schema version {version}, but this hoardd {} only knows up to {target}: a newer \
             Hoard has upgraded it. Run that version again, or restore the backup it made before upgrading \
             ({}/hoard-pre-upgrade-v{target}-*.db, or older).",
            hoard_core::VERSION,
            backups.display(),
        );
    }
    if version == target {
        return Ok(());
    }

    let empty: bool =
        conn.query_row("SELECT NOT EXISTS (SELECT 1 FROM sqlite_schema WHERE type = 'table')", [], |r| r.get(0))?;
    if !empty {
        let path = backup_before_upgrade(conn, backups, version)?;
        tracing::info!(from = version, to = target, backup = %path.display(), "upgrading the database schema");
    }

    let step = |conn: &mut Connection, sql: &str, to: i64| -> Result<()> {
        let tx = conn.transaction()?;
        tx.execute_batch(sql).with_context(|| format!("database migration to schema version {to}"))?;
        tx.execute_batch(&format!("PRAGMA user_version = {to}"))?;
        tx.commit()?;
        Ok(())
    };
    if version < BASELINE_VERSION {
        step(conn, BASELINE, BASELINE_VERSION)?;
    }
    for (i, sql) in steps.iter().enumerate() {
        let to = BASELINE_VERSION + i as i64 + 1;
        if to > version {
            step(conn, sql, to)?;
        }
    }
    Ok(())
}

/// Snapshot the database before a schema upgrade, keeping the newest [`KEEP_UPGRADE_BACKUPS`] such snapshots.
fn backup_before_upgrade(conn: &Connection, dir: &Path, from: i64) -> Result<std::path::PathBuf> {
    std::fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
    let stamp = now().replace([':', '-'], "").replace('T', "-");
    let path = dir.join(format!("hoard-pre-upgrade-v{from}-{stamp}.db"));
    conn.execute("VACUUM INTO ?1", [path.to_string_lossy()])
        .with_context(|| format!("back up the database to {} before upgrading it", path.display()))?;

    let mut old: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("hoard-pre-upgrade-")))
        .filter_map(|p| Some((p.metadata().ok()?.modified().ok()?, p)))
        .collect();
    old.sort();
    for (_, p) in old.iter().rev().skip(KEEP_UPGRADE_BACKUPS) {
        if let Err(e) = std::fs::remove_file(p) {
            tracing::warn!(path = %p.display(), error = %e, "could not remove an old pre-upgrade backup");
        }
    }
    Ok(path)
}

#[cfg(test)]
mod tests;
