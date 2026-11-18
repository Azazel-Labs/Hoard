//! PVR textures (`.pvr`), from Imagination's tools for its GPUs and common in older iOS games: version 3, and the version 2
//! header before it. The top mip level comes first.

use anyhow::{Context, Result, bail};

use super::{ASTC_BLOCKS, Decoded, Layout, decode_layout, gpu_facts};

pub(super) fn decode(bytes: &[u8]) -> Result<Decoded> {
    let word = |i: usize| bytes.get(i * 4..).and_then(<[u8]>::first_chunk).map(|b| u32::from_le_bytes(*b));
    let header = (0..13).map(word).collect::<Option<Vec<u32>>>().context("PVR header is cut short")?;
    let (layout, width, height, mips, start) = if header[0] == 0x0352_5650 {
        let format = u64::from(header[2]) | (u64::from(header[3]) << 32);
        (v3_layout(format, header[5])?, header[7], header[6], header[11], 52 + header[12] as usize)
    } else if header[0] == 52 && header[11] == u32::from_le_bytes(*b"PVR!") {
        (v2_layout(header[4] & 0xFF)?, header[2], header[1], header[3] + 1, 52)
    } else {
        bail!("not a PowerVR texture")
    };
    let data = bytes.get(start..).context("PVR data is cut short")?;
    let image = decode_layout(data, width, height, layout)?;
    Ok(Decoded { image, facts: gpu_facts(layout.name(), mips) })
}

fn v3_layout(format: u64, channel_type: u32) -> Result<Layout> {
    if format >> 32 == 0 {
        let l = match format {
            0 | 1 => Layout::Pvrtc { two_bit: true },
            2 | 3 => Layout::Pvrtc { two_bit: false },
            6 => Layout::Etc1,
            7 => Layout::Bc1,
            8 | 9 => Layout::Bc2,
            10 | 11 => Layout::Bc3,
            12 => Layout::Bc4,
            13 => Layout::Bc5,
            14 => Layout::Bc6 { signed: false },
            15 => Layout::Bc7,
            22 => Layout::Etc2Rgb,
            23 => Layout::Etc2Rgba8,
            24 => Layout::Etc2Rgba1,
            25 => Layout::EacR,
            26 => Layout::EacRg,
            27..=40 => {
                let (bw, bh) = ASTC_BLOCKS[format as usize - 27];
                Layout::Astc { bw, bh }
            }
            _ => bail!("PowerVR format {format} isn't supported yet"),
        };
        return Ok(l);
    }
    // Uncompressed: the channels' names, then each one's bits.
    let order: String = format.to_le_bytes()[..4].iter().take_while(|&&c| c != 0).map(|&c| char::from(c)).collect();
    let bits = &format.to_le_bytes()[4..];
    let float = matches!(channel_type, 12 | 13);
    let l = match (order.as_str(), bits, float) {
        ("rgba", [8, 8, 8, 8], false) => Layout::Rgba8,
        ("bgra", [8, 8, 8, 8], false) => Layout::Bgra8,
        ("rgb", [8, 8, 8, 0], false) => Layout::Rgb8,
        ("bgr", [8, 8, 8, 0], false) => Layout::Bgr8,
        ("r" | "l" | "i", [8, 0, 0, 0], false) => Layout::R8,
        ("rg", [8, 8, 0, 0], false) => Layout::Rg8,
        ("la", [8, 8, 0, 0], false) => Layout::La8,
        ("rgba", [16, 16, 16, 16], true) => Layout::Rgba16F,
        ("rgba", [32, 32, 32, 32], true) => Layout::Rgba32F,
        _ => bail!("PowerVR format {order} {bits:?} isn't supported yet"),
    };
    Ok(l)
}

fn v2_layout(kind: u32) -> Result<Layout> {
    Ok(match kind {
        0x0C | 0x18 => Layout::Pvrtc { two_bit: true },
        0x0D | 0x19 => Layout::Pvrtc { two_bit: false },
        0x12 => Layout::Rgba8,
        0x15 => Layout::Rgb8,
        0x16 => Layout::R8,
        0x17 => Layout::La8,
        0x1A => Layout::Bgra8,
        0x20 => Layout::Bc1,
        0x22 => Layout::Bc2,
        0x24 => Layout::Bc3,
        0x36 => Layout::Etc1,
        _ => bail!("PowerVR format 0x{kind:02X} isn't supported yet"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_both_versions() {
        let mut v3 = Vec::new();
        for v in [0x0352_5650u32, 0, 7, 0, 0, 0, 4, 4, 1, 1, 1, 1, 0] {
            v3.extend(v.to_le_bytes());
        }
        v3.extend(super::super::tests::BC1_BLOCK);
        let d = decode(&v3).unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(0, 0).0, [0, 0, 255, 255]);
        assert_eq!(d.facts["gpu_format"], "BC1");

        // Version 2: a 2x1 RGBA image.
        let mut v2 = Vec::new();
        for v in [52u32, 1, 2, 0, 0x12, 8, 32, 0, 0, 0, 0, u32::from_le_bytes(*b"PVR!"), 1] {
            v2.extend(v.to_le_bytes());
        }
        v2.extend([1, 2, 3, 4, 5, 6, 7, 8]);
        let d = decode(&v2).unwrap();
        assert_eq!(d.image.to_rgba8().get_pixel(1, 0).0, [5, 6, 7, 8]);

        // Uncompressed version 3, named channels.
        let mut raw = Vec::new();
        let format = u64::from_le_bytes([b'b', b'g', b'r', b'a', 8, 8, 8, 8]);
        for v in [0x0352_5650u32, 0, format as u32, (format >> 32) as u32, 0, 0, 1, 1, 1, 1, 1, 1, 0] {
            raw.extend(v.to_le_bytes());
        }
        raw.extend([30, 20, 10, 255]);
        assert_eq!(decode(&raw).unwrap().image.to_rgba8().get_pixel(0, 0).0, [10, 20, 30, 255]);
    }
}
