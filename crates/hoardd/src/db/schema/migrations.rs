//! Every change to the schema since the baseline, in order.

/// Changes since the baseline, oldest first. Step `i` takes the database from `BASELINE_VERSION + i` to
/// `BASELINE_VERSION + i + 1`, inside one transaction, so a failing step leaves the database untouched.
///
/// **Append only.** Never edit, reorder or remove a step once it has shipped: databases out there have run it.
/// And don't change [`BASELINE`](super::baseline::BASELINE) either; a new table, column or index is a new step here. A step can be any SQL
/// SQLite accepts in a transaction, e.g. `ALTER TABLE packs ADD COLUMN notes TEXT;` or a table rebuild.
pub(in crate::db) const MIGRATIONS: &[&str] = &[
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
    // 75: each take of a file of many is an asset of its own (see `takes`), found by the file it came from.
    "CREATE INDEX assets_take_of ON assets (json_extract(meta, '$.take_of'));",
    // 76: each person's dragon hoard (see `me`): how many assets of each kind they've collected, as JSON.
    "ALTER TABLE users ADD COLUMN hoard_seen TEXT;",
];
