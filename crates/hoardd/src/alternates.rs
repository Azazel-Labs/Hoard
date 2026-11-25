//! One asset in several formats: a SketchUp model exported as `.obj` beside it, a sound as `.wav` and `.ogg`, or
//! Kenney's kits with `Models/FBX format/car.fbx`, `Models/OBJ format/car.obj` and `Models/GLB format/car.glb`. The
//! format Hoard reads best, or that loses least, is the asset; the others are linked from it (`format`), hidden with
//! it and downloadable from its "Download as".

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use rusqlite::{Connection, params};

/// Model formats, best first: the ones Hoard draws itself, the richer before the plainer, then the rest.
const MODEL: &[&str] = &[
    "glb", "gltf", "vrm", "fbx", "obj", "dae", "zae", "blend", "skp", "3mf", "usdz", "usd", "usdc", "usda", "x3d",
    "x3dz", "x3dv", "3ds", "wrl", "wrz", "vox", "stl", "ply", "off", "splat", "spz", "abc",
];
/// Sound formats, lossless before lossy, notes last.
const AUDIO: &[&str] =
    &["flac", "wav", "wave", "aiff", "aif", "ogg", "oga", "opus", "m4a", "aac", "mp3", "wma", "mid", "midi"];
const FONT: &[&str] = &["otf", "ttf", "woff2", "woff", "eot"];

fn order(kind: &str) -> Option<&'static [&'static str]> {
    match kind {
        "model" => Some(MODEL),
        "audio" => Some(AUDIO),
        "font" => Some(FONT),
        _ => None,
    }
}

/// Words a folder named after a format may have around the format's name.
const FILLER: &[&str] = &["format", "formats", "file", "files", "version", "versions", "export", "exports"];
/// Formats' names that aren't their extensions.
const NAMES: &[&str] = &["collada", "wavefront", "vorbis", "gltf2", "sketchup", "truetype", "opentype"];

/// A folder named after a format (`FBX format`, `OGG`, `glTF files`), where a pack keeps one format of each asset.
fn format_folder(name: &str) -> bool {
    let words: Vec<String> =
        name.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect();
    let mut named = words.iter().filter(|w| !FILLER.contains(&w.as_str())).peekable();
    named.peek().is_some()
        && named.all(|w| {
            let w = w.as_str();
            NAMES.contains(&w) || MODEL.contains(&w) || AUDIO.contains(&w) || FONT.contains(&w)
        })
}

/// What one asset's formats share: its kind, its folder with folders named after a format as one, and its name.
fn key(kind: &str, path: &str) -> String {
    let (dir, file) = path.rsplit_once('/').unwrap_or(("", path));
    let stem = file.rsplit_once('.').map_or(file, |(s, _)| s);
    let dir: Vec<&str> = dir.split('/').map(|d| if format_folder(d) { "\u{1}" } else { d }).collect();
    format!("{kind}\u{0}{}\u{0}{stem}", dir.join("/")).to_lowercase()
}

/// Link each asset of the pack in several formats from the one shown, and unlink any no longer so. Returns how many
/// assets have other formats.
pub(crate) fn detect_pack(conn: &mut Connection, pack_id: &str) -> Result<usize> {
    struct File {
        id: String,
        path: String,
        ext: String,
        drawn: bool,
    }
    let mut groups: BTreeMap<String, Vec<File>> = BTreeMap::new();
    {
        let mut stmt = conn.prepare(
            "SELECT id, path, lower(ext), kind, thumb_hash IS NOT NULL FROM assets
              WHERE pack_id = ?1 AND archived_at IS NULL AND kind IN ('model', 'audio', 'font')",
        )?;
        let rows = stmt.query_map([pack_id], |r| {
            Ok((File { id: r.get(0)?, path: r.get(1)?, ext: r.get(2)?, drawn: r.get(4)? }, r.get::<_, String>(3)?))
        })?;
        for row in rows {
            let (f, kind) = row?;
            groups.entry(key(&kind, &f.path)).or_default().push(f);
        }
    }
    let mut links = Vec::new();
    for (k, mut files) in groups {
        if files.iter().map(|f| f.ext.as_str()).collect::<BTreeSet<_>>().len() < 2 {
            continue;
        }
        let rank = order(k.split('\u{0}').next().unwrap_or_default()).unwrap_or_default();
        // One Hoard couldn't read loses to one it could, whatever its format.
        files.sort_by_cached_key(|f| {
            (!f.drawn, rank.iter().position(|e| *e == f.ext).unwrap_or(rank.len()), f.path.clone())
        });
        let (first, rest) = files.split_first().expect("two or more");
        links.extend(rest.iter().map(|f| (first.id.clone(), f.id.clone())));
    }
    let shown = links.iter().map(|(from, _)| from).collect::<BTreeSet<_>>().len();
    let tx = conn.transaction()?;
    tx.execute(
        "DELETE FROM asset_links WHERE rel = 'format' AND from_id IN (SELECT id FROM assets WHERE pack_id = ?1)",
        [pack_id],
    )?;
    for (from, to) in &links {
        tx.prepare_cached(
            "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'format') ON CONFLICT DO NOTHING",
        )?
        .execute(params![from, to])?;
    }
    tx.commit()?;
    Ok(shown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folders_named_after_a_format() {
        for name in ["FBX format", "OBJ", "glTF files", "GLB format", "OGG Vorbis", "Collada"] {
            assert!(format_folder(name), "{name}");
        }
        for name in ["Models", "Audio", "Format", "Kenney Car Kit", "Vector", "PNG"] {
            assert!(!format_folder(name), "{name}");
        }
        assert_eq!(key("model", "Kit/Models/FBX format/Car.fbx"), key("model", "Kit/Models/OBJ format/car.obj"));
        assert_ne!(key("model", "Kit/Models/FBX format/car.fbx"), key("model", "Kit/Props/OBJ format/car.obj"));
        assert_ne!(key("model", "a/car.obj"), key("audio", "a/car.ogg"));
    }

    #[test]
    fn the_best_format_is_shown_and_the_others_linked_from_it() {
        let mut conn = crate::db::test_library();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, thumb_hash, created_at, updated_at) VALUES
               ('skp', 'p', 'h', 'Guides/house.skp', 'House', 'skp', 'model', 't', 'x', 'x'),
               ('obj', 'p', 'h', 'Guides/house.obj', 'House', 'obj', 'model', 't', 'x', 'x'),
               ('wav', 'p', 'h', 'Audio/WAV/click.wav', 'Click', 'wav', 'audio', 't', 'x', 'x'),
               ('ogg', 'p', 'h', 'Audio/OGG/click.ogg', 'Click', 'ogg', 'audio', 't', 'x', 'x'),
               ('glb', 'p', 'h', 'Models/GLB format/car.glb', 'Car', 'glb', 'model', NULL, 'x', 'x'),
               ('fbx', 'p', 'h', 'Models/FBX format/car.fbx', 'Car', 'fbx', 'model', 't', 'x', 'x'),
               ('one', 'p', 'h', 'Models/OBJ format/tree.obj', 'Tree', 'obj', 'model', 't', 'x', 'x'),
               ('tex', 'p', 'h', 'Guides/house.png', 'House', 'png', 'texture', 't', 'x', 'x');",
        )
        .unwrap();
        assert_eq!(detect_pack(&mut conn, "p").unwrap(), 3);
        let links: Vec<(String, String)> = conn
            .prepare("SELECT from_id, to_id FROM asset_links WHERE rel = 'format' ORDER BY from_id")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        let pair = |a: &str, b: &str| (a.to_owned(), b.to_owned());
        // The GLB has no thumbnail: Hoard couldn't read it, so the FBX is shown.
        assert_eq!(links, [pair("fbx", "glb"), pair("obj", "skp"), pair("wav", "ogg")]);

        conn.execute("UPDATE assets SET archived_at = 'x' WHERE id = 'ogg'", []).unwrap();
        assert_eq!(detect_pack(&mut conn, "p").unwrap(), 2);
    }
}
