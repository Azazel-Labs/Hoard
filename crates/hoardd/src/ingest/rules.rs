//! What holds for a whole pack: its license, the sets of texture maps that are one material, and what Unity's
//! `.meta` files say about its files.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use hoard_core::{AssetKind, TagSource};

use super::unpack::{is_sidecar, rel_of};
use super::{Item, ext_of, stem_of};
use crate::tagger::{self, TagSet};

const LICENSE_SCAN_BYTES: u64 = 256 << 10;

/// The GUID in a Unity `.meta` file (`guid: 0123…`, 32 hex digits).
pub(crate) fn unity_guid(text: &str) -> Option<&str> {
    let line = text.lines().take(8).find_map(|l| l.trim().strip_prefix("guid:"))?.trim();
    (line.len() == 32 && line.bytes().all(|b| b.is_ascii_hexdigit())).then_some(line)
}

/// The materials a Unity model importer puts in its model's material slots (`externalObjects`): each slot's name and
/// the GUID of the `.mat` it wears.
pub(crate) fn unity_remaps(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let (mut inside, mut material, mut name) = (false, false, None::<String>);
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("externalObjects:") {
            inside = !t.ends_with("{}");
            continue;
        }
        if !inside {
            continue;
        }
        // The list ends where a key no deeper than `externalObjects` starts.
        if !line.starts_with("  -") && !line.starts_with("   ") && !t.is_empty() {
            break;
        }
        if t.starts_with("- first:") {
            (material, name) = (false, None);
        } else if let Some(ty) = t.strip_prefix("type:") {
            material |= ty.trim() == "UnityEngine:Material";
        } else if let Some(n) = t.strip_prefix("name:") {
            name = Some(n.trim().trim_matches(['\'', '"']).to_owned());
        } else if let Some(second) = t.strip_prefix("second:")
            && material
            && let Some(n) = name.take().filter(|n| !n.is_empty())
            && let Some(g) = second.split_once("guid:").map(|(_, g)| g.trim_start())
            && let Some(g) = g.get(..32).filter(|g| g.bytes().all(|b| b.is_ascii_hexdigit()))
        {
            out.push((n, g.to_ascii_lowercase()));
        }
    }
    out
}

/// Unity's facts about the pack's files, from the `.meta` files beside them.
#[derive(Default)]
pub(super) struct UnityMeta {
    /// `(path relative to root, guid)`.
    pub(super) guids: Vec<(String, String)>,
    /// `(model's path, material slot, the .mat's guid)`.
    pub(super) remaps: Vec<(String, String, String)>,
}

pub(super) fn harvest_unity_meta(root: &Path, files: &[PathBuf]) -> UnityMeta {
    let mut out = UnityMeta::default();
    for f in files {
        let rel = rel_of(root, f);
        let Some(target) = rel.strip_suffix(".meta").filter(|_| is_sidecar(&rel)) else { continue };
        let Ok(text) = fs::read_to_string(f) else { continue };
        if let Some(g) = unity_guid(&text) {
            out.guids.push((target.to_owned(), g.to_ascii_lowercase()));
        }
        for (slot, guid) in unity_remaps(&text) {
            out.remaps.push((target.to_owned(), slot, guid));
        }
    }
    out
}

/// Which textures form PBR map sets (`Rock_Albedo` + `Rock_Normal` + ...), worked out from paths alone before
/// any file is read, so a pack can be tagged a batch at a time and never held in memory whole.
pub(crate) struct MapSets(HashMap<String, (tagger::MapHit, bool)>);

/// Whether a file might be a texture, judging by its path: an image, or an Unreal package (whose class is only
/// known once it's read).
pub(super) fn could_be_texture(rel: &str) -> bool {
    let ext = ext_of(rel);
    ext == "uasset" || AssetKind::from_ext(&ext) == AssetKind::Texture
}

impl MapSets {
    pub(crate) fn new<'a>(texture_paths: impl IntoIterator<Item = &'a str>) -> Self {
        let paths: Vec<&str> = texture_paths.into_iter().filter(|rel| tagger::could_be_map(rel)).collect();
        let series = tagger::lettered_series(paths.iter().copied());
        let mut groups: HashMap<(&str, String), Vec<(&str, tagger::MapHit)>> = HashMap::new();
        for rel in paths {
            if let Some(hit) = tagger::map_suffix(stem_of(rel)) {
                let dir = rel.rsplit_once('/').map_or("", |(d, _)| d);
                if hit.suffix.len() == 1 && series.contains(&tagger::series_key(dir, &hit.base)) {
                    continue;
                }
                groups.entry((dir, hit.base.clone())).or_default().push((rel, hit));
            }
        }
        let mut out = HashMap::new();
        for members in groups.into_values() {
            let distinct: std::collections::BTreeSet<&str> = members.iter().map(|(_, h)| h.map).collect();
            let is_set = distinct.len() >= 2;
            for (rel, hit) in members {
                out.insert(rel.to_owned(), (hit, is_set));
            }
        }
        Self(out)
    }
}

/// Rules that need the rest of the pack: PBR map sets, the pack's license, then the category.
pub(crate) fn apply_pack_rules(it: &mut Item, sets: &MapSets, license: Option<&str>) {
    if it.kind == AssetKind::Texture
        && let Some((hit, is_set)) = sets.0.get(&it.rel)
    {
        let is_set = *is_set;
        // Unreal's T_Name_N convention makes one-letter suffixes trustworthy on their own.
        let unreal_texture = it.ext == "uasset" && stem_of(&it.rel).starts_with("T_");
        if is_set || hit.strong || unreal_texture {
            it.tags.add(&format!("map:{}", hit.map), TagSource::Rule, if is_set { 0.95 } else { 0.8 });
            if is_set {
                it.tags.add(&format!("set:{}", hit.base), TagSource::Rule, 0.9);
                it.tags.add("pbr", TagSource::Rule, 0.85);
            }
            // "Rock_Metallic.png" is a map, not a metal rock: drop vocab tags that only came from the suffix.
            let dir = it.rel.rsplit_once('/').map_or("", |(d, _)| d);
            let mut from_suffix = TagSet::default();
            tagger::vocab_tags(&hit.suffix, it.kind, &mut from_suffix);
            let mut from_base = TagSet::default();
            tagger::vocab_tags(&format!("{dir}/{}", hit.base), it.kind, &mut from_base);
            for s in from_suffix.into_vec() {
                if !from_base.has(&s.tag) {
                    it.tags.remove(&s.tag);
                }
            }
        }
    }
    let data_map = [
        "map:normal",
        "map:roughness",
        "map:metallic",
        "map:ao",
        "map:height",
        "map:orm",
        "map:specular",
        "map:opacity",
    ]
    .iter()
    .any(|m| it.tags.has(m));
    if data_map {
        it.tags.remove_prefix("color:");
    }
    if let Some(l) = license {
        it.tags.add(&format!("license:{l}"), TagSource::Rule, 0.9);
    }
    it.category = tagger::categorize(it.kind, &it.tags);
}

pub(super) fn detect_pack_license(root: &Path, files: &[PathBuf]) -> Option<&'static str> {
    license_in(files.iter().filter(|f| tagger::is_license_file(&rel_of(root, f))))
}

/// The first license recognized in these files (only their first few KB are read).
pub(super) fn license_in<'a>(files: impl IntoIterator<Item = &'a PathBuf>) -> Option<&'static str> {
    files.into_iter().find_map(|f| {
        let mut text = String::new();
        File::open(f).ok()?.take(LICENSE_SCAN_BYTES).read_to_string(&mut text).ok()?;
        tagger::detect_license(&text)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lettered_variants_and_drawings_are_not_map_sets() {
        let paths = [
            "Brick Pack/PNG/Special/extra_character_a.png",
            "Brick Pack/PNG/Special/extra_character_d.png",
            "Brick Pack/PNG/Special/extra_character_e.png",
            "Brick Pack/Vector/rock_d.svg",
            "Brick Pack/Vector/rock_n.svg",
            "Textures/T_Rock_D.png",
            "Textures/T_Rock_N.png",
        ];
        let sets = MapSets::new(paths);
        let tagged: Vec<&str> = paths.into_iter().filter(|p| sets.0.contains_key(*p)).collect();
        assert_eq!(tagged, ["Textures/T_Rock_D.png", "Textures/T_Rock_N.png"]);
    }

    #[test]
    fn a_model_importers_material_slots() {
        let meta = "fileFormatVersion: 2
guid: 0f8c1d2e3a4b5c6d7e8f901a2b3c4d5e
ModelImporter:
  serializedVersion: 19301
  internalIDToNameTable: []
  externalObjects:
  - first:
      type: UnityEngine:Material
      assembly: UnityEngine.CoreModule
      name: lambert2
    second: {fileID: 2100000, guid: 6E0D5F4D4A0B6A64A8B0D3E9E6B9C2A1, type: 2}
  - first:
      type: UnityEngine:AnimationClip
      assembly: UnityEngine.AnimationModule
      name: Take 001
    second: {fileID: 7400000, guid: 11111111111111111111111111111111, type: 2}
  - first:
      type: UnityEngine:Material
      assembly: UnityEngine.CoreModule
      name: 'Glass #2'
    second: {fileID: 2100000, guid: 22222222222222222222222222222222, type: 2}
  materials:
    materialImportMode: 1
    name: notaslot
    second: {fileID: 2100000, guid: 33333333333333333333333333333333, type: 2}
";
        assert_eq!(
            unity_remaps(meta),
            [
                ("lambert2".to_owned(), "6e0d5f4d4a0b6a64a8b0d3e9e6b9c2a1".to_owned()),
                ("Glass #2".to_owned(), "22222222222222222222222222222222".to_owned()),
            ]
        );
        assert_eq!(unity_remaps("ModelImporter:\n  externalObjects: {}\n  materials:\n").len(), 0);
    }
}
