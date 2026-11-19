//! The schema as of the first published release, frozen: every database starts from it.

/// Schema version of [`BASELINE`]: the schema as of the first published release (0.8.0).
pub(super) const BASELINE_VERSION: i64 = 8;

/// Frozen: see [`MIGRATIONS`](super::MIGRATIONS). Every statement is `IF NOT EXISTS`, which is how databases from before the baseline
/// (development builds) are brought up to it.
pub(super) const BASELINE: &str = "
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
