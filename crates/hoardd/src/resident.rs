//! A map of what the database knows about each thumbnail or file (a look, a sound's descriptor, an embedding), kept in
//! memory for searches: loaded the first time one needs it, once however many ask at the same time, then kept up to
//! date by the job that adds to it until it's forgotten.

use std::collections::HashMap;
use std::sync::{PoisonError, RwLock};

use anyhow::Result;

use crate::db;

/// A thumbnail's or file's hash as a [`Resident`] map keeps it: a hash's 64 hex digits as the 32 bytes they spell, in
/// the map's own entry, rather than a string of their own elsewhere (more than twice the room, and another place in
/// memory to go to for each). Anything else (a test's name for one) is kept as it's written.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Key {
    Hash([u8; 32]),
    Other(Box<str>),
}

impl Key {
    pub(crate) fn new(s: &str) -> Self {
        hash_bytes(s).map_or_else(|| Self::Other(s.into()), Self::Hash)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        hash_bytes(&s).map_or_else(|| Self::Other(s.into_boxed_str()), Self::Hash)
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hash(bytes) => bytes.iter().try_for_each(|b| write!(f, "{b:02x}")),
            Self::Other(s) => f.write_str(s),
        }
    }
}

/// The bytes 64 lowercase hex digits spell (as a hash is written), or `None` for anything else.
fn hash_bytes(s: &str) -> Option<[u8; 32]> {
    let s = s.as_bytes();
    if s.len() != 64 {
        return None;
    }
    let digit = |c: u8| match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    };
    let mut out = [0; 32];
    for (o, pair) in out.iter_mut().zip(s.as_chunks::<2>().0) {
        *o = digit(pair[0])? << 4 | digit(pair[1])?;
    }
    Some(out)
}

pub(crate) type Map<V> = HashMap<Key, V>;

#[derive(Debug)]
pub(crate) struct Resident<V> {
    map: RwLock<Option<Map<V>>>,
    /// Held while the map loads, so searches arriving together wait for one load rather than each making their own
    /// (and holding a second copy until theirs is thrown away).
    loading: tokio::sync::Mutex<()>,
}

impl<V: Send + 'static> Default for Resident<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Send + 'static> Resident<V> {
    pub(crate) const fn new() -> Self {
        Self { map: RwLock::new(None), loading: tokio::sync::Mutex::const_new(()) }
    }

    /// `f` of the map, or `None` when it isn't loaded.
    pub(crate) fn read<R>(&self, f: impl FnOnce(&Map<V>) -> R) -> Option<R> {
        self.with(|m| m.map(f))
    }

    /// `f` of the map, or of `None` when it isn't loaded.
    pub(crate) fn with<R>(&self, f: impl FnOnce(Option<&Map<V>>) -> R) -> R {
        f(self.map.read().unwrap_or_else(PoisonError::into_inner).as_ref())
    }

    /// Change the map, when it's loaded (otherwise the change is in the database for when it is).
    pub(crate) fn write(&self, f: impl FnOnce(&mut Map<V>)) {
        if let Some(m) = self.map.write().unwrap_or_else(PoisonError::into_inner).as_mut() {
            f(m);
        }
    }

    #[cfg(test)]
    pub(crate) fn set(&self, m: Map<V>) {
        *self.map.write().unwrap_or_else(PoisonError::into_inner) = Some(m);
    }

    /// Let the map go: it's loaded again when next needed.
    pub(crate) fn forget(&self) {
        *self.map.write().unwrap_or_else(PoisonError::into_inner) = None;
    }

    pub(crate) fn len(&self) -> usize {
        self.read(HashMap::len).unwrap_or(0)
    }

    /// Load the map with `load` unless it's loaded. What's loaded is kept only if `wanted` still says so (another
    /// model's embeddings may have been asked for meanwhile).
    pub(crate) async fn ensure<F>(&self, pool: &db::Pool, what: &str, load: F, wanted: impl Fn() -> bool) -> Result<()>
    where
        F: FnOnce(&rusqlite::Connection) -> Result<Map<V>> + Send + 'static,
    {
        if self.read(|_| ()).is_some() {
            return Ok(());
        }
        let _loading = self.loading.lock().await;
        if self.read(|_| ()).is_some() {
            return Ok(());
        }
        let loaded = db::call(pool, move |c| load(c)).await?;
        let mut guard = self.map.write().unwrap_or_else(PoisonError::into_inner);
        if guard.is_none() && wanted() {
            tracing::debug!(entries = loaded.len(), what, "loaded into memory");
            *guard = Some(loaded);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn a_hash_is_kept_as_its_bytes() {
        let hash = "00ff".repeat(16);
        let key = Key::new(&hash);
        assert!(matches!(key, Key::Hash(_)));
        assert_eq!(key.to_string(), hash);
        assert_eq!(key, Key::from(hash.clone()));
        for other in ["t0", &hash.to_uppercase(), &format!("{hash}0"), &"0g".repeat(32)] {
            assert_eq!(Key::new(other), Key::Other(other.into()));
            assert_eq!(Key::new(other).to_string(), other);
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn searches_arriving_together_load_it_once() {
        static LOADS: AtomicUsize = AtomicUsize::new(0);
        let dir = tempfile::tempdir().unwrap();
        let (_, pool) = crate::blobs::tests::setup(dir.path());
        let resident = std::sync::Arc::new(Resident::<u8>::new());
        let asks = (0..8).map(|_| {
            let (resident, pool) = (std::sync::Arc::clone(&resident), pool.clone());
            tokio::spawn(async move {
                let load = |_: &rusqlite::Connection| {
                    LOADS.fetch_add(1, Ordering::Relaxed);
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    Ok(HashMap::from([(Key::new("a"), 1)]))
                };
                resident.ensure(&pool, "test", load, || true).await.unwrap();
            })
        });
        for ask in asks.collect::<Vec<_>>() {
            ask.await.unwrap();
        }
        assert_eq!(LOADS.load(Ordering::Relaxed), 1);
        assert_eq!(resident.read(|m| m[&Key::new("a")]), Some(1));
        resident.forget();
        assert_eq!(resident.len(), 0);
        resident.ensure(&pool, "test", |_| Ok(HashMap::new()), || false).await.unwrap();
        assert!(resident.read(|_| ()).is_none(), "what's no longer wanted isn't kept");
    }
}
