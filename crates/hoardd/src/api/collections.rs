//! Collections.

use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{AppState, db};
use anyhow::Context;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use hoard_core::{Collection, CollectionMembers, CollectionPatch};
use rusqlite::params;

pub(super) async fn list_collections(State(state): State<AppState>) -> ApiResult<Json<Vec<Collection>>> {
    Ok(Json(db::call(&state.pool, |c| db::list_collections(c)).await?))
}

pub(super) async fn get_collection(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Collection>> {
    let key = id.clone();
    db::call(&state.pool, move |c| db::get_collection(c, &key))
        .await?
        .map(Json)
        .ok_or_else(|| AppError::not_found(tr!("error-collection-not-found", id = id)))
}

fn clean_name(name: Option<String>) -> Option<String> {
    name.map(|n| n.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(80).collect::<String>())
        .filter(|n| !n.is_empty())
}

pub(super) async fn create_collection(
    State(state): State<AppState>,
    Json(body): Json<CollectionPatch>,
) -> ApiResult<(StatusCode, Json<Collection>)> {
    let name = clean_name(body.name).ok_or_else(|| AppError::bad_request(tr!("error-collection-needs-name")))?;
    let description =
        body.description.map(|d| d.trim().chars().take(1000).collect::<String>()).filter(|d| !d.is_empty());
    let created = db::call(&state.pool, move |c| {
        if db::get_collection(c, &name)?.is_some() {
            return Ok(Err(name));
        }
        let id = uuid::Uuid::now_v7().to_string();
        let now = db::now();
        c.execute(
            "INSERT INTO collections (id, name, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
            params![id, name, description, now],
        )?;
        Ok(Ok(db::get_collection(c, &id)?.context("collection vanished")?))
    })
    .await?;
    match created {
        Ok(c) => Ok((StatusCode::CREATED, Json(c))),
        Err(name) => Err(AppError::conflict(tr!("error-collection-exists", name = name))),
    }
}

pub(super) async fn patch_collection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CollectionPatch>,
) -> ApiResult<Json<Collection>> {
    let name = clean_name(body.name);
    let description = body.description.map(|d| d.trim().chars().take(1000).collect::<String>());
    let key = id.clone();
    let updated = db::call(&state.pool, move |c| {
        let Some(existing) = db::get_collection(c, &key)? else { return Ok(None) };
        if let Some(n) = &name
            && db::get_collection(c, n)?.is_some_and(|other| other.id != existing.id)
        {
            return Ok(Some(Err(n.clone())));
        }
        c.execute(
            "UPDATE collections SET name = COALESCE(?2, name),
                    description = CASE WHEN ?3 IS NULL THEN description WHEN ?3 = '' THEN NULL ELSE ?3 END,
                    updated_at = ?4 WHERE id = ?1",
            params![existing.id, name, description, db::now()],
        )?;
        Ok(Some(Ok(db::get_collection(c, &existing.id)?.context("collection vanished")?)))
    })
    .await?;
    match updated {
        None => Err(AppError::not_found(tr!("error-collection-not-found", id = id))),
        Some(Err(name)) => Err(AppError::conflict(tr!("error-collection-exists", name = name))),
        Some(Ok(c)) => Ok(Json(c)),
    }
}

pub(super) async fn delete_collection(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let key = id.clone();
    let n = db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM collections WHERE id = ?1", [&key])?)).await?;
    if n == 0 {
        Err(AppError::not_found(tr!("error-collection-not-found", id = id)))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub(super) async fn collection_members(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CollectionMembers>,
) -> ApiResult<Json<Collection>> {
    if body.add.len() + body.remove.len() > 10_000 {
        return Err(AppError::bad_request(tr!("error-too-many-assets")));
    }
    let key = id.clone();
    let updated = db::call(&state.pool, move |c| {
        let Some(existing) = db::get_collection(c, &key)? else { return Ok(None) };
        let now = db::now();
        let tx = c.transaction()?;
        for asset in &body.add {
            tx.execute(
                "INSERT OR IGNORE INTO collection_assets (collection_id, asset_id, added_at)
                 SELECT ?1, id, ?3 FROM assets WHERE id = ?2",
                params![existing.id, asset, now],
            )?;
        }
        for asset in &body.remove {
            tx.execute(
                "DELETE FROM collection_assets WHERE collection_id = ?1 AND asset_id = ?2",
                params![existing.id, asset],
            )?;
        }
        tx.execute("UPDATE collections SET updated_at = ?2 WHERE id = ?1", params![existing.id, now])?;
        tx.commit()?;
        db::get_collection(c, &existing.id)
    })
    .await?;
    updated.map(Json).ok_or_else(|| AppError::not_found(tr!("error-collection-not-found", id = id)))
}
