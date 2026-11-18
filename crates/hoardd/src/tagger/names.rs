//! Reading a file's name: its words, the asset's name made readable from it, and the texture-map endings
//! (`_Normal`, `_AO`) and lettered series (`Rock_A`, `Rock_B`) it's part of.

use std::collections::HashSet;

use super::match_set;

/// Last-token suffixes that name a PBR map. `true` = unambiguous on its own; `false` = only trusted when
/// sibling textures with other map suffixes share the same base name.
const MAP_SUFFIXES: &[(&str, &str, bool)] = &[
    ("albedo", "albedo", true),
    ("basecolor", "albedo", true),
    ("basecolour", "albedo", true),
    ("diffuse", "albedo", true),
    ("diff", "albedo", true),
    ("alb", "albedo", true),
    ("color", "albedo", false),
    ("colour", "albedo", false),
    ("col", "albedo", false),
    ("d", "albedo", false),
    ("normal", "normal", true),
    ("normalgl", "normal", true),
    ("normaldx", "normal", true),
    ("nrm", "normal", true),
    ("nor", "normal", true),
    ("norm", "normal", true),
    ("n", "normal", false),
    ("roughness", "roughness", true),
    ("rough", "roughness", true),
    ("rgh", "roughness", true),
    ("r", "roughness", false),
    ("metallic", "metallic", true),
    ("metalness", "metallic", true),
    ("metal", "metallic", false),
    ("met", "metallic", false),
    ("m", "metallic", false),
    ("ao", "ao", true),
    ("occlusion", "ao", true),
    ("ambientocclusion", "ao", true),
    ("occ", "ao", false),
    ("height", "height", false),
    ("displacement", "height", true),
    ("disp", "height", true),
    ("bump", "height", false),
    ("h", "height", false),
    ("emissive", "emissive", true),
    ("emission", "emissive", true),
    ("glow", "emissive", false),
    ("e", "emissive", false),
    ("specular", "specular", true),
    ("spec", "specular", true),
    ("gloss", "specular", true),
    ("glossiness", "specular", true),
    ("opacity", "opacity", true),
    ("alpha", "opacity", false),
    ("mask", "opacity", false),
    ("orm", "orm", true),
    ("arm", "orm", false),
    ("rma", "orm", true),
    ("mra", "orm", true),
];

const STOPWORDS: &[&str] = &[
    "the", "and", "of", "a", "an", "for", "with", "by", "in", "on", "to", "at", "final", "copy", "new", "old", "test",
    "untitled", "file", "files", "asset", "assets", "pack", "free", "export", "exports", "version", "ver", "rev",
    "img", "image", "tex", "texture", "textures", "audio", "png", "jpg", "wav", "ogg", "mp3",
];

/// Split a path into lowercase word tokens: `Sci-Fi/FootstepGrass_01.wav` -> `[sci, fi, footstep, grass]`.
/// Splits on punctuation, camelCase and letter->digit boundaries; keeps digit->letter runs (`8bit`, `4k`, `2d`).
pub(crate) fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for piece in text.split(|c: char| !c.is_alphanumeric()) {
        if piece.is_empty() {
            continue;
        }
        let chars: Vec<char> = piece.chars().collect();
        let mut cur = String::new();
        for (i, &c) in chars.iter().enumerate() {
            if i > 0 {
                let prev = chars[i - 1];
                let next_lower = chars.get(i + 1).is_some_and(|n| n.is_lowercase());
                let boundary = (prev.is_lowercase() && c.is_uppercase())
                    || (prev.is_alphabetic() && c.is_ascii_digit())
                    // "HTTPServer" -> "http" + "server"
                    || (prev.is_uppercase() && c.is_uppercase() && next_lower);
                if boundary && !cur.is_empty() {
                    tokens.push(std::mem::take(&mut cur).to_lowercase());
                }
            }
            cur.push(c);
        }
        if !cur.is_empty() {
            tokens.push(cur.to_lowercase());
        }
    }
    tokens
}

pub(super) fn singular(t: &str) -> Option<String> {
    if t.len() > 3 && t.ends_with('s') && !t.ends_with("ss") { Some(t[..t.len() - 1].to_owned()) } else { None }
}

/// Words for the full-text index: all path tokens plus singular forms, deduplicated.
pub(crate) fn fts_words(rel_path: &str) -> String {
    let tokens = tokenize(rel_path);
    let set = match_set(&tokens);
    set.into_iter().filter(|w| w.len() > 1 && !STOPWORDS.contains(&w.as_str())).collect::<Vec<_>>().join(" ")
}

/// Human-friendly display name from a file stem: `footstep_grass_01` -> `Footstep grass 01`.
/// Unreal naming-convention prefixes (`SM_Rock`, `T_Rock_N`, `MI_Rock`), dropped from display names.
const UNREAL_PREFIXES: &[&str] = &[
    "SM_", "SK_", "SKM_", "SKEL_", "T_", "TX_", "M_", "MI_", "MF_", "MM_", "S_", "SW_", "SC_", "MS_", "A_", "AS_",
    "AM_", "ABP_", "BS_", "BP_", "WBP_", "NS_", "NE_", "P_", "PS_", "PM_", "DT_", "E_", "F_", "L_", "LV_",
];

/// The part of a file name to build a display name from: the stem, minus Unreal prefixes for packages.
pub(crate) fn name_stem(rel: &str) -> String {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    let (stem, ext) = file.rsplit_once('.').unwrap_or((file, ""));
    if matches!(ext.to_ascii_lowercase().as_str(), "uasset" | "umap")
        && let Some(rest) = UNREAL_PREFIXES.iter().find_map(|p| stem.strip_prefix(p))
        && !rest.is_empty()
    {
        // Texture suffixes spelled out: T_Rock_N → "Rock normal".
        if stem.starts_with("T_")
            && let Some((base, suffix)) = rest.rsplit_once('_')
            && let Some(word) = UNREAL_TEXTURE_SUFFIXES.iter().find(|(s, _)| *s == suffix).map(|(_, w)| *w)
        {
            return format!("{base}_{word}");
        }
        return rest.to_owned();
    }
    stem.to_owned()
}

/// Unreal texture suffixes and what they mean.
const UNREAL_TEXTURE_SUFFIXES: &[(&str, &str)] = &[
    ("D", "albedo"),
    ("BC", "base color"),
    ("C", "color"),
    ("N", "normal"),
    ("R", "roughness"),
    ("M", "mask"),
    ("E", "emissive"),
    ("H", "height"),
    ("AO", "AO"),
    ("ORM", "ORM"),
    ("ARM", "ARM"),
    ("MRA", "MRA"),
];

/// Abbreviations that read better in capitals in display names: "UI click", "SFX hit", "PBR rock".
const UPPER: &[&str] = &[
    "ui", "hud", "sfx", "vfx", "fx", "pbr", "ao", "lod", "npc", "fps", "rpg", "2d", "3d", "hdr", "hdri", "bgm", "sdf",
    "ik", "fk", "id", "tv", "ufo", "gui", "orm", "arm", "mra",
];

/// What an asset is called when it's found: the title its file gives itself, unless that title says nothing a name
/// could (a track number, `TITLE=1`) or is only the file's name again (`TITLE=bookFlip1`), and otherwise its file
/// name made readable ("Book flip 1").
pub(crate) fn asset_name(title: Option<&str>, rel: &str) -> String {
    let stem = name_stem(rel);
    let file = rel.rsplit('/').next().unwrap_or(rel);
    let said = |t: &&str| {
        let words = tokenize(t);
        t.chars().any(char::is_alphabetic) && words != tokenize(&stem) && words != tokenize(file)
    };
    match title.map(str::trim).filter(said) {
        Some(t) => t.to_owned(),
        None => display_name(&stem),
    }
}

pub(crate) fn display_name(stem: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    for t in tokenize(stem) {
        words.push(t);
    }
    if words.is_empty() {
        return stem.to_owned();
    }
    for w in &mut words {
        if UPPER.contains(&w.as_str()) {
            *w = w.to_uppercase();
        }
    }
    let mut name = words.join(" ");
    if let Some(first) = name.get(0..1) {
        name = first.to_uppercase() + &name[1..];
    }
    name
}

/// A PBR map suffix found on a texture's file stem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapHit {
    /// Stem without the suffix, tokens joined by `_` (`rock_cliff`).
    pub(crate) base: String,
    pub(crate) map: &'static str,
    /// Unambiguous suffix ("normal") vs one that needs sibling maps to be trusted ("n", "metal").
    pub(crate) strong: bool,
    /// The suffix words that matched ("base color").
    pub(crate) suffix: String,
}

/// Whether a texture could be a PBR map: a raster image. A drawing (SVG) is artwork, whatever its name ends in.
pub(crate) fn could_be_map(rel: &str) -> bool {
    let ext = rel.rsplit_once('.').map_or("", |(_, e)| e).to_ascii_lowercase();
    !matches!(ext.as_str(), "svg" | "svgz")
}

/// Names whose one-letter endings are a lettered series of variants (`extra_character_a` … `extra_character_d`), not
/// map types: some file of the series ends in a letter that names no map. Each is `folder/name`, lowercase, as
/// [`series_key`] writes it.
pub(crate) fn lettered_series<'a>(paths: impl IntoIterator<Item = &'a str>) -> HashSet<String> {
    let mut out = HashSet::new();
    for rel in paths {
        let (dir, file) = rel.rsplit_once('/').unwrap_or(("", rel));
        let stem = file.rsplit_once('.').map_or(file, |(s, _)| s);
        let tokens = tokenize(stem);
        if let [base @ .., last] = tokens.as_slice()
            && !base.is_empty()
            && last.len() == 1
            && last.chars().all(|c| c.is_ascii_alphabetic())
            && !MAP_SUFFIXES.iter().any(|(s, _, _)| s == last)
        {
            out.insert(series_key(dir, &base.join("_")));
        }
    }
    out
}

/// A name's key among [`lettered_series`]: its folder and its words before the last, joined by `_`.
pub(crate) fn series_key(dir: &str, base: &str) -> String {
    format!("{}/{base}", dir.to_lowercase())
}

/// Detect a PBR map suffix on a texture stem.
pub(crate) fn map_suffix(stem: &str) -> Option<MapHit> {
    let tokens = tokenize(stem);
    // Skip trailing resolution / variant tokens: "Rock_Normal_2k" / "Rock_Normal_01".
    let mut end = tokens.len();
    while end > 1 {
        let t = &tokens[end - 1];
        let is_res = t.ends_with('k') && t[..t.len() - 1].chars().all(|c| c.is_ascii_digit()) && t.len() <= 3;
        if is_res || t.chars().all(|c| c.is_ascii_digit()) || t == "gl" || t == "dx" {
            end -= 1;
        } else {
            break;
        }
    }
    if end < 2 {
        return None;
    }
    // Two-token suffixes first ("base color", "ambient occlusion").
    let candidates = [
        (end.saturating_sub(2), format!("{}{}", tokens[end - 2], tokens[end - 1])),
        (end - 1, tokens[end - 1].clone()),
    ];
    for (start, suffix) in candidates {
        if start == 0 {
            continue;
        }
        if let Some((_, map, strong)) = MAP_SUFFIXES.iter().find(|(s, _, _)| *s == suffix) {
            return Some(MapHit {
                base: tokens[..start].join("_"),
                map,
                strong: *strong,
                suffix: tokens[start..end].join(" "),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_title_thats_only_a_number_doesnt_name_an_asset() {
        assert_eq!(asset_name(Some("1"), "OGG/footstep00.ogg"), "Footstep 00");
        assert_eq!(asset_name(Some(" 07 "), "OGG/footstep07.ogg"), "Footstep 07");
        assert_eq!(asset_name(None, "OGG/footstep00.ogg"), "Footstep 00");
        assert_eq!(asset_name(Some("Door creak 2"), "door_02.wav"), "Door creak 2");
    }

    #[test]
    fn a_title_thats_only_the_file_name_is_made_readable_like_it() {
        assert_eq!(asset_name(Some("bookFlip1"), "OGG/bookFlip1.ogg"), "Book flip 1");
        assert_eq!(asset_name(Some("book_flip_1.ogg"), "OGG/bookFlip1.ogg"), "Book flip 1");
        assert_eq!(asset_name(Some("Book Flip"), "OGG/bookFlip1.ogg"), "Book Flip");
    }

    #[test]
    fn tokenizes_paths() {
        assert_eq!(tokenize("Sci-Fi/FootstepGrass_01.wav"), ["sci", "fi", "footstep", "grass", "01", "wav"]);
        assert_eq!(tokenize("8bitJump"), ["8bit", "jump"]);
        assert_eq!(tokenize("HTTPServer"), ["http", "server"]);
    }

    #[test]
    fn finds_map_suffixes() {
        let hit = map_suffix("Rock_Normal_2k").unwrap();
        assert_eq!((hit.base.as_str(), hit.map, hit.strong), ("rock", "normal", true));
        let hit = map_suffix("Bricks_base_color").unwrap();
        assert_eq!((hit.base.as_str(), hit.map, hit.suffix.as_str()), ("bricks", "albedo", "base color"));
        assert_eq!(name_stem("Pack/Content/T_Rock_N.uasset"), "Rock_normal");
        assert_eq!(name_stem("Pack/Content/SM_Rock_01.uasset"), "Rock_01");
        assert_eq!(name_stem("Textures/T_Rock_N.png"), "T_Rock_N");
        let hit = map_suffix("Wood_N").unwrap();
        assert_eq!((hit.map, hit.strong), ("normal", false));
        assert_eq!(map_suffix("Metal"), None);
    }
}
