//! KTX 1 (`.ktx`), Khronos' first texture container: an OpenGL format number, then each mip level's size and data,
//! largest first.

use anyhow::{Context, Result, bail, ensure};

use super::{ASTC_BLOCKS, Decoded, Layout, decode_layout, gpu_facts};

/// OpenGL's `GL_UNSIGNED_BYTE`.
const UNSIGNED_BYTE: u32 = 0x1401;
const MAGIC: &[u8; 12] = b"\xABKTX 11\xBB\r\n\x1A\n";
const HEADER: usize = 64;

pub(super) fn decode(bytes: &[u8]) -> Result<Decoded> {
    ensure!(bytes.get(..12) == Some(MAGIC), "not a KTX file");
    let word = |i: usize| bytes.get(12 + i * 4..).and_then(<[u8]>::first_chunk).map(|b| u32::from_le_bytes(*b));
    let header = (0..13).map(word).collect::<Option<Vec<u32>>>().context("KTX header is cut short")?;
    let [endianness, gl_type, _type_size, gl_format, internal, _base, width, height, ..] = header[..] else {
        unreachable!()
    };
    ensure!(endianness == 0x0403_0201, "big-endian KTX files aren't supported");
    let (mips, key_values) = (header[11], header[12] as usize);
    let layout = layout(gl_type, gl_format, internal)?;
    let (w, h) = (width as usize, height.max(1) as usize);
    let start = HEADER.checked_add(key_values).context("size")? + 4;
    let data = bytes.get(start..).context("KTX data is cut short")?;
    // Uncompressed rows are padded to four bytes.
    let row = layout.bytes(w, 1);
    let data = match layout {
        Layout::R8 | Layout::Rg8 | Layout::La8 | Layout::Rgb8 | Layout::Bgr8 if row % 4 != 0 => {
            let padded = row.next_multiple_of(4);
            data.chunks(padded).take(h).flat_map(|r| &r[..row.min(r.len())]).copied().collect()
        }
        _ => data.to_vec(),
    };
    let image = decode_layout(&data, width, height.max(1), layout)?;
    Ok(Decoded { image, facts: gpu_facts(layout.name(), mips) })
}

/// The pixel layout for OpenGL's type, format and internal format.
fn layout(gl_type: u32, gl_format: u32, internal: u32) -> Result<Layout> {
    let compressed = match internal {
        0x83F0 | 0x83F1 | 0x8C4C | 0x8C4D => Some(Layout::Bc1),
        0x83F2 | 0x8C4E => Some(Layout::Bc2),
        0x83F3 | 0x8C4F => Some(Layout::Bc3),
        0x8DBB | 0x8DBC => Some(Layout::Bc4),
        0x8DBD | 0x8DBE => Some(Layout::Bc5),
        0x8E8C | 0x8E8D => Some(Layout::Bc7),
        0x8E8E => Some(Layout::Bc6 { signed: true }),
        0x8E8F => Some(Layout::Bc6 { signed: false }),
        0x8D64 => Some(Layout::Etc1),
        0x9274 | 0x9275 => Some(Layout::Etc2Rgb),
        0x9276 | 0x9277 => Some(Layout::Etc2Rgba1),
        0x9278 | 0x9279 => Some(Layout::Etc2Rgba8),
        0x9270 | 0x9271 => Some(Layout::EacR),
        0x9272 | 0x9273 => Some(Layout::EacRg),
        0x8C00 | 0x8C02 => Some(Layout::Pvrtc { two_bit: false }),
        0x8C01 | 0x8C03 => Some(Layout::Pvrtc { two_bit: true }),
        0x93B0..=0x93BD | 0x93D0..=0x93DD => {
            let (bw, bh) = ASTC_BLOCKS[(internal & 0xF) as usize];
            Some(Layout::Astc { bw, bh })
        }
        _ => None,
    };
    if let Some(l) = compressed {
        return Ok(l);
    }
    let l = match (gl_type, gl_format) {
        (UNSIGNED_BYTE, 0x1908) => Layout::Rgba8,
        (UNSIGNED_BYTE, 0x80E1) => Layout::Bgra8,
        (UNSIGNED_BYTE, 0x1907) => Layout::Rgb8,
        (UNSIGNED_BYTE, 0x80E0) => Layout::Bgr8,
        (UNSIGNED_BYTE, 0x1903 | 0x1909 | 0x1906) => Layout::R8,
        (UNSIGNED_BYTE, 0x8227) => Layout::Rg8,
        (UNSIGNED_BYTE, 0x190A) => Layout::La8,
        (0x140B | 0x8D61, 0x1908) => Layout::Rgba16F,
        (0x1406, 0x1908) => Layout::Rgba32F,
        _ => bail!("KTX format 0x{internal:04X} isn't supported yet"),
    };
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ktx(gl_type: u32, gl_format: u32, internal: u32, w: u32, h: u32, data: &[u8]) -> Vec<u8> {
        let mut b = MAGIC.to_vec();
        for v in [0x0403_0201, gl_type, 1, gl_format, internal, gl_format, w, h, 0, 0, 1, 1, 0] {
            b.extend(u32::to_le_bytes(v));
        }
        b.extend(u32::to_le_bytes(data.len() as u32));
        b.extend(data);
        b
    }

    #[test]
    fn reads_compressed_and_padded_rows() {
        let bc1 = ktx(0, 0, 0x83F0, 4, 4, &super::super::tests::BC1_BLOCK);
        let d = decode(&bc1).unwrap();
        let px = d.image.to_rgba8();
        assert_eq!(px.get_pixel(0, 0).0, [0, 0, 255, 255]);
        assert_eq!(px.get_pixel(0, 1).0, [255, 0, 0, 255]);
        assert_eq!(d.facts["gpu_format"], "BC1");

        // A 3x2 RGB image: each nine-byte row is padded to twelve.
        let mut rows = Vec::new();
        for y in 0..2u8 {
            for x in 0..3u8 {
                rows.extend([x * 10, y * 10, 7]);
            }
            rows.extend([0, 0, 0]);
        }
        let d = decode(&ktx(0x1401, 0x1907, 0x8051, 3, 2, &rows)).unwrap();
        assert_eq!(d.image.to_rgb8().get_pixel(2, 1).0, [20, 10, 7]);
        assert!(decode(b"\xABKTX 20\xBB\r\n\x1A\n").is_err());
    }
}
