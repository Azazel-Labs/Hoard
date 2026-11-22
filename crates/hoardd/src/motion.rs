//! Animations drawn moving. Every animation's thumbnail is a still from the middle of its clip (its `still`
//! derivative marks it drawn), so a grid of them isn't a grid of the same rest pose. Motion thumbnails go further: an
//! animation's tile plays it when pointed at, a few seconds of the character moving as an animated WebP (its `motion`
//! derivative), off until turned on in Settings → Thumbnails, as each takes room and a few seconds of rendering.
//!
//! Every clip is played on its skeleton's character (see `animations::plays_on`), the one the viewer plays it on too,
//! so a grid of a skeleton's clips is one character moving many ways: its keys are copied onto that character's
//! bones, matched by name. A clip that moves none of them is played on its own mesh, when it has one.
//! The character is posed at each frame by polysmelt and drawn by the thumbnail renderer in a tiled room scaled to it.
//! A clip that stays put is drawn from one camera, so the character moves and the room holds still. One that travels
//! (a fall, a run that goes somewhere) is followed, so the character stays big and the tiles stream past.

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

/// Whether a fill is running, and whether another was asked for while it ran.
static BUSY: AtomicBool = AtomicBool::new(false);
static AGAIN: AtomicBool = AtomicBool::new(false);
/// Animations that couldn't be drawn since the server started, which aren't tried again until then.
static FAILED: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);
/// Clips with no mesh that fit no character yet, tried again when a pack comes in (it may bring one).
static WAITING: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);
/// How many packs have come in, so a clip found to fit nothing while one came in is tried again rather than put aside.
static PACKS_IN: AtomicU64 = AtomicU64::new(0);

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
    kick(state);
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
    let motion_on = state.settings.get().motion_thumbs;
    // A thumbnail a browser drew is kept: it drew the clip mid-way too, with every texture.
    let todo: Vec<Todo> = db::call(&state.pool, move |c| {
        let rows = c
            .prepare(
                "SELECT id, path, pack_id, motion, still FROM (
                   SELECT a.id, a.path, a.pack_id,
                          ?1 AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'motion')
                            AS motion,
                          a.thumb_source IS NULL
                            AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'still')
                            AS still
                     FROM assets a WHERE a.kind = 'animation' AND a.archived_at IS NULL)
                  WHERE motion OR still ORDER BY pack_id, path",
            )?
            .query_map([motion_on], |r| {
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
            for (kind, b) in kinds {
                tx.execute(
                    "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash, created_at = excluded.created_at",
                    params![id, kind, b.hash, now],
                )?;
            }
            // The rest pose it was drawn in goes, unless something still shows it.
            let old: Option<(String, i64)> = tx
                .query_row(
                    "SELECT b.hash, b.size FROM assets a JOIN blobs b ON b.hash = a.thumb_hash
                     WHERE a.id = ?1 AND a.thumb_source IS NULL AND a.thumb_hash <> ?2",
                    params![id, still.hash],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()?;
            tx.execute("UPDATE assets SET thumb_hash = ?2 WHERE id = ?1 AND thumb_source IS NULL", params![id, still.hash])?;
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
        if let Some(old) = replaced {
            crate::jobs::delete_unused(state, vec![old]).await?;
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
    let renderer = crate::analyze::renderer().context("model thumbnails are off")?;
    let guard = renderer.lock().unwrap_or_else(PoisonError::into_inner);
    // wgpu panics on what it finds invalid; that costs this clip its thumbnail, not the server.
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| guard.render_frames(&frames, SIZE)))
        .map_err(|_| anyhow!("the renderer crashed"))??;
    drop(guard);
    let still = drawn.get(drawn.len() / 2).context("no frames")?.clone();
    let webp = if motion.is_some() { Some(encode(&drawn, ms)?) } else { None };
    Ok((webp, crate::analyze::thumb_bytes(still, false)?, on_body))
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
    let target = |c: &Value| -> Option<usize> {
        let node = nodes.get(usize::try_from(c.get("target")?.get("node")?.as_u64()?).ok()?)?;
        let name = node.get("name")?.as_str()?;
        exact.get(name).or_else(|| loose.get(&crate::animations::bone(name)?)).copied()
    };
    let samplers = list(anim.get("samplers"));
    let mut copy = Copier::new(from, to);
    let (mut new_samplers, mut by_sampler, mut channels) = (Vec::new(), HashMap::new(), Vec::new());
    for c in list(anim.get("channels")) {
        if c.pointer("/target/path").and_then(Value::as_str) == Some("weights") {
            continue;
        }
        let Some(node) = target(c) else { continue };
        let Some(s) = c.get("sampler").and_then(Value::as_u64).and_then(|s| usize::try_from(s).ok()) else { continue };
        let Some(sampler) = samplers.get(s) else { continue };
        let at = if let Some(&at) = by_sampler.get(&s) {
            at
        } else {
            let mut ns = sampler.clone();
            for k in ["input", "output"] {
                let a = sampler.get(k).and_then(Value::as_u64).and_then(|a| usize::try_from(a).ok());
                ns[k] = json!(copy.accessor(a.with_context(|| format!("a sampler without an {k}"))?)?);
            }
            new_samplers.push(ns);
            by_sampler.insert(s, new_samplers.len() - 1);
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
