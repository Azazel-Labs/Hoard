//! Looking a pack up in the stores it may have come from, so its store details can be filled in with a click.
//!
//! Fab, `itch.io`, Kenney and `OpenGameArt` are searched from the server. Humble and the Unity Asset Store can't be (one
//! refuses non-browser clients, the other searches in the page's script), so for those the UI only gets search links.
//!
//! A pack's name is often too plain to pick out its listing ("Medieval Village" has dozens), so the pack's own files
//! are read for clues first: store links in its README, the seller's name ("Created by", "©", `Assets/<Publisher>/`),
//! product folders (`Content/MedievalVillage`), its engine and how many meshes and textures it has. The name and a
//! few variants of it are searched, and the clues decide the order.

mod clues;
mod pages;
mod rank;

use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Duration;

use anyhow::{Context as _, Result};
use axum::Json;
use axum::extract::{Path, Query, State};
use futures_util::future::join_all;
use rusqlite::OptionalExtension as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::{decode_entities, fetch_page};
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::sidecar;
use crate::{AppState, db};
use clues::{Link, clues, queries, same_title};
use pages::{inner_text, parse_fab_page, parse_itch, parse_kenney, parse_opengameart};
use rank::rank;
pub(crate) use rank::search_terms;

/// Results kept from each store after ranking.
const PER_STORE: usize = 6;
const MAX_MATCHES: usize = 12;
const SEARCH_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_QUERY: usize = 120;
/// Searches sent to Fab for one lookup (its search is quick and answers in JSON); the other stores get two.
const FAB_QUERIES: usize = 4;
const OTHER_QUERIES: usize = 2;
const MAX_PATHS: i64 = 50_000;
/// How many store pages the pack links to are looked at.
const MAX_LINKS: usize = 3;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct StoreMatch {
    store: &'static str,
    title: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
    /// The license the store sells it under, where the store says ([`FAB_STANDARD`], [`CC_BY_4`], [`CC0`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<&'static str>,
    score: f32,
    /// Why it ranks where it does, beyond its title ("By Marco Maria Rossi, named in the README").
    #[serde(skip_serializing_if = "Vec::is_empty")]
    why: Vec<Why>,
    /// Engines and tools the listing is made for (`unreal-engine`, `unity`), where the store says.
    #[serde(skip)]
    formats: Vec<String>,
    /// The listing's technical details as plain text, where the store gives them in search results.
    #[serde(skip)]
    specs: Option<String>,
    /// Its title is exactly what was searched for (the pack's name, or the product name in its files).
    #[serde(skip)]
    named: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct StoreLink {
    store: &'static str,
    url: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct StoreMatches {
    /// When the stores were searched.
    searched_at: String,
    query: String,
    /// The other searches made from the name and the pack's files.
    also: Vec<String>,
    /// What the pack's files gave away, in words ("Seller: Marco Maria Rossi (README)").
    clues: Vec<String>,
    matches: Vec<StoreMatch>,
    /// Stores that can only be searched in the browser.
    links: Vec<StoreLink>,
    /// Stores whose search failed this time.
    failed: Vec<&'static str>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MatchQuery {
    q: Option<String>,
    /// Search again instead of showing what the last lookup found.
    #[serde(default)]
    fresh: bool,
}

/// Licenses stores sell under. Fab's Personal and Professional tiers are both the Fab Standard License ("You are not
/// required to give credit to the publisher", fab.com/eula); its Creative Commons listings are CC BY 4.0.
pub(crate) const FAB_STANDARD: &str = "Fab Standard License";
pub(crate) const CC_BY_4: &str = "CC BY 4.0";
pub(crate) const CC0: &str = "CC0";

/// Where in the pack a clue was seen.
#[derive(Clone, Debug, PartialEq)]
enum Place {
    Readme,
    Folders,
    /// A store's own file, as its format calls it ("Sketchfab file").
    File(String),
}

impl Place {
    /// In English, for the clues an AI is given.
    fn english(&self) -> &str {
        match self {
            Self::Readme => "README",
            Self::Folders => "folders",
            Self::File(called) => called,
        }
    }

    /// In the request's language.
    fn text(&self) -> String {
        match self {
            Self::Readme => tr!("server-store-place-readme"),
            Self::Folders => tr!("server-store-place-folders"),
            Self::File(called) => called.clone(),
        }
    }
}

/// Why a match ranks where it does, beyond its title: sent written out in the request's language.
#[derive(Clone, Debug, PartialEq)]
enum Why {
    Linked(Place),
    SameTitle(Place),
    By { author: String, place: Place },
    Meshes(u64),
    Textures(u64),
}

impl Why {
    /// Whether this makes a match certain: the pack names that listing itself.
    const fn is_certain(&self) -> bool {
        matches!(self, Self::Linked(_) | Self::SameTitle(_))
    }
}

impl std::fmt::Display for Why {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Linked(place) => tr!("server-store-why-linked", place = place.text()),
            Self::SameTitle(place) => tr!("server-store-why-same-title", place = place.text()),
            Self::By { author, place } => tr!("server-store-why-by", author = author, place = place.text()),
            Self::Meshes(count) => tr!("server-store-why-meshes", count = count),
            Self::Textures(count) => tr!("server-store-why-textures", count = count),
        })
    }
}

impl Serialize for Why {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl StoreMatch {
    pub(crate) fn store(&self) -> &'static str {
        self.store
    }
    pub(crate) fn url(&self) -> &str {
        &self.url
    }
    pub(crate) fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }
    pub(crate) fn license(&self) -> Option<&'static str> {
        self.license
    }
    /// Whether the pack names this listing: it links to it, or has exactly its title in a store's file, or the
    /// listing has exactly the pack's name and is sold by the seller the pack's files name.
    fn is_certain(&self) -> bool {
        let named_by_seller = self.named && self.why.iter().any(|w| matches!(w, Why::By { .. }));
        (self.score - 1.0).abs() < f32::EPSILON && (named_by_seller || self.why.iter().any(Why::is_certain))
    }
}

impl StoreMatches {
    /// The one listing the pack names itself (its README links to it, it has the exact title of the pack's
    /// Megascans file, or it has exactly the pack's name and the seller its files name). None if no listing, or more
    /// than one, does.
    pub(crate) fn certain(&self) -> Option<&StoreMatch> {
        let mut sure = self.matches.iter().filter(|m| m.is_certain());
        let first = sure.next()?;
        sure.next().is_none().then_some(first)
    }

    /// Every store's search failed: likely offline.
    pub(crate) fn all_failed(&self) -> bool {
        self.failed.len() == 4 && self.matches.is_empty()
    }
}

/// `GET /api/packs/{id}/store-matches?q=&fresh=`: likely store pages for a pack, best first. `q` defaults to the pack's
/// name. Without `q` or `fresh`, what the last lookup found is shown if there is one; a new search is kept for next time.
pub(crate) async fn store_matches(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(mq): Query<MatchQuery>,
) -> ApiResult<Json<Value>> {
    let typed = mq.q.as_deref().map(str::trim).filter(|q| !q.is_empty());
    if typed.is_none() && !mq.fresh {
        let id2 = id.clone();
        let kept: Option<String> = db::call(&state.pool, move |c| {
            Ok(c.query_row("SELECT store_matches FROM pack_context WHERE pack_id = ?1", [&id2], |r| r.get(0))
                .optional()?
                .flatten())
        })
        .await?;
        if let Some(v) = kept.and_then(|k| serde_json::from_str::<Value>(&k).ok()) {
            return Ok(Json(v));
        }
    }
    let found = find_matches(&state, &id, typed).await?;
    let json = serde_json::to_value(&found)?;
    if typed.is_none() && !found.all_failed() {
        let (id2, text) = (id.clone(), json.to_string());
        db::call(&state.pool, move |c| {
            c.execute(
                "INSERT INTO pack_context (pack_id, store_matches) VALUES (?1, ?2)
                 ON CONFLICT (pack_id) DO UPDATE SET store_matches = excluded.store_matches",
                rusqlite::params![id2, text],
            )?;
            Ok(())
        })
        .await?;
    }
    Ok(Json(json))
}

/// Search the stores for a pack: for `typed`, or for what the pack's name and files say.
pub(crate) async fn find_matches(state: &AppState, id: &str, typed: Option<&str>) -> ApiResult<StoreMatches> {
    let id2 = id.to_owned();
    let found = db::call(&state.pool, move |c| {
        let Some(pack) = db::get_pack(c, &id2)? else { return Ok(None) };
        let readme = db::pack_context(c, &id2)?.readme;
        let mut stmt = c.prepare(
            "SELECT COALESCE(o.path, a.path) FROM assets a LEFT JOIN asset_origin o ON o.asset_id = a.id
             WHERE a.pack_id = ?1 LIMIT ?2",
        )?;
        let paths = stmt
            .query_map(rusqlite::params![id2, MAX_PATHS], |r| r.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some((pack, readme, paths)))
    })
    .await?;
    let (pack, readme, paths) = found.ok_or_else(|| AppError::not_found(tr!("error-pack-not-found", id = id)))?;
    let mut clues = clues(&pack.name, readme.as_deref(), &paths, &pack.kinds);
    clues.add_sidecars(&sidecar::pack_sidecars(state, id).await?);
    let query: String = match (typed, &clues.title) {
        (Some(q), _) => q.split_whitespace().collect::<Vec<_>>().join(" "),
        (None, Some((title, _))) => title.clone(),
        (None, None) => search_terms(&pack.name),
    }
    .chars()
    .take(MAX_QUERY)
    .collect();
    if query.is_empty() {
        return Err(AppError::bad_request(tr!("error-store-search-empty")));
    }
    let mut all = queries(&query, typed.is_some(), &clues);
    // With an exact title in hand the pack's name is the fallback, searched second.
    let named = search_terms(&pack.name);
    if typed.is_none() && clues.title.is_some() && !named.is_empty() && !all.iter().any(|q| same_title(q, &named)) {
        all.insert(1, named);
    }

    let http = crate::context::web();
    let fab = join_all(all.iter().take(FAB_QUERIES).enumerate().map(|(i, q)| search_fab(http, q, i == 0)));
    let others = join_all(all.iter().take(OTHER_QUERIES).flat_map(|q| {
        [
            search(http, "itch.io", url_with("https://itch.io/search", "q", q), "text/html", |b| Ok(parse_itch(b))),
            search(http, "Kenney", url_with("https://kenney.nl/assets", "search", q), "text/html", |b| {
                Ok(parse_kenney(b))
            }),
            search(
                http,
                "OpenGameArt",
                url_with("https://opengameart.org/art-search-advanced", "keys", q),
                "text/html",
                |b| Ok(parse_opengameart(b)),
            ),
        ]
    }));
    let pages = join_all(clues.links.iter().map(|l| linked_page(http, l)));
    let (fab, others, pages) = tokio::join!(fab, others, pages);

    let mut matches = Vec::new();
    let mut tried: HashMap<&'static str, (usize, usize)> = HashMap::new();
    let stores = std::iter::repeat_n("Fab", fab.len())
        .chain(["itch.io", "Kenney", "OpenGameArt"].into_iter().cycle().take(others.len()));
    for (store, found) in stores.zip(fab.into_iter().chain(others)) {
        let t = tried.entry(store).or_default();
        t.0 += 1;
        match found {
            Ok(found) => matches.extend(found),
            Err(e) => {
                t.1 += 1;
                tracing::info!(store, error = %format!("{e:#}"), "store search failed");
            }
        }
    }
    let mut failed: Vec<&'static str> = ["Fab", "itch.io", "Kenney", "OpenGameArt"]
        .into_iter()
        .filter(|s| tried.get(s).is_some_and(|(n, bad)| n == bad))
        .collect();
    failed.dedup();
    // A page the pack links to is as good as it gets; if a search turned it up too, that one carries the details.
    for page in pages.into_iter().flatten() {
        match matches.iter_mut().find(|m| same_page(&m.url, &page.url)) {
            Some(m) => {
                m.why.splice(0..0, page.why);
                m.license = m.license.or(page.license);
            }
            None => matches.push(page),
        }
    }
    let matches = rank(&query, &clues, matches);
    let links = vec![
        StoreLink {
            store: "Unity Asset Store",
            url: format!("https://assetstore.unity.com/search#q={}", percent_encode(&query)),
        },
        StoreLink {
            store: "Humble Bundle",
            url: url_with("https://www.humblebundle.com/store/search", "search", &query),
        },
    ];
    Ok(StoreMatches {
        searched_at: db::now(),
        query,
        also: all.into_iter().skip(1).collect(),
        clues: clues.describe(),
        matches,
        links,
        failed,
    })
}

async fn search(
    http: &reqwest::Client,
    store: &'static str,
    url: String,
    accept: &str,
    parse: fn(&str) -> Result<Vec<StoreMatch>>,
) -> Result<Vec<StoreMatch>> {
    let body = tokio::time::timeout(SEARCH_TIMEOUT, fetch_page(http, &url, accept))
        .await
        .with_context(|| format!("{store} took too long"))??;
    parse(&body)
}

/// One Fab search; with `deep`, its second page of results too.
async fn search_fab(http: &reqwest::Client, q: &str, deep: bool) -> Result<Vec<StoreMatch>> {
    let url = url_with("https://www.fab.com/i/listings/search", "q", q);
    let body = tokio::time::timeout(SEARCH_TIMEOUT, fetch_page(http, &url, "application/json"))
        .await
        .context("Fab took too long")??;
    let (mut found, next) = parse_fab_page(&body)?;
    if deep && let Some(cursor) = next {
        let more =
            reqwest::Url::parse_with_params("https://www.fab.com/i/listings/search", [("q", q), ("cursor", &cursor)])
                .map(String::from)
                .context("Fab's next page")?;
        if let Ok(Ok(body)) = tokio::time::timeout(SEARCH_TIMEOUT, fetch_page(http, &more, "application/json")).await
            && let Ok((page, _)) = parse_fab_page(&body)
        {
            found.extend(page);
        }
    }
    Ok(found)
}

/// A store page the pack links to. A store's file gives its title; a README's link is fetched for it (and named by
/// its address if it can't be).
async fn linked_page(http: &reqwest::Client, link: &Link) -> Option<StoreMatch> {
    let url = link.url.as_str();
    let title = match &link.title {
        Some(t) => t.clone(),
        None => match tokio::time::timeout(SEARCH_TIMEOUT, fetch_page(http, url, "text/html")).await {
            Ok(Ok(html)) => html.find("<title").and_then(|i| {
                let rest = &html[i..];
                let open = rest.find('>')?;
                Some(inner_text(&rest[open + 1..], "</title>").trim().to_owned())
            }),
            _ => None,
        }
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| url.to_owned()),
    };
    let mut m = found(link.store, &title, url.replace("http://", "https://"))?;
    m.author.clone_from(&link.author);
    m.license = link.license.as_deref().and_then(crate::credits::license_name);
    m.why.push(Why::Linked(link.from.clone()));
    Some(m)
}

/// The same page, ignoring the scheme, `www.`, a trailing slash and the query.
fn same_page(a: &str, b: &str) -> bool {
    let norm = |u: &str| {
        let u = u.split_once("://").map_or(u, |(_, r)| r);
        let u = u.strip_prefix("www.").unwrap_or(u);
        u.split(['?', '#']).next().unwrap_or(u).trim_end_matches('/').to_ascii_lowercase()
    };
    norm(a) == norm(b)
}

fn url_with(base: &str, key: &str, value: &str) -> String {
    reqwest::Url::parse_with_params(base, [(key, value)]).map_or_else(|_| base.to_owned(), String::from)
}

pub(crate) fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(char::from(b));
        } else {
            let _ = write!(out, "%{b:02X}");
        }
    }
    out
}

fn clip(s: &str, max: usize) -> String {
    s.trim().chars().take(max).collect()
}

fn found(store: &'static str, title: &str, url: String) -> Option<StoreMatch> {
    let title = clip(&decode_entities(title), 200);
    (!title.is_empty() && url.starts_with("https://")).then_some(StoreMatch {
        store,
        title,
        url,
        author: None,
        summary: None,
        thumb: None,
        price: None,
        license: None,
        score: 0.0,
        why: Vec::new(),
        formats: Vec::new(),
        specs: None,
        named: false,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::store_search::clues::Clues;
    use crate::store_search::pages::fab_license;
    use crate::store_search::rank::squash;

    /// What a lookup gives, holding these.
    pub(crate) fn matches(matches: Vec<StoreMatch>) -> StoreMatches {
        StoreMatches {
            searched_at: String::new(),
            query: String::new(),
            also: Vec::new(),
            clues: Vec::new(),
            matches,
            links: Vec::new(),
            failed: Vec::new(),
        }
    }

    /// A Fab result, as a lookup finds it.
    pub(crate) fn fab_match(title: &str, author: &str, licenses: &[&str]) -> StoreMatch {
        let mut m = found("Fab", title, format!("https://www.fab.com/listings/{}", squash(title))).unwrap();
        m.author = Some(author.to_owned());
        let licenses: Vec<Value> = licenses.iter().map(|l| serde_json::json!({ "name": l })).collect();
        m.license = fab_license(&Value::Array(licenses));
        m
    }

    #[tokio::test]
    async fn a_sketchfab_file_links_to_its_page() {
        let text = include_str!("../../formats/samples/sketchfab/license.txt");
        let mut c = Clues::default();
        c.add_sidecars(&[sidecar::read("Lantern/license.txt", text).unwrap()]);
        assert_eq!(c.title, Some(("Brass Lantern".to_owned(), Place::File("Sketchfab file".to_owned()))));
        assert_eq!(c.describe().last().map(String::as_str), Some("The Sketchfab file links to Sketchfab"));

        // Its page needn't be fetched: the file gives the title, the author and the license.
        let page = linked_page(&reqwest::Client::new(), &c.links[0]).await.unwrap();
        assert_eq!(
            (page.title.as_str(), page.author.as_deref(), page.license),
            ("Brass Lantern", Some("Example Artist"), Some(CC_BY_4))
        );
        let found = matches(rank("lantern", &c, vec![page]));
        assert_eq!(
            found.certain().map(StoreMatch::url),
            Some("https://sketchfab.com/3d-models/brass-lantern-0123456789abcdef0123456789abcdef")
        );
    }

    #[test]
    fn encodes_links() {
        assert_eq!(percent_encode("forest & trees"), "forest%20%26%20trees");
        assert_eq!(
            url_with("https://www.humblebundle.com/store/search", "search", "a b"),
            "https://www.humblebundle.com/store/search?search=a+b"
        );
    }
}
