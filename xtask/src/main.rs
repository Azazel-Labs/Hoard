//! `cargo xtask`: Hoard's project automation, in Rust so it runs the same on Linux, macOS and Windows.
//!
//! ```text
//! cargo xtask dev                         # hoardd + Vite dev server with hot reload
//! cargo xtask dev --llm                   # ...plus a local model server (pick a model in Settings)
//! cargo xtask llm                         # just the local model server (llama.cpp in router mode)
//! cargo xtask ci                          # what CI runs: fmt, clippy, tests, web build
//! cargo xtask translations check          # the catalogs in locales/ against each other and the code
//! ```
#![expect(clippy::print_stdout, reason = "a CLI tool: printing is the point")]

mod llm;
mod translations;

use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cargo xtask", about = "Hoard project automation", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Run a local model server (llama.cpp in router mode) for AI tagging; pick and download models in Settings.
    Llm {
        #[command(flatten)]
        serve: ServeArgs,
        #[arg(long, value_name = "PATH", help = "Also write the HOARD_LOCAL_LLM_URL variable to this file once ready")]
        env_file: Option<PathBuf>,
        /// Extra llama.cpp arguments, after `--`.
        #[arg(last = true)]
        extra: Vec<String>,
    },
    /// Run hoardd and the Vite dev server together (and a local model server with `--llm`). Ctrl-C stops everything.
    Dev {
        /// Also run a local model server and point hoardd at it; pick a model in Settings.
        #[arg(long)]
        llm: bool,
        /// Only run hoardd (serving the built portal from web/dist).
        #[arg(long)]
        no_web: bool,
        #[command(flatten)]
        serve: ServeArgs,
    },
    /// Everything CI checks: rustfmt, clippy (warnings are errors), tests and the web build.
    Ci,
    /// Install web dependencies if needed and build the portal into web/dist.
    Web,
    /// Export an Unreal content folder to FBX/PNG/WAV with Unreal Editor (headless), ready to import into Hoard.
    UnrealExport(UnrealExportArgs),
    /// Generate the sample asset pack (needs Python with Pillow and numpy) into target/sample.
    Sample {
        /// Upload it to a running server afterwards.
        #[arg(long)]
        upload: bool,
    },
    /// Check, format and review the translations in locales/ (see docs/translations.md).
    #[command(subcommand)]
    Translations(translations::Action),
}

#[derive(Debug, Args)]
struct UnrealExportArgs {
    /// The pack's Content folder (or a folder containing it), e.g. a Fab download or a project's Content/<Pack>.
    content: PathBuf,
    /// Where the exported files go (default: target/unreal-export/<name>).
    #[arg(long)]
    out: Option<PathBuf>,
    /// Unreal's command-line editor (`UnrealEditor-Cmd`). Defaults to `$UNREAL_EDITOR`.
    #[arg(long, env = "UNREAL_EDITOR")]
    editor: PathBuf,
    /// Upload the result to the Hoard server afterwards (uses `hoard upload`).
    #[arg(long)]
    upload: bool,
}

#[derive(Debug, Clone, Args)]
struct ServeArgs {
    /// Address for llama.cpp to listen on (0.0.0.0 to reach it from containers).
    #[arg(long = "llm-host", default_value = "127.0.0.1")]
    host: String,
    /// Port for llama.cpp.
    #[arg(long = "llm-port", default_value_t = 8081)]
    port: u16,
    /// Context window in tokens: a 16-tile contact sheet plus a batch of 12 assets fits comfortably in 16k.
    #[arg(long = "llm-ctx", default_value_t = 16384)]
    ctx: u32,
}

impl ServeArgs {
    fn to_serve(&self, extra: Vec<String>) -> llm::Serve {
        llm::Serve { host: self.host.clone(), port: self.port, ctx: self.ctx, extra }
    }
}

/// The repository root (this crate lives in `<root>/xtask`).
pub(crate) fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Llm { serve, env_file, extra } => {
            let opts = serve.to_serve(extra);
            let mut child = llm::spawn(&opts)?;
            let ready = llm::wait_ready(&opts, &mut child, Duration::from_secs(60));
            let mut guard = Guard(vec![("llama.cpp", child)]);
            ready?;
            let env = llm::hoard_env(&opts);
            if let Some(path) = env_file {
                let text = env.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(
                    "
",
                ) + "
";
                std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
            }
            println!(
                "
The local model server is ready. Start hoardd with:
"
            );
            for (k, v) in env {
                println!("  {k}={v}");
            }
            println!(
                "
then pick a model in Settings → AI tagging → Local model. Ctrl-C to stop."
            );
            let (_, mut child) = guard.0.remove(0);
            let status = child.wait()?;
            check(status, "llama.cpp")
        }
        Cmd::Dev { llm, no_web, serve } => dev(llm, no_web, &serve),
        Cmd::Ci => ci(),
        Cmd::Web => web(),
        Cmd::Sample { upload } => sample(upload),
        Cmd::UnrealExport(args) => unreal_export(&args),
        Cmd::Translations(action) => translations::run(&root(), action).map_err(|e| anyhow::anyhow!("{e}")),
    }
}

/// A child process that's killed if we leave early.
struct Guard(Vec<(&'static str, Child)>);

impl Drop for Guard {
    fn drop(&mut self) {
        for (_, c) in &mut self.0 {
            c.kill().ok();
            c.wait().ok();
        }
    }
}

fn dev(llm: bool, no_web: bool, serve: &ServeArgs) -> Result<()> {
    let root = root();
    let mut guard = Guard(Vec::new());
    let mut hoardd = cargo();
    hoardd.current_dir(&root).args(["run", "-p", "hoardd", "--"]);

    if llm {
        let opts = serve.to_serve(Vec::new());
        let mut child = llm::spawn(&opts)?;
        let ready = llm::wait_ready(&opts, &mut child, Duration::from_secs(60));
        guard.0.push(("llama.cpp", child));
        ready?;
        for (k, v) in llm::hoard_env(&opts) {
            hoardd.env(k, v);
        }
        eprintln!("Pick a model in Settings → AI tagging → Local model.");
    }

    if !no_web && !root.join("web/node_modules").is_dir() {
        run(npm().current_dir(root.join("web")).arg("ci"), "npm ci")?;
    }
    guard.0.push(("hoardd", hoardd.spawn().context("start hoardd")?));
    if no_web {
        eprintln!("\nhoardd: http://127.0.0.1:4627 (serving web/dist)");
    } else {
        guard.0.push(("vite", npm().current_dir(root.join("web")).args(["run", "dev"]).spawn().context("start vite")?));
        eprintln!("\nPortal with hot reload: http://127.0.0.1:5173 (API on :4627)");
    }

    // Run until any of them stops; the guard stops the rest.
    loop {
        for (name, child) in &mut guard.0 {
            if let Some(status) = child.try_wait()? {
                bail!("{name} exited ({status}); stopping the others");
            }
        }
        std::thread::sleep(Duration::from_millis(300));
    }
}

fn ci() -> Result<()> {
    let root = root();
    run(cargo().current_dir(&root).args(["fmt", "--all", "--", "--check"]), "cargo fmt")?;
    run(
        cargo().current_dir(&root).args(["clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"]),
        "cargo clippy",
    )?;
    run(cargo().current_dir(&root).args(["test", "--workspace", "--locked"]), "cargo test")?;
    web()?;
    println!("\nAll checks passed.");
    Ok(())
}

fn web() -> Result<()> {
    let dir = root().join("web");
    if !dir.join("node_modules").is_dir() {
        run(npm().current_dir(&dir).arg("ci"), "npm ci")?;
    }
    run(npm().current_dir(&dir).args(["run", "build"]), "npm run build")
}

fn sample(upload: bool) -> Result<()> {
    let root = root();
    let out = root.join("target/sample");
    let python = if cfg!(windows) { "python" } else { "python3" };
    run(Command::new(python).current_dir(&root).arg("tools/make_sample_pack.py").arg(&out), "make_sample_pack.py")?;
    let zip = out.join("forest-starter-kit.zip");
    println!("Sample pack: {}", zip.display());
    if upload {
        run(cargo().current_dir(&root).args(["run", "-q", "-p", "hoard", "--", "upload"]).arg(&zip), "hoard upload")?;
    }
    Ok(())
}

/// Wrap a pack's Content folder in a throwaway project and run `tools/unreal_export.py` in the headless editor.
fn unreal_export(args: &UnrealExportArgs) -> Result<()> {
    let root = root();
    // Accept either `.../Content` or a folder that contains one.
    let content = if args.content.file_name().is_some_and(|n| n.eq_ignore_ascii_case("Content")) {
        args.content.clone()
    } else if args.content.join("Content").is_dir() {
        args.content.join("Content")
    } else {
        args.content.clone()
    };
    anyhow::ensure!(content.is_dir(), "{} isn't a folder", content.display());
    let name = content
        .parent()
        .and_then(Path::file_name)
        .or_else(|| content.file_name())
        .map_or_else(|| "pack".to_owned(), |n| n.to_string_lossy().into_owned());
    let work = root.join("target/unreal-export/project");
    let out = args.out.clone().unwrap_or_else(|| root.join("target/unreal-export").join(&name));
    if work.exists() {
        std::fs::remove_dir_all(&work)?;
    }
    std::fs::create_dir_all(&work)?;
    // The editor needs a project; packages mount under /Game from its Content folder.
    std::fs::write(
        work.join("HoardExport.uproject"),
        r#"{ "FileVersion": 3, "EngineAssociation": "", "Plugins": [ { "Name": "PythonScriptPlugin", "Enabled": true } ] }"#,
    )?;
    eprintln!("Copying {} into a temporary project…", content.display());
    copy_dir(&content, &work.join("Content"))?;
    std::fs::create_dir_all(&out)?;
    let script = root.join("tools/unreal_export.py");
    let mut cmd = Command::new(&args.editor);
    cmd.arg(work.join("HoardExport.uproject"))
        .args(["-run=pythonscript", "-unattended", "-nosplash", "-nop4", "-NullRHI", "-stdout"])
        // Script arguments go through the environment: paths with spaces survive Unreal's command-line parsing.
        .env("HOARD_EXPORT_ROOT", "/Game")
        .env("HOARD_EXPORT_OUT", &out);
    script_arg(&mut cmd, &script);
    run(&mut cmd, "Unreal Editor export")?;
    let report = out.join("hoard-export.json");
    anyhow::ensure!(report.is_file(), "the editor finished but wrote no {}; check its log above", report.display());
    println!("Exported to {} (see hoard-export.json)", out.display());
    if args.upload {
        run(
            cargo().current_dir(&root).args(["run", "-q", "-p", "hoard", "--", "upload", "--name", &name]).arg(&out),
            "hoard upload",
        )?;
    }
    Ok(())
}

/// `-script="<path>"`, quoted the way Unreal's parser expects on each platform.
#[cfg(windows)]
fn script_arg(cmd: &mut Command, script: &Path) {
    use std::os::windows::process::CommandExt as _;
    // Windows hands Unreal the raw command line, so the quotes must be literal.
    cmd.raw_arg(format!("-script=\"{}\"", script.display()));
}

/// `-script="<path>"`, quoted the way Unreal's parser expects on each platform.
#[cfg(not(windows))]
fn script_arg(cmd: &mut Command, script: &Path) {
    // Unix launchers rebuild the command line from argv and quote values with spaces themselves.
    cmd.arg(format!("-script={}", script.display()));
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let dest = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &dest)?;
        } else {
            std::fs::copy(entry.path(), &dest)?;
        }
    }
    Ok(())
}

fn cargo() -> Command {
    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
}

fn npm() -> Command {
    Command::new(if cfg!(windows) { "npm.cmd" } else { "npm" })
}

fn run(cmd: &mut Command, what: &str) -> Result<()> {
    eprintln!("» {what}");
    let status = cmd.status().with_context(|| format!("couldn't run {what}"))?;
    check(status, what)
}

fn check(status: ExitStatus, what: &str) -> Result<()> {
    if status.success() { Ok(()) } else { bail!("{what} failed ({status})") }
}
