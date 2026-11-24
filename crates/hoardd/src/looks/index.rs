//! The in-memory index "more like this", near copies and color search rank against: each thumbnail's descriptor,
//! hash and palette.

use crate::resident::Resident;
use std::collections::HashMap;

use anyhow::Result;

use super::features::DIMS;
use super::{Features, Swatch, VERSION, decode_palette};
use crate::AppState;

/// "More like this" keeps at most this many matches, and only those scoring at least the floor.
const SIMILAR_MAX: usize = 1000;
pub(super) const SIMILAR_FLOOR: f32 = 0.55;
/// How much what a picture shows counts in "more like this", against how it looks, when image search is on.
const MEANING_WEIGHT: f32 = 0.7;
/// "Find by color" keeps assets with at least about this share of the picture near the color.
const COLOR_MAX: usize = 3000;
const COLOR_FLOOR: f32 = 0.08;
/// How far (in `OKLab`) a color still counts as the one asked for, and how little lightness matters next to hue.
const COLOR_SIGMA: f32 = 0.08;
const COLOR_L_WEIGHT: f32 = 0.5;

#[derive(Debug)]
pub(super) struct Entry {
    pub(super) palette: Vec<Swatch>,
    pub(super) descriptor: Box<[i8]>,
    pub(super) phash: u64,
}

impl Entry {
    pub(super) fn from(f: Features) -> Option<Self> {
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
pub(crate) struct Index(Resident<Entry>);

impl Index {
    /// An empty index that's loaded, for tests.
    #[cfg(test)]
    pub(crate) fn loaded() -> Self {
        let index = Self::default();
        index.0.set(HashMap::new());
        index
    }

    fn read<R>(&self, f: impl FnOnce(&HashMap<String, Entry>) -> R) -> Option<R> {
        self.0.read(f)
    }

    fn write(&self, f: impl FnOnce(&mut HashMap<String, Entry>)) {
        self.0.write(f);
    }

    /// Load the index if it isn't yet.
    pub(crate) async fn ensure(&self, state: &AppState) -> Result<()> {
        let load = |c: &rusqlite::Connection| {
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
                        print: Vec::new(),
                    })?;
                    Some((hash, entry))
                })
                .collect();
            Ok(rows)
        };
        self.0.ensure(&state.pool, "looks", load, || true).await
    }

    pub(crate) fn insert(&self, hash: String, features: Features) {
        if let Some(entry) = Entry::from(features) {
            self.write(|m| {
                m.insert(hash, entry);
            });
        }
    }

    pub(super) fn remove(&self, hashes: &[String]) {
        self.write(|m| {
            for h in hashes {
                m.remove(h);
            }
        });
    }

    /// Thumbnails that look like `hash`'s, best first. With image search's scores (`meaning`, by what the pictures
    /// show), those count most where a thumbnail has one; colors and grain decide the rest.
    pub(super) fn similar(&self, hash: &str, meaning: Option<crate::embed::Likeness<'_>>) -> Vec<(String, f32)> {
        self.read(|m| {
            let Some(target) = m.get(hash) else { return Vec::new() };
            let scored = m.iter().filter(|(h, _)| h.as_str() != hash).map(|(h, e)| {
                let look = similarity(target, e);
                let score = match meaning.and_then(|s| s(h)) {
                    // Rescaled so the same floor applies: a close look alone still gets in.
                    Some(clip) => {
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

    /// Thumbnails that could be the same picture as `hash`'s: the perceptual hashes at most `bits` apart and the
    /// descriptors at least `look` alike. Their prints decide (see [`same_print`]).
    pub(crate) fn near(&self, hash: &str, bits: u32, look: f32) -> Vec<String> {
        self.read(|m| {
            let Some(target) = m.get(hash) else { return Vec::new() };
            m.iter()
                .filter(|(h, e)| h.as_str() != hash && close(target, e, bits, look))
                .map(|(h, _)| h.clone())
                .collect()
        })
        .unwrap_or_default()
    }

    /// Every pair of thumbnails that could be the same picture, as [`Index::near`] has them. The hashes are split in
    /// `bits + 1` pieces: two at most `bits` apart have one piece the same, so only those sharing a piece are compared.
    pub(crate) fn near_pairs(&self, bits: u32, look: f32) -> Vec<(String, String)> {
        self.read(|m| {
            let entries: Vec<(&String, &Entry)> = m.iter().filter(|(_, e)| !e.palette.is_empty()).collect();
            let pieces = bits as usize + 1;
            let mut pairs = std::collections::HashSet::new();
            for piece in 0..pieces {
                let (from, to) = (piece * 64 / pieces, (piece + 1) * 64 / pieces);
                let mask = if to - from == 64 { u64::MAX } else { ((1u64 << (to - from)) - 1) << from };
                let mut buckets: HashMap<u64, Vec<usize>> = HashMap::new();
                for (i, (_, e)) in entries.iter().enumerate() {
                    buckets.entry(e.phash & mask).or_default().push(i);
                }
                for bucket in buckets.values() {
                    for (n, &i) in bucket.iter().enumerate() {
                        for &j in &bucket[n + 1..] {
                            let (a, b) = if entries[i].0 < entries[j].0 { (i, j) } else { (j, i) };
                            if !pairs.contains(&(a, b)) && close(entries[a].1, entries[b].1, bits, look) {
                                pairs.insert((a, b));
                            }
                        }
                    }
                }
            }
            pairs.into_iter().map(|(a, b)| (entries[a].0.clone(), entries[b].0.clone())).collect()
        })
        .unwrap_or_default()
    }

    /// Whether two thumbnails could be the same picture (see [`Index::near`]).
    pub(crate) fn close(&self, a: &str, b: &str, bits: u32, look: f32) -> bool {
        a == b || self.read(|m| m.get(a).zip(m.get(b)).is_some_and(|(x, y)| close(x, y, bits, look))).unwrap_or(false)
    }

    /// Thumbnails with much of `lab` in them, most first.
    pub(super) fn by_color(&self, lab: [f32; 3]) -> Vec<(String, f32)> {
        self.read(|m| top(m.iter().map(|(h, e)| (h, color_score(&e.palette, lab))), COLOR_FLOOR, COLOR_MAX))
            .unwrap_or_default()
    }
}

fn top<'a>(scored: impl Iterator<Item = (&'a String, f32)>, floor: f32, max: usize) -> Vec<(String, f32)> {
    crate::embed::best(scored.filter(|(_, s)| *s >= floor).collect(), max)
}

/// How alike two thumbnails look, about 0..=1: the descriptor's weighted cosine, plus a bonus for near-identical
/// perceptual hashes.
pub(super) fn similarity(a: &Entry, b: &Entry) -> f32 {
    let dot: i32 = a.descriptor.iter().zip(b.descriptor.iter()).map(|(x, y)| i32::from(*x) * i32::from(*y)).sum();
    let descriptor = dot as f32 / (127.0 * 127.0);
    let distance = (a.phash ^ b.phash).count_ones() as f32;
    let near_copy = ((20.0 - distance) / 20.0).clamp(0.0, 1.0);
    0.85 * descriptor + 0.15 * near_copy
}

/// Perceptual hashes at most `bits` apart and descriptors at least `look` alike (a cosine).
fn close(a: &Entry, b: &Entry, bits: u32, look: f32) -> bool {
    (a.phash ^ b.phash).count_ones() <= bits && {
        let dot: i32 = a.descriptor.iter().zip(b.descriptor.iter()).map(|(x, y)| i32::from(*x) * i32::from(*y)).sum();
        dot as f32 / (127.0 * 127.0) >= look
    }
}

/// How much of a picture is close to `lab`: each palette color's share, counted less the further it is.
pub(super) fn color_score(palette: &[Swatch], lab: [f32; 3]) -> f32 {
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
