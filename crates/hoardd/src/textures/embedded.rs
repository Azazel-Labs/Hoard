//! Source files whose layers Hoard can't draw but which carry a picture of themselves: Clip Studio (`.clip`) keeps
//! its canvas preview in the SQLite database inside it, Procreate (`.procreate`) a thumbnail in its zip, Affinity
//! Photo and Designer (`.afphoto`, `.afdesign`) a PNG among their data, and EPS a TIFF named by its binary header.

use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::DynamicImage;
use serde_json::{Map, json};

use super::Decoded;

/// Formats read here.
pub(super) const EXTS: &[&str] = &["clip", "procreate", "afphoto", "afdesign", "eps"];

/// The most read of an embedded picture.
const MAX_PREVIEW: u64 = 64 << 20;

pub(super) fn decode(path: &Path, ext: &str) -> Result<Decoded> {
    let image = match ext {
        "clip" => clip(&std::fs::read(path)?)?,
        "procreate" => procreate(path)?,
        "afphoto" | "afdesign" => largest_png(&std::fs::read(path)?)?,
        "eps" => eps(&std::fs::read(path)?)?,
        _ => bail!("no embedded preview read from .{ext}"),
    };
    let mut facts = Map::new();
    // The file's own picture of itself, made smaller than the art: not pixels to judge.
    facts.insert("embedded_preview".into(), json!(true));
    Ok(Decoded { image, facts })
}

/// Clip Studio: chunks after a `CSFCHUNK` header, one of them (`CHNKSQLi`) a SQLite database whose `CanvasPreview`
/// table holds a PNG.
fn clip(bytes: &[u8]) -> Result<DynamicImage> {
    ensure!(bytes.starts_with(b"CSFCHUNK"), "not a Clip Studio file");
    // The header: the file's size, then where the first chunk is.
    let first = bytes.get(16..).and_then(<[u8]>::first_chunk).map(|o| u64::from_be_bytes(*o));
    let mut at = usize::try_from(first.context("Clip Studio header is cut short")?)?;
    let db = loop {
        let name = bytes.get(at..at + 8).context("no database in the Clip Studio file")?;
        let size = bytes.get(at + 8..).and_then(<[u8]>::first_chunk).map(|s| u64::from_be_bytes(*s));
        let size = usize::try_from(size.context("Clip Studio chunk is cut short")?)?;
        let body = bytes.get(at + 16..(at + 16).saturating_add(size)).context("Clip Studio chunk is cut short")?;
        if name == b"CHNKSQLi" {
            break body;
        }
        at += 16 + size;
    };
    let mut file = tempfile::Builder::new().suffix(".sqlite").tempfile()?;
    std::io::Write::write_all(&mut file, db)?;
    let conn = rusqlite::Connection::open_with_flags(file.path(), rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let png: Vec<u8> = conn
        .query_row("SELECT ImageData FROM CanvasPreview ORDER BY ImageWidth * ImageHeight DESC LIMIT 1", [], |r| {
            r.get(0)
        })
        .context("no canvas preview in the Clip Studio file")?;
    Ok(image::load_from_memory(&png)?)
}

/// Procreate: a zip with `QuickLook/Thumbnail.png`.
fn procreate(path: &Path) -> Result<DynamicImage> {
    let mut zip =
        zip::ZipArchive::new(std::io::BufReader::new(std::fs::File::open(path)?)).context("not a Procreate file")?;
    let mut png = Vec::new();
    zip.by_name("QuickLook/Thumbnail.png")
        .context("no thumbnail in the Procreate file")?
        .take(MAX_PREVIEW)
        .read_to_end(&mut png)?;
    Ok(image::load_from_memory_with_format(&png, image::ImageFormat::Png)?)
}

/// The largest whole PNG anywhere in `bytes`.
fn largest_png(bytes: &[u8]) -> Result<DynamicImage> {
    const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    let mut best: Option<&[u8]> = None;
    let mut from = 0;
    while let Some(start) = bytes[from..].windows(SIGNATURE.len()).position(|w| w == SIGNATURE).map(|i| from + i) {
        // Walk the chunks to IEND, so a PNG is only taken whole.
        let mut at = start + SIGNATURE.len();
        let end = loop {
            let Some(len) = bytes.get(at..).and_then(<[u8]>::first_chunk).map(|l| u32::from_be_bytes(*l) as usize)
            else {
                break None;
            };
            let kind = bytes.get(at + 4..at + 8);
            at = at.saturating_add(12 + len);
            if at > bytes.len() {
                break None;
            }
            if kind == Some(b"IEND") {
                break Some(at);
            }
        };
        match end {
            Some(end) => {
                if best.is_none_or(|b| end - start > b.len()) {
                    best = Some(&bytes[start..end]);
                }
                from = end;
            }
            None => from = start + 1,
        }
    }
    Ok(image::load_from_memory_with_format(best.context("no preview in the file")?, image::ImageFormat::Png)?)
}

/// EPS: a binary header, when there is one, gives where a TIFF preview is.
fn eps(bytes: &[u8]) -> Result<DynamicImage> {
    ensure!(bytes.starts_with(&[0xC5, 0xD0, 0xD3, 0xC6]), "this EPS file has no preview");
    let word = |i: usize| bytes.get(i..).and_then(<[u8]>::first_chunk).map(|w| u32::from_le_bytes(*w) as usize);
    let (offset, len) = (word(20).unwrap_or(0), word(24).unwrap_or(0));
    ensure!(len > 0, "this EPS file has no TIFF preview");
    let tiff = bytes.get(offset..offset.saturating_add(len)).context("EPS preview is cut short")?;
    Ok(image::load_from_memory_with_format(tiff, image::ImageFormat::Tiff)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(w: u32, h: u32) -> Vec<u8> {
        let mut out = Vec::new();
        DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(w, h, image::Rgba([200, 30, 40, 255])))
            .write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
            .unwrap();
        out
    }

    #[test]
    fn finds_the_whole_largest_png() {
        let mut bytes = b"\0\xffKA junk".to_vec();
        bytes.extend(png(4, 4));
        bytes.extend(b"more");
        bytes.extend(png(16, 9));
        // A signature with nothing whole after it.
        bytes.extend(&png(32, 32)[..40]);
        assert_eq!(largest_png(&bytes).unwrap().width(), 16);
        assert!(largest_png(b"nothing here").is_err());
    }

    #[test]
    fn reads_clip_studio_previews() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch("CREATE TABLE CanvasPreview (ImageWidth INTEGER, ImageHeight INTEGER, ImageData BLOB)")
            .unwrap();
        for (w, h) in [(8, 8), (40, 30)] {
            conn.execute("INSERT INTO CanvasPreview VALUES (?1, ?2, ?3)", rusqlite::params![w, h, png(w, h)]).unwrap();
        }
        drop(conn);
        let db = std::fs::read(&db_path).unwrap();
        let mut bytes = b"CSFCHUNK".to_vec();
        bytes.extend([0; 8]);
        bytes.extend(24u64.to_be_bytes());
        for (name, body) in [(&b"CHNKHead"[..], &b"head"[..]), (b"CHNKSQLi", &db)] {
            bytes.extend(name);
            bytes.extend((body.len() as u64).to_be_bytes());
            bytes.extend(body);
        }
        assert_eq!(clip(&bytes).unwrap().width(), 40);
    }

    #[test]
    fn reads_the_tiff_in_an_eps() {
        let mut tiff = Vec::new();
        DynamicImage::ImageRgb8(image::RgbImage::new(5, 3))
            .write_to(&mut std::io::Cursor::new(&mut tiff), image::ImageFormat::Tiff)
            .unwrap();
        let ps = b"%!PS-Adobe-3.0 EPSF-3.0\n";
        let mut bytes = vec![0xC5, 0xD0, 0xD3, 0xC6];
        for v in [30, ps.len(), 0, 0, 30 + ps.len(), tiff.len()] {
            bytes.extend((v as u32).to_le_bytes());
        }
        bytes.extend([0xFF, 0xFF]);
        bytes.extend(ps);
        bytes.extend(&tiff);
        assert_eq!(eps(&bytes).unwrap().width(), 5);
        assert!(eps(ps).is_err());
    }
}
