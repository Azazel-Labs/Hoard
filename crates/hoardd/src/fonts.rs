//! Fonts: names, style and coverage facts, and a specimen thumbnail. TTF, OTF, TTC (first face), WOFF and WOFF2,
//! plus conversion between TTF/OTF, WOFF and WOFF2 for "download as".

#![expect(clippy::many_single_char_names, reason = "pixel code: x/y/s/b")]

use std::io::Read;
use std::path::Path;

use ab_glyph::{Font as _, FontRef, PxScale, ScaleFont as _};
use anyhow::{Context, Result, bail, ensure};
use image::{Rgba, RgbaImage};
use serde_json::{Map, Value, json};

pub(crate) const EXTS: &[&str] = &["ttf", "otf", "ttc", "otc", "woff", "woff2"];

/// The font as an sfnt (TrueType/OpenType bytes), unwrapping WOFF.
pub(crate) fn sfnt(path: &Path, ext: &str) -> Result<Vec<u8>> {
    let data = std::fs::read(path)?;
    if ext == "woff" || data.starts_with(b"wOFF") {
        return unwoff(&data);
    }
    if ext == "woff2" || data.starts_with(b"wOF2") {
        return unwoff2(&data);
    }
    Ok(data)
}

/// WOFF2 → sfnt: `allsorts` decompresses and undoes the glyf/loca/hmtx transforms; the tables are reassembled here.
fn unwoff2(data: &[u8]) -> Result<Vec<u8>> {
    use allsorts::tables::{FontTableProvider as _, SfntVersion as _};
    let font = allsorts::binary::read::ReadScope::new(data)
        .read::<allsorts::woff2::Woff2Font<'_>>()
        .map_err(|e| anyhow::anyhow!("not a readable WOFF2 font: {e}"))?;
    let provider = font.table_provider(0).map_err(|e| anyhow::anyhow!("not a readable WOFF2 font: {e}"))?;
    let mut tags = provider.table_tags().context("WOFF2 font lists no tables")?;
    tags.sort_unstable();
    let mut tables = Vec::with_capacity(tags.len());
    for tag in tags {
        let data = provider.read_table_data(tag).map_err(|e| anyhow::anyhow!("WOFF2 table: {e}"))?.into_owned();
        tables.push((tag.to_be_bytes(), data));
    }
    assemble_sfnt(provider.sfnt_version(), tables)
}

fn table_checksum(data: &[u8]) -> u32 {
    data.chunks(4).fold(0u32, |sum, c| {
        let mut word = [0u8; 4];
        word[..c.len()].copy_from_slice(c);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

/// Build an sfnt from tables sorted by tag, with checksums and `head`'s checksum adjustment filled in.
fn assemble_sfnt(flavor: u32, mut tables: Vec<([u8; 4], Vec<u8>)>) -> Result<Vec<u8>> {
    let n = tables.len();
    ensure!(n > 0, "font has no tables");
    // head.checkSumAdjustment is computed over the whole font with the field zeroed.
    if let Some((_, head)) = tables.iter_mut().find(|(t, _)| t == b"head")
        && head.len() >= 12
    {
        head[8..12].fill(0);
    }
    let n16 = u16::try_from(n)?;
    let pow = 1u16 << (15 - n16.leading_zeros().min(15));
    let mut out = Vec::new();
    out.extend_from_slice(&flavor.to_be_bytes());
    for v in [n16, pow * 16, u16::try_from(pow.trailing_zeros())?, n16 * 16 - pow * 16] {
        out.extend_from_slice(&v.to_be_bytes());
    }
    let mut offset = 12 + 16 * n;
    let mut head_at = None;
    for (tag, data) in &tables {
        out.extend_from_slice(tag);
        out.extend_from_slice(&table_checksum(data).to_be_bytes());
        out.extend_from_slice(&u32::try_from(offset)?.to_be_bytes());
        out.extend_from_slice(&u32::try_from(data.len())?.to_be_bytes());
        if tag == b"head" {
            head_at = Some(offset);
        }
        offset += data.len().div_ceil(4) * 4;
    }
    for (_, data) in &tables {
        out.extend_from_slice(data);
        out.resize(out.len().div_ceil(4) * 4, 0);
    }
    if let Some(at) = head_at.filter(|&at| at + 12 <= out.len()) {
        let adjust = 0xB1B0_AFBAu32.wrapping_sub(table_checksum(&out));
        out[at + 8..at + 12].copy_from_slice(&adjust.to_be_bytes());
    }
    Ok(out)
}

/// The tables of an sfnt: `(tag, checksum, data)`, in directory order.
/// One table of an sfnt: tag, checksum, contents.
type Table<'a> = ([u8; 4], u32, &'a [u8]);

fn sfnt_tables(sfnt: &[u8]) -> Result<Vec<Table<'_>>> {
    let be32 = |at: usize| -> Result<u32> {
        Ok(u32::from_be_bytes(sfnt.get(at..at + 4).context("font ends early")?.try_into()?))
    };
    ensure!(!sfnt.starts_with(b"ttcf"), "font collections can't be converted (pick one font from it first)");
    let n = usize::from(u16::from_be_bytes(sfnt.get(4..6).context("font ends early")?.try_into()?));
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let at = 12 + i * 16;
        let tag: [u8; 4] = sfnt.get(at..at + 4).context("font ends early")?.try_into()?;
        let (checksum, offset, len) = (be32(at + 4)?, be32(at + 8)? as usize, be32(at + 12)? as usize);
        out.push((tag, checksum, sfnt.get(offset..offset + len).context("font table runs past the end")?));
    }
    Ok(out)
}

/// sfnt → WOFF 1: each table zlib-compressed when that makes it smaller.
pub(crate) fn to_woff(sfnt: &[u8]) -> Result<Vec<u8>> {
    use std::io::Write as _;
    let tables = sfnt_tables(sfnt)?;
    let n = tables.len();
    let header_len = 44 + 20 * n;
    let (mut dir, mut body) = (Vec::new(), Vec::new());
    let mut total_sfnt = 12 + 16 * n;
    for (tag, checksum, data) in &tables {
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        z.write_all(data)?;
        let packed = z.finish()?;
        let stored: &[u8] = if packed.len() < data.len() { &packed } else { data };
        dir.extend_from_slice(tag);
        dir.extend_from_slice(&u32::try_from(header_len + body.len())?.to_be_bytes());
        dir.extend_from_slice(&u32::try_from(stored.len())?.to_be_bytes());
        dir.extend_from_slice(&u32::try_from(data.len())?.to_be_bytes());
        dir.extend_from_slice(&checksum.to_be_bytes());
        body.extend_from_slice(stored);
        body.resize(body.len().div_ceil(4) * 4, 0);
        total_sfnt += data.len().div_ceil(4) * 4;
    }
    let mut w = b"wOFF".to_vec();
    w.extend_from_slice(&sfnt[0..4]);
    w.extend_from_slice(&u32::try_from(header_len + body.len())?.to_be_bytes());
    w.extend_from_slice(&u16::try_from(n)?.to_be_bytes());
    w.extend_from_slice(&0u16.to_be_bytes());
    w.extend_from_slice(&u32::try_from(total_sfnt)?.to_be_bytes());
    w.extend_from_slice(&[0, 1, 0, 0]); // font version 1.0
    w.extend_from_slice(&[0; 20]); // no metadata or private block
    w.extend(dir);
    w.extend(body);
    Ok(w)
}

/// WOFF2's table tags that get a one-byte code instead of being spelled out.
const WOFF2_TAGS: [&[u8; 4]; 63] = [
    b"cmap", b"head", b"hhea", b"hmtx", b"maxp", b"name", b"OS/2", b"post", b"cvt ", b"fpgm", b"glyf", b"loca",
    b"prep", b"CFF ", b"VORG", b"EBDT", b"EBLC", b"gasp", b"hdmx", b"kern", b"LTSH", b"PCLT", b"VDMX", b"vhea",
    b"vmtx", b"BASE", b"GDEF", b"GPOS", b"GSUB", b"EBSC", b"JSTF", b"MATH", b"CBDT", b"CBLC", b"COLR", b"CPAL",
    b"SVG ", b"sbix", b"acnt", b"avar", b"bdat", b"bloc", b"bsln", b"cvar", b"fdsc", b"feat", b"fmtx", b"fvar",
    b"gvar", b"hsty", b"just", b"lcar", b"mort", b"morx", b"opbd", b"prop", b"trak", b"Zapf", b"Silf", b"Glat",
    b"Gloc", b"Feat", b"Sill",
];

fn base128(mut v: u32, out: &mut Vec<u8>) {
    let mut bytes = vec![(v & 0x7f) as u8];
    v >>= 7;
    while v > 0 {
        bytes.push(((v & 0x7f) as u8) | 0x80);
        v >>= 7;
    }
    out.extend(bytes.iter().rev());
}

/// sfnt → WOFF2, with every table untransformed (allowed by the spec, and what keeps this simple) and the whole
/// table stream brotli-compressed.
pub(crate) fn to_woff2(sfnt: &[u8]) -> Result<Vec<u8>> {
    let tables = sfnt_tables(sfnt)?;
    let mut dir = Vec::new();
    let mut stream = Vec::new();
    let mut total_sfnt = 12 + 16 * tables.len();
    for (tag, _, data) in &tables {
        // glyf and loca's null transform is version 3; every other table's is version 0.
        let version = if tag == b"glyf" || tag == b"loca" { 3u8 } else { 0 };
        if let Some(i) = WOFF2_TAGS.iter().position(|t| *t == tag) {
            dir.push(u8::try_from(i)? | (version << 6));
        } else {
            dir.push(63 | (version << 6));
            dir.extend_from_slice(tag);
        }
        base128(u32::try_from(data.len())?, &mut dir);
        stream.extend_from_slice(data);
        total_sfnt += data.len().div_ceil(4) * 4;
    }
    let mut compressed = Vec::new();
    let params = brotli::enc::BrotliEncoderParams {
        quality: 11,
        lgwin: 22,
        mode: brotli::enc::backward_references::BrotliEncoderMode::BROTLI_MODE_FONT,
        ..Default::default()
    };
    brotli::BrotliCompress(&mut stream.as_slice(), &mut compressed, &params)?;
    let header_len = 48 + dir.len();
    let total = (header_len + compressed.len()).div_ceil(4) * 4;
    let mut w = b"wOF2".to_vec();
    w.extend_from_slice(&sfnt[0..4]);
    w.extend_from_slice(&u32::try_from(total)?.to_be_bytes());
    w.extend_from_slice(&u16::try_from(tables.len())?.to_be_bytes());
    w.extend_from_slice(&0u16.to_be_bytes());
    w.extend_from_slice(&u32::try_from(total_sfnt)?.to_be_bytes());
    w.extend_from_slice(&u32::try_from(compressed.len())?.to_be_bytes());
    w.extend_from_slice(&[0, 1, 0, 0]); // font version 1.0
    w.extend_from_slice(&[0; 20]); // no metadata or private block
    w.extend(dir);
    w.extend(compressed);
    w.resize(total, 0);
    Ok(w)
}

/// WOFF 1 → sfnt: the same tables, each zlib-inflated when it was compressed, under a rebuilt table directory.
fn unwoff(w: &[u8]) -> Result<Vec<u8>> {
    let be32 = |at: usize| -> Result<u32> {
        Ok(u32::from_be_bytes(w.get(at..at + 4).context("WOFF ends early")?.try_into()?))
    };
    let be16 = |at: usize| -> Result<u16> {
        Ok(u16::from_be_bytes(w.get(at..at + 2).context("WOFF ends early")?.try_into()?))
    };
    ensure!(w.starts_with(b"wOFF"), "not a WOFF font");
    let flavor = be32(4)?;
    let n = usize::from(be16(12)?);
    ensure!(n > 0 && n < 512, "implausible WOFF table count");
    let mut tables = Vec::with_capacity(n);
    for i in 0..n {
        let at = 44 + i * 20;
        let (tag, offset, comp, orig, checksum) =
            (be32(at)?, be32(at + 4)? as usize, be32(at + 8)? as usize, be32(at + 12)? as usize, be32(at + 16)?);
        ensure!(orig <= 64 << 20, "WOFF table is too large");
        let raw = w.get(offset..offset + comp).context("WOFF table runs past the end")?;
        let data = if comp < orig {
            let mut out = Vec::with_capacity(orig);
            flate2::read::ZlibDecoder::new(raw).take(orig as u64).read_to_end(&mut out)?;
            ensure!(out.len() == orig, "WOFF table inflated to the wrong size");
            out
        } else {
            raw.to_vec()
        };
        tables.push((tag, checksum, data));
    }
    tables.sort_by_key(|t| t.0);
    let n16 = u16::try_from(n)?;
    let pow = 1u16 << (15 - n16.leading_zeros().min(15));
    let search = pow * 16;
    let selector = u16::try_from(pow.trailing_zeros())?;
    let mut out = Vec::new();
    out.extend_from_slice(&flavor.to_be_bytes());
    for v in [n16, search, selector, n16 * 16 - search] {
        out.extend_from_slice(&v.to_be_bytes());
    }
    let mut offset = 12 + 16 * n;
    let mut body = Vec::new();
    for (tag, checksum, data) in &tables {
        out.extend_from_slice(&tag.to_be_bytes());
        out.extend_from_slice(&checksum.to_be_bytes());
        out.extend_from_slice(&u32::try_from(offset)?.to_be_bytes());
        out.extend_from_slice(&u32::try_from(data.len())?.to_be_bytes());
        body.extend_from_slice(data);
        let padded = data.len().div_ceil(4) * 4;
        body.resize(body.len() + padded - data.len(), 0);
        offset += padded;
    }
    out.extend(body);
    Ok(out)
}

/// Whether an sfnt holds CFF outlines (so it's an `.otf`) rather than TrueType (`.ttf`).
pub(crate) fn is_cff(sfnt: &[u8]) -> bool {
    sfnt.starts_with(b"OTTO")
}

pub(crate) struct FontInfo {
    pub(crate) facts: Map<String, Value>,
    /// Family and style, e.g. "Inter Bold".
    pub(crate) title: Option<String>,
    pub(crate) thumb: Option<RgbaImage>,
}

fn name(face: &ttf_parser::Face<'_>, ids: &[u16]) -> Option<String> {
    for id in ids {
        if let Some(n) = face.names().into_iter().filter(|n| n.name_id == *id).find_map(|n| n.to_string()) {
            let n = n.trim().to_owned();
            if !n.is_empty() {
                return Some(n);
            }
        }
    }
    None
}

pub(crate) fn analyze(path: &Path, ext: &str, thumb_size: u32) -> Result<FontInfo> {
    let data = sfnt(path, ext)?;
    let face = ttf_parser::Face::parse(&data, 0).map_err(|e| anyhow::anyhow!("not a readable font: {e}"))?;
    let mut f = Map::new();
    let family = name(&face, &[16, 1]);
    let style = name(&face, &[17, 2]);
    if let Some(v) = &family {
        f.insert("family".into(), json!(v));
    }
    if let Some(v) = &style {
        f.insert("style".into(), json!(v));
    }
    for (key, id) in [("designer", 9), ("version", 5), ("license", 13), ("license_url", 14)] {
        if let Some(v) = name(&face, &[id]) {
            f.insert(key.into(), json!(v.chars().take(300).collect::<String>()));
        }
    }
    f.insert("glyphs".into(), json!(face.number_of_glyphs()));
    f.insert("weight".into(), json!(face.weight().to_number()));
    if face.is_italic() {
        f.insert("italic".into(), json!(true));
    }
    if face.is_monospaced() {
        f.insert("monospace".into(), json!(true));
    }
    if face.is_variable() {
        let axes: Vec<String> = face.variation_axes().into_iter().map(|a| a.tag.to_string()).collect();
        f.insert("variable_axes".into(), json!(axes));
    }
    f.insert("outlines".into(), json!(if is_cff(&data) { "cff" } else { "truetype" }));
    // Which writing systems it covers, judged by a few telltale characters each.
    let covers = |chars: &str| chars.chars().all(|c| face.glyph_index(c).is_some());
    let scripts: Vec<&str> = [
        ("latin", "AZaz09"),
        ("latin-ext", "ĄŁŽđŐ"),
        ("cyrillic", "АЯаяЖ"),
        ("greek", "ΑΩαωλ"),
        ("vietnamese", "ẠẾỗự"),
        ("arabic", "ابتث"),
        ("hebrew", "אבגד"),
        ("devanagari", "अआकख"),
        ("thai", "กขคง"),
        ("cjk", "中国人日"),
        ("japanese-kana", "あいアイ"),
        ("korean", "가나다한"),
    ]
    .into_iter()
    .filter(|(_, chars)| covers(chars))
    .map(|(s, _)| s)
    .collect();
    f.insert("scripts".into(), json!(scripts));
    let icons = scripts.is_empty() && face.number_of_glyphs() > 20;
    if icons {
        f.insert("symbols".into(), json!(true));
    }
    let title = match (&family, &style) {
        (Some(fam), Some(st)) if !matches!(st.as_str(), "Regular" | "Normal" | "Book" | "Roman") => {
            Some(format!("{fam} {st}"))
        }
        (Some(fam), _) => Some(fam.clone()),
        _ => None,
    };
    let thumb = specimen(&data, thumb_size, if icons { None } else { Some("Aa") }).ok();
    Ok(FontInfo { facts: f, title, thumb })
}

/// White glyphs on transparent: "Aa" large over a pangram line, or the font's first glyphs for symbol fonts.
fn specimen(data: &[u8], size: u32, sample: Option<&str>) -> Result<RgbaImage> {
    let font = FontRef::try_from_slice(data).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut img = RgbaImage::new(size, size);
    let s = size as f32;
    let draw = |img: &mut RgbaImage, text: &str, px: f32, baseline: f32| -> Result<()> {
        let scaled = font.as_scaled(PxScale::from(px));
        let width: f32 = text.chars().map(|c| scaled.h_advance(font.glyph_id(c))).sum();
        let mut x = ((s - width) / 2.0).max(s * 0.04);
        for c in text.chars() {
            let id = font.glyph_id(c);
            let glyph = id.with_scale_and_position(PxScale::from(px), ab_glyph::point(x, baseline));
            x += scaled.h_advance(id);
            if let Some(outline) = font.outline_glyph(glyph) {
                let b = outline.px_bounds();
                outline.draw(|gx, gy, cov| {
                    let (px, py) = (b.min.x as i64 + i64::from(gx), b.min.y as i64 + i64::from(gy));
                    if px >= 0 && py >= 0 && (px as u32) < size && (py as u32) < size {
                        let p = img.get_pixel_mut(px as u32, py as u32);
                        let a = (cov.clamp(0.0, 1.0) * 255.0) as u8;
                        *p = Rgba([236, 238, 242, p[3].max(a)]);
                    }
                });
            }
        }
        Ok(())
    };
    if let Some(big) = sample {
        // Leave the bottom-left corner clear: the grid puts the file-type badge there.
        draw(&mut img, big, s * 0.46, s * 0.5)?;
        draw(&mut img, "Quick fox 0123", s * 0.095, s * 0.69)?;
    } else {
        // Symbol fonts: the first few glyphs that have outlines, in a grid.
        let mut drawn = 0u32;
        let cell = s / 4.0;
        for gid in 1..font.glyph_count().min(400) {
            if drawn == 16 {
                break;
            }
            let id = ab_glyph::GlyphId(u16::try_from(gid)?);
            let (cx, cy) = ((drawn % 4) as f32 * cell, (drawn / 4) as f32 * cell);
            let glyph = id
                .with_scale_and_position(PxScale::from(cell * 0.7), ab_glyph::point(cx + cell * 0.15, cy + cell * 0.8));
            let Some(outline) = font.outline_glyph(glyph) else { continue };
            let b = outline.px_bounds();
            outline.draw(|gx, gy, cov| {
                let (px, py) = (b.min.x as i64 + i64::from(gx), b.min.y as i64 + i64::from(gy));
                if px >= 0 && py >= 0 && (px as u32) < size && (py as u32) < size {
                    let p = img.get_pixel_mut(px as u32, py as u32);
                    *p = Rgba([236, 238, 242, p[3].max((cov.clamp(0.0, 1.0) * 255.0) as u8)]);
                }
            });
            drawn += 1;
        }
        if drawn == 0 {
            bail!("no drawable glyphs");
        }
    }
    Ok(img)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    /// A `TrueType` font to test with: `HOARD_TEST_FONT`, or `DejaVu` Sans if installed.
    fn test_font() -> Option<Vec<u8>> {
        let path = std::env::var("HOARD_TEST_FONT").ok().or_else(|| {
            [
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                "/usr/local/lib/python3.13/dist-packages/matplotlib/mpl-data/fonts/ttf/DejaVuSans.ttf",
            ]
            .into_iter()
            .find(|p| Path::new(p).is_file())
            .map(str::to_owned)
        })?;
        std::fs::read(path).ok()
    }

    /// Wrap an sfnt as WOFF 1 (every table zlib-compressed).
    fn woff(sfnt: &[u8]) -> Vec<u8> {
        let n = usize::from(u16::from_be_bytes([sfnt[4], sfnt[5]]));
        let mut dir = Vec::new();
        let mut body = Vec::new();
        let header_len = 44 + 20 * n;
        for i in 0..n {
            let e = &sfnt[12 + i * 16..28 + i * 16];
            let (off, len) = (
                u32::from_be_bytes(e[8..12].try_into().unwrap()) as usize,
                u32::from_be_bytes(e[12..16].try_into().unwrap()) as usize,
            );
            let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
            z.write_all(&sfnt[off..off + len]).unwrap();
            let comp = z.finish().unwrap();
            let comp = if comp.len() < len { comp } else { sfnt[off..off + len].to_vec() };
            dir.extend_from_slice(&e[0..4]);
            dir.extend_from_slice(&((header_len + body.len()) as u32).to_be_bytes());
            dir.extend_from_slice(&(comp.len() as u32).to_be_bytes());
            dir.extend_from_slice(&(len as u32).to_be_bytes());
            dir.extend_from_slice(&e[4..8]);
            body.extend_from_slice(&comp);
            body.resize(body.len().div_ceil(4) * 4, 0);
        }
        let mut w = b"wOFF".to_vec();
        w.extend_from_slice(&sfnt[0..4]);
        w.extend_from_slice(&((header_len + body.len()) as u32).to_be_bytes());
        w.extend_from_slice(&(n as u16).to_be_bytes());
        w.resize(44, 0);
        w.extend(dir);
        w.extend(body);
        w
    }

    #[test]
    fn font_facts_specimen_and_woff() {
        let Some(ttf) = test_font() else { return };
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("f.ttf");
        std::fs::write(&p, &ttf).unwrap();
        let info = analyze(&p, "ttf", 128).unwrap();
        assert!(info.facts.contains_key("family"));
        assert!(info.facts["glyphs"].as_u64().unwrap() > 100);
        let scripts = info.facts["scripts"].as_array().unwrap();
        assert!(scripts.iter().any(|s| s == "latin"));
        let thumb = info.thumb.unwrap();
        assert!(thumb.pixels().filter(|p| p[3] > 128).count() > 500, "the specimen draws something");

        let w = dir.path().join("f.woff");
        std::fs::write(&w, woff(&ttf)).unwrap();
        let unwrapped = sfnt(&w, "woff").unwrap();
        let a = ttf_parser::Face::parse(&ttf, 0).unwrap();
        let b = ttf_parser::Face::parse(&unwrapped, 0).unwrap();
        assert_eq!(a.number_of_glyphs(), b.number_of_glyphs());
        assert_eq!(analyze(&w, "woff", 64).unwrap().facts["family"], info.facts["family"]);

        // Our own WOFF and WOFF2 encoders, read back by the decoders.
        for (ext, bytes) in [("woff", to_woff(&ttf).unwrap()), ("woff2", to_woff2(&ttf).unwrap())] {
            let p = dir.path().join(format!("g.{ext}"));
            std::fs::write(&p, &bytes).unwrap();
            assert!(bytes.len() < ttf.len(), "{ext} is smaller");
            let back = sfnt(&p, ext).unwrap();
            let c = ttf_parser::Face::parse(&back, 0).unwrap();
            assert_eq!(c.number_of_glyphs(), a.number_of_glyphs(), "{ext}");
            assert_eq!(analyze(&p, ext, 64).unwrap().facts["family"], info.facts["family"]);
        }
    }
}
