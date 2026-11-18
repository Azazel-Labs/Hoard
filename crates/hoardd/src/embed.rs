//! What image search (`clip`) and sound search (`clap`) share: a model's files, downloaded when its search is turned
//! on, and its weights read from them; its text half, loaded on the first search; the in-memory index searches rank
//! against; which embeddings there are, for the background jobs; and the threads the models' arithmetic runs on.

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex, PoisonError, RwLock};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, ensure};
use candle_core::{Device, Tensor};
use rusqlite::params;
use serde::Deserialize;

use crate::tools::{self, Slot, Status};
use crate::{AppState, db};

/// The tensors whose names start with one of `prefixes`, read from a safetensors file one at a time (so loading half
/// the model never holds the other half, or the whole file, in memory).
pub(crate) fn read_tensors(path: &Path, prefixes: &[&str]) -> Result<HashMap<String, Tensor>> {
    let (base, header) = safetensors_header(path)?;
    let mut file = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut out = HashMap::new();
    for (name, info) in header {
        if !prefixes.iter().any(|p| name.starts_with(p)) {
            continue;
        }
        // Position ids (I64) are buffers the model doesn't need; every weight is F32.
        if info.dtype != "F32" {
            continue;
        }
        let [start, end] = info.data_offsets;
        let count: usize = info.shape.iter().product();
        ensure!(end >= start && (end - start) as usize == count * 4, "tensor {name} has the wrong size");
        file.seek(SeekFrom::Start(base + start))?;
        let mut bytes = vec![0u8; count * 4];
        file.read_exact(&mut bytes)?;
        let values: Vec<f32> = bytes.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)).collect();
        out.insert(name, Tensor::from_vec(values, info.shape, &Device::Cpu)?);
    }
    ensure!(!out.is_empty(), "the model file has none of the tensors needed");
    Ok(out)
}

/// A tensor in a safetensors file.
#[derive(Debug, Deserialize)]
pub(crate) struct TensorInfo {
    pub(crate) dtype: String,
    pub(crate) shape: Vec<usize>,
    /// From `base`, see [`safetensors_header`].
    pub(crate) data_offsets: [u64; 2],
}

/// Where a safetensors file's data starts, and its tensors.
pub(crate) fn safetensors_header(path: &Path) -> Result<(u64, HashMap<String, TensorInfo>)> {
    let mut file = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut len = [0u8; 8];
    file.read_exact(&mut len)?;
    let len = u64::from_le_bytes(len);
    ensure!(len < 16 << 20, "the model file's header is unexpectedly large");
    let mut header = vec![0u8; len as usize];
    file.read_exact(&mut header)?;
    let header: HashMap<String, serde_json::Value> = serde_json::from_slice(&header)?;
    let tensors = header
        .into_iter()
        .filter(|(name, _)| name != "__metadata__")
        .map(|(name, info)| {
            let info = serde_json::from_value(info).with_context(|| format!("tensor {name}"))?;
            Ok((name, info))
        })
        .collect::<Result<_>>()?;
    Ok((8 + len, tensors))
}

pub(crate) fn normalized(mut v: Vec<f32>) -> Vec<f32> {
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut v {
            *x /= norm;
        }
    }
    v
}

/// Signed bytes scaled so the largest is ±127.
pub(crate) fn quantize(v: &[f32]) -> Vec<i8> {
    let max = v.iter().fold(0.0f32, |m, x| m.max(x.abs()));
    let k = if max > 0.0 { 127.0 / max } else { 0.0 };
    v.iter().map(|x| (x * k).round().clamp(-127.0, 127.0) as i8).collect()
}

// ---------------------------------------------------------------------------------------------
// Model files and status

/// A model that's downloaded when its search is turned on.
#[derive(Debug)]
pub(crate) struct Model {
    /// Its folder in `<cache>/models`.
    pub(crate) name: &'static str,
    pub(crate) repo: &'static str,
    pub(crate) files: &'static Files,
    /// What it's for, as the log and the activity say it.
    pub(crate) what: &'static str,
    /// The activity's kind while it downloads.
    pub(crate) activity: &'static str,
}

/// A model's files: name, size and SHA-256 of each.
pub(crate) type Files = [(&'static str, u64, &'static str)];

impl Model {
    fn dir(&self, state: &AppState) -> PathBuf {
        state.cfg.models_dir().join(self.name)
    }

    /// Find the model, downloading it if need be, and run `ready` once it's there. Retries hourly while it can't be
    /// had, until `slot` is restarted.
    pub(crate) fn fetch<F>(&'static self, state: &AppState, slot: &'static Slot<PathBuf>, ready: fn(AppState) -> F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let generation = slot.restart(Status::Looking);
        let st = Arc::clone(state);
        tokio::spawn(async move {
            let dir = self.dir(&st);
            loop {
                if files_present(&dir, self.files) {
                    if slot.set(generation, Status::Ready(dir.clone())) {
                        tracing::info!(dir = %dir.display(), what = self.what, "model ready");
                        ready(st).await;
                    }
                    return;
                }
                if !slot.set(generation, Status::Downloading) {
                    return;
                }
                if let Err(e) = self.download(&dir, &|| !slot.current(generation)).await {
                    if !slot.current(generation) {
                        return;
                    }
                    let msg = format!("{e:#}");
                    tracing::warn!(error = %msg, what = self.what, "couldn't download the model; retrying in an hour");
                    slot.set(generation, Status::Unavailable(msg));
                    tokio::time::sleep(Duration::from_secs(3600)).await;
                    if !slot.current(generation) {
                        return;
                    }
                }
            }
        });
    }

    /// The search was switched off: the model goes (the embeddings stay, for when it's back on).
    pub(crate) async fn remove_files(&self, state: &AppState) {
        let dir = self.dir(state);
        if dir.is_dir()
            && let Err(e) = tokio::fs::remove_dir_all(&dir).await
        {
            tracing::warn!(dir = %dir.display(), error = %e, "couldn't remove a model");
        }
    }

    /// Fetch whichever files aren't there yet, each checked against its SHA-256.
    async fn download(&self, dir: &Path, canceled: &(dyn Fn() -> bool + Sync)) -> Result<()> {
        let http = tools::client()?;
        tokio::fs::create_dir_all(dir).await?;
        let activity = crate::activity::start(self.activity, format!("Downloading the {} model", self.what), None);
        for &(name, size, sha256) in self.files {
            let dest = dir.join(name);
            if std::fs::metadata(&dest).is_ok_and(|m| m.len() == size) {
                continue;
            }
            activity.detail(name);
            let part = dir.join(format!("{name}.part"));
            let fetched = tools::fetch(&http, &format!("{}/{name}", self.repo), &part, size, &activity, canceled)
                .await
                .with_context(|| format!("downloading {name}"))?;
            if fetched.sha256 != sha256 {
                tokio::fs::remove_file(&part).await.ok();
                anyhow::bail!("{name} doesn't match its SHA-256");
            }
            tokio::fs::rename(&part, &dest).await?;
        }
        Ok(())
    }
}

/// All the files, at their sizes (their hashes were checked when they were downloaded).
pub(crate) fn files_present(dir: &Path, files: &Files) -> bool {
    files.iter().all(|(name, size, _)| std::fs::metadata(dir.join(name)).is_ok_and(|m| m.len() == *size))
}

// ---------------------------------------------------------------------------------------------
// Text embeddings for searches

/// How long a text half stays loaded after the last search.
const TEXT_IDLE: Duration = Duration::from_mins(15);

#[derive(Debug)]
struct Loaded<M> {
    model: Arc<M>,
    used: Instant,
    /// Recent searches' embeddings, as people scroll through the same results.
    recent: HashMap<String, Arc<Vec<f32>>>,
}

/// A model's text half, loaded on the first search and dropped after a while without one.
#[derive(Debug)]
pub(crate) struct Text<M> {
    loaded: Mutex<Option<Loaded<M>>>,
    unloader: std::sync::Once,
    what: &'static str,
    load: fn(&Path) -> Result<M>,
    embed: fn(&M, &str) -> Result<Vec<f32>>,
}

impl<M: Send + Sync + 'static> Text<M> {
    pub(crate) const fn new(
        what: &'static str,
        load: fn(&Path) -> Result<M>,
        embed: fn(&M, &str) -> Result<Vec<f32>>,
    ) -> Self {
        Self { loaded: Mutex::new(None), unloader: std::sync::Once::new(), what, load, embed }
    }

    pub(crate) fn clear(&self) {
        *self.loaded.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }

    /// Drop the model once nobody has searched for a while.
    pub(crate) fn unload_when_idle(&'static self) {
        self.unloader.call_once(|| {
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(60)).await;
                    let mut guard = self.loaded.lock().unwrap_or_else(PoisonError::into_inner);
                    if guard.as_ref().is_some_and(|l| l.used.elapsed() > TEXT_IDLE) {
                        *guard = None;
                        tracing::debug!(what = self.what, "text model unloaded");
                    }
                }
            });
        });
    }

    /// The embedding of `text` with the model in `dir`.
    pub(crate) async fn embed(&'static self, dir: PathBuf, text: String) -> Result<Arc<Vec<f32>>> {
        let model = {
            let mut guard = self.loaded.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some(l) = guard.as_mut() {
                l.used = Instant::now();
                if let Some(v) = l.recent.get(&text) {
                    return Ok(Arc::clone(v));
                }
                Some(Arc::clone(&l.model))
            } else {
                None
            }
        };
        let model = if let Some(m) = model {
            m
        } else {
            let started = Instant::now();
            let load = self.load;
            let m = Arc::new(tokio::task::spawn_blocking(move || load(&dir)).await??);
            tracing::info!(ms = started.elapsed().as_millis(), what = self.what, "text model loaded");
            m
        };
        let (m, t, embed) = (Arc::clone(&model), text.clone(), self.embed);
        let v = Arc::new(tokio::task::spawn_blocking(move || embed(&m, &t)).await??);
        let mut guard = self.loaded.lock().unwrap_or_else(PoisonError::into_inner);
        let loaded = guard.get_or_insert_with(|| Loaded { model, used: Instant::now(), recent: HashMap::new() });
        if loaded.recent.len() >= 64 {
            loaded.recent.clear();
        }
        loaded.recent.insert(text, Arc::clone(&v));
        Ok(v)
    }
}

// ---------------------------------------------------------------------------------------------
// Index and ranking

#[derive(Debug)]
pub(crate) struct Entry {
    v: Box<[i8]>,
    /// 1 / the vector's length, so a dot product becomes a cosine.
    inv_norm: f32,
}

impl Entry {
    /// `None` for an empty vector, kept for what isn't a picture (or a sound) after all.
    pub(crate) fn new(v: Vec<i8>) -> Option<Self> {
        let norm = v.iter().map(|&x| f32::from(x) * f32::from(x)).sum::<f32>().sqrt();
        (norm > 0.0).then(|| Self { v: v.into_boxed_slice(), inv_norm: 1.0 / norm })
    }

    fn cosine(&self, other: &Self) -> f32 {
        let dot: i32 = self.v.iter().zip(other.v.iter()).map(|(a, b)| i32::from(*a) * i32::from(*b)).sum();
        dot as f32 * self.inv_norm * other.inv_norm
    }

    pub(crate) fn cosine_f32(&self, q: &[f32]) -> f32 {
        self.v.iter().zip(q).map(|(a, b)| f32::from(*a) * b).sum::<f32>() * self.inv_norm
    }
}

/// Where an [`Index`]'s embeddings are kept and how its text matches are judged.
#[derive(Debug)]
pub(crate) struct Spec {
    /// Selects each hash and its vector, `?1` being the model.
    pub(crate) load: &'static str,
    pub(crate) model: &'static str,
    /// How many numbers each embedding has; 0 for any.
    pub(crate) dims: usize,
    /// What it searches, for the log.
    pub(crate) what: &'static str,
    /// The cosines of two unrelated pictures and of the same thing from another angle, for "more like this"; NaN to
    /// measure them against each picture.
    pub(crate) unrelated: f32,
    pub(crate) same: f32,
    /// The text thresholds below are in standard deviations of each search's scores rather than in cosines.
    pub(crate) relative: bool,
    /// How far above the median a text match must score, and how close to the best.
    pub(crate) lift: f32,
    pub(crate) margin: f32,
    /// How far above the median a match always counts, however far ahead the best one is.
    pub(crate) sure: f32,
    /// The lift where a text match starts to count, and how much more makes it a strong one.
    pub(crate) weak: f32,
    pub(crate) strong: f32,
}

/// Every embedding, in memory, loaded on first use and kept up to date by the job.
#[derive(Debug)]
pub(crate) struct Index {
    spec: RwLock<&'static Spec>,
    map: RwLock<Option<HashMap<String, Entry>>>,
}

impl Index {
    pub(crate) const fn new(spec: &'static Spec) -> Self {
        Self { spec: RwLock::new(spec), map: RwLock::new(None) }
    }

    pub(crate) fn spec(&self) -> &'static Spec {
        *self.spec.read().unwrap_or_else(PoisonError::into_inner)
    }

    /// Search another model's embeddings from now on.
    pub(crate) fn respec(&self, spec: &'static Spec) {
        let changed = {
            let mut cur = self.spec.write().unwrap_or_else(PoisonError::into_inner);
            let old: &'static Spec = std::mem::replace(&mut *cur, spec);
            !std::ptr::eq(old, spec)
        };
        if changed {
            self.forget();
        }
    }

    fn read<R>(&self, f: impl FnOnce(&HashMap<String, Entry>) -> R) -> Option<R> {
        self.map.read().unwrap_or_else(PoisonError::into_inner).as_ref().map(f)
    }

    pub(crate) fn write(&self, f: impl FnOnce(&mut HashMap<String, Entry>)) {
        if let Some(m) = self.map.write().unwrap_or_else(PoisonError::into_inner).as_mut() {
            f(m);
        }
    }

    pub(crate) async fn ensure(&self, state: &AppState) -> Result<()> {
        if self.read(|_| ()).is_some() {
            return Ok(());
        }
        let spec = self.spec();
        let loaded = db::call(&state.pool, move |c| {
            let mut stmt = c.prepare(spec.load)?;
            let rows = stmt
                .query_map([spec.model], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?)))?
                .filter_map(|row| {
                    let (hash, v) = row.ok()?;
                    (spec.dims == 0 || v.len() == spec.dims).then_some(())?;
                    Some((hash, Entry::new(v.into_iter().map(|b| b as i8).collect())?))
                })
                .collect::<HashMap<_, _>>();
            Ok(rows)
        })
        .await?;
        let mut guard = self.map.write().unwrap_or_else(PoisonError::into_inner);
        // Unless another model's were asked for meanwhile.
        if guard.is_none() && std::ptr::eq(self.spec(), spec) {
            tracing::debug!(embeddings = loaded.len(), what = spec.what, "embeddings loaded");
            *guard = Some(loaded);
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn set(&self, m: HashMap<String, Entry>) {
        *self.map.write().unwrap_or_else(PoisonError::into_inner) = Some(m);
    }

    pub(crate) fn len(&self) -> usize {
        self.read(HashMap::len).unwrap_or(0)
    }

    /// Let go of every embedding (they'll be loaded again if wanted).
    pub(crate) fn forget(&self) {
        *self.map.write().unwrap_or_else(PoisonError::into_inner) = None;
    }

    /// How much every thumbnail looks like `hash`'s by meaning, about 0..=1, or `None` when it has no embedding.
    pub(crate) fn similar(&self, hash: &str) -> Option<HashMap<String, f32>> {
        let spec = self.spec();
        self.read(|m| {
            let target = m.get(hash)?;
            let cosines: Vec<(&String, f32)> = m.iter().map(|(h, e)| (h, target.cosine(e))).collect();
            let (unrelated, same) = if spec.unrelated.is_nan() {
                // The typical picture's score against this one is what unrelated looks like.
                let mut all: Vec<f32> = cosines.iter().map(|c| c.1).collect();
                let mid = all.len() / 2;
                let typical = *all.select_nth_unstable_by(mid, f32::total_cmp).1;
                (typical, typical + HOSTED_SAME * (1.0 - typical))
            } else {
                (spec.unrelated, spec.same)
            };
            let score = |cosine: f32| ((cosine - unrelated) / (same - unrelated).max(1e-6)).clamp(0.0, 1.0);
            Some(cosines.into_iter().map(|(h, c)| (h.clone(), score(c))).collect())
        })
        .flatten()
    }

    /// What matches a text embedding, best first, scored about 0..=0.5.
    pub(crate) fn matching(&self, q: &[f32]) -> Vec<(String, f32)> {
        let spec = self.spec();
        self.read(|m| {
            let scored: Vec<(&String, f32)> = m.iter().map(|(h, e)| (h, e.cosine_f32(q))).collect();
            if scored.is_empty() {
                return Vec::new();
            }
            // Each query has its own level (some words sit closer to every picture), so a match is judged by how far
            // it stands above the typical picture's score for the same words.
            let mut all: Vec<f32> = scored.iter().map(|s| s.1).collect();
            let mid = all.len() / 2;
            let median = *all.select_nth_unstable_by(mid, f32::total_cmp).1;
            let unit = if spec.relative {
                let mean = all.iter().sum::<f32>() / all.len() as f32;
                (all.iter().map(|x| (x - mean) * (x - mean)).sum::<f32>() / all.len() as f32).sqrt()
            } else {
                1.0
            };
            if unit <= 0.0 || unit.is_nan() {
                return Vec::new();
            }
            let [lift, margin, sure, weak, strong] =
                [spec.lift, spec.margin, spec.sure, spec.weak, spec.strong].map(|x| x * unit);
            let best = scored.iter().map(|s| s.1).fold(f32::MIN, f32::max);
            let floor = (median + lift).max((best - margin).min(median + sure));
            let mut hits: Vec<(&String, f32)> = scored.into_iter().filter(|s| s.1 >= floor).collect();
            hits.sort_by(|a, b| b.1.total_cmp(&a.1));
            hits.truncate(TEXT_MAX);
            // A strong match counts double a word match in the name (0.25), a borderline one well under it.
            let score = |above: f32| ((above - weak) / (strong - weak)).clamp(0.0, 1.0) * 0.5;
            hits.into_iter().map(|(h, c)| (h.clone(), score(c - median))).collect()
        })
        .unwrap_or_default()
    }
}

/// The most a text search keeps.
const TEXT_MAX: usize = 300;

/// How far from a picture's typical score to a perfect one the same thing from another angle scores, from SigLIP 2's.
const HOSTED_SAME: f32 = 0.83;

// ---------------------------------------------------------------------------------------------
// Which embeddings there are

/// Where a search's embeddings are stored, and which assets get one.
#[derive(Debug)]
pub(crate) struct Table {
    /// The table, and its column naming what's embedded.
    pub(crate) name: &'static str,
    pub(crate) key: &'static str,
    /// The assets' column that names it, and which assets count (SQL over `a`).
    pub(crate) asset_key: &'static str,
    pub(crate) kinds: &'static str,
    /// Embeddings nothing uses any more (SQL over `e`).
    pub(crate) orphan: &'static str,
    /// The job that fills it.
    pub(crate) job: &'static str,
}

impl Table {
    /// SQL over `a`: the assets that could have an embedding.
    pub(crate) fn wanted(&self) -> String {
        format!("a.{} IS NOT NULL AND {}", self.asset_key, self.kinds)
    }

    /// SQL over `a`: no embedding from the model `?1` yet.
    pub(crate) fn missing(&self) -> String {
        format!(
            "NOT EXISTS (SELECT 1 FROM {} e WHERE e.{} = a.{} AND e.model = ?1)",
            self.name, self.key, self.asset_key
        )
    }

    /// How many have an embedding from `model`, and how many could, for the Settings page.
    pub(crate) fn progress(&self, conn: &rusqlite::Connection, model: &str) -> Result<(u64, u64)> {
        let total: i64 = conn.query_row(
            &format!("SELECT COUNT(DISTINCT a.{}) FROM assets a WHERE {}", self.asset_key, self.wanted()),
            [],
            |r| r.get(0),
        )?;
        let done: i64 = conn.query_row(
            &format!(
                "SELECT COUNT(*) FROM {} e WHERE e.model = ?1
                   AND EXISTS (SELECT 1 FROM assets a WHERE a.{} = e.{} AND {})",
                self.name, self.asset_key, self.key, self.kinds
            ),
            [model],
            |r| r.get(0),
        )?;
        Ok((done as u64, total as u64))
    }

    /// Queue the job when something has no embedding from `model` or one is left over, unless it's queued already.
    /// Whether it was.
    pub(crate) async fn queue(&'static self, state: &AppState, model: String) -> Result<bool> {
        let queued = db::call(&state.pool, move |c| {
            let waiting: bool = c.query_row(
                "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued')",
                [self.job],
                |r| r.get(0),
            )?;
            if waiting {
                return Ok(false);
            }
            let work: bool = c.query_row(
                &format!(
                    "SELECT EXISTS (SELECT 1 FROM assets a WHERE {} AND {})
                         OR EXISTS (SELECT 1 FROM {} e WHERE {})",
                    self.wanted(),
                    self.missing(),
                    self.name,
                    self.orphan
                ),
                [&model],
                |r| r.get(0),
            )?;
            if work {
                db::enqueue(c, self.job, None, &serde_json::json!({}))?;
            }
            Ok(work)
        })
        .await?;
        if queued {
            state.wake.notify_one();
        }
        Ok(queued)
    }

    /// Delete the embeddings nothing uses any more, saying which.
    pub(crate) async fn forget_orphans(&'static self, state: &AppState) -> Result<Vec<String>> {
        db::call(&state.pool, move |c| {
            let gone = c
                .prepare(&format!("SELECT {} FROM {} e WHERE {}", self.key, self.name, self.orphan))?
                .query_map([], |r| r.get(0))?
                .collect::<rusqlite::Result<Vec<String>>>()?;
            c.execute(&format!("DELETE FROM {} AS e WHERE {}", self.name, self.orphan), [])?;
            Ok(gone)
        })
        .await
    }

    /// Store what `model` made of each. What it couldn't read (not a picture or a sound after all) gets an empty
    /// row, so it isn't tried again.
    pub(crate) async fn save(
        &'static self,
        state: &AppState,
        model: &str,
        computed: &[(String, Option<Vec<i8>>)],
    ) -> Result<()> {
        let rows: Vec<(String, Vec<u8>)> = computed
            .iter()
            .map(|(h, v)| (h.clone(), v.as_ref().map_or_default(|v| v.iter().map(|&x| x as u8).collect())))
            .collect();
        let model = model.to_owned();
        db::call(&state.pool, move |c| {
            let tx = c.transaction()?;
            {
                let mut stmt = tx.prepare(&format!(
                    "INSERT OR REPLACE INTO {} ({}, model, vector) VALUES (?1, ?2, ?3)",
                    self.name, self.key
                ))?;
                for (hash, v) in &rows {
                    stmt.execute(params![hash, model, v])?;
                }
            }
            tx.commit()?;
            Ok(())
        })
        .await
    }
}

// ---------------------------------------------------------------------------------------------
// Threads

/// Background threads for the model's arithmetic: half the cores, at low priority.
static POOL: LazyLock<Option<rayon::ThreadPool>> = LazyLock::new(|| {
    let threads = std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1));
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .thread_name(|i| format!("hoard-model-{i}"))
        .start_handler(|_| crate::heavy::lower_priority())
        .build()
        .map_err(|e| tracing::warn!(error = %e, "couldn't start the search models' threads"))
        .ok()
});

pub(crate) fn in_pool<T: Send>(f: impl FnOnce() -> T + Send) -> T {
    match POOL.as_ref() {
        Some(pool) => pool.install(f),
        None => f(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIMS: usize = 512;

    #[test]
    fn quantized_vectors_keep_their_cosine() {
        let a = normalized((0..DIMS).map(|i| ((i * 7) % 13) as f32 - 6.0).collect());
        let b = normalized((0..DIMS).map(|i| ((i * 5) % 11) as f32 - 5.0 + (i % 3) as f32).collect());
        let exact: f32 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
        let (qa, qb) = (Entry::new(quantize(&a)).unwrap(), Entry::new(quantize(&b)).unwrap());
        assert!((qa.cosine(&qb) - exact).abs() < 0.01, "{} vs {exact}", qa.cosine(&qb));
        assert!((qa.cosine_f32(&b) - exact).abs() < 0.01);
        assert!((qa.cosine(&qa) - 1.0).abs() < 1e-4);
        assert!(Entry::new(vec![0; DIMS]).is_none(), "an unreadable thumbnail's empty vector");
        assert!(Entry::new(Vec::new()).is_none());
    }
}
