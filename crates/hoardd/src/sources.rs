//! Readers for authoring ("source") formats that general image/model libraries don't handle. Each reads only
//! what Hoard needs, a preview and a few facts, so a 3 GB file costs a few reads, not a full parse.
//!
//! - `.blend`: Blender embeds a small preview (the `TEST` block) near the start of every file it saves with
//!   "save preview" on (the default). Files may be gzip (pre-3.0) or zstd (3.0+) compressed. Image (`IM`) and
//!   library (`LI`) blocks hold the paths of external textures and linked .blend files.
//! - `.psd`/`.psb`: header facts, layer count, and the JPEG thumbnail Photoshop stores in image resource 1036.
//! - `.kra` (Krita) and `.ora` (OpenRaster): zip files with a flattened `mergedimage.png` inside.

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};

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
                    info.thumbnail = Some(img);
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
                for s in c_strings(&data) {
                    let ext = Path::new(&s).extension().map(|e| format!(".{}", e.to_string_lossy().to_lowercase()));
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

// ---------------------------------------------------------------------------------------------
// .psd / .psb

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

// ---------------------------------------------------------------------------------------------
// .kra / .ora

#[derive(Debug, Default)]
pub(crate) struct LayeredZipInfo {
    pub(crate) composite: Option<DynamicImage>,
    pub(crate) layers: Option<u32>,
}

/// Krita and `OpenRaster` both store a flattened `mergedimage.png`; fall back to the small preview.
pub(crate) fn read_layered_zip(path: &Path) -> Result<LayeredZipInfo> {
    let mut zip = zip::ZipArchive::new(BufReader::new(File::open(path)?)).context("not a zip-based image")?;
    let mut info = LayeredZipInfo::default();
    for name in ["mergedimage.png", "preview.png", "Thumbnails/thumbnail.png"] {
        let Ok(mut entry) = zip.by_name(name) else { continue };
        if entry.size() > 512 << 20 {
            continue;
        }
        let mut buf = Vec::with_capacity(usize::try_from(entry.size()).unwrap_or(0));
        entry.read_to_end(&mut buf)?;
        if let Ok(img) = image::load_from_memory_with_format(&buf, image::ImageFormat::Png) {
            info.composite = Some(img);
            break;
        }
    }
    // Layer count from the document XML: maindoc.xml (Krita) or stack.xml (OpenRaster).
    for name in ["maindoc.xml", "stack.xml"] {
        let Ok(mut entry) = zip.by_name(name) else { continue };
        let mut xml = String::new();
        if entry.by_ref().take(16 << 20).read_to_string(&mut xml).is_ok() {
            let n = xml.matches("<layer ").count() + xml.matches("<layer>").count();
            info.layers = Some(n as u32);
            break;
        }
    }
    Ok(info)
}

// ---------------------------------------------------------------------------------------------
// WAV (RIFF / RF64 / BWF) metadata

/// What sound libraries embed in WAV files. Professional SFX libraries often carry a full description, keywords and
/// UCS category per file inside the file itself (BWF `bext`, `LIST/INFO`, iXML written by Soundminer/BaseHead), which
/// beats anything a filename like `spell35.wav` can say.
#[derive(Debug, Default)]
pub(crate) struct RiffMeta {
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) keywords: Vec<String>,
    pub(crate) genre: Option<String>,
    pub(crate) artist: Option<String>,
    pub(crate) library: Option<String>,
    /// UCS fields from iXML (`CATEGORY`, `SUBCATEGORY`, `CATID`).
    pub(crate) ucs_category: Option<String>,
    pub(crate) ucs_subcategory: Option<String>,
    pub(crate) ucs_catid: Option<String>,
    /// First sampler loop, in sample frames (`smpl` chunk).
    pub(crate) loop_points: Option<(u32, u32)>,
    /// ACID: one-shot vs loop, tempo, root note.
    pub(crate) acid_one_shot: Option<bool>,
    pub(crate) tempo: Option<f32>,
    pub(crate) root_note: Option<u16>,
    pub(crate) cue_points: u32,
}

fn clean_text(b: &[u8]) -> Option<String> {
    let s = String::from_utf8_lossy(b);
    let s = s.trim_matches(|c: char| c == '\0' || c.is_whitespace()).replace('\0', " ");
    let s = s.trim();
    (!s.is_empty()).then(|| s.to_owned())
}

fn le_u32(b: &[u8], at: usize) -> Option<u32> {
    b.get(at..at + 4).map(|x| u32::from_le_bytes([x[0], x[1], x[2], x[3]]))
}

/// iXML values for the given tags, wherever they sit (Soundminer and `BaseHead` put them under `<USER>` or `<BEXT>`).
fn ixml_field(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&format!("</{tag}>"))? + start;
    let v = xml[start..end].trim().replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">");
    (!v.is_empty()).then_some(v)
}

pub(crate) fn read_riff(path: &Path) -> Result<RiffMeta> {
    let mut f = BufReader::new(File::open(path)?);
    let mut header = [0u8; 12];
    f.read_exact(&mut header)?;
    ensure!(&header[0..4] == b"RIFF" || &header[0..4] == b"RF64" || &header[0..4] == b"BW64", "not a RIFF file");
    ensure!(&header[8..12] == b"WAVE", "not a WAVE file");
    let file_len = f.get_ref().metadata()?.len();
    let mut meta = RiffMeta::default();
    let mut rf64_data_size: Option<u64> = None;
    let mut pos = 12u64;
    while pos + 8 <= file_len {
        let mut ch = [0u8; 8];
        if f.read_exact(&mut ch).is_err() {
            break;
        }
        let id = [ch[0], ch[1], ch[2], ch[3]];
        let mut size = u64::from(u32::from_le_bytes([ch[4], ch[5], ch[6], ch[7]]));
        if &id == b"data" && size == 0xFFFF_FFFF {
            size = rf64_data_size.unwrap_or(file_len - pos - 8);
        }
        let padded = size + size % 2;
        // Metadata chunks are small; anything big (audio data) is skipped without reading.
        let small = size <= 4 << 20;
        match &id {
            b"ds64" if small => {
                let d = read_vec(&mut f, size)?;
                rf64_data_size = d.get(8..16).map(|x| u64::from_le_bytes(x.try_into().unwrap_or([0; 8])));
                skip_pad(&mut f, size)?;
            }
            b"bext" if small => {
                let d = read_vec(&mut f, size)?;
                meta.description = meta.description.or_else(|| d.get(0..256).and_then(clean_text));
                meta.artist = meta.artist.or_else(|| d.get(256..288).and_then(clean_text));
                skip_pad(&mut f, size)?;
            }
            b"LIST" if small => {
                let d = read_vec(&mut f, size)?;
                if d.get(0..4) == Some(b"INFO") {
                    let mut i = 4;
                    while i + 8 <= d.len() {
                        let sub = &d[i..i + 4];
                        let len = le_u32(&d, i + 4).unwrap_or(0) as usize;
                        let val = d.get(i + 8..(i + 8 + len).min(d.len())).and_then(clean_text);
                        match sub {
                            b"INAM" => meta.title = meta.title.take().or(val),
                            b"ICMT" | b"ISBJ" => meta.description = meta.description.take().or(val),
                            b"IKEY" => {
                                if let Some(v) = val {
                                    meta.keywords.extend(
                                        v.split([';', ',']).map(|k| k.trim().to_owned()).filter(|k| !k.is_empty()),
                                    );
                                }
                            }
                            b"IGNR" => meta.genre = meta.genre.take().or(val),
                            b"IART" => meta.artist = meta.artist.take().or(val),
                            b"IPRD" => meta.library = meta.library.take().or(val),
                            _ => {}
                        }
                        i += 8 + len + len % 2;
                    }
                }
                skip_pad(&mut f, size)?;
            }
            b"iXML" if small => {
                let d = read_vec(&mut f, size)?;
                let xml = String::from_utf8_lossy(&d);
                meta.description = meta
                    .description
                    .take()
                    .or_else(|| ixml_field(&xml, "DESCRIPTION"))
                    .or_else(|| ixml_field(&xml, "NOTE"));
                meta.title =
                    meta.title.take().or_else(|| ixml_field(&xml, "FXNAME")).or_else(|| ixml_field(&xml, "TRACKTITLE"));
                meta.library = meta.library.take().or_else(|| ixml_field(&xml, "LIBRARY"));
                meta.ucs_category = ixml_field(&xml, "CATEGORY");
                meta.ucs_subcategory = ixml_field(&xml, "SUBCATEGORY");
                meta.ucs_catid = ixml_field(&xml, "CATID");
                if let Some(k) = ixml_field(&xml, "KEYWORDS") {
                    meta.keywords.extend(k.split([',', ';']).map(|k| k.trim().to_owned()).filter(|k| !k.is_empty()));
                }
                skip_pad(&mut f, size)?;
            }
            b"smpl" if small => {
                let d = read_vec(&mut f, size)?;
                if le_u32(&d, 28).unwrap_or(0) > 0
                    && let (Some(start), Some(end)) = (le_u32(&d, 36 + 8), le_u32(&d, 36 + 12))
                    && end > start
                {
                    meta.loop_points = Some((start, end));
                }
                skip_pad(&mut f, size)?;
            }
            b"acid" if small && size >= 24 => {
                let d = read_vec(&mut f, size)?;
                let flags = le_u32(&d, 0).unwrap_or(0);
                meta.acid_one_shot = Some(flags & 1 == 1);
                if flags & 2 == 2 {
                    meta.root_note = d.get(4..6).map(|x| u16::from_le_bytes([x[0], x[1]]));
                }
                let tempo = d.get(20..24).map_or(0.0, |x| f32::from_le_bytes([x[0], x[1], x[2], x[3]]));
                if (20.0..400.0).contains(&tempo) {
                    meta.tempo = Some(tempo);
                }
                skip_pad(&mut f, size)?;
            }
            b"cue " if small => {
                let d = read_vec(&mut f, size)?;
                meta.cue_points = le_u32(&d, 0).unwrap_or(0);
                skip_pad(&mut f, size)?;
            }
            _ => {
                f.seek_relative(i64::try_from(padded).unwrap_or(i64::MAX))?;
            }
        }
        pos += 8 + padded;
    }
    meta.keywords.sort();
    meta.keywords.dedup();
    Ok(meta)
}

fn read_vec(f: &mut impl Read, size: u64) -> io::Result<Vec<u8>> {
    let mut v = vec![0u8; usize::try_from(size).unwrap_or(0)];
    f.read_exact(&mut v)?;
    Ok(v)
}

fn skip_pad(f: &mut BufReader<File>, size: u64) -> io::Result<()> {
    if size % 2 == 1 {
        f.seek_relative(1)?;
    }
    Ok(())
}

/// MIDI note number to a name like "C4".
pub(crate) fn note_name(n: u16) -> String {
    const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    format!("{}{}", NAMES[usize::from(n % 12)], i32::from(n / 12) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn legacy_blend(compress: bool) -> Vec<u8> {
        let mut out = b"BLENDER-v405".to_vec();
        let block = |out: &mut Vec<u8>, code: &[u8; 4], data: &[u8]| {
            out.extend_from_slice(code);
            out.extend_from_slice(&(data.len() as i32).to_le_bytes());
            out.extend_from_slice(&0u64.to_le_bytes()); // old address
            out.extend_from_slice(&0i32.to_le_bytes()); // sdna
            out.extend_from_slice(&1i32.to_le_bytes()); // count
            out.extend_from_slice(data);
        };
        block(&mut out, b"REND", &[0u8; 72]);
        let mut test = Vec::new();
        test.extend_from_slice(&2i32.to_le_bytes());
        test.extend_from_slice(&2i32.to_le_bytes());
        // bottom row red, top row blue
        test.extend_from_slice(&[255, 0, 0, 255, 255, 0, 0, 255, 0, 0, 255, 255, 0, 0, 255, 255]);
        block(&mut out, b"TEST", &test);
        let mut im = vec![0u8; 16];
        im.extend_from_slice(b"IMrock\0");
        im.extend_from_slice(b"//textures/rock_albedo.png\0");
        im.extend_from_slice(&[0u8; 32]);
        block(&mut out, &[b'I', b'M', 0, 0], &im);
        block(&mut out, &[b'L', b'I', 0, 0], b"\0\0//../shared/props.blend\0\0");
        block(&mut out, b"ENDB", &[]);
        if compress { zstd::encode_all(&out[..], 3).unwrap() } else { out }
    }

    fn chunk(id: [u8; 4], data: &[u8]) -> Vec<u8> {
        let mut v = id.to_vec();
        v.extend_from_slice(&(data.len() as u32).to_le_bytes());
        v.extend_from_slice(data);
        if data.len() % 2 == 1 {
            v.push(0);
        }
        v
    }

    #[test]
    fn reads_wav_library_metadata() {
        let mut fmt = Vec::new();
        fmt.extend_from_slice(&1u16.to_le_bytes());
        fmt.extend_from_slice(&1u16.to_le_bytes());
        fmt.extend_from_slice(&44_100u32.to_le_bytes());
        fmt.extend_from_slice(&88_200u32.to_le_bytes());
        fmt.extend_from_slice(&2u16.to_le_bytes());
        fmt.extend_from_slice(&16u16.to_le_bytes());
        let mut bext = vec![0u8; 602];
        bext[..34].copy_from_slice(b"Short crystalline ice cast, bright");
        let mut info = b"INFO".to_vec();
        info.extend(chunk(*b"INAM", b"Ice Cast 35\0"));
        info.extend(chunk(*b"IKEY", b"ice; cast; shatter\0"));
        let ixml = b"<BWFXML><USER><CATEGORY>MAGIC</CATEGORY><SUBCATEGORY>ICE</SUBCATEGORY><CATID>MAGIce</CATID></USER></BWFXML>";
        let mut smpl = vec![0u8; 36 + 24];
        smpl[28..32].copy_from_slice(&1u32.to_le_bytes());
        smpl[36 + 8..36 + 12].copy_from_slice(&100u32.to_le_bytes());
        smpl[36 + 12..36 + 16].copy_from_slice(&44_000u32.to_le_bytes());
        let mut acid = vec![0u8; 24];
        acid[0..4].copy_from_slice(&2u32.to_le_bytes()); // loop, root note set
        acid[4..6].copy_from_slice(&60u16.to_le_bytes());
        acid[20..24].copy_from_slice(&120.0f32.to_le_bytes());
        let mut body = b"WAVE".to_vec();
        for c in [
            chunk(*b"fmt ", &fmt),
            chunk(*b"bext", &bext),
            chunk(*b"data", &[0u8; 400]),
            chunk(*b"LIST", &info),
            chunk(*b"iXML", ixml),
            chunk(*b"smpl", &smpl),
            chunk(*b"acid", &acid),
        ] {
            body.extend(c);
        }
        let mut wav = b"RIFF".to_vec();
        wav.extend_from_slice(&(body.len() as u32).to_le_bytes());
        wav.extend(body);
        let p = std::env::temp_dir().join("hoard-riff-test.wav");
        File::create(&p).unwrap().write_all(&wav).unwrap();
        let m = read_riff(&p).unwrap();
        assert_eq!(m.description.as_deref(), Some("Short crystalline ice cast, bright"));
        assert_eq!(m.title.as_deref(), Some("Ice Cast 35"));
        assert_eq!(m.keywords, ["cast", "ice", "shatter"]);
        assert_eq!((m.ucs_category.as_deref(), m.ucs_subcategory.as_deref()), (Some("MAGIC"), Some("ICE")));
        assert_eq!(m.loop_points, Some((100, 44_000)));
        assert_eq!(
            (m.acid_one_shot, m.tempo, m.root_note.map(note_name)),
            (Some(false), Some(120.0), Some("C4".into()))
        );
    }

    #[test]
    fn reads_blend_preview_and_paths() {
        for compress in [false, true] {
            let dir = std::env::temp_dir().join(format!("hoard-blend-test-{compress}"));
            std::fs::create_dir_all(&dir).unwrap();
            let p = dir.join("scene.blend");
            File::create(&p).unwrap().write_all(&legacy_blend(compress)).unwrap();
            let info = read_blend(&p, true).unwrap();
            assert_eq!(info.version, "4.5");
            assert_eq!(info.compression, compress.then_some("zstd"));
            let thumb = info.thumbnail.unwrap();
            assert_eq!(thumb.dimensions(), (2, 2));
            assert_eq!(thumb.get_pixel(0, 0).0, [0, 0, 255, 255], "flipped so the top row is first");
            assert_eq!(info.image_paths, ["//textures/rock_albedo.png"]);
            assert_eq!(info.library_paths, ["//../shared/props.blend"]);
        }
    }
}
