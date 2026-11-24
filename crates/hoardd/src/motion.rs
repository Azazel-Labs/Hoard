//! Animations drawn moving. Every animation's thumbnail is a still from the middle of its clip (its `still`
//! derivative marks it drawn), so a grid of them isn't a grid of the same rest pose. Motion thumbnails go further: an
//! animation's tile plays it when pointed at, a few seconds of the character moving as an animated WebP (its `motion`
//! derivative), off until turned on in Settings → Thumbnails, as each takes room and a few seconds of rendering.
//!
//! Every clip is played on its skeleton's character (see `animations::plays_on`), the one the viewer plays it on too,
//! so a grid of a skeleton's clips is one character moving many ways: its keys are copied onto that character's
//! bones, matched by name. A clip that moves none of them is played on its own mesh, when it has one.
//! The character is posed at each frame by polysmelt and drawn by the thumbnail renderer in a tiled room scaled to it.
//! A clip that stays put is drawn from one camera, so the character moves and the room holds still. One that goes
//! anywhere (a walk, a run, a fall) is followed close in, so the character stays big and the tiles stream past.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex, PoisonError};

use anyhow::{Context, Result, anyhow};
use hoard_core::Facing;
use polysmelt::document::{Document, Pose};
use rusqlite::{OptionalExtension, params};
use serde_json::{Map, Value, json};

use crate::{AppState, db};

/// The longest stretch of a clip played: a longer one is cut there.
const MAX_SECS: f64 = 3.0;
/// The frame's side, as a still thumbnail's.
const SIZE: u32 = 256;
/// libwebp's lossy quality, 0 to 100.
const QUALITY: f32 = 70.0;
/// Which drawing of an animation's still and motion thumbnail this is, kept with each (`derivatives.rev`). One more
/// whenever a fix changes how they're drawn: those drawn before are drawn again in the background, each showing until
/// its new one is in.
///
/// 1. A clip from a file in other units (an FBX in centimeters) moves the character it's drawn on by as much in its
///    own units, not 100 times as far.
pub(crate) const DRAWN: i64 = 1;

/// Whether a fill is running, and whether another was asked for while it ran.
static BUSY: AtomicBool = AtomicBool::new(false);
static AGAIN: AtomicBool = AtomicBool::new(false);
/// Animations that couldn't be drawn since the server started, which aren't tried again until then.
static FAILED: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);
/// Clips with no mesh that fit no character yet, tried again when a pack comes in (it may bring one).
static WAITING: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);
/// How many packs have come in, so a clip found to fit nothing while one came in is tried again rather than put aside.
static PACKS_IN: AtomicU64 = AtomicU64::new(0);
/// The packs come in and the skeletons' revision when animations were last checked for being drawn on the character
/// they play on, so they're checked again only when either may have changed it.
static CHECKED: Mutex<Option<(u64, u64)>> = Mutex::new(None);

/// Why a clip with no mesh of its own can't be drawn: it fits no character.
#[derive(Debug)]
struct NoCharacter;

impl std::fmt::Display for NoCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("it fits no character to play on")
    }
}

impl std::error::Error for NoCharacter {}

/// Draw the mid-clip stills the library's animations lack, and their motion thumbnails when those are turned on, in
/// the background. Asked for again while it runs, it goes round once more when done, for what was added meanwhile.
pub(crate) fn kick(state: &AppState) {
    if !state.settings.get().render {
        return;
    }
    AGAIN.store(true, Ordering::SeqCst);
    if BUSY.swap(true, Ordering::SeqCst) {
        return;
    }
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        loop {
            while AGAIN.swap(false, Ordering::SeqCst) {
                if let Err(e) = fill(&state).await {
                    tracing::warn!(error = %format!("{e:#}"), "couldn't make motion thumbnails");
                }
            }
            BUSY.store(false, Ordering::SeqCst);
            // Asked again between the last round and letting go: take it back and go round.
            if !AGAIN.load(Ordering::SeqCst) || BUSY.swap(true, Ordering::SeqCst) {
                break;
            }
        }
    });
}

/// A pack came in: draw what it brought, and try again the clips that fit no character, as it may have brought one.
pub(crate) fn pack_in(state: &AppState) {
    PACKS_IN.fetch_add(1, Ordering::SeqCst);
    WAITING.lock().unwrap_or_else(PoisonError::into_inner).clear();
    kick(state);
}

/// Forget every motion thumbnail and delete their files, when they're turned off.
pub(crate) async fn remove_all(state: &AppState) -> Result<()> {
    let blobs = db::call(&state.pool, |c| {
        let blobs: Vec<(String, i64)> = c
            .prepare(
                "SELECT d.blob_hash, b.size FROM derivatives d JOIN blobs b ON b.hash = d.blob_hash
                 WHERE d.kind = 'motion'",
            )?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        c.execute("DELETE FROM derivatives WHERE kind = 'motion'", [])?;
        Ok(blobs)
    })
    .await?;
    let (count, bytes) = crate::jobs::delete_unused(state, blobs).await?;
    tracing::info!(count, freed = %crate::maintenance::human_bytes(bytes), "removed motion thumbnails");
    Ok(())
}

/// Draw these animations again (another character was chosen for their skeleton): forget their thumbnails, deleting
/// the files nothing else shows, and go round.
pub(crate) async fn redraw(state: &AppState, ids: Vec<String>) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    forget(state, ids).await?;
    kick(state);
    Ok(())
}

/// Forget these animations' thumbnails, deleting their files, so the next round draws them.
async fn forget(state: &AppState, ids: Vec<String>) -> Result<()> {
    let blobs = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let mut blobs = Vec::new();
        for id in &ids {
            let mut stmt = tx.prepare(
                "SELECT d.blob_hash, b.size FROM derivatives d JOIN blobs b ON b.hash = d.blob_hash
                 WHERE d.asset_id = ?1 AND d.kind IN ('motion', 'still')",
            )?;
            blobs.extend(
                stmt.query_map([id], |r| Ok((r.get(0)?, r.get(1)?)))?
                    .collect::<rusqlite::Result<Vec<(String, i64)>>>()?,
            );
            drop(stmt);
            tx.execute("DELETE FROM derivatives WHERE asset_id = ?1 AND kind IN ('motion', 'still')", [id])?;
        }
        tx.commit()?;
        Ok(blobs)
    })
    .await?;
    crate::jobs::delete_unused(state, blobs).await?;
    FAILED.lock().unwrap_or_else(PoisonError::into_inner).clear();
    Ok(())
}

/// Draw again the animations drawn on a character they no longer play on: a pack came in with a better dressed
/// character of their skeleton, or the one they were drawn on went. Checked once after the server starts, and again
/// after a pack comes in or a rig changes.
async fn redraw_moved(state: &AppState) -> Result<()> {
    let now = (PACKS_IN.load(Ordering::SeqCst), crate::animations::revision());
    if *CHECKED.lock().unwrap_or_else(PoisonError::into_inner) == Some(now) {
        return Ok(());
    }
    let moved = crate::animations::drawn_on_another(state).await?;
    if !moved.is_empty() {
        tracing::info!(count = moved.len(), "drawing animations again on their skeleton's character");
        forget(state, moved).await?;
    }
    *CHECKED.lock().unwrap_or_else(PoisonError::into_inner) = Some(now);
    Ok(())
}

/// An animation to draw: its motion thumbnail, its mid-clip still, or both.
struct Todo {
    id: String,
    path: String,
    pack_id: String,
    motion: bool,
    still: bool,
}

async fn fill(state: &AppState) -> Result<()> {
    redraw_moved(state).await?;
    let motion_on = state.settings.get().motion_thumbs;
    // A thumbnail a browser drew is kept: it drew the clip mid-way too, with every texture. One drawn before the last
    // fix to drawing is drawn again; those not drawn at all go first.
    let todo: Vec<Todo> = db::call(&state.pool, move |c| {
        let rows = c
            .prepare(
                "SELECT id, path, pack_id, motion, still FROM (
                   SELECT a.id, a.path, a.pack_id,
                          ?1 AND NOT EXISTS (SELECT 1 FROM derivatives d
                                             WHERE d.asset_id = a.id AND d.kind = 'motion' AND d.rev >= ?2) AS motion,
                          a.thumb_source IS NULL
                            AND NOT EXISTS (SELECT 1 FROM derivatives d
                                            WHERE d.asset_id = a.id AND d.kind = 'still' AND d.rev >= ?2) AS still,
                          EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind IN ('motion', 'still'))
                            AS drawn
                     FROM assets a WHERE a.kind = 'animation' AND a.archived_at IS NULL)
                  WHERE motion OR still ORDER BY drawn, pack_id, path",
            )?
            .query_map(params![motion_on, DRAWN], |r| {
                Ok(Todo { id: r.get(0)?, path: r.get(1)?, pack_id: r.get(2)?, motion: r.get(3)?, still: r.get(4)? })
            })?
            .collect::<rusqlite::Result<_>>()?;
        Ok(rows)
    })
    .await?;
    let todo: Vec<Todo> = {
        let failed = FAILED.lock().unwrap_or_else(PoisonError::into_inner);
        let waiting = WAITING.lock().unwrap_or_else(PoisonError::into_inner);
        todo.into_iter().filter(|t| !failed.contains(&t.id) && !waiting.contains(&t.id)).collect()
    };
    if todo.is_empty() {
        return Ok(());
    }
    let label = if todo.iter().any(|t| t.motion) {
        format!("Making motion thumbnails of {} animations", todo.len())
    } else {
        format!("Drawing {} animations mid-clip", todo.len())
    };
    let activity = crate::activity::start("motion", label, None);
    let mut made = 0;
    for (i, t) in todo.iter().enumerate() {
        let settings = state.settings.get();
        // Thumbnails turned off meanwhile: stop. Motion thumbnails turned off: only the still.
        if !settings.render {
            break;
        }
        let motion = t.motion && settings.motion_thumbs;
        if !motion && !t.still {
            continue;
        }
        #[expect(clippy::cast_precision_loss, reason = "a fraction for a progress bar")]
        activity.progress(i as f32 / todo.len() as f32);
        let packs = PACKS_IN.load(Ordering::SeqCst);
        match make(state, t, motion, settings.motion_fps).await {
            Ok(()) => made += 1,
            // A pack that came in meanwhile asked for another round, which tries it again.
            Err(e) if e.is::<NoCharacter>() && PACKS_IN.load(Ordering::SeqCst) != packs => {}
            Err(e) if e.is::<NoCharacter>() => {
                WAITING.lock().unwrap_or_else(PoisonError::into_inner).insert(t.id.clone());
            }
            Err(e) => {
                tracing::info!(path = %t.path, pack = %t.pack_id, error = %format!("{e:#}"), "couldn't draw an animation");
                FAILED.lock().unwrap_or_else(PoisonError::into_inner).insert(t.id.clone());
            }
        }
    }
    tracing::info!(made, of = todo.len(), "drew animations");
    Ok(())
}

/// A model's files laid out in a folder of its own, and the one to read.
pub(crate) struct Local {
    pub(crate) path: PathBuf,
    pub(crate) ext: String,
    pub(crate) root: PathBuf,
}

/// Lay out an asset's file, and those it needs (a model's textures), under `work`. A model only Blender reads is laid
/// out as the GLB Blender made of it.
pub(crate) async fn fetch(state: &AppState, id: &str, work: &Path) -> Result<Local> {
    let id2 = id.to_owned();
    let (path, ext, files) = db::call(&state.pool, move |c| {
        let (path, ext): (String, String) =
            c.query_row("SELECT path, ext FROM assets WHERE id = ?1", [&id2], |r| Ok((r.get(0)?, r.get(1)?)))?;
        let ext = ext.to_ascii_lowercase();
        if crate::meshes::reads(&ext) {
            let ids = db::dependency_closure(c, &id2, false)?;
            return Ok((path, ext, db::bundle_files(c, &ids)?));
        }
        let glb: Option<String> = c
            .query_row("SELECT blob_hash FROM derivatives WHERE asset_id = ?1 AND kind = 'glb'", [&id2], |r| r.get(0))
            .optional()?;
        let glb = glb.with_context(|| format!("{path} hasn't been converted (is Blender on?)"))?;
        let path = format!("{}.glb", path.rsplit_once('.').map_or(path.as_str(), |(s, _)| s));
        let file = db::BundleFile { id: id2.clone(), path: path.clone(), hash: glb, size: 0, unity_guid: None };
        Ok((path, "glb".to_owned(), vec![file]))
    })
    .await?;
    for f in &files {
        let dest = work.join(crate::ingest::safe_rel_path(&f.path).context("unsafe path in pack")?);
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        state.blobs.download_to(&f.hash, &dest).await?;
    }
    let local = work.join(crate::ingest::safe_rel_path(&path).context("unsafe path in pack")?);
    Ok(Local { path: local, ext, root: work.to_owned() })
}

/// Draw one animation: its still from the middle of the clip, which becomes its thumbnail (unless a browser drew
/// one), and its motion thumbnail when `motion`, at `fps` frames a second.
async fn make(state: &AppState, t: &Todo, motion: bool, fps: u32) -> Result<()> {
    let pack = t.pack_id.clone();
    // Its skeleton's character, as everywhere else shows it.
    let body = crate::animations::plays_on(state, t.id.clone()).await?.map(|f| (f.id, f.pack_id));
    let (id, b) = (t.id.clone(), body.clone());
    let facings = db::call(&state.pool, move |c| {
        let clip_facing = db::asset_facing(c, &id, &pack)?;
        let body_facing = match &b {
            Some((b, pack)) => db::asset_facing(c, b, pack)?,
            None => clip_facing,
        };
        Ok((clip_facing, body_facing))
    })
    .await?;
    let work = state.cfg.tmp_dir().join(format!("motion-{}", uuid::Uuid::now_v7()));
    let result = async {
        let clip = fetch(state, &t.id, &work.join("clip")).await?;
        let local = match &body {
            Some((b, _)) => Some(fetch(state, b, &work.join("body")).await?),
            None => None,
        };
        let (webp, still, on_body) =
            crate::heavy::run("motion", move || draw(&clip, local.as_ref(), facings, motion.then_some(fps))).await??;
        let drawn_on = body.filter(|_| on_body).map(|(b, _)| b);
        let motion = match webp {
            Some(webp) => Some(state.blobs.put_cache_bytes(webp).await?),
            None => None,
        };
        let still = state.blobs.put_cache_bytes(still).await?;
        let id = t.id.clone();
        let replaced = db::call(&state.pool, move |c| {
            let now = db::now();
            let tx = c.transaction()?;
            for b in motion.iter().chain([&still]) {
                tx.execute(
                    "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
                    params![b.hash, i64::try_from(b.size).unwrap_or(i64::MAX), now],
                )?;
            }
            let kinds = motion.iter().map(|m| ("motion", m)).chain([("still", &still)]);
            // What it replaces (an older drawing of it) goes, unless something still shows it.
            let mut old: Vec<(String, i64)> = Vec::new();
            for (kind, b) in kinds {
                old.extend(
                    tx.query_row(
                        "SELECT b.hash, b.size FROM derivatives d JOIN blobs b ON b.hash = d.blob_hash
                         WHERE d.asset_id = ?1 AND d.kind = ?2 AND d.blob_hash <> ?3",
                        params![id, kind, b.hash],
                        |r| Ok((r.get(0)?, r.get(1)?)),
                    )
                    .optional()?,
                );
                tx.execute(
                    "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at, rev) VALUES (?1, ?2, ?3, ?4, ?5)
                     ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash,
                       created_at = excluded.created_at, rev = excluded.rev",
                    params![id, kind, b.hash, now, DRAWN],
                )?;
            }
            // As does the rest pose it was drawn in.
            old.extend(
                tx.query_row(
                    "SELECT b.hash, b.size FROM assets a JOIN blobs b ON b.hash = a.thumb_hash
                     WHERE a.id = ?1 AND a.thumb_source IS NULL AND a.thumb_hash <> ?2",
                    params![id, still.hash],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()?,
            );
            tx.execute(
                "UPDATE assets SET thumb_hash = ?2 WHERE id = ?1 AND thumb_source IS NULL",
                params![id, still.hash],
            )?;
            match &drawn_on {
                Some(b) => tx.execute(
                    "INSERT INTO drawn_on (asset_id, body_id) VALUES (?1, ?2)
                     ON CONFLICT (asset_id) DO UPDATE SET body_id = excluded.body_id",
                    params![id, b],
                )?,
                None => tx.execute("DELETE FROM drawn_on WHERE asset_id = ?1", [&id])?,
            };
            tx.commit()?;
            Ok(old)
        })
        .await?;
        if !replaced.is_empty() {
            crate::jobs::delete_unused(state, replaced).await?;
        }
        Ok(())
    }
    .await;
    tokio::fs::remove_dir_all(&work).await.ok();
    result
}

/// The clip drawn: its middle as a still thumbnail, and when `motion` all its frames as an animated WebP; and whether
/// it was drawn on `body` (its skeleton's character), which it is whenever it moves that character's bones, so every
/// clip of a skeleton is drawn on the same one. Otherwise it's drawn on its own mesh, when it has one.
/// `facings` turn the clip's own mesh and the character's: each its own way (see [`db::asset_facing`]).
fn draw(
    clip: &Local,
    body: Option<&Local>,
    facings: (Facing, Facing),
    motion: Option<u32>,
) -> Result<(Option<Vec<u8>>, Vec<u8>, bool)> {
    let from = crate::meshes::posable(&clip.path, &clip.ext, &clip.root)?;
    let (index, secs) = take(&from.json).context("it has no clip to play")?;
    let worn = match body {
        Some(body) => {
            let mut doc = crate::meshes::posable(&body.path, &body.ext, &body.root)?;
            transplant(&from, index, &mut doc)?.map(|i| (doc, i))
        }
        None => None,
    };
    let (mut doc, index, facing, on_body) = match worn {
        Some((doc, i)) => (doc, i, facings.1, true),
        None if from.counts().triangles > 0 => (from, index, facings.0, false),
        None if body.is_some() => return Err(anyhow!("it moves none of the character's bones")),
        None => return Err(NoCharacter.into()),
    };
    crate::meshes::turn(&mut doc.json, facing);
    let fps = f64::from(motion.unwrap_or(crate::settings::DEFAULT_MOTION_FPS));
    let (frames, ms) = if motion.is_some() {
        frames(&doc, index, secs, fps)?
    } else {
        (vec![posed(&doc, index, middle(secs, fps))?], 0)
    };
    let guard = crate::analyze::renderer().context("model thumbnails are off")?;
    // wgpu panics on what it finds invalid; that costs this clip its thumbnail, not the server.
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| guard.render_frames(&frames, SIZE)))
        .map_err(|_| anyhow!("the renderer crashed"))??;
    drop(guard);
    let still = drawn.get(drawn.len() / 2).context("no frames")?.clone();
    // The motion thumbnail plays at its own size; the still is a thumbnail like any other, at the size set.
    let px = crate::analyze::thumb_size();
    let still = if px < SIZE {
        image::DynamicImage::ImageRgba8(still).resize(px, px, image::imageops::FilterType::Lanczos3).to_rgba8()
    } else {
        still
    };
    let webp = if motion.is_some() { Some(encode(&drawn, ms)?) } else { None };
    Ok((webp, crate::analyze::thumb_bytes(&still, false)?, on_body))
}

/// The clip a motion thumbnail plays, as its index and length: a document's first that isn't a pose.
fn take(json: &Map<String, Value>) -> Option<(usize, f64)> {
    let rig = crate::animations::rig(&Value::Object(json.clone()))?;
    rig.clips.iter().enumerate().find(|(_, c)| c.secs >= crate::animations::POSE_SECS).map(|(i, c)| (i, c.secs))
}

/// The model posed at each frame of clip `index` at `fps`, each as a GLB, and how long a frame shows in milliseconds.
fn frames(doc: &Document<'_>, index: usize, secs: f64, fps: f64) -> Result<(Vec<Vec<u8>>, u32)> {
    let span = secs.min(MAX_SECS);
    let (count, ms) = timing(span, fps);
    let step = span / f64::from(count);
    let out = (0..count).map(|k| posed(doc, index, f64::from(k) * step)).collect::<Result<_>>()?;
    Ok((out, ms))
}

/// The moment of a clip `secs` long its still shows: the middle of the stretch a motion thumbnail plays, which is the
/// motion thumbnail's middle frame.
fn middle(secs: f64, fps: f64) -> f64 {
    let span = secs.min(MAX_SECS);
    let (count, _) = timing(span, fps);
    f64::from(count / 2) * span / f64::from(count)
}

/// The model posed `time` seconds into clip `index`, as a GLB.
fn posed(doc: &Document<'_>, index: usize, time: f64) -> Result<Vec<u8>> {
    #[expect(clippy::cast_possible_truncation, reason = "a time in a clip of seconds")]
    let time = time as f32;
    let mesh = doc.flatten(Pose::Animation { index, time })?.context("the posed model draws nothing")?;
    Ok(crate::meshes::to_glb(&mesh)?)
}

/// How many frames a stretch of `span` seconds has at `fps` (two at least), and how long each shows.
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a few dozen frames of a few seconds")]
fn timing(span: f64, fps: f64) -> (u32, u32) {
    let count = ((span * fps).round() as u32).max(2);
    (count, ((span / f64::from(count)) * 1000.0).round().max(1.0) as u32)
}

/// The frames as a looping, lossy animated WebP.
fn encode(frames: &[image::RgbaImage], ms: u32) -> Result<Vec<u8>> {
    let first = frames.first().context("no frames")?;
    let (w, h) = first.dimensions();
    let mut config = webp::WebPConfig::new().map_err(|()| anyhow!("libwebp has no settings"))?;
    config.quality = QUALITY;
    config.method = 4;
    let mut enc = webp::AnimEncoder::new(w, h, &config);
    enc.set_loop_count(0);
    enc.set_bgcolor([0, 0, 0, 0]);
    let mut at = 0i32;
    for f in frames {
        enc.add_frame(webp::AnimFrame::from_rgba(f.as_raw(), w, h, at));
        at = at.saturating_add(i32::try_from(ms).unwrap_or(i32::MAX));
    }
    let out = enc.try_encode().map_err(|e| anyhow!("encoding the animated WebP: {e:?}"))?;
    Ok(out.to_vec())
}

/// Copy clip `index` of `from` (a file of clips with no mesh) into `to` (a character), each channel moving the node
/// of `to` named as the one it moved in `from` (or, failing that, the same bone named another way). Channels moving
/// nothing `to` has, and morph weights, are left out. Its index in `to`, or `None` when it would move nothing.
///
/// Its moves are put in `to`'s units and to its size, as the viewer's are (`bindClip` in modelScene.ts): a step of 10
/// from an FBX in centimeters, under a node scaled by 0.01, is 10 cm on its own character but would be 10 meters on
/// one made in meters, and a step that's a fifth of one character's height stays a fifth of a smaller one's.
fn transplant(from: &Document<'_>, index: usize, to: &mut Document<'_>) -> Result<Option<usize>> {
    let Some(anim) = list(from.json.get("animations")).get(index) else { return Ok(None) };
    let (mut exact, mut loose) = (HashMap::new(), HashMap::new());
    for (i, n) in list(to.json.get("nodes")).iter().enumerate() {
        let Some(name) = n.get("name").and_then(Value::as_str) else { continue };
        exact.entry(name.to_owned()).or_insert(i);
        if let Some(b) = crate::animations::bone(name) {
            loose.entry(b).or_insert(i);
        }
    }
    let nodes = list(from.json.get("nodes"));
    let counterpart = |i: usize| -> Option<usize> {
        let name = nodes.get(i)?.get("name")?.as_str()?;
        exact.get(name).or_else(|| loose.get(&crate::animations::bone(name)?)).copied()
    };
    // The bones both skeletons have, each measured in its own file, for how much bigger `to`'s is.
    let (mut pairs, mut taken) = (Vec::new(), HashSet::new());
    for i in 0..nodes.len() {
        if let Some(t) = counterpart(i).filter(|t| taken.insert(*t)) {
            pairs.push((i, t));
        }
    }
    let (from_world, to_world) = (Skeleton::of(&from.json), Skeleton::of(&to.json));
    let (from_length, to_length) = (
        from_world.length(&pairs.iter().map(|p| p.0).collect()),
        to_world.length(&pairs.iter().map(|p| p.1).collect()),
    );
    let ratio = (from_length > 1e-9 && to_length > 1e-9).then(|| to_length / from_length);
    let samplers = list(anim.get("samplers"));
    let mut copy = Copier::new(from, to);
    let (mut new_samplers, mut by_sampler, mut channels) = (Vec::new(), HashMap::new(), Vec::new());
    for c in list(anim.get("channels")) {
        let path = c.pointer("/target/path").and_then(Value::as_str);
        if path == Some("weights") {
            continue;
        }
        let Some(source) = c.pointer("/target/node").and_then(Value::as_u64).and_then(|n| usize::try_from(n).ok())
        else {
            continue;
        };
        let Some(node) = counterpart(source) else { continue };
        let Some(s) = c.get("sampler").and_then(Value::as_u64).and_then(|s| usize::try_from(s).ok()) else { continue };
        let Some(sampler) = samplers.get(s) else { continue };
        // How much a move grows going across: the size ratio, from the clip's units to `to`'s.
        let grow = match ratio {
            Some(r) if path == Some("translation") => {
                let k = r * from_world.unit(source) / to_world.unit(node);
                if k.is_finite() && (k - 1.0).abs() >= 1e-3 { k } else { 1.0 }
            }
            _ => 1.0,
        };
        let at = if let Some(&at) = by_sampler.get(&(s, grow.to_bits())) {
            at
        } else {
            let mut ns = sampler.clone();
            let input = sampler.get("input").and_then(Value::as_u64).and_then(|a| usize::try_from(a).ok());
            ns["input"] = json!(copy.accessor(input.context("a sampler without an input")?)?);
            let output = sampler.get("output").and_then(Value::as_u64).and_then(|a| usize::try_from(a).ok());
            ns["output"] = json!(copy.scaled(output.context("a sampler without an output")?, grow)?);
            new_samplers.push(ns);
            by_sampler.insert((s, grow.to_bits()), new_samplers.len() - 1);
            new_samplers.len() - 1
        };
        let mut nc = c.clone();
        nc["sampler"] = json!(at);
        nc["target"]["node"] = json!(node);
        channels.push(nc);
    }
    if channels.is_empty() {
        return Ok(None);
    }
    let name = anim.get("name").cloned().unwrap_or(Value::Null);
    let Copier { accessors, views, buffers, .. } = copy;
    let base = list(to.json.get("buffers")).len();
    for (k, bytes) in buffers.into_iter().enumerate() {
        let at = to.push_buffer(bytes);
        anyhow::ensure!(at == base + k, "buffers were added out of order");
    }
    array(&mut to.json, "accessors").extend(accessors);
    array(&mut to.json, "bufferViews").extend(views);
    let animations = array(&mut to.json, "animations");
    animations.push(json!({ "name": name, "samplers": new_samplers, "channels": channels }));
    Ok(Some(animations.len() - 1))
}

/// Where a document's nodes stand at rest: each one's parent, and its place in the world (its own transform under its
/// parents').
struct Skeleton {
    parent: Vec<Option<usize>>,
    world: Vec<glam::Mat4>,
}

impl Skeleton {
    fn of(json: &Map<String, Value>) -> Self {
        let nodes = list(json.get("nodes"));
        let mut parent = vec![None; nodes.len()];
        for (i, n) in nodes.iter().enumerate() {
            for c in list(n.get("children")).iter().filter_map(Value::as_u64).filter_map(|c| usize::try_from(c).ok()) {
                if let Some(p) = parent.get_mut(c).filter(|p| p.is_none()) {
                    *p = Some(i);
                }
            }
        }
        let mut world = vec![None; nodes.len()];
        for i in 0..nodes.len() {
            place(i, nodes, &parent, &mut world, 0);
        }
        Self { parent, world: world.into_iter().map(|m| m.unwrap_or(glam::Mat4::IDENTITY)).collect() }
    }

    /// How long the bones between these nodes are, end to end: each node's distance from its parent, when that's one
    /// of them too.
    fn length(&self, nodes: &HashSet<usize>) -> f32 {
        let at = |i: usize| self.world[i].w_axis.truncate();
        nodes.iter().filter_map(|&i| self.parent[i].filter(|p| nodes.contains(p)).map(|p| at(i).distance(at(p)))).sum()
    }

    /// How big one unit of a node's translation is in the world: its parent's scale.
    fn unit(&self, i: usize) -> f32 {
        let Some(p) = self.parent[i] else { return 1.0 };
        let m = self.world[p];
        (m.x_axis.truncate().length() + m.y_axis.truncate().length() + m.z_axis.truncate().length()) / 3.0
    }
}

/// Node `i`'s place in the world, worked out once each: its parents' first. A loop of parents stops at its first turn.
fn place(
    i: usize,
    nodes: &[Value],
    parent: &[Option<usize>],
    world: &mut [Option<glam::Mat4>],
    depth: usize,
) -> glam::Mat4 {
    if let Some(m) = world[i] {
        return m;
    }
    let up = match parent[i] {
        Some(p) if depth < nodes.len() => place(p, nodes, parent, world, depth + 1),
        _ => glam::Mat4::IDENTITY,
    };
    let m = up * local(&nodes[i]);
    world[i] = Some(m);
    m
}

/// A node's own transform: its matrix, or its translation, rotation and scale.
fn local(node: &Value) -> glam::Mat4 {
    if let Some(m) = floats::<16>(node.get("matrix")) {
        return glam::Mat4::from_cols_array(&m);
    }
    let t = floats::<3>(node.get("translation")).map_or(glam::Vec3::ZERO, glam::Vec3::from_array);
    let r = floats::<4>(node.get("rotation")).map_or(glam::Quat::IDENTITY, |q| glam::Quat::from_array(q).normalize());
    let s = floats::<3>(node.get("scale")).map_or(glam::Vec3::ONE, glam::Vec3::from_array);
    glam::Mat4::from_scale_rotation_translation(s, r, t)
}

/// A JSON array of exactly `N` numbers.
fn floats<const N: usize>(v: Option<&Value>) -> Option<[f32; N]> {
    let a = v?.as_array()?;
    if a.len() != N {
        return None;
    }
    let mut out = [0f32; N];
    for (o, x) in out.iter_mut().zip(a) {
        #[expect(clippy::cast_possible_truncation, reason = "glTF stores these as 32-bit floats")]
        let x = x.as_f64()? as f32;
        *o = x;
    }
    Some(out)
}

fn list(v: Option<&Value>) -> &[Value] {
    v.and_then(Value::as_array).map_or(&[], Vec::as_slice)
}

fn array<'v>(json: &'v mut Map<String, Value>, key: &str) -> &'v mut Vec<Value> {
    let v = json.entry(key).or_insert_with(|| json!([]));
    if !v.is_array() {
        *v = json!([]);
    }
    v.as_array_mut().expect("an array, as just made sure")
}

/// The accessors, buffer views and buffers of one document copied to the end of another's, each once.
struct Copier<'d> {
    from: &'d Document<'d>,
    /// Where `to`'s lists end, before anything is copied.
    base: (usize, usize, usize),
    accessor_at: HashMap<usize, usize>,
    view_at: HashMap<usize, usize>,
    buffer_at: HashMap<usize, usize>,
    accessors: Vec<Value>,
    views: Vec<Value>,
    buffers: Vec<Vec<u8>>,
}

impl<'d> Copier<'d> {
    fn new(from: &'d Document<'d>, to: &Document<'_>) -> Self {
        let n = |k| list(to.json.get(k)).len();
        Self {
            from,
            base: (n("accessors"), n("bufferViews"), n("buffers")),
            accessor_at: HashMap::new(),
            view_at: HashMap::new(),
            buffer_at: HashMap::new(),
            accessors: Vec::new(),
            views: Vec::new(),
            buffers: Vec::new(),
        }
    }

    fn accessor(&mut self, i: usize) -> Result<usize> {
        if let Some(&at) = self.accessor_at.get(&i) {
            return Ok(at);
        }
        let mut a = list(self.from.json.get("accessors")).get(i).with_context(|| format!("no accessor {i}"))?.clone();
        for p in ["/bufferView", "/sparse/indices/bufferView", "/sparse/values/bufferView"] {
            let view = a.pointer(p).and_then(Value::as_u64).and_then(|v| usize::try_from(v).ok());
            if let Some(v) = view {
                let at = self.view(v)?;
                *a.pointer_mut(p).expect("the pointer just read") = json!(at);
            }
        }
        self.accessors.push(a);
        let at = self.base.0 + self.accessors.len() - 1;
        self.accessor_at.insert(i, at);
        Ok(at)
    }

    /// Accessor `i` copied with every value times `k`, as a new accessor of its own: or as it is when `k` is 1, or it
    /// isn't plain floats (a sparse one, say) to scale.
    fn scaled(&mut self, i: usize, k: f32) -> Result<usize> {
        #[expect(clippy::float_cmp, reason = "exactly 1 is what's left alone")]
        if k == 1.0 {
            return self.accessor(i);
        }
        let a = list(self.from.json.get("accessors")).get(i).with_context(|| format!("no accessor {i}"))?.clone();
        let index = |v: Option<&Value>| v.and_then(Value::as_u64).and_then(|v| usize::try_from(v).ok());
        let width = match a.get("type").and_then(Value::as_str) {
            Some("SCALAR") => 1,
            Some("VEC2") => 2,
            Some("VEC3") => 3,
            Some("VEC4") => 4,
            _ => 0,
        };
        let (Some(v), Some(count)) = (index(a.get("bufferView")), index(a.get("count"))) else {
            return self.accessor(i);
        };
        if width == 0 || a.get("sparse").is_some() || index(a.get("componentType")) != Some(5126) {
            return self.accessor(i);
        }
        let view = list(self.from.json.get("bufferViews")).get(v).with_context(|| format!("no buffer view {v}"))?;
        let buffer = index(view.get("buffer")).with_context(|| format!("buffer view {v} names no buffer"))?;
        let start = index(view.get("byteOffset")).unwrap_or(0) + index(a.get("byteOffset")).unwrap_or(0);
        let stride = index(view.get("byteStride")).unwrap_or(width * 4);
        let bytes = self.from.buffer(buffer)?;
        let mut out = Vec::with_capacity(count * width * 4);
        for element in 0..count {
            for component in 0..width {
                let at = start + element * stride + component * 4;
                let four = bytes.get(at..at + 4).with_context(|| format!("accessor {i} runs past its buffer"))?;
                out.extend_from_slice(&(f32::from_le_bytes(four.try_into()?) * k).to_le_bytes());
            }
        }
        let len = out.len();
        self.buffers.push(out);
        self.views.push(json!({ "buffer": self.base.2 + self.buffers.len() - 1, "byteLength": len }));
        let mut copy = a;
        copy["bufferView"] = json!(self.base.1 + self.views.len() - 1);
        if let Some(o) = copy.as_object_mut() {
            o.remove("byteOffset");
            for bound in ["min", "max"] {
                if let Some(Value::Array(xs)) = o.get_mut(bound) {
                    // `k` is a ratio of lengths, so more than nothing: the least stays the least.
                    for x in xs.iter_mut() {
                        *x = json!(x.as_f64().unwrap_or(0.0) * f64::from(k));
                    }
                }
            }
        }
        self.accessors.push(copy);
        Ok(self.base.0 + self.accessors.len() - 1)
    }

    fn view(&mut self, i: usize) -> Result<usize> {
        if let Some(&at) = self.view_at.get(&i) {
            return Ok(at);
        }
        let mut v =
            list(self.from.json.get("bufferViews")).get(i).with_context(|| format!("no buffer view {i}"))?.clone();
        let b = v.get("buffer").and_then(Value::as_u64).and_then(|b| usize::try_from(b).ok());
        let b = b.with_context(|| format!("buffer view {i} names no buffer"))?;
        let at = if let Some(&at) = self.buffer_at.get(&b) {
            at
        } else {
            self.buffers.push(self.from.buffer(b)?.into_owned());
            let at = self.base.2 + self.buffers.len() - 1;
            self.buffer_at.insert(b, at);
            at
        };
        v["buffer"] = json!(at);
        self.views.push(v);
        let at = self.base.1 + self.views.len() - 1;
        self.view_at.insert(i, at);
        Ok(at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polysmelt::assets::Memory;
    use polysmelt::document::Keep;

    fn doc(gltf: &str) -> Document<'static> {
        let keep = Keep { animations: true, ..Keep::MESHES };
        polysmelt::formats::load_bytes(gltf.as_bytes(), "m.gltf", "gltf", &Memory::new(), keep).unwrap().into_owned()
    }

    /// A character: one triangle on a node named `Hips`, under a root.
    fn character() -> Document<'static> {
        let data = "AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAA";
        doc(&format!(
            r#"{{"asset":{{"version":"2.0"}},"scene":0,"scenes":[{{"nodes":[0]}}],
            "nodes":[{{"name":"Root","children":[1]}},{{"name":"Hips","mesh":0}}],
            "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}}}}]}}],
            "accessors":[{{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3",
                "min":[0,0,0],"max":[1,1,0]}}],
            "bufferViews":[{{"buffer":0,"byteLength":36}}],
            "buffers":[{{"byteLength":36,"uri":"data:application/octet-stream;base64,{data}"}}]}}"#
        ))
    }

    /// A clip with no mesh moving `bone` 5 along x over a second, and a pose.
    fn clip(bone: &str) -> Document<'static> {
        let data = "AAAAAAAAgD8AAAAAAAAAAAAAAAAAAKBAAAAAAAAAAAA=";
        doc(&format!(
            r#"{{"asset":{{"version":"2.0"}},"scene":0,"scenes":[{{"nodes":[0]}}],
            "nodes":[{{"name":"Armature","children":[1]}},{{"name":"{bone}"}}],
            "animations":[
                {{"name":"Pose","samplers":[{{"input":0,"output":2}}],
                  "channels":[{{"sampler":0,"target":{{"node":1,"path":"translation"}}}}]}},
                {{"name":"Run","samplers":[{{"input":1,"output":3}}],
                  "channels":[{{"sampler":0,"target":{{"node":1,"path":"translation"}}}},
                              {{"sampler":0,"target":{{"node":0,"path":"weights"}}}}]}}],
            "accessors":[
                {{"bufferView":0,"componentType":5126,"count":1,"type":"SCALAR","min":[0],"max":[0]}},
                {{"bufferView":0,"componentType":5126,"count":2,"type":"SCALAR","min":[0],"max":[1]}},
                {{"bufferView":1,"componentType":5126,"count":1,"type":"VEC3"}},
                {{"bufferView":1,"componentType":5126,"count":2,"type":"VEC3"}}],
            "bufferViews":[{{"buffer":0,"byteLength":8}},{{"buffer":0,"byteOffset":8,"byteLength":24}}],
            "buffers":[{{"byteLength":32,"uri":"data:application/octet-stream;base64,{data}"}}]}}"#
        ))
    }

    fn x_at(doc: &Document<'_>, index: usize, time: f32) -> f32 {
        let mesh = doc.flatten(Pose::Animation { index, time }).unwrap().unwrap();
        mesh.prims[0].positions.iter().map(|p| p[0]).fold(f32::INFINITY, f32::min)
    }

    #[test]
    fn a_clip_without_a_mesh_plays_on_the_character_by_bone_name() {
        let from = clip("mixamorig:Hips");
        let (index, secs) = take(&from.json).unwrap();
        assert_eq!((index, secs), (1, 1.0), "the pose is passed over");
        let mut body = character();
        let at = transplant(&from, index, &mut body).unwrap().unwrap();
        assert_eq!(at, 0);
        assert_eq!(body.json["animations"][0]["channels"].as_array().unwrap().len(), 1, "morph weights are left out");
        assert!(x_at(&body, at, 0.0).abs() < 1e-5);
        assert!((x_at(&body, at, 0.5) - 2.5).abs() < 1e-4);
        let (frames, ms) = frames(&body, at, secs, 12.0).unwrap();
        assert_eq!((frames.len(), ms), (12, 83));
    }

    /// A clip from an FBX in centimeters, its bones under a node scaled by 0.01, moving its hips 5 meters (500) along
    /// x over a second, played on a character made in meters with a spine as far above its hips: it moves 5 meters
    /// there too, not 500.
    #[test]
    fn a_clip_in_other_units_moves_the_character_as_far_in_its_own() {
        let data = "AAAAAAAAgD8AAAAAAADIQgAAAAAAAPpDAADIQgAAAAA=";
        let from = doc(&format!(
            r#"{{"asset":{{"version":"2.0"}},"scene":0,"scenes":[{{"nodes":[0]}}],
            "nodes":[{{"name":"RootNode","scale":[0.01,0.01,0.01],"children":[1]}},
                     {{"name":"Hips","translation":[0,100,0],"children":[2]}},
                     {{"name":"Spine","translation":[0,50,0]}}],
            "animations":[{{"name":"Run","samplers":[{{"input":0,"output":1}}],
                "channels":[{{"sampler":0,"target":{{"node":1,"path":"translation"}}}}]}}],
            "accessors":[
                {{"bufferView":0,"componentType":5126,"count":2,"type":"SCALAR","min":[0],"max":[1]}},
                {{"bufferView":1,"componentType":5126,"count":2,"type":"VEC3","min":[0,100,0],"max":[500,100,0]}}],
            "bufferViews":[{{"buffer":0,"byteLength":8}},{{"buffer":0,"byteOffset":8,"byteLength":24}}],
            "buffers":[{{"byteLength":32,"uri":"data:application/octet-stream;base64,{data}"}}]}}"#
        ));
        let tri = "AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAA";
        let mut body = doc(&format!(
            r#"{{"asset":{{"version":"2.0"}},"scene":0,"scenes":[{{"nodes":[0]}}],
            "nodes":[{{"name":"Root","children":[1]}},{{"name":"Hips","translation":[0,1,0],"mesh":0,"children":[2]}},
                     {{"name":"Spine","translation":[0,0.5,0]}}],
            "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}}}}]}}],
            "accessors":[{{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3",
                "min":[0,0,0],"max":[1,1,0]}}],
            "bufferViews":[{{"buffer":0,"byteLength":36}}],
            "buffers":[{{"byteLength":36,"uri":"data:application/octet-stream;base64,{tri}"}}]}}"#
        ));
        let at = transplant(&from, 0, &mut body).unwrap().unwrap();
        assert!(x_at(&body, at, 0.0).abs() < 1e-4);
        let x = x_at(&body, at, 0.5);
        assert!((x - 2.5).abs() < 1e-3, "half way it's {x} along, not 2.5");
        let max = &body.json["accessors"]
            [body.json["animations"][at]["samplers"][0]["output"].as_u64().unwrap() as usize]["max"];
        let max: Vec<f64> = max.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
        assert!(
            max.iter().zip([5.0, 1.0, 0.0]).all(|(a, b)| (a - b).abs() < 1e-4),
            "its bounds are scaled with it, not {max:?}"
        );
    }

    #[test]
    fn a_clip_of_other_bones_moves_nothing() {
        let mut body = character();
        assert!(transplant(&clip("Spine"), 1, &mut body).unwrap().is_none());
        assert!(body.json.get("animations").is_none());
    }

    #[test]
    fn a_long_clip_is_cut_and_a_short_one_has_two_frames() {
        assert_eq!(timing(MAX_SECS, 12.0), (36, 83));
        assert_eq!(timing(0.1, 12.0), (2, 50));
        // Smoother is more frames, each shown for less time.
        assert_eq!(timing(MAX_SECS, 30.0), (90, 33));
        // A still shows the middle of what a motion thumbnail plays.
        assert_eq!((middle(1.0, 12.0), middle(10.0, 12.0), middle(0.1, 12.0)), (0.5, 1.5, 0.05));
    }

    #[test]
    fn frames_encode_as_a_looping_animated_webp() {
        let frames: Vec<image::RgbaImage> =
            (0..3u8).map(|k| image::RgbaImage::from_pixel(16, 16, image::Rgba([k * 80, 40, 200, 255]))).collect();
        let webp = encode(&frames, 83).unwrap();
        assert_eq!((&webp[..4], &webp[8..12]), (&b"RIFF"[..], &b"WEBP"[..]));
        assert!(webp.windows(4).any(|w| w == b"ANIM") && webp.windows(4).filter(|w| *w == b"ANMF").count() >= 2);
    }
}
