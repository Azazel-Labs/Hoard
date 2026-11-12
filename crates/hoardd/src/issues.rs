//! Settings → Issues: what went wrong opening or processing the library's files, in one place.
//!
//! Most of it is read from where it already lives: failed jobs and packs, files whose analysis failed (their
//! `meta.analysis_error`), Unreal packages that didn't convert, damaged stored files and watched packs whose folder
//! is gone. What isn't kept anywhere else (an archive that wouldn't unpack, a file left out of its pack, a model
//! Blender couldn't convert) is recorded in the `issues` table by the code that hit it, and cleared when that code
//! runs again for the same pack or folder.

use std::collections::HashSet;

use anyhow::Result;
use axum::Json;
use axum::extract::State;
use rusqlite::{Connection, OptionalExtension, params};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::error::{ApiResult, AppError};
use crate::{AppState, db, jobs};

/// An archive in an upload that couldn't be unpacked; it's kept in the pack as a plain file.
pub(crate) const UNPACK: &str = "unpack";
/// A file that couldn't be read or stored, so it was left out of its pack.
pub(crate) const FILE: &str = "file";
/// A pack's store files (a Megascans JSON) that couldn't be read.
pub(crate) const STORE_FILES: &str = "store_files";
/// A pack whose materials couldn't be worked out.
pub(crate) const MATERIALS: &str = "materials";
/// A material that was found but whose thumbnail couldn't be drawn.
pub(crate) const MATERIAL_THUMB: &str = "material_thumb";
/// A model Blender couldn't convert, so it has no GLB to show in the browser.
pub(crate) const CONVERT: &str = "convert";
/// A watched folder that can't be read, or that came up empty.
pub(crate) const FOLDER: &str = "folder";

/// Where an issue happened: a pack (and a file in it), or a library location (and a folder in it).
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct At<'a> {
    pub(crate) pack: Option<&'a str>,
    pub(crate) location: Option<i64>,
    /// A file's path in its pack, a folder, or nothing when it's about the pack or location as a whole.
    pub(crate) subject: &'a str,
}

impl<'a> At<'a> {
    pub(crate) fn pack(pack: &'a str) -> Self {
        Self { pack: Some(pack), ..Self::default() }
    }
    pub(crate) fn file(pack: &'a str, path: &'a str) -> Self {
        Self { pack: Some(pack), location: None, subject: path }
    }
    pub(crate) fn location(location: i64) -> Self {
        Self { location: Some(location), ..Self::default() }
    }
}

/// Record an issue. The same issue seen again (same kind, place and file) is counted, not listed twice.
pub(crate) fn record(conn: &Connection, kind: &str, at: At<'_>, message: &str) -> Result<()> {
    let key = format!("{kind}\u{1f}{}\u{1f}{}\u{1f}{}", at.pack.unwrap_or(""), at.location.unwrap_or(0), at.subject);
    let now = db::now();
    conn.execute(
        "INSERT INTO issues (key, kind, pack_id, location_id, subject, message, first_at, last_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
         ON CONFLICT (key) DO UPDATE SET message = excluded.message, count = count + 1, last_at = excluded.last_at",
        params![key, kind, at.pack, at.location, at.subject, message, now],
    )?;
    Ok(())
}

/// [`record`], for code whose own work mustn't fail because the issue couldn't be written down.
pub(crate) fn note(conn: &Connection, kind: &str, at: At<'_>, message: &str) {
    if let Err(e) = record(conn, kind, at, message) {
        tracing::warn!(kind, error = %format!("{e:#}"), "couldn't record an issue");
    }
}

/// [`note`] with a connection of its own, from the pool.
pub(crate) fn note_in(state: &AppState, kind: &str, at: At<'_>, message: &str) {
    match state.pool.get() {
        Ok(conn) => note(&conn, kind, at, message),
        Err(e) => tracing::warn!(kind, error = %e, "couldn't record an issue"),
    }
}

/// After a pack's files were read again: those that failed are listed, the rest no longer are. Files that were left
/// out when it was added aren't read again, so they stay listed.
pub(crate) fn read_again(conn: &Connection, pack: &str, failed: &[(String, String)]) -> Result<()> {
    conn.execute(
        "DELETE FROM issues WHERE kind = ?1 AND pack_id = ?2 AND subject IN (SELECT path FROM assets WHERE pack_id = ?2)",
        params![FILE, pack],
    )?;
    for (path, why) in failed {
        note(conn, FILE, At::file(pack, path), why);
    }
    Ok(())
}

/// Forget a pack's or location's issues of these kinds, before the code that records them runs over it again.
pub(crate) fn clear(conn: &Connection, kinds: &[&str], at: At<'_>) {
    for kind in kinds {
        let res = conn.execute(
            "DELETE FROM issues WHERE kind = ?1 AND pack_id IS ?2 AND location_id IS ?3",
            params![kind, at.pack, at.location],
        );
        if let Err(e) = res {
            tracing::warn!(kind, error = %e, "couldn't clear old issues");
        }
    }
}

/// [`clear`] with a connection of its own, from the pool.
pub(crate) fn clear_in(state: &AppState, kinds: &[&str], at: At<'_>) {
    match state.pool.get() {
        Ok(conn) => clear(&conn, kinds, at),
        Err(e) => tracing::warn!(error = %e, "couldn't clear old issues"),
    }
}

/// How many of a group are listed; the rest are counted.
const SHOWN: i64 = 200;

/// One kind of issue, as Settings → Issues shows it. `retry` is what the retry button says, when there is one.
struct Group {
    kind: &'static str,
    title: &'static str,
    blurb: &'static str,
    retry: Option<&'static str>,
    dismiss: bool,
}

const GROUPS: &[Group] = &[
    Group {
        kind: "pack",
        title: "Packs that couldn’t be added",
        blurb: "Delete the pack and add it again, or open it to see what it has.",
        retry: None,
        dismiss: false,
    },
    Group {
        kind: "damaged",
        title: "Stored files that are damaged or gone",
        blurb: "Found by Check storage. Restore them from a backup or add their pack again, then run Check storage.",
        retry: None,
        dismiss: false,
    },
    Group {
        kind: FOLDER,
        title: "Watched folders that can’t be read",
        blurb: "Their packs are left as they are until the folder can be read again.",
        retry: Some("Scan again"),
        dismiss: true,
    },
    Group {
        kind: "missing",
        title: "Watched packs whose folder is gone",
        blurb: "Each pack is kept for 3 days in case its folder is only offline, then removed.",
        retry: Some("Scan again"),
        dismiss: false,
    },
    Group {
        kind: "job",
        title: "Background jobs that failed",
        blurb: "Retrying runs the job again from the start.",
        retry: Some("Retry"),
        dismiss: true,
    },
    Group {
        kind: FILE,
        title: "Files that couldn’t be opened",
        blurb: "Hoard couldn’t open or store them. A watched pack tries them again at its next scan; a file already in the library kept what it had before.",
        retry: Some("Scan again"),
        dismiss: true,
    },
    Group {
        kind: UNPACK,
        title: "Archives that couldn’t be unpacked",
        blurb: "Each is kept in its pack as a plain file, so nothing inside it is in the library.",
        retry: None,
        dismiss: true,
    },
    Group {
        kind: "read",
        title: "Files Hoard couldn’t make sense of",
        blurb: "They’re in the library, but without the facts, tags or thumbnail that reading them gives.",
        retry: Some("Re-analyze pack"),
        dismiss: false,
    },
    Group {
        kind: "render",
        title: "Models the thumbnail renderer failed on",
        blurb: "They have no thumbnail. Opening one in the viewer may still work, and can save one.",
        retry: Some("Re-analyze pack"),
        dismiss: false,
    },
    Group {
        kind: CONVERT,
        title: "Models Blender couldn’t convert",
        blurb: "They have no GLB copy, so they can’t be shown in 3D.",
        retry: Some("Convert again"),
        dismiss: true,
    },
    Group {
        kind: "unreal",
        title: "Unreal packages that couldn’t be converted",
        blurb: "The pack’s Unreal page has the rest of its packages.",
        retry: None,
        dismiss: false,
    },
    Group {
        kind: MATERIALS,
        title: "Packs whose materials couldn’t be worked out",
        blurb: "Their textures are all there; they just aren’t grouped into materials.",
        retry: Some("Look again"),
        dismiss: true,
    },
    Group {
        kind: MATERIAL_THUMB,
        title: "Materials without a thumbnail",
        blurb: "The material was found, but its preview couldn’t be drawn.",
        retry: Some("Look again"),
        dismiss: true,
    },
    Group {
        kind: STORE_FILES,
        title: "Store files that couldn’t be read",
        blurb: "Files from the store a pack came from, such as a Megascans JSON, that name and tag it.",
        retry: Some("Read again"),
        dismiss: true,
    },
];

/// An item of a group, before it's turned into JSON.
#[derive(Default)]
struct Item {
    id: String,
    title: String,
    message: String,
    pack_id: Option<String>,
    pack_name: Option<String>,
    asset_id: Option<String>,
    path: Option<String>,
    location_id: Option<i64>,
    location_name: Option<String>,
    count: i64,
    at: Option<String>,
    retry: bool,
}

impl Item {
    fn json(self) -> Value {
        json!({
            "id": self.id, "title": self.title, "message": self.message, "pack_id": self.pack_id,
            "pack_name": self.pack_name, "asset_id": self.asset_id, "path": self.path,
            "location_id": self.location_id, "location_name": self.location_name, "count": self.count.max(1),
            "at": self.at, "retry": self.retry,
        })
    }
}

/// A group's items (at most [`SHOWN`]) and how many there are in all.
fn items(c: &Connection, kind: &str) -> Result<(Vec<Item>, i64)> {
    // Each query lists what it shows; the count is the same query without the limit.
    let (sql, map): (&str, fn(&rusqlite::Row<'_>) -> rusqlite::Result<Item>) = match kind {
        "pack" => {
            ("SELECT id, name, error, created_at FROM packs WHERE status = 'failed' ORDER BY created_at DESC", |r| {
                Ok(Item {
                    id: format!("pack:{}", r.get::<_, String>(0)?),
                    pack_id: r.get(0)?,
                    title: r.get(1)?,
                    pack_name: r.get(1)?,
                    message: r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                    at: r.get(3)?,
                    ..Item::default()
                })
            })
        }
        "damaged" => (
            "SELECT b.hash, b.status, a.id, a.pack_id, p.name, a.path, b.verified_at
               FROM blobs b JOIN assets a ON a.blob_hash = b.hash JOIN packs p ON p.id = a.pack_id
              WHERE b.status <> 'ok' ORDER BY p.name, a.path",
            |r| {
                let status: String = r.get(1)?;
                Ok(Item {
                    id: format!("blob:{}:{}", r.get::<_, String>(0)?, r.get::<_, String>(2)?),
                    asset_id: r.get(2)?,
                    pack_id: r.get(3)?,
                    pack_name: r.get(4)?,
                    path: r.get(5)?,
                    title: r.get(5)?,
                    message: if status == "missing" {
                        "The stored file is gone.".to_owned()
                    } else {
                        "The stored file doesn’t match what was stored: it’s damaged.".to_owned()
                    },
                    at: r.get(6)?,
                    ..Item::default()
                })
            },
        ),
        "missing" => (
            "SELECT p.id, p.name, p.location_id, l.name, p.folder, p.missing_since
               FROM packs p LEFT JOIN locations l ON l.id = p.location_id
              WHERE p.missing_since IS NOT NULL ORDER BY p.missing_since",
            |r| {
                let folder: Option<String> = r.get(4)?;
                Ok(Item {
                    id: format!("missing:{}", r.get::<_, String>(0)?),
                    pack_id: r.get(0)?,
                    title: r.get(1)?,
                    pack_name: r.get(1)?,
                    location_id: r.get(2)?,
                    location_name: r.get(3)?,
                    message: format!("The folder “{}” wasn’t found.", folder.unwrap_or_default()),
                    at: r.get(5)?,
                    retry: true,
                    ..Item::default()
                })
            },
        ),
        "job" => (
            "SELECT j.id, j.kind, j.pack_id, p.name, j.error, j.updated_at, j.attempts
               FROM jobs j LEFT JOIN packs p ON p.id = j.pack_id
              WHERE j.status = 'failed' AND j.kind NOT IN ('ingest', 'retag') ORDER BY j.id DESC",
            |r| {
                let kind: String = r.get(1)?;
                let pack: Option<String> = r.get(3)?;
                Ok(Item {
                    id: format!("job:{}", r.get::<_, i64>(0)?),
                    title: jobs::job_label(&kind, pack.as_deref().unwrap_or("a pack")),
                    pack_id: r.get(2)?,
                    pack_name: pack,
                    message: r.get::<_, Option<String>>(4)?.unwrap_or_default(),
                    at: r.get(5)?,
                    count: r.get(6)?,
                    retry: true,
                    ..Item::default()
                })
            },
        ),
        "read" | "render" => (
            if kind == "read" {
                "SELECT a.id, a.pack_id, p.name, a.path, json_extract(a.meta, '$.analysis_error'), a.updated_at
                   FROM assets a JOIN packs p ON p.id = a.pack_id
                  WHERE json_extract(a.meta, '$.analysis_error') IS NOT NULL ORDER BY a.updated_at DESC"
            } else {
                "SELECT a.id, a.pack_id, p.name, a.path, json_extract(a.meta, '$.thumb_error'), a.updated_at
                   FROM assets a JOIN packs p ON p.id = a.pack_id
                  WHERE json_extract(a.meta, '$.thumb_error') IS NOT NULL ORDER BY a.updated_at DESC"
            },
            |r| {
                Ok(Item {
                    id: format!("asset:{}", r.get::<_, String>(0)?),
                    asset_id: r.get(0)?,
                    pack_id: r.get(1)?,
                    pack_name: r.get(2)?,
                    path: r.get(3)?,
                    title: r.get(3)?,
                    message: r.get::<_, Option<String>>(4)?.unwrap_or_default(),
                    at: r.get(5)?,
                    retry: true,
                    ..Item::default()
                })
            },
        ),
        "unreal" => (
            "SELECT u.asset_id, a.pack_id, p.name, a.path, u.reason, u.updated_at
               FROM unreal_status u JOIN assets a ON a.id = u.asset_id JOIN packs p ON p.id = a.pack_id
              WHERE u.status = 'failed' ORDER BY u.updated_at DESC",
            |r| {
                Ok(Item {
                    id: format!("unreal:{}", r.get::<_, String>(0)?),
                    asset_id: r.get(0)?,
                    pack_id: r.get(1)?,
                    pack_name: r.get(2)?,
                    path: r.get(3)?,
                    title: r.get(3)?,
                    message: r.get::<_, Option<String>>(4)?.unwrap_or_default(),
                    at: r.get(5)?,
                    ..Item::default()
                })
            },
        ),
        _ => return recorded(c, kind),
    };
    let total: i64 = c.query_row(&format!("SELECT COUNT(*) FROM ({sql})"), [], |r| r.get(0))?;
    let mut stmt = c.prepare(&format!("{sql} LIMIT {SHOWN}"))?;
    let rows = stmt.query_map([], map)?.collect::<rusqlite::Result<Vec<_>>>()?;
    Ok((rows, total))
}

/// A group from the `issues` table.
fn recorded(c: &Connection, kind: &str) -> Result<(Vec<Item>, i64)> {
    let total: i64 = c.query_row("SELECT COUNT(*) FROM issues WHERE kind = ?1", [kind], |r| r.get(0))?;
    let mut stmt = c.prepare(
        "SELECT i.id, i.pack_id, p.name, p.location_id, i.location_id, l.name, i.subject, i.message, i.count,
                i.last_at, (SELECT a.id FROM assets a WHERE a.pack_id = i.pack_id AND a.path = i.subject)
           FROM issues i LEFT JOIN packs p ON p.id = i.pack_id LEFT JOIN locations l ON l.id = i.location_id
          WHERE i.kind = ?1 ORDER BY i.last_at DESC LIMIT ?2",
    )?;
    let rows = stmt
        .query_map(params![kind, SHOWN], |r| {
            let pack_location: Option<i64> = r.get(3)?;
            let subject: String = r.get(6)?;
            let pack_name: Option<String> = r.get(2)?;
            let location_name: Option<String> = r.get(5)?;
            let asset_id: Option<String> = r.get(10)?;
            let title = if subject.is_empty() {
                pack_name.clone().or_else(|| location_name.clone()).unwrap_or_default()
            } else {
                subject.clone()
            };
            Ok(Item {
                id: format!("issue:{}", r.get::<_, i64>(0)?),
                title,
                pack_id: r.get(1)?,
                pack_name,
                location_id: r.get(4)?,
                location_name,
                path: (!subject.is_empty()).then_some(subject),
                message: r.get(7)?,
                count: r.get(8)?,
                at: r.get(9)?,
                // A file left out of an upload is gone with the upload; one in a watched folder is tried again, and
                // one that's in the library is read again with its pack.
                retry: match kind {
                    UNPACK => false,
                    FILE => pack_location.is_some() || asset_id.is_some(),
                    _ => true,
                },
                asset_id,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok((rows, total))
}

/// `GET /api/admin/issues`: every kind of issue that has any, most serious first.
pub(crate) async fn list(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let groups = db::call(&state.pool, |c| {
        let mut out = Vec::new();
        for g in GROUPS {
            let (rows, total) = items(c, g.kind)?;
            if total == 0 {
                continue;
            }
            out.push(json!({
                "kind": g.kind, "title": g.title, "blurb": g.blurb, "retry": g.retry, "dismiss": g.dismiss,
                "total": total, "items": rows.into_iter().map(Item::json).collect::<Vec<_>>(),
            }));
        }
        Ok(out)
    })
    .await?;
    let total: i64 = groups.iter().filter_map(|g| g["total"].as_i64()).sum();
    Ok(Json(json!({ "total": total, "groups": groups })))
}

/// The issues an action is for: their ids as [`list`] gives them, or every one of a kind.
#[derive(Debug, Deserialize)]
pub(crate) struct Pick {
    #[serde(default)]
    ids: Vec<String>,
    #[serde(default)]
    kind: Option<String>,
}

/// The ids of a whole group, for "Retry all" and "Dismiss all".
fn ids_of(c: &Connection, pick: Pick) -> Result<Vec<String>> {
    let mut ids = pick.ids;
    if let Some(kind) = pick.kind {
        // Everything in the group, not only what's listed.
        let (sql, prefix): (&str, &str) = match kind.as_str() {
            "job" => {
                ("SELECT CAST(id AS TEXT) FROM jobs WHERE status = 'failed' AND kind NOT IN ('ingest', 'retag')", "job")
            }
            "read" => ("SELECT id FROM assets WHERE json_extract(meta, '$.analysis_error') IS NOT NULL", "asset"),
            "render" => ("SELECT id FROM assets WHERE json_extract(meta, '$.thumb_error') IS NOT NULL", "asset"),
            "missing" => ("SELECT id FROM packs WHERE missing_since IS NOT NULL", "missing"),
            _ => ("SELECT CAST(id AS TEXT) FROM issues WHERE kind = ?1", "issue"),
        };
        let mut stmt = c.prepare(sql)?;
        let args: &[&dyn rusqlite::ToSql] = if sql.contains("?1") { &[&kind] } else { &[] };
        let found = stmt.query_map(args, |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        ids.extend(found.into_iter().map(|id| format!("{prefix}:{id}")));
    }
    Ok(ids)
}

/// `POST /api/admin/issues/dismiss` with `{ids}` or `{kind}`: forget them. Only what Hoard can forget is: an issue
/// it recorded, or a failed job. The rest is the state of the library, and goes away when that's put right.
pub(crate) async fn dismiss(State(state): State<AppState>, Json(chosen): Json<Pick>) -> ApiResult<Json<Value>> {
    let n = db::call(&state.pool, move |c| {
        let ids = ids_of(c, chosen)?;
        let tx = c.transaction()?;
        let mut n = 0;
        for id in &ids {
            n += match id.split_once(':') {
                Some(("issue", i)) => tx.execute("DELETE FROM issues WHERE id = ?1", [i])?,
                Some(("job", i)) => tx.execute("DELETE FROM jobs WHERE id = ?1 AND status = 'failed'", [i])?,
                _ => 0,
            };
        }
        tx.commit()?;
        Ok(n)
    })
    .await?;
    Ok(Json(json!({ "dismissed": n })))
}

/// What retrying an issue does.
#[derive(Debug, PartialEq, Eq, Hash)]
enum Redo {
    Job(i64),
    Scan(i64),
    Retag(String),
    Queue(&'static str, String),
    Convert(String),
}

/// What retrying a recorded issue does, if anything.
fn redo_recorded(c: &Connection, id: &str) -> Result<Option<Redo>> {
    struct Row {
        kind: String,
        pack: Option<String>,
        location: Option<i64>,
        pack_location: Option<i64>,
        asset: Option<String>,
    }
    let row = c
        .query_row(
            "SELECT i.kind, i.pack_id, i.location_id, p.location_id,
                    (SELECT a.id FROM assets a WHERE a.pack_id = i.pack_id AND a.path = i.subject)
               FROM issues i LEFT JOIN packs p ON p.id = i.pack_id WHERE i.id = ?1",
            [id],
            |r| {
                Ok(Row {
                    kind: r.get(0)?,
                    pack: r.get(1)?,
                    location: r.get(2)?,
                    pack_location: r.get(3)?,
                    asset: r.get(4)?,
                })
            },
        )
        .optional()?;
    let Some(row) = row else { return Ok(None) };
    Ok(match row.kind.as_str() {
        FOLDER => row.location.map(Redo::Scan),
        // A file that's in the library couldn't be read again when its pack was re-analyzed; one that isn't was
        // left out when a watched pack was scanned.
        FILE if row.asset.is_some() => row.pack.map(Redo::Retag),
        FILE => row.pack_location.map(Redo::Scan),
        STORE_FILES => row.pack.map(|p| Redo::Queue(jobs::READ_FILES, p)),
        MATERIALS | MATERIAL_THUMB => row.pack.map(|p| Redo::Queue(jobs::MATERIALS, p)),
        CONVERT => row.pack.map(Redo::Convert),
        _ => None,
    })
}

/// `POST /api/admin/issues/retry` with `{ids}` or `{kind}`: run again whatever went wrong. A pack is re-analyzed,
/// a folder scanned or a job run once however many of its issues are picked. A recorded issue is forgotten; if it
/// goes wrong again, it's recorded again.
pub(crate) async fn retry(State(state): State<AppState>, Json(chosen): Json<Pick>) -> ApiResult<Json<Value>> {
    let st = std::sync::Arc::clone(&state);
    let n = db::call(&state.pool, move |c| {
        let ids = ids_of(c, chosen)?;
        let mut todo: Vec<Redo> = Vec::new();
        let mut forget = Vec::new();
        for id in &ids {
            let redo = match id.split_once(':') {
                Some(("job", i)) => i.parse().ok().map(Redo::Job),
                Some(("asset", a)) => c
                    .query_row("SELECT pack_id FROM assets WHERE id = ?1", [a], |r| r.get(0))
                    .optional()?
                    .map(Redo::Retag),
                Some(("missing", p)) => c
                    .query_row("SELECT location_id FROM packs WHERE id = ?1", [p], |r| r.get::<_, Option<i64>>(0))
                    .optional()?
                    .flatten()
                    .map(Redo::Scan),
                Some(("issue", i)) => {
                    let redo = redo_recorded(c, i)?;
                    if redo.is_some() {
                        forget.push(i.to_owned());
                    }
                    redo
                }
                _ => None,
            };
            if let Some(r) = redo
                && !todo.contains(&r)
            {
                todo.push(r);
            }
        }
        let now = db::now();
        for r in &todo {
            match r {
                Redo::Job(id) => {
                    c.execute(
                        "UPDATE jobs SET status = 'queued', attempts = 0, error = NULL, run_after = ?2, updated_at = ?2
                         WHERE id = ?1 AND status = 'failed'",
                        params![id, now],
                    )?;
                }
                Redo::Scan(loc) => crate::scan::enqueue(c, &st, *loc)?,
                Redo::Retag(pack) => {
                    if !db::job_pending(c, jobs::RETAG, Some(pack), None)? {
                        db::set_pack_status(c, pack, hoard_core::PackStatus::Ingesting, None)?;
                        db::enqueue(c, jobs::RETAG, Some(pack), &json!({ "then_llm": false }))?;
                    }
                }
                Redo::Queue(kind, pack) => {
                    if !db::job_pending(c, kind, Some(pack), None)? {
                        db::enqueue(c, kind, Some(pack), &json!({}))?;
                    }
                }
                Redo::Convert(pack) => crate::convert::maybe_enqueue(&st, c, pack)?,
            }
        }
        let forget: HashSet<String> = forget.into_iter().collect();
        for i in &forget {
            c.execute("DELETE FROM issues WHERE id = ?1", [i])?;
        }
        Ok(todo.len())
    })
    .await?;
    if n == 0 {
        return Err(AppError::bad_request("nothing there can be retried"));
    }
    state.wake.notify_one();
    Ok(Json(json!({ "started": n })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool() -> (tempfile::TempDir, db::Pool) {
        let dir = tempfile::tempdir().unwrap();
        let pool = db::open(&dir.path().join("hoard.db"), dir.path()).unwrap();
        (dir, pool)
    }

    #[test]
    fn the_same_issue_is_counted_and_cleared() {
        let (_dir, pool) = pool();
        let c = pool.get().unwrap();
        let now = db::now();
        c.execute("INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Pack', 'ready', ?1)", [&now])
            .unwrap();
        record(&c, FILE, At::file("p1", "a/b.png"), "bad").unwrap();
        record(&c, FILE, At::file("p1", "a/b.png"), "worse").unwrap();
        record(&c, FILE, At::file("p1", "a/c.png"), "bad").unwrap();
        record(&c, MATERIALS, At::pack("p1"), "no").unwrap();
        let (rows, total) = recorded(&c, FILE).unwrap();
        assert_eq!(total, 2);
        let b = rows.iter().find(|i| i.path.as_deref() == Some("a/b.png")).unwrap();
        assert_eq!((b.count, b.message.as_str()), (2, "worse"));
        // An uploaded pack's left-out files can't be tried again.
        assert!(!b.retry);

        clear(&c, &[FILE], At::pack("p1"));
        assert_eq!(recorded(&c, FILE).unwrap().1, 0);
        assert_eq!(recorded(&c, MATERIALS).unwrap().1, 1);
        // Deleting the pack takes its issues with it.
        c.execute("DELETE FROM packs WHERE id = 'p1'", []).unwrap();
        assert_eq!(recorded(&c, MATERIALS).unwrap().1, 0);
    }

    #[test]
    fn every_group_query_runs() {
        let (_dir, pool) = pool();
        let c = pool.get().unwrap();
        for g in GROUPS {
            let (rows, total) = items(&c, g.kind).unwrap();
            assert!(rows.is_empty() && total == 0, "{}", g.kind);
        }
    }
}
