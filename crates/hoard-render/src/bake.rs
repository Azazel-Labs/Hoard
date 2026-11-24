//! A glTF document baked into draws: every mesh primitive of the default scene in world space, with its material.

use glam::{Mat3, Mat4, Vec3};

use super::{Alpha, Draw, Vertex};

pub(super) fn gltf_images(images: &[gltf::image::Data]) -> Vec<Option<image::RgbaImage>> {
    images.iter().map(|i| to_rgba8(i).and_then(|px| image::RgbaImage::from_raw(i.width, i.height, px))).collect()
}

fn to_rgba8(img: &gltf::image::Data) -> Option<Vec<u8>> {
    use gltf::image::Format;
    let px = (img.width * img.height) as usize;
    let p = &img.pixels;
    Some(match img.format {
        Format::R8G8B8A8 => p.clone(),
        Format::R8G8B8 => p.as_chunks::<3>().0.iter().flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        Format::R8G8 => p.as_chunks::<2>().0.iter().flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        Format::R8 => p.iter().flat_map(|&v| [v, v, v, 255]).collect(),
        Format::R16G16B16A16 => p
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| c[1])
            .collect::<Vec<_>>()
            .as_chunks::<4>()
            .0
            .iter()
            .flatten()
            .copied()
            .collect(),
        Format::R16G16B16 => p.as_chunks::<6>().0.iter().flat_map(|c| [c[1], c[3], c[5], 255]).collect(),
        _ => return None,
    })
    .filter(|v: &Vec<u8>| v.len() == px * 4)
}

/// Flatten the default scene into world-space triangle lists.
pub(super) fn bake(doc: &gltf::Document, buffers: &[gltf::buffer::Data]) -> Vec<Draw> {
    let mut draws = Vec::new();
    let scene = doc.default_scene().or_else(|| doc.scenes().next());
    let Some(scene) = scene else {
        // No scene: draw every mesh once, untransformed.
        for mesh in doc.meshes() {
            bake_mesh(&mesh, Mat4::IDENTITY, buffers, &mut draws);
        }
        return draws;
    };
    let mut stack: Vec<(gltf::Node<'_>, Mat4)> = scene.nodes().map(|n| (n, Mat4::IDENTITY)).collect();
    while let Some((node, parent)) = stack.pop() {
        let world = parent * Mat4::from_cols_array_2d(&node.transform().matrix());
        if let Some(mesh) = node.mesh() {
            bake_mesh(&mesh, world, buffers, &mut draws);
        }
        stack.extend(node.children().map(|c| (c, world)));
    }
    draws
}

fn bake_mesh(mesh: &gltf::Mesh<'_>, world: Mat4, buffers: &[gltf::buffer::Data], out: &mut Vec<Draw>) {
    let normal_m = Mat3::from_mat4(world).inverse().transpose();
    for prim in mesh.primitives() {
        if prim.mode() != gltf::mesh::Mode::Triangles {
            continue;
        }
        let reader = prim.reader(|b| buffers.get(b.index()).map(|d| &d.0[..]));
        let Some(positions) = reader.read_positions() else { continue };
        let positions: Vec<Vec3> = positions.map(|p| world.transform_point3(Vec3::from(p))).collect();
        let indices: Vec<u32> = match reader.read_indices() {
            Some(i) => i.into_u32().collect(),
            None => (0..positions.len() as u32).collect(),
        };
        if indices.iter().any(|&i| i as usize >= positions.len()) {
            continue;
        }
        let normals: Vec<Vec3> = match reader.read_normals() {
            Some(n) => n.map(|n| (normal_m * Vec3::from(n)).normalize_or_zero()).collect(),
            None => flat_normals(&positions, &indices),
        };
        let uvs: Vec<[f32; 2]> =
            reader.read_tex_coords(0).map_or_else(|| vec![[0.0, 0.0]; positions.len()], |t| t.into_f32().collect());
        let vertices = positions
            .iter()
            .enumerate()
            .map(|(i, p)| Vertex {
                pos: p.to_array(),
                normal: normals.get(i).copied().unwrap_or(Vec3::Y).to_array(),
                uv: uvs.get(i).copied().unwrap_or([0.0, 0.0]),
            })
            .collect();
        let material = prim.material();
        let pbr = material.pbr_metallic_roughness();
        out.push(Draw {
            vertices,
            indices,
            color: pbr.base_color_factor(),
            texture: pbr.base_color_texture().map(|t| t.texture().source().index()),
            alpha: match material.alpha_mode() {
                gltf::material::AlphaMode::Opaque => Alpha::Opaque,
                gltf::material::AlphaMode::Mask => Alpha::Mask(material.alpha_cutoff().unwrap_or(0.5)),
                gltf::material::AlphaMode::Blend => Alpha::Blend,
            },
            transmission: material.transmission().map_or(0.0, |t| t.transmission_factor().clamp(0.0, 1.0)),
            metallic: pbr.metallic_factor(),
            roughness: pbr.roughness_factor(),
            metallic_roughness: pbr.metallic_roughness_texture().map(|t| t.texture().source().index()),
        });
    }
}

fn flat_normals(positions: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for tri in indices.as_chunks::<3>().0 {
        let [a, b, c] = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
        let n = (positions[b] - positions[a]).cross(positions[c] - positions[a]);
        for i in [a, b, c] {
            normals[i] += n;
        }
    }
    normals.into_iter().map(Vec3::normalize_or_zero).collect()
}
