//! Models read natively, by [polysmelt]: STL, PLY, `MagicaVoxel` `.vox`, OBJ (with its MTL materials and textures),
//! FBX (binary and ASCII), 3DS, 3MF and USD. Each becomes a GLB, which feeds the thumbnail renderer and "download
//! as GLB"; textures are embedded so the GLB stands on its own, re-encoded as PNG from anything Hoard can decode.
//! glTF and GLB are read too, for the triangles they draw, and decoded into plain glTF when compressed or quantized
//! so the thumbnail renderer can draw them; their download GLB is packed from the file as it is. Every one of them
//! reaches the 3D viewer as a plain GLB with web-ready textures ([`view_glb`]).

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use polysmelt::assets::{Assets, Folder};
use polysmelt::document::{Keep, Pose};
use serde_json::{Map, Value};

pub(crate) use polysmelt::mesh::{Image, Material, Mesh, to_glb};

/// Whether a model in this format becomes a GLB by being read into a mesh. A glTF is packed as it is instead,
/// keeping what a mesh leaves out (animation, skins, compression).
pub(crate) fn rebuilt_as_glb(ext: &str) -> bool {
    polysmelt::formats::EXTENSIONS.contains(&ext) && !matches!(ext, "gltf" | "glb")
}

/// A pack laid out in a folder: polysmelt finds the files a model points at, and Hoard decodes the textures.
struct Pack {
    folder: Folder,
    /// Whether a model naming no color texture takes one named after it.
    guess: bool,
    /// Image files looked up, by lowercase file name: whether each was found.
    images: RefCell<BTreeMap<String, (String, bool)>>,
}

impl Pack {
    fn new(root: &Path) -> Self {
        Self { folder: Folder::new(root), guess: true, images: RefCell::default() }
    }

    /// The image files the model named that aren't in the pack. A reader may try a texture under several names (an
    /// FBX's relative and absolute paths): one found under the same file name counts.
    fn missing(&self) -> Vec<String> {
        self.images.borrow().values().filter(|(_, found)| !found).map(|(name, _)| name.clone()).collect()
    }
}

impl Assets for Pack {
    fn find(&self, reference: &str, model_dir: &Path) -> Option<PathBuf> {
        let found = self.folder.find(reference, model_dir);
        let name = reference.trim().rsplit(['/', '\\']).next().unwrap_or_default();
        let ext = name.rsplit_once('.').map_or_default(|(_, e)| e.to_ascii_lowercase());
        if crate::formats::IMAGE_IN.contains(&ext.as_str()) || ext == "tga" {
            let mut images = self.images.borrow_mut();
            let seen = images.entry(name.to_lowercase()).or_insert_with(|| (name.to_owned(), false));
            seen.1 |= found.is_some();
        }
        found
    }

    fn image(&self, bytes: &[u8], name: &Path) -> Option<Image> {
        image_from_bytes(bytes, name)
    }

    fn color_texture_for(&self, model: &Path) -> Option<PathBuf> {
        self.guess.then(|| self.folder.color_texture_for(model)).flatten()
    }
}

/// Load a model and the facts worth recording about it. Point clouds (PLY without faces) give no mesh. `root` is
/// the folder laid out like the pack, searched for textures the model points at by bare name or a foreign path.
pub(crate) fn load(path: &Path, ext: &str, root: &Path) -> Result<(Option<Mesh>, Map<String, Value>)> {
    // What a rest-pose mesh needs: animations, cameras and lights are skipped.
    let doc = polysmelt::formats::load(path, ext, &Pack::new(root), Keep::MESHES)?;
    Ok((doc.flatten(Pose::Rest)?, doc.facts))
}

/// The facts about a model without drawing its mesh. A glTF's are counted from its JSON, so nothing in its buffers
/// (Draco, meshopt, textures) is decoded.
pub(crate) fn facts(path: &Path, ext: &str, root: &Path) -> Result<Map<String, Value>> {
    let nothing = Keep {
        animations: false,
        skins: false,
        morph_targets: false,
        materials: false,
        textures: false,
        cameras: false,
        lights: false,
        extras: false,
    };
    Ok(polysmelt::formats::load(path, ext, &Pack::new(root), nothing)?.facts)
}

/// A model in another format as a GLB for download: its whole document (hierarchy, skins, morph targets, animation,
/// materials and embedded textures), not just the mesh it draws.
pub(crate) fn glb(path: &Path, ext: &str, root: &Path) -> Result<Vec<u8>> {
    let doc = polysmelt::formats::load(path, ext, &Pack::new(root), Keep::ALL)?;
    anyhow::ensure!(doc.counts().triangles > 0, "this model has no surfaces to export");
    Ok(doc.to_glb()?)
}

/// A glTF rewritten as plain glTF in a GLB, for a reader without meshopt, Draco or quantization (the thumbnail
/// renderer): polysmelt decodes those and takes the images in. Animations, cameras and lights are left out.
pub(crate) fn plain_glb(path: &Path, ext: &str) -> Result<Vec<u8>> {
    let folder = path.parent().unwrap_or(Path::new("."));
    let mut doc = polysmelt::formats::load(path, ext, &Pack::new(folder), Keep::MESHES)?;
    doc.decode()?;
    Ok(doc.to_glb()?)
}

/// A model as the 3D viewer shows it: its whole document as a plain GLB, Draco, meshopt and quantization decoded,
/// with web-ready textures (see [`crate::webglb`]). Nothing is guessed: the viewer tries textures named after a
/// model on itself, where it can tell whether the model is colored already. The image files it names that the pack
/// lacks are listed in `asset.extras.missing_textures`.
pub(crate) fn view_glb(path: &Path, ext: &str, root: &Path) -> Result<Vec<u8>> {
    let pack = Pack { guess: false, ..Pack::new(root) };
    let mut doc = polysmelt::formats::load(path, ext, &pack, Keep::ALL)?;
    doc.decode()?;
    let c = doc.counts();
    anyhow::ensure!(c.triangles + c.points + c.lines > 0, "this model draws nothing");
    let missing = pack.missing();
    if !missing.is_empty()
        && let Some(asset) = doc.json.get_mut("asset").and_then(Value::as_object_mut)
    {
        let extras = asset.entry("extras").or_insert_with(|| Value::Object(Map::new()));
        if let Some(extras) = extras.as_object_mut() {
            extras.insert("missing_textures".into(), missing.into());
        }
    }
    crate::webglb::repack(&doc.to_glb()?)
}

/// Texture bytes (a file, or embedded in an FBX) as PNG or JPEG: those as they are, anything else Hoard can
/// decode (TGA, BMP, DDS, PSD, …) re-encoded as PNG.
fn image_from_bytes(bytes: &[u8], hint: &Path) -> Option<Image> {
    if let Some(img) = Image::png_or_jpeg(bytes) {
        return Some(img);
    }
    let ext = hint.extension().map_or_default(|e| e.to_string_lossy().to_lowercase());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textures_polysmelt_wont_embed_are_converted_to_png() {
        let dir = tempfile::tempdir().unwrap();
        let obj = dir.path().join("rock.obj");
        std::fs::write(
            &obj,
            "mtllib rock.mtl\nv 0 0 0\nv 1 0 0\nv 1 1 0\nvt 0 0\nvt 1 0\nvt 1 1\nusemtl Rock\nf 1/1 2/2 3/3\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("rock.mtl"), "newmtl Rock\nKd 1 1 1\nmap_Kd rock.tga\n").unwrap();
        image::RgbaImage::from_pixel(4, 4, image::Rgba([200, 40, 30, 255]))
            .save_with_format(dir.path().join("rock.tga"), image::ImageFormat::Tga)
            .unwrap();
        let (Some(mesh), facts) = load(&obj, "obj", dir.path()).unwrap() else { panic!("no mesh") };
        assert_eq!(facts["triangles"], 1);
        assert_eq!(mesh.images.len(), 1);
        assert_eq!(mesh.images[0].mime, "image/png");
        assert_eq!(mesh.materials[0].color_image, Some(0));
        assert!(to_glb(&mesh).unwrap().starts_with(b"glTF"));
    }

    #[test]
    fn the_viewer_glb_has_web_textures_and_names_the_missing_ones() {
        let dir = tempfile::tempdir().unwrap();
        let obj = dir.path().join("rock.obj");
        std::fs::write(
            &obj,
            "mtllib rock.mtl\nv 0 0 0\nv 1 0 0\nv 1 1 0\nvt 0 0\nvt 1 0\nvt 1 1\nusemtl Rock\nf 1/1 2/2 3/3\n\
             usemtl Moss\nf 3/3 2/2 1/1\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rock.mtl"),
            "newmtl Rock\nKd 1 1 1\nmap_Kd C:\\art\\rock.tga\nnewmtl Moss\nKd 1 1 1\nmap_Kd moss.png\n",
        )
        .unwrap();
        image::RgbaImage::from_pixel(4, 4, image::Rgba([200, 40, 30, 255]))
            .save_with_format(dir.path().join("rock.tga"), image::ImageFormat::Tga)
            .unwrap();
        let (json, _) = crate::formats::read_glb(&view_glb(&obj, "obj", dir.path()).unwrap()).unwrap();
        assert_eq!(json["images"][0]["mimeType"], "image/jpeg");
        assert_eq!(json["asset"]["extras"]["missing_textures"], serde_json::json!(["moss.png"]));
    }

    #[test]
    fn png_and_jpeg_pass_through() {
        let png = b"\x89PNG\r\n\x1a\nrest".to_vec();
        assert_eq!(image_from_bytes(&png, Path::new("x.png")).unwrap().bytes, png);
        assert!(image_from_bytes(b"not an image", Path::new("x.bin")).is_none());
    }
}
