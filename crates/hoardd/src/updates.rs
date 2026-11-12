//! Tells the portal when a newer Hoard release is out. Once a day the server asks GitHub for the latest release
//! (pre-releases and drafts excluded); if it's newer than this build, `GET /api/stats` carries it as `update` and
//! the portal shows a link to its notes. Nothing is installed: updating is pulling the new image (or binary), and
//! the database migrates itself on the next start. Settings (or `HOARD_UPDATE_CHECK=false`) turns the check off.

use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use hoard_core::UpdateInfo;

const LATEST_RELEASE: &str = "https://api.github.com/repos/Azazel-Labs/Hoard/releases/latest";
const FIRST_CHECK: Duration = Duration::from_secs(60);
const INTERVAL: Duration = Duration::from_hours(24);

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
