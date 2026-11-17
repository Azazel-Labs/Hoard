//! Writes a [`Model`] as binary glTF: one node per component instance carrying its transform, one mesh per
//! definition and inherited material, flat normals, and SketchUp's inches and Z-up turned into metres and Y-up.

use std::collections::{HashMap, HashSet};

use anyhow::{Result, bail, ensure};
use serde_json::{Value, json};

use crate::model::{Definition, Face, Id, Model};
use crate::triangulate::{cross, dot, newell, normalize, triangulate};

const INCH: f64 = 0.0254;
/// More nodes than any real model has; a file that nests big instance arrays inside each other could otherwise ask
/// for billions.
const MAX_NODES: usize = 1_000_000;
/// Total vertices across all meshes, which bounds the binary buffer to a few hundred megabytes.
const MAX_VERTICES: usize = 16_000_000;
/// Component nesting deeper than this is skipped rather than recursed into.
const MAX_NESTING: usize = 64;
/// SketchUp's default front-face colour, for faces nobody painted.
const DEFAULT_RGB: [u8; 3] = [255, 255, 255];

/// The image type of an embedded texture, from its leading bytes. Anything other than PNG or JPEG is dropped,
/// since glTF viewers only have to understand those two.
pub(crate) fn image_mime(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        Some("image/jpeg")
    } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("image/png")
    } else {
        None
    }
}

/// Whether a PNG has an alpha channel (greyscale-alpha or RGBA in its header), so cut-out textures like leaves
/// and fences keep their holes.
fn png_has_alpha(bytes: &[u8]) -> bool {
    bytes.starts_with(b"\x89PNG\r\n\x1a\n") && matches!(bytes.get(25), Some(4 | 6))
}

fn srgb_to_linear(c: u8) -> f64 {
    let c = f64::from(c) / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// SketchUp's inches, Z up, to glTF's metres, Y up. A rotation, so it keeps winding.
fn to_gltf(p: [f64; 3]) -> [f64; 3] {
    [p[0] * INCH, p[2] * INCH, -p[1] * INCH]
}

/// SketchUp's 13-value transform as a column-major glTF matrix in metres and Y up, or `None` when it holds
/// non-finite values. Index 12 is a uniform scale that multiplies the rest.
fn node_matrix(m: &[f64; 13]) -> Option<[f64; 16]> {
    let s = if m[12] == 0.0 { 1.0 } else { m[12] };
    let a = |r: usize, c: usize| m[r * 3 + c] * s;
    // swap axes on both sides: R * A * R^T with R = (x, y, z) -> (x, z, -y)
    let src = [0, 2, 1];
    let sign = [1.0, 1.0, -1.0];
    let mut out = [0.0; 16];
    for r in 0..3 {
        for c in 0..3 {
            out[c * 4 + r] = sign[r] * sign[c] * a(src[r], src[c]);
        }
    }
    out[12] = m[9] * s * INCH;
    out[13] = m[11] * s * INCH;
    out[14] = -m[10] * s * INCH;
    out[15] = 1.0;
    out.iter().all(|v| v.is_finite()).then_some(out)
}

const IDENTITY: [f64; 16] = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

/// Face-plane axes for SketchUp's default texture projection: U along Z × n, V along n × U, snapping near-flat
/// faces to world X and Y.
fn uv_basis(n: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    let (cx, cy) = (-n[1], n[0]);
    let len = cx.hypot(cy);
    if len < 1e-3 {
        return ([if n[2] >= 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0], [0.0, 1.0, 0.0]);
    }
    let xr = [cx / len, cy / len, 0.0];
    (xr, cross(n, xr))
}

#[expect(clippy::many_single_char_names, reason = "the textbook names for a 3x3 matrix's entries")]
fn invert_3x3(m: &[f64; 9]) -> [f64; 9] {
    let [a, b, c, d, e, f, g, h, i] = *m;
    let det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
    if det.abs() < 1e-12 || !det.is_finite() {
        return [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    }
    let k = 1.0 / det;
    [
        (e * i - f * h) * k,
        (c * h - b * i) * k,
        (b * f - c * e) * k,
        (f * g - d * i) * k,
        (a * i - c * g) * k,
        (c * d - a * f) * k,
        (d * h - e * g) * k,
        (b * g - a * h) * k,
        (a * e - b * d) * k,
    ]
}

/// A point's texture coordinate, with V flipped because glTF images start at the top and SketchUp's at the bottom.
fn face_uv(p: [f64; 3], basis: ([f64; 3], [f64; 3]), inv: Option<&[f64; 9]>, tile: [f64; 2]) -> [f32; 2] {
    let (px, py) = (dot(p, basis.0), dot(p, basis.1));
    let (u, v) = match inv {
        None => (px, py),
        Some(i) => {
            let q = px * i[2] + py * i[5] + i[8];
            let q = if q.abs() < 1e-12 { 1.0 } else { q };
            ((px * i[0] + py * i[3] + i[6]) / q, (px * i[1] + py * i[4] + i[7]) / q)
        }
    };
    #[expect(clippy::cast_possible_truncation, reason = "glTF texture coordinates are f32")]
    [(u / tile[0]) as f32, (1.0 - v / tile[1]) as f32]
}

#[derive(Default)]
struct Prim {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    indices: Vec<u32>,
}

struct Writer<'m> {
    model: &'m Model,
    hidden_layers: HashSet<Id>,
    bin: Vec<u8>,
    views: Vec<Value>,
    accessors: Vec<Value>,
    meshes: Vec<Value>,
    nodes: Vec<Value>,
    materials: Vec<Value>,
    material_index: HashMap<Option<Id>, usize>,
    images: Vec<Value>,
    image_index: HashMap<&'m [u8], usize>,
    mesh_cache: HashMap<(Option<Id>, Option<Id>), Option<usize>>,
    active: Vec<Id>,
    vertices: usize,
}

/// Converts the model to a GLB file.
pub(crate) fn write(model: &Model) -> Result<Vec<u8>> {
    let mut w = Writer {
        model,
        hidden_layers: model.hidden_layers.iter().copied().collect(),
        bin: Vec::new(),
        views: Vec::new(),
        accessors: Vec::new(),
        meshes: Vec::new(),
        nodes: Vec::new(),
        materials: Vec::new(),
        material_index: HashMap::new(),
        images: Vec::new(),
        image_index: HashMap::new(),
        mesh_cache: HashMap::new(),
        active: Vec::new(),
        vertices: 0,
    };
    let root = w.node(&model.root, None, None, "SketchUp model")?;
    ensure!(w.vertices > 0, "the SketchUp model has no visible faces");
    w.finish(root)
}

impl<'m> Writer<'m> {
    fn visible(&self, hidden: bool, layer: Option<Id>) -> bool {
        !hidden && layer.is_none_or(|l| !self.hidden_layers.contains(&l))
    }

    /// The node for one placement of `def`, with nodes for the instances inside it as its children.
    fn node(&mut self, def: &'m Definition, key: Option<Id>, inherited: Option<Id>, name: &str) -> Result<usize> {
        let mesh = self.mesh(def, key, inherited)?;
        let mut children = Vec::new();
        for inst in &def.instances {
            if !self.visible(inst.hidden, inst.layer) || self.active.len() >= MAX_NESTING {
                continue;
            }
            let Some(child) = self.model.definitions.get(&inst.definition) else { continue };
            let Some(matrix) = node_matrix(&inst.matrix) else { continue };
            // a definition that contains itself would recurse forever
            if self.active.contains(&inst.definition) {
                continue;
            }
            ensure!(self.nodes.len() < MAX_NODES, "the SketchUp model has too many component instances");
            self.active.push(inst.definition);
            let name = if inst.name.is_empty() { &child.name } else { &inst.name };
            let index = self.node(child, Some(inst.definition), inst.material.or(inherited), name);
            self.active.pop();
            let index = index?;
            if matrix != IDENTITY
                && let Some(node) = self.nodes.get_mut(index)
            {
                node["matrix"] = json!(matrix);
            }
            children.push(index);
        }
        let mut node = json!({ "name": name });
        if let Some(mesh) = mesh {
            node["mesh"] = json!(mesh);
        }
        if !children.is_empty() {
            node["children"] = json!(children);
        }
        self.nodes.push(node);
        Ok(self.nodes.len() - 1)
    }

    /// A definition's faces as a mesh, painting unpainted faces with the material the placement inherited. Built
    /// once per definition and inherited material, then shared by every node that needs it.
    fn mesh(&mut self, def: &'m Definition, key: Option<Id>, inherited: Option<Id>) -> Result<Option<usize>> {
        if let Some(&cached) = self.mesh_cache.get(&(key, inherited)) {
            return Ok(cached);
        }
        let mut prims: Vec<(Option<Id>, Prim)> = Vec::new();
        for face in &def.faces {
            if !self.visible(face.hidden, face.layer) {
                continue;
            }
            let mat = face.material.or(inherited).filter(|m| self.model.materials.contains_key(m));
            let slot = if let Some(i) = prims.iter().position(|p| p.0 == mat) {
                i
            } else {
                prims.push((mat, Prim::default()));
                prims.len() - 1
            };
            let texture = mat.and_then(|m| self.model.materials.get(&m)).and_then(|m| m.texture.as_ref());
            let tile = texture.map(|t| t.tile.map(|s| if s.abs() > 1e-9 && s.is_finite() { s } else { 1.0 }));
            let added = add_face(&mut prims[slot].1, face, tile);
            self.vertices += added;
            ensure!(self.vertices <= MAX_VERTICES, "the SketchUp model is too large to convert");
        }
        let mut primitives = Vec::new();
        for (mat, prim) in prims {
            if prim.indices.is_empty() {
                continue;
            }
            let material = self.material(mat);
            primitives.push(self.primitive(&prim, material));
        }
        let mesh = if primitives.is_empty() {
            None
        } else {
            let mut m = json!({ "primitives": primitives });
            if !def.name.is_empty() {
                m["name"] = json!(def.name);
            }
            self.meshes.push(m);
            Some(self.meshes.len() - 1)
        };
        self.mesh_cache.insert((key, inherited), mesh);
        Ok(mesh)
    }

    fn view(&mut self, bytes: &[u8], target: Option<u32>) -> usize {
        while !self.bin.len().is_multiple_of(4) {
            self.bin.push(0);
        }
        let mut v = json!({ "buffer": 0, "byteOffset": self.bin.len(), "byteLength": bytes.len() });
        if let Some(t) = target {
            v["target"] = json!(t);
        }
        self.bin.extend_from_slice(bytes);
        self.views.push(v);
        self.views.len() - 1
    }

    fn floats(&mut self, values: &[f32], kind: &str, bounds: bool) -> usize {
        let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
        let view = self.view(&bytes, Some(34962));
        let width = if kind == "VEC3" { 3 } else { 2 };
        let mut a = json!({
            "bufferView": view, "componentType": 5126, "count": values.len() / width, "type": kind,
        });
        if bounds {
            let mut min = vec![f32::INFINITY; width];
            let mut max = vec![f32::NEG_INFINITY; width];
            for chunk in values.chunks_exact(width) {
                for (k, &v) in chunk.iter().enumerate() {
                    min[k] = min[k].min(v);
                    max[k] = max[k].max(v);
                }
            }
            a["min"] = json!(min);
            a["max"] = json!(max);
        }
        self.accessors.push(a);
        self.accessors.len() - 1
    }

    fn primitive(&mut self, prim: &Prim, material: usize) -> Value {
        let position = self.floats(&prim.positions, "VEC3", true);
        let normal = self.floats(&prim.normals, "VEC3", false);
        let mut attributes = json!({ "POSITION": position, "NORMAL": normal });
        if !prim.uvs.is_empty() {
            attributes["TEXCOORD_0"] = json!(self.floats(&prim.uvs, "VEC2", false));
        }
        let bytes: Vec<u8> = prim.indices.iter().flat_map(|v| v.to_le_bytes()).collect();
        let view = self.view(&bytes, Some(34963));
        self.accessors.push(json!({
            "bufferView": view, "componentType": 5125, "count": prim.indices.len(), "type": "SCALAR",
        }));
        json!({ "attributes": attributes, "indices": self.accessors.len() - 1, "material": material })
    }

    /// The glTF material for a SketchUp material, or for unpainted faces when `None`.
    fn material(&mut self, id: Option<Id>) -> usize {
        if let Some(&i) = self.material_index.get(&id) {
            return i;
        }
        let model = self.model;
        let (name, rgb, alpha, texture) = match id.and_then(|m| model.materials.get(&m)) {
            Some(m) => (m.name.as_str(), m.rgb, m.alpha, m.texture.as_ref()),
            None => ("Default", DEFAULT_RGB, 1.0, None),
        };
        let alpha = if alpha.is_finite() { alpha.clamp(0.0, 1.0) } else { 1.0 };
        let mut pbr = json!({ "metallicFactor": 0.0, "roughnessFactor": 0.9 });
        let mut cutout = false;
        if let Some(tex) = texture {
            pbr["baseColorFactor"] = json!([1.0, 1.0, 1.0, alpha]);
            pbr["baseColorTexture"] = json!({ "index": self.texture(&tex.bytes) });
            cutout = png_has_alpha(&tex.bytes);
        } else {
            let [r, g, b] = rgb.map(srgb_to_linear);
            pbr["baseColorFactor"] = json!([r, g, b, alpha]);
        }
        let mut m = json!({ "name": name, "pbrMetallicRoughness": pbr, "doubleSided": true });
        if alpha < 0.999 {
            m["alphaMode"] = json!("BLEND");
        } else if cutout {
            m["alphaMode"] = json!("MASK");
            m["alphaCutoff"] = json!(0.5);
        }
        self.materials.push(m);
        let i = self.materials.len() - 1;
        self.material_index.insert(id, i);
        i
    }

    /// The texture for an embedded image, adding the image once however many materials share it.
    fn texture(&mut self, bytes: &'m [u8]) -> usize {
        if let Some(&i) = self.image_index.get(bytes) {
            return i;
        }
        let view = self.view(bytes, None);
        let mime = image_mime(bytes).unwrap_or("image/png");
        self.images.push(json!({ "bufferView": view, "mimeType": mime }));
        let i = self.images.len() - 1;
        self.image_index.insert(bytes, i);
        i
    }

    fn finish(self, root: usize) -> Result<Vec<u8>> {
        let mut doc = json!({
            "asset": { "version": "2.0", "generator": "hoard-skp" },
            "scene": 0,
            "scenes": [{ "nodes": [root] }],
            "nodes": self.nodes,
            "meshes": self.meshes,
            "materials": self.materials,
            "accessors": self.accessors,
            "bufferViews": self.views,
            "buffers": [{ "byteLength": self.bin.len() }],
        });
        if !self.images.is_empty() {
            let textures: Vec<Value> = (0..self.images.len()).map(|i| json!({ "source": i, "sampler": 0 })).collect();
            doc["images"] = json!(self.images);
            doc["textures"] = json!(textures);
            doc["samplers"] = json!([{ "wrapS": 10497, "wrapT": 10497 }]);
        }
        let mut text = serde_json::to_vec(&doc)?;
        while !text.len().is_multiple_of(4) {
            text.push(b' ');
        }
        let mut bin = self.bin;
        while !bin.len().is_multiple_of(4) {
            bin.push(0);
        }
        let total = 12 + 8 + text.len() + 8 + bin.len();
        let (Ok(total32), Ok(text32), Ok(bin32)) =
            (u32::try_from(total), u32::try_from(text.len()), u32::try_from(bin.len()))
        else {
            bail!("the converted model is larger than a GLB file can hold");
        };
        let mut out = Vec::with_capacity(total);
        out.extend_from_slice(b"glTF");
        out.extend_from_slice(&2u32.to_le_bytes());
        out.extend_from_slice(&total32.to_le_bytes());
        out.extend_from_slice(&text32.to_le_bytes());
        out.extend_from_slice(b"JSON");
        out.extend_from_slice(&text);
        out.extend_from_slice(&bin32.to_le_bytes());
        out.extend_from_slice(b"BIN\0");
        out.extend_from_slice(&bin);
        Ok(out)
    }
}

/// Triangulates one face into `prim`. Returns how many vertices it added; faces with non-finite points add none.
#[expect(clippy::cast_possible_truncation, reason = "glTF vertex data is f32")]
fn add_face(prim: &mut Prim, face: &Face, tile: Option<[f64; 2]>) -> usize {
    let points: Vec<[f64; 3]> = face.loops.iter().flatten().copied().collect();
    let out: Vec<[f32; 3]> = points.iter().map(|&p| to_gltf(p).map(|v| v as f32)).collect();
    if out.iter().flatten().any(|v| !v.is_finite()) {
        return 0;
    }
    let Some(n) = normalize(face.normal).or_else(|| face.loops.first().and_then(|l| newell(l))) else { return 0 };
    let tris = triangulate(&face.loops, n);
    let Ok(base) = u32::try_from(prim.positions.len() / 3) else { return 0 };
    if tris.is_empty() || u32::try_from(points.len()).ok().and_then(|c| base.checked_add(c)).is_none() {
        return 0;
    }
    let normal = to_gltf(n).map(|v| (v / INCH) as f32);
    let inv = face.uv_matrix.as_ref().map(invert_3x3);
    let basis = uv_basis(n);
    for (p, q) in points.iter().zip(&out) {
        prim.positions.extend_from_slice(q);
        prim.normals.extend_from_slice(&normal);
        if let Some(tile) = tile {
            prim.uvs.extend_from_slice(&face_uv(*p, basis, inv.as_ref(), tile));
        }
    }
    for t in tris {
        #[expect(clippy::cast_possible_truncation, reason = "the face's vertex count was checked to fit")]
        prim.indices.extend(t.map(|i| base + i as u32));
    }
    points.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Instance, Material};

    fn square_face(material: Option<Id>) -> Face {
        Face {
            loops: vec![vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 10.0, 0.0], [0.0, 10.0, 0.0]]],
            normal: [0.0, 0.0, 1.0],
            material,
            uv_matrix: None,
            hidden: false,
            layer: None,
        }
    }

    #[test]
    fn axis_swap_is_a_rotation() {
        assert_eq!(to_gltf([1.0, 0.0, 0.0]), [INCH, 0.0, 0.0]);
        // SketchUp's up becomes glTF's up, and SketchUp's north (+Y) points away from a glTF viewer (-Z)
        assert_eq!(to_gltf([0.0, 0.0, 1.0]), [0.0, INCH, 0.0]);
        assert_eq!(to_gltf([0.0, 1.0, 0.0]), [0.0, 0.0, -INCH]);
    }

    #[test]
    fn node_matrices_match_baked_points() {
        // rotate 90 degrees about Z, then move by (1, 2, 3) inches
        let m = [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 1.0];
        let g = node_matrix(&m).unwrap();
        let p = [5.0, 7.0, 11.0];
        let moved = [
            m[0] * p[0] + m[1] * p[1] + m[2] * p[2] + m[9],
            m[3] * p[0] + m[4] * p[1] + m[5] * p[2] + m[10],
            m[6] * p[0] + m[7] * p[1] + m[8] * p[2] + m[11],
        ];
        let want = to_gltf(moved);
        let q = to_gltf(p);
        for r in 0..3 {
            let got = g[r] * q[0] + g[4 + r] * q[1] + g[8 + r] * q[2] + g[12 + r];
            assert!((got - want[r]).abs() < 1e-12, "row {r}: {got} vs {}", want[r]);
        }
        assert!(node_matrix(&[f64::NAN; 13]).is_none());
    }

    #[test]
    fn writes_a_glb_with_instances_and_skips_hidden_ones() {
        let mut model = Model::default();
        model.materials.insert(7, Material { name: "Red".to_owned(), rgb: [255, 0, 0], alpha: 0.5, texture: None });
        model
            .definitions
            .insert(3, Definition { name: "Box".to_owned(), faces: vec![square_face(None)], instances: vec![] });
        let mut inst = Instance {
            definition: 3,
            name: String::new(),
            matrix: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            material: Some(7),
            hidden: false,
            layer: None,
        };
        model.root.instances.push(inst.clone());
        inst.hidden = true;
        model.root.instances.push(inst.clone());
        inst.hidden = false;
        inst.layer = Some(9);
        model.hidden_layers.push(9);
        model.root.instances.push(inst);
        model.root.faces.push(square_face(None));
        let glb = write(&model).unwrap();
        let (doc, buffers, _) = gltf::import_slice(&glb).unwrap();
        assert_eq!(buffers.len(), 1);
        let root = doc.default_scene().unwrap().nodes().next().unwrap();
        assert_eq!(root.children().count(), 1);
        let mats: Vec<_> = doc.materials().map(|m| m.name().unwrap_or_default().to_owned()).collect();
        assert!(mats.contains(&"Red".to_owned()) && mats.contains(&"Default".to_owned()));
    }

    #[test]
    fn a_model_with_nothing_visible_is_an_error() {
        let mut model = Model::default();
        let mut f = square_face(None);
        f.hidden = true;
        model.root.faces.push(f);
        assert!(write(&model).is_err());
    }

    #[test]
    fn recursive_definitions_terminate() {
        let mut model = Model::default();
        let inst = Instance {
            definition: 1,
            name: String::new(),
            matrix: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            material: None,
            hidden: false,
            layer: None,
        };
        model.definitions.insert(
            1,
            Definition { name: "Loop".to_owned(), faces: vec![square_face(None)], instances: vec![inst.clone()] },
        );
        model.root.instances.push(inst);
        assert!(write(&model).is_ok());
    }

    #[test]
    fn sniffs_images() {
        assert_eq!(image_mime(&[0xff, 0xd8, 0xff, 0xe0]), Some("image/jpeg"));
        assert_eq!(image_mime(b"\x89PNG\r\n\x1a\nrest"), Some("image/png"));
        assert_eq!(image_mime(b"BM"), None);
    }
}
