//! GLBs handed out with their textures as files beside them instead of inside: a texture shared by several models
//! is downloaded once, and can be opened, swapped or compressed on its own. The model is turned the way its pack's
//! models face, so every model downloaded stands Y up and faces +Z.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Cursor, Write};

use anyhow::{Context, Result};
use hoard_core::Facing;
use serde_json::{Map, Value, json};

/// The folder textures go in, beside the models.
pub(crate) const TEXTURES: &str = "textures";

/// The file a texture came from (its name without the extension), by the BLAKE3 hash of its bytes.
pub(crate) type Names = BTreeMap<[u8; 32], String>;

/// A model as a GLB whose images are files in [`TEXTURES`], each named once.
pub(crate) struct Split {
    pub(crate) glb: Vec<u8>,
    /// `(file name, bytes)`, without the folder.
    pub(crate) textures: Vec<(String, Vec<u8>)>,
}

/// Turn `glb` by `facing` and move its embedded images out to files, named after the files they came from (`names`),
/// the images' own names, the material and slot that use them (`Wood_BaseColor`), or `stem` and a number. The images' bytes leave the BIN chunk, unless something else in
/// it would have to move too (meshopt compression, a second buffer), when only the images' references change.
pub(crate) fn split(glb: &[u8], facing: Facing, stem: &str, names: &Names) -> Result<Split> {
    let (doc, mut bin) = crate::formats::read_glb(glb)?;
    let Value::Object(mut doc) = doc else { anyhow::bail!("the GLB's JSON isn't an object") };
    crate::meshes::turn(&mut doc, facing);

    let views = doc.get("bufferViews").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut textures: Vec<(String, Vec<u8>)> = Vec::new();
    let mut moved = BTreeSet::new();
    let mut used = BTreeSet::new();
    let slots = slot_names(&doc, stem);
    let images = doc.get_mut("images").and_then(Value::as_array_mut).map_or_default(std::mem::take);
    let mut out_images = Vec::with_capacity(images.len());
    for (i, mut img) in images.into_iter().enumerate() {
        let view = img.get("bufferView").and_then(Value::as_u64).map(|v| v as usize);
        let bytes = view.and_then(|v| view_bytes(&views, &bin, v));
        if let (Some(v), Some(bytes), Some(obj)) = (view, bytes, img.as_object_mut()) {
            let ext = image_ext(obj.get("mimeType").and_then(Value::as_str), bytes);
            let file = names.get(blake3::hash(bytes).as_bytes()).map(String::as_str);
            let own = obj.get("name").and_then(Value::as_str).filter(|n| !generic(n));
            let base = file.or(own).or(slots.get(&i).map(String::as_str)).map(clean).filter(|n| !n.is_empty());
            let base = base.unwrap_or_else(|| format!("{}_{i}", clean(stem)));
            let name = place(&mut textures, &base, ext, bytes);
            obj.remove("bufferView");
            obj.remove("mimeType");
            obj.insert("uri".into(), json!(format!("{TEXTURES}/{name}")));
            moved.insert(v);
        }
        out_images.push(img);
    }
    if !out_images.is_empty() {
        doc.insert("images".into(), Value::Array(out_images));
    }
    // A view an image moved out of goes too, unless something else reads it.
    refs(&Value::Object(doc.clone()), &mut used);
    let dropped: BTreeSet<usize> = moved.difference(&used).copied().collect();
    let buffers = doc.get("buffers").and_then(Value::as_array).map_or(0, Vec::len);
    let simple =
        buffers == 1 && views.iter().all(|v| v.get("extensions").is_none() && v.get("buffer") == Some(&json!(0)));
    if simple && !dropped.is_empty() {
        bin = compact(&mut doc, &views, &bin, &dropped)?;
    }
    Ok(Split { glb: write_glb(&doc, &bin)?, textures })
}

/// A name for each image a material uses, by image: the material's name run together (the model's `stem` when it
/// has none) and the slot it fills, in the suffixes Hoard names maps with (`Wood_BaseColor`, `Wood_AO`). Metal and
/// roughness packed with occlusion in one image is `ORM`. An image in several slots takes the first.
fn slot_names(doc: &Map<String, Value>, stem: &str) -> BTreeMap<usize, String> {
    use crate::materials::Role;
    let list = |key: &str| doc.get(key).and_then(Value::as_array).map_or(&[][..], Vec::as_slice);
    let (textures, materials) = (list("textures"), list("materials"));
    // The image a texture shows: its own source, or one an extension gives (KTX2, WebP, DDS).
    let image = |slot: &Value| -> Option<usize> {
        let t = textures.get(slot.get("index")?.as_u64()? as usize)?;
        let ext = t.get("extensions").and_then(Value::as_object);
        let source = t.get("source").or_else(|| ext?.values().find_map(|e| e.get("source")));
        source?.as_u64().map(|s| s as usize)
    };
    let mut out = BTreeMap::new();
    for m in materials {
        let name = m.get("name").and_then(Value::as_str).map(crate::naming::run_together).filter(|n| !n.is_empty());
        let name = name.unwrap_or_else(|| crate::naming::run_together(stem));
        let occlusion = m.get("occlusionTexture").and_then(image);
        let metal = m.get("pbrMetallicRoughness").and_then(|p| p.get("metallicRoughnessTexture")).and_then(image);
        let packed = occlusion.is_some() && occlusion == metal;
        let mut found = Vec::new();
        texture_slots(m, &mut found);
        for (key, slot) in found {
            let Some(img) = image(slot) else { continue };
            let suffix = match key {
                "occlusion" | "metallicRoughness" if packed => Role::Orm.suffix().to_owned(),
                "occlusion" => Role::Ao.suffix().to_owned(),
                "baseColor" | "diffuse" => Role::BaseColor.suffix().to_owned(),
                "normal" => Role::Normal.suffix().to_owned(),
                "emissive" => Role::Emissive.suffix().to_owned(),
                k => k[..1].to_ascii_uppercase() + &k[1..],
            };
            out.entry(img).or_insert_with(|| format!("{name}_{suffix}"));
        }
    }
    out
}

/// The texture slots in a material, core and extensions alike (`baseColorTexture`, `clearcoatNormalTexture`, …), as
/// the slot's name without `Texture` and the slot, in the order they're written.
fn texture_slots<'a>(v: &'a Value, out: &mut Vec<(&'a str, &'a Value)>) {
    let Value::Object(map) = v else { return };
    for (key, value) in map {
        match key.strip_suffix("Texture") {
            Some(slot) if !slot.is_empty() && value.get("index").is_some() => out.push((slot, value)),
            _ => texture_slots(value, out),
        }
    }
}

/// A name an exporter makes up rather than one somebody gave: `Image`, `image_3`, `Texture0`.
fn generic(name: &str) -> bool {
    let word = name.trim().trim_end_matches(|c: char| c.is_ascii_digit() || matches!(c, '_' | '-' | ' ' | '.'));
    ["image", "img", "texture", "tex", ""].iter().any(|g| word.eq_ignore_ascii_case(g))
}

/// `split`, as one file: the GLB alone when it has no textures, otherwise a zip of `<stem>.glb` and [`TEXTURES`].
pub(crate) fn split_output(glb: &[u8], facing: Facing, stem: &str, names: &Names) -> Result<crate::formats::Output> {
    let s = split(glb, facing, stem, names)?;
    if s.textures.is_empty() {
        return Ok(crate::formats::Output { bytes: s.glb, ext: "glb" });
    }
    let mut out = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut out));
        let stored = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .last_modified_time(zip::DateTime::default_for_write());
        zip.start_file(format!("{}.glb", clean(stem)), stored)?;
        zip.write_all(&s.glb)?;
        for (name, bytes) in &s.textures {
            zip.start_file(format!("{TEXTURES}/{name}"), stored)?;
            zip.write_all(bytes)?;
        }
        zip.finish()?;
    }
    Ok(crate::formats::Output { bytes: out, ext: "zip" })
}

/// A file name of letters, digits, `-`, `_` and `.`, safe in a URI and on every file system.
pub(crate) fn clean(name: &str) -> String {
    let s: String = name
        .trim()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') { c } else { '_' })
        .collect();
    let s = s.trim_matches('.').to_owned();
    s.chars().take(100).collect()
}

/// Add an image under `base.ext`, or find it there already. A different image of the same name gets `_2`, `_3`, ….
pub(crate) fn place(textures: &mut Vec<(String, Vec<u8>)>, base: &str, ext: &str, bytes: &[u8]) -> String {
    let base = base.strip_suffix(&format!(".{ext}")).unwrap_or(base);
    if let Some((name, _)) = textures.iter().find(|(_, b)| b == bytes) {
        return name.clone();
    }
    let name = free_name(&format!("{base}.{ext}"), |n| textures.iter().any(|(t, _)| t.eq_ignore_ascii_case(n)));
    textures.push((name.clone(), bytes.to_vec()));
    name
}

/// `name`, or `name_2`, `name_3`, … (before the extension) when `taken` says it is.
pub(crate) fn free_name(name: &str, taken: impl Fn(&str) -> bool) -> String {
    let (base, ext) = name.rsplit_once('.').map_or((name, String::new()), |(b, e)| (b, format!(".{e}")));
    let mut out = name.to_owned();
    let mut n = 2;
    while taken(&out) {
        out = format!("{base}_{n}{ext}");
        n += 1;
    }
    out
}

/// The files of a zip of several models, each name once: textures shared by the models are kept once, by their
/// bytes.
#[derive(Default)]
pub(crate) struct Shelf {
    by_hash: std::collections::HashMap<[u8; 32], String>,
    taken: std::collections::HashSet<String>,
}

impl Shelf {
    /// A name for `name` not used yet, taken now.
    pub(crate) fn name(&mut self, name: &str) -> String {
        let out = free_name(name, |n| self.taken.contains(&n.to_lowercase()));
        self.taken.insert(out.to_lowercase());
        out
    }

    /// Where a file with these bytes goes: where they went before, or a new name for them (and `true`).
    pub(crate) fn place(&mut self, name: &str, bytes: &[u8]) -> (String, bool) {
        let hash = *blake3::hash(bytes).as_bytes();
        if let Some(n) = self.by_hash.get(&hash) {
            return (n.clone(), false);
        }
        let out = self.name(name);
        self.by_hash.insert(hash, out.clone());
        (out, true)
    }
}

/// The extension an image's bytes should be saved under.
fn image_ext(mime: Option<&str>, bytes: &[u8]) -> &'static str {
    match mime {
        Some("image/png") => "png",
        Some("image/jpeg") => "jpg",
        Some("image/webp") => "webp",
        Some("image/ktx2") => "ktx2",
        _ if bytes.starts_with(b"\x89PNG") => "png",
        _ if bytes.starts_with(&[0xFF, 0xD8]) => "jpg",
        _ if bytes.get(8..12) == Some(b"WEBP") => "webp",
        _ if bytes.starts_with(b"\xABKTX 20") => "ktx2",
        _ => "bin",
    }
}

/// A buffer view's bytes in the BIN chunk.
fn view_bytes<'a>(views: &[Value], bin: &'a [u8], v: usize) -> Option<&'a [u8]> {
    let view = views.get(v)?;
    if view.get("buffer").and_then(Value::as_u64) != Some(0) {
        return None;
    }
    let off = view.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let len = view.get("byteLength").and_then(Value::as_u64)? as usize;
    bin.get(off..off.checked_add(len)?)
}

/// Every buffer view something names, anywhere in the document.
fn refs(v: &Value, out: &mut BTreeSet<usize>) {
    match v {
        Value::Object(m) => {
            for (k, x) in m {
                if k == "bufferView"
                    && let Some(i) = x.as_u64()
                {
                    out.insert(i as usize);
                } else {
                    refs(x, out);
                }
            }
        }
        Value::Array(a) => a.iter().for_each(|x| refs(x, out)),
        _ => {}
    }
}

/// Renumber the buffer views things name.
fn renumber(v: &mut Value, map: &BTreeMap<usize, usize>) {
    match v {
        Value::Object(m) => {
            for (k, x) in m.iter_mut() {
                if k == "bufferView"
                    && let Some(new) = x.as_u64().and_then(|i| map.get(&(i as usize)))
                {
                    *x = json!(new);
                } else {
                    renumber(x, map);
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(|x| renumber(x, map)),
        _ => {}
    }
}

/// Lay the views left (all but `dropped`) into a new BIN, each starting on four bytes, and point everything at
/// them.
fn compact(doc: &mut Map<String, Value>, views: &[Value], bin: &[u8], dropped: &BTreeSet<usize>) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(bin.len());
    let mut kept = Vec::new();
    let mut map = BTreeMap::new();
    for (i, view) in views.iter().enumerate() {
        if dropped.contains(&i) {
            continue;
        }
        let bytes = view_bytes(views, bin, i).context("a buffer view runs past the end of the GLB")?;
        out.resize(out.len().next_multiple_of(4), 0);
        let mut view = view.clone();
        view["byteOffset"] = json!(out.len());
        out.extend_from_slice(bytes);
        map.insert(i, kept.len());
        kept.push(view);
    }
    let mut whole = Value::Object(std::mem::take(doc));
    whole["bufferViews"] = Value::Array(Vec::new());
    renumber(&mut whole, &map);
    let Value::Object(mut d) = whole else { unreachable!() };
    if kept.is_empty() {
        d.remove("bufferViews");
        d.remove("buffers");
        out.clear();
    } else {
        d.insert("bufferViews".into(), Value::Array(kept));
        d["buffers"][0]["byteLength"] = json!(out.len());
    }
    *doc = d;
    Ok(out)
}

/// A GLB of `doc` and `bin`.
pub(crate) fn write_glb(doc: &Map<String, Value>, bin: &[u8]) -> Result<Vec<u8>> {
    let mut json = serde_json::to_vec(doc)?;
    json.resize(json.len().next_multiple_of(4), b' ');
    let mut bin = bin.to_vec();
    bin.resize(bin.len().next_multiple_of(4), 0);
    let total = 12 + 8 + json.len() + if bin.is_empty() { 0 } else { 8 + bin.len() };
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&u32::try_from(total)?.to_le_bytes());
    out.extend_from_slice(&u32::try_from(json.len())?.to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json);
    if !bin.is_empty() {
        out.extend_from_slice(&u32::try_from(bin.len())?.to_le_bytes());
        out.extend_from_slice(b"BIN\0");
        out.extend_from_slice(&bin);
    }
    Ok(out)
}

/// Point a GLB's images somewhere else: `rename` maps each `textures/<name>` URI to the name to use instead.
pub(crate) fn rename_textures(glb: &[u8], rename: &BTreeMap<String, String>) -> Result<Vec<u8>> {
    if rename.is_empty() {
        return Ok(glb.to_vec());
    }
    let (doc, bin) = crate::formats::read_glb(glb)?;
    let Value::Object(mut doc) = doc else { anyhow::bail!("the GLB's JSON isn't an object") };
    for img in doc.get_mut("images").and_then(Value::as_array_mut).into_iter().flatten() {
        let new = img.get("uri").and_then(Value::as_str).and_then(|u| u.strip_prefix(&format!("{TEXTURES}/")));
        if let Some(new) = new.and_then(|n| rename.get(n)) {
            img["uri"] = json!(format!("{TEXTURES}/{new}"));
        }
    }
    write_glb(&doc, &bin)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PNG: &[u8] = b"\x89PNG\r\n\x1a\n-not-really-a-png";

    /// A GLB of one triangle (positions in view 1) with a PNG in view 0.
    fn glb_with_image() -> Vec<u8> {
        let mut bin = Vec::new();
        for f in [0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0] {
            bin.extend_from_slice(&f.to_le_bytes());
        }
        let png_at = bin.len();
        bin.extend_from_slice(PNG);
        let doc = json!({
            "asset": { "version": "2.0" },
            "scene": 0,
            "scenes": [{ "nodes": [0] }],
            "nodes": [{ "mesh": 0 }],
            "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0 }, "material": 0 }] }],
            "materials": [{ "pbrMetallicRoughness": { "baseColorTexture": { "index": 0 } } }],
            "textures": [{ "source": 0 }],
            "images": [{ "bufferView": 0, "mimeType": "image/png", "name": "Wood Planks" }],
            "accessors": [{ "bufferView": 1, "componentType": 5126, "count": 3, "type": "VEC3",
                            "min": [0, 0, 0], "max": [1, 1, 0] }],
            "bufferViews": [
                { "buffer": 0, "byteOffset": png_at, "byteLength": PNG.len() },
                { "buffer": 0, "byteOffset": 0, "byteLength": 36 },
            ],
            "buffers": [{ "byteLength": bin.len() }],
        });
        write_glb(doc.as_object().unwrap(), &bin).unwrap()
    }

    #[test]
    fn images_move_out_and_the_bin_shrinks() {
        let s = split(&glb_with_image(), Facing::default(), "Crate", &Names::new()).unwrap();
        assert_eq!(s.textures, vec![("Wood_Planks.png".to_owned(), PNG.to_vec())]);
        let (doc, bin) = crate::formats::read_glb(&s.glb).unwrap();
        assert_eq!(doc["images"][0], json!({ "name": "Wood Planks", "uri": "textures/Wood_Planks.png" }));
        assert_eq!(doc["bufferViews"].as_array().unwrap().len(), 1);
        assert_eq!(doc["accessors"][0]["bufferView"], 0);
        assert_eq!(doc["bufferViews"][0]["byteOffset"], 0);
        assert_eq!(bin.len(), 36);
        assert_eq!(doc["buffers"][0]["byteLength"], 36);
        assert_eq!(f32::from_le_bytes(bin[12..16].try_into().unwrap()), 1.0);
    }

    #[test]
    fn a_turned_pack_turns_the_model() {
        let s = split(&glb_with_image(), Facing::new(90, 0), "Crate", &Names::new()).unwrap();
        let (doc, _) = crate::formats::read_glb(&s.glb).unwrap();
        let root = doc["scenes"][0]["nodes"][0].as_u64().unwrap() as usize;
        assert_eq!(doc["nodes"][root]["name"], "Facing");
        assert_eq!(doc["nodes"][root]["children"], json!([0]));
    }

    #[test]
    fn the_output_is_a_zip_only_with_textures() {
        let out = split_output(&glb_with_image(), Facing::default(), "Crate", &Names::new()).unwrap();
        assert_eq!(out.ext, "zip");
        let mut zip = zip::ZipArchive::new(Cursor::new(out.bytes)).unwrap();
        let names: Vec<String> = zip.file_names().map(|n| n.unwrap().into_owned()).collect();
        assert_eq!(names, ["Crate.glb", "textures/Wood_Planks.png"]);
        assert!(zip.by_name("Crate.glb").is_ok());

        let plain = json!({ "asset": { "version": "2.0" } });
        let glb = write_glb(plain.as_object().unwrap(), &[]).unwrap();
        assert_eq!(split_output(&glb, Facing::default(), "Empty", &Names::new()).unwrap().ext, "glb");

        // Named after the file the texture came from, over the image's own name.
        let names = Names::from([(*blake3::hash(PNG).as_bytes(), "T_Crate_D".to_owned())]);
        let s = split(&glb_with_image(), Facing::default(), "Crate", &names).unwrap();
        assert_eq!(s.textures[0].0, "T_Crate_D.png");
    }

    /// [`glb_with_image`] with its image's name taken off and the material named `material`.
    fn unnamed(material: Value) -> Vec<u8> {
        let (doc, bin) = crate::formats::read_glb(&glb_with_image()).unwrap();
        let mut doc = doc.as_object().unwrap().clone();
        doc["images"][0].as_object_mut().unwrap().remove("name");
        doc["materials"][0] = material;
        write_glb(&doc, &bin).unwrap()
    }

    #[test]
    fn an_unnamed_image_is_named_by_its_material_and_slot() {
        let names = |material: Value| {
            let s = split(&unnamed(material), Facing::default(), "Crate", &Names::new()).unwrap();
            s.textures.into_iter().map(|t| t.0).collect::<Vec<_>>()
        };
        let base = json!({ "name": "old wood", "pbrMetallicRoughness": { "baseColorTexture": { "index": 0 } } });
        assert_eq!(names(base), ["OldWood_BaseColor.png"]);
        // No material name: the model's.
        assert_eq!(names(json!({ "normalTexture": { "index": 0 } })), ["Crate_Normal.png"]);
        // Occlusion in the metal-roughness image.
        let orm = json!({ "name": "Rock", "occlusionTexture": { "index": 0 },
                          "pbrMetallicRoughness": { "metallicRoughnessTexture": { "index": 0 } } });
        assert_eq!(names(orm), ["Rock_ORM.png"]);
        let mr = json!({ "name": "Rock", "pbrMetallicRoughness": { "metallicRoughnessTexture": { "index": 0 } } });
        assert_eq!(names(mr), ["Rock_MetallicRoughness.png"]);
        let coat = json!({ "name": "Paint",
                           "extensions": { "KHR_materials_clearcoat": { "clearcoatNormalTexture": { "index": 0 } } } });
        assert_eq!(names(coat), ["Paint_ClearcoatNormal.png"]);
        // Used by nothing: the model's name and a number.
        assert_eq!(names(json!({})), ["Crate_0.png"]);
    }

    #[test]
    fn made_up_image_names_give_way_to_the_slot() {
        assert!(generic("Image") && generic("image_3") && generic("Texture0") && generic("tex-12"));
        assert!(!generic("Wood Planks") && !generic("Texture Wood"));
    }

    #[test]
    fn names_are_shared_by_the_same_bytes_and_numbered_for_others() {
        let mut t = Vec::new();
        assert_eq!(place(&mut t, "wood", "png", b"a"), "wood.png");
        assert_eq!(place(&mut t, "other", "png", b"a"), "wood.png");
        assert_eq!(place(&mut t, "Wood", "png", b"b"), "Wood_2.png");
        assert_eq!(place(&mut t, "wood.png", "png", b"c"), "wood_3.png");
    }

    #[test]
    fn a_shelf_keeps_each_file_once() {
        let mut shelf = Shelf::default();
        assert_eq!(shelf.place("wood.png", b"a"), ("wood.png".to_owned(), true));
        assert_eq!(shelf.place("bark.png", b"a"), ("wood.png".to_owned(), false));
        assert_eq!(shelf.place("Wood.png", b"b"), ("Wood_2.png".to_owned(), true));
        assert_eq!(shelf.name("Crate.glb"), "Crate.glb");
        assert_eq!(shelf.name("crate.glb"), "crate_2.glb");
        assert_eq!(free_name("README", |n| n == "README"), "README_2");
    }

    #[test]
    fn renaming_points_images_at_the_new_files() {
        let s = split(&glb_with_image(), Facing::default(), "Crate", &Names::new()).unwrap();
        let rename = BTreeMap::from([("Wood_Planks.png".to_owned(), "Wood_Planks_2.png".to_owned())]);
        let (doc, _) = crate::formats::read_glb(&rename_textures(&s.glb, &rename).unwrap()).unwrap();
        assert_eq!(doc["images"][0]["uri"], "textures/Wood_Planks_2.png");
    }
}
