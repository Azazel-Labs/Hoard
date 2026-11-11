//! Small local models for Hoard's LLM tagger, downloaded from Hugging Face as GGUF files for llama.cpp.
//!
//! Every model here takes images (via a separate `mmproj` projector file), runs on a laptop CPU, and fits in a few
//! gigabytes. Files land in `models/<id>/model.gguf` and `models/<id>/mmproj.gguf` (stable names, so compose files
//! and scripts don't care which quantization you picked), with a `model.json` recording where they came from and
//! their SHA-256. Downloads resume, and are verified against the hashes Hugging Face publishes.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// One entry in the catalog.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Model {
    pub(crate) id: &'static str,
    /// Hugging Face repository holding the GGUF files.
    pub(crate) repo: &'static str,
    /// Default quantization of the language model, matched against file names.
    pub(crate) quant: &'static str,
    /// Projector precisions to look for, best first.
    pub(crate) mmproj: &'static [&'static str],
    /// Rough download size at the default quantization, for the listing.
    pub(crate) approx_gb: f32,
    pub(crate) license: &'static str,
    /// Thinks before answering by default; the server is started with reasoning off for tagging.
    pub(crate) thinking: bool,
    pub(crate) note: &'static str,
}

pub(crate) const DEFAULT: &str = "qwen3.5-2b";

pub(crate) const CATALOG: &[Model] = &[
    Model {
        id: "qwen3.5-2b",
        repo: "unsloth/Qwen3.5-2B-GGUF",
        quant: "Q4_K_M",
        mmproj: &["F16", "BF16", "F32"],
        approx_gb: 1.9,
        license: "Apache-2.0",
        thinking: true,
        note: "Default. Natively multimodal, good at following the JSON format.",
    },
    Model {
        id: "qwen3.5-4b",
        repo: "unsloth/Qwen3.5-4B-GGUF",
        quant: "Q4_K_M",
        mmproj: &["F16", "BF16", "F32"],
        approx_gb: 3.4,
        license: "Apache-2.0",
        thinking: true,
        note: "Noticeably better names and descriptions; wants 6 GB of RAM.",
    },
    Model {
        id: "qwen3-vl-2b",
        repo: "Qwen/Qwen3-VL-2B-Instruct-GGUF",
        quant: "Q4_K_M",
        mmproj: &["F16", "Q8_0"],
        approx_gb: 1.9,
        license: "Apache-2.0",
        thinking: false,
        note: "Qwen's own GGUF build of the previous generation; no thinking mode to switch off.",
    },
    Model {
        id: "gemma4-e2b",
        repo: "unsloth/gemma-4-E2B-it-GGUF",
        quant: "Q4_K_M",
        mmproj: &["F16", "BF16", "F32"],
        approx_gb: 3.5,
        license: "Apache-2.0",
        thinking: false,
        note: "Google's edge model; strong captions, larger file for its speed.",
    },
    Model {
        id: "lfm2.5-vl-1.6b",
        repo: "LiquidAI/LFM2.5-VL-1.6B-GGUF",
        quant: "Q4_0",
        mmproj: &["Q8_0", "F16"],
        approx_gb: 1.3,
        license: "LFM Open License 1.0",
        thinking: false,
        note: "Smallest and fastest; fine for tags, weaker at descriptions. Check the license for commercial use.",
    },
];

pub(crate) fn find(id: &str) -> Result<&'static Model> {
    CATALOG.iter().find(|m| m.id.eq_ignore_ascii_case(id)).with_context(|| {
        let ids: Vec<_> = CATALOG.iter().map(|m| m.id).collect();
        format!("no model called '{id}'; pick one of: {}", ids.join(", "))
    })
}

/// Where downloaded models live: `$HOARD_MODELS_DIR`, or `models/` at the repository root.
pub(crate) fn models_dir() -> PathBuf {
    std::env::var_os("HOARD_MODELS_DIR").map_or_else(|| crate::root().join("models"), PathBuf::from)
}

/// What `model.json` records about a downloaded model.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Installed {
    pub(crate) id: String,
    pub(crate) repo: String,
    pub(crate) files: Vec<InstalledFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct InstalledFile {
    /// Local name: `model.gguf` or `mmproj.gguf`.
    pub(crate) name: String,
    /// Path inside the Hugging Face repository.
    pub(crate) source: String,
    pub(crate) size: u64,
    pub(crate) sha256: Option<String>,
}

impl Installed {
    pub(crate) fn load(id: &str) -> Option<Self> {
        let text = fs::read_to_string(models_dir().join(id).join("model.json")).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub(crate) fn path(&self, name: &str) -> Option<PathBuf> {
        let p = models_dir().join(&self.id).join(name);
        (self.files.iter().any(|f| f.name == name) && p.is_file()).then_some(p)
    }
}

/// One file in a Hugging Face repository listing.
#[derive(Debug, Deserialize)]
struct RepoFile {
    #[serde(rename = "type")]
    kind: String,
    path: String,
    #[serde(default)]
    size: u64,
    #[serde(default)]
    lfs: Option<Lfs>,
}

#[derive(Debug, Deserialize)]
struct Lfs {
    /// SHA-256 of the file contents.
    oid: String,
    size: u64,
}

fn endpoint() -> String {
    // Same variable the huggingface_hub tools honor, so mirrors work.
    std::env::var("HF_ENDPOINT")
        .unwrap_or_else(|_| "https://huggingface.co".to_owned())
        .trim_end_matches('/')
        .to_owned()
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_recv_response(Some(Duration::from_secs(60)))
        .user_agent(concat!("hoard-xtask/", env!("CARGO_PKG_VERSION")))
        .build()
        .into()
}

fn with_token<B>(req: ureq::RequestBuilder<B>) -> ureq::RequestBuilder<B> {
    match std::env::var("HF_TOKEN") {
        Ok(t) if !t.is_empty() => req.header("Authorization", &format!("Bearer {t}")),
        _ => req,
    }
}

fn list_repo(agent: &ureq::Agent, repo: &str) -> Result<Vec<RepoFile>> {
    let url = format!("{}/api/models/{repo}/tree/main?recursive=true", endpoint());
    let text = with_token(agent.get(&url))
        .call()
        .with_context(|| format!("list files of {repo} (set HF_TOKEN if the repository is gated)"))?
        .body_mut()
        .read_to_string()?;
    Ok(serde_json::from_str::<Vec<RepoFile>>(&text)?.into_iter().filter(|f| f.kind == "file").collect())
}

/// The language model at `quant`: `<name>-Q4_K_M.gguf` or `<name>.Q4_K_M.gguf`, never a projector or a split part.
fn pick_model<'a>(files: &'a [RepoFile], quant: &str) -> Option<&'a RepoFile> {
    let q = quant.to_ascii_lowercase();
    files
        .iter()
        .filter(|f| {
            let name = f.path.rsplit('/').next().unwrap_or(&f.path).to_ascii_lowercase();
            let Some(stem) = name.strip_suffix(".gguf") else { return false };
            !name.contains("mmproj")
                && !name.contains("-of-0")
                && (stem.ends_with(&format!("-{q}")) || stem.ends_with(&format!(".{q}")) || stem.ends_with(&format!("_{q}")))
        })
        // Prefer the plain build over variants like "UD-Q4_K_M" or files in subfolders.
        .min_by_key(|f| (f.path.contains('/'), f.path.len()))
}

fn pick_mmproj<'a>(files: &'a [RepoFile], prefs: &[&str]) -> Option<&'a RepoFile> {
    let projectors: Vec<&RepoFile> = files
        .iter()
        .filter(|f| {
            let p = f.path.to_ascii_lowercase();
            is_gguf(&p) && p.contains("mmproj")
        })
        .collect();
    prefs.iter().find_map(|pref| {
        let pref = pref.to_ascii_lowercase();
        projectors
            .iter()
            .filter(|f| f.path.to_ascii_lowercase().contains(&pref))
            .min_by_key(|f| (f.path.contains('/'), f.path.len()))
            .copied()
    })
}

/// Download a catalog model (language model plus projector). Returns the install record.
pub(crate) fn fetch(model: &Model, quant: Option<&str>, force: bool) -> Result<Installed> {
    let agent = agent();
    let quant = quant.unwrap_or(model.quant);
    eprintln!("Looking up {} in {} …", model.id, model.repo);
    let files = list_repo(&agent, model.repo)?;
    let Some(main) = pick_model(&files, quant) else {
        let ggufs: Vec<_> = files.iter().filter(|f| is_gguf(&f.path)).map(|f| f.path.as_str()).collect();
        bail!("{} has no {quant} build. GGUF files there:\n  {}", model.repo, ggufs.join("\n  "));
    };
    let proj = pick_mmproj(&files, model.mmproj);
    if proj.is_none() {
        eprintln!("warning: {} has no vision projector; the model will be text-only", model.repo);
    }

    let dir = models_dir().join(model.id);
    fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
    let mut installed = Installed { id: model.id.to_owned(), repo: model.repo.to_owned(), files: Vec::new() };
    for (name, file) in [("model.gguf", Some(main)), ("mmproj.gguf", proj)] {
        let Some(file) = file else { continue };
        let sha = file.lfs.as_ref().map(|l| l.oid.clone());
        let size = file.lfs.as_ref().map_or(file.size, |l| l.size);
        let dest = dir.join(name);
        let previous = Installed::load(model.id);
        let same = previous.as_ref().and_then(|p| p.files.iter().find(|f| f.name == name)).is_some_and(|f| {
            f.source == file.path && f.sha256 == sha && fs::metadata(&dest).is_ok_and(|m| m.len() == size)
        });
        if same && !force {
            eprintln!("{name}: already have {} ({})", file.path, human(size));
        } else {
            let url = format!("{}/{}/resolve/main/{}", endpoint(), model.repo, file.path);
            eprintln!("{name}: {} ({})", file.path, human(size));
            download(&agent, &url, &dest, size, sha.as_deref())?;
        }
        installed.files.push(InstalledFile { name: name.to_owned(), source: file.path.clone(), size, sha256: sha });
    }
    fs::write(dir.join("model.json"), serde_json::to_string_pretty(&installed)? + "\n")?;
    Ok(installed)
}

/// Download `url` to `dest` through `dest.partial`, resuming a previous attempt, then verify the SHA-256.
fn download(agent: &ureq::Agent, url: &str, dest: &Path, size: u64, sha256: Option<&str>) -> Result<()> {
    let partial = dest.with_extension("gguf.partial");
    let mut have = fs::metadata(&partial).map_or(0, |m| m.len());
    if have > size {
        fs::remove_file(&partial)?;
        have = 0;
    }
    // Hash what's already on disk so the final check covers the whole file.
    let mut hasher = Sha256::new();
    if have > 0 {
        let mut f = File::open(&partial)?;
        io::copy(&mut (&mut f).take(have), &mut HashWriter(&mut hasher))?;
        eprintln!("  resuming at {}", human(have));
    }

    if have < size {
        let mut req = with_token(agent.get(url));
        if have > 0 {
            req = req.header("Range", &format!("bytes={have}-"));
        }
        let mut resp = req.call().with_context(|| format!("download {url}"))?;
        let mut out = OpenOptions::new().create(true).append(true).open(&partial)?;
        if have > 0 && resp.status() != 206 {
            // The server ignored the range: start over.
            out.set_len(0)?;
            out.seek(SeekFrom::Start(0))?;
            hasher = Sha256::new();
            have = 0;
        }
        let mut reader = resp.body_mut().with_config().limit(u64::MAX).reader();
        let mut buf = vec![0u8; 1 << 20];
        let (started, from) = (Instant::now(), have);
        let mut last_report = Instant::now();
        loop {
            let n = reader.read(&mut buf).context("download interrupted; run the same command again to resume")?;
            if n == 0 {
                break;
            }
            out.write_all(&buf[..n])?;
            hasher.update(&buf[..n]);
            have += n as u64;
            if last_report.elapsed() >= Duration::from_millis(500) {
                last_report = Instant::now();
                progress(have, size, have - from, started.elapsed());
            }
        }
        out.sync_all()?;
        progress(have, size, have - from, started.elapsed());
        eprintln!();
    }

    anyhow::ensure!(have == size, "download stopped at {} of {}; run again to resume", human(have), human(size));
    let got = hex(&hasher.finalize());
    if let Some(want) = sha256
        && !got.eq_ignore_ascii_case(want)
    {
        fs::remove_file(&partial).ok();
        bail!("checksum mismatch for {}: expected {want}, got {got}; the partial file was removed", dest.display());
    }
    fs::rename(&partial, dest)?;
    Ok(())
}

/// Re-hash a downloaded model's files against `model.json`.
pub(crate) fn verify(installed: &Installed) -> Result<()> {
    for f in &installed.files {
        let path = models_dir().join(&installed.id).join(&f.name);
        let mut hasher = Sha256::new();
        io::copy(
            &mut File::open(&path).with_context(|| format!("open {}", path.display()))?,
            &mut HashWriter(&mut hasher),
        )?;
        let got = hex(&hasher.finalize());
        match &f.sha256 {
            Some(want) if want.eq_ignore_ascii_case(&got) => eprintln!("{}/{}: ok", installed.id, f.name),
            Some(want) => bail!("{}/{}: expected {want}, got {got}", installed.id, f.name),
            None => eprintln!("{}/{}: no published hash (sha256 {got})", installed.id, f.name),
        }
    }
    Ok(())
}

struct HashWriter<'a>(&'a mut Sha256);

impl Write for HashWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn progress(have: u64, size: u64, moved: u64, elapsed: Duration) {
    let pct = if size == 0 { 100.0 } else { have as f64 / size as f64 * 100.0 };
    let rate = moved as f64 / elapsed.as_secs_f64().max(0.001);
    let eta = if rate > 0.0 { (size - have.min(size)) as f64 / rate } else { 0.0 };
    eprint!("\r  {pct:5.1}%  {} of {}  {}/s  {:>4.0}s left   ", human(have), human(size), human(rate as u64), eta);
}

pub(crate) fn human(bytes: u64) -> String {
    let b = bytes as f64;
    if b >= 1e9 {
        format!("{:.2} GB", b / 1e9)
    } else if b >= 1e6 {
        format!("{:.0} MB", b / 1e6)
    } else {
        format!("{:.0} KB", b / 1e3)
    }
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

fn is_gguf(path: &str) -> bool {
    Path::new(path).extension().is_some_and(|e| e.eq_ignore_ascii_case("gguf"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str) -> RepoFile {
        RepoFile { kind: "file".into(), path: path.into(), size: 1, lfs: None }
    }

    #[test]
    fn picks_quant_and_projector() {
        let files = [
            "Qwen3.5-2B-BF16.gguf",
            "Qwen3.5-2B-UD-Q4_K_XL.gguf",
            "Qwen3.5-2B-Q4_K_M.gguf",
            "Qwen3.5-2B-UD-Q4_K_M.gguf",
            "mmproj-BF16.gguf",
            "mmproj-F16.gguf",
            "BF16/Qwen3.5-2B-BF16-00001-of-00002.gguf",
        ]
        .map(file);
        assert_eq!(pick_model(&files, "Q4_K_M").unwrap().path, "Qwen3.5-2B-Q4_K_M.gguf");
        assert_eq!(pick_mmproj(&files, &["F16", "BF16"]).unwrap().path, "mmproj-F16.gguf");
        let lfm = ["LFM2.5-VL-1.6B-Q4_0.gguf", "LFM2.5-VL-1.6B-Q8_0.gguf", "mmproj-LFM2.5-VL-1.6b-Q8_0.gguf"].map(file);
        assert_eq!(pick_model(&lfm, "Q4_0").unwrap().path, "LFM2.5-VL-1.6B-Q4_0.gguf");
        assert_eq!(pick_mmproj(&lfm, &["Q8_0"]).unwrap().path, "mmproj-LFM2.5-VL-1.6b-Q8_0.gguf");
        assert!(pick_model(&lfm, "Q4_K_M").is_none());
    }

    #[test]
    fn catalog_ids_are_unique() {
        let mut ids: Vec<_> = CATALOG.iter().map(|m| m.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), CATALOG.len());
        assert!(find(DEFAULT).is_ok());
    }
}
