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
//! Detection runs after a pack is ingested or re-tagged, and as a `materials` job for packs from before it existed.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use anyhow::Result;
use hoard_core::{AssetKind, MaterialInfo, MaterialMap, TagSource};
use rusqlite::{Connection, OptionalExtension, params};
use tokio::runtime::Handle;

use crate::tagger::{self, TagSet};
use crate::{AppState, db, links};

/// What a map is for. Stored by name in `material_maps.role`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Role {
    BaseColor,
    /// OpenGL-style (green up), or not said.
    Normal,
    /// DirectX-style (green down).
    NormalDx,
    Roughness,
    /// Inverted roughness.
    Gloss,
    Metalness,
    Ao,
    /// Red occlusion, green roughness, blue metalness (also called ARM).
    Orm,
    /// Red metalness, green roughness, blue occlusion.
    Mra,
    /// Red roughness, green metalness, blue occlusion.
    Rma,
    Height,
    Bump,
    Cavity,
    Specular,
    Opacity,
    Emissive,
    Translucency,
}

impl Role {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Role::BaseColor => "base_color",
            Role::Normal => "normal",
            Role::NormalDx => "normal_dx",
            Role::Roughness => "roughness",
            Role::Gloss => "gloss",
            Role::Metalness => "metalness",
            Role::Ao => "ao",
            Role::Orm => "orm",
            Role::Mra => "mra",
            Role::Rma => "rma",
            Role::Height => "height",
            Role::Bump => "bump",
            Role::Cavity => "cavity",
            Role::Specular => "specular",
            Role::Opacity => "opacity",
            Role::Emissive => "emissive",
            Role::Translucency => "translucency",
        }
    }

    pub(crate) fn parse(s: &str) -> Option<Self> {
        ALL_ROLES.iter().copied().find(|r| r.as_str() == s)
    }

    /// Roles that say the same thing count once toward "two kinds of map" (GL and DX normals are one).
    fn family(self) -> Self {
        if self == Role::NormalDx { Role::Normal } else { self }
    }
}

const ALL_ROLES: [Role; 17] = [
    Role::BaseColor,
    Role::Normal,
    Role::NormalDx,
    Role::Roughness,
    Role::Gloss,
    Role::Metalness,
    Role::Ao,
    Role::Orm,
    Role::Mra,
    Role::Rma,
    Role::Height,
    Role::Bump,
    Role::Cavity,
    Role::Specular,
    Role::Opacity,
    Role::Emissive,
    Role::Translucency,
];

/// Map-type words at the end of a file name, as lowercase tokens (see [`tokens`]). `true` marks a word that's only
/// trusted next to a colour or normal map with the same name (`T_Rock_D` + `T_Rock_N`), since on its own it's
/// as likely to mean something else.
const ROLE_WORDS: &[(&str, Role, bool)] = &[
    ("base color", Role::BaseColor, false),
    ("base colour", Role::BaseColor, false),
    ("basecolor", Role::BaseColor, false),
    ("basecolour", Role::BaseColor, false),
    ("base map", Role::BaseColor, false),
    ("albedo transparency", Role::BaseColor, false),
    ("albedo", Role::BaseColor, false),
    ("alb", Role::BaseColor, false),
    ("diffuse", Role::BaseColor, false),
    ("diff", Role::BaseColor, false),
    ("dif", Role::BaseColor, false),
    ("color", Role::BaseColor, false),
    ("colour", Role::BaseColor, false),
    ("col", Role::BaseColor, true),
    ("bc", Role::BaseColor, true),
    ("d", Role::BaseColor, true),
    ("normal", Role::Normal, false),
    ("normals", Role::Normal, false),
    ("normal map", Role::Normal, false),
    ("normalgl", Role::Normal, false),
    ("normaldx", Role::NormalDx, false),
    ("nor", Role::Normal, false),
    ("nrm", Role::Normal, false),
    ("norm", Role::Normal, false),
    ("nml", Role::Normal, false),
    ("n", Role::Normal, true),
    ("roughness", Role::Roughness, false),
    ("rough", Role::Roughness, false),
    ("rgh", Role::Roughness, false),
    ("r", Role::Roughness, true),
    ("glossiness", Role::Gloss, false),
    ("gloss", Role::Gloss, false),
    ("smoothness", Role::Gloss, false),
    ("metallic", Role::Metalness, false),
    ("metalness", Role::Metalness, false),
    ("metallic smoothness", Role::Metalness, false),
    ("metal", Role::Metalness, true),
    ("met", Role::Metalness, true),
    ("ambient occlusion", Role::Ao, false),
    ("ambientocclusion", Role::Ao, false),
    ("mixed ao", Role::Ao, false),
    ("occlusion", Role::Ao, false),
    ("ao", Role::Ao, false),
    ("occ", Role::Ao, true),
    ("orm", Role::Orm, false),
    ("arm", Role::Orm, false),
    ("occlusion roughness metallic", Role::Orm, false),
    ("ao roughness metallic", Role::Orm, false),
    ("mra", Role::Mra, false),
    ("rma", Role::Rma, false),
    ("displacement", Role::Height, false),
    ("disp", Role::Height, false),
    ("displace", Role::Height, false),
    ("height", Role::Height, true),
    ("h", Role::Height, true),
    ("bump", Role::Bump, false),
    ("cavity", Role::Cavity, false),
    ("cav", Role::Cavity, true),
    ("specular", Role::Specular, false),
    ("spec", Role::Specular, false),
    ("opacity", Role::Opacity, false),
    ("transparency", Role::Opacity, false),
    ("alpha", Role::Opacity, true),
    ("mask", Role::Opacity, true),
    ("emissive", Role::Emissive, false),
    ("emission", Role::Emissive, false),
    ("emit", Role::Emissive, false),
    ("glow", Role::Emissive, true),
    ("e", Role::Emissive, true),
    ("translucency", Role::Translucency, false),
    ("transmission", Role::Translucency, false),
];

/// Words after a normal map's type saying which way its green channel points.
const GL_WORDS: &[&str] = &["gl", "ogl", "opengl", "open gl"];
const DX_WORDS: &[&str] = &["dx", "directx", "direct x"];

/// A file-name token with where it sits in the name.
#[derive(Debug, Clone)]
struct Token {
    text: String,
    end: usize,
}

/// Split a file stem into lowercase word tokens like [`tagger::tokenize`] does (punctuation, camelCase and
/// letter-to-digit boundaries; `8K` stays one token), keeping each token's byte range.
fn tokens(stem: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut cur: Option<(usize, usize)> = None;
    let chars: Vec<(usize, char)> = stem.char_indices().collect();
    let flush = |cur: &mut Option<(usize, usize)>, out: &mut Vec<Token>| {
        if let Some((s, e)) = cur.take() {
            out.push(Token { text: stem[s..e].to_lowercase(), end: e });
        }
    };
    for (i, &(pos, c)) in chars.iter().enumerate() {
        if !c.is_alphanumeric() {
            flush(&mut cur, &mut out);
            continue;
        }
        if let Some((_, _)) = cur
            && i > 0
        {
            let prev = chars[i - 1].1;
            let next_lower = chars.get(i + 1).is_some_and(|(_, n)| n.is_lowercase());
            let boundary = (prev.is_lowercase() && c.is_uppercase())
                || (prev.is_alphabetic() && c.is_ascii_digit())
                || (prev.is_uppercase() && c.is_uppercase() && next_lower);
            if boundary {
                flush(&mut cur, &mut out);
            }
        }
        let end = pos + c.len_utf8();
        cur = Some(cur.map_or((pos, end), |(s, _)| (s, end)));
    }
    flush(&mut cur, &mut out);
    out
}

/// A resolution written into a name: `2k`, `8K`, `2048`, `4096x4096`. Returns it as shown (`2K`, `512`).
fn resolution_token(t: &str) -> Option<String> {
    if let Some(n) = t.strip_suffix('k')
        && (1..=2).contains(&n.len())
        && n.chars().all(|c| c.is_ascii_digit())
    {
        return Some(format!("{}K", n.trim_start_matches('0')));
    }
    let px =
        |s: &str| -> Option<u32> { s.parse().ok().filter(|n| [256, 512, 1024, 2048, 4096, 8192, 16384].contains(n)) };
    if let Some(n) = px(t) {
        return Some(resolution_label(n));
    }
    let (a, b) = t.split_once('x')?;
    px(a).zip(px(b)).map(|(a, b)| resolution_label(a.max(b)))
}

/// `2048` → `2K`, `512` → `512`.
pub(crate) fn resolution_label(px: u32) -> String {
    if px >= 1024 && px.is_multiple_of(1024) { format!("{}K", px / 1024) } else { px.to_string() }
}

/// Words a material's file names carry that aren't part of its name: sizes and file formats (`2K-JPG`).
fn is_noise(t: &str) -> bool {
    resolution_token(t).is_some()
        || matches!(t, "jpg" | "jpeg" | "png" | "exr" | "tif" | "tiff" | "tga" | "psd" | "webp")
}

/// What a texture's name says about it as part of a material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapName {
    /// The name without the map type, resolution and format, as written (`Bricks059`, `rock_wall`, `T_Rock`).
    pub(crate) base: String,
    /// `base` as lowercase words joined by `_`, to group by.
    pub(crate) key: String,
    pub(crate) role: Role,
    /// Only trusted beside a colour or normal map (see [`ROLE_WORDS`]).
    pub(crate) weak: bool,
    /// A resolution in the name (`8K`).
    pub(crate) resolution: Option<String>,
}

/// Does `words` (space-separated) end exactly at `end` in `toks`? Returns where it starts.
fn ends_with(toks: &[Token], end: usize, words: &str) -> Option<usize> {
    let w: Vec<&str> = words.split(' ').collect();
    let start = end.checked_sub(w.len())?;
    toks[start..end].iter().zip(&w).all(|(t, w)| t.text == *w).then_some(start)
}

/// The longest map-type word ending at `end`: (start, role, weak).
fn role_at(toks: &[Token], end: usize) -> Option<(usize, Role, bool)> {
    ROLE_WORDS
        .iter()
        .filter_map(|(words, role, weak)| ends_with(toks, end, words).map(|start| (start, *role, *weak)))
        .min_by_key(|(start, _, _)| *start)
}

/// Read a texture's file stem as a map: `Rock_8K_Normal` → base `Rock`, normal, 8K.
pub(crate) fn parse_map(stem: &str) -> Option<MapName> {
    let toks = tokens(stem);
    let mut resolution = None;
    let mut note = |t: &str| {
        if resolution.is_none() {
            resolution = resolution_token(t);
        }
    };
    let mut end = toks.len();
    while end > 0 && is_noise(&toks[end - 1].text) {
        note(&toks[end - 1].text);
        end -= 1;
    }
    // "nor_gl", "Normal_OpenGL", "NormalDX".
    // Every spelling is tried: "OpenGL" ends in "gl" too, but only "open gl" leaves "normal" in front of it.
    let flip =
        GL_WORDS.iter().map(|w| (w, false)).chain(DX_WORDS.iter().map(|w| (w, true))).find_map(
            |(w, dx)| match role_at(&toks, ends_with(&toks, end, w)?)? {
                (start, Role::Normal | Role::NormalDx, weak) => {
                    Some((start, if dx { Role::NormalDx } else { Role::Normal }, weak))
                }
                _ => None,
            },
        );
    let (start, role, weak) = match flip {
        Some(found) => found,
        None => role_at(&toks, end)?,
    };
    let mut keep = start;
    while keep > 0 && is_noise(&toks[keep - 1].text) {
        note(&toks[keep - 1].text);
        keep -= 1;
    }
    let base = if keep == 0 { String::new() } else { stem[..toks[keep - 1].end].trim_end().to_owned() };
    let key = toks[..keep].iter().map(|t| t.text.as_str()).collect::<Vec<_>>().join("_");
    // Unreal's T_Name_N names its normal maps the DirectX way.
    let role =
        if role == Role::Normal && toks.first().is_some_and(|t| t.text == "t") && keep > 1 && toks[start].text == "n" {
            Role::NormalDx
        } else {
            role
        };
    Some(MapName { base, key, role, weak, resolution })
}

/// A folder as a grouping key: segments that carry a resolution (`2K`, `Bricks059_2K-JPG`) lose it, so the same set
/// in several sizes lands in one group.
fn dir_key(dir: &str) -> String {
    dir.split('/')
        .map(|seg| {
            let toks = tokens(seg);
            if toks.iter().any(|t| resolution_token(&t.text).is_some()) {
                toks.iter().filter(|t| !is_noise(&t.text)).map(|t| t.text.as_str()).collect::<Vec<_>>().join("_")
            } else {
                seg.to_lowercase()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// A texture to consider: its path and its longer side in pixels, when known.
#[derive(Debug, Clone)]
pub(crate) struct Candidate<'a> {
    pub(crate) path: &'a str,
    pub(crate) size_px: Option<u32>,
}

/// One material found among a pack's textures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Found {
    /// The material's own path: `<folder>/<name>.material`.
    pub(crate) path: String,
    pub(crate) name: String,
    /// Lowercase words of the shared name (`rock_wall`), for matching models to it.
    pub(crate) key: String,
    /// (index into the candidates, role, resolution), by role then largest first.
    pub(crate) maps: Vec<(usize, Role, Option<String>)>,
}

fn dir_of(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(d, _)| d)
}

fn stem_of(path: &str) -> &str {
    let file = path.rsplit('/').next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

/// A readable name for a material: its shared file name, or the folder's when the file name is only an id
/// (Megascans' `vlkhcbxia_8K_Albedo.jpg` in `rock_cliff_vlkhcbxia/`) or there's none (`Rock/albedo.png`).
fn material_name(base: &str, dir: &str) -> String {
    let words: Vec<String> = tokens(base).into_iter().map(|t| t.text).collect();
    // Unreal's texture prefix.
    let words: Vec<String> = if words.len() > 1 && words[0] == "t" { words[1..].to_vec() } else { words };
    let folder = dir.rsplit('/').next().unwrap_or("");
    let folder_words: Vec<String> = tokens(folder).into_iter().map(|t| t.text).filter(|t| !is_noise(t)).collect();
    let chosen = if words.is_empty() {
        folder_words
    } else if words.len() == 1 && folder_words.len() > 1 && folder_words.contains(&words[0]) {
        folder_words.into_iter().filter(|w| *w != words[0]).collect()
    } else {
        words
    };
    if chosen.is_empty() { "Material".to_owned() } else { tagger::display_name(&chosen.join(" ")) }
}

/// Group a pack's textures into materials. A material needs two kinds of map (a colour map and a normal map,
/// say); a set named only with one-letter or other loose words also needs a colour or normal map among them.
pub(crate) fn find(textures: &[Candidate<'_>]) -> Vec<Found> {
    struct Member {
        idx: usize,
        name: MapName,
    }
    let mut groups: BTreeMap<(String, String), Vec<Member>> = BTreeMap::new();
    for (idx, c) in textures.iter().enumerate() {
        let Some(name) = parse_map(stem_of(c.path)) else { continue };
        groups.entry((dir_key(dir_of(c.path)), name.key.clone())).or_default().push(Member { idx, name });
    }
    let taken: HashSet<String> = textures.iter().map(|c| c.path.to_lowercase()).collect();
    let mut seen_paths = HashSet::new();
    let mut out = Vec::new();
    for ((_, key), members) in groups {
        let families: BTreeSet<Role> = members.iter().map(|m| m.name.role.family()).collect();
        if families.len() < 2 {
            continue;
        }
        let only_loose = members.iter().all(|m| m.name.weak);
        if only_loose && !families.contains(&Role::BaseColor) && !families.contains(&Role::Normal) {
            continue;
        }
        let dirs: Vec<&str> = members.iter().map(|m| textures[m.idx].path).collect();
        let dir = db::common_dir(&dirs);
        // Spelled as the first file spells it; a map with no shared name is named after its folder.
        let base = members.iter().map(|m| m.name.base.as_str()).find(|b| !b.is_empty()).unwrap_or("");
        let name = material_name(base, &dir);
        let file =
            if base.is_empty() { dir.rsplit('/').next().filter(|f| !f.is_empty()).unwrap_or("Material") } else { base };
        let path = if dir.is_empty() { format!("{file}.material") } else { format!("{dir}/{file}.material") };
        if taken.contains(&path.to_lowercase()) || !seen_paths.insert(path.to_lowercase()) {
            continue;
        }
        let mut maps: Vec<(usize, Role, Option<String>, u32)> = members
            .iter()
            .map(|m| {
                let px = textures[m.idx].size_px;
                let res = px.map(resolution_label).or_else(|| m.name.resolution.clone());
                (m.idx, m.name.role, res, px.unwrap_or(0))
            })
            .collect();
        maps.sort_by(|a, b| a.1.cmp(&b.1).then(b.3.cmp(&a.3)).then(textures[a.0].path.cmp(textures[b.0].path)));
        out.push(Found { path, name, key, maps: maps.into_iter().map(|(i, r, res, _)| (i, r, res)).collect() });
    }
    out
}

// ---------------------------------------------------------------------------------------------
// The library side

/// A material made from texture maps, as SQL over an asset `a`.
pub(crate) const IS_MATERIAL: &str = "(a.ext = 'material' AND json_extract(a.meta, '$.material_maps') IS NOT NULL)";

/// Thumbnail size, as for every other asset.
const THUMB_SIZE: u32 = 256;

/// A material ready to write: its file and thumbnail are stored.
struct Made {
    found: Found,
    descriptor: crate::blobs::StoredBlob,
    thumb: Option<crate::blobs::StoredBlob>,
}

#[derive(Debug)]
struct Texture {
    id: String,
    path: String,
    hash: String,
    thumb: Option<String>,
    size_px: Option<u32>,
}

/// Find the pack's materials and bring the library in line: new ones added, changed ones updated (keeping their
/// id, tags people gave them and collections), ones whose maps are gone removed. Returns how many it has.
pub(crate) fn detect_pack(state: &AppState, rt: &Handle, conn: &mut Connection, pack_id: &str) -> Result<usize> {
    let textures: Vec<Texture> = {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.path, a.blob_hash, a.thumb_hash,
                    MAX(COALESCE(json_extract(a.meta, '$.width'), 0), COALESCE(json_extract(a.meta, '$.height'), 0))
             FROM assets a WHERE a.pack_id = ?1 AND a.kind = 'texture' ORDER BY a.path",
        )?;
        stmt.query_map([pack_id], |r| {
            Ok(Texture {
                id: r.get(0)?,
                path: r.get(1)?,
                hash: r.get(2)?,
                thumb: r.get(3)?,
                size_px: r.get::<_, Option<i64>>(4)?.and_then(|n| u32::try_from(n).ok()).filter(|&n| n > 0),
            })
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    let candidates: Vec<Candidate<'_>> =
        textures.iter().map(|t| Candidate { path: &t.path, size_px: t.size_px }).collect();
    let found = find(&candidates);
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
        let descriptor = serde_json::json!({
            "hoard_material": 1,
            "name": f.name,
            "maps": f.maps.iter().map(|(i, role, res)| serde_json::json!({
                "role": role.as_str(),
                "file": db::strip_dir(&textures[*i].path, &dir),
                "resolution": res,
                "hash": textures[*i].hash,
            })).collect::<Vec<_>>(),
        });
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
                None
            }
        };
        made.push(Made { found: f, descriptor, thumb });
    }

    let now = db::now();
    let tx = conn.transaction()?;
    let mut kept = HashSet::new();
    for m in &made {
        let f = &m.found;
        let id = existing.get(&f.path).map_or_else(|| uuid::Uuid::now_v7().to_string(), |(id, ..)| id.clone());
        kept.insert(id.clone());
        for b in [Some(&m.descriptor), m.thumb.as_ref()].into_iter().flatten() {
            tx.execute(
                "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
                params![b.hash, b.size as i64, now],
            )?;
        }
        let roles: BTreeSet<&str> = f.maps.iter().map(|(_, r, _)| r.as_str()).collect();
        let resolutions: Vec<&str> = {
            let mut v: Vec<&str> = f.maps.iter().filter_map(|(_, _, r)| r.as_deref()).collect();
            v.sort_by_key(|r| std::cmp::Reverse(res_px(r)));
            v.dedup();
            v
        };
        let meta = serde_json::json!({ "material_maps": f.maps.len(), "roles": roles, "resolutions": resolutions });
        let tags = material_tags(&tx, &textures, f)?;
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
                format!("{} material pbr", tagger::fts_words(&f.path)),
                m.thumb.as_ref().map(|t| t.hash.clone()),
                now,
            ],
        )?;
        tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND source IN ('rule', 'meta')", [&id])?;
        for s in tags.into_vec() {
            db::upsert_tag(&tx, &id, &s.tag, s.source, s.confidence)?;
        }
        db::reindex_asset(&tx, &id)?;
        tx.execute("DELETE FROM material_maps WHERE material_id = ?1", [&id])?;
        tx.execute("DELETE FROM asset_links WHERE from_id = ?1", [&id])?;
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
    }
    // Materials whose maps are gone (or no longer form a set).
    for (id, _, _, synthetic) in existing.values() {
        if *synthetic && !kept.contains(id) {
            tx.execute("DELETE FROM assets_fts WHERE asset_id = ?1", [id])?;
            tx.execute("DELETE FROM assets WHERE id = ?1", [id])?;
        }
    }
    link_models(&tx, pack_id)?;
    links::assign_families(&tx, pack_id)?;
    tx.commit()?;
    Ok(made.len())
}

/// `8K` → 8192, for sorting.
fn res_px(r: &str) -> u32 {
    r.strip_suffix('K').and_then(|n| n.parse::<u32>().ok()).map_or_else(|| r.parse().unwrap_or(0), |k| k * 1024)
}

/// A material's tags: what its colour map (or first map) was tagged with, less what only describes a single file
/// (its map type, format, size or shape), plus the sizes it comes in.
fn material_tags(tx: &Connection, textures: &[Texture], f: &Found) -> Result<TagSet> {
    const DROP_PREFIX: &[&str] = &["map:", "set:", "format:", "depth:", "res:"];
    const DROP: &[&str] = &["pot", "square", "alpha", "spritesheet", "frames", "hdr", "hdri", "sky", "layered"];
    let lead = f.maps.iter().find(|(_, r, _)| *r == Role::BaseColor).unwrap_or(&f.maps[0]);
    let mut stmt = tx.prepare_cached("SELECT tag, source, confidence FROM asset_tags WHERE asset_id = ?1")?;
    let rows: Vec<(String, String, f64)> = stmt
        .query_map([&textures[lead.0].id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
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

/// Link the pack's models to the materials made for them, by name: `Barrel.fbx` or `vlkhcbxia_LOD0.fbx` in the
/// material's folder (or above it) to the `Barrel` or `vlkhcbxia` material. A model that links to a material's maps
/// shows the material already; this is for the models that name no textures.
fn link_models(tx: &Connection, pack_id: &str) -> Result<()> {
    tx.execute(
        &format!(
            "DELETE FROM asset_links WHERE rel = 'material'
               AND to_id IN (SELECT a.id FROM assets a WHERE a.pack_id = ?1 AND {IS_MATERIAL})"
        ),
        [pack_id],
    )?;
    let materials: Vec<(String, String, String)> = {
        let mut stmt = tx.prepare(&format!(
            "SELECT a.id, a.path, (SELECT group_concat(m.texture_id) FROM material_maps m WHERE m.material_id = a.id)
             FROM assets a WHERE a.pack_id = ?1 AND {IS_MATERIAL}"
        ))?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, Option<String>>(2)?.unwrap_or_default())))?
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
        let words: Vec<String> = tokens(stem_of(model_path)).into_iter().map(|t| t.text).collect();
        let model_dir = dir_key(dir_of(model_path));
        for (mat_id, mat_path, maps) in &materials {
            let key: Vec<String> = tokens(stem_of(mat_path)).into_iter().map(|t| t.text).collect();
            let mat_dir = dir_key(dir_of(mat_path));
            let near = model_dir == mat_dir || mat_dir.starts_with(&format!("{model_dir}/")) || model_dir.is_empty();
            let named = !key.is_empty() && words.len() >= key.len() && words[..key.len()] == key[..];
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

/// Shade a sphere from the maps' thumbnails. None when there's nothing to shade it with.
fn render_thumb(state: &AppState, rt: &Handle, textures: &[Texture], f: &Found) -> Result<Option<Vec<u8>>> {
    use hoard_render::sphere::{Maps, Packed, render_sphere};
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
    if by_role.is_empty() {
        return Ok(None);
    }
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
    };
    let img = render_sphere(&maps, THUMB_SIZE);
    Ok(Some(crate::analyze::png_bytes(img)?))
}

/// One material with its maps, for an asset's detail.
fn info(conn: &Connection, id: &str) -> Result<Option<MaterialInfo>> {
    let head: Option<(String, String, bool)> = conn
        .query_row("SELECT name, path, thumb_hash IS NOT NULL FROM assets WHERE id = ?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .optional()?;
    let Some((name, path, has_thumb)) = head else { return Ok(None) };
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
    Ok(Some(MaterialInfo { id: id.to_owned(), name, path, has_thumb, maps }))
}

/// Fill in an asset's material facts: a material's maps, or the materials a texture is part of or a model uses
/// (through a map it links to, or linked by name).
pub(crate) fn attach(conn: &Connection, asset: &mut hoard_core::Asset) -> Result<()> {
    let is_material: bool =
        conn.query_row("SELECT EXISTS (SELECT 1 FROM material_maps WHERE material_id = ?1)", [&asset.id], |r| {
            r.get(0)
        })?;
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
         ) LIMIT 8"
    ))?;
    let ids: Vec<String> = stmt.query_map([&asset.id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    for id in ids {
        if let Some(m) = info(conn, &id)? {
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
        Err(e) => tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not look for materials"),
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

    fn role(stem: &str) -> Option<(String, Role, Option<String>)> {
        parse_map(stem).map(|m| (m.base, m.role, m.resolution))
    }

    #[expect(clippy::unnecessary_wraps, reason = "reads as the parse it's compared with")]
    fn r(base: &str, role: Role, res: Option<&str>) -> Option<(String, Role, Option<String>)> {
        Some((base.to_owned(), role, res.map(str::to_owned)))
    }

    #[test]
    fn reads_megascans_names() {
        assert_eq!(role("vlkhcbxia_8K_AO"), r("vlkhcbxia", Role::Ao, Some("8K")));
        assert_eq!(role("vlkhcbxia_8K_BaseColor"), r("vlkhcbxia", Role::BaseColor, Some("8K")));
        assert_eq!(role("vlkhcbxia_8K_Bump"), r("vlkhcbxia", Role::Bump, Some("8K")));
        assert_eq!(role("vlkhcbxia_8K_Cavity"), r("vlkhcbxia", Role::Cavity, Some("8K")));
        assert_eq!(role("vlkhcbxia_8K_Gloss"), r("vlkhcbxia", Role::Gloss, Some("8K")));
        assert_eq!(role("vlkhcbxia_8K_Normal"), r("vlkhcbxia", Role::Normal, Some("8K")));
        assert_eq!(role("vlkhcbxia_8K_Roughness"), r("vlkhcbxia", Role::Roughness, Some("8K")));
        assert_eq!(role("vlkhcbxia_8K_Specular"), r("vlkhcbxia", Role::Specular, Some("8K")));
        assert_eq!(role("vlkhcbxia_4K_Albedo"), r("vlkhcbxia", Role::BaseColor, Some("4K")));
        assert_eq!(role("vlkhcbxia_4K_Displacement"), r("vlkhcbxia", Role::Height, Some("4K")));
    }

    #[test]
    fn reads_poly_haven_names() {
        assert_eq!(role("rock_wall_diff_4k"), r("rock_wall", Role::BaseColor, Some("4K")));
        assert_eq!(role("rock_wall_nor_gl_4k"), r("rock_wall", Role::Normal, Some("4K")));
        assert_eq!(role("rock_wall_nor_dx_4k"), r("rock_wall", Role::NormalDx, Some("4K")));
        assert_eq!(role("rock_wall_rough_4k"), r("rock_wall", Role::Roughness, Some("4K")));
        assert_eq!(role("rock_wall_arm_4k"), r("rock_wall", Role::Orm, Some("4K")));
        assert_eq!(role("rock_wall_disp_4k"), r("rock_wall", Role::Height, Some("4K")));
        assert_eq!(role("rock_wall_ao_1k"), r("rock_wall", Role::Ao, Some("1K")));
    }

    #[test]
    fn reads_ambientcg_names() {
        assert_eq!(role("Bricks059_2K-JPG_Color"), r("Bricks059", Role::BaseColor, Some("2K")));
        assert_eq!(role("Bricks059_2K-JPG_NormalGL"), r("Bricks059", Role::Normal, Some("2K")));
        assert_eq!(role("Bricks059_2K-JPG_NormalDX"), r("Bricks059", Role::NormalDx, Some("2K")));
        assert_eq!(role("Bricks059_2K-JPG_Roughness"), r("Bricks059", Role::Roughness, Some("2K")));
        assert_eq!(role("Bricks059_2K-JPG_Displacement"), r("Bricks059", Role::Height, Some("2K")));
        assert_eq!(role("Bricks059_2K-JPG_AmbientOcclusion"), r("Bricks059", Role::Ao, Some("2K")));
        assert_eq!(role("Metal032_1K_Metalness"), r("Metal032", Role::Metalness, Some("1K")));
        assert_eq!(role("Bricks059_4K-PNG_Color"), r("Bricks059", Role::BaseColor, Some("4K")));
    }

    #[test]
    fn reads_substance_and_unreal_names() {
        assert_eq!(role("Crate_BaseColor"), r("Crate", Role::BaseColor, None));
        assert_eq!(role("Crate_Base_Color"), r("Crate", Role::BaseColor, None));
        assert_eq!(role("Crate_Normal_OpenGL"), r("Crate", Role::Normal, None));
        assert_eq!(role("Crate_Normal_DirectX"), r("Crate", Role::NormalDx, None));
        assert_eq!(role("Crate_Mixed_AO"), r("Crate", Role::Ao, None));
        assert_eq!(role("Crate_OcclusionRoughnessMetallic"), r("Crate", Role::Orm, None));
        assert_eq!(role("Crate_Metallic"), r("Crate", Role::Metalness, None));
        assert_eq!(role("Crate_Height"), r("Crate", Role::Height, None));
        assert_eq!(role("Crate_Emissive"), r("Crate", Role::Emissive, None));
        assert_eq!(role("Crate_AlbedoTransparency"), r("Crate", Role::BaseColor, None));
        assert_eq!(role("T_Rock_D"), r("T_Rock", Role::BaseColor, None));
        assert_eq!(role("T_Rock_N"), r("T_Rock", Role::NormalDx, None));
        assert_eq!(role("T_Rock_ORM"), r("T_Rock", Role::Orm, None));
        assert_eq!(role("T_Rock_BC"), r("T_Rock", Role::BaseColor, None));
        assert_eq!(role("albedo"), r("", Role::BaseColor, None));
        assert_eq!(role("Rock-Wall Normal 2048"), r("Rock-Wall", Role::Normal, Some("2K")));
    }

    #[test]
    fn leaves_other_names_alone() {
        assert_eq!(role("Hero_Walk_01"), None);
        assert_eq!(role("button_hover"), None);
        assert_eq!(role("sky_4k"), None);
        assert_eq!(role("Rock_Normal_01"), None);
    }

    fn found(paths: &[&str]) -> Vec<Found> {
        let c: Vec<Candidate<'_>> = paths.iter().map(|p| Candidate { path: p, size_px: None }).collect();
        find(&c)
    }

    #[test]
    fn groups_a_megascans_surface() {
        let files = [
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_AO.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_BaseColor.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_Bump.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_Cavity.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_Gloss.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_Normal.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_Roughness.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_8K_Specular.jpg",
            "rock_cliff_vlkhcbxia/vlkhcbxia_Preview.png",
        ];
        let m = found(&files);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].path, "rock_cliff_vlkhcbxia/vlkhcbxia.material");
        assert_eq!(m[0].name, "Rock cliff");
        assert_eq!(m[0].maps.len(), 8);
        assert_eq!(m[0].maps[0].1, Role::BaseColor);
        assert!(m[0].maps.iter().all(|(i, _, res)| *i != 8 && res.as_deref() == Some("8K")));
    }

    #[test]
    fn resolutions_are_variants_of_one_material() {
        let m = found(&[
            "Bricks059_2K-JPG/Bricks059_2K-JPG_Color.jpg",
            "Bricks059_2K-JPG/Bricks059_2K-JPG_NormalGL.jpg",
            "Bricks059_4K-JPG/Bricks059_4K-JPG_Color.jpg",
            "Bricks059_4K-JPG/Bricks059_4K-JPG_NormalGL.jpg",
        ]);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].path, "Bricks059.material");
        assert_eq!(m[0].name, "Bricks 059");
        assert_eq!(m[0].maps.len(), 4);
        let same = found(&["tex/rock_diff_1k.jpg", "tex/rock_diff_4k.jpg", "tex/rock_nor_gl_4k.jpg"]);
        assert_eq!(same.len(), 1);
        assert_eq!(same[0].path, "tex/rock.material");
    }

    #[test]
    fn needs_two_kinds_of_map() {
        assert_eq!(found(&["a/Rock_BaseColor.png"]).len(), 0);
        assert_eq!(found(&["a/Rock_Diffuse_1k.png", "a/Rock_Diffuse_2k.png"]).len(), 0);
        assert_eq!(found(&["a/rock_nor_gl.png", "a/rock_nor_dx.png"]).len(), 0);
        // Loose words alone need a colour or normal map: these are just two masks.
        assert_eq!(found(&["ui/panel_mask.png", "ui/panel_glow.png"]).len(), 0);
        assert_eq!(found(&["T_Rock_D.png", "T_Rock_N.png"]).len(), 1);
        assert_eq!(found(&["a/Rock_Roughness.png", "a/Rock_Metallic.png"]).len(), 1);
        // Same name in different folders: two materials.
        assert_eq!(found(&["a/Rock_Color.png", "a/Rock_Normal.png", "b/Rock_Color.png", "b/Rock_Normal.png"]).len(), 2);
    }

    #[test]
    fn names_from_the_folder_when_files_have_none() {
        let m = found(&["Wood Planks/albedo.png", "Wood Planks/normal.png", "Wood Planks/roughness.png"]);
        assert_eq!(m[0].path, "Wood Planks/Wood Planks.material");
        assert_eq!(m[0].name, "Wood planks");
        let m = found(&["T_Rock_D.png", "T_Rock_N.png", "T_Rock_ORM.png"]);
        assert_eq!((m[0].path.as_str(), m[0].name.as_str()), ("T_Rock.material", "Rock"));
    }

    #[test]
    fn never_takes_a_real_files_path() {
        assert_eq!(found(&["Rock_Color.png", "Rock_Normal.png", "Rock.material"]).len(), 0);
    }
}
