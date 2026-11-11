//! `hoard unreal-worker`: converts the Unreal packages a Hoard server can't convert by itself (meshes, animations,
//! textures imported uncompressed, which Unreal stores Oodle-compressed) by running Unreal Editor headless on this
//! machine.
//!
//! For each pack with waiting packages it rebuilds a throwaway project, putting every package back at the
//! `/Game/...` path its references expect, runs the bundled export script, uploads the FBX/PNG/WAV results as
//! replacements, and asks the server to re-analyze the pack.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use hoard_core::{Asset, PendingPackage, SearchQuery};
use serde::Deserialize;

use crate::Client;

/// The editor-side export script, shipped inside the binary.
const EXPORT_SCRIPT: &str = include_str!("../../../tools/unreal_export.py");

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct WorkerArgs {
    /// Unreal Editor command-line executable (`UnrealEditor-Cmd.exe`, `UnrealEditor-Cmd`).
    #[arg(long, env = "HOARD_UNREAL_EDITOR")]
    editor: PathBuf,
    /// Keep each original package on the server after its export is uploaded (default: discard it).
    #[arg(long)]
    keep_originals: bool,
    /// Work through what's waiting, then exit (default: keep polling).
    #[arg(long)]
    once: bool,
    /// Seconds between polls.
    #[arg(long, default_value_t = 60)]
    poll: u64,
    /// Scratch folder for the temporary projects (default: the system temp folder).
    #[arg(long)]
    work_dir: Option<PathBuf>,
    /// Lay out the project and show what would run, without starting the editor or uploading.
    #[arg(long)]
    dry_run: bool,
    /// Seconds before an editor run is given up on.
    #[arg(long, default_value_t = 3600)]
    timeout: u64,
}

pub(crate) async fn run(client: &Client, args: &WorkerArgs) -> Result<()> {
    if !args.dry_run && !args.editor.is_file() {
        bail!("{} isn't a file; point --editor at UnrealEditor-Cmd", args.editor.display());
    }
    // In polling mode, packages the editor couldn't export stay pending on the server; don't retry them forever.
    let mut tried: HashSet<String> = HashSet::new();
    loop {
        let pending: Vec<PendingPackage> = client.json(client.http.get(client.url("/api/unreal/pending"))).await?;
        let fresh: Vec<PendingPackage> = pending.into_iter().filter(|p| !tried.contains(&p.asset_id)).collect();
        if fresh.is_empty() {
            if args.once {
                println!("Nothing waiting for Unreal Editor.");
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(args.poll)).await;
            continue;
        }
        let mut by_pack: BTreeMap<String, Vec<PendingPackage>> = BTreeMap::new();
        for p in fresh {
            tried.insert(p.asset_id.clone());
            by_pack.entry(p.pack_id.clone()).or_default().push(p);
        }
        for (pack_id, items) in &by_pack {
            if let Err(e) = convert_pack(client, args, pack_id, items).await {
                eprintln!("pack {pack_id}: {e:#}");
            }
        }
        if args.once {
            return Ok(());
        }
    }
}

/// Every asset in a pack, a page at a time.
async fn pack_assets(client: &Client, pack_id: &str) -> Result<Vec<Asset>> {
    let mut out = Vec::new();
    loop {
        let q = SearchQuery {
            pack: Some(pack_id.to_owned()),
            limit: Some(500),
            offset: Some(u32::try_from(out.len())?),
            sort: Some("old".into()), // stable order for paging
            ..Default::default()
        };
        let page = client.search(&q).await?;
        let n = page.items.len();
        out.extend(page.items);
        if n == 0 || out.len() as u64 >= page.total {
            return Ok(out);
        }
    }
}

/// Where a package goes under the project's `Content/`, mount prefix included: `RockPack/Meshes/SM_Rock`.
#[derive(Debug, Default)]
struct Layout {
    /// Prepended to pack paths without a `Content/` folder (uploads often lose `Content/<Mount>/`).
    prefix: String,
}

impl Layout {
    fn infer(packages: &[&str], dependencies: &[String]) -> Self {
        let mut votes: HashMap<String, usize> = HashMap::new();
        for dep in dependencies {
            // "/Game/RockPack/Textures/T_Rock_D" -> "RockPack/Textures/T_Rock_D"
            let Some(rel) = dep.strip_prefix('/').and_then(|d| d.split_once('/')).map(|(_, r)| r) else { continue };
            for p in packages {
                if Self::content_rel(p).is_none()
                    && let Some(prefix) = rel.strip_suffix(*p)
                    && (prefix.is_empty() || prefix.ends_with('/'))
                {
                    *votes.entry(prefix.to_owned()).or_default() += 1;
                }
            }
        }
        let prefix = votes.into_iter().max_by_key(|(p, n)| (*n, p.len())).map(|(p, _)| p).unwrap_or_default();
        Self { prefix }
    }

    /// The part after the last `Content/` folder, if the path has one.
    fn content_rel(path: &str) -> Option<&str> {
        let lower = path.to_ascii_lowercase();
        if lower.starts_with("content/") {
            return Some(&path[8..]);
        }
        lower.rfind("/content/").map(|i| &path[i + 9..])
    }

    /// Package path relative to `Content/`, without extension.
    fn place(&self, pack_path: &str) -> String {
        let no_ext = pack_path.rsplit_once('.').map_or(pack_path, |(s, _)| s);
        match Self::content_rel(no_ext) {
            Some(rel) => rel.to_owned(),
            None => format!("{}{no_ext}", self.prefix),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Report {
    #[serde(default)]
    exported: Vec<ReportEntry>,
    #[serde(default)]
    failed: Vec<ReportEntry>,
}

#[derive(Debug, Deserialize)]
struct ReportEntry {
    package: String,
    #[serde(default)]
    file: Option<String>,
}

fn package_name(placed: &str) -> String {
    format!("/Game/{placed}")
}

async fn convert_pack(client: &Client, args: &WorkerArgs, pack_id: &str, items: &[PendingPackage]) -> Result<()> {
    let assets = pack_assets(client, pack_id).await?;
    let pack_name = assets.first().map_or_else(|| pack_id.to_owned(), |a| a.pack_name.clone());
    println!("{pack_name}: {} package(s) for Unreal Editor", items.len());

    // Package paths as the pack had them: current packages, and the originals of converted assets.
    let original = |a: &Asset| a.meta.get("converted_from").and_then(|v| v.as_str()).map(str::to_owned);
    let mut packages: Vec<(String, &Asset, bool)> = Vec::new(); // (pack path, asset, is the original of a conversion)
    let mut deps: Vec<String> = Vec::new();
    for a in &assets {
        if let Some(d) = a.meta.get("ue_dependencies").and_then(|v| v.as_array()) {
            deps.extend(d.iter().filter_map(|x| x.as_str().map(str::to_owned)));
        }
        if matches!(a.ext.as_str(), "uasset" | "umap") {
            packages.push((a.path.clone(), a, false));
        } else if let Some(p) = original(a) {
            packages.push((p, a, true));
        }
    }
    let paths: Vec<String> =
        packages.iter().map(|(p, ..)| p.rsplit_once('.').map_or(p.as_str(), |(s, _)| s).to_owned()).collect();
    let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
    let layout = Layout::infer(&path_refs, &deps);

    let base = args.work_dir.clone().unwrap_or_else(std::env::temp_dir).join(format!("hoard-unreal-{pack_id}"));
    if base.exists() {
        tokio::fs::remove_dir_all(&base).await?;
    }
    let content = base.join("project/Content");
    let out = base.join("out");
    tokio::fs::create_dir_all(&content).await?;
    tokio::fs::create_dir_all(&out).await?;

    // Lay out every package we can get, so the waiting ones load with their materials, skeletons and textures.
    let wanted: HashSet<&str> = items.iter().map(|p| p.asset_id.as_str()).collect();
    let mut by_package: HashMap<String, String> = HashMap::new(); // package name -> asset id to replace
    let mut fetched = 0usize;
    for (path, asset, is_original) in &packages {
        let placed = layout.place(path);
        let ext = path.rsplit_once('.').map_or("uasset", |(_, e)| e);
        let dest = content.join(format!("{placed}.{ext}"));
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let url = if *is_original {
            format!("/api/assets/{}/original", asset.id)
        } else {
            format!("/api/assets/{}/file", asset.id)
        };
        let resp = client.auth(client.http.get(client.url(&url))).send().await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND && *is_original {
            continue; // the original was discarded; the editor will do without it
        }
        let bytes = resp.error_for_status()?.bytes().await?;
        tokio::fs::write(&dest, &bytes).await?;
        fetched += 1;
        if wanted.contains(asset.id.as_str()) && !is_original {
            by_package.insert(package_name(&placed), asset.id.clone());
        }
    }
    let only: Vec<&String> = by_package.keys().collect();
    let only_file = base.join("only.txt");
    let list: Vec<&str> = only.iter().map(|s| s.as_str()).collect();
    tokio::fs::write(&only_file, list.join("\n") + "\n").await?;
    let script = base.join("hoard_unreal_export.py");
    tokio::fs::write(&script, EXPORT_SCRIPT).await?;
    let project = base.join("project/HoardWorker.uproject");
    tokio::fs::write(
        &project,
        r#"{ "FileVersion": 3, "EngineAssociation": "", "Plugins": [ { "Name": "PythonScriptPlugin", "Enabled": true } ] }"#,
    )
    .await?;
    let mount = if layout.prefix.is_empty() { "(as uploaded)".to_owned() } else { format!("/Game/{}", layout.prefix) };
    println!("  laid out {fetched} package(s) in {} (mount {mount})", content.display());
    if args.dry_run {
        for p in &only {
            println!("  would export {p}");
        }
        return Ok(());
    }

    let mut cmd = tokio::process::Command::new(&args.editor);
    cmd.arg(&project)
        .args(["-run=pythonscript", "-unattended", "-nosplash", "-nop4", "-NullRHI", "-stdout"])
        // Script arguments go through the environment: paths with spaces survive Unreal's command-line parsing.
        .env("HOARD_EXPORT_ROOT", "/Game")
        .env("HOARD_EXPORT_OUT", &out)
        .env("HOARD_EXPORT_ONLY", &only_file)
        .stdin(Stdio::null())
        .kill_on_drop(true);
    script_arg(&mut cmd, &script);
    let started = std::time::Instant::now();
    let output = tokio::time::timeout(Duration::from_secs(args.timeout), cmd.output())
        .await
        .context("Unreal Editor timed out")?
        .with_context(|| format!("couldn't start {}", args.editor.display()))?;
    let report_path = out.join("hoard-export.json");
    if !report_path.is_file() {
        let tail: Vec<String> =
            String::from_utf8_lossy(&output.stdout).lines().rev().take(15).map(str::to_owned).collect();
        bail!(
            "the editor exited with {} and wrote no report; last lines:\n{}",
            output.status,
            tail.into_iter().rev().collect::<Vec<_>>().join("\n")
        );
    }
    let report: Report = serde_json::from_slice(&tokio::fs::read(&report_path).await?)?;
    println!("  editor finished in {:.0}s", started.elapsed().as_secs_f32());

    let mut uploaded = 0usize;
    for e in &report.exported {
        let (Some(id), Some(file)) = (by_package.get(&e.package), &e.file) else { continue };
        let local = out.join(file);
        let ext = file.rsplit_once('.').map_or("bin", |(_, x)| x).to_ascii_lowercase();
        let bytes = tokio::fs::read(&local).await.with_context(|| format!("read {}", local.display()))?;
        let req = client
            .http
            .post(client.url(&format!("/api/assets/{id}/replace")))
            .query(&[
                ("ext", ext.as_str()),
                ("method", "editor"),
                ("keep_original", if args.keep_originals { "1" } else { "0" }),
                ("notes", "exported by Unreal Editor"),
            ])
            .body(bytes);
        match client.auth(req).send().await {
            Ok(r) if r.status().is_success() || r.status() == reqwest::StatusCode::CONFLICT => {
                uploaded += 1;
                println!("  {} -> .{ext}", e.package);
            }
            Ok(r) => eprintln!("  {}: server returned {}", e.package, r.status()),
            Err(err) => eprintln!("  {}: {err}", e.package),
        }
    }
    for e in &report.failed {
        eprintln!("  {}: the editor couldn't export it", e.package);
    }
    if uploaded > 0 {
        let r =
            client.auth(client.http.post(client.url(&format!("/api/packs/{pack_id}/unreal/finish")))).send().await?;
        r.error_for_status()?;
    }
    println!("  uploaded {uploaded}, failed {}", report.failed.len());
    tokio::fs::remove_dir_all(&base).await.ok();
    Ok(())
}

/// `-script="<path>"`, quoted the way Unreal's parser expects on each platform.
#[cfg(windows)]
fn script_arg(cmd: &mut tokio::process::Command, script: &Path) {
    // Windows hands Unreal the raw command line, so the quotes must be literal.
    cmd.raw_arg(format!("-script=\"{}\"", script.display()));
}

/// `-script="<path>"`, quoted the way Unreal's parser expects on each platform.
#[cfg(not(windows))]
fn script_arg(cmd: &mut tokio::process::Command, script: &Path) {
    // Unix launchers rebuild the command line from argv and quote values with spaces themselves.
    cmd.arg(format!("-script={}", script.display()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_recovers_the_mount() {
        let packages = ["Textures/T_Rock_D", "Materials/MI_Rock", "Meshes/SM_Rock"];
        let deps = vec!["/Game/RockPack/Textures/T_Rock_D".to_owned(), "/Game/RockPack/Materials/MI_Rock".to_owned()];
        let layout = Layout::infer(&packages, &deps);
        assert_eq!(layout.prefix, "RockPack/");
        assert_eq!(layout.place("Meshes/SM_Rock.uasset"), "RockPack/Meshes/SM_Rock");
        // Paths that kept their Content folder are used as they are.
        assert_eq!(layout.place("Kit/Content/Kit/Meshes/SM_A.uasset"), "Kit/Meshes/SM_A");
        assert_eq!(Layout::infer(&packages, &[]).place("Meshes/SM_Rock.uasset"), "Meshes/SM_Rock");
    }
}
