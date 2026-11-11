//! Run a downloaded model behind llama.cpp's OpenAI-compatible server, which is what Hoard's LLM tagger talks to.
//!
//! llama.cpp isn't downloaded here: its own installers pick the right build for your GPU. We look for the unified
//! `llama` binary (`llama serve`), then the classic `llama-server`, or whatever `$HOARD_LLAMA_BIN` points at.

use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

use crate::models::{self, Installed, Model};

/// How to start a llama.cpp server for one model.
#[derive(Debug, Clone)]
pub(crate) struct Serve {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) ctx: u32,
    /// Extra arguments passed through to llama.cpp.
    pub(crate) extra: Vec<String>,
}

impl Serve {
    pub(crate) fn base_url(&self) -> String {
        let host = if self.host == "0.0.0.0" { "127.0.0.1" } else { &self.host };
        format!("http://{host}:{}", self.port)
    }
}

const INSTALL_HELP: &str =
    "llama.cpp isn't installed (looked for `llama` and `llama-server` on PATH). Install it with one of:
  macOS / Linux:  curl -LsSf https://llama.app/install.sh | sh
  Windows:        winget install llama.cpp
  Homebrew:       brew install llama.cpp
or point HOARD_LLAMA_BIN at a llama-server binary you built.";

/// The llama.cpp binary to run and whether it's the unified `llama` (which needs the `serve` subcommand).
fn server_binary() -> Result<(PathBuf, bool)> {
    if let Some(p) = std::env::var_os("HOARD_LLAMA_BIN") {
        let p = PathBuf::from(p);
        let unified = p.file_stem().is_some_and(|s| s == "llama");
        return Ok((p, unified));
    }
    if let Some(p) = which("llama") {
        return Ok((p, true));
    }
    which("llama-server").map(|p| (p, false)).context(INSTALL_HELP)
}

fn which(name: &str) -> Option<PathBuf> {
    let exe = format!("{name}{}", std::env::consts::EXE_SUFFIX);
    std::env::split_paths(&std::env::var_os("PATH")?).map(|d| d.join(&exe)).find(|p| p.is_file())
}

/// The model's files, downloading them first if needed.
pub(crate) fn ensure_downloaded(model: &Model) -> Result<Installed> {
    if let Some(i) = Installed::load(model.id).filter(|i| i.path("model.gguf").is_some()) {
        return Ok(i);
    }
    eprintln!("{} isn't downloaded yet; fetching it first.", model.id);
    models::fetch(model, None, false)
}

/// Start llama.cpp for `model`. The child inherits stdout/stderr.
pub(crate) fn spawn(model: &Model, installed: &Installed, opts: &Serve) -> Result<Child> {
    let (bin, unified) = server_binary()?;
    let main = installed.path("model.gguf").context("model.gguf missing; run `cargo xtask models fetch` again")?;
    let mut cmd = Command::new(&bin);
    if unified {
        cmd.arg("serve");
    }
    cmd.arg("-m").arg(&main);
    if let Some(proj) = installed.path("mmproj.gguf") {
        cmd.arg("--mmproj").arg(proj);
    }
    cmd.args(["--host", &opts.host, "--port", &opts.port.to_string(), "-c", &opts.ctx.to_string()]);
    // Requests name the model by its catalog id (HOARD_LLM_MODEL).
    cmd.args(["--alias", model.id]);
    if model.thinking {
        // Tagging wants a quick JSON answer, not a chain of thought.
        cmd.args(["--reasoning-budget", "0"]);
    }
    cmd.args(&opts.extra);
    eprintln!("Starting {} for {} on {}", bin.display(), model.id, opts.base_url());
    cmd.spawn().with_context(|| format!("start {}", bin.display()))
}

/// Wait until the server answers `/health` (the model can take a while to load).
pub(crate) fn wait_ready(opts: &Serve, child: &mut Child, timeout: Duration) -> Result<()> {
    let agent: ureq::Agent = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(2))).build().into();
    let url = format!("{}/health", opts.base_url());
    let start = Instant::now();
    while start.elapsed() < timeout {
        if let Some(status) = child.try_wait()? {
            bail!("llama.cpp exited before it was ready ({status})");
        }
        if agent.get(&url).call().is_ok() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    bail!("llama.cpp didn't become ready within {}s", timeout.as_secs())
}

/// Environment that points hoardd at the server.
pub(crate) fn hoard_env(model: &Model, installed: &Installed, opts: &Serve) -> Vec<(&'static str, String)> {
    vec![
        ("HOARD_LLM_URL", format!("{}/v1", opts.base_url())),
        ("HOARD_LLM_MODEL", model.id.to_owned()),
        ("HOARD_LLM_VISION", installed.path("mmproj.gguf").is_some().to_string()),
        // Small models do better with fewer files per request.
        ("HOARD_LLM_BATCH", "12".to_owned()),
    ]
}
