//! What's slow to work out from the library and fine to show a little late: kept as last made, answered at once,
//! and made again in the background once the library has changed since.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use anyhow::Result;
use rusqlite::Connection;

use super::Pool;

/// Something worked out from the whole library, by `make`, kept between asks.
pub(crate) struct Kept<T> {
    /// What it is, for the log.
    name: &'static str,
    make: fn(&Connection) -> Result<T>,
    /// One this new isn't made again, whatever has changed since: a busy library changes all the time.
    fresh_enough: Duration,
    made: Mutex<Option<Made<T>>>,
    busy: AtomicBool,
}

struct Made<T> {
    value: Arc<T>,
    at: Instant,
    at_text: String,
    /// The library's write count when it was made: while it's the same, nothing has changed.
    writes: u64,
}

/// What was kept, when it was made, and whether a newer one is on its way.
pub(crate) struct Got<T> {
    pub(crate) value: Arc<T>,
    pub(crate) made_at: String,
    pub(crate) refreshing: bool,
}

impl<T: Send + Sync + 'static> Kept<T> {
    pub(crate) const fn new(name: &'static str, make: fn(&Connection) -> Result<T>, fresh_enough: Duration) -> Self {
        Self { name, make, fresh_enough, made: Mutex::new(None), busy: AtomicBool::new(false) }
    }

    /// The one kept, made first when there's none yet. When the library has changed since, a new one is started in
    /// the background, for whoever asks next.
    pub(crate) async fn get(&'static self, pool: &Pool) -> Result<Got<T>> {
        let kept = || {
            self.made.lock().unwrap_or_else(PoisonError::into_inner).as_ref().map(|m| {
                let outdated = m.writes != super::writes() && m.at.elapsed() >= self.fresh_enough;
                (Arc::clone(&m.value), m.at_text.clone(), outdated)
            })
        };
        let (value, made_at, outdated) = if let Some(k) = kept() {
            k
        } else {
            self.refresh(pool).await?;
            kept().ok_or_else(|| anyhow::anyhow!("{} couldn't be worked out", self.name))?
        };
        if outdated && !self.busy.load(Ordering::Acquire) {
            let pool = pool.clone();
            tokio::spawn(async move { self.refresh_logged(&pool).await });
        }
        Ok(Got { value, made_at, refreshing: outdated || self.busy.load(Ordering::Acquire) })
    }

    /// Whether the library has changed since the one kept was made, or there's none.
    pub(crate) fn changed(&self) -> bool {
        self.made.lock().unwrap_or_else(PoisonError::into_inner).as_ref().is_none_or(|m| m.writes != super::writes())
    }

    /// Make it again and keep it. When that's being done already, it waits for that one instead.
    pub(crate) async fn refresh(&'static self, pool: &Pool) -> Result<()> {
        if self.busy.swap(true, Ordering::AcqRel) {
            while self.busy.load(Ordering::Acquire) {
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            return Ok(());
        }
        // Free again however this ends, a request that was waiting for it gone included.
        let _done = Done(&self.busy);
        // Counted before it's made, so a write while it's being made leaves it outdated, not passed for current.
        let writes = super::writes();
        let make = self.make;
        let value = super::call(pool, move |c| make(c)).await?;
        *self.made.lock().unwrap_or_else(PoisonError::into_inner) =
            Some(Made { value: Arc::new(value), at: Instant::now(), at_text: super::now(), writes });
        Ok(())
    }

    /// [`Self::refresh`], a failure only logged.
    pub(crate) async fn refresh_logged(&'static self, pool: &Pool) {
        if let Err(e) = self.refresh(pool).await {
            tracing::warn!(error = %format!("{e:#}"), what = self.name, "could not work out what's kept");
        }
    }
}

struct Done<'a>(&'a AtomicBool);

impl Drop for Done<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static COUNTED: Kept<i64> =
        Kept::new("a count", |c| Ok(c.query_row("SELECT COUNT(*) FROM packs", [], |r| r.get(0))?), Duration::ZERO);

    #[tokio::test]
    async fn it_is_kept_until_the_library_changes_then_made_again_in_the_background() {
        let dir = tempfile::tempdir().unwrap();
        let pool = super::super::open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let first = COUNTED.get(&pool).await.unwrap();
        assert_eq!(*first.value, 0);

        super::super::call(&pool, |c| {
            c.execute("INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'p', 'ready', '2026-01-01')", [])?;
            Ok(())
        })
        .await
        .unwrap();
        assert!(COUNTED.changed());
        // Answered at once with the one kept, and a new one started.
        let stale = COUNTED.get(&pool).await.unwrap();
        assert_eq!(*stale.value, 0);
        assert!(stale.refreshing);
        for _ in 0..100 {
            if !COUNTED.busy.load(Ordering::Acquire)
                && COUNTED.made.lock().unwrap().as_ref().is_some_and(|m| *m.value == 1)
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        let fresh = COUNTED.get(&pool).await.unwrap();
        assert_eq!(*fresh.value, 1);
    }
}
