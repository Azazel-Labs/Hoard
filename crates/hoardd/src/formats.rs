//! "Download as": an asset in another format than it was uploaded in, converted on request and cached as a
//! derivative (`as:<format>:<max size>`), so the second download is a plain blob read.
//!
//! Images go through `image` (plus Hoard's PSD/Krita/OpenRaster composites), fonts, meshes and glTF through Hoard's
//! own code. Audio Hoard decodes itself becomes WAV or FLAC here; MP3, OGG Vorbis and Opus use ffmpeg's reference
//! encoders when ffmpeg is ready (pure-Rust MP3 and Opus otherwise), and formats only ffmpeg reads (AAC/M4A, Opus,
//! WMA, …) are offered once it is. Models Blender converted also offer that GLB.

use std::collections::BTreeMap;
use std::io::{Cursor, Write};
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use hoard_core::FormatOption;
use image::DynamicImage;
use serde_json::{Value, json};

use crate::audio::{Audio, decode_audio, write_flac, write_wav};
use crate::i18n::tr;
use crate::{analyze, sources};

/// Image formats we can read pixels from.
pub(crate) const IMAGE_IN: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "tga", "webp", "tif", "tiff", "hdr", "exr", "psd", "psb", "kra", "ora", "dds",
    "ktx", "ktx2", "basis", "pvr", "vtf", "svg", "svgz", "ai", "aseprite", "ase", "pcx", "ico", "cur", "qoi", "ppm",
    "pgm", "pbm", "pnm", "pam",
];
/// Rendered size for vector art when no size is asked for.
const SVG_DEFAULT: u32 = 2048;
const HDR_IN: &[&str] = &["hdr", "exr"];
/// Audio formats symphonia decodes here.
const AUDIO_IN: &[&str] = &["wav", "wave", "ogg", "oga", "flac", "mp3", "aif", "aiff"];

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
        e if crate::textures::EXTS.contains(&e) => crate::textures::decode(path, e)?.image,
        _ => image::ImageReader::open(path)?.with_guessed_format()?.decode()?,
    })
}

/// Audio to `format`. WAV and FLAC are written by Hoard from what it decodes or renders; lossy formats go through
/// ffmpeg's reference encoders when it's ready, else the pure-Rust MP3/Opus encoders. Sources Hoard can't decode go
/// to ffmpeg whole.
fn convert_audio(local: &Path, src: &str, format: &str, progress: &Progress<'_>) -> Result<Output> {
    let ext = output_ext(format);
    let lossy = matches!(format, "mp3" | "ogg" | "opus");
    let ffmpeg = crate::ffmpeg::tool();
    let out_path = local.with_extension(format!("hoard-out.{ext}"));
    let via_ffmpeg = |input: &Path, input_ext: &str, duration: Option<f64>| -> Result<Output> {
        let tool = ffmpeg.as_ref().context("ffmpeg isn't ready yet; try again shortly")?;
        crate::ffmpeg::convert(tool, input, input_ext, format, &out_path, duration, progress.report)?;
        let bytes = std::fs::read(&out_path)?;
        std::fs::remove_file(&out_path).ok();
        Ok(Output { bytes, ext })
    };
    let ours = |audio: &Audio| -> Result<Output> {
        Ok(match format {
            "wav" => Output { bytes: write_wav(audio), ext },
            "flac" => Output { bytes: write_flac(audio)?, ext },
            "mp3" => Output { bytes: crate::audio::write_mp3(audio)?, ext },
            "opus" => Output { bytes: crate::audio::write_opus(audio)?, ext },
            _ => bail!("OGG Vorbis needs ffmpeg, which isn't ready yet"),
        })
    };

    // MIDI and modules are rendered by Hoard (MIDI with the SoundFont, if one is set up), then encoded.
    if crate::music::EXTS.contains(&src) {
        let audio = crate::music::render_full(local, src)?;
        if lossy && crate::ffmpeg::encodes(format) {
            let wav = local.with_extension("hoard-render.wav");
            std::fs::write(&wav, write_wav(&audio))?;
            let seconds = audio.samples.len() as f64 / f64::from(audio.channels.max(1)) / f64::from(audio.rate.max(1));
            let out = via_ffmpeg(&wav, "wav", Some(seconds));
            std::fs::remove_file(&wav).ok();
            return out;
        }
        return ours(&audio);
    }
    let ffmpeg_reads = ffmpeg.is_some() && crate::ffmpeg::demuxer(src).is_some();
    if lossy && crate::ffmpeg::encodes(format) && ffmpeg_reads {
        return via_ffmpeg(local, src, progress.duration);
    }
    if !AUDIO_IN.contains(&src) {
        ensure!(ffmpeg_reads, ".{src} files need ffmpeg, which isn't ready yet");
        return via_ffmpeg(local, src, progress.duration);
    }
    match decode_audio(local) {
        Ok(audio) => ours(&audio),
        // An Ogg holding Opus, an MP4 called .mp3, … : ffmpeg reads what symphonia can't.
        Err(e) if ffmpeg_reads => {
            tracing::debug!(error = %format!("{e:#}"), "Hoard couldn't decode this audio; trying ffmpeg");
            via_ffmpeg(local, src, progress.duration)
        }
        Err(e) => Err(e),
    }
}

fn output_ext(format: &str) -> &'static str {
    match format {
        "wav" => "wav",
        "flac" => "flac",
        "mp3" => "mp3",
        "ogg" => "ogg",
        _ => "opus",
    }
}

fn encode_image(img: &DynamicImage, format: &str) -> Result<Output> {
    use image::ImageFormat;
    // HDR into an 8/16-bit format: tone-map rather than clip.
    let ldr = || if analyze::is_float(img) { analyze::tonemap(img) } else { img.clone() };
    let mut bytes = Vec::new();
    let mut cur = Cursor::new(&mut bytes);
    let ext = match format {
        "png" => {
            // 16-bit sources stay 16-bit.
            ldr().write_to(&mut cur, ImageFormat::Png)?;
            "png"
        }
        "jpg" => {
            let rgb = DynamicImage::ImageRgb8(ldr().to_rgb8());
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cur, 92).encode_image(&rgb)?;
            "jpg"
        }
        "webp" => {
            let rgba = ldr().to_rgba8();
            image::codecs::webp::WebPEncoder::new_lossless(&mut cur).encode(
                rgba.as_raw(),
                rgba.width(),
                rgba.height(),
                image::ExtendedColorType::Rgba8,
            )?;
            "webp"
        }
        "tga" => {
            let i = ldr();
            let i = if i.color().has_alpha() {
                DynamicImage::ImageRgba8(i.to_rgba8())
            } else {
                DynamicImage::ImageRgb8(i.to_rgb8())
            };
            i.write_to(&mut cur, ImageFormat::Tga)?;
            "tga"
        }
        "tiff" => {
            let i = if analyze::is_float(img) {
                DynamicImage::ImageRgba32F(img.to_rgba32f())
            } else if img.color().bytes_per_pixel() / img.color().channel_count() >= 2 {
                DynamicImage::ImageRgba16(img.to_rgba16())
            } else {
                DynamicImage::ImageRgba8(img.to_rgba8())
            };
            i.write_to(&mut cur, ImageFormat::Tiff)?;
            "tiff"
        }
        "exr" => {
            DynamicImage::ImageRgba32F(img.to_rgba32f()).write_to(&mut cur, ImageFormat::OpenExr)?;
            "exr"
        }
        f => bail!("unknown image format {f}"),
    };
    Ok(Output { bytes, ext })
}

/// The JSON and BIN chunks of a GLB.
pub(crate) fn read_glb(data: &[u8]) -> Result<(Value, Vec<u8>)> {
    ensure!(data.len() >= 20 && &data[..4] == b"glTF", "not a GLB file");
    let mut pos = 12;
    let (mut json, mut bin) = (None, Vec::new());
    while pos + 8 <= data.len() {
        let len = u32::from_le_bytes(data[pos..pos + 4].try_into()?) as usize;
        let kind = &data[pos + 4..pos + 8];
        let chunk = data.get(pos + 8..pos + 8 + len).context("GLB chunk runs past the end")?;
        match kind {
            b"JSON" => json = Some(serde_json::from_slice(chunk)?),
            b"BIN\0" => bin = chunk.to_vec(),
            _ => {}
        }
        pos += 8 + len;
    }
    Ok((json.context("GLB has no JSON chunk")?, bin))
}

/// GLB → `name.gltf` + `name.bin` in a zip (textures stay inside the .bin, which glTF allows).
fn glb_to_gltf_zip(path: &Path) -> Result<Output> {
    let (mut doc, bin) = read_glb(&std::fs::read(path)?)?;
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("model");
    let stem = if stem.chars().all(|c| c.is_ascii_digit()) { "model" } else { stem };
    if let Some(b) = doc.get_mut("buffers").and_then(Value::as_array_mut).and_then(|b| b.first_mut()) {
        b["uri"] = json!(format!("{stem}.bin"));
    }
    let mut out = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut out));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .last_modified_time(zip::DateTime::default_for_write());
        zip.start_file(format!("{stem}.gltf"), opts)?;
        zip.write_all(&serde_json::to_vec_pretty(&doc)?)?;
        if !bin.is_empty() {
            zip.start_file(format!("{stem}.bin"), opts)?;
            zip.write_all(&bin)?;
        }
        zip.finish()?;
    }
    Ok(Output { bytes: out, ext: "zip" })
}

/// A file's name without its extension.
fn file_stem(path: &Path) -> String {
    path.file_stem().map_or_else(|| "model".to_owned(), |s| s.to_string_lossy().into_owned())
}

/// The files a `.gltf`'s images are read from, by their bytes. Only files inside `root` are read.
fn gltf_image_files_found(path: &Path, root: &Path) -> crate::glbfiles::Found {
    let dir = path.parent().unwrap_or(Path::new("."));
    let Ok(doc) = std::fs::read(path).map_err(anyhow::Error::from).and_then(|b| Ok(serde_json::from_slice(&b)?)) else {
        return BTreeMap::new();
    };
    gltf_image_files(&doc)
        .into_iter()
        .filter_map(|rel| {
            let mut file = std::path::PathBuf::new();
            for part in dir.join(&rel).components() {
                match part {
                    std::path::Component::ParentDir => {
                        file.pop();
                    }
                    std::path::Component::CurDir => {}
                    p => file.push(p),
                }
            }
            if !file.starts_with(root) {
                return None;
            }
            Some((*blake3::hash(&std::fs::read(&file).ok()?).as_bytes(), file))
        })
        .collect()
}

/// `.gltf` → `.glb` with its buffers packed in, its images left where they are (a GLB may point at image files
/// beside it, as the glTF did): the same model in one file instead of two, meshopt compression and all. Refused
/// for what can't move losslessly, such as a buffer without a URI.
pub(crate) fn gltf_buffers_to_glb(path: &Path) -> Result<Vec<u8>> {
    build_glb(path, false)
}

/// The buffer files a `.gltf` reads, relative to its folder (data URIs aren't files).
pub(crate) fn gltf_buffer_files(doc: &Value) -> Vec<String> {
    gltf_uris(doc, "buffers")
}

/// The image files a `.gltf` reads, relative to its folder.
pub(crate) fn gltf_image_files(doc: &Value) -> Vec<String> {
    gltf_uris(doc, "images")
}

fn gltf_uris(doc: &Value, key: &str) -> Vec<String> {
    doc.get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|b| b.get("uri").and_then(Value::as_str))
        .filter(|uri| !uri.starts_with("data:"))
        .map(percent_decode)
        .collect()
}

/// Packs a `.gltf` and the files it reads (from its own folder, never outside it) into a GLB with polysmelt, which
/// re-points meshopt-compressed buffer views as it goes. With `images` false, image files stay beside the GLB.
pub(crate) fn build_glb(path: &Path, images: bool) -> Result<Vec<u8>> {
    let dir = path.parent().context("no folder")?;
    let mut doc: Value = serde_json::from_slice(&std::fs::read(path)?)?;
    // A buffer must say where its bytes are, unless it's a meshopt fallback (which decoders never read).
    let fallback = |b: &Value| {
        polysmelt::gltf::MESHOPT_EXTENSIONS
            .iter()
            .any(|n| b.pointer(&format!("/extensions/{n}/fallback")) == Some(&json!(true)))
    };
    let buffers = doc.get("buffers").and_then(Value::as_array).map_or_default(Vec::as_slice);
    ensure!(buffers.iter().all(|b| b.get("uri").is_some() || fallback(b)), "a glTF buffer has no URI");
    // Image files that stay put: their URIs are set aside while the rest is packed, then put back.
    let mut kept = Vec::new();
    if !images {
        for (i, img) in doc.get_mut("images").and_then(Value::as_array_mut).into_iter().flatten().enumerate() {
            let external = img.get("uri").and_then(Value::as_str).is_some_and(|uri| !uri.starts_with("data:"));
            if let Some(obj) = img.as_object_mut().filter(|_| external) {
                kept.push((i, obj.remove("uri")));
            }
        }
    }
    let mut read = polysmelt::gltf::files_in(dir);
    let glb = polysmelt::gltf::pack_glb(&serde_json::to_vec(&doc)?, |uri| {
        read(uri).map_err(|e| match e {
            polysmelt::Error::Io(_) => {
                polysmelt::Error::InvalidInput(format!("glTF file {uri} is missing from the pack"))
            }
            e @ polysmelt::Error::InvalidInput(_) => e,
        })
    })?;
    if kept.is_empty() {
        return Ok(glb);
    }
    // Put the image URIs back into the GLB's JSON chunk.
    let json_len = u32::from_le_bytes(glb.get(12..16).context("short GLB")?.try_into()?) as usize;
    let mut json: Value = serde_json::from_slice(glb.get(20..20 + json_len).context("short GLB")?)?;
    for (i, uri) in kept {
        if let (Some(img), Some(uri)) = (json.pointer_mut(&format!("/images/{i}")).and_then(Value::as_object_mut), uri)
        {
            img.insert("uri".into(), uri);
        }
    }
    let mut text = serde_json::to_vec(&json)?;
    text.resize(text.len().div_ceil(4) * 4, b' ');
    let rest = &glb[20 + json_len..];
    let total = 20 + text.len() + rest.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&u32::try_from(total)?.to_le_bytes());
    out.extend_from_slice(&u32::try_from(text.len())?.to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend(text);
    out.extend_from_slice(rest);
    Ok(out)
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && let Some(v) = s.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(h, 16).ok())
        {
            out.push(v);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Conversions running at once; more wait their turn (shown as queued).
static SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(2);
/// The 3D viewer's GLBs have slots of their own: someone looking at a model doesn't wait behind downloads.
static VIEW_SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(4);

/// Sizes a client may ask for: powers of two from 64 to 16384.
pub(crate) fn valid_max(max: u32) -> bool {
    max.is_power_of_two() && (64..=16_384).contains(&max)
}

/// Why a conversion was refused, for the HTTP layer.
#[derive(Debug)]
pub(crate) enum Refused {
    NotFound,
    Unsupported(String),
    Failed(String),
}

/// A converted download, ready to serve.
#[derive(Clone, Debug)]
pub(crate) struct Ready {
    pub(crate) hash: String,
    pub(crate) ext: String,
    /// The original file name without extension, for the download name.
    pub(crate) stem: String,
}

/// Where a conversion stands.
#[derive(Clone, Debug)]
pub(crate) enum Stage {
    Queued,
    /// 0..=1, when the conversion can tell.
    Working(f32),
    Done(Ready),
    Failed(String),
}

/// Conversions in progress, by asset and cache key. Several requests for the same one share it, and it carries on
/// if they all go away.
static RUNNING: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, tokio::sync::watch::Receiver<Stage>>>,
> = std::sync::LazyLock::new(Default::default);

/// A failure is remembered this long, so a client polling for it sees it rather than starting it again.
const FAILURE_KEPT: std::time::Duration = std::time::Duration::from_secs(60);

/// A cached conversion handed out in the last hour isn't dropped to make room: someone may be downloading it.
const IN_USE: time::Duration = time::Duration::hours(1);
/// How stale `used_at` may get before it's written again (so a busy conversion isn't a write per request).
const TOUCH_EVERY: time::Duration = time::Duration::minutes(10);

fn ago(d: time::Duration) -> String {
    (time::OffsetDateTime::now_utc() - d).format(&time::format_description::well_known::Rfc3339).unwrap_or_default()
}

/// Note that a cached conversion was just handed out, in the background.
fn touch_conversion(state: &crate::AppState, asset_id: &str, kind: &str) {
    let (pool, id, kind) = (state.pool.clone(), asset_id.to_owned(), kind.to_owned());
    tokio::spawn(async move {
        let r = crate::db::call(&pool, move |c| {
            c.execute(
                "UPDATE derivatives SET used_at = ?3
                  WHERE asset_id = ?1 AND kind = ?2 AND COALESCE(used_at, created_at) < ?4",
                rusqlite::params![id, kind, crate::db::now(), ago(TOUCH_EVERY)],
            )?;
            Ok(())
        })
        .await;
        if let Err(e) = r {
            tracing::debug!(error = %format!("{e:#}"), "couldn't note a conversion's use");
        }
    });
}

/// Space the cached conversions take, each stored file counted once.
pub(crate) fn conversion_cache_bytes(c: &rusqlite::Connection) -> Result<u64> {
    let n: i64 = c.query_row(
        "SELECT COALESCE(SUM(size), 0) FROM blobs
          WHERE hash IN (SELECT blob_hash FROM derivatives WHERE kind LIKE 'as:%')",
        [],
        |r| r.get(0),
    )?;
    Ok(u64::try_from(n).unwrap_or(0))
}

/// Drop the least recently used cached conversions until the rest fit in Settings → Performance's limit. Ones handed out
/// in the last hour stay, so the cache can be over for a while when it's busy. Returns (conversions, bytes) freed.
pub(crate) async fn trim_conversions(state: &crate::AppState) -> Result<(u64, u64)> {
    let limit = state.settings.get().conversion_cache_bytes();
    let doomed = crate::db::call(&state.pool, move |c| {
        let total = conversion_cache_bytes(c)?;
        if total <= limit {
            return Ok(Vec::new());
        }
        let tx = c.transaction()?;
        let rows: Vec<(String, String, String, i64)> = tx
            .prepare(
                "SELECT d.asset_id, d.kind, d.blob_hash, b.size FROM derivatives d JOIN blobs b ON b.hash = d.blob_hash
                  WHERE d.kind LIKE 'as:%' AND COALESCE(d.used_at, d.created_at) < ?1
                  ORDER BY COALESCE(d.used_at, d.created_at) ASC",
            )?
            .query_map([ago(IN_USE)], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut over = total - limit;
        let mut doomed: Vec<(String, i64)> = Vec::new();
        for (id, kind, hash, size) in rows {
            if over == 0 {
                break;
            }
            tx.execute("DELETE FROM derivatives WHERE asset_id = ?1 AND kind = ?2", [&id, &kind])?;
            if !doomed.iter().any(|(h, _)| *h == hash) {
                over = over.saturating_sub(u64::try_from(size).unwrap_or(0));
                doomed.push((hash, size));
            }
        }
        tx.commit()?;
        Ok(doomed)
    })
    .await?;
    if doomed.is_empty() {
        return Ok((0, 0));
    }
    crate::jobs::delete_unused(state, doomed).await
}

/// [`trim_conversions`], logging what it did.
pub(crate) async fn log_trim(state: &crate::AppState) {
    match trim_conversions(state).await {
        Ok((0, _)) => {}
        Ok((n, bytes)) => {
            tracing::info!(conversions = n, freed = %crate::maintenance::human_bytes(bytes), "trimmed the conversion cache");
        }
        Err(e) => tracing::warn!(error = %format!("{e:#}"), "couldn't trim the conversion cache"),
    }
}

/// The answer to "give me this asset as `format`".
pub(crate) enum Lookup {
    Ready(Ready),
    Pending(tokio::sync::watch::Receiver<Stage>),
}

/// The cached conversion of an asset, or the conversion that will make it (started now if it isn't running). Never
/// waits for the conversion itself.
pub(crate) async fn request(
    state: &crate::AppState,
    asset_id: &str,
    format: &str,
    max: Option<u32>,
) -> std::result::Result<Lookup, Refused> {
    use crate::db;
    let fail = |e: anyhow::Error| Refused::Failed(format!("{e:#}"));
    let id = asset_id.to_owned();
    let (settings, glb) = (state.settings.get(), format == "glb");
    let info = db::call(&state.pool, move |c| {
        let row: Option<(String, String, String, String, String)> = rusqlite::OptionalExtension::optional(
            c.query_row("SELECT path, ext, blob_hash, meta, pack_id FROM assets WHERE id = ?1", [&id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
            }),
        )?;
        let Some((path, ext, hash, meta, pack_id)) = row else { return Ok(None) };
        let mut stmt = c.prepare("SELECT kind, blob_hash FROM derivatives WHERE asset_id = ?1")?;
        let derivs: Vec<(String, String)> =
            stmt.query_map([&id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
        // A GLB's textures are named by the convention, so it's made again when that changes.
        let naming = if glb {
            crate::naming::ForDownload::of_pack(c, &settings.texture_names, &pack_id)?
        } else {
            crate::naming::ForDownload::default()
        };
        Ok(Some((path, ext, hash, meta, derivs, db::asset_facing(c, &id, &pack_id)?, naming)))
    })
    .await
    .map_err(fail)?
    .ok_or(Refused::NotFound)?;
    let (mut path, mut ext, mut hash, meta, derivs, facing, naming) = info;
    let file = path.rsplit('/').next().unwrap_or(&path).to_owned();
    let stem = file.rsplit_once('.').map_or(file.as_str(), |(s, _)| s).to_owned();
    let names: Vec<String> = derivs.iter().map(|(k, _)| k.clone()).collect();
    // The 3D viewer's GLB, of any model polysmelt reads or Blender converted. Not offered for download.
    let readable = crate::meshes::reads(&ext.to_ascii_lowercase());
    let view = format == "view" && max.is_none() && (readable || names.iter().any(|n| n == "glb"));
    let allowed = view
        || options(&ext, &names).iter().any(|o| o.format == format)
        || (max.is_some() && resizable_as_itself(&ext) == Some(format));
    if !allowed {
        return Err(Refused::Unsupported(tr!("error-cant-download-as", ext = ext, format = format)));
    }
    if max.is_some() && !(IMAGE_IN.contains(&ext.to_ascii_lowercase().as_str())) {
        return Err(Refused::Unsupported(tr!("error-only-images-resize")));
    }
    // A GLB download is turned the way the pack's models face and names its textures by the convention, and is made
    // again when either changes.
    let sheet = crate::spritesheet::is_format(format);
    let key = if format == "glb" {
        format!("as:glb~{}~{}~{}:0", facing.turn, facing.tilt, naming.mark())
    } else if sheet {
        // Made from the sheet, so made again when the sheet changes.
        let (_, h) = derivs.iter().find(|(k, _)| k == "sheet").ok_or(Refused::NotFound)?;
        format!("as:{format}~{}:0", &h[..h.len().min(16)])
    } else {
        format!("as:{format}:{}", max.unwrap_or(0))
    };
    // A sprite sheet is written from Hoard's own sheet of it, beside its file.
    if sheet {
        let (_, h) = derivs.iter().find(|(k, _)| k == "sheet").ok_or(Refused::NotFound)?;
        path = format!("{}.{}", path.rsplit_once('.').map_or(path.as_str(), |(s, _)| s), crate::spritesheet::EXT);
        hash.clone_from(h);
    }
    // A model only Blender reads is shown, and downloaded as a GLB, from the GLB Blender made of it.
    if (view || format == "glb") && !readable {
        let glb = derivs.iter().find(|(k, _)| k == "glb").ok_or(Refused::NotFound)?;
        path = format!("{}.glb", path.rsplit_once('.').map_or(path.as_str(), |(s, _)| s));
        ext = "glb".into();
        hash.clone_from(&glb.1);
    }
    // Cached as `as:<format>:<max>:<output ext>`: a WOFF unwraps to .ttf or .otf, a glTF zips.
    let prefix = format!("{key}:");
    if let Some((k, h)) = derivs.iter().find(|(k, _)| k.starts_with(&prefix)) {
        touch_conversion(state, asset_id, k);
        return Ok(Lookup::Ready(Ready { hash: h.clone(), ext: k[prefix.len()..].to_owned(), stem }));
    }

    let running_key = format!("{asset_id}|{key}");
    let mut running = RUNNING.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(rx) = running.get(&running_key) {
        return Ok(Lookup::Pending(rx.clone()));
    }
    let (tx, rx) = tokio::sync::watch::channel(Stage::Queued);
    running.insert(running_key.clone(), rx.clone());
    drop(running);
    let job = Job {
        state: std::sync::Arc::clone(state),
        asset_id: asset_id.to_owned(),
        path,
        ext,
        hash,
        format: format.to_owned(),
        max,
        key,
        stem,
        facing,
        naming,
        duration: serde_json::from_str::<Value>(&meta).ok().and_then(|m| m.get("duration_secs")?.as_f64()),
    };
    tokio::spawn(async move {
        let file = job.path.rsplit('/').next().unwrap_or(&job.path).to_owned();
        let size = job.max.map_or_default(|m| format!(" up to {m} px"));
        let activity = std::sync::Arc::new(crate::activity::start(
            "download_as",
            format!("Converting {file} to {}{size}", format_name(&job.format)),
            None,
        ));
        activity.queued(true);
        let result = job.run(&tx, &activity).await;
        drop(activity);
        let failed = result.is_err();
        tx.send_replace(match result {
            Ok(ready) => Stage::Done(ready),
            Err(e) => {
                tracing::warn!(key = %running_key, error = %format!("{e:#}"), "conversion failed");
                Stage::Failed(format!("{e:#}"))
            }
        });
        if failed {
            tokio::time::sleep(FAILURE_KEPT).await;
        }
        RUNNING.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&running_key);
    });
    Ok(Lookup::Pending(rx))
}

/// A format as people write it: `Opus`, `glTF`, `WebP`, `MP3`.
fn format_name(format: &str) -> String {
    match format {
        "opus" => "Opus".into(),
        "ogg" => "OGG Vorbis".into(),
        "gltf" => "glTF".into(),
        "webp" => "WebP".into(),
        "jpg" => "JPEG".into(),
        "tiff" => "TIFF".into(),
        "exr" => "OpenEXR".into(),
        "view" => "GLB for the viewer".into(),
        other => crate::spritesheet::label(other).map_or_else(|| other.to_ascii_uppercase(), str::to_owned),
    }
}

/// Wait up to `limit` for a pending conversion to finish, and say where it stands.
pub(crate) async fn wait(mut rx: tokio::sync::watch::Receiver<Stage>, limit: std::time::Duration) -> Stage {
    let finished = |s: &Stage| matches!(s, Stage::Done(_) | Stage::Failed(_));
    let done = tokio::time::timeout(limit, rx.wait_for(finished)).await.ok().and_then(|r| r.ok().map(|s| s.clone()));
    done.unwrap_or_else(|| rx.borrow().clone())
}

/// Wait up to `limit` for a pending conversion to move (progress, or finishing), and say where it stands.
pub(crate) async fn wait_for_change(mut rx: tokio::sync::watch::Receiver<Stage>, limit: std::time::Duration) -> Stage {
    let current = rx.borrow_and_update().clone();
    if matches!(current, Stage::Done(_) | Stage::Failed(_)) {
        return current;
    }
    tokio::time::timeout(limit, rx.changed()).await.ok();
    rx.borrow().clone()
}

/// One conversion, detached from whoever asked for it.
struct Job {
    state: crate::AppState,
    asset_id: String,
    path: String,
    ext: String,
    hash: String,
    format: String,
    max: Option<u32>,
    key: String,
    stem: String,
    /// How the pack's models are turned (see [`crate::meshes::facing`]).
    facing: hoard_core::Facing,
    /// How a GLB's textures are named (see [`crate::naming::for_download`]).
    naming: crate::naming::ForDownload,
    duration: Option<f64>,
}

impl Job {
    async fn run(
        &self,
        tx: &tokio::sync::watch::Sender<Stage>,
        activity: &std::sync::Arc<crate::activity::Handle>,
    ) -> Result<Ready> {
        use crate::db;
        let _slot = if self.format == "view" { VIEW_SLOTS.acquire().await? } else { SLOTS.acquire().await? };
        activity.queued(false);
        tx.send_replace(Stage::Working(0.0));
        let state = &self.state;
        let work = state.cfg.tmp_dir().join(format!("as-{}", uuid::Uuid::now_v7()));
        let result = async {
            // A .gltf, .obj or .fbx needs its buffers, materials and images beside it, at their pack-relative paths.
            let sheet = crate::spritesheet::is_format(&self.format);
            let files = if sheet {
                // The sheet, and its pages at their paths in the pack, as the library has them now.
                let pages = crate::spritesheet::pages(&state.blobs.read_all(&self.hash).await?)?;
                let me = db::BundleFile {
                    id: self.asset_id.clone(),
                    path: self.path.clone(),
                    hash: self.hash.clone(),
                    size: 0,
                    unity_guid: None,
                };
                db::call(&state.pool, move |c| {
                    let mut now = c.prepare("SELECT blob_hash FROM assets WHERE id = ?1")?;
                    let mut files = vec![me];
                    for p in pages {
                        let current: Option<String> = match &p.asset {
                            Some(id) => rusqlite::OptionalExtension::optional(now.query_row([id], |r| r.get(0)))?,
                            None => None,
                        };
                        let Some(hash) = current.or(p.hash) else { continue };
                        files.push(db::BundleFile {
                            id: p.asset.unwrap_or_default(),
                            path: p.image,
                            hash,
                            size: 0,
                            unity_guid: None,
                        });
                    }
                    Ok(files)
                })
                .await?
            } else if matches!(self.ext.to_ascii_lowercase().as_str(), "gltf" | "obj" | "fbx") {
                let id = self.asset_id.clone();
                db::call(&state.pool, move |c| {
                    let ids = db::dependency_closure(c, &id, false)?;
                    db::bundle_files(c, &ids)
                })
                .await?
            } else {
                vec![db::BundleFile {
                    id: self.asset_id.clone(),
                    path: self.path.clone(),
                    hash: self.hash.clone(),
                    size: 0,
                    unity_guid: None,
                }]
            };
            for f in &files {
                let rel = crate::ingest::safe_rel_path(&f.path).context("unsafe path in pack")?;
                let dest = work.join(rel);
                if let Some(parent) = dest.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                state.blobs.download_to(&f.hash, &dest).await?;
            }
            let local = work.join(crate::ingest::safe_rel_path(&self.path).context("unsafe path in pack")?);
            let (src, fmt, max, duration, root, facing) =
                (self.ext.clone(), self.format.clone(), self.max, self.duration, work.clone(), self.facing);
            let naming = self.naming.clone();
            let progress_tx = tx.clone();
            let shown = std::sync::Arc::clone(activity);
            let out = crate::heavy::run("convert", move || {
                // Report in whole percents, so watchers aren't woken for every ffmpeg line.
                let last = std::sync::atomic::AtomicU32::new(0);
                let report = move |p: f32| {
                    let pct = (p.clamp(0.0, 1.0) * 100.0) as u32;
                    if last.swap(pct, std::sync::atomic::Ordering::Relaxed) != pct {
                        progress_tx.send_replace(Stage::Working(p.clamp(0.0, 1.0)));
                        shown.progress(p);
                    }
                };
                if sheet {
                    return crate::spritesheet::export(&local, &root, &fmt);
                }
                crate::naming::for_download(naming, || {
                    crate::meshes::facing(facing, || {
                        convert(&local, &root, &src, &fmt, max, &Progress { duration, report: &report })
                    })
                })
            })
            .await??;
            let out_ext = out.ext;
            let blob = state.blobs.put_bytes(out.bytes, hoard_core::AssetKind::from_ext(&self.ext)).await?;
            let (id, k, h) = (self.asset_id.clone(), format!("{}:{out_ext}", self.key), blob.hash.clone());
            db::call(&state.pool, move |c| {
                let now = db::now();
                c.execute(
                    "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
                    rusqlite::params![h, blob.size as i64, now],
                )?;
                c.execute(
                    "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash",
                    rusqlite::params![id, k, h, now],
                )?;
                Ok(())
            })
            .await?;
            let st = std::sync::Arc::clone(state);
            tokio::spawn(async move { log_trim(&st).await });
            anyhow::Ok(Ready { hash: blob.hash, ext: out_ext.to_owned(), stem: self.stem.clone() })
        }
        .await;
        tokio::fs::remove_dir_all(&work).await.ok();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::Audio;

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
