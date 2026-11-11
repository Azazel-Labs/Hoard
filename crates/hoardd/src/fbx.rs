//! Binary FBX (7.x) to a `meshes::Mesh`: geometry with normals and UVs, materials with their diffuse color and
//! textures (embedded or found in the pack), node transforms through the hierarchy, and the file's axis system and
//! unit converted to glTF's Y-up meters. Skinning and animation aren't carried over: the result is the bind pose.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use fbxcel::low::v7400::AttributeValue as A;
use fbxcel::tree::v7400::NodeHandle;

use crate::meshes::{GRAY, Image, Material, Mesh, Prim, find_texture, image_from_bytes, load_image, smooth_normals};

const MAX_TRIANGLES: usize = 4_000_000;

type M4 = [[f64; 4]; 4];

const IDENTITY: M4 = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]];

fn mul(a: &M4, b: &M4) -> M4 {
    let mut r = [[0.0; 4]; 4];
    for (i, row) in r.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = (0..4).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    r
}

fn translate(t: [f64; 3]) -> M4 {
    let mut m = IDENTITY;
    m[0][3] = t[0];
    m[1][3] = t[1];
    m[2][3] = t[2];
    m
}

fn scale(s: [f64; 3]) -> M4 {
    let mut m = IDENTITY;
    m[0][0] = s[0];
    m[1][1] = s[1];
    m[2][2] = s[2];
    m
}

/// Euler rotation in degrees, FBX rotation order (0 = XYZ: X applied first).
fn rotate(deg: [f64; 3], order: i32) -> M4 {
    let [x, y, z] = deg.map(f64::to_radians);
    let rx = [[1., 0., 0., 0.], [0., x.cos(), -x.sin(), 0.], [0., x.sin(), x.cos(), 0.], [0., 0., 0., 1.]];
    let ry = [[y.cos(), 0., y.sin(), 0.], [0., 1., 0., 0.], [-y.sin(), 0., y.cos(), 0.], [0., 0., 0., 1.]];
    let rz = [[z.cos(), -z.sin(), 0., 0.], [z.sin(), z.cos(), 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]];
    match order {
        1 => mul(&ry, &mul(&rz, &rx)), // XZY
        2 => mul(&rx, &mul(&rz, &ry)), // YZX
        3 => mul(&rz, &mul(&rx, &ry)), // YXZ
        4 => mul(&ry, &mul(&rx, &rz)), // ZXY
        5 => mul(&rx, &mul(&ry, &rz)), // ZYX
        _ => mul(&rz, &mul(&ry, &rx)), // XYZ
    }
}

fn apply(m: &M4, p: [f64; 3]) -> [f64; 3] {
    [0, 1, 2].map(|i| m[i][0] * p[0] + m[i][1] * p[1] + m[i][2] * p[2] + m[i][3])
}

fn apply_dir(m: &M4, d: [f64; 3]) -> [f64; 3] {
    let v = [0, 1, 2].map(|i| m[i][0] * d[0] + m[i][1] * d[1] + m[i][2] * d[2]);
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 0.0 { v.map(|c| c / len) } else { [0.0, 1.0, 0.0] }
}

fn det3(m: &M4) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

fn floats(v: &A) -> Option<Vec<f64>> {
    match v {
        A::ArrF64(a) => Some(a.clone()),
        A::ArrF32(a) => Some(a.iter().map(|&f| f64::from(f)).collect()),
        _ => None,
    }
}

fn ints(v: &A) -> Option<Vec<i64>> {
    match v {
        A::ArrI32(a) => Some(a.iter().map(|&i| i64::from(i)).collect()),
        A::ArrI64(a) => Some(a.clone()),
        _ => None,
    }
}

fn num(v: &A) -> Option<f64> {
    match v {
        A::F64(f) => Some(*f),
        A::F32(f) => Some(f64::from(*f)),
        A::I32(i) => Some(f64::from(*i)),
        A::I64(i) => Some(*i as f64),
        A::I16(i) => Some(f64::from(*i)),
        A::Bool(b) => Some(f64::from(u8::from(*b))),
        _ => None,
    }
}

fn string(v: &A) -> Option<&str> {
    if let A::String(s) = v { Some(s.as_str()) } else { None }
}

fn child_attr<'a>(node: NodeHandle<'a>, name: &str) -> Option<&'a A> {
    node.first_child_by_name(name)?.attributes().first()
}

/// `Properties70` → name → values (after the name, type, label and flags strings).
fn props(node: NodeHandle<'_>) -> HashMap<String, Vec<A>> {
    let mut out = HashMap::new();
    if let Some(p70) = node.first_child_by_name("Properties70") {
        for p in p70.children_by_name("P") {
            let a = p.attributes();
            if let Some(name) = a.first().and_then(string) {
                out.insert(name.to_owned(), a.get(4..).unwrap_or_default().to_vec());
            }
        }
    }
    out
}

fn vec3(p: &HashMap<String, Vec<A>>, name: &str, default: [f64; 3]) -> [f64; 3] {
    match p.get(name) {
        Some(v) if v.len() >= 3 => [num(&v[0]), num(&v[1]), num(&v[2])].map(|x| x.unwrap_or(0.0)),
        _ => default,
    }
}

fn scalar(p: &HashMap<String, Vec<A>>, name: &str) -> Option<f64> {
    p.get(name).and_then(|v| v.first()).and_then(num)
}

/// A layer element's values and how they map onto polygon vertices.
struct Layer {
    values: Vec<f64>,
    index: Option<Vec<i64>>,
    mapping: String,
    width: usize,
}

impl Layer {
    fn read(geom: NodeHandle<'_>, element: &str, values: &str, index: &str, width: usize) -> Option<Self> {
        let el = geom.children_by_name(element).find(|e| e.attributes().first().and_then(num).unwrap_or(0.0) == 0.0)?;
        Some(Self {
            values: floats(child_attr(el, values)?)?,
            index: child_attr(el, index).and_then(ints),
            mapping: child_attr(el, "MappingInformationType").and_then(string).unwrap_or("ByPolygonVertex").to_owned(),
            width,
        })
    }

    /// The value for polygon-vertex `pv` (control point `cp`, polygon `poly`).
    fn get(&self, pv: usize, cp: usize, poly: usize) -> Option<(usize, &[f64])> {
        let i = match self.mapping.as_str() {
            "ByVertice" | "ByVertex" | "ByControlPoint" => cp,
            "ByPolygon" => poly,
            "AllSame" => 0,
            _ => pv,
        };
        let i = match &self.index {
            Some(ix) => usize::try_from(*ix.get(i)?).ok()?,
            None => i,
        };
        Some((i, self.values.get(i * self.width..i * self.width + self.width)?))
    }
}

/// Object id → (node, class).
struct Objects<'a> {
    nodes: HashMap<i64, (NodeHandle<'a>, String)>,
    /// child → parents (object-object connections), in file order.
    parents: HashMap<i64, Vec<i64>>,
    /// parent → children with the property they plug into ("" for object-object).
    children: HashMap<i64, Vec<(i64, String)>>,
}

pub(crate) fn load(path: &Path, root: &Path) -> Result<Mesh> {
    let file = std::io::BufReader::new(std::fs::File::open(path)?);
    let head = std::fs::read(path).map(|d| d.get(..20).map(<[u8]>::to_vec).unwrap_or_default()).unwrap_or_default();
    ensure!(
        head.starts_with(b"Kaydara FBX Binary"),
        "only binary FBX files can be read (this one is ASCII or not FBX)"
    );
    let fbxcel::tree::any::AnyTree::V7400(_, tree, _) = fbxcel::tree::any::AnyTree::from_seekable_reader(file)
        .map_err(|e| anyhow::anyhow!("not a readable FBX file: {e}"))?
    else {
        bail!("unsupported FBX version")
    };
    let rootnode = tree.root();
    let objects_node = rootnode.first_child_by_name("Objects").context("FBX has no Objects")?;
    let mut objs = Objects { nodes: HashMap::new(), parents: HashMap::new(), children: HashMap::new() };
    for o in objects_node.children() {
        if let Some(id) = o.attributes().first().and_then(num) {
            objs.nodes.insert(id as i64, (o, o.name().to_owned()));
        }
    }
    if let Some(conns) = rootnode.first_child_by_name("Connections") {
        for c in conns.children_by_name("C") {
            let a = c.attributes();
            let (Some(kind), Some(child), Some(parent)) =
                (a.first().and_then(string), a.get(1).and_then(num), a.get(2).and_then(num))
            else {
                continue;
            };
            let (child, parent) = (child as i64, parent as i64);
            let prop = if kind == "OP" { a.get(3).and_then(string).unwrap_or("").to_owned() } else { String::new() };
            if kind == "OO" {
                objs.parents.entry(child).or_default().push(parent);
            }
            objs.children.entry(parent).or_default().push((child, prop));
        }
    }

    // The file's axis system and unit, mapped to glTF (X right, Y up, Z toward the viewer, meters).
    let settings = rootnode.first_child_by_name("GlobalSettings").map(props).unwrap_or_default();
    let axis = |name: &str, sign: &str, default: i32| -> (usize, f64) {
        let a = scalar(&settings, name).map_or(default, |v| v as i32);
        let s = scalar(&settings, sign).unwrap_or(1.0);
        (usize::try_from(a.clamp(0, 2)).unwrap_or(1), if s < 0.0 { -1.0 } else { 1.0 })
    };
    let (up, up_s) = axis("UpAxis", "UpAxisSign", 1);
    let (front, front_s) = axis("FrontAxis", "FrontAxisSign", 2);
    let (coord, coord_s) = axis("CoordAxis", "CoordAxisSign", 0);
    let unit = scalar(&settings, "UnitScaleFactor").unwrap_or(1.0) / 100.0;
    let mut basis = [[0.0; 4]; 4];
    basis[0][coord] = coord_s * unit;
    basis[1][up] = up_s * unit;
    basis[2][front] = front_s * unit;
    basis[3][3] = 1.0;
    // Degenerate axis settings (two axes the same): fall back to Y-up.
    if det3(&basis).abs() < 1e-12 {
        basis = scale([unit; 3]);
    }

    let mut world_cache: HashMap<i64, M4> = HashMap::new();
    let dir = path.parent().unwrap_or(root);
    let mut mesh = Mesh::default();
    let mut material_index: HashMap<i64, usize> = HashMap::new();
    let mut image_index: HashMap<i64, Option<usize>> = HashMap::new();

    let geometries: Vec<i64> = objs
        .nodes
        .iter()
        .filter(|(_, (n, _))| n.name() == "Geometry" && kind_of(*n) == "Mesh")
        .map(|(id, _)| *id)
        .collect();
    let mut geometries = geometries;
    geometries.sort_unstable();
    for gid in geometries {
        let (geom, _) = objs.nodes[&gid];
        // A geometry can be instanced by several models.
        let models: Vec<i64> = objs
            .parents
            .get(&gid)
            .map(|p| {
                p.iter().copied().filter(|m| objs.nodes.get(m).is_some_and(|(n, _)| n.name() == "Model")).collect()
            })
            .unwrap_or_default();
        for model in if models.is_empty() { vec![0] } else { models } {
            let world = if model == 0 { IDENTITY } else { world_of(model, &objs, &mut world_cache) };
            let geometric = if model == 0 {
                IDENTITY
            } else {
                let p = props(objs.nodes[&model].0);
                mul(
                    &translate(vec3(&p, "GeometricTranslation", [0.0; 3])),
                    &mul(
                        &rotate(vec3(&p, "GeometricRotation", [0.0; 3]), 0),
                        &scale(vec3(&p, "GeometricScaling", [1.0; 3])),
                    ),
                )
            };
            let m = mul(&basis, &mul(&world, &geometric));
            // The model's materials, in connection order: the geometry's material layer indexes into this list.
            let mats: Vec<i64> = objs
                .children
                .get(&model)
                .map(|c| {
                    c.iter()
                        .filter(|(id, p)| {
                            p.is_empty() && objs.nodes.get(id).is_some_and(|(n, _)| n.name() == "Material")
                        })
                        .map(|(id, _)| *id)
                        .collect()
                })
                .unwrap_or_default();
            let mut local_materials = Vec::new();
            for mid in &mats {
                let idx = if let Some(&i) = material_index.get(mid) {
                    i
                } else {
                    let mat = read_material(*mid, &objs, dir, root, &mut mesh.images, &mut image_index);
                    mesh.materials.push(mat);
                    material_index.insert(*mid, mesh.materials.len() - 1);
                    mesh.materials.len() - 1
                };
                local_materials.push(idx);
            }
            add_geometry(geom, &m, &local_materials, &mut mesh)?;
            ensure!(mesh.triangles() <= MAX_TRIANGLES, "too many triangles to convert");
        }
    }
    ensure!(!mesh.prims.is_empty(), "FBX has no mesh geometry");
    if mesh.materials.is_empty() || mesh.prims.iter().any(|p| p.material >= mesh.materials.len()) {
        let gray = mesh.materials.len();
        mesh.materials.push(Material::plain("default", GRAY));
        for p in &mut mesh.prims {
            if p.material > gray {
                p.material = gray;
            }
        }
    }
    Ok(mesh)
}

/// "Mesh" in `Geometry: 123, "Geometry::Name", "Mesh"`.
fn kind_of(node: NodeHandle<'_>) -> &str {
    node.attributes().get(2).and_then(string).unwrap_or("")
}

/// A model's world transform: its parents' times its own local transform.
fn world_of(id: i64, objs: &Objects<'_>, cache: &mut HashMap<i64, M4>) -> M4 {
    if let Some(m) = cache.get(&id) {
        return *m;
    }
    let Some((node, _)) = objs.nodes.get(&id) else { return IDENTITY };
    let p = props(*node);
    let order = scalar(&p, "RotationOrder").map_or(0, |v| v as i32);
    let r_pre = rotate(vec3(&p, "PreRotation", [0.0; 3]), 0);
    let r_post_inv = {
        // The inverse of a rotation is its transpose.
        let r = rotate(vec3(&p, "PostRotation", [0.0; 3]), 0);
        let mut t = IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                t[i][j] = r[j][i];
            }
        }
        t
    };
    let local = mul(
        &translate(vec3(&p, "Lcl Translation", [0.0; 3])),
        &mul(
            &r_pre,
            &mul(
                &rotate(vec3(&p, "Lcl Rotation", [0.0; 3]), order),
                &mul(&r_post_inv, &scale(vec3(&p, "Lcl Scaling", [1.0; 3]))),
            ),
        ),
    );
    let parent = objs
        .parents
        .get(&id)
        .and_then(|ps| ps.iter().find(|pid| objs.nodes.get(pid).is_some_and(|(n, _)| n.name() == "Model")))
        .copied();
    // Guard against cycles in broken files.
    cache.insert(id, local);
    let world = match parent {
        Some(pid) if pid != id => mul(&world_of(pid, objs, cache), &local),
        _ => local,
    };
    cache.insert(id, world);
    world
}

fn read_material(
    id: i64,
    objs: &Objects<'_>,
    dir: &Path,
    root: &Path,
    images: &mut Vec<Image>,
    image_index: &mut HashMap<i64, Option<usize>>,
) -> Material {
    let (node, _) = objs.nodes[&id];
    let p = props(node);
    let name = node
        .attributes()
        .get(1)
        .and_then(string)
        .unwrap_or("material")
        .split('\0')
        .next()
        .unwrap_or("material")
        .to_owned();
    let d = vec3(&p, "DiffuseColor", vec3(&p, "Diffuse", [0.8, 0.8, 0.8]));
    let alpha =
        scalar(&p, "Opacity").unwrap_or_else(|| 1.0 - scalar(&p, "TransparencyFactor").unwrap_or(0.0).clamp(0.0, 1.0));
    let mut mat = Material::plain(&name, [d[0] as f32, d[1] as f32, d[2] as f32, alpha.clamp(0.0, 1.0) as f32]);
    for (tex, prop) in objs.children.get(&id).cloned().unwrap_or_default() {
        let slot = prop.to_lowercase();
        let is_color = slot.contains("diffusecolor") || slot.contains("basecolor") || slot == "diffuse";
        let is_normal = slot.contains("normalmap") || slot == "bump" || slot.contains("normal");
        if !is_color && !is_normal {
            continue;
        }
        let img = *image_index.entry(tex).or_insert_with(|| {
            let img = texture_image(tex, objs, dir, root)?;
            images.push(img);
            Some(images.len() - 1)
        });
        if is_color && mat.color_image.is_none() {
            mat.color_image = img;
            // A textured material's diffuse factor is usually a leftover; white shows the texture as painted.
            if img.is_some() {
                mat.color[..3].copy_from_slice(&[1.0, 1.0, 1.0]);
            }
        } else if is_normal && mat.normal_image.is_none() {
            mat.normal_image = img;
        }
    }
    mat
}

/// A texture's pixels: embedded in a connected `Video`'s `Content`, or the file it names, found in the pack.
fn texture_image(id: i64, objs: &Objects<'_>, dir: &Path, root: &Path) -> Option<Image> {
    let (node, _) = objs.nodes.get(&id)?;
    let names: Vec<String> = ["RelativeFilename", "FileName"]
        .iter()
        .filter_map(|k| child_attr(*node, k).and_then(string).map(str::to_owned))
        .filter(|s| !s.is_empty())
        .collect();
    for (vid, _) in objs.children.get(&id).cloned().unwrap_or_default() {
        if let Some((video, _)) = objs.nodes.get(&vid)
            && video.name() == "Video"
            && let Some(A::Binary(bytes)) = child_attr(*video, "Content")
            && !bytes.is_empty()
        {
            let hint = names
                .first()
                .map_or_else(|| Path::new("t.png").to_path_buf(), |n| Path::new(&n.replace('\\', "/")).to_path_buf());
            if let Some(img) = image_from_bytes(bytes, &hint) {
                return Some(img);
            }
        }
    }
    names.iter().find_map(|n| find_texture(n, dir, root)).and_then(|p| load_image(&p))
}

/// Vertices already emitted for a primitive, by (control point, normal index, uv index).
type Seen = HashMap<(usize, usize, usize), u32>;

/// Append one geometry's triangles, transformed by `m`, split by material.
fn add_geometry(geom: NodeHandle<'_>, m: &M4, materials: &[usize], mesh: &mut Mesh) -> Result<()> {
    let verts = floats(child_attr(geom, "Vertices").context("mesh has no vertices")?).context("bad vertices")?;
    let poly_index =
        ints(child_attr(geom, "PolygonVertexIndex").context("mesh has no polygons")?).context("bad polygons")?;
    let normals = Layer::read(geom, "LayerElementNormal", "Normals", "NormalsIndex", 3);
    let uvs = Layer::read(geom, "LayerElementUV", "UV", "UVIndex", 2);
    let mat_layer = geom.children_by_name("LayerElementMaterial").next().and_then(|el| {
        Some(Layer {
            values: ints(child_attr(el, "Materials")?)?.into_iter().map(|i| i as f64).collect(),
            index: None,
            mapping: child_attr(el, "MappingInformationType").and_then(string).unwrap_or("AllSame").to_owned(),
            width: 1,
        })
    });
    let flip = det3(m) < 0.0;
    // Per material slot: a primitive with de-duplicated vertices keyed by (control point, normal, uv).
    let mut prims: HashMap<usize, (Prim, Seen)> = HashMap::new();
    let mut poly_start = 0;
    let mut poly = 0;
    for (k, &raw) in poly_index.iter().enumerate() {
        if raw >= 0 {
            continue;
        }
        // A negative index ends a polygon (stored as !index).
        let corners: Vec<(usize, usize)> = (poly_start..=k)
            .map(|pv| {
                let v = poly_index[pv];
                (pv, usize::try_from(if v < 0 { !v } else { v }).unwrap_or(usize::MAX))
            })
            .collect();
        let slot = mat_layer.as_ref().and_then(|l| l.get(k, 0, poly)).map_or(0, |(_, v)| v[0] as usize);
        let material = materials.get(slot).copied().unwrap_or(usize::MAX);
        let (prim, seen) =
            prims.entry(material).or_insert_with(|| (Prim { material, ..Prim::default() }, HashMap::new()));
        let mut ids = Vec::with_capacity(corners.len());
        for &(pv, cp) in &corners {
            let Some(pos) = verts.get(cp * 3..cp * 3 + 3) else { bail!("polygon points past the vertex list") };
            let n = normals.as_ref().and_then(|l| l.get(pv, cp, poly));
            let t = uvs.as_ref().and_then(|l| l.get(pv, cp, poly));
            let key = (cp, n.map_or(usize::MAX, |x| x.0), t.map_or(usize::MAX, |x| x.0));
            let id = *seen.entry(key).or_insert_with(|| {
                let p = apply(m, [pos[0], pos[1], pos[2]]);
                prim.positions.push(p.map(|c| c as f32));
                if let Some((_, nv)) = n {
                    prim.normals.push(apply_dir(m, [nv[0], nv[1], nv[2]]).map(|c| c as f32));
                }
                if let Some((_, tv)) = t {
                    // FBX's texture origin is bottom-left, glTF's top-left.
                    prim.uvs.push([tv[0] as f32, 1.0 - tv[1] as f32]);
                }
                u32::try_from(prim.positions.len() - 1).unwrap_or(u32::MAX)
            });
            ids.push(id);
        }
        for i in 1..ids.len().saturating_sub(1) {
            if flip {
                prim.indices.extend([ids[0], ids[i + 1], ids[i]]);
            } else {
                prim.indices.extend([ids[0], ids[i], ids[i + 1]]);
            }
        }
        poly_start = k + 1;
        poly += 1;
    }
    let mut keys: Vec<usize> = prims.keys().copied().collect();
    keys.sort_unstable();
    for key in keys {
        let (mut prim, _) = prims.remove(&key).unwrap_or_default();
        if prim.indices.is_empty() {
            continue;
        }
        if prim.normals.len() != prim.positions.len() {
            prim.normals = smooth_normals(&prim.positions, &prim.indices);
        }
        if prim.uvs.len() != prim.positions.len() {
            prim.uvs.clear();
        }
        mesh.prims.push(prim);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fbxcel::writer::v7400::binary::{FbxFooter, Writer};

    fn png(rgb: [u8; 3]) -> Vec<u8> {
        let mut b = Vec::new();
        image::RgbImage::from_pixel(2, 2, image::Rgb(rgb))
            .write_to(&mut std::io::Cursor::new(&mut b), image::ImageFormat::Png)
            .unwrap();
        b
    }

    /// A Z-up, centimeter FBX (3ds Max style): one textured quad on the ground, its model raised 100 cm.
    pub(crate) fn z_up_fbx() -> Vec<u8> {
        let mut w = Writer::new(std::io::Cursor::new(Vec::new()), fbxcel::low::FbxVersion::V7_4).unwrap();
        let p = |w: &mut Writer<std::io::Cursor<Vec<u8>>>, name: &str, vals: &[A]| {
            let mut a = w.new_node("P").unwrap();
            for s in [name, name, "", "A"] {
                a.append_string_direct(s).unwrap();
            }
            for v in vals {
                match v {
                    A::F64(f) => a.append_f64(*f).unwrap(),
                    A::I32(i) => a.append_i32(*i).unwrap(),
                    _ => unreachable!(),
                }
            }
            w.close_node().unwrap();
        };
        let object = |w: &mut Writer<std::io::Cursor<Vec<u8>>>, kind: &str, id: i64, name: &str, sub: &str| {
            let mut a = w.new_node(kind).unwrap();
            a.append_i64(id).unwrap();
            a.append_string_direct(&format!("{name}\0\x01{kind}")).unwrap();
            a.append_string_direct(sub).unwrap();
        };
        // Global settings: Z up, -Y front, X right, centimeters.
        w.new_node("GlobalSettings").unwrap();
        w.new_node("Properties70").unwrap();
        for (k, v) in [
            ("UpAxis", 2),
            ("UpAxisSign", 1),
            ("FrontAxis", 1),
            ("FrontAxisSign", -1),
            ("CoordAxis", 0),
            ("CoordAxisSign", 1),
        ] {
            p(&mut w, k, &[A::I32(v)]);
        }
        p(&mut w, "UnitScaleFactor", &[A::F64(1.0)]);
        w.close_node().unwrap();
        w.close_node().unwrap();

        w.new_node("Objects").unwrap();
        object(&mut w, "Geometry", 10, "Quad", "Mesh");
        {
            let mut a = w.new_node("Vertices").unwrap();
            a.append_arr_f64_from_iter(None, [0., 0., 0., 100., 0., 0., 100., 100., 0., 0., 100., 0.]).unwrap();
        }
        w.close_node().unwrap();
        {
            let mut a = w.new_node("PolygonVertexIndex").unwrap();
            a.append_arr_i32_from_iter(None, [0, 1, 2, !3]).unwrap();
        }
        w.close_node().unwrap();
        {
            let mut a = w.new_node("LayerElementUV").unwrap();
            a.append_i32(0).unwrap();
        }
        for (name, s) in [("MappingInformationType", "ByPolygonVertex"), ("ReferenceInformationType", "IndexToDirect")]
        {
            w.new_node(name).unwrap().append_string_direct(s).unwrap();
            w.close_node().unwrap();
        }
        w.new_node("UV").unwrap().append_arr_f64_from_iter(None, [0., 0., 1., 0., 1., 1., 0., 1.]).unwrap();
        w.close_node().unwrap();
        w.new_node("UVIndex").unwrap().append_arr_i32_from_iter(None, [0, 1, 2, 3]).unwrap();
        w.close_node().unwrap();
        w.close_node().unwrap(); // LayerElementUV
        w.close_node().unwrap(); // Geometry

        object(&mut w, "Model", 20, "Floor", "Mesh");
        w.new_node("Properties70").unwrap();
        p(&mut w, "Lcl Translation", &[A::F64(0.0), A::F64(0.0), A::F64(100.0)]);
        w.close_node().unwrap();
        w.close_node().unwrap();

        object(&mut w, "Material", 30, "Stone", "");
        w.new_node("Properties70").unwrap();
        p(&mut w, "DiffuseColor", &[A::F64(0.5), A::F64(0.4), A::F64(0.3)]);
        w.close_node().unwrap();
        w.close_node().unwrap();

        object(&mut w, "Texture", 40, "StoneTex", "");
        w.new_node("RelativeFilename").unwrap().append_string_direct("C:\\art\\stone.png").unwrap();
        w.close_node().unwrap();
        w.close_node().unwrap();

        object(&mut w, "Video", 50, "StoneVideo", "Clip");
        w.new_node("Content").unwrap().append_binary_direct(&png([10, 200, 30])).unwrap();
        w.close_node().unwrap();
        w.close_node().unwrap();
        w.close_node().unwrap(); // Objects

        w.new_node("Connections").unwrap();
        for (kind, child, parent, prop) in [
            ("OO", 10, 20, ""),
            ("OO", 20, 0, ""),
            ("OO", 30, 20, ""),
            ("OP", 40, 30, "DiffuseColor"),
            ("OO", 50, 40, ""),
        ] {
            let mut a = w.new_node("C").unwrap();
            a.append_string_direct(kind).unwrap();
            a.append_i64(child).unwrap();
            a.append_i64(parent).unwrap();
            if !prop.is_empty() {
                a.append_string_direct(prop).unwrap();
            }
            w.close_node().unwrap();
        }
        w.close_node().unwrap();
        w.finalize_and_flush(&FbxFooter::default()).unwrap().into_inner()
    }

    #[test]
    fn z_up_centimeters_become_y_up_meters_with_embedded_texture() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("floor.fbx");
        std::fs::write(&p, z_up_fbx()).unwrap();
        let mesh = load(&p, dir.path()).unwrap();
        assert_eq!(mesh.triangles(), 2);
        let prim = &mesh.prims[0];
        for v in &prim.positions {
            assert!((v[1] - 1.0).abs() < 1e-5, "raised 100 cm = 1 m up: {v:?}");
        }
        let xs: Vec<f32> = prim.positions.iter().map(|v| v[0]).collect();
        let zs: Vec<f32> = prim.positions.iter().map(|v| v[2]).collect();
        assert!(
            xs.iter().any(|&x| (x - 1.0).abs() < 1e-5) && zs.iter().any(|&z| (z + 1.0).abs() < 1e-5),
            "{:?}",
            prim.positions
        );
        // Ground-facing quad in Z-up faces +Y once converted.
        assert!(prim.normals.iter().all(|n| n[1] > 0.99), "{:?}", prim.normals);
        assert_eq!(prim.uvs.len(), prim.positions.len());
        assert!(prim.uvs.iter().any(|t| (t[1] - 1.0).abs() < 1e-6), "V flipped to glTF's origin");
        let mat = &mesh.materials[prim.material];
        assert_eq!(mat.name, "Stone");
        assert_eq!(mat.color_image, Some(0));
        assert_eq!(mesh.images.len(), 1);
        let glb = crate::meshes::to_glb(&mesh).unwrap();
        let doc: serde_json::Value = {
            let len = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
            serde_json::from_slice(&glb[20..20 + len]).unwrap()
        };
        assert_eq!(doc["materials"][0]["pbrMetallicRoughness"]["baseColorTexture"]["index"], 0);
        assert!(doc["meshes"][0]["primitives"][0]["attributes"]["TEXCOORD_0"].is_number());
    }

    #[test]
    fn ascii_fbx_is_refused_clearly() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("a.fbx");
        std::fs::write(&p, "; FBX 7.4.0 project file\nFBXHeaderExtension: {\n}\n").unwrap();
        let err = load(&p, dir.path()).err().unwrap();
        assert!(format!("{err}").contains("binary"), "{err}");
    }
}

#[cfg(test)]
#[test]
#[ignore = "writes files; run by hand"]
fn write_fbx_fixture() {
    let Some(out) = std::env::var_os("HOARD_FIXTURE_OUT") else { return };
    let out = std::path::PathBuf::from(out);
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("floor.fbx"), tests::z_up_fbx()).unwrap();
}
