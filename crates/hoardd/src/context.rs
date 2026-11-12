//! Pack context: what the seller says about a pack. Store pages and READMEs often explain files better than their
//! names do ("120 ice and fire spells for RPGs" says a lot about `spell35.wav`), so the LLM tagger sees this text
//! with every batch.

use std::fmt::Write as _;
use std::fs::File;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context as _, Result};
use futures_util::StreamExt as _;

/// Longest store description kept.
pub(crate) const MAX_STORE_TEXT: usize = 6000;
/// Longest README text kept (all READMEs together).
const MAX_README: usize = 6000;
const MAX_PAGE_BYTES: usize = 2 << 20;
const FETCH_TIMEOUT: Duration = Duration::from_secs(12);

/// Marketplaces recognised from a URL's host.
const STORES: &[(&str, &str)] = &[
    ("fab.com", "Fab"),
    ("unrealengine.com", "Unreal Marketplace"),
    ("assetstore.unity.com", "Unity Asset Store"),
    ("itch.io", "itch.io"),
    ("humblebundle.com", "Humble Bundle"),
    ("gumroad.com", "Gumroad"),
    ("artstation.com", "ArtStation"),
    ("kenney.nl", "Kenney"),
    ("opengameart.org", "OpenGameArt"),
    ("sonniss.com", "Sonniss"),
    ("asoundeffect.com", "A Sound Effect"),
    ("prosoundeffects.com", "Pro Sound Effects"),
    ("craftpix.net", "CraftPix"),
    ("gamedevmarket.net", "GameDev Market"),
    ("sketchfab.com", "Sketchfab"),
    ("cgtrader.com", "CGTrader"),
    ("turbosquid.com", "TurboSquid"),
    ("polyhaven.com", "Poly Haven"),
    ("ambientcg.com", "ambientCG"),
    ("quixel.com", "Quixel"),
    ("synty", "Synty"),
];

/// The marketplace a URL belongs to, if it's one we know.
pub(crate) fn store_from_url(url: &str) -> Option<&'static str> {
    let host = url.split("://").nth(1)?.split(['/', '?', '#']).next()?.to_ascii_lowercase();
    STORES.iter().find(|(h, _)| host == *h || host.ends_with(&format!(".{h}")) || host.contains(h)).map(|(_, s)| *s)
}

/// The first store link mentioned in some text (a README often links back to its store page).
pub(crate) fn store_link(text: &str) -> Option<(&'static str, String)> {
    text.split(|c: char| c.is_whitespace() || matches!(c, '<' | '>' | '(' | ')' | '"' | '\'' | '[' | ']'))
        .filter(|w| w.starts_with("https://") || w.starts_with("http://"))
        .find_map(|w| {
            let url = w.trim_end_matches(['.', ',', ';', ':']);
            store_from_url(url).map(|s| (s, url.to_owned()))
        })
}

/// Is this pack file a README-like text worth keeping as context?
fn is_readme(rel: &str) -> bool {
    let name = rel.rsplit('/').next().unwrap_or(rel).to_ascii_lowercase();
    let depth = rel.matches('/').count();
    let stem = name.rsplit_once('.').map_or(name.as_str(), |(s, _)| s);
    let ext = name.rsplit_once('.').map_or("", |(_, e)| e);
    depth <= 1
        && matches!(ext, "txt" | "md" | "rtf" | "")
        && ["readme", "read me", "read_me", "about", "info", "description", "contents", "credits", "notes"]
            .iter()
            .any(|k| stem.starts_with(k))
}

/// Text of the pack's READMEs (top two levels), capped.
pub(crate) fn read_readmes(root: &Path, files: &[PathBuf]) -> Option<String> {
    let mut out = String::new();
    let mut picked: Vec<(String, &PathBuf)> = files
        .iter()
        .filter_map(|f| {
            let rel = f.strip_prefix(root).ok()?.to_string_lossy().replace('\\', "/");
            is_readme(&rel).then_some((rel, f))
        })
        .collect();
    picked.sort_by_key(|(rel, _)| (rel.matches('/').count(), rel.clone()));
    for (rel, f) in picked {
        let mut bytes = Vec::new();
        if File::open(f).and_then(|h| h.take(MAX_README as u64).read_to_end(&mut bytes)).is_err() {
            continue;
        }
        let text = String::from_utf8_lossy(&bytes);
        let text = text.trim();
        if text.is_empty() {
            continue;
        }
        let _ = writeln!(out, "--- {rel}\n{text}");
        if out.len() >= MAX_README {
            break;
        }
    }
    let out: String = out.chars().take(MAX_README).collect();
    (!out.is_empty()).then_some(out)
}

/// Fetch the store page's text for a pack that has a store link but no text yet (the link was found in a README,
/// or the fetch failed before). Never fails: a pack without its store blurb is still a pack.
pub(crate) async fn fill_store_text(state: &crate::AppState, pack_id: &str) {
    let pid = pack_id.to_owned();
    let url = crate::db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT store_url FROM pack_context WHERE pack_id = ?1 AND store_text IS NULL AND store_url IS NOT NULL",
            [&pid],
            |r| r.get::<_, String>(0),
        )
        .ok())
    })
    .await
    .ok()
    .flatten();
    let Some(url) = url else { return };
    let text = match fetch_store_text(&state.http, &url).await {
        Ok(Some(text)) => text.chars().take(MAX_STORE_TEXT).collect::<String>(),
        Ok(None) => return,
        Err(e) => {
            tracing::info!(url, error = %format!("{e:#}"), "couldn't fetch store page");
            return;
        }
    };
    let pid = pack_id.to_owned();
    let saved = crate::db::call(&state.pool, move |c| {
        c.execute(
            "UPDATE pack_context SET store_text = ?2 WHERE pack_id = ?1 AND store_text IS NULL",
            rusqlite::params![pid, text],
        )?;
        Ok(())
    })
    .await;
    if let Err(e) = saved {
        tracing::warn!(pack_id, error = %format!("{e:#}"), "couldn't save store page text");
    }
}

/// Fetch a store page and pull out its title and description (`<title>`, `description`, `og:description`).
pub(crate) async fn fetch_store_text(http: &reqwest::Client, url: &str) -> Result<Option<String>> {
    let resp = http
        .get(url)
        .header(reqwest::header::ACCEPT, "text/html")
        .header(reqwest::header::USER_AGENT, hoard_core::user_agent())
        .timeout(FETCH_TIMEOUT)
        .send()
        .await?
        .error_for_status()?;
    let mut body = Vec::new();
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        body.extend_from_slice(&chunk.context("read store page")?);
        if body.len() >= MAX_PAGE_BYTES {
            break;
        }
    }
    Ok(page_summary(&String::from_utf8_lossy(&body)))
}

/// Title and description of an HTML page as plain text.
fn page_summary(html: &str) -> Option<String> {
    let title = between(html, "<title", "</title>").map(|t| t.split_once('>').map_or(t, |(_, t)| t));
    let mut descs: Vec<String> = Vec::new();
    for tag in html.split("<meta").skip(1).map(|t| t.split('>').next().unwrap_or("")) {
        let key = attr(tag, "name").or_else(|| attr(tag, "property")).unwrap_or_default().to_ascii_lowercase();
        if matches!(key.as_str(), "description" | "og:description" | "twitter:description")
            && let Some(c) = attr(tag, "content").map(|c| decode_entities(&c))
            && !descs.iter().any(|d| d == &c)
        {
            descs.push(c);
        }
    }
    let mut out = String::new();
    if let Some(t) = title.map(|t| decode_entities(t.trim())).filter(|t| !t.is_empty()) {
        out.push_str(&t);
        out.push('\n');
    }
    // Longest description first: og:description is often the full blurb, `description` a trimmed copy.
    descs.sort_by_key(|d| std::cmp::Reverse(d.len()));
    if let Some(d) = descs.first() {
        out.push_str(d);
    }
    let out: String = out.trim().chars().take(MAX_STORE_TEXT).collect();
    (!out.is_empty()).then_some(out)
}

fn between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let lower = s.to_ascii_lowercase();
    let a = lower.find(open)? + open.len();
    let b = a + lower[a..].find(close)?;
    s.get(a..b)
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let lower = tag.to_ascii_lowercase();
    let mut from = 0;
    while let Some(i) = lower[from..].find(name) {
        let at = from + i;
        from = at + name.len();
        let before_ok = at == 0 || lower.as_bytes()[at - 1].is_ascii_whitespace();
        let rest = tag[at + name.len()..].trim_start();
        if !before_ok || !rest.starts_with('=') {
            continue;
        }
        let rest = rest[1..].trim_start();
        let quote = rest.chars().next()?;
        return Some(if matches!(quote, '"' | '\'') {
            rest[1..].split(quote).next().unwrap_or("").to_owned()
        } else {
            rest.split(|c: char| c.is_whitespace() || c == '/').next().unwrap_or("").to_owned()
        });
    }
    None
}

fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let Some(end) = rest[..rest.len().min(12)].find(';') else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let ent = &rest[1..end];
        let ch = match ent {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            _ => ent
                .strip_prefix("#x")
                .or_else(|| ent.strip_prefix("#X"))
                .and_then(|h| u32::from_str_radix(h, 16).ok())
                .or_else(|| ent.strip_prefix('#').and_then(|d| d.parse().ok()))
                .and_then(char::from_u32),
        };
        if let Some(c) = ch {
            out.push(c);
            rest = &rest[end + 1..];
        } else {
            out.push('&');
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_store_page() {
        let html = r#"<html><head><TITLE>Ice &amp; Fire Spells | Fab</TITLE>
            <meta name="description" content="120 spells.">
            <meta property='og:description' content="120 ice and fire spell sounds for RPGs &#8211; 24-bit WAV.">
            </head></html>"#;
        assert_eq!(
            page_summary(html).unwrap(),
            "Ice & Fire Spells | Fab\n120 ice and fire spell sounds for RPGs \u{2013} 24-bit WAV."
        );
    }

    #[test]
    fn recognises_stores() {
        assert_eq!(store_from_url("https://www.fab.com/listings/abc"), Some("Fab"));
        assert_eq!(store_from_url("https://kenney.itch.io/pack"), Some("itch.io"));
        assert_eq!(store_from_url("https://example.com/x"), None);
        assert_eq!(
            store_link("Thanks for buying! More at (https://www.humblebundle.com/store/x)."),
            Some(("Humble Bundle", "https://www.humblebundle.com/store/x".to_owned()))
        );
        assert!(is_readme("README.txt") && is_readme("Pack/Read Me.md") && !is_readme("a/b/readme.txt"));
        assert!(!is_readme("music/theme.txt"));
    }
}
