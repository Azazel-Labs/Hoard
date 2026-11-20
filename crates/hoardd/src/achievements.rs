//! Achievements: for the library reaching a size (packs, assets, bytes) and for trying each of Hoard's tools the
//! first time, so someone new finds them all.
//!
//! What's been earned is kept in the database (`achievements`), and whatever embeds the server can hear of each
//! ([`on_achievement`]) to pass it on, to a game store's achievements, say. At each start every one earned so far is
//! handed over again, so one earned while whatever it's passed to wasn't there to hear reaches it the next time.
//!
//! A tool counts as tried when a request for it succeeds ([`watch`], on every API route), and the sizes are looked at
//! now and then while the library changes. docs/ACHIEVEMENTS.md says what each is called and how it's earned.

use std::collections::HashSet;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::Duration;

use anyhow::Result;
use axum::extract::{MatchedPath, Request};
use axum::http::Method;
use axum::middleware::Next;
use axum::response::Response;
use rusqlite::Connection;

use crate::db;

/// What's handed each achievement earned: whatever embeds the server.
type Hook = Box<dyn Fn(&str) + Send + Sync>;

static HOOK: OnceLock<Hook> = OnceLock::new();

/// The achievements earned so far and where to keep the next, once [`start`] has run.
static EARNED: Mutex<Option<Arc<Earned>>> = Mutex::new(None);

struct Earned {
    pool: db::Pool,
    ids: Mutex<HashSet<&'static str>>,
}

/// How often the library's size is looked at, when it has changed since.
const SIZE_CHECK: Duration = Duration::from_secs(60);

/// What a size achievement counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Measure {
    Packs,
    Assets,
    Bytes,
}

const GB: u64 = 1 << 30;

/// The library reaching a size.
const SIZES: &[(Measure, u64, &str)] = &[
    (Measure::Packs, 1, "PACKS_1"),
    (Measure::Packs, 10, "PACKS_10"),
    (Measure::Packs, 100, "PACKS_100"),
    (Measure::Assets, 100, "ASSETS_100"),
    (Measure::Assets, 1_000, "ASSETS_1K"),
    (Measure::Assets, 10_000, "ASSETS_10K"),
    (Measure::Assets, 100_000, "ASSETS_100K"),
    (Measure::Bytes, GB, "BYTES_1GB"),
    (Measure::Bytes, 10 * GB, "BYTES_10GB"),
    (Measure::Bytes, 100 * GB, "BYTES_100GB"),
    (Measure::Bytes, 1024 * GB, "BYTES_1TB"),
];

/// A tool tried: a request that succeeds with this method to this route (as the router writes it, or a path exactly),
/// with this query parameter given when there is one.
struct Use {
    method: Method,
    route: &'static str,
    query: Option<&'static str>,
    id: &'static str,
}

const fn tried(method: Method, route: &'static str, id: &'static str) -> Use {
    Use { method, route, query: None, id }
}

const fn searched(query: &'static str, id: &'static str) -> Use {
    Use { method: Method::GET, route: "/api/assets", query: Some(query), id }
}

const USES: &[Use] = &[
    searched("q", "SEARCH"),
    searched("similar", "MORE_LIKE_THIS"),
    searched("color", "COLOR_SEARCH"),
    searched("imitation", "HUM_A_SOUND"),
    searched("bpm_min", "TEMPO_OR_KEY"),
    searched("bpm_max", "TEMPO_OR_KEY"),
    searched("key", "TEMPO_OR_KEY"),
    tried(Method::POST, "/api/collections", "COLLECTION"),
    tried(Method::POST, "/api/me/favorites", "FAVORITE"),
    tried(Method::POST, "/api/me/searches", "SAVED_SEARCH"),
    tried(Method::PATCH, "/api/assets/{id}", "EDIT_ASSET"),
    tried(Method::POST, "/api/assets/bulk", "BULK_EDIT"),
    tried(Method::POST, "/api/tags/merge", "MERGE_TAGS"),
    tried(Method::GET, "/api/assets/{id}/as/{format}", "DOWNLOAD_AS"),
    tried(Method::GET, "/api/assets/{id}/bundle.zip", "ZIP"),
    tried(Method::GET, "/api/bundle.zip", "ZIP"),
    tried(Method::GET, "/api/collections/{id}/bundle.zip", "ZIP"),
    tried(Method::GET, "/api/packs/{id}/bundle.zip", "ZIP"),
    tried(Method::POST, "/api/credits", "CREDITS"),
    tried(Method::GET, "/api/packs/{id}/credits", "CREDITS"),
    tried(Method::GET, "/api/collections/{id}/credits", "CREDITS"),
    tried(Method::POST, "/api/check", "STORE_CHECK"),
    tried(Method::GET, "/api/duplicates", "NEAR_COPIES"),
    tried(Method::GET, "/api/packs/{id}/store-matches", "STORE_DETAILS"),
    tried(Method::POST, "/api/packs/{id}/retag", "RETAG"),
    tried(Method::POST, "/api/archive", "ARCHIVE"),
    tried(Method::POST, "/api/admin/locations", "LOCATION"),
    tried(Method::POST, "/api/admin/backup", "BACKUP"),
    tried(Method::POST, "/api/admin/maintenance/backup", "BACKUP"),
    tried(Method::POST, "/api/admin/maintenance/{task}", "MAINTENANCE"),
    tried(Method::POST, "/api/admin/tokens", "ACCESS_TOKEN"),
    tried(Method::POST, "/mcp", "AI_APP"),
    tried(Method::PUT, "/api/projects/{name}", "GAME_PROJECT"),
    tried(Method::POST, "/api/admin/conversions/run", "CONVERSION_RULES"),
    tried(Method::POST, "/api/packs/{id}/unreal/convert", "UNREAL"),
    tried(Method::POST, "/api/admin/store-formats", "STORE_FORMAT"),
];

/// Every achievement's API name, each once: the sizes, then the tools.
pub fn all() -> Vec<&'static str> {
    let mut ids: Vec<&'static str> = SIZES.iter().map(|s| s.2).collect();
    for u in USES {
        if !ids.contains(&u.id) {
            ids.push(u.id);
        }
    }
    ids
}

/// Hand each achievement earned to `f`, every one earned before included, once the server has started. Set once, before [`crate::run`]; a second call is ignored.
pub fn on_achievement(f: impl Fn(&str) + Send + Sync + 'static) {
    HOOK.set(Box::new(f)).ok();
}

/// Load what's been earned, hand it all to the hook again and look at the library's size from now on.
pub(crate) async fn start(state: &crate::AppState) {
    let pool = state.pool.clone();
    let earned = db::call(&pool, |c| recorded(c)).await.unwrap_or_else(|e| {
        tracing::warn!(error = %format!("{e:#}"), "couldn't read the achievements earned");
        Vec::new()
    });
    if let Some(hook) = HOOK.get() {
        for id in &earned {
            hook(id);
        }
    }
    *EARNED.lock().unwrap_or_else(PoisonError::into_inner) =
        Some(Arc::new(Earned { pool: pool.clone(), ids: Mutex::new(earned.into_iter().collect()) }));
    tokio::spawn(async move {
        let mut seen = None;
        loop {
            let now = db::writes();
            if seen != Some(now) {
                seen = Some(now);
                match db::call(&pool, |c| sizes(c)).await {
                    Ok(reached) => reached.into_iter().for_each(earn),
                    Err(e) => tracing::debug!(error = %format!("{e:#}"), "couldn't look at the library's size"),
                }
            }
            tokio::time::sleep(SIZE_CHECK).await;
        }
    });
}

/// The achievements in the database, known ones only (one dropped from a later build is let be).
fn recorded(c: &Connection) -> Result<Vec<&'static str>> {
    let known = all();
    let mut ids = Vec::new();
    for id in c.prepare("SELECT id FROM achievements")?.query_map([], |r| r.get::<_, String>(0))? {
        let id = id?;
        if let Some(k) = known.iter().find(|k| **k == id) {
            ids.push(*k);
        }
    }
    Ok(ids)
}

/// The size achievements the library has reached: its packs, its assets, and their bytes (a file in two packs counted
/// in each).
fn sizes(c: &Connection) -> Result<Vec<&'static str>> {
    let packs: i64 = c.query_row("SELECT COUNT(*) FROM packs", [], |r| r.get(0))?;
    let (assets, bytes): (i64, i64) =
        c.query_row("SELECT COALESCE(SUM(assets), 0), COALESCE(SUM(bytes), 0) FROM pack_kinds", [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })?;
    let n = |v: i64| u64::try_from(v).unwrap_or(0);
    Ok(reached(n(packs), n(assets), n(bytes)))
}

fn reached(packs: u64, assets: u64, bytes: u64) -> Vec<&'static str> {
    SIZES
        .iter()
        .filter(|(m, at, _)| {
            *at <= match m {
                Measure::Packs => packs,
                Measure::Assets => assets,
                Measure::Bytes => bytes,
            }
        })
        .map(|s| s.2)
        .collect()
}

/// Earn `id`, if it wasn't already: kept, and handed to the hook.
fn earn(id: &'static str) {
    let Some(earned) = EARNED.lock().unwrap_or_else(PoisonError::into_inner).clone() else {
        return;
    };
    if !earned.ids.lock().unwrap_or_else(PoisonError::into_inner).insert(id) {
        return;
    }
    tracing::info!(achievement = id, "achievement earned");
    if let Some(hook) = HOOK.get() {
        hook(id);
    }
    let pool = earned.pool.clone();
    tokio::spawn(async move {
        let kept = db::call(&pool, move |c| {
            c.execute("INSERT OR IGNORE INTO achievements (id, unlocked_at) VALUES (?1, ?2)", (id, db::now()))?;
            Ok(())
        })
        .await;
        if let Err(e) = kept {
            tracing::warn!(achievement = id, error = %format!("{e:#}"), "couldn't keep an achievement");
        }
    });
}

/// The tools a request tried, if it succeeds: its method, its route as the router matched it, its path and its query.
fn tools<'a>(
    method: &'a Method,
    route: &'a str,
    path: &'a str,
    query: &'a str,
) -> impl Iterator<Item = &'static str> + 'a {
    USES.iter()
        .filter(move |u| {
            u.method == *method
                && (u.route == route || u.route == path)
                && u.query.is_none_or(|key| {
                    query.split('&').any(|pair| pair.split_once('=').is_some_and(|(k, v)| k == key && !v.is_empty()))
                })
        })
        .map(|u| u.id)
}

/// Middleware on every API route: a request that succeeds earns the tool it tried.
pub(crate) async fn watch(req: Request, next: Next) -> Response {
    if EARNED.lock().unwrap_or_else(PoisonError::into_inner).is_none() {
        return next.run(req).await;
    }
    let method = req.method().clone();
    let route = req.extensions().get::<MatchedPath>().map_or_else(String::new, |p| p.as_str().to_owned());
    let path = req.uri().path().to_owned();
    let query = req.uri().query().unwrap_or_default().to_owned();
    let res = next.run(req).await;
    if res.status().is_success() {
        tools(&method, &route, &path, &query).for_each(earn);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_earns_the_tool_it_tried() {
        let got = |m: Method, route: &str, path: &str, q: &str| tools(&m, route, path, q).collect::<Vec<_>>();
        assert_eq!(got(Method::GET, "/api/assets", "/api/assets", "q=crate&limit=60"), ["SEARCH"]);
        // A search box left empty isn't a search.
        assert_eq!(got(Method::GET, "/api/assets", "/api/assets", "q=&limit=60"), [""; 0]);
        assert_eq!(got(Method::GET, "/api/assets", "/api/assets", "similar=a1&q=x"), ["SEARCH", "MORE_LIKE_THIS"]);
        assert_eq!(got(Method::GET, "/api/assets", "/api/assets", "bpm_min=90&bpm_max=120"), ["TEMPO_OR_KEY"; 2]);
        assert_eq!(got(Method::POST, "/api/collections", "/api/collections", ""), ["COLLECTION"]);
        // Listing collections isn't making one.
        assert_eq!(got(Method::GET, "/api/collections", "/api/collections", ""), [""; 0]);
        let zip = got(Method::GET, "/api/packs/{id}/bundle.zip", "/api/packs/p1/bundle.zip", "");
        assert_eq!(zip, ["ZIP"]);
        let backup = "/api/admin/maintenance/backup";
        assert_eq!(got(Method::POST, "/api/admin/maintenance/{task}", backup, ""), ["BACKUP", "MAINTENANCE"]);
        assert_eq!(
            got(Method::POST, "/api/admin/maintenance/{task}", "/api/admin/maintenance/gc", ""),
            ["MAINTENANCE"]
        );
    }

    #[test]
    fn sizes_are_reached_at_their_thresholds() {
        assert_eq!(reached(0, 0, 0), [""; 0]);
        assert_eq!(reached(1, 99, GB - 1), ["PACKS_1"]);
        assert_eq!(reached(12, 1_000, GB), ["PACKS_1", "PACKS_10", "ASSETS_100", "ASSETS_1K", "BYTES_1GB"]);
        assert_eq!(reached(100, 100_000, 1024 * GB).len(), SIZES.len());
    }

    /// Every achievement has a name and a description in docs/ACHIEVEMENTS.md, and the ids are the kind game stores
    /// take: capitals, digits and underscores.
    #[test]
    fn every_achievement_is_documented() {
        let doc = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/ACHIEVEMENTS.md")).unwrap();
        for id in all() {
            assert!(id.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'), "{id}");
            assert!(doc.contains(&format!("| `{id}` |")), "{id} isn't in docs/ACHIEVEMENTS.md");
        }
    }

    #[test]
    fn what_was_earned_is_kept_and_read_back() {
        let dir = tempfile::tempdir().unwrap();
        let pool = db::open(&dir.path().join("hoard.db"), dir.path()).unwrap();
        let c = pool.get().unwrap();
        c.execute_batch(
            "INSERT INTO achievements (id, unlocked_at) VALUES ('SEARCH', 'x'), ('SOMETHING_RETIRED', 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');
             INSERT INTO blobs (hash, size, created_at, status) VALUES ('h', 2000, 'x', 'ok');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a', 'p1', 'h', 'a.png', 'a', 'png', 'texture', 'x', 'x');",
        )
        .unwrap();
        assert_eq!(recorded(&c).unwrap(), ["SEARCH"]);
        assert_eq!(sizes(&c).unwrap(), ["PACKS_1"]);
    }
}
