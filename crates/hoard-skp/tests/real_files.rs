//! Converts the fixture files and, when present, the larger SketchUp 2015 models kept outside the repo, checking
//! that each GLB parses and holds geometry of a plausible size.

#![expect(clippy::unwrap_used, reason = "test helpers")]
#![expect(clippy::print_stdout, reason = "the sizes are worth seeing with --nocapture")]

use std::path::{Path, PathBuf};

/// Triangle count, world-space bounding box in metres, and material and image counts.
struct Stats {
    triangles: usize,
    min: [f32; 3],
    max: [f32; 3],
    materials: usize,
    images: usize,
}

fn mul(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0; 4]; 4];
    for (c, col) in out.iter_mut().enumerate() {
        for (r, v) in col.iter_mut().enumerate() {
            *v = (0..4).map(|k| a[k][r] * b[c][k]).sum();
        }
    }
    out
}

fn visit(node: &gltf::Node<'_>, parent: &[[f32; 4]; 4], buffers: &[gltf::buffer::Data], s: &mut Stats) {
    let m = mul(parent, &node.transform().matrix());
    if let Some(mesh) = node.mesh() {
        for prim in mesh.primitives() {
            let reader = prim.reader(|b| Some(&buffers[b.index()]));
            s.triangles += reader.read_indices().unwrap().into_u32().count() / 3;
            for p in reader.read_positions().unwrap() {
                for axis in 0..3 {
                    let v = (0..3).map(|k| m[k][axis] * p[k]).sum::<f32>() + m[3][axis];
                    s.min[axis] = s.min[axis].min(v);
                    s.max[axis] = s.max[axis].max(v);
                }
            }
        }
    }
    for child in node.children() {
        visit(&child, &m, buffers, s);
    }
}

fn stats(glb: &[u8]) -> Stats {
    let (doc, buffers, images) = gltf::import_slice(glb).unwrap();
    let mut s = Stats {
        triangles: 0,
        min: [f32::INFINITY; 3],
        max: [f32::NEG_INFINITY; 3],
        materials: doc.materials().count(),
        images: images.len(),
    };
    let identity = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];
    for scene in doc.scenes() {
        for node in scene.nodes() {
            visit(&node, &identity, &buffers, &mut s);
        }
    }
    s
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size: Vec<String> = (0..3).map(|i| format!("{:.2}", self.max[i] - self.min[i])).collect();
        write!(
            f,
            "{} triangles, {} materials, {} images, size {} m (x, y, z), min {:.2?}, max {:.2?}",
            self.triangles,
            self.materials,
            self.images,
            size.join(" x "),
            self.min,
            self.max
        )
    }
}

fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data")
}

/// Fixtures that hold materials or annotations but no faces, which `to_glb` reports as an error.
const NO_FACES: [&str; 5] = [
    "blank_v17",
    "legacy_empty_2020",
    "legacy_textured_colorize_2017",
    "single_material_v17",
    "zero_material_custom_layer_v18",
];

#[test]
fn fixtures_convert() {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(data_dir()).unwrap().map(|e| e.unwrap().path()).collect();
    paths.sort();
    for path in paths {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let bytes = std::fs::read(&path).unwrap();
        assert!(hoard_skp::is_skp(&bytes), "{name}");
        match hoard_skp::to_glb(&bytes) {
            Ok(glb) => {
                if let Some(dir) = std::env::var_os("HOARD_SKP_FIXTURE_OUT") {
                    std::fs::write(Path::new(&dir).join(format!("{name}.glb")), &glb).unwrap();
                }
                let s = stats(&glb);
                println!("{name}: {s}");
                assert!(s.triangles > 0, "{name}");
                assert!(s.min.iter().chain(&s.max).all(|v| v.is_finite()), "{name}");
            }
            Err(e) => {
                println!("{name}: {e:#}");
                let message = format!("{e:#}");
                assert!(NO_FACES.contains(&name.as_str()) && message.contains("no visible faces"), "{name}: {message}");
            }
        }
    }
}

/// The three SketchUp 2015 models this reader was built against, downloaded beside the session's scratch files.
/// Set `HOARD_SKP_SAMPLES` to the folder holding them; the test is skipped when it isn't set or they are absent.
#[test]
fn sample_models_convert() {
    let Some(dir) = std::env::var_os("HOARD_SKP_SAMPLES").map(PathBuf::from) else { return };
    for name in ["house", "towerDefense", "spaceInteriorElements"] {
        let path = dir.join(format!("{name}.skp"));
        let Ok(bytes) = std::fs::read(&path) else { continue };
        let glb = hoard_skp::to_glb(&bytes).unwrap();
        let s = stats(&glb);
        println!("{name}: {s}");
        assert!(s.triangles > 0, "{name}");
        let size: Vec<f32> = (0..3).map(|i| s.max[i] - s.min[i]).collect();
        assert!(size.iter().all(|v| v.is_finite() && *v < 10_000.0), "{name}: {size:?}");
        assert!(size.iter().any(|v| *v > 0.1), "{name}: {size:?}");
        std::fs::write(dir.join(format!("{name}.glb")), glb).unwrap();
    }
}
