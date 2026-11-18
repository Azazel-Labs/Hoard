//! The job queue.

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, params};

use super::now;

/// Whether a job of this kind (for this pack, if given) is waiting or running, optionally only one whose payload
/// has `field` set to `value`.
pub(crate) fn job_pending(
    conn: &Connection,
    kind: &str,
    pack_id: Option<&str>,
    field: Option<(&str, i64)>,
) -> Result<bool> {
    let (path, value) = match field {
        Some((f, v)) => (Some(format!("$.{f}")), Some(v)),
        None => (None, None),
    };
    Ok(conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status IN ('queued', 'running')
                          AND (?2 IS NULL OR pack_id = ?2)
                          AND (?3 IS NULL OR json_extract(payload, ?3) = ?4))",
        params![kind, pack_id, path, value],
        |r| r.get(0),
    )?)
}

pub(crate) fn enqueue(
    conn: &Connection,
    kind: &str,
    pack_id: Option<&str>,
    payload: &serde_json::Value,
) -> Result<i64> {
    let now = now();
    conn.execute(
        "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4, ?4)",
        params![kind, pack_id, payload.to_string(), now],
    )?;
    Ok(conn.last_insert_rowid())
}

#[derive(Debug)]
pub(crate) struct JobRow {
    pub(crate) id: i64,
    pub(crate) kind: String,
    pub(crate) pack_id: Option<String>,
    pub(crate) payload: serde_json::Value,
    pub(crate) attempts: i64,
}

pub(crate) fn claim_job(conn: &Connection) -> Result<Option<JobRow>> {
    let now = now();
    Ok(conn
        .query_row(
            "UPDATE jobs SET status = 'running', attempts = attempts + 1, updated_at = ?1
             WHERE id = (SELECT id FROM jobs WHERE status = 'queued' AND run_after <= ?1 ORDER BY id LIMIT 1)
             RETURNING id, kind, pack_id, payload, attempts",
            [&now],
            |r| {
                let payload: String = r.get(3)?;
                Ok(JobRow {
                    id: r.get(0)?,
                    kind: r.get(1)?,
                    pack_id: r.get(2)?,
                    payload: serde_json::from_str(&payload).unwrap_or_default(),
                    attempts: r.get(4)?,
                })
            },
        )
        .optional()?)
}

pub(crate) fn finish_job(conn: &Connection, id: i64, error: Option<&str>, retry_in_secs: Option<i64>) -> Result<()> {
    let now = time::OffsetDateTime::now_utc();
    let fmt = |t: time::OffsetDateTime| t.format(&time::format_description::well_known::Rfc3339).unwrap_or_default();
    match (error, retry_in_secs) {
        (None, _) => conn.execute(
            "UPDATE jobs SET status = 'done', error = NULL, updated_at = ?2 WHERE id = ?1",
            params![id, fmt(now)],
        )?,
        (Some(e), Some(secs)) => conn.execute(
            "UPDATE jobs SET status = 'queued', error = ?2, run_after = ?3, updated_at = ?4 WHERE id = ?1",
            params![id, e, fmt(now + time::Duration::seconds(secs)), fmt(now)],
        )?,
        (Some(e), None) => conn.execute(
            "UPDATE jobs SET status = 'failed', error = ?2, updated_at = ?3 WHERE id = ?1",
            params![id, e, fmt(now)],
        )?,
    };
    Ok(())
}

/// After a crash, anything left `running` gets another go.
pub(crate) fn requeue_stale_jobs(conn: &Connection) -> Result<usize> {
    Ok(conn.execute("UPDATE jobs SET status = 'queued' WHERE status = 'running'", [])?)
}
