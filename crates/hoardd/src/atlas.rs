//! Sprite atlases: one image (a page) packed with many sprites, and a file that says where each one is. Starling and
//! Sparrow XML, TexturePacker JSON (hash, array and multipack, and Aseprite's export, which is the same), libGDX and
//! Spine `.atlas` files and Cocos2d `.plist` files are read.
//!
//! The atlas file becomes a `sheet` (see [`crate::spritesheet`]), linked to its pages (`texture`), so its download
//! brings the images it describes. Packs often come with each sprite as a loose image too (Kenney's `PNG/` folders
//! beside each `Spritesheet/`): a loose image with a sprite's name and picture is linked from the sheet (`sprite`)
//! and shown with it rather than on its own in the grid, as the pages are.
//!
//! Atlases are looked for after materials, when a pack is ingested or re-tagged and in the `materials` job.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

use std::collections::{HashMap, HashSet};

use anyhow::Result;
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Atlas {
    pub(crate) format: &'static str,
    pub(crate) pages: Vec<Page>,
    /// The animations the file names (Aseprite's tags, a Godot `SpriteFrames`'s animations).
    pub(crate) animations: Vec<Anim>,
    /// A tile set's tile size: its sprites are cells of a grid.
    pub(crate) tile: Option<[u32; 2]>,
}

/// An animation: sprites shown one after another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Anim {
    pub(crate) name: String,
    pub(crate) frames: Vec<Frame>,
    pub(crate) looped: bool,
}

/// One frame of an animation: a sprite (by page, and its place in the page) and how long it shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Frame {
    pub(crate) page: usize,
    pub(crate) sprite: usize,
    pub(crate) ms: u32,
}

/// How long a frame shows when the file doesn't say.
pub(crate) const FRAME_MS: u32 = 100;

/// Bigger files aren't atlases.
const MAX_FILE: i64 = 8 << 20;
/// An atlas with more sprites than this isn't read.
const MAX_SPRITES: usize = 20_000;
/// Pages and sprites bigger than this (in either direction) aren't cut.
const MAX_SIDE: u32 = 16_384;

/// Read an atlas's file. `None` when it isn't one.
pub(crate) fn parse(ext: &str, text: &str) -> Option<Atlas> {
    let text = text.trim_start_matches('\u{feff}');
    let atlas = match ext {
        "xml" => starling(text),
        "json" => {
            let v: Value = serde_json::from_str(text).ok()?;
            texture_packer(&v).or_else(|| tiled_json(&v))
        }
        "tsj" => tiled_json(&serde_json::from_str(text).ok()?),
        "tsx" => tiled(text),
        "atlas" => libgdx(text),
        "plist" => cocos(&plist(text)?),
        "tres" => godot(text),
        _ => None,
    }?;
    // Pages with nothing on them are left out, and the animations follow the pages that stay.
    let mut moved = Vec::with_capacity(atlas.pages.len());
    let mut pages = Vec::new();
    for p in atlas.pages {
        if p.image.is_empty() || p.sprites.is_empty() {
            moved.push(None);
        } else {
            moved.push(Some(pages.len()));
            pages.push(p);
        }
    }
    let animations = atlas
        .animations
        .into_iter()
        .filter_map(|a| {
            let frames: Vec<Frame> = a
                .frames
                .into_iter()
                .filter_map(|f| {
                    let page = (*moved.get(f.page)?)?;
                    (f.sprite < pages[page].sprites.len()).then_some(Frame { page, ..f })
                })
                .collect();
            (!frames.is_empty()).then_some(Anim { frames, ..a })
        })
        .collect();
    (!pages.is_empty()).then_some(Atlas { pages, animations, ..atlas })
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

/// A Tiled tileset's grid: its image, tile size, the space between tiles and round the edge, and how many columns and
/// tiles.
struct Tileset {
    image: String,
    tile: [u32; 2],
    spacing: u32,
    margin: u32,
    columns: u32,
    count: u32,
    /// Tiles that play an animation: the tile, what it's called (its class, if it has one) and its frames (tile and
    /// how long it shows).
    anims: Vec<TileAnim>,
}

/// A tile's animation: the tile, its class and its frames (tile, and milliseconds).
type TileAnim = (u32, Option<String>, Vec<(u32, u32)>);

/// A tileset as an atlas of one page, its tiles named as Kenney names his loose tiles (`tile_0000`, by tile id) and
/// its tiles' animations each named after its class or the tile it's on.
fn tileset_atlas(t: Tileset) -> Option<Atlas> {
    let [tw, th] = t.tile;
    if tw == 0 || th == 0 || t.columns == 0 || t.count == 0 || t.count as usize > MAX_SPRITES || t.image.is_empty() {
        return None;
    }
    let sprites = (0..t.count)
        .map(|id| Sprite {
            name: format!("tile_{id:04}"),
            rect: [t.margin + id % t.columns * (tw + t.spacing), t.margin + id / t.columns * (th + t.spacing), tw, th],
            turn: Turn::Upright,
            trim: None,
        })
        .collect();
    let animations = t
        .anims
        .into_iter()
        .filter(|(_, _, frames)| !frames.is_empty())
        .map(|(id, class, frames)| Anim {
            name: class.filter(|c| !c.trim().is_empty()).unwrap_or_else(|| format!("tile_{id:04}")),
            frames: frames
                .into_iter()
                .filter(|&(tile, _)| tile < t.count)
                .map(|(tile, ms)| Frame { page: 0, sprite: tile as usize, ms: if ms == 0 { FRAME_MS } else { ms } })
                .collect(),
            looped: true,
        })
        .collect();
    Some(Atlas { format: "tiled", pages: vec![Page { image: t.image, sprites }], animations, tile: Some(t.tile) })
}

/// Tiled's tileset (`.tsx`): `<tileset tilewidth tileheight spacing margin tilecount columns><image source/>` and
/// `<tile id [type|class]><animation><frame tileid duration/>…`. A tileset of separate images has no grid to cut.
fn tiled(text: &str) -> Option<Atlas> {
    let mut reader = quick_xml::Reader::from_str(text);
    let mut set: Option<Tileset> = None;
    let mut current: Option<TileAnim> = None;
    loop {
        let (e, empty) = match reader.read_event() {
            Ok(Event::Start(e)) => (e, false),
            Ok(Event::Empty(e)) => (e, true),
            Ok(Event::End(e)) => {
                if e.local_name().as_ref() == b"tile"
                    && let (Some(s), Some(c)) = (&mut set, current.take())
                {
                    s.anims.push(c);
                }
                continue;
            }
            Ok(Event::Eof) => break,
            Err(_) => return None,
            _ => continue,
        };
        let a = attrs(&e);
        let get = |k: &str| a.get(k).and_then(|v| v.trim().parse::<u32>().ok());
        match (e.local_name().as_ref(), &mut set) {
            (b"tileset", None) => {
                set = Some(Tileset {
                    image: String::new(),
                    tile: [get("tilewidth")?, get("tileheight")?],
                    spacing: get("spacing").unwrap_or(0),
                    margin: get("margin").unwrap_or(0),
                    columns: get("columns").unwrap_or(0),
                    count: get("tilecount").unwrap_or(0),
                    anims: Vec::new(),
                });
            }
            (_, None) => return None,
            // A tile's own image: a tileset of separate images.
            (b"image", Some(_)) if current.is_some() => return None,
            (b"image", Some(s)) => s.image.clone_from(a.get("source")?),
            (b"tile", Some(_)) if !empty => {
                let class = a.get("class").or_else(|| a.get("type")).cloned();
                current = get("id").map(|id| (id, class, Vec::new()));
            }
            (b"frame", Some(_)) => {
                if let (Some(c), Some(tile)) = (&mut current, get("tileid")) {
                    c.2.push((tile, get("duration").unwrap_or(0)));
                }
            }
            _ => {}
        }
    }
    tileset_atlas(set?)
}

/// Tiled's tileset as JSON (`.tsj`, or `.json`): `{"tilewidth", "tileheight", "image", "columns", "tilecount",
/// "spacing", "margin", "tiles": [{"id", "class"|"type", "animation": [{"tileid", "duration"}]}]}`.
fn tiled_json(v: &Value) -> Option<Atlas> {
    let n = |o: &Value, k: &str| o.get(k).and_then(Value::as_u64).and_then(|n| u32::try_from(n).ok());
    let anims = v
        .get("tiles")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|t| {
            let frames: Vec<(u32, u32)> = t
                .get("animation")?
                .as_array()?
                .iter()
                .filter_map(|f| Some((n(f, "tileid")?, n(f, "duration").unwrap_or(0))))
                .collect();
            let class = t.get("class").or_else(|| t.get("type")).and_then(Value::as_str).map(str::to_owned);
            Some((n(t, "id")?, class, frames))
        })
        .collect();
    tileset_atlas(Tileset {
        image: v.get("image")?.as_str()?.to_owned(),
        tile: [n(v, "tilewidth")?, n(v, "tileheight")?],
        spacing: n(v, "spacing").unwrap_or(0),
        margin: n(v, "margin").unwrap_or(0),
        columns: n(v, "columns")?,
        count: n(v, "tilecount")?,
        anims,
    })
}

/// Starling and Sparrow:`<TextureAtlas imagePath="…"><SubTexture name x y width height [frameX frameY frameWidth
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
    Some(Atlas { format: "starling", pages: vec![page?], ..Atlas::default() })
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
/// `{"textures": [{"image": …, "frames": [...]}]}`. Aseprite's adds each frame's `duration` and, in `meta`, the
/// `frameTags` that are its animations (`from` and `to` counting frames in order, played `forward`, `reverse` or
/// `pingpong`).
fn texture_packer(v: &Value) -> Option<Atlas> {
    let image = |o: &Value| o.get("image").and_then(Value::as_str).map(str::to_owned);
    if let Some(textures) = v.get("textures").and_then(Value::as_array) {
        let pages = textures
            .iter()
            .filter_map(|t| Some(Page { image: image(t)?, sprites: tp_frames(t.get("frames")?) }))
            .collect();
        return Some(Atlas { format: "texturepacker", pages, ..Atlas::default() });
    }
    let meta = v.get("meta")?;
    let frames = v.get("frames")?;
    let Some(tags) = meta.get("frameTags").and_then(Value::as_array).filter(|t| !t.is_empty()) else {
        let pages = vec![Page { image: image(meta)?, sprites: tp_frames(frames) }];
        return Some(Atlas { format: "texturepacker", pages, ..Atlas::default() });
    };
    // Aseprite: frames in the order it wrote them (a hash's are read sorted, so put back by their numbers).
    let mut list: Vec<(String, &Value)> = match frames {
        Value::Object(m) => m.iter().map(|(k, f)| (k.clone(), f)).collect(),
        Value::Array(a) => {
            a.iter().map(|f| (f.get("filename").and_then(Value::as_str).unwrap_or("").to_owned(), f)).collect()
        }
        _ => return None,
    };
    if frames.is_object() {
        list.sort_by_cached_key(|(name, _)| natural(name));
    }
    let mut sprites = Vec::new();
    let mut at = Vec::new();
    for (name, f) in &list {
        let ms = field(f, "duration").map_or(FRAME_MS, |d| d.clamp(1.0, 60_000.0) as u32);
        at.push(tp_frame(name, f).map(|s| {
            sprites.push(s);
            (sprites.len() - 1, ms)
        }));
    }
    let animations = tags
        .iter()
        .filter_map(|t| {
            let (from, to) = (field(t, "from")? as usize, field(t, "to")? as usize);
            let ids: Vec<usize> = (from..=to.min(list.len().saturating_sub(1))).collect();
            let order: Vec<usize> = match t.get("direction").and_then(Value::as_str) {
                Some("reverse") => ids.into_iter().rev().collect(),
                Some("pingpong") => {
                    let back: Vec<usize> =
                        ids.iter().rev().skip(1).take(ids.len().saturating_sub(2)).copied().collect();
                    ids.into_iter().chain(back).collect()
                }
                _ => ids,
            };
            let frames: Vec<Frame> = order
                .into_iter()
                .filter_map(|i| at.get(i).copied().flatten())
                .map(|(sprite, ms)| Frame { page: 0, sprite, ms })
                .collect();
            Some(Anim { name: t.get("name")?.as_str()?.to_owned(), frames, looped: true })
        })
        .collect();
    Some(Atlas { format: "aseprite", pages: vec![Page { image: image(meta)?, sprites }], animations, tile: None })
}

/// A name for sorting the way people count: `walk 2` before `walk 10`.
pub(crate) fn natural(s: &str) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut digits = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            if !digits.is_empty() {
                out.push((std::mem::take(&mut text), digits.parse().unwrap_or(u64::MAX)));
                digits.clear();
            }
            text.extend(c.to_lowercase());
        }
    }
    out.push((text, if digits.is_empty() { 0 } else { digits.parse().unwrap_or(u64::MAX) }));
    out
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
    Some(Atlas { format: "libgdx", pages, ..Atlas::default() })
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
    Some(Atlas { format: "cocos2d", pages: vec![Page { image, sprites }], ..Atlas::default() })
}

/// Godot's `SpriteFrames` and `TileSet` resources (`.tres`, Godot 3 and 4). A page is an image the resource names,
/// as it names it (`res://…`).
fn godot(text: &str) -> Option<Atlas> {
    let res = crate::godot::parse(text)?;
    match res.kind()?.as_str() {
        "SpriteFrames" => godot_frames(&res),
        "TileSet" => godot_tiles(&res),
        _ => None,
    }
}

/// The pages of a Godot resource, one per image it names, in the order they're first used.
struct GodotPages {
    paths: HashMap<String, String>,
    pages: Vec<Page>,
    by_ext: HashMap<String, usize>,
}

impl GodotPages {
    fn new(res: &crate::godot::Resource) -> Self {
        GodotPages { paths: res.ext_paths(), pages: Vec::new(), by_ext: HashMap::new() }
    }

    /// The page an `ExtResource(…)` is, `None` when the resource doesn't name it.
    fn of(&mut self, v: &crate::godot::Val) -> Option<usize> {
        let id = v.reference("ExtResource")?;
        if let Some(&i) = self.by_ext.get(&id) {
            return Some(i);
        }
        let image = self.paths.get(&id)?.clone();
        self.pages.push(Page { image, sprites: Vec::new() });
        self.by_ext.insert(id, self.pages.len() - 1);
        Some(self.pages.len() - 1)
    }

    fn add(&mut self, page: usize, sprite: Sprite) -> usize {
        self.pages[page].sprites.push(sprite);
        self.pages[page].sprites.len() - 1
    }
}

/// A rectangle from Godot's numbers (`Rect2(x, y, w, h)`).
fn godot_rect(v: Option<&crate::godot::Val>) -> Option<[u32; 4]> {
    match v?.nums()[..] {
        [x, y, w, h, ..] => Some([px(x)?, px(y)?, px(w)?, px(h)?]),
        _ => None,
    }
}

fn godot_pair(v: Option<&crate::godot::Val>, or: [u32; 2]) -> [u32; 2] {
    match v.map(crate::godot::Val::nums).as_deref() {
        Some([a, b, ..]) => [px(*a).unwrap_or(or[0]), px(*b).unwrap_or(or[1])],
        _ => or,
    }
}

/// A file's name without its folders or extension.
fn godot_stem(path: &str) -> String {
    let file = path.rsplit(['/', '\\']).next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(s, _)| s).to_owned()
}

/// `SpriteFrames`: `animations`, each a `name`, a `speed` (frames a second), whether it loops, and its frames: an
/// `AtlasTexture` (a `region` of an image, with a `margin` for the trimmed canvas) or a whole image, which Godot 4
/// gives as `{"duration": …, "texture": …}` (the duration in frames). A sprite taking a whole image has an empty
/// rectangle until its image's size is known.
fn godot_frames(res: &crate::godot::Resource) -> Option<Atlas> {
    use crate::godot::Val;
    let Val::Arr(list) = res.main()?.prop("animations")? else { return None };
    let mut pages = GodotPages::new(res);
    let mut seen: HashMap<String, Frame> = HashMap::new();
    let mut animations = Vec::new();
    for a in list {
        let name = a.get("name").and_then(Val::str).unwrap_or("default").to_owned();
        let speed = a.get("speed").and_then(Val::num).filter(|s| *s > 0.0 && s.is_finite()).unwrap_or(5.0);
        let looped = a.get("loop").and_then(Val::num).is_none_or(|l| l != 0.0);
        let Some(Val::Arr(frames)) = a.get("frames") else { continue };
        let mut out = Vec::new();
        for (n, f) in frames.iter().enumerate() {
            let (texture, duration) = match f.get("texture") {
                Some(t) => (t, f.get("duration").and_then(Val::num).filter(|d| *d > 0.0).unwrap_or(1.0)),
                None => (f, 1.0),
            };
            let ms = (1000.0 * duration / speed).round().clamp(1.0, 60_000.0) as u32;
            let key = texture
                .reference("SubResource")
                .map(|s| format!("sub:{s}"))
                .or_else(|| texture.reference("ExtResource").map(|e| format!("ext:{e}")));
            let Some(key) = key else { continue };
            if let Some(&frame) = seen.get(&key) {
                out.push(Frame { ms, ..frame });
                continue;
            }
            let placed = if let Some(id) = texture.reference("SubResource") {
                let sub = res.sub(&id).filter(|s| s.attr_str("type").as_deref() == Some("AtlasTexture"));
                sub.and_then(|sub| {
                    let page = pages.of(sub.prop("atlas")?)?;
                    let rect = godot_rect(sub.prop("region"))?;
                    let trim = godot_rect(sub.prop("margin")).and_then(|[l, t, mw, mh]| {
                        trim_of([rect[2] + mw, rect[3] + mh], [i64::from(l), i64::from(t)], [rect[2], rect[3]])
                    });
                    let sprite = Sprite { name: format!("{name}_{n}"), rect, turn: Turn::Upright, trim };
                    Some((page, pages.add(page, sprite)))
                })
            } else {
                pages.of(texture).map(|page| {
                    let sprite = Sprite {
                        name: godot_stem(&pages.pages[page].image),
                        rect: [0, 0, 0, 0],
                        turn: Turn::Upright,
                        trim: None,
                    };
                    (page, pages.add(page, sprite))
                })
            };
            let Some((page, sprite)) = placed else { continue };
            let frame = Frame { page, sprite, ms };
            seen.insert(key, frame);
            out.push(frame);
        }
        if !out.is_empty() {
            animations.push(Anim { name, frames: out, looped });
        }
    }
    Some(Atlas { format: "godot-spriteframes", pages: pages.pages, animations, tile: None })
}

/// `TileSet`. Godot 4's: `sources/N`, each a `TileSetAtlasSource` naming its image, the tile size
/// (`texture_region_size`), `margins` and `separation`, and a `X:Y/0` line per tile (`X:Y/size_in_atlas` for one
/// bigger than a cell). Godot 3's: numbered tiles (`N/texture`, `N/region`, `N/tile_mode`): a single tile is its
/// region, an autotile or atlas tile is cut into `N/autotile/tile_size` cells `N/autotile/spacing` apart.
fn godot_tiles(res: &crate::godot::Resource) -> Option<Atlas> {
    use crate::godot::Val;
    let main = res.main()?;
    let mut pages = GodotPages::new(res);
    let mut tile = None;
    let sources: Vec<&crate::godot::Section> = main
        .props
        .iter()
        .filter(|(k, _)| k.starts_with("sources/"))
        .filter_map(|(_, v)| res.sub(&v.reference("SubResource")?))
        .filter(|s| s.attr_str("type").as_deref() == Some("TileSetAtlasSource"))
        .collect();
    for (si, src) in sources.iter().enumerate() {
        let Some(page) = src.prop("texture").and_then(|t| pages.of(t)) else { continue };
        let [rw, rh] = godot_pair(src.prop("texture_region_size"), [16, 16]);
        let [mx, my] = godot_pair(src.prop("margins"), [0, 0]);
        let [sx, sy] = godot_pair(src.prop("separation"), [0, 0]);
        if rw == 0 || rh == 0 {
            continue;
        }
        tile.get_or_insert([rw, rh]);
        for (k, _) in &src.props {
            let Some((coords, "0")) = k.split_once('/') else { continue };
            let Some((x, y)) = coords.split_once(':') else { continue };
            let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) else { continue };
            let [w, h] = godot_pair(src.prop(&format!("{coords}/size_in_atlas")), [1, 1]);
            let (w, h) = (w.max(1), h.max(1));
            let rect = [mx + x * (rw + sx), my + y * (rh + sy), rw * w + sx * (w - 1), rh * h + sy * (h - 1)];
            let name = if sources.len() > 1 { format!("tile_{si}_{x}_{y}") } else { format!("tile_{x}_{y}") };
            pages.add(page, Sprite { name, rect, turn: Turn::Upright, trim: None });
        }
    }
    // Godot 3: tiles numbered by the first part of their keys.
    let mut ids: Vec<u64> = main.props.iter().filter_map(|(k, _)| k.strip_suffix("/texture")?.parse().ok()).collect();
    ids.sort_unstable();
    ids.dedup();
    for id in ids {
        let prop = |k: &str| main.prop(&format!("{id}/{k}"));
        let Some(page) = prop("texture").and_then(|t| pages.of(t)) else { continue };
        let name = prop("name").and_then(Val::str).map_or_else(|| format!("tile_{id}"), str::to_owned);
        let region = godot_rect(prop("region")).unwrap_or([0, 0, 0, 0]);
        let mode = prop("tile_mode").and_then(Val::num).unwrap_or(0.0);
        if mode == 0.0 {
            if region[2] > 0 && region[3] > 0 {
                tile.get_or_insert([region[2], region[3]]);
            }
            pages.add(page, Sprite { name, rect: region, turn: Turn::Upright, trim: None });
            continue;
        }
        let [tw, th] = godot_pair(prop("autotile/tile_size"), [16, 16]);
        let gap = prop("autotile/spacing").and_then(Val::num).and_then(px).unwrap_or(0);
        if tw == 0 || th == 0 || region[2] == 0 || region[3] == 0 {
            continue;
        }
        tile.get_or_insert([tw, th]);
        let (cols, rows) = ((region[2] + gap) / (tw + gap), (region[3] + gap) / (th + gap));
        for r in 0..rows {
            for c in 0..cols {
                let rect = [region[0] + c * (tw + gap), region[1] + r * (th + gap), tw, th];
                pages.add(page, Sprite { name: format!("{name}_{c}_{r}"), rect, turn: Turn::Upright, trim: None });
            }
        }
    }
    Some(Atlas { format: "godot-tileset", pages: pages.pages, animations: Vec::new(), tile })
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

/// The pack's image an atlas file names. As named, relative to the file; a Godot `res://` path from the Godot
/// project's root, which is the file's folder or one above it; or else, for an atlas of one page, the image beside
/// it with its name (a page renamed after packing).
fn find_page(by_path: &HashMap<String, usize>, file_path: &str, image: &str, one_page: bool) -> Option<usize> {
    if let Some(rest) = image.strip_prefix("res://") {
        let want = normalize(rest).to_lowercase();
        let mut dir = dir_of(file_path);
        loop {
            let at = if dir.is_empty() { want.clone() } else { format!("{}/{want}", dir.to_lowercase()) };
            if let Some(&i) = by_path.get(&at) {
                return Some(i);
            }
            if dir.is_empty() {
                return None;
            }
            dir = dir_of(dir);
        }
    }
    let at = normalize(&format!("{}/{image}", dir_of(file_path)));
    by_path.get(&at.to_lowercase()).copied().or_else(|| {
        let ext = crate::ingest::ext_of(image);
        let base = file_path.rsplit_once('.').map_or(file_path, |(b, _)| b);
        one_page.then(|| by_path.get(&format!("{base}.{ext}").to_lowercase()).copied()).flatten()
    })
}

// ---------------------------------------------------------------------------------------------
// The library side

/// The files atlases are read from, as SQL.
const EXTS: &str = "'xml', 'json', 'atlas', 'plist', 'tres', 'tsx', 'tsj'";

#[derive(Debug)]
struct Texture {
    id: String,
    path: String,
    hash: String,
    /// From its meta; 0 when it doesn't say.
    size: [u32; 2],
}

/// An atlas file found to be a sheet.
struct Found {
    file_id: String,
    file_path: String,
    format: &'static str,
    /// Its pages, by texture.
    pages: Vec<usize>,
    sprites: usize,
    /// The sheet written again (new, or changed): it, its summary, the words its sprites add, its thumbnail.
    new: Option<New>,
    /// The loose images that are its sprites, by texture, when they were looked for again.
    loose: Option<Vec<usize>>,
}

struct New {
    blob: crate::blobs::StoredBlob,
    summary: Value,
    words: String,
    thumb: Option<crate::blobs::StoredBlob>,
}

/// Leave out a tile set's empty tiles (see [`crate::grid`]), its animations following the tiles that stay.
fn drop_empty(atlas: &mut Atlas, imgs: &[Option<RgbaImage>]) {
    let mut moved: Vec<Vec<Option<usize>>> = Vec::new();
    for (p, img) in atlas.pages.iter_mut().zip(imgs) {
        let Some(img) = img else {
            moved.push((0..p.sprites.len()).map(Some).collect());
            continue;
        };
        // The colour between tiles, when there's space between them.
        let background = match (p.sprites.first(), p.sprites.get(1)) {
            (Some(a), Some(b)) if b.rect[1] == a.rect[1] && b.rect[0] > a.rect[0] + a.rect[2] => {
                let (x, y) = (a.rect[0] + a.rect[2], a.rect[1]);
                (x < img.width() && y < img.height()).then(|| *img.get_pixel(x, y))
            }
            _ => None,
        };
        let mut map = Vec::with_capacity(p.sprites.len());
        let mut kept = Vec::new();
        for s in p.sprites.drain(..) {
            let [x, y, w, h] = s.rect;
            if x + w <= img.width() && y + h <= img.height() && !crate::grid::empty(img, s.rect, background) {
                map.push(Some(kept.len()));
                kept.push(s);
            } else {
                map.push(None);
            }
        }
        p.sprites = kept;
        moved.push(map);
    }
    for a in &mut atlas.animations {
        a.frames.retain_mut(|f| match moved.get(f.page).and_then(|m| m.get(f.sprite)).copied().flatten() {
            Some(i) => {
                f.sprite = i;
                true
            }
            None => false,
        });
    }
    atlas.animations.retain(|a| !a.frames.is_empty());
}

/// A file that may be a sheet: an atlas, or a note of a grid.
struct Candidate {
    id: String,
    path: String,
    hash: String,
    /// Lowercased.
    ext: String,
    /// The sheet it was, if it was one.
    was: Option<String>,
    has_thumb: bool,
    /// Whether its sprites' loose images were looked for.
    matched: bool,
}

/// A file found to be this sheet: written again when it changed, its sprites' loose images looked for again when it
/// changed or they never were.
fn finish(
    state: &AppState,
    rt: &Handle,
    c: Candidate,
    atlas: &Atlas,
    sheet: &crate::spritesheet::Sheet,
    by_path: &HashMap<String, usize>,
    loose: impl FnOnce() -> Vec<usize>,
) -> Result<Found> {
    let page_ids: Vec<usize> =
        sheet.pages.iter().filter_map(|p| by_path.get(&p.image.to_lowercase()).copied()).collect();
    let json = serde_json::to_vec(sheet)?;
    let sheet_hash = blake3::hash(&json).to_hex().to_string();
    let unchanged = c.was.as_deref() == Some(sheet_hash.as_str()) && c.has_thumb;
    let mut f = Found {
        file_id: c.id,
        file_path: c.path,
        format: atlas.format,
        pages: page_ids,
        sprites: sheet.sprites.len(),
        new: None,
        loose: (!unchanged || !c.matched).then(loose),
    };
    if unchanged {
        return Ok(f);
    }
    let thumb = sheet_thumbnail(state, rt, sheet).map(|b| rt.block_on(state.blobs.put_cache_bytes(b))).transpose()?;
    let mut words: Vec<String> = Vec::new();
    let mut seen = HashSet::new();
    for s in sheet.sprites.iter().take(MAX_WORDS) {
        for w in tagger::fts_words(&s.name).split_whitespace() {
            if seen.insert(w.to_owned()) {
                words.push(w.to_owned());
            }
        }
    }
    let blob = rt.block_on(state.blobs.put_bytes(json, AssetKind::Sheet))?;
    f.new = Some(New { blob, summary: sheet.summary(), words: words.join(" "), thumb });
    Ok(f)
}

/// An image of the pack, unless it's too big to read.
fn load_texture(state: &AppState, rt: &Handle, t: &Texture) -> Option<RgbaImage> {
    if t.size[0] > MAX_SIDE || t.size[1] > MAX_SIDE {
        return None;
    }
    let bytes = rt.block_on(state.blobs.read_all(&t.hash)).ok()?;
    Some(image::load_from_memory(&bytes).ok()?.to_rgba8())
}

/// The images below a note's folder that are on the grid it describes and not already a sheet's pages, cut into their
/// tiles: the atlas, each page's image (by texture), and the images.
fn grid_pages(
    state: &AppState,
    rt: &Handle,
    note: &crate::grid::Note,
    note_path: &str,
    textures: &[Texture],
    claimed: &HashSet<usize>,
) -> Option<(Atlas, Vec<usize>, Vec<RgbaImage>)> {
    let dir = dir_of(note_path).to_lowercase();
    let fits: Vec<(usize, crate::grid::Grid)> = textures
        .iter()
        .enumerate()
        .filter(|(i, _)| !claimed.contains(i))
        .filter_map(|(i, t)| {
            let rel = if dir.is_empty() {
                t.path.as_str()
            } else {
                let below = t.path.to_lowercase().strip_prefix(&format!("{dir}/"))?.len();
                &t.path[t.path.len() - below..]
            };
            if !crate::grid::candidate(note, note_path, rel) {
                return None;
            }
            Some((i, crate::grid::fit(note, rel.rsplit('/').next().unwrap_or(rel), t.size)?))
        })
        .collect();
    let several = fits.len() > 1;
    let (mut pages, mut page_textures, mut imgs, mut tiles) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut total = 0;
    for (i, grid) in fits {
        let t = &textures[i];
        let Some(img) = load_texture(state, rt, t) else { continue };
        if [img.width(), img.height()] != t.size {
            continue;
        }
        let stem = crate::spritesheet::image_stem(t.path.rsplit('/').next().unwrap_or(&t.path));
        let page = crate::grid::page(&img, &grid, &t.path, several.then_some(stem));
        if page.sprites.is_empty() || total + page.sprites.len() > MAX_SPRITES {
            continue;
        }
        total += page.sprites.len();
        pages.push(page);
        page_textures.push(i);
        imgs.push(img);
        tiles.push(grid.tile);
    }
    (!pages.is_empty()).then(|| (crate::grid::atlas(pages, &tiles), page_textures, imgs))
}

/// The loose images in a note's kit that are its tiles: each of a tile's size, with the tile's name (`tile_0000`) or,
/// failing that, its picture (Kenney's icons have names of their own: `arrow_clockwise.png`), give or take what saving
/// it again changed. Of several, one with the tile's name is, then the closest.
fn grid_loose(
    atlas: &Atlas,
    imgs: &[RgbaImage],
    pages: &[usize],
    textures: &[Texture],
    kit: &str,
    claimed: &HashSet<usize>,
    load: impl Fn(&Texture) -> Option<RgbaImage>,
) -> Vec<usize> {
    let kit = kit.to_lowercase();
    let sizes: HashSet<[u32; 2]> =
        atlas.pages.iter().flat_map(|p| p.sprites.iter().map(|s| [s.rect[2], s.rect[3]])).collect();
    let loose: Vec<(usize, String, RgbaImage, [u8; 64])> = textures
        .iter()
        .enumerate()
        .filter(|(i, t)| {
            !pages.contains(i)
                && !claimed.contains(i)
                && sizes.contains(&t.size)
                && (kit.is_empty() || t.path.to_lowercase().starts_with(&format!("{kit}/")))
        })
        .take(MAX_SPRITES)
        .filter_map(|(i, t)| {
            let img = load(t)?;
            let sig = crate::grid::signature(&img);
            Some((i, sprite_key(&t.path), img, sig))
        })
        .collect();
    let mut out = Vec::new();
    if loose.is_empty() {
        return out;
    }
    for (page, img) in atlas.pages.iter().zip(imgs) {
        for s in &page.sprites {
            let Some(tile) = cut(img, s) else { continue };
            let sig = crate::grid::signature(&tile);
            let key = sprite_key(&s.name);
            let best = loose
                .iter()
                .filter(|(_, k, l, ls)| {
                    l.dimensions() == tile.dimensions() && (*k == key || crate::grid::alike(&sig, ls))
                })
                .filter_map(|(i, k, l, _)| difference(&tile, l).map(|off| (*k != key, off, *i)))
                .min();
            out.extend(best.map(|(_, _, i)| i));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Find the pack's atlas files and notes of a tile sheet's grid (see [`crate::grid`]), and bring the library in line:
/// each is a `sheet`, kept with it as a sheet (see [`crate::spritesheet`]) and linked to its pages (`texture`, so its
/// download brings them), which say in their meta which atlas they're a page of. A file that isn't an atlas any more
/// is what its extension says again. Returns how many sheets the pack has.
pub(crate) fn detect_pack(state: &AppState, rt: &Handle, conn: &mut Connection, pack_id: &str) -> Result<usize> {
    let files: Vec<Candidate> = {
        // Atlases first, so a note of a grid doesn't take their pages.
        let mut stmt = conn.prepare(&format!(
            "SELECT a.id, a.path, a.blob_hash, a.ext,
                    CASE WHEN a.kind = 'sheet' THEN d.blob_hash END, a.thumb_hash IS NOT NULL,
                    json_extract(a.meta, '$.loose') IS NOT NULL
               FROM assets a JOIN blobs b ON b.hash = a.blob_hash
               LEFT JOIN derivatives d ON d.asset_id = a.id AND d.kind = 'sheet'
              WHERE a.pack_id = ?1 AND (lower(a.ext) IN ({EXTS}) AND b.size <= {MAX_FILE}
                                        OR lower(a.ext) = 'txt' AND b.size <= {})
              ORDER BY lower(a.ext) = 'txt', a.path",
            crate::grid::MAX_NOTE
        ))?;
        stmt.query_map([pack_id], |r| {
            Ok(Candidate {
                id: r.get(0)?,
                path: r.get(1)?,
                hash: r.get(2)?,
                ext: r.get::<_, String>(3)?.to_lowercase(),
                was: r.get(4)?,
                has_thumb: r.get(5)?,
                matched: r.get(6)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    let textures: Vec<Texture> = {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.path, a.blob_hash, COALESCE(json_extract(a.meta, '$.width'), 0),
                    COALESCE(json_extract(a.meta, '$.height'), 0)
               FROM assets a WHERE a.pack_id = ?1 AND a.kind = 'texture' ORDER BY a.path",
        )?;
        stmt.query_map([pack_id], |r| {
            Ok(Texture {
                id: r.get(0)?,
                path: r.get(1)?,
                hash: r.get(2)?,
                size: [r.get::<_, f64>(3)? as u32, r.get::<_, f64>(4)? as u32],
            })
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    let by_path: HashMap<String, usize> =
        textures.iter().enumerate().map(|(i, t)| (t.path.to_lowercase(), i)).collect();
    let mut by_stem: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, t) in textures.iter().enumerate() {
        by_stem.entry(sprite_key(&t.path)).or_default().push(i);
    }
    // Sizes read from the images themselves, for those whose meta doesn't say.
    let mut sizes: HashMap<usize, Option<[u32; 2]>> = HashMap::new();
    // The images already a sheet's pages.
    let mut claimed: HashSet<usize> = HashSet::new();
    let mut found = Vec::new();
    for c in files {
        let text = rt.block_on(state.blobs.read_all(&c.hash))?;
        let text = String::from_utf8_lossy(&text);
        if c.ext == "txt" {
            let Some(note) = crate::grid::parse_note(&text) else { continue };
            let Some((atlas, page_textures, imgs)) = grid_pages(state, rt, &note, &c.path, &textures, &claimed) else {
                continue;
            };
            let pages: Vec<Option<crate::spritesheet::Page>> = page_textures
                .iter()
                .zip(&imgs)
                .map(|(&i, img)| {
                    let t = &textures[i];
                    Some(crate::spritesheet::Page {
                        image: t.path.clone(),
                        size: [img.width(), img.height()],
                        asset: Some(t.id.clone()),
                        hash: Some(t.hash.clone()),
                    })
                })
                .collect();
            let Some(sheet) = crate::spritesheet::Sheet::from_atlas(&atlas, &pages) else { continue };
            let kit = crate::grid::kit_dir(&c.path).to_owned();
            let f = finish(state, rt, c, &atlas, &sheet, &by_path, || {
                grid_loose(&atlas, &imgs, &page_textures, &textures, &kit, &claimed, |t| load_texture(state, rt, t))
            })?;
            claimed.extend(f.pages.iter().copied());
            found.push(f);
            continue;
        }
        let Some(mut atlas) = parse(&c.ext, &text) else { continue };
        if atlas.pages.iter().map(|p| p.sprites.len()).sum::<usize>() > MAX_SPRITES {
            tracing::debug!(atlas = %c.path, "atlas has too many sprites to read");
            continue;
        }
        let one_page = atlas.pages.len() == 1;
        let page_textures: Vec<Option<usize>> = atlas
            .pages
            .iter()
            .map(|p| {
                let found = find_page(&by_path, &c.path, &p.image, one_page);
                if found.is_none() {
                    tracing::debug!(atlas = %c.path, page = %p.image, "atlas page not in the pack");
                }
                found
            })
            .collect();
        if atlas.format == "tiled" {
            let imgs: Vec<Option<RgbaImage>> =
                page_textures.iter().map(|&i| i.and_then(|i| load_texture(state, rt, &textures[i]))).collect();
            drop_empty(&mut atlas, &imgs);
        }
        let pages: Vec<Option<crate::spritesheet::Page>> = page_textures
            .iter()
            .map(|&i| {
                let i = i?;
                let t = &textures[i];
                let size = if t.size[0] > 0 && t.size[1] > 0 {
                    t.size
                } else {
                    (*sizes.entry(i).or_insert_with(|| {
                        let bytes = rt.block_on(state.blobs.read_all(&t.hash)).ok()?;
                        let (w, h) = image::ImageReader::new(std::io::Cursor::new(&bytes[..]))
                            .with_guessed_format()
                            .ok()?
                            .into_dimensions()
                            .ok()?;
                        Some([w, h])
                    }))?
                };
                Some(crate::spritesheet::Page {
                    image: t.path.clone(),
                    size,
                    asset: Some(t.id.clone()),
                    hash: Some(t.hash.clone()),
                })
            })
            .collect();
        let Some(sheet) = crate::spritesheet::Sheet::from_atlas(&atlas, &pages) else { continue };
        let sheet_path = c.path.clone();
        let f = finish(state, rt, c, &atlas, &sheet, &by_path, || {
            loose_copies(state, rt, &atlas, &page_textures, &textures, &by_stem, &sheet_path)
        })?;
        claimed.extend(f.pages.iter().copied());
        found.push(f);
    }
    let n = found.len();
    write(conn, pack_id, &textures, &found)?;
    Ok(n)
}

/// Sprites whose names a sheet adds to its words, at most.
const MAX_WORDS: usize = 500;

/// What a sprite and a loose image of it are both known by: the file's name without its folder or extension.
fn sprite_key(name: &str) -> String {
    crate::spritesheet::image_stem(name.rsplit(['/', '\\']).next().unwrap_or(name)).to_lowercase()
}

/// How far apart two channels of a sprite and its loose image may be: the sheet was saved again, not drawn again.
const CHANNEL_SLACK: u8 = 24;

/// The pack's loose images that are this sheet's sprites. A loose image is a sprite's when it has the sprite's name
/// (whatever its folder) and its picture, give or take what saving it again changed, with or without the transparent
/// margin the packer trimmed off. Of several (the same button in each of a pack's styles), the closest is, then the
/// one whose path shares most words with the sheet's.
fn loose_copies(
    state: &AppState,
    rt: &Handle,
    atlas: &Atlas,
    page_textures: &[Option<usize>],
    textures: &[Texture],
    by_stem: &HashMap<String, Vec<usize>>,
    sheet_path: &str,
) -> Vec<usize> {
    let sheet_words = path_words(sheet_path);
    let pages: HashSet<usize> = page_textures.iter().flatten().copied().collect();
    let load = |t: &Texture| load_texture(state, rt, t);
    let mut out = Vec::new();
    for (page, &tex) in atlas.pages.iter().zip(page_textures) {
        let wanted: Vec<(&Sprite, &Vec<usize>)> =
            page.sprites.iter().filter_map(|s| by_stem.get(&sprite_key(&s.name)).map(|c| (s, c))).collect();
        let Some(tex) = tex.filter(|_| !wanted.is_empty()) else { continue };
        let Some(img) = load(&textures[tex]) else { continue };
        for (s, candidates) in wanted {
            let Some(sprite) = cut(&img, s) else { continue };
            let size = [sprite.width(), sprite.height()];
            let mut best: Option<(usize, usize, usize)> = None;
            for &c in candidates {
                let t = &textures[c];
                // Smaller than the sprite, or bigger when the margin it had is known: not it.
                let known = t.size != [0, 0];
                if pages.contains(&c)
                    || known && t.size != size && (s.trim.is_some() || t.size[0] < size[0] || t.size[1] < size[1])
                {
                    continue;
                }
                let Some(off) = load(t).and_then(|l| difference(&sprite, &l)) else { continue };
                let unshared = usize::MAX - path_words(&t.path).intersection(&sheet_words).count();
                if best.is_none_or(|b| (off, unshared, c) < b) {
                    best = Some((off, unshared, c));
                }
            }
            out.extend(best.map(|(_, _, c)| c));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The words of a path, lowercased, for telling a pack's styles apart.
fn path_words(path: &str) -> HashSet<String> {
    path.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

/// How many pixels a sprite and a loose image differ in, when they're the same picture: at most one in a hundred,
/// each channel within [`CHANNEL_SLACK`] (colour only where both are mostly opaque, as saving again changes what
/// hides under transparency). One with a transparent margin the other hasn't is compared without it.
fn difference(a: &RgbaImage, b: &RgbaImage) -> Option<usize> {
    if a.dimensions() != b.dimensions() {
        let (a, b) = (opaque_part(a)?, opaque_part(b)?);
        return (a.dimensions() == b.dimensions()).then(|| difference(&a, &b)).flatten();
    }
    let near = |x: u8, y: u8| x.abs_diff(y) <= CHANNEL_SLACK;
    let off = a
        .pixels()
        .zip(b.pixels())
        .filter(|(p, q)| {
            !near(p[3], q[3]) || p[3].min(q[3]) >= 128 && !(near(p[0], q[0]) && near(p[1], q[1]) && near(p[2], q[2]))
        })
        .count();
    (off * 100 <= (a.width() * a.height()) as usize).then_some(off)
}

/// An image without its transparent margin; `None` when it's all transparent.
fn opaque_part(img: &RgbaImage) -> Option<RgbaImage> {
    let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0, 0);
    for (x, y, p) in img.enumerate_pixels() {
        if p[3] > CHANNEL_SLACK {
            (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
        }
    }
    (x0 <= x1).then(|| imageops::crop_imm(img, x0, y0, x1 - x0 + 1, y1 - y0 + 1).to_image())
}

/// A sheet's thumbnail, from the pages its first sprites are on.
fn sheet_thumbnail(state: &AppState, rt: &Handle, sheet: &crate::spritesheet::Sheet) -> Option<Vec<u8>> {
    let mut pages = HashMap::new();
    for i in crate::spritesheet::shown_sprites(sheet) {
        let p = sheet.sprites[i].page;
        if pages.contains_key(&p) {
            continue;
        }
        let page = &sheet.pages[p];
        if page.size[0] > MAX_SIDE || page.size[1] > MAX_SIDE {
            continue;
        }
        let Some(hash) = &page.hash else { continue };
        let img = rt.block_on(state.blobs.read_all(hash)).ok().and_then(|b| image::load_from_memory(&b).ok());
        if let Some(img) = img {
            pages.insert(p, img.to_rgba8());
        }
    }
    let (img, exact) = crate::spritesheet::thumbnail(sheet, &pages)?;
    crate::analyze::thumb_bytes(img, exact).ok()
}

/// Write what was found: each sheet's file a `sheet` (its sheet, thumbnail, words and category brought up to date if
/// it changed) linked to its pages, the pages marked, and files that were sheets and aren't any more put back.
fn write(conn: &mut Connection, pack_id: &str, textures: &[Texture], found: &[Found]) -> Result<()> {
    let marked: bool = conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = ?1
                          AND (a.kind = 'sheet' OR json_extract(a.meta, '$.atlas') IS NOT NULL))",
        [pack_id],
        |r| r.get(0),
    )?;
    if found.is_empty() && !marked {
        return Ok(());
    }
    let now = db::now();
    let tx = conn.transaction()?;
    let before: Vec<(String, String, String)> = {
        let mut stmt = tx.prepare("SELECT id, path, ext FROM assets WHERE pack_id = ?1 AND kind = 'sheet'")?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    tx.execute(
        "UPDATE assets SET meta = json_remove(meta, '$.atlas')
          WHERE pack_id = ?1 AND json_extract(meta, '$.atlas') IS NOT NULL",
        [pack_id],
    )?;
    let tags_of = |id: &str| -> Result<tagger::TagSet> {
        let mut set = tagger::TagSet::default();
        let mut stmt = tx.prepare_cached("SELECT tag, source, confidence FROM asset_tags WHERE asset_id = ?1")?;
        let rows =
            stmt.query_map([id], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, f64>(2)?)))?;
        for row in rows {
            let (tag, source, confidence) = row?;
            set.add(&tag, TagSource::parse(&source), confidence as f32);
        }
        Ok(set)
    };
    let mut kept = HashSet::new();
    for f in found {
        kept.insert(f.file_id.clone());
        for &p in &f.pages {
            tx.execute(
                "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'texture') ON CONFLICT DO NOTHING",
                params![f.file_id, textures[p].id],
            )?;
            tx.execute(
                "UPDATE assets SET meta = json_set(meta, '$.atlas', json(?2)) WHERE id = ?1",
                params![
                    textures[p].id,
                    json!({ "file": f.file_path, "format": f.format, "sprites": f.sprites }).to_string()
                ],
            )?;
        }
        if let Some(loose) = &f.loose {
            tx.execute("DELETE FROM asset_links WHERE from_id = ?1 AND rel = 'sprite'", [&f.file_id])?;
            for &t in loose {
                tx.execute(
                    "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'sprite') ON CONFLICT DO NOTHING",
                    params![f.file_id, textures[t].id],
                )?;
            }
            tx.execute(
                "UPDATE assets SET meta = json_set(meta, '$.loose', ?2) WHERE id = ?1",
                params![f.file_id, loose.len() as i64],
            )?;
        }
        let Some(new) = &f.new else { continue };
        for b in [Some(&new.blob), new.thumb.as_ref()].into_iter().flatten() {
            tx.execute(
                "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
                params![b.hash, b.size as i64, now],
            )?;
        }
        tx.execute(
            "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, 'sheet', ?2, ?3)
             ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash, created_at = excluded.created_at",
            params![f.file_id, new.blob.hash, now],
        )?;
        let category = tagger::categorize(AssetKind::Sheet, &tags_of(&f.file_id)?);
        tx.execute(
            "UPDATE assets SET kind = 'sheet', meta = json_set(meta, '$.sheet', json(?2)), words = ?3,
                    thumb_hash = COALESCE(?4, thumb_hash), updated_at = ?5,
                    category = CASE WHEN human_edited THEN category ELSE ?6 END
              WHERE id = ?1",
            params![
                f.file_id,
                new.summary.to_string(),
                format!("{} sprite sheet {}", tagger::fts_words(&f.file_path), new.words),
                new.thumb.as_ref().map(|t| t.hash.clone()),
                now,
                category,
            ],
        )?;
        db::reindex_asset(&tx, &f.file_id)?;
    }
    for (id, path, ext) in before {
        if kept.contains(&id) {
            continue;
        }
        let kind = AssetKind::from_ext(&ext.to_lowercase());
        let category = tagger::categorize(kind, &tags_of(&id)?);
        tx.execute("DELETE FROM derivatives WHERE asset_id = ?1 AND kind = 'sheet'", [&id])?;
        tx.execute("DELETE FROM asset_links WHERE from_id = ?1 AND rel = 'sprite'", [&id])?;
        tx.execute(
            "UPDATE assets SET kind = ?2, meta = json_remove(meta, '$.sheet', '$.loose'), words = ?3, thumb_hash = NULL,
                    updated_at = ?4, category = CASE WHEN human_edited THEN category ELSE ?5 END
              WHERE id = ?1",
            params![id, kind.as_str(), tagger::fts_words(&path), now, category],
        )?;
        db::reindex_asset(&tx, &id)?;
    }
    crate::links::assign_families(&tx, pack_id)?;
    tx.commit()?;
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
    fn tiled_tilesets_xml_and_json() {
        let tsx = r#"<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.10.2" name="town" tilewidth="16" tileheight="16" spacing="1" margin="2" tilecount="6" columns="3">
 <image source="../Tilemap/tilemap.png" width="54" height="37"/>
 <tile id="1" type="water">
  <animation>
   <frame tileid="1" duration="200"/>
   <frame tileid="2" duration="200"/>
  </animation>
 </tile>
 <tile id="4"/>
</tileset>"#;
        let a = parse("tsx", tsx).unwrap();
        assert_eq!(a.format, "tiled");
        assert_eq!(a.tile, Some([16, 16]));
        assert_eq!(a.pages[0].image, "../Tilemap/tilemap.png");
        let s = &a.pages[0].sprites;
        assert_eq!(s.len(), 6);
        assert_eq!((s[0].name.as_str(), s[0].rect), ("tile_0000", [2, 2, 16, 16]));
        assert_eq!((s[4].name.as_str(), s[4].rect), ("tile_0004", [19, 19, 16, 16]));
        assert_eq!(a.animations.len(), 1);
        assert_eq!(a.animations[0].name, "water");
        assert_eq!(a.animations[0].frames.iter().map(|f| (f.sprite, f.ms)).collect::<Vec<_>>(), [(1, 200), (2, 200)]);
        // A tileset of separate images, a map and a TypeScript file aren't sheets.
        let collection = r#"<tileset tilewidth="64" tileheight="64" tilecount="2" columns="0"><tile id="0"><image source="a.png"/></tile></tileset>"#;
        assert_eq!(parse("tsx", collection), None);
        assert_eq!(parse("tsx", "export const App = () => <div className=\"app\" />;"), None);

        let json = r#"{"type": "tileset", "image": "tiles.png", "tilewidth": 8, "tileheight": 8, "columns": 2, "tilecount": 4,
            "tiles": [{"id": 3, "animation": [{"tileid": 3, "duration": 0}, {"tileid": 0, "duration": 50}]}]}"#;
        let a = parse("json", json).unwrap();
        assert_eq!(a.format, "tiled");
        assert_eq!(a.pages[0].sprites[3].rect, [8, 8, 8, 8]);
        assert_eq!(a.animations[0].name, "tile_0003");
        assert_eq!(
            a.animations[0].frames.iter().map(|f| (f.sprite, f.ms)).collect::<Vec<_>>(),
            [(3, FRAME_MS), (0, 50)]
        );
        assert_eq!(parse("tsj", json).map(|a| a.pages[0].sprites.len()), Some(4));
        assert_eq!(parse("json", r#"{"type": "map", "layers": [], "tilewidth": 8, "tileheight": 8}"#), None);

        // Empty tiles are left out once the image is read, and the animations follow.
        let mut a = parse("json", json).unwrap();
        let mut img = RgbaImage::new(16, 16);
        for (x, y) in [(0, 0), (8, 8)] {
            for yy in y..y + 8 {
                for xx in x..x + 8 {
                    img.put_pixel(xx, yy, Rgba([200, 50, 50, 255]));
                }
            }
        }
        drop_empty(&mut a, &[Some(img)]);
        assert_eq!(a.pages[0].sprites.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["tile_0000", "tile_0003"]);
        assert_eq!(a.animations[0].frames.iter().map(|f| f.sprite).collect::<Vec<_>>(), [1, 0]);
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
    fn pages_found_as_named() {
        let by_path: HashMap<String, usize> =
            [("pack/img/a.png", 0), ("pack/sheets/b.png", 1), ("pack/godot/art/c.png", 2)]
                .into_iter()
                .map(|(p, i)| (p.to_owned(), i))
                .collect();
        assert_eq!(normalize("Sheets/../img/./a.png"), "img/a.png");
        assert_eq!(find_page(&by_path, "pack/sheets/a.json", "../img/A.png", false), Some(0));
        // Renamed after packing: the atlas's own name, when it has one page.
        assert_eq!(find_page(&by_path, "pack/sheets/b.xml", "atlas.png", true), Some(1));
        assert_eq!(find_page(&by_path, "pack/sheets/b.xml", "atlas.png", false), None);
        // Godot's, from the project's root above the resource.
        assert_eq!(find_page(&by_path, "pack/godot/res/hero.tres", "res://art/c.png", false), Some(2));
        assert_eq!(find_page(&by_path, "pack/godot/hero.tres", "res://art/c.png", false), Some(2));
        assert_eq!(find_page(&by_path, "pack/godot/hero.tres", "res://art/d.png", false), None);
    }

    #[test]
    fn sprites_known_by_their_file_name() {
        assert_eq!(sprite_key("PNG/Default/Blue/button_rectangle.png"), "button_rectangle");
        assert_eq!(sprite_key("button_rectangle"), "button_rectangle");
        assert_eq!(sprite_key("Hero_01.PNG"), "hero_01");
    }

    #[test]
    fn a_loose_image_is_the_sprite_saved_again_with_or_without_its_margin() {
        let pic = RgbaImage::from_fn(10, 10, |x, y| Rgba([x as u8 * 20, y as u8 * 20, 90, 255]));
        assert_eq!(difference(&pic, &pic), Some(0));

        // Saved again: every channel a little off, and what hides under transparency changed.
        let mut again = RgbaImage::from_fn(10, 10, |x, y| Rgba([x as u8 * 20 + 10, y as u8 * 20, 80, 255]));
        let mut clear = pic.clone();
        clear.put_pixel(0, 0, Rgba([0, 0, 0, 0]));
        again.put_pixel(0, 0, Rgba([255, 255, 255, 0]));
        assert_eq!(difference(&clear, &again), Some(0));

        // Another picture.
        let other = RgbaImage::from_fn(10, 10, |x, y| Rgba([y as u8 * 20, x as u8 * 20, 90, 255]));
        assert_eq!(difference(&pic, &other), None);

        // The packer trimmed the loose image's transparent margin off.
        let padded = RgbaImage::from_fn(14, 12, |x, y| {
            if (2..12).contains(&x) && (1..11).contains(&y) { *pic.get_pixel(x - 2, y - 1) } else { Rgba([0, 0, 0, 0]) }
        });
        assert_eq!(difference(&pic, &padded), Some(0));
        assert_eq!(difference(&other, &padded), None);
        assert_eq!(difference(&RgbaImage::new(4, 4), &padded), None);
    }
}
