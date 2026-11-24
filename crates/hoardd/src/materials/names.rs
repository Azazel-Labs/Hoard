//! Reading what a texture is for from its name (`Rock_4K_Normal_DX`), and grouping a folder's textures into
//! sets that are one material.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use hoard_core::MaterialLook;

use crate::db;
use crate::tagger;

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

pub(crate) const ALL_ROLES: [Role; 17] = [
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
/// trusted next to a color or normal map with the same name (`T_Rock_D` + `T_Rock_N`), since on its own it's
/// as likely to mean something else.
const ROLE_WORDS: &[(&str, Role, bool)] = &[
    ("base color", Role::BaseColor, false),
    ("base colour", Role::BaseColor, false), // spelling: allow (file names written that way)
    ("basecolor", Role::BaseColor, false),
    ("basecolour", Role::BaseColor, false), // spelling: allow (file names written that way)
    ("base map", Role::BaseColor, false),
    ("albedo transparency", Role::BaseColor, false),
    ("albedo", Role::BaseColor, false),
    ("alb", Role::BaseColor, false),
    ("diffuse", Role::BaseColor, false),
    ("diff", Role::BaseColor, false),
    ("dif", Role::BaseColor, false),
    ("color", Role::BaseColor, false),
    ("colour", Role::BaseColor, false), // spelling: allow (file names written that way)
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
/// A stem's lowercase word tokens, as [`tokens`] splits them, without where they sit.
pub(super) fn words(stem: &str) -> Vec<String> {
    tokens(stem).into_iter().map(|t| t.text).collect()
}

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
    /// Only trusted beside a color or normal map (see [`ROLE_WORDS`]).
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
    loop {
        if end > 0 && is_noise(&toks[end - 1].text) {
            note(&toks[end - 1].text);
            end -= 1;
        } else if end >= 2 && toks[end - 2].text == "lod" && toks[end - 1].text.chars().all(|c| c.is_ascii_digit()) {
            // One normal map per level of detail: `xeokah0_4K_Normal_LOD0`.
            end -= 2;
        } else {
            break;
        }
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
pub(super) fn dir_key(dir: &str) -> String {
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
    /// What the store's JSON beside it says it is, and the asset it's part of (its id), to group by when the file
    /// name doesn't say.
    pub(crate) listed: Option<(Role, &'a str)>,
}

/// A store's word for a kind of map (`albedo`, `displacement`, `ambient occlusion`) as a role.
pub(crate) fn role_of_word(word: &str) -> Option<Role> {
    let words = tokens(word).into_iter().map(|t| t.text).collect::<Vec<_>>().join(" ");
    ROLE_WORDS.iter().find(|(w, _, _)| *w == words).map(|(_, role, _)| *role)
}

/// One material found among a pack's textures.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Found {
    /// The material's own path: `<folder>/<name>.material`.
    pub(crate) path: String,
    pub(crate) name: String,
    /// Lowercase words of the shared name (`rock_wall`), for matching models to it.
    pub(crate) key: String,
    /// (index into the candidates, role, resolution), by role then largest first.
    pub(crate) maps: Vec<(usize, Role, Option<String>)>,
    /// How an engine's material says to draw the maps.
    pub(crate) look: Option<MaterialLook>,
    /// The engine's materials it was read from (a Unity `.mat`, or the `.mtl` files defining it): asset id and path.
    pub(crate) sources: Vec<(String, String)>,
}

pub(super) fn dir_of(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(d, _)| d)
}

pub(super) fn stem_of(path: &str) -> &str {
    let file = path.rsplit('/').next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

/// A readable name for a material: its shared file name, or the folder's when the file name is only an id
/// (Megascans' `vlkhcbxia_8K_Albedo.jpg` in `rock_cliff_vlkhcbxia/`) or there's none (`Rock/albedo.png`).
pub(super) fn material_name(base: &str, dir: &str) -> String {
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

/// Group a pack's textures into materials. A material needs two kinds of map (a color map and a normal map,
/// say); a set named only with one-letter or other loose words also needs a color or normal map among them.
pub(crate) fn find(textures: &[Candidate<'_>]) -> Vec<Found> {
    struct Member {
        idx: usize,
        name: MapName,
    }
    let series = tagger::lettered_series(textures.iter().map(|c| c.path));
    let mut groups: BTreeMap<(String, String), Vec<Member>> = BTreeMap::new();
    for (idx, c) in textures.iter().enumerate() {
        if !tagger::could_be_map(c.path) {
            continue;
        }
        let parsed = parse_map(stem_of(c.path)).filter(|name| {
            // `extra_character_d` beside `extra_character_a`: variant d, not a diffuse map.
            let letter = tokens(stem_of(c.path)).last().is_some_and(|t| t.text.len() == 1);
            !(name.weak && letter && series.contains(&tagger::series_key(dir_of(c.path), &name.key)))
        });
        let name = match (parsed, c.listed) {
            // The store's list beats the file name.
            (Some(name), Some((role, _))) => MapName { role, weak: false, ..name },
            (Some(name), None) => name,
            (None, Some((role, asset))) => {
                MapName { base: asset.to_owned(), key: asset.to_lowercase(), role, weak: false, resolution: None }
            }
            (None, None) => continue,
        };
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
        let maps = maps.into_iter().map(|(i, r, res, _)| (i, r, res)).collect();
        out.push(Found { path, name, key, maps, look: None, sources: Vec::new() });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_normal_map_per_level_of_detail() {
        assert_eq!(role("xeokah0_4K_Normal_LOD0"), r("xeokah0", Role::Normal, Some("4K")));
        assert_eq!(role("Rock_Normal_LOD12"), r("Rock", Role::Normal, None));
        assert_eq!(role("Rock_LOD0"), None);
    }

    #[test]
    fn the_stores_list_says_what_a_map_is() {
        assert_eq!(role_of_word("albedo"), Some(Role::BaseColor));
        assert_eq!(role_of_word("Displacement"), Some(Role::Height));
        assert_eq!(role_of_word("ambientOcclusion"), Some(Role::Ao));
        assert_eq!(role_of_word("fuzz"), None);
        // Named so nothing could tell, listed by the JSON beside them.
        let c = [
            Candidate { path: "S/sbkpa_4K_a.jpg", size_px: None, listed: Some((Role::BaseColor, "sbkpa")) },
            Candidate { path: "S/sbkpa_4K_b.jpg", size_px: None, listed: Some((Role::Normal, "sbkpa")) },
            // The list beats a name that says otherwise.
            Candidate { path: "S/sbkpa_4K_Bump.jpg", size_px: None, listed: Some((Role::Height, "sbkpa")) },
        ];
        let found = find(&c);
        assert_eq!(found.len(), 1, "{found:?}");
        let roles: Vec<Role> = found[0].maps.iter().map(|(_, r, _)| *r).collect();
        assert_eq!(roles, [Role::BaseColor, Role::Normal, Role::Height]);
    }

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
        let c: Vec<Candidate<'_>> = paths.iter().map(|p| Candidate { path: p, size_px: None, listed: None }).collect();
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
        // Loose words alone need a color or normal map: these are just two masks.
        assert_eq!(found(&["ui/panel_mask.png", "ui/panel_glow.png"]).len(), 0);
        assert_eq!(found(&["T_Rock_D.png", "T_Rock_N.png"]).len(), 1);
        // Kenney's lettered variants: d is the fourth of a series, e the fifth, not a diffuse and an emissive map.
        let series =
            ["Special/extra_character_a.png", "Special/extra_character_d.png", "Special/extra_character_e.png"];
        assert_eq!(found(&series).len(), 0);
        // And a drawing is never a map.
        assert_eq!(found(&["Vector/brick_d.svg", "Vector/brick_n.svg"]).len(), 0);
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
