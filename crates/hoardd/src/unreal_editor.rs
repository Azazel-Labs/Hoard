//! Optional, version-specific Unreal Editor installation from Epic's Linux development images.
//! crane merges OCI layers before GNU tar selects the engine tree; no Docker daemon or socket is needed.
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};
use serde::Deserialize;
use sha2::Digest;
use tokio::process::{Child, Command};

use crate::tools::{self, Slot, Status};

const REPO: &str = "ghcr.io/epicgames/unreal-engine";
const TREE: &str = "home/ue4/UnrealEngine";
const EDITOR: &str = "Engine/Binaries/Linux/UnrealEditor";
pub(crate) const DEFAULT_IMAGE: &str = "5.8";
static SLOT: Slot<Tool> = Slot::new();

#[derive(Clone, Debug)]
struct Tool {
    path: PathBuf,
    version: String,
}

pub(crate) fn describe() -> String {
    SLOT.describe(|t| format!("Unreal Engine {} — {}", t.version, t.path.display()))
}

pub(crate) fn can_download() -> bool {
    cfg!(all(target_os = "linux", target_arch = "x86_64"))
}

/// Only Epic development images; accepting arbitrary registries would also accept arbitrary executables.
pub(crate) fn image_ref(value: &str) -> Result<String> {
    let value = value.trim();
    let version = |v: &str| {
        let parts: Vec<_> = v.split('.').collect();
        (2..=3).contains(&parts.len())
            && parts.iter().all(|p| !p.is_empty() && p.len() <= 3 && p.bytes().all(|b| b.is_ascii_digit()))
            && parts[0] == "5"
    };
    if version(value) {
        return Ok(format!("{REPO}:dev-slim-{value}"));
    }
    if let Some(digest) = value.strip_prefix(&format!("{REPO}@")) {
        digest_key(digest)?;
        return Ok(value.into());
    }
    if let Some(tag) = value.strip_prefix(&format!("{REPO}:"))
        && tag.strip_prefix("dev-slim-").or_else(|| tag.strip_prefix("dev-")).is_some_and(version)
    {
        return Ok(value.into());
    }
    bail!("Use an Unreal 5 version (such as 5.8 or 5.8.0), an Epic dev/dev-slim version tag, or {REPO}@sha256:<digest>")
}

fn digest_key(digest: &str) -> Result<&str> {
    let key = digest.strip_prefix("sha256:").context("Expected a SHA-256 image digest")?;
    ensure!(
        key.len() == 64 && key.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "Invalid image digest"
    );
    Ok(key)
}

pub(crate) fn start(setting: &str, image: &str, tools_dir: PathBuf, allow_download: bool) {
    if tools::is_off(setting) {
        SLOT.restart(Status::Off);
        return;
    }
    if tools::is_auto(setting) && !allow_download {
        SLOT.restart(Status::Unavailable("Give the path to your installed Unreal Editor in Settings; automatic installation is available only in Docker".into()));
        return;
    }
    let generation = SLOT.restart(Status::Looking);
    let setting = setting.trim().to_owned();
    let image = image.to_owned();
    tokio::spawn(async move {
        let canceled = || !SLOT.current(generation);
        let result = if tools::is_auto(&setting) {
            match image_ref(&image) {
                Ok(image) => install(&tools_dir.join("unreal"), &image, generation, &canceled).await,
                Err(e) => Err(e),
            }
        } else {
            probe(Path::new(&setting), &canceled).await
        };
        match result {
            Ok(tool) => {
                SLOT.set(generation, Status::Ready(tool));
            }
            Err(e) => {
                if SLOT.set(generation, Status::Unavailable(format!("{e:#}"))) {
                    tracing::warn!(error = %format!("{e:#}"), "Unreal Editor installation unavailable");
                }
            }
        }
    });
}

#[derive(Deserialize)]
struct BuildVersion {
    #[serde(rename = "MajorVersion")]
    major: u32,
    #[serde(rename = "MinorVersion")]
    minor: u32,
    #[serde(rename = "PatchVersion")]
    patch: u32,
}

async fn probe(path: &Path, canceled: &(dyn Fn() -> bool + Sync)) -> Result<Tool> {
    let name = path.file_name().and_then(|n| n.to_str()).context("Expected an editor executable path")?;
    let names =
        if cfg!(windows) { ["UnrealEditor.exe", "UnrealEditor-Cmd.exe"] } else { ["UnrealEditor", "UnrealEditor-Cmd"] };
    ensure!(names.contains(&name), "Select the native UnrealEditor or UnrealEditor-Cmd executable for this system");
    let metadata = std::fs::metadata(path).with_context(|| format!("Editor not found: {}", path.display()))?;
    ensure!(metadata.is_file(), "Editor is not a file");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        ensure!(metadata.permissions().mode() & 0o111 != 0, "Editor is not executable");
    }
    let engine = path
        .ancestors()
        .find(|p| p.file_name().is_some_and(|n| n == "Engine"))
        .context("Expected an editor inside an Unreal Engine directory")?;
    let build: BuildVersion = serde_json::from_slice(&std::fs::read(engine.join("Build/Build.version"))?)?;
    ensure!(build.major == 5, "Expected Unreal Engine 5");
    if cfg!(target_os = "linux") {
        let mut cmd = crate::heavy::command("ldd");
        cmd.arg(path).stdin(Stdio::null());
        let output = run(&mut cmd, canceled, Duration::from_secs(60)).await.context("Cannot load Unreal Editor")?;
        ensure!(!output.contains("not found"), "Install the editor's missing shared libraries on the server: {output}");
    }
    Ok(Tool { path: path.to_owned(), version: format!("{}.{}.{}", build.major, build.minor, build.patch) })
}

/// Nonblocking, cross-process lock. A canceled waiter never touches another install's files.
async fn lock(path: &Path, canceled: &(dyn Fn() -> bool + Sync)) -> Result<File> {
    let file = OpenOptions::new().create(true).truncate(false).write(true).open(path)?;
    loop {
        ensure!(!canceled(), "Canceled");
        match file.try_lock() {
            Ok(()) => return Ok(file),
            Err(std::fs::TryLockError::WouldBlock) => tokio::time::sleep(Duration::from_millis(200)).await,
            Err(e) => return Err(e.into()),
        }
    }
}

fn completed(root: &Path, digest: &str) -> Result<Option<PathBuf>> {
    let install = root.join(digest_key(digest)?);
    if !install.try_exists()? {
        return Ok(None);
    }
    let expected = format!("{REPO}@{digest}");
    ensure!(
        std::fs::read_to_string(install.join(".complete")).ok().as_deref().map(str::trim) == Some(&expected),
        "Incomplete or damaged Unreal installation at {}; move it aside before retrying",
        install.display()
    );
    let editor = install.join(TREE).join(EDITOR);
    ensure!(
        editor.is_file(),
        "Cached Unreal Editor is missing: {}; move the installation aside before retrying",
        editor.display()
    );
    Ok(Some(editor))
}

fn crane(auth: Option<&Path>) -> Command {
    let mut cmd = crate::heavy::command("crane");
    cmd.stdin(Stdio::null());
    if let Some(auth) = auth {
        cmd.env("DOCKER_CONFIG", auth);
    }
    cmd
}

async fn login(canceled: &(dyn Fn() -> bool + Sync)) -> Result<Option<tempfile::TempDir>> {
    let Some(token_path) = std::env::var_os("GHCR_TOKEN_FILE") else {
        return Ok(None);
    };
    let user = std::env::var("GHCR_USER").context("Set GHCR_USER when using GHCR_TOKEN_FILE")?;
    ensure!(!user.trim().is_empty(), "Set GHCR_USER when using GHCR_TOKEN_FILE");
    let auth = tempfile::tempdir()?;
    let mut cmd = crane(Some(auth.path()));
    cmd.args(["auth", "login", "ghcr.io", "-u", &user, "--password-stdin"])
        .stdin(File::open(token_path).context("Cannot open GHCR_TOKEN_FILE")?);
    // Do not return authentication command output; credentials must never enter settings or logs.
    ensure!(
        run(&mut cmd, canceled, Duration::from_secs(120)).await.is_ok(),
        "GHCR login failed; check GHCR_USER, GHCR_TOKEN_FILE and access to Epic's private Unreal repository"
    );
    Ok(Some(auth))
}

async fn install(root: &Path, image: &str, generation: u64, canceled: &(dyn Fn() -> bool + Sync)) -> Result<Tool> {
    ensure!(
        can_download(),
        "Epic's editor images are Linux x86-64 only. Run Hoard in a Linux x86-64 container, or select an installed native editor's path"
    );
    std::fs::create_dir_all(root)?;
    let root = std::fs::canonicalize(root)?;
    // Remember tag resolutions so a completed installation works offline, without credentials or crane.
    let reference = tools::hex(&sha2::Sha256::digest(image.as_bytes()));
    let _reference_lock = lock(&root.join(format!("{reference}.ref.lock")), canceled).await?;
    let ref_file = root.join(format!("{reference}.ref"));
    let pinned = image.split_once('@').map(|(_, d)| d.to_owned());
    let cached = pinned.or_else(|| std::fs::read_to_string(&ref_file).ok());
    if let Some(digest) = cached.as_deref()
        && let Some(editor) = completed(&root, digest.trim())?
    {
        return probe(&editor, canceled).await;
    }
    let auth = login(canceled).await?;
    let auth_path = auth.as_ref().map(tempfile::TempDir::path);
    let digest = if let Some(digest) = cached {
        digest.trim().to_owned()
    } else {
        let mut cmd = crane(auth_path);
        cmd.args(["digest", "--platform", "linux/amd64", image]);
        run(&mut cmd, canceled, Duration::from_secs(300)).await
            .context("Cannot resolve Unreal image; install crane and authenticate to ghcr.io with an Epic-linked GitHub account (read:packages)")?.trim().to_owned()
    };
    let key = digest_key(&digest)?;
    let _install_lock = lock(&root.join(format!("{key}.lock")), canceled).await?;
    let editor = if let Some(editor) = completed(&root, &digest)? {
        editor
    } else {
        ensure!(SLOT.set(generation, Status::Downloading), "Canceled");
        let activity = crate::activity::start("unreal", format!("Downloading Unreal Editor ({image})"), None);
        activity.detail("Extracting the Linux editor; this download can take tens of GB");
        let stage = tempfile::Builder::new().prefix(&format!(".install-{key}-")).tempdir_in(&root)?;
        let pinned = format!("{REPO}@{digest}");
        export(&pinned, stage.path(), auth_path, canceled).await?;
        let editor = stage.path().join(TREE).join(EDITOR);
        probe(&editor, canceled).await?;
        ensure!(!canceled(), "Canceled");
        std::fs::write(stage.path().join(".complete"), &pinned)?;
        let install = root.join(key);
        std::fs::rename(stage.path(), &install)?;
        install.join(TREE).join(EDITOR)
    };
    // Publish the reference only after the editor and completion marker have been validated.
    let tool = probe(&editor, canceled).await?;
    ensure!(!canceled(), "Canceled");
    let mut file = tempfile::NamedTempFile::new_in(&root)?;
    std::io::Write::write_all(&mut file, digest.as_bytes())?;
    file.persist(&ref_file)?;
    Ok(tool)
}

/// Stream the merged filesystem straight into tar, without an image-sized temporary archive.
async fn export(image: &str, stage: &Path, auth: Option<&Path>, canceled: &(dyn Fn() -> bool + Sync)) -> Result<()> {
    let mut cmd = crane(auth);
    cmd.args(["export", "--platform", "linux/amd64", image, "-"]);
    extract(&mut cmd, stage, canceled).await
}

async fn extract(cmd: &mut Command, stage: &Path, canceled: &(dyn Fn() -> bool + Sync)) -> Result<()> {
    let mut log = tempfile::tempfile()?;
    cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(log.try_clone()?);
    let mut producer = cmd.spawn().context("Install crane to download Unreal Editor")?;
    let result = async {
        let stream: Stdio = producer.stdout.take().context("Missing crane output")?.try_into()?;
        let mut tar = crate::heavy::command("tar");
        tar.args(["--extract", "--file=-", "--no-same-owner", "--directory"])
            .arg(stage)
            // Handles the optional leading ./ emitted by some tar producers.
            .args(["--wildcards", "--no-anchored", "home/ue4/UnrealEngine/*"])
            .stdin(stream).stdout(Stdio::null()).stderr(log.try_clone()?);
        let mut consumer = tar.spawn().context("GNU tar is required to extract Unreal Editor")?;
        let outcome = wait_children(&mut [&mut producer, &mut consumer], canceled, Duration::from_hours(24)).await;
        if outcome.is_err() {
            consumer.kill().await.ok();
        }
        outcome
    }
    .await;
    if result.is_err() {
        producer.kill().await.ok();
    }
    result.with_context(|| format!("Could not extract Unreal Editor: {}", tail(&mut log).unwrap_or_default()))
}

async fn wait_children(
    children: &mut [&mut Child],
    canceled: &(dyn Fn() -> bool + Sync),
    timeout: Duration,
) -> Result<()> {
    let start = Instant::now();
    loop {
        ensure!(!canceled(), "Canceled");
        ensure!(start.elapsed() < timeout, "Operation timed out");
        let mut done = true;
        for child in children.iter_mut() {
            match child.try_wait()? {
                Some(status) => ensure!(status.success(), "Process exited with {status}"),
                None => done = false,
            }
        }
        if done {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

/// Bounded diagnostics: outputs go to anonymous files, never an unbounded in-memory pipe.
async fn run(cmd: &mut Command, canceled: &(dyn Fn() -> bool + Sync), timeout: Duration) -> Result<String> {
    let mut output = tempfile::tempfile()?;
    let mut error = tempfile::tempfile()?;
    let mut child = cmd.stdout(output.try_clone()?).stderr(error.try_clone()?).spawn()?;
    let result = wait_children(&mut [&mut child], canceled, timeout).await;
    if result.is_err() {
        child.kill().await.ok();
    }
    result.with_context(|| tail(&mut error).unwrap_or_default())?;
    ensure!(output.metadata()?.len() <= 1024 * 1024, "Unexpectedly large command output");
    output.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new();
    output.take(1024 * 1024).read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn tail(file: &mut File) -> Result<String> {
    let length = file.metadata()?.len();
    file.seek(SeekFrom::Start(length.saturating_sub(8192)))?;
    let mut bytes = Vec::new();
    file.take(8192).read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_inputs_are_epic_development_versions_or_digests() {
        assert_eq!(image_ref("5.8").unwrap(), format!("{REPO}:dev-slim-5.8"));
        assert_eq!(image_ref(" 5.8.1 ").unwrap(), format!("{REPO}:dev-slim-5.8.1"));
        for image in [format!("{REPO}:dev-5.8.0"), format!("{REPO}@sha256:{}", "a".repeat(64))] {
            assert_eq!(image_ref(&image).unwrap(), image);
        }
        for image in [
            "",
            "latest",
            "5.8/../../tmp",
            "5.8;id",
            "4.27",
            "evil.example/dev-slim-5.8",
            "ghcr.io/epicgames/unreal-engine:runtime-5.8",
            "ghcr.io/epicgames/unreal-engine@sha256:abc",
        ] {
            assert!(image_ref(image).is_err(), "{image}");
        }
    }

    #[test]
    fn cache_requires_matching_marker_and_editor() {
        let root = tempfile::tempdir().unwrap();
        let digest = format!("sha256:{}", "a".repeat(64));
        assert!(completed(root.path(), &digest).unwrap().is_none());
        let install = root.path().join(digest_key(&digest).unwrap());
        std::fs::create_dir_all(install.join(TREE).join("Engine/Binaries/Linux")).unwrap();
        assert!(completed(root.path(), &digest).is_err());
        std::fs::write(install.join(".complete"), format!("{REPO}@{digest}")).unwrap();
        assert!(completed(root.path(), &digest).is_err());
        std::fs::write(install.join(TREE).join(EDITOR), b"editor").unwrap();
        assert!(completed(root.path(), &digest).unwrap().is_some());
        std::fs::write(install.join(".complete"), "wrong image").unwrap();
        assert!(completed(root.path(), &digest).is_err());
        assert!(completed(root.path(), "sha256:../../outside").is_err());
    }

    #[tokio::test]
    async fn lock_waiter_can_cancel_and_lock_releases_on_drop() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("install.lock");
        let first = lock(&path, &|| false).await.unwrap();
        let start = Instant::now();
        assert!(lock(&path, &|| start.elapsed() > Duration::from_millis(100)).await.is_err());
        drop(first);
        assert!(lock(&path, &|| false).await.is_ok());
    }

    #[tokio::test]
    async fn desktop_auto_never_starts_a_download() {
        let dir = tempfile::tempdir().unwrap();
        start("auto", "5.8", dir.path().into(), false);
        assert!(describe().contains("only in Docker"));
        assert!(std::fs::read_dir(dir.path()).unwrap().next().is_none());
        start("off", "5.8", dir.path().into(), false);
        assert_eq!(describe(), "off");
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn merged_tar_keeps_only_engine_tree_and_executable_permissions() {
        use std::os::unix::fs::PermissionsExt;
        for prefix in ["", "./"] {
            let root = tempfile::tempdir().unwrap();
            let archive = root.path().join("image.tar");
            let mut builder = tar::Builder::new(File::create(&archive).unwrap());
            for (name, bytes) in [
                (format!("{prefix}{TREE}/{EDITOR}"), b"editor".as_slice()),
                (format!("{prefix}{TREE}/Engine/Content/test.txt"), b"content".as_slice()),
                (format!("{prefix}etc/should-not-extract"), b"other".as_slice()),
            ] {
                let mut h = tar::Header::new_gnu();
                // Set the raw name so the fixture actually retains the optional ./ prefix.
                h.as_mut_bytes()[..name.len()].copy_from_slice(name.as_bytes());
                h.set_mode(0o755);
                h.set_size(bytes.len() as u64);
                h.set_cksum();
                builder.append(&h, bytes).unwrap();
            }
            builder.finish().unwrap();
            drop(builder);
            let stage = root.path().join("stage");
            std::fs::create_dir(&stage).unwrap();
            let mut cmd = crate::heavy::command("cat");
            cmd.arg(&archive);
            extract(&mut cmd, &stage, &|| false).await.unwrap();
            let exe = stage.join(TREE).join(EDITOR);
            assert_eq!(std::fs::read(&exe).unwrap(), b"editor");
            assert_ne!(std::fs::metadata(exe).unwrap().permissions().mode() & 0o111, 0);
            assert!(stage.join(TREE).join("Engine/Content/test.txt").is_file());
            assert!(!stage.join("etc").exists());
        }
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn extraction_failure_and_cancellation_never_publish_completion() {
        let stage = tempfile::tempdir().unwrap();
        let mut failed = crate::heavy::command("false");
        assert!(extract(&mut failed, stage.path(), &|| false).await.is_err());
        let started = Instant::now();
        let mut sleeping = crate::heavy::command("sleep");
        sleeping.arg("60");
        assert!(
            extract(&mut sleeping, stage.path(), &|| started.elapsed() > Duration::from_millis(100)).await.is_err()
        );
        assert!(started.elapsed() < Duration::from_secs(5));
        assert!(!stage.path().join(".complete").exists());
    }
}
