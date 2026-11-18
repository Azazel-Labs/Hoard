//! The audio half: HTSAT, a Swin transformer over the spectrogram's picture, and its projection into the space the
//! text half shares.

use std::path::Path;

use anyhow::Result;
use candle_core::{DType, Device, Module, Tensor};
use candle_nn::{Conv2d, LayerNorm, Linear, VarBuilder};

use super::spectrogram::{MELS, SIDE, log_mel, picture};
use super::{DIMS, WEIGHTS};
use crate::embed::{normalized, read_tensors};

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
pub(super) struct Projection {
    linear1: Linear,
    linear2: Linear,
}

impl Projection {
    pub(super) fn new(vb: &VarBuilder, dim: usize) -> candle_core::Result<Self> {
        Ok(Self {
            linear1: candle_nn::linear(dim, DIMS, vb.pp("linear1"))?,
            linear2: candle_nn::linear(DIMS, DIMS, vb.pp("linear2"))?,
        })
    }

    pub(super) fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        self.linear2.forward(&self.linear1.forward(xs)?.relu()?)
    }
}

/// The audio half: ten seconds in, an embedding out.
#[derive(Debug)]
pub(super) struct AudioModel {
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
    pub(super) fn load(dir: &Path) -> Result<Self> {
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
    pub(super) fn embed(&self, clips: &[Vec<f32>]) -> Result<Vec<Vec<f32>>> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
