//! What each write to the library changed, so what's kept from it (a search's tags, say) can be put right where it
//! changed rather than worked out again from the whole library, and isn't thrown away for writes that don't touch
//! what it's from at all (a job's progress, someone signing in).
//!
//! Each connection notes what its transaction changes as it goes (see [`watch`]), forgets it if the transaction is
//! rolled back, and once it's committed, adds it to [`JOURNAL`] (see `super::open`).

use std::collections::{HashSet, VecDeque};
use std::fmt::Write as _;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use rusqlite::Connection;
use rusqlite::functions::FunctionFlags;

/// Tables whose rows don't change what a search finds or counts: writing only to these leaves what's kept from
/// searches as it was. Any table not here counts as the library changing.
const NOT_SEARCHED: &[&str] = &[
    "access_tokens",
    "achievements",
    "asset_history",
    "derivatives",
    "embeddings",
    "embeddings_by_model",
    "issues",
    "jobs",
    "kept_formats",
    "look_embeddings",
    "looks",
    "maintenance_runs",
    "oauth_clients",
    "oauth_grants",
    "saved_searches",
    "sessions",
    "settings",
    "skipped_files",
    "skipped_folders",
    "sound_embeddings",
    "sounds",
    "upload_files",
    "upload_meta",
    "upload_parts",
    "usage_ask",
    "users",
];

/// The most assets one [`Changed`] names before it gives up and says only that it's [`Changed::too_much`]: past
/// this, working out again from the whole library is about as quick as putting each one right.
const MOST: usize = 50_000;

/// What some writes to the library changed, as far as what's kept from it needs to know.
#[derive(Default, Clone, Debug)]
pub(crate) struct Changed {
    /// Assets whose tags changed, by id.
    pub(crate) tagged: HashSet<String>,
    /// Assets added, changed or removed, by row.
    pub(crate) rows: HashSet<i64>,
    /// The tag tree changed.
    pub(crate) tree: bool,
    /// What a search tells assets apart by changed, which it keeps for all of them (see `search::Facts`): an asset
    /// was added or removed, or its kind, pack, place in the library, or whether it's archived or supports another
    /// changed, or a pack was added, removed, archived or brought back.
    pub(crate) facts: bool,
    /// More changed than is named here: everything kept has to be worked out again.
    pub(crate) too_much: bool,
}

impl Changed {
    fn add(&mut self, other: &Changed) {
        self.tree |= other.tree;
        self.facts |= other.facts;
        self.too_much |= other.too_much;
        if !self.too_much {
            self.tagged.extend(other.tagged.iter().cloned());
            self.rows.extend(&other.rows);
        }
        self.check_size();
    }

    fn check_size(&mut self) {
        if self.too_much || self.tagged.len() + self.rows.len() > MOST {
            self.too_much = true;
            self.tagged = HashSet::new();
            self.rows = HashSet::new();
        }
    }

    fn size(&self) -> usize {
        self.tagged.len() + self.rows.len()
    }
}

/// What one connection's transaction has changed so far.
#[derive(Default)]
pub(crate) struct Pending {
    changed: Changed,
    /// It wrote to a table a search reads (anything not [`NOT_SEARCHED`]).
    library: bool,
    /// A hook saw any row of the library's database change.
    seen: bool,
}

/// Have this connection note what each of its transactions changes in `pending`. SQLite's update hook sees rows of
/// tables with rowids change (`assets`), but not of those without (`asset_tags`, `tag_parents`): a temporary trigger
/// on each of those notes them instead. (SQLite's pre-update hook sees both, but needs a C toolchain's libclang to
/// build.) The triggers are on the tables there are when the connection opens, so it must open once the schema is up
/// to date.
pub(crate) fn watch(c: &mut Connection, pending: &Arc<Mutex<Pending>>) -> rusqlite::Result<()> {
    let seen = Arc::clone(pending);
    c.update_hook(Some(move |_, db: &str, table: &str, row: i64| {
        row_changed(&mut seen.lock().unwrap_or_else(PoisonError::into_inner), db, table, row);
    }))?;
    let seen = Arc::clone(pending);
    // Not SQLITE_DIRECTONLY: the triggers call it.
    c.create_scalar_function("hoard_noted", 2, FunctionFlags::SQLITE_UTF8, move |f| {
        let table: String = f.get(0)?;
        let asset: Option<String> = f.get(1)?;
        noted(&mut seen.lock().unwrap_or_else(PoisonError::into_inner), &table, asset);
        Ok(rusqlite::types::Null)
    })?;
    let tables: Vec<String> = c
        .prepare(
            "SELECT name FROM pragma_table_list
             WHERE schema = 'main' AND type = 'table' AND wr = 1 AND name NOT LIKE 'sqlite_%'",
        )?
        .query_map([], |r| r.get(0))?
        .collect::<rusqlite::Result<_>>()?;
    let mut sql = String::new();
    for t in tables {
        // Which asset, where a table's rows are an asset's.
        let key = |row: &str| if t == "asset_tags" { format!("{row}.asset_id") } else { "NULL".to_owned() };
        let (new, old) = (key("new"), key("old"));
        let _ = write!(
            sql,
            "CREATE TEMP TRIGGER IF NOT EXISTS \"noted_{t}_added\" AFTER INSERT ON main.\"{t}\"
               BEGIN SELECT hoard_noted('{t}', {new}); END;
             CREATE TEMP TRIGGER IF NOT EXISTS \"noted_{t}_changed\" AFTER UPDATE ON main.\"{t}\"
               BEGIN SELECT hoard_noted('{t}', {old}), hoard_noted('{t}', {new}); END;
             CREATE TEMP TRIGGER IF NOT EXISTS \"noted_{t}_removed\" AFTER DELETE ON main.\"{t}\"
               BEGIN SELECT hoard_noted('{t}', {old}); END;\n"
        );
    }
    sql.push_str(FACT_TRIGGERS);
    c.execute_batch(&sql)
}

/// Triggers noting what a search keeps of each asset changing (see [`Changed::facts`]), which the update hook can't
/// tell apart from what it doesn't keep (a thumbnail made, a rating). Whether an asset supports another depends on
/// what its meta says it's a take of or owned by.
const FACT_TRIGGERS: &str = "
    CREATE TEMP TRIGGER IF NOT EXISTS noted_assets_added AFTER INSERT ON main.assets
      BEGIN SELECT hoard_noted('assets', NULL); END;
    CREATE TEMP TRIGGER IF NOT EXISTS noted_assets_removed AFTER DELETE ON main.assets
      BEGIN SELECT hoard_noted('assets', NULL); END;
    CREATE TEMP TRIGGER IF NOT EXISTS noted_assets_changed
      AFTER UPDATE OF id, pack_id, kind, ext, path, created_at, archived_at, supports, fts_row ON main.assets
      BEGIN SELECT hoard_noted('assets', NULL); END;
    CREATE TEMP TRIGGER IF NOT EXISTS noted_assets_meta AFTER UPDATE OF meta ON main.assets
      WHEN json_extract(old.meta, '$.take_of') IS NOT json_extract(new.meta, '$.take_of')
        OR json_extract(old.meta, '$.owned_by') IS NOT json_extract(new.meta, '$.owned_by')
      BEGIN SELECT hoard_noted('assets', NULL); END;
    CREATE TEMP TRIGGER IF NOT EXISTS noted_packs_added AFTER INSERT ON main.packs
      BEGIN SELECT hoard_noted('packs', NULL); END;
    CREATE TEMP TRIGGER IF NOT EXISTS noted_packs_removed AFTER DELETE ON main.packs
      BEGIN SELECT hoard_noted('packs', NULL); END;
    CREATE TEMP TRIGGER IF NOT EXISTS noted_packs_changed AFTER UPDATE OF id, archived_at ON main.packs
      BEGIN SELECT hoard_noted('packs', NULL); END;
";

/// A row of a table with rowids was added, changed or removed (SQLite's update hook).
fn row_changed(pending: &mut Pending, db: &str, table: &str, row: i64) {
    // Temporary tables (a search's own, see `search_ranked`) aren't the library.
    if db != "main" {
        return;
    }
    pending.seen = true;
    if NOT_SEARCHED.contains(&table) {
        return;
    }
    pending.library = true;
    let changed = &mut pending.changed;
    if changed.too_much {
        return;
    }
    if table == "assets" {
        changed.rows.insert(row);
    }
    changed.check_size();
}

/// A row of a table without rowids was added, changed or removed (one of [`watch`]'s triggers), with the asset's id
/// for an asset's tags.
fn noted(pending: &mut Pending, table: &str, asset: Option<String>) {
    pending.seen = true;
    if NOT_SEARCHED.contains(&table) {
        return;
    }
    pending.library = true;
    let changed = &mut pending.changed;
    if changed.too_much {
        return;
    }
    match table {
        "asset_tags" => match asset {
            Some(id) => {
                changed.tagged.insert(id);
            }
            None => changed.too_much = true,
        },
        "tag_parents" => changed.tree = true,
        "assets" | "packs" => changed.facts = true,
        _ => {}
    }
    changed.check_size();
}

/// The transaction was rolled back: what it changed never happened.
pub(crate) fn rolled_back(pending: &mut Pending) {
    *pending = Pending::default();
}

/// How many recent writes to the library [`JOURNAL`] keeps what changed of.
const KEPT: usize = 512;
/// The most assets [`JOURNAL`] names in all, once it has more than one write.
const KEPT_ASSETS: usize = 4 * MOST;

/// What each recent write to the library changed, with its number ([`library_writes`] once it was made), oldest
/// first.
static JOURNAL: Mutex<VecDeque<(u64, Changed)>> = Mutex::new(VecDeque::new());

/// How many writes to the library (see [`NOT_SEARCHED`]) there have been since the server started.
static LIBRARY_WRITES: AtomicU64 = AtomicU64::new(0);

/// What's kept from the library (a search's counts, say) is good while this stays the same. Unlike
/// [`super::writes`], writes to tables a search doesn't read don't count.
pub(crate) fn library_writes() -> u64 {
    LIBRARY_WRITES.load(Ordering::Acquire)
}

/// The transaction was committed (SQLite's WAL hook): what it changed is there for others to read.
pub(crate) fn committed(pending: &mut Pending) {
    let Pending { mut changed, mut library, seen } = std::mem::take(pending);
    // Nothing a hook sees (the schema changing, say): it could have been anything.
    if !seen {
        library = true;
        changed.too_much = true;
        changed.check_size();
    }
    if !library {
        return;
    }
    let mut journal = JOURNAL.lock().unwrap_or_else(PoisonError::into_inner);
    // In the journal before it's counted, so whoever reads the count finds what changed up to it.
    let number = LIBRARY_WRITES.load(Ordering::Acquire) + 1;
    journal.push_back((number, changed));
    let mut size: usize = journal.iter().map(|(_, c)| c.size()).sum();
    while journal.len() > KEPT || (journal.len() > 1 && size > KEPT_ASSETS) {
        if let Some((_, c)) = journal.pop_front() {
            size -= c.size();
        }
    }
    LIBRARY_WRITES.store(number, Ordering::Release);
}

/// What changed in the library after its `from`th write, up to and including its `to`th, or `None` if that's no
/// longer known (it was too long ago).
pub(crate) fn since(from: u64, to: u64) -> Option<Changed> {
    let journal = JOURNAL.lock().unwrap_or_else(PoisonError::into_inner);
    let mut changed = Changed::default();
    if from >= to {
        return Some(changed);
    }
    // Everything after `from` has to be there.
    if journal.front().is_none_or(|(n, _)| *n > from + 1) {
        return None;
    }
    for (_, c) in journal.iter().filter(|(n, _)| *n > from && *n <= to) {
        changed.add(c);
        if changed.too_much {
            break;
        }
    }
    Some(changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Other tests write to libraries of their own meanwhile, and the journal is the server's (it has one library), so
    // this looks for its own assets among what changed, by names no other test uses.
    #[test]
    fn notes_what_a_committed_write_changed_and_forgets_a_rolled_back_one() {
        let dir = tempfile::tempdir().unwrap();
        let pool = super::super::open(&dir.path().join("hoard.db"), &dir.path().join("backups")).unwrap();
        let mut conn = pool.get().unwrap();
        conn.execute_batch(
            "INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'P', 'ready', 'x');
             INSERT INTO blobs (hash, size, created_at) VALUES ('b', 1, 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at)
               VALUES ('journal-1', 'p', 'b', 'a1.png', 'a1', 'png', 'texture', 'x', 'x'),
                      ('journal-2', 'p', 'b', 'a2.png', 'a2', 'png', 'texture', 'x', 'x');",
        )
        .unwrap();
        let row = |id: &str| conn.query_row("SELECT rowid FROM assets WHERE id = ?1", [id], |r| r.get::<_, i64>(0));
        let (a1, a2) = (row("journal-1").unwrap(), row("journal-2").unwrap());

        let before = library_writes();
        conn.execute("INSERT INTO asset_tags (asset_id, tag, source) VALUES ('journal-1', 'red', 'x')", []).unwrap();
        let changed = since(before, library_writes()).unwrap();
        assert!(changed.tagged.contains("journal-1"));
        assert!(!changed.tagged.contains("journal-2"));

        // Rolled back, nothing changed.
        let before = library_writes();
        {
            let tx = conn.transaction().unwrap();
            tx.execute("INSERT INTO asset_tags (asset_id, tag, source) VALUES ('journal-2', 'blue', 'x')", []).unwrap();
            tx.rollback().unwrap();
        }
        conn.execute("UPDATE assets SET name = 'one' WHERE id = 'journal-1'", []).unwrap();
        let changed = since(before, library_writes()).unwrap();
        assert!(!changed.tagged.contains("journal-2"));
        assert!(changed.rows.contains(&a1));
        let before = library_writes();
        conn.execute("UPDATE assets SET kind = 'model' WHERE id = 'journal-2'", []).unwrap();
        assert!(since(before, library_writes()).unwrap().facts, "a kind is something a search keeps");

        // An asset removed takes its tags with it; the tree changing is noted.
        let before = library_writes();
        conn.execute_batch(
            "BEGIN; DELETE FROM assets WHERE id = 'journal-1'; UPDATE assets SET name = 'two' WHERE id = 'journal-2';
             INSERT INTO tag_parents (tag, parent) VALUES ('red', 'warm'); COMMIT;",
        )
        .unwrap();
        let changed = since(before, library_writes()).unwrap();
        assert!(changed.tagged.contains("journal-1"));
        assert!(changed.rows.contains(&a1) && changed.rows.contains(&a2));
        assert!(changed.tree);
    }

    #[test]
    fn what_was_too_long_ago_is_not_known() {
        let mut journal = VecDeque::from([(5, Changed::default())]);
        std::mem::swap(&mut journal, &mut JOURNAL.lock().unwrap());
        // (Put back at once: other tests use it.)
        let (old, now) = (since(3, 5), since(4, 5));
        std::mem::swap(&mut journal, &mut JOURNAL.lock().unwrap());
        assert!(old.is_none());
        assert!(now.is_some());
    }
}
