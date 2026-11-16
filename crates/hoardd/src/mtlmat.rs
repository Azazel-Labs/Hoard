//! Wavefront materials (`.mtl`, the file beside an OBJ), read into what a Hoard material needs: each `newmtl`'s maps
//! by role and the colors and numbers it's drawn with, read the way polysmelt reads them for the OBJ, so the material
//! and the model it's on look alike. A material with only a color (as SketchUp exports them) becomes a swatch.

use hoard_core::MaterialLook;

use crate::materials::{Role, parse_map};

/// One `newmtl`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct MtlMat {
    pub(crate) name: String,
    /// The files it names, as written, by role: the first of each role wins.
    pub(crate) maps: Vec<(Role, String)>,
    pub(crate) look: MaterialLook,
}

/// What a material statement sets, as read so far.
#[derive(Default)]
struct Entry {
    name: String,
    color: Option<[f32; 3]>,
    alpha: Option<f32>,
    emissive: Option<[f32; 3]>,
    roughness: Option<f32>,
    metallic: Option<f32>,
    shininess: Option<f32>,
    maps: Vec<(Role, String)>,
    scale: Option<[f32; 2]>,
    offset: Option<[f32; 2]>,
}

/// The materials of an MTL file, in the order written. One with neither a color nor a map says nothing to draw and
/// is left out.
pub(crate) fn parse(text: &str) -> Vec<MtlMat> {
    let mut out = Vec::new();
    let mut cur: Option<Entry> = None;
    for line in text.lines() {
        let line = line.trim();
        let (key, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
        let rest = rest.trim();
        let key = key.to_ascii_lowercase();
        if key == "newmtl" {
            out.extend(cur.take().and_then(finish));
            cur = Some(Entry { name: rest.to_owned(), ..Entry::default() });
            continue;
        }
        let Some(e) = cur.as_mut() else { continue };
        let nums: Vec<f32> = rest.split_whitespace().map_while(|w| w.parse().ok()).collect();
        let rgb = || match nums[..] {
            [r, g, b, ..] => Some([r, g, b]),
            [v] => Some([v; 3]),
            _ => None,
        };
        match key.as_str() {
            "kd" => e.color = rgb().or(e.color),
            "ke" => e.emissive = rgb().or(e.emissive),
            "d" if !nums.is_empty() => e.alpha = Some(nums[0]),
            "tr" if !nums.is_empty() => e.alpha = Some(1.0 - nums[0]),
            "pr" if !nums.is_empty() => e.roughness = Some(nums[0]),
            "pm" if !nums.is_empty() => e.metallic = Some(nums[0]),
            "ns" if !nums.is_empty() => e.shininess = Some(nums[0]),
            _ => {
                let Some(role) = map_role(&key) else { continue };
                let (file, scale, offset) = map_ref(rest);
                if file.is_empty() || e.maps.iter().any(|(r, _)| *r == role) {
                    continue;
                }
                // A bump map named as a height or bump map is one; otherwise it's taken as polysmelt takes it, a
                // normal map.
                let role = match role {
                    Role::Normal => parse_map(stem(&file))
                        .map(|m| m.role)
                        .filter(|r| matches!(r, Role::Height | Role::Bump | Role::NormalDx))
                        .unwrap_or(Role::Normal),
                    r => r,
                };
                if role == Role::BaseColor {
                    (e.scale, e.offset) = (scale, offset);
                }
                e.maps.push((role, file));
            }
        }
    }
    out.extend(cur.and_then(finish));
    out
}

fn map_role(key: &str) -> Option<Role> {
    Some(match key {
        "map_kd" => Role::BaseColor,
        "map_bump" | "bump" | "norm" | "map_kn" => Role::Normal,
        "map_pr" => Role::Roughness,
        "map_pm" => Role::Metalness,
        "map_ke" => Role::Emissive,
        "map_d" => Role::Opacity,
        "map_ks" => Role::Specular,
        _ => return None,
    })
}

fn stem(file: &str) -> &str {
    let name = file.rsplit(['/', '\\']).next().unwrap_or(file);
    name.rsplit_once('.').map_or(name, |(s, _)| s)
}

/// A map statement's file name (which may hold spaces) after its options, and its `-s` scale and `-o` offset.
fn map_ref(rest: &str) -> (String, Option<[f32; 2]>, Option<[f32; 2]>) {
    let words: Vec<&str> = rest.split_whitespace().collect();
    let (mut scale, mut offset) = (None, None);
    let mut i = 0;
    while i < words.len() && words[i].starts_with('-') {
        let opt = words[i].to_ascii_lowercase();
        i += 1;
        let numbers = |i: &mut usize, most: usize| {
            let start = *i;
            while *i < words.len() && *i - start < most && words[*i].parse::<f32>().is_ok() {
                *i += 1;
            }
            words[start..*i].iter().filter_map(|w| w.parse::<f32>().ok()).collect::<Vec<_>>()
        };
        match opt.as_str() {
            "-s" | "-o" | "-t" => {
                let n = numbers(&mut i, 3);
                let uv = match n[..] {
                    [u, v, ..] => Some([u, v]),
                    [u] => Some([u, u]),
                    [] => None,
                };
                match opt.as_str() {
                    "-s" => scale = uv,
                    "-o" => offset = uv,
                    _ => {}
                }
            }
            "-mm" => {
                numbers(&mut i, 2);
            }
            "-bm" | "-boost" | "-texres" => {
                numbers(&mut i, 1);
            }
            // on / off, or a channel letter.
            "-blendu" | "-blendv" | "-cc" | "-clamp" | "-imfchan" | "-type" => i += 1,
            _ => {}
        }
    }
    (words[i.min(words.len())..].join(" "), scale, offset)
}

fn finish(e: Entry) -> Option<MtlMat> {
    if e.color.is_none() && e.maps.is_empty() {
        return None;
    }
    let mut look = MaterialLook::default();
    let alpha = e.alpha.map_or(1.0, |a| a.clamp(0.0, 1.0));
    if let Some([r, g, b]) = e.color {
        look.base_color = Some([linear(r), linear(g), linear(b), alpha]);
    }
    if alpha < 1.0 {
        look.alpha_mode = Some("blend".into());
    }
    if let Some(c) = e.emissive.filter(|c| c.iter().any(|v| *v > 0.0)) {
        look.emissive = Some(c.map(linear));
    }
    let has = |role: Role| e.maps.iter().any(|(r, _)| *r == role);
    if !has(Role::Metalness) {
        look.metallic = e.metallic.map(|m| m.clamp(0.0, 1.0)).filter(|m| *m > 0.0);
    }
    if !has(Role::Roughness) {
        // Blender's reading of the specular exponent when there's no roughness.
        look.roughness = e
            .roughness
            .or(e.shininess.map(|ns| 1.0 - (ns.clamp(0.0, 1000.0) / 1000.0).sqrt()))
            .map(|r| r.clamp(0.0, 1.0));
    }
    let fine = |v: &[f32; 2]| v.iter().all(|n| n.is_finite());
    look.uv_scale =
        e.scale.filter(|s| fine(s) && s.iter().any(|n| (n - 1.0).abs() > 1e-6) && s.iter().all(|n| n.abs() > 1e-6));
    look.uv_offset = e.offset.filter(|o| fine(o) && o.iter().any(|n| n.abs() > 1e-6));
    Some(MtlMat { name: e.name, maps: e.maps, look })
}

/// An MTL color channel (written as picked, in gamma space) as linear light, as polysmelt reads it.
fn linear(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sketchup_colors_become_swatches() {
        let mats = parse(
            "# Exported from SketchUp\n\nnewmtl Asphalt\nKa 0 0 0\nKd 0.411765 0.411765 0.411765\nKs 0.33 0.33 0.33\n\n\
             newmtl Stone\nKd 0.847059 0.827451 0.749020\n\nnewmtl Empty\nKa 0 0 0\n",
        );
        assert_eq!(mats.iter().map(|m| m.name.as_str()).collect::<Vec<_>>(), ["Asphalt", "Stone"]);
        let c = mats[0].look.base_color.unwrap();
        assert!((c[0] - 0.141).abs() < 0.01 && (c[3] - 1.0).abs() < 1e-6, "{c:?}");
        assert_eq!(mats[0].maps.len(), 0);
        assert_eq!(mats[0].look.alpha_mode, None);
    }

    #[test]
    fn maps_options_and_numbers() {
        let mats = parse(
            "newmtl Glass Door\nKd 1 1 1\nd 0.5\nNs 250\nKe 0 0 0\nmap_Kd -s 4 2 1 -o 0.5 0 textures/door wood.png\n\
             map_Bump -bm 0.5 door_n.png\nbump door_height.png\nmap_d door_a.png\n",
        );
        let m = &mats[0];
        assert_eq!(m.name, "Glass Door");
        assert_eq!(
            m.maps,
            [
                (Role::BaseColor, "textures/door wood.png".to_owned()),
                (Role::Normal, "door_n.png".to_owned()),
                (Role::Opacity, "door_a.png".to_owned()),
            ]
        );
        assert_eq!(m.look.uv_scale, Some([4.0, 2.0]));
        assert_eq!(m.look.uv_offset, Some([0.5, 0.0]));
        assert_eq!(m.look.alpha_mode.as_deref(), Some("blend"));
        assert!((m.look.roughness.unwrap() - 0.5).abs() < 1e-6);
        assert_eq!(m.look.emissive, None);
        let bump = parse("newmtl B\nbump -bm 2 rock_height.png\n");
        assert_eq!(bump[0].maps, [(Role::Height, "rock_height.png".to_owned())]);
    }
}
