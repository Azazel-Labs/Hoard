//! What a sound is like, for "more like this": its descriptor (timbre, envelope and character), how alike two
//! are, and the coarse print that tells a copy from another take.

use super::listening::{Frame, HIGH_HZ, MAX_SECS, mel_edges};

pub(super) const BANDS: usize = 32;
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

/// A timbre band the file's rate can't hold.
const UNKNOWN: f32 = f32::INFINITY;

#[cfg(test)]
pub(super) fn describe(mono: &[f32], rate: u32) -> Option<Vec<i8>> {
    super::listening::hear(mono, rate, false).map(|h| h.descriptor)
}

/// The descriptor (see the module's doc) of trimmed, normalized samples `x`.
pub(super) fn descriptor(x: &[f32], sr: f32, frames: &[Frame], voiced: &[&Frame], secs: f32) -> Vec<i8> {
    let nv = voiced.len() as f32;

    // Timbre: the mean spectrum in dB, floored 80 dB under its loudest band, less its mean. Bands above what the
    // file's rate can hold are unknown, not silent, so two files recorded at a low rate don't match on what neither has.
    let mut timbre = [0f32; BANDS];
    for f in voiced {
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

    let character = character(frames, voiced, secs);

    let mut out = Vec::with_capacity(DIMS);
    out.extend(
        timbre.iter().map(|&t| if t.is_finite() { (t.clamp(-1.0, 1.0) * 127.0).round() as i8 } else { i8::MIN }),
    );
    out.extend(envelope.iter().chain(character.iter()).map(|v| (v.clamp(-1.0, 1.0) * 127.0).round() as i8));
    out
}

/// A print's stretches of time.
const PRINT_SLICES: usize = 16;
/// A print's bands: pairs of mel bands, up to what a file at 16 kHz holds.
const PRINT_BANDS: usize = 12;
/// How far under the print's loudest cell its quietest can be, in dB.
const PRINT_FLOOR: f32 = 60.0;
/// Prints whose cells are at most this far apart on average, in dB, are of the same sound...
const PRINT_DB: f32 = 1.0;
/// ...when their lengths (less the silence around them) are this close, as a share or in seconds.
const PRINT_LENGTH: (f32, f32) = (0.015, 0.02);

/// A coarse spectrogram of the sound, to tell another copy of it (another rate or format, louder or quieter, with
/// more silence around it) from another take: its length in seconds (as an `f32`, little-endian), then
/// [`PRINT_SLICES`] equal stretches of [`PRINT_BANDS`] bands each, in dB under the loudest (× 4).
pub(super) fn print(frames: &[Frame], secs: f32) -> Vec<u8> {
    let n = frames.len();
    let mut cells = [[0f32; PRINT_BANDS]; PRINT_SLICES];
    for (i, row) in cells.iter_mut().enumerate() {
        let from = (i * n / PRINT_SLICES).min(n - 1);
        let to = ((i + 1) * n / PRINT_SLICES).clamp(from + 1, n);
        for f in &frames[from..to] {
            for (b, cell) in row.iter_mut().enumerate() {
                *cell += 10f32.powf(f.db[2 * b] / 10.0) + 10f32.powf(f.db[2 * b + 1] / 10.0);
            }
        }
        for cell in row.iter_mut() {
            *cell = 10.0 * (*cell / (to - from) as f32 + 1e-12).log10();
        }
    }
    let top = cells.iter().flatten().fold(f32::MIN, |m, &c| m.max(c));
    let mut out = secs.to_le_bytes().to_vec();
    out.extend(cells.iter().flatten().map(|&c| ((c - top + PRINT_FLOOR).max(0.0) * 4.0).round() as u8));
    out
}

/// Whether two prints are of the same sound (see [`print`]).
pub(crate) fn same_print(a: &[u8], b: &[u8]) -> bool {
    let len = 4 + PRINT_SLICES * PRINT_BANDS;
    if a.len() != len || b.len() != len {
        return false;
    }
    let secs = |p: &[u8]| f32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let (x, y) = (secs(a), secs(b));
    let off = (x - y).abs();
    if off > PRINT_LENGTH.1 && off > PRINT_LENGTH.0 * x.max(y) {
        return false;
    }
    // Only cells loud enough in either count: the quiet ones are what a format or noise floor changes.
    let (mut sum, mut cells) = (0f32, 0usize);
    for (p, q) in a[4..].iter().zip(&b[4..]) {
        if f32::from((*p).max(*q)) > (PRINT_FLOOR - 40.0) * 4.0 {
            sum += (f32::from(*p) - f32::from(*q)).abs() / 4.0;
            cells += 1;
        }
    }
    cells == 0 || sum / cells as f32 <= PRINT_DB
}

/// The length a print was taken over (less the silence around it), in seconds.
pub(crate) fn print_secs(p: &[u8]) -> Option<f32> {
    (p.len() == 4 + PRINT_SLICES * PRINT_BANDS).then(|| f32::from_le_bytes([p[0], p[1], p[2], p[3]]))
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

/// How alike two sounds are, about 0..=1.
pub(super) fn similarity(a: &[i8], b: &[i8]) -> f32 {
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
