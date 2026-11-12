//! A local model server for development: llama.cpp's `llama-server` in router mode, the same thing the compose files'
//! `llm` service runs. It starts with no model; the portal's Settings downloads, switches and deletes models through
//! it, and they're kept in `<repo>/models`.
//!
//! llama.cpp isn't downloaded here: its own installers pick the right build for your GPU. We look for the unified
//! `llama` binary (`llama serve`), then the classic `llama-server`, or whatever `$HOARD_LLAMA_BIN` points at.

use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

/// How to start the server.
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

/// Where downloaded models are kept.
pub(crate) fn models_dir() -> PathBuf {
    crate::root().join("models")
}

/// Start the server with no model loaded. The child inherits stdout/stderr.
pub(crate) fn spawn(opts: &Serve) -> Result<Child> {
    let (bin, unified) = server_binary()?;
    let dir = models_dir();
    std::fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
    let mut cmd = Command::new(&bin);
    if unified {
        cmd.arg("serve");
    }
    // Downloads from Settings land here.
    cmd.env("LLAMA_CACHE", &dir);
    cmd.args(["--host", &opts.host, "--port", &opts.port.to_string(), "-c", &opts.ctx.to_string()]);
    // One model in memory at a time; thinking models answer straight away, since tagging wants quick JSON.
    cmd.args(["--models-max", "1", "--reasoning-budget", "0"]);
    cmd.args(&opts.extra);
    eprintln!("Starting {} on {} (models in {})", bin.display(), opts.base_url(), dir.display());
    cmd.spawn().with_context(|| format!("start {}", bin.display()))
}

/// Wait until the server answers `/health`.
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

/// Environment that tells hoardd where the server is. The model is picked in Settings, so AI tagging stays off until
/// then.
pub(crate) fn hoard_env(opts: &Serve) -> Vec<(&'static str, String)> {
    vec![("HOARD_LOCAL_LLM_URL", format!("{}/v1", opts.base_url()))]
}
