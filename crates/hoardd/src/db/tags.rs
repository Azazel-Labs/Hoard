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
        // What the old name said about itself goes along, unless the tag it joins already says it.
        tx.execute(
            "INSERT INTO tag_info (tag, description, image_id) SELECT ?2, description, image_id FROM tag_info WHERE tag = ?1
             ON CONFLICT (tag) DO UPDATE SET description = COALESCE(tag_info.description, excluded.description),
                                             image_id = COALESCE(tag_info.image_id, excluded.image_id)",
            params![old, into],
        )?;
        tx.execute("DELETE FROM tag_info WHERE tag = ?1", [old])?;
        // So do the broader tags it went under and the narrower ones under it.
        tx.execute("UPDATE OR IGNORE tag_parents SET tag = ?2 WHERE tag = ?1", params![old, into])?;
        tx.execute("UPDATE OR IGNORE tag_parents SET parent = ?2 WHERE parent = ?1", params![old, into])?;
        tx.execute("DELETE FROM tag_parents WHERE tag = ?1 OR parent = ?1", [old])?;
    }
    tx.execute("DELETE FROM tag_parents WHERE tag = parent", [])?;
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

/// What a tag's page shows: how much has it, what it means and the picture that stands for it, and its old names.
#[derive(Debug, Default, PartialEq, serde::Serialize, schemars::JsonSchema)]
pub(crate) struct TagInfo {
    pub tag: String,
    /// How many assets have it, as the tag counts say.
    pub count: u64,
    pub description: Option<String>,
    /// The asset whose picture stands for the tag.
    pub image_id: Option<String>,
    /// Old names merged into it.
    pub aliases: Vec<String>,
    /// When `tag` is itself an old name: the tag it was merged into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into: Option<String>,
    /// The broader tags it goes under.
    pub parents: Vec<String>,
    /// The narrower tags under it, with how many assets have each.
    pub children: Vec<hoard_core::Facet>,
}

/// The SQL for a tag and every tag under it, however deep, for the tag in place of `?`. A loop ends where it
/// started.
const TAG_AND_NARROWER: &str = "WITH RECURSIVE narrower(x) AS (VALUES (?) UNION
     SELECT p.tag FROM tag_parents p JOIN narrower ON p.parent = narrower.x) SELECT x FROM narrower";

/// Every tag under `tag`, however deep (not `tag` itself).
fn narrower_tags(conn: &Connection, tag: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(TAG_AND_NARROWER)?;
    let all = stmt.query_map([tag], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(all.into_iter().filter(|t| t != tag).collect())
}

/// Put a tag under broader ones, in place of those it went under before. A tag can't go under itself or under one
/// of its own narrower tags: that one comes back as `Err`, and nothing changes.
pub(crate) fn set_tag_parents(conn: &mut Connection, tag: &str, parents: &[String]) -> Result<Result<(), String>> {
    let below = narrower_tags(conn, tag)?;
    if let Some(bad) = parents.iter().find(|p| p.as_str() == tag || below.contains(p)) {
        return Ok(Err(bad.clone()));
    }
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM tag_parents WHERE tag = ?1", [tag])?;
    for p in parents {
        tx.execute("INSERT OR IGNORE INTO tag_parents (tag, parent) VALUES (?1, ?2)", params![tag, p])?;
    }
    tx.commit()?;
    Ok(Ok(()))
}

/// Every tag that goes under another: (tag, the broader one), for drawing them as a tree.
pub(crate) fn tag_tree(conn: &Connection) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare("SELECT tag, parent FROM tag_parents ORDER BY parent, tag")?;
    Ok(stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?)
}

/// The tag tree, looked up both ways: each tag's narrower tags and its broader ones, however deep. Read whole when a
/// search needs it: there are a few hundred tags at most, and most libraries have none under others.
#[derive(Debug, Default)]
pub(crate) struct Tree {
    below: std::collections::HashMap<String, Vec<String>>,
    above: std::collections::HashMap<String, Vec<String>>,
}

impl Tree {
    pub(crate) fn load(conn: &Connection) -> Result<Tree> {
        Ok(Tree::of(&tag_tree(conn)?))
    }

    /// The tree of these (tag, broader tag) pairs.
    pub(crate) fn of(pairs: &[(String, String)]) -> Tree {
        use std::collections::HashMap;
        let mut parents: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
        for (tag, parent) in pairs {
            parents.entry(tag).or_default().push(parent);
            children.entry(parent).or_default().push(tag);
        }
        // Everything reachable from a tag, however deep, not the tag itself. A loop ends where it started.
        let reach = |from: &str, step: &HashMap<&str, Vec<&str>>| {
            let (mut seen, mut next) = (Vec::<String>::new(), vec![from]);
            while let Some(t) = next.pop() {
                for &n in step.get(t).into_iter().flatten() {
                    if n != from && !seen.iter().any(|s| s == n) {
                        seen.push(n.to_owned());
                        next.push(n);
                    }
                }
            }
            seen.sort();
            seen
        };
        Tree {
            below: children.keys().map(|&t| (t.to_owned(), reach(t, &children))).collect(),
            above: parents.keys().map(|&t| (t.to_owned(), reach(t, &parents))).collect(),
        }
    }

    /// Every tag under `tag`, however deep.
    pub(crate) fn narrower(&self, tag: &str) -> &[String] {
        self.below.get(tag).map_or(&[], Vec::as_slice)
    }

    /// Each tag with tags under it, with them.
    pub(crate) fn broad(&self) -> impl Iterator<Item = (&str, &[String])> {
        self.below.iter().map(|(t, under)| (t.as_str(), under.as_slice()))
    }

    /// Every tag `tag` goes under, however high.
    pub(crate) fn broader(&self, tag: &str) -> &[String] {
        self.above.get(tag).map_or(&[], Vec::as_slice)
    }
}

/// The longest description a tag keeps.
pub(crate) const TAG_DESCRIPTION_MAX: usize = 4000;

pub(crate) fn tag_info(conn: &Connection, tag: &str) -> Result<TagInfo> {
    let merged_into: Option<String> =
        conn.query_row("SELECT into_tag FROM tag_aliases WHERE tag = ?1", [tag], |r| r.get(0)).optional()?;
    let (description, image_id): (Option<String>, Option<String>) = conn
        .query_row("SELECT description, image_id FROM tag_info WHERE tag = ?1", [tag], |r| Ok((r.get(0)?, r.get(1)?)))
        .optional()?
        .unwrap_or_default();
    // A picture whose asset was deleted stands for nothing.
    let image_id = match image_id {
        Some(id) if conn.query_row("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1)", [&id], |r| r.get(0))? => {
            Some(id)
        }
        _ => None,
    };
    // Every tag's count is kept worked out, so looking up a few is cheap.
    let counts: std::collections::HashMap<String, u64> =
        super::tag_counts(conn, "", false, u32::MAX)?.into_iter().map(|f| (f.value, f.count)).collect();
    let count = counts.get(tag).copied().unwrap_or(0);
    let names = |sql: &str| -> Result<Vec<String>> {
        let mut stmt = conn.prepare(sql)?;
        Ok(stmt.query_map([tag], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?)
    };
    let aliases = names("SELECT tag FROM tag_aliases WHERE into_tag = ?1 ORDER BY tag")?;
    let parents = names("SELECT parent FROM tag_parents WHERE tag = ?1 ORDER BY parent")?;
    let mut children: Vec<hoard_core::Facet> = names("SELECT tag FROM tag_parents WHERE parent = ?1")?
        .into_iter()
        .map(|value| hoard_core::Facet { count: counts.get(&value).copied().unwrap_or(0), value })
        .collect();
    children.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
    Ok(TagInfo { tag: tag.to_owned(), count, description, image_id, aliases, merged_into, parents, children })
}

/// Say what a tag means, or which asset's picture stands for it. `None` leaves either as it is; an empty one clears
/// it.
pub(crate) fn set_tag_info(
    conn: &Connection,
    tag: &str,
    description: Option<&str>,
    image_id: Option<&str>,
) -> Result<()> {
    let given = |s: &str| Some(s.trim()).filter(|s| !s.is_empty()).map(str::to_owned);
    conn.execute("INSERT INTO tag_info (tag) VALUES (?1) ON CONFLICT (tag) DO NOTHING", [tag])?;
    if let Some(d) = description {
        let d: String = d.chars().take(TAG_DESCRIPTION_MAX).collect();
        conn.execute("UPDATE tag_info SET description = ?2 WHERE tag = ?1", params![tag, given(&d)])?;
    }
    if let Some(id) = image_id {
        conn.execute("UPDATE tag_info SET image_id = ?2 WHERE tag = ?1", params![tag, given(id)])?;
    }
    conn.execute("DELETE FROM tag_info WHERE tag = ?1 AND description IS NULL AND image_id IS NULL", [tag])?;
    Ok(())
}

/// Tags merged into others: (old name, the tag it became).
pub(crate) fn tag_aliases(conn: &Connection) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare("SELECT tag, into_tag FROM tag_aliases ORDER BY tag")?;
    Ok(stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?)
}

/// A tag offered while someone types one: a tag in use, or (with `alias`) the tag an old name they typed was merged
/// into.
#[derive(Debug, Clone, PartialEq, serde::Serialize, schemars::JsonSchema)]
pub(crate) struct TagSuggestion {
    pub tag: String,
    pub count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// What to offer for what's typed into a tag box.
#[derive(Debug, Default, PartialEq, serde::Serialize, schemars::JsonSchema)]
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
/// (`stonewall`, `stone-wall`), one a plural of the other, or one letter off for a word long enough to say (but not a
/// number off: `lod-1` isn't `lod-2`).
fn looks_like(tag: &str, q: &str) -> bool {
    if tag == q {
        return false;
    }
    let (t, q) = (squash(tag), squash(q));
    if t == q || singulars(&t).contains(&q) || singulars(&q).contains(&t) {
        return true;
    }
    let digits = |s: &str| s.chars().filter(char::is_ascii_digit).collect::<String>();
    q.chars().count() >= 5 && digits(&t) == digits(&q) && one_edit_apart(&t, &q)
}

/// A tag with only its letters, digits and namespace colon: `stone-wall` and `stonewall` are the same.
fn squash(tag: &str) -> String {
    tag.chars().filter(|c| c.is_alphanumeric() || *c == ':').collect()
}

/// What a word might be the plural of: `tiles` a `tile`, `boxes` a `box`, `bodies` a `body`.
fn singulars(word: &str) -> Vec<String> {
    let mut one: Vec<String> =
        [word.strip_suffix('s'), word.strip_suffix("es")].into_iter().flatten().map(str::to_owned).collect();
    one.extend(word.strip_suffix("ies").map(|w| format!("{w}y")));
    one.retain(|w| !w.is_empty() && !w.ends_with(':'));
    one
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

/// Two tags in use that are probably one: `other` would merge into `keep`, the more used.
#[derive(Debug, Clone, PartialEq, serde::Serialize, schemars::JsonSchema)]
pub(crate) struct LookAlike {
    pub keep: String,
    pub keep_count: u64,
    pub other: String,
    pub other_count: u64,
}

/// Tags in use that look like each other (as [`looks_like`] has it, both ways round), the most used pairs first,
/// leaving out those someone said are different ([`keep_tags_apart`]).
pub(crate) fn lookalike_tags(conn: &Connection, limit: usize) -> Result<Vec<LookAlike>> {
    use std::collections::{BTreeSet, HashMap, HashSet};
    let all = super::tag_counts(conn, "", false, u32::MAX)?;
    // Tags that could look alike share a key: their squashed name, what it's a plural of, or either with a letter
    // dropped. Only tags sharing one are compared, rather than every tag with every other.
    let mut by_key: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, f) in all.iter().enumerate() {
        let squashed = squash(&f.value);
        let mut keys = singulars(&squashed);
        let chars: Vec<char> = squashed.chars().collect();
        if chars.len() >= 5 {
            keys.extend((0..chars.len()).map(|j| chars[..j].iter().chain(&chars[j + 1..]).collect::<String>()));
        }
        keys.push(squashed);
        keys.sort_unstable();
        keys.dedup();
        for k in keys {
            by_key.entry(k).or_default().push(i);
        }
    }
    let apart: HashSet<(String, String)> = {
        let mut stmt = conn.prepare("SELECT a, b FROM tags_apart")?;
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?
    };
    let mut pairs = BTreeSet::new();
    for ids in by_key.values().filter(|ids| ids.len() > 1) {
        for (n, &i) in ids.iter().enumerate() {
            for &j in &ids[n + 1..] {
                let (a, b) = (&all[i].value, &all[j].value);
                if looks_like(a, b) && looks_like(b, a) && !apart.contains(&ordered(a, b)) {
                    pairs.insert((i.min(j), i.max(j)));
                }
            }
        }
    }
    // The more used stays; of two as used, the shorter (the singular), the one with its words apart, then the first
    // A to Z.
    let rank = |f: &hoard_core::Facet| {
        (std::cmp::Reverse(f.count), squash(&f.value).len(), std::cmp::Reverse(f.value.len()), f.value.clone())
    };
    let mut found: Vec<LookAlike> = pairs
        .into_iter()
        .map(|(i, j)| {
            let (keep, other) = if rank(&all[i]) <= rank(&all[j]) { (&all[i], &all[j]) } else { (&all[j], &all[i]) };
            LookAlike {
                keep: keep.value.clone(),
                keep_count: keep.count,
                other: other.value.clone(),
                other_count: other.count,
            }
        })
        .collect();
    found.sort_by(|x, y| {
        (y.keep_count + y.other_count)
            .cmp(&(x.keep_count + x.other_count))
            .then_with(|| x.keep.cmp(&y.keep))
            .then_with(|| x.other.cmp(&y.other))
    });
    found.truncate(limit);
    Ok(found)
}

/// Two tags, the first A to Z first, as `tags_apart` keeps them.
fn ordered(a: &str, b: &str) -> (String, String) {
    if a <= b { (a.to_owned(), b.to_owned()) } else { (b.to_owned(), a.to_owned()) }
}

/// Say two tags that look alike are different, so they aren't offered for merging again.
pub(crate) fn keep_tags_apart(conn: &Connection, a: &str, b: &str) -> Result<()> {
    let (a, b) = ordered(a, b);
    conn.execute("INSERT INTO tags_apart (a, b) VALUES (?1, ?2) ON CONFLICT DO NOTHING", params![a, b])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{get_asset, get_pack, open, search};
    use hoard_core::SearchQuery;

    #[test]
    fn the_tree_looks_up_both_ways_however_deep() {
        let pair = |t: &str, p: &str| (t.to_owned(), p.to_owned());
        let tree = Tree::of(&[
            pair("car", "vehicle"),
            pair("sedan", "car"),
            pair("sedan", "red"),
            pair("a", "b"),
            pair("b", "a"),
        ]);
        assert_eq!(tree.narrower("vehicle"), ["car", "sedan"]);
        assert_eq!(tree.broader("sedan"), ["car", "red", "vehicle"]);
        assert!(tree.narrower("sedan").is_empty() && tree.broader("vehicle").is_empty());
        // A loop ends where it started.
        assert_eq!((tree.narrower("a"), tree.broader("a")), (&["b".to_owned()][..], &["b".to_owned()][..]));
        assert_eq!(Tree::of(&[]).broad().count(), 0);
    }

    #[test]
    fn a_tag_says_what_it_means_and_keeps_it_through_a_merge() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let mut conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x');
             INSERT INTO packs (id, name, status, tags, created_at) VALUES ('p1', 'Forest', 'ready', '[]', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.png', 'a1', 'png', 'texture', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES ('a1', 'rocks', 'human', 1.0);",
        )
        .unwrap();
        set_tag_info(&conn, "rocks", Some("  Loose stones  "), Some("a1")).unwrap();
        set_tag_info(&conn, "stone", Some("Building stone"), None).unwrap();
        let info = tag_info(&conn, "rocks").unwrap();
        assert_eq!(
            (info.count, info.description.as_deref(), info.image_id.as_deref()),
            (1, Some("Loose stones"), Some("a1"))
        );

        // The tag it joins keeps its own description and takes the picture it lacked; the old name says where it went.
        merge_tags(&mut conn, &["rocks".to_owned()], "stone").unwrap();
        let info = tag_info(&conn, "stone").unwrap();
        assert_eq!(info.description.as_deref(), Some("Building stone"));
        assert_eq!(info.image_id.as_deref(), Some("a1"));
        assert_eq!(info.aliases, vec!["rocks".to_owned()]);
        assert_eq!(info.count, 1);
        assert_eq!(tag_info(&conn, "rocks").unwrap().merged_into.as_deref(), Some("stone"));

        // Clearing both forgets the tag's row; a deleted picture stands for nothing.
        set_tag_info(&conn, "stone", Some(""), None).unwrap();
        conn.execute("DELETE FROM assets WHERE id = 'a1'", []).unwrap();
        assert_eq!(tag_info(&conn, "stone").unwrap().image_id, None);
        set_tag_info(&conn, "stone", None, Some("")).unwrap();
        let rows: i64 = conn.query_row("SELECT COUNT(*) FROM tag_info", [], |r| r.get(0)).unwrap();
        assert_eq!(rows, 0);
    }

    #[test]
    fn a_broader_tag_finds_what_is_under_it() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let mut conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x');
             INSERT INTO packs (id, name, status, tags, created_at) VALUES ('p1', 'Garage', 'ready', '[]', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.glb', 'a1', 'glb', 'model', 'x', 'x'),
               ('a2', 'p1', 'h1', 'a2.glb', 'a2', 'glb', 'model', 'x', 'x'),
               ('a3', 'p1', 'h1', 'a3.glb', 'a3', 'glb', 'model', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('a1', 'car', 'human', 1.0), ('a2', 'sedan', 'human', 1.0), ('a3', 'tree', 'human', 1.0);",
        )
        .unwrap();
        let found = |conn: &Connection, tag: &str| -> Vec<String> {
            let q = SearchQuery { tags: vec![tag.to_owned()], ..Default::default() };
            let mut ids: Vec<String> = search(conn, &q, false).unwrap().items.into_iter().map(|a| a.id).collect();
            ids.sort();
            ids
        };
        assert_eq!(found(&conn, "vehicle"), Vec::<String>::new());

        // vehicle > car > sedan: each finds everything under it.
        set_tag_parents(&mut conn, "car", &["vehicle".to_owned()]).unwrap().unwrap();
        set_tag_parents(&mut conn, "sedan", &["car".to_owned()]).unwrap().unwrap();
        assert_eq!(found(&conn, "vehicle"), vec!["a1", "a2"]);
        assert_eq!(found(&conn, "car"), vec!["a1", "a2"]);
        assert_eq!(found(&conn, "sedan"), vec!["a2"]);
        let info = tag_info(&conn, "car").unwrap();
        assert_eq!(info.parents, vec!["vehicle".to_owned()]);
        assert_eq!(info.children, vec![hoard_core::Facet { value: "sedan".to_owned(), count: 1 }]);

        // Going round in a circle is refused, and changes nothing.
        assert_eq!(set_tag_parents(&mut conn, "vehicle", &["sedan".to_owned()]).unwrap(), Err("sedan".to_owned()));
        assert_eq!(set_tag_parents(&mut conn, "car", &["car".to_owned()]).unwrap(), Err("car".to_owned()));
        assert_eq!(tag_info(&conn, "car").unwrap().parents, vec!["vehicle".to_owned()]);

        // A merged tag's place in the tree goes to the tag it joins.
        merge_tags(&mut conn, &["car".to_owned()], "automobile").unwrap();
        assert_eq!(
            tag_tree(&conn).unwrap(),
            vec![("sedan".to_owned(), "automobile".to_owned()), ("automobile".to_owned(), "vehicle".to_owned()),]
        );
        assert_eq!(found(&conn, "vehicle"), vec!["a1", "a2"]);
    }

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

    /// Tags in use that look like each other are offered for merging, the less used into the more, until someone
    /// merges them or says they're different.
    #[test]
    fn tags_that_look_alike_are_offered_for_merging() {
        let dir = tempfile::tempdir().unwrap();
        let pool = open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let mut conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x'), ('h2', 1, 'x'), ('h3', 1, 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Crypt', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.png', 'a1', 'png', 'texture', 'x', 'x'),
               ('a2', 'p1', 'h2', 'a2.png', 'a2', 'png', 'texture', 'x', 'x'),
               ('a3', 'p1', 'h3', 'a3.png', 'a3', 'png', 'texture', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('a1', 'tile', 'human', 1.0), ('a2', 'tile', 'human', 1.0), ('a3', 'tiles', 'ai', 0.7),
               ('a1', 'stone-wall', 'human', 1.0), ('a2', 'stonewall', 'human', 1.0),
               ('a1', 'cobblestone', 'rule', 0.8), ('a3', 'cobblestone', 'rule', 0.8), ('a2', 'cobbelstone', 'ai', 0.6),
               ('a1', 'body', 'human', 1.0), ('a2', 'bodies', 'ai', 0.6),
               ('a1', 'lod-1', 'meta', 1.0), ('a2', 'lod-2', 'meta', 1.0),
               ('a1', 'metal', 'human', 1.0), ('a2', 'meta', 'human', 1.0),
               ('a3', 'cat', 'human', 1.0), ('a1', 'car', 'human', 1.0);",
        )
        .unwrap();
        let pairs = |conn: &Connection| {
            let found = lookalike_tags(conn, 50).unwrap();
            found.into_iter().map(|p| format!("{} > {}", p.other, p.keep)).collect::<Vec<_>>()
        };
        assert_eq!(
            pairs(&conn),
            ["cobbelstone > cobblestone", "tiles > tile", "bodies > body", "stonewall > stone-wall"],
            "plurals, separators and typos; not numbers, nor short words a letter apart"
        );
        keep_tags_apart(&conn, "tiles", "tile").unwrap();
        merge_tags(&mut conn, &["bodies".to_owned()], "body").unwrap();
        assert_eq!(pairs(&conn), ["cobbelstone > cobblestone", "stonewall > stone-wall"]);
        assert_eq!(lookalike_tags(&conn, 1).unwrap().len(), 1);
    }
}
