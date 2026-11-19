//! Near copies: the same picture at another size or in another format, the same sound at another rate or in another
//! format, the same model in another format. Candidates come from what "more like this" already knows (see `looks`
//! and `sounds`), and only those whose coarse prints match too are copies, so a recolored texture or another take of
//! a sound stays a variant. Byte-identical files are identical copies, not near ones; archived assets, and other
//! formats of the same asset (see `alternates`), don't count.
//!
//! Of a set of copies one is marked the best: the biggest, lossless, most complete. Nothing here changes or deletes
//! anything, least of all in a watched folder.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use anyhow::Result;
use hoard_core::{CopyGroup, NearCopy};
use rusqlite::{Connection, OptionalExtension, params_from_iter};

use crate::{AppState, db, looks, sounds};

/// Pictures whose perceptual hashes are at most this many bits apart...
const PICTURE_BITS: u32 = 6;
/// ...and whose descriptors are this alike may be copies; their prints decide.
const PICTURE_LOOK: f32 = 0.9;
/// The same for models' renders, looser: another loader can light and shade a model a little differently.
const MODEL_BITS: u32 = 12;
const MODEL_LOOK: f32 = 0.8;
/// How far apart two pictures' shapes (width over height) may be, as a share.
const ASPECT: f64 = 0.02;
/// Near copies listed with one asset at most.
const MOST: usize = 50;
/// How long the library's sets are answered before they're worked out again, even if nothing seems to have changed.
const FRESH: Duration = Duration::from_secs(300);
/// How long after start the sets are first worked out, once the start-up work is under way.
const WARM_AFTER: Duration = Duration::from_secs(20);

/// What's compared with what.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Sort {
    Picture,
    Sound,
    Model,
}

fn sort_of(kind: &str) -> Option<Sort> {
    match kind {
        "texture" | "vector" => Some(Sort::Picture),
        "audio" => Some(Sort::Sound),
        "model" => Some(Sort::Model),
        _ => None,
    }
}

const SORT_SQL: &str = "a.kind IN ('texture', 'vector', 'audio', 'model')";

/// One file as a candidate: the first asset with it (by pack and path), and what decides which copy is best.
#[derive(Clone, Debug)]
struct Item {
    id: String,
    name: String,
    pack_id: String,
    pack_name: String,
    path: String,
    ext: String,
    kind: String,
    blob: String,
    thumb: Option<String>,
    size: i64,
    width: Option<i64>,
    height: Option<i64>,
    sample_rate: Option<i64>,
    bit_depth: Option<i64>,
    channels: Option<i64>,
    triangles: Option<i64>,
    animations: Option<i64>,
    /// Textures, materials and clips: those it holds, and those linked to it.
    parts: i64,
}

const ITEM_SQL: &str = "SELECT a.id, a.name, a.pack_id, p.name, a.path, a.ext, a.kind, a.blob_hash, a.thumb_hash, b.size,
        json_extract(a.meta, '$.width'), json_extract(a.meta, '$.height'), json_extract(a.meta, '$.sample_rate'),
        json_extract(a.meta, '$.bit_depth'), json_extract(a.meta, '$.channels'), json_extract(a.meta, '$.triangles'),
        json_extract(a.meta, '$.animations'),
        coalesce(json_extract(a.meta, '$.textures'), 0) + coalesce(json_extract(a.meta, '$.materials'), 0)
          + (SELECT COUNT(*) FROM asset_links l WHERE l.from_id = a.id AND l.rel IN ('texture', 'material', 'animation'))
     FROM assets a JOIN packs p ON p.id = a.pack_id JOIN blobs b ON b.hash = a.blob_hash
     WHERE a.archived_at IS NULL AND p.archived_at IS NULL";

fn item(r: &rusqlite::Row<'_>) -> rusqlite::Result<Item> {
    // Meta numbers can be stored as floats; whole ones are wanted.
    let num = |i: usize| -> rusqlite::Result<Option<i64>> {
        Ok(match r.get_ref(i)? {
            rusqlite::types::ValueRef::Integer(n) => Some(n),
            rusqlite::types::ValueRef::Real(f) => Some(f.round() as i64),
            _ => None,
        })
    };
    Ok(Item {
        id: r.get(0)?,
        name: r.get(1)?,
        pack_id: r.get(2)?,
        pack_name: r.get(3)?,
        path: r.get(4)?,
        ext: r.get(5)?,
        kind: r.get(6)?,
        blob: r.get(7)?,
        thumb: r.get(8)?,
        size: r.get(9)?,
        width: num(10)?,
        height: num(11)?,
        sample_rate: num(12)?,
        bit_depth: num(13)?,
        channels: num(14)?,
        triangles: num(15)?,
        animations: num(16)?,
        parts: num(17)?.unwrap_or(0),
    })
}

/// The assets matching `filter` (more SQL after `WHERE`), one per file.
fn items(conn: &Connection, filter: &str, params: &[String]) -> Result<Vec<Item>> {
    let sql = format!("{ITEM_SQL} AND {filter} ORDER BY p.name, a.path");
    let rows = conn.prepare(&sql)?.query_map(params_from_iter(params), item)?.collect::<rusqlite::Result<Vec<_>>>()?;
    let mut seen = HashSet::new();
    Ok(rows.into_iter().filter(|i| seen.insert(i.blob.clone())).collect())
}

fn marks(n: usize) -> String {
    vec!["?"; n].join(", ")
}

/// The prints of these sound files (those that have one).
fn sound_prints(conn: &Connection, blobs: &[&str]) -> Result<HashMap<String, Vec<u8>>> {
    let mut out = HashMap::new();
    let mut stmt = conn.prepare_cached("SELECT print FROM sounds WHERE hash = ?1 AND print IS NOT NULL")?;
    for &h in blobs {
        if let Some(p) = stmt.query_row([h], |r| r.get::<_, Vec<u8>>(0)).optional()? {
            out.insert(h.to_owned(), p);
        }
    }
    Ok(out)
}

/// What's needed to tell whether two candidates are copies.
struct Judge<'a> {
    looks: &'a looks::Index,
    sounds: &'a sounds::Index,
    /// Thumbnails' prints, by thumbnail.
    pictures: HashMap<String, Vec<u8>>,
    /// Sound files' prints, by file.
    heard: HashMap<String, Vec<u8>>,
    /// Asset pairs that are formats of one asset, either way round.
    formats: HashSet<(String, String)>,
}

impl Judge<'_> {
    fn copies(&self, a: &Item, b: &Item) -> bool {
        if a.blob == b.blob || self.formats.contains(&(a.id.clone(), b.id.clone())) {
            return false;
        }
        match (sort_of(&a.kind), sort_of(&b.kind)) {
            (Some(Sort::Picture), Some(Sort::Picture)) => {
                same_shape(a, b) && self.look_alike(a, b, PICTURE_BITS, PICTURE_LOOK, false)
            }
            (Some(Sort::Model), Some(Sort::Model)) => {
                a.triangles.is_some_and(|t| t > 0)
                    && a.triangles == b.triangles
                    && a.animations.unwrap_or(0) == b.animations.unwrap_or(0)
                    && self.look_alike(a, b, MODEL_BITS, MODEL_LOOK, true)
            }
            (Some(Sort::Sound), Some(Sort::Sound)) => {
                self.sounds.close(&a.blob, &b.blob)
                    && match (self.heard.get(&a.blob), self.heard.get(&b.blob)) {
                        (Some(x), Some(y)) => sounds::same_print(x, y),
                        _ => false,
                    }
            }
            _ => false,
        }
    }

    /// Their thumbnails are the same, or as good as.
    fn look_alike(&self, a: &Item, b: &Item, bits: u32, look: f32, loose: bool) -> bool {
        let (Some(ta), Some(tb)) = (&a.thumb, &b.thumb) else { return false };
        ta == tb
            || self.looks.close(ta, tb, bits, look)
                && match (self.pictures.get(ta), self.pictures.get(tb)) {
                    (Some(pa), Some(pb)) => looks::same_print(pa, pb, loose),
                    _ => false,
                }
    }
}

/// Pictures the same shape (a print is stretched to a square, so it can't tell).
fn same_shape(a: &Item, b: &Item) -> bool {
    match (a.width, a.height, b.width, b.height) {
        (Some(aw), Some(ah), Some(bw), Some(bh)) if aw > 0 && ah > 0 && bw > 0 && bh > 0 => {
            let (x, y) = (aw as f64 / ah as f64, bw as f64 / bh as f64);
            (x - y).abs() <= ASPECT * x.max(y)
        }
        // A drawing may not say: its print still has to match.
        _ => true,
    }
}

/// Format links among these assets, both ways round.
fn format_links(conn: &Connection, ids: &[&str]) -> Result<HashSet<(String, String)>> {
    let mut out = HashSet::new();
    for chunk in ids.chunks(400) {
        let sql = format!(
            "SELECT from_id, to_id FROM asset_links WHERE rel = 'format' AND (from_id IN ({0}) OR to_id IN ({0}))",
            marks(chunk.len())
        );
        let doubled: Vec<&str> = chunk.iter().chain(chunk.iter()).copied().collect();
        let rows = conn
            .prepare(&sql)?
            .query_map(params_from_iter(doubled), |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        for (a, b) in rows {
            out.insert((b.clone(), a.clone()));
            out.insert((a, b));
        }
    }
    Ok(out)
}

/// How good a copy is, to pick the best: bigger first, then lossless, then more complete, then more bytes.
fn rank(i: &Item) -> (i64, i64, i64, i64, i64, i64) {
    let ext = i.ext.to_ascii_lowercase();
    match sort_of(&i.kind) {
        Some(Sort::Picture) => {
            let vector = i64::from(i.kind == "vector");
            let pixels = i.width.unwrap_or(0) * i.height.unwrap_or(0);
            (vector, pixels, i64::from(lossless_picture(&ext)), 0, 0, i.size)
        }
        Some(Sort::Sound) => (
            i64::from(lossless_sound(&ext)),
            i.sample_rate.unwrap_or(0),
            i.bit_depth.unwrap_or(0),
            i.channels.unwrap_or(0),
            0,
            i.size,
        ),
        _ => (i.parts, i.animations.unwrap_or(0), model_format(&ext), 0, 0, i.size),
    }
}

fn lossless_picture(ext: &str) -> bool {
    matches!(
        ext,
        "png" | "tga" | "bmp" | "tif" | "tiff" | "psd" | "psb" | "exr" | "hdr" | "qoi" | "kra" | "ora" | "xcf"
    )
}

fn lossless_sound(ext: &str) -> bool {
    matches!(ext, "wav" | "wave" | "flac" | "aif" | "aiff" | "aifc" | "w64" | "wv" | "ape" | "alac")
}

/// Formats that keep more of a model (a source file, then scenes with materials and animation) score higher.
fn model_format(ext: &str) -> i64 {
    match ext {
        "blend" | "max" | "ma" | "mb" | "c4d" => 4,
        "glb" | "gltf" | "fbx" | "usd" | "usdz" | "usda" | "usdc" => 3,
        "dae" | "3ds" | "x" => 2,
        _ => 1,
    }
}

fn near_copy(i: &Item, best: bool) -> NearCopy {
    let n = |v: Option<i64>| v.and_then(|v| u32::try_from(v).ok()).filter(|&v| v > 0);
    NearCopy {
        id: i.id.clone(),
        name: i.name.clone(),
        pack_id: i.pack_id.clone(),
        pack_name: i.pack_name.clone(),
        path: i.path.clone(),
        ext: i.ext.clone(),
        size: i.size,
        width: n(i.width),
        height: n(i.height),
        sample_rate: n(i.sample_rate),
        bit_depth: n(i.bit_depth),
        best,
    }
}

/// The near copies of an asset, the best marked (none is when this asset is the best).
pub(crate) fn of_asset(
    conn: &Connection,
    looks: &looks::Index,
    sounds: &sounds::Index,
    id: &str,
) -> Result<Vec<NearCopy>> {
    let Some(target) = items(conn, "a.id = ?", &[id.to_owned()])?.pop() else { return Ok(Vec::new()) };
    let Some(sort) = sort_of(&target.kind) else { return Ok(Vec::new()) };
    let not_it = "a.blob_hash != ?";
    let mut candidates = match sort {
        Sort::Picture => {
            let Some(thumb) = &target.thumb else { return Ok(Vec::new()) };
            let mut thumbs = looks.near(thumb, PICTURE_BITS, PICTURE_LOOK);
            thumbs.truncate(400);
            thumbs.push(thumb.clone());
            let filter =
                format!("a.kind IN ('texture', 'vector') AND {not_it} AND a.thumb_hash IN ({})", marks(thumbs.len()));
            items(conn, &filter, &[vec![target.blob.clone()], thumbs].concat())?
        }
        Sort::Model => {
            let Some(triangles) = target.triangles.filter(|&t| t > 0) else { return Ok(Vec::new()) };
            let filter = format!("a.kind = 'model' AND {not_it} AND json_extract(a.meta, '$.triangles') = {triangles}");
            items(conn, &filter, std::slice::from_ref(&target.blob))?
        }
        Sort::Sound => {
            let mut blobs = sounds.near(&target.blob);
            blobs.truncate(400);
            if blobs.is_empty() {
                return Ok(Vec::new());
            }
            let filter = format!("a.kind = 'audio' AND a.blob_hash IN ({})", marks(blobs.len()));
            items(conn, &filter, &blobs)?
        }
    };
    candidates.retain(|c| c.blob != target.blob);
    let judge = judge(conn, looks, sounds, std::iter::once(&target).chain(&candidates))?;
    candidates.retain(|c| judge.copies(&target, c));
    let here = rank(&target);
    let best =
        candidates.iter().enumerate().filter(|(_, c)| rank(c) > here).max_by_key(|(_, c)| rank(c)).map(|(i, _)| i);
    let mut out: Vec<NearCopy> = candidates.iter().enumerate().map(|(i, c)| near_copy(c, Some(i) == best)).collect();
    out.sort_by_key(|c| !c.best);
    out.truncate(MOST);
    Ok(out)
}

fn judge<'a, 'b>(
    conn: &Connection,
    looks: &'a looks::Index,
    sounds: &'a sounds::Index,
    of: impl Iterator<Item = &'b Item> + Clone,
) -> Result<Judge<'a>> {
    let thumbs: Vec<&str> = of.clone().filter_map(|i| i.thumb.as_deref()).collect();
    let audio: Vec<&str> = of.clone().filter(|i| i.kind == "audio").map(|i| i.blob.as_str()).collect();
    let ids: Vec<&str> = of.map(|i| i.id.as_str()).collect();
    Ok(Judge {
        looks,
        sounds,
        pictures: looks::prints(conn, &thumbs)?,
        heard: sound_prints(conn, &audio)?,
        formats: format_links(conn, &ids)?,
    })
}

/// The root of `i`'s set, for a union-find over `parent`.
fn root(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

/// Every set of near copies in the library, the most space taken by the copies that aren't the best first.
fn all(conn: &Connection, looks: &looks::Index, sounds: &sounds::Index) -> Result<Vec<CopyGroup>> {
    let items = items(conn, SORT_SQL, &[])?;
    let index: HashMap<&str, usize> = items.iter().enumerate().map(|(i, it)| (it.blob.as_str(), i)).collect();
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    // Pictures: those whose thumbnails could be the same, and those sharing one.
    let mut by_thumb: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, it) in items.iter().enumerate() {
        if let (Some(t), Some(Sort::Picture)) = (&it.thumb, sort_of(&it.kind)) {
            by_thumb.entry(t.as_str()).or_default().push(i);
        }
    }
    for group in by_thumb.values() {
        for (n, &a) in group.iter().enumerate() {
            pairs.extend(group[n + 1..].iter().map(|&b| (a, b)));
        }
    }
    for (x, y) in looks.near_pairs(PICTURE_BITS, PICTURE_LOOK) {
        if let (Some(xs), Some(ys)) = (by_thumb.get(x.as_str()), by_thumb.get(y.as_str())) {
            pairs.extend(xs.iter().flat_map(|&a| ys.iter().map(move |&b| (a, b))));
        }
    }

    // Models: those with as many triangles.
    let mut by_triangles: HashMap<i64, Vec<usize>> = HashMap::new();
    for (i, it) in items.iter().enumerate() {
        if let (Some(t), Some(Sort::Model)) = (it.triangles.filter(|&t| t > 0), sort_of(&it.kind)) {
            by_triangles.entry(t).or_default().push(i);
        }
    }
    for group in by_triangles.values() {
        for (n, &a) in group.iter().enumerate() {
            pairs.extend(group[n + 1..].iter().map(|&b| (a, b)));
        }
    }

    // Sounds: those about as long, by their prints.
    let audio: Vec<&str> =
        items.iter().filter(|i| sort_of(&i.kind) == Some(Sort::Sound)).map(|i| i.blob.as_str()).collect();
    let heard = sound_prints(conn, &audio)?;
    let mut by_length: Vec<(f32, usize)> =
        heard.iter().filter_map(|(h, p)| Some((sounds::print_secs(p)?, *index.get(h.as_str())?))).collect();
    by_length.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (n, &(secs, a)) in by_length.iter().enumerate() {
        for &(other, b) in &by_length[n + 1..] {
            if other - secs > (secs * 0.015).max(0.02) {
                break;
            }
            pairs.push((a, b));
        }
    }

    let involved: HashSet<usize> = pairs.iter().flat_map(|&(a, b)| [a, b]).collect();
    let thumbs: Vec<&str> = involved.iter().filter_map(|&i| items[i].thumb.as_deref()).collect();
    let ids: Vec<&str> = involved.iter().map(|&i| items[i].id.as_str()).collect();
    let judge =
        Judge { looks, sounds, pictures: looks::prints(conn, &thumbs)?, heard, formats: format_links(conn, &ids)? };

    let mut parent: Vec<usize> = (0..items.len()).collect();
    for (a, b) in pairs {
        if judge.copies(&items[a], &items[b]) {
            let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
            if ra != rb {
                parent[ra.max(rb)] = ra.min(rb);
            }
        }
    }
    let mut sets: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in involved {
        let r = root(&mut parent, i);
        sets.entry(r).or_default().push(i);
    }
    let mut groups: Vec<CopyGroup> = sets
        .into_values()
        .filter(|s| s.len() > 1)
        .map(|mut s| {
            s.sort_by(|&a, &b| rank(&items[b]).cmp(&rank(&items[a])).then_with(|| items[a].path.cmp(&items[b].path)));
            let wasted = s[1..].iter().map(|&i| items[i].size).sum();
            CopyGroup { wasted, copies: s.iter().enumerate().map(|(n, &i)| near_copy(&items[i], n == 0)).collect() }
        })
        .collect();
    groups.sort_by(|a, b| b.wasted.cmp(&a.wasted).then_with(|| a.copies[0].path.cmp(&b.copies[0].path)));
    Ok(groups)
}

/// The library's sets of near copies as last worked out, and whether they're being worked out again.
#[derive(Debug, Default)]
pub(crate) struct Cache {
    kept: Mutex<Option<Kept>>,
    /// Held while the sets are worked out, so it's done once at a time.
    working: tokio::sync::Mutex<()>,
    /// How many times they've been worked out since the server started.
    generation: AtomicU64,
}

#[derive(Clone, Debug)]
struct Kept {
    at: Instant,
    print: Print,
    generation: u64,
    sets: Arc<Vec<CopyGroup>>,
}

/// What would change the library's sets: its live assets and packs, the last change to an asset, and how many
/// pictures have been looked at and sounds listened to. Cheap to read, unlike the sets.
type Print = (i64, i64, Option<String>, i64, i64);

fn print(conn: &Connection) -> Result<Print> {
    Ok(conn.query_row(
        "SELECT (SELECT COUNT(*) FROM assets WHERE archived_at IS NULL), (SELECT COUNT(*) FROM packs WHERE archived_at IS NULL),
                (SELECT MAX(updated_at) FROM assets), (SELECT COUNT(*) FROM looks),
                (SELECT COUNT(*) FROM sounds WHERE print IS NOT NULL)",
        [],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
    )?)
}

/// The library's sets of near copies, as they were last worked out.
#[derive(Clone, Debug)]
pub(crate) struct Groups {
    pub(crate) sets: Arc<Vec<CopyGroup>>,
    /// Which working-out these are; it goes up each time.
    pub(crate) generation: u64,
    /// The library has changed since, and they're being worked out again in the background.
    pub(crate) refreshing: bool,
}

impl Cache {
    fn kept(&self) -> Option<Kept> {
        self.kept.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }
}

/// The library's sets of near copies (see [`all`]). Once worked out they're always answered at once: when the library
/// has changed since, or they're a few minutes old, they're worked out again in the background, and the answer says so.
/// Only the very first time (they're worked out shortly after start, see [`warm`]) does this wait for them.
pub(crate) async fn groups(state: &AppState) -> Result<Groups> {
    let Some(kept) = state.near_copies.kept() else {
        let kept = work_out(state).await?;
        return Ok(Groups { sets: kept.sets, generation: kept.generation, refreshing: false });
    };
    let current = db::call(&state.pool, |c| print(c)).await?;
    let outdated = current != kept.print || kept.at.elapsed() >= FRESH;
    if outdated {
        refresh(state);
    }
    let refreshing = outdated || state.near_copies.working.try_lock().is_err();
    Ok(Groups { sets: kept.sets, generation: kept.generation, refreshing })
}

/// Works the sets out again in the background, unless that's happening already.
fn refresh(state: &AppState) {
    if state.near_copies.working.try_lock().is_err() {
        return;
    }
    let st = Arc::clone(state);
    tokio::spawn(async move {
        if let Err(e) = work_out(&st).await {
            tracing::warn!(error = %format!("{e:#}"), "could not work out the library's near copies");
        }
    });
}

/// Works the sets out shortly after the server starts, so the Near copies page doesn't wait for them.
pub(crate) async fn warm(state: &AppState) {
    tokio::time::sleep(WARM_AFTER).await;
    if let Err(e) = work_out(state).await {
        tracing::warn!(error = %format!("{e:#}"), "could not work out the library's near copies");
    }
}

/// Works the sets out and keeps them. If they were worked out while this waited its turn, those are taken instead.
async fn work_out(state: &AppState) -> Result<Kept> {
    let asked = Instant::now();
    let _working = state.near_copies.working.lock().await;
    if let Some(kept) = state.near_copies.kept()
        && kept.at >= asked
    {
        return Ok(kept);
    }
    state.looks.ensure(state).await?;
    state.sounds.ensure(state).await?;
    let st = Arc::clone(state);
    let (seen, sets) = db::call(&state.pool, move |c| Ok((print(c)?, all(c, &st.looks, &st.sounds)?))).await?;
    let kept = Kept {
        at: Instant::now(),
        print: seen,
        generation: state.near_copies.generation.fetch_add(1, Ordering::Relaxed) + 1,
        sets: Arc::new(sets),
    };
    *state.near_copies.kept.lock().unwrap_or_else(PoisonError::into_inner) = Some(kept.clone());
    Ok(kept)
}

/// The near copies of an asset (see [`of_asset`]).
pub(crate) async fn for_asset(state: &AppState, id: String) -> Result<Vec<NearCopy>> {
    state.looks.ensure(state).await?;
    state.sounds.ensure(state).await?;
    let st = Arc::clone(state);
    db::call(&state.pool, move |c| of_asset(c, &st.looks, &st.sounds, &id)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(kind: &str, ext: &str, size: i64) -> Item {
        Item {
            id: String::new(),
            name: String::new(),
            pack_id: String::new(),
            pack_name: String::new(),
            path: String::new(),
            ext: ext.into(),
            kind: kind.into(),
            blob: String::new(),
            thumb: None,
            size,
            width: None,
            height: None,
            sample_rate: None,
            bit_depth: None,
            channels: None,
            triangles: None,
            animations: None,
            parts: 0,
        }
    }

    #[test]
    fn the_best_copy_is_bigger_then_lossless_then_more_complete() {
        let at = |kind: &str, ext: &str, w: i64, size: i64| Item {
            width: Some(w),
            height: Some(w),
            ..item(kind, ext, size)
        };
        // A drawing beats any size of picture; then more pixels; then lossless over lossy at the same size.
        assert!(rank(&at("vector", "svg", 0, 10)) > rank(&at("texture", "png", 4096, 9_000_000)));
        assert!(rank(&at("texture", "jpg", 2048, 100)) > rank(&at("texture", "png", 1024, 900)));
        assert!(rank(&at("texture", "png", 1024, 900)) > rank(&at("texture", "jpg", 1024, 100)));

        let heard = |ext: &str, rate: i64, bits: i64| Item {
            sample_rate: Some(rate),
            bit_depth: Some(bits),
            ..item("audio", ext, 1)
        };
        assert!(rank(&heard("wav", 22_050, 16)) > rank(&heard("ogg", 48_000, 0)));
        assert!(rank(&heard("flac", 48_000, 24)) > rank(&heard("wav", 48_000, 16)));

        let model = |ext: &str, parts: i64| Item { parts, ..item("model", ext, 1) };
        assert!(rank(&model("obj", 3)) > rank(&model("glb", 1)), "the one with its textures");
        assert!(rank(&model("glb", 1)) > rank(&model("obj", 1)));
    }

    fn png(img: &image::RgbaImage) -> Vec<u8> {
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }

    /// A blotchy picture, different for each seed.
    fn blotches(seed: u32, w: u32, h: u32) -> image::RgbaImage {
        image::RgbaImage::from_fn(w, h, |x, y| {
            let (fx, fy) = (x as f32 / w as f32, y as f32 / h as f32);
            let s = seed as f32;
            let v = |k: f32| (fx * (3.0 + s) + fy * k * 2.0 + s).sin() * (fy * (2.0 + k) - fx * s).cos() * 0.5 + 0.5;
            image::Rgba([(v(1.0) * 255.0) as u8, (v(2.3) * 200.0) as u8, (v(3.7) * 120.0) as u8, 255])
        })
    }

    /// A struck, ringing sound: a few decaying partials.
    fn ring(hz: f32, secs: f32, rate: u32) -> Vec<u8> {
        let samples = (0..(secs * rate as f32) as usize)
            .map(|i| {
                let t = i as f32 / rate as f32;
                let v = (1..4).map(|k| (std::f32::consts::TAU * hz * k as f32 * t).sin() / k as f32).sum::<f32>();
                (v * (-t * 6.0).exp() * 0.5 * 32_767.0) as i32
            })
            .collect();
        crate::audio::write_wav(&crate::audio::Audio { samples, channels: 1, rate, bits: 16 })
    }

    #[test]
    fn finds_near_copies_and_the_best_of_them() {
        let c = db::test_library();
        let (looks, sounds) = (looks::Index::loaded(), sounds::Index::loaded());
        c.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'Pack', 'ready', 'x'), ('q', 'Other', 'ready', 'x');",
        )
        .unwrap();
        let add = |id: &str, pack: &str, ext: &str, kind: &str, size: i64, meta: &str, thumb: Option<&str>| {
            c.execute(
                "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, 'x') ON CONFLICT DO NOTHING",
                rusqlite::params![format!("b-{id}"), size],
            )
            .unwrap();
            c.execute(
                "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, thumb_hash, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?1, ?5, ?6, ?7, ?8, 'x', 'x')",
                rusqlite::params![id, pack, format!("b-{id}"), format!("{id}.{ext}"), ext, kind, meta, thumb],
            )
            .unwrap();
        };
        let look = |thumb: &str, img: &image::RgbaImage| {
            let f = looks::analyze(&png(img)).unwrap();
            c.execute(
                "INSERT INTO looks (thumb_hash, version, palette, descriptor, phash, print) VALUES (?1, 1, x'', x'', 0, ?2)",
                rusqlite::params![thumb, f.print],
            )
            .unwrap();
            looks.insert(thumb.to_owned(), f);
        };
        let hear = |id: &str, wav: Vec<u8>| {
            let h = sounds::analyze(bytes::Bytes::from(wav), Some("wav")).unwrap();
            c.execute(
                "INSERT INTO sounds (hash, version, descriptor, print) VALUES (?1, 3, x'', ?2)",
                rusqlite::params![format!("b-{id}"), h.print],
            )
            .unwrap();
            sounds.insert(&format!("b-{id}"), h.descriptor);
        };

        // A picture, the same smaller as a JPEG in another pack, another picture, and the first one recolored.
        let art = blotches(1, 256, 256);
        look("t-big", &art);
        look("t-small", &image::imageops::resize(&art, 128, 128, image::imageops::FilterType::Triangle));
        look("t-other", &blotches(4, 256, 256));
        let mut recolored = art.clone();
        recolored.pixels_mut().for_each(|p| p.0 = [p[2], p[0], p[1], 255]);
        look("t-recolored", &recolored);
        add("big", "p", "png", "texture", 900, r#"{"width":1024,"height":1024}"#, Some("t-big"));
        add("small", "q", "jpg", "texture", 100, r#"{"width":512,"height":512}"#, Some("t-small"));
        add("other", "q", "png", "texture", 100, r#"{"width":1024,"height":1024}"#, Some("t-other"));
        add("recolored", "q", "png", "texture", 900, r#"{"width":1024,"height":1024}"#, Some("t-recolored"));
        // Archived: never a copy.
        add("gone", "q", "png", "texture", 900, r#"{"width":1024,"height":1024}"#, Some("t-small"));
        c.execute("UPDATE assets SET archived_at = 'x' WHERE id = 'gone'", []).unwrap();

        // A sound, the same at half the rate, and a lower one.
        hear("ding", ring(880.0, 0.8, 44_100));
        hear("ding-22k", ring(880.0, 0.8, 22_050));
        hear("dong", ring(440.0, 0.8, 44_100));
        add("ding", "p", "ogg", "audio", 50, r#"{"sample_rate":44100}"#, None);
        add("ding-22k", "q", "wav", "audio", 70, r#"{"sample_rate":22050,"bit_depth":16}"#, None);
        add("dong", "q", "wav", "audio", 70, r#"{"sample_rate":44100,"bit_depth":16}"#, None);

        // A model, the same in another format (but linked as another format of it), and as an FBX elsewhere.
        look("t-ship", &blotches(9, 256, 256));
        add("ship", "p", "glb", "model", 500, r#"{"triangles":1200}"#, Some("t-ship"));
        add("ship-obj", "p", "obj", "model", 300, r#"{"triangles":1200}"#, Some("t-ship"));
        add("ship-fbx", "q", "fbx", "model", 800, r#"{"triangles":1200,"materials":2}"#, Some("t-ship"));
        add("boat", "q", "glb", "model", 800, r#"{"triangles":1300}"#, Some("t-ship"));
        c.execute("INSERT INTO asset_links (from_id, to_id, rel) VALUES ('ship', 'ship-obj', 'format')", []).unwrap();

        let near = |id: &str| -> Vec<(String, bool)> {
            of_asset(&c, &looks, &sounds, id).unwrap().into_iter().map(|n| (n.id, n.best)).collect()
        };
        // The big PNG is the best: its copy doesn't say so of anything; the copy points to it.
        assert_eq!(near("big"), [("small".to_owned(), false)]);
        assert_eq!(near("small"), [("big".to_owned(), true)]);
        assert_eq!(near("other"), []);
        assert!(near("recolored").is_empty(), "a recolor is a variant");
        assert_eq!(near("ding"), [("ding-22k".to_owned(), true)], "lossless beats a higher rate");
        assert_eq!(near("dong"), []);
        assert_eq!(near("ship"), [("ship-fbx".to_owned(), true)], "not its own other format; the one with materials");
        assert!(near("boat").is_empty(), "as many triangles or it isn't the same model");

        let groups = all(&c, &looks, &sounds).unwrap();
        let ids: Vec<(Vec<&str>, i64)> =
            groups.iter().map(|g| (g.copies.iter().map(|n| n.id.as_str()).collect(), g.wasted)).collect();
        // The OBJ and the GLB are each the same as the FBX, so all three are one set.
        assert_eq!(
            ids,
            [(vec!["ship-fbx", "ship", "ship-obj"], 800), (vec!["big", "small"], 100), (vec!["ding-22k", "ding"], 50)]
        );
        assert!(groups.iter().all(|g| g.copies[0].best && g.copies[1..].iter().all(|n| !n.best)));
    }

    #[test]
    fn pictures_must_be_the_same_shape() {
        let at = |w: i64, h: i64| Item { width: Some(w), height: Some(h), ..item("texture", "png", 1) };
        assert!(same_shape(&at(1024, 512), &at(256, 128)));
        assert!(same_shape(&at(1000, 1000), &at(1010, 1000)));
        assert!(!same_shape(&at(1024, 512), &at(512, 512)));
        assert!(same_shape(&at(1024, 512), &item("vector", "svg", 1)), "a drawing that doesn't say");
    }
}
