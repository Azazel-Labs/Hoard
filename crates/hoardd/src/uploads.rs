//! Resumable bulk uploads, for imports measured in gigabytes.
//!
//! A session is a pack in the `uploading` state plus a list of expected files and sizes:
//!
//! 1. `POST /api/uploads` registers every file (`path`, `size`) up front and returns what's already received
//!    (zero for a new session).
//! 2. Big files go up in chunks: `PUT /api/uploads/{id}/chunk?path=…&offset=…`. The offset must equal what the server
//!    has; otherwise it answers 409 with the right offset, so a client resumes after any failure by asking.
//!    Chunks are capped below 100 MB so they also pass proxies with request-size limits.
//! 3. Small files go up many at a time: `POST /api/uploads/{id}/batch` (multipart, `relpath` + `file` pairs).
//! 4. `POST /api/uploads/{id}/complete` checks every file arrived and queues the normal ingest.
//!
//! `GET /api/uploads/{id}` reports per-file progress, and `DELETE` abandons a session.
//!
//! The server fingerprints each import's file list (paths + sizes). Registering the same list again resumes the open
//! session, from any browser or machine; registering a list that's already in the library is refused with 409 unless
//! `force` is set, so re-running an interrupted bulk import doesn't duplicate the packs that already finished.
//! What's on disk is the source of truth for progress, so a server restart mid-upload loses nothing.

use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Context;
use axum::extract::{DefaultBodyLimit, Multipart, Path, Query, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use futures_util::StreamExt;
use hoard_core::Pack;
use rusqlite::{OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::AsyncWriteExt;

use crate::error::{ApiResult, AppError};
use crate::ingest::safe_rel_path;
use crate::{AppState, db, jobs};

/// Largest single chunk or batch request. Below Cloudflare's 100 MB request cap, with room for multipart overhead.
pub(crate) const MAX_REQUEST_BYTES: usize = 96 << 20;
/// Sessions with no activity for this long are removed at startup.
const STALE_DAYS: i64 = 7;
/// A request body that sends nothing for this long is abandoned, releasing its file for a retry. Without this, a
/// client that vanished mid-chunk (closed laptop, dead wifi) could hold a file's write slot indefinitely.
const BODY_IDLE: std::time::Duration = std::time::Duration::from_secs(60);

fn stalled(path: &str) -> AppError {
    AppError {
        status: StatusCode::REQUEST_TIMEOUT,
        err: anyhow::anyhow!("upload of {path} stalled; retry from the server's offset"),
    }
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/uploads", post(create).layer(DefaultBodyLimit::max(64 << 20)))
        .route("/api/uploads/{id}", get(status).delete(abort))
        .route("/api/uploads/{id}/chunk", put(chunk).layer(DefaultBodyLimit::max(MAX_REQUEST_BYTES)))
        .route("/api/uploads/{id}/batch", post(batch).layer(DefaultBodyLimit::max(MAX_REQUEST_BYTES)))
        .route("/api/uploads/{id}/complete", post(complete))
}

#[derive(Debug, Deserialize)]
struct NewFile {
    path: String,
    size: u64,
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    source: Option<String>,
    files: Vec<NewFile>,
    /// Import even if a pack with exactly these files already exists.
    #[serde(default)]
    force: bool,
}

fn import_key(files: &[(String, u64)]) -> String {
    let mut lines: Vec<String> = files.iter().map(|(p, s)| format!("{p}\0{s}")).collect();
    lines.sort();
    blake3::hash(lines.join("\n").as_bytes()).to_hex()[..32].to_owned()
}

enum Existing {
    Open(String),
    Imported(Box<Pack>),
}

#[derive(Debug, Serialize)]
struct FileStatus {
    path: String,
    size: u64,
    received: u64,
}

#[derive(Debug, Serialize)]
struct SessionStatus {
    pack: Pack,
    files: Vec<FileStatus>,
    total_bytes: u64,
    received_bytes: u64,
    max_request_bytes: usize,
    /// True when this call picked up an existing session instead of starting one.
    resumed: bool,
}

fn staging(state: &AppState, id: &str) -> PathBuf {
    state.cfg.staging_dir().join(id)
}

async fn session_status(state: &AppState, id: &str) -> ApiResult<SessionStatus> {
    let id2 = id.to_owned();
    let found = db::call(&state.pool, move |c| {
        let Some(pack) = db::get_pack(c, &id2)? else { return Ok(None) };
        let mut stmt = c.prepare("SELECT path, size, received FROM upload_files WHERE pack_id = ?1 ORDER BY path")?;
        let files = stmt
            .query_map([&id2], |r| {
                Ok(FileStatus {
                    path: r.get(0)?,
                    size: r.get::<_, i64>(1)? as u64,
                    received: r.get::<_, i64>(2)? as u64,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(Some((pack, files)))
    })
    .await?;
    let (pack, files) = found.ok_or_else(|| AppError::not_found(format!("upload {id}")))?;
    let total_bytes = files.iter().map(|f| f.size).sum();
    let received_bytes = files.iter().map(|f| f.received).sum();
    Ok(SessionStatus { pack, files, total_bytes, received_bytes, max_request_bytes: MAX_REQUEST_BYTES, resumed: false })
}

async fn create(State(state): State<AppState>, Json(body): Json<CreateBody>) -> ApiResult<Response> {
    if body.files.is_empty() {
        return Err(AppError::bad_request("no files"));
    }
    let mut seen = HashSet::new();
    let mut files = Vec::with_capacity(body.files.len());
    for f in body.files {
        let Some(path) = safe_rel_path(&f.path) else { continue };
        if seen.insert(path.clone()) {
            files.push((path, f.size));
        }
    }
    let key = import_key(&files);
    let k2 = key.clone();
    let existing = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare(
            "SELECT p.id, p.status FROM import_keys k JOIN packs p ON p.id = k.pack_id
             WHERE k.import_key = ?1 AND p.status != 'failed' ORDER BY p.status = 'uploading' DESC, p.created_at DESC",
        )?;
        let rows: Vec<(String, String)> =
            stmt.query_map([&k2], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
        Ok(match rows.into_iter().next() {
            Some((id, status)) if status == "uploading" => Some(Existing::Open(id)),
            Some((id, _)) => db::get_pack(c, &id)?.map(|p| Existing::Imported(Box::new(p))),
            None => None,
        })
    })
    .await?;
    match existing {
        Some(Existing::Open(id)) => {
            let mut s = session_status(&state, &id).await?;
            s.resumed = true;
            return Ok((StatusCode::OK, Json(s)).into_response());
        }
        Some(Existing::Imported(pack)) if !body.force => {
            let msg = format!("already imported as \"{}\"", pack.name);
            return Ok((StatusCode::CONFLICT, Json(json!({ "error": msg, "pack": pack }))).into_response());
        }
        _ => {}
    }

    let id = uuid::Uuid::now_v7().to_string();
    tokio::fs::create_dir_all(staging(&state, &id)).await?;
    let paths: Vec<String> = files.iter().map(|(p, _)| p.clone()).collect();
    let auto_name = body.name.as_deref().is_none_or(|n| n.trim().is_empty());
    let name = body.name.filter(|n| !n.trim().is_empty()).unwrap_or_else(|| crate::api::default_pack_name(&paths));
    let id2 = id.clone();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        tx.execute(
            "INSERT INTO packs (id, name, status, source, files_total, created_at) VALUES (?1, ?2, 'uploading', ?3, ?4, ?5)",
            params![id2, name, body.source, files.len() as i64, db::now()],
        )?;
        tx.execute("INSERT INTO upload_meta (pack_id, auto_name, touched_at) VALUES (?1, ?2, ?3)", params![id2, auto_name, db::now()])?;
        tx.execute("INSERT INTO import_keys (import_key, pack_id) VALUES (?1, ?2)", params![key, id2])?;
        {
            let mut stmt = tx.prepare("INSERT INTO upload_files (pack_id, path, size, received) VALUES (?1, ?2, ?3, 0)")?;
            for (path, size) in &files {
                stmt.execute(params![id2, path, *size as i64])?;
            }
        }
        tx.commit()?;
        Ok(())
    })
    .await?;
    Ok((StatusCode::CREATED, Json(session_status(&state, &id).await?)).into_response())
}

async fn status(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<SessionStatus>> {
    Ok(Json(session_status(&state, &id).await?))
}

/// Look up an expected file of an open session. Returns (size, received).
async fn expected(state: &AppState, id: &str, path: &str) -> ApiResult<(u64, u64)> {
    let (id2, p2) = (id.to_owned(), path.to_owned());
    let row = db::call(&state.pool, move |c| {
        let status: Option<String> =
            c.query_row("SELECT status FROM packs WHERE id = ?1", [&id2], |r| r.get(0)).optional()?;
        if status.as_deref() != Some("uploading") {
            return Ok(Err(status.is_some()));
        }
        Ok(c.query_row(
            "SELECT size, received FROM upload_files WHERE pack_id = ?1 AND path = ?2",
            params![id2, p2],
            |r| Ok((r.get::<_, i64>(0)? as u64, r.get::<_, i64>(1)? as u64)),
        )
        .optional()?
        .ok_or(true))
    })
    .await?;
    match row {
        Ok(v) => Ok(v),
        Err(true) => Err(AppError::bad_request(format!("{path} is not part of upload {id}, or the upload is closed"))),
        Err(false) => Err(AppError::not_found(format!("upload {id}"))),
    }
}

async fn record(state: &AppState, id: &str, path: &str, received: u64, size: u64) -> ApiResult<()> {
    let (id2, p2) = (id.to_owned(), path.to_owned());
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        tx.execute(
            "UPDATE upload_files SET received = ?3 WHERE pack_id = ?1 AND path = ?2",
            params![id2, p2, received as i64],
        )?;
        tx.execute("UPDATE upload_meta SET touched_at = ?2 WHERE pack_id = ?1", params![id2, db::now()])?;
        if received == size {
            tx.execute(
                "UPDATE packs SET files_done = (SELECT COUNT(*) FROM upload_files WHERE pack_id = ?1 AND received = size)
                 WHERE id = ?1",
                [&id2],
            )?;
        }
        tx.commit()?;
        Ok(())
    })
    .await?;
    Ok(())
}

/// One request per file at a time: a second concurrent writer for the same file is turned away.
struct InFlight<'a> {
    state: &'a AppState,
    key: String,
}

impl<'a> InFlight<'a> {
    fn claim(state: &'a AppState, id: &str, path: &str) -> ApiResult<Self> {
        let key = format!("{id}\0{path}");
        let mut set = state.uploads_in_flight.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !set.insert(key.clone()) {
            return Err(AppError {
                status: StatusCode::CONFLICT,
                err: anyhow::anyhow!("{path} is already being written"),
            });
        }
        Ok(Self { state, key })
    }
}

impl Drop for InFlight<'_> {
    fn drop(&mut self) {
        self.state.uploads_in_flight.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&self.key);
    }
}

#[derive(Debug, Deserialize)]
struct ChunkParams {
    path: String,
    offset: u64,
}

async fn chunk(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<ChunkParams>,
    req: Request,
) -> ApiResult<Response> {
    let path = safe_rel_path(&p.path).ok_or_else(|| AppError::bad_request("bad path"))?;
    let (size, _) = expected(&state, &id, &path).await?;
    let _guard = InFlight::claim(&state, &id, &path)?;
    let dest = staging(&state, &id).join(&path);
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = tokio::fs::OpenOptions::new().create(true).append(true).open(&dest).await?;
    // The bytes on disk are the truth (they survive restarts; the DB row may lag by one chunk).
    let have = file.metadata().await?.len();
    if p.offset != have {
        record(&state, &id, &path, have, size).await?;
        return Ok(
            (StatusCode::CONFLICT, Json(json!({ "error": "offset mismatch", "received": have }))).into_response()
        );
    }
    let mut written = have;
    let mut body = req.into_body().into_data_stream();
    loop {
        let next = tokio::time::timeout(BODY_IDLE, body.next()).await;
        let Ok(next) = next else {
            file.flush().await?;
            record(&state, &id, &path, written, size).await?;
            return Err(stalled(&path));
        };
        let Some(bytes) = next else { break };
        let bytes = match bytes {
            Ok(b) => b,
            Err(e) => {
                // Keep what arrived; the client resumes from here.
                file.flush().await?;
                record(&state, &id, &path, written, size).await?;
                return Err(AppError::bad_request(format!("upload interrupted: {e}")));
            }
        };
        if written + bytes.len() as u64 > size {
            // Keep what fits so the client can't corrupt the file by overrunning it.
            file.flush().await?;
            file.set_len(have).await?;
            return Err(AppError::bad_request(format!("chunk runs past the declared size of {path}")));
        }
        file.write_all(&bytes).await?;
        written += bytes.len() as u64;
    }
    file.flush().await?;
    record(&state, &id, &path, written, size).await?;
    Ok(Json(json!({ "received": written, "size": size })).into_response())
}

/// Many small, whole files in one request: `relpath` text field, then `file`, repeated.
async fn batch(
    State(state): State<AppState>,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> ApiResult<Json<serde_json::Value>> {
    let root = staging(&state, &id);
    let mut done = Vec::new();
    let mut pending_rel: Option<String> = None;
    while let Some(mut field) = multipart.next_field().await.map_err(|e| AppError::bad_request(format!("{e}")))? {
        let field_name = field.name().map(str::to_owned);
        let file_name = field.file_name().map(str::to_owned);
        match (field_name.as_deref(), file_name) {
            (Some("relpath"), None) => {
                pending_rel = Some(field.text().await.map_err(|e| AppError::bad_request(format!("{e}")))?);
            }
            (_, Some(file_name)) => {
                let raw = pending_rel.take().unwrap_or(file_name);
                let path = safe_rel_path(&raw).ok_or_else(|| AppError::bad_request("bad path"))?;
                let (size, _) = expected(&state, &id, &path).await?;
                let _guard = InFlight::claim(&state, &id, &path)?;
                let dest = root.join(&path);
                if let Some(parent) = dest.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                // Batches always carry whole files, so (re)write from scratch.
                let mut out = tokio::io::BufWriter::new(tokio::fs::File::create(&dest).await?);
                let mut written = 0u64;
                while let Some(bytes) = tokio::time::timeout(BODY_IDLE, field.chunk())
                    .await
                    .map_err(|_| stalled(&path))?
                    .map_err(|e| AppError::bad_request(format!("{e}")))?
                {
                    written += bytes.len() as u64;
                    if written > size {
                        return Err(AppError::bad_request(format!("{path} is larger than declared")));
                    }
                    out.write_all(&bytes).await?;
                }
                out.flush().await?;
                if written != size {
                    return Err(AppError::bad_request(format!("{path}: got {written} bytes, expected {size}")));
                }
                record(&state, &id, &path, written, size).await?;
                done.push(path);
            }
            _ => {}
        }
    }
    Ok(Json(json!({ "received": done })))
}

async fn complete(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<(StatusCode, Json<Pack>)> {
    let current = session_status(&state, &id).await?;
    if current.pack.status != hoard_core::PackStatus::Uploading {
        return Err(AppError::bad_request("upload is already complete"));
    }
    let missing: Vec<&str> =
        current.files.iter().filter(|f| f.received != f.size).map(|f| f.path.as_str()).take(20).collect();
    if !missing.is_empty() {
        return Err(AppError {
            status: StatusCode::CONFLICT,
            err: anyhow::anyhow!(
                "{} files are incomplete, e.g. {}",
                current.files.iter().filter(|f| f.received != f.size).count(),
                missing.join(", ")
            ),
        });
    }
    // Empty files never receive a byte, so create them now; ingest only sees what's on disk.
    for f in current.files.iter().filter(|f| f.size == 0) {
        let dest = staging(&state, &id).join(&f.path);
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::OpenOptions::new().create(true).append(true).open(&dest).await?;
    }
    let id2 = id.clone();
    let pack = db::call(&state.pool, move |c| {
        let auto_name: bool = c
            .query_row("SELECT auto_name FROM upload_meta WHERE pack_id = ?1", [&id2], |r| r.get(0))
            .optional()?
            .unwrap_or(true);
        let tx = c.transaction()?;
        tx.execute("UPDATE packs SET status = 'ingesting', files_done = 0 WHERE id = ?1", [&id2])?;
        tx.execute("DELETE FROM upload_files WHERE pack_id = ?1", [&id2])?;
        tx.execute("DELETE FROM upload_meta WHERE pack_id = ?1", [&id2])?;
        db::enqueue(&tx, jobs::INGEST, Some(&id2), &json!({ "auto_name": auto_name }))?;
        tx.commit()?;
        db::get_pack(c, &id2)?.context("pack vanished")
    })
    .await?;
    state.wake.notify_one();
    Ok((StatusCode::ACCEPTED, Json(pack)))
}

async fn abort(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let id2 = id.clone();
    let removed = db::call(&state.pool, move |c| {
        let status: Option<String> =
            c.query_row("SELECT status FROM packs WHERE id = ?1", [&id2], |r| r.get(0)).optional()?;
        if status.as_deref() != Some("uploading") {
            return Ok(false);
        }
        db::delete_pack(c, &id2)
    })
    .await?;
    if !removed {
        return Err(AppError::not_found(format!("open upload {id}")));
    }
    tokio::fs::remove_dir_all(staging(&state, &id)).await.ok();
    Ok(StatusCode::NO_CONTENT)
}

/// Drop upload sessions nobody has touched in a week, with their staged bytes.
pub(crate) fn sweep_stale(state: &AppState) -> anyhow::Result<usize> {
    let cutoff = (time::OffsetDateTime::now_utc() - time::Duration::days(STALE_DAYS))
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default();
    let mut conn = state.pool.get()?;
    let ids: Vec<String> = {
        let mut stmt = conn.prepare(
            "SELECT p.id FROM packs p LEFT JOIN upload_meta m ON m.pack_id = p.id
             WHERE p.status = 'uploading' AND COALESCE(m.touched_at, p.created_at) < ?1",
        )?;
        stmt.query_map([&cutoff], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
    };
    for id in &ids {
        db::delete_pack(&mut conn, id)?;
        std::fs::remove_dir_all(staging(state, id)).ok();
    }
    Ok(ids.len())
}
