//! Families: files that are variants of one another (`whoosh_01.wav`, `whoosh_02.wav`), grouped so they show as one.

use std::collections::HashMap;

use anyhow::Result;
use hoard_core::AssetKind;
use rusqlite::{Connection, params};

/// The family a file belongs to: same folder, same kind, same name once a trailing variant number is stripped
/// (`whoosh_01.wav`, `whoosh_02.wav`, `Whoosh (3).wav` → `sfx/whoosh`). Single-letter suffixes count only for
/// audio, since on textures they're usually map types (`rock_n.png`).
pub(crate) fn family_base(rel: &str, kind: AssetKind) -> Option<String> {
    let (dir, file) = rel.rsplit_once('/').unwrap_or(("", rel));
    let stem = file.rsplit_once('.').map_or(file, |(s, _)| s).trim().to_lowercase();
    let is_sep = |c: char| matches!(c, '_' | '-' | ' ' | '.');
    let base = if let Some(inner) = stem.strip_suffix(')')
        && let Some((b, n)) = inner.rsplit_once('(')
        && !n.is_empty()
        && n.chars().all(|c| c.is_ascii_digit())
    {
        // "Whoosh (3)"
        b
    } else if let Some(b) = stem.strip_suffix(|c: char| c.is_ascii_digit()) {
        // "whoosh_01", "tree2", "rock_v3" (at most four digits: "texture_2048" is a size, not a variant)
        let b = b.trim_end_matches(|c: char| c.is_ascii_digit());
        if stem.len() - b.len() > 4 {
            return None;
        }
        b.strip_suffix('v').filter(|b| b.ends_with(is_sep)).unwrap_or(b)
    } else if kind == AssetKind::Audio
        && let Some((b, letter)) = stem.rsplit_once(is_sep)
        && letter.len() == 1
        && letter.chars().all(|c| c.is_ascii_lowercase())
    {
        // "hit_a", "hit_b"
        b
    } else {
        return None;
    };
    let base = base.trim_end_matches(is_sep);
    (!base.is_empty()).then(|| format!("{dir}/{base}\0{}", kind.as_str()))
}

/// Group a pack's numbered variants into families (only groups of two or more). Returns the number of families.
pub(crate) fn assign_families(conn: &Connection, pack_id: &str) -> Result<usize> {
    let rows: Vec<(String, String, String)> = {
        let mut stmt = conn.prepare("SELECT id, path, kind FROM assets WHERE pack_id = ?1")?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for (id, path, kind) in rows {
        let Some(kind) = AssetKind::parse(&kind) else { continue };
        if matches!(kind, AssetKind::Other | AssetKind::Document) {
            continue;
        }
        if let Some(base) = family_base(&path, kind) {
            groups.entry(format!("{pack_id}\0{base}")).or_default().push(id);
        }
    }
    conn.execute("DELETE FROM asset_family WHERE asset_id IN (SELECT id FROM assets WHERE pack_id = ?1)", [pack_id])?;
    let mut n = 0;
    let mut stmt = conn.prepare("INSERT OR REPLACE INTO asset_family (asset_id, family_key) VALUES (?1, ?2)")?;
    for (key, ids) in groups.iter().filter(|(_, ids)| ids.len() >= 2) {
        n += 1;
        for id in ids {
            stmt.execute(params![id, key])?;
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_variant_bases() {
        let b = |p| family_base(p, AssetKind::Audio);
        assert_eq!(b("sfx/Whoosh_01.wav"), b("sfx/whoosh-12.wav"));
        assert_eq!(b("sfx/Whoosh (3).wav"), b("sfx/whoosh_01.wav"));
        assert_eq!(b("sfx/hit_a.wav"), b("sfx/hit_b.wav"));
        assert_eq!(b("sfx/rock_v2.wav"), b("sfx/rock_3.wav"));
        assert_ne!(b("sfx/whoosh_01.wav"), b("other/whoosh_01.wav"));
        assert_eq!(b("sfx/whoosh.wav"), None);
        assert_eq!(family_base("tex/rock_n.png", AssetKind::Texture), None);
        assert_eq!(
            family_base("tex/sand_2048.png", AssetKind::Texture),
            Some("tex/sand_\0texture".replace("_\0", "\0"))
        );
        assert_eq!(family_base("tex/sand_20480.png", AssetKind::Texture), None);
    }
}
