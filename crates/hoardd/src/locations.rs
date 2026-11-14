//! Library locations: the folders (or buckets) a library's files live in, set up in the portal the way media
//! servers do it.
//!
//! - **Managed:** Hoard copies uploads into it, content-addressed (see `blobs.rs`). Each ticks the asset kinds it
//!   takes, and one is the catch-all for everything else.
//! - **Watched:** an existing folder of packs, one subfolder per pack, indexed where it sits and rescanned for new
//!   ones. Hoard never writes, moves or deletes anything there.
//!
//! A new server has none, and the portal opens its setup to add the first.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use axum::Json;
use axum::extract::{Path as UrlPath, Query, State};
use axum::http::StatusCode;
use futures_util::StreamExt;
use hoard_core::AssetKind;
use rusqlite::{Connection, OptionalExtension as _, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::AppState;
use crate::config::{Config, StorageKind};
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Mode {
    Managed,
    Watched,
}

impl Mode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Managed => "managed",
            Self::Watched => "watched",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Location {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) mode: Mode,
    /// An absolute folder path, or `s3://bucket` for a managed location in a bucket.
    pub(crate) path: String,
    /// Asset kinds a managed location takes (`AssetKind::as_str`).
    pub(crate) kinds: Vec<String>,
    pub(crate) catch_all: bool,
    /// When a watched location was last scanned.
    pub(crate) scanned_at: Option<String>,
    pub(crate) created_at: String,
}

const COLUMNS: &str = "id, name, mode, path, kinds, catch_all, scanned_at, created_at";

fn from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<Location> {
    let bad = |col: usize, e: Box<dyn std::error::Error + Send + Sync>| {
        rusqlite::Error::FromSqlConversionFailure(col, rusqlite::types::Type::Text, e)
    };
    // A location read wrong could be written to, or be emptied by a rebalance: refuse what isn't understood.
    let mode = match r.get::<_, String>(2)?.as_str() {
        "managed" => Mode::Managed,
        "watched" => Mode::Watched,
        other => return Err(bad(2, format!("unknown location mode {other:?}").into())),
    };
    let kinds: String = r.get(4)?;
    Ok(Location {
        id: r.get(0)?,
        name: r.get(1)?,
        mode,
        path: r.get(3)?,
        kinds: serde_json::from_str(&kinds).map_err(|e| bad(4, e.into()))?,
        catch_all: r.get::<_, i64>(5)? != 0,
        scanned_at: r.get(6)?,
        created_at: r.get(7)?,
    })
}

pub(crate) fn list(c: &Connection) -> Result<Vec<Location>> {
    let mut stmt = c.prepare(&format!("SELECT {COLUMNS} FROM locations ORDER BY id"))?;
    Ok(stmt.query_map([], from_row)?.collect::<rusqlite::Result<_>>()?)
}

pub(crate) fn get(c: &Connection, id: i64) -> Result<Option<Location>> {
    Ok(c.query_row(&format!("SELECT {COLUMNS} FROM locations WHERE id = ?1"), [id], from_row).optional()?)
}

/// A file of a watched location: its folder, the pack's subfolder, then the asset's path inside the pack.
pub(crate) fn watched_path(root: &str, folder: Option<&str>, rel: &str) -> PathBuf {
    let mut p = PathBuf::from(root);
    if let Some(f) = folder {
        p.push(f);
    }
    p.extend(rel.split('/').filter(|s| !s.is_empty()));
    p
}

/// The folder a watched pack is read from, or `None` for a pack Hoard keeps itself.
pub(crate) fn watched_root(c: &Connection, pack_id: &str) -> Result<Option<PathBuf>> {
    Ok(c.query_row(
        "SELECT l.path, p.folder FROM packs p JOIN locations l ON l.id = p.location_id
          WHERE p.id = ?1 AND l.mode = 'watched'",
        [pack_id],
        |r| Ok(watched_path(&r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?.as_deref(), "")),
    )
    .optional()?)
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NewLocation {
    #[serde(default)]
    pub(crate) name: Option<String>,
    pub(crate) mode: Mode,
    pub(crate) path: String,
    #[serde(default)]
    pub(crate) kinds: Vec<String>,
    #[serde(default)]
    pub(crate) catch_all: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LocationPatch {
    name: Option<String>,
    kinds: Option<Vec<String>>,
    catch_all: Option<bool>,
}

fn check_kinds(kinds: &[String]) -> Result<Vec<String>, AppError> {
    let mut out: Vec<String> = Vec::new();
    for k in kinds {
        if !AssetKind::ALL.iter().any(|a| a.as_str() == k) {
            return Err(AppError::bad_request(tr!("error-unknown-kind", kind = format!("{k:?}"))));
        }
        if !out.contains(k) {
            out.push(k.clone());
        }
    }
    Ok(out)
}

/// Tidy a path as typed: trimmed, without a trailing separator (except a root's).
fn normalize(path: &str) -> String {
    let p = path.trim();
    if p.starts_with("s3://") {
        return p.trim_end_matches('/').to_owned();
    }
    let trimmed = p.trim_end_matches(['/', '\\']);
    if trimmed.is_empty() || trimmed.ends_with(':') { p.to_owned() } else { trimmed.to_owned() }
}

/// Add a location. The first managed one becomes the catch-all, and a new catch-all takes over from the old.
pub(crate) fn insert(conn: &Connection, new: &NewLocation) -> Result<Location> {
    let c = conn.unchecked_transaction()?;
    let managed = new.mode == Mode::Managed;
    let has_catch_all: bool =
        c.query_row("SELECT EXISTS (SELECT 1 FROM locations WHERE mode = 'managed' AND catch_all = 1)", [], |r| {
            r.get(0)
        })?;
    let catch_all = managed && (new.catch_all || !has_catch_all);
    if catch_all {
        c.execute("UPDATE locations SET catch_all = 0", [])?;
    }
    let kinds = if managed { serde_json::to_string(&new.kinds)? } else { "[]".to_owned() };
    let name = new
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map(str::to_owned)
        .or_else(|| Path::new(&new.path).file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "Library".to_owned());
    c.execute(
        "INSERT INTO locations (name, mode, path, kinds, catch_all, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![name, new.mode.as_str(), new.path, kinds, catch_all, db::now()],
    )?;
    let loc = get(&c, c.last_insert_rowid())?.context("location just added is gone")?;
    c.commit()?;
    Ok(loc)
}

/// An install from before library locations keeps its files where they are: they become its one managed
/// location, the catch-all. A new server gets nothing, and the portal asks.
pub(crate) fn adopt_existing(c: &Connection, cfg: &Config) -> Result<Option<Location>> {
    let any = |sql: &str| -> Result<bool> { Ok(c.query_row(sql, [], |r| r.get(0))?) };
    if any("SELECT EXISTS (SELECT 1 FROM locations)")? || !any("SELECT EXISTS (SELECT 1 FROM blobs)")? {
        return Ok(None);
    }
    let path = match cfg.storage {
        StorageKind::Local => std::path::absolute(legacy_files_dir(cfg))?.display().to_string(),
        StorageKind::S3 => format!("s3://{}", cfg.s3_bucket.as_deref().context("HOARD_S3_BUCKET is required for s3")?),
    };
    let new =
        NewLocation { name: Some("Library".to_owned()), mode: Mode::Managed, path, kinds: vec![], catch_all: true };
    insert(c, &new).map(Some)
}

/// Where an install from before library locations has its files: `HOARD_FILES_DIR`, unless that's empty and they're
/// still in `<data-dir>/blobs` (a compose file changed to a separate volume without moving them).
fn legacy_files_dir(cfg: &Config) -> PathBuf {
    let has_files = |p: &Path| std::fs::read_dir(p).is_ok_and(|mut d| d.next().is_some());
    let configured = cfg.blobs_dir();
    let old = cfg.data_dir.join("blobs");
    if configured != old && !has_files(&configured) && has_files(&old) {
        tracing::warn!(
            files_dir = %configured.display(),
            "HOARD_FILES_DIR is empty but the files are still in {}; using that",
            old.display()
        );
        return old;
    }
    configured
}

/// Where the setup suggests the first library goes: the bucket with `HOARD_STORAGE=s3`; in the container image its
/// `/library` volume, whatever an old compose file sets `HOARD_FILES_DIR` to; else `HOARD_FILES_DIR`, else a
/// `library` folder next to the data.
pub(crate) fn suggested_path(cfg: &Config) -> String {
    if let (StorageKind::S3, Some(bucket)) = (cfg.storage, cfg.s3_bucket.as_deref()) {
        return format!("s3://{bucket}");
    }
    if cfg!(unix) && Path::new("/library").is_dir() {
        return "/library".to_owned();
    }
    let p = cfg.files_dir.clone().unwrap_or_else(|| cfg.data_dir.join("library"));
    std::path::absolute(&p).unwrap_or(p).display().to_string()
}

/// Check a new location's folder before adding it: a managed one is made if missing and must be writable, a
/// watched one must exist. Neither may sit inside another location (a managed store in a watched folder would be
/// scanned as packs; two locations over one folder would count files twice).
async fn probe(state: &AppState, new: &NewLocation, others: &[Location]) -> Result<(), AppError> {
    let path = new.path.clone();
    if crate::blobs::s3_bucket(&path).is_some() {
        if new.mode == Mode::Watched {
            return Err(AppError::bad_request(tr!("error-watched-is-folder")));
        }
        let (store, _) = crate::blobs::open_store(&state.cfg, &path).map_err(AppError::bad_request)?;
        if let Some(Err(e)) = store.list(None).next().await {
            return Err(AppError::bad_request(tr!("error-cant-reach", path = path, reason = e.to_string())));
        }
    } else {
        if !Path::new(&path).is_absolute() {
            return Err(AppError::bad_request(tr!("error-folder-full-path")));
        }
        let mode = new.mode;
        let path = path.clone();
        let language = crate::i18n::current();
        tokio::task::spawn_blocking(move || {
            crate::i18n::blocking(language, || -> Result<()> {
                let dir = Path::new(&path);
                match mode {
                    Mode::Managed => {
                        std::fs::create_dir_all(dir).with_context(|| tr!("error-cant-create", path = path))?;
                        let test = dir.join(".hoard-write-test");
                        std::fs::write(&test, b"ok").with_context(|| tr!("error-cant-write", path = path))?;
                        std::fs::remove_file(&test).ok();
                    }
                    Mode::Watched => {
                        std::fs::read_dir(dir).with_context(|| tr!("error-cant-read", path = path))?;
                    }
                }
                Ok(())
            })
        })
        .await
        .map_err(anyhow::Error::from)?
        .map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    }
    let (others, cfg, language) = (others.to_vec(), state.cfg.clone(), crate::i18n::current());
    tokio::task::spawn_blocking(move || crate::i18n::blocking(language, || overlaps(&path, &others, &cfg)))
        .await
        .map_err(anyhow::Error::from)?
        .map_or(Ok(()), |msg| Err(AppError::conflict(msg)))
}

/// What a new location's folder would overlap, if anything. Folders are compared as they really are (`..`, links and
/// letter case resolved), so the same folder typed two ways is still caught.
fn overlaps(path: &str, others: &[Location], cfg: &Config) -> Option<String> {
    let real = |p: &Path| std::fs::canonicalize(p).or_else(|_| std::path::absolute(p)).unwrap_or_else(|_| p.to_owned());
    let nests = |a: &Path, b: &Path| a.starts_with(b) || b.starts_with(a);
    let is_bucket = crate::blobs::s3_bucket(path).is_some();
    let mine = real(Path::new(path));
    for o in others {
        let clash = if is_bucket || crate::blobs::s3_bucket(&o.path).is_some() {
            o.path == path
        } else {
            nests(&mine, &real(Path::new(&o.path)))
        };
        if clash {
            return Some(tr!("error-overlaps-location", name = format!("{:?}", o.name), path = o.path));
        }
    }
    if is_bucket {
        return None;
    }
    let data = real(&cfg.data_dir);
    if nests(&mine, &data) && mine != data.join("library") {
        return Some(tr!("error-overlaps-data", path = cfg.data_dir.display().to_string()));
    }
    for dir in [cfg.thumbs_dir(), cfg.staging_dir(), cfg.tmp_dir()] {
        if nests(&mine, &real(&dir)) {
            return Some(tr!("error-overlaps-cache", path = dir.display().to_string()));
        }
    }
    None
}

/// `GET /api/admin/locations`: the locations, with whether each can be reached, and what the setup suggests.
pub(crate) async fn list_locations(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let (locations, packs) = db::call(&state.pool, |c| {
        let mut stmt = c.prepare("SELECT location_id, COUNT(*) FROM packs WHERE location_id IS NOT NULL GROUP BY 1")?;
        let packs: std::collections::HashMap<i64, i64> =
            stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
        Ok((list(c)?, packs))
    })
    .await?;
    // A disk mounted since the server started is picked up when someone looks.
    if locations.iter().any(|l| l.mode == Mode::Managed && state.blobs.managed_store(l.id).is_none()) {
        state.blobs.refresh().await?;
    }
    let watched: Vec<(i64, String)> =
        locations.iter().filter(|l| l.mode == Mode::Watched).map(|l| (l.id, l.path.clone())).collect();
    let readable: std::collections::HashSet<i64> = tokio::task::spawn_blocking(move || {
        watched.into_iter().filter(|(_, p)| Path::new(p).is_dir()).map(|(id, _)| id).collect()
    })
    .await
    .map_err(anyhow::Error::from)?;
    let rows: Vec<Value> = locations
        .into_iter()
        .map(|l| {
            let online = match l.mode {
                Mode::Managed => state.blobs.managed_store(l.id).is_some(),
                Mode::Watched => readable.contains(&l.id),
            };
            let mut v = serde_json::to_value(&l).unwrap_or_default();
            v["online"] = json!(online);
            v["packs"] = json!(packs.get(&l.id).copied().unwrap_or(0));
            v
        })
        .collect();
    let kinds: Vec<&str> = AssetKind::ALL.iter().map(AssetKind::as_str).collect();
    Ok(Json(json!({ "locations": rows, "suggested": suggested_path(&state.cfg), "kinds": kinds })))
}

/// `POST /api/admin/locations`
pub(crate) async fn create_location(
    State(state): State<AppState>,
    Json(mut new): Json<NewLocation>,
) -> ApiResult<(StatusCode, Json<Location>)> {
    new.path = normalize(&new.path);
    if new.path.is_empty() {
        return Err(AppError::bad_request(tr!("error-pick-folder")));
    }
    new.kinds = check_kinds(&new.kinds)?;
    let others = db::call(&state.pool, |c| list(c)).await?;
    probe(&state, &new, &others).await?;
    let loc = db::call(&state.pool, move |c| insert(c, &new)).await?;
    state.blobs.refresh().await?;
    tracing::info!(name = %loc.name, path = %loc.path, mode = loc.mode.as_str(), "library location added");
    if loc.mode == Mode::Watched {
        let (s, id) = (std::sync::Arc::clone(&state), loc.id);
        db::call(&state.pool, move |c| crate::scan::enqueue(c, &s, id)).await?;
    }
    Ok((StatusCode::CREATED, Json(loc)))
}

/// `POST /api/admin/locations/{id}/scan`: look through a watched folder now rather than at the next rescan.
pub(crate) async fn scan_location(State(state): State<AppState>, UrlPath(id): UrlPath<i64>) -> ApiResult<StatusCode> {
    let loc = db::call(&state.pool, move |c| get(c, id))
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-location-not-found", id = id)))?;
    if loc.mode != Mode::Watched {
        return Err(AppError::bad_request(tr!("error-not-watched", name = loc.name)));
    }
    let s = std::sync::Arc::clone(&state);
    db::call(&state.pool, move |c| crate::scan::enqueue(c, &s, id)).await?;
    Ok(StatusCode::ACCEPTED)
}

/// `PATCH /api/admin/locations/{id}`: rename, change the kinds it takes, or make it the catch-all. Where a
/// location is can't change: add another and move the files.
pub(crate) async fn patch_location(
    State(state): State<AppState>,
    UrlPath(id): UrlPath<i64>,
    Json(patch): Json<LocationPatch>,
) -> ApiResult<Json<Location>> {
    let kinds = patch.kinds.as_deref().map(check_kinds).transpose()?;
    let current = db::call(&state.pool, move |c| get(c, id))
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-location-not-found", id = id)))?;
    if current.mode == Mode::Watched && (kinds.is_some() || patch.catch_all.is_some()) {
        return Err(AppError::bad_request(tr!("error-watched-no-kinds")));
    }
    if current.catch_all && patch.catch_all == Some(false) {
        return Err(AppError::bad_request(tr!("error-keep-catch-all")));
    }
    let loc = db::call(&state.pool, move |c| {
        let Some(loc) = get(c, id)? else { return Ok(None) };
        let tx = c.unchecked_transaction()?;
        if let Some(name) = patch.name.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
            tx.execute("UPDATE locations SET name = ?1 WHERE id = ?2", params![name, id])?;
        }
        if loc.mode == Mode::Managed {
            if let Some(kinds) = kinds {
                tx.execute(
                    "UPDATE locations SET kinds = ?1 WHERE id = ?2",
                    params![serde_json::to_string(&kinds)?, id],
                )?;
            }
            if patch.catch_all == Some(true) {
                tx.execute("UPDATE locations SET catch_all = (id = ?1)", [id])?;
            }
        }
        tx.commit()?;
        get(c, id)
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-location-not-found", id = id)))?;
    state.blobs.refresh().await?;
    Ok(Json(loc))
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct DeleteQuery {
    /// Remove a managed location that can't be reached anyway (a dead disk): files still on it go missing.
    #[serde(default)]
    force: bool,
}

/// `DELETE /api/admin/locations/{id}[?force=true]`: a watched location goes with its packs (the folder itself is
/// left alone); a managed one once its files have been moved out (409 while it holds any). One that can't be reached
/// is refused with 503, since Hoard can't tell what's on it, unless forced.
pub(crate) async fn delete_location(
    State(state): State<AppState>,
    UrlPath(id): UrlPath<i64>,
    Query(q): Query<DeleteQuery>,
) -> ApiResult<StatusCode> {
    let loc = db::call(&state.pool, move |c| get(c, id))
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-location-not-found", id = id)))?;
    if loc.mode == Mode::Managed {
        match state.blobs.managed_store(id) {
            Some(store) => {
                if crate::blobs::holds_any(&*store).await? {
                    return Err(AppError::conflict(tr!("error-location-holds-files", name = loc.name, id = id)));
                }
            }
            None if q.force => {
                tracing::warn!(name = %loc.name, path = %loc.path, "removing a library location that can't be reached; files on it are missing now");
            }
            None => {
                return Err(AppError::unavailable(tr!("error-location-unreachable", name = loc.name)));
            }
        }
    }
    db::call(&state.pool, move |c| remove(c, id)).await?;
    state.blobs.refresh().await?;
    tracing::info!(name = %loc.name, path = %loc.path, "library location removed");
    Ok(StatusCode::NO_CONTENT)
}

/// Queue moving stored files between managed locations, unless a move is already waiting or running.
async fn queue_relocate(state: &AppState, req: crate::relocate::Request) -> ApiResult<()> {
    let queued = db::call(&state.pool, move |c| {
        let waiting = db::job_pending(c, crate::jobs::RELOCATE, None, None)?;
        if !waiting {
            db::enqueue(c, crate::jobs::RELOCATE, None, &serde_json::to_value(req)?)?;
        }
        Ok(!waiting)
    })
    .await?;
    if !queued {
        return Err(AppError::conflict(tr!("error-relocate-running")));
    }
    state.wake.notify_one();
    Ok(())
}

/// `POST /api/admin/locations/{id}/empty`: move a managed location's files to where they'd go without it, then
/// remove it. Runs as a job.
pub(crate) async fn empty_location(State(state): State<AppState>, UrlPath(id): UrlPath<i64>) -> ApiResult<StatusCode> {
    let loc = db::call(&state.pool, move |c| get(c, id))
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-location-not-found", id = id)))?;
    if loc.mode != Mode::Managed {
        return Err(AppError::bad_request(tr!("error-watched-not-moved", name = loc.name)));
    }
    let others = state.blobs.managed_stores().iter().filter(|(i, _)| *i != id).count();
    if others == 0 {
        return Err(AppError::conflict(tr!("error-no-other-location")));
    }
    queue_relocate(&state, crate::relocate::Request { from: Some(id), remove: true }).await?;
    Ok(StatusCode::ACCEPTED)
}

/// `POST /api/admin/locations/rebalance`: move stored files to the locations their kinds go to now. Runs as a job.
pub(crate) async fn rebalance(State(state): State<AppState>) -> ApiResult<StatusCode> {
    queue_relocate(&state, crate::relocate::Request::default()).await?;
    Ok(StatusCode::ACCEPTED)
}

/// Remove a location's row, with the packs of a watched one. A managed one's files must have been moved out first.
pub(crate) fn remove(c: &mut Connection, id: i64) -> Result<()> {
    let packs: Vec<String> = c
        .prepare("SELECT id FROM packs WHERE location_id = ?1")?
        .query_map([id], |r| r.get(0))?
        .collect::<rusqlite::Result<_>>()?;
    for p in &packs {
        db::delete_pack(c, p)?;
    }
    c.execute("DELETE FROM locations WHERE id = ?1", [id])?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub(crate) struct BrowseQuery {
    #[serde(default)]
    path: String,
}

/// `GET /api/admin/fs?path=`: the folders in a folder on the server, for picking a location. No path lists the
/// drives on Windows and `/` elsewhere.
pub(crate) async fn browse(Query(q): Query<BrowseQuery>) -> ApiResult<Json<Value>> {
    let language = crate::i18n::current();
    let v = tokio::task::spawn_blocking(move || crate::i18n::blocking(language, || browse_dir(q.path.trim())))
        .await
        .map_err(anyhow::Error::from)?
        .map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    Ok(Json(v))
}

fn browse_dir(path: &str) -> Result<Value> {
    if path.is_empty() {
        if cfg!(windows) {
            let dirs: Vec<Value> = ('A'..='Z')
                .map(|d| format!("{d}:\\"))
                .filter(|p| Path::new(p).is_dir())
                .map(|p| json!({ "name": p, "path": p }))
                .collect();
            return Ok(json!({ "path": "", "parent": null, "dirs": dirs }));
        }
        return browse_dir("/");
    }
    let dir = Path::new(path);
    if !dir.is_absolute() {
        bail!(tr!("error-full-path"));
    }
    let mut dirs: Vec<(String, String)> = Vec::new();
    for entry in std::fs::read_dir(dir).with_context(|| tr!("error-cant-read", path = path))? {
        let Ok(entry) = entry else { continue };
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') || !entry.path().is_dir() {
            continue;
        }
        dirs.push((name, entry.path().display().to_string()));
    }
    dirs.sort_by_key(|(n, _)| n.to_lowercase());
    dirs.truncate(5000);
    // A drive's root goes up to the list of drives.
    let parent = match dir.parent() {
        Some(p) => Some(p.display().to_string()),
        None if cfg!(windows) => Some(String::new()),
        None => None,
    };
    let dirs: Vec<Value> = dirs.into_iter().map(|(name, path)| json!({ "name": name, "path": path })).collect();
    Ok(json!({ "path": dir.display().to_string(), "parent": parent, "dirs": dirs }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blobs::tests::setup;

    #[test]
    fn the_first_managed_location_is_the_catch_all_until_another_takes_over() {
        let dir = tempfile::tempdir().unwrap();
        let (_cfg, pool) = setup(dir.path());
        let c = pool.get().unwrap();
        let new = |name: &str, mode, catch_all| NewLocation {
            name: Some(name.to_owned()),
            mode,
            path: dir.path().join(name).display().to_string(),
            kinds: vec!["audio".to_owned()],
            catch_all,
        };
        let watched = insert(&c, &new("packs", Mode::Watched, true)).unwrap();
        assert!(!watched.catch_all && watched.kinds.is_empty());
        let a = insert(&c, &new("a", Mode::Managed, false)).unwrap();
        assert!(a.catch_all);
        let b = insert(&c, &new("b", Mode::Managed, false)).unwrap();
        assert!(!b.catch_all);
        insert(&c, &new("c", Mode::Managed, true)).unwrap();
        let all = list(&c).unwrap();
        assert_eq!(all.iter().filter(|l| l.catch_all).map(|l| l.name.as_str()).collect::<Vec<_>>(), ["c"]);
    }

    #[test]
    fn an_install_from_before_locations_keeps_its_files_where_they_are() {
        let dir = tempfile::tempdir().unwrap();
        let (cfg, pool) = setup(dir.path());
        let c = pool.get().unwrap();
        assert!(adopt_existing(&c, &cfg).unwrap().is_none(), "a new server is asked instead");
        c.execute("INSERT INTO blobs (hash, size, created_at) VALUES ('x', 1, 'now')", []).unwrap();
        let adopted = adopt_existing(&c, &cfg).unwrap().unwrap();
        assert!(adopted.catch_all && adopted.mode == Mode::Managed);
        assert_eq!(Path::new(&adopted.path), std::path::absolute(cfg.blobs_dir()).unwrap());
        assert!(adopt_existing(&c, &cfg).unwrap().is_none(), "only once");
    }

    #[test]
    fn paths_are_tidied() {
        assert_eq!(normalize(" /library/ "), "/library");
        assert_eq!(normalize("/"), "/");
        assert_eq!(normalize("C:\\"), "C:\\");
        assert_eq!(normalize("D:\\Assets\\"), "D:\\Assets");
        assert_eq!(normalize("s3://bucket/"), "s3://bucket");
    }

    #[test]
    fn watched_files_are_found_under_the_pack_folder() {
        let p = watched_path("/packs", Some("Forest Kit"), "Textures/bark.png");
        assert_eq!(p, Path::new("/packs").join("Forest Kit").join("Textures").join("bark.png"));
    }
}
