//! Readers for authoring ("source") formats that general image/model libraries don't handle. Each reads only
//! what Hoard needs, a preview and a few facts, so a 3 GB file costs a few reads, not a full parse.
//!
//! - `.blend`: Blender embeds a small preview (the `TEST` block) near the start of every file it saves with
//!   "save preview" on (the default). A preview of the camera or viewport is square; a screenshot of the whole
//!   Blender window keeps the window's shape, and is left out. Files may be gzip (pre-3.0) or zstd (3.0+) compressed. Image (`IM`) and
//!   library (`LI`) blocks hold the paths of external textures and linked .blend files.
//! - `.psd`/`.psb`: header facts, layer count, and the JPEG thumbnail Photoshop stores in image resource 1036.
//! - `.kra` (Krita) and `.ora` (OpenRaster): zip files with a flattened `mergedimage.png` inside.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use anyhow::{Context, Result};
use image::DynamicImage;

mod blend;
mod psd;
mod riff;

pub(crate) use blend::read_blend;
pub(crate) use psd::{psd_composite, read_psd};
pub(crate) use riff::{note_name, read_riff};

#[derive(Debug, Default)]
pub(crate) struct LayeredZipInfo {
    pub(crate) composite: Option<DynamicImage>,
    pub(crate) layers: Option<u32>,
}

/// Krita and `OpenRaster` both store a flattened `mergedimage.png`; fall back to the small preview.
pub(crate) fn read_layered_zip(path: &Path) -> Result<LayeredZipInfo> {
    let mut zip = zip::ZipArchive::new(BufReader::new(File::open(path)?)).context("not a zip-based image")?;
    let mut info = LayeredZipInfo::default();
    for name in ["mergedimage.png", "preview.png", "Thumbnails/thumbnail.png"] {
        let Ok(mut entry) = zip.by_name(name) else { continue };
        if entry.size() > 512 << 20 {
            continue;
        }
        let mut buf = Vec::with_capacity(usize::try_from(entry.size()).unwrap_or(0));
        entry.read_to_end(&mut buf)?;
        if let Ok(img) = image::load_from_memory_with_format(&buf, image::ImageFormat::Png) {
            info.composite = Some(img);
            break;
        }
    }
    // Layer count from the document XML: maindoc.xml (Krita) or stack.xml (OpenRaster).
    for name in ["maindoc.xml", "stack.xml"] {
        let Ok(mut entry) = zip.by_name(name) else { continue };
        let mut xml = String::new();
        if entry.by_ref().take(16 << 20).read_to_string(&mut xml).is_ok() {
            let n = xml.matches("<layer ").count() + xml.matches("<layer>").count();
            info.layers = Some(n as u32);
            break;
        }
    }
    Ok(info)
}

// WAV (RIFF / RF64 / BWF) metadata

#[cfg(test)]
mod tests;
