//! Hoard for one person: the asset library in a window, with no server to set up.
//!
//! The app runs the Hoard server inside itself, on a loopback address only this computer can reach, and shows the
//! portal in its window (see `hoardd`'s `desktop` module for what's different from a team's server). The engine
//! plugins, the `hoard` CLI and MCP clients reach it at that address, with an access token made in Settings → Account.
//!
//! The library lives in the app's data folder; thumbnails, downloaded tools and models in its cache folder; the log
//! in its log folder. An update found at start downloads in the background and installs when the app quits.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{File, OpenOptions};
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::{Context as _, Result};
use tauri::webview::{DownloadEvent, NewWindowResponse};
use tauri::{AppHandle, Manager as _, RunEvent, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt as _;
use tauri_plugin_updater::{Update, UpdaterExt as _};

/// The port the app asks for the first time: a server's, so the plugins' default address finds it.
const FIRST_PORT: u16 = 4627;
/// How long quitting waits for the server to finish what it's doing.
const STOP_WAIT: Duration = Duration::from_secs(10);
/// A log bigger than this starts over when the app does.
const LOG_LIMIT: u64 = 16 * 1024 * 1024;

/// The server's thread, joined when the app quits.
static SERVER: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
/// An update downloaded and waiting for the app to quit.
static PENDING: Mutex<Option<(Update, Vec<u8>)>> = Mutex::new(None);

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| show(app)))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            start(app.handle())?;
            Ok(())
        })
        .build(tauri::generate_context!());
    let app = match app {
        Ok(app) => app,
        Err(e) => {
            tracing::error!(error = %e, "couldn't start");
            std::process::exit(1);
        }
    };
    app.run(|_, event| {
        if let RunEvent::Exit = event {
            stop();
        }
    });
}

/// Start logging and the server, and open the window on a page that waits for it.
fn start(app: &AppHandle) -> Result<()> {
    let paths = app.path();
    let data = paths.app_data_dir()?;
    let cache = paths.app_cache_dir()?;
    log_to(&paths.app_log_dir()?);
    let web = web_dir(&paths.resource_dir()?);
    let listen = SocketAddr::from((Ipv4Addr::LOCALHOST, port(&data)));
    tracing::info!(version = env!("CARGO_PKG_VERSION"), %listen, data = %data.display(), "starting Hoard");

    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
        .title("Hoard")
        .inner_size(1440.0, 900.0)
        .min_inner_size(720.0, 480.0)
        // The portal takes files dropped on it itself.
        .disable_drag_drop_handler()
        .on_navigation({
            let app = app.clone();
            move |url| stays_inside(&app, url)
        })
        .on_new_window({
            let app = app.clone();
            move |url, _| {
                if stays_inside(&app, &url) {
                    NewWindowResponse::Allow
                } else {
                    NewWindowResponse::Deny
                }
            }
        })
        .on_download({
            let app = app.clone();
            move |_, event| download(&app, event)
        })
        .build()?;

    let server = std::thread::Builder::new().name("hoardd".into()).spawn({
        let window = window.clone();
        let cfg = hoardd::Config::desktop(data, cache, web, listen);
        move || serve(cfg, &window)
    })?;
    *SERVER.lock().unwrap_or_else(PoisonError::into_inner) = Some(server);

    tauri::async_runtime::spawn({
        let window = window.clone();
        async move {
            hoardd::listening().await;
            if let Some(url) = hoardd::launch_url().and_then(|u| Url::parse(&u).ok()) {
                window.navigate(url).ok();
            }
        }
    });
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = fetch_update(&app).await {
            tracing::info!(error = %format!("{e:#}"), "couldn't check for an update");
        }
    });
    Ok(())
}

/// Run the server until [`hoardd::shut_down`]. If it can't start, the waiting page says why.
fn serve(cfg: hoardd::Config, window: &WebviewWindow) {
    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("hoardd")
        .build()
        .context("start the server's runtime")
        .and_then(|rt| {
            let result = rt.block_on(hoardd::run(cfg));
            // Background work (a conversion, a model download) doesn't hold up quitting.
            rt.shutdown_timeout(Duration::from_secs(2));
            result
        });
    if let Err(e) = result {
        let message = format!("{e:#}");
        tracing::error!(error = %message, "the server stopped");
        // The waiting page may still be loading: say it again for a few seconds (saying it twice does no harm).
        let js = format!("window.failed && window.failed({})", serde_json::Value::from(message));
        for _ in 0..12 {
            window.eval(&js).ok();
            std::thread::sleep(Duration::from_millis(250));
        }
    }
}

/// Quit: stop the server, wait for it, and install an update if one is waiting.
fn stop() {
    hoardd::shut_down();
    let server = SERVER.lock().unwrap_or_else(PoisonError::into_inner).take();
    if let Some(server) = server {
        let (done, wait) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            server.join().ok();
            done.send(()).ok();
        });
        if wait.recv_timeout(STOP_WAIT).is_err() {
            tracing::warn!("the server didn't stop in time");
        }
    }
    let pending = PENDING.lock().unwrap_or_else(PoisonError::into_inner).take();
    if let Some((update, bytes)) = pending {
        tracing::info!(version = %update.version, "installing the update");
        if let Err(e) = update.install(bytes) {
            tracing::warn!(error = %e, "couldn't install the update");
        }
    }
}

/// Bring the window forward (another copy of the app was started).
fn show(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.unminimize().ok();
        window.show().ok();
        window.set_focus().ok();
    }
}

/// Whether the window may go to `url`: the server and the app's own pages, yes; anywhere else opens in the
/// browser instead.
fn stays_inside(app: &AppHandle, url: &Url) -> bool {
    let inside = match url.scheme() {
        "http" | "https" => url.host_str().is_some_and(is_loopback),
        "tauri" | "about" | "data" | "blob" => true,
        _ => false,
    };
    if !inside && let Err(e) = app.opener().open_url(url.as_str(), None::<&str>) {
        tracing::warn!(%url, error = %e, "couldn't open a link");
    }
    inside
}

/// A loopback address: `localhost` or a name under it (Windows serves the app's own pages from
/// `http://tauri.localhost`), or a loopback IP.
fn is_loopback(host: &str) -> bool {
    let host = host.trim_start_matches('[').trim_end_matches(']').trim_end_matches('.').to_ascii_lowercase();
    host == "localhost"
        || host.ends_with(".localhost")
        || host.parse::<std::net::IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

/// Save downloads in the Downloads folder (macOS's web view doesn't save them anywhere by itself), and show the
/// file when it's there.
fn download(app: &AppHandle, event: DownloadEvent<'_>) -> bool {
    match event {
        DownloadEvent::Requested { url, destination } => {
            let name = destination
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .or_else(|| url.path_segments().and_then(|mut s| s.next_back()).map(str::to_owned))
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| "download".to_owned());
            if let Ok(dir) = app.path().download_dir() {
                *destination = unused_path(&dir, &name);
            }
            true
        }
        DownloadEvent::Finished { path: Some(path), success: true, .. } => {
            app.opener().reveal_item_in_dir(&path).ok();
            true
        }
        _ => true,
    }
}

/// `dir/name`, or `dir/name (2)` and so on if that's taken.
fn unused_path(dir: &Path, name: &str) -> PathBuf {
    let first = dir.join(name);
    if !first.exists() {
        return first;
    }
    let (stem, ext) = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem, format!(".{ext}")),
        _ => (name, String::new()),
    };
    (2..10_000).map(|n| dir.join(format!("{stem} ({n}){ext}"))).find(|p| !p.exists()).unwrap_or(first)
}

/// The port to listen on: the one used last time (the window's saved settings belong to its address), else
/// [`FIRST_PORT`], else any free one. Remembered in the data folder.
fn port(data: &Path) -> u16 {
    let file = data.join("desktop-port");
    let saved = std::fs::read_to_string(&file).ok().and_then(|s| s.trim().parse::<u16>().ok());
    let free = |p: u16| TcpListener::bind((Ipv4Addr::LOCALHOST, p)).ok().and_then(|l| l.local_addr().ok());
    let port = saved.into_iter().chain([FIRST_PORT, 0]).find_map(|p| free(p).map(|a| a.port())).unwrap_or(FIRST_PORT);
    if saved != Some(port) {
        std::fs::create_dir_all(data).and_then(|()| std::fs::write(&file, port.to_string())).ok();
    }
    port
}

/// The portal's files: bundled with the app, or the repository's `web/dist` when run with `cargo run`.
fn web_dir(resources: &Path) -> PathBuf {
    let bundled = resources.join("web");
    if bundled.join("index.html").is_file() {
        return bundled;
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web/dist")
}

/// Log to `hoard.log` in `dir` (and to the terminal, when there is one).
fn log_to(dir: &Path) {
    use tracing_subscriber::prelude::*;
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,tower_http=warn"));
    let file = std::fs::create_dir_all(dir).ok().and_then(|()| log_file(&dir.join("hoard.log")).ok());
    let file_layer = file.map(|f| tracing_subscriber::fmt::layer().with_ansi(false).with_writer(Mutex::new(f)));
    tracing_subscriber::registry().with(filter).with(tracing_subscriber::fmt::layer()).with(file_layer).try_init().ok();
}

fn log_file(path: &Path) -> std::io::Result<File> {
    let big = std::fs::metadata(path).is_ok_and(|m| m.len() > LOG_LIMIT);
    OpenOptions::new().create(true).append(!big).write(true).truncate(big).open(path)
}

/// Download an update, if there is one, to install when the app quits.
async fn fetch_update(app: &AppHandle) -> Result<()> {
    let Some(update) = app.updater()?.check().await? else {
        return Ok(());
    };
    tracing::info!(version = %update.version, "downloading an update");
    let bytes = update.download(|_, _| {}, || {}).await?;
    tracing::info!(version = %update.version, "the update installs when Hoard quits");
    *PENDING.lock().unwrap_or_else(PoisonError::into_inner) = Some((update, bytes));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback() {
        assert!(is_loopback("127.0.0.1") && is_loopback("[::1]") && is_loopback("LocalHost"));
        assert!(is_loopback("tauri.localhost"));
        assert!(!is_loopback("example.com") && !is_loopback("192.168.0.1") && !is_loopback("localhost.example.com"));
    }
}
