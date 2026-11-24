//! Assets: searching, reading and editing them, their near copies, the archive, and tags.

use crate::auth::Who;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{AppState, db, jobs};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::{Extension, Json};
use hoard_core::{Asset, AssetPatch, SearchQuery, SearchResult};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
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
    /// Only what's been reviewed, or only what hasn't.
    #[serde(default)]
    reviewed: Option<bool>,
    /// Only what's rated this many stars or more.
    #[serde(default)]
    rating_min: Option<u8>,
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
            reviewed: p.reviewed,
            rating_min: p.rating_min,
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
    asset.near_copies = crate::nearcopies::for_asset(&state, id.clone()).await?;
    let (fits, total) = crate::animations::fits_for(&state, id.clone()).await?;
    asset.fits_total = (total > fits.len()).then_some(total as u64);
    asset.fits = fits;
    match asset.kind {
        hoard_core::AssetKind::Animation => asset.plays_on = crate::animations::plays_on(&state, id).await?,
        hoard_core::AssetKind::Model => {
            asset.plays_its_skeleton = crate::animations::plays_its_skeleton(&state, id).await?;
        }
        _ => {}
    }
    Ok(Json(asset))
}

/// `PUT /api/assets/{id}/skeleton-body`: make this character the one its skeleton's animations play on, everywhere
/// they're shown, and draw again those drawn on another.
pub(super) async fn choose_skeleton_body(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    let redo = crate::animations::choose_body(&state, id.clone())
        .await?
        .ok_or_else(|| AppError::bad_request(tr!("error-not-a-character")))?;
    let st = std::sync::Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(e) = crate::motion::redraw(&st, redo).await {
            tracing::warn!(error = %format!("{e:#}"), "couldn't draw animations again on their skeleton's character");
        }
    });
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, schemars::JsonSchema)]
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
    let found = crate::nearcopies::groups(&state).await?;
    let all = &found.sets;
    Ok(Json(hoard_core::CopyGroups {
        groups: all.iter().skip(q.offset).take(q.limit.min(200)).cloned().collect(),
        total: all.len(),
        wasted: all.iter().map(|g| g.wasted).sum(),
        generation: found.generation,
        refreshing: found.refreshing,
    }))
}

pub(super) async fn patch_asset(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<String>,
    Json(patch): Json<AssetPatch>,
) -> ApiResult<Json<Asset>> {
    let id2 = id.clone();
    let actor = db::Actor::of(&who);
    let asset = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        if !db::apply(&tx, &id2, &patch, &actor)? {
            return Ok(None);
        }
        tx.commit()?;
        db::get_asset(c, &id2)
    })
    .await?;
    asset.map(Json).ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))
}

/// An asset's history: what was changed, from what to what, by whom and how, the latest first.
pub(super) async fn asset_history(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Vec<db::Change>>> {
    Ok(Json(db::call(&state.pool, move |c| db::history(c, &id, 500)).await?))
}

/// Put back what one change in an asset's history changed.
pub(super) async fn revert_change(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path((id, change)): Path<(String, i64)>,
) -> ApiResult<Json<Asset>> {
    let actor = db::Actor::of(&who);
    let id2 = id.clone();
    let asset = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        if !db::revert(&tx, &id2, change, &actor)? {
            return Ok(None);
        }
        tx.commit()?;
        db::get_asset(c, &id2)
    })
    .await?;
    asset.map(Json).ok_or_else(|| AppError::not_found(tr!("error-change-not-found")))
}

/// The same edit to many assets at once, in one transaction: to those listed, or to everything a search finds.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(super) struct BulkEdit {
    #[serde(default)]
    ids: Vec<String>,
    /// Instead of `ids`: everything this search finds, however many pages of it.
    #[serde(default)]
    query: Option<SearchQuery>,
    #[serde(flatten)]
    patch: AssetPatch,
}

/// The most assets one bulk edit changes.
const BULK_MAX: usize = 50_000;

pub(super) async fn bulk_assets(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(edit): Json<BulkEdit>,
) -> ApiResult<Json<Value>> {
    if edit.ids.len() > BULK_MAX {
        return Err(AppError::bad_request(tr!("error-too-many-assets")));
    }
    let query = edit.query;
    if query.as_ref().is_some_and(|q| q.similar.is_some() || q.color.is_some() || q.imitation.is_some()) {
        return Err(AppError::bad_request(tr!("error-bulk-by-look")));
    }
    // A name is an asset's own: the same one on many is never meant.
    let patch = AssetPatch { name: None, ..edit.patch };
    let actor = db::Actor::of(&who);
    let ids = edit.ids;
    let updated = db::call(&state.pool, move |c| {
        let ids = match &query {
            Some(q) => db::matching_ids(c, q)?,
            None => ids,
        };
        if ids.len() > BULK_MAX {
            return Ok(None);
        }
        let tx = c.transaction()?;
        let mut n = 0u64;
        for id in &ids {
            n += u64::from(db::apply(&tx, id, &patch, &actor)?);
        }
        tx.commit()?;
        Ok(Some(n))
    })
    .await?
    .ok_or_else(|| AppError::bad_request(tr!("error-too-many-assets")))?;
    Ok(Json(json!({ "updated": updated })))
}

/// Packs and assets to archive, restore or delete for good. `all` (purge only) empties the whole archive.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(super) struct SuggestParams {
    #[serde(default)]
    q: String,
    limit: Option<usize>,
}

/// Tags to offer while someone types one: matches (old names of merged tags lead to what they became), and tags in
/// use a new one is probably meant to be.
pub(super) async fn suggest_tags(
    State(state): State<AppState>,
    Query(p): Query<SuggestParams>,
) -> ApiResult<Json<db::TagSuggestions>> {
    let limit = p.limit.unwrap_or(12).min(50);
    Ok(Json(db::call(&state.pool, move |c| db::suggest_tags(c, &p.q, limit)).await?))
}

/// The categories in the library, the most used first, for picking one.
pub(super) async fn categories(State(state): State<AppState>) -> ApiResult<Json<Vec<hoard_core::Facet>>> {
    Ok(Json(db::call(&state.pool, |c| db::category_counts(c, 500)).await?))
}

#[derive(Deserialize, schemars::JsonSchema)]
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

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct TagInfoParams {
    tag: String,
}

/// A tag as it is written down: lower case, words joined by dashes.
fn a_tag(text: &str) -> Result<String, AppError> {
    hoard_core::normalize_tag(text).ok_or_else(|| AppError::bad_request(tr!("error-not-a-tag", text = text)))
}

/// What a tag's page shows: how many have it, what it means, its picture and its old names.
pub(super) async fn tag_info(
    State(state): State<AppState>,
    Query(p): Query<TagInfoParams>,
) -> ApiResult<Json<db::TagInfo>> {
    let tag = a_tag(&p.tag)?;
    Ok(Json(db::call(&state.pool, move |c| db::tag_info(c, &tag)).await?))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct SetTagInfo {
    tag: String,
    /// Left out leaves it; empty clears it.
    description: Option<String>,
    /// The asset whose picture stands for the tag. Left out leaves it; empty clears it.
    image_id: Option<String>,
}

/// Say what a tag means, or pick the asset whose picture stands for it.
pub(super) async fn set_tag_info(
    State(state): State<AppState>,
    Json(b): Json<SetTagInfo>,
) -> ApiResult<Json<db::TagInfo>> {
    let tag = a_tag(&b.tag)?;
    let info = db::call(&state.pool, move |c| {
        if let Some(id) = b.image_id.as_deref().filter(|id| !id.trim().is_empty()) {
            let found: bool = c.query_row("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1)", [id], |r| r.get(0))?;
            if !found {
                return Ok(None);
            }
        }
        db::set_tag_info(c, &tag, b.description.as_deref(), b.image_id.as_deref())?;
        db::tag_info(c, &tag).map(Some)
    })
    .await?;
    info.map(Json).ok_or_else(|| AppError::not_found(tr!("error-asset-gone")))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct SetTagParents {
    tag: String,
    /// The broader tags it goes under, in place of those it had.
    parents: Vec<String>,
}

/// Put a tag under broader ones: searching one of those then finds it too.
pub(super) async fn set_tag_parents(
    State(state): State<AppState>,
    Json(b): Json<SetTagParents>,
) -> ApiResult<Json<db::TagInfo>> {
    let tag = a_tag(&b.tag)?;
    let mut parents = Vec::new();
    for p in b.parents.iter().take(32) {
        let p = a_tag(p)?;
        if !parents.contains(&p) {
            parents.push(p);
        }
    }
    let tag2 = tag.clone();
    let done = db::call(&state.pool, move |c| match db::set_tag_parents(c, &tag2, &parents)? {
        Ok(()) => db::tag_info(c, &tag2).map(Ok),
        Err(bad) => Ok(Err(bad)),
    })
    .await?;
    match done {
        Ok(info) => Ok(Json(info)),
        Err(parent) => {
            Err(AppError::bad_request(tr!("error-tag-parent-loop", tag = tag.as_str(), parent = parent.as_str())))
        }
    }
}

/// Every tag that goes under another, as (tag, parent) pairs.
pub(super) async fn tag_tree(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let rows = db::call(&state.pool, |c| db::tag_tree(c)).await?;
    Ok(Json(json!(rows.into_iter().map(|(tag, parent)| json!({ "tag": tag, "parent": parent })).collect::<Vec<_>>())))
}

pub(super) async fn tag_aliases(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let rows = db::call(&state.pool, |c| db::tag_aliases(c)).await?;
    Ok(Json(json!(rows.into_iter().map(|(tag, into)| json!({ "tag": tag, "into": into })).collect::<Vec<_>>())))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct LookAlikeParams {
    /// At most this many pairs (100 when left out).
    limit: Option<usize>,
}

/// Tags in use that look like each other, each with the one the other would merge into.
pub(super) async fn lookalike_tags(
    State(state): State<AppState>,
    Query(p): Query<LookAlikeParams>,
) -> ApiResult<Json<Vec<db::LookAlike>>> {
    let limit = p.limit.unwrap_or(100).min(1000);
    Ok(Json(db::call(&state.pool, move |c| db::lookalike_tags(c, limit)).await?))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct TagsApart {
    a: String,
    b: String,
}

/// Say two tags that look alike are different.
pub(super) async fn keep_tags_apart(State(state): State<AppState>, Json(p): Json<TagsApart>) -> ApiResult<StatusCode> {
    let (a, b) = (a_tag(&p.a)?, a_tag(&p.b)?);
    db::call(&state.pool, move |c| db::keep_tags_apart(c, &a, &b)).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, schemars::JsonSchema)]
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
