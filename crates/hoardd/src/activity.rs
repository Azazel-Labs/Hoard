//! What the server is doing in the background, for the portal's activity indicator: running jobs (ingest, tagging,
//! Blender and Unreal conversion, scrub), "download as" conversions, zips being streamed, and the ffmpeg download.
//! Work registers itself and gets a handle; dropping the handle removes the entry. Queued jobs come from the job
//! table (see `api::activity`).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex, PoisonError};

use serde::Serialize;

/// One thing in progress.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Entry {
    pub(crate) id: u64,
    /// `ingest`, `retag`, `llm_tag`, `scrub`, `convert` (Blender), `unreal_convert`, `download_as`, `zip`, `ffmpeg`.
    pub(crate) kind: &'static str,
    /// What it is, e.g. `Ingesting Forest Pack` or `Converting theme.mid to MP3`.
    pub(crate) label: String,
    /// More detail, e.g. `120 of 300 files`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) detail: Option<String>,
    /// 0..=1, when the work can tell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) progress: Option<f32>,
    /// `running` or `queued`.
    pub(crate) state: &'static str,
    /// When it started (RFC 3339).
    pub(crate) since: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pack_id: Option<String>,
}

static NEXT: AtomicU64 = AtomicU64::new(1);
static RUNNING: LazyLock<Mutex<BTreeMap<u64, Entry>>> = LazyLock::new(Default::default);

fn with<R>(f: impl FnOnce(&mut BTreeMap<u64, Entry>) -> R) -> R {
    f(&mut RUNNING.lock().unwrap_or_else(PoisonError::into_inner))
}

/// Registers work until dropped.
#[derive(Debug)]
pub(crate) struct Handle(u64);

/// Start showing a piece of work.
pub(crate) fn start(kind: &'static str, label: impl Into<String>, pack_id: Option<&str>) -> Handle {
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let entry = Entry {
        id,
        kind,
        label: label.into(),
        detail: None,
        progress: None,
        state: "running",
        since: crate::db::now(),
        pack_id: pack_id.map(str::to_owned),
    };
    with(|m| m.insert(id, entry));
    Handle(id)
}

impl Handle {
    pub(crate) fn progress(&self, p: f32) {
        with(|m| {
            if let Some(e) = m.get_mut(&self.0) {
                e.progress = Some(p.clamp(0.0, 1.0));
            }
        });
    }

    pub(crate) fn detail(&self, d: impl Into<String>) {
        let d = d.into();
        with(|m| {
            if let Some(e) = m.get_mut(&self.0) {
                e.detail = Some(d);
            }
        });
    }

    /// Waiting for a free slot (shown as queued) or running.
    pub(crate) fn queued(&self, queued: bool) {
        with(|m| {
            if let Some(e) = m.get_mut(&self.0) {
                e.state = if queued { "queued" } else { "running" };
            }
        });
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        with(|m| m.remove(&self.0));
    }
}

/// Report progress on a running job from inside it, found by kind and pack.
pub(crate) fn job_progress(kind: &str, pack_id: Option<&str>, done: usize, total: usize, unit: &str) {
    with(|m| {
        if let Some(e) = m.values_mut().find(|e| e.kind == kind && e.pack_id.as_deref() == pack_id) {
            e.progress = (total > 0).then(|| done as f32 / total as f32);
            e.detail = Some(format!("{done} of {total} {unit}"));
        }
    });
}

/// A job's progress as words alone, for work whose total isn't known yet ("unpacked 2,000 files").
pub(crate) fn job_note(kind: &str, pack_id: Option<&str>, detail: &str) {
    with(|m| {
        if let Some(e) = m.values_mut().find(|e| e.kind == kind && e.pack_id.as_deref() == pack_id) {
            e.progress = None;
            e.detail = Some(detail.to_owned());
        }
    });
}

/// Everything registered now, oldest first.
pub(crate) fn snapshot() -> Vec<Entry> {
    with(|m| m.values().cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_live_as_long_as_their_handle() {
        let h = start("zip", "Zipping Test Pack", Some("pack-x"));
        h.progress(0.5);
        job_progress("zip", Some("pack-x"), 3, 4, "files");
        let e = snapshot().into_iter().find(|e| e.id == h.0).unwrap();
        assert_eq!(e.progress, Some(0.75));
        assert_eq!(e.detail.as_deref(), Some("3 of 4 files"));
        drop(h);
        assert!(!snapshot().iter().any(|e| e.label == "Zipping Test Pack"));
    }
}
