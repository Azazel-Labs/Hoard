//! Search: the filters a query becomes, full-text and look-ranked, and folder browsing.

use anyhow::Result;
use hoard_core::{Asset, Facet, SearchQuery};
use rusqlite::types::Value as SqlValue;
use rusqlite::{Connection, params, params_from_iter};

use super::{ASSET_COLS, asset_from_row, attach_tags};

/// Turn free text into a safe FTS5 query: every term becomes a quoted prefix match, and all must match.
fn fts_query(q: &str) -> Option<String> {
    let terms: Vec<String> = q
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| format!("\"{}\"*", t.to_lowercase()))
        .collect();
    if terms.is_empty() { None } else { Some(terms.join(" AND ")) }
}

struct Filter {
    join: String,
    clauses: Vec<String>,
    params: Vec<SqlValue>,
    ranked: bool,
}

/// Assets ranked by how they look (see `looks`): thumbnail hashes with a score, best first.
#[derive(Debug, Default)]
pub(crate) struct LookRank {
    pub(crate) scores: Vec<(String, f32)>,
    /// Leave this asset out (the one "more like this" started from).
    pub(crate) exclude: Option<String>,
    /// With a text query: match either the words or the look, and rank by both (text-to-image search). Without
    /// it, only assets in `scores` match.
    pub(crate) blend: bool,
    /// What the hashes are of.
    pub(crate) keys: Keys,
}

/// What a [`LookRank`]'s hashes are of.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Keys {
    /// The assets' thumbnails (pictures, see `looks` and `clip`).
    #[default]
    Thumbs,
    /// The assets' files (sounds, see `sounds` and `clap`).
    Files,
    /// Sounds by their files, everything else by its thumbnail: image and sound search together.
    Mixed,
}

const FTS_JOIN: &str = "JOIN (SELECT asset_id, bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0) AS rank
          FROM assets_fts WHERE assets_fts MATCH ?) f ON f.asset_id = a.id";
fn look_join(look: &LookRank) -> &'static str {
    match look.keys {
        Keys::Thumbs => "JOIN temp.look_rank lr ON lr.hash = a.thumb_hash",
        Keys::Files => "JOIN temp.look_rank lr ON lr.hash = a.blob_hash",
        Keys::Mixed => {
            "JOIN temp.look_rank lr ON lr.hash = CASE WHEN a.kind = 'audio' THEN a.blob_hash ELSE a.thumb_hash END"
        }
    }
}

/// Music by tempo, as its `bpm:` tag has it or (when its file states none) as the other way to count what was heard,
/// and by key, as its `key:` tag has it.
fn music_filter(q: &SearchQuery, f: &mut Filter) {
    if q.bpm_min.is_some() || q.bpm_max.is_some() {
        let (lo, hi) = (f64::from(q.bpm_min.unwrap_or(0.0)), f64::from(q.bpm_max.unwrap_or(10_000.0)));
        f.clauses.push(
            "(EXISTS (SELECT 1 FROM asset_tags t WHERE t.asset_id = a.id AND t.tag >= 'bpm:' AND t.tag < 'bpm;'
                      AND CAST(substr(t.tag, 5) AS REAL) BETWEEN ? AND ?)
              OR (json_extract(a.meta, '$.tempo_bpm') IS NULL
                  AND json_extract(a.meta, '$.measured_bpm_alt') BETWEEN ? AND ?))"
                .to_owned(),
        );
        f.params.extend([lo, hi, lo, hi].map(SqlValue::Real));
    }
    if let Some(key) = q.key.as_deref().filter(|k| !k.trim().is_empty()) {
        match crate::rhythm::Key::parse(key) {
            Some(k) => {
                let keys = if q.compatible_keys { k.compatible().to_vec() } else { vec![k] };
                f.clauses.push(format!(
                    "EXISTS (SELECT 1 FROM asset_tags t WHERE t.asset_id = a.id AND t.tag IN ({}))",
                    keys.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
                ));
                f.params.extend(keys.iter().map(|k| SqlValue::Text(k.tag())));
            }
            // A key that isn't one matches nothing.
            None => f.clauses.push("0".to_owned()),
        }
    }
}

fn build_filter(q: &SearchQuery, include_kind: bool, look: Option<&LookRank>) -> Filter {
    let mut f = Filter { join: String::new(), clauses: Vec::new(), params: Vec::new(), ranked: false };
    let fts = q.q.as_deref().and_then(fts_query);
    match (fts, look) {
        (Some(fts), Some(look)) if look.blend => {
            f.join = format!("LEFT {FTS_JOIN} LEFT {}", look_join(look));
            f.clauses.push("(f.asset_id IS NOT NULL OR lr.hash IS NOT NULL)".to_owned());
            f.params.push(SqlValue::Text(fts));
            f.ranked = true;
        }
        (fts, look) => {
            if let Some(fts) = fts {
                FTS_JOIN.clone_into(&mut f.join);
                f.params.push(SqlValue::Text(fts));
                f.ranked = true;
            }
            if let Some(look) = look {
                f.join = format!("{} {}", f.join, look_join(look));
            }
        }
    }
    if let Some(id) = look.and_then(|l| l.exclude.as_deref()) {
        f.clauses.push("a.id != ?".to_owned());
        f.params.push(SqlValue::Text(id.to_owned()));
    }
    if include_kind && let Some(kind) = q.kind.as_deref().filter(|k| !k.is_empty()) {
        f.clauses.push("a.kind = ?".to_owned());
        f.params.push(SqlValue::Text(kind.to_owned()));
    }
    if let Some(pack) = q.pack.as_deref().filter(|p| !p.is_empty()) {
        f.clauses.push("a.pack_id = ?".to_owned());
        f.params.push(SqlValue::Text(pack.to_owned()));
    }
    if let Some(folder) = browsed_folder(q) {
        // Under the folder: a range of the (pack_id, path) index, '/' + 1 being '0'.
        if !folder.is_empty() {
            f.clauses.push("a.path > ? AND a.path < ?".to_owned());
            f.params.push(SqlValue::Text(format!("{folder}/")));
            f.params.push(SqlValue::Text(format!("{folder}0")));
        }
        // Directly in it, unless searching it.
        if fts_query_of(q).is_none() {
            f.clauses.push("instr(substr(a.path, ?), '/') = 0".to_owned());
            f.params.push(SqlValue::Integer(folder_skip(folder)));
        }
    }
    if let Some(ext) = q.ext.as_deref().filter(|e| !e.is_empty()) {
        f.clauses.push("a.ext = ?".to_owned());
        f.params.push(SqlValue::Text(ext.trim_start_matches('.').to_lowercase()));
    }
    for tag in &q.tags {
        f.clauses.push("EXISTS (SELECT 1 FROM asset_tags t WHERE t.asset_id = a.id AND t.tag = ?)".to_owned());
        f.params.push(SqlValue::Text(tag.clone()));
    }
    if q.hide_supporting {
        f.clauses.push(format!("NOT {SUPPORTING}"));
    }
    music_filter(q, &mut f);
    f.clauses.push(
        match (q.archived, q.pack.as_deref().is_some_and(|p| !p.is_empty())) {
            (false, _) => LIVE,
            // Browsing one archived pack: everything archived from it, one by one or with the pack.
            (true, true) => {
                "(a.archived_at IS NOT NULL OR a.pack_id IN (SELECT id FROM packs WHERE archived_at IS NOT NULL))"
            }
            // The archive's own grid: assets archived one by one. Archived packs are listed as packs.
            (true, false) => {
                "a.archived_at IS NOT NULL AND a.pack_id NOT IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)"
            }
        }
        .to_owned(),
    );
    if let Some(user) = q.favorites_of {
        f.clauses.push("a.id IN (SELECT asset_id FROM favorites WHERE user_id = ?)".to_owned());
        f.params.push(SqlValue::Integer(user));
    }
    if let Some(p) = q.project.as_deref().filter(|p| !p.is_empty()) {
        f.clauses.push(
            "a.id IN (SELECT pa.asset_id FROM project_assets pa JOIN projects p ON p.id = pa.project_id
                      WHERE CAST(p.id AS TEXT) = ? OR p.name = ?)"
                .to_owned(),
        );
        f.params.push(SqlValue::Text(p.to_owned()));
        f.params.push(SqlValue::Text(p.to_owned()));
    }
    if let Some(c) = q.collection.as_deref().filter(|c| !c.is_empty()) {
        f.clauses.push(
            "a.id IN (SELECT ca.asset_id FROM collection_assets ca JOIN collections c ON c.id = ca.collection_id
                      WHERE c.id = ? OR c.name = ?)"
                .to_owned(),
        );
        f.params.push(SqlValue::Text(c.to_owned()));
        f.params.push(SqlValue::Text(c.to_owned()));
    }
    f
}

/// The folder of a pack being browsed, without slashes at either end (`""` for its top).
fn browsed_folder(q: &SearchQuery) -> Option<&str> {
    q.pack.as_deref().filter(|p| !p.is_empty())?;
    q.folder.as_deref().map(|f| f.trim_matches('/'))
}

fn fts_query_of(q: &SearchQuery) -> Option<String> {
    q.q.as_deref().and_then(fts_query)
}

/// Where a path's part below `folder` starts, counting from 1 in characters as SQLite's `substr` does.
fn folder_skip(folder: &str) -> i64 {
    if folder.is_empty() { 1 } else { folder.chars().count() as i64 + 2 }
}

/// The folders directly in the one being browsed, each with how many matches are under it.
fn subfolders(conn: &Connection, q: &SearchQuery, look: Option<&LookRank>) -> Result<Vec<Facet>> {
    let Some(folder) = browsed_folder(q) else { return Ok(Vec::new()) };
    if fts_query_of(q).is_some() {
        return Ok(Vec::new());
    }
    let everything = SearchQuery { folder: None, ..q.clone() };
    let mut f = build_filter(&everything, true, look);
    if !folder.is_empty() {
        f.clauses.push("a.path > ? AND a.path < ?".to_owned());
        f.params.push(SqlValue::Text(format!("{folder}/")));
        f.params.push(SqlValue::Text(format!("{folder}0")));
    }
    let mut params = vec![SqlValue::Integer(folder_skip(folder))];
    params.extend(f.params.iter().cloned());
    let mut stmt = conn.prepare(&format!(
        "SELECT substr(rest, 1, instr(rest, '/') - 1) AS name, COUNT(*)
           FROM (SELECT substr(a.path, ?) AS rest FROM assets a {} {})
          WHERE instr(rest, '/') > 0 GROUP BY name ORDER BY name COLLATE NOCASE",
        f.join,
        f.where_sql()
    ))?;
    let rows = stmt
        .query_map(params_from_iter(&params), |r| Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 }))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

/// In the library rather than the archive: neither the asset nor its pack is archived. Expects the asset as `a`.
pub(crate) const LIVE: &str =
    "a.archived_at IS NULL AND a.pack_id NOT IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)";

/// An asset another asset needs as raw data (a glTF's .bin, an OBJ's .mtl), the store's file describing a model
/// (a Megascans `<id>.json`) or a map of a material (its roughness
/// map, say): not something you'd browse for. The material is, and it shows its maps, unless it's the material of
/// one object only (`owned_by`, see `materials::mark_owned`): then it's part of that model, which shows it.
/// A sprite sheet's pages, and the loose images it has its sprites as too, are shown with the sheet.
pub(crate) const SUPPORTING: &str = "(EXISTS (SELECT 1 FROM asset_links l WHERE l.to_id = a.id
     AND (l.rel IN ('buffer', 'sidecar', 'sprite', 'format', 'sample') OR (l.rel = 'material' AND a.ext = 'mtl')
          OR (l.rel = 'texture' AND a.kind = 'texture'
              AND EXISTS (SELECT 1 FROM assets s WHERE s.id = l.from_id AND s.kind = 'sheet'))))
     OR EXISTS (SELECT 1 FROM material_maps m WHERE m.texture_id = a.id)
     OR (a.ext = 'material' AND EXISTS (SELECT 1 FROM assets o WHERE o.id = json_extract(a.meta, '$.owned_by'))))";

impl Filter {
    fn where_sql(&self) -> String {
        if self.clauses.is_empty() { String::new() } else { format!("WHERE {}", self.clauses.join(" AND ")) }
    }
}

pub(crate) struct SearchPage {
    pub(crate) items: Vec<Asset>,
    pub(crate) total: u64,
    pub(crate) kinds: Vec<Facet>,
    pub(crate) tags: Vec<Facet>,
    pub(crate) supporting_hidden: u64,
    pub(crate) folders: Vec<Facet>,
}

pub(crate) fn search(conn: &Connection, q: &SearchQuery, with_facets: bool) -> Result<SearchPage> {
    search_ranked(conn, q, with_facets, None)
}

/// [`search`], optionally ranked by look: only assets whose thumbnail is in `look` match (or, blended, those and the
/// text matches), best first unless another order is asked for.
pub(crate) fn search_ranked(
    conn: &Connection,
    q: &SearchQuery,
    with_facets: bool,
    look: Option<&LookRank>,
) -> Result<SearchPage> {
    let limit = i64::from(q.limit.unwrap_or(60).clamp(1, 5000));
    let offset = i64::from(q.offset.unwrap_or(0));
    if let Some(look) = look {
        conn.execute_batch(
            "CREATE TEMP TABLE IF NOT EXISTS look_rank (hash TEXT PRIMARY KEY, score REAL NOT NULL) WITHOUT ROWID;
             DELETE FROM temp.look_rank;",
        )?;
        let mut insert = conn.prepare_cached("INSERT OR REPLACE INTO temp.look_rank (hash, score) VALUES (?1, ?2)")?;
        for (hash, score) in &look.scores {
            insert.execute(params![hash, f64::from(*score)])?;
        }
    }
    let f = build_filter(q, true, look);
    let where_sql = f.where_sql();
    let base = format!(
        "FROM assets a JOIN packs p ON p.id = a.pack_id JOIN blobs b ON b.hash = a.blob_hash {} {where_sql}",
        f.join
    );

    let order = match q.sort.as_deref() {
        Some("name") => "a.name COLLATE NOCASE ASC",
        Some("size") => "b.size DESC",
        Some("old") => "a.created_at ASC, a.path ASC",
        Some("new") => "a.created_at DESC, a.path ASC",
        // A word match counts about as much as a fairly close look.
        _ if look.is_some_and(|l| l.blend) && f.ranked => {
            "(CASE WHEN f.asset_id IS NULL THEN 0.0 ELSE 0.25 END + COALESCE(lr.score, 0.0)) DESC, f.rank IS NULL, f.rank, a.id"
        }
        _ if look.is_some() => "lr.score DESC, a.id",
        _ if f.ranked => "f.rank ASC",
        _ => "a.created_at DESC, a.path ASC",
    };

    let total: i64 = conn.query_row(&format!("SELECT COUNT(*) {base}"), params_from_iter(&f.params), |r| r.get(0))?;
    let supporting_hidden: i64 = if q.hide_supporting {
        let shown = SearchQuery { hide_supporting: false, ..q.clone() };
        let fs = build_filter(&shown, true, look);
        let mut clauses = fs.clauses.clone();
        clauses.push(SUPPORTING.to_owned());
        conn.query_row(
            &format!("SELECT COUNT(*) FROM assets a {} WHERE {}", fs.join, clauses.join(" AND ")),
            params_from_iter(&fs.params),
            |r| r.get(0),
        )?
    } else {
        0
    };

    let mut page_params = f.params.clone();
    page_params.push(SqlValue::Integer(limit));
    page_params.push(SqlValue::Integer(offset));
    let mut stmt = conn.prepare(&format!("SELECT {ASSET_COLS} {base} ORDER BY {order} LIMIT ? OFFSET ?"))?;
    let mut items =
        stmt.query_map(params_from_iter(&page_params), asset_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
    attach_tags(conn, &mut items)?;

    let (mut kinds, mut tags) = (Vec::new(), Vec::new());
    if with_facets {
        // Kind facet ignores the kind filter so the sidebar can show "switch to audio (42)".
        let fk = build_filter(q, false, look);
        let mut stmt = conn.prepare(&format!(
            "SELECT a.kind, COUNT(*) FROM assets a {} {} GROUP BY a.kind ORDER BY 2 DESC",
            fk.join,
            fk.where_sql()
        ))?;
        kinds = stmt
            .query_map(params_from_iter(&fk.params), |r| {
                Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut stmt = conn.prepare(&format!(
            "SELECT t.tag, COUNT(*) FROM asset_tags t WHERE t.tag NOT LIKE 'set:%' AND t.asset_id IN
               (SELECT a.id FROM assets a {} {where_sql})
             GROUP BY t.tag ORDER BY 2 DESC, 1 ASC LIMIT 60",
            f.join
        ))?;
        tags = stmt
            .query_map(params_from_iter(&f.params), |r| {
                Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
    }

    let folders = subfolders(conn, q, look)?;
    Ok(SearchPage { items, total: total as u64, kinds, tags, supporting_hidden: supporting_hidden as u64, folders })
}

/// Tags in use and how many assets have each, most used first: those starting with `prefix`, or with `anywhere`
/// those containing it.
pub(crate) fn tag_counts(conn: &Connection, prefix: &str, anywhere: bool, limit: u32) -> Result<Vec<Facet>> {
    let lead = if anywhere { "'%' || " } else { "" };
    let mut stmt = conn.prepare(&format!(
        "SELECT tag, COUNT(*) FROM asset_tags WHERE tag LIKE {lead}?1 || '%' ESCAPE '\\'
           AND asset_id IN (SELECT a.id FROM assets a WHERE {LIVE})
         GROUP BY tag ORDER BY 2 DESC, 1 LIMIT ?2",
    ))?;
    let escaped = prefix.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_");
    let rows = stmt
        .query_map(params![escaped, i64::from(limit)], |r| {
            Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{open, test_library};

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
        assert_eq!(
            browse("/UI/Sheet/", None).0,
            vec!["a".to_owned(), "b".to_owned()],
            "slashes at the ends don't matter"
        );
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
        let bpm =
            |lo: Option<f32>, hi: Option<f32>| find(SearchQuery { bpm_min: lo, bpm_max: hi, ..Default::default() });
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
             INSERT INTO assets_fts (asset_id, name) VALUES ('a1', 'gray rock'), ('a2', 'mossy rock'), ('a3', 'rock');",
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
}
