//! What a project takes from the library, and the credits it owes.

use crate::{AssetKind, SearchQuery};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request a flat, engine-friendly list of files: either explicit ids, a search, or both.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Manifest {
    pub generated_at: String,
    pub entries: Vec<ManifestEntry>,
}
