//! Model formats read natively: STL (binary and ASCII), PLY (ASCII and binary), `MagicaVoxel` `.vox`, OBJ (with its
//! MTL materials and textures) and binary FBX (see `fbx`). Each becomes a GLB, which feeds the thumbnail renderer
//! and "download as GLB"; textures are embedded so the GLB stands on its own.

#![expect(clippy::many_single_char_names, clippy::similar_names, reason = "geometry code: a/b/c, u/v, x/y/z")]

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use serde_json::{Map, Value, json};

pub(crate) const EXTS: &[&str] = &["stl", "ply", "vox", "obj", "fbx"];
const MAX_TRIANGLES: usize = 4_000_000;
const MAX_VOXELS: usize = 4_000_000;

/// One material's worth of triangles.
#[derive(Debug, Default, Clone)]
pub(crate) struct Prim {
    pub(crate) positions: Vec<[f32; 3]>,
    pub(crate) normals: Vec<[f32; 3]>,
    /// Texture coordinates, glTF convention (origin top-left); empty when the model has none.
    pub(crate) uvs: Vec<[f32; 2]>,
    pub(crate) indices: Vec<u32>,
    pub(crate) material: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Material {
    pub(crate) name: String,
    pub(crate) color: [f32; 4],
    /// Index into `Mesh::images`.
    pub(crate) color_image: Option<usize>,
    pub(crate) normal_image: Option<usize>,
}

impl Material {
    pub(crate) fn plain(name: &str, color: [f32; 4]) -> Self {
        Self { name: name.to_owned(), color, color_image: None, normal_image: None }
    }
}

/// An image ready to embed: PNG or JPEG bytes.
#[derive(Debug, Clone)]
pub(crate) struct Image {
    pub(crate) bytes: Vec<u8>,
    pub(crate) mime: &'static str,
}

#[derive(Debug, Default)]
pub(crate) struct Mesh {
    pub(crate) prims: Vec<Prim>,
    pub(crate) materials: Vec<Material>,
    pub(crate) images: Vec<Image>,
}

impl Mesh {
    pub(crate) fn triangles(&self) -> usize {
        self.prims.iter().map(|p| p.indices.len() / 3).sum()
    }
}

pub(crate) const GRAY: [f32; 4] = [0.72, 0.72, 0.74, 1.0];

/// Load a model and the facts worth recording about it. Point clouds (PLY without faces) give no mesh. `root` is
/// the folder laid out like the pack, searched for textures the model points at by bare name or a foreign path.
pub(crate) fn load(path: &Path, ext: &str, root: &Path) -> Result<(Option<Mesh>, Map<String, Value>)> {
    let counts = |m: Mesh| {
        let mut f = Map::new();
        f.insert("triangles".into(), json!(m.triangles()));
        (Some(m), f)
    };
    match ext {
        "stl" => stl(&std::fs::read(path)?).map(counts),
        "ply" => ply(&std::fs::read(path)?),
        "vox" => vox(&std::fs::read(path)?),
        "obj" | "fbx" => {
            let mut mesh = if ext == "obj" { obj(path, root)? } else { crate::fbx::load(path, root)? };
            if !linear_colors(path, ext) {
                for m in &mut mesh.materials {
                    for c in &mut m.color[..3] {
                        *c = srgb_to_linear(*c);
                    }
                }
            }
            guess_color_texture(&mut mesh, path, root);
            Ok(counts(mesh))
        }
        _ => bail!("not a mesh format handled here: .{ext}"),
    }
}

/// Whether the file's material colours are linear values (Blender's OBJ and FBX exporters write them so) rather
/// than the sRGB other tools write. The browser viewer makes the same call.
fn linear_colors(path: &Path, ext: &str) -> bool {
    use std::io::Read as _;
    let mut head = Vec::new();
    let Ok(f) = std::fs::File::open(path) else { return false };
    if f.take(65_536).read_to_end(&mut head).is_err() {
        return false;
    }
    if ext == "obj" {
        let text: String = String::from_utf8_lossy(&head).chars().take(500).collect();
        text.lines().any(|l| l.starts_with("# Blender"))
    } else {
        head.windows(7).any(|w| w == b"Blender")
    }
}

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.040_45 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// A model that names no colour texture takes one by name, as the browser viewer does: `Barrel.png` or
/// `Barrel_Albedo.png` for `Barrel.obj`, from anywhere in the pack. Its colour then shows as painted.
fn guess_color_texture(mesh: &mut Mesh, path: &Path, root: &Path) {
    let has_uvs = mesh.prims.iter().any(|p| !p.uvs.is_empty());
    if !has_uvs || mesh.materials.iter().any(|m| m.color_image.is_some()) {
        return;
    }
    let Some(stem) = path.file_stem().map(|s| s.to_string_lossy().to_lowercase()) else { return };
    if stem.len() < 3 || stem.contains('@') {
        return;
    }
    // A name that says it's the colour wins; one that says nothing ("Barrel.png") will do.
    let mut plain = None;
    let mut colour = None;
    walk_images(root, |p, file_stem| {
        let Some(rest) = file_stem.strip_prefix(&stem) else { return false };
        if !(rest.is_empty() || rest.starts_with(['_', '-', ' ', '.'])) {
            return false;
        }
        match texture_role(file_stem) {
            Some(Role::Color) if colour.is_none() => colour = Some(p.to_path_buf()),
            None if plain.is_none() => plain = Some(p.to_path_buf()),
            _ => {}
        }
        colour.is_some()
    });
    let Some(img) = colour.or(plain).and_then(|p| load_image(&p)) else { return };
    mesh.images.push(img);
    let i = mesh.images.len() - 1;
    for (n, m) in mesh.materials.iter_mut().enumerate() {
        if mesh.prims.iter().any(|p| p.material == n && !p.uvs.is_empty()) {
            m.color_image = Some(i);
            m.color[..3].copy_from_slice(&[1.0, 1.0, 1.0]);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Role {
    Color,
    Other,
}

/// What a texture's name says it holds, by the browser viewer's rules: `None` when it says nothing.
fn texture_role(stem: &str) -> Option<Role> {
    let tail = stem.rsplit(['-', '_', '.', ' ']).next().unwrap_or(stem);
    let split = tail.len() < stem.len();
    let ends = |list: &[&str]| split && list.contains(&tail);
    let has = |list: &[&str]| list.iter().any(|w| stem.contains(w));
    if has(&["normal", "nrm", "emissi", "emit", "glow", "rough", "metal", "occlusion", "height", "disp", "bump"])
        || has(&["mask", "opacity", "alpha", "spec", "gloss", "smooth"])
        || ends(&["n", "nor", "norm", "e", "r", "m", "mt", "ao", "occ", "h", "orm", "arm", "mra", "rma", "s", "g"])
    {
        return Some(Role::Other);
    }
    if has(&["albedo", "basecolor", "base_color", "base color", "basecolour", "diffuse"])
        || ends(&["diff", "dif", "col", "color", "colour", "d", "bc", "a", "c"])
    {
        return Some(Role::Color);
    }
    None
}

const IMAGE_EXTS: &[&str] = &["png", "jpg", "jpeg", "tga", "bmp", "webp", "gif", "tif", "tiff", "dds", "psd"];

/// Visit the image files under `root` (at most 20 000 entries), with their lowercased stems, until `visit` says
/// stop.
fn walk_images(root: &Path, mut visit: impl FnMut(&Path, &str) -> bool) {
    let mut stack = vec![root.to_path_buf()];
    let mut seen = 0;
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            seen += 1;
            if seen > 20_000 {
                return;
            }
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let ext = p.extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
            if !IMAGE_EXTS.contains(&ext.as_str()) {
                continue;
            }
            let stem = p.file_stem().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
            if visit(&p, &stem) {
                return;
            }
        }
    }
}

/// Find a texture a model refers to: next to it, relative to it, or anywhere under `root` by file name (artists'
/// absolute paths like `C:\Users\me\tex\rock.tga` survive only as names). Failing that, an image with the same
/// name in another format (`rock.png` for `rock.tga`, as packs often ship), preferring PNG and JPEG.
pub(crate) fn find_texture(reference: &str, model_dir: &Path, root: &Path) -> Option<std::path::PathBuf> {
    let cleaned = reference.trim().replace('\\', "/");
    if cleaned.is_empty() {
        return None;
    }
    let rel = model_dir.join(&cleaned);
    if !cleaned.starts_with('/') && !cleaned.contains(':') && !cleaned.split('/').any(|c| c == "..") && rel.is_file() {
        return Some(rel);
    }
    let name = cleaned.rsplit('/').next()?.to_lowercase();
    let mut stack = vec![root.to_path_buf()];
    let mut seen = 0;
    while let Some(dir) = stack.pop() {
        for e in std::fs::read_dir(&dir).ok()?.flatten() {
            seen += 1;
            if seen > 20_000 {
                return None;
            }
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n.to_string_lossy().to_lowercase() == name) {
                return Some(p);
            }
        }
    }
    let is_image = name.rsplit_once('.').is_some_and(|(_, e)| IMAGE_EXTS.contains(&e));
    let stem = name.rsplit_once('.').map_or(name.as_str(), |(s, _)| s);
    if !is_image {
        return None;
    }
    let mut best: Option<(u8, std::path::PathBuf)> = None;
    walk_images(root, |p, s| {
        if s == stem {
            let ext = p.extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
            let rank = u8::from(!matches!(ext.as_str(), "png" | "jpg" | "jpeg"));
            if best.as_ref().is_none_or(|(r, _)| rank < *r) {
                best = Some((rank, p.to_path_buf()));
            }
        }
        best.as_ref().is_some_and(|(r, _)| *r == 0)
    });
    best.map(|(_, p)| p)
}

/// Load a texture file for embedding: PNG and JPEG as they are, anything else Hoard can decode (TGA, BMP, DDS,
/// PSD, …) re-encoded as PNG.
pub(crate) fn load_image(path: &Path) -> Option<Image> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.starts_with(b"\x89PNG") {
        return Some(Image { bytes, mime: "image/png" });
    }
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some(Image { bytes, mime: "image/jpeg" });
    }
    image_from_bytes(&bytes, path)
}

/// Embedded texture bytes (FBX videos) or a texture file, as PNG/JPEG.
pub(crate) fn image_from_bytes(bytes: &[u8], hint: &Path) -> Option<Image> {
    if bytes.starts_with(b"\x89PNG") {
        return Some(Image { bytes: bytes.to_vec(), mime: "image/png" });
    }
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some(Image { bytes: bytes.to_vec(), mime: "image/jpeg" });
    }
    let ext = hint.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    let img = if crate::textures::EXTS.contains(&ext.as_str()) {
        let dir = tempfile::tempdir().ok()?;
        let p = dir.path().join(format!("t.{ext}"));
        std::fs::write(&p, bytes).ok()?;
        crate::textures::decode(&p, &ext).ok()?.image
    } else if ext == "psd" {
        let dir = tempfile::tempdir().ok()?;
        let p = dir.path().join("t.psd");
        std::fs::write(&p, bytes).ok()?;
        crate::sources::psd_composite(&p).ok()?
    } else {
        image::ImageReader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.decode().ok())
            .or_else(|| image::load_from_memory_with_format(bytes, image::ImageFormat::Tga).ok())?
    };
    let mut png = Vec::new();
    image::DynamicImage::ImageRgba8(img.to_rgba8())
        .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    Some(Image { bytes: png, mime: "image/png" })
}

/// Smooth normals from the triangles, for models that don't carry any.
pub(crate) fn smooth_normals(positions: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut nrm = vec![[0.0f32; 3]; positions.len()];
    for t in indices.as_chunks::<3>().0 {
        let n = face_normal(positions[t[0] as usize], positions[t[1] as usize], positions[t[2] as usize]);
        for &i in t {
            for (a, b) in nrm[i as usize].iter_mut().zip(n) {
                *a += b;
            }
        }
    }
    for n in &mut nrm {
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        *n = if len > 0.0 { [n[0] / len, n[1] / len, n[2] / len] } else { [0.0, 1.0, 0.0] };
    }
    nrm
}

/// One face corner of an OBJ file: position, texture coordinate and normal indices (the last two optional).
type Corner = (usize, Option<usize>, Option<usize>);

/// One `newmtl` block of an MTL file.
#[derive(Debug, Default)]
struct MtlEntry {
    color: Option<[f32; 3]>,
    alpha: Option<f32>,
    color_map: Option<String>,
    normal_map: Option<String>,
}

/// An MTL file: material name → what it says. Texture options (`-bm 1.0`, `-s 1 1 1`) are skipped; the file name is
/// the last word.
fn read_mtl(path: &Path) -> HashMap<String, MtlEntry> {
    let mut out = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    let mut current: Option<String> = None;
    for line in text.lines() {
        let line = line.trim();
        let (key, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
        let rest = rest.trim();
        if key.eq_ignore_ascii_case("newmtl") {
            current = Some(rest.to_owned());
            out.entry(rest.to_owned()).or_insert_with(MtlEntry::default);
            continue;
        }
        let Some(entry) = current.as_ref().and_then(|c| out.get_mut(c)) else { continue };
        let floats: Vec<f32> = rest.split_whitespace().filter_map(|w| w.parse().ok()).collect();
        let file = || rest.split_whitespace().last().map(str::to_owned);
        match key.to_ascii_lowercase().as_str() {
            "kd" if floats.len() >= 3 => entry.color = Some([floats[0], floats[1], floats[2]]),
            "d" if !floats.is_empty() => entry.alpha = Some(floats[0]),
            "tr" if !floats.is_empty() => entry.alpha = Some(1.0 - floats[0]),
            "map_kd" => entry.color_map = file(),
            "map_bump" | "bump" | "norm" | "map_kn" => entry.normal_map = file(),
            _ => {}
        }
    }
    out
}

/// OBJ, with MTL colors and textures (base color and normal map). Faces may mix corners with and without texture
/// coordinates or normals; polygons are fanned into triangles.
fn obj(path: &Path, root: &Path) -> Result<Mesh> {
    let text = std::fs::read_to_string(path).context("not a text OBJ file")?;
    let dir = path.parent().unwrap_or(root);
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut vt: Vec<[f32; 2]> = Vec::new();
    let mut vn: Vec<[f32; 3]> = Vec::new();
    let mut mtl: HashMap<String, MtlEntry> = HashMap::new();
    // Faces grouped by material name, as corner triples (position, uv, normal), uv/normal may be missing.
    let mut groups: Vec<(String, Vec<[Corner; 3]>)> = Vec::new();
    let mut current = String::new();
    let resolve = |i: &str, len: usize| -> Option<usize> {
        let n: i64 = i.parse().ok()?;
        let idx = if n < 0 { i64::try_from(len).ok()? + n } else { n - 1 };
        usize::try_from(idx).ok().filter(|&i| i < len)
    };
    let mut triangles = 0usize;
    for line in text.lines() {
        let mut words = line.split_whitespace();
        let Some(key) = words.next() else { continue };
        let nums = |w: std::str::SplitWhitespace<'_>| -> Vec<f32> { w.filter_map(|x| x.parse().ok()).collect() };
        match key {
            "v" => {
                let f = nums(words);
                if f.len() >= 3 {
                    v.push([f[0], f[1], f[2]]);
                }
            }
            "vt" => {
                let f = nums(words);
                if !f.is_empty() {
                    vt.push([f[0], f.get(1).copied().unwrap_or(0.0)]);
                }
            }
            "vn" => {
                let f = nums(words);
                if f.len() >= 3 {
                    vn.push([f[0], f[1], f[2]]);
                }
            }
            "mtllib" => {
                let name = line.trim_start()[6..].trim();
                if let Some(p) = find_texture(name, dir, root) {
                    mtl.extend(read_mtl(&p));
                }
            }
            "usemtl" => current = words.collect::<Vec<_>>().join(" "),
            "f" => {
                let corners: Vec<Corner> = words
                    .filter_map(|c| {
                        let mut parts = c.split('/');
                        let p = resolve(parts.next()?, v.len())?;
                        let t = parts.next().filter(|s| !s.is_empty()).and_then(|s| resolve(s, vt.len()));
                        let n = parts.next().filter(|s| !s.is_empty()).and_then(|s| resolve(s, vn.len()));
                        Some((p, t, n))
                    })
                    .collect();
                if corners.len() < 3 {
                    continue;
                }
                if groups.last().is_none_or(|(m, _)| *m != current) {
                    groups.push((current.clone(), Vec::new()));
                }
                let faces = &mut groups.last_mut().context("face group")?.1;
                for k in 1..corners.len() - 1 {
                    faces.push([corners[0], corners[k], corners[k + 1]]);
                }
                triangles += corners.len() - 2;
                ensure!(triangles <= MAX_TRIANGLES, "too many triangles to convert");
            }
            _ => {}
        }
    }
    ensure!(triangles > 0, "OBJ has no faces");

    let mut mesh = Mesh::default();
    let mut material_of: HashMap<String, usize> = HashMap::new();
    let mut image_of: HashMap<std::path::PathBuf, Option<usize>> = HashMap::new();
    for (name, faces) in groups {
        let material = if let Some(&m) = material_of.get(&name) {
            m
        } else {
            let entry = mtl.get(&name);
            let d = entry.and_then(|e| e.color).unwrap_or([0.8, 0.8, 0.8]);
            let mut mat = Material::plain(
                if name.is_empty() { "default" } else { &name },
                [d[0], d[1], d[2], entry.and_then(|e| e.alpha).unwrap_or(1.0)],
            );
            let mut texture = |reference: Option<&String>| -> Option<usize> {
                let found = find_texture(reference?, dir, root)?;
                *image_of.entry(found.clone()).or_insert_with(|| {
                    let img = load_image(&found)?;
                    mesh.images.push(img);
                    Some(mesh.images.len() - 1)
                })
            };
            mat.color_image = texture(entry.and_then(|e| e.color_map.as_ref()));
            mat.normal_image = texture(entry.and_then(|e| e.normal_map.as_ref()));
            mesh.materials.push(mat);
            material_of.insert(name.clone(), mesh.materials.len() - 1);
            mesh.materials.len() - 1
        };
        let has_uvs = faces.iter().flatten().any(|c| c.1.is_some());
        let has_normals = faces.iter().flatten().all(|c| c.2.is_some());
        let mut prim = Prim { material, ..Prim::default() };
        let mut seen: HashMap<Corner, u32> = HashMap::new();
        for tri in &faces {
            for &corner in tri {
                let id = *seen.entry(corner).or_insert_with(|| {
                    prim.positions.push(v[corner.0]);
                    if has_normals {
                        prim.normals.push(corner.2.map_or([0.0, 1.0, 0.0], |n| vn[n]));
                    }
                    if has_uvs {
                        // OBJ puts the texture origin bottom-left; glTF top-left.
                        let t = corner.1.map_or([0.0, 0.0], |t| vt[t]);
                        prim.uvs.push([t[0], 1.0 - t[1]]);
                    }
                    u32::try_from(prim.positions.len() - 1).unwrap_or(u32::MAX)
                });
                prim.indices.push(id);
            }
        }
        if !has_normals {
            prim.normals = smooth_normals(&prim.positions, &prim.indices);
        }
        mesh.prims.push(prim);
    }
    // A texture needs coordinates: drop textures from materials whose triangles have none.
    for (i, mat) in mesh.materials.iter_mut().enumerate() {
        if !mesh.prims.iter().any(|p| p.material == i && !p.uvs.is_empty()) {
            mat.color_image = None;
            mat.normal_image = None;
        }
    }
    Ok(mesh)
}

/// One gray primitive.
fn single(p: Prim) -> Mesh {
    Mesh { prims: vec![p], materials: vec![Material::plain("default", GRAY)], images: Vec::new() }
}

fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 0.0 { [n[0] / len, n[1] / len, n[2] / len] } else { [0.0, 1.0, 0.0] }
}

/// A triangle soup with flat normals.
fn soup(tris: &[[[f32; 3]; 3]]) -> Prim {
    let mut p = Prim::default();
    for t in tris {
        let n = face_normal(t[0], t[1], t[2]);
        for v in t {
            p.indices.push(u32::try_from(p.positions.len()).unwrap_or(u32::MAX));
            p.positions.push(*v);
            p.normals.push(n);
        }
    }
    p
}

fn stl(data: &[u8]) -> Result<Mesh> {
    let le_f32 = |b: &[u8], at: usize| f32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]);
    // Binary files may also start with "solid", so trust the size arithmetic first.
    if data.len() >= 84 {
        let n = u32::from_le_bytes([data[80], data[81], data[82], data[83]]) as usize;
        if 84 + n * 50 == data.len() {
            ensure!(n <= MAX_TRIANGLES, "too many triangles to preview ({n})");
            let tris: Vec<[[f32; 3]; 3]> = (0..n)
                .map(|i| {
                    let at = 84 + i * 50 + 12;
                    let v = |k: usize| {
                        [le_f32(data, at + k * 12), le_f32(data, at + k * 12 + 4), le_f32(data, at + k * 12 + 8)]
                    };
                    [v(0), v(1), v(2)]
                })
                .collect();
            let attrs: Vec<u16> =
                (0..n).map(|i| u16::from_le_bytes([data[84 + i * 50 + 48], data[84 + i * 50 + 49]])).collect();
            return Ok(match stl_colors(&data[..80], &attrs) {
                Some(colors) => colored_soup(&tris, &colors),
                None => single(soup(&tris)),
            });
        }
    }
    let text = std::str::from_utf8(data).context("neither binary nor ASCII STL")?;
    ensure!(text.trim_start().starts_with("solid"), "not an STL file");
    let mut verts = Vec::new();
    for line in text.lines() {
        let mut it = line.split_whitespace();
        if it.next() == Some("vertex") {
            let v: Vec<f32> = it.take(3).filter_map(|s| s.parse().ok()).collect();
            ensure!(v.len() == 3, "bad vertex line in STL");
            verts.push([v[0], v[1], v[2]]);
        }
    }
    ensure!(verts.len() / 3 <= MAX_TRIANGLES, "too many triangles to preview");
    let tris: Vec<[[f32; 3]; 3]> = verts.as_chunks::<3>().0.to_vec();
    ensure!(!tris.is_empty(), "STL has no triangles");
    Ok(single(soup(&tris)))
}

/// Each facet's colour (sRGB, 0-255) from a binary STL's attribute bytes, when the file has colours. Materialise
/// Magics writes `COLOR=rgba` in the header, the default for facets whose top bit is set, and red in the low bits of
/// the rest (as three.js reads it). `VisCAM` and `SolidView` set the top bit on coloured facets, blue in the low bits;
/// there's no header saying so, and other tools leave junk in those bytes, so only two colours or more count.
fn stl_colors(header: &[u8], attrs: &[u16]) -> Option<Vec<Option<[u8; 3]>>> {
    let five = |v: u16| u8::try_from(u32::from(v & 0x1F) * 255 / 31).unwrap_or(u8::MAX);
    if let Some(at) = header.windows(6).position(|w| w == b"COLOR=").filter(|at| at + 10 <= header.len()) {
        let default = [header[at + 6], header[at + 7], header[at + 8]];
        return Some(
            attrs
                .iter()
                .map(|&a| Some(if a & 0x8000 == 0 { [five(a), five(a >> 5), five(a >> 10)] } else { default }))
                .collect(),
        );
    }
    let mut seen = std::collections::HashSet::new();
    for &a in attrs.iter().filter(|&&a| a & 0x8000 != 0) {
        seen.insert(a);
        if seen.len() > 1 {
            return Some(
                attrs.iter().map(|&a| (a & 0x8000 != 0).then(|| [five(a >> 10), five(a >> 5), five(a)])).collect(),
            );
        }
    }
    None
}

/// Facets grouped by colour (gray for those without one): a primitive and plain material for each.
fn colored_soup(tris: &[[[f32; 3]; 3]], colors: &[Option<[u8; 3]>]) -> Mesh {
    type Tri = [[f32; 3]; 3];
    let mut groups: Vec<(Option<[u8; 3]>, Vec<Tri>)> = Vec::new();
    let mut index: HashMap<Option<[u8; 3]>, usize> = HashMap::new();
    for (t, c) in tris.iter().zip(colors) {
        let i = *index.entry(*c).or_insert_with(|| {
            groups.push((*c, Vec::new()));
            groups.len() - 1
        });
        groups[i].1.push(*t);
    }
    let mut mesh = Mesh::default();
    for (i, (c, tris)) in groups.into_iter().enumerate() {
        let mut p = soup(&tris);
        p.material = i;
        mesh.prims.push(p);
        let color = c.map_or(GRAY, |c| {
            let [r, g, b] = c.map(|v| srgb_to_linear(f32::from(v) / 255.0));
            [r, g, b, 1.0]
        });
        mesh.materials.push(Material::plain(&format!("color_{i}"), color));
    }
    mesh
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlyFormat {
    Ascii,
    Le,
    Be,
}

#[derive(Debug, Clone)]
enum PlyProp {
    Scalar { name: String, ty: String },
    List { name: String, count_ty: String, item_ty: String },
}

#[derive(Debug, Clone)]
struct PlyElement {
    name: String,
    count: usize,
    props: Vec<PlyProp>,
}

fn ply_size(ty: &str) -> Result<usize> {
    Ok(match ty {
        "char" | "uchar" | "int8" | "uint8" => 1,
        "short" | "ushort" | "int16" | "uint16" => 2,
        "int" | "uint" | "float" | "int32" | "uint32" | "float32" => 4,
        "double" | "float64" => 8,
        t => bail!("unknown PLY type {t}"),
    })
}

/// Reads PLY values one at a time, in whichever encoding the file uses.
struct PlyReader<'a> {
    format: PlyFormat,
    bin: &'a [u8],
    pos: usize,
    words: std::str::SplitAsciiWhitespace<'a>,
}

impl PlyReader<'_> {
    fn value(&mut self, ty: &str) -> Result<f64> {
        if self.format == PlyFormat::Ascii {
            return self.words.next().context("PLY data ends early")?.parse::<f64>().context("bad PLY number");
        }
        let n = ply_size(ty)?;
        let b = self.bin.get(self.pos..self.pos + n).context("PLY data ends early")?;
        self.pos += n;
        let mut a = [0u8; 8];
        a[..n].copy_from_slice(b);
        if self.format == PlyFormat::Be {
            a[..n].reverse();
        }
        Ok(match ty {
            "char" | "int8" => f64::from(a[0] as i8),
            "uchar" | "uint8" => f64::from(a[0]),
            "short" | "int16" => f64::from(i16::from_le_bytes([a[0], a[1]])),
            "ushort" | "uint16" => f64::from(u16::from_le_bytes([a[0], a[1]])),
            "int" | "int32" => f64::from(i32::from_le_bytes([a[0], a[1], a[2], a[3]])),
            "uint" | "uint32" => f64::from(u32::from_le_bytes([a[0], a[1], a[2], a[3]])),
            "float" | "float32" => f64::from(f32::from_le_bytes([a[0], a[1], a[2], a[3]])),
            _ => f64::from_le_bytes(a),
        })
    }
}

#[expect(clippy::cast_possible_truncation, reason = "PLY coordinates and indices fit their targets")]
fn ply(data: &[u8]) -> Result<(Option<Mesh>, Map<String, Value>)> {
    let end = data.windows(11).position(|w| w == b"end_header\n").context("not a PLY file")? + 11;
    let header = std::str::from_utf8(&data[..end]).context("bad PLY header")?;
    let mut lines = header.lines();
    ensure!(lines.next().map(str::trim) == Some("ply"), "not a PLY file");
    let mut format = PlyFormat::Ascii;
    let mut elements: Vec<PlyElement> = Vec::new();
    for line in lines {
        let w: Vec<&str> = line.split_whitespace().collect();
        match w.as_slice() {
            ["format", f, ..] => {
                format = match *f {
                    "ascii" => PlyFormat::Ascii,
                    "binary_little_endian" => PlyFormat::Le,
                    "binary_big_endian" => PlyFormat::Be,
                    f => bail!("unknown PLY format {f}"),
                }
            }
            ["element", name, count] => {
                elements.push(PlyElement { name: (*name).to_owned(), count: count.parse()?, props: Vec::new() });
            }
            ["property", "list", c, i, name] => elements
                .last_mut()
                .context("PLY property before element")?
                .props
                .push(PlyProp::List { name: (*name).to_owned(), count_ty: (*c).to_owned(), item_ty: (*i).to_owned() }),
            ["property", ty, name] => elements
                .last_mut()
                .context("PLY property before element")?
                .props
                .push(PlyProp::Scalar { name: (*name).to_owned(), ty: (*ty).to_owned() }),
            _ => {}
        }
    }
    let body = &data[end..];
    let text = if format == PlyFormat::Ascii { std::str::from_utf8(body).context("bad ASCII PLY")? } else { "" };
    let mut r = PlyReader { format, bin: body, pos: 0, words: text.split_ascii_whitespace() };
    let (mut pos, mut nrm) = (Vec::new(), Vec::new());
    let mut faces: Vec<Vec<u32>> = Vec::new();
    for el in &elements {
        ensure!(el.count <= MAX_TRIANGLES * 3, "PLY element {} is too large to preview", el.name);
        for _ in 0..el.count {
            let mut vals: HashMap<&str, f64> = HashMap::new();
            let mut list: Option<Vec<u32>> = None;
            for p in &el.props {
                match p {
                    PlyProp::Scalar { name, ty } => {
                        vals.insert(name.as_str(), r.value(ty)?);
                    }
                    PlyProp::List { name, count_ty, item_ty } => {
                        let n = r.value(count_ty)? as usize;
                        ensure!(n <= 64, "PLY list is too long");
                        let mut items = Vec::with_capacity(n);
                        for _ in 0..n {
                            items.push(r.value(item_ty)? as u32);
                        }
                        if name == "vertex_indices" || name == "vertex_index" {
                            list = Some(items);
                        }
                    }
                }
            }
            match el.name.as_str() {
                "vertex" => {
                    let g = |k: &str| vals.get(k).copied().unwrap_or(0.0) as f32;
                    pos.push([g("x"), g("y"), g("z")]);
                    if vals.contains_key("nx") {
                        nrm.push([g("nx"), g("ny"), g("nz")]);
                    }
                }
                "face" => faces.extend(list),
                _ => {}
            }
        }
    }
    let mut facts = Map::new();
    if faces.is_empty() {
        facts.insert("points".into(), json!(pos.len()));
        return Ok((None, facts));
    }
    let mut indices = Vec::new();
    for f in &faces {
        for k in 1..f.len().saturating_sub(1) {
            let tri = [f[0], f[k], f[k + 1]];
            if tri.iter().all(|&i| (i as usize) < pos.len()) {
                indices.extend(tri);
            }
        }
    }
    ensure!(indices.len() / 3 <= MAX_TRIANGLES, "too many triangles to preview");
    // Smooth normals from the faces when the file has none.
    if nrm.len() != pos.len() {
        nrm = smooth_normals(&pos, &indices);
    }
    facts.insert("triangles".into(), json!(indices.len() / 3));
    facts.insert("vertices".into(), json!(pos.len()));
    Ok((Some(single(Prim { positions: pos, normals: nrm, uvs: Vec::new(), indices, material: 0 })), facts))
}

/// `MagicaVoxel`: the first model, its exposed faces grouped by palette color.
#[expect(clippy::cast_precision_loss, reason = "voxel coordinates are small")]
fn vox(data: &[u8]) -> Result<(Option<Mesh>, Map<String, Value>)> {
    ensure!(data.len() >= 20 && &data[..4] == b"VOX ", "not a MagicaVoxel file");
    let le = |at: usize| -> Result<u32> {
        Ok(u32::from_le_bytes(data.get(at..at + 4).context("VOX ends early")?.try_into()?))
    };
    let (mut size, mut voxels, mut palette, mut models) = (None, None, None, 0usize);
    // MAIN's children follow its 12-byte chunk header, after the 8-byte file header.
    let mut at = 8 + 12;
    while at + 12 <= data.len() {
        let id = &data[at..at + 4];
        let content = le(at + 4)? as usize;
        let body = at + 12;
        let next = body
            .checked_add(content)
            .context("VOX chunk overflow")?
            .checked_add(le(at + 8)? as usize)
            .context("overflow")?;
        match id {
            b"SIZE" => {
                models += 1;
                if size.is_none() {
                    size = Some((le(body)?, le(body + 4)?, le(body + 8)?));
                }
            }
            b"XYZI" if voxels.is_none() => {
                let n = le(body)? as usize;
                ensure!(n <= MAX_VOXELS, "too many voxels to preview ({n})");
                let raw = data.get(body + 4..body + 4 + n * 4).context("VOX voxels end early")?;
                voxels = Some(raw.as_chunks::<4>().0.to_vec());
            }
            b"RGBA" => {
                let raw = data.get(body..body + 1024).context("VOX palette ends early")?;
                palette = Some(raw.as_chunks::<4>().0.to_vec());
            }
            _ => {}
        }
        at = next;
    }
    let (sx, sy, sz) = size.context("VOX file has no model")?;
    let voxels = voxels.context("VOX file has no voxels")?;
    let occupied: std::collections::HashSet<(i32, i32, i32)> =
        voxels.iter().map(|v| (i32::from(v[0]), i32::from(v[1]), i32::from(v[2]))).collect();
    // Color index i uses palette[i - 1]. Without a palette chunk, spread hues so shapes still read.
    let color = |i: u8| -> [f32; 4] {
        if let Some(p) = &palette {
            let c = p[usize::from(i.wrapping_sub(1))];
            return [f32::from(c[0]) / 255.0, f32::from(c[1]) / 255.0, f32::from(c[2]) / 255.0, 1.0];
        }
        let h = f32::from(i) / 255.0 * 6.0;
        let x = 1.0 - (h % 2.0 - 1.0).abs();
        let (r, g, b) = match h as u32 {
            0 => (1.0, x, 0.0),
            1 => (x, 1.0, 0.0),
            2 => (0.0, 1.0, x),
            3 => (0.0, x, 1.0),
            4 => (x, 0.0, 1.0),
            _ => (1.0, 0.0, x),
        };
        [0.25 + r * 0.6, 0.25 + g * 0.6, 0.25 + b * 0.6, 1.0]
    };
    // Faces as (neighbor offset, normal, four corners); MagicaVoxel is Z-up, glTF Y-up: (x, y, z) -> (x, z, -y).
    let faces: [([i32; 3], [[f32; 3]; 4]); 6] = [
        ([1, 0, 0], [[1., 0., 0.], [1., 1., 0.], [1., 1., 1.], [1., 0., 1.]]),
        ([-1, 0, 0], [[0., 0., 0.], [0., 0., 1.], [0., 1., 1.], [0., 1., 0.]]),
        ([0, 1, 0], [[0., 1., 0.], [0., 1., 1.], [1., 1., 1.], [1., 1., 0.]]),
        ([0, -1, 0], [[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]]),
        ([0, 0, 1], [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]]),
        ([0, 0, -1], [[0., 0., 0.], [0., 1., 0.], [1., 1., 0.], [1., 0., 0.]]),
    ];
    let to_gltf = |p: [f32; 3]| [p[0], p[2], -p[1]];
    let mut by_color: HashMap<u8, Prim> = HashMap::new();
    for v in &voxels {
        let (x, y, z) = (i32::from(v[0]), i32::from(v[1]), i32::from(v[2]));
        for (d, corners) in &faces {
            if occupied.contains(&(x + d[0], y + d[1], z + d[2])) {
                continue;
            }
            let p = by_color.entry(v[3]).or_default();
            let base = u32::try_from(p.positions.len())?;
            let normal = to_gltf([d[0] as f32, d[1] as f32, d[2] as f32]);
            for c in corners {
                p.positions.push(to_gltf([x as f32 + c[0], y as f32 + c[1], z as f32 + c[2]]));
                p.normals.push(normal);
            }
            p.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }
    let mut prims: Vec<(u8, Prim)> = by_color.into_iter().collect();
    prims.sort_by_key(|(c, _)| *c);
    let mut facts = Map::new();
    facts.insert("voxels".into(), json!(voxels.len()));
    facts.insert("voxel_size".into(), json!(format!("{sx}x{sy}x{sz}")));
    if models > 1 {
        facts.insert("models".into(), json!(models));
    }
    let mut mesh = Mesh::default();
    for (i, (c, mut p)) in prims.into_iter().enumerate() {
        p.material = i;
        mesh.prims.push(p);
        mesh.materials.push(Material::plain(&format!("color_{c}"), color(c)));
    }
    Ok((Some(mesh), facts))
}

/// A GLB with one mesh: a primitive per `Prim`, its materials, and the images embedded.
pub(crate) fn to_glb(mesh: &Mesh) -> Result<Vec<u8>> {
    let mut bin: Vec<u8> = Vec::new();
    let mut views: Vec<Value> = Vec::new();
    let mut push_view = |bin: &mut Vec<u8>, bytes: &[u8], target: Option<u32>| -> usize {
        let offset = bin.len();
        bin.extend_from_slice(bytes);
        bin.resize(bin.len().div_ceil(4) * 4, 0);
        let mut v = json!({ "buffer": 0, "byteOffset": offset, "byteLength": bytes.len() });
        if let Some(t) = target {
            v["target"] = json!(t);
        }
        views.push(v);
        views.len() - 1
    };
    let mut accessors = Vec::new();
    let mut primitives = Vec::new();
    for p in &mesh.prims {
        if p.indices.is_empty() {
            continue;
        }
        let f32s = |v: &[f32]| v.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>();
        let pv = push_view(&mut bin, &f32s(p.positions.as_flattened()), Some(34962));
        let nv = push_view(&mut bin, &f32s(p.normals.as_flattened()), Some(34962));
        let iv = push_view(&mut bin, &p.indices.iter().flat_map(|i| i.to_le_bytes()).collect::<Vec<u8>>(), Some(34963));
        let (mut min, mut max) = ([f32::MAX; 3], [f32::MIN; 3]);
        for v in &p.positions {
            for k in 0..3 {
                min[k] = min[k].min(v[k]);
                max[k] = max[k].max(v[k]);
            }
        }
        let a = accessors.len();
        accessors.push(json!({ "bufferView": pv, "componentType": 5126, "count": p.positions.len(), "type": "VEC3", "min": min, "max": max }));
        accessors.push(json!({ "bufferView": nv, "componentType": 5126, "count": p.normals.len(), "type": "VEC3" }));
        accessors.push(json!({ "bufferView": iv, "componentType": 5125, "count": p.indices.len(), "type": "SCALAR" }));
        let mut attributes = json!({ "POSITION": a, "NORMAL": a + 1 });
        if p.uvs.len() == p.positions.len() {
            let uv = push_view(&mut bin, &f32s(p.uvs.as_flattened()), Some(34962));
            accessors.push(json!({ "bufferView": uv, "componentType": 5126, "count": p.uvs.len(), "type": "VEC2" }));
            attributes["TEXCOORD_0"] = json!(accessors.len() - 1);
        }
        primitives.push(json!({ "attributes": attributes, "indices": a + 2, "material": p.material.min(mesh.materials.len().saturating_sub(1)) }));
    }
    ensure!(!primitives.is_empty(), "nothing to draw");
    let mut images = Vec::new();
    for img in &mesh.images {
        let v = push_view(&mut bin, &img.bytes, None);
        images.push(json!({ "bufferView": v, "mimeType": img.mime }));
    }
    let materials: Vec<Value> = mesh
        .materials
        .iter()
        .map(|m| {
            let mut pbr = json!({ "baseColorFactor": m.color, "metallicFactor": 0.0, "roughnessFactor": 0.85 });
            if let Some(i) = m.color_image.filter(|&i| i < images.len()) {
                pbr["baseColorTexture"] = json!({ "index": i });
            }
            let mut mat = json!({ "name": m.name, "pbrMetallicRoughness": pbr });
            if let Some(i) = m.normal_image.filter(|&i| i < images.len()) {
                mat["normalTexture"] = json!({ "index": i });
            }
            if m.color[3] < 1.0 {
                mat["alphaMode"] = json!("BLEND");
            }
            mat
        })
        .collect();
    let mut doc = json!({
        "asset": { "version": "2.0", "generator": "Hoard" },
        "scene": 0,
        "scenes": [{ "nodes": [0] }],
        "nodes": [{ "mesh": 0 }],
        "meshes": [{ "primitives": primitives }],
        "materials": materials,
        "accessors": accessors,
        "bufferViews": views,
        "buffers": [{ "byteLength": bin.len() }],
    });
    if !images.is_empty() {
        // One texture per image, all sharing a repeating, mipmapped sampler.
        doc["samplers"] = json!([{ "magFilter": 9729, "minFilter": 9987, "wrapS": 10497, "wrapT": 10497 }]);
        doc["textures"] = json!((0..images.len()).map(|i| json!({ "sampler": 0, "source": i })).collect::<Vec<_>>());
        doc["images"] = json!(images);
    }
    let mut json_bytes = serde_json::to_vec(&doc)?;
    json_bytes.resize(json_bytes.len().div_ceil(4) * 4, b' ');
    let total = 12 + 8 + json_bytes.len() + 8 + bin.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&u32::try_from(total)?.to_le_bytes());
    out.extend_from_slice(&u32::try_from(json_bytes.len())?.to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend(json_bytes);
    out.extend_from_slice(&u32::try_from(bin.len())?.to_le_bytes());
    out.extend_from_slice(b"BIN\0");
    out.extend(bin);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_bytes(name: &str, bytes: &[u8]) -> (Option<Mesh>, Map<String, Value>) {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join(name);
        std::fs::write(&p, bytes).unwrap();
        let ext = name.rsplit('.').next().unwrap();
        load(&p, ext, dir.path()).unwrap()
    }

    #[test]
    fn stl_binary_and_ascii() {
        let mut bin = vec![0u8; 80];
        bin.extend_from_slice(&1u32.to_le_bytes());
        for v in [0f32, 0., 1., 0., 0., 0., 1., 0., 0., 0., 1., 0.] {
            bin.extend_from_slice(&v.to_le_bytes());
        }
        bin.extend_from_slice(&[0, 0]);
        let (m, f) = load_bytes("a.stl", &bin);
        assert_eq!(f["triangles"], 1);
        let glb = to_glb(&m.unwrap()).unwrap();
        assert!(glb.starts_with(b"glTF"));

        let ascii = "solid t\nfacet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\nendsolid t\n";
        let (m, _) = load_bytes("b.stl", ascii.as_bytes());
        assert_eq!(m.unwrap().triangles(), 1);
    }

    #[test]
    fn stl_facet_colors() {
        let stl = |header: &[u8], attrs: &[u16]| {
            let mut bin = header.to_vec();
            bin.resize(80, 0);
            bin.extend_from_slice(&u32::try_from(attrs.len()).unwrap().to_le_bytes());
            for &a in attrs {
                for v in [0f32, 0., 1., 0., 0., 0., 1., 0., 0., 0., 1., 0.] {
                    bin.extend_from_slice(&v.to_le_bytes());
                }
                bin.extend_from_slice(&a.to_le_bytes());
            }
            load_bytes("c.stl", &bin).0.unwrap()
        };
        // Magics: red in the low bits; a set top bit takes the header's colour.
        let m = stl(b"COLOR=\x00\xff\x00\xff", &[0x001F, 0x8000, 0x001F]);
        assert_eq!(m.prims.len(), 2);
        assert_eq!(m.prims[0].indices.len(), 6);
        assert_eq!(m.materials[0].color, [1.0, 0.0, 0.0, 1.0]);
        let m = stl(b"COLOR=\x00\x00\xff\xff", &[0x03E0, 0x8000]);
        assert_eq!(m.materials[0].color, [0.0, 1.0, 0.0, 1.0]);
        assert_eq!(m.materials[1].color, [0.0, 0.0, 1.0, 1.0]);
        // VisCAM: the top bit marks a coloured facet, blue in the low bits.
        let m = stl(b"binary", &[0x801F, 0xFC00, 0]);
        assert_eq!(
            m.materials.iter().map(|m| m.color).collect::<Vec<_>>(),
            [[0.0, 0.0, 1.0, 1.0], [1.0, 0.0, 0.0, 1.0], GRAY]
        );
        // One colour without a header is as likely junk as a colour: plain gray.
        let m = stl(b"binary", &[0xFFFF, 0xFFFF]);
        assert_eq!((m.prims.len(), m.materials[0].color), (1, GRAY));
    }

    #[test]
    fn ply_ascii_quad_and_point_cloud() {
        let ply = "ply\nformat ascii 1.0\nelement vertex 4\nproperty float x\nproperty float y\nproperty float z\n\
                   element face 1\nproperty list uchar int vertex_indices\nend_header\n0 0 0\n1 0 0\n1 1 0\n0 1 0\n4 0 1 2 3\n";
        let (m, f) = load_bytes("q.ply", ply.as_bytes());
        assert_eq!(f["triangles"], 2, "a quad fans into two triangles");
        assert_eq!(m.unwrap().prims[0].normals[0], [0.0, 0.0, 1.0]);

        let mut bin = b"ply\nformat binary_little_endian 1.0\nelement vertex 2\nproperty float x\nproperty float y\nproperty float z\nend_header\n".to_vec();
        for v in [0f32, 0., 0., 1., 2., 3.] {
            bin.extend_from_slice(&v.to_le_bytes());
        }
        let (m, f) = load_bytes("p.ply", &bin);
        assert!(m.is_none());
        assert_eq!(f["points"], 2);
    }

    #[test]
    fn vox_exposed_faces_by_color() {
        // Two adjacent voxels with different colors: 12 faces minus the 2 touching ones.
        let chunk = |id: &[u8], body: &[u8]| {
            let mut c = id.to_vec();
            c.extend_from_slice(&(body.len() as u32).to_le_bytes());
            c.extend_from_slice(&0u32.to_le_bytes());
            c.extend_from_slice(body);
            c
        };
        let mut size = Vec::new();
        for v in [2u32, 1, 1] {
            size.extend_from_slice(&v.to_le_bytes());
        }
        let mut xyzi = 2u32.to_le_bytes().to_vec();
        xyzi.extend_from_slice(&[0, 0, 0, 1, 1, 0, 0, 2]);
        let mut rgba = vec![0u8; 1024];
        rgba[..8].copy_from_slice(&[255, 0, 0, 255, 0, 0, 255, 255]);
        let children = [chunk(b"SIZE", &size), chunk(b"XYZI", &xyzi), chunk(b"RGBA", &rgba)].concat();
        let mut file = b"VOX ".to_vec();
        file.extend_from_slice(&150u32.to_le_bytes());
        file.extend_from_slice(b"MAIN");
        file.extend_from_slice(&0u32.to_le_bytes());
        file.extend_from_slice(&(children.len() as u32).to_le_bytes());
        file.extend(children);
        let (m, f) = load_bytes("v.vox", &file);
        let m = m.unwrap();
        assert_eq!(f["voxels"], 2);
        assert_eq!(m.prims.len(), 2);
        assert_eq!(m.triangles(), 10 * 2);
        assert_eq!(m.materials[m.prims[0].material].color, [1.0, 0.0, 0.0, 1.0]);
        assert!(to_glb(&m).unwrap().starts_with(b"glTF"));
    }

    #[test]
    fn obj_mixed_faces_negative_indices_and_quads() {
        let dir = tempfile::tempdir().unwrap();
        // A quad with UVs and normals by negative index, then a triangle without UVs (a cap).
        let obj = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\nvn 0 0 1\n\
                   f -4/-4/-1 -3/-3/-1 -2/-2/-1 -1/-1/-1\nf 1 2 3\n";
        std::fs::write(dir.path().join("m.obj"), obj).unwrap();
        let (m, f) = load(&dir.path().join("m.obj"), "obj", dir.path()).unwrap();
        let m = m.unwrap();
        assert_eq!(f["triangles"], 3);
        let p = &m.prims[0];
        assert_eq!(p.uvs.len(), p.positions.len(), "every vertex gets a UV, missing ones zero");
        assert_eq!(p.normals.len(), p.positions.len());
    }
}

#[cfg(test)]
mod obj_tests {
    use super::*;

    fn png(rgb: [u8; 3]) -> Vec<u8> {
        let mut png = Vec::new();
        image::RgbImage::from_pixel(2, 2, image::Rgb(rgb))
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        png
    }

    const QUAD: &str = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n";

    #[test]
    fn obj_texture_in_another_format_and_colour_space() {
        let dir = tempfile::tempdir().unwrap();
        // The MTL names a TGA; the pack ships it as PNG.
        std::fs::write(dir.path().join("rock.png"), png([10, 20, 30])).unwrap();
        std::fs::write(
            dir.path().join("a.mtl"),
            "newmtl Rock\nKd 0.5 0.5 0.5\nmap_Kd rock.tga\nnewmtl Plain\nKd 0.5 0.5 0.5\n",
        )
        .unwrap();
        let obj = format!("mtllib a.mtl\n{QUAD}usemtl Rock\nf 1/1 2/2 3/3\nusemtl Plain\nf 1 3 4\n");
        std::fs::write(dir.path().join("a.obj"), &obj).unwrap();
        let (m, _) = load(&dir.path().join("a.obj"), "obj", dir.path()).unwrap();
        let m = m.unwrap();
        let rock = m.materials.iter().find(|x| x.name == "Rock").unwrap();
        assert_eq!(rock.color_image, Some(0));
        // sRGB 0.5 → linear ~0.214 for the glTF factor.
        let plain = m.materials.iter().find(|x| x.name == "Plain").unwrap();
        assert!((plain.color[0] - 0.214).abs() < 0.01, "{:?}", plain.color);

        // Blender writes linear values: kept as they are.
        std::fs::write(dir.path().join("a.obj"), format!("# Blender 4.2\n{obj}")).unwrap();
        let (m, _) = load(&dir.path().join("a.obj"), "obj", dir.path()).unwrap();
        let plain = m.unwrap().materials.into_iter().find(|x| x.name == "Plain").unwrap();
        assert!((plain.color[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn obj_without_textures_takes_one_by_name() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("Textures")).unwrap();
        std::fs::write(dir.path().join("Textures/Barrel_Normal.png"), png([128, 128, 255])).unwrap();
        std::fs::write(dir.path().join("Textures/Barrel_Albedo.png"), png([200, 100, 50])).unwrap();
        std::fs::write(dir.path().join("Textures/Barrels.png"), png([0, 0, 0])).unwrap();
        std::fs::write(dir.path().join("Barrel.obj"), format!("{QUAD}f 1/1 2/2 3/3 4/4\n")).unwrap();
        let (m, _) = load(&dir.path().join("Barrel.obj"), "obj", dir.path()).unwrap();
        let m = m.unwrap();
        assert_eq!(m.images.len(), 1);
        assert_eq!(m.materials[0].color_image, Some(0));
        assert_eq!(m.materials[0].color, [1.0, 1.0, 1.0, 1.0]);
        let px = image::load_from_memory(&m.images[0].bytes).unwrap().to_rgb8();
        assert_eq!(px.get_pixel(0, 0).0, [200, 100, 50]);
        assert_eq!(texture_role("barrel_normal"), Some(Role::Other));
        assert_eq!(texture_role("barrel_d"), Some(Role::Color));
        assert_eq!(texture_role("barrel"), None);
    }

    #[test]
    fn obj_with_mtl_and_texture() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("Models/textures")).unwrap();
        let mut png = Vec::new();
        image::RgbImage::from_pixel(2, 2, image::Rgb([200, 100, 50]))
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        // Referenced by a foreign absolute path; found by name.
        std::fs::write(dir.path().join("Models/textures/crate.png"), &png).unwrap();
        std::fs::write(
            dir.path().join("Models/crate.mtl"),
            "newmtl Wood\nKd 0.9 0.8 0.7\nmap_Kd D:\\Work\\crate.png\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("Models/crate.obj"),
            "mtllib crate.mtl\nv 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\nusemtl Wood\nf 1/1 2/2 3/3 4/4\n",
        )
        .unwrap();
        let (m, f) = load(&dir.path().join("Models/crate.obj"), "obj", dir.path()).unwrap();
        let m = m.unwrap();
        assert_eq!(f["triangles"], 2);
        assert_eq!(m.materials[m.prims[0].material].name, "Wood");
        assert_eq!(m.materials[m.prims[0].material].color_image, Some(0));
        assert_eq!(m.prims[0].uvs[0], [0.0, 1.0], "V flipped");
        assert!(to_glb(&m).unwrap().starts_with(b"glTF"));
    }

    #[test]
    fn obj_mixed_faces_negative_indices_and_quads() {
        let dir = tempfile::tempdir().unwrap();
        // A quad with UVs and normals by negative index, then a triangle without UVs (a cap).
        let obj = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\nvn 0 0 1\n\
                   f -4/-4/-1 -3/-3/-1 -2/-2/-1 -1/-1/-1\nf 1 2 3\n";
        std::fs::write(dir.path().join("m.obj"), obj).unwrap();
        let (m, f) = load(&dir.path().join("m.obj"), "obj", dir.path()).unwrap();
        let m = m.unwrap();
        assert_eq!(f["triangles"], 3);
        let p = &m.prims[0];
        assert_eq!(p.uvs.len(), p.positions.len(), "every vertex gets a UV, missing ones zero");
        assert_eq!(p.normals.len(), p.positions.len());
    }
}
