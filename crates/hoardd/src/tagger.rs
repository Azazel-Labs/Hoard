//! The offline, conventional tagger. No network, no model weights: filename vocabulary,
//! metadata-derived facts, and a few cheap signal-processing heuristics (see `analyze`).
//!
//! It is intentionally deterministic so re-running it (`POST /api/packs/{id}/retag`) after improving the
//! vocabulary gives predictable results. The LLM pass layers on top; it never replaces these tags.

use std::collections::{BTreeSet, HashMap, HashSet};

use hoard_core::{AssetKind, TagSource};

/// Canonical category tree, shared with the LLM prompt so both taggers speak the same language.
pub(crate) const CATEGORIES: &[&str] = &[
    "music",
    "ambience",
    "voice",
    "sfx",
    "sfx/ui",
    "sfx/explosion",
    "sfx/weapon",
    "sfx/footsteps",
    "sfx/impact",
    "sfx/magic",
    "sfx/creature",
    "sfx/pickup",
    "sfx/movement",
    "sfx/foley",
    "sfx/vehicle",
    "sfx/nature",
    "textures",
    "textures/pbr",
    "textures/surface",
    "textures/sky",
    "textures/decal",
    "sprites",
    "sprites/character",
    "tilesets",
    "ui",
    "ui/icons",
    "vfx",
    "models",
    "models/characters",
    "models/vehicles",
    "models/weapons",
    "models/architecture",
    "models/props",
    "models/nature",
    "animations",
    "fonts",
    "shaders",
    "materials",
    "video",
    "docs",
    "data",
];

/// Vocabulary that only makes sense for sound ("run" is footsteps in a .wav, an animation in a .png).
const AUDIO_VOCAB: &[(&[&str], &[&str])] = &[
    (&["footstep", "footsteps", "step", "steps", "walk", "walking", "run", "running", "sprint"], &["footsteps"]),
    (
        &[
            "gun", "gunshot", "shot", "shoot", "pistol", "rifle", "shotgun", "smg", "reload", "laser", "blaster",
            "bullet",
        ],
        &["weapon"],
    ),
    (&["sword", "slash", "blade", "parry", "axe", "dagger"], &["melee"]),
    (&["whoosh", "swoosh", "swish", "woosh", "swing", "pass", "passby"], &["whoosh"]),
    (&["hit", "punch", "kick", "impact", "thud", "smash", "crash", "bump", "slam"], &["impact"]),
    (
        &[
            "click",
            "hover",
            "button",
            "select",
            "confirm",
            "cancel",
            "back",
            "toggle",
            "menu",
            "notification",
            "beep",
            "blip",
            "tick",
        ],
        &["ui"],
    ),
    (&["jump", "land", "landing", "dash", "slide", "roll"], &["movement"]),
    (&["door", "creak", "lock", "unlock", "drawer", "cloth", "paper", "glass", "drop", "pour"], &["foley"]),
    (&["engine", "car", "motor", "vehicle", "truck", "tire", "horn", "spaceship", "thruster"], &["vehicle"]),
    (&["ambience", "ambient", "atmosphere", "atmos", "roomtone", "drone"], &["ambience"]),
    (
        &[
            "wind", "rain", "thunder", "storm", "forest", "birds", "ocean", "waves", "river", "stream", "crickets",
            "fire", "campfire",
        ],
        &["nature"],
    ),
    (
        &["voice", "vo", "dialog", "dialogue", "grunt", "scream", "shout", "laugh", "breath", "speech", "announcer"],
        &["voice"],
    ),
    (
        &["music", "theme", "track", "bgm", "ost", "song", "soundtrack", "score", "melody", "jingle", "stinger"],
        &["music"],
    ),
    (&["battle", "boss", "combat", "fight"], &["combat"]),
    (&["victory", "win", "gameover", "defeat", "levelup", "complete"], &["game-state"]),
    (&["loop", "loopable", "looping", "looped", "seamless"], &["loop"]),
    (&["sfx", "fx", "soundfx", "sound"], &["sfx"]),
    (&["chiptune", "8bit", "chip", "nes", "gameboy", "sid"], &["chiptune", "style:retro"]),
    (&["orchestral", "orchestra", "strings", "cinematic", "epic"], &["genre:orchestral"]),
    (&["synthwave", "synth", "electronic", "edm", "techno"], &["genre:electronic"]),
    (&["lofi", "chill", "calm", "relaxing", "peaceful"], &["mood:calm"]),
    (&["dark", "horror", "creepy", "scary", "spooky", "tense", "suspense"], &["mood:dark"]),
    (&["happy", "upbeat", "cheerful", "fun", "bouncy"], &["mood:upbeat"]),
];

/// keyword(s) -> tags. Keywords are matched against path tokens (and adjacent-token joins like "lowpoly").
const VOCAB: &[(&[&str], &[&str])] = &[
    // --- subjects that apply to any kind
    (&["explosion", "explode", "boom", "blast", "detonation", "detonate", "grenade", "kaboom"], &["explosion"]),
    (&["coin", "pickup", "collect", "gem", "powerup", "bonus", "reward", "chest", "loot"], &["pickup"]),
    (&["magic", "spell", "cast", "heal", "buff", "debuff", "arcane", "teleport", "portal", "enchant"], &["magic"]),
    (&["monster", "creature", "growl", "roar", "zombie", "beast", "dragon", "goblin", "orc", "slime"], &["creature"]),
    // --- surfaces / materials
    (&["grass", "lawn", "turf"], &["material:grass"]),
    (&["dirt", "soil", "mud", "ground", "earth"], &["material:dirt"]),
    (&["sand", "desert", "dune"], &["material:sand"]),
    (&["stone", "rock", "cobble", "cobblestone", "boulder", "pebble", "gravel", "cliff"], &["material:stone"]),
    (&["brick", "bricks"], &["material:brick"]),
    (&["concrete", "cement", "asphalt", "pavement", "road"], &["material:concrete"]),
    (&["wood", "wooden", "plank", "planks", "bark", "timber", "log"], &["material:wood"]),
    (
        &["metal", "rust", "rusty", "iron", "steel", "copper", "brass", "gold", "chrome", "aluminum"],
        &["material:metal"],
    ),
    (&["fabric", "cloth", "leather", "carpet", "wool", "denim", "silk"], &["material:fabric"]),
    (&["marble", "granite", "tile", "tiles", "ceramic", "porcelain"], &["material:tile"]),
    (&["ice", "snow", "frost", "frozen"], &["material:ice"]),
    (&["water", "lava", "liquid", "ocean", "puddle"], &["material:liquid"]),
    (&["moss", "leaves", "foliage", "ivy"], &["material:foliage"]),
    (&["plastic", "rubber", "glass"], &["material:synthetic"]),
    // --- 2D / UI
    (
        &[
            "ui",
            "gui",
            "hud",
            "button",
            "panel",
            "frame",
            "window",
            "slider",
            "checkbox",
            "cursor",
            "healthbar",
            "bar",
            "slot",
            "dialog",
        ],
        &["ui"],
    ),
    (&["icon", "icons", "emblem", "badge"], &["icon"]),
    (&["sprite", "sprites"], &["sprite"]),
    (&["sheet", "spritesheet", "strip", "atlas", "frames"], &["spritesheet"]),
    (&["tile", "tileset", "tilemap", "tiles", "autotile", "terrain"], &["tileset"]),
    (&["sky", "skybox", "cubemap", "hdri", "clouds", "sunset"], &["sky"]),
    (
        &[
            "particle",
            "particles",
            "smoke",
            "spark",
            "sparks",
            "flare",
            "muzzle",
            "flash",
            "trail",
            "vfx",
            "burst",
            "flame",
        ],
        &["vfx"],
    ),
    (&["decal", "decals", "graffiti", "blood", "crack", "cracks", "dirtmask", "stain"], &["decal"]),
    (&["background", "bg", "parallax", "backdrop"], &["background"]),
    // --- subjects shared by sprites and models
    (
        &[
            "character",
            "player",
            "hero",
            "heroine",
            "npc",
            "enemy",
            "knight",
            "soldier",
            "warrior",
            "mage",
            "wizard",
            "villager",
            "human",
            "humanoid",
            "mannequin",
        ],
        &["character"],
    ),
    (
        &[
            "car",
            "truck",
            "tank",
            "ship",
            "boat",
            "plane",
            "airplane",
            "helicopter",
            "spaceship",
            "bike",
            "motorcycle",
            "train",
            "cart",
        ],
        &["vehicle"],
    ),
    (
        &["weapon", "weapons", "sword", "axe", "bow", "gun", "pistol", "rifle", "shield", "spear", "hammer", "staff"],
        &["weapon"],
    ),
    (
        &[
            "house", "building", "wall", "tower", "castle", "roof", "bridge", "fence", "stairs", "floor", "window",
            "pillar", "column", "ruins", "dungeon",
        ],
        &["architecture"],
    ),
    (
        &[
            "chair",
            "table",
            "bed",
            "barrel",
            "crate",
            "box",
            "chest",
            "lamp",
            "shelf",
            "bench",
            "sign",
            "bottle",
            "furniture",
            "prop",
            "props",
            "pot",
            "vase",
        ],
        &["prop"],
    ),
    (
        &[
            "tree", "trees", "bush", "plant", "plants", "flower", "flowers", "mushroom", "cactus", "palm", "pine",
            "fern", "log", "stump", "rock", "rocks",
        ],
        &["nature"],
    ),
    (&["food", "apple", "bread", "meat", "cheese", "fruit", "potion"], &["item"]),
    (
        &["idle", "walkcycle", "attack", "death", "die", "hurt", "anim", "animation", "animated", "mixamo"],
        &["animated"],
    ),
    (&["rig", "rigged", "skeleton", "armature", "bones", "skinned"], &["rigged"]),
    // --- style / theme
    (&["pixel", "pixelart", "8bit", "16bit", "retro"], &["style:pixel-art"]),
    (&["lowpoly", "polygon", "lowres"], &["style:low-poly"]),
    (&["toon", "cartoon", "stylized", "stylised", "cel", "cute"], &["style:stylized"]),
    (&["realistic", "photoreal", "photoscan", "scan", "scanned", "pbr"], &["style:realistic"]),
    (&["handpainted", "painted"], &["style:hand-painted"]),
    (&["scifi", "futuristic", "cyber", "cyberpunk", "space", "robot", "mech", "alien"], &["theme:sci-fi"]),
    (&["fantasy", "medieval", "rpg", "elven", "dwarf"], &["theme:fantasy"]),
    (&["horror", "gore", "haunted", "graveyard"], &["theme:horror"]),
    (&["western", "cowboy"], &["theme:western"]),
    (&["modern", "city", "urban", "office", "street"], &["theme:modern"]),
    (&["nature", "outdoor", "outdoors"], &["theme:nature"]),
    // --- fonts
    (&["bold", "black", "heavy"], &["weight:bold"]),
    (&["italic", "oblique"], &["italic"]),
    (&["mono", "monospace", "monospaced"], &["monospace"]),
];

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

fn singular(t: &str) -> Option<String> {
    if t.len() > 3 && t.ends_with('s') && !t.ends_with("ss") { Some(t[..t.len() - 1].to_owned()) } else { None }
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

/// The tags a sound's measured features bring: its envelope, whether it's bright or dark, noisy or tonal, whether its
/// pitch rises or falls by an octave or more, and `loop` (a guess) when it loops cleanly.
pub(crate) fn measured_sound_tags(
    meta: &serde_json::Map<String, serde_json::Value>,
) -> Vec<(&'static str, TagSource, f32)> {
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
    let mut out: Vec<_> = out.into_iter().map(|t| (t, TagSource::Meta, 1.0)).collect();
    if meta.get("loops_cleanly").and_then(serde_json::Value::as_bool) == Some(true) {
        out.push(("loop", TagSource::Rule, 0.6));
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
            if let Some(bpm) = num("tempo_bpm") {
                tags.add(&format!("bpm:{}", bpm.round() as u32), TagSource::Meta, 1.0);
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
            for (tag, source, confidence) in measured_sound_tags(meta) {
                tags.add(tag, source, confidence);
            }
            ucs_from_rules(tags);
        }
        AssetKind::Texture => texture_tags(ext, meta, tags),
        AssetKind::Vector => vector_tags(meta, tags),
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
    ["license", "licence", "copying", "readme", "credits", "terms", "eula"].iter().any(|p| file.starts_with(p))
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

    #[test]
    fn licenses() {
        assert_eq!(detect_license("License: (CC0 1.0 Universal)"), Some("cc0"));
        assert_eq!(detect_license("https://creativecommons.org/licenses/by/4.0/"), Some("cc-by"));
    }
}
