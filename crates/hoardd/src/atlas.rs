//! Sprite atlases: one image (a page) packed with many sprites, and a file that says where each one is. Starling and
//! Sparrow XML, TexturePacker JSON (hash, array and multipack, and Aseprite's export, which is the same), libGDX and
//! Spine `.atlas` files and Cocos2d `.plist` files are read.
//!
//! Each sprite becomes an asset of its own: a PNG cut from the page, put upright and given back the transparent margin
//! the packer trimmed off, at `<atlas folder>/<atlas name>/<sprite name>.png`, with `sprite` in its meta saying where
//! it came from. A sprite the pack already has as a loose image (the same pixels, or the same name and shape) is that
//! image instead of a copy. Either way the page links to it (`sprite`), and the atlas file links to its page
//! (`texture`), so the atlas's download brings the image it describes.
//!
//! Atlases are looked for after materials, when a pack is ingested or re-tagged and in the `materials` job.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

use std::collections::{HashMap, HashSet};

use anyhow::{Context as _, Result};
use hoard_core::{AssetKind, TagSource};
use image::{RgbaImage, imageops};
use quick_xml::events::{BytesStart, Event};
use rusqlite::{Connection, params};
use serde_json::{Value, json};
use tokio::runtime::Handle;

use crate::{AppState, db, tagger};

/// How the packer turned a sprite to fit it in, so how to turn it back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Turn {
    Upright,
    /// Stored turned clockwise: turned back counter-clockwise (TexturePacker, Starling, Cocos2d).
    Clockwise,
    /// Stored turned counter-clockwise: turned back clockwise (libGDX, Spine).
    CounterClockwise,
}

/// A sprite's transparent margin the packer trimmed off: the size it was, and where what's left sits in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Trim {
    pub(crate) size: [u32; 2],
    pub(crate) at: [i64; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sprite {
    pub(crate) name: String,
    /// Where it is in the page, as stored there (turned, if it was).
    pub(crate) rect: [u32; 4],
    pub(crate) turn: Turn,
    pub(crate) trim: Option<Trim>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Page {
    /// The image, as the atlas names it (relative to the atlas file).
    pub(crate) image: String,
    pub(crate) sprites: Vec<Sprite>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Atlas {
    pub(crate) format: &'static str,
    pub(crate) pages: Vec<Page>,
}

/// A sprite cut from an atlas, as SQL over an asset `a`.
pub(crate) const IS_SPRITE: &str = "(json_extract(a.meta, '$.sprite') IS NOT NULL)";

/// Bigger files aren't atlases.
const MAX_FILE: i64 = 8 << 20;
/// Sprites one pack's atlases may make, all told.
const MAX_SPRITES: usize = 20_000;
/// Pages and sprites bigger than this (in either direction) aren't cut.
const MAX_SIDE: u32 = 16_384;

/// Read an atlas's file. `None` when it isn't one.
pub(crate) fn parse(ext: &str, text: &str) -> Option<Atlas> {
    let text = text.trim_start_matches('\u{feff}');
    let atlas = match ext {
        "xml" => starling(text),
        "json" => texture_packer(&serde_json::from_str(text).ok()?),
        "atlas" => libgdx(text),
        "plist" => cocos(&plist(text)?),
        _ => None,
    }?;
    let pages: Vec<Page> = atlas.pages.into_iter().filter(|p| !p.image.is_empty() && !p.sprites.is_empty()).collect();
    (!pages.is_empty()).then_some(Atlas { pages, ..atlas })
}

fn attrs(e: &BytesStart<'_>) -> HashMap<String, String> {
    e.attributes()
        .with_checks(false)
        .flatten()
        .map(|a| {
            let key = String::from_utf8_lossy(a.key.local_name().as_ref()).into_owned();
            let value = a
                .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                .map_or_else(|_| String::from_utf8_lossy(&a.value).into_owned(), Into::into);
            (key, value)
        })
        .collect()
}

fn num(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok().filter(|n| n.is_finite())
}

fn px(n: f64) -> Option<u32> {
    (n >= 0.0 && n <= f64::from(MAX_SIDE)).then(|| n.round() as u32)
}

/// A trim that changes something: a canvas bigger than the sprite, or the sprite not at its corner.
fn trim_of(size: [u32; 2], at: [i64; 2], upright: [u32; 2]) -> Option<Trim> {
    let size = [size[0].max(1), size[1].max(1)];
    (size != upright || at != [0, 0]).then_some(Trim { size, at })
}

/// Starling and Sparrow: `<TextureAtlas imagePath="…"><SubTexture name x y width height [frameX frameY frameWidth
/// frameHeight] [rotated]/>`. The rectangle is the region as stored; `frameX` and `frameY` are where the canvas starts
/// relative to the sprite, so negative.
fn starling(text: &str) -> Option<Atlas> {
    let mut reader = quick_xml::Reader::from_str(text);
    let mut page: Option<Page> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e) | Event::Empty(e)) => {
                let name = e.local_name();
                let a = attrs(&e);
                match (name.as_ref(), &mut page) {
                    (b"TextureAtlas", None) => {
                        page = Some(Page { image: a.get("imagePath")?.clone(), sprites: Vec::new() });
                    }
                    (_, None) => return None,
                    (b"SubTexture", Some(p)) => {
                        let get = |k: &str| a.get(k).and_then(|v| num(v));
                        let (Some(x), Some(y), Some(w), Some(h)) = (
                            get("x").and_then(px),
                            get("y").and_then(px),
                            get("width").and_then(px),
                            get("height").and_then(px),
                        ) else {
                            continue;
                        };
                        let rotated = a.get("rotated").is_some_and(|v| v.eq_ignore_ascii_case("true"));
                        let upright = if rotated { [h, w] } else { [w, h] };
                        let trim = match (get("frameWidth").and_then(px), get("frameHeight").and_then(px)) {
                            (Some(fw), Some(fh)) => trim_of(
                                [fw, fh],
                                [
                                    -get("frameX").unwrap_or(0.0).round() as i64,
                                    -get("frameY").unwrap_or(0.0).round() as i64,
                                ],
                                upright,
                            ),
                            _ => None,
                        };
                        p.sprites.push(Sprite {
                            name: a.get("name").cloned().unwrap_or_default(),
                            rect: [x, y, w, h],
                            turn: if rotated { Turn::Clockwise } else { Turn::Upright },
                            trim,
                        });
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => return None,
            _ => {}
        }
    }
    Some(Atlas { format: "starling", pages: vec![page?] })
}

fn field(v: &Value, k: &str) -> Option<f64> {
    v.get(k)?.as_f64().filter(|n| n.is_finite())
}

/// A TexturePacker frame: `frame` is where it is with its upright size (so stored `h`×`w` when turned),
/// `spriteSourceSize` where it sits in its untrimmed `sourceSize`.
fn tp_frame(name: &str, f: &Value) -> Option<Sprite> {
    let r = f.get("frame")?;
    let (x, y, w, h) = (field(r, "x")?, field(r, "y")?, px(field(r, "w")?)?, px(field(r, "h")?)?);
    let rotated = f.get("rotated").and_then(Value::as_bool).unwrap_or(false);
    let rect = if rotated { [px(x)?, px(y)?, h, w] } else { [px(x)?, px(y)?, w, h] };
    let trim = match (f.get("sourceSize"), f.get("spriteSourceSize")) {
        (Some(src), Some(at)) => trim_of(
            [px(field(src, "w")?)?, px(field(src, "h")?)?],
            [field(at, "x").unwrap_or(0.0).round() as i64, field(at, "y").unwrap_or(0.0).round() as i64],
            [w, h],
        ),
        _ => None,
    };
    Some(Sprite { name: name.to_owned(), rect, turn: if rotated { Turn::Clockwise } else { Turn::Upright }, trim })
}

/// The frames of a TexturePacker page: a map of name to frame (hash), or a list of frames with a `filename` (array).
fn tp_frames(frames: &Value) -> Vec<Sprite> {
    match frames {
        Value::Object(m) => m.iter().filter_map(|(name, f)| tp_frame(name, f)).collect(),
        Value::Array(list) => {
            list.iter().filter_map(|f| tp_frame(f.get("filename").and_then(Value::as_str).unwrap_or(""), f)).collect()
        }
        _ => Vec::new(),
    }
}

/// TexturePacker's JSON (and Aseprite's, which is the same): `{"frames": …, "meta": {"image": …}}`, or a multipack's
/// `{"textures": [{"image": …, "frames": [...]}]}`.
fn texture_packer(v: &Value) -> Option<Atlas> {
    let image = |o: &Value| o.get("image").and_then(Value::as_str).map(str::to_owned);
    let pages = if let Some(textures) = v.get("textures").and_then(Value::as_array) {
        textures.iter().filter_map(|t| Some(Page { image: image(t)?, sprites: tp_frames(t.get("frames")?) })).collect()
    } else {
        vec![Page { image: image(v.get("meta")?)?, sprites: tp_frames(v.get("frames")?) }]
    };
    Some(Atlas { format: "texturepacker", pages })
}

/// libGDX's and Spine's `.atlas`: a page's image file on a line of its own (first, or after a blank line), then its
/// settings and regions; a region is a name on a line of its own, then `key: value` lines. The old format gives `xy`,
/// `size` (upright), `orig` and `offset` (from the bottom left); the new one `bounds` and `offsets`. A turned region
/// is stored counter-clockwise, `size` wide by `size` high the other way round.
fn libgdx(text: &str) -> Option<Atlas> {
    #[derive(Default)]
    struct Region {
        name: String,
        xy: Option<[f64; 2]>,
        size: Option<[f64; 2]>,
        orig: Option<[f64; 2]>,
        offset: Option<[f64; 2]>,
        rotate: bool,
        index: Option<i64>,
    }
    fn finish(r: Region, page: &mut Page) {
        let (Some([x, y]), Some([w, h])) = (r.xy, r.size) else { return };
        let (Some(x), Some(y), Some(w), Some(h)) = (px(x), px(y), px(w), px(h)) else { return };
        let rect = if r.rotate { [x, y, h, w] } else { [x, y, w, h] };
        let trim = match (r.orig, r.offset) {
            (Some([ow, oh]), offset) if ow > 0.0 && oh > 0.0 => {
                let [ox, oy] = offset.unwrap_or([0.0, 0.0]);
                trim_of(
                    [px(ow).unwrap_or(w), px(oh).unwrap_or(h)],
                    [ox.round() as i64, (oh - oy - f64::from(h)).round() as i64],
                    [w, h],
                )
            }
            _ => None,
        };
        let name = match r.index {
            Some(i) if i >= 0 => format!("{}_{i}", r.name),
            _ => r.name,
        };
        page.sprites.push(Sprite {
            name,
            rect,
            turn: if r.rotate { Turn::CounterClockwise } else { Turn::Upright },
            trim,
        });
    }
    let nums = |v: &str| -> Vec<f64> { v.split(',').filter_map(num).collect() };
    let pair = |v: &str| -> Option<[f64; 2]> {
        match nums(v)[..] {
            [a, b, ..] => Some([a, b]),
            _ => None,
        }
    };
    let mut pages: Vec<Page> = Vec::new();
    let mut region: Option<Region> = None;
    let mut after_blank = true;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            after_blank = true;
            continue;
        }
        let blank_before = std::mem::replace(&mut after_blank, false);
        match t.split_once(':') {
            None if blank_before || pages.is_empty() => {
                if let (Some(r), Some(p)) = (region.take(), pages.last_mut()) {
                    finish(r, p);
                }
                // A page's image has an extension; anything else first in the file isn't an atlas.
                let ext = crate::ingest::ext_of(t);
                if AssetKind::from_ext(&ext) != AssetKind::Texture {
                    return None;
                }
                pages.push(Page { image: t.to_owned(), sprites: Vec::new() });
            }
            None => {
                if let (Some(r), Some(p)) = (region.take(), pages.last_mut()) {
                    finish(r, p);
                }
                region = Some(Region { name: t.to_owned(), ..Region::default() });
            }
            Some((k, v)) => {
                let Some(r) = region.as_mut() else { continue };
                let v = v.trim();
                match k.trim() {
                    "xy" => r.xy = pair(v),
                    "size" => r.size = pair(v),
                    "orig" => r.orig = pair(v),
                    "offset" => r.offset = pair(v),
                    "bounds" => {
                        if let [x, y, w, h, ..] = nums(v)[..] {
                            r.xy = Some([x, y]);
                            r.size = Some([w, h]);
                        }
                    }
                    "offsets" => {
                        if let [x, y, w, h, ..] = nums(v)[..] {
                            r.offset = Some([x, y]);
                            r.orig = Some([w, h]);
                        }
                    }
                    "rotate" => r.rotate = v == "true" || v == "90",
                    "index" => r.index = v.parse().ok(),
                    _ => {}
                }
            }
        }
    }
    if let (Some(r), Some(p)) = (region.take(), pages.last_mut()) {
        finish(r, p);
    }
    Some(Atlas { format: "libgdx", pages })
}

/// An XML property list as JSON: dicts as objects, arrays as arrays, strings, numbers and booleans as themselves
/// (dates and data as strings). A binary one isn't read.
fn plist(text: &str) -> Option<Value> {
    enum Open {
        Dict(serde_json::Map<String, Value>, Option<String>),
        Array(Vec<Value>),
    }
    fn put(stack: &mut [Open], root: &mut Option<Value>, v: Value) {
        match stack.last_mut() {
            Some(Open::Dict(m, key)) => {
                if let Some(k) = key.take() {
                    m.insert(k, v);
                }
            }
            Some(Open::Array(a)) => a.push(v),
            None => *root = Some(v),
        }
    }
    let mut reader = quick_xml::Reader::from_str(text);
    let mut stack: Vec<Open> = Vec::new();
    let mut root: Option<Value> = None;
    let mut leaf: Option<(Vec<u8>, String)> = None;
    let mut seen_plist = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => match e.local_name().as_ref() {
                b"plist" => seen_plist = true,
                b"dict" => stack.push(Open::Dict(serde_json::Map::new(), None)),
                b"array" => stack.push(Open::Array(Vec::new())),
                other => leaf = Some((other.to_vec(), String::new())),
            },
            Ok(Event::Empty(e)) => match e.local_name().as_ref() {
                b"true" => put(&mut stack, &mut root, Value::Bool(true)),
                b"false" => put(&mut stack, &mut root, Value::Bool(false)),
                b"dict" => put(&mut stack, &mut root, Value::Object(serde_json::Map::new())),
                b"array" => put(&mut stack, &mut root, Value::Array(Vec::new())),
                b"string" => put(&mut stack, &mut root, Value::String(String::new())),
                _ => {}
            },
            Ok(Event::Text(t)) => {
                if let Some((_, s)) = leaf.as_mut() {
                    s.push_str(&t.decode().ok()?);
                }
            }
            Ok(Event::GeneralRef(r)) => {
                if let Some((_, s)) = leaf.as_mut() {
                    let name = r.decode().ok()?;
                    s.push_str(match name.as_ref() {
                        "amp" => "&",
                        "lt" => "<",
                        "gt" => ">",
                        "quot" => "\"",
                        "apos" => "'",
                        _ => "",
                    });
                }
            }
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"dict" | b"array" => {
                    let v = match stack.pop()? {
                        Open::Dict(m, _) => Value::Object(m),
                        Open::Array(a) => Value::Array(a),
                    };
                    put(&mut stack, &mut root, v);
                }
                b"plist" => {}
                _ => {
                    let (tag, s) = leaf.take()?;
                    match tag.as_slice() {
                        b"key" => {
                            if let Some(Open::Dict(_, key)) = stack.last_mut() {
                                *key = Some(s);
                            }
                        }
                        b"integer" | b"real" => put(&mut stack, &mut root, num(&s).map_or(Value::Null, |n| json!(n))),
                        _ => put(&mut stack, &mut root, Value::String(s)),
                    }
                }
            },
            Ok(Event::Eof) => break,
            Err(_) => return None,
            _ => {}
        }
    }
    root.filter(|_| seen_plist)
}

/// Numbers in a Cocos2d string: `{{1,2},{3,4}}` → [1, 2, 3, 4].
fn braces(s: &str) -> Vec<f64> {
    s.split(['{', '}', ',']).filter_map(num).collect()
}

/// Cocos2d's `.plist`: `frames` by name and `metadata` naming the page. Format 0 gives `x`, `y`, `width`, `height` and
/// offsets; 1 and 2 a `frame` (upright size) with `offset` or `sourceColorRect` and `sourceSize`; 3 a `textureRect`
/// with `spriteOffset` and `spriteSourceSize`. Offsets are from the center, y up. A turned frame is stored clockwise.
fn cocos(v: &Value) -> Option<Atlas> {
    let frames = v.get("frames")?.as_object()?;
    let meta = v.get("metadata");
    let image = meta
        .and_then(|m| m.get("realTextureFileName").or_else(|| m.get("textureFileName")))
        .and_then(Value::as_str)
        .or_else(|| meta?.get("target")?.get("textureFileName")?.as_str())?
        .to_owned();
    let format = meta.and_then(|m| field(m, "format")).unwrap_or(0.0) as i64;
    let text = |f: &Value, k: &str| braces(f.get(k).and_then(Value::as_str).unwrap_or(""));
    let mut sprites = Vec::new();
    for (name, f) in frames {
        let (x, y, w, h, rotated, source, offset, color_rect) = if format == 0 && f.get("x").is_some() {
            let g = |k| field(f, k).unwrap_or(0.0);
            let source = [g("originalWidth").abs(), g("originalHeight").abs()];
            (g("x"), g("y"), g("width"), g("height"), false, Some(source), Some([g("offsetX"), g("offsetY")]), None)
        } else {
            let rect = if format == 3 { text(f, "textureRect") } else { text(f, "frame") };
            let [x, y, w, h, ..] = rect[..] else { continue };
            let rotated = f.get(if format == 3 { "textureRotated" } else { "rotated" }).and_then(Value::as_bool);
            let source = if format == 3 { text(f, "spriteSourceSize") } else { text(f, "sourceSize") };
            let offset = if format == 3 { text(f, "spriteOffset") } else { text(f, "offset") };
            let color = text(f, "sourceColorRect");
            (
                x,
                y,
                w,
                h,
                rotated.unwrap_or(false),
                match source[..] {
                    [a, b, ..] => Some([a, b]),
                    _ => None,
                },
                match offset[..] {
                    [a, b, ..] => Some([a, b]),
                    _ => None,
                },
                match color[..] {
                    [a, b, ..] if format == 2 => Some([a, b]),
                    _ => None,
                },
            )
        };
        let (Some(x), Some(y), Some(w), Some(h)) = (px(x), px(y), px(w), px(h)) else { continue };
        let rect = if rotated { [x, y, h, w] } else { [x, y, w, h] };
        let trim = source.and_then(|[sw, sh]| {
            let at = color_rect.unwrap_or_else(|| {
                let [ox, oy] = offset.unwrap_or([0.0, 0.0]);
                [(sw - f64::from(w)) / 2.0 + ox, (sh - f64::from(h)) / 2.0 - oy]
            });
            trim_of([px(sw)?, px(sh)?], [at[0].round() as i64, at[1].round() as i64], [w, h])
        });
        sprites.push(Sprite {
            name: name.clone(),
            rect,
            turn: if rotated { Turn::Clockwise } else { Turn::Upright },
            trim,
        });
    }
    Some(Atlas { format: "cocos2d", pages: vec![Page { image, sprites }] })
}

/// A sprite as it was before packing: cut from its page, turned upright and put back on its untrimmed canvas.
/// `None` when it isn't inside the page.
pub(crate) fn cut(page: &RgbaImage, s: &Sprite) -> Option<RgbaImage> {
    let [x, y, w, h] = s.rect;
    if w == 0 || h == 0 || x.checked_add(w)? > page.width() || y.checked_add(h)? > page.height() {
        return None;
    }
    let region = imageops::crop_imm(page, x, y, w, h).to_image();
    let upright = match s.turn {
        Turn::Upright => region,
        Turn::Clockwise => imageops::rotate270(&region),
        Turn::CounterClockwise => imageops::rotate90(&region),
    };
    let Some(trim) = s.trim else { return Some(upright) };
    let [cw, ch] = trim.size;
    if cw > MAX_SIDE || ch > MAX_SIDE {
        return None;
    }
    let mut canvas = RgbaImage::new(cw, ch);
    imageops::overlay(&mut canvas, &upright, trim.at[0], trim.at[1]);
    Some(canvas)
}

/// The same picture, whatever the file: its size and pixels, with the color of fully transparent ones left out.
fn pixel_hash(img: &RgbaImage) -> blake3::Hash {
    let mut h = blake3::Hasher::new();
    h.update(&img.width().to_le_bytes());
    h.update(&img.height().to_le_bytes());
    for p in img.pixels() {
        h.update(&if p[3] == 0 { [0; 4] } else { p.0 });
    }
    h.finalize()
}

/// Where a sprite is kept: `<atlas folder>/<atlas name>/<sprite name>.png`, the sprite's folders kept and anything
/// climbing out of them dropped.
fn sprite_path(atlas: &str, name: &str) -> Option<String> {
    let base = atlas.rsplit_once('.').map_or(atlas, |(b, _)| b);
    let parts: Vec<&str> =
        name.split(['/', '\\']).map(str::trim).filter(|p| !p.is_empty() && *p != "." && *p != "..").collect();
    let (last, dirs) = parts.split_last()?;
    let stem = image_stem(last);
    if stem.is_empty() {
        return None;
    }
    let mut out = base.to_owned();
    for d in dirs {
        out.push('/');
        out.push_str(d);
    }
    Some(format!("{out}/{stem}.png"))
}

/// A sprite's name without the extension of the file it was packed from (`hero.png`, `walk 0.aseprite`).
fn image_stem(name: &str) -> &str {
    match name.rsplit_once('.') {
        Some((stem, ext))
            if AssetKind::from_ext(&ext.to_lowercase()) == AssetKind::Texture
                || matches!(ext.to_lowercase().as_str(), "ase" | "aseprite" | "psd") =>
        {
            stem
        }
        _ => name,
    }
}

/// `a/b/../c.png` → `a/c.png`.
fn normalize(path: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for part in path.split(['/', '\\']) {
        match part {
            "" | "." => {}
            ".." => {
                out.pop();
            }
            p => out.push(p),
        }
    }
    out.join("/")
}

fn dir_of(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(d, _)| d)
}

fn stem_of(path: &str) -> &str {
    let file = path.rsplit('/').next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

// ---------------------------------------------------------------------------------------------
// The library side

#[derive(Debug)]
struct Texture {
    id: String,
    path: String,
    hash: String,
    size: [u32; 2],
    bytes: i64,
    /// Cut from an atlas earlier.
    cut: bool,
}

/// What became of one sprite.
enum Placed {
    /// The pack has it already, as this texture.
    Loose(String),
    /// Kept as a sprite of its own.
    Own(Box<Made>),
}

/// A sprite of its own, ready to write.
struct Made {
    path: String,
    name: String,
    blob: crate::blobs::StoredBlob,
    /// Read again (it's new or its pixels changed): its facts, thumbnail and tags. Otherwise the row stands.
    item: Option<(crate::ingest::Item, Option<crate::blobs::StoredBlob>)>,
    sprite: Value,
}

/// One atlas found in the pack.
struct Found {
    file_id: String,
    page_id: String,
    page_path: String,
    file_path: String,
    format: &'static str,
    sprites: Vec<Placed>,
}

/// Find the pack's atlases and bring the library in line: each sprite linked from its page, as the loose image the
/// pack has of it or as a sprite of its own (added, updated, or removed when its atlas is gone). Returns how many
/// atlases it has.
pub(crate) fn detect_pack(state: &AppState, rt: &Handle, conn: &mut Connection, pack_id: &str) -> Result<usize> {
    let files: Vec<(String, String, String, String)> = {
        let mut stmt = conn.prepare(&format!(
            "SELECT a.id, a.path, a.blob_hash, a.ext FROM assets a JOIN blobs b ON b.hash = a.blob_hash
              WHERE a.pack_id = ?1 AND a.ext IN ('xml', 'json', 'atlas', 'plist') AND b.size <= {MAX_FILE}
              ORDER BY a.path"
        ))?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
            .collect::<rusqlite::Result<_>>()?
    };
    let textures: Vec<Texture> = {
        let mut stmt = conn.prepare(&format!(
            "SELECT a.id, a.path, a.blob_hash, COALESCE(json_extract(a.meta, '$.width'), 0),
                    COALESCE(json_extract(a.meta, '$.height'), 0), COALESCE(b.size, 0), {IS_SPRITE}
               FROM assets a LEFT JOIN blobs b ON b.hash = a.blob_hash
              WHERE a.pack_id = ?1 AND a.kind = 'texture' ORDER BY a.path"
        ))?;
        stmt.query_map([pack_id], |r| {
            Ok(Texture {
                id: r.get(0)?,
                path: r.get(1)?,
                hash: r.get(2)?,
                size: [r.get::<_, f64>(3)? as u32, r.get::<_, f64>(4)? as u32],
                bytes: r.get(5)?,
                cut: r.get(6)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    let by_path: HashMap<String, usize> =
        textures.iter().enumerate().filter(|(_, t)| !t.cut).map(|(i, t)| (t.path.to_lowercase(), i)).collect();
    let cut_before: HashMap<&str, (&str, &str)> =
        textures.iter().filter(|t| t.cut).map(|t| (t.path.as_str(), (t.id.as_str(), t.hash.as_str()))).collect();
    let taken: HashSet<String> = {
        let mut stmt =
            conn.prepare(&format!("SELECT lower(a.path) FROM assets a WHERE a.pack_id = ?1 AND NOT {IS_SPRITE}"))?;
        stmt.query_map([pack_id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
    };
    let license: Vec<String> = {
        let mut stmt = conn.prepare("SELECT 'license:' || license FROM packs WHERE id = ?1 AND license IS NOT NULL")?;
        stmt.query_map([pack_id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
    };

    let mut loose = Loose { textures: &textures, hashes: HashMap::new(), by_size: HashMap::new() };
    let mut found: Vec<Found> = Vec::new();
    let mut paths = HashSet::new();
    let mut count = 0;
    let work = tempfile::Builder::new().prefix("atlas-").tempdir_in({
        std::fs::create_dir_all(state.cfg.tmp_dir())?;
        state.cfg.tmp_dir()
    })?;
    for (file_id, file_path, hash, ext) in &files {
        let text = rt.block_on(state.blobs.read_all(hash))?;
        let Some(atlas) = parse(ext, &String::from_utf8_lossy(&text)) else { continue };
        let one_page = atlas.pages.len() == 1;
        for page in atlas.pages {
            let at = normalize(&format!("{}/{}", dir_of(file_path), page.image));
            // As named, or else beside the atlas with the atlas's name (a page renamed after packing).
            let page_idx = by_path.get(&at.to_lowercase()).copied().or_else(|| {
                let ext = crate::ingest::ext_of(&page.image);
                let base = file_path.rsplit_once('.').map_or(file_path.as_str(), |(b, _)| b);
                one_page.then(|| by_path.get(&format!("{base}.{ext}").to_lowercase()).copied()).flatten()
            });
            let Some(page_idx) = page_idx else {
                tracing::debug!(atlas = %file_path, page = %page.image, "atlas page not in the pack");
                continue;
            };
            let page_tex = &textures[page_idx];
            let img = match rt
                .block_on(state.blobs.read_all(&page_tex.hash))
                .and_then(|b| Ok(image::load_from_memory(&b)?.to_rgba8()))
            {
                Ok(img) if img.width() <= MAX_SIDE && img.height() <= MAX_SIDE => img,
                Ok(_) => continue,
                Err(e) => {
                    tracing::debug!(atlas = %file_path, page = %page_tex.path, error = %format!("{e:#}"), "atlas page unreadable");
                    continue;
                }
            };
            let mut placed = Vec::new();
            for s in &page.sprites {
                if count >= MAX_SPRITES {
                    break;
                }
                let Some(sprite) = cut(&img, s) else { continue };
                let Some(path) = sprite_path(file_path, &s.name) else { continue };
                count += 1;
                if let Some(t) = loose.find(state, rt, &sprite, &path, page_idx) {
                    placed.push(Placed::Loose(t.to_owned()));
                    continue;
                }
                if taken.contains(&path.to_lowercase()) || !paths.insert(path.to_lowercase()) {
                    continue;
                }
                let png = crate::analyze::png_bytes(sprite)?;
                let blob_hash = blake3::hash(&png).to_hex().to_string();
                let meta = json!({
                    "atlas": file_path,
                    "page": page_tex.path,
                    "rect": s.rect,
                    "rotated": s.turn != Turn::Upright,
                });
                let name = tagger::display_name(&tagger::name_stem(&path));
                if cut_before.get(path.as_str()).is_some_and(|(_, h)| *h == blob_hash) {
                    let blob = crate::blobs::StoredBlob { hash: blob_hash, size: png.len() as u64 };
                    placed.push(Placed::Own(Box::new(Made { path, name, blob, item: None, sprite: meta })));
                    continue;
                }
                let local = work.path().join("sprite.png");
                std::fs::write(&local, &png)?;
                let blob = rt.block_on(state.blobs.put_bytes(png, AssetKind::Texture))?;
                let mut item = crate::ingest::analyze_one(local, work.path(), path.clone(), Some(blob.clone()))
                    .with_context(|| format!("{path}: could not read the sprite"))?;
                let thumb = match item.analysis.thumb.take() {
                    Some(t) => Some(rt.block_on(state.blobs.put_cache_bytes(t))?),
                    None => None,
                };
                placed.push(Placed::Own(Box::new(Made { path, name, blob, item: Some((item, thumb)), sprite: meta })));
            }
            found.push(Found {
                file_id: file_id.clone(),
                page_id: page_tex.id.clone(),
                page_path: page_tex.path.clone(),
                file_path: file_path.clone(),
                format: atlas.format,
                sprites: placed,
            });
        }
    }
    drop(work);
    let n = found.len();
    write(conn, pack_id, found, &license)?;
    Ok(n)
}

/// The pack's loose textures, to find a sprite among: by its pixels (each hashed when a sprite of its size comes
/// along), or by its name and shape.
struct Loose<'a> {
    textures: &'a [Texture],
    hashes: HashMap<usize, Option<blake3::Hash>>,
    by_size: HashMap<[u32; 2], Vec<usize>>,
}

/// Loose images bigger than this aren't read to compare with sprites.
const MAX_LOOSE: i64 = 16 << 20;

impl Loose<'_> {
    fn find(&mut self, state: &AppState, rt: &Handle, sprite: &RgbaImage, path: &str, page: usize) -> Option<&str> {
        if self.by_size.is_empty() {
            for (i, t) in self.textures.iter().enumerate() {
                if !t.cut && i != page && t.size[0] > 0 && t.bytes <= MAX_LOOSE {
                    self.by_size.entry(t.size).or_default().push(i);
                }
            }
            if self.by_size.is_empty() {
                self.by_size.insert([0, 0], Vec::new());
            }
        }
        let size = [sprite.width(), sprite.height()];
        if let Some(same) = self.by_size.get(&size) {
            let want = pixel_hash(sprite);
            for &i in same {
                let got = *self.hashes.entry(i).or_insert_with(|| {
                    let bytes = rt.block_on(state.blobs.read_all(&self.textures[i].hash)).ok()?;
                    Some(pixel_hash(&image::load_from_memory(&bytes).ok()?.to_rgba8()))
                });
                if got == Some(want) {
                    return Some(&self.textures[i].id);
                }
            }
        }
        // The same name and shape: a loose copy at another scale, or saved another way.
        let stem = stem_of(path).to_lowercase();
        let ratio = f64::from(size[0]) / f64::from(size[1].max(1));
        self.textures
            .iter()
            .enumerate()
            .find(|(i, t)| {
                !t.cut
                    && *i != page
                    && t.size[1] > 0
                    && stem_of(&t.path).to_lowercase() == stem
                    && (f64::from(t.size[0]) / f64::from(t.size[1]) - ratio).abs() <= ratio * 0.02
            })
            .map(|(_, t)| t.id.as_str())
    }
}

/// Write what was found: sprites added or updated, pages marked and linked to their sprites, atlas files linked to
/// their pages, and sprites whose atlas is gone removed.
fn write(conn: &mut Connection, pack_id: &str, found: Vec<Found>, license: &[String]) -> Result<()> {
    let marked: bool = conn.query_row(
        &format!(
            "SELECT EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = ?1
                              AND ({IS_SPRITE} OR json_extract(a.meta, '$.atlas') IS NOT NULL))"
        ),
        [pack_id],
        |r| r.get(0),
    )?;
    if found.is_empty() && !marked {
        return Ok(());
    }
    let now = db::now();
    let tx = conn.transaction()?;
    let existing: HashMap<String, String> = {
        let mut stmt =
            tx.prepare(&format!("SELECT a.path, a.id FROM assets a WHERE a.pack_id = ?1 AND {IS_SPRITE}"))?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?
    };
    tx.execute(
        "DELETE FROM asset_links WHERE rel = 'sprite' AND from_id IN (SELECT id FROM assets WHERE pack_id = ?1)",
        [pack_id],
    )?;
    tx.execute(
        "UPDATE assets SET meta = json_remove(meta, '$.atlas')
          WHERE pack_id = ?1 AND json_extract(meta, '$.atlas') IS NOT NULL",
        [pack_id],
    )?;
    let mut kept = HashSet::new();
    let mut link =
        tx.prepare_cached("INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING")?;
    let mut mark = tx.prepare_cached("UPDATE assets SET meta = json_set(meta, '$.atlas', json(?2)) WHERE id = ?1")?;
    for f in found {
        link.execute(params![f.file_id, f.page_id, "texture"])?;
        mark.execute(params![
            f.page_id,
            json!({ "file": f.file_path, "format": f.format, "sprites": f.sprites.len() }).to_string()
        ])?;
        for p in f.sprites {
            let id = match p {
                Placed::Loose(id) => id,
                Placed::Own(m) => {
                    let id = existing.get(&m.path).cloned().unwrap_or_else(|| uuid::Uuid::now_v7().to_string());
                    write_sprite(&tx, pack_id, &id, *m, &f.page_path, license, &now)?;
                    kept.insert(id.clone());
                    id
                }
            };
            link.execute(params![f.page_id, id, "sprite"])?;
        }
    }
    drop((link, mark));
    for id in existing.values() {
        if !kept.contains(id) {
            tx.execute("DELETE FROM assets_fts WHERE asset_id = ?1", [id])?;
            tx.execute("DELETE FROM assets WHERE id = ?1", [id])?;
        }
    }
    crate::links::assign_families(&tx, pack_id)?;
    tx.commit()?;
    Ok(())
}

/// One sprite of its own into the library: a new row, or the row it had with its pixels, facts and tags brought up to
/// date (a name or category someone gave it kept). One unchanged only has its meta's note of where it came from
/// renewed.
fn write_sprite(
    tx: &rusqlite::Transaction<'_>,
    pack_id: &str,
    id: &str,
    m: Made,
    page: &str,
    license: &[String],
    now: &str,
) -> Result<()> {
    let Some((item, thumb)) = m.item else {
        tx.execute(
            "UPDATE assets SET meta = json_set(meta, '$.sprite', json(?2)) WHERE id = ?1",
            params![id, m.sprite.to_string()],
        )?;
        return Ok(());
    };
    for b in [Some(&m.blob), thumb.as_ref()].into_iter().flatten() {
        tx.execute(
            "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
            params![b.hash, b.size as i64, now],
        )?;
    }
    let mut meta = item.analysis.meta;
    meta.insert("sprite".into(), m.sprite);
    let mut tags = item.tags;
    for l in license {
        tags.add(l, TagSource::Rule, 0.9);
    }
    tags.add("sprite", TagSource::Rule, 0.9);
    let category = tagger::categorize(AssetKind::Texture, &tags);
    tx.execute(
        "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, category, meta, words, thumb_hash,
                             created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 'png', 'texture', ?6, ?7, ?8, ?9, ?10, ?10)
         ON CONFLICT (pack_id, path) DO UPDATE SET
             blob_hash = excluded.blob_hash, meta = excluded.meta, words = excluded.words,
             thumb_hash = COALESCE(excluded.thumb_hash, thumb_hash), updated_at = excluded.updated_at,
             name = CASE WHEN human_edited THEN name ELSE excluded.name END,
             category = CASE WHEN human_edited THEN category ELSE excluded.category END",
        params![
            id,
            pack_id,
            m.blob.hash,
            m.path,
            m.name,
            category,
            Value::Object(meta).to_string(),
            format!("{} sprite {}", tagger::fts_words(&m.path), tagger::fts_words(page)),
            thumb.as_ref().map(|t| t.hash.clone()),
            now,
        ],
    )?;
    tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND source IN ('rule', 'meta')", [id])?;
    for s in tags.into_vec() {
        db::upsert_tag(tx, id, &s.tag, s.source, s.confidence)?;
    }
    db::reindex_asset(tx, id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    fn only(a: &Atlas) -> &Page {
        assert_eq!(a.pages.len(), 1);
        &a.pages[0]
    }

    #[test]
    fn starling_with_trim_and_rotation() {
        let a = parse(
            "xml",
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <TextureAtlas imagePath="sheet.png">
              <SubTexture name="hero_0001" x="0" y="0" width="16" height="24"/>
              <SubTexture name="coin" x="16" y="0" width="10" height="8" rotated="true"
                          frameX="-2" frameY="-1" frameWidth="12" frameHeight="14"/>
            </TextureAtlas>"#,
        )
        .expect("an atlas");
        assert_eq!(a.format, "starling");
        let p = only(&a);
        assert_eq!(p.image, "sheet.png");
        assert_eq!(
            p.sprites[0],
            Sprite { name: "hero_0001".into(), rect: [0, 0, 16, 24], turn: Turn::Upright, trim: None }
        );
        assert_eq!(
            p.sprites[1],
            Sprite {
                name: "coin".into(),
                rect: [16, 0, 10, 8],
                turn: Turn::Clockwise,
                trim: Some(Trim { size: [12, 14], at: [2, 1] })
            }
        );
        assert!(parse("xml", "<root><SubTexture x='0'/></root>").is_none());
    }

    #[test]
    fn texture_packer_hash_array_and_multipack() {
        let frame = r#"{"frame":{"x":4,"y":2,"w":10,"h":20},"rotated":true,"trimmed":true,
                        "spriteSourceSize":{"x":3,"y":1,"w":10,"h":20},"sourceSize":{"w":16,"h":24}}"#;
        let hash = format!(r#"{{"frames":{{"walk/01.png":{frame}}},"meta":{{"image":"atlas.png"}}}}"#);
        let a = parse("json", &hash).expect("hash");
        let want = Sprite {
            name: "walk/01.png".into(),
            rect: [4, 2, 20, 10],
            turn: Turn::Clockwise,
            trim: Some(Trim { size: [16, 24], at: [3, 1] }),
        };
        assert_eq!(only(&a).sprites, vec![want.clone()]);
        let array = format!(
            r#"{{"frames":[{}],"meta":{{"image":"atlas.png"}}}}"#,
            frame.replacen('{', r#"{"filename":"walk/01.png","#, 1)
        );
        assert_eq!(only(&parse("json", &array).expect("array")).sprites, vec![want.clone()]);
        let multi = format!(
            r#"{{"textures":[{{"image":"a0.png","frames":[{}]}},{{"image":"a1.png","frames":[]}}]}}"#,
            frame.replacen('{', r#"{"filename":"walk/01.png","#, 1)
        );
        let m = parse("json", &multi).expect("multipack");
        assert_eq!(m.pages.len(), 1, "a page with no frames is left out");
        assert_eq!(m.pages[0].image, "a0.png");
        assert!(parse("json", r#"{"id": "x", "semanticTags": {}}"#).is_none());
    }

    #[test]
    fn libgdx_old_and_new_formats() {
        let old = "\nsheet.png\nsize: 64,64\nformat: RGBA8888\nfilter: Linear,Linear\nrepeat: none\nhero\n  rotate: true\n  xy: 2, 4\n  size: 10, 20\n  orig: 12, 24\n  offset: 1, 2\n  index: -1\nwalk\n  rotate: false\n  xy: 30, 4\n  size: 8, 8\n  orig: 8, 8\n  offset: 0, 0\n  index: 3\n";
        let a = parse("atlas", old).expect("old format");
        let p = only(&a);
        assert_eq!(p.image, "sheet.png");
        assert_eq!(
            p.sprites[0],
            Sprite {
                name: "hero".into(),
                rect: [2, 4, 20, 10],
                turn: Turn::CounterClockwise,
                trim: Some(Trim { size: [12, 24], at: [1, 2] })
            }
        );
        assert_eq!(
            p.sprites[1],
            Sprite { name: "walk_3".into(), rect: [30, 4, 8, 8], turn: Turn::Upright, trim: None }
        );

        let new = "sheet.png\nsize:64,64\nfilter:Linear,Linear\nhero\nbounds:2,4,10,20\noffsets:1,2,12,24\nrotate:90\n\npage2.png\nsize:32,32\ncoin\nbounds:0,0,5,5\n";
        let a = parse("atlas", new).expect("new format");
        assert_eq!(a.pages.len(), 2);
        assert_eq!(a.pages[0].sprites[0].rect, [2, 4, 20, 10]);
        assert_eq!(a.pages[0].sprites[0].trim, Some(Trim { size: [12, 24], at: [1, 2] }));
        assert_eq!(a.pages[1].image, "page2.png");
        assert_eq!(a.pages[1].sprites[0].name, "coin");
        assert!(parse("atlas", "just some notes\nabout things\n").is_none());
    }

    fn plist_doc(format: u8, frame: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0"><dict>
              <key>frames</key><dict><key>gem.png</key><dict>{frame}</dict></dict>
              <key>metadata</key><dict><key>format</key><integer>{format}</integer>
                <key>textureFileName</key><string>gems.png</string></dict>
            </dict></plist>"#
        )
    }

    #[test]
    fn cocos_formats_two_and_three() {
        let two = plist_doc(
            2,
            "<key>frame</key><string>{{4,2},{10,20}}</string><key>offset</key><string>{0,0}</string>
             <key>rotated</key><true/><key>sourceColorRect</key><string>{{3,1},{10,20}}</string>
             <key>sourceSize</key><string>{16,24}</string>",
        );
        let a = parse("plist", &two).expect("format 2");
        assert_eq!(a.format, "cocos2d");
        assert_eq!(
            only(&a).sprites,
            vec![Sprite {
                name: "gem.png".into(),
                rect: [4, 2, 20, 10],
                turn: Turn::Clockwise,
                trim: Some(Trim { size: [16, 24], at: [3, 1] })
            }]
        );
        assert_eq!(only(&a).image, "gems.png");
        // Centered offset, y up: 16-10 = 6 wide, so 3 + 1 from the left; 24-20 = 4 high, so 2 - (-1) from the top.
        let three = plist_doc(
            3,
            "<key>textureRect</key><string>{{4,2},{10,20}}</string><key>textureRotated</key><false/>
             <key>spriteOffset</key><string>{1,-1}</string><key>spriteSourceSize</key><string>{16,24}</string>
             <key>spriteSize</key><string>{10,20}</string>",
        );
        let a = parse("plist", &three).expect("format 3");
        let s = &only(&a).sprites[0];
        assert_eq!(s.rect, [4, 2, 10, 20]);
        assert_eq!(s.trim, Some(Trim { size: [16, 24], at: [4, 3] }));
        assert!(parse("plist", "<plist><dict><key>CFBundleName</key><string>x</string></dict></plist>").is_none());
    }

    /// A 3×2 sprite with a different color in each pixel, so a wrong turn shows.
    fn sprite() -> RgbaImage {
        RgbaImage::from_fn(3, 2, |x, y| Rgba([x as u8 * 50 + 10, y as u8 * 100 + 10, 7, 255]))
    }

    #[test]
    fn cut_turns_back_and_untrims() {
        let want = sprite();
        // Turned clockwise into a 2×3 region at (1, 1) of a page.
        let mut page = RgbaImage::new(6, 6);
        imageops::overlay(&mut page, &imageops::rotate90(&want), 1, 1);
        let s = Sprite { name: "s".into(), rect: [1, 1, 2, 3], turn: Turn::Clockwise, trim: None };
        assert_eq!(cut(&page, &s).expect("cut"), want);
        // Turned counter-clockwise, as libGDX does.
        let mut page = RgbaImage::new(6, 6);
        imageops::overlay(&mut page, &imageops::rotate270(&want), 2, 0);
        let s = Sprite { name: "s".into(), rect: [2, 0, 2, 3], turn: Turn::CounterClockwise, trim: None };
        assert_eq!(cut(&page, &s).expect("cut"), want);
        // Trimmed: back on a 5×4 canvas at (1, 2).
        let mut page = RgbaImage::new(6, 6);
        imageops::overlay(&mut page, &want, 0, 0);
        let s = Sprite {
            name: "s".into(),
            rect: [0, 0, 3, 2],
            turn: Turn::Upright,
            trim: Some(Trim { size: [5, 4], at: [1, 2] }),
        };
        let got = cut(&page, &s).expect("cut");
        assert_eq!(got.dimensions(), (5, 4));
        assert_eq!(*got.get_pixel(0, 0), Rgba([0, 0, 0, 0]));
        assert_eq!(*got.get_pixel(1, 2), *want.get_pixel(0, 0));
        assert_eq!(*got.get_pixel(3, 3), *want.get_pixel(2, 1));
        // Outside the page.
        assert!(cut(&page, &Sprite { rect: [4, 4, 3, 3], ..s }).is_none());
    }

    #[test]
    fn pixels_match_whatever_hides_under_transparency() {
        let a = RgbaImage::from_pixel(2, 2, Rgba([255, 0, 0, 0]));
        let b = RgbaImage::from_pixel(2, 2, Rgba([0, 255, 0, 0]));
        assert_eq!(pixel_hash(&a), pixel_hash(&b));
        assert_ne!(pixel_hash(&a), pixel_hash(&RgbaImage::from_pixel(2, 2, Rgba([0, 255, 0, 1]))));
    }

    #[test]
    fn sprite_paths() {
        assert_eq!(
            sprite_path("Spritesheet/sheet.xml", "hero_0001").as_deref(),
            Some("Spritesheet/sheet/hero_0001.png")
        );
        assert_eq!(sprite_path("a.json", "walk/01.png").as_deref(), Some("a/walk/01.png"));
        assert_eq!(sprite_path("x/a.atlas", "../../etc/passwd").as_deref(), Some("x/a/etc/passwd.png"));
        assert_eq!(sprite_path("a.json", "walk 0.aseprite").as_deref(), Some("a/walk 0.png"));
        assert_eq!(sprite_path("a.json", "v1.2").as_deref(), Some("a/v1.2.png"));
        assert_eq!(sprite_path("a.json", "/").as_deref(), None);
        assert_eq!(normalize("Sheets/../img/./a.png"), "img/a.png");
    }
}
