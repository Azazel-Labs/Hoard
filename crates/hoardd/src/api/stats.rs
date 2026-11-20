//! The library's numbers, and what it has been doing.

use std::time::Duration;

use crate::error::ApiResult;
use crate::{AppState, db, jobs};
use axum::Json;
use axum::extract::State;
use hoard_core::Stats;
use rusqlite::Connection;
use serde_json::{Value, json};

/// What takes the space: these read every file the library has (most of a second, on a big one), so they're kept
/// and made again in the background once it changes. The counts are always current.
struct Sizes {
    stored_bytes: u64,
    logical_bytes: u64,
    duplicate_assets: u64,
    preview_bytes: u64,
    conversion_cache_bytes: u64,
}

static SIZES: db::Kept<Sizes> = db::Kept::new("the library's sizes", sizes, Duration::from_secs(10));

fn sizes(c: &Connection) -> anyhow::Result<Sizes> {
    let one = |sql: &str| -> anyhow::Result<u64> { Ok(c.query_row(sql, [], |r| r.get::<_, i64>(0))? as u64) };
    Ok(Sizes {
        stored_bytes: one("SELECT COALESCE(SUM(size), 0) FROM blobs")?,
        logical_bytes: one("SELECT COALESCE(SUM(b.size), 0) FROM assets a JOIN blobs b ON b.hash = a.blob_hash")?,
        duplicate_assets: one(
            "SELECT COALESCE(SUM(n - 1), 0) FROM (SELECT COUNT(*) AS n FROM assets GROUP BY blob_hash HAVING n > 1)",
        )?,
        preview_bytes: one("SELECT COALESCE(SUM(size), 0) FROM blobs WHERE hash IN (
               SELECT thumb_hash FROM assets WHERE thumb_hash IS NOT NULL
               UNION SELECT blob_hash FROM derivatives WHERE kind IN ('views', 'preview', 'motion'))")?,
        conversion_cache_bytes: crate::formats::conversion_cache_bytes(c)?,
    })
}

pub(super) async fn stats(State(state): State<AppState>) -> ApiResult<Json<Stats>> {
    let llm_enabled = state.settings.get().llm_enabled();
    let conversion_cache_limit = state.settings.get().conversion_cache_bytes();
    let storage = state.blobs.label();
    let sizes = SIZES.get(&state.pool).await?.value;
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
            assets: kinds.iter().map(|k| k.count).sum(),
            packs: one("SELECT COUNT(*) FROM packs WHERE archived_at IS NULL")?,
            archived_packs: one("SELECT COUNT(*) FROM packs WHERE archived_at IS NOT NULL")?,
            archived_assets: one("SELECT COUNT(*) FROM assets a WHERE a.archived_at IS NOT NULL
                   AND a.pack_id NOT IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)")?,
            blobs: one("SELECT COUNT(*) FROM blobs")?,
            blobs_bad: one("SELECT COUNT(*) FROM blobs WHERE status != 'ok'")?,
            stored_bytes: sizes.stored_bytes,
            logical_bytes: sizes.logical_bytes,
            jobs_pending: one("SELECT COUNT(*) FROM jobs WHERE status IN ('queued', 'running')")?,
            llm_enabled,
            storage,
            kinds,
            duplicate_assets: sizes.duplicate_assets,
            version: hoard_core::VERSION.to_owned(),
            ffmpeg: crate::ffmpeg::describe(),
            blender: crate::blender::describe(),
            update: crate::updates::available(),
            renderer: crate::analyze::describe_renderer(),
            renderer_hint: crate::analyze::renderer_hint(),
            setup_needed: one("SELECT COUNT(*) FROM locations")? == 0,
            conversion_cache_bytes: sizes.conversion_cache_bytes,
            conversion_cache_limit,
            preview_bytes: sizes.preview_bytes,
        })
    })
    .await?;
    Ok(Json(s))
}

/// The library report, for Settings' Library page. Working it out reads the whole library (seconds, on a big one),
/// so it's kept: the page is answered at once with the one last made, and when the library has changed since, a new
/// one is made in the background; `refreshing` says so, and the page asks again for it.
static LIBRARY: db::Kept<Value> = db::Kept::new("the library report", library_report, Duration::from_secs(30));

/// How often what's kept is looked at in the background, and made again if the library has changed.
const EVERY: Duration = Duration::from_hours(1);

pub(super) async fn library_stats(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let got = LIBRARY.get(&state.pool).await?;
    let mut out = (*got.value).clone();
    if let Value::Object(m) = &mut out {
        m.insert("made_at".into(), json!(got.made_at));
        m.insert("refreshing".into(), json!(got.refreshing));
    }
    Ok(Json(out))
}

/// Make the reports soon after the server starts, so the first visit needn't wait, then once an hour when the library
/// has changed.
pub(crate) async fn keep_reports(state: AppState) {
    tokio::time::sleep(Duration::from_secs(20)).await;
    loop {
        if SIZES.changed() {
            SIZES.refresh_logged(&state.pool).await;
        }
        if LIBRARY.changed() {
            LIBRARY.refresh_logged(&state.pool).await;
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
