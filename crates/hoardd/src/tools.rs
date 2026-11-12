//! Outside programs Hoard can use (ffmpeg, Blender): where each one stands, and fetching them, as a streamed
//! download with a size cap, its SHA-256 computed on the way through, progress on the activity indicator, and a way
//! to stop when the tool is switched off partway through.

use std::path::Path;
use std::sync::{PoisonError, RwLock};
use std::time::Duration;

use anyhow::{Result, bail, ensure};
use futures_util::StreamExt;
use sha2::Digest;

/// Where a tool stands.
#[derive(Clone, Debug)]
pub(crate) enum Status<T> {
    Off,
    Looking,
    Downloading,
    Ready(T),
    Unavailable(String),
}

/// A tool's status, replaced as it's found, fetched or switched off. Every [`Slot::restart`] begins a new
/// generation, and work from an older one (a download still running after the setting changed) can't overwrite it.
#[derive(Debug)]
pub(crate) struct Slot<T> {
    inner: RwLock<(u64, Status<T>)>,
}

impl<T: Clone> Slot<T> {
    pub(crate) const fn new() -> Self {
        Self { inner: RwLock::new((0, Status::Looking)) }
    }

    /// Start over with `status`; returns the new generation.
    pub(crate) fn restart(&self, status: Status<T>) -> u64 {
        let mut g = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        g.0 += 1;
        g.1 = status;
        g.0
    }

    /// Whether `generation` is still the current one.
    pub(crate) fn current(&self, generation: u64) -> bool {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).0 == generation
    }

    /// Set the status, unless a newer generation has started. Returns whether it was set.
    pub(crate) fn set(&self, generation: u64, status: Status<T>) -> bool {
        let mut g = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if g.0 != generation {
            return false;
        }
        g.1 = status;
        true
    }

    pub(crate) fn ready(&self) -> Option<T> {
        match &self.inner.read().unwrap_or_else(PoisonError::into_inner).1 {
            Status::Ready(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// One line for the status page and the settings panel: `off`, `looking`, `downloading`, what `ready` says
    /// about the tool (its version), or `unavailable: <why>`.
    pub(crate) fn describe(&self, ready: impl Fn(&T) -> String) -> String {
        match &self.inner.read().unwrap_or_else(PoisonError::into_inner).1 {
            Status::Off => "off".into(),
            Status::Looking => "looking".into(),
            Status::Downloading => "downloading".into(),
            Status::Ready(t) => ready(t),
            Status::Unavailable(why) => format!("unavailable: {why}"),
        }
    }
}

/// Whether a tool setting means "never run it".
pub(crate) fn is_off(setting: &str) -> bool {
    matches!(setting.trim().to_ascii_lowercase().as_str(), "off" | "false" | "0" | "none" | "no")
}

/// Whether a tool setting means "find one, or download it".
pub(crate) fn is_auto(setting: &str) -> bool {
    let s = setting.trim();
    s.is_empty() || s.eq_ignore_ascii_case("auto")
}

/// An HTTP client for tool downloads: patient with slow mirrors, and honest about who's asking.
pub(crate) fn client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(30))
        .read_timeout(Duration::from_secs(120))
        .user_agent(concat!("hoard/", env!("CARGO_PKG_VERSION")))
        .build()?)
}

/// A finished download.
pub(crate) struct Fetched {
    /// Lowercase hex SHA-256 of what was written.
    pub(crate) sha256: String,
    /// Where it came from after redirects.
    pub(crate) final_url: reqwest::Url,
}

/// Download `url` to `dest`, refusing anything over `max` bytes and stopping as soon as `cancelled` says so. Leaves
/// no file behind on failure.
pub(crate) async fn fetch(
    http: &reqwest::Client,
    url: &str,
    dest: &Path,
    max: u64,
    activity: &crate::activity::Handle,
    cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<Fetched> {
    let result = fetch_inner(http, url, dest, max, activity, cancelled).await;
    if result.is_err() {
        tokio::fs::remove_file(dest).await.ok();
    }
    result
}

async fn fetch_inner(
    http: &reqwest::Client,
    url: &str,
    dest: &Path,
    max: u64,
    activity: &crate::activity::Handle,
    cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<Fetched> {
    let resp = http.get(url).send().await?.error_for_status()?;
    let final_url = resp.url().clone();
    let expected_len = resp.content_length();
    ensure!(expected_len.is_none_or(|n| n <= max), "the download is unexpectedly large");
    let mut file = tokio::fs::File::create(dest).await?;
    let mut hasher = sha2::Sha256::new();
    let mut size = 0u64;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        if cancelled() {
            bail!("cancelled");
        }
        let chunk = chunk?;
        size += chunk.len() as u64;
        ensure!(size <= max, "the download is unexpectedly large");
        hasher.update(&chunk);
        if let Some(len) = expected_len.filter(|&l| l > 0) {
            activity.progress(size as f32 / len as f32);
        }
        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
    }
    tokio::io::AsyncWriteExt::flush(&mut file).await?;
    Ok(Fetched { sha256: hex(&hasher.finalize()), final_url })
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
        write!(s, "{b:02x}").ok();
        s
    })
}

/// The hash for `name` in a `sha256sum`-style list (`<hash>  <name>` per line, `*` marking binary mode).
pub(crate) fn sum_for(list: &str, name: &str) -> Option<String> {
    list.lines()
        .find(|l| l.split_whitespace().nth(1).is_some_and(|n| n.trim_start_matches('*') == name))
        .and_then(|l| l.split_whitespace().next())
        .map(str::to_ascii_lowercase)
        .filter(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_hexdigit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_checksum_lists() {
        let a = "a".repeat(64);
        let b = "B".repeat(64);
        let list = format!("{a}  one.zip\n{b} *two.tar.xz\nnot a line\n");
        assert_eq!(sum_for(&list, "one.zip").as_deref(), Some(a.as_str()));
        assert_eq!(sum_for(&list, "two.tar.xz"), Some("b".repeat(64)));
        assert_eq!(sum_for(&list, "three.zip"), None);
        assert_eq!(sum_for("abc  short.zip", "short.zip"), None, "not a SHA-256");
    }
}
