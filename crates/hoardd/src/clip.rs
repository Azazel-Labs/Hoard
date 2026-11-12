//! Search by meaning with CLIP (`OpenAI`'s ViT-B/32): every thumbnail gets a 512-number embedding, and so does what's
//! typed in the search box, so "mossy rock" finds mossy rocks whatever the files are called, and "more like this"
//! knows a barrel from a crate rather than only matching colours and grain.
//!
//! It's off until an admin turns it on in Settings, because the model is a 605 MB download and takes 250–350 MB of
//! memory while it runs. Turned on, the model is fetched (pinned to one revision, checked against its SHA-256) into
//! `<cache>/models`, and a background job works through the thumbnails at low priority. The text half is loaded on
//! the first search and dropped after a while without one; the image half only lives while the job runs.
//!
//! Embeddings are stored per thumbnail blob (`embeddings`, keyed by its hash) as 512 signed bytes, scaled per
//! vector, which cosine similarity doesn't mind. Searching is brute force over an in-memory index, like `looks`.

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex, PoisonError, RwLock};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, ensure};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_nn::{Conv2d, Embedding, LayerNorm, Linear, VarBuilder};
use rusqlite::params;
use serde::Deserialize;

use crate::tools::{self, Slot, Status};
use crate::{AppState, db};

/// Stored with each embedding; rows from another model are redone.
pub(crate) const MODEL: &str = "clip-vit-b32";

/// Where the files come from: one revision of the model, pinned, so a change upstream can't change what's run.
const REPO: &str =
    "https://huggingface.co/openai/clip-vit-base-patch32/resolve/b33cedfd0df4e43b8238760678fcc89e1a0d38b3";
const WEIGHTS: &str = "model.safetensors";
const TOKENIZER: &str = "tokenizer.json";
/// Name, size and SHA-256 of each file.
const FILES: [(&str, u64, &str); 2] = [
    (TOKENIZER, 2_224_041, "b556ac8c99757ffb677208af34bc8c6721572114111a6e0aaf5fa69ff0b8d842"),
    (WEIGHTS, 605_157_884, "99d28a652e6ec46629ab7047a0ac82c69b1fe11e0ce672c43af65d3a9a3fc05d"),
];
/// The whole download, for the Settings page.
pub(crate) const DOWNLOAD_BYTES: u64 = FILES[0].1 + FILES[1].1;

pub(crate) const DIMS: usize = 512;

// ---------------------------------------------------------------------------------------------
// The model (adapted from candle-transformers' CLIP, MIT/Apache-2.0)

const IMAGE: usize = 224;
const PATCH: usize = 32;
const POSITIONS: usize = (IMAGE / PATCH) * (IMAGE / PATCH) + 1;
const MAX_TOKENS: usize = 77;
const VOCAB: usize = 49_408;
const EOS: u32 = 49_407;
const MEAN: [f32; 3] = [0.481_454_66, 0.457_827_5, 0.408_210_73];
const STD: [f32; 3] = [0.268_629_54, 0.261_302_6, 0.275_777_1];

fn quick_gelu(xs: &Tensor) -> candle_core::Result<Tensor> {
    xs * candle_nn::ops::sigmoid(&(xs * 1.702)?)?
}

#[derive(Debug)]
struct Attention {
    q: Linear,
    k: Linear,
    v: Linear,
    out: Linear,
    heads: usize,
}

impl Attention {
    fn new(vb: &VarBuilder, dim: usize, heads: usize) -> candle_core::Result<Self> {
        let l = |name: &str| candle_nn::linear(dim, dim, vb.pp(name));
        Ok(Self { q: l("q_proj")?, k: l("k_proj")?, v: l("v_proj")?, out: l("out_proj")?, heads })
    }

    #[expect(clippy::many_single_char_names, reason = "the usual names in attention: batch, tokens, q, k, v, w, o")]
    fn forward(&self, xs: &Tensor, mask: Option<&Tensor>) -> candle_core::Result<Tensor> {
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
            projection: candle_nn::linear_no_bias(512, DIMS, vb.pp("text_projection"))?,
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

/// The image half: 224×224 pictures in, embeddings out.
#[derive(Debug)]
struct VisionModel {
    patches: Conv2d,
    class: Tensor,
    positions: Tensor,
    pre_ln: LayerNorm,
    layers: Vec<Layer>,
    post_ln: LayerNorm,
    projection: Linear,
}

impl VisionModel {
    fn load(dir: &Path) -> Result<Self> {
        let vb = VarBuilder::from_tensors(
            read_tensors(&dir.join(WEIGHTS), &["vision_model.", "visual_projection."])?,
            DType::F32,
            &Device::Cpu,
        );
        let v = vb.pp("vision_model");
        let conv = candle_nn::Conv2dConfig { stride: PATCH, ..Default::default() };
        Ok(Self {
            patches: candle_nn::conv2d_no_bias(3, 768, PATCH, conv, v.pp("embeddings.patch_embedding"))?,
            class: v.pp("embeddings").get(768, "class_embedding")?,
            positions: v.pp("embeddings.position_embedding").get((POSITIONS, 768), "weight")?,
            pre_ln: candle_nn::layer_norm(768, 1e-5, v.pp("pre_layrnorm"))?,
            layers: encoder(&v, 768, 12, 3072)?,
            post_ln: candle_nn::layer_norm(768, 1e-5, v.pp("post_layernorm"))?,
            projection: candle_nn::linear_no_bias(768, DIMS, vb.pp("visual_projection"))?,
        })
    }

    /// Embeddings of pictures made by [`pixels`].
    fn embed(&self, pictures: &[Vec<f32>]) -> Result<Vec<Vec<f32>>> {
        let b = pictures.len();
        if b == 0 {
            return Ok(Vec::new());
        }
        let flat: Vec<f32> = pictures.iter().flatten().copied().collect();
        let input = Tensor::from_vec(flat, (b, 3, IMAGE, IMAGE), &Device::Cpu)?;
        let patches = self.patches.forward(&input)?.flatten_from(2)?.transpose(1, 2)?;
        let class = self.class.reshape((1, 1, 768))?.broadcast_as((b, 1, 768))?.contiguous()?;
        let mut xs = Tensor::cat(&[&class, &patches], 1)?.broadcast_add(&self.positions)?;
        xs = self.pre_ln.forward(&xs)?;
        for layer in &self.layers {
            xs = layer.forward(&xs, None)?;
        }
        let pooled = self.post_ln.forward(&xs.i((.., 0, ..))?)?;
        let out = self.projection.forward(&pooled)?.to_vec2::<f32>()?;
        Ok(out.into_iter().map(normalized).collect())
    }
}

/// The tensors whose names start with one of `prefixes`, read from a safetensors file one at a time (so loading half
/// the model never holds the other half, or the whole file, in memory).
fn read_tensors(path: &Path, prefixes: &[&str]) -> Result<HashMap<String, Tensor>> {
    #[derive(Deserialize)]
    struct Info {
        dtype: String,
        shape: Vec<usize>,
        data_offsets: [u64; 2],
    }
    let mut file = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut len = [0u8; 8];
    file.read_exact(&mut len)?;
    let len = u64::from_le_bytes(len);
    ensure!(len < 16 << 20, "the model file's header is unexpectedly large");
    let mut header = vec![0u8; len as usize];
    file.read_exact(&mut header)?;
    let header: HashMap<String, serde_json::Value> = serde_json::from_slice(&header)?;
    let mut out = HashMap::new();
    for (name, info) in header {
        if !prefixes.iter().any(|p| name.starts_with(p)) {
            continue;
        }
        let info: Info = serde_json::from_value(info).with_context(|| format!("tensor {name}"))?;
        // Position ids (I64) are buffers the model doesn't need; every weight is F32.
        if info.dtype != "F32" {
            continue;
        }
        let [start, end] = info.data_offsets;
        let count: usize = info.shape.iter().product();
        ensure!(end >= start && (end - start) as usize == count * 4, "tensor {name} has the wrong size");
        file.seek(SeekFrom::Start(8 + len + start))?;
        let mut bytes = vec![0u8; count * 4];
        file.read_exact(&mut bytes)?;
        let values: Vec<f32> = bytes.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)).collect();
        out.insert(name, Tensor::from_vec(values, info.shape, &Device::Cpu)?);
    }
    ensure!(!out.is_empty(), "the model file has none of the tensors needed");
    Ok(out)
}

fn normalized(mut v: Vec<f32>) -> Vec<f32> {
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut v {
            *x /= norm;
        }
    }
    v
}

/// A thumbnail as the model wants it: transparency over white, the short side scaled to 224 (bicubic), the middle
/// 224×224, normalized per channel, channels first. `None` when it can't be decoded or is all transparent.
pub(crate) fn pixels(bytes: &[u8]) -> Option<Vec<f32>> {
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 || img.pixels().all(|p| p[3] < 8) {
        return None;
    }
    let rgb = image::RgbImage::from_fn(w, h, |x, y| {
        let p = img.get_pixel(x, y);
        let a = f32::from(p[3]) / 255.0;
        image::Rgb([0, 1, 2].map(|i| (f32::from(p[i]) * a + 255.0 * (1.0 - a)).round() as u8))
    });
    let side = IMAGE as f32 / w.min(h) as f32;
    let (nw, nh) =
        (((w as f32 * side).round() as u32).max(IMAGE as u32), ((h as f32 * side).round() as u32).max(IMAGE as u32));
    let resized = image::imageops::resize(&rgb, nw, nh, image::imageops::FilterType::CatmullRom);
    let (x0, y0) = ((nw as usize - IMAGE) / 2, (nh as usize - IMAGE) / 2);
    let mut out = vec![0.0f32; 3 * IMAGE * IMAGE];
    for y in 0..IMAGE {
        for x in 0..IMAGE {
            let p = resized.get_pixel((x0 + x) as u32, (y0 + y) as u32);
            for c in 0..3 {
                out[c * IMAGE * IMAGE + y * IMAGE + x] = (f32::from(p[c]) / 255.0 - MEAN[c]) / STD[c];
            }
        }
    }
    Some(out)
}

/// 512 signed bytes scaled so the largest is ±127.
pub(crate) fn quantize(v: &[f32]) -> Vec<i8> {
    let max = v.iter().fold(0.0f32, |m, x| m.max(x.abs()));
    let k = if max > 0.0 { 127.0 / max } else { 0.0 };
    v.iter().map(|x| (x * k).round().clamp(-127.0, 127.0) as i8).collect()
}

// ---------------------------------------------------------------------------------------------
// Model files and status

static SLOT: Slot<PathBuf> = Slot::new();

/// Where the model is, once it's downloaded and checked.
fn ready_dir() -> Option<PathBuf> {
    SLOT.ready()
}

pub(crate) fn is_ready() -> bool {
    ready_dir().is_some()
}

/// `off`, `looking`, `downloading`, `ready` or `unavailable: <why>`.
pub(crate) fn describe() -> String {
    SLOT.describe(|_| "ready".into())
}

fn model_dir(state: &AppState) -> PathBuf {
    state.cfg.models_dir().join(MODEL)
}

/// Turn image search on (find the model, downloading it if need be) or off. Calling it again drops whatever the
/// last call was doing.
pub(crate) fn start(state: &AppState, enabled: bool) {
    *TEXT.lock().unwrap_or_else(PoisonError::into_inner) = None;
    if !enabled {
        SLOT.restart(Status::Off);
        return;
    }
    let generation = SLOT.restart(Status::Looking);
    let st = Arc::clone(state);
    tokio::spawn(async move {
        let dir = model_dir(&st);
        loop {
            if files_present(&dir) {
                if SLOT.set(generation, Status::Ready(dir.clone())) {
                    tracing::info!(dir = %dir.display(), "image search model ready");
                    spawn_unloader();
                    schedule(&st).await;
                }
                return;
            }
            if !SLOT.set(generation, Status::Downloading) {
                return;
            }
            match download(&dir, &|| !SLOT.current(generation)).await {
                Ok(()) => {}
                Err(e) => {
                    if !SLOT.current(generation) {
                        return;
                    }
                    let msg = format!("{e:#}");
                    tracing::warn!(error = %msg, "couldn't download the image search model; retrying in an hour");
                    SLOT.set(generation, Status::Unavailable(msg));
                    tokio::time::sleep(Duration::from_secs(3600)).await;
                    if !SLOT.current(generation) {
                        return;
                    }
                }
            }
        }
    });
}

/// Image search was switched off: the model goes (the embeddings stay, for when it's back on).
pub(crate) async fn remove_files(state: &AppState) {
    let dir = model_dir(state);
    if dir.is_dir()
        && let Err(e) = tokio::fs::remove_dir_all(&dir).await
    {
        tracing::warn!(dir = %dir.display(), error = %e, "couldn't remove the image search model");
    }
}

/// Both files, at their sizes (their hashes were checked when they were downloaded).
fn files_present(dir: &Path) -> bool {
    FILES.iter().all(|(name, size, _)| std::fs::metadata(dir.join(name)).is_ok_and(|m| m.len() == *size))
}

async fn download(dir: &Path, cancelled: &(dyn Fn() -> bool + Sync)) -> Result<()> {
    let http = tools::client()?;
    tokio::fs::create_dir_all(dir).await?;
    let activity = crate::activity::start("clip", "Downloading the image search model", None);
    for (name, size, sha256) in FILES {
        let dest = dir.join(name);
        if std::fs::metadata(&dest).is_ok_and(|m| m.len() == size) {
            continue;
        }
        activity.detail(name);
        let part = dir.join(format!("{name}.part"));
        let fetched = tools::fetch(&http, &format!("{REPO}/{name}"), &part, size, &activity, cancelled)
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

// ---------------------------------------------------------------------------------------------
// Text embeddings for searches

/// How long the text half stays loaded after the last search.
const TEXT_IDLE: Duration = Duration::from_mins(15);

#[derive(Debug)]
struct Loaded {
    model: Arc<TextModel>,
    used: Instant,
    /// Recent searches' embeddings, as people scroll through the same results.
    recent: HashMap<String, Arc<Vec<f32>>>,
}

static TEXT: Mutex<Option<Loaded>> = Mutex::new(None);

/// Drop the text half once nobody has searched for a while.
fn spawn_unloader() {
    static STARTED: std::sync::Once = std::sync::Once::new();
    STARTED.call_once(|| {
        tokio::spawn(async {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let mut guard = TEXT.lock().unwrap_or_else(PoisonError::into_inner);
                if guard.as_ref().is_some_and(|l| l.used.elapsed() > TEXT_IDLE) {
                    *guard = None;
                    tracing::debug!("image search text model unloaded");
                }
            }
        });
    });
}

/// The embedding of what was typed, or `None` while the model isn't ready.
async fn embed_text(text: &str) -> Result<Option<Arc<Vec<f32>>>> {
    let Some(dir) = ready_dir() else { return Ok(None) };
    let key = text.trim().to_lowercase();
    let model = {
        let mut guard = TEXT.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(l) = guard.as_mut() {
            l.used = Instant::now();
            if let Some(v) = l.recent.get(&key) {
                return Ok(Some(Arc::clone(v)));
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
        let m = Arc::new(tokio::task::spawn_blocking(move || TextModel::load(&dir)).await??);
        tracing::info!(ms = started.elapsed().as_millis(), "image search text model loaded");
        m
    };
    let m = Arc::clone(&model);
    let k = key.clone();
    // "a photo of …" matches pictures a little better than the bare words.
    let v = Arc::new(tokio::task::spawn_blocking(move || m.embed(&format!("a photo of {k}"))).await??);
    let mut guard = TEXT.lock().unwrap_or_else(PoisonError::into_inner);
    let loaded = guard.get_or_insert_with(|| Loaded { model, used: Instant::now(), recent: HashMap::new() });
    if loaded.recent.len() >= 64 {
        loaded.recent.clear();
    }
    loaded.recent.insert(key, Arc::clone(&v));
    Ok(Some(v))
}

// ---------------------------------------------------------------------------------------------
// Index and ranking

#[derive(Debug)]
struct Entry {
    v: Box<[i8]>,
    /// 1 / the vector's length, so a dot product becomes a cosine.
    inv_norm: f32,
}

impl Entry {
    fn new(v: Vec<i8>) -> Option<Self> {
        let norm = v.iter().map(|&x| f32::from(x) * f32::from(x)).sum::<f32>().sqrt();
        (v.len() == DIMS && norm > 0.0).then(|| Self { v: v.into_boxed_slice(), inv_norm: 1.0 / norm })
    }

    fn cosine(&self, other: &Self) -> f32 {
        let dot: i32 = self.v.iter().zip(other.v.iter()).map(|(a, b)| i32::from(*a) * i32::from(*b)).sum();
        dot as f32 * self.inv_norm * other.inv_norm
    }

    fn cosine_f32(&self, q: &[f32]) -> f32 {
        self.v.iter().zip(q).map(|(a, b)| f32::from(*a) * b).sum::<f32>() * self.inv_norm
    }
}

/// Every thumbnail's embedding, in memory, loaded from `embeddings` on first use and kept up to date by the job.
#[derive(Debug, Default)]
pub(crate) struct Index(RwLock<Option<HashMap<String, Entry>>>);

impl Index {
    fn read<R>(&self, f: impl FnOnce(&HashMap<String, Entry>) -> R) -> Option<R> {
        self.0.read().unwrap_or_else(PoisonError::into_inner).as_ref().map(f)
    }

    fn write(&self, f: impl FnOnce(&mut HashMap<String, Entry>)) {
        if let Some(m) = self.0.write().unwrap_or_else(PoisonError::into_inner).as_mut() {
            f(m);
        }
    }

    pub(crate) async fn ensure(&self, state: &AppState) -> Result<()> {
        if self.read(|_| ()).is_some() {
            return Ok(());
        }
        let loaded = db::call(&state.pool, |c| {
            let mut stmt = c.prepare("SELECT thumb_hash, vector FROM embeddings WHERE model = ?1")?;
            let rows = stmt
                .query_map([MODEL], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?)))?
                .filter_map(|row| {
                    let (hash, v) = row.ok()?;
                    Some((hash, Entry::new(v.into_iter().map(|b| b as i8).collect())?))
                })
                .collect::<HashMap<_, _>>();
            Ok(rows)
        })
        .await?;
        let mut guard = self.0.write().unwrap_or_else(PoisonError::into_inner);
        if guard.is_none() {
            tracing::debug!(embeddings = loaded.len(), "image search index loaded");
            *guard = Some(loaded);
        }
        Ok(())
    }

    fn len(&self) -> usize {
        self.read(HashMap::len).unwrap_or(0)
    }

    /// How much every thumbnail looks like `hash`'s by meaning, about 0..=1, or `None` when it has no embedding.
    pub(crate) fn similar(&self, hash: &str) -> Option<HashMap<String, f32>> {
        self.read(|m| {
            let target = m.get(hash)?;
            Some(m.iter().map(|(h, e)| (h.clone(), image_score(target.cosine(e)))).collect())
        })
        .flatten()
    }

    /// Thumbnails that match a text embedding, best first, scored about 0..=0.5.
    fn matching(&self, q: &[f32]) -> Vec<(String, f32)> {
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
            let best = scored.iter().map(|s| s.1).fold(f32::MIN, f32::max);
            let floor = (median + TEXT_LIFT).max(best - TEXT_MARGIN);
            let mut hits: Vec<(&String, f32)> = scored.into_iter().filter(|s| s.1 >= floor).collect();
            hits.sort_by(|a, b| b.1.total_cmp(&a.1));
            hits.truncate(TEXT_MAX);
            hits.into_iter().map(|(h, c)| (h.clone(), text_score(c - median))).collect()
        })
        .unwrap_or_default()
    }
}

/// Two pictures' cosine (unrelated ones sit around 0.5, the same thing from another angle near 0.9) as 0..=1.
fn image_score(cosine: f32) -> f32 {
    ((cosine - 0.5) / 0.45).clamp(0.0, 1.0)
}

/// How far above the median picture a match must score. Words score far lower against pictures than pictures do
/// against each other: on a mixed library, a true match stands 0.05–0.1 above the median, and words with nothing
/// to find (a car among rocks and furniture) peak around 0.03.
const TEXT_LIFT: f32 = 0.045;
/// Keep only what's within this of the best match.
const TEXT_MARGIN: f32 = 0.04;
const TEXT_MAX: usize = 300;

/// A text match as a rank score, from how far it stands above the median: a strong one (0.1 up) counts double a
/// word match in the name (0.25), a borderline one well under it.
fn text_score(lift: f32) -> f32 {
    ((lift - 0.03) / 0.07).clamp(0.0, 1.0) * 0.5
}

/// How much every thumbnail shows what `hash`'s does, about 0..=1, when image search is on and it has an embedding.
pub(crate) async fn similar(state: &AppState, hash: &str) -> Option<HashMap<String, f32>> {
    if !is_ready() {
        return None;
    }
    if let Err(e) = state.clip.ensure(state).await {
        tracing::warn!(error = %format!("{e:#}"), "couldn't load image search's index");
        return None;
    }
    state.clip.similar(hash)
}

/// Thumbnails that match what was typed, best first; empty when image search is off or has nothing yet.
pub(crate) async fn search_text(state: &AppState, text: &str) -> Result<Vec<(String, f32)>> {
    if !is_ready() || text.trim().is_empty() {
        return Ok(Vec::new());
    }
    state.clip.ensure(state).await?;
    if state.clip.len() == 0 {
        return Ok(Vec::new());
    }
    let Some(q) = embed_text(text).await? else { return Ok(Vec::new()) };
    Ok(state.clip.matching(&q))
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
        [MODEL],
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
            [MODEL],
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
        Ok(false) => {}
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

fn in_pool<T: Send>(f: impl FnOnce() -> T + Send) -> T {
    match POOL.as_ref() {
        Some(pool) => pool.install(f),
        None => f(),
    }
}

/// Embed every thumbnail that has no embedding, and forget the ones nothing uses.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    let Some(dir) = ready_dir() else { return Ok(()) };
    state.clip.ensure(state).await?;
    let gone: Vec<String> = db::call(&state.pool, |c| {
        let gone = c
            .prepare(&format!("SELECT thumb_hash FROM embeddings e WHERE {ORPHAN_SQL}"))?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        c.execute(&format!("DELETE FROM embeddings AS e WHERE {ORPHAN_SQL}"), [])?;
        Ok(gone)
    })
    .await?;
    state.clip.write(|m| {
        for h in &gone {
            m.remove(h);
        }
    });

    let kinds = crate::looks::KINDS_SQL;
    let mut model: Option<Arc<VisionModel>> = None;
    let mut unreadable = std::collections::HashSet::new();
    let mut done = 0usize;
    loop {
        let skip = unreadable.clone();
        let todo: Vec<String> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT DISTINCT a.thumb_hash FROM assets a
                      WHERE a.thumb_hash IS NOT NULL AND {kinds} AND {MISSING_SQL}"
                ))?
                .query_map([MODEL], |r| r.get(0))?
                .collect::<rusqlite::Result<Vec<String>>>()?;
            Ok(rows.into_iter().filter(|h| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return Ok(());
        }
        let model = if let Some(m) = &model {
            Arc::clone(m)
        } else {
            let d = dir.clone();
            let m = Arc::new(crate::heavy::run("clip-load", move || VisionModel::load(&d)).await??);
            Arc::clone(model.insert(m))
        };
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            // Switched off partway through: stop, keeping what's done.
            if !is_ready() {
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
            let m = Arc::clone(&model);
            let computed = crate::heavy::run("clip", move || in_pool(|| embed_all(&m, &pictures))).await??;
            save(state, &computed).await?;
            done += chunk.len();
            state.clip.write(|m| {
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

fn embed_all(model: &VisionModel, pictures: &[(String, bytes::Bytes)]) -> Result<Vec<(String, Option<Vec<i8>>)>> {
    use rayon::prelude::*;
    let prepared: Vec<(String, Option<Vec<f32>>)> = pictures.par_iter().map(|(h, b)| (h.clone(), pixels(b))).collect();
    let inputs: Vec<Vec<f32>> = prepared.iter().filter_map(|(_, p)| p.clone()).collect();
    let mut embedded = model.embed(&inputs)?.into_iter();
    Ok(prepared.into_iter().map(|(h, p)| (h, p.and_then(|_| embedded.next()).map(|v| quantize(&v)))).collect())
}

async fn save(state: &AppState, computed: &[(String, Option<Vec<i8>>)]) -> Result<()> {
    let rows: Vec<(String, Vec<u8>)> = computed
        .iter()
        // Not a picture after all: an empty row, so it isn't tried again.
        .map(|(h, v)| (h.clone(), v.as_ref().map(|v| v.iter().map(|&x| x as u8).collect()).unwrap_or_default()))
        .collect();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        {
            let mut stmt =
                tx.prepare("INSERT OR REPLACE INTO embeddings (thumb_hash, model, vector) VALUES (?1, ?2, ?3)")?;
            for (hash, v) in &rows {
                stmt.execute(params![hash, MODEL, v])?;
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

    fn png(w: u32, h: u32, f: impl Fn(u32, u32) -> image::Rgba<u8>) -> Vec<u8> {
        let img = image::RgbaImage::from_fn(w, h, f);
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }

    #[test]
    fn pixels_are_cropped_normalized_and_over_white() {
        // Wide: left half red, right half transparent. The crop is the middle, so both show.
        let bytes =
            png(512, 256, |x, _| if x < 256 { image::Rgba([255, 0, 0, 255]) } else { image::Rgba([0, 0, 0, 0]) });
        let p = pixels(&bytes).unwrap();
        assert_eq!(p.len(), 3 * IMAGE * IMAGE);
        let at = |c: usize, x: usize, y: usize| p[c * IMAGE * IMAGE + y * IMAGE + x];
        let norm = |c: usize, v: f32| (v - MEAN[c]) / STD[c];
        assert!((at(0, 0, 100) - norm(0, 1.0)).abs() < 1e-3, "red on the left");
        assert!((at(1, 0, 100) - norm(1, 0.0)).abs() < 1e-3);
        assert!((at(1, IMAGE - 1, 100) - norm(1, 1.0)).abs() < 1e-3, "white where it was transparent");
        assert!(pixels(&png(8, 8, |_, _| image::Rgba([0, 0, 0, 0]))).is_none(), "all transparent");
        assert!(pixels(b"not a picture").is_none());
    }

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

    #[test]
    fn scores() {
        assert!(image_score(0.5).abs() < 1e-6);
        assert!((image_score(0.95) - 1.0).abs() < 1e-6);
        assert!(text_score(0.1) > 0.25, "a strong text match beats a name match alone");
        assert!(text_score(TEXT_LIFT) < 0.25, "a borderline one doesn't");
        assert!(text_score(0.02).abs() < 1e-6);
    }

    /// The real model, when it's been downloaded: `HOARD_CLIP_DIR=<folder with the two files> cargo test -- --ignored`.
    #[test]
    #[ignore = "needs the 605 MB model"]
    fn real_model_matches_words_to_pictures() {
        let dir = PathBuf::from(std::env::var("HOARD_CLIP_DIR").expect("HOARD_CLIP_DIR"));
        let text = TextModel::load(&dir).unwrap();
        assert_eq!(tokens(&text.tokenizer, "a photo of a cat").unwrap(), [49406, 320, 1125, 539, 320, 2368, 49407]);
        let vision = VisionModel::load(&dir).unwrap();
        let red = png(256, 256, |_, _| image::Rgba([220, 20, 20, 255]));
        let stripes =
            png(256, 256, |x, _| if (x / 16) % 2 == 0 { image::Rgba([0, 0, 0, 255]) } else { image::Rgba([255; 4]) });
        let started = Instant::now();
        let pics = vision.embed(&[pixels(&red).unwrap(), pixels(&stripes).unwrap()]).unwrap();
        eprintln!("2 pictures in {:?}", started.elapsed());
        let cos = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
        let (t_red, t_stripes) = (text.embed("a red square").unwrap(), text.embed("black and white stripes").unwrap());
        eprintln!(
            "red: {:.3} / {:.3}, stripes: {:.3} / {:.3}",
            cos(&pics[0], &t_red),
            cos(&pics[0], &t_stripes),
            cos(&pics[1], &t_stripes),
            cos(&pics[1], &t_red)
        );
        assert!(cos(&pics[0], &t_red) > cos(&pics[0], &t_stripes));
        assert!(cos(&pics[1], &t_stripes) > cos(&pics[1], &t_red));
    }
}
