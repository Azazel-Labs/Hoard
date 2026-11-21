//! Shared wire types between the Hoard server (`hoardd`) and clients (`hoard` CLI, engine plugins).
//!
//! Everything here is plain serde so an engine plugin in any language can mirror it from the JSON.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This build's version. Releases set `HOARD_VERSION` from their git tag when building, so version numbers never
/// have to be committed; without it (a local build) it's Cargo.toml's version.
pub const VERSION: &str = match option_env!("HOARD_VERSION") {
    Some(v) if !v.is_empty() => v,
    _ => env!("CARGO_PKG_VERSION"),
};

/// `hoard/<version>`, the User-Agent for Hoard's own HTTP requests.
pub fn user_agent() -> String {
    format!("hoard/{VERSION}")
}

/// Broad bucket an asset falls into. Decided by extension + magic bytes, never by the LLM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetKind {
    Texture,
    /// Drawings made of shapes rather than pixels (SVG, Illustrator), which scale to any size.
    Vector,
    /// Sprites packed into one or more images, with a file saying where each one is (a TexturePacker JSON, a
    /// libGDX `.atlas`, a Godot `SpriteFrames` or `TileSet`…). Never decided by extension: found by reading the file.
    Sheet,
    Audio,
    Model,
    Animation,
    Font,
    Shader,
    Material,
    Video,
    Document,
    Data,
    Other,
}

impl AssetKind {
    pub const ALL: [AssetKind; 13] = [
        AssetKind::Texture,
        AssetKind::Vector,
        AssetKind::Sheet,
        AssetKind::Audio,
        AssetKind::Model,
        AssetKind::Animation,
        AssetKind::Font,
        AssetKind::Shader,
        AssetKind::Material,
        AssetKind::Video,
        AssetKind::Document,
        AssetKind::Data,
        AssetKind::Other,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            AssetKind::Texture => "texture",
            AssetKind::Vector => "vector",
            AssetKind::Sheet => "sheet",
            AssetKind::Audio => "audio",
            AssetKind::Model => "model",
            AssetKind::Animation => "animation",
            AssetKind::Font => "font",
            AssetKind::Shader => "shader",
            AssetKind::Material => "material",
            AssetKind::Video => "video",
            AssetKind::Document => "document",
            AssetKind::Data => "data",
            AssetKind::Other => "other",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.as_str() == s)
    }

    /// Classify by lowercase file extension (no dot).
    pub fn from_ext(ext: &str) -> Self {
        match ext {
            // Images, including layered/source formats (PSD/PSB, Krita, OpenRaster, GIMP, Clip Studio, Affinity Photo).
            "png" | "jpg" | "jpeg" | "tga" | "bmp" | "gif" | "webp" | "tif" | "tiff" | "psd" | "psb" | "dds"
            | "ktx" | "ktx2" | "exr" | "hdr" | "basis" | "aseprite" | "ase" | "kra" | "ora" | "xcf" | "clip"
            | "afphoto" | "procreate" | "pvr" | "vtf" | "pcx" | "ico" | "cur" | "qoi" | "ppm" | "pgm" | "pbm"
            | "pnm" | "pam"
            // Camera raw photos, shown by the JPEG inside.
            | "dng" | "cr2" | "nef" | "nrw" | "arw" | "pef" | "orf" | "rw2" | "srw" | "raf"
            | "jxl" | "avif" | "heic" | "heif" => AssetKind::Texture,
            // Vector art: SVG, Illustrator, Affinity Designer, EPS, dotLottie animations.
            "svg" | "svgz" | "ai" | "afdesign" | "eps" | "lottie" | "riv" => AssetKind::Vector,
            // Audio, including game audio banks (FMOD, Wwise) and DAW projects (Ableton, FL Studio, Reaper, Pro Tools, Logic, Cubase, Studio One).
            "wav" | "ogg" | "oga" | "mp3" | "flac" | "aif" | "aiff" | "aifc" | "opus" | "m4a" | "aac" | "wma"
            | "caf" | "wv" | "ape" | "mka" | "w64" | "au" | "mid" | "midi" | "xm" | "it" | "mod" | "s3m" | "mptm" | "umx" | "nsf" | "nsfe" | "vgm" | "vgz" | "fsb" | "bank" | "wem" | "bnk" | "als"
            | "flp" | "rpp" | "ptx" | "logicx" | "cpr" | "song" | "aup3" | "sfz" | "sf2" => AssetKind::Audio,
            // Models, including DCC scenes (Blender, Max, Maya, ZBrush, Cinema 4D, Houdini, Modo, SketchUp).
            "fbx" | "obj" | "gltf" | "glb" | "dae" | "blend" | "3ds" | "stl" | "ply" | "usd" | "usda" | "usdc"
            | "usdz" | "vox" | "max" | "ma" | "mb" | "ztl" | "zpr" | "c4d" | "hip" | "hiplc" | "hipnc" | "lxo"
            | "skp" | "abc" | "x3d" | "3mf" => AssetKind::Model,
            "anim" | "bvh" => AssetKind::Animation,
            "ttf" | "otf" | "ttc" | "otc" | "woff" | "woff2" | "fnt" => AssetKind::Font,
            "glsl" | "hlsl" | "wgsl" | "shader" | "vert" | "frag" | "comp" | "gdshader" | "usf" | "ush" | "metal"
            | "cginc" | "shadergraph" | "shadersubgraph" => AssetKind::Shader,
            // Materials, including Substance Designer/Painter sources.
            "mat" | "mtl" | "sbsar" | "sbs" | "spp" | "spsm" | "tres" => AssetKind::Material,
            "mp4" | "webm" | "mov" | "mkv" | "avi" | "ogv" => AssetKind::Video,
            "txt" | "md" | "pdf" | "html" | "htm" | "rtf" | "url" => AssetKind::Document,
            // Unreal packages (`uasset`, `umap`) start as data; the server refines the kind from the asset's class.
            // Unity scenes, prefabs and serialized assets are YAML (or binary) data.
            "json" | "xml" | "csv" | "yaml" | "yml" | "toml" | "tmx" | "tsx" | "ldtk" | "ini" | "uasset" | "umap"
            | "prefab" | "unity" | "asset" | "controller" | "overridecontroller" | "physicmaterial" | "mask"
            | "atlas" | "tscn" | "tx" => AssetKind::Data,
            _ => AssetKind::Other,
        }
    }
}

/// Where a tag came from. Human tags always win and are never overwritten by machines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TagSource {
    /// Derived from file metadata (dimensions, duration, format). Facts.
    Meta,
    /// Offline heuristic tagger (filename vocab, image analysis, audio heuristics).
    Rule,
    /// An AI model (the local model server, or any OpenAI-compatible endpoint).
    /// Stored as `llm` before v1.0.1; that still reads as this.
    #[serde(alias = "llm")]
    Ai,
    /// A person typed it.
    Human,
}

impl TagSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagSource::Meta => "meta",
            TagSource::Rule => "rule",
            TagSource::Ai => "ai",
            TagSource::Human => "human",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "meta" => TagSource::Meta,
            "rule" => TagSource::Rule,
            "ai" | "llm" => TagSource::Ai,
            _ => TagSource::Human,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    pub source: TagSource,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub pack_id: String,
    pub pack_name: String,
    pub name: String,
    /// Original relative path inside the pack.
    pub path: String,
    pub ext: String,
    pub kind: AssetKind,
    pub category: Option<String>,
    /// Written by a person (the portal or MCP). Automatic passes never touch it.
    pub description: Option<String>,
    /// What the source says about this file: the description embedded in it (BWF `bext`, `ICMT`, iXML). The pack's
    /// store page, which covers the whole pack, is in its context (`GET /api/packs/{id}/context`).
    #[serde(default)]
    pub source_description: Option<String>,
    /// Written by the AI tagger: what it is and how a game might use it.
    #[serde(default)]
    pub ai_description: Option<String>,
    /// BLAKE3 hex of the file contents. Stable identity for caching / lockfiles.
    pub hash: String,
    pub size: u64,
    pub has_thumb: bool,
    /// Changes whenever the thumbnail does, so its URL can too (`/thumb?v=…`): thumbnails are cached for good.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb_rev: Option<String>,
    /// Where the thumbnail came from when it wasn't drawn by the server: `browser` (a signed-in person's browser
    /// drew the model with all its textures, as the 3D viewer shows it) or `browser-partial` (some textures were
    /// missing). Re-analysis keeps these; a better one replaces a worse one, never the other way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb_source: Option<String>,
    /// An animation has a short looping movie of it playing (`/derivative/motion`, an animated WebP), shown when its
    /// tile is hovered. Made only while the "Moving thumbnails" setting is on.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_motion: bool,
    /// An animation's length in seconds: its longest clip that isn't a pose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_secs: Option<f64>,
    pub meta: Value,
    pub tags: Vec<Tag>,
    pub created_at: String,
    pub updated_at: String,
    /// When this asset, or the pack it's in, went to the archive. Archived assets are left out of searches; the
    /// archive is where they're restored or deleted for good.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    /// When a person said its details are right. Rules and the AI then leave its tags, name and category alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    /// A person's rating, 1 to 5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<u8>,
    /// Its own license, when it isn't its pack's (`pack_license`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Where it came from, when that isn't its pack's store page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Its pack's license, which is its own unless `license` says otherwise. Detail view only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_license: Option<String>,
    /// Fields of the library's own, by name ("Artist": "Kim"). Detail view only.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub fields: std::collections::BTreeMap<String, String>,
    /// Tags a rule or the AI gave it that a person turned down, which retagging won't give it again. Detail view
    /// only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejected_tags: Vec<String>,
    /// Files this asset needs (textures, buffers, materials) or goes with (animations). Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<AssetLink>,
    /// Assets that reference this one (e.g. the meshes using a texture). Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_from: Vec<AssetLink>,
    /// Server-made conversions available for this asset, e.g. `["glb"]` for a .blend converted for preview.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derivatives: Vec<String>,
    /// Numbered/lettered siblings in the same folder (`whoosh_01` … `whoosh_12`). Detail view only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
    /// The identical file (same bytes) elsewhere in the library, usually in other packs. Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub duplicates: Vec<Duplicate>,
    /// The same picture, sound or model in other files (see [`NearCopy`]), the best first. Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub near_copies: Vec<NearCopy>,
    /// For a character, the animations across the library that fit its skeleton; for an animation, the characters
    /// it fits (see [`Fit`]). The same pack first, at most 200. Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fits: Vec<Fit>,
    /// How many fit in all, when that's more than `fits` lists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fits_total: Option<u64>,
    /// Collections this asset is in (detail view only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub collections: Vec<CollectionRef>,
    /// What this file was converted from (an Unreal package, say). Detail view only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<AssetOrigin>,
    /// For Unreal packages not yet converted: why. Detail view only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unreal: Option<UnrealStatus>,
    /// Other formats this asset can be downloaded as (`GET /api/assets/{id}/as/{format}`). Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formats: Vec<FormatOption>,
    /// Dominant colors of the thumbnail, most of the picture first. Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub palette: Vec<Swatch>,
    /// For a material made from texture maps: its maps. Detail view only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<MaterialInfo>,
    /// Materials made from texture maps that this texture is one of, or this model uses. Detail view only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<MaterialInfo>,
    /// For a model: how its pack's models are turned to be shown, when they are. Detail view only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facing: Option<Facing>,
}

/// One color of an asset's palette.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swatch {
    /// `#rrggbb`.
    pub hex: String,
    /// Share of the picture (its opaque part), 0..=1.
    pub share: f32,
}

/// A material made from texture maps that share a name (`Rock_BaseColor.png`, `Rock_Normal.png`, …).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub has_thumb: bool,
    /// By role (color first), each role's largest first.
    pub maps: Vec<MaterialMap>,
    /// How much ground one repeat of a tiling material covers, width and height in meters, when the store says
    /// (a Megascans surface's `<id>.json`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_size: Option<[f32; 2]>,
    /// How far its height map reaches, in meters, when the store says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub displacement: Option<f32>,
    /// The settings its maps are drawn with, when it was read from an engine's material (a Unity `.mat`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub look: Option<MaterialLook>,
    /// On a model's materials: the model's material slots this one goes in (Unity's `.mat` for an FBX's slot).
    /// Empty when it goes on the whole model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<String>,
}

/// How a material read from an engine's file draws its maps, in glTF's terms. Anything unset is as a material
/// made from maps alone draws it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MaterialLook {
    /// Linear RGBA, multiplied into the color map (or the color itself, without one).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_color: Option<[f32; 4]>,
    /// 0..=1, where there's no metalness map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metallic: Option<f32>,
    /// 0..=1, where there's no roughness map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roughness: Option<f32>,
    /// Linear RGB light it gives off, multiplied into the emissive map (or the light itself, without one).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive: Option<[f32; 3]>,
    /// `mask` (cut out where the color's alpha is under `alpha_cutoff`) or `blend` (see-through by it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_scale: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occlusion_strength: Option<f32>,
    /// Times the maps repeat across the surface, and how far they're moved, in U and V.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_scale: Option<[f32; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_offset: Option<[f32; 2]>,
}

/// One texture of a material and what it's for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialMap {
    /// The texture's asset id.
    pub id: String,
    pub name: String,
    pub path: String,
    pub ext: String,
    /// `base_color`, `normal` (OpenGL style), `normal_dx` (DirectX style), `roughness`, `gloss` (inverted roughness),
    /// `metalness`, `ao`, `orm` (occlusion, roughness, metalness in R, G, B; also called ARM), `mra`, `rma`,
    /// `height`, `bump`, `cavity`, `specular`, `opacity`, `emissive` or `translucency`.
    pub role: String,
    /// `2K`, `512`: the set comes in more than one size when the same role appears more than once.
    pub resolution: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub has_thumb: bool,
    pub size: u64,
}

/// A format an asset can be converted to on download.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatOption {
    /// Extension-like id: `png`, `wav`, `glb`, …
    pub format: String,
    pub label: String,
    /// Accepts `?max=<pixels>` to scale down (images).
    pub resizable: bool,
}

/// Where a converted asset came from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOrigin {
    /// Path in the pack before conversion (`Content/Rocks/T_Rock_D.uasset`).
    pub path: String,
    pub ext: String,
    /// Whether the original file is still stored (download it from `/api/assets/{id}/original`).
    pub original_kept: bool,
    pub class: Option<String>,
    pub engine: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// `native` (Hoard's own reader), `editor` (exported by Unreal Editor) or `rule` (a conversion rule).
    pub method: String,
    pub notes: Option<String>,
    pub converted_at: String,
    /// When a conversion rule's original is deleted (RFC 3339); until then it can be put back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept_until: Option<String>,
}

/// Why an Unreal package hasn't been converted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrealStatus {
    /// `needs_editor`, `unsupported` or `failed`.
    pub status: String,
    pub class: Option<String>,
    pub reason: String,
}

/// Unreal conversion progress for a pack.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnrealPackStatus {
    pub packages: i64,
    pub converted: i64,
    pub originals_kept: i64,
    pub needs_editor: i64,
    pub unsupported: i64,
    pub failed: i64,
    /// Packages not looked at yet.
    pub pending: i64,
    /// `(reason, count)`, most common first.
    #[serde(default)]
    pub reasons: Vec<(String, i64)>,
}

/// A package waiting for Unreal Editor (`hoard unreal-worker`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingPackage {
    pub asset_id: String,
    pub pack_id: String,
    pub path: String,
    pub class: Option<String>,
    pub reason: String,
}

/// Options for converting a pack's Unreal packages.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct UnrealConvertRequest {
    /// Discard each original package once its content is out.
    #[serde(default)]
    pub discard_originals: bool,
    /// Delete packages with no common format (Blueprints, levels, …).
    #[serde(default)]
    pub drop_unsupported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Family {
    /// Shared base, e.g. `Sounds/whoosh`.
    pub key: String,
    /// Total members, including this asset.
    pub size: u64,
    /// Up to 60 other members.
    pub members: Vec<AssetLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duplicate {
    pub id: String,
    pub name: String,
    pub pack_id: String,
    pub pack_name: String,
    pub path: String,
}

/// The same picture, sound or model as an asset, in another file: another size, rate or format. Detail view and the
/// duplicates page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NearCopy {
    pub id: String,
    pub name: String,
    pub pack_id: String,
    pub pack_name: String,
    pub path: String,
    pub ext: String,
    /// Bytes.
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<u32>,
    /// The best of the copies: the biggest, lossless, most complete.
    #[serde(default)]
    pub best: bool,
}

/// An animation that plays on a character, or a character an animation plays on: one whose skeleton has the bones
/// the clip moves. Detail view.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fit {
    pub id: String,
    pub name: String,
    pub path: String,
    pub pack_id: String,
    pub pack_name: String,
    pub kind: AssetKind,
    pub has_thumb: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb_rev: Option<String>,
    /// An animation's length in seconds, its longest take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secs: Option<f64>,
}

/// A set of near copies across the library, the best first.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CopyGroup {
    /// Bytes taken by the copies that aren't the best.
    pub wasted: i64,
    pub copies: Vec<NearCopy>,
}

/// A page of the library's near copies, the most space taken first.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CopyGroups {
    pub groups: Vec<CopyGroup>,
    /// How many sets there are in all.
    pub total: usize,
    /// Bytes taken by the copies that aren't the best, in all of them.
    pub wasted: i64,
    /// Which working-out of the library's sets this page is from; it goes up each time they're worked out again.
    pub generation: u64,
    /// The library has changed since these were worked out, and they're being worked out again: ask again for the next
    /// generation.
    pub refreshing: bool,
}

/// A named set of assets put together by hand ("Forest level props"), across packs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub asset_count: u64,
    pub total_bytes: u64,
    /// Up to four asset ids with thumbnails, for a cover.
    #[serde(default)]
    pub preview: Vec<String>,
    /// The kind of each `preview` asset (`font` thumbnails are light glyphs on transparency, for clients to tint).
    #[serde(default)]
    pub preview_kinds: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CollectionRef {
    pub id: String,
    pub name: String,
}

/// Create or rename a collection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionPatch {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Add assets to, or take them out of, a collection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionMembers {
    #[serde(default)]
    pub add: Vec<String>,
    #[serde(default)]
    pub remove: Vec<String>,
}

/// Where a pack came from and what it is, as written by its seller. The LLM tagger sees this with every file,
/// because a store page saying "120 ice and fire spells" explains `spell35.wav` better than the file can.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackContext {
    /// Store or marketplace, e.g. "Fab", "itch.io", "Humble Bundle".
    #[serde(default)]
    pub store: Option<String>,
    #[serde(default)]
    pub store_url: Option<String>,
    /// The store page's description and listed contents (pasted or fetched).
    #[serde(default)]
    pub store_text: Option<String>,
    /// License tier where stores sell several (Fab: Personal / Professional).
    #[serde(default)]
    pub license_tier: Option<String>,
    /// Text of README/LICENSE files found in the pack (read-only, captured at ingest).
    #[serde(default)]
    pub readme: Option<String>,
    /// Who made or sells it ("Quixel Megascans"), for credits.
    #[serde(default)]
    pub seller: Option<String>,
    /// The license the store sells it under ("Fab Standard License", "CC BY 4.0"), for credits.
    #[serde(default)]
    pub store_license: Option<String>,
    /// When the store page was found and attached automatically (read-only; cleared when the page is changed).
    #[serde(default)]
    pub store_auto: Option<String>,
}

/// How two assets relate. `Texture`, `Buffer` and `Material` are hard dependencies (the file is broken
/// without them); `Animation` is a clip made for the model; `Guess` is a name-based hunch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkRel {
    Texture,
    Buffer,
    Material,
    Animation,
    /// Another source file this one links in (a .blend library).
    Library,
    Guess,
    /// The store's file that describes this asset (a Megascans `<id>.json`). Not something the asset needs: it goes
    /// with the whole pack's download, not with the model's or material's.
    Sidecar,
    /// A sibling in the same variant family. Computed, never stored as a link.
    Variant,
    /// A loose image that is one of this sheet's sprites, saved on its own as well. Not something the sheet needs:
    /// each sprite is in it already.
    Sprite,
    /// The same asset in another format, which the pack has as well (the `.obj` beside a `.skp`).
    Format,
    /// A sound an instrument plays (an SFZ's samples): part of it, not an asset on its own.
    Sample,
}

impl LinkRel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkRel::Texture => "texture",
            LinkRel::Buffer => "buffer",
            LinkRel::Material => "material",
            LinkRel::Animation => "animation",
            LinkRel::Library => "library",
            LinkRel::Guess => "guess",
            LinkRel::Sidecar => "sidecar",
            LinkRel::Variant => "variant",
            LinkRel::Sprite => "sprite",
            LinkRel::Format => "format",
            LinkRel::Sample => "sample",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "texture" => LinkRel::Texture,
            "buffer" => LinkRel::Buffer,
            "material" => LinkRel::Material,
            "animation" => LinkRel::Animation,
            "library" => LinkRel::Library,
            "guess" => LinkRel::Guess,
            "sidecar" => LinkRel::Sidecar,
            "variant" => LinkRel::Variant,
            "sprite" => LinkRel::Sprite,
            "format" => LinkRel::Format,
            "sample" => LinkRel::Sample,
            _ => return None,
        })
    }
    /// Whether a bundle download should include the target by default.
    pub fn is_dependency(&self) -> bool {
        matches!(
            self,
            LinkRel::Texture
                | LinkRel::Buffer
                | LinkRel::Material
                | LinkRel::Library
                | LinkRel::Guess
                | LinkRel::Sample
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetLink {
    pub id: String,
    pub name: String,
    pub path: String,
    pub kind: AssetKind,
    pub rel: LinkRel,
    pub has_thumb: bool,
    /// File size in bytes.
    #[serde(default)]
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackStatus {
    Uploading,
    Ingesting,
    Analyzing,
    Ready,
    Failed,
}

impl PackStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PackStatus::Uploading => "uploading",
            PackStatus::Ingesting => "ingesting",
            PackStatus::Analyzing => "analyzing",
            PackStatus::Ready => "ready",
            PackStatus::Failed => "failed",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "uploading" => PackStatus::Uploading,
            "ingesting" => PackStatus::Ingesting,
            "analyzing" => PackStatus::Analyzing,
            "ready" => PackStatus::Ready,
            _ => PackStatus::Failed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pack {
    pub id: String,
    pub name: String,
    pub status: PackStatus,
    pub source: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub asset_count: u64,
    pub total_bytes: u64,
    pub error: Option<String>,
    /// Ingest progress: files processed so far / files found after unpacking.
    pub files_done: u64,
    pub files_total: u64,
    pub jobs_pending: u64,
    #[serde(default)]
    pub store: Option<String>,
    #[serde(default)]
    pub store_url: Option<String>,
    #[serde(default)]
    pub license_tier: Option<String>,
    pub created_at: String,
    /// When any of its assets last changed (a file added, replaced or read again, a name, tag or description), or
    /// `created_at` while it has none.
    #[serde(default)]
    pub updated_at: String,
    /// Up to four asset ids with thumbnails, for a cover mosaic (mixed kinds where the pack has them).
    #[serde(default)]
    pub preview: Vec<String>,
    /// The kind of each `preview` asset (`font` thumbnails are light glyphs on transparency, for clients to tint).
    #[serde(default)]
    pub preview_kinds: Vec<String>,
    /// Unreal packages still in package form.
    #[serde(default)]
    pub unreal_packages: u64,
    /// Assets converted from Unreal packages.
    #[serde(default)]
    pub unreal_converted: u64,
    /// When the pack went to the archive. Its assets are left out of searches until it's restored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    /// How many assets of each kind it holds (`model`, `texture`, …), for filtering the list of packs.
    #[serde(default)]
    pub kinds: std::collections::BTreeMap<String, u64>,
    /// How its models are turned to be shown standing and facing forward.
    #[serde(default)]
    pub facing: Facing,
}

/// How a pack's models are turned to be shown standing up and facing forward, for files that don't say which way
/// is up or front (OBJ, STL, PLY, 3DS, Collada), or say it wrongly: a tip about the side-to-side axis, then a turn
/// about the up axis, each a multiple of 90 degrees. Thumbnails, the 3D viewer and GLB downloads show the models
/// turned; their files are left as they were made.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Facing {
    /// Degrees about the up axis, counterclockwise seen from above: 0, 90, 180 or 270.
    #[serde(default)]
    pub turn: i32,
    /// Degrees about the side-to-side axis, the top tipping toward the viewer: 0, 90, 180 or 270.
    #[serde(default)]
    pub tilt: i32,
}

impl Facing {
    /// Each angle brought to a quarter turn in 0..360 (rounded down).
    pub fn new(turn: i64, tilt: i64) -> Self {
        let quarter = |deg: i64| (deg.rem_euclid(360) / 90 * 90) as i32;
        Self { turn: quarter(turn), tilt: quarter(tilt) }
    }

    /// Shown as made.
    pub fn is_upright(self) -> bool {
        self == Self::default()
    }

    /// The glTF rotation (`[x, y, z, w]`) that tips, then turns: the turn about Y times the tip about X.
    pub fn quaternion(self) -> [f64; 4] {
        let (sy, cy) = (f64::from(self.turn).to_radians() / 2.0).sin_cos();
        let (sx, cx) = (f64::from(self.tilt).to_radians() / 2.0).sin_cos();
        // Quarter turns' halves have sines and cosines of 0, ±½, ±√½ and ±1: rounding leaves no specks.
        let snap = |v: f64| (v * 1e9).round() / 1e9 + 0.0;
        [snap(cy * sx), snap(cx * sy), snap(-sy * sx), snap(cy * cx)]
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Free text (FTS5 over name, path, category, description, tags).
    #[serde(default)]
    pub q: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    /// All of these tags must be present.
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub pack: Option<String>,
    /// With `pack`: only what's directly in this folder of it (`""` for its top, `a/b` for a folder in that), as if
    /// the pack were still a folder on disk; with a text query, anything under it.
    #[serde(default)]
    pub folder: Option<String>,
    #[serde(default)]
    pub ext: Option<String>,
    /// Best match first (default; newest first without a text, color or similar query), or "created", "updated",
    /// "name" or "size", each its usual way round (newest, A to Z, biggest first) or with ":asc" or ":desc" after it.
    /// "new" and "old" are "created" and "created:asc".
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    /// Leave out files that only exist to support another asset (glTF buffers, OBJ material libraries); they
    /// still come along in bundles and show under "Needs" on the model.
    #[serde(default)]
    pub hide_supporting: bool,
    /// Only assets in this collection (id or name).
    #[serde(default)]
    pub collection: Option<String>,
    /// Only assets this project's last `hoard sync` reported having (id or name).
    #[serde(default)]
    pub project: Option<String>,
    /// Search the archive instead of the library: assets archived one by one, or with `pack`, everything archived
    /// from that pack.
    #[serde(default)]
    pub archived: bool,
    /// "More like this": assets that look like this one (an asset id), most alike first.
    #[serde(default)]
    pub similar: Option<String>,
    /// Assets with much of this color (`#rrggbb`) in them, most first.
    #[serde(default)]
    pub color: Option<String>,
    /// Sounds shaped like this word imitating them (`whooom`, `pew`, `krssh`), most alike first. In place of
    /// `similar` and `color`.
    #[serde(default)]
    pub imitation: Option<String>,
    /// Music at this tempo (BPM) or faster, as its file states it or as heard (either way of counting it, half or
    /// double).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bpm_min: Option<f32>,
    /// Music at this tempo (BPM) or slower.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bpm_max: Option<f32>,
    /// Music in this key, however it's written (`a-minor`, `A minor`, `Am`, `E♭ major`, `8A`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// With `key`: the keys that mix with it too (its relative major or minor, and the keys a fifth either side).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub compatible_keys: bool,
    /// Only what a person has said is right (`true`), or only what they haven't yet (`false`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed: Option<bool>,
    /// Only this account's favorites. Set by the server for whoever asks, never sent.
    #[serde(skip)]
    pub favorites_of: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facet {
    pub value: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub items: Vec<Asset>,
    pub total: u64,
    pub kinds: Vec<Facet>,
    pub tags: Vec<Facet>,
    /// Matches left out by `hide_supporting`.
    #[serde(default)]
    pub supporting_hidden: u64,
    /// Browsing a folder of a pack (`folder`, without a text query): the folders in it, each with how many matches
    /// are somewhere under it, by name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<Facet>,
}

/// A change to an asset. Each field left out stays as it is; an empty text clears what it's for.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetPatch {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    /// Its own license, when it isn't its pack's.
    #[serde(default)]
    pub license: Option<String>,
    /// Where it came from.
    #[serde(default)]
    pub source_url: Option<String>,
    /// 1 to 5; 0 clears it.
    #[serde(default)]
    pub rating: Option<u8>,
    /// Its details are right (or not any more).
    #[serde(default)]
    pub reviewed: Option<bool>,
    /// Fields of the library's own to set, by name; an empty value removes one.
    #[serde(default)]
    pub fields: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    pub add_tags: Vec<String>,
    /// Tags to take off. A rule's or the AI's tag taken off stays off: retagging won't add it again, until
    /// `unreject_tags` puts it back.
    #[serde(default)]
    pub remove_tags: Vec<String>,
    /// A rule's or the AI's tags to keep as a person's.
    #[serde(default)]
    pub accept_tags: Vec<String>,
    /// Keep every tag a rule or the AI gave it as a person's.
    #[serde(default)]
    pub accept_all_tags: bool,
    /// A rule's or the AI's tags taken off before, to put back as they were.
    #[serde(default)]
    pub unreject_tags: Vec<String>,
    /// Apply the changes to every variant in the asset's family too (all but its name, rating, fields and whether
    /// it's reviewed).
    #[serde(default)]
    pub apply_to_family: bool,
}

/// Request a flat, engine-friendly list of files: either explicit ids, a search, or both.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManifestRequest {
    #[serde(default)]
    pub ids: Vec<String>,
    #[serde(default)]
    pub query: Option<SearchQuery>,
    /// Also return each asset's dependencies (textures, buffers, materials), grouped into bundles.
    #[serde(default)]
    pub include_deps: bool,
    /// With `include_deps`, also pull in linked animation clips.
    #[serde(default)]
    pub include_animations: bool,
}

/// Credits for the packs a project uses: whole packs, collections, assets, and selections as `hoard.toml` writes
/// them. Everything given is put together into one list of packs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreditsRequest {
    /// Pack ids or names.
    #[serde(default)]
    pub packs: Vec<String>,
    /// Collection ids or names.
    #[serde(default)]
    pub collections: Vec<String>,
    /// Asset ids.
    #[serde(default)]
    pub ids: Vec<String>,
    /// Selections, as for a manifest.
    #[serde(default)]
    pub selections: Vec<ManifestRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub id: String,
    pub hash: String,
    pub size: u64,
    pub kind: AssetKind,
    pub name: String,
    pub pack_name: String,
    /// Original path within its pack — a sensible default destination.
    pub path: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub meta: Value,
    /// Relative URL to fetch the bytes (`/api/blobs/<hash>`). Immutable, cache forever.
    pub url: String,
    /// When this entry is part of a dependency bundle: the id of the bundle's root asset (the mesh).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,
    /// Path inside the bundle, relative to the files' common folder. Place every member of a bundle at
    /// `<somewhere>/<bundle_path>` and relative references between them keep working.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_path: Option<String>,
    /// The name to save the file as, when it isn't the one in `path`: texture maps are named by the server's
    /// convention (`Rock_BaseColor.png`). A bundle's `bundle_path` already has it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub generated_at: String,
    pub entries: Vec<ManifestEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub assets: u64,
    pub packs: u64,
    pub blobs: u64,
    /// Blobs whose last scrub found them corrupt or missing.
    pub blobs_bad: u64,
    pub stored_bytes: u64,
    pub logical_bytes: u64,
    pub jobs_pending: u64,
    pub llm_enabled: bool,
    /// Human-readable storage backend, e.g. `local:./hoard-data/blobs` or `s3:http://nas:3900/hoard`.
    pub storage: String,
    pub kinds: Vec<Facet>,
    /// Packs in the archive.
    #[serde(default)]
    pub archived_packs: u64,
    /// Assets archived one by one (not counting the ones in archived packs).
    #[serde(default)]
    pub archived_assets: u64,
    /// Assets whose bytes are identical to an earlier asset (same file shipped in several packs).
    #[serde(default)]
    pub duplicate_assets: u64,
    /// Server version, e.g. `0.2.0`.
    #[serde(default)]
    pub version: String,
    /// ffmpeg's state: its version line, `downloading`, `off` or `unavailable: <why>`.
    #[serde(default)]
    pub ffmpeg: String,
    /// Blender's state, for model conversion: its version, `looking`, `downloading`, `off` or `unavailable: <why>`.
    #[serde(default)]
    pub blender: String,
    /// A newer Hoard release than this server, if the daily check found one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateInfo>,
    /// What model thumbnails render on: `gpu: <adapter> (<api>)`, `cpu: <adapter> (<api>)` (software, no GPU
    /// visible), `starting`, `off` or `unavailable: <why>`.
    #[serde(default)]
    pub renderer: String,
    /// When the renderer fell back to software although a GPU seems to be there: what's missing, in a sentence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renderer_hint: Option<String>,
    /// No library location yet: the portal opens its first-run setup.
    #[serde(default)]
    pub setup_needed: bool,
    /// Space taken by cached "download as" conversions, and the most they may take.
    #[serde(default)]
    pub conversion_cache_bytes: u64,
    #[serde(default)]
    pub conversion_cache_limit: u64,
    /// Space taken by thumbnails, model views and audio previews (the local cache).
    #[serde(default)]
    pub preview_bytes: u64,
    /// The editors this build fetches itself when a conversion needs one (`blender`, `unreal`; the Docker image);
    /// elsewhere they are found where they are installed.
    #[serde(default)]
    pub editor_downloads: Vec<String>,
}

/// A Hoard release newer than the running server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// e.g. `0.9.0`.
    pub version: String,
    /// The release page, with its notes.
    pub url: String,
}

/// Normalize a tag: lowercase, trimmed, spaces/underscores -> '-', optional `namespace:value`. Any script works: letters
/// and digits are kept with the marks written on them (Hindi's vowel signs, Thai's tone marks) and the joiners some scripts
/// spell with, in composed form so an accent typed on its own matches one typed with its letter. At most 64 characters.
pub fn normalize_tag(raw: &str) -> Option<String> {
    use icu_normalizer::ComposingNormalizerBorrowed;
    use icu_properties::{
        CodePointMapData,
        props::{GeneralCategory, GeneralCategoryGroup},
    };
    let raw = ComposingNormalizerBorrowed::new_nfc().normalize(&raw.trim().to_lowercase()).into_owned();
    if raw.is_empty() {
        return None;
    }
    let category = CodePointMapData::<GeneralCategory>::new();
    let norm_part = |s: &str| -> String {
        let mut out = String::with_capacity(s.len());
        for c in s.chars() {
            let after_text = out.chars().next_back().is_some_and(|p| p != '-');
            if c.is_alphanumeric() || c == '/' || c == '.' || c == '+' {
                out.push(c);
            } else if c == '\u{200c}' || c == '\u{200d}' || GeneralCategoryGroup::Mark.contains(category.get(c)) {
                // A mark or joiner belongs to the letter before it, and means nothing without one.
                if after_text {
                    out.push(c);
                }
            } else if after_text {
                out.push('-');
            }
        }
        out.trim_end_matches(['-', '\u{200c}', '\u{200d}']).to_owned()
    };
    let tag = match raw.split_once(':') {
        Some((ns, v)) => {
            let (ns, v) = (norm_part(ns), norm_part(v));
            if ns.is_empty() || v.is_empty() {
                return None;
            }
            format!("{ns}:{v}")
        }
        None => norm_part(&raw),
    };
    if tag.is_empty() || tag.chars().count() > 64 { None } else { Some(tag) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tags_normalize() {
        assert_eq!(normalize_tag("  Pixel Art "), Some("pixel-art".into()));
        assert_eq!(normalize_tag("Genre: Lo Fi"), Some("genre:lo-fi".into()));
        assert_eq!(normalize_tag("sfx/explosion"), Some("sfx/explosion".into()));
        assert_eq!(normalize_tag(":"), None);
        // Native text in both halves, whatever the script.
        assert_eq!(normalize_tag("楽器:ドラム"), Some("楽器:ドラム".into()));
        assert_eq!(normalize_tag("Инструмент: Барабан"), Some("инструмент:барабан".into()));
        assert_eq!(normalize_tag("संगीत:क्षेत्र"), Some("संगीत:क्षेत्र".into()));
        assert_eq!(normalize_tag("เสียง:ไม้กลอง"), Some("เสียง:ไม้กลอง".into()));
        assert_eq!(normalize_tag("מוזיקה:כְּלִי"), Some("מוזיקה:כְּלִי".into()));
        assert_eq!(normalize_tag("می\u{200c}خواهم"), Some("می\u{200c}خواهم".into()));
        // An accent typed apart from its letter is the same tag as one typed with it.
        assert_eq!(normalize_tag("Cafe\u{301}"), Some("café".into()));
        assert_eq!(normalize_tag("Café"), Some("café".into()));
        // The limit is characters, not bytes.
        assert!(normalize_tag(&"楽器".repeat(32)).is_some());
        assert!(normalize_tag(&"楽器".repeat(33)).is_none());
    }

    /// Rotate `point` by the unit quaternion `[x, y, z, w]`: p + 2w(q×p) + 2q×(q×p).
    fn rotate(quat: [f64; 4], point: [f64; 3]) -> [f64; 3] {
        let cross = |a: [f64; 3], b: [f64; 3]| {
            [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
        };
        let axis = [quat[0], quat[1], quat[2]];
        let once = cross(axis, point);
        let twice = cross(axis, once);
        let rotated: [f64; 3] = std::array::from_fn(|i| point[i] + 2.0 * (quat[3] * once[i] + twice[i]));
        rotated.map(|c| (c * 1e6).round() / 1e6 + 0.0)
    }

    #[test]
    fn facing_snaps_to_quarter_turns_and_turns_as_described() {
        assert_eq!(Facing::new(-90, 450), Facing { turn: 270, tilt: 90 });
        assert_eq!(Facing::new(100, 0), Facing { turn: 90, tilt: 0 });
        assert!(Facing::new(360, -360).is_upright());
        assert_eq!(Facing::default().quaternion(), [0.0, 0.0, 0.0, 1.0]);
        // Turning half way round faces a model's back to the viewer.
        assert_eq!(Facing::new(180, 0).quaternion(), [0.0, 1.0, 0.0, 0.0]);
        // A quarter turn counterclockwise seen from above takes the front (+Z) to the right (+X).
        assert_eq!(rotate(Facing::new(90, 0).quaternion(), [0.0, 0.0, 1.0]), [1.0, 0.0, 0.0]);
        // Tipping a quarter brings the top (+Y) toward the viewer (+Z).
        assert_eq!(rotate(Facing::new(0, 90).quaternion(), [0.0, 1.0, 0.0]), [0.0, 0.0, 1.0]);
        // A model lying with its top toward the viewer stands up tipped three quarters.
        assert_eq!(rotate(Facing::new(0, 270).quaternion(), [0.0, 0.0, 1.0]), [0.0, 1.0, 0.0]);
        // Tipped, then turned: the tip happens first, in the model's own frame.
        assert_eq!(rotate(Facing::new(90, 90).quaternion(), [0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]);
    }
}
