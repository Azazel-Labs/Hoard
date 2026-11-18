//! The sprites a game engine or map editor slices an image into: Tiled's tilesets, Unity's sprite import
//! settings and Godot's atlas textures, sprite frames and tile sets.
#![expect(clippy::many_single_char_names, reason = "rectangles: x/y/w/h")]

use std::collections::HashMap;

use quick_xml::events::Event;
use serde_json::Value;

use super::{Anim, Atlas, FRAME_MS, Frame, MAX_SPRITES, Page, Sprite, TileAnim, Turn, attrs, num, px, trim_of};

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
pub(super) fn tiled(text: &str) -> Option<Atlas> {
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
pub(super) fn tiled_json(v: &Value) -> Option<Atlas> {
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

/// A Unity image's `.meta` when the image is cut into sprites (`spriteMode: 2`): each sprite's name and rectangle in
/// `spriteSheet.sprites`. `image` is the image the `.meta` sits beside. Unity measures from the bottom of the image,
/// so the rectangles are turned over once the image's height is known ([`flip_unity`]).
pub(crate) fn unity(text: &str, image: &str) -> Option<Atlas> {
    let text = text.trim_start_matches('\u{feff}');
    if !text.lines().any(|l| l.trim() == "spriteMode: 2") {
        return None;
    }
    let indent = |l: &str| l.len() - l.trim_start().len();
    let mut lines = text.lines().skip_while(|l| l.trim() != "spriteSheet:").skip(1);
    let list = lines.find(|l| l.trim_start().starts_with("sprites:"))?;
    let at = indent(list);
    let mut sprites = Vec::new();
    // The sprite being read: its name, and x, y, width and height.
    let mut cur: Option<(Option<String>, [Option<f64>; 4])> = None;
    let mut in_rect = false;
    let done = |cur: Option<(Option<String>, [Option<f64>; 4])>, sprites: &mut Vec<Sprite>| {
        let Some((Some(name), [Some(x), Some(y), Some(w), Some(h)])) = cur else { return };
        if let (Some(x), Some(y), Some(w), Some(h)) = (px(x), px(y), px(w), px(h))
            && w > 0
            && h > 0
        {
            sprites.push(Sprite { name, rect: [x, y, w, h], turn: Turn::Upright, trim: None });
        }
    };
    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let i = indent(line);
        let field = if i == at && t.starts_with("- ") {
            done(cur.take(), &mut sprites);
            cur = Some((None, [None; 4]));
            in_rect = false;
            &t[2..]
        } else if i <= at {
            break;
        } else if i == at + 2 {
            in_rect = false;
            t
        } else {
            if in_rect && let (Some((_, rect)), Some((k, v))) = (cur.as_mut(), t.split_once(':')) {
                let slot = match k {
                    "x" => 0,
                    "y" => 1,
                    "width" => 2,
                    "height" => 3,
                    _ => continue,
                };
                rect[slot] = num(v.trim());
            }
            continue;
        };
        let Some((name, _)) = cur.as_mut() else { continue };
        if let Some(v) = field.strip_prefix("name:") {
            let v = v.trim().trim_matches(|c| c == '\'' || c == '"');
            *name = (!v.is_empty()).then(|| v.to_owned());
        } else if field.trim_end() == "rect:" {
            in_rect = true;
        }
    }
    done(cur, &mut sprites);
    (sprites.len() >= 2).then(|| Atlas {
        format: "unity",
        pages: vec![Page { image: image.to_owned(), sprites }],
        ..Atlas::default()
    })
}

/// Turn a Unity sheet's rectangles over, from measured from the bottom to from the top, by its pages' heights.
pub(super) fn flip_unity(atlas: &mut Atlas, heights: &[Option<u32>]) {
    for (p, h) in atlas.pages.iter_mut().zip(heights) {
        let Some(h) = *h else { continue };
        p.sprites.retain_mut(|s| {
            let Some(y) = h.checked_sub(s.rect[1] + s.rect[3]) else { return false };
            s.rect[1] = y;
            true
        });
    }
}

/// Godot's `SpriteFrames` and `TileSet` resources (`.tres`, Godot 3 and 4). A page is an image the resource names,
/// as it names it (`res://…`).
pub(super) fn godot(text: &str) -> Option<Atlas> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::detect::drop_empty;
    use crate::atlas::parse;
    use image::{Rgba, RgbaImage};

    #[test]
    fn unity_sprite_slicing() {
        let meta = "fileFormatVersion: 2
guid: 7e1a2e907c7a4339be186c6b102fde95
TextureImporter:
  spriteMode: 2
  spriteSheet:
    serializedVersion: 2
    sprites:
    - serializedVersion: 2
      name: hero_0
      rect:
        serializedVersion: 2
        x: 0
        y: 16
        width: 16
        height: 16
      alignment: 0
      pivot: {x: 0.5, y: 0.5}
      outline:
      - - {x: 1, y: 2}
      bones:
      - name: root
        length: 3
    - serializedVersion: 2
      name: 'hero 1'
      rect:
        serializedVersion: 2
        x: 16
        y: 0
        width: 16
        height: 8
    outline: []
    nameFileIdTable:
      hero_0: 21300000
  spritePackingTag:
";
        let mut atlas = unity(meta, "hero.png").unwrap();
        assert_eq!(atlas.format, "unity");
        assert_eq!(atlas.pages[0].image, "hero.png");
        let names: Vec<&str> = atlas.pages[0].sprites.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["hero_0", "hero 1"]);
        flip_unity(&mut atlas, &[Some(32)]);
        let rects: Vec<[u32; 4]> = atlas.pages[0].sprites.iter().map(|s| s.rect).collect();
        assert_eq!(rects, [[0, 0, 16, 16], [16, 24, 16, 8]]);
        // One sprite, or an image not cut into sprites, isn't a sheet.
        assert!(unity(&meta.replace("spriteMode: 2", "spriteMode: 1"), "hero.png").is_none());
        assert!(
            unity(
                "fileFormatVersion: 2\nTextureImporter:\n  spriteMode: 2\n  spriteSheet:\n    sprites: []\n",
                "a.png"
            )
            .is_none()
        );
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
}
