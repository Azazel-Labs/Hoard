//! The files texture packers write: Starling and Sparrow XML, DragonBones, TexturePacker JSON, libGDX and Spine
//! `.atlas` files and Cocos2d `.plist` files.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

use hoard_core::AssetKind;
use quick_xml::events::Event;
use serde_json::{Value, json};

use super::{Anim, Atlas, FRAME_MS, Frame, Page, Sprite, Turn, attrs, natural, num, px, trim_of};

/// Starling and Sparrow:`<TextureAtlas imagePath="…"><SubTexture name x y width height [frameX frameY frameWidth
/// frameHeight] [rotated]/>`. The rectangle is the region as stored; `frameX` and `frameY` are where the canvas starts
/// relative to the sprite, so negative.
pub(super) fn starling(text: &str) -> Option<Atlas> {
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

/// A DragonBones texture atlas (`…_tex.json`): `{"imagePath": …, "SubTexture": [{name, x, y, width, height, [frameX,
/// frameY, frameWidth, frameHeight], [rotated]}]}`, Starling's fields as JSON. Its width and height are the sprite's
/// upright, so a turned sprite's region in the page has them the other way round.
pub(super) fn dragonbones(v: &Value) -> Option<Atlas> {
    let image = v.get("imagePath")?.as_str()?;
    let mut sprites = Vec::new();
    for t in v.get("SubTexture")?.as_array()? {
        let get = |k: &str| field(t, k);
        let (Some(x), Some(y), Some(w), Some(h)) =
            (get("x").and_then(px), get("y").and_then(px), get("width").and_then(px), get("height").and_then(px))
        else {
            continue;
        };
        let rotated = t.get("rotated").and_then(Value::as_bool).unwrap_or(false);
        let trim = match (get("frameWidth").and_then(px), get("frameHeight").and_then(px)) {
            (Some(fw), Some(fh)) => trim_of(
                [fw, fh],
                [-get("frameX").unwrap_or(0.0).round() as i64, -get("frameY").unwrap_or(0.0).round() as i64],
                [w, h],
            ),
            _ => None,
        };
        sprites.push(Sprite {
            name: t.get("name").and_then(Value::as_str).unwrap_or_default().to_owned(),
            rect: if rotated { [x, y, h, w] } else { [x, y, w, h] },
            turn: if rotated { Turn::Clockwise } else { Turn::Upright },
            trim,
        });
    }
    Some(Atlas { format: "dragonbones", pages: vec![Page { image: image.to_owned(), sprites }], ..Atlas::default() })
}

pub(super) fn field(v: &Value, k: &str) -> Option<f64> {
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
pub(super) fn texture_packer(v: &Value) -> Option<Atlas> {
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

/// libGDX's and Spine's `.atlas`: a page's image file on a line of its own (first, or after a blank line), then its
/// settings and regions; a region is a name on a line of its own, then `key: value` lines. The old format gives `xy`,
/// `size` (upright), `orig` and `offset` (from the bottom left); the new one `bounds` and `offsets`. A turned region
/// is stored counter-clockwise, `size` wide by `size` high the other way round.
pub(super) fn libgdx(text: &str) -> Option<Atlas> {
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
pub(super) fn plist(text: &str) -> Option<Value> {
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
pub(super) fn cocos(v: &Value) -> Option<Atlas> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::{Atlas, Page, Trim, parse};

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
    fn dragonbones_texture_atlas() {
        let text = r#"{"name":"Dragon","imagePath":"Dragon_tex.png","width":256,"height":256,"SubTexture":[
            {"name":"parts/tailTip","x":1,"y":1,"width":108,"height":131,"frameX":-2,"frameY":-1,"frameWidth":112,"frameHeight":134},
            {"name":"parts/wing","x":120,"y":1,"width":40,"height":20,"rotated":true}]}"#;
        let atlas = parse("json", text).unwrap();
        assert_eq!(atlas.format, "dragonbones");
        assert_eq!(atlas.pages[0].image, "Dragon_tex.png");
        let [tail, wing] = &atlas.pages[0].sprites[..] else { panic!() };
        assert_eq!((tail.name.as_str(), tail.rect), ("parts/tailTip", [1, 1, 108, 131]));
        assert_eq!(tail.trim, Some(Trim { size: [112, 134], at: [2, 1] }));
        assert_eq!((wing.rect, wing.turn), ([120, 1, 20, 40], Turn::Clockwise));
        // The skeleton beside it isn't an atlas.
        assert!(parse("json", r#"{"frameRate":24,"name":"Dragon","version":"5.5","armature":[]}"#).is_none());
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
}
