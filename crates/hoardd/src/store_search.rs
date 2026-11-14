//! Looking a pack up in the stores it may have come from, so its store details can be filled in with a click.
//!
//! Fab, `itch.io`, Kenney and `OpenGameArt` are searched from the server. Humble and the Unity Asset Store can't be (one
//! refuses non-browser clients, the other searches in the page's script), so for those the UI only gets search links.
//!
//! A pack's name is often too plain to pick out its listing ("Medieval Village" has dozens), so the pack's own files
//! are read for clues first: store links in its README, the seller's name ("Created by", "©", `Assets/<Publisher>/`),
//! product folders (`Content/MedievalVillage`), its engine and how many meshes and textures it has. The name and a
//! few variants of it are searched, and the clues decide the order.

use crate::i18n::tr;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::time::Duration;

use anyhow::{Context as _, Result};
use axum::Json;
use axum::extract::{Path, Query, State};
use futures_util::future::join_all;
use rusqlite::OptionalExtension as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::{attr, decode_entities, fetch_page, store_from_url};
use crate::error::{ApiResult, AppError};
use crate::sidecar::{self, Sidecar};
use crate::{AppState, db};

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

/// Folder names that say what's in them, not which product they belong to.
const GENERIC_FOLDERS: &[&str] = &[
    "content",
    "assets",
    "asset",
    "textures",
    "texture",
    "meshes",
    "mesh",
    "materials",
    "material",
    "maps",
    "map",
    "blueprints",
    "audio",
    "sounds",
    "sound",
    "music",
    "sfx",
    "models",
    "model",
    "prefabs",
    "prefab",
    "scenes",
    "scene",
    "demo",
    "demos",
    "source",
    "sources",
    "fbx",
    "obj",
    "gltf",
    "glb",
    "blend",
    "sprites",
    "sprite",
    "animations",
    "animation",
    "anims",
    "anim",
    "characters",
    "props",
    "environment",
    "environments",
    "fx",
    "vfx",
    "particles",
    "shaders",
    "shader",
    "ui",
    "fonts",
    "font",
    "documentation",
    "docs",
    "doc",
    "data",
    "levels",
    "level",
    "plugins",
    "plugin",
    "resources",
    "editor",
    "scripts",
    "script",
    "misc",
    "extras",
    "extra",
    "sample",
    "samples",
    "example",
    "examples",
    "images",
    "image",
    "png",
    "tga",
    "psd",
    "files",
    "export",
    "exports",
    "previews",
    "preview",
    "screenshots",
    "thumbnails",
    "standard",
    "hdrp",
    "urp",
    "builtin",
    "packages",
    "streamingassets",
    "collections",
    "developers",
    "lods",
    "lod",
    "static",
    "skeletal",
    "decals",
    "foliage",
    "landscape",
    "lighting",
    "physics",
    "sounds_and_music",
    "wav",
    "ogg",
    "mp3",
    "license",
    "licenses",
];

/// Words that don't name a product on their own ("Thank you for purchasing this asset").
const NOT_A_NAME: &[&str] =
    &["asset", "pack", "package", "product", "this", "it", "our", "the", "my", "item", "content"];

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

#[derive(Debug, Deserialize)]
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
    fn is_certain(&self) -> bool {
        (self.score - 1.0).abs() < f32::EPSILON && self.why.iter().any(Why::is_certain)
    }
}

impl StoreMatches {
    /// The one listing the pack names itself (its README links to it, or it has the exact title of the pack's
    /// Megascans file). None if no listing, or more than one, does.
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

/// What a pack's own files say about where it came from.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct Clues {
    /// The product's exact title, from a file the store ships with it (a Megascans `<id>.json`), and that file.
    title: Option<(String, Place)>,
    /// Product names besides the pack's: folders like `Content/MedievalVillage`, "Thank you for purchasing X".
    names: Vec<String>,
    /// Seller names, and where each was seen.
    sellers: Vec<(String, Place)>,
    /// Store pages the pack links to: in its README, or in a store's file (Sketchfab's `license.txt`).
    links: Vec<Link>,
    /// `unreal-engine`, `unity` or `blender`, from the files' types.
    engine: Option<&'static str>,
    meshes: u64,
    textures: u64,
}

/// A store page the pack links to.
#[derive(Debug, PartialEq)]
struct Link {
    store: &'static str,
    url: String,
    from: Place,
    /// What the store's file says about the page, when it's one that links to it.
    title: Option<String>,
    author: Option<String>,
    license: Option<String>,
}

impl Link {
    fn readme(store: &'static str, url: String) -> Self {
        Self { store, url, from: Place::Readme, title: None, author: None, license: None }
    }
}

impl Clues {
    fn describe(&self) -> Vec<String> {
        let mut out: Vec<String> =
            self.title.iter().map(|(t, from)| format!("Title: {t} ({})", from.english())).collect();
        out.extend(self.names.iter().map(|n| format!("Product: {n}")));
        out.extend(self.sellers.iter().map(|(s, from)| format!("Seller: {s} ({})", from.english())));
        out.extend(self.links.iter().map(|l| format!("The {} links to {}", l.from.english(), l.store)));
        if let Some(e) = self.engine {
            out.push(format!("Files for {}", engine_name(e)));
        }
        out
    }
}

fn engine_name(code: &str) -> &str {
    match code {
        "unreal-engine" => "Unreal Engine",
        "unity" => "Unity",
        "blender" => "Blender",
        other => other,
    }
}

fn is_generic(folder: &str) -> bool {
    let l = folder.to_ascii_lowercase();
    l.chars().filter(|c| c.is_alphanumeric()).count() < 3
        || l.starts_with("__")
        || l.starts_with('.')
        || GENERIC_FOLDERS.contains(&l.as_str())
        || l.chars().all(|c| c.is_ascii_digit() || !c.is_alphanumeric())
}

/// Same words, ignoring case and punctuation.
fn same_name(a: &str, b: &str) -> bool {
    words(&search_terms(a)) == words(&search_terms(b))
}

/// A name cut out of a line of README text: up to the end of its sentence, without a leading year or "our".
fn clean_name(raw: &str) -> Option<String> {
    let mut s = raw.trim_start_matches(|c: char| c.is_whitespace() || matches!(c, ':' | '-' | '"' | '\'' | '“'));
    for stop in [" all rights", " for ", " from ", " - ", " – ", " | ", ". ", "! ", ", ", " (", " <", " http"] {
        if let Some(i) = s.to_ascii_lowercase().find(stop) {
            s = &s[..i];
        }
    }
    let s =
        s.trim_end_matches(|c: char| c.is_whitespace() || matches!(c, '.' | '!' | ',' | ';' | ':' | '"' | '\'' | '”'));
    let kept: Vec<&str> = s
        .split_whitespace()
        .skip_while(|w| {
            w.chars().all(|c| c.is_ascii_digit() || c == '-' || c == ',')
                || matches!(w.to_ascii_lowercase().as_str(), "our" | "the" | "this" | "by")
        })
        .collect();
    let name = kept.join(" ");
    let letters = name.chars().filter(|c| c.is_alphabetic()).count();
    (letters >= 2 && name.len() <= 48 && kept.len() <= 6).then_some(name)
}

/// Every store link in some text, each once.
fn store_links(text: &str) -> Vec<(&'static str, String)> {
    let mut out: Vec<(&'static str, String)> = Vec::new();
    for w in text.split(|c: char| c.is_whitespace() || matches!(c, '<' | '>' | '(' | ')' | '"' | '\'' | '[' | ']')) {
        if !(w.starts_with("https://") || w.starts_with("http://")) {
            continue;
        }
        let url = w.trim_end_matches(['.', ',', ';', ':']);
        if let Some(store) = store_from_url(url)
            && !out.iter().any(|(_, u)| u == url)
        {
            out.push((store, url.to_owned()));
        }
    }
    out
}

/// Read a pack's README, file paths and kind counts for hints about which product it is and who made it.
pub(crate) fn clues(pack_name: &str, readme: Option<&str>, paths: &[String], kinds: &BTreeMap<String, u64>) -> Clues {
    let mut c = Clues {
        meshes: kinds.get("model").copied().unwrap_or(0),
        textures: kinds.get("texture").copied().unwrap_or(0),
        ..Clues::default()
    };
    let mut folders: HashMap<&str, usize> = HashMap::new();
    let mut publishers: HashMap<&str, usize> = HashMap::new();
    let (mut unreal, mut unity, mut blender) = (0usize, 0usize, 0usize);
    for p in paths {
        let parts: Vec<&str> = p.split('/').filter(|s| !s.is_empty()).collect();
        let Some((file, dirs)) = parts.split_last() else { continue };
        match file.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase()).as_deref() {
            Some("uasset" | "umap" | "uproject" | "uplugin") => unreal += 1,
            Some("unity" | "prefab" | "unitypackage" | "asmdef" | "controller" | "anim") => unity += 1,
            Some("blend") => blender += 1,
            _ => {}
        }
        let at = |name: &str| dirs.iter().position(|d| d.eq_ignore_ascii_case(name));
        let product = if let Some(i) = at("content") {
            dirs.get(i + 1)
        } else if let Some(i) = at("assets") {
            // Unity: Assets/<Publisher>/<Product>/<what>/…, or Assets/<Product>/<what>/…
            match (dirs.get(i + 1), dirs.get(i + 2)) {
                (Some(a), Some(b)) if !is_generic(a) && !is_generic(b) && dirs.len() > i + 3 => {
                    *publishers.entry(a).or_default() += 1;
                    Some(b)
                }
                (a, _) => a,
            }
        } else {
            dirs.first()
        };
        if let Some(f) = product.filter(|f| !is_generic(f)) {
            *folders.entry(f).or_default() += 1;
        }
    }
    // A product folder holds a good share of the files; the biggest two that aren't just the pack's name.
    let floor = (paths.len() / 5).max(2);
    let mut by_size: Vec<(&str, usize)> = folders.into_iter().filter(|(_, n)| *n >= floor).collect();
    by_size.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (f, _) in by_size {
        let terms = search_terms(f);
        if c.names.len() < 2
            && !terms.is_empty()
            && !same_name(&terms, pack_name)
            && !c.names.iter().any(|n| same_name(n, &terms))
        {
            c.names.push(terms);
        }
    }
    let mut pubs: Vec<(&str, usize)> = publishers.into_iter().filter(|(_, n)| *n >= floor).collect();
    pubs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    if let Some((p, _)) = pubs.first() {
        c.sellers.push((search_terms(p), Place::Folders));
    }

    if let Some(text) = readme {
        c.links = store_links(text).into_iter().take(MAX_LINKS).map(|(store, url)| Link::readme(store, url)).collect();
        for line in text.lines().map(str::trim).filter(|l| !l.is_empty()).take(300) {
            let lower = line.to_ascii_lowercase();
            for key in [
                "thank you for purchasing",
                "thanks for purchasing",
                "thank you for buying",
                "thanks for buying",
                "thank you for downloading",
                "thanks for downloading",
                "thank you for choosing",
            ] {
                if let Some(name) = lower.find(key).and_then(|i| clean_name(&line[i + key.len()..])) {
                    let terms = search_terms(&name);
                    let meaningful = words(&terms).iter().any(|w| !NOT_A_NAME.contains(&w.as_str()));
                    if meaningful
                        && c.names.len() < 3
                        && !same_name(&terms, pack_name)
                        && !c.names.iter().any(|n| same_name(n, &terms))
                    {
                        c.names.push(terms);
                    }
                }
            }
            for key in [
                "created by",
                "made by",
                "developed by",
                "published by",
                "author:",
                "publisher:",
                "copyright",
                "©",
                "(c)",
            ] {
                if let Some(seller) = lower.find(key).and_then(|i| clean_name(&line[i + key.len()..])) {
                    let seller = seller
                        .trim_start_matches(|c: char| {
                            matches!(c, '©' | '(' | ')') || c.is_whitespace() || c.is_ascii_digit()
                        })
                        .trim()
                        .to_owned();
                    let seller = seller
                        .strip_prefix("(c)")
                        .or_else(|| seller.strip_prefix("(C)"))
                        .map_or(seller.as_str(), str::trim)
                        .to_owned();
                    if seller.chars().filter(|c| c.is_alphabetic()).count() >= 2
                        && c.sellers.len() < 3
                        && !c.sellers.iter().any(|(s, _)| s.eq_ignore_ascii_case(&seller))
                    {
                        c.sellers.push((seller, Place::Readme));
                    }
                    break;
                }
            }
        }
    }
    c.engine = if unreal > 0 {
        Some("unreal-engine")
    } else if unity > 0 {
        Some("unity")
    } else if blender > 0 {
        Some("blender")
    } else {
        None
    };
    c
}

impl Clues {
    /// Take what the store's own files in the pack say (see [`crate::sidecar`]). Megascans assets carry `<id>.json`
    /// with the listing's exact title; the pack's own name is usually the file name, with the id and "Raw" in it,
    /// which finds nothing. A pack of several assets has no one title, but its seller is still a clue. A file that
    /// gives the store page (Sketchfab's `license.txt`) links to it as surely as a README does.
    pub(crate) fn add_sidecars(&mut self, files: &[Sidecar]) {
        let titles: BTreeSet<&str> = files.iter().map(|f| f.title.as_str()).collect();
        if let (Some(first), 1) = (files.first(), titles.len())
            && self.title.is_none()
        {
            self.title = Some((first.title.clone(), Place::File(first.source.clone())));
        }
        for f in files {
            if let Some(seller) = &f.seller
                && !self.sellers.iter().any(|(s, _)| s == seller)
            {
                self.sellers.insert(0, (seller.clone(), Place::File(f.source.clone())));
            }
            if let Some(url) = &f.url
                && let Some(store) = store_from_url(url)
                && self.links.len() < MAX_LINKS
                && !self.links.iter().any(|l| same_page(&l.url, url))
            {
                self.links.push(Link {
                    store,
                    url: url.clone(),
                    from: Place::File(f.source.clone()),
                    title: Some(f.title.clone()),
                    author: f.seller.clone(),
                    license: f.license.clone(),
                });
            }
        }
    }
}

/// Words and numbers, ignoring case and punctuation: "Stand Wood Old 05" and `stand_wood_old_05` are the same title.
fn same_title(a: &str, b: &str) -> bool {
    let w = |s: &str| {
        s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect::<Vec<_>>()
    };
    let a = w(a);
    !a.is_empty() && a == w(b)
}

/// What to search for: the name, the product names from the files, and shorter pieces of the name (a listing
/// called "Medieval Village" doesn't come up for "Medieval Village Props").
fn queries(primary: &str, typed: bool, clues: &Clues) -> Vec<String> {
    let mut out: Vec<String> = vec![primary.to_owned()];
    let mut add = |q: String| {
        if !q.trim().is_empty() && !out.iter().any(|o| o.eq_ignore_ascii_case(&q)) {
            out.push(q);
        }
    };
    if !typed {
        clues.names.iter().cloned().for_each(&mut add);
    }
    let w: Vec<&str> = primary.split_whitespace().collect();
    if w.len() >= 3 {
        add(w[..w.len() - 1].join(" "));
        add(w[1..].join(" "));
    }
    if !typed && let Some((seller, _)) = clues.sellers.first() {
        add(format!("{seller} {primary}"));
    }
    out
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

/// Letters and digits only, lower case, for comparing names written differently (`Synty Studios`, `SyntyStudios`).
fn squash(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

/// The number a listing's technical details give for something ("Number of Meshes: 496", "173 textures").
fn spec_count(specs: &str, stem: &str) -> Option<u64> {
    let lower = specs.to_lowercase();
    let number = |s: &str| -> Option<u64> {
        let digits: String = s
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit() || *c == ',')
            .filter(char::is_ascii_digit)
            .collect();
        digits.parse().ok()
    };
    lower.match_indices(stem).find_map(|(i, _)| {
        // "Meshes: 496": a number just after; "496 static meshes": a number just before.
        let after: String = lower[i..].chars().skip_while(|c| c.is_alphabetic()).take(14).collect();
        let after_ok = after.trim_start().starts_with(|c: char| c == ':' || c == '-' || c == '(' || c.is_ascii_digit());
        if after_ok && let Some(n) = number(&after) {
            return Some(n);
        }
        let before: String = lower[..i].chars().rev().take(18).collect::<Vec<_>>().into_iter().rev().collect();
        let tail = before.split(['.', ';', ':', '\n']).next_back().unwrap_or("");
        let n: String = tail.split_whitespace().rfind(|w| w.chars().any(|c| c.is_ascii_digit()))?.to_owned();
        number(&n)
    })
}

/// Close enough to be the same pack: within 15%.
#[expect(clippy::cast_precision_loss, reason = "asset counts are far below 2^52")]
fn about(a: u64, b: u64) -> bool {
    a > 0 && b > 0 && (a.min(b) as f64) / (a.max(b) as f64) >= 0.85
}

/// Score each match against the name and the clues and keep the likely ones, best first.
fn rank(query: &str, clues: &Clues, matches: Vec<StoreMatch>) -> Vec<StoreMatch> {
    let primary = words(query);
    let others: Vec<Vec<String>> = clues.names.iter().map(|n| words(n)).collect();
    let mut kept: Vec<StoreMatch> = Vec::new();
    for mut m in matches {
        if let Some(k) = kept.iter_mut().find(|k| same_page(&k.url, &m.url)) {
            // The same listing from two searches: keep one, with every reason.
            for w in m.why.drain(..) {
                if !k.why.contains(&w) {
                    k.why.push(w);
                }
            }
            continue;
        }
        let by_name = others.iter().map(|o| 0.9 * score(o, &m.title)).fold(score(&primary, &m.title), f32::max);
        let mut s = by_name;
        if m.why.iter().any(|w| matches!(w, Why::Linked(_))) {
            s = 1.0;
        }
        if let Some((title, from)) = &clues.title
            && same_title(title, &m.title)
        {
            s = 1.0;
            m.why.push(Why::SameTitle(from.clone()));
        }
        if let Some(author) = m.author.as_deref().map(squash).filter(|a| a.len() >= 3)
            && let Some((seller, from)) = clues.sellers.iter().find(|(s, _)| {
                let s = squash(s);
                s.len() >= 3 && (author.contains(&s) || s.contains(&author))
            })
        {
            s += 0.35;
            let author = m.author.clone().unwrap_or_else(|| seller.clone());
            m.why.push(Why::By { author, place: from.clone() });
        }
        if let Some(specs) = m.specs.as_deref() {
            if clues.meshes >= 5 && spec_count(specs, "mesh").is_some_and(|n| about(n, clues.meshes)) {
                s += 0.2;
                m.why.push(Why::Meshes(clues.meshes));
            }
            if clues.textures >= 5 && spec_count(specs, "texture").is_some_and(|n| about(n, clues.textures)) {
                s += 0.15;
                m.why.push(Why::Textures(clues.textures));
            }
        }
        // A listing for another engine is less likely, though the same art is often sold for several.
        if let Some(engine) = clues.engine
            && !m.formats.is_empty()
            && !m.formats.iter().any(|f| f == engine)
        {
            s -= 0.15;
        }
        if by_name <= 0.0 && m.why.is_empty() {
            continue;
        }
        // Knowing the exact title, a listing with another one (the same scan's "Old 04") is a near miss at best.
        if let Some((title, _)) = &clues.title
            && !same_title(title, &m.title)
            && !m.why.iter().any(|w| matches!(w, Why::Linked(_)))
        {
            s = s.min(0.85);
        }
        m.score = (s.clamp(0.0, 1.0) * 100.0).round() / 100.0;
        kept.push(m);
    }
    kept.sort_by(|a, b| b.score.total_cmp(&a.score).then(b.why.len().cmp(&a.why.len())));
    let mut per: HashMap<&'static str, usize> = HashMap::new();
    kept.retain(|m| {
        let n = per.entry(m.store).or_default();
        *n += 1;
        *n <= PER_STORE || !m.why.is_empty()
    });
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
        license: None,
        score: 0.0,
        why: Vec::new(),
        formats: Vec::new(),
        specs: None,
    })
}

/// A page of Fab's listing search, which answers in JSON, and the cursor for the next page.
fn parse_fab_page(body: &str) -> Result<(Vec<StoreMatch>, Option<String>)> {
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
fn fab_license(licenses: &Value) -> Option<&'static str> {
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
pub(crate) mod tests {
    use super::*;

    /// A Fab result, as a lookup finds it.
    pub(crate) fn fab_match(title: &str, author: &str, licenses: &[&str]) -> StoreMatch {
        let mut m = found("Fab", title, format!("https://www.fab.com/listings/{}", squash(title))).unwrap();
        m.author = Some(author.to_owned());
        let licenses: Vec<Value> = licenses.iter().map(|l| serde_json::json!({ "name": l })).collect();
        m.license = fab_license(&Value::Array(licenses));
        m
    }

    #[test]
    fn cleans_pack_names_into_search_terms() {
        assert_eq!(search_terms("MedievalVillage_v1_3"), "Medieval Village");
        assert_eq!(search_terms("POLYGON - Fantasy Kingdom (Unity 2020)"), "POLYGON Fantasy Kingdom");
        assert_eq!(search_terms("kenney_platformer-pack-redux"), "platformer pack redux");
        assert_eq!(search_terms("UE5 Stylized Trees V2"), "Stylized Trees");
    }

    #[test]
    fn megascans_files_name_the_listing() {
        let paths: Vec<String> = [
            "Historic_Miscellaneous_Stand_Wood_Old_05_xeokah0_Raw.fbx",
            "Historic_Miscellaneous_Stand_Wood_Old_05_xeokah0_Raw_8K_BaseColor.jpg",
            "xeokah0.json",
        ]
        .map(String::from)
        .to_vec();
        let mut c = clues("historic_miscellaneous_stand_wood_old_05_xeokah0_raw", None, &paths, &BTreeMap::new());
        c.add_sidecars(&[sidecar::read("xeokah0.json", sidecar::tests::STAND).unwrap()]);
        assert_eq!(
            c.title,
            Some(("Historic Miscellaneous Stand Wood Old 05".to_owned(), Place::File("Megascans file".to_owned())))
        );
        assert_eq!(c.sellers[0].0, "Quixel Megascans");
        assert!(c.describe()[0].starts_with("Title: Historic Miscellaneous Stand Wood Old 05"));

        let m = |title: &str, author: &str| {
            let mut m = found("Fab", title, format!("https://www.fab.com/listings/{}", squash(title))).unwrap();
            m.author = Some(author.to_owned());
            m
        };
        let ranked = rank(
            "Historic Miscellaneous Stand Wood Old 05",
            &c,
            vec![
                m("Historic Miscellaneous Door Wood Old 01", "Quixel Megascans"),
                m("Old Styled Wooden Info Stand", "Incg5764"),
                m("Historic Miscellaneous Stand Wood Old 04", "Quixel Megascans"),
                m("Historic Miscellaneous Stand Wood Old 05", "Quixel Megascans"),
            ],
        );
        assert_eq!(ranked[0].title, "Historic Miscellaneous Stand Wood Old 05");
        assert!((ranked[0].score - 1.0).abs() < f32::EPSILON);
        assert!(ranked[0].why.iter().any(|w| w.to_string() == "Same title as the pack's Megascans file"));
        assert!(ranked[1].score < 1.0, "a different number isn't the same listing");
        assert!(same_title("stand_wood_old_05", "Stand Wood Old 05") && !same_title("Old 05", "Old 04"));
    }

    #[tokio::test]
    async fn a_sketchfab_file_links_to_its_page() {
        let text = include_str!("../formats/samples/sketchfab/license.txt");
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

    fn matches(matches: Vec<StoreMatch>) -> StoreMatches {
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

    #[test]
    fn only_a_listing_the_pack_names_is_certain() {
        let mut c = Clues::default();
        c.add_sidecars(&[sidecar::read("xeokah0.json", sidecar::tests::STAND).unwrap()]);
        let stand = "Historic Miscellaneous Stand Wood Old 05";
        let found = matches(rank(
            stand,
            &c,
            vec![
                fab_match(stand, "Quixel Megascans", &["Personal", "Professional"]),
                fab_match("Historic Miscellaneous Stand Wood Old 04", "Quixel Megascans", &["Personal"]),
            ],
        ));
        let sure = found.certain().unwrap();
        assert_eq!(
            (sure.url(), sure.license()),
            ("https://www.fab.com/listings/historicmiscellaneousstandwoodold05", Some(FAB_STANDARD))
        );

        // The same words aren't enough: plain names have many listings.
        let plain = matches(rank("medieval village", &Clues::default(), vec![fab_match("Medieval Village", "A", &[])]));
        assert!((plain.matches[0].score - 1.0).abs() < f32::EPSILON && plain.certain().is_none());

        // Two pages the README links to: which one is it?
        let mut two = matches(vec![fab_match("One", "A", &[]), fab_match("Two", "B", &["CC-BY"])]);
        for m in &mut two.matches {
            m.score = 1.0;
            m.why.push(Why::Linked(Place::Readme));
        }
        assert!(two.certain().is_none());
        assert_eq!(two.matches[1].license(), Some(CC_BY_4));
        two.matches.truncate(1);
        assert_eq!(two.certain().map(StoreMatch::url), Some("https://www.fab.com/listings/one"));
    }

    #[test]
    fn ranks_by_shared_words() {
        let m = |store, title: &str| found(store, title, format!("https://example.com/{}", squash(title))).unwrap();
        let ranked = rank(
            "medieval village",
            &Clues::default(),
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

    #[test]
    fn reads_clues_from_the_files() {
        let paths: Vec<String> = [
            "MedievalVillage/Content/MedievalVillage/Meshes/SM_House.uasset",
            "MedievalVillage/Content/MedievalVillage/Meshes/SM_Well.uasset",
            "MedievalVillage/Content/MedievalVillage/Textures/T_Wall_D.uasset",
            "MedievalVillage/Content/MedievalVillage/Maps/Demo.umap",
            "MedievalVillage/Content/Megascans/Rocks/T_Rock.uasset",
        ]
        .map(str::to_owned)
        .into();
        let readme = "Thank you for purchasing Rustic Hamlet Kit!\nCreated by Marco Maria Rossi.\n\
                      Store page: https://www.fab.com/listings/f89fe420-1942 (rate it!)\n© 2024 Marco Maria Rossi";
        let kinds = BTreeMap::from([("model".to_owned(), 496), ("texture".to_owned(), 173)]);
        let c = clues("Medieval Village v1.2", Some(readme), &paths, &kinds);
        // The product folder is just the pack's name again, so it adds nothing; the README's name does.
        assert_eq!(c.names, ["Rustic Hamlet Kit"]);
        assert_eq!(c.sellers, [("Marco Maria Rossi".to_owned(), Place::Readme)]);
        assert_eq!(c.links, [Link::readme("Fab", "https://www.fab.com/listings/f89fe420-1942".to_owned())]);
        assert_eq!((c.engine, c.meshes, c.textures), (Some("unreal-engine"), 496, 173));

        let unity: Vec<String> =
            (0..6).map(|i| format!("Assets/Synty/PolygonFantasyKingdom/Models/m{i}.fbx")).collect();
        let c = clues("pack_2023", None, &unity, &BTreeMap::new());
        assert_eq!(c.names, ["Polygon Fantasy Kingdom"]);
        assert_eq!(c.sellers, [("Synty".to_owned(), Place::Folders)]);
        assert_eq!(c.engine, None);
    }

    #[test]
    fn searches_pieces_of_the_name_too() {
        let c = Clues {
            names: vec!["Rustic Hamlet".into()],
            sellers: vec![("Marco".into(), Place::Readme)],
            ..Clues::default()
        };
        assert_eq!(
            queries("Medieval Village Props", false, &c),
            [
                "Medieval Village Props",
                "Rustic Hamlet",
                "Medieval Village",
                "Village Props",
                "Marco Medieval Village Props"
            ]
        );
        assert_eq!(queries("castle", true, &c), ["castle"]);
    }

    #[test]
    fn reads_counts_from_technical_details() {
        let specs = "Number of Meshes: 496 Static meshes in total\nTexture: 173\nCollision: Yes";
        assert_eq!(spec_count(specs, "mesh"), Some(496));
        assert_eq!(spec_count(specs, "texture"), Some(173));
        assert_eq!(spec_count("Includes 1,204 textures and 80 materials", "texture"), Some(1204));
        assert_eq!(spec_count("Nanite: Yes", "mesh"), None);
        assert!(about(496, 480) && !about(496, 300));
    }

    #[test]
    fn clues_pick_the_listing_among_same_named_ones() {
        let listing = |title: &str, author: &str, url: &str, specs: &str| {
            let mut m = found("Fab", title, format!("https://www.fab.com/listings/{url}")).unwrap();
            m.author = Some(author.to_owned());
            m.formats = vec!["unreal-engine".to_owned()];
            m.specs = Some(specs.to_owned());
            m
        };
        let c = Clues {
            sellers: vec![("Marco Maria Rossi".into(), Place::Readme)],
            engine: Some("unreal-engine"),
            meshes: 490,
            ..Clues::default()
        };
        let ranked = rank(
            "Medieval Village",
            &c,
            vec![
                listing("Medieval Village", "Anatolii Umanets", "a", "Meshes: 40"),
                listing("Medieval Village", "Marco Maria Rossi", "b", "Number of Meshes: 496"),
                listing("Medieval Village", "Anatolii Umanets", "a", ""),
            ],
        );
        assert_eq!(ranked.len(), 2, "the same listing found twice is kept once");
        assert_eq!(ranked[0].url, "https://www.fab.com/listings/b");
        assert_eq!(ranked[0].why.len(), 2);
        assert!(ranked[0].why[0].to_string().contains("Marco Maria Rossi"));
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
