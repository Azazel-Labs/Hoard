//! Motion thumbnails: an animation's tile plays it when pointed at, a few seconds of the character moving as an
//! animated WebP (its `motion` derivative). Off until turned on in Settings → Thumbnails, as each takes room and a
//! few seconds of rendering to make.
//!
//! A clip shipped without a mesh (`run.fbx`) is played on the first character it fits: its keys are copied onto that
//! character's bones, matched by name. The character is posed at each frame by polysmelt and drawn by the thumbnail
//! renderer from one camera for the whole clip, so the character moves in the picture and the picture holds still.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex, PoisonError};

use anyhow::{Context, Result, anyhow};
use polysmelt::document::{Document, Pose};
use rusqlite::{OptionalExtension, params};
use serde_json::{Map, Value, json};

use crate::{AppState, db};

/// Frames a second.
const FPS: f64 = 12.0;
/// The longest stretch of a clip played: a longer one is cut there.
const MAX_SECS: f64 = 3.0;
/// The frame's side, as a still thumbnail's.
const SIZE: u32 = 256;
/// libwebp's lossy quality, 0 to 100.
const QUALITY: f32 = 70.0;

/// Whether a fill is running, and whether another was asked for while it ran.
static BUSY: AtomicBool = AtomicBool::new(false);
static AGAIN: AtomicBool = AtomicBool::new(false);
/// Animations a motion thumbnail couldn't be made of since the server started, which aren't tried again until then.
static FAILED: LazyLock<Mutex<std::collections::HashSet<String>>> = LazyLock::new(Mutex::default);

/// Make the motion thumbnails the library lacks, in the background, when they're turned on. Asked for again while
/// it runs, it goes round once more when done, for what was added meanwhile.
pub(crate) fn kick(state: &AppState) {
    let settings = state.settings.get();
    if !settings.motion_thumbs || !settings.render {
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

/// An animation to make a motion thumbnail of.
struct Todo {
    id: String,
    path: String,
    pack_id: String,
}

async fn fill(state: &AppState) -> Result<()> {
    let todo: Vec<Todo> = db::call(&state.pool, |c| {
        let rows = c
            .prepare(
                "SELECT a.id, a.path, a.pack_id FROM assets a WHERE a.kind = 'animation'
                 AND NOT EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'motion')
                 ORDER BY a.pack_id, a.path",
            )?
            .query_map([], |r| Ok(Todo { id: r.get(0)?, path: r.get(1)?, pack_id: r.get(2)? }))?
            .collect::<rusqlite::Result<_>>()?;
        Ok(rows)
    })
    .await?;
    let todo: Vec<Todo> = {
        let failed = FAILED.lock().unwrap_or_else(PoisonError::into_inner);
        todo.into_iter().filter(|t| !failed.contains(&t.id)).collect()
    };
    if todo.is_empty() {
        return Ok(());
    }
    let activity =
        crate::activity::start("motion", format!("Making motion thumbnails of {} animations", todo.len()), None);
    let mut made = 0;
    for (i, t) in todo.iter().enumerate() {
        // Turned off meanwhile: stop.
        if !state.settings.get().motion_thumbs {
            break;
        }
        #[expect(clippy::cast_precision_loss, reason = "a fraction for a progress bar")]
        activity.progress(i as f32 / todo.len() as f32);
        match make(state, t).await {
            Ok(()) => made += 1,
            Err(e) => {
                tracing::info!(path = %t.path, pack = %t.pack_id, error = %format!("{e:#}"), "no motion thumbnail");
                FAILED.lock().unwrap_or_else(PoisonError::into_inner).insert(t.id.clone());
            }
        }
    }
    tracing::info!(made, of = todo.len(), "made motion thumbnails");
    Ok(())
}

/// A model's files laid out in a folder of its own, and the one to read.
struct Local {
    path: PathBuf,
    ext: String,
    root: PathBuf,
}

/// Lay out an asset's file, and those it needs (a model's textures), under `work`. A model only Blender reads is laid
/// out as the GLB Blender made of it.
async fn fetch(state: &AppState, id: &str, work: &Path) -> Result<Local> {
    let id2 = id.to_owned();
    let (path, ext, files) = db::call(&state.pool, move |c| {
        let (path, ext): (String, String) =
            c.query_row("SELECT path, ext FROM assets WHERE id = ?1", [&id2], |r| Ok((r.get(0)?, r.get(1)?)))?;
        let ext = ext.to_ascii_lowercase();
        if polysmelt::formats::EXTENSIONS.contains(&ext.as_str()) {
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

/// Make one animation's motion thumbnail, and its still from the middle of the clip when it has none.
async fn make(state: &AppState, t: &Todo) -> Result<()> {
    let (id, pack) = (t.id.clone(), t.pack_id.clone());
    let (body, facing) = db::call(&state.pool, move |c| {
        let body: Option<String> = c
            .query_row(
                "SELECT l.from_id FROM asset_links l JOIN assets m ON m.id = l.from_id
                 WHERE l.to_id = ?1 AND l.rel = 'animation' ORDER BY m.path LIMIT 1",
                [&id],
                |r| r.get(0),
            )
            .optional()?;
        Ok((body, db::pack_facing(c, &pack)?))
    })
    .await?;
    let work = state.cfg.tmp_dir().join(format!("motion-{}", uuid::Uuid::now_v7()));
    let result = async {
        let clip = fetch(state, &t.id, &work.join("clip")).await?;
        let body = match &body {
            Some(b) => Some(fetch(state, b, &work.join("body")).await?),
            None => None,
        };
        let (webp, still) = crate::heavy::run("motion", move || draw(&clip, body.as_ref(), facing)).await??;
        let motion = state.blobs.put_cache_bytes(webp).await?;
        let still = state.blobs.put_cache_bytes(still).await?;
        let id = t.id.clone();
        db::call(&state.pool, move |c| {
            let now = db::now();
            let tx = c.transaction()?;
            for b in [&motion, &still] {
                tx.execute(
                    "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
                    params![b.hash, i64::try_from(b.size).unwrap_or(i64::MAX), now],
                )?;
            }
            tx.execute(
                "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, 'motion', ?2, ?3)
                 ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash, created_at = excluded.created_at",
                params![id, motion.hash, now],
            )?;
            tx.execute("UPDATE assets SET thumb_hash = ?2 WHERE id = ?1 AND thumb_hash IS NULL", params![id, still.hash])?;
            tx.commit()?;
            Ok(())
        })
        .await
    }
    .await;
    tokio::fs::remove_dir_all(&work).await.ok();
    result
}

/// The clip's frames, drawn and encoded: the animated WebP, and its middle frame as a still thumbnail.
fn draw(clip: &Local, body: Option<&Local>, facing: hoard_core::Facing) -> Result<(Vec<u8>, Vec<u8>)> {
    let from = crate::meshes::posable(&clip.path, &clip.ext, &clip.root)?;
    let (index, secs) = take(&from.json).context("it has no clip to play")?;
    let (mut doc, index) = if from.counts().triangles > 0 {
        (from, index)
    } else {
        let body = body.context("it fits no character to play on")?;
        let mut doc = crate::meshes::posable(&body.path, &body.ext, &body.root)?;
        let index = transplant(&from, index, &mut doc)?.context("it moves none of the character's bones")?;
        (doc, index)
    };
    crate::meshes::turn(&mut doc.json, facing);
    let (frames, ms) = frames(&doc, index, secs)?;
    let renderer = crate::analyze::renderer().context("model thumbnails are off")?;
    let guard = renderer.lock().unwrap_or_else(PoisonError::into_inner);
    // wgpu panics on what it finds invalid; that costs this clip its thumbnail, not the server.
    let drawn = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| guard.render_frames(&frames, SIZE)))
        .map_err(|_| anyhow!("the renderer crashed"))??;
    drop(guard);
    let still = drawn.get(drawn.len() / 2).context("no frames")?.clone();
    Ok((encode(&drawn, ms)?, crate::analyze::thumb_bytes(still, false)?))
}

/// The clip a motion thumbnail plays, as its index and length: a document's first that isn't a pose.
fn take(json: &Map<String, Value>) -> Option<(usize, f64)> {
    let rig = crate::animations::rig(&Value::Object(json.clone()))?;
    rig.clips.iter().enumerate().find(|(_, c)| c.secs >= crate::animations::POSE_SECS).map(|(i, c)| (i, c.secs))
}

/// The model posed at each frame of clip `index`, each as a GLB, and how long a frame shows in milliseconds.
fn frames(doc: &Document<'_>, index: usize, secs: f64) -> Result<(Vec<Vec<u8>>, u32)> {
    let span = secs.min(MAX_SECS);
    let (count, ms) = timing(span);
    let step = span / f64::from(count);
    let mut out = Vec::new();
    for k in 0..count {
        #[expect(clippy::cast_possible_truncation, reason = "a time in a clip of seconds")]
        let time = (f64::from(k) * step) as f32;
        let mesh = doc.flatten(Pose::Animation { index, time })?.context("the posed model draws nothing")?;
        out.push(crate::meshes::to_glb(&mesh)?);
    }
    Ok((out, ms))
}

/// How many frames a stretch of `span` seconds has at [`FPS`] (two at least), and how long each shows.
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a few dozen frames of a few seconds")]
fn timing(span: f64) -> (u32, u32) {
    let count = ((span * FPS).round() as u32).max(2);
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
        let (frames, ms) = frames(&body, at, secs).unwrap();
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
        assert_eq!(timing(MAX_SECS), (36, 83));
        assert_eq!(timing(0.1), (2, 50));
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
