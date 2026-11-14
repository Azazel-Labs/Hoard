//! Content-addressed, write-once blob stores, one per managed library location, over any `object_store` backend.
//!
//! Keys look like `ab/cd/abcd…(64 hex)` where the name is the BLAKE3 hash of the bytes, so:
//! - identical files across packs are stored once,
//! - corruption is detectable (re-hash and compare: see `verify` and the scrub job),
//! - blobs never change, so CDN / browser caches can keep them forever, and backups are a plain sync.
//!
//! Where a file goes is set by the library locations (see `locations.rs`): a managed location is a folder on this
//! machine, or an S3-compatible bucket (`s3://bucket`, using the server's `HOARD_S3_*` settings), and new files go
//! to the one that takes their kind, else the catch-all. A watched location isn't a blob store at all: its files
//! stay where they are, and a read of one of them finds it through the database by its hash.
//!
//! Thumbnails, model views and audio previews go to another store, always a local directory in the cache dir
//! (`put_cache_bytes`): they're small, read constantly, and re-analyzing a pack makes them again. Reads look there
//! first, then in the managed locations (local before remote), then in the watched ones.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::{Path as FsPath, PathBuf};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use anyhow::{Context, Result, bail};
use bytes::Bytes;
use futures_util::StreamExt;
use futures_util::stream::BoxStream;
use hoard_core::AssetKind;
use object_store::aws::AmazonS3Builder;
use object_store::buffered::BufWriter;
use object_store::local::LocalFileSystem;
use object_store::path::Path;
use object_store::{GetOptions, GetRange, ObjectStore, ObjectStoreExt, PutPayload};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::config::Config;
use crate::db;
use crate::i18n::tr;
use crate::locations::Mode;

/// What an upload is refused with when there's nowhere to put it.
pub(crate) fn no_library() -> String {
    tr!("error-no-library")
}

#[derive(Clone)]
pub(crate) struct BlobStore {
    cache: Arc<dyn ObjectStore>,
    /// For reading watched files by their absolute paths.
    fs: Arc<LocalFileSystem>,
    libs: Arc<RwLock<Arc<Libraries>>>,
    traffic: Arc<Mutex<Traffic>>,
    cfg: Arc<Config>,
    pool: db::Pool,
}

impl fmt::Debug for BlobStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlobStore").field("locations", &self.label()).finish_non_exhaustive()
    }
}

/// Which managed locations new files may go to, and which are being written to. One lock covers both, so a
/// location being emptied can wait out the writes that chose it before it started.
#[derive(Default)]
struct Traffic {
    /// Managed locations being emptied: new files don't go to them.
    draining: HashSet<i64>,
    /// Writes under way, by location.
    writing: HashMap<i64, usize>,
}

/// The open stores of the library locations, swapped whole when they change.
#[derive(Default)]
struct Libraries {
    /// In the order they were added, which is the order routing tries them in.
    managed: Vec<Managed>,
    watched: Vec<String>,
}

pub(crate) struct Managed {
    pub(crate) id: i64,
    pub(crate) store: Arc<dyn ObjectStore>,
    kinds: Vec<String>,
    catch_all: bool,
    remote: bool,
    label: String,
}

impl Libraries {
    /// Where a new file of this kind goes: the first location that takes the kind, else the catch-all, else the
    /// first managed location there is.
    /// Locations in `skip` (ones being emptied) are left out.
    fn target(&self, kind: AssetKind, skip: &HashSet<i64>) -> Option<&Managed> {
        let mut open = self.managed.iter().filter(|m| !skip.contains(&m.id));
        open.clone()
            .find(|m| m.kinds.iter().any(|k| k == kind.as_str()))
            .or_else(|| open.clone().find(|m| m.catch_all))
            .or_else(|| open.next())
    }

    /// Managed stores in read order: local disks before buckets.
    fn read_order(&self) -> impl Iterator<Item = &Managed> {
        self.managed.iter().filter(|m| !m.remote).chain(self.managed.iter().filter(|m| m.remote))
    }

    async fn holds(&self, hash: &str) -> Result<bool> {
        for m in self.read_order() {
            if exists_in(&*m.store, hash).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StoredBlob {
    pub(crate) hash: String,
    pub(crate) size: u64,
}

/// A (possibly partial) read of a blob, ready to stream to a client.
pub(crate) struct BlobRead {
    pub(crate) total_size: u64,
    pub(crate) range: std::ops::Range<u64>,
    pub(crate) stream: BoxStream<'static, object_store::Result<Bytes>>,
}

impl fmt::Debug for BlobRead {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlobRead")
            .field("total_size", &self.total_size)
            .field("range", &self.range)
            .finish_non_exhaustive()
    }
}

pub(crate) fn is_valid_hash(h: &str) -> bool {
    h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub(crate) fn key(hash: &str) -> Path {
    Path::from(format!("{}/{}/{hash}", &hash[0..2], &hash[2..4]))
}

/// The bucket of an `s3://bucket` location path, or `None` for a folder.
pub(crate) fn s3_bucket(path: &str) -> Option<&str> {
    path.strip_prefix("s3://").map(|b| b.trim_end_matches('/'))
}

/// Open the store of a managed location. A folder must already exist: one on a disk that isn't mounted right now
/// shouldn't be quietly recreated on the disk underneath.
pub(crate) fn open_store(cfg: &Config, path: &str) -> Result<(Arc<dyn ObjectStore>, bool)> {
    let Some(bucket) = s3_bucket(path) else {
        let dir = FsPath::new(path);
        if !dir.is_dir() {
            bail!(tr!("error-folder-missing", path = path));
        }
        return Ok((Arc::new(LocalFileSystem::new_with_prefix(dir)?), false));
    };
    if bucket.is_empty() || bucket.contains('/') {
        bail!(tr!("error-s3-path"));
    }
    let mut b = AmazonS3Builder::new()
        .with_bucket_name(bucket)
        .with_region(&cfg.s3_region)
        // Self-hosted servers want path-style URLs: http://nas:3900/bucket/key
        .with_virtual_hosted_style_request(false);
    if let Some(ep) = &cfg.s3_endpoint {
        b = b.with_endpoint(ep).with_allow_http(ep.starts_with("http://"));
    }
    if let (Some(k), Some(s)) = (&cfg.s3_access_key, &cfg.s3_secret_key) {
        b = b.with_access_key_id(k).with_secret_access_key(s);
    }
    if !cfg.s3_headers.is_empty() {
        let mut headers = http::HeaderMap::new();
        for h in &cfg.s3_headers {
            let (name, value) = h.split_once('=').context("HOARD_S3_HEADERS entries look like Name=value")?;
            headers.insert(
                http::HeaderName::from_bytes(name.trim().as_bytes())?,
                http::HeaderValue::from_str(value.trim())?,
            );
        }
        b = b.with_client_options(object_store::ClientOptions::new().with_default_headers(headers));
    }
    Ok((Arc::new(b.build()?), true))
}

fn describe(cfg: &Config, path: &str) -> String {
    match s3_bucket(path) {
        Some(bucket) => format!("s3:{}/{bucket}", cfg.s3_endpoint.as_deref().unwrap_or("aws")),
        None => format!("local:{path}"),
    }
}

impl BlobStore {
    /// Open the cache store and every library location in the database.
    pub(crate) fn open(cfg: &Config, pool: db::Pool) -> Result<Self> {
        let thumbs = cfg.thumbs_dir();
        std::fs::create_dir_all(&thumbs).with_context(|| format!("create {}", thumbs.display()))?;
        let cache = Arc::new(LocalFileSystem::new_with_prefix(&thumbs)?);
        let store = Self {
            cache,
            fs: Arc::new(LocalFileSystem::new()),
            libs: Arc::default(),
            traffic: Arc::default(),
            cfg: Arc::new(cfg.clone()),
            pool,
        };
        store.reload()?;
        Ok(store)
    }

    /// Re-read the library locations, after one is added, changed or removed.
    pub(crate) fn reload(&self) -> Result<()> {
        let locations = crate::locations::list(&*self.pool.get()?)?;
        let mut libs = Libraries::default();
        for l in &locations {
            match l.mode {
                Mode::Managed => match open_store(&self.cfg, &l.path) {
                    Ok((store, remote)) => libs.managed.push(Managed {
                        id: l.id,
                        store,
                        kinds: l.kinds.clone(),
                        catch_all: l.catch_all,
                        remote,
                        label: describe(&self.cfg, &l.path),
                    }),
                    Err(e) => {
                        tracing::warn!(location = %l.name, error = %e, "can't open this library location; skipping it");
                    }
                },
                Mode::Watched => libs.watched.push(format!("watched:{}", l.path)),
            }
        }
        *self.libs.write().unwrap_or_else(PoisonError::into_inner) = Arc::new(libs);
        Ok(())
    }

    /// [`reload`](Self::reload) off the async threads: it opens folders, which can stall on a sleeping share.
    pub(crate) async fn refresh(&self) -> Result<()> {
        let me = self.clone();
        tokio::task::spawn_blocking(move || me.reload()).await?
    }

    fn traffic(&self) -> std::sync::MutexGuard<'_, Traffic> {
        self.traffic.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Stop sending new files to a managed location while it's emptied, until the guard is dropped.
    pub(crate) fn drain(&self, id: i64) -> Draining {
        self.traffic().draining.insert(id);
        Draining { traffic: Arc::clone(&self.traffic), id }
    }

    /// Wait until the writes to a location that started before it was drained have finished.
    pub(crate) async fn writes_done(&self, id: i64) {
        while self.traffic().writing.get(&id).is_some_and(|n| *n > 0) {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        }
    }

    /// Pick where a new file of this kind goes and count the write against it until the guard is dropped.
    fn claim<'a>(&self, libs: &'a Libraries, kind: AssetKind) -> Option<(&'a Managed, Writing)> {
        let mut t = self.traffic();
        let m = libs.target(kind, &t.draining)?;
        *t.writing.entry(m.id).or_default() += 1;
        Some((m, Writing { traffic: Arc::clone(&self.traffic), id: m.id }))
    }

    fn libs(&self) -> Arc<Libraries> {
        Arc::clone(&self.libs.read().unwrap_or_else(PoisonError::into_inner))
    }

    /// Where the files are, for the log and the portal's About.
    pub(crate) fn label(&self) -> String {
        let libs = self.libs();
        let all: Vec<&str> =
            libs.managed.iter().map(|m| m.label.as_str()).chain(libs.watched.iter().map(String::as_str)).collect();
        if all.is_empty() { "no library location yet".to_owned() } else { all.join(", ") }
    }

    /// Whether there's a managed location for new files to go to.
    pub(crate) fn can_store(&self) -> bool {
        !self.libs().managed.is_empty()
    }

    /// The store of one managed location, if it's open.
    pub(crate) fn managed_store(&self, id: i64) -> Option<Arc<dyn ObjectStore>> {
        self.libs().managed.iter().find(|m| m.id == id).map(|m| Arc::clone(&m.store))
    }

    /// The open managed locations, as (id, store), in the order they were added.
    pub(crate) fn managed_stores(&self) -> Vec<(i64, Arc<dyn ObjectStore>)> {
        self.libs().managed.iter().map(|m| (m.id, Arc::clone(&m.store))).collect()
    }

    /// Where a file of this kind belongs, leaving out the location `skip`: (id, store).
    pub(crate) fn route(&self, kind: AssetKind, skip: Option<i64>) -> Option<(i64, Arc<dyn ObjectStore>)> {
        let mut skip_all = self.traffic().draining.clone();
        skip_all.extend(skip);
        self.libs().target(kind, &skip_all).map(|m| (m.id, Arc::clone(&m.store)))
    }

    /// Hash a local file without storing it.
    pub(crate) fn hash_file(path: &FsPath) -> Result<StoredBlob> {
        let mut hasher = blake3::Hasher::new();
        let file = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
        let size = std::io::copy(&mut std::io::BufReader::with_capacity(1 << 20, file), &mut hasher)?;
        Ok(StoredBlob { hash: hasher.finalize().to_hex().to_string(), size })
    }

    /// Upload a local file (streamed, multipart for big files) to the location that takes its kind. Skips the
    /// upload if any managed location already has the blob, which is what makes re-uploading the same pack cheap.
    pub(crate) async fn put_file(&self, path: &FsPath, blob: &StoredBlob, kind: AssetKind) -> Result<()> {
        let libs = self.libs();
        if libs.holds(&blob.hash).await? {
            return Ok(());
        }
        let (target, _writing) = self.claim(&libs, kind).with_context(no_library)?;
        write_file(&target.store, path, &key(&blob.hash)).await
    }

    /// Store bytes made here (a conversion, an Unreal package's content) with the files of that kind. A library
    /// that's only watched folders has nowhere for them, so they go to the cache, which re-analysis remakes.
    pub(crate) async fn put_bytes(&self, bytes: Vec<u8>, kind: AssetKind) -> Result<StoredBlob> {
        let hash = blake3::hash(&bytes).to_hex().to_string();
        let size = bytes.len() as u64;
        let libs = self.libs();
        if !libs.holds(&hash).await? {
            match self.claim(&libs, kind) {
                Some((m, _writing)) => {
                    m.store.put(&key(&hash), PutPayload::from(bytes)).await?;
                }
                None => return self.put_cache_bytes(bytes).await,
            }
        }
        Ok(StoredBlob { hash, size })
    }

    /// Store a thumbnail, model views or an audio preview in the local cache store rather than with the files.
    pub(crate) async fn put_cache_bytes(&self, bytes: Vec<u8>) -> Result<StoredBlob> {
        let hash = blake3::hash(&bytes).to_hex().to_string();
        let size = bytes.len() as u64;
        if !exists_in(&*self.cache, &hash).await? {
            self.cache.put(&key(&hash), PutPayload::from(bytes)).await?;
        }
        Ok(StoredBlob { hash, size })
    }

    /// Open a blob for streaming, optionally a byte range (for HTTP `Range`, so audio/video can seek).
    pub(crate) async fn read(&self, hash: &str, range: Option<GetRange>) -> Result<Option<BlobRead>> {
        let opts = GetOptions { range, ..Default::default() };
        match self.cache.get_opts(&key(hash), opts.clone()).await {
            Ok(res) => return Ok(Some(blob_read(res))),
            Err(object_store::Error::NotFound { .. }) => {}
            Err(e) => tracing::warn!(hash, error = %e, "could not read the cache store; trying the library"),
        }
        let libs = self.libs();
        let mut failed = None;
        for m in libs.read_order() {
            match m.store.get_opts(&key(hash), opts.clone()).await {
                Ok(res) => return Ok(Some(blob_read(res))),
                Err(object_store::Error::NotFound { .. }) => {}
                Err(e) => {
                    tracing::warn!(hash, location = %m.label, error = %e, "could not read from a library location");
                    failed = Some(e);
                }
            }
        }
        if !libs.watched.is_empty()
            && let Some(file) = self.watched_file(hash).await?
        {
            let path = Path::from_filesystem_path(&file)?;
            match self.fs.get_opts(&path, opts).await {
                Ok(res) => return Ok(Some(blob_read(res))),
                Err(object_store::Error::NotFound { .. }) => {}
                Err(e) => failed = Some(e),
            }
        }
        match failed {
            Some(e) => Err(e.into()),
            None => Ok(None),
        }
    }

    /// Where a file of a watched location with this hash is on disk, if one is there.
    async fn watched_file(&self, hash: &str) -> Result<Option<PathBuf>> {
        let hash = hash.to_owned();
        db::call(&self.pool, move |c| {
            let mut stmt = c.prepare_cached(
                "SELECT l.path, p.folder, a.path FROM assets a
                   JOIN packs p ON p.id = a.pack_id
                   JOIN locations l ON l.id = p.location_id
                  WHERE a.blob_hash = ?1 AND l.mode = 'watched'
                  LIMIT 8",
            )?;
            let rows = stmt.query_map([&hash], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?, r.get::<_, String>(2)?))
            })?;
            for row in rows {
                let (root, folder, rel) = row?;
                let file = crate::locations::watched_path(&root, folder.as_deref(), &rel);
                if file.is_file() {
                    return Ok(Some(file));
                }
            }
            Ok(None)
        })
        .await
    }

    pub(crate) async fn read_all(&self, hash: &str) -> Result<Bytes> {
        let Some(mut read) = self.read(hash, None).await? else { bail!("blob {hash} missing from storage") };
        let mut out = Vec::with_capacity(usize::try_from(read.total_size).unwrap_or(0));
        while let Some(chunk) = read.stream.next().await {
            out.extend_from_slice(&chunk?);
        }
        Ok(out.into())
    }

    /// A blob's first `n` bytes (fewer if it's shorter).
    pub(crate) async fn head(&self, hash: &str, n: u64) -> Result<Vec<u8>> {
        let Some(mut read) = self.read(hash, Some(GetRange::Bounded(0..n))).await? else {
            bail!("blob {hash} missing from storage")
        };
        let mut out = Vec::new();
        while let Some(chunk) = read.stream.next().await {
            out.extend_from_slice(&chunk?);
        }
        Ok(out)
    }

    /// Copy a blob into a local file (used when re-analyzing an asset).
    pub(crate) async fn download_to(&self, hash: &str, dest: &FsPath) -> Result<()> {
        let Some(mut read) = self.read(hash, None).await? else { bail!("blob {hash} missing from storage") };
        let mut out = tokio::fs::File::create(dest).await?;
        while let Some(chunk) = read.stream.next().await {
            out.write_all(&chunk?).await?;
        }
        out.flush().await?;
        Ok(())
    }

    /// Re-hash a stored blob and report whether it still matches its name. `Ok(None)` = missing.
    pub(crate) async fn verify(&self, hash: &str) -> Result<Option<bool>> {
        let Some(mut read) = self.read(hash, None).await? else { return Ok(None) };
        let mut hasher = blake3::Hasher::new();
        while let Some(chunk) = read.stream.next().await {
            hasher.update(&chunk?);
        }
        Ok(Some(hasher.finalize().to_hex().as_str() == hash))
    }

    /// Delete a blob from the cache and every managed location. Watched folders are never touched.
    pub(crate) async fn delete(&self, hash: &str) -> Result<()> {
        let libs = self.libs();
        for store in std::iter::once(&self.cache).chain(libs.managed.iter().map(|m| &m.store)) {
            match store.delete(&key(hash)).await {
                Ok(()) | Err(object_store::Error::NotFound { .. }) => {}
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}

/// While held, new files aren't routed to the location being emptied.
pub(crate) struct Draining {
    traffic: Arc<Mutex<Traffic>>,
    id: i64,
}

impl Drop for Draining {
    fn drop(&mut self) {
        self.traffic.lock().unwrap_or_else(PoisonError::into_inner).draining.remove(&self.id);
    }
}

/// A write under way to a managed location.
struct Writing {
    traffic: Arc<Mutex<Traffic>>,
    id: i64,
}

impl Drop for Writing {
    fn drop(&mut self) {
        let mut t = self.traffic.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(n) = t.writing.get_mut(&self.id) {
            *n = n.saturating_sub(1);
        }
    }
}

/// Whether a store holds any of Hoard's files. Anything else in it (a `Thumbs.db`, other objects in a shared bucket)
/// doesn't count, so it doesn't stop a location from being removed.
pub(crate) async fn holds_any(store: &dyn ObjectStore) -> Result<bool> {
    let mut list = store.list(None);
    while let Some(meta) = list.next().await {
        if meta?.location.filename().is_some_and(is_valid_hash) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn blob_read(res: object_store::GetResult) -> BlobRead {
    let total_size = res.meta.size;
    let range = res.range.clone();
    BlobRead { total_size, range, stream: res.into_stream() }
}

async fn write_file(store: &Arc<dyn ObjectStore>, path: &FsPath, to: &Path) -> Result<()> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut writer = BufWriter::with_capacity(Arc::clone(store), to.clone(), 16 << 20);
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).await?;
    }
    writer.shutdown().await.context("finish blob upload")?;
    Ok(())
}

pub(crate) async fn exists_in(store: &dyn ObjectStore, hash: &str) -> Result<bool> {
    match store.head(&key(hash)).await {
        Ok(_) => Ok(true),
        Err(object_store::Error::NotFound { .. }) => Ok(false),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use clap::Parser;

    use super::*;
    use crate::locations::{self, NewLocation};

    pub(crate) fn setup(dir: &FsPath) -> (Config, db::Pool) {
        let arg = |p: &str| dir.join(p).to_str().unwrap().to_owned();
        let cfg = Config::parse_from(["hoardd", "--data-dir", &arg("data"), "--cache-dir", &arg("cache")]);
        std::fs::create_dir_all(&cfg.data_dir).unwrap();
        let pool = db::open(&cfg.db_path(), &cfg.backups_dir()).unwrap();
        (cfg, pool)
    }

    pub(crate) fn add(pool: &db::Pool, dir: &FsPath, name: &str, kinds: &[&str], catch_all: bool) -> i64 {
        let path = dir.join(name);
        std::fs::create_dir_all(&path).unwrap();
        let new = NewLocation {
            name: Some(name.to_owned()),
            mode: Mode::Managed,
            path: path.to_str().unwrap().to_owned(),
            kinds: kinds.iter().map(|k| (*k).to_owned()).collect(),
            catch_all,
        };
        locations::insert(&pool.get().unwrap(), &new).unwrap().id
    }

    pub(crate) fn on_disk(dir: &FsPath, root: &str, h: &str) -> bool {
        dir.join(root).join(&h[0..2]).join(&h[2..4]).join(h).is_file()
    }

    #[tokio::test]
    async fn files_go_to_the_location_for_their_kind_and_thumbnails_to_the_cache() {
        let dir = tempfile::tempdir().unwrap();
        let (cfg, pool) = setup(dir.path());
        let blobs = BlobStore::open(&cfg, pool.clone()).unwrap();
        assert!(!blobs.can_store());
        let nowhere = blobs.put_bytes(b"nowhere".to_vec(), AssetKind::Audio).await.unwrap();
        assert!(on_disk(dir.path(), "cache/thumbs", &nowhere.hash));

        add(&pool, dir.path(), "sounds", &["audio"], false);
        add(&pool, dir.path(), "everything", &[], true);
        blobs.reload().unwrap();
        assert!(blobs.can_store());
        let sound = blobs.put_bytes(b"a sound".to_vec(), AssetKind::Audio).await.unwrap();
        let model = blobs.put_bytes(b"a model".to_vec(), AssetKind::Model).await.unwrap();
        let thumb = blobs.put_cache_bytes(b"a thumbnail".to_vec()).await.unwrap();
        assert!(on_disk(dir.path(), "sounds", &sound.hash) && !on_disk(dir.path(), "everything", &sound.hash));
        assert!(on_disk(dir.path(), "everything", &model.hash) && !on_disk(dir.path(), "sounds", &model.hash));
        assert!(on_disk(dir.path(), "cache/thumbs", &thumb.hash));

        // Already stored somewhere: not stored again where its kind would go now.
        let again = blobs.put_bytes(b"a model".to_vec(), AssetKind::Audio).await.unwrap();
        assert!(!on_disk(dir.path(), "sounds", &again.hash));

        // Reads find each, wherever it is.
        assert_eq!(&blobs.read_all(&sound.hash).await.unwrap()[..], b"a sound");
        assert_eq!(&blobs.read_all(&model.hash).await.unwrap()[..], b"a model");
        assert_eq!(&blobs.read_all(&thumb.hash).await.unwrap()[..], b"a thumbnail");
        assert_eq!(blobs.verify(&model.hash).await.unwrap(), Some(true));

        blobs.delete(&thumb.hash).await.unwrap();
        blobs.delete(&sound.hash).await.unwrap();
        assert_eq!(blobs.verify(&thumb.hash).await.unwrap(), None);
        assert_eq!(blobs.verify(&sound.hash).await.unwrap(), None);
    }
}
