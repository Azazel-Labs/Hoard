//! FMOD's FSB5 sound banks, on their own (`.fsb`) or inside an FMOD Studio bank (`.bank`). PCM, FMOD's ADPCM, MPEG
//! and Vorbis play; the console codecs don't.
//!
//! FMOD's Vorbis leaves out the setup packet that holds the codebooks, naming it by its CRC-32 instead. The setup
//! packets FMOD uses are in `fsb_vorbis_setups.xz`, converted from vgmstream's table (ISC license), which python-fsb5
//! extracted from FMOD.

use std::sync::OnceLock;

use anyhow::{Context, Result, bail, ensure};

use super::{Bank, Reader, Sound, frames_for, from_i16};

/// Sample rates by the index in a sound's header.
const RATES: [u32; 11] = [4_000, 8_000, 11_000, 11_025, 16_000, 22_050, 24_000, 32_000, 44_100, 48_000, 96_000];

/// One sound's header.
struct Entry {
    rate: u32,
    channels: usize,
    frames: usize,
    data: (usize, usize),
    /// Vorbis: the CRC-32 of the setup packet.
    setup: Option<u32>,
}

/// The FSB5 inside an FMOD Studio bank.
pub(super) fn studio_bank(data: &[u8]) -> Result<Bank<'_>> {
    ensure!(data.starts_with(b"RIFF") && data.get(8..12) == Some(b"FEV "), "not an FMOD Studio bank");
    let snd = find(data, b"SND ", 12).context("the bank holds no sounds (a strings or metadata bank)")?;
    let at = find(data, b"FSB5", snd).context("the bank's sounds are encrypted")?;
    fsb(&data[at..], "FMOD Studio bank")
}

fn find(data: &[u8], what: &[u8], from: usize) -> Option<usize> {
    data.get(from..)?.windows(what.len()).position(|w| w == what).map(|i| from + i)
}

pub(super) fn fsb<'a>(data: &'a [u8], container: &'static str) -> Result<Bank<'a>> {
    ensure!(data.starts_with(b"FSB5"), "not an FSB5 sound bank");
    let r = Reader::le(data);
    let version = r.u32(4)?;
    ensure!(version <= 1, "FSB5 version {version} isn't known");
    let count = r.u32(8)? as usize;
    let (headers, names, size) = (r.u32(12)? as usize, r.u32(16)? as usize, r.u32(20)? as usize);
    let codec = r.u32(24)?;
    let big = version == 1 && r.u32(0x20)? & 1 != 0;
    let base = if version == 1 { 0x3C } else { 0x40 };
    let names_at = base + headers;
    let data_at = names_at + names;
    ensure!(count > 0 && count <= headers / 8 && data_at <= data.len(), "the FSB5's header doesn't fit");
    let size = size.min(data.len() - data_at);

    let mut entries: Vec<Entry> = Vec::with_capacity(count);
    let mut at = base;
    for _ in 0..count {
        let mode = r.u64(at)?;
        at += 8;
        let mut e = Entry {
            rate: RATES.get((mode >> 1 & 0xF) as usize).copied().unwrap_or(0),
            channels: [1, 2, 6, 8][(mode >> 5 & 3) as usize],
            frames: (mode >> 34) as usize,
            data: (((mode >> 7 & 0x07FF_FFFF) << 5) as usize, 0),
            setup: None,
        };
        let mut chained = mode & 1 != 0;
        while chained && at < names_at {
            let chunk = r.u32(at)?;
            let (kind, len) = (chunk >> 25, (chunk >> 1 & 0x00FF_FFFF) as usize);
            chained = chunk & 1 != 0;
            match kind {
                1 => e.channels = usize::from(r.bytes(at + 4, 1)?[0]),
                2 => e.rate = r.u32(at + 4)?,
                0x0B => e.setup = Some(r.u32(at + 4)?),
                // Vorbis in layers (multichannel, FMOD from about 2021).
                0x0E => e.channels *= r.u32(at + 4)? as usize,
                _ => {}
            }
            at += 4 + len;
        }
        entries.push(e);
    }
    for i in 0..entries.len() {
        let end = entries.get(i + 1).map_or(size, |n| n.data.0).min(size);
        entries[i].data = (data_at + entries[i].data.0, end.saturating_sub(entries[i].data.0));
    }
    let names = if names == 0 {
        Vec::new()
    } else {
        (0..count)
            .map(|i| {
                let at = names_at + r.u32(names_at + 4 * i).unwrap_or(0) as usize;
                let text = data.get(at..names_at + names).unwrap_or_default();
                let text = &text[..text.iter().position(|&b| b == 0).unwrap_or(text.len())];
                crate::music::clip_text(&String::from_utf8_lossy(text), 80)
            })
            .collect()
    };
    let codec_name = match codec {
        1..=5 => "PCM",
        6 => "GameCube ADPCM",
        7 => "IMA ADPCM",
        8 => "VAG",
        9 => "HEVAG",
        10 => "XMA",
        11 => "MPEG",
        12 => "CELT",
        13 => "ATRAC9",
        14 => "xWMA",
        15 => "Vorbis",
        16 => "FADPCM",
        17 => "Opus",
        _ => "unknown",
    };
    let decode = move |i: usize, secs: f32| -> Result<Sound> {
        let e = &entries[i];
        ensure!(e.channels > 0 && e.rate > 0, "the sound's header gives no channels or rate");
        let body = r.bytes(e.data.0, e.data.1)?;
        let limit = frames_for(secs, e.rate).min(e.frames.max(1)) * e.channels;
        let mut pcm = match codec {
            1 => body.iter().take(limit).map(|&b| (f32::from(b) - 128.0) / 128.0).collect(),
            2 => {
                let r = Reader { data: body, big };
                from_i16((0..body.len() / 2).take(limit).map(|k| r.u16(2 * k).unwrap_or(0) as i16))
            }
            3 => body
                .as_chunks::<3>()
                .0
                .iter()
                .take(limit)
                .map(|s| i32::from_le_bytes([0, s[0], s[1], s[2]]) as f32 / 2f32.powi(31))
                .collect(),
            4 => body
                .as_chunks::<4>()
                .0
                .iter()
                .take(limit)
                .map(|s| i32::from_le_bytes(*s) as f32 / 2f32.powi(31))
                .collect(),
            5 => body.as_chunks::<4>().0.iter().take(limit).map(|s| f32::from_le_bytes(*s)).collect(),
            11 => super::decode_stream(body.to_vec(), "mp3", secs)?.pcm,
            15 => vorbis(body, e, limit)?,
            16 => fadpcm(body, e.channels, limit),
            _ => bail!("FMOD's {codec_name} codec isn't played here"),
        };
        pcm.truncate(limit);
        Ok(Sound { pcm, channels: e.channels, rate: e.rate })
    };
    Ok(Bank { container, codec: codec_name.into(), names, count, decode: Box::new(decode) })
}

/// FMOD's Vorbis: the stream's packets, each after its 16-bit size, decoded with the setup packet it names.
fn vorbis(body: &[u8], e: &Entry, limit: usize) -> Result<Vec<f32>> {
    use lewton::audio::{PreviousWindowRight, read_audio_packet_generic};
    use lewton::samples::InterleavedSamples;

    let setup = e.setup.context("the Vorbis sound names no setup packet")?;
    let setup = setup_packet(setup).with_context(|| format!("FMOD's Vorbis setup {setup:08x} isn't known"))?;
    let channels = u8::try_from(e.channels)?;
    // The identification packet FMOD leaves out: blocks of 256 and 2048 samples.
    let mut ident = vec![1];
    ident.extend(b"vorbis");
    ident.extend(0u32.to_le_bytes());
    ident.push(channels);
    ident.extend(e.rate.to_le_bytes());
    ident.extend([0; 12]);
    ident.extend([(11 << 4) | 8, 1]);
    let ident = lewton::header::read_header_ident(&ident).map_err(|e| anyhow::anyhow!("{e:?}"))?;
    let setup = lewton::header::read_header_setup(setup, channels, (ident.blocksize_0, ident.blocksize_1))
        .map_err(|e| anyhow::anyhow!("FMOD's Vorbis setup doesn't read: {e:?}"))?;
    let mut window = PreviousWindowRight::new();
    let (mut out, mut at) = (Vec::new(), 0);
    while at + 2 <= body.len() && out.len() < limit {
        let len = usize::from(u16::from_le_bytes([body[at], body[at + 1]]));
        at += 2;
        if len == 0 || len == 0xFFFF || at + len > body.len() {
            break;
        }
        let pcm: InterleavedSamples<f32> = read_audio_packet_generic(&ident, &setup, &body[at..at + len], &mut window)
            .map_err(|e| anyhow::anyhow!("a Vorbis packet doesn't decode: {e:?}"))?;
        out.extend(pcm.samples);
        at += len;
    }
    Ok(out)
}

/// The setup packet with this CRC-32, from the table.
fn setup_packet(crc: u32) -> Option<&'static [u8]> {
    static TABLE: OnceLock<Vec<u8>> = OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut out = Vec::new();
        let packed: &[u8] = include_bytes!("fsb_vorbis_setups.xz");
        let read = std::io::Read::read_to_end(&mut lzma_rust2::XzReader::new(packed, true), &mut out);
        if let Err(e) = read {
            tracing::error!(error = %e, "the FMOD Vorbis setup table doesn't unpack");
            out.clear();
        }
        out
    });
    // A count, then each entry's CRC-32 and size (sorted by CRC-32), then the packets in the same order.
    let r = Reader::le(table);
    let count = r.u32(0).ok()? as usize;
    let mut at = 4 + 6 * count;
    for i in 0..count {
        let (id, len) = (r.u32(4 + 6 * i).ok()?, usize::from(r.u16(8 + 6 * i).ok()?));
        if id == crc {
            return r.bytes(at, len).ok();
        }
        at += len;
    }
    None
}

/// FMOD's ADPCM: frames of 256 samples per channel (each 0x8C bytes, the channels' frames in turn), with eight sets
/// of 32 samples sharing a predictor and a shift. From vgmstream's decoder.
fn fadpcm(body: &[u8], channels: usize, limit: usize) -> Vec<f32> {
    const COEFS: [[i32; 2]; 7] = [[0, 0], [60, 0], [122, 60], [115, 52], [98, 55], [0, 0], [0, 0]];
    const FRAME: usize = 0x8C;
    let mut out = Vec::new();
    for group in body.chunks_exact(FRAME * channels) {
        let mut frames = vec![[0i16; 256]; channels];
        for (c, frame) in group.as_chunks::<FRAME>().0.iter().enumerate() {
            let word = |at: usize| u32::from_le_bytes([frame[at], frame[at + 1], frame[at + 2], frame[at + 3]]);
            let (coefs, shifts) = (word(0), word(4));
            let mut hist1 = i32::from(i16::from_le_bytes([frame[8], frame[9]]));
            let mut hist2 = i32::from(i16::from_le_bytes([frame[10], frame[11]]));
            let mut n = 0;
            for set in 0..8 {
                let [c1, c2] = COEFS[((coefs >> (set * 4)) & 0xF) as usize % 7];
                let shift = 22 - ((shifts >> (set * 4)) & 0xF);
                for j in 0..4 {
                    let nibbles = word(0x0C + 0x10 * set as usize + 4 * j);
                    for k in 0..8 {
                        let raw = ((nibbles >> (k * 4)) & 0xF) as i32;
                        let sample = (((raw << 28) >> shift) - hist2 * c2 + hist1 * c1) >> 6;
                        let sample = sample.clamp(-32_768, 32_767);
                        frames[c][n] = sample as i16;
                        n += 1;
                        (hist2, hist1) = (hist1, sample);
                    }
                }
            }
        }
        out.extend(from_i16((0..256).flat_map(|i| frames.iter().map(move |f| f[i]))));
        if out.len() >= limit {
            break;
        }
    }
    out
}

#[cfg(test)]
pub(crate) mod tests {
    use super::super::tests::tone;
    use super::*;

    /// An FSB5 holding two named 16-bit mono sounds, a quarter second each at 44.1 kHz.
    pub(crate) fn tiny_fsb() -> Vec<u8> {
        let sounds = [tone(440.0, 0.25, 44_100), tone(660.0, 0.25, 44_100)];
        let mut headers = Vec::new();
        let mut body = Vec::new();
        for s in &sounds {
            let mode = ((s.len() as u64) << 34) | (((body.len() / 32) as u64) << 7) | (8 << 1);
            headers.extend(mode.to_le_bytes());
            body.extend(s.iter().flat_map(|v| v.to_le_bytes()));
            body.resize(body.len().next_multiple_of(32), 0);
        }
        let mut names = Vec::new();
        names.extend(8u32.to_le_bytes());
        names.extend(13u32.to_le_bytes());
        names.extend(b"beep\0boop\0");
        names.resize(names.len().next_multiple_of(16), 0);
        let mut fsb = b"FSB5".to_vec();
        for v in [1, 2, headers.len() as u32, names.len() as u32, body.len() as u32, 2, 0, 0] {
            fsb.extend(u32::to_le_bytes(v));
        }
        fsb.resize(0x3C, 0);
        fsb.extend(headers);
        fsb.extend(names);
        fsb.extend(body);
        fsb
    }

    /// An FMOD Studio bank around [`tiny_fsb`].
    pub(crate) fn tiny_fmod_bank() -> Vec<u8> {
        let mut snd = vec![0; 12];
        snd.extend(tiny_fsb());
        let mut chunks = Vec::new();
        chunks.extend(b"FMT ");
        chunks.extend(8u32.to_le_bytes());
        chunks.extend([0x92, 0, 0, 0, 0x92, 0, 0, 0]);
        chunks.extend(b"SND ");
        chunks.extend((snd.len() as u32).to_le_bytes());
        chunks.extend(snd);
        let mut bank = b"RIFF".to_vec();
        bank.extend((chunks.len() as u32 + 4).to_le_bytes());
        bank.extend(b"FEV ");
        bank.extend(chunks);
        bank
    }

    #[test]
    fn reads_a_studio_bank() {
        let r = super::super::render(&tiny_fmod_bank(), "bank", 30.0).unwrap();
        assert_eq!(r.facts["container"], "FMOD Studio bank");
        assert_eq!(r.facts["sounds"], 2);
        let mut strings = tiny_fmod_bank();
        strings.truncate(32);
        assert!(super::super::render(&strings, "bank", 1.0).is_err());
    }

    #[test]
    fn knows_fmods_vorbis_setups() {
        for crc in [0xC4C3_0A29, 0x3552_95CA] {
            let packet = setup_packet(crc).unwrap();
            assert!(packet.starts_with(b"\x05vorbis"));
        }
        assert!(setup_packet(0x1234_5678).is_none());
    }

    #[test]
    fn decodes_fadpcm() {
        // One frame: predictor 1 (60/64 of the last sample) everywhere, no shift, every nibble +7.
        let mut frame = vec![0u8; 0x8C];
        frame[0..4].copy_from_slice(&0x1111_1111u32.to_le_bytes());
        frame[0x0C..].fill(0x77);
        let pcm = fadpcm(&frame, 1, 10_000);
        assert_eq!(pcm.len(), 256);
        // Rises towards a steady level.
        assert!(pcm[0] > 0.0 && pcm[255] > pcm[0] && pcm[255] < 1.0);
    }

    /// A real FMOD Studio bank (Vorbis), named by `HOARD_FMOD_BANK`.
    #[test]
    #[ignore = "needs a game's FMOD bank"]
    fn plays_a_real_bank() {
        let data = std::fs::read(std::env::var("HOARD_FMOD_BANK").unwrap()).unwrap();
        let r = super::super::render(&data, "bank", 20.0).unwrap();
        eprintln!("{:?}", r.facts);
        assert_eq!(r.facts["codec"], "Vorbis");
        let peak = r.audio.samples.iter().map(|s| s.abs()).max().unwrap();
        assert!(peak > 1000, "peak {peak}");
    }
}
