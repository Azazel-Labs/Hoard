//! A light profiler for Settings → Performance: how busy the machine's processors are each second and how much of
//! that is Hoard, what Hoard is working on, and where the time of each request it serves goes: waiting on the
//! database, reading the disk, sending to the browser, or everything else (working the answer out). Kept in memory
//! for the last ten minutes; nothing is written anywhere.
//!
//! A request's database time is its time in `db::call` (waiting for a connection and running the queries), and its
//! disk time is opening the stored file plus producing the response body's chunks (reading the file, compressing
//! text). Sending time is the gaps after a chunk is handed over before the next is asked for, which is the network
//! (or a slow browser) taking it. The parts can add up to more than the whole when a request does two at once.

use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, PoisonError};
use std::task::{Context, Poll};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::body::{Body, Bytes, HttpBody};
use axum::extract::{MatchedPath, Query, Request};
use axum::middleware::Next;
use axum::response::Response;
use http_body::{Frame, SizeHint};
use serde::{Deserialize, Serialize};

/// How far back it remembers.
const KEEP: Duration = Duration::from_secs(600);
/// At most this many requests, however busy it gets.
const MAX_REQUESTS: usize = 20_000;
/// The profiler's own requests aren't recorded: asking every couple of seconds would fill the table with them.
const OWN_ROUTE: &str = "/api/admin/profile";

/// One second of the machine.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Sample {
    seq: u64,
    /// Milliseconds since the Unix epoch.
    at: u64,
    /// Percent of every core together: Hoard's process, and the whole machine (Hoard included).
    hoard: f32,
    machine: f32,
    /// Hoard's memory, in bytes.
    memory: u64,
    /// Requests being served then.
    serving: u32,
    /// What was running then (jobs, conversions, zips).
    running: Vec<Running>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Running {
    kind: &'static str,
    label: String,
}

/// One request served.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Served {
    seq: u64,
    /// When it came in, in milliseconds since the Unix epoch.
    at: u64,
    method: String,
    /// The route it matched (`/api/assets/{id}/thumb`), or empty for the portal's own files.
    route: String,
    status: u16,
    /// Milliseconds: from coming in to the last byte handed over, then its parts.
    total: f64,
    db: f64,
    disk: f64,
    send: f64,
    /// Bytes of body sent.
    bytes: u64,
}

#[derive(Default)]
struct Store {
    seq: u64,
    samples: VecDeque<Sample>,
    served: VecDeque<Served>,
}

static STORE: LazyLock<Mutex<Store>> = LazyLock::new(Default::default);
static SERVING: AtomicU32 = AtomicU32::new(0);

fn with<R>(f: impl FnOnce(&mut Store) -> R) -> R {
    f(&mut STORE.lock().unwrap_or_else(PoisonError::into_inner))
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

fn ms(d: Duration) -> f64 {
    (d.as_secs_f64() * 1e5).round() / 1e2
}

/// The time a request spends waiting on the database and the disk, in nanoseconds.
#[derive(Default, Debug)]
struct Timings {
    db: AtomicU64,
    disk: AtomicU64,
}

tokio::task_local! {
    static TIMINGS: Arc<Timings>;
}

fn add(pick: fn(&Timings) -> &AtomicU64, d: Duration) {
    let nanos = u64::try_from(d.as_nanos()).unwrap_or(u64::MAX);
    let _ = TIMINGS.try_with(|t| pick(t).fetch_add(nanos, Ordering::Relaxed));
}

/// Await `f`, counting its time as the request's time in the database.
pub(crate) async fn db<T>(f: impl Future<Output = T>) -> T {
    let start = Instant::now();
    let out = f.await;
    add(|t| &t.db, start.elapsed());
    out
}

/// Await `f`, counting its time as the request's time reading the disk.
pub(crate) async fn disk<T>(f: impl Future<Output = T>) -> T {
    let start = Instant::now();
    let out = f.await;
    add(|t| &t.disk, start.elapsed());
    out
}

/// Measure every request: the handler with its database and disk waits, then the body as it's sent.
pub(crate) async fn record(req: Request, next: Next) -> Response {
    let route = req.extensions().get::<MatchedPath>().map_or_else(String::new, |m| m.as_str().to_owned());
    if route == OWN_ROUTE {
        return next.run(req).await;
    }
    let method = req.method().to_string();
    let (at, start) = (now_ms(), Instant::now());
    let timings = Arc::new(Timings::default());
    SERVING.fetch_add(1, Ordering::Relaxed);
    let resp = TIMINGS.scope(Arc::clone(&timings), next.run(req)).await;
    let status = resp.status().as_u16();
    let (parts, inner) = resp.into_parts();
    let body = Timed {
        inner,
        at,
        method,
        route,
        status,
        start,
        timings,
        produce: Duration::ZERO,
        send: Duration::ZERO,
        bytes: 0,
        mark: Instant::now(),
        waiting: false,
    };
    Response::from_parts(parts, Body::new(body))
}

/// A response body that times itself: producing each chunk, waiting for one, and the gaps while the last is sent.
/// The request is recorded when it's dropped, after the last chunk or when the browser goes away.
struct Timed {
    inner: Body,
    at: u64,
    method: String,
    route: String,
    status: u16,
    start: Instant,
    timings: Arc<Timings>,
    produce: Duration,
    send: Duration,
    bytes: u64,
    /// When the body last answered, and whether that answer was "not ready yet".
    mark: Instant,
    waiting: bool,
}

impl HttpBody for Timed {
    type Data = Bytes;
    type Error = axum::Error;

    fn poll_frame(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Bytes>, axum::Error>>> {
        let this = &mut *self;
        let asked = Instant::now();
        let gap = asked - this.mark;
        if this.waiting {
            this.produce += gap;
        } else {
            this.send += gap;
        }
        let polled = Pin::new(&mut this.inner).poll_frame(cx);
        this.mark = Instant::now();
        this.produce += this.mark - asked;
        this.waiting = polled.is_pending();
        if let Poll::Ready(Some(Ok(frame))) = &polled
            && let Some(data) = frame.data_ref()
        {
            this.bytes += data.len() as u64;
        }
        polled
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.inner.size_hint()
    }
}

impl Drop for Timed {
    fn drop(&mut self) {
        SERVING.fetch_sub(1, Ordering::Relaxed);
        let nanos = |a: &AtomicU64| Duration::from_nanos(a.load(Ordering::Relaxed));
        let served = Served {
            seq: 0,
            at: self.at,
            method: std::mem::take(&mut self.method),
            route: std::mem::take(&mut self.route),
            status: self.status,
            total: ms(self.start.elapsed()),
            db: ms(nanos(&self.timings.db)),
            disk: ms(nanos(&self.timings.disk) + self.produce),
            send: ms(self.send),
            bytes: self.bytes,
        };
        with(|s| {
            s.seq += 1;
            let oldest = now_ms().saturating_sub(KEEP.as_millis() as u64);
            while s.served.len() >= MAX_REQUESTS || s.served.front().is_some_and(|r| r.at < oldest) {
                s.served.pop_front();
            }
            s.served.push_back(Served { seq: s.seq, ..served });
        });
    }
}

fn push_sample(sample: Sample) {
    with(|s| {
        s.seq += 1;
        let oldest = sample.at.saturating_sub(KEEP.as_millis() as u64);
        while s.samples.front().is_some_and(|x| x.at < oldest) {
            s.samples.pop_front();
        }
        s.samples.push_back(Sample { seq: s.seq, ..sample });
    });
}

/// Sample the processors, Hoard's memory and what's running, once a second, on a thread of its own.
pub(crate) fn start() {
    let spawned = std::thread::Builder::new().name("profiler".into()).spawn(|| {
        use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
        let mut sys = match System::new() {
            Ok(sys) => sys,
            Err(e) => {
                tracing::warn!(error = %e, "the profiler can't read the processors");
                return;
            }
        };
        let pid = sysinfo::get_current_pid().ok();
        let refresh = |sys: &mut System| {
            sys.refresh_cpu_usage();
            if let Some(pid) = pid {
                sys.refresh_processes_specifics(
                    ProcessesToUpdate::Some(&[pid]),
                    true,
                    ProcessRefreshKind::nothing().with_cpu().with_memory(),
                );
            }
        };
        refresh(&mut sys);
        loop {
            std::thread::sleep(Duration::from_secs(1));
            refresh(&mut sys);
            let cores = sys.cpus().len().max(1) as f32;
            let (hoard, memory) =
                pid.and_then(|p| sys.process(p)).map_or((0.0_f32, 0), |p| (p.cpu_usage() / cores, p.memory()));
            let running = crate::activity::snapshot()
                .into_iter()
                .filter(|e| e.state == "running")
                .map(|e| Running { kind: e.kind, label: e.label })
                .collect();
            push_sample(Sample {
                seq: 0,
                at: now_ms(),
                hoard: hoard.clamp(0.0, 100.0),
                machine: sys.global_cpu_usage().clamp(0.0, 100.0),
                memory,
                serving: SERVING.load(Ordering::Relaxed),
                running,
            });
        }
    });
    if let Err(e) = spawned {
        tracing::warn!(error = %e, "could not start the profiler");
    }
}

#[derive(Deserialize)]
pub(crate) struct Since {
    /// The `seq` of the last answer: only what's newer comes back.
    #[serde(default)]
    after: u64,
}

#[derive(Serialize)]
pub(crate) struct Profile {
    /// Pass back as `after` to get only what's new.
    seq: u64,
    now: u64,
    cores: usize,
    /// How far back it remembers, in milliseconds.
    keep: u64,
    samples: Vec<Sample>,
    served: Vec<Served>,
}

fn profile(after: u64) -> Profile {
    with(|s| Profile {
        seq: s.seq,
        now: now_ms(),
        cores: std::thread::available_parallelism().map_or(1, std::num::NonZero::get),
        keep: KEEP.as_millis() as u64,
        samples: s.samples.iter().filter(|x| x.seq > after).cloned().collect(),
        served: s.served.iter().filter(|x| x.seq > after).cloned().collect(),
    })
}

/// `GET /api/admin/profile?after=` — the last ten minutes, or what's new since `after`.
pub(crate) async fn handler(Query(since): Query<Since>) -> Json<Profile> {
    Json(profile(since.after))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::routing::get;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn a_request_is_timed_in_its_parts() {
        let app = Router::new()
            .route(
                "/slow/{n}",
                get(|| async {
                    db(tokio::time::sleep(Duration::from_millis(30))).await;
                    disk(tokio::time::sleep(Duration::from_millis(20))).await;
                    "done"
                }),
            )
            .route(OWN_ROUTE, get(|| async { "mine" }))
            .layer(axum::middleware::from_fn(record));
        let before = profile(0).seq;
        let resp = app.clone().oneshot(Request::get("/slow/7").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(resp.into_body().collect().await.unwrap().to_bytes(), "done");
        let resp = app.oneshot(Request::get(OWN_ROUTE).body(Body::empty()).unwrap()).await.unwrap();
        resp.into_body().collect().await.unwrap();
        let p = profile(before);
        let served: Vec<_> = p.served.iter().filter(|r| r.route.starts_with("/slow") || r.route == OWN_ROUTE).collect();
        assert_eq!(served.len(), 1, "{served:?}");
        let r = served[0];
        assert_eq!((r.method.as_str(), r.route.as_str(), r.status, r.bytes), ("GET", "/slow/{n}", 200, 4));
        assert!(r.db >= 30.0 && r.disk >= 20.0 && r.total >= r.db + r.disk, "{r:?}");
        assert!(p.seq > before);
        assert!(profile(p.seq).served.iter().all(|x| x.seq > p.seq));
    }
}
