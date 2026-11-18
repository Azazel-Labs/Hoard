//! Valve Texture Format (`.vtf`), the Source engine's textures. The mip levels are stored smallest first, each with
//! every frame, face and slice, so the full-size first frame is found by adding up the levels before it.

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};

use super::{Decoded, Layout, check_size, decode_layout, gpu_facts};

/// The cube map flag: six faces (seven in early versions, with a sphere map).
const ENVMAP: u32 = 0x4000;

pub(super) fn decode(bytes: &[u8]) -> Result<Decoded> {
    ensure!(bytes.starts_with(b"VTF\0"), "not a VTF file");
    let u16_at = |i: usize| bytes.get(i..).and_then(<[u8]>::first_chunk).map(|b| u16::from_le_bytes(*b));
    let u32_at = |i: usize| bytes.get(i..).and_then(<[u8]>::first_chunk).map(|b| u32::from_le_bytes(*b));
    let short = || anyhow::anyhow!("VTF header is cut short");
    let (major, minor) = (u32_at(4).ok_or_else(short)?, u32_at(8).ok_or_else(short)?);
    ensure!(major == 7, "VTF version {major}.{minor} isn't supported");
    let header = u32_at(12).ok_or_else(short)? as usize;
    let (width, height) = (u16_at(16).ok_or_else(short)?, u16_at(18).ok_or_else(short)?);
    let flags = u32_at(20).ok_or_else(short)?;
    let frames = usize::from(u16_at(24).ok_or_else(short)?.max(1));
    let first_frame = u16_at(26).ok_or_else(short)?;
    let format = u32_at(52).ok_or_else(short)?;
    let mips = u32::from(*bytes.get(56).ok_or_else(short)?).max(1);
    let low_format = u32_at(57).ok_or_else(short)?;
    let (low_w, low_h) =
        (usize::from(*bytes.get(61).ok_or_else(short)?), usize::from(*bytes.get(62).ok_or_else(short)?));
    let depth = if minor >= 2 { usize::from(u16_at(63).ok_or_else(short)?.max(1)) } else { 1 };
    let faces = match (flags & ENVMAP != 0, minor < 5 && first_frame == 0xFFFF) {
        (false, _) => 1,
        (true, false) => 6,
        (true, true) => 7,
    };

    let pixels = Pixels::of(format)?;
    let (w, h) = (usize::from(width), usize::from(height));
    check_size(w as u32, h as u32)?;
    // Where the full-size images start: after the low-res thumbnail, or where the resource list says.
    let start = if minor >= 3 {
        let count = u32_at(68).ok_or_else(short)? as usize;
        (0..count.min(32))
            .find_map(|i| {
                let at = 80 + i * 8;
                (bytes.get(at..at + 3)? == [0x30, 0, 0]).then(|| u32_at(at + 4)).flatten()
            })
            .context("VTF has no image data")? as usize
    } else {
        let low = if low_format == u32::MAX { 0 } else { Pixels::of(low_format)?.bytes(low_w, low_h) };
        header + low
    };
    let smaller: usize = (1..mips).map(|m| pixels.bytes((w >> m).max(1), (h >> m).max(1))).sum();
    let at = start + smaller * frames * faces * depth;
    let data = bytes.get(at..at + pixels.bytes(w, h)).context("VTF data is cut short")?;
    let image = pixels.decode(data, u32::from(width), u32::from(height))?;
    let mut known = gpu_facts(pixels.name(), mips);
    if frames > 1 {
        known.insert("frames".into(), frames.into());
    }
    Ok(Decoded { image, facts: known })
}

/// VTF's pixel formats: most are a [`Layout`], a few are swizzled or packed first.
#[derive(Clone, Copy)]
enum Pixels {
    Layout(Layout),
    Abgr8,
    Argb8,
    Rgb565,
    Bgr565,
    Bgra4,
    Bgra5551,
    A8,
}

impl Pixels {
    fn of(format: u32) -> Result<Self> {
        Ok(match format {
            0 => Self::Layout(Layout::Rgba8),
            1 => Self::Abgr8,
            2 => Self::Layout(Layout::Rgb8),
            3 => Self::Layout(Layout::Bgr8),
            4 => Self::Rgb565,
            5 => Self::Layout(Layout::R8),
            6 => Self::Layout(Layout::La8),
            8 => Self::A8,
            11 => Self::Argb8,
            12 => Self::Layout(Layout::Bgra8),
            13 | 20 => Self::Layout(Layout::Bc1),
            14 => Self::Layout(Layout::Bc2),
            15 => Self::Layout(Layout::Bc3),
            16 => Self::Layout(Layout::Bgrx8),
            17 => Self::Bgr565,
            19 => Self::Bgra4,
            21 => Self::Bgra5551,
            22 => Self::Layout(Layout::Rg8),
            24 => Self::Layout(Layout::Rgba16F),
            _ => bail!("VTF format {format} isn't supported yet"),
        })
    }

    fn bytes(self, w: usize, h: usize) -> usize {
        match self {
            Self::Layout(l) => l.bytes(w, h),
            Self::Abgr8 | Self::Argb8 => w * h * 4,
            Self::Rgb565 | Self::Bgr565 | Self::Bgra4 | Self::Bgra5551 => w * h * 2,
            Self::A8 => w * h,
        }
    }

    fn name(self) -> String {
        match self {
            Self::Layout(l) => l.name(),
            Self::Abgr8 => "ABGR8".into(),
            Self::Argb8 => "ARGB8".into(),
            Self::Rgb565 => "RGB565".into(),
            Self::Bgr565 => "BGR565".into(),
            Self::Bgra4 => "BGRA4".into(),
            Self::Bgra5551 => "BGRA5551".into(),
            Self::A8 => "A8".into(),
        }
    }

    fn decode(self, data: &[u8], w: u32, h: u32) -> Result<DynamicImage> {
        let rgba: Vec<u8> = match self {
            Self::Layout(l) => return decode_layout(data, w, h, l),
            Self::Abgr8 => data.as_chunks::<4>().0.iter().flat_map(|p| [p[3], p[2], p[1], p[0]]).collect(),
            Self::Argb8 => data.as_chunks::<4>().0.iter().flat_map(|p| [p[1], p[2], p[3], p[0]]).collect(),
            Self::A8 => data.iter().flat_map(|&a| [255, 255, 255, a]).collect(),
            Self::Rgb565 | Self::Bgr565 | Self::Bgra4 | Self::Bgra5551 => data
                .as_chunks::<2>()
                .0
                .iter()
                .flat_map(|p| {
                    let v = u16::from_le_bytes(*p);
                    let scale = |x: u16, bits: u32| (u32::from(x) * 255 / ((1 << bits) - 1)) as u8;
                    match self {
                        Self::Rgb565 => [scale(v & 31, 5), scale((v >> 5) & 63, 6), scale(v >> 11, 5), 255],
                        Self::Bgr565 => [scale(v >> 11, 5), scale((v >> 5) & 63, 6), scale(v & 31, 5), 255],
                        Self::Bgra4 => {
                            [scale((v >> 8) & 15, 4), scale((v >> 4) & 15, 4), scale(v & 15, 4), scale(v >> 12, 4)]
                        }
                        _ => [
                            scale((v >> 10) & 31, 5),
                            scale((v >> 5) & 31, 5),
                            scale(v & 31, 5),
                            if v >> 15 == 1 { 255 } else { 0 },
                        ],
                    }
                })
                .collect(),
        };
        Ok(DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, rgba).context("VTF data is cut short")?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A version 7.2 file: `format` at `w` x `h` with `mips` levels, a 1x1 BC1 thumbnail, and each level's data.
    fn vtf(format: u32, w: u16, h: u16, levels: &[&[u8]]) -> Vec<u8> {
        let mut b = b"VTF\0".to_vec();
        for v in [7u32, 2, 80] {
            b.extend(v.to_le_bytes());
        }
        b.extend(w.to_le_bytes());
        b.extend(h.to_le_bytes());
        b.extend(0u32.to_le_bytes());
        b.extend(1u16.to_le_bytes());
        b.extend(0u16.to_le_bytes());
        b.resize(52, 0);
        b.extend(format.to_le_bytes());
        b.push(levels.len() as u8);
        b.extend(13u32.to_le_bytes());
        b.extend([1, 1]);
        b.extend(1u16.to_le_bytes());
        b.resize(80, 0);
        b.extend([0xAA; 8]);
        for l in levels.iter().rev() {
            b.extend(*l);
        }
        b
    }

    #[test]
    fn finds_the_largest_level_after_the_smaller_ones() {
        // 2x2 BGRA with a 1x1 level stored before it.
        let full = [10, 20, 30, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255];
        let d = decode(&vtf(12, 2, 2, &[&full, &[9, 9, 9, 9]])).unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(0, 0).0, [30, 20, 10, 255]);
        assert_eq!(d.facts["mip_levels"], 2);
        assert_eq!(d.facts["gpu_format"], "BGRA8");

        let d = decode(&vtf(13, 4, 4, &[&crate::textures::tests::BC1_BLOCK])).unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(0, 0).0, [0, 0, 255, 255]);
        // 565: pure green.
        let d = decode(&vtf(4, 1, 1, &[&0x07E0u16.to_le_bytes()])).unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(0, 0).0, [0, 255, 0, 255]);
        assert!(decode(&vtf(99, 1, 1, &[&[0]])).is_err());
    }
}
