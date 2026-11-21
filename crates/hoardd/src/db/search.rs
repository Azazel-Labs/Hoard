//! Search: the filters a query becomes, full-text and look-ranked, and folder browsing.

use std::sync::Arc;

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

/// The word matches, joined by their rows in the index (`assets.fts_row`): joined by its `asset_id` column, each
/// match's contents were read for it. (`+` keeps SQLite from going through the library matching the words again
/// for each asset, the row being something the index could look up.)
const FTS_JOIN: &str = "JOIN (SELECT rowid AS row, bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0) AS rank
          FROM assets_fts WHERE assets_fts MATCH ?) f ON a.fts_row = +f.row";
/// The word matches of a search by words and look together, worked out once into `temp.word_rank` (see
/// [`search_ranked`]): joined as a subquery, SQLite can run the match again for every asset in the library.
const WORD_RANK: &str = "INSERT INTO temp.word_rank (asset_id, rank)
     SELECT a.id, bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0)
       FROM assets_fts JOIN assets a ON a.fts_row = +assets_fts.rowid WHERE assets_fts MATCH ?1";

/// The assets a search by words and look together can find, the word matches and those the look ranks, worked out
/// into `temp.blend` so the search goes through them alone. (Joined as a subquery, SQLite can go through the whole
/// library and look each asset up in it instead.)
fn blend_candidates(look: &LookRank) -> &'static str {
    match look.keys {
        Keys::Thumbs => {
            "INSERT OR IGNORE INTO temp.blend (id) SELECT asset_id FROM temp.word_rank
               UNION SELECT m.id FROM temp.look_rank r JOIN assets m ON m.thumb_hash = r.hash"
        }
        Keys::Files => {
            "INSERT OR IGNORE INTO temp.blend (id) SELECT asset_id FROM temp.word_rank
               UNION SELECT m.id FROM temp.look_rank r JOIN assets m ON m.blob_hash = r.hash"
        }
        Keys::Mixed => {
            "INSERT OR IGNORE INTO temp.blend (id) SELECT asset_id FROM temp.word_rank
               UNION SELECT m.id FROM temp.look_rank r JOIN assets m ON m.blob_hash = r.hash WHERE m.kind = 'audio'
               UNION SELECT m.id FROM temp.look_rank r JOIN assets m ON m.thumb_hash = r.hash WHERE m.kind != 'audio'"
        }
    }
}
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
        // The word matches are in temp.word_rank already, and with the look's in temp.blend (see search_ranked).
        (Some(_), Some(look)) if look.blend => {
            f.join = format!("LEFT JOIN temp.word_rank f ON f.asset_id = a.id LEFT {}", look_join(look));
            f.clauses.push("a.id IN (SELECT id FROM temp.blend)".to_owned());
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

/// What a search counts, in one pass through what it finds: how many it finds, how many more it would without hiding
/// supporting files, how many of each kind (whatever kind is asked for, so the sidebar can offer "switch to audio
/// (42)"), and browsing a folder, the folders in it with how many matches are under each. Each of these used to be a
/// pass of its own, which in a big pack meant asking whether every file in it was a supporting one four times over.
/// The sidebar's tags take one more pass, when asked for.
fn count(conn: &Connection, q: &SearchQuery, look: Option<&LookRank>, f: &Filter, with_facets: bool) -> Result<Counts> {
    let w = wide_filter(q, look);
    // Searching words in a folder finds what's anywhere under it, so there are no folders to show.
    let mut params = Vec::new();
    let rest = match browsed_folder(q).filter(|_| fts_query_of(q).is_none()) {
        Some(folder) => {
            params.push(SqlValue::Integer(folder_skip(folder)));
            "substr(a.path, ?)"
        }
        None => "''",
    };
    let supporting = if q.hide_supporting { SUPPORTING } else { "0" };
    params.extend(w.params.iter().cloned());
    let mut stmt = conn.prepare(&format!(
        "SELECT kind, supporting, CASE WHEN instr(rest, '/') > 0 THEN substr(rest, 1, instr(rest, '/') - 1) END AS folder,
                COUNT(*)
           FROM (SELECT a.kind, {supporting} AS supporting, {rest} AS rest FROM assets a {} {})
          GROUP BY 1, 2, 3",
        w.join,
        w.where_sql()
    ))?;
    let groups = stmt
        .query_map(params_from_iter(&params), |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, bool>(1)?, r.get::<_, Option<String>>(2)?, r.get::<_, i64>(3)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    let asked = q.kind.as_deref().filter(|k| !k.is_empty());
    let (mut total, mut supporting_hidden) = (0, 0);
    let mut kinds = std::collections::HashMap::<String, i64>::new();
    let mut folders = std::collections::BTreeMap::<String, i64>::new();
    for (kind, supports, folder, n) in groups {
        let of_kind = asked.is_none_or(|k| k == kind);
        // Kinds count every kind; the rest only the kind asked for. Supporting files count only as hidden ones.
        match folder {
            None if supports => supporting_hidden += if of_kind { n } else { 0 },
            None => {
                total += if of_kind { n } else { 0 };
                *kinds.entry(kind).or_default() += n;
            }
            Some(folder) if of_kind && !supports => *folders.entry(folder).or_default() += n,
            Some(_) => {}
        }
    }
    let facet = |(value, count): (String, i64)| Facet { value, count: count as u64 };
    let mut folders = folders.into_iter().map(facet).collect::<Vec<_>>();
    folders.sort_by_cached_key(|f| f.value.to_ascii_lowercase());
    let tags = if with_facets { Some(tags(conn, f)?) } else { None };
    Ok(Counts { total, supporting_hidden, kinds: kind_facets(kinds), folders, tags })
}

/// The search without the three things [`count`] tells apart: the kind, whether a file supports another, and in a
/// folder, whether it's directly in it or in a folder of its own.
fn wide_filter(q: &SearchQuery, look: Option<&LookRank>) -> Filter {
    let wide = SearchQuery { kind: None, hide_supporting: false, folder: None, ..q.clone() };
    let mut w = build_filter(&wide, false, look);
    if let Some(folder) = browsed_folder(q).filter(|f| !f.is_empty()) {
        w.clauses.push("a.path > ? AND a.path < ?".to_owned());
        w.params.push(SqlValue::Text(format!("{folder}/")));
        w.params.push(SqlValue::Text(format!("{folder}0")));
    }
    w
}

/// How many of each kind, most first.
fn kind_facets(kinds: std::collections::HashMap<String, i64>) -> Vec<Facet> {
    let mut kinds = kinds.into_iter().map(|(value, n)| Facet { value, count: n as u64 }).collect::<Vec<_>>();
    kinds.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
    kinds
}

/// A search by words, in one pass through its matches: what it finds in order (by row, see [`assets_in_order`]),
/// and its counts but the tags. Ranking the matches goes through all of them anyway, which for the first letter typed
/// is most of the library, and counting them was a second pass as long. (Searching words finds what's anywhere under
/// a folder, so there are no folders to count.)
fn rank_and_count(conn: &Connection, q: &SearchQuery, order: &str, blobs: &str) -> Result<(Vec<i64>, Counts)> {
    let w = wide_filter(q, None);
    let supporting = if q.hide_supporting { SUPPORTING } else { "0" };
    let mut stmt = conn.prepare(&format!(
        "SELECT a.rowid, a.kind, {supporting} FROM assets a {blobs} {} {} ORDER BY {order}",
        w.join,
        w.where_sql()
    ))?;
    let mut rows = stmt.query(params_from_iter(&w.params))?;
    let asked = q.kind.as_deref().filter(|k| !k.is_empty());
    let (mut found, mut supporting_hidden) = (Vec::new(), 0);
    let mut kinds = std::collections::HashMap::<String, i64>::new();
    while let Some(r) = rows.next()? {
        let kind = r.get_ref(1)?.as_str()?;
        let of_kind = asked.is_none_or(|k| k == kind);
        // Kinds count every kind; the rest only the kind asked for. Supporting files count only as hidden ones.
        if r.get::<_, bool>(2)? {
            supporting_hidden += i64::from(of_kind);
            continue;
        }
        if of_kind {
            found.push(r.get::<_, i64>(0)?);
        }
        match kinds.get_mut(kind) {
            Some(n) => *n += 1,
            None => {
                kinds.insert(kind.to_owned(), 1);
            }
        }
    }
    let total = found.len() as i64;
    Ok((found, Counts { total, supporting_hidden, kinds: kind_facets(kinds), folders: Vec::new(), tags: None }))
}

/// In the library rather than the archive: neither the asset nor its pack is archived. Expects the asset as `a`.
pub(crate) const LIVE: &str =
    "a.archived_at IS NULL AND a.pack_id NOT IN (SELECT id FROM packs WHERE archived_at IS NOT NULL)";

/// An asset another asset needs as raw data (a glTF's .bin, an OBJ's .mtl), the store's file describing a model
/// (a Megascans `<id>.json`) or a map of a material (its roughness
/// map, say): not something you'd browse for. The material is, and it shows its maps, unless it's the material of
/// one object only (`owned_by`, see `materials::mark_owned`): then it's part of that model, which shows it.
/// A sprite sheet's pages, and the loose images it has its sprites as too, are shown with the sheet. A file of clips
/// whose takes are animations of their own (see `takes`) is shown by them. (`+a.id` drops the column's text affinity,
/// so the lookup can use the `take_of` index instead of reading all of it for every animation.)
///
/// What the links and material maps say is kept in `assets.supports` (migration 82, whose triggers keep it up to
/// date): asked of every asset each time, it was most of a search's work. A new kind of link that makes a supporting
/// file needs a migration that works it out afresh, and triggers that know it.
pub(crate) const SUPPORTING: &str = "(a.supports
     OR (a.ext = 'material' AND EXISTS (SELECT 1 FROM assets o WHERE o.id = json_extract(a.meta, '$.owned_by')))
     OR (a.kind = 'animation' AND EXISTS (SELECT 1 FROM assets t WHERE json_extract(t.meta, '$.take_of') = +a.id)))";

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

/// The order a `sort` asks for, or `None` for best match first: `created`, `updated`, `name` or `size`, each in its
/// usual direction (newest, A to Z, biggest first) or with `:asc` or `:desc` after it. `new` and `old`, which links
/// and saved searches made before may hold, are `created` and `created:asc`.
fn chosen_order(sort: Option<&str>) -> Option<&'static str> {
    let (by, dir) = match sort? {
        "new" => ("created", None),
        "old" => ("created", Some("asc")),
        s => match s.split_once(':') {
            Some((by, dir)) => (by, Some(dir)),
            None => (s, None),
        },
    };
    let up = match dir {
        None => by == "name",
        Some("asc") => true,
        Some("desc") => false,
        Some(_) => return None,
    };
    // Each is read in order from an index, either way (assets_created, assets_updated, assets_name).
    Some(match (by, up) {
        ("created", false) => "a.created_at DESC, a.path ASC",
        ("created", true) => "a.created_at ASC, a.path DESC",
        ("updated", false) => "a.updated_at DESC, a.path ASC",
        ("updated", true) => "a.updated_at ASC, a.path DESC",
        ("name", true) => "a.name COLLATE NOCASE ASC",
        ("name", false) => "a.name COLLATE NOCASE DESC",
        ("size", false) => "b.size DESC",
        ("size", true) => "b.size ASC",
        _ => return None,
    })
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
        if look.blend
            && let Some(fts) = fts_query_of(q)
        {
            conn.execute_batch(
                "CREATE TEMP TABLE IF NOT EXISTS word_rank (asset_id TEXT PRIMARY KEY, rank REAL NOT NULL) WITHOUT ROWID;
                 DELETE FROM temp.word_rank;",
            )?;
            conn.prepare_cached(WORD_RANK)?.execute([fts])?;
            conn.execute_batch(
                "CREATE TEMP TABLE IF NOT EXISTS blend (id TEXT PRIMARY KEY) WITHOUT ROWID;
                 DELETE FROM temp.blend;",
            )?;
            conn.prepare_cached(blend_candidates(look))?.execute([])?;
        }
    }
    let f = build_filter(q, true, look);
    let where_sql = f.where_sql();
    // Only the sizes are needed to put the page in order: the rest of each asset is read once it's on the page, not for
    // everything the search finds.
    let chosen = chosen_order(q.sort.as_deref());
    let blobs =
        if chosen.is_some_and(|o| o.starts_with("b.size")) { "JOIN blobs b ON b.hash = a.blob_hash" } else { "" };

    let order = match chosen {
        Some(o) => o,
        // A word match counts about as much as a fairly close look.
        _ if look.is_some_and(|l| l.blend) && f.ranked => {
            "(CASE WHEN f.asset_id IS NULL THEN 0.0 ELSE 0.25 END + COALESCE(lr.score, 0.0)) DESC, f.rank IS NULL, f.rank, a.id"
        }
        _ if look.is_some() => "lr.score DESC, a.id",
        _ if f.ranked => "f.rank ASC",
        _ => "a.created_at DESC, a.path ASC",
    };

    // Kept for the next pages of the same search while the library stays the same (a ranking by look is worked out
    // afresh each time, so its counts aren't). Read before counting, so a write while counting makes them stale.
    let writes = super::writes();
    let key = if look.is_none() { counts_key(conn, q) } else { None };
    // A search by words goes through all it finds to rank them, so what it finds in order is kept too, for its next
    // pages, and counted on the way.
    let words = look.is_none() && f.ranked;
    let order_key = key.as_ref().filter(|_| words).map(|k| format!("{k}\n{}", q.sort.as_deref().unwrap_or_default()));
    let mut counted = None;
    let ranked = match order_key.as_deref().and_then(|k| kept_ranked(k, writes)) {
        Some(ranked) => Some(ranked),
        None if words => {
            let (found, counts) = rank_and_count(conn, q, order, blobs)?;
            counted = Some(counts);
            let found = Arc::new(found);
            if let Some(k) = order_key {
                keep_ranked(k, writes, &found);
            }
            Some(found)
        }
        None => None,
    };
    // A search by words has what it finds already: its tags are counted from those, not by searching again.
    let sidebar_tags = || match &ranked {
        Some(found) => tags_of_rows(conn, found),
        None => tags(conn, &f),
    };
    let counts = match (key.as_deref().and_then(|k| kept_counts(k, writes)), counted) {
        // The next page of a search, or the same search again, while the library is the same.
        (Some(counts), _) if counts.tags.is_some() || !with_facets => counts,
        // The sidebar asked for after the first page came without it: only the tags are left to count.
        (Some(counts), _) => Counts { tags: Some(sidebar_tags()?), ..counts },
        (None, Some(counts)) if with_facets => Counts { tags: Some(sidebar_tags()?), ..counts },
        (None, Some(counts)) => counts,
        (None, None) => count(conn, q, look, &f, with_facets)?,
    };
    if let Some(key) = key {
        keep_counts(key, writes, &counts);
    }
    let rows = if let Some(ranked) = ranked {
        let from = usize::try_from(offset).unwrap_or(usize::MAX).min(ranked.len());
        ranked[from..].iter().take(usize::try_from(limit).unwrap_or(usize::MAX)).copied().collect()
    } else {
        let mut page_params = f.params.clone();
        page_params.push(SqlValue::Integer(limit));
        page_params.push(SqlValue::Integer(offset));
        conn.prepare(&format!(
            "SELECT a.rowid FROM assets a {blobs} {} {where_sql} ORDER BY {order} LIMIT ? OFFSET ?",
            f.join
        ))?
        .query_map(params_from_iter(&page_params), |r| r.get::<_, i64>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?
    };
    let mut items = assets_in_order(conn, &rows)?;
    attach_tags(conn, &mut items)?;

    let Counts { total, supporting_hidden, kinds, folders, tags } = counts;
    // The kinds are counted anyway; the tags take a pass of their own.
    let tags = if with_facets { tags.unwrap_or_default() } else { Vec::new() };
    Ok(SearchPage { items, total: total as u64, kinds, tags, supporting_hidden: supporting_hidden as u64, folders })
}

/// The assets in these rows, in their order. (A search keeps what it finds by row, a number, rather than by id: a
/// few bytes each for the many thousands the first letter typed finds.)
fn assets_in_order(conn: &Connection, rows: &[i64]) -> Result<Vec<Asset>> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let mut items = conn
        .prepare_cached(&format!(
            "SELECT {ASSET_COLS}, a.rowid FROM assets a JOIN packs p ON p.id = a.pack_id JOIN blobs b ON b.hash = a.blob_hash
              WHERE a.rowid IN (SELECT value FROM json_each(?1))"
        ))?
        .query_map([serde_json::to_string(rows)?], |r| Ok((r.get::<_, i64>(r.as_ref().column_count() - 1)?, asset_from_row(r)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let place: std::collections::HashMap<i64, usize> = rows.iter().enumerate().map(|(i, row)| (*row, i)).collect();
    items.sort_by_key(|(row, _)| place.get(row).copied().unwrap_or(usize::MAX));
    Ok(items.into_iter().map(|(_, asset)| asset).collect())
}

/// How many assets a search finds and how many more hiding supporting files left out, how many of each kind, the
/// folders in the one browsed, and once the sidebar has asked, the tags. See [`count`].
#[derive(Clone)]
struct Counts {
    total: i64,
    supporting_hidden: i64,
    kinds: Vec<Facet>,
    folders: Vec<Facet>,
    tags: Option<Vec<Facet>>,
}

/// The sidebar's tags: the most common among what the search finds.
fn tags(conn: &Connection, f: &Filter) -> Result<Vec<Facet>> {
    Ok(conn
        .prepare(&format!(
            "SELECT t.tag, COUNT(*) FROM asset_tags t WHERE t.tag NOT LIKE 'set:%' AND t.asset_id IN
               (SELECT a.id FROM assets a {} {})
             GROUP BY t.tag ORDER BY 2 DESC, 1 ASC LIMIT 60",
            f.join,
            f.where_sql()
        ))?
        .query_map(params_from_iter(&f.params), |r| Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 }))?
        .collect::<rusqlite::Result<Vec<_>>>()?)
}

/// [`tags`] of what a search by words found (see [`rank_and_count`]), by row: what it finds is already worked out,
/// where [`tags`] matches the words and goes through every filter again for each asset.
fn tags_of_rows(conn: &Connection, rows: &[i64]) -> Result<Vec<Facet>> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    // CROSS JOIN keeps the order: from the rows found to their assets to their tags, never through every tag.
    Ok(conn
        .prepare_cached(
            "SELECT t.tag, COUNT(*) FROM json_each(?1) j CROSS JOIN assets a ON a.rowid = j.value
               CROSS JOIN asset_tags t ON t.asset_id = a.id
              WHERE t.tag NOT LIKE 'set:%'
              GROUP BY t.tag ORDER BY 2 DESC, 1 ASC LIMIT 60",
        )?
        .query_map([serde_json::to_string(rows)?], |r| {
            Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?)
}

/// How many recent searches' counts are kept.
const COUNTS_KEPT: usize = 16;

/// What a search's counts depend on: the library's file and the search, but not which page or in what order.
fn counts_key(conn: &Connection, q: &SearchQuery) -> Option<String> {
    // An in-memory library (a test's) has no [`super::WRITES`] kept for it.
    let path = conn.path().filter(|p| !p.is_empty())?;
    let q = SearchQuery { limit: None, offset: None, sort: None, ..q.clone() };
    Some(format!(
        "{path}
{}",
        serde_json::to_string(&q).ok()?
    ))
}

/// Recent searches' counts, newest last, with how many writes the library had had when they were counted. Counting
/// goes through everything the search finds, which in a big library takes as long as the rest of the search
/// together, while the next pages of the same search, and going back to one, are asked for far more often than the
/// library changes.
static COUNTS: std::sync::Mutex<Vec<(String, u64, Counts)>> = std::sync::Mutex::new(Vec::new());

fn kept_counts(key: &str, writes: u64) -> Option<Counts> {
    let kept = COUNTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    kept.iter().find(|(k, at, _)| *k == key && *at == writes).map(|(_, _, c)| c.clone())
}

/// Keeps `counts` as of `writes`, read before they were worked out: a write since makes them stale at once. Kept
/// again, the search is the newest kept, the last to go.
fn keep_counts(key: String, writes: u64, counts: &Counts) {
    let mut kept = COUNTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // Those counted before a write are stale for good.
    kept.retain(|(k, at, _)| *k != key && *at == writes);
    if kept.len() >= COUNTS_KEPT {
        kept.remove(0);
    }
    kept.push((key, writes, counts.clone()));
}

/// How many recent searches by words keep what they found in order.
const RANKED_KEPT: usize = 8;

/// One kept search: its key, the writes it was made at, and what it found.
type Ranked = (String, u64, Arc<Vec<i64>>);

/// Recent searches by words, what they found in order (see [`rank_and_count`]) under their [`counts_key`] and order,
/// with how many writes the library had had when they searched: newest last.
static RANKED: std::sync::Mutex<Vec<Ranked>> = std::sync::Mutex::new(Vec::new());

fn kept_ranked(key: &str, writes: u64) -> Option<Arc<Vec<i64>>> {
    let kept = RANKED.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    kept.iter().find(|(k, at, _)| *k == key && *at == writes).map(|(_, _, found)| Arc::clone(found))
}

/// Keeps what a search found as of `writes`, read before it searched, as [`keep_counts`] does its counts.
fn keep_ranked(key: String, writes: u64, found: &Arc<Vec<i64>>) {
    let mut kept = RANKED.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    kept.retain(|(k, at, _)| *k != key && *at == writes);
    if kept.len() >= RANKED_KEPT {
        kept.remove(0);
    }
    kept.push((key, writes, Arc::clone(found)));
}

/// Tags in use and how many assets have each, most used first: those starting with `prefix`, or with `anywhere`
/// those containing it.
pub(crate) fn tag_counts(conn: &Connection, prefix: &str, anywhere: bool, limit: u32) -> Result<Vec<Facet>> {
    // Read before counting, so a write while counting leaves them stale.
    let writes = super::writes();
    // An in-memory library (a test's) has no [`super::WRITES`] kept for it.
    let path = conn.path().filter(|p| !p.is_empty());
    let kept = path.and_then(|path| {
        let kept = TAG_COUNTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        kept.as_ref().filter(|(p, at, _)| p == path && *at == writes).map(|(_, _, all)| Arc::clone(all))
    });
    let all = if let Some(all) = kept {
        all
    } else {
        let all = Arc::new(count_tags(conn)?);
        if let Some(path) = path {
            *TAG_COUNTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
                Some((path.to_owned(), writes, Arc::clone(&all)));
        }
        all
    };
    let prefix = prefix.to_lowercase();
    Ok(all
        .iter()
        .filter(|t| {
            let tag = t.value.to_lowercase();
            if anywhere { tag.contains(&prefix) } else { tag.starts_with(&prefix) }
        })
        .take(limit as usize)
        .cloned()
        .collect())
}

/// Every tag on an asset in the library (not the archive) and how many have it, most used first. All are counted along
/// the tags' index, without going to the assets, and the archive's taken off after: it holds far fewer.
fn count_tags(conn: &Connection) -> Result<Vec<Facet>> {
    let mut counts: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    let all = "SELECT tag, COUNT(*) FROM asset_tags GROUP BY tag";
    let archived = "SELECT tag, COUNT(*) FROM asset_tags WHERE asset_id IN (
                      SELECT id FROM assets WHERE archived_at IS NOT NULL
                      UNION ALL
                      SELECT a.id FROM packs p JOIN assets a ON a.pack_id = p.id
                       WHERE p.archived_at IS NOT NULL AND a.archived_at IS NULL)
                    GROUP BY tag";
    for (sql, sign) in [(all, 1), (archived, -1)] {
        let mut stmt = conn.prepare_cached(sql)?;
        let mut rows = stmt.query([])?;
        while let Some(r) = rows.next()? {
            *counts.entry(r.get(0)?).or_default() += sign * r.get::<_, i64>(1)?;
        }
    }
    let mut tags: Vec<Facet> =
        counts.into_iter().filter(|(_, n)| *n > 0).map(|(value, n)| Facet { value, count: n as u64 }).collect();
    tags.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
    Ok(tags)
}

/// Every tag in use and how many assets have each, most used first, with the library's path and how many writes it
/// had had when they were counted: suggesting tags as a name is typed went through all of every asset's tags for each
/// letter, while there are only a few hundred tags.
static TAG_COUNTS: std::sync::Mutex<Option<KeptTags>> = std::sync::Mutex::new(None);

/// The library's path, its write count and [`TAG_COUNTS`]' tags.
type KeptTags = (String, u64, Arc<Vec<Facet>>);

#[cfg(test)]
mod tests {
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
               ('c', 'p1', 's', 'c.png', 'Cliff', 'png', 'texture', '2026-03-01', '2026-03-02');",
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
    fn a_search_by_words_counts_the_tags_of_what_it_found() {
        let conn = test_library();
        conn.execute_batch(
            "INSERT INTO blobs (hash, size, created_at) VALUES ('h', 1, 'x');
             INSERT INTO packs (id, name, status, created_at) VALUES ('p1', 'Kit', 'ready', 'x');",
        )
        .unwrap();
        for (id, name, kind, tags) in [
            ("r1", "rock one", "texture", "mossy,grey,set:a"),
            ("r2", "rock two", "model", "grey,set:a"),
            ("r3", "rock three", "texture", "grey,dark"),
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
            assert_eq!(found, listed(&tags(&conn, &build_filter(q, true, None)).unwrap()), "{q:?}");
            found
        };
        assert_eq!(shown(&rock), ["grey 2", "mossy 2"]);
        assert_eq!(shown(&SearchQuery { kind: Some("texture".into()), ..rock.clone() }), ["mossy 2", "grey 1"]);
        assert_eq!(shown(&SearchQuery { hide_supporting: false, ..rock.clone() }), ["grey 3", "mossy 2", "dark 1"]);
        assert_eq!(shown(&SearchQuery { q: Some("nothing".into()), ..rock }), Vec::<String>::new());
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
        println!("made {n} assets in {:.1?}", started.elapsed());
        let pictures: Vec<(String, f32)> = (0..300).map(|i| (format!("t{}", i * 997), 0.4)).collect();
        let look = LookRank { scores: pictures, exclude: None, blend: true, keys: Keys::Thumbs };
        let time = |label: &str, q: SearchQuery, facets: bool, look: Option<&LookRank>| {
            let started = std::time::Instant::now();
            let page = search_ranked(&conn, &q, facets, look).unwrap();
            println!("{label}: {} of {} in {:.1?}", page.items.len(), page.total, started.elapsed());
        };
        let alone = |label: &str, q: SearchQuery| {
            let f = build_filter(&q, true, None);
            let started = std::time::Instant::now();
            let counts = count(&conn, &q, None, &f, false).unwrap();
            let counted = started.elapsed();
            let started = std::time::Instant::now();
            let tags = tags(&conn, &f).unwrap();
            println!(
                "{label}: counted {} in {counted:.1?}, {} tags in {:.1?}",
                counts.total,
                tags.len(),
                started.elapsed()
            );
        };
        // The page alone, put in order without counting: what's left of a search once its counts are kept.
        let page_alone = |label: &str, q: SearchQuery| {
            let f = build_filter(&q, true, None);
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
        alone("everything, alone", SearchQuery { hide_supporting: true, ..Default::default() });
        alone(
            "the first letter, alone",
            SearchQuery { q: Some("t".into()), hide_supporting: true, ..Default::default() },
        );
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
        time(
            "words and pictures, textures",
            SearchQuery { kind: Some("texture".into()), ..words() },
            true,
            Some(&look),
        );
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
    }
}
