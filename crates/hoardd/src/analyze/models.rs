//! Models: their meshes, materials and animations counted, and drawn for a thumbnail.

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;

use anyhow::Context as _;
use serde_json::{Map, Value, json};

use super::{Analysis, VIEWS_ENABLED, renderer, thumb_bytes, thumb_size};

/// A glTF/GLB the renderer can read: one using meshopt, Draco or quantization (which the renderer's glTF reader
/// rejects) is rewritten by polysmelt as a plain GLB (its images taken in) in a temporary folder, anything else is read
/// where it is.
struct Drawable {
    path: PathBuf,
    _dir: Option<tempfile::TempDir>,
}

/// The extensions plain glTF readers lack, which `polysmelt` decodes away.
const ENCODINGS: [&str; 4] =
    ["EXT_meshopt_compression", "KHR_meshopt_compression", "KHR_draco_mesh_compression", "KHR_mesh_quantization"];

fn drawable(path: &Path) -> anyhow::Result<Drawable> {
    let doc = glb_json(path).or_else(|_| anyhow::Ok(serde_json::from_slice::<Value>(&std::fs::read(path)?)?))?;
    let encoded = doc
        .get("extensionsUsed")
        .and_then(Value::as_array)
        .is_some_and(|e| e.iter().filter_map(Value::as_str).any(|e| ENCODINGS.contains(&e)));
    // A pack whose models are turned has them drawn turned (see `meshes::facing`).
    let facing = crate::meshes::current_facing();
    if !encoded && facing.is_upright() {
        return Ok(Drawable { path: path.to_owned(), _dir: None });
    }
    let glb = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("glb"));
    let ext = if glb { "glb" } else { "gltf" };
    let glb = crate::meshes::plain_glb(path, ext, facing).context("rewriting a glTF to draw")?;
    let dir = tempfile::tempdir()?;
    let plain = dir.path().join("model.glb");
    std::fs::write(&plain, glb)?;
    Ok(Drawable { path: plain, _dir: Some(dir) })
}

/// Thumbnail, plus the multi-view sheet when enabled. A model that crashes the renderer gets a `thumb_error` in
/// its facts, which Settings → Issues lists.
pub(super) fn render_model_thumb(path: &Path, a: &mut Analysis) {
    if !super::can_render() {
        return;
    }
    let model = match drawable(path) {
        Ok(model) => model,
        Err(e) => {
            tracing::debug!(path = %path.display(), error = %format!("{e:#}"), "no model thumbnail");
            return;
        }
    };
    let file = model.path.as_path();
    let Some(guard) = renderer() else { return };
    // wgpu panics on what it finds invalid; that costs this model its thumbnail, not the whole pack its ingest.
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let img = guard.render_file(file, thumb_size())?;
        let views = VIEWS_ENABLED
            .load(Ordering::Relaxed)
            .then(|| guard.render_views_file(file, thumb_size()).ok().and_then(|v| thumb_bytes(&v, false).ok()))
            .flatten();
        anyhow::Ok((thumb_bytes(&img, false).ok(), views))
    }));
    match drawn {
        Ok(Ok(out)) => (a.thumb, a.views) = out,
        Ok(Err(e)) => {
            tracing::debug!(path = %path.display(), error = %format!("{e:#}"), "no model thumbnail");
        }
        Err(_) => {
            tracing::warn!(path = %path.display(), "the renderer failed on this model; it has no thumbnail");
            a.meta.insert("thumb_error".into(), json!("The thumbnail renderer crashed on this model."));
        }
    }
}

/// A thumbnail of a glTF/GLB model as it is; `None` when no renderer is running.
pub(crate) fn gltf_thumb(path: &Path) -> anyhow::Result<Option<Vec<u8>>> {
    if !super::can_render() {
        return Ok(None);
    }
    let model = drawable(path)?;
    let path = model.path.as_path();
    let Some(guard) = renderer() else { return Ok(None) };
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| guard.render_file(path, thumb_size())))
        .map_err(|_| anyhow::anyhow!("the renderer failed on this model"))??;
    thumb_bytes(&drawn, false).map(Some)
}

/// A thumbnail of a glTF/GLB model wearing `color` (a texture's thumbnail) as its color map, or its own maps when
/// it has some; `None` when no renderer is running.
pub(crate) fn gltf_thumb_dressed(path: &Path, color: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
    if !super::can_render() {
        return Ok(None);
    }
    let color = image::load_from_memory(color)?.to_rgba8();
    let model = drawable(path)?;
    let path = model.path.as_path();
    let Some(guard) = renderer() else { return Ok(None) };
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        match guard.render_file_dressed(path, thumb_size(), color)? {
            Some(img) => Ok(img),
            None => guard.render_file(path, thumb_size()),
        }
    }))
    .map_err(|_| anyhow::anyhow!("the renderer failed on this model"))??;
    thumb_bytes(&drawn, false).map(Some)
}

/// A model polysmelt reads other than glTF, OBJ and FBX (STL, Collada, VRM, BVH, splats…): its facts, skeleton and
/// clips, and a thumbnail rendered from a GLB made on the spot when it draws anything.
pub(super) fn analyze_mesh(path: &Path, root: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let model = crate::meshes::load_rigged(path, ext, root)?;
    a.meta.extend(model.facts);
    a.rig = model.rig;
    let Some(mesh) = model.mesh else { return Ok(()) };
    render_mesh(&mesh, path, ext, a)
}

/// Thumbnail for a model Hoard converts itself (OBJ, FBX), keeping the facts the stats pass found.
pub(super) fn mesh_thumb(path: &Path, root: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let model = crate::meshes::load_rigged(path, ext, root)?;
    a.rig = model.rig;
    match model.mesh {
        Some(m) => render_mesh(&m, path, ext, a),
        None => Ok(()),
    }
}

/// A thumbnail of a mesh put together elsewhere, made like any model's; `path` names a scratch file beside which
/// the GLB is written for a moment.
pub(crate) fn mesh_thumb_bytes(mesh: &crate::meshes::Mesh, path: &Path) -> anyhow::Result<Option<Vec<u8>>> {
    let mut a = Analysis::default();
    render_mesh(mesh, path, "mesh", &mut a)?;
    Ok(a.thumb)
}

fn render_mesh(mesh: &crate::meshes::Mesh, path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let glb = crate::meshes::to_glb(mesh)?;
    let tmp = path.with_extension(format!("{ext}.hoard-preview.glb"));
    std::fs::write(&tmp, glb)?;
    render_model_thumb(&tmp, a);
    std::fs::remove_file(&tmp).ok();
    Ok(())
}

pub(super) fn analyze_model(
    path: &Path,
    root: &Path,
    ext: &str,
    meta: &mut Map<String, Value>,
) -> anyhow::Result<Option<crate::animations::Rig>> {
    let mut rig = None;
    match ext {
        "gltf" | "glb" => {
            let json = if ext == "glb" {
                glb_json(path)?
            } else {
                serde_json::from_reader(BufReader::new(File::open(path)?))?
            };
            gltf_stats(&json, meta);
            rig = crate::animations::rig(&json);
            // The triangles the scene draws (strips and fans, meshes placed more than once, points and lines
            // apart), when polysmelt can read it.
            match crate::meshes::facts(path, ext, root) {
                Ok(facts) => {
                    meta.insert("triangles".into(), json!(0));
                    meta.extend(facts);
                }
                Err(e) => {
                    tracing::debug!(path = %path.display(), error = %format!("{e:#}"), "glTF counted from its JSON");
                }
            }
        }
        "obj" => obj_stats(path, meta)?,
        "fbx" => fbx_stats(path, meta)?,
        _ => {}
    }
    Ok(rig)
}

fn glb_json(path: &Path) -> anyhow::Result<Value> {
    let mut f = File::open(path)?;
    let mut header = [0u8; 20];
    f.read_exact(&mut header)?;
    anyhow::ensure!(&header[0..4] == b"glTF", "not a GLB file");
    let chunk_len = u32::from_le_bytes([header[12], header[13], header[14], header[15]]) as usize;
    anyhow::ensure!(&header[16..20] == b"JSON", "first GLB chunk is not JSON");
    anyhow::ensure!(chunk_len < 128 << 20, "GLB JSON chunk is unreasonably large");
    let mut buf = vec![0u8; chunk_len];
    f.read_exact(&mut buf)?;
    Ok(serde_json::from_slice(&buf)?)
}

fn gltf_stats(doc: &Value, meta: &mut Map<String, Value>) {
    let count = |k: &str| doc.get(k).and_then(Value::as_array).map_or(0, Vec::len);
    for k in ["meshes", "materials", "animations", "skins", "nodes", "textures"] {
        meta.insert(k.into(), json!(count(k)));
    }
    let accessors = doc.get("accessors").and_then(Value::as_array);
    let acc_count = |idx: &Value| -> u64 {
        idx.as_u64().and_then(|i| accessors?.get(i as usize)?.get("count")?.as_u64()).unwrap_or(0)
    };
    let (mut verts, mut tris) = (0u64, 0u64);
    for mesh in doc.get("meshes").and_then(Value::as_array).into_iter().flatten() {
        for prim in mesh.get("primitives").and_then(Value::as_array).into_iter().flatten() {
            let v = prim.get("attributes").and_then(|a| a.get("POSITION")).map_or(0, acc_count);
            verts += v;
            tris += prim.get("indices").map_or(v, acc_count) / 3;
        }
    }
    meta.insert("vertices".into(), json!(verts));
    meta.insert("triangles".into(), json!(tris));
    if let Some(generator) = doc.get("asset").and_then(|a| a.get("generator")).and_then(Value::as_str) {
        meta.insert("generator".into(), json!(generator));
    }
}

fn obj_stats(path: &Path, meta: &mut Map<String, Value>) -> anyhow::Result<()> {
    let (mut verts, mut tris, mut objects) = (0u64, 0u64, 0u64);
    let mut materials = std::collections::BTreeSet::new();
    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => verts += 1,
            Some("f") => tris += (parts.count() as u64).saturating_sub(2),
            Some("o" | "g") => objects += 1,
            Some("usemtl") => {
                if let Some(m) = parts.next() {
                    materials.insert(m.to_owned());
                }
            }
            _ => {}
        }
    }
    meta.insert("vertices".into(), json!(verts));
    meta.insert("triangles".into(), json!(tris));
    meta.insert("objects".into(), json!(objects));
    meta.insert("materials".into(), json!(materials.len()));
    Ok(())
}

/// FBX is proprietary, but node names are plain strings in the file, so a byte scan finds
/// skinning (`Deformer`) and animation (`AnimationCurve`) without a full parser.
fn fbx_stats(path: &Path, meta: &mut Map<String, Value>) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    File::open(path)?.take(96 << 20).read_to_end(&mut buf)?;
    if buf.starts_with(b"Kaydara FBX Binary") && buf.len() > 27 {
        let v = u32::from_le_bytes([buf[23], buf[24], buf[25], buf[26]]);
        meta.insert("fbx_version".into(), json!(v));
    } else if buf.starts_with(b"; FBX") {
        meta.insert("fbx_ascii".into(), json!(true));
    }
    let contains = |needle: &[u8]| buf.windows(needle.len()).any(|w| w == needle);
    meta.insert("has_deformers".into(), json!(contains(b"Deformer")));
    meta.insert("has_animation".into(), json!(contains(b"AnimationCurve")));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gltf_triangles_are_the_ones_drawn() {
        let dir = tempfile::tempdir().unwrap();
        // Four corners of a quad as one triangle strip: two triangles, though four vertices over three is one.
        let bin: Vec<u8> = [[0f32, 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]]
            .iter()
            .flatten()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        std::fs::write(dir.path().join("quad.bin"), &bin).unwrap();
        let doc = json!({
            "asset": {"version": "2.0"},
            "buffers": [{"uri": "quad.bin", "byteLength": bin.len()}],
            "bufferViews": [{"buffer": 0, "byteLength": bin.len()}],
            "accessors": [{"bufferView": 0, "componentType": 5126, "count": 4, "type": "VEC3",
                           "min": [0, 0, 0], "max": [1, 1, 0]}],
            "meshes": [{"primitives": [{"attributes": {"POSITION": 0}, "mode": 5}]}],
            // The quad placed twice.
            "nodes": [{"mesh": 0}, {"mesh": 0, "translation": [2, 0, 0]}],
            "scenes": [{"nodes": [0, 1]}],
            "scene": 0,
        });
        let path = dir.path().join("quad.gltf");
        std::fs::write(&path, serde_json::to_vec(&doc).unwrap()).unwrap();
        let mut meta = Map::new();
        analyze_model(&path, dir.path(), "gltf", &mut meta).unwrap();
        assert_eq!(meta["triangles"], json!(4));
        assert_eq!(meta["vertices"], json!(4));
        assert_eq!(meta["meshes"], json!(1));

        // What polysmelt can't read (its buffer is missing) is still counted from the JSON.
        let mut broken = doc.clone();
        broken["buffers"][0]["uri"] = json!("missing.bin");
        std::fs::write(&path, serde_json::to_vec(&broken).unwrap()).unwrap();
        let mut meta = Map::new();
        analyze_model(&path, dir.path(), "gltf", &mut meta).unwrap();
        assert_eq!(meta["triangles"], json!(1));
    }

    #[test]
    fn compressed_gltf_is_drawn_from_a_plain_copy() {
        use polysmelt::mesh::{Material, Mesh, Prim};
        let mut p = Prim::default();
        for y in 0..=8u32 {
            for x in 0..=8u32 {
                p.positions.push([x as f32, 0.0, y as f32]);
                p.normals.push([0.0, 1.0, 0.0]);
            }
        }
        for y in 0..8 {
            for x in 0..8 {
                let i = y * 9 + x;
                p.indices.extend_from_slice(&[i, i + 9, i + 1, i + 1, i + 9, i + 10]);
            }
        }
        let mesh = Mesh {
            prims: vec![p],
            materials: vec![Material::plain("Grey", [0.5; 4])],
            images: Vec::new(),
            skeleton: Vec::new(),
        };
        let packed = polysmelt::gltf::compress(&polysmelt::mesh::to_glb(&mesh).unwrap()).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("grid.glb");
        std::fs::write(&path, packed).unwrap();
        assert!(glb_json(&path).unwrap()["extensionsUsed"].to_string().contains("meshopt"));

        let drawn = drawable(&path).unwrap();
        assert_ne!(drawn.path, path);
        let json = glb_json(&drawn.path).unwrap();
        assert!(json.get("extensionsUsed").is_none_or(|e| !ENCODINGS.iter().any(|n| e.to_string().contains(n))));
        let mut meta = Map::new();
        analyze_model(&drawn.path, dir.path(), "glb", &mut meta).unwrap();
        assert_eq!(meta["triangles"], json!(128));

        // Plain glTF is drawn where it is.
        let plain = dir.path().join("plain.glb");
        std::fs::write(&plain, polysmelt::mesh::to_glb(&mesh).unwrap()).unwrap();
        assert_eq!(drawable(&plain).unwrap().path, plain);
    }
    /// A Collada file is read by Hoard itself: counted, drawn for its thumbnail, and offered as a GLB.
    #[test]
    fn collada_is_counted_and_drawn() {
        let dae = r##"<?xml version="1.0" encoding="utf-8"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset><unit name="meter" meter="1"/><up_axis>Y_UP</up_axis></asset>
  <library_geometries>
    <geometry id="tri" name="Tri"><mesh>
      <source id="tri-pos"><float_array id="tri-pos-a" count="9">0 0 0 1 0 0 0 1 0</float_array>
        <technique_common><accessor source="#tri-pos-a" count="3" stride="3">
          <param name="X" type="float"/><param name="Y" type="float"/><param name="Z" type="float"/>
        </accessor></technique_common></source>
      <vertices id="tri-v"><input semantic="POSITION" source="#tri-pos"/></vertices>
      <triangles count="1"><input semantic="VERTEX" source="#tri-v" offset="0"/><p>0 1 2</p></triangles>
    </mesh></geometry>
  </library_geometries>
  <library_visual_scenes><visual_scene id="s"><node id="n"><instance_geometry url="#tri"/></node></visual_scene>
  </library_visual_scenes>
  <scene><instance_visual_scene url="#s"/></scene>
</COLLADA>"##;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tri.dae");
        std::fs::write(&path, dae).unwrap();
        let a = crate::analyze::analyze(&path, dir.path(), hoard_core::AssetKind::Model, "dae");
        assert_eq!(a.meta["triangles"], json!(1), "{:?}", a.meta);
        assert!(a.thumb.is_some(), "no thumbnail");
        let options = crate::formats::options("dae", &[]);
        assert!(options.iter().any(|o| o.format == "glb"));
    }
}
