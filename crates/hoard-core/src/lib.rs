//! Shared wire types between the Hoard server (`hoardd`) and clients (`hoard` CLI, engine plugins).
//!
//! Everything here is plain serde so an engine plugin in any language can mirror it from the JSON.

mod asset;
mod collections;
mod copies;
mod manifest;
mod pack;
mod search;
mod tags;
mod unreal;

pub use asset::*;
pub use collections::*;
pub use copies::*;
pub use manifest::*;
pub use pack::*;
pub use search::*;
pub use tags::*;
pub use unreal::*;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// The sizes above are from before the library last changed, and new ones are being worked out: ask again in a
    /// moment for them.
    #[serde(default)]
    pub sizes_refreshing: bool,
    /// The editors this build fetches itself when a conversion needs one (`blender`, `unreal`; the Docker image);
    /// elsewhere they are found where they are installed.
    #[serde(default)]
    pub editor_downloads: Vec<String>,
}

/// A Hoard release newer than the running server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UpdateInfo {
    /// e.g. `0.9.0`.
    pub version: String,
    /// The release page, with its notes.
    pub url: String,
}
