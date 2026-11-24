//! Thumbnails kept up with how they're made now. Each is kept with how it was made: which drawing of its kind
//! (`assets.thumb_drawn`, see [`DRAWN`]), how many pixels across and at what WebP quality (Settings → Performance →
//! Thumbnails). Wherever a thumbnail is written the database marks it with what's current (the `assets_thumb_*`
//! triggers, from `thumb_now` and `thumb_drawing`, which [`mark_now`] keeps), so those made differently are found
//! and made again in the background, each old one showing until its new one is in:
//!
//! - **Encoded again** from the stored thumbnail, with no need to read the asset's file: smaller or at a lower
//!   quality, or JPEG or PNG from before thumbnails were WebP. A small picture a browser shows as it is (a sprite,
//!   an icon) becomes its own thumbnail when that's smaller, kept in the local cache so showing it never wakes the
//!   drive the library is on.
//! - **Drawn again** from the asset's file, by analyzing it again: a fix to how its kind is drawn, or bigger or at a
//!   higher quality, which the stored thumbnail hasn't the detail for. Animations are drawn again by `motion`, and
//!   materials, takes and `.blend` files keep what they have (they're drawn otherwise), encoded again.
//!
//! What a thumbnail looks like to image search and to search by look is carried over to the one made from it, so the
//! library isn't looked at all over again.

use std::collections::{BTreeMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex, PoisonError};

use anyhow::Result;
use futures_util::{StreamExt as _, stream};
use rusqlite::{Connection, params};
use serde_json::json;

use crate::{AppState, db};

/// Which drawing of each kind's thumbnail is current, for the kinds whose drawing was fixed since thumbnails were
/// kept with it; [`FIRST`] for the rest. One more for a kind whenever a fix changes how its thumbnails are drawn:
/// those drawn before are drawn again in the background.
pub(crate) const DRAWN: &[(&str, i64)] = &[];
/// The drawing of every thumbnail made before they were kept with theirs.
pub(crate) const FIRST: i64 = 1;

/// Pictures browsers show as they are, which can be their own thumbnail when they're small.
const AS_IS: &[&str] = &["png", "jpg", "jpeg", "webp"];

/// Whether a fill is running, and whether another was asked for while it ran.
static BUSY: AtomicBool = AtomicBool::new(false);
static AGAIN: AtomicBool = AtomicBool::new(false);
/// Thumbnails that couldn't be made again, and assets sent to be drawn again, since the server started; neither is
/// tried again until it starts again, so a file that can't be drawn isn't read over and over.
static TRIED: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);

/// The drawing of this kind's thumbnails that's current.
pub(crate) fn drawn(kind: &str) -> i64 {
    DRAWN.iter().find(|(k, _)| *k == kind).map_or(FIRST, |(_, rev)| *rev)
}

/// Whether a picture can be its own thumbnail: one browsers show as it is, a still no bigger than a thumbnail, and no
/// more bytes than the thumbnail made of it.
pub(crate) fn is_own_thumb(ext: &str, (w, h): (u32, u32), bytes: u64, thumb: usize, still: bool) -> bool {
    still && AS_IS.contains(&ext) && w.max(h) <= crate::analyze::thumb_size() && bytes <= thumb as u64
}

/// Tell the database how thumbnails are made now, so it marks those written with it.
pub(crate) fn mark_now(c: &Connection, px: u32, q: u8) -> rusqlite::Result<()> {
    let tx = c.unchecked_transaction()?;
    tx.execute("UPDATE thumb_now SET px = ?1, q = ?2", params![px, q])?;
    tx.execute("DELETE FROM thumb_drawing", [])?;
    for (kind, rev) in DRAWN.iter().copied().chain([("*", FIRST)]) {
        tx.execute("INSERT INTO thumb_drawing (kind, rev) VALUES (?1, ?2)", params![kind, rev])?;
    }
    tx.commit()
}

/// Make again, in the background, the thumbnails made otherwise than they're made now. Asked for again while it
/// runs, it goes round once more when done.
pub(crate) fn kick(state: &AppState) {
    AGAIN.store(true, Ordering::SeqCst);
    if BUSY.swap(true, Ordering::SeqCst) {
        return;
    }
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        loop {
            while AGAIN.swap(false, Ordering::SeqCst) {
                if let Err(e) = fill(&state).await {
                    tracing::warn!(error = %format!("{e:#}"), "couldn't make thumbnails again");
                }
            }
            BUSY.store(false, Ordering::SeqCst);
            if !AGAIN.load(Ordering::SeqCst) || BUSY.swap(true, Ordering::SeqCst) {
                break;
            }
        }
    });
}

/// A thumbnail made otherwise than thumbnails are made now.
#[expect(clippy::struct_excessive_bools, reason = "independent facts about the asset")]
#[derive(Clone, Debug, Default)]
struct Stale {
    id: String,
    pack_id: String,
    ext: String,
    thumb: String,
    thumb_size: i64,
    blob: String,
    blob_size: i64,
    drawn: i64,
    current: i64,
    px: Option<u32>,
    q: Option<u8>,
    pixel_art: bool,
    /// Its width and height, when known.
    dims: Option<(u32, u32)>,
    /// A still picture (not animated, not HDR), as far as is known.
    still: bool,
    /// Analyzing its file again draws it: not an animation, a material, a take or a `.blend`, in a pack that's ready.
    drawable: bool,
    /// Text drawn as its lines, kept exact as pixel art is.
    text: bool,
}

/// How a stale thumbnail is made again.
#[derive(Debug, PartialEq, Eq)]
enum Redo {
    /// From the stored thumbnail.
    Encode,
    /// From the asset's file.
    Draw,
}

impl Stale {
    fn redo(&self, px: u32, q: u8) -> Redo {
        let own = self.thumb == self.blob;
        let needs_more = self.drawn < self.current
            || (!own && (self.px.is_some_and(|p| p < px) || self.q.is_some_and(|old| old < q)));
        if self.drawable && needs_more { Redo::Draw } else { Redo::Encode }
    }

    /// Whether its own file could be its thumbnail, at most `px` across.
    fn could_be_own(&self, px: u32) -> bool {
        self.still
            && !self.pixel_art
            && AS_IS.contains(&self.ext.as_str())
            && self.dims.is_some_and(|(w, h)| w.max(h) <= px)
    }
}

async fn fill(state: &AppState) -> Result<()> {
    let (px, q) = (crate::analyze::thumb_size(), crate::analyze::thumb_quality());
    db::call(&state.pool, move |c| Ok(mark_now(c, px, q)?)).await?;
    let avif = db::call(&state.pool, |c| Ok(take_avif(c)?)).await?;
    if !avif.is_empty() {
        let (count, bytes) = crate::jobs::delete_unused(state, avif).await?;
        tracing::info!(count, freed = %crate::maintenance::human_bytes(bytes), "dropped the AVIF copies of thumbnails");
    }
    let found = db::call(&state.pool, move |c| Ok(stale(c, px, q)?)).await?;
    let due: Vec<Stale> = {
        let tried = TRIED.lock().unwrap_or_else(PoisonError::into_inner);
        found.into_iter().filter(|s| !tried.contains(&s.id)).collect()
    };
    if due.is_empty() {
        return Ok(());
    }
    let (draw, encode): (Vec<Stale>, Vec<Stale>) = due.into_iter().partition(|s| s.redo(px, q) == Redo::Draw);
    if !draw.is_empty() {
        redraw(state, draw).await?;
    }
    if encode.is_empty() {
        return Ok(());
    }
    let activity = crate::activity::start("thumbs", format!("Making {} thumbnails again", encode.len()), None);
    let (mut made, mut done, mut freed) = (0usize, 0usize, 0i64);
    // Several at once, as a library can have tens of thousands, but on half the cores and below everything else, so
    // what's being added meanwhile is still read first.
    let cores = std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1));
    let mut results = stream::iter(encode.iter().cloned())
        .map(|s| {
            let state = std::sync::Arc::clone(state);
            async move {
                // Settings changed meanwhile: the next round goes by them.
                if (crate::analyze::thumb_size(), crate::analyze::thumb_quality()) != (px, q) {
                    return None;
                }
                let saved = encode_again(&state, &s, px, q).await;
                Some((s.id, saved))
            }
        })
        .buffer_unordered(cores);
    while let Some(result) = results.next().await {
        done += 1;
        #[expect(clippy::cast_precision_loss, reason = "a fraction for a progress bar")]
        activity.progress(done as f32 / encode.len() as f32);
        match result {
            None => {}
            Some((_, Ok(saved))) => {
                made += 1;
                freed += saved;
            }
            Some((id, Err(e))) => {
                tracing::debug!(asset = %id, error = %format!("{e:#}"), "couldn't make a thumbnail again");
                TRIED.lock().unwrap_or_else(PoisonError::into_inner).insert(id);
            }
        }
    }
    tracing::info!(made, of = encode.len(), freed = %crate::maintenance::human_bytes(freed.max(0).unsigned_abs()), "made thumbnails again");
    crate::looks::schedule(state).await;
    Ok(())
}

/// Forget the AVIF copies of thumbnails made for browsers before thumbnails were WebP, returning their blobs (hash,
/// size) to delete where nothing else uses them.
fn take_avif(c: &Connection) -> rusqlite::Result<Vec<(String, i64)>> {
    let tx = c.unchecked_transaction()?;
    let blobs = tx
        .prepare(
            "SELECT DISTINCT d.blob_hash, COALESCE(b.size, 0) FROM derivatives d
               LEFT JOIN blobs b ON b.hash = d.blob_hash WHERE d.kind LIKE 'as:thumb.avif:%'",
        )?
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    tx.execute("DELETE FROM derivatives WHERE kind LIKE 'as:thumb.avif:%'", [])?;
    tx.commit()?;
    Ok(blobs)
}

/// The thumbnails made otherwise than at `px` across and quality `q`, or by an older drawing of their kind, a pack at
/// a time. A thumbnail a browser drew is left be.
fn stale(c: &Connection, px: u32, q: u8) -> rusqlite::Result<Vec<Stale>> {
    let current = "COALESCE((SELECT rev FROM thumb_drawing WHERE kind = a.kind),
                            (SELECT rev FROM thumb_drawing WHERE kind = '*'), 1)";
    let sql = format!(
        "SELECT a.id, a.pack_id, a.ext, a.thumb_hash, COALESCE(tb.size, 0), a.blob_hash, COALESCE(ob.size, 0),
                a.thumb_drawn, {current}, a.thumb_px, a.thumb_q,
                COALESCE(json_extract(a.meta, '$.\"pixel-art-likely\"'), 0),
                json_extract(a.meta, '$.width'), json_extract(a.meta, '$.height'),
                json_extract(a.meta, '$.frames') IS NULL AND json_extract(a.meta, '$.hdr') IS NULL,
                a.kind <> 'animation' AND a.ext <> 'blend' AND NOT {} AND NOT {} AND p.status = 'ready',
                a.kind IN ('document', 'data', 'shader', 'other') AND a.ext <> 'pdf'
           FROM assets a JOIN packs p ON p.id = a.pack_id
           LEFT JOIN blobs tb ON tb.hash = a.thumb_hash LEFT JOIN blobs ob ON ob.hash = a.blob_hash
          WHERE a.thumb_hash IS NOT NULL AND a.thumb_source IS NULL AND a.archived_at IS NULL
            AND (a.thumb_drawn < {current} OR a.thumb_px IS NOT ?1 OR a.thumb_q IS NOT ?2)
          ORDER BY a.pack_id, a.path",
        crate::materials::IS_MATERIAL,
        crate::takes::IS_TAKE,
    );
    c.prepare(&sql)?
        .query_map(params![px, q], |r| {
            let (w, h): (Option<u32>, Option<u32>) = (r.get(12)?, r.get(13)?);
            Ok(Stale {
                id: r.get(0)?,
                pack_id: r.get(1)?,
                ext: r.get(2)?,
                thumb: r.get(3)?,
                thumb_size: r.get(4)?,
                blob: r.get(5)?,
                blob_size: r.get(6)?,
                drawn: r.get(7)?,
                current: r.get(8)?,
                px: r.get(9)?,
                q: r.get(10)?,
                pixel_art: r.get::<_, i64>(11)? != 0,
                dims: w.zip(h),
                still: r.get(14)?,
                drawable: r.get(15)?,
                text: r.get(16)?,
            })
        })?
        .collect()
}

/// Analyze these assets' files again, which draws their thumbnails, a job for each pack (unless it's being analyzed
/// already, which draws them anyway).
async fn redraw(state: &AppState, draw: Vec<Stale>) -> Result<()> {
    let mut by_pack: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for s in draw {
        by_pack.entry(s.pack_id).or_default().push(s.id);
    }
    TRIED.lock().unwrap_or_else(PoisonError::into_inner).extend(by_pack.values().flatten().cloned());
    let count: usize = by_pack.values().map(Vec::len).sum();
    let queued = db::call(&state.pool, move |c| {
        let mut queued = 0;
        for (pack, ids) in &by_pack {
            if !db::job_pending(c, crate::jobs::RETAG, Some(pack), None)? {
                db::enqueue(c, crate::jobs::RETAG, Some(pack), &json!({ "assets": ids, "then_llm": false }))?;
                queued += 1;
            }
        }
        Ok(queued)
    })
    .await?;
    if queued > 0 {
        tracing::info!(thumbnails = count, packs = queued, "drawing thumbnails again");
        state.wake.notify_one();
    }
    Ok(())
}

/// What a stale thumbnail becomes.
#[derive(Debug, PartialEq, Eq)]
enum Pick {
    /// The stored thumbnail stays as it is.
    Keep,
    /// The asset's own file.
    Own,
    /// The thumbnail made again.
    Made,
}

/// The smallest of the thumbnail as it is (when it's WebP or the asset's own file), the asset's own file (when it
/// could be its thumbnail) and the one made again, in that order when they're as small. One kept bigger than `px`
/// is shown at `px` all the same: shrinking lines drawn sharp, like text, can make more bytes, not fewer.
fn pick(s: &Stale, px: u32, webp: bool, made: usize) -> Pick {
    let own = s.could_be_own(px);
    let made = i64::try_from(made).unwrap_or(i64::MAX);
    let keep = webp || s.thumb == s.blob;
    if keep && s.thumb_size <= made && (!own || s.thumb_size <= s.blob_size) {
        Pick::Keep
    } else if own && s.blob_size <= made {
        Pick::Own
    } else {
        Pick::Made
    }
}

/// Make a thumbnail again from the stored one, keeping the smallest of it as it is, the asset's own file and the new
/// one, and mark it made now. Returns the bytes freed (less than nothing when it grew).
async fn encode_again(state: &AppState, s: &Stale, px: u32, q: u8) -> Result<i64> {
    let stored = state.blobs.read_all(&s.thumb).await?;
    let webp = crate::analyze::thumb_mime(&stored) == "image/webp";
    let (pixel_art, exact) = (s.pixel_art, s.pixel_art || s.text);
    let made = crate::heavy::run_idle("thumb-again", move || -> Result<Vec<u8>> {
        let img = image::load_from_memory(&stored)?;
        let filter =
            if pixel_art { image::imageops::FilterType::Nearest } else { image::imageops::FilterType::Lanczos3 };
        let whole = crate::analyze::thumb_bytes_at(&img.to_rgba8(), exact, q)?;
        if img.width().max(img.height()) <= px {
            return Ok(whole);
        }
        // Shrinking lines drawn sharp, like text, can make more bytes, not fewer.
        let smaller = crate::analyze::thumb_bytes_at(&img.resize(px, px, filter).to_rgba8(), exact, q)?;
        Ok(if whole.len() < smaller.len() { whole } else { smaller })
    })
    .await??;
    let new = match pick(s, px, webp, made.len()) {
        Pick::Keep => None,
        // Copied into the local cache, so showing it never reads the library's drive.
        Pick::Own => Some(state.blobs.put_cache_bytes(state.blobs.read_all(&s.blob).await?.to_vec()).await?),
        Pick::Made => Some(state.blobs.put_cache_bytes(made).await?),
    };
    let (id, old, current) = (s.id.clone(), s.thumb.clone(), s.current);
    let new2 = new.clone();
    let replaced = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        if let Some(b) = &new2 {
            tx.execute(
                "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
                params![b.hash, i64::try_from(b.size).unwrap_or(i64::MAX), db::now()],
            )?;
            // Unless it was replaced meanwhile.
            if tx.execute(
                "UPDATE assets SET thumb_hash = ?3 WHERE id = ?1 AND thumb_hash = ?2 AND thumb_source IS NULL",
                params![id, old, b.hash],
            )? == 0
            {
                return Ok(false);
            }
            carry_looks(&tx, &old, &b.hash)?;
        }
        tx.execute(
            "UPDATE assets SET thumb_drawn = MAX(thumb_drawn, ?2), thumb_px = ?3, thumb_q = ?4
              WHERE id = ?1 AND thumb_source IS NULL",
            params![id, current, px, q],
        )?;
        tx.commit()?;
        Ok(new2.is_some())
    })
    .await?;
    let Some(new) = new.filter(|_| replaced) else { return Ok(0) };
    let old_size = s.thumb_size;
    crate::jobs::delete_unused(state, vec![(s.thumb.clone(), old_size)]).await?;
    Ok(old_size - i64::try_from(new.size).unwrap_or(i64::MAX))
}

/// What image search and search by look made of a thumbnail, for one made from it: it looks the same.
fn carry_looks(c: &Connection, from: &str, to: &str) -> rusqlite::Result<()> {
    c.execute(
        "INSERT OR IGNORE INTO looks (thumb_hash, version, palette, descriptor, phash, print)
         SELECT ?2, version, palette, descriptor, phash, print FROM looks WHERE thumb_hash = ?1",
        [from, to],
    )?;
    for table in ["look_embeddings", "embeddings"] {
        c.execute(
            &format!(
                "INSERT OR IGNORE INTO {table} (thumb_hash, model, vector)
                 SELECT ?2, model, vector FROM {table} WHERE thumb_hash = ?1"
            ),
            [from, to],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> (tempfile::TempDir, db::Pool) {
        let dir = tempfile::tempdir().unwrap();
        let pool = db::open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        pool.get()
            .unwrap()
            .execute_batch(
                "INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'Rocks', 'ready', 'x');
                 INSERT INTO blobs (hash, size, created_at)
                   VALUES ('orig', 900, 'x'), ('t1', 5000, 'x'), ('t2', 3000, 'x');",
            )
            .unwrap();
        (dir, pool)
    }

    fn add(c: &Connection, id: &str, ext: &str, kind: &str, thumb: Option<&str>) {
        c.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, thumb_hash, meta, created_at, updated_at)
             VALUES (?1, 'p', 'orig', ?1 || '.' || ?2, ?1, ?2, ?3, ?4, '{\"width\": 32, \"height\": 32}', 'x', 'x')",
            params![id, ext, kind, thumb],
        )
        .unwrap();
    }

    fn mark(c: &Connection, id: &str) -> (i64, Option<u32>, Option<u8>) {
        c.query_row("SELECT thumb_drawn, thumb_px, thumb_q FROM assets WHERE id = ?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .unwrap()
    }

    #[test]
    fn a_thumbnail_written_anywhere_is_marked_with_how_thumbnails_are_made_now() {
        let (_dir, pool) = db();
        let c = pool.get().unwrap();
        c.execute("UPDATE thumb_now SET px = 128, q = 60", []).unwrap();
        c.execute("INSERT INTO thumb_drawing (kind, rev) VALUES ('model', 3), ('*', 1)", []).unwrap();
        add(&c, "rock", "glb", "model", Some("t1"));
        assert_eq!(mark(&c, "rock"), (3, Some(128), Some(60)), "written with the asset");
        add(&c, "wall", "png", "texture", None);
        assert_eq!(mark(&c, "wall"), (0, None, None), "no thumbnail, nothing to mark");
        c.execute("UPDATE thumb_now SET px = 256, q = 75", []).unwrap();
        c.execute("UPDATE assets SET thumb_hash = 't2' WHERE id = 'wall'", []).unwrap();
        assert_eq!(mark(&c, "wall"), (1, Some(256), Some(75)), "written afterwards; its kind's drawing is the first");
        c.execute("UPDATE thumb_now SET px = 128", []).unwrap();
        c.execute("UPDATE assets SET name = 'Wall', thumb_hash = 't2' WHERE id = 'wall'", []).unwrap();
        assert_eq!(mark(&c, "wall"), (1, Some(256), Some(75)), "the same thumbnail is as it was made");
    }

    #[test]
    fn thumbnails_made_otherwise_than_now_are_found_and_those_a_browser_drew_are_left_be() {
        let (_dir, pool) = db();
        let c = pool.get().unwrap();
        add(&c, "now", "png", "texture", Some("t1"));
        add(&c, "sharper", "png", "texture", Some("t2"));
        add(&c, "drawn", "glb", "model", Some("t2"));
        add(&c, "anim", "fbx", "animation", Some("t2"));
        add(&c, "notes", "md", "document", Some("t2"));
        c.execute_batch(
            "UPDATE assets SET thumb_q = 60 WHERE id = 'sharper';
             UPDATE assets SET thumb_q = NULL, thumb_source = 'browser' WHERE id = 'drawn';
             UPDATE assets SET thumb_drawn = 0 WHERE id = 'anim';
             UPDATE assets SET thumb_px = 128 WHERE id = 'notes';",
        )
        .unwrap();
        let found = stale(&c, 256, 75).unwrap();
        let ids: Vec<&str> = found.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["anim", "notes", "sharper"]);
        let anim = &found[0];
        assert!(!anim.drawable, "motion draws animations");
        assert_eq!(anim.redo(256, 75), Redo::Encode);
        assert!(found[1].text, "text stays exact");
        let sharper = &found[2];
        assert!(sharper.drawable && sharper.still && !sharper.text);
        assert_eq!(sharper.dims, Some((32, 32)));
        assert_eq!(sharper.redo(256, 75), Redo::Draw, "the stored one hasn't the detail for a higher quality");
    }

    fn stale_one() -> Stale {
        Stale {
            id: "a".into(),
            ext: "png".into(),
            thumb: "t".into(),
            thumb_size: 4000,
            blob: "orig".into(),
            blob_size: 900,
            drawn: FIRST,
            current: FIRST,
            px: Some(256),
            q: Some(75),
            dims: Some((32, 32)),
            still: true,
            drawable: true,
            ..Stale::default()
        }
    }

    #[test]
    fn a_thumbnail_is_drawn_again_only_for_what_the_stored_one_cant_give() {
        let s = stale_one();
        assert_eq!(s.redo(128, 75), Redo::Encode, "smaller");
        assert_eq!(s.redo(256, 60), Redo::Encode, "a lower quality");
        assert_eq!(Stale { q: None, ..stale_one() }.redo(256, 75), Redo::Encode, "a JPEG or PNG from before");
        assert_eq!(s.redo(512, 75), Redo::Draw, "bigger");
        assert_eq!(s.redo(256, 88), Redo::Draw, "a higher quality");
        assert_eq!(Stale { drawn: 0, ..stale_one() }.redo(256, 75), Redo::Draw, "an older drawing");
        assert_eq!(Stale { drawn: 0, drawable: false, ..stale_one() }.redo(256, 75), Redo::Encode);
        let own = Stale { thumb: "orig".into(), ..stale_one() };
        assert_eq!(own.redo(256, 88), Redo::Encode, "its own file is as sharp as it gets");
    }

    #[test]
    fn the_smallest_picture_wins_and_a_small_one_is_its_own_thumbnail() {
        let s = stale_one();
        // The sprite's own 900 bytes beat a 4000 byte thumbnail kept or one of 2000 made again.
        assert_eq!(pick(&s, 256, true, 2000), Pick::Own);
        assert_eq!(pick(&s, 256, true, 800), Pick::Made);
        // A picture too big to be its own thumbnail, or one browsers don't show as it is.
        let big = Stale { dims: Some((1024, 512)), ..stale_one() };
        assert_eq!(pick(&big, 256, true, 2000), Pick::Made);
        assert_eq!(pick(&big, 256, true, 5000), Pick::Keep, "a WebP kept when the new one is no smaller");
        assert_eq!(pick(&big, 256, false, 5000), Pick::Made, "a JPEG or PNG from before becomes WebP");
        assert_eq!(pick(&big, 128, true, 3000), Pick::Made, "smaller made smaller");
        assert_eq!(pick(&big, 128, true, 5000), Pick::Keep, "bigger, but fewer bytes than made smaller");
        let tga = Stale { ext: "tga".into(), ..stale_one() };
        assert_eq!(pick(&tga, 256, true, 2000), Pick::Made);
        let animated = Stale { still: false, ..stale_one() };
        assert_eq!(pick(&animated, 256, true, 2000), Pick::Made);
        let pixel_art = Stale { pixel_art: true, ..stale_one() };
        assert_eq!(pick(&pixel_art, 256, true, 2000), Pick::Made, "its pixels are kept square, made bigger");
        // Already its own thumbnail: it stays while it's the fewest bytes.
        let own = Stale { thumb: "orig".into(), thumb_size: 900, ..stale_one() };
        assert_eq!(pick(&own, 256, false, 2000), Pick::Keep);
        let own_big = Stale { dims: Some((200, 200)), ..own };
        assert_eq!(pick(&own_big, 128, false, 2000), Pick::Keep, "fewer bytes than made smaller");
        assert_eq!(pick(&own_big, 128, false, 800), Pick::Made);
    }

    #[test]
    fn what_search_made_of_a_thumbnail_carries_over_to_the_one_made_from_it() {
        let (_dir, pool) = db();
        let c = pool.get().unwrap();
        c.execute_batch(
            "INSERT INTO looks (thumb_hash, version, palette, descriptor, phash, print)
               VALUES ('t1', 2, x'01', x'02', 7, x'03');
             INSERT INTO embeddings (thumb_hash, model, vector) VALUES ('t1', 'siglip', x'0405');
             INSERT INTO look_embeddings (thumb_hash, model, vector) VALUES ('t1', 'm', x'06');",
        )
        .unwrap();
        carry_looks(&c, "t1", "t2").unwrap();
        let count = |sql: &str| c.query_row(sql, [], |r| r.get::<_, i64>(0)).unwrap();
        assert_eq!(count("SELECT count(*) FROM looks WHERE thumb_hash = 't2' AND phash = 7 AND version = 2"), 1);
        assert_eq!(count("SELECT count(*) FROM embeddings WHERE thumb_hash = 't2' AND model = 'siglip'"), 1);
        assert_eq!(count("SELECT count(*) FROM look_embeddings WHERE thumb_hash = 't2'"), 1);
        // Carried again (a thumbnail shared by two assets): what's there stays.
        carry_looks(&c, "t1", "t2").unwrap();
    }

    #[test]
    fn the_avif_copies_of_thumbnails_are_let_go() {
        let (_dir, pool) = db();
        let c = pool.get().unwrap();
        add(&c, "rock", "glb", "model", Some("t1"));
        c.execute_batch(
            "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at)
               VALUES ('rock', 'as:thumb.avif:t1', 't2', 'x'), ('rock', 'as:view:0:glb', 'orig', 'x');",
        )
        .unwrap();
        assert_eq!(take_avif(&c).unwrap(), [("t2".to_owned(), 3000)]);
        let left: Vec<String> = c
            .prepare("SELECT kind FROM derivatives")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(left, ["as:view:0:glb"]);
        assert_eq!(take_avif(&c).unwrap(), []);
    }

    #[test]
    fn the_database_is_told_how_thumbnails_are_made_now() {
        let (_dir, pool) = db();
        let c = pool.get().unwrap();
        mark_now(&c, 128, 60).unwrap();
        mark_now(&c, 128, 60).unwrap();
        let now: (u32, u8) = c.query_row("SELECT px, q FROM thumb_now", [], |r| Ok((r.get(0)?, r.get(1)?))).unwrap();
        assert_eq!(now, (128, 60));
        let star: i64 = c.query_row("SELECT rev FROM thumb_drawing WHERE kind = '*'", [], |r| r.get(0)).unwrap();
        assert_eq!(star, FIRST);
        assert_eq!(drawn("model"), FIRST);
    }
}
