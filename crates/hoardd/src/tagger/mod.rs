//! The offline, conventional tagger. No network, no model weights: filename vocabulary,
//! metadata-derived facts, and a few cheap signal-processing heuristics (see `analyze`).
//!
//! It is intentionally deterministic so re-running it (`POST /api/packs/{id}/retag`) after improving the
//! vocabulary gives predictable results. The LLM pass layers on top; it never replaces these tags.

mod meta;
mod names;
mod vocab;

use std::collections::{BTreeSet, HashMap};

use hoard_core::{AssetKind, TagSource};
pub(crate) use meta::{MEASURED_SOUND_TAGS, MUSIC_TAG_PREFIXES, measured_sound_tags, meta_tags};
use names::singular;
pub(crate) use names::{
    MapHit, asset_name, could_be_map, display_name, fts_words, lettered_series, map_suffix, name_stem, series_key,
    tokenize,
};
pub(crate) use vocab::CATEGORIES;
use vocab::{AUDIO_VOCAB, VOCAB};

/// A suggested tag with provenance.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Suggestion {
    pub(crate) tag: String,
    pub(crate) source: TagSource,
    pub(crate) confidence: f32,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct TagSet {
    inner: HashMap<String, (TagSource, f32)>,
}

impl TagSet {
    pub(crate) fn add(&mut self, tag: &str, source: TagSource, confidence: f32) {
        let Some(tag) = hoard_core::normalize_tag(tag) else { return };
        let entry = self.inner.entry(tag).or_insert((source, confidence));
        if confidence > entry.1 {
            *entry = (source, confidence);
        }
    }
    pub(crate) fn has(&self, tag: &str) -> bool {
        self.inner.contains_key(tag)
    }
    pub(crate) fn has_prefix(&self, prefix: &str) -> bool {
        self.inner.keys().any(|k| k.starts_with(prefix))
    }
    pub(crate) fn remove(&mut self, tag: &str) {
        self.inner.remove(tag);
    }
    /// Takes back a tag guessed from the path when the file itself says otherwise.
    fn drop_guess(&mut self, tag: &str) {
        if self.inner.get(tag).is_some_and(|(s, _)| *s == TagSource::Rule) {
            self.inner.remove(tag);
        }
    }
    pub(crate) fn remove_prefix(&mut self, prefix: &str) {
        self.inner.retain(|k, _| !k.starts_with(prefix));
    }
    pub(crate) fn into_vec(self) -> Vec<Suggestion> {
        let mut v: Vec<Suggestion> =
            self.inner.into_iter().map(|(tag, (source, confidence))| Suggestion { tag, source, confidence }).collect();
        v.sort_by(|a, b| a.tag.cmp(&b.tag));
        v
    }
}

/// Token set used for vocabulary matching: tokens, singulars, and adjacent joins ("low"+"poly").
fn match_set(tokens: &[String]) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    for (i, t) in tokens.iter().enumerate() {
        if t.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        set.insert(t.clone());
        if let Some(s) = singular(t) {
            set.insert(s);
        }
        if let Some(next) = tokens.get(i + 1) {
            set.insert(format!("{t}{next}"));
        }
    }
    set
}

/// Apply filename vocabulary. Confidence is lower for matches that came only from parent folders,
/// because folder names describe the group ("Sounds/") more loosely than the file name does.
pub(crate) fn vocab_tags(rel_path: &str, kind: AssetKind, tags: &mut TagSet) {
    let (dirs, file) = rel_path.rsplit_once('/').unwrap_or(("", rel_path));
    let stem = file.rsplit_once('.').map_or(file, |(s, _)| s);
    let file_set = match_set(&tokenize(stem));
    let dir_set = match_set(&tokenize(dirs));
    let audio: &[(&[&str], &[&str])] = if kind == AssetKind::Audio { AUDIO_VOCAB } else { &[] };
    for (keywords, out) in VOCAB.iter().chain(audio) {
        let in_file = keywords.iter().any(|k| file_set.contains(*k));
        let in_dir = !in_file && keywords.iter().any(|k| dir_set.contains(*k));
        if in_file || in_dir {
            for t in *out {
                tags.add(t, TagSource::Rule, if in_file { 0.8 } else { 0.6 });
            }
        }
    }
}

/// Words a pack puts in brackets in a folder or file name, `Audio (Female)/level_up.ogg` or `Door (Wood).png`, are
/// a fact about the file: each becomes a tag. A copy number (`(1)`), a year, "copy" and anything longer than a few
/// words are left out.
pub(crate) fn aside_tags(rel_path: &str, tags: &mut TagSet) {
    let path = rel_path.rsplit_once('.').map_or(rel_path, |(s, _)| s);
    for (open, close) in [('(', ')'), ('[', ']')] {
        let mut rest = path;
        while let Some(start) = rest.find(open) {
            let after = &rest[start + open.len_utf8()..];
            let Some(end) = after.find(close) else { break };
            let inner = after[..end].trim();
            rest = &after[end + close.len_utf8()..];
            let words = inner.split_whitespace().count();
            let letters = inner.chars().any(char::is_alphabetic);
            if inner.contains('/') || !(1..=3).contains(&words) || !letters || inner.chars().count() > 32 {
                continue;
            }
            if matches!(inner.to_lowercase().as_str(), "copy" | "old" | "new" | "final" | "backup" | "unused") {
                continue;
            }
            tags.add(inner, TagSource::Rule, 0.8);
        }
    }
}

/// Top-level Universal Category System (UCS) categories for sounds, from rule tags. UCS is the public-domain SFX
/// category list used by most commercial libraries; only unambiguous mappings are made here, the LLM (or metadata
/// embedded by library tools) supplies subcategories.
const UCS_MAP: &[(&str, &str)] = &[
    ("explosion", "explosions"),
    ("footsteps", "footsteps"),
    ("whoosh", "swooshes"),
    ("magic", "magic"),
    ("creature", "creatures"),
    ("ui", "user-interface"),
    ("ambience", "ambience"),
    ("voice", "voices"),
    ("vehicle", "vehicles"),
    ("music", "musical"),
];

fn ucs_from_rules(tags: &mut TagSet) {
    if tags.has_prefix("ucs:") {
        return;
    }
    if let Some((_, cat)) = UCS_MAP.iter().find(|(t, _)| tags.has(t)) {
        tags.add(&format!("ucs:{cat}"), TagSource::Rule, 0.6);
    }
}

/// Pick a category from the final tag set. First match wins, so order is priority.
pub(crate) fn categorize(kind: AssetKind, tags: &TagSet) -> String {
    let pick = |rules: &[(&str, &str)], default: &str| -> String {
        rules
            .iter()
            .find(|(t, _)| tags.has(t) || tags.has_prefix(&format!("{t}:")))
            .map_or(default, |(_, c)| c)
            .to_owned()
    };
    match kind {
        AssetKind::Audio => pick(
            &[
                ("voice", "voice"),
                ("music", "music"),
                ("ambience", "ambience"),
                ("ui", "sfx/ui"),
                ("explosion", "sfx/explosion"),
                ("weapon", "sfx/weapon"),
                ("footsteps", "sfx/footsteps"),
                ("melee", "sfx/impact"),
                ("impact", "sfx/impact"),
                ("magic", "sfx/magic"),
                ("creature", "sfx/creature"),
                ("pickup", "sfx/pickup"),
                ("movement", "sfx/movement"),
                ("vehicle", "sfx/vehicle"),
                ("foley", "sfx/foley"),
                ("nature", "sfx/nature"),
            ],
            "sfx",
        ),
        AssetKind::Texture => pick(
            &[
                ("icon", "ui/icons"),
                ("ui", "ui"),
                ("map", "textures/pbr"),
                ("tileset", "tilesets"),
                ("character", "sprites/character"),
                ("spritesheet", "sprites"),
                ("sprite", "sprites"),
                ("sky", "textures/sky"),
                ("vfx", "vfx"),
                ("decal", "textures/decal"),
                ("material", "textures/surface"),
                ("style:pixel-art", "sprites"),
            ],
            "textures",
        ),
        AssetKind::Vector => pick(&[("icon", "ui/icons"), ("ui", "ui"), ("character", "vector/characters")], "vector"),
        AssetKind::Sheet => pick(
            &[("icon", "ui/icons"), ("ui", "ui"), ("tileset", "tilesets"), ("character", "sprites/character")],
            "sprites",
        ),
        AssetKind::Model => pick(
            &[
                ("character", "models/characters"),
                ("vehicle", "models/vehicles"),
                ("weapon", "models/weapons"),
                ("architecture", "models/architecture"),
                ("nature", "models/nature"),
                ("prop", "models/props"),
            ],
            "models",
        ),
        AssetKind::Animation => "animations".to_owned(),
        AssetKind::Font => "fonts".to_owned(),
        AssetKind::Shader => "shaders".to_owned(),
        AssetKind::Material => "materials".to_owned(),
        AssetKind::Video => "video".to_owned(),
        AssetKind::Document => "docs".to_owned(),
        AssetKind::Data => "data".to_owned(),
        AssetKind::Other => "other".to_owned(),
    }
}

/// Spot the license of a pack from its LICENSE / README text. Returns a short SPDX-ish id.
pub(crate) fn detect_license(text: &str) -> Option<&'static str> {
    let t = text.to_lowercase();
    let has = |needle: &str| t.contains(needle);
    if has("cc0") || has("creative commons zero") || has("publicdomain/zero") || has("public domain dedication") {
        Some("cc0")
    } else if has("licenses/by-nc") || has("noncommercial") || has("non-commercial") {
        Some("cc-by-nc")
    } else if has("licenses/by-sa") || has("attribution-sharealike") || has("attribution share alike") {
        Some("cc-by-sa")
    } else if has("licenses/by/")
        || has("creative commons attribution")
        || has("cc-by")
        || has("cc by 4")
        || has("cc by 3")
    {
        Some("cc-by")
    } else if has("open font license") || has("sil ofl") {
        Some("ofl")
    } else if has("mit license") || has("permission is hereby granted, free of charge") {
        Some("mit")
    } else if has("gnu general public license") {
        Some("gpl")
    } else if has("public domain") {
        Some("public-domain")
    } else if has("royalty free") || has("royalty-free") {
        Some("royalty-free")
    } else {
        None
    }
}

pub(crate) fn is_license_file(rel_path: &str) -> bool {
    let file = rel_path.rsplit('/').next().unwrap_or(rel_path).to_lowercase();
    ["license", "licence", "copying", "readme", "credits", "terms", "eula"].iter().any(|p| file.starts_with(p)) // spelling: allow (file names written that way)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vocab_and_category() {
        let mut t = TagSet::default();
        vocab_tags("Sounds/Impacts/explosion_big_03.ogg", AssetKind::Audio, &mut t);
        assert!(t.has("explosion"));
        assert_eq!(categorize(AssetKind::Audio, &t), "sfx/explosion");
    }

    #[test]
    fn bracketed_words_are_tags() {
        const NONE: [&str; 0] = [];
        let tags = |p: &str| {
            let mut t = TagSet::default();
            aside_tags(p, &mut t);
            t.into_vec().into_iter().map(|s| s.tag).collect::<Vec<_>>()
        };
        assert_eq!(tags("Audio/Voiceover Pack/Audio (Female)/level_up.ogg"), ["female"]);
        assert_eq!(tags("Doors/door (Old Wood) [Hi Res].png"), ["hi-res", "old-wood"]);
        assert_eq!(tags("rock (1).png"), NONE, "a copy number");
        assert_eq!(tags("Music (2019)/theme (copy).ogg"), NONE, "a year and a copy");
        assert_eq!(tags("Read me (this explains every file in the pack).txt"), NONE, "a sentence");
        assert_eq!(tags("weird (name.png"), NONE);
    }

    #[test]
    fn licenses() {
        assert_eq!(detect_license("License: (CC0 1.0 Universal)"), Some("cc0"));
        assert_eq!(detect_license("https://creativecommons.org/licenses/by/4.0/"), Some("cc-by"));
    }
}
