//! ffmpeg, for the parts of audio that aren't worth writing ourselves: MP3, Vorbis and Opus encoding with the
//! reference encoders (LAME, libvorbis, libopus), and reading formats Hoard's own decoder doesn't (AAC/M4A, Opus,
//! WMA, CAF, `WavPack`, …).
//!
//! Nothing waits for it. At startup a background task finds a usable ffmpeg (the one in Settings, one downloaded
//! earlier, or a system one with the right encoders) or downloads a static build into `<data dir>/tools`, checking
//! its SHA-256 against the publisher's. Until it's ready, or if it never is (offline, `off`), MP3 and Opus fall back
//! to the built-in pure-Rust encoders and the formats only ffmpeg can read aren't offered.
//!
//! Uploaded files are untrusted, so ffmpeg runs with a forced input format (no playlists or concat scripts that
//! could pull in other files), local files only, no stdin, a time limit, and at low priority.

use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};

use crate::tools::{self, Slot, Status};

/// What ffmpeg can do here.
#[derive(Clone, Debug)]
pub(crate) struct Tool {
    pub(crate) path: PathBuf,
    /// First line of `ffmpeg -version`, e.g. `ffmpeg version 9.0.2`.
    version: String,
    mp3: bool,
    vorbis: bool,
    opus: bool,
}

static SLOT: Slot<Tool> = Slot::new();

/// The ffmpeg to use, if one is ready. Never waits.
pub(crate) fn tool() -> Option<Tool> {
    SLOT.ready()
}

/// Whether ffmpeg is ready to encode this format with its reference encoder.
pub(crate) fn encodes(format: &str) -> bool {
    tool().is_some_and(|t| match format {
        "mp3" => t.mp3,
        "ogg" => t.vorbis,
        "opus" => t.opus,
        "wav" | "flac" => true,
        _ => false,
    })
}

/// One line for the status page.
pub(crate) fn describe() -> String {
    SLOT.describe(|t| t.version.clone())
}

/// Whether there's a static build to download for this machine.
pub(crate) fn can_download() -> bool {
    source().is_some()
}

/// Find or fetch ffmpeg in the background. `setting` is `auto`, `off` or a path. Calling it again (the setting
/// changed in the portal) drops whatever the last call was doing.
pub(crate) fn start(setting: &str, tools_dir: PathBuf) {
    let setting = setting.trim().to_owned();
    if tools::is_off(&setting) {
        SLOT.restart(Status::Off);
        return;
    }
    let generation = SLOT.restart(Status::Looking);
    tokio::spawn(async move {
        if tools::is_auto(&setting) {
            auto(generation, &tools_dir).await;
        } else {
            match probe(Path::new(&setting)).await {
                Ok(t) => ready(generation, t),
                Err(e) => {
                    tracing::warn!(path = %setting, error = %format!("{e:#}"), "the ffmpeg setting isn't usable");
                    SLOT.set(generation, Status::Unavailable(format!("{setting}: {e:#}")));
                }
            }
        }
    });
}

fn ready(generation: u64, t: Tool) {
    let (path, version, mp3, vorbis, opus) = (t.path.display().to_string(), t.version.clone(), t.mp3, t.vorbis, t.opus);
    if SLOT.set(generation, Status::Ready(t)) {
        tracing::info!(%path, %version, mp3, vorbis, opus, "ffmpeg ready");
    }
}

fn exe_name() -> &'static str {
    if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" }
}

/// A downloaded ffmpeg, then a complete system one, then download (retrying hourly while offline).
async fn auto(generation: u64, tools_dir: &Path) {
    let ours = tools_dir.join("ffmpeg").join(exe_name());
    if ours.is_file()
        && let Ok(t) = probe(&ours).await
    {
        return ready(generation, t);
    }
    if let Ok(t) = probe(Path::new(exe_name())).await {
        if t.mp3 && t.vorbis && t.opus {
            return ready(generation, t);
        }
        tracing::info!(version = %t.version, "system ffmpeg lacks LAME, libvorbis or libopus; downloading a full build");
    }
    loop {
        if !SLOT.set(generation, Status::Downloading) {
            return;
        }
        match download(generation, tools_dir).await {
            Ok(path) => match probe(&path).await {
                Ok(t) => return ready(generation, t),
                Err(e) => {
                    tracing::warn!(error = %format!("{e:#}"), "downloaded ffmpeg doesn't run");
                    SLOT.set(generation, Status::Unavailable(format!("downloaded ffmpeg doesn't run: {e:#}")));
                    return;
                }
            },
            Err(e) => {
                if !SLOT.current(generation) {
                    return;
                }
                let msg = format!("{e:#}");
                tracing::warn!(error = %msg, "couldn't download ffmpeg; using built-in encoders, retrying in an hour");
                SLOT.set(generation, Status::Unavailable(msg));
            }
        }
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

/// Run `-version` and `-encoders` to see what this ffmpeg can do.
async fn probe(path: &Path) -> Result<Tool> {
    let run = |arg: &'static str| async move {
        let out = tokio::time::timeout(
            Duration::from_secs(20),
            crate::heavy::command(path).args(["-hide_banner", arg]).stdin(Stdio::null()).output(),
        )
        .await
        .context("ffmpeg didn't answer")??;
        ensure!(out.status.success(), "ffmpeg {arg} exited with {}", out.status);
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    };
    let version = run("-version").await?;
    let encoders = run("-encoders").await?;
    let has = |name: &str| encoders.lines().any(|l| l.split_whitespace().nth(1) == Some(name));
    let version = version.lines().next().unwrap_or("ffmpeg").split(" Copyright").next().unwrap_or("ffmpeg").trim();
    Ok(Tool {
        path: path.to_owned(),
        version: version.to_owned(),
        mp3: has("libmp3lame"),
        vorbis: has("libvorbis"),
        opus: has("libopus"),
    })
}

// ---------------------------------------------------------------------------------------------
// Download

/// Where a static build for this platform comes from, and how its checksum is published.
struct Source {
    url: String,
    checksum: Checksum,
}

enum Checksum {
    /// `<final url>.sha256`, after redirects (Martin Riedl's builds).
    Sibling,
    /// A `sha256sum`-style list with a line for this file name (`BtbN`'s builds).
    List { url: String, name: String },
}

/// Static GPL builds that include LAME, libvorbis and libopus: Martin Riedl's for Linux and macOS
/// (<https://ffmpeg.martin-riedl.de>), `BtbN`'s for Windows (<https://github.com/BtbN/FFmpeg-Builds>).
fn source() -> Option<Source> {
    let riedl = |url: &str| Some(Source { url: url.to_owned(), checksum: Checksum::Sibling });
    let btbn = |url: &str, name: &str| {
        Some(Source {
            url: url.to_owned(),
            checksum: Checksum::List {
                url: "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/checksums.sha256".to_owned(),
                name: name.to_owned(),
            },
        })
    };
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => riedl("https://ffmpeg.martin-riedl.de/redirect/latest/linux/amd64/release/ffmpeg.zip"),
        ("linux", "aarch64") => riedl("https://ffmpeg.martin-riedl.de/redirect/latest/linux/arm64/release/ffmpeg.zip"),
        ("macos", "x86_64") => riedl("https://ffmpeg.martin-riedl.de/redirect/latest/macos/amd64/release/ffmpeg.zip"),
        ("macos", "aarch64") => riedl("https://ffmpeg.martin-riedl.de/redirect/latest/macos/arm64/release/ffmpeg.zip"),
        ("windows", "x86_64") => btbn(
            "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip",
            "ffmpeg-master-latest-win64-gpl.zip",
        ),
        ("windows", "aarch64") => btbn(
            "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-winarm64-gpl.zip",
            "ffmpeg-master-latest-winarm64-gpl.zip",
        ),
        _ => None,
    }
}

/// Builds are 30–200 MB; anything bigger isn't what we asked for.
const MAX_DOWNLOAD: u64 = 400 * 1024 * 1024;

async fn download(generation: u64, tools_dir: &Path) -> Result<PathBuf> {
    let src = source().with_context(|| {
        format!(
            "no ffmpeg build to download for {}/{}; install ffmpeg and give its path in Settings",
            std::env::consts::OS,
            std::env::consts::ARCH
        )
    })?;
    download_from(&src, tools_dir, &|| !SLOT.current(generation)).await
}

async fn download_from(src: &Source, tools_dir: &Path, cancelled: &(dyn Fn() -> bool + Sync)) -> Result<PathBuf> {
    let http = tools::client()?;
    tokio::fs::create_dir_all(tools_dir).await?;
    tracing::info!(url = %src.url, "downloading ffmpeg");
    let zip_path = tools_dir.join(format!("ffmpeg-download-{}.zip", std::process::id()));
    let activity = crate::activity::start("ffmpeg", "Downloading ffmpeg for audio conversions", None);
    let fetched = tools::fetch(&http, &src.url, &zip_path, MAX_DOWNLOAD, &activity, cancelled)
        .await
        .context("ffmpeg download")?;
    activity.detail("checking and unpacking");
    let finish = async {
        let final_url = &fetched.final_url;
        let expected = match &src.checksum {
            Checksum::Sibling => {
                let text = http.get(format!("{final_url}.sha256")).send().await?.error_for_status()?.text().await?;
                text.split_whitespace().next().unwrap_or_default().to_ascii_lowercase()
            }
            Checksum::List { url, name } => {
                let text = http.get(url).send().await?.error_for_status()?.text().await?;
                tools::sum_for(&text, name).unwrap_or_default()
            }
        };
        ensure!(expected.len() == 64, "the ffmpeg checksum couldn't be read");
        ensure!(fetched.sha256 == expected, "the ffmpeg download doesn't match its published SHA-256");
        let dest = tools_dir.join("ffmpeg");
        let zip = zip_path.clone();
        let exe = crate::heavy::run("ffmpeg-unpack", move || unpack(&zip, &dest)).await??;
        tokio::fs::write(
            tools_dir.join("ffmpeg").join("SOURCE"),
            format!("{final_url}\nsha256 {}\nfetched {}\n", fetched.sha256, crate::db::now()),
        )
        .await
        .ok();
        Ok(exe)
    };
    let out = finish.await;
    tokio::fs::remove_file(&zip_path).await.ok();
    out
}

/// Take the ffmpeg executable out of the build's zip, wherever it sits inside.
fn unpack(zip_path: &Path, dest_dir: &Path) -> Result<PathBuf> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open(zip_path)?)?;
    let index = (0..zip.len())
        .find(|&i| {
            zip.by_index(i).is_ok_and(|e| {
                e.is_file() && e.enclosed_name().and_then(|p| p.file_name().map(|n| n == exe_name())).unwrap_or(false)
            })
        })
        .context("no ffmpeg executable in the download")?;
    std::fs::create_dir_all(dest_dir)?;
    let dest = dest_dir.join(exe_name());
    let partial = dest_dir.join(format!("{}.partial", exe_name()));
    {
        let mut entry = zip.by_index(index)?;
        let mut out = std::fs::File::create(&partial)?;
        std::io::copy(&mut (&mut entry).take(MAX_DOWNLOAD * 2), &mut out)?;
        out.flush()?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&partial, std::fs::Permissions::from_mode(0o755))?;
    }
    std::fs::rename(&partial, &dest)?;
    Ok(dest)
}

// ---------------------------------------------------------------------------------------------
// Running

/// ffmpeg's demuxer for a file extension. Forcing it keeps a file from claiming to be a playlist or concat script.
pub(crate) fn demuxer(ext: &str) -> Option<&'static str> {
    Some(match ext.to_ascii_lowercase().as_str() {
        "wav" => "wav",
        "w64" => "w64",
        "flac" => "flac",
        "mp3" => "mp3",
        "ogg" | "oga" | "opus" => "ogg",
        "m4a" | "mp4" | "m4b" | "mov" => "mov",
        "aac" => "aac",
        "wma" => "asf",
        "aif" | "aiff" | "aifc" => "aiff",
        "caf" => "caf",
        "wv" => "wv",
        "ape" => "ape",
        "mka" | "webm" => "matroska",
        "au" => "au",
        _ => return None,
    })
}

/// Output codec arguments for a target format.
fn encoder_args(tool: &Tool, format: &str, lossless_source: bool) -> Result<Vec<&'static str>> {
    Ok(match format {
        "mp3" if tool.mp3 => vec!["-c:a", "libmp3lame", "-q:a", "2"],
        "ogg" if tool.vorbis => vec!["-c:a", "libvorbis", "-q:a", "5"],
        "opus" if tool.opus => vec!["-c:a", "libopus", "-b:a", "128k", "-vbr", "on"],
        "flac" => vec!["-c:a", "flac"],
        "wav" if lossless_source => vec!["-c:a", "pcm_s24le"],
        "wav" => vec!["-c:a", "pcm_s16le"],
        _ => bail!("this ffmpeg can't encode {format}"),
    })
}

/// Convert `input` (with extension `ext`) to `format` at `output`. Runs on the calling (heavy) thread, which the
/// child inherits its priority from. `duration` (seconds), when known, turns ffmpeg's progress into `progress`.
pub(crate) fn convert(
    tool: &Tool,
    input: &Path,
    ext: &str,
    format: &str,
    output: &Path,
    duration: Option<f64>,
    progress: &(dyn Fn(f32) + Sync),
) -> Result<()> {
    let demuxer = demuxer(ext).with_context(|| format!("ffmpeg isn't used for .{ext} files"))?;
    let lossless = matches!(
        ext.to_ascii_lowercase().as_str(),
        "wav" | "w64" | "flac" | "aif" | "aiff" | "aifc" | "wv" | "ape" | "caf"
    );
    let mut args: Vec<std::ffi::OsString> = [
        "-nostdin",
        "-hide_banner",
        "-loglevel",
        "error",
        "-nostats",
        "-progress",
        "pipe:1",
        "-protocol_whitelist",
        "file,pipe",
        "-f",
        demuxer,
        "-i",
    ]
    .iter()
    .map(Into::into)
    .collect();
    args.push(input.as_os_str().to_owned());
    for a in ["-map", "0:a:0", "-vn", "-sn", "-dn", "-map_metadata", "0"] {
        args.push(a.into());
    }
    for a in encoder_args(tool, format, lossless)? {
        args.push(a.into());
    }
    args.extend(["-f".into(), muxer(format).into(), "-y".into(), output.as_os_str().to_owned()]);
    run(tool, &args, duration, progress)
}

fn muxer(format: &str) -> &'static str {
    match format {
        "mp3" => "mp3",
        "ogg" | "opus" => "ogg",
        "flac" => "flac",
        _ => "wav",
    }
}

/// Decode any audio ffmpeg reads into a 16 or 24-bit WAV that Hoard's own code can analyze.
pub(crate) fn decode_to_wav(tool: &Tool, input: &Path, ext: &str, output: &Path) -> Result<()> {
    convert(tool, input, ext, "wav", output, None, &|_| {})
}

/// Encode a WAV Hoard made (music renders) to a lossy format.
pub(crate) fn encode_wav(
    tool: &Tool,
    wav: &Path,
    format: &str,
    output: &Path,
    duration: Option<f64>,
    progress: &(dyn Fn(f32) + Sync),
) -> Result<()> {
    convert(tool, wav, "wav", format, output, duration, progress)
}

/// Run ffmpeg with a time limit, reading `-progress` output.
fn run(tool: &Tool, args: &[std::ffi::OsString], duration: Option<f64>, progress: &(dyn Fn(f32) + Sync)) -> Result<()> {
    // Generous: ffmpeg is far faster than real time, but the machine may be busy ingesting.
    let limit = Duration::from_secs_f64(duration.map_or(600.0, |d| (d * 2.0).clamp(120.0, 3600.0)));
    let mut child = std::process::Command::new(&tool.path)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("couldn't start {}", tool.path.display()))?;
    let stdout = child.stdout.take().context("ffmpeg stdout")?;
    let stderr = child.stderr.take().context("ffmpeg stderr")?;
    let errors = std::thread::spawn(move || {
        let mut text = String::new();
        std::io::BufReader::new(stderr).take(64 * 1024).read_to_string(&mut text).ok();
        text
    });
    let started = Instant::now();
    let total_us = duration.filter(|d| *d > 0.0).map(|d| d * 1e6);
    let status = std::thread::scope(|s| {
        s.spawn(|| {
            for line in std::io::BufReader::new(stdout).lines().map_while(Result::ok) {
                if let (Some(total), Some(us)) = (total_us, line.strip_prefix("out_time_us="))
                    && let Ok(us) = us.trim().parse::<f64>()
                {
                    progress(((us / total) as f32).clamp(0.0, 0.99));
                }
            }
        });
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok::<_, anyhow::Error>(status);
            }
            if started.elapsed() > limit {
                child.kill().ok();
                child.wait().ok();
                bail!("ffmpeg took longer than {} s", limit.as_secs());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    })?;
    let errors = errors.join().unwrap_or_default();
    if !status.success() {
        let tail: Vec<&str> = errors.lines().rev().take(4).collect();
        bail!("ffmpeg failed ({status}): {}", tail.into_iter().rev().collect::<Vec<_>>().join(" | "));
    }
    progress(1.0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A system ffmpeg, if this machine has one (the tests that need it skip otherwise).
    fn system() -> Option<Tool> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().ok()?;
        rt.block_on(probe(Path::new("ffmpeg"))).ok()
    }

    #[test]
    fn every_platform_we_ship_for_has_a_source() {
        if matches!(
            (std::env::consts::OS, std::env::consts::ARCH),
            ("linux" | "macos" | "windows", "x86_64" | "aarch64")
        ) {
            assert!(source().is_some());
        }
    }

    #[test]
    fn demuxers_are_forced_for_every_audio_extension() {
        for ext in ["wav", "m4a", "opus", "wma", "aac", "caf", "wv", "ape", "mka", "aiff", "au", "w64"] {
            assert!(demuxer(ext).is_some(), "{ext}");
        }
        // Playlists and scripts are never handed to ffmpeg.
        for ext in ["m3u8", "txt", "ffconcat", "sdp"] {
            assert!(demuxer(ext).is_none(), "{ext}");
        }
    }

    #[test]
    fn unpacks_the_executable_from_anywhere_in_the_zip() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = dir.path().join("b.zip");
        let mut w = zip::ZipWriter::new(std::fs::File::create(&zip_path).unwrap());
        let o = zip::write::SimpleFileOptions::default();
        w.start_file("build/doc/README.txt", o).unwrap();
        w.write_all(b"hi").unwrap();
        w.start_file(format!("build/bin/{}", exe_name()), o).unwrap();
        w.write_all(b"#!/bin/sh\necho ffmpeg\n").unwrap();
        w.finish().unwrap();
        let exe = unpack(&zip_path, &dir.path().join("out")).unwrap();
        assert_eq!(std::fs::read(&exe).unwrap(), b"#!/bin/sh\necho ffmpeg\n");
    }

    #[test]
    fn converts_with_a_system_ffmpeg() {
        let Some(tool) = system() else { return };
        let dir = tempfile::tempdir().unwrap();
        let wav = dir.path().join("t.wav");
        let samples: Vec<i32> = (0..44100 * 2).map(|i| ((i as f32 * 0.05).sin() * 12000.0) as i32).collect();
        let audio = crate::audio::Audio { samples, channels: 2, rate: 44100, bits: 16 };
        std::fs::write(&wav, crate::audio::write_wav(&audio)).unwrap();
        let seen = std::sync::Mutex::new(Vec::new());
        for fmt in ["mp3", "ogg", "opus", "flac"] {
            if !encodes_with(&tool, fmt) {
                continue;
            }
            let out = dir.path().join(format!("o.{fmt}"));
            convert(&tool, &wav, "wav", fmt, &out, Some(1.0), &|p| seen.lock().unwrap().push(p)).unwrap();
            assert!(std::fs::metadata(&out).unwrap().len() > 500, "{fmt}");
        }
        assert!(seen.lock().unwrap().contains(&1.0));
        // A file whose contents lie about its type fails cleanly rather than being guessed at.
        let fake = dir.path().join("fake.m4a");
        std::fs::write(&fake, b"#EXTM3U\n#EXTINF:1,\n/etc/passwd\n").unwrap();
        assert!(decode_to_wav(&tool, &fake, "m4a", &dir.path().join("x.wav")).is_err());
    }

    /// A fake build served locally: the download must follow a redirect, check the published SHA-256 (both
    /// styles), unpack the executable, and refuse a build that doesn't match.
    #[tokio::test(flavor = "multi_thread")]
    async fn downloads_verifies_and_unpacks() {
        use axum::response::{IntoResponse, Redirect};
        use axum::routing::get;
        use sha2::Digest;
        let mut zip_bytes = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_bytes));
            w.start_file(format!("ffmpeg-9/bin/{}", exe_name()), zip::write::SimpleFileOptions::default()).unwrap();
            w.write_all(b"pretend ffmpeg").unwrap();
            w.finish().unwrap();
        }
        let sum = tools::hex(&sha2::Sha256::digest(&zip_bytes));
        let (z1, z2, s1, s2, s3) = (zip_bytes.clone(), zip_bytes.clone(), sum.clone(), sum.clone(), sum.clone());
        let app = axum::Router::new()
            .route("/latest.zip", get(|| async { Redirect::temporary("/builds/9/ffmpeg.zip") }))
            .route("/builds/9/ffmpeg.zip", get(move || async move { z1.into_response() }))
            .route("/builds/9/ffmpeg.zip.sha256", get(move || async move { format!("{s1}  ffmpeg.zip\n") }))
            .route("/win.zip", get(move || async move { z2.into_response() }))
            .route("/sums", get(move || async move { format!("{}  other.zip\n{s2}  win.zip\n", "0".repeat(64)) }))
            .route("/bad.zip", get(|| async { b"not the build".to_vec().into_response() }))
            .route("/bad.zip.sha256", get(move || async move { format!("{s3}  bad.zip\n") }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, app).await.ok() });

        let dir = tempfile::tempdir().unwrap();
        let sibling = Source { url: format!("{base}/latest.zip"), checksum: Checksum::Sibling };
        let exe = download_from(&sibling, dir.path(), &|| false).await.unwrap();
        assert_eq!(std::fs::read(&exe).unwrap(), b"pretend ffmpeg");
        let source = std::fs::read_to_string(dir.path().join("ffmpeg/SOURCE")).unwrap();
        assert!(source.contains(&format!("sha256 {sum}")) && source.contains("/builds/9/"), "{source}");

        let dir = tempfile::tempdir().unwrap();
        let list = Source {
            url: format!("{base}/win.zip"),
            checksum: Checksum::List { url: format!("{base}/sums"), name: "win.zip".into() },
        };
        assert!(download_from(&list, dir.path(), &|| false).await.is_ok());

        let dir = tempfile::tempdir().unwrap();
        let bad = Source { url: format!("{base}/bad.zip"), checksum: Checksum::Sibling };
        let err = download_from(&bad, dir.path(), &|| false).await.unwrap_err();
        assert!(format!("{err:#}").contains("SHA-256"), "{err:#}");
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0, "the bad download is cleaned up");
    }

    fn encodes_with(t: &Tool, fmt: &str) -> bool {
        match fmt {
            "mp3" => t.mp3,
            "ogg" => t.vorbis,
            "opus" => t.opus,
            _ => true,
        }
    }
}
