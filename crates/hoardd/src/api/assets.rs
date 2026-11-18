//! Assets: searching, reading and editing them, their near copies, the archive, and tags.

use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{AppState, db, jobs};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Method, StatusCode};
use hoard_core::{Asset, AssetPatch, SearchQuery, SearchResult, TagSource};
use rusqlite::params;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]
pub(super) struct AssetsParams {
    q: Option<String>,
    kind: Option<String>,
    /// Comma-separated; all must match.
    tags: Option<String>,
    pack: Option<String>,
    /// With `pack`: one folder of it (see [`SearchQuery::folder`]).
    #[serde(default)]
    folder: Option<String>,
    ext: Option<String>,
    sort: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    #[serde(default)]
    facets: Option<bool>,
    #[serde(default)]
    hide_supporting: Option<bool>,
    #[serde(default)]
    collection: Option<String>,
    /// Only what this project (id or name) has.
    #[serde(default)]
    project: Option<String>,
    /// Search the archive instead of the library.
    #[serde(default)]
    archived: Option<bool>,
    /// "More like this": an asset id.
    #[serde(default)]
    similar: Option<String>,
    /// `#rrggbb`: assets with much of this color.
    #[serde(default)]
    color: Option<String>,
    /// A word imitating a sound (`whooom`): sounds shaped like it.
    #[serde(default)]
    imitation: Option<String>,
    /// Only the favorites of whoever asks.
    #[serde(default)]
    favorites: Option<bool>,
    /// Music at this tempo or faster, or slower (BPM).
    #[serde(default)]
    bpm_min: Option<f32>,
    #[serde(default)]
    bpm_max: Option<f32>,
    /// Music in this key (`a-minor`, `Am`, `8A`).
    #[serde(default)]
    key: Option<String>,
    /// With `key`: the keys that mix with it too.
    #[serde(default)]
    compatible_keys: Option<bool>,
}

impl From<AssetsParams> for SearchQuery {
    fn from(p: AssetsParams) -> Self {
        SearchQuery {
            q: p.q,
            kind: p.kind,
            tags: p.tags.map_or_default(|t| t.split(',').filter_map(hoard_core::normalize_tag).collect()),
            pack: p.pack,
            folder: p.folder,
            ext: p.ext,
            sort: p.sort,
            limit: p.limit,
            offset: p.offset,
            hide_supporting: p.hide_supporting.unwrap_or(false),
            collection: p.collection,
            project: p.project,
            archived: p.archived.unwrap_or(false),
            similar: p.similar,
            color: p.color,
            imitation: p.imitation,
            bpm_min: p.bpm_min,
            bpm_max: p.bpm_max,
            key: p.key,
            compatible_keys: p.compatible_keys.unwrap_or(false),
            favorites_of: None,
        }
    }
}

pub(super) async fn search_assets(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<AssetsParams>,
) -> ApiResult<Json<SearchResult>> {
    let facets = p.facets.unwrap_or(true);
    let favorites = p.favorites.unwrap_or(false);
    let mut q: SearchQuery = p.into();
    if favorites {
        let who = crate::auth::authenticate(&state, &Method::GET, &headers).await?;
        q.favorites_of = Some(who.and_then(|w| w.owner()).ok_or_else(AppError::unauthorized)?);
    }
    let page = crate::looks::search(&state, q, facets).await?;
    Ok(Json(SearchResult {
        items: page.items,
        total: page.total,
        kinds: page.kinds,
        tags: page.tags,
        supporting_hidden: page.supporting_hidden,
        folders: page.folders,
    }))
}

pub(super) async fn get_asset(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<Asset>> {
    let id2 = id.clone();
    let mut asset = db::call(&state.pool, move |c| db::get_asset(c, &id2))
        .await?
        .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    if let Some(o) = &mut asset.origin {
        o.kept_until = crate::reformat::kept_until(o, state.settings.get().conversion_rules.keep_days);
    }
    asset.near_copies = crate::nearcopies::for_asset(&state, id).await?;
    Ok(Json(asset))
}

#[derive(Deserialize)]
pub(super) struct DuplicatesQuery {
    #[serde(default)]
    offset: usize,
    #[serde(default = "default_duplicates_limit")]
    limit: usize,
}

fn default_duplicates_limit() -> usize {
    50
}

/// The library's sets of near copies, the most space taken by the copies that aren't the best first.
pub(super) async fn duplicates(
    State(state): State<AppState>,
    Query(q): Query<DuplicatesQuery>,
) -> ApiResult<Json<hoard_core::CopyGroups>> {
    let all = crate::nearcopies::groups(&state).await?;
    Ok(Json(hoard_core::CopyGroups {
        groups: all.iter().skip(q.offset).take(q.limit.min(200)).cloned().collect(),
        total: all.len(),
        wasted: all.iter().map(|g| g.wasted).sum(),
    }))
}

pub(super) async fn patch_asset(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(patch): Json<AssetPatch>,
) -> ApiResult<Json<Asset>> {
    let id2 = id.clone();
    let asset = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let exists: bool = tx.query_row("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1)", [&id2], |r| r.get(0))?;
        if !exists {
            return Ok(None);
        }
        let touches_fields = patch.name.is_some() || patch.category.is_some() || patch.description.is_some();
        let category = patch.category.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty());
        // An empty description clears it.
        let description = patch.description.map(|s| s.trim().chars().take(2000).collect::<String>());
        let add_tags: Vec<String> = patch.add_tags.iter().filter_map(|t| hoard_core::normalize_tag(t)).collect();
        // Variants share everything but their name: category, description and tag edits fan out to the family.
        let targets = if patch.apply_to_family { db::family_ids(&tx, &id2)? } else { vec![id2.clone()] };
        for target in targets.iter().filter(|t| **t != id2) {
            let touches = category.is_some() || description.is_some();
            tx.execute(
                "UPDATE assets SET category = COALESCE(?2, category),
                        description = CASE WHEN ?3 IS NULL THEN description WHEN ?3 = '' THEN NULL ELSE ?3 END,
                        human_edited = human_edited OR ?4, updated_at = ?5
                 WHERE id = ?1",
                params![target, category, description, touches, db::now()],
            )?;
        }
        tx.execute(
            "UPDATE assets SET name = COALESCE(?2, name), category = COALESCE(?3, category),
                    description = CASE WHEN ?4 IS NULL THEN description WHEN ?4 = '' THEN NULL ELSE ?4 END,
                    human_edited = human_edited OR ?5, updated_at = ?6
             WHERE id = ?1",
            params![
                id2,
                patch.name.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()),
                category,
                description,
                touches_fields,
                db::now()
            ],
        )?;
        for target in &targets {
            for t in &add_tags {
                db::upsert_tag(&tx, target, t, TagSource::Human, 1.0)?;
            }
            for t in &patch.remove_tags {
                tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag = ?2", params![target, t])?;
            }
            db::reindex_asset(&tx, target)?;
        }
        tx.commit()?;
        db::get_asset(c, &id2)
    })
    .await?;
    asset.map(Json).ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))
}

/// The same edit to many assets at once, in one transaction: tags in and out, and a category.
#[derive(Debug, Deserialize)]
pub(super) struct BulkEdit {
    ids: Vec<String>,
    #[serde(default)]
    add_tags: Vec<String>,
    #[serde(default)]
    remove_tags: Vec<String>,
    /// Set the category; an empty one clears it.
    #[serde(default)]
    category: Option<String>,
}

pub(super) async fn bulk_assets(State(state): State<AppState>, Json(edit): Json<BulkEdit>) -> ApiResult<Json<Value>> {
    if edit.ids.len() > 50_000 {
        return Err(AppError::bad_request(tr!("error-too-many-assets")));
    }
    let add: Vec<String> = edit.add_tags.iter().filter_map(|t| hoard_core::normalize_tag(t)).collect();
    let remove: Vec<String> = edit.remove_tags.iter().filter_map(|t| hoard_core::normalize_tag(t)).collect();
    let category = edit.category.map(|c| c.trim().to_owned());
    let updated = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let now = db::now();
        let mut n = 0u64;
        for id in &edit.ids {
            let exists: bool =
                tx.query_row("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1)", [id], |r| r.get(0))?;
            if !exists {
                continue;
            }
            for t in &add {
                db::upsert_tag(&tx, id, t, TagSource::Human, 1.0)?;
            }
            for t in &remove {
                tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag = ?2", params![id, t])?;
            }
            if let Some(cat) = &category {
                tx.execute(
                    "UPDATE assets SET category = NULLIF(?2, ''), human_edited = 1, updated_at = ?3 WHERE id = ?1",
                    params![id, cat, now],
                )?;
            } else {
                tx.execute("UPDATE assets SET updated_at = ?2 WHERE id = ?1", params![id, now])?;
            }
            db::reindex_asset(&tx, id)?;
            n += 1;
        }
        tx.commit()?;
        Ok(n)
    })
    .await?;
    Ok(Json(json!({ "updated": updated })))
}

/// Packs and assets to archive, restore or delete for good. `all` (purge only) empties the whole archive.
#[derive(Debug, Deserialize)]
pub(super) struct ArchiveItems {
    #[serde(default)]
    packs: Vec<String>,
    #[serde(default)]
    assets: Vec<String>,
    #[serde(default)]
    all: bool,
}

impl ArchiveItems {
    fn check(&self) -> ApiResult<()> {
        if self.packs.len() + self.assets.len() > 50_000 {
            return Err(AppError::bad_request(tr!("error-too-many-items")));
        }
        Ok(())
    }
}

pub(super) async fn archive(
    State(state): State<AppState>,
    Json(items): Json<ArchiveItems>,
) -> ApiResult<Json<db::ArchiveCount>> {
    items.check()?;
    let n = db::call(&state.pool, move |c| db::set_archived(c, &items.packs, &items.assets, true)).await?;
    tracing::info!(packs = n.packs, assets = n.assets, "archived");
    Ok(Json(n))
}

pub(super) async fn restore(
    State(state): State<AppState>,
    Json(items): Json<ArchiveItems>,
) -> ApiResult<Json<db::ArchiveCount>> {
    items.check()?;
    let n = db::call(&state.pool, move |c| db::set_archived(c, &items.packs, &items.assets, false)).await?;
    tracing::info!(packs = n.packs, assets = n.assets, "restored from the archive");
    Ok(Json(n))
}

/// Delete for good. Only what's in the archive goes; bytes nothing else uses are freed right after.
pub(super) async fn purge(
    State(state): State<AppState>,
    Json(items): Json<ArchiveItems>,
) -> ApiResult<Json<db::ArchiveCount>> {
    items.check()?;
    let (n, gone) =
        db::call(&state.pool, move |c| db::purge_archived(c, &items.packs, &items.assets, items.all)).await?;
    tracing::info!(packs = n.packs, assets = n.assets, "deleted from the archive");
    for id in &gone {
        tokio::fs::remove_dir_all(state.cfg.staging_dir().join(id)).await.ok();
    }
    if n.packs + n.assets > 0 {
        let state = std::sync::Arc::clone(&state);
        tokio::spawn(async move {
            match jobs::gc(&state).await {
                Ok((count, bytes)) => tracing::info!(count, bytes, "freed the archive's files"),
                Err(e) => tracing::warn!(error = %e, "could not free the archive's files"),
            }
        });
    }
    Ok(Json(n))
}

#[derive(Debug, Deserialize)]
pub(super) struct TagParams {
    #[serde(default)]
    prefix: String,
    /// Match `prefix` anywhere in the tag, not only at the start.
    #[serde(default)]
    anywhere: bool,
    limit: Option<u32>,
}

pub(super) async fn tags(State(state): State<AppState>, Query(p): Query<TagParams>) -> ApiResult<Json<Value>> {
    let limit = p.limit.unwrap_or(30).min(5000);
    let rows = db::call(&state.pool, move |c| db::tag_counts(c, &p.prefix.to_lowercase(), p.anywhere, limit)).await?;
    Ok(Json(json!(rows)))
}

#[derive(Deserialize)]
pub(super) struct MergeTags {
    from: Vec<String>,
    into: String,
}

/// Merge tags into one (or rename one: a single tag into a new name).
pub(super) async fn merge_tags(State(state): State<AppState>, Json(m): Json<MergeTags>) -> ApiResult<Json<Value>> {
    let into = hoard_core::normalize_tag(&m.into).ok_or_else(|| AppError::bad_request(tr!("error-merge-into-tag")))?;
    let from: Vec<String> =
        m.from.iter().map(|t| t.trim().to_lowercase()).filter(|t| !t.is_empty() && *t != into).collect();
    if from.is_empty() {
        return Err(AppError::bad_request(tr!("error-nothing-to-merge")));
    }
    let (into2, from2) = (into.clone(), from.clone());
    let assets = db::call(&state.pool, move |c| db::merge_tags(c, &from2, &into2)).await?;
    tracing::info!(?from, into, assets, "tags merged");
    Ok(Json(json!({ "into": into, "assets": assets })))
}

pub(super) async fn tag_aliases(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let rows = db::call(&state.pool, |c| db::tag_aliases(c)).await?;
    Ok(Json(json!(rows.into_iter().map(|(tag, into)| json!({ "tag": tag, "into": into })).collect::<Vec<_>>())))
}

#[derive(Deserialize)]
pub(super) struct AliasParams {
    tag: String,
}

/// Stop mapping a merged tag's old name: the taggers may use it again.
pub(super) async fn drop_tag_alias(
    State(state): State<AppState>,
    Query(p): Query<AliasParams>,
) -> ApiResult<StatusCode> {
    db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM tag_aliases WHERE tag = ?1", [&p.tag]).map(drop)?))
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
