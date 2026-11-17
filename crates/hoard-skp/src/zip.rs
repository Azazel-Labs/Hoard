//! Just enough of a ZIP reader for SketchUp 2021+ files: the central directory, stored and deflated entries, and a
//! cap on how much any entry may inflate to.

use std::io::Read;

use anyhow::{Context, Result, bail, ensure};
use flate2::read::DeflateDecoder;

use crate::reader::{u16_at, u32_at};

/// The largest entry this will inflate. Real `model.dat` files run to tens of megabytes.
const MAX_ENTRY_BYTES: u64 = 1 << 30;
/// Real entries compress about 10:1; twenty times that is a decompression bomb, not a model.
const MAX_RATIO: u64 = 200;

#[derive(Debug)]
struct Entry {
    name: String,
    method: u16,
    compressed: u64,
    local_header: usize,
}

#[derive(Debug)]
pub(crate) struct Zip<'a> {
    d: &'a [u8],
    entries: Vec<Entry>,
}

impl<'a> Zip<'a> {
    /// Reads the central directory of the archive that fills `d`.
    pub(crate) fn open(d: &'a [u8]) -> Result<Self> {
        // the end record sits in the last 22 bytes plus at most a 64 KiB comment
        let lowest = d.len().saturating_sub(22 + 0xffff);
        let eocd = (lowest..=d.len().saturating_sub(22))
            .rev()
            .find(|&i| u32_at(d, i) == Some(0x0605_4b50))
            .context("ZIP container has no end record")?;
        let count = u16_at(d, eocd + 10).context("ZIP end record is cut short")?;
        let dir = u32_at(d, eocd + 16).context("ZIP end record is cut short")? as usize;
        ensure!(dir != 0xffff_ffff, "ZIP64 containers are not supported");
        let mut entries = Vec::new();
        let mut p = dir;
        for _ in 0..count {
            ensure!(u32_at(d, p) == Some(0x0201_4b50), "ZIP central directory is damaged");
            let field = |at: usize| u16_at(d, p + at).context("ZIP central directory is cut short");
            let method = field(10)?;
            let compressed = u32_at(d, p + 20).context("ZIP central directory is cut short")?;
            let (name_len, extra_len, comment_len) = (field(28)?, field(30)?, field(32)?);
            let local_header = u32_at(d, p + 42).context("ZIP central directory is cut short")? as usize;
            let name = d.get(p + 46..p + 46 + usize::from(name_len)).context("ZIP entry name is cut short")?;
            entries.push(Entry {
                name: String::from_utf8_lossy(name).into_owned(),
                method,
                compressed: u64::from(compressed),
                local_header,
            });
            p += 46 + usize::from(name_len) + usize::from(extra_len) + usize::from(comment_len);
        }
        Ok(Self { d, entries })
    }

    pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.name.as_str())
    }

    /// An entry's contents, or `None` when there is no entry by that name.
    pub(crate) fn get(&self, name: &str) -> Option<Result<Vec<u8>>> {
        self.entries.iter().find(|e| e.name == name).map(|e| self.extract(e))
    }

    fn extract(&self, e: &Entry) -> Result<Vec<u8>> {
        let d = self.d;
        let h = e.local_header;
        ensure!(u32_at(d, h) == Some(0x0403_4b50), "ZIP entry {} has no local header", e.name);
        let name_len = usize::from(u16_at(d, h + 26).context("ZIP local header is cut short")?);
        let extra_len = usize::from(u16_at(d, h + 28).context("ZIP local header is cut short")?);
        let start = h + 30 + name_len + extra_len;
        let len = usize::try_from(e.compressed)?;
        let data = d
            .get(start..start.checked_add(len).context("ZIP entry is too large")?)
            .context("ZIP entry is cut short")?;
        match e.method {
            0 => Ok(data.to_vec()),
            8 => {
                let limit = MAX_ENTRY_BYTES.min(e.compressed.saturating_mul(MAX_RATIO).saturating_add(1 << 20));
                let mut out = Vec::new();
                DeflateDecoder::new(data)
                    .take(limit + 1)
                    .read_to_end(&mut out)
                    .with_context(|| format!("inflating {}", e.name))?;
                ensure!((out.len() as u64) <= limit, "ZIP entry {} inflates implausibly far", e.name);
                Ok(out)
            }
            m => bail!("ZIP entry {} uses unsupported compression method {m}", e.name),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::DeflateEncoder;
    use std::io::Write;

    /// An archive of `entries`, deflated when `deflate` is set.
    pub(crate) fn archive_of(entries: &[(&str, &[u8])], deflate: bool) -> Vec<u8> {
        let method: u16 = if deflate { 8 } else { 0 };
        let mut out = Vec::new();
        let mut dir = Vec::new();
        for (name, body) in entries {
            let data = if deflate {
                let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
                enc.write_all(body).unwrap();
                enc.finish().unwrap()
            } else {
                body.to_vec()
            };
            let local = out.len() as u32;
            let sizes = [(data.len() as u32).to_le_bytes(), (body.len() as u32).to_le_bytes()].concat();
            out.extend(0x0403_4b50u32.to_le_bytes());
            out.extend([20, 0, 0, 0]);
            out.extend(method.to_le_bytes());
            out.extend([0; 8]); // time, date, crc
            out.extend(&sizes);
            out.extend((name.len() as u16).to_le_bytes());
            out.extend([0, 0]);
            out.extend(name.as_bytes());
            out.extend(&data);
            dir.extend(0x0201_4b50u32.to_le_bytes());
            dir.extend([20, 0, 20, 0, 0, 0]);
            dir.extend(method.to_le_bytes());
            dir.extend([0; 8]);
            dir.extend(&sizes);
            dir.extend((name.len() as u16).to_le_bytes());
            dir.extend([0; 12]);
            dir.extend(local.to_le_bytes());
            dir.extend(name.as_bytes());
        }
        let dir_at = out.len() as u32;
        out.extend(&dir);
        out.extend(0x0605_4b50u32.to_le_bytes());
        out.extend([0; 4]);
        out.extend((entries.len() as u16).to_le_bytes());
        out.extend((entries.len() as u16).to_le_bytes());
        out.extend((dir.len() as u32).to_le_bytes());
        out.extend(dir_at.to_le_bytes());
        out.extend([0, 0]);
        out
    }

    fn archive(name: &str, body: &[u8], deflate: bool) -> Vec<u8> {
        archive_of(&[(name, body)], deflate)
    }

    #[test]
    fn reads_stored_and_deflated_entries() {
        for deflate in [false, true] {
            let z = archive("model.dat", b"hello hello hello", deflate);
            let zip = Zip::open(&z).unwrap();
            assert_eq!(zip.names().collect::<Vec<_>>(), ["model.dat"]);
            assert_eq!(zip.get("model.dat").unwrap().unwrap(), b"hello hello hello");
            assert!(zip.get("other").is_none());
        }
    }

    #[test]
    fn damaged_archives_fail_cleanly() {
        let z = archive("model.dat", &[7; 5000], true);
        assert!(Zip::open(&z[..20]).is_err());
        for cut in (0..z.len()).step_by(3) {
            if let Ok(zip) = Zip::open(&z[cut..]) {
                let _ = zip.get("model.dat");
            }
        }
    }

    #[test]
    fn caps_inflation() {
        // three megabytes of zeros deflate to a few kilobytes: far past the ratio cap
        let z = archive("bomb", &vec![0; 3 << 20], true);
        assert!(Zip::open(&z).unwrap().get("bomb").unwrap().is_err());
    }
}
