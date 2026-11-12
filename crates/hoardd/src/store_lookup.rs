//! Looking up packs' store pages in the background, one pack at a time. A page the pack's README links to, or a
//! listing with exactly the title of the pack's Megascans file, is certain: it's attached as the pack's store page
//! (and marked as found automatically, so Store details can offer to take it off again). Anything less is kept as
//! suggestions, so Store details shows them at once.
//!
//! Each lookup is one job: it takes the newest pack without a store page that hasn't been looked up, searches, and
//! queues the next lookup a few seconds later, until none are left. The first is queued after each ingest, at startup
//! and when the setting is turned on.

use anyhow::Result;
use rusqlite::{OptionalExtension as _, params};

use crate::store_search::{self, StoreMatches};
use crate::{AppState, db, jobs};

/// The pause between packs, so the stores see one quiet visitor.
const GAP_SECS: i64 = 3;

/// Queue a lookup unless one is already queued or running (a running one queues the next itself).
pub(crate) async fn start(state: &AppState) {
    if !state.settings.get().store_lookup {
        return;
    }
    let queued = db::call(&state.pool, |c| queue(c, 0, true)).await;
    match queued {
        Ok(true) => state.wake.notify_one(),
        Ok(false) => {}
        Err(e) => tracing::warn!(error = %format!("{e:#}"), "could not queue the store lookup"),
    }
}

/// Queue a lookup to run `after_secs` from now if there's a pack to look up, unless one is queued (or, with
/// `or_running`, running). Returns whether it queued one.
fn queue(c: &rusqlite::Connection, after_secs: i64, or_running: bool) -> Result<bool> {
    let pending: bool = c.query_row(
        "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND (status = 'queued' OR (?2 AND status = 'running')))",
        params![jobs::STORE_LOOKUP, or_running],
        |r| r.get(0),
    )?;
    if pending || next_pack(c)?.is_none() {
        return Ok(false);
    }
    let id = db::enqueue(c, jobs::STORE_LOOKUP, None, &serde_json::json!({}))?;
    if after_secs > 0 {
        let at = (time::OffsetDateTime::now_utc() + time::Duration::seconds(after_secs))
            .format(&time::format_description::well_known::Rfc3339)?;
        c.execute("UPDATE jobs SET run_after = ?2 WHERE id = ?1", params![id, at])?;
    }
    Ok(true)
}

/// The next pack to look up: the newest without a store page that hasn't been looked up yet.
fn next_pack(c: &rusqlite::Connection) -> Result<Option<(String, String)>> {
    Ok(c.query_row(
        "SELECT p.id, p.name FROM packs p LEFT JOIN pack_context c ON c.pack_id = p.id
         WHERE p.status IN ('ready', 'analyzing') AND p.archived_at IS NULL
           AND COALESCE(c.store_url, '') = '' AND c.store_lookup_at IS NULL
         ORDER BY p.created_at DESC LIMIT 1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )
    .optional()?)
}

/// The lookup job: look up the next pack, then queue the one after it.
pub(crate) async fn run(state: &AppState, activity: &crate::activity::Handle) -> Result<()> {
    if !state.settings.get().store_lookup {
        return Ok(());
    }
    let Some((id, name)) = db::call(&state.pool, |c| next_pack(c)).await? else { return Ok(()) };
    activity.detail(name);
    let found = match store_search::find_matches(state, &id, None).await {
        // Every store failed: likely offline. The pack is tried again at the next start or ingest.
        Ok(m) if m.all_failed() => {
            tracing::info!("store lookup stopped: no store could be reached");
            return Ok(());
        }
        Ok(m) => Some(m),
        Err(e) => {
            tracing::info!(pack = %id, error = %format!("{:#}", e.err), "store lookup found nothing to search for");
            None
        }
    };
    if record(state, &id, found).await? {
        crate::context::fill_store_text(state, &id).await;
    }
    if db::call(&state.pool, |c| queue(c, GAP_SECS, false)).await? {
        state.wake.notify_one();
    }
    Ok(())
}

/// Keep what a lookup found: attach the store page if it's certain, or keep the suggestions. Returns whether a page
/// was attached.
async fn record(state: &AppState, pack_id: &str, found: Option<StoreMatches>) -> Result<bool> {
    let pid = pack_id.to_owned();
    db::call(&state.pool, move |c| {
        let now = db::now();
        let tx = c.transaction()?;
        tx.execute("INSERT INTO pack_context (pack_id) VALUES (?1) ON CONFLICT DO NOTHING", [&pid])?;
        let mut attached = false;
        if let Some(found) = &found {
            tx.execute(
                "UPDATE pack_context SET store_matches = ?2 WHERE pack_id = ?1",
                params![pid, serde_json::to_string(found)?],
            )?;
            // A store page set while the search ran stands.
            let free: bool = tx.query_row(
                "SELECT COALESCE(store_url, '') = '' FROM pack_context WHERE pack_id = ?1",
                [&pid],
                |r| r.get(0),
            )?;
            if let Some(m) = found.certain().filter(|_| free) {
                attach(&tx, &pid, m, &now)?;
                attached = true;
            }
        }
        tx.execute("UPDATE pack_context SET store_lookup_at = ?2 WHERE pack_id = ?1", params![pid, now])?;
        tx.commit()?;
        Ok(attached)
    })
    .await
}

/// Make a listing the pack's store page, marked as found automatically. The seller is filled in only where the
/// pack has none, and `store_auto` remembers it, so undoing takes back exactly what this wrote.
fn attach(c: &rusqlite::Connection, pack_id: &str, m: &store_search::StoreMatch, now: &str) -> Result<()> {
    let seller_was: Option<String> =
        c.query_row("SELECT seller FROM pack_context WHERE pack_id = ?1", [pack_id], |r| r.get(0))?;
    let wrote_seller = if seller_was.is_none() { m.author() } else { None };
    let auto = serde_json::json!({ "at": now, "seller": wrote_seller });
    c.execute(
        "UPDATE pack_context SET store = ?2, store_url = ?3, store_text = NULL, store_license = ?4,
                seller = COALESCE(seller, ?5), store_auto = ?6
         WHERE pack_id = ?1",
        params![pack_id, m.store(), m.url(), m.license(), wrote_seller, auto.to_string()],
    )?;
    tracing::info!(pack = pack_id, url = m.url(), "store page attached automatically");
    Ok(())
}

/// Take back a store page that was attached automatically, keeping the suggestions. Returns false if the pack's store
/// page wasn't attached automatically.
pub(crate) fn undo(c: &rusqlite::Connection, pack_id: &str) -> Result<bool> {
    let n = c.execute(
        "UPDATE pack_context SET store = NULL, store_url = NULL, store_text = NULL, store_license = NULL,
                seller = CASE WHEN seller IS json_extract(store_auto, '$.seller') THEN NULL ELSE seller END,
                store_auto = NULL
         WHERE pack_id = ?1 AND store_auto IS NOT NULL",
        [pack_id],
    )?;
    Ok(n > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> rusqlite::Connection {
        let c = rusqlite::Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE pack_context (pack_id TEXT PRIMARY KEY, store TEXT, store_url TEXT, store_text TEXT,
                 license_tier TEXT, readme TEXT, seller TEXT, store_license TEXT, store_auto TEXT, store_matches TEXT,
                 store_lookup_at TEXT);
             INSERT INTO pack_context (pack_id) VALUES ('a'), ('b');
             UPDATE pack_context SET seller = 'Quixel Megascans' WHERE pack_id = 'b';",
        )
        .unwrap();
        c
    }

    #[test]
    fn undo_takes_back_what_was_attached() {
        let c = db();
        let m = store_search::tests::fab_match("Historic Stand", "Quixel", &["Personal", "Professional"]);
        for id in ["a", "b"] {
            attach(&c, id, &m, "2026-11-12T00:00:00Z").unwrap();
        }
        let row = |id: &str| -> (Option<String>, Option<String>, Option<String>) {
            c.query_row("SELECT store_url, seller, store_license FROM pack_context WHERE pack_id = ?1", [id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .unwrap()
        };
        assert_eq!(row("a").1.as_deref(), Some("Quixel"));
        assert_eq!(row("a").2.as_deref(), Some(store_search::FAB_STANDARD));
        assert_eq!(row("b").1.as_deref(), Some("Quixel Megascans"));
        assert!(undo(&c, "a").unwrap() && undo(&c, "b").unwrap());
        assert_eq!(row("a"), (None, None, None));
        // The seller that came from the pack's own files stays.
        assert_eq!(row("b"), (None, Some("Quixel Megascans".to_owned()), None));
        assert!(!undo(&c, "a").unwrap());
    }
}
