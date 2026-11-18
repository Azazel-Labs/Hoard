//! Tags from what a file's facts say: its music's tempo, key and loops, a font's family, an Unreal asset's class,
//! a texture's size and a vector's, and the artist and title it names.

use super::{TagSet, ucs_from_rules};
use hoard_core::{AssetKind, TagSource};

/// MIDI, tracker modules and chiptunes are music by nature; game audio banks are named for their middleware.
fn music_tags(ext: &str, tags: &mut TagSet) {
    match ext {
        "mod" | "xm" | "s3m" | "it" | "mptm" | "umx" => {
            tags.add("music", TagSource::Meta, 0.9);
            tags.add("tracker-module", TagSource::Meta, 1.0);
        }
        "mid" | "midi" => {
            tags.add("music", TagSource::Meta, 0.8);
            tags.add("midi", TagSource::Meta, 1.0);
        }
        "nsf" | "nsfe" | "vgm" | "vgz" => {
            tags.add("music", TagSource::Meta, 0.9);
            tags.add("chiptune", TagSource::Meta, 1.0);
        }
        "fsb" | "bank" => tags.add("fmod", TagSource::Meta, 1.0),
        "wem" | "bnk" => tags.add("wwise", TagSource::Meta, 1.0),
        _ => {}
    }
}

/// Font tags: style flags, weight class and writing systems beyond Latin.
fn font_tags(meta: &serde_json::Map<String, serde_json::Value>, tags: &mut TagSet) {
    let num = |k: &str| meta.get(k).and_then(serde_json::Value::as_f64);
    let flag = |k: &str| meta.get(k).and_then(serde_json::Value::as_bool).unwrap_or(false);
    for (k, tag) in [("monospace", "monospace"), ("italic", "italic"), ("symbols", "icons")] {
        if flag(k) {
            tags.add(tag, TagSource::Meta, 1.0);
        }
    }
    if meta.contains_key("variable_axes") {
        tags.add("variable-font", TagSource::Meta, 1.0);
    }
    if let Some(w) = num("weight") {
        let name = match w as u32 {
            0..=150 => "thin",
            151..=250 => "extra-light",
            251..=350 => "light",
            351..=450 => "regular",
            451..=550 => "medium",
            551..=650 => "semibold",
            651..=750 => "bold",
            751..=850 => "extra-bold",
            _ => "black",
        };
        tags.add(&format!("weight:{name}"), TagSource::Meta, 1.0);
    }
    for s in meta.get("scripts").and_then(serde_json::Value::as_array).into_iter().flatten() {
        if let Some(s) = s.as_str().filter(|s| *s != "latin") {
            tags.add(&format!("script:{s}"), TagSource::Meta, 1.0);
        }
    }
}

/// Tags for Unreal packages: the class, engine version, and whether a source file came out of it.
fn unreal_tags(meta: &serde_json::Map<String, serde_json::Value>, tags: &mut TagSet) {
    if let Some(class) = meta.get("ue_class").and_then(|v| v.as_str()) {
        tags.add("unreal", TagSource::Meta, 1.0);
        if !class.is_empty() {
            tags.add(&hoard_unreal::class_tag(class), TagSource::Meta, 1.0);
        }
        if let Some(v) = meta.get("ue_engine").and_then(|v| v.as_str()) {
            let short: Vec<&str> = v.split('.').take(2).collect();
            tags.add(&format!("engine:ue{}", short.join(".")), TagSource::Meta, 1.0);
        }
        if let Some(ext) = meta.get("source_ext").and_then(|v| v.as_str()) {
            tags.add(&format!("extracted:{ext}"), TagSource::Meta, 1.0);
        }
        if class.starts_with("Anim") || class.contains("BlendSpace") {
            tags.add("animated", TagSource::Meta, 0.9);
        }
        match class {
            "World" => tags.add("level", TagSource::Meta, 1.0),
            c if c.contains("Blueprint") => tags.add("blueprint", TagSource::Meta, 1.0),
            c if c.starts_with("Niagara") || c == "ParticleSystem" => tags.add("vfx", TagSource::Meta, 1.0),
            _ => {}
        }
    }
}

/// Facts about an image: its map type, color, layers, depth and HDR, and whether it says AI made it.
fn texture_tags(ext: &str, meta: &serde_json::Map<String, serde_json::Value>, tags: &mut TagSet) {
    let num = |k: &str| meta.get(k).and_then(serde_json::Value::as_f64);
    if let (Some(w), Some(h)) = (num("width"), num("height")) {
        let (w, h) = (w as u32, h as u32);
        let max = w.max(h);
        let res = match max {
            0..=64 => "res:tiny",
            65..=256 => "res:256",
            257..=512 => "res:512",
            513..=1024 => "res:1k",
            1025..=2048 => "res:2k",
            2049..=4096 => "res:4k",
            _ => "res:8k+",
        };
        tags.add(res, TagSource::Meta, 1.0);
        if w.is_power_of_two() && h.is_power_of_two() {
            tags.add("pot", TagSource::Meta, 1.0);
        }
        if w == h {
            tags.add("square", TagSource::Meta, 1.0);
        }
        // A long strip of square frames is almost always an animation strip.
        if h > 0 && w % h == 0 && w / h >= 3 && h <= 512 {
            tags.add("spritesheet", TagSource::Rule, 0.6);
            tags.add("frames", TagSource::Meta, 0.6);
        }
    }
    for flag in ["alpha", "tileable", "pixel-art-likely", "normal-map-likely"] {
        if meta.get(flag).and_then(serde_json::Value::as_bool) == Some(true) {
            match flag {
                "pixel-art-likely" => tags.add("style:pixel-art", TagSource::Rule, 0.65),
                "normal-map-likely" => tags.add("map:normal", TagSource::Rule, 0.75),
                "tileable" => tags.add("tileable", TagSource::Rule, 0.7),
                _ => tags.add(flag, TagSource::Meta, 1.0),
            }
        }
    }
    if let Some(c) = meta.get("color").and_then(|v| v.as_str()) {
        tags.add(&format!("color:{c}"), TagSource::Rule, 0.7);
    }
    if num("layers").is_some_and(|n| n > 1.0) {
        tags.add("layered", TagSource::Meta, 1.0);
    }
    if meta.get("hdr").and_then(serde_json::Value::as_bool) == Some(true) {
        tags.add("hdr", TagSource::Meta, 1.0);
    }
    if let Some(d) = num("bit_depth").filter(|d| *d > 8.0) {
        tags.add(&format!("depth:{}-bit", d as u32), TagSource::Meta, 1.0);
    }
    if matches!(ext, "hdr" | "exr") {
        tags.add("hdri", TagSource::Rule, 0.7);
        tags.add("sky", TagSource::Rule, 0.5);
    }
    if let Some(t) = crate::provenance::tag(meta) {
        tags.add(t, TagSource::Meta, 1.0);
    }
}

/// Facts about a drawing: its main color and whether it says AI made it. Pixel facts (resolution, power of two,
/// tiling) don't apply to something that scales to any size.
fn vector_tags(meta: &serde_json::Map<String, serde_json::Value>, tags: &mut TagSet) {
    if let Some(c) = meta.get("color").and_then(|v| v.as_str()) {
        tags.add(&format!("color:{c}"), TagSource::Rule, 0.7);
    }
    if let Some(t) = crate::provenance::tag(meta) {
        tags.add(t, TagSource::Meta, 1.0);
    }
}

/// Metadata-derived facts and heuristics per kind.
/// The tags a sound's measured features (see `sounds::FEATURES`) can bring, besides `loop`.
pub(crate) const MEASURED_SOUND_TAGS: [&str; 9] = [
    "envelope:hit",
    "envelope:swell",
    "envelope:sustained",
    "tone:bright",
    "tone:dark",
    "tone:noisy",
    "tone:tonal",
    "pitch:rising",
    "pitch:falling",
];

/// The kinds of tag a sound's tempo and key bring (`bpm:120`, `key:a-minor`, `bars:8`), whether the file states them
/// or they were measured.
pub(crate) const MUSIC_TAG_PREFIXES: [&str; 3] = ["bpm:", "key:", "bars:"];

/// The tags a sound's measured features bring: its envelope, whether it's bright or dark, noisy or tonal, whether its
/// pitch rises or falls by an octave or more, `loop` (a guess) when it loops cleanly, and its tempo, key and length
/// in bars (see [`music_tags_of`]). `looped`: it's tagged a loop.
pub(crate) fn measured_sound_tags(
    meta: &serde_json::Map<String, serde_json::Value>,
    looped: bool,
) -> Vec<(String, TagSource, f32)> {
    let num = |k: &str| meta.get(k).and_then(serde_json::Value::as_f64);
    let mut out = Vec::new();
    match meta.get("envelope").and_then(|v| v.as_str()) {
        Some("hit") => out.push(MEASURED_SOUND_TAGS[0]),
        Some("swell") => out.push(MEASURED_SOUND_TAGS[1]),
        Some("sustained") => out.push(MEASURED_SOUND_TAGS[2]),
        _ => {}
    }
    match num("brightness_hz") {
        Some(hz) if hz >= 4000.0 => out.push("tone:bright"),
        Some(hz) if hz <= 800.0 => out.push("tone:dark"),
        _ => {}
    }
    match num("noisiness") {
        Some(n) if n >= 0.45 => out.push("tone:noisy"),
        Some(n) if n <= 0.15 => out.push("tone:tonal"),
        _ => {}
    }
    match num("sweep_octaves") {
        Some(o) if o >= 1.0 => out.push("pitch:rising"),
        Some(o) if o <= -1.0 => out.push("pitch:falling"),
        _ => {}
    }
    let mut out: Vec<_> = out.into_iter().map(|t| (t.to_owned(), TagSource::Meta, 1.0)).collect();
    let loops_cleanly = meta.get("loops_cleanly").and_then(serde_json::Value::as_bool) == Some(true);
    if loops_cleanly {
        out.push(("loop".to_owned(), TagSource::Rule, 0.6));
    }
    out.extend(music_tags_of(meta, looped || loops_cleanly));
    out
}

/// The tempo (`bpm:120`) and key (`key:a-minor`) a sound's file states, or else those measured of it (unless it's a
/// one-shot), and how many bars long a loop is at that tempo, in 4/4 (`bars:8`), when that comes out whole.
fn music_tags_of(meta: &serde_json::Map<String, serde_json::Value>, looped: bool) -> Vec<(String, TagSource, f32)> {
    use serde_json::Value;
    let num = |k: &str| meta.get(k).and_then(Value::as_f64);
    let one_shot = meta.get("one_shot").and_then(Value::as_bool) == Some(true);
    // A guess is as sure as the measuring was.
    let guessed = |k: &str| (TagSource::Rule, 0.5 + 0.4 * num(k).unwrap_or(0.0).clamp(0.0, 1.0) as f32);
    let mut out = Vec::new();
    let tempo = match (num("tempo_bpm"), num("measured_bpm")) {
        (Some(bpm), _) => Some((bpm, (TagSource::Meta, 1.0))),
        (None, Some(bpm)) if !one_shot => Some((bpm, guessed("measured_bpm_confidence"))),
        _ => None,
    };
    if let Some((bpm, (source, confidence))) = tempo.filter(|(bpm, _)| *bpm > 0.0) {
        out.push((format!("bpm:{}", bpm.round() as u32), source, confidence));
        let looped =
            looped || meta.contains_key("loop_start") || meta.get("one_shot").and_then(Value::as_bool) == Some(false);
        if let Some(secs) = num("duration_secs").filter(|_| looped) {
            let bars = secs * bpm / 240.0;
            if (0.95..=256.0).contains(&bars) && (bars - bars.round()).abs() <= 0.05 {
                out.push((format!("bars:{}", bars.round() as u32), source, confidence));
            }
        }
    }
    let stated = meta.get("key").and_then(Value::as_str).and_then(crate::rhythm::Key::parse);
    let heard = meta.get("measured_key").and_then(Value::as_str).and_then(crate::rhythm::Key::parse);
    match (stated, heard) {
        (Some(k), _) => out.push((k.tag(), TagSource::Meta, 1.0)),
        (None, Some(k)) if !one_shot => {
            let (source, confidence) = guessed("measured_key_confidence");
            out.push((k.tag(), source, confidence));
        }
        _ => {}
    }
    out
}

pub(crate) fn meta_tags(
    kind: AssetKind,
    ext: &str,
    meta: &serde_json::Map<String, serde_json::Value>,
    tags: &mut TagSet,
) {
    tags.add(&format!("format:{ext}"), TagSource::Meta, 1.0);
    // Who made it, as the file says (a sound's artist tag), so everything by them is a click away.
    if let Some(a) = meta.get("artist").and_then(|v| v.as_str()) {
        tags.add(&format!("artist:{a}"), TagSource::Meta, 1.0);
    }
    music_tags(ext, tags);
    let num = |k: &str| meta.get(k).and_then(serde_json::Value::as_f64);
    unreal_tags(meta, tags);
    match kind {
        AssetKind::Font => font_tags(meta, tags),
        AssetKind::Audio => {
            if let Some(d) = num("duration_secs") {
                let bucket = match d {
                    d if d < 2.0 => "duration:short",
                    d if d < 15.0 => "duration:medium",
                    d if d < 120.0 => "duration:long",
                    _ => "duration:very-long",
                };
                tags.add(bucket, TagSource::Meta, 1.0);
                let musicish = tags.has("music") || tags.has("ambience") || tags.has("voice");
                if d < 8.0 && !musicish {
                    tags.add("sfx", TagSource::Rule, 0.7);
                } else if d > 45.0 && !musicish && !tags.has("sfx") {
                    tags.add("music", TagSource::Rule, 0.55);
                }
            }
            match num("channels").map(|c| c as u32) {
                Some(1) => tags.add("mono", TagSource::Meta, 1.0),
                Some(2) => tags.add("stereo", TagSource::Meta, 1.0),
                Some(n) if n > 2 => tags.add("surround", TagSource::Meta, 1.0),
                _ => {}
            }
            if let Some(g) = meta.get("genre").and_then(|v| v.as_str()) {
                tags.add(&format!("genre:{g}"), TagSource::Meta, 0.9);
            }
            // Facts embedded in WAV chunks (sampler loops, ACID, keywords, UCS from library tools).
            if meta.contains_key("loop_start")
                || meta.get("one_shot").and_then(serde_json::Value::as_bool) == Some(false)
            {
                tags.add("loop", TagSource::Meta, 1.0);
            }
            if meta.get("one_shot").and_then(serde_json::Value::as_bool) == Some(true) {
                tags.add("one-shot", TagSource::Meta, 1.0);
            }
            if let Some(k) = meta.get("root_note").and_then(|v| v.as_str()) {
                tags.add(&format!("root:{k}"), TagSource::Meta, 1.0);
            }
            for k in meta.get("keywords").and_then(|v| v.as_array()).into_iter().flatten().take(24) {
                if let Some(k) = k.as_str() {
                    tags.add(k, TagSource::Meta, 0.9);
                }
            }
            if let Some(c) = meta.get("ucs_category").and_then(|v| v.as_str()) {
                tags.add(&format!("ucs:{c}"), TagSource::Meta, 1.0);
                if let Some(s) = meta.get("ucs_subcategory").and_then(|v| v.as_str()) {
                    tags.add(&format!("ucs:{c}/{s}"), TagSource::Meta, 1.0);
                }
            }
            if let Some(rms) = num("rms_dbfs") {
                if rms < -36.0 {
                    tags.add("level:quiet", TagSource::Meta, 1.0);
                } else if rms > -12.0 {
                    tags.add("level:loud", TagSource::Meta, 1.0);
                }
            }
            if meta.get("clipped").and_then(serde_json::Value::as_bool) == Some(true) {
                tags.add("clipped", TagSource::Meta, 1.0);
            }
            for (tag, source, confidence) in measured_sound_tags(meta, tags.has("loop")) {
                tags.add(&tag, source, confidence);
            }
            ucs_from_rules(tags);
        }
        AssetKind::Texture => texture_tags(ext, meta, tags),
        AssetKind::Vector => vector_tags(meta, tags),
        // A DragonBones or Spine skeleton: a rigged 2D character.
        AssetKind::Animation if meta.contains_key("dragonbones") || meta.contains_key("spine") => {
            tags.add(if meta.contains_key("spine") { "spine" } else { "dragonbones" }, TagSource::Meta, 1.0);
            tags.add("rigged", TagSource::Meta, 0.9);
            tags.add("2d", TagSource::Meta, 0.9);
        }
        AssetKind::Model => {
            if let Some(v) = meta.get("blender_version").and_then(|v| v.as_str()) {
                tags.add(&format!("blender:{v}"), TagSource::Meta, 1.0);
            }
            // What the file says beats what its folders suggest: a hat in "Animated Characters Bundle/Accessories"
            // isn't animated. Files that don't say (no metadata for these keys) keep the guess.
            let says = |count: &str, flag: &str| {
                let (n, f) = (num(count), meta.get(flag).and_then(serde_json::Value::as_bool));
                (n.is_some() || f.is_some()).then(|| n.unwrap_or(0.0) > 0.0 || f == Some(true))
            };
            for (tag, yes) in
                [("rigged", says("skins", "has_deformers")), ("animated", says("animations", "has_animation"))]
            {
                match yes {
                    Some(true) => tags.add(tag, TagSource::Meta, 0.9),
                    Some(false) => tags.drop_guess(tag),
                    None => {}
                }
            }
            if let Some(tris) = num("triangles") {
                let bucket = match tris as u64 {
                    0..=2_000 => "poly:low",
                    2_001..=20_000 => "poly:mid",
                    20_001..=200_000 => "poly:high",
                    _ => "poly:very-high",
                };
                tags.add(bucket, TagSource::Meta, 1.0);
                if tris < 1500.0 {
                    tags.add("style:low-poly", TagSource::Rule, 0.5);
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tagger::vocab_tags;

    #[test]
    fn the_artist_is_a_tag() {
        let mut t = TagSet::default();
        let meta = serde_json::json!({ "artist": "Kenney Vleugels" });
        meta_tags(AssetKind::Audio, "ogg", meta.as_object().unwrap(), &mut t);
        assert!(t.has("artist:kenney-vleugels"));
    }

    #[test]
    fn a_model_that_says_it_has_no_animation_isnt_animated_by_its_folder() {
        let tagged = |path: &str, meta: serde_json::Value| {
            let mut t = TagSet::default();
            vocab_tags(path, AssetKind::Model, &mut t);
            meta_tags(AssetKind::Model, "fbx", meta.as_object().unwrap(), &mut t);
            (t.has("animated"), t.has("rigged"))
        };
        let hat = serde_json::json!({ "has_animation": false, "has_deformers": false });
        assert_eq!(tagged("Animated Characters Bundle/Accessories/cap.fbx", hat), (false, false));
        let body = serde_json::json!({ "has_animation": false, "has_deformers": true });
        assert_eq!(tagged("Animated Characters Bundle/Models/characterMedium.fbx", body), (false, true));
        let clip = serde_json::json!({ "animations": 2, "skins": 0 });
        assert_eq!(tagged("Rigged/door.glb", clip), (true, false));
        // A file that says nothing about it keeps the guess.
        assert_eq!(tagged("Animated Characters Bundle/Source/accessories.blend", serde_json::json!({})), (true, false));
    }
}
