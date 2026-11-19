//! Conversions on request: a download that isn't cached yet starts a job (a few at a time, the viewer's own ahead of
//! the rest), and whoever asks waits for it, or for word of how it's going.

use anyhow::{Context, Result};
use serde_json::Value;

use super::cache::{log_trim, touch_conversion};
use super::{IMAGE_IN, Progress, convert, options, resizable_as_itself};
use crate::i18n::tr;

/// Conversions running at once; more wait their turn (shown as queued).
static SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(2);
/// The 3D viewer's GLBs have slots of their own: someone looking at a model doesn't wait behind downloads.
static VIEW_SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(4);

/// Sizes a client may ask for: powers of two from 64 to 16384.
pub(crate) fn valid_max(max: u32) -> bool {
    max.is_power_of_two() && (64..=16_384).contains(&max)
}

/// Why a conversion was refused, for the HTTP layer.
#[derive(Debug)]
pub(crate) enum Refused {
    NotFound,
    Unsupported(String),
    Failed(String),
}

/// A converted download, ready to serve.
#[derive(Clone, Debug)]
pub(crate) struct Ready {
    pub(crate) hash: String,
    pub(crate) ext: String,
    /// The original file name without extension, for the download name.
    pub(crate) stem: String,
}

/// Where a conversion stands.
#[derive(Clone, Debug)]
pub(crate) enum Stage {
    Queued,
    /// 0..=1, when the conversion can tell.
    Working(f32),
    Done(Ready),
    Failed(String),
}

/// Conversions in progress, by asset and cache key. Several requests for the same one share it, and it carries on
/// if they all go away.
static RUNNING: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, tokio::sync::watch::Receiver<Stage>>>,
> = std::sync::LazyLock::new(Default::default);

/// A failure is remembered this long, so a client polling for it sees it rather than starting it again.
const FAILURE_KEPT: std::time::Duration = std::time::Duration::from_secs(60);

/// The answer to "give me this asset as `format`".
pub(crate) enum Lookup {
    Ready(Ready),
    Pending(tokio::sync::watch::Receiver<Stage>),
}

/// The cached conversion of an asset, or the conversion that will make it (started now if it isn't running). Never
/// waits for the conversion itself.
pub(crate) async fn request(
    state: &crate::AppState,
    asset_id: &str,
    format: &str,
    max: Option<u32>,
) -> std::result::Result<Lookup, Refused> {
    use crate::db;
    let fail = |e: anyhow::Error| Refused::Failed(format!("{e:#}"));
    let id = asset_id.to_owned();
    let (settings, glb) = (state.settings.get(), format == "glb");
    let info = db::call(&state.pool, move |c| {
        let row: Option<(String, String, String, String, String, String)> = rusqlite::OptionalExtension::optional(
            c.query_row("SELECT path, ext, blob_hash, meta, pack_id, kind FROM assets WHERE id = ?1", [&id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?))
            }),
        )?;
        let Some((path, ext, hash, meta, pack_id, kind)) = row else { return Ok(None) };
        let mut stmt = c.prepare("SELECT kind, blob_hash FROM derivatives WHERE asset_id = ?1")?;
        let derivs: Vec<(String, String)> =
            stmt.query_map([&id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
        // A GLB's textures are named by the convention, so it's made again when that changes.
        let naming = if glb {
            crate::naming::ForDownload::of_pack(c, &settings.texture_names, &pack_id)?
        } else {
            crate::naming::ForDownload::default()
        };
        Ok(Some((path, ext, hash, meta, derivs, db::asset_facing(c, &id, &pack_id)?, naming, kind == "animation")))
    })
    .await
    .map_err(fail)?
    .ok_or(Refused::NotFound)?;
    let (mut path, mut ext, mut hash, meta, derivs, facing, naming, animation) = info;
    let file = path.rsplit('/').next().unwrap_or(&path).to_owned();
    let stem = file.rsplit_once('.').map_or(file.as_str(), |(s, _)| s).to_owned();
    let names: Vec<String> = derivs.iter().map(|(k, _)| k.clone()).collect();
    // The 3D viewer's GLB, of any model polysmelt reads or Blender converted. Not offered for download.
    let readable = crate::meshes::reads(&ext.to_ascii_lowercase());
    let view = format == "view" && max.is_none() && (readable || names.iter().any(|n| n == "glb"));
    let allowed = view
        || options(&ext, &names).iter().any(|o| o.format == format)
        || (format == super::CLIP && super::offers_clip(animation, &ext, &names))
        || (max.is_some() && resizable_as_itself(&ext) == Some(format));
    if !allowed {
        return Err(Refused::Unsupported(tr!("error-cant-download-as", ext = ext, format = format)));
    }
    if max.is_some() && !(IMAGE_IN.contains(&ext.to_ascii_lowercase().as_str())) {
        return Err(Refused::Unsupported(tr!("error-only-images-resize")));
    }
    // A GLB download is turned the way the pack's models face and names its textures by the convention, and is made
    // again when either changes.
    let sheet = crate::spritesheet::is_format(format);
    let key = if format == "glb" {
        format!("as:glb~{}~{}~{}:0", facing.turn, facing.tilt, naming.mark())
    } else if sheet {
        // Made from the sheet, so made again when the sheet changes.
        let (_, h) = derivs.iter().find(|(k, _)| k == "sheet").ok_or(Refused::NotFound)?;
        format!("as:{format}~{}:0", &h[..h.len().min(16)])
    } else {
        format!("as:{format}:{}", max.unwrap_or(0))
    };
    // A sprite sheet is written from Hoard's own sheet of it, beside its file.
    if sheet {
        let (_, h) = derivs.iter().find(|(k, _)| k == "sheet").ok_or(Refused::NotFound)?;
        path = format!("{}.{}", path.rsplit_once('.').map_or(path.as_str(), |(s, _)| s), crate::spritesheet::EXT);
        hash.clone_from(h);
    }
    // A model only Blender reads is shown, and downloaded as a GLB, from the GLB Blender made of it.
    if (view || format == "glb" || format == super::CLIP) && !readable {
        let glb = derivs.iter().find(|(k, _)| k == "glb").ok_or(Refused::NotFound)?;
        path = format!("{}.glb", path.rsplit_once('.').map_or(path.as_str(), |(s, _)| s));
        ext = "glb".into();
        hash.clone_from(&glb.1);
    }
    // Cached as `as:<format>:<max>:<output ext>`: a WOFF unwraps to .ttf or .otf, a glTF zips.
    let prefix = format!("{key}:");
    if let Some((k, h)) = derivs.iter().find(|(k, _)| k.starts_with(&prefix)) {
        touch_conversion(state, asset_id, k);
        return Ok(Lookup::Ready(Ready { hash: h.clone(), ext: k[prefix.len()..].to_owned(), stem }));
    }

    let running_key = format!("{asset_id}|{key}");
    let mut running = RUNNING.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(rx) = running.get(&running_key) {
        return Ok(Lookup::Pending(rx.clone()));
    }
    let (tx, rx) = tokio::sync::watch::channel(Stage::Queued);
    running.insert(running_key.clone(), rx.clone());
    drop(running);
    let job = Job {
        state: std::sync::Arc::clone(state),
        asset_id: asset_id.to_owned(),
        path,
        ext,
        hash,
        format: format.to_owned(),
        max,
        key,
        stem,
        facing,
        naming,
        duration: serde_json::from_str::<Value>(&meta).ok().and_then(|m| m.get("duration_secs")?.as_f64()),
    };
    tokio::spawn(async move {
        let file = job.path.rsplit('/').next().unwrap_or(&job.path).to_owned();
        let size = job.max.map_or_default(|m| format!(" up to {m} px"));
        let activity = std::sync::Arc::new(crate::activity::start(
            "download_as",
            format!("Converting {file} to {}{size}", format_name(&job.format)),
            None,
        ));
        activity.queued(true);
        let result = job.run(&tx, &activity).await;
        drop(activity);
        let failed = result.is_err();
        tx.send_replace(match result {
            Ok(ready) => Stage::Done(ready),
            Err(e) => {
                tracing::warn!(key = %running_key, error = %format!("{e:#}"), "conversion failed");
                Stage::Failed(format!("{e:#}"))
            }
        });
        if failed {
            tokio::time::sleep(FAILURE_KEPT).await;
        }
        RUNNING.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&running_key);
    });
    Ok(Lookup::Pending(rx))
}

/// A format as people write it: `Opus`, `glTF`, `WebP`, `MP3`.
fn format_name(format: &str) -> String {
    match format {
        "opus" => "Opus".into(),
        "ogg" => "OGG Vorbis".into(),
        "gltf" => "glTF".into(),
        "webp" => "WebP".into(),
        "jpg" => "JPEG".into(),
        "tiff" => "TIFF".into(),
        "exr" => "OpenEXR".into(),
        "view" => "GLB for the viewer".into(),
        super::CLIP => "GLB clip".into(),
        other => crate::spritesheet::label(other).map_or_else(|| other.to_ascii_uppercase(), str::to_owned),
    }
}

/// Wait up to `limit` for a pending conversion to finish, and say where it stands.
pub(crate) async fn wait(mut rx: tokio::sync::watch::Receiver<Stage>, limit: std::time::Duration) -> Stage {
    let finished = |s: &Stage| matches!(s, Stage::Done(_) | Stage::Failed(_));
    let done = tokio::time::timeout(limit, rx.wait_for(finished)).await.ok().and_then(|r| r.ok().map(|s| s.clone()));
    done.unwrap_or_else(|| rx.borrow().clone())
}

/// Wait up to `limit` for a pending conversion to move (progress, or finishing), and say where it stands.
pub(crate) async fn wait_for_change(mut rx: tokio::sync::watch::Receiver<Stage>, limit: std::time::Duration) -> Stage {
    let current = rx.borrow_and_update().clone();
    if matches!(current, Stage::Done(_) | Stage::Failed(_)) {
        return current;
    }
    tokio::time::timeout(limit, rx.changed()).await.ok();
    rx.borrow().clone()
}

/// One conversion, detached from whoever asked for it.
struct Job {
    state: crate::AppState,
    asset_id: String,
    path: String,
    ext: String,
    hash: String,
    format: String,
    max: Option<u32>,
    key: String,
    stem: String,
    /// How the pack's models are turned (see [`crate::meshes::facing`]).
    facing: hoard_core::Facing,
    /// How a GLB's textures are named (see [`crate::naming::for_download`]).
    naming: crate::naming::ForDownload,
    duration: Option<f64>,
}

impl Job {
    async fn run(
        &self,
        tx: &tokio::sync::watch::Sender<Stage>,
        activity: &std::sync::Arc<crate::activity::Handle>,
    ) -> Result<Ready> {
        use crate::db;
        let _slot = if self.format == "view" { VIEW_SLOTS.acquire().await? } else { SLOTS.acquire().await? };
        activity.queued(false);
        tx.send_replace(Stage::Working(0.0));
        let state = &self.state;
        let work = state.cfg.tmp_dir().join(format!("as-{}", uuid::Uuid::now_v7()));
        let result = async {
            // A .gltf, .obj or .fbx needs its buffers, materials and images beside it, at their pack-relative paths.
            let sheet = crate::spritesheet::is_format(&self.format);
            let files = if sheet {
                // The sheet, and its pages at their paths in the pack, as the library has them now.
                let pages = crate::spritesheet::pages(&state.blobs.read_all(&self.hash).await?)?;
                let me = db::BundleFile {
                    id: self.asset_id.clone(),
                    path: self.path.clone(),
                    hash: self.hash.clone(),
                    size: 0,
                    unity_guid: None,
                };
                db::call(&state.pool, move |c| {
                    let mut now = c.prepare("SELECT blob_hash FROM assets WHERE id = ?1")?;
                    let mut files = vec![me];
                    for p in pages {
                        let current: Option<String> = match &p.asset {
                            Some(id) => rusqlite::OptionalExtension::optional(now.query_row([id], |r| r.get(0)))?,
                            None => None,
                        };
                        let Some(hash) = current.or(p.hash) else { continue };
                        files.push(db::BundleFile {
                            id: p.asset.unwrap_or_default(),
                            path: p.image,
                            hash,
                            size: 0,
                            unity_guid: None,
                        });
                    }
                    Ok(files)
                })
                .await?
            } else if matches!(self.ext.to_ascii_lowercase().as_str(), "gltf" | "obj" | "fbx") {
                let id = self.asset_id.clone();
                db::call(&state.pool, move |c| {
                    let ids = db::dependency_closure(c, &id, false)?;
                    db::bundle_files(c, &ids)
                })
                .await?
            } else {
                vec![db::BundleFile {
                    id: self.asset_id.clone(),
                    path: self.path.clone(),
                    hash: self.hash.clone(),
                    size: 0,
                    unity_guid: None,
                }]
            };
            for f in &files {
                let rel = crate::ingest::safe_rel_path(&f.path).context("unsafe path in pack")?;
                let dest = work.join(rel);
                if let Some(parent) = dest.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                state.blobs.download_to(&f.hash, &dest).await?;
            }
            let local = work.join(crate::ingest::safe_rel_path(&self.path).context("unsafe path in pack")?);
            let (src, fmt, max, duration, root, facing) =
                (self.ext.clone(), self.format.clone(), self.max, self.duration, work.clone(), self.facing);
            let naming = self.naming.clone();
            let progress_tx = tx.clone();
            let shown = std::sync::Arc::clone(activity);
            let out = crate::heavy::run("convert", move || {
                // Report in whole percents, so watchers aren't woken for every ffmpeg line.
                let last = std::sync::atomic::AtomicU32::new(0);
                let report = move |p: f32| {
                    let pct = (p.clamp(0.0, 1.0) * 100.0) as u32;
                    if last.swap(pct, std::sync::atomic::Ordering::Relaxed) != pct {
                        progress_tx.send_replace(Stage::Working(p.clamp(0.0, 1.0)));
                        shown.progress(p);
                    }
                };
                if sheet {
                    return crate::spritesheet::export(&local, &root, &fmt);
                }
                crate::naming::for_download(naming, || {
                    crate::meshes::facing(facing, || {
                        convert(&local, &root, &src, &fmt, max, &Progress { duration, report: &report })
                    })
                })
            })
            .await??;
            let out_ext = out.ext;
            let blob = state.blobs.put_bytes(out.bytes, hoard_core::AssetKind::from_ext(&self.ext)).await?;
            let (id, k, h) = (self.asset_id.clone(), format!("{}:{out_ext}", self.key), blob.hash.clone());
            db::call(&state.pool, move |c| {
                let now = db::now();
                c.execute(
                    "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
                    rusqlite::params![h, blob.size as i64, now],
                )?;
                c.execute(
                    "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash",
                    rusqlite::params![id, k, h, now],
                )?;
                Ok(())
            })
            .await?;
            let st = std::sync::Arc::clone(state);
            tokio::spawn(async move { log_trim(&st).await });
            anyhow::Ok(Ready { hash: blob.hash, ext: out_ext.to_owned(), stem: self.stem.clone() })
        }
        .await;
        tokio::fs::remove_dir_all(&work).await.ok();
        result
    }
}
