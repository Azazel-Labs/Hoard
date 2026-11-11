//! Texture formats `image` doesn't read: GPU containers (DDS, KTX2) with block-compressed data, vector art (SVG)
//! and pixel-art sources (Aseprite). Each decodes to an ordinary image (mip 0 of the first layer or face) for
//! thumbnails, color analysis and "download as", plus the container's own facts.

use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};
use serde_json::{Map, Value, json};

/// Refuse to allocate decode buffers past this (16k x 16k).
const MAX_PIXELS: usize = 268_435_456;

/// A decoded texture and what its container says about itself.
pub(crate) struct Decoded {
    pub(crate) image: DynamicImage,
    pub(crate) facts: Map<String, Value>,
}

/// Extensions handled here.
pub(crate) const EXTS: &[&str] = &["dds", "ktx2", "basis", "svg", "svgz", "aseprite", "ase"];

pub(crate) fn decode(path: &Path, ext: &str) -> Result<Decoded> {
    match ext {
        "dds" => dds(path),
        "ktx2" => ktx2(path),
        "basis" => basis(&std::fs::read(path)?),
        "svg" | "svgz" => svg(path, 1024),
        "aseprite" | "ase" => aseprite(path),
        _ => bail!("not a texture format handled here: .{ext}"),
    }
}

/// Block-compressed or raw pixel data in one of the layouts below.
#[derive(Debug, Clone, Copy)]
enum Layout {
    Bc1,
    Bc2,
    Bc3,
    Bc4,
    Bc5,
    Bc6 { signed: bool },
    Bc7,
    Etc2Rgb,
    Etc2Rgba1,
    Etc2Rgba8,
    EacR,
    EacRg,
    Astc { bw: usize, bh: usize },
    Rgba8,
    Bgra8,
    Bgrx8,
    Rgb8,
    Bgr8,
    R8,
    Rg8,
    La8,
    Rgba16F,
    Rgba32F,
}

fn check_size(w: u32, h: u32) -> Result<(usize, usize)> {
    let (w, h) = (w as usize, h as usize);
    ensure!(w > 0 && h > 0 && w.saturating_mul(h) <= MAX_PIXELS, "texture is too large or empty ({w}x{h})");
    Ok((w, h))
}

fn half(h: u16) -> f32 {
    let sign = u32::from(h >> 15) << 31;
    let exp = u32::from((h >> 10) & 0x1f);
    let frac = u32::from(h & 0x3ff);
    let bits = match (exp, frac) {
        (0, 0) => sign,
        (0, mut f) => {
            let mut e = 127 - 15 + 1;
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

/// A texture2ddecoder whole-image decoder.
type BlockDecoder = fn(&[u8], usize, usize, &mut [u32]) -> Result<(), &'static str>;

/// texture2ddecoder writes BGRA as little-endian u32s.
fn bgra_words(px: &[u32]) -> Vec<u8> {
    px.iter()
        .flat_map(|p| {
            let c = p.to_le_bytes();
            [c[2], c[1], c[0], c[3]]
        })
        .collect()
}

/// Decode mip 0 of `layout` from `data`.
fn decode_layout(data: &[u8], w: u32, h: u32, layout: Layout) -> Result<DynamicImage> {
    use texture2ddecoder as t;
    let (wu, hu) = check_size(w, h)?;
    let n = wu * hu;
    let need = |bpp: usize| data.get(..n * bpp).context("texture data is shorter than its size says");
    let block = |f: BlockDecoder| -> Result<DynamicImage> {
        let mut px = vec![0u32; n];
        f(data, wu, hu, &mut px).map_err(|e| anyhow::anyhow!("{e}"))?;
        // texture2ddecoder writes BGRA as little-endian u32s.
        let rgba = bgra_words(&px);
        Ok(DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, rgba).context("size")?))
    };
    Ok(match layout {
        Layout::Bc1 => block(t::decode_bc1)?,
        Layout::Bc2 => block(t::decode_bc2)?,
        Layout::Bc3 => block(t::decode_bc3)?,
        Layout::Bc4 => block(t::decode_bc4)?,
        Layout::Bc5 => block(t::decode_bc5)?,
        Layout::Bc6 { signed: false } => block(t::decode_bc6_unsigned)?,
        Layout::Bc6 { signed: true } => block(t::decode_bc6_signed)?,
        Layout::Bc7 => block(t::decode_bc7)?,
        Layout::Etc2Rgb => block(t::decode_etc2_rgb)?,
        Layout::Etc2Rgba1 => block(t::decode_etc2_rgba1)?,
        Layout::Etc2Rgba8 => block(t::decode_etc2_rgba8)?,
        Layout::EacR => block(t::decode_eacr)?,
        Layout::EacRg => block(t::decode_eacrg)?,
        Layout::Astc { bw, bh } => {
            let mut px = vec![0u32; n];
            t::decode_astc(data, wu, hu, bw, bh, &mut px).map_err(|e| anyhow::anyhow!("{e}"))?;
            let rgba = bgra_words(&px);
            DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, rgba).context("size")?)
        }
        Layout::Rgba8 => DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, need(4)?.to_vec()).context("size")?),
        Layout::Bgra8 | Layout::Bgrx8 => {
            let mut v = need(4)?.to_vec();
            for p in v.as_chunks_mut::<4>().0 {
                p.swap(0, 2);
                if matches!(layout, Layout::Bgrx8) {
                    p[3] = 255;
                }
            }
            DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, v).context("size")?)
        }
        Layout::Rgb8 => DynamicImage::ImageRgb8(image::RgbImage::from_raw(w, h, need(3)?.to_vec()).context("size")?),
        Layout::Bgr8 => {
            let v: Vec<u8> = need(3)?.as_chunks::<3>().0.iter().flat_map(|p| [p[2], p[1], p[0]]).collect();
            DynamicImage::ImageRgb8(image::RgbImage::from_raw(w, h, v).context("size")?)
        }
        Layout::R8 => DynamicImage::ImageLuma8(image::GrayImage::from_raw(w, h, need(1)?.to_vec()).context("size")?),
        Layout::La8 => {
            DynamicImage::ImageLumaA8(image::GrayAlphaImage::from_raw(w, h, need(2)?.to_vec()).context("size")?)
        }
        Layout::Rg8 => {
            // Two-channel data (normal maps): show it the way engines sample it, blue at zero.
            let v: Vec<u8> = need(2)?.as_chunks::<2>().0.iter().flat_map(|p| [p[0], p[1], 0, 255]).collect();
            DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, v).context("size")?)
        }
        Layout::Rgba16F => {
            let v: Vec<f32> = need(8)?.as_chunks::<2>().0.iter().map(|c| half(u16::from_le_bytes(*c))).collect();
            DynamicImage::ImageRgba32F(image::Rgba32FImage::from_raw(w, h, v).context("size")?)
        }
        Layout::Rgba32F => {
            let v: Vec<f32> = need(16)?.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)).collect();
            DynamicImage::ImageRgba32F(image::Rgba32FImage::from_raw(w, h, v).context("size")?)
        }
    })
}

fn dds(path: &Path) -> Result<Decoded> {
    use ddsfile::{D3DFormat as D, DxgiFormat as X};
    let dds = ddsfile::Dds::read(std::io::BufReader::new(std::fs::File::open(path)?))
        .map_err(|e| anyhow::anyhow!("not a readable DDS file: {e}"))?;
    let (w, h) = (dds.get_width(), dds.get_height());
    let (layout, name) = if let Some(f) = dds.get_dxgi_format() {
        let l = match f {
            X::BC1_UNorm | X::BC1_UNorm_sRGB | X::BC1_Typeless => Layout::Bc1,
            X::BC2_UNorm | X::BC2_UNorm_sRGB | X::BC2_Typeless => Layout::Bc2,
            X::BC3_UNorm | X::BC3_UNorm_sRGB | X::BC3_Typeless => Layout::Bc3,
            X::BC4_UNorm | X::BC4_SNorm | X::BC4_Typeless => Layout::Bc4,
            X::BC5_UNorm | X::BC5_SNorm | X::BC5_Typeless => Layout::Bc5,
            X::BC6H_UF16 | X::BC6H_Typeless => Layout::Bc6 { signed: false },
            X::BC6H_SF16 => Layout::Bc6 { signed: true },
            X::BC7_UNorm | X::BC7_UNorm_sRGB | X::BC7_Typeless => Layout::Bc7,
            X::R8G8B8A8_UNorm | X::R8G8B8A8_UNorm_sRGB => Layout::Rgba8,
            X::B8G8R8A8_UNorm | X::B8G8R8A8_UNorm_sRGB => Layout::Bgra8,
            X::B8G8R8X8_UNorm | X::B8G8R8X8_UNorm_sRGB => Layout::Bgrx8,
            X::R8_UNorm => Layout::R8,
            X::R8G8_UNorm => Layout::Rg8,
            X::R16G16B16A16_Float => Layout::Rgba16F,
            X::R32G32B32A32_Float => Layout::Rgba32F,
            other => bail!("DDS format {other:?} isn't supported yet"),
        };
        (l, format!("{f:?}"))
    } else if let Some(f) = dds.get_d3d_format() {
        let l = match f {
            D::DXT1 => Layout::Bc1,
            D::DXT2 | D::DXT3 => Layout::Bc2,
            D::DXT4 | D::DXT5 => Layout::Bc3,
            D::A8R8G8B8 => Layout::Bgra8,
            D::X8R8G8B8 => Layout::Bgrx8,
            D::A8B8G8R8 | D::X8B8G8R8 => Layout::Rgba8,
            D::R8G8B8 => Layout::Bgr8,
            D::L8 => Layout::R8,
            D::A8L8 => Layout::La8,
            other => bail!("DDS format {other:?} isn't supported yet"),
        };
        (l, format!("{f:?}"))
    } else {
        bail!("DDS file with an unknown pixel format")
    };
    let data = dds.get_data(0).map_err(|e| anyhow::anyhow!("DDS data: {e}"))?;
    let image = decode_layout(data, w, h, layout)?;
    let mut facts = Map::new();
    facts.insert("gpu_format".into(), json!(name));
    facts.insert("mip_levels".into(), json!(dds.get_num_mipmap_levels().max(1)));
    let layers = dds.get_num_array_layers();
    if layers > 1 {
        facts.insert("array_layers".into(), json!(layers));
    }
    if dds.get_depth() > 1 {
        facts.insert("depth".into(), json!(dds.get_depth()));
    }
    Ok(Decoded { image, facts })
}

fn ktx2(path: &Path) -> Result<Decoded> {
    let bytes = std::fs::read(path)?;
    let reader = ktx2::Reader::new(&bytes).map_err(|e| anyhow::anyhow!("not a readable KTX2 file: {e:?}"))?;
    let header = reader.header();
    let Some(format) = header.format else {
        // VK_FORMAT_UNDEFINED: Basis Universal (ETC1S or UASTC), transcoded instead of decoded.
        return basis(&bytes);
    };
    let v = format.value();
    let layout = match v {
        9 => Layout::R8,
        16 => Layout::Rg8,
        23 | 29 => Layout::Rgb8,
        37 | 43 => Layout::Rgba8,
        44 | 50 => Layout::Bgra8,
        97 => Layout::Rgba16F,
        109 => Layout::Rgba32F,
        131..=134 => Layout::Bc1,
        135 | 136 => Layout::Bc2,
        137 | 138 => Layout::Bc3,
        139 | 140 => Layout::Bc4,
        141 | 142 => Layout::Bc5,
        143 => Layout::Bc6 { signed: false },
        144 => Layout::Bc6 { signed: true },
        145 | 146 => Layout::Bc7,
        147 | 148 => Layout::Etc2Rgb,
        149 | 150 => Layout::Etc2Rgba1,
        151 | 152 => Layout::Etc2Rgba8,
        153 | 154 => Layout::EacR,
        155 | 156 => Layout::EacRg,
        157..=184 => {
            const BLOCKS: [(usize, usize); 14] = [
                (4, 4),
                (5, 4),
                (5, 5),
                (6, 5),
                (6, 6),
                (8, 5),
                (8, 6),
                (8, 8),
                (10, 5),
                (10, 6),
                (10, 8),
                (10, 10),
                (12, 10),
                (12, 12),
            ];
            let (bw, bh) = BLOCKS[((v - 157) / 2) as usize];
            Layout::Astc { bw, bh }
        }
        _ => bail!("KTX2 format {format:?} isn't supported yet"),
    };
    let level = reader.levels().next().context("KTX2 file has no image data")?;
    let data: std::borrow::Cow<'_, [u8]> = match header.supercompression_scheme {
        None => level.data.into(),
        Some(s) if s == ktx2::SupercompressionScheme::Zstandard => {
            let len = usize::try_from(level.uncompressed_byte_length)?;
            ensure!(len <= MAX_PIXELS * 16, "implausible KTX2 level size");
            zstd::bulk::decompress(level.data, len)?.into()
        }
        Some(s) if s == ktx2::SupercompressionScheme::ZLIB => {
            let mut out = Vec::new();
            flate2::read::ZlibDecoder::new(level.data).take(MAX_PIXELS as u64 * 16).read_to_end(&mut out)?;
            out.into()
        }
        Some(_) => return basis(&bytes),
    };
    let image = decode_layout(&data, header.pixel_width, header.pixel_height.max(1), layout)?;
    let mut facts = Map::new();
    facts.insert("gpu_format".into(), json!(format!("{format:?}")));
    facts.insert("mip_levels".into(), json!(header.level_count.max(1)));
    if header.face_count == 6 {
        facts.insert("cubemap".into(), json!(true));
    }
    if header.layer_count > 1 {
        facts.insert("array_layers".into(), json!(header.layer_count));
    }
    Ok(Decoded { image, facts })
}

/// Basis Universal (`.basis`, or KTX2 holding ETC1S/UASTC): transcoded with the pure-Rust `basisu` to RGBA8, or to
/// half floats for the HDR formats.
fn basis(bytes: &[u8]) -> Result<Decoded> {
    use basisu::{DecodeFlags, SourceFormat, TargetFormat, Transcoder};
    let tex = Transcoder::new(bytes).map_err(|e| anyhow::anyhow!("not a readable Basis Universal texture: {e:?}"))?;
    let (w, h) = tex.base_dimensions();
    check_size(w, h)?;
    let source = tex.source_format();
    let hdr = matches!(source, SourceFormat::UastcHdr4x4 | SourceFormat::AstcHdr6x6 | SourceFormat::UastcHdr6x6);
    let target = if hdr { TargetFormat::RgbaHalf } else { TargetFormat::Rgba32 };
    let data =
        tex.transcode(0, target, DecodeFlags::NONE).map_err(|e| anyhow::anyhow!("Basis transcoding failed: {e:?}"))?;
    let image = if hdr {
        let v: Vec<f32> = data.as_chunks::<2>().0.iter().map(|c| half(u16::from_le_bytes(*c))).collect();
        DynamicImage::ImageRgba32F(image::Rgba32FImage::from_raw(w, h, v).context("Basis output size")?)
    } else {
        DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, data).context("Basis output size")?)
    };
    let mut facts = Map::new();
    let name = match source {
        SourceFormat::Etc1s => "Basis ETC1S".to_owned(),
        SourceFormat::UastcLdr => "Basis UASTC".to_owned(),
        SourceFormat::UastcHdr4x4 | SourceFormat::UastcHdr6x6 => "Basis UASTC HDR".to_owned(),
        other => format!("Basis {other:?}"),
    };
    facts.insert("gpu_format".into(), json!(name));
    facts.insert("mip_levels".into(), json!(tex.level_count().max(1)));
    if tex.face_count() == 6 {
        facts.insert("cubemap".into(), json!(true));
    }
    if tex.layer_count() > 1 {
        facts.insert("array_layers".into(), json!(tex.layer_count()));
    }
    Ok(Decoded { image, facts })
}

/// The machine's fonts, loaded once, with the generic families pointed at fonts that exist here.
fn font_db() -> std::sync::Arc<resvg::usvg::fontdb::Database> {
    static DB: std::sync::OnceLock<std::sync::Arc<resvg::usvg::fontdb::Database>> = std::sync::OnceLock::new();
    std::sync::Arc::clone(DB.get_or_init(|| {
        let mut db = resvg::usvg::fontdb::Database::new();
        db.load_system_fonts();
        let families: std::collections::HashSet<String> =
            db.faces().flat_map(|f| f.families.iter().map(|(n, _)| n.clone())).collect();
        let pick = |names: &[&str]| names.iter().find(|n| families.contains(**n)).map(|n| (*n).to_owned());
        let (sans, serif, mono) = (
            pick(&["DejaVu Sans", "Liberation Sans", "Noto Sans", "Arial", "Helvetica", "Segoe UI"]),
            pick(&["DejaVu Serif", "Liberation Serif", "Noto Serif", "Times New Roman", "Times"]),
            pick(&["DejaVu Sans Mono", "Liberation Mono", "Noto Sans Mono", "Courier New", "Menlo"]),
        );
        if let Some(f) = sans {
            db.set_sans_serif_family(f);
        }
        if let Some(f) = serif {
            db.set_serif_family(f);
        }
        if let Some(f) = mono {
            db.set_monospace_family(f);
        }
        tracing::debug!(faces = db.len(), "fonts for SVG text loaded");
        std::sync::Arc::new(db)
    }))
}

/// Render an SVG with its longest side at `size` pixels (vector art has no native size worth keeping small). Text is
/// drawn with the machine's fonts; families an SVG asks for that aren't installed fall back to a sans-serif.
pub(crate) fn svg(path: &Path, size: u32) -> Result<Decoded> {
    let data = std::fs::read(path)?;
    let opts = resvg::usvg::Options { fontdb: font_db(), font_family: "sans-serif".into(), ..Default::default() };
    let tree = resvg::usvg::Tree::from_data(&data, &opts)?;
    let s = tree.size();
    let scale = size as f32 / s.width().max(s.height()).max(1.0);
    let (w, h) = (((s.width() * scale).ceil() as u32).max(1), ((s.height() * scale).ceil() as u32).max(1));
    let mut pixmap = resvg::tiny_skia::Pixmap::new(w, h).context("SVG size")?;
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    // tiny-skia stores premultiplied RGBA.
    let rgba: Vec<u8> = pixmap
        .pixels()
        .iter()
        .flat_map(|p| {
            let c = p.demultiply();
            [c.red(), c.green(), c.blue(), c.alpha()]
        })
        .collect();
    let mut facts = Map::new();
    facts.insert("vector".into(), json!(true));
    facts.insert("svg_width".into(), json!(s.width().round() as u32));
    facts.insert("svg_height".into(), json!(s.height().round() as u32));
    Ok(Decoded { image: DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, rgba).context("size")?), facts })
}

fn aseprite(path: &Path) -> Result<Decoded> {
    let ase =
        asefile::AsepriteFile::read_file(path).map_err(|e| anyhow::anyhow!("not a readable Aseprite file: {e}"))?;
    ensure!(ase.num_frames() > 0, "Aseprite file has no frames");
    let frame = ase.frame(0).image();
    let (w, h) = (frame.width(), frame.height());
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, frame.into_raw()).context("size")?);
    let mut facts = Map::new();
    facts.insert("frames".into(), json!(ase.num_frames()));
    facts.insert("layers".into(), json!(ase.num_layers()));
    let tags: Vec<String> = (0..ase.num_tags()).map(|i| ase.tag(i).name().to_owned()).collect();
    if !tags.is_empty() {
        facts.insert("animations".into(), json!(tags));
    }
    Ok(Decoded { image, facts })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &tempfile::TempDir, name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let p = dir.path().join(name);
        std::fs::write(&p, bytes).unwrap();
        p
    }

    /// One BC1 block: color0 pure red, color1 pure blue, every texel index 0 (red) except the first row (1, blue).
    const BC1_BLOCK: [u8; 8] = [0x00, 0xF8, 0x1F, 0x00, 0b0101_0101, 0, 0, 0];

    #[test]
    fn dds_bc1_and_raw() {
        let dir = tempfile::tempdir().unwrap();
        let mut dds = ddsfile::Dds::new_dxgi(ddsfile::NewDxgiParams {
            height: 4,
            width: 4,
            depth: None,
            format: ddsfile::DxgiFormat::BC1_UNorm,
            mipmap_levels: None,
            array_layers: None,
            caps2: None,
            is_cubemap: false,
            resource_dimension: ddsfile::D3D10ResourceDimension::Texture2D,
            alpha_mode: ddsfile::AlphaMode::Unknown,
        })
        .unwrap();
        dds.get_mut_data(0).unwrap().copy_from_slice(&BC1_BLOCK);
        let mut bytes = Vec::new();
        dds.write(&mut bytes).unwrap();
        let d = decode(&write(&dir, "t.dds", &bytes), "dds").unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.get_pixel(0, 0).0, [0, 0, 255, 255], "first row is color1 (blue)");
        assert_eq!(img.get_pixel(2, 3).0, [255, 0, 0, 255], "the rest is color0 (red)");
        assert_eq!(d.facts["gpu_format"], "BC1_UNorm");

        let mut raw = ddsfile::Dds::new_dxgi(ddsfile::NewDxgiParams {
            height: 1,
            width: 2,
            depth: None,
            format: ddsfile::DxgiFormat::B8G8R8A8_UNorm,
            mipmap_levels: None,
            array_layers: None,
            caps2: None,
            is_cubemap: false,
            resource_dimension: ddsfile::D3D10ResourceDimension::Texture2D,
            alpha_mode: ddsfile::AlphaMode::Unknown,
        })
        .unwrap();
        raw.get_mut_data(0).unwrap().copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut bytes = Vec::new();
        raw.write(&mut bytes).unwrap();
        let img = decode(&write(&dir, "r.dds", &bytes), "dds").unwrap().image.to_rgba8();
        assert_eq!(img.get_pixel(1, 0).0, [7, 6, 5, 8]);
    }

    /// A minimal KTX2 container around one level.
    fn ktx2_file(vk_format: u32, w: u32, h: u32, scheme: u32, level: &[u8], raw_len: usize) -> Vec<u8> {
        let mut b = vec![0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A];
        for v in [vk_format, 1, w, h, 0, 0, 1, 1, scheme] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        // Header (80 bytes), one level index entry (24), a DFD holding only its length field (4), then the level.
        let dfd_at = 80 + 24u32;
        let data_at = dfd_at + 4;
        for v in [dfd_at, 4, data_at, 0] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.extend_from_slice(&[0; 16]); // no supercompression global data
        for v in [u64::from(data_at), level.len() as u64, raw_len as u64] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.extend_from_slice(&4u32.to_le_bytes());
        b.extend_from_slice(level);
        b
    }

    #[test]
    fn ktx2_raw_bc1_and_zstd() {
        let dir = tempfile::tempdir().unwrap();
        let rgba = [10u8, 20, 30, 255, 40, 50, 60, 255];
        let d = decode(&write(&dir, "a.ktx2", &ktx2_file(37, 2, 1, 0, &rgba, 8)), "ktx2").unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(1, 0).0, [40, 50, 60, 255]);
        assert_eq!(d.facts["gpu_format"], "R8G8B8A8_UNORM");

        let d = decode(&write(&dir, "b.ktx2", &ktx2_file(131, 4, 4, 0, &BC1_BLOCK, 8)), "ktx2").unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(3, 3).0, [255, 0, 0, 255]);

        let big: Vec<u8> = (0..64 * 64).flat_map(|i| [(i % 64) as u8, 0, 0, 255]).collect();
        let z = zstd::bulk::compress(&big, 3).unwrap();
        let d = decode(&write(&dir, "c.ktx2", &ktx2_file(37, 64, 64, 2, &z, big.len())), "ktx2").unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(5, 9).0, [5, 0, 0, 255]);

        // A KTX2 claiming Basis Universal with garbage inside goes to the transcoder, which refuses it cleanly.
        let basis = ktx2_file(0, 4, 4, 1, &[0; 16], 0);
        let err = decode(&write(&dir, "d.ktx2", &basis), "ktx2").err().unwrap();
        assert!(format!("{err}").contains("Basis"), "{err}");
        assert!(decode(&write(&dir, "e.basis", b"sB not really"), "basis").is_err());
    }

    #[test]
    fn svg_renders() {
        let dir = tempfile::tempdir().unwrap();
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10"><rect width="10" height="10" fill="#ff0000"/></svg>"##;
        let d = decode(&write(&dir, "i.svg", svg), "svg").unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.dimensions(), (1024, 512));
        assert_eq!(img.get_pixel(100, 100).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(900, 100).0[3], 0);
    }

    #[test]
    fn svg_text_renders_with_system_fonts() {
        if font_db().is_empty() {
            return; // no fonts on this machine: text can't be drawn
        }
        let dir = tempfile::tempdir().unwrap();
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="50"><text x="10" y="40" font-family="SomeFontNobodyHas, sans-serif" font-size="40" fill="#000">Hello</text></svg>"##;
        let img = decode(&write(&dir, "t.svg", svg), "svg").unwrap().image.to_rgba8();
        let inked = img.pixels().filter(|p| p[3] > 128).count();
        assert!(inked > 1000, "text drew {inked} pixels");
    }

    /// A one-frame, one-layer 2x2 RGBA Aseprite file, laid out from the format spec.
    fn tiny_aseprite() -> Vec<u8> {
        let mut layer = Vec::new();
        layer.extend_from_slice(&1u16.to_le_bytes()); // visible
        layer.extend_from_slice(&[0; 2 * 5]); // type, child level, default w/h, blend mode
        layer.push(255); // opacity
        layer.extend_from_slice(&[0; 3]);
        layer.extend_from_slice(&4u16.to_le_bytes());
        layer.extend_from_slice(b"Base");
        let mut cel = Vec::new();
        cel.extend_from_slice(&0u16.to_le_bytes()); // layer 0
        cel.extend_from_slice(&[0; 4]); // x, y
        cel.push(255);
        cel.extend_from_slice(&0u16.to_le_bytes()); // raw cel
        cel.extend_from_slice(&[0; 2 + 5]); // z-index, reserved
        cel.extend_from_slice(&2u16.to_le_bytes());
        cel.extend_from_slice(&2u16.to_le_bytes());
        cel.extend_from_slice(&[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255]);
        let chunk = |kind: u16, data: &[u8]| {
            let mut c = ((data.len() + 6) as u32).to_le_bytes().to_vec();
            c.extend_from_slice(&kind.to_le_bytes());
            c.extend_from_slice(data);
            c
        };
        let chunks = [chunk(0x2004, &layer), chunk(0x2005, &cel)].concat();
        let mut frame = ((16 + chunks.len()) as u32).to_le_bytes().to_vec();
        frame.extend_from_slice(&0xF1FAu16.to_le_bytes());
        frame.extend_from_slice(&2u16.to_le_bytes());
        frame.extend_from_slice(&100u16.to_le_bytes());
        frame.extend_from_slice(&[0; 2]);
        frame.extend_from_slice(&2u32.to_le_bytes());
        frame.extend(chunks);
        let mut h = ((128 + frame.len()) as u32).to_le_bytes().to_vec();
        h.extend_from_slice(&0xA5E0u16.to_le_bytes());
        for v in [1u16, 2, 2, 32] {
            h.extend_from_slice(&v.to_le_bytes()); // frames, width, height, depth
        }
        h.extend_from_slice(&1u32.to_le_bytes()); // flags
        h.extend_from_slice(&100u16.to_le_bytes());
        h.extend_from_slice(&[0; 8]);
        h.push(0); // transparent index
        h.extend_from_slice(&[0; 3]);
        h.extend_from_slice(&0u16.to_le_bytes()); // colors
        h.extend_from_slice(&[1, 1]); // pixel ratio
        h.resize(128, 0);
        h.extend(frame);
        h
    }

    #[test]
    fn aseprite_first_frame() {
        let dir = tempfile::tempdir().unwrap();
        let d = decode(&write(&dir, "s.aseprite", &tiny_aseprite()), "aseprite").unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.dimensions(), (2, 2));
        assert_eq!(img.get_pixel(1, 0).0, [0, 255, 0, 255]);
        assert_eq!(d.facts["frames"], 1);
    }
}
