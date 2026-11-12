//! HTTP API. Reads are open; writes go through `require_token` when `HOARD_TOKEN` is set.

use std::collections::BTreeSet;

use anyhow::Context;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Multipart, Path, Query, Request, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use hoard_core::{
    Asset, AssetPatch, Collection, CollectionMembers, CollectionPatch, Manifest, ManifestEntry, ManifestRequest, Pack,
    PackContext, SearchQuery, SearchResult, Stats, TagSource,
};
use object_store::GetRange;
use rusqlite::{OptionalExtension as _, params};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::AsyncWriteExt;

use crate::AppState;
use crate::blobs::is_valid_hash;
use crate::error::{ApiResult, AppError};
use crate::ingest::safe_rel_path;
use crate::{context, db, jobs};

pub(crate) fn router(state: &AppState) -> Router<AppState> {
    // The upload limit is a setting that can change while the server runs, so it's checked per request.
    let upload_limit = || {
        tower::ServiceBuilder::new()
            .layer(DefaultBodyLimit::disable())
            .layer(middleware::from_fn_with_state(std::sync::Arc::clone(state), limit_upload))
    };
    let writes = Router::new()
        .route("/api/packs", post(upload_pack).layer(upload_limit()))
        .route("/api/packs/{id}", axum::routing::delete(delete_pack))
        .route("/api/packs/{id}/retag", post(retag_pack))
        .route("/api/packs/{id}/context", axum::routing::put(put_pack_context))
        .route("/api/packs/{id}/unreal/convert", post(unreal_convert))
        .route("/api/packs/{id}/unreal/finish", post(unreal_finish))
        .route("/api/assets/{id}/replace", post(replace_asset).layer(upload_limit()))
        .route("/api/collections", post(create_collection))
        .route("/api/collections/{id}", axum::routing::patch(patch_collection).delete(delete_collection))
        .route("/api/collections/{id}/assets", post(collection_members))
        .route("/api/assets/{id}", axum::routing::patch(patch_asset))
        .route("/api/admin/scrub", post(admin_scrub))
        .route("/api/admin/gc", post(admin_gc))
        .route("/api/admin/backup", post(admin_backup))
        .route("/api/admin/settings", get(get_settings).put(put_settings))
        .route("/api/admin/llm/models", post(llm_models))
        .merge(crate::uploads::router())
        .route_layer(middleware::from_fn_with_state(std::sync::Arc::clone(state), require_token));

    Router::new()
        .route("/api/health", get(|| async { Json(json!({ "ok": true })) }))
        .route("/api/stats", get(stats))
        .route("/api/packs", get(list_packs))
        .route("/api/packs/{id}", get(get_pack))
        .route("/api/packs/{id}/context", get(get_pack_context))
        .route("/api/assets", get(search_assets))
        .route("/api/assets/{id}", get(get_asset))
        .route("/api/assets/{id}/file", get(asset_file))
        .route("/api/assets/{id}/thumb", get(asset_thumb))
        .route("/api/blobs/{hash}", get(blob))
        .route("/api/tags", get(tags))
        .route("/api/manifest", post(manifest))
        .route("/api/admin/jobs", get(admin_jobs))
        .route("/api/assets/{id}/bundle.zip", get(asset_bundle))
        .route("/api/bundle.zip", get(selection_bundle))
        .route("/api/collections", get(list_collections))
        .route("/api/collections/{id}", get(get_collection))
        .route("/api/collections/{id}/bundle.zip", get(collection_bundle))
        .route("/api/assets/{id}/derivative/{kind}", get(asset_derivative))
        .route("/api/packs/{id}/bundle.zip", get(pack_bundle))
        .route("/api/packs/{id}/files/{*path}", get(pack_file))
        .route("/api/packs/{id}/unreal", get(unreal_status))
        .route("/api/unreal/pending", get(unreal_pending))
        .route("/api/assets/{id}/original", get(asset_original))
        .route("/api/assets/{id}/as/{format}", get(asset_as))
        .route("/api/assets/{id}/as/{format}/status", get(asset_as_status))
        .route("/api/activity", get(activity))
        .merge(crate::mcp::router(state))
        .merge(writes)
}

async fn require_token(State(state): State<AppState>, req: Request, next: Next) -> Result<Response, AppError> {
    if let Some(expected) = &state.cfg.token {
        let given = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .unwrap_or("");
        // Constant-time compare so the token can't be guessed byte by byte.
        let same = given.len() == expected.len()
            && given.bytes().zip(expected.bytes()).fold(0u8, |acc, (a, b)| acc | (a ^ b)) == 0;
        if !same {
            return Err(AppError::unauthorized());
        }
    }
    Ok(next.run(req).await)
}

/// Holds an upload to the "Largest upload" setting as it is now: refused up front when it says it's bigger, and
/// cut off if it turns out to be.
async fn limit_upload(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let limits = state.settings.get();
    let max = limits.max_upload_bytes();
    let declared = req.headers().get(header::CONTENT_LENGTH).and_then(|v| v.to_str().ok()?.parse::<u64>().ok());
    if declared.is_some_and(|n| n > max) {
        let msg = format!("this upload is over the {} MiB limit (raise it in Settings)", limits.max_upload_mb);
        return (StatusCode::PAYLOAD_TOO_LARGE, msg).into_response();
    }
    let (parts, body) = req.into_parts();
    let body = Body::new(http_body_util::Limited::new(body, usize::try_from(max).unwrap_or(usize::MAX)));
    next.run(Request::from_parts(parts, body)).await
}

// ---------------------------------------------------------------------------------------------
// Stats / packs

async fn stats(State(state): State<AppState>) -> ApiResult<Json<Stats>> {
    let llm_enabled = state.settings.get().llm_enabled();
    let storage = state.blobs.label().to_owned();
    let auth_required = state.cfg.token.is_some();
    let s = db::call(&state.pool, move |c| {
        let one = |sql: &str| -> anyhow::Result<u64> { Ok(c.query_row(sql, [], |r| r.get::<_, i64>(0))? as u64) };
        let mut stmt = c.prepare("SELECT kind, COUNT(*) FROM assets GROUP BY kind ORDER BY 2 DESC")?;
        let kinds = stmt
            .query_map([], |r| Ok(hoard_core::Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 }))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(Stats {
            assets: one("SELECT COUNT(*) FROM assets")?,
            packs: one("SELECT COUNT(*) FROM packs")?,
            blobs: one("SELECT COUNT(*) FROM blobs")?,
            blobs_bad: one("SELECT COUNT(*) FROM blobs WHERE status != 'ok'")?,
            stored_bytes: one("SELECT COALESCE(SUM(size), 0) FROM blobs")?,
            logical_bytes: one("SELECT COALESCE(SUM(b.size), 0) FROM assets a JOIN blobs b ON b.hash = a.blob_hash")?,
            jobs_pending: one("SELECT COUNT(*) FROM jobs WHERE status IN ('queued', 'running')")?,
            llm_enabled,
            storage,
            auth_required,
            kinds,
            duplicate_assets: one(
                "SELECT COALESCE(SUM(n - 1), 0) FROM (SELECT COUNT(*) AS n FROM assets GROUP BY blob_hash HAVING n > 1)",
            )?,
            version: hoard_core::VERSION.to_owned(),
            ffmpeg: crate::ffmpeg::describe(),
            blender: crate::blender::describe(),
            update: crate::updates::available(),
            renderer: crate::analyze::describe_renderer(),
        })
    })
    .await?;
    Ok(Json(s))
}

async fn get_pack_context(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<PackContext>> {
    let id2 = id.clone();
    let ctx =
        db::call(&state.pool, move |c| db::get_pack(c, &id2)?.is_some().then(|| db::pack_context(c, &id2)).transpose())
            .await?;
    ctx.map(Json).ok_or_else(|| AppError::not_found(format!("pack {id}")))
}

/// Set what the seller says about a pack. `readme` is captured at ingest and ignored here. Giving a store URL
/// without store text fetches the page's title and description (best effort) so the tagger has something to read.
async fn put_pack_context(
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
    if ctx.store.is_none() {
        ctx.store = ctx.store_url.as_deref().and_then(context::store_from_url).map(str::to_owned);
    }
    if ctx.store_text.is_none()
        && let Some(url) = &ctx.store_url
    {
        match context::fetch_store_text(&state.http, url).await {
            Ok(text) => ctx.store_text = text,
            Err(e) => tracing::info!(url, error = %format!("{e:#}"), "couldn't fetch store page"),
        }
    }
    let id2 = id.clone();
    let saved = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(None);
        }
        c.execute(
            "INSERT INTO pack_context (pack_id, store, store_url, store_text, license_tier) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT (pack_id) DO UPDATE SET store = excluded.store, store_url = excluded.store_url,
                 store_text = excluded.store_text, license_tier = excluded.license_tier",
            params![id2, ctx.store, ctx.store_url, ctx.store_text, ctx.license_tier],
        )?;
        Ok(Some(db::pack_context(c, &id2)?))
    })
    .await?;
    saved.map(Json).ok_or_else(|| AppError::not_found(format!("pack {id}")))
}

async fn list_packs(State(state): State<AppState>) -> ApiResult<Json<Vec<Pack>>> {
    Ok(Json(db::call(&state.pool, |c| db::list_packs(c)).await?))
}

async fn get_pack(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<Pack>> {
    let id2 = id.clone();
    db::call(&state.pool, move |c| db::get_pack(c, &id2))
        .await?
        .map(Json)
        .ok_or_else(|| AppError::not_found(format!("pack {id}")))
}

async fn delete_pack(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let id2 = id.clone();
    if db::call(&state.pool, move |c| db::delete_pack(c, &id2)).await? {
        tokio::fs::remove_dir_all(state.cfg.staging_dir().join(&id)).await.ok();
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found(format!("pack {id}")))
    }
}

/// `multipart/form-data` upload of one pack. Fields, in order:
/// - `name` (optional), `source` (optional, e.g. where you got it)
/// - for each file: optional `relpath` text field, then a `file` field. Without `relpath`, the part's
///   filename is used. Archives (.zip/.tar/.tar.gz) are unpacked server-side, nested ones too.
async fn upload_pack(State(state): State<AppState>, mut multipart: Multipart) -> ApiResult<(StatusCode, Json<Pack>)> {
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
        return Err(AppError::bad_request(format!("upload failed: {e:#}")));
    }
    if files.is_empty() {
        tokio::fs::remove_dir_all(&staging).await.ok();
        return Err(AppError::bad_request("no files in upload"));
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
struct RetagBody {
    #[serde(default = "yes")]
    rules: bool,
    #[serde(default)]
    llm: Option<bool>,
}
fn yes() -> bool {
    true
}

async fn retag_pack(
    State(state): State<AppState>,
    Path(id): Path<String>,
    body: Option<Json<RetagBody>>,
) -> ApiResult<StatusCode> {
    let body = body.map_or(RetagBody { rules: true, llm: None }, |Json(b)| b);
    let llm_on = state.settings.get().llm_enabled();
    let llm = body.llm.unwrap_or(llm_on) && llm_on;
    if !body.rules && !llm {
        return Err(AppError::bad_request("nothing to do: rules=false and LLM is off"));
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
        return Err(AppError::not_found(format!("pack {id}")));
    }
    state.wake.notify_one();
    Ok(StatusCode::ACCEPTED)
}

// ---------------------------------------------------------------------------------------------
// Assets

#[derive(Debug, Deserialize)]
struct AssetsParams {
    q: Option<String>,
    kind: Option<String>,
    /// Comma-separated; all must match.
    tags: Option<String>,
    pack: Option<String>,
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
}

impl From<AssetsParams> for SearchQuery {
    fn from(p: AssetsParams) -> Self {
        SearchQuery {
            q: p.q,
            kind: p.kind,
            tags: p.tags.map(|t| t.split(',').filter_map(hoard_core::normalize_tag).collect()).unwrap_or_default(),
            pack: p.pack,
            ext: p.ext,
            sort: p.sort,
            limit: p.limit,
            offset: p.offset,
            hide_supporting: p.hide_supporting.unwrap_or(false),
            collection: p.collection,
        }
    }
}

async fn search_assets(State(state): State<AppState>, Query(p): Query<AssetsParams>) -> ApiResult<Json<SearchResult>> {
    let facets = p.facets.unwrap_or(true);
    let q: SearchQuery = p.into();
    let page = db::call(&state.pool, move |c| db::search(c, &q, facets)).await?;
    Ok(Json(SearchResult {
        items: page.items,
        total: page.total,
        kinds: page.kinds,
        tags: page.tags,
        supporting_hidden: page.supporting_hidden,
    }))
}

async fn get_asset(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<Asset>> {
    let id2 = id.clone();
    db::call(&state.pool, move |c| db::get_asset(c, &id2))
        .await?
        .map(Json)
        .ok_or_else(|| AppError::not_found(format!("asset {id}")))
}

async fn patch_asset(
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
        let description = patch.description.map(|s| s.trim().to_owned());
        let add_tags: Vec<String> = patch.add_tags.iter().filter_map(|t| hoard_core::normalize_tag(t)).collect();
        // Variants share everything but their name: category, description and tag edits fan out to the family.
        let targets = if patch.apply_to_family { db::family_ids(&tx, &id2)? } else { vec![id2.clone()] };
        for target in targets.iter().filter(|t| **t != id2) {
            let touches = category.is_some() || description.is_some();
            tx.execute(
                "UPDATE assets SET category = COALESCE(?2, category), description = COALESCE(?3, description),
                        human_edited = human_edited OR ?4, updated_at = ?5
                 WHERE id = ?1",
                params![target, category, description, touches, db::now()],
            )?;
        }
        tx.execute(
            "UPDATE assets SET name = COALESCE(?2, name), category = COALESCE(?3, category),
                    description = COALESCE(?4, description), human_edited = human_edited OR ?5, updated_at = ?6
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
    asset.map(Json).ok_or_else(|| AppError::not_found(format!("asset {id}")))
}

#[derive(Debug, Deserialize)]
struct FileParams {
    #[serde(default)]
    download: Option<u8>,
}

async fn asset_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<FileParams>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let (hash, path, ext) = db::call(&state.pool, move |c| db::asset_file_info(c, &id2))
        .await?
        .ok_or_else(|| AppError::not_found(format!("asset {id}")))?;
    let mime = mime_guess::from_ext(&ext).first_or_octet_stream().to_string();
    let disposition = p.download.filter(|d| *d != 0).map(|_| {
        let file = path.rsplit('/').next().unwrap_or(&path).replace(['"', '\\'], "_");
        format!("attachment; filename=\"{file}\"")
    });
    serve_blob(&state, &hash, &headers, &mime, disposition).await
}

async fn asset_thumb(State(state): State<AppState>, Path(id): Path<String>, headers: HeaderMap) -> ApiResult<Response> {
    let id2 = id.clone();
    let hash = db::call(&state.pool, move |c| db::asset_thumb(c, &id2))
        .await?
        .ok_or_else(|| AppError::not_found(format!("thumbnail for {id}")))?;
    serve_blob(&state, &hash, &headers, "image/png", None).await
}

async fn blob(State(state): State<AppState>, Path(hash): Path<String>, headers: HeaderMap) -> ApiResult<Response> {
    if !is_valid_hash(&hash) {
        return Err(AppError::bad_request("not a blake3 hex hash"));
    }
    serve_blob(&state, &hash, &headers, "application/octet-stream", None).await
}

/// Parse a single-range `Range: bytes=…` header. Multi-range requests get the full body.
fn parse_range(headers: &HeaderMap) -> Option<GetRange> {
    let v = headers.get(header::RANGE)?.to_str().ok()?.strip_prefix("bytes=")?;
    if v.contains(',') {
        return None;
    }
    let (start, end) = v.split_once('-')?;
    match (start.trim(), end.trim()) {
        ("", n) => Some(GetRange::Suffix(n.parse().ok()?)),
        (s, "") => Some(GetRange::Offset(s.parse().ok()?)),
        (s, e) => {
            let (s, e): (u64, u64) = (s.parse().ok()?, e.parse().ok()?);
            (e >= s).then(|| GetRange::Bounded(s..e + 1))
        }
    }
}

/// Stream a blob from storage to the client. This is the proxy path: when blobs live on a NAS, bytes flow
/// NAS -> this server -> client. Blobs are immutable, so anything in front (Cloudflare, nginx, the browser)
/// may cache them forever, which keeps repeat downloads off your home uplink.
async fn serve_blob(
    state: &AppState,
    hash: &str,
    headers: &HeaderMap,
    content_type: &str,
    disposition: Option<String>,
) -> ApiResult<Response> {
    let etag = format!("\"{hash}\"");
    if headers.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()) == Some(etag.as_str()) {
        return Ok((StatusCode::NOT_MODIFIED, [(header::ETAG, etag)]).into_response());
    }
    let range = parse_range(headers);
    let partial = range.is_some();
    let read = match state.blobs.read(hash, range).await {
        Ok(Some(r)) => r,
        Ok(None) => return Err(AppError::not_found(format!("blob {hash}"))),
        Err(_) if partial => return Ok(StatusCode::RANGE_NOT_SATISFIABLE.into_response()),
        Err(e) => return Err(e.into()),
    };
    let len = read.range.end - read.range.start;
    let mut resp = Response::builder()
        .status(if partial { StatusCode::PARTIAL_CONTENT } else { StatusCode::OK })
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, len)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::ETAG, &etag)
        .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable");
    if partial {
        resp = resp.header(
            header::CONTENT_RANGE,
            format!("bytes {}-{}/{}", read.range.start, read.range.end.saturating_sub(1), read.total_size),
        );
    }
    if let Some(d) = disposition
        && let Ok(v) = HeaderValue::from_str(&d)
    {
        resp = resp.header(header::CONTENT_DISPOSITION, v);
    }
    Ok(resp.body(Body::from_stream(read.stream))?)
}

#[derive(Debug, Deserialize)]
struct TagParams {
    #[serde(default)]
    prefix: String,
    limit: Option<u32>,
}

async fn tags(State(state): State<AppState>, Query(p): Query<TagParams>) -> ApiResult<Json<Value>> {
    let limit = p.limit.unwrap_or(30).min(500);
    let rows = db::call(&state.pool, move |c| db::tag_counts(c, &p.prefix.to_lowercase(), limit)).await?;
    Ok(Json(json!(rows)))
}

/// Flatten a selection (explicit ids and/or a search) into an engine-friendly list of files.
async fn manifest(State(state): State<AppState>, Json(req): Json<ManifestRequest>) -> ApiResult<Json<Manifest>> {
    let entries = db::call(&state.pool, move |c| {
        let mut seen = BTreeSet::new();
        let mut assets = Vec::new();
        for id in &req.ids {
            if let Some(a) = db::get_asset(c, id)? {
                assets.push(a);
            }
        }
        if let Some(mut q) = req.query {
            q.limit = Some(q.limit.unwrap_or(5000).min(5000));
            assets.extend(db::search(c, &q, false)?.items);
        }
        let mut entries = Vec::new();
        for a in assets.into_iter().filter(|a| seen.insert(a.id.clone())) {
            let closure =
                if req.include_deps { db::dependency_closure(c, &a.id, req.include_animations)? } else { Vec::new() };
            if closure.len() <= 1 {
                entries.push(manifest_entry(a, None));
                continue;
            }
            // A bundle: the root plus its dependencies, with paths relative to their common folder.
            let files = db::bundle_files(c, &closure)?;
            let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
            let common = db::common_dir(&paths);
            let root = a.id.clone();
            for f in &files {
                let member = if f.id == root { Some(a.clone()) } else { db::get_asset(c, &f.id)? };
                if let Some(m) = member {
                    let bundle_path = db::strip_dir(&f.path, &common).to_owned();
                    entries.push(manifest_entry(m, Some((root.clone(), bundle_path))));
                }
            }
        }
        Ok(entries)
    })
    .await?;
    Ok(Json(Manifest { generated_at: db::now(), entries }))
}

fn manifest_entry(a: Asset, bundle: Option<(String, String)>) -> ManifestEntry {
    let (bundle, bundle_path) = bundle.map_or((None, None), |(b, p)| (Some(b), Some(p)));
    ManifestEntry {
        url: format!("/api/blobs/{}", a.hash),
        tags: a.tags.into_iter().map(|t| t.tag).collect(),
        id: a.id,
        hash: a.hash,
        size: a.size,
        kind: a.kind,
        name: a.name,
        pack_name: a.pack_name,
        path: a.path,
        category: a.category,
        meta: a.meta,
        bundle,
        bundle_path,
    }
}

#[derive(Debug, Deserialize)]
struct BundleParams {
    #[serde(default)]
    animations: Option<u8>,
}

/// An asset and everything it needs, zipped on the fly.
async fn asset_bundle(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<BundleParams>,
) -> ApiResult<Response> {
    let with_anims = p.animations.is_some_and(|a| a != 0);
    let id2 = id.clone();
    let (files, root_name) = db::call(&state.pool, move |c| {
        let Some((_, path, _)) = db::asset_file_info(c, &id2)? else { return Ok(None) };
        let ids = db::dependency_closure(c, &id2, with_anims)?;
        let stem =
            path.rsplit('/').next().unwrap_or(&path).rsplit_once('.').map_or(path.as_str(), |(s, _)| s).to_owned();
        Ok(Some((db::bundle_files(c, &ids)?, stem)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("asset {id}")))?;
    Ok(crate::bundle::zip_files(&state, &root_name, &files)?)
}

#[derive(Debug, Deserialize)]
struct SelectionParams {
    /// Comma-separated asset ids.
    ids: String,
    /// Include each asset's dependencies (default on).
    #[serde(default)]
    deps: Option<u8>,
    #[serde(default)]
    animations: Option<u8>,
    /// Swap in recovered source files where an asset has one (an Unreal `SoundWave`'s WAV instead of the .uasset).
    #[serde(default)]
    sources: Option<u8>,
}

const MAX_SELECTION: usize = 1000;

/// Several assets (and what they need) in one zip. Files keep their layout inside a folder per pack, so two packs'
/// `Audio/hit.wav` don't collide and models still find their textures.
async fn selection_bundle(State(state): State<AppState>, Query(p): Query<SelectionParams>) -> ApiResult<Response> {
    let ids: Vec<String> = p.ids.split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned).collect();
    if ids.is_empty() || ids.len() > MAX_SELECTION {
        return Err(AppError::bad_request(format!("give between 1 and {MAX_SELECTION} asset ids")));
    }
    let opts = ZipOptions {
        deps: p.deps.is_none_or(|d| d != 0),
        animations: p.animations.is_some_and(|a| a != 0),
        sources: p.sources.is_some_and(|s| s != 0),
    };
    zip_assets(&state, ids, opts, "hoard-selection").await
}

#[derive(Debug, Clone, Copy)]
struct ZipOptions {
    deps: bool,
    animations: bool,
    sources: bool,
}

/// Zip assets (and what they need) into one archive. Files keep their layout inside a folder per pack, so two
/// packs' `Audio/hit.wav` don't collide and models still find their textures.
async fn zip_assets(state: &AppState, ids: Vec<String>, opts: ZipOptions, name: &str) -> ApiResult<Response> {
    let groups = db::call(&state.pool, move |c| {
        let mut all = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for id in &ids {
            let closure = if opts.deps { db::dependency_closure(c, id, opts.animations)? } else { vec![id.clone()] };
            all.extend(closure.into_iter().filter(|x| seen.insert(x.clone())));
        }
        let mut stmt = c.prepare("SELECT p.name FROM assets a JOIN packs p ON p.id = a.pack_id WHERE a.id = ?1")?;
        let mut by_pack: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
        for id in all {
            if let Some(pack) = stmt.query_row([&id], |r| r.get::<_, String>(0)).optional()? {
                by_pack.entry(pack).or_default().push(id);
            }
        }
        by_pack
            .into_iter()
            .map(|(pack, ids)| {
                let mut files = db::bundle_files(c, &ids)?;
                if opts.sources {
                    db::prefer_sources(c, &mut files)?;
                }
                Ok((pack, files))
            })
            .collect::<anyhow::Result<Vec<_>>>()
    })
    .await?;
    if groups.is_empty() {
        return Err(AppError::not_found("those assets"));
    }
    let single = groups.len() == 1;
    let entries = groups
        .into_iter()
        .flat_map(|(pack, files)| {
            let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
            let common = db::common_dir(&paths);
            let prefix = if single { String::new() } else { format!("{}/", pack.replace(['/', '\\'], "_")) };
            files
                .iter()
                .map(|f| crate::bundle::Entry {
                    name: format!("{prefix}{}", db::strip_dir(&f.path, &common)),
                    hash: f.hash.clone(),
                    size: f.size,
                    unity_guid: f.unity_guid.clone(),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let root = name.replace(['/', '\\', '"'], "_");
    Ok(crate::bundle::zip_response(state, &root, entries)?)
}

// ---------------------------------------------------------------------------------------------
// Collections

async fn list_collections(State(state): State<AppState>) -> ApiResult<Json<Vec<Collection>>> {
    Ok(Json(db::call(&state.pool, |c| db::list_collections(c)).await?))
}

async fn get_collection(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<Collection>> {
    let key = id.clone();
    db::call(&state.pool, move |c| db::get_collection(c, &key))
        .await?
        .map(Json)
        .ok_or_else(|| AppError::not_found(format!("collection {id}")))
}

fn clean_name(name: Option<String>) -> Option<String> {
    name.map(|n| n.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(80).collect::<String>())
        .filter(|n| !n.is_empty())
}

async fn create_collection(
    State(state): State<AppState>,
    Json(body): Json<CollectionPatch>,
) -> ApiResult<(StatusCode, Json<Collection>)> {
    let name = clean_name(body.name).ok_or_else(|| AppError::bad_request("a collection needs a name"))?;
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
        Err(name) => Err(AppError::conflict(format!("there's already a collection called {name}"))),
    }
}

async fn patch_collection(
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
        None => Err(AppError::not_found(format!("collection {id}"))),
        Some(Err(name)) => Err(AppError::conflict(format!("there's already a collection called {name}"))),
        Some(Ok(c)) => Ok(Json(c)),
    }
}

async fn delete_collection(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let key = id.clone();
    let n = db::call(&state.pool, move |c| Ok(c.execute("DELETE FROM collections WHERE id = ?1", [&key])?)).await?;
    if n == 0 { Err(AppError::not_found(format!("collection {id}"))) } else { Ok(StatusCode::NO_CONTENT) }
}

async fn collection_members(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CollectionMembers>,
) -> ApiResult<Json<Collection>> {
    if body.add.len() + body.remove.len() > 10_000 {
        return Err(AppError::bad_request("too many assets in one request"));
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
    updated.map(Json).ok_or_else(|| AppError::not_found(format!("collection {id}")))
}

#[derive(Debug, Deserialize)]
struct CollectionZipParams {
    #[serde(default)]
    animations: Option<u8>,
    #[serde(default)]
    sources: Option<u8>,
}

async fn collection_bundle(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<CollectionZipParams>,
) -> ApiResult<Response> {
    let key = id.clone();
    let (name, ids) = db::call(&state.pool, move |c| {
        let Some(col) = db::get_collection(c, &key)? else { return Ok(None) };
        Ok(Some((col.name, db::collection_asset_ids(c, &col.id)?)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("collection {id}")))?;
    if ids.is_empty() {
        return Err(AppError::bad_request(format!("collection {name} is empty")));
    }
    let opts = ZipOptions {
        deps: true,
        animations: p.animations.is_some_and(|a| a != 0),
        sources: p.sources.is_some_and(|s| s != 0),
    };
    zip_assets(&state, ids, opts, &name).await
}

/// A whole pack as one zip, in its original layout.
async fn pack_bundle(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Response> {
    let id2 = id.clone();
    let (name, files) = db::call(&state.pool, move |c| {
        let Some(pack) = db::get_pack(c, &id2)? else { return Ok(None) };
        let mut stmt = c.prepare("SELECT id FROM assets WHERE pack_id = ?1 ORDER BY path")?;
        let ids: Vec<String> = stmt.query_map([&id2], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
        Ok(Some((pack.name, db::bundle_files(c, &ids)?)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("pack {id}")))?;
    Ok(crate::bundle::zip_files(&state, &name, &files)?)
}

/// Serve a pack's file by its original path. Browser model loaders resolve relative references
/// (`textures/wood.png` next to `crate.gltf`) against the model's URL, so this makes them just work.
async fn pack_file(
    State(state): State<AppState>,
    Path((id, path)): Path<(String, String)>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let p2 = path.clone();
    let (hash, ext) = db::call(&state.pool, move |c| db::pack_file(c, &id2, &p2))
        .await?
        .ok_or_else(|| AppError::not_found(format!("{path} in pack {id}")))?;
    let mime = mime_guess::from_ext(&ext).first_or_octet_stream().to_string();
    serve_blob(&state, &hash, &headers, &mime, None).await
}

async fn asset_derivative(
    State(state): State<AppState>,
    Path((id, kind)): Path<(String, String)>,
    Query(p): Query<FileParams>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let (id2, k2) = (id.clone(), kind.clone());
    let (hash, path, source_ext) = db::call(&state.pool, move |c| {
        let Some(hash) = db::derivative(c, &id2, &k2)? else { return Ok(None) };
        let (path, ext): (String, Option<String>) =
            c.query_row("SELECT path, json_extract(meta, '$.source_ext') FROM assets WHERE id = ?1", [&id2], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })?;
        Ok(Some((hash, path, ext)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("{kind} derivative of {id}")))?;
    // A recovered source file is served as what it is (a .wav out of a .uasset).
    let ext = match kind.as_str() {
        "glb" => "glb".to_owned(),
        "views" => "png".to_owned(),
        "source" => source_ext.unwrap_or_else(|| "bin".to_owned()),
        "preview" => "mp3".to_owned(),
        _ => "bin".to_owned(),
    };
    let mime = match ext.as_str() {
        "glb" => "model/gltf-binary".to_owned(),
        e => mime_guess::from_ext(e).first_or_octet_stream().to_string(),
    };
    let disposition = p.download.filter(|d| *d != 0).map(|_| {
        let file = path.rsplit('/').next().unwrap_or(&path);
        let stem = file.rsplit_once('.').map_or(file, |(s, _)| s).replace(['"', '\\'], "_");
        format!("attachment; filename=\"{stem}.{ext}\"")
    });
    serve_blob(&state, &hash, &headers, &mime, disposition).await
}

// ---------------------------------------------------------------------------------------------
// Download as

#[derive(Deserialize)]
struct AsParams {
    /// Scale images down so neither side exceeds this (a power of two, 64-16384).
    #[serde(default)]
    max: Option<u32>,
    /// `0` to serve inline (for previews) instead of as an attachment.
    #[serde(default)]
    download: Option<u8>,
    /// Seconds to wait for a conversion that isn't cached yet before answering 202 with its progress (0-60,
    /// default 20).
    #[serde(default)]
    wait: Option<u64>,
}

/// How long `/as/` waits for a conversion by default. Waiting holds no thread, only the connection.
const AS_WAIT_DEFAULT: u64 = 20;

/// Where a conversion stands, as JSON: `ready`, `queued`, `converting` (with `progress`) or `failed` (with `error`).
fn stage_json(stage: &crate::formats::Stage) -> Value {
    use crate::formats::Stage;
    match stage {
        Stage::Done(_) => json!({ "state": "ready" }),
        Stage::Queued => json!({ "state": "queued" }),
        Stage::Working(p) => json!({ "state": "converting", "progress": (f64::from(*p) * 100.0).round() / 100.0 }),
        Stage::Failed(e) => json!({ "state": "failed", "error": e }),
    }
}

fn refused(format: &str, r: crate::formats::Refused) -> AppError {
    use crate::formats::Refused;
    match r {
        Refused::NotFound => AppError::not_found("asset"),
        Refused::Unsupported(m) => AppError::bad_request(m),
        Refused::Failed(m) => AppError::unprocessable(format!("couldn't convert to {format}: {m}")),
    }
}

fn check_max(max: Option<u32>) -> ApiResult<()> {
    if let Some(m) = max
        && !crate::formats::valid_max(m)
    {
        return Err(AppError::bad_request("max must be a power of two from 64 to 16384"));
    }
    Ok(())
}

/// The asset converted to another format (images, audio, models, fonts; see `formats`), made on first request and
/// cached. A conversion that isn't cached is started (or joined, if another request started it) and waited for up
/// to `wait` seconds; if it's still running then, the answer is 202 with its progress and `Retry-After`. The
/// conversion carries on either way.
async fn asset_as(
    State(state): State<AppState>,
    Path((id, format)): Path<(String, String)>,
    Query(p): Query<AsParams>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    use crate::formats::{Lookup, Stage};
    check_max(p.max)?;
    let format = format.to_ascii_lowercase();
    let ready = match crate::formats::request(&state, &id, &format, p.max).await.map_err(|r| refused(&format, r))? {
        Lookup::Ready(r) => r,
        Lookup::Pending(rx) => {
            let limit = std::time::Duration::from_secs(p.wait.unwrap_or(AS_WAIT_DEFAULT).min(60));
            match crate::formats::wait(rx, limit).await {
                Stage::Done(r) => r,
                Stage::Failed(m) => return Err(AppError::unprocessable(format!("couldn't convert to {format}: {m}"))),
                pending => {
                    let mut resp = (StatusCode::ACCEPTED, Json(stage_json(&pending))).into_response();
                    resp.headers_mut().insert(header::RETRY_AFTER, HeaderValue::from_static("2"));
                    return Ok(resp);
                }
            }
        }
    };
    let name = match (ready.ext.as_str(), p.max) {
        ("zip", _) => format!("{}.gltf.zip", ready.stem),
        (e, Some(m)) => format!("{}_{m}.{e}", ready.stem),
        (e, None) => format!("{}.{e}", ready.stem),
    };
    let mime = match ready.ext.as_str() {
        "glb" => "model/gltf-binary".to_owned(),
        "exr" => "image/x-exr".to_owned(),
        "opus" => "audio/ogg".to_owned(),
        e => mime_guess::from_ext(e).first_or_octet_stream().to_string(),
    };
    let disposition =
        (p.download != Some(0)).then(|| format!("attachment; filename=\"{}\"", name.replace(['"', '\\'], "_")));
    serve_blob(&state, &ready.hash, &headers, &mime, disposition).await
}

/// Start (or look in on) a conversion without downloading it: `{"state": "ready" | "queued" | "converting" |
/// "failed", "progress"?, "error"?}`. With `wait`, answers as soon as anything changes or after that many seconds
/// (at most 10). The portal polls this, then downloads from `/as/{format}`.
async fn asset_as_status(
    State(state): State<AppState>,
    Path((id, format)): Path<(String, String)>,
    Query(p): Query<AsParams>,
) -> ApiResult<Json<Value>> {
    use crate::formats::{Lookup, Stage};
    check_max(p.max)?;
    let format = format.to_ascii_lowercase();
    Ok(Json(match crate::formats::request(&state, &id, &format, p.max).await.map_err(|r| refused(&format, r))? {
        Lookup::Ready(_) => stage_json(&Stage::Done(crate::formats::Ready {
            hash: String::new(),
            ext: String::new(),
            stem: String::new(),
        })),
        Lookup::Pending(rx) => {
            let limit = std::time::Duration::from_secs(p.wait.unwrap_or(0).min(10));
            stage_json(&crate::formats::wait_for_change(rx, limit).await)
        }
    }))
}

// ---------------------------------------------------------------------------------------------
// Unreal conversion

async fn unreal_convert(
    State(state): State<AppState>,
    Path(id): Path<String>,
    body: Option<Json<hoard_core::UnrealConvertRequest>>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let req = body.map(|Json(b)| b).unwrap_or_default();
    let job = crate::extract::Request {
        discard_originals: req.discard_originals,
        drop_unsupported: req.drop_unsupported,
        then_llm: None,
    };
    let id2 = id.clone();
    let queued = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(None);
        }
        Ok(Some(db::enqueue(c, jobs::UNREAL, Some(&id2), &serde_json::to_value(job)?)?))
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("pack {id}")))?;
    state.wake.notify_one();
    Ok((StatusCode::ACCEPTED, Json(json!({ "job": queued }))))
}

async fn unreal_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<hoard_core::UnrealPackStatus>> {
    let id2 = id.clone();
    db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(None);
        }
        crate::extract::pack_status(c, &id2).map(Some)
    })
    .await?
    .map(Json)
    .ok_or_else(|| AppError::not_found(format!("pack {id}")))
}

#[derive(Deserialize)]
struct PendingParams {
    #[serde(default)]
    limit: Option<u32>,
}

async fn unreal_pending(
    State(state): State<AppState>,
    Query(p): Query<PendingParams>,
) -> ApiResult<Json<Vec<hoard_core::PendingPackage>>> {
    let limit = p.limit.unwrap_or(500).clamp(1, 5000);
    Ok(Json(db::call(&state.pool, move |c| crate::extract::pending(c, limit)).await?))
}

#[derive(Deserialize)]
struct ReplaceParams {
    /// Extension of the uploaded file (`fbx`, `png`, …).
    ext: String,
    /// `editor` for files exported by Unreal Editor, `native` otherwise.
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    keep_original: Option<u8>,
    #[serde(default)]
    notes: Option<String>,
}

/// Swap an asset's file for a converted one (`hoard unreal-worker` uploads editor exports here).
async fn replace_asset(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<ReplaceParams>,
    body: axum::body::Bytes,
) -> ApiResult<Json<Asset>> {
    let ext = p.ext.trim_start_matches('.').to_ascii_lowercase();
    if ext.is_empty() || ext.len() > 12 || !ext.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AppError::bad_request("ext must be a short file extension"));
    }
    if body.is_empty() {
        return Err(AppError::bad_request("empty file"));
    }
    let id2 = id.clone();
    let class = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT COALESCE(u.class, json_extract(a.meta, '$.ue_class')) FROM assets a
             LEFT JOIN unreal_status u ON u.asset_id = a.id WHERE a.id = ?1",
            [&id2],
            |r| r.get::<_, Option<String>>(0),
        )
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("asset {id}")))?;
    let blob = state.blobs.put_bytes(body.to_vec()).await?;
    let origin = crate::extract::Origin {
        class,
        engine: None,
        dependencies: Vec::new(),
        method: if p.method.as_deref() == Some("native") { "native" } else { "editor" },
        notes: p.notes.into_iter().collect(),
    };
    let keep = p.keep_original.unwrap_or(1) != 0;
    let id2 = id.clone();
    let asset = db::call(&state.pool, move |c| {
        // Engine and dependencies come from the package, while we still have its analysis.
        let mut origin = origin;
        let meta: Option<String> =
            c.query_row("SELECT meta FROM assets WHERE id = ?1", [&id2], |r| r.get(0)).optional()?;
        if let Some(m) = meta.and_then(|m| serde_json::from_str::<Value>(&m).ok()) {
            origin.engine = m.get("ue_engine").and_then(Value::as_str).map(str::to_owned);
            origin.dependencies = m
                .get("ue_dependencies")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(|d| d.as_str().map(str::to_owned)).collect())
                .unwrap_or_default();
        }
        let tx = c.transaction()?;
        let kept = crate::extract::replace_asset(&tx, &id2, &blob, &ext, &origin, keep)?;
        tx.commit()?;
        if kept { db::get_asset(c, &id2) } else { Ok(None) }
    })
    .await?
    .ok_or_else(|| {
        AppError::conflict("the converted file duplicates one already in the pack; the package was merged into it")
    })?;
    Ok(Json(asset))
}

/// After a batch of replacements: retag the pack, queue previews (Blender) and free discarded originals.
async fn unreal_finish(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StatusCode> {
    let id2 = id.clone();
    let st = std::sync::Arc::clone(&state);
    let found = db::call(&state.pool, move |c| {
        if db::get_pack(c, &id2)?.is_none() {
            return Ok(false);
        }
        crate::convert::maybe_enqueue(&st, c, &id2)?;
        Ok(true)
    })
    .await?;
    if !found {
        return Err(AppError::not_found(format!("pack {id}")));
    }
    crate::extract::after_changes(&state, &id, true, None).await?;
    Ok(StatusCode::ACCEPTED)
}

/// The original file of a converted asset, when it was kept.
async fn asset_original(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let (hash, path) = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT blob_hash, path FROM asset_origin WHERE asset_id = ?1 AND blob_hash IS NOT NULL",
            [&id2],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        )
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("original file of {id}")))?;
    let file = path.rsplit('/').next().unwrap_or(&path).replace(['"', '\\'], "_");
    serve_blob(&state, &hash, &headers, "application/octet-stream", Some(format!("attachment; filename=\"{file}\"")))
        .await
}

// ---------------------------------------------------------------------------------------------
// Admin

/// What the server is doing in the background, for the portal's activity indicator: running work (with progress
/// where it's known), then jobs waiting their turn. `{"items": [{"kind", "label", "detail"?, "progress"?, "state",
/// "since", "pack_id"?}]}`.
async fn activity(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let mut items = crate::activity::snapshot();
    let queued = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT j.id, j.kind, j.pack_id, p.name, j.created_at FROM jobs j LEFT JOIN packs p ON p.id = j.pack_id
             WHERE j.status = 'queued' ORDER BY j.id LIMIT 50",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, String>(4)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    for (id, kind, pack_id, pack, created) in queued {
        items.push(crate::activity::Entry {
            id: u64::MAX - id.unsigned_abs(),
            kind: "job",
            label: jobs::job_label(&kind, pack.as_deref().unwrap_or("a pack")),
            detail: None,
            progress: None,
            state: "queued",
            since: created,
            pack_id,
        });
    }
    Ok(Json(json!({ "items": items })))
}

async fn admin_jobs(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let rows = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT id, kind, pack_id, status, attempts, error, created_at, updated_at FROM jobs ORDER BY id DESC LIMIT 50",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?, "kind": r.get::<_, String>(1)?, "pack_id": r.get::<_, Option<String>>(2)?,
                    "status": r.get::<_, String>(3)?, "attempts": r.get::<_, i64>(4)?,
                    "error": r.get::<_, Option<String>>(5)?, "created_at": r.get::<_, String>(6)?,
                    "updated_at": r.get::<_, String>(7)?,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!(rows)))
}

async fn admin_scrub(State(state): State<AppState>) -> ApiResult<StatusCode> {
    db::call(&state.pool, |c| db::enqueue(c, jobs::SCRUB, None, &json!({}))).await?;
    state.wake.notify_one();
    Ok(StatusCode::ACCEPTED)
}

async fn admin_gc(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let (count, bytes) = jobs::gc(&state).await?;
    Ok(Json(json!({ "deleted_blobs": count, "freed_bytes": bytes })))
}

/// Consistent snapshot of the metadata DB. Blobs are immutable, so back those up with any file/S3 sync.
async fn admin_backup(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let dir = state.cfg.backups_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let path = dir.join(format!("hoard-{}.db", db::now().replace([':', '-'], "").replace('T', "-")));
    let p = path.to_string_lossy().into_owned();
    db::call(&state.pool, move |c| {
        c.execute("VACUUM INTO ?1", [&p])?;
        Ok(())
    })
    .await?;
    Ok(Json(json!({ "path": path })))
}

// ---------------------------------------------------------------------------------------------
// Settings

/// A tool's line in the settings panel. `describe` is its `tools::Slot::describe` text.
fn tool_setting(value: &str, flag: &str, saved: bool, describe: &str, can_download: bool) -> Value {
    let (state, detail) = match describe {
        "off" | "looking" | "downloading" => (describe, ""),
        s => s.strip_prefix("unavailable: ").map_or(("ready", s), |why| ("unavailable", why)),
    };
    json!({
        "value": value,
        "default": flag,
        "saved": saved,
        "state": state,
        "detail": detail,
        "can_download": can_download,
    })
}

fn settings_json(state: &AppState) -> Value {
    let cfg = &state.cfg;
    let saved = state.settings.saved();
    let now = state.settings.get();
    json!({
        "blender": tool_setting(
            &now.blender, &cfg.blender, saved.blender.is_some(), &crate::blender::describe(), crate::blender::can_download(),
        ),
        "ffmpeg": tool_setting(
            &now.ffmpeg, &cfg.ffmpeg, saved.ffmpeg.is_some(), &crate::ffmpeg::describe(), crate::ffmpeg::can_download(),
        ),
        "unreal_convert": {
            "value": now.unreal_convert,
            "default": cfg.unreal_convert,
            "saved": saved.unreal_convert.is_some(),
        },
        // The key is never sent back, only whether there is one.
        "llm": {
            "url": now.llm_url.as_deref().unwrap_or(""),
            "model": now.llm_model.as_deref().unwrap_or(""),
            "has_key": now.llm_api_key.is_some(),
            "vision": now.llm_vision,
            "batch": option(now.llm_batch, cfg.llm_batch, saved.llm_batch.is_some()),
            "enabled": now.llm_enabled(),
            "saved": saved.llm_url.is_some() || saved.llm_model.is_some() || saved.llm_api_key.is_some()
                || saved.llm_vision.is_some(),
            "default": {
                "url": cfg.llm_url.as_deref().unwrap_or(""),
                "model": cfg.llm_model.as_deref().unwrap_or(""),
                "has_key": cfg.llm_api_key.is_some(),
                "vision": cfg.llm_vision,
            },
        },
        // Server options that used to need a restart. `default` is what the flags (or env) give.
        "options": {
            "workers": option(now.workers, cfg.workers, saved.workers.is_some()),
            "max_upload_mb": option(now.max_upload_mb, cfg.max_upload_mb, saved.max_upload_mb.is_some()),
            "max_expand_mb": option(now.max_expand_mb, cfg.max_expand_mb, saved.max_expand_mb.is_some()),
            "max_files": option(now.max_files, cfg.max_files, saved.max_files.is_some()),
            "render": option(now.render, cfg.render, saved.render.is_some()),
            "update_check": option(now.update_check, cfg.update_check, saved.update_check.is_some()),
            "public_url": option(
                now.public_url.as_deref().unwrap_or(""),
                cfg.public_url.as_deref().unwrap_or(""),
                saved.public_url.is_some(),
            ),
            "soundfont": option(
                now.soundfont.as_deref().map(|p| p.display().to_string()).unwrap_or_default(),
                cfg.soundfont.as_deref().map(|p| p.display().to_string()).unwrap_or_default(),
                saved.soundfont.is_some(),
            ),
        },
        "max_workers": crate::settings::MAX_WORKERS,
    })
}

#[derive(Deserialize)]
struct ModelsBody {
    url: String,
    /// Absent: the saved key, when `url` is the address in use. Empty: no key.
    api_key: Option<String>,
}

/// The models an AI service offers, for the Settings dialog. Doubles as a connection test.
async fn llm_models(State(state): State<AppState>, Json(body): Json<ModelsBody>) -> ApiResult<Json<Value>> {
    let url = body.url.trim();
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(AppError::bad_request("The AI address should start with http:// or https://"));
    }
    let now = state.settings.get();
    let key = match body.api_key {
        Some(k) => Some(k.trim().to_owned()).filter(|k| !k.is_empty()),
        None if now.llm_url.as_deref().map(|u| u.trim_end_matches('/')) == Some(url.trim_end_matches('/')) => {
            now.llm_api_key.clone()
        }
        None => None,
    };
    let models = crate::llm::list_models(&state.http, url, key.as_deref())
        .await
        .map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    Ok(Json(json!({ "models": models })))
}

fn option(value: impl serde::Serialize, default: impl serde::Serialize, saved: bool) -> Value {
    json!({ "value": value, "default": default, "saved": saved })
}

async fn get_settings(State(state): State<AppState>) -> Json<Value> {
    Json(settings_json(&state))
}

/// Change settings: a JSON object of the ones to change, `null` to go back to the server's flag. Whatever changed
/// takes effect at once (a tool switched on starts looking for, or downloading, its program).
async fn put_settings(State(state): State<AppState>, Json(patch): Json<Value>) -> ApiResult<Json<Value>> {
    let st = std::sync::Arc::clone(&state);
    let (before, after) = db::call(&state.pool, move |c| match st.settings.update(&st.cfg, c, &patch) {
        Ok(v) => Ok(Ok(v)),
        Err(Ok(msg)) => Ok(Err(msg)),
        Err(Err(e)) => Err(e),
    })
    .await?
    .map_err(AppError::bad_request)?;
    if before.blender != after.blender {
        crate::convert::start_blender(&state, true);
    }
    if before.ffmpeg != after.ffmpeg {
        crate::ffmpeg::start(&after.ffmpeg, state.cfg.tools_dir());
    }
    crate::analyze::set_render_enabled(after.render, after.llm_vision && after.llm_enabled());
    crate::updates::set_enabled(after.update_check);
    crate::music::configure(after.soundfont.clone());
    if after.workers > before.workers {
        state.wake.notify_waiters();
    }
    tracing::info!(
        blender = %after.blender,
        ffmpeg = %after.ffmpeg,
        unreal = ?after.unreal_convert,
        llm = after.llm_url.as_deref().unwrap_or("off"),
        "settings changed"
    );
    Ok(Json(settings_json(&state)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        let mut h = HeaderMap::new();
        h.insert(header::RANGE, HeaderValue::from_static("bytes=0-99"));
        assert_eq!(parse_range(&h), Some(GetRange::Bounded(0..100)));
        h.insert(header::RANGE, HeaderValue::from_static("bytes=500-"));
        assert_eq!(parse_range(&h), Some(GetRange::Offset(500)));
        h.insert(header::RANGE, HeaderValue::from_static("bytes=-200"));
        assert_eq!(parse_range(&h), Some(GetRange::Suffix(200)));
        h.insert(header::RANGE, HeaderValue::from_static("bytes=0-1,5-6"));
        assert_eq!(parse_range(&h), None);
    }

    #[test]
    fn pack_names() {
        assert_eq!(default_pack_name(&["kenney_ui.zip".into()]), "kenney_ui");
        assert_eq!(default_pack_name(&["Forest/a.png".into(), "Forest/b/c.wav".into()]), "Forest");
    }

    #[test]
    fn ext() {
        assert_eq!(crate::ingest::ext_of("a/b/Thing.PNG"), "png");
    }
}
