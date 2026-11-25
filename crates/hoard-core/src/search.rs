//! Searching the library.

use crate::Asset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// "name", "size" or "rating", each its usual way round (newest, A to Z, biggest and best rated first) or with
    /// ":asc" or ":desc" after it.
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
    /// Only what's rated this many stars (1 to 5) or more.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating_min: Option<u8>,
    /// Only this account's favorites. Set by the server for whoever asks, never sent.
    #[serde(skip)]
    pub favorites_of: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Facet {
    pub value: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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
