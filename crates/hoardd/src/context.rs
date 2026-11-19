//! Pack context: what the seller says about a pack. Store pages and READMEs often explain files better than their
//! names do ("120 ice and fire spells for RPGs" says a lot about `spell35.wav`), so the LLM tagger sees this text
//! with every batch.

use std::fmt::Write as _;
use std::fs::File;
use std::io::Read as _;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::Duration;

use anyhow::{Context as _, Result};
use futures_util::StreamExt as _;

/// Longest store description kept.
pub(crate) const MAX_STORE_TEXT: usize = 6000;
/// Longest README text kept (all READMEs together).
const MAX_README: usize = 6000;
const MAX_PAGE_BYTES: usize = 2 << 20;
const FETCH_TIMEOUT: Duration = Duration::from_secs(12);

/// Marketplaces recognized from a URL's host.
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
    let url = reqwest::Url::parse(url).ok()?;
    let host = url.host_str()?.to_ascii_lowercase();
    STORES.iter().find(|(h, _)| host == *h || host.ends_with(&format!(".{h}"))).map(|(_, s)| *s)
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
    let text = match fetch_store_text(web(), &url).await {
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
            "UPDATE pack_context SET store_text = ?2, store_text_read = ?3 WHERE pack_id = ?1 AND store_text IS NULL",
            rusqlite::params![pid, text, STORE_READER],
        )?;
        Ok(())
    })
    .await;
    if let Err(e) = saved {
        tracing::warn!(pack_id, error = %format!("{e:#}"), "couldn't save store page text");
    }
}

/// Read a pack's store page again with this version's reader, replacing the text an older one wrote. Text from before
/// versions were kept is replaced only if it's still what the first reader made of the page; otherwise someone typed
/// or changed it, and it's theirs from then on. Returns false if the page couldn't be reached, to try again later.
pub(crate) async fn reread_store_text(state: &crate::AppState, pack_id: &str) -> Result<bool> {
    use rusqlite::OptionalExtension as _;
    let pid = pack_id.to_owned();
    let row = crate::db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT store_url, store_text, store_text_read FROM pack_context
             WHERE pack_id = ?1 AND store_text_read < ?2 AND store_url IS NOT NULL",
            rusqlite::params![pid, STORE_READER],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?, r.get::<_, i64>(2)?)),
        )
        .optional()?)
    })
    .await?;
    let Some((url, old, read)) = row else { return Ok(true) };
    let page = match fetch_page(web(), &url, "text/html").await {
        Ok(page) => Some(page),
        // Gone, or turned away: there's nothing better to read.
        Err(e)
            if e.downcast_ref::<reqwest::Error>()
                .and_then(reqwest::Error::status)
                .is_some_and(|s| s.is_client_error()) =>
        {
            None
        }
        Err(e) => {
            tracing::info!(url, error = %format!("{e:#}"), "couldn't read the store page again");
            return Ok(false);
        }
    };
    let fetched = read > 0
        || old.as_deref().zip(page.as_deref().and_then(first_summary)).is_some_and(|(old, first)| {
            let old = gist(old, usize::MAX);
            !old.is_empty() && gist(&first, usize::MAX).starts_with(&old)
        });
    let (text, read) = match page.as_deref().and_then(page_summary) {
        Some(new) if fetched => (Some(new), Some(STORE_READER)),
        _ if fetched => (old.clone(), Some(STORE_READER)),
        _ => (old.clone(), None),
    };
    let pid = pack_id.to_owned();
    crate::db::call(&state.pool, move |c| {
        // Unless it was changed meanwhile.
        c.execute(
            "UPDATE pack_context SET store_text = ?3, store_text_read = ?4 WHERE pack_id = ?1 AND store_text IS ?2",
            rusqlite::params![pid, old, text, read],
        )?;
        Ok(())
    })
    .await?;
    Ok(true)
}

/// Fetch a store page and pull out its title and description (`<title>`, `description`, `og:description`).
pub(crate) async fn fetch_store_text(http: &reqwest::Client, url: &str) -> Result<Option<String>> {
    Ok(page_summary(&fetch_page(http, url, "text/html").await?))
}

/// Is this an address on the internet, rather than this machine, the local network or something reserved?
pub(crate) fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            let [a, b, c, _] = v4.octets();
            !(v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                || v4.is_unspecified()
                || v4.is_multicast()
                || a == 0
                || a >= 240
                || (a == 100 && (64..128).contains(&b))
                || (a == 192 && b == 0 && c == 0)
                || (a == 198 && (18..20).contains(&b)))
        }
        IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_public_ip(IpAddr::V4(v4));
            }
            let first = v6.segments()[0];
            !(v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_multicast()
                || (first & 0xfe00) == 0xfc00
                || (first & 0xffc0) == 0xfe80)
        }
    }
}

/// Looks names up as usual, but only answers with addresses on the internet.
struct PublicOnly;

impl reqwest::dns::Resolve for PublicOnly {
    fn resolve(&self, name: reqwest::dns::Name) -> reqwest::dns::Resolving {
        let host = name.as_str().to_owned();
        Box::pin(async move {
            let addrs: Vec<SocketAddr> = tokio::net::lookup_host((host.as_str(), 0)).await?.collect();
            let public: Vec<SocketAddr> = addrs.into_iter().filter(|a| is_public_ip(a.ip())).collect();
            if public.is_empty() {
                return Err(format!("{host} isn't an address on the internet").into());
            }
            Ok(Box::new(public.into_iter()) as reqwest::dns::Addrs)
        })
    }
}

/// A host written as an address that isn't on the internet.
fn private_literal(url: &reqwest::Url) -> bool {
    match url.host_str() {
        Some(host) => {
            host.trim_start_matches('[').trim_end_matches(']').parse::<IpAddr>().is_ok_and(|ip| !is_public_ip(ip))
        }
        None => true,
    }
}

/// The client for pages named by people or packs (store pages, store searches): it only goes to the internet, never
/// to this machine or the local network (a NAS's admin page, a cloud's metadata address), however the address is
/// written or wherever a redirect points.
pub(crate) fn web() -> &'static reqwest::Client {
    static WEB: LazyLock<reqwest::Client> = LazyLock::new(|| {
        reqwest::Client::builder()
            .dns_resolver(PublicOnly)
            .redirect(reqwest::redirect::Policy::custom(|attempt| {
                if attempt.previous().len() >= 5 {
                    attempt.stop()
                } else if private_literal(attempt.url()) {
                    attempt.error("that address isn't on the internet")
                } else {
                    attempt.follow()
                }
            }))
            .timeout(Duration::from_secs(180))
            .build()
            .expect("an HTTP client")
    });
    &WEB
}

/// A web page's text (the first 2 MB), fetched as Hoard.
pub(crate) async fn fetch_page(http: &reqwest::Client, url: &str, accept: &str) -> Result<String> {
    let parsed = reqwest::Url::parse(url).context("not a web address")?;
    anyhow::ensure!(matches!(parsed.scheme(), "http" | "https"), "not a web address");
    anyhow::ensure!(!private_literal(&parsed), "that address isn't on the internet");
    let resp = http
        .get(url)
        .header(reqwest::header::ACCEPT, accept)
        .header(reqwest::header::USER_AGENT, hoard_core::user_agent())
        .timeout(FETCH_TIMEOUT)
        .send()
        .await?
        .error_for_status()?;
    let mut body = Vec::new();
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        body.extend_from_slice(&chunk.context("read the page")?);
        if body.len() >= MAX_PAGE_BYTES {
            break;
        }
    }
    Ok(String::from_utf8_lossy(&body).into_owned())
}

/// The version of [`page_summary`]. Store text read by an older one is read again (see `store_lookup`); 0 is text from
/// before versions were kept, which is read again only if it's still what that reader made of the page.
pub(crate) const STORE_READER: i64 = 1;

/// Title and description of a store page as plain text, the description keeping its paragraphs and lists: the
/// listing's own description where the page carries it (Fab and the Unity Asset Store keep it in the page's data,
/// itch.io in a block of its own), else the page's description tag, which stores flatten into one run of text.
pub(crate) fn page_summary(html: &str) -> Option<String> {
    let title = page_title(html);
    let metas = meta_descriptions(html);
    let meta = metas.first().map_or("", String::as_str);
    let desc = full_description(html, title.as_deref().unwrap_or(""), meta).or_else(|| metas.first().cloned());
    summary(title, desc)
}

/// What [`page_summary`] made of a page before [`STORE_READER`] 1: the title and the longest description tag, all
/// on one line each.
pub(crate) fn first_summary(html: &str) -> Option<String> {
    summary(page_title(html), meta_descriptions(html).first().map(|d| one_line(d)))
}

fn summary(title: Option<String>, desc: Option<String>) -> Option<String> {
    let mut out = String::new();
    if let Some(t) = title {
        out.push_str(&t);
        out.push('\n');
    }
    if let Some(d) = desc {
        out.push_str(&d);
    }
    let out: String = out.trim().chars().take(MAX_STORE_TEXT).collect();
    (!out.is_empty()).then_some(out)
}

fn page_title(html: &str) -> Option<String> {
    let title = between(html, "<title", "</title>").map(|t| t.split_once('>').map_or(t, |(_, t)| t))?;
    Some(decode_entities(title)).filter(|t| !t.is_empty())
}

/// The page's description tags, longest first: og:description is often the full blurb, `description` a trimmed copy.
fn meta_descriptions(html: &str) -> Vec<String> {
    let mut descs: Vec<String> = Vec::new();
    for tag in html.split("<meta").skip(1).map(|t| t.split('>').next().unwrap_or("")) {
        let key = attr(tag, "name").or_else(|| attr(tag, "property")).unwrap_or_default().to_ascii_lowercase();
        if matches!(key.as_str(), "description" | "og:description" | "twitter:description")
            && let Some(c) = attr(tag, "content").map(|c| tidy_lines(&unescape(&c)))
            && !c.is_empty()
            && !descs.iter().any(|d| d == &c)
        {
            descs.push(c);
        }
    }
    descs.sort_by_key(|d| std::cmp::Reverse(d.len()));
    descs
}

/// The listing's whole description, if the page has it, the longest found: a
/// `"description"` in the page's data that has the tag's words or sits next to the listing's name (`title` starts
/// with it), or an itch.io description block.
fn full_description(html: &str, title: &str, meta: &str) -> Option<String> {
    let key = gist(meta, 40);
    let mut found: Vec<String> = Vec::new();
    for (i, k) in html.match_indices("\"description\"") {
        let Some(value) = html[i + k.len()..].trim_start().strip_prefix(':').and_then(|r| json_string(r.trim_start()))
        else {
            continue;
        };
        let text = if value.contains('<') { html_text(&value) } else { tidy_lines(&unescape(&value)) };
        if (key.len() >= 20 && gist(&text, 4000).contains(&key)) || names_listing(&html[..i], title) {
            found.push(text);
        }
    }
    found.extend(blocks_with_class(html, "formatted_description").into_iter().map(html_text));
    found.into_iter().filter(|t| !t.is_empty()).max_by_key(String::len)
}

/// Whether the closest `"name"` or `"title"` before some data is the listing's: the page's title starts with it.
fn names_listing(before: &str, title: &str) -> bool {
    let mut from = before.len().saturating_sub(600);
    while !before.is_char_boundary(from) {
        from += 1;
    }
    let near = &before[from..];
    ["\"name\"", "\"title\""]
        .iter()
        .filter_map(|k| near.rfind(k).map(|j| &near[j + k.len()..]))
        .filter_map(|r| json_string(r.trim_start().strip_prefix(':')?.trim_start()))
        .any(|name| {
            let name = name.trim();
            name.chars().count() >= 3 && title.strip_prefix(name).is_some_and(|r| !r.starts_with(char::is_alphanumeric))
        })
}

/// The JSON string `s` starts with.
fn json_string(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    if bytes.first() != Some(&b'"') {
        return None;
    }
    let mut i = 1;
    while i < bytes.len().min(1 << 20) {
        match bytes[i] {
            b'\\' => i += 2,
            b'"' => return serde_json::from_str(s.get(..=i)?).ok(),
            _ => i += 1,
        }
    }
    None
}

/// What's inside each element whose class list has `class`.
fn blocks_with_class<'a>(html: &'a str, class: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    for (i, _) in html.match_indices(class) {
        let Some(open) = html[..i].rfind('<') else { continue };
        let Some(end) = html[i..].find('>').map(|e| i + e) else { continue };
        let tag = &html[open..end];
        if tag[1..].contains('<') || !attr(tag, "class").is_some_and(|c| c.split_whitespace().any(|c| c == class)) {
            continue;
        }
        let name: String = tag[1..].chars().take_while(char::is_ascii_alphanumeric).collect();
        if let Some(inner) = element_inner(&html[end + 1..], &name.to_ascii_lowercase()) {
            out.push(inner);
        }
    }
    out
}

/// The HTML up to the tag that closes an element named `name`, whose opening tag came just before it.
fn element_inner<'a>(html: &'a str, name: &str) -> Option<&'a str> {
    let lower = html.to_ascii_lowercase();
    let (open, close) = (format!("<{name}"), format!("</{name}"));
    let (mut depth, mut at) = (1, 0);
    loop {
        let next_close = at + lower[at..].find(&close)?;
        if let Some(after) = lower[at..next_close].find(&open).map(|o| at + o + open.len()) {
            if lower[after..].starts_with(|c: char| c.is_ascii_whitespace() || c == '>') {
                depth += 1;
            }
            at = after;
        } else {
            depth -= 1;
            if depth == 0 {
                return html.get(..next_close);
            }
            at = next_close + close.len();
        }
    }
}

/// HTML as plain text that keeps its paragraphs, line breaks and list items.
pub(crate) fn html_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(i) = rest.find('<') {
        out.extend(rest[..i].chars().map(|c| if c.is_whitespace() { ' ' } else { c }));
        rest = &rest[i + 1..];
        let Some(end) = rest.find('>') else {
            rest = "";
            break;
        };
        let tag = &rest[..end];
        rest = &rest[end + 1..];
        let closing = tag.starts_with('/');
        let name = tag.trim_start_matches('/').chars().take_while(char::is_ascii_alphanumeric).collect::<String>();
        match name.to_ascii_lowercase().as_str() {
            n @ ("script" | "style") if !closing => {
                let skip = rest.to_ascii_lowercase().find(&format!("</{n}")).unwrap_or(rest.len());
                rest = &rest[skip..];
            }
            "br" | "div" | "tr" | "dt" | "dd" => out.push('\n'),
            "li" => out.push_str(if closing { "\n" } else { "\n\u{2022} " }),
            "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "ul" | "ol" | "table" | "blockquote" | "pre" | "hr"
            | "section" | "article" => out.push_str("\n\n"),
            _ => {}
        }
    }
    out.extend(rest.chars().map(|c| if c.is_whitespace() { ' ' } else { c }));
    tidy_lines(&unescape(&out))
}

/// Text with the spaces in each line closed up, blank lines between paragraphs kept (one at most), and a bullet left
/// alone on its line joined to the line after it. The items of a list stay on lines one after another.
fn tidy_lines(s: &str) -> String {
    let mut out = String::new();
    let (mut blank, mut bullet, mut in_list) = (false, false, false);
    for line in s.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words.as_slice() {
            [] => blank = !out.is_empty(),
            ["\u{2022}"] => bullet = true,
            _ => {
                let item = bullet || words[0] == "\u{2022}";
                if !out.is_empty() {
                    out.push_str(if blank && !(item && in_list) { "\n\n" } else { "\n" });
                }
                if bullet && words[0] != "\u{2022}" {
                    out.push_str("\u{2022} ");
                }
                out.push_str(&words.join(" "));
                (blank, bullet, in_list) = (false, false, item);
            }
        }
    }
    out
}

fn one_line(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The letters and digits of some text, lowercased: what's left to compare when spacing, punctuation and accents
/// differ.
pub(crate) fn gist(s: &str, max: usize) -> String {
    s.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).take(max).collect()
}

fn between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let lower = s.to_ascii_lowercase();
    let a = lower.find(open)? + open.len();
    let b = a + lower[a..].find(close)?;
    s.get(a..b)
}

pub(crate) fn attr(tag: &str, name: &str) -> Option<String> {
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

/// Text with its character references decoded, on one line.
pub(crate) fn decode_entities(s: &str) -> String {
    one_line(&unescape(s))
}

/// Text with its character references (`&amp;`, `&#8211;`) decoded.
fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        // Looked for by byte: twelve bytes on can be inside a character ("&amp;–"), and ';' is never part of one.
        let Some(end) = rest.bytes().take(12).position(|b| b == b';') else {
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
            "mdash" => Some('\u{2014}'),
            "ndash" => Some('\u{2013}'),
            "hellip" => Some('\u{2026}'),
            "lsquo" => Some('\u{2018}'),
            "rsquo" => Some('\u{2019}'),
            "ldquo" => Some('\u{201c}'),
            "rdquo" => Some('\u{201d}'),
            "bull" => Some('\u{2022}'),
            "middot" => Some('\u{b7}'),
            "times" => Some('\u{d7}'),
            "deg" => Some('\u{b0}'),
            "copy" => Some('\u{a9}'),
            "reg" => Some('\u{ae}'),
            "trade" => Some('\u{2122}'),
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
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_ampersand_before_wide_characters_is_kept() {
        assert_eq!(decode_entities("Rock & Stone – Pack"), "Rock & Stone – Pack");
        assert_eq!(decode_entities("A&B–C–D&amp;E"), "A&B–C–D&E");
    }

    #[test]
    fn only_internet_addresses_are_public() {
        for ip in [
            "127.0.0.1",
            "10.0.0.1",
            "192.168.0.150",
            "172.16.3.4",
            "169.254.169.254",
            "100.64.0.1",
            "0.0.0.0",
            "::1",
            "fd00::1",
            "fe80::1",
            "::ffff:127.0.0.1",
            "::ffff:192.168.1.1",
            "198.18.0.1",
            "255.255.255.255",
        ] {
            assert!(!is_public_ip(ip.parse().unwrap()), "{ip}");
        }
        for ip in ["1.1.1.1", "93.184.216.34", "2606:4700::1111", "::ffff:8.8.8.8"] {
            assert!(is_public_ip(ip.parse().unwrap()), "{ip}");
        }
    }

    #[test]
    fn a_store_is_known_by_the_real_host() {
        assert_eq!(store_from_url("https://someone.itch.io/pack"), Some("itch.io"));
        assert_eq!(store_from_url("http://10.0.0.1\\@x.itch.io/"), None);
        assert_eq!(store_from_url("http://evil.example/?x.itch.io"), None);
    }

    #[tokio::test]
    async fn pages_on_this_machine_are_never_fetched() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        for url in
            [format!("http://127.0.0.1:{port}/"), format!("http://localhost:{port}/"), format!("http://[::1]:{port}/")]
        {
            assert!(fetch_page(web(), &url, "text/html").await.is_err(), "{url}");
        }
    }

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
    fn a_listing_keeps_its_paragraphs() {
        // Fab: the description tag runs the paragraphs together; the page's data has them.
        let fab = r#"<title>Old Stand | Fab</title>
            <meta name="description" content="The stand is worn by daily use.Ideal for rustic dwellings.Texel density: 5941">
            <script>{"related": {"name": "Other", "description": "<p>Another listing.</p>"},
            "description": "<p><span>The stand is worn by daily use.</span></p><p><br><span>Ideal for rustic dwellings.</span></p><br><p><strong>Texel density:</strong> 5941</p><ul><li>FBX</li><li><p>GLTF</p></li></ul>"}</script>"#;
        assert_eq!(
            page_summary(fab).unwrap(),
            "Old Stand | Fab\nThe stand is worn by daily use.\n\nIdeal for rustic dwellings.\n\nTexel density: 5941\n\n\u{2022} FBX\n\u{2022} GLTF"
        );
        // Unity: the tag is boilerplate, and the page's data holds other listings too; the one named in the title is it.
        let unity = r#"<title>Dungeon Pack | 3D | Unity Asset Store</title>
            <meta property="og:description" content="Elevate your workflow with the Dungeon Pack asset.">
            {"1":{"name":"Cave Pack","description":"<p>Caves, and nothing but caves, for any game at all.</p>"},
             "2":{"id":"2","name":"Dungeon Pack","description":"<p>Walls &amp; floors.</p><p>Doors.<br>Torches, chests and bones.</p>"}}"#;
        assert_eq!(
            page_summary(unity).unwrap(),
            "Dungeon Pack | 3D | Unity Asset Store\nWalls & floors.\n\nDoors.\nTorches, chests and bones."
        );
        // itch.io: a block of its own, holding other blocks.
        let itch = r#"<title>Pixel Pack by someone</title><meta name="description" content="Pixels.">
            <div class="formatted_description user_formatted"><p>Free pixel art.</p><div><p>Tiles &mdash; 16&times;16.</p></div>
            </div><div class="more">Not this.</div>"#;
        assert_eq!(page_summary(itch).unwrap(), "Pixel Pack by someone\nFree pixel art.\n\nTiles \u{2014} 16\u{d7}16.");
        // A description tag with lines keeps them; the first reader put it on one line.
        let lines = "<title>Pack</title><meta name=\"description\" content=\"One.&#10;Two.&#10;&#10;&#10;Three.\">";
        assert_eq!(page_summary(lines).unwrap(), "Pack\nOne.\nTwo.\n\nThree.");
        assert_eq!(first_summary(lines).unwrap(), "Pack\nOne. Two. Three.");
    }

    #[test]
    fn recognizes_stores() {
        assert_eq!(store_from_url("https://www.fab.com/listings/abc"), Some("Fab"));
        assert_eq!(store_from_url("https://sketchfab.com/3d-models/abc"), Some("Sketchfab"));
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
