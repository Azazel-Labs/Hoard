//! Aseprite pixel art: its first frame with its frames, layers, timings and tags, and every frame on one sheet for a
//! browser to play.

use std::path::Path;

use anyhow::{Context, Result, ensure};
use image::{DynamicImage, RgbaImage};
use serde_json::{Map, Value, json};

use super::Decoded;

pub(super) fn aseprite(path: &Path) -> Result<Decoded> {
    let ase =
        asefile::AsepriteFile::read_file(path).map_err(|e| anyhow::anyhow!("not a readable Aseprite file: {e}"))?;
    ensure!(ase.num_frames() > 0, "Aseprite file has no frames");
    let frame = ase.frame(0).image();
    let (w, h) = (frame.width(), frame.height());
    let image = DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, frame.into_raw()).context("size")?);
    let mut facts = Map::new();
    let frames = ase.num_frames();
    facts.insert("frames".into(), json!(frames));
    facts.insert("layers".into(), json!(ase.num_layers()));
    if frames > 1 {
        // How long each frame shows, for playing it (see `aseprite_sheet`).
        let ms: Vec<u32> = (0..frames).map(|i| ase.frame(i).duration()).collect();
        facts.insert("duration_ms".into(), json!(ms.iter().map(|&m| u64::from(m)).sum::<u64>()));
        facts.insert("frame_ms".into(), json!(ms));
    }
    // Its tags are its animations: a run of frames each ("walk", "idle"), played forward, backward or there and back.
    let tags: Vec<Value> = (0..ase.num_tags())
        .map(|i| {
            let t = ase.tag(i);
            let direction = match t.animation_direction() {
                asefile::AnimationDirection::Forward => "forward",
                asefile::AnimationDirection::Reverse => "reverse",
                asefile::AnimationDirection::PingPong => "pingpong",
            };
            json!({ "name": t.name(), "from": t.from_frame(), "to": t.to_frame(), "direction": direction })
        })
        .collect();
    if !tags.is_empty() {
        facts.insert("animations".into(), json!(tags));
    }
    Ok(Decoded { image, facts })
}

/// Most frames a sheet holds, and most pixels across them.
const SHEET_MAX_FRAMES: usize = 1024;
const SHEET_MAX_PIXELS: u64 = 32 << 20;

/// Every frame of an Aseprite file side by side, in as many rows as columns (the last row may be short), as a PNG:
/// what a browser plays it from, with the frame times in the asset's `frame_ms`. None for a single frame, or more
/// than a sheet holds.
pub(crate) fn aseprite_sheet(path: &Path) -> Result<Option<Vec<u8>>> {
    use image::ImageEncoder as _;
    let ase =
        asefile::AsepriteFile::read_file(path).map_err(|e| anyhow::anyhow!("not a readable Aseprite file: {e}"))?;
    let n = ase.num_frames() as usize;
    let (w, h) = (ase.width() as u32, ase.height() as u32);
    if !(2..=SHEET_MAX_FRAMES).contains(&n) || n as u64 * u64::from(w) * u64::from(h) > SHEET_MAX_PIXELS {
        return Ok(None);
    }
    let cols = (n as f64).sqrt().ceil() as u32;
    let rows = (n as u32).div_ceil(cols);
    let mut sheet = RgbaImage::new(w * cols, h * rows);
    for i in 0..n as u32 {
        let f = ase.frame(i).image();
        let f = RgbaImage::from_raw(f.width(), f.height(), f.into_raw()).context("frame size")?;
        image::imageops::replace(&mut sheet, &f, i64::from(i % cols * w), i64::from(i / cols * h));
    }
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out).write_image(
        sheet.as_raw(),
        sheet.width(),
        sheet.height(),
        image::ExtendedColorType::Rgba8,
    )?;
    Ok(Some(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::textures::decode;
    use crate::textures::tests::write;

    /// A one-frame, one-layer 2x2 RGBA Aseprite file, laid out from the format spec.
    fn tiny_aseprite() -> Vec<u8> {
        aseprite_file(&[[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255]], &[])
    }

    /// A one-layer 2x2 RGBA Aseprite file of these frames, 100 ms each but the second (150 ms), and tags
    /// (name, from, to, direction).
    fn aseprite_file(frames: &[[u8; 16]], tags: &[(&str, u16, u16, u8)]) -> Vec<u8> {
        let mut layer = Vec::new();
        layer.extend_from_slice(&1u16.to_le_bytes()); // visible
        layer.extend_from_slice(&[0; 2 * 5]); // type, child level, default w/h, blend mode
        layer.push(255); // opacity
        layer.extend_from_slice(&[0; 3]);
        layer.extend_from_slice(&4u16.to_le_bytes());
        layer.extend_from_slice(b"Base");
        let mut cel = Vec::new();
        cel.extend_from_slice(&0u16.to_le_bytes()); // layer 0
        cel.extend_from_slice(&[0; 4]); // x, y
        cel.push(255);
        cel.extend_from_slice(&0u16.to_le_bytes()); // raw cel
        cel.extend_from_slice(&[0; 2 + 5]); // z-index, reserved
        cel.extend_from_slice(&2u16.to_le_bytes());
        cel.extend_from_slice(&2u16.to_le_bytes());
        let chunk = |kind: u16, data: &[u8]| {
            let mut c = ((data.len() + 6) as u32).to_le_bytes().to_vec();
            c.extend_from_slice(&kind.to_le_bytes());
            c.extend_from_slice(data);
            c
        };
        let mut tag_chunk = (tags.len() as u16).to_le_bytes().to_vec();
        tag_chunk.extend_from_slice(&[0; 8]);
        for (name, from, to, dir) in tags {
            tag_chunk.extend_from_slice(&from.to_le_bytes());
            tag_chunk.extend_from_slice(&to.to_le_bytes());
            tag_chunk.push(*dir);
            tag_chunk.extend_from_slice(&[0; 2 + 6 + 4]); // repeat, reserved, color
            tag_chunk.extend_from_slice(&(name.len() as u16).to_le_bytes());
            tag_chunk.extend_from_slice(name.as_bytes());
        }
        let mut body = Vec::new();
        for (i, pixels) in frames.iter().enumerate() {
            let mut chunks = Vec::new();
            if i == 0 {
                chunks.push(chunk(0x2004, &layer));
                if !tags.is_empty() {
                    chunks.push(chunk(0x2018, &tag_chunk));
                }
            }
            chunks.push(chunk(0x2005, &[&cel[..], pixels].concat()));
            let count = chunks.len();
            let chunks = chunks.concat();
            let mut frame = ((16 + chunks.len()) as u32).to_le_bytes().to_vec();
            frame.extend_from_slice(&0xF1FAu16.to_le_bytes());
            frame.extend_from_slice(&(count as u16).to_le_bytes());
            frame.extend_from_slice(&(if i == 1 { 150u16 } else { 100 }).to_le_bytes());
            frame.extend_from_slice(&[0; 2]);
            frame.extend_from_slice(&(count as u32).to_le_bytes());
            frame.extend(chunks);
            body.extend(frame);
        }
        let mut h = ((128 + body.len()) as u32).to_le_bytes().to_vec();
        h.extend_from_slice(&0xA5E0u16.to_le_bytes());
        for v in [frames.len() as u16, 2, 2, 32] {
            h.extend_from_slice(&v.to_le_bytes()); // frames, width, height, depth
        }
        h.extend_from_slice(&1u32.to_le_bytes()); // flags
        h.extend_from_slice(&100u16.to_le_bytes());
        h.extend_from_slice(&[0; 8]);
        h.push(0); // transparent index
        h.extend_from_slice(&[0; 3]);
        h.extend_from_slice(&0u16.to_le_bytes()); // colors
        h.extend_from_slice(&[1, 1]); // pixel ratio
        h.resize(128, 0);
        h.extend(body);
        h
    }

    #[test]
    fn aseprite_first_frame() {
        let dir = tempfile::tempdir().unwrap();
        let d = decode(&write(&dir, "s.aseprite", &tiny_aseprite()), "aseprite").unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.dimensions(), (2, 2));
        assert_eq!(img.get_pixel(1, 0).0, [0, 255, 0, 255]);
        assert_eq!(d.facts["frames"], 1);
        assert!(!d.facts.contains_key("frame_ms"), "a still has no timing");
    }

    #[test]
    fn aseprite_frames_play_from_a_sheet() {
        let [r, g, b] = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]].map(|c: [u8; 4]| {
            let mut f = [0; 16];
            f[..4].copy_from_slice(&c);
            f
        });
        let dir = tempfile::tempdir().unwrap();
        let path = write(&dir, "s.aseprite", &aseprite_file(&[r, g, b], &[("walk", 1, 2, 2)]));
        let d = decode(&path, "aseprite").unwrap();
        assert_eq!(d.facts["frames"], 3);
        assert_eq!(d.facts["frame_ms"], json!([100, 150, 100]));
        assert_eq!(d.facts["duration_ms"], 350);
        assert_eq!(d.facts["animations"], json!([{ "name": "walk", "from": 1, "to": 2, "direction": "pingpong" }]));

        // Three frames: two across, two down, in order.
        let sheet = image::load_from_memory(&aseprite_sheet(&path).unwrap().unwrap()).unwrap().to_rgba8();
        assert_eq!(sheet.dimensions(), (4, 4));
        assert_eq!(sheet.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(sheet.get_pixel(2, 0).0, [0, 255, 0, 255]);
        assert_eq!(sheet.get_pixel(0, 2).0, [0, 0, 255, 255]);
        assert_eq!(sheet.get_pixel(2, 2).0[3], 0, "the empty corner");

        let still = write(&dir, "still.aseprite", &tiny_aseprite());
        assert!(aseprite_sheet(&still).unwrap().is_none());
    }
}
