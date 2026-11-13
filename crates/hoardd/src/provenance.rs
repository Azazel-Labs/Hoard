//! Whether an image says it was made or edited by generative AI, and by what. Read from what the file carries: C2PA
//! Content Credentials (OpenAI, Adobe Firefly, Google and others sign their images with them), the IPTC digital
//! source type in XMP (Google, Midjourney, Adobe, Meta), and the settings Stable Diffusion front ends write
//! (`AUTOMATIC1111` and Forge, `ComfyUI`, `InvokeAI`, `NovelAI`, Fooocus). Only what the file declares: metadata can be
//! stripped, and invisible watermarks (`SynthID`) can't be read without the maker's detector.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::Result;
use flate2::read::ZlibDecoder;
use hoard_core::TagSource;
use rusqlite::Connection;
use serde_json::{Map, Value, json};
use tokio::runtime::Handle;

use crate::AppState;

/// The most kept of one metadata block: more isn't metadata.
const MAX_BLOCK: u64 = 16 << 20;
/// Formats without a reader here are searched this far in.
const MAX_HEAD: u64 = 4 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    C2pa,
    Xmp,
    Exif,
    /// A PNG text chunk, by its key.
    Text,
    /// The start of a file in a format not read here.
    Raw,
}

struct Block {
    source: Source,
    key: String,
    text: String,
}

/// What the file declares, as facts to keep (`ai`: made, by, how; `content_credentials`), or nothing.
pub(crate) fn scan(path: &Path) -> Map<String, Value> {
    let blocks = blocks(path).unwrap_or_default();
    let mut facts = Map::new();
    if blocks.iter().any(|b| b.source == Source::C2pa) {
        facts.insert("content_credentials".into(), json!(true));
    }
    if let Some(ai) = classify(&blocks) {
        facts.insert("ai".into(), ai);
    }
    facts
}

/// The tag an asset with these facts gets: `ai-generated` or `ai-edited`.
pub(crate) fn tag(meta: &Map<String, Value>) -> Option<&'static str> {
    match meta.get("ai")?.get("made")?.as_str()? {
        "generated" => Some("ai-generated"),
        "edited" => Some("ai-edited"),
        _ => None,
    }
}

/// Images read before Hoard looked for these (marked `provenance` on upgrade) looked at now, with each pack's
/// materials job. Large ones are read from their start, where the metadata nearly always is.
pub(crate) fn backfill(state: &AppState, rt: &Handle, conn: &Connection, pack_id: &str) -> Result<usize> {
    const WHOLE: u64 = 32 << 20;
    let todo: Vec<(String, String, String, i64)> = conn
        .prepare(
            "SELECT a.id, a.blob_hash, a.ext, COALESCE(b.size, 0) FROM assets a LEFT JOIN blobs b ON b.hash = a.blob_hash
              WHERE a.pack_id = ?1 AND a.kind = 'texture' AND json_extract(a.meta, '$.provenance') IS NOT NULL",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<rusqlite::Result<_>>()?;
    if todo.is_empty() {
        return Ok(0);
    }
    std::fs::create_dir_all(state.cfg.tmp_dir())?;
    let work = tempfile::Builder::new().prefix("provenance-").tempdir_in(state.cfg.tmp_dir())?;
    let mut found = 0;
    for (id, blob, ext, size) in todo {
        let file = work.path().join(format!("image.{ext}"));
        let read = if u64::try_from(size).unwrap_or(0) <= WHOLE {
            rt.block_on(state.blobs.download_to(&blob, &file))
        } else {
            rt.block_on(state.blobs.head(&blob, WHOLE)).and_then(|b| Ok(std::fs::write(&file, b)?))
        };
        let facts = match read {
            Ok(()) => scan(&file),
            Err(e) => {
                tracing::debug!(asset = %id, error = %format!("{e:#}"), "couldn't read an image for its provenance");
                Map::new()
            }
        };
        std::fs::remove_file(&file).ok();
        conn.execute(
            "UPDATE assets SET meta = json_patch(json_remove(meta, '$.provenance'), ?2) WHERE id = ?1",
            rusqlite::params![id, Value::Object(facts.clone()).to_string()],
        )?;
        if let Some(t) = tag(&facts) {
            crate::db::upsert_tag(conn, &id, t, TagSource::Meta, 1.0)?;
            found += 1;
        }
    }
    if found > 0 {
        tracing::info!(pack = pack_id, images = found, "images that say they were made with AI");
    }
    Ok(found)
}

fn classify(blocks: &[Block]) -> Option<Value> {
    // The IPTC digital source type, in XMP or a C2PA manifest's actions. Case tells the two apart.
    let declared = |needle: &str| blocks.iter().find(|b| b.text.contains(needle));
    let (made, how) = if let Some(b) = declared("trainedAlgorithmicMedia") {
        ("generated", how(b.source))
    } else if let Some(b) = declared("compositeWithTrainedAlgorithmicMedia") {
        ("edited", how(b.source))
    } else {
        let by = blocks.iter().find_map(generator_settings)?;
        return Some(json!({ "made": "generated", "by": by, "how": "generator settings" }));
    };
    let mut ai = json!({ "made": made, "how": how });
    if let Some(by) = maker(blocks) {
        ai["by"] = json!(by);
    }
    Some(ai)
}

const fn how(source: Source) -> &'static str {
    match source {
        Source::C2pa => "C2PA Content Credentials",
        _ => "IPTC digital source type",
    }
}

/// A Stable Diffusion front end's own settings, kept in PNG text chunks.
fn generator_settings(b: &Block) -> Option<&'static str> {
    if b.source != Source::Text {
        return None;
    }
    match b.key.as_str() {
        "parameters" if b.text.contains("Steps:") || b.text.contains("Sampler") => Some("Stable Diffusion"),
        "parameters" | "fooocus_scheme" if b.text.to_ascii_lowercase().contains("fooocus") => Some("Fooocus"),
        "prompt" | "workflow" if b.text.contains("class_type") || b.text.contains("\"nodes\"") => Some("ComfyUI"),
        "invokeai_metadata" | "invokeai_graph" | "sd-metadata" | "Dream" => Some("InvokeAI"),
        "Software" if b.text.trim() == "NovelAI" => Some("NovelAI"),
        _ => None,
    }
}

/// Who made it, when a block names a generator (most specific first).
fn maker(blocks: &[Block]) -> Option<&'static str> {
    const NAMES: &[(&str, &str)] = &[
        ("midjourney", "Midjourney"),
        ("dall-e", "DALL·E"),
        ("dall·e", "DALL·E"),
        ("chatgpt", "ChatGPT"),
        ("openai", "OpenAI"),
        ("firefly", "Adobe Firefly"),
        ("imagen", "Google Imagen"),
        ("gemini", "Google Gemini"),
        ("bing image creator", "Bing Image Creator"),
        ("microsoft designer", "Microsoft Designer"),
        ("leonardo.ai", "Leonardo.Ai"),
        ("ideogram", "Ideogram"),
        ("black forest labs", "FLUX"),
        ("novelai", "NovelAI"),
        ("comfyui", "ComfyUI"),
        ("stable diffusion", "Stable Diffusion"),
        ("stability ai", "Stability AI"),
        ("meta ai", "Meta AI"),
        ("photoshop", "Adobe Photoshop"),
        ("google", "Google"),
    ];
    let lower: Vec<String> = blocks.iter().map(|b| b.text.to_lowercase()).collect();
    NAMES.iter().find(|(needle, _)| lower.iter().any(|t| t.contains(needle))).map(|(_, name)| *name)
}

fn blocks(path: &Path) -> std::io::Result<Vec<Block>> {
    let mut f = File::open(path)?;
    let mut magic = [0u8; 12];
    let n = f.read(&mut magic)?;
    f.seek(SeekFrom::Start(0))?;
    let magic = &magic[..n];
    if magic.starts_with(b"\x89PNG\r\n\x1a\n") {
        png(&mut f)
    } else if magic.starts_with(&[0xFF, 0xD8]) {
        jpeg(&mut f)
    } else if magic.len() == 12 && &magic[..4] == b"RIFF" && &magic[8..] == b"WEBP" {
        webp(&mut f)
    } else {
        let mut head = Vec::new();
        f.take(MAX_HEAD).read_to_end(&mut head)?;
        let c2pa = head.windows(4).any(|w| w == b"jumb") && head.windows(4).any(|w| w == b"c2pa");
        let text = String::from_utf8_lossy(&head).into_owned();
        Ok(vec![Block { source: if c2pa { Source::C2pa } else { Source::Raw }, key: String::new(), text }])
    }
}

fn read_block(f: &mut File, len: u64) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    f.take(len.min(MAX_BLOCK)).read_to_end(&mut buf)?;
    if len > MAX_BLOCK {
        f.seek(SeekFrom::Current(i64::try_from(len - MAX_BLOCK).unwrap_or(i64::MAX)))?;
    }
    Ok(buf)
}

fn inflate(data: &[u8]) -> Option<String> {
    let mut out = Vec::new();
    ZlibDecoder::new(data).take(MAX_BLOCK).read_to_end(&mut out).ok()?;
    Some(String::from_utf8_lossy(&out).into_owned())
}

fn png(f: &mut File) -> std::io::Result<Vec<Block>> {
    f.seek(SeekFrom::Start(8))?;
    let mut out = Vec::new();
    loop {
        let mut head = [0u8; 8];
        if f.read_exact(&mut head).is_err() {
            break;
        }
        let len = u64::from(u32::from_be_bytes([head[0], head[1], head[2], head[3]]));
        let kind = [head[4], head[5], head[6], head[7]];
        let wanted = matches!(&kind, b"tEXt" | b"zTXt" | b"iTXt" | b"caBX" | b"eXIf");
        if !wanted {
            if &kind == b"IEND" {
                break;
            }
            f.seek(SeekFrom::Current(i64::try_from(len + 4).unwrap_or(i64::MAX)))?;
            continue;
        }
        let data = read_block(f, len)?;
        f.seek(SeekFrom::Current(4))?;
        let text_block = |key: &[u8], text: String| Block {
            source: if key == b"XML:com.adobe.xmp" { Source::Xmp } else { Source::Text },
            key: String::from_utf8_lossy(key).into_owned(),
            text,
        };
        match &kind {
            b"caBX" => {
                out.push(Block { source: Source::C2pa, key: String::new(), text: lossy(&data) });
            }
            b"eXIf" => out.push(Block { source: Source::Exif, key: String::new(), text: lossy(&data) }),
            b"tEXt" => {
                if let Some((key, text)) = split0(&data) {
                    out.push(text_block(key, lossy(text)));
                }
            }
            b"zTXt" => {
                if let Some((key, rest)) = split0(&data)
                    && let Some(text) = rest.get(1..).and_then(inflate)
                {
                    out.push(text_block(key, text));
                }
            }
            _ => {
                // iTXt: key, compressed flag, method, language, translated key, text.
                let Some((key, rest)) = split0(&data) else { continue };
                let (Some(&compressed), Some(rest)) = (rest.first(), rest.get(2..)) else { continue };
                let Some((_, rest)) = split0(rest) else { continue };
                let Some((_, text)) = split0(rest) else { continue };
                let text = if compressed == 1 { inflate(text) } else { Some(lossy(text)) };
                if let Some(text) = text {
                    out.push(text_block(key, text));
                }
            }
        }
    }
    Ok(out)
}

fn jpeg(f: &mut File) -> std::io::Result<Vec<Block>> {
    const XMP: &[u8] = b"http://ns.adobe.com/xap/1.0/\0";
    const XMP_EXT: &[u8] = b"http://ns.adobe.com/xmp/extension/\0";
    f.seek(SeekFrom::Start(2))?;
    let mut out = Vec::new();
    // C2PA's JUMBF box may be split over many APP11 segments: kept as one.
    let mut c2pa = Vec::new();
    loop {
        let mut m = [0u8; 2];
        if f.read_exact(&mut m).is_err() || m[0] != 0xFF {
            break;
        }
        let marker = m[1];
        // Fill bytes, and markers without a length.
        if marker == 0xFF || marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            if marker == 0xFF {
                f.seek(SeekFrom::Current(-1))?;
            }
            continue;
        }
        // Start of scan or end of image: the metadata comes before.
        if marker == 0xDA || marker == 0xD9 {
            break;
        }
        let mut l = [0u8; 2];
        f.read_exact(&mut l)?;
        let len = u64::from(u16::from_be_bytes(l)).saturating_sub(2);
        match marker {
            0xE1 => {
                let data = read_block(f, len)?;
                if data.starts_with(b"Exif\0") {
                    out.push(Block { source: Source::Exif, key: String::new(), text: lossy(&data) });
                } else if let Some(x) = data.strip_prefix(XMP).or_else(|| data.strip_prefix(XMP_EXT)) {
                    out.push(Block { source: Source::Xmp, key: String::new(), text: lossy(x) });
                }
            }
            0xEB => c2pa.extend(read_block(f, len)?),
            _ => {
                f.seek(SeekFrom::Current(i64::try_from(len).unwrap_or(i64::MAX)))?;
            }
        }
    }
    if c2pa.windows(4).any(|w| w == b"c2pa") {
        out.push(Block { source: Source::C2pa, key: String::new(), text: lossy(&c2pa) });
    }
    Ok(out)
}

fn webp(f: &mut File) -> std::io::Result<Vec<Block>> {
    f.seek(SeekFrom::Start(12))?;
    let mut out = Vec::new();
    loop {
        let mut head = [0u8; 8];
        if f.read_exact(&mut head).is_err() {
            break;
        }
        let len = u64::from(u32::from_le_bytes([head[4], head[5], head[6], head[7]]));
        let padded = len + (len & 1);
        let source = match &head[..4] {
            b"XMP " => Some(Source::Xmp),
            b"EXIF" => Some(Source::Exif),
            b"C2PA" => Some(Source::C2pa),
            _ => None,
        };
        match source {
            Some(source) => {
                let data = read_block(f, len)?;
                out.push(Block { source, key: String::new(), text: lossy(&data) });
                f.seek(SeekFrom::Current(i64::try_from(padded - len).unwrap_or(0)))?;
            }
            None => {
                f.seek(SeekFrom::Current(i64::try_from(padded).unwrap_or(i64::MAX)))?;
            }
        }
    }
    Ok(out)
}

fn split0(data: &[u8]) -> Option<(&[u8], &[u8])> {
    let at = data.iter().position(|&b| b == 0)?;
    Some((&data[..at], &data[at + 1..]))
}

fn lossy(data: &[u8]) -> String {
    String::from_utf8_lossy(data).into_owned()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::io::Write as _;

    fn chunk(kind: [u8; 4], data: &[u8]) -> Vec<u8> {
        let mut c = (u32::try_from(data.len()).unwrap()).to_be_bytes().to_vec();
        c.extend_from_slice(&kind);
        c.extend_from_slice(data);
        c.extend_from_slice(&[0, 0, 0, 0]); // CRC, not checked here
        c
    }

    /// A 1x1 PNG with these extra chunks before its image data.
    pub(crate) fn png_with(extra: &[(&[u8; 4], Vec<u8>)]) -> Vec<u8> {
        let mut p = b"\x89PNG\r\n\x1a\n".to_vec();
        p.extend(chunk(*b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0]));
        for (k, d) in extra {
            p.extend(chunk(**k, d));
        }
        p.extend(chunk(*b"IDAT", &[0x78, 0x9C, 0x63, 0x60, 0x60, 0x60, 0, 0, 0, 4, 0, 1]));
        p.extend(chunk(*b"IEND", &[]));
        p
    }

    fn scan_bytes(bytes: &[u8], ext: &str) -> Map<String, Value> {
        let mut f = tempfile::Builder::new().suffix(&format!(".{ext}")).tempfile().unwrap();
        f.write_all(bytes).unwrap();
        scan(f.path())
    }

    #[test]
    fn stable_diffusion_settings_in_png_text() {
        let text = b"parameters\0a castle on a hill\nSteps: 20, Sampler: Euler a, CFG scale: 7".to_vec();
        let facts = scan_bytes(&png_with(&[(b"tEXt", text)]), "png");
        assert_eq!(facts["ai"], json!({ "made": "generated", "by": "Stable Diffusion", "how": "generator settings" }));
    }

    #[test]
    fn comfyui_workflow_compressed() {
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        z.write_all(br#"{"3": {"class_type": "KSampler", "inputs": {}}}"#).unwrap();
        let mut data = b"prompt\0\0".to_vec();
        data.extend(z.finish().unwrap());
        let facts = scan_bytes(&png_with(&[(b"zTXt", data)]), "png");
        assert_eq!(facts["ai"]["by"], "ComfyUI");
    }

    #[test]
    fn iptc_source_type_in_jpeg_xmp() {
        let xmp = br#"<x:xmpmeta><rdf:Description Iptc4xmpExt:DigitalSourceType="http://cv.iptc.org/newscodes/digitalsourcetype/trainedAlgorithmicMedia" xmp:CreatorTool="Google"/></x:xmpmeta>"#;
        let mut app1 = b"http://ns.adobe.com/xap/1.0/\0".to_vec();
        app1.extend_from_slice(xmp);
        let mut j = vec![0xFF, 0xD8, 0xFF, 0xE1];
        j.extend_from_slice(&u16::try_from(app1.len() + 2).unwrap().to_be_bytes());
        j.extend(app1);
        j.extend_from_slice(&[0xFF, 0xDA, 0, 2, 0xFF, 0xD9]);
        let facts = scan_bytes(&j, "jpg");
        assert_eq!(facts["ai"], json!({ "made": "generated", "by": "Google", "how": "IPTC digital source type" }));
        assert!(!facts.contains_key("content_credentials"));
    }

    #[test]
    fn c2pa_edit_in_webp() {
        let manifest = b"jumb....c2pa....c2pa.actions..digitalSourceType.http://cv.iptc.org/newscodes/digitalsourcetype/compositeWithTrainedAlgorithmicMedia..claim_generator.Adobe_Photoshop/26.0";
        let mut w = b"RIFF\0\0\0\0WEBP".to_vec();
        w.extend_from_slice(b"C2PA");
        w.extend_from_slice(&u32::try_from(manifest.len()).unwrap().to_le_bytes());
        w.extend_from_slice(manifest);
        if manifest.len() % 2 == 1 {
            w.push(0);
        }
        let facts = scan_bytes(&w, "webp");
        assert_eq!(facts["content_credentials"], true);
        assert_eq!(
            facts["ai"],
            json!({ "made": "edited", "by": "Adobe Photoshop", "how": "C2PA Content Credentials" })
        );
    }

    #[test]
    fn plain_images_say_nothing() {
        let text = b"Software\0GIMP 2.10".to_vec();
        assert!(scan_bytes(&png_with(&[(b"tEXt", text)]), "png").is_empty());
        assert!(scan_bytes(b"GIF89a....", "gif").is_empty());
    }
}
