//! Dependency discovery: which files in a pack does each file need?
//!
//! - glTF/GLB: `buffers[].uri` and `images[].uri`
//! - OBJ: `mtllib` → MTL: `map_Kd`, `map_Bump`, `norm`, … (bundles follow links transitively)
//! - FBX: texture file names embedded in the binary (absolute artist paths are matched by file name)
//! - DAE: `<init_from>` image paths
//! - .blend: external image paths and linked libraries (other .blend files)
//! - animations: the Unity/Mixamo `Model@Clip.fbx` naming convention
//! - guesses: textures whose name starts with the model's name, for models that reference nothing
//!
//! Extraction happens per file during analysis (`extract_refs`); resolution needs the whole pack, so it
//! runs after the assets are inserted (`link_pack`).

use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::Result;
use hoard_core::{AssetKind, LinkRel};
use rusqlite::{Connection, params};
use serde_json::Value;

const IMAGE_EXTS: &[&str] =
    &[".png", ".jpg", ".jpeg", ".tga", ".tif", ".tiff", ".psd", ".bmp", ".dds", ".exr", ".hdr", ".webp", ".ktx2"];
const MAX_GUESSES: usize = 12;

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
fn attr_refs(text: &str, name: &str) -> Vec<Ref> {
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
fn atlas_refs(text: &str) -> Vec<Ref> {
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
    Some(out)
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

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16)
        {
            out.push(v);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Join a reference onto the referring file's folder, normalizing `.`/`..`. `None` if it escapes the pack.
fn join_rel(from_rel: &str, target: &str) -> Option<String> {
    let target = percent_decode(target.trim_start_matches("file://")).replace('\\', "/");
    let dir = from_rel.rsplit_once('/').map_or("", |(d, _)| d);
    let mut parts: Vec<&str> =
        if target.starts_with('/') { Vec::new() } else { dir.split('/').filter(|s| !s.is_empty()).collect() };
    for seg in target.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    Some(parts.join("/"))
}

/// An Unreal package path (`/Game/Pack/T_Rock`): rooted, no file extension.
fn is_package_path(target: &str) -> bool {
    target.starts_with('/') && !base_name(target).contains('.')
}

/// The pack file for an Unreal package path, matched against `.uasset`/`.umap` files under a `Content/` folder
/// (`Pack/Content/Pack/Textures/T_Rock.uasset` for `/Game/Pack/Textures/T_Rock`), or failing that by path suffix.
fn resolve_unreal(assets: &[PackAsset], content_rel: &str) -> Option<usize> {
    let want = content_rel.to_lowercase();
    // Converted assets answer to the package they came from.
    let packages: Vec<(usize, String)> = assets
        .iter()
        .enumerate()
        .filter_map(|(i, a)| {
            let lower = a.origin.as_deref().unwrap_or(&a.path).to_lowercase();
            let no_ext = lower.strip_suffix(".uasset").or_else(|| lower.strip_suffix(".umap"))?.to_owned();
            Some((i, no_ext))
        })
        .collect();
    if let Some((i, _)) = packages.iter().find(|(_, p)| p.ends_with(&format!("content/{want}"))) {
        return Some(*i);
    }
    // Uploads often lose the `Content/<Mount>/` prefix: try ever shorter suffixes of the package path, and the bare
    // name only when it's unambiguous.
    let parts: Vec<&str> = want.split('/').collect();
    for k in 0..parts.len() {
        let suffix = parts[k..].join("/");
        let mut hits = packages.iter().filter(|(_, p)| *p == suffix || p.ends_with(&format!("/{suffix}")));
        let first = hits.next();
        if first.is_some() && (k + 1 < parts.len() || hits.next().is_none()) {
            return first.map(|(i, _)| *i);
        }
    }
    None
}

/// What a reference by id (an Unreal package import, a Unity GUID) means depends on what it points at.
fn rel_for(kind: AssetKind) -> LinkRel {
    match kind {
        AssetKind::Texture => LinkRel::Texture,
        AssetKind::Material => LinkRel::Material,
        AssetKind::Animation => LinkRel::Animation,
        _ => LinkRel::Library,
    }
}

fn base_name(p: &str) -> &str {
    p.rsplit(['/', '\\']).next().unwrap_or(p)
}

fn stem(p: &str) -> &str {
    let file = base_name(p);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

#[derive(Debug)]
struct PackAsset {
    id: String,
    path: String,
    kind: AssetKind,
    /// Path before conversion (`T_Rock.uasset` for what is now `T_Rock.png`).
    origin: Option<String>,
}

/// Path lookups for one pack: exact, case-insensitive, and by bare file name.
#[derive(Debug, Default)]
struct PathIndex {
    exact: HashMap<String, usize>,
    lower: HashMap<String, usize>,
    by_name: HashMap<String, Vec<usize>>,
}

impl PathIndex {
    fn build(assets: &[PackAsset]) -> Self {
        let mut idx = Self::default();
        for (i, a) in assets.iter().enumerate() {
            idx.exact.insert(a.path.clone(), i);
            idx.lower.insert(a.path.to_lowercase(), i);
            idx.by_name.entry(base_name(&a.path).to_lowercase()).or_default().push(i);
        }
        idx
    }

    fn resolve(&self, assets: &[PackAsset], from_rel: &str, target: &str) -> Option<usize> {
        if let Some(joined) = join_rel(from_rel, target)
            && let Some(&i) = self.exact.get(&joined).or_else(|| self.lower.get(&joined.to_lowercase()))
        {
            return Some(i);
        }
        // Absolute artist paths ("C:\Users\me\textures\Rock.png") and wrong folders: fall back to the file
        // name, preferring the candidate that shares the most leading folders with the referring file.
        let candidates = self.by_name.get(&base_name(&percent_decode(target)).to_lowercase())?;
        let from_dir: Vec<&str> = from_rel.split('/').collect();
        candidates
            .iter()
            .copied()
            .max_by_key(|&i| assets[i].path.split('/').zip(&from_dir).take_while(|(a, b)| a == *b).count())
    }
}

/// Resolve the refs collected during analysis into `asset_links` rows, plus animation and name-based links.
/// `refs` maps asset id -> refs extracted from that asset's file.
pub(crate) fn link_pack(conn: &Connection, pack_id: &str, refs: &HashMap<String, Vec<Ref>>) -> Result<usize> {
    let assets: Vec<PackAsset> = {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.path, a.kind, o.path FROM assets a LEFT JOIN asset_origin o ON o.asset_id = a.id
             WHERE a.pack_id = ?1",
        )?;
        stmt.query_map([pack_id], |r| {
            Ok(PackAsset {
                id: r.get(0)?,
                path: r.get(1)?,
                kind: AssetKind::parse(&r.get::<_, String>(2)?).unwrap_or(AssetKind::Other),
                origin: r.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    let index = PathIndex::build(&assets);
    let mut links: BTreeSet<(usize, usize, LinkRel)> = BTreeSet::new();
    // Unity GUID -> asset, from the .meta files the pack came with.
    let by_path: HashMap<&str, usize> = assets.iter().enumerate().map(|(i, a)| (a.path.as_str(), i)).collect();
    let unity: HashMap<String, usize> = {
        let mut stmt = conn.prepare("SELECT guid, path FROM unity_guids WHERE pack_id = ?1")?;
        let rows = stmt.query_map([pack_id], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        let mut m = HashMap::new();
        for row in rows {
            let (guid, path) = row?;
            if let Some(&i) = by_path.get(path.as_str()) {
                m.insert(guid, i);
            }
        }
        m
    };

    let by_id: HashMap<&str, usize> = assets.iter().enumerate().map(|(i, a)| (a.id.as_str(), i)).collect();
    for (id, list) in refs {
        let Some(&from) = by_id.get(id.as_str()) else { continue };
        for r in list {
            if let Some(guid) = r.target.strip_prefix("unity-guid:") {
                if let Some(&to) = unity.get(guid)
                    && to != from
                {
                    links.insert((from, to, rel_for(assets[to].kind)));
                }
                continue;
            }
            if let Some(pkg) = hoard_unreal::content_relative(&r.target).filter(|_| is_package_path(&r.target)) {
                if let Some(to) = resolve_unreal(&assets, pkg)
                    && to != from
                {
                    links.insert((from, to, rel_for(assets[to].kind)));
                }
                continue;
            }
            if let Some(to) = index.resolve(&assets, &assets[from].path, &r.target)
                && to != from
            {
                links.insert((from, to, r.rel));
            }
        }
    }

    // "Knight@Run.fbx" is an animation clip for "Knight.fbx" (or any model format with that stem).
    let models: Vec<usize> = (0..assets.len()).filter(|&i| assets[i].kind == AssetKind::Model).collect();
    for &clip in &models {
        let s = stem(&assets[clip].path);
        let Some((base, _)) = s.split_once('@') else { continue };
        for &m in &models {
            if m != clip && stem(&assets[m].path).eq_ignore_ascii_case(base) {
                links.insert((m, clip, LinkRel::Animation));
            }
        }
    }

    // Models that reference no textures: guess from names ("Barrel.fbx" + "Barrel_Albedo.png").
    for &m in &models {
        let references_something =
            links.iter().any(|(f, _, r)| *f == m && matches!(r, LinkRel::Texture | LinkRel::Material));
        if stem(&assets[m].path).contains('@') || references_something {
            continue;
        }
        let ms = stem(&assets[m].path).to_lowercase();
        if ms.len() < 3 {
            continue;
        }
        let guesses = assets
            .iter()
            .enumerate()
            .filter(|(_, a)| a.kind == AssetKind::Texture)
            .filter(|(_, a)| {
                let ts = stem(&a.path).to_lowercase();
                ts == ms || ts.strip_prefix(&ms).is_some_and(|rest| rest.starts_with(['_', '-', ' ', '.']))
            })
            .map(|(i, _)| i)
            .take(MAX_GUESSES)
            .collect::<Vec<_>>();
        for t in guesses {
            links.insert((m, t, LinkRel::Guess));
        }
    }

    let mut stmt =
        conn.prepare("INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING")?;
    for (f, t, rel) in &links {
        stmt.execute(params![assets[*f].id, assets[*t].id, rel.as_str()])?;
    }
    Ok(links.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins() {
        assert_eq!(join_rel("Models/crate.gltf", "textures/a%20b.png").as_deref(), Some("Models/textures/a b.png"));
        assert_eq!(join_rel("Models/sub/x.obj", "../t.png").as_deref(), Some("Models/t.png"));
        assert_eq!(join_rel("x.obj", "../../t.png"), None);
        assert_eq!(join_rel("Models/x.fbx", "C:\\art\\Rock.png").as_deref(), Some("Models/C:/art/Rock.png"));
    }

    #[test]
    fn reads_tile_font_and_atlas_references() {
        let tmx = r#"<map><tileset firstgid="1" source="terrain.tsx"/><imagelayer><image source="bg/sky.png"/></imagelayer></map>"#;
        let refs: Vec<String> = attr_refs(tmx, "source=").into_iter().map(|r| r.target).collect();
        assert_eq!(refs, ["terrain.tsx", "bg/sky.png"]);
        let fnt = "info face=\"Pixel\" size=16\npage id=0 file=\"pixel_0.png\"\n";
        assert_eq!(attr_refs(fnt, "file=")[0].target, "pixel_0.png");
        assert_eq!(attr_refs(fnt, "file=")[0].rel, LinkRel::Texture);
        let atlas = "\nhero.png\nsize: 512,512\nformat: RGBA8888\nidle\n  xy: 0, 0\n";
        assert_eq!(atlas_refs(atlas)[0].target, "hero.png");
        let tres = r#"[ext_resource type="Texture2D" uid="uid://abc" path="res://art/rock.png" id="1"]"#;
        assert_eq!(attr_refs(tres, "path=")[0].target, "res://art/rock.png");
    }

    #[test]
    fn reads_unity_guid_references() {
        let dir = tempfile::tempdir().unwrap();
        let mat = dir.path().join("M.mat");
        std::fs::write(
            &mat,
            "%YAML 1.1\nMaterial:\n  m_Shader: {fileID: 46, guid: 0000000000000000f000000000000000, type: 0}\n  \
             m_Texture: {fileID: 2800000, guid: 7E1A2E907C7A4339BE186C6B102FDE95, type: 3}\n",
        )
        .unwrap();
        let refs = extract_refs(&mat, "mat");
        assert_eq!(refs.len(), 1, "built-in shader GUIDs are skipped");
        assert_eq!(refs[0].target, "unity-guid:7e1a2e907c7a4339be186c6b102fde95");
    }

    #[test]
    fn resolves_unreal_package_paths() {
        let assets = vec![
            PackAsset {
                id: "a".into(),
                path: "Forest/Content/Forest/Meshes/SM_Rock.uasset".into(),
                kind: AssetKind::Model,
                origin: None,
            },
            PackAsset {
                id: "b".into(),
                path: "Forest/Content/Forest/Textures/T_Rock_D.uasset".into(),
                kind: AssetKind::Texture,
                origin: None,
            },
        ];
        let rel = hoard_unreal::content_relative("/Game/Forest/Textures/T_Rock_D").unwrap();
        assert_eq!(resolve_unreal(&assets, rel), Some(1));
        // After conversion the texture is a PNG that remembers its package.
        let mut converted = assets;
        converted[1].path = "Forest/Content/Forest/Textures/T_Rock_D.png".into();
        converted[1].origin = Some("Forest/Content/Forest/Textures/T_Rock_D.uasset".into());
        assert_eq!(resolve_unreal(&converted, rel), Some(1));
        // Uploaded without the Content/Forest prefix.
        converted[1].origin = Some("Textures/T_Rock_D.uasset".into());
        assert_eq!(resolve_unreal(&converted, rel), Some(1));
        assert_eq!(resolve_unreal(&converted, "Forest/Textures/T_Missing"), None);
        assert!(is_package_path("/Game/Forest/Textures/T_Rock_D"));
        assert!(!is_package_path("/textures/rock.png"));
    }

    #[test]
    fn resolves_by_name_fallback() {
        let assets = vec![
            PackAsset { id: "a".into(), path: "Models/barrel.fbx".into(), kind: AssetKind::Model, origin: None },
            PackAsset {
                id: "b".into(),
                path: "Models/Textures/Barrel_Albedo.png".into(),
                kind: AssetKind::Texture,
                origin: None,
            },
            PackAsset {
                id: "c".into(),
                path: "Other/Barrel_Albedo.png".into(),
                kind: AssetKind::Texture,
                origin: None,
            },
        ];
        let idx = PathIndex::build(&assets);
        assert_eq!(idx.resolve(&assets, "Models/barrel.fbx", "C:\\Users\\art\\barrel_albedo.PNG"), Some(1));
    }
}

/// The family a file belongs to: same folder, same kind, same name once a trailing variant number is stripped
/// (`whoosh_01.wav`, `whoosh_02.wav`, `Whoosh (3).wav` → `sfx/whoosh`). Single-letter suffixes count only for
/// audio, since on textures they're usually map types (`rock_n.png`).
pub(crate) fn family_base(rel: &str, kind: AssetKind) -> Option<String> {
    let (dir, file) = rel.rsplit_once('/').unwrap_or(("", rel));
    let stem = file.rsplit_once('.').map_or(file, |(s, _)| s).trim().to_lowercase();
    let is_sep = |c: char| matches!(c, '_' | '-' | ' ' | '.');
    let base = if let Some(inner) = stem.strip_suffix(')')
        && let Some((b, n)) = inner.rsplit_once('(')
        && !n.is_empty()
        && n.chars().all(|c| c.is_ascii_digit())
    {
        // "Whoosh (3)"
        b
    } else if let Some(b) = stem.strip_suffix(|c: char| c.is_ascii_digit()) {
        // "whoosh_01", "tree2", "rock_v3" (at most four digits: "texture_2048" is a size, not a variant)
        let b = b.trim_end_matches(|c: char| c.is_ascii_digit());
        if stem.len() - b.len() > 4 {
            return None;
        }
        b.strip_suffix('v').filter(|b| b.ends_with(is_sep)).unwrap_or(b)
    } else if kind == AssetKind::Audio
        && let Some((b, letter)) = stem.rsplit_once(is_sep)
        && letter.len() == 1
        && letter.chars().all(|c| c.is_ascii_lowercase())
    {
        // "hit_a", "hit_b"
        b
    } else {
        return None;
    };
    let base = base.trim_end_matches(is_sep);
    (!base.is_empty()).then(|| format!("{dir}/{base}\0{}", kind.as_str()))
}

/// Group a pack's numbered variants into families (only groups of two or more). Returns the number of families.
pub(crate) fn assign_families(conn: &Connection, pack_id: &str) -> Result<usize> {
    let rows: Vec<(String, String, String)> = {
        let mut stmt = conn.prepare("SELECT id, path, kind FROM assets WHERE pack_id = ?1")?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for (id, path, kind) in rows {
        let Some(kind) = AssetKind::parse(&kind) else { continue };
        if matches!(kind, AssetKind::Other | AssetKind::Document) {
            continue;
        }
        if let Some(base) = family_base(&path, kind) {
            groups.entry(format!("{pack_id}\0{base}")).or_default().push(id);
        }
    }
    conn.execute("DELETE FROM asset_family WHERE asset_id IN (SELECT id FROM assets WHERE pack_id = ?1)", [pack_id])?;
    let mut n = 0;
    let mut stmt = conn.prepare("INSERT OR REPLACE INTO asset_family (asset_id, family_key) VALUES (?1, ?2)")?;
    for (key, ids) in groups.iter().filter(|(_, ids)| ids.len() >= 2) {
        n += 1;
        for id in ids {
            stmt.execute(params![id, key])?;
        }
    }
    Ok(n)
}

#[cfg(test)]
mod family_tests {
    use super::*;

    #[test]
    fn finds_variant_bases() {
        let b = |p| family_base(p, AssetKind::Audio);
        assert_eq!(b("sfx/Whoosh_01.wav"), b("sfx/whoosh-12.wav"));
        assert_eq!(b("sfx/Whoosh (3).wav"), b("sfx/whoosh_01.wav"));
        assert_eq!(b("sfx/hit_a.wav"), b("sfx/hit_b.wav"));
        assert_eq!(b("sfx/rock_v2.wav"), b("sfx/rock_3.wav"));
        assert_ne!(b("sfx/whoosh_01.wav"), b("other/whoosh_01.wav"));
        assert_eq!(b("sfx/whoosh.wav"), None);
        assert_eq!(family_base("tex/rock_n.png", AssetKind::Texture), None);
        assert_eq!(
            family_base("tex/sand_2048.png", AssetKind::Texture),
            Some("tex/sand_\0texture".replace("_\0", "\0"))
        );
        assert_eq!(family_base("tex/sand_20480.png", AssetKind::Texture), None);
    }
}
