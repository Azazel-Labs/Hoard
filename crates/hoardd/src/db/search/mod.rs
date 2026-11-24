//! Search: the filters a query becomes, full-text and look-ranked, and folder browsing.

use std::sync::Arc;

use anyhow::Result;
use hoard_core::{Asset, Facet, SearchQuery};
use rusqlite::types::Value as SqlValue;
use rusqlite::{Connection, params, params_from_iter};

use super::{ASSET_COLS, Tree, asset_from_row, attach_tags};

mod facts;
mod tag_rows;
#[cfg(test)]
mod tests;

use facts::{facts, facts_search, search_facts};
use tag_rows::{tags, tags_of_rows};

/// Turn free text into a safe FTS5 query: every term becomes a quoted prefix match, and all must match.
fn fts_query(q: &str) -> Option<String> {
    words(q).find
}

/// What's typed in the search box, for FTS5: what to find, and what to leave out.
#[derive(Debug, Default, PartialEq)]
struct Words {
    /// Every word, each as the start of a word (`rock` finds `rocks`); `a or b` either; `"a b"` those words together
    /// as they are. A word with dashes or colons in it (`sci-fi`, `set:forest`) is its parts together. `pack:forest`
    /// is a word of the pack's name.
    ///
    /// Words are found in an asset's pack's name and tags too, as long as one of them is the asset's own: `robot
    /// forest` finds a robot in a forest pack, but `forest` alone doesn't find everything in that pack.
    find: Option<String>,
    /// Anything with any of the words after a `-` (`-moss`, `-"dry moss"`, `-pack:"Sci-Fi Kit"`), its pack's
    /// included: `-sci-fi` leaves out everything in a pack called "Sci-Fi Kit".
    without: Option<String>,
    /// What to find without the marks: the words for image and sound search to look for.
    plain: String,
    /// Every word to find is the start of a word under three letters long: what's typed first. See
    /// [`FTS_JOIN_AS_FOUND`].
    short: bool,
}

/// The columns of the search index that are an asset's own, rather than its pack's.
const OWN_COLUMNS: &str = "{name words category description tags}";

fn words(q: &str) -> Words {
    words_in(q, &Tree::default())
}

/// [`words`], where a word that is a tag finds what has any tag under it too, and leaves it out after a `-`: with sedan
/// under vehicle, `vehicle` finds the sedans and `-vehicle` leaves them out. (The tag tree has tags whole, so those
/// under it are found as they are, not as the start of a word.)
fn words_in(q: &str, tree: &Tree) -> Words {
    /// One word or phrase, as FTS5 has it.
    fn term(text: &str, exact: bool) -> Option<String> {
        let parts: Vec<String> =
            text.split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()).map(str::to_lowercase).collect();
        (!parts.is_empty()).then(|| format!("\"{}\"{}", parts.join(" "), if exact { "" } else { "*" }))
    }
    let mut find: Vec<Vec<String>> = Vec::new();
    let (mut own, mut terms) = (Vec::new(), 0);
    let mut without = Vec::new();
    let mut plain = Vec::new();
    let (mut either, mut chars) = (false, q.chars().peekable());
    let mut short = true;
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '|' {
            either |= c == '|';
            chars.next();
            continue;
        }
        let not = c == '-' && chars.clone().nth(1).is_some_and(|n| !n.is_whitespace());
        if not {
            chars.next();
        }
        let pack = chars.clone().take(5).collect::<String>().eq_ignore_ascii_case("pack:");
        if pack {
            chars.nth(4);
        }
        let exact = chars.peek() == Some(&'"');
        let text: String = if exact {
            chars.next();
            chars.by_ref().take_while(|&c| c != '"').collect()
        } else {
            let mut w = String::new();
            while let Some(&c) = chars.peek().filter(|c| !c.is_whitespace() && **c != '"' && **c != '|') {
                w.push(c);
                chars.next();
            }
            w
        };
        if !not && !exact && !pack && text.eq_ignore_ascii_case("or") {
            either = true;
            continue;
        }
        let Some(t) = term(&text, exact) else { continue };
        let under = if pack { &[][..] } else { tree.narrower(&text.trim().to_lowercase()) };
        let t = if pack {
            format!("pack_name : {t}")
        } else if under.is_empty() {
            t
        } else {
            let under = under.iter().filter_map(|n| term(n, true)).map(|n| format!("tags : {n}"));
            format!("({})", std::iter::once(t).chain(under).collect::<Vec<_>>().join(" OR "))
        };
        if not {
            without.push(t);
        } else {
            short &= !exact && !pack && under.is_empty() && text.chars().filter(|c| c.is_alphanumeric()).count() < 3;
            if !pack {
                own.push(t.clone());
                plain.push(text.trim().to_owned());
            }
            terms += 1;
            match find.last_mut() {
                Some(any) if either => any.push(t),
                _ => find.push(vec![t]),
            }
        }
        either = false;
    }
    let mut find = find
        .into_iter()
        .map(|any| if any.len() == 1 { any.concat() } else { format!("({})", any.join(" OR ")) })
        .collect::<Vec<_>>();
    if !own.is_empty() {
        if find.len() == 1 && own.len() == terms {
            // One word, or either of a few: the asset's own, or nothing.
            find[0] = format!("{OWN_COLUMNS} : {}", find[0]);
        } else {
            find.push(format!("{OWN_COLUMNS} : ({})", own.join(" OR ")));
        }
    }
    Words {
        find: (!find.is_empty()).then(|| find.join(" AND ")),
        without: (!without.is_empty()).then(|| without.join(" OR ")),
        plain: plain.join(" "),
        short: short && terms > 0,
    }
}

/// The words of a search for image and sound search to look for, without its marks or what it leaves out.
pub(crate) fn plain_words(q: &str) -> String {
    words(q).plain
}

struct Filter {
    join: String,
    clauses: Vec<String>,
    params: Vec<SqlValue>,
    /// Searching by words: found through the search index, each with its rank.
    ranked: bool,
    /// The words are what's typed first, so what they find goes newest first, unranked (see [`FTS_JOIN_AS_FOUND`]).
    as_found: bool,
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
const FTS_JOIN: &str = "JOIN (SELECT rowid AS row, bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0, 1.5, 1.0) AS rank
          FROM assets_fts WHERE assets_fts MATCH ?) f ON a.fts_row = +f.row";
/// [`FTS_JOIN`] unranked, for the first letter or two typed: they find most of a big library (279,000 of 300,000 for
/// one letter), which ranking took most of a second to put in an order that says little, a word starting with
/// them being about as good a match as any other. What they find goes newest first, as browsing does.
const FTS_JOIN_AS_FOUND: &str = "JOIN (SELECT rowid AS row, 0.0 AS rank FROM assets_fts WHERE assets_fts MATCH ?) f
          ON a.fts_row = +f.row";
/// The word matches of a search by words and look together, worked out once into `temp.word_rank` (see
/// [`search_ranked`]): joined as a subquery, SQLite can run the match again for every asset in the library.
const WORD_RANK: &str = "INSERT INTO temp.word_rank (asset_id, rank)
     SELECT a.id, bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0, 1.5, 1.0)
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

fn build_filter(q: &SearchQuery, include_kind: bool, look: Option<&LookRank>, tree: &Tree) -> Filter {
    let mut f = Filter { join: String::new(), clauses: Vec::new(), params: Vec::new(), ranked: false, as_found: false };
    let Words { find: fts, without, short, .. } = q.q.as_deref().map_or_else(Words::default, |t| words_in(t, tree));
    match (fts, look) {
        // The word matches are in temp.word_rank already, and with the look's in temp.blend (see search_ranked).
        (Some(_), Some(look)) if look.blend => {
            f.join = format!("LEFT JOIN temp.word_rank f ON f.asset_id = a.id LEFT {}", look_join(look));
            f.clauses.push("a.id IN (SELECT id FROM temp.blend)".to_owned());
            f.ranked = true;
        }
        (fts, look) => {
            if let Some(fts) = fts {
                f.as_found = short && look.is_none();
                (if f.as_found { FTS_JOIN_AS_FOUND } else { FTS_JOIN }).clone_into(&mut f.join);
                f.params.push(SqlValue::Text(fts));
                f.ranked = true;
            }
            if let Some(look) = look {
                f.join = format!("{} {}", f.join, look_join(look));
            }
        }
    }
    // Words after a `-`: nothing that has them, whether it matched the words or the look.
    if let Some(without) = without {
        f.clauses
            .push("IFNULL(a.fts_row, 0) NOT IN (SELECT rowid FROM assets_fts WHERE assets_fts MATCH ?)".to_owned());
        f.params.push(SqlValue::Text(without));
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
    // A tag finds what has it or any narrower tag under it.
    // (From the tree in hand: worked out in SQL, the tags under it were worked out again for every asset. What has
    // them is read from the tags' index once, not looked up for every asset tag by tag; `+` keeps SQLite going through
    // the library in order and checking each asset against them, rather than putting all of them in order for a page.)
    for tag in &q.tags {
        let under = tree.narrower(tag);
        f.clauses.push(format!(
            "+a.id IN (SELECT t.asset_id FROM asset_tags t WHERE t.tag IN ({}))",
            vec!["?"; 1 + under.len()].join(", ")
        ));
        f.params.extend(std::iter::once(tag).chain(under).map(|t| SqlValue::Text(t.clone())));
    }
    if q.hide_supporting {
        f.clauses.push(format!("NOT {SUPPORTING}"));
    }
    if let Some(reviewed) = q.reviewed {
        f.clauses.push(if reviewed { "a.reviewed_at IS NOT NULL" } else { "a.reviewed_at IS NULL" }.to_owned());
    }
    if let Some(stars) = q.rating_min.filter(|n| *n > 0) {
        f.clauses.push("a.rating >= ?".to_owned());
        f.params.push(SqlValue::Integer(i64::from(stars.min(5))));
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
fn count(
    conn: &Connection,
    q: &SearchQuery,
    look: Option<&LookRank>,
    with_facets: bool,
    tree: &Tree,
) -> Result<Counts> {
    let w = wide_filter(q, look, tree);
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
    // Tallied here as they come rather than grouped in SQL, which put every asset found in order first: most of the
    // time counting a quarter of a million took. Those in the total are what the search finds, whose tags the sidebar
    // counts, so their rows are kept for that.
    let mut stmt = conn.prepare(&format!(
        "SELECT a.kind, {supporting}, {rest}, a.rowid FROM assets a {} {}",
        w.join,
        w.where_sql()
    ))?;
    let mut rows = stmt.query(params_from_iter(&params))?;
    let asked = q.kind.as_deref().filter(|k| !k.is_empty());
    let (mut total, mut supporting_hidden) = (0, 0);
    let mut kinds = std::collections::HashMap::<String, i64>::new();
    let mut folders = std::collections::BTreeMap::<String, i64>::new();
    let mut found = Vec::new();
    while let Some(r) = rows.next()? {
        let kind = r.get_ref(0)?.as_str()?;
        let of_kind = asked.is_none_or(|k| k == kind);
        let supports = r.get::<_, bool>(1)?;
        let rest = r.get_ref(2)?.as_str()?;
        // Kinds count every kind; the rest only the kind asked for. Supporting files count only as hidden ones.
        match rest.split_once('/') {
            None if supports => supporting_hidden += i64::from(of_kind),
            None => {
                if of_kind {
                    total += 1;
                    if with_facets {
                        found.push(r.get::<_, i64>(3)?);
                    }
                }
                match kinds.get_mut(kind) {
                    Some(n) => *n += 1,
                    None => {
                        kinds.insert(kind.to_owned(), 1);
                    }
                }
            }
            Some((folder, _)) if of_kind && !supports => match folders.get_mut(folder) {
                Some(n) => *n += 1,
                None => {
                    folders.insert(folder.to_owned(), 1);
                }
            },
            Some(_) => {}
        }
    }
    let facet = |(value, count): (String, i64)| Facet { value, count: count as u64 };
    let mut folders = folders.into_iter().map(facet).collect::<Vec<_>>();
    folders.sort_by_cached_key(|f| f.value.to_ascii_lowercase());
    let tags = if with_facets { Some(tags_of_rows(conn, &found, tree)?) } else { None };
    Ok(Counts { total, supporting_hidden, kinds: kind_facets(kinds), folders, tags })
}

/// The search without the three things [`count`] tells apart: the kind, whether a file supports another, and in a
/// folder, whether it's directly in it or in a folder of its own.
fn wide_filter(q: &SearchQuery, look: Option<&LookRank>, tree: &Tree) -> Filter {
    let wide = SearchQuery { kind: None, hide_supporting: false, folder: None, ..q.clone() };
    let mut w = build_filter(&wide, false, look, tree);
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
fn rank_and_count(
    conn: &Connection,
    q: &SearchQuery,
    order: &str,
    blobs: &str,
    tree: &Tree,
) -> Result<(Vec<i64>, Counts)> {
    let w = wide_filter(q, None, tree);
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

/// The order a `sort` asks for, or `None` for best match first: `created`, `updated`, `name`, `size` or `rating`, each
/// in its usual direction (newest, A to Z, biggest and best rated first) or with `:asc` or `:desc` after it. What
/// isn't rated comes last either way. `new` and `old`, which links
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
        ("rating", false) => "a.rating IS NULL, a.rating DESC, a.created_at DESC",
        ("rating", true) => "a.rating IS NULL, a.rating ASC, a.created_at DESC",
        _ => return None,
    })
}

/// Every asset a search finds, in no particular order: for doing something to all of them. Not for searches by look
/// (`similar`, `color`, `imitation`), whose matches are worked out elsewhere.
pub(crate) fn matching_ids(conn: &Connection, q: &SearchQuery) -> Result<Vec<String>> {
    let f = build_filter(q, true, None, &Tree::load(conn)?);
    let mut stmt = conn.prepare(&format!("SELECT a.id FROM assets a {} {}", f.join, f.where_sql()))?;
    let ids = stmt.query_map(params_from_iter(&f.params), |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    Ok(ids)
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
    let tree = Tree::load(conn)?;
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
            && let Some(fts) = q.q.as_deref().and_then(|t| words_in(t, &tree).find)
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
    let f = build_filter(q, true, look, &tree);
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
        _ if f.ranked && !f.as_found => "f.rank ASC",
        _ => "a.created_at DESC, a.path ASC",
    };

    // Kept for the next pages of the same search while the library stays the same (a ranking by look is worked out
    // afresh each time, so its counts aren't). Read before counting, so a write while counting makes them stale.
    let writes = super::library_writes();
    let key = if look.is_none() { counts_key(conn, q) } else { None };
    // A search by words goes through all it finds to rank them, so what it finds in order is kept too, for its next
    // pages, and counted on the way.
    let words = look.is_none() && f.ranked;
    // So is what a search finds when it asks only for what's kept of every asset (see [`Facts`]): found and counted
    // in memory, in one pass.
    let quick = if look.is_none() { facts_search(q, &tree) } else { None };
    let in_order = words || quick.is_some();
    let order_key =
        key.as_ref().filter(|_| in_order).map(|k| format!("{k}\n{}", q.sort.as_deref().unwrap_or_default()));
    let mut counted = None;
    let ranked = if let Some(ranked) = order_key.as_deref().and_then(|k| kept_ranked(k, writes)) {
        Some(ranked)
    } else {
        let quick = match &quick {
            Some(s) => facts(conn)?.map(|facts| search_facts(conn, &facts, s, &tree)).transpose()?,
            None => None,
        };
        let found = match quick {
            Some(found) => Some(found),
            None if words => Some(rank_and_count(conn, q, order, blobs, &tree)?),
            None => None,
        };
        found.map(|(found, counts)| {
            counted = Some(counts);
            let found = Arc::new(found);
            if let Some(k) = order_key {
                keep_ranked(k, writes, &found);
            }
            found
        })
    };
    // A search by words has what it finds already: its tags are counted from those, not by searching again.
    let sidebar_tags = || match &ranked {
        Some(found) => tags_of_rows(conn, found, &tree),
        None => tags(conn, &f, &tree),
    };
    let counts = match (key.as_deref().and_then(|k| kept_counts(k, writes)), counted) {
        // The next page of a search, or the same search again, while the library is the same.
        (Some(counts), _) if counts.tags.is_some() || !with_facets => counts,
        // The sidebar asked for after the first page came without it: only the tags are left to count.
        (Some(counts), _) => Counts { tags: Some(sidebar_tags()?), ..counts },
        (None, Some(counts)) if with_facets => Counts { tags: Some(sidebar_tags()?), ..counts },
        (None, Some(counts)) => counts,
        (None, None) => count(conn, q, look, with_facets, &tree)?,
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

/// How many recent searches' counts are kept.
const COUNTS_KEPT: usize = 16;

/// What a search's counts depend on: the library's file and the search, but not which page or in what order.
fn counts_key(conn: &Connection, q: &SearchQuery) -> Option<String> {
    // An in-memory library (a test's) has no writes counted for it.
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

/// The categories assets in the library have, the most used first.
pub(crate) fn category_counts(conn: &Connection, limit: u32) -> Result<Vec<Facet>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT a.category, COUNT(*) FROM assets a WHERE a.category IS NOT NULL AND a.category != '' AND {LIVE}
          GROUP BY a.category ORDER BY 2 DESC, 1 LIMIT ?1"
    ))?;
    let rows = stmt.query_map([limit], |r| Ok(Facet { value: r.get(0)?, count: r.get::<_, i64>(1)? as u64 }))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Tags in use and how many assets have each, most used first: those starting with `prefix`, or with `anywhere`
/// those containing it.
pub(crate) fn tag_counts(conn: &Connection, prefix: &str, anywhere: bool, limit: u32) -> Result<Vec<Facet>> {
    // Read before counting, so a write while counting leaves them stale.
    let writes = super::library_writes();
    // An in-memory library (a test's) has no writes counted for it.
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
