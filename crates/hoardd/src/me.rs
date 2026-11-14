//! What's each person's own in the library: the assets they've starred (favourites) and the searches they've saved.
//! Looking is enough for both, since neither changes the library. An access token or connected app acts for the
//! account that made it.

use crate::i18n::tr;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get};
use axum::{Extension, Json, Router};
use rusqlite::{Connection, params};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::auth::Who;
use crate::db;
use crate::error::{ApiResult, AppError};

/// Behind [`crate::auth::require_user`].
pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/me/favourites", get(list_favourites).post(change_favourites))
        .route("/api/me/searches", get(list_searches).post(save_search))
        .route("/api/me/searches/{id}", delete(delete_search))
}

/// The person `who` is, or acts for.
fn owner(who: &Who) -> ApiResult<i64> {
    who.owner().ok_or_else(|| AppError::forbidden(tr!("error-token-account-gone")))
}

/// A person's favourite assets, most recently starred first.
pub(crate) fn favourite_ids(c: &Connection, user: i64) -> anyhow::Result<Vec<String>> {
    let mut stmt =
        c.prepare("SELECT asset_id FROM favourites WHERE user_id = ?1 ORDER BY added_at DESC, rowid DESC")?;
    Ok(stmt.query_map([user], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?)
}

async fn list_favourites(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    let ids = db::call(&state.pool, move |c| favourite_ids(c, user)).await?;
    Ok(Json(json!({ "assets": ids })))
}

#[derive(Debug, Deserialize)]
struct FavouriteChange {
    #[serde(default)]
    add: Vec<String>,
    #[serde(default)]
    remove: Vec<String>,
}

/// Star or unstar assets; answers with all of them. Ids that aren't assets are passed over.
async fn change_favourites(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(change): Json<FavouriteChange>,
) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    if change.add.len() + change.remove.len() > 10_000 {
        return Err(AppError::bad_request(tr!("error-favourites-too-many")));
    }
    let ids = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let now = db::now();
        for id in &change.add {
            tx.execute(
                "INSERT OR IGNORE INTO favourites (user_id, asset_id, added_at)
                 SELECT ?1, id, ?3 FROM assets WHERE id = ?2",
                params![user, id, now],
            )?;
        }
        for id in &change.remove {
            tx.execute("DELETE FROM favourites WHERE user_id = ?1 AND asset_id = ?2", params![user, id])?;
        }
        let ids = favourite_ids(&tx, user)?;
        tx.commit()?;
        Ok(ids)
    })
    .await?;
    Ok(Json(json!({ "assets": ids })))
}

async fn list_searches(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    let searches = db::call(&state.pool, move |c| searches(c, user)).await?;
    Ok(Json(json!({ "searches": searches })))
}

fn searches(c: &Connection, user: i64) -> anyhow::Result<Vec<Value>> {
    let mut stmt = c.prepare(
        "SELECT id, name, query, created_at FROM saved_searches WHERE user_id = ?1 ORDER BY name COLLATE NOCASE",
    )?;
    Ok(stmt
        .query_map([user], |r| {
            Ok(json!({ "id": r.get::<_, i64>(0)?, "name": r.get::<_, String>(1)?,
                       "query": r.get::<_, String>(2)?, "created_at": r.get::<_, String>(3)? }))
        })?
        .collect::<rusqlite::Result<_>>()?)
}

/// A search to keep: its name and the portal's query string (`q=rock&kind=texture`).
#[derive(Debug, Deserialize)]
struct NewSearch {
    name: String,
    query: String,
}

/// Save a search; one with the same name is replaced. Answers with all of them.
async fn save_search(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(s): Json<NewSearch>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let user = owner(&who)?;
    let name = s.name.trim().to_owned();
    let query = s.query.trim().trim_start_matches('?').to_owned();
    if name.is_empty() || name.chars().count() > 80 {
        return Err(AppError::bad_request(tr!("error-saved-search-name")));
    }
    if query.len() > 2000 {
        return Err(AppError::bad_request(tr!("error-saved-search-too-long")));
    }
    let all = db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO saved_searches (user_id, name, query, created_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT (user_id, name) DO UPDATE SET query = excluded.query",
            params![user, name, query, db::now()],
        )?;
        searches(c, user)
    })
    .await?;
    Ok((StatusCode::CREATED, Json(json!({ "searches": all }))))
}

async fn delete_search(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    let user = owner(&who)?;
    let n = db::call(&state.pool, move |c| {
        Ok(c.execute("DELETE FROM saved_searches WHERE id = ?1 AND user_id = ?2", params![id, user])?)
    })
    .await?;
    if n == 0 {
        return Err(AppError::not_found(tr!("error-saved-search-not-found", id = id)));
    }
    Ok(StatusCode::NO_CONTENT)
}
