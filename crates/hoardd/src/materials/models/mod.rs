//! The models that wear a pack's materials: linked to them by what their files name, put in the slots Unity's
//! `.meta` files say, their thumbnails drawn again wearing them, and a material only one object wears made part of it.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use rusqlite::{Connection, OptionalExtension, params};
use tokio::runtime::Handle;

use super::replace_thumb;
use crate::{AppState, db};

mod linking;

pub(super) use linking::{link_models, mark_owned};

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

#[cfg(test)]
mod tests;
