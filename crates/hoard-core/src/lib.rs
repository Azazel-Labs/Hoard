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
    pub const ALL: [AssetKind; 11] = [
        AssetKind::Texture,
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
            // Images, including layered/source formats (PSD/PSB, Krita, OpenRaster, GIMP, Clip Studio, Affinity).
            "png" | "jpg" | "jpeg" | "tga" | "bmp" | "gif" | "webp" | "tif" | "tiff" | "psd" | "psb" | "dds"
            | "ktx" | "ktx2" | "exr" | "hdr" | "basis" | "svg" | "svgz" | "aseprite" | "ase" | "kra" | "ora"
            | "xcf" | "clip" | "afphoto" | "afdesign" | "ai" | "procreate" => AssetKind::Texture,
            // Audio, including DAW projects (Ableton, FL Studio, Reaper, Pro Tools, Logic, Cubase, Studio One).
            "wav" | "ogg" | "oga" | "mp3" | "flac" | "aif" | "aiff" | "aifc" | "opus" | "m4a" | "aac" | "wma"
            | "caf" | "wv" | "ape" | "mka" | "w64" | "au" | "mid" | "midi" | "xm" | "it" | "mod" | "s3m" | "als"
            | "flp" | "rpp" | "ptx" | "logicx" | "cpr" | "song" | "aup3" | "sfz" => AssetKind::Audio,
            // Models, including DCC scenes (Blender, Max, Maya, ZBrush, Cinema 4D, Houdini, Modo, SketchUp).
            "fbx" | "obj" | "gltf" | "glb" | "dae" | "blend" | "3ds" | "stl" | "ply" | "usd" | "usda" | "usdc"
            | "usdz" | "vox" | "max" | "ma" | "mb" | "ztl" | "zpr" | "c4d" | "hip" | "hiplc" | "hipnc" | "lxo"
            | "skp" | "abc" | "x3d" => AssetKind::Model,
            "anim" | "bvh" => AssetKind::Animation,
            "ttf" | "otf" | "ttc" | "otc" | "woff" | "woff2" | "fnt" => AssetKind::Font,
            "glsl" | "hlsl" | "wgsl" | "shader" | "vert" | "frag" | "comp" | "gdshader" | "usf" | "ush" | "metal"
            | "cginc" | "shadergraph" | "shadersubgraph" => AssetKind::Shader,
            // Materials, including Substance Designer/Painter sources.
            "mat" | "mtl" | "sbsar" | "sbs" | "spp" | "spsm" | "tres" => AssetKind::Material,
            "mp4" | "webm" | "mov" | "mkv" | "avi" | "ogv" => AssetKind::Video,
            "txt" | "md" | "pdf" | "html" | "htm" | "rtf" => AssetKind::Document,
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
    pub meta: Value,
    pub tags: Vec<Tag>,
    pub created_at: String,
    pub updated_at: String,
    /// When this asset, or the pack it's in, went to the archive. Archived assets are left out of searches; the
    /// archive is where they're restored or deleted for good.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
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
    /// `native` (Hoard's own reader) or `editor` (exported by Unreal Editor).
    pub method: String,
    pub notes: Option<String>,
    pub converted_at: String,
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
    /// A sibling in the same variant family. Computed, never stored as a link.
    Variant,
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
            LinkRel::Variant => "variant",
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
            "variant" => LinkRel::Variant,
            _ => return None,
        })
    }
    /// Whether a bundle download should include the target by default.
    pub fn is_dependency(&self) -> bool {
        matches!(self, LinkRel::Texture | LinkRel::Buffer | LinkRel::Material | LinkRel::Library | LinkRel::Guess)
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
    #[serde(default)]
    pub ext: Option<String>,
    /// "new" (default), "name", "size"
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
    /// Search the archive instead of the library: assets archived one by one, or with `pack`, everything archived
    /// from that pack.
    #[serde(default)]
    pub archived: bool,
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetPatch {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub add_tags: Vec<String>,
    #[serde(default)]
    pub remove_tags: Vec<String>,
    /// Apply the tag changes to every variant in the asset's family too.
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
}

/// A Hoard release newer than the running server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// e.g. `0.9.0`.
    pub version: String,
    /// The release page, with its notes.
    pub url: String,
}

/// Normalize a tag: lowercase, trimmed, spaces/underscores -> '-', optional `namespace:value`.
pub fn normalize_tag(raw: &str) -> Option<String> {
    let raw = raw.trim().to_lowercase();
    if raw.is_empty() {
        return None;
    }
    let norm_part = |s: &str| -> String {
        let mut out = String::with_capacity(s.len());
        let mut last_dash = false;
        for c in s.chars() {
            if c.is_alphanumeric() || c == '/' || c == '.' || c == '+' {
                out.push(c);
                last_dash = false;
            } else if !last_dash && !out.is_empty() {
                out.push('-');
                last_dash = true;
            }
        }
        out.trim_matches('-').to_owned()
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
    if tag.is_empty() || tag.len() > 64 { None } else { Some(tag) }
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
    }
}
