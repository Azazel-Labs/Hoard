//! Hoard's own sprite sheets. An atlas file found in a pack (see [`crate::atlas`]) is kept as a [`Sheet`]: its pages
//! (the pack's images, which stay where they are), each sprite's place on them, its trim and how it's turned, and
//! its animations (the file's own, or guessed from numbered names). The sheet is stored with the atlas file's asset,
//! which becomes a `sheet`, and it's what the sheet's page, thumbnail and downloads are made from.
//!
//! A sheet downloads in many formats. Each says what it can describe (sprites turned one way, or any right angle, or
//! not at all; trimmed sprites or not), and the sprites are packed again only when the sheet uses something the format
//! can't say, or when its sprites are each an image of their own.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::io::{Cursor, Write as _};
use std::path::Path;

use anyhow::{Context as _, Result, bail};
use image::{RgbaImage, imageops};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::atlas::{self, Atlas, Turn};
use crate::formats::Output;

/// The version of the format a sheet is written in.
pub(crate) const VERSION: u32 = 1;
/// The extension a sheet downloads with.
pub(crate) const EXT: &str = "hoardsheet";

/// Pixels between packed sprites.
const PAD: u32 = 2;
/// The biggest page packing makes; more sprites go on more pages.
const MAX_PAGE: u32 = 4096;
/// The biggest grid image; more cells go on more images.
const MAX_GRID: u32 = 8192;

#[expect(clippy::trivially_copy_pass_by_ref, reason = "serde passes fields by reference")]
fn is_zero(n: &u16) -> bool {
    *n == 0
}

fn is_origin(p: &[i64; 2]) -> bool {
    *p == [0, 0]
}

fn yes() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Sheet {
    /// The version of the format, under a name that says what the file is.
    #[serde(rename = "hoard_sheet")]
    pub(crate) version: u32,
    /// The format it was read from: `texturepacker`, `libgdx`, `godot-tileset`, …
    pub(crate) from: String,
    pub(crate) pages: Vec<Page>,
    pub(crate) sprites: Vec<Sprite>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) animations: Vec<Anim>,
    /// A tile set's tile size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) tile: Option<[u32; 2]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Page {
    /// The image: in the library its path in the pack, in a downloaded sheet its file beside the sheet.
    pub(crate) image: String,
    pub(crate) size: [u32; 2],
    /// The library's asset for the image. Not in a downloaded sheet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) asset: Option<String>,
    /// The image's stored file, so the sheet changes when its pixels do. Not in a downloaded sheet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Sprite {
    pub(crate) name: String,
    pub(crate) page: usize,
    /// Where its pixels are on the page, as stored there (turned, if they are).
    pub(crate) rect: [u32; 4],
    /// How far its stored pixels are turned clockwise from upright: 0, 90, 180 or 270.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub(crate) turn: u16,
    /// Its size, upright, before its transparent margin was trimmed off.
    pub(crate) size: [u32; 2],
    /// Where its pixels, upright, sit in that size.
    #[serde(default, skip_serializing_if = "is_origin")]
    pub(crate) at: [i64; 2],
}

impl Sprite {
    /// The size of its stored pixels, upright.
    pub(crate) fn upright(&self) -> [u32; 2] {
        let [_, _, w, h] = self.rect;
        if self.turn % 180 == 90 { [h, w] } else { [w, h] }
    }

    pub(crate) fn trimmed(&self) -> bool {
        self.at != [0, 0] || self.size != self.upright()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Anim {
    pub(crate) name: String,
    pub(crate) frames: Vec<AnimFrame>,
    #[serde(rename = "loop", default = "yes")]
    pub(crate) looped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AnimFrame {
    pub(crate) sprite: usize,
    pub(crate) ms: u32,
}

impl Sheet {
    /// A sheet from an atlas file, given each of its pages as the library has it (`None` for one it doesn't).
    /// A sprite outside its page is left out; one with an empty rectangle takes its whole page (Godot names whole
    /// images as frames). Names are made unique. Without animations of its own, and unless it's a tile set, numbered
    /// sprites are taken as animations (`walk_0`, `walk_1`, … is `walk`).
    pub(crate) fn from_atlas(atlas: &Atlas, pages: &[Option<Page>]) -> Option<Sheet> {
        let mut out_pages = Vec::new();
        let mut sprites = Vec::new();
        let mut index: HashMap<(usize, usize), usize> = HashMap::new();
        let mut names = HashSet::new();
        for (pi, (page, found)) in atlas.pages.iter().zip(pages).enumerate() {
            let Some(found) = found else { continue };
            let [pw, ph] = found.size;
            let mut any = false;
            for (si, s) in page.sprites.iter().enumerate() {
                let mut rect = s.rect;
                if rect[2] == 0 && rect[3] == 0 {
                    rect = [0, 0, pw, ph];
                }
                let [x, y, w, h] = rect;
                if w == 0 || h == 0 || x.saturating_add(w) > pw || y.saturating_add(h) > ph {
                    continue;
                }
                let turn = match s.turn {
                    Turn::Upright => 0,
                    Turn::Clockwise => 90,
                    Turn::CounterClockwise => 270,
                };
                let upright = if turn == 0 { [w, h] } else { [h, w] };
                let (size, at) = s.trim.map_or((upright, [0, 0]), |t| (t.size, t.at));
                let base = s.name.trim();
                let base = if base.is_empty() { "sprite" } else { base };
                let mut name = base.to_owned();
                let mut n = 2;
                while !names.insert(name.to_lowercase()) {
                    name = format!("{base}_{n}");
                    n += 1;
                }
                index.insert((pi, si), sprites.len());
                sprites.push(Sprite { name, page: out_pages.len(), rect, turn, size, at });
                any = true;
            }
            if any {
                out_pages.push(found.clone());
            }
        }
        if sprites.is_empty() {
            return None;
        }
        let mut animations: Vec<Anim> = atlas
            .animations
            .iter()
            .filter_map(|a| {
                let frames: Vec<AnimFrame> = a
                    .frames
                    .iter()
                    .filter_map(|f| Some(AnimFrame { sprite: *index.get(&(f.page, f.sprite))?, ms: f.ms.max(1) }))
                    .collect();
                (!frames.is_empty()).then(|| Anim { name: a.name.clone(), frames, looped: a.looped })
            })
            .collect();
        if animations.is_empty() && atlas.tile.is_none() {
            animations = guess_animations(&sprites);
        }
        Some(Sheet {
            version: VERSION,
            from: atlas.format.to_owned(),
            pages: out_pages,
            sprites,
            animations,
            tile: atlas.tile,
        })
    }

    /// What the sheet's asset says about it in its meta.
    pub(crate) fn summary(&self) -> Value {
        let mut v = json!({
            "format": self.from,
            "sprites": self.sprites.len(),
            "pages": self.pages.len(),
            "animations": self.animations.len(),
        });
        if let Some(t) = self.tile {
            v["tile"] = json!(t);
        }
        v
    }
}

/// Numbered sprites as animations: names that differ only in a number at the end, two or more of them, in order.
fn guess_animations(sprites: &[Sprite]) -> Vec<Anim> {
    let mut groups: BTreeMap<String, Vec<(u64, usize)>> = BTreeMap::new();
    for (i, s) in sprites.iter().enumerate() {
        let name = image_stem(&s.name);
        let digits = name.len() - name.trim_end_matches(|c: char| c.is_ascii_digit()).len();
        if digits == 0 || digits > 6 {
            continue;
        }
        let (prefix, n) = name.split_at(name.len() - digits);
        let Ok(n) = n.parse::<u64>() else { continue };
        let prefix = prefix.trim_end_matches(['_', '-', ' ', '.', '/', '(']);
        groups.entry(prefix.to_owned()).or_default().push((n, i));
    }
    groups
        .into_iter()
        .filter(|(_, f)| f.len() >= 2)
        .map(|(name, mut f)| {
            f.sort_unstable();
            let name = if name.is_empty() { "default".to_owned() } else { name };
            let frames = f.into_iter().map(|(_, sprite)| AnimFrame { sprite, ms: atlas::FRAME_MS }).collect();
            Anim { name, frames, looped: true }
        })
        .collect()
}

/// A sprite's name without the extension of the image it was packed from (`walk_01.png` → `walk_01`).
pub(crate) fn image_stem(name: &str) -> &str {
    match name.rsplit_once('.') {
        Some((stem, ext))
            if matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tga" | "webp" | "psd" | "ase" | "aseprite"
            ) =>
        {
            stem
        }
        _ => name,
    }
}

/// A sprite's stored pixels, upright: what's left of it after trimming.
pub(crate) fn cut(page: &RgbaImage, s: &Sprite) -> Option<RgbaImage> {
    let [x, y, w, h] = s.rect;
    if w == 0 || h == 0 || x.checked_add(w)? > page.width() || y.checked_add(h)? > page.height() {
        return None;
    }
    let region = imageops::crop_imm(page, x, y, w, h).to_image();
    Some(match s.turn {
        90 => imageops::rotate270(&region),
        180 => imageops::rotate180(&region),
        270 => imageops::rotate90(&region),
        _ => region,
    })
}

/// A sprite as it was drawn: upright, on its untrimmed canvas.
pub(crate) fn whole(page: &RgbaImage, s: &Sprite) -> Option<RgbaImage> {
    let pixels = cut(page, s)?;
    if !s.trimmed() {
        return Some(pixels);
    }
    let [w, h] = s.size;
    if w == 0 || h == 0 || w > 16_384 || h > 16_384 {
        return None;
    }
    let mut canvas = RgbaImage::new(w, h);
    imageops::overlay(&mut canvas, &pixels, s.at[0], s.at[1]);
    Some(canvas)
}

/// The thumbnail: the first sprites (an animation's first frames, if it has any) side by side, upright. `pages`
/// holds the pages those sprites are on. Also whether it's pixel art, to be kept exact.
pub(crate) fn thumbnail(sheet: &Sheet, pages: &HashMap<usize, RgbaImage>) -> Option<(RgbaImage, bool)> {
    let size = 256u32;
    let shown = shown_sprites(sheet);
    let images: Vec<RgbaImage> =
        shown.iter().filter_map(|&i| whole(pages.get(&sheet.sprites[i].page)?, &sheet.sprites[i])).collect();
    if images.is_empty() {
        return None;
    }
    let n = images.len();
    let cols = (n as f64).sqrt().ceil() as u32;
    let rows = (n as u32).div_ceil(cols);
    let cell = size / cols.max(rows);
    let small = images.iter().all(|i| i.width() <= 64 && i.height() <= 64);
    let mut out = RgbaImage::new(size, size);
    let (ox, oy) = ((size - cols * cell) / 2, (size - rows * cell) / 2);
    for (k, img) in images.iter().enumerate() {
        let (c, r) = (k as u32 % cols, k as u32 / cols);
        let inner = cell.saturating_sub(if n > 1 { 4 } else { 0 }).max(1);
        let scale = f64::from(inner) / f64::from(img.width().max(img.height()));
        // Pixel art grows by whole steps, so its pixels stay square.
        let scale = if small && scale >= 1.0 { scale.floor() } else { scale };
        let (w, h) = (
            ((f64::from(img.width()) * scale).round() as u32).max(1),
            ((f64::from(img.height()) * scale).round() as u32).max(1),
        );
        let filter = if small || scale >= 1.0 { imageops::FilterType::Nearest } else { imageops::FilterType::Triangle };
        let fitted = imageops::resize(img, w, h, filter);
        let x = ox + c * cell + (cell - w.min(cell)) / 2;
        let y = oy + r * cell + (cell - h.min(cell)) / 2;
        imageops::overlay(&mut out, &fitted, i64::from(x), i64::from(y));
    }
    Some((out, small))
}

/// The sprites a thumbnail shows: one from each animation if it has some, otherwise the first ones.
pub(crate) fn shown_sprites(sheet: &Sheet) -> Vec<usize> {
    const MOST: usize = 16;
    let mut out: Vec<usize> = Vec::new();
    for a in &sheet.animations {
        if let Some(f) = a.frames.first()
            && !out.contains(&f.sprite)
        {
            out.push(f.sprite);
        }
    }
    if out.len() < 4 {
        for i in 0..sheet.sprites.len() {
            if out.len() >= MOST.min(sheet.sprites.len()) {
                break;
            }
            if !out.contains(&i) {
                out.push(i);
            }
        }
    }
    out.truncate(MOST);
    out
}

// ---------------------------------------------------------------------------------------------
// Downloads

/// Which way a format can say a sprite is turned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rot {
    No,
    /// Turned clockwise only (TexturePacker, Starling, Cocos2d).
    Clockwise,
    /// Turned counter-clockwise only (libGDX).
    CounterClockwise,
    /// Any right angle.
    Any,
}

impl Rot {
    fn allows(self, turn: u16) -> bool {
        turn == 0
            || match self {
                Rot::No => false,
                Rot::Clockwise => turn == 90,
                Rot::CounterClockwise => turn == 270,
                Rot::Any => true,
            }
    }

    /// The turn packing gives a sprite it turns.
    fn turn(self) -> Option<u16> {
        match self {
            Rot::No => None,
            Rot::Clockwise | Rot::Any => Some(90),
            Rot::CounterClockwise => Some(270),
        }
    }
}

struct Target {
    format: &'static str,
    label: &'static str,
    rot: Rot,
    trim: bool,
}

const TARGETS: &[Target] = &[
    Target { format: "sheet-hoard", label: "Hoard sprite sheet (.hoardsheet)", rot: Rot::Any, trim: true },
    Target {
        format: "sheet-tp-hash",
        label: "TexturePacker JSON hash (Phaser, PixiJS)",
        rot: Rot::Clockwise,
        trim: true,
    },
    Target { format: "sheet-tp-array", label: "TexturePacker JSON array", rot: Rot::Clockwise, trim: true },
    Target { format: "sheet-starling", label: "Starling / Sparrow XML", rot: Rot::Clockwise, trim: true },
    Target { format: "sheet-libgdx", label: "libGDX / Spine .atlas", rot: Rot::CounterClockwise, trim: true },
    Target { format: "sheet-cocos2d", label: "Cocos2d .plist", rot: Rot::Clockwise, trim: true },
    Target { format: "sheet-godot-frames", label: "Godot SpriteFrames (.tres)", rot: Rot::No, trim: true },
    Target { format: "sheet-godot-tileset", label: "Godot TileSet (.tres)", rot: Rot::No, trim: false },
    Target { format: "sheet-grid", label: "Grid image (PNG)", rot: Rot::No, trim: false },
    Target { format: "sheet-frames", label: "Each sprite as a PNG", rot: Rot::No, trim: false },
];

/// The formats a sheet downloads as, each zipped with its images.
pub(crate) fn options() -> Vec<hoard_core::FormatOption> {
    TARGETS
        .iter()
        .map(|t| hoard_core::FormatOption {
            format: t.format.to_owned(),
            label: format!("{} (zip)", t.label),
            resizable: false,
        })
        .collect()
}

pub(crate) fn is_format(format: &str) -> bool {
    TARGETS.iter().any(|t| t.format == format)
}

/// A format's name, as people say it.
pub(crate) fn label(format: &str) -> Option<&'static str> {
    TARGETS.iter().find(|t| t.format == format).map(|t| t.label)
}

/// A stored sheet's pages: each one's image (its path in the pack), asset and stored file.
pub(crate) fn pages(bytes: &[u8]) -> Result<Vec<Page>> {
    let sheet: Sheet = serde_json::from_slice(bytes).context("reading the sheet")?;
    Ok(sheet.pages)
}

/// A file going into the download.
type Files = Vec<(String, Vec<u8>)>;

/// A sheet saved at `sheet_path` (its pages at their pack paths under `root`) in another format, zipped with its
/// images. The files are named after the sheet's file.
pub(crate) fn export(sheet_path: &Path, root: &Path, format: &str) -> Result<Output> {
    let target = TARGETS.iter().find(|t| t.format == format).with_context(|| format!("no sheet format {format}"))?;
    let sheet: Sheet = serde_json::from_slice(&std::fs::read(sheet_path)?).context("reading the sheet")?;
    let stem = sheet_path.file_stem().map_or_else(|| "sheet".to_owned(), |s| s.to_string_lossy().into_owned());
    let stem = clean_name(&stem).unwrap_or_else(|| "sheet".to_owned());
    let mut images = Vec::with_capacity(sheet.pages.len());
    let mut originals = Vec::with_capacity(sheet.pages.len());
    for p in &sheet.pages {
        let rel = crate::ingest::safe_rel_path(&p.image).context("unsafe page path")?;
        let path = root.join(rel);
        let ext = crate::ingest::ext_of(&p.image);
        let img = crate::formats::load_image(&path, &ext).with_context(|| format!("reading {}", p.image))?.to_rgba8();
        originals.push((ext == "png").then(|| std::fs::read(&path)).transpose()?);
        images.push(img);
    }
    let files = write_format(&sheet, &images, &originals, target, &stem)?;
    zip(files)
}

fn zip(files: Files) -> Result<Output> {
    let mut out = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut out));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .last_modified_time(zip::DateTime::default_for_write());
        for (name, bytes) in files {
            zip.start_file(name, opts)?;
            zip.write_all(&bytes)?;
        }
        zip.finish()?;
    }
    Ok(Output { bytes: out, ext: "zip" })
}

fn write_format(
    sheet: &Sheet,
    images: &[RgbaImage],
    originals: &[Option<Vec<u8>>],
    t: &Target,
    stem: &str,
) -> Result<Files> {
    match t.format {
        "sheet-grid" => return grid_files(sheet, images, stem),
        "sheet-frames" => return frame_files(sheet, images, stem),
        "sheet-godot-tileset" => return godot_tileset(sheet, images, originals, stem),
        _ => {}
    }
    // Pages of their own sprites only (Godot's whole-image frames) are packed together, except in Hoard's own
    // format, which keeps a sheet as it is.
    let scattered = t.format != "sheet-hoard" && sheet.pages.len() >= 4 && sheet.pages.len() * 2 > sheet.sprites.len();
    let unsayable = sheet.sprites.iter().any(|s| !t.rot.allows(s.turn) || (!t.trim && s.trimmed()));
    let (mut out, pngs) = if scattered || unsayable {
        let (packed, pages) = repack(sheet, images, t.rot, t.trim);
        let pngs = pages.into_iter().map(crate::analyze::png_bytes).collect::<Result<Vec<_>>>()?;
        (packed, pngs)
    } else {
        let pngs = images
            .iter()
            .zip(originals)
            .map(|(img, orig)| orig.clone().map_or_else(|| crate::analyze::png_bytes(img.clone()), Ok))
            .collect::<Result<Vec<_>>>()?;
        (sheet.clone(), pngs)
    };
    let one = out.pages.len() == 1;
    let page_name = |i: usize| if one { format!("{stem}.png") } else { format!("{stem}-{i}.png") };
    for (i, p) in out.pages.iter_mut().enumerate() {
        p.image = page_name(i);
        p.asset = None;
        p.hash = None;
    }
    let mut files: Files = match t.format {
        "sheet-hoard" => vec![(format!("{stem}.{EXT}"), serde_json::to_vec_pretty(&out)?)],
        "sheet-tp-hash" => texture_packer(&out, stem, false),
        "sheet-tp-array" => texture_packer(&out, stem, true),
        "sheet-starling" => starling(&out, stem),
        "sheet-libgdx" => vec![(format!("{stem}.atlas"), libgdx(&out).into_bytes())],
        "sheet-cocos2d" => cocos(&out, stem),
        "sheet-godot-frames" => vec![(format!("{stem}.tres"), godot_frames(&out).into_bytes())],
        other => bail!("no sheet format {other}"),
    };
    files.extend(pngs.into_iter().enumerate().map(|(i, b)| (page_name(i), b)));
    Ok(files)
}

/// A name safe as a file name: folders and anything climbing out of them dropped, odd characters replaced.
fn clean_name(name: &str) -> Option<String> {
    let parts: Vec<String> = name
        .split(['/', '\\'])
        .map(str::trim)
        .filter(|p| !p.is_empty() && *p != "." && *p != "..")
        .map(|p| p.chars().map(|c| if c.is_control() || r#"<>:"|?*"#.contains(c) { '_' } else { c }).collect())
        .collect();
    (!parts.is_empty()).then(|| parts.join("/"))
}

// ---------------------------------------------------------------------------------------------
// Packing

/// Free space in a page being packed (`MaxRects`).
struct Bin {
    w: u32,
    h: u32,
    free: Vec<[u32; 4]>,
}

impl Bin {
    fn new(w: u32, h: u32) -> Self {
        Bin { w, h, free: vec![[0, 0, w, h]] }
    }

    /// Where a `w`×`h` box fits best (the least left over on its shorter side), and whether it's turned to fit.
    fn find(&self, w: u32, h: u32, turn: bool) -> Option<(u32, u32, bool)> {
        // The best so far: how much it leaves over (the shorter side first), and where.
        let mut best: Option<([u32; 2], (u32, u32, bool))> = None;
        for f in &self.free {
            for (bw, bh, turned) in [(w, h, false), (h, w, true)] {
                if (turned && (!turn || w == h)) || bw > f[2] || bh > f[3] {
                    continue;
                }
                let (a, b) = (f[2] - bw, f[3] - bh);
                let score = [a.min(b), a.max(b)];
                if best.is_none_or(|(s, _)| score < s) {
                    best = Some((score, (f[0], f[1], turned)));
                }
            }
        }
        best.map(|(_, at)| at)
    }

    fn place(&mut self, r: [u32; 4]) {
        let [x, y, w, h] = r;
        let mut next = Vec::with_capacity(self.free.len() + 4);
        for &f in &self.free {
            let [fx, fy, fw, fh] = f;
            if x >= fx + fw || x + w <= fx || y >= fy + fh || y + h <= fy {
                next.push(f);
                continue;
            }
            if x > fx {
                next.push([fx, fy, x - fx, fh]);
            }
            if x + w < fx + fw {
                next.push([x + w, fy, fx + fw - x - w, fh]);
            }
            if y > fy {
                next.push([fx, fy, fw, y - fy]);
            }
            if y + h < fy + fh {
                next.push([fx, y + h, fw, fy + fh - y - h]);
            }
        }
        // Drop the free rectangles inside others.
        let inside = |a: &[u32; 4], b: &[u32; 4]| {
            a[0] >= b[0] && a[1] >= b[1] && a[0] + a[2] <= b[0] + b[2] && a[1] + a[3] <= b[1] + b[3]
        };
        let mut kept: Vec<[u32; 4]> = Vec::with_capacity(next.len());
        for (i, a) in next.iter().enumerate() {
            let covered = next.iter().enumerate().any(|(j, b)| i != j && inside(a, b) && (a != b || j < i));
            if !covered {
                kept.push(*a);
            }
        }
        self.free = kept;
    }
}

/// Where packing put a box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Spot {
    page: usize,
    x: u32,
    y: u32,
    turned: bool,
}

/// Pack boxes onto as few pages as fit, each at most [`MAX_PAGE`] a side (a box bigger than that gets a page of its
/// own), [`PAD`] pixels apart, turned when `turn` allows and it fits better. Returns each page's size and each box's
/// spot.
fn pack(sizes: &[[u32; 2]], turn: bool) -> (Vec<[u32; 2]>, Vec<Spot>) {
    // Not on a page until placed.
    let mut spots = vec![Spot { page: usize::MAX, x: 0, y: 0, turned: false }; sizes.len()];
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse((sizes[i][0].max(sizes[i][1]), sizes[i][0] * sizes[i][1])));
    let mut pages = Vec::new();
    let mut left = order;
    while !left.is_empty() {
        let area: u64 = left.iter().map(|&i| u64::from(sizes[i][0] + PAD) * u64::from(sizes[i][1] + PAD)).sum();
        let widest =
            left.iter().map(|&i| sizes[i][0].min(if turn { sizes[i][1] } else { u32::MAX })).max().unwrap_or(1);
        let mut side = ((area as f64).sqrt() as u32).max(widest).max(16).next_power_of_two();
        let mut tries = Vec::new();
        while side <= MAX_PAGE {
            tries.push([side, side / 2]);
            tries.push([side, side]);
            side *= 2;
        }
        let page = pages.len();
        let mut done = false;
        for [w, h] in tries {
            if let Some(placed) = try_pack(&left, sizes, w, h, turn, true) {
                for (i, s) in placed {
                    spots[i] = Spot { page, ..s };
                }
                left.clear();
                done = true;
                break;
            }
        }
        if !done {
            let placed = try_pack(&left, sizes, MAX_PAGE, MAX_PAGE, turn, false).unwrap_or_default();
            if placed.is_empty() {
                // Too big for any page: one of its own.
                let i = left.remove(0);
                spots[i] = Spot { page, x: 0, y: 0, turned: false };
            } else {
                let on: HashSet<usize> = placed.iter().map(|(i, _)| *i).collect();
                left.retain(|i| !on.contains(i));
                for (i, s) in placed {
                    spots[i] = Spot { page, ..s };
                }
            }
        }
        // The page is as big as what's on it.
        let mut extent = [1, 1];
        for (i, s) in spots.iter().enumerate() {
            if s.page == page {
                let [w, h] = if s.turned { [sizes[i][1], sizes[i][0]] } else { sizes[i] };
                extent = [extent[0].max(s.x + w), extent[1].max(s.y + h)];
            }
        }
        pages.push(extent);
    }
    (pages, spots)
}

/// Pack `items` into a `w`×`h` page. With `all`, every one must fit, or `None`.
fn try_pack(items: &[usize], sizes: &[[u32; 2]], w: u32, h: u32, turn: bool, all: bool) -> Option<Vec<(usize, Spot)>> {
    let mut bin = Bin::new(w + PAD, h + PAD);
    let mut out = Vec::new();
    for &i in items {
        let [iw, ih] = sizes[i];
        match bin.find(iw + PAD, ih + PAD, turn) {
            Some((x, y, turned)) => {
                let (bw, bh) = if turned { (ih + PAD, iw + PAD) } else { (iw + PAD, ih + PAD) };
                bin.place([x, y, bw, bh]);
                out.push((i, Spot { page: 0, x, y, turned }));
            }
            None if all => return None,
            None => {}
        }
    }
    debug_assert!(bin.w > 0 && bin.h > 0);
    Some(out)
}

/// A sprite as repacked: its unique image, its canvas size and where the image sits on it.
type Repacked = (usize, [u32; 2], [i64; 2]);

/// Pack the sheet's sprites again: each its upright pixels (trimmed again of any transparent margin when the format
/// can say so, otherwise its whole canvas), identical ones stored once, turned only the way the format can say.
fn repack(sheet: &Sheet, images: &[RgbaImage], rot: Rot, trim: bool) -> (Sheet, Vec<RgbaImage>) {
    let mut uniques: Vec<RgbaImage> = Vec::new();
    let mut by_hash: HashMap<blake3::Hash, usize> = HashMap::new();
    let mut each: Vec<Option<Repacked>> = Vec::with_capacity(sheet.sprites.len());
    for s in &sheet.sprites {
        let Some(page) = images.get(s.page) else {
            each.push(None);
            continue;
        };
        let got = if trim {
            cut(page, s).map(|px| {
                let (px, [dx, dy]) = trim_transparent(px);
                (px, s.size, [s.at[0] + dx, s.at[1] + dy])
            })
        } else {
            whole(page, s).map(|px| {
                let size = [px.width(), px.height()];
                (px, size, [0, 0])
            })
        };
        let Some((px, size, at)) = got else {
            each.push(None);
            continue;
        };
        let mut h = blake3::Hasher::new();
        h.update(&px.width().to_le_bytes());
        h.update(&px.height().to_le_bytes());
        h.update(px.as_raw());
        let key = h.finalize();
        let u = *by_hash.entry(key).or_insert_with(|| {
            uniques.push(px);
            uniques.len() - 1
        });
        each.push(Some((u, size, at)));
    }
    let sizes: Vec<[u32; 2]> = uniques.iter().map(|i| [i.width(), i.height()]).collect();
    let turn_as = rot.turn();
    let (page_sizes, spots) = pack(&sizes, turn_as.is_some());
    let mut pages: Vec<RgbaImage> = page_sizes.iter().map(|[w, h]| RgbaImage::new(*w, *h)).collect();
    for (u, img) in uniques.iter().enumerate() {
        let s = spots[u];
        let stored = match (s.turned, turn_as) {
            (true, Some(270)) => imageops::rotate270(img),
            (true, _) => imageops::rotate90(img),
            (false, _) => img.clone(),
        };
        imageops::replace(&mut pages[s.page], &stored, i64::from(s.x), i64::from(s.y));
    }
    let mut index = vec![None; sheet.sprites.len()];
    let mut sprites = Vec::new();
    for (i, (s, e)) in sheet.sprites.iter().zip(&each).enumerate() {
        let Some((u, size, at)) = *e else { continue };
        let spot = spots[u];
        let [w, h] = sizes[u];
        let rect = if spot.turned { [spot.x, spot.y, h, w] } else { [spot.x, spot.y, w, h] };
        let turn = if spot.turned { turn_as.unwrap_or(0) } else { 0 };
        index[i] = Some(sprites.len());
        sprites.push(Sprite { name: s.name.clone(), page: spot.page, rect, turn, size, at });
    }
    let animations = sheet
        .animations
        .iter()
        .filter_map(|a| {
            let frames: Vec<AnimFrame> = a
                .frames
                .iter()
                .filter_map(|f| Some(AnimFrame { sprite: index.get(f.sprite).copied().flatten()?, ..*f }))
                .collect();
            (!frames.is_empty()).then(|| Anim { frames, ..a.clone() })
        })
        .collect();
    let out_pages =
        page_sizes.iter().map(|&size| Page { image: String::new(), size, asset: None, hash: None }).collect();
    (Sheet { pages: out_pages, sprites, animations, ..sheet.clone() }, pages)
}

/// An image without its transparent margin (at least a pixel left), and where what's left starts.
fn trim_transparent(img: RgbaImage) -> (RgbaImage, [i64; 2]) {
    let (w, h) = img.dimensions();
    let (mut x0, mut y0, mut x1, mut y1) = (w, h, 0, 0);
    for (x, y, p) in img.enumerate_pixels() {
        if p[3] != 0 {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x + 1);
            y1 = y1.max(y + 1);
        }
    }
    if x1 <= x0 || y1 <= y0 {
        return (imageops::crop_imm(&img, 0, 0, 1, 1).to_image(), [0, 0]);
    }
    if [x0, y0, x1, y1] == [0, 0, w, h] {
        return (img, [0, 0]);
    }
    (imageops::crop_imm(&img, x0, y0, x1 - x0, y1 - y0).to_image(), [i64::from(x0), i64::from(y0)])
}

// ---------------------------------------------------------------------------------------------
// Writers

/// A frame for TexturePacker's JSON: `frame` upright (`w` and `h` swapped from the stored region when turned).
fn tp_frame(s: &Sprite) -> serde_json::Map<String, Value> {
    let [x, y, _, _] = s.rect;
    let [w, h] = s.upright();
    let mut m = serde_json::Map::new();
    m.insert("frame".into(), json!({ "x": x, "y": y, "w": w, "h": h }));
    m.insert("rotated".into(), json!(s.turn != 0));
    m.insert("trimmed".into(), json!(s.trimmed()));
    m.insert("spriteSourceSize".into(), json!({ "x": s.at[0], "y": s.at[1], "w": w, "h": h }));
    m.insert("sourceSize".into(), json!({ "w": s.size[0], "h": s.size[1] }));
    m
}

/// TexturePacker's JSON, a file per page, with the animations (`PixiJS`'s `animations`) in the file whose page holds
/// their first frame.
fn texture_packer(sheet: &Sheet, stem: &str, array: bool) -> Files {
    let one = sheet.pages.len() == 1;
    let name = |i: usize| if one { format!("{stem}.json") } else { format!("{stem}-{i}.json") };
    (0..sheet.pages.len())
        .map(|p| {
            let on: Vec<&Sprite> = sheet.sprites.iter().filter(|s| s.page == p).collect();
            let frames = if array {
                Value::Array(
                    on.iter()
                        .map(|s| {
                            let mut m = serde_json::Map::new();
                            m.insert("filename".into(), json!(s.name));
                            m.extend(tp_frame(s));
                            Value::Object(m)
                        })
                        .collect(),
                )
            } else {
                Value::Object(on.iter().map(|s| (s.name.clone(), Value::Object(tp_frame(s)))).collect())
            };
            let page = &sheet.pages[p];
            let mut meta = json!({
                "app": "Hoard",
                "version": "1.0",
                "image": page.image,
                "format": "RGBA8888",
                "size": { "w": page.size[0], "h": page.size[1] },
                "scale": "1",
            });
            if !one {
                meta["related_multi_packs"] =
                    json!((0..sheet.pages.len()).filter(|&i| i != p).map(name).collect::<Vec<_>>());
            }
            let mut doc = json!({ "frames": frames });
            let anims: serde_json::Map<String, Value> = sheet
                .animations
                .iter()
                .filter(|a| a.frames.first().is_some_and(|f| sheet.sprites[f.sprite].page == p))
                .map(|a| {
                    (a.name.clone(), json!(a.frames.iter().map(|f| &sheet.sprites[f.sprite].name).collect::<Vec<_>>()))
                })
                .collect();
            if !anims.is_empty() {
                doc["animations"] = Value::Object(anims);
            }
            doc["meta"] = meta;
            (name(p), serde_json::to_vec_pretty(&doc).unwrap_or_default())
        })
        .collect()
}

fn xml_attr(s: &str) -> String {
    quick_xml::escape::escape(s).into_owned()
}

/// Starling and Sparrow's XML, a file per page: `width` and `height` as stored, and the canvas's start relative to
/// the sprite (`frameX`, `frameY`, so not above zero).
fn starling(sheet: &Sheet, stem: &str) -> Files {
    let one = sheet.pages.len() == 1;
    (0..sheet.pages.len())
        .map(|p| {
            let mut x = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
            let _ = writeln!(x, "<TextureAtlas imagePath=\"{}\">", xml_attr(&sheet.pages[p].image));
            for s in sheet.sprites.iter().filter(|s| s.page == p) {
                let [rx, ry, w, h] = s.rect;
                let _ = write!(
                    x,
                    "  <SubTexture name=\"{}\" x=\"{rx}\" y=\"{ry}\" width=\"{w}\" height=\"{h}\"",
                    xml_attr(&s.name)
                );
                if s.turn != 0 {
                    x.push_str(" rotated=\"true\"");
                }
                if s.trimmed() {
                    let _ = write!(
                        x,
                        " frameX=\"{}\" frameY=\"{}\" frameWidth=\"{}\" frameHeight=\"{}\"",
                        -s.at[0], -s.at[1], s.size[0], s.size[1]
                    );
                }
                x.push_str("/>\n");
            }
            x.push_str("</TextureAtlas>\n");
            (if one { format!("{stem}.xml") } else { format!("{stem}-{p}.xml") }, x.into_bytes())
        })
        .collect()
}

/// libGDX's `.atlas` (the format every version reads): `size` upright, `offset` from the bottom left.
fn libgdx(sheet: &Sheet) -> String {
    let mut x = String::new();
    for (p, page) in sheet.pages.iter().enumerate() {
        let _ = write!(
            x,
            "\n{}\nsize: {}, {}\nformat: RGBA8888\nfilter: Linear,Linear\nrepeat: none\n",
            page.image, page.size[0], page.size[1]
        );
        for s in sheet.sprites.iter().filter(|s| s.page == p) {
            let [rx, ry, _, _] = s.rect;
            let [w, h] = s.upright();
            let oy = i64::from(s.size[1]) - s.at[1] - i64::from(h);
            let _ = write!(
                x,
                "{}\n  rotate: {}\n  xy: {rx}, {ry}\n  size: {w}, {h}\n  orig: {}, {}\n  offset: {}, {oy}\n  index: -1\n",
                s.name.replace(['\n', '\r'], " "),
                s.turn != 0,
                s.size[0],
                s.size[1],
                s.at[0],
            );
        }
    }
    x
}

/// A number as Cocos2d and Godot write it: whole numbers without a fraction.
fn num(n: f64) -> String {
    if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{n}") }
}

/// Cocos2d's `.plist` (format 3), a file per page: `textureRect` upright, the offset from the canvas's center, y up.
fn cocos(sheet: &Sheet, stem: &str) -> Files {
    let one = sheet.pages.len() == 1;
    (0..sheet.pages.len())
        .map(|p| {
            let page = &sheet.pages[p];
            let mut x = String::from(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
                 \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n<key>frames</key>\n<dict>\n",
            );
            for s in sheet.sprites.iter().filter(|s| s.page == p) {
                let [rx, ry, _, _] = s.rect;
                let [w, h] = s.upright();
                let ox = s.at[0] as f64 + f64::from(w) / 2.0 - f64::from(s.size[0]) / 2.0;
                let oy = -(s.at[1] as f64 + f64::from(h) / 2.0 - f64::from(s.size[1]) / 2.0);
                let _ = write!(
                    x,
                    "<key>{}</key>\n<dict>\n<key>aliases</key><array/>\n\
                     <key>spriteOffset</key><string>{{{},{}}}</string>\n\
                     <key>spriteSize</key><string>{{{w},{h}}}</string>\n\
                     <key>spriteSourceSize</key><string>{{{},{}}}</string>\n\
                     <key>textureRect</key><string>{{{{{rx},{ry}}},{{{w},{h}}}}}</string>\n\
                     <key>textureRotated</key><{}/>\n</dict>\n",
                    xml_attr(&s.name),
                    num(ox),
                    num(oy + 0.0),
                    s.size[0],
                    s.size[1],
                    s.turn != 0,
                );
            }
            let _ = write!(
                x,
                "</dict>\n<key>metadata</key>\n<dict>\n<key>format</key><integer>3</integer>\n\
                 <key>pixelFormat</key><string>RGBA8888</string>\n<key>premultiplyAlpha</key><false/>\n\
                 <key>realTextureFileName</key><string>{0}</string>\n<key>size</key><string>{{{1},{2}}}</string>\n\
                 <key>textureFileName</key><string>{0}</string>\n</dict>\n</dict>\n</plist>\n",
                xml_attr(&page.image),
                page.size[0],
                page.size[1],
            );
            (if one { format!("{stem}.plist") } else { format!("{stem}-{p}.plist") }, x.into_bytes())
        })
        .collect()
}

fn gd_num(n: f64) -> String {
    if n.fract() == 0.0 { format!("{n:.1}") } else { format!("{n}") }
}

/// Godot 4's `SpriteFrames`: an `AtlasTexture` per sprite (its trim as a `margin`), its animations (the sprites none
/// of them shows in one more, `sprites`, or all of them as `default` when it has none). Images are named relative to
/// the file, which Godot reads beside it.
fn godot_frames(sheet: &Sheet) -> String {
    let mut animations: Vec<Anim> = sheet.animations.clone();
    let used: HashSet<usize> = animations.iter().flat_map(|a| a.frames.iter().map(|f| f.sprite)).collect();
    let rest: Vec<AnimFrame> = (0..sheet.sprites.len())
        .filter(|i| !used.contains(i))
        .map(|sprite| AnimFrame { sprite, ms: atlas::FRAME_MS })
        .collect();
    if !rest.is_empty() {
        let name = if animations.is_empty() { "default" } else { "sprites" };
        animations.push(Anim { name: name.into(), frames: rest, looped: animations.is_empty() });
    }
    let mut x = format!(
        "[gd_resource type=\"SpriteFrames\" load_steps={} format=3]\n\n",
        sheet.pages.len() + sheet.sprites.len() + 1
    );
    for (p, page) in sheet.pages.iter().enumerate() {
        let _ = writeln!(
            x,
            "[ext_resource type=\"Texture2D\" path={} id=\"{}_page\"]",
            crate::godot::quote(&page.image),
            p + 1
        );
    }
    for (i, s) in sheet.sprites.iter().enumerate() {
        let [rx, ry, w, h] = s.rect;
        let _ = write!(
            x,
            "\n[sub_resource type=\"AtlasTexture\" id=\"AtlasTexture_{i}\"]\natlas = ExtResource(\"{}_page\")\n\
             region = Rect2({rx}, {ry}, {w}, {h})\n",
            s.page + 1
        );
        if s.trimmed() {
            let _ = writeln!(
                x,
                "margin = Rect2({}, {}, {}, {})",
                s.at[0],
                s.at[1],
                i64::from(s.size[0]) - i64::from(w),
                i64::from(s.size[1]) - i64::from(h)
            );
        }
    }
    x.push_str("\n[resource]\nanimations = [");
    for (k, a) in animations.iter().enumerate() {
        // The speed is the most common frame's; each frame lasts a multiple of it.
        let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
        for f in &a.frames {
            *counts.entry(f.ms.max(1)).or_default() += 1;
        }
        let base =
            counts.iter().max_by_key(|(ms, n)| (**n, std::cmp::Reverse(**ms))).map_or(atlas::FRAME_MS, |(ms, _)| *ms);
        if k > 0 {
            x.push_str(", ");
        }
        x.push_str("{\n\"frames\": [");
        for (j, f) in a.frames.iter().enumerate() {
            if j > 0 {
                x.push_str(", ");
            }
            let _ = write!(
                x,
                "{{\n\"duration\": {},\n\"texture\": SubResource(\"AtlasTexture_{}\")\n}}",
                gd_num((f64::from(f.ms.max(1)) / f64::from(base) * 1000.0).round() / 1000.0),
                f.sprite
            );
        }
        let _ = write!(
            x,
            "],\n\"loop\": {},\n\"name\": &{},\n\"speed\": {}\n}}",
            a.looped,
            crate::godot::quote(&a.name),
            gd_num((1000.0 / f64::from(base) * 1000.0).round() / 1000.0)
        );
    }
    x.push_str("]\n");
    x
}

/// A sheet laid out as a grid.
struct Grid {
    cell: [u32; 2],
    pages: Vec<RgbaImage>,
    /// Each sprite's image and cell (column, row).
    cells: Vec<(usize, [u32; 2])>,
}

/// The sheet laid out as a grid: every sprite on its whole canvas, centered in a cell as big as the biggest (or the
/// tile size), in the sheet's order, rows filling images of at most [`MAX_GRID`] a side.
fn grid(sheet: &Sheet, images: &[RgbaImage]) -> Grid {
    let canvases: Vec<Option<RgbaImage>> =
        sheet.sprites.iter().map(|s| images.get(s.page).and_then(|p| whole(p, s))).collect();
    let mut cell = sheet.tile.unwrap_or([1, 1]);
    for c in canvases.iter().flatten() {
        cell = [cell[0].max(c.width()), cell[1].max(c.height())];
    }
    let cell = [cell[0].min(MAX_GRID), cell[1].min(MAX_GRID)];
    let n = canvases.iter().flatten().count().max(1) as u32;
    let cols = ((f64::from(n).sqrt().ceil() as u32).max(1)).min((MAX_GRID / cell[0]).max(1));
    let rows_per = (MAX_GRID / cell[1]).max(1);
    let per_page = cols * rows_per;
    let pages_n = n.div_ceil(per_page);
    let mut pages: Vec<RgbaImage> = (0..pages_n)
        .map(|p| {
            let here = (n - p * per_page).min(per_page);
            RgbaImage::new(cell[0] * cols.min(here), cell[1] * here.div_ceil(cols))
        })
        .collect();
    let mut cells = Vec::new();
    for (k, c) in canvases.iter().flatten().enumerate() {
        let k = k as u32;
        let (p, i) = (k / per_page, k % per_page);
        let (cx, cy) = (i % cols, i / cols);
        let x = cx * cell[0] + (cell[0] - c.width().min(cell[0])) / 2;
        let y = cy * cell[1] + (cell[1] - c.height().min(cell[1])) / 2;
        imageops::replace(&mut pages[p as usize], c, i64::from(x), i64::from(y));
        cells.push((p as usize, [cx, cy]));
    }
    Grid { cell, pages, cells }
}

/// A grid image (`<name>_<cell width>x<cell height>.png`, numbered when there are several).
fn grid_files(sheet: &Sheet, images: &[RgbaImage], stem: &str) -> Result<Files> {
    let Grid { cell: [w, h], pages, .. } = grid(sheet, images);
    let one = pages.len() == 1;
    pages
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            let name = if one { format!("{stem}_{w}x{h}.png") } else { format!("{stem}_{w}x{h}_{i}.png") };
            Ok((name, crate::analyze::png_bytes(p)?))
        })
        .collect()
}

/// Every sprite as a PNG of its own, named after it, in a folder named after the sheet.
fn frame_files(sheet: &Sheet, images: &[RgbaImage], stem: &str) -> Result<Files> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for s in &sheet.sprites {
        let Some(img) = images.get(s.page).and_then(|p| whole(p, s)) else { continue };
        let base = clean_name(image_stem(&s.name)).unwrap_or_else(|| "sprite".into());
        let mut name = format!("{stem}/{base}.png");
        let mut n = 2;
        while !seen.insert(name.to_lowercase()) {
            name = format!("{stem}/{base}_{n}.png");
            n += 1;
        }
        out.push((name, crate::analyze::png_bytes(img)?));
    }
    Ok(out)
}

/// An image's tiles: each its cell and how many cells it covers.
type Tiles = Vec<([u32; 2], [u32; 2])>;

/// Godot 4's `TileSet`: a `TileSetAtlasSource` per image. A tile set whose tiles already sit on a grid of its tile
/// size keeps its images (a tile bigger than a cell given its `size_in_atlas`); anything else is laid out as a grid.
fn godot_tileset(sheet: &Sheet, images: &[RgbaImage], originals: &[Option<Vec<u8>>], stem: &str) -> Result<Files> {
    let on_grid = sheet.tile.filter(|&[tw, th]| {
        tw > 0
            && th > 0
            && sheet.sprites.iter().all(|s| {
                let [x, y, w, h] = s.rect;
                s.turn == 0 && !s.trimmed() && x % tw == 0 && y % th == 0 && w % tw == 0 && h % th == 0
            })
    });
    // Per image: its file, and its tiles (cell, cells it covers).
    let (tile, pngs, tiles): ([u32; 2], Vec<Vec<u8>>, Vec<Tiles>) = if let Some([tw, th]) = on_grid {
        let pngs = images
            .iter()
            .zip(originals)
            .map(|(img, orig)| orig.clone().map_or_else(|| crate::analyze::png_bytes(img.clone()), Ok))
            .collect::<Result<Vec<_>>>()?;
        let mut tiles = vec![Vec::new(); sheet.pages.len()];
        for s in &sheet.sprites {
            let [x, y, w, h] = s.rect;
            tiles[s.page].push(([x / tw, y / th], [w / tw, h / th]));
        }
        ([tw, th], pngs, tiles)
    } else {
        let Grid { cell, pages, cells } = grid(sheet, images);
        let mut tiles = vec![Vec::new(); pages.len()];
        for (p, c) in cells {
            tiles[p].push((c, [1, 1]));
        }
        let pngs = pages.into_iter().map(crate::analyze::png_bytes).collect::<Result<Vec<_>>>()?;
        (cell, pngs, tiles)
    };
    let one = pngs.len() == 1;
    let name = |i: usize| if one { format!("{stem}.png") } else { format!("{stem}-{i}.png") };
    let mut x = format!("[gd_resource type=\"TileSet\" load_steps={} format=3]\n\n", pngs.len() * 2 + 1);
    for i in 0..pngs.len() {
        let _ = writeln!(
            x,
            "[ext_resource type=\"Texture2D\" path={} id=\"{}_page\"]",
            crate::godot::quote(&name(i)),
            i + 1
        );
    }
    for (i, list) in tiles.iter().enumerate() {
        let _ = write!(
            x,
            "\n[sub_resource type=\"TileSetAtlasSource\" id=\"TileSetAtlasSource_{i}\"]\ntexture = ExtResource(\"{}_page\")\n\
             texture_region_size = Vector2i({}, {})\n",
            i + 1,
            tile[0],
            tile[1]
        );
        let mut seen = HashSet::new();
        for ([cx, cy], [w, h]) in list {
            if !seen.insert([*cx, *cy]) {
                continue;
            }
            if [*w, *h] != [1, 1] {
                let _ = writeln!(x, "{cx}:{cy}/size_in_atlas = Vector2i({w}, {h})");
            }
            let _ = writeln!(x, "{cx}:{cy}/0 = 0");
        }
    }
    let _ = write!(x, "\n[resource]\ntile_size = Vector2i({}, {})\n", tile[0], tile[1]);
    for i in 0..pngs.len() {
        let _ = writeln!(x, "sources/{i} = SubResource(\"TileSetAtlasSource_{i}\")");
    }
    let mut files = vec![(format!("{stem}.tres"), x.into_bytes())];
    files.extend(pngs.into_iter().enumerate().map(|(i, b)| (name(i), b)));
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    /// A sprite whose every pixel differs, so a wrong turn or offset shows.
    fn art(w: u32, h: u32, seed: u8) -> RgbaImage {
        RgbaImage::from_fn(w, h, |x, y| Rgba([x as u8 * 20 + seed, y as u8 * 30 + 5, seed.wrapping_mul(3), 255]))
    }

    /// A page with three sprites: `hero` upright, `coin` stored turned clockwise, `gem` trimmed onto a bigger
    /// canvas, and their pictures as drawn.
    fn sample() -> (Sheet, RgbaImage, Vec<(String, RgbaImage)>) {
        let hero = art(4, 6, 10);
        let coin = art(5, 3, 60);
        let gem = art(3, 3, 120);
        let mut page = RgbaImage::new(16, 16);
        imageops::replace(&mut page, &hero, 0, 0);
        imageops::replace(&mut page, &imageops::rotate90(&coin), 6, 0);
        imageops::replace(&mut page, &gem, 10, 0);
        let mut gem_whole = RgbaImage::new(7, 5);
        imageops::replace(&mut gem_whole, &gem, 2, 1);
        let sheet = Sheet {
            version: VERSION,
            from: "test".into(),
            pages: vec![Page { image: "page.png".into(), size: [16, 16], asset: None, hash: None }],
            sprites: vec![
                Sprite { name: "hero_1".into(), page: 0, rect: [0, 0, 4, 6], turn: 0, size: [4, 6], at: [0, 0] },
                Sprite { name: "hero_2".into(), page: 0, rect: [6, 0, 3, 5], turn: 90, size: [5, 3], at: [0, 0] },
                Sprite { name: "gem".into(), page: 0, rect: [10, 0, 3, 3], turn: 0, size: [7, 5], at: [2, 1] },
            ],
            animations: vec![Anim {
                name: "hero".into(),
                frames: vec![AnimFrame { sprite: 0, ms: 100 }, AnimFrame { sprite: 1, ms: 200 }],
                looped: true,
            }],
            tile: None,
        };
        let drawn = vec![("hero_1".into(), hero), ("hero_2".into(), coin), ("gem".into(), gem_whole)];
        (sheet, page, drawn)
    }

    fn files_of(format: &str) -> Files {
        let (sheet, page, _) = sample();
        let t = TARGETS.iter().find(|t| t.format == format).expect("a target");
        write_format(&sheet, &[page], &[None], t, "hero").expect("written")
    }

    fn png(files: &Files, name: &str) -> RgbaImage {
        let bytes = &files.iter().find(|(n, _)| n == name).unwrap_or_else(|| panic!("no {name}")).1;
        image::load_from_memory(bytes).expect("a png").to_rgba8()
    }

    #[test]
    fn every_atlas_format_reads_back_as_drawn() {
        let (_, _, drawn) = sample();
        for (format, file, ext) in [
            ("sheet-tp-hash", "hero.json", "json"),
            ("sheet-tp-array", "hero.json", "json"),
            ("sheet-starling", "hero.xml", "xml"),
            ("sheet-libgdx", "hero.atlas", "atlas"),
            ("sheet-cocos2d", "hero.plist", "plist"),
            ("sheet-godot-frames", "hero.tres", "tres"),
        ] {
            let files = files_of(format);
            let text = String::from_utf8(files.iter().find(|(n, _)| n == file).expect(file).1.clone()).unwrap();
            let read = atlas::parse(ext, &text).unwrap_or_else(|| panic!("{format} reads back:\n{text}"));
            let page = png(&files, &read.pages[0].image.replace("res://", ""));
            // Godot names its sprites after their animation's frames: they're found in the order its frames show.
            let in_order: Vec<&atlas::Sprite> =
                read.animations.iter().flat_map(|a| &a.frames).map(|f| &read.pages[f.page].sprites[f.sprite]).collect();
            for (k, (name, want)) in drawn.iter().enumerate() {
                let s = if format == "sheet-godot-frames" {
                    in_order.get(k).copied()
                } else {
                    read.pages.iter().flat_map(|p| &p.sprites).find(|s| s.name.starts_with(name.as_str()))
                };
                let s = s.unwrap_or_else(|| panic!("{format}: no {name} in\n{text}"));
                assert_eq!(atlas::cut(&page, s).as_ref(), Some(want), "{format}: {name}");
            }
            if format == "sheet-godot-frames" {
                assert_eq!(read.animations.len(), 2, "hero, and the sprites no animation shows");
                assert_eq!(read.animations[0].frames.iter().map(|f| f.ms).collect::<Vec<_>>(), [100, 200]);
            }
        }
    }

    #[test]
    fn turned_only_where_the_format_can_say() {
        let read = |format: &str, file: &str, ext: &str| {
            let files = files_of(format);
            atlas::parse(ext, &String::from_utf8(files.iter().find(|(n, _)| n == file).unwrap().1.clone()).unwrap())
                .unwrap()
        };
        let turns = |a: &Atlas| a.pages.iter().flat_map(|p| &p.sprites).map(|s| s.turn).collect::<Vec<_>>();
        // TexturePacker keeps the sheet as it is: its sprite turned clockwise stays so.
        assert!(turns(&read("sheet-tp-hash", "hero.json", "json")).contains(&Turn::Clockwise));
        // libGDX can only turn counter-clockwise, Godot not at all: packed again.
        assert!(!turns(&read("sheet-libgdx", "hero.atlas", "atlas")).contains(&Turn::Clockwise));
        assert!(turns(&read("sheet-godot-frames", "hero.tres", "tres")).iter().all(|t| *t == Turn::Upright));
    }

    #[test]
    fn hoard_sheet_round_trips() {
        let (sheet, _, _) = sample();
        let files = files_of("sheet-hoard");
        let back: Sheet =
            serde_json::from_slice(&files.iter().find(|(n, _)| n == "hero.hoardsheet").unwrap().1).unwrap();
        assert_eq!(back.sprites, sheet.sprites);
        assert_eq!(back.animations, sheet.animations);
        assert_eq!(back.pages[0].image, "hero.png");
        assert!(files.iter().any(|(n, _)| n == "hero.png"));
    }

    #[test]
    fn grids_frames_and_tile_sets() {
        let (_, _, drawn) = sample();
        let files = files_of("sheet-grid");
        assert_eq!(files.len(), 1);
        let grid = png(&files, "hero_7x6.png");
        assert_eq!(grid.dimensions(), (14, 12), "2×2 cells of the biggest canvas");

        let files = files_of("sheet-frames");
        for (name, want) in &drawn {
            assert_eq!(&png(&files, &format!("hero/{name}.png")), want);
        }

        let files = files_of("sheet-godot-tileset");
        let text = String::from_utf8(files[0].1.clone()).unwrap();
        let read = atlas::parse("tres", &text).expect("a tile set");
        assert_eq!(read.tile, Some([7, 6]));
        assert_eq!(read.pages[0].sprites.len(), 3);
    }

    #[test]
    fn a_tile_set_on_its_grid_keeps_its_image() {
        let page = art(8, 4, 1);
        let sprites = (0..2)
            .map(|i| Sprite {
                name: format!("tile_{i}"),
                page: 0,
                rect: [i * 4, 0, 4, 4],
                turn: 0,
                size: [4, 4],
                at: [0, 0],
            })
            .collect();
        let sheet = Sheet {
            version: VERSION,
            from: "godot-tileset".into(),
            pages: vec![Page { image: "t.png".into(), size: [8, 4], asset: None, hash: None }],
            sprites,
            animations: Vec::new(),
            tile: Some([4, 4]),
        };
        let t = TARGETS.iter().find(|t| t.format == "sheet-godot-tileset").unwrap();
        let files = write_format(&sheet, std::slice::from_ref(&page), &[None], t, "t").unwrap();
        assert_eq!(png(&files, "t.png"), page);
        let text = String::from_utf8(files[0].1.clone()).unwrap();
        assert!(text.contains("1:0/0 = 0"), "{text}");
    }

    #[test]
    fn packing_fills_pages_without_overlap() {
        let sizes: Vec<[u32; 2]> = (0..200).map(|i| [8 + i % 13 * 3, 6 + i % 7 * 5]).collect();
        for turn in [false, true] {
            let (pages, spots) = pack(&sizes, turn);
            assert_eq!(pages.len(), 1);
            let rects: Vec<[u32; 4]> = spots
                .iter()
                .zip(&sizes)
                .map(|(s, [w, h])| if s.turned { [s.x, s.y, *h, *w] } else { [s.x, s.y, *w, *h] })
                .collect();
            for (i, a) in rects.iter().enumerate() {
                assert!(a[0] + a[2] <= pages[0][0] && a[1] + a[3] <= pages[0][1]);
                for b in &rects[i + 1..] {
                    let apart = a[0] + a[2] + PAD <= b[0]
                        || b[0] + b[2] + PAD <= a[0]
                        || a[1] + a[3] + PAD <= b[1]
                        || b[1] + b[3] + PAD <= a[1];
                    assert!(apart, "{a:?} and {b:?} overlap");
                }
            }
        }
        // More than a page holds: more pages.
        let (pages, spots) = pack(&[[3000, 3000]; 3], false);
        assert_eq!(pages, [[3000, 3000]; 3], "each page as big as what's on it");
        assert_eq!(spots.iter().map(|s| s.page).collect::<HashSet<_>>().len(), 3);
    }

    #[test]
    fn numbered_names_are_animations() {
        let s =
            |name: &str| Sprite { name: name.into(), page: 0, rect: [0, 0, 1, 1], turn: 0, size: [1, 1], at: [0, 0] };
        let sprites =
            vec![s("walk_10.png"), s("walk_2.png"), s("walk_1.png"), s("idle"), s("jump 1"), s("run01"), s("run02")];
        let a = guess_animations(&sprites);
        let names: Vec<&str> = a.iter().map(|a| a.name.as_str()).collect();
        assert_eq!(names, ["run", "walk"]);
        assert_eq!(a[1].frames.iter().map(|f| f.sprite).collect::<Vec<_>>(), [2, 1, 0]);
    }
}
