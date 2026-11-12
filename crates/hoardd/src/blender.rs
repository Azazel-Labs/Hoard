//! Blender, for converting authoring formats to GLB (see `convert.rs`).
//!
//! Like ffmpeg, nothing waits for it. Switched on (`auto`, from Settings in the portal or `HOARD_BLENDER`), a
//! background task uses a Blender downloaded earlier, then an installed one, and otherwise downloads the newest
//! Blender 4.5 LTS build from Blender's official mirrors into `<data dir>/tools/blender`. The build is checked
//! against the SHA-256 list Blender publishes, read from a different mirror than the build came from. A path uses
//! that Blender as it is.
//!
//! 4.5 LTS rather than the newest release: Blender 5 dropped the Collada (.dae) importer, and the LTS series gets
//! fixes until 2027.

use std::io::{BufReader, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};

use crate::tools::{self, Slot, Status};

/// A Blender that answered `--version`.
#[derive(Clone, Debug)]
pub(crate) struct Tool {
    pub(crate) path: PathBuf,
    /// E.g. `Blender 4.5.16 LTS`.
    version: String,
}

static SLOT: Slot<Tool> = Slot::new();

/// The Blender to use, if one is ready. Never waits.
pub(crate) fn tool() -> Option<Tool> {
    SLOT.ready()
}

/// One line for the status page and the settings panel.
pub(crate) fn describe() -> String {
    SLOT.describe(|t| t.version.clone())
}

/// Whether there's an official build to download for this machine (otherwise, install Blender and give its path).
pub(crate) fn can_download() -> bool {
    platform().is_some()
}

/// Find or fetch Blender in the background. `setting` is `auto`, `off` or a path. `on_ready` runs once Blender is
/// usable, told whether it was downloaded just now. Calling this again drops whatever the last call was doing,
/// including a download in progress.
pub(crate) fn start(setting: &str, tools_dir: PathBuf, on_ready: impl FnOnce(bool) + Send + 'static) {
    let setting = setting.trim().to_owned();
    if tools::is_off(&setting) {
        SLOT.restart(Status::Off);
        return;
    }
    let generation = SLOT.restart(Status::Looking);
    tokio::spawn(async move {
        let found = if tools::is_auto(&setting) {
            auto(generation, &tools_dir).await
        } else {
            match probe(Path::new(&setting)).await {
                Ok(t) => Some((t, false)),
                Err(e) => {
                    tracing::warn!(path = %setting, error = %format!("{e:#}"), "the Blender setting isn't usable");
                    SLOT.set(generation, Status::Unavailable(format!("{setting}: {e:#}")));
                    None
                }
            }
        };
        if let Some((t, downloaded)) = found {
            let (path, version) = (t.path.display().to_string(), t.version.clone());
            if SLOT.set(generation, Status::Ready(t)) {
                tracing::info!(%path, %version, "Blender ready");
                on_ready(downloaded);
            }
        }
    });
}

fn exe_name() -> &'static str {
    if cfg!(windows) { "blender.exe" } else { "blender" }
}

/// Installed Blenders worth trying, most likely first.
fn installed() -> Vec<PathBuf> {
    let mut out = vec![PathBuf::from(exe_name())];
    if cfg!(target_os = "macos") {
        out.push("/Applications/Blender.app/Contents/MacOS/Blender".into());
    }
    if cfg!(windows) {
        // The installer puts each version in its own folder; newest first.
        let root = Path::new(r"C:\Program Files\Blender Foundation");
        let mut dirs: Vec<PathBuf> =
            std::fs::read_dir(root).into_iter().flatten().flatten().map(|e| e.path().join(exe_name())).collect();
        dirs.sort();
        out.extend(dirs.into_iter().rev());
    }
    out
}

/// A downloaded Blender, then an installed one, then download (retrying hourly while offline).
async fn auto(generation: u64, tools_dir: &Path) -> Option<(Tool, bool)> {
    let ours = tools_dir.join("blender").join(exe_name());
    if ours.is_file() {
        match probe(&ours).await {
            Ok(t) => return Some((t, false)),
            Err(e) => tracing::warn!(error = %format!("{e:#}"), "the downloaded Blender doesn't run"),
        }
    }
    for path in installed() {
        if let Ok(t) = probe(&path).await {
            return Some((t, false));
        }
    }
    loop {
        if !SLOT.set(generation, Status::Downloading) {
            return None;
        }
        match download(generation, tools_dir).await {
            Ok(exe) => match probe(&exe).await {
                Ok(t) => return Some((t, true)),
                Err(e) => {
                    tracing::warn!(error = %format!("{e:#}"), "the downloaded Blender doesn't run");
                    SLOT.set(generation, Status::Unavailable(format!("{e:#}")));
                    return None;
                }
            },
            Err(e) => {
                if !SLOT.current(generation) {
                    return None;
                }
                let msg = format!("{e:#}");
                tracing::warn!(error = %msg, "couldn't download Blender; retrying in an hour");
                SLOT.set(generation, Status::Unavailable(msg));
            }
        }
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

/// Run `--version`. A Blender missing a system library says which, with the Debian package that has it.
async fn probe(path: &Path) -> Result<Tool> {
    let out = tokio::time::timeout(
        Duration::from_secs(60),
        crate::heavy::command(path).arg("--version").stdin(Stdio::null()).output(),
    )
    .await
    .context("Blender didn't answer")?
    .with_context(|| format!("couldn't start {}", path.display()))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        if let Some(lib) = missing_library(&stderr) {
            match debian_package(&lib) {
                Some(pkg) => bail!("Blender needs {lib}, which isn't installed (Debian/Ubuntu: apt install {pkg})"),
                None => bail!("Blender needs {lib}, which isn't installed"),
            }
        }
        let tail: Vec<&str> = stderr.lines().rev().take(3).collect();
        bail!("Blender exited with {}: {}", out.status, tail.into_iter().rev().collect::<Vec<_>>().join(" | "));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let version = stdout.lines().map(str::trim).find(|l| l.starts_with("Blender ")).context("not Blender")?;
    Ok(Tool { path: path.to_owned(), version: version.to_owned() })
}

/// `…: error while loading shared libraries: libXi.so.6: cannot open shared object file…` → `libXi.so.6`.
fn missing_library(stderr: &str) -> Option<String> {
    let rest = stderr.split("error while loading shared libraries: ").nth(1)?;
    Some(rest.split(':').next()?.trim().to_owned()).filter(|s| !s.is_empty())
}

/// The libraries portable Blender builds expect from the system, and the Debian packages that ship them.
fn debian_package(lib: &str) -> Option<&'static str> {
    Some(match lib {
        "libX11.so.6" => "libx11-6",
        "libXi.so.6" => "libxi6",
        "libXxf86vm.so.1" => "libxxf86vm1",
        "libXfixes.so.3" => "libxfixes3",
        "libXrender.so.1" => "libxrender1",
        "libXext.so.6" => "libxext6",
        "libxkbcommon.so.0" => "libxkbcommon0",
        "libSM.so.6" => "libsm6",
        "libICE.so.6" => "libice6",
        "libGL.so.1" => "libgl1",
        "libEGL.so.1" => "libegl1",
        "libOpenGL.so.0" => "libopengl0",
        "libwayland-client.so.0" => "libwayland-client0",
        _ => return None,
    })
}

// ---------------------------------------------------------------------------------------------
// Download

/// The LTS series to download.
const SERIES: &str = "4.5";

/// Official mirrors listed on blender.org. `download.blender.org` itself sits behind a browser check that programs
/// can't pass, so builds come from these.
const MIRRORS: &[&str] = &[
    "https://mirrors.ocf.berkeley.edu/blender/release",
    "https://ftp.nluug.nl/pub/graphics/blender/release",
    "https://mirror.clarkson.edu/blender/release",
];

/// Builds are about 400 MB.
const MAX_DOWNLOAD: u64 = 1536 * 1024 * 1024;
/// And about 1.3 GB unpacked.
const MAX_UNPACKED: u64 = 6 * 1024 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Archive {
    TarXz,
    Zip,
}

/// This machine's build: its file name suffix and archive type. Linux is what the image runs; Windows is for running
/// hoardd straight from a dev checkout. Elsewhere (macOS, Linux on ARM), install Blender and give its path.
fn platform() -> Option<(&'static str, Archive)> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Some(("linux-x64.tar.xz", Archive::TarXz)),
        ("windows", "x86_64") => Some(("windows-x64.zip", Archive::Zip)),
        _ => None,
    }
}

async fn download(generation: u64, tools_dir: &Path) -> Result<PathBuf> {
    let (suffix, archive) = platform().with_context(|| {
        format!(
            "Blender has no portable build for {}/{}; install Blender and give its path in Settings",
            std::env::consts::OS,
            std::env::consts::ARCH
        )
    })?;
    let mirrors: Vec<String> = MIRRORS.iter().map(|m| (*m).to_owned()).collect();
    download_from(&mirrors, suffix, archive, tools_dir, &|| !SLOT.current(generation)).await
}

/// The newest `blender-<SERIES>.N-<suffix>` in a mirror's folder listing: `(file name, version)`.
fn newest(listing: &str, suffix: &str) -> Option<(String, String)> {
    let re =
        regex::Regex::new(&format!(r"blender-({}\.(\d+))-{}", regex::escape(SERIES), regex::escape(suffix))).ok()?;
    re.captures_iter(listing)
        .filter_map(|c| Some((c.get(2)?.as_str().parse::<u32>().ok()?, c.get(1)?.as_str().to_owned())))
        .max_by_key(|(patch, _)| *patch)
        .map(|(_, version)| (format!("blender-{version}-{suffix}"), version))
}

async fn download_from(
    mirrors: &[String],
    suffix: &str,
    archive: Archive,
    tools_dir: &Path,
    cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<PathBuf> {
    let http = tools::client()?;
    // The first mirror that answers decides the version.
    let mut found = None;
    let mut last_err = None;
    for (i, m) in mirrors.iter().enumerate() {
        let listing = async {
            anyhow::Ok(http.get(format!("{m}/Blender{SERIES}/")).send().await?.error_for_status()?.text().await?)
        };
        match listing.await {
            Ok(text) => match newest(&text, suffix) {
                Some((name, version)) => {
                    found = Some((i, name, version));
                    break;
                }
                None => last_err = Some(anyhow::anyhow!("no {suffix} build of Blender {SERIES} on {m}")),
            },
            Err(e) => last_err = Some(e.context(format!("listing Blender builds on {m}"))),
        }
    }
    let Some((mirror, name, version)) = found else {
        return Err(last_err.unwrap_or_else(|| anyhow::anyhow!("no Blender mirrors configured")));
    };

    tokio::fs::create_dir_all(tools_dir).await?;
    let url = format!("{}/Blender{SERIES}/{name}", mirrors[mirror]);
    tracing::info!(%url, "downloading Blender");
    let ext = if archive == Archive::Zip { "zip" } else { "tar.xz" };
    let file = tools_dir.join(format!("blender-download-{}.{ext}", std::process::id()));
    let activity =
        crate::activity::start("blender", format!("Downloading Blender {version} for model conversion"), None);
    let fetched =
        tools::fetch(&http, &url, &file, MAX_DOWNLOAD, &activity, cancelled).await.context("Blender download")?;
    activity.detail("checking and unpacking");
    activity.progress(1.0);

    let finish = async {
        // The checksum list from another mirror than the build, so one bad mirror can't vouch for itself.
        let sums_name = format!("blender-{version}.sha256");
        let mut expected = None;
        for m in mirrors.iter().cycle().skip(mirror + 1).take(mirrors.len()) {
            let list = async {
                anyhow::Ok(
                    http.get(format!("{m}/Blender{SERIES}/{sums_name}"))
                        .send()
                        .await?
                        .error_for_status()?
                        .text()
                        .await?,
                )
            };
            if let Ok(text) = list.await
                && let Some(sum) = tools::sum_for(&text, &name)
            {
                expected = Some(sum);
                break;
            }
        }
        let expected = expected.context("Blender's published checksum couldn't be read")?;
        ensure!(fetched.sha256 == expected, "the Blender download doesn't match its published SHA-256");
        if cancelled() {
            bail!("cancelled");
        }

        let partial = tools_dir.join("blender.partial");
        let dest = tools_dir.join("blender");
        let (src, part) = (file.clone(), partial.clone());
        crate::heavy::run("blender-unpack", move || {
            if part.exists() {
                std::fs::remove_dir_all(&part)?;
            }
            match archive {
                Archive::TarXz => unpack_tar_xz(&src, &part),
                Archive::Zip => unpack_zip(&src, &part),
            }
        })
        .await??;
        if dest.exists() {
            tokio::fs::remove_dir_all(&dest).await?;
        }
        tokio::fs::rename(&partial, &dest).await?;
        tokio::fs::write(
            dest.join("SOURCE"),
            format!("{}\nsha256 {}\nfetched {}\n", fetched.final_url, fetched.sha256, crate::db::now()),
        )
        .await
        .ok();
        let exe = dest.join(exe_name());
        ensure!(exe.is_file(), "no {} in the Blender download", exe_name());
        Ok(exe)
    };
    let out = finish.await;
    tokio::fs::remove_file(&file).await.ok();
    if out.is_err() {
        tokio::fs::remove_dir_all(tools_dir.join("blender.partial")).await.ok();
    }
    out
}

/// A path inside the archive without its top folder (`blender-4.5.16-linux-x64/`), or `None` for the folder itself.
/// Anything that would land outside the destination is an error.
fn strip_top(path: &Path) -> Result<Option<PathBuf>> {
    let mut parts = path.components();
    match parts.next() {
        Some(Component::Normal(_)) => {}
        _ => bail!("unexpected path in the Blender archive: {}", path.display()),
    }
    let mut out = PathBuf::new();
    for c in parts {
        match c {
            Component::Normal(p) => out.push(p),
            Component::CurDir => {}
            _ => bail!("unexpected path in the Blender archive: {}", path.display()),
        }
    }
    Ok((!out.as_os_str().is_empty()).then_some(out))
}

fn unpack_tar_xz(archive: &Path, dest: &Path) -> Result<()> {
    let xz = lzma_rust2::XzReader::new(BufReader::new(std::fs::File::open(archive)?), true);
    let mut tar = tar::Archive::new(xz);
    tar.set_preserve_permissions(true);
    std::fs::create_dir_all(dest)?;
    let mut total = 0u64;
    for entry in tar.entries()? {
        let mut entry = entry?;
        let Some(rel) = strip_top(&entry.path()?)? else { continue };
        total += entry.size();
        ensure!(total <= MAX_UNPACKED, "the Blender archive unpacks to more than expected");
        let out = dest.join(&rel);
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if entry.header().entry_type().is_hard_link() {
            // Hard links name their target from the archive root; copy it from where it was unpacked.
            let target = entry.link_name()?.context("hard link without a target")?;
            let target = strip_top(&target)?.context("hard link to the top folder")?;
            std::fs::copy(dest.join(target), &out)?;
            continue;
        }
        entry.unpack(&out)?;
    }
    Ok(())
}

fn unpack_zip(archive: &Path, dest: &Path) -> Result<()> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open(archive)?)?;
    std::fs::create_dir_all(dest)?;
    let mut total = 0u64;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let name = entry.enclosed_name().context("unexpected path in the Blender archive")?;
        let Some(rel) = strip_top(&name)? else { continue };
        let out = dest.join(rel);
        if entry.is_dir() {
            std::fs::create_dir_all(&out)?;
            continue;
        }
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::File::create(&out)?;
        total += std::io::copy(&mut (&mut entry).take(MAX_UNPACKED - total.min(MAX_UNPACKED)), &mut file)?;
        ensure!(total < MAX_UNPACKED, "the Blender archive unpacks to more than expected");
        file.flush()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_newest_patch_of_the_series() {
        let listing = r#"<a href="blender-4.5.9-linux-x64.tar.xz">x</a> <a href="blender-4.5.16-linux-x64.tar.xz">
            <a href="blender-4.5.16-windows-x64.zip"> <a href="blender-4.5.2-linux-x64.tar.xz"> blender-4.50.1-linux-x64.tar.xz"#;
        assert_eq!(
            newest(listing, "linux-x64.tar.xz"),
            Some(("blender-4.5.16-linux-x64.tar.xz".into(), "4.5.16".into()))
        );
        assert_eq!(newest(listing, "windows-arm64.zip"), None);
    }

    #[test]
    fn names_the_missing_library_and_its_package() {
        let err =
            "./blender: error while loading shared libraries: libXi.so.6: cannot open shared object file: No such file";
        let lib = missing_library(err).unwrap();
        assert_eq!(lib, "libXi.so.6");
        assert_eq!(debian_package(&lib), Some("libxi6"));
        assert_eq!(missing_library("Blender 4.5.16 LTS"), None);
    }

    #[test]
    fn strips_the_top_folder_and_refuses_escapes() {
        assert_eq!(strip_top(Path::new("blender-4.5/lib/libfoo.so")).unwrap(), Some(PathBuf::from("lib/libfoo.so")));
        assert_eq!(strip_top(Path::new("blender-4.5/")).unwrap(), None);
        assert!(strip_top(Path::new("blender-4.5/../../etc/passwd")).is_err());
        assert!(strip_top(Path::new("/abs/path")).is_err());
    }

    #[test]
    fn unpacks_a_tar_xz_without_its_top_folder() {
        let dir = tempfile::tempdir().unwrap();
        let mut tar_bytes = Vec::new();
        {
            let mut b = tar::Builder::new(&mut tar_bytes);
            let mut add = |path: &str, data: &[u8]| {
                let mut h = tar::Header::new_gnu();
                h.set_size(data.len() as u64);
                h.set_mode(0o755);
                h.set_cksum();
                b.append_data(&mut h, path, data).unwrap();
            };
            add("blender-4.5.16-linux-x64/blender", b"#!/bin/sh\necho Blender 4.5.16\n");
            add("blender-4.5.16-linux-x64/4.5/scripts/startup.py", b"pass\n");
            b.finish().unwrap();
        }
        let xz_path = dir.path().join("b.tar.xz");
        let mut w =
            lzma_rust2::XzWriter::new(std::fs::File::create(&xz_path).unwrap(), lzma_rust2::XzOptions::with_preset(1))
                .unwrap();
        w.write_all(&tar_bytes).unwrap();
        w.finish().unwrap();
        let out = dir.path().join("out");
        unpack_tar_xz(&xz_path, &out).unwrap();
        assert_eq!(std::fs::read(out.join("blender")).unwrap(), b"#!/bin/sh\necho Blender 4.5.16\n");
        assert!(out.join("4.5/scripts/startup.py").is_file());
    }

    /// Two fake mirrors served locally: the build comes from the first, its checksum from the second, and a build
    /// that doesn't match is refused and cleaned up.
    #[tokio::test(flavor = "multi_thread")]
    async fn downloads_from_a_mirror_and_checks_against_another() {
        use axum::response::IntoResponse;
        use axum::routing::get;
        use sha2::Digest;
        let mut zip_bytes = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_bytes));
            let o = zip::write::SimpleFileOptions::default();
            w.add_directory("blender-4.5.16-windows-x64/", o).unwrap();
            w.start_file(format!("blender-4.5.16-windows-x64/{}", exe_name()), o).unwrap();
            w.write_all(b"pretend blender").unwrap();
            w.start_file("blender-4.5.16-windows-x64/4.5/datafiles/x.txt", o).unwrap();
            w.write_all(b"data").unwrap();
            w.finish().unwrap();
        }
        let name = "blender-4.5.16-windows-x64.zip";
        let sum = tools::hex(&sha2::Sha256::digest(&zip_bytes));
        let listing = format!(r#"<a href="blender-4.5.15-windows-x64.zip"></a><a href="{name}"></a>"#);
        let sums = format!("{}  blender-4.5.16-linux-x64.tar.xz\n{sum}  {name}\n", "0".repeat(64));
        let app = axum::Router::new()
            .route("/a/Blender4.5/", get(move || async move { listing.clone() }))
            .route(&format!("/a/Blender4.5/{name}"), get(move || async move { zip_bytes.clone().into_response() }))
            // The first mirror's own checksum list is wrong; the second one's is used.
            .route("/a/Blender4.5/blender-4.5.16.sha256", get(|| async { format!("{}  x\n", "f".repeat(64)) }))
            .route("/b/Blender4.5/blender-4.5.16.sha256", get(move || async move { sums.clone() }))
            .route("/bad/Blender4.5/", get(|| async { format!(r#"<a href="{}"></a>"#, "blender-4.5.16-windows-x64.zip") }))
            .route("/bad/Blender4.5/blender-4.5.16-windows-x64.zip", get(|| async { b"not it".to_vec().into_response() }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, app).await.ok() });

        let dir = tempfile::tempdir().unwrap();
        let mirrors = vec![format!("{base}/a"), format!("{base}/b")];
        let exe = download_from(&mirrors, "windows-x64.zip", Archive::Zip, dir.path(), &|| false).await.unwrap();
        assert_eq!(exe, dir.path().join("blender").join(exe_name()));
        assert_eq!(std::fs::read(&exe).unwrap(), b"pretend blender");
        assert!(dir.path().join("blender/4.5/datafiles/x.txt").is_file());
        let source = std::fs::read_to_string(dir.path().join("blender/SOURCE")).unwrap();
        assert!(source.contains(&sum), "{source}");

        let dir = tempfile::tempdir().unwrap();
        let mirrors = vec![format!("{base}/bad"), format!("{base}/b")];
        let err = download_from(&mirrors, "windows-x64.zip", Archive::Zip, dir.path(), &|| false).await.unwrap_err();
        assert!(format!("{err:#}").contains("SHA-256"), "{err:#}");
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0, "the bad download is cleaned up");
    }
}
