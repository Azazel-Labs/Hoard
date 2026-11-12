//! `hoard`: command-line client for a Hoard server.
//!
//! The interesting command is `hoard sync`: it reads `hoard.toml` in a game project, resolves the selections
//! against the server, downloads exactly the files needed (verified by BLAKE3), lays them out for the engine,
//! and records what it did in `hoard.lock`, a bit like a package manager for art and audio.
#![expect(clippy::print_stdout, reason = "printing results is this binary's job")]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use futures_util::{StreamExt, TryStreamExt, stream};
use hoard_core::{Manifest, ManifestEntry, ManifestRequest, Pack, SearchQuery, SearchResult};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Parser)]
#[command(version = hoard_core::VERSION, about = "Search, upload and sync assets from a Hoard server")]
struct Cli {
    /// Server base URL.
    #[arg(long, global = true, env = "HOARD_SERVER")]
    server: Option<String>,
    /// Bearer token for uploads (same as the server's `HOARD_TOKEN`).
    #[arg(long, global = true, env = "HOARD_TOKEN", hide_env_values = true)]
    token: Option<String>,
    /// Extra CA certificate (PEM) to trust, e.g. the server's self-signed `tls/cert.pem`.
    #[arg(long, global = true, env = "HOARD_CA_CERT")]
    ca_cert: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Full-text search with filters.
    Search {
        query: Vec<String>,
        #[arg(long)]
        kind: Option<String>,
        /// Repeatable; all must match. E.g. `--tag sfx --tag material:grass`.
        #[arg(long = "tag")]
        tags: Vec<String>,
        #[arg(long)]
        pack: Option<String>,
        #[arg(long, default_value_t = 30)]
        limit: u32,
        /// Print raw JSON.
        #[arg(long)]
        json: bool,
    },
    /// Upload files, folders or archives as one pack (or one pack per subfolder with `--each`). Resumable: if it
    /// stops, run the same command again and it continues where it left off.
    Upload {
        #[arg(required = true)]
        paths: Vec<PathBuf>,
        /// Pack name (defaults to the folder or archive name). Ignored with `--each`.
        #[arg(long)]
        name: Option<String>,
        /// Where the pack came from (URL, store name…).
        #[arg(long)]
        source: Option<String>,
        /// Treat each given folder as a library: every subfolder and every archive inside it becomes its own pack.
        #[arg(long)]
        each: bool,
        /// Import even if the server already has a pack with exactly these files.
        #[arg(long)]
        force: bool,
    },
    /// List packs and their ingest status.
    Packs,
    /// Write a starter `hoard.toml` in the current directory.
    Init,
    /// Bring the project's assets in line with `hoard.toml`, recording the result in `hoard.lock`.
    Sync {
        #[arg(long, default_value = "hoard.toml")]
        config: PathBuf,
        /// Show what would change without touching files.
        #[arg(long)]
        dry_run: bool,
        /// Re-hash every file on disk instead of trusting the lockfile.
        #[arg(long)]
        verify: bool,
    },
    /// Convert the Unreal packages the server can't (meshes, animations) with Unreal Editor on this machine, and
    /// upload the results. Leave it running next to an editor install, or use `--once`.
    UnrealWorker(unreal_worker::WorkerArgs),
}

// ---------------------------------------------------------------------------------------------
// hoard.toml / hoard.lock

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectConfig {
    server: Option<String>,
    /// Root folder for synced assets, relative to hoard.toml.
    out_dir: PathBuf,
    /// `category` (out/sfx/footsteps/file.wav), `pack` (out/Pack Name/original/path.wav) or `flat`.
    #[serde(default = "default_layout")]
    layout: Layout,
    /// Write `<file>.hoard.json` next to each asset with its tags and provenance, for engine import scripts.
    #[serde(default)]
    sidecars: bool,
    /// Delete files that a previous sync wrote but no selection wants any more.
    #[serde(default = "yes")]
    prune: bool,
    #[serde(default, rename = "select")]
    selects: Vec<Select>,
}

fn default_layout() -> Layout {
    Layout::Category
}
fn yes() -> bool {
    true
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Layout {
    Category,
    Pack,
    Flat,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Select {
    /// Free-text query.
    query: Option<String>,
    kind: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    pack: Option<String>,
    ext: Option<String>,
    /// A collection made in the portal (name or id): everything in it.
    collection: Option<String>,
    /// Explicit asset ids.
    #[serde(default)]
    ids: Vec<String>,
    /// Subfolder (under `out_dir`) for this selection; overrides the layout's folder.
    dest: Option<String>,
    /// Also fetch what each asset needs (a mesh's textures, .bin buffers, .mtl files). Assets with
    /// dependencies land in their own folder, `<folder>/<mesh name>/…`, so relative references keep working.
    #[serde(default = "yes")]
    with_deps: bool,
    /// With `with_deps`, also fetch linked animation clips (`Knight@Run.fbx` for `Knight.fbx`).
    #[serde(default)]
    with_animations: bool,
    /// Safety valve so a loose query can't pull 40 GB.
    #[serde(default = "default_limit")]
    limit: u32,
}

fn default_limit() -> u32 {
    500
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Lockfile {
    version: u32,
    /// Relative path (under `out_dir`) -> entry.
    files: BTreeMap<String, LockEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LockEntry {
    id: String,
    hash: String,
    size: u64,
}

const STARTER: &str = r#"# Hoard project config. `hoard sync` makes `out_dir` match these selections and writes hoard.lock.
# Commit both files; teammates run `hoard sync` to get identical, hash-verified assets.

server = "http://127.0.0.1:7878"
out_dir = "assets/hoard"        # Godot: a folder under res://, Unity: "Assets/Hoard", Bevy: "assets/hoard"
layout = "category"             # category | pack | flat
sidecars = true                 # write <file>.hoard.json with tags/provenance for import scripts
prune = true                    # remove files a previous sync added that nothing selects anymore

[[select]]
query = "footstep"
kind = "audio"
dest = "sfx/footsteps"

[[select]]
tags = ["pbr"]                  # every PBR texture set
kind = "texture"
limit = 200

# [[select]]
# collection = "Forest level"   # everything in a collection you made in the portal
# dest = "levels/forest"

# [[select]]
# ids = ["0192f0c4-..."]        # pin specific assets
# dest = "ui"
"#;

// ---------------------------------------------------------------------------------------------
// HTTP

mod unreal_worker;
mod upload;

/// The `--ca-cert` file's contents, read once at startup.
static EXTRA_CA: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();

#[derive(Debug)]
struct Client {
    base: String,
    token: Option<String>,
    http: reqwest::Client,
}

impl Client {
    fn new(base: &str, token: Option<String>) -> Result<Self> {
        Ok(Self {
            base: base.trim_end_matches('/').to_owned(),
            token,
            http: {
                let mut b = reqwest::Client::builder().connect_timeout(Duration::from_secs(10));
                if let Some(pem) = EXTRA_CA.get() {
                    b = b.add_root_certificate(reqwest::Certificate::from_pem(pem)?);
                }
                b.build()?
            },
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base)
    }

    fn auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.token {
            Some(t) => req.bearer_auth(t),
            None => req,
        }
    }

    async fn json<T: serde::de::DeserializeOwned>(&self, req: reqwest::RequestBuilder) -> Result<T> {
        let resp = self.auth(req).send().await.with_context(|| format!("can't reach {}", self.base))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("server returned {status}: {body}");
        }
        Ok(resp.json().await?)
    }

    async fn search(&self, q: &SearchQuery) -> Result<SearchResult> {
        let mut params: Vec<(&str, String)> = vec![("facets", "false".into())];
        let opt = [("q", &q.q), ("kind", &q.kind), ("pack", &q.pack), ("ext", &q.ext)];
        for (k, v) in opt {
            if let Some(v) = v {
                params.push((k, v.clone()));
            }
        }
        if !q.tags.is_empty() {
            params.push(("tags", q.tags.join(",")));
        }
        if let Some(l) = q.limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(o) = q.offset {
            params.push(("offset", o.to_string()));
        }
        if let Some(s) = &q.sort {
            params.push(("sort", s.clone()));
        }
        self.json(self.http.get(self.url("/api/assets")).query(&params)).await
    }

    async fn manifest(&self, req: &ManifestRequest) -> Result<Manifest> {
        self.json(self.http.post(self.url("/api/manifest")).json(req)).await
    }

    /// Download a blob to `dest`, verifying its BLAKE3 hash before it lands under its final name.
    async fn fetch_verified(&self, entry: &ManifestEntry, dest: &Path) -> Result<()> {
        let resp = self.auth(self.http.get(self.url(&entry.url))).send().await?.error_for_status()?;
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        // Append rather than replace the extension: crate.gltf and crate.bin must not share a temp file.
        let mut tmp_name = dest.file_name().unwrap_or_default().to_os_string();
        tmp_name.push(".hoard-partial");
        let tmp = dest.with_file_name(tmp_name);
        let mut file = tokio::fs::File::create(&tmp).await?;
        let mut hasher = blake3::Hasher::new();
        let mut body = resp.bytes_stream();
        while let Some(chunk) = body.next().await {
            let chunk = chunk?;
            hasher.update(&chunk);
            file.write_all(&chunk).await?;
        }
        file.flush().await?;
        drop(file);
        let got = hasher.finalize().to_hex().to_string();
        if got != entry.hash {
            tokio::fs::remove_file(&tmp).await.ok();
            bail!("hash mismatch for {}: expected {}, got {got}", entry.path, entry.hash);
        }
        tokio::fs::rename(&tmp, dest).await?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------------------------
// Commands

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(path) = &cli.ca_cert {
        let pem = std::fs::read(path).with_context(|| format!("read CA certificate {}", path.display()))?;
        EXTRA_CA.set(pem).ok();
    }
    let server = cli.server.clone().unwrap_or_else(|| "http://127.0.0.1:7878".into());
    match cli.cmd {
        Cmd::Search { query, kind, tags, pack, limit, json } => {
            let client = Client::new(&server, cli.token)?;
            let q = SearchQuery {
                q: (!query.is_empty()).then(|| query.join(" ")),
                kind,
                tags,
                pack,
                limit: Some(limit),
                ..Default::default()
            };
            let res = client.search(&q).await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                for a in &res.items {
                    let tags: Vec<&str> = a
                        .tags
                        .iter()
                        .map(|t| t.tag.as_str())
                        .filter(|t| !t.starts_with("format:") && !t.starts_with("kind:"))
                        .take(6)
                        .collect();
                    println!(
                        "{}  {:<8} {:<22} {:<36} {}",
                        &a.id[a.id.len() - 8..],
                        a.kind.as_str(),
                        a.category.as_deref().unwrap_or("-"),
                        truncate(&a.name, 36),
                        tags.join(" ")
                    );
                }
                println!("{} of {} results", res.items.len(), res.total);
            }
        }
        Cmd::Upload { paths, name, source, each, force } => {
            upload::run(&Client::new(&server, cli.token)?, &paths, name, source, each, force).await?;
        }
        Cmd::Packs => {
            let client = Client::new(&server, cli.token)?;
            let packs: Vec<Pack> = client.json(client.http.get(client.url("/api/packs"))).await?;
            for p in packs {
                println!(
                    "{}  {:<10} {:>6} assets  {:>10}  {:<12} {}",
                    &p.id[p.id.len() - 8..],
                    p.status.as_str(),
                    p.asset_count,
                    human_bytes(p.total_bytes),
                    p.license.as_deref().unwrap_or("?"),
                    p.name
                );
            }
        }
        Cmd::Init => {
            let path = Path::new("hoard.toml");
            if path.exists() {
                bail!("hoard.toml already exists");
            }
            std::fs::write(path, STARTER)?;
            println!("wrote hoard.toml — edit the selections, then run `hoard sync`");
        }
        Cmd::Sync { config, dry_run, verify } => sync(cli.server, cli.token, &config, dry_run, verify).await?,
        Cmd::UnrealWorker(args) => unreal_worker::run(&Client::new(&server, cli.token)?, &args).await?,
    }
    Ok(())
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n { s.to_owned() } else { format!("{}…", s.chars().take(n - 1).collect::<String>()) }
}

fn human_bytes(b: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut v = b as f64;
    let mut i = 0;
    while v >= 1024.0 && i < units.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    format!("{v:.1} {}", units[i])
}

fn sanitize_segment(s: &str) -> String {
    let cleaned: String =
        s.chars().map(|c| if c.is_control() || r#"<>:"/\|?*"#.contains(c) { '_' } else { c }).collect();
    let t = cleaned.trim().trim_matches('.').to_owned();
    if t.is_empty() { "_".to_owned() } else { t }
}

fn file_name(entry: &ManifestEntry) -> String {
    sanitize_segment(entry.path.rsplit('/').next().unwrap_or(&entry.path))
}

/// Where a bundle member goes: the root's folder, then a folder named after the root, then its bundle path.
fn bundle_dest(root: &ManifestEntry, bundle_path: &str, layout: Layout, dest: Option<&str>) -> String {
    let folder = folder_for(root, layout, dest);
    let root_file = file_name(root);
    let root_stem = root_file.rsplit_once('.').map_or(root_file.as_str(), |(s, _)| s);
    let inner = bundle_path
        .split('/')
        .filter(|s| !s.is_empty() && *s != "..")
        .map(sanitize_segment)
        .collect::<Vec<_>>()
        .join("/");
    [folder.as_str(), root_stem, inner.as_str()].iter().filter(|s| !s.is_empty()).copied().collect::<Vec<_>>().join("/")
}

/// Where an entry goes under `out_dir`, as a `/`-separated relative path.
fn dest_for(entry: &ManifestEntry, layout: Layout, dest: Option<&str>) -> String {
    let folder = folder_for(entry, layout, dest);
    if folder.is_empty() { file_name(entry) } else { format!("{folder}/{}", file_name(entry)) }
}

fn folder_for(entry: &ManifestEntry, layout: Layout, dest: Option<&str>) -> String {
    match (dest, layout) {
        (Some(d), _) => {
            d.split('/').filter(|s| !s.is_empty() && *s != "..").map(sanitize_segment).collect::<Vec<_>>().join("/")
        }
        (None, Layout::Category) => {
            entry.category.as_deref().unwrap_or("misc").split('/').map(sanitize_segment).collect::<Vec<_>>().join("/")
        }
        (None, Layout::Pack) => {
            let dir = entry.path.rsplit_once('/').map_or("", |(d, _)| d);
            std::iter::once(entry.pack_name.as_str())
                .chain(dir.split('/').filter(|s| !s.is_empty() && *s != ".."))
                .map(sanitize_segment)
                .collect::<Vec<_>>()
                .join("/")
        }
        (None, Layout::Flat) => String::new(),
    }
}

async fn sync(server: Option<String>, token: Option<String>, config: &Path, dry_run: bool, verify: bool) -> Result<()> {
    let text =
        std::fs::read_to_string(config).with_context(|| format!("read {} (try `hoard init`)", config.display()))?;
    let cfg: ProjectConfig = toml::from_str(&text).with_context(|| format!("parse {}", config.display()))?;
    let project = config.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let out_dir = project.join(&cfg.out_dir);
    let lock_path = project.join("hoard.lock");
    let old_lock: Lockfile = match std::fs::read_to_string(&lock_path) {
        Ok(s) => serde_json::from_str(&s).context("parse hoard.lock")?,
        Err(_) => Lockfile::default(),
    };
    let server = server.or(cfg.server.clone()).unwrap_or_else(|| "http://127.0.0.1:7878".into());
    let client = Client::new(&server, token)?;

    // 1. Resolve selections -> desired files.
    let mut wanted: BTreeMap<String, ManifestEntry> = BTreeMap::new();
    for (i, sel) in cfg.selects.iter().enumerate() {
        let has_query = sel.query.is_some()
            || sel.kind.is_some()
            || !sel.tags.is_empty()
            || sel.pack.is_some()
            || sel.ext.is_some()
            || sel.collection.is_some();
        let req = ManifestRequest {
            include_deps: sel.with_deps,
            include_animations: sel.with_animations,
            ids: sel.ids.clone(),
            query: has_query.then(|| SearchQuery {
                q: sel.query.clone(),
                kind: sel.kind.clone(),
                tags: sel.tags.clone(),
                pack: sel.pack.clone(),
                ext: sel.ext.clone(),
                collection: sel.collection.clone(),
                limit: Some(sel.limit),
                ..Default::default()
            }),
        };
        let manifest = client.manifest(&req).await.with_context(|| format!("resolve [[select]] #{}", i + 1))?;
        println!("select #{}: {} files", i + 1, manifest.entries.len());
        let roots: BTreeMap<String, ManifestEntry> = manifest
            .entries
            .iter()
            .filter(|e| e.bundle.as_deref() == Some(e.id.as_str()))
            .map(|e| (e.id.clone(), e.clone()))
            .collect();
        // An asset that already travels inside a bundle (a clip with its character) isn't placed again alone.
        let in_bundles: BTreeSet<String> = manifest
            .entries
            .iter()
            .filter(|e| e.bundle.as_ref().is_some_and(|b| *b != e.id))
            .map(|e| e.id.clone())
            .collect();
        for entry in manifest.entries {
            if entry.bundle.is_none() && in_bundles.contains(&entry.id) {
                continue;
            }
            let mut rel = match (&entry.bundle, &entry.bundle_path) {
                (Some(root_id), Some(bundle_path)) if roots.contains_key(root_id) => {
                    let root = &roots[root_id];
                    bundle_dest(root, bundle_path, cfg.layout, sel.dest.as_deref())
                }
                _ => dest_for(&entry, cfg.layout, sel.dest.as_deref()),
            };
            // Two different assets wanting the same path: disambiguate with a short hash.
            if wanted.get(&rel).is_some_and(|e| e.hash != entry.hash) {
                let (stem, ext) = rel.rsplit_once('.').map_or((rel.as_str(), ""), |(s, e)| (s, e));
                rel = format!("{stem}.{}{}{ext}", &entry.hash[..8], if ext.is_empty() { "" } else { "." });
            }
            wanted.entry(rel).or_insert(entry);
        }
    }

    // 2. Plan.
    let mut to_fetch: Vec<(String, &ManifestEntry)> = Vec::new();
    let mut unchanged = 0usize;
    for (rel, entry) in &wanted {
        let path = out_dir.join(rel);
        let locked_same = old_lock.files.get(rel).is_some_and(|l| l.hash == entry.hash);
        let on_disk_ok = if !path.is_file() {
            false
        } else if verify || !locked_same {
            hash_file(&path).is_ok_and(|h| h == entry.hash)
        } else {
            std::fs::metadata(&path).is_ok_and(|m| m.len() == entry.size)
        };
        if on_disk_ok {
            unchanged += 1;
        } else {
            to_fetch.push((rel.clone(), entry));
        }
    }
    let wanted_paths: BTreeSet<&String> = wanted.keys().collect();
    let to_remove: Vec<&String> =
        if cfg.prune { old_lock.files.keys().filter(|k| !wanted_paths.contains(k)).collect() } else { Vec::new() };

    let fetch_bytes: u64 = to_fetch.iter().map(|(_, e)| e.size).sum();
    println!(
        "{} to download ({}), {unchanged} unchanged, {} to remove",
        to_fetch.len(),
        human_bytes(fetch_bytes),
        to_remove.len()
    );
    if dry_run {
        for (rel, _) in &to_fetch {
            println!("  + {rel}");
        }
        for rel in &to_remove {
            println!("  - {rel}");
        }
        return Ok(());
    }

    // 3. Apply.
    stream::iter(to_fetch.iter().map(Ok::<_, anyhow::Error>))
        .try_for_each_concurrent(6, |(rel, entry)| {
            let client = &client;
            let path = out_dir.join(rel);
            async move {
                client.fetch_verified(entry, &path).await?;
                println!("  + {rel}");
                Ok(())
            }
        })
        .await?;
    if cfg.sidecars {
        for (rel, entry) in &wanted {
            let sidecar = out_dir.join(format!("{rel}.hoard.json"));
            let json = serde_json::to_string_pretty(&serde_json::json!({
                "id": entry.id, "hash": entry.hash, "name": entry.name, "kind": entry.kind, "category": entry.category,
                "tags": entry.tags, "pack": entry.pack_name, "source_path": entry.path, "meta": entry.meta,
            }))?;
            if std::fs::read_to_string(&sidecar).ok().as_deref() != Some(json.as_str()) {
                std::fs::write(&sidecar, json)?;
            }
        }
    }
    for rel in &to_remove {
        let path = out_dir.join(rel);
        std::fs::remove_file(&path).ok();
        std::fs::remove_file(out_dir.join(format!("{rel}.hoard.json"))).ok();
        println!("  - {rel}");
    }

    let lock = Lockfile {
        version: 1,
        files: wanted
            .iter()
            .map(|(rel, e)| (rel.clone(), LockEntry { id: e.id.clone(), hash: e.hash.clone(), size: e.size }))
            .collect(),
    };
    std::fs::write(&lock_path, serde_json::to_string_pretty(&lock)? + "\n")?;
    println!("synced {} files into {}", wanted.len(), out_dir.display());
    Ok(())
}

fn hash_file(path: &Path) -> Result<String> {
    let mut hasher = blake3::Hasher::new();
    let mut file = std::io::BufReader::new(std::fs::File::open(path)?);
    std::io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(path: &str, category: Option<&str>) -> ManifestEntry {
        ManifestEntry {
            id: "id".into(),
            hash: "h".repeat(64),
            size: 1,
            kind: hoard_core::AssetKind::Audio,
            name: "n".into(),
            pack_name: "My Pack".into(),
            path: path.into(),
            category: category.map(str::to_owned),
            tags: vec![],
            meta: serde_json::Value::Null,
            url: String::new(),
            bundle: None,
            bundle_path: None,
        }
    }

    #[test]
    fn layouts() {
        let e = entry("Audio/SFX/step_01.wav", Some("sfx/footsteps"));
        assert_eq!(dest_for(&e, Layout::Category, None), "sfx/footsteps/step_01.wav");
        assert_eq!(dest_for(&e, Layout::Pack, None), "My Pack/Audio/SFX/step_01.wav");
        assert_eq!(dest_for(&e, Layout::Flat, None), "step_01.wav");
        assert_eq!(dest_for(&e, Layout::Flat, Some("../../evil")), "evil/step_01.wav");
        let mesh = entry("Models/crate.gltf", Some("models/props"));
        assert_eq!(
            bundle_dest(&mesh, "textures/wood.png", Layout::Category, None),
            "models/props/crate/textures/wood.png"
        );
    }

    #[test]
    fn starter_config_parses() {
        let cfg: ProjectConfig = toml::from_str(STARTER).unwrap();
        assert_eq!(cfg.selects.len(), 2);
    }
}
