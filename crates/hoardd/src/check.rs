//! A ship check: given the BLAKE3 hashes of the files a project ships, which ones came from Hoard, what to credit,
//! and which files need a look before shipping (an unknown or non-commercial license, share alike, an image that says
//! it was made with AI). Hoard isn't told about projects: whatever builds them asks, with the hashes of what it ships
//! (`b3sum`, or the `blake3` library).
//!
//! A hash matches a file Hoard keeps as it is, the original of a file Hoard converted on upload (an FBX's GLB), or a
//! file Hoard made from one (a download in another format).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use anyhow::Result;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::credits::{Attribution, Credits, Entry, FormatQuery, Limit};
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{AppState, db};

/// The most hashes one check takes.
pub(crate) const MAX_HASHES: usize = 20_000;

#[derive(Debug, Deserialize)]
pub(crate) struct CheckRequest {
    pub(crate) hashes: Vec<String>,
}

/// How a hash matched an asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Match {
    /// The asset's file, as Hoard keeps it.
    File,
    /// The file as it was uploaded, before Hoard converted it.
    Original,
    /// A file Hoard made from the asset (another format).
    Conversion,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Warning {
    /// `license_unknown`, `non_commercial`, `share_alike`, `keep_notice`, `fab_personal`, `ai_generated`,
    /// `ai_edited`.
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Found {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) kind: String,
    pub(crate) pack_id: String,
    pub(crate) pack: String,
    #[serde(rename = "match")]
    pub(crate) matched: Match,
    /// Whether its pack's license asks for credit (the line is in `credits`).
    pub(crate) attribution: Attribution,
    pub(crate) warnings: Vec<Warning>,
}

#[derive(Debug, Serialize)]
pub(crate) struct File {
    pub(crate) hash: String,
    /// Clear to ship: one of the assets it matches has no warnings (the same bytes in a CC0 pack and an unknown one
    /// are covered by the CC0 one).
    pub(crate) ok: bool,
    pub(crate) assets: Vec<Found>,
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct Summary {
    pub(crate) checked: usize,
    pub(crate) found: usize,
    pub(crate) not_found: usize,
    pub(crate) need_a_look: usize,
    /// How many files have each warning.
    pub(crate) warnings: BTreeMap<&'static str, usize>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Check {
    /// Nothing found needs a look.
    pub(crate) ok: bool,
    pub(crate) summary: Summary,
    /// The files Hoard knows, the ones that need a look first.
    pub(crate) files: Vec<File>,
    /// Hashes Hoard doesn't know: not from Hoard (the project's own work, or from elsewhere).
    pub(crate) not_found: Vec<String>,
    /// What wasn't a BLAKE3 hash (64 hex digits).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) invalid: Vec<String>,
    /// Credits for the packs the files came from.
    pub(crate) credits: Credits,
}

/// An asset's name, path, kind, pack id, pack name and AI facts.
type Info = (String, String, String, String, String, Option<String>);

/// Check these hashes (any case, duplicates ignored).
pub(crate) fn check(c: &Connection, hashes: &[String]) -> Result<Check> {
    let mut wanted = BTreeSet::new();
    let mut invalid = Vec::new();
    for h in hashes {
        let lower = h.trim().to_ascii_lowercase();
        if crate::blobs::is_valid_hash(&lower) {
            wanted.insert(lower);
        } else {
            invalid.push(h.clone());
        }
    }
    // (hash, asset id, how), in that order.
    let mut matches: Vec<(String, String, Match)> = Vec::new();
    {
        let mut stmt = c.prepare_cached(
            "SELECT id, 'file' FROM assets WHERE blob_hash = ?1
             UNION SELECT asset_id, 'original' FROM asset_origin WHERE blob_hash = ?1
             UNION SELECT asset_id, 'conversion' FROM derivatives WHERE blob_hash = ?1",
        )?;
        for h in &wanted {
            let rows = stmt.query_map([h], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
            for row in rows {
                let (id, how) = row?;
                let how = match how.as_str() {
                    "file" => Match::File,
                    "original" => Match::Original,
                    _ => Match::Conversion,
                };
                matches.push((h.clone(), id, how));
            }
        }
    }
    // An asset matched more than one way keeps the closest.
    matches.sort_by(|a, b| (&a.0, &a.1, a.2 as u8).cmp(&(&b.0, &b.1, b.2 as u8)));
    matches.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    let mut info = c.prepare_cached(
        "SELECT a.name, a.path, a.kind, a.pack_id, p.name, json_extract(a.meta, '$.ai')
           FROM assets a JOIN packs p ON p.id = a.pack_id WHERE a.id = ?1",
    )?;
    let mut assets: Vec<(String, String, Match, Info)> = Vec::new();
    for (h, id, how) in matches {
        let i: Info =
            info.query_row([&id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)))?;
        assets.push((h, id, how, i));
    }
    let packs: BTreeSet<String> = assets.iter().map(|a| a.3.3.clone()).collect();
    let credits = crate::credits::for_packs(c, &packs)?;
    let entries: BTreeMap<&str, &Entry> = credits.packs.iter().map(|e| (e.pack_id.as_str(), e)).collect();

    let mut files: BTreeMap<String, File> = BTreeMap::new();
    for (hash, id, matched, (name, path, kind, pack_id, pack, ai)) in assets {
        let entry = entries.get(pack_id.as_str()).copied();
        let mut warnings = entry.map(license_warnings).unwrap_or_default();
        if let Some(w) = ai.as_deref().and_then(ai_warning) {
            warnings.push(w);
        }
        let found = Found {
            id,
            name,
            path,
            kind,
            pack_id,
            pack,
            matched,
            attribution: entry.map_or(Attribution::Unknown, |e| e.attribution),
            warnings,
        };
        files.entry(hash.clone()).or_insert_with(|| File { hash, ok: true, assets: Vec::new() }).assets.push(found);
    }
    let mut summary = Summary { checked: wanted.len(), found: files.len(), ..Default::default() };
    for f in files.values_mut() {
        // AI is in the bytes, so every match says it; a license warning only counts if no other pack covers it.
        f.ok = f.assets.iter().any(|a| a.warnings.is_empty());
        if !f.ok {
            summary.need_a_look += 1;
            let codes: BTreeSet<&'static str> =
                f.assets.iter().flat_map(|a| a.warnings.iter().map(|w| w.code)).collect();
            for code in codes {
                *summary.warnings.entry(code).or_default() += 1;
            }
        }
    }
    let not_found: Vec<String> = wanted.iter().filter(|h| !files.contains_key(*h)).cloned().collect();
    summary.not_found = not_found.len();
    let mut files: Vec<File> = files.into_values().collect();
    files.sort_by_key(|f| f.ok);
    Ok(Check { ok: summary.need_a_look == 0, summary, files, not_found, invalid, credits })
}

fn license_warnings(e: &Entry) -> Vec<Warning> {
    let w = |code, message: String| Warning { code, message };
    let mut out = Vec::new();
    if e.attribution == Attribution::Unknown {
        out.push(w(
            "license_unknown",
            match &e.license {
                Some(l) => format!("\"{}\" is licensed as \"{l}\", which Hoard can't read: check its terms.", e.pack),
                None => format!("Hoard doesn't know the license of \"{}\": find out before shipping it.", e.pack),
            },
        ));
    }
    match e.limit {
        Some(Limit::NonCommercial) => out.push(w(
            "non_commercial",
            format!("\"{}\" is {}: non-commercial use only.", e.pack, e.license.as_deref().unwrap_or("non-commercial")),
        )),
        Some(Limit::ShareAlike) => out.push(w(
            "share_alike",
            format!(
                "\"{}\" is {}: what's made from it must be shared under the same license.",
                e.pack,
                e.license.as_deref().unwrap_or("share alike")
            ),
        )),
        None => {}
    }
    if e.attribution == Attribution::Terms {
        out.push(w(
            "keep_notice",
            format!(
                "\"{}\" is {}: ship its license notice with it.",
                e.pack,
                e.license.as_deref().unwrap_or("licensed with a notice")
            ),
        ));
    }
    if e.store.as_deref() == Some("Fab")
        && e.license_tier.as_deref().is_some_and(|t| t.eq_ignore_ascii_case("personal"))
    {
        out.push(w(
            "fab_personal",
            format!(
                "\"{}\" was bought under Fab's Personal license, which is for individuals and companies earning under \
                 $100,000 a year; above that it needs the Professional license.",
                e.pack
            ),
        ));
    }
    out
}

fn ai_warning(ai: &str) -> Option<Warning> {
    let ai: Value = serde_json::from_str(ai).ok()?;
    let by = ai.get("by").and_then(Value::as_str).map(|b| format!(" ({b})")).unwrap_or_default();
    let how = ai.get("how").and_then(Value::as_str).unwrap_or("its metadata");
    let (code, made) = match ai.get("made")?.as_str()? {
        "generated" => ("ai_generated", "made with generative AI"),
        "edited" => ("ai_edited", "edited with generative AI"),
        _ => return None,
    };
    Some(Warning {
        code,
        message: format!("It says it was {made}{by}, in its {how}. Stores such as Steam ask for this to be disclosed."),
    })
}

/// `POST /api/check?format=`: check the hashes of a project's files. `format=md` returns `CREDITS.md` with a section
/// on what needs a look.
pub(crate) async fn check_handler(
    State(state): State<AppState>,
    Query(f): Query<FormatQuery>,
    Json(req): Json<CheckRequest>,
) -> ApiResult<Response> {
    if req.hashes.len() > MAX_HASHES {
        return Err(AppError::bad_request(tr!("error-too-many-hashes", max = MAX_HASHES)));
    }
    let result = db::call(&state.pool, move |c| check(c, &req.hashes)).await?;
    Ok(if f.markdown() {
        ([(header::CONTENT_TYPE, "text/markdown; charset=utf-8")], markdown(&result)).into_response()
    } else {
        Json(result).into_response()
    })
}

/// `CREDITS.md` for what was found, then what needs a look.
pub(crate) fn markdown(check: &Check) -> String {
    let mut md = crate::credits::markdown(&check.credits);
    let s = &check.summary;
    let _ = write!(md, "\n## Ship check\n\n{} files checked: {} from Hoard, {} not.", s.checked, s.found, s.not_found);
    if check.ok {
        md.push_str(" Nothing from Hoard needs a look.\n");
        return md;
    }
    let _ = writeln!(md, " {} need a look:\n", s.need_a_look);
    for f in check.files.iter().filter(|f| !f.ok) {
        let Some(a) = f.assets.first() else { continue };
        let _ = writeln!(md, "- `{}` ({}, in {}, {})", a.path, a.name, a.pack, &f.hash[..12]);
        let mut said = BTreeSet::new();
        for w in f.assets.iter().flat_map(|a| &a.warnings) {
            if said.insert(&w.message) {
                let _ = writeln!(md, "  - {}", w.message);
            }
        }
    }
    md
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(n: u8) -> String {
        format!("{n:02x}").repeat(32)
    }

    fn library() -> (tempfile::TempDir, db::Conn) {
        let dir = tempfile::tempdir().unwrap();
        let pool = db::open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let c = pool.get().unwrap();
        c.execute_batch(&format!(
            "INSERT INTO packs (id, name, status, license, created_at) VALUES
               ('p1', 'Free Rocks', 'ready', 'cc0', 'x'), ('p2', 'Mystery Pack', 'ready', NULL, 'x'),
               ('p3', 'Paid Rocks', 'ready', NULL, 'x');
             INSERT INTO pack_context (pack_id, seller, store, store_url, license_tier) VALUES
               ('p3', 'Rocky', 'Fab', 'https://fab.com/x', 'Personal');
             INSERT INTO blobs (hash, size, created_at) VALUES ('{a}', 1, 'x'), ('{b}', 1, 'x'), ('{c}', 1, 'x'),
               ('{d}', 1, 'x'), ('{e}', 1, 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('a1', 'p1', '{a}', 'rock.png', 'Rock', 'png', 'texture', '{{}}', 'x', 'x'),
               ('a2', 'p2', '{a}', 'rock.png', 'Rock', 'png', 'texture', '{{}}', 'x', 'x'),
               ('a3', 'p2', '{b}', 'tree.png', 'Tree', 'png', 'texture', '{{}}', 'x', 'x'),
               ('a4', 'p1', '{c}', 'sky.png', 'Sky', 'png', 'texture',
                '{{\"ai\": {{\"made\": \"generated\", \"by\": \"Google Gemini\", \"how\": \"C2PA Content Credentials\"}}}}', 'x', 'x'),
               ('a5', 'p3', '{d}', 'boulder.glb', 'Boulder', 'glb', 'model', '{{}}', 'x', 'x');
             INSERT INTO asset_origin (asset_id, path, ext, blob_hash, method, converted_at)
               VALUES ('a5', 'boulder.fbx', 'fbx', '{e}', 'native', 'x');",
            a = h(1),
            b = h(2),
            c = h(3),
            d = h(4),
            e = h(5)
        ))
        .unwrap();
        (dir, c)
    }

    #[test]
    fn finds_files_credits_and_what_needs_a_look() {
        let (_dir, c) = library();
        let hashes = vec![h(1).to_uppercase(), h(2), h(3), h(5), h(9), "nope".into()];
        let r = check(&c, &hashes).unwrap();
        assert!(!r.ok);
        assert_eq!(r.invalid, vec!["nope".to_owned()]);
        assert_eq!(r.not_found, vec![h(9)]);
        assert_eq!((r.summary.checked, r.summary.found, r.summary.need_a_look), (5, 4, 3));
        let file = |n: u8| r.files.iter().find(|f| f.hash == h(n)).unwrap();
        // In a CC0 pack and an unknown one: the CC0 one covers it.
        assert!(file(1).ok);
        assert_eq!(file(1).assets.len(), 2);
        assert_eq!(file(2).assets[0].warnings[0].code, "license_unknown");
        assert_eq!(file(3).assets[0].warnings[0].code, "ai_generated");
        assert!(file(3).assets[0].warnings[0].message.contains("Google Gemini"));
        // The FBX as uploaded, before Hoard made it a GLB.
        let boulder = &file(5).assets[0];
        assert_eq!((boulder.id.as_str(), boulder.matched), ("a5", Match::Original));
        assert_eq!(boulder.warnings.iter().map(|w| w.code).collect::<Vec<_>>(), ["fab_personal"]);
        // Files needing a look come first.
        assert!(!r.files[0].ok);
        assert_eq!(r.credits.packs.len(), 3);
        let md = markdown(&r);
        assert!(md.contains("## Ship check") && md.contains("`tree.png`"), "{md}");
    }
}
