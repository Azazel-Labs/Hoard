//! Packs: what came in together, and how its models face.

use serde::{Deserialize, Serialize};

/// Where a pack came from and what it is, as written by its seller. The LLM tagger sees this with every file,
/// because a store page saying "120 ice and fire spells" explains `spell35.wav` better than the file can.
#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// How many of the pack's assets have a license of their own instead of the pack's (read-only).
    #[serde(default, skip_deserializing)]
    pub own_licenses: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// Up to twelve of the pack's pictures of itself (its previews, screenshots and other marketing shots), for a
    /// cover that goes from one to the next. Empty when it has none, and the cover is `preview`'s mosaic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub showcase: Vec<String>,
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
