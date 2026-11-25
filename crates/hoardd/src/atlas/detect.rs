//! The library side: finding a pack's atlases, linking them to their pages and to the loose images that are their
//! sprites saved again, and drawing the sheet's thumbnail.

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use hoard_core::{AssetKind, TagSource};
use image::{RgbaImage, imageops};
use rusqlite::{Connection, params};
use serde_json::{Value, json};
use tokio::runtime::Handle;

use super::engines::{flip_unity, unity};
use super::{Atlas, MAX_FILE, MAX_SIDE, MAX_SPRITES, Sprite, cut, dir_of, find_page, parse};
use crate::{AppState, db, tagger};

/// The files atlases are read from, as SQL.
const EXTS: &str = "'xml', 'json', 'atlas', 'plist', 'tres', 'tsx', 'tsj', 'meta'";

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
pub(super) fn drop_empty(atlas: &mut Atlas, imgs: &[Option<RgbaImage>]) {
    let mut moved: Vec<Vec<Option<usize>>> = Vec::new();
    for (p, img) in atlas.pages.iter_mut().zip(imgs) {
        let Some(img) = img else {
            moved.push((0..p.sprites.len()).map(Some).collect());
            continue;
        };
        // The color between tiles, when there's space between them.
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
        let parsed = match c.path.strip_suffix(".meta").filter(|_| c.ext == "meta") {
            Some(image) => unity(&text, image.rsplit('/').next().unwrap_or(image)),
            None => parse(&c.ext, &text),
        };
        let Some(mut atlas) = parsed else { continue };
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
        if atlas.format == "unity" {
            let heights: Vec<Option<u32>> = pages.iter().map(|p| p.as_ref().map(|p| p.size[1])).collect();
            flip_unity(&mut atlas, &heights);
        }
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
/// each channel within [`CHANNEL_SLACK`] (color only where both are mostly opaque, as saving again changes what
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
    crate::analyze::thumb_bytes(&img, exact).ok()
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
    tx.prepare_cached(
        "UPDATE assets SET meta = json_remove(meta, '$.atlas')
          WHERE pack_id = ?1 AND json_extract(meta, '$.atlas') IS NOT NULL",
    )?
    .execute([pack_id])?;
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
    // Cached, as these run for each sheet and its pages.
    for f in found {
        kept.insert(f.file_id.clone());
        for &p in &f.pages {
            tx.prepare_cached(
                "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'texture') ON CONFLICT DO NOTHING",
            )?
            .execute(params![f.file_id, textures[p].id])?;
            tx.prepare_cached("UPDATE assets SET meta = json_set(meta, '$.atlas', json(?2)) WHERE id = ?1")?.execute(
                params![
                    textures[p].id,
                    json!({ "file": f.file_path, "format": f.format, "sprites": f.sprites }).to_string()
                ],
            )?;
        }
        if let Some(loose) = &f.loose {
            tx.prepare_cached("DELETE FROM asset_links WHERE from_id = ?1 AND rel = 'sprite'")?
                .execute([&f.file_id])?;
            for &t in loose {
                tx.prepare_cached(
                    "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'sprite') ON CONFLICT DO NOTHING",
                )?
                .execute(params![f.file_id, textures[t].id])?;
            }
            tx.prepare_cached("UPDATE assets SET meta = json_set(meta, '$.loose', ?2) WHERE id = ?1")?
                .execute(params![f.file_id, loose.len() as i64])?;
        }
        let Some(new) = &f.new else { continue };
        for b in [Some(&new.blob), new.thumb.as_ref()].into_iter().flatten() {
            tx.prepare_cached(
                "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
            )?
            .execute(params![b.hash, b.size as i64, now])?;
        }
        tx.prepare_cached(
            "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, 'sheet', ?2, ?3)
             ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash, created_at = excluded.created_at",
        )?
        .execute(params![f.file_id, new.blob.hash, now])?;
        let category = tagger::categorize(AssetKind::Sheet, &tags_of(&f.file_id)?);
        tx.prepare_cached(
            "UPDATE assets SET kind = 'sheet', meta = json_set(meta, '$.sheet', json(?2)), words = ?3,
                    thumb_hash = COALESCE(?4, thumb_hash), updated_at = ?5,
                    category = CASE WHEN human_edited THEN category ELSE ?6 END
              WHERE id = ?1",
        )?
        .execute(params![
            f.file_id,
            new.summary.to_string(),
            format!("{} sprite sheet {}", tagger::fts_words(&f.file_path), new.words),
            new.thumb.as_ref().map(|t| t.hash.clone()),
            now,
            category,
        ])?;
        db::reindex_asset(&tx, &f.file_id)?;
    }
    for (id, path, ext) in before {
        if kept.contains(&id) {
            continue;
        }
        let kind = AssetKind::from_ext(&ext.to_lowercase());
        let category = tagger::categorize(kind, &tags_of(&id)?);
        tx.prepare_cached("DELETE FROM derivatives WHERE asset_id = ?1 AND kind = 'sheet'")?.execute([&id])?;
        tx.prepare_cached("DELETE FROM asset_links WHERE from_id = ?1 AND rel = 'sprite'")?.execute([&id])?;
        tx.prepare_cached(
            "UPDATE assets SET kind = ?2, meta = json_remove(meta, '$.sheet', '$.loose'), words = ?3, thumb_hash = NULL,
                    updated_at = ?4, category = CASE WHEN human_edited THEN category ELSE ?5 END
              WHERE id = ?1",
        )?
        .execute(params![id, kind.as_str(), tagger::fts_words(&path), now, category])?;
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
