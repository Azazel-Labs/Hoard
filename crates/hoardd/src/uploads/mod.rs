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
//!
//! A pack uploaded again goes into the pack it already is: the library's pack by the same name (the one the files are
//! all in, or the one asked for), or the pack these very files made when `force` sends them again. Its new files are
//! added and the ones that changed are replaced; `copy` makes a pack of its own instead.
//! What's on disk is the source of truth for progress, so a server restart mid-upload loses nothing.
//!
//! Files aren't sent over MCP. Registered over MCP, an upload is answered with addresses to send its files to over
//! HTTP instead, and a key ([`KEY_HEADER`]) that lets requests send this one upload's files, see what's arrived and
//! finish it, without signing in. The key is the registration's claim, so it lasts as long as the upload does, and
//! stops working once the same files are registered again.

mod watch;

pub(crate) use watch::watch_loop;
use watch::{Phase, Watched};

use crate::catalog::{Access, Operation};
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::ingest::safe_rel_path;
use crate::{AppState, db, jobs};
use anyhow::Context;
use axum::Json;
use axum::extract::{Multipart, Path, Query, Request, State};
use axum::http::{HeaderMap, Method, StatusCode, Version};
use axum::response::{IntoResponse, Response};
use futures_util::StreamExt;
use hoard_core::Pack;
use rusqlite::{OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::path::PathBuf;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

/// Largest single chunk or batch request. Below Cloudflare's 100 MB request cap, with room for multipart overhead.
pub(crate) const MAX_REQUEST_BYTES: usize = 96 << 20;
/// The size of each part of a big file. Small enough that a lost connection costs little and several are in flight at
/// once, big enough that a round trip per part doesn't matter.
const PART_BYTES: u64 = 8 << 20;
/// The header carrying an upload's key (see the module's notes).
const KEY_HEADER: &str = "x-hoard-upload-key";

/// Uploads nothing has arrived for in this long are removed, with what they staged (looked for at startup and every
/// hour).
const STALE_HOURS: i64 = 24;
/// A request body that sends nothing for this long is abandoned, releasing its file for a retry. Without this, a
/// client that vanished mid-chunk (closed laptop, dead wifi) could hold a file's write slot indefinitely.
const BODY_IDLE: std::time::Duration = std::time::Duration::from_secs(60);

fn stalled(path: &str) -> AppError {
    AppError { status: StatusCode::REQUEST_TIMEOUT, err: anyhow::anyhow!(tr!("error-upload-stalled", path = path)) }
}

/// Sending a pack's files in pieces.
pub(crate) fn operations() -> Vec<Operation> {
    use Access::Editor;
    vec![
        Editor
            .post("/api/uploads", create)
            .named(
                "create_upload",
                "Start sending a pack's files a piece at a time, for a pack too big to send in one request: list every \
                 file's relative path and size, and get an upload id and what has arrived already (so an interrupted \
                 upload carries on). Send each file with `send_upload_chunk` (big ones) or `send_upload_batch` (many \
                 small ones), then `complete_upload`. A pack with exactly these files already is added to, unless \
                 `copy`. Files aren't sent over MCP: called over MCP, the answer's `send` gives addresses to send them \
                 to over HTTP instead, with a key that lets those requests in for this upload only.",
            )
            .limit(64 << 20),
        Editor.get("/api/uploads/{id}", status).named(
            "get_upload",
            "What an upload in pieces has received so far, file by file, to carry on from where it stopped.",
        ),
        Editor.delete("/api/uploads/{id}", abort).named(
            "cancel_upload",
            "Cancel an upload in pieces and throw away what it received. `claim` (from `create_upload`) leaves it be if \
             the same files were registered again since.",
        ),
        Editor
            .put("/api/uploads/{id}/chunk", chunk)
            .named(
                "send_upload_chunk",
                "Send part of one file of an upload in pieces: the bytes from `offset` on, of the file at `path`.",
            )
            .limit(MAX_REQUEST_BYTES),
        Editor
            .post("/api/uploads/{id}/batch", batch)
            .named(
                "send_upload_batch",
                "Send many small, whole files of an upload in pieces in one request: a multipart form with a `relpath` \
                 text field, then a `file`, repeated.",
            )
            .limit(MAX_REQUEST_BYTES),
        Editor.post("/api/uploads/{id}/complete", complete).named(
            "complete_upload",
            "Finish an upload in pieces once every file has arrived: the pack is made (or added to) and read, and \
             answered with (202).",
        ),
    ]
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct NewFile {
    path: String,
    size: u64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CreateBody {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    source: Option<String>,
    files: Vec<NewFile>,
    /// Import even if a pack with exactly these files already exists (into that pack, unless `copy`).
    #[serde(default)]
    force: bool,
    /// Make a pack of its own even when the library has this pack already, instead of adding to it.
    #[serde(default)]
    copy: bool,
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

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct FileStatus {
    path: String,
    size: u64,
    received: u64,
    /// The parts that have arrived, for a big file that isn't all there yet.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    parts: Vec<u64>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct SessionStatus {
    pack: Pack,
    files: Vec<FileStatus>,
    total_bytes: u64,
    received_bytes: u64,
    max_request_bytes: usize,
    part_bytes: u64,
    /// True when this call picked up an existing session instead of starting one.
    resumed: bool,
    /// This registration's hold on the session, for canceling it (see `abort`); only in the answer to registering.
    #[serde(skip_serializing_if = "Option::is_none")]
    claim: Option<String>,
    /// The pack the upload goes into once it's complete, when the library has it already.
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_into: Option<Pack>,
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
        let target: Option<String> = c
            .query_row("SELECT merge_into FROM upload_meta WHERE pack_id = ?1", [&id2], |r| r.get(0))
            .optional()?
            .flatten();
        let merge_into = match target {
            Some(t) => db::get_pack(c, &t)?,
            None => None,
        };
        Ok(Some((pack, files, merge_into)))
    })
    .await?;
    let (pack, files, merge_into) = found.ok_or_else(|| AppError::not_found(tr!("error-upload-not-found", id = id)))?;
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
        claim: None,
        merge_into,
    })
}

/// The pack an upload goes into: `same` (the pack these very files made) or else the oldest pack called `name`, if
/// it's one Hoard keeps the files of itself (not a watched folder's, which is read where it sits) and isn't archived.
fn merge_target(c: &rusqlite::Connection, same: Option<&str>, name: Option<&str>) -> anyhow::Result<Option<String>> {
    Ok(c.query_row(
        "SELECT p.id FROM packs p
          WHERE p.status <> 'uploading' AND p.archived_at IS NULL
            AND (p.id = ?1 OR (?1 IS NULL AND p.name = ?2 COLLATE NOCASE))
            AND NOT EXISTS (SELECT 1 FROM locations l WHERE l.id = p.location_id AND l.mode = 'watched')
          ORDER BY p.created_at LIMIT 1",
        params![same, name],
        |r| r.get(0),
    )
    .optional()?)
}

async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreateBody>,
) -> ApiResult<Response> {
    if body.files.is_empty() {
        return Err(AppError::bad_request(tr!("error-upload-no-files-listed")));
    }
    if !state.blobs.can_store() {
        return Err(AppError::conflict(crate::blobs::no_library()));
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
    match &existing {
        Some(Existing::Open(id)) => {
            // It's this registration's now: whoever registered it before can no longer cancel it. Asked for a copy,
            // it's a pack of its own after all.
            let claim = uuid::Uuid::new_v4().to_string();
            let (id2, c2, copy) = (id.clone(), claim.clone(), body.copy);
            db::call(&state.pool, move |c| {
                c.execute(
                    "UPDATE upload_meta SET claim = ?2, touched_at = ?3, merge_into = IIF(?4, NULL, merge_into)
                      WHERE pack_id = ?1",
                    params![id2, c2, db::now(), copy],
                )?;
                Ok(())
            })
            .await?;
            let mut s = session_status(&state, id).await?;
            s.resumed = true;
            s.claim = Some(claim);
            return Ok(answer(&state, &headers, StatusCode::OK, s));
        }
        Some(Existing::Imported(pack)) if !body.force && !body.copy => {
            let msg = tr!("error-already-imported", name = pack.name);
            return Ok((StatusCode::CONFLICT, Json(json!({ "error": msg, "pack": pack }))).into_response());
        }
        _ => {}
    }

    let paths: Vec<String> = files.iter().map(|(p, _)| p.clone()).collect();
    let asked = body.name.filter(|n| !n.trim().is_empty());
    let auto_name = asked.is_none();
    // A name made up from the date says nothing about which pack this is, so it goes into none.
    let telling = asked.or_else(|| crate::api::pack_name_from_files(&paths));
    let name = telling.clone().unwrap_or_else(|| crate::api::default_pack_name(&paths));
    let merge_into = if body.copy {
        None
    } else {
        let same = match &existing {
            Some(Existing::Imported(pack)) => Some(pack.id.clone()),
            _ => None,
        };
        db::call(&state.pool, move |c| merge_target(c, same.as_deref(), telling.as_deref())).await?
    };
    let id = uuid::Uuid::now_v7().to_string();
    tokio::fs::create_dir_all(staging(&state, &id)).await?;
    let claim = uuid::Uuid::new_v4().to_string();
    let (id2, c2) = (id.clone(), claim.clone());
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        tx.execute(
            // Empty files are there already.
            "INSERT INTO packs (id, name, status, source, files_total, files_done, created_at)
             VALUES (?1, ?2, 'uploading', ?3, ?4, ?5, ?6)",
            params![
                id2,
                name,
                body.source,
                files.len() as i64,
                files.iter().filter(|(_, size)| *size == 0).count() as i64,
                db::now()
            ],
        )?;
        tx.execute(
            "INSERT INTO upload_meta (pack_id, auto_name, touched_at, claim, merge_into) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id2, auto_name, db::now(), c2, merge_into],
        )?;
        tx.execute("INSERT INTO import_keys (import_key, pack_id) VALUES (?1, ?2)", params![key, id2])?;
        {
            let mut stmt =
                tx.prepare("INSERT INTO upload_files (pack_id, path, size, received) VALUES (?1, ?2, ?3, 0)")?;
            for (path, size) in &files {
                stmt.execute(params![id2, path, *size as i64])?;
            }
        }
        tx.commit()?;
        Ok(())
    })
    .await?;
    let mut s = session_status(&state, &id).await?;
    s.claim = Some(claim);
    Ok(answer(&state, &headers, StatusCode::CREATED, s))
}

/// A registered upload, with (for an assistant, over MCP) the addresses to send its files to.
fn answer(state: &AppState, headers: &HeaderMap, status: StatusCode, s: SessionStatus) -> Response {
    if !crate::mcp::operations::over_mcp() {
        return (status, Json(s)).into_response();
    }
    let send = addresses(&crate::mcp::base_url(state, headers), &s);
    let mut v = serde_json::to_value(s).unwrap_or_default();
    v["send"] = send;
    (status, Json(v)).into_response()
}

/// How to send an upload's files over HTTP, for an assistant (see the module's notes).
fn addresses(base: &str, s: &SessionStatus) -> serde_json::Value {
    let (id, key) = (&s.pack.id, s.claim.as_deref().unwrap_or_default());
    let up = format!("{base}/api/uploads/{id}");
    let mb = PART_BYTES >> 20;
    json!({
        "how": format!(
            "Send the files over HTTP yourself (with curl, say), not through MCP, each request with the header \
             `{KEY_HEADER}: {key}`. It works for this upload only, until it's finished or canceled. A file over {mb} MB \
             goes in parts of `part_bytes` (the last one shorter), each to `part` with its offset (0, {PART_BYTES}, \
             …), several at once if you like; smaller files go whole, many to a request, to `batch`, as a multipart \
             form of a `relpath` field then a `file`, repeated, up to {} MB a request. Paths are those registered, \
             URL-encoded in `part`'s. `files` lists what has arrived already (and a big file's `parts`), so after \
             any failure send just the rest. Once every file is there, POST to `complete` (or call complete_upload).",
            MAX_REQUEST_BYTES >> 20
        ),
        "header": format!("{KEY_HEADER}: {key}"),
        "part": format!("PUT {up}/chunk?path={{path}}&offset={{offset}}"),
        "batch": format!("POST {up}/batch"),
        "status": format!("GET {up}"),
        "complete": format!("POST {up}/complete"),
        "examples": [
            format!("curl -fsS -H '{KEY_HEADER}: {key}' -F relpath=Textures/rock.png -F file=@Textures/rock.png {up}/batch"),
            format!(
                "dd if=Rock.fbx bs={mb}M skip=1 count=1 status=none | curl -fsS -X PUT -H '{KEY_HEADER}: {key}' \
                 --data-binary @- '{up}/chunk?path=Rock.fbx&offset={PART_BYTES}'"
            ),
            format!("curl -fsS -X POST -H '{KEY_HEADER}: {key}' {up}/complete"),
        ],
    })
}

/// Who a request is that carries an upload's key instead of a sign-in: allowed to send that upload's files, see what
/// has arrived and finish it, and nothing else.
pub(crate) async fn keyed(
    state: &AppState,
    method: &Method,
    path: &str,
    headers: &HeaderMap,
) -> ApiResult<Option<crate::auth::Who>> {
    let Some(key) = headers.get(KEY_HEADER).and_then(|v| v.to_str().ok()) else { return Ok(None) };
    let Some(id) = keyed_upload(method, path) else { return Ok(None) };
    let (id, key) = (id.to_owned(), key.to_owned());
    let valid = db::call(&state.pool, move |c| {
        Ok(c.query_row("SELECT 1 FROM upload_meta WHERE pack_id = ?1 AND claim = ?2", params![id, key], |_| Ok(()))
            .optional()?
            .is_some())
    })
    .await?;
    Ok(valid.then(|| crate::auth::Who::Token { name: "upload key".into(), role: crate::auth::Role::Edit, owner: None }))
}

/// The upload a request with a key may be for: sending its files, what has arrived, finishing it.
fn keyed_upload<'a>(method: &Method, path: &'a str) -> Option<&'a str> {
    let rest = path.strip_prefix("/api/uploads/")?;
    let (id, then) = rest.split_once('/').unwrap_or((rest, ""));
    let allowed = matches!(
        (method, then),
        (&Method::PUT, "chunk") | (&Method::POST, "batch" | "complete") | (&Method::GET | &Method::HEAD, "")
    );
    (allowed && !id.is_empty()).then_some(id)
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
        Err(true) => Err(AppError::bad_request(tr!("error-upload-not-part", path = path, id = id))),
        Err(false) => Err(AppError::not_found(tr!("error-upload-not-found", id = id))),
    }
}

/// What's arrived of a file: all of it (`part` is `None`), or one more part of `bytes`. Returns how much is there now.
async fn record(state: &AppState, id: &str, path: &str, part: Option<u64>, bytes: u64, size: u64) -> ApiResult<u64> {
    let (id2, p2) = (id.to_owned(), path.to_owned());
    let received = db::call(&state.pool, move |c| {
        // Cached, as this runs for every part of every file uploaded.
        let tx = c.transaction()?;
        let before = tx
            .prepare_cached("SELECT received FROM upload_files WHERE pack_id = ?1 AND path = ?2")?
            .query_row(params![id2, p2], |r| r.get::<_, i64>(0))? as u64;
        if let Some(part) = part {
            // A part that was already counted (sent twice by a retry) adds nothing.
            let new = tx
                .prepare_cached(
                    "INSERT OR IGNORE INTO upload_parts (pack_id, path, part, bytes) VALUES (?1, ?2, ?3, ?4)",
                )?
                .execute(params![id2, p2, part as i64, bytes as i64])?;
            if new == 1 {
                tx.prepare_cached(
                    "UPDATE upload_files SET received = MIN(size, received + ?3) WHERE pack_id = ?1 AND path = ?2",
                )?
                .execute(params![id2, p2, bytes as i64])?;
            }
        } else {
            tx.prepare_cached("UPDATE upload_files SET received = ?3 WHERE pack_id = ?1 AND path = ?2")?
                .execute(params![id2, p2, bytes as i64])?;
        }
        let received = tx
            .prepare_cached("SELECT received FROM upload_files WHERE pack_id = ?1 AND path = ?2")?
            .query_row(params![id2, p2], |r| r.get::<_, i64>(0))? as u64;
        tx.prepare_cached("UPDATE upload_meta SET touched_at = ?2 WHERE pack_id = ?1")?
            .execute(params![id2, db::now()])?;
        // Counted as it completes: counting them all again took longer the more files the upload had.
        if received == size && before < size {
            tx.prepare_cached("UPDATE packs SET files_done = files_done + 1 WHERE id = ?1")?.execute([&id2])?;
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
                err: anyhow::anyhow!(tr!("error-upload-busy", path = path)),
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
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
    let path = safe_rel_path(&p.path).ok_or_else(|| AppError::bad_request(tr!("error-bad-path")))?;
    let watch = Watched::start(format!("{id} {path} @{}", p.offset), req.version(), req.headers());
    let (size, received) = expected(&state, &id, &path).await?;
    if p.offset % PART_BYTES != 0 || p.offset >= size {
        return Err(AppError::bad_request(tr!("error-upload-part-offset", offset = p.offset, path = path)));
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
        // Read what the client is sending anyway: answering before the body is in breaks its connection mid-write.
        let mut body = req.into_body().into_data_stream();
        let mut skipped = 0u64;
        watch.phase(Phase::Receiving);
        while skipped <= len {
            match tokio::time::timeout(BODY_IDLE, body.next()).await {
                Ok(Some(Ok(bytes))) => {
                    watch.got(bytes.len());
                    skipped += bytes.len() as u64;
                }
                _ => break,
            }
        }
        watch.finished();
        return Ok(Json(json!({ "received": received, "size": size })).into_response());
    }
    let dest = staging(&state, &id).join(&path);
    watch.phase(Phase::Saving);
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
        watch.phase(Phase::Receiving);
        let Ok(next) = tokio::time::timeout(BODY_IDLE, body.next()).await else { return Err(stalled(&path)) };
        let Some(bytes) = next else { break };
        // A part that didn't all arrive isn't counted: the client sends it again.
        let bytes =
            bytes.map_err(|e| AppError::bad_request(tr!("error-upload-interrupted", reason = e.to_string())))?;
        watch.got(bytes.len());
        written += bytes.len() as u64;
        if written > len {
            return Err(AppError::bad_request(tr!("error-upload-part-long", part = part, path = path, len = len)));
        }
        watch.phase(Phase::Saving);
        out.write_all(&bytes).await?;
    }
    if written != len {
        return Err(AppError::bad_request(tr!(
            "error-upload-part-short",
            part = part,
            path = path,
            got = written,
            len = len
        )));
    }
    watch.phase(Phase::Saving);
    out.flush().await?;
    // On disk before it's counted, so a crash can't leave a part that's recorded but missing.
    out.get_ref().sync_data().await?;
    watch.phase(Phase::Database);
    let received = record(&state, &id, &path, Some(part), len, size).await?;
    watch.finished();
    Ok(Json(json!({ "received": received, "size": size })).into_response())
}

/// Many small, whole files in one request: `relpath` text field, then `file`, repeated.
async fn batch(
    State(state): State<AppState>,
    Path(id): Path<String>,
    version: Version,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> ApiResult<Json<serde_json::Value>> {
    let watch = Watched::start(format!("{id} batch"), version, &headers);
    let root = staging(&state, &id);
    let mut done = Vec::new();
    let mut pending_rel: Option<String> = None;
    loop {
        watch.phase(Phase::Receiving);
        let Some(mut field) = multipart.next_field().await.map_err(|e| AppError::bad_request(format!("{e}")))? else {
            break;
        };
        let field_name = field.name().map(str::to_owned);
        let file_name = field.file_name().map(str::to_owned);
        match (field_name.as_deref(), file_name) {
            (Some("relpath"), None) => {
                pending_rel = Some(field.text().await.map_err(|e| AppError::bad_request(format!("{e}")))?);
            }
            (_, Some(file_name)) => {
                let raw = pending_rel.take().unwrap_or(file_name);
                let path = safe_rel_path(&raw).ok_or_else(|| AppError::bad_request(tr!("error-bad-path")))?;
                watch.phase(Phase::Database);
                let (size, _) = expected(&state, &id, &path).await?;
                let _guard = InFlight::claim(&state, &id, &path, None)?;
                watch.phase(Phase::Saving);
                let dest = root.join(&path);
                if let Some(parent) = dest.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                // Batches always carry whole files, so (re)write from scratch.
                let mut out = tokio::io::BufWriter::new(tokio::fs::File::create(&dest).await?);
                let mut written = 0u64;
                loop {
                    watch.phase(Phase::Receiving);
                    let Some(bytes) = tokio::time::timeout(BODY_IDLE, field.chunk())
                        .await
                        .map_err(|_| stalled(&path))?
                        .map_err(|e| AppError::bad_request(format!("{e}")))?
                    else {
                        break;
                    };
                    watch.got(bytes.len());
                    written += bytes.len() as u64;
                    if written > size {
                        return Err(AppError::bad_request(tr!("error-upload-file-long", path = path)));
                    }
                    watch.phase(Phase::Saving);
                    out.write_all(&bytes).await?;
                }
                watch.phase(Phase::Saving);
                out.flush().await?;
                if written != size {
                    return Err(AppError::bad_request(tr!(
                        "error-upload-file-short",
                        path = path,
                        got = written,
                        len = size
                    )));
                }
                watch.phase(Phase::Database);
                record(&state, &id, &path, None, written, size).await?;
                done.push(path);
            }
            _ => {}
        }
    }
    watch.finished();
    Ok(Json(json!({ "received": done })))
}

async fn complete(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<(StatusCode, Json<Pack>)> {
    let current = session_status(&state, &id).await?;
    if current.pack.status != hoard_core::PackStatus::Uploading {
        return Err(AppError::bad_request(tr!("error-upload-complete")));
    }
    let missing: Vec<&str> =
        current.files.iter().filter(|f| f.received != f.size).map(|f| f.path.as_str()).take(20).collect();
    if !missing.is_empty() {
        return Err(AppError {
            status: StatusCode::CONFLICT,
            err: anyhow::anyhow!(tr!(
                "error-upload-incomplete",
                count = current.files.iter().filter(|f| f.received != f.size).count(),
                files = missing.join(", ")
            )),
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
        let (auto_name, target): (bool, Option<String>) = c
            .query_row("SELECT auto_name, merge_into FROM upload_meta WHERE pack_id = ?1", [&id2], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .optional()?
            .unwrap_or((true, None));
        let tx = c.transaction()?;
        if let Some(target) = target
            && hand_over(&tx, &id2, &target)?
        {
            tx.commit()?;
            tracing::info!(upload = %id2, pack = %target, "an upload goes into the pack it already is");
            return db::get_pack(c, &target)?.context("pack vanished");
        }
        tx.execute("UPDATE packs SET status = 'ingesting', files_done = 0 WHERE id = ?1", [&id2])?;
        forget_upload(&tx, &id2)?;
        db::enqueue(&tx, jobs::INGEST, Some(&id2), &json!({ "auto_name": auto_name }))?;
        tx.commit()?;
        db::get_pack(c, &id2)?.context("pack vanished")
    })
    .await?;
    state.wake.notify_one();
    Ok((StatusCode::ACCEPTED, Json(pack)))
}

/// An upload's own rows, once its files are all there.
fn forget_upload(tx: &rusqlite::Transaction<'_>, upload: &str) -> rusqlite::Result<()> {
    for table in ["upload_files", "upload_parts", "upload_meta"] {
        tx.execute(&format!("DELETE FROM {table} WHERE pack_id = ?1"), [upload])?;
    }
    Ok(())
}

/// A complete upload goes into `target`, if that's still there to go into (it may have been deleted or archived
/// since): the upload stops being a pack of its own, and the pack reads its files from the upload's staging folder.
fn hand_over(tx: &rusqlite::Transaction<'_>, upload: &str, target: &str) -> anyhow::Result<bool> {
    if merge_target(tx, Some(target), None)?.is_none() {
        return Ok(false);
    }
    tx.execute(
        "INSERT OR IGNORE INTO import_keys (import_key, pack_id) SELECT import_key, ?2 FROM import_keys WHERE pack_id = ?1",
        params![upload, target],
    )?;
    forget_upload(tx, upload)?;
    db::delete_pack_in(tx, upload)?;
    tx.execute(
        "UPDATE packs SET status = 'ingesting', error = NULL WHERE id = ?1 AND status IN ('ready', 'failed')",
        [target],
    )?;
    db::enqueue(tx, jobs::INGEST, Some(target), &json!({ "auto_name": false, "from": upload }))?;
    Ok(true)
}

#[derive(Deserialize, schemars::JsonSchema)]
struct AbortParams {
    /// The claim registering the upload gave. If the same files were registered again since (dropped again to retry),
    /// the upload is the later registration's, and canceling the earlier one leaves it be. Without one, it's canceled
    /// whoever registered it.
    claim: Option<String>,
}

enum Abort {
    Removed,
    NotOpen,
    TakenOver,
}

async fn abort(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<AbortParams>,
) -> ApiResult<StatusCode> {
    let id2 = id.clone();
    let outcome = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let row: Option<(String, Option<String>)> = tx
            .query_row(
                "SELECT p.status, m.claim FROM packs p LEFT JOIN upload_meta m ON m.pack_id = p.id WHERE p.id = ?1",
                [&id2],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let outcome = match row {
            Some((status, held)) if status == "uploading" => {
                if p.claim.is_some() && p.claim != held {
                    Abort::TakenOver
                } else if db::delete_pack_in(&tx, &id2)? {
                    Abort::Removed
                } else {
                    Abort::NotOpen
                }
            }
            _ => Abort::NotOpen,
        };
        tx.commit()?;
        Ok(outcome)
    })
    .await?;
    match outcome {
        Abort::Removed => {}
        Abort::NotOpen => return Err(AppError::not_found(tr!("error-open-upload-not-found", id = id))),
        Abort::TakenOver => {
            tracing::info!(id, "kept an upload canceled by an earlier registration of it");
            return Ok(StatusCode::NO_CONTENT);
        }
    }
    tokio::fs::remove_dir_all(staging(&state, &id)).await.ok();
    Ok(StatusCode::NO_CONTENT)
}

/// Drop uploads nothing has arrived for in a day, with their staged bytes: sessions never resumed, and folders left by
/// single uploads that never became a pack (the server stopped partway).
pub(crate) fn sweep_stale(state: &AppState) -> anyhow::Result<usize> {
    let cutoff = (time::OffsetDateTime::now_utc() - time::Duration::hours(STALE_HOURS))
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
    let receiving = |id: &str| {
        let prefix = format!("{id}\0");
        state
            .uploads_in_flight
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .any(|k| k.starts_with(&prefix))
    };
    let mut removed = 0;
    for id in ids.iter().filter(|id| !receiving(id)) {
        db::delete_pack(&mut conn, id)?;
        std::fs::remove_dir_all(staging(state, id)).ok();
        removed += 1;
    }
    let max_age = std::time::Duration::from_secs(STALE_HOURS.unsigned_abs() * 3600);
    for entry in std::fs::read_dir(state.cfg.staging_dir()).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let old = entry.metadata().and_then(|m| m.modified()).is_ok_and(|t| t.elapsed().is_ok_and(|age| age > max_age));
        if old && db::get_pack(&conn, &name)?.is_none() && !merging(&conn, &name)? {
            std::fs::remove_dir_all(entry.path()).ok();
            removed += 1;
        }
    }
    Ok(removed)
}

/// Whether an upload's files are waiting to go into a pack the library had already (see `complete`).
fn merging(c: &rusqlite::Connection, upload: &str) -> anyhow::Result<bool> {
    Ok(c.query_row(
        "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status IN ('queued', 'running')
                          AND json_extract(payload, '$.from') = ?2)",
        params![jobs::INGEST, upload],
        |r| r.get(0),
    )?)
}

/// Sweep abandoned uploads now and every hour.
pub(crate) async fn sweep_loop(state: AppState) {
    let mut every = tokio::time::interval(std::time::Duration::from_secs(3600));
    loop {
        every.tick().await;
        let st = std::sync::Arc::clone(&state);
        match tokio::task::spawn_blocking(move || sweep_stale(&st)).await {
            Ok(Ok(0)) => {}
            Ok(Ok(n)) => tracing::info!(removed = n, "removed abandoned uploads"),
            Ok(Err(e)) => tracing::warn!(error = %e, "could not sweep abandoned uploads"),
            Err(e) => tracing::warn!(error = %e, "the upload sweep stopped"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_key_opens_only_its_uploads_own_requests() {
        use axum::http::Method;
        assert_eq!(keyed_upload(&Method::PUT, "/api/uploads/u1/chunk"), Some("u1"));
        assert_eq!(keyed_upload(&Method::POST, "/api/uploads/u1/batch"), Some("u1"));
        assert_eq!(keyed_upload(&Method::POST, "/api/uploads/u1/complete"), Some("u1"));
        assert_eq!(keyed_upload(&Method::GET, "/api/uploads/u1"), Some("u1"));
        // Not canceling it, registering another, or anything else.
        assert_eq!(keyed_upload(&Method::DELETE, "/api/uploads/u1"), None);
        assert_eq!(keyed_upload(&Method::POST, "/api/uploads"), None);
        assert_eq!(keyed_upload(&Method::POST, "/api/uploads/"), None);
        assert_eq!(keyed_upload(&Method::GET, "/api/uploads/u1/chunk"), None);
        assert_eq!(keyed_upload(&Method::PUT, "/api/packs/u1"), None);
    }

    fn pack(c: &rusqlite::Connection, id: &str, name: &str, status: &str, created: &str) {
        c.execute(
            "INSERT INTO packs (id, name, status, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, name, status, created],
        )
        .unwrap();
    }

    /// A pack uploaded again goes into the oldest pack by its name that Hoard keeps the files of, or the pack its very
    /// files made, and never into an upload, an archived pack or a watched folder's.
    #[test]
    fn an_upload_goes_into_the_pack_the_library_has_already() {
        let c = db::test_library();
        pack(&c, "old", "Viking Village", "ready", "2026-01-01");
        pack(&c, "new", "viking village", "failed", "2026-02-01");
        pack(&c, "sending", "Castle", "uploading", "2026-01-01");
        pack(&c, "shelved", "Market", "ready", "2026-01-01");
        pack(&c, "watched", "Cathedral", "ready", "2026-01-01");
        c.execute("UPDATE packs SET archived_at = 'x' WHERE id = 'shelved'", []).unwrap();
        c.execute_batch(
            "INSERT INTO locations (id, name, path, mode, created_at) VALUES (1, 'Disk', '/packs', 'watched', 'x');
             UPDATE packs SET location_id = 1, folder = 'Cathedral' WHERE id = 'watched';",
        )
        .unwrap();
        let target = |same: Option<&str>, name: Option<&str>| merge_target(&c, same, name).unwrap();
        assert_eq!(target(None, Some("VIKING VILLAGE")).as_deref(), Some("old"));
        assert_eq!(target(Some("new"), Some("Viking Village")).as_deref(), Some("new"));
        for name in ["Castle", "Market", "Cathedral", "Elsewhere"] {
            assert_eq!(target(None, Some(name)), None, "{name}");
        }
        assert_eq!(target(None, None), None);
        assert_eq!(target(Some("watched"), Some("Viking Village")), None);
    }

    #[test]
    fn a_complete_upload_hands_its_files_to_the_pack_it_goes_into() {
        let mut c = db::test_library();
        pack(&c, "pack", "Viking Village", "ready", "2026-01-01");
        pack(&c, "upload", "Viking Village", "uploading", "2026-03-01");
        c.execute_batch(
            "INSERT INTO import_keys (import_key, pack_id) VALUES ('first', 'pack'), ('second', 'upload');
             INSERT INTO upload_meta (pack_id, auto_name, touched_at, merge_into) VALUES ('upload', 1, 'x', 'pack');
             INSERT INTO upload_files (pack_id, path, size, received) VALUES ('upload', 'a.png', 1, 1);",
        )
        .unwrap();
        let tx = c.transaction().unwrap();
        assert!(hand_over(&tx, "upload", "pack").unwrap());
        tx.commit().unwrap();
        let one = |sql: &str| -> String { c.query_row(sql, [], |r| r.get(0)).unwrap() };
        assert_eq!(one("SELECT status FROM packs WHERE id = 'pack'"), "ingesting");
        assert_eq!(one("SELECT count(*) || '' FROM packs WHERE id = 'upload'"), "0");
        assert_eq!(
            one("SELECT group_concat(import_key) FROM import_keys WHERE pack_id = 'pack' ORDER BY 1"),
            "first,second"
        );
        assert_eq!(one("SELECT count(*) || '' FROM upload_files"), "0");
        assert_eq!(
            one("SELECT json_extract(payload, '$.from') FROM jobs WHERE kind = 'ingest' AND pack_id = 'pack'"),
            "upload"
        );
        assert!(merging(&c, "upload").unwrap());
        // Gone by the time the upload's done: it stays a pack of its own.
        pack(&c, "again", "Market", "uploading", "2026-03-01");
        let tx = c.transaction().unwrap();
        assert!(!hand_over(&tx, "again", "deleted").unwrap());
    }
}
