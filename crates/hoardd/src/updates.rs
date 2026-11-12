//! Tells the portal when a newer Hoard release is out. Every six hours the server asks GitHub for the latest release
//! (pre-releases and drafts excluded); if it's newer than this build, `GET /api/stats` carries it as `update` and
//! the portal shows a link to its notes. Nothing is installed: updating is pulling the new image (or binary), and
//! the database migrates itself on the next start. Settings turns the check off.
//!
//! With Watchtower's HTTP API wired up (`HOARD_WATCHTOWER_URL`), Hoard also asks Watchtower to update it as soon as
//! it's idle, rather than waiting for Watchtower's nightly look.

use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use hoard_core::UpdateInfo;

const LATEST_RELEASE: &str = "https://api.github.com/repos/Azazel-Labs/Hoard/releases/latest";
const FIRST_CHECK: Duration = Duration::from_secs(60);
const INTERVAL: Duration = Duration::from_hours(6);
/// How often to see whether the server is idle while an update waits.
const IDLE_POLL: Duration = Duration::from_secs(60);
/// Between asks for the same release: its image is pushed some minutes after the release is published, so the
/// first ask can find nothing new.
const RETRY: Duration = Duration::from_hours(1);
const MAX_ASKS: u32 = 6;

static AVAILABLE: RwLock<Option<UpdateInfo>> = RwLock::new(None);
static ENABLED: AtomicBool = AtomicBool::new(false);
static CHANGED: tokio::sync::Notify = tokio::sync::Notify::const_new();

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

/// Ask Watchtower to update this container once a newer release is out and nothing is going on: no jobs, uploads,
/// maintenance or other work, twice in a row a minute apart. Watchtower stops this container, so nothing here runs
/// after a successful ask; if the image isn't there yet, it tries again an hour later, up to [`MAX_ASKS`] times.
pub(crate) fn start_auto_update(state: &crate::AppState) {
    let Some(url) = state.cfg.watchtower_url.clone() else { return };
    let token = state.cfg.watchtower_token.clone().unwrap_or_default();
    let container = state.cfg.watchtower_container.clone();
    let state = std::sync::Arc::clone(state);
    tokio::spawn(async move {
        let http = match reqwest::Client::builder().timeout(Duration::from_secs(30)).build() {
            Ok(h) => h,
            Err(e) => return tracing::warn!(error = %e, "automatic updates off"),
        };
        let (mut asked_for, mut asks, mut idle_before) = (String::new(), 0u32, false);
        loop {
            tokio::time::sleep(IDLE_POLL).await;
            let Some(update) = available() else { continue };
            if update.version != asked_for {
                (asked_for, asks) = (update.version.clone(), 0);
            }
            if asks >= MAX_ASKS {
                continue;
            }
            let idle = is_idle(&state).await;
            if !(idle && std::mem::replace(&mut idle_before, idle)) {
                continue;
            }
            asks += 1;
            idle_before = false;
            let endpoint = format!("{}/v1/update", url.trim_end_matches('/'));
            tracing::info!(version = %update.version, container = %container, "idle: asking Watchtower to update Hoard");
            match http
                .post(&endpoint)
                .bearer_auth(&token)
                .query(&[("container", format!("^{container}$").as_str()), ("async", "true")])
                .send()
                .await
                .and_then(reqwest::Response::error_for_status)
            {
                Ok(_) => {}
                Err(e) => tracing::warn!(error = %e, endpoint, "Watchtower didn't take the update request"),
            }
            tokio::time::sleep(RETRY).await;
        }
    });
}

/// Nothing running or waiting: no activity (jobs, maintenance, conversions, downloads), no queued jobs, no uploads.
async fn is_idle(state: &crate::AppState) -> bool {
    if !crate::activity::snapshot().is_empty()
        || !state.uploads_in_flight.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty()
    {
        return false;
    }
    crate::db::call(&state.pool, |c| {
        Ok(c.query_row("SELECT NOT EXISTS (SELECT 1 FROM jobs WHERE status IN ('queued', 'running'))", [], |r| {
            r.get::<_, bool>(0)
        })?)
    })
    .await
    .unwrap_or(false)
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
    use super::is_newer;

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
