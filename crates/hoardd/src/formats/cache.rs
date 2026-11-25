//! The conversions kept: when each was last downloaded, how much room they take, and trimming the ones no one has
//! asked for in a while when the cache is over its limit.

use anyhow::Result;

/// A cached conversion handed out in the last hour isn't dropped to make room: someone may be downloading it.
const IN_USE: time::Duration = time::Duration::hours(1);
/// How stale `used_at` may get before it's written again (so a busy conversion isn't a write per request).
const TOUCH_EVERY: time::Duration = time::Duration::minutes(10);

fn ago(d: time::Duration) -> String {
    (time::OffsetDateTime::now_utc() - d).format(&time::format_description::well_known::Rfc3339).unwrap_or_default()
}

/// Note that a cached conversion was just handed out, in the background.
pub(crate) fn touch_conversion(state: &crate::AppState, asset_id: &str, kind: &str) {
    let (pool, id, kind) = (state.pool.clone(), asset_id.to_owned(), kind.to_owned());
    tokio::spawn(async move {
        let r = crate::db::call(&pool, move |c| {
            c.execute(
                "UPDATE derivatives SET used_at = ?3
                  WHERE asset_id = ?1 AND kind = ?2 AND COALESCE(used_at, created_at) < ?4",
                rusqlite::params![id, kind, crate::db::now(), ago(TOUCH_EVERY)],
            )?;
            Ok(())
        })
        .await;
        if let Err(e) = r {
            tracing::debug!(error = %format!("{e:#}"), "couldn't note a conversion's use");
        }
    });
}

/// Space the cached conversions take, each stored file counted once.
pub(crate) fn conversion_cache_bytes(c: &rusqlite::Connection) -> Result<u64> {
    let n: i64 = c.query_row(
        "SELECT COALESCE(SUM(size), 0) FROM blobs
          WHERE hash IN (SELECT blob_hash FROM derivatives WHERE kind LIKE 'as:%')",
        [],
        |r| r.get(0),
    )?;
    Ok(u64::try_from(n).unwrap_or(0))
}

/// Drop the least recently used cached conversions until the rest fit in Settings → Performance's limit. Ones handed out
/// in the last hour stay, so the cache can be over for a while when it's busy. Returns (conversions, bytes) freed.
pub(crate) async fn trim_conversions(state: &crate::AppState) -> Result<(u64, u64)> {
    let limit = state.settings.get().conversion_cache_bytes();
    let doomed = crate::db::call(&state.pool, move |c| {
        let total = conversion_cache_bytes(c)?;
        if total <= limit {
            return Ok(Vec::new());
        }
        let tx = c.transaction()?;
        let rows: Vec<(String, String, String, i64)> = tx
            .prepare(
                "SELECT d.asset_id, d.kind, d.blob_hash, b.size FROM derivatives d JOIN blobs b ON b.hash = d.blob_hash
                  WHERE d.kind LIKE 'as:%' AND COALESCE(d.used_at, d.created_at) < ?1
                  ORDER BY COALESCE(d.used_at, d.created_at) ASC",
            )?
            .query_map([ago(IN_USE)], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut over = total - limit;
        let mut doomed: Vec<(String, i64)> = Vec::new();
        for (id, kind, hash, size) in rows {
            if over == 0 {
                break;
            }
            tx.prepare_cached("DELETE FROM derivatives WHERE asset_id = ?1 AND kind = ?2")?.execute([&id, &kind])?;
            if !doomed.iter().any(|(h, _)| *h == hash) {
                over = over.saturating_sub(u64::try_from(size).unwrap_or(0));
                doomed.push((hash, size));
            }
        }
        tx.commit()?;
        Ok(doomed)
    })
    .await?;
    if doomed.is_empty() {
        return Ok((0, 0));
    }
    crate::jobs::delete_unused(state, doomed).await
}

/// [`trim_conversions`], logging what it did.
pub(crate) async fn log_trim(state: &crate::AppState) {
    match trim_conversions(state).await {
        Ok((0, _)) => {}
        Ok((n, bytes)) => {
            tracing::info!(conversions = n, freed = %crate::maintenance::human_bytes(bytes), "trimmed the conversion cache");
        }
        Err(e) => tracing::warn!(error = %format!("{e:#}"), "couldn't trim the conversion cache"),
    }
}
