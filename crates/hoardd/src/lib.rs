//! The Hoard asset server, as a library: [`run`] serves a library from a [`Config`]. The `hoardd` binary runs it
//! from the command line, and the desktop app runs it inside its window.

mod achievements;
mod activity;
mod alternates;
mod analyze;
mod animations;
mod api;
mod atlas;
mod audio;
mod auth;
mod blender;
mod blobs;
mod bundle;
mod catalog;
mod chatgpt;
mod check;
mod chiptune;
mod clap;
mod clip;
mod config;
mod context;
mod convert;
mod credits;
mod db;
mod desktop;
#[cfg(feature = "ldap")]
mod directory;
#[cfg(not(feature = "ldap"))]
#[path = "directory_off.rs"]
mod directory;
mod dragonbones;
mod embed;
mod error;
mod extract;
mod farcopy;
mod ffmpeg;
mod flac;
mod fonts;
mod format_wizard;
mod formats;
mod gamebank;
mod glbfiles;
mod godot;
mod gpu_hint;
mod grid;
mod heavy;
mod hosted;
#[cfg(feature = "https")]
mod http3;
#[cfg(feature = "https")]
mod https;
#[cfg(not(feature = "https"))]
#[path = "https_off.rs"]
mod https;
mod i18n;
mod imitation;
mod ingest;
mod instruments;
mod issues;
mod jobs;
mod links;
mod llm;
mod local_llm;
mod locations;
mod looks;
mod lottie;
mod maintenance;
mod materials;
mod mcp;
mod me;
mod meshes;
mod motion;
mod mtlmat;
mod music;
mod naming;
mod nearcopies;
mod oauth;
mod previews;
mod profiler;
mod projects;
mod provenance;
mod recent_log;
mod reformat;
mod relocate;
mod resident;
mod rhythm;
mod rive;
mod scan;
mod secrets;
mod server;
mod settings;
mod sheet;
mod sidecar;
mod siglip;
#[cfg(any(feature = "ldap", feature = "https"))]
mod signature_only;
mod signin;
mod sounds;
mod sources;
mod spine;
mod spritesheet;
mod store_formats;
mod store_lookup;
mod store_search;
mod tagger;
mod takes;
mod text_thumbs;
mod textures;
mod thumbs;
mod tools;
mod unitymat;
mod unreal_editor;
mod unreal_recolor;
mod updates;
mod uploads;
mod webglb;

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use tokio::sync::Notify;
use tower_http::compression::CompressionLayer;
use tower_http::compression::predicate::{DefaultPredicate, NotForContentType, Predicate};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

pub use crate::achievements::{all as achievements, on_achievement};
use crate::blobs::BlobStore;
pub use crate::config::Config;
pub use crate::desktop::{launch_url, listening, shut_down};
pub use crate::recent_log::layer as recent_log;

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
    /// Every thumbnail's look, for "find by color" and "more like this".
    pub(crate) looks: looks::Index,
    /// Every thumbnail's SigLIP 2 embedding, for image search.
    pub(crate) pictures: embed::Index,
    /// Every thumbnail's CLIP embedding, which image search uses until SigLIP 2 has relearned them.
    pub(crate) clip: embed::Index,
    /// Every sound's CLAP embedding, for searching sounds by what they sound like, in words.
    pub(crate) clap: embed::Index,
    /// The library's sets of near copies, for the duplicates page.
    pub(crate) near_copies: nearcopies::Cache,
    /// Every rigged model's skeleton, for the animations that fit a character and the characters an animation fits.
    pub(crate) skeletons: animations::Skeletons,
    /// Every sound's acoustic descriptor, for "more like this" on sounds.
    pub(crate) sounds: sounds::Index,
}

pub(crate) type AppState = Arc<Inner>;

/// Which build this is.
pub(crate) fn edition(cfg: &Config) -> &'static str {
    match (cfg.steam_app, cfg.desktop) {
        (Some(_), _) => "desktop app from Steam",
        (None, true) => "desktop app",
        (None, false) => "server",
    }
}

/// Which build this is, where it runs and the SQLite it has.
fn log_build(cfg: &Config) {
    let edition = edition(cfg);
    tracing::info!(
        version = hoard_core::VERSION,
        edition,
        os = std::env::consts::OS,
        arch = std::env::consts::ARCH,
        sqlite = rusqlite::version(),
        "Hoard {}",
        hoard_core::VERSION
    );
}

/// Serves the library `cfg` describes until the process is told to stop (Ctrl+C, or SIGTERM on Unix).
///
/// `--reset-password` and `--https-off` do their one thing and return instead.
pub async fn run(cfg: Config) -> Result<()> {
    // First, so every log (and every bug report pasted from one) says which build it's from.
    log_build(&cfg);
    config::warn_retired_env();
    for dir in [cfg.data_dir.clone(), cfg.staging_dir(), cfg.tmp_dir()] {
        std::fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
    }
    let pool = db::open(&cfg.db_path(), &cfg.backups_dir())?;
    if let Some(name) = &cfg.reset_password {
        let (name, password) = auth::reset_password(&*pool.get()?, name)?;
        #[expect(clippy::print_stdout, reason = "the new password is the command's output")]
        {
            println!("New password for {name}: {password}");
        }
        return Ok(());
    }
    if cfg.https_off {
        https::turn_off(&*pool.get()?)?;
        #[expect(clippy::print_stdout, reason = "the command's output")]
        {
            println!("HTTPS is off. Start Hoard again and set it up in Settings → Connection.");
        }
        return Ok(());
    }
    if cfg.desktop {
        i18n::for_desktop();
        secrets::for_desktop();
        desktop::ensure_owner(&*pool.get()?)?;
    }
    if let Some(loc) = locations::adopt_existing(&*pool.get()?, &cfg)? {
        tracing::info!(path = %loc.path, "your files stay where they are, as the library location \"Library\"");
    }
    store_formats::load(&cfg.data_dir.join("formats"));
    db::keep_in(cfg.kept_dir());
    let settings = settings::Store::load(&cfg, &*pool.get()?)?;
    let effective = settings.get();
    let blobs = BlobStore::open(&cfg, pool.clone(), effective.s3.clone())?;
    let requeued = db::requeue_stale_jobs(&*pool.get()?)?;
    db::settle_packs(&*pool.get()?, None, None)?;
    maintenance::mark_interrupted(&*pool.get()?)?;
    analyze::set_render_enabled(effective.render, effective.llm_vision && effective.llm_enabled());
    analyze::set_thumbs(effective.thumb_quality, effective.thumb_size);
    music::configure(effective.soundfont.clone());
    let http = reqwest::Client::builder().timeout(Duration::from_secs(180)).build()?;

    tracing::info!(
        listen = %cfg.listen,
        data = %cfg.data_dir.display(),
        storage = %blobs.label(),
        llm = effective.llm_label(),
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
        looks: looks::Index::default(),
        pictures: embed::Index::new(&clip::SPEC),
        clip: embed::Index::new(&clip::CLIP_SPEC),
        clap: embed::Index::new(&clap::SPEC),
        sounds: sounds::Index::default(),
        near_copies: nearcopies::Cache::default(),
        skeletons: animations::Skeletons::default(),
    });
    achievements::start(&state).await;
    tokio::spawn(signin::recheck_loop(Arc::clone(&state)));
    tokio::spawn(uploads::sweep_loop(Arc::clone(&state)));
    tokio::spawn(uploads::watch_loop());
    tokio::spawn(api::keep_reports(Arc::clone(&state)));
    ffmpeg::start(&effective.ffmpeg, state.cfg.tools_dir());
    clip::start(&state, effective.clip, effective.clip_hosted.clone());
    clap::start(&state, effective.clap);
    convert::start_blender(&state, false);
    // Starts Unreal Editor's download too, once a GitHub sign-in that ran out while Hoard was off has been renewed.
    tokio::spawn(api::keep_github_fresh(Arc::clone(&state)));
    updates::start(effective.update_check && !state.cfg.desktop);
    updates::start_hook(&state);
    analyze::warm_renderer();
    jobs::spawn_workers(&state);
    motion::kick(&state);
    thumbs::kick(&state);
    let st = Arc::clone(&state);
    tokio::spawn(async move { takes::backfill(&st).await });
    tokio::spawn(unreal_recolor::run(Arc::clone(&state)));
    let st = Arc::clone(&state);
    tokio::spawn(async move { looks::schedule(&st).await });
    {
        let st = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = jobs::read_unread_files(&st).await {
                tracing::warn!(error = %format!("{e:#}"), "could not queue reading packs' store files");
            }
            store_lookup::start(&st).await;
        });
    }
    let st = Arc::clone(&state);
    tokio::spawn(async move { formats::log_trim(&st).await });
    {
        let st = Arc::clone(&state);
        tokio::spawn(async move { nearcopies::warm(&st).await });
    }
    {
        let st = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = text_thumbs::draw_missing(&st).await {
                tracing::warn!(error = %format!("{e:#}"), "could not draw the thumbnails of text added before Hoard drew them");
            }
            if let Err(e) = text_thumbs::sniff_others(&st).await {
                tracing::warn!(error = %format!("{e:#}"), "could not look for text among files of no known kind");
            }
        });
    }
    scan::spawn_rescans(&state);
    reformat::spawn_expiry(&state);
    maintenance::spawn_schedules(&state);
    profiler::start();

    let mut app = api::router(&state);
    if state.cfg.desktop {
        app = app.merge(desktop::router());
    }
    let web = &state.cfg.web_dir;
    if web.join("index.html").is_file() {
        // SPA: unknown paths fall back to index.html so client-side routes work on reload.
        app = app.fallback_service(ServeDir::new(web).fallback(ServeFile::new(web.join("index.html"))));
    } else {
        tracing::warn!(dir = %web.display(), "web portal not built; serving API only (run `npm run build` in web/)");
    }
    if state.cfg.desktop {
        // Everything, the portal's files too: a page elsewhere has no business reading them.
        app = app.layer(middleware::from_fn(desktop::loopback_only));
    }
    let app = app
        .layer(middleware::from_fn(i18n::negotiate))
        .layer(middleware::from_fn(cache_headers))
        .layer(CompressionLayer::new().br(true).zstd(true).gzip(true).compress_when(compressible()))
        .layer(middleware::from_fn(profiler::record))
        // Each request's logs say which request it was (its path only: a query string can carry an OAuth code).
        .layer(TraceLayer::new_for_http().make_span_with(|req: &Request| {
            tracing::info_span!("request", method = %req.method(), path = req.uri().path())
        }))
        .with_state(Arc::clone(&state));

    https::init(&state)?;
    server::serve(&state.cfg, app).await?;
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

/// Cache headers for the portal's own files.
/// Vite puts a content hash in every file name under `/assets/`, so those never change; `index.html` must always be
/// revalidated so a new release is picked up.
async fn cache_headers(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let mut resp = next.run(req).await;
    let headers = resp.headers_mut();
    // Nothing is guessed at from its bytes, and no other site can show the portal in a frame to trick clicks on it.
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("SAMEORIGIN"));
    if !path.starts_with("/api/") && !path.starts_with("/mcp") && !headers.contains_key(header::CACHE_CONTROL) {
        let value = if path.starts_with("/assets/") { "public, max-age=31536000, immutable" } else { "no-cache" };
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(value));
    }
    resp
}
