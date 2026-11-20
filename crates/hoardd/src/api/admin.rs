//! Maintenance: the job list, checking blobs, collecting garbage and backing up.

use crate::error::ApiResult;
use crate::{AppState, db};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::{Value, json};

pub(super) async fn admin_jobs(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let rows = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT id, kind, pack_id, status, attempts, error, created_at, updated_at FROM jobs ORDER BY id DESC LIMIT 50",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?, "kind": r.get::<_, String>(1)?, "pack_id": r.get::<_, Option<String>>(2)?,
                    "status": r.get::<_, String>(3)?, "attempts": r.get::<_, i64>(4)?,
                    "error": r.get::<_, Option<String>>(5)?, "created_at": r.get::<_, String>(6)?,
                    "updated_at": r.get::<_, String>(7)?,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!(rows)))
}

// The scriptable maintenance endpoints (for cron); they're recorded in Settings → Maintenance like a run from there.

pub(super) async fn admin_scrub(State(state): State<AppState>) -> ApiResult<StatusCode> {
    crate::maintenance::start(&state, crate::maintenance::SCRUB).await?;
    Ok(StatusCode::ACCEPTED)
}

pub(super) async fn admin_gc(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let summary = crate::maintenance::run_now(&state, crate::maintenance::GC).await?;
    Ok(Json(json!({ "summary": summary })))
}

/// Consistent snapshot of the metadata DB. Blobs are immutable, so back those up with any file/S3 sync.
pub(super) async fn admin_backup(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let summary = crate::maintenance::run_now(&state, crate::maintenance::BACKUP).await?;
    Ok(Json(json!({ "summary": summary })))
}

/// Settings → About's "Report a problem": what an issue on GitHub needs for the problem to be looked into (the build,
/// where it runs, what's turned on, and the last lines the server logged), with nothing private in it, so it can be
/// pasted as it is.
pub(super) async fn report(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    use crate::recent_log::scrub;
    let cfg = &state.cfg;
    let now = state.settings.get();
    let (assets, locations) = db::read(&state.pool, |c| {
        let assets: i64 =
            c.query_row(&format!("SELECT COUNT(*) FROM assets a WHERE {}", db::LIVE), [], |r| r.get(0))?;
        let mut stmt =
            c.prepare("SELECT COUNT(*), mode, path LIKE 's3://%' FROM locations GROUP BY 2, 3 ORDER BY 2, 3")?;
        let locations = stmt
            .query_map([], |r| {
                let s3 = if r.get::<_, bool>(2)? { " in S3 storage" } else { "" };
                Ok(format!("{} {}{s3}", r.get::<_, i64>(0)?, r.get::<_, String>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok((assets, locations))
    })
    .await?;
    let on = |b: bool| if b { "on" } else { "off" };
    let environment = format!(
        "{} on {} {}{}, SQLite {}",
        crate::edition(cfg),
        std::env::consts::OS,
        std::env::consts::ARCH,
        if std::path::Path::new("/.dockerenv").is_file() { " (Docker)" } else { "" },
        rusqlite::version(),
    );
    let facts: Vec<(&str, String)> = vec![
        ("Version", hoard_core::VERSION.to_owned()),
        ("Running as", environment.clone()),
        (
            "Library",
            format!(
                "{assets} assets; locations: {}",
                if locations.is_empty() { "none".into() } else { locations.join(", ") }
            ),
        ),
        ("Thumbnails render on", scrub(&crate::analyze::describe_renderer())),
        ("FFmpeg", scrub(&crate::ffmpeg::describe())),
        ("Blender", scrub(&crate::blender::describe())),
        ("Unreal Editor", scrub(&crate::unreal_editor::describe())),
        (
            "Tagging",
            match (now.llm_enabled(), now.llm_chatgpt) {
                (false, _) => "off".into(),
                (true, true) => "a ChatGPT plan".into(),
                (true, false) => "an OpenAI-compatible server".into(),
            },
        ),
        (
            "Image search",
            if !now.clip {
                "off".into()
            } else if now.clip_hosted.is_some() {
                "on, on another server".into()
            } else {
                "on".into()
            },
        ),
        ("Sound search", on(now.clap).into()),
        ("Model thumbnails", on(now.render).into()),
        ("Moving thumbnails", on(now.motion_thumbs).into()),
        ("Guest browsing", on(now.guest_browsing).into()),
        ("Workers", now.workers.to_string()),
    ];
    Ok(Json(json!({
        "version": hoard_core::VERSION,
        "environment": environment,
        "facts": facts.iter().map(|(name, value)| json!({ "name": name, "value": value })).collect::<Vec<_>>(),
        "log": crate::recent_log::lines(),
    })))
}
