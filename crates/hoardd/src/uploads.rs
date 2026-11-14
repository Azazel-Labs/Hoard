//! Resumable bulk uploads, for imports measured in gigabytes.
//!
//! A session is a pack in the `uploading` state plus a list of expected files and sizes:
//!
//! 1. `POST /api/uploads` registers every file (`path`, `size`) up front and returns what's already received
//!    (zero for a new session).
//! 2. Big files go up in parts of `part_bytes` (the last one shorter): `PUT /api/uploads/{id}/chunk?path=…&offset=…`
//!    with the part's offset. Parts can go up at the same time and in any order, so one big file uses as many
//!    connections as the client likes instead of waiting out each round trip; the session lists the parts that have
//!    arrived, so a client resumes after any failure by sending the rest. Parts are far below 100 MB, so they also
//!    pass proxies with request-size limits.
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
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

use crate::error::{ApiResult, AppError};
use crate::ingest::safe_rel_path;
use crate::{AppState, db, jobs};

/// Largest single chunk or batch request. Below Cloudflare's 100 MB request cap, with room for multipart overhead.
pub(crate) const MAX_REQUEST_BYTES: usize = 96 << 20;
/// The size of each part of a big file. Small enough that a lost connection costs little and several are in flight at
/// once, big enough that a round trip per part doesn't matter.
const PART_BYTES: u64 = 8 << 20;
/// Sessions with no activity for this long are removed at startup.
const STALE_DAYS: i64 = 7;
/// A request body that sends nothing for this long is abandoned, releasing its file for a retry. Without this, a
/// client that vanished mid-chunk (closed laptop, dead wifi) could hold a file's write slot indefinitely.
const BODY_IDLE: std::time::Duration = std::time::Duration::from_secs(60);

fn stalled(path: &str) -> AppError {
    AppError {
        status: StatusCode::REQUEST_TIMEOUT,
        err: anyhow::anyhow!("upload of {path} stalled; send that part again"),
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
    /// The parts that have arrived, for a big file that isn't all there yet.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    parts: Vec<u64>,
}

#[derive(Debug, Serialize)]
struct SessionStatus {
    pack: Pack,
    files: Vec<FileStatus>,
    total_bytes: u64,
    received_bytes: u64,
    max_request_bytes: usize,
    part_bytes: u64,
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
        let mut files = stmt
            .query_map([&id2], |r| {
                Ok(FileStatus {
                    path: r.get(0)?,
                    size: r.get::<_, i64>(1)? as u64,
                    received: r.get::<_, i64>(2)? as u64,
                    parts: Vec::new(),
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let mut stmt = c.prepare("SELECT path, part FROM upload_parts WHERE pack_id = ?1 ORDER BY path, part")?;
        let mut parts: std::collections::HashMap<String, Vec<u64>> = std::collections::HashMap::new();
        for row in stmt.query_map([&id2], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)? as u64)))? {
            let (path, part) = row?;
            parts.entry(path).or_default().push(part);
        }
        for f in files.iter_mut().filter(|f| f.received < f.size) {
            f.parts = parts.remove(&f.path).unwrap_or_default();
        }
        Ok(Some((pack, files)))
    })
    .await?;
    let (pack, files) = found.ok_or_else(|| AppError::not_found(format!("upload {id}")))?;
    let total_bytes = files.iter().map(|f| f.size).sum();
    let received_bytes = files.iter().map(|f| f.received).sum();
    Ok(SessionStatus {
        pack,
        files,
        total_bytes,
        received_bytes,
        max_request_bytes: MAX_REQUEST_BYTES,
        part_bytes: PART_BYTES,
        resumed: false,
    })
}

async fn create(State(state): State<AppState>, Json(body): Json<CreateBody>) -> ApiResult<Response> {
    if body.files.is_empty() {
        return Err(AppError::bad_request("no files"));
    }
    if !state.blobs.can_store() {
        return Err(AppError::conflict(crate::blobs::NO_LIBRARY));
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

/// What's arrived of a file: all of it (`part` is `None`), or one more part of `bytes`. Returns how much is there now.
async fn record(state: &AppState, id: &str, path: &str, part: Option<u64>, bytes: u64, size: u64) -> ApiResult<u64> {
    let (id2, p2) = (id.to_owned(), path.to_owned());
    let received = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        if let Some(part) = part {
            // A part that was already counted (sent twice by a retry) adds nothing.
            let new = tx.execute(
                "INSERT OR IGNORE INTO upload_parts (pack_id, path, part, bytes) VALUES (?1, ?2, ?3, ?4)",
                params![id2, p2, part as i64, bytes as i64],
            )?;
            if new == 1 {
                tx.execute(
                    "UPDATE upload_files SET received = MIN(size, received + ?3) WHERE pack_id = ?1 AND path = ?2",
                    params![id2, p2, bytes as i64],
                )?;
            }
        } else {
            tx.execute(
                "UPDATE upload_files SET received = ?3 WHERE pack_id = ?1 AND path = ?2",
                params![id2, p2, bytes as i64],
            )?;
        }
        let received = tx.query_row(
            "SELECT received FROM upload_files WHERE pack_id = ?1 AND path = ?2",
            params![id2, p2],
            |r| r.get::<_, i64>(0),
        )? as u64;
        tx.execute("UPDATE upload_meta SET touched_at = ?2 WHERE pack_id = ?1", params![id2, db::now()])?;
        if received == size {
            tx.execute(
                "UPDATE packs SET files_done = (SELECT COUNT(*) FROM upload_files WHERE pack_id = ?1 AND received = size)
                 WHERE id = ?1",
                [&id2],
            )?;
        }
        tx.commit()?;
        Ok(received)
    })
    .await?;
    Ok(received)
}

/// One request per file (or part of one) at a time: a second concurrent writer for the same bytes is turned away.
struct InFlight<'a> {
    state: &'a AppState,
    key: String,
}

impl<'a> InFlight<'a> {
    fn claim(state: &'a AppState, id: &str, path: &str, part: Option<u64>) -> ApiResult<Self> {
        let key = part.map_or_else(|| format!("{id}\0{path}"), |part| format!("{id}\0{path}\0{part}"));
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
    let (size, received) = expected(&state, &id, &path).await?;
    if p.offset % PART_BYTES != 0 || p.offset >= size {
        return Err(AppError::bad_request(format!("{} isn't where a part of {path} starts", p.offset)));
    }
    let part = p.offset / PART_BYTES;
    let len = PART_BYTES.min(size - p.offset);
    let _guard = InFlight::claim(&state, &id, &path, Some(part))?;
    let key = (id.clone(), path.clone());
    let have = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT 1 FROM upload_parts WHERE pack_id = ?1 AND path = ?2 AND part = ?3",
            params![key.0, key.1, part as i64],
            |_| Ok(()),
        )
        .optional()?
        .is_some())
    })
    .await?;
    if have || received == size {
        return Ok(Json(json!({ "received": received, "size": size })).into_response());
    }
    let dest = staging(&state, &id).join(&path);
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    // Each part is written in place, so parts can arrive in any order and at once.
    let mut file = tokio::fs::OpenOptions::new().create(true).write(true).truncate(false).open(&dest).await?;
    file.seek(std::io::SeekFrom::Start(p.offset)).await?;
    let mut out = tokio::io::BufWriter::with_capacity(1 << 20, file);
    let mut written = 0u64;
    let mut body = req.into_body().into_data_stream();
    loop {
        let Ok(next) = tokio::time::timeout(BODY_IDLE, body.next()).await else { return Err(stalled(&path)) };
        let Some(bytes) = next else { break };
        // A part that didn't all arrive isn't counted: the client sends it again.
        let bytes = bytes.map_err(|e| AppError::bad_request(format!("upload interrupted: {e}")))?;
        written += bytes.len() as u64;
        if written > len {
            return Err(AppError::bad_request(format!("part {part} of {path} is longer than {len} bytes")));
        }
        out.write_all(&bytes).await?;
    }
    if written != len {
        return Err(AppError::bad_request(format!("part {part} of {path}: got {written} bytes, expected {len}")));
    }
    out.flush().await?;
    // On disk before it's counted, so a crash can't leave a part that's recorded but missing.
    out.get_ref().sync_data().await?;
    let received = record(&state, &id, &path, Some(part), len, size).await?;
    Ok(Json(json!({ "received": received, "size": size })).into_response())
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
                let _guard = InFlight::claim(&state, &id, &path, None)?;
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
                record(&state, &id, &path, None, written, size).await?;
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
        tx.execute("DELETE FROM upload_parts WHERE pack_id = ?1", [&id2])?;
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
