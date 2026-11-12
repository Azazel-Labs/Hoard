//! Per-file metadata extraction. Everything here is local CPU work on a file in staging.
//!
//! Each analyzer is best-effort: a file it can't parse still becomes an asset, just with less metadata.

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

use anyhow::Context as _;
use hoard_core::AssetKind;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageReader};
use serde_json::{Map, Value, json};

use crate::sources;
use hoard_unreal as unreal;

static RENDER_ENABLED: AtomicBool = AtomicBool::new(true);
/// Also render 2x2 multi-view sheets of models (only worth it when a vision LLM will look at them).
static VIEWS_ENABLED: AtomicBool = AtomicBool::new(false);
static RENDERER: OnceLock<Option<Mutex<hoard_render::Renderer>>> = OnceLock::new();

pub(crate) fn set_render_enabled(on: bool, views: bool) {
    RENDER_ENABLED.store(on, Ordering::Relaxed);
    VIEWS_ENABLED.store(views, Ordering::Relaxed);
}

/// What the renderer runs on, for `GET /api/stats`; set once the renderer has been created (or failed to be).
static RENDERER_STATUS: OnceLock<String> = OnceLock::new();
/// Why there's no GPU when one seems to be there; set with [`RENDERER_STATUS`].
static RENDERER_HINT: OnceLock<Option<String>> = OnceLock::new();

/// The shared headless renderer, created on first use. `None` if disabled or no adapter exists.
pub(crate) fn renderer() -> Option<&'static Mutex<hoard_render::Renderer>> {
    if !RENDER_ENABLED.load(Ordering::Relaxed) {
        return None;
    }
    RENDERER
        .get_or_init(|| match hoard_render::Renderer::new() {
            Ok(r) => {
                let kind = if r.is_hardware() { "gpu" } else { "cpu" };
                tracing::info!(adapter = r.adapter_name(), backend = r.backend(), kind, "model thumbnails enabled");
                if !r.is_hardware() {
                    let hint = crate::gpu_hint::why_no_gpu();
                    if let Some(h) = &hint {
                        tracing::warn!("a GPU is there but model thumbnails render in software: {h}");
                    } else {
                        tracing::info!(
                            "no GPU visible: model thumbnails render in software (see docs/DEPLOYMENT.md, GPU)"
                        );
                    }
                    let _ = RENDERER_HINT.set(hint);
                }
                let _ = RENDERER_STATUS.set(format!("{kind}: {} ({})", r.adapter_name(), r.backend()));
                Some(Mutex::new(r))
            }
            Err(e) => {
                tracing::warn!(error = %format!("{e:#}"), "model thumbnails disabled: no GPU adapter");
                let _ = RENDERER_STATUS.set(format!("unavailable: {e:#}"));
                None
            }
        })
        .as_ref()
}

/// Create the renderer in the background at startup, so the server page can say whether a GPU is in use before the
/// first model arrives.
pub(crate) fn warm_renderer() {
    if RENDER_ENABLED.load(Ordering::Relaxed) {
        std::thread::spawn(|| {
            renderer();
        });
    }
}

/// The renderer's state: `gpu: <adapter> (<api>)`, `cpu: <adapter> (<api>)` for a software rasterizer,
/// `starting`, `off`, or `unavailable: <why>`.
pub(crate) fn describe_renderer() -> String {
    if !RENDER_ENABLED.load(Ordering::Relaxed) {
        return "off".into();
    }
    RENDERER_STATUS.get().cloned().unwrap_or_else(|| "starting".into())
}

/// What's missing when a GPU seems to be there but the renderer runs in software.
pub(crate) fn renderer_hint() -> Option<String> {
    RENDERER_HINT.get().cloned().flatten()
}

/// JPEG quality for thumbnails and model views; 0 keeps them as PNG. See [`crate::settings::ThumbQuality`].
static THUMB_JPEG_QUALITY: AtomicU8 = AtomicU8::new(82);

pub(crate) fn set_thumb_quality(q: crate::settings::ThumbQuality) {
    THUMB_JPEG_QUALITY.store(q.jpeg_quality().unwrap_or(0), Ordering::Relaxed);
}

/// PNG at the strongest compression, and without the alpha channel when every pixel is opaque.
pub(crate) fn png_bytes(img: image::RgbaImage) -> anyhow::Result<Vec<u8>> {
    use image::ImageEncoder as _;
    use image::codecs::png::{CompressionType, FilterType as PngFilter, PngEncoder};
    let mut png = Vec::new();
    let enc = PngEncoder::new_with_quality(&mut png, CompressionType::Best, PngFilter::Adaptive);
    let (w, h) = img.dimensions();
    if is_opaque(&img) {
        let rgb = DynamicImage::ImageRgba8(img).to_rgb8();
        enc.write_image(rgb.as_raw(), w, h, image::ExtendedColorType::Rgb8)?;
    } else {
        enc.write_image(img.as_raw(), w, h, image::ExtendedColorType::Rgba8)?;
    }
    Ok(png)
}

fn is_opaque(img: &image::RgbaImage) -> bool {
    img.pixels().all(|p| p[3] == 255)
}

/// Encode a thumbnail or model view sheet the way Settings → Server → Thumbnails asks. PNG when it has any
/// transparency (JPEG has none), when it's pixel art (`exact`: JPEG would smear its hard edges), or when the setting
/// is Lossless. Otherwise JPEG, unless PNG is about as small anyway (flat colors, line art), since then it's exact.
pub(crate) fn thumb_bytes(img: image::RgbaImage, exact: bool) -> anyhow::Result<Vec<u8>> {
    let q = THUMB_JPEG_QUALITY.load(Ordering::Relaxed);
    if q == 0 || exact || !is_opaque(&img) {
        return png_bytes(img);
    }
    let mut jpeg = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, q)
        .encode_image(&DynamicImage::ImageRgba8(img.clone()).to_rgb8())?;
    let png = png_bytes(img)?;
    Ok(if jpeg.len() * 10 < png.len() * 8 { jpeg } else { png })
}

/// The content type of a stored thumbnail, from its first bytes.
pub(crate) fn thumb_mime(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) { "image/jpeg" } else { "image/png" }
}

/// Thumbnail, plus the multi-view sheet when enabled. A model that crashes the renderer gets a `thumb_error` in
/// its facts, which Settings → Issues lists.
fn render_model_thumb(path: &Path, a: &mut Analysis) {
    let Some(r) = renderer() else { return };
    let guard = r.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // wgpu panics on what it finds invalid; that costs this model its thumbnail, not the whole pack its ingest.
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let img = guard.render_file(path, THUMB_SIZE)?;
        let views = VIEWS_ENABLED
            .load(Ordering::Relaxed)
            .then(|| guard.render_views_file(path, THUMB_SIZE).ok().and_then(|v| thumb_bytes(v, false).ok()))
            .flatten();
        anyhow::Ok((thumb_bytes(img, false).ok(), views))
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

/// Don't try to decode images bigger than this on disk (thumbnails of 30k² PSD exports aren't worth it).
pub(crate) const MAX_IMAGE_DECODE_BYTES: u64 = 256 << 20;
const THUMB_SIZE: u32 = 256;

#[derive(Debug, Default)]
pub(crate) struct Analysis {
    pub(crate) meta: Map<String, Value>,
    pub(crate) thumb: Option<Vec<u8>>,
    /// A better display name than the file stem, when the file carries one (WAV `INAM`, iXML `FXNAME`, ID3 title).
    pub(crate) title: Option<String>,
    /// A description embedded in the file (BWF `bext`, `ICMT`, iXML `DESCRIPTION`).
    pub(crate) description: Option<String>,
    /// Multi-view render sheet for models (2x2: front, side, back, top), used for LLM vision.
    pub(crate) views: Option<Vec<u8>>,
    /// The kind, when the file says better than its extension (an Unreal package holding a `StaticMesh`).
    pub(crate) kind: Option<AssetKind>,
    /// A source file recovered from inside a container (the WAV inside an Unreal `SoundWave`), stored as the
    /// asset's `source` derivative.
    pub(crate) source: Option<unreal::Embedded>,
    /// A playable MP3 rendering of audio browsers can't play (MIDI, tracker modules), stored as the `preview`
    /// derivative.
    pub(crate) preview_mp3: Option<Vec<u8>>,
    /// A light preview of an image too big or too foreign for browsers to show as it is, stored as the `lite`
    /// derivative.
    pub(crate) lite: Option<Vec<u8>>,
}

/// Overall level of decoded audio.
#[derive(Debug, Default, Clone, Copy)]
struct Levels {
    peak: f32,
    sum_sq: f64,
    samples: u64,
    clipped: u64,
}

/// Decide the kind from extension first, then magic bytes for unknown extensions.
pub(crate) fn detect_kind(path: &Path, ext: &str) -> AssetKind {
    let by_ext = AssetKind::from_ext(ext);
    if by_ext != AssetKind::Other {
        return by_ext;
    }
    match infer::get_from_path(path).ok().flatten().map(|t| t.matcher_type()) {
        Some(infer::MatcherType::Image) => AssetKind::Texture,
        Some(infer::MatcherType::Audio) => AssetKind::Audio,
        Some(infer::MatcherType::Video) => AssetKind::Video,
        Some(infer::MatcherType::Font) => AssetKind::Font,
        Some(infer::MatcherType::Doc | infer::MatcherType::Text) => AssetKind::Document,
        _ => AssetKind::Other,
    }
}

/// `root` is the folder laid out like the pack that holds `path`, where a model's textures are looked for.
pub(crate) fn analyze(path: &Path, root: &Path, kind: AssetKind, ext: &str) -> Analysis {
    let mut a = Analysis::default();
    let result = match kind {
        _ if matches!(ext, "uasset" | "umap") => analyze_unreal(path, ext, &mut a),
        _ if ext == "json" => analyze_converted_json(path, &mut a),
        AssetKind::Texture if matches!(ext, "psd" | "psb") => analyze_psd(path, &mut a),
        AssetKind::Texture if matches!(ext, "kra" | "ora") => analyze_layered_zip(path, &mut a),
        AssetKind::Model if ext == "blend" => analyze_blend(path, &mut a),
        AssetKind::Texture if crate::textures::EXTS.contains(&ext) => analyze_decoded(path, ext, &mut a),
        AssetKind::Texture => analyze_image(path, ext, &mut a),
        AssetKind::Audio if crate::music::EXTS.contains(&ext) => analyze_music(path, ext, &mut a),
        AssetKind::Audio => analyze_sound(path, ext, &mut a),
        AssetKind::Font if crate::fonts::EXTS.contains(&ext) => crate::fonts::analyze(path, ext, THUMB_SIZE).map(|f| {
            a.meta.extend(f.facts);
            a.title = f.title;
            a.thumb = f.thumb.and_then(|t| thumb_bytes(t, false).ok());
        }),
        AssetKind::Model if matches!(ext, "stl" | "ply" | "vox") => analyze_mesh(path, root, ext, &mut a),
        AssetKind::Model | AssetKind::Animation => analyze_model(path, ext, &mut a.meta).map(|()| {
            // Animation-only glTF clips have no meshes to draw.
            let has_mesh = a.meta.get("meshes").and_then(Value::as_u64).is_none_or(|n| n > 0);
            if matches!(ext, "gltf" | "glb") && has_mesh {
                render_model_thumb(path, &mut a);
            } else if matches!(ext, "obj" | "fbx") {
                // Hoard reads OBJ (with its MTL) and binary FBX itself, finding their textures in the pack: render
                // a thumbnail from the GLB it makes.
                if let Err(e) = mesh_thumb(path, root, ext, &mut a) {
                    tracing::debug!(path = %path.display(), error = %format!("{e:#}"), "no model thumbnail");
                }
            }
        }),
        _ => Ok(()),
    };
    if let Err(e) = result {
        a.meta.insert("analysis_error".into(), json!(format!("{e:#}")));
    }
    a
}

// ---------------------------------------------------------------------------------------------
// Images

fn analyze_image(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let size = std::fs::metadata(path)?.len();
    let reader = ImageReader::open(path)?.with_guessed_format()?;
    if size > MAX_IMAGE_DECODE_BYTES {
        let (w, h) = reader.into_dimensions()?;
        a.meta.insert("width".into(), json!(w));
        a.meta.insert("height".into(), json!(h));
        return Ok(());
    }
    let img = reader.decode()?;
    let (w, h) = img.dimensions();
    a.meta.insert("width".into(), json!(w));
    a.meta.insert("height".into(), json!(h));
    a.meta.insert("color_type".into(), json!(format!("{:?}", img.color())));
    if crate::previews::wants_lite(ext, w, h) {
        a.lite = crate::previews::lite_bytes(&img).ok();
    }
    let img = if is_float(&img) {
        a.meta.insert("hdr".into(), json!(true));
        tonemap(&img.thumbnail(1024, 1024))
    } else {
        img
    };
    analyze_pixels(&img, a, true)
}

/// MIDI and tracker modules: rendered to audio for the facts a sound gets (duration, levels), a waveform and a
/// playable MP3 preview.
fn analyze_music(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let (facts, audio) = crate::music::analyze(path, ext)?;
    let secs = facts.get("duration_secs").and_then(Value::as_f64).unwrap_or(60.0);
    // Tracker modules carry a real song title; a MIDI track name ("Piano") rarely is one.
    if !matches!(ext, "mid" | "midi") {
        a.title = facts.get("title").and_then(Value::as_str).map(str::to_owned);
    }
    a.meta.extend(facts);
    let dir = tempfile::tempdir()?;
    let wav = dir.path().join("render.wav");
    std::fs::write(&wav, crate::audio::write_wav(&audio))?;
    if let Ok((png, levels)) = waveform_png(&wav, secs) {
        a.thumb = Some(png);
        let db = |v: f64| if v > 0.0 { (20.0 * v.log10() * 10.0).round() / 10.0 } else { -120.0 };
        a.meta.insert("peak_dbfs".into(), json!(db(f64::from(levels.peak))));
    }
    a.preview_mp3 = Some(preview_mp3(&wav, &audio, secs)?);
    a.meta.insert("preview".into(), json!("mp3"));
    Ok(())
}

/// Sound files: tags and properties, a waveform thumbnail, peak/RMS level and clipping. Formats Hoard can't decode
/// (WMA, CAF, AAC/M4A, Opus, …) are decoded by ffmpeg into a temporary WAV, when ffmpeg is ready.
fn analyze_sound(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let mut decoded: Option<Option<std::path::PathBuf>> = None;
    let mut via_ffmpeg = || -> Option<std::path::PathBuf> {
        decoded
            .get_or_insert_with(|| {
                let tool = crate::ffmpeg::tool()?;
                crate::ffmpeg::demuxer(ext)?;
                let wav = dir.path().join("decoded.wav");
                match crate::ffmpeg::decode_to_wav(&tool, path, ext, &wav) {
                    Ok(()) => Some(wav),
                    Err(e) => {
                        tracing::debug!(path = %path.display(), error = %format!("{e:#}"), "ffmpeg couldn't decode");
                        None
                    }
                }
            })
            .clone()
    };
    if let Err(e) = analyze_audio(path, ext, a) {
        // Tags unreadable (WMA, CAF): take duration, rate and channels from the decoded audio instead.
        let wav = via_ffmpeg().ok_or(e)?;
        analyze_audio(&wav, "wav", a)?;
        a.meta.remove("bit_depth");
        a.meta.remove("bitrate_kbps");
    }
    let secs = a.meta.get("duration_secs").and_then(Value::as_f64).unwrap_or(600.0);
    let wave = waveform_png(path, secs).or_else(|e| via_ffmpeg().map_or(Err(e), |wav| waveform_png(&wav, secs)));
    match wave {
        Ok((png, levels)) => {
            a.thumb = Some(png);
            let db = |v: f64| if v > 0.0 { (20.0 * v.log10() * 10.0).round() / 10.0 } else { -120.0 };
            a.meta.insert("peak_dbfs".into(), json!(db(f64::from(levels.peak))));
            let rms = (levels.sum_sq / levels.samples.max(1) as f64).sqrt();
            a.meta.insert("rms_dbfs".into(), json!(db(rms)));
            // More than 0.05% of samples pinned at full scale: audible clipping.
            if levels.clipped * 2000 > levels.samples {
                a.meta.insert("clipped".into(), json!(true));
            }
        }
        Err(e) => tracing::debug!(path = %path.display(), error = %e, "no waveform"),
    }
    Ok(())
}

/// The MP3 that plays a rendered song in the grid: ffmpeg's LAME when it's ready, the built-in encoder otherwise.
fn preview_mp3(wav: &Path, audio: &crate::audio::Audio, secs: f64) -> anyhow::Result<Vec<u8>> {
    if let Some(tool) = crate::ffmpeg::tool().filter(|_| crate::ffmpeg::encodes("mp3")) {
        let out = wav.with_extension("mp3");
        match crate::ffmpeg::encode_wav(&tool, wav, "mp3", &out, Some(secs), &|_| {}) {
            Ok(()) => return Ok(std::fs::read(&out)?),
            Err(e) => {
                tracing::warn!(error = %format!("{e:#}"), "ffmpeg MP3 preview failed; using the built-in encoder");
            }
        }
    }
    crate::audio::write_mp3(audio)
}

/// STL, PLY and `MagicaVoxel`: facts, and a thumbnail rendered from a GLB made on the spot.
fn analyze_mesh(path: &Path, root: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let (mesh, facts) = crate::meshes::load(path, ext, root)?;
    a.meta.extend(facts);
    let Some(mesh) = mesh else { return Ok(()) };
    render_mesh(&mesh, path, ext, a)
}

/// Thumbnail for a model Hoard converts itself (OBJ, FBX), keeping the facts the stats pass found.
fn mesh_thumb(path: &Path, root: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let (mesh, _) = crate::meshes::load(path, ext, root)?;
    match mesh {
        Some(m) => render_mesh(&m, path, ext, a),
        None => Ok(()),
    }
}

fn render_mesh(mesh: &crate::meshes::Mesh, path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let glb = crate::meshes::to_glb(mesh)?;
    let tmp = path.with_extension(format!("{ext}.hoard-preview.glb"));
    std::fs::write(&tmp, glb)?;
    render_model_thumb(&tmp, a);
    std::fs::remove_file(&tmp).ok();
    Ok(())
}

/// GPU containers, SVG and Aseprite: decoded by `textures`, then analyzed like any image.
fn analyze_decoded(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let d = crate::textures::decode(path, ext)?;
    let vector = d.facts.contains_key("vector");
    let (w, h) = match (d.facts.get("svg_width"), d.facts.get("svg_height")) {
        (Some(w), Some(h)) => (w.clone(), h.clone()),
        _ => (json!(d.image.width()), json!(d.image.height())),
    };
    a.meta.insert("width".into(), w);
    a.meta.insert("height".into(), h);
    for (k, v) in d.facts {
        if !k.starts_with("svg_") {
            a.meta.insert(k, v);
        }
    }
    if !vector && crate::previews::wants_lite(ext, d.image.width(), d.image.height()) {
        a.lite = crate::previews::lite_bytes(&d.image).ok();
    }
    let img = if is_float(&d.image) {
        a.meta.insert("hdr".into(), json!(true));
        tonemap(&d.image.thumbnail(1024, 1024))
    } else {
        d.image
    };
    // Rendered vector art isn't real pixels: skip the pixel-art and tiling tests.
    analyze_pixels(&img, a, !vector)
}

pub(crate) fn is_float(img: &DynamicImage) -> bool {
    matches!(img, DynamicImage::ImageRgb32F(_) | DynamicImage::ImageRgba32F(_))
}

/// HDR/EXR → displayable 8-bit: auto-exposure from log-average luminance, Reinhard curve, sRGB-ish gamma.
pub(crate) fn tonemap(img: &DynamicImage) -> DynamicImage {
    let f = img.to_rgba32f();
    let n = f64::from(f.width() * f.height()).max(1.0);
    let log_sum: f64 =
        f.pixels().map(|p| f64::from((0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]).max(0.0) + 1e-4).ln()).sum();
    let key = (0.18 / (log_sum / n).exp()) as f32;
    let map = |v: f32| {
        let v = (v * key).max(0.0);
        ((v / (1.0 + v)).powf(1.0 / 2.2) * 255.0).round() as u8
    };
    let out = image::RgbaImage::from_fn(f.width(), f.height(), |x, y| {
        let p = f.get_pixel(x, y);
        image::Rgba([map(p[0]), map(p[1]), map(p[2]), (p[3].clamp(0.0, 1.0) * 255.0) as u8])
    });
    DynamicImage::ImageRgba8(out)
}

/// Thumbnail and color facts from decoded pixels. `full` = these are the real pixels (not a small proxy such as
/// an embedded preview), so pixel-art and tiling tests are meaningful.
fn analyze_pixels(img: &DynamicImage, a: &mut Analysis, full: bool) -> anyhow::Result<()> {
    let (w, h) = img.dimensions();
    // Pixel art: small and few distinct colors. Upscale its thumbnail with nearest-neighbour so it stays crisp.
    let pixel_art = full && w.max(h) <= 512 && distinct_colors(img, 64).is_some_and(|n| n <= 48);
    let thumb = if pixel_art && w.max(h) < THUMB_SIZE {
        let scale = (THUMB_SIZE / w.max(h)).max(1);
        img.resize(w * scale, h * scale, FilterType::Nearest)
    } else {
        img.thumbnail(THUMB_SIZE, THUMB_SIZE)
    };
    let rgba = thumb.to_rgba8();
    a.thumb = Some(thumb_bytes(rgba.clone(), pixel_art)?);

    let has_alpha = img.color().has_alpha() && rgba.pixels().any(|p| p[3] < 250);
    a.meta.insert("alpha".into(), json!(has_alpha));
    if pixel_art {
        a.meta.insert("pixel-art-likely".into(), json!(true));
    }

    let stats = color_stats(&rgba);
    if let Some(c) = stats.dominant {
        a.meta.insert("color".into(), json!(c));
    }
    a.meta.insert("brightness".into(), json!((stats.mean_value * 100.0).round() / 100.0));
    if looks_like_normal_map(&stats) {
        a.meta.insert("normal-map-likely".into(), json!(true));
    }

    if full && !has_alpha && !pixel_art && w >= 64 && h >= 64 {
        let (tx, ty) = tileability(img);
        a.meta.insert("tileable_x".into(), json!(tx));
        a.meta.insert("tileable_y".into(), json!(ty));
        if tx && ty {
            a.meta.insert("tileable".into(), json!(true));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Source formats (readers live in `sources`)

fn analyze_psd(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    let info = sources::read_psd(path)?;
    let m = &mut a.meta;
    m.insert("width".into(), json!(info.width));
    m.insert("height".into(), json!(info.height));
    m.insert("channels".into(), json!(info.channels));
    m.insert("bit_depth".into(), json!(info.depth));
    m.insert("color_mode".into(), json!(info.color_mode));
    if info.psb {
        m.insert("large_document".into(), json!(true));
    }
    if let Some(n) = info.layers {
        m.insert("layers".into(), json!(n));
    }
    if let Some(t) = &info.thumbnail {
        return analyze_pixels(t, a, false);
    }
    a.meta.insert("note".into(), json!("saved without a preview (Photoshop: Preferences > File Handling)"));
    Ok(())
}

fn analyze_layered_zip(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    let info = sources::read_layered_zip(path)?;
    if let Some(n) = info.layers {
        a.meta.insert("layers".into(), json!(n));
    }
    if let Some(img) = &info.composite {
        a.meta.insert("width".into(), json!(img.width()));
        a.meta.insert("height".into(), json!(img.height()));
        analyze_pixels(img, a, true)?;
    }
    Ok(())
}

fn analyze_blend(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    let info = sources::read_blend(path, false)?;
    if !info.version.is_empty() {
        a.meta.insert("blender_version".into(), json!(info.version));
    }
    if let Some(c) = info.compression {
        a.meta.insert("compression".into(), json!(c));
    }
    match info.thumbnail {
        Some(t) => a.thumb = Some(thumb_bytes(t, false)?),
        None => {
            a.meta.insert("note".into(), json!("saved without a preview"));
        }
    }
    Ok(())
}

/// Count distinct RGBA colors, stopping early once `cap` is exceeded.
fn distinct_colors(img: &DynamicImage, cap: usize) -> Option<usize> {
    let rgba = img.to_rgba8();
    let mut seen = std::collections::HashSet::with_capacity(cap + 1);
    for p in rgba.pixels() {
        if p[3] == 0 {
            continue;
        }
        seen.insert(p.0);
        if seen.len() > cap {
            return None;
        }
    }
    Some(seen.len())
}

#[derive(Debug, Default)]
struct ColorStats {
    mean_rgb: [f32; 3],
    mean_value: f32,
    dominant: Option<&'static str>,
}

#[expect(clippy::many_single_char_names, reason = "r/g/b/h/s/v are the standard names in color math")]
fn color_stats(rgba: &image::RgbaImage) -> ColorStats {
    // Coarse hue buckets; "brown" is dark/desaturated orange.
    const NAMES: [&str; 8] = ["red", "orange", "yellow", "green", "cyan", "blue", "purple", "pink"];
    let mut buckets = [0u32; 9]; // last = brown
    let (mut n, mut saturated) = (0u32, 0u32);
    let mut sum = [0f32; 3];
    let mut sum_v = 0f32;
    for p in rgba.pixels() {
        if p[3] < 128 {
            continue;
        }
        n += 1;
        let [r, g, b] = [f32::from(p[0]) / 255.0, f32::from(p[1]) / 255.0, f32::from(p[2]) / 255.0];
        sum[0] += r;
        sum[1] += g;
        sum[2] += b;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let v = max;
        sum_v += v;
        let s = if max > 0.0 { (max - min) / max } else { 0.0 };
        if s < 0.25 || v < 0.2 {
            continue;
        }
        saturated += 1;
        let d = max - min;
        let mut hue = if (max - r).abs() < f32::EPSILON {
            60.0 * ((g - b) / d)
        } else if (max - g).abs() < f32::EPSILON {
            60.0 * ((b - r) / d + 2.0)
        } else {
            60.0 * ((r - g) / d + 4.0)
        };
        if hue < 0.0 {
            hue += 360.0;
        }
        let idx = match hue {
            h if !(15.0..345.0).contains(&h) => 0,
            h if h < 45.0 => {
                if v < 0.6 {
                    8
                } else {
                    1
                }
            }
            h if h < 70.0 => 2,
            h if h < 160.0 => 3,
            h if h < 200.0 => 4,
            h if h < 255.0 => 5,
            h if h < 290.0 => 6,
            _ => 7,
        };
        buckets[idx] += 1;
    }
    let mut stats = ColorStats::default();
    if n == 0 {
        return stats;
    }
    let nf = n as f32;
    stats.mean_rgb = [sum[0] / nf, sum[1] / nf, sum[2] / nf];
    stats.mean_value = sum_v / nf;
    if (saturated as f32) < nf * 0.15 {
        stats.dominant = Some(if stats.mean_value < 0.25 {
            "black"
        } else if stats.mean_value > 0.85 {
            "white"
        } else {
            "gray"
        });
    } else if let Some((idx, &count)) = buckets.iter().enumerate().max_by_key(|(_, c)| **c)
        && count as f32 > saturated as f32 * 0.4
    {
        stats.dominant = Some(if idx == 8 { "brown" } else { NAMES[idx] });
    }
    stats
}

/// Tangent-space normal maps average out to roughly (0.5, 0.5, 1.0): lavender-blue.
fn looks_like_normal_map(s: &ColorStats) -> bool {
    let [r, g, b] = s.mean_rgb;
    (0.38..0.62).contains(&r) && (0.38..0.62).contains(&g) && b > 0.72 && b > r + 0.15 && b > g + 0.15
}

/// Seamless check: if the jump from the last column back to the first is no bigger than a typical
/// column-to-column step, the texture wraps horizontally. Same for rows.
fn tileability(img: &DynamicImage) -> (bool, bool) {
    const N: u32 = 128;
    let small = img.resize_exact(N, N, FilterType::Triangle).to_luma8();
    let px = |x: u32, y: u32| f32::from(small.get_pixel(x, y)[0]);
    let col_diff = |a: u32, b: u32| (0..N).map(|y| (px(a, y) - px(b, y)).abs()).sum::<f32>() / N as f32;
    let row_diff = |a: u32, b: u32| (0..N).map(|x| (px(x, a) - px(x, b)).abs()).sum::<f32>() / N as f32;
    let base_x = (0..N - 1).map(|x| col_diff(x, x + 1)).sum::<f32>() / (N - 1) as f32;
    let base_y = (0..N - 1).map(|y| row_diff(y, y + 1)).sum::<f32>() / (N - 1) as f32;
    let edge_x = col_diff(N - 1, 0);
    let edge_y = row_diff(N - 1, 0);
    // Flat images are trivially "tileable" but tagging them as such is useless.
    let tx = base_x > 0.8 && edge_x <= base_x * 1.6;
    let ty = base_y > 0.8 && edge_y <= base_y * 1.6;
    (tx, ty)
}

// ---------------------------------------------------------------------------------------------
// Audio

/// Unreal packages: class, engine version, the editor's thumbnail, registry tags, and any source file that was
/// kept inside (a `SoundWave`'s WAV, a UE4 texture's PNG), which then gets analyzed like the real thing.
/// JSON written by Hoard's Unreal converter: a material (parameters, PBR slots) or a data table.
fn analyze_converted_json(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    if std::fs::metadata(path)?.len() > 8 << 20 {
        return Ok(());
    }
    let Ok(doc) = serde_json::from_slice::<Value>(&std::fs::read(path)?) else { return Ok(()) };
    let format = doc.get("format").and_then(Value::as_str).unwrap_or_default();
    let m = &mut a.meta;
    let keys = |k: &str| -> Vec<String> {
        doc.get(k).and_then(Value::as_object).map(|o| o.keys().cloned().collect()).unwrap_or_default()
    };
    if format.starts_with("hoard-material/") {
        a.kind = Some(AssetKind::Material);
        m.insert("pbr_slots".into(), json!(keys("pbr")));
        m.insert("texture_parameters".into(), json!(keys("textures").len()));
        if let Some(p) = doc.get("parent").and_then(Value::as_str) {
            m.insert("parent".into(), json!(p));
        }
    } else if format.starts_with("hoard-datatable/") {
        m.insert("rows".into(), json!(doc.get("rows").and_then(Value::as_object).map_or(0, Map::len)));
        if let Some(s) = doc.get("row_struct").and_then(Value::as_str) {
            m.insert("row_struct".into(), json!(s));
        }
    }
    Ok(())
}

fn analyze_unreal(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let pkg = unreal::read(path)?;
    let class = if ext == "umap" { "World".to_owned() } else { pkg.class.clone().unwrap_or_default() };
    let m = &mut a.meta;
    m.insert("ue_class".into(), json!(class));
    if !pkg.engine.is_empty() {
        m.insert("ue_engine".into(), json!(pkg.engine));
    }
    if pkg.cooked {
        m.insert("ue_cooked".into(), json!(true));
    }
    if !pkg.dependencies.is_empty() {
        m.insert("ue_dependencies".into(), json!(pkg.dependencies));
    }
    // Registry tags, minus the opaque blobs (base64 actor metadata, serialized structs).
    let mut tags = Map::new();
    for (k, v) in &pkg.tags {
        if v.is_empty() || v.len() > 240 || tags.len() >= 40 {
            continue;
        }
        tags.insert(k.clone(), json!(v));
        let num = || v.trim().parse::<f64>().ok();
        match k.as_str() {
            "Dimensions" | "ImportedSize" => {
                if let Some((w, h)) = v.split_once('x')
                    && let (Ok(w), Ok(h)) = (w.trim().parse::<u32>(), h.trim().parse::<u32>())
                {
                    m.insert("width".into(), json!(w));
                    m.insert("height".into(), json!(h));
                }
            }
            "Triangles" => {
                if let Some(n) = num() {
                    m.insert("triangles".into(), json!(n as u64));
                }
            }
            "Vertices" => {
                if let Some(n) = num() {
                    m.insert("vertices".into(), json!(n as u64));
                }
            }
            "Materials" => {
                if let Some(n) = num() {
                    m.insert("materials".into(), json!(n as u64));
                }
            }
            "Duration" | "SequenceLength" => {
                if let Some(n) = num().filter(|n| *n > 0.0) {
                    m.insert("duration_secs".into(), json!((n * 100.0).round() / 100.0));
                }
            }
            "NumChannels" | "Channels" => {
                if let Some(n) = num() {
                    m.insert("channels".into(), json!(n as u64));
                }
            }
            "SampleRate" => {
                if let Some(n) = num() {
                    m.insert("sample_rate".into(), json!(n as u64));
                }
            }
            _ => {}
        }
    }
    if !tags.is_empty() {
        m.insert("ue_tags".into(), Value::Object(tags));
    }
    if class.contains("SkeletalMesh") {
        m.insert("skins".into(), json!(1));
    }
    a.kind = Some(unreal::kind_of(&class));

    if let Some(thumb) = &pkg.thumbnail
        && let Ok(img) = image::load_from_memory(thumb)
    {
        let img = img.thumbnail(THUMB_SIZE, THUMB_SIZE).to_rgba8();
        a.thumb = thumb_bytes(img, false).ok();
    }

    // The original file, if the package kept it.
    let size = std::fs::metadata(path).map_or(0, |m| m.len());
    if !pkg.cooked && size <= unreal::MAX_CARVE && (class == "SoundWave" || class.starts_with("Texture")) {
        let data = std::fs::read(path)?;
        if let Some(src) = unreal::carve_source(&data, &class, usize::try_from(pkg.header_size)?) {
            let tmp = tempfile::Builder::new().suffix(&format!(".{}", src.ext)).tempfile()?;
            std::fs::write(tmp.path(), &src.bytes)?;
            let kind = if src.ext == "wav" { AssetKind::Audio } else { AssetKind::Texture };
            let inner = analyze(tmp.path(), tmp.path().parent().unwrap_or(tmp.path()), kind, src.ext);
            // Measured facts from the real file win over registry guesses; so does its thumbnail.
            for (k, v) in inner.meta {
                a.meta.insert(k, v);
            }
            if inner.thumb.is_some() {
                a.thumb = inner.thumb;
            }
            a.title = inner.title.or(a.title.take());
            a.description = inner.description.or(a.description.take());
            a.meta.insert("source_ext".into(), json!(src.ext));
            a.meta.insert("source_bytes".into(), json!(src.bytes.len()));
            a.source = Some(src);
        }
    }
    Ok(())
}

fn analyze_audio(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    use lofty::prelude::*;

    // Library metadata inside WAVs (description, keywords, UCS, loops, tempo). Read first: lofty may reject odd
    // but valid files, and this is the richer source.
    if matches!(ext, "wav" | "wave" | "bwf")
        && let Ok(r) = sources::read_riff(path)
    {
        let m = &mut a.meta;
        if let Some((start, end)) = r.loop_points {
            m.insert("loop_start".into(), json!(start));
            m.insert("loop_end".into(), json!(end));
        }
        if let Some(one_shot) = r.acid_one_shot {
            m.insert("one_shot".into(), json!(one_shot));
        }
        if let Some(t) = r.tempo {
            m.insert("tempo_bpm".into(), json!((t * 10.0).round() / 10.0));
        }
        if let Some(n) = r.root_note {
            m.insert("root_note".into(), json!(sources::note_name(n)));
        }
        if r.cue_points > 0 {
            m.insert("cue_points".into(), json!(r.cue_points));
        }
        if !r.keywords.is_empty() {
            m.insert("keywords".into(), json!(r.keywords));
        }
        for (k, v) in [
            ("ucs_category", r.ucs_category),
            ("ucs_subcategory", r.ucs_subcategory),
            ("ucs_catid", r.ucs_catid),
            ("library", r.library),
            ("genre", r.genre),
            ("artist", r.artist),
        ] {
            if let Some(v) = v {
                m.insert(k.into(), json!(v));
            }
        }
        a.title = r.title.filter(|t| t.chars().count() <= 100);
        a.description = r.description.filter(|d| d.chars().count() >= 3);
    }
    let meta = &mut a.meta;

    let tagged = lofty::read_from_path(path)?;
    let props = tagged.properties();
    let secs = props.duration().as_secs_f64();
    meta.insert("duration_secs".into(), json!((secs * 1000.0).round() / 1000.0));
    if let Some(sr) = props.sample_rate() {
        meta.insert("sample_rate".into(), json!(sr));
    }
    if let Some(ch) = props.channels() {
        meta.insert("channels".into(), json!(ch));
    }
    if let Some(bits) = props.bit_depth() {
        meta.insert("bit_depth".into(), json!(bits));
    }
    if let Some(kbps) = props.audio_bitrate() {
        meta.insert("bitrate_kbps".into(), json!(kbps));
    }
    if let Some(tag) = tagged.primary_tag().or_else(|| tagged.first_tag()) {
        for (k, v) in [("title", tag.title()), ("artist", tag.artist()), ("album", tag.album()), ("genre", tag.genre())]
        {
            if let Some(v) = v.filter(|v| !v.trim().is_empty()) {
                meta.insert(k.into(), json!(v.trim()));
            }
        }
    }
    if a.title.is_none() {
        a.title = a.meta.get("title").and_then(Value::as_str).filter(|t| t.chars().count() <= 100).map(str::to_owned);
    }
    Ok(())
}

/// Decode audio and draw its peak envelope as a transparent PNG, so a library of SFX can be scanned by eye
/// (a sharp hit and a long swell look nothing alike). Decoding stops after `MAX_WAVEFORM_SECS`.
fn waveform_png(path: &Path, duration_secs: f64) -> anyhow::Result<(Vec<u8>, Levels)> {
    use symphonia::core::codecs::audio::AudioDecoderOptions;
    use symphonia::core::formats::probe::Hint;
    use symphonia::core::formats::{FormatOptions, TrackType};
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;

    const MAX_WAVEFORM_SECS: f64 = 15.0 * 60.0;

    let mss =
        MediaSourceStream::new(Box::new(File::open(path)?), symphonia::core::io::MediaSourceStreamOptions::default());
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }
    let mut format =
        symphonia::default::get_probe().probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())?;
    let track = format.default_track(TrackType::Audio).context("no audio track")?;
    let params = track.codec_params.as_ref().and_then(|p| p.audio()).context("no audio codec parameters")?;
    let mut decoder = symphonia::default::get_codecs().make_audio_decoder(params, &AudioDecoderOptions::default())?;
    let track_id = track.id;

    let mut bars = vec![Bar::default(); WAVE_BARS];
    let mut last_sign = 0i8;
    let mut frames_per_col = 0usize;
    let mut frame_idx = 0usize;
    let mut scratch: Vec<f32> = Vec::new();
    let mut levels = Levels::default();
    while let Some(packet) = format.next_packet()? {
        if packet.track_id != track_id {
            continue;
        }
        let buf = match decoder.decode(&packet) {
            Ok(b) => b,
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(e.into()),
        };
        let rate = buf.spec().rate().max(1) as usize;
        let channels = buf.spec().channels().count().max(1);
        if frames_per_col == 0 {
            let total = (duration_secs.min(MAX_WAVEFORM_SECS) * rate as f64).ceil() as usize;
            frames_per_col = (total / WAVE_BARS).max(1);
        }
        scratch.resize(buf.samples_interleaved(), 0.0);
        buf.copy_to_slice_interleaved(&mut scratch);
        for frame in scratch.chunks_exact(channels) {
            let col = frame_idx / frames_per_col;
            if col >= WAVE_BARS {
                break;
            }
            let peak = frame.iter().fold(0f32, |m, s| m.max(s.abs()));
            let mono = frame.iter().sum::<f32>() / channels as f32;
            let bar = &mut bars[col];
            bar.peak = bar.peak.max(peak);
            bar.sum_sq += f64::from(mono) * f64::from(mono);
            bar.frames += 1;
            // Zero crossings (ignoring near-silence) say how bright the sound is: a cheap spectral centroid.
            let sign = if mono > 1e-3 {
                1
            } else if mono < -1e-3 {
                -1
            } else {
                0
            };
            if sign != 0 {
                if last_sign != 0 && sign != last_sign {
                    bar.crossings += 1;
                }
                last_sign = sign;
            }
            for s in frame {
                levels.sum_sq += f64::from(*s) * f64::from(*s);
                if s.abs() >= 0.999 {
                    levels.clipped += 1;
                }
            }
            levels.samples += frame.len() as u64;
            levels.peak = levels.peak.max(peak);
            frame_idx += 1;
        }
        if frame_idx / frames_per_col >= WAVE_BARS {
            break;
        }
    }
    anyhow::ensure!(frame_idx > 0, "no audio decoded");
    let used = frame_idx.div_ceil(frames_per_col).clamp(1, WAVE_BARS);
    bars.truncate(used);
    Ok((draw_waveform(&bars)?, levels))
}

/// Bars across a waveform thumbnail (fewer when the sound is too short to fill them).
const WAVE_BARS: usize = 32;

/// One bar of a waveform: loudest sample, energy, and how often the signal crossed zero.
#[derive(Clone, Copy, Debug, Default)]
struct Bar {
    peak: f32,
    sum_sq: f64,
    frames: u64,
    crossings: u64,
}

/// Colors from dull to bright sound (by zero-crossing rate): azure lows, violet and orchid mids, orange highs.
/// Mid-luminance, so they read on the portal's light and dark themes alike.
const WAVE_STOPS: [(f32, [f32; 3]); 4] = [
    (0.0, [79.0, 124.0, 255.0]),
    (0.38, [155.0, 108.0, 255.0]),
    (0.68, [224.0, 111.0, 216.0]),
    (1.0, [244.0, 128.0, 66.0]),
];

fn wave_color(brightness: f32) -> [f32; 3] {
    let at = brightness.clamp(0.0, 1.0);
    let next = WAVE_STOPS.iter().position(|(stop, _)| *stop >= at).unwrap_or(WAVE_STOPS.len() - 1).max(1);
    let ((from, low), (to, high)) = (WAVE_STOPS[next - 1], WAVE_STOPS[next]);
    let mix = ((at - from) / (to - from).max(1e-6)).clamp(0.0, 1.0);
    [0, 1, 2].map(|k| low[k] + (high[k] - low[k]) * mix)
}

/// Mirrored, rounded bars: a solid core at the bar's RMS loudness and a translucent halo out to its peak, each bar
/// tinted by how bright it sounds. Heights are normalized and gently compressed so quiet sounds still show a shape.
fn draw_waveform(bars: &[Bar]) -> anyhow::Result<Vec<u8>> {
    const W: u32 = 512;
    const H: u32 = 288;
    const PITCH: f32 = W as f32 / WAVE_BARS as f32;
    const BAR_W: f32 = PITCH * 0.6;
    const SUB: u32 = 4;
    let max = bars.iter().map(|b| b.peak).fold(0f32, f32::max).max(1e-4);
    let mid = H as f32 / 2.0;
    let reach = mid - 3.0;
    let shape = |v: f32| (v / max).clamp(0.0, 1.0).powf(0.7);
    // Brightness per bar. Near-silence crosses zero rarely whatever it is, so quiet bars (a decaying tail) keep the
    // color of the last bar loud enough to judge.
    let rms_of = |b: &Bar| (b.sum_sq / b.frames.max(1) as f64).sqrt() as f32;
    let loudest = bars.iter().map(rms_of).fold(0f32, f32::max).max(1e-6);
    let judged: Vec<Option<f32>> = bars
        .iter()
        .map(|b| {
            let rate = b.crossings as f32 / b.frames.max(1) as f32;
            // 0.003 (≈70 Hz at 44.1 kHz) to 0.3 (hiss), on a log scale.
            (rms_of(b) >= loudest * 0.08 && rate > 0.0).then(|| f32::midpoint(rate.log10(), 2.523))
        })
        .collect();
    let mut carried = judged.iter().flatten().next().copied().unwrap_or(0.0);
    let brightness: Vec<f32> = judged
        .iter()
        .map(|j| {
            carried = j.unwrap_or(carried);
            carried
        })
        .collect();
    let mut img = image::RgbaImage::new(W, H);
    // Center the bars when there are fewer than fill the width.
    let x_start = (W as f32 - PITCH * bars.len() as f32) / 2.0;
    for (i, bar) in bars.iter().enumerate() {
        let rms = rms_of(bar);
        let [r, g, b] = wave_color(brightness[i]);
        let halo_h = (shape(bar.peak) * reach).max(1.5);
        let core_h = (shape(rms * std::f32::consts::SQRT_2) * reach).clamp(1.5, halo_h);
        let x0 = x_start + i as f32 * PITCH + (PITCH - BAR_W) / 2.0;
        let x1 = x0 + BAR_W;
        let radius = BAR_W / 2.0;
        // Coverage of a vertical capsule from mid-h to mid+h, sampled on a SUB×SUB grid per pixel.
        let covered = |px: f32, py: f32, h: f32| -> bool {
            if px < x0 || px > x1 {
                return false;
            }
            let (top, bottom) = (mid - h, mid + h);
            let cx = f32::midpoint(x0, x1);
            let cy = py.clamp(top + radius.min(h), bottom - radius.min(h));
            let (dx, dy) = (px - cx, py - cy);
            dx * dx + dy * dy <= radius * radius
        };
        for x in (x0.floor().max(0.0) as u32)..(x1.ceil() as u32).min(W) {
            for y in ((mid - halo_h).floor().max(0.0) as u32)..((mid + halo_h).ceil() as u32).min(H) {
                let (mut halo, mut core) = (0u32, 0u32);
                for sy in 0..SUB {
                    for sx in 0..SUB {
                        let px = x as f32 + (sx as f32 + 0.5) / SUB as f32;
                        let py = y as f32 + (sy as f32 + 0.5) / SUB as f32;
                        halo += u32::from(covered(px, py, halo_h));
                        core += u32::from(covered(px, py, core_h));
                    }
                }
                let n = (SUB * SUB) as f32;
                let (halo, core) = (halo as f32 / n * 0.38, core as f32 / n);
                // The core over the halo, both the bar's color.
                let alpha = core + halo * (1.0 - core);
                if alpha > 0.0 {
                    // A little lighter toward the tips, for some depth.
                    let lift = ((y as f32 + 0.5 - mid).abs() / reach).clamp(0.0, 1.0) * 0.15;
                    let tint = |c: f32| (c + (255.0 - c) * lift).round() as u8;
                    img.put_pixel(x, y, image::Rgba([tint(r), tint(g), tint(b), (alpha * 255.0).round() as u8]));
                }
            }
        }
    }
    png_bytes(img)
}

// ---------------------------------------------------------------------------------------------
// Models

fn analyze_model(path: &Path, ext: &str, meta: &mut Map<String, Value>) -> anyhow::Result<()> {
    match ext {
        "gltf" => {
            let json: Value = serde_json::from_reader(BufReader::new(File::open(path)?))?;
            gltf_stats(&json, meta);
        }
        "glb" => {
            let json = glb_json(path)?;
            gltf_stats(&json, meta);
        }
        "obj" => obj_stats(path, meta)?,
        "fbx" => fbx_stats(path, meta)?,
        _ => {}
    }
    Ok(())
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
    fn thumbnails_are_jpeg_only_when_opaque_and_not_pixel_art() {
        // Like a photo or a rendered texture: smooth shading with grain, which is what JPEG is for.
        let mut seed = 0x2545_f491_u32;
        let photo = image::RgbaImage::from_fn(256, 256, |x, y| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let grain = (seed % 24) as f32 - 12.0;
            let (fx, fy) = (x as f32 / 40.0, y as f32 / 33.0);
            let c = |phase: f32| (128.0 + 90.0 * (fx + phase).sin() * fy.cos() + grain).clamp(0.0, 255.0) as u8;
            image::Rgba([c(0.0), c(1.3), c(2.6), 255])
        });
        let jpeg = thumb_bytes(photo.clone(), false).unwrap();
        assert_eq!(thumb_mime(&jpeg), "image/jpeg");
        let png = thumb_bytes(photo.clone(), true).unwrap();
        assert_eq!(thumb_mime(&png), "image/png", "pixel art stays PNG");
        assert!(jpeg.len() * 3 < png.len(), "JPEG {} vs PNG {}", jpeg.len(), png.len());

        // Flat colors: PNG is as small and exact.
        let flat =
            image::RgbaImage::from_fn(256, 256, |x, _| image::Rgba([if x < 128 { 200 } else { 30 }, 90, 60, 255]));
        assert_eq!(thumb_mime(&thumb_bytes(flat, false).unwrap()), "image/png");

        let mut cutout = photo;
        cutout.put_pixel(0, 0, image::Rgba([0, 0, 0, 0]));
        let kept = thumb_bytes(cutout, false).unwrap();
        assert_eq!(thumb_mime(&kept), "image/png", "transparency stays PNG");
        assert!(image::load_from_memory(&kept).unwrap().color().has_alpha());
    }

    fn wav_of(samples: &[f32], rate: u32) -> std::path::PathBuf {
        let audio = crate::audio::Audio {
            samples: samples.iter().map(|s| (s * 32767.0) as i32).collect(),
            channels: 1,
            rate,
            bits: 16,
        };
        let p = std::env::temp_dir().join(format!("hoard-wave-{}.wav", uuid::Uuid::now_v7()));
        std::fs::write(&p, crate::audio::write_wav(&audio)).unwrap();
        p
    }

    /// Mean color of the opaque pixels in a waveform PNG.
    fn mean_color(png: &[u8]) -> [f32; 3] {
        let img = image::load_from_memory(png).unwrap().to_rgba8();
        let (mut sum, mut n) = ([0f32; 3], 0f32);
        for p in img.pixels().filter(|p| p[3] > 200) {
            for k in 0..3 {
                sum[k] += f32::from(p[k]);
            }
            n += 1.0;
        }
        sum.map(|v| v / n.max(1.0))
    }

    fn test_sounds() -> Vec<(&'static str, Vec<f32>)> {
        let rate = 44_100.0;
        let mut seed = 1u32;
        let mut noise = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / (1 << 23) as f32 - 1.0
        };
        let n = 44_100usize;
        vec![
            ("bass", (0..n).map(|i| (i as f32 * 70.0 * std::f32::consts::TAU / rate).sin() * 0.8).collect()),
            ("hihat", (0..n / 2).map(|i| noise() * (-(i as f32) / 2500.0).exp() * 0.7).collect()),
            (
                "sweep",
                (0..n * 2)
                    .map(|i| {
                        let t = i as f32 / rate;
                        (std::f32::consts::TAU * (60.0 * t + 2400.0 * t * t)).sin() * (0.3 + 0.35 * t)
                    })
                    .collect(),
            ),
            (
                "kick",
                (0..n)
                    .map(|i| {
                        let t = i as f32 / rate;
                        (std::f32::consts::TAU * (50.0 * t + 80.0 * (1.0 - (-t * 30.0).exp()) / 30.0)).sin()
                            * (-t * 6.0).exp()
                    })
                    .collect(),
            ),
        ]
    }

    #[test]
    fn waveform_color_follows_brightness() {
        let sounds = test_sounds();
        let render = |name: &str| {
            let (_, s) = sounds.iter().find(|(n, _)| *n == name).unwrap();
            let p = wav_of(s, 44_100);
            let (png, levels) = waveform_png(&p, s.len() as f64 / 44_100.0).unwrap();
            std::fs::remove_file(&p).ok();
            assert!(levels.peak > 0.5);
            mean_color(&png)
        };
        let (bass, hiss) = (render("bass"), render("hihat"));
        // Bass leans blue, hiss leans warm.
        assert!(bass[2] > bass[0], "{bass:?}");
        assert!(hiss[0] > hiss[2], "{hiss:?}");
    }

    #[test]
    #[ignore = "writes sample waveforms to /tmp/hoard-waves for a look"]
    fn write_sample_waveforms() {
        let out = Path::new("/tmp/hoard-waves");
        std::fs::create_dir_all(out).unwrap();
        for (name, s) in test_sounds() {
            let p = wav_of(&s, 44_100);
            let (png, _) = waveform_png(&p, s.len() as f64 / 44_100.0).unwrap();
            std::fs::write(out.join(format!("{name}.png")), png).unwrap();
            std::fs::remove_file(&p).ok();
        }
    }

    #[test]
    fn exr_gets_a_tonemapped_thumbnail() {
        // A bright HDR gradient (values up to 50.0) must come out visible, not clipped to white or black.
        let img = image::Rgb32FImage::from_fn(64, 32, |x, _| {
            let v = (x as f32 / 63.0) * 50.0;
            image::Rgb([v, v * 0.5, 0.1])
        });
        let dir = std::env::temp_dir().join("hoard-exr-test");
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("sky.exr");
        DynamicImage::ImageRgb32F(img).save(&p).unwrap();
        let a = analyze(&p, &dir, AssetKind::Texture, "exr");
        assert_eq!(a.meta.get("hdr"), Some(&json!(true)));
        assert_eq!(a.meta.get("width"), Some(&json!(64)));
        let thumb = image::load_from_memory(&a.thumb.expect("thumbnail")).unwrap().to_rgba8();
        let (dark, bright) = (thumb.get_pixel(1, 1)[0], thumb.get_pixel(thumb.width() - 2, 1)[0]);
        assert!(dark < bright && bright < 255 && bright > 100, "dark={dark} bright={bright}");
    }
}
