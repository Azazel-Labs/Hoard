//! Materials an engine or exporter wrote down: Unity's `.mat` files, read through the GUIDs in their textures'
//! `.meta` files, and the colors in a Wavefront `.mtl`, which also get drawn as the `.mtl`'s thumbnail.

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use hoard_core::MaterialLook;
use rusqlite::Connection;
use tokio::runtime::Handle;

use super::names::{Found, Role, dir_of, material_name, resolution_label, stem_of, words};
use super::{THUMB_SIZE, Texture, replace_thumb, sphere};
use crate::AppState;

/// The pack's Unity materials (`.mat`), each with the textures it points at through their `.meta` GUIDs and the
/// look it gives them. A file that isn't a material Hoard can read is left as it is.
pub(super) fn unity_materials(
    state: &AppState,
    rt: &Handle,
    conn: &Connection,
    pack_id: &str,
    textures: &[Texture],
) -> Result<Vec<Found>> {
    let files: Vec<(String, String, String)> = conn
        .prepare("SELECT id, path, blob_hash FROM assets WHERE pack_id = ?1 AND ext = 'mat' ORDER BY path")?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    if files.is_empty() {
        return Ok(Vec::new());
    }
    let guids: HashMap<String, String> = conn
        .prepare("SELECT guid, path FROM unity_guids WHERE pack_id = ?1")?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let by_path: HashMap<&str, usize> = textures.iter().enumerate().map(|(i, t)| (t.path.as_str(), i)).collect();
    let mut out = Vec::new();
    for (id, path, hash) in files {
        let bytes = rt.block_on(state.blobs.read_all(&hash))?;
        let mat = match crate::unitymat::parse(&bytes) {
            Ok(m) => m,
            Err(e) => {
                tracing::debug!(file = %path, error = %format!("{e:#}"), "not a Unity material Hoard reads");
                continue;
            }
        };
        let mut maps: Vec<(usize, Role, Option<String>)> = mat
            .maps()
            .into_iter()
            .filter_map(|(role, guid)| {
                let &i = by_path.get(guids.get(guid)?.as_str())?;
                Some((i, role, textures[i].size_px.map(resolution_label)))
            })
            .collect();
        maps.sort_by_key(|(_, role, _)| *role);
        let smoothness = mat
            .smoothness_in()
            .and_then(|s| maps.iter().find(|(_, role, _)| *role == s.role))
            .and_then(|(i, ..)| mean_alpha(state, rt, &textures[*i]));
        let look = mat.look(smoothness);
        let (dir, stem) = (dir_of(&path), stem_of(&path));
        let base = Some(mat.name.trim()).filter(|n| !n.is_empty()).unwrap_or(stem);
        let key = words(base).join("_");
        out.push(Found {
            path: if dir.is_empty() { format!("{stem}.material") } else { format!("{dir}/{stem}.material") },
            name: material_name(base, dir),
            key,
            maps,
            look: Some(look).filter(|l| *l != MaterialLook::default()),
            sources: vec![(id, path.clone())],
        });
    }
    Ok(out)
}

/// The pack's Wavefront materials with a color and no maps, as swatches. The same name and color in several `.mtl`
/// files of one folder is one material with each of them as a source; the same name in another color is another,
/// its file named after the `.mtl` too.
pub(super) fn mtl_materials(state: &AppState, rt: &Handle, conn: &Connection, pack_id: &str) -> Result<Vec<Found>> {
    let files: Vec<(String, String, String)> = conn
        .prepare("SELECT id, path, blob_hash FROM assets WHERE pack_id = ?1 AND ext = 'mtl' ORDER BY path")?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let mut out: Vec<Found> = Vec::new();
    let mut by_name: HashMap<(String, String), Vec<usize>> = HashMap::new();
    for (id, path, hash) in files {
        let bytes = rt.block_on(state.blobs.read_all(&hash))?;
        let dir = dir_of(&path);
        for m in crate::mtlmat::parse(&String::from_utf8_lossy(&bytes)) {
            if !m.maps.is_empty() {
                continue;
            }
            let base = Some(m.name.trim().replace(['/', '\\'], "_")).filter(|n| !n.is_empty());
            let base = base.unwrap_or_else(|| stem_of(&path).to_owned());
            let look = Some(m.look);
            let same = by_name.entry((dir.to_lowercase(), base.to_lowercase())).or_default();
            if let Some(&i) = same.iter().find(|&&i| out[i].look == look) {
                if !out[i].sources.iter().any(|(s, _)| *s == id) {
                    out[i].sources.push((id.clone(), path.clone()));
                }
                continue;
            }
            let file = if same.is_empty() { base.clone() } else { format!("{base} ({})", stem_of(&path)) };
            same.push(out.len());
            out.push(Found {
                path: if dir.is_empty() { format!("{file}.material") } else { format!("{dir}/{file}.material") },
                name: material_name(&base, dir),
                key: words(&base).join("_"),
                maps: Vec::new(),
                look,
                sources: vec![(id.clone(), path.clone())],
            });
        }
    }
    Ok(out)
}

/// Each `.mtl` without a thumbnail drawn as its materials side by side (up to nine), with the maps it links.
pub(super) fn draw_mtls(
    state: &AppState,
    rt: &Handle,
    conn: &Connection,
    pack_id: &str,
    textures: &[Texture],
) -> Result<()> {
    let files: Vec<(String, String, String)> = conn
        .prepare(
            "SELECT id, path, blob_hash FROM assets WHERE pack_id = ?1 AND ext = 'mtl' AND thumb_hash IS NULL
             ORDER BY path",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let by_id: HashMap<&str, usize> = textures.iter().enumerate().map(|(i, t)| (t.id.as_str(), i)).collect();
    let mut linked = conn.prepare_cached("SELECT to_id FROM asset_links WHERE from_id = ?1 AND rel = 'texture'")?;
    for (id, path, hash) in files {
        let bytes = rt.block_on(state.blobs.read_all(&hash))?;
        let defs = crate::mtlmat::parse(&String::from_utf8_lossy(&bytes));
        if defs.is_empty() {
            continue;
        }
        let maps: Vec<usize> = linked
            .query_map([&id], |r| r.get::<_, String>(0))?
            .filter_map(|t| by_id.get(t.ok()?.as_str()).copied())
            .collect();
        let file_of = |p: &str| p.rsplit(['/', '\\']).next().unwrap_or(p).to_lowercase();
        let found: Vec<Found> = defs
            .into_iter()
            .take(9)
            .map(|m| Found {
                path: String::new(),
                name: m.name,
                key: String::new(),
                maps: m
                    .maps
                    .iter()
                    .filter_map(|(role, file)| {
                        let i = *maps.iter().find(|&&i| file_of(&textures[i].path) == file_of(file))?;
                        Some((i, *role, None))
                    })
                    .collect(),
                look: Some(m.look),
                sources: Vec::new(),
            })
            .collect();
        let cols: u32 = match found.len() {
            1 => 1,
            2..=4 => 2,
            _ => 3,
        };
        let cell = THUMB_SIZE / cols;
        let rows = found.len().div_ceil(cols as usize) as u32;
        let mut sheet = image::RgbaImage::new(THUMB_SIZE, THUMB_SIZE);
        let top = (THUMB_SIZE - rows * cell) / 2;
        for (n, f) in found.iter().enumerate() {
            let Some(img) = sphere(state, rt, textures, f, cell)? else { continue };
            let (c, r) = (n as u32 % cols, n as u32 / cols);
            // A last row that's short is centered.
            let in_row = if r + 1 == rows { found.len() as u32 - r * cols } else { cols };
            let left = (THUMB_SIZE - in_row * cell) / 2;
            image::imageops::overlay(&mut sheet, &img, i64::from(left + c * cell), i64::from(top + r * cell));
        }
        let png = crate::analyze::png_bytes(sheet)?;
        if let Err(e) = replace_thumb(state, rt, conn, &id, png, "meta") {
            tracing::debug!(file = %path, error = %format!("{e:#}"), "could not draw an MTL file");
        }
    }
    Ok(())
}

/// How opaque a texture is on average, from its thumbnail: where Unity keeps a material's smoothness.
fn mean_alpha(state: &AppState, rt: &Handle, t: &Texture) -> Option<f32> {
    let bytes = rt.block_on(state.blobs.read_all(t.thumb.as_ref()?)).ok()?;
    let img = image::load_from_memory(&bytes).ok()?.to_rgba8();
    let n = (img.width() * img.height()).max(1);
    Some(img.pixels().map(|p| f32::from(p[3])).sum::<f32>() / 255.0 / n as f32)
}

/// Put the Unity materials first, in place of any set found by name that shares a map or a path with one; then the
/// Wavefront swatches whose paths are still free.
pub(super) fn prefer_engines(found: Vec<Found>, unity: Vec<Found>, mtl: Vec<Found>) -> Vec<Found> {
    let paths: HashSet<String> = unity.iter().map(|f| f.path.to_lowercase()).collect();
    let used: HashSet<usize> = unity.iter().flat_map(|f| f.maps.iter().map(|(i, ..)| *i)).collect();
    let mut seen = HashSet::new();
    let mut out: Vec<Found> = unity.into_iter().filter(|f| seen.insert(f.path.to_lowercase())).collect();
    out.extend(
        found
            .into_iter()
            .filter(|f| !paths.contains(&f.path.to_lowercase()) && !f.maps.iter().any(|(i, ..)| used.contains(i))),
    );
    let taken: HashSet<String> = out.iter().map(|f| f.path.to_lowercase()).collect();
    out.extend(
        mtl.into_iter().filter(|f| !taken.contains(&f.path.to_lowercase()) && seen.insert(f.path.to_lowercase())),
    );
    out
}
