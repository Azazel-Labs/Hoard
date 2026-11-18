//! Collections.

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Row};

const COLLECTION_COLS: &str = "c.id, c.name, c.description, c.created_at, c.updated_at,
    (SELECT COUNT(*) FROM collection_assets ca WHERE ca.collection_id = c.id),
    (SELECT COALESCE(SUM(b.size), 0) FROM collection_assets ca JOIN assets a ON a.id = ca.asset_id
       JOIN blobs b ON b.hash = a.blob_hash WHERE ca.collection_id = c.id)";

fn collection_from_row(r: &Row<'_>) -> rusqlite::Result<hoard_core::Collection> {
    Ok(hoard_core::Collection {
        id: r.get(0)?,
        name: r.get(1)?,
        description: r.get(2)?,
        created_at: r.get(3)?,
        updated_at: r.get(4)?,
        asset_count: r.get::<_, i64>(5)? as u64,
        total_bytes: r.get::<_, i64>(6)? as u64,
        preview: Vec::new(),
        preview_kinds: Vec::new(),
    })
}

fn collection_preview(conn: &Connection, c: &mut hoard_core::Collection) -> Result<()> {
    let mut stmt = conn.prepare_cached(
        "SELECT a.id, a.kind FROM collection_assets ca JOIN assets a ON a.id = ca.asset_id
         WHERE ca.collection_id = ?1 AND a.thumb_hash IS NOT NULL ORDER BY ca.added_at DESC LIMIT 4",
    )?;
    let cover: Vec<(String, String)> =
        stmt.query_map([&c.id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
    (c.preview, c.preview_kinds) = cover.into_iter().unzip();
    Ok(())
}

pub(crate) fn list_collections(conn: &Connection) -> Result<Vec<hoard_core::Collection>> {
    let mut stmt =
        conn.prepare(&format!("SELECT {COLLECTION_COLS} FROM collections c ORDER BY c.name COLLATE NOCASE"))?;
    let mut rows = stmt.query_map([], collection_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
    for c in &mut rows {
        collection_preview(conn, c)?;
    }
    Ok(rows)
}

/// A collection by id or (case-insensitive) name.
pub(crate) fn get_collection(conn: &Connection, key: &str) -> Result<Option<hoard_core::Collection>> {
    let found = conn
        .query_row(
            &format!("SELECT {COLLECTION_COLS} FROM collections c WHERE c.id = ?1 OR c.name = ?1"),
            [key],
            collection_from_row,
        )
        .optional()?;
    let Some(mut c) = found else { return Ok(None) };
    collection_preview(conn, &mut c)?;
    Ok(Some(c))
}

pub(crate) fn collection_asset_ids(conn: &Connection, id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT ca.asset_id FROM collection_assets ca JOIN assets a ON a.id = ca.asset_id
         WHERE ca.collection_id = ?1 ORDER BY a.path",
    )?;
    Ok(stmt.query_map([id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?)
}
