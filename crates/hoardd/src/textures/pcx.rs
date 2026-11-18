//! PCX (`.pcx`), the paint format from the DOS days, still in old game art: run-length encoded scanlines, one
//! plane per channel or packed palette indices, with a 16-color palette in the header or 256 colors at the end.

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};
use serde_json::Map;

use super::{Decoded, check_size};

const HEADER: usize = 128;

pub(super) fn decode(bytes: &[u8]) -> Result<Decoded> {
    ensure!(bytes.len() > HEADER && bytes[0] == 0x0A && bytes[2] <= 1, "not a PCX file");
    let u16_at = |i: usize| u16::from_le_bytes([bytes[i], bytes[i + 1]]);
    let bits = usize::from(bytes[3]);
    let (left, top, right, bottom) = (u16_at(4), u16_at(6), u16_at(8), u16_at(10));
    ensure!(right >= left && bottom >= top, "PCX size is negative");
    let (width, height) = (u32::from(right - left) + 1, u32::from(bottom - top) + 1);
    let (wu, hu) = check_size(width, height)?;
    let planes = usize::from(bytes[65]);
    let line = usize::from(u16_at(66));
    ensure!(line * 8 >= wu * bits, "PCX lines are shorter than the image");
    let rle = bytes[2] == 1;

    // Unpack every scanline: `planes` runs of `line` bytes each.
    let stride = line * planes;
    let mut rows = Vec::with_capacity(stride * hu);
    let mut at = HEADER;
    while rows.len() < stride * hu {
        let b = *bytes.get(at).context("PCX data is cut short")?;
        at += 1;
        if rle && b >= 0xC0 {
            let v = *bytes.get(at).context("PCX data is cut short")?;
            at += 1;
            rows.extend(std::iter::repeat_n(v, usize::from(b & 0x3F)));
        } else {
            rows.push(b);
        }
    }
    rows.truncate(stride * hu);

    let ega = bytes[16..64].as_chunks::<3>().0;
    // 256 colors after a 0x0C marker at the end, or gray without one.
    let vga = bytes
        .len()
        .checked_sub(769)
        .filter(|&end| end >= at && bytes[end] == 0x0C)
        .map(|end| bytes[end + 1..].as_chunks::<3>().0);
    let mut rgba = Vec::with_capacity(wu * hu * 4);
    for row in rows.chunks(stride) {
        let plane = |p: usize| &row[p * line..(p + 1) * line];
        for x in 0..wu {
            let px = match (bits, planes) {
                (8, 1) => {
                    let v = plane(0)[x];
                    match vga {
                        Some(colors) => {
                            let c = colors[usize::from(v)];
                            [c[0], c[1], c[2], 255]
                        }
                        None => [v, v, v, 255],
                    }
                }
                (8, 3) => [plane(0)[x], plane(1)[x], plane(2)[x], 255],
                (8, 4) => [plane(0)[x], plane(1)[x], plane(2)[x], plane(3)[x]],
                (1, 1) => {
                    let v = if plane(0)[x / 8] >> (7 - x % 8) & 1 == 1 { 255 } else { 0 };
                    [v, v, v, 255]
                }
                (1, 2..=4) => {
                    let index = (0..planes).fold(0, |n, p| n | ((plane(p)[x / 8] >> (7 - x % 8) & 1) << p));
                    let c = ega[usize::from(index)];
                    [c[0], c[1], c[2], 255]
                }
                (2 | 4, 1) => {
                    let per = 8 / bits;
                    let shift = (per - 1 - x % per) * bits;
                    let c = ega[usize::from(plane(0)[x / per] >> shift) & ((1 << bits) - 1)];
                    [c[0], c[1], c[2], 255]
                }
                _ => bail!("PCX with {bits}-bit pixels in {planes} planes isn't supported"),
            };
            rgba.extend(px);
        }
    }
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(width, height, rgba).context("size")?);
    Ok(Decoded { image, facts: Map::new() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(bits: u8, planes: u8, w: u16, h: u16, line: u16) -> Vec<u8> {
        let mut b = vec![0u8; HEADER];
        b[..4].copy_from_slice(&[0x0A, 5, 1, bits]);
        b[8..10].copy_from_slice(&(w - 1).to_le_bytes());
        b[10..12].copy_from_slice(&(h - 1).to_le_bytes());
        b[65] = planes;
        b[66..68].copy_from_slice(&line.to_le_bytes());
        b
    }

    #[test]
    fn reads_palettes_and_planes() {
        // 4x1, 256 colors, a run of three index-1 pixels then index 2.
        let mut b = header(8, 1, 4, 1, 4);
        b.extend([0xC3, 1, 2]);
        b.push(0x0C);
        let mut palette = vec![0u8; 768];
        palette[3..9].copy_from_slice(&[200, 100, 50, 1, 2, 3]);
        b.extend(palette);
        let img = decode(&b).unwrap().image.to_rgba8();
        assert_eq!(img.get_pixel(2, 0).0, [200, 100, 50, 255]);
        assert_eq!(img.get_pixel(3, 0).0, [1, 2, 3, 255]);

        // 2x1 truecolor in three planes, each line padded to two bytes.
        let mut b = header(8, 3, 2, 1, 2);
        b.extend([10, 11, 20, 21, 30, 31]);
        assert_eq!(decode(&b).unwrap().image.to_rgba8().get_pixel(1, 0).0, [11, 21, 31, 255]);

        // 16 colors in four bit planes: pixel 0 is index 0b0101.
        let mut b = header(1, 4, 1, 1, 2);
        b[16 + 5 * 3..16 + 6 * 3].copy_from_slice(&[9, 8, 7]);
        b.extend([0x80, 0, 0, 0, 0x80, 0, 0, 0]);
        assert_eq!(decode(&b).unwrap().image.to_rgba8().get_pixel(0, 0).0, [9, 8, 7, 255]);
        assert!(decode(&b[..100]).is_err());
    }
}
