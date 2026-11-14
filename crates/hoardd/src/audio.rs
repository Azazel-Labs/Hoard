//! Audio in and out, all pure Rust: decoding with `symphonia`, and writing WAV, FLAC (`flacenc`), MP3 (`rusty_mp3`)
//! and Opus (`opus-pure`). MP3, OGG Vorbis and Opus downloads prefer ffmpeg's reference encoders when ffmpeg is
//! ready (see `ffmpeg`); the MP3 and Opus encoders here are the fallback.

use std::path::Path;

use anyhow::{Context, Result, ensure};

/// Decoded audio is held in memory; past this (about 17 minutes of 48 kHz stereo) we refuse.
const MAX_SAMPLES: usize = 100_000_000;
/// Half-width of the resampling filter, in input samples.
const TAPS: isize = 24;

/// Interleaved PCM at the source's bit depth (16 or 24).
pub(crate) struct Audio {
    pub(crate) samples: Vec<i32>,
    pub(crate) channels: u16,
    pub(crate) rate: u32,
    pub(crate) bits: u16,
}

pub(crate) fn decode_audio(path: &Path) -> Result<Audio> {
    let (mut samples, mut channels, mut rate, mut bits) = (Vec::new(), 0u16, 0u32, 16u16);
    decode_each(Box::new(std::fs::File::open(path)?), path.extension().and_then(|e| e.to_str()), |pcm, c, r, b| {
        (channels, rate, bits) = (c, r, b);
        let max = if b == 24 { 8_388_607.0 } else { 32_767.0 };
        ensure!(samples.len() + pcm.len() <= MAX_SAMPLES, "too long to convert here");
        samples.extend(pcm.iter().map(|s| (s.clamp(-1.0, 1.0) * max).round() as i32));
        Ok(true)
    })?;
    ensure!(channels > 0 && rate > 0, "no audio decoded");
    Ok(Audio { samples, channels, rate, bits })
}

/// Decode `source` (its file extension a hint), handing each packet's interleaved samples (about -1..=1) to `each`
/// with the channel count, the rate and the source's bit depth (16 or 24), until the stream ends or `each` says stop.
pub(crate) fn decode_each(
    source: Box<dyn symphonia::core::io::MediaSource>,
    ext: Option<&str>,
    mut each: impl FnMut(&[f32], u16, u32, u16) -> Result<bool>,
) -> Result<()> {
    use symphonia::core::codecs::audio::AudioDecoderOptions;
    use symphonia::core::formats::probe::Hint;
    use symphonia::core::formats::{FormatOptions, TrackType};
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;

    let mss = MediaSourceStream::new(source, symphonia::core::io::MediaSourceStreamOptions::default());
    let mut hint = Hint::new();
    if let Some(ext) = ext {
        hint.with_extension(ext);
    }
    let mut format =
        symphonia::default::get_probe().probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())?;
    let track = format.default_track(TrackType::Audio).context("no audio track")?;
    let params = track.codec_params.as_ref().and_then(|p| p.audio()).context("no audio codec parameters")?;
    let bits: u16 = match params.bits_per_sample {
        Some(b) if b > 16 => 24,
        _ => 16,
    };
    let mut decoder = symphonia::default::get_codecs().make_audio_decoder(params, &AudioDecoderOptions::default())?;
    let track_id = track.id;
    let mut scratch: Vec<f32> = Vec::new();
    loop {
        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            // Some readers (FLAC) report the end of the stream as an unexpected EOF.
            Err(symphonia::core::errors::Error::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        };
        if packet.track_id != track_id {
            continue;
        }
        let buf = match decoder.decode(&packet) {
            Ok(b) => b,
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(e.into()),
        };
        let rate = buf.spec().rate();
        let channels = u16::try_from(buf.spec().channels().count().max(1))?;
        scratch.resize(buf.samples_interleaved(), 0.0);
        buf.copy_to_slice_interleaved(&mut scratch);
        if !each(&scratch, channels, rate, bits)? {
            break;
        }
    }
    Ok(())
}

pub(crate) fn write_wav(a: &Audio) -> Vec<u8> {
    let bytes_per = usize::from(a.bits / 8);
    let data_len = a.samples.len() * bytes_per;
    let mut out = Vec::with_capacity(44 + data_len);
    let block = u32::from(a.channels) * u32::from(a.bits / 8);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&u32::try_from(36 + data_len).unwrap_or(u32::MAX).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&a.channels.to_le_bytes());
    out.extend_from_slice(&a.rate.to_le_bytes());
    out.extend_from_slice(&(a.rate * block).to_le_bytes());
    out.extend_from_slice(&u16::try_from(block).unwrap_or(u16::MAX).to_le_bytes());
    out.extend_from_slice(&a.bits.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&u32::try_from(data_len).unwrap_or(u32::MAX).to_le_bytes());
    for s in &a.samples {
        out.extend_from_slice(&s.to_le_bytes()[..bytes_per]);
    }
    out
}

pub(crate) fn write_flac(a: &Audio) -> Result<Vec<u8>> {
    use flacenc::component::BitRepr as _;
    use flacenc::error::Verify as _;
    let config = flacenc::config::Encoder::default().into_verified().map_err(|e| anyhow::anyhow!("{e:?}"))?;
    let source = flacenc::source::MemSource::from_samples(
        &a.samples,
        usize::from(a.channels),
        usize::from(a.bits),
        usize::try_from(a.rate)?,
    );
    let stream = flacenc::encode_with_fixed_block_size(&config, source, config.block_size)
        .map_err(|e| anyhow::anyhow!("FLAC encoding failed: {e:?}"))?;
    let mut sink = flacenc::bitsink::ByteSink::new();
    stream.write(&mut sink).map_err(|e| anyhow::anyhow!("FLAC encoding failed: {e:?}"))?;
    let mut out = sink.as_slice().to_vec();
    // STREAMINFO's minimum block size must exclude the final, shorter block (the spec says so, and strict decoders
    // reject the file otherwise); flacenc counts it. Fixed-size blocks: min = max. (First block type 0 = STREAMINFO.)
    if out.len() >= 12 && out.starts_with(b"fLaC") && out[4].trailing_zeros() >= 7 {
        out.copy_within(10..12, 8);
    }
    Ok(out)
}

impl Audio {
    /// Samples as floats in [-1, 1].
    fn floats(&self) -> Vec<f32> {
        let scale = if self.bits == 24 { 8_388_607.0 } else { 32_767.0 };
        self.samples.iter().map(|&s| s as f32 / scale).collect()
    }
}

/// At most two channels (MP3 and Opus): mono stays mono, more than two mix down to stereo.
fn stereo_or_mono(pcm: &[f32], channels: usize) -> (Vec<f32>, usize) {
    if channels <= 2 {
        return (pcm.to_vec(), channels);
    }
    let out = pcm
        .chunks_exact(channels)
        .flat_map(|f| {
            // Front left/right plus a share of everything else (center, surrounds, LFE).
            let rest: f32 = f[2..].iter().sum::<f32>() * 0.5 / (channels - 2) as f32;
            [(f[0] + rest) * 0.7, (f[1] + rest) * 0.7]
        })
        .collect();
    (out, 2)
}

/// Windowed-sinc resampling of interleaved audio, good enough for delivery formats.
fn resample(pcm: &[f32], channels: usize, from: u32, to: u32) -> Vec<f32> {
    if from == to {
        return pcm.to_vec();
    }
    let frames = pcm.len() / channels;
    let ratio = f64::from(to) / f64::from(from);
    let out_frames = (frames as f64 * ratio).round() as usize;
    // Below the lower Nyquist frequency, with a little headroom.
    let cutoff = ratio.min(1.0) * 0.94;
    let mut out = vec![0.0f32; out_frames * channels];
    for o in 0..out_frames {
        let center = o as f64 / ratio;
        let base = center.floor() as isize;
        let mut acc = vec![0.0f64; channels];
        let mut norm = 0.0;
        for k in (base - TAPS + 1)..=(base + TAPS) {
            if k < 0 || k as usize >= frames {
                continue;
            }
            let x = center - k as f64;
            let sinc = if x.abs() < 1e-9 {
                1.0
            } else {
                (std::f64::consts::PI * x * cutoff).sin() / (std::f64::consts::PI * x)
            };
            let w = 0.42
                + 0.5 * (std::f64::consts::PI * x / TAPS as f64).cos()
                + 0.08 * (2.0 * std::f64::consts::PI * x / TAPS as f64).cos();
            let h = sinc * w;
            norm += h;
            for (c, a) in acc.iter_mut().enumerate() {
                *a += h * f64::from(pcm[k as usize * channels + c]);
            }
        }
        for (c, a) in acc.iter().enumerate() {
            out[o * channels + c] = if norm.abs() > 1e-9 { (a / norm) as f32 } else { 0.0 };
        }
    }
    out
}

/// MP3, VBR around V2 (about 190 kbps for stereo music). Rates MP3 can't carry are resampled to 44.1 kHz.
pub(crate) fn write_mp3(a: &Audio) -> Result<Vec<u8>> {
    let (pcm, ch) = stereo_or_mono(&a.floats(), usize::from(a.channels));
    let (pcm, rate) = if [8_000, 11_025, 12_000, 16_000, 22_050, 24_000, 32_000, 44_100, 48_000].contains(&a.rate) {
        (pcm, a.rate)
    } else {
        (resample(&pcm, ch, a.rate, 44_100), 44_100)
    };
    let mut enc = rusty_mp3::Mp3Encoder::new(rusty_mp3::Mp3EncoderConfig {
        bitrate_kbps: 192,
        vbr_quality: Some(rusty_mp3::vbr_quality_index(2.0)),
    });
    enc.push_pcm_f32(&pcm, u16::try_from(ch)?, rate).map_err(|e| anyhow::anyhow!("MP3 encoding failed: {e:?}"))?;
    enc.finish();
    let mut out = Vec::new();
    while let Ok(p) = enc.next_packet() {
        out.extend(p);
    }
    ensure!(!out.is_empty(), "MP3 encoder produced nothing");
    Ok(out)
}

/// Opus in an Ogg container at 128 kbps, resampled to 48 kHz (Opus' native rate), mono or stereo.
pub(crate) fn write_opus(a: &Audio) -> Result<Vec<u8>> {
    let (pcm, ch) = stereo_or_mono(&a.floats(), usize::from(a.channels));
    let pcm = resample(&pcm, ch, a.rate, 48_000);
    let frame = 960; // 20 ms
    let mut enc = opus_pure::OpusEncoder::new(48_000, ch, opus_pure::Application::Audio)
        .map_err(|e| anyhow::anyhow!("Opus encoder: {e:?}"))?;
    enc.bitrate_bps = if ch == 1 { 64_000 } else { 128_000 };
    let head = opus_pure::OpusHead::for_encoder(&enc, a.rate);
    let mut out = Vec::new();
    {
        let mut w = opus_pure::OggOpusWriter::new(&mut out, head).map_err(|e| anyhow::anyhow!("Opus writer: {e:?}"))?;
        let mut packet = vec![0u8; opus_pure::MAX_PACKET_BYTES];
        // Pad the last frame with silence so nothing is cut off.
        let mut padded = pcm;
        padded.resize(padded.len().div_ceil(frame * ch) * frame * ch, 0.0);
        for block in padded.chunks_exact(frame * ch) {
            let n =
                enc.encode(block, frame, &mut packet).map_err(|e| anyhow::anyhow!("Opus encoding failed: {e:?}"))?;
            w.write_packet(&packet[..n]).map_err(|e| anyhow::anyhow!("Opus writer: {e:?}"))?;
        }
        w.finish().map_err(|e| anyhow::anyhow!("Opus writer: {e:?}"))?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(rate: u32, channels: u16, secs: f32) -> Audio {
        let frames = (rate as f32 * secs) as usize;
        let samples = (0..frames)
            .flat_map(|i| {
                let v = ((i as f32 * 440.0 * std::f32::consts::TAU / rate as f32).sin() * 12_000.0) as i32;
                std::iter::repeat_n(v, usize::from(channels))
            })
            .collect();
        Audio { samples, channels, rate, bits: 16 }
    }

    fn decoded_len(bytes: &[u8], ext: &str) -> (usize, u32) {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join(format!("x.{ext}"));
        std::fs::write(&p, bytes).unwrap();
        let a = decode_audio(&p).unwrap();
        (a.samples.len() / usize::from(a.channels), a.rate)
    }

    #[test]
    fn mp3_decodes_back() {
        let a = tone(44_100, 2, 1.0);
        let mp3 = write_mp3(&a).unwrap();
        let (frames, rate) = decoded_len(&mp3, "mp3");
        assert_eq!(rate, 44_100);
        assert!((44_000..=46_500).contains(&frames), "{frames}");
    }

    #[test]
    fn odd_rates_and_channel_counts() {
        // 96 kHz six-channel: MP3 and Opus mix down and resample.
        let a = tone(96_000, 6, 0.25);
        let (frames, rate) = decoded_len(&write_mp3(&a).unwrap(), "mp3");
        assert_eq!(rate, 44_100);
        assert!((10_500..=13_000).contains(&frames), "{frames}");
        let opus = write_opus(&a).unwrap();
        assert!(opus.starts_with(b"OggS") && opus.windows(8).any(|w| w == b"OpusHead"));
    }

    #[test]
    fn resampler_keeps_a_tone() {
        let pcm: Vec<f32> = (0..44_100).map(|i| (i as f32 * 1000.0 * std::f32::consts::TAU / 44_100.0).sin()).collect();
        let out = resample(&pcm, 1, 44_100, 48_000);
        assert_eq!(out.len(), 48_000);
        // Still a 1 kHz sine at the new rate.
        let err: f32 = (1000..47_000)
            .map(|i| (out[i] - (i as f32 * 1000.0 * std::f32::consts::TAU / 48_000.0).sin()).abs())
            .fold(0.0, f32::max);
        assert!(err < 0.02, "{err}");
    }
}
