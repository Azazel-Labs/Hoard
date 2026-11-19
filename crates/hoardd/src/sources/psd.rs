//! Photoshop files (.psd, .psb): header facts, the layer count, the stored thumbnail and the flattened image.

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};

#[derive(Debug, Default)]
pub(crate) struct PsdInfo {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) channels: u16,
    pub(crate) depth: u16,
    pub(crate) color_mode: &'static str,
    pub(crate) psb: bool,
    pub(crate) layers: Option<u32>,
    pub(crate) thumbnail: Option<DynamicImage>,
}

fn be_u16(r: &mut impl Read) -> io::Result<u16> {
    let mut b = [0u8; 2];
    r.read_exact(&mut b)?;
    Ok(u16::from_be_bytes(b))
}
fn be_u32(r: &mut impl Read) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_be_bytes(b))
}
fn be_u64(r: &mut impl Read) -> io::Result<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_be_bytes(b))
}

pub(crate) fn read_psd(path: &Path) -> Result<PsdInfo> {
    let mut f = BufReader::new(File::open(path)?);
    let mut sig = [0u8; 4];
    f.read_exact(&mut sig)?;
    ensure!(&sig == b"8BPS", "not a PSD/PSB file");
    let version = be_u16(&mut f)?;
    ensure!(version == 1 || version == 2, "unknown PSD version {version}");
    f.seek_relative(6)?;
    let mut info = PsdInfo { psb: version == 2, ..Default::default() };
    info.channels = be_u16(&mut f)?;
    info.height = be_u32(&mut f)?;
    info.width = be_u32(&mut f)?;
    info.depth = be_u16(&mut f)?;
    info.color_mode = match be_u16(&mut f)? {
        0 => "bitmap",
        1 => "grayscale",
        2 => "indexed",
        3 => "rgb",
        4 => "cmyk",
        7 => "multichannel",
        8 => "duotone",
        9 => "lab",
        _ => "unknown",
    };

    let color_data = be_u32(&mut f)?;
    f.seek_relative(i64::from(color_data))?;

    // Image resources: "8BIM" id pascal-name size data, everything padded to even lengths.
    let res_len = u64::from(be_u32(&mut f)?);
    let res_start = f.stream_position()?;
    while f.stream_position()? + 12 <= res_start + res_len {
        let mut sig = [0u8; 4];
        f.read_exact(&mut sig)?;
        if &sig != b"8BIM" && &sig != b"MeSa" && &sig != b"AgHg" {
            break;
        }
        let id = be_u16(&mut f)?;
        let mut name_len = [0u8; 1];
        f.read_exact(&mut name_len)?;
        let name_total = 1 + i64::from(name_len[0]);
        f.seek_relative(name_len[0].into())?;
        if name_total % 2 == 1 {
            f.seek_relative(1)?;
        }
        let size = be_u32(&mut f)?;
        let padded = i64::from(size) + i64::from(size % 2);
        // 1036: JPEG RGB thumbnail (1033 is the Photoshop 4 variant with BGR order).
        if (id == 1036 || id == 1033) && size > 28 && size < 16 << 20 {
            let mut data = vec![0u8; size as usize];
            f.read_exact(&mut data)?;
            if size % 2 == 1 {
                f.seek_relative(1)?;
            }
            if let Ok(img) = image::load_from_memory_with_format(&data[28..], image::ImageFormat::Jpeg) {
                let mut rgba = img.to_rgba8();
                if id == 1033 {
                    for p in rgba.pixels_mut() {
                        p.0.swap(0, 2);
                    }
                }
                info.thumbnail = Some(DynamicImage::ImageRgba8(rgba));
            }
        } else {
            f.seek_relative(padded)?;
        }
    }
    f.seek(SeekFrom::Start(res_start + res_len))?;

    // Layer and mask information: total length, then layer info length, then a signed layer count.
    let lm_len = if info.psb { be_u64(&mut f)? } else { u64::from(be_u32(&mut f)?) };
    if lm_len >= 6 {
        let layer_info_len = if info.psb { be_u64(&mut f)? } else { u64::from(be_u32(&mut f)?) };
        if layer_info_len >= 2 {
            let count = be_u16(&mut f)? as i16;
            info.layers = Some(u32::from(count.unsigned_abs()));
        }
    } else {
        info.layers = Some(0);
    }
    Ok(info)
}

/// The flattened image a PSD/PSB stores after its layers (the "merged" or composite image), full size. Present
/// when the file was saved with Maximize Compatibility, which is Photoshop's default. RGB and grayscale, 8/16/32-bit.
#[expect(clippy::many_single_char_names, reason = "w/h/n read naturally in pixel code")]
pub(crate) fn psd_composite(path: &Path) -> Result<DynamicImage> {
    const MAX_PIXELS: u64 = 268_435_456; // 16k x 16k
    let mut f = BufReader::new(File::open(path)?);
    let mut sig = [0u8; 4];
    f.read_exact(&mut sig)?;
    ensure!(&sig == b"8BPS", "not a PSD/PSB file");
    let psb = be_u16(&mut f)? == 2;
    f.seek_relative(6)?;
    let channels = usize::from(be_u16(&mut f)?);
    let (h, w) = (be_u32(&mut f)?, be_u32(&mut f)?);
    let depth = be_u16(&mut f)?;
    let mode = be_u16(&mut f)?;
    ensure!(matches!(mode, 1 | 3), "only RGB and grayscale documents can be exported");
    ensure!(matches!(depth, 8 | 16 | 32), "{depth}-bit documents can't be exported");
    ensure!(u64::from(w) * u64::from(h) <= MAX_PIXELS && w > 0 && h > 0, "document is too large to export");
    let skip = |f: &mut BufReader<File>, n: u64| -> Result<()> { Ok(f.seek_relative(i64::try_from(n)?)?) };
    let color = u64::from(be_u32(&mut f)?);
    skip(&mut f, color)?;
    let resources = u64::from(be_u32(&mut f)?);
    skip(&mut f, resources)?;
    let layers = if psb { be_u64(&mut f)? } else { u64::from(be_u32(&mut f)?) };
    skip(&mut f, layers)?;

    // Image data: compression, then each channel's plane (row byte counts first when RLE).
    let compression = be_u16(&mut f)?;
    let (wu, hu) = (w as usize, h as usize);
    let bpc = usize::from(depth / 8);
    let row_bytes = wu * bpc;
    let wanted = if mode == 3 { channels.min(4) } else { channels.min(2) };
    ensure!(wanted >= 1 && (mode == 1 || wanted >= 3), "document has too few channels");
    let mut planes: Vec<Vec<u8>> = Vec::with_capacity(wanted);
    match compression {
        0 => {
            for _ in 0..wanted {
                let mut plane = vec![0u8; row_bytes * hu];
                f.read_exact(&mut plane)?;
                planes.push(plane);
            }
        }
        1 => {
            let mut counts = Vec::with_capacity(channels * hu);
            for _ in 0..channels * hu {
                counts.push(if psb { be_u32(&mut f)? as usize } else { usize::from(be_u16(&mut f)?) });
            }
            for c in 0..wanted {
                let mut plane = Vec::with_capacity(row_bytes * hu);
                for row in 0..hu {
                    let mut packed = vec![0u8; counts[c * hu + row]];
                    f.read_exact(&mut packed)?;
                    unpack_bits(&packed, row_bytes, &mut plane)?;
                }
                planes.push(plane);
            }
        }
        c => bail!("composite image compression {c} isn't supported (save with Maximize Compatibility)"),
    }
    // Interleave planes into pixels.
    let n = wu * hu;
    let interleave = |get: &dyn Fn(&[u8], usize) -> f32| -> Vec<f32> {
        let mut out = Vec::with_capacity(n * wanted);
        for i in 0..n {
            for p in &planes {
                out.push(get(p, i));
            }
        }
        out
    };
    Ok(match (depth, wanted) {
        (8, k) => {
            let mut px = Vec::with_capacity(n * k);
            for i in 0..n {
                px.extend(planes.iter().map(|p| p[i]));
            }
            match k {
                1 => DynamicImage::ImageLuma8(image::GrayImage::from_raw(w, h, px).context("size")?),
                2 => DynamicImage::ImageLumaA8(image::GrayAlphaImage::from_raw(w, h, px).context("size")?),
                3 => DynamicImage::ImageRgb8(image::RgbImage::from_raw(w, h, px).context("size")?),
                _ => DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, px).context("size")?),
            }
        }
        (16, k) => {
            let mut px: Vec<u16> = Vec::with_capacity(n * k);
            for i in 0..n {
                px.extend(planes.iter().map(|p| u16::from_be_bytes([p[i * 2], p[i * 2 + 1]])));
            }
            match k {
                1 => DynamicImage::ImageLuma16(image::ImageBuffer::from_raw(w, h, px).context("size")?),
                2 => DynamicImage::ImageLumaA16(image::ImageBuffer::from_raw(w, h, px).context("size")?),
                3 => DynamicImage::ImageRgb16(image::ImageBuffer::from_raw(w, h, px).context("size")?),
                _ => DynamicImage::ImageRgba16(image::ImageBuffer::from_raw(w, h, px).context("size")?),
            }
        }
        (_, k) => {
            let px = interleave(&|p, i| f32::from_be_bytes([p[i * 4], p[i * 4 + 1], p[i * 4 + 2], p[i * 4 + 3]]));
            // 32-bit documents are HDR; widen grayscale to RGB for the float image types.
            let rgba: Vec<f32> = px
                .chunks_exact(k)
                .flat_map(|c| match k {
                    1 => [c[0], c[0], c[0], 1.0],
                    2 => [c[0], c[0], c[0], c[1]],
                    3 => [c[0], c[1], c[2], 1.0],
                    _ => [c[0], c[1], c[2], c[3]],
                })
                .collect();
            DynamicImage::ImageRgba32F(image::Rgba32FImage::from_raw(w, h, rgba).context("size")?)
        }
    })
}

/// `PackBits` RLE, one row: appends exactly `len` bytes to `out`.
fn unpack_bits(src: &[u8], len: usize, out: &mut Vec<u8>) -> Result<()> {
    let start = out.len();
    let mut i = 0;
    while out.len() - start < len && i < src.len() {
        let n = src[i] as i8;
        i += 1;
        if n >= 0 {
            let count = n as usize + 1;
            out.extend_from_slice(src.get(i..i + count).context("RLE literal runs past the row")?);
            i += count;
        } else if n != -128 {
            let b = *src.get(i).context("RLE run without a byte")?;
            out.extend(std::iter::repeat_n(b, usize::try_from(1 - i16::from(n))?));
            i += 1;
        }
    }
    ensure!(out.len() - start == len, "RLE row decoded to {} bytes, expected {len}", out.len() - start);
    Ok(())
}
