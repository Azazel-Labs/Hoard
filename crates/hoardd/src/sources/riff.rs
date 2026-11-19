//! What sound libraries embed in WAV files: BWF bext, LIST/INFO, iXML and the sampler chunk.

use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

use anyhow::{Result, ensure};

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
