//! Turn Unreal assets into common files, natively:
//!
//! | Class | Output |
//! |---|---|
//! | `SoundWave` | the imported WAV, byte for byte |
//! | `Texture2D` | the imported PNG/JPEG as-is, or the raw source pixels as PNG (8/16-bit) or EXR (HDR) |
//! | `Material`, `MaterialInstanceConstant` | `material.json`: parent, texture/scalar/vector parameters, PBR slot guesses |
//! | `DataTable` | JSON rows |
//!
//! Meshes and animations are stored as Unreal mesh descriptions and animation models; those come back as
//! [`Outcome::NeedsEditor`] so Unreal Editor can export them. Blueprints, levels and other engine-only types are
//! [`Outcome::Unsupported`]: there's no common format to put them in.

use anyhow::{Context, Result, bail, ensure};
use serde_json::{Map, Value, json};

use crate::bulk::{self, TrailerEntry, Undecodable, UndecodableError};
use crate::package::{Package, carve_source};
use crate::props::{self, Properties};
use crate::reader::Reader;

/// `FUE5MainStreamObjectVersion` GUID (697DD581-E64F-41AB-AA4A-51ECBEB7B628), as serialized.
const UE5_MAIN_STREAM: [u8; 16] = guid(0x697D_D581, 0xE64F_41AB, 0xAA4A_51EC, 0xBEB7_B628);
/// UE5 main-stream version from which texture sources use `FEditorBulkData`.
const TEXTURE_SOURCE_VIRTUALIZATION: i32 = 42;

const fn guid(a: u32, b: u32, c: u32, d: u32) -> [u8; 16] {
    let (a, b, c, d) = (a.to_le_bytes(), b.to_le_bytes(), c.to_le_bytes(), d.to_le_bytes());
    [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3], c[0], c[1], c[2], c[3], d[0], d[1], d[2], d[3]]
}

/// A converted file.
#[derive(Debug, Clone)]
pub struct OutFile {
    /// File extension, e.g. `png`, `wav`, `json`.
    pub ext: &'static str,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Converted {
        file: OutFile,
        notes: Vec<String>,
    },
    /// Convertible, but only by Unreal Editor (meshes, animations, or payloads we can't decode).
    NeedsEditor(String),
    /// No common format for this kind of asset.
    Unsupported(String),
    Failed(String),
}

/// Convert the main asset of a package (`data` is the whole `.uasset` file).
pub fn convert(data: &[u8], pkg: &Package) -> Outcome {
    if pkg.cooked {
        return Outcome::Unsupported("cooked game package: editor data was stripped".into());
    }
    let class = pkg.class.clone().unwrap_or_default();
    let result = match class.as_str() {
        "SoundWave" => sound(data, pkg),
        "Texture2D" => texture(data, pkg),
        "Material" | "MaterialInstanceConstant" => material(data, pkg),
        "DataTable" => data_table(data, pkg),
        "StaticMesh" | "SkeletalMesh" | "AnimSequence" | "AnimMontage" | "AnimComposite" | "BlendSpace"
        | "BlendSpace1D" | "PoseAsset" | "TextureCube" | "Texture2DArray" | "VolumeTexture" => {
            return Outcome::NeedsEditor(format!("{class} is converted by Unreal Editor (FBX/EXR export)"));
        }
        "" => return Outcome::Failed("couldn't tell what this package contains".into()),
        _ => return Outcome::Unsupported(format!("{class} has no common file format")),
    };
    match result {
        Ok((file, notes)) => Outcome::Converted { file, notes },
        Err(e) => match e.downcast_ref::<UndecodableError>() {
            Some(UndecodableError(Undecodable::Oodle)) => Outcome::NeedsEditor(
                "the source was imported uncompressed and Unreal stores that Oodle-compressed, which only Unreal \
                 can decode"
                    .into(),
            ),
            Some(UndecodableError(Undecodable::NotInFile)) => {
                Outcome::NeedsEditor("the source data is virtualized (not stored in this file)".into())
            }
            _ => Outcome::Failed(format!("{e:#}")),
        },
    }
}

type Converted = Result<(OutFile, Vec<String>)>;

fn trailer(data: &[u8], pkg: &Package) -> Vec<TrailerEntry> {
    if pkg.payload_toc_offset.is_some() { bulk::read_trailer(data, pkg).unwrap_or_default() } else { Vec::new() }
}

fn main_props(data: &[u8], pkg: &Package) -> Result<Properties> {
    let export = pkg.main_export().context("package has no main export")?;
    props::read_export(data, pkg, export)
}

/// The original WAV: from the UE5 trailer, else carved from UE4 bulk data.
fn sound(data: &[u8], pkg: &Package) -> Converted {
    let mut oodle = false;
    for entry in trailer(data, pkg) {
        match bulk::decode_entry(data, &entry) {
            Ok(bytes) if bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WAVE") => {
                return Ok((OutFile { ext: "wav", bytes }, vec!["from the package trailer".into()]));
            }
            Err(e) if matches!(e.downcast_ref::<UndecodableError>(), Some(UndecodableError(Undecodable::Oodle))) => {
                oodle = true;
            }
            _ => {}
        }
    }
    if let Some(src) = carve_source(data, "SoundWave", usize::try_from(pkg.header_size)?) {
        return Ok((OutFile { ext: src.ext, bytes: src.bytes }, vec!["from inline bulk data".into()]));
    }
    if oodle {
        bail!(UndecodableError(Undecodable::Oodle));
    }
    bail!("no WAV data found in this SoundWave")
}

/// The source payload of an asset whose native data starts right after its properties with
/// `[object guid][strip flags][bulk data]` (textures).
fn source_payload(data: &[u8], pkg: &Package, p: &Properties) -> Result<Vec<u8>> {
    let mut r = Reader::at(data, p.end)?;
    if r.i32()? != 0 {
        r.guid()?; // object guid
    }
    let global = r.u8()?;
    r.u8()?; // class strip flags
    ensure!(global & 1 == 0, "editor data was stripped from this package");
    let virtualized = pkg.custom_version(UE5_MAIN_STREAM).is_some_and(|v| v >= TEXTURE_SOURCE_VIRTUALIZATION);
    if virtualized {
        let ebd = bulk::read_editor_bulk_data(&mut r)?;
        let trailer = trailer(data, pkg);
        ebd.load(data, &trailer)
    } else {
        Ok(bulk::read_byte_bulk_data(&mut r, data, pkg)?.to_vec())
    }
}

fn str_or_num(v: Option<&Value>) -> Option<std::borrow::Cow<'_, str>> {
    match v? {
        Value::String(s) => Some(std::borrow::Cow::Borrowed(s.as_str())),
        Value::Number(n) => Some(std::borrow::Cow::Owned(n.to_string())),
        _ => None,
    }
}

const TEXTURE_FORMATS: &[&str] = &[
    "TSF_Invalid",
    "TSF_G8",
    "TSF_BGRA8",
    "TSF_BGRE8",
    "TSF_RGBA16",
    "TSF_RGBA16F",
    "TSF_RGBA8_DEPRECATED",
    "TSF_RGBE8_DEPRECATED",
    "TSF_G16",
    "TSF_RGBA32F",
    "TSF_R16F",
    "TSF_R32F",
];

/// Texture source → PNG/JPEG passthrough, or raw pixels → PNG/EXR.
fn texture(data: &[u8], pkg: &Package) -> Converted {
    let props = main_props(data, pkg);
    let Some((p, src)) =
        props.as_ref().ok().and_then(|p| Some((p, p.values.get("Source").and_then(Value::as_object)?)))
    else {
        // No readable source settings: an image stored whole in the bulk data is still the source.
        return match carve_source(data, "Texture2D", usize::try_from(pkg.header_size)?) {
            Some(c) => Ok((OutFile { ext: c.ext, bytes: c.bytes }, vec!["from inline bulk data".into()])),
            None => Err(props.err().unwrap_or_else(|| anyhow::anyhow!("texture has no Source settings"))),
        };
    };
    let int = |k: &str| src.get(k).and_then(Value::as_u64);
    let (w, h) = (int("SizeX").unwrap_or(0), int("SizeY").unwrap_or(0));
    // Formats are enum names (TSF_BGRA8) or, in older files, the byte value.
    let format = str_or_num(src.get("Format")).map_or_else(
        || "TSF_BGRA8".to_owned(),
        |f| {
            f.parse::<usize>()
                .ok()
                .and_then(|i| TEXTURE_FORMATS.get(i).map(|s| (*s).to_owned()))
                .unwrap_or(f.into_owned())
        },
    );
    let compression = str_or_num(src.get("CompressionFormat")).map(std::borrow::Cow::into_owned).unwrap_or_default();
    let png = src.get("bPNGCompressed").and_then(Value::as_bool).unwrap_or(false)
        || compression == "TSCF_PNG"
        || compression == "1";
    let jpeg = compression == "TSCF_JPEG" || compression == "2";

    let payload = match source_payload(data, pkg, p) {
        Ok(b) => b,
        Err(e) if e.downcast_ref::<UndecodableError>().is_some() => return Err(e),
        // Layout surprise: fall back to the trailer's only/largest payload, then to carving a PNG.
        Err(first) => {
            let entries = trailer(data, pkg);
            let best = entries.iter().filter(|e| e.access == bulk::Access::Local).max_by_key(|e| e.raw_size);
            match best {
                Some(entry) => bulk::decode_entry(data, entry)?,
                None => match carve_source(data, "Texture2D", usize::try_from(pkg.header_size)?) {
                    Some(c) => c.bytes,
                    None => return Err(first.context("couldn't find the texture source")),
                },
            }
        }
    };
    let notes = vec![format!(
        "{w}x{h} {format}{}",
        if compression.is_empty() { String::new() } else { format!(" {compression}") }
    )];
    if png || payload.starts_with(b"\x89PNG") {
        ensure!(payload.starts_with(b"\x89PNG"), "source says PNG but isn't one");
        return Ok((OutFile { ext: "png", bytes: payload }, notes));
    }
    if jpeg || payload.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Ok((OutFile { ext: "jpg", bytes: payload }, notes));
    }
    ensure!(w > 0 && h > 0 && w <= 65_536 && h <= 65_536, "texture source has no size");
    let (w, h) = (u32::try_from(w)?, u32::try_from(h)?);
    raw_pixels(&payload, w, h, &format).map(|file| (file, notes))
}

#[expect(clippy::many_single_char_names, reason = "width/height/pixel count read better short")]
fn raw_pixels(px: &[u8], w: u32, h: u32, format: &str) -> Result<OutFile> {
    let n = (w as usize) * (h as usize);
    let need = |bpp: usize| -> Result<&[u8]> {
        px.get(..n * bpp).with_context(|| format!("source has {} bytes, {w}x{h} {format} needs {}", px.len(), n * bpp))
    };
    let encode = |img: image::DynamicImage, ext: &'static str| -> Result<OutFile> {
        let mut bytes = Vec::new();
        let fmt = if ext == "exr" { image::ImageFormat::OpenExr } else { image::ImageFormat::Png };
        img.write_to(&mut std::io::Cursor::new(&mut bytes), fmt)?;
        Ok(OutFile { ext, bytes })
    };
    Ok(match format {
        "TSF_G8" => encode(image::GrayImage::from_raw(w, h, need(1)?.to_vec()).context("size")?.into(), "png")?,
        "TSF_BGRA8" | "TSF_RGBA8_DEPRECATED" => {
            let mut v = need(4)?.to_vec();
            if format == "TSF_BGRA8" {
                v.as_chunks_mut::<4>().0.iter_mut().for_each(|p| p.swap(0, 2));
            }
            encode(image::RgbaImage::from_raw(w, h, v).context("size")?.into(), "png")?
        }
        "TSF_G16" => {
            let v: Vec<u16> = u16s(need(2)?);
            encode(image::ImageBuffer::<image::Luma<u16>, _>::from_raw(w, h, v).context("size")?.into(), "png")?
        }
        "TSF_RGBA16" => {
            let v: Vec<u16> = u16s(need(8)?);
            encode(image::ImageBuffer::<image::Rgba<u16>, _>::from_raw(w, h, v).context("size")?.into(), "png")?
        }
        "TSF_RGBA16F" => {
            let v: Vec<f32> = halves(need(8)?);
            encode(image::Rgba32FImage::from_raw(w, h, v).context("size")?.into(), "exr")?
        }
        "TSF_RGBA32F" => {
            let v: Vec<f32> = f32s(need(16)?);
            encode(image::Rgba32FImage::from_raw(w, h, v).context("size")?.into(), "exr")?
        }
        "TSF_R16F" | "TSF_R32F" => {
            let gray: Vec<f32> = if format == "TSF_R16F" { halves(need(2)?) } else { f32s(need(4)?) };
            let rgb: Vec<f32> = gray.iter().flat_map(|&g| [g, g, g]).collect();
            encode(image::Rgb32FImage::from_raw(w, h, rgb).context("size")?.into(), "exr")?
        }
        "TSF_BGRE8" | "TSF_RGBE8_DEPRECATED" => {
            let rgb: Vec<f32> = need(4)?
                .as_chunks::<4>()
                .0
                .iter()
                .flat_map(|p| {
                    let (r, g, b) = if format == "TSF_BGRE8" { (p[2], p[1], p[0]) } else { (p[0], p[1], p[2]) };
                    let scale = if p[3] == 0 { 0.0 } else { (f32::from(p[3]) - 136.0).exp2() };
                    [f32::from(r) * scale, f32::from(g) * scale, f32::from(b) * scale]
                })
                .collect();
            encode(image::Rgb32FImage::from_raw(w, h, rgb).context("size")?.into(), "exr")?
        }
        other => bail!("texture source format {other} isn't converted yet"),
    })
}

fn u16s(b: &[u8]) -> Vec<u16> {
    b.as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect()
}

fn halves(b: &[u8]) -> Vec<f32> {
    b.as_chunks::<2>().0.iter().map(|c| f16_to_f32(u16::from_le_bytes(*c))).collect()
}

fn f32s(b: &[u8]) -> Vec<f32> {
    b.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)).collect()
}

/// IEEE half → f32.
fn f16_to_f32(h: u16) -> f32 {
    let sign = u32::from(h >> 15) << 31;
    let exp = u32::from((h >> 10) & 0x1f);
    let frac = u32::from(h & 0x3ff);
    let bits = match (exp, frac) {
        (0, 0) => sign,
        (0, f) => {
            // Subnormal: normalize.
            let mut e = 127 - 15 + 1;
            let mut f = f;
            while f & 0x400 == 0 {
                f <<= 1;
                e -= 1;
            }
            sign | (e << 23) | ((f & 0x3ff) << 13)
        }
        (0x1f, f) => sign | 0x7f80_0000 | (f << 13),
        (e, f) => sign | ((e + 127 - 15) << 23) | (f << 13),
    };
    f32::from_bits(bits)
}

/// Material or material instance → a JSON description other engines' importers (or people) can rebuild from.
fn material(data: &[u8], pkg: &Package) -> Converted {
    let p = main_props(data, pkg)?;
    let v = &p.values;
    let param_name = |entry: &Value| -> Option<String> {
        entry
            .get("ParameterInfo")
            .and_then(|i| i.get("Name"))
            .or_else(|| entry.get("ParameterName"))?
            .as_str()
            .map(str::to_owned)
    };
    let collect = |key: &str| -> Map<String, Value> {
        let mut out = Map::new();
        for e in v.get(key).and_then(Value::as_array).into_iter().flatten() {
            if let (Some(name), Some(val)) = (param_name(e), e.get("ParameterValue")) {
                out.insert(name, val.clone());
            }
        }
        out
    };
    let mut textures = collect("TextureParameterValues");
    // Base materials: texture samples are expression objects in the same package.
    for e in &pkg.exports {
        let class = export_class(pkg, e);
        if !class.starts_with("MaterialExpression") || !class.contains("Texture") {
            continue;
        }
        let Ok(ep) = props::read_export(data, pkg, e) else { continue };
        if let Some(tex) = ep.values.get("Texture").filter(|t| t.is_string()) {
            let key =
                ep.values.get("ParameterName").and_then(Value::as_str).map_or_else(|| e.name.clone(), str::to_owned);
            textures.entry(key).or_insert_with(|| tex.clone());
        }
    }
    let scalars = collect("ScalarParameterValues");
    let vectors = collect("VectorParameterValues");
    // Base materials reference textures from expression objects; list what the package imports.
    let referenced: Vec<String> = pkg
        .imports
        .iter()
        .enumerate()
        .filter(|(_, i)| i.class_name.starts_with("Texture"))
        .filter_map(|(n, _)| pkg.object_path(-i32::try_from(n).ok()? - 1))
        .collect();
    // Which texture is which PBR slot, guessed from parameter names (and, failing that, texture names).
    let mut pbr = Map::new();
    let candidates: Vec<(String, String)> = if textures.is_empty() {
        referenced.iter().map(|t| (t.rsplit(['/', '.']).next().unwrap_or(t).to_owned(), t.clone())).collect()
    } else {
        textures.iter().filter_map(|(k, t)| Some((k.clone(), t.as_str()?.to_owned()))).collect()
    };
    for (name, tex) in candidates {
        if let Some(slot) = pbr_slot(&name) {
            pbr.entry(slot.to_owned()).or_insert(Value::String(tex));
        }
    }
    let mut out = json!({
        "format": "hoard-material/1",
        "name": pkg.asset_name,
        "unreal_class": pkg.class,
        "engine": pkg.engine,
        "pbr": pbr,
        "textures": textures,
        "scalars": scalars,
        "vectors": vectors,
    });
    if let Some(parent) = v.get("Parent") {
        out["parent"] = parent.clone();
    }
    if !referenced.is_empty() {
        out["referenced_textures"] = json!(referenced);
    }
    for key in ["BlendMode", "TwoSided", "ShadingModel", "MaterialDomain", "OpacityMaskClipValue"] {
        if let Some(x) = v.get(key) {
            out[key] = x.clone();
        }
    }
    if let Some(over) = v.get("BasePropertyOverrides") {
        out["overrides"] = over.clone();
    }
    let bytes = serde_json::to_vec_pretty(&out)?;
    Ok((
        OutFile { ext: "json", bytes },
        vec![format!("{} texture parameters", out["textures"].as_object().map_or(0, Map::len))],
    ))
}

/// Class name of an export (`MaterialExpressionTextureSample`, …).
fn export_class<'a>(pkg: &'a Package, e: &crate::package::Export) -> &'a str {
    usize::try_from(-i64::from(e.class_index) - 1)
        .ok()
        .and_then(|i| pkg.imports.get(i))
        .map_or("", |imp| imp.object_name.as_str())
}

fn pbr_slot(name: &str) -> Option<&'static str> {
    let n = name.to_lowercase();
    let has = |k: &[&str]| k.iter().any(|w| n.contains(w));
    Some(if has(&["normal", "nrm"]) || n.ends_with("_n") {
        "normal"
    } else if has(&["orm", "arm", "mra"]) {
        "occlusion_roughness_metallic"
    } else if has(&["rough"]) || n.ends_with("_r") {
        "roughness"
    } else if has(&["metal"]) || n.ends_with("_m") {
        "metallic"
    } else if has(&["occlusion", "_ao", "ambientocc"]) {
        "occlusion"
    } else if has(&["emiss", "glow"]) || n.ends_with("_e") {
        "emissive"
    } else if has(&["opacity", "alpha", "mask"]) {
        "opacity"
    } else if has(&["height", "disp", "bump"]) || n.ends_with("_h") {
        "height"
    } else if has(&["base", "albedo", "diffuse", "color", "colour"]) || n.ends_with("_d") {
        "base_color"
    } else {
        return None;
    })
}

/// `DataTable`: the row struct name, then rows of (name, tagged struct), after the properties.
fn data_table(data: &[u8], pkg: &Package) -> Converted {
    let export = pkg.main_export().context("package has no main export")?;
    let p = props::read_export(data, pkg, export)?;
    let mut r = Reader::at(data, p.end)?;
    if r.i32()? != 0 {
        r.guid()?;
    }
    let rows = props::read_rows(&mut r, pkg, export)?;
    let out = json!({
        "format": "hoard-datatable/1",
        "name": pkg.asset_name,
        "row_struct": p.values.get("RowStruct"),
        "rows": Value::Object(rows),
    });
    let n = out["rows"].as_object().map_or(0, Map::len);
    Ok((OutFile { ext: "json", bytes: serde_json::to_vec_pretty(&out)? }, vec![format!("{n} rows")]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testpkg::{Synth, compressed_buffer_lz4, compressed_buffer_raw, trailer};

    const PAYLOAD_ID: [u8; 20] = [7; 20];

    fn convert_bytes(bytes: &[u8]) -> Outcome {
        let pkg = crate::parse(bytes).unwrap();
        convert(bytes, &pkg)
    }

    fn converted(o: Outcome) -> (OutFile, Vec<String>) {
        match o {
            Outcome::Converted { file, notes } => (file, notes),
            other => panic!("expected a conversion, got {other:?}"),
        }
    }

    /// A texture package: `Source` properties, then guid flag, strip flags and the given bulk data record.
    fn texture_package(ue5: i32, w: i32, h: i32, format: &str, compression: Option<&str>, bulk: &[u8]) -> Synth {
        let mut p = Synth::new("Texture2D", "T_Test", ue5);
        if ue5 > 0 {
            p.custom_versions.push((UE5_MAIN_STREAM, 60));
        }
        let mut w_ = p.props();
        w_.structure("Source", "TextureSource", |s| {
            s.int("SizeX", w).int("SizeY", h).int("NumSlices", 1).int("NumMips", 1);
            s.enumeration("Format", "ETextureSourceFormat", format);
            if let Some(c) = compression {
                s.enumeration("CompressionFormat", "ETextureSourceCompressionFormat", c);
            }
        });
        w_.boolean("SRGB", true);
        w_.end();
        let mut export = w_.out;
        export.extend_from_slice(&0i32.to_le_bytes()); // no object guid
        export.extend_from_slice(&[0, 0]); // strip flags
        export.extend_from_slice(bulk);
        p.export = export;
        p
    }

    /// An `FEditorBulkData` record pointing into the trailer.
    fn editor_bulk(size: usize) -> Vec<u8> {
        let mut b = bulk::EBD_STORED_IN_TRAILER.to_le_bytes().to_vec();
        b.extend_from_slice(&[1; 16]);
        b.extend_from_slice(&PAYLOAD_ID);
        b.extend_from_slice(&(size as i64).to_le_bytes());
        b
    }

    #[test]
    fn ue5_texture_from_lz4_trailer() {
        // 64x2 BGRA: compressible, two LZ4 blocks of 256 bytes.
        let (w, h) = (64u32, 2u32);
        let bgra: Vec<u8> = (0..w * h).flat_map(|i| [10, 20, (i % 7) as u8 * 30, 255]).collect();
        let mut p = texture_package(1009, w as i32, h as i32, "TSF_BGRA8", None, &editor_bulk(bgra.len()));
        p.trailer = Some(trailer(&[(PAYLOAD_ID, compressed_buffer_lz4(&bgra, 8), bgra.len() as u64)]));
        let (file, notes) = converted(convert_bytes(&p.build()));
        assert_eq!(file.ext, "png");
        assert_eq!(notes, ["64x2 TSF_BGRA8"]);
        let img = image::load_from_memory(&file.bytes).unwrap().to_rgba8();
        assert_eq!(img.dimensions(), (w, h));
        assert_eq!(img.get_pixel(3, 1).0, [120, 20, 10, 255]); // swizzled to RGBA
    }

    #[test]
    fn ue54_hdr_texture_to_exr() {
        let px: Vec<u8> = [0x3c00u16, 0x4000, 0x3800, 0x3c00].repeat(4).iter().flat_map(|h| h.to_le_bytes()).collect();
        let mut p = texture_package(1012, 2, 2, "TSF_RGBA16F", None, &editor_bulk(px.len()));
        p.trailer = Some(trailer(&[(PAYLOAD_ID, compressed_buffer_raw(&px), px.len() as u64)]));
        let (file, _) = converted(convert_bytes(&p.build()));
        assert_eq!(file.ext, "exr");
        let img = image::load_from_memory(&file.bytes).unwrap().to_rgba32f();
        assert_eq!(img.get_pixel(1, 1).0, [1.0, 2.0, 0.5, 1.0]);
    }

    #[test]
    fn ue4_texture_inline_bulk_16bit() {
        let px: Vec<u8> = [0u16, 1000, 40_000, 65_535].iter().flat_map(|v| v.to_le_bytes()).collect();
        let mut bulk = 0u32.to_le_bytes().to_vec(); // inline
        bulk.extend_from_slice(&(px.len() as i32).to_le_bytes());
        bulk.extend_from_slice(&(px.len() as i32).to_le_bytes());
        bulk.extend_from_slice(&(-1i64).to_le_bytes());
        bulk.extend_from_slice(&px);
        let p = texture_package(0, 2, 2, "TSF_G16", None, &bulk);
        let bytes = p.build();
        let pkg = crate::parse(&bytes).unwrap();
        assert_eq!((pkg.engine.as_str(), pkg.ue5), ("4.27.2", 0));
        let (file, _) = converted(convert(&bytes, &pkg));
        let img = image::load_from_memory(&file.bytes).unwrap().to_luma16();
        assert_eq!(img.as_raw(), &[0, 1000, 40_000, 65_535]);
    }

    #[test]
    fn png_source_passes_through() {
        let mut png = Vec::new();
        image::RgbaImage::from_pixel(3, 3, image::Rgba([1, 2, 3, 4]))
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        let mut p = texture_package(1012, 3, 3, "TSF_BGRA8", Some("TSCF_PNG"), &editor_bulk(png.len()));
        p.trailer = Some(trailer(&[(PAYLOAD_ID, compressed_buffer_raw(&png), png.len() as u64)]));
        let (file, notes) = converted(convert_bytes(&p.build()));
        assert_eq!((file.ext, file.bytes), ("png", png));
        assert_eq!(notes, ["3x3 TSF_BGRA8 TSCF_PNG"]);
    }

    #[test]
    fn oodle_compressed_source_needs_editor() {
        let px = vec![9u8; 64];
        let mut buf = compressed_buffer_lz4(&px, 6);
        buf[8] = 3; // claim Oodle
        let mut p = texture_package(1009, 4, 4, "TSF_BGRA8", None, &editor_bulk(px.len()));
        p.trailer = Some(trailer(&[(PAYLOAD_ID, buf, px.len() as u64)]));
        match convert_bytes(&p.build()) {
            Outcome::NeedsEditor(why) => assert!(why.contains("Oodle"), "{why}"),
            other => panic!("{other:?}"),
        }
    }

    /// A PNG doesn't shrink, so even in a buffer marked Oodle its blocks are stored raw and need no decoder.
    #[test]
    fn png_in_oodle_buffer_passes_through() {
        let mut seed = 0x2545_f491_u32;
        let noise: Vec<u8> = (0..64 * 64 * 4)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                seed.to_le_bytes()[0]
            })
            .collect();
        let mut png = Vec::new();
        image::RgbaImage::from_raw(64, 64, noise)
            .unwrap()
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        let mut buf = compressed_buffer_lz4(&png, 10);
        buf[8] = 3; // Oodle
        let mut p = texture_package(1012, 64, 64, "TSF_BGRA8", Some("TSCF_PNG"), &editor_bulk(png.len()));
        p.trailer = Some(trailer(&[(PAYLOAD_ID, buf, png.len() as u64)]));
        let (file, _) = converted(convert_bytes(&p.build()));
        assert_eq!((file.ext, file.bytes), ("png", png));
    }

    #[test]
    fn ue5_sound_from_trailer() {
        let mut wav = b"RIFF\x24\0\0\0WAVEfmt ".to_vec();
        wav.extend_from_slice(&[16, 0, 0, 0, 1, 0, 1, 0, 0x44, 0xac, 0, 0, 0x88, 0x58, 1, 0, 2, 0, 16, 0]);
        wav.extend_from_slice(b"data\0\0\0\0");
        let mut p = Synth::new("SoundWave", "S_Test", 1012);
        let mut w = p.props();
        w.float("Duration", 0.0).end();
        p.export = w.out;
        p.trailer = Some(trailer(&[(PAYLOAD_ID, compressed_buffer_lz4(&wav, 16), wav.len() as u64)]));
        let (file, _) = converted(convert_bytes(&p.build()));
        assert_eq!((file.ext, file.bytes), ("wav", wav));
    }

    #[test]
    fn material_instance_parameters() {
        for ue5 in [0, 1009, 1012] {
            let mut p = Synth::new("MaterialInstanceConstant", "MI_Rock", ue5);
            p.objects = vec![
                ("Material".into(), "M_Base".into(), String::new()),
                ("Texture2D".into(), "T_Rock_N".into(), String::new()),
            ];
            let (parent, tex) = (p.object_index("M_Base"), p.object_index("T_Rock_N"));
            let mut w = p.props();
            w.object("Parent", parent);
            w.struct_array(
                "TextureParameterValues",
                "TextureParameterValue",
                &[&|s| {
                    s.structure("ParameterInfo", "MaterialParameterInfo", |i| {
                        i.name("Name", "NormalMap");
                    });
                    s.object("ParameterValue", tex);
                }],
            );
            w.struct_array(
                "ScalarParameterValues",
                "ScalarParameterValue",
                &[&|s| {
                    s.structure("ParameterInfo", "MaterialParameterInfo", |i| {
                        i.name("Name", "Roughness");
                    });
                    s.float("ParameterValue", 0.75);
                }],
            );
            w.end();
            p.export = w.out;
            let (file, notes) = converted(convert_bytes(&p.build()));
            let json: Value = serde_json::from_slice(&file.bytes).unwrap();
            assert_eq!(json["parent"], "/Script/Engine.M_Base", "ue5 {ue5}");
            assert_eq!(json["textures"]["NormalMap"], "/Script/Engine.T_Rock_N");
            assert_eq!(json["pbr"]["normal"], "/Script/Engine.T_Rock_N");
            assert_eq!(json["scalars"]["Roughness"], 0.75);
            assert_eq!(notes, ["1 texture parameters"]);
        }
    }

    #[test]
    fn routing() {
        let mesh = Synth::new("StaticMesh", "SM_Rock", 1009).build();
        assert!(matches!(convert_bytes(&mesh), Outcome::NeedsEditor(_)));
        let bp = Synth::new("Blueprint", "BP_Door", 1009).build();
        assert!(matches!(convert_bytes(&bp), Outcome::Unsupported(_)));
    }

    /// Writes a small fake Unreal project for end-to-end runs: `HOARD_FIXTURE_OUT=dir cargo test -p hoard-unreal
    /// write_fixtures -- --ignored`.
    #[test]
    #[ignore = "writes files; run by hand"]
    fn write_fixtures() {
        let Some(out) = std::env::var_os("HOARD_FIXTURE_OUT") else { return };
        let root = std::path::PathBuf::from(out).join("RockPack/Content/RockPack");
        let write = |rel: &str, bytes: Vec<u8>| {
            let p = root.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, bytes).unwrap();
        };
        // An albedo texture: a 64x64 gradient, LZ4 in the trailer.
        let (w, h) = (64u32, 64u32);
        let bgra: Vec<u8> = (0..w * h).flat_map(|i| [(i % w * 4) as u8, (i / w * 4) as u8, 160, 255]).collect();
        let mut t = texture_package(1009, 64, 64, "TSF_BGRA8", None, &editor_bulk(bgra.len()));
        t.asset = "T_Rock_D".into();
        t.names.idx("T_Rock_D");
        t.trailer = Some(trailer(&[(PAYLOAD_ID, compressed_buffer_lz4(&bgra, 12), bgra.len() as u64)]));
        write("Textures/T_Rock_D.uasset", t.build());
        // An HDR sky, 5.4 layout.
        let px: Vec<u8> =
            (0..16 * 8).flat_map(|i: u16| [0x3c00u16 + i, 0x4000, 0x3800, 0x3c00]).flat_map(u16::to_le_bytes).collect();
        let mut t = texture_package(1012, 16, 8, "TSF_RGBA16F", None, &editor_bulk(px.len()));
        t.asset = "T_Sky_HDR".into();
        t.names.idx("T_Sky_HDR");
        t.trailer = Some(trailer(&[(PAYLOAD_ID, compressed_buffer_raw(&px), px.len() as u64)]));
        write("Textures/T_Sky_HDR.uasset", t.build());
        // A one-second 440 Hz tone.
        let samples: Vec<u8> = (0..22_050)
            .flat_map(|i| {
                (((f64::from(i) * 440.0 * std::f64::consts::TAU / 22_050.0).sin() * 9000.0) as i16).to_le_bytes()
            })
            .collect();
        let mut wav = b"RIFF".to_vec();
        wav.extend_from_slice(&(36 + samples.len() as u32).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&[16, 0, 0, 0, 1, 0, 1, 0, 0x22, 0x56, 0, 0, 0x44, 0xac, 0, 0, 2, 0, 16, 0]);
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&(samples.len() as u32).to_le_bytes());
        wav.extend_from_slice(&samples);
        let mut snd = Synth::new("SoundWave", "S_Wind", 1012);
        let mut w_ = snd.props();
        w_.float("Duration", 1.0).end();
        snd.export = w_.out;
        snd.trailer = Some(trailer(&[(PAYLOAD_ID, compressed_buffer_lz4(&wav, 16), wav.len() as u64)]));
        write("Audio/S_Wind.uasset", snd.build());
        // A material instance using the texture.
        let mut mi = Synth::new("MaterialInstanceConstant", "MI_Rock", 1009);
        mi.deps = vec!["/Game/RockPack/Textures/T_Rock_D".into(), "/Game/RockPack/Materials/M_Master".into()];
        mi.objects = vec![
            ("Texture2D".into(), "T_Rock_D".into(), "/Game/RockPack/Textures/T_Rock_D".into()),
            ("Material".into(), "M_Master".into(), "/Game/RockPack/Materials/M_Master".into()),
        ];
        let (tex, parent) = (mi.object_index("T_Rock_D"), mi.object_index("M_Master"));
        let mut w_ = mi.props();
        w_.object("Parent", parent);
        w_.struct_array(
            "TextureParameterValues",
            "TextureParameterValue",
            &[&|s| {
                s.structure("ParameterInfo", "MaterialParameterInfo", |i| {
                    i.name("Name", "BaseColor");
                });
                s.object("ParameterValue", tex);
            }],
        );
        w_.end();
        mi.export = w_.out;
        write("Materials/MI_Rock.uasset", mi.build());
        // A mesh (needs the editor) using the material, and a Blueprint (no common format).
        let mut sm = Synth::new("StaticMesh", "SM_Rock", 1009);
        sm.deps = vec!["/Game/RockPack/Materials/MI_Rock".into()];
        write("Meshes/SM_Rock.uasset", sm.build());
        write("Blueprints/BP_Door.uasset", Synth::new("Blueprint", "BP_Door", 1009).build());
    }

    #[test]
    fn half_floats() {
        assert!((f16_to_f32(0x3c00) - 1.0).abs() < f32::EPSILON);
        assert!((f16_to_f32(0xc000) + 2.0).abs() < f32::EPSILON);
        assert!((f16_to_f32(0x3555) - 0.333_251_95).abs() < 1e-6);
        assert!(f16_to_f32(0x7c00).is_infinite());
        assert!(f16_to_f32(0x0001) > 0.0);
    }

    #[test]
    fn pbr_slots() {
        assert_eq!(pbr_slot("BaseColorTexture"), Some("base_color"));
        assert_eq!(pbr_slot("T_Rock_N"), Some("normal"));
        assert_eq!(pbr_slot("ORM"), Some("occlusion_roughness_metallic"));
        assert_eq!(pbr_slot("Tint"), None);
    }

    #[test]
    fn guid_layout() {
        assert_eq!(&UE5_MAIN_STREAM[..4], &[0x81, 0xD5, 0x7D, 0x69]);
    }
}
