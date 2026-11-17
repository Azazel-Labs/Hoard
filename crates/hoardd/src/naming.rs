//! Names for texture maps. A material's maps come named however their maker named them
//! (`historic_miscellaneous_stand_wood_old_05_xeokah0_raw_8K_AO.jpg`); Hoard calls them by the material and what
//! each map is for, in the library and in what it sends.
//!
//! - In the library, a map's name is its material's name and its role ("Stand Wood Old 05 Ambient occlusion"),
//!   set when the material is found ([`name_maps`]) unless someone has named it by hand.
//! - Downloads, bundles and manifests give each map a file name by the convention in Settings ([`TextureNames`]):
//!   `StandWoodOld05_AO.jpg` by default. The files themselves are never renamed; only what's sent is. A map that a
//!   file sent with it refers to by name (an OBJ's MTL, a glTF) keeps its name, so the reference still works.
//! - A model downloaded as a GLB names the textures beside it the same way ([`ForDownload`]): a map by the
//!   convention, and an image that was only ever inside a model by its material and the slot it fills.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::db::BundleFile;
use crate::i18n::tr;
use crate::materials::{ALL_ROLES, Role};

impl Role {
    /// The kind of map as it reads mid-sentence ("Give normal maps a suffix"), in the request's language.
    fn name(self) -> String {
        match self {
            Role::BaseColor => tr!("server-role-base-color"),
            Role::Normal => tr!("server-role-normal"),
            Role::NormalDx => tr!("server-role-normal-dx"),
            Role::Roughness => tr!("server-role-roughness"),
            Role::Gloss => tr!("server-role-gloss"),
            Role::Metalness => tr!("server-role-metalness"),
            Role::Ao => tr!("server-role-ao"),
            Role::Orm => tr!("server-role-orm"),
            Role::Mra => tr!("server-role-mra"),
            Role::Rma => tr!("server-role-rma"),
            Role::Height => tr!("server-role-height"),
            Role::Bump => tr!("server-role-bump"),
            Role::Cavity => tr!("server-role-cavity"),
            Role::Specular => tr!("server-role-specular"),
            Role::Opacity => tr!("server-role-opacity"),
            Role::Emissive => tr!("server-role-emissive"),
            Role::Translucency => tr!("server-role-translucency"),
        }
    }

    /// What the map is for, in words: what the portal shows for it too.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Role::BaseColor => "Color",
            Role::Normal => "Normal",
            Role::NormalDx => "Normal (DirectX)",
            Role::Roughness => "Roughness",
            Role::Gloss => "Gloss",
            Role::Metalness => "Metalness",
            Role::Ao => "Ambient occlusion",
            Role::Orm => "ORM",
            Role::Mra => "MRA",
            Role::Rma => "RMA",
            Role::Height => "Height",
            Role::Bump => "Bump",
            Role::Cavity => "Cavity",
            Role::Specular => "Specular",
            Role::Opacity => "Opacity",
            Role::Emissive => "Emissive",
            Role::Translucency => "Translucency",
        }
    }

    /// The suffix a file name gets by default.
    pub(crate) fn suffix(self) -> &'static str {
        match self {
            Role::BaseColor => "BaseColor",
            Role::Normal => "Normal",
            Role::NormalDx => "NormalDX",
            Role::Roughness => "Roughness",
            Role::Gloss => "Gloss",
            Role::Metalness => "Metallic",
            Role::Ao => "AO",
            Role::Orm => "ORM",
            Role::Mra => "MRA",
            Role::Rma => "RMA",
            Role::Height => "Height",
            Role::Bump => "Bump",
            Role::Cavity => "Cavity",
            Role::Specular => "Specular",
            Role::Opacity => "Opacity",
            Role::Emissive => "Emissive",
            Role::Translucency => "Translucency",
        }
    }
}

/// `CASE <col> WHEN 'base_color' THEN 'Color' … END`, for naming maps in SQL.
fn label_sql(col: &str) -> String {
    let arms: Vec<String> = ALL_ROLES.iter().map(|r| format!("WHEN '{}' THEN '{}'", r.as_str(), r.label())).collect();
    format!("CASE {col} {} ELSE {col} END", arms.join(" "))
}

/// The pack's maps (id, path) that nobody named by hand, taken before its materials are found again.
pub(crate) fn maps_of_pack(conn: &Connection, pack_id: &str) -> Result<Vec<(String, String)>> {
    let rows = conn
        .prepare(
            "SELECT DISTINCT a.id, a.path FROM material_maps mm JOIN assets a ON a.id = mm.texture_id
              WHERE a.pack_id = ?1 AND a.human_edited = 0",
        )?
        .query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    Ok(rows)
}

/// After a pack's materials are found: its maps are called by their material and what they're for
/// ("Stand Wood Old 05 Normal") rather than the file's long name, and a texture in `were_maps` that's no longer a
/// map gets its own name back.
pub(crate) fn rename_maps(conn: &Connection, pack_id: &str, were_maps: &[(String, String)]) -> Result<()> {
    let mut renamed = name_maps(conn, Some(pack_id))?;
    let mut still_map = conn.prepare("SELECT 1 FROM material_maps WHERE texture_id = ?1")?;
    for (id, path) in were_maps {
        if !still_map.exists([id])? {
            let name = crate::tagger::display_name(&crate::tagger::name_stem(path));
            conn.execute(
                "UPDATE assets SET name = ?2 WHERE id = ?1 AND human_edited = 0",
                rusqlite::params![id, name],
            )?;
            renamed.push(id.clone());
        }
    }
    for id in &renamed {
        crate::db::reindex_asset(conn, id)?;
    }
    Ok(())
}

/// A map's size when its material has that role in more than one size (`Rock_2K_Color`, `Rock_4K_Color`), else NULL.
const SIZE_IF_SEVERAL: &str = "CASE WHEN EXISTS (SELECT 1 FROM material_maps o WHERE o.material_id = mm.material_id
        AND o.role = mm.role AND o.texture_id <> mm.texture_id) THEN mm.resolution END";

/// Name the maps of a pack's materials after their material and role:
/// "Stand Wood Old 05 Ambient occlusion", with the size when the set comes in more than one ("… Normal 4K").
/// Names someone typed are left alone. Returns the ids renamed, whose search entries need redoing.
pub(crate) fn name_maps(conn: &Connection, pack_id: Option<&str>) -> Result<Vec<String>> {
    // `None` (every pack) is for tests.
    let name = format!(
        "(SELECT ma.name || ' ' || {} || COALESCE(' ' || {SIZE_IF_SEVERAL}, '')
            FROM material_maps mm JOIN assets ma ON ma.id = mm.material_id
           WHERE mm.texture_id = assets.id ORDER BY ma.path LIMIT 1)",
        label_sql("mm.role")
    );
    let sql = format!(
        "UPDATE assets SET name = {name}
          WHERE (?1 IS NULL OR pack_id = ?1) AND human_edited = 0
            AND id IN (SELECT texture_id FROM material_maps) AND name IS NOT {name}
          RETURNING id"
    );
    let ids = conn.prepare(&sql)?.query_map([pack_id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
    Ok(ids)
}

/// How maps are named in what Hoard sends. Saved from Settings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct TextureNames {
    /// Off: files go out with the names they came with.
    pub(crate) on: bool,
    /// `{name}` is the material's name run together (`StandWoodOld05`), `{map}` the map's suffix, `{size}` its size
    /// (`4K`) when the set comes in more than one. Without `{size}`, a set in several sizes gets `_4K` on the end.
    pub(crate) pattern: String,
    /// Suffixes by role (`base_color`, `ao`, …) or glTF slot ([`SLOTS`]); one not here uses its default.
    pub(crate) maps: BTreeMap<String, String>,
    /// Endings of your own: a texture that isn't a map whose name ends in one goes out with it swapped.
    pub(crate) endings: Vec<Ending>,
}

impl Default for TextureNames {
    fn default() -> Self {
        Self { on: true, pattern: DEFAULT_PATTERN.to_owned(), maps: BTreeMap::new(), endings: Vec::new() }
    }
}

/// An ending swapped for another in a texture's name: `_diff` → `_BC` sends `Rock_diff.png` as `Rock_BC.png`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Ending {
    pub(crate) from: String,
    pub(crate) to: String,
}

/// A slot a glTF material has for a texture beyond the kinds of map Hoard finds materials by. What fills it in a
/// model is named after it in GLB downloads, with a suffix Settings can change.
pub(crate) struct Slot {
    /// What Settings saves the suffix under.
    pub(crate) key: &'static str,
    /// The slot in glTF, without `Texture` (`clearcoatNormal` for `clearcoatNormalTexture`).
    pub(crate) gltf: &'static str,
    pub(crate) suffix: &'static str,
}

const fn slot(key: &'static str, gltf: &'static str, suffix: &'static str) -> Slot {
    Slot { key, gltf, suffix }
}

/// The glTF slots named in GLB downloads, core and extensions (`KHR_materials_*`).
pub(crate) const SLOTS: &[Slot] = &[
    slot("metallic_roughness", "metallicRoughness", "MetallicRoughness"),
    slot("specular_color", "specularColor", "SpecularColor"),
    slot("specular_glossiness", "specularGlossiness", "SpecularGlossiness"),
    slot("clearcoat", "clearcoat", "Clearcoat"),
    slot("clearcoat_roughness", "clearcoatRoughness", "ClearcoatRoughness"),
    slot("clearcoat_normal", "clearcoatNormal", "ClearcoatNormal"),
    slot("sheen_color", "sheenColor", "SheenColor"),
    slot("sheen_roughness", "sheenRoughness", "SheenRoughness"),
    slot("transmission", "transmission", "Transmission"),
    slot("thickness", "thickness", "Thickness"),
    slot("diffuse_transmission", "diffuseTransmission", "DiffuseTransmission"),
    slot("diffuse_transmission_color", "diffuseTransmissionColor", "DiffuseTransmissionColor"),
    slot("iridescence", "iridescence", "Iridescence"),
    slot("iridescence_thickness", "iridescenceThickness", "IridescenceThickness"),
    slot("anisotropy", "anisotropy", "Anisotropy"),
];

pub(crate) const DEFAULT_PATTERN: &str = "{name}_{map}";

/// Characters a file name can't have on one system or another.
fn bad_char(c: char) -> bool {
    c.is_control() || r#"<>:"/\|?*"#.contains(c)
}

impl TextureNames {
    /// Problems a person can fix, in their words.
    pub(crate) fn check(&self) -> Result<(), String> {
        if !self.pattern.contains("{map}") {
            return Err(tr!("error-texture-pattern-map"));
        }
        let rest = self.pattern.replace("{name}", "").replace("{map}", "").replace("{size}", "");
        if rest.contains(['{', '}']) {
            return Err(tr!("error-texture-pattern-fields"));
        }
        if rest.chars().any(bad_char) {
            return Err(tr!("error-texture-name-chars"));
        }
        for (key, suffix) in &self.maps {
            let name = match Role::parse(key) {
                Some(r) => r.name(),
                None => match SLOTS.iter().find(|s| s.key == key) {
                    Some(s) => s.gltf.to_owned(),
                    None => return Err(tr!("error-texture-map-type", role = key)),
                },
            };
            if suffix.trim().is_empty() {
                return Err(tr!("error-texture-suffix-missing", map = name));
            }
            if suffix.chars().any(bad_char) {
                return Err(tr!("error-texture-suffix-chars", map = name));
            }
        }
        let mut seen = HashSet::new();
        for e in &self.endings {
            let from = e.from.trim();
            if from.is_empty() {
                return Err(tr!("error-texture-ending-missing"));
            }
            if from.chars().chain(e.to.trim().chars()).any(bad_char) {
                return Err(tr!("error-texture-ending-chars"));
            }
            if !seen.insert(from.to_lowercase()) {
                return Err(tr!("error-texture-ending-twice", ending = from));
            }
        }
        Ok(())
    }

    fn suffix(&self, role: Role) -> &str {
        self.maps.get(role.as_str()).map_or(role.suffix(), |s| s.trim())
    }

    /// The suffix for a glTF slot ([`SLOTS`]).
    pub(crate) fn slot_suffix<'a>(&'a self, slot: &'a Slot) -> &'a str {
        self.maps.get(slot.key).map_or(slot.suffix, |s| s.trim())
    }

    /// `stem` with an ending of your own swapped, the longest that fits; `None` when none does.
    pub(crate) fn rename_ending(&self, stem: &str) -> Option<String> {
        self.endings
            .iter()
            .filter_map(|e| {
                let from = e.from.trim();
                let at = ending_at(stem, from)?;
                (at > 0).then(|| (from.chars().count(), format!("{}{}", &stem[..at], e.to.trim())))
            })
            .max_by_key(|(n, _)| *n)
            .map(|(_, s)| s)
    }

    /// The file name (without extension) for a map of `material` for `role`, at `size` when the set has several.
    pub(crate) fn stem(&self, material: &str, role: Role, size: Option<&str>) -> String {
        self.stem_with(material, self.suffix(role), size)
    }

    /// [`Self::stem`] with a suffix of its own, for a map that isn't one of the roles (a glTF's clearcoat).
    pub(crate) fn stem_with(&self, material: &str, suffix: &str, size: Option<&str>) -> String {
        let mut out = self.pattern.replace("{name}", &run_together(material)).replace("{map}", suffix);
        match size {
            Some(s) if out.contains("{size}") => out = out.replace("{size}", s),
            Some(s) => out = format!("{out}_{s}"),
            None if out.contains("{size}") => out = tidy(&out.replace("{size}", "")),
            None => {}
        }
        out
    }
}

/// Where `ending` starts in `name`, matched without regard to case, or `None` when `name` doesn't end in it.
fn ending_at(name: &str, ending: &str) -> Option<usize> {
    let n = ending.chars().count();
    if n == 0 {
        return None;
    }
    let at = name.char_indices().rev().nth(n - 1)?.0;
    (name[at..].to_lowercase() == ending.to_lowercase()).then_some(at)
}

/// "Stand Wood Old 05" -> "StandWoodOld05": words run together, each starting with a capital.
pub(crate) fn run_together(name: &str) -> String {
    name.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut cs = w.chars();
            cs.next().map_or_default(|c| c.to_uppercase().chain(cs).collect::<String>())
        })
        .collect()
}

/// Separators left doubled or dangling where `{size}` was taken out: `Rock__Normal` -> `Rock_Normal`.
fn tidy(s: &str) -> String {
    let sep = |c: char| matches!(c, '_' | '-' | '.' | ' ');
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if sep(c) && out.chars().last().is_some_and(sep) {
            continue;
        }
        out.push(c);
    }
    out.trim_matches(sep).to_owned()
}

/// Which material a texture is a map of, as what, and in which size when the set has several.
fn map_of(conn: &Connection, id: &str) -> Result<Option<(String, Role, Option<String>)>> {
    let row: Option<(String, String, Option<String>)> = conn
        .query_row(
            &format!(
                "SELECT ma.name, mm.role, {SIZE_IF_SEVERAL} FROM material_maps mm JOIN assets ma ON ma.id = mm.material_id
                  WHERE mm.texture_id = ?1 ORDER BY ma.path LIMIT 1"
            ),
            [id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .optional()?;
    Ok(row.and_then(|(m, role, size)| Role::parse(&role).map(|r| (m, r, size))))
}

fn split_path(path: &str) -> (&str, &str) {
    path.rsplit_once('/').map_or(("", path), |(d, f)| (d, f))
}

fn ext_of(file: &str) -> Option<&str> {
    file.rsplit_once('.').map(|(_, e)| e)
}

/// A texture's file name without its extension as it's sent: a map's by the convention, another's with an ending of
/// your own swapped; `None` to keep its own (or when naming is off).
pub(crate) fn stem_of(conn: &Connection, names: &TextureNames, id: &str) -> Result<Option<String>> {
    if !names.on {
        return Ok(None);
    }
    if let Some((material, role, size)) = map_of(conn, id)? {
        return Ok(Some(names.stem(&material, role, size.as_deref())));
    }
    if names.endings.is_empty() {
        return Ok(None);
    }
    let texture: Option<String> =
        conn.query_row("SELECT path FROM assets WHERE id = ?1 AND kind = 'texture'", [id], |r| r.get(0)).optional()?;
    Ok(texture.and_then(|path| {
        let (_, file) = split_path(&path);
        names.rename_ending(file.rsplit_once('.').map_or(file, |(s, _)| s))
    }))
}

/// A single map downloaded on its own: its file name by the convention, or `None` to keep its own.
pub(crate) fn file_name(conn: &Connection, names: &TextureNames, id: &str, path: &str) -> Result<Option<String>> {
    let Some(stem) = stem_of(conn, names, id)? else { return Ok(None) };
    let (_, file) = split_path(path);
    Ok(Some(match ext_of(file) {
        Some(ext) => format!("{stem}.{ext}"),
        None => stem,
    }))
}

/// The convention's file name (without extension) for each map in a pack, by its path in the pack.
pub(crate) fn pack_stems(conn: &Connection, names: &TextureNames, pack_id: &str) -> Result<BTreeMap<String, String>> {
    if !names.on {
        return Ok(BTreeMap::new());
    }
    let mut stmt = conn.prepare(
        "SELECT DISTINCT t.id, t.path FROM material_maps mm JOIN assets t ON t.id = mm.texture_id WHERE t.pack_id = ?1",
    )?;
    let maps: Vec<(String, String)> =
        stmt.query_map([pack_id], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<_>>()?;
    let mut out = BTreeMap::new();
    for (id, path) in maps {
        if let Some(stem) = stem_of(conn, names, &id)? {
            out.insert(path, stem);
        }
    }
    Ok(out)
}

/// How a model converted for download names the textures it hands out beside it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub(crate) struct ForDownload {
    /// The convention, for images named by their material and slot; the one out of the box when naming is off,
    /// since those images have no name of their own to keep.
    pub(crate) names: TextureNames,
    /// The pack's maps' file names under the convention, by their paths in the pack ([`pack_stems`]).
    pub(crate) maps: BTreeMap<String, String>,
}

impl ForDownload {
    /// What a download from `pack_id` is named by, under the convention in Settings.
    pub(crate) fn of_pack(conn: &Connection, names: &TextureNames, pack_id: &str) -> Result<Self> {
        let maps = pack_stems(conn, names, pack_id)?;
        Ok(Self { names: if names.on { names.clone() } else { TextureNames::default() }, maps })
    }

    /// A short mark of these names, so a download made under other names is made again.
    pub(crate) fn mark(&self) -> String {
        let json = serde_json::to_vec(self).unwrap_or_default();
        blake3::hash(&json).to_hex()[..12].to_owned()
    }
}

thread_local! {
    /// How the conversion on this thread names its textures, while [`for_download`] runs.
    static FOR_DOWNLOAD: RefCell<ForDownload> = RefCell::default();
}

/// Run `f` with conversions on this thread naming their textures by `names`.
pub(crate) fn for_download<R>(names: ForDownload, f: impl FnOnce() -> R) -> R {
    struct Restore(ForDownload);
    impl Drop for Restore {
        fn drop(&mut self) {
            FOR_DOWNLOAD.set(std::mem::take(&mut self.0));
        }
    }
    let _restore = Restore(FOR_DOWNLOAD.replace(names));
    f()
}

/// How the conversion on this thread names its textures.
pub(crate) fn current_for_download() -> ForDownload {
    FOR_DOWNLOAD.with_borrow(Clone::clone)
}

/// Every role's and glTF slot's suffix under `names`, for Settings.
pub(crate) fn suffixes(names: &TextureNames) -> BTreeMap<&'static str, String> {
    let roles = ALL_ROLES.iter().map(|&r| (r.as_str(), names.suffix(r).to_owned()));
    roles.chain(SLOTS.iter().map(|s| (s.key, names.slot_suffix(s).to_owned()))).collect()
}

/// The convention out of the box, for Settings to go back to.
pub(crate) fn defaults() -> serde_json::Value {
    let d = TextureNames::default();
    serde_json::json!({ "on": d.on, "pattern": d.pattern, "maps": suffixes(&d), "endings": d.endings })
}

/// Rename the maps among `files` by the convention, and other textures by the endings of your own, in the folders
/// they're in. A texture another file here refers to by name keeps its name, and so does one whose new name another
/// file already has.
pub(crate) fn apply(conn: &Connection, names: &TextureNames, files: &mut [BundleFile]) -> Result<()> {
    if !names.on || files.is_empty() {
        return Ok(());
    }
    let here: HashSet<&str> = files.iter().map(|f| f.id.as_str()).collect();
    // Files here that name a texture: models, MTLs, glTFs. A material Hoard made from maps doesn't count; it's not
    // a file anyone opens.
    let mut referrers = conn.prepare(
        "SELECT l.from_id FROM asset_links l JOIN assets f ON f.id = l.from_id
          WHERE l.to_id = ?1 AND l.rel = 'texture' AND f.ext != 'material'",
    )?;
    let mut renames = Vec::new();
    for (i, f) in files.iter().enumerate() {
        let Some(new) = file_name(conn, names, &f.id, &f.path)? else { continue };
        let named_by: Vec<String> = referrers.query_map([&f.id], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
        if named_by.iter().any(|r| here.contains(r.as_str())) {
            continue;
        }
        renames.push((i, new));
    }
    if renames.is_empty() {
        return Ok(());
    }
    let mut taken: HashSet<String> = files.iter().map(|f| f.path.to_lowercase()).collect();
    for (i, new) in renames {
        let (dir, file) = split_path(&files[i].path);
        if file == new {
            continue;
        }
        let path = if dir.is_empty() { new } else { format!("{dir}/{new}") };
        if !taken.insert(path.to_lowercase()) {
            continue;
        }
        taken.remove(&files[i].path.to_lowercase());
        files[i].path = path;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names() -> TextureNames {
        TextureNames::default()
    }

    #[test]
    fn names_a_map_by_its_material_and_role() {
        let n = names();
        assert_eq!(
            n.stem("Historic Miscellaneous Stand Wood Old 05", Role::Ao, None),
            "HistoricMiscellaneousStandWoodOld05_AO"
        );
        assert_eq!(n.stem("rock moss", Role::BaseColor, Some("4K")), "RockMoss_BaseColor_4K");
        let short = TextureNames {
            pattern: "T_{name}_{map}_{size}".into(),
            maps: BTreeMap::from([("base_color".into(), "BC".into())]),
            ..names()
        };
        assert_eq!(short.stem("Rock", Role::BaseColor, Some("2K")), "T_Rock_BC_2K");
        assert_eq!(short.stem("Rock", Role::BaseColor, None), "T_Rock_BC", "no size: the separator goes too");
        assert_eq!(short.stem("Rock", Role::Normal, None), "T_Rock_Normal", "a role not given keeps its default");
    }

    #[test]
    fn refuses_patterns_that_cant_work() {
        let bad = |pattern: &str| TextureNames { pattern: pattern.into(), ..names() }.check().unwrap_err();
        assert!(bad("{name}").contains("{map}"));
        assert!(bad("{name}_{map}_{res}").contains("{size}"));
        assert!(bad("{name}/{map}").contains("can't"));
        let suffix = TextureNames { maps: BTreeMap::from([("ao".into(), " ".into())]), ..names() };
        assert!(suffix.check().unwrap_err().contains("ambient occlusion"));
        let role = TextureNames { maps: BTreeMap::from([("shiny".into(), "S".into())]), ..names() };
        assert!(role.check().unwrap_err().contains("shiny"));
        let slot = TextureNames { maps: BTreeMap::from([("sheen_color".into(), "S/C".into())]), ..names() };
        assert!(slot.check().unwrap_err().contains("sheenColor"));
        let slot = TextureNames { maps: BTreeMap::from([("sheen_color".into(), "SC".into())]), ..names() };
        assert!(slot.check().is_ok());
        let endings = |e: &[(&str, &str)]| TextureNames {
            endings: e.iter().map(|(from, to)| Ending { from: (*from).into(), to: (*to).into() }).collect(),
            ..names()
        };
        assert!(endings(&[(" ", "_BC")]).check().is_err(), "an ending to look for");
        assert!(endings(&[("_d", "_B/C")]).check().is_err());
        assert!(endings(&[("_diff", "_BC"), ("_DIFF", "_D")]).check().unwrap_err().contains("_DIFF"));
        assert!(endings(&[("_diff", "_BC"), ("_d", "")]).check().is_ok(), "swapping for nothing drops it");
        assert!(names().check().is_ok());
    }

    #[test]
    fn swaps_the_longest_ending_that_fits() {
        let n = TextureNames {
            endings: vec![
                Ending { from: "_d".into(), to: "_BC".into() },
                Ending { from: "_mask_d".into(), to: "_Mask".into() },
                Ending { from: "_ü".into(), to: "_U".into() },
            ],
            ..names()
        };
        assert_eq!(n.rename_ending("Rock_D").as_deref(), Some("Rock_BC"), "whatever the case");
        assert_eq!(n.rename_ending("Rock_mask_d").as_deref(), Some("Rock_Mask"));
        assert_eq!(n.rename_ending("Grün_Ü").as_deref(), Some("Grün_U"));
        assert_eq!(n.rename_ending("_d"), None, "a name that's only the ending keeps it");
        assert_eq!(n.rename_ending("Rock_dx"), None);
        assert_eq!(n.rename_ending("d"), None);
    }

    fn db() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, name TEXT, ext TEXT, path TEXT,
                                  human_edited INTEGER NOT NULL DEFAULT 0);
             CREATE TABLE material_maps (material_id TEXT, texture_id TEXT, role TEXT, resolution TEXT);
             CREATE TABLE asset_links (from_id TEXT, to_id TEXT, rel TEXT);
             INSERT INTO assets VALUES
               ('mat', 'p', 'Stand Wood Old 05', 'material', 'm/stand.material', 0),
               ('ao', 'p', 'stand wood old 05 xeokah 0 raw 8k AO', 'jpg', 'm/stand_xeokah_raw_8K_AO.jpg', 0),
               ('n', 'p', 'stand normal', 'jpg', 'm/stand_xeokah_raw_8K_Normal.jpg', 1),
               ('fbx', 'p', 'Stand', 'fbx', 'm/stand.fbx', 0),
               ('obj', 'p', 'Crate', 'obj', 'c/crate.obj', 0),
               ('mtl', 'p', 'crate', 'mtl', 'c/crate.mtl', 0),
               ('col', 'p', 'crate col', 'png', 'c/crate_col.png', 0),
               ('cmat', 'p', 'Crate', 'material', 'c/crate.material', 0),
               ('col4', 'p', 'crate col 4k', 'png', 'c/4k/crate_col.png', 0);
             INSERT INTO material_maps VALUES ('mat', 'ao', 'ao', '8K'), ('mat', 'n', 'normal', '8K'),
                                              ('cmat', 'col', 'base_color', '1K'), ('cmat', 'col4', 'base_color', '4K');
             INSERT INTO asset_links VALUES ('fbx', 'ao', 'guess'), ('mat', 'ao', 'texture'),
                                            ('obj', 'mtl', 'material'), ('mtl', 'col', 'texture'),
                                            ('cmat', 'col', 'texture');",
        )
        .unwrap();
        c
    }

    fn file(id: &str, path: &str) -> BundleFile {
        BundleFile { id: id.into(), path: path.into(), hash: String::new(), size: 0, unity_guid: None }
    }

    #[test]
    fn maps_are_named_after_their_material_unless_someone_named_them() {
        let c = db();
        assert_eq!(name_maps(&c, Some("p")).unwrap(), vec!["ao".to_owned(), "col".to_owned(), "col4".to_owned()]);
        let name =
            |id: &str| c.query_row("SELECT name FROM assets WHERE id = ?1", [id], |r| r.get::<_, String>(0)).unwrap();
        assert_eq!(name("ao"), "Stand Wood Old 05 Ambient occlusion", "one size, so no size in the name");
        assert_eq!(name("col4"), "Crate Color 4K", "two sizes of the color map");
        assert_eq!(name("n"), "stand normal", "named by hand");
        assert!(name_maps(&c, Some("p")).unwrap().is_empty(), "nothing left to do the second time");
    }

    #[test]
    fn sends_maps_by_the_convention_but_not_ones_a_file_names() {
        let c = db();
        let mut files = vec![
            file("fbx", "m/stand.fbx"),
            file("ao", "m/stand_xeokah_raw_8K_AO.jpg"),
            file("n", "m/stand_xeokah_raw_8K_Normal.jpg"),
        ];
        apply(&c, &names(), &mut files).unwrap();
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(paths, ["m/stand.fbx", "m/StandWoodOld05_AO.jpg", "m/StandWoodOld05_Normal.jpg"]);

        // The MTL names crate_col.png, so with the MTL it keeps that name; on its own it doesn't need to.
        let mut obj = vec![file("obj", "c/crate.obj"), file("mtl", "c/crate.mtl"), file("col", "c/crate_col.png")];
        apply(&c, &names(), &mut obj).unwrap();
        assert_eq!(obj[2].path, "c/crate_col.png");
        let mut alone = vec![file("col", "c/crate_col.png")];
        apply(&c, &names(), &mut alone).unwrap();
        assert_eq!(alone[0].path, "c/Crate_BaseColor_1K.png", "two sizes, so the size tells them apart");

        let mut off = vec![file("ao", "m/stand_xeokah_raw_8K_AO.jpg")];
        apply(&c, &TextureNames { on: false, ..names() }, &mut off).unwrap();
        assert_eq!(off[0].path, "m/stand_xeokah_raw_8K_AO.jpg");
    }

    #[test]
    fn sends_a_texture_that_isnt_a_map_with_your_ending() {
        let c = db();
        c.execute_batch(
            "ALTER TABLE assets ADD COLUMN kind TEXT;
             UPDATE assets SET kind = 'texture' WHERE ext IN ('jpg', 'png');
             INSERT INTO assets VALUES ('diff', 'p', 'rock diff', 'png', 'r/rock_diff.png', 0, 'texture'),
                                       ('dobj', 'p', 'rock diff', 'obj', 'r/rock_diff.obj', 0, 'model');",
        )
        .unwrap();
        let n = TextureNames { endings: vec![Ending { from: "_diff".into(), to: "_BC".into() }], ..names() };
        assert_eq!(file_name(&c, &n, "diff", "r/rock_diff.png").unwrap().as_deref(), Some("rock_BC.png"));
        assert_eq!(file_name(&c, &n, "dobj", "r/rock_diff.obj").unwrap(), None, "only textures");
        let col = file_name(&c, &n, "col", "c/crate_col.png").unwrap();
        assert_eq!(col.as_deref(), Some("Crate_BaseColor_1K.png"), "a map goes by the convention");
        assert_eq!(file_name(&c, &TextureNames { on: false, ..n }, "diff", "r/rock_diff.png").unwrap(), None);
    }

    #[test]
    fn a_new_name_another_file_has_is_not_taken() {
        let c = db();
        let mut files = vec![file("ao", "m/a.jpg"), file("fbx", "m/StandWoodOld05_AO.jpg")];
        apply(&c, &names(), &mut files).unwrap();
        assert_eq!(files[0].path, "m/a.jpg");
    }
}
