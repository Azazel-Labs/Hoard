//! Unreal packages and turning them into models.

use serde::{Deserialize, Serialize};

/// Why an Unreal package hasn't been converted.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UnrealStatus {
    /// `needs_editor`, `unsupported` or `failed`.
    pub status: String,
    pub class: Option<String>,
    pub reason: String,
}

/// Unreal conversion progress for a pack.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct PendingPackage {
    pub asset_id: String,
    pub pack_id: String,
    pub path: String,
    pub class: Option<String>,
    pub reason: String,
}

/// Options for converting a pack's Unreal packages.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UnrealConvertRequest {
    /// Discard each original package once its content is out.
    #[serde(default)]
    pub discard_originals: bool,
    /// Delete packages with no common format (Blueprints, levels, …).
    #[serde(default)]
    pub drop_unsupported: bool,
}
