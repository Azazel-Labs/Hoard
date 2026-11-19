//! Getting a pack's files out: archives inside it unpacked (zip, tar, `.unitypackage`, 7z and RAR, whole or in
//! parts, see `archives`) within a size budget, its root found, junk and sidecar files left out, and a glTF's loose
//! buffers and images packed into one GLB.

use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use anyhow::{Context, Result};

use super::{ext_of, workers};

mod archives;
use archives::{Format, Joined};

const MAX_ARCHIVE_DEPTH: u32 = 3;
/// Clean a client- or archive-supplied relative path. Returns `None` for anything that could escape
/// the target directory (`..`, absolute paths, drive letters) or is empty.
pub(crate) fn safe_rel_path(raw: &str) -> Option<String> {
    let mut parts = Vec::new();
    for part in raw.split(['/', '\\']) {
        let part: String = part.chars().filter(|c| !c.is_control()).collect();
        let part = part.trim();
        match part {
            "" | "." => {}
            ".." => return None,
            p if p.ends_with(':') => {} // "C:" drive prefix
            p => parts.push(p.to_owned()),
        }
    }
    if parts.is_empty() { None } else { Some(parts.join("/")) }
}

pub(crate) fn is_junk(rel: &str) -> bool {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    rel.split('/').any(|c| matches!(c, "__MACOSX" | ".git" | ".svn" | ".hg"))
        || matches!(file, ".DS_Store" | "Thumbs.db" | "desktop.ini")
        || file.starts_with("._")
}

/// Engine sidecars that only make sense next to their engine's project (Unity `.meta`, Godot `.import`). Kept
/// through unpacking so Unity GUIDs can be read from them, then left out of the pack.
pub(super) fn is_sidecar(rel: &str) -> bool {
    Path::new(rel).extension().is_some_and(|e| e.eq_ignore_ascii_case("meta") || e.eq_ignore_ascii_case("import"))
}

/// A Unity `.meta` that cuts its image into sprites: kept in the pack (unlike other sidecars), as the sheet of its
/// image's sprites.
fn sprite_meta(local: &Path, rel: &str) -> bool {
    let lower = rel.to_ascii_lowercase();
    let Some(image) = lower.strip_suffix(".meta") else { return false };
    matches!(ext_of(image).as_str(), "png" | "psd" | "tga" | "jpg" | "jpeg" | "gif" | "bmp" | "tif" | "tiff" | "webp")
        && fs::metadata(local).is_ok_and(|m| m.len() <= 8 << 20)
        && fs::read_to_string(local).is_ok_and(|t| crate::atlas::unity(&t, "").is_some())
}

/// Whether a file of the pack folder becomes one of its assets: not junk left by an engine.
pub(super) fn kept(root: &Path, f: &Path) -> bool {
    let rel = rel_of(root, f);
    !is_sidecar(&rel) || sprite_meta(f, &rel)
}

/// What an upload may still unpack to, shared by the unpacking threads, plus a running file count for progress.
#[derive(Debug)]
pub(super) struct Budget {
    pub(super) bytes_left: AtomicU64,
    pub(super) files: AtomicUsize,
    pub(super) pack_id: String,
    /// Archives that wouldn't unpack, and why: they're kept as plain files, and listed in Settings → Issues.
    pub(super) unpack_failures: std::sync::Mutex<Vec<(PathBuf, String)>>,
}

impl Budget {
    pub(super) fn new(bytes: u64, pack_id: &str) -> Self {
        Self {
            bytes_left: AtomicU64::new(bytes),
            files: AtomicUsize::new(0),
            pack_id: pack_id.to_owned(),
            unpack_failures: std::sync::Mutex::default(),
        }
    }

    fn reserve(&self, n: u64) -> Result<()> {
        self.bytes_left
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |left| left.checked_sub(n))
            .map(drop)
            .map_err(|_| anyhow::anyhow!("archive expands past the size limit (raise Largest pack in Settings)"))
    }

    /// Stream `src` to `dest`, a buffer at a time, charging every byte before it's written: a zip bomb stops at
    /// the limit, however many threads are unpacking.
    fn copy_limited(&self, src: &mut (impl Read + ?Sized), dest: &Path) -> Result<()> {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut out = File::create(dest)?;
        let mut buf = vec![0u8; 64 * 1024];
        loop {
            let n = match src.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            };
            self.reserve(n as u64)?;
            out.write_all(&buf[..n])?;
        }
        self.placed();
        Ok(())
    }

    /// Count a file unpacked, for the progress shown while unpacking.
    fn placed(&self) {
        let n = self.files.fetch_add(1, Ordering::Relaxed) + 1;
        if n.is_multiple_of(250) {
            crate::activity::job_note(crate::jobs::INGEST, Some(&self.pack_id), &format!("unpacked {n} files"));
        }
    }
}

fn walk_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if ft.is_dir() {
            walk_files(&entry.path(), out)?;
        } else if ft.is_file() {
            out.push(entry.path());
        }
    }
    Ok(())
}

/// Every file under a pack's root but junk (OS litter, VCS folders); Unity sidecars are still in.
pub(super) fn listed(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    walk_files(root, &mut files)?;
    files.retain(|f| !is_junk(&rel_of(root, f)));
    Ok(files)
}

/// An uploaded pack's `.gltf` models become `.glb`, their buffers packed in: a `.bin` is no use without its glTF,
/// and one file is easier to keep, view and send. Their images stay as files beside it (a GLB may point at them),
/// so textures are still assets of their own. Only done when nothing is lost: each buffer file is read by this glTF
/// alone, no `.glb` of that name is there already, and none of the files is known to the pack or to Unity. Returns
/// how many were converted. Never run on a watched folder: Hoard doesn't change the files there.
pub(super) fn pack_gltfs(root: &Path, files: &[PathBuf], keep: &HashSet<String>) -> usize {
    let dir_rel = |gltf: &str, uri: &str| -> String {
        match gltf.rsplit_once('/') {
            Some((dir, _)) => format!("{dir}/{uri}"),
            None => uri.to_owned(),
        }
    };
    let gltfs: Vec<(String, serde_json::Value)> = files
        .iter()
        .map(|f| rel_of(root, f))
        .filter(|rel| ext_of(rel) == "gltf")
        .filter_map(|rel| {
            let doc = serde_json::from_slice(&fs::read(root.join(&rel)).ok()?).ok()?;
            Some((rel, doc))
        })
        .collect();
    // How many glTFs read each file, as a buffer or an image.
    let mut readers: HashMap<String, usize> = HashMap::new();
    for (rel, doc) in &gltfs {
        let mut uris = crate::formats::gltf_buffer_files(doc);
        uris.extend(crate::formats::gltf_image_files(doc));
        uris.sort();
        uris.dedup();
        for uri in uris {
            *readers.entry(dir_rel(rel, &uri)).or_default() += 1;
        }
    }
    let mut done = 0;
    for (rel, doc) in &gltfs {
        let glb = format!("{}.glb", &rel[..rel.len() - ".gltf".len()]);
        let buffers: Vec<String> = crate::formats::gltf_buffer_files(doc).iter().map(|u| dir_rel(rel, u)).collect();
        let shared = buffers.iter().any(|b| readers.get(b).copied().unwrap_or(0) > 1 || keep.contains(b));
        if shared || keep.contains(rel) || keep.contains(&glb) || root.join(&glb).exists() {
            continue;
        }
        let bytes = match crate::formats::gltf_buffers_to_glb(&root.join(rel)) {
            Ok(out) => out,
            Err(e) => {
                tracing::debug!(file = %rel, error = %format!("{e:#}"), "glTF kept as it is");
                continue;
            }
        };
        if let Err(e) = fs::write(root.join(&glb), bytes) {
            tracing::warn!(file = %rel, error = %e, "couldn't write the GLB; glTF kept as it is");
            fs::remove_file(root.join(&glb)).ok();
            continue;
        }
        fs::remove_file(root.join(rel)).ok();
        for b in &buffers {
            fs::remove_file(root.join(b)).ok();
        }
        done += 1;
    }
    done
}

/// The relative paths a pack folder would give its assets: what a rescan compares the pack against.
pub(crate) fn pack_rels(root: &Path) -> Result<HashSet<String>> {
    Ok(listed(root)?.iter().filter(|f| kept(root, f)).map(|f| rel_of(root, f)).collect())
}

pub(super) fn rel_of(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Unpack the archives under `dir` in place, and the archives inside those, down to [`MAX_ARCHIVE_DEPTH`]. An archive
/// in parts is unpacked once, from all of them. One that won't unpack is kept as it is and noted in the budget.
pub(super) fn expand_archives(dir: &Path, budget: &Budget, depth: u32) -> Result<()> {
    if depth > MAX_ARCHIVE_DEPTH {
        return Ok(());
    }
    let mut files = Vec::new();
    walk_files(dir, &mut files)?;
    for set in archives::sets(&files) {
        let first = &set.parts[0];
        let mut target = first.with_file_name(&set.stem);
        while target.exists() {
            target.set_file_name(format!("{}_x", target.file_name().unwrap_or_default().to_string_lossy()));
        }
        let res = match &set.problem {
            Some(problem) => Err(anyhow::anyhow!("{problem}")),
            None => unpack_set(&set, &target, budget),
        };
        match res {
            Ok(()) => {
                for part in &set.parts {
                    fs::remove_file(part)?;
                }
                expand_archives(&target, budget, depth + 1)?;
            }
            // A corrupt nested archive is kept as a plain file instead of failing the whole pack,
            // but limit violations must abort.
            Err(e) if e.to_string().contains("size limit") => return Err(e),
            Err(e) => {
                tracing::warn!(archive = %first.display(), error = %e, "could not unpack; keeping as a file");
                fs::remove_dir_all(&target).ok();
                budget
                    .unpack_failures
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .push((first.clone(), format!("{e:#}")));
            }
        }
    }
    Ok(())
}

/// Unpack one archive, whole or in parts, into `target`.
fn unpack_set(set: &archives::Set, target: &Path, budget: &Budget) -> Result<()> {
    let bytes = set.bytes()?;
    let open = || Ok(BufReader::new(Joined::new(std::sync::Arc::clone(&bytes))));
    match set.format {
        Format::Zip => extract_zip(open, target, budget),
        Format::Tar => extract_tar(open()?, target, budget, false),
        Format::Tgz => extract_tar(open()?, target, budget, true),
        Format::Unity => extract_unitypackage(open()?, target, budget),
        Format::SevenZ => archives::extract_7z(bytes, target, budget),
        // The RAR library reads volumes from their files; pieces are joined into one first.
        Format::Rar if set.pieces() => {
            let joined = target.with_file_name(format!("{}.joined.rar", set.stem));
            let res = io::copy(&mut open()?, &mut File::create(&joined)?)
                .map_err(anyhow::Error::from)
                .and_then(|_| archives::extract_rar(&joined, target, budget));
            fs::remove_file(&joined).ok();
            res
        }
        Format::Rar => archives::extract_rar(&set.parts[0], target, budget),
    }
}

/// Unpack a zip on several threads, each with its own handle on the file: an all-in-one pack can hold 100,000
/// small files, and creating them one at a time is what takes the time.
fn extract_zip<R: Read + Seek + Send>(open: impl Fn() -> Result<R>, target: &Path, budget: &Budget) -> Result<()> {
    let open = || zip::ZipArchive::new(open()?).context("read zip");
    let first = open()?;
    let count = first.len();
    let threads = workers().min(count.div_ceil(256)).max(1);
    let mut archives = vec![first];
    for _ in 1..threads {
        archives.push(open()?);
    }
    let next = AtomicUsize::new(0);
    let failed = AtomicBool::new(false);
    std::thread::scope(|s| {
        let handles: Vec<_> = archives
            .into_iter()
            .map(|mut zip| {
                let (next, failed) = (&next, &failed);
                s.spawn(move || -> Result<()> {
                    crate::heavy::lower_priority();
                    while !failed.load(Ordering::Relaxed) {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= count {
                            break;
                        }
                        let res = (|| -> Result<()> {
                            let mut entry = zip.by_index(i)?;
                            if entry.is_dir() {
                                return Ok(());
                            }
                            // `enclosed_name` is the zip crate's zip-slip guard; `safe_rel_path` normalizes on top.
                            let Some(rel) = entry.enclosed_name().and_then(|p| safe_rel_path(&p.to_string_lossy()))
                            else {
                                return Ok(());
                            };
                            if !is_junk(&rel) {
                                budget.copy_limited(&mut entry, &target.join(rel))?;
                            }
                            Ok(())
                        })();
                        if res.is_err() {
                            failed.store(true, Ordering::Relaxed);
                            return res;
                        }
                    }
                    Ok(())
                })
            })
            .collect();
        handles
            .into_iter()
            .try_for_each(|h| h.join().unwrap_or_else(|_| Err(anyhow::anyhow!("an unpacking thread panicked"))))
    })
}

fn extract_tar(file: impl Read, target: &Path, budget: &Budget, gz: bool) -> Result<()> {
    let reader: Box<dyn Read> = if gz { Box::new(flate2::read::GzDecoder::new(file)) } else { Box::new(file) };
    let mut ar = tar::Archive::new(reader);
    for entry in ar.entries()? {
        let mut entry = entry?;
        if !entry.header().entry_type().is_file() {
            continue; // skips symlinks and hardlinks too
        }
        let Some(rel) = safe_rel_path(&entry.path()?.to_string_lossy()) else { continue };
        if is_junk(&rel) {
            continue;
        }
        budget.copy_limited(&mut entry, &target.join(rel))?;
    }
    Ok(())
}

/// A Unity package: a gzipped tar of `<guid>/asset` (the file), `<guid>/pathname` (where it lived, `Assets/…`),
/// `<guid>/asset.meta` and an optional `<guid>/preview.png`. Rebuilds the original tree, `.meta` files included.
fn extract_unitypackage(file: impl Read, target: &Path, budget: &Budget) -> Result<()> {
    let scratch = target.with_file_name(format!(
        "{}.unity-unpack",
        target.file_name().map_or_default(|n| n.to_string_lossy().into_owned())
    ));
    let result = (|| -> Result<usize> {
        extract_tar(file, &scratch, budget, true)?;
        let mut placed = 0;
        for entry in fs::read_dir(&scratch)? {
            let dir = entry?.path();
            let Ok(raw) = fs::read_to_string(dir.join("pathname")) else { continue };
            // The first line is the path; newer exporters append a second line ("00").
            let Some(rel) = raw.lines().next().and_then(|l| safe_rel_path(l.trim())) else { continue };
            for (from, to) in [("asset", rel.clone()), ("asset.meta", format!("{rel}.meta"))] {
                let src = dir.join(from);
                if src.is_file() {
                    let dest = target.join(&to);
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::rename(&src, &dest)?;
                    placed += usize::from(from == "asset");
                }
            }
        }
        Ok(placed)
    })();
    fs::remove_dir_all(&scratch).ok();
    let placed = result?;
    anyhow::ensure!(placed > 0, "not a Unity package (no asset/pathname entries)");
    Ok(())
}

/// Descend through wrapper folders: `upload/MyPack_v2/MyPack/...` -> `.../MyPack`. Returns the root and
/// the innermost wrapper name (a good default pack name).
pub(super) fn find_root(staging: &Path) -> Result<(PathBuf, Option<String>)> {
    let mut root = staging.to_path_buf();
    let mut name = None;
    loop {
        let entries: Vec<_> = fs::read_dir(&root)?
            .filter_map(Result::ok)
            .filter(|e| {
                let n = e.file_name().to_string_lossy().into_owned();
                !is_junk(&n) && !is_sidecar(&n)
            })
            .collect();
        match entries.as_slice() {
            [only] if only.file_type()?.is_dir() => {
                name = Some(only.file_name().to_string_lossy().into_owned());
                root = only.path();
            }
            _ => return Ok((root, name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingest::rules::unity_guid;

    #[test]
    fn rejects_traversal() {
        assert_eq!(safe_rel_path("../etc/passwd"), None);
        assert_eq!(safe_rel_path("a/../../b"), None);
        assert_eq!(safe_rel_path("/abs/file.png"), Some("abs/file.png".into()));
        assert_eq!(safe_rel_path("C:\\Users\\x\\file.png"), Some("Users/x/file.png".into()));
        assert_eq!(safe_rel_path("./ok/./file.wav"), Some("ok/file.wav".into()));
    }

    #[test]
    fn junk_filter() {
        assert!(is_junk("__MACOSX/a/._b.png"));
        assert!(is_sidecar("Assets/thing.png.meta"));
        assert_eq!(
            unity_guid("fileFormatVersion: 2\nguid: 7E1A2E907C7A4339be186c6b102fde95\nTextureImporter:\n"),
            Some("7E1A2E907C7A4339be186c6b102fde95")
        );
        assert_eq!(unity_guid("fileFormatVersion: 2\nguid: nothex\n"), None);
        assert!(!is_junk("Assets/thing.png.meta"));
        assert!(!is_junk("Assets/thing.png"));
    }

    /// A one-triangle glTF reading `bin` for its buffer and `png` for its color map.
    fn tri_gltf(bin: &str, png: &str) -> String {
        serde_json::json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "uri": bin, "byteLength": 36 }],
            "bufferViews": [{ "buffer": 0, "byteOffset": 0, "byteLength": 36 }],
            "accessors": [{ "bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3",
                            "min": [0, 0, 0], "max": [1, 1, 0] }],
            "images": [{ "uri": png }],
            "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0 } }] }],
            "nodes": [{ "mesh": 0 }], "scenes": [{ "nodes": [0] }], "scene": 0
        })
        .to_string()
    }

    #[test]
    fn gltf_buffers_packed_into_glb() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let write = |rel: &str, data: &[u8]| {
            let p = root.join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, data).unwrap();
        };
        let tri: Vec<u8> = [0f32, 0., 0., 1., 0., 0., 0., 1., 0.].iter().flat_map(|f| f.to_le_bytes()).collect();
        // Its own buffer: packed in, the texture stays beside it.
        write("crate/crate.gltf", tri_gltf("crate.bin", "tex/crate.png").as_bytes());
        write("crate/crate.bin", &tri);
        write("crate/tex/crate.png", b"\x89PNG");
        // Two glTFs sharing one buffer: left alone.
        write("pair/a.gltf", tri_gltf("shared.bin", "a.png").as_bytes());
        write("pair/b.gltf", tri_gltf("shared.bin", "b.png").as_bytes());
        write("pair/shared.bin", &tri);
        // A GLB of that name is there already: left alone.
        write("both/m.gltf", tri_gltf("m.bin", "m.png").as_bytes());
        write("both/m.bin", &tri);
        write("both/m.glb", b"glTF");
        // Known to the pack (or to Unity): left alone.
        write("kept/k.gltf", tri_gltf("k.bin", "k.png").as_bytes());
        write("kept/k.bin", &tri);

        let keep = HashSet::from(["kept/k.bin".to_owned()]);
        assert_eq!(pack_gltfs(root, &listed(root).unwrap(), &keep), 1);
        let mut left: Vec<String> = listed(root).unwrap().iter().map(|f| rel_of(root, f)).collect();
        left.sort();
        assert_eq!(
            left,
            [
                "both/m.bin",
                "both/m.glb",
                "both/m.gltf",
                "crate/crate.glb",
                "crate/tex/crate.png",
                "kept/k.bin",
                "kept/k.gltf",
                "pair/a.gltf",
                "pair/b.gltf",
                "pair/shared.bin",
            ]
        );
        // The GLB holds the geometry and still points at the texture beside it.
        let glb = fs::read(root.join("crate/crate.glb")).unwrap();
        let json_len = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
        let doc: serde_json::Value = serde_json::from_slice(&glb[20..20 + json_len]).unwrap();
        assert_eq!(doc["images"][0]["uri"], "tex/crate.png");
        assert!(doc["buffers"][0].get("uri").is_none());
        assert_eq!(&glb[20 + json_len + 4..20 + json_len + 8], b"BIN\0");
        assert_eq!(&glb[20 + json_len + 8..], &tri[..]);
    }

    /// A GLB's JSON and binary chunk.
    fn glb_chunks(glb: &[u8]) -> (serde_json::Value, Vec<u8>) {
        let json_len = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
        let doc = serde_json::from_slice(&glb[20..20 + json_len]).unwrap();
        (doc, glb.get(20 + json_len + 8..).unwrap_or_default().to_vec())
    }

    #[test]
    fn meshopt_gltf_packed_into_glb_stays_compressed() {
        // A meshopt-compressed triangle, as a .gltf reading its compressed buffer from m.bin.
        let tri: Vec<u8> = [0f32, 0., 0., 1., 0., 0., 0., 1., 0.].iter().flat_map(|f| f.to_le_bytes()).collect();
        let mut plain = serde_json::from_str::<serde_json::Value>(&tri_gltf("x", "x")).unwrap();
        plain.as_object_mut().unwrap().remove("images");
        plain["buffers"] = serde_json::json!([{ "byteLength": 36 }]);
        let mut json = serde_json::to_vec(&plain).unwrap();
        json.resize(json.len().div_ceil(4) * 4, b' ');
        let mut glb = b"glTF".to_vec();
        glb.extend(2u32.to_le_bytes());
        glb.extend(u32::try_from(20 + json.len() + 8 + 36).unwrap().to_le_bytes());
        glb.extend(u32::try_from(json.len()).unwrap().to_le_bytes());
        glb.extend(b"JSON");
        glb.extend(json);
        glb.extend(36u32.to_le_bytes());
        glb.extend(b"BIN\0");
        glb.extend(&tri);
        let compressed = polysmelt::gltf::compress(&glb).unwrap();
        let (mut doc, bin) = glb_chunks(&compressed);
        assert!(doc["extensionsUsed"].to_string().contains("meshopt"));
        doc["buffers"][0]["uri"] = "m.bin".into();
        doc["images"] = serde_json::json!([{ "uri": "m.png" }]);

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("m.gltf"), doc.to_string()).unwrap();
        fs::write(root.join("m.bin"), &bin).unwrap();
        fs::write(root.join("m.png"), b"\x89PNG").unwrap();
        assert_eq!(pack_gltfs(root, &listed(root).unwrap(), &HashSet::new()), 1);
        let mut left: Vec<String> = listed(root).unwrap().iter().map(|f| rel_of(root, f)).collect();
        left.sort();
        assert_eq!(left, ["m.glb", "m.png"]);
        let packed = fs::read(root.join("m.glb")).unwrap();
        let (doc, _) = glb_chunks(&packed);
        assert!(doc["extensionsUsed"].to_string().contains("meshopt"));
        assert_eq!(doc["images"][0]["uri"], "m.png");
        // It decodes to the triangle it started as.
        let (_, decoded) = glb_chunks(&polysmelt::gltf::decompress(&packed).unwrap());
        assert_eq!(&decoded[..36], &tri[..]);
    }
}
