//! Packs, the folders they were indexed from, and the archive.

use std::collections::HashMap;

use anyhow::Result;
use hoard_core::{Pack, PackStatus};
use rusqlite::{Connection, OptionalExtension, Row, params};

use super::now;

const PACK_COLS: &str = "p.id, p.name, p.status, p.source, p.license, p.description, p.tags, p.error,
    p.files_done, p.files_total, p.created_at, c.store, c.store_url, c.license_tier,
    p.archived_at, p.model_turn, p.model_tilt,
    COALESCE((SELECT MAX(a.updated_at) FROM assets a WHERE a.pack_id = p.id), p.created_at)";

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
        updated_at: r.get(17)?,
        asset_count: 0,
        total_bytes: 0,
        jobs_pending: 0,
        store: r.get(11)?,
        store_url: r.get(12)?,
        license_tier: r.get(13)?,
        preview: Vec::new(),
        preview_kinds: Vec::new(),
        unreal_packages: 0,
        unreal_converted: 0,
        archived_at: r.get(14)?,
        kinds: std::collections::BTreeMap::new(),
        facing: hoard_core::Facing::new(r.get(15)?, r.get(16)?),
    })
}

/// How a model is turned to be shown: its pack's way ([`pack_facing`]), unless a conversion rule turned it upright
/// as it wrote the file.
pub(crate) fn asset_facing(conn: &Connection, asset_id: &str, pack_id: &str) -> Result<hoard_core::Facing> {
    let upright: bool = conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM asset_origin WHERE asset_id = ?1 AND upright = 1)",
        [asset_id],
        |r| r.get(0),
    )?;
    if upright { Ok(hoard_core::Facing::default()) } else { pack_facing(conn, pack_id) }
}

/// The models in a pack a conversion rule turned upright as it wrote them (see [`asset_facing`]).
pub(crate) fn upright_models(conn: &Connection, pack_id: &str) -> Result<std::collections::HashSet<String>> {
    let mut stmt = conn.prepare_cached(
        "SELECT o.asset_id FROM asset_origin o JOIN assets a ON a.id = o.asset_id WHERE a.pack_id = ?1 AND o.upright = 1",
    )?;
    Ok(stmt.query_map([pack_id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?)
}

/// How the pack's models are turned to be shown; upright for a pack that's gone.
pub(crate) fn pack_facing(conn: &Connection, pack_id: &str) -> Result<hoard_core::Facing> {
    let angles: Option<(i64, i64)> = conn
        .query_row("SELECT model_turn, model_tilt FROM packs WHERE id = ?1", [pack_id], |r| Ok((r.get(0)?, r.get(1)?)))
        .optional()?;
    Ok(angles.map_or_else(hoard_core::Facing::default, |(turn, tilt)| hoard_core::Facing::new(turn, tilt)))
}

/// Fill in what each pack holds: its assets by kind (and in all), their bytes, its Unreal packages and how many were
/// converted, and the jobs still to do for it. Kept by migration 83's triggers as assets come and go, but for the
/// conversions, which are few, and the jobs, which are counted by those waiting.
fn fill_counts(conn: &Connection, packs: &mut [Pack]) -> Result<()> {
    let at: HashMap<String, usize> = packs.iter().enumerate().map(|(i, p)| (p.id.clone(), i)).collect();
    let mut kinds = conn.prepare_cached("SELECT pack_id, kind, assets, bytes, unreal FROM pack_kinds")?;
    let mut rows = kinds.query([])?;
    while let Some(r) = rows.next()? {
        let Some(&i) = at.get(r.get_ref(0)?.as_str()?) else { continue };
        let p = &mut packs[i];
        let assets = r.get::<_, i64>(2)? as u64;
        p.kinds.insert(r.get(1)?, assets);
        p.asset_count += assets;
        p.total_bytes += r.get::<_, i64>(3)? as u64;
        p.unreal_packages += r.get::<_, i64>(4)? as u64;
    }
    // Through the conversions, not the pack's assets: a pack has far more of those.
    let mut converted = conn.prepare_cached(
        "SELECT a.pack_id, COUNT(*) FROM asset_origin o CROSS JOIN assets a ON a.id = o.asset_id GROUP BY a.pack_id",
    )?;
    let mut jobs = conn.prepare_cached(
        "SELECT pack_id, COUNT(*) FROM jobs WHERE status IN ('queued', 'running') AND pack_id IS NOT NULL GROUP BY pack_id",
    )?;
    for (stmt, converted) in [(&mut converted, true), (&mut jobs, false)] {
        let mut rows = stmt.query([])?;
        while let Some(r) = rows.next()? {
            let Some(&i) = at.get(r.get_ref(0)?.as_str()?) else { continue };
            let n = r.get::<_, i64>(1)? as u64;
            if converted { packs[i].unreal_converted = n } else { packs[i].jobs_pending = n }
        }
    }
    Ok(())
}

/// The kinds a pack's cover shows, in the order it shows one of each before more of the same.
const COVER_KINDS: [&str; 6] = ["model", "texture", "vector", "audio", "animation", "font"];

/// Up to four of the pack's pictures for its cover: one of each kind first (a model, a texture, a sound…), then more of
/// the same, each kind's first by path.
fn cover(conn: &Connection, pack_id: &str) -> Result<(Vec<String>, Vec<String>)> {
    let mut stmt = conn.prepare_cached(
        "SELECT id FROM assets WHERE pack_id = ?1 AND kind = ?2 AND thumb_hash IS NOT NULL ORDER BY path LIMIT 4",
    )?;
    let mut each = Vec::new();
    for kind in COVER_KINDS {
        each.push((
            kind,
            stmt.query_map(params![pack_id, kind], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?,
        ));
    }
    let (mut ids, mut kinds) = (Vec::new(), Vec::new());
    for n in 0..4 {
        for (kind, found) in &each {
            if let Some(id) = found.get(n)
                && ids.len() < 4
            {
                ids.push(id.clone());
                kinds.push((*kind).to_owned());
            }
        }
    }
    Ok((ids, kinds))
}

/// Which packs [`list_packs`] lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Packs {
    /// The library's, newest first.
    Library,
    /// The library's that are being uploaded, ingested or analyzed, newest first.
    Busy,
    /// The archive's, most recently archived first.
    Archived,
}

/// The packs `which` names.
pub(crate) fn list_packs(conn: &Connection, which: Packs) -> Result<Vec<Pack>> {
    let filter = match which {
        Packs::Library => "WHERE p.archived_at IS NULL ORDER BY p.created_at DESC",
        Packs::Busy => {
            "WHERE p.archived_at IS NULL AND p.status IN ('uploading', 'ingesting', 'analyzing') ORDER BY p.created_at DESC"
        }
        Packs::Archived => "WHERE p.archived_at IS NOT NULL ORDER BY p.archived_at DESC",
    };
    let mut stmt = conn
        .prepare(&format!("SELECT {PACK_COLS} FROM packs p LEFT JOIN pack_context c ON c.pack_id = p.id {filter}"))?;
    let mut rows = stmt.query_map([], pack_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
    for p in &mut rows {
        (p.preview, p.preview_kinds) = cover(conn, &p.id)?;
    }
    fill_counts(conn, &mut rows)?;
    Ok(rows)
}

pub(crate) fn get_pack(conn: &Connection, id: &str) -> Result<Option<Pack>> {
    let mut pack = conn
        .query_row(
            &format!("SELECT {PACK_COLS} FROM packs p LEFT JOIN pack_context c ON c.pack_id = p.id WHERE p.id = ?1"),
            [id],
            pack_from_row,
        )
        .optional()?;
    if let Some(p) = &mut pack {
        fill_counts(conn, std::slice::from_mut(p))?;
    }
    Ok(pack)
}

pub(crate) fn set_pack_status(conn: &Connection, id: &str, status: PackStatus, error: Option<&str>) -> Result<()> {
    conn.execute("UPDATE packs SET status = ?2, error = ?3 WHERE id = ?1", params![id, status.as_str(), error])?;
    Ok(())
}

pub(crate) fn delete_pack(conn: &mut Connection, id: &str) -> Result<bool> {
    let tx = conn.transaction()?;
    let removed = delete_pack_in(&tx, id)?;
    tx.commit()?;
    Ok(removed)
}

/// `delete_pack` as part of a transaction of the caller's.
pub(crate) fn delete_pack_in(tx: &rusqlite::Transaction<'_>, id: &str) -> Result<bool> {
    tx.execute("DELETE FROM jobs WHERE pack_id = ?1 AND status IN ('queued', 'failed')", [id])?;
    Ok(tx.execute("DELETE FROM packs WHERE id = ?1", [id])? > 0)
}

// Watched folders

/// Remember that an asset of a watched pack was deleted for good, so a rescan doesn't add its file back: its path,
/// and the path of the file it was converted from. Does nothing for a pack Hoard keeps itself.
pub(crate) fn skip_on_rescan(c: &Connection, asset_id: &str) -> Result<()> {
    c.execute(
        "INSERT OR IGNORE INTO skipped_files (pack_id, path)
         SELECT a.pack_id, a.path FROM assets a JOIN packs p ON p.id = a.pack_id
          WHERE a.id = ?1 AND p.folder IS NOT NULL
         UNION
         SELECT a.pack_id, o.path FROM asset_origin o JOIN assets a ON a.id = o.asset_id JOIN packs p ON p.id = a.pack_id
          WHERE o.asset_id = ?1 AND p.folder IS NOT NULL",
        [asset_id],
    )?;
    Ok(())
}

/// The files of a pack's folder Hoard already accounts for: its assets' paths, the files assets were converted
/// from, and files deleted for good.
pub(crate) fn known_paths(c: &Connection, pack_id: &str) -> Result<std::collections::HashSet<String>> {
    let mut stmt = c.prepare(
        "SELECT path FROM assets WHERE pack_id = ?1
         UNION SELECT o.path FROM asset_origin o JOIN assets a ON a.id = o.asset_id WHERE a.pack_id = ?1
         UNION SELECT path FROM skipped_files WHERE pack_id = ?1",
    )?;
    Ok(stmt.query_map([pack_id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?)
}

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
        tx.execute("DELETE FROM jobs WHERE pack_id = ?1 AND status IN ('queued', 'failed')", [id])?;
        tx.execute(
            "INSERT OR IGNORE INTO skipped_folders (location_id, folder)
             SELECT location_id, folder FROM packs WHERE id = ?1 AND location_id IS NOT NULL AND folder IS NOT NULL",
            [id],
        )?;
        n.packs += tx.execute("DELETE FROM packs WHERE id = ?1", [id])? as u64;
        gone.push(id.clone());
    }
    for id in &asset_ids {
        let archived: bool = tx.query_row(
            "SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1 AND archived_at IS NOT NULL)",
            [id],
            |r| r.get(0),
        )?;
        if !archived {
            continue;
        }
        skip_on_rescan(&tx, id)?;
        n.assets += tx.execute("DELETE FROM assets WHERE id = ?1 AND archived_at IS NOT NULL", [id])? as u64;
    }
    tx.commit()?;
    Ok((n, gone))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{open, search, tag_counts};
    use hoard_core::SearchQuery;

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
        assert_eq!(
            list_packs(&conn, Packs::Library).unwrap().iter().map(|p| p.id.clone()).collect::<Vec<_>>(),
            s(&["p1"])
        );
        assert!(
            tag_counts(&conn, "", false, 100).unwrap().iter().all(|f| f.value != "stone"),
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

    /// The counts kept for each pack stay what counting its assets gives, through assets added, changed, moved and
    /// deleted, and a pack deleted with its assets.
    #[test]
    fn a_packs_counts_follow_its_assets() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let conn = pool.get().unwrap();
        // Each pack's kinds, bytes and Unreal packages, kept and counted afresh.
        let check = |conn: &Connection| {
            let mut packs = list_packs(conn, Packs::Library).unwrap();
            packs.sort_by(|a, b| a.id.cmp(&b.id));
            let kept: Vec<_> = packs
                .iter()
                .map(|p| (p.id.clone(), p.kinds.clone(), p.asset_count, p.total_bytes, p.unreal_packages))
                .collect();
            let counted: Vec<_> = packs
                .iter()
                .map(|p| {
                    let kinds = conn
                        .prepare("SELECT kind, COUNT(*) FROM assets WHERE pack_id = ?1 GROUP BY kind")
                        .unwrap()
                        .query_map([&p.id], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)? as u64)))
                        .unwrap()
                        .collect::<rusqlite::Result<_>>()
                        .unwrap();
                    let (n, bytes, unreal): (i64, i64, i64) = conn
                        .query_row(
                            "SELECT COUNT(*), COALESCE(SUM(b.size), 0), COALESCE(SUM(a.ext IN ('uasset', 'umap')), 0)
                               FROM assets a JOIN blobs b ON b.hash = a.blob_hash WHERE a.pack_id = ?1",
                            [&p.id],
                            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
                        )
                        .unwrap();
                    (p.id.clone(), kinds, n as u64, bytes as u64, unreal as u64)
                })
                .collect();
            assert_eq!(kept, counted);
            kept
        };
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 10, 'x'), ('h2', 200, 'x'), ('h3', 3000, 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'One', 'ready', 'x'), ('p2', 'Two', 'ready', 'y');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a', 'p1', 'h1', 'a.png', 'a', 'png', 'texture', 'x', 'x'),
               ('b', 'p1', 'h2', 'b.png', 'b', 'png', 'texture', 'x', 'x'),
               ('c', 'p1', 'h3', 'c.uasset', 'c', 'uasset', 'other', 'x', 'x'),
               ('d', 'p2', 'h1', 'd.glb', 'd', 'glb', 'model', 'x', 'x');",
        )
        .unwrap();
        let first = check(&conn);
        assert_eq!((first[0].2, first[0].3, first[0].4), (3, 3210, 1));

        conn.execute_batch(
            "UPDATE assets SET kind = 'sheet' WHERE id = 'a';
             UPDATE assets SET blob_hash = 'h3', name = 'Bee' WHERE id = 'b';
             UPDATE assets SET ext = 'umap' WHERE id = 'd';
             UPDATE assets SET thumb_hash = 'h1' WHERE id = 'c';",
        )
        .unwrap();
        check(&conn);
        conn.execute_batch("UPDATE assets SET pack_id = 'p2' WHERE id = 'c'; DELETE FROM assets WHERE id = 'a';")
            .unwrap();
        let moved = check(&conn);
        assert_eq!(moved[0].1.keys().collect::<Vec<_>>(), ["texture"]);
        // Upserted the way a material is written again.
        conn.execute(
            "INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
               VALUES ('b2', 'p1', 'h1', 'b.png', 'b', 'png', 'texture', 'x', 'x')
             ON CONFLICT (pack_id, path) DO UPDATE SET blob_hash = excluded.blob_hash",
            [],
        )
        .unwrap();
        check(&conn);
        conn.execute("DELETE FROM packs WHERE id = 'p2'", []).unwrap();
        check(&conn);
        let left: i64 =
            conn.query_row("SELECT COUNT(*) FROM pack_kinds WHERE pack_id = 'p2'", [], |r| r.get(0)).unwrap();
        assert_eq!(left, 0);
    }

    /// A pack's cover takes one picture of each kind before a second of any, models first, each kind's by path.
    #[test]
    fn a_packs_cover_shows_one_of_each_kind_first() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'P', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, thumb_hash, created_at, updated_at) VALUES
               ('t2', 'p', 'h', 'z/rock.png', 'rock', 'png', 'texture', 'h', 'x', 'x'),
               ('t1', 'p', 'h', 'a/wood.png', 'wood', 'png', 'texture', 'h', 'x', 'x'),
               ('t0', 'p', 'h', 'a/bare.png', 'bare', 'png', 'texture', NULL, 'x', 'x'),
               ('s1', 'p', 'h', 'boom.ogg', 'boom', 'ogg', 'audio', 'h', 'x', 'x'),
               ('m2', 'p', 'h', 'b/ship.glb', 'ship', 'glb', 'model', 'h', 'x', 'x'),
               ('m1', 'p', 'h', 'a/crate.glb', 'crate', 'glb', 'model', 'h', 'x', 'x'),
               ('d1', 'p', 'h', 'notes.txt', 'notes', 'txt', 'document', 'h', 'x', 'x');",
        )
        .unwrap();
        let (ids, kinds) = cover(&conn, "p").unwrap();
        assert_eq!(ids, ["m1", "t1", "s1", "m2"]);
        assert_eq!(kinds, ["model", "texture", "audio", "model"]);
    }

    /// A pack was updated when any of its assets last was; one with none yet, when it came.
    #[test]
    fn a_pack_was_updated_when_its_assets_last_were() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES
               ('p1', 'Forest', 'ready', '2026-01-01T00:00:00Z'), ('p2', 'Empty', 'ready', '2026-02-01T00:00:00Z');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h', 'a1.wav', 'a1', 'wav', 'audio', '2026-01-01T00:00:00Z', '2026-03-05T00:00:00Z'),
               ('a2', 'p1', 'h', 'a2.wav', 'a2', 'wav', 'audio', '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z');",
        )
        .unwrap();
        let updated = |id: &str| get_pack(&conn, id).unwrap().unwrap().updated_at;
        assert_eq!(updated("p1"), "2026-03-05T00:00:00Z");
        assert_eq!(updated("p2"), "2026-02-01T00:00:00Z");
        let listed: Vec<_> = list_packs(&conn, Packs::Library).unwrap().into_iter().map(|p| p.updated_at).collect();
        assert_eq!(listed, ["2026-02-01T00:00:00Z", "2026-03-05T00:00:00Z"]);
        // Each pack's is looked up in an index, not by reading all its assets.
        let plan: Vec<String> = conn
            .prepare("EXPLAIN QUERY PLAN SELECT MAX(a.updated_at) FROM assets a WHERE a.pack_id = 'p1'")
            .unwrap()
            .query_map([], |r| r.get(3))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert!(plan.iter().any(|p| p.contains("assets_pack_updated")), "{plan:?}");
    }
}
