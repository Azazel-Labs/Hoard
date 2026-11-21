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
    // Looked for first: an UPDATE waits its turn to write even when it finds nothing, and the workers ask often.
    let waiting: bool = conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM jobs WHERE status = 'queued' AND run_after <= ?1)",
        [&now],
        |r| r.get(0),
    )?;
    if !waiting {
        return Ok(None);
    }
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

/// Packs left being read or tagged with no job left to finish them (`pack`, or every one) are ready: a job that failed
/// partway (an Unreal conversion, say) used to leave its pack "LLM is tagging" for good, with its menu greyed out.
/// What's done of it is usable, and it can be read or tagged again from there; `note` says why, when there's a reason.
/// A watched folder's scan reads its packs itself, so none is touched while a scan or a move is waiting or running.
pub(crate) fn settle_packs(conn: &Connection, pack: Option<&str>, note: Option<&str>) -> Result<usize> {
    let settled = conn.execute(
        "UPDATE packs SET status = 'ready', error = COALESCE(?2, error)
          WHERE status IN ('ingesting', 'analyzing') AND (?1 IS NULL OR id = ?1)
            AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'))
            AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind IN (?3, ?4) AND j.status IN ('queued', 'running'))",
        params![pack, note, crate::jobs::SCAN, crate::jobs::RELOCATE],
    )?;
    if pack.is_none() && settled > 0 {
        tracing::info!(packs = settled, "packs left unfinished by a job that stopped are ready as they are");
    }
    Ok(settled)
}

/// After a crash, anything left `running` gets another go.
pub(crate) fn requeue_stale_jobs(conn: &Connection) -> Result<usize> {
    Ok(conn.execute("UPDATE jobs SET status = 'queued' WHERE status = 'running'", [])?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pack_with_no_job_left_to_finish_it_is_ready() {
        let conn = crate::db::test_library();
        conn.execute_batch(
            "INSERT INTO packs (id, name, status, created_at) VALUES
               ('stuck', 'Stuck', 'analyzing', 'x'), ('reading', 'Reading', 'ingesting', 'x'),
               ('tagging', 'Tagging', 'analyzing', 'x'), ('done', 'Done', 'ready', 'x'),
               ('broken', 'Broken', 'failed', 'x'), ('sent', 'Sent', 'uploading', 'x');
             INSERT INTO jobs (kind, pack_id, payload, status, run_after, created_at, updated_at) VALUES
               ('unreal', 'stuck', '{}', 'failed', 'x', 'x', 'x'),
               ('ingest', 'reading', '{}', 'running', 'x', 'x', 'x'),
               ('llm_tag', 'tagging', '{}', 'queued', 'x', 'x', 'x');",
        )
        .unwrap();
        let status = |id: &str| -> (String, Option<String>) {
            conn.query_row("SELECT status, error FROM packs WHERE id = ?1", [id], |r| Ok((r.get(0)?, r.get(1)?)))
                .unwrap()
        };
        // Only its own pack, after a job.
        assert_eq!(settle_packs(&conn, Some("done"), Some("no")).unwrap(), 0);
        assert_eq!(settle_packs(&conn, Some("stuck"), Some("unreal stopped: bad package")).unwrap(), 1);
        assert_eq!(status("stuck"), ("ready".into(), Some("unreal stopped: bad package".into())));
        // Every pack, at start: those with a job waiting or running, and those not being read, are left as they are.
        conn.execute("UPDATE packs SET status = 'analyzing', error = NULL WHERE id = 'stuck'", []).unwrap();
        // Not while a watched folder's scan may be reading packs of its own.
        conn.execute(
            "INSERT INTO jobs (kind, payload, status, run_after, created_at, updated_at)
             VALUES (?1, '{}', 'queued', 'x', 'x', 'x')",
            [crate::jobs::SCAN],
        )
        .unwrap();
        assert_eq!(settle_packs(&conn, None, None).unwrap(), 0);
        conn.execute("UPDATE jobs SET status = 'done' WHERE kind = ?1", [crate::jobs::SCAN]).unwrap();
        assert_eq!(settle_packs(&conn, None, None).unwrap(), 1);
        assert_eq!(status("stuck"), ("ready".into(), None));
        for (id, want) in
            [("reading", "ingesting"), ("tagging", "analyzing"), ("broken", "failed"), ("sent", "uploading")]
        {
            assert_eq!(status(id).0, want, "{id}");
        }
    }
}
