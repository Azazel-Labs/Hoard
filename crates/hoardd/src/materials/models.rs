//! The models that wear a pack's materials: linked to them by what their files name, put in the slots Unity's
//! `.meta` files say, their thumbnails drawn again wearing them, and a material only one object wears made part of it.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use rusqlite::{Connection, OptionalExtension, params};
use tokio::runtime::Handle;

use super::names::{dir_key, dir_of, stem_of, words};
use super::{IS_MATERIAL, replace_thumb};
use crate::{AppState, db};

/// Models whose file names no textures but which wear a material (by name, or through its maps beside them) get a
/// white thumbnail when they're read, before the material is found. Render them again with the material's color
/// map on, as the 3D viewers show them: FBX, OBJ, glTF, GLB, STL and PLY. A thumbnail a browser drew is left alone,
/// and one already dressed isn't redone, unless the model's materials changed since (marked `redress`): then it's
/// dressed in its new ones, or drawn plain again when it wears none.
pub(super) fn dress_models(state: &AppState, rt: &Handle, conn: &Connection, pack_id: &str) -> Result<usize> {
    let todo: Vec<(String, String, String)> = conn
        .prepare(
            "SELECT o.id, o.ext, MAX(t.thumb_hash) FROM assets o
               JOIN material_maps m ON m.role = 'base_color' AND EXISTS (
                    SELECT 1 FROM asset_links l WHERE l.from_id = o.id
                       AND ((l.rel = 'material' AND l.to_id = m.material_id)
                            OR l.to_id IN (SELECT x.texture_id FROM material_maps x WHERE x.material_id = m.material_id)))
               JOIN assets t ON t.id = m.texture_id AND t.thumb_hash IS NOT NULL
              WHERE o.pack_id = ?1 AND o.kind = 'model' AND o.ext IN ('fbx', 'obj', 'glb', 'gltf', 'stl', 'ply')
                AND o.thumb_source IS NULL AND o.thumb_hash IS NOT NULL
                AND o.thumb_hash IS NOT json_extract(o.meta, '$.thumb_dressed')
                AND json_extract(o.meta, '$.material_slots') IS NULL
              GROUP BY o.id",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let slotted: Vec<(String, String, String)> = conn
        .prepare(
            "SELECT id, ext, json_extract(meta, '$.material_slots') FROM assets
              WHERE pack_id = ?1 AND kind = 'model' AND ext IN ('fbx', 'obj', '3ds')
                AND thumb_source IS NULL AND thumb_hash IS NOT NULL
                AND thumb_hash IS NOT json_extract(meta, '$.thumb_dressed')
                AND json_extract(meta, '$.material_slots') IS NOT NULL",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let redress: Vec<(String, String)> = conn
        .prepare(
            "SELECT id, ext FROM assets
              WHERE pack_id = ?1 AND kind = 'model' AND json_extract(meta, '$.redress') IS NOT NULL",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    if (todo.is_empty() && slotted.is_empty() && redress.is_empty()) || crate::analyze::renderer().is_none() {
        return Ok(0);
    }
    let mut dressed_ids = HashSet::new();
    std::fs::create_dir_all(state.cfg.tmp_dir())?;
    let work = tempfile::Builder::new().prefix("dress-").tempdir_in(state.cfg.tmp_dir())?;
    let mut done = 0;
    for (id, ext, slots) in slotted {
        let dir = work.path().join(&id);
        let dressed = turned_as(conn, &id, || dress_slots(state, rt, conn, &id, &ext, &slots, &dir))?;
        std::fs::remove_dir_all(&dir).ok();
        match dressed {
            Ok(Some(thumb)) => {
                if replace_thumb(state, rt, conn, &id, thumb, "json_set(meta, '$.thumb_dressed', ?2)")? {
                    done += 1;
                    dressed_ids.insert(id);
                }
            }
            Ok(None) => {}
            Err(e) => tracing::debug!(asset = %id, error = %format!("{e:#}"), "model thumbnail kept undressed"),
        }
    }
    for (id, ext, color) in todo {
        let dir = work.path().join(&id);
        let dressed = turned_as(conn, &id, || -> Result<Option<Vec<u8>>> {
            let file = fetch_model(state, rt, conn, &id, &dir)?;
            if matches!(ext.as_str(), "glb" | "gltf") {
                let color = rt.block_on(state.blobs.read_all(&color))?;
                return crate::analyze::gltf_thumb_dressed(&file, &color);
            }
            let (Some(mut mesh), _) = crate::meshes::load(&file, &ext, &dir)? else { return Ok(None) };
            // Maps of its own after all (now that the files beside it are here): drawn with those.
            if mesh.materials.iter().any(|m| m.color_image.is_some()) {
                return crate::analyze::mesh_thumb_bytes(&mesh, &file);
            }
            let bytes = rt.block_on(state.blobs.read_all(&color))?.to_vec();
            let mime = crate::analyze::thumb_mime(&bytes);
            mesh.images.push(crate::meshes::Image { bytes, mime });
            let image = mesh.images.len() - 1;
            if mesh.materials.is_empty() {
                mesh.materials.push(crate::meshes::Material::plain("material", [1.0; 4]));
            }
            for m in &mut mesh.materials {
                m.color_image = Some(image);
                m.color = [1.0, 1.0, 1.0, m.color[3]];
            }
            crate::analyze::mesh_thumb_bytes(&mesh, &file)
        })?;
        std::fs::remove_dir_all(&dir).ok();
        let thumb = match dressed {
            Ok(Some(t)) => t,
            Ok(None) => continue,
            Err(e) => {
                tracing::debug!(asset = %id, error = %format!("{e:#}"), "model thumbnail kept undressed");
                continue;
            }
        };
        if replace_thumb(state, rt, conn, &id, thumb, "json_set(meta, '$.thumb_dressed', ?2)")? {
            done += 1;
            dressed_ids.insert(id);
        }
    }
    for (id, ext) in redress {
        if !dressed_ids.contains(&id) {
            let dir = work.path().join(&id);
            let plain = turned_as(conn, &id, || -> Result<Option<Vec<u8>>> {
                let file = fetch_model(state, rt, conn, &id, &dir)?;
                let (Some(mesh), _) = crate::meshes::load(&file, &ext, &dir)? else { return Ok(None) };
                crate::analyze::mesh_thumb_bytes(&mesh, &file)
            })?;
            std::fs::remove_dir_all(&dir).ok();
            match plain {
                Ok(Some(thumb)) => {
                    if replace_thumb(state, rt, conn, &id, thumb, "json_remove(meta, '$.thumb_dressed')")? {
                        done += 1;
                    }
                }
                Ok(None) => {}
                Err(e) => tracing::debug!(asset = %id, error = %format!("{e:#}"), "model thumbnail kept as it was"),
            }
        }
        conn.execute("UPDATE assets SET meta = json_remove(meta, '$.redress') WHERE id = ?1", [&id])?;
    }
    if done > 0 {
        tracing::info!(pack = pack_id, models = done, "model thumbnails redrawn with their materials");
    }
    Ok(done)
}

/// A model's thumbnail drawn with the material in each of its slots (`material_slots`, see [`link_unity_slots`]):
/// a slot with a color map of its own keeps it, the others take their material's color map and color. `None` when
/// that changes nothing.
fn dress_slots(
    state: &AppState,
    rt: &Handle,
    conn: &Connection,
    id: &str,
    ext: &str,
    slots: &str,
    dir: &Path,
) -> Result<Option<Vec<u8>>> {
    let slots: HashMap<String, String> = serde_json::from_str(slots)?;
    let file = fetch_model(state, rt, conn, id, dir)?;
    let (Some(mut mesh), _) = crate::meshes::load(&file, ext, dir)? else { return Ok(None) };
    let mut paint = conn.prepare_cached(
        "SELECT (SELECT t.thumb_hash FROM material_maps m JOIN assets t ON t.id = m.texture_id
                  WHERE m.material_id = a.id AND m.role = 'base_color' AND t.thumb_hash IS NOT NULL
                  ORDER BY COALESCE(json_extract(t.meta, '$.width'), 0) DESC LIMIT 1),
                json_extract(a.meta, '$.look.base_color')
         FROM assets a WHERE a.id = ?1",
    )?;
    let mut images: HashMap<String, usize> = HashMap::new();
    let mut changed = false;
    for m in &mut mesh.materials {
        let Some(mat_id) = slots.get(&m.name).filter(|_| m.color_image.is_none()) else { continue };
        let Some((map, tint)) = paint
            .query_row([mat_id], |r| Ok((r.get::<_, Option<String>>(0)?, r.get::<_, Option<String>>(1)?)))
            .optional()?
        else {
            continue;
        };
        let tint = tint.and_then(|t| serde_json::from_str::<[f32; 4]>(&t).ok());
        if map.is_none() && tint.is_none() {
            continue;
        }
        if let Some(hash) = map {
            if !images.contains_key(&hash) {
                let bytes = rt.block_on(state.blobs.read_all(&hash))?.to_vec();
                let mime = crate::analyze::thumb_mime(&bytes);
                mesh.images.push(crate::meshes::Image { bytes, mime });
                images.insert(hash.clone(), mesh.images.len() - 1);
            }
            m.color_image = images.get(&hash).copied();
        }
        m.color = tint.unwrap_or([1.0, 1.0, 1.0, m.color[3]]);
        changed = true;
    }
    if !changed {
        return Ok(None);
    }
    crate::analyze::mesh_thumb_bytes(&mesh, &file)
}

/// The names of the material slots of the pack's models in formats Unity imports and Hoard reads (FBX, OBJ, 3DS),
/// as `material_names` in their meta, for the ones not read yet: what Unity's materials are matched to. A model
/// that can't be read gets none.
pub(super) fn read_material_names(state: &AppState, rt: &Handle, conn: &Connection, pack_id: &str) -> Result<()> {
    let todo: Vec<(String, String, String)> = conn
        .prepare(
            "SELECT id, ext, blob_hash FROM assets WHERE pack_id = ?1 AND kind = 'model'
                AND ext IN ('fbx', 'obj', '3ds') AND json_type(meta, '$.material_names') IS NULL",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    if todo.is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(state.cfg.tmp_dir())?;
    let work = tempfile::Builder::new().prefix("slots-").tempdir_in(state.cfg.tmp_dir())?;
    for (id, ext, hash) in todo {
        let file = work.path().join(format!("model.{ext}"));
        let names = rt
            .block_on(state.blobs.download_to(&hash, &file))
            .and_then(|()| crate::meshes::material_names(&file, &ext))
            .unwrap_or_else(|e| {
                tracing::debug!(asset = %id, error = %format!("{e:#}"), "material names not read");
                Vec::new()
            });
        std::fs::remove_file(&file).ok();
        conn.execute(
            "UPDATE assets SET meta = json_set(meta, '$.material_names', json(?2)) WHERE id = ?1",
            params![id, serde_json::to_string(&names)?],
        )?;
    }
    Ok(())
}

/// A model's file with the files it needs (a glTF's buffers and images, an OBJ's MTL), where the pack has them,
/// downloaded into `dir`; the model's own file's path.
/// Run `f`, which draws model `id`, with the model turned its own way: upright when a conversion rule turned it so
/// as it wrote the file, otherwise as its pack's models are (see [`db::asset_facing`]).
fn turned_as<R>(conn: &Connection, id: &str, f: impl FnOnce() -> R) -> Result<R> {
    let upright: bool =
        conn.query_row("SELECT EXISTS (SELECT 1 FROM asset_origin WHERE asset_id = ?1 AND upright = 1)", [id], |r| {
            r.get(0)
        })?;
    Ok(if upright { crate::meshes::facing(hoard_core::Facing::default(), f) } else { f() })
}

pub(crate) fn fetch_model(state: &AppState, rt: &Handle, conn: &Connection, id: &str, dir: &Path) -> Result<PathBuf> {
    let files = db::bundle_files(conn, &db::dependency_closure(conn, id, false)?)?;
    let mut file = None;
    for f in &files {
        let Some(rel) = crate::ingest::safe_rel_path(&f.path) else { continue };
        let dest = dir.join(rel);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        rt.block_on(state.blobs.download_to(&f.hash, &dest))?;
        if f.id == id {
            file = Some(dest);
        }
    }
    file.context("the model's file is missing")
}

/// Whether a glTF or GLB file has glass (`KHR_materials_transmission`), from its JSON alone.
async fn uses_transmission(state: &AppState, hash: &str, ext: &str) -> Result<bool> {
    const NEEDLE: &[u8] = b"KHR_materials_transmission";
    const MOST: u64 = 64 << 20;
    let json = if ext == "glb" {
        // 12-byte header, then the JSON chunk's length and type.
        let head = state.blobs.head(hash, 20).await?;
        let Some(len) = head.get(12..16).filter(|_| head.starts_with(b"glTF")) else { return Ok(false) };
        let len = u64::from(u32::from_le_bytes(len.try_into()?));
        state.blobs.head(hash, 20 + len.min(MOST)).await?
    } else {
        state.blobs.head(hash, MOST).await?
    };
    Ok(json.windows(NEEDLE.len()).any(|w| w == NEEDLE))
}

/// glTF and GLB models drawn by the server before its renderer knew glass (marked `reglass` on upgrade), drawn again
/// when they have some. One a material dressed is left for [`dress_models`] to redraw.
pub(super) fn redraw_glass(state: &AppState, rt: &Handle, conn: &Connection, pack_id: &str) -> Result<usize> {
    let todo: Vec<(String, String, String, bool)> = conn
        .prepare(
            "SELECT id, ext, blob_hash, json_extract(meta, '$.thumb_dressed') IS NOT NULL FROM assets
              WHERE pack_id = ?1 AND kind = 'model' AND json_extract(meta, '$.reglass') IS NOT NULL",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<rusqlite::Result<_>>()?;
    if todo.is_empty() || crate::analyze::renderer().is_none() {
        return Ok(0);
    }
    std::fs::create_dir_all(state.cfg.tmp_dir())?;
    let work = tempfile::Builder::new().prefix("glass-").tempdir_in(state.cfg.tmp_dir())?;
    let mut done = 0;
    for (id, ext, blob, dressed) in todo {
        let dir = work.path().join(&id);
        let drawn = (|| -> Result<Option<Vec<u8>>> {
            if !rt.block_on(uses_transmission(state, &blob, &ext))? {
                return Ok(None);
            }
            if dressed {
                conn.execute("UPDATE assets SET meta = json_remove(meta, '$.thumb_dressed') WHERE id = ?1", [&id])?;
                return Ok(None);
            }
            let file = fetch_model(state, rt, conn, &id, &dir)?;
            turned_as(conn, &id, || crate::analyze::gltf_thumb(&file))?
        })();
        std::fs::remove_dir_all(&dir).ok();
        let redrawn = match drawn {
            Ok(Some(thumb)) => replace_thumb(state, rt, conn, &id, thumb, "json_remove(meta, '$.reglass')")?,
            Ok(None) => false,
            Err(e) => {
                tracing::debug!(asset = %id, error = %format!("{e:#}"), "model thumbnail kept without its glass");
                false
            }
        };
        if redrawn {
            done += 1;
        } else {
            conn.execute("UPDATE assets SET meta = json_remove(meta, '$.reglass') WHERE id = ?1", [&id])?;
        }
    }
    if done > 0 {
        tracing::info!(pack = pack_id, models = done, "model thumbnails redrawn with their glass see-through");
    }
    Ok(done)
}

/// STL files read before Hoard knew their facet colors (marked `recolor` on upgrade) drawn again, now in color.
/// Ones without colors keep the thumbnail they have.
pub(super) fn recolor_stls(state: &AppState, rt: &Handle, conn: &Connection, pack_id: &str) -> Result<usize> {
    let todo: Vec<(String, String)> = conn
        .prepare(
            "SELECT id, blob_hash FROM assets
              WHERE pack_id = ?1 AND kind = 'model' AND ext = 'stl' AND json_extract(meta, '$.recolor') IS NOT NULL",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    if todo.is_empty() || crate::analyze::renderer().is_none() {
        return Ok(0);
    }
    std::fs::create_dir_all(state.cfg.tmp_dir())?;
    let work = tempfile::Builder::new().prefix("recolor-").tempdir_in(state.cfg.tmp_dir())?;
    let file = work.path().join("model.stl");
    let mut done = 0;
    for (id, blob) in todo {
        let drawn = (|| -> Result<Option<Vec<u8>>> {
            rt.block_on(state.blobs.download_to(&blob, &file))?;
            match crate::meshes::load(&file, "stl", work.path())? {
                (Some(mesh), _) if mesh.materials.len() > 1 => crate::analyze::mesh_thumb_bytes(&mesh, &file),
                _ => Ok(None),
            }
        })();
        std::fs::remove_file(&file).ok();
        let redrawn = match drawn {
            Ok(Some(thumb)) => replace_thumb(state, rt, conn, &id, thumb, "json_remove(meta, '$.recolor')")?,
            Ok(None) => false,
            Err(e) => {
                tracing::debug!(asset = %id, error = %format!("{e:#}"), "STL thumbnail kept as it was");
                false
            }
        };
        if redrawn {
            done += 1;
        } else {
            conn.execute("UPDATE assets SET meta = json_remove(meta, '$.recolor') WHERE id = ?1", [&id])?;
        }
    }
    if done > 0 {
        tracing::info!(pack = pack_id, models = done, "STL thumbnails redrawn in their colors");
    }
    Ok(done)
}

/// Link the pack's models to the materials made for them: an OBJ to the swatches its `.mtl` defines, and by name,
/// `Barrel.fbx` or `vlkhcbxia_LOD0.fbx` in the material's folder (or above it) to the `Barrel` or `vlkhcbxia`
/// material. A model that links to a material's maps shows the material already; naming is for the models that name
/// no textures.
pub(super) fn link_models(tx: &Connection, pack_id: &str) -> Result<()> {
    tx.execute(
        &format!(
            "DELETE FROM asset_links WHERE rel = 'material'
               AND to_id IN (SELECT a.id FROM assets a WHERE a.pack_id = ?1 AND {IS_MATERIAL})"
        ),
        [pack_id],
    )?;
    tx.execute(
        &format!(
            "INSERT INTO asset_links (from_id, to_id, rel)
             SELECT o.from_id, a.id, 'material' FROM assets a
               JOIN asset_links s ON s.from_id = a.id AND s.rel = 'sidecar'
               JOIN assets f ON f.id = s.to_id AND f.ext = 'mtl'
               JOIN asset_links o ON o.to_id = f.id AND o.rel = 'material'
              WHERE a.pack_id = ?1 AND {IS_MATERIAL}
             ON CONFLICT DO NOTHING"
        ),
        [pack_id],
    )?;
    let slotted = link_unity_slots(tx, pack_id)?;
    // Swatches go only where a `.mtl` puts them: a color has no name to match by.
    let materials: Vec<(String, String, String, bool)> = {
        let mut stmt = tx.prepare(&format!(
            "SELECT a.id, a.path, (SELECT group_concat(m.texture_id) FROM material_maps m WHERE m.material_id = a.id),
                    EXISTS (SELECT 1 FROM asset_links s JOIN assets f ON f.id = s.to_id AND f.ext = 'mat'
                             WHERE s.from_id = a.id AND s.rel = 'sidecar')
             FROM assets a WHERE a.pack_id = ?1 AND {IS_MATERIAL}
               AND NOT EXISTS (SELECT 1 FROM asset_links s JOIN assets f ON f.id = s.to_id AND f.ext = 'mtl'
                                WHERE s.from_id = a.id AND s.rel = 'sidecar')"
        ))?;
        stmt.query_map([pack_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get::<_, Option<String>>(2)?.unwrap_or_default(), r.get(3)?))
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    if materials.is_empty() {
        return Ok(());
    }
    let models: Vec<(String, String)> = {
        let mut stmt = tx.prepare("SELECT id, path FROM assets WHERE pack_id = ?1 AND kind = 'model'")?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?
    };
    let mut links_to = tx.prepare_cached("SELECT 1 FROM asset_links WHERE from_id = ?1 AND to_id = ?2")?;
    let mut insert = tx.prepare_cached(
        "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'material') ON CONFLICT DO NOTHING",
    )?;
    for (model_id, model_path) in &models {
        if stem_of(model_path).contains('@') {
            continue;
        }
        let model_words = words(stem_of(model_path));
        let model_dir = dir_key(dir_of(model_path));
        for (mat_id, mat_path, maps, unity) in &materials {
            // A model whose slots wear Unity's materials wears no others of Unity's by its file name.
            if *unity && slotted.contains(model_id) {
                continue;
            }
            let key = words(stem_of(mat_path));
            let mat_dir = dir_key(dir_of(mat_path));
            let near = model_dir == mat_dir || mat_dir.starts_with(&format!("{model_dir}/")) || model_dir.is_empty();
            let named = !key.is_empty() && model_words.len() >= key.len() && model_words[..key.len()] == key[..];
            if !(near && named) {
                continue;
            }
            let uses_maps = maps.split(',').any(|t| links_to.exists(params![model_id, t]).unwrap_or(false));
            if !uses_maps {
                insert.execute(params![model_id, mat_id])?;
            }
        }
    }
    Ok(())
}

/// Unity puts a `.mat` in each of a model's material slots: the one its importer's `.meta` names for the slot
/// (`unity_remaps`), or else one named after the slot (or after the model and the slot, `Crate-Paint`) within the
/// model's reach (see [`slot_reach`]), the nearest to the model where several are. Each model wearing them is linked
/// to them, and `material_slots` in its meta says which slot wears which (by material id), for drawing it. Returns
/// the models that wear any.
fn link_unity_slots(tx: &Connection, pack_id: &str) -> Result<HashSet<String>> {
    let worn: HashMap<String, serde_json::Value> = tx
        .prepare(
            "SELECT id, json_extract(meta, '$.material_slots') FROM assets
              WHERE pack_id = ?1 AND kind = 'model' AND json_extract(meta, '$.material_slots') IS NOT NULL",
        )?
        .query_map([pack_id], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
        .filter_map(|r| r.map(|(id, s)| serde_json::from_str(&s).ok().map(|v| (id, v))).transpose())
        .collect::<rusqlite::Result<_>>()?;
    tx.execute(
        "UPDATE assets SET meta = json_remove(meta, '$.material_slots')
          WHERE pack_id = ?1 AND kind = 'model' AND json_extract(meta, '$.material_slots') IS NOT NULL",
        [pack_id],
    )?;
    let unity: Vec<(String, String)> = tx
        .prepare(&format!(
            "SELECT a.id, f.path FROM assets a
               JOIN asset_links s ON s.from_id = a.id AND s.rel = 'sidecar'
               JOIN assets f ON f.id = s.to_id AND f.ext = 'mat'
              WHERE a.pack_id = ?1 AND {IS_MATERIAL} ORDER BY f.path"
        ))?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let mut slotted = HashSet::new();
    // A thumbnail dressed in materials a model no longer wears is drawn again (see [`dress_models`]).
    let mut changed = tx.prepare_cached(
        "UPDATE assets SET meta = json_set(json_remove(meta, '$.thumb_dressed'), '$.redress', 1)
          WHERE id = ?1 AND thumb_hash IS json_extract(meta, '$.thumb_dressed')",
    )?;
    if unity.is_empty() {
        for id in worn.keys() {
            changed.execute([id])?;
        }
        return Ok(slotted);
    }
    let by_file: HashMap<&str, &str> = unity.iter().map(|(id, path)| (path.as_str(), id.as_str())).collect();
    let guids: HashMap<String, String> = tx
        .prepare("SELECT guid, path FROM unity_guids WHERE pack_id = ?1")?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let remaps: HashMap<(String, String), String> = tx
        .prepare("SELECT path, slot, guid FROM unity_remaps WHERE pack_id = ?1")?
        .query_map([pack_id], |r| Ok(((r.get(0)?, r.get(1)?), r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let unity_words: Vec<Vec<String>> = unity.iter().map(|(_, path)| words(stem_of(path))).collect();
    let models: Vec<(String, String, String)> = tx
        .prepare(
            "SELECT id, path, json_extract(meta, '$.material_names') FROM assets
              WHERE pack_id = ?1 AND kind = 'model' AND json_array_length(meta, '$.material_names') > 0",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let mut insert = tx.prepare_cached(
        "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'material') ON CONFLICT DO NOTHING",
    )?;
    let mut save =
        tx.prepare_cached("UPDATE assets SET meta = json_set(meta, '$.material_slots', json(?2)) WHERE id = ?1")?;
    for (model_id, model_path, names) in models {
        let names: Vec<String> = serde_json::from_str(&names).unwrap_or_default();
        let model_words = words(stem_of(&model_path));
        let model_dir: Vec<&str> = dir_of(&model_path).split('/').filter(|s| !s.is_empty()).collect();
        let reach = slot_reach(&model_dir);
        let mut slots = serde_json::Map::new();
        for name in names {
            let remapped = remaps
                .get(&(model_path.clone(), name.clone()))
                .and_then(|g| guids.get(g))
                .and_then(|p| by_file.get(p.as_str()).copied());
            let by_name = || {
                let key = words(&name);
                if key.is_empty() {
                    return None;
                }
                let both: Vec<String> = model_words.iter().chain(&key).cloned().collect();
                (0..unity.len())
                    .filter(|&i| unity_words[i] == key || unity_words[i] == both)
                    .filter(|&i| reach.is_empty() || within(dir_of(&unity[i].1), &reach))
                    // The nearest: the most folders shared with the model, then the first by path.
                    .max_by_key(|&i| {
                        let shared = dir_of(&unity[i].1)
                            .split('/')
                            .filter(|s| !s.is_empty())
                            .zip(&model_dir)
                            .take_while(|(a, b)| a.eq_ignore_ascii_case(b))
                            .count();
                        (shared, std::cmp::Reverse(i))
                    })
                    .map(|i| unity[i].0.as_str())
            };
            if let Some(mat_id) = remapped.or_else(by_name) {
                insert.execute(params![model_id, mat_id])?;
                slots.insert(name, mat_id.into());
            }
        }
        let slots = (!slots.is_empty()).then_some(serde_json::Value::Object(slots));
        if worn.contains_key(&model_id) && worn.get(&model_id) != slots.as_ref() {
            changed.execute([&model_id])?;
        }
        if let Some(slots) = slots {
            save.execute(params![model_id, slots.to_string()])?;
            slotted.insert(model_id);
        }
    }
    Ok(slotted)
}

/// Where a material named after a model's slot may come from. In a Unity project, the project (the folder holding
/// its `Assets`), as Unity looks for one no further; elsewhere, the model's own part of the pack, its folder's parent
/// and below. Either way a bundle's other kits are out of reach: "skin" in one character kit isn't another's.
fn slot_reach(model_dir: &[&str]) -> String {
    let end = match model_dir.iter().rposition(|s| s.eq_ignore_ascii_case("assets")) {
        Some(i) => i + 1,
        None => model_dir.len().saturating_sub(1),
    };
    model_dir[..end].join("/")
}

/// Whether `dir` is `root` or a folder inside it, ignoring case.
fn within(dir: &str, root: &str) -> bool {
    dir.get(..root.len()).is_some_and(|head| head.eq_ignore_ascii_case(root))
        && (dir.len() == root.len() || dir.as_bytes()[root.len()] == b'/')
}

/// The object a model is of: its folder and name without an LOD number, so `Rock_LOD0.fbx` and `Rock_LOD3.fbx` are
/// one rock.
fn object_key(path: &str) -> String {
    let mut words = words(stem_of(path));
    if words.len() > 2 && words[words.len() - 2] == "lod" && words[words.len() - 1].bytes().all(|b| b.is_ascii_digit())
    {
        words.truncate(words.len() - 2);
    }
    format!("{}/{}", dir_key(dir_of(path)), words.join(" "))
}

/// A material only one object uses (a model, or that model's LODs) is part of it, as its maps are part of the
/// material: it goes out of the grid ([`db::SUPPORTING`]), and the model shows it. `owned_by` in its meta names the
/// model, its most detailed LOD. A material several objects share, or none, stays in the grid as itself.
pub(super) fn mark_owned(tx: &Connection, pack_id: &str) -> Result<()> {
    let users: Vec<(String, String, String)> = {
        let mut stmt = tx.prepare(&format!(
            "SELECT a.id, o.id, o.path FROM assets a
               JOIN asset_links l ON l.to_id = a.id AND l.rel = 'material'
               JOIN assets o ON o.id = l.from_id AND o.kind = 'model'
              WHERE a.pack_id = ?1 AND {IS_MATERIAL}
             UNION
             SELECT a.id, o.id, o.path FROM assets a
               JOIN material_maps m ON m.material_id = a.id
               JOIN asset_links l ON l.to_id = m.texture_id
               JOIN assets o ON o.id = l.from_id AND o.kind = 'model'
              WHERE a.pack_id = ?1 AND {IS_MATERIAL}
             ORDER BY 3"
        ))?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    // Each material's objects, and the first model of each by path (LOD0 before LOD1).
    let mut objects: HashMap<&str, BTreeMap<String, &str>> = HashMap::new();
    for (material, model, path) in &users {
        objects.entry(material).or_default().entry(object_key(path)).or_insert(model);
    }
    tx.execute(
        &format!(
            "UPDATE assets AS a SET meta = json_remove(a.meta, '$.owned_by')
              WHERE a.pack_id = ?1 AND {IS_MATERIAL} AND json_extract(a.meta, '$.owned_by') IS NOT NULL"
        ),
        [pack_id],
    )?;
    let mut own = tx.prepare_cached("UPDATE assets SET meta = json_set(meta, '$.owned_by', ?2) WHERE id = ?1")?;
    for (material, by) in &objects {
        if let [(_, model)] = by.iter().collect::<Vec<_>>()[..] {
            own.execute(params![material, model])?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lods_are_one_object() {
        assert_eq!(object_key("Rocks/Rock_LOD0.fbx"), object_key("Rocks/Rock_LOD3.fbx"));
        assert_ne!(object_key("Rocks/Rock_LOD0.fbx"), object_key("Rocks/Cliff_LOD0.fbx"));
        assert_ne!(object_key("a/Rock.fbx"), object_key("b/Rock.fbx"));
        // Only a trailing LOD number goes.
        assert_ne!(object_key("Rock_LOD0_Base.fbx"), object_key("Rock_LOD0_Top.fbx"));
    }

    #[test]
    fn a_material_one_object_wears_is_part_of_it() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            r#"CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, path TEXT, ext TEXT, kind TEXT, meta TEXT);
               CREATE TABLE asset_links (from_id TEXT, to_id TEXT, rel TEXT);
               CREATE TABLE material_maps (material_id TEXT, texture_id TEXT);
               INSERT INTO assets VALUES
                 ('rockmat', 'p', 'Rock.material', 'material', 'material', '{"material_maps":1}'),
                 ('sharedmat', 'p', 'Moss.material', 'material', 'material', '{"material_maps":1}'),
                 ('lonemat', 'p', 'Sand.material', 'material', 'material', '{"material_maps":1,"owned_by":"gone"}'),
                 ('rock_tex', 'p', 'Rock_BaseColor.png', 'png', 'texture', '{}'),
                 ('lod1', 'p', 'Rock_LOD1.fbx', 'fbx', 'model', '{}'),
                 ('lod0', 'p', 'Rock_LOD0.fbx', 'fbx', 'model', '{}'),
                 ('cliff', 'p', 'Cliff.fbx', 'fbx', 'model', '{}');
               INSERT INTO material_maps VALUES ('rockmat', 'rock_tex');
               INSERT INTO asset_links VALUES
                 ('lod0', 'rock_tex', 'texture'), ('lod1', 'rockmat', 'material'),
                 ('lod0', 'sharedmat', 'material'), ('cliff', 'sharedmat', 'material');"#,
        )
        .unwrap();
        mark_owned(&c, "p").unwrap();
        let owner = |id: &str| -> Option<String> {
            c.query_row("SELECT json_extract(meta, '$.owned_by') FROM assets WHERE id = ?1", [id], |r| r.get(0))
                .unwrap()
        };
        assert_eq!(owner("rockmat").as_deref(), Some("lod0"), "the rock's LODs are one object; LOD0 owns it");
        assert_eq!(owner("sharedmat"), None, "two objects share it");
        assert_eq!(owner("lonemat"), None, "nothing uses it any more");
    }

    #[test]
    fn unity_materials_go_in_the_slots_they_are_named_for() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            r#"CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, path TEXT, ext TEXT, kind TEXT, meta TEXT);
               CREATE TABLE asset_links (from_id TEXT, to_id TEXT, rel TEXT, PRIMARY KEY (from_id, to_id, rel));
               CREATE TABLE unity_guids (pack_id TEXT, guid TEXT, path TEXT);
               CREATE TABLE unity_remaps (pack_id TEXT, path TEXT, slot TEXT, guid TEXT);
               INSERT INTO assets VALUES
                 ('paint', 'p', 'Kit/Materials/Paint.material', 'material', 'material', '{"material_maps":1}'),
                 ('paint.mat', 'p', 'Kit/Materials/Paint.mat', 'mat', 'other', '{}'),
                 ('far', 'p', 'Other/Paint.material', 'material', 'material', '{"material_maps":1}'),
                 ('far.mat', 'p', 'Other/Paint.mat', 'mat', 'other', '{}'),
                 ('brick', 'p', 'Kit/Materials/Wall_Brick.material', 'material', 'material', '{"material_maps":1}'),
                 ('brick.mat', 'p', 'Kit/Materials/Wall_Brick.mat', 'mat', 'other', '{}'),
                 ('wood', 'p', 'Kit/Materials/Barrel-Wood.material', 'material', 'material', '{"material_maps":0}'),
                 ('wood.mat', 'p', 'Kit/Materials/Barrel-Wood.mat', 'mat', 'other', '{}'),
                 ('crate', 'p', 'Kit/Models/Crate.fbx', 'fbx', 'model',
                  '{"material_names":["Paint","lambert2","Steel"],"material_slots":{"Old":"gone"}}'),
                 ('barrel', 'p', 'Kit/Models/Barrel.fbx', 'fbx', 'model', '{"material_names":["Wood"]}'),
                 ('plain', 'p', 'Kit/Models/Plain.fbx', 'fbx', 'model', '{"material_names":[]}');
               INSERT INTO asset_links VALUES
                 ('paint', 'paint.mat', 'sidecar'), ('far', 'far.mat', 'sidecar'), ('brick', 'brick.mat', 'sidecar'),
                 ('wood', 'wood.mat', 'sidecar');
               INSERT INTO unity_guids VALUES ('p', 'gw', 'Kit/Materials/Wall_Brick.mat');
               INSERT INTO unity_remaps VALUES ('p', 'Kit/Models/Crate.fbx', 'lambert2', 'gw');
               ALTER TABLE assets ADD COLUMN thumb_hash TEXT;"#,
        )
        .unwrap();
        let slotted = link_unity_slots(&c, "p").unwrap();
        assert_eq!(slotted, HashSet::from(["crate".to_owned(), "barrel".to_owned()]));
        let slots = |id: &str| -> Option<String> {
            c.query_row("SELECT json_extract(meta, '$.material_slots') FROM assets WHERE id = ?1", [id], |r| r.get(0))
                .unwrap()
        };
        assert_eq!(
            slots("crate").as_deref(),
            Some(r#"{"Paint":"paint","lambert2":"brick"}"#),
            "the nearest Paint, the .meta's material for lambert2, and nothing for Steel"
        );
        assert_eq!(slots("barrel").as_deref(), Some(r#"{"Wood":"wood"}"#), "named after the model and the slot");
        assert_eq!(slots("plain"), None);
        let worn: Vec<String> = c
            .prepare("SELECT from_id || '>' || to_id FROM asset_links WHERE rel = 'material' ORDER BY 1")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(worn, ["barrel>wood", "crate>brick", "crate>paint"]);
    }

    #[test]
    fn a_kit_doesnt_wear_another_kits_material_of_the_same_name() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            r#"CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, path TEXT, ext TEXT, kind TEXT, meta TEXT);
               CREATE TABLE asset_links (from_id TEXT, to_id TEXT, rel TEXT, PRIMARY KEY (from_id, to_id, rel));
               CREATE TABLE unity_guids (pack_id TEXT, guid TEXT, path TEXT);
               CREATE TABLE unity_remaps (pack_id TEXT, path TEXT, slot TEXT, guid TEXT);
               INSERT INTO assets VALUES
                 ('heroes', 'p', 'Heroes/Unity/Assets/Materials/skin.material', 'material', 'material',
                  '{"material_maps":1}'),
                 ('heroes.mat', 'p', 'Heroes/Unity/Assets/Materials/skin.mat', 'mat', 'other', '{}'),
                 ('zombies', 'p', 'Zombies/Unity/Assets/Materials/skin.material', 'material', 'material',
                  '{"material_maps":1}'),
                 ('zombies.mat', 'p', 'Zombies/Unity/Assets/Materials/skin.mat', 'mat', 'other', '{}'),
                 ('zombie', 'p', 'Zombies/Unity/Assets/Models/zombie.fbx', 'fbx', 'model',
                  '{"material_names":["skin"]}'),
                 ('loose', 'p', 'Zombies/Models/zombie.fbx', 'fbx', 'model', '{"material_names":["skin"]}'),
                 ('bundle', 'p', 'Bundle/Models/character.fbx', 'fbx', 'model', '{"material_names":["skin"]}');
               INSERT INTO asset_links VALUES
                 ('heroes', 'heroes.mat', 'sidecar'), ('zombies', 'zombies.mat', 'sidecar');
               ALTER TABLE assets ADD COLUMN thumb_hash TEXT;
               UPDATE assets SET thumb_hash = 'h', meta = json_set(meta, '$.thumb_dressed', 'h',
                   '$.material_slots',
                   json(CASE id WHEN 'loose' THEN '{"skin":"zombies"}' ELSE '{"skin":"heroes"}' END))
                WHERE id IN ('zombie', 'loose', 'bundle');"#,
        )
        .unwrap();
        link_unity_slots(&c, "p").unwrap();
        let slots = |id: &str| -> Option<String> {
            c.query_row("SELECT json_extract(meta, '$.material_slots') FROM assets WHERE id = ?1", [id], |r| r.get(0))
                .unwrap()
        };
        assert_eq!(slots("zombie").as_deref(), Some(r#"{"skin":"zombies"}"#), "its own project's, not the first");
        assert_eq!(slots("loose").as_deref(), Some(r#"{"skin":"zombies"}"#), "within its part of the pack");
        assert_eq!(slots("bundle"), None, "no kit of its own has a skin");
        let redress: Vec<String> = c
            .prepare(
                "SELECT id FROM assets
                  WHERE json_extract(meta, '$.redress') AND meta ->> 'thumb_dressed' IS NULL ORDER BY id",
            )
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(redress, ["bundle", "zombie"], "drawn again: their materials changed, the loose one's didn't");
    }
}
