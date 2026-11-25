//! Conversion rules (Settings → Conversion): files Hoard keeps in its own storage converted in place to the format
//! a rule names, such as every OBJ and FBX to GLB, turned as they're written to stand Y up and face +Z the way their
//! pack's models are shown. The converted file takes over the asset (same id, tags, collections and links), and
//! `asset_origin` keeps the original for [`Rules::keep_days`], during which the asset's page can put it back; after
//! that it's deleted. A model's textures aren't copied into it: the GLB points at the image files beside it in the
//! pack. WAV files become FLAC: the same samples in about half the space, with every other chunk of the WAV kept in
//! it (see `flac`), so the WAV isn't kept: it's made again from the FLAC, exactly, to download or put back, at any
//! time. A WAV with samples a FLAC doesn't hold stays a WAV, and the rules leave it alone. Watched folders are never
//! changed.
//!
//! A rule applies by itself to packs added from then on. Files already in the library are converted only when asked
//! (`POST /api/admin/conversions/run`), after Settings has shown what that would change
//! (`POST /api/admin/conversions/preview`).

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use hoard_core::{AssetKind, Facing};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::runtime::Handle;

use crate::extract::Origin;
use crate::i18n::tr;
use crate::{AppState, db, issues, jobs};

/// How long a converted file's original is kept, unless Settings says otherwise.
pub(crate) const DEFAULT_KEEP_DAYS: u32 = 30;
const MAX_KEEP_DAYS: u32 = 365;
/// How many of the files a change would convert are listed by name.
const SAMPLE: usize = 200;

/// The libraries that make what rules convert to, as `asset_origin.converter` names them. Keep in step with
/// Cargo.lock (a test checks).
const POLYSMELT: &str = "polysmelt 0.14.0";
const FLACENC: &str = "flacenc 0.5.1";

/// The formats rules convert to.
pub(crate) const TARGETS: &[&str] = &["glb", "flac"];

/// The formats a rule converting to `to` can convert from.
pub(crate) fn sources(to: &str) -> Vec<&'static str> {
    match to {
        "glb" => polysmelt::formats::EXTENSIONS
            .iter()
            .chain(crate::convert::CONVERTIBLE)
            .copied()
            .filter(|e| *e != "glb")
            .collect(),
        "flac" => vec!["wav"],
        _ => Vec::new(),
    }
}

/// Settings → Conversion: the rules, and how long originals are kept.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Rules {
    #[serde(default)]
    pub(crate) rules: Vec<Rule>,
    /// Days a converted file's original is kept, so the conversion can be undone.
    #[serde(default = "default_keep_days")]
    pub(crate) keep_days: u32,
}

impl Default for Rules {
    fn default() -> Self {
        Self { rules: Vec::new(), keep_days: DEFAULT_KEEP_DAYS }
    }
}

fn default_keep_days() -> u32 {
    DEFAULT_KEEP_DAYS
}

fn yes() -> bool {
    true
}

/// Files in any of the formats `from` become `to`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Rule {
    #[serde(default = "yes")]
    pub(crate) on: bool,
    /// Extensions, lowercase, without the dot.
    pub(crate) from: Vec<String>,
    pub(crate) to: String,
    /// Models are turned as they're written, the way their pack's models are shown, to stand Y up and face +Z.
    #[serde(default = "yes")]
    pub(crate) upright: bool,
}

impl Rules {
    /// Problems a person can fix, in their words.
    pub(crate) fn check(&self) -> Result<(), String> {
        if !(1..=MAX_KEEP_DAYS).contains(&self.keep_days) {
            return Err(tr!("error-conversion-keep-days", max = MAX_KEEP_DAYS));
        }
        let mut seen = HashSet::new();
        for rule in &self.rules {
            if !TARGETS.contains(&rule.to.as_str()) {
                return Err(tr!("error-conversion-target", format = rule.to.as_str()));
            }
            if rule.from.is_empty() {
                return Err(tr!("error-conversion-no-source"));
            }
            let sources = sources(&rule.to);
            for ext in &rule.from {
                if !sources.contains(&ext.as_str()) {
                    return Err(tr!("error-conversion-source", ext = ext.as_str(), format = rule.to.as_str()));
                }
                if rule.on && !seen.insert(ext.as_str()) {
                    return Err(tr!("error-conversion-twice", ext = ext.as_str()));
                }
            }
        }
        Ok(())
    }

    fn exts(&self) -> Vec<String> {
        self.rules.iter().filter(|r| r.on).flat_map(|r| r.from.iter().cloned()).collect()
    }
}

/// A file a rule's format matches.
#[derive(Debug, Clone)]
#[expect(clippy::struct_excessive_bools, reason = "independent facts about the file")]
struct Candidate {
    id: String,
    pack_id: String,
    pack_name: String,
    path: String,
    ext: String,
    watched: bool,
    /// Its Unity project knows it by its path (`.meta` files, prefabs): converting it would break the project.
    unity: bool,
    /// A scene or another file names it.
    used: bool,
    /// Blender's GLB of it, for the formats only Blender reads.
    blender_glb: Option<String>,
    ready: bool,
}

/// Why a file a rule matches is left as it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Skip {
    Watched,
    Unity,
    Used,
    /// The file it would become is in the pack already (the GLB that ships beside an OBJ).
    Beside,
    /// Blender hasn't made a GLB of it (yet).
    NoBlender,
}

impl Skip {
    fn key(self) -> &'static str {
        match self {
            Self::Watched => "watched",
            Self::Unity => "unity",
            Self::Used => "used",
            Self::Beside => "beside",
            Self::NoBlender => "blender",
        }
    }
}

/// A file to convert, and what it becomes.
#[derive(Debug, Clone)]
struct Todo {
    file: Candidate,
    rule: usize,
    to: String,
}

/// What the rules would do.
#[derive(Debug, Default)]
struct Plan {
    todo: Vec<Todo>,
    skipped: BTreeMap<Skip, usize>,
}

/// The files the rules match, in `pack` or everywhere: ones that haven't been converted before, put back by hand,
/// or archived.
fn candidates(conn: &Connection, exts: &[String], pack: Option<&str>) -> Result<Vec<Candidate>> {
    if exts.is_empty() {
        return Ok(Vec::new());
    }
    let marks = vec!["?"; exts.len()].join(",");
    let sql = format!(
        "SELECT a.id, a.pack_id, p.name, a.path, a.ext, COALESCE(l.mode, '') = 'watched',
                EXISTS (SELECT 1 FROM unity_guids u WHERE u.pack_id = a.pack_id AND u.path = a.path),
                EXISTS (SELECT 1 FROM asset_links k JOIN assets f ON f.id = k.from_id
                         WHERE k.to_id = a.id AND f.kind NOT IN ('model', 'animation')),
                (SELECT d.blob_hash FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'glb'),
                p.status = 'ready'
           FROM assets a JOIN packs p ON p.id = a.pack_id LEFT JOIN locations l ON l.id = p.location_id
          WHERE lower(a.ext) IN ({marks}) AND a.kind IN ('model', 'animation', 'audio')
            AND a.archived_at IS NULL AND p.archived_at IS NULL
            AND NOT EXISTS (SELECT 1 FROM asset_origin o WHERE o.asset_id = a.id)
            AND NOT EXISTS (SELECT 1 FROM kept_formats k WHERE k.asset_id = a.id)
            AND (?{n} IS NULL OR a.pack_id = ?{n})
          ORDER BY p.name COLLATE NOCASE, p.id, a.path",
        n = exts.len() + 1
    );
    let mut args: Vec<&dyn rusqlite::ToSql> = exts.iter().map(|e| e as &dyn rusqlite::ToSql).collect();
    args.push(&pack);
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(args.as_slice(), |r| {
            Ok(Candidate {
                id: r.get(0)?,
                pack_id: r.get(1)?,
                pack_name: r.get(2)?,
                path: r.get(3)?,
                ext: r.get::<_, String>(4)?.to_ascii_lowercase(),
                watched: r.get(5)?,
                unity: r.get(6)?,
                used: r.get(7)?,
                blender_glb: r.get(8)?,
                ready: r.get(9)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(rows)
}

/// The path a file takes in its new format: same folder and name.
fn renamed(path: &str, to: &str) -> String {
    let file_start = path.rfind('/').map_or(0, |i| i + 1);
    let stem = match path[file_start..].rfind('.') {
        Some(dot) => &path[..file_start + dot],
        None => path,
    };
    format!("{stem}.{to}")
}

/// What `rules` would convert in `pack` (or everywhere), and what they'd leave and why.
fn plan(conn: &Connection, rules: &Rules, pack: Option<&str>) -> Result<Plan> {
    let mut out = Plan::default();
    let mut taken: HashSet<(String, String)> = HashSet::new();
    let mut exists = conn.prepare_cached("SELECT EXISTS (SELECT 1 FROM assets WHERE pack_id = ?1 AND path = ?2)")?;
    for file in candidates(conn, &rules.exts(), pack)? {
        let Some((rule, r)) = rules.rules.iter().enumerate().find(|(_, r)| r.on && r.from.contains(&file.ext)) else {
            continue;
        };
        let to = renamed(&file.path, &r.to);
        let blender_only = r.to == "glb" && !crate::meshes::reads(&file.ext);
        let skip = if file.watched {
            Some(Skip::Watched)
        } else if file.unity {
            Some(Skip::Unity)
        } else if file.used {
            Some(Skip::Used)
        } else if blender_only && file.blender_glb.is_none() {
            Some(Skip::NoBlender)
        } else if taken.contains(&(file.pack_id.clone(), to.clone()))
            || exists.query_row(params![file.pack_id, to], |r| r.get::<_, bool>(0))?
        {
            Some(Skip::Beside)
        } else {
            None
        };
        if let Some(s) = skip {
            *out.skipped.entry(s).or_default() += 1;
            continue;
        }
        taken.insert((file.pack_id.clone(), to.clone()));
        out.todo.push(Todo { file, rule, to });
    }
    Ok(out)
}

/// `POST /api/admin/conversions/preview` with `{rules}` (Settings' rules, saved or not): how many files already in
/// the library they'd convert, by rule and in how many packs, the first [`SAMPLE`] of them by name, and how many
/// they'd leave as they are and why.
pub(crate) fn preview(conn: &Connection, rules: &Rules) -> Result<Value> {
    let plan = plan(conn, rules, None)?;
    let mut by_rule = vec![0usize; rules.rules.len()];
    for t in &plan.todo {
        by_rule[t.rule] += 1;
    }
    let packs: HashSet<&str> = plan.todo.iter().map(|t| t.file.pack_id.as_str()).collect();
    let sample: Vec<Value> = plan
        .todo
        .iter()
        .take(SAMPLE)
        .map(|t| {
            json!({ "id": t.file.id, "pack_id": t.file.pack_id, "pack": t.file.pack_name, "path": t.file.path,
                         "to": t.to })
        })
        .collect();
    let skipped: serde_json::Map<String, Value> =
        plan.skipped.iter().map(|(s, n)| (s.key().to_owned(), json!(n))).collect();
    let busy = plan.todo.iter().filter(|t| !t.file.ready).count();
    Ok(json!({
        "files": plan.todo.len(),
        "packs": packs.len(),
        "by_rule": by_rule,
        "busy": busy,
        "skipped": skipped,
        "sample": sample,
    }))
}

/// `POST /api/admin/conversions/run`: queue the conversion of files already in the library, by the saved rules, in
/// every pack that's ready. Packs still being read are converted when they're next added to or read again. Returns
/// how many packs were queued.
pub(crate) fn run_existing(state: &AppState, conn: &Connection) -> Result<usize> {
    let rules = state.settings.get().conversion_rules.clone();
    let plan = plan(conn, &rules, None)?;
    let packs: std::collections::BTreeSet<&str> =
        plan.todo.iter().filter(|t| t.file.ready).map(|t| t.file.pack_id.as_str()).collect();
    for pack in &packs {
        enqueue(conn, pack, Request::default())?;
    }
    if !packs.is_empty() {
        state.wake.notify_one();
    }
    Ok(packs.len())
}

/// Job payload for [`jobs::REFORMAT`].
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub(crate) struct Request {
    /// Set when queued while a pack is added: finish adding it afterwards (with or without the AI stage).
    #[serde(default)]
    pub(crate) then_llm: Option<bool>,
    /// The pack changed before this job (an earlier conversion), so it's read again even if nothing converts here.
    #[serde(default)]
    pub(crate) changed: bool,
}

fn enqueue(conn: &Connection, pack_id: &str, req: Request) -> Result<()> {
    let queued: bool = conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND pack_id = ?2 AND status = 'queued')",
        params![jobs::REFORMAT, pack_id],
        |r| r.get(0),
    )?;
    if !queued || req.then_llm.is_some() {
        db::enqueue(conn, jobs::REFORMAT, Some(pack_id), &serde_json::to_value(req)?)?;
    }
    Ok(())
}

/// After a pack is added (or another conversion of it): queue converting what the rules match in it. Returns
/// whether it did; the job then carries on adding the pack.
pub(crate) fn maybe_enqueue(state: &AppState, conn: &Connection, pack_id: &str, req: Request) -> Result<bool> {
    let rules = state.settings.get().conversion_rules.clone();
    if rules.exts().is_empty() || plan(conn, &rules, Some(pack_id))?.todo.is_empty() {
        return Ok(false);
    }
    enqueue(conn, pack_id, req)?;
    state.wake.notify_one();
    Ok(true)
}

/// Run a [`jobs::REFORMAT`] job: convert what the rules match in the pack, then read it again (and finish adding it,
/// when that's waiting on this).
pub(crate) async fn run_job(state: &AppState, pack_id: &str, req: Request) -> Result<()> {
    let st = std::sync::Arc::clone(state);
    let rt = Handle::current();
    let pid = pack_id.to_owned();
    let converted = crate::heavy::run("job", move || convert_pack(&st, &rt, &pid)).await.and_then(|r| r);
    let changed = req.changed || converted.as_ref().is_ok_and(|&n| n > 0);
    crate::extract::after_changes(state, pack_id, changed, req.then_llm).await?;
    converted.map(drop)
}

/// Convert what the rules match in a pack. Returns how many files were converted; one that fails is listed in
/// Settings → Issues and left as it was.
fn convert_pack(state: &AppState, rt: &Handle, pack_id: &str) -> Result<usize> {
    let rules = state.settings.get().conversion_rules.clone();
    let conn = state.pool.get()?;
    issues::clear(&conn, &[issues::REFORMAT], issues::At::pack(pack_id));
    let todo = plan(&conn, &rules, Some(pack_id))?.todo;
    if todo.is_empty() {
        return Ok(0);
    }
    let facing = db::pack_facing(&conn, pack_id)?;
    let images = pack_images(&conn, pack_id)?;
    std::fs::create_dir_all(state.cfg.tmp_dir())?;
    let work = tempfile::Builder::new().prefix("reformat-").tempdir_in(state.cfg.tmp_dir())?;
    let mut done = 0;
    for (n, t) in todo.iter().enumerate() {
        crate::activity::job_progress(jobs::REFORMAT, Some(pack_id), n, todo.len(), "files");
        let rule = &rules.rules[t.rule];
        let dir = work.path().join(n.to_string());
        let made = if rule.to == "flac" {
            wav_to_flac(state, rt, &conn, &t.file).map(|f| f.map(|b| (b, format!("encoded by {FLACENC}"))))
        } else {
            convert_one(state, rt, &conn, &t.file, rule, facing, &images, &dir).map(Some)
        };
        std::fs::remove_dir_all(&dir).ok();
        let (bytes, how) = match made {
            Ok(Some(b)) => b,
            Ok(None) => continue,
            Err(e) => {
                let why = format!("{e:#}");
                tracing::warn!(pack = pack_id, file = %t.file.path, error = %why, "conversion rule failed");
                issues::note(&conn, issues::REFORMAT, issues::At::file(pack_id, &t.file.path), &why);
                continue;
            }
        };
        let blob = rt.block_on(state.blobs.put_bytes(bytes, AssetKind::from_ext(&rule.to)))?;
        let origin = Origin {
            method: "rule",
            notes: vec![format!("{} → {}", t.file.ext.to_uppercase(), rule.to.to_uppercase())],
            converter: format!("hoard {} rule: {how}", env!("CARGO_PKG_VERSION")),
            ..Origin::default()
        };
        let mut conn = state.pool.get()?;
        let tx = conn.transaction()?;
        // A FLAC gives back the WAV it was made from.
        let keep = rule.to != "flac";
        if crate::extract::replace_asset(&tx, &t.file.id, &blob, &rule.to, &origin, keep)? {
            tx.prepare_cached("UPDATE asset_origin SET upright = ?2 WHERE asset_id = ?1")?
                .execute(params![t.file.id, rule.upright])?;
        }
        tx.commit()?;
        done += 1;
    }
    tracing::info!(pack = pack_id, converted = done, of = todo.len(), "conversion rules applied");
    Ok(done)
}

/// The pack's PNG and JPEG files, by the BLAKE3 hash of their bytes (which is their blob's name), with their paths:
/// a converted model's textures that are one of these point at it rather than being copied in.
fn pack_images(conn: &Connection, pack_id: &str) -> Result<Vec<([u8; 32], String)>> {
    let mut stmt = conn.prepare(
        "SELECT blob_hash, path FROM assets WHERE pack_id = ?1 AND lower(ext) IN ('png', 'jpg', 'jpeg')
          ORDER BY path",
    )?;
    let rows: Vec<(String, String)> =
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
    let mut out = Vec::with_capacity(rows.len());
    for (hash, path) in rows {
        if let Ok(h) = blake3::Hash::from_hex(&hash) {
            out.push((*h.as_bytes(), path));
        }
    }
    Ok(out)
}

/// One file in its new format: a GLB of the whole model (hierarchy, skins, animation, materials), turned by
/// `facing` when the rule says, whose textures are the pack's own image files where they can be.
#[expect(clippy::too_many_arguments, reason = "the pieces of one conversion")]
fn convert_one(
    state: &AppState,
    rt: &Handle,
    conn: &Connection,
    file: &Candidate,
    rule: &Rule,
    facing: Facing,
    images: &[([u8; 32], String)],
    dir: &Path,
) -> Result<(Vec<u8>, String)> {
    let (glb, how) = if let Some(hash) = &file.blender_glb
        && !crate::meshes::reads(&file.ext)
    {
        (rt.block_on(state.blobs.read_all(hash))?.to_vec(), "the GLB Blender made of it".to_owned())
    } else {
        let local = crate::materials::fetch_model(state, rt, conn, &file.id, dir)?;
        match file.ext.as_str() {
            // Its image files stay where they are, beside it.
            "gltf" => (crate::formats::build_glb(&local, false)?, "glTF packed as GLB".to_owned()),
            ext => (crate::meshes::glb(&local, ext, dir)?.0, format!("read and written by {POLYSMELT}")),
        }
    };
    let mut files = BTreeMap::new();
    for (hash, path) in images {
        files.entry(*hash).or_insert_with(|| crate::glbfiles::relative_uri(&file.path, path));
    }
    let facing = if rule.upright { facing } else { Facing::default() };
    let (glb, _) = crate::glbfiles::link(&glb, facing, &files).context("pointing the model at its textures")?;
    Ok((glb, how))
}

/// A WAV file as a FLAC that gives it back. One a FLAC can't (`None`) stays as it is, and the rules leave it alone
/// from then on.
fn wav_to_flac(state: &AppState, rt: &Handle, conn: &Connection, file: &Candidate) -> Result<Option<Vec<u8>>> {
    let hash: String = conn.query_row("SELECT blob_hash FROM assets WHERE id = ?1", [&file.id], |r| r.get(0))?;
    let wav = rt.block_on(state.blobs.read_all(&hash))?;
    match crate::flac::from_wav(&wav)? {
        Ok(flac) => Ok(Some(flac)),
        Err(why) => {
            tracing::info!(pack = %file.pack_id, file = %file.path, why, "kept as WAV");
            conn.execute("INSERT INTO kept_formats (asset_id) VALUES (?1) ON CONFLICT DO NOTHING", [&file.id])?;
            Ok(None)
        }
    }
}

/// An `asset_origin o` joined to its asset `a` whose original is in it: a WAV a rule made FLAC.
pub(crate) const REBUILDS: &str = "o.method = 'rule' AND lower(o.ext) = 'wav' AND lower(a.ext) = 'flac'";

/// The WAV a rule made a FLAC, made again from the FLAC, with its path: for an asset whose original isn't kept but
/// is in it. `None` for any other.
pub(crate) async fn rebuilt_original(state: &AppState, asset_id: &str) -> Result<Option<(Vec<u8>, String)>> {
    let id = asset_id.to_owned();
    let row = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            &format!(
                "SELECT a.blob_hash, o.path FROM asset_origin o JOIN assets a ON a.id = o.asset_id
                  WHERE o.asset_id = ?1 AND o.blob_hash IS NULL AND {REBUILDS}"
            ),
            [&id],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        )
        .optional()?)
    })
    .await?;
    let Some((hash, path)) = row else { return Ok(None) };
    let flac = state.blobs.read_all(&hash).await?;
    let wav = tokio::task::spawn_blocking(move || crate::flac::to_wav(&flac)).await??;
    Ok(wav.map(|w| (w, path)))
}

/// Keep the WAV a rule made a FLAC as the asset's original again, made from the FLAC, so it can be put back.
pub(crate) async fn restore_original(state: &AppState, asset_id: &str) -> Result<()> {
    let Some((wav, _)) = rebuilt_original(state, asset_id).await? else { return Ok(()) };
    let blob = state.blobs.put_bytes(wav, AssetKind::Audio).await?;
    let id = asset_id.to_owned();
    db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
            params![blob.hash, blob.size as i64, db::now()],
        )?;
        c.execute("UPDATE asset_origin SET blob_hash = ?2 WHERE asset_id = ?1", params![id, blob.hash])?;
        Ok(())
    })
    .await
}

/// `POST /api/assets/{id}/unconvert`: put back the file a rule converted, while its original is kept. The rules
/// leave it alone from then on. Returns its pack, to read again; `Err(Ok(msg))` is why it can't be.
pub(crate) fn put_back(conn: &mut Connection, asset_id: &str) -> Result<String, Result<String, anyhow::Error>> {
    let row: Option<(String, String, Option<String>, String, String)> = conn
        .query_row(
            "SELECT o.path, o.ext, o.blob_hash, o.method, a.pack_id FROM asset_origin o JOIN assets a ON a.id = o.asset_id
              WHERE o.asset_id = ?1",
            [asset_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )
        .optional()
        .map_err(|e| Err(e.into()))?;
    let Some((path, ext, Some(hash), method, pack_id)) = row.filter(|r| r.3 == "rule") else {
        return Err(Ok(tr!("error-conversion-cant-undo")));
    };
    let _ = method;
    let taken: bool = conn
        .query_row(
            "SELECT EXISTS (SELECT 1 FROM assets WHERE pack_id = ?1 AND path = ?2 AND id <> ?3)",
            params![pack_id, path, asset_id],
            |r| r.get(0),
        )
        .map_err(|e| Err(e.into()))?;
    if taken {
        return Err(Ok(tr!("error-conversion-path-taken", path = path.as_str())));
    }
    let mut undo = || -> Result<()> {
        let tx = conn.transaction()?;
        tx.execute(
            "UPDATE assets SET blob_hash = ?2, path = ?3, ext = ?4, kind = ?5, words = ?6, updated_at = ?7 WHERE id = ?1",
            params![
                asset_id,
                hash,
                path,
                ext,
                AssetKind::from_ext(&ext).as_str(),
                crate::tagger::fts_words(&path),
                db::now()
            ],
        )?;
        tx.execute("DELETE FROM asset_origin WHERE asset_id = ?1", [asset_id])?;
        tx.execute(
            "DELETE FROM derivatives WHERE asset_id = ?1 AND (kind IN ('source', 'views') OR kind LIKE 'as:%')",
            [asset_id],
        )?;
        tx.execute("INSERT INTO kept_formats (asset_id) VALUES (?1) ON CONFLICT DO NOTHING", [asset_id])?;
        db::enqueue(&tx, jobs::RETAG, Some(&pack_id), &json!({ "then_llm": false }))?;
        tx.commit()?;
        Ok(())
    };
    undo().map_err(Err)?;
    Ok(pack_id)
}

/// Delete the originals of files rules converted more than `keep_days` ago. Returns how many were let go.
pub(crate) fn expire(conn: &Connection, keep_days: u32) -> Result<usize> {
    let cutoff = time::OffsetDateTime::now_utc() - time::Duration::days(i64::from(keep_days));
    let cutoff = cutoff.format(&time::format_description::well_known::Rfc3339)?;
    // `db::now()` writes the same form, so the strings sort as the times do.
    Ok(conn.execute(
        "UPDATE asset_origin SET blob_hash = NULL WHERE method = 'rule' AND blob_hash IS NOT NULL AND converted_at < ?1",
        [cutoff],
    )?)
}

/// When the original of a file a rule converted is let go, while it's kept. A WAV made FLAC never is: it's made
/// again from the FLAC.
pub(crate) fn kept_until(origin: &hoard_core::AssetOrigin, keep_days: u32) -> Option<String> {
    use time::format_description::well_known::Rfc3339;
    if origin.method != "rule" || !origin.original_kept || origin.ext.eq_ignore_ascii_case("wav") {
        return None;
    }
    let at = time::OffsetDateTime::parse(&origin.converted_at, &Rfc3339).ok()?;
    (at + time::Duration::days(i64::from(keep_days))).format(&Rfc3339).ok()
}

/// Every few hours: let go of originals kept longer than Settings says, and free their storage.
pub(crate) fn spawn_expiry(state: &AppState) {
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(std::time::Duration::from_hours(6));
        loop {
            tick.tick().await;
            let days = state.settings.get().conversion_rules.keep_days;
            match db::call(&state.pool, move |c| expire(c, days)).await {
                Ok(0) => {}
                Ok(n) => match jobs::gc(&state).await {
                    Ok((blobs, bytes)) => {
                        tracing::info!(originals = n, blobs, bytes, "converted files' originals let go");
                    }
                    Err(e) => tracing::warn!(error = %format!("{e:#}"), "could not free converted files' originals"),
                },
                Err(e) => tracing::warn!(error = %format!("{e:#}"), "could not let go of converted files' originals"),
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(from: &[&str]) -> Rule {
        Rule { on: true, from: from.iter().map(|s| (*s).to_owned()).collect(), to: "glb".into(), upright: true }
    }

    /// A converted file's origin names the library version that made it, so that has to be the one built in.
    #[test]
    fn converter_names_match_the_libraries_built_in() {
        let lock = include_str!("../../../Cargo.lock");
        for named in [POLYSMELT, FLACENC] {
            let (name, version) = named.split_once(' ').unwrap();
            let entry = format!("name = \"{name}\"\nversion = \"{version}\"\n");
            assert!(lock.replace("\r\n", "\n").contains(&entry), "{named} isn't what Cargo.lock has: update it");
        }
    }

    #[test]
    fn rules_name_formats_that_convert_and_each_format_once() {
        let ok = Rules { rules: vec![rule(&["obj", "fbx"]), rule(&["blend"])], keep_days: 30 };
        assert!(ok.check().is_ok());
        let twice = Rules { rules: vec![rule(&["obj"]), rule(&["obj"])], keep_days: 30 };
        assert!(twice.check().unwrap_err().contains("obj"));
        let mut off = twice.clone();
        off.rules[1].on = false;
        assert!(off.check().is_ok(), "a rule that's off doesn't count");
        assert!(Rules { rules: vec![rule(&["png"])], keep_days: 30 }.check().is_err());
        assert!(Rules { rules: vec![rule(&["glb"])], keep_days: 30 }.check().is_err(), "nothing to do");
        assert!(Rules { rules: vec![rule(&[])], keep_days: 30 }.check().is_err());
        assert!(Rules { rules: vec![], keep_days: 0 }.check().is_err());
        let mut to = rule(&["obj"]);
        to.to = "fbx".into();
        assert!(Rules { rules: vec![to], keep_days: 30 }.check().is_err());
    }

    #[test]
    fn a_converted_file_keeps_its_folder_and_name() {
        assert_eq!(renamed("Models/OBJ format/tree.obj", "glb"), "Models/OBJ format/tree.glb");
        assert_eq!(renamed("v1.2/tree", "glb"), "v1.2/tree.glb");
        assert_eq!(renamed("tree.tar.fbx", "glb"), "tree.tar.glb");
    }

    fn library() -> Connection {
        let conn = db::test_library();
        conn.execute_batch(
            "INSERT INTO locations (id, name, mode, path, created_at) VALUES (1, 'w', 'watched', '/w', 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('p', 'Trees', 'ready', 'x'), ('q', 'Busy', 'ingesting', 'x');
             INSERT INTO packs (id, name, status, location_id, created_at) VALUES
               ('w', 'Watched', 'ready', 1, 'x');
             INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('oak', 'p', 'h', 'OBJ/oak.obj', 'oak', 'obj', 'model', '{}', 'x', 'x'),
               ('oak-fbx', 'p', 'h', 'OBJ/oak.fbx', 'oak', 'fbx', 'model', '{}', 'x', 'x'),
               ('elm', 'p', 'h', 'OBJ/elm.obj', 'elm', 'obj', 'model', '{}', 'x', 'x'),
               ('elm-glb', 'p', 'h', 'OBJ/elm.glb', 'elm', 'glb', 'model', '{}', 'x', 'x'),
               ('ash', 'p', 'h', 'ash.blend', 'ash', 'blend', 'model', '{}', 'x', 'x'),
               ('fir', 'p', 'h', 'fir.obj', 'fir', 'obj', 'model', '{}', 'x', 'x'),
               ('scene', 'p', 'h', 'level.tscn', 'level', 'tscn', 'scene', '{}', 'x', 'x'),
               ('box', 'q', 'h', 'box.obj', 'box', 'obj', 'model', '{}', 'x', 'x'),
               ('far', 'w', 'h', 'far.obj', 'far', 'obj', 'model', '{}', 'x', 'x');
             INSERT INTO asset_links (from_id, to_id, rel) VALUES ('scene', 'fir', 'guess');",
        )
        .unwrap();
        conn
    }

    #[test]
    fn a_preview_says_what_would_convert_and_what_is_left_and_why() {
        let conn = library();
        let rules = Rules { rules: vec![rule(&["obj", "fbx"]), rule(&["blend"])], keep_days: 30 };
        let p = preview(&conn, &rules).unwrap();
        let paths: Vec<&str> = p["sample"].as_array().unwrap().iter().map(|s| s["path"].as_str().unwrap()).collect();
        // The busy pack sorts first by name.
        assert_eq!(paths, ["box.obj", "OBJ/oak.fbx"]);
        assert_eq!(p["files"], 2);
        assert_eq!(p["packs"], 2);
        assert_eq!(p["busy"], 1);
        assert_eq!(p["by_rule"], json!([2, 0]));
        assert_eq!(p["sample"][1]["to"], "OBJ/oak.glb");
        // oak.obj loses to oak.fbx for oak.glb (a path sorts after), elm.glb ships already.
        assert_eq!(p["skipped"], json!({ "watched": 1, "used": 1, "beside": 2, "blender": 1 }));

        conn.execute("INSERT INTO kept_formats (asset_id) VALUES ('box')", []).unwrap();
        assert_eq!(preview(&conn, &rules).unwrap()["files"], 1, "one put back is left alone");
        assert_eq!(preview(&conn, &Rules::default()).unwrap()["files"], 0);
    }

    #[test]
    fn originals_are_let_go_after_the_days_settings_keeps_them() {
        let conn = library();
        conn.execute_batch(
            "INSERT INTO asset_origin (asset_id, path, ext, blob_hash, dependencies, method, converted_at) VALUES
               ('oak', 'OBJ/oak.obj', 'obj', 'h', '[]', 'rule', '2020-01-01T00:00:00Z'),
               ('elm', 'OBJ/elm.obj', 'obj', 'h', '[]', 'native', '2020-01-01T00:00:00Z'),
               ('fir', 'fir.obj', 'obj', 'h', '[]', 'rule', '2999-01-01T00:00:00Z');",
        )
        .unwrap();
        assert_eq!(expire(&conn, 30).unwrap(), 1);
        let kept: i64 =
            conn.query_row("SELECT COUNT(*) FROM asset_origin WHERE blob_hash IS NOT NULL", [], |r| r.get(0)).unwrap();
        assert_eq!(kept, 2, "Unreal originals and recent ones stay");
    }

    #[test]
    fn putting_a_file_back_restores_it_and_keeps_rules_off_it() {
        let mut conn = library();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('g', 1, 'x');
             UPDATE assets SET path = 'fir.glb', ext = 'glb', blob_hash = 'g' WHERE id = 'fir';
             INSERT INTO asset_origin (asset_id, path, ext, blob_hash, dependencies, method, converted_at, upright)
               VALUES ('fir', 'fir.obj', 'obj', 'h', '[]', 'rule', 'x', 1);",
        )
        .unwrap();
        assert_eq!(put_back(&mut conn, "fir").unwrap(), "p");
        let (path, ext, hash): (String, String, String) = conn
            .query_row("SELECT path, ext, blob_hash FROM assets WHERE id = 'fir'", [], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .unwrap();
        assert_eq!((path.as_str(), ext.as_str(), hash.as_str()), ("fir.obj", "obj", "h"));
        let kept: bool = conn
            .query_row("SELECT EXISTS (SELECT 1 FROM kept_formats WHERE asset_id = 'fir')", [], |r| r.get(0))
            .unwrap();
        assert!(kept);
        assert!(matches!(put_back(&mut conn, "fir"), Err(Ok(_))), "nothing left to undo");
        assert!(matches!(put_back(&mut conn, "oak"), Err(Ok(_))), "never converted");
    }
}
