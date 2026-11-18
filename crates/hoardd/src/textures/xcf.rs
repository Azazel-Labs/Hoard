//! GIMP (`.xcf`): there's no flattened copy inside, so the layers are composited here the way GIMP shows them, each
//! at its offset, with its opacity and mask, hidden ones and those in hidden groups left out. Every layer is
//! blended as Normal: a thumbnail doesn't need Multiply or Screen to be recognizable. Pixels are kept in 64x64 tiles,
//! stored plain, run-length encoded per channel or zlib-compressed. Files from GIMP 2.10 on can hold 16-bit and float
//! pixels; 8- and 16-bit integer ones are read.

use std::io::Read;

use anyhow::{Context, Result, bail, ensure};
use image::{DynamicImage, RgbaImage};
use serde_json::{Map, json};

use super::{Decoded, check_size};

const TILE: usize = 64;

// Properties read, by number.
const PROP_END: u32 = 0;
const PROP_COLORMAP: u32 = 1;
const PROP_OPACITY: u32 = 6;
const PROP_VISIBLE: u32 = 8;
const PROP_APPLY_MASK: u32 = 11;
const PROP_OFFSETS: u32 = 15;
const PROP_COMPRESSION: u32 = 17;
const PROP_GROUP_ITEM: u32 = 29;
const PROP_ITEM_PATH: u32 = 30;
const PROP_FLOAT_OPACITY: u32 = 33;

/// Layers read, at most.
const MAX_LAYERS: usize = 4096;

pub(super) fn decode(bytes: &[u8]) -> Result<Decoded> {
    let xcf = Xcf::new(bytes)?;
    let (width, height) = check_size(xcf.width, xcf.height)?;
    let layers = xcf.layers()?;
    // Premultiplied RGBA, painted bottom layer first (the file lists the top one first).
    let mut canvas = vec![[0f32; 4]; width * height];
    for layer in layers.iter().rev().filter(|l| !l.group) {
        let Some(opacity) = shown(layer, &layers) else { continue };
        let (lw, lh) = check_size(layer.width, layer.height)?;
        let (pixels, bpp) = xcf.pixels(layer.hierarchy)?;
        let mask = match (layer.apply_mask, layer.mask) {
            (true, at) if at != 0 => Some(xcf.mask(at)?),
            _ => None,
        };
        let colors = layer.kind.channels();
        ensure!(bpp == colors * xcf.depth, "XCF layer has {bpp} bytes a pixel, not {}", colors * xcf.depth);
        ensure!(pixels.len() >= lw * lh * bpp, "XCF layer data is cut short");
        for y in 0..lh {
            let Some(cy) = usize::try_from(layer.y + y as i64).ok().filter(|&cy| cy < height) else { continue };
            for x in 0..lw {
                let Some(cx) = usize::try_from(layer.x + x as i64).ok().filter(|&cx| cx < width) else { continue };
                let i = y * lw + x;
                let color = xcf.rgba(layer.kind, &pixels[i * bpp..(i + 1) * bpp]);
                let covered = mask.as_ref().and_then(|m| m.get(i)).map_or(1.0, |&v| f32::from(v) / 255.0);
                let alpha = f32::from(color[3]) / 255.0 * opacity * covered;
                let under = &mut canvas[cy * width + cx];
                for (k, &v) in color[..3].iter().enumerate() {
                    under[k] = f32::from(v) / 255.0 * alpha + under[k] * (1.0 - alpha);
                }
                under[3] = alpha + under[3] * (1.0 - alpha);
            }
        }
    }
    let rgba = canvas
        .iter()
        .flat_map(|&[r, g, b, a]| {
            let straight = |v: f32| if a > 0.0 { (v / a * 255.0).round().clamp(0.0, 255.0) as u8 } else { 0 };
            [straight(r), straight(g), straight(b), (a * 255.0).round() as u8]
        })
        .collect();
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(xcf.width, xcf.height, rgba).context("size")?);
    let mut facts = Map::new();
    facts.insert("layers".into(), json!(layers.iter().filter(|l| !l.group).count()));
    Ok(Decoded { image, facts })
}

/// A layer's opacity once its groups' are counted in, or `None` when it or a group it's in is hidden.
fn shown(layer: &Layer, layers: &[Layer]) -> Option<f32> {
    let mut opacity = layer.opacity;
    if !layer.visible {
        return None;
    }
    // Each group the layer is in has the path of the layer's first indices.
    for depth in 1..layer.path.len() {
        let group = layers.iter().find(|g| g.group && g.path == layer.path[..depth])?;
        if !group.visible {
            return None;
        }
        opacity *= group.opacity;
    }
    Some(opacity)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Rgb,
    Rgba,
    Gray,
    GrayA,
    Indexed,
    IndexedA,
}

impl Kind {
    fn of(n: u32) -> Result<Self> {
        Ok(match n {
            0 => Self::Rgb,
            1 => Self::Rgba,
            2 => Self::Gray,
            3 => Self::GrayA,
            4 => Self::Indexed,
            5 => Self::IndexedA,
            _ => bail!("XCF layer type {n} isn't known"),
        })
    }

    fn channels(self) -> usize {
        match self {
            Self::Gray | Self::Indexed => 1,
            Self::GrayA | Self::IndexedA => 2,
            Self::Rgb => 3,
            Self::Rgba => 4,
        }
    }
}

struct Layer {
    width: u32,
    height: u32,
    kind: Kind,
    x: i64,
    y: i64,
    opacity: f32,
    visible: bool,
    group: bool,
    /// Where the layer sits among groups: its index in each, outermost first. Empty for a layer in no group.
    path: Vec<u32>,
    apply_mask: bool,
    hierarchy: usize,
    mask: usize,
}

struct Xcf<'a> {
    bytes: &'a [u8],
    /// The file format's version: 0 for the oldest (`file`), 11 and up with 64-bit offsets.
    version: u32,
    width: u32,
    height: u32,
    /// Bytes a channel.
    depth: usize,
    compression: u32,
    colormap: Vec<[u8; 3]>,
    /// Where the list of layer offsets starts.
    layer_list: usize,
}

impl<'a> Xcf<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self> {
        ensure!(bytes.starts_with(b"gimp xcf "), "not a GIMP file");
        let version = match bytes.get(9..13).context("XCF header is cut short")? {
            b"file" => 0,
            [b'v', digits @ ..] => std::str::from_utf8(digits)?.parse().context("XCF version isn't a number")?,
            _ => bail!("XCF version isn't known"),
        };
        let mut xcf =
            Self { bytes, version, width: 0, height: 0, depth: 1, compression: 0, colormap: Vec::new(), layer_list: 0 };
        xcf.width = xcf.u32(14)?;
        xcf.height = xcf.u32(18)?;
        let mut at = 26;
        if version >= 4 {
            xcf.depth = match (version, xcf.u32(26)?) {
                // Version 4 numbered precisions from 0; later ones by hundreds, linear then gamma.
                (4, 0) | (5.., 100 | 150) => 1,
                (4, 1) | (5.., 200 | 250) => 2,
                (_, p) => bail!("XCF precision {p} isn't supported yet: only 8- and 16-bit integer pixels are"),
            };
            at = 30;
        }
        let mut compression = 0;
        let mut colormap = Vec::new();
        at = xcf.props(at, |kind, body, _| {
            match kind {
                PROP_COMPRESSION => compression = u32::from(*xcf.bytes.get(body).context("XCF property is cut short")?),
                PROP_COLORMAP => {
                    let n = xcf.u32(body)? as usize;
                    let colors =
                        xcf.bytes.get(body + 4..body + 4 + n.min(256) * 3).context("XCF colormap is cut short")?;
                    colormap = colors.as_chunks::<3>().0.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;
        xcf.compression = compression;
        xcf.colormap = colormap;
        xcf.layer_list = at;
        Ok(xcf)
    }

    /// One pixel of a layer as RGBA.
    fn rgba(&self, kind: Kind, px: &[u8]) -> [u8; 4] {
        // The high byte of each channel: big-endian, so the first.
        let c = |n: usize| px[n * self.depth];
        match kind {
            Kind::Rgb => [c(0), c(1), c(2), 255],
            Kind::Rgba => [c(0), c(1), c(2), c(3)],
            Kind::Gray => [c(0), c(0), c(0), 255],
            Kind::GrayA => [c(0), c(0), c(0), c(1)],
            Kind::Indexed | Kind::IndexedA => {
                let [r, g, b] = self.colormap.get(usize::from(c(0))).copied().unwrap_or_default();
                [r, g, b, if kind == Kind::IndexedA { c(1) } else { 255 }]
            }
        }
    }

    fn u32(&self, at: usize) -> Result<u32> {
        self.bytes
            .get(at..)
            .and_then(<[u8]>::first_chunk)
            .map(|b| u32::from_be_bytes(*b))
            .context("XCF file is cut short")
    }

    /// An offset into the file, and where the next field is.
    fn offset(&self, at: usize) -> Result<(usize, usize)> {
        if self.version >= 11 {
            let b = self.bytes.get(at..).and_then(<[u8]>::first_chunk).context("XCF file is cut short")?;
            Ok((usize::try_from(u64::from_be_bytes(*b))?, at + 8))
        } else {
            Ok((self.u32(at)? as usize, at + 4))
        }
    }

    /// Calls `each(kind, body, len)` for every property in the list at `at`, returning where the list ends.
    fn props(&self, mut at: usize, mut each: impl FnMut(u32, usize, usize) -> Result<()>) -> Result<usize> {
        loop {
            let (kind, len) = (self.u32(at)?, self.u32(at + 4)? as usize);
            if kind == PROP_END {
                return Ok(at + 8 + len);
            }
            each(kind, at + 8, len)?;
            at = at + 8 + len;
        }
    }

    fn layers(&self) -> Result<Vec<Layer>> {
        let mut out = Vec::new();
        let mut at = self.layer_list;
        loop {
            let (layer, next) = self.offset(at)?;
            if layer == 0 || out.len() >= MAX_LAYERS {
                return Ok(out);
            }
            out.push(self.layer(layer)?);
            at = next;
        }
    }

    fn layer(&self, at: usize) -> Result<Layer> {
        let (width, height, kind) = (self.u32(at)?, self.u32(at + 4)?, Kind::of(self.u32(at + 8)?)?);
        let name = self.u32(at + 12)? as usize;
        let mut layer = Layer {
            width,
            height,
            kind,
            x: 0,
            y: 0,
            opacity: 1.0,
            visible: true,
            group: false,
            path: Vec::new(),
            apply_mask: false,
            hierarchy: 0,
            mask: 0,
        };
        let end = self.props(at + 16 + name, |prop, body, len| {
            match prop {
                PROP_OPACITY => layer.opacity = self.u32(body)?.min(255) as f32 / 255.0,
                PROP_FLOAT_OPACITY => layer.opacity = f32::from_bits(self.u32(body)?).clamp(0.0, 1.0),
                PROP_VISIBLE => layer.visible = self.u32(body)? != 0,
                PROP_APPLY_MASK => layer.apply_mask = self.u32(body)? != 0,
                PROP_OFFSETS => {
                    // Signed: a layer can hang off the top left.
                    layer.x = i64::from(self.u32(body)?.cast_signed());
                    layer.y = i64::from(self.u32(body + 4)?.cast_signed());
                }
                PROP_GROUP_ITEM => layer.group = true,
                PROP_ITEM_PATH => layer.path = (0..len / 4).map(|i| self.u32(body + i * 4)).collect::<Result<_>>()?,
                _ => {}
            }
            Ok(())
        })?;
        let (hierarchy, next) = self.offset(end)?;
        layer.hierarchy = hierarchy;
        layer.mask = self.offset(next)?.0;
        Ok(layer)
    }

    /// A layer mask's values, one byte a pixel: a channel with a name and properties, then its pixels.
    fn mask(&self, at: usize) -> Result<Vec<u8>> {
        let name = self.u32(at + 8)? as usize;
        let end = self.props(at + 12 + name, |_, _, _| Ok(()))?;
        let (pixels, bpp) = self.pixels(self.offset(end)?.0)?;
        Ok(pixels.iter().step_by(bpp).copied().collect())
    }

    /// The full-size pixels a hierarchy holds, interleaved, and the bytes a pixel.
    fn pixels(&self, at: usize) -> Result<(Vec<u8>, usize)> {
        let bpp = self.u32(at + 8)? as usize;
        ensure!((1..=16).contains(&bpp), "XCF pixels are {bpp} bytes");
        // The first level is the full-size one.
        let (level, _) = self.offset(at + 12)?;
        let (w, h) = check_size(self.u32(level)?, self.u32(level + 4)?)?;
        let mut out = vec![0; w * h * bpp];
        let mut at = level + 8;
        for row in 0..h.div_ceil(TILE) {
            for col in 0..w.div_ceil(TILE) {
                let (tile, next) = self.offset(at)?;
                ensure!(tile != 0, "XCF layer has fewer tiles than it needs");
                at = next;
                let (tw, th) = (TILE.min(w - col * TILE), TILE.min(h - row * TILE));
                let data = self.tile(tile, tw * th, bpp)?;
                for y in 0..th {
                    let to = ((row * TILE + y) * w + col * TILE) * bpp;
                    out[to..to + tw * bpp].copy_from_slice(&data[y * tw * bpp..(y + 1) * tw * bpp]);
                }
            }
        }
        Ok((out, bpp))
    }

    /// One tile's `n` pixels of `bpp` bytes, interleaved.
    fn tile(&self, at: usize, n: usize, bpp: usize) -> Result<Vec<u8>> {
        let data = self.bytes.get(at..).context("XCF tile is past the end")?;
        match self.compression {
            0 => Ok(data.get(..n * bpp).context("XCF tile is cut short")?.to_vec()),
            1 => rle(data, n, bpp),
            2 => {
                let mut out = vec![0; n * bpp];
                flate2::read::ZlibDecoder::new(data).read_exact(&mut out).context("XCF tile is cut short")?;
                Ok(out)
            }
            c => bail!("XCF compression {c} isn't supported"),
        }
    }
}

/// GIMP's run-length encoding: each byte of the pixel in turn, as runs of one value or of literal bytes.
fn rle(data: &[u8], n: usize, bpp: usize) -> Result<Vec<u8>> {
    let mut out = vec![0; n * bpp];
    let mut src = data.iter().copied();
    let mut next = || src.next().context("XCF tile is cut short");
    for channel in 0..bpp {
        let mut i = 0;
        while i < n {
            let op = next()?;
            // 128 and 127 say a 16-bit count follows.
            let long = |next: &mut dyn FnMut() -> Result<u8>| -> Result<usize> {
                Ok(usize::from(u16::from_be_bytes([next()?, next()?])))
            };
            let (count, literal) = match op {
                128 => (long(&mut next)?, true),
                129.. => (256 - usize::from(op), true),
                127 => (long(&mut next)?, false),
                _ => (usize::from(op) + 1, false),
            };
            ensure!(i + count <= n, "XCF tile runs past its end");
            let value = if literal { 0 } else { next()? };
            for _ in 0..count {
                out[i * bpp + channel] = if literal { next()? } else { value };
                i += 1;
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLayer {
        kind: u32,
        size: (u32, u32),
        offset: (i32, i32),
        props: Vec<(u32, Vec<u8>)>,
        pixels: Vec<u8>,
    }

    fn be(v: &[u32]) -> Vec<u8> {
        v.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    /// Runs where a channel is one value throughout, literals otherwise.
    fn encode(pixels: &[u8], bpp: usize) -> Vec<u8> {
        let mut out = Vec::new();
        for channel in 0..bpp {
            let values: Vec<u8> = pixels.iter().skip(channel).step_by(bpp).copied().collect();
            if values.iter().all(|&v| v == values[0]) {
                out.extend([values.len() as u8 - 1, values[0]]);
            } else {
                out.push((256 - values.len()) as u8);
                out.extend(values);
            }
        }
        out
    }

    /// An XCF file of version `version`, its layers listed top first, with one tile each.
    fn xcf(version: u32, compression: u8, size: (u32, u32), layers: &[TestLayer]) -> Vec<u8> {
        let wide = version >= 11;
        let offset =
            |v: usize| if wide { (v as u64).to_be_bytes().to_vec() } else { (v as u32).to_be_bytes().to_vec() };
        let mut b = format!("gimp xcf v{version:03}\0").into_bytes();
        b.extend(be(&[size.0, size.1, 0]));
        if version >= 4 {
            b.extend(be(&[150]));
        }
        b.extend(be(&[PROP_COMPRESSION, 1]));
        b.push(compression);
        b.extend(be(&[PROP_END, 0]));
        let list = b.len();
        let slot = if wide { 8 } else { 4 };
        b.resize(list + slot * (layers.len() + 2), 0);
        for (n, l) in layers.iter().enumerate() {
            let at = b.len();
            b[list + n * slot..list + (n + 1) * slot].copy_from_slice(&offset(at));
            b.extend(be(&[l.size.0, l.size.1, l.kind, 2]));
            b.extend(b"L\0");
            b.extend(be(&[PROP_OFFSETS, 8]));
            b.extend(l.offset.0.to_be_bytes());
            b.extend(l.offset.1.to_be_bytes());
            for (prop, body) in &l.props {
                b.extend(be(&[*prop, body.len() as u32]));
                b.extend(body);
            }
            b.extend(be(&[PROP_END, 0]));
            let bpp = [3, 4, 1, 2, 1, 2][l.kind as usize];
            // Hierarchy, then its level, then the tile.
            let hierarchy = b.len() + slot * 2;
            b.extend(offset(hierarchy));
            b.extend(offset(0));
            let level = hierarchy + 12 + slot * 2;
            b.extend(be(&[l.size.0, l.size.1, bpp as u32]));
            b.extend(offset(level));
            b.extend(offset(0));
            let tile = level + 8 + slot * 2;
            b.extend(be(&[l.size.0, l.size.1]));
            b.extend(offset(tile));
            b.extend(offset(0));
            match compression {
                1 => b.extend(encode(&l.pixels, bpp)),
                2 => {
                    let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                    std::io::Write::write_all(&mut z, &l.pixels).unwrap();
                    b.extend(z.finish().unwrap());
                }
                _ => b.extend(&l.pixels),
            }
        }
        b
    }

    #[test]
    fn composites_the_visible_layers() {
        let red = TestLayer { kind: 0, size: (2, 2), offset: (0, 0), props: vec![], pixels: [255, 0, 0].repeat(4) };
        // Half-opaque green over the bottom right pixel.
        let green = TestLayer {
            kind: 1,
            size: (1, 1),
            offset: (1, 1),
            props: vec![(PROP_OPACITY, be(&[128]))],
            pixels: vec![0, 255, 0, 255],
        };
        let hidden = TestLayer {
            kind: 0,
            size: (2, 2),
            offset: (0, 0),
            props: vec![(PROP_VISIBLE, be(&[0]))],
            pixels: [0, 0, 255].repeat(4),
        };
        for (version, compression) in [(3, 1), (3, 0), (11, 2)] {
            let file = xcf(version, compression, (2, 2), &[hidden.clone(), green.clone(), red.clone()]);
            let d = decode(&file).unwrap();
            let img = d.image.to_rgba8();
            assert_eq!(img.get_pixel(0, 0).0, [255, 0, 0, 255], "v{version} compression {compression}");
            assert_eq!(img.get_pixel(1, 1).0, [127, 128, 0, 255], "v{version} compression {compression}");
            assert_eq!(d.facts["layers"], 3);
        }
    }

    #[test]
    fn leaves_out_layers_in_hidden_groups_and_off_the_canvas() {
        let group = TestLayer {
            kind: 1,
            size: (2, 2),
            offset: (0, 0),
            props: vec![(PROP_GROUP_ITEM, vec![]), (PROP_VISIBLE, be(&[0])), (PROP_ITEM_PATH, be(&[0]))],
            pixels: vec![0; 16],
        };
        let inside = TestLayer {
            kind: 2,
            size: (2, 2),
            offset: (0, 0),
            props: vec![(PROP_ITEM_PATH, be(&[0, 0]))],
            pixels: vec![255; 4],
        };
        // Gray, hanging one pixel off the top left.
        let shifted =
            TestLayer { kind: 2, size: (2, 2), offset: (-1, -1), props: vec![], pixels: vec![10, 20, 30, 40] };
        let d = decode(&xcf(3, 1, (2, 2), &[group, inside, shifted])).unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.get_pixel(0, 0).0, [40, 40, 40, 255]);
        assert_eq!(img.get_pixel(1, 1).0[3], 0);
        assert_eq!(d.facts["layers"], 2);
        assert!(decode(b"gimp xcf v003\0").is_err());
    }

    impl Clone for TestLayer {
        fn clone(&self) -> Self {
            Self {
                kind: self.kind,
                size: self.size,
                offset: self.offset,
                props: self.props.clone(),
                pixels: self.pixels.clone(),
            }
        }
    }
}
