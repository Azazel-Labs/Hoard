//! Pack ingest: staging dir -> unpacked files -> analyzed + tagged -> blobs + DB rows.
//!
//! Runs on a blocking thread (see `jobs`). Storage calls are async, so they go through `rt.block_on`.

mod rules;
mod store;
mod unpack;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result};
use hoard_core::{AssetKind, PackStatus, TagSource};
use rusqlite::params;
use tokio::runtime::Handle;

use crate::AppState;
use crate::analyze::{self, Analysis};
use crate::blobs::{BlobStore, StoredBlob};
use crate::db;
use crate::tagger::{self, TagSet};
use crate::{context, convert, issues, links};
use rules::{
    MapSets, UnityMeta, apply_pack_rules, could_be_texture, detect_pack_license, harvest_unity_meta, license_in,
};
use store::{Previews, insert_asset, insert_blob, save_readme, store_derivative, upload_all, write_tags};
use unpack::{Budget, expand_archives, find_root, kept, listed, pack_gltfs, rel_of};
pub(crate) use unpack::{pack_rels, safe_rel_path};

/// Threads for per-file work: every core, up to 8.
fn workers() -> usize {
    std::thread::available_parallelism().map_or(2, std::num::NonZero::get).min(8)
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
    file.rsplit_once('.').map_or_default(|(_, e)| e.to_lowercase())
}

fn stem_of(rel: &str) -> &str {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    file.rsplit_once('.').map_or(file, |(s, _)| s)
}

/// Hash + analyze + rule-tag one local file. `root` holds it at `rel`, with the pack's other files around it (or
/// those it depends on), where a model's textures are found.
pub(crate) fn analyze_one(local: PathBuf, root: &Path, rel: String, blob: Option<StoredBlob>) -> Result<Item> {
    let blob = match blob {
        Some(b) => b,
        None => BlobStore::hash_file(&local)?,
    };
    let ext = ext_of(&rel);
    let kind = analyze::detect_kind(&local, &ext);
    let analysis = analyze::analyze(&local, root, kind, &ext);
    let kind = analysis.kind.unwrap_or(kind);
    let mut tags = TagSet::default();
    tags.add(&format!("kind:{}", kind.as_str()), TagSource::Meta, 1.0);
    tagger::vocab_tags(&rel, kind, &mut tags);
    tagger::aside_tags(&rel, &mut tags);
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

/// Files analyzed per database write: enough to batch the writes, few enough that a batch (thumbnails included)
/// is a few MB and its transaction holds the write lock only briefly.
const BATCH: usize = 128;

/// Run `make` over `jobs` on low-priority worker threads and hand the results to `flush` a batch at a time, with
/// how many jobs are done so far. The queue between them is bounded, so however big the pack, only about two
/// batches are ever in memory. A job that fails is skipped, and its file and why come back in the list returned;
/// a `flush` that fails stops the lot.
fn pipeline<J: Sync>(
    pack_id: &str,
    jobs: &[J],
    rel: impl Fn(&J) -> &str + Sync,
    make: impl Fn(&J) -> Result<Item> + Sync,
    mut flush: impl FnMut(Vec<Item>, usize) -> Result<()>,
) -> Result<Vec<(String, String)>> {
    let failed: std::sync::Mutex<Vec<(String, String)>> = std::sync::Mutex::default();
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let (tx, rx) = std::sync::mpsc::sync_channel::<Item>(BATCH);
    std::thread::scope(|s| {
        for _ in 0..workers() {
            let tx = tx.clone();
            let (next, done, make, rel, failed) = (&next, &done, &make, &rel, &failed);
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
                        Err(e) => {
                            tracing::warn!(pack = pack_id, "skipped file: {e:#}");
                            let path = rel(job);
                            let msg = format!("{e:#}");
                            // The error names the file already, as its outermost context.
                            let msg = msg.strip_prefix(&format!("{path}: ")).map_or(msg.clone(), str::to_owned);
                            failed
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner)
                                .push((path.to_owned(), msg));
                        }
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
        anyhow::Ok(())
    })?;
    Ok(failed.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner))
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
    // Files left out last time are tried again (a watched pack's are, at least: an upload's went with it).
    issues::clear(&conn, &[issues::FILE, issues::UNPACK], issues::At::pack(pack_id));

    let (root, wrapper_name) = if let Some(root) = &watched {
        (root.clone(), None)
    } else {
        let budget = Budget::new(state.settings.get().max_expand_mb.saturating_mul(1 << 20), pack_id);
        expand_archives(&staging, &budget, 0)?;
        let (root, wrapper) = find_root(&staging)?;
        for (file, why) in budget.unpack_failures.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner) {
            let rel = rel_of(if file.starts_with(&root) { &root } else { &staging }, &file);
            issues::note(&conn, issues::UNPACK, issues::At::file(pack_id, &rel), &why);
        }
        (root, wrapper)
    };

    let mut files = listed(&root)?;
    let UnityMeta { guids, remaps } = harvest_unity_meta(&root, &files);
    let known = db::known_paths(&conn, pack_id)?;
    if watched.is_none() {
        let keep: HashSet<String> = known.iter().cloned().chain(guids.iter().map(|(path, _)| path.clone())).collect();
        let n = pack_gltfs(&root, &files, &keep);
        if n > 0 {
            tracing::info!(pack = pack_id, models = n, "glTF models packed into GLB");
            files = listed(&root)?;
        }
    }
    files.retain(|f| kept(&root, f));
    files.sort();
    let license = detect_pack_license(&root, &files);
    let readme = context::read_readmes(&root, &files);
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
    let facing = db::pack_facing(&conn, pack_id)?;
    let left_out = pipeline(
        pack_id,
        &jobs,
        |(_, rel)| rel,
        |(local, rel)| {
            crate::meshes::facing(facing, || analyze_one(local.clone(), &root, rel.clone(), None))
                .with_context(|| rel.clone())
        },
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
    for (rel, why) in &left_out {
        issues::note(&conn, issues::FILE, issues::At::file(pack_id, rel), why);
    }

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
    for (path, _) in &guids {
        tx.execute("DELETE FROM unity_remaps WHERE pack_id = ?1 AND path = ?2", params![pack_id, path])?;
    }
    for (path, slot, guid) in &remaps {
        tx.execute(
            "INSERT INTO unity_remaps (pack_id, path, slot, guid) VALUES (?1, ?2, ?3, ?4) ON CONFLICT DO NOTHING",
            params![pack_id, path, slot, guid],
        )?;
    }
    let n_links = links::link_pack(&tx, pack_id, &refs)?;
    let n_families = links::assign_families(&tx, pack_id)?;
    tx.execute("UPDATE packs SET files_done = files_total WHERE id = ?1", [pack_id])?;
    tx.commit()?;
    tracing::info!(pack = pack_id, links = n_links, families = n_families, "pack ingested");
    crate::materials::detect_logged(state, rt, &mut conn, pack_id);
    read_pack_files(state, rt, pack_id, auto_name, wrapper_name);

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
    let then = crate::reformat::Request { then_llm: Some(true), changed: false };
    if crate::reformat::maybe_enqueue(state, &conn, pack_id, then)? {
        // So does converting what the conversion rules match.
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
/// Fetch the files a model depends on (its MTL, its textures, its buffers) into `dir`, at their places in the pack.
fn fetch_dependencies(state: &AppState, rt: &Handle, id: &str, dir: &Path) -> Result<()> {
    let deps = {
        let conn = state.pool.get()?;
        let ids = db::dependency_closure(&conn, id, false)?;
        db::bundle_files(&conn, &ids)?
    };
    for f in deps.iter().filter(|f| f.id != id) {
        let Some(p) = safe_rel_path(&f.path) else { continue };
        let dest = dir.join(p);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        rt.block_on(state.blobs.download_to(&f.hash, &dest)).with_context(|| f.path.clone())?;
    }
    Ok(())
}

pub(crate) fn retag_pack(state: &AppState, rt: &Handle, pack_id: &str) -> Result<()> {
    struct Row {
        id: String,
        rel: String,
        hash: String,
        human: bool,
        kind: String,
    }
    let mut conn = state.pool.get()?;
    // A watched pack's files are read from its folder; they're never copied in. Models are drawn turned as the
    // pack's are, but for the ones a conversion rule already turned upright.
    let (watched, facing) =
        (crate::locations::watched_root(&conn, pack_id)?.is_some(), db::pack_facing(&conn, pack_id)?);
    let upright = db::upright_models(&conn, pack_id)?;
    let rows: Vec<Row> = {
        let mut stmt = conn.prepare(&format!(
            "SELECT id, path, blob_hash, human_edited, kind FROM assets a WHERE pack_id = ?1 AND NOT {}
                 ORDER BY path",
            crate::materials::IS_MATERIAL
        ))?;
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
    let unread = pipeline(
        pack_id,
        &rows,
        |r| &r.rel,
        |r| {
            // Each file is fetched into a folder of its own at its place in the pack; a model brings the files it
            // depends on (its MTL, its textures), so the thumbnail is drawn with them.
            let dir = work.join(&r.id);
            let rel = safe_rel_path(&r.rel).with_context(|| format!("unsafe path in pack: {}", r.rel))?;
            let local = dir.join(&rel);
            if matches!(ext_of(&r.rel).as_str(), "obj" | "fbx" | "gltf") {
                fetch_dependencies(state, rt, &r.id, &dir)?;
            }
            if let Some(parent) = local.parent() {
                fs::create_dir_all(parent)?;
            }
            rt.block_on(state.blobs.download_to(&r.hash, &local)).with_context(|| r.rel.clone())?;
            let blob = StoredBlob { hash: r.hash.clone(), size: fs::metadata(&local)?.len() };
            let facing = if upright.contains(&r.id) { hoard_core::Facing::default() } else { facing };
            let mut item = crate::meshes::facing(facing, || analyze_one(local, &dir, r.rel.clone(), Some(blob)))
                .with_context(|| r.rel.clone())?;
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
            let dirs: Vec<PathBuf> =
                batch.iter().filter_map(|it| it.asset_id.as_ref()).map(|id| work.join(id)).collect();
            let now = db::now();
            let tx = conn.transaction()?;
            for (item, Previews { thumb, views, source, preview, lite }) in batch.into_iter().zip(previews) {
                let id = item.asset_id.clone().unwrap_or_default();
                if let Some(t) = &thumb {
                    insert_blob(&tx, t, &now)?;
                }
                tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND source IN ('rule', 'meta')", [&id])?;
                tx.execute(
                    // A thumbnail a browser drew stays: it shows the model as the viewer does. A .blend's thumbnail is
                    // its own preview, or one drawn from its conversion (below); analysis that can't draw one
                    // otherwise keeps the one it has.
                    "UPDATE assets SET meta = ?2, kind = ?3,
                            thumb_hash = CASE WHEN thumb_source IS NOT NULL THEN thumb_hash WHEN ext = 'blend' THEN ?4
                                              ELSE COALESCE(?4, thumb_hash) END,
                            words = ?5,
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
                // A converted model's rig comes from its conversion, and stays.
                if item.analysis.rig.is_some() || !convert::CONVERTIBLE.contains(&item.ext.as_str()) {
                    crate::animations::store(&tx, &id, item.analysis.rig.as_ref())?;
                }
                store_derivative(&tx, &id, "views", views.as_ref(), &now)?;
                store_derivative(&tx, &id, "source", source.as_ref(), &now)?;
                store_derivative(&tx, &id, "preview", preview.as_ref(), &now)?;
                store_derivative(&tx, &id, crate::previews::LITE, lite.as_ref(), &now)?;
                if !item.refs.is_empty() {
                    refs.insert(id, item.refs);
                }
            }
            tx.execute("UPDATE packs SET files_done = ?2 WHERE id = ?1", params![pack_id, done as i64])?;
            tx.commit()?;
            for d in dirs {
                fs::remove_dir_all(d).ok();
            }
            Ok(())
        },
    )?;

    issues::read_again(&conn, pack_id, &unread)?;
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
    convert::thumbs_from_conversions(state, rt, pack_id)?;
    crate::animations::rigs_from_conversions(state, rt, pack_id)?;
    {
        let _in_runtime = rt.enter();
        crate::motion::kick(state);
    }
    crate::materials::detect_logged(state, rt, &mut conn, pack_id);
    read_pack_files(state, rt, pack_id, false, None);
    Ok(())
}

/// Name, tag and describe the pack from the store's files in it (a Megascans `<id>.json`). A file that can't be
/// read doesn't fail the pack.
fn read_pack_files(state: &AppState, rt: &Handle, pack_id: &str, auto_name: bool, folder: Option<String>) {
    match rt.block_on(crate::sidecar::apply(state, pack_id, auto_name, folder)) {
        Ok(a) if a.files > 0 => {
            tracing::info!(pack = pack_id, files = a.files, renamed = ?a.renamed, "read the pack's store files");
        }
        Ok(_) => {}
        Err(e) => {
            tracing::warn!(pack = pack_id, error = %format!("{e:#}"), "could not read the pack's store files");
            issues::note_in(state, issues::STORE_FILES, issues::At::pack(pack_id), &format!("{e:#}"));
            return;
        }
    }
    issues::clear_in(state, &[issues::STORE_FILES], issues::At::pack(pack_id));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext() {
        assert_eq!(ext_of("a/b/Thing.PNG"), "png");
    }
}
