//! Minimal Wavefront OBJ + MTL loader for thumbnails: positions, UVs, normals, `usemtl` groups, `Kd` color and
//! `map_Kd` texture. Polygons are fan-triangulated. Anything fancier is the browser viewer's job.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use glam::Vec3;

use crate::{Draw, Vertex};

#[derive(Debug, Default, Clone)]
struct Material {
    color: [f32; 4],
    texture: Option<PathBuf>,
}

pub(crate) fn load(path: &Path) -> Result<(Vec<Draw>, Vec<Option<image::RgbaImage>>)> {
    let text = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let dir = path.parent().unwrap_or(Path::new("."));
    let (mut pos, mut uv, mut nrm) = (Vec::<Vec3>::new(), Vec::<[f32; 2]>::new(), Vec::<Vec3>::new());
    let mut materials: HashMap<String, Material> = HashMap::new();
    // Faces grouped by material name, in first-use order.
    let mut groups: Vec<(String, Vec<Vertex>)> = vec![(String::new(), Vec::new())];

    let idx = |s: &str, len: usize| -> Option<usize> {
        let i: i64 = s.parse().ok()?;
        let i = if i < 0 { len as i64 + i } else { i - 1 };
        usize::try_from(i).ok().filter(|&i| i < len)
    };
    for line in text.lines() {
        let mut it = line.split_whitespace();
        let Some(tag) = it.next() else { continue };
        let nums = |it: std::str::SplitWhitespace<'_>| -> Vec<f32> { it.filter_map(|v| v.parse().ok()).collect() };
        match tag {
            "v" => {
                let v = nums(it);
                if v.len() >= 3 {
                    pos.push(Vec3::new(v[0], v[1], v[2]));
                }
            }
            "vt" => {
                let v = nums(it);
                if v.len() >= 2 {
                    uv.push([v[0], 1.0 - v[1]]);
                }
            }
            "vn" => {
                let v = nums(it);
                if v.len() >= 3 {
                    nrm.push(Vec3::new(v[0], v[1], v[2]).normalize_or_zero());
                }
            }
            "mtllib" => {
                let rest = line[6..].trim();
                if let Ok(mtl) = std::fs::read_to_string(dir.join(rest.replace('\\', "/"))) {
                    parse_mtl(&mtl, dir, &mut materials);
                }
            }
            "usemtl" => {
                let name = line[6..].trim().to_owned();
                if !groups.iter().any(|(n, _)| *n == name) {
                    groups.push((name.clone(), Vec::new()));
                }
                // Move the active group to the end so pushes go to it.
                let at = groups.iter().position(|(n, _)| *n == name).unwrap_or(0);
                let g = groups.remove(at);
                groups.push(g);
            }
            "f" => {
                let corners: Vec<Vertex> = it
                    .filter_map(|c| {
                        let mut parts = c.split('/');
                        let p = idx(parts.next()?, pos.len())?;
                        let t = parts.next().and_then(|s| idx(s, uv.len()));
                        let n = parts.next().and_then(|s| idx(s, nrm.len()));
                        Some(Vertex {
                            pos: pos[p].to_array(),
                            uv: t.map_or([0.0, 0.0], |t| uv[t]),
                            normal: n.map_or(Vec3::ZERO, |n| nrm[n]).to_array(),
                        })
                    })
                    .collect();
                let Some(active) = groups.last_mut() else { continue };
                for i in 1..corners.len().saturating_sub(1) {
                    active.1.extend([corners[0], corners[i], corners[i + 1]]);
                }
            }
            _ => {}
        }
    }

    let mut images: Vec<Option<image::RgbaImage>> = Vec::new();
    let mut image_index: HashMap<PathBuf, usize> = HashMap::new();
    let mut draws = Vec::new();
    for (name, mut verts) in groups {
        if verts.is_empty() {
            continue;
        }
        // Missing normals: compute flat ones per triangle.
        for tri in verts.as_chunks_mut::<3>().0 {
            if tri.iter().all(|v| v.normal == [0.0, 0.0, 0.0]) {
                let [a, b, c] = [Vec3::from(tri[0].pos), Vec3::from(tri[1].pos), Vec3::from(tri[2].pos)];
                let n = (b - a).cross(c - a).normalize_or_zero().to_array();
                for v in tri.iter_mut() {
                    v.normal = n;
                }
            }
        }
        let mat = materials.get(&name).cloned().unwrap_or(Material { color: [0.8, 0.8, 0.8, 1.0], texture: None });
        let texture = mat.texture.and_then(|p| {
            if let Some(&i) = image_index.get(&p) {
                return Some(i);
            }
            let img = image::open(&p).ok()?.to_rgba8();
            images.push(Some(img));
            image_index.insert(p, images.len() - 1);
            Some(images.len() - 1)
        });
        let indices = (0..verts.len() as u32).collect();
        draws.push(Draw { vertices: verts, indices, color: mat.color, texture });
    }
    Ok((draws, images))
}

fn parse_mtl(text: &str, dir: &Path, out: &mut HashMap<String, Material>) {
    let mut current: Option<String> = None;
    for line in text.lines() {
        let line = line.trim();
        let mut it = line.split_whitespace();
        match it.next() {
            Some("newmtl") => {
                let name = line[6..].trim().to_owned();
                out.insert(name.clone(), Material { color: [0.8, 0.8, 0.8, 1.0], texture: None });
                current = Some(name);
            }
            Some("Kd") => {
                let v: Vec<f32> = it.filter_map(|v| v.parse().ok()).collect();
                if let (Some(m), [r, g, b, ..]) = (current.as_ref().and_then(|c| out.get_mut(c)), v.as_slice()) {
                    m.color = [*r, *g, *b, 1.0];
                }
            }
            Some("map_Kd") => {
                // Options like `-s 1 1 1` can precede the file name; the name is the last token.
                if let (Some(m), Some(file)) =
                    (current.as_ref().and_then(|c| out.get_mut(c)), line.split_whitespace().last())
                {
                    m.texture = Some(dir.join(file.replace('\\', "/")));
                    m.color = [1.0, 1.0, 1.0, 1.0];
                }
            }
            _ => {}
        }
    }
}
