//! Autodesk 3DS, read here rather than by Blender, whose 3DS importer has been an extension (not shipped) since 4.2.
//! A 3DS file is nested chunks (`u16` id, `u32` length counting its 6-byte header). The meshes are under the editor
//! chunk, their vertices already in world space; materials have a diffuse color, a transparency and maybe a texture
//! named by file. Smoothing groups decide which faces share normals. 3DS is Z up; glTF is Y up.

#![expect(clippy::many_single_char_names, reason = "geometry code: a/b/c, u/v")]

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Result, ensure};

use crate::meshes::{GRAY, Image, MAX_TRIANGLES, Material, Mesh, Prim};

const MAIN: u16 = 0x4D4D;
const EDITOR: u16 = 0x3D3D;
const OBJECT: u16 = 0x4000;
const TRIMESH: u16 = 0x4100;
const VERTICES: u16 = 0x4110;
const FACES: u16 = 0x4120;
const FACE_MATERIAL: u16 = 0x4130;
const UVS: u16 = 0x4140;
const SMOOTHING: u16 = 0x4150;
const MATERIAL: u16 = 0xAFFF;
const MAT_NAME: u16 = 0xA000;
const MAT_DIFFUSE: u16 = 0xA020;
const MAT_TRANSPARENCY: u16 = 0xA050;
const MAT_TEXTURE: u16 = 0xA200;
const MAP_FILE: u16 = 0xA300;
const COLOR_F32: u16 = 0x0010;
const COLOR_U8: u16 = 0x0011;
const COLOR_U8_GAMMA: u16 = 0x0012;
const COLOR_F32_GAMMA: u16 = 0x0013;
const PERCENT_U16: u16 = 0x0030;
const PERCENT_F32: u16 = 0x0031;

/// The chunks in `data`, as (id, body). Stops at one that runs past the end.
fn chunks(mut data: &[u8]) -> impl Iterator<Item = (u16, &[u8])> {
    std::iter::from_fn(move || {
        let id = u16::from_le_bytes(data.get(..2)?.try_into().ok()?);
        let len = u32::from_le_bytes(data.get(2..6)?.try_into().ok()?) as usize;
        if len < 6 || len > data.len() {
            return None;
        }
        let body = &data[6..len];
        data = &data[len..];
        Some((id, body))
    })
}

fn u16_at(d: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_le_bytes(d.get(i..i + 2)?.try_into().ok()?))
}

fn f32_at(d: &[u8], i: usize) -> Option<f32> {
    Some(f32::from_le_bytes(d.get(i..i + 4)?.try_into().ok()?))
}

/// A zero-terminated name and what follows it.
fn cstr(d: &[u8]) -> (String, &[u8]) {
    let end = d.iter().position(|&c| c == 0).unwrap_or(d.len());
    (String::from_utf8_lossy(&d[..end]).into_owned(), d.get(end + 1..).unwrap_or_default())
}

/// A color chunk's value as linear RGB.
fn color(d: &[u8]) -> Option<[f32; 3]> {
    let mut best = None;
    for (id, b) in chunks(d) {
        let c = match id {
            COLOR_U8 | COLOR_U8_GAMMA if b.len() >= 3 => {
                [b[0], b[1], b[2]].map(|v| crate::meshes::srgb_to_linear(f32::from(v) / 255.0))
            }
            COLOR_F32 | COLOR_F32_GAMMA => [f32_at(b, 0)?, f32_at(b, 4)?, f32_at(b, 8)?],
            _ => continue,
        };
        // The plain 8-bit color is what most tools mean; take it over the others.
        if id == COLOR_U8 || best.is_none() {
            best = Some(c);
        }
    }
    best
}

fn percent(d: &[u8]) -> Option<f32> {
    chunks(d).find_map(|(id, b)| match id {
        PERCENT_U16 => u16_at(b, 0).map(|v| f32::from(v) / 100.0),
        PERCENT_F32 => f32_at(b, 0).map(|v| v / 100.0),
        _ => None,
    })
}

struct Mat {
    name: String,
    color: [f32; 3],
    alpha: f32,
    texture: Option<String>,
}

fn material(d: &[u8]) -> Mat {
    let mut m = Mat { name: String::new(), color: [GRAY[0], GRAY[1], GRAY[2]], alpha: 1.0, texture: None };
    for (id, b) in chunks(d) {
        match id {
            MAT_NAME => m.name = cstr(b).0,
            MAT_DIFFUSE => m.color = color(b).unwrap_or(m.color),
            MAT_TRANSPARENCY => m.alpha = 1.0 - percent(b).unwrap_or(0.0).clamp(0.0, 1.0),
            MAT_TEXTURE => {
                m.texture = chunks(b).find(|&(id, _)| id == MAP_FILE).map(|(_, f)| cstr(f).0).filter(|f| !f.is_empty());
            }
            _ => {}
        }
    }
    m
}

#[derive(Default)]
struct Trimesh {
    vertices: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    faces: Vec<[u16; 3]>,
    smoothing: Vec<u32>,
    /// Material name → its faces.
    materials: Vec<(String, Vec<u16>)>,
}

fn trimesh(d: &[u8]) -> Trimesh {
    let mut t = Trimesh::default();
    for (id, b) in chunks(d) {
        match id {
            VERTICES => {
                let n = u16_at(b, 0).unwrap_or(0) as usize;
                t.vertices = (0..n)
                    .map_while(|i| Some([f32_at(b, 2 + i * 12)?, f32_at(b, 6 + i * 12)?, f32_at(b, 10 + i * 12)?]))
                    .collect();
            }
            UVS => {
                let n = u16_at(b, 0).unwrap_or(0) as usize;
                t.uvs = (0..n).map_while(|i| Some([f32_at(b, 2 + i * 8)?, f32_at(b, 6 + i * 8)?])).collect();
            }
            FACES => {
                let n = u16_at(b, 0).unwrap_or(0) as usize;
                t.faces = (0..n)
                    .map_while(|i| {
                        let o = 2 + i * 8;
                        Some([u16_at(b, o)?, u16_at(b, o + 2)?, u16_at(b, o + 4)?])
                    })
                    .collect();
                for (id, s) in chunks(b.get(2 + n * 8..).unwrap_or_default()) {
                    match id {
                        FACE_MATERIAL => {
                            let (name, rest) = cstr(s);
                            let k = u16_at(rest, 0).unwrap_or(0) as usize;
                            let faces = (0..k).map_while(|i| u16_at(rest, 2 + i * 2)).collect();
                            t.materials.push((name, faces));
                        }
                        SMOOTHING => {
                            t.smoothing = s.as_chunks::<4>().0.iter().take(n).map(|c| u32::from_le_bytes(*c)).collect();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    t
}

/// Z up to Y up.
fn y_up(p: [f32; 3]) -> [f32; 3] {
    [p[0], p[2], -p[1]]
}

pub(crate) fn load(path: &Path, root: &Path) -> Result<Mesh> {
    let data = std::fs::read(path)?;
    let main = chunks(&data).find(|&(id, _)| id == MAIN).map(|(_, b)| b);
    let Some(main) = main else { anyhow::bail!("not a 3DS file") };
    let editor = chunks(main).find(|&(id, _)| id == EDITOR).map_or(&[][..], |(_, b)| b);

    let mut mesh = Mesh::default();
    let mut by_name: HashMap<String, usize> = HashMap::new();
    let dir = path.parent().unwrap_or(root);
    let mut images: HashMap<String, Option<usize>> = HashMap::new();
    for (_, b) in chunks(editor).filter(|&(id, _)| id == MATERIAL) {
        let m = material(b);
        let image = m.texture.as_ref().and_then(|f| {
            *images.entry(f.to_lowercase()).or_insert_with(|| {
                let img: Image =
                    crate::meshes::find_texture(f, dir, root).and_then(|p| crate::meshes::load_image(&p))?;
                mesh.images.push(img);
                Some(mesh.images.len() - 1)
            })
        });
        // A texture shows as painted, not tinted by the color under it.
        let rgb = if image.is_some() { [1.0; 3] } else { m.color };
        by_name.insert(m.name.clone(), mesh.materials.len());
        mesh.materials
            .push(Material { color_image: image, ..Material::plain(&m.name, [rgb[0], rgb[1], rgb[2], m.alpha]) });
    }

    // One prim per material, across every object.
    let mut prims: HashMap<usize, Prim> = HashMap::new();
    let mut triangles = 0usize;
    for (_, obj) in chunks(editor).filter(|&(id, _)| id == OBJECT) {
        let rest = cstr(obj).1;
        for (_, tri) in chunks(rest).filter(|&(id, _)| id == TRIMESH) {
            let t = trimesh(tri);
            triangles += t.faces.len();
            ensure!(triangles <= MAX_TRIANGLES, "more than {MAX_TRIANGLES} triangles");
            let mut face_mat = vec![None; t.faces.len()];
            for (name, faces) in &t.materials {
                let m = by_name.get(name).copied();
                for &f in faces {
                    if let Some(slot) = face_mat.get_mut(f as usize) {
                        *slot = m;
                    }
                }
            }
            add_faces(&t, &face_mat, &mut prims, &mut mesh.materials);
        }
    }
    ensure!(triangles > 0, "this 3DS file has no meshes");
    let mut prims: Vec<Prim> = prims.into_values().filter(|p| !p.indices.is_empty()).collect();
    prims.sort_by_key(|p| p.material);
    mesh.prims = prims;
    Ok(mesh)
}

/// A trimesh's faces, into the prims of their materials. Face corners share a vertex when they share a position
/// and a smoothing group; a face in no group gets its own, so it shows flat.
fn add_faces(t: &Trimesh, face_mat: &[Option<usize>], prims: &mut HashMap<usize, Prim>, materials: &mut Vec<Material>) {
    let pos: Vec<[f32; 3]> = t.vertices.iter().map(|&p| y_up(p)).collect();
    let has_uvs = t.uvs.len() == t.vertices.len() && !t.uvs.is_empty();
    // (prim material, vertex, smoothing group or a face of its own) → index in the prim.
    let mut seen: HashMap<(usize, u16, u64), u32> = HashMap::new();
    let mut sums: HashMap<(usize, u32), [f32; 3]> = HashMap::new();
    for (f, face) in t.faces.iter().enumerate() {
        let Some(corners) = face.iter().map(|&i| pos.get(i as usize).copied()).collect::<Option<Vec<_>>>() else {
            continue;
        };
        let (a, b, c) = (corners[0], corners[1], corners[2]);
        let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        let mat = face_mat[f].unwrap_or_else(|| {
            if materials.last().is_none_or(|m| m.name != "default") {
                materials.push(Material::plain("default", GRAY));
            }
            materials.len() - 1
        });
        let prim = prims.entry(mat).or_insert_with(|| Prim { material: mat, ..Prim::default() });
        let group = match t.smoothing.get(f) {
            Some(&g) if g != 0 => u64::from(g),
            _ => (1 << 32) | f as u64,
        };
        for &vi in face {
            let idx = *seen.entry((mat, vi, group)).or_insert_with(|| {
                prim.positions.push(pos[vi as usize]);
                if has_uvs {
                    let [s, tv] = t.uvs[vi as usize];
                    prim.uvs.push([s, 1.0 - tv]);
                }
                (prim.positions.len() - 1) as u32
            });
            let sum = sums.entry((mat, idx)).or_insert([0.0; 3]);
            for k in 0..3 {
                sum[k] += n[k];
            }
            prim.indices.push(idx);
        }
    }
    for ((mat, idx), n) in sums {
        let prim = prims.get_mut(&mat).expect("prim made above");
        if prim.normals.len() < prim.positions.len() {
            prim.normals.resize(prim.positions.len(), [0.0, 1.0, 0.0]);
        }
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if len > 0.0 {
            prim.normals[idx as usize] = [n[0] / len, n[1] / len, n[2] / len];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(id: u16, body: &[u8]) -> Vec<u8> {
        let mut out = id.to_le_bytes().to_vec();
        out.extend(u32::try_from(body.len() + 6).unwrap().to_le_bytes());
        out.extend(body);
        out
    }

    fn f32s(xs: &[f32]) -> Vec<u8> {
        xs.iter().flat_map(|x| x.to_le_bytes()).collect()
    }

    /// A square on the ground (Z up) of two triangles in one smoothing group, red, and a lone triangle standing up
    /// in no material.
    fn square() -> Vec<u8> {
        let mut mat = chunk(MAT_NAME, b"Red\0");
        mat.extend(chunk(MAT_DIFFUSE, &chunk(COLOR_U8, &[255, 0, 0])));
        mat.extend(chunk(MAT_TRANSPARENCY, &chunk(PERCENT_U16, &25u16.to_le_bytes())));

        let mut verts = 4u16.to_le_bytes().to_vec();
        verts.extend(f32s(&[0., 0., 0., 2., 0., 0., 2., 2., 0., 0., 2., 0.]));
        let mut uvs = 4u16.to_le_bytes().to_vec();
        uvs.extend(f32s(&[0., 0., 1., 0., 1., 1., 0., 1.]));
        let mut faces = 2u16.to_le_bytes().to_vec();
        for f in [[0u16, 1, 2, 7], [0, 2, 3, 7]] {
            faces.extend(f.iter().flat_map(|x| x.to_le_bytes()));
        }
        let mut fm = b"Red\0".to_vec();
        fm.extend(2u16.to_le_bytes());
        fm.extend([0u16, 1].iter().flat_map(|x| x.to_le_bytes()));
        faces.extend(chunk(FACE_MATERIAL, &fm));
        faces.extend(chunk(SMOOTHING, &[1u32, 1].iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<_>>()));
        let mut tri = chunk(VERTICES, &verts);
        tri.extend(chunk(UVS, &uvs));
        tri.extend(chunk(FACES, &faces));
        let mut obj = b"Floor\0".to_vec();
        obj.extend(chunk(TRIMESH, &tri));

        let mut verts = 3u16.to_le_bytes().to_vec();
        verts.extend(f32s(&[0., 0., 0., 1., 0., 0., 0., 0., 1.]));
        let mut faces = 1u16.to_le_bytes().to_vec();
        faces.extend([0u16, 1, 2, 0].iter().flat_map(|x| x.to_le_bytes()));
        let mut tri = chunk(VERTICES, &verts);
        tri.extend(chunk(FACES, &faces));
        let mut wall = b"Wall\0".to_vec();
        wall.extend(chunk(TRIMESH, &tri));

        let mut editor = chunk(MATERIAL, &mat);
        editor.extend(chunk(OBJECT, &obj));
        editor.extend(chunk(OBJECT, &wall));
        chunk(MAIN, &chunk(EDITOR, &editor))
    }

    #[test]
    fn meshes_materials_and_smoothing() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("floor.3ds");
        std::fs::write(&p, square()).unwrap();
        let mesh = load(&p, dir.path()).unwrap();
        assert_eq!(mesh.triangles(), 3);
        assert_eq!(mesh.prims.len(), 2);
        let floor = &mesh.prims[0];
        let red = &mesh.materials[floor.material];
        assert_eq!(red.name, "Red");
        assert_eq!(red.color, [1.0, 0.0, 0.0, 0.75]);
        // One smoothing group: the shared corners are shared vertices.
        assert_eq!(floor.positions.len(), 4);
        assert!(floor.positions.iter().all(|p| p[1] == 0.0), "Z up became Y up: {:?}", floor.positions);
        assert!(floor.positions.contains(&[2.0, 0.0, -2.0]));
        assert!(floor.normals.iter().all(|n| n[1] > 0.99), "{:?}", floor.normals);
        assert!(floor.uvs.contains(&[0.0, 1.0]), "V flipped to glTF's origin");
        let wall = &mesh.prims[1];
        assert_eq!(mesh.materials[wall.material].name, "default");
        assert_eq!(wall.positions.len(), 3);
        assert!(wall.normals.iter().all(|n| n[2] < -0.99 || n[2] > 0.99), "{:?}", wall.normals);
        assert!(crate::meshes::to_glb(&mesh).unwrap().starts_with(b"glTF"));
    }

    #[test]
    fn not_3ds_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("x.3ds");
        std::fs::write(&p, b"hello").unwrap();
        assert!(load(&p, dir.path()).is_err());
    }
}
