//! Files a store ships beside the art that say what it is. A Megascans asset (from Fab or Bridge) carries
//! `<id>.json` with the listing's exact title, tags, colour, where it was scanned and its real-world size, while its
//! files are named `Historic_Miscellaneous_Stand_Wood_Old_05_xeokah0_Raw…`.
//!
//! [`read`] understands one such file; [`apply`] reads a pack's and names, tags and describes the pack from them.
//! The store search reads them too ([`pack_sidecars`]), for the exact title.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use anyhow::Result;
use hoard_core::{AssetKind, TagSource};
use rusqlite::{OptionalExtension as _, params};
use serde_json::Value;

use crate::tagger::{self, TagSet};
use crate::{AppState, db};

/// Larger JSON files are data, not a description of the pack.
pub(crate) const SIDECAR_BYTES: i64 = 256 * 1024;
/// How many JSON files of a pack are read at most.
const MAX_SIDECARS: i64 = 50;
/// Tags taken from one file at most.
const MAX_TAGS: usize = 24;

/// What a store's file says about the asset beside it.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Sidecar {
    /// The file's path in the pack.
    pub(crate) path: String,
    /// The store's id for the asset (`xeokah0`), which its other files' names carry.
    pub(crate) id: Option<String>,
    /// The listing's exact title.
    pub(crate) title: String,
    /// The kind of file, in words ("Megascans file").
    pub(crate) source: &'static str,
    /// Who sells it.
    pub(crate) seller: &'static str,
    /// "3D asset", "Surface", "Decal"…
    pub(crate) asset_type: Option<String>,
    /// Plain, normalised tags (`table`, `wood`, `color:brown`, `megascans`).
    pub(crate) tags: Vec<String>,
    /// Where it was scanned ("Denmark").
    pub(crate) origin: Option<String>,
    /// Real-world measurements as written ("length", "120 cm").
    pub(crate) dims: Vec<(String, String)>,
    /// The size range it's meant for, as written ("1-2 m").
    pub(crate) size: Option<String>,
    pub(crate) texel_density: Option<String>,
}

/// Strings from a value that may be a string, a comma-separated string or an array of either.
fn strings(v: &Value) -> Vec<String> {
    match v {
        Value::String(s) => s.split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned).collect(),
        Value::Array(a) => a.iter().flat_map(strings).collect(),
        Value::Number(n) => vec![n.to_string()],
        _ => Vec::new(),
    }
}

fn first_string(v: &Value) -> Option<String> {
    strings(v).into_iter().next()
}

fn tidy(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Understand a JSON file from a pack, if it's one a store ships to describe an asset.
pub(crate) fn read(path: &str, json: &str) -> Option<Sidecar> {
    let v: Value = serde_json::from_str(json).ok()?;
    let sem = &v["semanticTags"];
    // Today's files name the listing under `semanticTags`; older ones have `id`, `name` and a `meta` list.
    let title = first_string(&sem["name_formats"]["title"])
        .or_else(|| first_string(&sem["name"]))
        .or_else(|| (v["id"].is_string() && v["meta"].is_array()).then(|| first_string(&v["name"])).flatten())
        .map(|t| tidy(&t))
        .filter(|t| !t.is_empty() && t.len() <= 120)?;
    let file_stem = path.rsplit('/').next().unwrap_or(path).rsplit_once('.').map_or(path, |(s, _)| s);
    let id = v["id"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| (file_stem.len() <= 16 && !file_stem.contains(' ')).then(|| file_stem.to_owned()))
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'));

    let mut words: Vec<String> = strings(&v["tags"]);
    words.extend(strings(&v["categories"]));
    let mut colors: Vec<String> = strings(&sem["color"]);
    if let Some(Value::Object(o)) = sem.as_object().map(|_| sem) {
        for (key, value) in o {
            let skip = matches!(key.as_str(), "name" | "name_formats" | "color" | "country" | "region" | "asset_type")
                || key.contains("size");
            // Lists of words (`contains`, `theme`, `descriptive`…) are tags; single values are facts read below.
            if !skip && value.is_array() {
                words.extend(strings(value));
            }
        }
    }
    let asset_type = first_string(&sem["asset_type"]).or_else(|| first_string(&v["type"])).map(|t| tidy(&t));
    if let Some(t) = &asset_type {
        words.push(t.clone());
    }
    if colors.is_empty()
        && let Some(c) = v["averageColor"].as_str().filter(|c| !c.starts_with('#'))
    {
        colors.push(c.to_owned());
    }

    let mut tags: Vec<String> = Vec::new();
    let mut push = |t: Option<String>| {
        if let Some(t) = t.filter(|t| t.len() <= 40 && !t.chars().all(|c| c.is_ascii_digit() || c == '-'))
            && !tags.contains(&t)
        {
            tags.push(t);
        }
    };
    push(Some("megascans".to_owned()));
    for w in &words {
        push(hoard_core::normalize_tag(&w.replace(':', " ")));
    }
    for c in &colors {
        push(hoard_core::normalize_tag(&format!("color:{c}")));
    }
    tags.truncate(MAX_TAGS);

    let mut places: Vec<String> = Vec::new();
    for p in [&sem["country"], &sem["region"], &v["environment"]["region"]].into_iter().filter_map(first_string) {
        if !places.iter().any(|q| q.eq_ignore_ascii_case(&p)) {
            places.push(tidy(&p));
        }
    }

    // `meta` is a list of `{key, name, value}`.
    let mut dims = Vec::new();
    let mut texel_density = None;
    let mut scan_area = None;
    for m in v["meta"].as_array().into_iter().flatten() {
        let key = m["key"].as_str().unwrap_or_default().to_ascii_lowercase();
        let Some(value) = first_string(&m["value"]).map(|s| tidy(&s)).filter(|s| !s.is_empty() && s.len() <= 40) else {
            continue;
        };
        match key.as_str() {
            "length" | "width" | "height" | "depth" => dims.push((key.clone(), value)),
            "texeldensity" => texel_density = Some(value),
            "scanarea" => scan_area = Some(value),
            _ => {}
        }
    }
    let size_of = |want: &str| {
        sem.as_object()?
            .iter()
            .find(|(k, _)| {
                let k = k.to_ascii_lowercase();
                k.contains("size") && k.contains(want)
            })
            .and_then(|(_, v)| v.as_str().map(tidy))
    };
    let size = match (size_of("min"), size_of("max")) {
        (Some(a), Some(b)) if a == b => Some(a),
        (Some(a), Some(b)) => Some(format!("{a} to {b}")),
        (a, b) => a.or(b).or(scan_area),
    };

    Some(Sidecar {
        path: path.to_owned(),
        id,
        title,
        source: "Megascans file",
        seller: "Quixel Megascans",
        asset_type,
        tags,
        origin: (!places.is_empty()).then(|| places.join(", ")),
        dims,
        size,
        texel_density,
    })
}

fn capitalised(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

/// The kind of file the asset type describes, for reading its tags as the tagger would.
fn kind_of(asset_type: Option<&str>) -> AssetKind {
    let t = asset_type.unwrap_or_default().to_ascii_lowercase();
    if ["surface", "decal", "atlas", "imperfection", "brush"].iter().any(|k| t.contains(k)) {
        AssetKind::Texture
    } else {
        AssetKind::Model
    }
}

impl Sidecar {
    /// Its tags, with the tagger's words for them where it has one (`wood` → `material:wood`).
    pub(crate) fn all_tags(&self) -> Vec<String> {
        let mut out = self.tags.clone();
        let kind = kind_of(self.asset_type.as_deref());
        for t in &self.tags {
            let mut set = TagSet::default();
            tagger::vocab_tags(&t.replace(['-', ':'], "_"), kind, &mut set);
            for s in set.into_vec() {
                if !out.contains(&s.tag) {
                    out.push(s.tag);
                }
            }
        }
        out
    }

    /// Whether a file of the pack belongs to this asset: next to the JSON, and named with the asset's id when its
    /// neighbours are (several assets can share a folder).
    fn owns(&self, path: &str, ids_in_folder: bool) -> bool {
        let dir = self.path.rsplit_once('/').map_or("", |(d, _)| d);
        let inside = if dir.is_empty() { true } else { path.starts_with(&format!("{dir}/")) };
        if !inside || !ids_in_folder {
            return inside;
        }
        let file = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
        self.id.as_deref().is_some_and(|id| file.contains(&id.to_ascii_lowercase())) || path == self.path
    }

    /// A short description from the facts: what it is, where it's from, its size and maps.
    pub(crate) fn describe(&self, maps: &[&str]) -> String {
        let kind = self.asset_type.as_deref().unwrap_or("asset");
        let lead = if kind.starts_with(['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u']) { "An" } else { "A" };
        let mut out = format!("{lead} {kind} from {}", self.seller);
        if let Some(o) = &self.origin {
            let _ = write!(out, ", scanned in {o}");
        }
        out.push('.');
        let mut facts: Vec<String> = Vec::new();
        if !self.dims.is_empty() {
            facts.push(self.dims.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
        } else if let Some(s) = &self.size {
            facts.push(format!("Size {s}"));
        }
        if let Some(t) = &self.texel_density {
            facts.push(format!("texel density {t}"));
        }
        if !facts.is_empty() {
            let text = facts.join("; ");
            out.push(' ');
            out.push_str(&capitalised(&text));
            out.push('.');
        }
        if !maps.is_empty() {
            let _ = write!(out, " Maps: {}.", maps.join(", "));
        }
        out
    }
}

/// A map's name in words.
fn map_words(map: &str) -> &str {
    match map {
        "albedo" => "base color",
        "ao" => "ambient occlusion",
        "height" => "displacement",
        "orm" => "ORM",
        other => other,
    }
}

/// The order maps are listed in.
const MAP_ORDER: &[&str] =
    &["albedo", "normal", "roughness", "metallic", "specular", "ao", "height", "opacity", "emissive", "orm"];

/// Is the pack's name one nobody typed: the upload's folder or file name, or the asset's file-name form
/// (`historic_miscellaneous_stand_wood_old_05_xeokah0_raw`)?
pub(crate) fn name_is_automatic(name: &str, sidecar: &Sidecar, folder: Option<&str>) -> bool {
    let words = |s: &str| -> Vec<String> {
        s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
    };
    let have = words(name);
    if have.is_empty() {
        return true;
    }
    // The upload's fallback name: "Pack 2026-11-12".
    if have.len() == 4
        && have[0] == "pack"
        && have[1].len() == 4
        && have[1..].iter().all(|w| w.chars().all(|c| c.is_ascii_digit()))
    {
        return true;
    }
    let id = sidecar.id.as_deref().map(str::to_lowercase);
    if id.as_ref().is_some_and(|id| have.contains(id)) {
        return true;
    }
    let bare: Vec<&String> = have.iter().filter(|w| w.as_str() != "raw").collect();
    let title = words(&sidecar.title);
    if have.iter().any(|w| w == "raw") && bare.iter().map(|w| w.as_str()).eq(title.iter().map(String::as_str)) {
        return true;
    }
    let json_dir = sidecar.path.rsplit_once('/').map(|(d, _)| d.rsplit('/').next().unwrap_or(d));
    [json_dir, folder].into_iter().flatten().any(|f| words(f) == have)
}

/// The store files in a pack (small JSON files only are looked at).
pub(crate) async fn pack_sidecars(state: &AppState, pack_id: &str) -> Result<Vec<Sidecar>> {
    let pid = pack_id.to_owned();
    let files = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare(
            "SELECT a.path, a.blob_hash FROM assets a JOIN blobs b ON b.hash = a.blob_hash
             WHERE a.pack_id = ?1 AND lower(a.path) LIKE '%.json' AND b.size <= ?2 ORDER BY a.path LIMIT ?3",
        )?;
        let rows = stmt
            .query_map(params![pid, SIDECAR_BYTES, MAX_SIDECARS], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            })?
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
                // A colour is the model's, not its normal map's.
                if t.starts_with("color:") && kind != AssetKind::Model.as_str() {
                    continue;
                }
                db::upsert_tag(&tx, asset, t, TagSource::Meta, 0.9)?;
            }
            touched.insert(asset.clone());
        }
        name_and_link(&tx, s, &mine)?;
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
        format!(
            "{} Megascans assets: {shown}{}.",
            sidecars.len(),
            if more > 0 { format!(" and {more} more") } else { String::new() }
        )
    };
    applied.described = description.is_none();
    tx.execute(
        "UPDATE packs SET name = COALESCE(?2, name), description = COALESCE(description, ?3), tags = ?4, file_tags = ?5
         WHERE id = ?1",
        params![
            pack_id,
            applied.renamed,
            new_description,
            serde_json::to_string(&pack_tags)?,
            serde_json::to_string(&file_tags)?
        ],
    )?;
    let seller = sidecars[0].seller;
    tx.execute(
        "INSERT INTO pack_context (pack_id, seller) VALUES (?1, ?2)
         ON CONFLICT (pack_id) DO UPDATE SET seller = COALESCE(seller, excluded.seller)",
        params![pack_id, seller],
    )?;
    tx.commit()?;
    Ok(applied)
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
        tx.execute("UPDATE assets SET name = ?2 WHERE id = ?1 AND NOT human_edited AND name <> ?2", params![id, name])?;
    }
    let Some((json, ..)) = mine.iter().find(|(_, p, _)| *p == s.path) else { return Ok(()) };
    for (id, _, kind) in mine {
        if kind == AssetKind::Model.as_str() || kind == AssetKind::Material.as_str() {
            tx.execute(
                "INSERT INTO asset_links (from_id, to_id, rel) VALUES (?1, ?2, 'sidecar') ON CONFLICT DO NOTHING",
                params![id, json],
            )?;
        }
    }
    Ok(())
}

/// The level of detail a model's file name says (`…_LOD2.fbx` → 2).
fn lod_of(path: &str) -> Option<u32> {
    let file = path.rsplit('/').next().unwrap_or(path);
    let stem = file.rsplit_once('.').map_or(file, |(st, _)| st).to_ascii_lowercase();
    stem.split(|c: char| !c.is_ascii_alphanumeric()).find_map(|w| w.strip_prefix("lod")?.parse().ok())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Shaped like a Megascans download's `<id>.json`.
    pub(crate) const STAND: &str = r#"{
        "id": "xeokah0",
        "semanticTags": {
            "name": "Historic Miscellaneous Stand Wood Old 05",
            "name_formats": { "title": "Historic Miscellaneous Stand Wood Old 05", "slug": "historic_miscellaneous_stand_wood_old_05" },
            "contains": ["table"],
            "color": ["brown"],
            "country": "Denmark",
            "region": "Europe",
            "asset_type": "3D asset",
            "3d_mesh": true,
            "minSize": "1 m",
            "maxSize": "2 m",
            "theme": ["historic", "medieval"]
        },
        "meta": [
            { "name": "Texel Density", "key": "texelDensity", "value": "4096 px/m" },
            { "name": "Length", "key": "length", "value": "118 cm" },
            { "name": "Width", "key": "width", "value": "54 cm" },
            { "name": "Height", "key": "height", "value": "81 cm" },
            { "name": "Tris", "key": "tris", "value": 12000 }
        ],
        "tags": ["wood", "old", "stand", "furniture", "rustic"]
    }"#;

    #[test]
    fn reads_a_megascans_file() {
        let s = read("xeokah0.json", STAND).unwrap();
        assert_eq!(s.title, "Historic Miscellaneous Stand Wood Old 05");
        assert_eq!(s.id.as_deref(), Some("xeokah0"));
        assert_eq!(s.asset_type.as_deref(), Some("3D asset"));
        assert_eq!(s.origin.as_deref(), Some("Denmark, Europe"));
        assert_eq!(s.texel_density.as_deref(), Some("4096 px/m"));
        assert_eq!(s.size.as_deref(), Some("1 m to 2 m"));
        for t in ["megascans", "wood", "table", "historic", "3d-asset", "color:brown"] {
            assert!(s.tags.contains(&t.to_owned()), "{t} in {:?}", s.tags);
        }
        assert!(s.all_tags().contains(&"material:wood".to_owned()));
        let d = s.describe(&["base color", "normal", "roughness"]);
        assert_eq!(
            d,
            "A 3D asset from Quixel Megascans, scanned in Denmark, Europe. Length 118 cm, width 54 cm, height 81 cm; \
             texel density 4096 px/m. Maps: base color, normal, roughness."
        );
        assert!(read("x.json", r#"{"name": "a package.json"}"#).is_none());
        assert!(read("x.json", "not json").is_none());
    }

    #[test]
    fn reads_the_older_shape() {
        let s = read(
            "Rock/abc123.json",
            r#"{"id":"abc123","name":"Mossy Rock","meta":[{"key":"scanArea","value":"2x2 m"}],"categories":["3d","rock"],
               "tags":"mossy, nordic","environment":{"region":"Iceland"}}"#,
        )
        .unwrap();
        assert_eq!(s.title, "Mossy Rock");
        assert_eq!(s.size.as_deref(), Some("2x2 m"));
        assert_eq!(s.origin.as_deref(), Some("Iceland"));
        assert!(s.tags.contains(&"mossy".to_owned()) && s.tags.contains(&"rock".to_owned()));
        assert!(s.describe(&[]).starts_with("An asset from Quixel Megascans, scanned in Iceland. Size 2x2 m."));
    }

    #[test]
    fn finds_the_lod_in_a_file_name() {
        assert_eq!(lod_of("Stand_xeokah0/Stand_xeokah0_LOD2.fbx"), Some(2));
        assert_eq!(lod_of("a/stand_lod0.obj"), Some(0));
        assert_eq!(lod_of("a/Stand_High.fbx"), None);
        assert_eq!(lod_of("a/lodge.obj"), None);
    }

    #[test]
    fn knows_a_name_nobody_typed() {
        let s = read("xeokah0.json", STAND).unwrap();
        assert!(name_is_automatic("historic_miscellaneous_stand_wood_old_05_xeokah0_raw", &s, None));
        assert!(name_is_automatic("Historic_Miscellaneous_Stand_Wood_Old_05_Raw", &s, None));
        assert!(name_is_automatic("Pack 2026-11-12", &s, None));
        assert!(name_is_automatic("My Download", &s, Some("my_download")));
        assert!(!name_is_automatic("Market stall table", &s, Some("xeokah0")));
        assert!(!name_is_automatic("Historic stand", &s, None));
    }
}
