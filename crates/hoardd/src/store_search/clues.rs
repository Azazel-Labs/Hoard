//! What a pack's own files say about where it's from: store links in its README, the seller's name, product
//! folders, its engine and how many meshes and textures it has; and the searches made from its name and them.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::rank::{search_terms, words};
use super::{MAX_LINKS, Place, same_page};
use crate::context::store_from_url;
use crate::sidecar::Sidecar;

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

/// What a pack's own files say about where it came from.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct Clues {
    /// The product's exact title, from a file the store ships with it (a Megascans `<id>.json`), and that file.
    pub(super) title: Option<(String, Place)>,
    /// Product names besides the pack's: folders like `Content/MedievalVillage`, "Thank you for purchasing X".
    pub(super) names: Vec<String>,
    /// Seller names, and where each was seen.
    pub(super) sellers: Vec<(String, Place)>,
    /// Store pages the pack links to: in its README, or in a store's file (Sketchfab's `license.txt`).
    pub(super) links: Vec<Link>,
    /// `unreal-engine`, `unity` or `blender`, from the files' types.
    pub(super) engine: Option<&'static str>,
    pub(super) meshes: u64,
    pub(super) textures: u64,
}

/// A store page the pack links to.
#[derive(Debug, PartialEq)]
pub(super) struct Link {
    pub(super) store: &'static str,
    pub(super) url: String,
    pub(super) from: Place,
    /// What the store's file says about the page, when it's one that links to it.
    pub(super) title: Option<String>,
    pub(super) author: Option<String>,
    pub(super) license: Option<String>,
}

impl Link {
    fn readme(store: &'static str, url: String) -> Self {
        Self { store, url, from: Place::Readme, title: None, author: None, license: None }
    }
}

impl Clues {
    pub(super) fn describe(&self) -> Vec<String> {
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
pub(super) fn same_title(a: &str, b: &str) -> bool {
    let w = |s: &str| {
        s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect::<Vec<_>>()
    };
    let a = w(a);
    !a.is_empty() && a == w(b)
}

/// What to search for: the name, the product names from the files, and shorter pieces of the name (a listing
/// called "Medieval Village" doesn't come up for "Medieval Village Props").
pub(super) fn queries(primary: &str, typed: bool, clues: &Clues) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
