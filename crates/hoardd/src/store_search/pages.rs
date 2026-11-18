//! Reading the stores' search results: Fab's JSON, and the HTML of `itch.io`, Kenney and `OpenGameArt`.

use anyhow::{Context as _, Result};
use serde_json::Value;

use super::{CC_BY_4, CC0, FAB_STANDARD, StoreMatch, clip, found};
use crate::context::{attr, decode_entities};

/// A page of Fab's listing search, which answers in JSON, and the cursor for the next page.
pub(super) fn parse_fab_page(body: &str) -> Result<(Vec<StoreMatch>, Option<String>)> {
    let v: Value = serde_json::from_str(body).context("Fab's answer isn't JSON")?;
    let results = v["results"].as_array().context("Fab's answer has no results")?;
    let next =
        v["cursors"]["next"].as_str().or_else(|| v["next"].as_str()).filter(|c| c.len() < 200).map(str::to_owned);
    let found = results
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
            m.license = fab_license(&r["licenses"]);
            for f in r["assetFormats"].as_array().into_iter().flatten() {
                if let Some(code) = f["assetFormatType"]["code"].as_str() {
                    m.formats.push(code.to_owned());
                }
                if let Some(details) = f["technicalSpecs"]["technicalDetails"].as_str() {
                    let text = plain_text(&details.replace("</p>", "\n").replace("<br", "\n<br"));
                    m.specs = Some(m.specs.take().map_or_else(|| text.clone(), |s| s + "\n" + &text));
                }
            }
            Some(m)
        })
        .collect();
    Ok((found, next))
}

/// The license a Fab listing is sold under, from its `licenses` (`CC-BY`, or the `Personal` and `Professional` tiers).
/// A Creative Commons license is named first: it's the one that asks for credit.
pub(super) fn fab_license(licenses: &Value) -> Option<&'static str> {
    let names: Vec<&str> = licenses.as_array()?.iter().filter_map(|l| l["name"].as_str()).collect();
    let has = |want: &str| names.iter().any(|n| n.eq_ignore_ascii_case(want));
    if has("CC-BY") {
        Some(CC_BY_4)
    } else if has("CC0") {
        Some(CC0)
    } else if has("Personal") || has("Professional") {
        Some(FAB_STANDARD)
    } else {
        None
    }
}

pub(super) fn parse_itch(html: &str) -> Vec<StoreMatch> {
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

pub(super) fn parse_kenney(html: &str) -> Vec<StoreMatch> {
    chunks(html, "div", "asset")
        .into_iter()
        .filter_map(|cell| {
            let h2 = cell.find("<h2")?;
            let (a, rest) = first_tag(&cell[h2..], "a")?;
            let mut m = found("Kenney", &inner_text(rest, "</a>"), attr(a, "href")?)?;
            m.author = Some("Kenney".to_owned());
            m.price = Some("Free".to_owned());
            // Everything on kenney.nl is CC0.
            m.license = Some(CC0);
            m.thumb = cell.split_once("url(").and_then(|(_, rest)| {
                let u = rest.split(')').next()?.trim_matches(|c| matches!(c, '"' | '\'' | ' '));
                u.starts_with("https://").then(|| u.to_owned())
            });
            Some(m)
        })
        .collect()
}

pub(super) fn parse_opengameart(html: &str) -> Vec<StoreMatch> {
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
pub(super) fn inner_text(html: &str, close: &str) -> String {
    plain_text(html.find(close).map_or(html, |end| &html[..end]))
}

/// HTML as plain text, without tags.
fn plain_text(html: &str) -> String {
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
    fn reads_fab_results() {
        let body = r#"{"results":[{"title":"Medieval Village","uid":"f89fe420-1942","isFree":false,
            "startingPrice":{"price":29.99},"user":{"sellerName":"Marco"},"category":{"name":"Medieval"},
            "thumbnails":[{"images":[{"url":"https://media.fab.com/s.jpg","width":160},
            {"url":"https://media.fab.com/m.jpg","width":320}]}]},
            {"title":"Bad","uid":"../x"}],"cursors":{"next":"bz0yNA==","previous":null}}"#;
        let (found, next) = parse_fab_page(body).unwrap();
        assert_eq!(next.as_deref(), Some("bz0yNA=="));
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
}
