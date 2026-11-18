//! What the model hears: a log-mel spectrogram of ten seconds at 48 kHz (as `ClapFeatureExtractor` makes it), laid
//! out as the square picture the audio half reads.

use std::f64::consts::PI;

pub(super) const RATE: u32 = 48_000;
pub(super) const SAMPLES: usize = 480_000;
const FFT: usize = 1024;
const HOP: usize = 480;
const BINS: usize = FFT / 2 + 1;
pub(super) const MELS: usize = 64;
const FRAMES: usize = SAMPLES / HOP + 1;
/// The spectrogram becomes a 256×256 picture: its time stretched to 1024 frames and cut in four, stacked.
pub(super) const SIDE: usize = 256;
const RATIO: usize = 4;

/// Mono samples at `rate` as the model wants them: the silence at either end left out, at 48 kHz, the first ten
/// seconds (a shorter sound repeated to fill them as often as it fits whole, then silence). `None` when silent.
pub(crate) fn prepare(mono: &[f32], rate: u32) -> Option<Vec<f32>> {
    let peak = mono.iter().fold(0f32, |m, s| m.max(s.abs()));
    if peak < 1e-4 || rate == 0 {
        return None;
    }
    let gate = peak * 0.003;
    let start = mono.iter().position(|s| s.abs() > gate)?;
    let end = mono.iter().rposition(|s| s.abs() > gate)? + 1;
    // A little more than ten seconds, so resampling has its neighbors at the cut.
    let keep = (u64::from(rate) * 10 + u64::from(rate) / 10) as usize;
    let x = &mono[start..end.min(start + keep)];
    let mut x = crate::audio::resample(x, 1, rate, RATE);
    Some(fit(&mut x))
}

/// Exactly ten seconds: the start of a longer clip, a shorter one tiled whole as often as it fits, then silence.
pub(super) fn fit(x: &mut Vec<f32>) -> Vec<f32> {
    if x.len() >= SAMPLES {
        x.truncate(SAMPLES);
        return std::mem::take(x);
    }
    let mut out = Vec::with_capacity(SAMPLES);
    if !x.is_empty() {
        for _ in 0..SAMPLES / x.len() {
            out.extend_from_slice(x);
        }
    }
    out.resize(SAMPLES, 0.0);
    out
}

/// Slaney-style triangular mel filters (librosa's defaults) from 50 Hz to 14 kHz, area-normalized, per FFT bin.
fn mel_filters() -> Vec<Vec<(usize, f32)>> {
    let hz_to_mel = |f: f64| {
        if f < 1000.0 { 3.0 * f / 200.0 } else { 15.0 + (f / 1000.0).ln() * 27.0 / 6.4f64.ln() }
    };
    let mel_to_hz = |m: f64| {
        if m < 15.0 { 200.0 * m / 3.0 } else { 1000.0 * ((m - 15.0) * 6.4f64.ln() / 27.0).exp() }
    };
    let (lo, hi) = (hz_to_mel(50.0), hz_to_mel(14_000.0));
    let edges: Vec<f64> = (0..MELS + 2).map(|i| mel_to_hz(lo + (hi - lo) * i as f64 / (MELS + 1) as f64)).collect();
    let nyquist = f64::from(RATE) / 2.0;
    (0..MELS)
        .map(|m| {
            let (left, center, right) = (edges[m], edges[m + 1], edges[m + 2]);
            let norm = 2.0 / (right - left);
            (0..BINS)
                .filter_map(|b| {
                    let f = nyquist * b as f64 / (BINS - 1) as f64;
                    let w = ((f - left) / (center - left)).min((right - f) / (right - center)).max(0.0);
                    (w > 0.0).then_some((b, (w * norm) as f32))
                })
                .collect()
        })
        .collect()
}

/// The log-mel spectrogram (decibels of power, frames × mel bands) of [`SAMPLES`] samples: a periodic Hann window of
/// 1024, hop 480, the ends padded by reflection.
pub(super) fn log_mel(x: &[f32]) -> Vec<f32> {
    debug_assert_eq!(x.len(), SAMPLES);
    let half = FFT / 2;
    let at = |i: isize| -> f32 {
        let n = x.len() as isize;
        let j = if i < 0 {
            -i
        } else if i >= n {
            2 * (n - 1) - i
        } else {
            i
        };
        x[j as usize]
    };
    let window: Vec<f32> = (0..FFT).map(|n| (0.5 - 0.5 * (2.0 * PI * n as f64 / FFT as f64).cos()) as f32).collect();
    let filters = mel_filters();
    let fft = crate::sounds::Fft::new(FFT);
    let mut out = vec![0f32; FRAMES * MELS];
    let (mut re, mut im) = (vec![0f32; FFT], vec![0f32; FFT]);
    let mut power = vec![0f32; BINS];
    for f in 0..FRAMES {
        let start = (f * HOP) as isize - half as isize;
        for n in 0..FFT {
            re[n] = at(start + n as isize) * window[n];
            im[n] = 0.0;
        }
        fft.run(&mut re, &mut im);
        for b in 0..BINS {
            power[b] = re[b] * re[b] + im[b] * im[b];
        }
        for (m, filter) in filters.iter().enumerate() {
            let e: f32 = filter.iter().map(|&(b, w)| power[b] * w).sum();
            out[f * MELS + m] = 10.0 * e.max(1e-10).log10();
        }
    }
    out
}

/// One cubic convolution weight (`PyTorch`'s bicubic, A = -0.75), `t` being the distance to the sample.
fn cubic(t: f64) -> f64 {
    const A: f64 = -0.75;
    let t = t.abs();
    if t <= 1.0 {
        ((A + 2.0) * t - (A + 3.0)) * t * t + 1.0
    } else if t < 2.0 {
        ((A * t - 5.0 * A) * t + 8.0 * A) * t - 4.0 * A
    } else {
        0.0
    }
}

/// The spectrogram as the encoder's picture: each band normalized (the encoder's batch norm), time stretched from
/// 1001 frames to 1024 (bicubic, corners aligned), then cut into four runs of 256 frames stacked top to bottom, so
/// row `run × 64 + band`, column `frame`.
pub(super) fn picture(mel: &[f32], scale: &[f32], shift: &[f32]) -> Vec<f32> {
    let time = SIDE * RATIO;
    let norm: Vec<f32> = mel
        .as_chunks::<MELS>()
        .0
        .iter()
        .flat_map(|f| f.iter().enumerate().map(|(m, v)| v * scale[m] + shift[m]))
        .collect();
    let mut out = vec![0f32; SIDE * SIDE];
    let ratio = (FRAMES - 1) as f64 / (time - 1) as f64;
    for t in 0..time {
        let src = t as f64 * ratio;
        let base = src.floor();
        let frac = src - base;
        let taps: [(usize, f32); 4] = std::array::from_fn(|k| {
            let i = (base as isize + k as isize - 1).clamp(0, FRAMES as isize - 1) as usize;
            (i, cubic(frac - (k as f64 - 1.0)) as f32)
        });
        let (run, col) = (t / SIDE, t % SIDE);
        for m in 0..MELS {
            let v: f32 = taps.iter().map(|&(i, w)| norm[i * MELS + m] * w).sum();
            out[(run * MELS + m) * SIDE + col] = v;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_sounds_repeat_whole_and_long_ones_are_cut() {
        let mut short = vec![1.0f32; 200_000];
        let out = fit(&mut short);
        assert_eq!(out.len(), SAMPLES);
        assert_eq!(out[399_999], 1.0, "twice whole");
        assert_eq!(out[400_000], 0.0, "then silence");
        let mut long: Vec<f32> = (0..600_000).map(|i| i as f32).collect();
        let out = fit(&mut long);
        assert_eq!((out.len(), out[SAMPLES - 1]), (SAMPLES, (SAMPLES - 1) as f32));
    }

    #[test]
    fn mel_filters_cover_50_hz_to_14_khz() {
        let f = mel_filters();
        assert_eq!(f.len(), MELS);
        let hz = |b: usize| f64::from(RATE) / 2.0 * b as f64 / (BINS - 1) as f64;
        assert!(hz(f[0][0].0) >= 50.0 && hz(f[0][0].0) < 120.0);
        assert!(hz(f[MELS - 1].last().unwrap().0) < 14_000.0);
        assert!(f.iter().all(|band| !band.is_empty()));
    }

    #[test]
    fn a_tone_lights_its_band() {
        let x: Vec<f32> = (0..SAMPLES).map(|i| (2.0 * PI * 1000.0 * i as f64 / f64::from(RATE)).sin() as f32).collect();
        let mel = log_mel(&x);
        assert_eq!(mel.len(), FRAMES * MELS);
        let frame = &mel[500 * MELS..501 * MELS];
        let loudest = (0..MELS).max_by(|&a, &b| frame[a].total_cmp(&frame[b])).unwrap();
        let f = mel_filters();
        let hz = |b: usize| f64::from(RATE) / 2.0 * b as f64 / (BINS - 1) as f64;
        let (lo, hi) = (hz(f[loudest][0].0), hz(f[loudest].last().unwrap().0));
        assert!(lo <= 1000.0 && 1000.0 <= hi, "band {loudest}: {lo}..{hi} Hz");
    }

    #[test]
    fn the_picture_stacks_four_runs_of_time() {
        // Every frame's value is its index, the same in every band: the picture's rows repeat in runs of 64, each run
        // a later quarter of the (stretched) time.
        let mel: Vec<f32> = (0..FRAMES).flat_map(|f| std::iter::repeat_n(f as f32, MELS)).collect();
        let p = picture(&mel, &[1.0; MELS], &[0.0; MELS]);
        assert!(p[0].abs() < 1e-4, "starts at frame 0");
        assert!((p[(3 * MELS + 5) * SIDE + SIDE - 1] - 1000.0).abs() < 1e-3, "ends at frame 1000");
        // Within what bicubic with A = -0.75 makes of a straight line.
        assert!(
            (p[MELS * SIDE] - 256.0 * 1000.0 / 1023.0).abs() < 0.1,
            "the second run picks up where the first ended"
        );
    }
}
