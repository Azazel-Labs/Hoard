use super::facts::*;
use super::tag_rows::*;
use super::*;
use crate::db::{open, test_library};

#[test]
fn counts_are_kept_until_the_library_changes() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('a', 'p1', 'h', 'a.png', 'a', 'png', 'texture', 'x', 'x');",
    )
    .unwrap();
    let q = SearchQuery { kind: Some("texture".into()), ..Default::default() };
    let counted = |page: u32| {
        let found = search(&conn, &SearchQuery { offset: Some(page), ..q.clone() }, true).unwrap();
        (found.total, found.kinds.iter().map(|k| (k.value.clone(), k.count)).collect::<Vec<_>>())
    };
    assert_eq!(counted(0), (1, vec![("texture".to_owned(), 1)]));
    // Written on another connection: the next page counts again.
    pool.get()
        .unwrap()
        .execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('b', 'p1', 'h', 'b.png', 'b', 'png', 'texture', 'x', 'x')",
            [],
        )
        .unwrap();
    assert_eq!(counted(60), (2, vec![("texture".to_owned(), 2)]));
}

#[test]
fn tag_rows_put_right_in_place_are_as_if_read_again() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('a', 'p1', 'h', 'a.png', 'a', 'png', 'texture', 'x', 'x'),
           ('b', 'p1', 'h', 'b.png', 'b', 'png', 'texture', 'x', 'x'),
           ('c', 'p1', 'h', 'c.png', 'c', 'png', 'texture', 'x', 'x');
         INSERT INTO asset_tags (asset_id, tag, source) VALUES
           ('a', 'red', 'x'), ('b', 'blue', 'x'), ('c', 'red', 'x'), ('c', 'set:one', 'x');
         INSERT INTO tag_parents (tag, parent) VALUES
           ('red', 'warm'), ('blue', 'cool'), ('warm', 'colored'), ('cool', 'colored');",
    )
    .unwrap();
    let listed = |rows: &TagRows| {
        let mut all: Vec<(String, Vec<u32>)> = rows
            .all()
            .map(|(t, r)| {
                let mut r = r.to_vec();
                r.sort_unstable();
                (t.to_owned(), r)
            })
            .collect();
        all.sort();
        all
    };
    let put_right_after = |kept: &mut TagRows, changes: &str| {
        let before = crate::db::library_writes();
        conn.execute_batch(changes).unwrap();
        let changed = crate::db::changes::since(before, crate::db::library_writes()).unwrap();
        assert!(!changed.too_much);
        let tree = Tree::load(&conn).unwrap();
        put_right(&conn, kept, &changed, &tree).unwrap();
        assert_eq!(listed(kept), listed(&read_tag_rows(&conn, &tree).unwrap()), "after {changes}");
    };
    let mut kept = read_tag_rows(&conn, &Tree::load(&conn).unwrap()).unwrap();
    assert!(listed(&kept).iter().any(|(t, r)| t == "colored" && r.len() == 3));
    // Tags added and taken off, an asset removed (its tags with it) and one added.
    put_right_after(
        &mut kept,
        "INSERT INTO asset_tags (asset_id, tag, source) VALUES ('a', 'blue', 'x');
         DELETE FROM asset_tags WHERE asset_id = 'c' AND tag = 'red';
         DELETE FROM assets WHERE id = 'b';
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('d', 'p1', 'h', 'd.png', 'd', 'png', 'texture', 'x', 'x');
         INSERT INTO asset_tags (asset_id, tag, source) VALUES ('d', 'red', 'x'), ('d', 'moss', 'x');",
    );
    // The tree changing: moss is put under cool, and red is no longer warm.
    put_right_after(
        &mut kept,
        "INSERT INTO tag_parents (tag, parent) VALUES ('moss', 'cool');
         DELETE FROM tag_parents WHERE tag = 'red';",
    );
    // A tag renamed in place.
    put_right_after(&mut kept, "UPDATE asset_tags SET tag = 'moss' WHERE asset_id = 'a' AND tag = 'blue';");
}

#[test]
fn the_library_goes_in_order_by_when_assets_came_or_changed_either_way() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('s', 1, 'x'), ('m', 2, 'x'), ('l', 3, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('a', 'p1', 'm', 'a.png', 'Bark', 'png', 'texture', '2026-01-01', '2026-03-01'),
           ('b', 'p1', 'l', 'b.png', 'apple', 'png', 'texture', '2026-02-01', '2026-02-01'),
           ('c', 'p1', 's', 'c.png', 'Cliff', 'png', 'texture', '2026-03-01', '2026-03-02');
         UPDATE assets SET rating = 3 WHERE id = 'a';
         UPDATE assets SET rating = 5 WHERE id = 'c';",
    )
    .unwrap();
    let ids = |sort: &str| {
        let q = SearchQuery { sort: Some(sort.into()), ..Default::default() };
        search(&conn, &q, false).unwrap().items.into_iter().map(|a| a.id).collect::<String>()
    };
    for (sort, ids_in_order) in [
        ("created", "cba"),
        ("created:asc", "abc"),
        ("updated", "cab"),
        ("updated:desc", "cab"),
        ("updated:asc", "bac"),
        ("name", "bac"),
        ("name:desc", "cab"),
        ("size", "bac"),
        ("size:asc", "cab"),
        // What isn't rated comes last either way.
        ("rating", "cab"),
        ("rating:asc", "acb"),
        // As links and saved searches made before have them.
        ("new", "cba"),
        ("old", "abc"),
        // Anything else is the usual order.
        ("created:sideways", "cba"),
    ] {
        assert_eq!(ids(sort), ids_in_order, "{sort}");
    }
}

#[test]
fn a_search_by_words_is_kept_for_its_next_pages_until_the_library_changes() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    let add = |c: &Connection, id: &str, name: &str, kind: &str| {
        c.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
             VALUES (?1, 'p1', 'h', ?1 || '.png', ?2, 'png', ?3, 'x', 'x')",
            params![id, name, kind],
        )
        .unwrap();
        c.execute_batch(&format!(
            "INSERT INTO assets_fts (rowid, asset_id, name, words, tags) SELECT rowid, id, name, '', '' FROM assets
               WHERE id = '{id}';
             UPDATE assets SET fts_row = rowid WHERE id = '{id}';"
        ))
        .unwrap();
    };
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');",
    )
    .unwrap();
    add(&conn, "r1", "rock one", "texture");
    add(&conn, "r2", "rock two", "model");
    add(&conn, "r3", "rock three", "texture");
    add(&conn, "m", "moss", "texture");
    // The model needs the third rock as raw data: it's a supporting file.
    conn.execute("INSERT INTO asset_links (from_id, to_id, rel) VALUES ('r2', 'r3', 'buffer')", []).unwrap();
    let q = SearchQuery {
        q: Some("rock".into()),
        sort: Some("name".into()),
        hide_supporting: true,
        limit: Some(1),
        ..Default::default()
    };
    let page = |q: &SearchQuery, offset: u32| {
        let found = search(&conn, &SearchQuery { offset: Some(offset), ..q.clone() }, false).unwrap();
        let kinds = found.kinds.iter().map(|k| format!("{} {}", k.value, k.count)).collect::<Vec<_>>();
        (found.items.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), found.total, found.supporting_hidden, kinds)
    };
    let both = vec!["model 1".to_owned(), "texture 1".to_owned()];
    assert_eq!(page(&q, 0), (vec!["r1".to_owned()], 2, 1, both.clone()));
    assert_eq!(page(&q, 1), (vec!["r2".to_owned()], 2, 1, both.clone()));
    assert_eq!(page(&q, 2), (vec![], 2, 1, both.clone()));
    let textures = SearchQuery { kind: Some("texture".into()), ..q.clone() };
    assert_eq!(page(&textures, 0), (vec!["r1".to_owned()], 1, 1, both.clone()));
    // Written on another connection: the next page searches again.
    add(&pool.get().unwrap(), "r0", "rock a", "texture");
    assert_eq!(page(&q, 1), (vec!["r1".to_owned()], 3, 1, vec!["texture 2".to_owned(), "model 1".to_owned()]));
}

#[test]
fn the_search_box_reads_or_minus_and_quotes() {
    let w = |q: &str| {
        let w = words(q);
        (w.find.unwrap_or_default(), w.without.unwrap_or_default(), w.plain)
    };
    let s = |a: &str, b: &str, c: &str| (a.to_owned(), b.to_owned(), c.to_owned());
    let own = |t: &str| format!("{{name words category description tags}} : {t}");
    assert_eq!(
        w("Rock moss"),
        s(&format!(r#""rock"* AND "moss"* AND {}"#, own(r#"("rock"* OR "moss"*)"#)), "", "Rock moss")
    );
    assert_eq!(
        w("rock or stone wall"),
        s(
            &format!(r#"("rock"* OR "stone"*) AND "wall"* AND {}"#, own(r#"("rock"* OR "stone"* OR "wall"*)"#)),
            "",
            "rock stone wall"
        )
    );
    assert_eq!(w("rock | stone"), s(&own(r#"("rock"* OR "stone"*)"#), "", "rock stone"));
    assert_eq!(w(r#""dry moss" -wet -"red brick""#), s(&own(r#""dry moss""#), r#""wet"* OR "red brick""#, "dry moss"));
    assert_eq!(w("sci-fi -set:forest"), s(&own(r#""sci fi"*"#), r#""set forest"*"#, "sci-fi"));
    assert_eq!(w("-moss"), s("", r#""moss"*"#, ""));
    // Marks with nothing to go with them are left be.
    assert_eq!(w(r#"or - "" | rock or"#), s(&own(r#""rock"*"#), "", "rock"));
    assert_eq!(
        w(r#"door "and NOT"#),
        s(&format!(r#""door"* AND "and not" AND {}"#, own(r#"("door"* OR "and not")"#)), "", "door and NOT")
    );
    assert_eq!(w("orc OR"), s(&own(r#""orc"*"#), "", "orc"));
    // A pack by the words of its name.
    assert_eq!(w(r#"Pack:"Forest Kit""#), s(r#"pack_name : "forest kit""#, "", ""));
    assert_eq!(
        w("robot pack:forest -pack:sci-fi"),
        s(
            &format!(r#""robot"* AND pack_name : "forest"* AND {}"#, own(r#"("robot"*)"#)),
            r#"pack_name : "sci fi"*"#,
            "robot"
        )
    );
    assert_eq!(w("pack: rock"), s(&own(r#""rock"*"#), "", "rock"));
}

#[test]
fn words_after_a_minus_are_left_out() {
    let conn = test_library();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');",
    )
    .unwrap();
    for (id, name, tags) in
        [("a", "mossy rock", "green"), ("b", "dry rock", "set forest"), ("c", "stone wall", ""), ("d", "wet moss", "")]
    {
        conn.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
             VALUES (?1, 'p1', 'h', ?1 || '.png', ?2, 'png', 'texture', 'x', 'x')",
            params![id, name],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO assets_fts (rowid, asset_id, name, words, tags) SELECT rowid, id, name, '', ?2 FROM assets
               WHERE id = ?1",
            params![id, tags],
        )
        .unwrap();
        conn.execute("UPDATE assets SET fts_row = rowid WHERE id = ?1", [id]).unwrap();
    }
    let found = |text: &str| {
        let mut ids = search(&conn, &SearchQuery { q: Some(text.into()), ..Default::default() }, false)
            .unwrap()
            .items
            .into_iter()
            .map(|a| a.id)
            .collect::<Vec<_>>();
        ids.sort();
        ids.join(",")
    };
    assert_eq!(found("rock"), "a,b");
    assert_eq!(found("rock -moss"), "b");
    assert_eq!(found("rock or stone -set:forest"), "a,c");
    assert_eq!(found("-rock"), "c,d");
    assert_eq!(found(r#""rock dry""#), "");
    assert_eq!(found(r#""dry rock""#), "b");
}

#[test]
fn assets_are_found_and_left_out_by_their_packs_words() {
    let conn = test_library();
    conn.execute_batch(
        r#"INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at, tags) VALUES
           ('scifi', 'Sci-Fi Corridor Kit', 'ready', 'x', '[]'),
           ('woods', 'Woodland', 'ready', 'x', '["biome:forest"]'),
           ('ue', 'Robot Pack', 'ready', 'x', '[]');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('bot1', 'scifi', 'h', 'bot1.glb', 'robot', 'glb', 'model', 'x', 'x'),
           ('wall', 'scifi', 'h', 'wall.png', 'wall', 'png', 'texture', 'x', 'x'),
           ('bot2', 'woods', 'h', 'bot2.glb', 'robot', 'glb', 'model', 'x', 'x'),
           ('tree', 'woods', 'h', 'tree.glb', 'tree', 'glb', 'model', 'x', 'x'),
           ('bot3', 'ue', 'h', 'bot3.fbx', 'robot', 'fbx', 'model', 'x', 'x'),
           ('bot4', 'ue', 'h', 'bot4.uasset', 'robot', 'uasset', 'model', 'x', 'x');"#,
    )
    .unwrap();
    for id in ["bot1", "wall", "bot2", "tree", "bot3", "bot4"] {
        crate::db::reindex_asset(&conn, id).unwrap();
    }
    let found = |text: &str| {
        let mut ids = search(&conn, &SearchQuery { q: Some(text.into()), ..Default::default() }, false)
            .unwrap()
            .items
            .into_iter()
            .map(|a| a.id)
            .collect::<Vec<_>>();
        ids.sort();
        ids.join(",")
    };
    assert_eq!(found("robot"), "bot1,bot2,bot3,bot4");
    assert_eq!(found("robot -sci-fi"), "bot2,bot3,bot4", "the pack's name");
    assert_eq!(found("robot -unreal"), "bot1,bot2", "the FBX in a pack of Unreal packages too");
    assert_eq!(found("robot forest"), "bot2", "the pack's tags, with a word of the asset's own");
    assert_eq!(found("forest"), "", "but not the pack's words alone");
    assert_eq!(found("woodland"), "");
    assert_eq!(found("pack:woodland"), "bot2,tree");
    assert_eq!(found(r#"robot -pack:"robot pack""#), "bot1,bot2");
    // The pack keeps its assets' rows up to date.
    conn.execute("UPDATE packs SET name = 'Space Station', tags = '[\"style:sci-fi\"]' WHERE id = 'woods'", [])
        .unwrap();
    assert_eq!(found("robot -sci-fi"), "bot3,bot4");
    assert_eq!(found("robot forest"), "");
    conn.execute("DELETE FROM assets WHERE id = 'bot4'", []).unwrap();
    assert_eq!(found("robot -unreal"), "bot1,bot2,bot3", "no Unreal packages left in the pack");
}

#[test]
fn the_first_letters_typed_find_newest_first_and_more_are_ranked() {
    for (q, short) in [("t", true), ("tr", true), ("T r", true), ("t or r", true), ("t -tree", true)] {
        assert_eq!(words(q).short, short, "{q}");
    }
    for q in ["tre", "t tree", r#""t""#, "pack:t", "-t", ""] {
        assert!(!words(q).short, "{q}");
    }

    let conn = test_library();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('old', 'p1', 'h', 'a.png', 'rock', 'png', 'texture', '2026-01-01', 'x'),
           ('mid', 'p1', 'h', 'b.png', 'rocky ridge of rocks', 'png', 'texture', '2026-02-01', 'x'),
           ('new', 'p1', 'h', 'c.png', 'river rock wall', 'png', 'texture', '2026-03-01', 'x');",
    )
    .unwrap();
    for id in ["old", "mid", "new"] {
        crate::db::reindex_asset(&conn, id).unwrap();
    }
    let found = |t: &str| {
        let q = SearchQuery { q: Some(t.into()), ..Default::default() };
        search(&conn, &q, false).unwrap().items.into_iter().map(|a| a.id).collect::<Vec<_>>().join(",")
    };
    assert_eq!(found("r"), "new,mid,old");
    assert_eq!(found("ro"), "new,mid,old");
    // Ranked: the name with two words starting with it first.
    assert_eq!(found("roc").split(',').next(), Some("mid"));
}

#[test]
fn assets_are_found_by_their_own_fields_and_by_how_well_they_are_rated() {
    let conn = test_library();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at, rating) VALUES
           ('a', 'p1', 'h', 'a.png', 'cactus', 'png', 'texture', 'x', 'x', 4),
           ('b', 'p1', 'h', 'b.png', 'boulder', 'png', 'texture', 'x', 'x', 2),
           ('c', 'p1', 'h', 'c.png', 'shrub', 'png', 'texture', 'x', 'x', NULL);
         INSERT INTO asset_fields (asset_id, key, value) VALUES
           ('a', 'biome', 'desert'), ('b', 'biome', 'tundra'), ('c', 'artist', 'Mara Quill');",
    )
    .unwrap();
    for id in ["a", "b", "c"] {
        crate::db::reindex_asset(&conn, id).unwrap();
    }
    let found = |q: SearchQuery| {
        let mut ids = search(&conn, &q, false).unwrap().items.into_iter().map(|a| a.id).collect::<Vec<_>>();
        ids.sort();
        ids.join(",")
    };
    let text = |t: &str| SearchQuery { q: Some(t.into()), ..Default::default() };
    assert_eq!(found(text("desert")), "a");
    assert_eq!(found(text("biome")), "a,b");
    assert_eq!(found(text("quill")), "c");
    let rated = |n: u8| SearchQuery { rating_min: Some(n), ..Default::default() };
    assert_eq!(found(rated(4)), "a");
    assert_eq!(found(rated(2)), "a,b");
    assert_eq!(found(SearchQuery { rating_min: Some(3), ..text("biome") }), "a");
    // 0 is no filter at all.
    assert_eq!(found(rated(0)), "a,b,c");
}

#[test]
fn a_search_by_words_counts_the_tags_of_what_it_found() {
    let conn = test_library();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');",
    )
    .unwrap();
    for (id, name, kind, tags) in [
        ("r1", "rock one", "texture", "mossy,gray,set:a"),
        ("r2", "rock two", "model", "gray,set:a"),
        ("r3", "rock three", "texture", "gray,dark"),
        ("r4", "rock four", "texture", "mossy"),
        ("m", "moss", "texture", "mossy,green"),
    ] {
        conn.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
             VALUES (?1, 'p1', 'h', ?1 || '.png', ?2, 'png', ?3, 'x', 'x')",
            params![id, name, kind],
        )
        .unwrap();
        conn.execute_batch(&format!(
            "INSERT INTO assets_fts (rowid, asset_id, name, words, tags) SELECT rowid, id, name, '', '' FROM assets
               WHERE id = '{id}';
             UPDATE assets SET fts_row = rowid WHERE id = '{id}';"
        ))
        .unwrap();
        for tag in tags.split(',') {
            conn.execute("INSERT INTO asset_tags (asset_id, tag, source) VALUES (?1, ?2, 'x')", params![id, tag])
                .unwrap();
        }
    }
    // The model needs the third rock as raw data: hidden, its tags aren't counted.
    conn.execute("INSERT INTO asset_links (from_id, to_id, rel) VALUES ('r2', 'r3', 'buffer')", []).unwrap();
    let rock = SearchQuery { q: Some("rock".into()), hide_supporting: true, ..Default::default() };
    let shown = |q: &SearchQuery| {
        let listed = |tags: &[Facet]| tags.iter().map(|t| format!("{} {}", t.value, t.count)).collect::<Vec<_>>();
        let found = listed(&search(&conn, q, true).unwrap().tags);
        assert_eq!(
            found,
            listed(&tags(&conn, &build_filter(q, true, None, &Tree::default()), &Tree::default()).unwrap()),
            "{q:?}"
        );
        found
    };
    assert_eq!(shown(&rock), ["gray 2", "mossy 2"]);
    assert_eq!(shown(&SearchQuery { kind: Some("texture".into()), ..rock.clone() }), ["mossy 2", "gray 1"]);
    assert_eq!(shown(&SearchQuery { hide_supporting: false, ..rock.clone() }), ["gray 3", "mossy 2", "dark 1"]);
    assert_eq!(shown(&SearchQuery { q: Some("nothing".into()), ..rock }), Vec::<String>::new());
}

#[test]
fn typed_words_and_the_sidebar_follow_the_tag_tree() {
    let conn = test_library();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');
         INSERT INTO tag_parents (tag, parent) VALUES ('car', 'vehicle'), ('sedan', 'car'), ('hard surface', 'car');",
    )
    .unwrap();
    for (id, name, tags) in [
        ("s", "blue one", "sedan,blue"),
        ("c", "red one", "car,sedan,red"),
        ("h", "gray one", "hard surface"),
        ("v", "old one", "vehicle"),
        ("r", "rock", "gray"),
    ] {
        conn.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
             VALUES (?1, 'p1', 'h', ?1 || '.png', ?2, 'png', 'texture', 'x', 'x')",
            params![id, name],
        )
        .unwrap();
        conn.execute_batch(&format!(
            "INSERT INTO assets_fts (rowid, asset_id, name, words, tags) SELECT rowid, id, name, '', '{}' FROM assets
               WHERE id = '{id}';
             UPDATE assets SET fts_row = rowid WHERE id = '{id}';",
            tags.replace(',', " ")
        ))
        .unwrap();
        for tag in tags.split(',') {
            conn.execute("INSERT INTO asset_tags (asset_id, tag, source) VALUES (?1, ?2, 'x')", params![id, tag])
                .unwrap();
        }
    }
    let found = |q: &str| {
        let mut ids = search(&conn, &SearchQuery { q: Some(q.into()), ..Default::default() }, false)
            .unwrap()
            .items
            .into_iter()
            .map(|a| a.id)
            .collect::<Vec<_>>();
        ids.sort();
        ids.concat()
    };
    assert_eq!(found("vehicle"), "chsv");
    assert_eq!(found("Car"), "chs");
    assert_eq!(found("car or rock"), "chrs");
    assert_eq!(found("one -car"), "v");
    assert_eq!(found("-vehicle"), "r");
    assert_eq!(found("one -\"hard surface\""), "csv");
    // A word that only starts a broad tag is a word like any other.
    assert_eq!(found("veh"), "v");

    let shown = |q: SearchQuery| {
        let listed = |tags: &[Facet]| tags.iter().map(|t| format!("{} {}", t.value, t.count)).collect::<Vec<_>>();
        let found = listed(&search(&conn, &q, true).unwrap().tags);
        let tree = Tree::load(&conn).unwrap();
        assert_eq!(found, listed(&tags(&conn, &build_filter(&q, true, None, &tree), &tree).unwrap()), "{q:?}");
        found
    };
    // Each counted once: the red car is a car and a sedan.
    assert_eq!(
        shown(SearchQuery::default()),
        ["vehicle 4", "car 3", "sedan 2", "blue 1", "gray 1", "hard surface 1", "red 1"]
    );
    // From what a search by words found: vehicle shows though nothing found is tagged vehicle itself.
    assert_eq!(
        shown(SearchQuery { q: Some("blue".into()), ..Default::default() }),
        ["blue 1", "car 1", "sedan 1", "vehicle 1"]
    );
}

#[test]
fn a_pack_browses_as_folders() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x'), ('p2', 'Other', 'ready', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('top', 'p1', 'h', 'License.txt', 'License', 'txt', 'text', 'x', 'x'),
           ('a', 'p1', 'h', 'UI/Sheet/ui.png', 'ui', 'png', 'texture', 'x', 'x'),
           ('b', 'p1', 'h', 'UI/Sheet/ui.xml', 'ui', 'xml', 'text', 'x', 'x'),
           ('c', 'p1', 'h', 'UI/PNG/Blue/button.png', 'button', 'png', 'texture', 'x', 'x'),
           ('d', 'p1', 'h', 'UI/readme.txt', 'readme', 'txt', 'text', 'x', 'x'),
           ('e', 'p1', 'h', 'UI0/odd.png', 'odd', 'png', 'texture', 'x', 'x'),
           ('f', 'p1', 'h', 'Ünï/x.png', 'x', 'png', 'texture', 'x', 'x'),
           ('g', 'p1', 'h', 'Ünï/deeper/y.png', 'y', 'png', 'texture', 'x', 'x'),
           ('o', 'p2', 'h', 'UI/other.png', 'other', 'png', 'texture', 'x', 'x');",
    )
    .unwrap();
    let browse = |folder: &str, kind: Option<&str>| {
        let q = SearchQuery {
            pack: Some("p1".into()),
            folder: Some(folder.into()),
            kind: kind.map(Into::into),
            sort: Some("name".into()),
            ..Default::default()
        };
        let page = search(&conn, &q, false).unwrap();
        let ids = page.items.into_iter().map(|a| a.id).collect::<Vec<_>>();
        let folders = page.folders.into_iter().map(|f| (f.value, f.count)).collect::<Vec<_>>();
        (ids, folders)
    };
    let f = |v: &[(&str, u64)]| v.iter().map(|(n, c)| ((*n).to_owned(), *c)).collect::<Vec<_>>();
    assert_eq!(browse("", None), (vec!["top".to_owned()], f(&[("UI", 4), ("UI0", 1), ("Ünï", 2)])));
    assert_eq!(browse("UI", None), (vec!["d".to_owned()], f(&[("PNG", 1), ("Sheet", 2)])));
    assert_eq!(browse("/UI/Sheet/", None).0, vec!["a".to_owned(), "b".to_owned()], "slashes at the ends don't matter");
    assert_eq!(browse("UI", Some("texture")), (vec![], f(&[("PNG", 1), ("Sheet", 1)])), "folders count matches");
    assert_eq!(browse("Ünï", None), (vec!["f".to_owned()], f(&[("deeper", 1)])));
    // Without a pack, a folder means nothing.
    let q = SearchQuery { folder: Some("UI".into()), ..Default::default() };
    assert_eq!(search(&conn, &q, false).unwrap().total, 9);
}

#[test]
fn music_by_tempo_and_key() {
    let conn = test_library();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Loops', 'ready', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
           ('stated', 'p1', 'h', 'a.wav', 'a', 'wav', 'audio', '{\"tempo_bpm\": 128}', 'x', 'x'),
           ('heard', 'p1', 'h', 'b.ogg', 'b', 'ogg', 'audio', '{\"measured_bpm\": 70, \"measured_bpm_alt\": 140}', 'x', 'x'),
           ('relative', 'p1', 'h', 'c.ogg', 'c', 'ogg', 'audio', '{}', 'x', 'x'),
           ('none', 'p1', 'h', 'd.ogg', 'd', 'ogg', 'audio', '{}', 'x', 'x');
         INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
           ('stated', 'bpm:128', 'meta', 1.0), ('stated', 'key:a-minor', 'meta', 1.0),
           ('heard', 'bpm:70', 'rule', 0.8), ('heard', 'key:e-minor', 'rule', 0.8),
           ('relative', 'key:c-major', 'rule', 0.8), ('none', 'bpm', 'human', 1.0);",
    )
    .unwrap();
    let find = |q: SearchQuery| {
        let mut ids = search(&conn, &SearchQuery { sort: Some("name".into()), ..q }, false)
            .unwrap()
            .items
            .into_iter()
            .map(|a| a.id)
            .collect::<Vec<_>>();
        ids.sort();
        ids
    };
    let bpm = |lo: Option<f32>, hi: Option<f32>| find(SearchQuery { bpm_min: lo, bpm_max: hi, ..Default::default() });
    assert_eq!(bpm(Some(120.0), Some(130.0)), ["stated"]);
    // What was heard counts at half or double, as a tempo the file states doesn't.
    assert_eq!(bpm(Some(135.0), None), ["heard"]);
    assert_eq!(bpm(None, Some(100.0)), ["heard"]);
    assert_eq!(bpm(Some(60.0), Some(64.0)), Vec::<String>::new());
    let key =
        |k: &str, compatible_keys| find(SearchQuery { key: Some(k.into()), compatible_keys, ..Default::default() });
    assert_eq!(key("Am", false), ["stated"]);
    assert_eq!(key("8A", true), ["heard", "relative", "stated"], "C major is its relative, E minor a fifth up");
    assert_eq!(key("C major", false), ["relative"]);
    assert_eq!(key("H minor", true), Vec::<String>::new(), "a key that isn't one matches nothing");
}

/// Whether an asset supports another is kept with it as links and maps come and go, and as kinds change.
#[test]
fn supporting_follows_links_and_maps() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
         INSERT INTO packs (id, name, status, tags, created_at) VALUES ('p', 'P', 'ready', '[]', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
           ('gltf', 'p', 'h', 'a.gltf', 'a', 'gltf', 'model', 'x', 'x'),
           ('bin', 'p', 'h', 'a.bin', 'a', 'bin', 'other', 'x', 'x'),
           ('mat', 'p', 'h', 'm.material', 'm', 'material', 'material', 'x', 'x'),
           ('tex', 'p', 'h', 't.png', 't', 'png', 'texture', 'x', 'x'),
           ('sheet', 'p', 'h', 's.json', 's', 'json', 'other', 'x', 'x'),
           ('page', 'p', 'h', 's.png', 's', 'png', 'texture', 'x', 'x');",
    )
    .unwrap();
    let supporting = || {
        conn.prepare(&format!("SELECT a.id FROM assets a WHERE {SUPPORTING} ORDER BY a.id"))
            .unwrap()
            .query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .collect::<rusqlite::Result<Vec<_>>>()
            .unwrap()
    };
    assert_eq!(supporting(), Vec::<String>::new());
    conn.execute_batch(
        "INSERT INTO asset_links (from_id, to_id, rel) VALUES ('gltf', 'bin', 'buffer'), ('sheet', 'page', 'texture');
         INSERT INTO material_maps (material_id, texture_id, role) VALUES ('mat', 'tex', 'albedo');",
    )
    .unwrap();
    assert_eq!(supporting(), ["bin", "tex"], "a texture linked from something that isn't a sheet stays");
    conn.execute("UPDATE assets SET kind = 'sheet' WHERE id = 'sheet'", []).unwrap();
    assert_eq!(supporting(), ["bin", "page", "tex"], "a sheet's pages go with it");
    conn.execute("DELETE FROM material_maps", []).unwrap();
    conn.execute("UPDATE asset_links SET rel = 'guess' WHERE to_id = 'bin'", []).unwrap();
    assert_eq!(supporting(), ["page"]);
    conn.execute("DELETE FROM assets WHERE id = 'sheet'", []).unwrap();
    assert_eq!(supporting(), Vec::<String>::new(), "the links of an asset that's gone go with it");
}

#[test]
fn look_ranked_search_keeps_filters_and_order() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch(
        "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x'), ('h2', 1, 'x'), ('h3', 1, 'x');
         INSERT INTO packs (id, name, status, tags, created_at) VALUES ('p1', 'Rocks', 'ready', '[]', 'x');
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, thumb_hash, created_at, updated_at) VALUES
           ('a1', 'p1', 'h1', 'gray rock.png', 'gray rock', 'png', 'texture', 't1', 'x', 'x'),
           ('a2', 'p1', 'h2', 'mossy rock.png', 'mossy rock', 'png', 'texture', 't2', 'x', 'x'),
           ('a3', 'p1', 'h3', 'rock.glb', 'rock', 'glb', 'model', 't3', 'x', 'x');
         INSERT INTO assets_fts (asset_id, name) VALUES ('a1', 'gray rock'), ('a2', 'mossy rock'), ('a3', 'rock');
         UPDATE assets SET fts_row = (SELECT f.rowid FROM assets_fts f WHERE f.asset_id = assets.id);",
    )
    .unwrap();
    let ids = |page: SearchPage| page.items.into_iter().map(|a| a.id).collect::<Vec<_>>();
    let rank = LookRank {
        scores: vec![("t3".into(), 0.9), ("t2".into(), 0.8), ("t1".into(), 0.7)],
        exclude: Some("a1".into()),
        blend: false,
        keys: Keys::Thumbs,
    };
    let q = SearchQuery::default();
    assert_eq!(ids(search_ranked(&conn, &q, true, Some(&rank)).unwrap()), ["a3", "a2"], "best first, minus a1");
    let q = SearchQuery { kind: Some("texture".into()), ..Default::default() };
    let page = search_ranked(&conn, &q, true, Some(&rank)).unwrap();
    assert_eq!(page.kinds.len(), 2, "the kind facet counts the ranked set");
    assert_eq!(ids(page), ["a2"]);

    // Blended (text-to-image): word matches and look matches both count.
    let rank = LookRank { scores: vec![("t3".into(), 0.2)], exclude: None, blend: true, keys: Keys::Thumbs };
    let q = SearchQuery { q: Some("mossy".into()), ..Default::default() };
    assert_eq!(ids(search_ranked(&conn, &q, false, Some(&rank)).unwrap()), ["a2", "a3"]);
    assert_eq!(ids(search(&conn, &q, false).unwrap()), ["a2"], "a plain search is unchanged");
}

/// How long searching a big library takes, step by step: `cargo test -p hoardd --release --lib big_library -- --ignored --nocapture`.
#[test]
#[ignore = "a timing, not a test"]
#[expect(clippy::print_stdout, reason = "the timings are its output")]
fn big_library() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    let n = 300_000;
    let started = std::time::Instant::now();
    make_big_library(&conn, n);
    println!("made {n} assets in {:.1?}", started.elapsed());
    let pictures: Vec<(String, f32)> = (0..300).map(|i| (format!("t{}", i * 997), 0.4)).collect();
    let look = LookRank { scores: pictures, exclude: None, blend: true, keys: Keys::Thumbs };
    let time = |label: &str, q: SearchQuery, facets: bool, look: Option<&LookRank>| {
        let started = std::time::Instant::now();
        let page = search_ranked(&conn, &q, facets, look).unwrap();
        println!("{label}: {} of {} in {:.1?}", page.items.len(), page.total, started.elapsed());
    };
    let alone = |label: &str, q: SearchQuery| {
        let tree = Tree::load(&conn).unwrap();
        let f = build_filter(&q, true, None, &tree);
        let started = std::time::Instant::now();
        let counts = count(&conn, &q, None, false, &tree).unwrap();
        let counted = started.elapsed();
        let started = std::time::Instant::now();
        let tags = tags(&conn, &f, &tree).unwrap();
        println!(
            "{label}: counted {} in {counted:.1?}, {} tags in {:.1?}",
            counts.total,
            tags.len(),
            started.elapsed()
        );
    };
    // The page alone, put in order without counting: what's left of a search once its counts are kept.
    let page_alone = |label: &str, q: SearchQuery| {
        let f = build_filter(&q, true, None, &Tree::default());
        let started = std::time::Instant::now();
        let n = conn
            .prepare(&format!(
                "SELECT a.rowid FROM assets a {} {} ORDER BY a.created_at DESC, a.path ASC LIMIT 60",
                f.join,
                f.where_sql()
            ))
            .unwrap()
            .query_map(params_from_iter(&f.params), |r| r.get::<_, i64>(0))
            .unwrap()
            .count();
        println!("{label}: {n} in order in {:.1?}", started.elapsed());
    };
    let pieces = |label: &str, typed: &str| time_in_pieces(&conn, label, typed);
    let started = std::time::Instant::now();
    let kept = read_facts(&conn).unwrap().map_or(0, |f| f.order.len());
    println!("what's kept of every asset: {kept} read in {:.1?}", started.elapsed());
    pieces("the first letter, in pieces", "t");
    pieces("two letters, in pieces", "th");
    pieces("words, in pieces", "rock");
    alone("everything, alone", SearchQuery { hide_supporting: true, ..Default::default() });
    alone("the first letter, alone", SearchQuery { q: Some("t".into()), hide_supporting: true, ..Default::default() });
    alone("words, alone", SearchQuery { q: Some("rock".into()), hide_supporting: true, ..Default::default() });
    let big_pack = SearchQuery { pack: Some("p0".into()), hide_supporting: true, ..Default::default() };
    alone("a big pack, alone", big_pack.clone());
    page_alone("a big pack's page, alone", big_pack.clone());
    alone("a big pack's top folder, alone", SearchQuery { folder: Some(String::new()), ..big_pack.clone() });
    page_alone("a big pack's top folder's page, alone", SearchQuery { folder: Some(String::new()), ..big_pack });
    let words = || SearchQuery { q: Some("rock".into()), hide_supporting: true, ..Default::default() };
    time("browse everything", SearchQuery { hide_supporting: true, ..Default::default() }, true, None);
    time("browse everything, no facets", SearchQuery { hide_supporting: true, ..Default::default() }, false, None);
    time("words", words(), true, None);
    let letter = || SearchQuery { q: Some("t".into()), hide_supporting: true, ..Default::default() };
    time("the first letter typed", letter(), true, None);
    time("the first letter typed, its next page", SearchQuery { offset: Some(60), ..letter() }, true, None);
    let by_name = SearchQuery { sort: Some("name".into()), hide_supporting: true, ..Default::default() };
    time("everything by name", by_name.clone(), true, None);
    time("everything by name, its next page", SearchQuery { offset: Some(60), ..by_name }, true, None);
    let by_size = SearchQuery { sort: Some("size".into()), hide_supporting: true, ..Default::default() };
    time("everything by size, its next page", SearchQuery { offset: Some(60), ..by_size }, true, None);
    let pack = |folder: Option<&str>| SearchQuery {
        pack: Some("p0".into()),
        folder: folder.map(Into::into),
        hide_supporting: true,
        ..Default::default()
    };
    time("a big pack", pack(None), true, None);
    time("a big pack's top folder", pack(Some("")), true, None);
    time("a folder in it", pack(Some("Dir3")), true, None);
    time("a folder in that", pack(Some("Dir3/Sub12")), true, None);
    time("words and pictures", words(), true, Some(&look));
    time("words and pictures, no facets", words(), false, Some(&look));
    time("words and pictures, textures", SearchQuery { kind: Some("texture".into()), ..words() }, true, Some(&look));
    let started = std::time::Instant::now();
    let packs = crate::db::list_packs(&conn, crate::db::Packs::Library).unwrap();
    println!("the pack list: {} in {:.1?}", packs.len(), started.elapsed());
    let started = std::time::Instant::now();
    let p0 = crate::db::get_pack(&conn, "p0").unwrap().unwrap();
    println!("a big pack's card: {} assets in {:.1?}", p0.asset_count, started.elapsed());
    for (prefix, anywhere) in [("", false), ("co", false), ("lo", true), ("co", false)] {
        let started = std::time::Instant::now();
        let tags = tag_counts(&conn, prefix, anywhere, 12).unwrap();
        println!("tags like {prefix:?}: {} in {:.1?}", tags.len(), started.elapsed());
    }

    // The same with a tag tree.
    let started = std::time::Instant::now();
    add_a_tag_tree(&conn);
    println!("with a tag tree, made in {:.1?}:", started.elapsed());
    alone("  everything, alone", SearchQuery { hide_supporting: true, ..Default::default() });
    alone("  the first letter, alone", letter());
    time("  browse everything", SearchQuery { hide_supporting: true, ..Default::default() }, true, None);
    time("  the first letter typed", letter(), true, None);
    time("  words", words(), true, None);
    let typed = |q: &str| SearchQuery { q: Some(q.into()), hide_supporting: true, ..Default::default() };
    time("  a broad tag typed", typed("warm"), true, None);
    time("  a broad tag typed, no facets", typed("warm"), false, None);
    time("  words but a broad tag", typed("rock -warm"), true, None);
    let picked = SearchQuery { tags: vec!["warm".into()], hide_supporting: true, ..Default::default() };
    time("  a broad tag picked", picked.clone(), true, None);
    time("  a broad tag picked, no facets", picked, false, None);

    // What's kept from the library after it changes: a setting (nothing a search reads), then one asset's tags.
    for (what, change) in [
        ("a setting", "INSERT INTO settings (key, value) VALUES ('timing', 'x')"),
        ("one asset's tags", "INSERT INTO asset_tags (asset_id, tag, source) VALUES ('a7', 'warm', 'x')"),
    ] {
        conn.execute(change, []).unwrap();
        time(&format!("  the first letter typed, {what} changed"), letter(), true, None);
    }
    // An import adding assets as it goes: newer than the rest, then one older (put in order again).
    let browse = || SearchQuery { hide_supporting: true, ..Default::default() };
    for (what, at) in [("an asset added", "2027"), ("another", "2028"), ("an older one", "2000")] {
        conn.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
               VALUES (?1, 'p1', 'b1', ?1, ?1, 'png', 'texture', ?2, 'x')",
            [&format!("added {at}"), at],
        )
        .unwrap();
        time(&format!("  browse everything, {what}"), browse(), true, None);
    }
}

/// A search by words in pieces (see [`big_library`]): the index's matches alone, ranked, joined to the library and
/// put in order, and the sidebar's tags of what it found.
#[expect(clippy::print_stdout, reason = "the timings are its output")]
fn time_in_pieces(conn: &Connection, label: &str, typed: &str) {
    let q = SearchQuery { q: Some(typed.into()), hide_supporting: true, ..Default::default() };
    let tree = Tree::load(conn).unwrap();
    let fts = words_in(typed, &tree).find.unwrap();
    let one = |sql: &str| {
        let started = std::time::Instant::now();
        let n: i64 = conn.query_row(sql, [&fts], |r| r.get(0)).unwrap();
        (n, started.elapsed())
    };
    let (n, matched) = one("SELECT count(*) FROM assets_fts WHERE assets_fts MATCH ?");
    let (_, ranked) = one("SELECT count(*) FROM (SELECT rowid FROM assets_fts WHERE assets_fts MATCH ?
          ORDER BY bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0, 1.5, 1.0))");
    let started = std::time::Instant::now();
    let (found, _) = rank_and_count(conn, &q, "f.rank ASC", "", &tree).unwrap();
    let in_order = started.elapsed();
    let started = std::time::Instant::now();
    tags_of_rows(conn, &found, &tree).unwrap();
    println!(
        "{label}: {n} matched in {matched:.1?}, ranked in {ranked:.1?}, {} in order and counted in \
         {in_order:.1?}, their tags in {:.1?}",
        found.len(),
        started.elapsed()
    );
}

/// What's kept of every asset (see [`Facts`]) finds and counts what SQL does, in the same order, and is read again
/// when what it keeps changes, but not when only tags do.
#[test]
fn what_is_kept_of_every_asset_finds_what_the_library_has() {
    let dir = tempfile::tempdir().unwrap();
    let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    make_big_library(&conn, 3000);
    add_a_tag_tree(&conn);
    let typed = |t: &str| SearchQuery { q: Some(t.into()), ..Default::default() };
    let hiding = |q: SearchQuery| SearchQuery { hide_supporting: true, ..q };
    let queries = [
        SearchQuery::default(),
        hiding(SearchQuery::default()),
        hiding(SearchQuery { kind: Some("texture".into()), ..Default::default() }),
        SearchQuery { pack: Some("p0".into()), ..Default::default() },
        SearchQuery { pack: Some("nowhere".into()), ..Default::default() },
        typed("t"),
        typed("rock"),
        typed("t -rock"),
        hiding(SearchQuery { sort: Some("created".into()), ..typed("rock") }),
        hiding(SearchQuery { kind: Some("model".into()), ..typed("thing") }),
        SearchQuery { tags: vec!["warm".into()], ..Default::default() },
        hiding(SearchQuery { tags: vec!["warm".into(), "color:1".into()], ..typed("thing") }),
        SearchQuery { tags: vec!["nothing".into()], ..Default::default() },
    ];
    let kinds = |k: &[Facet]| k.iter().map(|f| format!("{} {}", f.value, f.count)).collect::<Vec<_>>();
    let check = |when: &str| {
        let tree = Tree::load(&conn).unwrap();
        let facts = facts(&conn).unwrap().unwrap();
        for q in &queries {
            let s = facts_search(q, &tree).unwrap();
            let (mut found, counts) = search_facts(&conn, &facts, &s, &tree).unwrap();
            let f = build_filter(q, true, None, &tree);
            let order = if s.ranked { "a.rowid" } else { "a.created_at DESC, a.path ASC" };
            let sql = conn
                .prepare(&format!("SELECT a.rowid FROM assets a {} {} ORDER BY {order}", f.join, f.where_sql()))
                .unwrap()
                .query_map(params_from_iter(&f.params), |r| r.get::<_, i64>(0))
                .unwrap()
                .collect::<rusqlite::Result<Vec<_>>>()
                .unwrap();
            // Best match first: matches as good as each other can go either way.
            if s.ranked {
                found.sort_unstable();
            }
            assert_eq!(found, sql, "{when}: {q:?}");
            let c = count(&conn, q, None, false, &tree).unwrap();
            assert_eq!(
                (counts.total, counts.supporting_hidden, kinds(&counts.kinds)),
                (c.total, c.supporting_hidden, kinds(&c.kinds)),
                "{when}: {q:?}"
            );
        }
    };
    check("at first");
    // Put right where it changed, it's what's read afresh.
    let same = |kept: &Facts, when: &str| {
        let read = read_facts(&conn).unwrap().unwrap();
        assert_eq!(kept.order, read.order, "{when}: order");
        let of = |f: &Facts, r: usize| {
            let k = f.kind.get(r).copied().filter(|&k| k != NO_ASSET)?;
            let pack = f.packs.iter().find(|(_, p)| **p == f.pack[r]).map(|(n, _)| n.clone());
            Some((f.kinds[usize::from(k)].clone(), f.flags[r], pack))
        };
        for r in 0..kept.kind.len().max(read.kind.len()) {
            assert_eq!(of(kept, r), of(&read, r), "{when}: row {r}");
        }
        let fts = |f: &Facts, i: usize| f.by_fts_row.get(i).copied().unwrap_or(NO_ROW);
        for i in 0..kept.by_fts_row.len().max(read.by_fts_row.len()) {
            assert_eq!(fts(kept, i), fts(&read, i), "{when}: index row {i}");
        }
    };
    for (when, change) in [
        (
            "a pack and an asset archived, and a kind changed",
            "UPDATE packs SET archived_at = 'x' WHERE id = 'p3'; UPDATE assets SET archived_at = 'x' WHERE id = 'a10';
             UPDATE assets SET kind = 'audio' WHERE id = 'a20';",
        ),
        (
            "a link made, so a file supports another",
            "INSERT INTO asset_links (from_id, to_id, rel) VALUES ('a30', 'a32', 'buffer')",
        ),
        (
            "an asset added and one removed",
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
               VALUES ('new', 'p1', 'b1', 'new.png', 'thing new', 'png', 'texture', '2027', 'x');
             INSERT INTO assets_fts (asset_id, name, words, tags) VALUES ('new', 'thing new', '', '');
             UPDATE assets SET fts_row = (SELECT rowid FROM assets_fts WHERE asset_id = 'new') WHERE id = 'new';
             DELETE FROM assets WHERE id = 'a40';",
        ),
        ("an animation", "UPDATE assets SET kind = 'animation' WHERE id = 'a63'"),
        ("a take of it", "UPDATE assets SET meta = json_object('take_of', 'a63') WHERE id = 'a61'"),
        (
            "a material of an asset that isn't there",
            "UPDATE assets SET ext = 'material', meta = json_object('owned_by', 'owner') WHERE id = 'a70'",
        ),
        (
            "the asset it's of added, older than the rest",
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
               VALUES ('owner', 'p2', 'b2', 'owner.glb', 'owner', 'glb', 'model', '2000', 'x')",
        ),
        ("an asset moved", "UPDATE assets SET created_at = '1999' WHERE id = 'a80'"),
        ("many moved", "UPDATE assets SET created_at = '1998' WHERE rowid % 40 = 0"),
        ("the take removed", "DELETE FROM assets WHERE id = 'a61'"),
        ("the asset the material is of removed", "DELETE FROM assets WHERE id = 'owner'"),
    ] {
        let mut kept = Facts::clone(&facts(&conn).unwrap().unwrap());
        let before = crate::db::library_writes();
        conn.execute_batch(change).unwrap();
        // (What other tests change in libraries of their own meanwhile is noted too.)
        let changed = crate::db::changes::since(before, crate::db::library_writes());
        if let Some(c) = changed.filter(|c| !c.facts && !c.too_much) {
            assert!(patch_facts(&conn, &mut kept, &c).unwrap(), "{when}: put right");
            same(&kept, when);
        }
        check(when);
    }
    // Tags, a rating and a name change nothing kept.
    let before = crate::db::library_writes();
    conn.execute_batch(
        "INSERT INTO asset_tags (asset_id, tag, source) VALUES ('a50', 'warm', 'x');
         UPDATE assets SET rating = 4, name = 'renamed' WHERE id = 'a51';",
    )
    .unwrap();
    let changed = crate::db::changes::since(before, crate::db::library_writes()).unwrap();
    let row = |id: &str| conn.query_row("SELECT rowid FROM assets WHERE id = ?1", [id], |r| r.get::<_, i64>(0));
    let (a50, a51) = (row("a50").unwrap(), row("a51").unwrap());
    assert!(
        changed.too_much || !(changed.fact_rows.contains(&a50) || changed.fact_rows.contains(&a51)),
        "kept as it was"
    );
    check("tags changed");
}

/// The colors under warm and cool, both under colored, and in the index's tags (see [`big_library`]).
fn add_a_tag_tree(conn: &Connection) {
    conn.execute_batch(
        "BEGIN;
         WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < 11)
         INSERT INTO tag_parents (tag, parent) SELECT 'color:' || i, CASE WHEN i < 6 THEN 'warm' ELSE 'cool' END FROM k;
         INSERT INTO tag_parents (tag, parent) VALUES ('warm', 'colored'), ('cool', 'colored');
         UPDATE assets_fts SET tags = 'color:' || (rowid % 12);
         COMMIT;",
    )
    .unwrap();
}

/// A generated library of `n` assets in 200 packs, for timing searches in: two in five in one big pack, one in
/// fifty named "rock wall", each name with one of 26,000 words starting with t (about as many as a real library
/// has), every asset with a kind, a color and a set tag, and some supporting files.
fn make_big_library(conn: &Connection, n: usize) {
    conn.execute_batch(&format!(
        "BEGIN;
         WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < 199)
         INSERT INTO packs (id, name, status, created_at) SELECT 'p' || i, 'Pack ' || i, 'ready', 'x' FROM k;
         WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < {n} - 1)
         INSERT INTO blobs (hash, size, created_at) SELECT 'b' || i, i, 'x' FROM k;
         WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < {n} - 1)
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, words, thumb_hash, created_at, updated_at)
         SELECT 'a' || i, CASE WHEN i % 5 < 2 THEN 'p0' ELSE 'p' || (i % 200) END, 'b' || i,
                'Dir' || (i % 9) || '/Sub' || (i % 160) || '/file' || i || '.png',
                CASE i % 50 WHEN 0 THEN 'rock wall ' || i ELSE 'thing ' || i END
                  || ' t' || char(97 + i % 26) || (i % 997), 'png',
                CASE i % 4 WHEN 0 THEN 'texture' WHEN 1 THEN 'model' WHEN 2 THEN 'material' ELSE 'audio' END,
                'dir file', 't' || i, printf('2026-01-01T00:%06d', i), 'x' FROM k;
         INSERT INTO asset_tags (asset_id, tag, source) SELECT id, 'kind:' || kind, 'x' FROM assets;
         INSERT INTO asset_tags (asset_id, tag, source) SELECT id, 'color:' || (rowid % 12), 'x' FROM assets;
         INSERT INTO asset_tags (asset_id, tag, source) SELECT id, 'set:' || (rowid % 9000), 'x' FROM assets;
         INSERT INTO asset_links (from_id, to_id, rel) SELECT 'a' || (rowid - 1), id, 'texture' FROM assets
           WHERE kind = 'texture' AND rowid % 3 = 0;
         INSERT INTO material_maps (material_id, texture_id, role) SELECT 'a' || (rowid - 2), id, 'albedo' FROM assets
           WHERE kind = 'texture' AND rowid % 5 = 0;
         INSERT INTO assets_fts (rowid, asset_id, name, words, tags) SELECT rowid, id, name, words, '' FROM assets;
         UPDATE assets SET fts_row = rowid;
         COMMIT;
         ANALYZE;"
    ))
    .unwrap();
}

/// What upkeep buys a big, much-edited library: `cargo test -p hoardd --release --lib upkeep_timing -- --ignored --nocapture`.
#[test]
#[ignore = "a timing, not a test"]
#[expect(clippy::print_stdout, reason = "the timings are its output")]
fn upkeep_timing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hoard.db");
    let pool = open(&path, &dir.path().join("backups")).unwrap();
    let conn = pool.get().unwrap();
    let n = 300_000;
    let started = std::time::Instant::now();
    conn.execute_batch(&format!(
        "BEGIN;
         WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < 199)
         INSERT INTO packs (id, name, status, created_at) SELECT 'p' || i, 'Pack ' || i, 'ready', 'x' FROM k;
         WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < {n} - 1)
         INSERT INTO blobs (hash, size, created_at) SELECT 'b' || i, i, 'x' FROM k;
         WITH RECURSIVE k(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM k WHERE i < {n} - 1)
         INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, words, thumb_hash, created_at, updated_at)
         SELECT 'a' || i, CASE WHEN i % 5 < 2 THEN 'p0' ELSE 'p' || (i % 200) END, 'b' || i,
                'Dir' || (i % 9) || '/Sub' || (i % 160) || '/file' || i || '.png',
                CASE i % 50 WHEN 0 THEN 'rock wall ' || i ELSE 'thing ' || i END, 'png',
                CASE i % 4 WHEN 0 THEN 'texture' WHEN 1 THEN 'model' WHEN 2 THEN 'material' ELSE 'audio' END,
                'dir file', 't' || i, printf('2026-01-01T00:%06d', i), 'x' FROM k;
         INSERT INTO asset_tags (asset_id, tag, source) SELECT id, 'kind:' || kind, 'x' FROM assets;
         INSERT INTO asset_tags (asset_id, tag, source) SELECT id, 'color:' || (rowid % 12), 'x' FROM assets;
         INSERT INTO asset_tags (asset_id, tag, source) SELECT id, 'set:' || (rowid % 9000), 'x' FROM assets;
         COMMIT;"
    ))
    .unwrap();
    // Indexed the way imports do, one asset at a time in batches, so the search index grows as it really does.
    let ids: Vec<String> = (0..n).map(|i| format!("a{i}")).collect();
    for chunk in ids.chunks(500) {
        conn.execute_batch("BEGIN").unwrap();
        for id in chunk {
            crate::db::reindex_asset(&conn, id).unwrap();
        }
        conn.execute_batch("COMMIT").unwrap();
    }
    println!("made {n} assets in {:.1?}", started.elapsed());
    let size = |label: &str| {
        let pages: i64 = conn.query_row("PRAGMA page_count", [], |r| r.get(0)).unwrap();
        let free: i64 = conn.query_row("PRAGMA freelist_count", [], |r| r.get(0)).unwrap();
        let fts: i64 = conn.query_row("SELECT count(*) FROM assets_fts_data", [], |r| r.get(0)).unwrap();
        let wal = std::fs::metadata(path.with_extension("db-wal")).map_or(0, |m| m.len());
        println!(
            "  [{label}] file {} MB, {} MB free in it, wal {} MB, search index {fts} rows",
            pages * 4096 / 1_000_000,
            free * 4096 / 1_000_000,
            wal / 1_000_000
        );
    };
    let queries = || {
        let hide = || SearchQuery { hide_supporting: true, ..Default::default() };
        vec![
            ("browse", hide()),
            ("textures", SearchQuery { kind: Some("texture".into()), ..hide() }),
            ("a tag", SearchQuery { tags: vec!["set:42".into()], ..hide() }),
            ("two tags", SearchQuery { tags: vec!["color:3".into(), "kind:model".into()], ..hide() }),
            ("words", SearchQuery { q: Some("rock".into()), ..hide() }),
            ("a letter", SearchQuery { q: Some("t".into()), ..hide() }),
            ("words + kind", SearchQuery { q: Some("wall".into()), kind: Some("texture".into()), ..hide() }),
            ("a pack", SearchQuery { pack: Some("p7".into()), ..hide() }),
            ("big pack folder", SearchQuery { pack: Some("p0".into()), folder: Some("Dir3".into()), ..hide() }),
        ]
    };
    let run = |label: &str| {
        println!("{label}:");
        let mut all = std::time::Duration::ZERO;
        for (name, q) in queries() {
            let mut best = std::time::Duration::MAX;
            for _ in 0..3 {
                let started = std::time::Instant::now();
                search_ranked(&conn, &q, true, None).unwrap();
                best = best.min(started.elapsed());
            }
            all += best;
            println!("  {name}: {best:.1?}");
        }
        println!("  all: {all:.1?}");
    };
    size("built");
    run("never analyzed");

    // A year of edits: every asset retagged once, as AI tagging and hand edits do, a few at a time.
    let started = std::time::Instant::now();
    for chunk in ids.chunks(20) {
        conn.execute_batch("BEGIN").unwrap();
        for id in chunk {
            conn.execute(
                "INSERT OR IGNORE INTO asset_tags (asset_id, tag, source) VALUES (?1, 'mood:calm', 'x')",
                [id],
            )
            .unwrap();
            crate::db::reindex_asset(&conn, id).unwrap();
        }
        conn.execute_batch("COMMIT").unwrap();
    }
    println!("retagged everything in {:.1?}", started.elapsed());
    size("edited");
    run("edited, never analyzed");

    let started = std::time::Instant::now();
    conn.execute_batch("PRAGMA analysis_limit = 1000; PRAGMA optimize = 0x10002;").unwrap();
    println!("PRAGMA optimize (analysis_limit 1000) in {:.1?}", started.elapsed());
    run("optimized");

    let started = std::time::Instant::now();
    conn.execute_batch("ANALYZE;").unwrap();
    println!("full ANALYZE in {:.1?}", started.elapsed());
    run("fully analyzed");

    let started = std::time::Instant::now();
    conn.execute_batch("INSERT INTO assets_fts (assets_fts, rank) VALUES ('merge', 500);").unwrap();
    println!("FTS merge 500 in {:.1?}", started.elapsed());
    let started = std::time::Instant::now();
    conn.execute_batch("INSERT INTO assets_fts (assets_fts) VALUES ('optimize');").unwrap();
    println!("FTS optimize in {:.1?}", started.elapsed());
    size("index merged");
    run("index merged");

    // Deleting a big pack and giving its room back, while someone else keeps making small edits: how long the
    // longest of those waited is how long the deleting held up everyone else.
    let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let editor = {
        let (pool, stop) = (pool.clone(), Arc::clone(&stop));
        std::thread::spawn(move || {
            let c = pool.get().unwrap();
            let mut longest = std::time::Duration::ZERO;
            while !stop.load(std::sync::atomic::Ordering::Acquire) {
                let started = std::time::Instant::now();
                c.execute("UPDATE packs SET name = name WHERE id = 'p1'", []).unwrap();
                longest = longest.max(started.elapsed());
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            longest
        })
    };
    let started = std::time::Instant::now();
    let mut c = pool.get().unwrap();
    crate::db::delete_pack(&mut c, "p0").unwrap();
    println!("deleted the big pack in {:.1?}", started.elapsed());
    size("big pack deleted");
    let started = std::time::Instant::now();
    let tidied = crate::db::tidy(&c).unwrap();
    println!("tidied in {:.1?}: {tidied:?}", started.elapsed());
    size("tidied");
    stop.store(true, std::sync::atomic::Ordering::Release);
    println!("the longest an edit waited meanwhile: {:.1?}", editor.join().unwrap());
}
