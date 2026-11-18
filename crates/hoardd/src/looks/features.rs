//! Computing a picture's look: its palette, its descriptor (color, edge orientations, surface statistics and local
//! binary patterns), its perceptual hash and its coarse print.

use std::collections::HashMap;
use std::f32::consts::PI;

use anyhow::Result;
use rusqlite::OptionalExtension;

use super::{Features, Swatch, dist2, oklab};

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

    Some(Features { palette, descriptor, phash: phash(&work), print: print(&work) })
}

/// L2-normalize a block and scale it by √weight, so its part of a dot product is `weight × cosine`.
fn push_block(out: &mut Vec<f32>, block: Vec<f32>, weight: f32) {
    let norm = block.iter().map(|v| v * v).sum::<f32>().sqrt();
    let k = if norm > 1e-9 { weight.sqrt() / norm } else { 0.0 };
    out.extend(block.into_iter().map(|v| v * k));
}

/// Dominant colors by weighted k-means, seeded from the fullest cells of a coarse `OKLab` grid.
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
    // Clusters that ended up near each other read as one color.
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

/// Soft histogram over 5 lightness bands × (gray, 8 muted hues, 8 vivid hues), square-rooted (Hellinger) so a
/// large flat area doesn't drown everything else.
fn color_histogram(samples: &[([f32; 3], f32)]) -> Vec<f32> {
    let mut hist = vec![0.0f32; COLOR_BINS];
    for &([l, a, b], w) in samples {
        let (l0, l1, lf) = soft_bin(l * 5.0 - 0.5, 5, false);
        let chroma = a.hypot(b);
        // Weights for gray / muted / vivid, linear between 0, 0.05 and 0.13 of chroma.
        let (gray, muted, vivid) = if chroma < 0.05 {
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
            hist[base] += lw * gray;
            hist[base + 1 + h0] += lw * muted * (1.0 - hf);
            hist[base + 1 + h1] += lw * muted * hf;
            hist[base + 9 + h0] += lw * vivid * (1.0 - hf);
            hist[base + 9 + h1] += lw * vivid * hf;
        }
    }
    hist.into_iter().map(f32::sqrt).collect()
}

/// The two bins a continuous position `t` (bin centers at 0, 1, …) falls between and how far toward the second.
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

    /// Pixels whose whole `r`-neighborhood is in the mask (so edges against a transparent background aren't
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

/// Rotation-invariant uniform local binary patterns (8 neighbors at radius `r`): how often each pixel is a spot,
/// an edge, a corner or a ridge. Square-rooted like the color histogram.
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

/// 64-bit perceptual hash: the signs of the lowest 8×8 DCT frequencies of a 32×32 gray version, relative to their
/// median. Transparent pixels read as mid gray.
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

/// A print's side, in cells.
const PRINT: usize = 12;
/// Prints at most this far apart on average (see [`print_distance`]) are of the same picture...
const PRINT_MEAN: f32 = 0.012;
/// ...as long as no cell is further apart than this: a part recolored or moved is a variant, not a copy.
const PRINT_MAX: f32 = 0.05;

/// A coarse copy of the picture: [`PRINT`] × [`PRINT`] cells (stretched to the picture's shape), each its mean color
/// over mid gray where it's see-through, in `OKLab` (L × 255, a and b × 400 + 128), and its mean opacity × 255.
fn print(work: &Work) -> Vec<u8> {
    let mut sum = vec![[0f32; 4]; PRINT * PRINT];
    let mut count = vec![0f32; PRINT * PRINT];
    for y in 0..work.h {
        for x in 0..work.w {
            let i = y * work.w + x;
            let (lab, a) = (work.lab[i], work.alpha[i]);
            let cell = (y * PRINT / work.h) * PRINT + x * PRINT / work.w;
            let s = &mut sum[cell];
            s[0] += lab[0] * a + 0.5 * (1.0 - a);
            s[1] += lab[1] * a;
            s[2] += lab[2] * a;
            s[3] += a;
            count[cell] += 1.0;
        }
    }
    sum.iter()
        .zip(&count)
        .flat_map(|(s, &n)| {
            let n = n.max(1.0);
            [s[0] / n * 255.0, s[1] / n * 400.0 + 128.0, s[2] / n * 400.0 + 128.0, s[3] / n * 255.0]
                .map(|v| v.round().clamp(0.0, 255.0) as u8)
        })
        .collect()
}

/// How far apart two prints are, as the mean and the largest distance of a cell in `OKLab` (with a difference in
/// opacity counted half). `None` when either isn't a print.
pub(super) fn print_distance(a: &[u8], b: &[u8]) -> Option<(f32, f32)> {
    if a.len() != PRINT * PRINT * 4 || b.len() != a.len() {
        return None;
    }
    let (mut sum, mut max) = (0f32, 0f32);
    for (x, y) in a.as_chunks::<4>().0.iter().zip(b.as_chunks::<4>().0) {
        let d = |i: usize, scale: f32| (f32::from(x[i]) - f32::from(y[i])) / scale;
        let cell = (d(0, 255.0).powi(2) + d(1, 400.0).powi(2) + d(2, 400.0).powi(2)).sqrt() + 0.5 * d(3, 255.0).abs();
        sum += cell;
        max = max.max(cell);
    }
    Some((sum / (PRINT * PRINT) as f32, max))
}

/// Whether two prints are of the same picture (at another size, or saved another way). `loose` allows for a model
/// rendered by another loader, which can shade it a little differently.
pub(crate) fn same_print(a: &[u8], b: &[u8], loose: bool) -> bool {
    let k = if loose { 2.0 } else { 1.0 };
    print_distance(a, b).is_some_and(|(mean, max)| mean <= PRINT_MEAN * k && max <= PRINT_MAX * k)
}

/// The prints of these thumbnails (those that have one).
pub(crate) fn prints(conn: &rusqlite::Connection, hashes: &[&str]) -> Result<HashMap<String, Vec<u8>>> {
    let mut out = HashMap::new();
    let mut stmt = conn.prepare_cached("SELECT print FROM looks WHERE thumb_hash = ?1 AND print IS NOT NULL")?;
    for &h in hashes {
        if out.contains_key(h) {
            continue;
        }
        if let Some(p) = stmt.query_row([h], |r| r.get::<_, Vec<u8>>(0)).optional()? {
            out.insert(h.to_owned(), p);
        }
    }
    Ok(out)
}
