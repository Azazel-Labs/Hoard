//! Per-file metadata extraction. Everything here is local CPU work on a file in staging.
//!
//! Each analyzer is best-effort: a file it can't parse still becomes an asset, just with less metadata.

mod apps;
mod images;
mod models;
mod sounds;

use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

use hoard_core::AssetKind;
use image::DynamicImage;
use serde_json::{Map, Value, json};

use apps::{
    analyze_blend, analyze_converted_json, analyze_dotlottie, analyze_layered_zip, analyze_psd, analyze_rive,
    analyze_unreal,
};
use hoard_unreal as unreal;
use images::{analyze_decoded, analyze_image};
pub(crate) use images::{is_float, tonemap};
use models::{analyze_mesh, analyze_model, mesh_thumb, render_model_thumb};
pub(crate) use models::{gltf_thumb, gltf_thumb_dressed, mesh_thumb_bytes};
use sounds::{analyze_music, analyze_sound};

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

/// Encode a thumbnail or model view sheet the way Settings → Performance → Thumbnails asks. PNG when it has any
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

/// Don't try to decode images bigger than this on disk (thumbnails of 30k² PSD exports aren't worth it).
pub(crate) const MAX_IMAGE_DECODE_BYTES: u64 = 256 << 20;
pub(crate) const THUMB_SIZE: u32 = 256;

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
    /// A model's skeleton and clips, kept in `rigs`.
    pub(crate) rig: Option<crate::animations::Rig>,
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
        AssetKind::Document if ext == "pdf" => crate::textures::pdf_thumbnail(path, THUMB_SIZE).map(|page| {
            a.thumb = page.and_then(|p| thumb_bytes(p, false).ok());
        }),
        AssetKind::Texture if crate::textures::handles(ext) => analyze_decoded(path, ext, &mut a),
        AssetKind::Texture => analyze_image(path, ext, &mut a),
        AssetKind::Vector if ext == "lottie" => analyze_dotlottie(path, &mut a),
        AssetKind::Vector if ext == "riv" => analyze_rive(path, &mut a),
        AssetKind::Vector if crate::textures::handles(ext) => analyze_decoded(path, ext, &mut a),
        AssetKind::Audio if crate::music::EXTS.contains(&ext) || crate::instruments::EXTS.contains(&ext) => {
            analyze_music(path, ext, &mut a)
        }
        AssetKind::Audio => analyze_sound(path, ext, &mut a),
        AssetKind::Font if crate::fonts::EXTS.contains(&ext) => crate::fonts::analyze(path, ext, THUMB_SIZE).map(|f| {
            a.meta.extend(f.facts);
            a.title = f.title;
            a.thumb = f.thumb.and_then(|t| thumb_bytes(t, false).ok());
        }),
        AssetKind::Model
            if matches!(ext, "stl" | "ply" | "vox" | "3ds" | "3mf" | "usd" | "usda" | "usdc" | "usdz" | "skp") =>
        {
            analyze_mesh(path, root, ext, &mut a)
        }
        AssetKind::Model | AssetKind::Animation => analyze_model(path, root, ext, &mut a.meta).map(|rig| {
            a.rig = rig;
            // Animation-only glTF clips have no meshes to draw.
            let has_mesh = a.meta.get("meshes").and_then(Value::as_u64).is_none_or(|n| n > 0);
            if matches!(ext, "gltf" | "glb") && has_mesh {
                render_model_thumb(path, &mut a);
            } else if matches!(ext, "obj" | "fbx") {
                // Hoard reads OBJ (with its MTL) and FBX itself, finding their textures in the pack: render
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
    // Text with no picture of its own (code, data, prose): drawn as a silhouette of its lines.
    let text_kind =
        matches!(a.kind.unwrap_or(kind), AssetKind::Document | AssetKind::Data | AssetKind::Shader | AssetKind::Other);
    if a.thumb.is_none() && text_kind && crate::text_thumbs::handles(ext) {
        match crate::text_thumbs::thumbnail(path, ext, THUMB_SIZE) {
            Ok(drawn) => a.thumb = drawn.and_then(|img| thumb_bytes(img, true).ok()),
            Err(e) => tracing::debug!(path = %path.display(), error = %format!("{e:#}"), "no text thumbnail"),
        }
    }
    if matches!(kind, AssetKind::Texture | AssetKind::Vector) {
        a.meta.extend(crate::provenance::scan(path));
    }
    a
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
}
