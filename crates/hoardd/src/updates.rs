//! Tells the portal when a newer Hoard release is out. Every six hours the server asks GitHub for the latest release
//! (pre-releases and drafts excluded); if it's newer than this build, `GET /api/stats` carries it as `update` and
//! the portal shows a link to its notes. Nothing is installed: updating is pulling the new image (or binary), and
//! the database migrates itself on the next start. Settings turns the check off.
//!
//! Hoard can also hand the update to whatever replaces its container: Settings → Maintenance → Updates takes an
//! address to call (Watchtower's HTTP API, a Portainer webhook, a script behind a webhook listener). Hoard calls it
//! once a newer release is out and nothing is going on, so a restart doesn't cut an upload or a conversion short.
//! Scripts that would rather ask can poll `GET /api/admin/update`, which says whether an update is out and what's
//! keeping Hoard busy.

use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use hoard_core::UpdateInfo;

use crate::i18n::tr;

const LATEST_RELEASE: &str = "https://api.github.com/repos/Azazel-Labs/Hoard/releases/latest";
const FIRST_CHECK: Duration = Duration::from_secs(60);
const INTERVAL: Duration = Duration::from_hours(6);
/// How often to see whether the server is idle while an update waits.
const IDLE_POLL: Duration = Duration::from_secs(60);
/// Between calls for the same release after one fails.
const RETRY: Duration = Duration::from_hours(1);
const MAX_CALLS: u32 = 6;

static AVAILABLE: RwLock<Option<UpdateInfo>> = RwLock::new(None);
static ENABLED: AtomicBool = AtomicBool::new(false);
static CHANGED: tokio::sync::Notify = tokio::sync::Notify::const_new();
static LAST_CALL: RwLock<Option<HookCall>> = RwLock::new(None);

/// The newer release, if the last check found one.
pub(crate) fn available() -> Option<UpdateInfo> {
    AVAILABLE.read().ok()?.clone()
}

/// Turn the check on or off. Turning it on checks within a minute; off forgets what was found.
pub(crate) fn set_enabled(enabled: bool) {
    if ENABLED.swap(enabled, Ordering::Relaxed) != enabled {
        if !enabled && let Ok(mut slot) = AVAILABLE.write() {
            *slot = None;
        }
        CHANGED.notify_one();
    }
}

pub(crate) fn start(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
    tokio::spawn(async {
        loop {
            // Off: wait to be turned on. On: a minute's grace (so a restart loop doesn't hammer GitHub), then check.
            while !ENABLED.load(Ordering::Relaxed) {
                CHANGED.notified().await;
            }
            tokio::select! {
                () = tokio::time::sleep(FIRST_CHECK) => {}
                () = CHANGED.notified() => continue,
            }
            match latest().await {
                Ok(latest) => {
                    let newer = latest.filter(|u| is_newer(&u.version, hoard_core::VERSION));
                    if let Some(u) = &newer {
                        tracing::info!(version = %u.version, url = %u.url, "a newer Hoard release is available");
                    }
                    if let Ok(mut slot) = AVAILABLE.write() {
                        *slot = newer;
                    }
                }
                // Offline, rate-limited, or no release yet: try again tomorrow, quietly.
                Err(e) => tracing::debug!(error = %format!("{e:#}"), "update check failed"),
            }
            tokio::select! {
                () = tokio::time::sleep(INTERVAL) => {}
                () = CHANGED.notified() => {}
            }
        }
    });
}

async fn latest() -> Result<Option<UpdateInfo>> {
    let http =
        reqwest::Client::builder().timeout(Duration::from_secs(30)).user_agent(hoard_core::user_agent()).build()?;
    let resp = http.get(LATEST_RELEASE).header("Accept", "application/vnd.github+json").send().await?;
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let body: serde_json::Value = resp.error_for_status()?.json().await.context("parse the latest release")?;
    let tag = body["tag_name"].as_str().context("release has no tag_name")?;
    let url = body["html_url"].as_str().unwrap_or_default();
    Ok(Some(UpdateInfo { version: tag.trim_start_matches('v').to_owned(), url: url.to_owned() }))
}

/// What keeps the server from being idle. Jobs waiting for a later time (a store lookup's pause, a retry's back-off)
/// don't count: they'd keep a big library busy for good.
#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct Busy {
    /// What's running: jobs, maintenance, conversions, downloads, zips (the activity list's labels).
    pub(crate) work: Vec<String>,
    /// Jobs due now and waiting for a worker.
    pub(crate) queued: i64,
    pub(crate) uploads: usize,
}

impl Busy {
    pub(crate) fn idle(&self) -> bool {
        self.work.is_empty() && self.queued == 0 && self.uploads == 0
    }
}

pub(crate) async fn busy(state: &crate::AppState) -> Result<Busy> {
    let work = crate::activity::snapshot().into_iter().map(|e| e.label).collect();
    let uploads = state.uploads_in_flight.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len();
    let queued = crate::db::call(&state.pool, |c| due_jobs(c, &crate::db::now())).await?;
    Ok(Busy { work, queued, uploads })
}

/// Jobs queued to run by `now` (RFC 3339, as `run_after` is stored).
fn due_jobs(c: &rusqlite::Connection, now: &str) -> Result<i64> {
    Ok(c.query_row("SELECT COUNT(*) FROM jobs WHERE status = 'queued' AND run_after <= ?1", [now], |r| r.get(0))?)
}

/// The last call to the update address.
#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct HookCall {
    /// The release it was for.
    pub(crate) version: String,
    pub(crate) at: String,
    pub(crate) ok: bool,
    /// What came back: `202 Accepted`, or why nothing did.
    pub(crate) result: String,
}

pub(crate) fn last_call() -> Option<HookCall> {
    LAST_CALL.read().ok()?.clone()
}

/// Call the update address once a newer release is out and the server has been idle twice in a row, a minute
/// apart. A call that works isn't repeated for that release (if it updated the container, this build is gone; if
/// not, the tag is probably pinned). One that fails is tried again an hour later, up to [`MAX_CALLS`] times. A new
/// address starts over.
pub(crate) fn start_hook(state: &crate::AppState) {
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        let (mut key, mut calls, mut idle_before) = (String::new(), 0u32, false);
        let mut next = tokio::time::Instant::now();
        loop {
            tokio::time::sleep(IDLE_POLL).await;
            let (Some(url), Some(update)) = (state.settings.get().update_hook_url.clone(), available()) else {
                idle_before = false;
                continue;
            };
            let this = format!("{}\n{url}", update.version);
            if this != key {
                (key, calls, next) = (this, 0, tokio::time::Instant::now());
            }
            if calls >= MAX_CALLS || tokio::time::Instant::now() < next {
                continue;
            }
            let idle = busy(&state).await.is_ok_and(|b| b.idle());
            let ready = idle && idle_before;
            idle_before = idle && !ready;
            if !ready {
                continue;
            }
            calls += 1;
            if call_hook(&state, &update.version).await.ok {
                calls = MAX_CALLS;
            } else {
                next = tokio::time::Instant::now() + RETRY;
            }
        }
    });
}

/// POST to the update address now, with the release in the body for anything that wants it.
pub(crate) async fn call_hook(state: &crate::AppState, version: &str) -> HookCall {
    let settings = state.settings.get();
    let result = match settings.update_hook_url.as_deref() {
        None => Err(tr!("server-update-no-address")),
        Some(url) => {
            tracing::info!(version, url, "asking for an update");
            let mut req = state.http.post(url).timeout(Duration::from_secs(30)).json(&serde_json::json!({
                "event": "update",
                "version": version,
                "running": hoard_core::VERSION,
                "notes": available().map(|u| u.url),
            }));
            if let Some(auth) = settings.update_hook_auth.as_deref() {
                req = req.header(reqwest::header::AUTHORIZATION, auth);
            }
            match req.send().await {
                Ok(resp) if resp.status().is_success() => Ok(resp.status().to_string()),
                Ok(resp) => Err(resp.status().to_string()),
                Err(e) if e.is_timeout() => Err(tr!("server-update-no-answer")),
                Err(e) => Err(format!("{e}")),
            }
        }
    };
    if let Err(e) = &result {
        tracing::warn!(error = %e, "the update address didn't take the request");
    }
    let (ok, result) = match result {
        Ok(r) => (true, r),
        Err(r) => (false, r),
    };
    let call = HookCall { version: version.to_owned(), at: crate::db::now(), ok, result };
    if let Ok(mut slot) = LAST_CALL.write() {
        *slot = Some(call.clone());
    }
    call
}

/// `GET /api/admin/update`: this build, the newer release if there is one, what's keeping the server busy, and the
/// last call to the update address. For the Settings page, and for scripts that update Hoard themselves: poll
/// until `update` is set and `idle` is true, then pull and restart.
pub(crate) async fn status_handler(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
) -> crate::error::ApiResult<axum::Json<serde_json::Value>> {
    let busy = busy(&state).await?;
    Ok(axum::Json(serde_json::json!({
        "version": hoard_core::VERSION,
        "update": available(),
        "idle": busy.idle(),
        "busy": busy,
        "hook": { "set": state.settings.get().update_hook_url.is_some(), "last": last_call() },
    })))
}

/// `POST /api/admin/update/hook`: call the update address now, for the release that's out (or this one), to see
/// that it's wired up. Whatever it's wired to runs.
pub(crate) async fn call_handler(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
) -> crate::error::ApiResult<axum::Json<HookCall>> {
    if state.settings.get().update_hook_url.is_none() {
        return Err(crate::error::AppError::bad_request(tr!("error-update-hook-not-set")));
    }
    let version = available().map_or_else(|| hoard_core::VERSION.to_owned(), |u| u.version);
    Ok(axum::Json(call_hook(&state, &version).await))
}

/// Whether `candidate` is a later semver than `current`. A release beats its own pre-releases (0.9.0 > 0.9.0-rc.1);
/// anything unparseable is never newer.
fn is_newer(candidate: &str, current: &str) -> bool {
    fn parse(v: &str) -> Option<((u64, u64, u64), bool)> {
        let (core, pre) = match v.split_once('-') {
            Some((core, _)) => (core, true),
            None => (v.split('+').next()?, false),
        };
        let mut it = core.split('.').map(str::parse::<u64>);
        let parts = (it.next()?.ok()?, it.next()?.ok()?, it.next()?.ok()?);
        it.next().is_none().then_some((parts, pre))
    }
    match (parse(candidate), parse(current)) {
        (Some((a, a_pre)), Some((b, b_pre))) => a > b || (a == b && b_pre && !a_pre),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{due_jobs, is_newer};

    #[test]
    fn jobs_waiting_for_later_dont_count() {
        let c = rusqlite::Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE jobs (id INTEGER PRIMARY KEY, status TEXT, run_after TEXT);
             INSERT INTO jobs (status, run_after) VALUES
               ('queued', '2026-01-01T00:10:00Z'), ('done', '2026-01-01T00:00:00Z'), ('running', '2026-01-01T00:00:00Z');",
        )
        .unwrap();
        let now = "2026-01-01T00:05:00Z";
        assert_eq!(due_jobs(&c, now).unwrap(), 0, "a store lookup paused until later isn't work");
        c.execute("INSERT INTO jobs (status, run_after) VALUES ('queued', '2026-01-01T00:04:00Z')", []).unwrap();
        assert_eq!(due_jobs(&c, now).unwrap(), 1);
    }

    #[test]
    fn compares_semver() {
        assert!(is_newer("0.9.0", "0.8.0"));
        assert!(is_newer("0.8.10", "0.8.9"));
        assert!(is_newer("1.0.0", "0.99.99"));
        assert!(is_newer("0.9.0", "0.9.0-rc.1"));
        assert!(!is_newer("0.8.0", "0.8.0"));
        assert!(!is_newer("0.8.0", "0.9.0"));
        assert!(!is_newer("0.9.0-rc.1", "0.9.0"));
        assert!(!is_newer("garbage", "0.8.0"));
        assert!(!is_newer("1.2", "0.8.0"));
    }
}
