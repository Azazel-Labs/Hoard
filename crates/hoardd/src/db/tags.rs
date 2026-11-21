//! Tags: adding them, merging them and their aliases.

use anyhow::Result;
use hoard_core::TagSource;
use rusqlite::{Connection, OptionalExtension, params};

use super::reindex_asset;

pub(crate) fn upsert_tag(
    conn: &Connection,
    asset_id: &str,
    tag: &str,
    source: TagSource,
    confidence: f32,
) -> Result<()> {
    // A tag merged into another lands on the one it was merged into.
    let alias: Option<String> =
        conn.query_row("SELECT into_tag FROM tag_aliases WHERE tag = ?1", [tag], |r| r.get(0)).optional()?;
    let tag = alias.as_deref().unwrap_or(tag);
    if source == TagSource::Human {
        conn.execute("DELETE FROM rejected_tags WHERE asset_id = ?1 AND tag = ?2", params![asset_id, tag])?;
    } else {
        // A tag a person turned down stays off. Once a person has said an asset's details are right, rules and the
        // AI leave its tags alone; facts read from its file still come.
        let blocked: bool = conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM rejected_tags WHERE asset_id = ?1 AND tag = ?2)
                 OR (?3 AND EXISTS (SELECT 1 FROM assets WHERE id = ?1 AND reviewed_at IS NOT NULL))",
            params![asset_id, tag, source != TagSource::Meta],
            |r| r.get(0),
        )?;
        if blocked {
            return Ok(());
        }
    }
    // Never let a machine overwrite a human tag; otherwise keep the most confident source.
    conn.execute(
        &format!("INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES (?1, ?2, ?3, ?4) {KEEP_BEST_TAG}"),
        params![asset_id, tag, source.as_str(), f64::from(confidence)],
    )?;
    Ok(())
}

/// When an asset gets a tag it already has: a person's tag stands; otherwise the more confident one wins.
const KEEP_BEST_TAG: &str =
    "ON CONFLICT (asset_id, tag) DO UPDATE SET source = excluded.source, confidence = excluded.confidence
     WHERE asset_tags.source != 'human' AND (excluded.source = 'human' OR excluded.confidence > asset_tags.confidence)";

/// Fold tags into one: every asset with any of `from` gets `into` instead (keeping a person's tag over a machine's),
/// packs' tags follow, and the old names become aliases so the taggers don't bring them back. Merging a single tag
/// into a new name renames it. Returns how many assets changed.
pub(crate) fn merge_tags(conn: &mut Connection, from: &[String], into: &str) -> Result<usize> {
    let tx = conn.transaction()?;
    let mut touched = std::collections::BTreeSet::new();
    for old in from.iter().filter(|t| t.as_str() != into) {
        let mut stmt = tx.prepare("SELECT asset_id FROM asset_tags WHERE tag = ?1")?;
        touched.extend(stmt.query_map([old], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?);
        drop(stmt);
        tx.execute(
            &format!(
                "INSERT INTO asset_tags (asset_id, tag, source, confidence)
                 SELECT asset_id, ?2, source, confidence FROM asset_tags WHERE tag = ?1 AND true {KEEP_BEST_TAG}"
            ),
            params![old, into],
        )?;
        tx.execute("DELETE FROM asset_tags WHERE tag = ?1", [old])?;
        // Aliases that pointed at the old name follow it; one from the new name back to an old one would loop.
        tx.execute("UPDATE tag_aliases SET into_tag = ?2 WHERE into_tag = ?1", params![old, into])?;
        tx.execute(
            "INSERT INTO tag_aliases (tag, into_tag) VALUES (?1, ?2) ON CONFLICT (tag) DO UPDATE SET into_tag = excluded.into_tag",
            params![old, into],
        )?;
    }
    tx.execute("DELETE FROM tag_aliases WHERE tag = ?1", [into])?;
    // Packs keep their (AI-given) tags as a JSON list.
    let packs: Vec<(String, String)> = {
        let mut stmt = tx.prepare("SELECT id, tags FROM packs WHERE tags != '[]'")?;
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?
    };
    for (id, json) in packs {
        let tags: Vec<String> = serde_json::from_str(&json).unwrap_or_default();
        if !tags.iter().any(|t| from.contains(t)) {
            continue;
        }
        let mut merged: Vec<String> = Vec::new();
        for t in tags {
            let t = if from.contains(&t) { into.to_owned() } else { t };
            if !merged.contains(&t) {
                merged.push(t);
            }
        }
        tx.execute("UPDATE packs SET tags = ?2 WHERE id = ?1", params![id, serde_json::to_string(&merged)?])?;
    }
    for id in &touched {
        reindex_asset(&tx, id)?;
    }
    tx.commit()?;
    Ok(touched.len())
}

/// Tags merged into others: (old name, the tag it became).
pub(crate) fn tag_aliases(conn: &Connection) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare("SELECT tag, into_tag FROM tag_aliases ORDER BY tag")?;
    Ok(stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{get_asset, get_pack, open, search};
    use hoard_core::SearchQuery;

    #[test]
    fn merged_tags_stay_merged() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let mut conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x'), ('h2', 1, 'x');
             INSERT INTO packs (id, name, status, tags, created_at) VALUES ('p1', 'Forest', 'ready', '[\"rock\",\"stone\"]', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.png', 'a1', 'png', 'texture', 'x', 'x'),
               ('a2', 'p1', 'h2', 'a2.png', 'a2', 'png', 'texture', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('a1', 'rock', 'rule', 0.8), ('a1', 'stone', 'human', 1.0), ('a2', 'rocks', 'ai', 0.6);",
        )
        .unwrap();
        let s = |v: &[&str]| v.iter().map(ToString::to_string).collect::<Vec<_>>();
        assert_eq!(merge_tags(&mut conn, &s(&["rock", "rocks"]), "stone").unwrap(), 2);
        let tags = |id: &str| {
            get_asset(&conn, id).unwrap().unwrap().tags.into_iter().map(|t| (t.tag, t.source)).collect::<Vec<_>>()
        };
        assert_eq!(tags("a1"), vec![("stone".to_owned(), TagSource::Human)], "a person's tag wins");
        assert_eq!(tags("a2"), vec![("stone".to_owned(), TagSource::Ai)]);
        assert_eq!(get_pack(&conn, "p1").unwrap().unwrap().tags, s(&["stone"]));
        assert_eq!(get_pack(&conn, "p1").unwrap().unwrap().kinds.get("texture"), Some(&2));

        // The taggers' old name lands on the new one, and renaming again carries the aliases along.
        upsert_tag(&conn, "a2", "rock", TagSource::Rule, 0.9).unwrap();
        assert_eq!(tags("a2"), vec![("stone".to_owned(), TagSource::Rule)]);
        merge_tags(&mut conn, &s(&["stone"]), "rock").unwrap();
        assert_eq!(
            tag_aliases(&conn).unwrap(),
            vec![("rocks".to_owned(), "rock".to_owned()), ("stone".to_owned(), "rock".to_owned())]
        );
        let q = SearchQuery { q: Some("rock".into()), ..Default::default() };
        assert_eq!(search(&conn, &q, false).unwrap().total, 2, "search finds the new name");
    }
}
