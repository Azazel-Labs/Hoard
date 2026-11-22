//! Tile sheets that come with a note of their grid rather than an atlas. Kenney's do: a `Tilesheet.txt` saying "Tile
//! size • 16px × 16px, Space between tiles • 1px × 1px, Total tiles (horizontal) • 12 tiles", or an `Instructions.txt`
//! saying "The tiles are 16 x 16px and have a 2px margin between them", beside a `Tilemap/` or `Tilesheet/` folder of
//! images on that grid: with the space between the tiles, without it (`tilemap_packed.png`), or twice the size
//! (`tilesheet@2.png`). The note becomes a sheet of those images' tiles (see [`crate::atlas`]).

use image::{Rgba, RgbaImage};

use crate::atlas::{Atlas, Page, Sprite, Turn};

/// Bigger text files aren't notes of a grid.
pub(crate) const MAX_NOTE: i64 = 4096;
/// Tiles smaller or bigger than this (in either direction) aren't read.
const TILE: std::ops::RangeInclusive<u32> = 4..=1024;

/// What a note says of its grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Note {
    pub(crate) tile: [u32; 2],
    /// The tile size of the sheet twice the size, when the note gives it.
    pub(crate) double: Option<[u32; 2]>,
    /// Between tiles, in pixels.
    pub(crate) spacing: u32,
    /// Columns and rows, when the note gives them.
    pub(crate) cells: Option<[u32; 2]>,
}

/// The numbers in a line: where each starts and ends, and its value.
fn numbers(s: &str) -> Vec<(usize, usize, u32)> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in s.char_indices().chain(std::iter::once((s.len(), ' '))) {
        match (c.is_ascii_digit(), start) {
            (true, None) => start = Some(i),
            (false, Some(b)) => {
                if let Ok(n) = s[b..i].parse() {
                    out.push((b, i, n));
                }
                start = None;
            }
            _ => {}
        }
    }
    out
}

/// The first `16 x 16`, `16px × 16px` or `64*64` in a line: the two numbers, and where the second ends.
fn pair(s: &str) -> Option<([u32; 2], usize)> {
    numbers(s).windows(2).find_map(|w| {
        let between = s[w[0].1..w[1].0].replace("px", "");
        matches!(between.trim(), "x" | "×" | "*").then_some(([w[0].2, w[1].2], w[1].1))
    })
}

/// Read a note of a tile sheet's grid. `None` when it isn't one: it must give a tile size.
pub(crate) fn parse_note(text: &str) -> Option<Note> {
    let text = text.trim_start_matches('\u{feff}').to_lowercase();
    let (mut tile, mut double, mut spacing, mut cols, mut rows) = (None, None, None, None, None);
    for line in text.lines() {
        let after = |word: &str| line.find(word).and_then(|k| numbers(&line[k..]).first().map(|n| n.2));
        if line.contains("horizontal") {
            cols = cols.or(after("horizontal"));
            continue;
        }
        if line.contains("vertical") {
            rows = rows.or(after("vertical"));
            continue;
        }
        let gap = ["margin", "spacing", "space", "gap", "padding"].iter().filter_map(|w| line.find(w)).min();
        let head = &line[..gap.unwrap_or(line.len())];
        let mut size_end = 0;
        if line.contains("tile")
            && let Some((size, end)) = pair(head)
        {
            size_end = end;
            if head.contains("double") {
                double = double.or(Some(size));
            } else {
                tile = tile.or(Some(size));
            }
        }
        let Some(k) = gap else { continue };
        if spacing.is_some() {
            continue;
        }
        spacing = if head.trim_end().ends_with("no") {
            Some(0)
        } else {
            // "Margin: 1", "Space between tiles • 1px", or "a 2px margin between them".
            numbers(&line[k..]).first().map(|n| n.2).or_else(|| numbers(&head[size_end..]).last().map(|n| n.2))
        };
    }
    let tile = tile.filter(|t| t.iter().all(|n| TILE.contains(n)))?;
    Some(Note {
        tile,
        double: double.filter(|t| t.iter().all(|n| TILE.contains(n))),
        spacing: spacing.unwrap_or(0).min(64),
        cells: cols.zip(rows).map(|(c, r)| [c, r]).filter(|c| c.iter().all(|&n| n > 0)),
    })
}

/// Where an image's tiles are: their size, the space between them, the space before the first (none, or a border as
/// wide as the space between) and how many columns and rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Grid {
    pub(crate) tile: [u32; 2],
    pub(crate) spacing: u32,
    pub(crate) margin: u32,
    pub(crate) cells: [u32; 2],
}

impl Grid {
    /// Each cell's rectangle, row by row.
    pub(crate) fn rects(&self) -> impl Iterator<Item = [u32; 4]> + '_ {
        let [tw, th] = self.tile;
        let (s, m) = (self.spacing, self.margin);
        (0..self.cells[1])
            .flat_map(move |r| (0..self.cells[0]).map(move |c| [m + c * (tw + s), m + r * (th + s), tw, th]))
    }
}

/// A name that says the image is twice the size (`tilesheet@2.png`, `mapPack_tilesheet_2X.png`, `iconsDouble.png`).
fn doubled(name: &str) -> bool {
    let n = name.to_lowercase();
    let n = n.rsplit_once('.').map_or(n.as_str(), |(s, _)| s);
    n.contains("@2") || n.ends_with("2x") || n.contains("double") || n.contains("retina")
}

/// How many tiles of `t` fit along `w` with `s` between them, and how far in the first is: exactly, with or without
/// space after the last, or with it all round (`RTS Medieval`'s sheet has a border as wide as the space between).
fn along(w: u32, t: u32, s: u32) -> Option<(u32, u32)> {
    let step = t + s;
    let (n, margin) = if (w + s).is_multiple_of(step) {
        ((w + s) / step, 0)
    } else if s > 0 && w.is_multiple_of(step) {
        (w / step, 0)
    } else if s > 0 && w > s && (w - s).is_multiple_of(step) {
        ((w - s) / step, s)
    } else {
        return None;
    };
    (n > 0).then_some((n, margin))
}

/// The grid a note describes on an image of this name and size, if the image is on it. With the space between tiles,
/// or packed without it; twice the size when the note gives that size and the counts say so, or the name does.
pub(crate) fn fit(note: &Note, name: &str, size: [u32; 2]) -> Option<Grid> {
    let twice = note.double.unwrap_or([note.tile[0] * 2, note.tile[1] * 2]);
    let tiles: Vec<[u32; 2]> = match (note.cells, doubled(name)) {
        (Some(_), _) => vec![note.tile, twice],
        (None, true) => vec![twice],
        (None, false) => vec![note.tile],
    };
    tiles.into_iter().find_map(|tile| {
        let scale = (tile[0] / note.tile[0].max(1)).max(1);
        let spacing = note.spacing * scale;
        let mut spacings = vec![spacing];
        if spacing > 0 {
            spacings.push(0);
        }
        spacings.into_iter().find_map(|s| {
            let ((columns, left), (rows, top)) = (along(size[0], tile[0], s)?, along(size[1], tile[1], s)?);
            let cells = [columns, rows];
            let right = note.cells.is_none_or(|c| c == cells) && cells[0] * cells[1] >= 2 && left == top;
            right.then_some(Grid { tile, spacing: s, margin: left, cells })
        })
    })
}

/// A cell with nothing in it: all transparent, or all the sheet's background (magenta, or the colour between tiles).
pub(crate) fn empty(img: &RgbaImage, [x0, y0, w, h]: [u32; 4], background: Option<Rgba<u8>>) -> bool {
    let first = *img.get_pixel(x0, y0);
    let mut flat = true;
    let mut clear = true;
    for yy in y0..y0 + h {
        for xx in x0..x0 + w {
            let px = *img.get_pixel(xx, yy);
            clear &= px[3] == 0;
            flat &= px == first;
            if !clear && !flat {
                return false;
            }
        }
    }
    clear || first == Rgba([255, 0, 255, 255]) || Some(first) == background
}

/// An image's tiles as an atlas page, named as Kenney names its loose tiles (`tile_0000`, counting every cell, the
/// empty ones too), each under the image's name when the sheet has more than one. Empty cells are left out.
pub(crate) fn page(img: &RgbaImage, grid: &Grid, image: &str, prefix: Option<&str>) -> Page {
    let (gap, m) = (grid.margin + grid.tile[0], grid.margin);
    let background = (grid.spacing > 0 && gap < img.width() && m < img.height()).then(|| *img.get_pixel(gap, m));
    let sprites = grid
        .rects()
        .enumerate()
        .filter(|&(_, r)| r[0] + r[2] <= img.width() && r[1] + r[3] <= img.height() && !empty(img, r, background))
        .map(|(i, rect)| Sprite {
            name: prefix.map_or_else(|| format!("tile_{i:04}"), |p| format!("{p}/tile_{i:04}")),
            rect,
            turn: Turn::Upright,
            trim: None,
        })
        .collect();
    Page { image: image.to_owned(), sprites }
}

/// The atlas of a note's images, given each with its grid; the tile size is the smallest of theirs, so no animations
/// are guessed from the tiles' names.
pub(crate) fn atlas(pages: Vec<Page>, tiles: &[[u32; 2]]) -> Atlas {
    let tile = tiles.iter().min_by_key(|t| t[0] * t[1]).copied();
    Atlas { format: "grid", pages, tile, ..Atlas::default() }
}

/// Words that say a folder or image is of tile sheets.
fn sheetish(s: &str) -> bool {
    let s = s.to_lowercase();
    ["sheet", "tilemap", "tileset"].iter().any(|w| s.contains(w))
}

/// The folder a note's kit is in: the note's, or the one above when the note is in the sheets' own folder
/// (`Board Game Icons/Tilesheet/Tilesheet.txt`, whose loose icons are in `Board Game Icons/PNG/`).
pub(crate) fn kit_dir(note_path: &str) -> &str {
    let dir = note_path.rsplit_once('/').map_or("", |(d, _)| d);
    let (up, last) = dir.rsplit_once('/').unwrap_or(("", dir));
    if sheetish(last) { up } else { dir }
}

/// Whether an image below a note's folder (`rel`, the path from there) may be one of its sheets. A note that counts
/// its tiles says which images are; one that doesn't is taken at its word only for images in a folder of sheets or
/// named as one, so a screenshot beside it whose size happens to be on the grid isn't cut up.
pub(crate) fn candidate(note: &Note, note_path: &str, rel: &str) -> bool {
    note.cells.is_some() || kit_dir(note_path) != note_path.rsplit_once('/').map_or("", |(d, _)| d) || sheetish(rel)
}

/// A small picture of an image to find copies of it quickly: the mean of each channel, with colour weighted by how
/// opaque it is, in each of 4 × 4 blocks.
pub(crate) fn signature(img: &RgbaImage) -> [u8; 64] {
    let (w, h) = img.dimensions();
    let mut out = [0u8; 64];
    for by in 0..4 {
        for bx in 0..4 {
            let (x0, x1, y0, y1) = (
                bx * w / 4,
                ((bx + 1) * w / 4).max(bx * w / 4 + 1),
                by * h / 4,
                ((by + 1) * h / 4).max(by * h / 4 + 1),
            );
            let mut sum = [0u64; 4];
            let mut count = 0u64;
            for y in y0..y1.min(h) {
                for x in x0..x1.min(w) {
                    let px = img.get_pixel(x, y);
                    let alpha = u64::from(px[3]);
                    for c in 0..3 {
                        sum[c] += u64::from(px[c]) * alpha / 255;
                    }
                    sum[3] += alpha;
                    count += 1;
                }
            }
            for c in 0..4 {
                out[((by * 4 + bx) * 4) as usize + c] = (sum[c] / count.max(1)) as u8;
            }
        }
    }
    out
}

/// Whether two signatures could be of the same picture, saved again.
pub(crate) fn alike(a: &[u8; 64], b: &[u8; 64]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(*y) <= 48)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notes_as_kenney_writes_them() {
        let table = "Tilesheet information:\n\nTile size                 •  16px × 16px\nSpace between tiles       •  1px × 1px\n---\nTotal tiles (horizontal)  •  12 tiles\nTotal tiles (vertical)    •  11 tiles\n---\nTotal tiles in sheet      •  132 tiles\n";
        assert_eq!(parse_note(table), Some(Note { tile: [16, 16], double: None, spacing: 1, cells: Some([12, 11]) }));
        let double = "Tile size                 •  64px × 64px\nTile size (double)        •  128px × 128px\nSpace between tiles       •  0px × 0px\n";
        assert_eq!(
            parse_note(double),
            Some(Note { tile: [64, 64], double: Some([128, 128]), spacing: 0, cells: None })
        );
        let sentence = "The tiles are 16 x 16px and have a 2px margin between them.";
        assert_eq!(parse_note(sentence), Some(Note { tile: [16, 16], double: None, spacing: 2, cells: None }));
        let both =
            "The tiles are 16 x 16px and have a 1px margin between them.\n\n---\n\nTILE SIZE: 16 x 16\nMARGIN:\t   1\n";
        assert_eq!(parse_note(both).map(|n| (n.tile, n.spacing)), Some(([16, 16], 1)));
        let tabbed = "\t\n\n\tTile size : 64 x 64 px\n\tMargin\t  : 0 px\n";
        assert_eq!(parse_note(tabbed).map(|n| (n.tile, n.spacing)), Some(([64, 64], 0)));
        let none = "\n\tEach tile is 64 x 64 in size, no margin between tiles.\n";
        assert_eq!(parse_note(none).map(|n| (n.tile, n.spacing)), Some(([64, 64], 0)));
        for other in [
            "This package has no spritesheet, the reason is that the source 3D objects are a mix of tiles.",
            "License: CC0\nVisit kenney.nl for 1920 x 1080 wallpapers",
            "",
        ] {
            assert_eq!(parse_note(other), None, "{other}");
        }
    }

    #[test]
    fn images_on_the_grid() {
        let town = Note { tile: [16, 16], double: None, spacing: 1, cells: Some([12, 11]) };
        assert_eq!(
            fit(&town, "tilemap.png", [203, 186]),
            Some(Grid { tile: [16, 16], spacing: 1, margin: 0, cells: [12, 11] })
        );
        assert_eq!(
            fit(&town, "tilemap_packed.png", [192, 176]),
            Some(Grid { tile: [16, 16], spacing: 0, margin: 0, cells: [12, 11] })
        );
        assert_eq!(fit(&town, "Preview.png", [918, 515]), None);
        assert_eq!(fit(&town, "tile_0000.png", [16, 16]), None);
        let ui = Note { tile: [16, 16], double: None, spacing: 2, cells: None };
        assert_eq!(fit(&ui, "UIpackSheet_transparent.png", [538, 592]).map(|g| g.cells), Some([30, 33]));
        let icons = Note { tile: [64, 64], double: Some([128, 128]), spacing: 0, cells: Some([21, 13]) };
        assert_eq!(fit(&icons, "iconsDefault.png", [1344, 832]).map(|g| g.tile), Some([64, 64]));
        assert_eq!(fit(&icons, "iconsDouble.png", [2688, 1664]).map(|g| g.tile), Some([128, 128]));
        let rts = Note { tile: [64, 64], double: None, spacing: 0, cells: None };
        assert_eq!(fit(&rts, "scifi_tilesheet.png", [1152, 448]).map(|g| g.cells), Some([18, 7]));
        assert_eq!(fit(&rts, "scifi_tilesheet@2.png", [2304, 896]).map(|g| g.cells), Some([18, 7]));
        // RTS Medieval's sheet has its 32 px space round the edge as well as between the tiles.
        let medieval = Note { tile: [64, 64], double: None, spacing: 32, cells: None };
        let bordered = Grid { tile: [64, 64], spacing: 32, margin: 32, cells: [18, 7] };
        assert_eq!(fit(&medieval, "medieval_tilesheet.png", [1760, 704]), Some(bordered));
        assert_eq!(
            fit(&medieval, "RTS_medieval@2.png", [3520, 1408]).map(|g| (g.margin, g.cells)),
            Some((64, [18, 7]))
        );
        assert_eq!(bordered.rects().nth(19), Some([128, 128, 64, 64]));
        let g = Grid { tile: [16, 16], spacing: 1, margin: 0, cells: [2, 2] };
        assert_eq!(g.rects().collect::<Vec<_>>(), [[0, 0, 16, 16], [17, 0, 16, 16], [0, 17, 16, 16], [17, 17, 16, 16]]);
    }

    #[test]
    fn empty_cells_are_left_out_and_the_rest_named_by_cell() {
        // Two by two of 4 px tiles, 1 px apart on a grey background: the second cell is empty, the fourth transparent.
        let mut img = RgbaImage::from_pixel(9, 9, Rgba([90, 90, 90, 255]));
        for (x, y) in [(0, 0), (0, 5)] {
            for yy in y..y + 4 {
                for xx in x..x + 4 {
                    img.put_pixel(xx, yy, Rgba([200, (xx * 30) as u8, 0, 255]));
                }
            }
        }
        for yy in 5..9 {
            for xx in 5..9 {
                img.put_pixel(xx, yy, Rgba([0, 0, 0, 0]));
            }
        }
        let grid = Grid { tile: [4, 4], spacing: 1, margin: 0, cells: [2, 2] };
        let names: Vec<String> = page(&img, &grid, "t.png", None).sprites.into_iter().map(|s| s.name).collect();
        assert_eq!(names, ["tile_0000", "tile_0002"]);
        let names: Vec<String> = page(&img, &grid, "t.png", Some("t")).sprites.into_iter().map(|s| s.name).collect();
        assert_eq!(names, ["t/tile_0000", "t/tile_0002"]);
        assert_eq!(kit_dir("Board Game Icons/Tilesheet/Tilesheet.txt"), "Board Game Icons");
        assert_eq!(kit_dir("Tiny Town/Tilesheet.txt"), "Tiny Town");
        assert_eq!(kit_dir("Tilesheet.txt"), "");
        let uncounted = Note { tile: [16, 16], double: None, spacing: 2, cells: None };
        assert!(candidate(&uncounted, "UI Pixel/Instructions.txt", "Spritesheet/UIpackSheet_magenta.png"));
        assert!(!candidate(&uncounted, "UI Pixel/Instructions.txt", "Sample.png"));
        assert!(candidate(&uncounted, "Pirate/Tilesheet/Tilesheet.txt", "tiles_sheet.png"));
        assert!(candidate(&uncounted, "Dungeon/Spritesheet/Instructions.txt", "roguelikeDungeon_transparent.png"));
    }
}
