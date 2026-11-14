//! Search by look: a dominant-colour palette and a visual descriptor per thumbnail, for "find by colour" and
//! "more like this".
//!
//! Everything is computed from an asset's thumbnail, so textures, models (their renders), materials, PSDs and videos
//! are covered the same way. Features are stored per thumbnail blob (`looks`, keyed by its hash), so a changed
//! thumbnail is simply a new row, and the same picture shared by several assets is done once.
//!
//! - **Palette**: up to eight colours found by k-means in `OKLab` (a perceptual colour space), each with the share of
//!   the picture it covers. Transparent pixels (a model render's background) don't count.
//! - **Descriptor**: 127 signed bytes, four blocks each scaled so that the dot product of two descriptors is a
//!   weighted cosine similarity: a soft colour histogram (lightness × chroma × hue), a rotation-normalized edge
//!   orientation histogram, surface statistics (contrast, detail at four scales, lightness spread, coverage), and
//!   rotation-invariant local binary patterns at two radii (the "grain" of a texture).
//! - **Perceptual hash**: 64 bits from the low frequencies of a 32×32 DCT, so near-copies rank first.
//!
//! Searching is brute force over an in-memory index (a few hundred bytes per thumbnail).

use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::{LazyLock, PoisonError, RwLock};

use anyhow::Result;
use hoard_core::SearchQuery;
use rusqlite::{OptionalExtension, params};

use crate::{AppState, db};

/// Bump when the features change; rows from another version are recomputed by the next looks job.
pub(crate) const VERSION: i64 = 1;

/// Kinds whose thumbnails are pictures of the thing (not a waveform, a type specimen or a page).
pub(crate) const KINDS_SQL: &str = "a.kind IN ('texture', 'model', 'material', 'animation', 'video', 'other')";

/// Side of the working image every feature is computed on.
const WORK: u32 = 128;
const MAX_COLORS: usize = 8;

const COLOR_BINS: usize = 5 * 17;
const ORIENT_BINS: usize = 8;
const SCALARS: usize = 7;
const LBP_BINS: usize = 10;
/// Length of a descriptor.
pub(crate) const DIMS: usize = COLOR_BINS + ORIENT_BINS + 2 * SCALARS + 2 * LBP_BINS;

/// How much each block counts in the similarity (they add up to 1).
const W_COLOR: f32 = 0.42;
const W_ORIENT: f32 = 0.13;
const W_SCALARS: f32 = 0.2;
const W_LBP: f32 = 0.25;

/// One colour of a palette, in `OKLab`, with the share of the (opaque part of the) picture it covers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Swatch {
    pub(crate) lab: [f32; 3],
    pub(crate) share: f32,
}

/// What's stored for one thumbnail.
#[derive(Clone, Debug)]
pub(crate) struct Features {
    pub(crate) palette: Vec<Swatch>,
    pub(crate) descriptor: Vec<i8>,
    pub(crate) phash: u64,
}

// ---------------------------------------------------------------------------------------------
// Colour

static SRGB_TO_LINEAR: LazyLock<[f64; 256]> = LazyLock::new(|| {
    std::array::from_fn(|i| {
        let c = i as f64 / 255.0;
        if c <= 0.040_45 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    })
});

/// sRGB bytes to `OKLab` (L in 0..=1, a and b roughly -0.3..=0.3).
#[expect(clippy::many_single_char_names, reason = "the names colour science uses")]
pub(crate) fn oklab(rgb: [u8; 3]) -> [f32; 3] {
    let [r, g, b] = rgb.map(|c| SRGB_TO_LINEAR[usize::from(c)]);
    let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).cbrt();
    let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).cbrt();
    let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).cbrt();
    [
        (0.210_454_255_3 * l + 0.793_617_785 * m - 0.004_072_046_8 * s) as f32,
        (1.977_998_495_1 * l - 2.428_592_205 * m + 0.450_593_709_9 * s) as f32,
        (0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766 * s) as f32,
    ]
}

/// `OKLab` back to sRGB bytes, clipped to the gamut.
#[expect(clippy::many_single_char_names, reason = "the names colour science uses")]
pub(crate) fn srgb(lab: [f32; 3]) -> [u8; 3] {
    let [l, a, b] = lab.map(f64::from);
    let l_ = (l + 0.396_337_777_4 * a + 0.215_803_757_3 * b).powi(3);
    let m_ = (l - 0.105_561_345_8 * a - 0.063_854_172_8 * b).powi(3);
    let s_ = (l - 0.089_484_177_5 * a - 1.291_485_548 * b).powi(3);
    let lin = [
        4.076_741_662_1 * l_ - 3.307_711_591_3 * m_ + 0.230_969_929_2 * s_,
        -1.268_438_004_6 * l_ + 2.609_757_401_1 * m_ - 0.341_319_396_5 * s_,
        -0.004_196_086_3 * l_ - 0.703_418_614_7 * m_ + 1.707_614_701 * s_,
    ];
    lin.map(|c| {
        let c = c.clamp(0.0, 1.0);
        let v = if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
        (v * 255.0).round() as u8
    })
}

pub(crate) fn hex(lab: [f32; 3]) -> String {
    let [r, g, b] = srgb(lab);
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// `#rrggbb`, `rrggbb` or `#rgb` to `OKLab`.
pub(crate) fn parse_hex(s: &str) -> Option<[f32; 3]> {
    let s = s.trim().trim_start_matches('#');
    let digits: Vec<u8> = s.chars().map(|c| c.to_digit(16).map(|d| d as u8)).collect::<Option<_>>()?;
    let rgb = match digits.as_slice() {
        [r, g, b] => [r * 17, g * 17, b * 17],
        [r1, r2, g1, g2, b1, b2] => [r1 * 16 + r2, g1 * 16 + g2, b1 * 16 + b2],
        _ => return None,
    };
    Some(oklab(rgb))
}

fn dist2(x: [f32; 3], y: [f32; 3]) -> f32 {
    (x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2) + (x[2] - y[2]).powi(2)
}

// ---------------------------------------------------------------------------------------------
// Features

/// The thumbnail as the features see it: at most [`WORK`] on its long side.
struct Work {
    w: usize,
    h: usize,
    lab: Vec<[f32; 3]>,
    alpha: Vec<f32>,
}

impl Work {
    fn decode(bytes: &[u8]) -> Option<Self> {
        let img = image::load_from_memory(bytes).ok()?.to_rgba8();
        let (w, h) = img.dimensions();
        if w == 0 || h == 0 {
            return None;
        }
        let scale = WORK as f32 / w.max(h) as f32;
        let (nw, nh) = (((w as f32 * scale).round() as u32).max(1), ((h as f32 * scale).round() as u32).max(1));
        let img = if (nw, nh) == (w, h) {
            img
        } else {
            image::imageops::resize(&img, nw, nh, image::imageops::FilterType::Triangle)
        };
        let lab = img.pixels().map(|p| oklab([p[0], p[1], p[2]])).collect();
        let alpha = img.pixels().map(|p| f32::from(p[3]) / 255.0).collect();
        Some(Self { w: nw as usize, h: nh as usize, lab, alpha })
    }

    fn opaque(&self) -> Vec<bool> {
        self.alpha.iter().map(|&a| a >= 0.9).collect()
    }
}

/// Palette, descriptor and hash of a thumbnail; `None` when it can't be decoded or is fully transparent.
pub(crate) fn analyze(bytes: &[u8]) -> Option<Features> {
    let work = Work::decode(bytes)?;
    let samples: Vec<([f32; 3], f32)> = (0..work.h)
        .step_by(2)
        .flat_map(|y| (0..work.w).step_by(2).map(move |x| y * work.w + x))
        .filter(|&i| work.alpha[i] >= 0.5)
        .map(|i| (work.lab[i], work.alpha[i]))
        .collect();
    if samples.is_empty() {
        return None;
    }
    let palette = palette(&samples);

    let opaque = work.opaque();
    let lightness: Vec<f32> = work.lab.iter().map(|p| p[0]).collect();
    let plane = Plane { w: work.w, h: work.h, v: lightness, ok: opaque };
    let mut descriptor = Vec::with_capacity(DIMS);
    push_block(&mut descriptor, color_histogram(&samples), W_COLOR);
    push_block(&mut descriptor, orientations(&plane), W_ORIENT);
    push_block(&mut descriptor, scalars(&plane, &work), W_SCALARS);
    let mut lbp_block = lbp(&plane, 1.0).to_vec();
    lbp_block.extend(lbp(&plane, 2.0));
    push_block(&mut descriptor, lbp_block, W_LBP);
    debug_assert_eq!(descriptor.len(), DIMS);
    let descriptor = descriptor.iter().map(|v| (v * 127.0).round().clamp(-127.0, 127.0) as i8).collect();

    Some(Features { palette, descriptor, phash: phash(&work) })
}

/// L2-normalize a block and scale it by √weight, so its part of a dot product is `weight × cosine`.
fn push_block(out: &mut Vec<f32>, block: Vec<f32>, weight: f32) {
    let norm = block.iter().map(|v| v * v).sum::<f32>().sqrt();
    let k = if norm > 1e-9 { weight.sqrt() / norm } else { 0.0 };
    out.extend(block.into_iter().map(|v| v * k));
}

/// Dominant colours by weighted k-means, seeded from the fullest cells of a coarse `OKLab` grid.
fn palette(samples: &[([f32; 3], f32)]) -> Vec<Swatch> {
    const CELLS: usize = 6 * 8 * 8;
    let cell = |p: [f32; 3]| {
        let l = ((p[0] * 6.0) as usize).min(5);
        let a = (((p[1] + 0.3) / 0.6 * 8.0).max(0.0) as usize).min(7);
        let b = (((p[2] + 0.3) / 0.6 * 8.0).max(0.0) as usize).min(7);
        (l * 8 + a) * 8 + b
    };
    let mut grid = vec![(0.0f32, [0.0f32; 3]); CELLS];
    let mut total = 0.0;
    for &(p, w) in samples {
        let c = &mut grid[cell(p)];
        c.0 += w;
        for (acc, v) in c.1.iter_mut().zip(p) {
            *acc += v * w;
        }
        total += w;
    }
    let mut cells: Vec<_> = grid.into_iter().filter(|c| c.0 > 0.0).collect();
    cells.sort_by(|x, y| y.0.total_cmp(&x.0));
    let mut centers: Vec<[f32; 3]> = Vec::new();
    for (w, sum) in cells {
        let mean = sum.map(|s| s / w);
        if centers.iter().all(|c| dist2(*c, mean) > 0.06 * 0.06) {
            centers.push(mean);
            if centers.len() == MAX_COLORS {
                break;
            }
        }
    }

    let mut weights = vec![0.0f32; centers.len()];
    for _ in 0..10 {
        let mut sums = vec![[0.0f32; 3]; centers.len()];
        weights.fill(0.0);
        for &(p, w) in samples {
            let k = nearest(&centers, p);
            weights[k] += w;
            for d in 0..3 {
                sums[k][d] += p[d] * w;
            }
        }
        for k in 0..centers.len() {
            if weights[k] > 0.0 {
                centers[k] = sums[k].map(|s| s / weights[k]);
            }
        }
    }

    let mut swatches: Vec<Swatch> =
        centers.into_iter().zip(weights).map(|(lab, w)| Swatch { lab, share: w / total }).collect();
    // Clusters that ended up near each other read as one colour.
    loop {
        let mut closest: Option<(usize, usize, f32)> = None;
        for i in 0..swatches.len() {
            for j in i + 1..swatches.len() {
                let d = dist2(swatches[i].lab, swatches[j].lab);
                if d < 0.045 * 0.045 && closest.is_none_or(|c| d < c.2) {
                    closest = Some((i, j, d));
                }
            }
        }
        let Some((i, j, _)) = closest else { break };
        let (a, b) = (swatches[i], swatches.swap_remove(j));
        let share = a.share + b.share;
        if share > 0.0 {
            swatches[i] =
                Swatch { lab: std::array::from_fn(|d| (a.lab[d] * a.share + b.lab[d] * b.share) / share), share };
        }
    }
    swatches.retain(|s| s.share >= 0.02);
    swatches.sort_by(|x, y| y.share.total_cmp(&x.share));
    swatches
}

fn nearest(centers: &[[f32; 3]], p: [f32; 3]) -> usize {
    let mut best = (0, f32::MAX);
    for (k, c) in centers.iter().enumerate() {
        let d = dist2(*c, p);
        if d < best.1 {
            best = (k, d);
        }
    }
    best.0
}

/// Soft histogram over 5 lightness bands × (grey, 8 muted hues, 8 vivid hues), square-rooted (Hellinger) so a
/// large flat area doesn't drown everything else.
fn color_histogram(samples: &[([f32; 3], f32)]) -> Vec<f32> {
    let mut hist = vec![0.0f32; COLOR_BINS];
    for &([l, a, b], w) in samples {
        let (l0, l1, lf) = soft_bin(l * 5.0 - 0.5, 5, false);
        let chroma = a.hypot(b);
        // Weights for grey / muted / vivid, linear between 0, 0.05 and 0.13 of chroma.
        let (grey, muted, vivid) = if chroma < 0.05 {
            let t = chroma / 0.05;
            (1.0 - t, t, 0.0)
        } else if chroma < 0.13 {
            let t = (chroma - 0.05) / 0.08;
            (0.0, 1.0 - t, t)
        } else {
            (0.0, 0.0, 1.0)
        };
        let hue = b.atan2(a).rem_euclid(2.0 * PI) / (2.0 * PI) * 8.0;
        let (h0, h1, hf) = soft_bin(hue, 8, true);
        for (lb, lw) in [(l0, 1.0 - lf), (l1, lf)] {
            let base = lb * 17;
            let lw = lw * w;
            hist[base] += lw * grey;
            hist[base + 1 + h0] += lw * muted * (1.0 - hf);
            hist[base + 1 + h1] += lw * muted * hf;
            hist[base + 9 + h0] += lw * vivid * (1.0 - hf);
            hist[base + 9 + h1] += lw * vivid * hf;
        }
    }
    hist.into_iter().map(f32::sqrt).collect()
}

/// The two bins a continuous position `t` (bin centres at 0, 1, …) falls between and how far toward the second.
fn soft_bin(t: f32, n: usize, circular: bool) -> (usize, usize, f32) {
    if circular {
        let t = t.rem_euclid(n as f32);
        let i = (t.floor() as usize) % n;
        (i, (i + 1) % n, t - t.floor())
    } else {
        let t = t.clamp(0.0, (n - 1) as f32);
        let i = (t.floor() as usize).min(n - 1);
        (i, (i + 1).min(n - 1), t - t.floor())
    }
}

/// A single-channel image with a mask of the pixels that count.
struct Plane {
    w: usize,
    h: usize,
    v: Vec<f32>,
    ok: Vec<bool>,
}

impl Plane {
    fn at(&self, x: usize, y: usize) -> f32 {
        self.v[y * self.w + x]
    }

    /// Pixels whose whole `r`-neighbourhood is in the mask (so edges against a transparent background aren't
    /// counted as surface detail).
    fn interior(&self, r: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let eroded = erode(&self.ok, self.w, self.h, r);
        (r..self.h.saturating_sub(r))
            .flat_map(move |y| (r..self.w.saturating_sub(r)).map(move |x| (x, y)))
            .filter(move |&(x, y)| eroded[y * self.w + x])
    }

    /// Half the size, averaging 2×2 blocks; a block counts when all four pixels did.
    fn half(&self) -> Self {
        let (w, h) = (self.w / 2, self.h / 2);
        let mut v = Vec::with_capacity(w * h);
        let mut ok = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                let idx = [(2 * x, 2 * y), (2 * x + 1, 2 * y), (2 * x, 2 * y + 1), (2 * x + 1, 2 * y + 1)]
                    .map(|(px, py)| py * self.w + px);
                v.push(idx.iter().map(|&i| self.v[i]).sum::<f32>() / 4.0);
                ok.push(idx.iter().all(|&i| self.ok[i]));
            }
        }
        Self { w, h, v, ok }
    }
}

fn erode(mask: &[bool], w: usize, h: usize, r: usize) -> Vec<bool> {
    let mut out = mask.to_vec();
    for _ in 0..r {
        let prev = out.clone();
        for y in 0..h {
            for x in 0..w {
                if !prev[y * w + x] {
                    continue;
                }
                let edge = x == 0 || y == 0 || x + 1 == w || y + 1 == h;
                out[y * w + x] = !edge
                    && prev[y * w + x - 1]
                    && prev[y * w + x + 1]
                    && prev[(y - 1) * w + x]
                    && prev[(y + 1) * w + x];
            }
        }
    }
    out
}

/// Sobel gradient at an interior pixel.
fn sobel(p: &Plane, x: usize, y: usize) -> (f32, f32) {
    let v = |dx: isize, dy: isize| p.at(x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
    let gx = (v(1, -1) + 2.0 * v(1, 0) + v(1, 1)) - (v(-1, -1) + 2.0 * v(-1, 0) + v(-1, 1));
    let gy = (v(-1, 1) + 2.0 * v(0, 1) + v(1, 1)) - (v(-1, -1) + 2.0 * v(0, -1) + v(1, -1));
    (gx, gy)
}

/// Edge directions weighted by strength, turned so the strongest direction comes first: bricks and planks have
/// sharp peaks, noise is flat, and a texture rotated 90° still matches.
fn orientations(p: &Plane) -> Vec<f32> {
    let mut hist = [0.0f32; ORIENT_BINS];
    for (x, y) in p.interior(1) {
        let (gx, gy) = sobel(p, x, y);
        let mag = gx.hypot(gy);
        if mag < 0.02 {
            continue;
        }
        let t = gy.atan2(gx).rem_euclid(PI) / PI * ORIENT_BINS as f32;
        let (i0, i1, f) = soft_bin(t - 0.5, ORIENT_BINS, true);
        hist[i0] += mag * (1.0 - f);
        hist[i1] += mag * f;
    }
    let top = (0..ORIENT_BINS).max_by(|&a, &b| hist[a].total_cmp(&hist[b])).unwrap_or(0);
    hist.rotate_left(top);
    hist.iter().map(|v| v.sqrt()).collect()
}

/// Contrast, how detail spreads over four scales, lightness spread and coverage, each a 0..=1 value encoded as a
/// point on a half circle so the dot product falls off smoothly as two values drift apart.
fn scalars(p: &Plane, work: &Work) -> Vec<f32> {
    let mut energies = [0.0f32; 4];
    let mut level = None::<Plane>;
    for e in &mut energies {
        let cur = level.as_ref().unwrap_or(p);
        let (mut sum, mut n) = (0.0f32, 0usize);
        for (x, y) in cur.interior(1) {
            sum += (cur.at(x + 1, y) - cur.at(x - 1, y)).abs() + (cur.at(x, y + 1) - cur.at(x, y - 1)).abs();
            n += 1;
        }
        *e = if n > 0 { sum / n as f32 } else { 0.0 };
        level = Some(cur.half());
    }
    let total: f32 = energies.iter().sum();
    let contrast = total / (total + 0.15);
    let profile = energies.map(|e| if total > 0.0 { (e / total * 2.0).min(1.0) } else { 0.25 });

    let (mut sum, mut sum2, mut n) = (0.0f32, 0.0f32, 0usize);
    for (v, ok) in p.v.iter().zip(&p.ok) {
        if *ok {
            sum += v;
            sum2 += v * v;
            n += 1;
        }
    }
    let spread = if n > 0 {
        let mean = sum / n as f32;
        ((sum2 / n as f32 - mean * mean).max(0.0).sqrt() / 0.25).min(1.0)
    } else {
        0.0
    };
    let coverage = work.alpha.iter().sum::<f32>() / work.alpha.len() as f32;

    let values = [contrast, profile[0], profile[1], profile[2], profile[3], spread, coverage];
    values.iter().flat_map(|v| [(v * PI).cos(), (v * PI).sin()]).collect()
}

/// Rotation-invariant uniform local binary patterns (8 neighbours at radius `r`): how often each pixel is a spot,
/// an edge, a corner or a ridge. Square-rooted like the colour histogram.
fn lbp(p: &Plane, r: f32) -> [f32; LBP_BINS] {
    let mut hist = [0.0f32; LBP_BINS];
    let offsets: [(f32, f32); 8] = std::array::from_fn(|k| {
        let t = k as f32 * PI / 4.0;
        (r * t.cos(), r * t.sin())
    });
    let sample = |x: f32, y: f32| {
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let (x0, y0) = (x0 as usize, y0 as usize);
        let (x1, y1) = ((x0 + 1).min(p.w - 1), (y0 + 1).min(p.h - 1));
        let top = p.at(x0, y0) * (1.0 - fx) + p.at(x1, y0) * fx;
        let bottom = p.at(x0, y1) * (1.0 - fx) + p.at(x1, y1) * fx;
        top * (1.0 - fy) + bottom * fy
    };
    for (x, y) in p.interior(r.ceil() as usize) {
        let c = p.at(x, y);
        let bits: [bool; 8] = offsets.map(|(dx, dy)| sample(x as f32 + dx, y as f32 + dy) - c > 0.01);
        let transitions = (0..8).filter(|&k| bits[k] != bits[(k + 1) % 8]).count();
        let bin = if transitions <= 2 { bits.iter().filter(|&&b| b).count() } else { 9 };
        hist[bin] += 1.0;
    }
    hist.map(f32::sqrt)
}

/// 64-bit perceptual hash: the signs of the lowest 8×8 DCT frequencies of a 32×32 grey version, relative to their
/// median. Transparent pixels read as mid grey.
fn phash(work: &Work) -> u64 {
    const N: usize = 32;
    let mut small = [[0.0f32; N]; N];
    let mut counts = [[0.0f32; N]; N];
    for y in 0..work.h {
        for x in 0..work.w {
            let i = y * work.w + x;
            let v = work.lab[i][0] * work.alpha[i] + 0.5 * (1.0 - work.alpha[i]);
            let (sx, sy) = (x * N / work.w, y * N / work.h);
            small[sy][sx] += v;
            counts[sy][sx] += 1.0;
        }
    }
    for (row, count) in small.iter_mut().zip(&counts) {
        for (v, c) in row.iter_mut().zip(count) {
            if *c > 0.0 {
                *v /= c;
            }
        }
    }
    let cos: Vec<[f32; N]> =
        (0..8).map(|u| std::array::from_fn(|x| ((2 * x + 1) as f32 * u as f32 * PI / (2 * N) as f32).cos())).collect();
    // Rows first, then columns.
    let rows: Vec<[f32; 8]> =
        small.iter().map(|row| std::array::from_fn(|u| row.iter().zip(&cos[u]).map(|(v, c)| v * c).sum())).collect();
    let mut coef = [0.0f32; 64];
    for v in 0..8 {
        for u in 0..8 {
            coef[v * 8 + u] = (0..N).map(|y| rows[y][u] * cos[v][y]).sum();
        }
    }
    let mut ac: Vec<f32> = coef[1..].to_vec();
    ac.sort_by(f32::total_cmp);
    let median = ac[ac.len() / 2];
    coef.iter().enumerate().fold(0u64, |h, (i, &c)| if c > median { h | (1 << i) } else { h })
}

// ---------------------------------------------------------------------------------------------
// Storage

/// Four bytes a colour: L × 255, a × 400, b × 400 (signed), share × 255.
pub(crate) fn encode_palette(palette: &[Swatch]) -> Vec<u8> {
    palette
        .iter()
        .flat_map(|s| {
            [
                (s.lab[0] * 255.0).round().clamp(0.0, 255.0) as u8,
                (s.lab[1] * 400.0).round().clamp(-127.0, 127.0) as i8 as u8,
                (s.lab[2] * 400.0).round().clamp(-127.0, 127.0) as i8 as u8,
                (s.share * 255.0).round().clamp(0.0, 255.0) as u8,
            ]
        })
        .collect()
}

pub(crate) fn decode_palette(bytes: &[u8]) -> Vec<Swatch> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| Swatch {
            lab: [f32::from(c[0]) / 255.0, f32::from(c[1] as i8) / 400.0, f32::from(c[2] as i8) / 400.0],
            share: f32::from(c[3]) / 255.0,
        })
        .collect()
}

/// The palette of a thumbnail as shown on an asset's page, most of the picture first.
pub(crate) fn palette_of(conn: &rusqlite::Connection, thumb_hash: &str) -> Result<Vec<hoard_core::Swatch>> {
    let bytes: Option<Vec<u8>> =
        conn.query_row("SELECT palette FROM looks WHERE thumb_hash = ?1", [thumb_hash], |r| r.get(0)).optional()?;
    Ok(bytes
        .map(|b| decode_palette(&b))
        .unwrap_or_default()
        .into_iter()
        .map(|s| hoard_core::Swatch { hex: hex(s.lab), share: (s.share * 100.0).round() / 100.0 })
        .collect())
}

// ---------------------------------------------------------------------------------------------
// Index and ranking

/// "More like this" keeps at most this many matches, and only those scoring at least the floor.
const SIMILAR_MAX: usize = 1000;
const SIMILAR_FLOOR: f32 = 0.55;
/// How much what a picture shows counts in "more like this", against how it looks, when image search is on.
const MEANING_WEIGHT: f32 = 0.7;
/// "Find by colour" keeps assets with at least about this share of the picture near the colour.
const COLOR_MAX: usize = 3000;
const COLOR_FLOOR: f32 = 0.08;
/// How far (in `OKLab`) a colour still counts as the one asked for, and how little lightness matters next to hue.
const COLOR_SIGMA: f32 = 0.08;
const COLOR_L_WEIGHT: f32 = 0.5;

#[derive(Debug)]
struct Entry {
    palette: Vec<Swatch>,
    descriptor: Box<[i8]>,
    phash: u64,
}

impl Entry {
    fn from(f: Features) -> Option<Self> {
        (f.descriptor.len() == DIMS).then(|| Self {
            palette: f.palette,
            descriptor: f.descriptor.into_boxed_slice(),
            phash: f.phash,
        })
    }
}

/// Every thumbnail's look, in memory, loaded from the `looks` table on first use and kept up to date by the looks
/// job.
#[derive(Debug, Default)]
pub(crate) struct Index(RwLock<Option<HashMap<String, Entry>>>);

impl Index {
    fn read<R>(&self, f: impl FnOnce(&HashMap<String, Entry>) -> R) -> Option<R> {
        self.0.read().unwrap_or_else(PoisonError::into_inner).as_ref().map(f)
    }

    fn write(&self, f: impl FnOnce(&mut HashMap<String, Entry>)) {
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
            let mut stmt = c.prepare("SELECT thumb_hash, palette, descriptor, phash FROM looks WHERE version = ?1")?;
            let rows = stmt
                .query_map([VERSION], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?, r.get::<_, Vec<u8>>(2)?, r.get::<_, i64>(3)?))
                })?
                .filter_map(|row| {
                    let (hash, palette, descriptor, phash) = row.ok()?;
                    let entry = Entry::from(Features {
                        palette: decode_palette(&palette),
                        descriptor: descriptor.into_iter().map(|b| b as i8).collect(),
                        phash: phash as u64,
                    })?;
                    Some((hash, entry))
                })
                .collect::<HashMap<_, _>>();
            Ok(rows)
        })
        .await?;
        let mut guard = self.0.write().unwrap_or_else(PoisonError::into_inner);
        if guard.is_none() {
            tracing::debug!(looks = loaded.len(), "looks index loaded");
            *guard = Some(loaded);
        }
        Ok(())
    }

    fn insert(&self, hash: String, features: Features) {
        if let Some(entry) = Entry::from(features) {
            self.write(|m| {
                m.insert(hash, entry);
            });
        }
    }

    fn remove(&self, hashes: &[String]) {
        self.write(|m| {
            for h in hashes {
                m.remove(h);
            }
        });
    }

    /// Thumbnails that look like `hash`'s, best first. With image search's scores (`meaning`, by what the pictures
    /// show), those count most where a thumbnail has one; colours and grain decide the rest.
    fn similar(&self, hash: &str, meaning: Option<&HashMap<String, f32>>) -> Vec<(String, f32)> {
        self.read(|m| {
            let Some(target) = m.get(hash) else { return Vec::new() };
            let scored = m.iter().filter(|(h, _)| h.as_str() != hash).map(|(h, e)| {
                let look = similarity(target, e);
                let score = match meaning.and_then(|s| s.get(h)) {
                    // Rescaled so the same floor applies: a close look alone still gets in.
                    Some(&clip) => {
                        MEANING_WEIGHT * (SIMILAR_FLOOR + (1.0 - SIMILAR_FLOOR) * clip) + (1.0 - MEANING_WEIGHT) * look
                    }
                    None => look,
                };
                (h, score)
            });
            top(scored, SIMILAR_FLOOR, SIMILAR_MAX)
        })
        .unwrap_or_default()
    }

    /// Thumbnails with much of `lab` in them, most first.
    fn by_color(&self, lab: [f32; 3]) -> Vec<(String, f32)> {
        self.read(|m| top(m.iter().map(|(h, e)| (h, color_score(&e.palette, lab))), COLOR_FLOOR, COLOR_MAX))
            .unwrap_or_default()
    }
}

fn top<'a>(scored: impl Iterator<Item = (&'a String, f32)>, floor: f32, max: usize) -> Vec<(String, f32)> {
    let mut hits: Vec<(&String, f32)> = scored.filter(|(_, s)| *s >= floor).collect();
    hits.sort_by(|a, b| b.1.total_cmp(&a.1));
    hits.truncate(max);
    hits.into_iter().map(|(h, s)| (h.clone(), s)).collect()
}

/// How alike two thumbnails look, about 0..=1: the descriptor's weighted cosine, plus a bonus for near-identical
/// perceptual hashes.
fn similarity(a: &Entry, b: &Entry) -> f32 {
    let dot: i32 = a.descriptor.iter().zip(b.descriptor.iter()).map(|(x, y)| i32::from(*x) * i32::from(*y)).sum();
    let descriptor = dot as f32 / (127.0 * 127.0);
    let distance = (a.phash ^ b.phash).count_ones() as f32;
    let near_copy = ((20.0 - distance) / 20.0).clamp(0.0, 1.0);
    0.85 * descriptor + 0.15 * near_copy
}

/// How much of a picture is close to `lab`: each palette colour's share, counted less the further it is.
fn color_score(palette: &[Swatch], lab: [f32; 3]) -> f32 {
    palette
        .iter()
        .map(|s| {
            let d2 = (COLOR_L_WEIGHT * (s.lab[0] - lab[0])).powi(2)
                + (s.lab[1] - lab[1]).powi(2)
                + (s.lab[2] - lab[2]).powi(2);
            s.share * (-d2 / (COLOR_SIGMA * COLOR_SIGMA)).exp()
        })
        .sum()
}

// ---------------------------------------------------------------------------------------------
// Search

/// A library search, ranked by look when it asks for "more like this" (`similar`) or a colour (`color`).
pub(crate) async fn search(state: &AppState, q: SearchQuery, facets: bool) -> Result<db::SearchPage> {
    let rank = rank(state, &q).await?;
    db::call(&state.pool, move |c| db::search_ranked(c, &q, facets, rank.as_ref())).await
}

async fn rank(state: &AppState, q: &SearchQuery) -> Result<Option<db::LookRank>> {
    let similar = q.similar.clone().filter(|s| !s.is_empty());
    let color = q.color.as_deref().filter(|c| !c.is_empty()).map(parse_hex);
    if similar.is_none() && color.is_none() {
        // Words alone: with image search on, pictures that show them match too, beside names that have them.
        let text = q.q.as_deref().unwrap_or("").trim();
        if text.is_empty() {
            return Ok(None);
        }
        let hits = match crate::clip::search_text(state, text).await {
            Ok(hits) => hits,
            Err(e) => {
                tracing::warn!(error = %format!("{e:#}"), "image search failed; searching names only");
                Vec::new()
            }
        };
        return Ok((!hits.is_empty()).then_some(db::LookRank {
            scores: hits,
            exclude: None,
            blend: true,
            by_file: false,
        }));
    }
    let mut scores: Option<Vec<(String, f32)>> = None;
    if let Some(id) = &similar {
        let id = id.clone();
        let asset: Option<(String, String, Option<String>)> = db::call(&state.pool, move |c| {
            Ok(c.query_row("SELECT kind, blob_hash, thumb_hash FROM assets WHERE id = ?1", [id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .optional()?)
        })
        .await?;
        let (kind, file, thumb) = asset.unwrap_or_default();
        // A sound: what sounds like it (a waveform's colours say nothing).
        if kind == "audio" {
            state.sounds.ensure(state).await?;
            let scores = state.sounds.similar(&file);
            return Ok(Some(db::LookRank { scores, exclude: similar, blend: false, by_file: true }));
        }
        state.looks.ensure(state).await?;
        scores = Some(match thumb {
            Some(h) => {
                let meaning = crate::clip::similar(state, &h).await;
                state.looks.similar(&h, meaning.as_ref())
            }
            None => Vec::new(),
        });
    }
    if let Some(color) = color {
        state.looks.ensure(state).await?;
        // A colour that doesn't parse matches nothing rather than everything.
        let by_color = color.map(|lab| state.looks.by_color(lab)).unwrap_or_default();
        scores = Some(match scores {
            // Both: what looks alike and has the colour, by both scores.
            Some(similar) => {
                let by_color: HashMap<String, f32> = by_color.into_iter().collect();
                similar.into_iter().filter_map(|(h, s)| by_color.get(&h).map(|c| (h, s + c))).collect()
            }
            None => by_color,
        });
    }
    Ok(Some(db::LookRank { scores: scores.unwrap_or_default(), exclude: similar, blend: false, by_file: false }))
}

// ---------------------------------------------------------------------------------------------
// Background job

const MISSING_SQL: &str = "NOT EXISTS (SELECT 1 FROM looks l WHERE l.thumb_hash = a.thumb_hash AND l.version = ?1)";
const ORPHAN_SQL: &str = "NOT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash = l.thumb_hash)";

/// Queue a looks job when some thumbnail has no look yet (or one no longer belongs to any asset).
pub(crate) async fn schedule(state: &AppState) {
    let queued = db::call(&state.pool, |c| {
        let waiting: bool = c.query_row(
            "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued')",
            [crate::jobs::LOOKS],
            |r| r.get(0),
        )?;
        if waiting {
            return Ok(false);
        }
        let work: bool = c.query_row(
            &format!(
                "SELECT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash IS NOT NULL AND {KINDS_SQL} AND {MISSING_SQL})
                     OR EXISTS (SELECT 1 FROM looks l WHERE {ORPHAN_SQL})"
            ),
            [VERSION],
            |r| r.get(0),
        )?;
        if work {
            db::enqueue(c, crate::jobs::LOOKS, None, &serde_json::json!({}))?;
        }
        Ok(work)
    })
    .await;
    match queued {
        Ok(true) => state.wake.notify_one(),
        Ok(false) => {}
        Err(e) => tracing::warn!(error = ?e, "could not queue the looks job"),
    }
    crate::clip::schedule(state).await;
    crate::sounds::schedule(state).await;
}

/// Thumbnails done per round (read, analyzed, written).
const BATCH: usize = 96;

/// Work out the look of every thumbnail that has none, and forget the looks of thumbnails nothing uses.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    state.looks.ensure(state).await?;
    let gone: Vec<String> = db::call(&state.pool, |c| {
        let gone = c
            .prepare(&format!("SELECT thumb_hash FROM looks l WHERE {ORPHAN_SQL}"))?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        c.execute(&format!("DELETE FROM looks AS l WHERE {ORPHAN_SQL}"), [])?;
        Ok(gone)
    })
    .await?;
    state.looks.remove(&gone);

    // Thumbnails that couldn't be read this time (storage unreachable): left for a later run.
    let mut unreadable = std::collections::HashSet::new();
    let mut done = 0usize;
    loop {
        let skip = unreadable.clone();
        let todo: Vec<String> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT DISTINCT a.thumb_hash FROM assets a
                      WHERE a.thumb_hash IS NOT NULL AND {KINDS_SQL} AND {MISSING_SQL}"
                ))?
                .query_map([VERSION], |r| r.get(0))?
                .collect::<rusqlite::Result<Vec<String>>>()?;
            Ok(rows.into_iter().filter(|h| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return Ok(());
        }
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            crate::activity::job_progress(crate::jobs::LOOKS, None, done, total, "pictures");
            let mut pictures = Vec::with_capacity(chunk.len());
            for hash in chunk {
                match state.blobs.read_all(hash).await {
                    Ok(bytes) => pictures.push((hash.clone(), bytes)),
                    Err(e) => {
                        tracing::debug!(hash, error = %e, "thumbnail unreadable for looks");
                        unreadable.insert(hash.clone());
                    }
                }
            }
            let computed = crate::heavy::run("looks", move || analyze_all(&pictures)).await?;
            save(state, &computed).await?;
            done += chunk.len();
            for (hash, f) in computed {
                if let Some(f) = f {
                    state.looks.insert(hash, f);
                }
            }
        }
        crate::activity::job_progress(crate::jobs::LOOKS, None, done, total, "pictures");
    }
}

async fn save(state: &AppState, computed: &[(String, Option<Features>)]) -> Result<()> {
    let rows: Vec<(String, Vec<u8>, Vec<u8>, i64)> = computed
        .iter()
        .map(|(hash, f)| match f {
            Some(f) => (
                hash.clone(),
                encode_palette(&f.palette),
                f.descriptor.iter().map(|&v| v as u8).collect(),
                f.phash as i64,
            ),
            // Not a picture after all: an empty row, so it isn't tried again.
            None => (hash.clone(), Vec::new(), Vec::new(), 0),
        })
        .collect();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO looks (thumb_hash, version, palette, descriptor, phash) VALUES (?1, ?2, ?3, ?4, ?5)",
            )?;
            for (hash, palette, descriptor, phash) in &rows {
                stmt.execute(params![hash, VERSION, palette, descriptor, phash])?;
            }
        }
        tx.commit()?;
        Ok(())
    })
    .await
}

/// Analyze pictures on a few threads (half the cores: this is background work).
fn analyze_all(pictures: &[(String, bytes::Bytes)]) -> Vec<(String, Option<Features>)> {
    let threads = std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1));
    let per = pictures.len().div_ceil(threads).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = pictures
            .chunks(per)
            .map(|chunk| {
                scope.spawn(move || {
                    crate::heavy::lower_priority();
                    chunk.iter().map(|(h, b)| (h.clone(), analyze(b))).collect::<Vec<_>>()
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(img: &image::RgbaImage) -> Vec<u8> {
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }

    /// Cheap deterministic value noise in 0..=1.
    fn noise(seed: u32, x: u32, y: u32, scale: f32) -> f32 {
        let hash = |ix: i64, iy: i64| {
            let mut h =
                (ix as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (iy as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
            h ^= u64::from(seed).wrapping_mul(0x1656_67B1_9E37_79F9);
            h ^= h >> 29;
            h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
            h ^= h >> 32;
            (h & 0xFFFF) as f32 / 65535.0
        };
        let (fx, fy) = (x as f32 / scale, y as f32 / scale);
        let (ix, iy) = (fx.floor() as i64, fy.floor() as i64);
        let (tx, ty) = (fx - fx.floor(), fy - fy.floor());
        let top = hash(ix, iy) * (1.0 - tx) + hash(ix + 1, iy) * tx;
        let bottom = hash(ix, iy + 1) * (1.0 - tx) + hash(ix + 1, iy + 1) * tx;
        top * (1.0 - ty) + bottom * ty
    }

    fn rock(seed: u32, tint: [f32; 3]) -> image::RgbaImage {
        image::RgbaImage::from_fn(256, 256, |x, y| {
            let v = 0.5 * noise(seed, x, y, 24.0) + 0.3 * noise(seed + 1, x, y, 8.0) + 0.2 * noise(seed + 2, x, y, 3.0);
            let c = tint.map(|t| (t * (0.4 + v) * 255.0).clamp(0.0, 255.0) as u8);
            image::Rgba([c[0], c[1], c[2], 255])
        })
    }

    fn stripes(period: u32, vertical: bool, a: [u8; 3], b: [u8; 3]) -> image::RgbaImage {
        image::RgbaImage::from_fn(256, 256, |x, y| {
            let t = if vertical { x } else { y };
            let c = if (t / period).is_multiple_of(2) { a } else { b };
            image::Rgba([c[0], c[1], c[2], 255])
        })
    }

    fn entry(img: &image::RgbaImage) -> Entry {
        Entry::from(analyze(&png(img)).unwrap()).unwrap()
    }

    #[test]
    fn colours_round_trip_through_oklab() {
        for hex_in in ["#3a7d2c", "#000000", "#ffffff", "#ff0000", "#1e90ff"] {
            assert_eq!(hex(parse_hex(hex_in).unwrap()), hex_in);
        }
        assert_eq!(parse_hex("f00").map(hex).as_deref(), Some("#ff0000"));
        assert!(parse_hex("#12345").is_none());
        assert!(parse_hex("red").is_none());
    }

    #[test]
    fn palette_finds_the_colours_and_their_shares() {
        let img = image::RgbaImage::from_fn(200, 100, |x, _| {
            if x < 150 { image::Rgba([200, 30, 30, 255]) } else { image::Rgba([30, 60, 200, 255]) }
        });
        let f = analyze(&png(&img)).unwrap();
        assert_eq!(f.palette.len(), 2, "{:?}", f.palette);
        assert!((f.palette[0].share - 0.75).abs() < 0.05, "{:?}", f.palette);
        assert!(dist2(f.palette[0].lab, oklab([200, 30, 30])) < 0.001);
        assert!(dist2(f.palette[1].lab, oklab([30, 60, 200])) < 0.001);

        let stored = decode_palette(&encode_palette(&f.palette));
        assert_eq!(stored.len(), 2);
        assert!(dist2(stored[0].lab, f.palette[0].lab) < 0.0001);
        assert!((stored[0].share - f.palette[0].share).abs() < 0.01);
    }

    #[test]
    fn transparent_background_is_left_out() {
        let img = image::RgbaImage::from_fn(128, 128, |x, y| {
            if (32..96).contains(&x) && (32..96).contains(&y) {
                image::Rgba([40, 160, 60, 255])
            } else {
                image::Rgba([0, 0, 0, 0])
            }
        });
        let f = analyze(&png(&img)).unwrap();
        assert_eq!(f.palette.len(), 1, "{:?}", f.palette);
        assert!(f.palette[0].share > 0.95);
        assert!(analyze(&png(&image::RgbaImage::new(16, 16))).is_none(), "nothing opaque");
        assert!(analyze(b"not a picture").is_none());
    }

    #[test]
    fn colour_search_scores_by_how_much_of_the_picture_is_that_colour() {
        let green = oklab([40, 160, 60]);
        let palette = [Swatch { lab: green, share: 0.6 }, Swatch { lab: oklab([200, 200, 200]), share: 0.4 }];
        let exact = color_score(&palette, green);
        assert!((exact - 0.6).abs() < 0.05, "{exact}");
        assert!(color_score(&palette, oklab([60, 170, 70])) > 0.4, "a nearby shade still counts");
        assert!(color_score(&palette, oklab([200, 30, 30])) < 0.02, "red isn't green");
    }

    #[test]
    fn like_looks_like_like() {
        let brown = [0.55, 0.42, 0.3];
        let grey = [0.5, 0.5, 0.5];
        let rock_a = entry(&rock(1, brown));
        let rock_b = entry(&rock(7, brown));
        let grey_rock = entry(&rock(3, grey));
        let planks = entry(&stripes(16, false, [150, 110, 70], [110, 75, 45]));
        let planks_turned = entry(&stripes(16, true, [150, 110, 70], [110, 75, 45]));
        let blue_tiles = entry(&stripes(8, true, [30, 60, 200], [220, 220, 240]));

        let same_kind = similarity(&rock_a, &rock_b);
        assert!(same_kind > SIMILAR_FLOOR, "{same_kind}");
        assert!(same_kind > similarity(&rock_a, &grey_rock), "colour counts");
        assert!(similarity(&rock_a, &grey_rock) > similarity(&rock_a, &blue_tiles), "and so does surface");
        assert!(same_kind > similarity(&rock_a, &planks));
        // Only the perceptual hash (which is for near-copies) sees the turn.
        assert!(similarity(&planks, &planks_turned) > 0.75, "turning a texture doesn't change what it looks like");
        assert!(similarity(&rock_a, &blue_tiles) < SIMILAR_FLOOR);
    }

    #[test]
    fn near_copies_share_a_perceptual_hash() {
        let img = rock(5, [0.6, 0.5, 0.4]);
        let small = image::imageops::resize(&img, 100, 100, image::imageops::FilterType::Triangle);
        let a = analyze(&png(&img)).unwrap();
        let b = analyze(&png(&small)).unwrap();
        let c = analyze(&png(&rock(6, [0.6, 0.5, 0.4]))).unwrap();
        assert!((a.phash ^ b.phash).count_ones() <= 6, "{}", (a.phash ^ b.phash).count_ones());
        assert!((a.phash ^ c.phash).count_ones() >= 16, "{}", (a.phash ^ c.phash).count_ones());
    }
}
