//! Assets: one at a time, their families, links and files.

use std::collections::HashMap;

use anyhow::Result;
use hoard_core::{Asset, AssetKind, AssetLink, LinkRel, Tag, TagSource};
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};

use super::asset_facing;

pub(super) const ASSET_COLS: &str = "a.id, a.pack_id, p.name, a.name, a.path, a.ext, a.kind, a.category, a.description,
    a.blob_hash, b.size, a.thumb_hash IS NOT NULL, a.meta, a.created_at, a.updated_at, a.source_description,
    a.ai_description, COALESCE(a.archived_at, p.archived_at), substr(a.thumb_hash, 1, 12), a.thumb_source,
    a.kind = 'animation' AND EXISTS (SELECT 1 FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'motion'),
    CASE WHEN a.kind = 'animation' THEN (SELECT MAX(json_extract(c.value, '$.secs')) FROM rigs r, json_each(r.rig, '$.clips') c
      WHERE r.asset_id = a.id AND json_extract(c.value, '$.secs') >= 0.1) END,
    a.reviewed_at, a.rating, a.license, a.source_url";

pub(super) fn asset_from_row(r: &Row<'_>) -> rusqlite::Result<Asset> {
    let meta: String = r.get(12)?;
    Ok(Asset {
        id: r.get(0)?,
        pack_id: r.get(1)?,
        pack_name: r.get(2)?,
        name: r.get(3)?,
        path: r.get(4)?,
        ext: r.get(5)?,
        kind: AssetKind::parse(&r.get::<_, String>(6)?).unwrap_or(AssetKind::Other),
        category: r.get(7)?,
        description: r.get(8)?,
        source_description: r.get(15)?,
        ai_description: r.get(16)?,
        hash: r.get(9)?,
        size: r.get::<_, i64>(10)? as u64,
        has_thumb: r.get(11)?,
        thumb_rev: r.get(18)?,
        thumb_source: r.get(19)?,
        has_motion: r.get(20)?,
        clip_secs: r.get(21)?,
        meta: serde_json::from_str(&meta).unwrap_or_default(),
        tags: Vec::new(),
        created_at: r.get(13)?,
        updated_at: r.get(14)?,
        archived_at: r.get(17)?,
        reviewed_at: r.get(22)?,
        rating: r.get::<_, Option<i64>>(23)?.and_then(|n| u8::try_from(n).ok()).filter(|n| (1..=5).contains(n)),
        license: r.get(24)?,
        source_url: r.get(25)?,
        pack_license: None,
        fields: std::collections::BTreeMap::new(),
        rejected_tags: Vec::new(),
        links: Vec::new(),
        linked_from: Vec::new(),
        derivatives: Vec::new(),
        family: None,
        duplicates: Vec::new(),
        near_copies: Vec::new(),
        fits: Vec::new(),
        fits_total: None,
        plays_on: None,
        plays_its_skeleton: false,
        collections: Vec::new(),
        origin: None,
        unreal: None,
        formats: Vec::new(),
        palette: Vec::new(),
        material: None,
        materials: Vec::new(),
        facing: None,
    })
}

pub(super) fn attach_tags(conn: &Connection, assets: &mut [Asset]) -> Result<()> {
    if assets.is_empty() {
        return Ok(());
    }
    let placeholders = vec!["?"; assets.len()].join(",");
    let mut stmt = conn.prepare(&format!(
        "SELECT asset_id, tag, source, confidence FROM asset_tags WHERE asset_id IN ({placeholders})
         ORDER BY CASE source WHEN 'human' THEN 0 WHEN 'rule' THEN 1 WHEN 'ai' THEN 2 ELSE 3 END, tag"
    ))?;
    let mut by_id: HashMap<String, Vec<Tag>> = HashMap::new();
    let rows = stmt.query_map(params_from_iter(assets.iter().map(|a| a.id.as_str())), |r| {
        Ok((
            r.get::<_, String>(0)?,
            Tag { tag: r.get(1)?, source: TagSource::parse(&r.get::<_, String>(2)?), confidence: r.get(3)? },
        ))
    })?;
    for row in rows {
        let (id, tag) = row?;
        by_id.entry(id).or_default().push(tag);
    }
    for a in assets {
        a.tags = by_id.remove(&a.id).unwrap_or_default();
    }
    Ok(())
}

pub(crate) fn get_asset(conn: &Connection, id: &str) -> Result<Option<Asset>> {
    let asset = conn
        .query_row(
            &format!(
                "SELECT {ASSET_COLS} FROM assets a JOIN packs p ON p.id = a.pack_id
                 JOIN blobs b ON b.hash = a.blob_hash WHERE a.id = ?1"
            ),
            [id],
            asset_from_row,
        )
        .optional()?;
    let Some(asset) = asset else { return Ok(None) };
    let mut one = [asset];
    attach_tags(conn, &mut one)?;
    let [mut asset] = one;
    let (links, linked_from) = asset_links(conn, &asset.id)?;
    asset.links = links;
    asset.linked_from = linked_from;
    // Cached "download as" conversions are an implementation detail, not derivatives to show.
    let mut stmt =
        conn.prepare("SELECT kind FROM derivatives WHERE asset_id = ?1 AND kind NOT LIKE 'as:%' ORDER BY kind")?;
    asset.derivatives = stmt.query_map([&asset.id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    asset.formats = crate::formats::options(&asset.ext, &asset.derivatives);
    if crate::formats::offers_clip(asset.kind == AssetKind::Animation, &asset.ext, &asset.derivatives) {
        asset.formats.push(crate::formats::clip_option());
    }
    asset.family = family(conn, &asset.id)?;
    // The pack's own license file, or else the license its store sells it under.
    asset.pack_license = conn
        .query_row(
            "SELECT COALESCE(NULLIF(trim(p.license), ''), NULLIF(trim(c.store_license), ''))
             FROM packs p LEFT JOIN pack_context c ON c.pack_id = p.id WHERE p.id = ?1",
            [&asset.pack_id],
            |r| r.get(0),
        )
        .optional()?
        .flatten();
    let mut stmt = conn.prepare("SELECT key, value FROM asset_fields WHERE asset_id = ?1 ORDER BY key")?;
    asset.fields = stmt.query_map([&asset.id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
    let mut stmt = conn.prepare("SELECT tag FROM rejected_tags WHERE asset_id = ?1 ORDER BY tag")?;
    asset.rejected_tags = stmt.query_map([&asset.id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    if asset.kind == AssetKind::Model {
        asset.facing = Some(asset_facing(conn, &asset.id, &asset.pack_id)?).filter(|f| !f.is_upright());
    }
    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.pack_id, p.name, a.path FROM assets a JOIN packs p ON p.id = a.pack_id
         WHERE a.blob_hash = ?1 AND a.id != ?2 ORDER BY p.name, a.path LIMIT 50",
    )?;
    asset.duplicates = stmt
        .query_map(params![asset.hash, asset.id], |r| {
            Ok(hoard_core::Duplicate {
                id: r.get(0)?,
                name: r.get(1)?,
                pack_id: r.get(2)?,
                pack_name: r.get(3)?,
                path: r.get(4)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    let mut stmt = conn.prepare(
        "SELECT c.id, c.name FROM collection_assets ca JOIN collections c ON c.id = ca.collection_id
         WHERE ca.asset_id = ?1 ORDER BY c.name COLLATE NOCASE",
    )?;
    asset.collections = stmt
        .query_map([&asset.id], |r| Ok(hoard_core::CollectionRef { id: r.get(0)?, name: r.get(1)? }))?
        .collect::<rusqlite::Result<_>>()?;
    asset.origin = crate::extract::origin_of(conn, &asset.id)?;
    crate::materials::attach(conn, &mut asset)?;
    asset.unreal = crate::extract::status_of(conn, &asset.id)?;
    let thumb: Option<String> = conn.query_row("SELECT thumb_hash FROM assets WHERE id = ?1", [id], |r| r.get(0))?;
    if let Some(thumb) = thumb {
        asset.palette = crate::looks::palette_of(conn, &thumb)?;
    }
    Ok(Some(asset))
}

/// A family member that isn't another format of one: that's the same member, not one more of the family.
const ONE_FORMAT: &str = "NOT EXISTS (SELECT 1 FROM asset_links l WHERE l.to_id = f.asset_id AND l.rel = 'format')";

/// The variant family an asset belongs to (if any), with up to 60 other members.
pub(crate) fn family(conn: &Connection, id: &str) -> Result<Option<hoard_core::Family>> {
    let Some(key): Option<String> =
        conn.query_row("SELECT family_key FROM asset_family WHERE asset_id = ?1", [id], |r| r.get(0)).optional()?
    else {
        return Ok(None);
    };
    let size: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM asset_family f WHERE f.family_key = ?1 AND {ONE_FORMAT}"),
        [&key],
        |r| r.get(0),
    )?;
    let mut stmt = conn.prepare(&format!(
        "SELECT a.id, a.name, a.path, a.kind, a.thumb_hash IS NOT NULL, b.size FROM asset_family f
         JOIN assets a ON a.id = f.asset_id JOIN blobs b ON b.hash = a.blob_hash
         WHERE f.family_key = ?1 AND a.id != ?2 AND {ONE_FORMAT} ORDER BY a.path LIMIT 60"
    ))?;
    let members = stmt
        .query_map(params![key, id], |r| {
            Ok(AssetLink {
                id: r.get(0)?,
                name: r.get(1)?,
                path: r.get(2)?,
                kind: AssetKind::parse(&r.get::<_, String>(3)?).unwrap_or(AssetKind::Other),
                rel: LinkRel::Variant,
                has_thumb: r.get(4)?,
                size: r.get::<_, i64>(5)? as u64,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let display = key.split_once('\u{0}').map_or(key.as_str(), |(_, k)| k).to_owned();
    Ok(Some(hoard_core::Family { key: display, size: size as u64, members }))
}

/// Ids of every asset in the same family as `id` (including `id`), or just `id`.
pub(crate) fn family_ids(conn: &Connection, id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT f2.asset_id FROM asset_family f1 JOIN asset_family f2 ON f2.family_key = f1.family_key WHERE f1.asset_id = ?1",
    )?;
    let ids: Vec<String> = stmt.query_map([id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    Ok(if ids.is_empty() { vec![id.to_owned()] } else { ids })
}

pub(crate) fn pack_context(conn: &Connection, pack_id: &str) -> Result<hoard_core::PackContext> {
    let mut ctx = conn
        .query_row(
            "SELECT store, store_url, store_text, license_tier, readme, seller, store_license,
                    json_extract(store_auto, '$.at')
             FROM pack_context WHERE pack_id = ?1",
            [pack_id],
            |r| {
                Ok(hoard_core::PackContext {
                    store: r.get(0)?,
                    store_url: r.get(1)?,
                    store_text: r.get(2)?,
                    license_tier: r.get(3)?,
                    readme: r.get(4)?,
                    seller: r.get(5)?,
                    store_license: r.get(6)?,
                    store_auto: r.get(7)?,
                    own_licenses: 0,
                })
            },
        )
        .optional()?
        .unwrap_or_default();
    ctx.own_licenses = conn.query_row(
        "SELECT count(*) FROM assets WHERE pack_id = ?1 AND archived_at IS NULL AND NULLIF(trim(license), '') IS NOT NULL",
        [pack_id],
        |r| r.get(0),
    )?;
    Ok(ctx)
}

/// Outgoing and incoming links for one asset.
pub(crate) fn asset_links(conn: &Connection, id: &str) -> Result<(Vec<AssetLink>, Vec<AssetLink>)> {
    let query = |sql: &str| -> Result<Vec<AssetLink>> {
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt
            .query_map([id], |r| {
                Ok(AssetLink {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    path: r.get(2)?,
                    kind: AssetKind::parse(&r.get::<_, String>(3)?).unwrap_or(AssetKind::Other),
                    rel: LinkRel::parse(&r.get::<_, String>(4)?).unwrap_or(LinkRel::Guess),
                    has_thumb: r.get(5)?,
                    size: r.get::<_, i64>(6)? as u64,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    };
    let out = query(
        "SELECT a.id, a.name, a.path, a.kind, l.rel, a.thumb_hash IS NOT NULL, b.size
         FROM asset_links l JOIN assets a ON a.id = l.to_id JOIN blobs b ON b.hash = a.blob_hash WHERE l.from_id = ?1 ORDER BY l.rel, a.path",
    )?;
    let inc = query(
        "SELECT a.id, a.name, a.path, a.kind, l.rel, a.thumb_hash IS NOT NULL, b.size
         FROM asset_links l JOIN assets a ON a.id = l.from_id JOIN blobs b ON b.hash = a.blob_hash WHERE l.to_id = ?1 ORDER BY l.rel, a.path",
    )?;
    Ok((out, inc))
}

/// Everything an asset needs to work, following links transitively (mesh -> material -> textures).
/// The root comes first.
pub(crate) fn dependency_closure(conn: &Connection, root: &str, include_animations: bool) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT to_id, rel FROM asset_links WHERE from_id = ?1")?;
    let mut seen = std::collections::HashSet::from([root.to_owned()]);
    let mut order = vec![root.to_owned()];
    let mut queue = std::collections::VecDeque::from([root.to_owned()]);
    while let Some(id) = queue.pop_front() {
        let next: Vec<(String, String)> =
            stmt.query_map([&id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
        for (to, rel) in next {
            let Some(rel) = LinkRel::parse(&rel) else { continue };
            let wanted = rel.is_dependency() || (include_animations && rel == LinkRel::Animation);
            if wanted && seen.insert(to.clone()) {
                order.push(to.clone());
                queue.push_back(to);
            }
        }
    }
    Ok(order)
}

/// A file to put in a bundle.
#[derive(Debug, Clone)]
pub(crate) struct BundleFile {
    pub(crate) id: String,
    pub(crate) path: String,
    pub(crate) hash: String,
    pub(crate) size: u64,
    /// The file's Unity GUID, when it came with a `.meta`; bundles recreate the `.meta` so Unity keeps references.
    pub(crate) unity_guid: Option<String>,
}

/// Replace files that have a recovered source (`source` derivative) with that source, renamed to its type:
/// `Audio/S_Hit.uasset` becomes `Audio/S_Hit.wav`.
pub(crate) fn prefer_sources(conn: &Connection, files: &mut [BundleFile]) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT d.blob_hash, b.size, json_extract(a.meta, '$.source_ext') FROM derivatives d
         JOIN assets a ON a.id = d.asset_id JOIN blobs b ON b.hash = d.blob_hash
         WHERE d.asset_id = ?1 AND d.kind = 'source'",
    )?;
    for file in files.iter_mut() {
        let found: Option<(String, i64, Option<String>)> =
            stmt.query_row([&file.id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?))).optional()?;
        if let Some((hash, size, Some(ext))) = found {
            let stem = file.path.rsplit_once('.').map_or(file.path.as_str(), |(s, _)| s);
            file.path = format!("{stem}.{ext}");
            file.hash = hash;
            file.size = size as u64;
        }
    }
    Ok(())
}

pub(crate) fn bundle_files(conn: &Connection, ids: &[String]) -> Result<Vec<BundleFile>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.path, a.blob_hash, b.size, u.guid FROM assets a JOIN blobs b ON b.hash = a.blob_hash
         LEFT JOIN unity_guids u ON u.pack_id = a.pack_id AND u.path = a.path
         WHERE a.id = ?1 AND NOT EXISTS (SELECT 1 FROM material_maps m WHERE m.material_id = a.id)",
    )?;
    let mut out = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(f) = stmt
            .query_row([id], |r| {
                Ok(BundleFile {
                    id: r.get(0)?,
                    path: r.get(1)?,
                    hash: r.get(2)?,
                    size: r.get::<_, i64>(3)? as u64,
                    unity_guid: r.get(4)?,
                })
            })
            .optional()?
        {
            out.push(f);
        }
    }
    Ok(out)
}

/// Longest shared folder of a set of `/`-separated paths ("" if none).
pub(crate) fn common_dir(paths: &[&str]) -> String {
    let mut dirs = paths.iter().map(|p| p.rsplit_once('/').map_or("", |(d, _)| d).split('/').collect::<Vec<_>>());
    let Some(first) = dirs.next() else { return String::new() };
    let mut common = first;
    for d in dirs {
        let n = common.iter().zip(&d).take_while(|(a, b)| a == b).count();
        common.truncate(n);
    }
    common.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("/")
}

pub(crate) fn strip_dir<'a>(path: &'a str, dir: &str) -> &'a str {
    if dir.is_empty() { path } else { path.strip_prefix(dir).map_or(path, |p| p.trim_start_matches('/')) }
}

pub(crate) fn derivative(conn: &Connection, id: &str, kind: &str) -> Result<Option<String>> {
    Ok(conn
        .query_row("SELECT blob_hash FROM derivatives WHERE asset_id = ?1 AND kind = ?2", params![id, kind], |r| {
            r.get(0)
        })
        .optional()?)
}

/// Look up a file in a pack by its original relative path (case-insensitive fallback). Returns (hash, ext).
pub(crate) fn pack_file(conn: &Connection, pack_id: &str, path: &str) -> Result<Option<(String, String)>> {
    let row = |sql: &str| conn.query_row(sql, params![pack_id, path], |r| Ok((r.get(0)?, r.get(1)?))).optional();
    Ok(match row("SELECT blob_hash, ext FROM assets WHERE pack_id = ?1 AND path = ?2")? {
        Some(r) => Some(r),
        None => row("SELECT blob_hash, ext FROM assets WHERE pack_id = ?1 AND lower(path) = lower(?2)")?,
    })
}

pub(crate) fn asset_file_info(conn: &Connection, id: &str) -> Result<Option<(String, String, String)>> {
    Ok(conn
        .query_row("SELECT blob_hash, path, ext FROM assets WHERE id = ?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .optional()?)
}

pub(crate) fn asset_thumb(conn: &Connection, id: &str) -> Result<Option<String>> {
    Ok(conn.query_row("SELECT thumb_hash FROM assets WHERE id = ?1", [id], |r| r.get(0)).optional()?.flatten())
}

/// A pack's words in the search index besides its name (`p` being the pack): its tags, as an asset's are, and
/// `unreal` where it has Unreal packages. The packs and their counts keep the rows up to date as they change (step 97
/// of the migrations, which has its own copy of this).
const PACK_TAGS: &str = "concat_ws(' ', (SELECT group_concat(replace(replace(value, ':', ' '), '-', ' '), ' ')
                                    FROM json_each(CASE WHEN json_valid(p.tags) THEN p.tags ELSE '[]' END)),
                              CASE WHEN EXISTS (SELECT 1 FROM pack_kinds k WHERE k.pack_id = p.id AND k.unreal > 0)
                                   THEN 'unreal' END)";

/// Rebuild the FTS row for one asset from its current fields + tags, and its pack's name and tags. The fields of the library's own go in with
/// its description, each as its name and value: `biome desert` is found by either word.
///
/// The row is kept by number in `assets.fts_row`, found and replaced by that: the index can't look rows up by its
/// `asset_id` column, so a query on that reads all of it. Deleting the asset deletes its row (a trigger). Anything that
/// writes the index otherwise has to keep `fts_row` too.
pub(crate) fn reindex_asset(conn: &Connection, id: &str) -> Result<()> {
    let row: Option<i64> =
        conn.query_row("SELECT fts_row FROM assets WHERE id = ?1", [id], |r| r.get(0)).optional()?.flatten();
    if let Some(row) = row {
        conn.execute("DELETE FROM assets_fts WHERE rowid = ?1", [row])?;
    }
    let added = conn.execute(
        &format!(
            "INSERT INTO assets_fts (rowid, asset_id, name, words, category, description, tags, pack_name, pack_tags)
             SELECT ?2, a.id, a.name, a.words, COALESCE(a.category, ''),
                    concat_ws(' ', a.description, a.source_description, a.ai_description,
                              (SELECT group_concat(key || ' ' || value, ' ') FROM asset_fields f WHERE f.asset_id = a.id)),
                    COALESCE((SELECT group_concat(replace(replace(tag, ':', ' '), '-', ' '), ' ')
                              FROM asset_tags t WHERE t.asset_id = a.id), ''),
                    COALESCE(p.name, ''), {PACK_TAGS}
             FROM assets a LEFT JOIN packs p ON p.id = a.pack_id WHERE a.id = ?1"
        ),
        rusqlite::params![id, row],
    )?;
    if added == 1 && row.is_none() {
        conn.execute("UPDATE assets SET fts_row = ?2 WHERE id = ?1", rusqlite::params![id, conn.last_insert_rowid()])?;
    }
    Ok(())
}
