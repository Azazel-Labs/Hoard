//! Looking a pack up by name in the stores it may have come from, so its store details can be filled in with a click.
//!
//! Fab, `itch.io`, Kenney and `OpenGameArt` are searched from the server. Humble and the Unity Asset Store can't be (one
//! refuses non-browser clients, the other searches in the page's script), so for those the UI only gets search links.

use std::fmt::Write as _;
use std::time::Duration;

use anyhow::{Context as _, Result};
use axum::Json;
use axum::extract::{Path, Query, State};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::{attr, decode_entities, fetch_page};
use crate::error::{ApiResult, AppError};
use crate::{AppState, db};

/// Results kept from each store before ranking.
const PER_STORE: usize = 5;
const MAX_MATCHES: usize = 12;
const SEARCH_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_QUERY: usize = 120;

/// Words in pack names that say where a pack is from or what it's for, not what it is.
const NOISE: &[&str] = &[
    "unity",
    "unreal",
    "ue",
    "ue4",
    "ue5",
    "engine",
    "godot",
    "fab",
    "itch",
    "itchio",
    "humble",
    "kenney",
    "opengameart",
    "zip",
    "rar",
    "7z",
    "final",
    "copy",
];

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
    score: f32,
}

#[derive(Debug, Serialize)]
pub(crate) struct StoreLink {
    store: &'static str,
    url: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct StoreMatches {
    query: String,
    matches: Vec<StoreMatch>,
    /// Stores that can only be searched in the browser.
    links: Vec<StoreLink>,
    /// Stores whose search failed this time.
    failed: Vec<&'static str>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MatchQuery {
    q: Option<String>,
}

/// `GET /api/packs/{id}/store-matches?q=`: likely store pages for a pack, best first. `q` defaults to the pack's name.
pub(crate) async fn store_matches(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(mq): Query<MatchQuery>,
) -> ApiResult<Json<StoreMatches>> {
    let id2 = id.clone();
    let pack = db::call(&state.pool, move |c| db::get_pack(c, &id2))
        .await?
        .ok_or_else(|| AppError::not_found(format!("pack {id}")))?;
    let query: String = match mq.q.as_deref().map(str::trim).filter(|q| !q.is_empty()) {
        Some(q) => q.split_whitespace().collect::<Vec<_>>().join(" "),
        None => search_terms(&pack.name),
    }
    .chars()
    .take(MAX_QUERY)
    .collect();
    if query.is_empty() {
        return Err(AppError::bad_request("nothing to search for; type the pack's name"));
    }

    let http = &state.http;
    let (fab, itch, kenney, oga) = tokio::join!(
        search(
            http,
            "Fab",
            url_with("https://www.fab.com/i/listings/search", "q", &query),
            "application/json",
            parse_fab
        ),
        search(http, "itch.io", url_with("https://itch.io/search", "q", &query), "text/html", |b| Ok(parse_itch(b))),
        search(http, "Kenney", url_with("https://kenney.nl/assets", "search", &query), "text/html", |b| Ok(
            parse_kenney(b)
        )),
        search(
            http,
            "OpenGameArt",
            url_with("https://opengameart.org/art-search-advanced", "keys", &query),
            "text/html",
            |b| Ok(parse_opengameart(b)),
        ),
    );
    let mut matches = Vec::new();
    let mut failed = Vec::new();
    for (store, found) in [("Fab", fab), ("itch.io", itch), ("Kenney", kenney), ("OpenGameArt", oga)] {
        match found {
            Ok(found) => matches.extend(found.into_iter().take(PER_STORE)),
            Err(e) => {
                tracing::info!(store, error = %format!("{e:#}"), "store search failed");
                failed.push(store);
            }
        }
    }
    let matches = rank(&query, matches);
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
    Ok(Json(StoreMatches { query, matches, links, failed }))
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

/// The words of a pack name worth searching for: separators become spaces, `CamelCase` is split, and version
/// numbers and engine or store names are dropped.
pub(crate) fn search_terms(name: &str) -> String {
    let mut spaced = String::with_capacity(name.len() + 8);
    let mut prev: Option<char> = None;
    for c in name.chars() {
        if !c.is_alphanumeric() && c != '\'' {
            spaced.push(' ');
        } else {
            if c.is_uppercase() && prev.is_some_and(char::is_lowercase) {
                spaced.push(' ');
            }
            spaced.push(c);
        }
        prev = Some(c);
    }
    spaced
        .split_whitespace()
        .filter(|w| {
            let lower = w.to_lowercase();
            let version =
                lower.strip_prefix('v').is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
            !version && !lower.chars().all(|c| c.is_ascii_digit()) && !NOISE.contains(&lower.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn words(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for w in s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
        let mut w = w.to_lowercase();
        // "trees" finds "tree".
        if w.len() > 3 && w.ends_with('s') && !w.ends_with("ss") {
            w.pop();
        }
        if !out.contains(&w) {
            out.push(w);
        }
    }
    out
}

/// How well a title matches the query, 0 to 1: mostly how many of the query's words it has, a little how few
/// other words.
fn score(query: &[String], title: &str) -> f32 {
    let title = words(title);
    if query.is_empty() || title.is_empty() {
        return 0.0;
    }
    #[expect(clippy::cast_precision_loss, reason = "word counts are tiny")]
    let (hits, q, t) =
        (query.iter().filter(|w| title.contains(w)).count() as f32, query.len() as f32, title.len() as f32);
    0.7 * hits / q + 0.3 * hits / t
}

/// Matches that share a word with the query, best first.
fn rank(query: &str, matches: Vec<StoreMatch>) -> Vec<StoreMatch> {
    let query = words(query);
    let mut kept: Vec<StoreMatch> = matches
        .into_iter()
        .map(|mut m| {
            m.score = (score(&query, &m.title) * 100.0).round() / 100.0;
            m
        })
        .filter(|m| m.score > 0.0)
        .collect();
    kept.sort_by(|a, b| b.score.total_cmp(&a.score));
    kept.truncate(MAX_MATCHES);
    kept
}

fn url_with(base: &str, key: &str, value: &str) -> String {
    reqwest::Url::parse_with_params(base, [(key, value)]).map_or_else(|_| base.to_owned(), String::from)
}

fn percent_encode(s: &str) -> String {
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
        score: 0.0,
    })
}

/// Fab's listing search, which answers in JSON.
fn parse_fab(body: &str) -> Result<Vec<StoreMatch>> {
    let v: Value = serde_json::from_str(body).context("Fab's answer isn't JSON")?;
    let results = v["results"].as_array().context("Fab's answer has no results")?;
    Ok(results
        .iter()
        .filter_map(|r| {
            let uid = r["uid"].as_str().filter(|u| u.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'))?;
            let mut m = found("Fab", r["title"].as_str()?, format!("https://www.fab.com/listings/{uid}"))?;
            m.author = r["user"]["sellerName"].as_str().map(|s| clip(s, 120));
            m.summary = r["category"]["name"].as_str().map(|s| clip(s, 120));
            m.price = if r["isFree"].as_bool() == Some(true) {
                Some("Free".to_owned())
            } else {
                r["startingPrice"]["price"].as_f64().map(|p| format!("${p:.2}"))
            };
            let images = r["thumbnails"][0]["images"].as_array();
            let image = images
                .and_then(|i| i.iter().find(|i| i["width"].as_u64().is_some_and(|w| w >= 320)).or_else(|| i.last()));
            m.thumb = image.and_then(|i| i["url"].as_str()).filter(|u| u.starts_with("https://")).map(str::to_owned);
            Some(m)
        })
        .collect())
}

fn parse_itch(html: &str) -> Vec<StoreMatch> {
    chunks(html, "div", "game_cell")
        .into_iter()
        .filter_map(|cell| {
            let (_, after) = first_with_class(cell, "div", "game_title")?;
            let (a, rest) = first_tag(after, "a")?;
            let mut m = found("itch.io", &inner_text(rest, "</a>"), attr(a, "href")?)?;
            m.summary = first_with_class(cell, "div", "game_text")
                .and_then(|(tag, _)| attr(tag, "title"))
                .map(|s| clip(&decode_entities(&s), 300))
                .filter(|s| !s.is_empty());
            m.author = first_with_class(cell, "div", "game_author")
                .and_then(|(_, after)| first_tag(after, "a"))
                .map(|(_, rest)| clip(&inner_text(rest, "</a>"), 120))
                .filter(|s| !s.is_empty());
            m.price = first_with_class(cell, "div", "price_value")
                .map(|(_, rest)| clip(&inner_text(rest, "</div>"), 20))
                .filter(|s| !s.is_empty());
            m.thumb = first_tag(cell, "img")
                .and_then(|(img, _)| attr(img, "data-lazy_src").or_else(|| attr(img, "src")))
                .filter(|u| u.starts_with("https://"));
            Some(m)
        })
        .collect()
}

fn parse_kenney(html: &str) -> Vec<StoreMatch> {
    chunks(html, "div", "asset")
        .into_iter()
        .filter_map(|cell| {
            let h2 = cell.find("<h2")?;
            let (a, rest) = first_tag(&cell[h2..], "a")?;
            let mut m = found("Kenney", &inner_text(rest, "</a>"), attr(a, "href")?)?;
            m.author = Some("Kenney".to_owned());
            m.price = Some("Free".to_owned());
            m.thumb = cell.split_once("url(").and_then(|(_, rest)| {
                let u = rest.split(')').next()?.trim_matches(|c| matches!(c, '"' | '\'' | ' '));
                u.starts_with("https://").then(|| u.to_owned())
            });
            Some(m)
        })
        .collect()
}

fn parse_opengameart(html: &str) -> Vec<StoreMatch> {
    chunks(html, "span", "art-preview-title")
        .into_iter()
        .filter_map(|cell| {
            let (a, rest) = first_tag(cell, "a")?;
            let href = attr(a, "href")?;
            let url = if href.starts_with('/') { format!("https://opengameart.org{href}") } else { href };
            let mut m = found("OpenGameArt", &inner_text(rest, "</a>"), url)?;
            m.price = Some("Free".to_owned());
            m.thumb =
                first_tag(cell, "img").and_then(|(img, _)| attr(img, "src")).filter(|u| u.starts_with("https://"));
            Some(m)
        })
        .collect()
}

/// Opening tags `<name ...>` whose class list includes `class`, with their offsets.
fn with_class<'a>(html: &'a str, name: &str, class: &str) -> Vec<(usize, &'a str)> {
    let open = format!("<{name}");
    html.match_indices(&open)
        .filter_map(|(i, _)| {
            let rest = &html[i + open.len()..];
            if !rest.starts_with(|c: char| c.is_whitespace()) {
                return None;
            }
            let tag = &html[i..i + open.len() + rest.find('>')?];
            attr(tag, "class")?.split_whitespace().any(|c| c == class).then_some((i, tag))
        })
        .collect()
}

/// The page cut into pieces, each starting at one of the matching tags and running to the next.
fn chunks<'a>(html: &'a str, name: &str, class: &str) -> Vec<&'a str> {
    let starts: Vec<usize> = with_class(html, name, class).into_iter().map(|(i, _)| i).collect();
    starts.iter().enumerate().map(|(n, &a)| &html[a..starts.get(n + 1).copied().unwrap_or(html.len())]).collect()
}

/// The first matching tag and what follows it.
fn first_with_class<'a>(html: &'a str, name: &str, class: &str) -> Option<(&'a str, &'a str)> {
    let (i, tag) = *with_class(html, name, class).first()?;
    Some((tag, &html[i + tag.len() + 1..]))
}

/// The first `<name ...>` tag and what follows it.
fn first_tag<'a>(html: &'a str, name: &str) -> Option<(&'a str, &'a str)> {
    let open = format!("<{name}");
    let mut from = 0;
    while let Some(i) = html[from..].find(&open).map(|i| from + i) {
        let after = &html[i + open.len()..];
        if after.starts_with(|c: char| c.is_whitespace() || c == '>') {
            let end = i + open.len() + after.find('>')?;
            return Some((&html[i..end], &html[end + 1..]));
        }
        from = i + open.len();
    }
    None
}

/// Plain text up to `close`, without tags.
fn inner_text(html: &str, close: &str) -> String {
    let html = html.find(close).map_or(html, |end| &html[..end]);
    let mut text = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => text.push(c),
            _ => {}
        }
    }
    decode_entities(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_pack_names_into_search_terms() {
        assert_eq!(search_terms("MedievalVillage_v1_3"), "Medieval Village");
        assert_eq!(search_terms("POLYGON - Fantasy Kingdom (Unity 2020)"), "POLYGON Fantasy Kingdom");
        assert_eq!(search_terms("kenney_platformer-pack-redux"), "platformer pack redux");
        assert_eq!(search_terms("UE5 Stylized Trees V2"), "Stylized Trees");
    }

    #[test]
    fn ranks_by_shared_words() {
        let m = |store, title: &str| found(store, title, "https://example.com/x".to_owned()).unwrap();
        let ranked = rank(
            "medieval village",
            vec![
                m("Fab", "Space Station"),
                m("Fab", "Medieval Village Megakit Ultimate Edition"),
                m("itch.io", "Medieval Villages"),
            ],
        );
        let titles: Vec<&str> = ranked.iter().map(|m| m.title.as_str()).collect();
        assert_eq!(titles, ["Medieval Villages", "Medieval Village Megakit Ultimate Edition"]);
        assert!((ranked[0].score - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn reads_fab_results() {
        let body = r#"{"results":[{"title":"Medieval Village","uid":"f89fe420-1942","isFree":false,
            "startingPrice":{"price":29.99},"user":{"sellerName":"Marco"},"category":{"name":"Medieval"},
            "thumbnails":[{"images":[{"url":"https://media.fab.com/s.jpg","width":160},
            {"url":"https://media.fab.com/m.jpg","width":320}]}]},
            {"title":"Bad","uid":"../x"}]}"#;
        let found = parse_fab(body).unwrap();
        assert_eq!(found.len(), 1);
        let m = &found[0];
        assert_eq!(m.url, "https://www.fab.com/listings/f89fe420-1942");
        assert_eq!((m.price.as_deref(), m.author.as_deref()), (Some("$29.99"), Some("Marco")));
        assert_eq!(m.thumb.as_deref(), Some("https://media.fab.com/m.jpg"));
    }

    #[test]
    fn reads_itch_results() {
        let html = r#"<div class="game_grid_widget"><div dir="auto" data-game_id="1" class="game_cell has_cover">
            <div class="game_thumb"><a href="https://q.itch.io/mv"><img data-lazy_src="https://img.itch.zone/a.jpg"/></a>
            <div class="game_cell_tools"><a href="/g/q/mv/add">Add</a></div></div><div class="game_cell_data">
            <div class="game_title"><a href="https://q.itch.io/mv" class="title game_link">Medieval Village &amp; Co</a>
            <a class="price_tag"><div class="price_value">$2.50</div></a></div>
            <div title="300+ models" class="game_text">300+ models</div>
            <div class="game_author"><a href="https://q.itch.io">Quaternius</a></div></div></div>
            <div class="game_cell" data-game_id="2"><div class="game_title"><a href="https://b.itch.io/x">Other</a></div></div>"#;
        let found = parse_itch(html);
        assert_eq!(found.len(), 2);
        let m = &found[0];
        assert_eq!((m.title.as_str(), m.url.as_str()), ("Medieval Village & Co", "https://q.itch.io/mv"));
        assert_eq!((m.price.as_deref(), m.author.as_deref()), (Some("$2.50"), Some("Quaternius")));
        assert_eq!(m.summary.as_deref(), Some("300+ models"));
        assert_eq!(m.thumb.as_deref(), Some("https://img.itch.zone/a.jpg"));
        assert_eq!(found[1].price, None);
    }

    #[test]
    fn reads_kenney_results() {
        let html = r#"<div class='asset'>
            <a href='https://kenney.nl/assets/platformer-art-buildings'>
            <div class='cover' style='background-image:url("https://kenney.nl/media/sample.png")'></div></a>
            <h2><a href='https://kenney.nl/assets/platformer-art-buildings'>Platformer Art Buildings</a></h2>
            <span><a href='https://kenney.nl/assets/category:2D'>2D</a></span></div>"#;
        let found = parse_kenney(html);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].title, "Platformer Art Buildings");
        assert_eq!(found[0].url, "https://kenney.nl/assets/platformer-art-buildings");
        assert_eq!(found[0].thumb.as_deref(), Some("https://kenney.nl/media/sample.png"));
    }

    #[test]
    fn reads_opengameart_results() {
        let html = r#"<div class="views-row"><span class="art-preview-title"><a href="/content/lpc-village">[LPC] Village</a>
            </span></div><div><a href="/content/lpc-village"><img src='https://opengameart.org/thumb.png' /></a></div>"#;
        let found = parse_opengameart(html);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].url, "https://opengameart.org/content/lpc-village");
        assert_eq!(found[0].thumb.as_deref(), Some("https://opengameart.org/thumb.png"));
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
