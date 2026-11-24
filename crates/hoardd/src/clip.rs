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

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use anyhow::{Context, Result, ensure};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_nn::{Embedding, LayerNorm, Linear, VarBuilder};

use crate::embed::{
    Entry, Index, Model, Spec, Table, Text, files_present, in_pool, normalized, quantize, read_tensors,
};
use crate::tools::{Slot, Status};
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

// ---------------------------------------------------------------------------------------------
// Model files and status

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

/// How far above the median picture a match must score. Words score far lower against pictures than pictures do
/// against each other: on a mixed library, a true match stands 0.05–0.1 above the median, and words with nothing
/// to find (a car among rocks and furniture) peak around 0.03.
const CLIP_LIFT: f32 = 0.045;
/// Keep only what's within this of the best match.
const CLIP_MARGIN: f32 = 0.04;

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

/// Runs `f` with how much a thumbnail shows what `hash`'s does, about 0..=1, when image search is on and `hash` has
/// an embedding (see [`Index::similar`]); with `None` otherwise.
pub(crate) async fn similar<R>(
    state: &AppState,
    hash: &str,
    f: impl FnOnce(Option<crate::embed::Likeness<'_>>) -> R,
) -> R {
    let Some(index) = index(state) else { return f(None) };
    if let Err(e) = index.ensure(state).await {
        tracing::warn!(error = %format!("{e:#}"), "couldn't load image search's index");
        return f(None);
    }
    index.similar(hash, f)
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

/// Embeddings are per thumbnail, of what has a look.
static TABLE: Table = Table {
    name: "embeddings",
    key: "thumb_hash",
    asset_key: "thumb_hash",
    kinds: crate::looks::KINDS_SQL,
    orphan: "NOT EXISTS (SELECT 1 FROM assets a WHERE a.thumb_hash = e.thumb_hash)",
    job: crate::jobs::EMBED,
};

/// How many thumbnails have an embedding, and how many could, for the Settings page.
pub(crate) fn progress(conn: &rusqlite::Connection) -> Result<(u64, u64)> {
    TABLE.progress(conn, &model_id())
}

/// Queue an embedding job when image search is on and some thumbnail has no embedding yet.
pub(crate) async fn schedule(state: &AppState) {
    if !is_ready() {
        return;
    }
    match TABLE.queue(state, model_id()).await {
        Ok(true) => {}
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

/// Embed every thumbnail that has no embedding, and forget the ones nothing uses.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    let (remote, dir) = (hosted(), SLOT.ready());
    if remote.is_none() && dir.is_none() {
        return Ok(());
    }
    let id = model_id();
    state.pictures.ensure(state).await?;
    let gone = TABLE.forget_orphans(state).await?;
    for index in [&state.pictures, &state.clip] {
        index.write(|m| {
            for h in &gone {
                m.remove(h);
            }
        });
    }

    let mut model: Option<Arc<siglip::VisionModel>> = None;
    let mut unreadable = std::collections::HashSet::new();
    let mut done = 0usize;
    loop {
        let (skip, want) = (unreadable.clone(), id.clone());
        let todo: Vec<String> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT DISTINCT a.thumb_hash FROM assets a WHERE {} AND {}",
                    TABLE.wanted(),
                    TABLE.missing()
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
            TABLE.save(state, &id, &computed).await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
        index.set(m);
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
            index.set(m);
            let hits = index.matching(&q);
            assert_eq!(hits.iter().map(|h| h.0.as_str()).collect::<Vec<_>>(), ["strong"], "{base}: {hits:?}");
            assert!(hits[0].1 > 0.25);
            // More like this: measured against the typical picture's score for this one.
            let like = index.similar("t0", |s| ["t0", "t1", "strong", "none"].map(|h| s.unwrap()(h)));
            let [Some(t0), Some(t1), Some(strong), None] = like else { panic!("{like:?}") };
            assert!((t0 - 1.0).abs() < 1e-3);
            assert!(t1 > 0.5 && strong < 0.1, "{like:?}");
            assert!(index.similar("none", |s| s.is_none()), "no embedding, no scores");
        }
        assert!(std::ptr::eq(hosted_spec("test:model"), hosted_spec("test:model")), "made once per model");
        let index = Index::new(&SPEC);
        index.respec(hosted_spec("test:other"));
        assert_eq!(index.spec().model, "test:other");
    }
}
