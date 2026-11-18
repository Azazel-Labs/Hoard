//! Listening to a sound: decoding it to mono, cutting it into frames with their mel bands, and measuring the
//! features kept in its meta (how it starts and ends, how bright and noisy it is, whether it loops).

use std::f32::consts::PI;

use serde_json::{Map, Value, json};

use super::likeness::{BANDS, descriptor, print};

/// Sounds that die away at once but are at least this long (seconds) may still be music.
const MUSIC_HIT_SECS: f32 = 4.0;

/// How much of a file is listened to.
pub(super) const MAX_SECS: f32 = 30.0;
/// How much of a file is read: 30 seconds of 48 kHz 24-bit stereo WAV, and far more of anything compressed.
pub(super) const MAX_BYTES: u64 = 12 << 20;

/// The lowest and highest frequencies the mel bands cover, whatever the file's rate (so a file recorded at a low rate
/// has empty top bands, which is how it sounds).
const LOW_HZ: f32 = 40.0;
pub(super) const HIGH_HZ: f32 = 16_000.0;
/// Below this, a frame's energy counts as bass.
const BASS_HZ: f32 = 200.0;

/// The meta keys of the measured features.
pub(crate) const FEATURES: [&str; 12] = [
    "attack_ms",
    "tail_ms",
    "envelope",
    "brightness_hz",
    "noisiness",
    "sweep_octaves",
    "loops_cleanly",
    crate::rhythm::KEYS[0],
    crate::rhythm::KEYS[1],
    crate::rhythm::KEYS[2],
    crate::rhythm::KEYS[3],
    crate::rhythm::KEYS[4],
];

/// What listening to a file gives.
#[derive(Clone, Debug)]
pub(crate) struct Hearing {
    pub(crate) descriptor: Vec<i8>,
    /// See [`print`].
    pub(crate) print: Vec<u8>,
    pub(crate) features: Map<String, Value>,
}

/// A file's descriptor, print and measured features, or None if it isn't audio this can decode (or is silent).
/// `bytes` is the whole file or its first [`MAX_BYTES`].
pub(crate) fn analyze(bytes: bytes::Bytes, ext: Option<&str>) -> Option<Hearing> {
    let cut = bytes.len() as u64 >= MAX_BYTES;
    let (mono, rate, whole) = decode_mono(bytes, ext)?;
    hear(&mono, rate, whole && !cut)
}

/// The measured features of a sound file (see [`FEATURES`]), from its first [`MAX_BYTES`].
pub(crate) fn measure(path: &std::path::Path, ext: &str) -> Option<Map<String, Value>> {
    use std::io::Read;
    let mut head = Vec::new();
    std::fs::File::open(path).ok()?.take(MAX_BYTES).read_to_end(&mut head).ok()?;
    analyze(bytes::Bytes::from(head), Some(ext)).map(|h| h.features)
}

/// The measured features (see [`FEATURES`]) of mono samples at `rate`, taken as the start of something longer.
pub(crate) fn listen(mono: &[f32], rate: u32) -> Option<Map<String, Value>> {
    hear(mono, rate, false).map(|h| h.features)
}

/// Up to [`MAX_SECS`] of a file, mixed to mono, and whether that's all of it.
pub(crate) fn decode_mono(bytes: bytes::Bytes, ext: Option<&str>) -> Option<(Vec<f32>, u32, bool)> {
    let mut mono = Vec::new();
    let mut rate = 0u32;
    let decoded = crate::audio::decode_each(Box::new(std::io::Cursor::new(bytes)), ext, |pcm, channels, r, _| {
        if rate != 0 && r != rate {
            return Ok(false);
        }
        rate = r;
        let c = usize::from(channels.max(1));
        mono.extend(pcm.chunks_exact(c).map(|frame| frame.iter().sum::<f32>() / c as f32));
        Ok((mono.len() as f32) < MAX_SECS * r as f32)
    });
    // Only the start of a long file is read, so it can end abruptly: what decoded still counts.
    if let Err(e) = &decoded
        && mono.is_empty()
    {
        tracing::debug!(error = %format!("{e:#}"), "not a sound this can listen to");
        return None;
    }
    if rate == 0 || mono.is_empty() {
        return None;
    }
    let whole = decoded.is_ok() && (mono.len() as f32) < MAX_SECS * rate as f32;
    mono.truncate((MAX_SECS * rate as f32) as usize);
    Some((mono, rate, whole))
}

/// The descriptor, print and measured features of mono samples at `rate`; `whole` when they're the whole file, not its
/// start.
pub(super) fn hear(mono: &[f32], rate: u32, whole: bool) -> Option<Hearing> {
    let sr = rate as f32;
    let peak = mono.iter().fold(0f32, |m, s| m.max(s.abs()));
    if peak < 1e-4 {
        return None;
    }
    // The silence at either end (below about -50 dB of the peak) is left out.
    let gate = peak * 0.003;
    let start = mono.iter().position(|s| s.abs() > gate)?;
    let end = mono.iter().rposition(|s| s.abs() > gate)? + 1;
    let x: Vec<f32> = mono[start..end].iter().map(|s| s / peak).collect();
    let secs = x.len() as f32 / sr;

    let frames = frames(&x, sr);
    let loudest = frames.iter().fold(0f32, |m, f| m.max(f.rms));
    let voiced: Vec<&Frame> = frames.iter().filter(|f| f.rms > loudest * 0.03).collect();
    if voiced.is_empty() {
        return None;
    }
    let descriptor = descriptor(&x, sr, &frames, &voiced, secs);
    let features = features(mono, &x, sr, &frames, loudest, whole);
    Some(Hearing { descriptor, print: print(&frames, secs), features })
}

/// The loudness envelope's step, for the attack and the tail.
const STEP_SECS: f32 = 0.005;

fn rms(x: &[f32]) -> f32 {
    (x.iter().map(|s| s * s).sum::<f32>() / x.len().max(1) as f32).sqrt()
}

/// The measured features (see [`FEATURES`]) of `x`, the trimmed and normalized `mono`, whose `frames` peak at
/// `loudest`.
///
/// - `attack_ms`: how long it takes to get within 3 dB of its loudest.
/// - `tail_ms`: how long it goes on after it was last within 6 dB of its loudest.
/// - `envelope`: `hit` (a sharp attack that dies away), `swell` (it builds up to its loudest), or `sustained` (it
///   starts quickly and holds); none when it's none of those.
/// - `brightness_hz`: its spectral centroid, the louder moments counting more.
/// - `noisiness`: 0 for a pure tone, near 1 for white noise.
/// - `sweep_octaves`: how far its brightness rises (or, below zero, falls) from start to end.
/// - `loops_cleanly`: its end runs into its start without a jump in level or a click (only known for a whole file).
/// - The tempo and key of music (see `rhythm::KEYS`).
fn features(mono: &[f32], x: &[f32], sr: f32, frames: &[Frame], loudest: f32, whole: bool) -> Map<String, Value> {
    let mut m = Map::new();
    let secs = x.len() as f32 / sr;
    let ms = secs * 1000.0;

    let step = ((sr * STEP_SECS) as usize).max(1);
    let step_ms = step as f32 * 1000.0 / sr;
    let env: Vec<f32> = x.chunks(step).map(rms).collect();
    let top = env.iter().fold(0f32, |a, &b| a.max(b)).max(1e-9);
    let attack = env.iter().position(|&e| e >= top * 0.708).map_or(0.0, |i| ((i + 1) as f32 * step_ms).min(ms));
    let last_loud = env.iter().rposition(|&e| e >= top * 0.5).unwrap_or(0);
    let tail = (ms - (last_loud + 1) as f32 * step_ms).max(0.0);
    m.insert("attack_ms".into(), json!(attack.round() as u32));
    m.insert("tail_ms".into(), json!(tail.round() as u32));
    // How loud its last quarter is, against its loudest.
    let quarter = &env[env.len() - (env.len() / 4).max(1)..];
    let end = quarter.iter().sum::<f32>() / quarter.len() as f32 / top;
    let envelope = if attack <= 25.0 && end < 0.25 {
        Some("hit")
    } else if attack >= 150.0 && attack >= 0.4 * ms {
        Some("swell")
    } else if attack <= 100.0 && end >= 0.35 && secs >= 0.5 {
        Some("sustained")
    } else {
        None
    };
    if let Some(e) = envelope {
        m.insert("envelope".into(), json!(e));
    }

    let voiced: Vec<&Frame> = frames.iter().filter(|f| f.rms > loudest * 0.03).collect();
    let weight: f32 = voiced.iter().map(|f| f.rms * f.rms).sum::<f32>().max(1e-12);
    let brightness = voiced.iter().map(|f| f.centroid * f.rms * f.rms).sum::<f32>() / weight;
    m.insert("brightness_hz".into(), json!((brightness / 10.0).round() as u32 * 10));
    let flatness = voiced.iter().map(|f| f.flatness).sum::<f32>() / voiced.len().max(1) as f32;
    m.insert("noisiness".into(), json!((f64::from(flatness.sqrt()) * 100.0).round() / 100.0));

    // The slope of the brightness, in octaves, the louder moments counting more, over how long it's loud: a sound that
    // dies away quickly hardly sweeps, however its brightness moves in its quiet tail.
    if secs >= 0.2 && voiced.len() >= 4 {
        let w = |f: &Frame| f.rms;
        let y = |f: &Frame| f.centroid.max(20.0).log2();
        let weight: f32 = voiced.iter().map(|f| w(f)).sum();
        let mt = voiced.iter().map(|f| w(f) * f.at).sum::<f32>() / weight;
        let my = voiced.iter().map(|f| w(f) * y(f)).sum::<f32>() / weight;
        let (mut ty, mut tt) = (0.0, 0.0);
        for f in &voiced {
            ty += w(f) * (f.at - mt) * (y(f) - my);
            tt += w(f) * (f.at - mt) * (f.at - mt);
        }
        if tt > 1e-12 {
            // As long as the times would be if they were spread evenly, from their spread.
            let span = (12.0 * tt / weight).sqrt();
            m.insert("sweep_octaves".into(), json!((f64::from(ty / tt * span) * 10.0).round() / 10.0 + 0.0));
        }
    }

    if whole && mono.len() as f32 >= 0.5 * sr && loops_cleanly(mono, sr) {
        m.insert("loops_cleanly".into(), json!(true));
    }
    // The tempo and key of music. A short sound that dies away at once has neither; music that starts on its loudest
    // chord can look like one, so only short ones are passed over.
    if envelope != Some("hit") || secs >= MUSIC_HIT_SECS {
        m.extend(crate::rhythm::measure(x, sr, whole.then(|| mono.len() as f32 / sr)));
    }
    m
}

/// Whether a whole file's end runs into its start: both sound (within 30 dB of its peak), at levels within 6 dB of
/// each other, and the step from the last sample to the first is no bigger than the steps either side of it.
fn loops_cleanly(mono: &[f32], sr: f32) -> bool {
    let peak = mono.iter().fold(0f32, |m, s| m.max(s.abs()));
    let edge = ((0.05 * sr) as usize).min(mono.len() / 2).max(2);
    let (head, tail) = (rms(&mono[..edge]), rms(&mono[mono.len() - edge..]));
    if head.min(tail) < peak * 0.0316 || head.max(tail) > 2.0 * head.min(tail) {
        return false;
    }
    let near = ((0.01 * sr) as usize).clamp(2, edge);
    let steps = |s: &[f32]| s.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>() / (s.len() - 1) as f32;
    let typical = steps(&mono[..near]).max(steps(&mono[mono.len() - near..]));
    let jump = (mono[0] - mono[mono.len() - 1]).abs();
    jump <= 3.0 * typical + peak * 1e-3
}

/// What one analysis frame (about 46 ms) holds.
#[derive(Debug)]
pub(super) struct Frame {
    pub(super) rms: f32,
    /// Energy per mel band, in dB.
    pub(super) db: [f32; BANDS],
    /// Spectral centroid, in Hz.
    pub(super) centroid: f32,
    /// Geometric over arithmetic mean of the power spectrum: near 1 for noise, near 0 for a tone.
    pub(super) flatness: f32,
    /// Zero crossings per second.
    pub(super) zcr: f32,
    /// Share of the energy under [`BASS_HZ`].
    pub(super) bass: f32,
    /// Its middle, in seconds from the start.
    pub(super) at: f32,
}

/// The frames of `x`, half overlapping.
fn frames(x: &[f32], sr: f32) -> Vec<Frame> {
    let n = 1usize << ((sr * 0.046).log2().round() as u32).clamp(8, 12);
    let hop = n / 2;
    let fft = Fft::new(n);
    let window: Vec<f32> = (0..n).map(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / n as f32).cos()).collect();
    let filters = mel_filters(n, sr);
    let bins = n / 2 + 1;
    let hz = |k: usize| k as f32 * sr / n as f32;
    let count = if x.len() <= n { 1 } else { (x.len() - n).div_ceil(hop) + 1 };
    let (mut re, mut im) = (vec![0f32; n], vec![0f32; n]);
    let mut power = vec![0f32; bins];
    (0..count)
        .map(|i| {
            let at = i * hop;
            let raw = &x[at.min(x.len())..(at + n).min(x.len())];
            for k in 0..n {
                re[k] = raw.get(k).map_or(0.0, |s| s * window[k]);
                im[k] = 0.0;
            }
            fft.run(&mut re, &mut im);
            for (k, p) in power.iter_mut().enumerate() {
                *p = re[k] * re[k] + im[k] * im[k];
            }
            let total: f32 = power[1..].iter().sum::<f32>().max(1e-12);
            let centroid = power.iter().enumerate().skip(1).map(|(k, p)| hz(k) * p).sum::<f32>() / total;
            let log_mean = power[1..].iter().map(|p| (p + 1e-12).ln()).sum::<f32>() / (bins - 1) as f32;
            let flatness = (log_mean.exp() / (total / (bins - 1) as f32)).clamp(0.0, 1.0);
            let bass =
                power.iter().enumerate().skip(1).filter(|(k, _)| hz(*k) < BASS_HZ).map(|(_, p)| p).sum::<f32>() / total;
            let mut db = [0f32; BANDS];
            for (d, f) in db.iter_mut().zip(filters.iter()) {
                let e: f32 = f.iter().map(|&(k, w)| power[k] * w).sum();
                *d = 10.0 * (e + 1e-10).log10();
            }
            let crossings = raw.windows(2).filter(|w| (w[0] >= 0.0) != (w[1] >= 0.0)).count();
            Frame {
                rms: (raw.iter().map(|s| s * s).sum::<f32>() / raw.len().max(1) as f32).sqrt(),
                db,
                centroid,
                flatness,
                zcr: crossings as f32 * sr / raw.len().max(2) as f32,
                bass,
                at: (at + raw.len() / 2) as f32 / sr,
            }
        })
        .collect()
}

/// Triangular mel filters from [`LOW_HZ`] to [`HIGH_HZ`]: for each band, the power-spectrum bins and their weights.
#[expect(clippy::many_single_char_names, reason = "a filter's left, center and right edges")]
fn mel_filters(n: usize, sr: f32) -> Vec<Vec<(usize, f32)>> {
    let edges = mel_edges();
    let bin_hz = sr / n as f32;
    let bins = n / 2 + 1;
    (0..BANDS)
        .map(|b| {
            let (l, c, r) = (edges[b], edges[b + 1], edges[b + 2]);
            if l >= sr / 2.0 {
                return Vec::new();
            }
            let mut f: Vec<(usize, f32)> = (0..bins)
                .filter_map(|k| {
                    let h = k as f32 * bin_hz;
                    let w = if h <= l || h >= r {
                        0.0
                    } else if h <= c {
                        (h - l) / (c - l)
                    } else {
                        (r - h) / (r - c)
                    };
                    (w > 0.0).then_some((k, w))
                })
                .collect();
            if f.is_empty() {
                // A band narrower than a bin: the nearest bin.
                f.push((((c / bin_hz).round() as usize).min(bins - 1), 1.0));
            }
            f
        })
        .collect()
}

/// The mel bands' edges, in Hz: band `b` rises from `edges[b]`, peaks at `edges[b + 1]` and ends at `edges[b + 2]`.
pub(super) fn mel_edges() -> Vec<f32> {
    let mel = |f: f32| 2595.0 * (1.0 + f / 700.0).log10();
    let hz = |m: f32| 700.0 * (10f32.powf(m / 2595.0) - 1.0);
    let (lo, hi) = (mel(LOW_HZ), mel(HIGH_HZ));
    (0..BANDS + 2).map(|i| hz(lo + (hi - lo) * i as f32 / (BANDS + 1) as f32)).collect()
}

/// An in-place radix-2 FFT of a fixed power-of-two size.
#[derive(Debug)]
pub(crate) struct Fft {
    n: usize,
    rev: Vec<usize>,
    cos: Vec<f32>,
    sin: Vec<f32>,
}

impl Fft {
    pub(crate) fn new(n: usize) -> Self {
        let bits = n.trailing_zeros();
        Self {
            n,
            rev: (0..n).map(|i| i.reverse_bits() >> (usize::BITS - bits)).collect(),
            cos: (0..n / 2).map(|k| (-2.0 * PI * k as f32 / n as f32).cos()).collect(),
            sin: (0..n / 2).map(|k| (-2.0 * PI * k as f32 / n as f32).sin()).collect(),
        }
    }

    pub(crate) fn run(&self, re: &mut [f32], im: &mut [f32]) {
        for (i, &j) in self.rev.iter().enumerate() {
            if j > i {
                re.swap(i, j);
                im.swap(i, j);
            }
        }
        let mut len = 2;
        while len <= self.n {
            let step = self.n / len;
            for start in (0..self.n).step_by(len) {
                for k in 0..len / 2 {
                    let (wr, wi) = (self.cos[k * step], self.sin[k * step]);
                    let (a, b) = (start + k, start + k + len / 2);
                    let tr = re[b] * wr - im[b] * wi;
                    let ti = re[b] * wi + im[b] * wr;
                    re[b] = re[a] - tr;
                    im[b] = im[a] - ti;
                    re[a] += tr;
                    im[a] += ti;
                }
            }
            len <<= 1;
        }
    }
}
