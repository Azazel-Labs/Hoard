//! "Download as": an asset in another format than it was uploaded in, converted on request and cached as a
//! derivative (`as:<format>:<max size>`), so the second download is a plain blob read.
//!
//! Images go through `image` (plus Hoard's PSD/Krita/OpenRaster composites), fonts, meshes and glTF through Hoard's
//! own code. Audio Hoard decodes itself becomes WAV or FLAC here; MP3, OGG Vorbis and Opus use ffmpeg's reference
//! encoders when ffmpeg is ready (pure-Rust MP3 and Opus otherwise), and formats only ffmpeg reads (AAC/M4A,
//! WMA, …) are offered once it is. Models Blender converted also offer that GLB.

mod cache;
mod encode;
mod gltf;
mod jobs;

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use hoard_core::FormatOption;
use image::DynamicImage;

use crate::sources;
pub(crate) use cache::{conversion_cache_bytes, log_trim};
use encode::{convert_audio, encode_image};
pub(crate) use gltf::{build_glb, gltf_buffer_files, gltf_buffers_to_glb, gltf_image_files, read_glb};
use gltf::{file_stem, glb_to_gltf_zip, gltf_image_files_found};
pub(crate) use jobs::{Lookup, Ready, Refused, Stage, request, valid_max, wait, wait_for_change};

/// Image formats we can read pixels from.
pub(crate) const IMAGE_IN: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "tga", "webp", "tif", "tiff", "hdr", "exr", "psd", "psb", "kra", "ora", "dds",
    "ktx", "ktx2", "basis", "pvr", "vtf", "svg", "svgz", "ai", "aseprite", "ase", "pcx", "ico", "cur", "qoi", "ppm",
    "pgm", "pbm", "pnm", "pam", "xcf", "jxl", "avif", "heic", "heif",
];
/// Rendered size for vector art when no size is asked for.
const SVG_DEFAULT: u32 = 2048;
const HDR_IN: &[&str] = &["hdr", "exr"];
/// Audio formats decoded here (symphonia, and opus-pure for Opus).
const AUDIO_IN: &[&str] = &["wav", "wave", "ogg", "oga", "opus", "flac", "mp3", "aif", "aiff"];

fn opt(format: &str, label: &str, resizable: bool) -> FormatOption {
    FormatOption { format: format.to_owned(), label: label.to_owned(), resizable }
}

/// A model as a GLB with its textures as files beside it, turned the way its pack's models face.
const GLB_LABEL: &str = "GLB + textures (zip)";

/// What this asset can be downloaded as, besides itself.
pub(crate) fn options(ext: &str, derivatives: &[String]) -> Vec<FormatOption> {
    let ext = ext.to_ascii_lowercase();
    let mut out = Vec::new();
    if IMAGE_IN.contains(&ext.as_str()) {
        let same = |f: &str| ext == f || (f == "jpg" && ext == "jpeg") || (f == "tiff" && ext == "tif");
        for (f, label) in
            [("png", "PNG"), ("jpg", "JPEG"), ("webp", "WebP (lossless)"), ("tga", "TGA"), ("tiff", "TIFF")]
        {
            if !same(f) {
                out.push(opt(f, label, true));
            }
        }
        if HDR_IN.contains(&ext.as_str()) && ext != "exr" {
            out.push(opt("exr", "OpenEXR (HDR)", true));
        }
    }
    let ours = AUDIO_IN.contains(&ext.as_str()) || crate::music::EXTS.contains(&ext.as_str());
    let ffmpeg_reads = crate::ffmpeg::demuxer(&ext).is_some() && crate::ffmpeg::tool().is_some();
    if ours || ffmpeg_reads {
        if !matches!(ext.as_str(), "wav" | "wave") {
            out.push(opt("wav", "WAV", false));
        }
        if ext != "flac" {
            out.push(opt("flac", "FLAC (lossless)", false));
        }
        if ext != "mp3" && (ours || crate::ffmpeg::encodes("mp3")) {
            out.push(opt("mp3", "MP3 (VBR, ~190 kbps)", false));
        }
        if !matches!(ext.as_str(), "ogg" | "oga") && crate::ffmpeg::encodes("ogg") {
            out.push(opt("ogg", "OGG Vorbis (q5, ~160 kbps)", false));
        }
        if ext != "opus" && (ours || crate::ffmpeg::encodes("opus")) {
            out.push(opt("opus", "Opus (128 kbps)", false));
        }
    }
    match ext.as_str() {
        "stl" | "ply" | "vox" | "obj" | "fbx" | "3ds" | "3mf" | "usd" | "usda" | "usdc" | "usdz" | "gltf" | "skp" => {
            out.push(opt("glb", GLB_LABEL, false));
        }
        "ttf" | "otf" => {
            out.push(opt("woff2", "WOFF2 (for the web)", false));
            out.push(opt("woff", "WOFF", false));
        }
        "woff" => {
            out.push(opt("ttf", "TTF / OTF", false));
            out.push(opt("woff2", "WOFF2", false));
        }
        "woff2" => {
            out.push(opt("ttf", "TTF / OTF", false));
            out.push(opt("woff", "WOFF", false));
        }
        "glb" => {
            out.push(opt("glb", GLB_LABEL, false));
            out.push(opt("gltf", "glTF (.gltf + .bin, zipped)", false));
        }
        _ if derivatives.iter().any(|d| d == "glb") => {
            out.push(opt("glb", "GLB + textures (zip, converted by Blender)", false));
        }
        _ => {}
    }
    // A sprite sheet, in the formats it can be written in.
    if derivatives.iter().any(|d| d == "sheet") {
        out.extend(crate::spritesheet::options());
    }
    out
}

/// A converted file.
#[derive(Debug)]
pub(crate) struct Output {
    pub(crate) bytes: Vec<u8>,
    /// Extension of the result (`png`, or `zip` for a zipped glTF).
    pub(crate) ext: &'static str,
}

/// The same-format target for an image, for "original format, smaller".
pub(crate) fn resizable_as_itself(ext: &str) -> Option<&'static str> {
    Some(match ext.to_ascii_lowercase().as_str() {
        "png" => "png",
        "jpg" | "jpeg" => "jpg",
        "webp" => "webp",
        "tga" => "tga",
        "tif" | "tiff" => "tiff",
        "exr" => "exr",
        _ => return None,
    })
}

/// What a conversion may report while it runs, and what's known that helps it.
pub(crate) struct Progress<'a> {
    /// Seconds of audio, when the asset's analysis measured it.
    pub(crate) duration: Option<f64>,
    /// 0..=1.
    pub(crate) report: &'a (dyn Fn(f32) + Sync),
}

impl Progress<'_> {
    #[cfg(test)]
    pub(crate) const NONE: Progress<'static> = Progress { duration: None, report: &|_| {} };
}

/// Convert a local copy of the asset. A `.gltf` must sit in a folder laid out like the pack, so its external
/// buffers and images resolve.
pub(crate) fn convert(
    local: &Path,
    root: &Path,
    src_ext: &str,
    format: &str,
    max: Option<u32>,
    progress: &Progress<'_>,
) -> Result<Output> {
    let src = src_ext.to_ascii_lowercase();
    match format {
        "png" | "jpg" | "webp" | "tga" | "tiff" | "exr" if IMAGE_IN.contains(&src.as_str()) => {
            // Vector art renders straight at the asked size.
            if crate::textures::VECTOR.contains(&src.as_str()) {
                let img = crate::textures::vector(local, &src, max.unwrap_or(SVG_DEFAULT))?.image;
                return encode_image(&img, format);
            }
            let mut img = load_image(local, &src)?;
            if let Some(m) = max.filter(|&m| img.width() > m || img.height() > m) {
                img = img.resize(m, m, image::imageops::FilterType::Lanczos3);
            }
            encode_image(&img, format)
        }
        "wav" | "flac" | "mp3" | "ogg" | "opus"
            if AUDIO_IN.contains(&src.as_str())
                || crate::music::EXTS.contains(&src.as_str())
                || crate::ffmpeg::demuxer(&src).is_some() =>
        {
            convert_audio(local, &src, format, progress)
        }
        "gltf" if src == "glb" => glb_to_gltf_zip(local),
        // Textures named as Settings names maps, wherever they came from.
        "glb" if crate::meshes::rebuilt_as_glb(&src) || src == "gltf" || src == "glb" => {
            let (glb, found) = match src.as_str() {
                "gltf" => (build_glb(local, true)?, gltf_image_files_found(local, root)),
                "glb" => (std::fs::read(local)?, BTreeMap::new()),
                _ => crate::meshes::glb(local, &src, root)?,
            };
            let naming = crate::naming::current_for_download();
            let names = crate::glbfiles::names_of(found, root, &naming.maps, &naming.names);
            let facing = crate::meshes::current_facing();
            crate::glbfiles::split_output(&glb, facing, &file_stem(local), &names, &naming.names)
        }
        "ttf" | "woff" | "woff2" if crate::fonts::EXTS.contains(&src.as_str()) => {
            let sfnt = crate::fonts::sfnt(local, &src)?;
            Ok(match format {
                "woff" => Output { bytes: crate::fonts::to_woff(&sfnt)?, ext: "woff" },
                "woff2" => Output { bytes: crate::fonts::to_woff2(&sfnt)?, ext: "woff2" },
                _ => Output { ext: if crate::fonts::is_cff(&sfnt) { "otf" } else { "ttf" }, bytes: sfnt },
            })
        }
        "view" if crate::meshes::reads(&src) => {
            Ok(Output { bytes: crate::meshes::view_glb(local, &src, root)?, ext: "glb" })
        }
        _ => bail!("can't convert .{src} to {format}"),
    }
}

pub(crate) fn load_image(path: &Path, ext: &str) -> Result<DynamicImage> {
    Ok(match ext {
        "psd" | "psb" => sources::psd_composite(path)?,
        "kra" | "ora" => sources::read_layered_zip(path)?.composite.context("no flattened image inside")?,
        e if crate::textures::handles(e) => crate::textures::decode(path, e)?.image,
        _ => image::ImageReader::open(path)?.with_guessed_format()?.decode()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use serde_json::json;

    use crate::audio::{Audio, decode_audio, write_wav};

    fn tmp(name: &str, bytes: &[u8]) -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join(name);
        std::fs::write(&p, bytes).unwrap();
        (dir, p)
    }

    fn png_bytes(img: &DynamicImage) -> Vec<u8> {
        let mut b = Vec::new();
        img.write_to(&mut Cursor::new(&mut b), image::ImageFormat::Png).unwrap();
        b
    }

    #[test]
    fn images_convert_and_resize() {
        let img = DynamicImage::ImageRgba8(image::RgbaImage::from_fn(300, 200, |x, y| {
            image::Rgba([(x % 256) as u8, (y % 256) as u8, 90, 255])
        }));
        let (_d, p) = tmp("T_Wall.png", &png_bytes(&img));
        for f in ["jpg", "webp", "tga", "tiff", "exr"] {
            let out = convert(&p, p.parent().unwrap(), "png", f, None, &Progress::NONE).unwrap();
            // TGA has no magic number to sniff.
            let back = match f {
                "tga" => image::load_from_memory_with_format(&out.bytes, image::ImageFormat::Tga),
                _ => image::load_from_memory(&out.bytes),
            }
            .unwrap();
            assert_eq!((back.width(), back.height()), (300, 200), "{f}");
        }
        let small = convert(&p, p.parent().unwrap(), "png", "png", Some(128), &Progress::NONE).unwrap();
        let back = image::load_from_memory(&small.bytes).unwrap();
        assert_eq!((back.width(), back.height()), (128, 85));
    }

    #[test]
    fn sixteen_bit_stays_sixteen_bit() {
        let img = DynamicImage::ImageLuma16(image::ImageBuffer::from_fn(8, 8, |x, _| image::Luma([x as u16 * 8000])));
        let (_d, p) = tmp("H_Height.png", &png_bytes(&img));
        let tiff = image::load_from_memory(
            &convert(&p, p.parent().unwrap(), "png", "tiff", None, &Progress::NONE).unwrap().bytes,
        )
        .unwrap();
        assert_eq!(tiff.to_luma16().get_pixel(7, 0).0, [56_000]);
        let png = image::load_from_memory(
            &convert(&p, p.parent().unwrap(), "png", "png", Some(64), &Progress::NONE).unwrap().bytes,
        )
        .unwrap();
        assert!(matches!(png, DynamicImage::ImageLuma16(_)));
    }

    #[test]
    fn hdr_tonemaps_into_ldr() {
        let img = DynamicImage::ImageRgba32F(image::Rgba32FImage::from_pixel(4, 4, image::Rgba([4.0, 2.0, 1.0, 1.0])));
        let mut exr = Vec::new();
        img.write_to(&mut Cursor::new(&mut exr), image::ImageFormat::OpenExr).unwrap();
        let (_d, p) = tmp("sky.exr", &exr);
        let jpg = image::load_from_memory(
            &convert(&p, p.parent().unwrap(), "exr", "jpg", None, &Progress::NONE).unwrap().bytes,
        )
        .unwrap()
        .to_rgb8();
        let px = jpg.get_pixel(1, 1).0;
        assert!(px[0] > px[2] && px[0] < 255, "{px:?}");
        assert!(options("exr", &[]).iter().all(|o| o.format != "exr"));
        assert!(options("hdr", &[]).iter().any(|o| o.format == "exr"));
    }

    /// A 3x2 RGB PSD with an RLE composite, built by hand.
    fn tiny_psd() -> Vec<u8> {
        let mut b = b"8BPS".to_vec();
        b.extend_from_slice(&1u16.to_be_bytes());
        b.extend_from_slice(&[0; 6]);
        b.extend_from_slice(&3u16.to_be_bytes()); // channels
        b.extend_from_slice(&2u32.to_be_bytes()); // height
        b.extend_from_slice(&3u32.to_be_bytes()); // width
        b.extend_from_slice(&8u16.to_be_bytes());
        b.extend_from_slice(&3u16.to_be_bytes()); // RGB
        b.extend_from_slice(&0u32.to_be_bytes()); // color mode data
        b.extend_from_slice(&0u32.to_be_bytes()); // resources
        b.extend_from_slice(&0u32.to_be_bytes()); // layers
        b.extend_from_slice(&1u16.to_be_bytes()); // RLE
        // Rows: R = [10,10,10] (a run), G = [1,2,3] (literal), B = [7,7,7] (a run). Same for both rows.
        let rows: [&[u8]; 3] = [&[0xFE, 10], &[2, 1, 2, 3], &[0xFE, 7]];
        for r in rows {
            for _ in 0..2 {
                b.extend_from_slice(&(r.len() as u16).to_be_bytes());
            }
        }
        for r in rows {
            for _ in 0..2 {
                b.extend_from_slice(r);
            }
        }
        b
    }

    #[test]
    fn psd_composite_exports_full_size() {
        let (_d, p) = tmp("Painting.psd", &tiny_psd());
        let out = convert(&p, p.parent().unwrap(), "psd", "png", None, &Progress::NONE).unwrap();
        let img = image::load_from_memory(&out.bytes).unwrap().to_rgb8();
        assert_eq!(img.dimensions(), (3, 2));
        assert_eq!(img.get_pixel(1, 1).0, [10, 2, 7]);
        assert_eq!(img.get_pixel(2, 0).0, [10, 3, 7]);
    }

    fn wav(samples: &[i16], channels: u16) -> Vec<u8> {
        write_wav(&Audio { samples: samples.iter().map(|&s| i32::from(s)).collect(), channels, rate: 22_050, bits: 16 })
    }

    #[test]
    fn wav_to_flac_is_lossless() {
        let pcm: Vec<i16> = (0..8000).map(|i| ((f64::from(i) * 0.05).sin() * 12_000.0) as i16).collect();
        let (_d, p) = tmp("tone.wav", &wav(&pcm, 2));
        let flac = convert(&p, p.parent().unwrap(), "wav", "flac", None, &Progress::NONE).unwrap();
        assert!(flac.bytes.starts_with(b"fLaC"));
        let (_d2, fp) = tmp("tone.flac", &flac.bytes);
        let back = decode_audio(&fp).unwrap();
        assert_eq!((back.channels, back.rate, back.bits), (2, 22_050, 16));
        let orig: Vec<i32> = pcm.iter().map(|&s| i32::from(s)).collect();
        assert_eq!(back.samples, orig);
        let wav_again = convert(&fp, fp.parent().unwrap(), "flac", "wav", None, &Progress::NONE).unwrap();
        assert_eq!(wav_again.bytes, wav(&pcm, 2));
    }

    #[test]
    fn gltf_and_glb_round_trip() {
        use std::io::Read;
        let dir = tempfile::tempdir().unwrap();
        let png = png_bytes(&DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(2, 2, image::Rgba([9, 8, 7, 255]))));
        std::fs::create_dir_all(dir.path().join("tex")).unwrap();
        std::fs::write(dir.path().join("tex/crate albedo.png"), &png).unwrap();
        std::fs::write(dir.path().join("crate.bin"), [1u8, 2, 3, 4, 5, 6]).unwrap();
        let doc = json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "uri": "crate.bin", "byteLength": 6 }],
            "bufferViews": [{ "buffer": 0, "byteOffset": 2, "byteLength": 4 }],
            "images": [{ "uri": "tex/crate%20albedo.png" }],
        });
        let gltf = dir.path().join("crate.gltf");
        std::fs::write(&gltf, serde_json::to_vec(&doc).unwrap()).unwrap();

        // Packed into one GLB, its textures beside it under the names of their files.
        let out = convert(&gltf, gltf.parent().unwrap(), "gltf", "glb", None, &Progress::NONE).unwrap();
        assert_eq!(out.ext, "zip");
        let mut zip = zip::ZipArchive::new(Cursor::new(out.bytes)).unwrap();
        let mut tex = Vec::new();
        zip.by_name("textures/crate_albedo.png").unwrap().read_to_end(&mut tex).unwrap();
        assert_eq!(tex, png);
        let mut glb = Vec::new();
        zip.by_name("crate.glb").unwrap().read_to_end(&mut glb).unwrap();
        let (doc2, bin) = read_glb(&glb).unwrap();
        assert_eq!(&bin[..4], &[3, 4, 5, 6]);
        assert_eq!(doc2["images"][0]["uri"], "textures/crate_albedo.png");

        // A GLB with its textures inside unpacks to a glTF and its .bin.
        let glb_path = dir.path().join("crate.glb");
        std::fs::write(&glb_path, build_glb(&gltf, true).unwrap()).unwrap();
        let zipped = convert(&glb_path, glb_path.parent().unwrap(), "glb", "gltf", None, &Progress::NONE).unwrap();
        let mut zip = zip::ZipArchive::new(Cursor::new(zipped.bytes)).unwrap();
        let names: Vec<String> =
            (0..zip.len()).map(|i| zip.by_index(i).unwrap().name().unwrap().into_owned()).collect();
        assert_eq!(names, ["crate.gltf", "crate.bin"]);
    }

    #[test]
    fn refuses_escaping_paths() {
        let dir = tempfile::tempdir().unwrap();
        let gltf = dir.path().join("a.gltf");
        std::fs::write(&gltf, br#"{"asset":{"version":"2.0"},"buffers":[{"uri":"../../etc/passwd","byteLength":1}]}"#)
            .unwrap();
        assert!(convert(&gltf, gltf.parent().unwrap(), "gltf", "glb", None, &Progress::NONE).is_err());
    }
}
