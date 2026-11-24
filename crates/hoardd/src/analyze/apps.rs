//! Other tools' own files: Photoshop, Krita and OpenRaster documents and Blender scenes (their readers live in
//! `sources`), Unreal packages, Lottie and Rive animations, and the JSON Hoard's Unreal converter writes.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use hoard_core::AssetKind;
use serde_json::{Map, Value, json};

use super::images::analyze_pixels;
use super::{Analysis, analyze, thumb_bytes, thumb_size};
use crate::sources;
use hoard_unreal as unreal;

pub(super) fn analyze_psd(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
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

pub(super) fn analyze_layered_zip(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
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

pub(super) fn analyze_blend(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    let info = sources::read_blend(path, false)?;
    if !info.version.is_empty() {
        a.meta.insert("blender_version".into(), json!(info.version));
    }
    if let Some(c) = info.compression {
        a.meta.insert("compression".into(), json!(c));
    }
    match info.thumbnail {
        Some(t) => a.thumb = Some(thumb_bytes(&t, false)?),
        None => {
            a.meta.insert("note".into(), json!("saved without a preview"));
        }
    }
    Ok(())
}

/// JSON written by Hoard's Unreal converter (a material's parameters and PBR slots, or a data table), or a Lottie
/// animation, a vector drawing that plays.
pub(super) fn analyze_converted_json(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    if std::fs::metadata(path)?.len() > 8 << 20 {
        return Ok(());
    }
    let Ok(doc) = serde_json::from_slice::<Value>(&std::fs::read(path)?) else { return Ok(()) };
    let format = doc.get("format").and_then(Value::as_str).unwrap_or_default();
    let m = &mut a.meta;
    let keys = |k: &str| -> Vec<String> {
        doc.get(k).and_then(Value::as_object).map_or_default(|o| o.keys().cloned().collect())
    };
    if format.starts_with("hoard-material/") {
        a.kind = Some(AssetKind::Material);
        m.insert("pbr_slots".into(), json!(keys("pbr")));
        m.insert("texture_parameters".into(), json!(keys("textures").len()));
        if let Some(p) = doc.get("parent").and_then(Value::as_str) {
            m.insert("parent".into(), json!(p));
        }
    } else if let Some(facts) = crate::lottie::facts(&doc) {
        a.kind = Some(AssetKind::Vector);
        m.extend(facts);
    } else if let Some(facts) = crate::dragonbones::facts(&doc) {
        a.kind = Some(AssetKind::Animation);
        m.extend(facts);
    } else if let Some(facts) = crate::spine::facts(&doc) {
        a.kind = Some(AssetKind::Animation);
        m.extend(facts);
    } else if format.starts_with("hoard-datatable/") {
        m.insert("rows".into(), json!(doc.get("rows").and_then(Value::as_object).map_or(0, Map::len)));
        if let Some(s) = doc.get("row_struct").and_then(Value::as_str) {
            m.insert("row_struct".into(), json!(s));
        }
    }
    Ok(())
}

pub(super) fn analyze_dotlottie(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    anyhow::ensure!(std::fs::metadata(path)?.len() <= crate::lottie::MAX_FILE, "too big to read");
    let doc = crate::lottie::from_dotlottie(&std::fs::read(path)?)?;
    let facts = crate::lottie::facts(&doc).ok_or_else(|| anyhow::anyhow!("not a Lottie animation"))?;
    a.meta.extend(facts);
    Ok(())
}

pub(super) fn analyze_rive(path: &Path, a: &mut Analysis) -> anyhow::Result<()> {
    let mut head = Vec::new();
    File::open(path)?.take(crate::rive::HEAD as u64).read_to_end(&mut head)?;
    let facts = crate::rive::facts(&head).ok_or_else(|| anyhow::anyhow!("not a Rive file"))?;
    a.meta.extend(facts);
    Ok(())
}

/// Unreal packages: class, engine version, the editor's thumbnail, registry tags, and any source file that was
/// kept inside (a `SoundWave`'s WAV, a UE4 texture's PNG), which then gets analyzed like the real thing.
pub(super) fn analyze_unreal(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
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

    if let Some(img) = pkg.thumbnail_image() {
        let img = img.thumbnail(thumb_size(), thumb_size()).to_rgba8();
        a.thumb = thumb_bytes(&img, false).ok();
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
