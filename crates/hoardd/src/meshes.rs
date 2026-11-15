//! Models read natively, by [mithril]: STL, PLY, `MagicaVoxel` `.vox`, OBJ (with its MTL materials and textures),
//! FBX (binary and ASCII), 3DS, 3MF and USD. Each becomes a GLB, which feeds the thumbnail renderer and "download
//! as GLB"; textures are embedded so the GLB stands on its own, re-encoded as PNG from anything Hoard can decode.
//! glTF and GLB are read too, for the triangles they draw, and decoded into plain glTF when compressed or quantized
//! so the thumbnail renderer can draw them; their download GLB is packed from the file as it is.

use std::path::{Path, PathBuf};

use anyhow::Result;
use mithril::assets::{Assets, Folder};
use mithril::document::{Keep, Pose};
use serde_json::{Map, Value};

pub(crate) use mithril::mesh::{Image, Material, Mesh, to_glb};

/// Whether a model in this format becomes a GLB by being read into a mesh. A glTF is packed as it is instead,
/// keeping what a mesh leaves out (animation, skins, compression).
pub(crate) fn rebuilt_as_glb(ext: &str) -> bool {
    mithril::formats::EXTENSIONS.contains(&ext) && !matches!(ext, "gltf" | "glb")
}

/// A pack laid out in a folder: mithril finds the files a model points at, and Hoard decodes the textures.
struct Pack(Folder);

impl Assets for Pack {
    fn find(&self, reference: &str, model_dir: &Path) -> Option<PathBuf> {
        self.0.find(reference, model_dir)
    }

    fn image(&self, bytes: &[u8], name: &Path) -> Option<Image> {
        image_from_bytes(bytes, name)
    }

    fn color_texture_for(&self, model: &Path) -> Option<PathBuf> {
        self.0.color_texture_for(model)
    }
}

/// Load a model and the facts worth recording about it. Point clouds (PLY without faces) give no mesh. `root` is
/// the folder laid out like the pack, searched for textures the model points at by bare name or a foreign path.
pub(crate) fn load(path: &Path, ext: &str, root: &Path) -> Result<(Option<Mesh>, Map<String, Value>)> {
    // What a rest-pose mesh needs: animations, cameras and lights are skipped.
    let doc = mithril::formats::load(path, ext, &Pack(Folder::new(root)), Keep::MESHES)?;
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
    Ok(mithril::formats::load(path, ext, &Pack(Folder::new(root)), nothing)?.facts)
}

/// A model in another format as a GLB for download: its whole document (hierarchy, skins, morph targets, animation,
/// materials and embedded textures), not just the mesh it draws.
pub(crate) fn glb(path: &Path, ext: &str, root: &Path) -> Result<Vec<u8>> {
    let doc = mithril::formats::load(path, ext, &Pack(Folder::new(root)), Keep::ALL)?;
    anyhow::ensure!(doc.counts().triangles > 0, "this model has no surfaces to export");
    Ok(doc.to_glb()?)
}

/// A glTF rewritten as plain glTF in a GLB, for a reader without meshopt, Draco or quantization (the thumbnail
/// renderer): mithril decodes those and takes the images in. Animations, cameras and lights are left out.
pub(crate) fn plain_glb(path: &Path, ext: &str) -> Result<Vec<u8>> {
    let folder = path.parent().unwrap_or(Path::new("."));
    let mut doc = mithril::formats::load(path, ext, &Pack(Folder::new(folder)), Keep::MESHES)?;
    doc.decode()?;
    Ok(doc.to_glb()?)
}

/// Texture bytes (a file, or embedded in an FBX) as PNG or JPEG: those as they are, anything else Hoard can
/// decode (TGA, BMP, DDS, PSD, …) re-encoded as PNG.
fn image_from_bytes(bytes: &[u8], hint: &Path) -> Option<Image> {
    if let Some(img) = Image::png_or_jpeg(bytes) {
        return Some(img);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textures_mithril_wont_embed_are_converted_to_png() {
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
    fn png_and_jpeg_pass_through() {
        let png = b"\x89PNG\r\n\x1a\nrest".to_vec();
        assert_eq!(image_from_bytes(&png, Path::new("x.png")).unwrap().bytes, png);
        assert!(image_from_bytes(b"not an image", Path::new("x.bin")).is_none());
    }
}
