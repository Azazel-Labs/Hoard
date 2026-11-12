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
    let mode: String = r.get(2)?;
    let kinds: String = r.get(4)?;
    Ok(Location {
        id: r.get(0)?,
        name: r.get(1)?,
        mode: if mode == "watched" { Mode::Watched } else { Mode::Managed },
        path: r.get(3)?,
        kinds: serde_json::from_str(&kinds).unwrap_or_default(),
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
            return Err(AppError::bad_request(format!("unknown asset kind {k:?}")));
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
pub(crate) fn insert(c: &Connection, new: &NewLocation) -> Result<Location> {
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
    get(c, c.last_insert_rowid())?.context("location just added is gone")
}

/// An install from before library locations keeps its files where they are: they become its one managed
/// location, the catch-all. A new server gets nothing, and the portal asks.
pub(crate) fn adopt_existing(c: &Connection, cfg: &Config) -> Result<Option<Location>> {
    let any = |sql: &str| -> Result<bool> { Ok(c.query_row(sql, [], |r| r.get(0))?) };
    if any("SELECT EXISTS (SELECT 1 FROM locations)")? || !any("SELECT EXISTS (SELECT 1 FROM blobs)")? {
        return Ok(None);
    }
    let path = match cfg.storage {
        StorageKind::Local => std::path::absolute(cfg.blobs_dir())?.display().to_string(),
        StorageKind::S3 => format!("s3://{}", cfg.s3_bucket.as_deref().context("HOARD_S3_BUCKET is required for s3")?),
    };
    let new =
        NewLocation { name: Some("Library".to_owned()), mode: Mode::Managed, path, kinds: vec![], catch_all: true };
    insert(c, &new).map(Some)
}

/// Where the setup suggests the first library goes: the bucket with `HOARD_STORAGE=s3`, else `HOARD_FILES_DIR`
/// (`/library` in the container image), else a `library` folder next to the data.
pub(crate) fn suggested_path(cfg: &Config) -> String {
    if let (StorageKind::S3, Some(bucket)) = (cfg.storage, cfg.s3_bucket.as_deref()) {
        return format!("s3://{bucket}");
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
            return Err(AppError::bad_request("a watched location is a folder on this server"));
        }
        let (store, _) = crate::blobs::open_store(&state.cfg, &path).map_err(AppError::bad_request)?;
        if let Some(Err(e)) = store.list(None).next().await {
            return Err(AppError::bad_request(format!("can't reach {path}: {e}")));
        }
    } else {
        if !Path::new(&path).is_absolute() {
            return Err(AppError::bad_request("give the folder's full path"));
        }
        let mode = new.mode;
        tokio::task::spawn_blocking(move || -> Result<()> {
            let dir = Path::new(&path);
            match mode {
                Mode::Managed => {
                    std::fs::create_dir_all(dir).with_context(|| format!("can't create {path}"))?;
                    let test = dir.join(".hoard-write-test");
                    std::fs::write(&test, b"ok").with_context(|| format!("can't write to {path}"))?;
                    std::fs::remove_file(&test).ok();
                }
                Mode::Watched => {
                    std::fs::read_dir(dir).with_context(|| format!("can't read {path}"))?;
                }
            }
            Ok(())
        })
        .await
        .map_err(anyhow::Error::from)?
        .map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    }
    let mine = Path::new(&new.path);
    for o in others {
        let theirs = Path::new(&o.path);
        if o.path == new.path || mine.starts_with(theirs) || theirs.starts_with(mine) {
            return Err(AppError::conflict(format!("that overlaps the location {:?} ({})", o.name, o.path)));
        }
    }
    let data = std::path::absolute(&state.cfg.data_dir).unwrap_or_else(|_| state.cfg.data_dir.clone());
    if mine.is_absolute() && (data.starts_with(mine) || mine.starts_with(&data)) && mine != data.join("library") {
        return Err(AppError::conflict(format!("that overlaps Hoard's data folder ({})", data.display())));
    }
    Ok(())
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
    let rows: Vec<Value> = locations
        .into_iter()
        .map(|l| {
            let online = match l.mode {
                Mode::Managed => state.blobs.managed_store(l.id).is_some(),
                Mode::Watched => Path::new(&l.path).is_dir(),
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
        return Err(AppError::bad_request("pick a folder"));
    }
    new.kinds = check_kinds(&new.kinds)?;
    let others = db::call(&state.pool, |c| list(c)).await?;
    probe(&state, &new, &others).await?;
    let loc = db::call(&state.pool, move |c| insert(c, &new)).await?;
    state.blobs.reload()?;
    tracing::info!(name = %loc.name, path = %loc.path, mode = loc.mode.as_str(), "library location added");
    Ok((StatusCode::CREATED, Json(loc)))
}

/// `PATCH /api/admin/locations/{id}`: rename, change the kinds it takes, or make it the catch-all. Where a
/// location is can't change: add another and move the files.
pub(crate) async fn patch_location(
    State(state): State<AppState>,
    UrlPath(id): UrlPath<i64>,
    Json(patch): Json<LocationPatch>,
) -> ApiResult<Json<Location>> {
    let kinds = patch.kinds.as_deref().map(check_kinds).transpose()?;
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
    .ok_or_else(|| AppError::not_found(format!("location {id}")))?;
    state.blobs.reload()?;
    Ok(Json(loc))
}

/// `DELETE /api/admin/locations/{id}`: a watched location goes with its packs (the folder itself is left alone);
/// a managed one only once its files have been moved out.
pub(crate) async fn delete_location(State(state): State<AppState>, UrlPath(id): UrlPath<i64>) -> ApiResult<StatusCode> {
    let loc = db::call(&state.pool, move |c| get(c, id))
        .await?
        .ok_or_else(|| AppError::not_found(format!("location {id}")))?;
    if loc.mode == Mode::Managed
        && let Some(store) = state.blobs.managed_store(id)
        && store.list(None).next().await.is_some()
    {
        return Err(AppError::conflict(format!(
            "{} still holds files: move them to another location before removing it",
            loc.name
        )));
    }
    db::call(&state.pool, move |c| {
        let packs: Vec<String> = c
            .prepare("SELECT id FROM packs WHERE location_id = ?1")?
            .query_map([id], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        for p in &packs {
            db::delete_pack(c, p)?;
        }
        c.execute("DELETE FROM locations WHERE id = ?1", [id])?;
        Ok(())
    })
    .await?;
    state.blobs.reload()?;
    tracing::info!(name = %loc.name, path = %loc.path, "library location removed");
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub(crate) struct BrowseQuery {
    #[serde(default)]
    path: String,
}

/// `GET /api/admin/fs?path=`: the folders in a folder on the server, for picking a location. No path lists the
/// drives on Windows and `/` elsewhere.
pub(crate) async fn browse(Query(q): Query<BrowseQuery>) -> ApiResult<Json<Value>> {
    let v = tokio::task::spawn_blocking(move || browse_dir(q.path.trim()))
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
        bail!("give a full path");
    }
    let mut dirs: Vec<(String, String)> = Vec::new();
    for entry in std::fs::read_dir(dir).with_context(|| format!("can't read {path}"))? {
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
