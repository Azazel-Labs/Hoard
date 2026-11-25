//! A model's far copy: the viewer's GLB with each triangle mesh reduced by farthing to a small share of its triangles
//! and its textures made smaller, for the mesh explorer to show detailed models lighter. Materials, nodes,
//! skins and clips stay as they are; only the geometry each primitive points at is replaced.

use std::time::Instant;

use anyhow::{Context, Result, ensure};
use farthing::Mesh;
use farthing::glam::{Vec2, Vec3, Vec4};
use farthing::reduce::{Quality, SimplifyOptions, StopWhen};
use rayon::prelude::*;
use serde_json::{Map, Value, json};

/// The triangles a far copy aims for in all, shared among its meshes by size.
const BUDGET: usize = 3_000;
/// Models with no more than this many triangles are drawn whole even far off.
pub(crate) const SMALL: usize = BUDGET * 2;
/// The farthest its surface may move, as a share of the model's size: about a pixel when it spans a hundred.
const MAX_ERROR: f32 = 0.01;
/// Meshes this small are kept as they are.
const MIN_TRIANGLES: usize = 32;
/// The longest side its textures keep: it's drawn a few hundred pixels across at most.
pub(crate) const TEXTURE_SIDE: u32 = 512;
/// Part of the cache key: raised when far copies are made differently, so the cached ones are made again.
pub(crate) const REV: u32 = 1;

/// Vertex attributes a far copy carries over. Others (a second set of joints, custom `_NAMED` data) keep a mesh
/// whole when they matter for how it moves, and are dropped otherwise.
const CARRIED: &[&str] = &["POSITION", "NORMAL", "TANGENT", "COLOR_0", "JOINTS_0", "WEIGHTS_0"];

/// What reducing a model did, for the log and the explorer.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) struct Stats {
    pub(crate) triangles_in: usize,
    pub(crate) triangles_out: usize,
    /// How long farthing took, in milliseconds.
    pub(crate) ms: u64,
}

/// The far copy of a plain GLB (Draco, meshopt and quantization decoded, as the viewer's is), not yet repacked: the
/// replaced geometry's old bytes are still in its binary chunk, for [`crate::webglb::repack_to`] to drop.
pub(crate) fn reduce(glb: &[u8]) -> Result<(Vec<u8>, Stats)> {
    let (mut json, mut bin) = crate::formats::read_glb(glb)?;
    let mut prims = Vec::new();
    for (m, mesh) in json.get("meshes").and_then(Value::as_array).into_iter().flatten().enumerate() {
        for (p, prim) in mesh.get("primitives").and_then(Value::as_array).into_iter().flatten().enumerate() {
            // A mesh that can't be read as triangles this way is drawn whole.
            if let Ok(Some(mesh)) = read_primitive(&json, &bin, prim) {
                prims.push((m, p, mesh));
            }
        }
    }
    let triangles_in: usize = prims.iter().map(|(_, _, m)| m.triangle_count()).sum();
    let mut stats = Stats { triangles_in, triangles_out: triangles_in, ms: 0 };
    if triangles_in <= SMALL {
        return Ok((glb.to_vec(), stats));
    }
    let bounds = prims.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), (_, _, m)| {
        let b = m.bounds();
        (lo.min(b.min), hi.max(b.max))
    });
    let max_error = (bounds.1 - bounds.0).length() * MAX_ERROR;
    let share = BUDGET as f32 / triangles_in as f32;

    let started = Instant::now();
    let reduced: Vec<Option<Mesh>> = prims
        .par_iter()
        .map(|(_, _, mesh)| {
            let tris = mesh.triangle_count();
            if tris <= MIN_TRIANGLES {
                return None;
            }
            let target = ((tris as f32 * share).ceil() as usize).max(MIN_TRIANGLES);
            simplify(mesh, target, max_error)
        })
        .collect();
    stats.ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

    let mut out = Writer { json: &mut json, bin: &mut bin };
    let mut replaced = Vec::new();
    for ((m, p, before), after) in prims.iter().zip(reduced) {
        let Some(after) = after else { continue };
        stats.triangles_out -= before.triangle_count() - after.triangle_count();
        replaced.extend(out.replace(*m, *p, &after)?);
    }
    drop_unused(&mut json, &replaced);
    let extras = json
        .as_object_mut()
        .context("glTF JSON is not an object")?
        .entry("asset")
        .or_insert_with(|| json!({ "version": "2.0" }))
        .as_object_mut()
        .context("glTF asset is not an object")?
        .entry("extras")
        .or_insert_with(|| Value::Object(Map::new()));
    if let Some(extras) = extras.as_object_mut() {
        extras.insert(
            "far".into(),
            json!({ "triangles_in": stats.triangles_in, "triangles_out": stats.triangles_out, "ms": stats.ms }),
        );
    }
    tracing::info!(
        triangles_in = stats.triangles_in,
        triangles_out = stats.triangles_out,
        meshes = prims.len(),
        ms = stats.ms,
        "far copy made"
    );
    Ok((crate::webglb::write_glb(&json, &bin)?, stats))
}

/// One primitive reduced toward `target` triangles, without moving its surface more than `max_error`; `None` when
/// that saves too little to be worth new geometry.
fn simplify(mesh: &Mesh, target: usize, max_error: f32) -> Option<Mesh> {
    let mut welded = mesh.clone();
    // Formats that store each triangle's corners apart give every corner its own vertex: shared ones are made one.
    farthing::reduce::repair::weld(&mut welded, 0.0);
    let opts = SimplifyOptions {
        target_triangles: Some(target),
        max_error: Some(max_error),
        stop: StopWhen::Any,
        // The lean reducer is 3-4 times faster on static meshes and moves their surface no more; only the full one
        // weighs skinning, keeping joints where they bend.
        quality: if welded.skin.is_some() { Quality::Best } else { Quality::Fast },
        ..SimplifyOptions::default()
    };
    let mut out = farthing::reduce::simplify(&welded, &opts).ok()?.mesh;
    if out.triangle_count() * 10 > mesh.triangle_count() * 9 || out.triangle_count() == 0 {
        return None;
    }
    farthing::reduce::optimize::optimize_mesh(&mut out);
    Some(out)
}

/// A triangle primitive as a farthing mesh; `None` for one drawn some other way, morphing, or carrying data a far
/// copy would lose.
fn read_primitive(json: &Value, bin: &[u8], prim: &Value) -> Result<Option<Mesh>> {
    let mode = prim.get("mode").and_then(Value::as_u64).unwrap_or(4);
    let Some(attrs) = prim.get("attributes").and_then(Value::as_object) else { return Ok(None) };
    if mode != 4
        || prim.get("targets").is_some()
        || prim.get("extensions").is_some_and(|e| e.get("KHR_draco_mesh_compression").is_some())
    {
        return Ok(None);
    }
    if attrs.keys().any(|k| k.starts_with("JOINTS_") && k != "JOINTS_0") {
        return Ok(None);
    }
    let attr = |name: &str| attrs.get(name).and_then(Value::as_u64).and_then(|i| usize::try_from(i).ok());
    let Some(at) = attr("POSITION") else { return Ok(None) };
    let positions: Vec<Vec3> = Elements::of(json, bin, at)?.floats::<3>()?.into_iter().map(Vec3::from_array).collect();
    let n = positions.len();
    let fits = |len: usize| -> Result<()> {
        ensure!(len == n, "attribute count doesn't match its positions");
        Ok(())
    };
    let indices = match prim.get("indices").and_then(Value::as_u64) {
        Some(i) => Elements::of(json, bin, usize::try_from(i)?)?.uints()?,
        None => (0..u32::try_from(n)?).collect(),
    };
    ensure!(indices.len() % 3 == 0 && indices.iter().all(|&i| (i as usize) < n), "indices out of range");
    let normals = match attr("NORMAL") {
        Some(i) => {
            Some(Elements::of(json, bin, i)?.floats::<3>()?.into_iter().map(Vec3::from_array).collect::<Vec<_>>())
        }
        None => None,
    };
    let tangents = match attr("TANGENT") {
        Some(i) => {
            Some(Elements::of(json, bin, i)?.floats::<4>()?.into_iter().map(Vec4::from_array).collect::<Vec<_>>())
        }
        None => None,
    };
    let mut uvs = Vec::new();
    while let Some(i) = attr(&format!("TEXCOORD_{}", uvs.len())) {
        uvs.push(Elements::of(json, bin, i)?.floats::<2>()?.into_iter().map(Vec2::from_array).collect::<Vec<_>>());
    }
    let colors = match attr("COLOR_0") {
        Some(i) => {
            let e = Elements::of(json, bin, i)?;
            Some(match e.width {
                3 => e.floats::<3>()?.into_iter().map(|c| Vec3::from_array(c).extend(1.0)).collect::<Vec<_>>(),
                _ => e.floats::<4>()?.into_iter().map(Vec4::from_array).collect(),
            })
        }
        None => None,
    };
    let skin = match (attr("JOINTS_0"), attr("WEIGHTS_0")) {
        (Some(j), Some(w)) => {
            let joints = Elements::of(json, bin, j)?.uints()?;
            let weights = Elements::of(json, bin, w)?.floats::<4>()?;
            ensure!(joints.len() == n * 4, "joints don't match their positions");
            let joints = joints
                .as_chunks::<4>()
                .0
                .iter()
                .map(|c| Ok([u16::try_from(c[0])?, u16::try_from(c[1])?, u16::try_from(c[2])?, u16::try_from(c[3])?]))
                .collect::<Result<Vec<_>>>()?;
            Some(farthing::Skin { joints, weights })
        }
        _ => None,
    };
    for len in normals.iter().map(Vec::len).chain(tangents.iter().map(Vec::len)).chain(colors.iter().map(Vec::len)) {
        fits(len)?;
    }
    for set in &uvs {
        fits(set.len())?;
    }
    if let Some(s) = &skin {
        fits(s.weights.len())?;
    }
    let tris = indices.len() / 3;
    Ok(Some(Mesh {
        positions,
        normals,
        tangents,
        uvs,
        colors,
        skin,
        morph_targets: Vec::new(),
        indices,
        material_ids: vec![0; tris],
    }))
}

/// Of the accessors replaced geometry used, those nothing points at any more become empty stubs (indices elsewhere
/// stay as they are), so repacking drops their bytes.
fn drop_unused(json: &mut Value, replaced: &[u64]) {
    fn used(key: &str, v: &Value, holds: bool, found: &mut std::collections::HashSet<u64>) {
        match v {
            // Every number an accessor could be: an attribute or morph target's, indices, a skin's matrices, a
            // clip's keys. Counting more than that only keeps bytes.
            Value::Number(n) if holds || matches!(key, "indices" | "inverseBindMatrices" | "input" | "output") => {
                found.extend(n.as_u64());
            }
            Value::Array(a) => a.iter().for_each(|x| used(key, x, holds || key == "targets", found)),
            Value::Object(o) => {
                o.iter().for_each(|(k, x)| used(k, x, key == "attributes" || (holds && key == "targets"), found));
            }
            _ => {}
        }
    }
    let mut found = std::collections::HashSet::new();
    if let Some(doc) = json.as_object() {
        for (k, v) in doc.iter().filter(|(k, _)| k.as_str() != "accessors") {
            used(k, v, false, &mut found);
        }
    }
    let Some(accessors) = json.get_mut("accessors").and_then(Value::as_array_mut) else { return };
    for i in replaced.iter().filter(|i| !found.contains(i)) {
        if let Some(a) = usize::try_from(*i).ok().and_then(|i| accessors.get_mut(i)) {
            *a = json!({ "componentType": 5126, "count": 1, "type": "SCALAR" });
        }
    }
}

/// One accessor's bytes, laid out as its buffer view says.
struct Elements<'a> {
    bytes: &'a [u8],
    count: usize,
    width: usize,
    component: u64,
    size: usize,
    stride: usize,
    normalized: bool,
}

impl<'a> Elements<'a> {
    fn of(json: &Value, bin: &'a [u8], i: usize) -> Result<Self> {
        let a = json.get("accessors").and_then(|a| a.get(i)).with_context(|| format!("no accessor {i}"))?;
        ensure!(a.get("sparse").is_none(), "sparse accessors aren't read");
        let count = usize::try_from(a.get("count").and_then(Value::as_u64).context("accessor has no count")?)?;
        let width = match a.get("type").and_then(Value::as_str) {
            Some("SCALAR") => 1,
            Some("VEC2") => 2,
            Some("VEC3") => 3,
            Some("VEC4") => 4,
            other => anyhow::bail!("accessor type {other:?} isn't a vector"),
        };
        let component = a.get("componentType").and_then(Value::as_u64).unwrap_or(0);
        let size = match component {
            5120 | 5121 => 1,
            5122 | 5123 => 2,
            5125 | 5126 => 4,
            c => anyhow::bail!("unknown component type {c}"),
        };
        let normalized = a.get("normalized").and_then(Value::as_bool).unwrap_or(false);
        let Some(view) = a.get("bufferView").and_then(Value::as_u64) else {
            // No view: all zeros, which a mesh can't be drawn from.
            anyhow::bail!("accessor without a buffer view");
        };
        let view =
            json.get("bufferViews").and_then(|v| v.get(usize::try_from(view).ok()?)).context("no buffer view")?;
        ensure!(view.get("buffer").and_then(Value::as_u64).unwrap_or(0) == 0, "buffer view is outside the GLB");
        let offset = |v: &Value| usize::try_from(v.get("byteOffset").and_then(Value::as_u64).unwrap_or(0));
        let start = offset(view)? + offset(a)?;
        let stride = match view.get("byteStride").and_then(Value::as_u64) {
            Some(s) => usize::try_from(s)?,
            None => width * size,
        };
        ensure!(stride >= width * size, "buffer view stride is shorter than an element");
        let len = if count == 0 { 0 } else { (count - 1) * stride + width * size };
        let bytes = bin.get(start..start + len).context("accessor runs past the binary chunk")?;
        Ok(Self { bytes, count, width, component, size, stride, normalized })
    }

    fn component(&self, at: usize) -> f64 {
        let b = &self.bytes[at..at + self.size];
        match self.component {
            5120 => f64::from(b[0].cast_signed()),
            5121 => f64::from(b[0]),
            5122 => f64::from(i16::from_le_bytes([b[0], b[1]])),
            5123 => f64::from(u16::from_le_bytes([b[0], b[1]])),
            5125 => f64::from(u32::from_le_bytes([b[0], b[1], b[2], b[3]])),
            _ => f64::from(f32::from_le_bytes([b[0], b[1], b[2], b[3]])),
        }
    }

    /// Each element as `N` floats, normalized integers scaled as glTF says; missing components are 0 (1 for a
    /// fourth).
    fn floats<const N: usize>(&self) -> Result<Vec<[f32; N]>> {
        ensure!(self.width <= N, "accessor is wider than expected");
        let scale = match (self.normalized, self.component) {
            (true, 5120) => 127.0,
            (true, 5121) => 255.0,
            (true, 5122) => 32767.0,
            (true, 5123) => 65535.0,
            (true, 5125) => f64::from(u32::MAX),
            _ => 1.0,
        };
        Ok((0..self.count)
            .map(|e| {
                let mut v = [0.0; N];
                if N == 4 {
                    v[3] = 1.0;
                }
                for (c, slot) in v.iter_mut().enumerate().take(self.width) {
                    let x = self.component(e * self.stride + c * self.size) / scale;
                    *slot = if self.normalized { x.max(-1.0) } else { x } as f32;
                }
                v
            })
            .collect())
    }

    /// Every component as an unsigned integer, element after element.
    fn uints(&self) -> Result<Vec<u32>> {
        ensure!(matches!(self.component, 5121 | 5123 | 5125), "not unsigned integers");
        Ok((0..self.count)
            .flat_map(|e| (0..self.width).map(move |c| (e, c)))
            .map(|(e, c)| self.component(e * self.stride + c * self.size) as u32)
            .collect())
    }
}

/// New geometry appended to a GLB.
struct Writer<'a> {
    json: &'a mut Value,
    bin: &'a mut Vec<u8>,
}

impl Writer<'_> {
    /// Point primitive `p` of mesh `m` at `mesh`'s geometry, and say which accessors it used before.
    fn replace(&mut self, m: usize, p: usize, mesh: &Mesh) -> Result<Vec<u64>> {
        let old = self.json["meshes"][m]["primitives"][p]["attributes"].as_object().cloned().unwrap_or_default();
        let mut before: Vec<u64> = old.values().filter_map(Value::as_u64).collect();
        before.extend(self.json["meshes"][m]["primitives"][p]["indices"].as_u64());
        let mut attrs = Map::new();
        let flat3 = |v: &[Vec3]| v.iter().flat_map(Vec3::to_array).collect::<Vec<f32>>();
        let (lo, hi) =
            mesh.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
        let pos =
            self.floats(&flat3(&mesh.positions), "VEC3", Some((lo.to_array().to_vec(), hi.to_array().to_vec())))?;
        attrs.insert("POSITION".into(), json!(pos));
        if let Some(n) = &mesh.normals {
            attrs.insert("NORMAL".into(), json!(self.floats(&flat3(n), "VEC3", None)?));
        }
        if let Some(t) = &mesh.tangents {
            let flat: Vec<f32> = t.iter().flat_map(Vec4::to_array).collect();
            attrs.insert("TANGENT".into(), json!(self.floats(&flat, "VEC4", None)?));
        }
        for (i, set) in mesh.uvs.iter().enumerate() {
            let flat: Vec<f32> = set.iter().flat_map(Vec2::to_array).collect();
            attrs.insert(format!("TEXCOORD_{i}"), json!(self.floats(&flat, "VEC2", None)?));
        }
        if let Some(c) = &mesh.colors {
            // As wide as it was: a fourth component turns on vertex alpha in some viewers.
            let was3 = old
                .get("COLOR_0")
                .and_then(Value::as_u64)
                .is_some_and(|i| self.json["accessors"][i as usize]["type"] == "VEC3");
            let (flat, ty): (Vec<f32>, _) = if was3 {
                (c.iter().flat_map(|x| x.truncate().to_array()).collect(), "VEC3")
            } else {
                (c.iter().flat_map(Vec4::to_array).collect(), "VEC4")
            };
            attrs.insert("COLOR_0".into(), json!(self.floats(&flat, ty, None)?));
        }
        if let Some(s) = &mesh.skin {
            let joints: Vec<u8> = s.joints.iter().flatten().flat_map(|j| j.to_le_bytes()).collect();
            let view = self.view(&joints)?;
            attrs.insert("JOINTS_0".into(), json!(self.accessor(view, 5123, s.joints.len(), "VEC4", None)?));
            let flat: Vec<f32> = s.weights.iter().flatten().copied().collect();
            attrs.insert("WEIGHTS_0".into(), json!(self.floats(&flat, "VEC4", None)?));
        }
        debug_assert!(attrs.keys().all(|k| CARRIED.contains(&k.as_str()) || k.starts_with("TEXCOORD_")));
        let (bytes, component): (Vec<u8>, u64) = if u16::try_from(mesh.positions.len()).is_ok() {
            (mesh.indices.iter().flat_map(|&i| (i as u16).to_le_bytes()).collect(), 5123)
        } else {
            (mesh.indices.iter().flat_map(|i| i.to_le_bytes()).collect(), 5125)
        };
        let view = self.view(&bytes)?;
        let indices = self.accessor(view, component, mesh.indices.len(), "SCALAR", None)?;
        let prim = &mut self.json["meshes"][m]["primitives"][p];
        prim["attributes"] = Value::Object(attrs);
        prim["indices"] = json!(indices);
        Ok(before)
    }

    fn floats(&mut self, v: &[f32], ty: &str, bounds: Option<(Vec<f32>, Vec<f32>)>) -> Result<usize> {
        let width = match ty {
            "VEC2" => 2,
            "VEC3" => 3,
            _ => 4,
        };
        let bytes: Vec<u8> = v.iter().flat_map(|f| f.to_le_bytes()).collect();
        let view = self.view(&bytes)?;
        self.accessor(view, 5126, v.len() / width, ty, bounds)
    }

    /// Append `item` to the document's list `key`, and say where it went.
    fn push(&mut self, key: &str, item: Value) -> Result<usize> {
        let list =
            self.json.as_object_mut().context("glTF JSON is not an object")?.entry(key).or_insert_with(|| json!([]));
        let list = list.as_array_mut().with_context(|| format!("glTF {key} is not a list"))?;
        list.push(item);
        Ok(list.len() - 1)
    }

    fn view(&mut self, bytes: &[u8]) -> Result<usize> {
        self.bin.resize(self.bin.len().next_multiple_of(4), 0);
        let n =
            self.push("bufferViews", json!({ "buffer": 0, "byteOffset": self.bin.len(), "byteLength": bytes.len() }))?;
        self.bin.extend_from_slice(bytes);
        self.json["buffers"] = json!([{ "byteLength": self.bin.len() }]);
        Ok(n)
    }

    fn accessor(
        &mut self,
        view: usize,
        component: u64,
        count: usize,
        ty: &str,
        bounds: Option<(Vec<f32>, Vec<f32>)>,
    ) -> Result<usize> {
        let mut a = json!({ "bufferView": view, "componentType": component, "count": count, "type": ty });
        if let Some((lo, hi)) = bounds {
            a["min"] = json!(lo);
            a["max"] = json!(hi);
        }
        self.push("accessors", a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A GLB of one UV sphere with normals and texture coordinates, `seg` × `rings` quads, on two nodes.
    fn sphere_glb(seg: u32, rings: u32) -> Vec<u8> {
        let (mut pos, mut nrm, mut uv, mut idx) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for r in 0..=rings {
            for s in 0..=seg {
                let (u, v) = (s as f32 / seg as f32, r as f32 / rings as f32);
                let (th, ph) = (u * std::f32::consts::TAU, v * std::f32::consts::PI);
                let p = Vec3::new(ph.sin() * th.cos(), ph.cos(), ph.sin() * th.sin());
                pos.extend(p.to_array());
                nrm.extend(p.to_array());
                uv.extend([u, v]);
            }
        }
        for r in 0..rings {
            for s in 0..seg {
                let a = r * (seg + 1) + s;
                let b = a + seg + 1;
                idx.extend([a, b, a + 1, a + 1, b, b + 1]);
            }
        }
        let mut bin = Vec::new();
        let mut views = Vec::new();
        for chunk in [
            pos.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>(),
            nrm.iter().flat_map(|f| f.to_le_bytes()).collect(),
            uv.iter().flat_map(|f| f.to_le_bytes()).collect(),
            idx.iter().flat_map(|i| i.to_le_bytes()).collect(),
        ] {
            views.push(json!({ "buffer": 0, "byteOffset": bin.len(), "byteLength": chunk.len() }));
            bin.extend(chunk);
        }
        let n = pos.len() / 3;
        let doc = json!({
            "asset": { "version": "2.0" },
            "scenes": [{ "nodes": [0, 1] }],
            "nodes": [{ "mesh": 0 }, { "mesh": 0, "translation": [3, 0, 0] }],
            "materials": [{ "name": "Skin" }],
            "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 }, "indices": 3, "material": 0 }] }],
            "accessors": [
                { "bufferView": 0, "componentType": 5126, "count": n, "type": "VEC3", "min": [-1, -1, -1], "max": [1, 1, 1] },
                { "bufferView": 1, "componentType": 5126, "count": n, "type": "VEC3" },
                { "bufferView": 2, "componentType": 5126, "count": n, "type": "VEC2" },
                { "bufferView": 3, "componentType": 5125, "count": idx.len(), "type": "SCALAR" },
            ],
            "bufferViews": views,
            "buffers": [{ "byteLength": bin.len() }],
        });
        crate::webglb::write_glb(&doc, &bin).unwrap()
    }

    #[test]
    fn a_dense_mesh_comes_down_to_the_budget_and_keeps_its_material() {
        let glb = sphere_glb(160, 80);
        let (far, stats) = reduce(&glb).unwrap();
        assert_eq!(stats.triangles_in, 160 * 80 * 2);
        assert!(stats.triangles_out < BUDGET * 2, "{stats:?}");
        let far = crate::webglb::repack(&far).unwrap();
        assert!(far.len() < glb.len() / 4, "{} of {}", far.len(), glb.len());
        let (json, bin) = crate::formats::read_glb(&far).unwrap();
        let prim = &json["meshes"][0]["primitives"][0];
        assert_eq!(prim["material"], 0);
        assert_eq!(json["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(json["asset"]["extras"]["far"]["triangles_out"], stats.triangles_out);
        let mesh = read_primitive(&json, &bin, prim).unwrap().unwrap();
        assert_eq!(mesh.triangle_count(), stats.triangles_out);
        assert!(mesh.normals.is_some() && mesh.uvs.len() == 1);
        // Still a unit sphere: nothing moved more than the error allows.
        assert!(mesh.positions.iter().all(|p| (p.length() - 1.0).abs() < 0.05));
    }

    #[test]
    fn small_models_stay_whole() {
        let glb = sphere_glb(24, 12);
        let (far, stats) = reduce(&glb).unwrap();
        assert_eq!(far, glb);
        assert_eq!(stats.triangles_out, stats.triangles_in);
    }

    /// Times a real model through the viewer's pipeline and farthing:
    /// `HOARD_FAR_SAMPLE=path/to.glb cargo test -p hoardd --release far_sample -- --ignored --nocapture`. With
    /// `HOARD_FAR_OUT` naming a folder, its view and far copies are written there to compare.
    #[test]
    #[ignore = "needs a model named by HOARD_FAR_SAMPLE"]
    #[expect(clippy::print_stdout, reason = "its report is what it's run for")]
    fn far_sample() {
        let Ok(path) = std::env::var("HOARD_FAR_SAMPLE") else { return };
        let path = std::path::PathBuf::from(path);
        let ext = path.extension().unwrap().to_string_lossy().to_lowercase();
        let root = path.parent().unwrap();
        let view = crate::meshes::view_glb(&path, &ext, root).unwrap();
        let started = Instant::now();
        let (far, stats) = reduce(&crate::meshes::viewable(&path, &ext, root).unwrap()).unwrap();
        let far = crate::webglb::repack_to(&far, TEXTURE_SIDE).unwrap();
        let all = started.elapsed().as_millis();
        println!("{}: {stats:?}, view {} → far {} bytes, {all} ms in all", path.display(), view.len(), far.len());
        if let Ok(out) = std::env::var("HOARD_FAR_OUT") {
            let stem = path.file_stem().unwrap().to_string_lossy();
            let out = std::path::Path::new(&out);
            std::fs::write(out.join(format!("{stem}.view.glb")), &view).unwrap();
            std::fs::write(out.join(format!("{stem}.far.glb")), &far).unwrap();
        }
    }
}
