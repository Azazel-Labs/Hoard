//! Note-based music: tracker modules (MOD, XM, S3M, IT) and MIDI. Both are rendered to audio, which gives them a
//! waveform thumbnail, a playable preview and "download as" WAV/FLAC/MP3/OGG/Opus, plus facts about the song.
//!
//! Modules play through `xmrsplayer` (pure Rust). MIDI renders with `rustysynth` (pure Rust) and a General MIDI
//! SoundFont: `HOARD_SOUNDFONT`, or one found in the usual system places. Without a SoundFont a small built-in
//! synthesizer gives a rough but recognizable preview.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use anyhow::{Context, Result, ensure};
use serde_json::{Map, Value, json};

use crate::audio::Audio;

pub(crate) const EXTS: &[&str] = &["mod", "xm", "s3m", "it", "mid", "midi"];
const RATE: u32 = 44_100;

/// Where to look for a General MIDI SoundFont when `HOARD_SOUNDFONT` isn't set (Debian/Ubuntu packages, macOS
/// Homebrew's fluid-synth, a user folder).
const SOUNDFONT_PATHS: &[&str] = &[
    "/usr/share/sounds/sf2/default-GM.sf2",
    "/usr/share/sounds/sf2/FluidR3_GM.sf2",
    "/usr/share/sounds/sf2/TimGM6mb.sf2",
    "/usr/share/soundfonts/default.sf2",
    "/usr/share/soundfonts/FluidR3_GM.sf2",
    "/opt/homebrew/share/soundfonts/default.sf2",
    "/usr/local/share/soundfonts/default.sf2",
];

static SOUNDFONT_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Set the SoundFont from config (call once at startup); otherwise the usual paths are searched.
pub(crate) fn configure(path: Option<PathBuf>) {
    SOUNDFONT_PATH.set(path).ok();
}

fn soundfont() -> Option<(Arc<rustysynth::SoundFont>, String)> {
    static FONT: OnceLock<Option<(Arc<rustysynth::SoundFont>, String)>> = OnceLock::new();
    FONT.get_or_init(|| {
        let configured = SOUNDFONT_PATH.get().cloned().flatten();
        let path = configured.or_else(|| SOUNDFONT_PATHS.iter().map(PathBuf::from).find(|p| p.is_file()))?;
        let mut f = std::io::BufReader::new(std::fs::File::open(&path).ok()?);
        match rustysynth::SoundFont::new(&mut f) {
            Ok(sf) => {
                let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                tracing::info!(soundfont = %path.display(), "MIDI previews use this SoundFont");
                Some((Arc::new(sf), name))
            }
            Err(e) => {
                tracing::warn!(soundfont = %path.display(), error = ?e, "couldn't load the SoundFont; MIDI uses the built-in synth");
                None
            }
        }
    })
    .clone()
}

/// The song rendered (up to `max_secs`), and what's known about it.
pub(crate) struct Rendered {
    pub(crate) audio: Audio,
    pub(crate) facts: Map<String, Value>,
}

pub(crate) fn render(path: &Path, ext: &str, max_secs: f32) -> Result<Rendered> {
    let data = std::fs::read(path)?;
    match ext {
        "mid" | "midi" => midi(&data, path, max_secs),
        _ => module(&data, max_secs),
    }
}

fn clip_text(s: &str, n: usize) -> String {
    s.chars().filter(|c| !c.is_control() || *c == '\n').take(n).collect::<String>().trim().to_owned()
}

fn module(data: &[u8], max_secs: f32) -> Result<Rendered> {
    use xmrsplayer::prelude::*;
    let module =
        xmrs::prelude::Module::load(data).map_err(|e| anyhow::anyhow!("not a readable tracker module: {e:?}"))?;
    let mut facts = Map::new();
    let title = clip_text(&module.name, 80);
    if !title.is_empty() {
        facts.insert("title".into(), json!(title));
    }
    let comment = clip_text(&module.comment, 600);
    if !comment.is_empty() {
        facts.insert("comment".into(), json!(comment));
    }
    if let Some(origin) = &module.origin {
        facts.insert("tracker_format".into(), json!(format!("{origin:?}")));
    }
    facts.insert("bpm".into(), json!(module.default_bpm));
    // Instrument and sample names: often the musician's credits and greetings.
    let names: Vec<String> =
        module.instrument.iter().map(|i| clip_text(&i.name, 40)).filter(|n| !n.is_empty()).take(48).collect();
    facts.insert("instruments".into(), json!(module.instrument.len()));
    if !names.is_empty() {
        facts.insert("instrument_names".into(), json!(names));
    }
    let mut player = XmrsPlayer::new(&module, RATE, 0);
    player.set_max_loop_count(1);
    let cap = (max_secs * RATE as f32) as usize * 2;
    let samples: Vec<i32> = player.by_ref().take(cap).map(i32::from).collect();
    ensure!(samples.len() >= 2, "the module played no audio");
    if samples.len() >= cap {
        facts.insert("preview_truncated".into(), json!(true));
    }
    Ok(Rendered { audio: Audio { samples, channels: 2, rate: RATE, bits: 16 }, facts })
}

/// General MIDI program names, for the facts.
const GM: [&str; 128] = [
    "Acoustic Grand Piano",
    "Bright Acoustic Piano",
    "Electric Grand Piano",
    "Honky-tonk Piano",
    "Electric Piano 1",
    "Electric Piano 2",
    "Harpsichord",
    "Clavinet",
    "Celesta",
    "Glockenspiel",
    "Music Box",
    "Vibraphone",
    "Marimba",
    "Xylophone",
    "Tubular Bells",
    "Dulcimer",
    "Drawbar Organ",
    "Percussive Organ",
    "Rock Organ",
    "Church Organ",
    "Reed Organ",
    "Accordion",
    "Harmonica",
    "Tango Accordion",
    "Nylon Guitar",
    "Steel Guitar",
    "Jazz Guitar",
    "Clean Guitar",
    "Muted Guitar",
    "Overdriven Guitar",
    "Distortion Guitar",
    "Guitar Harmonics",
    "Acoustic Bass",
    "Fingered Bass",
    "Picked Bass",
    "Fretless Bass",
    "Slap Bass 1",
    "Slap Bass 2",
    "Synth Bass 1",
    "Synth Bass 2",
    "Violin",
    "Viola",
    "Cello",
    "Contrabass",
    "Tremolo Strings",
    "Pizzicato Strings",
    "Orchestral Harp",
    "Timpani",
    "String Ensemble 1",
    "String Ensemble 2",
    "Synth Strings 1",
    "Synth Strings 2",
    "Choir Aahs",
    "Voice Oohs",
    "Synth Voice",
    "Orchestra Hit",
    "Trumpet",
    "Trombone",
    "Tuba",
    "Muted Trumpet",
    "French Horn",
    "Brass Section",
    "Synth Brass 1",
    "Synth Brass 2",
    "Soprano Sax",
    "Alto Sax",
    "Tenor Sax",
    "Baritone Sax",
    "Oboe",
    "English Horn",
    "Bassoon",
    "Clarinet",
    "Piccolo",
    "Flute",
    "Recorder",
    "Pan Flute",
    "Blown Bottle",
    "Shakuhachi",
    "Whistle",
    "Ocarina",
    "Square Lead",
    "Sawtooth Lead",
    "Calliope Lead",
    "Chiff Lead",
    "Charang Lead",
    "Voice Lead",
    "Fifths Lead",
    "Bass + Lead",
    "New Age Pad",
    "Warm Pad",
    "Polysynth Pad",
    "Choir Pad",
    "Bowed Pad",
    "Metallic Pad",
    "Halo Pad",
    "Sweep Pad",
    "Rain",
    "Soundtrack",
    "Crystal",
    "Atmosphere",
    "Brightness",
    "Goblins",
    "Echoes",
    "Sci-fi",
    "Sitar",
    "Banjo",
    "Shamisen",
    "Koto",
    "Kalimba",
    "Bagpipe",
    "Fiddle",
    "Shanai",
    "Tinkle Bell",
    "Agogo",
    "Steel Drums",
    "Woodblock",
    "Taiko Drum",
    "Melodic Tom",
    "Synth Drum",
    "Reverse Cymbal",
    "Guitar Fret Noise",
    "Breath Noise",
    "Seashore",
    "Bird Tweet",
    "Telephone Ring",
    "Helicopter",
    "Applause",
    "Gunshot",
];

/// A note, in seconds.
struct Note {
    start: f64,
    end: f64,
    key: u8,
    velocity: u8,
    program: u8,
    drums: bool,
}

/// Notes and facts from a MIDI file, with ticks turned into seconds through the tempo map.
fn read_midi(data: &[u8]) -> Result<(Vec<Note>, Map<String, Value>, f64)> {
    use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
    let smf = Smf::parse(data).map_err(|e| anyhow::anyhow!("not a readable MIDI file: {e}"))?;
    let mut facts = Map::new();
    facts.insert("midi_format".into(), json!(format!("{:?}", smf.header.format)));
    facts.insert("tracks".into(), json!(smf.tracks.len()));
    // All events on one timeline: (tick, track, event).
    let mut events = Vec::new();
    for (t, track) in smf.tracks.iter().enumerate() {
        let mut tick = 0u64;
        for e in track {
            tick += u64::from(e.delta.as_int());
            events.push((tick, t, e.kind));
        }
    }
    events.sort_by_key(|(tick, t, _)| (*tick, *t));
    let (mut tempo_us, mut last_tick, mut secs) = (500_000.0f64, 0u64, 0.0f64);
    let tick_secs = |tempo: f64, ticks: u64| -> f64 {
        match smf.header.timing {
            Timing::Metrical(tpb) => ticks as f64 * tempo / 1e6 / f64::from(tpb.as_int().max(1)),
            Timing::Timecode(fps, sub) => ticks as f64 / (f64::from(fps.as_f32()) * f64::from(sub.max(1))),
        }
    };
    let mut notes: Vec<Note> = Vec::new();
    // Sounding notes by (channel, key): when they started and how hard.
    let mut open: HashMap<(u8, u8), (f64, u8)> = HashMap::new();
    let mut programs = [0u8; 16];
    let mut used_programs = std::collections::BTreeSet::new();
    let (mut names, mut first_tempo, mut time_sig, mut drums_used) = (Vec::new(), None, None, false);
    for (tick, _, kind) in events {
        secs += tick_secs(tempo_us, tick - last_tick);
        last_tick = tick;
        match kind {
            TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                tempo_us = f64::from(t.as_int());
                first_tempo.get_or_insert(60_000_000.0 / tempo_us);
            }
            TrackEventKind::Meta(MetaMessage::TrackName(n)) => {
                let n = clip_text(&String::from_utf8_lossy(n), 60);
                if !n.is_empty() && names.len() < 24 {
                    names.push(n);
                }
            }
            TrackEventKind::Meta(MetaMessage::TimeSignature(num, den, _, _)) => {
                time_sig.get_or_insert(format!("{num}/{}", 1u32 << den.min(6)));
            }
            TrackEventKind::Midi { channel, message } => {
                let ch = channel.as_int();
                match message {
                    MidiMessage::ProgramChange { program } => programs[usize::from(ch)] = program.as_int(),
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        open.insert((ch, key.as_int()), (secs, vel.as_int()));
                        if ch == 9 {
                            drums_used = true;
                        } else {
                            used_programs.insert(programs[usize::from(ch)]);
                        }
                    }
                    MidiMessage::NoteOn { key, .. } | MidiMessage::NoteOff { key, .. } => {
                        if let Some((start, velocity)) = open.remove(&(ch, key.as_int())) {
                            notes.push(Note {
                                start,
                                end: secs,
                                key: key.as_int(),
                                velocity,
                                program: programs[usize::from(ch)],
                                drums: ch == 9,
                            });
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    // Notes never released end with the song.
    for ((ch, key), (start, velocity)) in open {
        notes.push(Note {
            start,
            end: secs.max(start + 0.5),
            key,
            velocity,
            program: programs[usize::from(ch)],
            drums: ch == 9,
        });
    }
    if let Some(bpm) = first_tempo {
        facts.insert("tempo_bpm".into(), json!((bpm * 10.0).round() / 10.0));
    }
    if let Some(ts) = time_sig {
        facts.insert("time_signature".into(), json!(ts));
    }
    if !names.is_empty() {
        facts.insert("track_names".into(), json!(names));
        facts.insert("title".into(), json!(names[0]));
    }
    let instruments: Vec<&str> = used_programs.iter().map(|&p| GM[usize::from(p & 127)]).collect();
    facts.insert("instruments".into(), json!(instruments));
    if drums_used {
        facts.insert("drums".into(), json!(true));
    }
    facts.insert("notes".into(), json!(notes.len()));
    Ok((notes, facts, secs))
}

fn midi(data: &[u8], path: &Path, max_secs: f32) -> Result<Rendered> {
    let (notes, mut facts, length) = read_midi(data)?;
    ensure!(!notes.is_empty(), "the MIDI file has no notes");
    let secs = (length + 1.5).min(f64::from(max_secs));
    if length + 1.5 > f64::from(max_secs) {
        facts.insert("preview_truncated".into(), json!(true));
    }
    let frames = (secs * f64::from(RATE)) as usize;
    let (left, right, synth) = if let Some((sf, name)) = soundfont() {
        let (l, r) = soundfont_render(&sf, path, frames)?;
        (l, r, format!("SoundFont: {name}"))
    } else {
        let mono = builtin_synth(&notes, frames);
        (mono.clone(), mono, "built-in synth (set HOARD_SOUNDFONT for real instruments)".to_owned())
    };
    facts.insert("preview_synth".into(), json!(synth));
    // Normalize: SoundFont output levels vary a lot between files and fonts.
    let peak = left.iter().chain(&right).fold(0.0f32, |m, s| m.max(s.abs()));
    let gain = if peak > 0.0 { (0.89 / peak).min(8.0) } else { 1.0 };
    let samples = left
        .iter()
        .zip(&right)
        .flat_map(|(l, r)| [l, r])
        .map(|s| ((s * gain).clamp(-1.0, 1.0) * 32_767.0) as i32)
        .collect();
    Ok(Rendered { audio: Audio { samples, channels: 2, rate: RATE, bits: 16 }, facts })
}

fn soundfont_render(sf: &Arc<rustysynth::SoundFont>, path: &Path, frames: usize) -> Result<(Vec<f32>, Vec<f32>)> {
    let settings = rustysynth::SynthesizerSettings::new(i32::try_from(RATE)?);
    let synth = rustysynth::Synthesizer::new(sf, &settings).map_err(|e| anyhow::anyhow!("synthesizer: {e:?}"))?;
    let mut seq = rustysynth::MidiFileSequencer::new(synth);
    let mut f = std::io::BufReader::new(std::fs::File::open(path)?);
    let midi = Arc::new(rustysynth::MidiFile::new(&mut f).map_err(|e| anyhow::anyhow!("MIDI file: {e:?}"))?);
    seq.play(&midi, false);
    let (mut left, mut right) = (vec![0.0f32; frames], vec![0.0f32; frames]);
    for (l, r) in left.chunks_mut(4096).zip(right.chunks_mut(4096)) {
        seq.render(l, r);
    }
    Ok((left, right))
}

/// A few oscillators and envelopes, picked by General MIDI family, for when there's no SoundFont.
#[expect(clippy::cast_precision_loss, reason = "sample positions in a few minutes of audio")]
fn builtin_synth(notes: &[Note], frames: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; frames];
    let rate = f64::from(RATE);
    let mut noise = 0x1234_5678u32;
    for n in notes {
        let start = (n.start * rate) as usize;
        if start >= frames {
            continue;
        }
        let vel = f32::from(n.velocity) / 127.0;
        if n.drums {
            // Kicks are a falling sine, everything else a noise burst; the key sets the length.
            let (len, kick) = match n.key {
                35 | 36 => (0.25, true),
                42 | 44 | 46 => (0.06, false),
                49 | 51 | 52 | 55 | 57 | 59 => (0.6, false),
                _ => (0.15, false),
            };
            let count = ((len * rate) as usize).min(frames - start);
            for i in 0..count {
                let t = i as f64 / rate;
                let env = (1.0 - t / len).max(0.0) as f32;
                let v = if kick {
                    (std::f64::consts::TAU * (50.0 + 90.0 * (1.0 - t / len)) * t).sin() as f32
                } else {
                    noise ^= noise << 13;
                    noise ^= noise >> 17;
                    noise ^= noise << 5;
                    (noise as f32 / u32::MAX as f32) * 2.0 - 1.0
                };
                out[start + i] += v * env * env * vel * 0.3;
            }
            continue;
        }
        let freq = 440.0 * 2f64.powf((f64::from(n.key) - 69.0) / 12.0);
        let family = n.program / 8;
        // Plucked and struck families decay; sustained ones hold until release.
        let percussive = matches!(family, 0 | 1 | 3 | 4 | 13);
        let held = (n.end - n.start).max(0.05);
        let release = 0.08;
        let length = if percussive { held.min(2.5) + release } else { held + release };
        let count = ((length * rate) as usize).min(frames - start);
        for i in 0..count {
            let t = i as f64 / rate;
            let phase = (freq * t).fract();
            let wave = match family {
                2 | 10 | 11 | 12 => {
                    (std::f64::consts::TAU * phase).sin() * 0.6 + (std::f64::consts::TAU * 2.0 * phase).sin() * 0.25
                }
                5 | 7 | 8 | 9 => 2.0 * phase - 1.0,
                6 => 1.0 - 4.0 * (phase - 0.5).abs(),
                4 => {
                    if phase < 0.5 {
                        0.6
                    } else {
                        -0.6
                    }
                }
                _ => (std::f64::consts::TAU * phase).sin() + (std::f64::consts::TAU * 3.0 * phase).sin() * 0.2,
            };
            let attack = (t / 0.01).min(1.0);
            let body = if percussive { (-t * 3.0).exp() } else { 1.0 };
            let tail = if t > held { (1.0 - (t - held) / release).max(0.0) } else { 1.0 };
            out[start + i] += (wave * attack * body * tail) as f32 * vel * 0.12;
        }
    }
    out
}

/// Facts plus a waveform and an MP3 preview, for analysis. Renders up to six minutes.
pub(crate) fn analyze(path: &Path, ext: &str) -> Result<(Map<String, Value>, Audio)> {
    let r = render(path, ext, 360.0)?;
    let mut facts = r.facts;
    let frames = r.audio.samples.len() / usize::from(r.audio.channels);
    facts.insert("duration_secs".into(), json!((frames as f64 / f64::from(r.audio.rate) * 100.0).round() / 100.0));
    facts.insert("channels".into(), json!(2));
    facts.insert("sample_rate".into(), json!(r.audio.rate));
    Ok((facts, r.audio))
}

/// For "download as": the whole song (up to 20 minutes).
pub(crate) fn render_full(path: &Path, ext: &str) -> Result<Audio> {
    render(path, ext, 1200.0).map(|r| r.audio).context("couldn't render the music")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-track MIDI file: tempo, a name, a program change, C-E-G as quarter notes, a kick on the drum channel.
    pub(crate) fn tiny_midi() -> Vec<u8> {
        use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};
        let ev = |delta: u32, kind| TrackEvent { delta: delta.into(), kind };
        let on = |ch: u8, key: u8| TrackEventKind::Midi {
            channel: ch.into(),
            message: MidiMessage::NoteOn { key: key.into(), vel: 100.into() },
        };
        let off = |ch: u8, key: u8| TrackEventKind::Midi {
            channel: ch.into(),
            message: MidiMessage::NoteOff { key: key.into(), vel: 0.into() },
        };
        let track = vec![
            ev(0, TrackEventKind::Meta(MetaMessage::TrackName(b"Lead"))),
            ev(0, TrackEventKind::Meta(MetaMessage::Tempo(500_000.into()))),
            ev(
                0,
                TrackEventKind::Midi { channel: 0.into(), message: MidiMessage::ProgramChange { program: 73.into() } },
            ),
            ev(0, on(0, 60)),
            ev(0, on(9, 36)),
            ev(480, off(0, 60)),
            ev(0, off(9, 36)),
            ev(0, on(0, 64)),
            ev(480, off(0, 64)),
            ev(0, on(0, 67)),
            ev(480, off(0, 67)),
            ev(0, TrackEventKind::Meta(MetaMessage::EndOfTrack)),
        ];
        let smf = Smf { header: Header::new(Format::SingleTrack, Timing::Metrical(480.into())), tracks: vec![track] };
        let mut out = Vec::new();
        smf.write_std(&mut out).unwrap();
        out
    }

    #[test]
    fn midi_facts_and_builtin_render() {
        let (notes, facts, secs) = read_midi(&tiny_midi()).unwrap();
        assert_eq!(notes.len(), 4);
        assert!((secs - 1.5).abs() < 1e-6, "three quarter notes at 120 bpm: {secs}");
        assert_eq!(facts["instruments"], json!(["Flute"]));
        assert_eq!(facts["drums"], true);
        assert_eq!(facts["tempo_bpm"], 120.0);
        let audio = builtin_synth(&notes, 44_100 * 2);
        let loud = audio.iter().filter(|s| s.abs() > 0.01).count();
        assert!(loud > 44_100, "the notes sound: {loud}");
    }

    #[test]
    fn midi_renders_with_whatever_synth_is_available() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("t.mid");
        std::fs::write(&p, tiny_midi()).unwrap();
        let (facts, audio) = analyze(&p, "mid").unwrap();
        assert!(facts["duration_secs"].as_f64().unwrap() >= 1.5);
        assert!(audio.samples.iter().any(|&s| s.abs() > 3000), "rendered something audible");
        assert!(facts.contains_key("preview_synth"));
    }

    /// A minimal 4-channel ProTracker MOD: one square-wave sample, one pattern playing it on row 0.
    pub(crate) fn tiny_mod() -> Vec<u8> {
        let mut b = Vec::new();
        let mut title = b"hoard test".to_vec();
        title.resize(20, 0);
        b.extend(title);
        let sample_len_words = 32u16;
        for i in 0..31 {
            let mut name = if i == 0 { b"square by hoard".to_vec() } else { Vec::new() };
            name.resize(22, 0);
            b.extend(name);
            let len = if i == 0 { sample_len_words } else { 0 };
            b.extend(len.to_be_bytes());
            b.push(0); // finetune
            b.push(if i == 0 { 64 } else { 0 }); // volume
            b.extend(0u16.to_be_bytes()); // loop start
            b.extend((if i == 0 { sample_len_words } else { 1 }).to_be_bytes()); // loop length: loop the whole sample
        }
        b.push(1); // song length
        b.push(127);
        let mut order = vec![0u8; 128];
        order[0] = 0;
        b.extend(order);
        b.extend(b"M.K.");
        // Pattern 0: 64 rows x 4 channels x 4 bytes. Row 0, channel 0: sample 1, period 428 (C-2).
        let mut pattern = vec![0u8; 64 * 4 * 4];
        let period: u16 = 428;
        pattern[0] = (period >> 8) as u8 & 0x0f;
        pattern[1] = (period & 0xff) as u8;
        pattern[2] = 1 << 4;
        b.extend(pattern);
        b.extend((0..sample_len_words * 2).map(|i| if i % 16 < 8 { 100u8 } else { 156 }));
        b
    }

    #[test]
    fn module_facts_and_render() {
        let r = module(&tiny_mod(), 30.0).unwrap();
        assert_eq!(r.facts["title"], "hoard test");
        assert_eq!(r.facts["instrument_names"][0], "square by hoard");
        assert!(r.audio.samples.iter().any(|&s| s.abs() > 1000), "the sample plays");
        let secs = r.audio.samples.len() as f64 / 2.0 / f64::from(RATE);
        assert!(secs > 1.0 && secs < 30.0, "one pass through the pattern, not a forever loop: {secs}");
    }
}

/// Writes test fixtures for end-to-end runs: `HOARD_FIXTURE_OUT=dir cargo test -p hoardd write_music_fixtures --
/// --ignored`.
#[cfg(test)]
#[test]
#[ignore = "writes files; run by hand"]
fn write_music_fixtures() {
    let Some(out) = std::env::var_os("HOARD_FIXTURE_OUT") else { return };
    let out = PathBuf::from(out);
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("theme.mid"), tests::tiny_midi()).unwrap();
    std::fs::write(out.join("chip.mod"), tests::tiny_mod()).unwrap();
}
