//! Packs: listing, uploading, editing, retagging and deleting them, and what they're about.

use std::collections::BTreeSet;

use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::ingest::safe_rel_path;
use crate::{AppState, context, db, jobs};
use anyhow::Context;
use axum::Json;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use hoard_core::{Pack, PackContext};
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;
use tokio::io::AsyncWriteExt;

pub(super) async fn get_pack_context(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<PackContext>> {
    let id2 = id.clone();
    let ctx =
        db::call(&state.pool, move |c| db::get_pack(c, &id2)?.is_some().then(|| db::pack_context(c, &id2)).transpose())
            .await?;
    ctx.map(Json).ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))
}

/// Set what the seller says about a pack. `readme` is captured at ingest and ignored here. Giving a store URL
/// without store text fetches the page's title and description (best effort) so the tagger has something to read.
pub(super) async fn put_pack_context(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(mut ctx): Json<PackContext>,
) -> ApiResult<Json<PackContext>> {
    let clean = |s: Option<String>, max: usize| {
        s.map(|s| s.trim().chars().take(max).collect::<String>()).filter(|s| !s.is_empty())
    };
    ctx.store = clean(ctx.store, 80);
    ctx.store_url = clean(ctx.store_url, 500).filter(|u| u.starts_with("https://") || u.starts_with("http://"));
    ctx.store_text = clean(ctx.store_text, context::MAX_STORE_TEXT);
    ctx.license_tier = clean(ctx.license_tier, 80);
    ctx.seller = clean(ctx.seller, 120);
    ctx.store_license = clean(ctx.store_license, 80);
    if ctx.store.is_none() {
        ctx.store = ctx.store_url.as_deref().and_then(context::store_from_url).map(str::to_owned);
    }
    if ctx.store_text.is_none()
        && let Some(url) = &ctx.store_url
    {
        match context::fetch_store_text(context::web(), url).await {
            Ok(text) => ctx.store_text = text,
            Err(e) => tracing::info!(url, error = %format!("{e:#}"), "couldn't fetch store page"),
        }
    }
    let id2 = id.clone();
    let saved = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(None);
        }
        // A page someone changed is no longer the one found automatically.
        c.execute(
            "INSERT INTO pack_context (pack_id, store, store_url, store_text, license_tier, seller, store_license)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT (pack_id) DO UPDATE SET store = excluded.store, store_url = excluded.store_url,
                 store_text = excluded.store_text, license_tier = excluded.license_tier, seller = excluded.seller,
                 store_license = excluded.store_license,
                 store_auto = CASE WHEN store_url IS excluded.store_url THEN store_auto END",
            params![id2, ctx.store, ctx.store_url, ctx.store_text, ctx.license_tier, ctx.seller, ctx.store_license],
        )?;
        Ok(Some(db::pack_context(c, &id2)?))
    })
    .await?;
    saved.map(Json).ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))
}

#[derive(Debug, Deserialize)]
pub(super) struct PacksParams {
    /// The archive's packs instead of the library's.
    #[serde(default)]
    archived: Option<bool>,
}

pub(super) async fn list_packs(
    State(state): State<AppState>,
    Query(p): Query<PacksParams>,
) -> ApiResult<Json<Vec<Pack>>> {
    let archived = p.archived.unwrap_or(false);
    Ok(Json(db::call(&state.pool, move |c| db::list_packs(c, archived)).await?))
}

pub(super) async fn get_pack(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<Pack>> {
    let id2 = id.clone();
    db::call(&state.pool, move |c| db::get_pack(c, &id2))
        .await?
        .map(Json)
        .ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))
}

#[derive(Deserialize)]
pub(super) struct PackPatch {
    name: Option<String>,
    facing: Option<hoard_core::Facing>,
}

/// Rename a pack, or turn its models. Everything refers to packs by id, so searches, links and collections carry on
/// as they were. Models turned are drawn again: the thumbnails browsers drew of them give way to the server's until a
/// browser draws them again.
pub(super) async fn patch_pack(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(patch): Json<PackPatch>,
) -> ApiResult<Json<Pack>> {
    let name = patch.name.map(|n| n.trim().chars().take(200).collect::<String>());
    if name.as_deref() == Some("") {
        return Err(AppError::bad_request(tr!("error-pack-needs-name")));
    }
    let facing = patch.facing.map(|f| hoard_core::Facing::new(f.turn.into(), f.tilt.into()));
    let id2 = id.clone();
    let pack = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        if let Some(name) = &name {
            tx.execute("UPDATE packs SET name = ?2 WHERE id = ?1", params![id2, name])?;
        }
        if let Some(f) = facing
            && db::get_pack(&tx, &id2)?.is_some_and(|p| p.facing != f)
        {
            tx.execute(
                "UPDATE packs SET model_turn = ?2, model_tilt = ?3 WHERE id = ?1",
                params![id2, f.turn, f.tilt],
            )?;
            tx.execute(
                "UPDATE assets SET thumb_source = NULL WHERE pack_id = ?1 AND kind = 'model' AND thumb_source IS NOT NULL",
                [&id2],
            )?;
            let queued: bool = tx.query_row(
                "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND pack_id = ?2 AND status = 'queued')",
                params![jobs::RETAG, id2],
                |r| r.get(0),
            )?;
            if !queued {
                db::set_pack_status(&tx, &id2, hoard_core::PackStatus::Ingesting, None)?;
                db::enqueue(&tx, jobs::RETAG, Some(&id2), &json!({ "then_llm": false }))?;
            }
        }
        let pack = db::get_pack(&tx, &id2)?;
        tx.commit()?;
        Ok(pack)
    })
    .await?;
    state.wake.notify_one();
    pack.map(Json).ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))
}

pub(super) async fn delete_pack(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let id2 = id.clone();
    if db::call(&state.pool, move |c| db::delete_pack(c, &id2)).await? {
        tokio::fs::remove_dir_all(state.cfg.staging_dir().join(&id)).await.ok();
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found(tr!("error-pack-not-found", id = id)))
    }
}

/// `multipart/form-data` upload of one pack. Fields, in order:
/// - `name` (optional), `source` (optional, e.g. where you got it)
/// - for each file: optional `relpath` text field, then a `file` field. Without `relpath`, the part's
///   filename is used. Archives (.zip/.tar/.tar.gz) are unpacked server-side, nested ones too.
pub(super) async fn upload_pack(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> ApiResult<(StatusCode, Json<Pack>)> {
    if !state.blobs.can_store() {
        return Err(AppError::conflict(crate::blobs::no_library()));
    }
    let id = uuid::Uuid::now_v7().to_string();
    let staging = state.cfg.staging_dir().join(&id);
    tokio::fs::create_dir_all(&staging).await?;

    let mut name: Option<String> = None;
    let mut source: Option<String> = None;
    let mut pending_rel: Option<String> = None;
    let mut files: Vec<String> = Vec::new();

    let received: anyhow::Result<()> = async {
        while let Some(mut field) = multipart.next_field().await? {
            let field_name = field.name().map(str::to_owned);
            let file_name = field.file_name().map(str::to_owned);
            match (field_name.as_deref(), file_name) {
                (Some("name"), None) => name = Some(field.text().await?.trim().to_owned()).filter(|s| !s.is_empty()),
                (Some("source"), None) => {
                    source = Some(field.text().await?.trim().to_owned()).filter(|s| !s.is_empty());
                }
                (Some("relpath"), None) => pending_rel = Some(field.text().await?),
                (_, Some(file_name)) => {
                    let raw = pending_rel.take().unwrap_or(file_name);
                    let Some(rel) = safe_rel_path(&raw) else { continue };
                    let dest = staging.join(&rel);
                    if let Some(parent) = dest.parent() {
                        tokio::fs::create_dir_all(parent).await?;
                    }
                    let mut out = tokio::io::BufWriter::new(tokio::fs::File::create(&dest).await?);
                    while let Some(chunk) = field.chunk().await? {
                        out.write_all(&chunk).await?;
                    }
                    out.flush().await?;
                    files.push(rel);
                }
                _ => {}
            }
        }
        Ok(())
    }
    .await;

    if let Err(e) = received {
        tokio::fs::remove_dir_all(&staging).await.ok();
        return Err(AppError::bad_request(tr!("error-upload-failed", reason = format!("{e:#}"))));
    }
    if files.is_empty() {
        tokio::fs::remove_dir_all(&staging).await.ok();
        return Err(AppError::bad_request(tr!("error-upload-no-files")));
    }

    let auto_name = name.is_none();
    let name = name.unwrap_or_else(|| default_pack_name(&files));
    let pack_id = id.clone();
    let pack = db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO packs (id, name, status, source, files_total, created_at) VALUES (?1, ?2, 'ingesting', ?3, ?4, ?5)",
            params![pack_id, name, source, files.len() as i64, db::now()],
        )?;
        db::enqueue(c, jobs::INGEST, Some(&pack_id), &json!({ "auto_name": auto_name }))?;
        db::get_pack(c, &pack_id)?.context("pack vanished")
    })
    .await?;
    state.wake.notify_one();
    Ok((StatusCode::ACCEPTED, Json(pack)))
}

pub(crate) fn default_pack_name(files: &[String]) -> String {
    if let [only] = files {
        let file = only.rsplit('/').next().unwrap_or(only);
        return file.rsplit_once('.').map_or(file, |(s, _)| s).to_owned();
    }
    let tops: BTreeSet<&str> = files.iter().map(|f| f.split('/').next().unwrap_or(f)).collect();
    match tops.into_iter().collect::<Vec<_>>().as_slice() {
        [one] if files.iter().all(|f| f.contains('/')) => (*one).to_owned(),
        _ => format!("Pack {}", &db::now()[..10]),
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct RetagBody {
    #[serde(default = "yes")]
    rules: bool,
    #[serde(default)]
    llm: Option<bool>,
}

fn yes() -> bool {
    true
}

pub(super) async fn retag_pack(
    State(state): State<AppState>,
    Path(id): Path<String>,
    body: Option<Json<RetagBody>>,
) -> ApiResult<StatusCode> {
    let body = body.map_or(RetagBody { rules: true, llm: None }, |Json(b)| b);
    let llm_on = state.settings.get().llm_enabled();
    let llm = body.llm.unwrap_or(llm_on) && llm_on;
    if !body.rules && !llm {
        return Err(AppError::bad_request(tr!("error-analyze-nothing-to-do")));
    }
    let id2 = id.clone();
    let found = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(false);
        }
        if body.rules {
            db::set_pack_status(c, &id2, hoard_core::PackStatus::Ingesting, None)?;
            db::enqueue(c, jobs::RETAG, Some(&id2), &json!({ "then_llm": llm }))?;
        } else {
            db::set_pack_status(c, &id2, hoard_core::PackStatus::Analyzing, None)?;
            db::enqueue(c, jobs::LLM_TAG, Some(&id2), &json!({}))?;
        }
        Ok(true)
    })
    .await?;
    if !found {
        return Err(AppError::not_found(tr!("error-pack-not-found", id = id)));
    }
    state.wake.notify_one();
    Ok(StatusCode::ACCEPTED)
}

/// `POST /api/packs/{id}/read-files`: read the store's files in the pack again (a Megascans JSON) for its name, tags
/// and description.
pub(super) async fn read_pack_files(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let id2 = id.clone();
    let found = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(false);
        }
        if !db::job_pending(c, jobs::READ_FILES, Some(&id2), None)? {
            db::enqueue(c, jobs::READ_FILES, Some(&id2), &json!({}))?;
        }
        Ok(true)
    })
    .await?;
    if !found {
        return Err(AppError::not_found(tr!("error-pack-not-found", id = id)));
    }
    state.wake.notify_one();
    Ok(StatusCode::ACCEPTED)
}

/// `POST /api/packs/{id}/store/undo`: take off a store page that was found automatically. The pack isn't looked up
/// again; the other suggestions stay for Store details.
pub(super) async fn undo_store_page(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<PackContext>> {
    let id2 = id.clone();
    let ctx = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(None);
        }
        crate::store_lookup::undo(c, &id2)?;
        Ok(Some(db::pack_context(c, &id2)?))
    })
    .await?;
    ctx.map(Json).ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_names() {
        assert_eq!(default_pack_name(&["kenney_ui.zip".into()]), "kenney_ui");
        assert_eq!(default_pack_name(&["Forest/a.png".into(), "Forest/b/c.wav".into()]), "Forest");
    }
}
