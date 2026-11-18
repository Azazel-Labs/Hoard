//! The tempo and key of music, measured by ear: Hoard's own code, written from the textbook methods.
//!
//! - Tempo: an onset envelope (how much louder each band of the spectrum gets, 100 times a second), its
//!   autocorrelation, and the beat period whose multiples it repeats at best, preferring the tempos most music is at
//!   when the reading could as well be half or double. That other reading is kept as a second guess.
//! - Key: how much of each of the 12 notes the sound holds (its chroma), against the Krumhansl–Kessler profiles of
//!   every major and minor key; the best fit wins. MIDI files take theirs from their notes the same way.

use std::f32::consts::PI;

use serde_json::{Map, Value, json};

use crate::sounds::Fft;

/// Frames of the onset envelope per second.
const FPS: f32 = 100.0;
/// The tempos looked for.
const MIN_BPM: f32 = 50.0;
const MAX_BPM: f32 = 220.0;
/// Where most music is: a reading that could be half or double leans toward it, about an octave either way.
const LIKELY_BPM: f32 = 120.0;
const LIKELY_OCTAVES: f32 = 0.9;
/// How steady a beat must be (the onset envelope's autocorrelation at its multiples) to be called a tempo.
const MIN_BEAT: f32 = 0.2;
/// How much the onset envelope must vary (its standard deviation, in summed log-energy steps) to hold a beat.
const MIN_ONSETS: f32 = 0.3;
/// The shortest sound whose tempo is measured.
const MIN_TEMPO_SECS: f32 = 1.9;
/// How well the notes must fit a key's profile for it to be named, and how long a sound must be, with a beat or
/// without one.
const MIN_KEY_FIT: f32 = 0.6;
const MIN_KEY_SECS: f32 = 1.9;
const MIN_KEY_SECS_UNBEATEN: f32 = 8.0;
/// The notes counted for the key.
const KEY_LOW_HZ: f32 = 80.0;
const KEY_HIGH_HZ: f32 = 5000.0;

/// The meta keys of what this measures.
pub(crate) const KEYS: [&str; 5] =
    ["measured_bpm", "measured_bpm_alt", "measured_bpm_confidence", "measured_key", "measured_key_confidence"];

/// The tempo and key of trimmed, normalized samples `x` at `sr` (see [`KEYS`]), whichever can be told. `loop_secs`
/// is the whole file's length when `x` is all of it: a loop is a whole number of bars long, which pins its tempo.
pub(crate) fn measure(x: &[f32], sr: f32, loop_secs: Option<f32>) -> Map<String, Value> {
    let mut m = Map::new();
    let secs = x.len() as f32 / sr;
    let beat = if secs >= MIN_TEMPO_SECS { tempo(x, sr, loop_secs) } else { None };
    if let Some(b) = &beat {
        m.insert("measured_bpm".into(), json!(round1(b.bpm)));
        m.insert("measured_bpm_alt".into(), json!(round1(b.alt)));
        m.insert("measured_bpm_confidence".into(), json!(round2(b.confidence)));
    }
    if secs >= if beat.is_some() { MIN_KEY_SECS } else { MIN_KEY_SECS_UNBEATEN }
        && let Some((key, fit)) = Key::fit(&chroma(x, sr)).filter(|(_, fit)| *fit >= MIN_KEY_FIT)
    {
        m.insert("measured_key".into(), json!(key.name()));
        m.insert("measured_key_confidence".into(), json!(round2(fit)));
    }
    m
}

fn round1(v: f32) -> f64 {
    (f64::from(v) * 10.0).round() / 10.0
}

fn round2(v: f32) -> f64 {
    (f64::from(v) * 100.0).round() / 100.0
}

// ---------------------------------------------------------------------------------------------
// Tempo

/// A tempo reading.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Beat {
    pub(crate) bpm: f32,
    /// Half or double: the other way to count it.
    pub(crate) alt: f32,
    /// How steady the beat is, 0..=1.
    pub(crate) confidence: f32,
}

/// The tempo of `x`, when its beat is steady enough to have one.
pub(crate) fn tempo(x: &[f32], sr: f32, loop_secs: Option<f32>) -> Option<Beat> {
    let onset = onsets(x, sr);
    // At least two beats of the slowest tempo fit in the lags looked at.
    let max_lag = (onset.len() / 2).min((8.0 * FPS) as usize);
    if (max_lag as f32) < 2.0 * 60.0 * FPS / MAX_BPM {
        return None;
    }
    let ac = autocorrelation(&onset, max_lag);
    // A sound that hardly changes (a hum, a drone) has no beat, however regular its ripples.
    if ac[0].sqrt() < MIN_ONSETS {
        return None;
    }
    let period = |bpm: f32| 60.0 * FPS / bpm;
    // The beat period whose multiples (up to 4) the envelope repeats at best, leaning toward likely tempos.
    let prior = |bpm: f32| (-0.5 * ((bpm / LIKELY_BPM).log2() / LIKELY_OCTAVES).powi(2)).exp();
    let mut best = (0.0f32, f32::MIN);
    let mut bpm = MIN_BPM;
    while bpm <= MAX_BPM {
        let s = comb(&ac, period(bpm), 4).max(0.0) * prior(bpm);
        if s > best.1 {
            best = (bpm, s);
        }
        bpm += 0.25;
    }
    if best.1 <= 0.0 {
        return None;
    }
    // Then finely, over every multiple that fits, for the exact tempo.
    let (mut bpm, mut top) = (best.0, f32::MIN);
    let mut b = best.0 - 1.0;
    while b <= best.0 + 1.0 {
        let s = comb(&ac, period(b), usize::MAX);
        if s > top {
            (bpm, top) = (b, s);
        }
        b += 0.02;
    }
    let confidence = (comb(&ac, period(bpm), 4) / ac[0]).clamp(0.0, 1.0);
    if confidence < MIN_BEAT {
        return None;
    }
    // A whole loop holds a whole number of bars: the tempo that fits them exactly, when it's near.
    if let Some(len) = loop_secs.filter(|l| *l > 0.0) {
        let beats = (len * bpm / 60.0).round();
        let exact = 60.0 * beats / len;
        if beats >= 4.0 && beats % 4.0 == 0.0 && (exact / bpm - 1.0).abs() <= 0.015 {
            bpm = exact;
        }
    }
    if (bpm - bpm.round()).abs() <= 0.2 {
        bpm = bpm.round();
    }
    let alt = if bpm >= LIKELY_BPM { bpm / 2.0 } else { bpm * 2.0 };
    Some(Beat { bpm, alt, confidence })
}

/// How much louder each band of the spectrum gets, [`FPS`] times a second, less its local average.
fn onsets(x: &[f32], sr: f32) -> Vec<f32> {
    let hop = ((sr / FPS).round() as usize).max(1);
    let n = ((sr * 0.04) as usize).max(256).next_power_of_two();
    if x.len() < n {
        return Vec::new();
    }
    let window = hann(n);
    let fft = Fft::new(n);
    let bands = bands(n, sr, 24, 40.0, (sr * 0.45).min(12_000.0));
    let (mut re, mut im) = (vec![0f32; n], vec![0f32; n]);
    let mut prev = vec![0f32; bands.len()];
    let mut flux = Vec::with_capacity(x.len() / hop + 1);
    let mut start = 0;
    while start + n <= x.len() {
        for (i, (r, w)) in re.iter_mut().zip(&window).enumerate() {
            *r = x[start + i] * w;
        }
        im.fill(0.0);
        fft.run(&mut re, &mut im);
        let mut f = 0.0;
        for (band, p) in bands.iter().zip(prev.iter_mut()) {
            let e: f32 = band.clone().map(|k| re[k] * re[k] + im[k] * im[k]).sum::<f32>() / n as f32;
            let level = (1.0 + 1000.0 * e).ln();
            f += (level - *p).max(0.0);
            *p = level;
        }
        flux.push(f);
        start += hop;
    }
    if let Some(first) = flux.first_mut() {
        // The first frame rises from nothing.
        *first = 0.0;
    }
    // Less the average around it (half a second), so only the jumps are left.
    let half = (0.25 * FPS) as usize;
    let mut sums = vec![0f32; flux.len() + 1];
    for (i, f) in flux.iter().enumerate() {
        sums[i + 1] = sums[i] + f;
    }
    (0..flux.len())
        .map(|i| {
            let (a, b) = (i.saturating_sub(half), (i + half + 1).min(flux.len()));
            (flux[i] - (sums[b] - sums[a]) / (b - a) as f32).max(0.0)
        })
        .collect()
}

/// The bins of `count` bands spaced evenly in pitch from `lo` to `hi` Hz, for an `n`-point spectrum at `sr`.
fn bands(n: usize, sr: f32, count: usize, lo: f32, hi: f32) -> Vec<std::ops::Range<usize>> {
    let bin = |hz: f32| ((hz * n as f32 / sr).round() as usize).clamp(1, n / 2);
    let mut out: Vec<std::ops::Range<usize>> = Vec::with_capacity(count);
    for i in 0..count {
        let a = bin(lo * (hi / lo).powf(i as f32 / count as f32));
        let b = bin(lo * (hi / lo).powf((i + 1) as f32 / count as f32)).max(a + 1);
        let a = out.last().map_or(a, |r| r.end.max(a));
        if a < b {
            out.push(a..b);
        }
    }
    out
}

fn hann(n: usize) -> Vec<f32> {
    (0..n).map(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / n as f32).cos()).collect()
}

/// The autocorrelation of the envelope less its mean, per overlapping frame, lightly smoothed so a beat that falls
/// between frames still lines up.
fn autocorrelation(o: &[f32], max_lag: usize) -> Vec<f32> {
    let mean = o.iter().sum::<f32>() / o.len().max(1) as f32;
    let c: Vec<f32> = o.iter().map(|v| v - mean).collect();
    let raw: Vec<f32> =
        (0..=max_lag).map(|l| c.iter().zip(&c[l..]).map(|(a, b)| a * b).sum::<f32>() / (c.len() - l) as f32).collect();
    (0..raw.len())
        .map(|i| {
            if i == 0 {
                raw[0]
            } else {
                0.25 * raw[i - 1] + 0.5 * raw[i] + 0.25 * raw.get(i + 1).copied().unwrap_or(raw[i])
            }
        })
        .collect()
}

/// The autocorrelation at a lag between frames.
fn at(ac: &[f32], lag: f32) -> f32 {
    let i = lag.floor() as usize;
    let f = lag - i as f32;
    match (ac.get(i), ac.get(i + 1)) {
        (Some(a), Some(b)) => a + (b - a) * f,
        (Some(a), None) => *a,
        _ => 0.0,
    }
}

/// The mean autocorrelation at the first `most` multiples of `period` that fit.
fn comb(ac: &[f32], period: f32, most: usize) -> f32 {
    let last = (ac.len() - 1) as f32;
    let ks = ((last / period) as usize).min(most);
    if ks == 0 {
        return 0.0;
    }
    (1..=ks).map(|k| at(ac, k as f32 * period)).sum::<f32>() / ks as f32
}

// ---------------------------------------------------------------------------------------------
// Key

/// How much of each of the 12 notes (C first) `x` holds, each moment counting alike.
fn chroma(x: &[f32], sr: f32) -> [f32; 12] {
    let n = ((sr * 0.37) as usize).max(1024).next_power_of_two();
    let mut out = [0f32; 12];
    if x.len() < n / 2 {
        return out;
    }
    // Each bin's note, and how near its middle it falls.
    let notes: Vec<(usize, usize, f32)> = (1..n / 2)
        .filter_map(|k| {
            let hz = k as f32 * sr / n as f32;
            if !(KEY_LOW_HZ..=KEY_HIGH_HZ).contains(&hz) {
                return None;
            }
            let midi = 69.0 + 12.0 * (hz / 440.0).log2();
            let near = midi.round();
            let w = 1.0 - 2.0 * (midi - near).abs();
            (w > 0.0).then_some((k, near.rem_euclid(12.0) as usize, w))
        })
        .collect();
    let window = hann(n);
    let fft = Fft::new(n);
    let (mut re, mut im) = (vec![0f32; n], vec![0f32; n]);
    let mut frames: Vec<[f32; 12]> = Vec::new();
    let mut start = 0;
    loop {
        let end = (start + n).min(x.len());
        for (i, (r, w)) in re.iter_mut().zip(&window).enumerate() {
            *r = x.get(start + i).filter(|_| start + i < end).map_or(0.0, |s| s * w);
        }
        im.fill(0.0);
        fft.run(&mut re, &mut im);
        let mut c = [0f32; 12];
        for &(k, pc, w) in &notes {
            c[pc] += w * (re[k] * re[k] + im[k] * im[k]).sqrt();
        }
        frames.push(c);
        if end == x.len() {
            break;
        }
        start += n / 2;
    }
    // Quiet moments (under a tenth of the loudest) are left out; the rest count alike.
    let loudest = frames.iter().map(|c| c.iter().sum::<f32>()).fold(0f32, f32::max);
    if loudest <= 0.0 {
        return out;
    }
    for c in &frames {
        let sum: f32 = c.iter().sum();
        if sum >= loudest * 0.1 {
            for (o, v) in out.iter_mut().zip(c) {
                *o += v / sum;
            }
        }
    }
    out
}

/// A key: its tonic (0 = C) and mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Key {
    pub(crate) tonic: u8,
    pub(crate) minor: bool,
}

/// The Krumhansl–Kessler key profiles: how well each note of the scale (tonic first) fits a key.
const MAJOR: [f32; 12] = [6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88];
const MINOR: [f32; 12] = [6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17];

/// Tonics by how keys are usually written.
const MAJOR_NAMES: [&str; 12] = ["C", "D♭", "D", "E♭", "E", "F", "F♯", "G", "A♭", "A", "B♭", "B"];
const MINOR_NAMES: [&str; 12] = ["C", "C♯", "D", "E♭", "E", "F", "F♯", "G", "G♯", "A", "B♭", "B"];

impl Key {
    /// The key whose profile fits how much of each note there is (`weights`, C first) best, and how well (Pearson's
    /// r), when there are any notes at all.
    pub(crate) fn fit(weights: &[f32; 12]) -> Option<(Self, f32)> {
        let total: f32 = weights.iter().sum();
        if total <= 0.0 || !total.is_finite() {
            return None;
        }
        let mut best: Option<(Self, f32)> = None;
        for minor in [false, true] {
            for tonic in 0..12u8 {
                let key = Self { tonic, minor };
                let r = key.fits(weights);
                if best.is_none_or(|(_, b)| r > b) {
                    best = Some((key, r));
                }
            }
        }
        best
    }

    /// How well how much of each note there is (`weights`, C first) fits this key's profile (Pearson's r).
    pub(crate) fn fits(self, weights: &[f32; 12]) -> f32 {
        let profile = if self.minor { &MINOR } else { &MAJOR };
        let rotated: [f32; 12] = std::array::from_fn(|i| profile[(i + 12 - usize::from(self.tonic % 12)) % 12]);
        pearson(weights, &rotated)
    }

    /// "A minor", "E♭ major".
    pub(crate) fn name(self) -> String {
        let t = usize::from(self.tonic % 12);
        if self.minor { format!("{} minor", MINOR_NAMES[t]) } else { format!("{} major", MAJOR_NAMES[t]) }
    }

    /// Its tag: `key:a-minor`, `key:e-flat-major`.
    pub(crate) fn tag(self) -> String {
        let name = self.name().to_lowercase().replace('♭', "-flat").replace('♯', "-sharp").replace(' ', "-");
        format!("key:{name}")
    }

    /// Its place on the Camelot wheel, as DJs write keys: `8A` for A minor, `8B` for C major.
    pub(crate) fn camelot(self) -> String {
        let major = if self.minor { (self.tonic + 3) % 12 } else { self.tonic % 12 };
        format!("{}{}", (major * 7 + 7) % 12 + 1, if self.minor { 'A' } else { 'B' })
    }

    /// The keys that mix with it: itself, its relative major or minor, and the keys a fifth either side.
    pub(crate) fn compatible(self) -> [Self; 4] {
        let relative = if self.minor { (self.tonic + 3) % 12 } else { (self.tonic + 9) % 12 };
        [
            self,
            Self { tonic: relative, minor: !self.minor },
            Self { tonic: (self.tonic + 7) % 12, minor: self.minor },
            Self { tonic: (self.tonic + 5) % 12, minor: self.minor },
        ]
    }

    /// A key however it's written: `A minor`, `Am`, `a-minor`, `E♭ major`, `Eb`, `e-flat-major`, `F#m`, or `8A`.
    pub(crate) fn parse(s: &str) -> Option<Self> {
        let s = s.trim().to_lowercase().replace('♭', "b").replace('♯', "#").replace("flat", "b").replace("sharp", "#");
        let s: String =
            s.strip_prefix("key:").unwrap_or(&s).chars().filter(|c| !matches!(c, ' ' | '-' | '_')).collect();
        if let Some(letter @ ('a' | 'b')) = s.chars().last()
            && let Ok(n @ 1..=12) = s[..s.len() - 1].parse::<u8>()
        {
            // Camelot: 8B is C major, each step up a fifth higher; the A of a number is its relative minor.
            let major = ((n + 12 - 8) * 7) % 12;
            return Some(if letter == 'a' {
                Self { tonic: (major + 9) % 12, minor: true }
            } else {
                Self { tonic: major, minor: false }
            });
        }
        let mut chars = s.chars();
        let base: i8 = match chars.next()? {
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
        let mode = |m: &str| match m {
            "" | "maj" | "major" => Some(false),
            "m" | "min" | "minor" => Some(true),
            _ => None,
        };
        let (shift, minor) = if let Some(minor) = mode(rest) {
            (0, minor)
        } else if let Some(m) = rest.strip_prefix('#') {
            (1, mode(m)?)
        } else {
            (-1, mode(rest.strip_prefix('b')?)?)
        };
        Some(Self { tonic: (base + shift).rem_euclid(12) as u8, minor })
    }

    /// The key a MIDI key signature names: `sharps` sharps (flats below zero), major or minor.
    pub(crate) fn from_signature(sharps: i8, minor: bool) -> Self {
        let major = (i32::from(sharps) * 7).rem_euclid(12) as u8;
        if minor { Self { tonic: (major + 9) % 12, minor } } else { Self { tonic: major, minor } }
    }
}

fn pearson(a: &[f32; 12], b: &[f32; 12]) -> f32 {
    let (ma, mb) = (a.iter().sum::<f32>() / 12.0, b.iter().sum::<f32>() / 12.0);
    let (mut ab, mut aa, mut bb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        ab += (x - ma) * (y - mb);
        aa += (x - ma) * (x - ma);
        bb += (y - mb) * (y - mb);
    }
    if aa <= 0.0 || bb <= 0.0 { 0.0 } else { ab / (aa * bb).sqrt() }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f32 = 44_100.0;

    /// A repeatable noise source.
    struct Noise(u32);
    impl Noise {
        fn next(&mut self) -> f32 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 17;
            self.0 ^= self.0 << 5;
            (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0
        }
    }

    /// A drum loop at `bpm` for `secs`: a kick on every beat, a hat between, a snare on two and four.
    fn drums(bpm: f32, secs: f32) -> Vec<f32> {
        let mut x = vec![0f32; (secs * RATE) as usize];
        let mut noise = Noise(7);
        let beat = 60.0 / bpm;
        let mut hit = |at: f32, x: &mut Vec<f32>, f: &mut dyn FnMut(f32, &mut Noise) -> f32, len: f32| {
            let s = (at * RATE) as usize;
            for i in 0..(len * RATE) as usize {
                if let Some(v) = x.get_mut(s + i) {
                    *v += f(i as f32 / RATE, &mut noise);
                }
            }
        };
        let mut b = 0;
        while (b as f32) * beat < secs {
            let t = b as f32 * beat;
            hit(t, &mut x, &mut |s, _| (2.0 * PI * 60.0 * s).sin() * (-s * 18.0).exp(), 0.25);
            if b % 2 == 1 {
                hit(t, &mut x, &mut |s, n| 0.5 * n.next() * (-s * 25.0).exp(), 0.2);
            }
            hit(t + beat / 2.0, &mut x, &mut |s, n| 0.2 * n.next() * (-s * 80.0).exp(), 0.05);
            b += 1;
        }
        normalize(x)
    }

    fn normalize(mut x: Vec<f32>) -> Vec<f32> {
        let peak = x.iter().fold(0f32, |m, s| m.max(s.abs())).max(1e-9);
        for s in &mut x {
            *s /= peak;
        }
        x
    }

    /// A note with a few overtones, as an instrument might play it.
    fn note(midi: f32, at: usize, len: usize, x: &mut [f32]) {
        let hz = 440.0 * 2f32.powf((midi - 69.0) / 12.0);
        for i in 0..len {
            let t = i as f32 / RATE;
            let env = (t * 200.0).min(1.0) * (-t * 1.5).exp();
            let v: f32 = (1..=4).map(|h| (2.0 * PI * hz * h as f32 * t).sin() / h as f32).sum();
            if let Some(s) = x.get_mut(at + i) {
                *s += 0.2 * env * v;
            }
        }
    }

    /// Chords, a beat each at `bpm` (MIDI notes), over and over for `secs`.
    fn chords(progression: &[&[f32]], bpm: f32, secs: f32) -> Vec<f32> {
        let mut x = vec![0f32; (secs * RATE) as usize];
        let beat = (60.0 / bpm * RATE) as usize;
        let mut i = 0;
        while i * beat < x.len() {
            for &n in progression[i % progression.len()] {
                note(n, i * beat, beat, &mut x);
            }
            i += 1;
        }
        normalize(x)
    }

    #[test]
    fn the_tempo_of_a_beat() {
        for bpm in [72.0, 95.0, 120.0, 128.0, 140.0, 174.0] {
            let b = tempo(&drums(bpm, 20.0), RATE, None).unwrap_or_else(|| panic!("no tempo at {bpm}"));
            let off = (b.bpm - bpm).abs().min((b.alt - bpm).abs());
            assert!(off <= 0.5, "{bpm}: heard {b:?}");
            assert!(b.confidence >= 0.5, "{bpm}: {b:?}");
        }
        // Most of them as played, not half or double.
        let b = tempo(&drums(120.0, 20.0), RATE, None).unwrap();
        assert_eq!((b.bpm, b.alt), (120.0, 60.0));
    }

    #[test]
    fn a_loop_is_whole_bars() {
        // Two bars at 123.4: exactly what fits.
        let secs = 8.0 * 60.0 / 123.4;
        let b = tempo(&drums(123.4, secs), RATE, Some(secs)).unwrap();
        assert!((b.bpm - 123.4).abs() < 0.05, "{b:?}");
    }

    #[test]
    fn noise_and_a_hum_have_no_tempo() {
        let mut n = Noise(3);
        let hiss: Vec<f32> = (0..(10.0 * RATE) as usize).map(|_| n.next()).collect();
        assert_eq!(tempo(&hiss, RATE, None), None);
        let hum: Vec<f32> = (0..(10.0 * RATE) as usize).map(|i| (2.0 * PI * 110.0 * i as f32 / RATE).sin()).collect();
        assert_eq!(tempo(&hum, RATE, None), None);
        // Too short.
        assert_eq!(tempo(&drums(120.0, 0.5), RATE, None), None);
    }

    #[test]
    fn the_key_of_chords() {
        // i iv V i in A minor; I IV V I in E♭ major; I vi IV V in G major.
        let a_minor: &[&[f32]] = &[&[57.0, 60.0, 64.0], &[62.0, 65.0, 69.0], &[64.0, 68.0, 71.0], &[57.0, 60.0, 64.0]];
        let e_flat: &[&[f32]] = &[&[63.0, 67.0, 70.0], &[68.0, 72.0, 75.0], &[70.0, 74.0, 77.0], &[63.0, 67.0, 70.0]];
        let g: &[&[f32]] = &[&[55.0, 59.0, 62.0], &[52.0, 55.0, 59.0], &[48.0, 52.0, 55.0], &[50.0, 54.0, 57.0]];
        for (progression, want) in [(a_minor, "A minor"), (e_flat, "E♭ major"), (g, "G major")] {
            let m = measure(&chords(progression, 100.0, 12.0), RATE, None);
            assert_eq!(m.get("measured_key").and_then(Value::as_str), Some(want), "{m:?}");
        }
        // Drums alone are in no key.
        let m = measure(&drums(120.0, 12.0), RATE, None);
        assert!(m.contains_key("measured_bpm"), "{m:?}");
        assert!(!m.contains_key("measured_key"), "{m:?}");
    }

    #[test]
    fn keys_written_every_way() {
        let a_minor = Key { tonic: 9, minor: true };
        let e_flat = Key { tonic: 3, minor: false };
        for s in ["A minor", "Am", "a-minor", "amin", "key:a-minor", "8A", "8a"] {
            assert_eq!(Key::parse(s), Some(a_minor), "{s}");
        }
        for s in ["E♭ major", "Eb", "eb-major", "e-flat-major", "D#", "5B"] {
            assert_eq!(Key::parse(s), Some(e_flat), "{s}");
        }
        assert_eq!(Key::parse("F#m"), Some(Key { tonic: 6, minor: true }));
        assert_eq!(Key::parse("Bbm"), Some(Key { tonic: 10, minor: true }));
        assert_eq!(Key::parse("B"), Some(Key { tonic: 11, minor: false }));
        assert_eq!(Key::parse("H"), None);
        assert_eq!(Key::parse("13A"), None);
        assert_eq!(
            (a_minor.name(), a_minor.tag(), a_minor.camelot()),
            ("A minor".into(), "key:a-minor".into(), "8A".into())
        );
        assert_eq!(
            (e_flat.name(), e_flat.tag(), e_flat.camelot()),
            ("E♭ major".into(), "key:e-flat-major".into(), "5B".into())
        );
        assert_eq!(Key { tonic: 6, minor: true }.tag(), "key:f-sharp-minor");
        // Every key's tag reads back as it, and so does its Camelot.
        for tonic in 0..12 {
            for minor in [false, true] {
                let k = Key { tonic, minor };
                assert_eq!(Key::parse(&k.tag()), Some(k));
                assert_eq!(Key::parse(&k.camelot()), Some(k));
                assert_eq!(Some(k.tag()), hoard_core::normalize_tag(&k.tag()));
            }
        }
        // A minor mixes with C major, and E minor and D minor a fifth either side.
        let names: Vec<String> = a_minor.compatible().iter().map(|k| k.name()).collect();
        assert_eq!(names, ["A minor", "C major", "E minor", "D minor"]);
        // MIDI key signatures: none is C major or A minor, three flats E♭ major or C minor, two sharps D major.
        assert_eq!(Key::from_signature(0, true), a_minor);
        assert_eq!(Key::from_signature(-3, false), e_flat);
        assert_eq!(Key::from_signature(-3, true).name(), "C minor");
        assert_eq!(Key::from_signature(2, false).name(), "D major");
    }
}
