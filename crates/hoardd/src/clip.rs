//! Search by meaning: every thumbnail gets an embedding from a model that knows what pictures show (Google's SigLIP 2,
//! see `siglip`), and so does what's typed in the search box, so "mossy rock" finds mossy rocks whatever the files are
//! called, and "more like this" knows a barrel from a crate rather than only matching colors and grain.
//!
//! It's off until an admin turns it on in Settings, because the model is a 1.5 GB download and takes 300–600 MB of
//! memory while it runs. Turned on, the model is fetched (pinned to one revision, checked against its SHA-256) into
//! `<cache>/models`, and a background job works through the thumbnails at low priority. The text half is loaded on
//! the first search and dropped after a while without one; the image half only lives while the job runs.
//!
//! Embeddings are stored per thumbnail blob and model (`embeddings`) as signed bytes, scaled per vector, which cosine
//! similarity doesn't mind. Searching is brute force over an in-memory index, like `looks`.
//!
//! Image search ran CLIP (`OpenAI`'s ViT-B/32) before. Where a library has CLIP's embeddings, searches keep using them,
//! and CLIP's words half while its files are still there, until SigLIP 2 has relearned every thumbnail; then they and
//! CLIP's files go.
//!
//! An admin can have a hosted service do the model's part instead (see `hosted`): then nothing is downloaded, and
//! every thumbnail and every search's words are sent to it. Its embeddings are stored under its own model name, so
//! switching back and forth keeps what each learned.

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, PoisonError, RwLock};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, ensure};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_nn::{Embedding, LayerNorm, Linear, VarBuilder};
use rusqlite::params;
use serde::Deserialize;

use crate::tools::{self, Slot, Status};
use crate::{AppState, db, hosted, siglip};

/// What image search runs, stored with each embedding.
pub(crate) const MODEL: &str = siglip::MODEL;
/// The whole download, for the Settings page.
pub(crate) const DOWNLOAD_BYTES: u64 = siglip::FILES[0].1 + siglip::FILES[1].1;

/// CLIP's embeddings, used until SigLIP 2 has relearned every thumbnail.
const CLIP_MODEL: &str = "clip-vit-b32";
const CLIP_DIMS: usize = 512;
const WEIGHTS: &str = "model.safetensors";
const TOKENIZER: &str = "tokenizer.json";
/// CLIP's files, by name and size, in its folder from before.
const CLIP_FILES: [(&str, u64, &str); 2] = [(TOKENIZER, 2_224_041, ""), (WEIGHTS, 605_157_884, "")];

// ---------------------------------------------------------------------------------------------
// CLIP's words half, for searching what it learned (adapted from candle-transformers' CLIP, MIT/Apache-2.0)

const MAX_TOKENS: usize = 77;
const VOCAB: usize = 49_408;
const EOS: u32 = 49_407;

fn quick_gelu(xs: &Tensor) -> candle_core::Result<Tensor> {
    xs * candle_nn::ops::sigmoid(&(xs * 1.702)?)?
}

/// Multi-head self-attention, as CLIP and SigLIP have it.
#[derive(Debug)]
pub(crate) struct Attention {
    q: Linear,
    k: Linear,
    v: Linear,
    out: Linear,
    heads: usize,
}

impl Attention {
    pub(crate) fn new(vb: &VarBuilder, dim: usize, heads: usize) -> candle_core::Result<Self> {
        let l = |name: &str| candle_nn::linear(dim, dim, vb.pp(name));
        Ok(Self { q: l("q_proj")?, k: l("k_proj")?, v: l("v_proj")?, out: l("out_proj")?, heads })
    }

    #[expect(clippy::many_single_char_names, reason = "the usual names in attention: batch, tokens, q, k, v, w, o")]
    pub(crate) fn forward(&self, xs: &Tensor, mask: Option<&Tensor>) -> candle_core::Result<Tensor> {
        let (b, n, dim) = xs.dims3()?;
        let hd = dim / self.heads;
        let split = |t: Tensor| t.reshape((b, n, self.heads, hd))?.transpose(1, 2)?.reshape((b * self.heads, n, hd));
        let q = split((self.q.forward(xs)? * (hd as f64).powf(-0.5))?)?;
        let k = split(self.k.forward(xs)?)?;
        let v = split(self.v.forward(xs)?)?;
        let mut w = q.matmul(&k.t()?)?;
        if let Some(mask) = mask {
            w = w.broadcast_add(mask)?;
        }
        let w = candle_nn::ops::softmax_last_dim(&w)?;
        let o = w.matmul(&v)?.reshape((b, self.heads, n, hd))?.transpose(1, 2)?.reshape((b, n, dim))?;
        self.out.forward(&o)
    }
}

#[derive(Debug)]
struct Layer {
    attn: Attention,
    ln1: LayerNorm,
    fc1: Linear,
    fc2: Linear,
    ln2: LayerNorm,
}

impl Layer {
    fn new(vb: &VarBuilder, dim: usize, heads: usize, hidden: usize) -> candle_core::Result<Self> {
        Ok(Self {
            attn: Attention::new(&vb.pp("self_attn"), dim, heads)?,
            ln1: candle_nn::layer_norm(dim, 1e-5, vb.pp("layer_norm1"))?,
            fc1: candle_nn::linear(dim, hidden, vb.pp("mlp.fc1"))?,
            fc2: candle_nn::linear(hidden, dim, vb.pp("mlp.fc2"))?,
            ln2: candle_nn::layer_norm(dim, 1e-5, vb.pp("layer_norm2"))?,
        })
    }

    fn forward(&self, xs: &Tensor, mask: Option<&Tensor>) -> candle_core::Result<Tensor> {
        let xs = (self.attn.forward(&self.ln1.forward(xs)?, mask)? + xs)?;
        let h = self.fc2.forward(&quick_gelu(&self.fc1.forward(&self.ln2.forward(&xs)?)?)?)?;
        h + xs
    }
}

fn encoder(vb: &VarBuilder, dim: usize, heads: usize, hidden: usize) -> candle_core::Result<Vec<Layer>> {
    (0..12).map(|i| Layer::new(&vb.pp(format!("encoder.layers.{i}")), dim, heads, hidden)).collect()
}

/// The text half: words in, an embedding out.
#[derive(Debug)]
struct TextModel {
    tokenizer: tokenizers::Tokenizer,
    tokens: Embedding,
    positions: Tensor,
    layers: Vec<Layer>,
    final_ln: LayerNorm,
    projection: Linear,
}

impl TextModel {
    fn load(dir: &Path) -> Result<Self> {
        let tokenizer = tokenizers::Tokenizer::from_file(dir.join(TOKENIZER)).map_err(anyhow::Error::msg)?;
        let vb = VarBuilder::from_tensors(
            read_tensors(&dir.join(WEIGHTS), &["text_model.", "text_projection."])?,
            DType::F32,
            &Device::Cpu,
        );
        let t = vb.pp("text_model");
        Ok(Self {
            tokenizer,
            tokens: candle_nn::embedding(VOCAB, 512, t.pp("embeddings.token_embedding"))?,
            positions: t.pp("embeddings.position_embedding").get((MAX_TOKENS, 512), "weight")?,
            layers: encoder(&t, 512, 8, 2048)?,
            final_ln: candle_nn::layer_norm(512, 1e-5, t.pp("final_layer_norm"))?,
            projection: candle_nn::linear_no_bias(512, CLIP_DIMS, vb.pp("text_projection"))?,
        })
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let ids = tokens(&self.tokenizer, text)?;
        let n = ids.len();
        let input = Tensor::new(ids.as_slice(), &Device::Cpu)?.unsqueeze(0)?;
        let mut xs = self.tokens.forward(&input)?.broadcast_add(&self.positions.narrow(0, 0, n)?)?;
        let mask: Vec<f32> = (0..n).flat_map(|i| (0..n).map(move |j| if j > i { f32::MIN } else { 0.0 })).collect();
        let mask = Tensor::from_vec(mask, (n, n), &Device::Cpu)?;
        for layer in &self.layers {
            xs = layer.forward(&xs, Some(&mask))?;
        }
        // The end-of-text token, last, sums up the sentence.
        let pooled = self.final_ln.forward(&xs)?.i((.., n - 1, ..))?;
        let v = self.projection.forward(&pooled)?.squeeze(0)?.to_vec1::<f32>()?;
        Ok(normalized(v))
    }
}

/// Token ids for `text`, start and end markers included, cut to what the model reads.
fn tokens(tokenizer: &tokenizers::Tokenizer, text: &str) -> Result<Vec<u32>> {
    let mut ids = tokenizer.encode(text, true).map_err(anyhow::Error::msg)?.get_ids().to_vec();
    ensure!(ids.len() >= 2, "the tokenizer gave no start and end markers");
    if ids.len() > MAX_TOKENS {
        ids.truncate(MAX_TOKENS - 1);
        ids.push(EOS);
    }
    Ok(ids)
}

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
// Model files and status (shared with sound search, see `crate::clap`)

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
fn files_present(dir: &Path, files: &Files) -> bool {
    files.iter().all(|(name, size, _)| std::fs::metadata(dir.join(name)).is_ok_and(|m| m.len() == *size))
}

static SIGLIP: Model =
    Model { name: siglip::MODEL, repo: siglip::REPO, files: &siglip::FILES, what: "image search", activity: "clip" };

static SLOT: Slot<PathBuf> = Slot::new();

/// Searches still use CLIP's embeddings: some are kept, and SigLIP 2 hasn't relearned every thumbnail yet.
static RELEARNING: AtomicBool = AtomicBool::new(false);
/// How many times image search was turned on or off, so an older start can't set [`RELEARNING`].
static STARTS: AtomicU64 = AtomicU64::new(0);
/// CLIP's embeddings were deleted, and nothing writes them any more, so searches never go back to them.
static CLIP_GONE: AtomicBool = AtomicBool::new(false);

/// Image search through a hosted service, when Settings says so.
static HOSTED: RwLock<Option<Arc<hosted::Client>>> = RwLock::new(None);

fn hosted() -> Option<Arc<hosted::Client>> {
    HOSTED.read().unwrap_or_else(PoisonError::into_inner).clone()
}

/// What the embeddings searches use are stored as: the hosted service's model, or SigLIP 2.
pub(crate) fn model_id() -> String {
    hosted().map_or_else(|| MODEL.to_owned(), |c| c.hosted.id())
}

pub(crate) fn is_ready() -> bool {
    hosted().is_some() || SLOT.ready().is_some()
}

/// Whether a hosted service does the model's part.
pub(crate) fn is_hosted() -> bool {
    hosted().is_some()
}

fn is_on() -> bool {
    hosted().is_some() || SLOT.describe(|_| String::new()) != "off"
}

/// Whether searches still use what CLIP learned, while SigLIP 2 relearns it.
pub(crate) fn relearning() -> bool {
    RELEARNING.load(Ordering::Relaxed) && !CLIP_GONE.load(Ordering::Relaxed) && is_on()
}

/// `off`, `looking`, `downloading`, `ready` or `unavailable: <why>`.
pub(crate) fn describe() -> String {
    if let Some(c) = hosted() {
        return c.failed().map_or_else(|| "ready".into(), |why| format!("unavailable: {why}"));
    }
    SLOT.describe(|_| "ready".into())
}

/// CLIP's folder from before.
fn clip_dir(state: &AppState) -> PathBuf {
    state.cfg.models_dir().join(CLIP_MODEL)
}

/// Turn image search on (find the model, downloading it if need be, or use the hosted service `remote`) or off.
/// Calling it again drops whatever the last call was doing.
pub(crate) fn start(state: &AppState, enabled: bool, remote: Option<hosted::Hosted>) {
    TEXT.clear();
    CLIP_TEXT.clear();
    let generation = STARTS.fetch_add(1, Ordering::Relaxed) + 1;
    RELEARNING.store(false, Ordering::Relaxed);
    let client = remote.filter(|_| enabled).map(|h| Arc::new(hosted::Client::new(h)));
    state.pictures.respec(client.as_ref().map_or(&SPEC, |c| hosted_spec(&c.hosted.id())));
    (*HOSTED.write().unwrap_or_else(PoisonError::into_inner)).clone_from(&client);
    if !enabled {
        SLOT.restart(Status::Off);
        return;
    }
    // Searches fall back on what CLIP learned only while SigLIP 2 here relearns it: CLIP is a model here too.
    let local = client.is_none();
    let st = Arc::clone(state);
    tokio::spawn(async move {
        let kept = db::call(&st.pool, |c| {
            Ok(c.query_row("SELECT EXISTS (SELECT 1 FROM embeddings WHERE model = ?1)", [CLIP_MODEL], |r| r.get(0))?)
        })
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(error = ?e, "couldn't tell whether CLIP's embeddings are kept");
            false
        });
        if STARTS.load(Ordering::Relaxed) == generation {
            RELEARNING.store(kept && local, Ordering::Relaxed);
            CLIP_TEXT.unload_when_idle();
        }
    });
    if client.is_some() {
        SLOT.restart(Status::Off);
        let st = Arc::clone(state);
        tokio::spawn(async move {
            // A try that failed waits an hour; with the service or its key changed, try now.
            let now = db::now();
            let soon = db::call(&st.pool, move |c| {
                Ok(c.execute(
                    "UPDATE jobs SET run_after = ?2 WHERE kind = ?1 AND status = 'queued' AND run_after > ?2",
                    rusqlite::params![crate::jobs::EMBED, now],
                )?)
            })
            .await;
            match soon {
                Ok(0) => {}
                Ok(_) => st.wake.notify_one(),
                Err(e) => tracing::warn!(error = ?e, "couldn't bring image search's next try forward"),
            }
            schedule(&st).await;
        });
        return;
    }
    SIGLIP.fetch(state, &SLOT, |st| async move {
        TEXT.unload_when_idle();
        schedule(&st).await;
    });
}

/// Image search was switched off: the models go (the embeddings stay, for when it's back on).
pub(crate) async fn remove_files(state: &AppState) {
    SIGLIP.remove_files(state).await;
    remove_clip_files(state).await;
}

async fn remove_clip_files(state: &AppState) {
    let dir = clip_dir(state);
    if dir.is_dir()
        && let Err(e) = tokio::fs::remove_dir_all(&dir).await
    {
        tracing::warn!(dir = %dir.display(), error = %e, "couldn't remove CLIP");
    }
}

/// SigLIP 2 has every thumbnail: what CLIP learned, and CLIP itself, go.
async fn relearned(state: &AppState) -> Result<()> {
    if CLIP_GONE.load(Ordering::Relaxed) {
        return Ok(());
    }
    let n = db::call(&state.pool, |c| Ok(c.execute("DELETE FROM embeddings WHERE model = ?1", [CLIP_MODEL])?)).await?;
    CLIP_GONE.store(true, Ordering::Relaxed);
    RELEARNING.store(false, Ordering::Relaxed);
    CLIP_TEXT.clear();
    state.clip.forget();
    remove_clip_files(state).await;
    if n > 0 {
        tracing::info!(pictures = n, "image search relearned every picture; CLIP's embeddings are gone");
    }
    Ok(())
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

static TEXT: Text<siglip::TextModel> = Text::new("image search", siglip::TextModel::load, siglip::TextModel::embed);
static CLIP_TEXT: Text<TextModel> = Text::new("image search (CLIP)", TextModel::load, TextModel::embed);

/// The embedding of what was typed, or `None` while the model isn't ready.
async fn embed_text(state: &AppState, text: &str) -> Result<Option<Arc<Vec<f32>>>> {
    if let Some(c) = hosted() {
        return Ok(Some(c.words(&state.http, text.trim()).await?));
    }
    let Some(dir) = SLOT.ready() else { return Ok(None) };
    // Said the way it was taught, which matches pictures better than the bare words.
    let v = TEXT.embed(dir, format!("this is a photo of {}.", text.trim().to_lowercase())).await?;
    Ok(Some(v))
}

/// The same with CLIP, while its files are still there.
async fn embed_text_clip(state: &AppState, text: &str) -> Result<Option<Arc<Vec<f32>>>> {
    let dir = clip_dir(state);
    if !files_present(&dir, &CLIP_FILES) {
        return Ok(None);
    }
    // "a photo of …" matches pictures a little better than the bare words.
    let v = CLIP_TEXT.embed(dir, format!("a photo of {}", text.trim().to_lowercase())).await?;
    Ok(Some(v))
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

/// Image search's embeddings: per thumbnail.
pub(crate) static SPEC: Spec = Spec {
    load: "SELECT thumb_hash, vector FROM embeddings WHERE model = ?1",
    model: MODEL,
    dims: siglip::DIMS,
    what: "image search",
    unrelated: SIGLIP_UNRELATED,
    same: SIGLIP_SAME,
    relative: false,
    lift: SIGLIP_LIFT,
    margin: SIGLIP_MARGIN,
    sure: f32::INFINITY,
    weak: SIGLIP_WEAK,
    strong: SIGLIP_STRONG,
};

/// CLIP's, until SigLIP 2 has relearned every thumbnail.
pub(crate) static CLIP_SPEC: Spec = Spec {
    load: "SELECT thumb_hash, vector FROM embeddings WHERE model = ?1",
    model: CLIP_MODEL,
    dims: CLIP_DIMS,
    what: "image search (CLIP)",
    unrelated: 0.5,
    same: 0.95,
    relative: false,
    lift: CLIP_LIFT,
    margin: CLIP_MARGIN,
    sure: f32::INFINITY,
    weak: 0.03,
    strong: 0.1,
};

/// A hosted model's. Each model's scores sit differently and none can be measured here ahead of time, so a match is
/// judged against the spread of each search's own scores. Made once per model, as an admin switches seldom.
fn hosted_spec(id: &str) -> &'static Spec {
    static SPECS: Mutex<Vec<&'static Spec>> = Mutex::new(Vec::new());
    let mut specs = SPECS.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(s) = specs.iter().find(|s| s.model == id) {
        return s;
    }
    let spec: &'static Spec = Box::leak(Box::new(Spec {
        load: SPEC.load,
        model: Box::leak(id.to_owned().into_boxed_str()),
        dims: 0,
        what: "image search (hosted)",
        unrelated: f32::NAN,
        same: f32::NAN,
        relative: true,
        lift: HOSTED_LIFT,
        margin: HOSTED_MARGIN,
        sure: f32::INFINITY,
        weak: HOSTED_WEAK,
        strong: HOSTED_STRONG,
    }));
    specs.push(spec);
    spec
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

    fn spec(&self) -> &'static Spec {
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

/// How far above the median picture a match must score. Words score far lower against pictures than pictures do
/// against each other: on a mixed library, a true match stands 0.05–0.1 above the median, and words with nothing
/// to find (a car among rocks and furniture) peak around 0.03.
const CLIP_LIFT: f32 = 0.045;
/// Keep only what's within this of the best match.
const CLIP_MARGIN: f32 = 0.04;
const TEXT_MAX: usize = 300;

/// The same for SigLIP 2, measured on a mixed library of sprites, tiles and model renders. Unrelated pictures score
/// about 0.7 against each other, the same thing from another angle or in another format 0.85 and up. A true text
/// match stands 0.04–0.11 above the median, and words with nothing to find peak around 0.035–0.06.
const SIGLIP_UNRELATED: f32 = 0.7;
const SIGLIP_SAME: f32 = 0.95;
const SIGLIP_LIFT: f32 = 0.045;
const SIGLIP_MARGIN: f32 = 0.035;
const SIGLIP_WEAK: f32 = 0.035;
const SIGLIP_STRONG: f32 = 0.08;

/// A hosted model's, in standard deviations of each search's scores, taken from SigLIP 2's: there a true text match
/// stands 3 or more above the median and words with nothing to find peak around 2.7–3.2, so a lift of 3 keeps about
/// what 0.045 does.
const HOSTED_LIFT: f32 = 3.0;
const HOSTED_MARGIN: f32 = 2.2;
const HOSTED_WEAK: f32 = 2.8;
const HOSTED_STRONG: f32 = 4.5;
/// How far from a picture's typical score to a perfect one the same thing from another angle scores, from SigLIP 2's.
const HOSTED_SAME: f32 = 0.83;

/// The index searches use: SigLIP 2's (or the hosted model's), or CLIP's while it's relearned.
fn index(state: &AppState) -> Option<&Index> {
    if relearning() {
        Some(&state.clip)
    } else if is_ready() {
        Some(&state.pictures)
    } else {
        None
    }
}

/// How much every thumbnail shows what `hash`'s does, about 0..=1, when image search is on and it has an embedding.
pub(crate) async fn similar(state: &AppState, hash: &str) -> Option<HashMap<String, f32>> {
    let index = index(state)?;
    if let Err(e) = index.ensure(state).await {
        tracing::warn!(error = %format!("{e:#}"), "couldn't load image search's index");
        return None;
    }
    index.similar(hash)
}

/// Thumbnails that match what was typed, best first; empty when image search is off or has nothing yet.
pub(crate) async fn search_text(state: &AppState, text: &str) -> Result<Vec<(String, f32)>> {
    let Some(index) = index(state) else { return Ok(Vec::new()) };
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    index.ensure(state).await?;
    if index.len() == 0 {
        return Ok(Vec::new());
    }
    let q = if relearning() { embed_text_clip(state, text).await? } else { embed_text(state, text).await? };
    let Some(q) = q else { return Ok(Vec::new()) };
    Ok(index.matching(&q))
}

// ---------------------------------------------------------------------------------------------
// Background job

const MISSING_SQL: &str = "NOT EXISTS (SELECT 1 FROM embeddings e WHERE e.thumb_hash = a.thumb_hash AND e.model = ?1)";
const ORPHAN_SQL: &str = "NOT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash = e.thumb_hash)";

/// How many thumbnails have an embedding, and how many could, for the Settings page.
pub(crate) fn progress(conn: &rusqlite::Connection) -> Result<(u64, u64)> {
    let kinds = crate::looks::KINDS_SQL;
    let total: i64 = conn.query_row(
        &format!("SELECT COUNT(DISTINCT a.thumb_hash) FROM assets a WHERE a.thumb_hash IS NOT NULL AND {kinds}"),
        [],
        |r| r.get(0),
    )?;
    let done: i64 = conn.query_row(
        &format!(
            "SELECT COUNT(*) FROM embeddings e WHERE e.model = ?1
               AND EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash = e.thumb_hash AND {kinds})"
        ),
        [model_id()],
        |r| r.get(0),
    )?;
    Ok((done as u64, total as u64))
}

/// Queue an embedding job when image search is on and some thumbnail has no embedding yet.
pub(crate) async fn schedule(state: &AppState) {
    if !is_ready() {
        return;
    }
    let kinds = crate::looks::KINDS_SQL;
    let model = model_id();
    let queued = db::call(&state.pool, move |c| {
        let waiting: bool = c.query_row(
            "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued')",
            [crate::jobs::EMBED],
            |r| r.get(0),
        )?;
        if waiting {
            return Ok(false);
        }
        let work: bool = c.query_row(
            &format!(
                "SELECT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash IS NOT NULL AND {kinds} AND {MISSING_SQL})
                     OR EXISTS (SELECT 1 FROM embeddings e WHERE {ORPHAN_SQL})"
            ),
            [&model],
            |r| r.get(0),
        )?;
        if work {
            db::enqueue(c, crate::jobs::EMBED, None, &serde_json::json!({}))?;
        }
        Ok(work)
    })
    .await;
    match queued {
        Ok(true) => state.wake.notify_one(),
        Ok(false) => {
            if let Err(e) = relearned(state).await {
                tracing::warn!(error = ?e, "couldn't let CLIP's embeddings go");
            }
        }
        Err(e) => tracing::warn!(error = ?e, "could not queue the image search job"),
    }
}

/// Thumbnails read and embedded per round.
const BATCH: usize = 32;

/// Background threads for the model's arithmetic: half the cores, at low priority.
static POOL: LazyLock<Option<rayon::ThreadPool>> = LazyLock::new(|| {
    let threads = std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1));
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .thread_name(|i| format!("hoard-clip-{i}"))
        .start_handler(|_| crate::heavy::lower_priority())
        .build()
        .map_err(|e| tracing::warn!(error = %e, "couldn't start the image search threads"))
        .ok()
});

pub(crate) fn in_pool<T: Send>(f: impl FnOnce() -> T + Send) -> T {
    match POOL.as_ref() {
        Some(pool) => pool.install(f),
        None => f(),
    }
}

/// Embed every thumbnail that has no embedding, and forget the ones nothing uses.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    let (remote, dir) = (hosted(), SLOT.ready());
    if remote.is_none() && dir.is_none() {
        return Ok(());
    }
    let id = model_id();
    state.pictures.ensure(state).await?;
    let gone: Vec<String> = db::call(&state.pool, |c| {
        let gone = c
            .prepare(&format!("SELECT thumb_hash FROM embeddings e WHERE {ORPHAN_SQL}"))?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        c.execute(&format!("DELETE FROM embeddings AS e WHERE {ORPHAN_SQL}"), [])?;
        Ok(gone)
    })
    .await?;
    for index in [&state.pictures, &state.clip] {
        index.write(|m| {
            for h in &gone {
                m.remove(h);
            }
        });
    }

    let kinds = crate::looks::KINDS_SQL;
    let mut model: Option<Arc<siglip::VisionModel>> = None;
    let mut unreadable = std::collections::HashSet::new();
    let mut done = 0usize;
    loop {
        let (skip, want) = (unreadable.clone(), id.clone());
        let todo: Vec<String> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT DISTINCT a.thumb_hash FROM assets a
                      WHERE a.thumb_hash IS NOT NULL AND {kinds} AND {MISSING_SQL}"
                ))?
                .query_map([want], |r| r.get(0))?
                .collect::<rusqlite::Result<Vec<String>>>()?;
            Ok(rows.into_iter().filter(|h| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return relearned(state).await;
        }
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            // Switched off or to another model partway through: stop, keeping what's done.
            if !is_ready() || model_id() != id {
                return Ok(());
            }
            crate::activity::job_progress(crate::jobs::EMBED, None, done, total, "pictures");
            let mut pictures = Vec::with_capacity(chunk.len());
            for hash in chunk {
                match state.blobs.read_all(hash).await {
                    Ok(bytes) => pictures.push((hash.clone(), bytes)),
                    Err(e) => {
                        tracing::debug!(hash, error = %e, "thumbnail unreadable for image search");
                        unreadable.insert(hash.clone());
                    }
                }
            }
            let computed = if let Some(c) = &remote {
                let bytes: Vec<&[u8]> = pictures.iter().map(|(_, b)| b.as_ref()).collect();
                let vs = c.pictures(&state.http, &bytes).await?;
                pictures.iter().zip(vs).map(|((h, _), v)| (h.clone(), v.map(|v| quantize(&normalized(v))))).collect()
            } else {
                let m = if let Some(m) = &model {
                    Arc::clone(m)
                } else {
                    let d = dir.clone().context("image search's model went")?;
                    let m = Arc::new(crate::heavy::run("clip-load", move || siglip::VisionModel::load(&d)).await??);
                    Arc::clone(model.insert(m))
                };
                crate::heavy::run("clip", move || in_pool(|| embed_all(&m, &pictures))).await??
            };
            save(state, &computed, &id).await?;
            done += chunk.len();
            state.pictures.write(|m| {
                for (hash, v) in computed {
                    if let Some(e) = v.and_then(Entry::new) {
                        m.insert(hash, e);
                    }
                }
            });
        }
        crate::activity::job_progress(crate::jobs::EMBED, None, done, total, "pictures");
    }
}

fn embed_all(
    model: &siglip::VisionModel,
    pictures: &[(String, bytes::Bytes)],
) -> Result<Vec<(String, Option<Vec<i8>>)>> {
    use rayon::prelude::*;
    let prepared: Vec<(String, Option<Vec<f32>>)> =
        pictures.par_iter().map(|(h, b)| (h.clone(), siglip::pixels(b))).collect();
    let inputs: Vec<Vec<f32>> = prepared.iter().filter_map(|(_, p)| p.clone()).collect();
    let mut embedded = model.embed(&inputs)?.into_iter();
    Ok(prepared.into_iter().map(|(h, p)| (h, p.and_then(|_| embedded.next()).map(|v| quantize(&v)))).collect())
}

async fn save(state: &AppState, computed: &[(String, Option<Vec<i8>>)], model: &str) -> Result<()> {
    let rows: Vec<(String, Vec<u8>)> = computed
        .iter()
        // Not a picture after all: an empty row, so it isn't tried again.
        .map(|(h, v)| (h.clone(), v.as_ref().map_or_default(|v| v.iter().map(|&x| x as u8).collect())))
        .collect();
    let model = model.to_owned();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        {
            let mut stmt =
                tx.prepare("INSERT OR REPLACE INTO embeddings (thumb_hash, model, vector) VALUES (?1, ?2, ?3)")?;
            for (hash, v) in &rows {
                stmt.execute(params![hash, model, v])?;
            }
        }
        tx.commit()?;
        Ok(())
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantized_vectors_keep_their_cosine() {
        let a = normalized((0..CLIP_DIMS).map(|i| ((i * 7) % 13) as f32 - 6.0).collect());
        let b = normalized((0..CLIP_DIMS).map(|i| ((i * 5) % 11) as f32 - 5.0 + (i % 3) as f32).collect());
        let exact: f32 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
        let (qa, qb) = (Entry::new(quantize(&a)).unwrap(), Entry::new(quantize(&b)).unwrap());
        assert!((qa.cosine(&qb) - exact).abs() < 0.01, "{} vs {exact}", qa.cosine(&qb));
        assert!((qa.cosine_f32(&b) - exact).abs() < 0.01);
        assert!((qa.cosine(&qa) - 1.0).abs() < 1e-4);
        assert!(Entry::new(vec![0; CLIP_DIMS]).is_none(), "an unreadable thumbnail's empty vector");
        assert!(Entry::new(Vec::new()).is_none());
    }

    #[test]
    fn text_matches_stand_above_the_median() {
        // One strong match, one borderline, the rest typical.
        let index = Index::new(&CLIP_SPEC);
        let entry = |x: f32| {
            Entry::new(quantize(
                &normalized(vec![x, 1.0 - x]).into_iter().chain([0.0; CLIP_DIMS - 2]).collect::<Vec<_>>(),
            ))
            .unwrap()
        };
        let q = normalized([1.0f32, 0.0].into_iter().chain([0.0; CLIP_DIMS - 2]).collect());
        let mut m: HashMap<String, Entry> = (0..20).map(|i| (format!("t{i}"), entry(0.3))).collect();
        m.insert("strong".into(), entry(0.6));
        m.insert("faint".into(), entry(0.32));
        *index.map.write().unwrap() = Some(m);
        let hits = index.matching(&q);
        assert_eq!(hits.len(), 1, "{hits:?}");
        assert_eq!(hits[0].0, "strong");
        assert!(hits[0].1 > 0.25, "a strong text match beats a name match alone");
    }

    #[test]
    fn a_hosted_model_is_judged_by_each_search_s_spread() {
        let entry = |x: f32| Entry::new(quantize(&normalized(vec![x, 1.0 - x, 0.1]))).unwrap();
        let q = [1.0f32, 0.0, 0.0];
        // Whatever level a model's scores sit at, what stands far above the rest is a match and the rest isn't.
        for base in [0.2f32, 0.5] {
            let index = Index::new(hosted_spec("test:model"));
            let mut m: HashMap<String, Entry> =
                (0..60).map(|i| (format!("t{i}"), entry(base + (i % 10) as f32 * 0.004))).collect();
            m.insert("strong".into(), entry(base + 0.2));
            *index.map.write().unwrap() = Some(m);
            let hits = index.matching(&q);
            assert_eq!(hits.iter().map(|h| h.0.as_str()).collect::<Vec<_>>(), ["strong"], "{base}: {hits:?}");
            assert!(hits[0].1 > 0.25);
            // More like this: measured against the typical picture's score for this one.
            let like = index.similar("t0").unwrap();
            assert!((like["t0"] - 1.0).abs() < 1e-3);
            assert!(like["t1"] > 0.5 && like["strong"] < 0.1, "{like:?}");
        }
        assert!(std::ptr::eq(hosted_spec("test:model"), hosted_spec("test:model")), "made once per model");
        let index = Index::new(&SPEC);
        index.respec(hosted_spec("test:other"));
        assert_eq!(index.spec().model, "test:other");
    }
}
