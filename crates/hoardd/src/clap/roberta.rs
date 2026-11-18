//! The text half: `RoBERTa`, from what's typed to tokens to the embedding a sound's is compared with.

use std::path::Path;

use anyhow::{Result, ensure};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_nn::{Embedding, LayerNorm, Linear, VarBuilder};

use super::htsat::Projection;
use super::{TOKENIZER, WEIGHTS};
use crate::embed::{normalized, read_tensors};

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
pub(super) struct TextModel {
    pub(super) tokenizer: tokenizers::Tokenizer,
    words: Embedding,
    positions: Embedding,
    token_type: Tensor,
    ln: LayerNorm,
    layers: Vec<BertLayer>,
    pooler: Linear,
    projection: Projection,
}

impl TextModel {
    pub(super) fn load(dir: &Path) -> Result<Self> {
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

    pub(super) fn embed(&self, text: &str) -> Result<Vec<f32>> {
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
pub(super) fn tokens(tokenizer: &tokenizers::Tokenizer, text: &str) -> Result<Vec<u32>> {
    let mut ids = tokenizer.encode(text, true).map_err(anyhow::Error::msg)?.get_ids().to_vec();
    ensure!(ids.len() >= 2, "the tokenizer gave no start and end markers");
    if ids.len() > MAX_TOKENS {
        ids.truncate(MAX_TOKENS - 1);
        ids.push(EOS);
    }
    Ok(ids)
}
