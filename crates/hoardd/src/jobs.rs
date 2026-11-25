//! Durable background job queue in SQLite. Jobs survive restarts; anything left `running` after a crash
//! is re-queued at startup.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex, PoisonError};
use std::time::Duration;

use anyhow::Result;
use futures_util::{FutureExt, StreamExt, stream};
use hoard_core::PackStatus;
use rusqlite::params;
use tracing::Instrument;

use crate::AppState;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{db, ingest, llm};
use axum::http::StatusCode;

pub(crate) const INGEST: &str = "ingest";
pub(crate) const RETAG: &str = "retag";
pub(crate) const LLM_TAG: &str = "llm_tag";
pub(crate) const CONVERT: &str = "convert";
pub(crate) const UNREAL: &str = "unreal_convert";
/// Convert the files the conversion rules match in a pack (see [`crate::reformat`]).
pub(crate) const REFORMAT: &str = "reformat";
pub(crate) const SCAN: &str = "scan_location";
pub(crate) const RELOCATE: &str = "relocate";
pub(crate) const LOOKS: &str = "looks";
/// "More like this" for sounds: listen to sounds that haven't been (see [`crate::sounds`]).
pub(crate) const SOUNDS: &str = "sounds";
pub(crate) const MATERIALS: &str = "materials";
/// Image search: embed thumbnails that have no embedding yet (see [`crate::clip`]).
pub(crate) const EMBED: &str = "embed";
/// Sound search: embed sounds that have no embedding yet (see [`crate::clap`]).
pub(crate) const HEAR: &str = "hear";
/// Read the store's files in a pack again (a Megascans JSON): name, tags and description.
pub(crate) const READ_FILES: &str = "read_files";
/// Look up the next pack's store page (see [`crate::store_lookup`]).
pub(crate) const STORE_LOOKUP: &str = "store_lookup";

/// Work someone is waiting on, run before anything else: a pack being read, a watched folder looked over, the library
/// moved.
pub(crate) const URGENT: &[&str] = &[INGEST, SCAN, RELOCATE];
/// Work nobody is waiting on, run after everything else: the AI's tags, search, "more like this", store pages. It
/// never has every worker at once, and stops partway when asked (see [`stop`]).
pub(crate) const BACKGROUND: &[&str] = &[LLM_TAG, LOOKS, SOUNDS, EMBED, HEAR, STORE_LOOKUP];

/// The running background jobs, by id, with what stops each.
static STOPPERS: LazyLock<Mutex<HashMap<i64, Arc<tokio::sync::Notify>>>> = LazyLock::new(Default::default);

fn stoppers<R>(f: impl FnOnce(&mut HashMap<i64, Arc<tokio::sync::Notify>>) -> R) -> R {
    f(&mut STOPPERS.lock().unwrap_or_else(PoisonError::into_inner))
}

/// With the backoff below, the last try comes about 15 minutes after the first: long enough for a local model
/// that's still downloading on the first start of the compose stack.
const MAX_LLM_ATTEMPTS: i64 = 5;

/// Starts as many workers as Settings can ask for; the ones past the current count stay idle (checking every couple
/// of seconds, and never taking a wake-up meant for a working one) so the count can change without a restart.
pub(crate) fn spawn_workers(state: &AppState) {
    for n in 0..crate::settings::MAX_WORKERS {
        let state = Arc::clone(state);
        tokio::spawn(async move {
            tracing::debug!(worker = n, "job worker started");
            loop {
                if n >= state.settings.get().workers {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
                match next_job(&state).await {
                    Ok(Some(job)) => {
                        // Everything the job logs says which it is, its database writes included (see
                        // `db::began_writing`).
                        let span = tracing::info_span!("job", id = job.id, kind = %job.kind, pack = job.pack_id.as_deref().unwrap_or(""));
                        run(&state, job).instrument(span).await;
                    }
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
    // With more than one worker, one is kept from background work.
    let slots = state.settings.get().workers.saturating_sub(1).max(1);
    db::call(&state.pool, move |c| db::claim_job(c, slots)).await
}

/// How a job reads in the activity list.
pub(crate) fn job_label(kind: &str, pack: Option<&str>) -> String {
    let pack = pack.map_or_else(|| tr!("server-job-a-pack"), str::to_owned);
    let pack = pack.as_str();
    match kind {
        INGEST => tr!("server-job-ingest", pack = pack),
        RETAG => tr!("server-job-retag", pack = pack),
        LLM_TAG => tr!("server-job-llm-tag", pack = pack),
        CONVERT => tr!("server-job-convert", pack = pack),
        UNREAL => tr!("server-job-unreal", pack = pack),
        REFORMAT => tr!("server-job-reformat", pack = pack),
        SCAN => tr!("server-job-scan"),
        RELOCATE => tr!("server-job-relocate"),
        LOOKS => tr!("server-job-looks"),
        SOUNDS => tr!("server-job-sounds"),
        EMBED => tr!("server-job-embed"),
        HEAR => tr!("server-job-hear"),
        MATERIALS => tr!("server-job-materials", pack = pack),
        READ_FILES => tr!("server-job-read-files", pack = pack),
        STORE_LOOKUP => tr!("server-job-store-lookup"),
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
    let kind = static_kind(&job.kind);
    let activity = crate::activity::start(kind, job_label(&job.kind, pack_name.as_deref()), job.pack_id.as_deref());
    let stopper = BACKGROUND.contains(&kind).then(|| {
        let n = Arc::new(tokio::sync::Notify::new());
        stoppers(|m| m.insert(job.id, Arc::clone(&n)));
        n
    });
    activity.stop_at(stopper.is_some().then(|| stop_path(job.id)));

    // A job that panics is a job failed, not a worker gone with its job left running.
    let work = async {
        match job.kind.as_str() {
            INGEST => {
                let auto_name = job.payload.get("auto_name").and_then(serde_json::Value::as_bool).unwrap_or(false);
                // An upload going into a pack the library had already: its files are in that upload's staging folder.
                let from = job.payload.get("from").and_then(serde_json::Value::as_str).map(str::to_owned);
                let pid = pack_id.clone();
                let done =
                    blocking(state, move |s, rt| ingest::ingest_pack(s, rt, &pid, auto_name, from.as_deref())).await;
                if done.is_ok() {
                    crate::context::fill_store_text(state, &pack_id).await;
                    crate::store_lookup::start(state).await;
                }
                done
            }
            RETAG => {
                let then_llm = job.payload.get("then_llm").and_then(serde_json::Value::as_bool).unwrap_or(false);
                // Just some of the pack's assets: files changed where they sit in a watched folder.
                let only: Option<HashSet<String>> =
                    job.payload.get("assets").and_then(|a| serde_json::from_value(a.clone()).ok());
                blocking(state, move |s, rt| {
                    ingest::retag_pack(s, rt, &pack_id, only.as_ref())?;
                    let conn = s.pool.get()?;
                    if only.is_some() {
                        // A changed model is converted again.
                        crate::convert::maybe_enqueue(s, &conn, &pack_id)?;
                    }
                    ingest::finish_rules_stage(s, &conn, &pack_id, then_llm)
                })
                .await
            }
            LLM_TAG => llm::tag_pack(state, &pack_id).await,
            CONVERT => crate::convert::convert_pack(state, &pack_id).await,
            UNREAL => {
                let req = serde_json::from_value(job.payload.clone()).unwrap_or_default();
                crate::extract::convert_pack(state, &pack_id, req).await.map(|_| ())
            }
            REFORMAT => {
                let req = serde_json::from_value(job.payload.clone()).unwrap_or_default();
                crate::reformat::run_job(state, &pack_id, req).await
            }
            SCAN => match job.payload.get("location").and_then(serde_json::Value::as_i64) {
                Some(loc) => {
                    let full = job.payload.get("full").and_then(serde_json::Value::as_bool).unwrap_or(true);
                    blocking(state, move |s, _| crate::scan::scan(s, loc, full).map(|_| ())).await
                }
                None => Err(anyhow::anyhow!("scan job without a location")),
            },
            RELOCATE => {
                crate::relocate::run(state, serde_json::from_value(job.payload.clone()).unwrap_or_default()).await
            }
            LOOKS => crate::looks::run_job(state).await,
            SOUNDS => crate::sounds::run_job(state).await,
            EMBED => crate::clip::run_job(state).await,
            HEAR => crate::clap::run_job(state).await,
            MATERIALS => blocking(state, move |s, rt| crate::materials::run_job(s, rt, &pack_id)).await,
            READ_FILES => read_files(state, &pack_id).await,
            STORE_LOOKUP => crate::store_lookup::run(state, &activity).await,
            other => Err(anyhow::anyhow!("unknown job kind {other}")),
        }
    };
    let work = async {
        std::panic::AssertUnwindSafe(work)
            .catch_unwind()
            .await
            .unwrap_or_else(|_| Err(anyhow::anyhow!("the {} job panicked", job.kind)))
    };
    // A background job asked to stop is let go at its next wait: what it has written stays, the rest is left undone.
    let result: Option<Result<()>> = match &stopper {
        Some(n) => tokio::select! {
            r = work => Some(r),
            () = n.notified() => None,
        },
        None => Some(work.await),
    };
    stoppers(|m| m.remove(&job.id));
    let Some(result) = result else {
        forget_stopped(state, &job).await;
        return;
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
                // A hosted service that said no (a wrong key, an outage past its retries) is asked again in an hour.
                EMBED if crate::clip::is_hosted() => (Some(msg), Some(3600), None),
                _ => (Some(msg), None, None),
            }
        }
    };
    let id = job.id;
    let pack_id = job.pack_id.clone();
    let kind = job.kind.clone();
    let res = db::call(&state.pool, move |c| {
        db::finish_job(c, id, error.as_deref(), retry)?;
        match (pack_id, pack_update) {
            (Some(pid), Some((status, msg))) => db::set_pack_status(c, &pid, status, Some(&msg))?,
            // Nothing left to finish its pack (this was the last step, and it failed, or forgot to): it's ready.
            (Some(pid), None) if retry.is_none() => {
                let note = error.as_deref().map(|e| format!("{kind} stopped: {e}"));
                if db::settle_packs(c, Some(&pid), note.as_deref())? > 0 {
                    tracing::warn!(pack = %pid, kind = %kind, "a job left its pack unfinished; it's ready as it is");
                }
            }
            _ => {}
        }
        Ok(())
    })
    .await;
    if let Err(e) = res {
        tracing::error!(error = ?e, "could not record job result");
    }
    tracing::info!(id = job.id, kind = %job.kind, ok = result.is_ok(), secs = started.elapsed().as_secs_f32(), "job finished");
    // New or changed thumbnails get their colors and look worked out.
    if result.is_ok() && matches!(job.kind.as_str(), INGEST | RETAG | CONVERT | UNREAL | REFORMAT | SCAN | MATERIALS) {
        crate::looks::schedule(state).await;
    }
}

/// A job's kind as one of the constants, for the activity list.
fn static_kind(kind: &str) -> &'static str {
    [
        INGEST,
        RETAG,
        LLM_TAG,
        CONVERT,
        UNREAL,
        REFORMAT,
        SCAN,
        RELOCATE,
        LOOKS,
        SOUNDS,
        EMBED,
        HEAR,
        MATERIALS,
        READ_FILES,
        STORE_LOOKUP,
    ]
    .into_iter()
    .find(|k| *k == kind)
    .unwrap_or("job")
}

/// A job stopped partway is gone from the queue, and its pack is ready with what it got.
async fn forget_stopped(state: &AppState, job: &db::JobRow) {
    tracing::info!(id = job.id, kind = %job.kind, pack = ?job.pack_id, "job stopped");
    let (id, pack) = (job.id, job.pack_id.clone());
    let res = db::call(&state.pool, move |c| {
        c.execute("DELETE FROM jobs WHERE id = ?1", [id])?;
        if let Some(p) = pack {
            db::settle_packs(c, Some(&p), None)?;
        }
        Ok(())
    })
    .await;
    if let Err(e) = res {
        tracing::error!(error = ?e, "could not record a stopped job");
    }
}

/// Stopping background work: one job, every job of a kind, or a Maintenance task. An admin's to do.
pub(crate) fn operations() -> Vec<crate::catalog::Operation> {
    use crate::catalog::Access::Admin;
    vec![
        Admin.post("/api/admin/jobs/stop", stop_kind_route).named(
            "stop_jobs_of_kind",
            "Stop every job of a kind (`{\"kind\": \"llm_tag\"}`, a kind `list_jobs` shows), waiting or running; \
             answers with how many.",
        ),
        Admin.post("/api/admin/jobs/{id}/stop", stop_route).named(
            "stop_job",
            "Stop one job: one waiting is taken off the queue (204), one running is asked to stop at its next wait (202).",
        ),
        Admin.post("/api/admin/maintenance/{task}/stop", crate::maintenance::stop_route).named(
            "stop_maintenance",
            "Ask a running Maintenance task to stop after what it's doing now.",
        ),
    ]
}

pub(crate) fn stop_path(id: i64) -> String {
    format!("/api/admin/jobs/{id}/stop")
}

/// Stop a job: one waiting is taken off the queue, and a background one running is let go at its next wait. Reading a
/// pack isn't stopped, waiting or not, since its files wait in staging: deleting the pack is what does that.
pub(crate) async fn stop(state: &AppState, id: i64) -> ApiResult<StatusCode> {
    if let Some(n) = stoppers(|m| m.get(&id).cloned()) {
        n.notify_one();
        return Ok(StatusCode::ACCEPTED);
    }
    let found: Option<(String, String)> = db::call(&state.pool, move |c| {
        use rusqlite::OptionalExtension as _;
        let found: Option<(String, String, Option<String>)> = c
            .query_row("SELECT kind, status, pack_id FROM jobs WHERE id = ?1", [id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .optional()?;
        let Some((kind, status, pack)) = found else { return Ok(None) };
        if status == "queued" && kind != INGEST {
            c.execute("DELETE FROM jobs WHERE id = ?1 AND status = 'queued'", [id])?;
            if let Some(p) = pack {
                db::settle_packs(c, Some(&p), None)?;
            }
            return Ok(Some((kind, "stopped".to_owned())));
        }
        Ok(Some((kind, status)))
    })
    .await?;
    match found {
        Some((_, status)) if status == "stopped" => Ok(StatusCode::NO_CONTENT),
        Some((kind, status)) if status == "queued" || status == "running" => {
            Err(AppError::conflict(tr!("error-job-cant-stop", job = job_label(&kind, None))))
        }
        _ => Err(AppError::not_found(tr!("error-job-not-found"))),
    }
}

/// Stop every job of one kind, waiting or running (as [`stop`] would each): for after a big import, to call off the
/// AI tagging it queued. How many were stopped.
pub(crate) async fn stop_kind(state: &AppState, kind: String) -> Result<usize> {
    if kind == INGEST {
        return Ok(0);
    }
    let (queued, running): (usize, Vec<i64>) = db::call(&state.pool, move |c| {
        let packs: Vec<Option<String>> = c
            .prepare("DELETE FROM jobs WHERE kind = ?1 AND status = 'queued' RETURNING pack_id")?
            .query_map([&kind], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        for p in packs.iter().flatten() {
            db::settle_packs(c, Some(p), None)?;
        }
        let running = c
            .prepare("SELECT id FROM jobs WHERE kind = ?1 AND status = 'running'")?
            .query_map([&kind], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<i64>>>()?;
        Ok((packs.len(), running))
    })
    .await?;
    let stopped = running.into_iter().filter_map(|id| stoppers(|m| m.get(&id).cloned())).inspect(|n| n.notify_one());
    Ok(queued + stopped.count())
}

/// `POST /api/admin/jobs/{id}/stop`: 204 when a waiting job is gone, 202 when a running one has been asked to stop.
pub(crate) async fn stop_route(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> ApiResult<StatusCode> {
    stop(&state, id).await
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct StopKind {
    kind: String,
}

/// `POST /api/admin/jobs/stop` with `{"kind": "llm_tag"}`: stop every job of that kind, `{"stopped": n}`.
pub(crate) async fn stop_kind_route(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(body): axum::Json<StopKind>,
) -> ApiResult<axum::Json<serde_json::Value>> {
    let stopped = stop_kind(&state, body.kind).await?;
    state.wake.notify_waiters();
    Ok(axum::Json(serde_json::json!({ "stopped": stopped })))
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
    let state = Arc::clone(state);
    let rt = tokio::runtime::Handle::current();
    crate::heavy::run("job", move || f(&state, &rt)).await?
}

/// What a scrub found.
pub(crate) struct Scrubbed {
    /// Files checked, of how many there are (fewer when it was stopped).
    pub(crate) total: usize,
    pub(crate) of: usize,
    pub(crate) corrupt: usize,
    pub(crate) missing: usize,
    pub(crate) unreachable: usize,
}

/// Re-hash every blob in storage and record the result. Detects bit rot and missing files on the NAS.
/// `progress` is told (checked, total) as it goes.
pub(crate) async fn scrub(
    state: &AppState,
    progress: &(dyn Fn(usize, usize) + Sync),
    stopped: &(dyn Fn() -> bool + Sync),
) -> Result<Scrubbed> {
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
            // Asked to stop: what's left isn't checked, or marked.
            if stopped() {
                return (h, "skipped");
            }
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
    let found = Scrubbed {
        total: total - count("skipped"),
        of: total,
        corrupt: count("corrupt"),
        missing: count("missing"),
        unreachable: count("unreachable"),
    };
    db::call(&state.pool, move |c| {
        let now = db::now();
        let tx = c.transaction()?;
        for (h, s) in &results {
            if *s == "unreachable" || *s == "skipped" {
                continue;
            }
            tx.prepare_cached("UPDATE blobs SET status = ?2, verified_at = ?3 WHERE hash = ?1")?
                .execute(params![h, s, now])?;
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
            Ok(c.prepare_cached(
                "DELETE FROM blobs WHERE hash = ?1
                   AND NOT EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = ?1 OR a.thumb_hash = ?1)
                   AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.blob_hash = ?1)
                   AND NOT EXISTS (SELECT 1 FROM asset_origin o WHERE o.blob_hash = ?1)",
            )?
            .execute([&h])?)
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
