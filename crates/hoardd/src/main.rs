//! `hoardd`: the Hoard asset server.

mod activity;
mod analyze;
mod api;
mod audio;
mod blender;
mod blobs;
mod bundle;
mod config;
mod context;
mod convert;
mod db;
mod error;
mod extract;
mod fbx;
mod ffmpeg;
mod fonts;
mod formats;
mod heavy;
mod ingest;
mod jobs;
mod links;
mod llm;
mod local_llm;
mod mcp;
mod meshes;
mod music;
mod server;
mod settings;
mod sheet;
mod sources;
mod tagger;
mod textures;
mod tools;
mod updates;
mod uploads;

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use clap::Parser;
use tokio::sync::Notify;
use tower_http::compression::CompressionLayer;
use tower_http::compression::predicate::{DefaultPredicate, NotForContentType, Predicate};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::blobs::BlobStore;
use crate::config::Config;

#[derive(Debug)]
pub(crate) struct Inner {
    pub(crate) cfg: Config,
    pub(crate) pool: db::Pool,
    pub(crate) blobs: BlobStore,
    /// Poked whenever a job is enqueued so idle workers wake immediately.
    pub(crate) wake: Notify,
    /// Upload files currently being written (one writer per file).
    pub(crate) uploads_in_flight: std::sync::Mutex<std::collections::HashSet<String>>,
    pub(crate) http: reqwest::Client,
    /// Settings changed from the portal, over `cfg`. Read these, not `cfg`, for anything listed in [`settings::Saved`].
    pub(crate) settings: settings::Store,
}

pub(crate) type AppState = Arc<Inner>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("hoardd=info,tower_http=warn")),
        )
        .init();

    let cfg = Config::parse();
    check_files_dir(&cfg)?;
    for dir in [cfg.data_dir.clone(), cfg.staging_dir(), cfg.tmp_dir()] {
        std::fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
    }
    let pool = db::open(&cfg.db_path(), &cfg.backups_dir())?;
    let blobs = BlobStore::from_config(&cfg)?;
    let requeued = db::requeue_stale_jobs(&*pool.get()?)?;
    let settings = settings::Store::load(&cfg, &*pool.get()?)?;
    let effective = settings.get();
    analyze::set_render_enabled(effective.render, effective.llm_vision && effective.llm_enabled());
    music::configure(effective.soundfont.clone());
    let http = reqwest::Client::builder().timeout(Duration::from_secs(180)).build()?;

    tracing::info!(
        listen = %cfg.listen,
        data = %cfg.data_dir.display(),
        storage = blobs.label(),
        llm = effective.llm_url.as_deref().unwrap_or("off"),
        auth = cfg.token.is_some(),
        blender = %effective.blender,
        ffmpeg = %effective.ffmpeg,
        requeued,
        "starting hoardd"
    );

    let state: AppState = Arc::new(Inner {
        cfg,
        pool,
        blobs,
        wake: Notify::new(),
        uploads_in_flight: std::sync::Mutex::default(),
        http,
        settings,
    });
    match uploads::sweep_stale(&state) {
        Ok(0) => {}
        Ok(n) => tracing::info!(removed = n, "removed abandoned upload sessions"),
        Err(e) => tracing::warn!(error = %e, "could not sweep old upload sessions"),
    }
    ffmpeg::start(&effective.ffmpeg, state.cfg.tools_dir());
    convert::start_blender(&state, false);
    updates::start(effective.update_check);
    analyze::warm_renderer();
    jobs::spawn_workers(&state);

    let mut app = api::router(&state);
    let web = &state.cfg.web_dir;
    if web.join("index.html").is_file() {
        // SPA: unknown paths fall back to index.html so client-side routes work on reload.
        app = app.fallback_service(ServeDir::new(web).fallback(ServeFile::new(web.join("index.html"))));
    } else {
        tracing::warn!(dir = %web.display(), "web portal not built; serving API only (run `npm run build` in web/)");
    }
    let alt_svc = state.cfg.http3.then(|| server::alt_svc(state.cfg.listen.port()));
    let app = app
        .layer(middleware::from_fn(move |req, next| cache_and_alt_svc(req, next, alt_svc.clone())))
        .layer(CompressionLayer::new().br(true).zstd(true).gzip(true).compress_when(compressible()))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::clone(&state));

    server::serve(&state.cfg, app).await?;
    Ok(())
}

/// Refuse to start on an empty files dir while the files are still where they used to be (`<data-dir>/blobs`):
/// that's a compose file updated to separate volumes without moving the files, and every asset would look gone.
fn check_files_dir(cfg: &Config) -> Result<()> {
    let (Some(files), config::StorageKind::Local) = (&cfg.files_dir, cfg.storage) else { return Ok(()) };
    let old = cfg.legacy_blobs_dir();
    let has_entries = |p: &std::path::Path| std::fs::read_dir(p).is_ok_and(|mut d| d.next().is_some());
    if files != &old && has_entries(&old) && !has_entries(files) {
        anyhow::bail!(
            "HOARD_FILES_DIR ({}) is empty, but your files are still in {}. Move them there first (the folders \
             inside `blobs`, not `blobs` itself), or unset HOARD_FILES_DIR to keep using the old place.",
            files.display(),
            old.display()
        );
    }
    Ok(())
}

/// Compress text (JSON, JS, CSS, HTML, SVG), never media or binary downloads: they're already compressed, and
/// on-the-fly compression would drop `Content-Length` and break `Range` requests on multi-GB files.
fn compressible() -> impl Predicate {
    DefaultPredicate::new()
        .and(NotForContentType::new("application/octet-stream"))
        .and(NotForContentType::new("application/zip"))
        .and(NotForContentType::new("model/"))
        .and(NotForContentType::const_new("audio/"))
        .and(NotForContentType::const_new("video/"))
}

/// Cache headers for the portal's own files, and `Alt-Svc` when HTTP/3 is on.
/// Vite puts a content hash in every file name under `/assets/`, so those never change; `index.html` must always be
/// revalidated so a new release is picked up.
async fn cache_and_alt_svc(req: Request, next: Next, alt_svc: Option<String>) -> Response {
    let path = req.uri().path().to_owned();
    let mut resp = next.run(req).await;
    let headers = resp.headers_mut();
    if !path.starts_with("/api/") && !path.starts_with("/mcp") && !headers.contains_key(header::CACHE_CONTROL) {
        let value = if path.starts_with("/assets/") { "public, max-age=31536000, immutable" } else { "no-cache" };
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(value));
    }
    if let Some(v) = alt_svc.as_deref().and_then(|v| HeaderValue::from_str(v).ok()) {
        headers.insert(header::ALT_SVC, v);
    }
    resp
}
