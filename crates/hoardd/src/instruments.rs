//! Sampled instruments: SFZ (a text file mapping the samples beside it to keys and velocities) and SF2 SoundFonts
//! (the samples and their mapping in one file). Neither is a sound on its own, so Hoard plays a few notes on each, a
//! rising arpeggio and a chord (a beat for a drum kit), which gives it the waveform, preview and sound search any
//! sound gets, and says which keys it covers. An SFZ's samples are linked to it, so they go with it.
//!
//! SF2 plays through `rustysynth`, which MIDI previews use too. SFZ is read here: headers and opcodes with their
//! inheritance, `#define` and `#include`, and the opcodes that decide which sample sounds and at what pitch; each
//! sample is decoded by `symphonia`, pitched by resampling and given a short release.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result, ensure};
use serde_json::{Map, Value, json};

use crate::audio::Audio;

pub(crate) const EXTS: &[&str] = &["sfz", "sf2"];
const RATE: u32 = 44_100;
/// The notes played: semitones above the root, start and length in seconds.
const PHRASE: &[(i32, f32, f32)] = &[
    (0, 0.0, 0.4),
    (4, 0.4, 0.4),
    (7, 0.8, 0.4),
    (12, 1.2, 0.4),
    (0, 1.8, 1.4),
    (4, 1.8, 1.4),
    (7, 1.8, 1.4),
    (12, 1.8, 1.4),
];
/// Kick, snare, closed and open hats, crash, toms, ride: General MIDI's drum keys, for a kit.
const DRUM_KEYS: &[u8] = &[36, 38, 42, 46, 49, 45, 48, 51];
/// The preview's length, with room for the last notes to ring out.
const LENGTH: f32 = 4.5;
/// Facts, and the left and right channels played.
type Played = (Map<String, Value>, Vec<f32>, Vec<f32>);
const VELOCITY: u8 = 100;
/// The most of one sample decoded: plenty for a note.
const MAX_SAMPLE_SECS: u32 = 20;
/// `#include` depth, against loops.
const MAX_INCLUDES: usize = 8;

/// Facts plus the notes played, for analysis.
pub(crate) fn analyze(path: &Path, ext: &str) -> Result<(Map<String, Value>, Audio)> {
    let (mut facts, left, right) = if ext == "sf2" { sf2(path)? } else { sfz(path)? };
    let peak = left.iter().chain(&right).fold(0.0f32, |m, s| m.max(s.abs()));
    ensure!(peak > 0.0, "the instrument played nothing");
    let gain = 0.89 / peak;
    let samples =
        left.iter().zip(&right).flat_map(|(l, r)| [l, r]).map(|s| ((s * gain).clamp(-1.0, 1.0) * 32_767.0) as i32);
    let audio = Audio { samples: samples.collect(), channels: 2, rate: RATE, bits: 16 };
    facts.insert("duration_secs".into(), json!(LENGTH));
    facts.insert("channels".into(), json!(2));
    facts.insert("sample_rate".into(), json!(RATE));
    Ok((facts, audio))
}

/// The keys to play, given which keys sound: the phrase on the C nearest the middle of the range (middle C when
/// the range has it), or a beat on a kit.
fn notes(keys: &[bool; 128], drums: bool) -> Vec<(u8, f32, f32)> {
    let playable: Vec<u8> = (0..128u8).filter(|&k| keys[usize::from(k)]).collect();
    let (Some(&lo), Some(&hi)) = (playable.first(), playable.last()) else { return Vec::new() };
    if drums {
        let mut chosen: Vec<u8> = DRUM_KEYS.iter().copied().filter(|&k| keys[usize::from(k)]).collect();
        let others: Vec<u8> = playable.iter().copied().filter(|k| !chosen.contains(k)).collect();
        chosen.extend(others.into_iter().take(DRUM_KEYS.len().saturating_sub(chosen.len())));
        return chosen.into_iter().enumerate().map(|(i, k)| (k, i as f32 * 0.35, 0.3)).collect();
    }
    let root = if lo <= 60 && hi >= 72 {
        60
    } else {
        let mid = i32::midpoint(i32::from(lo), i32::from(hi));
        let c = ((mid - 6) as f32 / 12.0).round() as i32 * 12;
        c.clamp(i32::from(lo), (i32::from(hi) - 12).max(i32::from(lo)))
    };
    PHRASE
        .iter()
        .filter_map(|&(step, start, len)| {
            let key = u8::try_from(root + step).ok()?;
            keys.get(usize::from(key)).copied().unwrap_or(false).then_some((key, start, len))
        })
        .collect()
}

/// "C4" for key 60.
fn note_name(key: u8) -> String {
    const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    format!("{}{}", NAMES[usize::from(key % 12)], i32::from(key / 12) - 1)
}

fn key_range(keys: &[bool; 128]) -> Option<String> {
    let lo = keys.iter().position(|&k| k)?;
    let hi = keys.iter().rposition(|&k| k)?;
    let name = |k: usize| note_name(u8::try_from(k).unwrap_or(127));
    Some(if lo == hi { name(lo) } else { format!("{}–{}", name(lo), name(hi)) })
}

fn frames_at(secs: f32) -> usize {
    (secs * RATE as f32) as usize
}

// --- SF2 ---

fn sf2(path: &Path) -> Result<Played> {
    let mut file = std::io::BufReader::new(std::fs::File::open(path)?);
    let sf = Arc::new(rustysynth::SoundFont::new(&mut file).map_err(|e| anyhow::anyhow!("not a SoundFont: {e:?}"))?);
    // The first melodic preset by bank and program (a piano, in a General MIDI font), or else the first kit.
    let preset = sf
        .get_presets()
        .iter()
        .min_by_key(|p| (p.get_bank_number() >= 128, p.get_bank_number(), p.get_patch_number()))
        .context("the SoundFont has no presets")?;
    let drums = preset.get_bank_number() >= 128;
    let mut keys = [false; 128];
    for pr in preset.get_regions() {
        let Some(instrument) = sf.get_instruments().get(pr.get_instrument_id()) else { continue };
        for ir in instrument.get_regions() {
            let lo = pr.get_key_range_start().max(ir.get_key_range_start()).max(0);
            let hi = pr.get_key_range_end().min(ir.get_key_range_end()).min(127);
            for k in lo..=hi {
                keys[k as usize] = true;
            }
        }
    }

    let settings = rustysynth::SynthesizerSettings::new(i32::try_from(RATE)?);
    let mut synth = rustysynth::Synthesizer::new(&sf, &settings).map_err(|e| anyhow::anyhow!("synthesizer: {e:?}"))?;
    // A kit plays on the drum channel, whose bank numbers start at 128.
    let channel = if drums { 9 } else { 0 };
    synth.process_midi_message(channel, 0xB0, 0, preset.get_bank_number() % 128);
    synth.process_midi_message(channel, 0xC0, preset.get_patch_number(), 0);
    let mut events: Vec<(usize, bool, i32)> = notes(&keys, drums)
        .into_iter()
        .flat_map(|(k, start, len)| {
            [(frames_at(start), true, i32::from(k)), (frames_at(start + len), false, i32::from(k))]
        })
        .collect();
    // Releases before presses at the same moment.
    events.sort_by_key(|&(at, on, _)| (at, on));
    let frames = frames_at(LENGTH);
    let (mut left, mut right) = (vec![0.0f32; frames], vec![0.0f32; frames]);
    let mut done = 0;
    for (at, on, key) in events.into_iter().chain([(frames, false, -1)]) {
        let at = at.min(frames);
        if at > done {
            synth.render(&mut left[done..at], &mut right[done..at]);
            done = at;
        }
        match (key, on) {
            (-1, _) => {}
            (k, true) => synth.note_on(channel, k, i32::from(VELOCITY)),
            (k, false) => synth.note_off(channel, k),
        }
    }

    let info = sf.get_info();
    let mut facts = Map::new();
    let name = info.get_bank_name().trim();
    if !name.is_empty() {
        facts.insert("title".into(), json!(name));
    }
    let author = info.get_author().trim();
    if !author.is_empty() {
        facts.insert("author".into(), json!(author));
    }
    facts.insert("presets".into(), json!(sf.get_presets().len()));
    facts.insert("instruments".into(), json!(sf.get_instruments().len()));
    facts.insert("samples".into(), json!(sf.get_sample_headers().len()));
    facts.insert("preset_played".into(), json!(preset.get_name().trim()));
    if let Some(range) = key_range(&keys) {
        facts.insert("key_range".into(), json!(range));
    }
    if drums {
        facts.insert("drum_kit".into(), json!(true));
    }
    Ok((facts, left, right))
}

// --- SFZ ---

/// One `<region>`, with what it inherits from the headers above it.
#[derive(Debug, Clone)]
struct Region {
    /// The sample's path, relative to the SFZ file's folder, or a built-in wave (`*sine`).
    sample: String,
    lo: u8,
    hi: u8,
    lovel: u8,
    hivel: u8,
    /// The key at which the sample plays at its own pitch.
    center: f32,
    /// How far the pitch follows the key: 1 for an instrument, 0 for a drum.
    keytrack: f32,
    /// Tuning in cents, `transpose` included.
    cents: f32,
    gain: f32,
    release: f32,
    offset: usize,
    looped: Option<(usize, usize)>,
    one_shot: bool,
    /// Plays on a key press: not a release sound, and first in a round robin.
    sounds: bool,
    /// The key switch this region needs (`sw_last`).
    switch: Option<u8>,
    switch_default: Option<u8>,
}

/// The samples an SFZ file plays, relative to its folder, for links.
pub(crate) fn sfz_samples(path: &Path) -> Result<Vec<String>> {
    let mut samples: Vec<String> =
        read_sfz(path)?.into_iter().map(|r| r.sample).filter(|s| !s.starts_with('*')).collect();
    samples.sort();
    samples.dedup();
    Ok(samples)
}

fn read_sfz(path: &Path) -> Result<Vec<Region>> {
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut items = Vec::new();
    tokens(&std::fs::read_to_string(path)?, dir, 0, &mut HashMap::new(), &mut items)?;
    Ok(regions(&items))
}

enum Item {
    Header(String),
    Op(String, String),
}

/// Headers and opcodes in order, with comments left out, defines put in and includes read.
fn tokens(
    text: &str,
    dir: &Path,
    depth: usize,
    defines: &mut HashMap<String, String>,
    out: &mut Vec<Item>,
) -> Result<()> {
    for line in strip_block_comments(text).lines() {
        let line = line.split("//").next().unwrap_or_default().trim();
        if let Some(rest) = line.strip_prefix("#define") {
            let mut parts = rest.trim().splitn(2, char::is_whitespace);
            if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
                defines.insert(name.to_owned(), value.trim().to_owned());
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("#include") {
            let file = rest.trim().trim_matches('"').replace('\\', "/");
            // Paths are relative to the main file; a missing include leaves its regions out.
            if depth < MAX_INCLUDES
                && let Ok(text) = std::fs::read_to_string(dir.join(&file))
            {
                tokens(&text, dir, depth + 1, defines, out)?;
            }
            continue;
        }
        let mut line = line.to_owned();
        if line.contains('$') {
            // Longest names first, so `$KEY` doesn't eat `$KEYS`.
            let mut names: Vec<&String> = defines.keys().collect();
            names.sort_by_key(|n| std::cmp::Reverse(n.len()));
            for name in names {
                line = line.replace(name.as_str(), &defines[name]);
            }
        }
        let mut rest = line.as_str();
        while let Some(open) = rest.find('<') {
            opcodes(&rest[..open], out);
            let close = rest[open..].find('>').map_or(rest.len(), |c| open + c);
            out.push(Item::Header(rest[open + 1..close].trim().to_lowercase()));
            rest = rest.get(close + 1..).unwrap_or_default();
        }
        opcodes(rest, out);
    }
    Ok(())
}

fn strip_block_comments(text: &str) -> std::borrow::Cow<'_, str> {
    if !text.contains("/*") {
        return text.into();
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        rest = rest[start + 2..].find("*/").map_or("", |end| &rest[start + 2 + end + 2..]);
    }
    out.push_str(rest);
    out.into()
}

/// `key=value` pairs; a value runs to the next key, so sample paths may have spaces.
fn opcodes(segment: &str, out: &mut Vec<Item>) {
    let is_key = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let key_start =
        |eq: usize| segment[..eq].char_indices().rev().find(|&(_, c)| !is_key(c)).map_or(0, |(i, c)| i + c.len_utf8());
    let eqs: Vec<usize> = segment.match_indices('=').map(|(i, _)| i).collect();
    for (n, &eq) in eqs.iter().enumerate() {
        let key = &segment[key_start(eq)..eq];
        let end = eqs.get(n + 1).map_or(segment.len(), |&next| key_start(next));
        if !key.is_empty() && end > eq {
            out.push(Item::Op(key.to_lowercase(), segment[eq + 1..end].trim().to_owned()));
        }
    }
}

/// Every region with the opcodes it inherits: `<control>`, `<global>`, `<master>` and `<group>` each hold until a
/// header of their level or above comes again.
fn regions(items: &[Item]) -> Vec<Region> {
    const LEVELS: [&str; 5] = ["control", "global", "master", "group", "region"];
    let mut levels: [Vec<(&str, &str)>; 5] = Default::default();
    let mut current: Option<usize> = None;
    let mut out = Vec::new();
    let mut finish = |levels: &[Vec<(&str, &str)>; 5], current: Option<usize>| {
        if current == Some(4)
            && let Some(r) = region(levels)
        {
            out.push(r);
        }
    };
    for item in items {
        match item {
            Item::Header(name) => {
                finish(&levels, current);
                current = LEVELS.iter().position(|l| l == name);
                if let Some(level) = current {
                    for l in &mut levels[level..] {
                        l.clear();
                    }
                }
            }
            Item::Op(k, v) => {
                if let Some(level) = current {
                    levels[level].push((k.as_str(), v.as_str()));
                }
            }
        }
    }
    finish(&levels, current);
    let default = out.iter().find_map(|r| r.switch_default);
    let switch = default.or_else(|| out.iter().filter_map(|r| r.switch).min());
    for r in &mut out {
        r.sounds &= r.switch.is_none() || r.switch == switch;
    }
    out
}

fn region(levels: &[Vec<(&str, &str)>; 5]) -> Option<Region> {
    let mut op: HashMap<&str, &str> = HashMap::new();
    for (k, v) in levels.iter().flatten() {
        op.insert(*k, *v);
    }
    let num = |k: &str| op.get(k).and_then(|v| v.parse::<f32>().ok());
    let key = |k: &str| op.get(k).and_then(|v| parse_key(v));
    let sample = op.get("sample")?.replace('\\', "/");
    let sample = if sample.starts_with('*') {
        sample
    } else {
        let base = op.get("default_path").map_or_default(|p| p.replace('\\', "/"));
        format!("{base}{sample}")
    };
    let (mut lo, mut hi, mut center) = (0, 127, 60.0);
    if let Some(k) = key("key") {
        (lo, hi, center) = (k, k, f32::from(k));
    }
    lo = key("lokey").unwrap_or(lo);
    hi = key("hikey").unwrap_or(hi);
    center = key("pitch_keycenter").map_or(center, f32::from);
    let loop_mode = op.get("loop_mode").copied().unwrap_or("no_loop");
    let looped = match (loop_mode, num("loop_start"), num("loop_end")) {
        ("loop_continuous" | "loop_sustain", Some(s), Some(e)) if e > s => Some((s as usize, e as usize)),
        _ => None,
    };
    let first = num("seq_position").is_none_or(|p| p <= 1.0) && num("lorand").is_none_or(|r| r <= 0.0);
    let attack = op.get("trigger").is_none_or(|t| *t == "attack" || *t == "first");
    Some(Region {
        sample,
        lo,
        hi,
        lovel: num("lovel").map_or(1, |v| v.clamp(0.0, 127.0) as u8),
        hivel: num("hivel").map_or(127, |v| v.clamp(0.0, 127.0) as u8),
        center,
        keytrack: num("pitch_keytrack").unwrap_or(100.0) / 100.0,
        cents: num("tune").unwrap_or(0.0) + num("transpose").unwrap_or(0.0) * 100.0,
        gain: 10f32.powf(num("volume").unwrap_or(0.0) / 20.0),
        release: num("ampeg_release").unwrap_or(0.0).clamp(0.03, 2.0),
        offset: num("offset").map_or(0, |o| o.max(0.0) as usize),
        looped,
        one_shot: loop_mode == "one_shot",
        sounds: first && attack,
        switch: key("sw_last"),
        switch_default: key("sw_default"),
    })
}

/// A key as a number or a note name (`c4` is 60, `f#2`, `eb-1`).
fn parse_key(v: &str) -> Option<u8> {
    if let Ok(n) = v.parse::<i32>() {
        return u8::try_from(n).ok().filter(|&k| k < 128);
    }
    let mut chars = v.chars();
    let base = match chars.next()?.to_ascii_lowercase() {
        'c' => 0,
        'd' => 2,
        'e' => 4,
        'f' => 5,
        'g' => 7,
        'a' => 9,
        'b' => 11,
        _ => return None,
    };
    let rest = chars.as_str();
    let (shift, octave) = match rest.chars().next()? {
        '#' | '♯' => (1, &rest[rest.chars().next()?.len_utf8()..]),
        'b' | '♭' => (-1, &rest[rest.chars().next()?.len_utf8()..]),
        _ => (0, rest),
    };
    let key = (octave.parse::<i32>().ok()? + 1) * 12 + base + shift;
    u8::try_from(key).ok().filter(|&k| k < 128)
}

/// A decoded sample, in stereo.
struct Sample {
    left: Vec<f32>,
    right: Vec<f32>,
    rate: u32,
}

fn load_sample(dir: &Path, name: &str) -> Option<Sample> {
    if let Some(wave) = name.strip_prefix('*') {
        return Some(builtin_wave(wave));
    }
    let path = resolve(dir, name)?;
    let ext = path.extension().and_then(|e| e.to_str()).map(str::to_lowercase);
    let (mut left, mut right, mut rate) = (Vec::new(), Vec::new(), 0);
    crate::audio::decode_each(Box::new(std::fs::File::open(&path).ok()?), ext.as_deref(), |pcm, c, r, _| {
        rate = r;
        let c = usize::from(c.max(1));
        for frame in pcm.chunks_exact(c) {
            left.push(frame[0]);
            right.push(frame[c.min(2) - 1]);
        }
        Ok(left.len() < (r * MAX_SAMPLE_SECS) as usize)
    })
    .ok()?;
    (rate > 0 && !left.is_empty()).then_some(Sample { left, right, rate })
}

/// The sample's file: as written, or matched without regard to case (SFZ files come from Windows).
fn resolve(dir: &Path, name: &str) -> Option<PathBuf> {
    let direct = dir.join(name);
    if direct.is_file() {
        return Some(direct);
    }
    let mut at = dir.to_path_buf();
    for part in name.split('/').filter(|p| !p.is_empty() && *p != ".") {
        if part == ".." {
            at.pop();
            continue;
        }
        let lower = part.to_lowercase();
        at = std::fs::read_dir(&at)
            .ok()?
            .filter_map(Result::ok)
            .find(|e| e.file_name().to_string_lossy().to_lowercase() == lower)?
            .path();
    }
    at.is_file().then_some(at)
}

/// SFZ's built-in oscillators at middle C's pitch, as long as the preview.
fn builtin_wave(name: &str) -> Sample {
    let freq = 440.0 * 2f32.powf((60.0 - 69.0) / 12.0);
    let period = RATE as f32 / freq;
    let mut noise = 0x1234_5678u32;
    let wave: Vec<f32> = (0..frames_at(LENGTH))
        .map(|i| {
            let phase = (i as f32 / period).fract();
            match name {
                "sine" => (std::f32::consts::TAU * phase).sin(),
                "saw" => 2.0 * phase - 1.0,
                "square" => {
                    if phase < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                "triangle" => 1.0 - 4.0 * (phase - 0.5).abs(),
                "noise" => {
                    noise ^= noise << 13;
                    noise ^= noise >> 17;
                    noise ^= noise << 5;
                    noise as f32 / u32::MAX as f32 * 2.0 - 1.0
                }
                _ => 0.0,
            }
        })
        .collect();
    Sample { left: wave.clone(), right: wave, rate: RATE }
}

fn sfz(path: &Path) -> Result<Played> {
    let regions = read_sfz(path)?;
    ensure!(!regions.is_empty(), "the SFZ file has no regions");
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut cache: HashMap<&str, Option<Sample>> = HashMap::new();
    for r in &regions {
        cache.entry(&r.sample).or_insert_with(|| load_sample(dir, &r.sample));
    }
    let missing = cache.iter().filter(|(name, s)| s.is_none() && !name.starts_with('*')).count();
    ensure!(cache.values().any(Option::is_some), "the samples this SFZ file plays aren't beside it");

    let playing: Vec<&Region> = regions.iter().filter(|r| r.sounds && cache[r.sample.as_str()].is_some()).collect();
    let mut keys = [false; 128];
    for r in &playing {
        for k in r.lo..=r.hi.min(127) {
            keys[usize::from(k)] = true;
        }
    }
    let drums = playing.iter().all(|r| r.keytrack == 0.0) || playing.iter().all(|r| r.lo == r.hi && r.one_shot);
    let frames = frames_at(LENGTH);
    let (mut left, mut right) = (vec![0.0f32; frames], vec![0.0f32; frames]);
    for (key, start, len) in notes(&keys, drums) {
        for r in playing.iter().filter(|r| (r.lo..=r.hi).contains(&key) && (r.lovel..=r.hivel).contains(&VELOCITY)) {
            if let Some(sample) = &cache[r.sample.as_str()] {
                play(sample, r, key, frames_at(start), len, &mut left, &mut right);
            }
        }
    }

    let mut facts = Map::new();
    facts.insert("regions".into(), json!(regions.len()));
    facts.insert("samples".into(), json!(cache.keys().filter(|n| !n.starts_with('*')).count()));
    if missing > 0 {
        facts.insert("samples_missing".into(), json!(missing));
    }
    if let Some(range) = key_range(&keys) {
        facts.insert("key_range".into(), json!(range));
    }
    let layers = {
        let mut v: Vec<(u8, u8)> = playing.iter().map(|r| (r.lovel, r.hivel)).collect();
        v.sort_unstable();
        v.dedup();
        v.len()
    };
    if layers > 1 {
        facts.insert("velocity_layers".into(), json!(layers));
    }
    if drums {
        facts.insert("drum_kit".into(), json!(true));
    }
    Ok((facts, left, right))
}

/// Mix one note of `r` into the output: the sample resampled to the key's pitch, held for `len` seconds (to its
/// end for a one-shot), then faded over the release.
fn play(sample: &Sample, r: &Region, key: u8, start: usize, len: f32, left: &mut [f32], right: &mut [f32]) {
    let semis = (f32::from(key) - r.center) * r.keytrack + r.cents / 100.0;
    let step = f64::from(2f32.powf(semis / 12.0)) * f64::from(sample.rate) / f64::from(RATE);
    let velocity = f32::from(VELOCITY) / 127.0;
    let gain = r.gain * velocity * velocity;
    let held = frames_at(len);
    let release = frames_at(r.release);
    let attack = frames_at(0.002).max(1);
    let end = sample.left.len();
    let mut pos = r.offset as f64;
    for i in 0..left.len().saturating_sub(start) {
        if r.looped.is_none() && pos as usize + 1 >= end {
            break;
        }
        let env = if !r.one_shot && i > held {
            let t = (i - held) as f32 / release as f32;
            if t >= 1.0 {
                break;
            }
            1.0 - t
        } else {
            1.0
        };
        let env = env * (i as f32 / attack as f32).min(1.0);
        let at = pos as usize;
        let frac = (pos - at as f64) as f32;
        let next = (at + 1).min(end - 1);
        let (l, rr) = (
            sample.left[at.min(end - 1)] * (1.0 - frac) + sample.left[next] * frac,
            sample.right[at.min(end - 1)] * (1.0 - frac) + sample.right[next] * frac,
        );
        left[start + i] += l * gain * env;
        right[start + i] += rr * gain * env;
        pos += step;
        if let Some((ls, le)) = r.looped {
            let le = le.min(end - 1);
            if le > ls && pos >= le as f64 {
                pos -= (le - ls) as f64;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav(dir: &Path, name: &str, freq: f32) {
        let samples: Vec<i32> = (0..RATE / 2)
            .map(|i| ((std::f32::consts::TAU * freq * i as f32 / RATE as f32).sin() * 20_000.0) as i32)
            .collect();
        let audio = Audio { samples, channels: 1, rate: RATE, bits: 16 };
        std::fs::write(dir.join(name), crate::audio::write_wav(&audio)).unwrap();
    }

    #[test]
    fn reads_keys_and_note_names() {
        assert_eq!(parse_key("c4"), Some(60));
        assert_eq!(parse_key("C#4"), Some(61));
        assert_eq!(parse_key("eb-1"), Some(3));
        assert_eq!(parse_key("72"), Some(72));
        assert_eq!(parse_key("x"), None);
        assert_eq!(note_name(60), "C4");
        assert_eq!(note_name(21), "A0");
    }

    #[test]
    fn regions_inherit_and_defines_apply() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("more.sfz"), "<region> sample=high note.wav lokey=c5 hikey=127\n").unwrap();
        let sfz = dir.path().join("piano.sfz");
        std::fs::write(
            &sfz,
            "/* a piano */\n#define $LOW 48\n<control> default_path=Samples\\\n<global> volume=-6 // quieter\n\
             <group> lovel=1 hivel=127\n<region> sample=low.wav lokey=0 hikey=$LOW pitch_keycenter=36\n\
             <region> sample=mid.wav lokey=49 hikey=71 pitch_keycenter=c4 trigger=release\n#include \"more.sfz\"\n",
        )
        .unwrap();
        let regions = read_sfz(&sfz).unwrap();
        assert_eq!(regions.len(), 3);
        assert_eq!(regions[0].sample, "Samples/low.wav");
        assert_eq!((regions[0].lo, regions[0].hi, regions[0].center), (0, 48, 36.0));
        assert!((regions[0].gain - 0.501).abs() < 0.01);
        assert!(!regions[1].sounds);
        assert_eq!(regions[1].center, 60.0);
        assert_eq!(regions[2].sample, "Samples/high note.wav");
        assert_eq!(regions[2].lo, 72);
        assert_eq!(sfz_samples(&sfz).unwrap(), ["Samples/high note.wav", "Samples/low.wav", "Samples/mid.wav"]);
    }

    #[test]
    fn plays_an_sfz_on_its_samples() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("samples")).unwrap();
        wav(&dir.path().join("samples"), "A3.wav", 220.0);
        let sfz = dir.path().join("synth.sfz");
        std::fs::write(&sfz, "<region> sample=Samples/a3.wav pitch_keycenter=57 lokey=36 hikey=84 ampeg_release=0.2\n<region> sample=gone.wav key=30\n")
            .unwrap();
        let (facts, audio) = analyze(&sfz, "sfz").unwrap();
        assert_eq!(facts["key_range"], "C2–C6");
        assert_eq!(facts["samples"], 2);
        assert_eq!(facts["samples_missing"], 1);
        assert_eq!(audio.samples.len(), frames_at(LENGTH) * 2);
        // The first note (middle C) sounds, and the end is quiet.
        let early = audio.samples[1000..5000].iter().map(|s| s.abs()).max().unwrap();
        assert!(early > 3_000, "{early}");
        assert!(audio.samples[audio.samples.len() - 100..].iter().all(|s| s.abs() < 100));
        assert!(analyze(&dir.path().join("nothing.sfz"), "sfz").is_err());
    }

    /// A SoundFont with one preset ("Lead", bank 0, program 0) of one instrument playing a looped sine over keys
    /// 48–84.
    fn sf2() -> Vec<u8> {
        fn chunk(id: &[u8], body: &[u8]) -> Vec<u8> {
            let mut c = id.to_vec();
            c.extend((body.len() as u32).to_le_bytes());
            c.extend(body);
            if body.len() % 2 == 1 {
                c.push(0);
            }
            c
        }
        fn list(kind: &[u8], chunks: &[Vec<u8>]) -> Vec<u8> {
            let mut body = kind.to_vec();
            for c in chunks {
                body.extend(c);
            }
            chunk(b"LIST", &body)
        }
        fn name(n: &str) -> Vec<u8> {
            let mut b = n.as_bytes().to_vec();
            b.resize(20, 0);
            b
        }
        let u16s = |v: &[u16]| v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
        // 100 cycles of a sine at 441 Hz, then the 46 zeros the format wants after a sample.
        let len = 10_000u32;
        let mut smpl: Vec<u8> = (0..len)
            .flat_map(|i| (((i as f32 / 100.0 * std::f32::consts::TAU).sin() * 20_000.0) as i16).to_le_bytes())
            .collect();
        smpl.extend([0; 92]);
        let mut phdr = name("Lead");
        phdr.extend(u16s(&[0, 0, 0]));
        phdr.extend([0; 12]);
        phdr.extend(name("EOP"));
        phdr.extend(u16s(&[0, 0, 1]));
        phdr.extend([0; 12]);
        let mut inst = name("Sine");
        inst.extend(u16s(&[0]));
        inst.extend(name("EOI"));
        inst.extend(u16s(&[1]));
        let mut shdr = name("sine");
        for v in [0, len, 100, 9_900, RATE] {
            shdr.extend(v.to_le_bytes());
        }
        shdr.extend([69, 0]);
        shdr.extend(u16s(&[0, 1]));
        shdr.extend(name("EOS"));
        shdr.extend([0; 26]);
        let info = list(b"INFO", &[chunk(b"ifil", &u16s(&[2, 1])), chunk(b"INAM", b"Tiny Font\0")]);
        let sdta = list(b"sdta", &[chunk(b"smpl", &smpl)]);
        let pdta = list(
            b"pdta",
            &[
                chunk(b"phdr", &phdr),
                chunk(b"pbag", &u16s(&[0, 0, 1, 0])),
                chunk(b"pmod", &[0; 10]),
                // The instrument, then the end.
                chunk(b"pgen", &u16s(&[41, 0, 0, 0])),
                chunk(b"inst", &inst),
                chunk(b"ibag", &u16s(&[0, 0, 4, 0])),
                chunk(b"imod", &[0; 10]),
                // Keys 48–84, looping, root key 69, the sample, then the end.
                chunk(b"igen", &u16s(&[43, 0x30 | (0x54 << 8), 54, 1, 58, 69, 53, 0, 0, 0])),
                chunk(b"shdr", &shdr),
            ],
        );
        let mut body = b"sfbk".to_vec();
        for part in [info, sdta, pdta] {
            body.extend(part);
        }
        chunk(b"RIFF", &body)
    }

    #[test]
    fn plays_a_soundfont() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tiny.sf2");
        std::fs::write(&path, sf2()).unwrap();
        let (facts, audio) = analyze(&path, "sf2").unwrap();
        assert_eq!(facts["title"], "Tiny Font");
        assert_eq!(facts["presets"], 1);
        assert_eq!(facts["preset_played"], "Lead");
        assert_eq!(facts["key_range"], "C3–C6");
        assert!(audio.samples[1000..5000].iter().any(|s| s.abs() > 3_000));
        std::fs::write(&path, b"RIFF\0\0\0\0junk").unwrap();
        assert!(analyze(&path, "sf2").is_err());
    }

    #[test]
    fn plays_a_kit_by_its_keys() {
        let mut keys = [false; 128];
        for k in [36, 38, 40, 42] {
            keys[k] = true;
        }
        let played: Vec<u8> = notes(&keys, true).iter().map(|n| n.0).collect();
        assert_eq!(played, [36, 38, 42, 40]);
        // A bass: the phrase moves down to its range.
        let mut keys = [false; 128];
        keys[28..=52].fill(true);
        assert_eq!(notes(&keys, false)[0].0, 36);
    }
}
