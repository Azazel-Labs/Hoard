//! The newer picture formats: JPEG XL, decoded in pure Rust, and AVIF and HEIC (the photos iPhones take), which
//! ffmpeg reads once it's ready, the same as the sound formats Hoard doesn't decode itself.

use std::path::Path;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageDecoder};
use serde_json::Map;

use super::{Decoded, check_size};

/// Formats read here.
pub(super) const EXTS: &[&str] = &["jxl", "avif", "heic", "heif"];

pub(super) fn decode(path: &Path, ext: &str) -> Result<Decoded> {
    let image = if ext == "jxl" {
        jxl(path)?
    } else {
        let tool = crate::ffmpeg::tool().context("AVIF and HEIC pictures are read by ffmpeg, which isn't ready")?;
        crate::ffmpeg::decode_image(&tool, path)?
    };
    Ok(Decoded { image, facts: Map::new() })
}

fn jxl(path: &Path) -> Result<DynamicImage> {
    let decoder = jxl_oxide::integration::JxlDecoder::new(std::io::BufReader::new(std::fs::File::open(path)?))?;
    let (w, h) = decoder.dimensions();
    check_size(w, h)?;
    Ok(DynamicImage::from_decoder(decoder)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An 8x8 lossless JPEG XL of one color, #3366cc, made by libjxl.
    const JXL: [u8; 38] = [
        0xff, 0x0a, 0x41, 0x40, 0x24, 0x08, 0x04, 0x01, 0x00, 0x6c, 0x00, 0x4b, 0x18, 0x8b, 0x15, 0x00, 0xd4, 0x17,
        0x63, 0xbc, 0xbb, 0x8b, 0x77, 0x00, 0xe7, 0x70, 0x5e, 0x00, 0xe0, 0x7d, 0x5e, 0x6c, 0x90, 0x8a, 0x02, 0x4e,
        0x03, 0x18,
    ];

    #[test]
    fn decodes_jpeg_xl() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("blue.jxl");
        std::fs::write(&path, JXL).unwrap();
        let img = decode(&path, "jxl").unwrap().image.to_rgba8();
        assert_eq!(img.dimensions(), (8, 8));
        // Close to #3366cc: the encoder stored it a shade off.
        let px = img.get_pixel(3, 3).0;
        assert!(px.iter().zip([0x33, 0x66, 0xcc, 255]).all(|(&a, b): (&u8, u8)| a.abs_diff(b) <= 4), "{px:?}");
        std::fs::write(&path, &JXL[..12]).unwrap();
        assert!(decode(&path, "jxl").is_err());
    }
}
