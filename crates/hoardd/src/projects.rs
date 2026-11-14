//! Where assets are used: each `hoard sync` tells the server what its project's `hoard.lock` holds, so an asset's
//! details can list the projects that have it, the library can be filtered to one project's assets, and archiving or
//! deleting something a project uses asks first (its next sync would take the files out).
//!
//! A project is known by its name (`project` in `hoard.toml`, or its folder's name): everyone syncing the same name
//! reports into the same project, and each report replaces what it had.

use crate::i18n::tr;
use std::collections::BTreeMap;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::AppState;
use crate::auth::Who;
use crate::db;
use crate::error::{ApiResult, AppError};

/// What anyone who can browse may see. Behind [`crate::auth::require_reader`].
pub(crate) fn read_router() -> Router<AppState> {
    Router::new().route("/api/projects", get(list)).route("/api/usage", post(usage))
}

/// Reporting a sync: looking is enough, since it changes nothing in the library. Behind
/// [`crate::auth::require_user`].
pub(crate) fn report_router() -> Router<AppState> {
    Router::new().route("/api/projects/{name}", put(report))
}

/// Forgetting a project (by name). Behind [`crate::auth::require_editor`].
pub(crate) fn edit_router() -> Router<AppState> {
    Router::new().route("/api/projects/{name}", delete(forget))
}

#[derive(Debug, Serialize)]
struct Project {
    id: i64,
    name: String,
    synced_at: String,
    /// Who reported the last sync.
    synced_by: Option<String>,
    /// How many assets it has.
    assets: u64,
}

async fn list(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let projects = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT p.id, p.name, p.synced_at, p.synced_by, (SELECT COUNT(*) FROM project_assets pa WHERE pa.project_id = p.id)
               FROM projects p ORDER BY p.name COLLATE NOCASE",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(Project {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    synced_at: r.get(2)?,
                    synced_by: r.get(3)?,
                    assets: r.get::<_, i64>(4)?.unsigned_abs(),
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!({ "projects": projects })))
}

#[derive(Debug, Deserialize)]
struct Report {
    assets: Vec<ReportedAsset>,
}

#[derive(Debug, Deserialize)]
struct ReportedAsset {
    id: String,
    /// Where the project keeps it, relative to its `out_dir`.
    path: String,
}

/// What a project's `hoard.lock` holds now; replaces what it held. Ids that aren't (or are no longer) assets are
/// counted and passed over.
async fn report(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(name): Path<String>,
    Json(report): Json<Report>,
) -> ApiResult<Json<Value>> {
    let name = name.trim().to_owned();
    if name.is_empty() || name.chars().count() > 200 {
        return Err(AppError::bad_request(tr!("error-project-name")));
    }
    if report.assets.len() > 1_000_000 {
        return Err(AppError::bad_request(tr!("error-project-too-big")));
    }
    let by = match &who {
        Who::User { name, .. } | Who::Token { name, .. } => name.clone(),
    };
    let (id, known, unknown) = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let now = db::now();
        tx.execute(
            "INSERT INTO projects (name, synced_at, synced_by) VALUES (?1, ?2, ?3)
             ON CONFLICT (name) DO UPDATE SET synced_at = excluded.synced_at, synced_by = excluded.synced_by",
            params![name, now, by],
        )?;
        let id: i64 = tx.query_row("SELECT id FROM projects WHERE name = ?1", [&name], |r| r.get(0))?;
        tx.execute("DELETE FROM project_assets WHERE project_id = ?1", [id])?;
        let (mut known, mut unknown) = (0u64, 0u64);
        {
            let mut insert = tx.prepare(
                "INSERT OR IGNORE INTO project_assets (project_id, asset_id, path)
                 SELECT ?1, id, ?3 FROM assets WHERE id = ?2",
            )?;
            for a in &report.assets {
                if insert.execute(params![id, a.id, a.path])? > 0 {
                    known += 1;
                } else if !asset_exists(&tx, &a.id)? {
                    unknown += 1;
                }
            }
        }
        tx.commit()?;
        Ok((id, known, unknown))
    })
    .await?;
    Ok(Json(json!({ "id": id, "assets": known, "unknown": unknown })))
}

fn asset_exists(c: &Connection, id: &str) -> rusqlite::Result<bool> {
    c.query_row("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1)", [id], |r| r.get(0))
}

async fn forget(State(state): State<AppState>, Path(name): Path<String>) -> ApiResult<StatusCode> {
    let name2 = name.clone();
    let n = db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM projects WHERE name = ?1", [name2])?)).await?;
    if n == 0 {
        return Err(AppError::not_found(tr!("error-project-not-found", name = name)));
    }
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Default, Deserialize)]
struct UsageRequest {
    /// Whole packs: every asset in them.
    #[serde(default)]
    packs: Vec<String>,
    #[serde(default)]
    assets: Vec<String>,
    /// Everything in the archive (what emptying it would delete).
    #[serde(default)]
    archived: bool,
}

#[derive(Debug, Serialize)]
struct Usage {
    id: i64,
    name: String,
    synced_at: String,
    synced_by: Option<String>,
    /// How many of the assets asked about it has.
    assets: u64,
    /// A few of them and where the project keeps them.
    examples: Vec<UsageExample>,
}

#[derive(Debug, Serialize)]
struct UsageExample {
    asset_id: String,
    name: String,
    path: String,
}

/// Which projects use any of these packs or assets, most-using first.
async fn usage(State(state): State<AppState>, Json(req): Json<UsageRequest>) -> ApiResult<Json<Value>> {
    if req.packs.len() + req.assets.len() > 100_000 {
        return Err(AppError::bad_request(tr!("error-project-too-many-at-once")));
    }
    let projects = db::call(&state.pool, move |c| projects_using(c, &req)).await?;
    Ok(Json(json!({ "projects": projects })))
}

fn projects_using(c: &Connection, req: &UsageRequest) -> anyhow::Result<Vec<Usage>> {
    // The assets asked about, gathered in a temporary table so long lists don't hit SQLite's parameter limit.
    c.execute_batch("CREATE TEMP TABLE IF NOT EXISTS usage_ask (asset_id TEXT PRIMARY KEY); DELETE FROM usage_ask;")?;
    {
        let mut add = c.prepare("INSERT OR IGNORE INTO usage_ask VALUES (?1)")?;
        for id in &req.assets {
            add.execute([id])?;
        }
        let mut add_pack = c.prepare("INSERT OR IGNORE INTO usage_ask SELECT id FROM assets WHERE pack_id = ?1")?;
        for id in &req.packs {
            add_pack.execute([id])?;
        }
        if req.archived {
            c.execute(
                "INSERT OR IGNORE INTO usage_ask SELECT a.id FROM assets a
                  WHERE a.archived_at IS NOT NULL OR a.pack_id IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)",
                [],
            )?;
        }
    }
    let mut out: Vec<Usage> = Vec::new();
    let mut at: BTreeMap<i64, usize> = BTreeMap::new();
    let mut stmt = c.prepare(
        "SELECT p.id, p.name, p.synced_at, p.synced_by, COUNT(*)
           FROM project_assets pa JOIN usage_ask q ON q.asset_id = pa.asset_id JOIN projects p ON p.id = pa.project_id
          GROUP BY p.id ORDER BY COUNT(*) DESC, p.name COLLATE NOCASE",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Usage {
            id: r.get(0)?,
            name: r.get(1)?,
            synced_at: r.get(2)?,
            synced_by: r.get(3)?,
            assets: r.get::<_, i64>(4)?.unsigned_abs(),
            examples: Vec::new(),
        })
    })?;
    for u in rows {
        let u = u?;
        at.insert(u.id, out.len());
        out.push(u);
    }
    let mut ex = c.prepare(
        "SELECT pa.asset_id, a.name, pa.path FROM project_assets pa
           JOIN usage_ask q ON q.asset_id = pa.asset_id JOIN assets a ON a.id = pa.asset_id
          WHERE pa.project_id = ?1 ORDER BY pa.path LIMIT 3",
    )?;
    for (id, i) in &at {
        out[*i].examples = ex
            .query_map([id], |r| Ok(UsageExample { asset_id: r.get(0)?, name: r.get(1)?, path: r.get(2)? }))?
            .collect::<rusqlite::Result<_>>()?;
    }
    c.execute_batch("DELETE FROM usage_ask;")?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_counts_the_assets_asked_about_by_project() -> anyhow::Result<()> {
        let c = Connection::open_in_memory()?;
        c.execute_batch(
            "CREATE TABLE packs (id TEXT PRIMARY KEY, archived_at TEXT);
             CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, name TEXT, archived_at TEXT);
             CREATE TABLE projects (id INTEGER PRIMARY KEY, name TEXT, synced_at TEXT, synced_by TEXT);
             CREATE TABLE project_assets (project_id INTEGER, asset_id TEXT, path TEXT);
             INSERT INTO packs VALUES ('props', NULL), ('rocks', 'then');
             INSERT INTO assets VALUES ('crate', 'props', 'Crate', NULL), ('barrel', 'props', 'Barrel', NULL),
               ('rock', 'rocks', 'Rock', NULL);
             INSERT INTO projects VALUES (1, 'Forest', 'now', 'ann'), (2, 'Boss', 'now', NULL);
             INSERT INTO project_assets VALUES (1, 'crate', 'props/crate.obj'), (1, 'barrel', 'props/barrel.obj'),
               (2, 'crate', 'crate.obj'), (2, 'rock', 'rock.obj');",
        )?;
        let names = |u: &[Usage]| u.iter().map(|u| (u.name.clone(), u.assets)).collect::<Vec<_>>();
        let by_pack = projects_using(&c, &UsageRequest { packs: vec!["props".into()], ..Default::default() })?;
        assert_eq!(names(&by_pack), [("Forest".to_owned(), 2), ("Boss".to_owned(), 1)]);
        assert_eq!(by_pack[0].examples.len(), 2);
        let one = projects_using(&c, &UsageRequest { assets: vec!["barrel".into()], ..Default::default() })?;
        assert_eq!(names(&one), [("Forest".to_owned(), 1)]);
        assert_eq!(one[0].examples[0].path, "props/barrel.obj");
        let archived = projects_using(&c, &UsageRequest { archived: true, ..Default::default() })?;
        assert_eq!(names(&archived), [("Boss".to_owned(), 1)]);
        assert!(projects_using(&c, &UsageRequest::default())?.is_empty());
        Ok(())
    }
}
