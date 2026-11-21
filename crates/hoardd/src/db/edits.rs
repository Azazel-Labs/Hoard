//! Changes to an asset by a person, or by an app acting for one: each is applied and written down in the asset's
//! history (what changed, from what to what, by whom and how), from which it can be undone.

use std::collections::HashSet;

use anyhow::Result;
use hoard_core::{AssetPatch, TagSource};
use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;

use super::{family_ids, now, reindex_asset};

/// Who made a change, and how: a person in the portal (`human`), an app with an access token (`app`), an assistant
/// over MCP (`mcp`), a rule (`rule`) or the AI (`ai`).
#[derive(Clone, Debug)]
pub(crate) struct Actor {
    pub who: Option<String>,
    pub source: &'static str,
}

impl Actor {
    /// Someone signed in (`human`), or an app with one of their access tokens (`app`).
    pub(crate) fn of(who: &crate::auth::Who) -> Self {
        Self { who: Some(who.name().to_owned()), source: if who.is_token() { "app" } else { "human" } }
    }

    /// An assistant over MCP, signed in as `who`.
    pub(crate) fn mcp(who: &crate::auth::Who) -> Self {
        Self { who: Some(who.name().to_owned()), source: "mcp" }
    }
}

/// One change in an asset's history. `field` is a column (`name`, `category`, `description`, `license`,
/// `source_url`, `rating`, `reviewed`), `tag:<tag>` (old and new are its source then, `ai:0.83` with a confidence
/// under 1, or nothing where it didn't have it) or `field:<name>` for a field of the library's own.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct Change {
    pub id: i64,
    pub field: String,
    pub old: Option<String>,
    pub new: Option<String>,
    pub who: Option<String>,
    pub source: String,
    pub at: String,
}

/// The columns a patch may set, and whether changing one keeps the automatic passes from changing it back.
const COLUMNS: [(&str, bool); 6] = [
    ("name", true),
    ("category", true),
    ("description", true),
    ("license", false),
    ("source_url", false),
    ("rating", false),
];

/// The longest a field's name and value may be.
const FIELD_NAME_MAX: usize = 60;
const TEXT_MAX: usize = 2000;

fn clip(s: &str, max: usize) -> String {
    s.trim().chars().take(max).collect()
}

/// Apply `patch` to the asset `id` (and its family, with `apply_to_family`), writing each change into its history.
/// False when there's no such asset. The caller commits.
pub(crate) fn apply(tx: &Connection, id: &str, patch: &AssetPatch, actor: &Actor) -> Result<bool> {
    if !tx.query_row("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1)", [id], |r| r.get::<_, bool>(0))? {
        return Ok(false);
    }
    let at = now();
    let mut log = Log { tx, actor, at: &at, touched: HashSet::new() };
    let targets = if patch.apply_to_family { family_ids(tx, id)? } else { vec![id.to_owned()] };

    // Its own: the name, rating, whether it's reviewed and its fields.
    if let Some(name) = patch.name.as_deref().map(|s| clip(s, 300)).filter(|s| !s.is_empty()) {
        log.set_column(id, "name", Some(name))?;
    }
    if let Some(r) = patch.rating {
        log.set_column(id, "rating", (r > 0).then(|| r.min(5).to_string()))?;
    }
    if let Some(on) = patch.reviewed {
        log.set_reviewed(id, on)?;
    }
    for (key, value) in &patch.fields {
        let key = clip(key, FIELD_NAME_MAX);
        if !key.is_empty() {
            log.set_field(id, &key, Some(clip(value, TEXT_MAX)).filter(|v| !v.is_empty()))?;
        }
    }

    // Shared across the family: the category, description, license, source and tags.
    let text = |v: &Option<String>, max| v.as_deref().map(|s| clip(s, max));
    let shared = [
        ("category", text(&patch.category, 200)),
        ("description", text(&patch.description, TEXT_MAX)),
        ("license", text(&patch.license, 500)),
        ("source_url", text(&patch.source_url, TEXT_MAX)),
    ];
    let add: Vec<String> = patch.add_tags.iter().filter_map(|t| hoard_core::normalize_tag(t)).collect();
    for target in &targets {
        for (col, value) in &shared {
            if let Some(v) = value {
                log.set_column(target, col, Some(v.clone()).filter(|v| !v.is_empty()))?;
            }
        }
        for t in &add {
            log.add_tag(target, t)?;
        }
        for t in &patch.remove_tags {
            log.remove_tag(target, t)?;
        }
        let accept = if patch.accept_all_tags { log.guessed_tags(target)? } else { Vec::new() };
        for t in patch.accept_tags.iter().chain(&accept) {
            log.accept_tag(target, t)?;
        }
        for t in &patch.unreject_tags {
            log.unreject_tag(target, t)?;
        }
    }
    for target in &targets {
        if log.touched(target) {
            tx.execute("UPDATE assets SET updated_at = ?2 WHERE id = ?1", params![target, at])?;
            reindex_asset(tx, target)?;
        }
    }
    Ok(true)
}

/// An asset's history, the latest first.
pub(crate) fn history(conn: &Connection, id: &str, limit: u32) -> Result<Vec<Change>> {
    let mut stmt = conn.prepare(
        "SELECT id, field, old, new, who, source, at FROM asset_history WHERE asset_id = ?1 ORDER BY id DESC LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![id, limit], |r| {
        Ok(Change {
            id: r.get(0)?,
            field: r.get(1)?,
            old: r.get(2)?,
            new: r.get(3)?,
            who: r.get(4)?,
            source: r.get(5)?,
            at: r.get(6)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Put back what one change in an asset's history changed: the field as it was before it, which is itself written
/// down as a change. False when there's no such change for that asset.
pub(crate) fn revert(tx: &Connection, asset_id: &str, change_id: i64, actor: &Actor) -> Result<bool> {
    let change: Option<(String, Option<String>)> = tx
        .query_row(
            "SELECT field, old FROM asset_history WHERE id = ?1 AND asset_id = ?2",
            params![change_id, asset_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    let Some((field, old)) = change else { return Ok(false) };
    let at = now();
    let mut log = Log { tx, actor, at: &at, touched: HashSet::new() };
    if let Some(tag) = field.strip_prefix("tag:") {
        log.set_tag(asset_id, tag, old.as_deref())?;
    } else if let Some(key) = field.strip_prefix("field:") {
        log.set_field(asset_id, key, old)?;
    } else if field == "reviewed" {
        log.set_reviewed(asset_id, old.is_some())?;
    } else if COLUMNS.iter().any(|(c, _)| *c == field) {
        log.set_column(asset_id, &field, old)?;
    }
    if log.touched(asset_id) {
        tx.execute("UPDATE assets SET updated_at = ?2 WHERE id = ?1", params![asset_id, at])?;
        reindex_asset(tx, asset_id)?;
    }
    Ok(true)
}

/// A tag's source as history keeps it: `ai`, or `ai:0.83` when it wasn't sure.
fn tag_state(source: &str, confidence: f64) -> String {
    if confidence >= 0.999 { source.to_owned() } else { format!("{source}:{confidence:.2}") }
}

fn parse_tag_state(s: &str) -> (TagSource, f64) {
    let (source, conf) = s.split_once(':').map_or((s, 1.0_f64), |(s, c)| (s, c.parse().unwrap_or(1.0)));
    (TagSource::parse(source), conf.clamp(0.0, 1.0))
}

/// Changes as they're made, written down as they go.
struct Log<'a> {
    tx: &'a Connection,
    actor: &'a Actor,
    at: &'a str,
    /// The assets something's changed on.
    touched: HashSet<String>,
}

impl Log<'_> {
    fn record(&mut self, id: &str, field: &str, old: Option<&str>, new: Option<&str>) -> Result<()> {
        self.tx.execute(
            "INSERT INTO asset_history (asset_id, field, old, new, who, source, at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, field, old, new, self.actor.who, self.actor.source, self.at],
        )?;
        self.touched.insert(id.to_owned());
        Ok(())
    }

    /// Whether anything of `id`'s changed in this edit.
    fn touched(&self, id: &str) -> bool {
        self.touched.contains(id)
    }

    fn set_column(&mut self, id: &str, col: &str, new: Option<String>) -> Result<()> {
        let Some(&(col, protects)) = COLUMNS.iter().find(|(c, _)| *c == col) else { return Ok(()) };
        let old: Option<String> =
            self.tx.query_row(&format!("SELECT CAST({col} AS TEXT) FROM assets WHERE id = ?1"), [id], |r| r.get(0))?;
        // A name can't be cleared.
        if old == new || (col == "name" && new.is_none()) {
            return Ok(());
        }
        let value = match (col, &new) {
            ("rating", Some(n)) => rusqlite::types::Value::Integer(n.parse().unwrap_or(0)),
            (_, Some(s)) => rusqlite::types::Value::Text(s.clone()),
            (_, None) => rusqlite::types::Value::Null,
        };
        self.tx.execute(
            &format!("UPDATE assets SET {col} = ?2, human_edited = human_edited OR ?3 WHERE id = ?1"),
            params![id, value, protects],
        )?;
        self.record(id, col, old.as_deref(), new.as_deref())
    }

    /// Reviewing an asset also keeps the automatic passes from renaming or recategorising it.
    fn set_reviewed(&mut self, id: &str, on: bool) -> Result<()> {
        let old: Option<String> =
            self.tx.query_row("SELECT reviewed_at FROM assets WHERE id = ?1", [id], |r| r.get(0))?;
        if old.is_some() == on {
            return Ok(());
        }
        let new = on.then(|| self.at.to_owned());
        self.tx.execute(
            "UPDATE assets SET reviewed_at = ?2, human_edited = human_edited OR ?3 WHERE id = ?1",
            params![id, new, on],
        )?;
        self.record(id, "reviewed", old.as_deref(), new.as_deref())
    }

    fn set_field(&mut self, id: &str, key: &str, new: Option<String>) -> Result<()> {
        let old: Option<String> = self
            .tx
            .query_row("SELECT value FROM asset_fields WHERE asset_id = ?1 AND key = ?2", params![id, key], |r| {
                r.get(0)
            })
            .optional()?;
        if old == new {
            return Ok(());
        }
        match &new {
            Some(v) => self.tx.execute(
                "INSERT INTO asset_fields (asset_id, key, value) VALUES (?1, ?2, ?3)
                 ON CONFLICT (asset_id, key) DO UPDATE SET value = excluded.value",
                params![id, key, v],
            )?,
            None => self.tx.execute("DELETE FROM asset_fields WHERE asset_id = ?1 AND key = ?2", params![id, key])?,
        };
        self.record(id, &format!("field:{key}"), old.as_deref(), new.as_deref())
    }

    /// The tag as it's kept: a tag merged into another is that one.
    fn resolve(&self, tag: &str) -> Result<String> {
        let into: Option<String> =
            self.tx.query_row("SELECT into_tag FROM tag_aliases WHERE tag = ?1", [tag], |r| r.get(0)).optional()?;
        Ok(into.unwrap_or_else(|| tag.to_owned()))
    }

    fn tag_now(&self, id: &str, tag: &str) -> Result<Option<(String, f64)>> {
        Ok(self
            .tx
            .query_row(
                "SELECT source, confidence FROM asset_tags WHERE asset_id = ?1 AND tag = ?2",
                params![id, tag],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?)
    }

    fn add_tag(&mut self, id: &str, tag: &str) -> Result<()> {
        let tag = self.resolve(tag)?;
        let old = self.tag_now(id, &tag)?;
        if old.as_ref().is_some_and(|(s, _)| s == "human") {
            return Ok(());
        }
        super::upsert_tag(self.tx, id, &tag, TagSource::Human, 1.0)?;
        let old = old.map(|(s, c)| tag_state(&s, c));
        self.record(id, &format!("tag:{tag}"), old.as_deref(), Some("human"))
    }

    /// A rule's or the AI's tag taken off is remembered, so retagging doesn't put it back.
    fn remove_tag(&mut self, id: &str, tag: &str) -> Result<()> {
        let Some((source, conf)) = self.tag_now(id, tag)? else { return Ok(()) };
        self.tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag = ?2", params![id, tag])?;
        if matches!(source.as_str(), "rule" | "ai" | "llm") {
            self.tx.execute(
                "INSERT INTO rejected_tags (asset_id, tag, source, confidence, at) VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT (asset_id, tag) DO UPDATE SET source = excluded.source, confidence = excluded.confidence,
                   at = excluded.at",
                params![id, tag, source, conf, self.at],
            )?;
        }
        self.record(id, &format!("tag:{tag}"), Some(&tag_state(&source, conf)), None)
    }

    /// The tags a rule or the AI gave it.
    fn guessed_tags(&self, id: &str) -> Result<Vec<String>> {
        let mut stmt = self.tx.prepare(
            "SELECT tag FROM asset_tags WHERE asset_id = ?1 AND source IN ('rule', 'ai', 'llm') ORDER BY tag",
        )?;
        let tags = stmt.query_map([id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
        Ok(tags)
    }

    fn accept_tag(&mut self, id: &str, tag: &str) -> Result<()> {
        let Some((source, conf)) = self.tag_now(id, tag)? else { return Ok(()) };
        if source == "human" {
            return Ok(());
        }
        self.tx.execute(
            "UPDATE asset_tags SET source = 'human', confidence = 1.0 WHERE asset_id = ?1 AND tag = ?2",
            params![id, tag],
        )?;
        self.record(id, &format!("tag:{tag}"), Some(&tag_state(&source, conf)), Some("human"))
    }

    fn unreject_tag(&mut self, id: &str, tag: &str) -> Result<()> {
        let was: Option<(String, f64)> = self
            .tx
            .query_row(
                "DELETE FROM rejected_tags WHERE asset_id = ?1 AND tag = ?2 RETURNING source, confidence",
                params![id, tag],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let Some((source, conf)) = was else { return Ok(()) };
        let state = tag_state(&source, conf);
        self.set_tag(id, tag, Some(&state))
    }

    /// Give the asset the tag from `state` (`ai:0.83`), or take it off with none. Putting a rule's or the AI's tag
    /// back this way forgets that it was turned down.
    fn set_tag(&mut self, id: &str, tag: &str, state: Option<&str>) -> Result<()> {
        let old = self.tag_now(id, tag)?.map(|(s, c)| tag_state(&s, c));
        if old.as_deref() == state {
            return Ok(());
        }
        match state {
            Some(state) => {
                let (source, conf) = parse_tag_state(state);
                self.tx.execute("DELETE FROM rejected_tags WHERE asset_id = ?1 AND tag = ?2", params![id, tag])?;
                self.tx.execute(
                    "INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT (asset_id, tag) DO UPDATE SET source = excluded.source, confidence = excluded.confidence",
                    params![id, tag, source.as_str(), conf],
                )?;
            }
            None => {
                self.tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag = ?2", params![id, tag])?;
            }
        }
        self.record(id, &format!("tag:{tag}"), old.as_deref(), state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{get_asset, matching_ids, search, test_library, upsert_tag};
    use hoard_core::SearchQuery;

    fn library() -> Connection {
        let conn = test_library();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h1', 1, 'x'), ('h2', 1, 'x');
             INSERT INTO packs (id, name, status, tags, created_at, license) VALUES ('p1', 'Forest', 'ready', '[]', 'x', 'CC0');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('a1', 'p1', 'h1', 'a1.png', 'a1', 'png', 'texture', 'x', 'x'),
               ('a2', 'p1', 'h2', 'a2.png', 'a2', 'png', 'texture', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('a1', 'rock', 'ai', 0.8), ('a1', 'moss', 'rule', 1.0), ('a2', 'rock', 'ai', 0.7);",
        )
        .unwrap();
        conn
    }

    fn me() -> Actor {
        Actor { who: Some("sam".into()), source: "human" }
    }

    fn tags(conn: &Connection, id: &str) -> Vec<(String, TagSource)> {
        get_asset(conn, id).unwrap().unwrap().tags.into_iter().map(|t| (t.tag, t.source)).collect()
    }

    #[test]
    fn a_tag_turned_down_stays_off_until_put_back() {
        let conn = library();
        let off = AssetPatch { remove_tags: vec!["rock".into()], ..Default::default() };
        assert!(apply(&conn, "a1", &off, &me()).unwrap());
        assert_eq!(tags(&conn, "a1"), vec![("moss".to_owned(), TagSource::Rule)]);
        upsert_tag(&conn, "a1", "rock", TagSource::Ai, 0.95).unwrap();
        assert_eq!(tags(&conn, "a1").len(), 1, "the AI doesn't bring it back");
        assert_eq!(get_asset(&conn, "a1").unwrap().unwrap().rejected_tags, vec!["rock".to_owned()]);

        let back = AssetPatch { unreject_tags: vec!["rock".into()], ..Default::default() };
        apply(&conn, "a1", &back, &me()).unwrap();
        let rock = get_asset(&conn, "a1").unwrap().unwrap().tags.into_iter().find(|t| t.tag == "rock").unwrap();
        assert_eq!((rock.source, rock.confidence), (TagSource::Ai, 0.8), "back as it was");
        assert!(get_asset(&conn, "a1").unwrap().unwrap().rejected_tags.is_empty());
    }

    #[test]
    fn accepting_makes_a_guess_a_persons_tag() {
        let conn = library();
        let all = AssetPatch { accept_all_tags: true, ..Default::default() };
        apply(&conn, "a1", &all, &me()).unwrap();
        assert_eq!(
            tags(&conn, "a1"),
            vec![("moss".to_owned(), TagSource::Human), ("rock".to_owned(), TagSource::Human)]
        );
        assert_eq!(tags(&conn, "a2"), vec![("rock".to_owned(), TagSource::Ai)], "only that asset's");
    }

    #[test]
    fn reviewed_assets_keep_their_tags_from_the_taggers() {
        let conn = library();
        apply(&conn, "a1", &AssetPatch { reviewed: Some(true), ..Default::default() }, &me()).unwrap();
        upsert_tag(&conn, "a1", "stone", TagSource::Ai, 0.9).unwrap();
        upsert_tag(&conn, "a1", "512px", TagSource::Meta, 1.0).unwrap();
        let mut got: Vec<String> = tags(&conn, "a1").into_iter().map(|t| t.0).collect();
        got.sort();
        assert_eq!(got, ["512px", "moss", "rock"], "facts still come, guesses don't");
        let q = |reviewed| SearchQuery { reviewed: Some(reviewed), ..Default::default() };
        assert_eq!(search(&conn, &q(true), false).unwrap().total, 1);
        assert_eq!(matching_ids(&conn, &q(false)).unwrap(), vec!["a2".to_owned()]);
    }

    #[test]
    fn every_change_is_written_down_and_can_be_undone() {
        let conn = library();
        let patch = AssetPatch {
            name: Some("  Mossy rock ".into()),
            license: Some("CC-BY 4.0".into()),
            rating: Some(4),
            fields: [("biome".to_owned(), "forest".to_owned())].into(),
            add_tags: vec!["Stone".into()],
            ..Default::default()
        };
        apply(&conn, "a1", &patch, &me()).unwrap();
        let a = get_asset(&conn, "a1").unwrap().unwrap();
        assert_eq!(a.name, "Mossy rock");
        assert_eq!(
            (a.license.as_deref(), a.pack_license.as_deref(), a.rating),
            (Some("CC-BY 4.0"), Some("CC0"), Some(4))
        );
        assert_eq!(a.fields.get("biome").map(String::as_str), Some("forest"));

        let changes = history(&conn, "a1", 50).unwrap();
        let fields: Vec<&str> = changes.iter().map(|c| c.field.as_str()).collect();
        assert_eq!(fields, ["tag:stone", "license", "field:biome", "rating", "name"]);
        assert!(changes.iter().all(|c| c.who.as_deref() == Some("sam") && c.source == "human"));

        // The same again changes nothing, so writes nothing down.
        apply(&conn, "a1", &patch, &me()).unwrap();
        assert_eq!(history(&conn, "a1", 50).unwrap().len(), 5);

        let name = changes.iter().find(|c| c.field == "name").unwrap();
        assert!(revert(&conn, "a1", name.id, &me()).unwrap());
        assert_eq!(get_asset(&conn, "a1").unwrap().unwrap().name, "a1");
        let tag = changes.iter().find(|c| c.field == "tag:stone").unwrap();
        revert(&conn, "a1", tag.id, &me()).unwrap();
        assert!(!tags(&conn, "a1").iter().any(|t| t.0 == "stone"));
        assert!(!revert(&conn, "a2", name.id, &me()).unwrap(), "only that asset's changes");
    }
}
