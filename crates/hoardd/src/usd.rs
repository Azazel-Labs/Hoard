//! USD (`.usd`, `.usda`, `.usdc`, `.usdz`), read with the pure-Rust `openusd` crate. The composed stage's visible
//! meshes become one `meshes::Mesh`: references, payloads, variants and instances resolved, each mesh placed by its
//! world transform, the stage's up axis and units turned into glTF's (Y up, meters). Materials come from
//! `UsdPreviewSurface`: the diffuse colour or texture, the normal map and the opacity, with textures read from the
//! `.usdz` itself or found in the pack. A mesh with no material shows its display colour.

#![expect(clippy::many_single_char_names, reason = "geometry code: x/y/z, u/v")]

use std::collections::HashMap;
use std::io::Read as _;
use std::path::Path;

use anyhow::{Context, Result, ensure};
use openusd::usd::SchemaBase as _;
use openusd::{gf, sdf, usd};
use openusd_schemas::geom::{self, GprimSchema, ImageableExt, MeshSchema, PointBasedSchema, PrimvarsAPI, SubsetSchema};
use openusd_schemas::shade::{self, MaterialBindingAPI};

use crate::meshes::{GRAY, Image, MAX_TRIANGLES, Material, Mesh, Prim};

/// The file's meshes, or `None` when it has none to show (a material library, a layer of overrides).
pub(crate) fn load(path: &Path, root: &Path) -> Result<Option<Mesh>> {
    let stage = usd::Stage::builder()
        .schema_registry(openusd_schemas::schema_registry())
        .open(path.to_str().context("the path isn't UTF-8")?)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let z_up = matches!(stage.stage_metadata("upAxis"), Ok(Some(v)) if v.as_str() == Some("Z"));
    let meters = match stage.stage_metadata("metersPerUnit") {
        Ok(Some(sdf::Value::Double(m))) if m > 0.0 => m,
        Ok(Some(sdf::Value::Float(m))) if m > 0.0 => f64::from(m),
        _ => 0.01,
    };
    let mut paths = Vec::new();
    stage
        .traverse(usd::PrimPredicate::DEFAULT.with_instance_proxies(true), |p| paths.push(p.clone()))
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut out = Builder {
        stage: &stage,
        file: path,
        root,
        usdz: Usdz::open(path),
        mesh: Mesh::default(),
        materials: HashMap::new(),
        images: HashMap::new(),
        prims: HashMap::new(),
    };
    let mut xforms = geom::XformCache::new(None);
    for p in paths {
        let Ok(Some(mesh)) = geom::Mesh::get(&stage, p.clone()) else { continue };
        if !shown(&mesh) {
            continue;
        }
        let world = xforms.local_to_world_transform(mesh.prim()).unwrap_or_default();
        out.mesh(&mesh, world)?;
    }
    let Builder { mut mesh, .. } = out;
    let scale = meters as f32;
    for prim in &mut mesh.prims {
        for v in &mut prim.positions {
            *v = up(*v, z_up).map(|c| c * scale);
        }
        for n in &mut prim.normals {
            *n = up(*n, z_up);
        }
    }
    Ok((!mesh.prims.is_empty()).then_some(mesh))
}

/// Z-up to glTF's Y-up: (x, y, z) becomes (x, z, -y).
fn up([x, y, z]: [f32; 3], z_up: bool) -> [f32; 3] {
    if z_up { [x, z, -y] } else { [x, y, z] }
}

/// Visible, and drawn for rendering: guides and proxies stand in for the real thing.
fn shown(mesh: &geom::Mesh) -> bool {
    let visible = !matches!(mesh.compute_visibility(None), Ok(geom::Visibility::Invisible));
    let purpose = mesh.compute_purpose().unwrap_or_default();
    visible && !matches!(purpose, geom::Purpose::Guide | geom::Purpose::Proxy)
}

struct Builder<'a> {
    stage: &'a usd::Stage,
    file: &'a Path,
    root: &'a Path,
    usdz: Option<Usdz>,
    mesh: Mesh,
    /// Material prim path (or `display:<rgb>` for a display colour) to its index in `mesh.materials`.
    materials: HashMap<String, usize>,
    /// Texture asset path to its index in `mesh.images`, or `None` when it couldn't be found.
    images: HashMap<String, Option<usize>>,
    /// Material index to the prim in `mesh.prims` collecting its triangles.
    prims: HashMap<usize, usize>,
}

/// How a primvar's values spread over the mesh.
#[derive(Clone, Copy, PartialEq)]
enum Spread {
    Constant,
    Uniform,
    Vertex,
    FaceVarying,
}

impl Spread {
    fn of(token: Option<&str>) -> Self {
        match token {
            Some("constant") => Self::Constant,
            Some("uniform") => Self::Uniform,
            Some("faceVarying") => Self::FaceVarying,
            _ => Self::Vertex,
        }
    }

    /// The value index for one face corner: `face` is the face, `point` the point it uses, `corner` its place
    /// among all the face corners.
    fn index(self, face: usize, point: usize, corner: usize) -> usize {
        match self {
            Self::Constant => 0,
            Self::Uniform => face,
            Self::Vertex => point,
            Self::FaceVarying => corner,
        }
    }
}

/// A primvar's values with its indices and interpolation.
struct Primvar<T> {
    values: Vec<T>,
    indices: Option<Vec<i32>>,
    spread: Spread,
}

impl<T: Copy> Primvar<T> {
    fn at(&self, face: usize, point: usize, corner: usize) -> Option<T> {
        let i = self.spread.index(face, point, corner);
        let i = match &self.indices {
            Some(ix) => usize::try_from(*ix.get(i)?).ok()?,
            None => i,
        };
        self.values.get(i).copied()
    }
}

fn primvar<T>(api: &PrimvarsAPI, name: &str, decode: impl Fn(sdf::Value) -> Option<Vec<T>>) -> Option<Primvar<T>> {
    let pv = api.primvar(name).ok()?;
    let values = decode(pv.attribute().get::<sdf::Value>().ok()??)?;
    let indices = pv.indices(None).ok().flatten().filter(|i| !i.is_empty());
    let spread = pv.interpolation().ok().map_or(Spread::Vertex, |i| Spread::of(Some(i.as_token())));
    Some(Primvar { values, indices, spread })
}

fn vec3s(v: sdf::Value) -> Option<Vec<[f32; 3]>> {
    Some(match v {
        sdf::Value::Vec3fVec(v) => v.into_iter().map(|p| [p.x, p.y, p.z]).collect(),
        sdf::Value::Vec3dVec(v) => v.into_iter().map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect(),
        sdf::Value::Vec3hVec(v) => v.into_iter().map(|p| [p.x.to_f32(), p.y.to_f32(), p.z.to_f32()]).collect(),
        sdf::Value::Vec3f(p) => vec![[p.x, p.y, p.z]],
        sdf::Value::Vec3d(p) => vec![[p.x as f32, p.y as f32, p.z as f32]],
        _ => return None,
    })
}

fn vec2s(v: sdf::Value) -> Option<Vec<[f32; 2]>> {
    Some(match v {
        sdf::Value::Vec2fVec(v) => v.into_iter().map(|p| [p.x, p.y]).collect(),
        sdf::Value::Vec2dVec(v) => v.into_iter().map(|p| [p.x as f32, p.y as f32]).collect(),
        sdf::Value::Vec2hVec(v) => v.into_iter().map(|p| [p.x.to_f32(), p.y.to_f32()]).collect(),
        _ => return None,
    })
}

fn ints(v: Option<sdf::Value>) -> Vec<i32> {
    match v {
        Some(sdf::Value::IntVec(v)) => v,
        _ => Vec::new(),
    }
}

/// Names texture coordinates go by: USD's own `st`, then what exporters write.
const UV_NAMES: &[&str] = &["st", "st0", "st_0", "UVMap", "uv", "uv0", "map1", "UVChannel_1"];

impl Builder<'_> {
    fn mesh(&mut self, mesh: &geom::Mesh, world: gf::Matrix4d) -> Result<()> {
        let Some(points) = mesh.points_attr().get::<sdf::Value>().ok().flatten().and_then(vec3s) else {
            return Ok(());
        };
        let counts = ints(mesh.face_vertex_counts_attr().get::<sdf::Value>().ok().flatten());
        let corners = ints(mesh.face_vertex_indices_attr().get::<sdf::Value>().ok().flatten());
        if points.is_empty() || counts.is_empty() {
            return Ok(());
        }
        let points: Vec<[f32; 3]> = points
            .into_iter()
            .map(|p| {
                let v = world.transform_point(gf::vec3f(p[0], p[1], p[2]));
                [v.x, v.y, v.z]
            })
            .collect();
        let normal_matrix = normal_matrix(&world);
        let left_handed =
            matches!(mesh.orientation_attr().get::<sdf::Value>(), Ok(Some(v)) if v.as_str() == Some("leftHanded"));
        let flip = left_handed != (det3(&world) < 0.0);

        let api = PrimvarsAPI::from_prim_unchecked(mesh.prim().clone());
        let uvs = UV_NAMES.iter().find_map(|n| primvar(&api, n, vec2s));
        let normals = primvar(&api, "normals", vec3s).or_else(|| {
            let attr = mesh.normals_attr();
            let values = vec3s(attr.get::<sdf::Value>().ok()??)?;
            let spread = attr.get_metadata::<sdf::Value>("interpolation").ok().flatten();
            Some(Primvar { values, indices: None, spread: Spread::of(spread.as_ref().and_then(sdf::Value::as_str)) })
        });
        // No normals: a subdivision surface is smooth, a polygon mesh faceted.
        let smooth = normals.is_none()
            && !matches!(mesh.subdivision_scheme_attr().get::<sdf::Value>(), Ok(Some(v)) if v.as_str() == Some("none"));
        let point_normals = if smooth {
            let mut tris = Vec::new();
            let mut c = 0;
            for &n in &counts {
                let n = usize::try_from(n).unwrap_or(0);
                for i in 1..n.saturating_sub(1) {
                    for k in [c, c + i, c + i + 1] {
                        tris.push(corners.get(k).and_then(|&p| u32::try_from(p).ok()).unwrap_or(0));
                    }
                }
                c += n;
            }
            crate::meshes::smooth_normals(&points, &tris)
        } else {
            Vec::new()
        };
        let display = primvar(&api, "displayColor", vec3s);

        // Each face's material: a material-binding subset's, else the mesh's own.
        let own = self.material_of(mesh.prim(), display.as_ref().and_then(|d| d.values.first().copied()));
        let mut face_material = vec![own; counts.len()];
        if let Ok(children) = mesh.prim().children() {
            for child in children {
                let Ok(Some(subset)) = geom::Subset::get(self.stage, child.path().clone()) else { continue };
                let family = subset.family_name_attr().get::<sdf::Value>().ok().flatten();
                if family.as_ref().and_then(sdf::Value::as_str).is_some_and(|f| f != "materialBind") {
                    continue;
                }
                let material = self.material_of(&child, None);
                for i in ints(subset.indices_attr().get::<sdf::Value>().ok().flatten()) {
                    if let Some(slot) = usize::try_from(i).ok().and_then(|i| face_material.get_mut(i)) {
                        *slot = material;
                    }
                }
            }
        }

        let mut corner = 0;
        for (face, &n) in counts.iter().enumerate() {
            let n = usize::try_from(n).unwrap_or(0);
            let start = corner;
            corner += n;
            if n < 3 || corner > corners.len() {
                continue;
            }
            ensure!(self.mesh.triangles() + n - 2 <= MAX_TRIANGLES, "too many triangles to preview");
            let material = face_material[face];
            let prim = self.prim_for(material);
            let first = u32::try_from(prim.positions.len()).context("too many vertices")?;
            let flat = if normals.is_none() && !smooth {
                let p = |k: usize| points.get(usize::try_from(corners[start + k]).unwrap_or(usize::MAX)).copied();
                match (p(0), p(1), p(2)) {
                    (Some(a), Some(b), Some(c)) => Some(face_normal(a, b, c, flip)),
                    _ => None,
                }
            } else {
                None
            };
            for (c, &raw) in corners.iter().enumerate().take(corner).skip(start) {
                let point = usize::try_from(raw).unwrap_or(usize::MAX);
                prim.positions.push(points.get(point).copied().unwrap_or_default());
                let normal = match (&normals, flat) {
                    (Some(nv), _) => nv.at(face, point, c).map(|v| transform_normal(&normal_matrix, v, left_handed)),
                    (None, Some(f)) => Some(f),
                    (None, None) => point_normals.get(point).copied(),
                };
                prim.normals.push(normal.unwrap_or([0.0, 1.0, 0.0]));
                if let Some(uv) = &uvs {
                    let [u, v] = uv.at(face, point, c).unwrap_or_default();
                    prim.uvs.push([u, 1.0 - v]);
                }
            }
            if uvs.is_none() && !prim.uvs.is_empty() {
                prim.uvs.resize(prim.positions.len(), [0.0, 0.0]);
            }
            for i in 1..u32::try_from(n - 1).unwrap_or(0) {
                let tri = if flip { [first, first + i + 1, first + i] } else { [first, first + i, first + i + 1] };
                prim.indices.extend(tri);
            }
        }
        Ok(())
    }

    /// The prim collecting a material's triangles. Its UVs stay empty until a mesh with UVs adds to it, then are
    /// padded so every vertex has one.
    fn prim_for(&mut self, material: usize) -> &mut Prim {
        let next = self.mesh.prims.len();
        let i = *self.prims.entry(material).or_insert(next);
        if i == next {
            self.mesh.prims.push(Prim { material, ..Prim::default() });
        }
        &mut self.mesh.prims[i]
    }

    /// The material bound to `prim`, made once per material prim; with none bound, a plain one in the mesh's
    /// display colour (or gray).
    fn material_of(&mut self, prim: &usd::Prim, display: Option<[f32; 3]>) -> usize {
        let bound = MaterialBindingAPI::from_prim_unchecked(prim.clone()).compute_bound_material("").ok().flatten();
        let key = match &bound {
            Some(p) => p.as_str().to_owned(),
            None => format!("display:{display:?}"),
        };
        if let Some(&i) = self.materials.get(&key) {
            return i;
        }
        let material = match bound {
            Some(path) => self.preview_surface(&path),
            None => Material::plain("default", display.map_or(GRAY, |[r, g, b]| [r, g, b, 1.0])),
        };
        let i = self.mesh.materials.len();
        self.mesh.materials.push(material);
        self.materials.insert(key, i);
        i
    }

    fn preview_surface(&mut self, path: &sdf::Path) -> Material {
        let name = path.as_str().rsplit('/').next().unwrap_or("material").to_owned();
        let Ok(Some(ps)) = shade::read_preview_surface(self.stage, path) else {
            return Material::plain(&name, GRAY);
        };
        // UsdPreviewSurface colours are linear, as glTF's are.
        let mut color = match ps.diffuse_color.value() {
            Some(c) => [c.x, c.y, c.z, 1.0],
            None if ps.diffuse_color.texture().is_some() => [1.0; 4],
            None => [0.18, 0.18, 0.18, 1.0],
        };
        if let Some(&o) = ps.opacity.value() {
            color[3] = o.clamp(0.0, 1.0);
        }
        let color_image = ps.diffuse_color.texture().and_then(|t| self.image(t));
        let normal_image = ps.normal.texture().and_then(|t| self.image(t));
        Material { name, color, color_image, normal_image }
    }

    /// A texture by its asset path: inside the `.usdz`, beside the file, or anywhere in the pack by name.
    fn image(&mut self, asset: &str) -> Option<usize> {
        if let Some(&i) = self.images.get(asset) {
            return i;
        }
        let reference = asset.replace("<UDIM>", "1001").replace("<udim>", "1001");
        let reference = reference.trim_start_matches("./");
        let image = self
            .usdz
            .as_mut()
            .and_then(|z| z.read(reference))
            .and_then(|bytes| crate::meshes::image_from_bytes(&bytes, Path::new(reference)))
            .or_else(|| {
                let dir = self.file.parent().unwrap_or(self.root);
                crate::meshes::find_texture(reference, dir, self.root).and_then(|p| crate::meshes::load_image(&p))
            });
        let i = image.map(|img: Image| {
            self.mesh.images.push(img);
            self.mesh.images.len() - 1
        });
        self.images.insert(asset.to_owned(), i);
        i
    }
}

/// A `.usdz` opened for its textures.
struct Usdz(zip::ZipArchive<std::fs::File>);

impl Usdz {
    fn open(path: &Path) -> Option<Self> {
        let is_usdz = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("usdz"));
        let file = std::fs::File::open(path).ok().filter(|_| is_usdz)?;
        zip::ZipArchive::new(file).ok().map(Self)
    }

    /// An entry by its path in the archive, or failing that by its file name.
    fn read(&mut self, reference: &str) -> Option<Vec<u8>> {
        let name = reference.rsplit('/').next()?.to_lowercase();
        let names: Vec<String> = self.0.file_names().filter_map(Result::ok).map(std::borrow::Cow::into_owned).collect();
        let entry = names
            .iter()
            .find(|n| n.eq_ignore_ascii_case(reference))
            .or_else(|| names.iter().find(|n| n.rsplit('/').next().is_some_and(|f| f.to_lowercase() == name)))?;
        let mut f = self.0.by_name(entry).ok()?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes).ok()?;
        Some(bytes)
    }
}

/// The determinant of the matrix's rotation and scale: negative when it mirrors.
fn det3(m: &gf::Matrix4d) -> f64 {
    let a = &m.0;
    a[0] * (a[5] * a[10] - a[6] * a[9]) - a[1] * (a[4] * a[10] - a[6] * a[8]) + a[2] * (a[4] * a[9] - a[5] * a[8])
}

/// The cofactors of the matrix's rotation and scale, which carry normals as the inverse transpose does (up to a
/// scale, removed by normalizing).
fn normal_matrix(m: &gf::Matrix4d) -> [[f64; 3]; 3] {
    let a = |r: usize, c: usize| m.0[r * 4 + c];
    let sign = if det3(m) < 0.0 { -1.0 } else { 1.0 };
    let mut out = [[0.0; 3]; 3];
    for (r, row) in out.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            let (r1, r2) = ((r + 1) % 3, (r + 2) % 3);
            let (c1, c2) = ((c + 1) % 3, (c + 2) % 3);
            *v = sign * (a(r1, c1) * a(r2, c2) - a(r1, c2) * a(r2, c1));
        }
    }
    out
}

/// A normal through the cofactor matrix (row vector times matrix, as USD composes), unit length.
fn transform_normal(m: &[[f64; 3]; 3], n: [f32; 3], left_handed: bool) -> [f32; 3] {
    let n = n.map(f64::from);
    let s = if left_handed { -1.0 } else { 1.0 };
    let v: [f64; 3] = std::array::from_fn(|c| s * (n[0] * m[0][c] + n[1] * m[1][c] + n[2] * m[2][c]));
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 0.0 { v.map(|x| (x / len) as f32) } else { [0.0, 1.0, 0.0] }
}

fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3], flip: bool) -> [f32; 3] {
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let mut n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    if flip {
        n = n.map(|x| -x);
    }
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 0.0 { n.map(|x| x / len) } else { [0.0, 1.0, 0.0] }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CUBE: &str = r#"#usda 1.0
(
    upAxis = "Z"
    metersPerUnit = 1
)

def Xform "World"
{
    double3 xformOp:translate = (10, 0, 0)
    uniform token[] xformOpOrder = ["xformOp:translate"]

    def Mesh "Quad" (prepend apiSchemas = ["MaterialBindingAPI"])
    {
        point3f[] points = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0)]
        int[] faceVertexCounts = [4]
        int[] faceVertexIndices = [0, 1, 2, 3]
        uniform token subdivisionScheme = "none"
        texCoord2f[] primvars:st = [(0, 0), (1, 0), (1, 1), (0, 1)] (interpolation = "faceVarying")
        rel material:binding = </World/Looks/Red>
    }

    def Mesh "Hidden"
    {
        token visibility = "invisible"
        point3f[] points = [(0, 0, 0), (1, 0, 0), (1, 1, 0)]
        int[] faceVertexCounts = [3]
        int[] faceVertexIndices = [0, 1, 2]
    }

    def Scope "Looks"
    {
        def Material "Red"
        {
            token outputs:surface.connect = </World/Looks/Red/Surface.outputs:surface>
            def Shader "Surface"
            {
                uniform token info:id = "UsdPreviewSurface"
                color3f inputs:diffuseColor = (0.8, 0.1, 0.1)
                token outputs:surface
            }
        }
    }
}
"#;

    #[test]
    fn reads_meshes_transforms_and_materials() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("quad.usda");
        std::fs::write(&path, CUBE).unwrap();
        let mesh = load(&path, dir.path()).unwrap().unwrap();
        assert_eq!(mesh.prims.len(), 1, "the invisible mesh is left out");
        assert_eq!(mesh.triangles(), 2);
        let prim = &mesh.prims[0];
        // Translated 10 along X; Z up becomes Y up, so the quad in the XY plane lies in XZ.
        assert_eq!(prim.positions[1], [11.0, 0.0, 0.0]);
        assert_eq!(prim.positions[2], [11.0, 0.0, -1.0]);
        let n = prim.normals[0];
        assert!((n[1] - 1.0).abs() < 1e-5, "the +Z face normal points up: {n:?}");
        assert_eq!(prim.uvs[2], [1.0, 0.0], "V flipped to glTF's top-left origin");
        let m = &mesh.materials[prim.material];
        assert_eq!(m.name, "Red");
        assert!((m.color[0] - 0.8).abs() < 1e-6 && (m.color[1] - 0.1).abs() < 1e-6);
    }

    #[test]
    fn a_material_library_has_no_mesh() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("looks.usda");
        std::fs::write(
            &path,
            "#usda 1.0
def Material \"Planks\"
{
}
",
        )
        .unwrap();
        assert!(load(&path, dir.path()).unwrap().is_none());
    }

    #[test]
    fn a_mesh_with_no_material_shows_its_display_colour() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tri.usda");
        std::fs::write(
            &path,
            "#usda 1.0\ndef Mesh \"Tri\"\n{\n    point3f[] points = [(0, 0, 0), (1, 0, 0), (0, 1, 0)]\n    int[] faceVertexCounts = [3]\n    int[] faceVertexIndices = [0, 1, 2]\n    color3f[] primvars:displayColor = [(0, 0.5, 1)] (interpolation = \"constant\")\n}\n",
        )
        .unwrap();
        let mesh = load(&path, dir.path()).unwrap().unwrap();
        assert_eq!(mesh.materials[mesh.prims[0].material].color, [0.0, 0.5, 1.0, 1.0]);
        // Default units are centimeters.
        assert_eq!(mesh.prims[0].positions[1], [0.01, 0.0, 0.0]);
    }
}
