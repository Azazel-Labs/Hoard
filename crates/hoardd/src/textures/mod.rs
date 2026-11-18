//! Texture formats `image` doesn't read: GPU containers (DDS, KTX and KTX2, PVR, Valve's VTF) with
//! block-compressed data, vector art (SVG, Illustrator), pixel-art sources (Aseprite), GIMP's layers and old paint formats (PCX). Each decodes to an ordinary image (mip 0 of the first layer or face) for
//! thumbnails, color analysis and "download as", plus the container's own facts.

mod aseprite;
mod dds;
mod embedded;
mod ktx1;
mod ktx2;
mod newer;
mod pcx;
mod pvr;
mod raw;
mod vector;
mod vtf;
mod xcf;

use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};
use serde_json::{Map, Value, json};

use self::aseprite::aseprite;
pub(crate) use self::aseprite::aseprite_sheet;
use self::dds::dds;
use self::ktx2::{basis, ktx2};
use self::vector::{illustrator, svg};
pub(crate) use self::vector::{pdf_thumbnail, pdf_thumbnail_of, vector};

/// Refuse to allocate decode buffers past this (16k x 16k).
const MAX_PIXELS: usize = 268_435_456;

/// A decoded texture and what its container says about itself.
pub(crate) struct Decoded {
    pub(crate) image: DynamicImage,
    pub(crate) facts: Map<String, Value>,
}

/// GPU containers, vector art and the rest handled in this module.
const EXTS: &[&str] =
    &["dds", "ktx", "ktx2", "basis", "pvr", "vtf", "svg", "svgz", "ai", "aseprite", "ase", "pcx", "cur", "xcf"];

/// Vector art: drawn at whatever size it's asked for.
pub(crate) const VECTOR: &[&str] = &["svg", "svgz", "ai"];

/// Whether [`decode`] reads `.ext`.
pub(crate) fn handles(ext: &str) -> bool {
    [EXTS, embedded::EXTS, raw::EXTS, newer::EXTS].iter().any(|list| list.contains(&ext))
}

pub(crate) fn decode(path: &Path, ext: &str) -> Result<Decoded> {
    match ext {
        e if embedded::EXTS.contains(&e) => embedded::decode(path, e),
        e if raw::EXTS.contains(&e) => raw::decode(&std::fs::read(path)?),
        e if newer::EXTS.contains(&e) => newer::decode(path, e),
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

#[cfg(test)]
mod tests {

    pub(super) fn write(dir: &tempfile::TempDir, name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let p = dir.path().join(name);
        std::fs::write(&p, bytes).unwrap();
        p
    }

    /// One BC1 block: color0 pure red, color1 pure blue, every texel index 0 (red) except the first row (1, blue).
    pub(super) const BC1_BLOCK: [u8; 8] = [0x00, 0xF8, 0x1F, 0x00, 0b0101_0101, 0, 0, 0];
}
