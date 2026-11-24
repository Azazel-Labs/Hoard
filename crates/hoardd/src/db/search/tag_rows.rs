//! The sidebar's tags, and every tag's assets by row, kept between searches.

use std::sync::Arc;

use anyhow::Result;
use rusqlite::Connection;

use super::{Facet, Filter, Tree, params_from_iter};

/// The most tags the sidebar shows.
pub(super) const SIDEBAR_TAGS: usize = 60;

/// The sidebar's tags: the most common among what the search finds. A tag with tags under it counts what has any of
/// them, once each (see [`TagRows`]).
pub(super) fn tags(conn: &Connection, f: &Filter, tree: &Tree) -> Result<Vec<Facet>> {
    let rows = conn
        .prepare(&format!("SELECT a.rowid FROM assets a {} {}", f.join, f.where_sql()))?
        .query_map(params_from_iter(&f.params), |r| r.get::<_, i64>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    tags_of_rows(conn, &rows, tree)
}

/// [`tags`] of what a search found, by row (see [`rank_and_count`](super::rank_and_count)).
pub(super) fn tags_of_rows(conn: &Connection, rows: &[i64], tree: &Tree) -> Result<Vec<Facet>> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let kept = tag_rows(conn, tree)?;
    let found = Rows::of(rows.iter().copied());
    let mut tags: Vec<Facet> = kept
        .all()
        .filter_map(|(tag, rows)| {
            let n = rows.iter().filter(|&&r| found.has(i64::from(r))).count();
            (n > 0).then(|| Facet { value: tag.to_owned(), count: n as u64 })
        })
        .collect();
    tags.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
    tags.truncate(SIDEBAR_TAGS);
    Ok(tags)
}

/// Some assets by row, a bit each.
pub(super) struct Rows(Vec<u64>);

impl Rows {
    pub(super) fn of(rows: impl Iterator<Item = i64> + Clone) -> Rows {
        let most = rows.clone().max().map_or(0, |m| usize::try_from(m).unwrap_or(0));
        let mut bits = vec![0_u64; most / 64 + 1];
        for r in rows {
            if let Ok(r) = usize::try_from(r) {
                bits[r / 64] |= 1 << (r % 64);
            }
        }
        Rows(bits)
    }

    pub(super) fn has(&self, row: i64) -> bool {
        usize::try_from(row).is_ok_and(|r| self.0.get(r / 64).is_some_and(|w| w >> (r % 64) & 1 == 1))
    }
}

/// Every tag but `set:` ones, with the rows of the assets that have it, archived or not (a search leaves those out of
/// what it finds), in no order. A tag with tags under it has the rows of every asset with any of them, once each:
/// vehicle has the cars and the sedans, and is there even when nothing is tagged vehicle itself. (A row is taken as
/// fitting in 32 bits: four billion assets.)
#[derive(Clone, Default)]
pub(super) struct TagRows {
    /// Each tag's own assets.
    pub(super) own: std::collections::HashMap<String, Vec<u32>>,
    /// Each tag with tags under it, with its own assets and theirs.
    pub(super) broad: std::collections::HashMap<String, Vec<u32>>,
}

impl TagRows {
    /// The assets with `tag`, or with it or a tag under it for a broad one.
    pub(super) fn rows_of(&self, tag: &str) -> &[u32] {
        self.broad.get(tag).or_else(|| self.own.get(tag)).map_or(&[], Vec::as_slice)
    }

    /// Each tag with its assets, and those under it for a broad one.
    pub(super) fn all(&self) -> impl Iterator<Item = (&str, &[u32])> {
        let own = self.own.iter().filter(|(t, _)| !self.broad.contains_key(*t));
        own.chain(&self.broad).map(|(t, rows)| (t.as_str(), rows.as_slice()))
    }

    /// Add these (tag, row) pairs to the tags' own assets.
    pub(super) fn add_own(&mut self, pairs: impl IntoIterator<Item = (String, u32)>) {
        for (tag, row) in pairs {
            self.own.entry(tag).or_default().push(row);
        }
    }

    /// Work out the broad tags' assets from their own and those under them.
    pub(super) fn work_out_broad(&mut self, tree: &Tree) {
        self.broad = tree
            .broad()
            .filter(|(broad, _)| !broad.starts_with("set:"))
            .map(|(broad, under)| {
                let mut rows: Vec<u32> = std::iter::once(broad)
                    .chain(under.iter().map(String::as_str))
                    .filter_map(|t| self.own.get(t))
                    .flatten()
                    .copied()
                    .collect();
                rows.sort_unstable();
                rows.dedup();
                (broad.to_owned(), rows)
            })
            .collect();
    }
}

/// [`TagRows`], kept with the library's path and how many writes to it there had been (see
/// [`crate::db::library_writes`]): counting the tags of a quarter of a million assets in SQL took a second, and going
/// through these for them takes a few milliseconds. Reading them all again took a quarter of a second, so when the
/// library changes only the assets that changed are put right, unless a great many did (see [`crate::db::changes`]).
pub(super) static TAG_ROWS: std::sync::Mutex<Option<(String, u64, Arc<TagRows>)>> = std::sync::Mutex::new(None);

/// Each asset's tags, by its row: every one but `set:` ones (a pack's own sets, as many as there are packs).
pub(super) const TAG_PAIRS: &str = "SELECT t.tag, a.rowid FROM assets a JOIN asset_tags t ON t.asset_id = a.id
     WHERE t.tag NOT LIKE 'set:%'";

pub(super) fn tag_rows(conn: &Connection, tree: &Tree) -> Result<Arc<TagRows>> {
    // Read before reading the tags, so a write meanwhile is put right next time. (The tag tree changing is a write.)
    let writes = crate::db::library_writes();
    // An in-memory library (a test's) has no writes counted for it.
    let Some(path) = conn.path().filter(|p| !p.is_empty()) else { return Ok(Arc::new(read_tag_rows(conn, tree)?)) };
    let mut kept = TAG_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((_, at, rows)) = kept.as_mut().filter(|(p, _, _)| p == path) {
        if *at == writes {
            return Ok(Arc::clone(rows));
        }
        if let Some(changed) = crate::db::changes::since(*at, writes).filter(|c| !c.too_much) {
            put_right(conn, Arc::make_mut(rows), &changed, tree)?;
            *at = writes;
            return Ok(Arc::clone(rows));
        }
    }
    let rows = Arc::new(read_tag_rows(conn, tree)?);
    *kept = Some((path.to_owned(), writes, Arc::clone(&rows)));
    Ok(rows)
}

/// [`TagRows`] read from the whole library.
pub(super) fn read_tag_rows(conn: &Connection, tree: &Tree) -> Result<TagRows> {
    let mut all = TagRows::default();
    let mut stmt = conn.prepare_cached(TAG_PAIRS)?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let Ok(row) = u32::try_from(r.get::<_, i64>(1)?) else { continue };
        let tag = r.get_ref(0)?.as_str()?;
        match all.own.get_mut(tag) {
            Some(rows) => rows.push(row),
            None => {
                all.own.insert(tag.to_owned(), vec![row]);
            }
        }
    }
    all.work_out_broad(tree);
    Ok(all)
}

/// Put right in place the tags of the assets that `changed`: each is taken out of every tag's assets, and put back in
/// those it has now.
pub(super) fn put_right(
    conn: &Connection,
    kept: &mut TagRows,
    changed: &crate::db::changes::Changed,
    tree: &Tree,
) -> Result<()> {
    let mut rows: Vec<i64> = changed.rows.iter().copied().collect();
    if !changed.tagged.is_empty() {
        let ids = serde_json::to_string(&changed.tagged)?;
        let mut stmt = conn.prepare_cached("SELECT rowid FROM assets WHERE id IN (SELECT value FROM json_each(?1))")?;
        rows.extend(stmt.query_map([ids], |r| r.get::<_, i64>(0))?.collect::<rusqlite::Result<Vec<_>>>()?);
    }
    if !rows.is_empty() {
        let gone = Rows::of(rows.iter().copied());
        for list in kept.own.values_mut().chain(kept.broad.values_mut()) {
            list.retain(|&r| !gone.has(i64::from(r)));
        }
        kept.own.retain(|_, list| !list.is_empty());
    }
    let mut stmt = conn.prepare_cached(&format!("{TAG_PAIRS} AND a.rowid IN (SELECT value FROM json_each(?1))"))?;
    let mut now = Vec::new();
    if !rows.is_empty() {
        let mut found = stmt.query([serde_json::to_string(&rows)?])?;
        while let Some(r) = found.next()? {
            if let Ok(row) = u32::try_from(r.get::<_, i64>(1)?) {
                now.push((r.get::<_, String>(0)?, row));
            }
        }
    }
    if changed.tree {
        kept.add_own(now);
        kept.work_out_broad(tree);
        return Ok(());
    }
    // Each asset goes once in each broad tag one of its tags is under (or is).
    let mut broad = std::collections::HashSet::new();
    for (tag, row) in &now {
        for b in std::iter::once(tag).chain(tree.broader(tag)) {
            if kept.broad.contains_key(b) {
                broad.insert((b.clone(), *row));
            }
        }
    }
    for (b, row) in broad {
        if let Some(list) = kept.broad.get_mut(&b) {
            list.push(row);
        }
    }
    kept.add_own(now);
    Ok(())
}
