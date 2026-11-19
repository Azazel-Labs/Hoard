//! Finding the references inside one file, as it's analyzed: what it names, before the pack is there to resolve them.

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::Result;
use hoard_core::LinkRel;
use serde_json::Value;

use super::IMAGE_EXTS;

/// A raw reference found inside a file, before resolving it to an asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ref {
    pub(crate) target: String,
    pub(crate) rel: LinkRel,
}

pub(crate) fn extract_refs(local: &Path, ext: &str) -> Vec<Ref> {
    let result = match ext {
        "gltf" => File::open(local)
            .ok()
            .and_then(|f| serde_json::from_reader::<_, Value>(BufReader::new(f)).ok())
            .map(|doc| gltf_refs(&doc)),
        "glb" => glb_json(local).map(|doc| gltf_refs(&doc)),
        "obj" => Some(obj_refs(local)),
        "mtl" => Some(mtl_refs(local)),
        "fbx" => Some(fbx_refs(local)),
        "dae" => Some(dae_refs(local)),
        "blend" => crate::sources::read_blend(local, true).ok().map(|info| {
            // Blender's `//` prefix means "relative to this .blend"; everything else is absolute.
            let strip = |p: String| p.strip_prefix("//").map(str::to_owned).unwrap_or(p);
            let mut refs: Vec<Ref> =
                info.image_paths.into_iter().map(|t| Ref { target: strip(t), rel: LinkRel::Texture }).collect();
            refs.extend(info.library_paths.into_iter().map(|t| Ref { target: strip(t), rel: LinkRel::Library }));
            refs
        }),
        // Package paths like /Game/Pack/Textures/T_Rock; resolved against Content/ folders in link_pack.
        "json" => material_refs(local).or_else(|| json_refs(local)),
        "ldtk" => json_refs(local),
        "fnt" => text_file(local).map(|t| attr_refs(&t, "file=")),
        "tmx" | "tsx" | "tx" => text_file(local).map(|t| attr_refs(&t, "source=")),
        "tres" | "tscn" => text_file(local).map(|t| {
            attr_refs(&t, "path=")
                .into_iter()
                .filter_map(|r| {
                    let target = r.target.strip_prefix("res://")?.to_owned();
                    Some(Ref { rel: rel_by_ext(&target), target })
                })
                .collect()
        }),
        "atlas" => text_file(local).map(|t| atlas_refs(&t)),
        "sfz" => crate::instruments::sfz_samples(local)
            .ok()
            .map(|samples| samples.into_iter().map(|t| Ref { target: t, rel: LinkRel::Sample }).collect()),
        "mat" | "prefab" | "unity" | "asset" | "controller" | "overridecontroller" | "physicmaterial" | "mask"
        | "anim" => unity_refs(local),
        "uasset" | "umap" => hoard_unreal::read(local)
            .ok()
            .map(|pkg| pkg.dependencies.into_iter().map(|t| Ref { target: t, rel: LinkRel::Library }).collect()),
        _ => None,
    };
    let mut refs = result.unwrap_or_default();
    refs.sort_by(|a, b| a.target.cmp(&b.target));
    refs.dedup();
    refs
}

/// A small text file's contents (references live in the first few megabytes of any sane file).
fn text_file(local: &Path) -> Option<String> {
    if std::fs::metadata(local).ok()?.len() > 16 << 20 {
        return None;
    }
    std::fs::read_to_string(local).ok()
}

fn rel_by_ext(target: &str) -> LinkRel {
    let lower = target.to_lowercase();
    if IMAGE_EXTS.iter().any(|e| lower.ends_with(e)) { LinkRel::Texture } else { LinkRel::Library }
}

/// Values of `name="…"` (or `name=…` up to whitespace) attributes: `BMFont` pages, Tiled sources, Godot paths.
pub(super) fn attr_refs(text: &str, name: &str) -> Vec<Ref> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find(name) {
        // Only whole attribute names: `source=` but not `imagesource=`.
        let boundary = i == 0 || rest.as_bytes()[i - 1].is_ascii_whitespace() || rest.as_bytes()[i - 1] == b'<';
        rest = &rest[i + name.len()..];
        if !boundary {
            continue;
        }
        let value = if let Some(q) = rest.strip_prefix('"') {
            q.split('"').next()
        } else if let Some(q) = rest.strip_prefix('\'') {
            q.split('\'').next()
        } else {
            rest.split(|c: char| c.is_whitespace() || c == '>').next()
        };
        if let Some(v) = value.map(str::trim).filter(|v| !v.is_empty() && !v.starts_with("uid://")) {
            out.push(Ref { target: v.to_owned(), rel: rel_by_ext(v) });
        }
    }
    out
}

/// Spine / libGDX texture atlases: each page starts with its image's file name on a line of its own.
pub(super) fn atlas_refs(text: &str) -> Vec<Ref> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.contains(':') && IMAGE_EXTS.iter().any(|e| l.to_lowercase().ends_with(e)))
        .map(|l| Ref { target: l.to_owned(), rel: LinkRel::Texture })
        .collect()
}

/// Image paths in JSON: `relPath`s (`LDtk`) and `meta.image` (`TexturePacker`).
fn json_refs(local: &Path) -> Option<Vec<Ref>> {
    fn walk(v: &Value, out: &mut Vec<Ref>, depth: usize) {
        if depth > 12 {
            return;
        }
        match v {
            Value::Object(m) => {
                for (k, v) in m {
                    if let (true, Some(s)) = (k == "relPath" || k == "image", v.as_str()) {
                        out.push(Ref { target: s.to_owned(), rel: rel_by_ext(s) });
                    } else {
                        walk(v, out, depth + 1);
                    }
                }
            }
            Value::Array(a) => a.iter().for_each(|v| walk(v, out, depth + 1)),
            _ => {}
        }
    }
    let doc: Value = serde_json::from_str(&text_file(local)?).ok()?;
    let mut out = Vec::new();
    walk(&doc, &mut out, 0);
    // A DragonBones skeleton names no files: its atlas is the `_tex.json` and `_tex.png` beside it.
    let name = local.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    if let (true, Some([sheet, image])) = (doc.get("armature").is_some(), crate::dragonbones::atlas_files(name)) {
        out.push(Ref { target: sheet, rel: LinkRel::Library });
        out.push(Ref { target: image, rel: LinkRel::Texture });
    }
    // Nor does a Spine one: its atlas is named after it, or is the only one in its folder.
    if doc.get("skeleton").is_some_and(Value::is_object) && doc.get("bones").is_some_and(Value::is_array) {
        out.extend(crate::spine::atlas_files(name).into_iter().map(|target| Ref { target, rel: LinkRel::Library }));
        out.push(Ref { target: FOLDER_ATLAS.into(), rel: LinkRel::Library });
    }
    Some(out)
}

/// A Spine skeleton's last resort: the only atlas in its folder, when none is named after it.
pub(super) const FOLDER_ATLAS: &str = "folder-atlas:";

pub(super) fn is_atlas(path: &str) -> bool {
    let lower = path.to_lowercase();
    Path::new(lower.strip_suffix(".txt").unwrap_or(&lower)).extension().is_some_and(|e| e == "atlas")
}

/// GUID references in a text-serialized Unity file (`{fileID: 2800000, guid: 0123…, type: 3}`). Resolved through
/// the `.meta` files the pack came with.
fn unity_refs(local: &Path) -> Option<Vec<Ref>> {
    if std::fs::metadata(local).ok()?.len() > 32 << 20 {
        return None;
    }
    let text = std::fs::read_to_string(local).ok()?;
    if !text.starts_with("%YAML") {
        return None; // binary-serialized: nothing we can read
    }
    let mut refs = Vec::new();
    let mut rest = text.as_str();
    while let Some(i) = rest.find("guid: ") {
        rest = &rest[i + 6..];
        let g = rest.get(..32).filter(|g| g.bytes().all(|b| b.is_ascii_hexdigit()));
        // All zeros and Unity's built-in resources (0000000000000000e000000000000000 and friends) aren't files.
        if let Some(g) = g.filter(|g| !g.starts_with("0000000000000000")) {
            refs.push(Ref { target: format!("unity-guid:{}", g.to_ascii_lowercase()), rel: LinkRel::Library });
        }
    }
    Some(refs)
}

/// References in a converted Unreal material (`hoard-material/1`): its parent and textures, as package paths.
fn material_refs(local: &Path) -> Option<Vec<Ref>> {
    if std::fs::metadata(local).ok()?.len() > 4 << 20 {
        return None;
    }
    let doc: Value = serde_json::from_slice(&std::fs::read(local).ok()?).ok()?;
    if !doc.get("format")?.as_str()?.starts_with("hoard-material/") {
        return None;
    }
    // "/Game/Pack/T_Rock.T_Rock" names an object; the package is "/Game/Pack/T_Rock".
    let package = |v: &Value| -> Option<String> {
        let path = v.as_str()?;
        let cut = path.rfind('.').filter(|&dot| dot > path.rfind('/').unwrap_or(0));
        Some(cut.map_or(path, |dot| &path[..dot]).to_owned())
    };
    let mut refs: Vec<Ref> =
        doc.get("parent").and_then(package).map(|t| Ref { target: t, rel: LinkRel::Material }).into_iter().collect();
    let textures = doc.get("textures").and_then(Value::as_object).into_iter().flat_map(|m| m.values());
    let referenced = doc.get("referenced_textures").and_then(Value::as_array).into_iter().flatten();
    refs.extend(textures.chain(referenced).filter_map(package).map(|t| Ref { target: t, rel: LinkRel::Texture }));
    Some(refs)
}

fn glb_json(path: &Path) -> Option<Value> {
    let mut f = File::open(path).ok()?;
    let mut header = [0u8; 20];
    f.read_exact(&mut header).ok()?;
    if &header[0..4] != b"glTF" || &header[16..20] != b"JSON" {
        return None;
    }
    let len = u32::from_le_bytes([header[12], header[13], header[14], header[15]]) as usize;
    if len > 128 << 20 {
        return None;
    }
    let mut buf = vec![0u8; len];
    f.read_exact(&mut buf).ok()?;
    serde_json::from_slice(&buf).ok()
}

fn gltf_refs(doc: &Value) -> Vec<Ref> {
    let uris = |key: &str| -> Vec<String> {
        doc.get(key)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|v| v.get("uri")?.as_str())
            .filter(|u| !u.starts_with("data:"))
            .map(str::to_owned)
            .collect()
    };
    let mut refs: Vec<Ref> = uris("buffers").into_iter().map(|t| Ref { target: t, rel: LinkRel::Buffer }).collect();
    refs.extend(uris("images").into_iter().map(|t| Ref { target: t, rel: LinkRel::Texture }));
    refs
}

fn lines(path: &Path) -> impl Iterator<Item = String> {
    File::open(path).ok().into_iter().flat_map(|f| BufReader::new(f).lines().map_while(Result::ok))
}

fn obj_refs(path: &Path) -> Vec<Ref> {
    lines(path)
        .filter_map(|l| l.trim().strip_prefix("mtllib ").map(|r| r.trim().to_owned()))
        .map(|t| Ref { target: t, rel: LinkRel::Material })
        .collect()
}

fn mtl_refs(path: &Path) -> Vec<Ref> {
    lines(path)
        .filter_map(|l| {
            let l = l.trim();
            let key = l.split_whitespace().next()?.to_lowercase();
            let is_map = key.starts_with("map_") || matches!(key.as_str(), "bump" | "disp" | "decal" | "refl" | "norm");
            // Options like `-bm 0.5` can precede the file name; the name is the last token.
            is_map.then(|| l.split_whitespace().last().map(str::to_owned)).flatten()
        })
        .filter(|t| t.contains('.'))
        .map(|t| Ref { target: t, rel: LinkRel::Texture })
        .collect()
}

/// FBX stores texture paths as plain strings, so scan for printable runs ending in an image extension.
fn fbx_refs(path: &Path) -> Vec<Ref> {
    let mut buf = Vec::new();
    if File::open(path).and_then(|f| f.take(128 << 20).read_to_end(&mut buf)).is_err() {
        return Vec::new();
    }
    let lower = buf.to_ascii_lowercase();
    let mut out = BTreeSet::new();
    for ext in IMAGE_EXTS {
        let needle = ext.as_bytes();
        let mut from = 0;
        while let Some(pos) = find(&lower[from..], needle) {
            let end = from + pos + needle.len();
            let mut start = from + pos;
            while start > 0 && end - start < 300 {
                let b = buf[start - 1];
                if !(0x20..0x7f).contains(&b) || b == b'"' {
                    break;
                }
                start -= 1;
            }
            if let Ok(s) = std::str::from_utf8(&buf[start..end]) {
                out.insert(s.to_owned());
            }
            from = end;
        }
    }
    out.into_iter().map(|t| Ref { target: t, rel: LinkRel::Texture }).collect()
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn dae_refs(path: &Path) -> Vec<Ref> {
    let mut text = String::new();
    if File::open(path).and_then(|f| f.take(64 << 20).read_to_string(&mut text)).is_err() {
        return Vec::new();
    }
    let re = regex::Regex::new(r"<init_from>\s*([^<]+?)\s*</init_from>").expect("static regex");
    re.captures_iter(&text)
        .map(|c| c[1].to_owned())
        .filter(|t| IMAGE_EXTS.iter().any(|e| t.to_lowercase().ends_with(e)))
        .map(|t| Ref { target: t, rel: LinkRel::Texture })
        .collect()
}
