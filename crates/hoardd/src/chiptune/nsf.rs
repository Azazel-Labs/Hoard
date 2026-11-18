//! NSF and NSFe: NES music, the game's own sound driver and data run on an emulated 6502 and the NES's sound chip
//! (and the cartridge chips some games added) through `oxideav-nsf`, in pure Rust.

use anyhow::{Result, anyhow, ensure};
use serde_json::{Map, json};

use crate::audio::Audio;
use crate::music::{Rendered, clip_text};

const RATE: u32 = oxideav_nsf::OUTPUT_SAMPLE_RATE;
/// How long a track plays when the file doesn't say: NES music loops forever.
const DEFAULT_MS: u32 = 150_000;
const FADE_MS: u32 = 5_000;

/// The first track (the one the file starts on), up to `max_secs`.
pub(super) fn render(data: &[u8], max_secs: f32) -> Result<Rendered> {
    let header = oxideav_nsf::parse_nsf(data).map_err(|e| anyhow!("not a readable NSF: {e:?}"))?;
    let mut facts = Map::new();
    for (key, text) in [("title", &header.song_name), ("artist", &header.artist), ("copyright", &header.copyright)] {
        let text = clip_text(text, 120);
        if !text.is_empty() && text != "<?>" {
            facts.insert(key.into(), json!(text));
        }
    }
    facts.insert("songs".into(), json!(header.track_count()));
    let names: Vec<String> =
        header.tracks().filter_map(|t| t.label.map(|l| clip_text(l, 60))).filter(|l| !l.is_empty()).take(48).collect();
    if !names.is_empty() {
        facts.insert("song_names".into(), json!(names));
    }
    let x = header.expansion;
    let chips: Vec<&str> = [
        (x.vrc6(), "VRC6"),
        (x.vrc7(), "VRC7"),
        (x.fds(), "FDS"),
        (x.mmc5(), "MMC5"),
        (x.n163(), "N163"),
        (x.s5b(), "5B"),
    ]
    .into_iter()
    .filter_map(|(on, name)| on.then_some(name))
    .collect();
    if !chips.is_empty() {
        facts.insert("expansion_chips".into(), json!(chips));
    }
    let song = header.starting_song_number();
    let mut player = oxideav_nsf::NsfPlayer::new(header, RATE);
    player.set_default_track_time_ms(Some(DEFAULT_MS));
    player.set_default_fade_ms(FADE_MS);
    player.start_song(song);
    let cap = (max_secs * RATE as f32) as usize;
    let mut mono = vec![0i16; cap];
    let mut filled = 0;
    while filled < cap {
        let n = player.render(&mut mono[filled..(filled + RATE as usize).min(cap)]);
        filled += n;
        if n == 0 || player.track_finished() {
            break;
        }
    }
    mono.truncate(filled);
    ensure!(mono.iter().any(|&s| s != 0), "the NSF played no audio");
    if filled >= cap {
        facts.insert("preview_truncated".into(), json!(true));
    }
    let samples = mono.iter().flat_map(|&s| [i32::from(s); 2]).collect();
    Ok(Rendered { audio: Audio { samples, channels: 2, rate: RATE, bits: 16 }, facts })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A one-song NSF holding a steady square wave, named.
    pub(crate) fn tiny_nsf() -> Vec<u8> {
        let (header, program) = oxideav_nsf::player::_tiny_test_program();
        let mut nsf = header.to_vec();
        nsf[0x0E..0x0E + 10].copy_from_slice(b"Overworld\0");
        nsf[0x2E..0x2E + 6].copy_from_slice(b"Hoard\0");
        nsf.extend(program);
        nsf
    }

    #[test]
    fn plays_the_first_song() {
        let r = render(&tiny_nsf(), 2.0).unwrap();
        assert_eq!(r.facts["title"], "Overworld");
        assert_eq!(r.facts["artist"], "Hoard");
        assert_eq!(r.facts["songs"], 1);
        assert_eq!(r.audio.samples.len(), 2 * 2 * 44_100);
        assert_eq!(r.facts["preview_truncated"], true);
        assert!(r.audio.samples.iter().any(|&s| s.abs() > 1000));
        assert!(render(b"NESM\x1a", 1.0).is_err());
    }
}
