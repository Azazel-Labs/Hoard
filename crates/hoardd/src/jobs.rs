//! Durable background job queue in SQLite. Jobs survive restarts; anything left `running` after a crash
//! is re-queued at startup.

use std::time::Duration;

use anyhow::Result;
use futures_util::{StreamExt, stream};
use hoard_core::PackStatus;
use rusqlite::params;

use crate::AppState;
use crate::{db, ingest, llm};

pub(crate) const INGEST: &str = "ingest";
pub(crate) const RETAG: &str = "retag";
pub(crate) const LLM_TAG: &str = "llm_tag";
pub(crate) const SCRUB: &str = "scrub";
pub(crate) const CONVERT: &str = "convert";
pub(crate) const UNREAL: &str = "unreal_convert";

/// With the backoff below, the last try comes about 15 minutes after the first: long enough for a local model
/// that's still downloading on the first start of the compose stack.
const MAX_LLM_ATTEMPTS: i64 = 5;

/// Starts as many workers as Settings can ask for; the ones past the current count stay idle (checking every couple
/// of seconds, and never taking a wake-up meant for a working one) so the count can change without a restart.
pub(crate) fn spawn_workers(state: &AppState) {
    for n in 0..crate::settings::MAX_WORKERS {
        let state = std::sync::Arc::clone(state);
        tokio::spawn(async move {
            tracing::debug!(worker = n, "job worker started");
            loop {
                if n >= state.settings.get().workers {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
                match next_job(&state).await {
                    Ok(Some(job)) => run(&state, job).await,
                    Ok(None) => {
                        tokio::select! {
                            () = state.wake.notified() => {}
                            () = tokio::time::sleep(Duration::from_secs(5)) => {}
                        }
                    }
                    Err(e) => {
                        tracing::error!(error = ?e, "job queue error");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });
    }
}

async fn next_job(state: &AppState) -> Result<Option<db::JobRow>> {
    db::call(&state.pool, |c| db::claim_job(c)).await
}

/// How a job reads in the activity list.
pub(crate) fn job_label(kind: &str, pack: &str) -> String {
    match kind {
        INGEST => format!("Ingesting {pack}"),
        RETAG => format!("Re-tagging {pack}"),
        LLM_TAG => format!("Tagging {pack} with AI"),
        SCRUB => "Checking stored files for damage".to_owned(),
        CONVERT => format!("Converting {pack} with Blender"),
        UNREAL => format!("Converting Unreal packages in {pack}"),
        other => format!("{other} {pack}"),
    }
}

async fn run(state: &AppState, job: db::JobRow) {
    let started = std::time::Instant::now();
    let pack_id = job.pack_id.clone().unwrap_or_default();
    tracing::info!(id = job.id, kind = %job.kind, pack = %pack_id, attempt = job.attempts, "job started");
    let pack_name = match &job.pack_id {
        Some(pid) => {
            let pid = pid.clone();
            db::call(&state.pool, move |c| Ok(db::get_pack(c, &pid)?.map(|p| p.name))).await.ok().flatten()
        }
        None => None,
    };
    let kind: &'static str = match job.kind.as_str() {
        INGEST => INGEST,
        RETAG => RETAG,
        LLM_TAG => LLM_TAG,
        SCRUB => SCRUB,
        CONVERT => CONVERT,
        UNREAL => UNREAL,
        _ => "job",
    };
    let _activity = crate::activity::start(
        kind,
        job_label(&job.kind, pack_name.as_deref().unwrap_or("a pack")),
        job.pack_id.as_deref(),
    );

    let result: Result<()> = match job.kind.as_str() {
        INGEST => {
            let auto_name = job.payload.get("auto_name").and_then(serde_json::Value::as_bool).unwrap_or(false);
            let pid = pack_id.clone();
            let done = blocking(state, move |s, rt| ingest::ingest_pack(s, rt, &pid, auto_name)).await;
            if done.is_ok() {
                crate::context::fill_store_text(state, &pack_id).await;
            }
            done
        }
        RETAG => {
            let then_llm = job.payload.get("then_llm").and_then(serde_json::Value::as_bool).unwrap_or(false);
            blocking(state, move |s, rt| {
                ingest::retag_pack(s, rt, &pack_id)?;
                ingest::finish_rules_stage(s, &*s.pool.get()?, &pack_id, then_llm)
            })
            .await
        }
        LLM_TAG => llm::tag_pack(state, &pack_id).await,
        SCRUB => scrub(state).await,
        CONVERT => crate::convert::convert_pack(state, &pack_id).await,
        UNREAL => {
            let req = serde_json::from_value(job.payload.clone()).unwrap_or_default();
            crate::extract::convert_pack(state, &pack_id, req).await.map(|_| ())
        }
        other => Err(anyhow::anyhow!("unknown job kind {other}")),
    };

    let (error, retry, pack_update) = match &result {
        Ok(()) => (None, None, None),
        Err(e) => {
            let msg = format!("{e:#}");
            tracing::error!(id = job.id, kind = %job.kind, error = %msg, "job failed");
            match job.kind.as_str() {
                // LLM endpoints flake (rate limits, cold or still-downloading local models): back off (30 s, 2 min,
                // 4.5 min, 8 min) and retry, and if it keeps failing, the pack is still usable with rule tags, so
                // mark it ready with a note.
                LLM_TAG if job.attempts < MAX_LLM_ATTEMPTS => (Some(msg), Some(30 * job.attempts * job.attempts), None),
                LLM_TAG => (Some(msg.clone()), None, Some((PackStatus::Ready, format!("LLM tagging gave up: {msg}")))),
                // Ingest moves files out of staging as it goes, so it isn't safely re-runnable.
                INGEST | RETAG => (Some(msg.clone()), None, Some((PackStatus::Failed, msg))),
                _ => (Some(msg), None, None),
            }
        }
    };
    let id = job.id;
    let pack_id = job.pack_id.clone();
    let res = db::call(&state.pool, move |c| {
        db::finish_job(c, id, error.as_deref(), retry)?;
        if let (Some(pid), Some((status, msg))) = (pack_id, pack_update) {
            db::set_pack_status(c, &pid, status, Some(&msg))?;
        }
        Ok(())
    })
    .await;
    if let Err(e) = res {
        tracing::error!(error = ?e, "could not record job result");
    }
    tracing::info!(id = job.id, kind = %job.kind, ok = result.is_ok(), secs = started.elapsed().as_secs_f32(), "job finished");
}

/// Run CPU/disk-heavy work on a low-priority thread, with a runtime handle for storage calls.
async fn blocking<F>(state: &AppState, f: F) -> Result<()>
where
    F: FnOnce(&AppState, &tokio::runtime::Handle) -> Result<()> + Send + 'static,
{
    let state = std::sync::Arc::clone(state);
    let rt = tokio::runtime::Handle::current();
    crate::heavy::run("job", move || f(&state, &rt)).await?
}

/// Re-hash every blob in storage and record the result. Detects bit rot and missing files on the NAS.
async fn scrub(state: &AppState) -> Result<()> {
    let hashes: Vec<String> = db::call(&state.pool, |c| {
        let mut stmt = c.prepare("SELECT hash FROM blobs ORDER BY verified_at IS NOT NULL, verified_at")?;
        let rows = stmt.query_map([], |r| r.get(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    let total = hashes.len();
    let checked = std::sync::atomic::AtomicUsize::new(0);
    let checked = &checked;
    let results: Vec<(String, &'static str)> = stream::iter(hashes)
        .map(|h| async move {
            let n = checked.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            crate::activity::job_progress(SCRUB, None, n, total, "files");
            let status = match state.blobs.verify(&h).await {
                Ok(Some(true)) => "ok",
                Ok(Some(false)) => "corrupt",
                Ok(None) => "missing",
                Err(e) => {
                    tracing::warn!(hash = %h, error = %e, "could not verify blob");
                    "unreachable"
                }
            };
            (h, status)
        })
        .buffer_unordered(4)
        .collect()
        .await;
    let bad = results.iter().filter(|(_, s)| *s != "ok").count();
    db::call(&state.pool, move |c| {
        let now = db::now();
        let tx = c.transaction()?;
        for (h, s) in &results {
            if *s == "unreachable" {
                continue;
            }
            tx.execute("UPDATE blobs SET status = ?2, verified_at = ?3 WHERE hash = ?1", params![h, s, now])?;
        }
        tx.commit()?;
        Ok(())
    })
    .await?;
    tracing::info!(total, bad, "scrub complete");
    Ok(())
}

/// Delete blobs no asset or thumbnail points at. Returns (count, bytes).
pub(crate) async fn gc(state: &AppState) -> Result<(u64, u64)> {
    let orphans: Vec<(String, i64)> = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT hash, size FROM blobs b
             WHERE NOT EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = b.hash)
               AND NOT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash = b.hash)
               AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.blob_hash = b.hash)
               AND NOT EXISTS (SELECT 1 FROM asset_origin o WHERE o.blob_hash = b.hash)",
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    let (mut count, mut bytes) = (0u64, 0u64);
    for (hash, size) in orphans {
        let h = hash.clone();
        // Drop the row first, re-checking references under the write, and only then the bytes.
        let deleted = db::call(&state.pool, move |c| {
            Ok(c.execute(
                "DELETE FROM blobs WHERE hash = ?1
                   AND NOT EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = ?1 OR a.thumb_hash = ?1)
                   AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.blob_hash = ?1)
                   AND NOT EXISTS (SELECT 1 FROM asset_origin o WHERE o.blob_hash = ?1)",
                [&h],
            )?)
        })
        .await?;
        if deleted > 0 {
            state.blobs.delete(&hash).await?;
            count += 1;
            bytes += size as u64;
        }
    }
    Ok((count, bytes))
}
