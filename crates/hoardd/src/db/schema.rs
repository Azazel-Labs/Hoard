//! The schema: the baseline every database starts from, the steps since, and the upgrade that runs them,
//! backing the database up first.

use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::Connection;

use super::now;

/// Schema version of [`BASELINE`]: the schema as of the first published release (0.8.0).
const BASELINE_VERSION: i64 = 8;

/// Changes since the baseline, oldest first. Step `i` takes the database from `BASELINE_VERSION + i` to
/// `BASELINE_VERSION + i + 1`, inside one transaction, so a failing step leaves the database untouched.
///
/// **Append only.** Never edit, reorder or remove a step once it has shipped: databases out there have run it.
/// And don't change [`BASELINE`] either; a new table, column or index is a new step here. A step can be any SQL
/// SQLite accepts in a transaction, e.g. `ALTER TABLE packs ADD COLUMN notes TEXT;` or a table rebuild.
pub(super) const MIGRATIONS: &[&str] = &[
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
    // 13: library locations (see `locations.rs`). A pack found in a watched folder records which one and the
    // folder it is, since its files are read from there.
    "CREATE TABLE locations (
       id         INTEGER PRIMARY KEY,
       name       TEXT NOT NULL,
       mode       TEXT NOT NULL,
       path       TEXT NOT NULL,
       kinds      TEXT NOT NULL DEFAULT '[]',
       catch_all  INTEGER NOT NULL DEFAULT 0,
       scanned_at TEXT,
       created_at TEXT NOT NULL
     );
     ALTER TABLE packs ADD COLUMN location_id INTEGER REFERENCES locations(id);
     ALTER TABLE packs ADD COLUMN folder TEXT;
     CREATE INDEX packs_location ON packs(location_id) WHERE location_id IS NOT NULL;",
    // 14: watched folders. What was deleted for good from one stays deleted when it's rescanned (the files are
    // still there: Hoard never deletes them), and a pack whose folder went missing waits a while before it's
    // dropped, in case the folder is only offline.
    "CREATE TABLE skipped_files (
       pack_id TEXT NOT NULL REFERENCES packs(id) ON DELETE CASCADE,
       path    TEXT NOT NULL,
       PRIMARY KEY (pack_id, path)
     ) WITHOUT ROWID;
     CREATE TABLE skipped_folders (
       location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
       folder      TEXT NOT NULL,
       PRIMARY KEY (location_id, folder)
     ) WITHOUT ROWID;
     ALTER TABLE packs ADD COLUMN missing_since TEXT;",
    // 15: Settings → Maintenance keeps a history of its runs (see `maintenance.rs`).
    "CREATE TABLE maintenance_runs (
       id          INTEGER PRIMARY KEY AUTOINCREMENT,
       task        TEXT NOT NULL,
       started_at  TEXT NOT NULL,
       finished_at TEXT,
       ok          INTEGER,
       summary     TEXT
     );",
    // 16: accounts, replacing the shared HOARD_TOKEN (see the `auth` module). Sessions and access tokens are stored as
    // SHA-256 hashes, so a copy of the database doesn't let anyone in.
    "CREATE TABLE users (
       id            INTEGER PRIMARY KEY AUTOINCREMENT,
       username      TEXT NOT NULL UNIQUE COLLATE NOCASE,
       password_hash TEXT NOT NULL,
       created_at    TEXT NOT NULL
     );
     CREATE TABLE sessions (
       token_hash   TEXT PRIMARY KEY,
       user_id      INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
       created_at   TEXT NOT NULL,
       last_seen_at TEXT NOT NULL
     ) WITHOUT ROWID;
     CREATE TABLE access_tokens (
       id           INTEGER PRIMARY KEY AUTOINCREMENT,
       name         TEXT NOT NULL,
       token_hash   TEXT NOT NULL UNIQUE,
       created_by   INTEGER REFERENCES users(id) ON DELETE SET NULL,
       created_at   TEXT NOT NULL,
       last_used_at TEXT
     );",
    // 17: when a cached "download as" conversion was last handed out, so the cache can drop the least recently used
    // when it outgrows its limit (see `formats::trim_conversions`).
    "ALTER TABLE derivatives ADD COLUMN used_at TEXT;",
    // 18: apps (Claude and other MCP clients) that sign in over OAuth, and what each person let them do (see
    // `oauth.rs`). Tokens are stored as hashes.
    "CREATE TABLE oauth_clients (
       id            TEXT PRIMARY KEY,
       name          TEXT NOT NULL,
       redirect_uris TEXT NOT NULL,
       secret_hash   TEXT,
       created_at    TEXT NOT NULL
     ) WITHOUT ROWID;
     CREATE TABLE oauth_grants (
       id                INTEGER PRIMARY KEY AUTOINCREMENT,
       client_id         TEXT NOT NULL REFERENCES oauth_clients(id) ON DELETE CASCADE,
       user_id           INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
       access_hash       TEXT NOT NULL UNIQUE,
       access_expires_at INTEGER NOT NULL,
       refresh_hash      TEXT NOT NULL UNIQUE,
       created_at        TEXT NOT NULL,
       last_used_at      TEXT
     );",
    // 19: tags merged into others. Tagging maps the old name to the new one, so a merged tag stays merged when the
    // taggers run again (see `merge_tags`).
    "CREATE TABLE tag_aliases (tag TEXT PRIMARY KEY, into_tag TEXT NOT NULL) WITHOUT ROWID;",
    // 20: search by look (see `looks`): a palette, visual descriptor and perceptual hash per thumbnail, and image
    // embeddings from an optional model. Keyed by the thumbnail's blob, so a new thumbnail is simply a new row.
    "CREATE TABLE looks (
       thumb_hash TEXT PRIMARY KEY,
       version    INTEGER NOT NULL,
       palette    BLOB NOT NULL,
       descriptor BLOB NOT NULL,
       phash      INTEGER NOT NULL
     ) WITHOUT ROWID;
     CREATE TABLE look_embeddings (
       thumb_hash TEXT NOT NULL,
       model      TEXT NOT NULL,
       vector     BLOB NOT NULL,
       PRIMARY KEY (thumb_hash, model)
     ) WITHOUT ROWID;",
    // 21: materials made from texture maps (see `materials`): which textures each one uses, and as what. Every
    // pack gets a look for them.
    "CREATE TABLE material_maps (
       material_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
       texture_id  TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
       role        TEXT NOT NULL,
       resolution  TEXT,
       PRIMARY KEY (material_id, texture_id)
     ) WITHOUT ROWID;
     CREATE INDEX material_maps_texture ON material_maps(texture_id);
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs WHERE status = 'ready';",
    // 22: where an asset's thumbnail came from, when not the server's analysis: `browser` or `browser-partial`
    // (see `Asset::thumb_source`). NULL is the server's own.
    "ALTER TABLE assets ADD COLUMN thumb_source TEXT;",
    // 23: what a pack's own files say (a Megascans JSON's tags, and when they were read), who sells it, and the store
    // lookup: suggestions found in the background, and whether the store page was attached automatically.
    "ALTER TABLE packs ADD COLUMN file_tags TEXT;
     ALTER TABLE packs ADD COLUMN files_read_at TEXT;
     ALTER TABLE pack_context ADD COLUMN seller TEXT;
     ALTER TABLE pack_context ADD COLUMN store_license TEXT;
     ALTER TABLE pack_context ADD COLUMN store_auto TEXT;
     ALTER TABLE pack_context ADD COLUMN store_matches TEXT;
     ALTER TABLE pack_context ADD COLUMN store_lookup_at TEXT;",
    // 24: image search: each thumbnail's CLIP embedding (512 signed bytes; empty when it isn't a readable picture).
    "CREATE TABLE embeddings (
       thumb_hash TEXT PRIMARY KEY,
       model      TEXT NOT NULL,
       vector     BLOB NOT NULL
     ) WITHOUT ROWID;",
    // 25: Settings → Issues (see `issues.rs`): problems that aren't kept anywhere else, and quick ways to the files
    // whose analysis failed.
    "CREATE TABLE issues (
       id          INTEGER PRIMARY KEY AUTOINCREMENT,
       key         TEXT NOT NULL UNIQUE,
       kind        TEXT NOT NULL,
       pack_id     TEXT REFERENCES packs(id) ON DELETE CASCADE,
       location_id INTEGER REFERENCES locations(id) ON DELETE CASCADE,
       subject     TEXT NOT NULL DEFAULT '',
       message     TEXT NOT NULL,
       count       INTEGER NOT NULL DEFAULT 1,
       first_at    TEXT NOT NULL,
       last_at     TEXT NOT NULL
     );
     CREATE INDEX issues_kind ON issues(kind, last_at);
     CREATE INDEX issues_pack ON issues(pack_id) WHERE pack_id IS NOT NULL;
     CREATE INDEX assets_analysis_error ON assets(updated_at) WHERE json_extract(meta, '$.analysis_error') IS NOT NULL;
     CREATE INDEX assets_thumb_error ON assets(updated_at) WHERE json_extract(meta, '$.thumb_error') IS NOT NULL;",
    // 26: a material only one model uses becomes part of it (`materials::mark_owned`), worked out by looking at each
    // pack's materials again. A browser drawing of a model that wears a material may have been taken before the
    // material's maps arrived (all white): it's drawn again when the model is next opened.
    "UPDATE assets SET thumb_source = 'browser-partial'
       WHERE thumb_source = 'browser' AND id IN (SELECT from_id FROM asset_links WHERE rel = 'material');
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext = 'material');",
    // 27: the server's own thumbnails of models that wear a material are drawn again with its colour map on
    // (`materials::dress_models`, run with each pack's materials).
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext = 'material');",
    // 28: browser drawings of models that wear a material, by name or through its maps beside them, may be all white
    // (taken before the maps arrived; 26 caught only the first kind). A browser draws them afresh the next time the
    // model is opened, and FBX and OBJ ones go back to being the server's, which draws them again with the colour map.
    "UPDATE assets SET thumb_source = CASE WHEN ext IN ('fbx', 'obj') THEN NULL ELSE 'browser-partial' END
       WHERE thumb_source IN ('browser', 'browser-partial') AND kind = 'model'
         AND EXISTS (SELECT 1 FROM asset_links l WHERE l.from_id = assets.id
                       AND (l.rel = 'material' OR l.to_id IN (SELECT texture_id FROM material_maps)));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext = 'material')
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id);",
    // 29: the server draws glTF, GLB, STL and PLY models with the colour map of the material they wear too, so their
    // browser drawings that may be all white go back to being the server's (as 28 did for FBX and OBJ).
    "UPDATE assets SET thumb_source = NULL
       WHERE thumb_source IN ('browser', 'browser-partial') AND kind = 'model' AND ext IN ('glb', 'gltf', 'stl', 'ply')
         AND EXISTS (SELECT 1 FROM asset_links l WHERE l.from_id = assets.id
                       AND (l.rel = 'material' OR l.to_id IN (SELECT texture_id FROM material_maps)));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext = 'material')
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id);",
    // 30: STL files can colour their facets, which the server now draws (`materials::recolor_stls`, run with each
    // pack's materials). Browser drawings of them go back to the server, and a browser draws them afresh when opened.
    "UPDATE assets SET meta = json_set(meta, '$.recolor', 1), thumb_source = NULL
       WHERE kind = 'model' AND ext = 'stl' AND thumb_hash IS NOT NULL;
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext = 'stl' AND a.kind = 'model')
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id);",
    // 31: images say whether they were made or edited with AI (`provenance`), read on analysis; ones already read are
    // looked at with each pack's materials job.
    "UPDATE assets SET meta = json_set(meta, '$.provenance', 1) WHERE kind = 'texture';
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'texture')
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id);",
    // 32: a material's maps are named after it and what they're for (`naming::name_maps`), done with each pack's
    // materials job.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM material_maps mm JOIN assets a ON a.id = mm.material_id WHERE a.pack_id = p.id)
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id);",
    // 33: packs with USD models are analyzed again, now that Hoard reads USD, and so are packs with .blend files:
    // a .blend without a preview of its own had a picture of the Blender window as its thumbnail.
    "UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id
                     AND a.ext IN ('blend', 'usd', 'usda', 'usdc', 'usdz'))
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 34: packs with a Megascans `<id>.json` are read again, for the maps, levels of detail and real size it lists.
    "UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM asset_links l JOIN assets a ON a.id = l.to_id
                     WHERE l.rel = 'sidecar' AND a.pack_id = packs.id)
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 35: the server draws glass (glTF's transmission) see-through. Its thumbnails of glTF and GLB models are looked
    // at again with each pack's materials job (`materials::redraw_glass`), which redraws the ones with glass. The job
    // also does what 28-32 left for it in packs whose materials job had already run once (those skipped them).
    "UPDATE assets SET meta = json_set(COALESCE(meta, '{}'), '$.reglass', 1)
       WHERE kind = 'model' AND ext IN ('glb', 'gltf') AND thumb_source IS NULL AND thumb_hash IS NOT NULL;
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND (EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id
                     AND (json_extract(a.meta, '$.reglass') IS NOT NULL OR json_extract(a.meta, '$.recolor') IS NOT NULL
                          OR json_extract(a.meta, '$.provenance') IS NOT NULL
                          OR (a.ext = 'material' AND EXISTS (SELECT 1 FROM material_maps mm
                                                             WHERE mm.material_id = a.id)))))
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                          AND j.status IN ('queued', 'running'));",
    // 36: roles (see `auth::Role`). Everyone could do everything before, so existing accounts and tokens are admins.
    "ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'admin' CHECK (role IN ('view', 'edit', 'admin'));
     ALTER TABLE access_tokens ADD COLUMN role TEXT NOT NULL DEFAULT 'admin' CHECK (role IN ('view', 'edit', 'admin'));",
    // 37: each person's favourite assets and saved searches (see `me`).
    "CREATE TABLE favourites (
       user_id  INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
       asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
       added_at TEXT NOT NULL,
       PRIMARY KEY (user_id, asset_id)
     );
     CREATE INDEX favourites_asset ON favourites(asset_id);
     CREATE TABLE saved_searches (
       id         INTEGER PRIMARY KEY AUTOINCREMENT,
       user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
       name       TEXT NOT NULL,
       query      TEXT NOT NULL,
       created_at TEXT NOT NULL,
       UNIQUE (user_id, name)
     );",
    // 38: which projects use which assets, as each `hoard sync` reports its hoard.lock (see `projects`).
    "CREATE TABLE projects (
       id         INTEGER PRIMARY KEY AUTOINCREMENT,
       name       TEXT NOT NULL UNIQUE COLLATE NOCASE,
       synced_at  TEXT NOT NULL,
       synced_by  TEXT
     );
     CREATE TABLE project_assets (
       project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
       asset_id   TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
       path       TEXT NOT NULL,
       PRIMARY KEY (project_id, asset_id)
     );
     CREATE INDEX project_assets_asset ON project_assets(asset_id);",
    // 39: "more like this" for sounds (see `sounds`): an acoustic descriptor per audio file, keyed by its blob.
    "CREATE TABLE sounds (
       hash       TEXT PRIMARY KEY,
       version    INTEGER NOT NULL,
       descriptor BLOB NOT NULL
     ) WITHOUT ROWID;",
    // 40: accounts whose sign-in a directory (LDAP, Active Directory) or an OAuth provider checks (see `signin`).
    // They have no password of their own (an empty hash, which never matches).
    "ALTER TABLE users ADD COLUMN source TEXT NOT NULL DEFAULT 'internal' CHECK (source IN ('internal', 'ldap', 'oauth'));",
    // 41: who an OAuth account is at its provider (issuer and `sub`), so a name there can't stand in for them. Tokens
    // go with the account that made them.
    "ALTER TABLE users ADD COLUMN subject TEXT;
     CREATE UNIQUE INDEX users_subject ON users(subject) WHERE subject IS NOT NULL;
     DELETE FROM access_tokens WHERE created_by IS NULL;",
    // 42: big files upload as fixed-size parts, several at once and in any order (see `uploads`), so what's arrived
    // is a set of parts rather than a length. A file that was part-way through starts over.
    "CREATE TABLE upload_parts (
       pack_id TEXT NOT NULL REFERENCES packs(id) ON DELETE CASCADE,
       path    TEXT NOT NULL,
       part    INTEGER NOT NULL,
       bytes   INTEGER NOT NULL,
       PRIMARY KEY (pack_id, path, part)
     ) WITHOUT ROWID;
     UPDATE upload_files SET received = 0 WHERE received < size;",
    // 43: "favourites" is spelled the American way.
    "ALTER TABLE favourites RENAME TO favorites;
     DROP INDEX favourites_asset;
     CREATE INDEX favorites_asset ON favorites(asset_id);",
    // 44: packs are tagged again where the tagger now says more or groups differently: words in brackets in a path
    // and a sound's artist are tags, Illustrator files are pictures, drawings are never material maps, and a
    // lettered series (`character_a`, `character_d`) isn't a material.
    "UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id
                     AND (a.ext IN ('ai', 'svg', 'svgz') OR a.path LIKE '%(%' OR a.path LIKE '%[%'
                          OR json_extract(a.meta, '$.artist') IS NOT NULL
                          OR (a.ext = 'material' AND json_extract(a.meta, '$.material_maps') IS NOT NULL)))
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 45: vector art (SVG, Illustrator, Affinity Designer, EPS) is a kind of its own rather than a texture. A library
    // location that takes textures takes it too, so nothing has to move.
    "UPDATE assets SET kind = 'vector',
            category = CASE WHEN category = 'textures' THEN 'vector' ELSE category END
       WHERE kind = 'texture' AND ext IN ('svg', 'svgz', 'ai', 'afdesign', 'eps');
     UPDATE OR IGNORE asset_tags SET tag = 'kind:vector'
       WHERE tag = 'kind:texture' AND asset_id IN (SELECT id FROM assets WHERE kind = 'vector');
     DELETE FROM asset_tags WHERE tag = 'kind:texture' AND asset_id IN (SELECT id FROM assets WHERE kind = 'vector');
     DELETE FROM assets_fts WHERE asset_id IN (SELECT id FROM assets WHERE kind = 'vector');
     INSERT INTO assets_fts (asset_id, name, words, category, description, tags)
       SELECT a.id, a.name, a.words, COALESCE(a.category, ''),
              concat_ws(' ', a.description, a.source_description, a.ai_description),
              COALESCE((SELECT group_concat(replace(replace(tag, ':', ' '), '-', ' '), ' ')
                        FROM asset_tags t WHERE t.asset_id = a.id), '')
       FROM assets a WHERE a.kind = 'vector';
     UPDATE locations SET kinds = json_insert(kinds, '$[#]', 'vector')
       WHERE EXISTS (SELECT 1 FROM json_each(locations.kinds) WHERE value = 'texture');",
    // 46: Hoard reads ASCII FBX (and hands Blender a binary copy) and 3DS (no longer sent to Blender, whose 3DS
    // importer isn't shipped since 4.2). Packs with either are analyzed again, for thumbnails; packs where Blender
    // refused an FBX are converted again; and the 3DS conversion failures are gone.
    "UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id
                     AND (a.ext = '3ds' OR (a.ext = 'fbx' AND json_extract(a.meta, '$.fbx_ascii'))))
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'convert', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status IN ('ready', 'ingesting') AND p.archived_at IS NULL
         AND EXISTS (SELECT 1 FROM issues i WHERE i.kind = 'convert' AND i.pack_id = p.id
                     AND lower(i.subject) LIKE '%.fbx')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.kind = 'convert'
                           AND j.status IN ('queued', 'running'));
     DELETE FROM issues WHERE kind = 'convert' AND lower(subject) LIKE '%.3ds';",
    // 47: sprite atlases' sprites are assets of their own. Packs with a file that may be an atlas are looked
    // through for them.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready' AND p.archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext IN ('xml', 'json', 'atlas', 'plist'))
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'texture')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 48: meshopt-compressed glTF and GLB models are drawn. Packs with a glTF or GLB model that has no thumbnail
    // are analyzed again.
    "UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id AND a.kind = 'model'
                     AND a.ext IN ('gltf', 'glb') AND a.thumb_hash IS NULL)
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 49: Blender converts only the formats Hoard can't read (.blend, Collada, Alembic, X3D). Its FBX importer
    // made textures with an alpha channel see-through, so the GLBs it made of anything else are dropped (Hoard
    // builds its own) and their conversion failures go. The browser now shows OBJ through Hoard's GLB too, since
    // its own OBJ reader spilled concave polygons over their neighbors. Thumbnails the browser drew from either
    // are drawn again.
    "CREATE TEMP TABLE redraw AS SELECT id, pack_id FROM assets a
       WHERE a.thumb_source IS NOT NULL AND a.kind = 'model'
         AND (a.ext = 'obj' OR (a.ext NOT IN ('blend', 'dae', 'abc', 'x3d')
              AND EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'glb')));
     UPDATE assets SET thumb_source = NULL WHERE id IN (SELECT id FROM temp.redraw);
     UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL AND id IN (SELECT pack_id FROM temp.redraw)
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     DROP TABLE temp.redraw;
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));
     DELETE FROM derivatives WHERE kind = 'glb'
       AND asset_id IN (SELECT id FROM assets WHERE ext NOT IN ('blend', 'dae', 'abc', 'x3d'));
     DELETE FROM issues WHERE kind = 'convert' AND lower(subject) NOT LIKE '%.blend'
       AND lower(subject) NOT LIKE '%.dae' AND lower(subject) NOT LIKE '%.abc' AND lower(subject) NOT LIKE '%.x3d';",
    // 50: Unity materials (.mat) are read into materials, and a texture is guessed for a model only from the model's
    // own part of the pack. Packs with either are read again; thumbnails the browser drew with a guessed texture on
    // are drawn again.
    "CREATE TEMP TABLE redo AS SELECT DISTINCT a.pack_id FROM assets a
       WHERE a.ext = 'mat' OR EXISTS (SELECT 1 FROM asset_links l WHERE l.from_id = a.id AND l.rel = 'guess');
     UPDATE assets SET thumb_source = NULL WHERE thumb_source IS NOT NULL
       AND EXISTS (SELECT 1 FROM asset_links l WHERE l.from_id = assets.id AND l.rel = 'guess');
     UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL AND id IN (SELECT pack_id FROM temp.redo)
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     DROP TABLE temp.redo;
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 51: polysmelt paints a texture found by name only on a model whose materials are plain, and takes it only from
    // the model's part of the pack. Models with a texture whose file name starts with theirs (any that might have
    // been painted) are drawn again and their GLB downloads made again.
    "CREATE TEMP TABLE named AS SELECT id, pack_id, kind, ext,
            lower(substr(path, length(rtrim(path, replace(path, '/', ''))) + 1)) AS file
       FROM assets WHERE kind = 'texture' OR (kind = 'model' AND ext IN ('obj', 'fbx', '3ds'));
     CREATE INDEX temp.named_file ON named(pack_id, kind, file);
     CREATE TEMP TABLE guessed AS SELECT m.id, m.pack_id FROM
         (SELECT id, pack_id, substr(file, 1, length(file) - length(ext) - 1) AS stem FROM temp.named
           WHERE kind = 'model') m
       WHERE length(m.stem) >= 3 AND EXISTS (SELECT 1 FROM temp.named t
         WHERE t.pack_id = m.pack_id AND t.kind = 'texture' AND t.file >= m.stem AND t.file < m.stem || char(1114111));
     DROP TABLE temp.named;
     DELETE FROM derivatives WHERE kind LIKE 'as:glb:%' AND asset_id IN (SELECT id FROM temp.guessed);
     UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL AND id IN (SELECT pack_id FROM temp.guessed)
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     DROP TABLE temp.guessed;
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 52: Wavefront materials with a color and no maps become swatches, and `.mtl` files are drawn as their
    // materials. Packs with `.mtl` files look for materials again.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext = 'mtl')
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id);",
    // 53: a model wears the Unity materials its material slots name, or that its importer's `.meta` puts in them
    // (`unity_remaps`, read as packs are unpacked). Packs with `.mat` files look for materials again.
    "CREATE TABLE IF NOT EXISTS unity_remaps (
         pack_id TEXT NOT NULL REFERENCES packs(id) ON DELETE CASCADE,
         path    TEXT NOT NULL,
         slot    TEXT NOT NULL,
         guid    TEXT NOT NULL,
         PRIMARY KEY (pack_id, path, slot)
     ) WITHOUT ROWID;
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.ext = 'mat')
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                          AND j.status IN ('queued', 'running'));",
    // 54: polysmelt 0.11.3 takes a texture found by name only from beside or above the model or from a folder named
    // for textures, picks the nearest of several the same way every time, finds `../` and absolute paths' textures
    // nearest the model, embeds a texture named twice once, refuses ASCII FBX written with decimal commas, and widens
    // indices WebGL 2 would draw as a restart. Every 3D viewer and GLB download is made again when next asked for,
    // and models that might have been painted with a texture found by name are drawn again.
    "DELETE FROM derivatives WHERE kind LIKE 'as:view:%' OR kind LIKE 'as:glb:%';
     CREATE TEMP TABLE named AS SELECT id, pack_id, kind, ext,
            lower(substr(path, length(rtrim(path, replace(path, '/', ''))) + 1)) AS file
       FROM assets WHERE kind = 'texture' OR (kind = 'model' AND ext IN ('obj', 'fbx', '3ds'));
     CREATE INDEX temp.named_file ON named(pack_id, kind, file);
     CREATE TEMP TABLE guessed AS SELECT m.id, m.pack_id FROM
         (SELECT id, pack_id, substr(file, 1, length(file) - length(ext) - 1) AS stem FROM temp.named
           WHERE kind = 'model') m
       WHERE length(m.stem) >= 3 AND EXISTS (SELECT 1 FROM temp.named t
         WHERE t.pack_id = m.pack_id AND t.kind = 'texture' AND t.file >= m.stem AND t.file < m.stem || char(1114111));
     DROP TABLE temp.named;
     UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL AND id IN (SELECT pack_id FROM temp.guessed)
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     DROP TABLE temp.guessed;
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 55: polysmelt 0.12.0 turns STL models from Z up, as printing has them, to Y up. Their 3D viewers and GLB
    // downloads are made again when next asked for, and packs with STL models are drawn again.
    "DELETE FROM derivatives WHERE (kind LIKE 'as:view:%' OR kind LIKE 'as:glb:%')
       AND asset_id IN (SELECT id FROM assets WHERE ext = 'stl');
     UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id AND a.ext = 'stl' AND a.kind = 'model')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 56: a pack's models can be turned to stand up and face forward (`Facing`), for files that don't say which
    // way is which.
    "ALTER TABLE packs ADD COLUMN model_turn INTEGER NOT NULL DEFAULT 0;
     ALTER TABLE packs ADD COLUMN model_tilt INTEGER NOT NULL DEFAULT 0;",
    // 57: a model's skeleton and clips (`animations::Rig`, as JSON), from which clips shipped in files of their own
    // become animations linked to the characters they fit. Packs with skinned or animated models, or .blend files,
    // are read again.
    "CREATE TABLE IF NOT EXISTS rigs (
         asset_id TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
         rig      TEXT NOT NULL
     ) WITHOUT ROWID;
     UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id AND a.kind = 'model'
                       AND (a.ext = 'blend' OR json_extract(a.meta, '$.has_deformers') = 1
                            OR json_extract(a.meta, '$.has_animation') = 1 OR json_extract(a.meta, '$.skins') > 0
                            OR json_extract(a.meta, '$.animations') > 0))
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 58: a model whose file says it has no clips (or no skin) isn't tagged animated (or rigged) for its folders'
    // names, as every hat in "Animated Characters Bundle" was.
    "CREATE TEMP TABLE guessed AS
       SELECT t.asset_id, t.tag FROM asset_tags t JOIN assets a ON a.id = t.asset_id
        WHERE a.kind = 'model' AND t.source = 'rule'
          AND ((t.tag = 'animated'
                AND (json_type(a.meta, '$.animations') IS NOT NULL OR json_type(a.meta, '$.has_animation') IS NOT NULL)
                AND COALESCE(json_extract(a.meta, '$.animations'), 0) = 0
                AND COALESCE(json_extract(a.meta, '$.has_animation'), 0) = 0)
            OR (t.tag = 'rigged'
                AND (json_type(a.meta, '$.skins') IS NOT NULL OR json_type(a.meta, '$.has_deformers') IS NOT NULL)
                AND COALESCE(json_extract(a.meta, '$.skins'), 0) = 0
                AND COALESCE(json_extract(a.meta, '$.has_deformers'), 0) = 0));
     DELETE FROM asset_tags WHERE (asset_id, tag) IN (SELECT asset_id, tag FROM guessed);
     DELETE FROM assets_fts WHERE asset_id IN (SELECT asset_id FROM guessed);
     INSERT INTO assets_fts (asset_id, name, words, category, description, tags)
       SELECT a.id, a.name, a.words, COALESCE(a.category, ''),
              concat_ws(' ', a.description, a.source_description, a.ai_description),
              COALESCE((SELECT group_concat(replace(replace(tag, ':', ' '), '-', ' '), ' ')
                        FROM asset_tags t WHERE t.asset_id = a.id), '')
       FROM assets a WHERE a.id IN (SELECT DISTINCT asset_id FROM guessed);
     DROP TABLE guessed;",
    // 59: a model wears a Unity material named after its slot only from its own Unity project, or its own part of
    // the pack, not from another kit in the same bundle, as Kenney's characters all wore one kit's skin. Packs where
    // models wear such materials look for materials again, which redraws the thumbnails dressed in the wrong one.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
        AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'model'
                      AND json_extract(a.meta, '$.material_slots') IS NOT NULL)
        AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                          AND j.status IN ('queued', 'running'));",
    // 60: conversion rules (`reformat.rs`). A model a rule converted was turned upright as it was written
    // (`upright`), so its pack's facing no longer applies to it; one put back as it was (`kept_formats`) is left
    // alone by the rules from then on.
    "ALTER TABLE asset_origin ADD COLUMN upright INTEGER NOT NULL DEFAULT 0;
     CREATE TABLE IF NOT EXISTS kept_formats (
         asset_id TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE
     ) WITHOUT ROWID;",
    // 61: an atlas file is a sprite sheet (`spritesheet`), kept whole, rather than its sprites cut into assets of
    // their own. The sprites cut before are dropped, and packs with a file that may be an atlas (Godot's `.tres`
    // now too) are looked through again.
    "DELETE FROM assets_fts WHERE asset_id IN (SELECT id FROM assets WHERE json_extract(meta, '$.sprite') IS NOT NULL);
     DELETE FROM assets WHERE json_extract(meta, '$.sprite') IS NOT NULL;
     DELETE FROM asset_links WHERE rel = 'sprite';
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id
                       AND lower(a.ext) IN ('xml', 'json', 'atlas', 'plist', 'tres'))
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'texture')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                           AND j.status IN ('queued', 'running'));",
    // 62: OBJ thumbnails drawn before OBJ models were read and drawn as they are now may be wrong. Every OBJ model's
    // thumbnail goes back to being the server's, packs with OBJ models are drawn again, and a browser draws its own
    // afresh the next time the model is opened.
    "UPDATE assets SET thumb_source = NULL WHERE kind = 'model' AND ext = 'obj' AND thumb_source IS NOT NULL;
     UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id AND a.kind = 'model' AND a.ext = 'obj')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 63: a sound whose file gives it a title that's only a number (Kenney's footsteps all say TITLE=1) was called
    // that number. Such names, where nobody typed them, become the file's name made readable: `footstep00.ogg` is
    // "Footstep 00".
    "CREATE TEMP TABLE numbered AS
       WITH f AS (SELECT id, COALESCE(ext, '') AS ext,
                         substr(path, length(rtrim(path, replace(path, '/', ''))) + 1) AS file
                    FROM assets WHERE human_edited = 0 AND name NOT GLOB '*[^0-9 ._#-]*'),
            s AS (SELECT id, trim(replace(replace(replace(
                    CASE WHEN ext <> '' AND lower(file) LIKE '%.' || lower(ext)
                         THEN substr(file, 1, length(file) - length(ext) - 1) ELSE file END,
                    '_', ' '), '-', ' '), '.', ' ')) AS stem FROM f),
            d AS (SELECT id, stem, rtrim(stem, '0123456789') AS head FROM s WHERE stem GLOB '*[A-Za-z]*')
       SELECT id, CASE WHEN head <> stem AND substr(head, -1) <> ' ' THEN head || ' ' || substr(stem, length(head) + 1)
                       ELSE stem END AS name FROM d;
     UPDATE assets SET name = (SELECT upper(substr(n.name, 1, 1)) || lower(substr(n.name, 2)) FROM numbered n
                                WHERE n.id = assets.id)
       WHERE id IN (SELECT id FROM numbered);
     DELETE FROM assets_fts WHERE asset_id IN (SELECT id FROM numbered);
     INSERT INTO assets_fts (asset_id, name, words, category, description, tags)
       SELECT a.id, a.name, a.words, COALESCE(a.category, ''),
              concat_ws(' ', a.description, a.source_description, a.ai_description),
              COALESCE((SELECT group_concat(replace(replace(tag, ':', ' '), '-', ' '), ' ')
                        FROM asset_tags t WHERE t.asset_id = a.id), '')
       FROM assets a WHERE a.id IN (SELECT id FROM numbered);
     DROP TABLE numbered;",
    // 64: a sheet's sprites that the pack has as loose images too are linked from it (`sprite`) and shown with it,
    // as its pages are, rather than each on its own in the grid. Packs with a sheet and other images are looked
    // through again.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'sheet')
         AND (SELECT COUNT(*) FROM assets a WHERE a.pack_id = p.id AND a.kind = 'texture') > 1
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                           AND j.status IN ('queued', 'running'));",
    // 65: an asset the pack has in several formats (`house.skp` and `house.obj`, `click.wav` and `click.ogg`) is
    // one asset, its other formats linked from it (`format`) and hidden with it. Packs with files of one kind and
    // name in more than one format are looked through again.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
         AND EXISTS (SELECT 1 FROM (SELECT a.kind, lower(substr(a.path, length(rtrim(a.path, replace(a.path, '/', ''))) + 1,
                                          length(a.path) - length(rtrim(a.path, replace(a.path, '/', ''))) - length(a.ext) - 1)) AS stem,
                                          lower(a.ext) AS ext
                                     FROM assets a WHERE a.pack_id = p.id AND a.kind IN ('model', 'audio', 'font'))
                     GROUP BY kind, stem HAVING COUNT(DISTINCT ext) > 1)
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                           AND j.status IN ('queued', 'running'));",
    // 66: a tile sheet that comes with a note of its grid rather than an atlas (Kenney's `Tilesheet.txt`: "Tile size
    // 16px × 16px, Space between tiles 1px") is a sprite sheet too, cut into its tiles. Packs with a text file and
    // other images are looked through again.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND lower(a.ext) = 'txt')
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'texture')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                           AND j.status IN ('queued', 'running'));",
    // 67: Tiled tilesets (`.tsx`, `.tsj`, or a tileset saved as `.json`) are sprite sheets of their tiles. Packs with
    // one are looked through again.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND lower(a.ext) IN ('tsx', 'tsj', 'json'))
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'texture')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                           AND j.status IN ('queued', 'running'));",
    // 68: DragonBones texture atlases (`…_tex.json`) are sprite sheets. Packs with one are looked through again.
    "INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'materials', p.id, '{}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ready'
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND lower(a.path) GLOB '*_tex.json')
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = p.id AND a.kind = 'texture')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.kind = 'materials' AND j.pack_id = p.id
                           AND j.status IN ('queued', 'running'));",
    // 69: Lottie animations (`.lottie`, or Bodymovin's `.json`) are vector drawings that play. Packs with a `.lottie`,
    // or a `.json` in a folder or with a name that says Lottie or Bodymovin, are read again.
    "UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id
                       AND (lower(a.ext) = 'lottie'
                            OR lower(a.ext) = 'json' AND (lower(a.path) LIKE '%lottie%' OR lower(a.path) LIKE '%bodymovin%')))
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 70: Rive files (`.riv`) are vector animations played in the browser. Packs with one are read again.
    "UPDATE packs SET status = 'ingesting'
       WHERE status = 'ready' AND archived_at IS NULL
         AND EXISTS (SELECT 1 FROM assets a WHERE a.pack_id = packs.id AND lower(a.ext) = 'riv')
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = packs.id AND j.status IN ('queued', 'running'));
     INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
       SELECT 'retag', id, '{\"then_llm\":false}', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
              strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
       FROM packs p WHERE p.status = 'ingesting'
         AND NOT EXISTS (SELECT 1 FROM jobs j WHERE j.pack_id = p.id AND j.status IN ('queued', 'running'));",
    // 71: sound search (CLAP): each sound file's embedding.
    "CREATE TABLE sound_embeddings (hash TEXT PRIMARY KEY, model TEXT NOT NULL, vector BLOB NOT NULL) WITHOUT ROWID;",
    // 72: near copies (see `nearcopies`): a coarse print of each thumbnail and sound, filled in by their jobs.
    "ALTER TABLE looks ADD COLUMN print BLOB;
     ALTER TABLE sounds ADD COLUMN print BLOB;",
    // 73: image search moves to SigLIP 2, and searches keep using CLIP's embeddings until every thumbnail is
    // relearned, so a thumbnail can have one of each.
    "CREATE TABLE embeddings_by_model (
       thumb_hash TEXT NOT NULL,
       model      TEXT NOT NULL,
       vector     BLOB NOT NULL,
       PRIMARY KEY (thumb_hash, model)
     ) WITHOUT ROWID;
     INSERT INTO embeddings_by_model (thumb_hash, model, vector) SELECT thumb_hash, model, vector FROM embeddings;
     DROP TABLE embeddings;
     ALTER TABLE embeddings_by_model RENAME TO embeddings;",
    // 74: watched packs: the modified time of each folder in them, so a rescan can tell the packs whose files were
    // added, removed or renamed without listing them.
    "ALTER TABLE packs ADD COLUMN dir_times TEXT;",
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

fn user_version(conn: &Connection) -> Result<i64> {
    Ok(conn.query_row("PRAGMA user_version", [], |r| r.get(0))?)
}

pub(super) fn migrate(conn: &mut Connection, backups: &Path, steps: &[&str]) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{get_pack, open, pack_facing};

    const SCHEMA_VERSION: i64 = BASELINE_VERSION + MIGRATIONS.len() as i64;

    fn backups_in(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .map_or_default(|rd| rd.filter_map(|e| e.ok()?.file_name().into_string().ok()).collect());
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
               ('a3', 'p1', 'h3', 'a3.wav', 'a3', 'wav', 'audio', 'My favorite.', 1, 'x', 'x');
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
        assert_eq!(descriptions("a3"), (Some("My favorite.".into()), None, None));
        let name: String = conn.query_row("SELECT name FROM packs WHERE id = 'p1'", [], |r| r.get(0)).unwrap();
        assert_eq!(name, "Forest");
        let settings: i64 = conn.query_row("SELECT COUNT(*) FROM settings", [], |r| r.get(0)).unwrap();
        assert_eq!(settings, 0);
    }

    /// Models whose files say they have no clips, or no skin, lose the animated or rigged their folders gave them.
    #[test]
    fn folder_guesses_the_file_contradicts_are_taken_back() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..49]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('cap', 'p1', 'h1', 'Animated/cap.fbx', 'cap', 'fbx', 'model',
                '{\"has_animation\":false,\"has_deformers\":false}', 'x', 'x'),
               ('body', 'p1', 'h2', 'Animated/body.fbx', 'body', 'fbx', 'model',
                '{\"has_animation\":false,\"has_deformers\":true}', 'x', 'x'),
               ('door', 'p1', 'h3', 'Animated/door.glb', 'door', 'glb', 'model', '{\"animations\":1}', 'x', 'x'),
               ('src', 'p1', 'h4', 'Animated/src.blend', 'src', 'blend', 'model', '{}', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('cap', 'animated', 'rule', 0.6), ('cap', 'rigged', 'rule', 0.6), ('body', 'animated', 'rule', 0.6),
               ('body', 'rigged', 'meta', 0.9), ('door', 'animated', 'rule', 0.6), ('src', 'animated', 'rule', 0.6);
             INSERT INTO assets_fts (asset_id, name, tags) VALUES ('cap', 'cap', 'animated rigged');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
        let tags = |id: &str| -> String {
            conn.query_row(
                "SELECT COALESCE(group_concat(tag, ','), '')
                   FROM (SELECT tag FROM asset_tags WHERE asset_id = ?1 ORDER BY tag)",
                [id],
                |r| r.get(0),
            )
            .unwrap()
        };
        assert_eq!(tags("cap"), "");
        assert_eq!(tags("body"), "rigged");
        assert_eq!(tags("door"), "animated");
        assert_eq!(tags("src"), "animated");
        let stale: i64 = conn
            .query_row("SELECT count(*) FROM assets_fts WHERE assets_fts MATCH 'animated'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(stale, 0);
    }

    /// Sounds named after a title that was only a number are named after their files instead, unless someone named
    /// them, and a title in letters no number could be stays.
    #[test]
    fn names_that_are_only_a_number_become_the_file_name() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..54]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at,
                                 human_edited) VALUES
               ('a', 'p1', 'h1', 'OGG/footstep00.ogg', '1', 'ogg', 'audio', '{}', 'x', 'x', 0),
               ('b', 'p1', 'h2', 'Door_Creak-2.WAV', '07', 'wav', 'audio', '{}', 'x', 'x', 0),
               ('c', 'p1', 'h3', 'OGG/footstep01.ogg', '2', 'ogg', 'audio', '{}', 'x', 'x', 1),
               ('d', 'p1', 'h4', 'rain.ogg', '雨', 'ogg', 'audio', '{}', 'x', 'x', 0),
               ('e', 'p1', 'h5', '01.ogg', '1', 'ogg', 'audio', '{}', 'x', 'x', 0);
             INSERT INTO assets_fts (asset_id, name) VALUES ('a', '1');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
        let name = |id: &str| -> String {
            conn.query_row("SELECT name FROM assets WHERE id = ?1", [id], |r| r.get(0)).unwrap()
        };
        assert_eq!(name("a"), "Footstep 00");
        assert_eq!(name("b"), "Door creak 2");
        assert_eq!(name("c"), "2");
        assert_eq!(name("d"), "雨");
        assert_eq!(name("e"), "1");
        let found: String = conn
            .query_row("SELECT asset_id FROM assets_fts WHERE assets_fts MATCH 'footstep'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(found, "a");
    }

    /// Favorites keep their stars across the table's new name, and drawings become vector art, taken by the
    /// locations that took them as textures.
    #[test]
    fn favorites_and_vector_art_survive_their_upgrades() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        // Through 42 (the list starts at 9), before favorites were renamed.
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..34]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO users (id, username, password_hash, created_at) VALUES (1, 'ann', '', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, category, created_at, updated_at) VALUES
               ('svg', 'p1', 'h1', 'icon.svg', 'icon', 'svg', 'texture', 'textures', 'x', 'x'),
               ('png', 'p1', 'h2', 'icon.png', 'icon', 'png', 'texture', 'textures', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('svg', 'kind:texture', 'meta', 1.0), ('png', 'kind:texture', 'meta', 1.0);
             INSERT INTO favourites (user_id, asset_id, added_at) VALUES (1, 'svg', 'x');
             INSERT INTO locations (name, mode, path, kinds, created_at) VALUES
               ('Pictures', 'managed', '/p', '[\"texture\"]', 'x'), ('Sounds', 'managed', '/s', '[\"audio\"]', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
        let row = |id: &str| -> (String, String, String) {
            conn.query_row(
                "SELECT kind, category, (SELECT group_concat(tag) FROM asset_tags WHERE asset_id = a.id)
                   FROM assets a WHERE id = ?1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap()
        };
        assert_eq!(row("svg"), ("vector".into(), "vector".into(), "kind:vector".into()));
        assert_eq!(row("png"), ("texture".into(), "textures".into(), "kind:texture".into()));
        let found: String = conn
            .query_row("SELECT asset_id FROM assets_fts WHERE assets_fts MATCH 'vector'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(found, "svg");
        let kinds: Vec<String> = conn
            .prepare("SELECT kinds FROM locations ORDER BY id")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(kinds, [r#"["texture","vector"]"#, r#"["audio"]"#]);
        let stars: i64 =
            conn.query_row("SELECT COUNT(*) FROM favorites WHERE asset_id = 'svg'", [], |r| r.get(0)).unwrap();
        assert_eq!(stars, 1);
    }

    /// Packs with ASCII FBX or 3DS files are read again; FBX files Blender refused are converted again, and the 3DS
    /// failures go, since Blender isn't asked about 3DS any more.
    #[test]
    fn ascii_fbx_and_3ds_packs_are_read_again() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..37]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('kit', 'Space Kit', 'ready', 'x'), ('iso', 'Isometric', 'ready', 'x'), ('obj', 'Plain', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('a', 'kit', 'h1', 'Models/station.fbx', 'station', 'fbx', 'model', '{\"fbx_ascii\":true}', 'x', 'x'),
               ('b', 'iso', 'h2', 'car.3ds', 'car', '3ds', 'model', '{}', 'x', 'x'),
               ('c', 'obj', 'h3', 'rock.fbx', 'rock', 'fbx', 'model', '{}', 'x', 'x');
             INSERT INTO issues (key, kind, pack_id, subject, message, first_at, last_at) VALUES
               ('k1', 'convert', 'kit', 'Models/station.fbx', 'Blender exited', 'x', 'x'),
               ('k2', 'convert', 'iso', 'car.3ds', 'Blender exited', 'x', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..38]).unwrap();
        let jobs: Vec<(String, String)> = conn
            .prepare("SELECT kind, pack_id FROM jobs WHERE pack_id <> 'p1' ORDER BY pack_id, kind")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        let jobs: Vec<(&str, &str)> = jobs.iter().map(|(k, p)| (k.as_str(), p.as_str())).collect();
        assert_eq!(jobs, [("retag", "iso"), ("convert", "kit"), ("retag", "kit")]);
        let issues: Vec<String> = conn
            .prepare("SELECT subject FROM issues")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .flatten()
            .collect();
        assert_eq!(issues, ["Models/station.fbx"]);
    }

    /// Packs with images and a file that may be an atlas are looked through for sprites.
    #[test]
    fn packs_that_may_have_atlases_are_looked_through() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..38]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('sheet', 'Sheets', 'ready', 'x'), ('docs', 'Notes', 'ready', 'x'), ('busy', 'Busy', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('a', 'sheet', 'h1', 'sheet.xml', 'sheet', 'xml', 'data', '{}', 'x', 'x'),
               ('b', 'sheet', 'h2', 'sheet.png', 'sheet', 'png', 'texture', '{}', 'x', 'x'),
               ('c', 'docs', 'h3', 'notes.json', 'notes', 'json', 'data', '{}', 'x', 'x'),
               ('d', 'busy', 'h4', 'a.atlas', 'a', 'atlas', 'data', '{}', 'x', 'x'),
               ('e', 'busy', 'h5', 'a.png', 'a', 'png', 'texture', '{}', 'x', 'x');
             INSERT INTO jobs (kind, pack_id, payload, run_after, created_at, updated_at)
               VALUES ('ingest', 'busy', '{}', 'x', 'x', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..39]).unwrap();
        let jobs: Vec<(String, String)> = conn
            .prepare("SELECT kind, pack_id FROM jobs WHERE pack_id <> 'p1' ORDER BY pack_id, kind")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        let jobs: Vec<(&str, &str)> = jobs.iter().map(|(k, p)| (k.as_str(), p.as_str())).collect();
        assert_eq!(jobs, [("ingest", "busy"), ("materials", "sheet")]);
    }

    /// Packs with a glTF or GLB model that has no thumbnail are analyzed again, now meshopt-compressed ones are drawn.
    #[test]
    fn gltf_packs_without_thumbnails_are_read_again() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..39]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('opt', 'Compressed', 'ready', 'x'), ('drawn', 'Drawn', 'ready', 'x'), ('clip', 'Clips', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, thumb_hash, created_at, updated_at)
             VALUES
               ('a', 'opt', 'h1', 'ship.glb', 'ship', 'glb', 'model', '{}', NULL, 'x', 'x'),
               ('b', 'drawn', 'h2', 'rock.gltf', 'rock', 'gltf', 'model', '{}', 't2', 'x', 'x'),
               ('c', 'clip', 'h3', 'walk.glb', 'walk', 'glb', 'animation', '{}', NULL, 'x', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..40]).unwrap();
        let jobs: Vec<(String, String)> = conn
            .prepare("SELECT kind, pack_id FROM jobs WHERE pack_id <> 'p1' ORDER BY pack_id, kind")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        let jobs: Vec<(&str, &str)> = jobs.iter().map(|(k, p)| (k.as_str(), p.as_str())).collect();
        assert_eq!(jobs, [("retag", "opt")]);
    }

    /// Blender's GLBs of formats Hoard reads itself are dropped, and the browser thumbnails drawn from them, or from
    /// an OBJ, are drawn again; a .blend keeps its GLB and its conversion failures.
    #[test]
    fn blender_glbs_of_formats_hoard_reads_are_dropped() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..40]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('astro', 'Astronaut', 'ready', 'x'), ('scene', 'Scenes', 'ready', 'x'), ('city', 'City', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, thumb_source, created_at,
                                 updated_at) VALUES
               ('a', 'astro', 'h1', 'Astronaut.fbx', 'Astronaut', 'fbx', 'model', '{}', 'browser', 'x', 'x'),
               ('b', 'scene', 'h2', 'room.blend', 'room', 'blend', 'model', '{}', 'browser', 'x', 'x'),
               ('c', 'city', 'h3', 'road.obj', 'road', 'obj', 'model', '{}', 'browser-partial', 'x', 'x');
             INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES
               ('a', 'glb', 'g1', 'x'), ('b', 'glb', 'g2', 'x');
             INSERT INTO issues (key, kind, pack_id, subject, message, first_at, last_at) VALUES
               ('k1', 'convert', 'astro', 'Helmet.FBX', 'Blender exited', 'x', 'x'),
               ('k2', 'convert', 'scene', 'hall.blend', 'Blender exited', 'x', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..41]).unwrap();
        let strings = |sql: &str| -> Vec<String> {
            conn.prepare(sql).unwrap().query_map([], |r| r.get(0)).unwrap().flatten().collect()
        };
        assert_eq!(strings("SELECT asset_id FROM derivatives"), ["b"]);
        assert_eq!(
            strings("SELECT id || ':' || coalesce(thumb_source, '-') FROM assets ORDER BY id"),
            ["a:-", "b:browser", "c:-"]
        );
        assert_eq!(
            strings("SELECT kind || ':' || pack_id FROM jobs WHERE pack_id <> 'p1' ORDER BY pack_id"),
            ["retag:astro", "retag:city"]
        );
        assert_eq!(strings("SELECT subject FROM issues"), ["hall.blend"]);
    }

    /// Packs with Unity materials or guessed textures are read again, and a browser thumbnail drawn with a guessed
    /// texture on is drawn again.
    #[test]
    fn packs_with_unity_materials_or_guessed_textures_are_read_again() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..41]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('unity', 'Unity', 'ready', 'x'), ('kit', 'Kit', 'ready', 'x'), ('plain', 'Plain', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, thumb_source, created_at,
                                 updated_at) VALUES
               ('m', 'unity', 'h1', 'Wood.mat', 'Wood', 'mat', 'other', '{}', NULL, 'x', 'x'),
               ('s', 'kit', 'h2', 'stairs.fbx', 'stairs', 'fbx', 'model', '{}', 'browser', 'x', 'x'),
               ('t', 'kit', 'h3', 'stairs_0.png', 'stairs 0', 'png', 'texture', '{}', NULL, 'x', 'x'),
               ('c', 'plain', 'h4', 'crate.obj', 'crate', 'obj', 'model', '{}', 'browser', 'x', 'x');
             INSERT INTO asset_links (from_id, to_id, rel) VALUES ('s', 't', 'guess');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..42]).unwrap();
        let strings = |sql: &str| -> Vec<String> {
            conn.prepare(sql).unwrap().query_map([], |r| r.get(0)).unwrap().flatten().collect()
        };
        assert_eq!(
            strings("SELECT id || ':' || coalesce(thumb_source, '-') FROM assets WHERE kind = 'model' ORDER BY id"),
            ["c:browser", "s:-"]
        );
        assert_eq!(
            strings("SELECT kind || ':' || pack_id FROM jobs WHERE pack_id <> 'p1' ORDER BY pack_id"),
            ["retag:kit", "retag:unity"]
        );
    }

    /// Models that might have been painted with a texture found by name are drawn again, and their GLB downloads
    /// made again.
    #[test]
    fn models_named_like_a_texture_are_drawn_again() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..42]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('kit', 'Kit', 'ready', 'x'), ('plain', 'Plain', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('s', 'kit', 'h1', 'Kit3D/Models/Stairs.fbx', 'Stairs', 'fbx', 'model', '{}', 'x', 'x'),
               ('t', 'kit', 'h2', 'Kit2D/stairs_0.png', 'stairs 0', 'png', 'texture', '{}', 'x', 'x'),
               ('c', 'plain', 'h3', 'crate.obj', 'crate', 'obj', 'model', '{}', 'x', 'x'),
               ('w', 'plain', 'h4', 'wood.png', 'wood', 'png', 'texture', '{}', 'x', 'x');
             INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES
               ('s', 'as:glb:0', 'g1', 'x'), ('s', 'as:stl:0', 'g2', 'x'), ('c', 'as:glb:0', 'g3', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..43]).unwrap();
        let strings = |sql: &str| -> Vec<String> {
            conn.prepare(sql).unwrap().query_map([], |r| r.get(0)).unwrap().flatten().collect()
        };
        assert_eq!(
            strings("SELECT asset_id || ':' || kind FROM derivatives ORDER BY asset_id, kind"),
            ["c:as:glb:0", "s:as:stl:0"]
        );
        assert_eq!(strings("SELECT kind || ':' || pack_id FROM jobs WHERE pack_id <> 'p1'"), ["retag:kit"]);
    }

    /// Packs with `.mtl` files look for materials again.
    #[test]
    fn packs_with_mtl_files_look_for_materials_again() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..43]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('city', 'City', 'ready', 'x'), ('plain', 'Plain', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('m', 'city', 'h1', 'road.mtl', 'road', 'mtl', 'material', '{}', 'x', 'x'),
               ('w', 'plain', 'h2', 'wood.png', 'wood', 'png', 'texture', '{}', 'x', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..44]).unwrap();
        let jobs: Vec<String> = conn
            .prepare("SELECT kind || ':' || pack_id FROM jobs WHERE pack_id <> 'p1'")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .flatten()
            .collect();
        assert_eq!(jobs, ["materials:city"]);
    }

    /// Packs with Unity `.mat` files look for materials again, unless already about to.
    #[test]
    fn packs_with_unity_materials_look_for_materials_again() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..44]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('kit', 'Kit', 'ready', 'x'), ('busy', 'Busy', 'ready', 'x'), ('plain', 'Plain', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('m', 'kit', 'h1', 'Wood.mat', 'Wood', 'mat', 'other', '{}', 'x', 'x'),
               ('b', 'busy', 'h2', 'Rock.mat', 'Rock', 'mat', 'other', '{}', 'x', 'x'),
               ('w', 'plain', 'h3', 'wood.png', 'wood', 'png', 'texture', '{}', 'x', 'x');
             INSERT INTO jobs (kind, pack_id, payload, status, run_after, created_at, updated_at)
               VALUES ('materials', 'busy', '{}', 'queued', 'x', 'x', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..45]).unwrap();
        let jobs: Vec<String> = conn
            .prepare("SELECT kind || ':' || pack_id FROM jobs WHERE pack_id <> 'p1' ORDER BY id")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .flatten()
            .collect();
        assert_eq!(jobs, ["materials:busy", "materials:kit"]);
        conn.execute("INSERT INTO unity_remaps VALUES ('kit', 'Crate.fbx', 'lambert2', 'g')", []).unwrap();
    }

    /// Every 3D viewer and GLB download is made again, and models that might have been painted with a texture found
    /// by name are drawn again.
    #[test]
    fn viewers_and_glbs_are_made_again_and_guessed_models_drawn_again() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..45]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('kit', 'Kit', 'ready', 'x'), ('plain', 'Plain', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('s', 'kit', 'h1', 'Kit3D/Models/Stairs.fbx', 'Stairs', 'fbx', 'model', '{}', 'x', 'x'),
               ('t', 'kit', 'h2', 'Kit2D/stairs_0.png', 'stairs 0', 'png', 'texture', '{}', 'x', 'x'),
               ('c', 'plain', 'h3', 'tree.glb', 'tree', 'glb', 'model', '{}', 'x', 'x'),
               ('w', 'plain', 'h4', 'wood.png', 'wood', 'png', 'texture', '{}', 'x', 'x');
             INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES
               ('s', 'as:glb:0:glb', 'g1', 'x'), ('s', 'as:stl:0:stl', 'g2', 'x'), ('c', 'as:view:0:glb', 'g3', 'x'),
               ('w', 'as:webp:0:webp', 'g4', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..46]).unwrap();
        let strings = |sql: &str| -> Vec<String> {
            conn.prepare(sql).unwrap().query_map([], |r| r.get(0)).unwrap().flatten().collect()
        };
        assert_eq!(
            strings("SELECT asset_id || ':' || kind FROM derivatives ORDER BY asset_id, kind"),
            ["s:as:stl:0:stl", "w:as:webp:0:webp"]
        );
        assert_eq!(strings("SELECT kind || ':' || pack_id FROM jobs WHERE pack_id <> 'p1'"), ["retag:kit"]);
    }

    #[test]
    fn stl_models_are_drawn_again_standing_up() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..46]).unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('print', 'Print', 'ready', 'x'), ('plain', 'Plain', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('s', 'print', 'h1', 'bracket.stl', 'bracket', 'stl', 'model', '{}', 'x', 'x'),
               ('c', 'plain', 'h3', 'tree.glb', 'tree', 'glb', 'model', '{}', 'x', 'x');
             INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES
               ('s', 'as:glb:0:glb', 'g1', 'x'), ('s', 'as:view:0:glb', 'g2', 'x'), ('c', 'as:view:0:glb', 'g3', 'x');",
        )
        .unwrap();
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..47]).unwrap();
        let strings = |sql: &str| -> Vec<String> {
            conn.prepare(sql).unwrap().query_map([], |r| r.get(0)).unwrap().flatten().collect()
        };
        assert_eq!(strings("SELECT asset_id || ':' || kind FROM derivatives"), ["c:as:view:0:glb"]);
        assert_eq!(strings("SELECT kind || ':' || pack_id FROM jobs WHERE pack_id <> 'p1'"), ["retag:print"]);
    }

    #[test]
    fn packs_face_their_models_forward_until_turned() {
        let dir = tempfile::tempdir().unwrap();
        let mut conn = baseline_db(dir.path());
        migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..48]).unwrap();
        assert!(pack_facing(&conn, "p1").unwrap().is_upright());
        conn.execute("UPDATE packs SET model_turn = 180, model_tilt = 270 WHERE id = 'p1'", []).unwrap();
        assert_eq!(pack_facing(&conn, "p1").unwrap(), hoard_core::Facing { turn: 180, tilt: 270 });
        assert_eq!(get_pack(&conn, "p1").unwrap().unwrap().facing.turn, 180);
        assert!(pack_facing(&conn, "missing").unwrap().is_upright());
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
