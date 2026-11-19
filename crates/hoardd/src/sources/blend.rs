//! Blender files: the preview Blender embeds near the start, and the paths of the textures and .blend files they use.

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::RgbaImage;

/// How much decompressed .blend data to scan for texture/library paths. Thumbnails are found much earlier.
const BLEND_SCAN_LIMIT: u64 = 256 << 20;
const IMAGE_EXTS: &[&str] =
    &[".png", ".jpg", ".jpeg", ".tga", ".tif", ".tiff", ".exr", ".hdr", ".psd", ".bmp", ".webp", ".dds"];

// ---------------------------------------------------------------------------------------------
// .blend

#[derive(Debug, Default)]
pub(crate) struct BlendInfo {
    /// e.g. "4.5" or "5.0".
    pub(crate) version: String,
    pub(crate) compression: Option<&'static str>,
    pub(crate) thumbnail: Option<RgbaImage>,
    /// External file paths as stored (`//textures/rock.png` style, `//` = relative to the .blend).
    pub(crate) image_paths: Vec<String>,
    pub(crate) library_paths: Vec<String>,
}

/// Open a file, transparently decompressing gzip or zstd.
fn open_maybe_compressed(path: &Path) -> Result<(Box<dyn Read>, Option<&'static str>)> {
    let mut f = File::open(path)?;
    let mut magic = [0u8; 4];
    let n = f.read(&mut magic)?;
    f.seek(SeekFrom::Start(0))?;
    let reader = BufReader::with_capacity(1 << 16, f);
    Ok(match &magic[..n] {
        [0x1f, 0x8b, ..] => (Box::new(flate2::read::GzDecoder::new(reader)), Some("gzip")),
        [0x28, 0xb5, 0x2f, 0xfd] => (Box::new(zstd::stream::read::Decoder::with_buffer(reader)?), Some("zstd")),
        _ => (Box::new(reader), None),
    })
}

#[derive(Debug, Clone, Copy)]
enum BHeadKind {
    /// Pre-5.0: code, len(i32), old pointer (4 or 8 bytes), SDNA index, count.
    Legacy { ptr8: bool, big_endian: bool },
    /// Blender 5.0+ "large" headers: code, SDNA index, old pointer (u64), len (i64), count (i64).
    Large,
}

struct Reader<R: Read> {
    inner: R,
    big_endian: bool,
}

impl<R: Read> Reader<R> {
    fn bytes<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut b = [0u8; N];
        self.inner.read_exact(&mut b)?;
        Ok(b)
    }
    fn i32(&mut self) -> io::Result<i32> {
        let b = self.bytes::<4>()?;
        Ok(if self.big_endian { i32::from_be_bytes(b) } else { i32::from_le_bytes(b) })
    }
    fn i64(&mut self) -> io::Result<i64> {
        let b = self.bytes::<8>()?;
        Ok(if self.big_endian { i64::from_be_bytes(b) } else { i64::from_le_bytes(b) })
    }
    fn skip(&mut self, n: u64) -> io::Result<()> {
        let copied = io::copy(&mut (&mut self.inner).take(n), &mut io::sink())?;
        if copied < n { Err(io::ErrorKind::UnexpectedEof.into()) } else { Ok(()) }
    }
    fn vec(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut v = vec![0u8; n];
        self.inner.read_exact(&mut v)?;
        Ok(v)
    }
}

/// Read a .blend's header, embedded preview, and (optionally) external file paths.
pub(crate) fn read_blend(path: &Path, scan_paths: bool) -> Result<BlendInfo> {
    let (raw, compression) = open_maybe_compressed(path)?;
    let mut r = Reader { inner: raw.take(BLEND_SCAN_LIMIT), big_endian: false };
    let mut info = BlendInfo { compression, ..Default::default() };

    let magic = r.bytes::<7>()?;
    ensure!(&magic == b"BLENDER", "not a .blend file");
    let next = r.bytes::<1>()?[0];
    let kind = match next {
        b'_' | b'-' => {
            // Legacy 12-byte header: "BLENDER" + ptr size + endianness + "405".
            let [endian, v1, v2, v3] = r.bytes::<4>()?;
            let minor: u32 = String::from_utf8_lossy(&[v2, v3]).parse().unwrap_or(0);
            info.version = format!("{}.{minor}", char::from(v1));
            BHeadKind::Legacy { ptr8: next == b'-', big_endian: endian == b'V' }
        }
        b'0'..=b'9' => {
            // 5.0+: "BLENDER" + header size (2 digits) + "-" + format version (2) + "v" + version (4), e.g.
            // "BLENDER17-01v0500".
            let rest = r.bytes::<9>()?;
            let text = String::from_utf8_lossy(&rest);
            let ver = text.rsplit('v').next().unwrap_or("");
            if let (Some(major), Some(minor)) = (ver.get(..2), ver.get(2..4)) {
                let (major, minor) = (major.parse::<u32>().unwrap_or(0), minor.parse::<u32>().unwrap_or(0));
                info.version = format!("{major}.{minor}");
            }
            BHeadKind::Large
        }
        _ => bail!("unrecognized .blend header"),
    };
    if let BHeadKind::Legacy { big_endian, .. } = kind {
        r.big_endian = big_endian;
    }

    // Stops at ENDB, or quietly at EOF when the scan limit truncates the stream or the file is damaged.
    while let Ok(code) = r.bytes::<4>() {
        let len: u64 = match kind {
            BHeadKind::Legacy { ptr8, .. } => {
                let len = r.i32()?;
                r.skip(if ptr8 { 8 } else { 4 })?; // old memory address
                r.skip(8)?; // SDNA index, count
                u64::try_from(len).unwrap_or(0)
            }
            BHeadKind::Large => {
                r.skip(4 + 8)?; // SDNA index, old address
                let len = r.i64()?;
                r.skip(8)?; // count
                u64::try_from(len).unwrap_or(0)
            }
        };
        match &code {
            b"ENDB" => break,
            b"TEST" if len >= 8 => {
                let w = r.i32()?;
                let h = r.i32()?;
                let px = u64::try_from(w).unwrap_or(0) * u64::try_from(h).unwrap_or(0) * 4;
                if w > 0 && h > 0 && w <= 4096 && h <= 4096 && px == len - 8 {
                    let data = r.vec(px as usize)?;
                    let mut img = RgbaImage::from_raw(w as u32, h as u32, data).context("bad .blend preview")?;
                    // Stored bottom-up (OpenGL order).
                    image::imageops::flip_vertical_in_place(&mut img);
                    // Blender renders the camera or viewport square. A preview of another shape is a screenshot
                    // of the whole window, menus and panels and all, which shows the editor, not the model.
                    if w == h {
                        info.thumbnail = Some(img);
                    }
                } else {
                    r.skip(len - 8)?;
                }
                if !scan_paths {
                    break;
                }
            }
            [b'I', b'M', 0, 0] | [b'L', b'I', 0, 0] if scan_paths && len <= 64 << 10 => {
                let data = r.vec(len as usize)?;
                let is_lib = code[0] == b'L';
                let mut strings = c_strings(&data);
                // The block's own ID name, such as "IMrock.png" or "LIprops.blend", isn't a path.
                if let Some(i) = strings.iter().position(|s| s.as_bytes().starts_with(&code[..2])) {
                    strings.remove(i);
                }
                // Nor is the tail of an older, longer string left in a buffer, such as "ops.blend".
                let tails: Vec<bool> = strings
                    .iter()
                    .map(|s| strings.iter().any(|o| o.len() > s.len() && o.ends_with(s.as_str())))
                    .collect();
                for (s, _) in strings.into_iter().zip(tails).filter(|(_, tail)| !tail) {
                    // Not Path::extension: on Windows, Blender's relative "//dir/file.png" parses as a UNC prefix.
                    let name = s.rsplit(['/', '\\']).next().unwrap_or(&s);
                    let ext = name.rsplit_once('.').map(|(_, e)| format!(".{}", e.to_lowercase()));
                    let wanted =
                        ext.is_some_and(|e| if is_lib { e == ".blend" } else { IMAGE_EXTS.contains(&e.as_str()) });
                    if wanted {
                        if is_lib { &mut info.library_paths } else { &mut info.image_paths }.push(s);
                    }
                }
            }
            _ => r.skip(len)?,
        }
    }
    info.image_paths.sort();
    info.image_paths.dedup();
    info.library_paths.sort();
    info.library_paths.dedup();
    Ok(info)
}

/// NUL-terminated printable runs that look like file paths.
fn c_strings(data: &[u8]) -> Vec<String> {
    data.split(|&b| b == 0)
        .filter(|s| s.len() >= 5 && s.contains(&b'.') && s.iter().all(|b| (0x20..0x7f).contains(b) || *b >= 0x80))
        .filter_map(|s| std::str::from_utf8(s).ok().map(str::to_owned))
        .collect()
}
