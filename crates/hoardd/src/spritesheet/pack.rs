//! Packing sprites onto pages again, for a format that can't say how the sheet's sprites are turned or trimmed, or
//! when each sprite is an image of its own.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

use std::collections::{HashMap, HashSet};

use image::{RgbaImage, imageops};

use super::{Anim, AnimFrame, MAX_PAGE, PAD, Page, Rot, Sheet, Sprite, cut, whole};

/// Free space in a page being packed (`MaxRects`).
struct Bin {
    w: u32,
    h: u32,
    free: Vec<[u32; 4]>,
}

impl Bin {
    fn new(w: u32, h: u32) -> Self {
        Bin { w, h, free: vec![[0, 0, w, h]] }
    }

    /// Where a `w`×`h` box fits best (the least left over on its shorter side), and whether it's turned to fit.
    fn find(&self, w: u32, h: u32, turn: bool) -> Option<(u32, u32, bool)> {
        // The best so far: how much it leaves over (the shorter side first), and where.
        let mut best: Option<([u32; 2], (u32, u32, bool))> = None;
        for f in &self.free {
            for (bw, bh, turned) in [(w, h, false), (h, w, true)] {
                if (turned && (!turn || w == h)) || bw > f[2] || bh > f[3] {
                    continue;
                }
                let (a, b) = (f[2] - bw, f[3] - bh);
                let score = [a.min(b), a.max(b)];
                if best.is_none_or(|(s, _)| score < s) {
                    best = Some((score, (f[0], f[1], turned)));
                }
            }
        }
        best.map(|(_, at)| at)
    }

    fn place(&mut self, r: [u32; 4]) {
        let [x, y, w, h] = r;
        let mut next = Vec::with_capacity(self.free.len() + 4);
        for &f in &self.free {
            let [fx, fy, fw, fh] = f;
            if x >= fx + fw || x + w <= fx || y >= fy + fh || y + h <= fy {
                next.push(f);
                continue;
            }
            if x > fx {
                next.push([fx, fy, x - fx, fh]);
            }
            if x + w < fx + fw {
                next.push([x + w, fy, fx + fw - x - w, fh]);
            }
            if y > fy {
                next.push([fx, fy, fw, y - fy]);
            }
            if y + h < fy + fh {
                next.push([fx, y + h, fw, fy + fh - y - h]);
            }
        }
        // Drop the free rectangles inside others.
        let inside = |a: &[u32; 4], b: &[u32; 4]| {
            a[0] >= b[0] && a[1] >= b[1] && a[0] + a[2] <= b[0] + b[2] && a[1] + a[3] <= b[1] + b[3]
        };
        let mut kept: Vec<[u32; 4]> = Vec::with_capacity(next.len());
        for (i, a) in next.iter().enumerate() {
            let covered = next.iter().enumerate().any(|(j, b)| i != j && inside(a, b) && (a != b || j < i));
            if !covered {
                kept.push(*a);
            }
        }
        self.free = kept;
    }
}

/// Where packing put a box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Spot {
    page: usize,
    x: u32,
    y: u32,
    turned: bool,
}

/// Pack boxes onto as few pages as fit, each at most [`MAX_PAGE`] a side (a box bigger than that gets a page of its
/// own), [`PAD`] pixels apart, turned when `turn` allows and it fits better. Returns each page's size and each box's
/// spot.
fn pack(sizes: &[[u32; 2]], turn: bool) -> (Vec<[u32; 2]>, Vec<Spot>) {
    // Not on a page until placed.
    let mut spots = vec![Spot { page: usize::MAX, x: 0, y: 0, turned: false }; sizes.len()];
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse((sizes[i][0].max(sizes[i][1]), sizes[i][0] * sizes[i][1])));
    let mut pages = Vec::new();
    let mut left = order;
    while !left.is_empty() {
        let area: u64 = left.iter().map(|&i| u64::from(sizes[i][0] + PAD) * u64::from(sizes[i][1] + PAD)).sum();
        let widest =
            left.iter().map(|&i| sizes[i][0].min(if turn { sizes[i][1] } else { u32::MAX })).max().unwrap_or(1);
        let mut side = ((area as f64).sqrt() as u32).max(widest).max(16).next_power_of_two();
        let mut tries = Vec::new();
        while side <= MAX_PAGE {
            tries.push([side, side / 2]);
            tries.push([side, side]);
            side *= 2;
        }
        let page = pages.len();
        let mut done = false;
        for [w, h] in tries {
            if let Some(placed) = try_pack(&left, sizes, w, h, turn, true) {
                for (i, s) in placed {
                    spots[i] = Spot { page, ..s };
                }
                left.clear();
                done = true;
                break;
            }
        }
        if !done {
            let placed = try_pack(&left, sizes, MAX_PAGE, MAX_PAGE, turn, false).unwrap_or_default();
            if placed.is_empty() {
                // Too big for any page: one of its own.
                let i = left.remove(0);
                spots[i] = Spot { page, x: 0, y: 0, turned: false };
            } else {
                let on: HashSet<usize> = placed.iter().map(|(i, _)| *i).collect();
                left.retain(|i| !on.contains(i));
                for (i, s) in placed {
                    spots[i] = Spot { page, ..s };
                }
            }
        }
        // The page is as big as what's on it.
        let mut extent = [1, 1];
        for (i, s) in spots.iter().enumerate() {
            if s.page == page {
                let [w, h] = if s.turned { [sizes[i][1], sizes[i][0]] } else { sizes[i] };
                extent = [extent[0].max(s.x + w), extent[1].max(s.y + h)];
            }
        }
        pages.push(extent);
    }
    (pages, spots)
}

/// Pack `items` into a `w`×`h` page. With `all`, every one must fit, or `None`.
fn try_pack(items: &[usize], sizes: &[[u32; 2]], w: u32, h: u32, turn: bool, all: bool) -> Option<Vec<(usize, Spot)>> {
    let mut bin = Bin::new(w + PAD, h + PAD);
    let mut out = Vec::new();
    for &i in items {
        let [iw, ih] = sizes[i];
        match bin.find(iw + PAD, ih + PAD, turn) {
            Some((x, y, turned)) => {
                let (bw, bh) = if turned { (ih + PAD, iw + PAD) } else { (iw + PAD, ih + PAD) };
                bin.place([x, y, bw, bh]);
                out.push((i, Spot { page: 0, x, y, turned }));
            }
            None if all => return None,
            None => {}
        }
    }
    debug_assert!(bin.w > 0 && bin.h > 0);
    Some(out)
}

/// A sprite as repacked: its unique image, its canvas size and where the image sits on it.
type Repacked = (usize, [u32; 2], [i64; 2]);

/// Pack the sheet's sprites again: each its upright pixels (trimmed again of any transparent margin when the format
/// can say so, otherwise its whole canvas), identical ones stored once, turned only the way the format can say.
pub(super) fn repack(sheet: &Sheet, images: &[RgbaImage], rot: Rot, trim: bool) -> (Sheet, Vec<RgbaImage>) {
    let mut uniques: Vec<RgbaImage> = Vec::new();
    let mut by_hash: HashMap<blake3::Hash, usize> = HashMap::new();
    let mut each: Vec<Option<Repacked>> = Vec::with_capacity(sheet.sprites.len());
    for s in &sheet.sprites {
        let Some(page) = images.get(s.page) else {
            each.push(None);
            continue;
        };
        let got = if trim {
            cut(page, s).map(|px| {
                let (px, [dx, dy]) = trim_transparent(px);
                (px, s.size, [s.at[0] + dx, s.at[1] + dy])
            })
        } else {
            whole(page, s).map(|px| {
                let size = [px.width(), px.height()];
                (px, size, [0, 0])
            })
        };
        let Some((px, size, at)) = got else {
            each.push(None);
            continue;
        };
        let mut h = blake3::Hasher::new();
        h.update(&px.width().to_le_bytes());
        h.update(&px.height().to_le_bytes());
        h.update(px.as_raw());
        let key = h.finalize();
        let u = *by_hash.entry(key).or_insert_with(|| {
            uniques.push(px);
            uniques.len() - 1
        });
        each.push(Some((u, size, at)));
    }
    let sizes: Vec<[u32; 2]> = uniques.iter().map(|i| [i.width(), i.height()]).collect();
    let turn_as = rot.turn();
    let (page_sizes, spots) = pack(&sizes, turn_as.is_some());
    let mut pages: Vec<RgbaImage> = page_sizes.iter().map(|[w, h]| RgbaImage::new(*w, *h)).collect();
    for (u, img) in uniques.iter().enumerate() {
        let s = spots[u];
        let stored = match (s.turned, turn_as) {
            (true, Some(270)) => imageops::rotate270(img),
            (true, _) => imageops::rotate90(img),
            (false, _) => img.clone(),
        };
        imageops::replace(&mut pages[s.page], &stored, i64::from(s.x), i64::from(s.y));
    }
    let mut index = vec![None; sheet.sprites.len()];
    let mut sprites = Vec::new();
    for (i, (s, e)) in sheet.sprites.iter().zip(&each).enumerate() {
        let Some((u, size, at)) = *e else { continue };
        let spot = spots[u];
        let [w, h] = sizes[u];
        let rect = if spot.turned { [spot.x, spot.y, h, w] } else { [spot.x, spot.y, w, h] };
        let turn = if spot.turned { turn_as.unwrap_or(0) } else { 0 };
        index[i] = Some(sprites.len());
        sprites.push(Sprite { name: s.name.clone(), page: spot.page, rect, turn, size, at });
    }
    let animations = sheet
        .animations
        .iter()
        .filter_map(|a| {
            let frames: Vec<AnimFrame> = a
                .frames
                .iter()
                .filter_map(|f| Some(AnimFrame { sprite: index.get(f.sprite).copied().flatten()?, ..*f }))
                .collect();
            (!frames.is_empty()).then(|| Anim { frames, ..a.clone() })
        })
        .collect();
    let out_pages =
        page_sizes.iter().map(|&size| Page { image: String::new(), size, asset: None, hash: None }).collect();
    (Sheet { pages: out_pages, sprites, animations, ..sheet.clone() }, pages)
}

/// An image without its transparent margin (at least a pixel left), and where what's left starts.
fn trim_transparent(img: RgbaImage) -> (RgbaImage, [i64; 2]) {
    let (w, h) = img.dimensions();
    let (mut x0, mut y0, mut x1, mut y1) = (w, h, 0, 0);
    for (x, y, p) in img.enumerate_pixels() {
        if p[3] != 0 {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x + 1);
            y1 = y1.max(y + 1);
        }
    }
    if x1 <= x0 || y1 <= y0 {
        return (imageops::crop_imm(&img, 0, 0, 1, 1).to_image(), [0, 0]);
    }
    if [x0, y0, x1, y1] == [0, 0, w, h] {
        return (img, [0, 0]);
    }
    (imageops::crop_imm(&img, x0, y0, x1 - x0, y1 - y0).to_image(), [i64::from(x0), i64::from(y0)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packing_fills_pages_without_overlap() {
        let sizes: Vec<[u32; 2]> = (0..200).map(|i| [8 + i % 13 * 3, 6 + i % 7 * 5]).collect();
        for turn in [false, true] {
            let (pages, spots) = pack(&sizes, turn);
            assert_eq!(pages.len(), 1);
            let rects: Vec<[u32; 4]> = spots
                .iter()
                .zip(&sizes)
                .map(|(s, [w, h])| if s.turned { [s.x, s.y, *h, *w] } else { [s.x, s.y, *w, *h] })
                .collect();
            for (i, a) in rects.iter().enumerate() {
                assert!(a[0] + a[2] <= pages[0][0] && a[1] + a[3] <= pages[0][1]);
                for b in &rects[i + 1..] {
                    let apart = a[0] + a[2] + PAD <= b[0]
                        || b[0] + b[2] + PAD <= a[0]
                        || a[1] + a[3] + PAD <= b[1]
                        || b[1] + b[3] + PAD <= a[1];
                    assert!(apart, "{a:?} and {b:?} overlap");
                }
            }
        }
        // More than a page holds: more pages.
        let (pages, spots) = pack(&[[3000, 3000]; 3], false);
        assert_eq!(pages, [[3000, 3000]; 3], "each page as big as what's on it");
        assert_eq!(spots.iter().map(|s| s.page).collect::<HashSet<_>>().len(), 3);
    }
}
