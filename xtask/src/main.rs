//! `cargo xtask`: Hoard's project automation, in Rust so it runs the same on Linux, macOS and Windows.
//!
//! ```text
//! cargo xtask models list                 # small local models for the LLM tagger
//! cargo xtask models fetch qwen3.5-2b     # download (resumable, SHA-256 verified)
//! cargo xtask llm serve                   # run it behind llama.cpp's OpenAI-compatible server
//! cargo xtask dev --llm                   # hoardd + Vite dev server (+ the local model), wired together
//! cargo xtask ci                          # what CI runs: fmt, clippy, tests, web build
//! ```
#![expect(clippy::print_stdout, reason = "a CLI tool: printing is the point")]

mod llm;
mod models;

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
    /// Small local models for the LLM tagger (GGUF files from Hugging Face).
    #[command(subcommand)]
    Models(ModelsCmd),
    /// Run a downloaded model behind llama.cpp's OpenAI-compatible server.
    #[command(subcommand)]
    Llm(LlmCmd),
    /// Run hoardd and the Vite dev server together (and a local model with `--llm`). Ctrl-C stops everything.
    Dev {
        /// Also serve a local model and point hoardd's LLM tagger at it (default model if no id is given).
        #[arg(long, num_args = 0..=1, default_missing_value = models::DEFAULT, value_name = "MODEL")]
        llm: Option<String>,
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
}

#[derive(Debug, Subcommand)]
enum ModelsCmd {
    /// Show the catalog and what's downloaded.
    List,
    /// Download a model and its vision projector.
    Fetch {
        /// Catalog id (see `models list`).
        #[arg(default_value = models::DEFAULT)]
        id: String,
        #[arg(long, help = "Quantization to fetch instead of the default, e.g. Q8_0 or Q5_K_M")]
        quant: Option<String>,
        /// Download again even if the files are already here.
        #[arg(long)]
        force: bool,
    },
    /// Re-check downloaded files against their published SHA-256.
    Verify {
        /// Catalog id; all downloaded models if omitted.
        id: Option<String>,
    },
    /// Delete a downloaded model.
    Rm { id: String },
    /// Print the models folder.
    Path,
}

#[derive(Debug, Subcommand)]
enum LlmCmd {
    /// Serve a model (downloading it first if needed) and print the settings for hoardd.
    Serve {
        #[arg(default_value = models::DEFAULT)]
        id: String,
        #[command(flatten)]
        serve: ServeArgs,
        #[arg(long, value_name = "PATH", help = "Also write the HOARD_LLM_* settings to this file once ready")]
        env_file: Option<PathBuf>,
        /// Extra llama.cpp arguments, after `--`.
        #[arg(last = true)]
        extra: Vec<String>,
    },
    #[command(about = "Print the HOARD_LLM_* environment for a model, for a .env file or `export`")]
    Env {
        #[arg(default_value = models::DEFAULT)]
        id: String,
        #[command(flatten)]
        serve: ServeArgs,
    },
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
    /// Context window in tokens: a 16-tile contact sheet plus a batch of 12 files fits comfortably in 16k.
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
        Cmd::Models(cmd) => models_cmd(cmd),
        Cmd::Llm(LlmCmd::Serve { id, serve, env_file, extra }) => {
            let model = models::find(&id)?;
            let installed = llm::ensure_downloaded(model)?;
            let opts = serve.to_serve(extra);
            let mut child = llm::spawn(model, &installed, &opts)?;
            llm::wait_ready(&opts, &mut child, Duration::from_secs(300))?;
            let env = llm::hoard_env(model, &installed, &opts);
            if let Some(path) = env_file {
                let text = env.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("\n") + "\n";
                std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
            }
            println!("\n{} is ready. Start hoardd with:\n", model.id);
            for (k, v) in env {
                println!("  {k}={v}");
            }
            println!("\nCtrl-C to stop.");
            let status = child.wait()?;
            check(status, "llama.cpp")
        }
        Cmd::Llm(LlmCmd::Env { id, serve }) => {
            let model = models::find(&id)?;
            let installed = Installed::load(model.id).with_context(|| {
                format!("{} isn't downloaded; run `cargo xtask models fetch {}`", model.id, model.id)
            })?;
            for (k, v) in llm::hoard_env(model, &installed, &serve.to_serve(Vec::new())) {
                println!("{k}={v}");
            }
            Ok(())
        }
        Cmd::Dev { llm, no_web, serve } => dev(llm.as_deref(), no_web, &serve),
        Cmd::Ci => ci(),
        Cmd::Web => web(),
        Cmd::Sample { upload } => sample(upload),
        Cmd::UnrealExport(args) => unreal_export(&args),
    }
}

use models::Installed;

fn models_cmd(cmd: ModelsCmd) -> Result<()> {
    match cmd {
        ModelsCmd::List => {
            println!(
                "Models for the LLM tagger (all take images; sizes are model + projector at the default quant):\n"
            );
            for m in models::CATALOG {
                let have = Installed::load(m.id)
                    .filter(|i| i.path("model.gguf").is_some())
                    .map(|i| format!("downloaded, {}", models::human(i.files.iter().map(|f| f.size).sum())));
                let default = if m.id == models::DEFAULT { " (default)" } else { "" };
                println!(
                    "  {:<16} ~{:.1} GB  {:<22} {}{default}",
                    m.id,
                    m.approx_gb,
                    m.license,
                    have.unwrap_or_default()
                );
                println!("  {:<16} {}", "", m.note);
                println!("  {:<16} https://huggingface.co/{}\n", "", m.repo);
            }
            println!("Files go to {}", models::models_dir().display());
            Ok(())
        }
        ModelsCmd::Fetch { id, quant, force } => {
            let model = models::find(&id)?;
            let installed = models::fetch(model, quant.as_deref(), force)?;
            println!("\n{} is in {}", model.id, models::models_dir().join(&installed.id).display());
            println!("Serve it with: cargo xtask llm serve {}", model.id);
            Ok(())
        }
        ModelsCmd::Verify { id } => {
            let ids: Vec<&str> = match &id {
                Some(id) => vec![models::find(id)?.id],
                None => models::CATALOG.iter().map(|m| m.id).collect(),
            };
            let mut any = false;
            for id in ids {
                if let Some(i) = Installed::load(id) {
                    any = true;
                    models::verify(&i)?;
                }
            }
            if !any {
                println!("Nothing downloaded yet.");
            }
            Ok(())
        }
        ModelsCmd::Rm { id } => {
            let model = models::find(&id)?;
            let dir = models::models_dir().join(model.id);
            if dir.is_dir() {
                std::fs::remove_dir_all(&dir).with_context(|| format!("delete {}", dir.display()))?;
                println!("Deleted {}", dir.display());
            } else {
                println!("{} isn't downloaded.", model.id);
            }
            Ok(())
        }
        ModelsCmd::Path => {
            println!("{}", models::models_dir().display());
            Ok(())
        }
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

fn dev(llm: Option<&str>, no_web: bool, serve: &ServeArgs) -> Result<()> {
    let root = root();
    let mut guard = Guard(Vec::new());
    let mut hoardd = cargo();
    hoardd.current_dir(&root).args(["run", "-p", "hoardd", "--"]);

    if let Some(id) = llm {
        let model = models::find(id)?;
        let installed = llm::ensure_downloaded(model)?;
        let opts = serve.to_serve(Vec::new());
        let mut child = llm::spawn(model, &installed, &opts)?;
        eprintln!("Waiting for the model to load…");
        let ready = llm::wait_ready(&opts, &mut child, Duration::from_secs(300));
        guard.0.push(("llama.cpp", child));
        ready?;
        for (k, v) in llm::hoard_env(model, &installed, &opts) {
            hoardd.env(k, v);
        }
    }

    if !no_web && !root.join("web/node_modules").is_dir() {
        run(npm().current_dir(root.join("web")).arg("ci"), "npm ci")?;
    }
    guard.0.push(("hoardd", hoardd.spawn().context("start hoardd")?));
    if no_web {
        eprintln!("\nhoardd: http://127.0.0.1:7878 (serving web/dist)");
    } else {
        guard.0.push(("vite", npm().current_dir(root.join("web")).args(["run", "dev"]).spawn().context("start vite")?));
        eprintln!("\nPortal with hot reload: http://127.0.0.1:5173 (API on :7878)");
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
