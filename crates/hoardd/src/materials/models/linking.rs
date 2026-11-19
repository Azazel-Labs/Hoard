//! Which models wear which materials: by what their files name, the slots Unity's .meta files give them, and a material only one object wears made part of it.

use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::Result;
use rusqlite::{Connection, params};

use crate::materials::IS_MATERIAL;
use crate::materials::names::{dir_key, dir_of, stem_of, words};

/// Link the pack's models to the materials made for them: an OBJ to the swatches its `.mtl` defines, and by name,
/// `Barrel.fbx` or `vlkhcbxia_LOD0.fbx` in the material's folder (or above it) to the `Barrel` or `vlkhcbxia`
/// material. A model that links to a material's maps shows the material already; naming is for the models that name
/// no textures.
pub(in crate::materials) fn link_models(tx: &Connection, pack_id: &str) -> Result<()> {
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
pub(super) fn link_unity_slots(tx: &Connection, pack_id: &str) -> Result<HashSet<String>> {
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
pub(super) fn object_key(path: &str) -> String {
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
pub(in crate::materials) fn mark_owned(tx: &Connection, pack_id: &str) -> Result<()> {
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
