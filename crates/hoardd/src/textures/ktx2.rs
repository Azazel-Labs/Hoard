//! KTX2 and Basis Universal: a KTX2 file's levels (raw, or compressed with Zstandard) in the layouts its Vulkan format
//! names, and a `.basis` file transcoded to RGBA.

use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};
use serde_json::{Map, json};

use super::{Decoded, Layout, MAX_PIXELS, check_size, decode_layout, half};

pub(super) fn ktx2(path: &Path) -> Result<Decoded> {
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
pub(super) fn basis(bytes: &[u8]) -> Result<Decoded> {
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

#[cfg(test)]
mod tests {

    use crate::textures::decode;
    use crate::textures::tests::{BC1_BLOCK, write};

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
}
