//! PBR texture sets as materials. Textures in one folder whose names differ only by a map type (and maybe a
//! resolution or file format) are one material: `Rock_BaseColor.png`, `Rock_Normal.png` and `Rock_Roughness.png`
//! become "Rock". The material is an asset of its own (kind `material`, ext `material`) at `<folder>/<name>.material`,
//! whose file is a small JSON list of its maps, with a sphere rendered from the maps as its thumbnail. Its maps
//! are linked to it as textures, so a download of the material brings them all, and they're listed in
//! `material_maps` with their role, which keeps them out of the grid (they count as supporting files).
//!
//! The same set at several resolutions (`Rock_2K_Color.jpg`, `Rock_4K_Color.jpg`, or `Rock_2K/…` and `Rock_4K/…`
//! folders) is one material with each map in several sizes.
//!
//! A store's JSON beside the maps (a Megascans `<id>.json`) names the material, says what each listed map is where
//! the file name doesn't, and gives a surface's real size and height, which the material's facts carry.
//!
//! A Unity material (`.mat`, text or binary) is read into one: its textures found through the GUIDs their `.meta`
//! files give them, and its color, metal, smoothness, glow, cutout and tiling kept as the material's look, which
//! its thumbnail and the 3D viewer draw it with. The `.mat` goes behind the material as its source file, and a
//! set found by name that shares a map with it gives way to it.
//!
//! A Wavefront material (`newmtl` in an OBJ's `.mtl`) with a color and no maps (as SketchUp exports them) is one too, a
//! swatch, worn by the OBJs whose `.mtl` defines it; the same name and color in several `.mtl` files of a folder is
//! one material. The `.mtl` itself is drawn as its materials side by side.
//!
//! Detection runs after a pack is ingested or re-tagged, and as a `materials` job for packs from before it existed.

mod engines;
mod models;
mod names;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use anyhow::Result;
use hoard_core::{AssetKind, MaterialInfo, MaterialLook, MaterialMap, TagSource};
use rusqlite::{Connection, OptionalExtension, params};
use tokio::runtime::Handle;

use crate::sidecar::Sidecar;
use crate::tagger::{self, TagSet};
use crate::{AppState, db, links};
use engines::{draw_mtls, mtl_materials, prefer_engines, unity_materials};
pub(crate) use models::fetch_model;
use models::{dress_models, link_models, mark_owned, read_material_names, recolor_stls, redraw_glass};
pub(crate) use names::{ALL_ROLES, Role, parse_map};
use names::{Candidate, Found, dir_of, find, role_of_word};

/// A material made from texture maps, as SQL over an asset `a`.
pub(crate) const IS_MATERIAL: &str = "(a.ext = 'material' AND json_extract(a.meta, '$.material_maps') IS NOT NULL)";

/// A material ready to write: its file and thumbnail are stored.
struct Made {
    found: Found,
    descriptor: crate::blobs::StoredBlob,
    thumb: Option<crate::blobs::StoredBlob>,
    physical_size: Option<[f32; 2]>,
    displacement: Option<f32>,
}

#[derive(Debug)]
struct Texture {
    id: String,
    path: String,
    hash: String,
    thumb: Option<String>,
    size_px: Option<u32>,
}

/// What the store's JSON files of the pack say each texture is (see [`Candidate::listed`]).
fn listed_roles<'a>(textures: &[Texture], sidecars: &'a [Sidecar]) -> Vec<Option<(Role, &'a str)>> {
    textures
        .iter()
        .map(|t| {
            sidecars.iter().find_map(|s| {
                let role = role_of_word(&s.listed_map(&t.path)?.kind)?;
                Some((role, s.id.as_deref().unwrap_or(s.title.as_str())))
            })
        })
        .collect()
}

/// The store's JSON that speaks for a material: the one listing its maps, or else the only one in its folder.
fn sidecar_of<'a>(sidecars: &'a [Sidecar], textures: &[Texture], f: &Found) -> Option<&'a Sidecar> {
    let listing = sidecars.iter().find(|s| f.maps.iter().any(|(i, ..)| s.listed_map(&textures[*i].path).is_some()));
    listing.or_else(|| {
        let dir = dir_of(&f.path);
        let mut beside = sidecars.iter().filter(|s| dir_of(&s.path) == dir);
        match (beside.next(), beside.next()) {
            (Some(s), None) => Some(s),
            _ => None,
        }
    })
}

/// Name materials after the store's file beside them: `<name>.json`, or else the only one in their folder that's
/// about one asset (it has the asset's id, as Megascans' `<id>.json` does).
fn name_from_sidecars(sidecars: &[Sidecar], found: &mut [Found]) {
    for f in found {
        let json = format!("{}.json", f.path.strip_suffix(".material").unwrap_or(&f.path)).to_lowercase();
        let dir = dir_of(&f.path).to_lowercase();
        let mut beside = sidecars.iter().filter(|s| s.id.is_some() && dir_of(&s.path).to_lowercase() == dir);
        let only = match (beside.next(), beside.next()) {
            (Some(s), None) => Some(s),
            _ => None,
        };
        if let Some(s) = sidecars.iter().find(|s| s.path.to_lowercase() == json).or(only) {
            f.name.clone_from(&s.title);
        }
    }
}

/// Find the pack's materials and bring the library in line: new ones added, changed ones updated (keeping their
/// id, tags people gave them and collections), ones whose maps are gone removed; then the thumbnails of models that
/// wear one redrawn with it on, turned as the pack's models are. Returns how many materials it has.
pub(crate) fn detect_pack(state: &AppState, rt: &Handle, conn: &mut Connection, pack_id: &str) -> Result<usize> {
    let facing = db::pack_facing(conn, pack_id)?;
    crate::meshes::facing(facing, || detect_turned(state, rt, conn, pack_id))
}

fn detect_turned(state: &AppState, rt: &Handle, conn: &mut Connection, pack_id: &str) -> Result<usize> {
    // First, so materials take their color map's AI tags.
    if let Err(e) = crate::provenance::backfill(state, rt, conn, pack_id) {
        tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not read images' provenance");
    }
    let n = find_materials(state, rt, conn, pack_id)?;
    match crate::atlas::detect_pack(state, rt, conn, pack_id) {
        Ok(0) => {}
        Ok(atlases) => tracing::info!(pack = pack_id, atlases, "sprite atlases found"),
        Err(e) => tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not read sprite atlases"),
    }
    match crate::alternates::detect_pack(conn, pack_id) {
        Ok(0) => {}
        Ok(assets) => tracing::info!(pack = pack_id, assets, "assets in several formats found"),
        Err(e) => tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not find assets in several formats"),
    }
    // Before dressing, which then redraws the dressed ones among them.
    if let Err(e) = redraw_glass(state, rt, conn, pack_id) {
        tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not redraw thumbnails of models with glass");
    }
    if let Err(e) = dress_models(state, rt, conn, pack_id) {
        tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not put materials on model thumbnails");
    }
    if let Err(e) = recolor_stls(state, rt, conn, pack_id) {
        tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not redraw STL thumbnails in color");
    }
    Ok(n)
}

impl Made {
    /// The material's facts, as its asset's `meta`.
    fn meta(&self) -> serde_json::Value {
        let f = &self.found;
        let roles: BTreeSet<&str> = f.maps.iter().map(|(_, r, _)| r.as_str()).collect();
        let mut resolutions: Vec<&str> = f.maps.iter().filter_map(|(_, _, r)| r.as_deref()).collect();
        resolutions.sort_by_key(|r| std::cmp::Reverse(res_px(r)));
        resolutions.dedup();
        let mut meta = serde_json::json!({ "material_maps": f.maps.len(), "roles": roles, "resolutions": resolutions });
        if let Some(look) = &f.look {
            meta["look"] = serde_json::to_value(look).unwrap_or_default();
        }
        if let Some(size) = self.physical_size {
            meta["physical_size"] = serde_json::json!(size);
        }
        if let Some(d) = self.displacement {
            meta["displacement"] = serde_json::json!(d);
        }
        meta
    }
}

/// A material's file: its maps by role, and a tiling surface's real size for an engine to scale its UVs by.
fn descriptor(
    f: &Found,
    textures: &[Texture],
    dir: &str,
    physical_size: Option<[f32; 2]>,
    displacement: Option<f32>,
) -> serde_json::Value {
    let mut descriptor = serde_json::json!({
        "hoard_material": 1,
        "name": f.name,
        "maps": f.maps.iter().map(|(i, role, res)| serde_json::json!({
            "role": role.as_str(),
            "file": db::strip_dir(&textures[*i].path, dir),
            "resolution": res,
            "hash": textures[*i].hash,
        })).collect::<Vec<_>>(),
    });
    if let Some(size) = physical_size {
        descriptor["physical_size_m"] = serde_json::json!(size);
    }
    if let Some(d) = displacement {
        descriptor["displacement_m"] = serde_json::json!(d);
    }
    if let Some(look) = &f.look {
        descriptor["look"] = serde_json::to_value(look).unwrap_or_default();
    }
    if let Some((_, source)) = f.sources.first() {
        descriptor["source"] = serde_json::json!(db::strip_dir(source, dir));
    }
    descriptor
}

/// The materials in a pack's textures, made into assets, and the models that use them linked.
fn find_materials(state: &AppState, rt: &Handle, conn: &mut Connection, pack_id: &str) -> Result<usize> {
    crate::issues::clear(
        conn,
        &[crate::issues::MATERIALS, crate::issues::MATERIAL_THUMB],
        crate::issues::At::pack(pack_id),
    );
    let textures = pack_textures(conn, pack_id)?;
    let sidecars = rt.block_on(crate::sidecar::pack_sidecars(state, pack_id))?;
    let listed = listed_roles(&textures, &sidecars);
    let candidates: Vec<Candidate<'_>> = textures
        .iter()
        .zip(&listed)
        .map(|(t, l)| Candidate { path: &t.path, size_px: t.size_px, listed: *l })
        .collect();
    let mut found = find(&candidates);
    name_from_sidecars(&sidecars, &mut found);
    let unity = unity_materials(state, rt, conn, pack_id, &textures)?;
    if !unity.is_empty()
        && let Err(e) = read_material_names(state, rt, conn, pack_id)
    {
        tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not read models' material names");
    }
    let found = prefer_engines(found, unity, mtl_materials(state, rt, conn, pack_id)?);
    if let Err(e) = draw_mtls(state, rt, conn, pack_id, &textures) {
        tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not draw MTL files");
    }
    let existing: HashMap<String, (String, String, bool, bool)> = {
        let mut stmt = conn.prepare(&format!(
            "SELECT a.path, a.id, a.blob_hash, a.thumb_hash IS NOT NULL, {IS_MATERIAL} FROM assets a WHERE a.pack_id = ?1
               AND a.ext = 'material'"
        ))?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, (r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))))?
            .collect::<rusqlite::Result<_>>()?
    };

    // Files and thumbnails first, outside any transaction.
    let mut made = Vec::new();
    for f in found {
        let dir = dir_of(&f.path).to_owned();
        let real = sidecar_of(&sidecars, &textures, &f);
        let physical_size = real.and_then(|s| s.physical_size).map(|(w, h)| [w, h]);
        let displacement = real.and_then(|s| s.displacement);
        let descriptor = descriptor(&f, &textures, &dir, physical_size, displacement);
        let bytes = serde_json::to_vec_pretty(&descriptor)?;
        let hash = blake3::hash(&bytes).to_hex().to_string();
        match existing.get(&f.path) {
            // A real file of that name: leave it be.
            Some((_, _, _, false)) => continue,
            // Unchanged since last time: nothing to remake.
            Some((_, old, true, true)) if *old == hash => {
                made.push(Made {
                    found: f,
                    descriptor: crate::blobs::StoredBlob { hash, size: bytes.len() as u64 },
                    thumb: None,
                    physical_size,
                    displacement,
                });
                continue;
            }
            _ => {}
        }
        let descriptor = rt.block_on(state.blobs.put_bytes(bytes, AssetKind::Material))?;
        let thumb = match render_thumb(state, rt, &textures, &f) {
            Ok(Some(png)) => Some(rt.block_on(state.blobs.put_cache_bytes(png))?),
            Ok(None) => None,
            Err(e) => {
                tracing::warn!(material = %f.path, error = %format!("{e:#}"), "no material thumbnail");
                let at = crate::issues::At::file(pack_id, &f.path);
                crate::issues::note(conn, crate::issues::MATERIAL_THUMB, at, &format!("{e:#}"));
                None
            }
        };
        made.push(Made { found: f, descriptor, thumb, physical_size, displacement });
    }

    let now = db::now();
    let were_maps = crate::naming::maps_of_pack(conn, pack_id)?;
    // A big pack has thousands: in slices, so nobody else waits seconds to write meanwhile.
    let mut slices = db::Slices::new(conn);
    let mut kept = HashSet::new();
    for m in &made {
        let tx = slices.tx()?;
        let f = &m.found;
        let id = existing.get(&f.path).map_or_else(|| uuid::Uuid::now_v7().to_string(), |(id, ..)| id.clone());
        kept.insert(id.clone());
        for b in [Some(&m.descriptor), m.thumb.as_ref()].into_iter().flatten() {
            tx.execute(
                "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
                params![b.hash, b.size as i64, now],
            )?;
        }
        let meta = m.meta();
        let tags = material_tags(tx, &textures, f)?;
        let category = tagger::categorize(AssetKind::Material, &tags);
        tx.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, category, meta, words, thumb_hash,
                                 created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 'material', 'material', ?6, ?7, ?8, ?9, ?10, ?10)
             ON CONFLICT (pack_id, path) DO UPDATE SET
                 blob_hash = excluded.blob_hash, meta = excluded.meta, words = excluded.words,
                 thumb_hash = COALESCE(excluded.thumb_hash, thumb_hash), updated_at = excluded.updated_at,
                 name = CASE WHEN human_edited THEN name ELSE excluded.name END,
                 category = CASE WHEN human_edited THEN category ELSE excluded.category END",
            params![
                id,
                pack_id,
                m.descriptor.hash,
                f.path,
                f.name,
                category,
                meta.to_string(),
                format!("{} material pbr{}", tagger::fts_words(&f.path), engine_words(f)),
                m.thumb.as_ref().map(|t| t.hash.clone()),
                now,
            ],
        )?;
        // A reviewed asset keeps the rule tags it was reviewed with.
        tx.execute(
            "DELETE FROM asset_tags WHERE asset_id = ?1 AND (source = 'meta' OR source = 'rule' AND NOT EXISTS
               (SELECT 1 FROM assets WHERE id = ?1 AND reviewed_at IS NOT NULL))",
            [&id],
        )?;
        for s in tags.into_vec() {
            db::upsert_tag(tx, &id, &s.tag, s.source, s.confidence)?;
        }
        db::reindex_asset(tx, &id)?;
        tx.execute("DELETE FROM material_maps WHERE material_id = ?1", [&id])?;
        tx.execute("DELETE FROM asset_links WHERE from_id = ?1 AND rel = 'texture'", [&id])?;
        for (i, role, res) in &f.maps {
            let t = &textures[*i].id;
            tx.execute(
                "INSERT INTO material_maps (material_id, texture_id, role, resolution) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT DO NOTHING",
                params![id, t, role.as_str(), res],
            )?;
            tx.execute(
                "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'texture') ON CONFLICT DO NOTHING",
                params![id, t],
            )?;
        }
        link_source(tx, &id, f)?;
    }
    // Materials whose maps are gone (or no longer form a set).
    for (id, _, _, synthetic) in existing.values() {
        if *synthetic && !kept.contains(id) {
            slices.tx()?.execute("DELETE FROM assets WHERE id = ?1", [id])?;
        }
    }
    tie_up(&mut slices, pack_id, &were_maps)?;
    slices.commit()?;
    Ok(made.len())
}

/// The pack's textures, by path.
fn pack_textures(conn: &Connection, pack_id: &str) -> Result<Vec<Texture>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.path, a.blob_hash, a.thumb_hash,
                MAX(COALESCE(json_extract(a.meta, '$.width'), 0), COALESCE(json_extract(a.meta, '$.height'), 0))
         FROM assets a WHERE a.pack_id = ?1 AND a.kind = 'texture' ORDER BY a.path",
    )?;
    let textures = stmt
        .query_map([pack_id], |r| {
            Ok(Texture {
                id: r.get(0)?,
                path: r.get(1)?,
                hash: r.get(2)?,
                thumb: r.get(3)?,
                size_px: r.get::<_, Option<i64>>(4)?.and_then(|n| u32::try_from(n).ok()).filter(|&n| n > 0),
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(textures)
}

/// Search words for where a material was read from.
fn engine_words(f: &Found) -> &'static str {
    match f.sources.first().map(|(_, p)| p.rsplit('.').next().unwrap_or("").to_ascii_lowercase()).as_deref() {
        Some("mat") => " unity",
        Some("mtl") => " mtl swatch color",
        _ => "",
    }
}

/// The engine's materials a material was read from go behind it.
fn link_source(tx: &Connection, id: &str, f: &Found) -> Result<()> {
    tx.execute(
        "DELETE FROM asset_links WHERE from_id = ?1 AND rel = 'sidecar'
           AND to_id IN (SELECT id FROM assets WHERE ext IN ('mat', 'mtl'))",
        [id],
    )?;
    for (source, _) in &f.sources {
        tx.execute(
            "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'sidecar') ON CONFLICT DO NOTHING",
            params![id, source],
        )?;
    }
    Ok(())
}

/// Once a pack's materials are in: name their maps, put the materials on the models, and group the pack's files.
/// Each step is its own transaction.
fn tie_up(slices: &mut db::Slices<'_>, pack_id: &str, were_maps: &[(String, String)]) -> Result<()> {
    slices.turn()?;
    crate::naming::rename_maps(slices.tx()?, pack_id, were_maps)?;
    slices.turn()?;
    link_models(slices.tx()?, pack_id)?;
    slices.turn()?;
    mark_owned(slices.tx()?, pack_id)?;
    slices.turn()?;
    links::assign_families(slices.tx()?, pack_id)?;
    Ok(())
}

/// Puts `thumb` in place of an asset's server thumbnail, its meta becoming `meta_sql` (`?2` is the new thumbnail's
/// hash). A browser may have sent its own meanwhile: that one stays, and this returns false.
fn replace_thumb(
    state: &AppState,
    rt: &Handle,
    conn: &Connection,
    id: &str,
    thumb: Vec<u8>,
    meta_sql: &str,
) -> Result<bool> {
    let stored = rt.block_on(state.blobs.put_cache_bytes(thumb))?;
    let old: Option<String> = conn.query_row("SELECT thumb_hash FROM assets WHERE id = ?1", [id], |r| r.get(0))?;
    conn.execute(
        "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
        params![stored.hash, stored.size as i64, db::now()],
    )?;
    let n = conn.execute(
        &format!("UPDATE assets SET thumb_hash = ?2, meta = {meta_sql} WHERE id = ?1 AND thumb_source IS NULL"),
        params![id, stored.hash],
    )?;
    if n > 0
        && let Some(old) = old.filter(|o| *o != stored.hash)
    {
        let size: Option<i64> =
            conn.query_row("SELECT size FROM blobs WHERE hash = ?1", [&old], |r| r.get(0)).optional()?;
        if let Some(size) = size {
            rt.block_on(crate::jobs::delete_unused(state, vec![(old, size)])).ok();
        }
    }
    Ok(n > 0)
}

/// `8K` → 8192, for sorting.
fn res_px(r: &str) -> u32 {
    r.strip_suffix('K').and_then(|n| n.parse::<u32>().ok()).map_or_else(|| r.parse().unwrap_or(0), |k| k * 1024)
}

/// A material's tags: what its color map (or first map) was tagged with, less what only describes a single file
/// (its map type, format, size or shape), plus the sizes it comes in.
fn material_tags(tx: &Connection, textures: &[Texture], f: &Found) -> Result<TagSet> {
    const DROP_PREFIX: &[&str] = &["map:", "set:", "format:", "depth:", "res:"];
    const DROP: &[&str] = &["pot", "square", "alpha", "spritesheet", "frames", "hdr", "hdri", "sky", "layered"];
    let lead = f.maps.iter().find(|(_, r, _)| *r == Role::BaseColor).or(f.maps.first());
    let mut stmt = tx.prepare_cached("SELECT tag, source, confidence FROM asset_tags WHERE asset_id = ?1")?;
    let rows: Vec<(String, String, f64)> = match lead {
        Some(lead) => stmt
            .query_map([&textures[lead.0].id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<rusqlite::Result<_>>()?,
        None => Vec::new(),
    };
    let mut tags = TagSet::default();
    for (tag, source, confidence) in rows {
        if DROP_PREFIX.iter().any(|p| tag.starts_with(p)) || DROP.contains(&tag.as_str()) {
            continue;
        }
        let source = match TagSource::parse(&source) {
            TagSource::Human => TagSource::Rule,
            s => s,
        };
        tags.add(&tag, source, confidence as f32);
    }
    for (_, _, res) in &f.maps {
        if let Some(px) = res.as_deref().map(res_px).filter(|&p| p > 0) {
            let bucket = match px {
                0..=256 => "res:256",
                257..=512 => "res:512",
                513..=1024 => "res:1k",
                1025..=2048 => "res:2k",
                2049..=4096 => "res:4k",
                _ => "res:8k+",
            };
            tags.add(bucket, TagSource::Meta, 1.0);
        }
    }
    tagger::vocab_tags(&f.path, AssetKind::Material, &mut tags);
    tags.add("pbr", TagSource::Rule, 0.9);
    Ok(tags)
}

/// Shade a sphere from the maps' thumbnails. None when there's nothing to shade it with.
fn render_thumb(state: &AppState, rt: &Handle, textures: &[Texture], f: &Found) -> Result<Option<Vec<u8>>> {
    sphere(state, rt, textures, f, crate::analyze::thumb_size())?
        .map(|img| crate::analyze::thumb_bytes(&img, false))
        .transpose()
}

/// [`render_thumb`]'s sphere, `size` pixels square.
fn sphere(
    state: &AppState,
    rt: &Handle,
    textures: &[Texture],
    f: &Found,
    size: u32,
) -> Result<Option<image::RgbaImage>> {
    use hoard_render::sphere::{Alpha, Factors, Maps, Packed, render_sphere};
    // The first map of each role that has a thumbnail (they're ordered largest first).
    let mut by_role: HashMap<Role, image::RgbaImage> = HashMap::new();
    for (i, role, _) in &f.maps {
        if by_role.contains_key(role) {
            continue;
        }
        let Some(hash) = &textures[*i].thumb else { continue };
        let bytes = rt.block_on(state.blobs.read_all(hash))?;
        match image::load_from_memory(&bytes) {
            Ok(img) => {
                by_role.insert(*role, img.to_rgba8());
            }
            Err(e) => tracing::debug!(map = %textures[*i].path, error = %e, "unreadable map thumbnail"),
        }
    }
    if by_role.is_empty() && f.look.is_none() {
        return Ok(None);
    }
    let factors = f.look.as_ref().map_or_else(Factors::default, |l| Factors {
        base_color: l.base_color,
        metallic: l.metallic,
        roughness: l.roughness,
        emissive: l.emissive,
        alpha: match l.alpha_mode.as_deref() {
            Some("mask") => Alpha::Mask(l.alpha_cutoff.unwrap_or(0.5)),
            Some("blend") => Alpha::Blend,
            _ => Alpha::Opaque,
        },
        uv_scale: l.uv_scale.unwrap_or([1.0, 1.0]),
        uv_offset: l.uv_offset.unwrap_or([0.0, 0.0]),
        normal_scale: l.normal_scale.unwrap_or(1.0),
        occlusion_strength: l.occlusion_strength.unwrap_or(1.0),
    });
    let mut take = |r: Role| by_role.remove(&r);
    let (normal, normal_dx) = match take(Role::Normal) {
        Some(n) => (Some(n), false),
        None => (take(Role::NormalDx), true),
    };
    let (roughness, gloss) = match take(Role::Roughness) {
        Some(r) => (Some(r), false),
        None => (take(Role::Gloss), true),
    };
    let packed = take(Role::Orm)
        .map(|i| (i, Packed::Orm))
        .or_else(|| take(Role::Mra).map(|i| (i, Packed::Mra)))
        .or_else(|| take(Role::Rma).map(|i| (i, Packed::Rma)));
    let maps = Maps {
        base_color: take(Role::BaseColor),
        normal,
        normal_dx,
        roughness,
        gloss,
        metalness: take(Role::Metalness),
        ao: take(Role::Ao),
        cavity: take(Role::Cavity),
        height: take(Role::Height).or_else(|| take(Role::Bump)),
        opacity: take(Role::Opacity),
        emissive: take(Role::Emissive),
        packed,
        factors,
    };
    Ok(Some(render_sphere(&maps, size)))
}

/// One material with its maps, for an asset's detail.
fn info(conn: &Connection, id: &str) -> Result<Option<MaterialInfo>> {
    let head: Option<(String, String, bool, Option<String>)> = conn
        .query_row("SELECT name, path, thumb_hash IS NOT NULL, meta FROM assets WHERE id = ?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .optional()?;
    let Some((name, path, has_thumb, meta)) = head else { return Ok(None) };
    let meta: serde_json::Value = meta.and_then(|m| serde_json::from_str(&m).ok()).unwrap_or_default();
    let physical_size = serde_json::from_value::<[f32; 2]>(meta["physical_size"].clone()).ok();
    #[expect(clippy::cast_possible_truncation, reason = "meters, a handful of digits")]
    let displacement = meta["displacement"].as_f64().map(|d| d as f32);
    let mut stmt = conn.prepare_cached(
        "SELECT a.id, a.name, a.path, a.ext, m.role, m.resolution, json_extract(a.meta, '$.width'),
                json_extract(a.meta, '$.height'), a.thumb_hash IS NOT NULL, b.size
         FROM material_maps m JOIN assets a ON a.id = m.texture_id JOIN blobs b ON b.hash = a.blob_hash
         WHERE m.material_id = ?1",
    )?;
    let mut maps: Vec<MaterialMap> = stmt
        .query_map([id], |r| {
            Ok(MaterialMap {
                id: r.get(0)?,
                name: r.get(1)?,
                path: r.get(2)?,
                ext: r.get(3)?,
                role: r.get(4)?,
                resolution: r.get(5)?,
                width: r.get::<_, Option<i64>>(6)?.and_then(|n| u32::try_from(n).ok()),
                height: r.get::<_, Option<i64>>(7)?.and_then(|n| u32::try_from(n).ok()),
                has_thumb: r.get(8)?,
                size: r.get::<_, i64>(9)? as u64,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    let order = |m: &MaterialMap| Role::parse(&m.role).map_or(usize::MAX, |r| r as usize);
    maps.sort_by(|a, b| {
        order(a).cmp(&order(b)).then(b.width.unwrap_or(0).cmp(&a.width.unwrap_or(0))).then(a.path.cmp(&b.path))
    });
    let look = serde_json::from_value::<MaterialLook>(meta["look"].clone()).ok();
    Ok(Some(MaterialInfo {
        id: id.to_owned(),
        name,
        path,
        has_thumb,
        maps,
        physical_size,
        displacement,
        look,
        slots: Vec::new(),
    }))
}

/// Fill in an asset's material facts: a material's maps, or the materials a texture is part of or a model uses
/// (through a map it links to, or linked by name).
pub(crate) fn attach(conn: &Connection, asset: &mut hoard_core::Asset) -> Result<()> {
    let is_material = conn
        .query_row(&format!("SELECT {IS_MATERIAL} FROM assets a WHERE a.id = ?1"), [&asset.id], |r| r.get(0))
        .optional()?
        .unwrap_or(false);
    if is_material {
        asset.material = info(conn, &asset.id)?;
        return Ok(());
    }
    let mut stmt = conn.prepare_cached(&format!(
        "SELECT DISTINCT id FROM (
           SELECT m.material_id AS id FROM material_maps m WHERE m.texture_id = ?1
           UNION SELECT m.material_id FROM asset_links l JOIN material_maps m ON m.texture_id = l.to_id
             WHERE l.from_id = ?1
           UNION SELECT l.to_id FROM asset_links l JOIN assets a ON a.id = l.to_id
             WHERE l.from_id = ?1 AND l.rel = 'material' AND {IS_MATERIAL}
         ) LIMIT 16"
    ))?;
    let ids: Vec<String> = stmt.query_map([&asset.id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    let slots: BTreeMap<String, String> =
        asset.meta.get("material_slots").and_then(|s| serde_json::from_value(s.clone()).ok()).unwrap_or_default();
    for id in ids {
        if let Some(mut m) = info(conn, &id)? {
            m.slots = slots.iter().filter(|(_, mat)| **mat == id).map(|(slot, _)| slot.clone()).collect();
            asset.materials.push(m);
        }
    }
    Ok(())
}

/// [`detect_pack`] as a step of ingesting or re-tagging: a pack whose materials can't be worked out is still a
/// good pack, so a failure is only logged.
pub(crate) fn detect_logged(state: &AppState, rt: &Handle, conn: &mut Connection, pack_id: &str) {
    match detect_pack(state, rt, conn, pack_id) {
        Ok(0) => {}
        Ok(n) => tracing::info!(pack = pack_id, materials = n, "materials found"),
        Err(e) => {
            tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not look for materials");
            crate::issues::note(conn, crate::issues::MATERIALS, crate::issues::At::pack(pack_id), &format!("{e:#}"));
        }
    }
}

/// Run detection over one pack, as the `materials` job does.
pub(crate) fn run_job(state: &AppState, rt: &Handle, pack_id: &str) -> Result<()> {
    let mut conn = state.pool.get()?;
    let n = detect_pack(state, rt, &mut conn, pack_id)?;
    tracing::info!(pack = pack_id, materials = n, "materials found");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_from_json_beside_the_maps() {
        let file = |path: &str, json: &str| crate::sidecar::read(path, json).unwrap();
        let sidecars = [
            file(
                "Plate/tkxmfcwa.json",
                r#"{"id": "tkxmfcwa", "semanticTags": {"name_formats": {"title": "Metal  Plate"}}}"#,
            ),
            file("Iron/Rusty.json", r#"{"semanticTags": {"name": "Rusty Iron"}}"#),
        ];
        let mut found: Vec<Found> = ["Plate/Plate.material", "Iron/Rusty.material", "Other/Thing.material"]
            .into_iter()
            .map(|p| Found {
                path: p.to_owned(),
                name: "x".to_owned(),
                key: String::new(),
                maps: Vec::new(),
                look: None,
                sources: Vec::new(),
            })
            .collect();
        name_from_sidecars(&sidecars, &mut found);
        assert_eq!(found.iter().map(|f| f.name.as_str()).collect::<Vec<_>>(), ["Metal Plate", "Rusty Iron", "x"]);
        assert!(crate::sidecar::read("a.json", r#"{"name": "  ", "id": "a", "meta": []}"#).is_none());
    }
}
