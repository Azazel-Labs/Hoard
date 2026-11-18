//! Camera raw files, shown by the JPEG the camera keeps inside: most raw formats are TIFF underneath (DNG, Canon
//! CR2, Nikon NEF, Sony ARW, Pentax PEF, Olympus ORF, Panasonic RW2, Samsung SRW), with previews in their image
//! directories; Fujifilm's RAF says where its JPEG is in its own header.

use anyhow::{Context, Result};
use image::DynamicImage;
use serde_json::{Map, json};

use super::Decoded;

/// Formats read here.
pub(super) const EXTS: &[&str] = &["dng", "cr2", "nef", "nrw", "arw", "pef", "orf", "rw2", "srw", "raf"];

/// Image directories followed, at most, against loops.
const MAX_DIRS: usize = 64;

pub(super) fn decode(bytes: &[u8]) -> Result<Decoded> {
    let mut facts = Map::new();
    let candidates = if bytes.starts_with(b"FUJIFILMCCD-RAW") {
        let word = |i: usize| bytes.get(i..).and_then(<[u8]>::first_chunk).map(|w| u32::from_be_bytes(*w) as usize);
        vec![(word(84).context("RAF header is cut short")?, word(88).context("RAF header is cut short")?)]
    } else {
        let tiff = Tiff::new(bytes).context("not a camera raw file")?;
        let mut found = Vec::new();
        tiff.walk(&mut found, &mut facts);
        found
    };
    let mut candidates: Vec<&[u8]> = candidates
        .into_iter()
        .filter_map(|(at, len)| bytes.get(at..at.checked_add(len)?))
        .filter(|j| j.len() > 2)
        .collect();
    // The biggest first: the full-size preview rather than the thumbnail.
    candidates.sort_by_key(|j| std::cmp::Reverse(j.len()));
    // A raw image compressed as lossless JPEG doesn't decode here, and is skipped.
    let image = candidates
        .into_iter()
        .filter(|j| j.starts_with(&[0xFF, 0xD8]))
        .find_map(|j| image::load_from_memory_with_format(j, image::ImageFormat::Jpeg).ok())
        .context("no preview in the raw file")?;
    facts.insert("embedded_preview".into(), json!(true));
    Ok(Decoded { image: upright(image, facts.remove("orientation")), facts })
}

/// Turn the preview the way the camera was held.
fn upright(image: DynamicImage, orientation: Option<serde_json::Value>) -> DynamicImage {
    match orientation.and_then(|o| o.as_u64()) {
        Some(3) => image.rotate180(),
        Some(6) => image.rotate90(),
        Some(8) => image.rotate270(),
        _ => image,
    }
}

/// The model, with the make in front when the model doesn't already start with it: Nikon's model is "NIKON Z 6",
/// made by "NIKON CORPORATION", while Canon's is "EOS R5".
fn camera_name(make: &str, model: &str) -> String {
    let brand = make.split_whitespace().next().unwrap_or_default().to_lowercase();
    if model.to_lowercase().starts_with(&brand) { model.to_owned() } else { format!("{make} {model}") }
}

struct Tiff<'a> {
    bytes: &'a [u8],
    big_endian: bool,
}

impl<'a> Tiff<'a> {
    fn new(bytes: &'a [u8]) -> Option<Self> {
        let big_endian = match bytes.get(..4)? {
            // TIFF, and Olympus' and Panasonic's own magic numbers.
            b"II*\0" | b"IIRO" | b"IIRS" | b"IIU\0" => false,
            b"MM\0*" | b"MMOR" => true,
            _ => return None,
        };
        Some(Self { bytes, big_endian })
    }

    fn u16(&self, at: usize) -> Option<u16> {
        let b = *self.bytes.get(at..)?.first_chunk()?;
        Some(if self.big_endian { u16::from_be_bytes(b) } else { u16::from_le_bytes(b) })
    }

    fn u32(&self, at: usize) -> Option<u32> {
        let b = *self.bytes.get(at..)?.first_chunk()?;
        Some(if self.big_endian { u32::from_be_bytes(b) } else { u32::from_le_bytes(b) })
    }

    /// Every embedded JPEG's offset and length, following the chain of directories and the ones they point to.
    fn walk(&self, found: &mut Vec<(usize, usize)>, facts: &mut Map<String, serde_json::Value>) {
        let mut queue = vec![self.u32(4).unwrap_or(0) as usize];
        let mut seen = Vec::new();
        while let Some(dir) = queue.pop() {
            if dir == 0 || seen.contains(&dir) || seen.len() >= MAX_DIRS {
                continue;
            }
            seen.push(dir);
            let Some(count) = self.u16(dir) else { continue };
            let (mut jpeg, mut jpeg_len, mut compression, mut strip, mut strip_len) = (None, None, 0, None, None);
            for e in 0..usize::from(count) {
                let at = dir + 2 + e * 12;
                let (Some(tag), Some(kind), Some(n)) = (self.u16(at), self.u16(at + 2), self.u32(at + 4)) else {
                    break;
                };
                // A short value sits in the entry; anything else is pointed to.
                let value = if kind == 3 { self.u16(at + 8).map(u32::from) } else { self.u32(at + 8) };
                let Some(value) = value else { break };
                match tag {
                    0x0103 => compression = value,
                    0x0111 if n == 1 => strip = Some(value as usize),
                    0x0117 if n == 1 => strip_len = Some(value as usize),
                    0x0201 => jpeg = Some(value as usize),
                    0x0202 => jpeg_len = Some(value as usize),
                    // Panasonic keeps a full-size JPEG as one tag's data.
                    0x002E if kind == 7 => found.push((value as usize, n as usize)),
                    0x0112 => {
                        facts.entry("orientation").or_insert(json!(value));
                    }
                    0x010F | 0x0110 if kind == 2 => {
                        let text = self.ascii(at + 8, n as usize);
                        let key = if tag == 0x010F { "camera_make" } else { "camera" };
                        if !text.is_empty() {
                            facts.entry(key).or_insert(json!(text));
                        }
                    }
                    // Sub-directories, and the Exif one.
                    0x014A => {
                        if n == 1 {
                            queue.push(value as usize);
                        } else {
                            queue.extend(
                                (0..n.min(16) as usize)
                                    .filter_map(|i| self.u32(value as usize + i * 4))
                                    .map(|o| o as usize),
                            );
                        }
                    }
                    0x8769 => queue.push(value as usize),
                    _ => {}
                }
            }
            if let (Some(at), Some(len)) = (jpeg, jpeg_len) {
                found.push((at, len));
            }
            if let (6 | 7, Some(at), Some(len)) = (compression, strip, strip_len) {
                found.push((at, len));
            }
            queue.push(self.u32(dir + 2 + usize::from(count) * 12).unwrap_or(0) as usize);
        }
        if let (Some(make), Some(camera)) = (facts.remove("camera_make"), facts.get_mut("camera"))
            && let (Some(make), Some(model)) = (make.as_str(), camera.as_str())
        {
            *camera = json!(camera_name(make, model));
        }
    }

    /// The text of an ASCII entry of `n` bytes at `entry`'s value.
    fn ascii(&self, entry: usize, n: usize) -> String {
        let at = if n <= 4 { Some(entry) } else { self.u32(entry).map(|o| o as usize) };
        let text = at.and_then(|at| self.bytes.get(at..at.checked_add(n)?)).unwrap_or_default();
        String::from_utf8_lossy(text).trim_end_matches(['\0', ' ']).trim().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jpeg(w: u32, h: u32) -> Vec<u8> {
        let mut out = Vec::new();
        DynamicImage::ImageRgb8(image::RgbImage::from_pixel(w, h, image::Rgb([90, 160, 210])))
            .write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Jpeg)
            .unwrap();
        out
    }

    /// A little-endian TIFF: IFD0 with the camera's make and model, a thumbnail and a sub-directory holding a
    /// bigger JPEG as its strip, the way DNG and NEF keep their previews.
    fn raw(make: &str, model: &str, orientation: u16) -> Vec<u8> {
        let (small, big) = (jpeg(16, 8), jpeg(64, 32));
        let entry = |tag: u16, kind: u16, n: u32, v: u32| {
            let mut e = tag.to_le_bytes().to_vec();
            e.extend(kind.to_le_bytes());
            e.extend(n.to_le_bytes());
            e.extend(v.to_le_bytes());
            e
        };
        // Layout: header (8), IFD0 (2 + 6*12 + 4 = 78) at 8, sub-IFD (2 + 3*12 + 4 = 42) at 86, then data.
        let data = 8 + 78 + 42;
        let (make_at, model_at) = (data, data + 32);
        let small_at = data + 64;
        let big_at = small_at + small.len();
        let mut b = b"II*\0".to_vec();
        b.extend(8u32.to_le_bytes());
        b.extend(6u16.to_le_bytes());
        b.extend(entry(0x010F, 2, make.len() as u32 + 1, make_at as u32));
        b.extend(entry(0x0110, 2, model.len() as u32 + 1, model_at as u32));
        b.extend(entry(0x0112, 3, 1, u32::from(orientation)));
        b.extend(entry(0x014A, 4, 1, 86));
        b.extend(entry(0x0201, 4, 1, small_at as u32));
        b.extend(entry(0x0202, 4, 1, small.len() as u32));
        b.extend(0u32.to_le_bytes());
        b.extend(3u16.to_le_bytes());
        b.extend(entry(0x0103, 3, 1, 7));
        b.extend(entry(0x0111, 4, 1, big_at as u32));
        b.extend(entry(0x0117, 4, 1, big.len() as u32));
        b.extend(0u32.to_le_bytes());
        assert_eq!(b.len(), data);
        for (text, room) in [(make, 32), (model, 32)] {
            let mut t = text.as_bytes().to_vec();
            t.resize(room, 0);
            b.extend(t);
        }
        b.extend(small);
        b.extend(big);
        b
    }

    #[test]
    fn shows_the_largest_preview_the_right_way_up() {
        let d = decode(&raw("NIKON CORPORATION", "NIKON Z 6", 1)).unwrap();
        assert_eq!((d.image.width(), d.image.height()), (64, 32));
        assert_eq!(d.facts["camera"], "NIKON Z 6");
        let d = decode(&raw("Canon", "EOS R5", 6)).unwrap();
        assert_eq!((d.image.width(), d.image.height()), (32, 64));
        assert_eq!(d.facts["camera"], "Canon EOS R5");
        assert!(!d.facts.contains_key("orientation"));

        let mut raf = b"FUJIFILMCCD-RAW 0201".to_vec();
        raf.resize(100, 0);
        let j = jpeg(20, 10);
        raf[84..88].copy_from_slice(&100u32.to_be_bytes());
        raf[88..92].copy_from_slice(&(j.len() as u32).to_be_bytes());
        raf.extend(j);
        assert_eq!(decode(&raf).unwrap().image.width(), 20);
        assert!(decode(b"II*\0\0\0\0\0").is_err());
    }
}
