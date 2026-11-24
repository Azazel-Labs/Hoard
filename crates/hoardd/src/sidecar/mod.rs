//! Files a store ships beside the art that say what it is. A Megascans asset (from Fab or Bridge) carries
//! `<id>.json` with the listing's exact title, tags, color, where it was scanned and its real-world size, while its
//! files are named `Historic_Miscellaneous_Stand_Wood_Old_05_xeokah0_Raw…`. It also lists the asset's texture maps by
//! kind, its levels of detail with their triangle counts and, for a surface, how much ground one repeat covers. A
//! Sketchfab download's `license.txt` names the model, its page, its author and its license.
//!
//! Each store's file is described in TOML ([`crate::store_formats`]); this module makes sense of what they find.
//! [`read`] understands one such file; [`apply`] reads a pack's and names, tags and describes the pack from them.
//! The store search reads them too ([`pack_sidecars`]), for the exact title and the store page.

use std::fmt::Write as _;

use hoard_core::AssetKind;
use serde::Serialize;

use crate::store_formats::{self, Format};
use crate::tagger::{self, TagSet};

mod apply;

pub(crate) use apply::{apply, pack_sidecars};

/// Larger files are data, not a description of the pack.
pub(crate) const SIDECAR_BYTES: i64 = 256 * 1024;
/// How many store files of a pack are read at most.
const MAX_SIDECARS: i64 = 50;
/// Tags taken from one file at most.
const MAX_TAGS: usize = 24;

/// What a store's file says about the asset beside it.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub(crate) struct Sidecar {
    /// The file's path in the pack.
    pub(crate) path: String,
    /// The store's id for the asset (`xeokah0`), which its other files' names carry.
    pub(crate) id: Option<String>,
    /// The listing's exact title.
    pub(crate) title: String,
    /// The format that read it ("Megascans").
    pub(crate) format: String,
    /// The kind of file, in words ("Megascans file").
    pub(crate) source: String,
    /// It speaks for the whole pack, not one asset: no model takes its title.
    pub(crate) whole_pack: bool,
    /// Who sells it, or made it.
    pub(crate) seller: Option<String>,
    /// The license it's sold under, as the file writes it ("CC-BY-4.0").
    pub(crate) license: Option<String>,
    /// The store page.
    pub(crate) url: Option<String>,
    /// How the description says where it's from ("scanned in").
    #[serde(skip)]
    pub(crate) origin_words: String,
    /// "3D asset", "Surface", "Decal"…
    pub(crate) asset_type: Option<String>,
    /// Plain, normalized tags (`table`, `wood`, `color:brown`, `megascans`).
    pub(crate) tags: Vec<String>,
    /// Where it was scanned ("Denmark").
    pub(crate) origin: Option<String>,
    /// Real-world measurements as written ("length", "120 cm").
    pub(crate) dims: Vec<(String, String)>,
    /// The size range it's meant for, as written ("1-2 m").
    pub(crate) size: Option<String>,
    pub(crate) texel_density: Option<String>,
    /// The texture maps it lists.
    pub(crate) maps: Vec<ListedMap>,
    /// Its levels of detail, lowest number first.
    pub(crate) lods: Vec<Lod>,
    /// For a tiling surface: width and height of one repeat, in meters.
    pub(crate) physical_size: Option<(f32, f32)>,
    /// For a tiling surface: how far its height map reaches, in meters.
    pub(crate) displacement: Option<f32>,
}

/// A texture map the file lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ListedMap {
    /// Its file name, lowercase and without folders.
    pub(crate) file: String,
    /// What it's for, in the store's word, lowercase (`albedo`, `normal`, `displacement`).
    pub(crate) kind: String,
}

/// A level of detail the file lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Lod {
    pub(crate) number: u32,
    pub(crate) tris: u64,
    /// Its mesh files' names, lowercase and without folders.
    pub(crate) files: Vec<String>,
}

fn tidy(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A URI's file name, lowercase.
fn file_name(uri: &str) -> Option<String> {
    let f = uri.rsplit(['/', '\\']).next()?.trim().to_ascii_lowercase();
    (!f.is_empty()).then_some(f)
}

/// A length's unit in meters; `None` for one that isn't a length.
fn unit_meters(unit: &str) -> Option<f32> {
    match unit {
        "m" | "meter" | "meters" | "metre" | "metres" => Some(1.0),
        "dm" => Some(0.1),
        "cm" => Some(0.01),
        "mm" => Some(0.001),
        _ => None,
    }
}

/// A number and the unit after it (`"2cm"`, `"2 x 2 m"` → `("2 x 2", "m")`).
fn split_unit(s: &str) -> (&str, &str) {
    let s = s.trim();
    let at = s.rfind(|c: char| c.is_ascii_digit() || c == '.').map_or(0, |i| i + 1);
    (s[..at].trim(), s[at..].trim())
}

/// A length with its unit, in meters: `"2cm"` → 0.02.
fn meters(s: &str) -> Option<f32> {
    let (n, unit) = split_unit(s);
    let v: f32 = n.parse().ok()?;
    (v > 0.0).then_some(v * unit_meters(&unit.to_ascii_lowercase())?)
}

/// An area's width and height in meters: `"2x2 m"`, `"200 x 200 cm"`, `"0.91x0.91"`, `"40x40"`. Without a unit,
/// numbers up to 20 are meters and larger ones decimeters: Quixel's API writes meters and its file spec decimeters.
fn area_meters(s: &str) -> Option<(f32, f32)> {
    let (n, unit) = split_unit(s);
    let parts: Vec<f32> = n.split(['x', 'X', '×', '*']).map(|p| p.trim().parse().ok()).collect::<Option<_>>()?;
    let (w, h) = match parts.as_slice() {
        [w, h] => (*w, *h),
        [s] => (*s, *s),
        _ => return None,
    };
    if !(w > 0.0 && h > 0.0) {
        return None;
    }
    let scale = if unit.is_empty() {
        if w.max(h) > 20.0 { 0.1 } else { 1.0 }
    } else {
        unit_meters(&unit.to_ascii_lowercase())?
    };
    Some((w * scale, h * scale))
}

/// Understand a file from a pack, if it's one a store ships to describe an asset.
pub(crate) fn read(path: &str, text: &str) -> Option<Sidecar> {
    store_formats::all().iter().filter(|f| f.wants(path)).find_map(|f| read_as(f, path, text))
}

/// Values that may be comma-separated lists, one value each.
fn split_commas(values: &[String]) -> Vec<String> {
    values.iter().flat_map(|s| s.split(',')).map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned).collect()
}

/// Understand a file as this format reads it: `None` when it isn't the store's (see [`Format::open`]) or gives no
/// title.
pub(crate) fn read_as(f: &Format, path: &str, text: &str) -> Option<Sidecar> {
    let doc = f.open(text)?;
    let at = doc.root();
    let x = &f.fields;
    let title = x.title.first(at).map(|t| tidy(&t)).filter(|t| !t.is_empty() && t.len() <= 120)?;
    let file_stem = path.rsplit('/').next().unwrap_or(path).rsplit_once('.').map_or(path, |(s, _)| s);
    let id =
        x.id.first(at)
            .or_else(|| {
                (f.id_from_file_name && file_stem.len() <= 16 && !file_stem.contains(' ')).then(|| file_stem.to_owned())
            })
            .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'));

    let mut words = split_commas(&x.words.all(at));
    let asset_type = x.kind.first(at).map(|t| tidy(&t)).filter(|t| !t.is_empty());
    if let Some(t) = &asset_type {
        words.push(t.clone());
    }
    let colors = split_commas(&x.colors.all(at));
    let mut tags: Vec<String> = Vec::new();
    let mut push = |t: Option<String>| {
        if let Some(t) = t.filter(|t| t.len() <= 40 && !t.chars().all(|c| c.is_ascii_digit() || c == '-'))
            && !tags.contains(&t)
        {
            tags.push(t);
        }
    };
    for t in &f.tags {
        push(hoard_core::normalize_tag(t));
    }
    for w in &words {
        push(hoard_core::normalize_tag(&w.replace(':', " ")));
    }
    for c in &colors {
        push(hoard_core::normalize_tag(&format!("color:{c}")));
    }
    tags.truncate(MAX_TAGS);

    let mut places: Vec<String> = Vec::new();
    for p in x.origin.all(at).iter().map(|p| tidy(p)) {
        if !places.iter().any(|q| q.eq_ignore_ascii_case(&p)) {
            places.push(p);
        }
    }
    let fact = |v: Option<String>| v.map(|v| tidy(&v)).filter(|v| !v.is_empty() && v.len() <= 40);
    let mut dims = Vec::new();
    for g in &x.dims {
        for n in g.each.nodes(at) {
            if let (Some(name), Some(value)) = (fact(g.name.first(n)), fact(g.value.first(n))) {
                dims.push((name.to_lowercase(), value));
            }
        }
    }
    let size = match (fact(x.size_min.first(at)), fact(x.size_max.first(at))) {
        (Some(a), Some(b)) if a == b => Some(a),
        (Some(a), Some(b)) => Some(format!("{a} to {b}")),
        (a, b) => a.or(b).or_else(|| fact(x.size.first(at))),
    };

    let mut maps: Vec<ListedMap> = Vec::new();
    for g in &x.maps {
        for n in g.each.nodes(at) {
            let kind = g.kind.first(n).unwrap_or_default().to_lowercase();
            for file in g.file.all(n).iter().filter_map(|u| file_name(u)) {
                if !kind.is_empty() && !maps.iter().any(|m| m.file == file) {
                    maps.push(ListedMap { file, kind: kind.clone() });
                }
            }
        }
    }
    // Numbered as their files are (`…_LOD2.fbx`), or else in order.
    let mut lods: Vec<Lod> = Vec::new();
    for g in &x.lods {
        for (i, n) in g.each.nodes(at).into_iter().enumerate() {
            let files: Vec<String> = g.files.all(n).iter().filter_map(|u| file_name(u)).collect();
            let Some(tris) = g.tris.first(n).and_then(|t| t.parse::<u64>().ok()).filter(|&t| t > 0) else { continue };
            if let Some(number) = files.iter().find_map(|f| lod_of(f)).or_else(|| u32::try_from(i).ok()) {
                lods.push(Lod { number, tris, files });
            }
        }
    }
    lods.sort_by_key(|l| l.number);
    lods.dedup_by_key(|l| l.number);

    // Only a surface tiles: its size is one repeat's, and its height map's reach is its displacement.
    let surface = x.tiles.holds(at);
    let physical_size = surface.then(|| x.physical_size.all(at).iter().find_map(|s| area_meters(s))).flatten();
    let displacement =
        surface.then(|| x.displacement.all(at).iter().find_map(|s| meters(s))).flatten().filter(|&m| m < 1.0);

    Some(Sidecar {
        path: path.to_owned(),
        id,
        title,
        format: f.name.clone(),
        source: f.called.clone(),
        whole_pack: f.whole_pack,
        seller: x.seller.first(at).map(|s| tidy(&s)).filter(|s| !s.is_empty() && s.len() <= 80).or(f.seller.clone()),
        license: x.license.first(at).map(|s| tidy(&s)).filter(|s| !s.is_empty() && s.len() <= 120),
        url: x.url.first(at).filter(|u| u.starts_with("https://") || u.starts_with("http://")),
        origin_words: f.origin_words.clone(),
        asset_type,
        tags,
        origin: (!places.is_empty()).then(|| places.join(", ")),
        dims,
        size,
        texel_density: fact(x.texel_density.first(at)),
        maps,
        lods,
        physical_size,
        displacement,
    })
}

fn capitalized(s: &str) -> String {
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
    /// neighbors are (several assets can share a folder).
    fn owns(&self, path: &str, ids_in_folder: bool) -> bool {
        let dir = self.path.rsplit_once('/').map_or("", |(d, _)| d);
        let inside = if dir.is_empty() { true } else { path.starts_with(&format!("{dir}/")) };
        if !inside || !ids_in_folder {
            return inside;
        }
        let file = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
        self.id.as_deref().is_some_and(|id| file.contains(&id.to_ascii_lowercase())) || path == self.path
    }

    /// The map this file lists for a file of the pack (in the JSON's folder or below it), if it does.
    pub(crate) fn listed_map(&self, path: &str) -> Option<&ListedMap> {
        let dir = self.path.rsplit_once('/').map_or("", |(d, _)| d);
        if !dir.is_empty() && !path.starts_with(&format!("{dir}/")) {
            return None;
        }
        let file = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
        self.maps.iter().find(|m| m.file == file)
    }

    /// The triangle count this file gives a model file of the pack, if it lists it as a level of detail. The
    /// spec's LODs are FBX and OBJ; the same mesh in another format counts the same.
    pub(crate) fn lod_tris(&self, path: &str) -> Option<u64> {
        let file = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
        let stem = file.rsplit_once('.').map_or(file.as_str(), |(s, _)| s);
        self.lods
            .iter()
            .find(|l| l.files.iter().any(|f| *f == file || f.rsplit_once('.').is_some_and(|(s, _)| s == stem)))
            .map(|l| l.tris)
    }

    /// A short description from the facts: what it is, where it's from, its size and maps.
    pub(crate) fn describe(&self, maps: &[&str]) -> String {
        let kind = self.asset_type.as_deref().unwrap_or(if self.whole_pack { "pack" } else { "asset" });
        let lead = if kind.starts_with(['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u']) { "An" } else { "A" };
        let mut out = format!("{lead} {kind}");
        if let Some(s) = &self.seller {
            let _ = write!(out, " from {s}");
        }
        if let Some(o) = &self.origin {
            let _ = write!(out, ", {} {o}", self.origin_words);
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
        match self.lods.as_slice() {
            [] => {}
            [only] => facts.push(format!("{} triangles", thousands(only.tris))),
            [first, .., last] => facts.push(format!(
                "LOD{} to LOD{}, {} to {} triangles",
                first.number,
                last.number,
                thousands(first.tris),
                thousands(last.tris)
            )),
        }
        if !facts.is_empty() {
            let text = facts.join("; ");
            out.push(' ');
            out.push_str(&capitalized(&text));
            out.push('.');
        }
        if let Some(l) = &self.license {
            let _ = write!(out, " License: {l}.");
        }
        if !maps.is_empty() {
            let _ = write!(out, " Maps: {}.", maps.join(", "));
        }
        out
    }
}

/// `12000` → `12,000`.
fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
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

/// SQL that's true for an asset (`a`) whose path some format reads, and its parameters.
pub(crate) fn path_filter() -> (String, Vec<String>) {
    filter_for(&store_formats::all())
}

/// SQL that's true for an asset (`a`) whose path one of these formats reads, and its parameters.
pub(crate) fn filter_for(formats: &[Format]) -> (String, Vec<String>) {
    let mut patterns: Vec<String> = Vec::new();
    for p in formats.iter().flat_map(Format::like_patterns) {
        if !patterns.contains(&p) {
            patterns.push(p);
        }
    }
    if patterns.is_empty() {
        return ("0".to_owned(), patterns);
    }
    let sql = vec![r"lower(a.path) LIKE ? ESCAPE '\'"; patterns.len()].join(" OR ");
    (format!("({sql})"), patterns)
}

/// The level of detail a model's file name says (`…_LOD2.fbx` → 2).
fn lod_of(path: &str) -> Option<u32> {
    let file = path.rsplit('/').next().unwrap_or(path);
    let stem = file.rsplit_once('.').map_or(file, |(st, _)| st).to_ascii_lowercase();
    stem.split(|c: char| !c.is_ascii_alphanumeric()).find_map(|w| w.strip_prefix("lod")?.parse().ok())
}

#[cfg(test)]
pub(crate) mod tests;
