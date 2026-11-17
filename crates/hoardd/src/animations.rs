//! Animations as their own thing. Packs ship a character's clips in files of their own: an FBX with the skeleton
//! and no mesh (`run.fbx`), or a `.blend` with the whole character posed for one clip (`run.blend`). Each is an
//! animation, not a model, and plays on the characters whose skeleton it moves.
//!
//! What a model's skeleton and clips are is read once, during analysis (a [`Rig`], kept in `rigs`); a `.blend`'s is
//! read from the GLB Blender made of it. [`sort_pack`] then works out, from the rigs, which files are clips and
//! which characters each one fits: those whose bones it moves, nearest it in the pack.

use std::collections::{BTreeSet, HashMap};

use anyhow::Result;
use hoard_core::{AssetKind, LinkRel, TagSource};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{AppState, db};

/// A take shorter than this is a pose (Blender's "Targeting Pose", a frame or two long), not a clip.
const POSE_SECS: f64 = 0.1;

/// A model's skeleton and clips.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Rig {
    /// The names of its nodes as [`bone`] has them, sorted, each once.
    pub(crate) bones: Vec<String>,
    /// How many of its nodes draw a mesh.
    pub(crate) meshes: u32,
    /// Whether a mesh is skinned to bones.
    pub(crate) skinned: bool,
    pub(crate) clips: Vec<Clip>,
}

/// One animation in a model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Clip {
    pub(crate) name: String,
    pub(crate) secs: f64,
    /// The bones it moves, as [`bone`] has them, sorted.
    pub(crate) bones: Vec<String>,
}

impl Clip {
    fn is_pose(&self) -> bool {
        self.secs < POSE_SECS
    }
}

impl Rig {
    /// Its clips that aren't poses.
    fn takes(&self) -> impl Iterator<Item = &Clip> {
        self.clips.iter().filter(|c| !c.is_pose())
    }
}

/// A node's name as clips are matched by it: without a namespace or path (`mixamorig:Hips` and `Armature|Hips` are
/// `hips`), lowercase. Leaf markers (`Head_end`) and an importer's wrapper (`RootNode`) aren't bones.
fn bone(name: &str) -> Option<String> {
    let name = name.rsplit([':', '|']).next().unwrap_or(name).trim().to_lowercase();
    (!name.is_empty() && !name.ends_with("_end") && name != "rootnode").then_some(name)
}

/// A clip's name without the take path an exporter puts in front (`Root|Run` is `Run`).
fn clip_name(name: &str) -> &str {
    name.rsplit('|').next().unwrap_or(name).trim()
}

/// The rig of a model read as glTF: `None` for one with neither skins nor animations.
pub(crate) fn rig(gltf: &Value) -> Option<Rig> {
    let list = |v: &Value, k: &str| v.get(k).and_then(Value::as_array).cloned().unwrap_or_default();
    let (nodes, skins, animations, accessors) =
        (list(gltf, "nodes"), list(gltf, "skins"), list(gltf, "animations"), list(gltf, "accessors"));
    if skins.is_empty() && animations.is_empty() {
        return None;
    }
    let node_bone =
        |i: &Value| -> Option<String> { bone(nodes.get(usize::try_from(i.as_u64()?).ok()?)?.get("name")?.as_str()?) };
    let bones: BTreeSet<String> =
        nodes.iter().filter_map(|n| n.get("name").and_then(Value::as_str)).filter_map(bone).collect();
    let meshes = nodes.iter().filter(|n| n.get("mesh").is_some()).count();
    let skinned = !skins.is_empty() && nodes.iter().any(|n| n.get("mesh").is_some() && n.get("skin").is_some());
    let clips = animations
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let secs = list(a, "samplers")
                .iter()
                .filter_map(|s| accessors.get(usize::try_from(s.get("input")?.as_u64()?).ok()?)?.get("max")?.get(0))
                .filter_map(Value::as_f64)
                .fold(0.0, f64::max);
            let moved: BTreeSet<String> =
                list(a, "channels").iter().filter_map(|c| node_bone(c.get("target")?.get("node")?)).collect();
            let name = a.get("name").and_then(Value::as_str).map_or_default(clip_name);
            Clip {
                name: if name.is_empty() { format!("Clip {}", i + 1) } else { name.to_owned() },
                secs: (secs * 1000.0).round() / 1000.0,
                bones: moved.into_iter().collect(),
            }
        })
        .collect();
    Some(Rig { bones: bones.into_iter().collect(), meshes: u32::try_from(meshes).unwrap_or(u32::MAX), skinned, clips })
}

/// The rig in a GLB's bytes.
pub(crate) fn rig_of_glb(glb: &[u8]) -> Option<Rig> {
    rig(&crate::formats::read_glb(glb).ok()?.0)
}

/// Keep an asset's rig, or forget it.
pub(crate) fn store(conn: &Connection, asset_id: &str, rig: Option<&Rig>) -> Result<()> {
    match rig {
        Some(r) => conn.execute(
            "INSERT INTO rigs (asset_id, rig) VALUES (?1, ?2) ON CONFLICT (asset_id) DO UPDATE SET rig = excluded.rig",
            params![asset_id, serde_json::to_string(r)?],
        )?,
        None => conn.execute("DELETE FROM rigs WHERE asset_id = ?1", [asset_id])?,
    };
    Ok(())
}

/// Whether `moved` (a clip's bones) plays on a skeleton with `bones`: nearly all of them are its bones, and at least
/// three. A clip may also move the object its exporter wrapped the skeleton in, which a character needn't share.
fn fits(moved: &BTreeSet<&str>, bones: &BTreeSet<&str>) -> bool {
    let shared = moved.iter().filter(|b| bones.contains(*b)).count();
    shared >= 3 && shared * 10 >= moved.len() * 9
}

/// A name as clip and file names are compared: letters and digits, lowercase (`Run`, `run`, `crouch-idle`).
fn plain(s: &str) -> String {
    s.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect()
}

fn file_stem(path: &str) -> &str {
    let file = path.rsplit('/').next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

/// How many folders two paths share from the top.
fn shared_folders(a: &str, b: &str) -> usize {
    let dirs = |p: &str| p.rsplit_once('/').map_or(Vec::new(), |(d, _)| d.split('/').map(str::to_owned).collect());
    dirs(a).iter().zip(dirs(b).iter()).take_while(|(x, y)| x == y).count()
}

struct Entry {
    id: String,
    path: String,
    kind: AssetKind,
    rig: Option<Rig>,
}

/// Sort a pack's models into characters and animations, from their rigs and names, and link each animation to the
/// characters it plays on (`character → clip`, [`LinkRel::Animation`]):
///
/// - A file with no mesh and a clip is an animation (`run.fbx`).
/// - So is a skinned character with one clip that's named for the file (`run.blend` holding "Run"), or that a
///   character with no clips of its own in the pack is rigged for: a clip shipped with a copy of the character.
/// - `Knight@Run.fbx` is a clip for `Knight.fbx`, as Unity and Mixamo name them.
///
/// An animation plays on the characters it fits that are nearest it in the pack (sharing the most folders), so
/// each character set's clips go to its own characters when a bundle holds several, except one that has a nearer
/// copy of it, such as a Unity package's. Run again whenever the pack's models or rigs change; it puts back
/// whatever an earlier run decided.
pub(crate) fn sort_pack(conn: &Connection, pack_id: &str) -> Result<()> {
    let entries: Vec<Entry> = {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.path, a.ext, a.kind, r.rig FROM assets a LEFT JOIN rigs r ON r.asset_id = a.id
              WHERE a.pack_id = ?1 AND a.kind IN ('model', 'animation')",
        )?;
        let rows = stmt.query_map([pack_id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, String>(3)?, {
                r.get::<_, Option<String>>(4)?
            }))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (id, path, ext, kind, rig) = row?;
            // Clips of their own kind (Unity `.anim`, BVH) stay as they are.
            if AssetKind::from_ext(&ext) != AssetKind::Model {
                continue;
            }
            let rig = rig.and_then(|r| serde_json::from_str(&r).ok());
            out.push(Entry { id, path, kind: AssetKind::parse(&kind).unwrap_or(AssetKind::Model), rig });
        }
        out
    };

    let bones: Vec<BTreeSet<&str>> = entries
        .iter()
        .map(|e| e.rig.as_ref().map_or_default(|r| r.bones.iter().map(String::as_str).collect()))
        .collect();
    // The bones each file's clips move, all of them together.
    let moved: Vec<BTreeSet<&str>> = entries
        .iter()
        .map(|e| {
            e.rig.as_ref().map_or_default(|r| r.takes().flat_map(|c| c.bones.iter().map(String::as_str)).collect())
        })
        .collect();
    let skinned = |i: usize| entries[i].rig.as_ref().is_some_and(|r| r.meshes > 0 && r.skinned);
    let bare: Vec<usize> = (0..entries.len())
        .filter(|&i| skinned(i) && entries[i].rig.as_ref().is_some_and(|r| r.takes().next().is_none()))
        .collect();

    let is_clip: Vec<bool> = (0..entries.len())
        .map(|i| {
            let Some(rig) = &entries[i].rig else { return false };
            let takes: Vec<&Clip> = rig.takes().collect();
            if takes.is_empty() {
                return false;
            }
            if rig.meshes == 0 {
                return true;
            }
            let stem = file_stem(&entries[i].path);
            skinned(i)
                && takes.len() == 1
                && (stem.contains('@')
                    || plain(&takes[0].name) == plain(stem)
                    || bare.iter().any(|&c| c != i && fits(&moved[i], &bones[c])))
        })
        .collect();

    let mut links: BTreeSet<(usize, usize)> = BTreeSet::new();
    let characters: Vec<usize> = (0..entries.len()).filter(|&i| skinned(i) && !is_clip[i]).collect();
    // A clip goes to the characters it fits that share the most folders with it, unless a character has a nearer
    // copy of it (same name): the clips beside a Unity package's copy of a character go to that copy, and the set's
    // own clips to the original.
    let fitting: Vec<(usize, usize, usize)> = (0..entries.len())
        .filter(|&i| is_clip[i])
        .flat_map(|clip| characters.iter().map(move |&c| (c, clip)))
        .filter(|&(c, clip)| fits(&moved[clip], &bones[c]))
        .map(|(c, clip)| (c, clip, shared_folders(&entries[c].path, &entries[clip].path)))
        .collect();
    let name = |i: usize| file_stem(&entries[i].path).to_lowercase();
    let (mut for_clip, mut for_copy) = (HashMap::new(), HashMap::new());
    for &(c, clip, near) in &fitting {
        let n: &mut usize = for_clip.entry(clip).or_default();
        *n = (*n).max(near);
        let n: &mut usize = for_copy.entry((c, name(clip))).or_default();
        *n = (*n).max(near);
    }
    links.extend(
        fitting
            .iter()
            .filter(|&&(c, clip, near)| for_clip[&clip] == near && for_copy[&(c, name(clip))] == near)
            .map(|&(c, clip, _)| (c, clip)),
    );
    // "Knight@Run.fbx" is a clip for "Knight.fbx" (or any model format with that stem), rigged or not.
    let by_stem: HashMap<String, Vec<usize>> = entries.iter().enumerate().fold(HashMap::new(), |mut m, (i, e)| {
        m.entry(file_stem(&e.path).to_lowercase()).or_default().push(i);
        m
    });
    for (clip, e) in entries.iter().enumerate() {
        let Some((model, _)) = file_stem(&e.path).split_once('@') else { continue };
        for &m in by_stem.get(&model.to_lowercase()).into_iter().flatten() {
            links.insert((m, clip));
        }
    }

    // The links an earlier run made between these models are replaced; others (a Unity controller's clips) stay.
    let index: HashMap<&str, usize> = entries.iter().enumerate().map(|(i, e)| (e.id.as_str(), i)).collect();
    let before: Vec<(String, String)> = conn
        .prepare(
            "SELECT l.from_id, l.to_id FROM asset_links l JOIN assets a ON a.id = l.from_id
              WHERE a.pack_id = ?1 AND l.rel = 'animation'",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let mut drop = conn.prepare("DELETE FROM asset_links WHERE from_id = ?1 AND to_id = ?2 AND rel = 'animation'")?;
    for (from, to) in &before {
        if let (Some(&f), Some(&t)) = (index.get(from.as_str()), index.get(to.as_str()))
            && !links.contains(&(f, t))
        {
            drop.execute(params![from, to])?;
        }
    }
    let mut add =
        conn.prepare("INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING")?;
    for (from, to) in &links {
        add.execute(params![entries[*from].id, entries[*to].id, LinkRel::Animation.as_str()])?;
    }

    for (i, e) in entries.iter().enumerate() {
        let kind = if is_clip[i] { AssetKind::Animation } else { AssetKind::Model };
        if kind != e.kind {
            set_kind(conn, &e.id, kind)?;
        }
    }
    Ok(())
}

/// Turn a model into an animation or back: its kind, its `kind:` tag and, unless someone chose one, its category.
fn set_kind(conn: &Connection, id: &str, kind: AssetKind) -> Result<()> {
    let category = match kind {
        AssetKind::Animation => "animations",
        _ => "models",
    };
    conn.execute(
        "UPDATE assets SET kind = ?2, category = CASE WHEN human_edited THEN category ELSE ?3 END WHERE id = ?1",
        params![id, kind.as_str(), category],
    )?;
    conn.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag LIKE 'kind:%' AND source = 'meta'", [id])?;
    db::upsert_tag(conn, id, &format!("kind:{}", kind.as_str()), TagSource::Meta, 1.0)?;
    db::reindex_asset(conn, id)?;
    Ok(())
}

/// Rigs for a pack's converted models (a `.blend`), read from the GLB Blender made of each, for those that have
/// none yet; then the pack is sorted again if any were found. Runs on a blocking thread.
pub(crate) fn rigs_from_conversions(state: &AppState, rt: &tokio::runtime::Handle, pack_id: &str) -> Result<()> {
    let todo: Vec<(String, String)> = state
        .pool
        .get()?
        .prepare(
            "SELECT a.id, d.blob_hash FROM assets a JOIN derivatives d ON d.asset_id = a.id AND d.kind = 'glb'
             WHERE a.pack_id = ?1 AND NOT EXISTS (SELECT 1 FROM rigs r WHERE r.asset_id = a.id)",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let mut found = false;
    for (id, glb) in todo {
        let Some(rig) = rig_of_glb(&rt.block_on(state.blobs.read_all(&glb))?) else { continue };
        store(&*state.pool.get()?, &id, Some(&rig))?;
        found = true;
    }
    if found {
        let mut conn = state.pool.get()?;
        let tx = conn.transaction()?;
        sort_pack(&tx, pack_id)?;
        tx.commit()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A glTF with the named nodes, `mesh` on the ones given, and clips moving the named nodes for so long.
    fn gltf(nodes: &[&str], mesh: &[&str], skinned: bool, clips: &[(&str, f64, &[&str])]) -> Value {
        let at = |n: &str| nodes.iter().position(|x| *x == n).unwrap();
        let mut accessors = Vec::new();
        let animations: Vec<Value> = clips
            .iter()
            .map(|(name, secs, moved)| {
                accessors.push(json!({ "count": 2, "max": [secs], "min": [0.0] }));
                let input = accessors.len() - 1;
                json!({
                    "name": name,
                    "samplers": [{ "input": input, "output": input }],
                    "channels": moved.iter().map(|n| json!({ "sampler": 0, "target": { "node": at(n), "path": "rotation" } })).collect::<Vec<_>>(),
                })
            })
            .collect();
        json!({
            "nodes": nodes.iter().map(|n| {
                let mut v = json!({ "name": n });
                if mesh.contains(n) {
                    v["mesh"] = json!(0);
                    if skinned { v["skin"] = json!(0); }
                }
                v
            }).collect::<Vec<_>>(),
            "skins": if skinned { json!([{ "joints": [] }]) } else { json!([]) },
            "animations": animations,
            "accessors": accessors,
        })
    }

    const SKELETON: &[&str] = &["Root", "Hips", "Spine", "Head", "Head_end", "LeftArm", "RightArm"];
    const MOVED: &[&str] = &["Hips", "Spine", "Head", "LeftArm", "RightArm"];

    fn with_mesh(extra: &str) -> Vec<&str> {
        let mut v = SKELETON.to_vec();
        v.push(extra);
        v
    }

    #[test]
    fn a_rig_names_bones_as_clips_do_and_times_its_clips() {
        let mut doc =
            gltf(SKELETON, &[], false, &[("Root|0.Targeting Pose", 0.04, MOVED), ("Root|Run", 0.6667, MOVED)]);
        doc["nodes"][1]["name"] = json!("mixamorig:Hips");
        let r = rig(&doc).unwrap();
        assert_eq!(r.bones, ["head", "hips", "leftarm", "rightarm", "root", "spine"]);
        assert_eq!((r.meshes, r.skinned), (0, false));
        assert_eq!(r.clips[1].name, "Run");
        assert!((r.clips[1].secs - 0.667).abs() < 1e-9);
        assert_eq!(r.clips[1].bones, ["head", "hips", "leftarm", "rightarm", "spine"]);
        assert_eq!(r.takes().count(), 1, "the targeting pose is a pose");
        assert!(rig(&json!({ "nodes": [{ "name": "Crate", "mesh": 0 }] })).is_none());
    }

    fn pool() -> (tempfile::TempDir, db::Pool) {
        let dir = tempfile::tempdir().unwrap();
        let pool = db::open(&dir.path().join("hoard.db"), dir.path()).unwrap();
        (dir, pool)
    }

    fn add(c: &Connection, id: &str, path: &str, rig_doc: Option<Value>) {
        let ext = path.rsplit_once('.').unwrap().1;
        c.execute(
            "INSERT INTO blobs (hash, size, created_at) VALUES (?1, 1, 'x') ON CONFLICT DO NOTHING",
            [format!("h-{id}")],
        )
        .unwrap();
        c.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, category, created_at, updated_at)
             VALUES (?1, 'p', ?2, ?3, ?1, ?4, 'model', 'models', 'x', 'x')",
            params![id, format!("h-{id}"), path, ext],
        )
        .unwrap();
        db::upsert_tag(c, id, "kind:model", TagSource::Meta, 1.0).unwrap();
        if let Some(doc) = rig_doc {
            store(c, id, rig(&doc).as_ref()).unwrap();
        }
    }

    #[test]
    fn clips_become_animations_of_the_nearest_characters_they_fit() {
        let (_dir, pool) = pool();
        let c = pool.get().unwrap();
        c.execute("INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'Pack', 'ready', 'x')", []).unwrap();
        let character = gltf(&with_mesh("Body"), &["Body"], true, &[]);
        add(&c, "medium", "Bundle/Models/characterMedium.fbx", Some(character.clone()));
        add(&c, "small", "Bundle/Models/characterSmall.fbx", Some(character.clone()));
        add(&c, "other_set", "Retro/Model/characterMedium.fbx", Some(character.clone()));
        // The set again inside a Unity package, each with its own copy of a clip.
        add(&c, "unity_set", "Retro/Unity package/Assets/Models/characterMedium.fbx", Some(character));
        let idle = gltf(SKELETON, &[], false, &[("Root|Idle", 1.5, MOVED)]);
        add(&c, "retro_idle", "Retro/Animations/idle.fbx", Some(idle.clone()));
        add(&c, "unity_idle", "Retro/Unity package/Assets/Animations/idle.fbx", Some(idle));
        // A skeleton with no mesh, and a copy of the character posed for one clip, beside a pose take.
        add(&c, "run_fbx", "Bundle/Animations/run.fbx", Some(gltf(SKELETON, &[], false, &[("Root|Run", 0.67, MOVED)])));
        let posed = gltf(
            &with_mesh("Character"),
            &["Character"],
            true,
            &[("0.Targeting Pose", 0.0, MOVED), ("Run", 0.67, MOVED)],
        );
        add(&c, "run_blend", "Bundle/Animations/Source/run.blend", Some(posed));
        let crouch = gltf(&with_mesh("Character"), &["Character"], true, &[("Action", 1.2, MOVED)]);
        add(&c, "crouch_blend", "Bundle/Animations/Source/crouch.blend", Some(crouch));
        // A character with clips of its own, a door that swings, and a clip with a skeleton of its own.
        let hero = gltf(&with_mesh("Body"), &["Body"], true, &[("Idle", 2.0, MOVED), ("Walk", 1.0, MOVED)]);
        add(&c, "hero", "Bundle/Models/hero.glb", Some(hero));
        add(
            &c,
            "door",
            "Kit/door.glb",
            Some(gltf(&["Frame", "Door"], &["Frame", "Door"], false, &[("door", 1.0, &["Door"])])),
        );
        let tail = &["Tail1", "Tail2", "Tail3"];
        add(&c, "wag", "Bundle/Animations/wag.fbx", Some(gltf(tail, &[], false, &[("Wag", 1.0, tail)])));
        add(&c, "knight", "Mixamo/Knight.fbx", None);
        add(&c, "knight_run", "Mixamo/Knight@Run.fbx", None);
        add(&c, "static", "Kit/crate.fbx", None);
        sort_pack(&c, "p").unwrap();
        sort_pack(&c, "p").unwrap();

        let kind = |id: &str| -> (String, String, String) {
            c.query_row(
                "SELECT kind, category, (SELECT group_concat(tag) FROM asset_tags t WHERE t.asset_id = a.id
                                          AND t.tag LIKE 'kind:%') FROM assets a WHERE id = ?1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap()
        };
        for id in ["run_fbx", "run_blend", "crouch_blend", "wag", "retro_idle", "unity_idle"] {
            assert_eq!(kind(id), ("animation".into(), "animations".into(), "kind:animation".into()), "{id}");
        }
        for id in ["medium", "small", "other_set", "unity_set", "hero", "door", "knight", "knight_run", "static"] {
            assert_eq!(kind(id).0, "model", "{id}");
        }
        let links: Vec<(String, String)> = c
            .prepare("SELECT from_id, to_id FROM asset_links WHERE rel = 'animation' ORDER BY to_id, from_id")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        let pairs: Vec<(&str, &str)> = links.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
        assert_eq!(
            pairs,
            [
                ("hero", "crouch_blend"),
                ("medium", "crouch_blend"),
                ("small", "crouch_blend"),
                ("knight", "knight_run"),
                ("other_set", "retro_idle"),
                ("hero", "run_blend"),
                ("medium", "run_blend"),
                ("small", "run_blend"),
                ("hero", "run_fbx"),
                ("medium", "run_fbx"),
                ("small", "run_fbx"),
                ("unity_set", "unity_idle"),
            ]
        );

        // A character that loses its rig takes its clips with it: the copies posed for one go back to models.
        for id in ["medium", "small", "other_set", "unity_set"] {
            store(&c, id, None).unwrap();
        }
        sort_pack(&c, "p").unwrap();
        assert_eq!(kind("crouch_blend").0, "model", "nothing left that it was shipped with");
        assert_eq!(kind("run_blend").0, "animation", "still named for its clip");
    }
}
