//! Copies of an asset: families of formats, exact duplicates and near copies.

use crate::{AssetKind, AssetLink};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Family {
    /// Shared base, e.g. `Sounds/whoosh`.
    pub key: String,
    /// Total members, including this asset.
    pub size: u64,
    /// Up to 60 other members.
    pub members: Vec<AssetLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Duplicate {
    pub id: String,
    pub name: String,
    pub pack_id: String,
    pub pack_name: String,
    pub path: String,
}

/// The same picture, sound or model as an asset, in another file: another size, rate or format. Detail view and the
/// duplicates page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
pub struct CopyGroup {
    /// Bytes taken by the copies that aren't the best.
    pub wasted: i64,
    pub copies: Vec<NearCopy>,
}

/// A page of the library's near copies, the most space taken first.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
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
