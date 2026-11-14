//! "More like this" for sounds: a small acoustic descriptor per audio file, so a footstep finds other footsteps and a
//! laser other lasers, whatever they're called.
//!
//! A waveform thumbnail says little about how something sounds, so this is worked out from the file itself: its first
//! [`MAX_SECS`] seconds, mixed to mono, with the silence at either end trimmed and the peak brought to full scale (so
//! how loud it was recorded doesn't matter). Stored per file (`sounds`, keyed by its hash) and searched brute force
//! from memory, like `looks`.
//!
//! - **Timbre**: the average spectrum over 32 mel bands, in decibels, less its own mean (its shape, not its level).
//! - **Envelope**: how loud it is at 16 points from its start to its end (a hit's attack and decay, a riser's swell).
//! - **Character**: ten numbers, each scaled to 0..=1: length, brightness, how much the brightness moves, noisiness,
//!   zero crossings, onsets per second after the first, how much the spectrum changes, how much of it is sound rather
//!   than near-silence, loudness range, and how much of it is bass.

use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::sync::{PoisonError, RwLock};

use anyhow::Result;
use rusqlite::params;

use crate::{AppState, db};

/// Bump when the features change; rows from another version are worked out again by the next sounds job.
pub(crate) const VERSION: i64 = 1;
/// The assets this covers.
pub(crate) const KINDS_SQL: &str = "a.kind = 'audio'";

/// How much of a file is listened to.
const MAX_SECS: f32 = 30.0;
/// How much of a file is read: 30 seconds of 48 kHz 24-bit stereo WAV, and far more of anything compressed.
const MAX_BYTES: u64 = 12 << 20;

const BANDS: usize = 32;
const ENVELOPE: usize = 16;
const CHARACTER: usize = 10;
/// Length of a descriptor.
pub(crate) const DIMS: usize = BANDS + ENVELOPE + CHARACTER;

/// How much each part counts in the similarity (they add up to 1).
const W_TIMBRE: f32 = 0.45;
const W_ENVELOPE: f32 = 0.2;
const W_CHARACTER: f32 = 0.35;
/// How much each character number counts, against the others.
const CHARACTER_WEIGHTS: [f32; CHARACTER] = [2.0, 2.0, 1.0, 1.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0];

const SIMILAR_MAX: usize = 1000;
const SIMILAR_FLOOR: f32 = 0.6;
/// A timbre band the file's rate can't hold.
const UNKNOWN: f32 = f32::INFINITY;

/// The lowest and highest frequencies the mel bands cover, whatever the file's rate (so a file recorded at a low rate
/// has empty top bands, which is how it sounds).
const LOW_HZ: f32 = 40.0;
const HIGH_HZ: f32 = 16_000.0;
/// Below this, a frame's energy counts as bass.
const BASS_HZ: f32 = 200.0;

// ---------------------------------------------------------------------------------------------
// Analysis

/// A file's descriptor, or None if it isn't audio this can decode (or is silent).
pub(crate) fn analyze(bytes: bytes::Bytes, ext: Option<&str>) -> Option<Vec<i8>> {
    let (mono, rate) = decode_mono(bytes, ext)?;
    describe(&mono, rate)
}

/// Up to [`MAX_SECS`] of a file, mixed to mono.
fn decode_mono(bytes: bytes::Bytes, ext: Option<&str>) -> Option<(Vec<f32>, u32)> {
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
    mono.truncate((MAX_SECS * rate as f32) as usize);
    Some((mono, rate))
}

/// The descriptor of mono samples at `rate`.
fn describe(mono: &[f32], rate: u32) -> Option<Vec<i8>> {
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
    let nv = voiced.len() as f32;

    // Timbre: the mean spectrum in dB, floored 80 dB under its loudest band, less its mean. Bands above what the
    // file's rate can hold are unknown, not silent, so two files recorded at a low rate don't match on what neither has.
    let mut timbre = [0f32; BANDS];
    for f in &voiced {
        for (t, d) in timbre.iter_mut().zip(f.db.iter()) {
            *t += d / nv;
        }
    }
    let edges = mel_edges();
    let held: Vec<bool> = (0..BANDS).map(|b| edges[b + 2] <= 0.95 * sr / 2.0).collect();
    let known = || timbre.iter().zip(&held).filter(|(_, h)| **h).map(|(t, _)| *t);
    let top = known().fold(f32::MIN, f32::max);
    let count = held.iter().filter(|h| **h).count().max(1) as f32;
    let mean = known().map(|t| t.max(top - 80.0)).sum::<f32>() / count;
    for (t, h) in timbre.iter_mut().zip(&held) {
        *t = if *h { (t.max(top - 80.0) - mean) / 127.0 } else { UNKNOWN };
    }

    // Envelope: the loudness of 16 equal stretches, the loudest 1.
    let mut envelope = [0f32; ENVELOPE];
    for (i, e) in envelope.iter_mut().enumerate() {
        let (a, b) = (i * x.len() / ENVELOPE, ((i + 1) * x.len() / ENVELOPE).max(i * x.len() / ENVELOPE + 1));
        let part = &x[a.min(x.len() - 1)..b.min(x.len())];
        *e = (part.iter().map(|s| s * s).sum::<f32>() / part.len().max(1) as f32).sqrt();
    }
    let most = envelope.iter().fold(0f32, |m, &e| m.max(e)).max(1e-9);
    for e in &mut envelope {
        *e /= most;
    }

    let character = character(&frames, &voiced, secs);

    let mut out = Vec::with_capacity(DIMS);
    out.extend(
        timbre.iter().map(|&t| if t.is_finite() { (t.clamp(-1.0, 1.0) * 127.0).round() as i8 } else { i8::MIN }),
    );
    out.extend(envelope.iter().chain(character.iter()).map(|v| (v.clamp(-1.0, 1.0) * 127.0).round() as i8));
    Some(out)
}

/// What one analysis frame (about 46 ms) holds.
#[derive(Debug)]
struct Frame {
    rms: f32,
    /// Energy per mel band, in dB.
    db: [f32; BANDS],
    /// Spectral centroid, in Hz.
    centroid: f32,
    /// Geometric over arithmetic mean of the power spectrum: near 1 for noise, near 0 for a tone.
    flatness: f32,
    /// Zero crossings per second.
    zcr: f32,
    /// Share of the energy under [`BASS_HZ`].
    bass: f32,
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
            }
        })
        .collect()
}

/// The ten character numbers (see the module's doc).
fn character(frames: &[Frame], voiced: &[&Frame], secs: f32) -> [f32; CHARACTER] {
    let nv = voiced.len() as f32;
    let mean = |f: &dyn Fn(&Frame) -> f32| voiced.iter().map(|v| f(v)).sum::<f32>() / nv;
    let unit = |v: f32| v.clamp(0.0, 1.0);

    let length = unit((secs / 0.05).max(1e-6).ln() / (MAX_SECS / 0.05).ln());
    let pitch: Vec<f32> = voiced.iter().map(|v| (v.centroid.max(20.0) / 100.0).log2()).collect();
    let pitch_mean = pitch.iter().sum::<f32>() / nv;
    let brightness = unit(pitch_mean / (HIGH_HZ / 100.0).log2());
    let movement = unit((pitch.iter().map(|p| (p - pitch_mean).powi(2)).sum::<f32>() / nv).sqrt() / 1.5);
    let noisiness = unit(mean(&|f| f.flatness).sqrt());
    let zcr = unit((mean(&|f| f.zcr).max(50.0) / 50.0).ln() / (10_000f32 / 50.0).ln());

    // Spectral flux between frames: how much louder the bands got, and on average how much they changed.
    let mut flux = Vec::with_capacity(frames.len());
    let mut change = 0.0;
    for w in frames.windows(2) {
        let (a, b) = (&w[0].db, &w[1].db);
        flux.push(a.iter().zip(b.iter()).map(|(x, y)| (y - x).max(0.0)).sum::<f32>());
        change += a.iter().zip(b.iter()).map(|(x, y)| (y - x).abs()).sum::<f32>() / BANDS as f32;
    }
    let change = unit(change / flux.len().max(1) as f32 / 10.0);
    let onsets = if flux.len() < 3 {
        0
    } else {
        let m = flux.iter().sum::<f32>() / flux.len() as f32;
        let sd = (flux.iter().map(|f| (f - m).powi(2)).sum::<f32>() / flux.len() as f32).sqrt();
        (1..flux.len() - 1)
            .filter(|&i| flux[i] > m + 1.5 * sd && flux[i] >= flux[i - 1] && flux[i] > flux[i + 1])
            .count()
    };
    let busy = unit(onsets.saturating_sub(1) as f32 / secs.max(0.5) / 10.0);

    let sounding = unit(nv / frames.len() as f32);
    let mut levels: Vec<f32> = frames.iter().map(|f| 20.0 * f.rms.max(1e-4).log10()).collect();
    levels.sort_by(f32::total_cmp);
    let at = |q: f32| levels[((levels.len() - 1) as f32 * q).round() as usize];
    let range = unit((at(0.95) - at(0.1)) / 60.0);
    let bass = unit(mean(&|f| f.bass).sqrt());
    [length, brightness, movement, noisiness, zcr, busy, change, sounding, range, bass]
}

/// Triangular mel filters from [`LOW_HZ`] to [`HIGH_HZ`]: for each band, the power-spectrum bins and their weights.
#[expect(clippy::many_single_char_names, reason = "a filter's left, centre and right edges")]
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
fn mel_edges() -> Vec<f32> {
    let mel = |f: f32| 2595.0 * (1.0 + f / 700.0).log10();
    let hz = |m: f32| 700.0 * (10f32.powf(m / 2595.0) - 1.0);
    let (lo, hi) = (mel(LOW_HZ), mel(HIGH_HZ));
    (0..BANDS + 2).map(|i| hz(lo + (hi - lo) * i as f32 / (BANDS + 1) as f32)).collect()
}

/// An in-place radix-2 FFT of a fixed power-of-two size.
#[derive(Debug)]
struct Fft {
    n: usize,
    rev: Vec<usize>,
    cos: Vec<f32>,
    sin: Vec<f32>,
}

impl Fft {
    fn new(n: usize) -> Self {
        let bits = n.trailing_zeros();
        Self {
            n,
            rev: (0..n).map(|i| i.reverse_bits() >> (usize::BITS - bits)).collect(),
            cos: (0..n / 2).map(|k| (-2.0 * PI * k as f32 / n as f32).cos()).collect(),
            sin: (0..n / 2).map(|k| (-2.0 * PI * k as f32 / n as f32).sin()).collect(),
        }
    }

    fn run(&self, re: &mut [f32], im: &mut [f32]) {
        for i in 0..self.n {
            let j = self.rev[i];
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

/// How alike two sounds are, about 0..=1.
fn similarity(a: &[i8], b: &[i8]) -> f32 {
    let q = |v: i8| f32::from(v) / 127.0;
    let timbre = timbre_alike(&a[..BANDS], &b[..BANDS]);
    let envelope = 1.0
        - a[BANDS..BANDS + ENVELOPE]
            .iter()
            .zip(&b[BANDS..BANDS + ENVELOPE])
            .map(|(x, y)| (q(*x) - q(*y)).abs())
            .sum::<f32>()
            / ENVELOPE as f32;
    let total: f32 = CHARACTER_WEIGHTS.iter().sum();
    let character = 1.0
        - a[BANDS + ENVELOPE..]
            .iter()
            .zip(&b[BANDS + ENVELOPE..])
            .zip(CHARACTER_WEIGHTS)
            .map(|((x, y), w)| w * (q(*x) - q(*y)).abs())
            .sum::<f32>()
            / total;
    W_TIMBRE * timbre.max(0.0) + W_ENVELOPE * envelope + W_CHARACTER * character
}

/// How alike two spectra are in shape: their correlation over the bands both files can hold (too few, and they
/// can't be compared).
fn timbre_alike(a: &[i8], b: &[i8]) -> f32 {
    let both: Vec<(f32, f32)> = a
        .iter()
        .zip(b)
        .filter(|(x, y)| **x != i8::MIN && **y != i8::MIN)
        .map(|(x, y)| (f32::from(*x), f32::from(*y)))
        .collect();
    if both.len() < BANDS / 4 {
        return 0.0;
    }
    let n = both.len() as f32;
    let (ma, mb) = (both.iter().map(|p| p.0).sum::<f32>() / n, both.iter().map(|p| p.1).sum::<f32>() / n);
    let (mut ab, mut aa, mut bb) = (0.0, 0.0, 0.0);
    for (x, y) in both {
        ab += (x - ma) * (y - mb);
        aa += (x - ma) * (x - ma);
        bb += (y - mb) * (y - mb);
    }
    if aa < 1e-6 || bb < 1e-6 {
        // A flat spectrum against another: alike if both are.
        return if aa < 1e-6 && bb < 1e-6 { 1.0 } else { 0.0 };
    }
    ab / (aa * bb).sqrt()
}

// ---------------------------------------------------------------------------------------------
// Index

/// Every sound's descriptor, in memory, loaded from the `sounds` table on first use and kept up to date by the sounds
/// job.
#[derive(Debug, Default)]
pub(crate) struct Index(RwLock<Option<HashMap<String, Box<[i8]>>>>);

impl Index {
    fn read<R>(&self, f: impl FnOnce(&HashMap<String, Box<[i8]>>) -> R) -> Option<R> {
        self.0.read().unwrap_or_else(PoisonError::into_inner).as_ref().map(f)
    }

    fn write(&self, f: impl FnOnce(&mut HashMap<String, Box<[i8]>>)) {
        if let Some(m) = self.0.write().unwrap_or_else(PoisonError::into_inner).as_mut() {
            f(m);
        }
    }

    /// Load the index if it isn't yet.
    pub(crate) async fn ensure(&self, state: &AppState) -> Result<()> {
        if self.read(|_| ()).is_some() {
            return Ok(());
        }
        let loaded = db::call(&state.pool, |c| {
            let mut stmt = c.prepare("SELECT hash, descriptor FROM sounds WHERE version = ?1")?;
            let rows = stmt
                .query_map([VERSION], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?)))?
                .filter_map(|row| {
                    let (hash, d) = row.ok()?;
                    (d.len() == DIMS).then(|| (hash, d.into_iter().map(|b| b as i8).collect()))
                })
                .collect::<HashMap<_, _>>();
            Ok(rows)
        })
        .await?;
        let mut guard = self.0.write().unwrap_or_else(PoisonError::into_inner);
        if guard.is_none() {
            tracing::debug!(sounds = loaded.len(), "sounds index loaded");
            *guard = Some(loaded);
        }
        Ok(())
    }

    /// Files that sound like `hash`'s, best first.
    pub(crate) fn similar(&self, hash: &str) -> Vec<(String, f32)> {
        self.read(|m| {
            let Some(target) = m.get(hash) else { return Vec::new() };
            let mut hits: Vec<(&String, f32)> = m
                .iter()
                .filter(|(h, _)| h.as_str() != hash)
                .map(|(h, d)| (h, similarity(target, d)))
                .filter(|(_, s)| *s >= SIMILAR_FLOOR)
                .collect();
            hits.sort_by(|a, b| b.1.total_cmp(&a.1));
            hits.truncate(SIMILAR_MAX);
            hits.into_iter().map(|(h, s)| (h.clone(), s)).collect()
        })
        .unwrap_or_default()
    }
}

// ---------------------------------------------------------------------------------------------
// Background job

const MISSING_SQL: &str = "NOT EXISTS (SELECT 1 FROM sounds s WHERE s.hash = a.blob_hash AND s.version = ?1)";
const ORPHAN_SQL: &str = "NOT EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = s.hash AND a.kind = 'audio')";

/// Queue a sounds job when some sound has no descriptor yet (or one no longer belongs to any sound).
pub(crate) async fn schedule(state: &AppState) {
    let queued = db::call(&state.pool, |c| {
        let waiting: bool = c.query_row(
            "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued')",
            [crate::jobs::SOUNDS],
            |r| r.get(0),
        )?;
        if waiting {
            return Ok(false);
        }
        let work: bool = c.query_row(
            &format!(
                "SELECT EXISTS (SELECT 1 FROM assets a WHERE {KINDS_SQL} AND {MISSING_SQL})
                     OR EXISTS (SELECT 1 FROM sounds s WHERE {ORPHAN_SQL})"
            ),
            [VERSION],
            |r| r.get(0),
        )?;
        if work {
            db::enqueue(c, crate::jobs::SOUNDS, None, &serde_json::json!({}))?;
        }
        Ok(work)
    })
    .await;
    match queued {
        Ok(true) => state.wake.notify_one(),
        Ok(false) => {}
        Err(e) => tracing::warn!(error = ?e, "could not queue the sounds job"),
    }
}

/// Files listened to per round (read, analyzed, written).
const BATCH: usize = 32;

/// Work out the descriptor of every sound that has none, and forget those of files that are no longer sounds.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    state.sounds.ensure(state).await?;
    let gone: Vec<String> = db::call(&state.pool, |c| {
        let gone = c
            .prepare(&format!("SELECT hash FROM sounds s WHERE {ORPHAN_SQL}"))?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        c.execute(&format!("DELETE FROM sounds AS s WHERE {ORPHAN_SQL}"), [])?;
        Ok(gone)
    })
    .await?;
    state.sounds.write(|m| {
        for h in &gone {
            m.remove(h);
        }
    });

    // Files that couldn't be read this time (storage unreachable): left for a later run.
    let mut unreadable = HashSet::new();
    let mut done = 0usize;
    loop {
        let skip = unreadable.clone();
        let todo: Vec<(String, String)> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT a.blob_hash, MIN(a.ext) FROM assets a WHERE {KINDS_SQL} AND {MISSING_SQL} GROUP BY a.blob_hash"
                ))?
                .query_map([VERSION], |r| Ok((r.get(0)?, r.get(1)?)))?
                .collect::<rusqlite::Result<Vec<(String, String)>>>()?;
            Ok(rows.into_iter().filter(|(h, _)| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return Ok(());
        }
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            crate::activity::job_progress(crate::jobs::SOUNDS, None, done, total, "sounds");
            let mut files = Vec::with_capacity(chunk.len());
            for (hash, ext) in chunk {
                match state.blobs.head(hash, MAX_BYTES).await {
                    Ok(bytes) => files.push((hash.clone(), ext.clone(), bytes::Bytes::from(bytes))),
                    Err(e) => {
                        tracing::debug!(hash, error = %e, "sound unreadable");
                        unreadable.insert(hash.clone());
                    }
                }
            }
            let computed = crate::heavy::run("sounds", move || analyze_all(&files)).await?;
            save(state, &computed).await?;
            done += chunk.len();
            state.sounds.write(|m| {
                for (hash, d) in computed {
                    if let Some(d) = d {
                        m.insert(hash, d.into_boxed_slice());
                    }
                }
            });
        }
        crate::activity::job_progress(crate::jobs::SOUNDS, None, done, total, "sounds");
    }
}

async fn save(state: &AppState, computed: &[(String, Option<Vec<i8>>)]) -> Result<()> {
    let rows: Vec<(String, Vec<u8>)> = computed
        .iter()
        // Not a sound this can decode: an empty row, so it isn't tried again.
        .map(|(hash, d)| (hash.clone(), d.as_ref().map(|d| d.iter().map(|&v| v as u8).collect()).unwrap_or_default()))
        .collect();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        {
            let mut stmt =
                tx.prepare("INSERT OR REPLACE INTO sounds (hash, version, descriptor) VALUES (?1, ?2, ?3)")?;
            for (hash, d) in &rows {
                stmt.execute(params![hash, VERSION, d])?;
            }
        }
        tx.commit()?;
        Ok(())
    })
    .await
}

/// Listen on a few threads (half the cores: this is background work).
fn analyze_all(files: &[(String, String, bytes::Bytes)]) -> Vec<(String, Option<Vec<i8>>)> {
    let threads = std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1));
    let per = files.len().div_ceil(threads).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = files
            .chunks(per)
            .map(|chunk| {
                scope.spawn(move || {
                    crate::heavy::lower_priority();
                    chunk.iter().map(|(h, ext, b)| (h.clone(), analyze(b.clone(), Some(ext)))).collect::<Vec<_>>()
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 44_100;

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

    /// A footstep-ish thud: a short burst of low-passed noise that dies away.
    fn thud(seed: u32, secs: f32, cutoff: f32) -> Vec<f32> {
        let mut n = Noise(seed);
        let a = (-2.0 * PI * cutoff / RATE as f32).exp();
        let mut y = 0.0;
        (0..(secs * RATE as f32) as usize)
            .map(|i| {
                y = a * y + (1.0 - a) * n.next();
                y * (-(i as f32) / (0.04 * RATE as f32)).exp() * 4.0
            })
            .collect()
    }

    /// A laser: a falling tone.
    fn zap(from: f32, to: f32, secs: f32) -> Vec<f32> {
        let len = (secs * RATE as f32) as usize;
        let mut phase = 0.0f32;
        (0..len)
            .map(|i| {
                let t = i as f32 / len as f32;
                phase += 2.0 * PI * (from + (to - from) * t) / RATE as f32;
                phase.sin() * (1.0 - t)
            })
            .collect()
    }

    /// Steady rain: white noise for a few seconds.
    fn rain(seed: u32, secs: f32) -> Vec<f32> {
        let mut n = Noise(seed);
        (0..(secs * RATE as f32) as usize).map(|_| n.next() * 0.5).collect()
    }

    fn d(x: &[f32]) -> Vec<i8> {
        describe(x, RATE).expect("a descriptor")
    }

    #[test]
    fn like_sounds_like_like() {
        let step_a = d(&thud(1, 0.3, 300.0));
        let step_b = d(&thud(7, 0.35, 350.0));
        let zap_a = d(&zap(2000.0, 300.0, 0.4));
        let zap_b = d(&zap(1800.0, 250.0, 0.5));
        let rain_a = d(&rain(3, 4.0));
        let rain_b = d(&rain(9, 5.0));

        let s = similarity;
        assert!(s(&step_a, &step_b) > s(&step_a, &zap_a), "{} vs {}", s(&step_a, &step_b), s(&step_a, &zap_a));
        assert!(s(&step_a, &step_b) > s(&step_a, &rain_a));
        assert!(s(&zap_a, &zap_b) > s(&zap_a, &step_a));
        assert!(s(&zap_a, &zap_b) > s(&zap_a, &rain_a));
        assert!(s(&rain_a, &rain_b) > s(&rain_a, &step_a));
        for (a, b) in [(&step_a, &step_b), (&zap_a, &zap_b), (&rain_a, &rain_b)] {
            assert!(s(a, b) >= SIMILAR_FLOOR, "a variation gets in: {}", s(a, b));
        }
        assert!(s(&step_a, &rain_a) < SIMILAR_FLOOR, "a thud isn't rain: {}", s(&step_a, &rain_a));
        assert!(s(&zap_a, &rain_a) < SIMILAR_FLOOR, "a laser isn't rain: {}", s(&zap_a, &rain_a));
    }

    #[test]
    fn what_a_low_rate_cant_hold_isnt_alike() {
        // At half the rate, neither file has anything above 11 kHz: that's not something they share.
        let half = |x: Vec<f32>| -> Vec<i8> {
            let x: Vec<f32> = x.iter().step_by(2).copied().collect();
            describe(&x, RATE / 2).expect("a descriptor")
        };
        let (step, rain) = (half(thud(1, 0.3, 300.0)), half(rain(3, 4.0)));
        assert!(similarity(&step, &rain) < SIMILAR_FLOOR, "a thud isn't rain: {}", similarity(&step, &rain));
        // And the same sound at either rate is still alike.
        let full = d(&thud(7, 0.35, 350.0));
        assert!(similarity(&step, &full) >= SIMILAR_FLOOR, "{}", similarity(&step, &full));
    }

    #[test]
    fn loudness_and_silence_around_it_dont_matter() {
        let step = thud(1, 0.3, 300.0);
        let mut padded = vec![0.0; RATE as usize / 2];
        padded.extend(step.iter().map(|s| s * 0.1));
        padded.extend(vec![0.0; RATE as usize]);
        assert!(similarity(&d(&step), &d(&padded)) > 0.97, "{}", similarity(&d(&step), &d(&padded)));
    }

    #[test]
    fn silence_and_the_shortest_sounds() {
        assert!(describe(&vec![0.0; 1000], RATE).is_none());
        assert_eq!(describe(&[0.5, -0.5, 0.25], RATE).map(|d| d.len()), Some(DIMS));
    }

    #[test]
    fn decodes_a_wav_cut_short() {
        let samples: Vec<i32> = thud(4, 0.5, 400.0).iter().map(|s| (s.clamp(-1.0, 1.0) * 32_767.0) as i32).collect();
        let wav = crate::audio::write_wav(&crate::audio::Audio { samples, channels: 1, rate: RATE, bits: 16 });
        let whole = analyze(bytes::Bytes::from(wav.clone()), Some("wav")).expect("the whole file");
        let cut = analyze(bytes::Bytes::from(wav[..wav.len() * 2 / 3].to_vec()), Some("wav")).expect("its start");
        assert_eq!(whole.len(), DIMS);
        assert_eq!(cut.len(), DIMS);
        assert!(analyze(bytes::Bytes::from_static(b"not a sound at all"), Some("wav")).is_none());
    }

    #[test]
    fn the_fft_finds_a_tone() {
        let n = 1024;
        let fft = Fft::new(n);
        let mut re: Vec<f32> = (0..n).map(|i| (2.0 * PI * 10.0 * i as f32 / n as f32).cos()).collect();
        let mut im = vec![0.0; n];
        fft.run(&mut re, &mut im);
        let mag: Vec<f32> = re.iter().zip(&im).map(|(r, i)| (r * r + i * i).sqrt()).collect();
        let top = mag[..n / 2].iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).map(|(k, _)| k);
        assert_eq!(top, Some(10));
        assert!((mag[10] - n as f32 / 2.0).abs() < 1.0);
    }
}
