//! What's slow to work out from the library and fine to show a little late: kept as last made, answered at once,
//! and made again in the background once the library has changed since.
//!
//! What's kept is also saved, in a file of its own in the cache (see [`save_in`]), so the first ask after a restart
//! is answered at once with the one from before, and a new one made behind it. Not in the database: every write
//! there counts as the library changing ([`super::writes`]), so saving it would leave it outdated as soon as saved.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::Pool;

/// Something worked out from the whole library, by `make`, kept between asks.
pub(crate) struct Kept<T> {
    /// What it is, for the log.
    name: &'static str,
    /// The name of the file it's saved in between starts.
    file: &'static str,
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
    /// Saved before the server last started: what's changed since can't be told, so it's made again.
    from_before: bool,
}

impl<T> Made<T> {
    fn outdated(&self, fresh_enough: Duration) -> bool {
        self.from_before || (self.writes != super::writes() && self.at.elapsed() >= fresh_enough)
    }
}

/// The folder what's kept is saved in; until it's set (as in tests), nothing is saved or read back.
static DIR: OnceLock<PathBuf> = OnceLock::new();

/// Save what's kept in `dir` from now on, and read it back from there.
pub(crate) fn save_in(dir: PathBuf) {
    DIR.get_or_init(|| dir);
}

/// What's kept as it's saved: with when it was made, shown as its age.
#[derive(Serialize, Deserialize)]
struct Saved<T> {
    made_at: String,
    value: T,
}

/// Saves `value` as `file`, made at `made_at`, unless there's nowhere to (see [`save_in`]). Written beside it first and
/// then put in its place, so a server stopped halfway leaves the one before, not half of this one.
pub(crate) fn save<T: Serialize>(file: &str, made_at: &str, value: &T) -> Result<()> {
    let Some(dir) = DIR.get() else { return Ok(()) };
    save_to(dir, file, made_at, value)
}

fn save_to<T: Serialize>(dir: &Path, file: &str, made_at: &str, value: &T) -> Result<()> {
    std::fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
    let path = dir.join(format!("{file}.json"));
    let part = dir.join(format!("{file}.json.part"));
    let mut out =
        std::io::BufWriter::new(std::fs::File::create(&part).with_context(|| format!("create {}", part.display()))?);
    serde_json::to_writer(&mut out, &Saved { made_at: made_at.to_owned(), value })?;
    out.into_inner().map_err(std::io::IntoInnerError::into_error)?.sync_all()?;
    std::fs::rename(&part, &path).with_context(|| format!("save {}", path.display()))?;
    Ok(())
}

/// The one saved as `file` and when it was made, if there is one this build can read. One it can't, saved by an
/// older build, say, is as good as none: it's made again.
pub(crate) fn saved<T: DeserializeOwned>(file: &str) -> Option<(T, String)> {
    saved_in(DIR.get()?, file)
}

fn saved_in<T: DeserializeOwned>(dir: &Path, file: &str) -> Option<(T, String)> {
    let path = dir.join(format!("{file}.json"));
    let read = std::fs::File::open(&path).ok()?;
    match serde_json::from_reader::<_, Saved<T>>(std::io::BufReader::new(read)) {
        Ok(s) => Some((s.value, s.made_at)),
        Err(e) => {
            tracing::info!(error = %e, path = %path.display(), "what was kept before can't be read; it's made again");
            None
        }
    }
}

/// [`saved`], off the async runtime: some are big.
pub(crate) async fn saved_later<T: DeserializeOwned + Send + 'static>(file: &'static str) -> Option<(T, String)> {
    tokio::task::spawn_blocking(move || saved(file)).await.ok().flatten()
}

/// [`save`], off the async runtime, a failure only logged.
pub(crate) async fn save_later<T: Serialize + Send + Sync + 'static>(
    file: &'static str,
    made_at: String,
    value: Arc<T>,
) {
    let saving = tokio::task::spawn_blocking(move || save(file, &made_at, &*value)).await;
    if let Err(e) = saving.map_err(anyhow::Error::from).and_then(|r| r) {
        tracing::warn!(error = %format!("{e:#}"), file, "could not save what's kept for the next start");
    }
}

/// What was kept, when it was made, and whether a newer one is on its way.
pub(crate) struct Got<T> {
    pub(crate) value: Arc<T>,
    pub(crate) made_at: String,
    pub(crate) refreshing: bool,
}

impl<T: Serialize + DeserializeOwned + Send + Sync + 'static> Kept<T> {
    pub(crate) const fn new(
        name: &'static str,
        file: &'static str,
        make: fn(&Connection) -> Result<T>,
        fresh_enough: Duration,
    ) -> Self {
        Self { name, file, make, fresh_enough, made: Mutex::new(None), busy: AtomicBool::new(false) }
    }

    /// The one kept, made first when there's none yet: the one saved before the server started, if there is one.
    /// When the library has changed since, a new one is started in the background, for whoever asks next.
    pub(crate) async fn get(&'static self, pool: &Pool) -> Result<Got<T>> {
        let kept = || {
            self.made
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .as_ref()
                .map(|m| (Arc::clone(&m.value), m.at_text.clone(), m.outdated(self.fresh_enough)))
        };
        if kept().is_none()
            && let Some((value, at_text)) = saved_later::<T>(self.file).await
        {
            self.adopt(value, at_text);
        }
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

    /// Keeps one saved before the server started, unless one's been made meanwhile.
    fn adopt(&self, value: T, at_text: String) {
        let mut made = self.made.lock().unwrap_or_else(PoisonError::into_inner);
        if made.is_none() {
            *made = Some(Made { value: Arc::new(value), at: Instant::now(), at_text, writes: 0, from_before: true });
        }
    }

    /// Whether the library may have changed since the one kept was made, or there's none.
    pub(crate) fn changed(&self) -> bool {
        self.made
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
            .is_none_or(|m| m.from_before || m.writes != super::writes())
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
        let value = Arc::new(super::call(pool, move |c| make(c)).await?);
        let at_text = super::now();
        *self.made.lock().unwrap_or_else(PoisonError::into_inner) = Some(Made {
            value: Arc::clone(&value),
            at: Instant::now(),
            at_text: at_text.clone(),
            writes,
            from_before: false,
        });
        // Still busy while it's saved, so the next one can't be saved over it halfway.
        save_later(self.file, at_text, value).await;
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

    static COUNTED: Kept<i64> = Kept::new(
        "a count",
        "a-count",
        |c| Ok(c.query_row("SELECT COUNT(*) FROM packs", [], |r| r.get(0))?),
        Duration::ZERO,
    );

    #[test]
    fn what_is_saved_is_read_back_and_what_cant_be_read_is_none() {
        let dir = tempfile::tempdir().unwrap();
        save_to(dir.path(), "sizes", "2026-11-20T10:00:00Z", &vec![1_u64, 2, 3]).unwrap();
        assert_eq!(saved_in::<Vec<u64>>(dir.path(), "sizes"), Some((vec![1, 2, 3], "2026-11-20T10:00:00Z".to_owned())));
        // Saved again over it, and nothing left beside it.
        save_to(dir.path(), "sizes", "2026-11-20T11:00:00Z", &vec![4_u64]).unwrap();
        assert_eq!(saved_in::<Vec<u64>>(dir.path(), "sizes"), Some((vec![4], "2026-11-20T11:00:00Z".to_owned())));
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
        // Not what this build saves as that: made again.
        assert_eq!(saved_in::<String>(dir.path(), "sizes"), None);
        assert_eq!(saved_in::<Vec<u64>>(dir.path(), "nothing"), None);
    }

    #[test]
    fn one_saved_before_the_start_is_shown_and_made_again() {
        static FROM_BEFORE: Kept<i64> = Kept::new("a number", "a-number", |_| Ok(2), Duration::from_hours(1));
        FROM_BEFORE.adopt(1, "2026-11-19T10:00:00Z".to_owned());
        assert!(FROM_BEFORE.changed());
        assert!(FROM_BEFORE.made.lock().unwrap().as_ref().unwrap().outdated(Duration::from_hours(1)));
        // One made meanwhile isn't replaced by the one from before.
        FROM_BEFORE.adopt(0, "2026-11-18T10:00:00Z".to_owned());
        assert_eq!(*FROM_BEFORE.made.lock().unwrap().as_ref().unwrap().value, 1);
    }

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
