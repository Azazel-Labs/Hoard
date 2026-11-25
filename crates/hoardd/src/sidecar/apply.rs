//! A pack's store files put to use: read from the library, they name, tag and describe the pack and its assets.

use std::collections::BTreeSet;

use anyhow::Result;
use hoard_core::{AssetKind, TagSource};
use rusqlite::{OptionalExtension as _, params, params_from_iter};

use super::{MAP_ORDER, MAX_SIDECARS, SIDECAR_BYTES, Sidecar, lod_of, map_words, name_is_automatic, path_filter, read};
use crate::tagger;
use crate::{AppState, db};

/// The store files in a pack (small files any format reads).
pub(crate) async fn pack_sidecars(state: &AppState, pack_id: &str) -> Result<Vec<Sidecar>> {
    let pid = pack_id.to_owned();
    let files = db::call(&state.pool, move |c| {
        let (filter, patterns) = path_filter();
        let mut stmt = c.prepare(&format!(
            "SELECT a.path, a.blob_hash FROM assets a JOIN blobs b ON b.hash = a.blob_hash
             WHERE a.pack_id = ? AND b.size <= ? AND {filter} ORDER BY a.path LIMIT ?"
        ))?;
        let mut args: Vec<rusqlite::types::Value> = vec![pid.into(), SIDECAR_BYTES.into()];
        args.extend(patterns.into_iter().map(Into::into));
        args.push(MAX_SIDECARS.into());
        let rows = stmt
            .query_map(params_from_iter(args), |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    let mut out = Vec::new();
    for (path, hash) in files {
        if let Ok(bytes) = state.blobs.read_all(&hash).await
            && let Some(s) = read(&path, &String::from_utf8_lossy(&bytes))
        {
            out.push(s);
        }
    }
    Ok(out)
}

/// What [`apply`] did.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Applied {
    pub(crate) files: usize,
    pub(crate) renamed: Option<String>,
    pub(crate) described: bool,
}

/// Name, tag and describe a pack from its store files. The name changes only when nobody typed it (`auto_name`, or
/// see [`name_is_automatic`]; `folder` is the upload's folder name). Tags go on the pack and on the files each
/// JSON describes; the description is written only where the pack has none.
pub(crate) async fn apply(state: &AppState, pack_id: &str, auto_name: bool, folder: Option<String>) -> Result<Applied> {
    let sidecars = pack_sidecars(state, pack_id).await?;
    let pid = pack_id.to_owned();
    db::call(&state.pool, move |c| write(c, &pid, &sidecars, auto_name, folder.as_deref())).await
}

fn write(
    c: &mut rusqlite::Connection,
    pack_id: &str,
    sidecars: &[Sidecar],
    auto_name: bool,
    folder: Option<&str>,
) -> Result<Applied> {
    let now = db::now();
    let tx = c.transaction()?;
    let Some((name, description, tags, old_file_tags)) = tx
        .query_row("SELECT name, description, tags, file_tags FROM packs WHERE id = ?1", [pack_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, Option<String>>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
            ))
        })
        .optional()?
    else {
        return Ok(Applied::default());
    };
    tx.execute("UPDATE packs SET files_read_at = ?2 WHERE id = ?1", params![pack_id, now])?;
    if sidecars.is_empty() {
        tx.commit()?;
        return Ok(Applied::default());
    }
    let assets: Vec<(String, String, String)> = {
        let mut stmt = tx.prepare("SELECT id, path, kind FROM assets WHERE pack_id = ?1")?;
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect::<rusqlite::Result<_>>()?
    };
    let alias = |t: String| -> rusqlite::Result<String> {
        Ok(tx.query_row("SELECT into_tag FROM tag_aliases WHERE tag = ?1", [&t], |r| r.get(0)).optional()?.unwrap_or(t))
    };

    let mut file_tags: Vec<String> = Vec::new();
    let mut descriptions = Vec::new();
    let mut touched = BTreeSet::new();
    for s in sidecars {
        let dir = s.path.rsplit_once('/').map_or("", |(d, _)| d);
        let id = s.id.as_deref().map(str::to_ascii_lowercase);
        // Several assets in one folder: each JSON speaks for the files named with its id.
        let ids_in_folder = id.as_ref().is_some_and(|id| {
            assets.iter().any(|(_, p, _)| {
                p.rsplit_once('/').map_or("", |(d, _)| d) == dir
                    && *p != s.path
                    && p.rsplit('/').next().unwrap_or(p).to_ascii_lowercase().contains(id.as_str())
            })
        });
        let mine: Vec<&(String, String, String)> = assets.iter().filter(|(_, p, _)| s.owns(p, ids_in_folder)).collect();
        let mut maps: Vec<&str> = mine
            .iter()
            .filter(|(_, _, k)| k == AssetKind::Texture.as_str())
            .filter_map(|(_, p, _)| {
                let file = p.rsplit('/').next().unwrap_or(p);
                tagger::map_suffix(file.rsplit_once('.').map_or(file, |(st, _)| st)).map(|h| h.map)
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        maps.sort_by_key(|m| MAP_ORDER.iter().position(|o| o == m).unwrap_or(MAP_ORDER.len()));
        let words: Vec<&str> = maps.iter().map(|m| map_words(m)).collect();
        descriptions.push(s.describe(&words));
        let tags: Vec<String> = s.all_tags().into_iter().map(&alias).collect::<rusqlite::Result<_>>()?;
        for (asset, _, kind) in &mine {
            for t in &tags {
                // A color is the model's, not its normal map's.
                if t.starts_with("color:") && kind != AssetKind::Model.as_str() {
                    continue;
                }
                db::upsert_tag(&tx, asset, t, TagSource::Meta, 0.9)?;
            }
            touched.insert(asset.clone());
        }
        if !s.whole_pack {
            name_and_link(&tx, s, &mine)?;
        }
        // A model whose file couldn't be read for its triangles takes the count the store gives.
        for (asset, path, kind) in &mine {
            if kind == AssetKind::Model.as_str()
                && let Some(tris) = s.lod_tris(path)
            {
                tx.prepare_cached(
                    "UPDATE assets SET meta = json_set(COALESCE(meta, '{}'), '$.triangles', ?2)
                     WHERE id = ?1 AND json_extract(meta, '$.triangles') IS NULL",
                )?
                .execute(params![asset, i64::try_from(tris).unwrap_or(i64::MAX)])?;
            }
        }
        for t in tags {
            if !file_tags.contains(&t) {
                file_tags.push(t);
            }
        }
    }
    for a in &touched {
        db::reindex_asset(&tx, a)?;
    }

    // Pack tags: what the files said last time makes way for what they say now; other tags stay.
    let old: Vec<String> = old_file_tags.and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    let mut pack_tags: Vec<String> = serde_json::from_str::<Vec<String>>(&tags)
        .unwrap_or_default()
        .into_iter()
        .filter(|t| !old.contains(t))
        .collect();
    for t in &file_tags {
        if !pack_tags.contains(t) {
            pack_tags.push(t.clone());
        }
    }

    let titles: BTreeSet<&str> = sidecars.iter().map(|s| s.title.as_str()).collect();
    let mut applied = Applied { files: sidecars.len(), ..Applied::default() };
    if let [only] = sidecars
        && titles.len() == 1
        && name != only.title
        && (auto_name || name_is_automatic(&name, only, folder))
    {
        applied.renamed = Some(only.title.clone());
    }
    let new_description = if let [one] = descriptions.as_slice() {
        one.clone()
    } else {
        let names: Vec<&str> = titles.iter().copied().collect();
        let shown = names.iter().take(4).copied().collect::<Vec<_>>().join(", ");
        let more = names.len().saturating_sub(4);
        // Only so many files are read: a bundle of hundreds of kits has more than were counted.
        let capped = i64::try_from(sidecars.len()).is_ok_and(|n| n >= MAX_SIDECARS);
        let formats: BTreeSet<&str> = sidecars.iter().map(|s| s.format.as_str()).collect();
        let whose = match formats.into_iter().collect::<Vec<_>>().as_slice() {
            [one] => format!("{one} "),
            _ => String::new(),
        };
        let things = if sidecars.iter().all(|s| s.whole_pack) { "packs" } else { "assets" };
        format!(
            "{}{} {whose}{things}: {shown}{}.",
            sidecars.len(),
            if capped { "+" } else { "" },
            if capped {
                " and more".to_owned()
            } else if more > 0 {
                format!(" and {more} more")
            } else {
                String::new()
            }
        )
    };
    applied.described = description.is_none();
    tx.prepare_cached(
        "UPDATE packs SET name = COALESCE(?2, name), description = COALESCE(description, ?3), tags = ?4, file_tags = ?5
         WHERE id = ?1",
    )?
    .execute(params![
        pack_id,
        applied.renamed,
        new_description,
        serde_json::to_string(&pack_tags)?,
        serde_json::to_string(&file_tags)?
    ])?;
    credit(&tx, pack_id, sidecars)?;
    tx.commit()?;
    Ok(applied)
}

/// Who made the pack, and the license the credits give it when every file that names one names the same one, and it's
/// one they know. Neither replaces what's there.
fn credit(tx: &rusqlite::Connection, pack_id: &str, sidecars: &[Sidecar]) -> Result<()> {
    if let Some(seller) = sidecars.iter().find_map(|s| s.seller.as_deref()) {
        tx.prepare_cached(
            "INSERT INTO pack_context (pack_id, seller) VALUES (?1, ?2)
             ON CONFLICT (pack_id) DO UPDATE SET seller = COALESCE(seller, excluded.seller)",
        )?
        .execute(params![pack_id, seller])?;
    }
    let licenses: BTreeSet<&str> = sidecars.iter().filter_map(|s| s.license.as_deref()).collect();
    if let [license] = licenses.into_iter().collect::<Vec<_>>().as_slice()
        && crate::credits::license_name(license).is_some()
    {
        tx.prepare_cached(
            "INSERT INTO pack_context (pack_id, store_license) VALUES (?1, ?2)
             ON CONFLICT (pack_id) DO UPDATE SET store_license = COALESCE(store_license, excluded.store_license)",
        )?
        .execute(params![pack_id, license])?;
    }
    Ok(())
}

/// The model a store's file describes takes the listing's title (each LOD with its number), unless someone renamed
/// it; and the file goes behind the model or material it describes rather than showing as a tile of its own.
fn name_and_link(tx: &rusqlite::Connection, s: &Sidecar, mine: &[&(String, String, String)]) -> Result<()> {
    let models: Vec<&(String, String, String)> =
        mine.iter().copied().filter(|(_, _, k)| k == AssetKind::Model.as_str()).collect();
    for (id, path, _) in &models {
        let name = if models.len() == 1 {
            s.title.clone()
        } else if let Some(lod) = lod_of(path) {
            format!("{} LOD{lod}", s.title)
        } else {
            continue;
        };
        tx.prepare_cached("UPDATE assets SET name = ?2 WHERE id = ?1 AND NOT human_edited AND name <> ?2")?
            .execute(params![id, name])?;
    }
    let Some((json, ..)) = mine.iter().find(|(_, p, _)| *p == s.path) else { return Ok(()) };
    for (id, _, kind) in mine {
        if kind == AssetKind::Model.as_str() || kind == AssetKind::Material.as_str() {
            tx.prepare_cached(
                "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'sidecar') ON CONFLICT DO NOTHING",
            )?
            .execute(params![id, json])?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bundle_of_kits_is_credited_to_their_maker_and_its_models_keep_their_names() {
        let mut conn = db::test_library();
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO packs (id, name, status, created_at) VALUES ('p', 'Kenney All-in-1', 'ready', 'x');
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, created_at, updated_at) VALUES
               ('ship', 'p', 'h', 'Pirate Kit/Models/ship.glb', 'Ship', 'glb', 'model', 'x', 'x'),
               ('lic1', 'p', 'h', 'Pirate Kit/License.txt', 'License', 'txt', 'document', 'x', 'x'),
               ('car', 'p', 'h', 'Car Kit/Models/car.glb', 'Car', 'glb', 'model', 'x', 'x'),
               ('lic2', 'p', 'h', 'Car Kit/License.txt', 'License', 'txt', 'document', 'x', 'x');",
        )
        .unwrap();
        let license = |kit: &str| {
            format!(
                "\t\n\n\t{kit} (1.2)\n\n\tCreated/distributed by Kenney (www.kenney.nl)\n\n\t\t\t----------\n\n\
                 \tLicense: (Creative Commons Zero, CC0)\n\thttp://creativecommons.org/publicdomain/zero/1.0/\n"
            )
        };
        let sidecars: Vec<Sidecar> = [("Pirate Kit", "Pirate Kit/License.txt"), ("Car Kit", "Car Kit/License.txt")]
            .into_iter()
            .map(|(kit, path)| read(path, &license(kit)).expect("read as Kenney's"))
            .collect();
        let applied = write(&mut conn, "p", &sidecars, false, None).unwrap();
        assert_eq!(applied.renamed, None);

        let names: Vec<String> = conn
            .prepare("SELECT name FROM assets WHERE kind = 'model' ORDER BY id")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(names, ["Car", "Ship"]);
        let (description, seller, license): (String, String, String) = conn
            .query_row(
                "SELECT p.description, c.seller, c.store_license FROM packs p JOIN pack_context c ON c.pack_id = p.id",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert_eq!(description, "2 Kenney packs: Car Kit, Pirate Kit.");
        assert_eq!((seller.as_str(), license.as_str()), ("Kenney", "CC0"));
    }
}
