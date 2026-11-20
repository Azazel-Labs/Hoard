//! WAV files made FLAC for conversion rules: the same samples, bit for bit, in about half the space, with everything
//! else in the WAV (loop points, cue markers, a broadcast header, its INFO tags) kept as the reference encoder's
//! `flac --keep-foreign-metadata` keeps it: each of its chunks, as it was, in an APPLICATION block named `riff`, in
//! the file's order, the samples' chunk by its header alone. So the WAV can be made again exactly, byte for byte
//! ([`to_wav`]), and `flac -d --keep-foreign-metadata` makes the same one. Its INFO tags are also the FLAC's own,
//! for players to show. Every FLAC made is turned back into a WAV and compared with the original before it's used.
//!
//! Only floating-point or 32-bit samples, which a FLAC doesn't hold, keep a WAV a WAV, along with anything that
//! wouldn't come back the same (a chunk too big for a FLAC block, bytes after the last chunk).
//!
//! symphonia reads both, to integers at their own depth, which is exact. It passes over chunks it doesn't know, as
//! RIFF allows, and mangles INFO text that isn't UTF-8, so the chunks are walked here, and the tags read from them.

use anyhow::{Context, Result, bail, ensure};
use symphonia::core::codecs::audio::AudioCodecId;
use symphonia::core::codecs::audio::well_known::{CODEC_ID_PCM_S16LE, CODEC_ID_PCM_S24LE, CODEC_ID_PCM_U8};

use crate::audio::{Audio, write_flac};

/// The APPLICATION block id the reference encoder keeps a WAV's chunks under.
const RIFF: &[u8; 4] = b"riff";
/// FLAC metadata block types.
const APPLICATION: u8 = 2;
const VORBIS_COMMENT: u8 = 4;
/// The most a FLAC metadata block holds.
const MAX_BLOCK: usize = (1 << 24) - 1;

/// A WAV file as a FLAC, or (`Err`) why it stays a WAV.
pub(crate) fn from_wav(wav: &[u8]) -> Result<Result<Vec<u8>, String>> {
    let parts = match look_through(wav) {
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
    if audio.samples.len() * usize::from(width) != parts.data {
        return Ok(Err("its last samples are cut short".into()));
    }
    let mut flac = write_flac(&audio)?;
    let mut blocks = Vec::new();
    let mut tags = parts.tags;
    // As the reference encoder says a speaker layout other than the one FLAC assumes.
    if let Some(mask) = parts.mask.filter(|&m| m != 0 && Some(m) != default_layout(audio.channels)) {
        tags.push(("WAVEFORMATEXTENSIBLE_CHANNEL_MASK".into(), format!("0x{mask:04X}")));
    }
    if !tags.is_empty() {
        blocks.push((VORBIS_COMMENT, comments(&tags)?));
    }
    for piece in parts.pieces {
        blocks.push((APPLICATION, [&RIFF[..], piece].concat()));
    }
    append_blocks(&mut flac, blocks)?;
    let back = to_wav(&flac).context("making the WAV again from the FLAC")?;
    ensure!(back.as_deref() == Some(wav), "the FLAC made didn't give back the same WAV");
    Ok(Ok(flac))
}

/// The WAV file a FLAC was made from, made again from the chunks it keeps (`None` when it keeps none) and its
/// samples.
pub(crate) fn to_wav(flac: &[u8]) -> Result<Option<Vec<u8>>> {
    let pieces: Vec<&[u8]> = blocks(flac)?
        .into_iter()
        .filter(|(t, b)| *t == APPLICATION && b.starts_with(RIFF))
        .map(|(_, b)| &b[4..])
        .collect();
    if pieces.is_empty() {
        return Ok(None);
    }
    ensure!(pieces[0].len() == 12 && pieces[0].starts_with(b"RIFF"), "its first kept chunk isn't a RIFF header");
    let is_data = |p: &[u8]| p.len() == 8 && p.starts_with(b"data");
    ensure!(pieces.iter().filter(|p| is_data(p)).count() == 1, "it doesn't keep one data chunk's header");
    let audio = read(flac, "flac")?.audio;
    let width = usize::from(audio.bits / 8);
    ensure!(matches!(width, 1..=3) && audio.bits % 8 == 0, "{}-bit samples", audio.bits);
    let mut out = Vec::with_capacity(audio.samples.len() * width + pieces.iter().map(|p| p.len()).sum::<usize>() + 1);
    for p in pieces {
        out.extend_from_slice(p);
        if !is_data(p) {
            continue;
        }
        let start = out.len();
        for &s in &audio.samples {
            match width {
                // 8-bit WAV samples are unsigned.
                1 => out.push(u8::try_from(s + 128).context("an 8-bit sample out of range")?),
                _ => out.extend_from_slice(&s.to_le_bytes()[..width]),
            }
        }
        // A chunk of odd size is padded to an even one, unless the file ended before it was all written.
        let size = u32::from_le_bytes([p[4], p[5], p[6], p[7]]) as usize;
        if size % 2 == 1 && out.len() - start == size {
            out.push(0);
        }
    }
    Ok(Some(out))
}

/// What a WAV file is besides its samples: the pieces a FLAC keeps (its RIFF header, then each chunk whole, the
/// samples' by its header alone), how many bytes of samples it has, its INFO tags as FLAC fields and its speaker
/// layout.
struct Parts<'a> {
    pieces: Vec<&'a [u8]>,
    data: usize,
    tags: Vec<(String, String)>,
    mask: Option<u32>,
}

/// A WAV file's [`Parts`], or why a FLAC of it wouldn't give it back.
fn look_through(b: &[u8]) -> Result<Parts<'_>, String> {
    let u16_at = |i: usize| b.get(i..i + 2).map(|s| u16::from_le_bytes([s[0], s[1]]));
    let u32_at = |i: usize| b.get(i..i + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]));
    if b.len() < 12 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return Err("not a RIFF WAVE file".into());
    }
    let mut parts = Parts { pieces: vec![&b[..12]], data: 0, tags: Vec::new(), mask: None };
    let mut seen_data = false;
    let mut at = 12;
    while at < b.len() {
        if at + 8 > b.len() {
            return Err("it has something after its last chunk".into());
        }
        let id = &b[at..at + 4];
        let name = || String::from_utf8_lossy(id).trim().to_owned();
        let size = u32_at(at + 4).map_or(0, |n| n as usize);
        let body = at + 8;
        let Some(end) = body.checked_add(size).filter(|&e| e <= b.len()) else {
            // The last chunk running past the end of the file: a writer that never went back to fill in the size
            // of its samples. Anything else cut short is damaged.
            if id == b"data" && !seen_data {
                parts.pieces.push(&b[at..body]);
                parts.data = b.len() - body;
                return Ok(parts);
            }
            return Err(format!("its {} chunk runs past the end of the file", name()));
        };
        // A chunk of odd size is followed by a byte to make it even.
        let next = (end + (size & 1)).min(b.len());
        // The format's tag, an extensible format's own one included.
        let tag = |at: usize| if u16_at(at) == Some(0xFFFE) { u16_at(at + 24) } else { u16_at(at) };
        match id {
            b"fmt " if tag(body) == Some(3) => return Err("its samples are floating-point".into()),
            b"fmt " if u16_at(body) == Some(0xFFFE) => parts.mask = u32_at(body + 20),
            b"data" if seen_data => return Err("it has two data chunks".into()),
            b"data" => {
                // The samples' padding byte is written again as a zero.
                if size % 2 == 1 && b.get(end) != Some(&0) {
                    return Err("its samples end oddly".into());
                }
                seen_data = true;
                parts.data = size;
                parts.pieces.push(&b[at..body]);
                at = next;
                continue;
            }
            b"LIST" if b.get(body..body + 4) == Some(b"INFO") => parts.tags.extend(info_tags(&b[body + 4..end])),
            _ => {}
        }
        if next - at + RIFF.len() > MAX_BLOCK {
            return Err(format!("its {} chunk is too big for a FLAC to keep", name()));
        }
        parts.pieces.push(&b[at..next]);
        at = next;
    }
    if !seen_data {
        return Err("it has no samples".into());
    }
    Ok(parts)
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

/// A Vorbis comment block's body of `tags`.
fn comments(tags: &[(String, String)]) -> Result<Vec<u8>> {
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
    Ok(body)
}

/// A FLAC file's metadata blocks, as (type, body).
fn blocks(flac: &[u8]) -> Result<Vec<(u8, &[u8])>> {
    ensure!(flac.starts_with(b"fLaC"), "not a FLAC stream");
    let mut out = Vec::new();
    let mut at = 4;
    loop {
        let head = flac.get(at..at + 4).context("a FLAC stream cut short")?;
        let len = (usize::from(head[1]) << 16) | (usize::from(head[2]) << 8) | usize::from(head[3]);
        let body = flac.get(at + 4..at + 4 + len).context("a FLAC stream cut short")?;
        out.push((head[0] & 0x7F, body));
        at += 4 + len;
        if head[0] & 0x80 != 0 {
            return Ok(out);
        }
    }
}

/// Put `new` blocks (type, body) in a FLAC file, after the blocks it has.
fn append_blocks(flac: &mut Vec<u8>, new: Vec<(u8, Vec<u8>)>) -> Result<()> {
    let Some(last) = new.len().checked_sub(1) else { return Ok(()) };
    let have = blocks(flac)?;
    let end = 4 + have.iter().map(|(_, b)| 4 + b.len()).sum::<usize>();
    // The last block says it's last, which the last new one says instead.
    let last_head = end - 4 - have.last().map_or(0, |(_, b)| b.len());
    flac[last_head] &= 0x7F;
    let mut bytes = Vec::new();
    for (i, (kind, body)) in new.into_iter().enumerate() {
        if body.len() > MAX_BLOCK {
            bail!("a metadata block too big for a FLAC");
        }
        let n = u32::try_from(body.len())?.to_be_bytes();
        bytes.extend_from_slice(&[if i == last { 0x80 | kind } else { kind }, n[1], n[2], n[3]]);
        bytes.extend_from_slice(&body);
    }
    flac.splice(end..end, bytes);
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

    /// One RIFF chunk, padded to an even length.
    fn chunk(id: [u8; 4], body: &[u8]) -> Vec<u8> {
        let mut out = id.to_vec();
        out.extend_from_slice(&u32::try_from(body.len()).unwrap().to_le_bytes());
        out.extend_from_slice(body);
        if body.len() % 2 == 1 {
            out.push(0);
        }
        out
    }

    fn riff(body: &[u8]) -> Vec<u8> {
        [&b"RIFF"[..], &u32::try_from(body.len()).unwrap().to_le_bytes(), body].concat()
    }

    /// A WAV file of `samples` at `bits`, with whole chunks `before` and `after` its samples.
    fn wav_with(samples: &[i32], channels: u16, bits: u16, before: &[u8], after: &[u8]) -> Vec<u8> {
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
        riff(&[&b"WAVE"[..], &chunk(*b"fmt ", &fmt), before, &chunk(*b"data", &data), after].concat())
    }

    fn wav(samples: &[i32], channels: u16, bits: u16, after: &[u8]) -> Vec<u8> {
        wav_with(samples, channels, bits, &[], after)
    }

    /// A sweep across the whole range of `bits`, extremes included.
    fn sweep(bits: u16, n: usize) -> Vec<i32> {
        let (lo, hi) = (-(1i64 << (bits - 1)), (1i64 << (bits - 1)) - 1);
        (0..n)
            .map(|i| i32::try_from(lo + (hi - lo) * i64::try_from(i).unwrap() / i64::try_from(n - 1).unwrap()).unwrap())
            .collect()
    }

    /// Converts, and checks the FLAC's samples and that the WAV comes back whole.
    fn round_trip(wav: &[u8], samples: &[i32]) -> Vec<u8> {
        let flac = from_wav(wav).unwrap().unwrap();
        assert!(flac.starts_with(b"fLaC"));
        assert_eq!(read(&flac, "flac").unwrap().audio.samples, samples);
        assert!(to_wav(&flac).unwrap().unwrap() == wav, "the WAV comes back byte for byte");
        flac
    }

    #[test]
    fn plain_pcm_becomes_a_flac_of_the_same_samples() {
        for (bits, channels) in [(16, 2), (24, 1), (8, 1), (16, 6)] {
            let samples = sweep(bits, 9_000 * usize::from(channels));
            round_trip(&wav(&samples, channels, bits, &[]), &samples);
        }
        // An odd number of bytes of samples, padded.
        let samples = sweep(8, 1_001);
        round_trip(&wav(&samples, 1, 8, &[]), &samples);
    }

    #[test]
    fn info_tags_come_along_as_the_flacs_own() {
        let mut info = b"INFO".to_vec();
        for (id, v) in [(b"INAM", &b"Door slam\0"[..]), (b"IART", b"Caf\xe9\0")] {
            info.extend(chunk(*id, v));
        }
        let samples = sweep(16, 2_000);
        let flac = round_trip(&wav(&samples, 1, 16, &chunk(*b"LIST", &info)), &samples);
        let text = String::from_utf8_lossy(&flac);
        assert!(text.contains("TITLE=Door slam") && text.contains("ARTIST=Caf\u{e9}"));
    }

    #[test]
    fn every_other_chunk_is_kept_as_the_reference_encoder_keeps_it() {
        let samples = sweep(24, 3_000);
        // Loop points and markers after the samples; a broadcast header and an odd-sized chunk before them.
        let (smpl, cue, odd) = (chunk(*b"smpl", &[1; 60]), chunk(*b"cue ", &[2; 28]), chunk(*b"odd ", &[4; 5]));
        let before = [chunk(*b"bext", &[3; 602]), odd.clone()].concat();
        let wav = wav_with(&samples, 1, 24, &before, &[smpl.clone(), cue.clone()].concat());
        let flac = round_trip(&wav, &samples);
        let kept: Vec<&[u8]> = blocks(&flac)
            .unwrap()
            .into_iter()
            .filter(|(t, b)| *t == APPLICATION && b.starts_with(b"riff"))
            .map(|(_, b)| &b[4..])
            .collect();
        // The RIFF header, the format, what comes before the samples, the samples' header, what comes after.
        assert_eq!(kept.len(), 7);
        assert_eq!(kept[0], &wav[..12]);
        assert!(kept[1].starts_with(b"fmt "));
        assert!(kept[2].starts_with(b"bext") && kept[3] == &odd[..]);
        assert!(kept[4].starts_with(b"data") && kept[4].len() == 8);
        assert_eq!((kept[5], kept[6]), (&smpl[..], &cue[..]));
    }

    #[test]
    fn a_speaker_layout_flac_doesnt_assume_is_said_too() {
        let samples = sweep(16, 4_000);
        let plain = wav(&samples, 2, 16, &[]);
        // Made extensible: its own tag PCM, the speakers side left and right.
        let mut fmt = plain[20..36].to_vec();
        fmt[0..2].copy_from_slice(&0xFFFEu16.to_le_bytes());
        fmt.extend_from_slice(&22u16.to_le_bytes());
        fmt.extend_from_slice(&16u16.to_le_bytes());
        fmt.extend_from_slice(&0x600u32.to_le_bytes());
        fmt.extend_from_slice(&[1, 0, 0, 0, 0, 0, 0x10, 0, 0x80, 0, 0, 0xAA, 0, 0x38, 0x9B, 0x71]);
        let extensible = riff(&[&b"WAVE"[..], &chunk(*b"fmt ", &fmt), &plain[36..]].concat());
        let flac = round_trip(&extensible, &samples);
        assert!(String::from_utf8_lossy(&flac).contains("WAVEFORMATEXTENSIBLE_CHANNEL_MASK=0x0600"));
    }

    #[test]
    fn a_flac_without_kept_chunks_has_no_wav_to_give() {
        let plain = write_flac(&Audio { samples: sweep(16, 500), channels: 1, rate: 44_100, bits: 16 }).unwrap();
        assert!(to_wav(&plain).unwrap().is_none());
    }

    #[test]
    fn what_a_flac_cant_hold_or_give_back_stays_a_wav() {
        let samples = sweep(16, 100);
        // Floating-point samples.
        let floats: Vec<i32> = (0..100).map(|i| i32::from_le_bytes((i as f32 / 100.0).to_le_bytes())).collect();
        let mut float = wav(&floats, 1, 32, &[]);
        float[20] = 3;
        assert_eq!(from_wav(&float).unwrap().unwrap_err(), "its samples are floating-point");
        // 32-bit integers.
        assert!(from_wav(&wav(&sweep(16, 100), 1, 32, &[])).unwrap().is_err());
        // Bytes after the last chunk.
        let mut tail = wav(&samples, 1, 16, &[]);
        tail.extend_from_slice(&[1, 2, 3]);
        assert_eq!(from_wav(&tail).unwrap().unwrap_err(), "it has something after its last chunk");
        // Half a sample left over at the end.
        let mut odd = wav(&samples, 1, 16, &[]);
        odd.push(7);
        let n = u32::from_le_bytes(odd[40..44].try_into().unwrap()) + 1;
        odd[40..44].copy_from_slice(&n.to_le_bytes());
        assert!(from_wav(&odd).unwrap().is_err());
        assert!(from_wav(b"not a wav at all").unwrap().is_err());
    }
}
