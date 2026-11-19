//! Unity materials (`.mat`), read into what a Hoard material needs: the textures each property points at (by the
//! GUID their `.meta` gives them), and the colors and numbers they're drawn with. Both of Unity's ways of saving
//! one are read: text (YAML, the default since Unity 2019) and the binary serialized file older projects saved,
//! through the type tree it carries. Only the material's own object is read; nothing is run.

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use hoard_core::MaterialLook;

use crate::materials::Role;

mod binary;

use binary::parse_binary;

/// A Unity material's saved properties.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct UnityMat {
    pub(crate) name: String,
    /// Shader keywords switched on (`_EMISSION`, `_NORMALMAP`).
    pub(crate) keywords: Vec<String>,
    /// Texture properties that point at a texture: (GUID of the texture's file, tiling, offset).
    pub(crate) textures: BTreeMap<String, (String, [f32; 2], [f32; 2])>,
    pub(crate) floats: BTreeMap<String, f32>,
    /// As saved: gamma-space RGBA.
    pub(crate) colors: BTreeMap<String, [f32; 4]>,
}

/// Where the material's smoothness comes from, when a map holds it: Unity keeps it in the alpha of the metallic
/// map, or of the color map, scaled by `_GlossMapScale`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SmoothnessIn {
    pub(crate) role: Role,
    pub(crate) scale: f32,
}

/// The texture properties of Unity's shaders (built-in Standard and legacy, URP Lit, HDRP Lit) as Hoard's roles,
/// the first of each role that's set winning.
const TEXTURES: &[(&str, Role)] = &[
    ("_BaseMap", Role::BaseColor),
    ("_BaseColorMap", Role::BaseColor),
    ("_MainTex", Role::BaseColor),
    ("_BumpMap", Role::Normal),
    ("_NormalMap", Role::Normal),
    ("_MetallicGlossMap", Role::Metalness),
    ("_SpecGlossMap", Role::Specular),
    ("_OcclusionMap", Role::Ao),
    ("_EmissionMap", Role::Emissive),
    ("_EmissiveColorMap", Role::Emissive),
    ("_ParallaxMap", Role::Height),
    ("_HeightMap", Role::Height),
];

/// Read a `.mat` file, text or binary.
pub(crate) fn parse(bytes: &[u8]) -> Result<UnityMat> {
    if bytes.starts_with(b"%YAML") { parse_text(&String::from_utf8_lossy(bytes)) } else { parse_binary(bytes) }
}

impl UnityMat {
    /// The textures by role, as GUIDs of their files.
    pub(crate) fn maps(&self) -> Vec<(Role, &str)> {
        let mut out: Vec<(Role, &str)> = Vec::new();
        for (prop, role) in TEXTURES {
            if let Some((guid, ..)) = self.textures.get(*prop)
                && !out.iter().any(|(r, _)| r == role)
            {
                out.push((*role, guid));
            }
        }
        out
    }

    fn float(&self, names: &[&str]) -> Option<f32> {
        names.iter().find_map(|n| self.floats.get(*n).copied()).filter(|v| v.is_finite())
    }

    fn color(&self, names: &[&str]) -> Option<[f32; 4]> {
        names.iter().find_map(|n| self.colors.get(*n).copied()).filter(|c| c.iter().all(|v| v.is_finite()))
    }

    /// A setting that's a whole number (a mode, a switch), as one.
    #[expect(clippy::cast_possible_truncation, reason = "modes and switches: small whole numbers")]
    fn whole(&self, names: &[&str]) -> Option<i32> {
        self.float(names).map(|v| v.round() as i32)
    }

    fn has(&self, role: Role) -> bool {
        self.maps().iter().any(|(r, _)| *r == role)
    }

    /// Where its smoothness is kept, when a map holds it rather than `_Glossiness`.
    pub(crate) fn smoothness_in(&self) -> Option<SmoothnessIn> {
        let scale = self.float(&["_GlossMapScale"]).unwrap_or(1.0).clamp(0.0, 1.0);
        if self.whole(&["_SmoothnessTextureChannel"]) == Some(1) && self.has(Role::BaseColor) {
            Some(SmoothnessIn { role: Role::BaseColor, scale })
        } else if self.has(Role::Metalness) && !self.floats.contains_key("_Smoothness") {
            Some(SmoothnessIn { role: Role::Metalness, scale })
        } else {
            None
        }
    }

    /// How the material draws its maps. `smoothness` is what the map named by [`Self::smoothness_in`] holds on
    /// average (its alpha), when there is one.
    pub(crate) fn look(&self, smoothness: Option<f32>) -> MaterialLook {
        let mut look = MaterialLook::default();
        if let Some([r, g, b, a]) = self.color(&["_BaseColor", "_Color"]) {
            look.base_color = Some([linear(r), linear(g), linear(b), a.clamp(0.0, 1.0)]);
        }
        if !self.has(Role::Metalness) {
            look.metallic = self.float(&["_Metallic"]).map(|m| m.clamp(0.0, 1.0));
        }
        let smooth = match self.smoothness_in() {
            Some(s) => smoothness.map(|v| v * s.scale),
            None => self.float(&["_Smoothness", "_Glossiness"]),
        };
        look.roughness = smooth.map(|s| 1.0 - s.clamp(0.0, 1.0));
        let glows = self.keywords.iter().any(|k| k == "_EMISSION") || self.colors.contains_key("_EmissiveColor");
        if glows && let Some([r, g, b, _]) = self.color(&["_EmissiveColor", "_EmissionColor"]) {
            let c = [linear(r), linear(g), linear(b)];
            if c.iter().any(|v| *v > 0.0) || self.has(Role::Emissive) {
                look.emissive = Some(c);
            }
        }
        let cutoff = self.float(&["_Cutoff", "_AlphaCutoff"]).map(|c| c.clamp(0.0, 1.0));
        // Standard's rendering mode (cutout, fade, transparent), or URP's and HDRP's surface type and alpha clip.
        let mode = match self.whole(&["_Mode"]) {
            Some(1) => Some("mask"),
            Some(2 | 3) => Some("blend"),
            _ if self.whole(&["_Surface", "_SurfaceType"]) == Some(1) => Some("blend"),
            _ if self.whole(&["_AlphaClip", "_AlphaCutoffEnable"]) == Some(1) => Some("mask"),
            _ => None,
        };
        look.alpha_mode = mode.map(str::to_owned);
        look.alpha_cutoff = cutoff.filter(|_| mode == Some("mask"));
        look.normal_scale =
            self.float(&["_BumpScale", "_NormalScale"]).filter(|s| not_one(*s) && self.has(Role::Normal));
        look.occlusion_strength = self.float(&["_OcclusionStrength"]).filter(|s| not_one(*s) && self.has(Role::Ao));
        let base = ["_BaseMap", "_BaseColorMap", "_MainTex"].iter().find_map(|p| self.textures.get(*p));
        if let Some((_, scale, offset)) = base {
            let fine = |v: &[f32; 2]| v.iter().all(|n| n.is_finite());
            look.uv_scale =
                Some(*scale).filter(|s| s.iter().any(|n| not_one(*n)) && fine(s) && s.iter().all(|n| n.abs() > 1e-6));
            look.uv_offset = Some(*offset).filter(|o| o.iter().any(|n| n.abs() > 1e-6) && fine(o));
        }
        look
    }
}

/// Whether a setting is other than 1, its usual value.
fn not_one(v: f32) -> bool {
    (v - 1.0).abs() > 1e-6
}

/// A gamma-space color channel as linear light (Unity saves colors as picked, in gamma space; HDR ones go past 1).
fn linear(c: f32) -> f32 {
    let c = c.max(0.0);
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

// ---------------------------------------------------------------------------------------------
// Text

/// `{fileID: 2800000, guid: 0123…, type: 3}` as its keys and values.
fn flow_map(s: &str) -> BTreeMap<&str, &str> {
    let inner = s.trim().trim_start_matches('{').trim_end_matches('}');
    inner.split(',').filter_map(|kv| kv.split_once(':')).map(|(k, v)| (k.trim(), v.trim())).collect()
}

fn num(map: &BTreeMap<&str, &str>, key: &str, or: f32) -> f32 {
    map.get(key).and_then(|v| v.parse().ok()).unwrap_or(or)
}

#[derive(PartialEq)]
enum Section {
    None,
    Keywords,
    Textures,
    Floats,
    Colors,
}

/// A texture property being read: its name, and its texture's GUID, tiling and offset so far.
type TexEntry = (String, Option<String>, [f32; 2], [f32; 2]);

fn finish(mat: &mut UnityMat, tex: &mut Option<TexEntry>) {
    if let Some((prop, Some(guid), scale, offset)) = tex.take() {
        mat.textures.insert(prop, (guid, scale, offset));
    }
}

fn parse_text(text: &str) -> Result<UnityMat> {
    let mut mat = UnityMat::default();
    // The material's own document: from `Material:` to the next `--- `.
    let start = text.lines().position(|l| l.trim_end() == "Material:").context("no material in this file")?;
    let mut section = Section::None;
    let mut indent = 0;
    let mut tex: Option<TexEntry> = None;
    for line in text.lines().skip(start + 1) {
        if line.starts_with("--- ") {
            break;
        }
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let here = line.len() - trimmed.len();
        if section != Section::None && here <= indent && !trimmed.starts_with("- ") {
            finish(&mut mat, &mut tex);
            section = Section::None;
        }
        if section == Section::None {
            let (key, value) = trimmed.split_once(':').unwrap_or((trimmed, ""));
            let value = value.trim();
            indent = here;
            match key {
                "m_Name" if here <= 2 => value.clone_into(&mut mat.name),
                "m_ShaderKeywords" => mat.keywords.extend(value.split_whitespace().map(str::to_owned)),
                "m_ValidKeywords" if value.is_empty() => section = Section::Keywords,
                "m_TexEnvs" if value.is_empty() => section = Section::Textures,
                "m_Floats" if value.is_empty() => section = Section::Floats,
                "m_Colors" if value.is_empty() => section = Section::Colors,
                _ => {}
            }
            continue;
        }
        match section {
            Section::Keywords => {
                if let Some(k) = trimmed.strip_prefix("- ") {
                    mat.keywords.push(k.trim().to_owned());
                }
            }
            Section::Textures => {
                if let Some(entry) = trimmed.strip_prefix("- ") {
                    finish(&mut mat, &mut tex);
                    let prop = entry.trim().trim_end_matches(':');
                    tex = Some((prop.to_owned(), None, [1.0, 1.0], [0.0, 0.0]));
                } else if let Some((_, guid, scale, offset)) = tex.as_mut() {
                    let (key, value) = trimmed.split_once(':').unwrap_or((trimmed, ""));
                    let map = flow_map(value);
                    match key {
                        "m_Texture" => {
                            let file = map.get("fileID").copied().unwrap_or("0");
                            *guid =
                                map.get("guid").filter(|g| file != "0" && is_guid(g)).map(|g| g.to_ascii_lowercase());
                        }
                        "m_Scale" => *scale = [num(&map, "x", 1.0), num(&map, "y", 1.0)],
                        "m_Offset" => *offset = [num(&map, "x", 0.0), num(&map, "y", 0.0)],
                        _ => {}
                    }
                }
            }
            Section::Floats => {
                if let Some((k, v)) = trimmed.strip_prefix("- ").and_then(|e| e.split_once(':'))
                    && let Ok(v) = v.trim().parse::<f32>()
                {
                    mat.floats.insert(k.trim().to_owned(), v);
                }
            }
            Section::Colors => {
                if let Some((k, v)) = trimmed.strip_prefix("- ").and_then(|e| e.split_once(':')) {
                    let map = flow_map(v);
                    let c = [num(&map, "r", 0.0), num(&map, "g", 0.0), num(&map, "b", 0.0), num(&map, "a", 1.0)];
                    mat.colors.insert(k.trim().to_owned(), c);
                }
            }
            Section::None => {}
        }
    }
    finish(&mut mat, &mut tex);
    Ok(mat)
}

fn is_guid(g: &str) -> bool {
    g.len() == 32 && g.bytes().all(|b| b.is_ascii_hexdigit()) && !g.bytes().all(|b| b == b'0')
}

#[cfg(test)]
mod tests;
