//! glTF and GLB: reading a GLB's JSON and binary chunk, a .gltf with its buffers and images packed into one GLB, and
//! a GLB unpacked into a .gltf with its textures, zipped.

use std::collections::BTreeMap;
use std::io::{Cursor, Write};
use std::path::Path;

use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};

use super::Output;

/// The JSON and BIN chunks of a GLB.
pub(crate) fn read_glb(data: &[u8]) -> Result<(Value, Vec<u8>)> {
    ensure!(data.len() >= 20 && &data[..4] == b"glTF", "not a GLB file");
    let mut pos = 12;
    let (mut json, mut bin) = (None, Vec::new());
    while pos + 8 <= data.len() {
        let len = u32::from_le_bytes(data[pos..pos + 4].try_into()?) as usize;
        let kind = &data[pos + 4..pos + 8];
        let chunk = data.get(pos + 8..pos + 8 + len).context("GLB chunk runs past the end")?;
        match kind {
            b"JSON" => json = Some(serde_json::from_slice(chunk)?),
            b"BIN\0" => bin = chunk.to_vec(),
            _ => {}
        }
        pos += 8 + len;
    }
    Ok((json.context("GLB has no JSON chunk")?, bin))
}

/// GLB → `name.gltf` + `name.bin` in a zip (textures stay inside the .bin, which glTF allows).
pub(super) fn glb_to_gltf_zip(path: &Path) -> Result<Output> {
    let (mut doc, bin) = read_glb(&std::fs::read(path)?)?;
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("model");
    let stem = if stem.chars().all(|c| c.is_ascii_digit()) { "model" } else { stem };
    if let Some(b) = doc.get_mut("buffers").and_then(Value::as_array_mut).and_then(|b| b.first_mut()) {
        b["uri"] = json!(format!("{stem}.bin"));
    }
    let mut out = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut out));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .last_modified_time(zip::DateTime::default_for_write());
        zip.start_file(format!("{stem}.gltf"), opts)?;
        zip.write_all(&serde_json::to_vec_pretty(&doc)?)?;
        if !bin.is_empty() {
            zip.start_file(format!("{stem}.bin"), opts)?;
            zip.write_all(&bin)?;
        }
        zip.finish()?;
    }
    Ok(Output { bytes: out, ext: "zip" })
}

/// A file's name without its extension.
pub(super) fn file_stem(path: &Path) -> String {
    path.file_stem().map_or_else(|| "model".to_owned(), |s| s.to_string_lossy().into_owned())
}

/// The files a `.gltf`'s images are read from, by their bytes. Only files inside `root` are read.
pub(super) fn gltf_image_files_found(path: &Path, root: &Path) -> crate::glbfiles::Found {
    let dir = path.parent().unwrap_or(Path::new("."));
    let Ok(doc) = std::fs::read(path).map_err(anyhow::Error::from).and_then(|b| Ok(serde_json::from_slice(&b)?)) else {
        return BTreeMap::new();
    };
    gltf_image_files(&doc)
        .into_iter()
        .filter_map(|rel| {
            let mut file = std::path::PathBuf::new();
            for part in dir.join(&rel).components() {
                match part {
                    std::path::Component::ParentDir => {
                        file.pop();
                    }
                    std::path::Component::CurDir => {}
                    p => file.push(p),
                }
            }
            if !file.starts_with(root) {
                return None;
            }
            // Streamed through the hasher: a texture can be hundreds of megabytes, and only its hash is wanted.
            let hash = blake3::Hasher::new().update_reader(std::fs::File::open(&file).ok()?).ok()?.finalize();
            Some((*hash.as_bytes(), file))
        })
        .collect()
}

/// `.gltf` → `.glb` with its buffers packed in, its images left where they are (a GLB may point at image files
/// beside it, as the glTF did): the same model in one file instead of two, meshopt compression and all. Refused
/// for what can't move losslessly, such as a buffer without a URI.
pub(crate) fn gltf_buffers_to_glb(path: &Path) -> Result<Vec<u8>> {
    build_glb(path, false)
}

/// The buffer files a `.gltf` reads, relative to its folder (data URIs aren't files).
pub(crate) fn gltf_buffer_files(doc: &Value) -> Vec<String> {
    gltf_uris(doc, "buffers")
}

/// The image files a `.gltf` reads, relative to its folder.
pub(crate) fn gltf_image_files(doc: &Value) -> Vec<String> {
    gltf_uris(doc, "images")
}

fn gltf_uris(doc: &Value, key: &str) -> Vec<String> {
    doc.get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|b| b.get("uri").and_then(Value::as_str))
        .filter(|uri| !uri.starts_with("data:"))
        .map(percent_decode)
        .collect()
}

/// Packs a `.gltf` and the files it reads (from its own folder, never outside it) into a GLB with polysmelt, which
/// re-points meshopt-compressed buffer views as it goes. With `images` false, image files stay beside the GLB.
pub(crate) fn build_glb(path: &Path, images: bool) -> Result<Vec<u8>> {
    let dir = path.parent().context("no folder")?;
    let mut doc: Value = serde_json::from_slice(&std::fs::read(path)?)?;
    // A buffer must say where its bytes are, unless it's a meshopt fallback (which decoders never read).
    let fallback = |b: &Value| {
        polysmelt::gltf::MESHOPT_EXTENSIONS
            .iter()
            .any(|n| b.pointer(&format!("/extensions/{n}/fallback")) == Some(&json!(true)))
    };
    let buffers = doc.get("buffers").and_then(Value::as_array).map_or_default(Vec::as_slice);
    ensure!(buffers.iter().all(|b| b.get("uri").is_some() || fallback(b)), "a glTF buffer has no URI");
    // Image files that stay put: their URIs are set aside while the rest is packed, then put back.
    let mut kept = Vec::new();
    if !images {
        for (i, img) in doc.get_mut("images").and_then(Value::as_array_mut).into_iter().flatten().enumerate() {
            let external = img.get("uri").and_then(Value::as_str).is_some_and(|uri| !uri.starts_with("data:"));
            if let Some(obj) = img.as_object_mut().filter(|_| external) {
                kept.push((i, obj.remove("uri")));
            }
        }
    }
    let mut read = polysmelt::gltf::files_in(dir);
    let glb = polysmelt::gltf::pack_glb(&serde_json::to_vec(&doc)?, |uri| {
        read(uri).map_err(|e| match e.kind() {
            polysmelt::ErrorKind::Io => polysmelt::Error::new(
                polysmelt::ErrorKind::MissingAsset,
                format!("glTF file {uri} is missing from the pack"),
            ),
            _ => e,
        })
    })?;
    if kept.is_empty() {
        return Ok(glb);
    }
    // Put the image URIs back into the GLB's JSON chunk.
    let json_len = u32::from_le_bytes(glb.get(12..16).context("short GLB")?.try_into()?) as usize;
    let mut json: Value = serde_json::from_slice(glb.get(20..20 + json_len).context("short GLB")?)?;
    for (i, uri) in kept {
        if let (Some(img), Some(uri)) = (json.pointer_mut(&format!("/images/{i}")).and_then(Value::as_object_mut), uri)
        {
            img.insert("uri".into(), uri);
        }
    }
    let mut text = serde_json::to_vec(&json)?;
    text.resize(text.len().div_ceil(4) * 4, b' ');
    let rest = &glb[20 + json_len..];
    let total = 20 + text.len() + rest.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&u32::try_from(total)?.to_le_bytes());
    out.extend_from_slice(&u32::try_from(text.len())?.to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend(text);
    out.extend_from_slice(rest);
    Ok(out)
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && let Some(v) = s.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(h, 16).ok())
        {
            out.push(v);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gltf_images_are_found_by_their_bytes_inside_the_root_only() {
        let root = tempfile::tempdir().unwrap();
        let model = root.path().join("Models");
        std::fs::create_dir_all(&model).unwrap();
        std::fs::create_dir_all(root.path().join("Textures")).unwrap();
        let wood = vec![7u8; 3 << 20];
        std::fs::write(root.path().join("Textures/wood.png"), &wood).unwrap();
        let gltf = model.join("crate.gltf");
        let doc = json!({ "asset": { "version": "2.0" }, "images": [
            { "uri": "../Textures/wood.png" }, { "uri": "../../outside.png" }, { "uri": "./missing.png" },
        ] });
        std::fs::write(&gltf, serde_json::to_vec(&doc).unwrap()).unwrap();

        let found = gltf_image_files_found(&gltf, root.path());
        let wanted = root.path().join("Textures").join("wood.png");
        assert_eq!(found, BTreeMap::from([(*blake3::hash(&wood).as_bytes(), wanted)]));
    }
}
