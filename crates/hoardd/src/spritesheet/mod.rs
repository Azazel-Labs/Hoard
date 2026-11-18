//! Hoard's own sprite sheets. An atlas file found in a pack (see [`crate::atlas`]) is kept as a [`Sheet`]: its pages
//! (the pack's images, which stay where they are), each sprite's place on them, its trim and how it's turned, and
//! its animations (the file's own, or guessed from numbered names). The sheet is stored with the atlas file's asset,
//! which becomes a `sheet`, and it's what the sheet's page, thumbnail and downloads are made from.
//!
//! A sheet downloads in many formats. Each says what it can describe (sprites turned one way, or any right angle, or
//! not at all; trimmed sprites or not), and the sprites are packed again only when the sheet uses something the format
//! can't say, or when its sprites are each an image of their own.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

mod pack;
mod writers;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{Cursor, Write as _};
use std::path::Path;

use anyhow::{Context as _, Result, bail};
use image::{RgbaImage, imageops};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::atlas::{self, Atlas, Turn};
use crate::formats::Output;
use pack::repack;
use writers::{cocos, frame_files, godot_frames, godot_tileset, grid_files, libgdx, starling, texture_packer};

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
