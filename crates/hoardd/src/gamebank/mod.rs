//! Game audio middleware: FMOD's sound banks (FSB5, and the Studio `.bank` files that carry one) and Wwise's sounds
//! (WEM) and sound banks (BNK). A bank plays its sounds one after another with a short gap between them, which gives
//! it a waveform, a preview and "download as" like any other sound.

mod fsb;
mod wwise;

use anyhow::{Result, anyhow, bail};
use serde_json::{Map, json};

use crate::audio::Audio;
use crate::music::Rendered;

/// Formats played here.
pub(crate) const EXTS: &[&str] = &["fsb", "bank", "wem", "bnk"];
/// Silence between a bank's sounds.
const GAP_SECS: f32 = 0.3;
/// Sound names listed in the facts.
const MAX_NAMES: usize = 48;
/// When this many sounds in a row won't decode, the rest are taken to be the same.
const GIVE_UP_AFTER: usize = 8;

/// One decoded sound: interleaved samples (about -1..=1).
pub(super) struct Sound {
    pcm: Vec<f32>,
    channels: usize,
    rate: u32,
}

/// Decodes sound `i`, stopping once it has about `secs` seconds.
type Decode<'a> = Box<dyn Fn(usize, f32) -> Result<Sound> + 'a>;

/// What a file holds: its sounds, their names when it keeps them, and the codec they're in.
pub(super) struct Bank<'a> {
    container: &'static str,
    codec: String,
    names: Vec<String>,
    count: usize,
    decode: Decode<'a>,
}

pub(crate) fn render(data: &[u8], ext: &str, max_secs: f32) -> Result<Rendered> {
    let bank = match ext {
        "fsb" => fsb::fsb(data, "FMOD sound bank")?,
        "bank" => fsb::studio_bank(data)?,
        "wem" => wwise::wem(data)?,
        "bnk" => wwise::bnk(data)?,
        _ => bail!("no game audio player for .{ext}"),
    };
    play(&bank, max_secs)
}

/// The bank's sounds in order, as stereo at the first sound's rate, up to `max_secs`.
fn play(bank: &Bank, max_secs: f32) -> Result<Rendered> {
    let mut facts = Map::new();
    facts.insert("container".into(), json!(bank.container));
    facts.insert("codec".into(), json!(bank.codec));
    if bank.count > 1 {
        facts.insert("sounds".into(), json!(bank.count));
    }
    let names: Vec<&String> = bank.names.iter().filter(|n| !n.is_empty()).take(MAX_NAMES).collect();
    if !names.is_empty() {
        facts.insert("sound_names".into(), json!(names));
    }
    let (mut out, mut rate, mut played, mut failed) = (Vec::<f32>::new(), 0u32, 0usize, 0usize);
    let mut first_error = None;
    for i in 0..bank.count {
        let left = if rate == 0 { max_secs } else { max_secs - out.len() as f32 / 2.0 / rate as f32 };
        if left <= 0.0 {
            break;
        }
        let sound = match (bank.decode)(i, left) {
            Ok(s) if !s.pcm.is_empty() && s.channels > 0 && s.rate > 0 => s,
            Ok(_) => continue,
            Err(e) => {
                tracing::debug!(sound = i, error = %format!("{e:#}"), "a sound in the bank didn't decode");
                first_error.get_or_insert(e);
                failed += 1;
                if failed >= GIVE_UP_AFTER && played == 0 {
                    break;
                }
                continue;
            }
        };
        failed = 0;
        if rate == 0 {
            rate = sound.rate;
        }
        let (pcm, channels) = crate::audio::stereo_or_mono(&sound.pcm, sound.channels);
        let stereo: Vec<f32> = if channels == 1 { pcm.iter().flat_map(|&s| [s, s]).collect() } else { pcm };
        if !out.is_empty() {
            out.resize(out.len() + 2 * (GAP_SECS * rate as f32) as usize, 0.0);
        }
        out.extend(crate::audio::resample(&stereo, 2, sound.rate, rate));
        played += 1;
    }
    if played == 0 {
        return Err(first_error.unwrap_or_else(|| anyhow!("the bank holds no sounds")));
    }
    let cap = 2 * (max_secs * rate as f32) as usize;
    if out.len() > cap || played < bank.count {
        facts.insert("preview_truncated".into(), json!(true));
    }
    out.truncate(cap);
    let samples = out.iter().map(|s| (s.clamp(-1.0, 1.0) * 32_767.0).round() as i32).collect();
    Ok(Rendered { audio: Audio { samples, channels: 2, rate, bits: 16 }, facts })
}

/// Little- or big-endian reads that fail past the end of `data`.
#[derive(Clone, Copy)]
pub(super) struct Reader<'a> {
    data: &'a [u8],
    big: bool,
}

impl<'a> Reader<'a> {
    pub(super) fn le(data: &'a [u8]) -> Self {
        Self { data, big: false }
    }

    pub(super) fn bytes(&self, at: usize, n: usize) -> Result<&'a [u8]> {
        at.checked_add(n).and_then(|end| self.data.get(at..end)).ok_or_else(|| anyhow!("cut short at {at:#x}"))
    }

    pub(super) fn u16(&self, at: usize) -> Result<u16> {
        let b: [u8; 2] = self.bytes(at, 2)?.try_into()?;
        Ok(if self.big { u16::from_be_bytes(b) } else { u16::from_le_bytes(b) })
    }

    pub(super) fn u32(&self, at: usize) -> Result<u32> {
        let b: [u8; 4] = self.bytes(at, 4)?.try_into()?;
        Ok(if self.big { u32::from_be_bytes(b) } else { u32::from_le_bytes(b) })
    }

    pub(super) fn u64(&self, at: usize) -> Result<u64> {
        let b: [u8; 8] = self.bytes(at, 8)?.try_into()?;
        Ok(if self.big { u64::from_be_bytes(b) } else { u64::from_le_bytes(b) })
    }
}

/// Interleaved 16-bit samples as floats.
fn from_i16(samples: impl IntoIterator<Item = i16>) -> Vec<f32> {
    samples.into_iter().map(|s| f32::from(s) / 32_768.0).collect()
}

/// A frame count for `secs` at `rate`.
fn frames_for(secs: f32, rate: u32) -> usize {
    (secs.max(0.0) * rate as f32).ceil() as usize
}

/// Decode a compressed stream with the shared audio decoder (MP3, Ogg Vorbis), up to `secs`.
fn decode_stream(bytes: Vec<u8>, ext: &str, secs: f32) -> Result<Sound> {
    let (mut pcm, mut channels, mut rate) = (Vec::new(), 0usize, 0u32);
    crate::audio::decode_each(Box::new(std::io::Cursor::new(bytes)), Some(ext), |p, c, r, _| {
        (channels, rate) = (usize::from(c), r);
        pcm.extend_from_slice(p);
        Ok(pcm.len() / channels.max(1) < frames_for(secs, rate))
    })?;
    Ok(Sound { pcm, channels, rate })
}

#[cfg(test)]
pub(crate) mod tests {
    pub(crate) use super::fsb::tests::{tiny_fmod_bank, tiny_fsb};
    pub(crate) use super::wwise::tests::{tiny_bnk, tiny_wem};

    /// A sine at `hz` for `secs`, as 16-bit samples.
    pub(super) fn tone(hz: f32, secs: f32, rate: u32) -> Vec<i16> {
        (0..(secs * rate as f32) as usize)
            .map(|i| ((std::f32::consts::TAU * hz * i as f32 / rate as f32).sin() * 12_000.0) as i16)
            .collect()
    }

    #[test]
    fn plays_a_banks_sounds_in_turn() {
        let r = super::render(&tiny_fsb(), "fsb", 30.0).unwrap();
        assert_eq!(r.facts["container"], "FMOD sound bank");
        assert_eq!(r.facts["codec"], "PCM");
        assert_eq!(r.facts["sounds"], 2);
        assert_eq!(r.facts["sound_names"], serde_json::json!(["beep", "boop"]));
        assert!(r.facts.get("preview_truncated").is_none());
        // Two 0.25 s sounds and the gap between them.
        assert_eq!(r.audio.rate, 44_100);
        let frames = r.audio.samples.len() / 2;
        assert!((frames as f32 / 44_100.0 - 0.8).abs() < 0.01, "{frames} frames");
        // Cut short: one sound, and the facts say so.
        let r = super::render(&tiny_fsb(), "fsb", 0.1).unwrap();
        assert_eq!(r.audio.samples.len(), 2 * 4410);
        assert_eq!(r.facts["preview_truncated"], true);
    }
}
