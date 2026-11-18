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

mod detect;
mod engines;
mod packers;

use std::collections::HashMap;

use image::{RgbaImage, imageops};
use quick_xml::events::BytesStart;
use serde_json::Value;

pub(crate) use detect::detect_pack;
pub(crate) use engines::unity;
use engines::{godot, tiled, tiled_json};
use packers::{cocos, dragonbones, libgdx, plist, starling, texture_packer};

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
            texture_packer(&v).or_else(|| tiled_json(&v)).or_else(|| dragonbones(&v))
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

/// A tile's animation: the tile, its class and its frames (tile, and milliseconds).
type TileAnim = (u32, Option<String>, Vec<(u32, u32)>);

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

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

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
}
