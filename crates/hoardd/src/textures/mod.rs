//! Texture formats `image` doesn't read: GPU containers (DDS, KTX and KTX2, PVR, Valve's VTF) with
//! block-compressed data, vector art (SVG, Illustrator), pixel-art sources (Aseprite), GIMP's layers and old paint formats (PCX). Each decodes to an ordinary image (mip 0 of the first layer or face) for
//! thumbnails, color analysis and "download as", plus the container's own facts.

mod embedded;
mod ktx1;
mod pcx;
mod pvr;
mod raw;
mod vtf;
mod xcf;

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

/// GPU containers, vector art and the rest handled in this file.
const EXTS: &[&str] =
    &["dds", "ktx", "ktx2", "basis", "pvr", "vtf", "svg", "svgz", "ai", "aseprite", "ase", "pcx", "cur", "xcf"];

/// Vector art: drawn at whatever size it's asked for.
pub(crate) const VECTOR: &[&str] = &["svg", "svgz", "ai"];

/// Whether [`decode`] reads `.ext`.
pub(crate) fn handles(ext: &str) -> bool {
    EXTS.contains(&ext) || embedded::EXTS.contains(&ext) || raw::EXTS.contains(&ext)
}

pub(crate) fn decode(path: &Path, ext: &str) -> Result<Decoded> {
    match ext {
        e if embedded::EXTS.contains(&e) => embedded::decode(path, e),
        e if raw::EXTS.contains(&e) => raw::decode(&std::fs::read(path)?),
        "dds" => dds(path),
        "ktx" => ktx1::decode(&std::fs::read(path)?),
        "ktx2" => ktx2(path),
        "pvr" => pvr::decode(&std::fs::read(path)?),
        "vtf" => vtf::decode(&std::fs::read(path)?),
        "pcx" => pcx::decode(&std::fs::read(path)?),
        "xcf" => xcf::decode(&std::fs::read(path)?),
        // Cursors are icons with a hotspot; the image library only knows them by the icon's signature.
        "cur" => Ok(Decoded {
            image: image::load_from_memory_with_format(&std::fs::read(path)?, image::ImageFormat::Ico)?,
            facts: Map::new(),
        }),
        "basis" => basis(&std::fs::read(path)?),
        "svg" | "svgz" => svg(path, 1024),
        // Browsers can't show Illustrator files, so this drawing is their preview too.
        "ai" => illustrator(path, crate::previews::LITE_MAX),
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
    Etc1,
    Etc2Rgb,
    Etc2Rgba1,
    Etc2Rgba8,
    EacR,
    EacRg,
    Astc { bw: usize, bh: usize },
    Pvrtc { two_bit: bool },
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

impl Layout {
    /// What the format is called.
    fn name(self) -> String {
        match self {
            Self::Bc1 => "BC1".into(),
            Self::Bc2 => "BC2".into(),
            Self::Bc3 => "BC3".into(),
            Self::Bc4 => "BC4".into(),
            Self::Bc5 => "BC5".into(),
            Self::Bc6 { .. } => "BC6H".into(),
            Self::Bc7 => "BC7".into(),
            Self::Etc1 => "ETC1".into(),
            Self::Etc2Rgb => "ETC2".into(),
            Self::Etc2Rgba1 => "ETC2 (1-bit alpha)".into(),
            Self::Etc2Rgba8 => "ETC2 (alpha)".into(),
            Self::EacR => "EAC R11".into(),
            Self::EacRg => "EAC RG11".into(),
            Self::Astc { bw, bh } => format!("ASTC {bw}x{bh}"),
            Self::Pvrtc { two_bit } => format!("PVRTC {}bpp", if two_bit { 2 } else { 4 }),
            other => format!("{other:?}").to_uppercase(),
        }
    }

    /// Bytes of a `w` x `h` image in this layout.
    fn bytes(self, w: usize, h: usize) -> usize {
        let blocks = |bw: usize, bh: usize, size: usize| w.div_ceil(bw) * h.div_ceil(bh) * size;
        match self {
            Self::Bc1 | Self::Bc4 | Self::Etc1 | Self::Etc2Rgb | Self::Etc2Rgba1 | Self::EacR => blocks(4, 4, 8),
            Self::Bc2 | Self::Bc3 | Self::Bc5 | Self::Bc6 { .. } | Self::Bc7 | Self::Etc2Rgba8 | Self::EacRg => {
                blocks(4, 4, 16)
            }
            Self::Astc { bw, bh } => blocks(bw, bh, 16),
            // PVRTC works on at least two blocks each way.
            Self::Pvrtc { two_bit: true } => w.div_ceil(8).max(2) * h.div_ceil(4).max(2) * 8,
            Self::Pvrtc { two_bit: false } => w.div_ceil(4).max(2) * h.div_ceil(4).max(2) * 8,
            Self::R8 => w * h,
            Self::Rg8 | Self::La8 => w * h * 2,
            Self::Rgb8 | Self::Bgr8 => w * h * 3,
            Self::Rgba8 | Self::Bgra8 | Self::Bgrx8 => w * h * 4,
            Self::Rgba16F => w * h * 8,
            Self::Rgba32F => w * h * 16,
        }
    }
}

/// ASTC block sizes in the order KTX's and PVR's format numbers list them.
const ASTC_BLOCKS: [(usize, usize); 14] = [
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

/// Facts every GPU container gives.
fn gpu_facts(format: impl Into<String>, mips: u32) -> Map<String, Value> {
    let mut facts = Map::new();
    facts.insert("gpu_format".into(), json!(format.into()));
    facts.insert("mip_levels".into(), json!(mips.max(1)));
    facts
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
        Layout::Etc1 => block(t::decode_etc1)?,
        Layout::Etc2Rgb => block(t::decode_etc2_rgb)?,
        Layout::Pvrtc { two_bit: true } => block(t::decode_pvrtc_2bpp)?,
        Layout::Pvrtc { two_bit: false } => block(t::decode_pvrtc_4bpp)?,
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
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, rgba).context("size")?);
    Ok(Decoded { image, facts: vector_facts(s.width(), s.height()) })
}

/// Vector art at `size` pixels on its longer side.
pub(crate) fn vector(path: &Path, ext: &str, size: u32) -> Result<Decoded> {
    match ext {
        "ai" => illustrator(path, size),
        _ => svg(path, size),
    }
}

/// What's said of vector art: that it is, and its own size (in its units), which is its `width` and `height`.
fn vector_facts(width: f32, height: f32) -> Map<String, Value> {
    let mut facts = Map::new();
    facts.insert("vector".into(), json!(true));
    facts.insert("vector_width".into(), json!(width.round() as u32));
    facts.insert("vector_height".into(), json!(height.round() as u32));
    facts
}

/// Adobe Illustrator art. Since Illustrator 9 a file is a PDF too, and its first artboard is drawn like a PDF page.
/// Older files (PostScript), and newer ones saved without their PDF content, show the preview Illustrator keeps in
/// their XMP metadata instead.
pub(crate) fn illustrator(path: &Path, size: u32) -> Result<Decoded> {
    let data = std::fs::read(path)?;
    if data.starts_with(b"%PDF")
        && let Some(drawn) = pdf_page(data.clone(), size)?
    {
        return Ok(drawn);
    }
    let image =
        xmp_thumbnail(&data).context("this Illustrator file has neither a drawing Hoard can read nor a preview")?;
    let mut facts = vector_facts(image.width() as f32, image.height() as f32);
    // The preview is only small: its size isn't the art's.
    facts.remove("vector_width");
    facts.remove("vector_height");
    Ok(Decoded { image, facts })
}

/// A PDF's first page drawn `size` pixels on its longer side, over transparency. `None` for Illustrator's stand-in
/// page ("saved without PDF Content").
fn pdf_page(data: Vec<u8>, size: u32) -> Result<Option<Decoded>> {
    use hayro::hayro_interpret::InterpreterSettings;
    use hayro::vello_cpu::color::palette::css::TRANSPARENT;
    use hayro::vello_cpu::peniko::ImageAlphaType;
    let pdf = hayro::hayro_syntax::Pdf::new(data).map_err(|e| anyhow::anyhow!("not a readable PDF: {e:?}"))?;
    let page = pdf.pages().first().context("the PDF has no pages")?;
    if page.page_stream().is_some_and(|s| s.windows(19).any(|w| w == b"without PDF Content")) {
        return Ok(None);
    }
    let (w, h) = page.render_dimensions();
    ensure!(w >= 1.0 && h >= 1.0, "the page has no size");
    let scale = size.min(u32::from(u16::MAX)) as f32 / w.max(h);
    let settings = hayro::PixmapSettings { x_scale: scale, y_scale: scale, bg_color: TRANSPARENT };
    let pixmap = hayro::render(
        page,
        &hayro::RenderCache::new(),
        &InterpreterSettings::default(),
        &hayro::RenderSettings::default(),
        &settings,
    );
    let (pw, ph) = (u32::from(pixmap.width()), u32::from(pixmap.height()));
    ensure!(pw > 0 && ph > 0, "the page drew nothing");
    let rgba = pixmap.take_rgba8(ImageAlphaType::Alpha);
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(pw, ph, rgba).context("size")?);
    Ok(Some(Decoded { image, facts: vector_facts(w, h) }))
}

/// The JPEG preview an Adobe file keeps in its XMP metadata (`xmpGImg:image`, base64 with `&#xA;` line breaks).
fn xmp_thumbnail(data: &[u8]) -> Option<DynamicImage> {
    use base64::Engine as _;
    const OPEN: &[u8] = b"<xmpGImg:image>";
    let start = data.windows(OPEN.len()).position(|w| w == OPEN)? + OPEN.len();
    let len = data[start..].iter().position(|&b| b == b'<')?;
    let text = std::str::from_utf8(&data[start..start + len]).ok()?.replace("&#xA;", "");
    let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let jpeg = base64::engine::general_purpose::STANDARD.decode(text).ok()?;
    image::load_from_memory_with_format(&jpeg, image::ImageFormat::Jpeg).ok()
}

fn aseprite(path: &Path) -> Result<Decoded> {
    let ase =
        asefile::AsepriteFile::read_file(path).map_err(|e| anyhow::anyhow!("not a readable Aseprite file: {e}"))?;
    ensure!(ase.num_frames() > 0, "Aseprite file has no frames");
    let frame = ase.frame(0).image();
    let (w, h) = (frame.width(), frame.height());
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, frame.into_raw()).context("size")?);
    let mut facts = Map::new();
    let frames = ase.num_frames();
    facts.insert("frames".into(), json!(frames));
    facts.insert("layers".into(), json!(ase.num_layers()));
    if frames > 1 {
        // How long each frame shows, for playing it (see `aseprite_sheet`).
        let ms: Vec<u32> = (0..frames).map(|i| ase.frame(i).duration()).collect();
        facts.insert("duration_ms".into(), json!(ms.iter().map(|&m| u64::from(m)).sum::<u64>()));
        facts.insert("frame_ms".into(), json!(ms));
    }
    // Its tags are its animations: a run of frames each ("walk", "idle"), played forward, backward or there and back.
    let tags: Vec<Value> = (0..ase.num_tags())
        .map(|i| {
            let t = ase.tag(i);
            let direction = match t.animation_direction() {
                asefile::AnimationDirection::Forward => "forward",
                asefile::AnimationDirection::Reverse => "reverse",
                asefile::AnimationDirection::PingPong => "pingpong",
            };
            json!({ "name": t.name(), "from": t.from_frame(), "to": t.to_frame(), "direction": direction })
        })
        .collect();
    if !tags.is_empty() {
        facts.insert("animations".into(), json!(tags));
    }
    Ok(Decoded { image, facts })
}

/// Most frames a sheet holds, and most pixels across them.
const SHEET_MAX_FRAMES: usize = 1024;
const SHEET_MAX_PIXELS: u64 = 32 << 20;

/// Every frame of an Aseprite file side by side, in as many rows as columns (the last row may be short), as a PNG:
/// what a browser plays it from, with the frame times in the asset's `frame_ms`. None for a single frame, or more
/// than a sheet holds.
pub(crate) fn aseprite_sheet(path: &Path) -> Result<Option<Vec<u8>>> {
    use image::ImageEncoder as _;
    let ase =
        asefile::AsepriteFile::read_file(path).map_err(|e| anyhow::anyhow!("not a readable Aseprite file: {e}"))?;
    let n = ase.num_frames() as usize;
    let (w, h) = (ase.width() as u32, ase.height() as u32);
    if !(2..=SHEET_MAX_FRAMES).contains(&n) || n as u64 * u64::from(w) * u64::from(h) > SHEET_MAX_PIXELS {
        return Ok(None);
    }
    let cols = (n as f64).sqrt().ceil() as u32;
    let rows = (n as u32).div_ceil(cols);
    let mut sheet = RgbaImage::new(w * cols, h * rows);
    for i in 0..n as u32 {
        let f = ase.frame(i).image();
        let f = RgbaImage::from_raw(f.width(), f.height(), f.into_raw()).context("frame size")?;
        image::imageops::replace(&mut sheet, &f, i64::from(i % cols * w), i64::from(i / cols * h));
    }
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out).write_image(
        sheet.as_raw(),
        sheet.width(),
        sheet.height(),
        image::ExtendedColorType::Rgba8,
    )?;
    Ok(Some(out))
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
    pub(super) const BC1_BLOCK: [u8; 8] = [0x00, 0xF8, 0x1F, 0x00, 0b0101_0101, 0, 0, 0];

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

    /// A PDF with one 20x10 page whose content is `content`, with a correct cross-reference table.
    fn pdf(content: &str) -> Vec<u8> {
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_owned(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 20 10] /Contents 4 0 R >>".to_owned(),
            format!(
                "<< /Length {} >>
stream
{content}
endstream",
                content.len()
            ),
        ];
        let mut out = b"%PDF-1.5
"
        .to_vec();
        let mut offsets = Vec::new();
        for (i, o) in objects.iter().enumerate() {
            offsets.push(out.len());
            out.extend(
                format!(
                    "{} 0 obj
{o}
endobj
",
                    i + 1
                )
                .bytes(),
            );
        }
        let xref = out.len();
        out.extend(
            format!(
                "xref
0 {}
0000000000 65535 f 
",
                objects.len() + 1
            )
            .bytes(),
        );
        for o in offsets {
            out.extend(
                format!(
                    "{o:010} 00000 n 
"
                )
                .bytes(),
            );
        }
        out.extend(
            format!(
                "trailer
<< /Size {} /Root 1 0 R >>
startxref
{xref}
%%EOF
",
                objects.len() + 1
            )
            .bytes(),
        );
        out
    }

    #[test]
    fn illustrator_art_is_drawn_from_its_pdf() {
        let dir = tempfile::tempdir().unwrap();
        // Red over the left half of the artboard.
        let d = illustrator(&write(&dir, "icons.ai", &pdf("1 0 0 rg 0 0 10 10 re f")), 200).unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.dimensions(), (200, 100));
        assert_eq!(img.get_pixel(40, 50).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(160, 50).0[3], 0, "the rest is see-through");
        assert_eq!((d.facts["vector_width"].clone(), d.facts["vector_height"].clone()), (json!(20), json!(10)));
        assert_eq!(decode(&write(&dir, "big.ai", &pdf("0 0 1 rg 0 0 20 10 re f")), "ai").unwrap().image.width(), 2048);
    }

    #[test]
    fn illustrator_art_without_a_drawing_shows_its_preview() {
        use base64::Engine as _;
        let mut jpeg = Vec::new();
        DynamicImage::ImageRgb8(image::RgbImage::from_pixel(8, 4, image::Rgb([0, 200, 0])))
            .write_to(&mut std::io::Cursor::new(&mut jpeg), image::ImageFormat::Jpeg)
            .unwrap();
        let b64 = base64::engine::general_purpose::STANDARD.encode(&jpeg);
        let (a, b) = b64.split_at(b64.len() / 2);
        let xmp = format!("<xmpGImg:width>8</xmpGImg:width><xmpGImg:image>{a}&#xA;{b}</xmpGImg:image>");
        let dir = tempfile::tempdir().unwrap();
        // Illustrator 8 and earlier: PostScript.
        let old = format!(
            "%!PS-Adobe-3.0
%%Creator: Adobe Illustrator(R) 8.0
{xmp}
%%EOF
"
        );
        let d = illustrator(&write(&dir, "old.ai", old.as_bytes()), 512).unwrap();
        assert_eq!((d.image.width(), d.image.height()), (8, 4));
        assert!(d.image.to_rgb8().get_pixel(4, 2).0[1] > 150);
        assert!(!d.facts.contains_key("vector_width"));
        // Saved without PDF content: the page only says so.
        let mut stand_in = pdf("BT /F1 12 Tf (saved without PDF Content.) Tj ET");
        stand_in.extend(xmp.bytes());
        assert_eq!(
            illustrator(&write(&dir, "no-pdf.ai", &stand_in), 512).unwrap().image.to_rgba8().dimensions(),
            (8, 4)
        );
        assert!(
            illustrator(
                &write(
                    &dir,
                    "junk.ai",
                    b"%!PS-Adobe-3.0
"
                ),
                512
            )
            .is_err()
        );
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
        aseprite_file(&[[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255]], &[])
    }

    /// A one-layer 2x2 RGBA Aseprite file of these frames, 100 ms each but the second (150 ms), and tags
    /// (name, from, to, direction).
    fn aseprite_file(frames: &[[u8; 16]], tags: &[(&str, u16, u16, u8)]) -> Vec<u8> {
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
        let chunk = |kind: u16, data: &[u8]| {
            let mut c = ((data.len() + 6) as u32).to_le_bytes().to_vec();
            c.extend_from_slice(&kind.to_le_bytes());
            c.extend_from_slice(data);
            c
        };
        let mut tag_chunk = (tags.len() as u16).to_le_bytes().to_vec();
        tag_chunk.extend_from_slice(&[0; 8]);
        for (name, from, to, dir) in tags {
            tag_chunk.extend_from_slice(&from.to_le_bytes());
            tag_chunk.extend_from_slice(&to.to_le_bytes());
            tag_chunk.push(*dir);
            tag_chunk.extend_from_slice(&[0; 2 + 6 + 4]); // repeat, reserved, color
            tag_chunk.extend_from_slice(&(name.len() as u16).to_le_bytes());
            tag_chunk.extend_from_slice(name.as_bytes());
        }
        let mut body = Vec::new();
        for (i, pixels) in frames.iter().enumerate() {
            let mut chunks = Vec::new();
            if i == 0 {
                chunks.push(chunk(0x2004, &layer));
                if !tags.is_empty() {
                    chunks.push(chunk(0x2018, &tag_chunk));
                }
            }
            chunks.push(chunk(0x2005, &[&cel[..], pixels].concat()));
            let count = chunks.len();
            let chunks = chunks.concat();
            let mut frame = ((16 + chunks.len()) as u32).to_le_bytes().to_vec();
            frame.extend_from_slice(&0xF1FAu16.to_le_bytes());
            frame.extend_from_slice(&(count as u16).to_le_bytes());
            frame.extend_from_slice(&(if i == 1 { 150u16 } else { 100 }).to_le_bytes());
            frame.extend_from_slice(&[0; 2]);
            frame.extend_from_slice(&(count as u32).to_le_bytes());
            frame.extend(chunks);
            body.extend(frame);
        }
        let mut h = ((128 + body.len()) as u32).to_le_bytes().to_vec();
        h.extend_from_slice(&0xA5E0u16.to_le_bytes());
        for v in [frames.len() as u16, 2, 2, 32] {
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
        h.extend(body);
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
        assert!(!d.facts.contains_key("frame_ms"), "a still has no timing");
    }

    #[test]
    fn aseprite_frames_play_from_a_sheet() {
        let [r, g, b] = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]].map(|c: [u8; 4]| {
            let mut f = [0; 16];
            f[..4].copy_from_slice(&c);
            f
        });
        let dir = tempfile::tempdir().unwrap();
        let path = write(&dir, "s.aseprite", &aseprite_file(&[r, g, b], &[("walk", 1, 2, 2)]));
        let d = decode(&path, "aseprite").unwrap();
        assert_eq!(d.facts["frames"], 3);
        assert_eq!(d.facts["frame_ms"], json!([100, 150, 100]));
        assert_eq!(d.facts["duration_ms"], 350);
        assert_eq!(d.facts["animations"], json!([{ "name": "walk", "from": 1, "to": 2, "direction": "pingpong" }]));

        // Three frames: two across, two down, in order.
        let sheet = image::load_from_memory(&aseprite_sheet(&path).unwrap().unwrap()).unwrap().to_rgba8();
        assert_eq!(sheet.dimensions(), (4, 4));
        assert_eq!(sheet.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(sheet.get_pixel(2, 0).0, [0, 255, 0, 255]);
        assert_eq!(sheet.get_pixel(0, 2).0, [0, 0, 255, 255]);
        assert_eq!(sheet.get_pixel(2, 2).0[3], 0, "the empty corner");

        let still = write(&dir, "still.aseprite", &tiny_aseprite());
        assert!(aseprite_sheet(&still).unwrap().is_none());
    }
}
