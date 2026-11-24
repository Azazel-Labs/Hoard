//! `cargo xtask licenses`: the licenses of everything Hoard ships, for Settings → About.
//!
//! Every crate the server, the CLI and the desktop app are built from (not the ones only used to build or test them),
//! and every package in the portal's bundle (`web/package-lock.json`'s non-dev packages), with their license and the
//! license files they come with, art copied in from elsewhere (`ART`), and Hoard's own license. Identical texts are kept once. The result is
//! `web/public/licenses.json`, which the portal loads when the page asks for it; `--check` says whether it's current
//! (CI runs that after `npm ci`).

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

use crate::root;

/// The crates whose dependencies ship: the server, the CLI and the desktop app.
const SHIPPED: &[&str] = &["hoardd", "hoard", "hoard-desktop"];
/// What they're built for (release.yml), so a dependency only used on another system isn't listed.
const PLATFORMS: &[&str] =
    &["x86_64-pc-windows-msvc", "aarch64-apple-darwin", "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"];

/// Art copied into the portal rather than installed (an icon drawn from a set): its name, version, license, where
/// it's from, and the folder its license file is kept in.
struct Art {
    name: &'static str,
    version: &'static str,
    license: &'static str,
    url: &'static str,
    dir: &'static str,
}

const ART: &[Art] = &[Art {
    name: "with icons",
    version: "0.6.0",
    license: "MIT",
    url: "https://github.com/withevergrow/withicons",
    dir: "web/src/assets/withicons",
}];

/// Write `web/public/licenses.json`, or with `check` say whether it's current.
pub(crate) fn run(check: bool) -> Result<()> {
    let root = root();
    let mut texts = Texts::default();
    let mut rust = crates(&root, &mut texts)?;
    let mut web = npm(&root.join("web"), &mut texts)?;
    rust.sort_by_key(key);
    web.sort_by_key(key);
    let art: Vec<Value> = ART
        .iter()
        .map(|a| {
            let texts = texts.of(&root.join(a.dir));
            if texts.is_empty() {
                bail!("{} has no license file", a.dir);
            }
            Ok(json!({ "name": a.name, "version": a.version, "license": a.license, "url": a.url, "texts": texts }))
        })
        .collect::<Result<_>>()?;
    let own = std::fs::read_to_string(root.join("LICENSE")).context("read LICENSE")?.replace("\r\n", "\n");
    let out = json!({ "hoard": own.trim(), "crates": rust, "npm": web, "art": art, "texts": texts.list });
    let text = serde_json::to_string(&out)? + "\n";
    let path = root.join("web/public/licenses.json");
    if check {
        let now = std::fs::read_to_string(&path).unwrap_or_default().replace("\r\n", "\n");
        if now != text {
            bail!("{} is out of date: run `cargo xtask licenses` and commit it", path.display());
        }
        println!("{} is current ({} crates, {} npm packages).", path.display(), rust.len(), web.len());
    } else {
        std::fs::write(&path, &text).with_context(|| format!("write {}", path.display()))?;
        println!(
            "Wrote {} ({} crates, {} npm packages, {} KiB).",
            path.display(),
            rust.len(),
            web.len(),
            text.len() / 1024
        );
    }
    Ok(())
}

fn key(v: &Value) -> (String, String) {
    (v["name"].as_str().unwrap_or("").to_owned(), v["version"].as_str().unwrap_or("").to_owned())
}

/// License texts, each kept once; packages name them by their place in the list.
#[derive(Default)]
struct Texts {
    list: Vec<String>,
    seen: HashMap<String, usize>,
}

impl Texts {
    /// The license files in `dir` (LICENSE, LICENSE-MIT, COPYING, NOTICE and the like), as places in the list.
    fn of(&mut self, dir: &Path) -> Vec<usize> {
        let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
        let mut files: Vec<PathBuf> =
            entries.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_file() && is_license(p)).collect();
        files.sort();
        files
            .into_iter()
            .filter_map(|p| std::fs::read_to_string(p).ok())
            .map(|t| t.replace("\r\n", "\n").trim().to_owned())
            .filter(|t| !t.is_empty())
            .map(|t| {
                let next = self.list.len();
                *self.seen.entry(t.clone()).or_insert_with(|| {
                    self.list.push(t);
                    next
                })
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

fn is_license(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_ascii_uppercase();
    let stem = name.split('.').next().unwrap_or("");
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    ["LICENSE", "LICENCE", "COPYING", "NOTICE", "UNLICENSE"].iter().any(|k| stem.starts_with(k)) // spelling: allow (file names written that way)
        && !matches!(ext.as_str(), "rs" | "js" | "html")
}

/// The crates the shipped ones depend on (their normal dependencies, all the way down), not counting the workspace's
/// own.
fn crates(root: &Path, texts: &mut Texts) -> Result<Vec<Value>> {
    let out = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--locked", "--all-features"])
        .args(PLATFORMS.iter().flat_map(|p| ["--filter-platform", p]))
        .current_dir(root)
        .output()
        .context("run cargo metadata")?;
    if !out.status.success() {
        bail!("cargo metadata failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    let meta: Value = serde_json::from_slice(&out.stdout)?;
    let packages: HashMap<&str, &Value> =
        meta["packages"].as_array().into_iter().flatten().filter_map(|p| Some((p["id"].as_str()?, p))).collect();
    let members: BTreeSet<&str> =
        meta["workspace_members"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
    let nodes: HashMap<&str, &Value> = meta["resolve"]["nodes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|n| Some((n["id"].as_str()?, n)))
        .collect();

    let mut todo: Vec<&str> = packages
        .values()
        .filter(|p| p["name"].as_str().is_some_and(|n| SHIPPED.contains(&n)))
        .filter_map(|p| p["id"].as_str())
        .collect();
    let mut seen: BTreeSet<&str> = todo.iter().copied().collect();
    while let Some(id) = todo.pop() {
        for dep in nodes.get(id).and_then(|n| n["deps"].as_array()).into_iter().flatten() {
            let normal = dep["dep_kinds"].as_array().into_iter().flatten().any(|k| k["kind"].is_null());
            if let (true, Some(pkg)) = (normal, dep["pkg"].as_str())
                && seen.insert(pkg)
            {
                todo.push(pkg);
            }
        }
    }

    let mut list = Vec::new();
    for id in seen {
        if members.contains(id) {
            continue;
        }
        let Some(p) = packages.get(id) else { continue };
        let dir = p["manifest_path"].as_str().map(Path::new).and_then(Path::parent).unwrap_or(Path::new(""));
        list.push(json!({
            "name": p["name"],
            "version": p["version"],
            "license": p["license"],
            "url": p["repository"].as_str().or(p["homepage"].as_str()),
            "texts": texts.of(dir),
        }));
    }
    Ok(list)
}

/// The packages in the portal's bundle: `package-lock.json`'s non-dev packages, read from `node_modules`.
fn npm(web: &Path, texts: &mut Texts) -> Result<Vec<Value>> {
    let lock: Value = serde_json::from_str(
        &std::fs::read_to_string(web.join("package-lock.json")).context("read web/package-lock.json")?,
    )?;
    let mut list = Vec::new();
    let mut names = BTreeSet::new();
    for (path, p) in lock["packages"].as_object().into_iter().flatten() {
        if path.is_empty() || p["dev"].as_bool() == Some(true) || p["devOptional"].as_bool() == Some(true) {
            continue;
        }
        let dir = web.join(path);
        if !dir.join("package.json").is_file() {
            bail!("{} isn't installed: run `npm ci` in web first", dir.display());
        }
        let own: Value = serde_json::from_str(&std::fs::read_to_string(dir.join("package.json"))?)?;
        let name = path.rsplit_once("node_modules/").map_or(path.as_str(), |(_, n)| n).to_owned();
        let version = p["version"].as_str().unwrap_or("").to_owned();
        if !names.insert((name.clone(), version.clone())) {
            continue;
        }
        let url = match &own["repository"] {
            Value::String(s) => Some(s.clone()),
            Value::Object(o) => o.get("url").and_then(Value::as_str).map(str::to_owned),
            _ => None,
        }
        .map(|u| u.trim_start_matches("git+").trim_end_matches(".git").replace("git://", "https://"))
        .or_else(|| own["homepage"].as_str().map(str::to_owned));
        list.push(json!({
            "name": name,
            "version": version,
            "license": p["license"].as_str().or(own["license"].as_str()),
            "url": url,
            "texts": texts.of(&dir),
        }));
    }
    Ok(list)
}
