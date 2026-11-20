//! The library's numbers, and what it has been doing.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use crate::error::ApiResult;
use crate::{AppState, db, jobs};
use axum::Json;
use axum::extract::State;
use hoard_core::Stats;
use rusqlite::Connection;
use serde_json::{Value, json};

pub(super) async fn stats(State(state): State<AppState>) -> ApiResult<Json<Stats>> {
    let llm_enabled = state.settings.get().llm_enabled();
    let conversion_cache_limit = state.settings.get().conversion_cache_bytes();
    let storage = state.blobs.label();
    let s = db::read(&state.pool, move |c| {
        let one = |sql: &str| -> anyhow::Result<u64> { Ok(c.query_row(sql, [], |r| r.get::<_, i64>(0))? as u64) };
        // Counts are of the library; the archive has its own two.
        let live = db::LIVE;
        let mut stmt =
            c.prepare(&format!("SELECT a.kind, COUNT(*) FROM assets a WHERE {live} GROUP BY a.kind ORDER BY 2 DESC"))?;
        let kinds = stmt
            .query_map([], |r| Ok(hoard_core::Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 }))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(Stats {
            assets: one(&format!("SELECT COUNT(*) FROM assets a WHERE {live}"))?,
            packs: one("SELECT COUNT(*) FROM packs WHERE archived_at IS NULL")?,
            archived_packs: one("SELECT COUNT(*) FROM packs WHERE archived_at IS NOT NULL")?,
            archived_assets: one(
                "SELECT COUNT(*) FROM assets a WHERE a.archived_at IS NOT NULL
                   AND a.pack_id NOT IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)",
            )?,
            blobs: one("SELECT COUNT(*) FROM blobs")?,
            blobs_bad: one("SELECT COUNT(*) FROM blobs WHERE status != 'ok'")?,
            stored_bytes: one("SELECT COALESCE(SUM(size), 0) FROM blobs")?,
            logical_bytes: one("SELECT COALESCE(SUM(b.size), 0) FROM assets a JOIN blobs b ON b.hash = a.blob_hash")?,
            jobs_pending: one("SELECT COUNT(*) FROM jobs WHERE status IN ('queued', 'running')")?,
            llm_enabled,
            storage,
            kinds,
            duplicate_assets: one(
                "SELECT COALESCE(SUM(n - 1), 0) FROM (SELECT COUNT(*) AS n FROM assets GROUP BY blob_hash HAVING n > 1)",
            )?,
            version: hoard_core::VERSION.to_owned(),
            ffmpeg: crate::ffmpeg::describe(),
            blender: crate::blender::describe(),
            update: crate::updates::available(),
            renderer: crate::analyze::describe_renderer(),
            renderer_hint: crate::analyze::renderer_hint(),
            setup_needed: one("SELECT COUNT(*) FROM locations")? == 0,
            conversion_cache_bytes: crate::formats::conversion_cache_bytes(c)?,
            conversion_cache_limit,
            preview_bytes: one(
                "SELECT COALESCE(SUM(size), 0) FROM blobs WHERE hash IN (
                   SELECT thumb_hash FROM assets WHERE thumb_hash IS NOT NULL
                   UNION SELECT blob_hash FROM derivatives WHERE kind IN ('views', 'preview', 'motion'))",
            )?,
        })
    })
    .await?;
    Ok(Json(s))
}

/// The library report as last worked out: what it said, when, and the database's write count then.
struct Report {
    value: Arc<Value>,
    made: Instant,
    made_at: String,
    writes: u64,
}

static REPORT: Mutex<Option<Report>> = Mutex::new(None);
static REFRESHING: AtomicBool = AtomicBool::new(false);

/// A report this new isn't worked out again when it's opened, whatever has changed since.
const FRESH_ENOUGH: Duration = Duration::from_secs(30);
/// How often the report is looked at in the background, and worked out again if the library has changed.
const EVERY: Duration = Duration::from_hours(1);

/// The library in numbers, for Settings' Library page. Working it out reads the whole library (seconds, on a big
/// one), so the page is answered at once with the report as last made, and when the library has changed since, a
/// new one is made in the background: `refreshing` says so, and the page asks again for it.
pub(super) async fn library_stats(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let kept = || {
        REPORT.lock().unwrap_or_else(PoisonError::into_inner).as_ref().map(|r| {
            (Arc::clone(&r.value), r.made_at.clone(), r.writes != db::writes() && r.made.elapsed() >= FRESH_ENOUGH)
        })
    };
    let (value, made_at, outdated) = if let Some(k) = kept() {
        k
    } else {
        // The first since the server started: there's nothing to show until it's made.
        refresh_report(&state).await?;
        kept().ok_or_else(|| anyhow::anyhow!("the library report couldn't be made"))?
    };
    if outdated && !REFRESHING.load(Ordering::Acquire) {
        let st = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = refresh_report(&st).await {
                tracing::warn!(error = %format!("{e:#}"), "could not make the library report");
            }
        });
    }
    let mut out = (*value).clone();
    if let Value::Object(m) = &mut out {
        m.insert("made_at".into(), json!(made_at));
        m.insert("refreshing".into(), json!(outdated || REFRESHING.load(Ordering::Acquire)));
    }
    Ok(Json(out))
}

/// Work the report out again and keep it, unless that's being done already.
async fn refresh_report(state: &AppState) -> anyhow::Result<()> {
    if REFRESHING.swap(true, Ordering::AcqRel) {
        // Made by whoever started it; the first visit waits for that one.
        while REFRESHING.load(Ordering::Acquire) {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        return Ok(());
    }
    let writes = db::writes();
    let made = db::call(&state.pool, |c| library_report(c)).await;
    if let Ok(value) = &made {
        *REPORT.lock().unwrap_or_else(PoisonError::into_inner) =
            Some(Report { value: Arc::new(value.clone()), made: Instant::now(), made_at: db::now(), writes });
    }
    REFRESHING.store(false, Ordering::Release);
    made.map(|_| ())
}

/// Make the library report soon after the server starts, so the first visit needn't wait, then once an hour when the
/// library has changed.
pub(crate) async fn keep_library_report(state: AppState) {
    tokio::time::sleep(Duration::from_secs(20)).await;
    loop {
        let changed =
            REPORT.lock().unwrap_or_else(PoisonError::into_inner).as_ref().is_none_or(|r| r.writes != db::writes());
        if changed && let Err(e) = refresh_report(&state).await {
            tracing::warn!(error = %format!("{e:#}"), "could not make the library report");
        }
        tokio::time::sleep(EVERY).await;
    }
}

/// What kinds and file types take the space, how tagged and described things are, the biggest packs and files, and
/// how it grew. Archived things aren't counted.
fn library_report(c: &Connection) -> anyhow::Result<Value> {
    let live = db::LIVE;
    let one = |sql: &str| -> anyhow::Result<u64> { Ok(c.query_row(sql, [], |r| r.get::<_, i64>(0))? as u64) };
    // Rows of (text, count, number), as JSON objects with the given keys for the text and the number.
    let rows = |sql: &str, key: &str, third: &str| -> anyhow::Result<Vec<Value>> {
        let mut stmt = c.prepare(sql)?;
        let out = stmt
            .query_map([], |r| {
                Ok(json!({
                    key: r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                    "count": r.get::<_, i64>(1)?,
                    third: r.get::<_, i64>(2)?,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(out)
    };
    let from = format!("FROM assets a JOIN blobs b ON b.hash = a.blob_hash WHERE {live}");
    let kinds = rows(
        &format!("SELECT a.kind, COUNT(*), SUM(b.size) {from} GROUP BY a.kind ORDER BY 3 DESC"),
        "value",
        "bytes",
    )?;
    let exts = rows(
        &format!("SELECT a.ext, COUNT(*), SUM(b.size) {from} GROUP BY a.ext ORDER BY 3 DESC LIMIT 12"),
        "value",
        "bytes",
    )?;
    let months = rows(
        &format!(
            "SELECT substr(a.created_at, 1, 7) AS m, COUNT(*), SUM(b.size) {from}
                   GROUP BY m ORDER BY m DESC LIMIT 12"
        ),
        "month",
        "bytes",
    )?;
    let tag_sources = rows(
        &format!(
            "SELECT t.source, COUNT(*), COUNT(DISTINCT t.tag) FROM asset_tags t
                   WHERE t.asset_id IN (SELECT a.id FROM assets a WHERE {live}) GROUP BY t.source ORDER BY 2 DESC"
        ),
        "value",
        "tags",
    )?;
    let mut stmt = c.prepare(&format!(
        "SELECT t.tag, COUNT(*) FROM asset_tags t WHERE t.asset_id IN (SELECT a.id FROM assets a WHERE {live})
               GROUP BY t.tag ORDER BY 2 DESC, 1 LIMIT 30"
    ))?;
    let top_tags = stmt
        .query_map([], |r| Ok(json!({ "value": r.get::<_, String>(0)?, "count": r.get::<_, i64>(1)? })))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut stmt = c.prepare(&format!(
        "SELECT p.id, p.name, COUNT(*), SUM(b.size) FROM assets a JOIN packs p ON p.id = a.pack_id
               JOIN blobs b ON b.hash = a.blob_hash WHERE {live} GROUP BY p.id ORDER BY 4 DESC LIMIT 8"
    ))?;
    let packs = stmt
        .query_map([], |r| {
            Ok(json!({
                "id": r.get::<_, String>(0)?, "name": r.get::<_, String>(1)?,
                "count": r.get::<_, i64>(2)?, "bytes": r.get::<_, i64>(3)?,
            }))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut stmt = c.prepare(&format!(
        "SELECT a.id, a.name, a.kind, p.name, b.size FROM assets a JOIN packs p ON p.id = a.pack_id
               JOIN blobs b ON b.hash = a.blob_hash WHERE {live} ORDER BY b.size DESC LIMIT 8"
    ))?;
    let largest = stmt
        .query_map([], |r| {
            Ok(json!({
                "id": r.get::<_, String>(0)?, "name": r.get::<_, String>(1)?, "kind": r.get::<_, String>(2)?,
                "pack_name": r.get::<_, String>(3)?, "bytes": r.get::<_, i64>(4)?,
            }))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(json!({
        "kinds": kinds,
        "extensions": exts,
        "months": months,
        "tag_sources": tag_sources,
        "top_tags": top_tags,
        "packs": packs,
        "largest": largest,
        "distinct_tags": one(&format!(
            "SELECT COUNT(DISTINCT tag) FROM asset_tags WHERE asset_id IN (SELECT a.id FROM assets a WHERE {live})"
        ))?,
        "untagged": one(&format!(
            "SELECT COUNT(*) FROM assets a WHERE {live} AND NOT EXISTS (SELECT 1 FROM asset_tags t WHERE t.asset_id = a.id)"
        ))?,
        "described": one(&format!(
            "SELECT COUNT(*) FROM assets a WHERE {live}
               AND COALESCE(a.description, a.source_description, a.ai_description) IS NOT NULL"
        ))?,
        "human_edited": one(&format!("SELECT COUNT(*) FROM assets a WHERE {live} AND a.human_edited = 1"))?,
    }))
}

/// What the server is doing in the background, for the portal's activity indicator: running work (with progress
/// where it's known), then jobs waiting their turn. `{"items": [{"kind", "label", "detail"?, "progress"?, "state",
/// "since", "pack_id"?}]}`.
pub(super) async fn activity(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let mut items = crate::activity::snapshot();
    let queued = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT j.id, j.kind, j.pack_id, p.name, j.created_at FROM jobs j LEFT JOIN packs p ON p.id = j.pack_id
             WHERE j.status = 'queued' ORDER BY j.id LIMIT 50",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, String>(4)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    for (id, kind, pack_id, pack, created) in queued {
        items.push(crate::activity::Entry {
            id: u64::MAX - id.unsigned_abs(),
            kind: "job",
            label: jobs::job_label(&kind, pack.as_deref()),
            detail: None,
            progress: None,
            state: "queued",
            since: created,
            pack_id,
        });
    }
    Ok(Json(json!({ "items": items })))
}
