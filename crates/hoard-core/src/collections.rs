//! Collections people put assets in.

use serde::{Deserialize, Serialize};

/// A named set of assets put together by hand ("Forest level props"), across packs.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
pub struct CollectionRef {
    pub id: String,
    pub name: String,
}

/// Create or rename a collection.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CollectionPatch {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Add assets to, or take them out of, a collection.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CollectionMembers {
    #[serde(default)]
    pub add: Vec<String>,
    #[serde(default)]
    pub remove: Vec<String>,
}
