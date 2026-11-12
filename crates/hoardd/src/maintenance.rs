//! Settings → Maintenance: the jobs that look after a library (back up, check for damage, re-analyze, clear the
//! cache, delete unused files), started by hand, with their progress while they run and a history of past runs.
//!
//! One of each runs at a time. A run that was going when the server stopped is marked as stopped at the next start.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::Duration;

use anyhow::{Context as _, Result};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use rusqlite::{Connection, params};
use serde::Serialize;
use serde_json::{Value, json};

use crate::error::{ApiResult, AppError};
use crate::{AppState, activity, db, jobs};

pub(crate) struct Task {
    pub(crate) id: &'static str,
    label: &'static str,
    blurb: &'static str,
}

pub(crate) const BACKUP: &str = "backup";
pub(crate) const SCRUB: &str = "scrub";
pub(crate) const REANALYZE: &str = "reanalyze";
pub(crate) const CLEAR_CACHE: &str = "clear_cache";
pub(crate) const SHRINK_THUMBS: &str = "shrink_thumbs";
pub(crate) const GC: &str = "gc";

pub(crate) const TASKS: &[Task] = &[
    Task {
        id: BACKUP,
        label: "Back up the database",
        blurb: "A consistent copy of the database (packs, tags, notes, collections and Settings) in the backups \
                folder next to it. The library's files aren't in it: they never change once stored, so any file sync \
                backs them up.",
    },
    Task {
        id: SCRUB,
        label: "Check files for damage",
        blurb: "Reads every stored file back and checks it against its fingerprint, to catch files that have rotted or \
                gone missing from a disk or NAS. Takes a while on a big library.",
    },
    Task {
        id: REANALYZE,
        label: "Re-analyze every pack",
        blurb: "Reads every file again with this version of Hoard: thumbnails, previews, details and rule tags are \
                made afresh. Your own edits stay; the AI isn't asked again.",
    },
    Task {
        id: CLEAR_CACHE,
        label: "Clear cached conversions",
        blurb: "Drops the files kept from “Download as” conversions (they're made again when next asked for) and \
                what interrupted work left behind.",
    },
    Task {
        id: SHRINK_THUMBS,
        label: "Shrink thumbnails",
        blurb: "Stores the thumbnails and model views kept as PNG the way Settings → Server → Thumbnails asks, which \
                usually makes them several times smaller. Ones with transparency, and pixel art, stay PNG.",
    },
    Task {
        id: GC,
        label: "Delete unused files",
        blurb: "Deletes stored files nothing points at any more, such as the files of packs deleted for good.",
    },
];

/// Runs kept in the history.
const KEEP_RUNS: i64 = 200;
/// Leftovers in the temporary folder older than this are from work that was interrupted.
const STALE_TMP: Duration = Duration::from_hours(24);

#[derive(Clone, Debug, Serialize)]
struct Live {
    run: i64,
    started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

static RUNNING: LazyLock<Mutex<HashMap<&'static str, Live>>> = LazyLock::new(Default::default);

fn running<R>(f: impl FnOnce(&mut HashMap<&'static str, Live>) -> R) -> R {
    f(&mut RUNNING.lock().unwrap_or_else(PoisonError::into_inner))
}

/// A running task's progress, shown in Maintenance and in the activity indicator.
struct Report {
    task: &'static str,
    activity: activity::Handle,
}

impl Report {
    fn progress(&self, done: usize, total: usize, (one, many): (&str, &str)) {
        #[expect(clippy::cast_precision_loss, reason = "a fraction for a progress bar")]
        let p = (total > 0).then(|| done as f32 / total as f32);
        let d = format!("{done} of {}", plural(total, one, many));
        if let Some(p) = p {
            self.activity.progress(p);
        }
        self.activity.detail(d.clone());
        running(|m| {
            if let Some(l) = m.get_mut(self.task) {
                l.progress = p;
                l.detail = Some(d);
            }
        });
    }
}

fn task(id: &str) -> Option<&'static Task> {
    TASKS.iter().find(|t| t.id == id)
}

/// Mark the runs a stopped server left unfinished.
pub(crate) fn mark_interrupted(c: &Connection) -> Result<()> {
    c.execute(
        "UPDATE maintenance_runs SET finished_at = ?1, ok = 0, summary = 'Stopped when the server stopped'
         WHERE finished_at IS NULL",
        [db::now()],
    )?;
    Ok(())
}

/// Claim the task and record the run's start.
async fn begin(state: &AppState, t: &'static Task) -> ApiResult<i64> {
    let started_at = db::now();
    let claimed = running(|m| {
        if m.contains_key(t.id) {
            return false;
        }
        m.insert(t.id, Live { run: 0, started_at: started_at.clone(), progress: None, detail: None });
        true
    });
    if !claimed {
        return Err(AppError::conflict(format!("“{}” is already running", t.label)));
    }
    let (id, at) = (t.id, started_at.clone());
    let run = db::call(&state.pool, move |c| {
        c.execute("INSERT INTO maintenance_runs (task, started_at) VALUES (?1, ?2)", params![id, at])?;
        let run = c.last_insert_rowid();
        c.execute("DELETE FROM maintenance_runs WHERE id <= ?1 - ?2", params![run, KEEP_RUNS])?;
        Ok(run)
    })
    .await;
    match run {
        Ok(run) => {
            running(|m| m.get_mut(t.id).map(|l| l.run = run));
            Ok(run)
        }
        Err(e) => {
            running(|m| m.remove(t.id));
            Err(e.into())
        }
    }
}

/// Do the task, record how it went and let it go.
async fn execute(state: &AppState, t: &'static Task, run: i64) -> Result<String> {
    let report = Report { task: t.id, activity: activity::start("maintenance", t.label, None) };
    let result = match t.id {
        BACKUP => backup(state).await,
        SCRUB => scrub(state, &report).await,
        REANALYZE => reanalyze(state, &report).await,
        CLEAR_CACHE => clear_cache(state).await,
        SHRINK_THUMBS => shrink_thumbs(state, &report).await,
        GC => gc(state).await,
        other => Err(anyhow::anyhow!("no maintenance task {other}")),
    };
    if t.id == SHRINK_THUMBS {
        crate::looks::schedule(state).await;
    }
    let (ok, summary) = match &result {
        Ok(s) => (true, s.clone()),
        Err(e) => (false, format!("{e:#}")),
    };
    tracing::info!(task = t.id, ok, summary = %summary, "maintenance finished");
    let recorded = db::call(&state.pool, move |c| {
        c.execute(
            "UPDATE maintenance_runs SET finished_at = ?2, ok = ?3, summary = ?4 WHERE id = ?1",
            params![run, db::now(), ok, summary],
        )?;
        Ok(())
    })
    .await;
    if let Err(e) = recorded {
        tracing::error!(error = ?e, "could not record a maintenance run");
    }
    running(|m| m.remove(t.id));
    result
}

/// Start a task in the background.
pub(crate) async fn start(state: &AppState, id: &str) -> ApiResult<i64> {
    let t = task(id).ok_or_else(|| AppError::not_found(format!("maintenance task {id}")))?;
    let run = begin(state, t).await?;
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        let _ = execute(&state, t, run).await;
    });
    Ok(run)
}

/// Run a task and wait for it, for the scriptable endpoints (`POST /api/admin/backup` from cron).
pub(crate) async fn run_now(state: &AppState, id: &str) -> ApiResult<String> {
    let t = task(id).ok_or_else(|| AppError::not_found(format!("maintenance task {id}")))?;
    let run = begin(state, t).await?;
    Ok(execute(state, t, run).await?)
}

#[derive(Debug, Serialize)]
struct Run {
    id: i64,
    task: String,
    started_at: String,
    finished_at: Option<String>,
    ok: Option<bool>,
    summary: Option<String>,
}

fn runs(c: &Connection, sql_where: &str, limit: i64) -> Result<Vec<Run>> {
    let mut stmt = c.prepare(&format!(
        "SELECT id, task, started_at, finished_at, ok, summary FROM maintenance_runs {sql_where} ORDER BY id DESC LIMIT ?1"
    ))?;
    let rows = stmt
        .query_map([limit], |r| {
            Ok(Run {
                id: r.get(0)?,
                task: r.get(1)?,
                started_at: r.get(2)?,
                finished_at: r.get(3)?,
                ok: r.get(4)?,
                summary: r.get(5)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(rows)
}

/// `GET /api/admin/maintenance`: the tasks, what's running, each one's last run, and the recent history.
pub(crate) async fn overview(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let history = db::call(&state.pool, |c| runs(c, "WHERE finished_at IS NOT NULL", 50)).await?;
    let live = running(|m| m.clone());
    let tasks: Vec<Value> = TASKS
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "label": t.label,
                "blurb": t.blurb,
                "running": live.get(t.id),
                "last": history.iter().find(|r| r.task == t.id),
            })
        })
        .collect();
    Ok(Json(json!({ "tasks": tasks, "history": history })))
}

/// `POST /api/admin/maintenance/{task}`: start a task.
pub(crate) async fn start_route(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let run = start(&state, &id).await?;
    Ok((StatusCode::ACCEPTED, Json(json!({ "run": run }))))
}

pub(crate) fn human_bytes(b: u64) -> String {
    const UNITS: [&str; 5] = ["bytes", "KB", "MB", "GB", "TB"];
    #[expect(clippy::cast_precision_loss, reason = "a size for people to read")]
    let mut v = b as f64;
    let mut u = 0;
    while v >= 1024.0 && u < UNITS.len() - 1 {
        v /= 1024.0;
        u += 1;
    }
    if u == 0 { format!("{b} bytes") } else { format!("{v:.1} {}", UNITS[u]) }
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

async fn backup(state: &AppState) -> Result<String> {
    let dir = state.cfg.backups_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let stamp = time::OffsetDateTime::now_utc()
        .format(time::macros::format_description!("[year][month][day]-[hour][minute][second]"))
        .context("timestamp")?;
    let path = dir.join(format!("hoard-{stamp}.db"));
    let p = path.to_string_lossy().into_owned();
    db::call(&state.pool, move |c| {
        c.execute("VACUUM INTO ?1", [&p])?;
        Ok(())
    })
    .await?;
    let size = tokio::fs::metadata(&path).await.map_or(0, |m| m.len());
    Ok(format!("Saved {} ({})", path.display(), human_bytes(size)))
}

async fn scrub(state: &AppState, report: &Report) -> Result<String> {
    let found = jobs::scrub(state, &|done, total| report.progress(done, total, ("file", "files"))).await?;
    let mut bad = Vec::new();
    if found.corrupt > 0 {
        bad.push(format!("{} damaged", found.corrupt));
    }
    if found.missing > 0 {
        bad.push(format!("{} missing", found.missing));
    }
    if found.unreachable > 0 {
        bad.push(format!("{} couldn't be read (a disk or bucket offline?)", found.unreachable));
    }
    Ok(if bad.is_empty() {
        format!("Checked {}: no damage", plural(found.total, "file", "files"))
    } else {
        format!("Checked {}: {}", plural(found.total, "file", "files"), bad.join(", "))
    })
}

/// Queue every ready pack for re-analysis and wait for the queue to get through them.
async fn reanalyze(state: &AppState, report: &Report) -> Result<String> {
    let ids: Vec<i64> = db::call(&state.pool, |c| {
        let packs: Vec<String> = c
            .prepare(
                "SELECT id FROM packs p WHERE status = 'ready' AND archived_at IS NULL
                   AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'))",
            )?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        let tx = c.unchecked_transaction()?;
        let mut ids = Vec::with_capacity(packs.len());
        for p in &packs {
            db::set_pack_status(&tx, p, hoard_core::PackStatus::Ingesting, None)?;
            ids.push(db::enqueue(&tx, jobs::RETAG, Some(p), &json!({ "then_llm": false }))?);
        }
        tx.commit()?;
        Ok(ids)
    })
    .await?;
    state.wake.notify_one();
    let total = ids.len();
    if total == 0 {
        return Ok("No packs to re-analyze".to_owned());
    }
    let list = serde_json::to_string(&ids)?;
    loop {
        let l = list.clone();
        let (left, failed): (usize, usize) = db::call(&state.pool, move |c| {
            Ok(c.query_row(
                "SELECT COALESCE(SUM(status IN ('queued', 'running')), 0), COALESCE(SUM(status = 'failed'), 0)
                 FROM jobs WHERE id IN (SELECT value FROM json_each(?1))",
                [l],
                |r| Ok((r.get::<_, i64>(0)?.try_into().unwrap_or(0), r.get::<_, i64>(1)?.try_into().unwrap_or(0))),
            )?)
        })
        .await?;
        report.progress(total - left, total, ("pack", "packs"));
        if left == 0 {
            return Ok(match failed {
                0 => format!("Re-analyzed {}", plural(total, "pack", "packs")),
                f => format!("Re-analyzed {}; {f} failed (see the packs page)", plural(total, "pack", "packs")),
            });
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

async fn clear_cache(state: &AppState) -> Result<String> {
    // "Download as" conversions are cached as derivatives named `as:<format>`.
    let cached: Vec<(String, i64)> = db::call(&state.pool, |c| {
        let rows = c
            .prepare(
                "SELECT DISTINCT d.blob_hash, b.size FROM derivatives d JOIN blobs b ON b.hash = d.blob_hash
                 WHERE d.kind LIKE 'as:%'",
            )?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        c.execute("DELETE FROM derivatives WHERE kind LIKE 'as:%'", [])?;
        Ok(rows)
    })
    .await?;
    let (conversions, mut freed) = jobs::delete_unused(state, cached).await?;

    let tmp = state.cfg.tmp_dir();
    let (leftovers, bytes) =
        tokio::task::spawn_blocking(move || remove_stale(&tmp)).await.context("clear leftovers")?;
    freed += bytes;
    Ok(if conversions == 0 && leftovers == 0 {
        "Nothing to clear".to_owned()
    } else {
        format!(
            "Freed {}: {} and {}",
            human_bytes(freed),
            plural(usize::try_from(conversions).unwrap_or(usize::MAX), "conversion", "conversions"),
            plural(leftovers, "leftover", "leftovers"),
        )
    })
}

/// Re-encode the PNG thumbnails and model views with the current Thumbnails setting, keeping each new one only when
/// it's clearly smaller.
async fn shrink_thumbs(state: &AppState, report: &Report) -> Result<String> {
    // (hash, size, pixel art). A thumbnail shared by several assets is done once.
    let rows: Vec<(String, i64, bool)> = db::call(&state.pool, |c| {
        let rows = c
            .prepare(
                "SELECT b.hash, b.size, MAX(COALESCE(json_extract(a.meta, '$.\"pixel-art-likely\"'), 0))
                   FROM blobs b JOIN assets a ON a.thumb_hash = b.hash GROUP BY b.hash
                 UNION
                 SELECT b.hash, b.size, 0 FROM blobs b JOIN derivatives d ON d.blob_hash = b.hash
                  WHERE d.kind = 'views'",
            )?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, i64>(2)? != 0)))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    let total = rows.len();
    let (mut shrunk, mut saved) = (0usize, 0u64);
    for (i, (old, size, pixel_art)) in rows.into_iter().enumerate() {
        report.progress(i, total, ("thumbnail", "thumbnails"));
        if pixel_art {
            continue;
        }
        let Ok(bytes) = state.blobs.read_all(&old).await else { continue };
        if !bytes.starts_with(b"\x89PNG") {
            continue;
        }
        let smaller = crate::heavy::run("shrink-thumb", move || -> Result<Option<Vec<u8>>> {
            let img = image::load_from_memory(&bytes)?.to_rgba8();
            let out = crate::analyze::thumb_bytes(img, false)?;
            // A PNG made smaller by the stronger compression counts too.
            // Not worth swapping for less than a tenth.
            Ok((out.len() * 10 < bytes.len() * 9).then_some(out))
        })
        .await;
        let Ok(Ok(Some(out))) = smaller else { continue };
        let blob = state.blobs.put_cache_bytes(out).await?;
        let (o, n, new_size) = (old.clone(), blob.hash.clone(), i64::try_from(blob.size).unwrap_or(i64::MAX));
        db::call(&state.pool, move |c| {
            let tx = c.transaction()?;
            tx.execute(
                "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
                params![n, new_size, db::now()],
            )?;
            tx.execute("UPDATE assets SET thumb_hash = ?2 WHERE thumb_hash = ?1", [&o, &n])?;
            tx.execute("UPDATE derivatives SET blob_hash = ?2 WHERE blob_hash = ?1 AND kind = 'views'", [&o, &n])?;
            tx.commit()?;
            Ok(())
        })
        .await?;
        jobs::delete_unused(state, vec![(old, size)]).await?;
        shrunk += 1;
        saved += u64::try_from(size - new_size).unwrap_or(0);
    }
    report.progress(total, total, ("thumbnail", "thumbnails"));
    Ok(match shrunk {
        0 => "Nothing to shrink".to_owned(),
        n => format!("Shrank {}, saving {}", plural(n, "thumbnail", "thumbnails"), human_bytes(saved)),
    })
}

/// Remove what's in `dir` and hasn't been touched for a day. Returns (entries, bytes).
fn remove_stale(dir: &std::path::Path) -> (usize, u64) {
    let Ok(entries) = std::fs::read_dir(dir) else { return (0, 0) };
    let (mut n, mut bytes) = (0, 0);
    for e in entries.flatten() {
        let Ok(meta) = e.metadata() else { continue };
        if meta.modified().ok().and_then(|m| m.elapsed().ok()).is_none_or(|age| age < STALE_TMP) {
            continue;
        }
        let size = if meta.is_dir() { dir_size(&e.path()) } else { meta.len() };
        let gone = if meta.is_dir() { std::fs::remove_dir_all(e.path()) } else { std::fs::remove_file(e.path()) };
        if gone.is_ok() {
            n += 1;
            bytes += size;
        }
    }
    (n, bytes)
}

fn dir_size(dir: &std::path::Path) -> u64 {
    std::fs::read_dir(dir).map_or(0, |entries| {
        entries
            .flatten()
            .map(|e| match e.metadata() {
                Ok(m) if m.is_dir() => dir_size(&e.path()),
                Ok(m) => m.len(),
                Err(_) => 0,
            })
            .sum()
    })
}

async fn gc(state: &AppState) -> Result<String> {
    let (count, bytes) = jobs::gc(state).await?;
    Ok(if count == 0 {
        "Nothing unused".to_owned()
    } else {
        format!(
            "Deleted {} ({})",
            plural(usize::try_from(count).unwrap_or(usize::MAX), "file", "files"),
            human_bytes(bytes)
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_read_well() {
        assert_eq!(human_bytes(900), "900 bytes");
        assert_eq!(human_bytes(1536), "1.5 KB");
        assert_eq!(human_bytes(5 * 1024 * 1024 * 1024), "5.0 GB");
    }

    #[test]
    fn only_stale_leftovers_are_removed() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("fresh")).unwrap();
        std::fs::write(dir.path().join("fresh/a"), b"abc").unwrap();
        assert_eq!(remove_stale(dir.path()), (0, 0));
        assert!(dir.path().join("fresh/a").exists());
    }
}
