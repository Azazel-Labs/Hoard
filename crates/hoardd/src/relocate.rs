//! Moving stored files between managed locations: after a location's kinds change, so what's stored already follows
//! the new routing, or to empty a location before removing it. Each file is copied, checked against its hash, and only
//! then deleted from where it was, so it can always be read from one place or the other.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use futures_util::{StreamExt, TryStreamExt};
use hoard_core::AssetKind;
use object_store::buffered::BufWriter;
use object_store::{ObjectStore, ObjectStoreExt};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::AppState;
use crate::blobs::{BlobStore, exists_in, holds_any, is_valid_hash, key};
use crate::{activity, db, jobs};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Request {
    /// Empty this location (everything in it moves to where it would go without it). None moves every stored file
    /// to the location its kind is routed to now.
    #[serde(default)]
    pub(crate) from: Option<i64>,
    /// Remove the location once it's empty.
    #[serde(default)]
    pub(crate) remove: bool,
}

/// What each stored file is, so it can be routed the way it was when it was stored: the kind of the asset it is;
/// for a derivative, a converted model, a recovered source file (by its type) or a "download as" copy (by its
/// asset's type); or the original an asset was converted from.
fn kinds(c: &rusqlite::Connection) -> Result<HashMap<String, AssetKind>> {
    let by_name: HashMap<&str, AssetKind> = AssetKind::ALL.iter().map(|k| (k.as_str(), *k)).collect();
    let mut out = HashMap::new();
    let mut stmt = c.prepare("SELECT blob_hash, kind FROM assets")?;
    for row in stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))? {
        let (hash, kind) = row?;
        out.entry(hash).or_insert_with(|| by_name.get(kind.as_str()).copied().unwrap_or(AssetKind::Other));
    }
    let mut stmt = c.prepare(
        "SELECT d.blob_hash, d.kind, a.ext, json_extract(a.meta, '$.source_ext')
           FROM derivatives d JOIN assets a ON a.id = d.asset_id",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, Option<String>>(3)?))
    })?;
    for row in rows {
        let (hash, derivative, ext, source_ext) = row?;
        let kind = match (derivative.as_str(), source_ext) {
            ("glb", _) => AssetKind::Model,
            ("source", Some(src)) => AssetKind::from_ext(&src),
            _ => AssetKind::from_ext(&ext),
        };
        out.entry(hash).or_insert(kind);
    }
    let mut stmt = c.prepare("SELECT blob_hash, ext FROM asset_origin WHERE blob_hash IS NOT NULL")?;
    for row in stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))? {
        let (hash, ext) = row?;
        out.entry(hash).or_insert_with(|| AssetKind::from_ext(&ext));
    }
    Ok(out)
}

struct Move {
    hash: String,
    from: Arc<dyn ObjectStore>,
    to: Arc<dyn ObjectStore>,
}

pub(crate) async fn run(state: &AppState, req: Request) -> Result<()> {
    relocate(&state.blobs, &state.pool, req).await
}

async fn relocate(blobs: &BlobStore, pool: &db::Pool, req: Request) -> Result<()> {
    let stores = blobs.managed_stores();
    let sources: Vec<(i64, Arc<dyn ObjectStore>)> = match req.from {
        Some(id) => vec![
            stores
                .iter()
                .find(|(i, _)| *i == id)
                .cloned()
                .context("that location can't be reached right now, so its files can't be moved")?,
        ],
        None => stores.clone(),
    };
    if req.from.is_some() && stores.len() < 2 {
        bail!("there's no other library location to move the files to: add one first");
    }
    // From here on, new uploads go elsewhere, so the location can't fill up again behind the move; uploads that
    // were already on their way there land before it's looked through.
    let _draining = req.from.map(|id| blobs.drain(id));
    if let Some(id) = req.from {
        blobs.writes_done(id).await;
    }
    let kinds = db::call(pool, |c| kinds(c)).await?;

    activity::job_note(jobs::RELOCATE, None, "Looking through the stored files");
    let mut plan = Vec::new();
    for (id, store) in &sources {
        let keys: Vec<_> = store.list(None).map_ok(|m| m.location).try_collect().await?;
        for k in keys {
            let Some(hash) = k.filename().filter(|h| is_valid_hash(h)) else { continue };
            // A file nothing refers to stays where it is when rebalancing; emptying a location moves everything.
            let kind = match (kinds.get(hash), req.from) {
                (Some(kind), _) => *kind,
                (None, Some(_)) => AssetKind::Other,
                (None, None) => continue,
            };
            match blobs.route(kind, req.from) {
                Some((dest, to)) if dest != *id => {
                    plan.push(Move { hash: hash.to_owned(), from: Arc::clone(store), to });
                }
                _ => {}
            }
        }
    }

    let total = plan.len();
    let (mut moved, mut damaged) = (0usize, 0usize);
    for (n, m) in plan.into_iter().enumerate() {
        activity::job_progress(jobs::RELOCATE, None, n, total, "files");
        if move_blob(&m).await? {
            moved += 1;
        } else {
            tracing::warn!(hash = %m.hash, "a stored file doesn't match its hash; left where it is (Check storage will flag it)");
            damaged += 1;
        }
    }
    tracing::info!(moved, damaged, "stored files moved between library locations");

    if let (Some(id), true) = (req.from, req.remove) {
        let (_, store) = &sources[0];
        if holds_any(&**store).await? {
            bail!(
                "moved {moved} files, but some are still there{}: the location wasn't removed",
                if damaged > 0 { format!(" ({damaged} didn't match their hash)") } else { String::new() }
            );
        }
        db::call(pool, move |c| crate::locations::remove(c, id)).await?;
        blobs.refresh().await?;
        tracing::info!(location = id, "emptied library location removed");
    }
    Ok(())
}

/// Copy one file to its new location, check it, then delete the old copy. False if the file doesn't match its hash,
/// in which case nothing is deleted.
async fn move_blob(m: &Move) -> Result<bool> {
    let path = key(&m.hash);
    if !exists_in(&*m.to, &m.hash).await? {
        let mut src = match m.from.get(&path).await {
            Ok(r) => r.into_stream(),
            // Gone since the listing (deleted with its pack): nothing to move.
            Err(object_store::Error::NotFound { .. }) => return Ok(true),
            Err(e) => return Err(e.into()),
        };
        let mut hasher = blake3::Hasher::new();
        let mut out = BufWriter::with_capacity(Arc::clone(&m.to), path.clone(), 16 << 20);
        while let Some(chunk) = src.next().await {
            let chunk = chunk?;
            hasher.update(&chunk);
            out.write_all(&chunk).await?;
        }
        out.shutdown().await.context("finish writing the moved file")?;
        if hasher.finalize().to_hex().as_str() != m.hash {
            m.to.delete(&path).await.ok();
            return Ok(false);
        }
    }
    match m.from.delete(&path).await {
        Ok(()) | Err(object_store::Error::NotFound { .. }) => Ok(true),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blobs::tests::{add, on_disk, setup};

    #[tokio::test]
    async fn files_follow_new_routing_and_an_emptied_location_goes() {
        let dir = tempfile::tempdir().unwrap();
        let (cfg, pool) = setup(dir.path());
        let lib = add(&pool, dir.path(), "lib", &[], true);
        let blobs = BlobStore::open(&cfg, pool.clone()).unwrap();
        let sound = blobs.put_bytes(b"a sound".to_vec(), AssetKind::Audio).await.unwrap().hash;
        let model = blobs.put_bytes(b"a model".to_vec(), AssetKind::Model).await.unwrap().hash;
        {
            let c = pool.get().unwrap();
            c.execute("INSERT INTO packs (id, name, status, source, files_total, created_at) VALUES ('p', 'p', 'ready', 'upload', 2, '')", []).unwrap();
            for (id, h, kind) in [("a1", &sound, "audio"), ("a2", &model, "model")] {
                c.execute("INSERT OR IGNORE INTO blobs (hash, size, created_at) VALUES (?1, 1, '')", [h]).unwrap();
                c.execute(
                    "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
                     VALUES (?1, 'p', ?2, ?1, ?1, 'x', ?3, '', '')",
                    rusqlite::params![id, h, kind],
                )
                .unwrap();
            }
        }
        // Music gets its own disk afterwards: the sound moves there, the model stays.
        let music = add(&pool, dir.path(), "music", &["audio"], false);
        blobs.reload().unwrap();
        relocate(&blobs, &pool, Request::default()).await.unwrap();
        assert!(on_disk(dir.path(), "music", &sound) && !on_disk(dir.path(), "lib", &sound));
        assert!(on_disk(dir.path(), "lib", &model));

        // Emptying the music disk sends the sound back to the catch-all and removes the location.
        relocate(&blobs, &pool, Request { from: Some(music), remove: true }).await.unwrap();
        assert!(on_disk(dir.path(), "lib", &sound) && !on_disk(dir.path(), "music", &sound));
        let left: Vec<i64> = crate::locations::list(&pool.get().unwrap()).unwrap().iter().map(|l| l.id).collect();
        assert_eq!(left, [lib]);
        assert_eq!(blobs.read_all(&sound).await.unwrap().as_ref(), b"a sound");
    }
}
