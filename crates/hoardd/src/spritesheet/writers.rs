//! Writing a sheet in other tools' formats: TexturePacker's JSON, Starling, libGDX, Cocos2d, Godot's
//! `SpriteFrames` and `TileSet`, and plain grids and frames.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as _;

use anyhow::Result;
use image::{RgbaImage, imageops};
use serde_json::{Value, json};

use super::{Anim, AnimFrame, Files, MAX_GRID, Sheet, Sprite, clean_name, image_stem, whole};
use crate::atlas::{self};

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
pub(super) fn texture_packer(sheet: &Sheet, stem: &str, array: bool) -> Files {
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
pub(super) fn starling(sheet: &Sheet, stem: &str) -> Files {
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
pub(super) fn libgdx(sheet: &Sheet) -> String {
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
pub(super) fn cocos(sheet: &Sheet, stem: &str) -> Files {
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
pub(super) fn godot_frames(sheet: &Sheet) -> String {
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
pub(super) fn grid_files(sheet: &Sheet, images: &[RgbaImage], stem: &str) -> Result<Files> {
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
pub(super) fn frame_files(sheet: &Sheet, images: &[RgbaImage], stem: &str) -> Result<Files> {
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
pub(super) fn godot_tileset(
    sheet: &Sheet,
    images: &[RgbaImage],
    originals: &[Option<Vec<u8>>],
    stem: &str,
) -> Result<Files> {
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
