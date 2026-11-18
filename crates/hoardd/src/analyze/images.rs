//! Pictures: their size, frames and colors, whether they tile or are a normal map, and a thumbnail, tonemapped
//! from HDR and EXR.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageReader};
use serde_json::json;

use super::{Analysis, MAX_IMAGE_DECODE_BYTES, THUMB_SIZE, thumb_bytes};

pub(super) fn analyze_image(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
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
    if let Some((frames, ms)) = animation(path, ext) {
        a.meta.insert("frames".into(), json!(frames));
        a.meta.insert("duration_ms".into(), json!(ms));
    }
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

/// Most frames counted in an animated picture.
const MAX_FRAMES: usize = 10_000;

/// An animated GIF, PNG or WebP's frames and how long it takes to play through once, in ms. None for a still.
fn animation(path: &Path, ext: &str) -> Option<(usize, u64)> {
    use image::AnimationDecoder as _;
    use image::codecs::{gif::GifDecoder, png::PngDecoder, webp::WebPDecoder};
    let file = BufReader::new(File::open(path).ok()?);
    let frames: image::Frames<'static> = match ext {
        "gif" => GifDecoder::new(file).ok()?.into_frames(),
        "png" | "apng" => {
            let d = PngDecoder::new(file).ok()?;
            if !d.is_apng().ok()? {
                return None;
            }
            d.apng().ok()?.into_frames()
        }
        "webp" => {
            let d = WebPDecoder::new(file).ok()?;
            if !d.has_animation() {
                return None;
            }
            d.into_frames()
        }
        _ => return None,
    };
    let (mut n, mut ms) = (0usize, 0u64);
    for f in frames.take(MAX_FRAMES) {
        let Ok(f) = f else { break };
        let (num, den) = f.delay().numer_denom_ms();
        ms += u64::from(num) / u64::from(den.max(1));
        n += 1;
    }
    (n > 1).then_some((n, ms))
}

/// GPU containers, vector art, Aseprite, and the pictures kept inside source files and camera raws: decoded by
/// `textures`, then analyzed like any image.
pub(super) fn analyze_decoded(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let d = crate::textures::decode(path, ext)?;
    // Rendered vector art and a file's own small picture of itself aren't real pixels.
    let drawn = d.facts.contains_key("vector") || d.facts.contains_key("embedded_preview");
    let (w, h) = match (d.facts.get("vector_width"), d.facts.get("vector_height")) {
        (Some(w), Some(h)) => (w.clone(), h.clone()),
        _ => (json!(d.image.width()), json!(d.image.height())),
    };
    a.meta.insert("width".into(), w);
    a.meta.insert("height".into(), h);
    for (k, v) in d.facts {
        if !k.starts_with("vector_") {
            a.meta.insert(k, v);
        }
    }
    if crate::previews::wants_lite(ext, d.image.width(), d.image.height()) {
        a.lite = crate::previews::lite_bytes(&d.image).ok();
    }
    let img = if is_float(&d.image) {
        a.meta.insert("hdr".into(), json!(true));
        tonemap(&d.image.thumbnail(1024, 1024))
    } else {
        d.image
    };
    // Skip the pixel-art and tiling tests for what isn't real pixels.
    analyze_pixels(&img, a, !drawn)
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
pub(super) fn analyze_pixels(img: &DynamicImage, a: &mut Analysis, full: bool) -> anyhow::Result<()> {
    let (w, h) = img.dimensions();
    // Pixel art: small and few distinct colors. Upscale its thumbnail with nearest-neighbor so it stays crisp.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyze::analyze;
    use hoard_core::AssetKind;

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

    #[test]
    fn an_animated_gif_says_how_many_frames_and_how_long() {
        use image::codecs::gif::GifEncoder;
        use image::{Delay, Frame, RgbaImage};
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spin.gif");
        let mut out = Vec::new();
        {
            let mut enc = GifEncoder::new(&mut out);
            let frames = (0..4u8).map(|i| {
                let img = RgbaImage::from_pixel(8, 8, image::Rgba([i * 60, 0, 0, 255]));
                Frame::from_parts(img, 0, 0, Delay::from_numer_denom_ms(80, 1))
            });
            enc.encode_frames(frames).unwrap();
        }
        std::fs::write(&path, &out).unwrap();
        assert_eq!(animation(&path, "gif"), Some((4, 320)));
        let a = analyze(&path, dir.path(), AssetKind::Texture, "gif");
        assert_eq!(a.meta.get("frames"), Some(&json!(4)));

        let still = dir.path().join("still.png");
        RgbaImage::from_pixel(8, 8, image::Rgba([0, 0, 0, 255])).save(&still).unwrap();
        assert_eq!(animation(&still, "png"), None);
    }
}
