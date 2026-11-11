//! Content-addressed, write-once blob store over any `object_store` backend.
//!
//! Keys look like `ab/cd/abcd…(64 hex)` where the name is the BLAKE3 hash of the bytes, so:
//! - identical files across packs are stored once,
//! - corruption is detectable (re-hash and compare: see `verify` and the scrub job),
//! - blobs never change, so CDN / browser caches can keep them forever, and backups are a plain sync.
//!
//! Backends:
//! - `local`: a directory on this machine.
//! - `s3`:    any S3-compatible server. The intended setup is a bucket on a home NAS (Garage, `RustFS`,
//!   `SeaweedFS`, `VersityGW`), reached over Tailscale or a Cloudflare Tunnel, while this server runs wherever.

use std::fmt;
use std::path::Path as FsPath;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use bytes::Bytes;
use futures_util::StreamExt;
use futures_util::stream::BoxStream;
use object_store::aws::AmazonS3Builder;
use object_store::buffered::BufWriter;
use object_store::local::LocalFileSystem;
use object_store::path::Path;
use object_store::{GetOptions, GetRange, ObjectStore, ObjectStoreExt, PutPayload};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::config::{Config, StorageKind};

#[derive(Clone)]
pub(crate) struct BlobStore {
    store: Arc<dyn ObjectStore>,
    label: String,
}

impl fmt::Debug for BlobStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlobStore").field("backend", &self.label).finish_non_exhaustive()
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

fn key(hash: &str) -> Path {
    Path::from(format!("{}/{}/{hash}", &hash[0..2], &hash[2..4]))
}

impl BlobStore {
    pub(crate) fn from_config(cfg: &Config) -> Result<Self> {
        let (store, label): (Arc<dyn ObjectStore>, String) = match cfg.storage {
            StorageKind::Local => {
                let dir = cfg.blobs_dir();
                std::fs::create_dir_all(&dir)?;
                (Arc::new(LocalFileSystem::new_with_prefix(&dir)?), format!("local:{}", dir.display()))
            }
            StorageKind::S3 => {
                let bucket = cfg.s3_bucket.as_deref().context("HOARD_S3_BUCKET is required for s3 storage")?;
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
                        let (name, value) =
                            h.split_once('=').context("HOARD_S3_HEADERS entries look like Name=value")?;
                        headers.insert(
                            http::HeaderName::from_bytes(name.trim().as_bytes())?,
                            http::HeaderValue::from_str(value.trim())?,
                        );
                    }
                    b = b.with_client_options(object_store::ClientOptions::new().with_default_headers(headers));
                }
                let label = format!("s3:{}/{bucket}", cfg.s3_endpoint.as_deref().unwrap_or("aws"));
                (Arc::new(b.build()?), label)
            }
        };
        Ok(Self { store, label })
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    /// Hash a local file without storing it.
    pub(crate) fn hash_file(path: &FsPath) -> Result<StoredBlob> {
        let mut hasher = blake3::Hasher::new();
        let file = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
        let size = std::io::copy(&mut std::io::BufReader::with_capacity(1 << 20, file), &mut hasher)?;
        Ok(StoredBlob { hash: hasher.finalize().to_hex().to_string(), size })
    }

    pub(crate) async fn exists(&self, hash: &str) -> Result<bool> {
        match self.store.head(&key(hash)).await {
            Ok(_) => Ok(true),
            Err(object_store::Error::NotFound { .. }) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    /// Upload a local file (streamed, multipart for big files). Skips the upload if the blob already exists,
    /// which is what makes re-uploading the same pack cheap.
    pub(crate) async fn put_file(&self, path: &FsPath, blob: &StoredBlob) -> Result<()> {
        if self.exists(&blob.hash).await? {
            return Ok(());
        }
        let mut file = tokio::fs::File::open(path).await?;
        let mut writer = BufWriter::with_capacity(Arc::clone(&self.store), key(&blob.hash), 16 << 20);
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

    pub(crate) async fn put_bytes(&self, bytes: Vec<u8>) -> Result<StoredBlob> {
        let hash = blake3::hash(&bytes).to_hex().to_string();
        let size = bytes.len() as u64;
        if !self.exists(&hash).await? {
            self.store.put(&key(&hash), PutPayload::from(bytes)).await?;
        }
        Ok(StoredBlob { hash, size })
    }

    /// Open a blob for streaming, optionally a byte range (for HTTP `Range`, so audio/video can seek).
    pub(crate) async fn read(&self, hash: &str, range: Option<GetRange>) -> Result<Option<BlobRead>> {
        let opts = GetOptions { range, ..Default::default() };
        match self.store.get_opts(&key(hash), opts).await {
            Ok(res) => {
                let total_size = res.meta.size;
                let range = res.range.clone();
                Ok(Some(BlobRead { total_size, range, stream: res.into_stream() }))
            }
            Err(object_store::Error::NotFound { .. }) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn read_all(&self, hash: &str) -> Result<Bytes> {
        Ok(self.store.get(&key(hash)).await?.bytes().await?)
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

    pub(crate) async fn delete(&self, hash: &str) -> Result<()> {
        match self.store.delete(&key(hash)).await {
            Ok(()) | Err(object_store::Error::NotFound { .. }) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
