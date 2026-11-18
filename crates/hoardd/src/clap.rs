//! Search sounds by what they sound like, in words, with CLAP (LAION's `clap-htsat-unfused`): every sound gets a
//! 512-number embedding, and so does what's typed in the search box, so "glassy UI blip" or "distant thunder roll"
//! finds sounds whatever their files are called. It is to sounds what [`crate::clip`] is to pictures.
//!
//! It's off until an admin turns it on in Settings, because the model is a 614 MB download. Turned on, the model is
//! fetched (pinned to one revision, checked against its SHA-256) into `<cache>/models`, and a background job listens
//! to every sound at low priority: up to its first ten seconds, silence at the ends left out, at 48 kHz, as a log-mel
//! spectrogram. The text half is loaded on the first search and dropped after a while without one; the audio half
//! only lives while the job runs.
//!
//! Embeddings are stored per sound file (`sound_embeddings`, keyed by its hash) as 512 signed bytes, scaled per
//! vector, and searched brute force from memory.

use std::f64::consts::PI;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, ensure};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_nn::{Conv2d, Embedding, LayerNorm, Linear, VarBuilder};
use rusqlite::params;

use crate::clip::{Entry, Model, Spec, Text, normalized, quantize, read_tensors};
use crate::tools::{Slot, Status};
use crate::{AppState, db};

/// Stored with each embedding; rows from another model are redone.
pub(crate) const MODEL: &str = "clap-htsat-unfused";

/// Where the files come from: one revision of the model (the safetensors conversion of LAION's weights), pinned.
const REPO: &str = "https://huggingface.co/laion/clap-htsat-unfused/resolve/79b58ed25fc00386262a2bea4b19fd21dc4310a0";
const WEIGHTS: &str = "model.safetensors";
const TOKENIZER: &str = "tokenizer.json";
/// Name, size and SHA-256 of each file.
const FILES: [(&str, u64, &str); 2] = [
    (TOKENIZER, 2_108_746, "77ef92283d67f0d97e1454909a964afcbfa2019f0fb9f18f8e88d5c25c3ba729"),
    (WEIGHTS, 614_431_440, "abba8294f0c63d00eb3f8d998dacd16177cc5860cad3a11618e7f361a75587fe"),
];
/// The whole download, for the Settings page.
pub(crate) const DOWNLOAD_BYTES: u64 = FILES[0].1 + FILES[1].1;

const DIMS: usize = 512;

// ---------------------------------------------------------------------------------------------
// What the model hears: a log-mel spectrogram of ten seconds at 48 kHz (as `ClapFeatureExtractor` makes it)

const RATE: u32 = 48_000;
const SAMPLES: usize = 480_000;
const FFT: usize = 1024;
const HOP: usize = 480;
const BINS: usize = FFT / 2 + 1;
const MELS: usize = 64;
const FRAMES: usize = SAMPLES / HOP + 1;
/// The spectrogram becomes a 256×256 picture: its time stretched to 1024 frames and cut in four, stacked.
const SIDE: usize = 256;
const RATIO: usize = 4;

/// Mono samples at `rate` as the model wants them: the silence at either end left out, at 48 kHz, the first ten
/// seconds (a shorter sound repeated to fill them as often as it fits whole, then silence). `None` when silent.
pub(crate) fn prepare(mono: &[f32], rate: u32) -> Option<Vec<f32>> {
    let peak = mono.iter().fold(0f32, |m, s| m.max(s.abs()));
    if peak < 1e-4 || rate == 0 {
        return None;
    }
    let gate = peak * 0.003;
    let start = mono.iter().position(|s| s.abs() > gate)?;
    let end = mono.iter().rposition(|s| s.abs() > gate)? + 1;
    // A little more than ten seconds, so resampling has its neighbors at the cut.
    let keep = (u64::from(rate) * 10 + u64::from(rate) / 10) as usize;
    let x = &mono[start..end.min(start + keep)];
    let mut x = crate::audio::resample(x, 1, rate, RATE);
    Some(fit(&mut x))
}

/// Exactly ten seconds: the start of a longer clip, a shorter one tiled whole as often as it fits, then silence.
fn fit(x: &mut Vec<f32>) -> Vec<f32> {
    if x.len() >= SAMPLES {
        x.truncate(SAMPLES);
        return std::mem::take(x);
    }
    let mut out = Vec::with_capacity(SAMPLES);
    if !x.is_empty() {
        for _ in 0..SAMPLES / x.len() {
            out.extend_from_slice(x);
        }
    }
    out.resize(SAMPLES, 0.0);
    out
}

/// Slaney-style triangular mel filters (librosa's defaults) from 50 Hz to 14 kHz, area-normalized, per FFT bin.
fn mel_filters() -> Vec<Vec<(usize, f32)>> {
    let hz_to_mel = |f: f64| {
        if f < 1000.0 { 3.0 * f / 200.0 } else { 15.0 + (f / 1000.0).ln() * 27.0 / 6.4f64.ln() }
    };
    let mel_to_hz = |m: f64| {
        if m < 15.0 { 200.0 * m / 3.0 } else { 1000.0 * ((m - 15.0) * 6.4f64.ln() / 27.0).exp() }
    };
    let (lo, hi) = (hz_to_mel(50.0), hz_to_mel(14_000.0));
    let edges: Vec<f64> = (0..MELS + 2).map(|i| mel_to_hz(lo + (hi - lo) * i as f64 / (MELS + 1) as f64)).collect();
    let nyquist = f64::from(RATE) / 2.0;
    (0..MELS)
        .map(|m| {
            let (left, center, right) = (edges[m], edges[m + 1], edges[m + 2]);
            let norm = 2.0 / (right - left);
            (0..BINS)
                .filter_map(|b| {
                    let f = nyquist * b as f64 / (BINS - 1) as f64;
                    let w = ((f - left) / (center - left)).min((right - f) / (right - center)).max(0.0);
                    (w > 0.0).then_some((b, (w * norm) as f32))
                })
                .collect()
        })
        .collect()
}

/// The log-mel spectrogram (decibels of power, frames × mel bands) of [`SAMPLES`] samples: a periodic Hann window of
/// 1024, hop 480, the ends padded by reflection.
fn log_mel(x: &[f32]) -> Vec<f32> {
    debug_assert_eq!(x.len(), SAMPLES);
    let half = FFT / 2;
    let at = |i: isize| -> f32 {
        let n = x.len() as isize;
        let j = if i < 0 {
            -i
        } else if i >= n {
            2 * (n - 1) - i
        } else {
            i
        };
        x[j as usize]
    };
    let window: Vec<f32> = (0..FFT).map(|n| (0.5 - 0.5 * (2.0 * PI * n as f64 / FFT as f64).cos()) as f32).collect();
    let filters = mel_filters();
    let fft = crate::sounds::Fft::new(FFT);
    let mut out = vec![0f32; FRAMES * MELS];
    let (mut re, mut im) = (vec![0f32; FFT], vec![0f32; FFT]);
    let mut power = vec![0f32; BINS];
    for f in 0..FRAMES {
        let start = (f * HOP) as isize - half as isize;
        for n in 0..FFT {
            re[n] = at(start + n as isize) * window[n];
            im[n] = 0.0;
        }
        fft.run(&mut re, &mut im);
        for b in 0..BINS {
            power[b] = re[b] * re[b] + im[b] * im[b];
        }
        for (m, filter) in filters.iter().enumerate() {
            let e: f32 = filter.iter().map(|&(b, w)| power[b] * w).sum();
            out[f * MELS + m] = 10.0 * e.max(1e-10).log10();
        }
    }
    out
}

/// One cubic convolution weight (`PyTorch`'s bicubic, A = -0.75), `t` being the distance to the sample.
fn cubic(t: f64) -> f64 {
    const A: f64 = -0.75;
    let t = t.abs();
    if t <= 1.0 {
        ((A + 2.0) * t - (A + 3.0)) * t * t + 1.0
    } else if t < 2.0 {
        ((A * t - 5.0 * A) * t + 8.0 * A) * t - 4.0 * A
    } else {
        0.0
    }
}

/// The spectrogram as the encoder's picture: each band normalized (the encoder's batch norm), time stretched from
/// 1001 frames to 1024 (bicubic, corners aligned), then cut into four runs of 256 frames stacked top to bottom, so
/// row `run × 64 + band`, column `frame`.
fn picture(mel: &[f32], scale: &[f32], shift: &[f32]) -> Vec<f32> {
    let time = SIDE * RATIO;
    let norm: Vec<f32> = mel
        .as_chunks::<MELS>()
        .0
        .iter()
        .flat_map(|f| f.iter().enumerate().map(|(m, v)| v * scale[m] + shift[m]))
        .collect();
    let mut out = vec![0f32; SIDE * SIDE];
    let ratio = (FRAMES - 1) as f64 / (time - 1) as f64;
    for t in 0..time {
        let src = t as f64 * ratio;
        let base = src.floor();
        let frac = src - base;
        let taps: [(usize, f32); 4] = std::array::from_fn(|k| {
            let i = (base as isize + k as isize - 1).clamp(0, FRAMES as isize - 1) as usize;
            (i, cubic(frac - (k as f64 - 1.0)) as f32)
        });
        let (run, col) = (t / SIDE, t % SIDE);
        for m in 0..MELS {
            let v: f32 = taps.iter().map(|&(i, w)| norm[i * MELS + m] * w).sum();
            out[(run * MELS + m) * SIDE + col] = v;
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// The audio half: HTSAT, a Swin transformer over the spectrogram's picture

const WINDOW: usize = 8;
const DEPTHS: [usize; 4] = [2, 2, 6, 2];
const HEADS: [usize; 4] = [4, 8, 16, 32];
const PATCH_DIM: usize = 96;
const GRID: usize = SIDE / 4;

#[derive(Debug)]
struct SwinBlock {
    ln1: LayerNorm,
    q: Linear,
    k: Linear,
    v: Linear,
    out: Linear,
    /// The relative position bias, (heads, 64, 64).
    bias: Tensor,
    ln2: LayerNorm,
    fc1: Linear,
    fc2: Linear,
    heads: usize,
    shift: usize,
}

/// Which bias entry each pair of positions in a window uses.
fn relative_index() -> Vec<u32> {
    let n = WINDOW * WINDOW;
    let span = 2 * WINDOW - 1;
    (0..n)
        .flat_map(|i| {
            (0..n).map(move |j| {
                let (hi, wi, hj, wj) = (i / WINDOW, i % WINDOW, j / WINDOW, j % WINDOW);
                ((hi + WINDOW - 1 - hj) * span + (wi + WINDOW - 1 - wj)) as u32
            })
        })
        .collect()
}

impl SwinBlock {
    fn new(vb: &VarBuilder, dim: usize, heads: usize, shift: usize) -> candle_core::Result<Self> {
        let n = WINDOW * WINDOW;
        let a = vb.pp("attention.self");
        let table = a.get(((2 * WINDOW - 1) * (2 * WINDOW - 1), heads), "relative_position_bias_table")?;
        let index = Tensor::from_vec(relative_index(), n * n, &Device::Cpu)?;
        let bias = table.index_select(&index, 0)?.reshape((n, n, heads))?.permute((2, 0, 1))?.contiguous()?;
        let l = |p: &VarBuilder, name: &str| candle_nn::linear(dim, dim, p.pp(name));
        Ok(Self {
            ln1: candle_nn::layer_norm(dim, 1e-5, vb.pp("layernorm_before"))?,
            q: l(&a, "query")?,
            k: l(&a, "key")?,
            v: l(&a, "value")?,
            out: l(&vb.pp("attention.output"), "dense")?,
            bias,
            ln2: candle_nn::layer_norm(dim, 1e-5, vb.pp("layernorm_after"))?,
            fc1: candle_nn::linear(dim, 4 * dim, vb.pp("intermediate.dense"))?,
            fc2: candle_nn::linear(4 * dim, dim, vb.pp("output.dense"))?,
            heads,
            shift,
        })
    }

    #[expect(clippy::many_single_char_names, reason = "the usual names in attention: batch, tokens, q, k, v, w, o")]
    fn attend(&self, xs: &Tensor, mask: Option<&Tensor>) -> candle_core::Result<Tensor> {
        let (b, n, dim) = xs.dims3()?;
        let hd = dim / self.heads;
        let split = |t: Tensor| t.reshape((b, n, self.heads, hd))?.transpose(1, 2)?.contiguous();
        let q = split((self.q.forward(xs)? * (hd as f64).powf(-0.5))?)?;
        let k = split(self.k.forward(xs)?)?;
        let v = split(self.v.forward(xs)?)?;
        let mut w = q.matmul(&k.t()?)?.broadcast_add(&self.bias)?;
        if let Some(mask) = mask {
            let windows = mask.dim(0)?;
            w = w
                .reshape((b / windows, windows, self.heads, n, n))?
                .broadcast_add(&mask.unsqueeze(1)?)?
                .reshape((b, self.heads, n, n))?;
        }
        let w = candle_nn::ops::softmax_last_dim(&w)?;
        let o = w.matmul(&v)?.transpose(1, 2)?.reshape((b, n, dim))?;
        self.out.forward(&o)
    }

    /// `xs` is (batch, side × side, dim).
    fn forward(&self, xs: &Tensor, side: usize, mask: Option<&Tensor>) -> candle_core::Result<Tensor> {
        let (b, _, dim) = xs.dims3()?;
        let s = self.shift as i32;
        let mut h = self.ln1.forward(xs)?.reshape((b, side, side, dim))?;
        if s > 0 {
            h = h.roll(-s, 1)?.roll(-s, 2)?;
        }
        let per = side / WINDOW;
        let windows = h
            .reshape((b, per, WINDOW, per, WINDOW, dim))?
            .permute([0, 1, 3, 2, 4, 5])?
            .contiguous()?
            .reshape((b * per * per, WINDOW * WINDOW, dim))?;
        let attended = self.attend(&windows, if s > 0 { mask } else { None })?;
        let mut h = attended
            .reshape((b, per, per, WINDOW, WINDOW, dim))?
            .permute([0, 1, 3, 2, 4, 5])?
            .contiguous()?
            .reshape((b, side, side, dim))?;
        if s > 0 {
            h = h.roll(s, 1)?.roll(s, 2)?;
        }
        let xs = (xs + h.reshape((b, side * side, dim))?)?;
        let m = self.fc2.forward(&self.fc1.forward(&self.ln2.forward(&xs)?)?.gelu_erf()?)?;
        xs + m
    }
}

/// Which positions of a shifted window came from different places (and mustn't attend to each other): -100 there,
/// 0 elsewhere, (windows, 64, 64).
fn shift_mask(side: usize) -> candle_core::Result<Tensor> {
    let shift = WINDOW / 2;
    let region = |i: usize| {
        if i < side - WINDOW {
            0
        } else if i < side - shift {
            1
        } else {
            2
        }
    };
    let per = side / WINDOW;
    let n = WINDOW * WINDOW;
    let mut mask = Vec::with_capacity(per * per * n * n);
    for wy in 0..per {
        for wx in 0..per {
            let label = |p: usize| region(wy * WINDOW + p / WINDOW) * 3 + region(wx * WINDOW + p % WINDOW);
            for i in 0..n {
                for j in 0..n {
                    mask.push(if label(i) == label(j) { 0f32 } else { -100.0 });
                }
            }
        }
    }
    Tensor::from_vec(mask, (per * per, n, n), &Device::Cpu)
}

#[derive(Debug)]
struct Merge {
    norm: LayerNorm,
    reduction: Linear,
}

impl Merge {
    /// (batch, side², dim) to (batch, (side/2)², 2·dim): each 2×2 block's four in the order
    /// (0,0), (1,0), (0,1), (1,1) by (row, column).
    fn forward(&self, xs: &Tensor, side: usize) -> candle_core::Result<Tensor> {
        let (b, _, dim) = xs.dims3()?;
        let half = side / 2;
        let xs = xs.reshape((b, half, 2, half, 2, dim))?.permute([0, 1, 3, 4, 2, 5])?.contiguous()?.reshape((
            b,
            half * half,
            4 * dim,
        ))?;
        self.reduction.forward(&self.norm.forward(&xs)?)
    }
}

#[derive(Debug)]
struct Stage {
    blocks: Vec<SwinBlock>,
    mask: Option<Tensor>,
    merge: Option<Merge>,
    side: usize,
}

/// Two linear layers with a `ReLU` between: either half's way into the shared space.
#[derive(Debug)]
struct Projection {
    linear1: Linear,
    linear2: Linear,
}

impl Projection {
    fn new(vb: &VarBuilder, dim: usize) -> candle_core::Result<Self> {
        Ok(Self {
            linear1: candle_nn::linear(dim, DIMS, vb.pp("linear1"))?,
            linear2: candle_nn::linear(DIMS, DIMS, vb.pp("linear2"))?,
        })
    }

    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        self.linear2.forward(&self.linear1.forward(xs)?.relu()?)
    }
}

/// The audio half: ten seconds in, an embedding out.
#[derive(Debug)]
struct AudioModel {
    /// The batch norm over mel bands, as a scale and shift.
    scale: Vec<f32>,
    shift: Vec<f32>,
    patches: Conv2d,
    patch_ln: LayerNorm,
    stages: Vec<Stage>,
    norm: LayerNorm,
    projection: Projection,
}

impl AudioModel {
    fn load(dir: &Path) -> Result<Self> {
        let vb = VarBuilder::from_tensors(
            read_tensors(&dir.join(WEIGHTS), &["audio_model.", "audio_projection."])?,
            DType::F32,
            &Device::Cpu,
        );
        let e = vb.pp("audio_model.audio_encoder");
        let bn = e.pp("batch_norm");
        let get = |name: &str| bn.get(MELS, name).and_then(|t| t.to_vec1::<f32>());
        let (weight, bias) = (get("weight")?, get("bias")?);
        let (mean, var) = (get("running_mean")?, get("running_var")?);
        let scale: Vec<f32> = (0..MELS).map(|m| weight[m] / (var[m] + 1e-5).sqrt()).collect();
        let shift: Vec<f32> = (0..MELS).map(|m| bias[m] - mean[m] * scale[m]).collect();
        let conv = candle_nn::Conv2dConfig { stride: 4, ..Default::default() };
        let mut stages = Vec::new();
        let mut dim = PATCH_DIM;
        let mut side = GRID;
        for (i, (&depth, &heads)) in DEPTHS.iter().zip(&HEADS).enumerate() {
            let s = e.pp(format!("layers.{i}"));
            // Once a stage is a single window, there's nothing to shift.
            let shifts = side > WINDOW;
            let blocks = (0..depth)
                .map(|j| {
                    let shift = if shifts && j % 2 == 1 { WINDOW / 2 } else { 0 };
                    SwinBlock::new(&s.pp(format!("blocks.{j}")), dim, heads, shift)
                })
                .collect::<candle_core::Result<Vec<_>>>()?;
            let merge = if i + 1 < DEPTHS.len() {
                let d = s.pp("downsample");
                Some(Merge {
                    norm: candle_nn::layer_norm(4 * dim, 1e-5, d.pp("norm"))?,
                    reduction: candle_nn::linear_no_bias(4 * dim, 2 * dim, d.pp("reduction"))?,
                })
            } else {
                None
            };
            stages.push(Stage { blocks, mask: shifts.then(|| shift_mask(side)).transpose()?, merge, side });
            if i + 1 < DEPTHS.len() {
                dim *= 2;
                side /= 2;
            }
        }
        Ok(Self {
            scale,
            shift,
            patches: candle_nn::conv2d(1, PATCH_DIM, 4, conv, e.pp("patch_embed.proj"))?,
            patch_ln: candle_nn::layer_norm(PATCH_DIM, 1e-5, e.pp("patch_embed.norm"))?,
            stages,
            norm: candle_nn::layer_norm(dim, 1e-5, e.pp("norm"))?,
            projection: Projection::new(&vb.pp("audio_projection"), dim)?,
        })
    }

    /// Embeddings of clips made by [`prepare`].
    fn embed(&self, clips: &[Vec<f32>]) -> Result<Vec<Vec<f32>>> {
        let b = clips.len();
        if b == 0 {
            return Ok(Vec::new());
        }
        let flat: Vec<f32> = clips.iter().flat_map(|c| picture(&log_mel(c), &self.scale, &self.shift)).collect();
        let input = Tensor::from_vec(flat, (b, 1, SIDE, SIDE), &Device::Cpu)?;
        let mut xs = self.patch_ln.forward(&self.patches.forward(&input)?.flatten_from(2)?.transpose(1, 2)?)?;
        for stage in &self.stages {
            for block in &stage.blocks {
                xs = block.forward(&xs, stage.side, stage.mask.as_ref())?;
            }
            if let Some(merge) = &stage.merge {
                xs = merge.forward(&xs, stage.side)?;
            }
        }
        let pooled = self.norm.forward(&xs)?.mean(1)?;
        let out = self.projection.forward(&pooled)?.to_vec2::<f32>()?;
        Ok(out.into_iter().map(normalized).collect())
    }
}

// ---------------------------------------------------------------------------------------------
// The text half: RoBERTa

const MAX_TOKENS: usize = 77;
const VOCAB: usize = 50_265;
const POSITIONS: usize = 514;
const PAD: usize = 1;
const EOS: u32 = 2;
const HIDDEN: usize = 768;

#[derive(Debug)]
struct BertLayer {
    q: Linear,
    k: Linear,
    v: Linear,
    out: Linear,
    ln1: LayerNorm,
    fc1: Linear,
    fc2: Linear,
    ln2: LayerNorm,
}

impl BertLayer {
    fn new(vb: &VarBuilder) -> candle_core::Result<Self> {
        let l = |name: &str| candle_nn::linear(HIDDEN, HIDDEN, vb.pp(name));
        Ok(Self {
            q: l("attention.self.query")?,
            k: l("attention.self.key")?,
            v: l("attention.self.value")?,
            out: l("attention.output.dense")?,
            ln1: candle_nn::layer_norm(HIDDEN, 1e-12, vb.pp("attention.output.LayerNorm"))?,
            fc1: candle_nn::linear(HIDDEN, 4 * HIDDEN, vb.pp("intermediate.dense"))?,
            fc2: candle_nn::linear(4 * HIDDEN, HIDDEN, vb.pp("output.dense"))?,
            ln2: candle_nn::layer_norm(HIDDEN, 1e-12, vb.pp("output.LayerNorm"))?,
        })
    }

    #[expect(clippy::many_single_char_names, reason = "the usual names in attention: batch, tokens, q, k, v, w, o")]
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let heads = 12;
        let (b, n, dim) = xs.dims3()?;
        let hd = dim / heads;
        let split = |t: Tensor| t.reshape((b, n, heads, hd))?.transpose(1, 2)?.contiguous();
        let q = split((self.q.forward(xs)? * (hd as f64).powf(-0.5))?)?;
        let k = split(self.k.forward(xs)?)?;
        let v = split(self.v.forward(xs)?)?;
        let w = candle_nn::ops::softmax_last_dim(&q.matmul(&k.t()?)?)?;
        let o = w.matmul(&v)?.transpose(1, 2)?.reshape((b, n, dim))?;
        let xs = self.ln1.forward(&(self.out.forward(&o)? + xs)?)?;
        let h = self.fc2.forward(&self.fc1.forward(&xs)?.gelu_erf()?)?;
        self.ln2.forward(&(h + xs)?)
    }
}

#[derive(Debug)]
struct TextModel {
    tokenizer: tokenizers::Tokenizer,
    words: Embedding,
    positions: Embedding,
    token_type: Tensor,
    ln: LayerNorm,
    layers: Vec<BertLayer>,
    pooler: Linear,
    projection: Projection,
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
        let e = t.pp("embeddings");
        Ok(Self {
            tokenizer,
            words: candle_nn::embedding(VOCAB, HIDDEN, e.pp("word_embeddings"))?,
            positions: candle_nn::embedding(POSITIONS, HIDDEN, e.pp("position_embeddings"))?,
            token_type: e.pp("token_type_embeddings").get((1, HIDDEN), "weight")?,
            ln: candle_nn::layer_norm(HIDDEN, 1e-12, e.pp("LayerNorm"))?,
            layers: (0..12)
                .map(|i| BertLayer::new(&t.pp(format!("encoder.layer.{i}"))))
                .collect::<candle_core::Result<_>>()?,
            pooler: candle_nn::linear(HIDDEN, HIDDEN, t.pp("pooler.dense"))?,
            projection: Projection::new(&vb.pp("text_projection"), HIDDEN)?,
        })
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let ids = tokens(&self.tokenizer, text)?;
        let n = ids.len();
        let input = Tensor::new(ids.as_slice(), &Device::Cpu)?.unsqueeze(0)?;
        // Positions start after the padding id, as in fairseq.
        let positions: Vec<u32> = (0..n).map(|i| (PAD + 1 + i) as u32).collect();
        let positions = Tensor::new(positions.as_slice(), &Device::Cpu)?.unsqueeze(0)?;
        let xs =
            (self.words.forward(&input)? + self.positions.forward(&positions)?)?.broadcast_add(&self.token_type)?;
        let mut xs = self.ln.forward(&xs)?;
        for layer in &self.layers {
            xs = layer.forward(&xs)?;
        }
        // The start token sums up the sentence.
        let pooled = self.pooler.forward(&xs.i((.., 0, ..))?)?.tanh()?;
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

static CLAP: Model = Model { name: MODEL, repo: REPO, files: &FILES, what: "sound search", activity: "clap" };

static SLOT: Slot<PathBuf> = Slot::new();

pub(crate) fn is_ready() -> bool {
    SLOT.ready().is_some()
}

/// `off`, `looking`, `downloading`, `ready` or `unavailable: <why>`.
pub(crate) fn describe() -> String {
    SLOT.describe(|_| "ready".into())
}

/// Turn sound search on (find the model, downloading it if need be) or off.
pub(crate) fn start(state: &AppState, enabled: bool) {
    TEXT.clear();
    if !enabled {
        SLOT.restart(Status::Off);
        return;
    }
    CLAP.fetch(state, &SLOT, |st| async move {
        TEXT.unload_when_idle();
        schedule(&st).await;
    });
}

pub(crate) async fn remove_files(state: &AppState) {
    CLAP.remove_files(state).await;
}

// ---------------------------------------------------------------------------------------------
// Searching

static TEXT: Text<TextModel> = Text::new("sound search", TextModel::load, TextModel::embed);

/// Sound search's embeddings: per sound file. Words score far apart from sounds: on a library of game sounds (300 of
/// Kenney's), a true match stands 0.25–0.6 above the median sound for the same words ("an explosion" +0.54, "a laser
/// gun" +0.45, "footsteps" +0.25), and words with nothing to find (a cat, birds, someone talking) peak around 0.2.
pub(crate) static SPEC: Spec = Spec {
    load: "SELECT hash, vector FROM sound_embeddings WHERE model = ?1",
    model: MODEL,
    dims: DIMS,
    what: "sound search",
    // A sound's "more like this" listens instead (see `sounds`).
    unrelated: 0.5,
    same: 0.95,
    lift: 0.24,
    margin: 0.12,
    sure: 0.33,
    weak: 0.24,
    strong: 0.45,
};

/// Sound files that sound like what was typed, best first, scored about 0..=0.5; empty when sound search is off or
/// has nothing yet.
pub(crate) async fn search_text(state: &AppState, text: &str) -> Result<Vec<(String, f32)>> {
    let Some(dir) = SLOT.ready() else { return Ok(Vec::new()) };
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    state.clap.ensure(state).await?;
    if state.clap.len() == 0 {
        return Ok(Vec::new());
    }
    let q = TEXT.embed(dir, text.trim().to_lowercase()).await?;
    Ok(state.clap.matching(&q))
}

// ---------------------------------------------------------------------------------------------
// Background job

const KINDS_SQL: &str = crate::sounds::KINDS_SQL;
const MISSING_SQL: &str = "NOT EXISTS (SELECT 1 FROM sound_embeddings e WHERE e.hash = a.blob_hash AND e.model = ?1)";
const ORPHAN_SQL: &str = "NOT EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = e.hash AND a.kind = 'audio')";

/// How many sound files have an embedding, and how many could, for the Settings page.
pub(crate) fn progress(conn: &rusqlite::Connection) -> Result<(u64, u64)> {
    let total: i64 =
        conn.query_row(&format!("SELECT COUNT(DISTINCT a.blob_hash) FROM assets a WHERE {KINDS_SQL}"), [], |r| {
            r.get(0)
        })?;
    let done: i64 = conn.query_row(
        &format!(
            "SELECT COUNT(*) FROM sound_embeddings e WHERE e.model = ?1
               AND EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = e.hash AND {KINDS_SQL})"
        ),
        [MODEL],
        |r| r.get(0),
    )?;
    Ok((done as u64, total as u64))
}

/// Queue a job when sound search is on and some sound has no embedding yet.
pub(crate) async fn schedule(state: &AppState) {
    if !is_ready() {
        return;
    }
    let queued = db::call(&state.pool, |c| {
        let waiting: bool = c.query_row(
            "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued')",
            [crate::jobs::HEAR],
            |r| r.get(0),
        )?;
        if waiting {
            return Ok(false);
        }
        let work: bool = c.query_row(
            &format!(
                "SELECT EXISTS (SELECT 1 FROM assets a WHERE {KINDS_SQL} AND {MISSING_SQL})
                     OR EXISTS (SELECT 1 FROM sound_embeddings e WHERE {ORPHAN_SQL})"
            ),
            [MODEL],
            |r| r.get(0),
        )?;
        if work {
            db::enqueue(c, crate::jobs::HEAR, None, &serde_json::json!({}))?;
        }
        Ok(work)
    })
    .await;
    match queued {
        Ok(true) => state.wake.notify_one(),
        Ok(false) => {}
        Err(e) => tracing::warn!(error = ?e, "could not queue the sound search job"),
    }
}

/// Sounds read and embedded per round.
const BATCH: usize = 8;
/// As much of a file as is read: plenty for its first ten seconds.
const MAX_BYTES: u64 = 12 << 20;

/// Embed every sound that has no embedding, and forget the ones whose files are gone.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    let Some(dir) = SLOT.ready() else { return Ok(()) };
    state.clap.ensure(state).await?;
    let gone: Vec<String> = db::call(&state.pool, |c| {
        let gone = c
            .prepare(&format!("SELECT hash FROM sound_embeddings e WHERE {ORPHAN_SQL}"))?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        c.execute(&format!("DELETE FROM sound_embeddings AS e WHERE {ORPHAN_SQL}"), [])?;
        Ok(gone)
    })
    .await?;
    state.clap.write(|m| {
        for h in &gone {
            m.remove(h);
        }
    });

    let mut model: Option<Arc<AudioModel>> = None;
    let mut unreadable = std::collections::HashSet::new();
    let mut done = 0usize;
    loop {
        let skip = unreadable.clone();
        let todo: Vec<(String, String)> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT a.blob_hash, MIN(a.ext) FROM assets a WHERE {KINDS_SQL} AND {MISSING_SQL} GROUP BY a.blob_hash"
                ))?
                .query_map([MODEL], |r| Ok((r.get(0)?, r.get(1)?)))?
                .collect::<rusqlite::Result<Vec<(String, String)>>>()?;
            Ok(rows.into_iter().filter(|(h, _)| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return Ok(());
        }
        let model = if let Some(m) = &model {
            Arc::clone(m)
        } else {
            let d = dir.clone();
            let m = Arc::new(crate::heavy::run("clap-load", move || AudioModel::load(&d)).await??);
            Arc::clone(model.insert(m))
        };
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            // Switched off partway through: stop, keeping what's done.
            if !is_ready() {
                return Ok(());
            }
            crate::activity::job_progress(crate::jobs::HEAR, None, done, total, "sounds");
            let mut files = Vec::with_capacity(chunk.len());
            for (hash, ext) in chunk {
                match state.blobs.head(hash, MAX_BYTES).await {
                    Ok(bytes) => files.push((hash.clone(), ext.clone(), bytes::Bytes::from(bytes))),
                    Err(e) => {
                        tracing::debug!(hash, error = %e, "sound unreadable for sound search");
                        unreadable.insert(hash.clone());
                    }
                }
            }
            let m = Arc::clone(&model);
            let computed = crate::heavy::run("clap", move || crate::clip::in_pool(|| embed_all(&m, &files))).await??;
            save(state, &computed).await?;
            done += chunk.len();
            state.clap.write(|m| {
                for (hash, v) in computed {
                    if let Some(e) = v.and_then(Entry::new) {
                        m.insert(hash, e);
                    }
                }
            });
        }
        crate::activity::job_progress(crate::jobs::HEAR, None, done, total, "sounds");
    }
}

fn embed_all(model: &AudioModel, files: &[(String, String, bytes::Bytes)]) -> Result<Vec<(String, Option<Vec<i8>>)>> {
    use rayon::prelude::*;
    let prepared: Vec<(String, Option<Vec<f32>>)> = files
        .par_iter()
        .map(|(h, ext, b)| {
            let clip =
                crate::sounds::decode_mono(b.clone(), Some(ext)).and_then(|(mono, rate, _)| prepare(&mono, rate));
            (h.clone(), clip)
        })
        .collect();
    let inputs: Vec<Vec<f32>> = prepared.iter().filter_map(|(_, p)| p.clone()).collect();
    let mut embedded = model.embed(&inputs)?.into_iter();
    Ok(prepared.into_iter().map(|(h, p)| (h, p.and_then(|_| embedded.next()).map(|v| quantize(&v)))).collect())
}

async fn save(state: &AppState, computed: &[(String, Option<Vec<i8>>)]) -> Result<()> {
    let rows: Vec<(String, Vec<u8>)> = computed
        .iter()
        // Not a sound this can decode, or silent: an empty row, so it isn't tried again.
        .map(|(h, v)| (h.clone(), v.as_ref().map_or_default(|v| v.iter().map(|&x| x as u8).collect())))
        .collect();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        {
            let mut stmt =
                tx.prepare("INSERT OR REPLACE INTO sound_embeddings (hash, model, vector) VALUES (?1, ?2, ?3)")?;
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

    #[test]
    fn short_sounds_repeat_whole_and_long_ones_are_cut() {
        let mut short = vec![1.0f32; 200_000];
        let out = fit(&mut short);
        assert_eq!(out.len(), SAMPLES);
        assert_eq!(out[399_999], 1.0, "twice whole");
        assert_eq!(out[400_000], 0.0, "then silence");
        let mut long: Vec<f32> = (0..600_000).map(|i| i as f32).collect();
        let out = fit(&mut long);
        assert_eq!((out.len(), out[SAMPLES - 1]), (SAMPLES, (SAMPLES - 1) as f32));
    }

    #[test]
    fn mel_filters_cover_50_hz_to_14_khz() {
        let f = mel_filters();
        assert_eq!(f.len(), MELS);
        let hz = |b: usize| f64::from(RATE) / 2.0 * b as f64 / (BINS - 1) as f64;
        assert!(hz(f[0][0].0) >= 50.0 && hz(f[0][0].0) < 120.0);
        assert!(hz(f[MELS - 1].last().unwrap().0) < 14_000.0);
        assert!(f.iter().all(|band| !band.is_empty()));
    }

    #[test]
    fn a_tone_lights_its_band() {
        let x: Vec<f32> = (0..SAMPLES).map(|i| (2.0 * PI * 1000.0 * i as f64 / f64::from(RATE)).sin() as f32).collect();
        let mel = log_mel(&x);
        assert_eq!(mel.len(), FRAMES * MELS);
        let frame = &mel[500 * MELS..501 * MELS];
        let loudest = (0..MELS).max_by(|&a, &b| frame[a].total_cmp(&frame[b])).unwrap();
        let f = mel_filters();
        let hz = |b: usize| f64::from(RATE) / 2.0 * b as f64 / (BINS - 1) as f64;
        let (lo, hi) = (hz(f[loudest][0].0), hz(f[loudest].last().unwrap().0));
        assert!(lo <= 1000.0 && 1000.0 <= hi, "band {loudest}: {lo}..{hi} Hz");
    }

    #[test]
    fn the_picture_stacks_four_runs_of_time() {
        // Every frame's value is its index, the same in every band: the picture's rows repeat in runs of 64, each run
        // a later quarter of the (stretched) time.
        let mel: Vec<f32> = (0..FRAMES).flat_map(|f| std::iter::repeat_n(f as f32, MELS)).collect();
        let p = picture(&mel, &[1.0; MELS], &[0.0; MELS]);
        assert!(p[0].abs() < 1e-4, "starts at frame 0");
        assert!((p[(3 * MELS + 5) * SIDE + SIDE - 1] - 1000.0).abs() < 1e-3, "ends at frame 1000");
        // Within what bicubic with A = -0.75 makes of a straight line.
        assert!(
            (p[MELS * SIDE] - 256.0 * 1000.0 / 1023.0).abs() < 0.1,
            "the second run picks up where the first ended"
        );
    }

    #[test]
    fn shifted_windows_keep_apart_what_wrapped_around() {
        let m = shift_mask(16).unwrap().to_vec3::<f32>().unwrap();
        assert_eq!(m.len(), 4);
        assert!(m[0].iter().flatten().all(|&v| v == 0.0), "the first window is all one region");
        // The last window: its top-left and bottom-right quarters came from opposite corners.
        assert_eq!(m[3][0][63], -100.0);
        assert_eq!(m[3][0][1], 0.0);
    }

    #[test]
    fn relative_positions_index_the_bias_table() {
        let idx = relative_index();
        assert_eq!(idx.len(), 64 * 64);
        assert_eq!(idx[0], 7 * 15 + 7, "a position with itself is the middle");
        assert_eq!(idx[63], 0, "the far corner");
        assert_eq!(*idx.iter().max().unwrap(), 224);
    }

    /// The real model, when it's been downloaded: `HOARD_CLAP_DIR=<folder with the two files> cargo test -- --ignored`.
    /// With `HOARD_CLAP_WAVS` (a folder of WAV files), it prints each one's embedding's first numbers and its cosine
    /// with a few descriptions, to compare with a reference.
    #[test]
    #[ignore = "needs the 614 MB model"]
    fn real_model_matches_words_to_sounds() {
        let dir = PathBuf::from(std::env::var("HOARD_CLAP_DIR").expect("HOARD_CLAP_DIR"));
        let text = TextModel::load(&dir).unwrap();
        eprintln!("tokens: {:?}", tokens(&text.tokenizer, "a dog barking").unwrap());
        let audio = AudioModel::load(&dir).unwrap();
        let words = ["a dog barking", "rain falling", "a low thud", "a laser zap", "a pure tone", "white noise"];
        let texts: Vec<Vec<f32>> = words.iter().map(|w| text.embed(w).unwrap()).collect();
        for (w, t) in words.iter().zip(&texts) {
            eprintln!("text {w:?}: {:?}", &t[..6]);
        }
        let cos = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
        let tone: Vec<f32> =
            (0..SAMPLES).map(|i| (0.5 * (2.0 * PI * 440.0 * i as f64 / 48_000.0).sin()) as f32).collect();
        let mut seed = 1u32;
        let noise: Vec<f32> = (0..SAMPLES)
            .map(|_| {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (seed >> 8) as f32 / (1 << 24) as f32 - 0.5
            })
            .collect();
        let started = std::time::Instant::now();
        let got = audio.embed(&[tone, noise]).unwrap();
        eprintln!("2 clips in {:?}", started.elapsed());
        for (name, v) in ["tone", "noise"].iter().zip(&got) {
            let sims: Vec<String> = words.iter().zip(&texts).map(|(w, t)| format!("{w}: {:.3}", cos(v, t))).collect();
            eprintln!("{name}: {:?} | {}", &v[..6], sims.join(", "));
        }
        assert!(cos(&got[0], &texts[4]) > cos(&got[0], &texts[5]), "a tone is more a tone than noise");
        assert!(cos(&got[1], &texts[5]) > cos(&got[1], &texts[4]), "noise is more noise than a tone");
        if let Ok(wavs) = std::env::var("HOARD_CLAP_WAVS") {
            let mut paths: Vec<_> = std::fs::read_dir(wavs).unwrap().map(|e| e.unwrap().path()).collect();
            paths.sort();
            for p in paths.iter().filter(|p| p.extension().is_some_and(|e| e == "wav")) {
                let bytes = bytes::Bytes::from(std::fs::read(p).unwrap());
                let (mut mono, rate, _) = crate::sounds::decode_mono(bytes, Some("wav")).unwrap();
                // As given (at 48 kHz, so nothing is resampled), so a reference fed the same samples should agree.
                assert_eq!(rate, RATE, "{}", p.display());
                let v = &audio.embed(&[fit(&mut mono)]).unwrap()[0];
                let sims: Vec<String> =
                    words.iter().zip(&texts).map(|(w, t)| format!("{w}: {:.3}", cos(v, t))).collect();
                eprintln!("{}: {:?} | {}", p.display(), &v[..6], sims.join(", "));
            }
        }
    }

    /// Calibration on real sounds: `HOARD_CLAP_DIR=<model> HOARD_CLAP_SOUNDS=<folder with names.tsv> cargo test --
    /// --ignored calibrate --nocapture` prints, for each query, the median score, the best and the best few by name.
    #[test]
    #[ignore = "needs the model and a folder of sounds"]
    fn calibrate() {
        let dir = PathBuf::from(std::env::var("HOARD_CLAP_DIR").expect("HOARD_CLAP_DIR"));
        let folder = PathBuf::from(std::env::var("HOARD_CLAP_SOUNDS").expect("HOARD_CLAP_SOUNDS"));
        let names: Vec<(String, String)> = std::fs::read_to_string(folder.join("names.tsv"))
            .unwrap()
            .lines()
            .filter_map(|l| l.split_once('\t').map(|(f, n)| (f.to_owned(), n.to_owned())))
            .collect();
        // Embedding the sounds is the slow part: kept in a cache file between runs.
        let cache = std::env::temp_dir().join("hoard-clap-calibrate.tsv");
        let mut raw: Vec<(String, Vec<i8>)> = std::fs::read_to_string(&cache).map_or_default(|c| {
            c.lines()
                .filter_map(|l| l.split_once('\t'))
                .map(|(n, v)| (n.to_owned(), v.split(',').filter_map(|x| x.parse().ok()).collect()))
                .collect()
        });
        if raw.is_empty() {
            let audio = AudioModel::load(&dir).unwrap();
            let started = std::time::Instant::now();
            for chunk in names.chunks(BATCH) {
                let files: Vec<(String, String, bytes::Bytes)> = chunk
                    .iter()
                    .map(|(f, n)| {
                        let ext = f.rsplit('.').next().unwrap().to_owned();
                        (n.clone(), ext, bytes::Bytes::from(std::fs::read(folder.join(f)).unwrap()))
                    })
                    .collect();
                raw.extend(embed_all(&audio, &files).unwrap().into_iter().filter_map(|(n, v)| Some((n, v?))));
            }
            eprintln!("{} sounds in {:?}", raw.len(), started.elapsed());
            let lines: Vec<String> = raw
                .iter()
                .map(|(n, v)| format!("{n}\t{}", v.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")))
                .collect();
            std::fs::write(&cache, lines.join("\n")).unwrap();
        }
        let sounds: Vec<(String, Entry)> = raw.into_iter().filter_map(|(n, v)| Some((n, Entry::new(v)?))).collect();
        let text = TextModel::load(&dir).unwrap();
        let queries = std::env::var("HOARD_CLAP_QUERIES").unwrap_or_else(|_| {
            "a dog barking|a car engine|thunder|an explosion|footsteps|picking up a coin|a laser gun|a button click|\
             a man saying fight|a door creaking|music|glass breaking|a whoosh|a bell ringing|a piano|a robot voice|\
             a woman speaking|a sword hit|a punch|cards being shuffled|dice rolling|water splashing|a monster growl|\
             a cat meowing|birds singing|a jump sound effect|game over|an engine humming|a spaceship|a beep"
                .into()
        });
        for q in queries.split('|') {
            let t = text.embed(q).unwrap();
            let mut scored: Vec<(f32, &str)> = sounds.iter().map(|(n, v)| (v.cosine_f32(&t), n.as_str())).collect();
            scored.sort_by(|a, b| b.0.total_cmp(&a.0));
            let median = scored[scored.len() / 2].0;
            let top: Vec<String> = scored
                .iter()
                .take(5)
                .map(|(s, n)| format!("{:+.3} {}", s - median, n.rsplit('/').take(2).collect::<Vec<_>>().join("<")))
                .collect();
            eprintln!("{q:?}: median {median:.3} best {:.3}\n    {}", scored[0].0, top.join("\n    "));
        }
    }
}
