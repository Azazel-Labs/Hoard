//! Vector art drawn at the size it's asked for: SVG (with the machine's fonts for its text) and Illustrator files (their
//! PDF's first page, or the preview their XMP metadata keeps).

use std::path::Path;

use anyhow::{Context, Result, ensure};
use image::{DynamicImage, RgbaImage};
use serde_json::{Map, Value, json};

use super::Decoded;

/// The machine's fonts, loaded once, with the generic families pointed at fonts that exist here.
fn font_db() -> std::sync::Arc<resvg::usvg::fontdb::Database> {
    static DB: std::sync::OnceLock<std::sync::Arc<resvg::usvg::fontdb::Database>> = std::sync::OnceLock::new();
    std::sync::Arc::clone(DB.get_or_init(|| {
        let mut db = resvg::usvg::fontdb::Database::new();
        db.load_system_fonts();
        let families: std::collections::HashSet<String> =
            db.faces().flat_map(|f| f.families.iter().map(|(n, _)| n.clone())).collect();
        let pick = |names: &[&str]| names.iter().find(|n| families.contains(**n)).map(|n| (*n).to_owned());
        let (sans, serif, mono) = (
            pick(&["DejaVu Sans", "Liberation Sans", "Noto Sans", "Arial", "Helvetica", "Segoe UI"]),
            pick(&["DejaVu Serif", "Liberation Serif", "Noto Serif", "Times New Roman", "Times"]),
            pick(&["DejaVu Sans Mono", "Liberation Mono", "Noto Sans Mono", "Courier New", "Menlo"]),
        );
        if let Some(f) = sans {
            db.set_sans_serif_family(f);
        }
        if let Some(f) = serif {
            db.set_serif_family(f);
        }
        if let Some(f) = mono {
            db.set_monospace_family(f);
        }
        tracing::debug!(faces = db.len(), "fonts for SVG text loaded");
        std::sync::Arc::new(db)
    }))
}

/// Render an SVG with its longest side at `size` pixels (vector art has no native size worth keeping small). Text is
/// drawn with the machine's fonts; families an SVG asks for that aren't installed fall back to a sans-serif.
pub(super) fn svg(path: &Path, size: u32) -> Result<Decoded> {
    let data = std::fs::read(path)?;
    let opts = resvg::usvg::Options { fontdb: font_db(), font_family: "sans-serif".into(), ..Default::default() };
    let tree = resvg::usvg::Tree::from_data(&data, &opts)?;
    let s = tree.size();
    let scale = size as f32 / s.width().max(s.height()).max(1.0);
    let (w, h) = (((s.width() * scale).ceil() as u32).max(1), ((s.height() * scale).ceil() as u32).max(1));
    let mut pixmap = resvg::tiny_skia::Pixmap::new(w, h).context("SVG size")?;
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    // tiny-skia stores premultiplied RGBA.
    let rgba: Vec<u8> = pixmap
        .pixels()
        .iter()
        .flat_map(|p| {
            let c = p.demultiply();
            [c.red(), c.green(), c.blue(), c.alpha()]
        })
        .collect();
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, rgba).context("size")?);
    Ok(Decoded { image, facts: vector_facts(s.width(), s.height()) })
}

/// Vector art at `size` pixels on its longer side.
pub(crate) fn vector(path: &Path, ext: &str, size: u32) -> Result<Decoded> {
    match ext {
        "ai" => illustrator(path, size),
        _ => svg(path, size),
    }
}

/// What's said of vector art: that it is, and its own size (in its units), which is its `width` and `height`.
fn vector_facts(width: f32, height: f32) -> Map<String, Value> {
    let mut facts = Map::new();
    facts.insert("vector".into(), json!(true));
    facts.insert("vector_width".into(), json!(width.round() as u32));
    facts.insert("vector_height".into(), json!(height.round() as u32));
    facts
}

/// Adobe Illustrator art. Since Illustrator 9 a file is a PDF too, and its first artboard is drawn like a PDF page.
/// Older files (PostScript), and newer ones saved without their PDF content, show the preview Illustrator keeps in
/// their XMP metadata instead.
pub(super) fn illustrator(path: &Path, size: u32) -> Result<Decoded> {
    let data = std::fs::read(path)?;
    if data.starts_with(b"%PDF")
        && let Some(drawn) = pdf_page(data.clone(), size)?
    {
        return Ok(drawn);
    }
    let image =
        xmp_thumbnail(&data).context("this Illustrator file has neither a drawing Hoard can read nor a preview")?;
    let mut facts = vector_facts(image.width() as f32, image.height() as f32);
    // The preview is only small: its size isn't the art's.
    facts.remove("vector_width");
    facts.remove("vector_height");
    Ok(Decoded { image, facts })
}

/// A PDF's first page drawn `size` pixels on its longer side, over transparency. `None` for Illustrator's stand-in
/// page ("saved without PDF Content").
fn pdf_page(data: Vec<u8>, size: u32) -> Result<Option<Decoded>> {
    use hayro::hayro_interpret::InterpreterSettings;
    use hayro::vello_cpu::color::palette::css::TRANSPARENT;
    use hayro::vello_cpu::peniko::ImageAlphaType;
    let pdf = hayro::hayro_syntax::Pdf::new(data).map_err(|e| anyhow::anyhow!("not a readable PDF: {e:?}"))?;
    let page = pdf.pages().first().context("the PDF has no pages")?;
    if page.page_stream().is_some_and(|s| s.windows(19).any(|w| w == b"without PDF Content")) {
        return Ok(None);
    }
    let (w, h) = page.render_dimensions();
    ensure!(w >= 1.0 && h >= 1.0, "the page has no size");
    let scale = size.min(u32::from(u16::MAX)) as f32 / w.max(h);
    let settings = hayro::PixmapSettings { x_scale: scale, y_scale: scale, bg_color: TRANSPARENT };
    let pixmap = hayro::render(
        page,
        &hayro::RenderCache::new(),
        &InterpreterSettings::default(),
        &hayro::RenderSettings::default(),
        &settings,
    );
    let (pw, ph) = (u32::from(pixmap.width()), u32::from(pixmap.height()));
    ensure!(pw > 0 && ph > 0, "the page drew nothing");
    let rgba = pixmap.take_rgba8(ImageAlphaType::Alpha);
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(pw, ph, rgba).context("size")?);
    Ok(Some(Decoded { image, facts: vector_facts(w, h) }))
}

/// The JPEG preview an Adobe file keeps in its XMP metadata (`xmpGImg:image`, base64 with `&#xA;` line breaks).
fn xmp_thumbnail(data: &[u8]) -> Option<DynamicImage> {
    use base64::Engine as _;
    const OPEN: &[u8] = b"<xmpGImg:image>";
    let start = data.windows(OPEN.len()).position(|w| w == OPEN)? + OPEN.len();
    let len = data[start..].iter().position(|&b| b == b'<')?;
    let text = std::str::from_utf8(&data[start..start + len]).ok()?.replace("&#xA;", "");
    let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let jpeg = base64::engine::general_purpose::STANDARD.decode(text).ok()?;
    image::load_from_memory_with_format(&jpeg, image::ImageFormat::Jpeg).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::textures::decode;
    use crate::textures::tests::write;

    #[test]
    fn svg_renders() {
        let dir = tempfile::tempdir().unwrap();
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10"><rect width="10" height="10" fill="#ff0000"/></svg>"##;
        let d = decode(&write(&dir, "i.svg", svg), "svg").unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.dimensions(), (1024, 512));
        assert_eq!(img.get_pixel(100, 100).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(900, 100).0[3], 0);
    }

    /// A PDF with one 20x10 page whose content is `content`, with a correct cross-reference table.
    fn pdf(content: &str) -> Vec<u8> {
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_owned(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 20 10] /Contents 4 0 R >>".to_owned(),
            format!(
                "<< /Length {} >>
stream
{content}
endstream",
                content.len()
            ),
        ];
        let mut out = b"%PDF-1.5
"
        .to_vec();
        let mut offsets = Vec::new();
        for (i, o) in objects.iter().enumerate() {
            offsets.push(out.len());
            out.extend(
                format!(
                    "{} 0 obj
{o}
endobj
",
                    i + 1
                )
                .bytes(),
            );
        }
        let xref = out.len();
        out.extend(
            format!(
                "xref
0 {}
0000000000 65535 f 
",
                objects.len() + 1
            )
            .bytes(),
        );
        for o in offsets {
            out.extend(
                format!(
                    "{o:010} 00000 n 
"
                )
                .bytes(),
            );
        }
        out.extend(
            format!(
                "trailer
<< /Size {} /Root 1 0 R >>
startxref
{xref}
%%EOF
",
                objects.len() + 1
            )
            .bytes(),
        );
        out
    }

    #[test]
    fn illustrator_art_is_drawn_from_its_pdf() {
        let dir = tempfile::tempdir().unwrap();
        // Red over the left half of the artboard.
        let d = illustrator(&write(&dir, "icons.ai", &pdf("1 0 0 rg 0 0 10 10 re f")), 200).unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.dimensions(), (200, 100));
        assert_eq!(img.get_pixel(40, 50).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(160, 50).0[3], 0, "the rest is see-through");
        assert_eq!((d.facts["vector_width"].clone(), d.facts["vector_height"].clone()), (json!(20), json!(10)));
        assert_eq!(decode(&write(&dir, "big.ai", &pdf("0 0 1 rg 0 0 20 10 re f")), "ai").unwrap().image.width(), 2048);
    }

    #[test]
    fn illustrator_art_without_a_drawing_shows_its_preview() {
        use base64::Engine as _;
        let mut jpeg = Vec::new();
        DynamicImage::ImageRgb8(image::RgbImage::from_pixel(8, 4, image::Rgb([0, 200, 0])))
            .write_to(&mut std::io::Cursor::new(&mut jpeg), image::ImageFormat::Jpeg)
            .unwrap();
        let b64 = base64::engine::general_purpose::STANDARD.encode(&jpeg);
        let (a, b) = b64.split_at(b64.len() / 2);
        let xmp = format!("<xmpGImg:width>8</xmpGImg:width><xmpGImg:image>{a}&#xA;{b}</xmpGImg:image>");
        let dir = tempfile::tempdir().unwrap();
        // Illustrator 8 and earlier: PostScript.
        let old = format!(
            "%!PS-Adobe-3.0
%%Creator: Adobe Illustrator(R) 8.0
{xmp}
%%EOF
"
        );
        let d = illustrator(&write(&dir, "old.ai", old.as_bytes()), 512).unwrap();
        assert_eq!((d.image.width(), d.image.height()), (8, 4));
        assert!(d.image.to_rgb8().get_pixel(4, 2).0[1] > 150);
        assert!(!d.facts.contains_key("vector_width"));
        // Saved without PDF content: the page only says so.
        let mut stand_in = pdf("BT /F1 12 Tf (saved without PDF Content.) Tj ET");
        stand_in.extend(xmp.bytes());
        assert_eq!(
            illustrator(&write(&dir, "no-pdf.ai", &stand_in), 512).unwrap().image.to_rgba8().dimensions(),
            (8, 4)
        );
        assert!(
            illustrator(
                &write(
                    &dir,
                    "junk.ai",
                    b"%!PS-Adobe-3.0
"
                ),
                512
            )
            .is_err()
        );
    }

    #[test]
    fn svg_text_renders_with_system_fonts() {
        if font_db().is_empty() {
            return; // no fonts on this machine: text can't be drawn
        }
        let dir = tempfile::tempdir().unwrap();
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="50"><text x="10" y="40" font-family="SomeFontNobodyHas, sans-serif" font-size="40" fill="#000">Hello</text></svg>"##;
        let img = decode(&write(&dir, "t.svg", svg), "svg").unwrap().image.to_rgba8();
        let inked = img.pixels().filter(|p| p[3] > 128).count();
        assert!(inked > 1000, "text drew {inked} pixels");
    }
}
