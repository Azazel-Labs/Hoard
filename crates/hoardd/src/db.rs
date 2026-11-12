//! SQLite (WAL) metadata store with FTS5 search.
//!
//! The database only holds metadata. File bytes live in the blob store, so the DB stays small and
//! can be snapshotted with `VACUUM INTO` at any time (see the backup endpoint).

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use hoard_core::{Asset, AssetKind, AssetLink, Facet, LinkRel, Pack, PackStatus, SearchQuery, Tag, TagSource};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::Value as SqlValue;
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};

pub(crate) type Pool = r2d2::Pool<SqliteConnectionManager>;
pub(crate) type Conn = r2d2::PooledConnection<SqliteConnectionManager>;

/// Schema version of [`BASELINE`]: the schema as of the first published release (0.8.0).
const BASELINE_VERSION: i64 = 8;

/// Changes since the baseline, oldest first. Step `i` takes the database from `BASELINE_VERSION + i` to
/// `BASELINE_VERSION + i + 1`, inside one transaction, so a failing step leaves the database untouched.
///
/// **Append only.** Never edit, reorder or remove a step once it has shipped: databases out there have run it.
/// And don't change [`BASELINE`] either; a new table, column or index is a new step here. A step can be any SQL
/// SQLite accepts in a transaction, e.g. `ALTER TABLE packs ADD COLUMN notes TEXT;` or a table rebuild.
const MIGRATIONS: &[&str] = &[
    // 9: settings changed from the portal (see `settings.rs`).
    "CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL) WITHOUT ROWID;",
    // 10: the AI tagger's tags are source `ai`, not `llm`.
    "UPDATE asset_tags SET source = 'ai' WHERE source = 'llm';",
    // 11: descriptions by where they come from. `description` becomes a person's alone; what was there and not
    // edited by hand moves to the AI's (if the AI tagged that asset, since it wrote the description then) or the
    // file's own.
    "ALTER TABLE assets ADD COLUMN source_description TEXT;
     ALTER TABLE assets ADD COLUMN ai_description TEXT;
     UPDATE assets SET ai_description = description, description = NULL
      WHERE human_edited = 0 AND description IS NOT NULL
        AND EXISTS (SELECT 1 FROM asset_tags t WHERE t.asset_id = assets.id AND t.source = 'ai');
     UPDATE assets SET source_description = description, description = NULL
      WHERE human_edited = 0 AND description IS NOT NULL;",
    // 12: the archive. An archived pack or asset is left out of searches until it's restored or deleted for good.
    "ALTER TABLE packs ADD COLUMN archived_at TEXT;
     ALTER TABLE assets ADD COLUMN archived_at TEXT;
     CREATE INDEX assets_archived ON assets(archived_at) WHERE archived_at IS NOT NULL;",
];

/// How many automatic pre-upgrade backups to keep in `<data dir>/backups`.
const KEEP_UPGRADE_BACKUPS: usize = 5;

/// Frozen: see [`MIGRATIONS`]. Every statement is `IF NOT EXISTS`, which is how databases from before the baseline
/// (development builds) are brought up to it.
const BASELINE: &str = "
CREATE TABLE IF NOT EXISTS blobs (
    hash        TEXT PRIMARY KEY,
    size        INTEGER NOT NULL,
    status      TEXT NOT NULL DEFAULT 'ok',      -- ok | corrupt | missing
    created_at  TEXT NOT NULL,
    verified_at TEXT
);

CREATE TABLE IF NOT EXISTS packs (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    status      TEXT NOT NULL,
    source      TEXT,
    license     TEXT,
    description TEXT,
    tags        TEXT NOT NULL DEFAULT '[]',
    error       TEXT,
    files_done  INTEGER NOT NULL DEFAULT 0,
    files_total INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assets (
    id           TEXT PRIMARY KEY,
    pack_id      TEXT NOT NULL REFERENCES packs(id) ON DELETE CASCADE,
    blob_hash    TEXT NOT NULL REFERENCES blobs(hash),
    path         TEXT NOT NULL,
    name         TEXT NOT NULL,
    ext          TEXT NOT NULL,
    kind         TEXT NOT NULL,
    category     TEXT,
    description  TEXT,
    meta         TEXT NOT NULL DEFAULT '{}',
    words        TEXT NOT NULL DEFAULT '',        -- tokenized path words for FTS
    thumb_hash   TEXT,
    human_edited INTEGER NOT NULL DEFAULT 0,      -- 1 = AI must not overwrite name/category
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL,
    UNIQUE (pack_id, path)
);
CREATE INDEX IF NOT EXISTS assets_kind ON assets(kind);
CREATE INDEX IF NOT EXISTS assets_pack ON assets(pack_id);
CREATE INDEX IF NOT EXISTS assets_blob ON assets(blob_hash);
CREATE INDEX IF NOT EXISTS assets_thumb ON assets(thumb_hash);

CREATE TABLE IF NOT EXISTS asset_tags (
    asset_id   TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    tag        TEXT NOT NULL,
    source     TEXT NOT NULL,
    confidence REAL NOT NULL DEFAULT 1.0,
    PRIMARY KEY (asset_id, tag)
) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS asset_tags_tag ON asset_tags(tag);

CREATE VIRTUAL TABLE IF NOT EXISTS assets_fts USING fts5(
    asset_id UNINDEXED, name, words, category, description, tags,
    tokenize = 'unicode61 remove_diacritics 2'
);

CREATE TABLE IF NOT EXISTS jobs (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    kind       TEXT NOT NULL,
    pack_id    TEXT,
    payload    TEXT NOT NULL DEFAULT '{}',
    status     TEXT NOT NULL DEFAULT 'queued',   -- queued | running | done | failed
    attempts   INTEGER NOT NULL DEFAULT 0,
    error      TEXT,
    run_after  TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS jobs_queue ON jobs(status, run_after);

-- v2: dependency links between assets, and server-made conversions (e.g. .blend -> .glb).
CREATE TABLE IF NOT EXISTS asset_links (
    from_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    to_id   TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    rel     TEXT NOT NULL,                      -- texture | buffer | material | animation | guess
    PRIMARY KEY (from_id, to_id, rel)
) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS asset_links_to ON asset_links(to_id);

CREATE TABLE IF NOT EXISTS derivatives (
    asset_id   TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    kind       TEXT NOT NULL,                   -- glb
    blob_hash  TEXT NOT NULL REFERENCES blobs(hash),
    created_at TEXT NOT NULL,
    PRIMARY KEY (asset_id, kind)
) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS derivatives_blob ON derivatives(blob_hash);

-- v3: resumable upload sessions (a pack in status 'uploading' plus its expected files).
CREATE TABLE IF NOT EXISTS upload_files (
    pack_id  TEXT NOT NULL REFERENCES packs(id) ON DELETE CASCADE,
    path     TEXT NOT NULL,
    size     INTEGER NOT NULL,
    received INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (pack_id, path)
) WITHOUT ROWID;
-- v4: fingerprint of each import's file list (paths + sizes), to resume or refuse duplicate imports.
CREATE TABLE IF NOT EXISTS import_keys (
    import_key TEXT NOT NULL,
    pack_id    TEXT NOT NULL REFERENCES packs(id) ON DELETE CASCADE,
    PRIMARY KEY (import_key, pack_id)
) WITHOUT ROWID;
-- v5: what the seller says about a pack, and variant families (whoosh_01..whoosh_12).
CREATE TABLE IF NOT EXISTS pack_context (
    pack_id      TEXT PRIMARY KEY REFERENCES packs(id) ON DELETE CASCADE,
    store        TEXT,
    store_url    TEXT,
    store_text   TEXT,
    license_tier TEXT,
    readme       TEXT
);
CREATE TABLE IF NOT EXISTS asset_family (
    asset_id   TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    family_key TEXT NOT NULL
) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS asset_family_key ON asset_family(family_key);
-- v6: collections, hand-picked sets of assets across packs.
CREATE TABLE IF NOT EXISTS collections (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE COLLATE NOCASE,
    description TEXT,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS collection_assets (
    collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    asset_id      TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    added_at      TEXT NOT NULL,
    PRIMARY KEY (collection_id, asset_id)
) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS collection_assets_asset ON collection_assets(asset_id);
-- v7: where converted assets came from (a .uasset replaced by the PNG it held, say), and Unreal packages that
-- still need converting.
CREATE TABLE IF NOT EXISTS asset_origin (
    asset_id     TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    path         TEXT NOT NULL,                 -- path in the pack before conversion
    ext          TEXT NOT NULL,
    blob_hash    TEXT,                          -- the original bytes, if kept
    class        TEXT,
    engine       TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    method       TEXT NOT NULL,                 -- native | editor
    notes        TEXT,
    converted_at TEXT NOT NULL
) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS asset_origin_blob ON asset_origin(blob_hash);
CREATE TABLE IF NOT EXISTS unreal_status (
    asset_id   TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
    status     TEXT NOT NULL,                   -- needs_editor | unsupported | failed
    class      TEXT,
    reason     TEXT,
    updated_at TEXT NOT NULL
) WITHOUT ROWID;
-- v8: Unity GUIDs from the .meta files in a pack, for resolving materials' and prefabs' references.
CREATE TABLE IF NOT EXISTS unity_guids (
    pack_id TEXT NOT NULL REFERENCES packs(id) ON DELETE CASCADE,
    guid    TEXT NOT NULL,
    path    TEXT NOT NULL,
    PRIMARY KEY (pack_id, guid)
) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS upload_meta (
    pack_id    TEXT PRIMARY KEY REFERENCES packs(id) ON DELETE CASCADE,
    auto_name  INTEGER NOT NULL DEFAULT 1,
    touched_at TEXT NOT NULL
);
";

pub(crate) fn now() -> String {
    time::OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap_or_default()
}

/// Open the database, upgrading its schema first if this build is newer (backing it up into `backups` before
/// touching it). Refuses a database written by a newer build rather than guess at a schema it doesn't know.
pub(crate) fn open(path: &Path, backups: &Path) -> Result<Pool> {
    let manager = SqliteConnectionManager::file(path).with_init(|c| {
        // busy_timeout first: switching to WAL takes a lock, and connections opened at once must wait for it
        // rather than fail with "database is locked".
        c.execute_batch(
            "PRAGMA busy_timeout = 10000;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;",
        )
    });
    let pool = r2d2::Pool::builder().max_size(12).build(manager).context("open sqlite pool")?;
    let mut conn = pool.get()?;
    migrate(&mut conn, backups, MIGRATIONS)?;
    Ok(pool)
}

fn user_version(conn: &Connection) -> Result<i64> {
    Ok(conn.query_row("PRAGMA user_version", [], |r| r.get(0))?)
}

fn migrate(conn: &mut Connection, backups: &Path, steps: &[&str]) -> Result<()> {
    let target = BASELINE_VERSION + steps.len() as i64;
    let version = user_version(conn)?;
    if version > target {
        anyhow::bail!(
            "the database is at schema version {version}, but this hoardd {} only knows up to {target}: a newer \
             Hoard has upgraded it. Run that version again, or restore the backup it made before upgrading \
             ({}/hoard-pre-upgrade-v{target}-*.db, or older).",
            hoard_core::VERSION,
            backups.display(),
        );
    }
    if version == target {
        return Ok(());
    }

    let empty: bool =
        conn.query_row("SELECT NOT EXISTS (SELECT 1 FROM sqlite_schema WHERE type = 'table')", [], |r| r.get(0))?;
    if !empty {
        let path = backup_before_upgrade(conn, backups, version)?;
        tracing::info!(from = version, to = target, backup = %path.display(), "upgrading the database schema");
    }

    let step = |conn: &mut Connection, sql: &str, to: i64| -> Result<()> {
        let tx = conn.transaction()?;
        tx.execute_batch(sql).with_context(|| format!("database migration to schema version {to}"))?;
        tx.execute_batch(&format!("PRAGMA user_version = {to}"))?;
        tx.commit()?;
        Ok(())
    };
    if version < BASELINE_VERSION {
        step(conn, BASELINE, BASELINE_VERSION)?;
    }
    for (i, sql) in steps.iter().enumerate() {
        let to = BASELINE_VERSION + i as i64 + 1;
        if to > version {
            step(conn, sql, to)?;
        }
    }
    Ok(())
}

/// Snapshot the database before a schema upgrade, keeping the newest [`KEEP_UPGRADE_BACKUPS`] such snapshots.
fn backup_before_upgrade(conn: &Connection, dir: &Path, from: i64) -> Result<std::path::PathBuf> {
    std::fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
    let stamp = now().replace([':', '-'], "").replace('T', "-");
    let path = dir.join(format!("hoard-pre-upgrade-v{from}-{stamp}.db"));
    conn.execute("VACUUM INTO ?1", [path.to_string_lossy()])
        .with_context(|| format!("back up the database to {} before upgrading it", path.display()))?;

    let mut old: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("hoard-pre-upgrade-")))
        .filter_map(|p| Some((p.metadata().ok()?.modified().ok()?, p)))
        .collect();
    old.sort();
    for (_, p) in old.iter().rev().skip(KEEP_UPGRADE_BACKUPS) {
        if let Err(e) = std::fs::remove_file(p) {
            tracing::warn!(path = %p.display(), error = %e, "could not remove an old pre-upgrade backup");
        }
    }
    Ok(path)
}

/// Run blocking DB work off the async runtime.
pub(crate) async fn call<T, F>(pool: &Pool, f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce(&mut Conn) -> Result<T> + Send + 'static,
{
    let pool = pool.clone();
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get()?;
        f(&mut conn)
    })
    .await?
}

// ---------------------------------------------------------------------------------------------
// Packs

const PACK_COLS: &str = "p.id, p.name, p.status, p.source, p.license, p.description, p.tags, p.error,
    p.files_done, p.files_total, p.created_at,
    (SELECT COUNT(*) FROM assets a WHERE a.pack_id = p.id),
    (SELECT COALESCE(SUM(b.size), 0) FROM assets a JOIN blobs b ON b.hash = a.blob_hash WHERE a.pack_id = p.id),
    (SELECT COUNT(*) FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running')),
    c.store, c.store_url, c.license_tier,
    (SELECT COUNT(*) FROM assets a WHERE a.pack_id = p.id AND a.ext IN ('uasset', 'umap')),
    (SELECT COUNT(*) FROM asset_origin o JOIN assets a ON a.id = o.asset_id WHERE a.pack_id = p.id),
    p.archived_at";

fn pack_from_row(r: &Row<'_>) -> rusqlite::Result<Pack> {
    let tags: String = r.get(6)?;
    Ok(Pack {
        id: r.get(0)?,
        name: r.get(1)?,
        status: PackStatus::parse(&r.get::<_, String>(2)?),
        source: r.get(3)?,
        license: r.get(4)?,
        description: r.get(5)?,
        tags: serde_json::from_str(&tags).unwrap_or_default(),
        error: r.get(7)?,
        files_done: r.get::<_, i64>(8)? as u64,
        files_total: r.get::<_, i64>(9)? as u64,
        created_at: r.get(10)?,
        asset_count: r.get::<_, i64>(11)? as u64,
        total_bytes: r.get::<_, i64>(12)? as u64,
        jobs_pending: r.get::<_, i64>(13)? as u64,
        store: r.get(14)?,
        store_url: r.get(15)?,
        license_tier: r.get(16)?,
        preview: Vec::new(),
        preview_kinds: Vec::new(),
        unreal_packages: r.get::<_, i64>(17)? as u64,
        unreal_converted: r.get::<_, i64>(18)? as u64,
        archived_at: r.get(19)?,
    })
}

/// The packs in the library, newest first, or with `archived` the ones in the archive, most recently archived first.
pub(crate) fn list_packs(conn: &Connection, archived: bool) -> Result<Vec<Pack>> {
    let filter = if archived {
        "WHERE p.archived_at IS NOT NULL ORDER BY p.archived_at DESC"
    } else {
        "WHERE p.archived_at IS NULL ORDER BY p.created_at DESC"
    };
    let mut stmt = conn
        .prepare(&format!("SELECT {PACK_COLS} FROM packs p LEFT JOIN pack_context c ON c.pack_id = p.id {filter}"))?;
    let mut rows = stmt.query_map([], pack_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
    // Cover mosaic: one of each kind first (a model, a texture, a sound…), then more of the same.
    let mut cover = conn.prepare(
        "SELECT id, kind FROM (
           SELECT id, kind, path, ROW_NUMBER() OVER (PARTITION BY kind ORDER BY path) AS n
           FROM assets WHERE pack_id = ?1 AND thumb_hash IS NOT NULL
             AND kind IN ('model', 'texture', 'audio', 'animation', 'font')
         ) ORDER BY n, CASE kind WHEN 'model' THEN 0 WHEN 'texture' THEN 1 WHEN 'font' THEN 3 ELSE 2 END LIMIT 4",
    )?;
    for p in &mut rows {
        let cover: Vec<(String, String)> =
            cover.query_map([&p.id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
        (p.preview, p.preview_kinds) = cover.into_iter().unzip();
    }
    Ok(rows)
}

pub(crate) fn get_pack(conn: &Connection, id: &str) -> Result<Option<Pack>> {
    Ok(conn
        .query_row(
            &format!("SELECT {PACK_COLS} FROM packs p LEFT JOIN pack_context c ON c.pack_id = p.id WHERE p.id = ?1"),
            [id],
            pack_from_row,
        )
        .optional()?)
}

pub(crate) fn set_pack_status(conn: &Connection, id: &str, status: PackStatus, error: Option<&str>) -> Result<()> {
    conn.execute("UPDATE packs SET status = ?2, error = ?3 WHERE id = ?1", params![id, status.as_str(), error])?;
    Ok(())
}

pub(crate) fn delete_pack(conn: &mut Connection, id: &str) -> Result<bool> {
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM assets_fts WHERE asset_id IN (SELECT id FROM assets WHERE pack_id = ?1)", [id])?;
    tx.execute("DELETE FROM jobs WHERE pack_id = ?1 AND status IN ('queued', 'failed')", [id])?;
    let n = tx.execute("DELETE FROM packs WHERE id = ?1", [id])?;
    tx.commit()?;
    Ok(n > 0)
}

// ---------------------------------------------------------------------------------------------
// Archive

/// What an archive request touched.
#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct ArchiveCount {
    pub(crate) packs: u64,
    pub(crate) assets: u64,
}

/// Move packs and assets to the archive, or (`archived` false) back to the library. Only rows not already where
/// they're going count.
pub(crate) fn set_archived(
    conn: &mut Connection,
    packs: &[String],
    assets: &[String],
    archived: bool,
) -> Result<ArchiveCount> {
    let stamp = archived.then(now);
    let from = if archived { "IS NULL" } else { "IS NOT NULL" };
    let tx = conn.transaction()?;
    let mut n = ArchiveCount::default();
    {
        let mut pack =
            tx.prepare(&format!("UPDATE packs SET archived_at = ?2 WHERE id = ?1 AND archived_at {from}"))?;
        for id in packs {
            n.packs += pack.execute(params![id, stamp])? as u64;
        }
        let mut asset =
            tx.prepare(&format!("UPDATE assets SET archived_at = ?2 WHERE id = ?1 AND archived_at {from}"))?;
        for id in assets {
            n.assets += asset.execute(params![id, stamp])? as u64;
        }
    }
    tx.commit()?;
    Ok(n)
}

/// Delete archived packs and assets for good: these ones, or with `everything` the whole archive. Anything not in
/// the archive is left alone. Returns the counts and the ids of the packs that went. Their bytes go on the next
/// blob clean-up (`jobs::gc`).
pub(crate) fn purge_archived(
    conn: &mut Connection,
    packs: &[String],
    assets: &[String],
    everything: bool,
) -> Result<(ArchiveCount, Vec<String>)> {
    let tx = conn.transaction()?;
    let ids = |sql: &str| -> Result<Vec<String>> {
        let mut stmt = tx.prepare(sql)?;
        Ok(stmt.query_map([], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?)
    };
    let (pack_ids, asset_ids) = if everything {
        (
            ids("SELECT id FROM packs WHERE archived_at IS NOT NULL")?,
            ids("SELECT id FROM assets WHERE archived_at IS NOT NULL")?,
        )
    } else {
        (packs.to_vec(), assets.to_vec())
    };
    let mut n = ArchiveCount::default();
    let mut gone = Vec::new();
    for id in &pack_ids {
        let archived: Option<bool> =
            tx.query_row("SELECT archived_at IS NOT NULL FROM packs WHERE id = ?1", [id], |r| r.get(0)).optional()?;
        if archived != Some(true) {
            continue;
        }
        tx.execute("DELETE FROM assets_fts WHERE asset_id IN (SELECT id FROM assets WHERE pack_id = ?1)", [id])?;
        tx.execute("DELETE FROM jobs WHERE pack_id = ?1 AND status IN ('queued', 'failed')", [id])?;
        n.packs += tx.execute("DELETE FROM packs WHERE id = ?1", [id])? as u64;
        gone.push(id.clone());
    }
    for id in &asset_ids {
        tx.execute(
            "DELETE FROM assets_fts WHERE asset_id = ?1 AND EXISTS
               (SELECT 1 FROM assets WHERE id = ?1 AND archived_at IS NOT NULL)",
            [id],
        )?;
        n.assets += tx.execute("DELETE FROM assets WHERE id = ?1 AND archived_at IS NOT NULL", [id])? as u64;
    }
    tx.commit()?;
    Ok((n, gone))
}

// ---------------------------------------------------------------------------------------------
// Assets

const ASSET_COLS: &str = "a.id, a.pack_id, p.name, a.name, a.path, a.ext, a.kind, a.category, a.description,
    a.blob_hash, b.size, a.thumb_hash IS NOT NULL, a.meta, a.created_at, a.updated_at, a.source_description,
    a.ai_description, COALESCE(a.archived_at, p.archived_at)";

fn asset_from_row(r: &Row<'_>) -> rusqlite::Result<Asset> {
    let meta: String = r.get(12)?;
    Ok(Asset {
        id: r.get(0)?,
        pack_id: r.get(1)?,
        pack_name: r.get(2)?,
        name: r.get(3)?,
        path: r.get(4)?,
        ext: r.get(5)?,
        kind: AssetKind::parse(&r.get::<_, String>(6)?).unwrap_or(AssetKind::Other),
        category: r.get(7)?,
        description: r.get(8)?,
        source_description: r.get(15)?,
        ai_description: r.get(16)?,
        hash: r.get(9)?,
        size: r.get::<_, i64>(10)? as u64,
        has_thumb: r.get(11)?,
        meta: serde_json::from_str(&meta).unwrap_or_default(),
        tags: Vec::new(),
        created_at: r.get(13)?,
        updated_at: r.get(14)?,
        archived_at: r.get(17)?,
        links: Vec::new(),
        linked_from: Vec::new(),
        derivatives: Vec::new(),
        family: None,
        duplicates: Vec::new(),
        collections: Vec::new(),
        origin: None,
        unreal: None,
        formats: Vec::new(),
    })
}

fn attach_tags(conn: &Connection, assets: &mut [Asset]) -> Result<()> {
    if assets.is_empty() {
        return Ok(());
    }
    let placeholders = vec!["?"; assets.len()].join(",");
    let mut stmt = conn.prepare(&format!(
        "SELECT asset_id, tag, source, confidence FROM asset_tags WHERE asset_id IN ({placeholders})
         ORDER BY CASE source WHEN 'human' THEN 0 WHEN 'rule' THEN 1 WHEN 'ai' THEN 2 ELSE 3 END, tag"
    ))?;
    let mut by_id: HashMap<String, Vec<Tag>> = HashMap::new();
    let rows = stmt.query_map(params_from_iter(assets.iter().map(|a| a.id.as_str())), |r| {
        Ok((
            r.get::<_, String>(0)?,
            Tag { tag: r.get(1)?, source: TagSource::parse(&r.get::<_, String>(2)?), confidence: r.get(3)? },
        ))
    })?;
    for row in rows {
        let (id, tag) = row?;
        by_id.entry(id).or_default().push(tag);
    }
    for a in assets {
        a.tags = by_id.remove(&a.id).unwrap_or_default();
    }
    Ok(())
}

pub(crate) fn get_asset(conn: &Connection, id: &str) -> Result<Option<Asset>> {
    let asset = conn
        .query_row(
            &format!(
                "SELECT {ASSET_COLS} FROM assets a JOIN packs p ON p.id = a.pack_id
                 JOIN blobs b ON b.hash = a.blob_hash WHERE a.id = ?1"
            ),
            [id],
            asset_from_row,
        )
        .optional()?;
    let Some(asset) = asset else { return Ok(None) };
    let mut one = [asset];
    attach_tags(conn, &mut one)?;
    let [mut asset] = one;
    let (links, linked_from) = asset_links(conn, &asset.id)?;
    asset.links = links;
    asset.linked_from = linked_from;
    // Cached "download as" conversions are an implementation detail, not derivatives to show.
    let mut stmt =
        conn.prepare("SELECT kind FROM derivatives WHERE asset_id = ?1 AND kind NOT LIKE 'as:%' ORDER BY kind")?;
    asset.derivatives = stmt.query_map([&asset.id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    asset.formats = crate::formats::options(&asset.ext, &asset.derivatives);
    asset.family = family(conn, &asset.id)?;
    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.pack_id, p.name, a.path FROM assets a JOIN packs p ON p.id = a.pack_id
         WHERE a.blob_hash = ?1 AND a.id != ?2 ORDER BY p.name, a.path LIMIT 50",
    )?;
    asset.duplicates = stmt
        .query_map(params![asset.hash, asset.id], |r| {
            Ok(hoard_core::Duplicate {
                id: r.get(0)?,
                name: r.get(1)?,
                pack_id: r.get(2)?,
                pack_name: r.get(3)?,
                path: r.get(4)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    let mut stmt = conn.prepare(
        "SELECT c.id, c.name FROM collection_assets ca JOIN collections c ON c.id = ca.collection_id
         WHERE ca.asset_id = ?1 ORDER BY c.name COLLATE NOCASE",
    )?;
    asset.collections = stmt
        .query_map([&asset.id], |r| Ok(hoard_core::CollectionRef { id: r.get(0)?, name: r.get(1)? }))?
        .collect::<rusqlite::Result<_>>()?;
    asset.origin = crate::extract::origin_of(conn, &asset.id)?;
    asset.unreal = crate::extract::status_of(conn, &asset.id)?;
    Ok(Some(asset))
}

/// The variant family an asset belongs to (if any), with up to 60 other members.
pub(crate) fn family(conn: &Connection, id: &str) -> Result<Option<hoard_core::Family>> {
    let Some(key): Option<String> =
        conn.query_row("SELECT family_key FROM asset_family WHERE asset_id = ?1", [id], |r| r.get(0)).optional()?
    else {
        return Ok(None);
    };
    let size: i64 = conn.query_row("SELECT COUNT(*) FROM asset_family WHERE family_key = ?1", [&key], |r| r.get(0))?;
    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.path, a.kind, a.thumb_hash IS NOT NULL FROM asset_family f JOIN assets a ON a.id = f.asset_id
         WHERE f.family_key = ?1 AND a.id != ?2 ORDER BY a.path LIMIT 60",
    )?;
    let members = stmt
        .query_map(params![key, id], |r| {
            Ok(AssetLink {
                id: r.get(0)?,
                name: r.get(1)?,
                path: r.get(2)?,
                kind: AssetKind::parse(&r.get::<_, String>(3)?).unwrap_or(AssetKind::Other),
                rel: LinkRel::Variant,
                has_thumb: r.get(4)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let display = key.split_once('\u{0}').map_or(key.as_str(), |(_, k)| k).to_owned();
    Ok(Some(hoard_core::Family { key: display, size: size as u64, members }))
}

/// Ids of every asset in the same family as `id` (including `id`), or just `id`.
pub(crate) fn family_ids(conn: &Connection, id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT f2.asset_id FROM asset_family f1 JOIN asset_family f2 ON f2.family_key = f1.family_key WHERE f1.asset_id = ?1",
    )?;
    let ids: Vec<String> = stmt.query_map([id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    Ok(if ids.is_empty() { vec![id.to_owned()] } else { ids })
}

pub(crate) fn pack_context(conn: &Connection, pack_id: &str) -> Result<hoard_core::PackContext> {
    Ok(conn
        .query_row(
            "SELECT store, store_url, store_text, license_tier, readme FROM pack_context WHERE pack_id = ?1",
            [pack_id],
            |r| {
                Ok(hoard_core::PackContext {
                    store: r.get(0)?,
                    store_url: r.get(1)?,
                    store_text: r.get(2)?,
                    license_tier: r.get(3)?,
                    readme: r.get(4)?,
                })
            },
        )
        .optional()?
        .unwrap_or_default())
}

// ---------------------------------------------------------------------------------------------
// Collections

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

/// Outgoing and incoming links for one asset.
pub(crate) fn asset_links(conn: &Connection, id: &str) -> Result<(Vec<AssetLink>, Vec<AssetLink>)> {
    let query = |sql: &str| -> Result<Vec<AssetLink>> {
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt
            .query_map([id], |r| {
                Ok(AssetLink {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    path: r.get(2)?,
                    kind: AssetKind::parse(&r.get::<_, String>(3)?).unwrap_or(AssetKind::Other),
                    rel: LinkRel::parse(&r.get::<_, String>(4)?).unwrap_or(LinkRel::Guess),
                    has_thumb: r.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    };
    let out = query(
        "SELECT a.id, a.name, a.path, a.kind, l.rel, a.thumb_hash IS NOT NULL
         FROM asset_links l JOIN assets a ON a.id = l.to_id WHERE l.from_id = ?1 ORDER BY l.rel, a.path",
    )?;
    let inc = query(
        "SELECT a.id, a.name, a.path, a.kind, l.rel, a.thumb_hash IS NOT NULL
         FROM asset_links l JOIN assets a ON a.id = l.from_id WHERE l.to_id = ?1 ORDER BY l.rel, a.path",
    )?;
    Ok((out, inc))
}

/// Everything an asset needs to work, following links transitively (mesh -> material -> textures).
/// The root comes first.
pub(crate) fn dependency_closure(conn: &Connection, root: &str, include_animations: bool) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT to_id, rel FROM asset_links WHERE from_id = ?1")?;
    let mut seen = std::collections::HashSet::from([root.to_owned()]);
    let mut order = vec![root.to_owned()];
    let mut queue = std::collections::VecDeque::from([root.to_owned()]);
    while let Some(id) = queue.pop_front() {
        let next: Vec<(String, String)> =
            stmt.query_map([&id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
        for (to, rel) in next {
            let Some(rel) = LinkRel::parse(&rel) else { continue };
            let wanted = rel.is_dependency() || (include_animations && rel == LinkRel::Animation);
            if wanted && seen.insert(to.clone()) {
                order.push(to.clone());
                queue.push_back(to);
            }
        }
    }
    Ok(order)
}

/// A file to put in a bundle.
#[derive(Debug, Clone)]
pub(crate) struct BundleFile {
    pub(crate) id: String,
    pub(crate) path: String,
    pub(crate) hash: String,
    pub(crate) size: u64,
    /// The file's Unity GUID, when it came with a `.meta`; bundles recreate the `.meta` so Unity keeps references.
    pub(crate) unity_guid: Option<String>,
}

/// Replace files that have a recovered source (`source` derivative) with that source, renamed to its type:
/// `Audio/S_Hit.uasset` becomes `Audio/S_Hit.wav`.
pub(crate) fn prefer_sources(conn: &Connection, files: &mut [BundleFile]) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT d.blob_hash, b.size, json_extract(a.meta, '$.source_ext') FROM derivatives d
         JOIN assets a ON a.id = d.asset_id JOIN blobs b ON b.hash = d.blob_hash
         WHERE d.asset_id = ?1 AND d.kind = 'source'",
    )?;
    for file in files.iter_mut() {
        let found: Option<(String, i64, Option<String>)> =
            stmt.query_row([&file.id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?))).optional()?;
        if let Some((hash, size, Some(ext))) = found {
            let stem = file.path.rsplit_once('.').map_or(file.path.as_str(), |(s, _)| s);
            file.path = format!("{stem}.{ext}");
            file.hash = hash;
            file.size = size as u64;
        }
    }
    Ok(())
}

pub(crate) fn bundle_files(conn: &Connection, ids: &[String]) -> Result<Vec<BundleFile>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.path, a.blob_hash, b.size, u.guid FROM assets a JOIN blobs b ON b.hash = a.blob_hash
         LEFT JOIN unity_guids u ON u.pack_id = a.pack_id AND u.path = a.path WHERE a.id = ?1",
    )?;
    let mut out = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(f) = stmt
            .query_row([id], |r| {
                Ok(BundleFile {
                    id: r.get(0)?,
                    path: r.get(1)?,
                    hash: r.get(2)?,
                    size: r.get::<_, i64>(3)? as u64,
                    unity_guid: r.get(4)?,
                })
            })
            .optional()?
        {
            out.push(f);
        }
    }
    Ok(out)
}

/// Longest shared folder of a set of `/`-separated paths ("" if none).
pub(crate) fn common_dir(paths: &[&str]) -> String {
    let mut dirs = paths.iter().map(|p| p.rsplit_once('/').map_or("", |(d, _)| d).split('/').collect::<Vec<_>>());
    let Some(first) = dirs.next() else { return String::new() };
    let mut common = first;
    for d in dirs {
        let n = common.iter().zip(&d).take_while(|(a, b)| a == b).count();
        common.truncate(n);
    }
    common.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("/")
}

pub(crate) fn strip_dir<'a>(path: &'a str, dir: &str) -> &'a str {
    if dir.is_empty() { path } else { path.strip_prefix(dir).map_or(path, |p| p.trim_start_matches('/')) }
}

pub(crate) fn derivative(conn: &Connection, id: &str, kind: &str) -> Result<Option<String>> {
    Ok(conn
        .query_row("SELECT blob_hash FROM derivatives WHERE asset_id = ?1 AND kind = ?2", params![id, kind], |r| {
            r.get(0)
        })
        .optional()?)
}

/// Look up a file in a pack by its original relative path (case-insensitive fallback). Returns (hash, ext).
pub(crate) fn pack_file(conn: &Connection, pack_id: &str, path: &str) -> Result<Option<(String, String)>> {
    let row = |sql: &str| conn.query_row(sql, params![pack_id, path], |r| Ok((r.get(0)?, r.get(1)?))).optional();
    Ok(match row("SELECT blob_hash, ext FROM assets WHERE pack_id = ?1 AND path = ?2")? {
        Some(r) => Some(r),
        None => row("SELECT blob_hash, ext FROM assets WHERE pack_id = ?1 AND lower(path) = lower(?2)")?,
    })
}

pub(crate) fn asset_file_info(conn: &Connection, id: &str) -> Result<Option<(String, String, String)>> {
    Ok(conn
        .query_row("SELECT blob_hash, path, ext FROM assets WHERE id = ?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .optional()?)
}

pub(crate) fn asset_thumb(conn: &Connection, id: &str) -> Result<Option<String>> {
    Ok(conn.query_row("SELECT thumb_hash FROM assets WHERE id = ?1", [id], |r| r.get(0)).optional()?.flatten())
}

/// Rebuild the FTS row for one asset from its current fields + tags.
pub(crate) fn reindex_asset(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM assets_fts WHERE asset_id = ?1", [id])?;
    conn.execute(
        "INSERT INTO assets_fts (asset_id, name, words, category, description, tags)
         SELECT a.id, a.name, a.words, COALESCE(a.category, ''),
                concat_ws(' ', a.description, a.source_description, a.ai_description),
                COALESCE((SELECT group_concat(replace(replace(tag, ':', ' '), '-', ' '), ' ')
                          FROM asset_tags t WHERE t.asset_id = a.id), '')
         FROM assets a WHERE a.id = ?1",
        [id],
    )?;
    Ok(())
}

pub(crate) fn upsert_tag(
    conn: &Connection,
    asset_id: &str,
    tag: &str,
    source: TagSource,
    confidence: f32,
) -> Result<()> {
    // Never let a machine overwrite a human tag; otherwise keep the most confident source.
    conn.execute(
        "INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT (asset_id, tag) DO UPDATE SET source = excluded.source, confidence = excluded.confidence
         WHERE asset_tags.source != 'human' AND (excluded.source = 'human' OR excluded.confidence > asset_tags.confidence)",
        params![asset_id, tag, source.as_str(), f64::from(confidence)],
    )?;
    Ok(())
}

/// Turn free text into a safe FTS5 query: every term becomes a quoted prefix match, and all must match.
fn fts_query(q: &str) -> Option<String> {
    let terms: Vec<String> = q
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| format!("\"{}\"*", t.to_lowercase()))
        .collect();
    if terms.is_empty() { None } else { Some(terms.join(" AND ")) }
}

struct Filter {
    join: String,
    clauses: Vec<String>,
    params: Vec<SqlValue>,
    ranked: bool,
}

fn build_filter(q: &SearchQuery, include_kind: bool) -> Filter {
    let mut f = Filter { join: String::new(), clauses: Vec::new(), params: Vec::new(), ranked: false };
    if let Some(fts) = q.q.as_deref().and_then(fts_query) {
        "JOIN (SELECT asset_id, bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0) AS rank
          FROM assets_fts WHERE assets_fts MATCH ?) f ON f.asset_id = a.id"
            .clone_into(&mut f.join);
        f.params.push(SqlValue::Text(fts));
        f.ranked = true;
    }
    if include_kind && let Some(kind) = q.kind.as_deref().filter(|k| !k.is_empty()) {
        f.clauses.push("a.kind = ?".to_owned());
        f.params.push(SqlValue::Text(kind.to_owned()));
    }
    if let Some(pack) = q.pack.as_deref().filter(|p| !p.is_empty()) {
        f.clauses.push("a.pack_id = ?".to_owned());
        f.params.push(SqlValue::Text(pack.to_owned()));
    }
    if let Some(ext) = q.ext.as_deref().filter(|e| !e.is_empty()) {
        f.clauses.push("a.ext = ?".to_owned());
        f.params.push(SqlValue::Text(ext.trim_start_matches('.').to_lowercase()));
    }
    for tag in &q.tags {
        f.clauses.push("EXISTS (SELECT 1 FROM asset_tags t WHERE t.asset_id = a.id AND t.tag = ?)".to_owned());
        f.params.push(SqlValue::Text(tag.clone()));
    }
    if q.hide_supporting {
        f.clauses.push(SUPPORTING.replace("EXISTS", "NOT EXISTS"));
    }
    f.clauses.push(
        match (q.archived, q.pack.as_deref().is_some_and(|p| !p.is_empty())) {
            (false, _) => LIVE,
            // Browsing one archived pack: everything archived from it, one by one or with the pack.
            (true, true) => {
                "(a.archived_at IS NOT NULL OR a.pack_id IN (SELECT id FROM packs WHERE archived_at IS NOT NULL))"
            }
            // The archive's own grid: assets archived one by one. Archived packs are listed as packs.
            (true, false) => {
                "a.archived_at IS NOT NULL AND a.pack_id NOT IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)"
            }
        }
        .to_owned(),
    );
    if let Some(c) = q.collection.as_deref().filter(|c| !c.is_empty()) {
        f.clauses.push(
            "a.id IN (SELECT ca.asset_id FROM collection_assets ca JOIN collections c ON c.id = ca.collection_id
                      WHERE c.id = ? OR c.name = ?)"
                .to_owned(),
        );
        f.params.push(SqlValue::Text(c.to_owned()));
        f.params.push(SqlValue::Text(c.to_owned()));
    }
    f
}

/// In the library rather than the archive: neither the asset nor its pack is archived. Expects the asset as `a`.
pub(crate) const LIVE: &str =
    "a.archived_at IS NULL AND a.pack_id NOT IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)";

/// An asset another asset needs as raw data (a glTF's .bin, an OBJ's .mtl): not something you'd browse for.
const SUPPORTING: &str = "EXISTS (SELECT 1 FROM asset_links l WHERE l.to_id = a.id
     AND (l.rel = 'buffer' OR (l.rel = 'material' AND a.ext = 'mtl')))";

impl Filter {
    fn where_sql(&self) -> String {
        if self.clauses.is_empty() { String::new() } else { format!("WHERE {}", self.clauses.join(" AND ")) }
    }
}

pub(crate) struct SearchPage {
    pub(crate) items: Vec<Asset>,
    pub(crate) total: u64,
    pub(crate) kinds: Vec<Facet>,
    pub(crate) tags: Vec<Facet>,
    pub(crate) supporting_hidden: u64,
}

pub(crate) fn search(conn: &Connection, q: &SearchQuery, with_facets: bool) -> Result<SearchPage> {
    let limit = i64::from(q.limit.unwrap_or(60).clamp(1, 5000));
    let offset = i64::from(q.offset.unwrap_or(0));
    let f = build_filter(q, true);
    let where_sql = f.where_sql();
    let base = format!(
        "FROM assets a JOIN packs p ON p.id = a.pack_id JOIN blobs b ON b.hash = a.blob_hash {} {where_sql}",
        f.join
    );

    let order = match q.sort.as_deref() {
        Some("name") => "a.name COLLATE NOCASE ASC",
        Some("size") => "b.size DESC",
        Some("old") => "a.created_at ASC, a.path ASC",
        _ if f.ranked => "f.rank ASC",
        _ => "a.created_at DESC, a.path ASC",
    };

    let total: i64 = conn.query_row(&format!("SELECT COUNT(*) {base}"), params_from_iter(&f.params), |r| r.get(0))?;
    let supporting_hidden: i64 = if q.hide_supporting {
        let shown = SearchQuery { hide_supporting: false, ..q.clone() };
        let fs = build_filter(&shown, true);
        let mut clauses = fs.clauses.clone();
        clauses.push(SUPPORTING.to_owned());
        conn.query_row(
            &format!("SELECT COUNT(*) FROM assets a {} WHERE {}", fs.join, clauses.join(" AND ")),
            params_from_iter(&fs.params),
            |r| r.get(0),
        )?
    } else {
        0
    };

    let mut page_params = f.params.clone();
    page_params.push(SqlValue::Integer(limit));
    page_params.push(SqlValue::Integer(offset));
    let mut stmt = conn.prepare(&format!("SELECT {ASSET_COLS} {base} ORDER BY {order} LIMIT ? OFFSET ?"))?;
    let mut items =
        stmt.query_map(params_from_iter(&page_params), asset_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
    attach_tags(conn, &mut items)?;

    let (mut kinds, mut tags) = (Vec::new(), Vec::new());
    if with_facets {
        // Kind facet ignores the kind filter so the sidebar can show "switch to audio (42)".
        let fk = build_filter(q, false);
        let mut stmt = conn.prepare(&format!(
            "SELECT a.kind, COUNT(*) FROM assets a {} {} GROUP BY a.kind ORDER BY 2 DESC",
            fk.join,
            fk.where_sql()
        ))?;
        kinds = stmt
            .query_map(params_from_iter(&fk.params), |r| {
                Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut stmt = conn.prepare(&format!(
            "SELECT t.tag, COUNT(*) FROM asset_tags t WHERE t.tag NOT LIKE 'set:%' AND t.asset_id IN
               (SELECT a.id FROM assets a {} {where_sql})
             GROUP BY t.tag ORDER BY 2 DESC, 1 ASC LIMIT 60",
            f.join
        ))?;
        tags = stmt
            .query_map(params_from_iter(&f.params), |r| {
                Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
    }

    Ok(SearchPage { items, total: total as u64, kinds, tags, supporting_hidden: supporting_hidden as u64 })
}

pub(crate) fn tag_counts(conn: &Connection, prefix: &str, limit: u32) -> Result<Vec<Facet>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT tag, COUNT(*) FROM asset_tags WHERE tag LIKE ?1 || '%' ESCAPE '\\'
           AND asset_id IN (SELECT a.id FROM assets a WHERE {LIVE})
         GROUP BY tag ORDER BY 2 DESC, 1 LIMIT ?2",
    ))?;
    let escaped = prefix.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_");
    let rows = stmt
        .query_map(params![escaped, i64::from(limit)], |r| {
            Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

// ---------------------------------------------------------------------------------------------
// Jobs

pub(crate) fn enqueue(
    conn: &Connection,
    kind: &str,
    pack_id: Option<&str>,
    payload: &serde_json::Value,
) -> Result<i64> {
    let now = now();
    conn.execute(
        "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4, ?4)",
        params![kind, pack_id, payload.to_string(), now],
    )?;
    Ok(conn.last_insert_rowid())
}

#[derive(Debug)]
pub(crate) struct JobRow {
    pub(crate) id: i64,
    pub(crate) kind: String,
    pub(crate) pack_id: Option<String>,
    pub(crate) payload: serde_json::Value,
    pub(crate) attempts: i64,
}

pub(crate) fn claim_job(conn: &Connection) -> Result<Option<JobRow>> {
    let now = now();
    Ok(conn
        .query_row(
            "UPDATE jobs SET status = 'running', attempts = attempts + 1, updated_at = ?1
             WHERE id = (SELECT id FROM jobs WHERE status = 'queued' AND run_after <= ?1 ORDER BY id LIMIT 1)
             RETURNING id, kind, pack_id, payload, attempts",
            [&now],
            |r| {
                let payload: String = r.get(3)?;
                Ok(JobRow {
                    id: r.get(0)?,
                    kind: r.get(1)?,
                    pack_id: r.get(2)?,
                    payload: serde_json::from_str(&payload).unwrap_or_default(),
                    attempts: r.get(4)?,
                })
            },
        )
        .optional()?)
}

pub(crate) fn finish_job(conn: &Connection, id: i64, error: Option<&str>, retry_in_secs: Option<i64>) -> Result<()> {
    let now = time::OffsetDateTime::now_utc();
    let fmt = |t: time::OffsetDateTime| t.format(&time::format_description::well_known::Rfc3339).unwrap_or_default();
    match (error, retry_in_secs) {
        (None, _) => conn.execute(
            "UPDATE jobs SET status = 'done', error = NULL, updated_at = ?2 WHERE id = ?1",
            params![id, fmt(now)],
        )?,
        (Some(e), Some(secs)) => conn.execute(
            "UPDATE jobs SET status = 'queued', error = ?2, run_after = ?3, updated_at = ?4 WHERE id = ?1",
            params![id, e, fmt(now + time::Duration::seconds(secs)), fmt(now)],
        )?,
        (Some(e), None) => conn.execute(
            "UPDATE jobs SET status = 'failed', error = ?2, updated_at = ?3 WHERE id = ?1",
            params![id, e, fmt(now)],
        )?,
    };
    Ok(())
}

/// After a crash, anything left `running` gets another go.
pub(crate) fn requeue_stale_jobs(conn: &Connection) -> Result<usize> {
    Ok(conn.execute("UPDATE jobs SET status = 'queued' WHERE status = 'running'", [])?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA_VERSION: i64 = BASELINE_VERSION + MIGRATIONS.len() as i64;

    fn backups_in(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .map(|rd| rd.filter_map(|e| e.ok()?.file_name().into_string().ok()).collect())
            .unwrap_or_default();
        names.sort();
        names
    }

    /// A database as the first published release left it: the baseline schema, with a pack in it.
    fn baseline_db(dir: &Path) -> Connection {
        let mut conn = Connection::open(dir.join("hoard.db")).unwrap();
        migrate(&mut conn, &dir.join("backups"), &[]).unwrap();
        conn.execute(
            "INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Forest', 'ready', '2026-11-11T00:00:00Z')",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn fresh_database_gets_the_whole_schema_without_a_backup() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        assert_eq!(user_version(&pool.get().unwrap()).unwrap(), SCHEMA_VERSION);
        assert_eq!(backups_in(&dir.path().join("backups")), Vec::<String>::new());
    }

    /// The real migration list applies cleanly to a baseline database and keeps its data. When you add a step to
    /// MIGRATIONS, extend this with a check that the data you care about survived it.
    #[test]
    fn baseline_database_upgrades_to_current() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        // a1 was described by the AI, a2 by its file, a3 by a person. Blobs don't matter here.
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, description, human_edited,
                                 created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.wav', 'a1', 'wav', 'audio', 'Birds at dawn.', 0, 'x', 'x'),
               ('a2', 'p1', 'h2', 'a2.wav', 'a2', 'wav', 'audio', 'Recorded in Kent.', 0, 'x', 'x'),
               ('a3', 'p1', 'h3', 'a3.wav', 'a3', 'wav', 'audio', 'My favourite.', 1, 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES ('a1', 'forest', 'llm', 0.6);",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
        assert_eq!(user_version(&conn).unwrap(), SCHEMA_VERSION);
        let source: String =
            conn.query_row("SELECT source FROM asset_tags WHERE asset_id = 'a1'", [], |r| r.get(0)).unwrap();
        assert_eq!(source, "ai");
        let descriptions = |id: &str| -> (Option<String>, Option<String>, Option<String>) {
            conn.query_row(
                "SELECT description, source_description, ai_description FROM assets WHERE id = ?1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap()
        };
        assert_eq!(descriptions("a1"), (None, None, Some("Birds at dawn.".into())));
        assert_eq!(descriptions("a2"), (None, Some("Recorded in Kent.".into()), None));
        assert_eq!(descriptions("a3"), (Some("My favourite.".into()), None, None));
        let name: String = conn.query_row("SELECT name FROM packs WHERE id = 'p1'", [], |r| r.get(0)).unwrap();
        assert_eq!(name, "Forest");
        let settings: i64 = conn.query_row("SELECT COUNT(*) FROM settings", [], |r| r.get(0)).unwrap();
        assert_eq!(settings, 0);
    }

    #[test]
    fn upgrade_backs_up_then_applies_steps_in_order() {
        let dir = tempfile::tempdir().unwrap();
        let backups = dir.path().join("backups");
        let mut conn = baseline_db(dir.path());
        let steps = ["ALTER TABLE packs ADD COLUMN notes TEXT;", "UPDATE packs SET notes = 'from step 2';"];
        migrate(&mut conn, &backups, &steps).unwrap();

        assert_eq!(user_version(&conn).unwrap(), BASELINE_VERSION + 2);
        let notes: String = conn.query_row("SELECT notes FROM packs WHERE id = 'p1'", [], |r| r.get(0)).unwrap();
        assert_eq!(notes, "from step 2");

        let saved = backups_in(&backups);
        assert_eq!(saved.len(), 1);
        assert!(saved[0].starts_with(&format!("hoard-pre-upgrade-v{BASELINE_VERSION}-")), "{saved:?}");
        let snapshot = Connection::open(backups.join(&saved[0])).unwrap();
        assert_eq!(user_version(&snapshot).unwrap(), BASELINE_VERSION);

        // Already current: nothing to do, no new backup.
        migrate(&mut conn, &backups, &steps).unwrap();
        assert_eq!(backups_in(&backups).len(), 1);
    }

    #[test]
    fn failing_step_rolls_back_and_keeps_earlier_steps() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        let steps = [
            "ALTER TABLE packs ADD COLUMN notes TEXT;",
            "ALTER TABLE packs ADD COLUMN rating INTEGER; ALTER TABLE no_such_table ADD COLUMN x TEXT;",
        ];
        let err = migrate(&mut conn, &dir.path().join("backups"), &steps).unwrap_err();
        assert!(format!("{err:#}").contains(&format!("schema version {}", BASELINE_VERSION + 2)), "{err:#}");

        assert_eq!(user_version(&conn).unwrap(), BASELINE_VERSION + 1);
        let cols: Vec<String> = conn
            .prepare("SELECT name FROM pragma_table_info('packs')")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert!(cols.contains(&"notes".to_owned()));
        assert!(!cols.contains(&"rating".to_owned()), "half of the failed step stuck: {cols:?}");
    }

    /// Archived things leave search, the counts and the tag list; restoring brings them back; purging deletes only
    /// what's in the archive.
    #[test]
    fn the_archive_hides_restores_and_purges() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let mut conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x'), ('h2', 1, 'x'), ('h3', 1, 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('p1', 'Forest', 'ready', 'x'), ('p2', 'Castle', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.wav', 'a1', 'wav', 'audio', 'x', 'x'),
               ('a2', 'p1', 'h2', 'a2.wav', 'a2', 'wav', 'audio', 'x', 'x'),
               ('a3', 'p2', 'h3', 'a3.wav', 'a3', 'wav', 'audio', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES ('a3', 'stone', 'human', 1.0);",
        )
        .unwrap();
        let ids = |conn: &Connection, archived: bool| -> Vec<String> {
            let q = SearchQuery { archived, ..Default::default() };
            let mut v: Vec<String> = search(conn, &q, false).unwrap().items.into_iter().map(|a| a.id).collect();
            v.sort();
            v
        };
        let s = |v: &[&str]| v.iter().map(ToString::to_string).collect::<Vec<_>>();

        let n = set_archived(&mut conn, &s(&["p2"]), &s(&["a1"]), true).unwrap();
        assert_eq!((n.packs, n.assets), (1, 1));
        assert_eq!(ids(&conn, false), s(&["a2"]));
        assert_eq!(ids(&conn, true), s(&["a1"]), "archived packs are listed on their own");
        let q = SearchQuery { archived: true, pack: Some("p2".into()), ..Default::default() };
        assert_eq!(search(&conn, &q, false).unwrap().total, 1, "an archived pack's assets open from the archive");
        assert_eq!(list_packs(&conn, false).unwrap().iter().map(|p| p.id.clone()).collect::<Vec<_>>(), s(&["p1"]));
        assert!(
            tag_counts(&conn, "", 100).unwrap().iter().all(|f| f.value != "stone"),
            "archived tags leave the tag list"
        );

        // Archiving again counts nothing; restoring the pack brings its asset back.
        assert_eq!(set_archived(&mut conn, &s(&["p2"]), &[], true).unwrap().packs, 0);
        set_archived(&mut conn, &s(&["p2"]), &[], false).unwrap();
        assert_eq!(ids(&conn, false), s(&["a2", "a3"]));

        // Purging something still in the library does nothing; purging the archive deletes a1 only.
        let (n, gone) = purge_archived(&mut conn, &s(&["p1"]), &s(&["a2"]), false).unwrap();
        assert_eq!((n.packs, n.assets, gone.len()), (0, 0, 0));
        let (n, _) = purge_archived(&mut conn, &[], &[], true).unwrap();
        assert_eq!((n.packs, n.assets), (0, 1));
        let left: i64 = conn.query_row("SELECT COUNT(*) FROM assets", [], |r| r.get(0)).unwrap();
        assert_eq!(left, 2);

        // A purged pack takes its assets with it.
        set_archived(&mut conn, &s(&["p2"]), &[], true).unwrap();
        let (n, gone) = purge_archived(&mut conn, &[], &[], true).unwrap();
        assert_eq!((n.packs, gone), (1, s(&["p2"])));
        assert_eq!(ids(&conn, false), s(&["a2"]));
        assert_eq!(ids(&conn, true), Vec::<String>::new());
    }

    #[test]
    fn database_from_a_newer_build_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        conn.execute_batch(&format!("PRAGMA user_version = {}", SCHEMA_VERSION + 1)).unwrap();
        let err = migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap_err();
        assert!(err.to_string().contains("a newer Hoard has upgraded it"), "{err}");
    }

    #[test]
    fn only_the_newest_upgrade_backups_are_kept() {
        let dir = tempfile::tempdir().unwrap();
        let backups = dir.path().join("backups");
        std::fs::create_dir_all(&backups).unwrap();
        std::fs::write(backups.join("hoard-20260101-000000.db"), b"manual backup").unwrap();
        let conn = baseline_db(dir.path());
        for i in 0..(KEEP_UPGRADE_BACKUPS + 2) {
            let stale = backups.join(format!("hoard-pre-upgrade-v1-2020010{i}-000000.db"));
            std::fs::write(&stale, b"old").unwrap();
            let t = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000 + i as u64);
            std::fs::File::options().write(true).open(&stale).unwrap().set_modified(t).unwrap();
        }
        backup_before_upgrade(&conn, &backups, BASELINE_VERSION).unwrap();

        let left = backups_in(&backups);
        let upgrades: Vec<_> = left.iter().filter(|n| n.starts_with("hoard-pre-upgrade-")).collect();
        assert_eq!(upgrades.len(), KEEP_UPGRADE_BACKUPS, "{left:?}");
        assert!(upgrades.iter().any(|n| n.starts_with(&format!("hoard-pre-upgrade-v{BASELINE_VERSION}-"))));
        assert!(!left.contains(&"hoard-pre-upgrade-v1-20200100-000000.db".to_owned()), "oldest kept: {left:?}");
        assert!(left.contains(&"hoard-20260101-000000.db".to_owned()), "manual backups are never pruned");
    }
}
