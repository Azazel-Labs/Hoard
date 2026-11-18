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
