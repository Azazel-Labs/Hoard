//! Pack ingest: staging dir -> unpacked files -> analyzed + tagged -> blobs + DB rows.
//!
//! Runs on a blocking thread (see `jobs`). Storage calls are async, so they go through `rt.block_on`.

use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use anyhow::{Context, Result};
use futures_util::{StreamExt, TryStreamExt, stream};
use hoard_core::{AssetKind, PackStatus, TagSource};
use rusqlite::params;
use tokio::runtime::Handle;

use crate::AppState;
use crate::analyze::{self, Analysis};
use crate::blobs::{BlobStore, StoredBlob};
use crate::db;
use crate::tagger::{self, TagSet};
use crate::{context, convert, links};

const MAX_ARCHIVE_DEPTH: u32 = 3;
const LICENSE_SCAN_BYTES: u64 = 256 << 10;

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

fn is_junk(rel: &str) -> bool {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    rel.split('/').any(|c| matches!(c, "__MACOSX" | ".git" | ".svn" | ".hg"))
        || matches!(file, ".DS_Store" | "Thumbs.db" | "desktop.ini")
        || file.starts_with("._")
}

/// Engine sidecars that only make sense next to their engine's project (Unity `.meta`, Godot `.import`). Kept
/// through unpacking so Unity GUIDs can be read from them, then left out of the pack.
fn is_sidecar(rel: &str) -> bool {
    Path::new(rel).extension().is_some_and(|e| e.eq_ignore_ascii_case("meta") || e.eq_ignore_ascii_case("import"))
}

/// The GUID in a Unity `.meta` file (`guid: 0123…`, 32 hex digits).
pub(crate) fn unity_guid(text: &str) -> Option<&str> {
    let line = text.lines().take(8).find_map(|l| l.trim().strip_prefix("guid:"))?.trim();
    (line.len() == 32 && line.bytes().all(|b| b.is_ascii_hexdigit())).then_some(line)
}

/// GUIDs of the pack's files, from the `.meta` files beside them: `(path relative to root, guid)`.
fn harvest_unity_guids(root: &Path, files: &[PathBuf]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for f in files {
        let rel = rel_of(root, f);
        let Some(target) = rel.strip_suffix(".meta").filter(|_| is_sidecar(&rel)) else { continue };
        let Ok(text) = fs::read_to_string(f) else { continue };
        if let Some(g) = unity_guid(&text) {
            out.push((target.to_owned(), g.to_ascii_lowercase()));
        }
    }
    out
}

fn archive_stem(name: &str) -> Option<&str> {
    let lower = name.to_lowercase();
    for ext in [".tar.gz", ".tgz", ".tar", ".zip", ".unitypackage"] {
        if lower.ends_with(ext) {
            return Some(&name[..name.len() - ext.len()]);
        }
    }
    None
}

/// What an upload may still unpack to, shared by the unpacking threads, plus a running file count for progress.
#[derive(Debug)]
struct Budget {
    bytes_left: AtomicU64,
    files: AtomicUsize,
    pack_id: String,
}

impl Budget {
    fn new(bytes: u64, pack_id: &str) -> Self {
        Self { bytes_left: AtomicU64::new(bytes), files: AtomicUsize::new(0), pack_id: pack_id.to_owned() }
    }

    fn reserve(&self, n: u64) -> Result<()> {
        self.bytes_left
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |left| left.checked_sub(n))
            .map(drop)
            .map_err(|_| anyhow::anyhow!("archive expands past the size limit (raise Largest pack in Settings)"))
    }

    /// Stream `src` to `dest`, a buffer at a time, charging every byte before it's written: a zip bomb stops at
    /// the limit, however many threads are unpacking.
    fn copy_limited(&self, src: &mut impl Read, dest: &Path) -> Result<()> {
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
        let n = self.files.fetch_add(1, Ordering::Relaxed) + 1;
        if n.is_multiple_of(250) {
            crate::activity::job_note(crate::jobs::INGEST, Some(&self.pack_id), &format!("unpacked {n} files"));
        }
        Ok(())
    }
}

/// Threads for per-file work: every core, up to 8.
fn workers() -> usize {
    std::thread::available_parallelism().map_or(2, std::num::NonZero::get).min(8)
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
fn listed(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    walk_files(root, &mut files)?;
    files.retain(|f| !is_junk(&rel_of(root, f)));
    Ok(files)
}

/// The relative paths a pack folder would give its assets: what a rescan compares the pack against.
pub(crate) fn pack_rels(root: &Path) -> Result<HashSet<String>> {
    Ok(listed(root)?.iter().map(|f| rel_of(root, f)).filter(|rel| !is_sidecar(rel)).collect())
}

fn rel_of(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn expand_archives(dir: &Path, budget: &Budget, depth: u32) -> Result<()> {
    if depth > MAX_ARCHIVE_DEPTH {
        return Ok(());
    }
    let mut files = Vec::new();
    walk_files(dir, &mut files)?;
    for file in files {
        let Some(name) = file.file_name().map(|n| n.to_string_lossy().into_owned()) else { continue };
        let Some(stem) = archive_stem(&name) else { continue };
        let mut target = file.with_file_name(stem);
        while target.exists() {
            target.set_file_name(format!("{}_x", target.file_name().unwrap_or_default().to_string_lossy()));
        }
        let ext = Path::new(&name).extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
        let res = match ext.as_str() {
            "zip" => extract_zip(&file, &target, budget),
            "gz" | "tgz" => extract_tar(&file, &target, budget, true),
            "unitypackage" => extract_unitypackage(&file, &target, budget),
            _ => extract_tar(&file, &target, budget, false),
        };
        match res {
            Ok(()) => {
                fs::remove_file(&file)?;
                expand_archives(&target, budget, depth + 1)?;
            }
            // A corrupt nested archive is kept as a plain file instead of failing the whole pack,
            // but limit violations must abort.
            Err(e) if e.to_string().contains("size limit") => return Err(e),
            Err(e) => tracing::warn!(archive = %file.display(), error = %e, "could not unpack; keeping as a file"),
        }
    }
    Ok(())
}

/// Unpack a zip on several threads, each with its own handle on the file: an all-in-one pack can hold 100,000
/// small files, and creating them one at a time is what takes the time.
fn extract_zip(path: &Path, target: &Path, budget: &Budget) -> Result<()> {
    let open = || zip::ZipArchive::new(BufReader::new(File::open(path)?)).context("read zip");
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

fn extract_tar(path: &Path, target: &Path, budget: &Budget, gz: bool) -> Result<()> {
    let file = BufReader::new(File::open(path)?);
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
fn extract_unitypackage(path: &Path, target: &Path, budget: &Budget) -> Result<()> {
    let scratch = target.with_file_name(format!(
        "{}.unity-unpack",
        target.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
    ));
    let result = (|| -> Result<usize> {
        extract_tar(path, &scratch, budget, true)?;
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
fn find_root(staging: &Path) -> Result<(PathBuf, Option<String>)> {
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

/// One file moving through analysis.
#[derive(Debug)]
pub(crate) struct Item {
    pub(crate) asset_id: Option<String>,
    pub(crate) rel: String,
    pub(crate) ext: String,
    pub(crate) kind: AssetKind,
    pub(crate) local: PathBuf,
    pub(crate) blob: StoredBlob,
    pub(crate) analysis: Analysis,
    pub(crate) tags: TagSet,
    pub(crate) category: String,
    /// References to other files in the pack, resolved into links after insert.
    pub(crate) refs: Vec<links::Ref>,
}

pub(crate) fn ext_of(rel: &str) -> String {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    file.rsplit_once('.').map(|(_, e)| e.to_lowercase()).unwrap_or_default()
}

fn stem_of(rel: &str) -> &str {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

/// Hash + analyze + rule-tag one local file.
pub(crate) fn analyze_one(local: PathBuf, rel: String, blob: Option<StoredBlob>) -> Result<Item> {
    let blob = match blob {
        Some(b) => b,
        None => BlobStore::hash_file(&local)?,
    };
    let ext = ext_of(&rel);
    let kind = analyze::detect_kind(&local, &ext);
    let analysis = analyze::analyze(&local, kind, &ext);
    let kind = analysis.kind.unwrap_or(kind);
    let mut tags = TagSet::default();
    tags.add(&format!("kind:{}", kind.as_str()), TagSource::Meta, 1.0);
    tagger::vocab_tags(&rel, kind, &mut tags);
    tagger::meta_tags(kind, &ext, &analysis.meta, &mut tags);
    let refs = links::extract_refs(&local, &ext);
    Ok(Item { asset_id: None, rel, ext, kind, local, blob, analysis, tags, category: String::new(), refs })
}

/// A converted asset keeps its package's facts: class and engine (and so its `unreal`/`ue:*` tags), and the
/// package's dependencies as links.
fn carry_origin(item: &mut Item, o: &crate::extract::OriginFacts) {
    let m = &mut item.analysis.meta;
    m.insert("converted_from".into(), serde_json::json!(o.path));
    if let Some(class) = &o.class {
        m.entry("ue_class").or_insert_with(|| serde_json::json!(class));
    }
    if let Some(engine) = &o.engine {
        m.entry("ue_engine").or_insert_with(|| serde_json::json!(engine));
    }
    if !o.dependencies.is_empty() {
        m.entry("ue_dependencies").or_insert_with(|| serde_json::json!(o.dependencies));
        item.refs
            .extend(o.dependencies.iter().map(|d| links::Ref { target: d.clone(), rel: hoard_core::LinkRel::Library }));
    }
    tagger::meta_tags(item.kind, &item.ext, &item.analysis.meta, &mut item.tags);
}

/// Which textures form PBR map sets (`Rock_Albedo` + `Rock_Normal` + ...), worked out from paths alone before
/// any file is read, so a pack can be tagged a batch at a time and never held in memory whole.
pub(crate) struct MapSets(HashMap<String, (tagger::MapHit, bool)>);

/// Whether a file might be a texture, judging by its path: an image, or an Unreal package (whose class is only
/// known once it's read).
fn could_be_texture(rel: &str) -> bool {
    let ext = ext_of(rel);
    ext == "uasset" || AssetKind::from_ext(&ext) == AssetKind::Texture
}

impl MapSets {
    pub(crate) fn new<'a>(texture_paths: impl IntoIterator<Item = &'a str>) -> Self {
        let mut groups: HashMap<(&str, String), Vec<(&str, tagger::MapHit)>> = HashMap::new();
        for rel in texture_paths {
            if let Some(hit) = tagger::map_suffix(stem_of(rel)) {
                let dir = rel.rsplit_once('/').map_or("", |(d, _)| d);
                groups.entry((dir, hit.base.clone())).or_default().push((rel, hit));
            }
        }
        let mut out = HashMap::new();
        for members in groups.into_values() {
            let distinct: std::collections::BTreeSet<&str> = members.iter().map(|(_, h)| h.map).collect();
            let is_set = distinct.len() >= 2;
            for (rel, hit) in members {
                out.insert(rel.to_owned(), (hit, is_set));
            }
        }
        Self(out)
    }
}

/// Rules that need the rest of the pack: PBR map sets, the pack's license, then the category.
pub(crate) fn apply_pack_rules(it: &mut Item, sets: &MapSets, license: Option<&str>) {
    if it.kind == AssetKind::Texture
        && let Some((hit, is_set)) = sets.0.get(&it.rel)
    {
        let is_set = *is_set;
        // Unreal's T_Name_N convention makes one-letter suffixes trustworthy on their own.
        let unreal_texture = it.ext == "uasset" && stem_of(&it.rel).starts_with("T_");
        if is_set || hit.strong || unreal_texture {
            it.tags.add(&format!("map:{}", hit.map), TagSource::Rule, if is_set { 0.95 } else { 0.8 });
            if is_set {
                it.tags.add(&format!("set:{}", hit.base), TagSource::Rule, 0.9);
                it.tags.add("pbr", TagSource::Rule, 0.85);
            }
            // "Rock_Metallic.png" is a map, not a metal rock: drop vocab tags that only came from the suffix.
            let dir = it.rel.rsplit_once('/').map_or("", |(d, _)| d);
            let mut from_suffix = TagSet::default();
            tagger::vocab_tags(&hit.suffix, it.kind, &mut from_suffix);
            let mut from_base = TagSet::default();
            tagger::vocab_tags(&format!("{dir}/{}", hit.base), it.kind, &mut from_base);
            for s in from_suffix.into_vec() {
                if !from_base.has(&s.tag) {
                    it.tags.remove(&s.tag);
                }
            }
        }
    }
    let data_map = [
        "map:normal",
        "map:roughness",
        "map:metallic",
        "map:ao",
        "map:height",
        "map:orm",
        "map:specular",
        "map:opacity",
    ]
    .iter()
    .any(|m| it.tags.has(m));
    if data_map {
        it.tags.remove_prefix("color:");
    }
    if let Some(l) = license {
        it.tags.add(&format!("license:{l}"), TagSource::Rule, 0.9);
    }
    it.category = tagger::categorize(it.kind, &it.tags);
}

fn detect_pack_license(root: &Path, files: &[PathBuf]) -> Option<&'static str> {
    license_in(files.iter().filter(|f| tagger::is_license_file(&rel_of(root, f))))
}

/// The first license recognised in these files (only their first few KB are read).
fn license_in<'a>(files: impl IntoIterator<Item = &'a PathBuf>) -> Option<&'static str> {
    files.into_iter().find_map(|f| {
        let mut text = String::new();
        File::open(f).ok()?.take(LICENSE_SCAN_BYTES).read_to_string(&mut text).ok()?;
        tagger::detect_license(&text)
    })
}

/// Files analyzed per database write: enough to batch the writes, few enough that a batch (thumbnails included)
/// is a few MB and its transaction holds the write lock only briefly.
const BATCH: usize = 128;

/// Run `make` over `jobs` on low-priority worker threads and hand the results to `flush` a batch at a time, with
/// how many jobs are done so far. The queue between them is bounded, so however big the pack, only about two
/// batches are ever in memory. A job that fails is logged and skipped; a `flush` that fails stops the lot.
fn pipeline<J: Sync>(
    pack_id: &str,
    jobs: &[J],
    make: impl Fn(&J) -> Result<Item> + Sync,
    mut flush: impl FnMut(Vec<Item>, usize) -> Result<()>,
) -> Result<()> {
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let (tx, rx) = std::sync::mpsc::sync_channel::<Item>(BATCH);
    std::thread::scope(|s| {
        for _ in 0..workers() {
            let tx = tx.clone();
            let (next, done, make) = (&next, &done, &make);
            s.spawn(move || {
                crate::heavy::lower_priority();
                while let Some(job) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let item = make(job);
                    let n = done.fetch_add(1, Ordering::Relaxed) + 1;
                    for kind in [crate::jobs::INGEST, crate::jobs::RETAG] {
                        crate::activity::job_progress(kind, Some(pack_id), n, jobs.len(), "files");
                    }
                    match item {
                        // The receiver is gone only when a flush failed: stop.
                        Ok(item) => {
                            if tx.send(item).is_err() {
                                break;
                            }
                        }
                        Err(e) => tracing::warn!(pack = pack_id, "skipped file: {e:#}"),
                    }
                }
            });
        }
        drop(tx);
        // Moved in here so an early return drops it, which stops the workers before the scope waits for them.
        let rx = rx;
        let mut batch = Vec::with_capacity(BATCH);
        for item in &rx {
            batch.push(item);
            if batch.len() == BATCH {
                flush(std::mem::replace(&mut batch, Vec::with_capacity(BATCH)), done.load(Ordering::Relaxed))?;
            }
        }
        if !batch.is_empty() {
            flush(batch, done.load(Ordering::Relaxed))?;
        }
        Ok(())
    })
}

/// Previews stored alongside an asset: its thumbnail and (for models, when vision tagging is on) a multi-view sheet.
struct Previews {
    thumb: Option<StoredBlob>,
    views: Option<StoredBlob>,
    /// A source file recovered from inside the asset (Unreal `SoundWave` → WAV).
    source: Option<StoredBlob>,
    /// A playable rendering (MIDI, tracker modules → MP3).
    preview: Option<StoredBlob>,
}

/// Upload a batch's originals and previews to the blob store. The preview bytes are moved out of the items, not
/// copied. Returns the previews by item index.
/// A watched folder's files stay where they are (`store_originals` false); only their previews are stored.
fn upload_all(state: &AppState, rt: &Handle, items: &mut [Item], store_originals: bool) -> Result<Vec<Previews>> {
    let blobs = &state.blobs;
    rt.block_on(async {
        if store_originals {
            stream::iter(items.iter().map(Ok::<_, anyhow::Error>))
                .try_for_each_concurrent(8, |it| async move { blobs.put_file(&it.local, &it.blob, it.kind).await })
                .await?;
        }
        stream::iter(items.iter_mut())
            .map(|it| async move {
                let a = &mut it.analysis;
                // Thumbnails, views and previews go to the cache store; a recovered source file is content.
                Ok::<_, anyhow::Error>(Previews {
                    thumb: cached(blobs, a.thumb_png.take()).await?,
                    views: cached(blobs, a.views_png.take()).await?,
                    source: match a.source.take() {
                        Some(s) => Some(blobs.put_bytes(s.bytes, AssetKind::from_ext(s.ext)).await?),
                        None => None,
                    },
                    preview: cached(blobs, a.preview_mp3.take()).await?,
                })
            })
            .buffered(8)
            .try_collect::<Vec<_>>()
            .await
    })
}

async fn cached(blobs: &BlobStore, bytes: Option<Vec<u8>>) -> Result<Option<StoredBlob>> {
    Ok(match bytes {
        Some(b) => Some(blobs.put_cache_bytes(b).await?),
        None => None,
    })
}

fn insert_blob(tx: &rusqlite::Transaction<'_>, blob: &StoredBlob, now: &str) -> Result<()> {
    tx.execute(
        "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
        params![blob.hash, blob.size as i64, now],
    )?;
    Ok(())
}

/// Keep README text as pack context; a store link inside it fills in the store if none was set.
fn save_readme(tx: &rusqlite::Transaction<'_>, pack_id: &str, text: &str) -> Result<()> {
    let (store, url) = context::store_link(text).map_or((None, None), |(s, u)| (Some(s), Some(u)));
    tx.execute(
        "INSERT INTO pack_context (pack_id, readme, store, store_url) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT (pack_id) DO UPDATE SET readme = excluded.readme,
             store = COALESCE(store, excluded.store), store_url = COALESCE(store_url, excluded.store_url)",
        params![pack_id, text, store, url],
    )?;
    Ok(())
}

fn store_derivative(
    tx: &rusqlite::Transaction<'_>,
    asset_id: &str,
    kind: &str,
    blob: Option<&StoredBlob>,
    now: &str,
) -> Result<()> {
    if let Some(v) = blob {
        insert_blob(tx, v, now)?;
        tx.execute(
            "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, ?4, ?2, ?3)
             ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash, created_at = excluded.created_at",
            params![asset_id, v.hash, now, kind],
        )?;
    }
    Ok(())
}

fn write_tags(tx: &rusqlite::Transaction<'_>, asset_id: &str, tags: TagSet) -> Result<()> {
    for s in tags.into_vec() {
        db::upsert_tag(tx, asset_id, &s.tag, s.source, s.confidence)?;
    }
    Ok(())
}

/// Insert one analyzed file. A path the pack already has (written by an earlier run that was interrupted) is left
/// as it is, so an ingest can be run again and carry on.
fn insert_asset(
    tx: &rusqlite::Transaction<'_>,
    pack_id: &str,
    item: Item,
    previews: Previews,
    now: &str,
    refs: &mut HashMap<String, Vec<links::Ref>>,
) -> Result<()> {
    let Previews { thumb, views, source, preview } = previews;
    insert_blob(tx, &item.blob, now)?;
    if let Some(t) = &thumb {
        insert_blob(tx, t, now)?;
    }
    let id = uuid::Uuid::now_v7().to_string();
    let added = tx.execute(
        "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, category, meta, words, thumb_hash,
                             created_at, updated_at, source_description)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12, ?13)
         ON CONFLICT (pack_id, path) DO NOTHING",
        params![
            id,
            pack_id,
            item.blob.hash,
            item.rel,
            item.analysis.title.clone().unwrap_or_else(|| tagger::display_name(&tagger::name_stem(&item.rel))),
            item.ext,
            item.kind.as_str(),
            item.category,
            serde_json::Value::Object(item.analysis.meta).to_string(),
            tagger::fts_words(&item.rel),
            thumb.as_ref().map(|t| t.hash.clone()),
            now,
            item.analysis.description,
        ],
    )?;
    if added == 0 {
        return Ok(());
    }
    write_tags(tx, &id, item.tags)?;
    db::reindex_asset(tx, &id)?;
    store_derivative(tx, &id, "views", views.as_ref(), now)?;
    store_derivative(tx, &id, "source", source.as_ref(), now)?;
    store_derivative(tx, &id, "preview", preview.as_ref(), now)?;
    if !item.refs.is_empty() {
        refs.insert(id, item.refs);
    }
    Ok(())
}

/// Unpack an upload and add its files to the pack. Files are analyzed, stored and written a batch at a time, so
/// memory stays flat however many files a pack has, and the database is never locked for more than a moment.
///
/// A pack in a watched folder is read where it sits instead: nothing is unpacked, copied or cleaned up. Files the
/// pack already has (from an interrupted run, or a watched pack being rescanned) are skipped.
pub(crate) fn ingest_pack(state: &AppState, rt: &Handle, pack_id: &str, auto_name: bool) -> Result<()> {
    let mut conn = state.pool.get()?;
    let watched = crate::locations::watched_root(&conn, pack_id)?;
    let staging = state.cfg.staging_dir().join(pack_id);
    match &watched {
        Some(root) => anyhow::ensure!(root.is_dir(), "{} can't be read", root.display()),
        None => anyhow::ensure!(staging.is_dir(), "staging folder for pack {pack_id} is gone"),
    }
    db::set_pack_status(&conn, pack_id, PackStatus::Ingesting, None)?;
    // The file count isn't known until it's unpacked: no total shows as busy rather than stuck at 0 of 1.
    conn.execute("UPDATE packs SET files_total = 0, files_done = 0 WHERE id = ?1", [pack_id])?;

    let (root, wrapper_name) = if let Some(root) = &watched {
        (root.clone(), None)
    } else {
        let budget = Budget::new(state.settings.get().max_expand_mb.saturating_mul(1 << 20), pack_id);
        expand_archives(&staging, &budget, 0)?;
        find_root(&staging)?
    };

    let mut files = listed(&root)?;
    let guids = harvest_unity_guids(&root, &files);
    files.retain(|f| !is_sidecar(&rel_of(&root, f)));
    files.sort();
    let license = detect_pack_license(&root, &files);
    let readme = context::read_readmes(&root, &files);
    let known = db::known_paths(&conn, pack_id)?;
    let first = known.is_empty();
    files.retain(|f| !known.contains(&rel_of(&root, f)));
    drop(known);
    conn.execute(
        "UPDATE packs SET files_total = ?2, files_done = 0 WHERE id = ?1",
        params![pack_id, files.len() as i64],
    )?;
    if auto_name && let Some(name) = &wrapper_name {
        conn.execute("UPDATE packs SET name = ?2 WHERE id = ?1", params![pack_id, name])?;
    }

    let jobs: Vec<(PathBuf, String)> = files
        .into_iter()
        .map(|f| {
            let rel = rel_of(&root, &f);
            (f, rel)
        })
        .collect();
    let sets = MapSets::new(jobs.iter().map(|(_, rel)| rel.as_str()).filter(|rel| could_be_texture(rel)));

    // All that's kept of a file once its batch is written: its references, to link up at the end.
    let mut refs = HashMap::new();
    pipeline(
        pack_id,
        &jobs,
        |(local, rel)| analyze_one(local.clone(), rel.clone(), None).with_context(|| rel.clone()),
        |mut batch, done| {
            for it in &mut batch {
                apply_pack_rules(it, &sets, license);
            }
            let previews = upload_all(state, rt, &mut batch, watched.is_none())?;
            let now = db::now();
            let tx = conn.transaction()?;
            for (item, p) in batch.into_iter().zip(previews) {
                insert_asset(&tx, pack_id, item, p, &now, &mut refs)?;
            }
            tx.execute("UPDATE packs SET files_done = ?2 WHERE id = ?1", params![pack_id, done as i64])?;
            tx.commit()?;
            Ok(())
        },
    )?;
    drop(jobs);

    let tx = conn.transaction()?;
    if let Some(l) = license {
        tx.execute("UPDATE packs SET license = ?2 WHERE id = ?1", params![pack_id, l])?;
    }
    if let Some(text) = &readme {
        save_readme(&tx, pack_id, text)?;
    }
    for (path, guid) in &guids {
        tx.execute(
            "INSERT INTO unity_guids (pack_id, guid, path) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
            params![pack_id, guid, path],
        )?;
    }
    let n_links = links::link_pack(&tx, pack_id, &refs)?;
    let n_families = links::assign_families(&tx, pack_id)?;
    tx.execute("UPDATE packs SET files_done = files_total WHERE id = ?1", [pack_id])?;
    tx.commit()?;
    tracing::info!(pack = pack_id, links = n_links, families = n_families, "pack ingested");

    if watched.is_none() {
        fs::remove_dir_all(&staging).ok();
    } else if !first {
        // A rescan that found a few new files: they're tagged by the rules; the pack's conversions and AI tagging
        // aren't run over the whole pack again.
        db::set_pack_status(&conn, pack_id, PackStatus::Ready, None)?;
        return Ok(());
    }
    convert::maybe_enqueue(state, &conn, pack_id)?;
    if crate::extract::maybe_enqueue(state, &conn, pack_id)? {
        // The conversion job retags what it converted and carries on from there.
        db::set_pack_status(&conn, pack_id, PackStatus::Analyzing, None)?;
        return Ok(());
    }
    finish_rules_stage(state, &conn, pack_id, true)
}

/// After rule tagging: either hand off to the LLM or mark the pack ready.
pub(crate) fn finish_rules_stage(
    state: &AppState,
    conn: &rusqlite::Connection,
    pack_id: &str,
    want_llm: bool,
) -> Result<()> {
    if want_llm && state.settings.get().llm_enabled() {
        db::set_pack_status(conn, pack_id, PackStatus::Analyzing, None)?;
        db::enqueue(conn, "llm_tag", Some(pack_id), &serde_json::json!({}))?;
        state.wake.notify_one();
    } else {
        db::set_pack_status(conn, pack_id, PackStatus::Ready, None)?;
    }
    Ok(())
}

/// Re-run the offline tagger over an existing pack (after vocabulary improvements, say).
/// Replaces `rule`/`meta` tags; leaves `human` and `llm` tags alone. Like ingest, it works a batch at a time:
/// each file is fetched, analyzed, written and deleted again before the queue moves far past it.
pub(crate) fn retag_pack(state: &AppState, rt: &Handle, pack_id: &str) -> Result<()> {
    struct Row {
        id: String,
        rel: String,
        hash: String,
        human: bool,
        kind: String,
    }
    let mut conn = state.pool.get()?;
    // A watched pack's files are read from its folder; they're never copied in.
    let watched = crate::locations::watched_root(&conn, pack_id)?.is_some();
    let rows: Vec<Row> = {
        let mut stmt = conn
            .prepare("SELECT id, path, blob_hash, human_edited, kind FROM assets WHERE pack_id = ?1 ORDER BY path")?;
        stmt.query_map([pack_id], |r| {
            Ok(Row { id: r.get(0)?, rel: r.get(1)?, hash: r.get(2)?, human: r.get::<_, i64>(3)? != 0, kind: r.get(4)? })
        })?
        .collect::<rusqlite::Result<_>>()?
    };
    let work = state.cfg.tmp_dir().join(format!("retag-{pack_id}"));
    fs::create_dir_all(&work)?;
    conn.execute(
        "UPDATE packs SET files_total = ?2, files_done = 0 WHERE id = ?1",
        params![pack_id, rows.len() as i64],
    )?;

    let origins = crate::extract::pack_origins(&conn, pack_id)?;
    // The license comes first, from just the license files: every file's tags carry it.
    let license = {
        let mut texts = Vec::new();
        for (n, r) in rows.iter().filter(|r| tagger::is_license_file(&r.rel)).enumerate() {
            let local = work.join(format!("license-{n}.txt"));
            rt.block_on(state.blobs.download_to(&r.hash, &local))?;
            texts.push(local);
        }
        let found = license_in(&texts);
        for t in &texts {
            fs::remove_file(t).ok();
        }
        found
    };
    let sets = MapSets::new(
        rows.iter()
            .filter(|r| r.kind == AssetKind::Texture.as_str() || could_be_texture(&r.rel))
            .map(|r| r.rel.as_str()),
    );
    let human: HashSet<&str> = rows.iter().filter(|r| r.human).map(|r| r.id.as_str()).collect();

    let mut refs = HashMap::new();
    pipeline(
        pack_id,
        &rows,
        |r| {
            let local = work.join(format!("{}.{}", r.id, ext_of(&r.rel)));
            rt.block_on(state.blobs.download_to(&r.hash, &local)).with_context(|| r.rel.clone())?;
            let blob = StoredBlob { hash: r.hash.clone(), size: fs::metadata(&local)?.len() };
            let mut item = analyze_one(local, r.rel.clone(), Some(blob)).with_context(|| r.rel.clone())?;
            if let Some(o) = origins.get(&r.id) {
                carry_origin(&mut item, o);
            }
            item.asset_id = Some(r.id.clone());
            Ok(item)
        },
        |mut batch, done| {
            for it in &mut batch {
                apply_pack_rules(it, &sets, license);
            }
            let previews = upload_all(state, rt, &mut batch, !watched)?;
            let locals: Vec<PathBuf> = batch.iter().map(|it| it.local.clone()).collect();
            let now = db::now();
            let tx = conn.transaction()?;
            for (item, Previews { thumb, views, source, preview }) in batch.into_iter().zip(previews) {
                let id = item.asset_id.clone().unwrap_or_default();
                if let Some(t) = &thumb {
                    insert_blob(&tx, t, &now)?;
                }
                tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND source IN ('rule', 'meta')", [&id])?;
                tx.execute(
                    "UPDATE assets SET meta = ?2, kind = ?3, thumb_hash = COALESCE(?4, thumb_hash), words = ?5,
                            category = CASE WHEN ?6 THEN category ELSE ?7 END, updated_at = ?8
                     WHERE id = ?1",
                    params![
                        id,
                        serde_json::Value::Object(item.analysis.meta).to_string(),
                        item.kind.as_str(),
                        thumb.as_ref().map(|t| t.hash.clone()),
                        tagger::fts_words(&item.rel),
                        human.contains(id.as_str()),
                        item.category,
                        now,
                    ],
                )?;
                write_tags(&tx, &id, item.tags)?;
                db::reindex_asset(&tx, &id)?;
                store_derivative(&tx, &id, "views", views.as_ref(), &now)?;
                store_derivative(&tx, &id, "source", source.as_ref(), &now)?;
                store_derivative(&tx, &id, "preview", preview.as_ref(), &now)?;
                if !item.refs.is_empty() {
                    refs.insert(id, item.refs);
                }
            }
            tx.execute("UPDATE packs SET files_done = ?2 WHERE id = ?1", params![pack_id, done as i64])?;
            tx.commit()?;
            for l in locals {
                fs::remove_file(l).ok();
            }
            Ok(())
        },
    )?;

    let tx = conn.transaction()?;
    tx.execute("DELETE FROM asset_links WHERE from_id IN (SELECT id FROM assets WHERE pack_id = ?1)", [pack_id])?;
    links::link_pack(&tx, pack_id, &refs)?;
    links::assign_families(&tx, pack_id)?;
    if let Some(l) = license {
        tx.execute("UPDATE packs SET license = ?2 WHERE id = ?1", params![pack_id, l])?;
    }
    tx.execute("UPDATE packs SET files_done = files_total WHERE id = ?1", [pack_id])?;
    tx.commit()?;
    fs::remove_dir_all(&work).ok();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
