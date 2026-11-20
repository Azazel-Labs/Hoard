use super::*;
use crate::db::{get_pack, open, pack_facing};

const SCHEMA_VERSION: i64 = BASELINE_VERSION + MIGRATIONS.len() as i64;

fn backups_in(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> =
        std::fs::read_dir(dir).map_or_default(|rd| rd.filter_map(|e| e.ok()?.file_name().into_string().ok()).collect());
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
        conn.query_row("SELECT description, source_description, ai_description FROM assets WHERE id = ?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
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
    let stale: i64 =
        conn.query_row("SELECT count(*) FROM assets_fts WHERE assets_fts MATCH 'animated'", [], |r| r.get(0)).unwrap();
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
    let name =
        |id: &str| -> String { conn.query_row("SELECT name FROM assets WHERE id = ?1", [id], |r| r.get(0)).unwrap() };
    assert_eq!(name("a"), "Footstep 00");
    assert_eq!(name("b"), "Door creak 2");
    assert_eq!(name("c"), "2");
    assert_eq!(name("d"), "雨");
    assert_eq!(name("e"), "1");
    let found: String =
        conn.query_row("SELECT asset_id FROM assets_fts WHERE assets_fts MATCH 'footstep'", [], |r| r.get(0)).unwrap();
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
    let found: String =
        conn.query_row("SELECT asset_id FROM assets_fts WHERE assets_fts MATCH 'vector'", [], |r| r.get(0)).unwrap();
    assert_eq!(found, "svg");
    let kinds: Vec<String> = conn
        .prepare("SELECT kinds FROM locations ORDER BY id")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    assert_eq!(kinds, [r#"["texture","vector"]"#, r#"["audio"]"#]);
    let stars: i64 = conn.query_row("SELECT COUNT(*) FROM favorites WHERE asset_id = 'svg'", [], |r| r.get(0)).unwrap();
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
    let issues: Vec<String> =
        conn.prepare("SELECT subject FROM issues").unwrap().query_map([], |r| r.get(0)).unwrap().flatten().collect();
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
    // A pack's card counts what it holds from what later steps keep.
    migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
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

/// Search index rows are found by number from then on: an asset's row is noted, rows of assets that are gone and
/// second rows of one are dropped, and deleting an asset, or its pack, takes its row with it.
#[test]
fn search_rows_are_kept_by_number() {
    let dir = tempfile::tempdir().unwrap();
    let mut conn = baseline_db(dir.path());
    let numbered = MIGRATIONS.iter().position(|m| m.contains("ADD COLUMN fts_row")).unwrap();
    migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..numbered]).unwrap();
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('rock', 'p1', 'h1', 'rock.png', 'rock', 'png', 'texture', 'x', 'x'),
               ('moss', 'p1', 'h2', 'moss.png', 'moss', 'png', 'texture', 'x', 'x'),
               ('bare', 'p1', 'h3', 'bare.png', 'bare', 'png', 'texture', 'x', 'x');
             INSERT INTO assets_fts (asset_id, name) VALUES ('rock', 'rock'), ('gone', 'gone'), ('moss', 'old moss'),
               ('moss', 'moss');",
    )
    .unwrap();
    migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
    conn.execute_batch("PRAGMA foreign_keys = ON").unwrap();
    let rows = |c: &Connection| -> Vec<(String, String)> {
        let mut stmt = c.prepare("SELECT asset_id, name FROM assets_fts ORDER BY asset_id").unwrap();
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?))).unwrap().collect::<rusqlite::Result<_>>().unwrap()
    };
    let pair = |a: &str, b: &str| (a.to_owned(), b.to_owned());
    assert_eq!(rows(&conn), [pair("moss", "moss"), pair("rock", "rock")]);
    let noted: i64 = conn.query_row("SELECT count(fts_row) FROM assets", [], |r| r.get(0)).unwrap();
    assert_eq!(noted, 2);

    // Reindexing replaces the row it has, or adds one; deleting takes it away.
    conn.execute("UPDATE assets SET name = 'boulder' WHERE id = 'rock'", []).unwrap();
    crate::db::reindex_asset(&conn, "rock").unwrap();
    crate::db::reindex_asset(&conn, "bare").unwrap();
    crate::db::reindex_asset(&conn, "bare").unwrap();
    assert_eq!(rows(&conn), [pair("bare", "bare"), pair("moss", "moss"), pair("rock", "boulder")]);
    conn.execute("DELETE FROM assets WHERE id = 'moss'", []).unwrap();
    assert_eq!(rows(&conn), [pair("bare", "bare"), pair("rock", "boulder")]);
    conn.execute("DELETE FROM packs WHERE id = 'p1'", []).unwrap();
    assert_eq!(rows(&conn), []);
}

/// A take a browser drew (on another character, or on nothing) has its file's picture back, to be drawn again; one the
/// server drew, and files that aren't takes, keep theirs.
#[test]
fn takes_drawn_by_a_browser_get_their_files_picture_back() {
    let dir = tempfile::tempdir().unwrap();
    let mut conn = baseline_db(dir.path());
    let reset = MIGRATIONS.iter().position(|m| m.contains("take_of') IS NOT NULL AND thumb_source IN")).unwrap();
    migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..reset]).unwrap();
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, thumb_hash, thumb_source,
                             created_at, updated_at) VALUES
           ('hero', 'p1', 'h1', 'hero.fbx', 'hero', 'fbx', 'model', '{}', 'hero-pic', 'browser', 'x', 'x'),
           ('kick', 'p1', 'h2', 'hero@Kick.glb', 'Kick', 'glb', 'animation', '{\"take_of\":\"hero\"}', 'blank', 'browser', 'x', 'x'),
           ('walk', 'p1', 'h3', 'hero@Walk.glb', 'Walk', 'glb', 'animation', '{\"take_of\":\"hero\"}', 'served', NULL, 'x', 'x');",
    )
    .unwrap();
    migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
    let thumb = |id: &str| -> (Option<String>, Option<String>) {
        conn.query_row("SELECT thumb_hash, thumb_source FROM assets WHERE id = ?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .unwrap()
    };
    assert_eq!(thumb("kick"), (Some("hero-pic".into()), None));
    assert_eq!(thumb("walk"), (Some("served".into()), None));
    assert_eq!(thumb("hero"), (Some("hero-pic".into()), Some("browser".into())));
}

#[test]
fn what_was_made_from_an_animated_fbx_is_made_again() {
    let dir = tempfile::tempdir().unwrap();
    let mut conn = baseline_db(dir.path());
    let fix = MIGRATIONS.iter().position(|m| m.contains("as:thumb.avif:%")).unwrap();
    migrate(&mut conn, &dir.path().join("backups"), &MIGRATIONS[..fix]).unwrap();
    // A character with clips, its take, a still FBX, a clip-less FBX animation's row and a GLB animation.
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
           ('hero', 'p1', 'h1', 'hero.fbx', 'hero', 'fbx', 'model', '{}', 'x', 'x'),
           ('run', 'p1', 'h2', 'hero@Run.glb', 'Run', 'glb', 'animation',
              '{\"take_of\":\"hero\",\"take\":\"Run\",\"take_from\":\"h1\"}', 'x', 'x'),
           ('crate', 'p1', 'h3', 'crate.fbx', 'crate', 'fbx', 'model', '{}', 'x', 'x'),
           ('wave', 'p1', 'h4', 'wave.fbx', 'wave', 'fbx', 'animation', '{}', 'x', 'x'),
           ('jump', 'p1', 'h5', 'jump.glb', 'jump', 'glb', 'animation', '{}', 'x', 'x');
         INSERT INTO rigs (asset_id, rig) VALUES
           ('hero', '{\"clips\":[{\"name\":\"Run\",\"secs\":1.0},{\"name\":\"Walk\",\"secs\":1.0}]}'),
           ('crate', '{\"clips\":[]}'),
           ('wave', '{\"clips\":[{\"name\":\"Wave\",\"secs\":2.0}]}'),
           ('jump', '{\"clips\":[{\"name\":\"Jump\",\"secs\":1.0}]}');
         INSERT INTO derivatives (asset_id, kind, blob_hash, created_at) VALUES
           ('hero', 'as:view:0:glb', 'd1', 'x'), ('hero', 'as:glb~0~0~n:0', 'd2', 'x'),
           ('hero', 'as:thumb.avif:abc', 'd3', 'x'),
           ('crate', 'as:view:0:glb', 'd4', 'x'),
           ('wave', 'as:view:0:glb', 'd5', 'x'), ('wave', 'as:clip:0', 'd6', 'x'),
           ('wave', 'motion', 'd7', 'x'), ('wave', 'still', 'd8', 'x'),
           ('run', 'as:view:0:glb', 'd9', 'x'), ('run', 'motion', 'd10', 'x'),
           ('jump', 'as:view:0:glb', 'd11', 'x'), ('jump', 'motion', 'd12', 'x');",
    )
    .unwrap();
    migrate(&mut conn, &dir.path().join("backups"), MIGRATIONS).unwrap();
    let strings = |sql: &str| -> Vec<String> {
        conn.prepare(sql).unwrap().query_map([], |r| r.get(0)).unwrap().flatten().collect()
    };
    // The still FBX, the GLB animation and the thumbnail keep theirs. The take's are dropped when it's made again,
    // which its file's contents, no longer the ones it was made from, will have happen.
    assert_eq!(
        strings("SELECT asset_id || ' ' || kind FROM derivatives ORDER BY asset_id, kind"),
        [
            "crate as:view:0:glb",
            "hero as:thumb.avif:abc",
            "jump as:view:0:glb",
            "jump motion",
            "run as:view:0:glb",
            "run motion"
        ]
    );
    assert_eq!(
        strings(
            "SELECT json_extract(meta, '$.take_of') || ' ' || json_extract(meta, '$.take_from') FROM assets
                  WHERE id = 'run'"
        ),
        ["hero "]
    );
}
