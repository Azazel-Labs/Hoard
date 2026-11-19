//! Dependency discovery: which files in a pack does each file need?
//!
//! - glTF/GLB: `buffers[].uri` and `images[].uri`
//! - OBJ: `mtllib` → MTL: `map_Kd`, `map_Bump`, `norm`, … (bundles follow links transitively)
//! - FBX: texture file names embedded in the binary (absolute artist paths are matched by file name)
//! - DAE: `<init_from>` image paths
//! - .blend: external image paths and linked libraries (other .blend files)
//! - SFZ: the samples it plays
//! - DragonBones skeletons: their atlas (`hero_ske.json` → `hero_tex.json`, `hero_tex.png`)
//! - Spine skeletons: their atlas (`spineboy-pro.json` → `spineboy.atlas`, or the only atlas in the folder)
//! - animations: clips and the characters they fit, by skeleton or `Model@Clip.fbx` name (see [`crate::animations`])
//! - guesses: textures whose name starts with the model's name, for models that reference nothing, from the model's
//!   own part of the pack (its folder's parent and below: `Models/` and `Textures/` side by side)
//!
//! Extraction happens per file during analysis (`extract_refs`); resolution needs the whole pack, so it
//! runs after the assets are inserted (`link_pack`).

use std::collections::{BTreeSet, HashMap};

use anyhow::Result;
use hoard_core::{AssetKind, LinkRel};
use rusqlite::{Connection, params};

mod extract;
mod families;

use extract::{FOLDER_ATLAS, is_atlas};
pub(crate) use extract::{Ref, extract_refs};
pub(crate) use families::assign_families;

const IMAGE_EXTS: &[&str] =
    &[".png", ".jpg", ".jpeg", ".tga", ".tif", ".tiff", ".psd", ".bmp", ".dds", ".exr", ".hdr", ".webp", ".ktx2"];
const MAX_GUESSES: usize = 12;

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16)
        {
            out.push(v);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Join a reference onto the referring file's folder, normalizing `.`/`..`. `None` if it escapes the pack.
fn join_rel(from_rel: &str, target: &str) -> Option<String> {
    let target = percent_decode(target.trim_start_matches("file://")).replace('\\', "/");
    let dir = from_rel.rsplit_once('/').map_or("", |(d, _)| d);
    let mut parts: Vec<&str> =
        if target.starts_with('/') { Vec::new() } else { dir.split('/').filter(|s| !s.is_empty()).collect() };
    for seg in target.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    Some(parts.join("/"))
}

/// An Unreal package path (`/Game/Pack/T_Rock`): rooted, no file extension.
fn is_package_path(target: &str) -> bool {
    target.starts_with('/') && !base_name(target).contains('.')
}

/// The pack file for an Unreal package path, matched against `.uasset`/`.umap` files under a `Content/` folder
/// (`Pack/Content/Pack/Textures/T_Rock.uasset` for `/Game/Pack/Textures/T_Rock`), or failing that by path suffix.
fn resolve_unreal(assets: &[PackAsset], content_rel: &str) -> Option<usize> {
    let want = content_rel.to_lowercase();
    // Converted assets answer to the package they came from.
    let packages: Vec<(usize, String)> = assets
        .iter()
        .enumerate()
        .filter_map(|(i, a)| {
            let lower = a.origin.as_deref().unwrap_or(&a.path).to_lowercase();
            let no_ext = lower.strip_suffix(".uasset").or_else(|| lower.strip_suffix(".umap"))?.to_owned();
            Some((i, no_ext))
        })
        .collect();
    if let Some((i, _)) = packages.iter().find(|(_, p)| p.ends_with(&format!("content/{want}"))) {
        return Some(*i);
    }
    // Uploads often lose the `Content/<Mount>/` prefix: try ever shorter suffixes of the package path, and the bare
    // name only when it's unambiguous.
    let parts: Vec<&str> = want.split('/').collect();
    for k in 0..parts.len() {
        let suffix = parts[k..].join("/");
        let mut hits = packages.iter().filter(|(_, p)| *p == suffix || p.ends_with(&format!("/{suffix}")));
        let first = hits.next();
        if first.is_some() && (k + 1 < parts.len() || hits.next().is_none()) {
            return first.map(|(i, _)| *i);
        }
    }
    None
}

/// What a reference by id (an Unreal package import, a Unity GUID) means depends on what it points at.
fn rel_for(kind: AssetKind) -> LinkRel {
    match kind {
        AssetKind::Texture => LinkRel::Texture,
        AssetKind::Material => LinkRel::Material,
        AssetKind::Animation => LinkRel::Animation,
        _ => LinkRel::Library,
    }
}

fn base_name(p: &str) -> &str {
    p.rsplit(['/', '\\']).next().unwrap_or(p)
}

fn stem(p: &str) -> &str {
    let file = base_name(p);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

/// Whether a texture may be the one a model that names none wears: it's named after the model (`Barrel.fbx`,
/// `Barrel_Albedo.png`) and sits in the model's own part of the pack, its folder's parent or below, so a "stairs"
/// sprite in a 2D kit elsewhere in a bundle isn't painted on a 3D kit's stairs.
fn guessable(model: &str, texture: &str) -> bool {
    let (ms, ts) = (stem(model).to_lowercase(), stem(texture).to_lowercase());
    let named = ts == ms || ts.strip_prefix(&ms).is_some_and(|rest| rest.starts_with(['_', '-', ' ', '.']));
    let parent = |p: &str| p.rsplit_once(['/', '\\']).map_or(String::new(), |(d, _)| d.to_lowercase());
    let (home, td) = (parent(&parent(model)), parent(texture));
    named && (home.is_empty() || td == home || td.strip_prefix(&home).is_some_and(|rest| rest.starts_with(['/', '\\'])))
}

#[derive(Debug)]
struct PackAsset {
    id: String,
    path: String,
    kind: AssetKind,
    /// Path before conversion (`T_Rock.uasset` for what is now `T_Rock.png`).
    origin: Option<String>,
}

/// Path lookups for one pack: exact, case-insensitive, by bare file name, and (for textures) by name without the
/// extension.
#[derive(Debug, Default)]
struct PathIndex {
    exact: HashMap<String, usize>,
    lower: HashMap<String, usize>,
    by_name: HashMap<String, Vec<usize>>,
    texture_by_stem: HashMap<String, Vec<usize>>,
}

impl PathIndex {
    fn build(assets: &[PackAsset]) -> Self {
        let mut idx = Self::default();
        for (i, a) in assets.iter().enumerate() {
            idx.exact.insert(a.path.clone(), i);
            idx.lower.insert(a.path.to_lowercase(), i);
            idx.by_name.entry(base_name(&a.path).to_lowercase()).or_default().push(i);
            if a.kind == AssetKind::Texture {
                idx.texture_by_stem.entry(stem(&a.path).to_lowercase()).or_default().push(i);
            }
        }
        idx
    }

    fn resolve(&self, assets: &[PackAsset], from_rel: &str, target: &str) -> Option<usize> {
        if let Some(joined) = join_rel(from_rel, target)
            && let Some(&i) = self.exact.get(&joined).or_else(|| self.lower.get(&joined.to_lowercase()))
        {
            return Some(i);
        }
        // Absolute artist paths ("C:\Users\me\textures\Rock.png") and wrong folders: fall back to the file
        // name, preferring the candidate that shares the most leading folders with the referring file.
        let target = percent_decode(target);
        let candidates = self.by_name.get(&base_name(&target).to_lowercase()).or_else(|| {
            // Artists' files point at the source they painted (`Rock.psd`) while the pack ships an export of it
            // (`Rock.png`): any texture with the same name will do.
            let is_image = IMAGE_EXTS.iter().any(|e| target.to_lowercase().ends_with(e));
            is_image.then(|| self.texture_by_stem.get(&stem(&target).to_lowercase())).flatten()
        })?;
        let from_dir: Vec<&str> = from_rel.split('/').collect();
        candidates
            .iter()
            .copied()
            .max_by_key(|&i| assets[i].path.split('/').zip(&from_dir).take_while(|(a, b)| a == *b).count())
    }
}

/// Resolve the refs collected during analysis into `asset_links` rows, plus animation and name-based links.
/// `refs` maps asset id -> refs extracted from that asset's file.
pub(crate) fn link_pack(conn: &Connection, pack_id: &str, refs: &HashMap<String, Vec<Ref>>) -> Result<usize> {
    let assets: Vec<PackAsset> = {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.path, a.kind, o.path FROM assets a LEFT JOIN asset_origin o ON o.asset_id = a.id
             WHERE a.pack_id = ?1",
        )?;
        stmt.query_map([pack_id], |r| {
            Ok(PackAsset {
                id: r.get(0)?,
                path: r.get(1)?,
                kind: AssetKind::parse(&r.get::<_, String>(2)?).unwrap_or(AssetKind::Other),
                origin: r.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    let index = PathIndex::build(&assets);
    let mut links: BTreeSet<(usize, usize, LinkRel)> = BTreeSet::new();
    // Unity GUID -> asset, from the .meta files the pack came with.
    let by_path: HashMap<&str, usize> = assets.iter().enumerate().map(|(i, a)| (a.path.as_str(), i)).collect();
    let unity: HashMap<String, usize> = {
        let mut stmt = conn.prepare("SELECT guid, path FROM unity_guids WHERE pack_id = ?1")?;
        let rows = stmt.query_map([pack_id], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        let mut m = HashMap::new();
        for row in rows {
            let (guid, path) = row?;
            if let Some(&i) = by_path.get(path.as_str()) {
                m.insert(guid, i);
            }
        }
        m
    };

    let by_id: HashMap<&str, usize> = assets.iter().enumerate().map(|(i, a)| (a.id.as_str(), i)).collect();
    for (id, list) in refs {
        let Some(&from) = by_id.get(id.as_str()) else { continue };
        // Resolved last, once whatever the asset names has been.
        let (fallbacks, list): (Vec<&Ref>, Vec<&Ref>) = list.iter().partition(|r| r.target == FOLDER_ATLAS);
        for r in list {
            if let Some(guid) = r.target.strip_prefix("unity-guid:") {
                if let Some(&to) = unity.get(guid)
                    && to != from
                {
                    links.insert((from, to, rel_for(assets[to].kind)));
                }
                continue;
            }
            if let Some(pkg) = hoard_unreal::content_relative(&r.target).filter(|_| is_package_path(&r.target)) {
                if let Some(to) = resolve_unreal(&assets, pkg)
                    && to != from
                {
                    links.insert((from, to, rel_for(assets[to].kind)));
                }
                continue;
            }
            if let Some(to) = index.resolve(&assets, &assets[from].path, &r.target)
                && to != from
            {
                links.insert((from, to, r.rel));
            }
        }
        if !fallbacks.is_empty() && !links.iter().any(|&(f, t, _)| f == from && is_atlas(&assets[t].path)) {
            let folder = |p: &str| p.rfind('/').map_or("", |i| &p[..i]).to_owned();
            let here = folder(&assets[from].path);
            let mut atlases =
                (0..assets.len()).filter(|&i| is_atlas(&assets[i].path) && folder(&assets[i].path) == here);
            if let (Some(to), None) = (atlases.next(), atlases.next()) {
                links.insert((from, to, LinkRel::Library));
            }
        }
    }

    let models: Vec<usize> = (0..assets.len()).filter(|&i| assets[i].kind == AssetKind::Model).collect();
    // Models that reference no textures: guess from names ("Barrel.fbx" + "Barrel_Albedo.png").
    for &m in &models {
        let references_something =
            links.iter().any(|(f, _, r)| *f == m && matches!(r, LinkRel::Texture | LinkRel::Material));
        if stem(&assets[m].path).contains('@') || references_something {
            continue;
        }
        if stem(&assets[m].path).len() < 3 {
            continue;
        }
        let guesses = assets
            .iter()
            .enumerate()
            .filter(|(_, a)| a.kind == AssetKind::Texture && guessable(&assets[m].path, &a.path))
            .map(|(i, _)| i)
            .take(MAX_GUESSES)
            .collect::<Vec<_>>();
        for t in guesses {
            links.insert((m, t, LinkRel::Guess));
        }
    }

    let mut stmt =
        conn.prepare("INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING")?;
    for (f, t, rel) in &links {
        stmt.execute(params![assets[*f].id, assets[*t].id, rel.as_str()])?;
    }
    // Which models are animations, and the characters they play on.
    crate::animations::sort_pack(conn, pack_id)?;
    Ok(links.len())
}

#[cfg(test)]
mod tests;
