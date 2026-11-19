//! Zips: an asset with what it needs, a selection, a collection or a pack, and the manifest an engine plugin asks for.

use std::collections::BTreeSet;

use anyhow::Context;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::Response;
use hoard_core::{Asset, Manifest, ManifestEntry, ManifestRequest};
use rusqlite::OptionalExtension as _;
use serde::Deserialize;

use crate::AppState;
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;

/// Flatten a selection (explicit ids and/or a search) into an engine-friendly list of files.
pub(super) async fn manifest(
    State(state): State<AppState>,
    Json(req): Json<ManifestRequest>,
) -> ApiResult<Json<Manifest>> {
    let names = state.settings.get();
    let entries = db::call(&state.pool, move |c| {
        let names = &names.texture_names;
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
                let file_name = crate::naming::file_name(c, names, &a.id, &a.path)?;
                entries.push(manifest_entry(a, None, file_name));
                continue;
            }
            // A bundle: the root plus its dependencies, with paths relative to their common folder.
            let mut files = db::bundle_files(c, &closure)?;
            let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
            let common = db::common_dir(&paths);
            let original: Vec<String> = files.iter().map(|f| f.path.clone()).collect();
            crate::naming::apply(c, names, &mut files)?;
            let root = a.id.clone();
            for (f, was) in files.iter().zip(&original) {
                let member = if f.id == root { Some(a.clone()) } else { db::get_asset(c, &f.id)? };
                if let Some(m) = member {
                    let bundle_path = db::strip_dir(&f.path, &common).to_owned();
                    let file_name = (f.path != *was).then(|| f.path.rsplit('/').next().unwrap_or(&f.path).to_owned());
                    entries.push(manifest_entry(m, Some((root.clone(), bundle_path)), file_name));
                }
            }
        }
        Ok(entries)
    })
    .await?;
    Ok(Json(Manifest { generated_at: db::now(), entries }))
}

fn manifest_entry(a: Asset, bundle: Option<(String, String)>, file_name: Option<String>) -> ManifestEntry {
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
        file_name,
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct BundleParams {
    #[serde(default)]
    animations: Option<u8>,
    /// For an animation: a character (model) to put in with it, and what it needs; one from another pack goes in a
    /// folder named for its pack.
    #[serde(default)]
    character: Option<String>,
}

/// An asset and everything it needs, zipped on the fly.
pub(super) async fn asset_bundle(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(p): Query<BundleParams>,
) -> ApiResult<Response> {
    let with_anims = p.animations.is_some_and(|a| a != 0);
    let character = p.character;
    let id2 = id.clone();
    let names = state.settings.get();
    let (files, root_name) = db::call(&state.pool, move |c| {
        let Some((_, path, _)) = db::asset_file_info(c, &id2)? else { return Ok(None) };
        let ids = db::dependency_closure(c, &id2, with_anims)?;
        let stem =
            path.rsplit('/').next().unwrap_or(&path).rsplit_once('.').map_or(path.as_str(), |(s, _)| s).to_owned();
        let mut files = db::bundle_files(c, &ids)?;
        if let Some(character) = &character {
            files.extend(character_files(c, &id2, character, &ids)?);
        }
        crate::naming::apply(c, &names.texture_names, &mut files)?;
        Ok(Some((files, stem)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    Ok(crate::bundle::zip_files(&state, &root_name, &files, None)?)
}

/// A character to put in an animation's zip and the files it needs, those not already in (`have`); under its pack's
/// name when it's from another pack than the animation. Nothing for an id that isn't a model.
fn character_files(
    c: &rusqlite::Connection,
    animation: &str,
    character: &str,
    have: &[String],
) -> anyhow::Result<Vec<db::BundleFile>> {
    let packs: Option<(String, String, String)> = c
        .query_row(
            "SELECT ch.pack_id, p.name, an.pack_id FROM assets ch JOIN packs p ON p.id = ch.pack_id, assets an
             WHERE ch.id = ?1 AND ch.kind = 'model' AND an.id = ?2",
            [character, animation],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .optional()?;
    let Some((pack, pack_name, own)) = packs else { return Ok(Vec::new()) };
    let ids: Vec<String> =
        db::dependency_closure(c, character, false)?.into_iter().filter(|id| !have.contains(id)).collect();
    let mut files = db::bundle_files(c, &ids)?;
    if pack != own {
        let folder = pack_name.replace(['/', '\\'], "_");
        for f in &mut files {
            f.path = format!("{folder}/{}", f.path);
        }
    }
    Ok(files)
}

#[derive(Debug, Deserialize)]
pub(super) struct SelectionParams {
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
    /// Every model as a GLB with its textures beside it (see [`glb_bundle`]).
    #[serde(default)]
    glb: Option<u8>,
}

const MAX_SELECTION: usize = 1000;

/// Several assets (and what they need) in one zip. Files keep their layout inside a folder per pack, so two packs'
/// `Audio/hit.wav` don't collide and models still find their textures.
pub(super) async fn selection_bundle(
    State(state): State<AppState>,
    Query(p): Query<SelectionParams>,
) -> ApiResult<Response> {
    let ids: Vec<String> = p.ids.split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned).collect();
    if ids.is_empty() || ids.len() > MAX_SELECTION {
        return Err(AppError::bad_request(tr!("error-selection-size", max = MAX_SELECTION)));
    }
    if p.glb.is_some_and(|g| g != 0) {
        return glb_bundle(&state, ids).await;
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
    let names = state.settings.get();
    let (groups, credits) = db::call(&state.pool, move |c| {
        let mut all = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for id in &ids {
            let closure = if opts.deps { db::dependency_closure(c, id, opts.animations)? } else { vec![id.clone()] };
            all.extend(closure.into_iter().filter(|x| seen.insert(x.clone())));
        }
        let mut stmt =
            c.prepare("SELECT p.id, p.name FROM assets a JOIN packs p ON p.id = a.pack_id WHERE a.id = ?1")?;
        let mut by_pack: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
        let mut pack_ids = BTreeSet::new();
        for id in all {
            let pack = stmt.query_row([&id], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))).optional()?;
            if let Some((pack_id, pack)) = pack {
                pack_ids.insert(pack_id);
                by_pack.entry(pack).or_default().push(id);
            }
        }
        let credits = crate::credits::markdown(&crate::credits::for_packs(c, &pack_ids)?);
        let groups = by_pack
            .into_iter()
            .map(|(pack, ids)| {
                let mut files = db::bundle_files(c, &ids)?;
                if opts.sources {
                    db::prefer_sources(c, &mut files)?;
                }
                crate::naming::apply(c, &names.texture_names, &mut files)?;
                Ok((pack, files))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok((groups, credits))
    })
    .await?;
    if groups.is_empty() {
        return Err(AppError::not_found(tr!("error-those-assets-not-found")));
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
    Ok(crate::bundle::zip_response(state, &root, entries, Some(credits))?)
}

/// One file of a [`glb_bundle`].
struct GlbItem {
    id: String,
    /// Its file name.
    file: String,
    hash: String,
    size: u64,
    /// Whether it's a model that can be made a GLB.
    glb: bool,
}

/// How many models are converted ahead of the one being written, so the zip doesn't wait on each in turn.
const GLB_AHEAD: usize = 3;

/// How long one model is waited for before its own file goes in instead.
const GLB_WAIT: std::time::Duration = std::time::Duration::from_mins(30);

/// Several assets in one zip, every model as a GLB standing Y up and facing +Z (turned the way its pack's models
/// face), its textures in a `textures` folder the models share, each texture once. Anything else, and a model that
/// can't be converted, goes in as it is. The zip streams as the models are converted.
async fn glb_bundle(state: &AppState, ids: Vec<String>) -> ApiResult<Response> {
    let (items, credits) = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare(
            "SELECT a.id, a.path, a.ext, a.blob_hash, b.size, a.pack_id
             FROM assets a JOIN blobs b ON b.hash = a.blob_hash WHERE a.id = ?1",
        )?;
        let mut derivs = c.prepare("SELECT kind FROM derivatives WHERE asset_id = ?1")?;
        let mut items = Vec::new();
        let mut packs = BTreeSet::new();
        let mut seen = std::collections::HashSet::new();
        for id in ids.iter().filter(|id| seen.insert(id.as_str())) {
            let row = stmt
                .query_row([id], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                        r.get::<_, i64>(4)?,
                        r.get::<_, String>(5)?,
                    ))
                })
                .optional()?;
            let Some((id, path, ext, hash, size, pack)) = row else { continue };
            let kinds: Vec<String> = derivs.query_map([&id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
            let glb = crate::formats::options(&ext, &kinds).iter().any(|o| o.format == "glb");
            let file = path.rsplit('/').next().unwrap_or(&path).to_owned();
            packs.insert(pack);
            items.push(GlbItem { id, file, hash, size: u64::try_from(size).unwrap_or(0), glb });
        }
        let credits = crate::credits::markdown(&crate::credits::for_packs(c, &packs)?);
        Ok((items, credits))
    })
    .await?;
    if items.is_empty() {
        return Err(AppError::not_found(tr!("error-those-assets-not-found")));
    }
    let st = std::sync::Arc::clone(state);
    let root = "hoard-glb";
    Ok(crate::bundle::zip_stream(state, root, move |sink| fill_glb_bundle(&st, sink, root, &items, &credits))?)
}

/// Write a [`glb_bundle`]'s files.
fn fill_glb_bundle(
    state: &AppState,
    sink: &mut crate::bundle::Sink<'_>,
    root: &str,
    items: &[GlbItem],
    credits: &str,
) -> anyhow::Result<()> {
    use crate::glbfiles::{Shelf, TEXTURES};
    let mut files = Shelf::default();
    let mut textures = Shelf::default();
    files.name(TEXTURES);
    let credits_name = files.name("CREDITS.md");
    sink.bytes(&format!("{root}/{credits_name}"), credits.as_bytes())?;
    let total = items.len() as f32;
    sink.activity.detail(format!("{} files", items.len()));
    for (i, item) in items.iter().enumerate() {
        // Ask for the next few now: each is converted (or found ready) while the ones before it are written.
        for next in items[i..].iter().filter(|x| x.glb).take(GLB_AHEAD) {
            sink.rt().block_on(crate::formats::request(state, &next.id, "glb", None)).ok();
        }
        sink.activity.progress(i as f32 / total);
        if item.glb {
            match sink.rt().block_on(converted_glb(state, &item.id)) {
                Ok(ready) => {
                    let bytes = sink.read(&ready.hash)?;
                    let own = if ready.ext == "zip" {
                        unzip_glb(&bytes)?
                    } else {
                        crate::glbfiles::Split { glb: bytes.to_vec(), textures: Vec::new() }
                    };
                    let mut rename = std::collections::BTreeMap::new();
                    for (name, data) in own.textures {
                        let (placed, new) = textures.place(&name, &data);
                        if new {
                            sink.bytes(&format!("{root}/{TEXTURES}/{placed}"), &data)?;
                        }
                        if placed != name {
                            rename.insert(name, placed);
                        }
                    }
                    let glb = crate::glbfiles::rename_textures(&own.glb, &rename)?;
                    let name = files.name(&format!("{}.glb", crate::glbfiles::clean(&ready.stem)));
                    sink.bytes(&format!("{root}/{name}"), &glb)?;
                    continue;
                }
                Err(e) => tracing::warn!(asset = %item.id, error = %format!("{e:#}"), "couldn't make a GLB to zip"),
            }
        }
        let name = files.name(&item.file);
        sink.blob(&format!("{root}/{name}"), &item.hash, item.size, |_| i as f32 / total)?;
    }
    Ok(())
}

/// An asset's GLB download, waiting for it to be made.
async fn converted_glb(state: &AppState, id: &str) -> anyhow::Result<crate::formats::Ready> {
    use crate::formats::{Lookup, Refused, Stage};
    let lookup = crate::formats::request(state, id, "glb", None).await.map_err(|r| match r {
        Refused::NotFound => anyhow::anyhow!("not found"),
        Refused::Unsupported(m) | Refused::Failed(m) => anyhow::anyhow!(m),
    })?;
    match lookup {
        Lookup::Ready(r) => Ok(r),
        Lookup::Pending(rx) => match crate::formats::wait(rx, GLB_WAIT).await {
            Stage::Done(r) => Ok(r),
            Stage::Failed(m) => Err(anyhow::anyhow!(m)),
            _ => Err(anyhow::anyhow!("still converting after {} minutes", GLB_WAIT.as_secs() / 60)),
        },
    }
}

/// A GLB download's zip: the GLB, and its textures by file name.
fn unzip_glb(bytes: &[u8]) -> anyhow::Result<crate::glbfiles::Split> {
    use std::io::Read;
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let (mut glb, mut textures) = (None, Vec::new());
    for i in 0..zip.len() {
        let mut f = zip.by_index(i)?;
        let name = f.name().unwrap_or_default().into_owned();
        let mut data = Vec::new();
        f.read_to_end(&mut data)?;
        if let Some(t) = name.strip_prefix(&format!("{}/", crate::glbfiles::TEXTURES)) {
            textures.push((t.to_owned(), data));
        } else if name.to_ascii_lowercase().ends_with(".glb") {
            glb = Some(data);
        }
    }
    Ok(crate::glbfiles::Split { glb: glb.context("the GLB's zip has no GLB")?, textures })
}

#[derive(Debug, Deserialize)]
pub(super) struct CollectionZipParams {
    #[serde(default)]
    animations: Option<u8>,
    #[serde(default)]
    sources: Option<u8>,
}

pub(super) async fn collection_bundle(
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
    .ok_or_else(|| AppError::not_found(tr!("error-collection-not-found", id = id)))?;
    if ids.is_empty() {
        return Err(AppError::bad_request(tr!("error-collection-empty", name = name)));
    }
    let opts = ZipOptions {
        deps: true,
        animations: p.animations.is_some_and(|a| a != 0),
        sources: p.sources.is_some_and(|s| s != 0),
    };
    zip_assets(&state, ids, opts, &name).await
}

/// A whole pack as one zip, in its original layout.
pub(super) async fn pack_bundle(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Response> {
    let id2 = id.clone();
    let names = state.settings.get();
    let (name, files, credits) = db::call(&state.pool, move |c| {
        let Some(pack) = db::get_pack(c, &id2)? else { return Ok(None) };
        let mut stmt = c.prepare("SELECT id FROM assets WHERE pack_id = ?1 ORDER BY path")?;
        let ids: Vec<String> = stmt.query_map([&id2], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
        let credits = crate::credits::markdown(&crate::credits::for_packs(c, &BTreeSet::from([pack.id]))?);
        let mut files = db::bundle_files(c, &ids)?;
        crate::naming::apply(c, &names.texture_names, &mut files)?;
        Ok(Some((pack.name, files, credits)))
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))?;
    Ok(crate::bundle::zip_files(&state, &name, &files, Some(credits))?)
}
