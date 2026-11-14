//! Names for texture maps. A material's maps come named however their maker named them
//! (`historic_miscellaneous_stand_wood_old_05_xeokah0_raw_8K_AO.jpg`); Hoard calls them by the material and what
//! each map is for, in the library and in what it sends.
//!
//! - In the library, a map's name is its material's name and its role ("Stand Wood Old 05 Ambient occlusion"),
//!   set when the material is found ([`name_maps`]) unless someone has named it by hand.
//! - Downloads, bundles and manifests give each map a file name by the convention in Settings ([`TextureNames`]):
//!   `StandWoodOld05_AO.jpg` by default. The files themselves are never renamed; only what's sent is. A map that a
//!   file sent with it refers to by name (an OBJ's MTL, a glTF) keeps its name, so the reference still works.

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
            Role::BaseColor => "Colour",
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

/// `CASE <col> WHEN 'base_color' THEN 'Colour' … END`, for naming maps in SQL.
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
    /// Suffixes by role (`base_color`, `ao`, …); a role not here uses [`Role::suffix`].
    pub(crate) maps: BTreeMap<String, String>,
}

impl Default for TextureNames {
    fn default() -> Self {
        Self { on: true, pattern: DEFAULT_PATTERN.to_owned(), maps: BTreeMap::new() }
    }
}

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
        for (role, suffix) in &self.maps {
            let Some(r) = Role::parse(role) else { return Err(tr!("error-texture-map-type", role = role)) };
            if suffix.trim().is_empty() {
                return Err(tr!("error-texture-suffix-missing", map = r.name()));
            }
            if suffix.chars().any(bad_char) {
                return Err(tr!("error-texture-suffix-chars", map = r.name()));
            }
        }
        Ok(())
    }

    fn suffix(&self, role: Role) -> &str {
        self.maps.get(role.as_str()).map_or(role.suffix(), |s| s.trim())
    }

    /// The file name (without extension) for a map of `material` for `role`, at `size` when the set has several.
    pub(crate) fn stem(&self, material: &str, role: Role, size: Option<&str>) -> String {
        let mut out = self.pattern.replace("{name}", &run_together(material)).replace("{map}", self.suffix(role));
        match size {
            Some(s) if out.contains("{size}") => out = out.replace("{size}", s),
            Some(s) => out = format!("{out}_{s}"),
            None if out.contains("{size}") => out = tidy(&out.replace("{size}", "")),
            None => {}
        }
        out
    }
}

/// "Stand Wood Old 05" -> "StandWoodOld05": words run together, each starting with a capital.
fn run_together(name: &str) -> String {
    name.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut cs = w.chars();
            cs.next().map(|c| c.to_uppercase().chain(cs).collect::<String>()).unwrap_or_default()
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

/// A map's file name without its extension, by the convention, or `None` when it isn't a map (or naming is off).
pub(crate) fn stem_of(conn: &Connection, names: &TextureNames, id: &str) -> Result<Option<String>> {
    if !names.on {
        return Ok(None);
    }
    Ok(map_of(conn, id)?.map(|(material, role, size)| names.stem(&material, role, size.as_deref())))
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

/// Every role's suffix under `names`, for Settings.
pub(crate) fn suffixes(names: &TextureNames) -> BTreeMap<&'static str, String> {
    ALL_ROLES.iter().map(|&r| (r.as_str(), names.suffix(r).to_owned())).collect()
}

/// The convention out of the box, for Settings to go back to.
pub(crate) fn defaults() -> serde_json::Value {
    let d = TextureNames::default();
    serde_json::json!({ "on": d.on, "pattern": d.pattern, "maps": suffixes(&d) })
}

/// Rename the maps among `files` by the convention, in the folders they're in. A map another file here refers to by
/// name keeps its name, and so does one whose new name another file already has.
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
        assert!(names().check().is_ok());
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
        assert_eq!(name("col4"), "Crate Colour 4K", "two sizes of the colour map");
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
    fn a_new_name_another_file_has_is_not_taken() {
        let c = db();
        let mut files = vec![file("ao", "m/a.jpg"), file("fbx", "m/StandWoodOld05_AO.jpg")];
        apply(&c, &names(), &mut files).unwrap();
        assert_eq!(files[0].path, "m/a.jpg");
    }
}
