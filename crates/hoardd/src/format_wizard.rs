//! Settings → Store files: teach Hoard another store's file. The wizard in the portal writes a format
//! ([`crate::store_formats`]) against a file from the library and tries it here on the other files like it. A saved
//! format goes in `<data dir>/formats/`, and the packs with files it reads are read again. "Share it" puts the format,
//! the sample and what the sample reads as in a zip laid out like the repository, to send in for everyone.

use crate::i18n::tr;
use std::collections::HashMap;
use std::io::Write as _;

use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use rusqlite::params_from_iter;
use rusqlite::types::Value as Sql;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::error::{ApiResult, AppError};
use crate::sidecar::{self, SIDECAR_BYTES, Sidecar};
use crate::store_formats::{self, Format};
use crate::{AppState, db, jobs};

/// Files of these types may be a store's description of what's beside them.
const TEXT_EXTS: &[&str] = &["json", "txt", "xml", "ini", "yaml", "yml", "cfg", "nfo", "toml", "meta"];
/// Files no store writes, though there's one in many packs.
const NOT_STORES: &[&str] =
    &["readme", "read me", "read_me", "changelog", "change log", "package.json", "package-lock"];
/// A file in only one pack is shown when its name sounds like a description.
const SOUNDS_LIKE: &[&str] =
    &["licen", "info", "meta", "credit", "asset", "about", "attribution", "manifest", "details"];
/// Files no format reads, listed at most.
const MAX_UNREAD: usize = 30;
/// Files of each name looked at before deciding the formats read them all.
const LOOK_AT: usize = 5;
/// Files the wizard tries a format on.
const TRY_ON: i64 = 40;
/// Text files bigger than this aren't read.
const MAX_TOML: usize = 64 * 1024;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/store-formats", get(list).post(save))
        .route("/api/admin/store-formats/try", post(try_format))
        .route("/api/admin/store-formats/share", post(share))
        .route("/api/admin/store-formats/{file}", delete(remove))
}

#[derive(Serialize)]
struct FormatRow {
    name: String,
    file: String,
    /// In the data folder, so it can be changed or removed.
    custom: bool,
    toml: String,
}

#[derive(Serialize)]
struct Unread {
    /// The file name, lowercase.
    name: String,
    packs: usize,
    files: usize,
    example: Place,
}

#[derive(Serialize, Clone)]
struct Place {
    asset_id: String,
    pack_id: String,
    pack_name: String,
    path: String,
}

/// `GET /api/admin/store-formats`: the formats, and files in the library that may be a store's but no format reads.
async fn list(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let formats: Vec<FormatRow> = store_formats::all()
        .iter()
        .map(|f| FormatRow { name: f.name.clone(), file: f.file.clone(), custom: f.custom, toml: f.text.clone() })
        .collect();
    let unread = unread(&state).await?;
    Ok(Json(json!({ "formats": formats, "unread": unread, "dir": store_formats::dir() })))
}

/// Files of one name.
struct Group {
    name: String,
    packs: Vec<String>,
    files: Vec<(Place, String)>,
}

async fn unread(state: &AppState) -> anyhow::Result<Vec<Unread>> {
    let rows = db::call(&state.pool, |c| {
        let exts = vec!["?"; TEXT_EXTS.len()].join(", ");
        let mut stmt = c.prepare(&format!(
            "SELECT a.id, a.path, a.pack_id, p.name, a.blob_hash FROM assets a
             JOIN blobs b ON b.hash = a.blob_hash JOIN packs p ON p.id = a.pack_id
             WHERE lower(a.ext) IN ({exts}) AND b.size <= ? AND p.status = 'ready'
               AND a.archived_at IS NULL AND p.archived_at IS NULL
             ORDER BY p.name, a.path LIMIT 50000"
        ))?;
        let mut args: Vec<Sql> = TEXT_EXTS.iter().map(|e| Sql::from((*e).to_owned())).collect();
        args.push(SIDECAR_BYTES.into());
        let rows = stmt
            .query_map(params_from_iter(args), |r| {
                Ok((
                    Place { asset_id: r.get(0)?, path: r.get(1)?, pack_id: r.get(2)?, pack_name: r.get(3)? },
                    r.get::<_, String>(4)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;

    let mut groups: Vec<Group> = Vec::new();
    let mut at: HashMap<String, usize> = HashMap::new();
    for (place, hash) in rows {
        let name = place.path.rsplit('/').next().unwrap_or(&place.path).to_lowercase();
        if name.starts_with('.') || NOT_STORES.iter().any(|n| name.starts_with(n)) {
            continue;
        }
        let i = *at.entry(name.clone()).or_insert_with(|| {
            groups.push(Group { name, packs: Vec::new(), files: Vec::new() });
            groups.len() - 1
        });
        let g = &mut groups[i];
        if !g.packs.contains(&place.pack_id) {
            g.packs.push(place.pack_id.clone());
        }
        g.files.push((place, hash));
    }
    groups.retain(|g| g.packs.len() >= 2 || SOUNDS_LIKE.iter().any(|w| g.name.contains(w)));
    groups.sort_by(|a, b| b.packs.len().cmp(&a.packs.len()).then_with(|| a.name.cmp(&b.name)));

    let mut out = Vec::new();
    for g in groups {
        if out.len() >= MAX_UNREAD {
            break;
        }
        // One file from each of the first few packs: a name some format reads in one pack may be another store's in
        // the next (license.txt).
        let mut seen = Vec::new();
        let mut example = None;
        for (place, hash) in &g.files {
            if seen.len() >= LOOK_AT || example.is_some() {
                break;
            }
            if seen.contains(&place.pack_id) {
                continue;
            }
            seen.push(place.pack_id.clone());
            let Ok(bytes) = state.blobs.read_all(hash).await else { continue };
            if sidecar::read(&place.path, &String::from_utf8_lossy(&bytes)).is_none() {
                example = Some(place.clone());
            }
        }
        if let Some(example) = example {
            out.push(Unread { name: g.name, packs: g.packs.len(), files: g.files.len(), example });
        }
    }
    Ok(out)
}

#[derive(Deserialize)]
struct Example {
    path: String,
    text: String,
}

#[derive(Deserialize)]
struct Trial {
    toml: String,
    example: Option<Example>,
}

/// Why a file wasn't read.
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Miss {
    /// `files` doesn't take its name.
    Name,
    /// `require` says it isn't the store's (or it isn't JSON, for a JSON format).
    NotTheStores,
    /// The store's, but `fields.title` finds nothing.
    NoTitle,
}

fn read_with(f: &Format, path: &str, text: &str) -> Result<Sidecar, Miss> {
    if !f.wants(path) {
        return Err(Miss::Name);
    }
    if f.open(text).is_none() {
        return Err(Miss::NotTheStores);
    }
    sidecar::read_as(f, path, text).ok_or(Miss::NoTitle)
}

#[derive(Serialize)]
struct ExampleRead {
    read: Option<Sidecar>,
    miss: Option<Miss>,
    /// Another format that reads the file.
    also: Option<String>,
}

#[derive(Serialize)]
struct Tried {
    place: Place,
    title: Option<String>,
    miss: Option<Miss>,
}

#[derive(Serialize)]
struct LibraryRead {
    /// Files in the library the format takes by name, and the packs they're in.
    files: i64,
    packs: i64,
    /// What it made of one file from each of up to [`TRY_ON`] of those packs.
    tried: Vec<Tried>,
}

fn parse(toml: &str) -> ApiResult<Format> {
    if toml.len() > MAX_TOML {
        return Err(AppError::bad_request(tr!("error-format-too-long")));
    }
    Format::parse(toml).map_err(|e| AppError::bad_request(format!("{e:#}")))
}

/// `POST /api/admin/store-formats/try`: what a format makes of the example and of the library's files like it.
async fn try_format(State(state): State<AppState>, Json(t): Json<Trial>) -> ApiResult<Json<Value>> {
    let f = match Format::parse(&t.toml) {
        Ok(f) if t.toml.len() <= MAX_TOML => f,
        Ok(_) => return Ok(Json(json!({ "error": tr!("error-format-too-long") }))),
        Err(e) => return Ok(Json(json!({ "error": format!("{e:#}") }))),
    };
    let example = t.example.map(|ex| {
        let got = read_with(&f, &ex.path, &ex.text);
        let also = store_formats::all()
            .iter()
            .filter(|o| !o.name.eq_ignore_ascii_case(&f.name) && o.wants(&ex.path))
            .find(|o| sidecar::read_as(o, &ex.path, &ex.text).is_some())
            .map(|o| o.name.clone());
        ExampleRead { miss: got.as_ref().err().copied(), read: got.ok(), also }
    });
    let library = try_on_library(&state, f).await?;
    Ok(Json(json!({ "example": example, "library": library })))
}

async fn try_on_library(state: &AppState, f: Format) -> anyhow::Result<LibraryRead> {
    let f = std::sync::Arc::new(f);
    let f2 = std::sync::Arc::clone(&f);
    let (files, packs, rows) = db::call(&state.pool, move |c| {
        let (filter, patterns) = sidecar::filter_for(std::slice::from_ref(&*f2));
        let from = format!(
            "FROM assets a JOIN blobs b ON b.hash = a.blob_hash JOIN packs p ON p.id = a.pack_id
             WHERE b.size <= ? AND a.archived_at IS NULL AND p.archived_at IS NULL AND {filter}"
        );
        let mut args: Vec<Sql> = vec![SIDECAR_BYTES.into()];
        args.extend(patterns.into_iter().map(Sql::from));
        let (files, packs): (i64, i64) = c.query_row(
            &format!("SELECT COUNT(*), COUNT(DISTINCT a.pack_id) {from}"),
            params_from_iter(args.clone()),
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        args.push(TRY_ON.into());
        let mut stmt = c.prepare(&format!(
            "SELECT a.id, a.path, a.pack_id, p.name, a.blob_hash {from} GROUP BY a.pack_id ORDER BY p.name LIMIT ?"
        ))?;
        let rows = stmt
            .query_map(params_from_iter(args), |r| {
                Ok((
                    Place { asset_id: r.get(0)?, path: r.get(1)?, pack_id: r.get(2)?, pack_name: r.get(3)? },
                    r.get::<_, String>(4)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok((files, packs, rows))
    })
    .await?;
    let mut tried = Vec::new();
    for (place, hash) in rows {
        let Ok(bytes) = state.blobs.read_all(&hash).await else { continue };
        let got = read_with(&f, &place.path, &String::from_utf8_lossy(&bytes));
        tried.push(Tried { place, miss: got.as_ref().err().copied(), title: got.ok().map(|s| s.title) });
    }
    Ok(LibraryRead { files, packs, tried })
}

#[derive(Deserialize)]
struct Save {
    toml: String,
    /// The file it was opened from, which goes if the name changed.
    replaces: Option<String>,
}

/// A file in the data folder's formats, by a name from this API.
fn format_file(name: &str) -> ApiResult<std::path::PathBuf> {
    let dir = store_formats::dir().ok_or_else(|| AppError::unavailable(tr!("error-not-started")))?;
    let ok = !name.is_empty()
        && !name.contains(['/', '\\', ':'])
        && !name.starts_with('.')
        && std::path::Path::new(name).extension().is_some_and(|e| e.eq_ignore_ascii_case("toml"));
    if !ok {
        return Err(AppError::bad_request(tr!("error-not-format-file", name = name)));
    }
    Ok(dir.join(name))
}

/// `POST /api/admin/store-formats`: keep a format in the data folder and read the packs with files it reads again.
async fn save(State(state): State<AppState>, Json(s): Json<Save>) -> ApiResult<Json<Value>> {
    let f = parse(&s.toml)?;
    let slug = store_formats::slug(&f.name);
    if slug.is_empty() {
        return Err(AppError::bad_request(tr!("error-format-name")));
    }
    let file = format!("{slug}.toml");
    let path = format_file(&file)?;
    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await.map_err(anyhow::Error::from)?;
    }
    tokio::fs::write(&path, s.toml.as_bytes()).await.map_err(anyhow::Error::from)?;
    if let Some(old) = s.replaces.filter(|r| !r.eq_ignore_ascii_case(&file)) {
        let old = format_file(&old)?;
        if old.is_file() {
            tokio::fs::remove_file(old).await.map_err(anyhow::Error::from)?;
        }
    }
    store_formats::reload();
    tracing::info!(format = %f.name, file, "store file format saved");
    let packs = read_again(&state, f).await?;
    Ok(Json(json!({ "file": file, "packs": packs })))
}

/// Queue the packs with files the format takes by name to be read again. How many were queued.
async fn read_again(state: &AppState, f: Format) -> anyhow::Result<usize> {
    let n = db::call(&state.pool, move |c| {
        let ids: Vec<String> = {
            let (filter, patterns) = sidecar::filter_for(std::slice::from_ref(&f));
            let mut stmt = c.prepare(&format!(
                "SELECT DISTINCT a.pack_id FROM assets a JOIN blobs b ON b.hash = a.blob_hash
                 JOIN packs p ON p.id = a.pack_id
                 WHERE p.status = 'ready' AND b.size <= ? AND {filter}"
            ))?;
            let mut args: Vec<Sql> = vec![SIDECAR_BYTES.into()];
            args.extend(patterns.into_iter().map(Sql::from));
            stmt.query_map(params_from_iter(args), |r| r.get(0))?.collect::<rusqlite::Result<_>>()?
        };
        let tx = c.transaction()?;
        let mut n = 0;
        for id in &ids {
            if !db::job_pending(&tx, jobs::READ_FILES, Some(id), None)? {
                db::enqueue(&tx, jobs::READ_FILES, Some(id), &json!({}))?;
                n += 1;
            }
        }
        tx.commit()?;
        Ok(n)
    })
    .await?;
    if n > 0 {
        state.wake.notify_one();
    }
    Ok(n)
}

/// `DELETE /api/admin/store-formats/{file}`: stop using a format of the data folder. A built-in one of that name
/// comes back; what packs were given stays.
async fn remove(Path(file): Path<String>) -> ApiResult<StatusCode> {
    let path = format_file(&file)?;
    if !path.is_file() {
        return Err(AppError::not_found(tr!("error-format-not-found", file = file)));
    }
    tokio::fs::remove_file(&path).await.map_err(anyhow::Error::from)?;
    store_formats::reload();
    tracing::info!(file, "store file format removed");
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct Share {
    toml: String,
    example: Example,
}

/// `POST /api/admin/store-formats/share`: a zip of the format, the sample and what it reads as, laid out like the
/// repository (`crates/hoardd/formats/`), with a note on adding it.
async fn share(Json(s): Json<Share>) -> ApiResult<Response> {
    let f = parse(&s.toml)?;
    let slug = store_formats::slug(&f.name);
    if slug.is_empty() {
        return Err(AppError::bad_request(tr!("error-format-name")));
    }
    let name = s.example.path.rsplit(['/', '\\']).next().unwrap_or_default().to_owned();
    let got = read_with(&f, &name, &s.example.text).map_err(|m| {
        AppError::bad_request(match m {
            Miss::Name => tr!("error-format-miss-name", file = name),
            Miss::NotTheStores => tr!("error-format-miss-store", file = name),
            Miss::NoTitle => tr!("error-format-miss-title", file = name),
        })
    })?;
    let zip = share_zip(&f, &slug, &name, &s.example.text, &got).map_err(AppError::from)?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/zip".to_owned()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"hoard-format-{slug}.zip\"")),
        ],
        zip,
    )
        .into_response())
}

/// What a sample should read as: the values it gave, not the empty ones or its path.
fn expected(got: &Sidecar) -> anyhow::Result<String> {
    let mut v = serde_json::to_value(got)?;
    if let Value::Object(m) = &mut v {
        m.remove("path");
        m.retain(|_, x| match x {
            Value::Null => false,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            _ => true,
        });
    }
    Ok(serde_json::to_string_pretty(&v)? + "\n")
}

fn share_zip(f: &Format, slug: &str, name: &str, text: &str, got: &Sidecar) -> anyhow::Result<Vec<u8>> {
    let root = format!("hoard-format-{slug}");
    let how = format!(
        "{format}: a store file format for Hoard\n\
         \n\
         Copy crates/ into a checkout of Hoard, then add the format to BUILT_IN in\n\
         crates/hoardd/src/store_formats.rs, in name order:\n\
         \n\
         \x20   (\"{slug}.toml\", include_str!(\"../formats/{slug}.toml\")),\n\
         \n\
         and check it reads its sample as {name}.expected.json says:\n\
         \n\
         \x20   cargo test -p hoardd store_formats\n\
         \n\
         The sample is a file from a pack, as it was shared: take out anything that\n\
         shouldn't be public before sending it in.\n",
        format = f.name,
    );
    let files = [
        (format!("{root}/HOW-TO-ADD.txt"), how),
        (format!("{root}/crates/hoardd/formats/{slug}.toml"), f.text.clone()),
        (format!("{root}/crates/hoardd/formats/samples/{slug}/{name}"), text.to_owned()),
        (format!("{root}/crates/hoardd/formats/samples/{slug}/{name}.expected.json"), expected(got)?),
    ];
    let mut out = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut out));
        let opts = zip::write::SimpleFileOptions::default().last_modified_time(zip::DateTime::default_for_write());
        for (path, body) in files {
            zip.start_file(path, opts)?;
            zip.write_all(body.as_bytes())?;
        }
        zip.finish()?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORMAT: &str = r#"name = "Example Store"
files = ["asset_info.json"]
require = ["store"]
[fields]
title = "item.title"
seller = "item.by"
words = "item.tags"
"#;

    #[test]
    fn misses_say_why() {
        let f = Format::parse(FORMAT).unwrap();
        let ok = r#"{"store": "x", "item": {"title": "Old crate", "by": "Someone", "tags": ["wood", "crate"]}}"#;
        assert_eq!(read_with(&f, "a/asset_info.json", ok).unwrap().title, "Old crate");
        assert_eq!(read_with(&f, "a/other.json", ok), Err(Miss::Name));
        assert_eq!(read_with(&f, "asset_info.json", r#"{"item": {"title": "X"}}"#), Err(Miss::NotTheStores));
        assert_eq!(read_with(&f, "asset_info.json", "not json"), Err(Miss::NotTheStores));
        assert_eq!(read_with(&f, "asset_info.json", r#"{"store": 1, "item": {}}"#), Err(Miss::NoTitle));
    }

    /// The zip has what the samples test needs, and the sample reads as its expected file says.
    #[test]
    fn a_shared_format_is_laid_out_like_the_repository() {
        let f = Format::parse(FORMAT).unwrap();
        let text = r#"{"store": "x", "item": {"title": "Old crate", "by": "Someone", "tags": ["wood"]}}"#;
        let got = read_with(&f, "asset_info.json", text).unwrap();
        let bytes = share_zip(&f, "example-store", "asset_info.json", text, &got).unwrap();
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let mut names: Vec<String> = zip.file_names().map(|n| n.unwrap().into_owned()).collect();
        names.sort();
        assert_eq!(
            names,
            [
                "hoard-format-example-store/HOW-TO-ADD.txt",
                "hoard-format-example-store/crates/hoardd/formats/example-store.toml",
                "hoard-format-example-store/crates/hoardd/formats/samples/example-store/asset_info.json",
                "hoard-format-example-store/crates/hoardd/formats/samples/example-store/asset_info.json.expected.json",
            ]
        );
        let mut expected = String::new();
        std::io::Read::read_to_string(
            &mut zip
                .by_name(
                    "hoard-format-example-store/crates/hoardd/formats/samples/example-store/asset_info.json.expected.json",
                )
                .unwrap(),
            &mut expected,
        )
        .unwrap();
        let v: Value = serde_json::from_str(&expected).unwrap();
        assert_eq!(v["title"], "Old crate");
        assert_eq!(v["format"], "Example Store");
        assert_eq!(v["seller"], "Someone");
        assert!(v.get("path").is_none() && v.get("license").is_none() && v.get("maps").is_none(), "{v}");
    }
}
