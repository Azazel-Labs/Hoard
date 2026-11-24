//! What a search tells assets apart by, every asset's kept in memory between searches.

use std::sync::Arc;

use anyhow::Result;
use rusqlite::Connection;

use super::tag_rows::{Rows, tag_rows};
use super::{Counts, LIVE, SUPPORTING, SearchQuery, Tree, Words, browsed_folder, chosen_order, kind_facets, words_in};

/// What a search tells assets apart by, for every asset in the library by its row: its kind and pack, whether it's in
/// the library rather than the archive, whether it supports another, and where browsing shows it. With these, a
/// search by words goes through the index's matches, and browsing through the library in order, without reading each
/// asset for them: the first letter typed finds most of a big library, and joining each match to its asset took a
/// third of a second. Tags, ratings and thumbnails come and go without them changing, and where they do change (an
/// import adding assets, say) only those assets are read again (see [`patch_facts`]).
#[derive(Clone)]
pub(super) struct Facts {
    /// Each row's kind, as its place in `kinds`: [`NO_ASSET`] for a row no asset has.
    pub(super) kind: Vec<u16>,
    /// Each row's [`IN_LIBRARY`] and [`SUPPORTS`].
    pub(super) flags: Vec<u8>,
    /// Each row's pack, as its place in `packs`.
    pub(super) pack: Vec<u32>,
    pub(super) kinds: Vec<String>,
    pub(super) packs: std::collections::HashMap<String, u32>,
    /// The row of the asset each row of the search index is of, by the index's row ([`NO_ROW`] for none).
    pub(super) by_fts_row: Vec<u32>,
    /// Every asset's row, newest first, as browsing goes.
    pub(super) order: Vec<u32>,
    /// The rows of the materials each asset owns, by its id (see [`SUPPORTING`]), and some it no longer does: whether
    /// they support it changes when it's added or removed.
    pub(super) owners: std::collections::HashMap<String, Vec<u32>>,
}

pub(super) const NO_ASSET: u16 = u16::MAX;
pub(super) const NO_ROW: u32 = u32::MAX;
/// Neither the asset nor its pack is archived (see [`LIVE`]).
pub(super) const IN_LIBRARY: u8 = 1;
/// See [`SUPPORTING`].
pub(super) const SUPPORTS: u8 = 2;

/// What [`Facts`] read of each asset, as `a`: its row, its row in the search index, kind, pack, whether it's
/// [`IN_LIBRARY`], whether it [`SUPPORTS`] another, and the asset owning it, for a material (as text, as the
/// library compares it with its ids).
pub(super) fn fact_columns() -> String {
    format!(
        "a.rowid, a.fts_row, a.kind, a.pack_id, {LIVE}, {SUPPORTING},
         CASE WHEN a.ext = 'material' THEN CAST(json_extract(a.meta, '$.owned_by') AS TEXT) END"
    )
}

/// The most assets [`patch_facts`] reads again one by one: past this, reading them all is about as quick.
pub(super) const MOST_PATCHED: usize = 10_000;

/// [`Facts`], kept with the library's path and how many writes to it there had been (see [`crate::db::library_writes`]).
pub(super) static FACTS: std::sync::Mutex<Option<(String, u64, Arc<Facts>)>> = std::sync::Mutex::new(None);

/// The library's [`Facts`], or `None` if its rows are too far apart to keep by row.
pub(super) fn facts(conn: &Connection) -> Result<Option<Arc<Facts>>> {
    let writes = crate::db::library_writes();
    // An in-memory library (a test's) has no writes counted for it.
    let Some(path) = conn.path().filter(|p| !p.is_empty()) else { return Ok(read_facts(conn)?.map(Arc::new)) };
    let mut kept = FACTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // Taken while it's put right, so it's gone if that fails partway.
    if let Some((path, at, mut facts)) = kept.take().filter(|(p, _, _)| p == path) {
        let changed = if at == writes {
            Some(crate::db::changes::Changed::default())
        } else {
            crate::db::changes::since(at, writes)
        };
        if let Some(c) = changed.filter(|c| !c.facts && !c.too_much)
            && ((c.fact_rows.is_empty() && c.fact_ids.is_empty()) || patch_facts(conn, Arc::make_mut(&mut facts), &c)?)
        {
            *kept = Some((path, writes, Arc::clone(&facts)));
            return Ok(Some(facts));
        }
    }
    let facts = read_facts(conn)?.map(Arc::new);
    *kept = facts.as_ref().map(|f| (path.to_owned(), writes, Arc::clone(f)));
    Ok(facts)
}

/// One asset's [`fact_columns`] but its row.
pub(super) struct AssetFacts {
    pub(super) fts_row: Option<usize>,
    pub(super) kind: String,
    pub(super) pack: String,
    pub(super) flags: u8,
    pub(super) owner: Option<String>,
}

impl AssetFacts {
    /// From [`fact_columns`], after the row.
    pub(super) fn of(r: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            fts_row: r.get::<_, Option<i64>>(1)?.and_then(|f| usize::try_from(f).ok()),
            kind: r.get(2)?,
            pack: r.get(3)?,
            flags: (u8::from(r.get::<_, bool>(4)?) * IN_LIBRARY) | (u8::from(r.get::<_, bool>(5)?) * SUPPORTS),
            owner: r.get(6)?,
        })
    }
}

impl Facts {
    /// Rows up to `rows` kept, and the search index's up to `fts_rows`.
    pub(super) fn make_room(&mut self, rows: usize, fts_rows: usize) {
        if self.kind.len() <= rows {
            self.kind.resize(rows + 1, NO_ASSET);
            self.flags.resize(rows + 1, 0);
            self.pack.resize(rows + 1, 0);
        }
        if self.by_fts_row.len() <= fts_rows {
            self.by_fts_row.resize(fts_rows + 1, NO_ROW);
        }
    }

    /// Keeps `facts` as the asset at `at`'s, or returns `false` if there are too many kinds to.
    pub(super) fn set(&mut self, row: u32, facts: AssetFacts) -> bool {
        let at = row as usize;
        if let Some(fts_row) = facts.fts_row {
            self.by_fts_row[fts_row] = row;
        }
        let kind = if let Some(k) = self.kinds.iter().position(|k| *k == facts.kind) {
            k
        } else {
            self.kinds.push(facts.kind);
            self.kinds.len() - 1
        };
        let Some(kind) = u16::try_from(kind).ok().filter(|&k| k != NO_ASSET) else { return false };
        let next = u32::try_from(self.packs.len()).unwrap_or(NO_ROW);
        self.pack[at] = *self.packs.entry(facts.pack).or_insert(next);
        self.kind[at] = kind;
        self.flags[at] = facts.flags;
        if let Some(owner) = facts.owner {
            let owned = self.owners.entry(owner).or_default();
            if !owned.contains(&row) {
                owned.push(row);
            }
        }
        true
    }
}

/// Rows and the search index's rows past this would take more room kept by row than they save.
pub(super) fn most_row(assets: usize) -> usize {
    assets.saturating_mul(8).saturating_add(1_000_000)
}

/// [`Facts`] read from the whole library.
pub(super) fn read_facts(conn: &Connection) -> Result<Option<Facts>> {
    let (rows, most, most_fts): (i64, i64, i64) =
        conn.query_row("SELECT count(*), IFNULL(max(rowid), 0), IFNULL(max(fts_row), 0) FROM assets", [], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })?;
    let (Ok(rows), Ok(most), Ok(most_fts)) = (usize::try_from(rows), usize::try_from(most), usize::try_from(most_fts))
    else {
        return Ok(None);
    };
    if most > most_row(rows) || most_fts > most_row(rows) {
        return Ok(None);
    }
    let mut facts = Facts {
        kind: Vec::new(),
        flags: Vec::new(),
        pack: Vec::new(),
        kinds: Vec::new(),
        packs: std::collections::HashMap::new(),
        by_fts_row: Vec::new(),
        order: Vec::with_capacity(rows),
        owners: std::collections::HashMap::new(),
    };
    facts.make_room(most, most_fts);
    let mut stmt =
        conn.prepare(&format!("SELECT {} FROM assets a ORDER BY a.created_at DESC, a.path ASC", fact_columns()))?;
    let mut found = stmt.query([])?;
    while let Some(r) = found.next()? {
        let Some(row) = u32::try_from(r.get::<_, i64>(0)?).ok().filter(|&r| r as usize <= most_row(rows)) else {
            return Ok(None);
        };
        let asset = AssetFacts::of(r)?;
        // (Room for a row or index row written since the most were read.)
        match asset.fts_row {
            Some(f) if f > most_row(rows) => return Ok(None),
            f => facts.make_room(row as usize, f.unwrap_or(0)),
        }
        if !facts.set(row, asset) {
            return Ok(None);
        }
        facts.order.push(row);
    }
    Ok(Some(facts))
}

/// Puts `facts` right where `changed` says what they keep may have changed, reading again only those assets, or
/// returns `false` if they're better read again from the whole library (and are left to be).
pub(super) fn patch_facts(conn: &Connection, facts: &mut Facts, changed: &crate::db::changes::Changed) -> Result<bool> {
    use rusqlite::OptionalExtension as _;
    let mut rows = changed.fact_rows.clone();
    let mut by_id = conn.prepare_cached("SELECT rowid FROM assets WHERE id = ?1")?;
    for id in &changed.fact_ids {
        if let Some(row) = by_id.query_row([id], |r| r.get::<_, i64>(0)).optional()? {
            rows.insert(row);
        }
        if let Some(owned) = facts.owners.get(id) {
            rows.extend(owned.iter().map(|&r| i64::from(r)));
        }
    }
    if rows.len() > MOST_PATCHED {
        return Ok(false);
    }
    let most = most_row(facts.order.len() + rows.len());
    let mut read = conn
        .prepare_cached(&format!("SELECT {}, a.created_at, a.path FROM assets a WHERE a.rowid = ?1", fact_columns()))?;
    // Each asset read again, with where browsing shows it if it's to be put there: `None` for one that's gone.
    let mut fresh = Vec::with_capacity(rows.len());
    let (mut most_read, mut most_fts) = (0, 0);
    for row in rows {
        let Some(r) = u32::try_from(row).ok().filter(|&r| (r as usize) <= most) else { return Ok(false) };
        let asset = read
            .query_row([row], |r| {
                let place: (String, String) = (r.get(7)?, r.get(8)?);
                Ok((AssetFacts::of(r)?, place))
            })
            .optional()?;
        if let Some((a, _)) = &asset {
            if a.fts_row.is_some_and(|f| f > most) {
                return Ok(false);
            }
            most_fts = most_fts.max(a.fts_row.unwrap_or(0));
        }
        most_read = most_read.max(r as usize);
        fresh.push((r, asset));
    }
    facts.make_room(most_read, most_fts);
    // What the search index's rows were of, for the assets read again, is what they've just been read as.
    let mut again = vec![false; facts.kind.len()];
    for (row, _) in &fresh {
        again[*row as usize] = true;
    }
    for of in &mut facts.by_fts_row {
        if *of != NO_ROW && again[*of as usize] {
            *of = NO_ROW;
        }
    }
    let mut moved = Vec::new();
    let mut placing = Vec::new();
    for (row, asset) in fresh {
        let placed = changed.placed.contains(&i64::from(row));
        if placed {
            moved.push(row);
        }
        if let Some((asset, place)) = asset {
            if !facts.set(row, asset) {
                return Ok(false);
            }
            if placed {
                placing.push((place, row));
            }
        } else {
            facts.kind[row as usize] = NO_ASSET;
            facts.flags[row as usize] = 0;
            moved.push(row);
        }
    }
    if moved.is_empty() {
        return Ok(true);
    }
    again.fill(false);
    for row in moved {
        again[row as usize] = true;
    }
    facts.order.retain(|r| !again[*r as usize]);
    // Newest first, by path where they're as new; as `assets_created` has them.
    placing.sort_unstable_by(|((at, path), _), ((at2, path2), _)| at2.cmp(at).then_with(|| path.cmp(path2)));
    let first = match facts.order.first() {
        Some(&row) => conn
            .query_row("SELECT created_at, path FROM assets WHERE rowid = ?1", [row], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            })
            .optional()?,
        None => None,
    };
    // What's added is usually newer than everything there: it goes first. A few others are each put in place, and
    // for more, the order is read again, which only the index has to be.
    let newest = match (placing.last(), &first) {
        (Some(((at, path), _)), Some((first_at, first_path))) => at > first_at || (at == first_at && path < first_path),
        _ => facts.order.is_empty() || placing.is_empty(),
    };
    if newest {
        facts.order.splice(0..0, placing.into_iter().map(|(_, row)| row));
    } else if placing.len() > MOST_PLACED || !place_each(conn, &mut facts.order, placing)? {
        let mut stmt = conn.prepare_cached("SELECT rowid FROM assets ORDER BY created_at DESC, path ASC")?;
        let order = stmt.query_map([], |r| r.get::<_, i64>(0))?;
        facts.order.clear();
        for row in order {
            let Some(row) = u32::try_from(row?).ok().filter(|&r| (r as usize) < facts.kind.len()) else {
                return Ok(false);
            };
            facts.order.push(row);
        }
    }
    Ok(true)
}

/// The most assets [`patch_facts`] puts in browsing's order one by one, each found by halves.
pub(super) const MOST_PLACED: usize = 64;

/// Puts each of `placing` where it goes in `order` (see [`Facts::order`]), found by halves, or returns `false` if one
/// is as new as one there and at the same path, or one there is gone: then it's for the index to say.
pub(super) fn place_each(
    conn: &Connection,
    order: &mut Vec<u32>,
    placing: Vec<((String, String), u32)>,
) -> Result<bool> {
    use rusqlite::OptionalExtension as _;
    let mut place = conn.prepare_cached("SELECT created_at, path FROM assets WHERE rowid = ?1")?;
    for ((at, path), row) in placing {
        let (mut from, mut to) = (0, order.len());
        while from < to {
            let mid = from + (to - from) / 2;
            let there =
                place.query_row([order[mid]], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))).optional()?;
            let Some((there_at, there_path)) = there else { return Ok(false) };
            // Newest first, by path where as new.
            match there_at.cmp(&at).reverse().then_with(|| there_path.cmp(&path)) {
                std::cmp::Ordering::Less => from = mid + 1,
                std::cmp::Ordering::Greater => to = mid,
                std::cmp::Ordering::Equal => return Ok(false),
            }
        }
        order.insert(from, row);
    }
    Ok(true)
}

/// What a search asks of [`Facts`], when it asks for nothing they don't keep.
pub(super) struct FactsSearch {
    /// The words, as the search index takes them.
    pub(super) find: Option<String>,
    /// Words after a `-`.
    pub(super) without: Option<String>,
    /// Best match first, rather than newest first.
    pub(super) ranked: bool,
    pub(super) kind: Option<String>,
    pub(super) pack: Option<String>,
    pub(super) tags: Vec<String>,
    pub(super) hide_supporting: bool,
}

/// The search as one [`Facts`] can do, or `None` if it asks for something they don't keep (a folder, a rating, an
/// order but the best match or the newest first).
pub(super) fn facts_search(q: &SearchQuery, tree: &Tree) -> Option<FactsSearch> {
    let set = |t: &String| t.starts_with("set:");
    let fits = browsed_folder(q).is_none()
        && q.ext.as_deref().is_none_or(str::is_empty)
        && q.reviewed.is_none()
        && q.rating_min.is_none_or(|n| n == 0)
        && q.bpm_min.is_none()
        && q.bpm_max.is_none()
        && q.key.as_deref().is_none_or(|k| k.trim().is_empty())
        && q.favorites_of.is_none()
        && q.project.as_deref().is_none_or(str::is_empty)
        && q.collection.as_deref().is_none_or(str::is_empty)
        && !q.archived
        // `set:` tags aren't in [`TagRows`].
        && !q.tags.iter().any(|t| set(t) || tree.narrower(t).iter().any(set));
    // Any other order is put together in SQL. (A sort that isn't one is the usual order.)
    let newest = match chosen_order(q.sort.as_deref()) {
        None => false,
        Some("a.created_at DESC, a.path ASC") => true,
        Some(_) => return None,
    };
    if !fits {
        return None;
    }
    let Words { find, without, short, .. } = q.q.as_deref().map_or_else(Words::default, |t| words_in(t, tree));
    Some(FactsSearch {
        ranked: find.is_some() && !short && !newest,
        find,
        without,
        kind: q.kind.clone().filter(|k| !k.is_empty()),
        pack: q.pack.clone().filter(|p| !p.is_empty()),
        tags: q.tags.clone(),
        hide_supporting: q.hide_supporting,
    })
}

/// The rows of the assets with the index's rows that match `words`, in the index's order or best first.
pub(super) fn matched_rows(conn: &Connection, facts: &Facts, words: &str, ranked: bool) -> Result<Vec<u32>> {
    let sql = if ranked {
        "SELECT rowid FROM assets_fts WHERE assets_fts MATCH ?1
          ORDER BY bm25(assets_fts, 0.0, 8.0, 4.0, 3.0, 1.0, 5.0, 1.5, 1.0)"
    } else {
        "SELECT rowid FROM assets_fts WHERE assets_fts MATCH ?1"
    };
    let mut stmt = conn.prepare_cached(sql)?;
    let mut found = stmt.query([words])?;
    let mut rows = Vec::new();
    while let Some(r) = found.next()? {
        let row = usize::try_from(r.get::<_, i64>(0)?).ok().and_then(|f| facts.by_fts_row.get(f)).copied();
        if let Some(row) = row.filter(|&r| r != NO_ROW) {
            rows.push(row);
        }
    }
    Ok(rows)
}

/// [`rank_and_count`](super::rank_and_count) (or for browsing, [`count`](super::count) and the page) from [`Facts`].
pub(super) fn search_facts(
    conn: &Connection,
    facts: &Facts,
    s: &FactsSearch,
    tree: &Tree,
) -> Result<(Vec<i64>, Counts)> {
    let matched = s.find.as_deref().map(|w| matched_rows(conn, facts, w, s.ranked)).transpose()?;
    let without = s.without.as_deref().map(|w| matched_rows(conn, facts, w, false)).transpose()?;
    let without = without.map(|w| Rows::of(w.into_iter().map(i64::from)));
    let tagged = if s.tags.is_empty() {
        Vec::new()
    } else {
        let kept = tag_rows(conn, tree)?;
        s.tags.iter().map(|t| Rows::of(kept.rows_of(t).iter().map(|&r| i64::from(r)))).collect()
    };
    // A kind or pack no asset has finds nothing (`u16`/`u32::MAX`, which no asset is of).
    let kind = s
        .kind
        .as_deref()
        .map(|k| facts.kinds.iter().position(|n| n == k).map_or(NO_ASSET, |k| u16::try_from(k).unwrap_or(NO_ASSET)));
    let pack = s.pack.as_deref().map(|p| facts.packs.get(p).copied().unwrap_or(NO_ROW));
    let candidates: Box<dyn Iterator<Item = u32>> = match matched {
        Some(rows) if s.ranked => Box::new(rows.into_iter()),
        Some(rows) => {
            let matched = Rows::of(rows.iter().map(|&r| i64::from(r)));
            Box::new(facts.order.iter().copied().filter(move |&r| matched.has(i64::from(r))))
        }
        None => Box::new(facts.order.iter().copied()),
    };
    let (mut found, mut supporting_hidden) = (Vec::new(), 0);
    let mut kinds = vec![0_i64; facts.kinds.len()];
    for row in candidates {
        let at = row as usize;
        let (k, flags) = (facts.kind[at], facts.flags[at]);
        if k == NO_ASSET
            || flags & IN_LIBRARY == 0
            || pack.is_some_and(|p| facts.pack[at] != p)
            || without.as_ref().is_some_and(|w| w.has(i64::from(row)))
            || !tagged.iter().all(|t| t.has(i64::from(row)))
        {
            continue;
        }
        let of_kind = kind.is_none_or(|kind| kind == k);
        // Kinds count every kind; the rest only the kind asked for. Supporting files count only as hidden ones.
        if s.hide_supporting && flags & SUPPORTS != 0 {
            supporting_hidden += i64::from(of_kind);
            continue;
        }
        if of_kind {
            found.push(i64::from(row));
        }
        kinds[usize::from(k)] += 1;
    }
    let kinds = facts.kinds.iter().cloned().zip(kinds).filter(|(_, n)| *n > 0).collect();
    let total = found.len() as i64;
    Ok((found, Counts { total, supporting_hidden, kinds: kind_facets(kinds), folders: Vec::new(), tags: None }))
}
