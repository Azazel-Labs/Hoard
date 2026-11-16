//! A GLB made ready for the browser: its textures re-encoded as JPEG where they're opaque and PNG where they have
//! alpha, at most 2048 px a side, images in formats a browser may not draw (KTX2, WebP, AVIF behind their glTF
//! extensions) turned into those too, and the binary chunk rebuilt from only the bytes something still points at.
//! The 3D viewer is handed every model this way (see [`crate::meshes::view_glb`]), so it reads nothing but plain GLB.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use anyhow::{Context, Result, bail, ensure};
use image::DynamicImage;
use serde_json::{Value, json};

/// The longest side a texture keeps: sharper than the viewer shows it.
const MAX_SIDE: u32 = 2048;
const JPEG_QUALITY: u8 = 90;
/// Texture extensions that point at an image in a format browsers may not draw. The image is re-encoded as JPEG or
/// PNG, so the texture points at it plainly (or at the fallback beside it, already one of those).
const IMAGE_EXTENSIONS: &[&str] = &["KHR_texture_basisu", "EXT_texture_webp", "EXT_texture_avif"];

/// `glb` with web-ready textures and nothing in its binary chunk that no buffer view points at.
pub(crate) fn repack(glb: &[u8]) -> Result<Vec<u8>> {
    let (mut json, bin) = crate::formats::read_glb(glb)?;
    let doc = json.as_object_mut().context("glTF JSON is not an object")?;
    unwrap_image_extensions(doc);

    let views = doc.get("bufferViews").and_then(Value::as_array).cloned().unwrap_or_default();
    let bytes_of = |i: usize| -> Result<&[u8]> {
        let v = views.get(i).with_context(|| format!("no buffer view {i}"))?;
        ensure!(v.get("buffer").and_then(Value::as_u64).unwrap_or(0) == 0, "buffer view {i} is outside the GLB");
        let at = usize::try_from(v.get("byteOffset").and_then(Value::as_u64).unwrap_or(0))?;
        let len = usize::try_from(v.get("byteLength").and_then(Value::as_u64).context("buffer view has no length")?)?;
        bin.get(at..at + len).with_context(|| format!("buffer view {i} runs past the binary chunk"))
    };

    // Each image's bytes, re-encoded where that helps; the view holding them gets the new bytes.
    let mut replaced: HashMap<usize, Vec<u8>> = HashMap::new();
    for img in doc.get_mut("images").and_then(Value::as_array_mut).into_iter().flatten() {
        let Some(view) = img.get("bufferView").and_then(Value::as_u64) else { continue };
        let view = usize::try_from(view)?;
        if let Some((bytes, mime)) = web_image(bytes_of(view)?) {
            replaced.insert(view, bytes);
            img["mimeType"] = json!(mime);
        }
    }

    // Lay the views still pointed at end to end, each on a 4-byte boundary, and point everything at their new places.
    let mut used = BTreeSet::new();
    for (k, v) in doc.iter().filter(|(k, _)| k.as_str() != "bufferViews") {
        view_refs(k, v, &mut |i| {
            used.insert(i);
        });
    }
    let mut new_bin = Vec::with_capacity(bin.len());
    let mut new_views = Vec::with_capacity(used.len());
    let mut moved = BTreeMap::new();
    for &i in &used {
        let bytes = match replaced.get(&i) {
            Some(b) => b.as_slice(),
            None => bytes_of(i)?,
        };
        new_bin.resize(new_bin.len().next_multiple_of(4), 0);
        let mut v = views[i].clone();
        v["buffer"] = json!(0);
        v["byteOffset"] = json!(new_bin.len());
        v["byteLength"] = json!(bytes.len());
        new_bin.extend_from_slice(bytes);
        moved.insert(i, new_views.len());
        new_views.push(v);
    }
    for (k, v) in doc.iter_mut().filter(|(k, _)| k.as_str() != "bufferViews") {
        renumber_views(k, v, &moved);
    }
    if new_views.is_empty() {
        doc.remove("bufferViews");
        doc.remove("buffers");
    } else {
        doc.insert("bufferViews".into(), Value::Array(new_views));
        doc.insert("buffers".into(), json!([{ "byteLength": new_bin.len() }]));
    }
    write_glb(&json, &new_bin)
}

/// Every `bufferView` index under `v` (accessors, sparse accessors, images, extensions).
fn view_refs(key: &str, v: &Value, found: &mut dyn FnMut(usize)) {
    match v {
        Value::Number(n) if key == "bufferView" => {
            if let Some(i) = n.as_u64().and_then(|i| usize::try_from(i).ok()) {
                found(i);
            }
        }
        Value::Array(a) => a.iter().for_each(|x| view_refs(key, x, found)),
        Value::Object(o) => o.iter().for_each(|(k, x)| view_refs(k, x, found)),
        _ => {}
    }
}

fn renumber_views(key: &str, v: &mut Value, moved: &BTreeMap<usize, usize>) {
    match v {
        Value::Number(n) if key == "bufferView" => {
            if let Some(&to) = n.as_u64().and_then(|i| moved.get(&usize::try_from(i).ok()?)) {
                *v = json!(to);
            }
        }
        Value::Array(a) => a.iter_mut().for_each(|x| renumber_views(key, x, moved)),
        Value::Object(o) => o.iter_mut().for_each(|(k, x)| renumber_views(k, x, moved)),
        _ => {}
    }
}

/// Textures that point at a KTX2, WebP or AVIF image through an extension point at their fallback image when they
/// have one, else at that image (re-encoded below); images no texture points at any more are dropped.
fn unwrap_image_extensions(doc: &mut serde_json::Map<String, Value>) {
    let mut any = false;
    for tex in doc.get_mut("textures").and_then(Value::as_array_mut).into_iter().flatten() {
        let Some(exts) = tex.get_mut("extensions").and_then(Value::as_object_mut) else { continue };
        let mut source = None;
        for name in IMAGE_EXTENSIONS {
            if let Some(e) = exts.remove(*name) {
                any = true;
                source = source.or_else(|| e.get("source").cloned());
            }
        }
        if exts.is_empty() {
            tex.as_object_mut().map(|t| t.remove("extensions"));
        }
        if tex.get("source").is_none()
            && let Some(s) = source
        {
            tex["source"] = s;
        }
    }
    if !any {
        return;
    }
    for list in ["extensionsUsed", "extensionsRequired"] {
        if let Some(a) = doc.get_mut(list).and_then(Value::as_array_mut) {
            a.retain(|n| !n.as_str().is_some_and(|n| IMAGE_EXTENSIONS.contains(&n)));
            if a.is_empty() {
                doc.remove(list);
            }
        }
    }
    let shown: BTreeSet<u64> = doc
        .get("textures")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|t| t.get("source")?.as_u64())
        .collect();
    let Some(images) = doc.get_mut("images").and_then(Value::as_array_mut) else { return };
    let mut renumbered = HashMap::new();
    let mut kept = Vec::new();
    for (i, img) in std::mem::take(images).into_iter().enumerate() {
        if shown.contains(&(i as u64)) {
            renumbered.insert(i as u64, kept.len());
            kept.push(img);
        }
    }
    *images = kept;
    for tex in doc.get_mut("textures").and_then(Value::as_array_mut).into_iter().flatten() {
        if let Some(&to) = tex.get("source").and_then(Value::as_u64).and_then(|s| renumbered.get(&s)) {
            tex["source"] = json!(to);
        }
    }
}

/// An image re-encoded for the web, or `None` to keep it as it is: a JPEG, or a PNG with alpha, already no bigger
/// than [`MAX_SIDE`]; or something that can't be decoded.
fn web_image(bytes: &[u8]) -> Option<(Vec<u8>, &'static str)> {
    let format = image::guess_format(bytes).ok();
    let img = decode(bytes)?;
    let big = img.width() > MAX_SIDE || img.height() > MAX_SIDE;
    let img = if big { img.resize(MAX_SIDE, MAX_SIDE, image::imageops::FilterType::Lanczos3) } else { img };
    let alpha = img.color().has_alpha() && img.to_rgba8().pixels().any(|p| p.0[3] < 255);
    match format {
        Some(image::ImageFormat::Jpeg) if !big => return None,
        Some(image::ImageFormat::Png) if !big && alpha => return None,
        _ => {}
    }
    let mut out = Vec::new();
    if alpha {
        DynamicImage::ImageRgba8(img.to_rgba8())
            .write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
            .ok()?;
        Some((out, "image/png"))
    } else {
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, JPEG_QUALITY).encode_image(&img.to_rgb8()).ok()?;
        Some((out, "image/jpeg"))
    }
}

fn decode(bytes: &[u8]) -> Option<DynamicImage> {
    if bytes.starts_with(b"\xABKTX 20\xBB") {
        let dir = tempfile::tempdir().ok()?;
        let p = dir.path().join("t.ktx2");
        std::fs::write(&p, bytes).ok()?;
        return Some(crate::textures::decode(&p, "ktx2").ok()?.image);
    }
    image::load_from_memory(bytes).ok()
}

/// A GLB of this JSON and binary chunk.
pub(crate) fn write_glb(json: &Value, bin: &[u8]) -> Result<Vec<u8>> {
    let mut text = serde_json::to_vec(json)?;
    text.resize(text.len().next_multiple_of(4), b' ');
    let padded = bin.len().next_multiple_of(4);
    let total = 12 + 8 + text.len() + if bin.is_empty() { 0 } else { 8 + padded };
    let Ok(total32) = u32::try_from(total) else { bail!("GLB would be over 4 GiB") };
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&total32.to_le_bytes());
    out.extend_from_slice(&u32::try_from(text.len())?.to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend(text);
    if !bin.is_empty() {
        out.extend_from_slice(&u32::try_from(padded)?.to_le_bytes());
        out.extend_from_slice(b"BIN\0");
        out.extend_from_slice(bin);
        out.resize(total, 0);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(w: u32, h: u32, alpha: u8) -> Vec<u8> {
        let mut out = Vec::new();
        DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(w, h, image::Rgba([200, 90, 30, alpha])))
            .write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
            .unwrap();
        out
    }

    /// A GLB with one triangle, an unused stretch of bytes between its views, and these images.
    fn glb(images: &[Vec<u8>]) -> Vec<u8> {
        let mut bin = Vec::new();
        for c in [[0f32, 0., 0.], [1., 0., 0.], [0., 1., 0.]] {
            for x in c {
                bin.extend_from_slice(&x.to_le_bytes());
            }
        }
        let mut views = vec![json!({ "buffer": 0, "byteOffset": 0, "byteLength": 36 })];
        bin.extend_from_slice(&[7; 100]); // leftovers nothing points at
        views.push(json!({ "buffer": 0, "byteOffset": 36, "byteLength": 100 }));
        let mut imgs = Vec::new();
        for b in images {
            bin.resize(bin.len().next_multiple_of(4), 0);
            views.push(json!({ "buffer": 0, "byteOffset": bin.len(), "byteLength": b.len() }));
            imgs.push(json!({ "bufferView": views.len() - 1, "mimeType": "image/png" }));
            bin.extend_from_slice(b);
        }
        let textures: Vec<Value> = (0..images.len()).map(|i| json!({ "source": i })).collect();
        let doc = json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "byteLength": bin.len() }],
            "bufferViews": views,
            "accessors": [{ "bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3",
                            "min": [0, 0, 0], "max": [1, 1, 0] }],
            "images": imgs,
            "textures": textures,
            "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0 } }] }],
            "nodes": [{ "mesh": 0 }],
            "scenes": [{ "nodes": [0] }],
        });
        write_glb(&doc, &bin).unwrap()
    }

    fn image_bytes(json: &Value, bin: &[u8], i: usize) -> Vec<u8> {
        let v = &json["bufferViews"][json["images"][i]["bufferView"].as_u64().unwrap() as usize];
        let at = v["byteOffset"].as_u64().unwrap() as usize;
        bin[at..at + v["byteLength"].as_u64().unwrap() as usize].to_vec()
    }

    #[test]
    fn opaque_textures_become_jpeg_and_ones_with_alpha_stay_png() {
        let see_through = png(8, 8, 128);
        let out = repack(&glb(&[png(8, 8, 255), see_through.clone()])).unwrap();
        let (json, bin) = crate::formats::read_glb(&out).unwrap();
        assert_eq!(json["images"][0]["mimeType"], "image/jpeg");
        assert_eq!(image::guess_format(&image_bytes(&json, &bin, 0)).unwrap(), image::ImageFormat::Jpeg);
        assert_eq!(json["images"][1]["mimeType"], "image/png");
        assert_eq!(image_bytes(&json, &bin, 1), see_through, "a small PNG with alpha is kept as it is");
    }

    #[test]
    fn big_textures_are_scaled_down() {
        let out = repack(&glb(&[png(4096, 1024, 255), png(3000, 3000, 10)])).unwrap();
        let (json, bin) = crate::formats::read_glb(&out).unwrap();
        let first = image::load_from_memory(&image_bytes(&json, &bin, 0)).unwrap();
        assert_eq!((first.width(), first.height()), (2048, 512));
        let second = image::load_from_memory(&image_bytes(&json, &bin, 1)).unwrap();
        assert_eq!((second.width(), second.height()), (2048, 2048));
        assert_eq!(json["images"][1]["mimeType"], "image/png");
    }

    #[test]
    fn bytes_nothing_points_at_are_dropped_and_views_renumbered() {
        let out = repack(&glb(&[png(2, 2, 100)])).unwrap();
        let (json, bin) = crate::formats::read_glb(&out).unwrap();
        assert_eq!(json["bufferViews"].as_array().unwrap().len(), 2);
        assert_eq!(json["images"][0]["bufferView"], 1);
        assert_eq!(json["accessors"][0]["bufferView"], 0);
        assert!(!bin.windows(100).any(|w| w == [7; 100]), "the unused bytes are gone");
        assert_eq!(json["bufferViews"][1]["byteOffset"], 36);
        let end = 36 + image_bytes(&json, &bin, 0).len();
        assert_eq!(json["buffers"][0]["byteLength"].as_u64().unwrap() as usize, end);
    }

    #[test]
    fn webp_textures_point_at_their_fallback() {
        let fallback = png(4, 4, 255);
        let mut bytes = glb(&[fallback, png(4, 4, 255)]);
        let (mut json, bin) = crate::formats::read_glb(&bytes).unwrap();
        json["textures"] = json!([{ "source": 0, "extensions": { "EXT_texture_webp": { "source": 1 } } }]);
        json["extensionsUsed"] = json!(["EXT_texture_webp"]);
        bytes = write_glb(&json, &bin).unwrap();
        let (json, _) = crate::formats::read_glb(&repack(&bytes).unwrap()).unwrap();
        assert_eq!(json["textures"], json!([{ "source": 0 }]));
        assert_eq!(json["images"].as_array().unwrap().len(), 1, "the WebP no texture uses is dropped");
        assert!(json.get("extensionsUsed").is_none());
    }
}
