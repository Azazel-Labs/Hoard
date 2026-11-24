//! What each request sending an upload's bytes is doing, so the log tells of one that's slow or has stopped: which
//! part of its work it's in (the database, receiving, saving), for how long, and how it ended. A request that never
//! ends logs nothing of its own, so [`watch_loop`] looks over the ones under way and tells of any stuck in one part of
//! its work.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use axum::http::{HeaderMap, Version};

/// A request still in one part of its work after this long is told of in the log, and again each time as long again
/// passes.
const STUCK: Duration = Duration::from_secs(45);
/// A request that took this long in all is told of when it ends.
const SLOW: Duration = Duration::from_secs(20);
/// As is one that went this long without a byte while receiving.
const GAP: Duration = Duration::from_secs(10);

/// The part of its work a request is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Phase {
    /// Looking up or recording what's arrived.
    Database,
    /// Waiting for the bytes the browser sends.
    Receiving,
    /// Writing them to the staging folder, and making sure they're on disk.
    Saving,
}

impl Phase {
    const ALL: [Phase; 3] = [Phase::Database, Phase::Receiving, Phase::Saving];

    fn name(self) -> &'static str {
        match self {
            Phase::Database => "database",
            Phase::Receiving => "receiving",
            Phase::Saving => "saving",
        }
    }
}

struct Doing {
    phase: Phase,
    since: Instant,
    /// Time spent in each [`Phase`] before the current one, in [`Phase::ALL`]'s order.
    spent: [Duration; 3],
    bytes: u64,
    last_byte: Instant,
    longest_gap: Duration,
    /// When the log last told of this request being stuck.
    told: Option<Instant>,
    finished: bool,
}

impl Doing {
    /// The time spent in the current phase, added to its total.
    fn close(&mut self, now: Instant) {
        let i = self.phase as usize;
        self.spent[i] += now - self.since;
        self.since = now;
    }
}

/// One request being watched, from when it began until it's dropped (finished, failed, or the browser went away).
struct Watch {
    key: u64,
    started: Instant,
    /// Which upload, file and part, for the log.
    what: String,
    version: Version,
    /// The request's span, so what [`watch_loop`] logs of it carries its context.
    span: tracing::Span,
    doing: Mutex<Doing>,
}

impl Watch {
    fn doing(&self) -> MutexGuard<'_, Doing> {
        self.doing.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

static WATCHING: LazyLock<Mutex<HashMap<u64, Arc<Watch>>>> = LazyLock::new(Mutex::default);
static NEXT: AtomicU64 = AtomicU64::new(0);

fn watching() -> MutexGuard<'static, HashMap<u64, Arc<Watch>>> {
    WATCHING.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A request under watch; dropping it ends the watch, and logs how the request went if it went badly.
pub(super) struct Watched(Arc<Watch>);

impl Watched {
    /// Start watching a request. One the browser sends again after a failure says why it gave up on the last
    /// (`X-Hoard-Retry`), and the log tells of that too: it's the browser's side of a request that went wrong.
    pub(super) fn start(what: String, version: Version, headers: &HeaderMap) -> Self {
        if let Some(why) = headers.get("x-hoard-retry").and_then(|v| v.to_str().ok()) {
            tracing::warn!(%what, ?version, browser_said = %percent_decode(why), "an upload request sent again");
        }
        let now = Instant::now();
        let watch = Arc::new(Watch {
            key: NEXT.fetch_add(1, Ordering::Relaxed),
            started: now,
            what,
            version,
            span: tracing::Span::current(),
            doing: Mutex::new(Doing {
                phase: Phase::Database,
                since: now,
                spent: [Duration::ZERO; 3],
                bytes: 0,
                last_byte: now,
                longest_gap: Duration::ZERO,
                told: None,
                finished: false,
            }),
        });
        watching().insert(watch.key, Arc::clone(&watch));
        Self(watch)
    }

    pub(super) fn phase(&self, phase: Phase) {
        let mut d = self.0.doing();
        if d.phase != phase {
            let now = Instant::now();
            d.close(now);
            d.phase = phase;
            d.told = None;
            if phase == Phase::Receiving && d.bytes == 0 {
                // The wait for the first byte counts from here, not from when the request began.
                d.last_byte = now;
            }
        }
    }

    /// Bytes arrived.
    pub(super) fn got(&self, n: usize) {
        let mut d = self.0.doing();
        let now = Instant::now();
        d.longest_gap = d.longest_gap.max(now - d.last_byte);
        d.last_byte = now;
        d.bytes += n as u64;
    }

    /// It did all it was asked.
    pub(super) fn finished(&self) {
        self.0.doing().finished = true;
    }
}

impl Drop for Watched {
    fn drop(&mut self) {
        watching().remove(&self.0.key);
        let w = &self.0;
        let mut d = w.doing();
        let now = Instant::now();
        d.close(now);
        let (what, version, bytes) = (&w.what, w.version, d.bytes);
        let secs = (now - w.started).as_secs_f32();
        let (spent, gap) = (spent(&d.spent), d.longest_gap.as_secs_f32());
        if !d.finished {
            // It failed, or the browser went away (gave up waiting, or the tab was closed).
            let at = d.phase.name();
            tracing::warn!(parent: &w.span, %what, ?version, bytes, secs, at, %spent, longest_gap_secs = gap,
                "an upload request ended unfinished");
        } else if now - w.started >= SLOW || d.longest_gap >= GAP {
            tracing::warn!(parent: &w.span, %what, ?version, bytes, secs, %spent, longest_gap_secs = gap,
                "a slow upload request");
        } else {
            tracing::debug!(parent: &w.span, %what, ?version, bytes, secs, %spent, "an upload request");
        }
    }
}

/// Time spent in each phase, for the log: `database 0.1s, receiving 3.2s, saving 0.4s`.
fn spent(spent: &[Duration; 3]) -> String {
    Phase::ALL
        .iter()
        .zip(spent)
        .map(|(p, d)| format!("{} {:.1}s", p.name(), d.as_secs_f32()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// `encodeURIComponent` undone.
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let hex = |i: usize| bytes.get(i).and_then(|&b| (b as char).to_digit(16));
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let (Some(hi), Some(lo)) = (hex(i + 1), hex(i + 2))
        {
            out.push(u8::try_from(hi * 16 + lo).unwrap_or(b'?'));
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Tell the log of upload requests stuck in one part of their work, for as long as the server runs.
pub(crate) async fn watch_loop() {
    let mut every = tokio::time::interval(Duration::from_secs(5));
    loop {
        every.tick().await;
        look_over(Instant::now());
    }
}

fn look_over(now: Instant) {
    let under_way: Vec<Arc<Watch>> = watching().values().cloned().collect();
    for w in under_way {
        let mut d = w.doing();
        // Receiving is stuck once the bytes stop coming, however long it's been receiving.
        let still = now.saturating_duration_since(if d.phase == Phase::Receiving { d.last_byte } else { d.since });
        if still < STUCK || d.told.is_some_and(|at| now.saturating_duration_since(at) < STUCK) {
            continue;
        }
        d.told = Some(now);
        let secs = now.saturating_duration_since(w.started).as_secs_f32();
        tracing::warn!(parent: &w.span, what = %w.what, version = ?w.version, at = d.phase.name(),
            still_secs = still.as_secs_f32(), bytes = d.bytes, secs, "an upload request is stuck");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_reason_comes_back_as_the_browser_wrote_it() {
        assert_eq!(percent_decode("2%2F8%3A%20nothing%20for%20120%20s"), "2/8: nothing for 120 s");
        assert_eq!(percent_decode("caf%C3%A9 %zz %"), "café %zz %");
    }

    #[test]
    fn a_request_stuck_in_one_phase_is_told_of_now_and_then() {
        let w = Watched::start("t".into(), Version::HTTP_3, &HeaderMap::new());
        w.phase(Phase::Saving);
        let later = Instant::now() + STUCK + Duration::from_secs(1);
        look_over(later);
        let told = w.0.doing().told;
        assert!(told.is_some());
        look_over(later + Duration::from_secs(1));
        assert_eq!(w.0.doing().told, told, "not told again so soon");
        w.phase(Phase::Database);
        assert!(w.0.doing().told.is_none(), "a new phase is watched afresh");
        w.finished();
        drop(w);
        assert!(watching().values().all(|v| v.what != "t"), "no longer watched once it's over");
    }

    #[test]
    fn time_is_counted_by_phase() {
        let mut d = Doing {
            phase: Phase::Receiving,
            since: Instant::now(),
            spent: [Duration::ZERO; 3],
            bytes: 0,
            last_byte: Instant::now(),
            longest_gap: Duration::ZERO,
            told: None,
            finished: false,
        };
        d.close(d.since + Duration::from_millis(1500));
        assert_eq!(spent(&d.spent), "database 0.0s, receiving 1.5s, saving 0.0s");
    }
}
