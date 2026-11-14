//! Credits for the packs a project uses: who made each, where it's sold, its license and whether the license asks
//! for attribution. Only what's known is stated: a license read from the pack's own files, or the license the
//! store sells it under (Fab's listing says). Where neither is known, the entry says so.
//!
//! Written as `CREDITS.md` (into pack, collection and selection zips, too) or as JSON.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use anyhow::Result;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use hoard_core::{CreditsRequest, ManifestRequest};
use rusqlite::{Connection, OptionalExtension as _};
use serde::{Deserialize, Serialize};

use crate::error::{ApiResult, AppError};
use crate::store_search::{CC_BY_4, CC0, FAB_STANDARD};
use crate::{AppState, db};

/// Does the license ask for credit?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Attribution {
    /// Yes: the credit line below is what it asks for.
    Yes,
    /// No.
    No,
    /// Not credit as such, but its notice must go with the files (MIT, OFL, GPL).
    Terms,
    /// The license isn't known.
    Unknown,
}

/// One pack's entry.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct Entry {
    pub(crate) pack_id: String,
    pub(crate) pack: String,
    pub(crate) seller: Option<String>,
    pub(crate) store: Option<String>,
    pub(crate) store_url: Option<String>,
    /// The license's name, where it's known.
    pub(crate) license: Option<String>,
    /// The license's text or terms.
    pub(crate) license_url: Option<String>,
    pub(crate) attribution: Attribution,
    /// What to write in the credits, where the license asks for it.
    pub(crate) credit_line: Option<String>,
    /// Anything else the license asks ("Non-commercial use only").
    pub(crate) note: Option<String>,
    /// The license tier it was bought under, where the store has tiers (Fab: Personal, Professional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) license_tier: Option<String>,
    /// What the license keeps you from doing, for ship checks.
    #[serde(skip)]
    pub(crate) limit: Option<Limit>,
}

/// What a license keeps a commercial project from doing freely.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Limit {
    /// It can't be sold.
    NonCommercial,
    /// Changes must be shared under the same license (CC BY-SA; the GPL for code).
    ShareAlike,
}

#[derive(Debug, Serialize)]
pub(crate) struct Credits {
    pub(crate) generated_at: String,
    pub(crate) packs: Vec<Entry>,
}

/// What's known about a license: its name, where its terms are, whether it asks for credit, and anything else.
struct License {
    name: &'static str,
    url: Option<&'static str>,
    attribution: Attribution,
    note: Option<&'static str>,
    limit: Option<Limit>,
}

/// The usual name of a license, however it's written (`CC-BY-SA-4.0` → `CC BY-SA`).
pub(crate) fn license_name(name: &str) -> Option<&'static str> {
    license(name).map(|l| l.name)
}

/// A license by the id the tagger gives it (`cc-by`), the name a store gives it (`Fab Standard License`), or as
/// someone typed it (`CC-BY 4.0`): case, spaces and punctuation don't matter.
fn license(name: &str) -> Option<License> {
    let l = |name, url, attribution, note| Some(License { name, url, attribution, note, limit: None });
    let limited = |limit, l: Option<License>| l.map(|l| License { limit: Some(limit), ..l });
    let key = name.chars().filter(char::is_ascii_alphanumeric).collect::<String>().to_ascii_lowercase();
    match key.as_str() {
        "cc0" | "cc010" | "creativecommonszero" => {
            l(CC0, Some("https://creativecommons.org/publicdomain/zero/1.0/"), Attribution::No, None)
        }
        "publicdomain" => l("Public domain", None, Attribution::No, None),
        k if k.starts_with("ccbync") => {
            limited(Limit::NonCommercial, l("CC BY-NC", None, Attribution::Yes, Some("Non-commercial use only.")))
        }
        k if k.starts_with("ccbysa") => limited(
            Limit::ShareAlike,
            l(
                "CC BY-SA",
                None,
                Attribution::Yes,
                Some("Share alike: changes to these assets must be shared under the same license."),
            ),
        ),
        "ccby40" | "ccby4" => l(CC_BY_4, Some("https://creativecommons.org/licenses/by/4.0/"), Attribution::Yes, None),
        k if k.starts_with("ccby") => l("CC BY", None, Attribution::Yes, None),
        "ofl" | "silofl" | "openfontlicense" | "silopenfontlicense" => l(
            "SIL Open Font License",
            Some("https://openfontlicense.org/"),
            Attribution::Terms,
            Some("Keep the license with the font files."),
        ),
        "mit" | "mitlicense" => {
            l("MIT", None, Attribution::Terms, Some("Keep the copyright and license notice with the files."))
        }
        k if k.starts_with("gpl") || k.starts_with("gnugpl") => limited(
            Limit::ShareAlike,
            l("GPL", None, Attribution::Terms, Some("Keep the license with the files; read its terms.")),
        ),
        "royaltyfree" => {
            l("Royalty-free", None, Attribution::Unknown, Some("Read the license that came with the pack."))
        }
        "fabstandardlicense" | "fabstandard" => {
            l(FAB_STANDARD, Some("https://www.fab.com/eula"), Attribution::No, None)
        }
        _ => None,
    }
}

/// What's stored about a pack, before it's turned into an entry.
struct Row {
    id: String,
    name: String,
    license: Option<String>,
    seller: Option<String>,
    store: Option<String>,
    store_url: Option<String>,
    store_license: Option<String>,
    license_tier: Option<String>,
}

fn credit(r: Row) -> Entry {
    // The pack's own license file says it best; then the license the store sells it under. A Fab license tier
    // (Personal, Professional) is the Fab Standard License. "Royalty-free" in the pack's files says less than the
    // store does.
    let fab_tier = r.store.as_deref() == Some("Fab") && r.license_tier.is_some();
    let own = r.license.as_deref().and_then(license);
    let store =
        r.store_license.as_deref().and_then(license).or_else(|| fab_tier.then(|| license(FAB_STANDARD)).flatten());
    let lic = match (own, store) {
        (Some(o), s) if o.attribution != Attribution::Unknown || s.is_none() => Some(o),
        (_, s) => s,
    };
    // A license typed in Store details that isn't one of these is still named.
    let typed = r.store_license.as_deref().filter(|t| license(t).is_none()).map(str::to_owned);
    let attribution = lic.as_ref().map_or(Attribution::Unknown, |l| l.attribution);
    let credit_line = (attribution == Attribution::Yes).then(|| {
        let mut s = format!("\"{}\"", r.name);
        if let Some(seller) = &r.seller {
            let _ = write!(s, " by {seller}");
        }
        if let Some(url) = &r.store_url {
            let _ = write!(s, " ({url})");
        }
        if let Some(l) = &lic {
            let _ = write!(s, ", licensed under {}", l.name);
            if let Some(u) = l.url {
                let _ = write!(s, " ({u})");
            }
        }
        s
    });
    Entry {
        pack_id: r.id,
        pack: r.name,
        seller: r.seller,
        store: r.store,
        store_url: r.store_url,
        license: lic.as_ref().map(|l| l.name.to_owned()).or(typed),
        license_url: lic.as_ref().and_then(|l| l.url).map(str::to_owned),
        attribution,
        credit_line,
        note: lic.as_ref().and_then(|l| l.note).map(str::to_owned),
        license_tier: r.license_tier,
        limit: lic.and_then(|l| l.limit),
    }
}

/// Credits for these packs, by name.
pub(crate) fn for_packs(c: &Connection, ids: &BTreeSet<String>) -> Result<Credits> {
    let mut stmt = c.prepare_cached(
        "SELECT p.id, p.name, p.license, c.seller, c.store, c.store_url, c.store_license, c.license_tier
         FROM packs p LEFT JOIN pack_context c ON c.pack_id = p.id WHERE p.id = ?1",
    )?;
    let mut packs = Vec::new();
    for id in ids {
        let row = stmt.query_row([id], |r| {
            Ok(Row {
                id: r.get(0)?,
                name: r.get(1)?,
                license: r.get(2)?,
                seller: r.get(3)?,
                store: r.get(4)?,
                store_url: r.get(5)?,
                store_license: r.get(6)?,
                license_tier: r.get(7)?,
            })
        });
        match row {
            Ok(r) => packs.push(credit(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => {}
            Err(e) => return Err(e.into()),
        }
    }
    packs.sort_by_key(|p| p.pack.to_lowercase());
    Ok(Credits { generated_at: db::now(), packs })
}

/// The packs these assets are in, and with `deps` the packs of what they need (a mesh's textures), as a zip of them
/// would hold.
pub(crate) fn packs_of_assets(
    c: &Connection,
    ids: &[String],
    deps: bool,
    animations: bool,
) -> Result<BTreeSet<String>> {
    let mut stmt = c.prepare_cached("SELECT pack_id FROM assets WHERE id = ?1")?;
    let mut out = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for id in ids {
        let closure = if deps { db::dependency_closure(c, id, animations)? } else { vec![id.clone()] };
        for a in closure.into_iter().filter(|a| seen.insert(a.clone())) {
            if let Some(p) = stmt.query_row([&a], |r| r.get::<_, String>(0)).optional()? {
                out.insert(p);
            }
        }
    }
    Ok(out)
}

/// The packs a selection's assets are in: its ids and what its search finds, as a manifest of it would list them.
fn packs_of_selection(c: &Connection, sel: &ManifestRequest) -> Result<BTreeSet<String>> {
    let mut ids = sel.ids.clone();
    if let Some(q) = &sel.query {
        let mut q = q.clone();
        q.limit = Some(q.limit.unwrap_or(5000).min(5000));
        ids.extend(db::search(c, &q, false)?.items.into_iter().map(|a| a.id));
    }
    packs_of_assets(c, &ids, sel.include_deps, sel.include_animations)
}

/// The packs a collection's assets (and what they need) are in, or None if there's no such collection.
fn packs_of_collection(c: &Connection, key: &str) -> Result<Option<(String, BTreeSet<String>)>> {
    let Some(col) = db::get_collection(c, key)? else { return Ok(None) };
    let ids = db::collection_asset_ids(c, &col.id)?;
    Ok(Some((col.name, packs_of_assets(c, &ids, true, false)?)))
}

/// A pack's id, by id or name.
fn pack_id(c: &Connection, key: &str) -> Result<Option<String>> {
    Ok(c.query_row(
        "SELECT id FROM packs WHERE id = ?1 OR name = ?1 COLLATE NOCASE ORDER BY id = ?1 DESC LIMIT 1",
        [key],
        |r| r.get(0),
    )
    .optional()?)
}

/// Everything a request names, as one set of packs. Errors name a pack or collection that isn't there.
fn resolve(c: &Connection, req: &CreditsRequest) -> Result<Result<BTreeSet<String>, String>> {
    let mut packs = BTreeSet::new();
    for key in &req.packs {
        match pack_id(c, key)? {
            Some(id) => packs.insert(id),
            None => return Ok(Err(format!("pack {key}"))),
        };
    }
    for key in &req.collections {
        match packs_of_collection(c, key)? {
            Some((_, p)) => packs.extend(p),
            None => return Ok(Err(format!("collection {key}"))),
        }
    }
    packs.extend(packs_of_assets(c, &req.ids, false, false)?);
    for sel in &req.selections {
        packs.extend(packs_of_selection(c, sel)?);
    }
    Ok(Ok(packs))
}

/// `?format=md` for `CREDITS.md`; JSON otherwise.
#[derive(Debug, Deserialize)]
pub(crate) struct FormatQuery {
    #[serde(default)]
    format: Option<String>,
}

impl FormatQuery {
    /// `format=md` (or `markdown`) asks for Markdown.
    pub(crate) fn markdown(&self) -> bool {
        matches!(self.format.as_deref(), Some("md" | "markdown"))
    }
}

fn respond(credits: &Credits, f: &FormatQuery) -> Response {
    if f.markdown() {
        (
            [
                (header::CONTENT_TYPE, "text/markdown; charset=utf-8"),
                (header::CONTENT_DISPOSITION, "attachment; filename=\"CREDITS.md\""),
            ],
            markdown(credits),
        )
            .into_response()
    } else {
        Json(credits).into_response()
    }
}

/// `GET /api/packs/{id}/credits?format=`: one pack's credits.
pub(crate) async fn pack_credits(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(f): Query<FormatQuery>,
) -> ApiResult<Response> {
    let key = id.clone();
    let credits = db::call(&state.pool, move |c| {
        let Some(id) = pack_id(c, &key)? else { return Ok(None) };
        Ok(Some(for_packs(c, &BTreeSet::from([id]))?))
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("pack {id}")))?;
    Ok(respond(&credits, &f))
}

/// `GET /api/collections/{id}/credits?format=`: credits for the packs a collection's assets come from.
pub(crate) async fn collection_credits(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(f): Query<FormatQuery>,
) -> ApiResult<Response> {
    let key = id.clone();
    let credits = db::call(&state.pool, move |c| {
        let Some((_, packs)) = packs_of_collection(c, &key)? else { return Ok(None) };
        Ok(Some(for_packs(c, &packs)?))
    })
    .await?
    .ok_or_else(|| AppError::not_found(format!("collection {id}")))?;
    Ok(respond(&credits, &f))
}

/// `POST /api/credits?format=`: credits for packs, collections, assets and selections together.
pub(crate) async fn credits(
    State(state): State<AppState>,
    Query(f): Query<FormatQuery>,
    Json(req): Json<CreditsRequest>,
) -> ApiResult<Response> {
    Ok(respond(&for_request(&state, req).await?, &f))
}

/// Credits for packs, collections, assets and selections together.
pub(crate) async fn for_request(state: &AppState, req: CreditsRequest) -> ApiResult<Credits> {
    db::call(&state.pool, move |c| {
        Ok(match resolve(c, &req)? {
            Ok(packs) => Ok(for_packs(c, &packs)?),
            Err(missing) => Err(missing),
        })
    })
    .await?
    .map_err(AppError::not_found)
}

/// A table cell: no line breaks or pipes.
fn cell(s: &str) -> String {
    s.replace(['\r', '\n'], " ").replace('|', "\\|")
}

/// `CREDITS.md`.
pub(crate) fn markdown(credits: &Credits) -> String {
    let mut md = String::from("# Credits\n\n");
    if credits.packs.is_empty() {
        md.push_str("No packs.\n");
        return md;
    }
    let date = credits.generated_at.get(..10).unwrap_or(&credits.generated_at);
    let _ = writeln!(md, "Assets from these packs, as Hoard knew them on {date}.\n");
    md.push_str("| Pack | Seller | Store | License | Attribution |\n|---|---|---|---|---|\n");
    for p in &credits.packs {
        let store = match (&p.store, &p.store_url) {
            (Some(s), Some(u)) => format!("[{}]({u})", cell(s)),
            (None, Some(u)) => format!("<{u}>"),
            (Some(s), None) => cell(s),
            (None, None) => String::new(),
        };
        let license = match (&p.license, &p.license_url) {
            (Some(l), Some(u)) => format!("[{}]({u})", cell(l)),
            (Some(l), None) => cell(l),
            (None, _) => "Not known".to_owned(),
        };
        let attribution = match p.attribution {
            Attribution::Yes => "Required",
            Attribution::No => "Not required",
            Attribution::Terms => "Keep the license notice",
            Attribution::Unknown => "Not known",
        };
        let _ = writeln!(
            md,
            "| {} | {} | {store} | {license} | {attribution} |",
            cell(&p.pack),
            p.seller.as_deref().map(cell).unwrap_or_default()
        );
    }
    let lines: Vec<&Entry> = credits.packs.iter().filter(|p| p.credit_line.is_some()).collect();
    if !lines.is_empty() {
        md.push_str("\n## Credit lines\n\nThese licenses ask for credit. Put these lines where your project lists its credits.\n\n");
        for p in lines {
            let _ = writeln!(md, "- {}", p.credit_line.as_deref().unwrap_or_default());
        }
    }
    let notes: Vec<&Entry> = credits.packs.iter().filter(|p| p.note.is_some()).collect();
    if !notes.is_empty() {
        md.push_str("\n## Also\n\n");
        for p in notes {
            let _ = writeln!(md, "- {}: {}", p.pack, p.note.as_deref().unwrap_or_default());
        }
    }
    let unknown = credits.packs.iter().filter(|p| p.attribution == Attribution::Unknown).count();
    if unknown == 1 {
        md.push_str(
            "\nHoard doesn't know whether 1 pack's license asks for credit. Check its store page or the files that \
             came with it.\n",
        );
    } else if unknown > 1 {
        let _ = writeln!(
            md,
            "\nHoard doesn't know whether {unknown} packs' licenses ask for credit. Check their store pages or the \
             files that came with them."
        );
    }
    md
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str) -> Row {
        Row {
            id: name.to_lowercase(),
            name: name.to_owned(),
            license: None,
            seller: None,
            store: None,
            store_url: None,
            store_license: None,
            license_tier: None,
        }
    }

    #[test]
    fn says_only_what_is_known() {
        let stand = credit(Row {
            seller: Some("Quixel".into()),
            store: Some("Fab".into()),
            store_url: Some("https://www.fab.com/listings/abc".into()),
            store_license: Some(FAB_STANDARD.into()),
            ..row("Historic Miscellaneous Stand Wood Old 05")
        });
        assert_eq!(stand.attribution, Attribution::No);
        assert_eq!(stand.license_url.as_deref(), Some("https://www.fab.com/eula"));
        assert!(stand.credit_line.is_none());

        let rocks = credit(Row {
            seller: Some("Jane Doe".into()),
            store: Some("Fab".into()),
            store_url: Some("https://www.fab.com/listings/def".into()),
            store_license: Some(CC_BY_4.into()),
            ..row("Mossy Rocks")
        });
        assert_eq!(rocks.attribution, Attribution::Yes);
        assert_eq!(
            rocks.credit_line.as_deref(),
            Some(
                "\"Mossy Rocks\" by Jane Doe (https://www.fab.com/listings/def), licensed under CC BY 4.0 \
                 (https://creativecommons.org/licenses/by/4.0/)"
            )
        );
        // The pack's own license file wins over the store's.
        let own = credit(Row { license: Some("cc0".into()), store_license: Some(CC_BY_4.into()), ..row("Own") });
        assert_eq!((own.attribution, own.license.as_deref()), (Attribution::No, Some("CC0")));
        let tier = credit(Row { store: Some("Fab".into()), license_tier: Some("Personal".into()), ..row("Tier") });
        assert_eq!(tier.license.as_deref(), Some(FAB_STANDARD));
        let nc = credit(Row { license: Some("cc-by-nc".into()), ..row("Spells") });
        assert_eq!(nc.note.as_deref(), Some("Non-commercial use only."));
        assert_eq!(nc.credit_line.as_deref(), Some("\"Spells\", licensed under CC BY-NC"));
        let unknown = credit(row("Mystery | Pack"));
        assert_eq!((unknown.attribution, unknown.license.as_deref()), (Attribution::Unknown, None));

        let md =
            markdown(&Credits { generated_at: "2026-11-12T10:00:00Z".into(), packs: vec![stand, rocks, nc, unknown] });
        assert!(md.starts_with("# Credits\n\nAssets from these packs, as Hoard knew them on 2026-11-12."));
        assert!(md.contains(
            "| Historic Miscellaneous Stand Wood Old 05 | Quixel | [Fab](https://www.fab.com/listings/abc) | \
             [Fab Standard License](https://www.fab.com/eula) | Not required |"
        ));
        assert!(md.contains("| Mystery \\| Pack |  |  | Not known | Not known |"));
        assert!(md.contains("## Credit lines"));
        assert!(md.contains("- Spells: Non-commercial use only."));
        assert!(md.contains("Hoard doesn't know whether 1 pack's license asks for credit."));
        // Typed by hand, in any case and spacing.
        let typed = credit(Row { store_license: Some("cc-by 4.0".into()), ..row("Typed") });
        assert_eq!((typed.attribution, typed.license.as_deref()), (Attribution::Yes, Some(CC_BY_4)));
        let other = credit(Row { store_license: Some("Unity Asset Store EULA".into()), ..row("Other") });
        assert_eq!(
            (other.attribution, other.license.as_deref()),
            (Attribution::Unknown, Some("Unity Asset Store EULA"))
        );
    }
}
