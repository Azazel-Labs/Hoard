//! Finding sounds by a word that imitates them: "WHOOOM", "pew", "krssh". The word is read as the shape of a sound,
//! the way its letters would be said: a plosive start (b, p, k, t) is a sharp attack and an airy one (wh, sh, s, f, h)
//! a soft one; drawn-out letters are length; "oo" and "u" are low and dark, "ee" and "i" high and bright, and vowels
//! that move from one to the other are a pitch that rises or falls; hiss and air (s, sh, f, h) are noise; a hum at the
//! end (m, n, ng) rings on, a stop (k, t, p) cuts off; capitals are loud. Every sound's measured features (see
//! `sounds`) are scored against that shape. Nothing here is learned; it is a reading of English-style spelling.
//!
//! The noise can be made into a microphone instead ([`hear`]): the recording is measured the way every sound is, and
//! what it measured is the shape. It travels as `imitation` like a word, as [`HEARD`] and the shape's numbers.

use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{AppState, db};
use anyhow::Result;
use axum::Json;
use serde_json::{Value, json};

/// What a word says about the sound it imitates, or what someone says it's like; nothing where it says nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Shape {
    /// About how long, in seconds.
    pub(crate) secs: Option<f32>,
    /// A sharp attack (`true`) or a soft one; nothing for a start that says neither (a vowel, a hum).
    pub(crate) sharp: Option<bool>,
    /// How bright its sounds average to, in Hz.
    pub(crate) brightness_hz: Option<f32>,
    /// How much of it is hiss and air, as `sounds` measures noisiness (0 a pure tone, 1 white noise).
    pub(crate) noisiness: Option<f32>,
    /// How far its pitch moves, in octaves: vowels that move from dark to bright rise; anything else is steady.
    pub(crate) sweep_octaves: Option<f32>,
    /// Rings on at the end (`true`) or cuts off.
    pub(crate) rings: Option<bool>,
    /// Written in capitals.
    pub(crate) loud: bool,
}

impl Shape {
    /// Says nothing about a sound.
    pub(crate) fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

/// One sound of the word.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Part {
    /// From 0 ("oo") to 4 ("ee"), dark to bright.
    Vowel(u8),
    /// A plosive, at about this brightness.
    Stop(f32),
    /// A hiss (s, sh, f), at about this brightness.
    Hiss(f32),
    /// Breath (h, wh).
    Air,
    /// m, n, ng.
    Hum,
    /// r, l.
    Liquid,
}

/// Brightness of each vowel, dark to bright: oo, o/u, a, e, ee.
const VOWEL_HZ: [f32; 5] = [450.0, 800.0, 1500.0, 2600.0, 4200.0];

/// The word's sounds, each with how long it is drawn out (in beats: "ooo" is two beats of "oo").
fn parts(word: &str) -> Vec<(Part, f32)> {
    let w: Vec<char> = word.chars().filter(char::is_ascii_alphabetic).map(|c| c.to_ascii_lowercase()).collect();
    let mut out: Vec<(Part, f32)> = Vec::new();
    let mut i = 0;
    while i < w.len() {
        let c = w[i];
        let next = w.get(i + 1).copied();
        // Digraphs, drawn out by repeating their second letter ("shhh").
        let digraph = match (c, next) {
            ('s' | 'c', Some('h')) => Some(Part::Hiss(3500.0)),
            ('t' | 'p', Some('h')) => Some(Part::Hiss(2500.0)),
            ('w', Some('h')) => Some(Part::Air),
            ('n', Some('g')) => Some(Part::Hum),
            ('c', Some('k')) => Some(Part::Stop(3000.0)),
            _ => None,
        };
        if let Some(part) = digraph {
            let second = w[i + 1];
            let run = w[i + 1..].iter().take_while(|&&x| x == second).count();
            if c == 'c' && second == 'h' {
                out.push((Part::Stop(3000.0), 1.0));
            }
            out.push((part, if part == Part::Hum || matches!(part, Part::Stop(_)) { 1.0 } else { run as f32 }));
            i += 1 + run;
            continue;
        }
        let run = w[i..].iter().take_while(|&&x| x == c).count();
        let beats = run as f32;
        match c {
            // "oo" and "ee" are their own vowels; more letters draw them out.
            'o' | 'u' if run >= 2 => out.push((Part::Vowel(0), beats - 1.0)),
            'e' if run >= 2 => out.push((Part::Vowel(4), beats - 1.0)),
            'o' | 'u' => out.push((Part::Vowel(1), beats)),
            'a' => out.push((Part::Vowel(2), beats)),
            'e' => out.push((Part::Vowel(3), beats)),
            'i' | 'y' => out.push((Part::Vowel(4), beats)),
            // A w glides through "oo": "pew" falls, "bwee" rises.
            'w' => out.push((Part::Vowel(0), beats)),
            'r' | 'l' => out.push((Part::Liquid, beats)),
            'm' | 'n' => out.push((Part::Hum, beats)),
            's' | 'z' => out.push((Part::Hiss(7000.0), beats)),
            'f' | 'v' => out.push((Part::Hiss(2500.0), beats)),
            'h' => out.push((Part::Air, beats)),
            'x' => {
                out.push((Part::Stop(3000.0), 1.0));
                out.push((Part::Hiss(6000.0), beats));
            }
            'j' => {
                out.push((Part::Stop(1500.0), 1.0));
                out.push((Part::Hiss(3500.0), 1.0));
            }
            'b' | 'd' => out.push((Part::Stop(600.0), 1.0)),
            'g' => out.push((Part::Stop(800.0), 1.0)),
            'p' => out.push((Part::Stop(1500.0), 1.0)),
            _ => out.push((Part::Stop(3000.0), 1.0)), // t, k, c, q
        }
        i += run;
    }
    out
}

/// Read a word as the shape of a sound; nothing when it has no letters to read.
pub(crate) fn read(word: &str) -> Option<Shape> {
    let parts = parts(word);
    let (&(first, _), &(last, _)) = (parts.first()?, parts.last()?);

    // Length: drawn-out sounds double it a beat at a time, each syllable adds to it, and how it ends sets where it
    // starts from (a hum fades on, a stop is over at once).
    let drawn: f32 = parts.iter().filter(|(p, _)| !matches!(p, Part::Stop(_))).map(|(_, b)| b - 1.0).sum();
    let syllables = parts
        .iter()
        .zip(std::iter::once(None).chain(parts.iter().map(Some)))
        .filter(|((p, _), prev)| matches!(p, Part::Vowel(_)) && !matches!(prev, Some((Part::Vowel(_), _))))
        .count()
        .max(1);
    let base = match last {
        Part::Hum => 0.5,
        Part::Stop(_) => 0.15,
        _ => 0.3,
    };
    let secs = (base * 1.5f32.powf(drawn.min(8.0)) * (syllables as f32).powf(0.8)).clamp(0.05, 20.0);

    // Brightness and noise: each sound by how long it lasts, a plosive's burst counting for half a beat.
    let (mut log_hz, mut noise, mut weight) = (0.0f32, 0.0f32, 0.0f32);
    for &(p, beats) in &parts {
        let (hz, w, hissing) = match p {
            Part::Vowel(v) => (VOWEL_HZ[usize::from(v)], beats, 0.0),
            Part::Stop(hz) => (hz, 0.5, 0.3),
            Part::Hiss(hz) => (hz, beats, 1.0),
            Part::Air => (2500.0, 1.5 * beats, 1.0),
            Part::Hum => (300.0, beats, 0.0),
            Part::Liquid => (900.0, beats, 0.2),
        };
        log_hz += w * hz.log2();
        noise += w * hissing;
        weight += w;
    }

    let vowels: Vec<u8> =
        parts.iter().filter_map(|(p, _)| if let Part::Vowel(v) = p { Some(*v) } else { None }).collect();
    let sweep_octaves = match (vowels.first(), vowels.last()) {
        (Some(&a), Some(&b)) => 0.6 * (f32::from(b) - f32::from(a)),
        _ => 0.0,
    };
    let letters: Vec<char> = word.chars().filter(char::is_ascii_alphabetic).collect();
    let upper = letters.iter().filter(|c| c.is_ascii_uppercase()).count();
    Some(Shape {
        secs: Some(secs),
        sharp: match first {
            Part::Stop(_) => Some(true),
            // Breath fades in; a hiss can start at full ("shhh" is rain as much as a swell).
            Part::Air | Part::Hiss(2500.0) => Some(false),
            _ => None,
        },
        brightness_hz: Some((log_hz / weight).exp2()),
        noisiness: Some(0.12 + 0.6 * noise / weight),
        sweep_octaves: Some(sweep_octaves),
        rings: match last {
            Part::Hum => Some(true),
            Part::Stop(_) => Some(false),
            _ => None,
        },
        loud: letters.len() >= 2 && upper * 5 >= letters.len() * 3,
    })
}

/// What `sounds` measured of a sound, as far as scoring needs.
#[derive(Debug, Clone, Default)]
pub(crate) struct Measured {
    pub(crate) secs: f32,
    pub(crate) attack_ms: Option<f32>,
    pub(crate) tail_ms: Option<f32>,
    pub(crate) brightness_hz: f32,
    pub(crate) noisiness: f32,
    pub(crate) sweep_octaves: Option<f32>,
    pub(crate) rms_dbfs: Option<f32>,
}

impl Measured {
    #[cfg(test)]
    pub(crate) fn from_meta(m: &serde_json::Map<String, Value>) -> Option<Self> {
        let n = |k: &str| m.get(k).and_then(Value::as_f64).map(|v| v as f32);
        Some(Self {
            secs: n("duration_secs")?,
            attack_ms: n("attack_ms"),
            tail_ms: n("tail_ms"),
            brightness_hz: n("brightness_hz")?,
            noisiness: n("noisiness")?,
            sweep_octaves: n("sweep_octaves"),
            rms_dbfs: n("rms_dbfs"),
        })
    }
}

/// 1 at `target`, falling away as a bell curve `sigma` wide.
fn near(x: f32, target: f32, sigma: f32) -> f32 {
    (-((x - target) / sigma).powi(2) / 2.0).exp()
}

/// 1 at `limit` or beyond it (above it with `above`), falling away by octaves the other side.
fn past(x: f32, limit: f32, above: bool) -> f32 {
    let octaves = (x.max(1.0) / limit).log2();
    if (octaves >= 0.0) == above { 1.0 } else { near(octaves, 0.0, 1.5) }
}

/// How well a sound fits the shape, 0 to 1, over what the shape says.
pub(crate) fn score(shape: &Shape, m: &Measured) -> f32 {
    let mut fits: Vec<(f32, f32)> = Vec::new();
    if let Some(secs) = shape.secs {
        fits.push((1.0, near(m.secs.max(0.01).log2(), secs.max(0.01).log2(), 1.2)));
    }
    if let Some(hz) = shape.brightness_hz {
        fits.push((1.0, near(m.brightness_hz.max(20.0).log2(), hz.max(20.0).log2(), 1.3)));
    }
    if let Some(noisiness) = shape.noisiness {
        fits.push((0.8, near(m.noisiness, noisiness, 0.18)));
    }
    if let (Some(sharp), Some(attack)) = (shape.sharp, m.attack_ms) {
        fits.push((1.0, if sharp { past(attack, 15.0, false) } else { past(attack, 60.0, true) }));
    }
    if let (Some(rings), Some(tail)) = (shape.rings, m.tail_ms) {
        fits.push((0.7, if rings { past(tail, 150.0, true) } else { past(tail, 80.0, false) }));
    }
    if let Some(sweep) = shape.sweep_octaves {
        let measured = m.sweep_octaves.unwrap_or(0.0);
        // A word that stays put says less than vowels that move.
        fits.push(if sweep == 0.0 { (0.5, near(measured, 0.0, 1.0)) } else { (1.0, near(measured, sweep, 0.8)) });
    }
    if shape.loud
        && let Some(rms) = m.rms_dbfs
    {
        fits.push((0.4, if rms >= -18.0 { 1.0 } else { near(rms, -18.0, 8.0) }));
    }
    let weight: f32 = fits.iter().map(|f| f.0).sum();
    if weight == 0.0 {
        return 0.0;
    }
    fits.iter().map(|(w, f)| w * f).sum::<f32>() / weight
}

/// Sounds scoring below this don't match at all.
pub(crate) const FLOOR: f32 = 0.3;
/// At most this many sounds match.
const MAX: usize = 2000;

/// Every measured sound's file that fits the word (or a recording's shape, as [`Shape::token`] gives it), with how
/// well, best first.
pub(crate) async fn rank(state: &AppState, word: &str) -> Result<Vec<(String, f32)>> {
    let shape = match word.strip_prefix(HEARD) {
        Some(numbers) => Shape::from_token(numbers),
        None => read(word),
    };
    match shape {
        Some(shape) => rank_shape(state, shape).await,
        None => Ok(Vec::new()),
    }
}

/// Every measured sound's file that fits the shape, with how well, best first.
pub(crate) async fn rank_shape(state: &AppState, shape: Shape) -> Result<Vec<(String, f32)>> {
    if shape.is_empty() {
        return Ok(Vec::new());
    }
    db::call(&state.pool, move |c| {
        let mut stmt = c.prepare(
            "SELECT a.blob_hash, json_extract(a.meta, '$.duration_secs'), json_extract(a.meta, '$.attack_ms'),
                    json_extract(a.meta, '$.tail_ms'), json_extract(a.meta, '$.brightness_hz'),
                    json_extract(a.meta, '$.noisiness'), json_extract(a.meta, '$.sweep_octaves'),
                    json_extract(a.meta, '$.rms_dbfs')
               FROM assets a
              WHERE a.kind = 'audio' AND json_extract(a.meta, '$.brightness_hz') IS NOT NULL
              GROUP BY a.blob_hash",
        )?;
        let f = |v: Option<f64>| v.map(|v| v as f32);
        let mut hits: Vec<(String, f32)> = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    Measured {
                        secs: f(r.get(1)?).unwrap_or(1.0),
                        attack_ms: f(r.get(2)?),
                        tail_ms: f(r.get(3)?),
                        brightness_hz: f(r.get(4)?).unwrap_or(1000.0),
                        noisiness: f(r.get(5)?).unwrap_or(0.3),
                        sweep_octaves: f(r.get(6)?),
                        rms_dbfs: f(r.get(7)?),
                    },
                ))
            })?
            .filter_map(rusqlite::Result::ok)
            .map(|(hash, m)| (hash, score(&shape, &m)))
            .filter(|(_, s)| *s >= FLOOR)
            .collect();
        hits.sort_by(|a, b| b.1.total_cmp(&a.1));
        hits.truncate(MAX);
        Ok(hits)
    })
    .await
}

// ---------------------------------------------------------------------------------------------
// A recording

/// What a recording's shape starts with as `imitation`.
pub(crate) const HEARD: &str = "heard:";
/// How much of a recording is listened to.
const MAX_RECORDING_SECS: f32 = 10.0;
/// The largest recording taken: ten seconds of 48 kHz 16-bit mono WAV, and some.
pub(crate) const MAX_RECORDING_BYTES: usize = 2 << 20;

/// The shape of a noise made into a microphone: how long it is without the room's noise either side, how sharply it
/// starts and whether it rings on, and how bright, noisy and sweeping it is, as `sounds` measures every sound. How
/// loud it was says nothing (that's how near the microphone was). Nothing when nothing stands out from the room.
pub(crate) fn hear(bytes: bytes::Bytes) -> Option<Shape> {
    let (mut mono, rate, _) = crate::sounds::decode_mono(bytes, None)?;
    let sr = rate as f32;
    mono.truncate((MAX_RECORDING_SECS * sr) as usize);
    // How loud it is every 10 ms. The room is the level a twentieth of the recording is quieter than; the noise is
    // what is well above it.
    let step = ((sr * 0.01) as usize).max(1);
    let env: Vec<f32> =
        mono.chunks(step).map(|c| (c.iter().map(|s| s * s).sum::<f32>() / c.len() as f32).sqrt()).collect();
    let mut sorted = env.clone();
    sorted.sort_by(f32::total_cmp);
    let (room, top) = (sorted[sorted.len() / 20], *sorted.last()?);
    if top < 1e-3 || top < room * 3.0 {
        return None;
    }
    let gate = (room * 4.0).max(top * 0.05);
    let first = env.iter().position(|&e| e > gate)?;
    let last = env.iter().rposition(|&e| e > gate)? + 1;
    let noise = &mono[first * step..(last * step).min(mono.len())];
    let f = crate::sounds::listen(noise, rate)?;
    let n = |k: &str| f.get(k).and_then(Value::as_f64).map(|v| v as f32);
    let (attack, tail) = (n("attack_ms")?, n("tail_ms")?);
    Some(Shape {
        secs: Some(noise.len() as f32 / sr),
        // What `score` takes for sharp and soft, ringing and cut off, with room between for neither.
        sharp: if attack <= 30.0 { Some(true) } else { (attack >= 80.0).then_some(false) },
        rings: if tail >= 150.0 { Some(true) } else { (tail <= 60.0).then_some(false) },
        brightness_hz: n("brightness_hz"),
        noisiness: n("noisiness"),
        sweep_octaves: Some(n("sweep_octaves").unwrap_or(0.0)),
        loud: false,
    })
}

impl Shape {
    /// As `imitation` carries it: [`HEARD`] then its numbers, `-` for what it doesn't say.
    pub(crate) fn token(&self) -> String {
        fn num(v: Option<f32>, places: usize) -> String {
            v.map_or_else(|| "-".into(), |v| format!("{v:.places$}"))
        }
        let flag = |v: Option<bool>, yes: &str, no: &str| v.map_or("-", |v| if v { yes } else { no }).to_owned();
        let parts = [
            num(self.secs, 2),
            flag(self.sharp, "s", "f"),
            num(self.brightness_hz, 0),
            num(self.noisiness, 2),
            num(self.sweep_octaves, 1),
            flag(self.rings, "r", "c"),
            flag(Some(self.loud), "l", "q"),
        ];
        format!("{HEARD}{}", parts.join(","))
    }

    /// The shape [`Shape::token`] gave, from its numbers (after [`HEARD`]); nothing if they aren't its.
    fn from_token(numbers: &str) -> Option<Self> {
        let p: Vec<&str> = numbers.split(',').collect();
        let [secs, sharp, hz, noise, sweep, rings, loud] = p[..] else { return None };
        let num = |v: &str, lo: f32, hi: f32| -> Option<Option<f32>> {
            if v == "-" {
                return Some(None);
            }
            let x: f32 = v.parse().ok()?;
            (lo..=hi).contains(&x).then_some(Some(x))
        };
        let flag = |v: &str, yes: &str, no: &str| -> Option<Option<bool>> {
            match v {
                "-" => Some(None),
                _ if v == yes => Some(Some(true)),
                _ if v == no => Some(Some(false)),
                _ => None,
            }
        };
        Some(Self {
            secs: num(secs, 0.01, 600.0)?,
            sharp: flag(sharp, "s", "f")?,
            brightness_hz: num(hz, 20.0, 24_000.0)?,
            noisiness: num(noise, 0.0, 1.0)?,
            sweep_octaves: num(sweep, -10.0, 10.0)?,
            rings: flag(rings, "r", "c")?,
            loud: flag(loud, "l", "q")?.unwrap_or(false),
        })
    }
}

/// `POST /api/imitation`: a recording of someone making a noise (WAV), read as the shape of a sound. Answers
/// `{imitation, seconds}`: what `imitation=` takes to find sounds shaped like it, and how long the noise was.
pub(crate) async fn heard_route(body: axum::body::Bytes) -> ApiResult<Json<Value>> {
    if body.is_empty() {
        return Err(AppError::bad_request(tr!("error-empty-file")));
    }
    let shape = tokio::task::spawn_blocking(move || hear(body))
        .await?
        .ok_or_else(|| AppError::bad_request(tr!("error-imitation-silent")))?;
    Ok(Json(json!({ "imitation": shape.token(), "seconds": shape.secs })))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A WAV file of mono samples at `rate`.
    fn wav(samples: &[f32], rate: u32) -> bytes::Bytes {
        let mut out = Vec::new();
        let data = (samples.len() * 2) as u32;
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&(36 + data).to_le_bytes());
        out.extend_from_slice(b"WAVEfmt ");
        out.extend_from_slice(&16u32.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&rate.to_le_bytes());
        out.extend_from_slice(&(rate * 2).to_le_bytes());
        out.extend_from_slice(&2u16.to_le_bytes());
        out.extend_from_slice(&16u16.to_le_bytes());
        out.extend_from_slice(b"data");
        out.extend_from_slice(&data.to_le_bytes());
        for s in samples {
            out.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes());
        }
        out.into()
    }

    #[test]
    fn recordings_read_as_shapes() {
        let rate = 32_000u32;
        let sr = rate as f32;
        // Room hiss, half a second of it either side of a falling tone that starts at once and dies away.
        let mut seed = 1u32;
        let mut hiss = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5
        };
        let mut x: Vec<f32> = (0..rate / 2).map(|_| 0.004 * hiss()).collect();
        let n = (0.6 * sr) as usize;
        let mut phase = 0f32;
        for i in 0..n {
            let t = i as f32 / n as f32;
            phase += 2.0 * std::f32::consts::PI * 2000.0 * (-2.0 * t).exp2() / sr;
            x.push(0.8 * (1.0 - t).powi(2) * phase.sin() + 0.004 * hiss());
        }
        x.extend((0..rate / 2).map(|_| 0.004 * hiss()));
        let pew = hear(wav(&x, rate)).unwrap();
        assert!((0.3..0.7).contains(&pew.secs.unwrap()), "{pew:?}");
        assert_eq!(pew.sharp, Some(true), "{pew:?}");
        assert!(pew.sweep_octaves.unwrap() < -0.5 && pew.noisiness.unwrap() < 0.3, "{pew:?}");

        // It travels as its numbers and comes back the same.
        let token = pew.token();
        let back = Shape::from_token(token.strip_prefix(HEARD).unwrap()).unwrap();
        assert_eq!(back.token(), token);
        assert_eq!(back.sharp, pew.sharp);
        assert_eq!(Shape::from_token("1,s,500"), None);
        assert_eq!(Shape::from_token("1,x,500,0.1,0,-,q"), None);

        // Only the room: nothing.
        let room: Vec<f32> = (0..rate).map(|_| 0.004 * hiss()).collect();
        assert_eq!(hear(wav(&room, rate)), None);
    }

    #[test]
    fn words_read_as_sounds() {
        // An airy start, a long low middle, a fading hum, shouted.
        let whoom = read("WHOOOM").unwrap();
        assert_eq!(whoom.sharp, Some(false));
        assert_eq!(whoom.rings, Some(true));
        assert!(whoom.loud && (0.7..1.5).contains(&whoom.secs.unwrap()), "{whoom:?}");
        assert!(whoom.brightness_hz.unwrap() < 800.0, "{whoom:?}");
        assert_eq!(whoom.sweep_octaves, Some(0.0));

        // A sharp start and a vowel that falls; short.
        let pew = read("pew").unwrap();
        assert_eq!((pew.sharp, pew.loud), (Some(true), false));
        assert!(pew.sweep_octaves.unwrap() < -1.0 && pew.secs.unwrap() < 0.5, "{pew:?}");
        // Rising, the other way.
        assert!(read("bweee").unwrap().sweep_octaves.unwrap() > 1.0);

        // Hiss: bright and noisy, and drawn out it's long.
        let krssh = read("krssh").unwrap();
        assert!(krssh.brightness_hz.unwrap() > 3000.0 && krssh.noisiness.unwrap() > 0.45, "{krssh:?}");
        assert_eq!(krssh.sweep_octaves, Some(0.0));
        assert!(read("shhhhhh").unwrap().secs.unwrap() > 2.0);

        // A beep: sharp, bright, tonal, cut off.
        let beep = read("beep").unwrap();
        assert!(beep.brightness_hz.unwrap() > 1500.0 && beep.noisiness.unwrap() < 0.25, "{beep:?}");
        assert_eq!(beep.rings, Some(false));

        assert_eq!(read("!!! 123"), None);
        assert_eq!(read("ka-BOOM").map(|s| s.loud), Some(true));
    }
}
