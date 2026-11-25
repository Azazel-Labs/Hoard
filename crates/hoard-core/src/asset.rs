//! An asset and what belongs to it: its kind, materials, origin, links and edits.

use crate::{CollectionRef, Duplicate, Facing, Family, Fit, NearCopy, Tag, UnrealStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Broad bucket an asset falls into. Decided by extension + magic bytes, never by the LLM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
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
            // Models, including DCC scenes (Blender, Max, Maya, ZBrush, Cinema 4D, Houdini, Modo, SketchUp), Collada
            // (zipped as `.zae` too), X3D and VRML (and their gzipped forms), OFF, VRM avatars and Gaussian splats.
            "fbx" | "obj" | "gltf" | "glb" | "dae" | "zae" | "blend" | "3ds" | "stl" | "ply" | "usd" | "usda"
            | "usdc" | "usdz" | "vox" | "max" | "ma" | "mb" | "ztl" | "zpr" | "c4d" | "hip" | "hiplc" | "hipnc"
            | "lxo" | "skp" | "abc" | "x3d" | "x3dv" | "x3dz" | "wrl" | "wrz" | "off" | "vrm" | "splat" | "spz"
            | "3mf" => AssetKind::Model,
            // Clips: Unity's, BVH motion capture and VRM animations.
            "anim" | "bvh" | "vrma" => AssetKind::Animation,
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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// For an animation, the character it plays on wherever it's shown (its thumbnails, the viewer, a download with
    /// its character): the one its skeleton's animations play on. Detail view only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plays_on: Option<Fit>,
    /// For a character, whether it's the one its skeleton's animations play on. Detail view only.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub plays_its_skeleton: bool,
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
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Swatch {
    /// `#rrggbb`.
    pub hex: String,
    /// Share of the picture (its opaque part), 0..=1.
    pub share: f32,
}

/// A material made from texture maps that share a name (`Rock_BaseColor.png`, `Rock_Normal.png`, …).
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FormatOption {
    /// Extension-like id: `png`, `wav`, `glb`, …
    pub format: String,
    pub label: String,
    /// Accepts `?max=<pixels>` to scale down (images).
    pub resizable: bool,
}

/// Where a converted asset came from.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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

/// How two assets relate. `Texture`, `Buffer` and `Material` are hard dependencies (the file is broken
/// without them); `Animation` is a clip made for the model; `Guess` is a name-based hunch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, schemars::JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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

/// A change to an asset. Each field left out stays as it is; an empty text clears what it's for.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
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
