//! Keeping what was read: the files and their previews put in the blob store, and the assets, their tags,
//! READMEs and derived files written to the database.

use std::collections::HashMap;

use anyhow::Result;
use futures_util::{StreamExt, TryStreamExt, stream};
use hoard_core::AssetKind;
use rusqlite::params;
use tokio::runtime::Handle;

use super::Item;
use crate::AppState;
use crate::blobs::{BlobStore, StoredBlob};
use crate::db;
use crate::tagger::{self, TagSet};
use crate::{context, links};

/// Previews stored alongside an asset: its thumbnail and (for models, when vision tagging is on) a multi-view sheet.
pub(super) struct Previews {
    pub(super) thumb: Option<StoredBlob>,
    pub(super) views: Option<StoredBlob>,
    /// A source file recovered from inside the asset (Unreal `SoundWave` → WAV).
    pub(super) source: Option<StoredBlob>,
    /// A playable rendering (MIDI, tracker modules → MP3).
    pub(super) preview: Option<StoredBlob>,
    /// A light preview of a big image.
    pub(super) lite: Option<StoredBlob>,
}

/// Upload a batch's originals and previews to the blob store. The preview bytes are moved out of the items, not
/// copied. Returns the previews by item index.
/// A watched folder's files stay where they are (`store_originals` false); only their previews are stored.
pub(super) fn upload_all(
    state: &AppState,
    rt: &Handle,
    items: &mut [Item],
    store_originals: bool,
) -> Result<Vec<Previews>> {
    let blobs = &state.blobs;
    rt.block_on(async {
        if store_originals {
            stream::iter(items.iter().map(Ok::<_, anyhow::Error>))
                .try_for_each_concurrent(8, |it| async move { blobs.put_file(&it.local, &it.blob, it.kind).await })
                .await?;
        }
        stream::iter(items.iter_mut())
            .map(|it| async move {
                let a = &mut it.analysis;
                // Thumbnails, views and previews go to the cache store; a recovered source file is content.
                Ok::<_, anyhow::Error>(Previews {
                    thumb: cached(blobs, a.thumb.take()).await?,
                    views: cached(blobs, a.views.take()).await?,
                    source: match a.source.take() {
                        Some(s) => Some(blobs.put_bytes(s.bytes, AssetKind::from_ext(s.ext)).await?),
                        None => None,
                    },
                    preview: cached(blobs, a.preview_mp3.take()).await?,
                    lite: cached(blobs, a.lite.take()).await?,
                })
            })
            .buffered(8)
            .try_collect::<Vec<_>>()
            .await
    })
}

async fn cached(blobs: &BlobStore, bytes: Option<Vec<u8>>) -> Result<Option<StoredBlob>> {
    Ok(match bytes {
        Some(b) => Some(blobs.put_cache_bytes(b).await?),
        None => None,
    })
}

pub(super) fn insert_blob(tx: &rusqlite::Transaction<'_>, blob: &StoredBlob, now: &str) -> Result<()> {
    tx.execute(
        "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT (hash) DO NOTHING",
        params![blob.hash, blob.size as i64, now],
    )?;
    Ok(())
}

/// Keep README text as pack context; a store link inside it fills in the store if none was set.
pub(super) fn save_readme(tx: &rusqlite::Transaction<'_>, pack_id: &str, text: &str) -> Result<()> {
    let (store, url) = context::store_link(text).map_or((None, None), |(s, u)| (Some(s), Some(u)));
    tx.execute(
        "INSERT INTO pack_context (pack_id, readme, store, store_url) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT (pack_id) DO UPDATE SET readme = excluded.readme,
             store = COALESCE(store, excluded.store), store_url = COALESCE(store_url, excluded.store_url)",
        params![pack_id, text, store, url],
    )?;
    Ok(())
}

pub(super) fn store_derivative(
    tx: &rusqlite::Transaction<'_>,
    asset_id: &str,
    kind: &str,
    blob: Option<&StoredBlob>,
    now: &str,
) -> Result<()> {
    if let Some(v) = blob {
        insert_blob(tx, v, now)?;
        tx.execute(
            "INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES (?1, ?4, ?2, ?3)
             ON CONFLICT (asset_id, kind) DO UPDATE SET blob_hash = excluded.blob_hash, created_at = excluded.created_at",
            params![asset_id, v.hash, now, kind],
        )?;
    }
    Ok(())
}

pub(super) fn write_tags(tx: &rusqlite::Transaction<'_>, asset_id: &str, tags: TagSet) -> Result<()> {
    for s in tags.into_vec() {
        db::upsert_tag(tx, asset_id, &s.tag, s.source, s.confidence)?;
    }
    Ok(())
}

/// Insert one analyzed file. A path the pack already has (written by an earlier run that was interrupted) is left
/// as it is, so an ingest can be run again and carry on.
pub(super) fn insert_asset(
    tx: &rusqlite::Transaction<'_>,
    pack_id: &str,
    item: Item,
    previews: Previews,
    now: &str,
    refs: &mut HashMap<String, Vec<links::Ref>>,
) -> Result<()> {
    let Previews { thumb, views, source, preview, lite } = previews;
    insert_blob(tx, &item.blob, now)?;
    if let Some(t) = &thumb {
        insert_blob(tx, t, now)?;
    }
    let id = uuid::Uuid::now_v7().to_string();
    let added = tx.execute(
        "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, category, meta, words, thumb_hash,
                             created_at, updated_at, source_description)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12, ?13)
         ON CONFLICT (pack_id, path) DO NOTHING",
        params![
            id,
            pack_id,
            item.blob.hash,
            item.rel,
            tagger::asset_name(item.analysis.title.as_deref(), &item.rel),
            item.ext,
            item.kind.as_str(),
            item.category,
            serde_json::Value::Object(item.analysis.meta).to_string(),
            tagger::fts_words(&item.rel),
            thumb.as_ref().map(|t| t.hash.clone()),
            now,
            item.analysis.description,
        ],
    )?;
    if added == 0 {
        return Ok(());
    }
    write_tags(tx, &id, item.tags)?;
    db::reindex_asset(tx, &id)?;
    if let Some(rig) = &item.analysis.rig {
        crate::animations::store(tx, &id, Some(rig))?;
    }
    store_derivative(tx, &id, "views", views.as_ref(), now)?;
    store_derivative(tx, &id, "source", source.as_ref(), now)?;
    store_derivative(tx, &id, "preview", preview.as_ref(), now)?;
    store_derivative(tx, &id, crate::previews::LITE, lite.as_ref(), now)?;
    if !item.refs.is_empty() {
        refs.insert(id, item.refs);
    }
    Ok(())
}
