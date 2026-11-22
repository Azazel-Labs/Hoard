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

/// How soon a job runs, as SQL over the kind in `col`: 0 first. Work someone is waiting on (a pack being read, a
/// watched folder looked over, the library moved) comes before what was asked for (conversions, reading again), which
/// comes before what runs in the background (see [`crate::jobs::BACKGROUND`]); each in the order queued.
pub(crate) fn rank_sql(col: &str) -> String {
    let list = |kinds: &[&str]| kinds.iter().map(|k| format!("'{k}'")).collect::<Vec<_>>().join(", ");
    format!(
        "CASE WHEN {col} IN ({}) THEN 0 WHEN {col} IN ({}) THEN 2 ELSE 1 END",
        list(crate::jobs::URGENT),
        list(crate::jobs::BACKGROUND)
    )
}

/// The next job to run, most urgent first. Background jobs take at most `background_slots` workers between them, so
/// one left free can take anything more urgent that comes along.
pub(crate) fn claim_job(conn: &Connection, background_slots: usize) -> Result<Option<JobRow>> {
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
    // An upload going into a pack the library had already waits for whatever's working on that pack to finish, so
    // two jobs never add to one pack at once.
    let background = crate::jobs::BACKGROUND.iter().map(|k| format!("'{k}'")).collect::<Vec<_>>().join(", ");
    let slots = i64::try_from(background_slots).unwrap_or(i64::MAX);
    Ok(conn
        .query_row(
            &format!(
                "UPDATE jobs SET status = 'running', attempts = attempts + 1, updated_at = ?1
                 WHERE id = (SELECT q.id FROM jobs q WHERE q.status = 'queued' AND q.run_after <= ?1
                               AND NOT (q.kind = ?2 AND json_extract(q.payload, '$.from') IS NOT NULL
                                        AND EXISTS (SELECT 1 FROM jobs r WHERE r.pack_id = q.pack_id AND r.status = 'running'))
                               AND (q.kind NOT IN ({background})
                                    OR (SELECT COUNT(*) FROM jobs r WHERE r.status = 'running' AND r.kind IN ({background})) < ?3)
                              ORDER BY {}, q.id LIMIT 1)
                 RETURNING id, kind, pack_id, payload, attempts",
                rank_sql("q.kind")
            ),
            params![&now, crate::jobs::INGEST, slots],
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

    /// An upload going into a pack waits while another job works on that pack; other jobs carry on past it.
    #[test]
    fn an_upload_into_a_busy_pack_waits_its_turn() {
        let conn = crate::db::test_library();
        conn.execute_batch(
            "INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'P', 'analyzing', 'x'), ('q', 'Q', 'ready', 'x');
             INSERT INTO jobs (kind, pack_id, payload, status, run_after, created_at, updated_at) VALUES
               ('llm_tag', 'p', '{}', 'running', '0', 'x', 'x'),
               ('ingest', 'p', '{\"from\":\"u\"}', 'queued', '0', 'x', 'x'),
               ('ingest', 'q', '{}', 'queued', '0', 'x', 'x');",
        )
        .unwrap();
        assert_eq!(claim_job(&conn, 1).unwrap().unwrap().pack_id.as_deref(), Some("q"));
        assert!(claim_job(&conn, 1).unwrap().is_none());
        conn.execute("UPDATE jobs SET status = 'done' WHERE pack_id = 'p' AND kind = 'llm_tag'", []).unwrap();
        let job = claim_job(&conn, 1).unwrap().unwrap();
        assert_eq!((job.kind.as_str(), job.pack_id.as_deref()), ("ingest", Some("p")));
    }

    /// An upload queued behind a library's worth of AI tagging runs first, and the tagging never takes the last
    /// worker while something more urgent could want it.
    #[test]
    fn urgent_work_goes_before_background_work() {
        let conn = crate::db::test_library();
        conn.execute_batch(
            "INSERT INTO jobs (kind, pack_id, payload, status, run_after, created_at, updated_at) VALUES
               ('llm_tag', 'a', '{}', 'queued', '0', 'x', 'x'),
               ('embed', NULL, '{}', 'queued', '0', 'x', 'x'),
               ('convert', 'b', '{}', 'queued', '0', 'x', 'x'),
               ('ingest', 'c', '{}', 'queued', '0', 'x', 'x');",
        )
        .unwrap();
        let next = |slots| claim_job(&conn, slots).unwrap().map(|j| j.kind);
        assert_eq!(next(1).as_deref(), Some("ingest"));
        assert_eq!(next(1).as_deref(), Some("convert"));
        assert_eq!(next(1).as_deref(), Some("llm_tag"));
        // One background job is running and that's all it may have.
        assert_eq!(next(1), None);
        assert_eq!(next(2).as_deref(), Some("embed"));
    }

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
