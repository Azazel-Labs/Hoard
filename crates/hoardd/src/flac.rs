//! WAV files made FLAC for conversion rules: the same samples, bit for bit, in about half the space. The original is
//! deleted once its keeping time is up, so only a WAV a FLAC can hold whole is converted: plain integer PCM, with
//! nothing beside the samples but its INFO tags, which become the FLAC's. Anything else (floating-point samples,
//! loop points, cue markers, a broadcast header, a speaker layout FLAC doesn't assume) stays a WAV. Every FLAC made
//! is decoded again and compared with the WAV before it's used.
//!
//! symphonia reads both, to integers at their own depth, which is exact. What it doesn't say is what it passed over:
//! a chunk it doesn't know (loop points, markers) is skipped, as RIFF allows, and INFO text that isn't UTF-8 is
//! mangled. So the WAV's chunks are looked through here first, and its tags read from them.

use anyhow::{Context, Result, ensure};
use symphonia::core::codecs::audio::AudioCodecId;
use symphonia::core::codecs::audio::well_known::{CODEC_ID_PCM_S16LE, CODEC_ID_PCM_S24LE, CODEC_ID_PCM_U8};

use crate::audio::{Audio, write_flac};

/// A WAV file as a FLAC, or (`Err`) why it stays a WAV: what a FLAC of it would leave out.
pub(crate) fn from_wav(wav: &[u8]) -> Result<Result<Vec<u8>, String>> {
    let (data_len, tags) = match look_through(wav) {
        Ok(r) => r,
        Err(why) => return Ok(Err(why)),
    };
    let Ok(Read { audio, codec }) = read(wav, "wav") else { return Ok(Err("it can't be read".into())) };
    let width = match codec {
        c if c == CODEC_ID_PCM_U8 => 1,
        c if c == CODEC_ID_PCM_S16LE => 2,
        c if c == CODEC_ID_PCM_S24LE => 3,
        _ => return Ok(Err("its samples aren't 8, 16 or 24-bit PCM".into())),
    };
    if !(1..=8).contains(&audio.channels) {
        return Ok(Err(format!("it has {} channels", audio.channels)));
    }
    if audio.bits != width * 8 || audio.rate == 0 || audio.rate >= 1 << 20 {
        return Ok(Err("its format doesn't add up".into()));
    }
    // Anything past the last whole set of samples would be lost.
    if audio.samples.len() * usize::from(width) != data_len {
        return Ok(Err("its last samples are cut short".into()));
    }
    let mut flac = write_flac(&audio)?;
    if !tags.is_empty() {
        add_tags(&mut flac, &tags)?;
    }
    let back = read(&flac, "flac").context("reading the FLAC back")?;
    ensure!(back.audio.samples == audio.samples, "the FLAC made didn't decode to the WAV's samples");
    Ok(Ok(flac))
}

/// A WAV file's chunks: how long its samples are and its INFO tags as FLAC fields, or why a FLAC of it would leave
/// something out.
fn look_through(b: &[u8]) -> Result<(usize, Vec<(String, String)>), String> {
    let u16_at = |i: usize| b.get(i..i + 2).map(|s| u16::from_le_bytes([s[0], s[1]]));
    let u32_at = |i: usize| b.get(i..i + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]));
    if b.len() < 12 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return Err("not a RIFF WAVE file".into());
    }
    let mut data = None;
    let mut tags = Vec::new();
    let mut at = 12;
    while at + 8 <= b.len() {
        let id = &b[at..at + 4];
        let size = u32_at(at + 4).map_or(0, |n| n as usize);
        let body = at + 8;
        let end = body.checked_add(size).filter(|&e| e <= b.len());
        let Some(end) = end else {
            // The last chunk running past the end of the file: a writer that never went back to fill in the size
            // of its samples. Anything else cut short is damaged.
            if id == b"data" && data.is_none() {
                data = Some(b.len() - body);
                break;
            }
            return Err(format!("its {} chunk runs past the end of the file", String::from_utf8_lossy(id).trim()));
        };
        // The format's tag, an extensible format's own one included.
        let tag = |at: usize| if u16_at(at) == Some(0xFFFE) { u16_at(at + 24) } else { u16_at(at) };
        match id {
            b"fmt " if tag(body) == Some(3) => return Err("its samples are floating-point".into()),
            // An extensible format can say its samples are padded or its speakers laid out otherwise, which a FLAC
            // can't.
            b"fmt " if u16_at(body) == Some(0xFFFE) => {
                let (bits, valid, mask) = (u16_at(body + 14), u16_at(body + 18), u32_at(body + 20));
                if valid.is_some_and(|v| Some(v) != bits) {
                    return Err("its samples are padded".into());
                }
                let channels = u16_at(body + 2).unwrap_or(0);
                if mask.is_some_and(|m| m != 0 && Some(m) != default_layout(channels)) {
                    return Err("its speakers are laid out in a way a FLAC doesn't say".into());
                }
            }
            b"fmt " | b"fact" | b"JUNK" | b"junk" | b"PAD " | b"FLLR" => {}
            b"data" if data.is_none() => data = Some(size),
            b"LIST" if b.get(body..body + 4) == Some(b"INFO") => tags.extend(info_tags(&b[body + 4..end])),
            other => return Err(format!("it has a {} chunk", String::from_utf8_lossy(other).trim())),
        }
        at = end + (size & 1);
    }
    Ok((data.ok_or("it has no samples")?, tags))
}

/// The speakers FLAC assumes for a channel count (its spec's, as WAVE channel masks).
fn default_layout(channels: u16) -> Option<u32> {
    Some(match channels {
        1 => 0x4,
        2 => 0x3,
        3 => 0x7,
        4 => 0x33,
        5 => 0x37,
        6 => 0x3F,
        7 => 0x70F,
        8 => 0x63F,
        _ => return None,
    })
}

/// A LIST INFO chunk's tags, named as FLAC (Vorbis comment) fields.
fn info_tags(mut b: &[u8]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    while b.len() >= 8 {
        let id = &b[..4];
        let size = u32::from_le_bytes([b[4], b[5], b[6], b[7]]) as usize;
        let Some(raw) = b.get(8..8 + size) else { break };
        let raw = raw.split(|&c| c == 0).next().unwrap_or_default();
        // UTF-8 when it reads as that; Windows wrote its own code page, Latin-1 near enough.
        let text =
            std::str::from_utf8(raw).map_or_else(|_| raw.iter().map(|&c| char::from(c)).collect(), str::to_owned);
        let name = match id {
            b"INAM" => "TITLE".to_owned(),
            b"IART" => "ARTIST".to_owned(),
            b"IPRD" => "ALBUM".to_owned(),
            b"ICMT" => "COMMENT".to_owned(),
            b"ICRD" => "DATE".to_owned(),
            b"IGNR" => "GENRE".to_owned(),
            b"ICOP" => "COPYRIGHT".to_owned(),
            b"ISFT" => "ENCODER".to_owned(),
            b"ITRK" | b"IPRT" => "TRACKNUMBER".to_owned(),
            b"IENG" => "ENGINEER".to_owned(),
            b"IKEY" => "KEYWORDS".to_owned(),
            b"ISBJ" => "SUBJECT".to_owned(),
            b"ISRC" => "SOURCE".to_owned(),
            // Field names are printable ASCII without `=`.
            other => {
                other.iter().filter(|c| (0x20..0x7E).contains(*c) && **c != b'=').map(|&c| char::from(c)).collect()
            }
        };
        if !name.is_empty() && !text.trim().is_empty() {
            out.push((name, text.trim().to_owned()));
        }
        b = &b[(8 + size + (size & 1)).min(b.len())..];
    }
    out
}

/// Put `tags` in a FLAC file as its Vorbis comment block, after the blocks it has.
fn add_tags(flac: &mut Vec<u8>, tags: &[(String, String)]) -> Result<()> {
    ensure!(flac.starts_with(b"fLaC"), "not a FLAC stream");
    let mut body = Vec::new();
    let vendor = concat!("Hoard ", env!("CARGO_PKG_VERSION"));
    body.extend_from_slice(&u32::try_from(vendor.len())?.to_le_bytes());
    body.extend_from_slice(vendor.as_bytes());
    body.extend_from_slice(&u32::try_from(tags.len())?.to_le_bytes());
    for (k, v) in tags {
        let field = format!("{k}={v}");
        body.extend_from_slice(&u32::try_from(field.len())?.to_le_bytes());
        body.extend_from_slice(field.as_bytes());
    }
    ensure!(body.len() < 1 << 24, "too many tags");
    // Past the blocks there are: the last one says it's last, which the new one says instead.
    let mut at = 4;
    loop {
        let head = *flac.get(at).context("a FLAC stream cut short")?;
        let len = flac.get(at + 1..at + 4).context("a FLAC stream cut short")?;
        let len = (usize::from(len[0]) << 16) | (usize::from(len[1]) << 8) | usize::from(len[2]);
        if head & 0x80 != 0 {
            flac[at] = head & 0x7F;
            at += 4 + len;
            break;
        }
        at += 4 + len;
    }
    let n = u32::try_from(body.len())?.to_be_bytes();
    let mut block = vec![0x80 | 4, n[1], n[2], n[3]];
    block.extend_from_slice(&body);
    flac.splice(at..at, block);
    Ok(())
}

/// What symphonia reads of a WAV or FLAC file.
struct Read {
    /// Its samples, at their own bit depth, interleaved.
    audio: Audio,
    codec: AudioCodecId,
}

fn read(bytes: &[u8], ext: &str) -> Result<Read> {
    use symphonia::core::codecs::audio::AudioDecoderOptions;
    use symphonia::core::formats::probe::Hint;
    use symphonia::core::formats::{FormatOptions, TrackType};
    use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
    use symphonia::core::meta::MetadataOptions;

    let mss =
        MediaSourceStream::new(Box::new(std::io::Cursor::new(bytes.to_vec())), MediaSourceStreamOptions::default());
    let mut hint = Hint::new();
    hint.with_extension(ext);
    let mut format =
        symphonia::default::get_probe().probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())?;
    let track = format.default_track(TrackType::Audio).context("no audio track")?;
    let params = track.codec_params.as_ref().and_then(|p| p.audio()).context("no audio codec parameters")?;
    let bits = params.bits_per_sample.context("no bit depth")?;
    ensure!((4..=32).contains(&bits), "{bits}-bit samples");
    let channels = u16::try_from(params.channels.as_ref().context("no channels")?.count())?;
    let rate = params.sample_rate.context("no sample rate")?;
    let codec = params.codec;
    let mut decoder = symphonia::default::get_codecs().make_audio_decoder(params, &AudioDecoderOptions::default())?;
    let track_id = track.id;
    let (mut samples, mut scratch) = (Vec::new(), Vec::<i32>::new());
    loop {
        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            Err(symphonia::core::errors::Error::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        };
        if packet.track_id != track_id {
            continue;
        }
        let buf = decoder.decode(&packet)?;
        scratch.resize(buf.samples_interleaved(), 0);
        buf.copy_to_slice_interleaved(&mut scratch);
        // Handed over at full scale (32 bits).
        samples.extend(scratch.iter().map(|s| s >> (32 - bits)));
    }
    Ok(Read { audio: Audio { samples, channels, rate, bits: u16::try_from(bits)? }, codec })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A WAV file of `samples` at `bits`, with `extra` chunks after its samples.
    fn wav(samples: &[i32], channels: u16, bits: u16, extra: &[u8]) -> Vec<u8> {
        let width = usize::from(bits / 8);
        let mut data = Vec::new();
        for &s in samples {
            match bits {
                8 => data.push(u8::try_from(s + 128).unwrap()),
                _ => data.extend_from_slice(&s.to_le_bytes()[..width]),
            }
        }
        let align = channels * bits / 8;
        let mut fmt = Vec::new();
        fmt.extend_from_slice(&1u16.to_le_bytes());
        fmt.extend_from_slice(&channels.to_le_bytes());
        fmt.extend_from_slice(&44_100u32.to_le_bytes());
        fmt.extend_from_slice(&(44_100 * u32::from(align)).to_le_bytes());
        fmt.extend_from_slice(&align.to_le_bytes());
        fmt.extend_from_slice(&bits.to_le_bytes());
        let mut body = b"WAVE".to_vec();
        for (id, c) in [(&b"fmt "[..], &fmt[..]), (b"data", &data[..])] {
            body.extend_from_slice(id);
            body.extend_from_slice(&u32::try_from(c.len()).unwrap().to_le_bytes());
            body.extend_from_slice(c);
            if c.len() % 2 == 1 {
                body.push(0);
            }
        }
        body.extend_from_slice(extra);
        let mut out = b"RIFF".to_vec();
        out.extend_from_slice(&u32::try_from(body.len()).unwrap().to_le_bytes());
        out.extend(body);
        out
    }

    /// A sweep across the whole range of `bits`, extremes included.
    fn sweep(bits: u16, n: usize) -> Vec<i32> {
        let (lo, hi) = (-(1i64 << (bits - 1)), (1i64 << (bits - 1)) - 1);
        (0..n)
            .map(|i| i32::try_from(lo + (hi - lo) * i64::try_from(i).unwrap() / i64::try_from(n - 1).unwrap()).unwrap())
            .collect()
    }

    fn samples_of(flac: &[u8]) -> Vec<i32> {
        read(flac, "flac").unwrap().audio.samples
    }

    #[test]
    fn plain_pcm_becomes_a_flac_of_the_same_samples() {
        for (bits, channels) in [(16, 2), (24, 1), (8, 1), (16, 6)] {
            let samples = sweep(bits, 9_000 * usize::from(channels));
            let flac = from_wav(&wav(&samples, channels, bits, &[])).unwrap().unwrap();
            assert!(flac.starts_with(b"fLaC"));
            assert_eq!(samples_of(&flac), samples, "{bits}-bit, {channels} channels");
        }
    }

    #[test]
    fn info_tags_come_along() {
        let mut info = b"INFO".to_vec();
        for (id, v) in [(b"INAM", &b"Door slam\0"[..]), (b"IART", b"Caf\xe9\0")] {
            info.extend_from_slice(id);
            info.extend_from_slice(&u32::try_from(v.len()).unwrap().to_le_bytes());
            info.extend_from_slice(v);
            if v.len() % 2 == 1 {
                info.push(0);
            }
        }
        let mut list = b"LIST".to_vec();
        list.extend_from_slice(&u32::try_from(info.len()).unwrap().to_le_bytes());
        list.extend(info);
        let samples = sweep(16, 2_000);
        let flac = from_wav(&wav(&samples, 1, 16, &list)).unwrap().unwrap();
        let text = String::from_utf8_lossy(&flac);
        assert!(text.contains("TITLE=Door slam") && text.contains("ARTIST=Café"));
        assert_eq!(samples_of(&flac), samples);
    }

    #[test]
    fn what_a_flac_would_lose_stays_a_wav() {
        let samples = sweep(16, 100);
        // Loop points.
        let smpl = [&b"smpl"[..], &4u32.to_le_bytes(), &[0; 4]].concat();
        assert!(from_wav(&wav(&samples, 1, 16, &smpl)).unwrap().unwrap_err().contains("smpl"));
        // Floating-point samples.
        let floats: Vec<i32> = (0..100).map(|i| i32::from_le_bytes((i as f32 / 100.0).to_le_bytes())).collect();
        let mut float = wav(&floats, 1, 32, &[]);
        float[20] = 3;
        assert_eq!(from_wav(&float).unwrap().unwrap_err(), "its samples are floating-point");
        // 32-bit integers.
        assert!(from_wav(&wav(&sweep(16, 100), 1, 32, &[])).unwrap().is_err());
        // Half a sample left over at the end.
        let mut odd = wav(&samples, 1, 16, &[]);
        odd.push(7);
        let n = u32::from_le_bytes(odd[40..44].try_into().unwrap()) + 1;
        odd[40..44].copy_from_slice(&n.to_le_bytes());
        assert!(from_wav(&odd).unwrap().is_err());
        assert!(from_wav(b"not a wav at all").unwrap().is_err());
    }
}
