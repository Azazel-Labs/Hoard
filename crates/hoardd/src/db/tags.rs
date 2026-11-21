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

/// A tag offered while someone types one: a tag in use, or (with `alias`) the tag an old name they typed was merged
/// into.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub(crate) struct TagSuggestion {
    pub tag: String,
    pub count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// What to offer for what's typed into a tag box.
#[derive(Debug, Default, PartialEq, serde::Serialize)]
pub(crate) struct TagSuggestions {
    /// Tags that match, the best first: the tag itself, then those starting with it, those whose name after the
    /// namespace does, those with a word starting with it, and those containing it; the more used first within each.
    pub matches: Vec<TagSuggestion>,
    /// What's typed, as it would be saved (`None` for nothing that makes a tag).
    pub normalized: Option<String>,
    /// The tag it would land on is already in use (directly, or as a merged tag's old name).
    pub exists: bool,
    /// When it would make a new tag: tags in use it's probably meant to be (another spelling, a plural, a typo).
    pub near: Vec<TagSuggestion>,
}

/// Tags to offer for `typed`: see [`TagSuggestions`]. Nothing typed offers the most used.
pub(crate) fn suggest_tags(conn: &Connection, typed: &str, limit: usize) -> Result<TagSuggestions> {
    let all = super::tag_counts(conn, "", false, u32::MAX)?;
    let Some(q) = hoard_core::normalize_tag(typed) else {
        let matches =
            all.iter().take(limit).map(|f| TagSuggestion { tag: f.value.clone(), count: f.count, alias: None });
        return Ok(TagSuggestions { matches: matches.collect(), ..Default::default() });
    };
    let count = |tag: &str| all.iter().find(|f| f.value == tag).map_or(0, |f| f.count);
    let mut ranked: Vec<(u8, TagSuggestion)> = all
        .iter()
        .filter_map(|f| {
            rank(&f.value, &q).map(|r| (r, TagSuggestion { tag: f.value.clone(), count: f.count, alias: None }))
        })
        .collect();
    // An old name that matches offers the tag it became, unless that's offered already on its own.
    let aliases = tag_aliases(conn)?;
    for (old, into) in &aliases {
        let Some(r) = rank(old, &q) else { continue };
        match ranked.iter_mut().find(|(_, s)| &s.tag == into) {
            Some((best, _)) => *best = (*best).min(r),
            None => ranked.push((r, TagSuggestion { tag: into.clone(), count: count(into), alias: Some(old.clone()) })),
        }
    }
    ranked.sort_by(|(ra, a), (rb, b)| ra.cmp(rb).then(b.count.cmp(&a.count)).then(a.tag.cmp(&b.tag)));
    let exists = all.iter().any(|f| f.value == q) || aliases.iter().any(|(old, _)| *old == q);
    let near = if exists || q.chars().count() < 3 {
        Vec::new()
    } else {
        let mut near: Vec<TagSuggestion> = all
            .iter()
            .filter(|f| looks_like(&f.value, &q))
            .map(|f| TagSuggestion { tag: f.value.clone(), count: f.count, alias: None })
            .collect();
        near.sort_by(|a, b| b.count.cmp(&a.count).then(a.tag.cmp(&b.tag)));
        near.truncate(3);
        near
    };
    Ok(TagSuggestions {
        matches: ranked.into_iter().take(limit).map(|(_, s)| s).collect(),
        normalized: Some(q),
        exists,
        near,
    })
}

/// How well `tag` matches what's typed (lower is better), or `None` when it doesn't.
fn rank(tag: &str, q: &str) -> Option<u8> {
    let value = tag.split_once(':').map_or(tag, |(_, v)| v);
    if tag == q {
        Some(0)
    } else if tag.starts_with(q) {
        Some(1)
    } else if value.starts_with(q) {
        Some(2)
    } else if tag.split(['-', ':', '/', '.']).any(|w| w.starts_with(q)) {
        Some(3)
    } else if tag.contains(q) {
        Some(4)
    } else {
        None
    }
}

/// Whether `tag` is probably what someone meant by `q`, a tag not in use: the same once its separators are taken out
/// (`stonewall`, `stone-wall`), one a plural of the other, or one letter off for a word long enough to say.
fn looks_like(tag: &str, q: &str) -> bool {
    if tag == q {
        return false;
    }
    let squash = |s: &str| s.chars().filter(|c| c.is_alphanumeric() || *c == ':').collect::<String>();
    let (t, q) = (squash(tag), squash(q));
    if t == q {
        return true;
    }
    let single = |s: &str| s.strip_suffix("es").or_else(|| s.strip_suffix('s')).map(str::to_owned);
    if single(&t).as_deref() == Some(q.as_str()) || single(&q).as_deref() == Some(t.as_str()) {
        return true;
    }
    q.chars().count() >= 5 && one_edit_apart(&t, &q)
}

/// Whether two words differ by one letter added, dropped or changed, or two side by side swapped.
fn one_edit_apart(a: &str, b: &str) -> bool {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let (short, long) = if a.len() <= b.len() { (&a, &b) } else { (&b, &a) };
    if long.len() - short.len() > 1 {
        return false;
    }
    let head = short.iter().zip(long.iter()).take_while(|(x, y)| x == y).count();
    if short.len() == long.len() {
        if head == short.len() {
            return false;
        }
        let swapped = head + 1 < short.len()
            && short[head] == long[head + 1]
            && short[head + 1] == long[head]
            && short[head + 2..] == long[head + 2..];
        swapped || short[head + 1..] == long[head + 1..]
    } else {
        short[head..] == long[head + 1..]
    }
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

    #[test]
    fn suggests_tags_and_catches_near_copies() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let mut conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x'), ('h2', 1, 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Crypt', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.png', 'a1', 'png', 'texture', 'x', 'x'),
               ('a2', 'p1', 'h2', 'a2.png', 'a2', 'png', 'texture', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('a1', 'stone-wall', 'human', 1.0), ('a2', 'stone-wall', 'human', 1.0), ('a1', 'style:stone', 'ai', 0.7),
               ('a1', 'cobblestone', 'rule', 0.8), ('a2', 'wall', 'human', 1.0), ('a1', 'rocks', 'human', 1.0);",
        )
        .unwrap();
        merge_tags(&mut conn, &["rocks".to_owned()], "rock").unwrap();
        let tags = |s: &TagSuggestions| s.matches.iter().map(|m| m.tag.clone()).collect::<Vec<_>>();

        let s = suggest_tags(&conn, "Stone", 10).unwrap();
        assert_eq!(tags(&s), ["stone-wall", "style:stone", "cobblestone"], "start, after the namespace, inside");
        assert_eq!((s.normalized.as_deref(), s.exists, s.near.len()), (Some("stone"), false, 0));

        // An old name offers what it was merged into, and counts as there.
        let s = suggest_tags(&conn, "rocks", 10).unwrap();
        assert_eq!(s.matches, [TagSuggestion { tag: "rock".into(), count: 1, alias: Some("rocks".into()) }]);
        assert!(s.exists);

        // A new tag that's probably one in use says so.
        let near =
            |typed: &str| suggest_tags(&conn, typed, 10).unwrap().near.into_iter().map(|m| m.tag).collect::<Vec<_>>();
        assert_eq!(near("stone wall"), Vec::<String>::new(), "the same tag once it's normalized");
        assert_eq!(near("stonewall"), ["stone-wall"]);
        assert_eq!(near("walls"), ["wall"]);
        assert_eq!(near("cobbelstone"), ["cobblestone"], "letters swapped");
        assert_eq!(near("cobblestones"), ["cobblestone"]);
        assert_eq!(near("wal"), Vec::<String>::new(), "too short to guess at");

        assert_eq!(suggest_tags(&conn, "", 2).unwrap().matches.len(), 2, "nothing typed offers the most used");
    }
}
