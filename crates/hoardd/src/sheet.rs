//! Contact sheets: many thumbnails in one numbered grid, so a vision LLM sees a whole batch in one image (fewer
//! image tokens, one request) and can refer to tiles by their file number.

use image::{Rgba, RgbaImage, imageops};

/// 3x5 bitmap digits, one row per `u8` (low 3 bits, MSB on the left).
const DIGITS: [[u8; 5]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111],
    [0b010, 0b110, 0b010, 0b010, 0b111],
    [0b111, 0b001, 0b111, 0b100, 0b111],
    [0b111, 0b001, 0b111, 0b001, 0b111],
    [0b101, 0b101, 0b111, 0b001, 0b001],
    [0b111, 0b100, 0b111, 0b001, 0b111],
    [0b111, 0b100, 0b111, 0b101, 0b111],
    [0b111, 0b001, 0b010, 0b010, 0b010],
    [0b111, 0b101, 0b111, 0b101, 0b111],
    [0b111, 0b101, 0b111, 0b001, 0b111],
];
const SCALE: u32 = 3;
const PAD: u32 = 4;

/// Lay `tiles` (label, image) out in a grid of `cell`-sized squares, `cols` wide, each labelled in its corner.
pub(crate) fn contact_sheet(tiles: &[(usize, RgbaImage)], cell: u32, cols: u32) -> RgbaImage {
    let cols = cols.max(1);
    let rows = (tiles.len() as u32).div_ceil(cols).max(1);
    let gap = 4;
    let mut sheet =
        RgbaImage::from_pixel(cols * (cell + gap) + gap, rows * (cell + gap) + gap, Rgba([48, 48, 52, 255]));
    for (n, (label, img)) in tiles.iter().enumerate() {
        let (cx, cy) = (gap + (n as u32 % cols) * (cell + gap), gap + (n as u32 / cols) * (cell + gap));
        // Checkerboard behind transparent sprites, so alpha reads as alpha.
        for y in 0..cell {
            for x in 0..cell {
                let light = ((x / 16) + (y / 16)).is_multiple_of(2);
                sheet.put_pixel(
                    cx + x,
                    cy + y,
                    if light { Rgba([200, 200, 200, 255]) } else { Rgba([160, 160, 160, 255]) },
                );
            }
        }
        let fitted = fit(img, cell);
        let (ox, oy) = ((cell - fitted.width()) / 2, (cell - fitted.height()) / 2);
        imageops::overlay(&mut sheet, &fitted, i64::from(cx + ox), i64::from(cy + oy));
        draw_label(&mut sheet, cx, cy, *label);
    }
    sheet
}

fn fit(img: &RgbaImage, cell: u32) -> RgbaImage {
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 {
        return RgbaImage::new(1, 1);
    }
    let s = f64::from(cell) / f64::from(w.max(h));
    let (nw, nh) = (((f64::from(w) * s) as u32).max(1), ((f64::from(h) * s) as u32).max(1));
    imageops::resize(img, nw, nh, imageops::FilterType::Triangle)
}

fn draw_label(sheet: &mut RgbaImage, x0: u32, y0: u32, n: usize) {
    let text = n.to_string();
    let w = text.len() as u32 * 4 * SCALE - SCALE + PAD * 2;
    let h = 5 * SCALE + PAD * 2;
    for y in 0..h {
        for x in 0..w {
            sheet.put_pixel(x0 + x, y0 + y, Rgba([0, 0, 0, 255]));
        }
    }
    for (i, ch) in text.bytes().enumerate() {
        let glyph = DIGITS[usize::from(ch - b'0')];
        let gx = x0 + PAD + i as u32 * 4 * SCALE;
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..3 {
                if bits & (0b100 >> col) != 0 {
                    for dy in 0..SCALE {
                        for dx in 0..SCALE {
                            let (px, py) = (gx + col * SCALE + dx, y0 + PAD + row as u32 * SCALE + dy);
                            sheet.put_pixel(px, py, Rgba([255, 255, 255, 255]));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lays_out_grid() {
        let red = RgbaImage::from_pixel(64, 32, Rgba([255, 0, 0, 255]));
        let tiles: Vec<_> = (0..5).map(|i| (i * 7, red.clone())).collect();
        let s = contact_sheet(&tiles, 96, 4);
        assert_eq!(s.dimensions(), (4 * 100 + 4, 2 * 100 + 4));
        // Middle of the first tile is the (letterboxed) red image.
        assert_eq!(s.get_pixel(4 + 48, 4 + 48), &Rgba([255, 0, 0, 255]));
        // Label box in the corner, with a white pixel of the "0" glyph.
        assert_eq!(s.get_pixel(4 + PAD, 4 + PAD), &Rgba([255, 255, 255, 255]));
    }
}
