//! Google's SigLIP 2 (ViT-B/32 at 256×256), the model image search runs (see `clip` for the search itself): every
//! thumbnail gets a 768-number embedding, and so does what's typed in the search box.
//!
//! The base model at 32-pixel patches: a picture is 64 patches, about the work of CLIP's ViT-B/32, where 16-pixel
//! patches would be four times it on a server's processor for a little more. The words half's table of 256,000
//! tokens is most of the 1.5 GB file; it's never loaded, only the rows a search needs are read from the file.

use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_nn::{Conv2d, LayerNorm, Linear, VarBuilder};

use crate::clip::{Attention, normalized, read_tensors, safetensors_header};

/// Stored with each embedding.
pub(crate) const MODEL: &str = "siglip2-b32-256";

/// One revision of the model, pinned, so a change upstream can't change what's run.
pub(crate) const REPO: &str =
    "https://huggingface.co/google/siglip2-base-patch32-256/resolve/94dffa8cb1179de3e03f091dbc3917e5d5a9ae84";
const WEIGHTS: &str = "model.safetensors";
const TOKENIZER: &str = "tokenizer.json";
/// Name, size and SHA-256 of each file.
pub(crate) const FILES: [(&str, u64, &str); 2] = [
    (TOKENIZER, 34_363_039, "cb9140fae3ac5122c972d37adf83e1248471a38147ad76f8215c8872c6fd8322"),
    (WEIGHTS, 1_507_473_288, "7d241bb3becad218f211f480487f491df4f8c0a472ecf7afdec5615815a301f1"),
];

pub(crate) const DIMS: usize = 768;

// ---------------------------------------------------------------------------------------------
// The model (after candle-transformers' SigLIP, MIT/Apache-2.0)

const IMAGE: usize = 256;
const PATCH: usize = 32;
const PATCHES: usize = (IMAGE / PATCH) * (IMAGE / PATCH);
const WIDTH: usize = 768;
const HEADS: usize = 12;
const HIDDEN: usize = 3072;
const EPS: f64 = 1e-6;
/// Every text is padded to this many tokens: the model was taught that way, and sums it up in the last one.
const MAX_TOKENS: usize = 64;
const VOCAB: usize = 256_000;
const PAD: u32 = 0;
const EOS: u32 = 1;
const TOKEN_TABLE: &str = "text_model.embeddings.token_embedding.weight";

#[derive(Debug)]
struct Layer {
    attn: Attention,
    ln1: LayerNorm,
    fc1: Linear,
    fc2: Linear,
    ln2: LayerNorm,
}

impl Layer {
    fn new(vb: &VarBuilder) -> candle_core::Result<Self> {
        Ok(Self {
            attn: Attention::new(&vb.pp("self_attn"), WIDTH, HEADS)?,
            ln1: candle_nn::layer_norm(WIDTH, EPS, vb.pp("layer_norm1"))?,
            fc1: candle_nn::linear(WIDTH, HIDDEN, vb.pp("mlp.fc1"))?,
            fc2: candle_nn::linear(HIDDEN, WIDTH, vb.pp("mlp.fc2"))?,
            ln2: candle_nn::layer_norm(WIDTH, EPS, vb.pp("layer_norm2"))?,
        })
    }

    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let xs = (self.attn.forward(&self.ln1.forward(xs)?, None)? + xs)?;
        // GELU, the tanh approximation, as it was trained.
        let h = self.fc2.forward(&self.fc1.forward(&self.ln2.forward(&xs)?)?.gelu()?)?;
        h + xs
    }
}

fn encoder(vb: &VarBuilder) -> candle_core::Result<Vec<Layer>> {
    (0..12).map(|i| Layer::new(&vb.pp(format!("encoder.layers.{i}")))).collect()
}

/// The words half.
#[derive(Debug)]
pub(crate) struct TextModel {
    tokenizer: tokenizers::Tokenizer,
    weights: PathBuf,
    /// Where the token table starts in the file.
    table: u64,
    positions: Tensor,
    layers: Vec<Layer>,
    final_ln: LayerNorm,
    head: Linear,
}

impl TextModel {
    pub(crate) fn load(dir: &Path) -> Result<Self> {
        let mut tokenizer = tokenizers::Tokenizer::from_file(dir.join(TOKENIZER)).map_err(anyhow::Error::msg)?;
        // It pads to 64 itself, which would put the end marker out of reach of cutting a long text short.
        tokenizer.with_padding(None);
        let weights = dir.join(WEIGHTS);
        let (base, header) = safetensors_header(&weights)?;
        let table = header.get(TOKEN_TABLE).context("the model file has no token table")?;
        ensure!(table.dtype == "F32" && table.shape == [VOCAB, WIDTH], "the token table isn't what was expected");
        let mut tensors = read_tensors(&weights, &["text_model."])?;
        tensors.remove(TOKEN_TABLE);
        let vb = VarBuilder::from_tensors(tensors, DType::F32, &Device::Cpu);
        let t = vb.pp("text_model");
        Ok(Self {
            tokenizer,
            table: base + table.data_offsets[0],
            weights,
            positions: t.pp("embeddings.position_embedding").get((MAX_TOKENS, WIDTH), "weight")?,
            layers: encoder(&t)?,
            final_ln: candle_nn::layer_norm(WIDTH, EPS, t.pp("final_layer_norm"))?,
            head: candle_nn::linear(WIDTH, WIDTH, t.pp("head"))?,
        })
    }

    /// The token table's rows for `ids`, read from the file.
    fn rows(&self, ids: &[u32]) -> Result<Tensor> {
        let mut file = std::fs::File::open(&self.weights)?;
        let mut values = Vec::with_capacity(ids.len() * WIDTH);
        let mut row = vec![0u8; WIDTH * 4];
        for &id in ids {
            ensure!((id as usize) < VOCAB, "token {id} is past the table");
            file.seek(SeekFrom::Start(self.table + u64::from(id) * (WIDTH as u64 * 4)))?;
            file.read_exact(&mut row)?;
            values.extend(row.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)));
        }
        Ok(Tensor::from_vec(values, (ids.len(), WIDTH), &Device::Cpu)?)
    }

    pub(crate) fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let ids = tokens(&self.tokenizer, text)?;
        let mut xs = (self.rows(&ids)? + &self.positions)?.unsqueeze(0)?;
        for layer in &self.layers {
            xs = layer.forward(&xs)?;
        }
        let pooled = self.final_ln.forward(&xs)?.i((.., MAX_TOKENS - 1, ..))?;
        let v = self.head.forward(&pooled)?.squeeze(0)?.to_vec1::<f32>()?;
        Ok(normalized(v))
    }
}

/// Token ids for `text` (lowercased, as the model was taught), the end marker kept, padded to [`MAX_TOKENS`].
fn tokens(tokenizer: &tokenizers::Tokenizer, text: &str) -> Result<Vec<u32>> {
    let mut ids = tokenizer.encode(text.to_lowercase(), true).map_err(anyhow::Error::msg)?.get_ids().to_vec();
    ensure!(ids.last() == Some(&EOS), "the tokenizer gave no end marker");
    if ids.len() > MAX_TOKENS {
        ids.truncate(MAX_TOKENS - 1);
        ids.push(EOS);
    }
    ids.resize(MAX_TOKENS, PAD);
    Ok(ids)
}

/// Sums up a picture's patches: one learned question asked of all of them.
#[derive(Debug)]
struct Pooling {
    probe: Tensor,
    q: Linear,
    k: Linear,
    v: Linear,
    out: Linear,
    ln: LayerNorm,
    fc1: Linear,
    fc2: Linear,
}

impl Pooling {
    fn new(vb: &VarBuilder) -> candle_core::Result<Self> {
        let w = vb.get((3 * WIDTH, WIDTH), "attention.in_proj_weight")?;
        let b = vb.get(3 * WIDTH, "attention.in_proj_bias")?;
        let part = |i: usize| -> candle_core::Result<Linear> {
            Ok(Linear::new(w.narrow(0, i * WIDTH, WIDTH)?, Some(b.narrow(0, i * WIDTH, WIDTH)?)))
        };
        Ok(Self {
            probe: vb.get((1, 1, WIDTH), "probe")?,
            q: part(0)?,
            k: part(1)?,
            v: part(2)?,
            out: candle_nn::linear(WIDTH, WIDTH, vb.pp("attention.out_proj"))?,
            ln: candle_nn::layer_norm(WIDTH, EPS, vb.pp("layernorm"))?,
            fc1: candle_nn::linear(WIDTH, HIDDEN, vb.pp("mlp.fc1"))?,
            fc2: candle_nn::linear(HIDDEN, WIDTH, vb.pp("mlp.fc2"))?,
        })
    }

    #[expect(clippy::many_single_char_names, reason = "the usual names in attention: batch, tokens, q, k, v, w, o, h")]
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let (b, n, _) = xs.dims3()?;
        let hd = WIDTH / HEADS;
        let heads = |t: Tensor, len: usize| t.reshape((b, len, HEADS, hd))?.transpose(1, 2)?.contiguous();
        let probe = self.probe.broadcast_as((b, 1, WIDTH))?.contiguous()?;
        let q = heads((self.q.forward(&probe)? * (hd as f64).powf(-0.5))?, 1)?;
        let k = heads(self.k.forward(xs)?, n)?;
        let v = heads(self.v.forward(xs)?, n)?;
        let w = candle_nn::ops::softmax_last_dim(&q.matmul(&k.t()?)?)?;
        let o = w.matmul(&v)?.transpose(1, 2)?.reshape((b, 1, WIDTH))?;
        let h = self.out.forward(&o)?;
        let h = (self.fc2.forward(&self.fc1.forward(&self.ln.forward(&h)?)?.gelu()?)? + h)?;
        h.i((.., 0, ..))
    }
}

/// The picture half: 256×256 pictures in, embeddings out.
#[derive(Debug)]
pub(crate) struct VisionModel {
    patches: Conv2d,
    positions: Tensor,
    layers: Vec<Layer>,
    post_ln: LayerNorm,
    pooling: Pooling,
}

impl VisionModel {
    pub(crate) fn load(dir: &Path) -> Result<Self> {
        let vb =
            VarBuilder::from_tensors(read_tensors(&dir.join(WEIGHTS), &["vision_model."])?, DType::F32, &Device::Cpu);
        let v = vb.pp("vision_model");
        let conv = candle_nn::Conv2dConfig { stride: PATCH, ..Default::default() };
        Ok(Self {
            patches: candle_nn::conv2d(3, WIDTH, PATCH, conv, v.pp("embeddings.patch_embedding"))?,
            positions: v.pp("embeddings.position_embedding").get((PATCHES, WIDTH), "weight")?,
            layers: encoder(&v)?,
            post_ln: candle_nn::layer_norm(WIDTH, EPS, v.pp("post_layernorm"))?,
            pooling: Pooling::new(&v.pp("head"))?,
        })
    }

    /// Embeddings of pictures made by [`pixels`].
    pub(crate) fn embed(&self, pictures: &[Vec<f32>]) -> Result<Vec<Vec<f32>>> {
        let b = pictures.len();
        if b == 0 {
            return Ok(Vec::new());
        }
        let flat: Vec<f32> = pictures.iter().flatten().copied().collect();
        let input = Tensor::from_vec(flat, (b, 3, IMAGE, IMAGE), &Device::Cpu)?;
        let mut xs = self.patches.forward(&input)?.flatten_from(2)?.transpose(1, 2)?.broadcast_add(&self.positions)?;
        for layer in &self.layers {
            xs = layer.forward(&xs)?;
        }
        let out = self.pooling.forward(&self.post_ln.forward(&xs)?)?.to_vec2::<f32>()?;
        Ok(out.into_iter().map(normalized).collect())
    }
}

/// A thumbnail as the model wants it: transparency over white, squeezed to 256×256 (bilinear), each channel from -1
/// to 1, channels first. `None` when it can't be decoded or is all transparent.
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
    let side = IMAGE as u32;
    let resized = image::imageops::resize(&rgb, side, side, image::imageops::FilterType::Triangle);
    let mut out = vec![0.0f32; 3 * IMAGE * IMAGE];
    for (x, y, p) in resized.enumerate_pixels() {
        let at = y as usize * IMAGE + x as usize;
        for c in 0..3 {
            out[c * IMAGE * IMAGE + at] = f32::from(p[c]) / 127.5 - 1.0;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    fn png(w: u32, h: u32, f: impl Fn(u32, u32) -> image::Rgba<u8>) -> Vec<u8> {
        let img = image::RgbaImage::from_fn(w, h, f);
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }

    #[test]
    fn pixels_are_squeezed_square_and_over_white() {
        // Wide: left half red, right half transparent. Squeezed, not cropped, so both halves stay.
        let bytes =
            png(512, 128, |x, _| if x < 256 { image::Rgba([255, 0, 0, 255]) } else { image::Rgba([0, 0, 0, 0]) });
        let p = pixels(&bytes).unwrap();
        assert_eq!(p.len(), 3 * IMAGE * IMAGE);
        let at = |c: usize, x: usize, y: usize| p[c * IMAGE * IMAGE + y * IMAGE + x];
        assert!((at(0, 10, 0) - 1.0).abs() < 1e-3 && (at(1, 10, IMAGE - 1) + 1.0).abs() < 1e-3, "red on the left");
        assert!((at(1, IMAGE - 10, 100) - 1.0).abs() < 1e-3, "white where it was transparent");
        assert!(pixels(&png(8, 8, |_, _| image::Rgba([0, 0, 0, 0]))).is_none(), "all transparent");
        assert!(pixels(b"not a picture").is_none());
    }

    /// The real model, when it's been downloaded: `HOARD_SIGLIP_DIR=<folder with the two files> cargo test --
    /// --ignored`.
    #[test]
    #[ignore = "needs the 1.5 GB model"]
    fn real_model_matches_words_to_pictures() {
        let dir = PathBuf::from(std::env::var("HOARD_SIGLIP_DIR").expect("HOARD_SIGLIP_DIR"));
        let text = TextModel::load(&dir).unwrap();
        let ids = tokens(&text.tokenizer, "This is a photo of a cat.").unwrap();
        eprintln!("tokens: {:?}", &ids[..12]);
        assert_eq!(ids.len(), MAX_TOKENS);
        let vision = VisionModel::load(&dir).unwrap();
        let red = png(256, 256, |_, _| image::Rgba([220, 20, 20, 255]));
        let stripes =
            png(256, 256, |x, _| if (x / 16) % 2 == 0 { image::Rgba([0, 0, 0, 255]) } else { image::Rgba([255; 4]) });
        let started = Instant::now();
        let pics = vision.embed(&[pixels(&red).unwrap(), pixels(&stripes).unwrap()]).unwrap();
        eprintln!("2 pictures in {:?}", started.elapsed());
        let cos = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
        let started = Instant::now();
        let (t_red, t_stripes) = (
            text.embed("this is a photo of a red square.").unwrap(),
            text.embed("this is a photo of black and white stripes.").unwrap(),
        );
        eprintln!("2 texts in {:?}", started.elapsed());
        // As SigLIP judges a match by itself: the sigmoid of its scaled cosine plus its bias.
        let all = read_tensors(&dir.join(WEIGHTS), &["logit_"]).unwrap();
        let (scale, bias) = (
            all["logit_scale"].flatten_all().unwrap().to_vec1::<f32>().unwrap()[0].exp(),
            all["logit_bias"].flatten_all().unwrap().to_vec1::<f32>().unwrap()[0],
        );
        let p = |c: f32| 1.0 / (1.0 + (-(c * scale + bias)).exp());
        let (rr, rs, ss, sr) =
            (cos(&pics[0], &t_red), cos(&pics[0], &t_stripes), cos(&pics[1], &t_stripes), cos(&pics[1], &t_red));
        eprintln!("scale {scale:.1} bias {bias:.2}");
        eprintln!("red: {rr:.3} (p {:.3}) / {rs:.3} (p {:.3})", p(rr), p(rs));
        eprintln!("stripes: {ss:.3} (p {:.3}) / {sr:.3} (p {:.3})", p(ss), p(sr));
        eprintln!("red vs stripes, as pictures: {:.3}", cos(&pics[0], &pics[1]));
        assert!(rr > rs && ss > sr);
        assert!(p(rr) > 0.05 && p(ss) > 0.05, "the model believes its matches");
        assert!(p(rs) < 0.01 && p(sr) < 0.01, "and is sure of its mismatches");
    }
}
