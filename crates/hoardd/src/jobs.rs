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
pub(crate) const CONVERT: &str = "convert";
pub(crate) const UNREAL: &str = "unreal_convert";
pub(crate) const SCAN: &str = "scan_location";
pub(crate) const RELOCATE: &str = "relocate";
pub(crate) const LOOKS: &str = "looks";
pub(crate) const MATERIALS: &str = "materials";
/// Image search: embed thumbnails that have no embedding yet (see [`crate::clip`]).
pub(crate) const EMBED: &str = "embed";
/// Read the store's files in a pack again (a Megascans JSON): name, tags and description.
pub(crate) const READ_FILES: &str = "read_files";
/// Look up the next pack's store page (see [`crate::store_lookup`]).
pub(crate) const STORE_LOOKUP: &str = "store_lookup";

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
        CONVERT => format!("Converting {pack} with Blender"),
        UNREAL => format!("Converting Unreal packages in {pack}"),
        SCAN => "Scanning a watched folder".to_owned(),
        RELOCATE => "Moving files between library locations".to_owned(),
        LOOKS => "Working out colours and looks".to_owned(),
        EMBED => "Learning what pictures show".to_owned(),
        MATERIALS => format!("Finding materials in {pack}"),
        READ_FILES => format!("Reading the files of {pack}"),
        STORE_LOOKUP => "Looking up a store page".to_owned(),
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
        CONVERT => CONVERT,
        UNREAL => UNREAL,
        SCAN => SCAN,
        RELOCATE => RELOCATE,
        LOOKS => LOOKS,
        EMBED => EMBED,
        MATERIALS => MATERIALS,
        READ_FILES => READ_FILES,
        STORE_LOOKUP => STORE_LOOKUP,
        _ => "job",
    };
    let activity = crate::activity::start(
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
                crate::store_lookup::start(state).await;
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
        CONVERT => crate::convert::convert_pack(state, &pack_id).await,
        UNREAL => {
            let req = serde_json::from_value(job.payload.clone()).unwrap_or_default();
            crate::extract::convert_pack(state, &pack_id, req).await.map(|_| ())
        }
        SCAN => match job.payload.get("location").and_then(serde_json::Value::as_i64) {
            Some(loc) => blocking(state, move |s, _| crate::scan::scan(s, loc).map(|_| ())).await,
            None => Err(anyhow::anyhow!("scan job without a location")),
        },
        RELOCATE => crate::relocate::run(state, serde_json::from_value(job.payload.clone()).unwrap_or_default()).await,
        LOOKS => crate::looks::run_job(state).await,
        EMBED => crate::clip::run_job(state).await,
        MATERIALS => blocking(state, move |s, rt| crate::materials::run_job(s, rt, &pack_id)).await,
        READ_FILES => read_files(state, &pack_id).await,
        STORE_LOOKUP => crate::store_lookup::run(state, &activity).await,
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
    // New or changed thumbnails get their colours and look worked out.
    if result.is_ok() && matches!(job.kind.as_str(), INGEST | RETAG | CONVERT | UNREAL | SCAN | MATERIALS) {
        crate::looks::schedule(state).await;
    }
}

/// Read the store's files in a pack again. What they say may find its store page now, so a pack without one is
/// looked up again.
async fn read_files(state: &AppState, pack_id: &str) -> Result<()> {
    crate::sidecar::apply(state, pack_id, false, None).await?;
    let pid = pack_id.to_owned();
    db::call(&state.pool, move |c| {
        c.execute(
            "UPDATE pack_context SET store_lookup_at = NULL WHERE pack_id = ?1 AND COALESCE(store_url, '') = ''",
            [pid],
        )?;
        Ok(())
    })
    .await?;
    crate::store_lookup::start(state).await;
    Ok(())
}

/// At startup: read the store's files of packs that have one but were added before Hoard read them.
pub(crate) async fn read_unread_files(state: &AppState) -> Result<()> {
    let n = db::call(&state.pool, |c| {
        let ids: Vec<String> = {
            let (filter, patterns) = crate::sidecar::path_filter();
            let mut stmt = c.prepare(&format!(
                "SELECT p.id FROM packs p
                 WHERE p.description IS NULL AND p.files_read_at IS NULL AND p.status = 'ready'
                   AND EXISTS (SELECT 1 FROM assets a JOIN blobs b ON b.hash = a.blob_hash
                               WHERE a.pack_id = p.id AND b.size <= ? AND {filter})"
            ))?;
            let mut args: Vec<rusqlite::types::Value> = vec![crate::sidecar::SIDECAR_BYTES.into()];
            args.extend(patterns.into_iter().map(Into::into));
            stmt.query_map(rusqlite::params_from_iter(args), |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
        };
        let tx = c.transaction()?;
        let mut n = 0;
        for id in &ids {
            if !db::job_pending(&tx, READ_FILES, Some(id), None)? {
                db::enqueue(&tx, READ_FILES, Some(id), &serde_json::json!({}))?;
                n += 1;
            }
        }
        tx.commit()?;
        Ok(n)
    })
    .await?;
    if n > 0 {
        tracing::info!(packs = n, "reading the store files of packs added before Hoard read them");
        state.wake.notify_one();
    }
    Ok(())
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

/// What a scrub found.
pub(crate) struct Scrubbed {
    pub(crate) total: usize,
    pub(crate) corrupt: usize,
    pub(crate) missing: usize,
    pub(crate) unreachable: usize,
}

/// Re-hash every blob in storage and record the result. Detects bit rot and missing files on the NAS.
/// `progress` is told (checked, total) as it goes.
pub(crate) async fn scrub(state: &AppState, progress: &(dyn Fn(usize, usize) + Sync)) -> Result<Scrubbed> {
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
            progress(n, total);
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
    let count = |status: &str| results.iter().filter(|(_, s)| *s == status).count();
    let found =
        Scrubbed { total, corrupt: count("corrupt"), missing: count("missing"), unreachable: count("unreachable") };
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
    tracing::info!(
        total,
        corrupt = found.corrupt,
        missing = found.missing,
        unreachable = found.unreachable,
        "scrub complete"
    );
    Ok(found)
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
    delete_unused(state, orphans).await
}

/// Delete these blobs (hash, size) where nothing points at them any more. Returns (count, bytes).
pub(crate) async fn delete_unused(state: &AppState, blobs: Vec<(String, i64)>) -> Result<(u64, u64)> {
    let (mut count, mut bytes) = (0u64, 0u64);
    for (hash, size) in blobs {
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
            bytes += u64::try_from(size).unwrap_or(0);
        }
    }
    Ok((count, bytes))
}
