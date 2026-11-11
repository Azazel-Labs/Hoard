//! Unreal Engine packages (`.uasset`, `.umap`), read without Unreal.
//!
//! Marketplace and Fab packs often ship as Unreal content rather than FBX/PNG/WAV. An editor (uncooked) package
//! still holds a lot that's useful on its own:
//!
//! - the package summary: engine version, the asset's class (`StaticMesh`, `Texture2D`, `SoundWave`, …);
//! - the thumbnail the editor rendered for the Content Browser (PNG, or JPEG in newer versions);
//! - the asset registry tags the editor writes for search (texture dimensions, triangle counts, durations, the
//!   skeleton a mesh uses, …);
//! - the import table: which other packages it depends on (`/Game/Pack/Textures/T_Rock`), which become links;
//! - for some classes the imported source file itself: a `SoundWave` keeps the original WAV, and UE4-era textures
//!   keep their source as PNG. Those are carved out as a `source` derivative.
//!
//! Meshes and animations are stored in Unreal's own formats; getting FBX out of those needs the editor
//! (`tools/unreal_export.py`). Cooked packages (`.uasset` + `.uexp` from a shipped game) parse too, but have no
//! thumbnails, tags or sources.
//!
//! Layout follows Unreal's `FPackageFileSummary`, `FObjectImport` and `FObjectExport`, gated on the file's
//! UE4/UE5 object versions so packages from UE 4.x through 5.x read.

use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};

use crate::reader::Reader;
use hoard_core::AssetKind;

pub(crate) const PACKAGE_TAG: u32 = 0x9E2A_83C1;
/// Package flag: editor-only data stripped (a cooked package).
const PKG_FILTER_EDITOR_ONLY: u32 = 0x8000_0000;
/// Package flag: properties serialized without tags (cooked); export entries then lack script offsets.
const PKG_UNVERSIONED_PROPERTIES: u32 = 0x0000_2000;
/// Don't read headers bigger than this; real ones are kilobytes to a few megabytes.
const MAX_HEADER: usize = 256 << 20;
/// Don't scan files bigger than this for embedded sources.
pub const MAX_CARVE: u64 = 2 << 30;

// UE4 object versions (EUnrealEngineObjectUE4Version).
pub(crate) mod ue4 {
    pub(super) const REMOVE_NET_INDEX: i32 = 196;
    pub(super) const ENGINE_VERSION_OBJECT: i32 = 336;
    pub(super) const LOAD_FOR_EDITOR_GAME: i32 = 365;
    pub(super) const ADD_STRING_ASSET_REFERENCES_MAP: i32 = 384;
    pub(super) const PACKAGE_SUMMARY_HAS_COMPATIBLE_ENGINE_VERSION: i32 = 444;
    pub(super) const SERIALIZE_TEXT_IN_PACKAGES: i32 = 459;
    pub(super) const COOKED_ASSETS_IN_EDITOR_SUPPORT: i32 = 485;
    pub(super) const NAME_HASHES_SERIALIZED: i32 = 504;
    pub(super) const PRELOAD_DEPENDENCIES_IN_COOKED_EXPORTS: i32 = 507;
    pub(super) const TEMPLATE_INDEX_IN_COOKED_EXPORTS: i32 = 508;
    pub(super) const ADDED_SEARCHABLE_NAMES: i32 = 510;
    pub(super) const BIT64_EXPORTMAP_SERIALSIZES: i32 = 511;
    pub(super) const ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID: i32 = 516;
    pub(super) const ADDED_PACKAGE_OWNER: i32 = 518;
    pub(super) const NON_OUTER_PACKAGE_IMPORT: i32 = 520;
    pub(super) const ASSETREGISTRY_DEPENDENCYFLAGS: i32 = 521;
    pub(super) const WORLD_LEVEL_INFO: i32 = 224;
    pub(super) const ADDED_CHUNKID_TO_ASSETDATA_AND_UPACKAGE: i32 = 278;
    pub(super) const CHANGED_CHUNKID_TO_BE_AN_ARRAY_OF_CHUNKIDS: i32 = 326;
    pub(crate) const STRUCT_GUID_IN_PROPERTY_TAG: i32 = 441;
    pub(crate) const INNER_ARRAY_TAG_INFO: i32 = 500;
    pub(crate) const PROPERTY_GUID_IN_PROPERTY_TAG: i32 = 503;
}

// UE5 object versions (EUnrealEngineObjectUE5Version).
pub(crate) mod ue5 {
    pub(super) const OPTIONAL_RESOURCES: i32 = 1003;
    pub(super) const REMOVE_OBJECT_EXPORT_PACKAGE_GUID: i32 = 1005;
    pub(super) const TRACK_OBJECT_EXPORT_IS_INHERITED: i32 = 1006;
    pub(super) const ADD_SOFTOBJECTPATH_LIST: i32 = 1008;
    pub(super) const SCRIPT_SERIALIZATION_OFFSET: i32 = 1010;
    pub(super) const METADATA_SERIALIZATION_OFFSET: i32 = 1014;
    pub(super) const VERSE_CELLS: i32 = 1015;
    pub(super) const PACKAGE_SAVED_HASH: i32 = 1016;
    pub(super) const IMPORT_TYPE_HIERARCHIES: i32 = 1018;
    pub(super) const NAMES_REFERENCED_FROM_EXPORT_DATA: i32 = 1001;
    pub(super) const PAYLOAD_TOC: i32 = 1002;
    pub(crate) const LARGE_WORLD_COORDINATES: i32 = 1004;
    pub(crate) const FSOFTOBJECTPATH_REMOVE_ASSET_PATH_FNAMES: i32 = 1007;
    pub(super) const DATA_RESOURCES: i32 = 1009;
    pub(crate) const PROPERTY_TAG_EXTENSION_AND_OVERRIDABLE_SERIALIZATION: i32 = 1011;
    pub(crate) const PROPERTY_TAG_COMPLETE_TYPE_NAME: i32 = 1012;
}

/// What we learned from one package's header.
#[derive(Debug, Default, Clone)]
pub struct Package {
    /// "5.3.2" (empty if the package didn't record it).
    pub engine: String,
    pub cooked: bool,
    /// Class of the main asset, e.g. `StaticMesh`.
    pub class: Option<String>,
    /// Object name of the main asset.
    pub asset_name: Option<String>,
    /// Other packages this one imports from, e.g. `/Game/Pack/Textures/T_Rock` (engine packages left out).
    pub dependencies: Vec<String>,
    /// Editor thumbnail (PNG or JPEG bytes).
    pub thumbnail: Option<Vec<u8>>,
    /// Asset registry tags of the main asset.
    pub tags: Vec<(String, String)>,
    pub header_size: u64,
    pub ue4: i32,
    pub ue5: i32,
    pub flags: u32,
    /// Custom version GUIDs and their versions (engine subsystems version their own data).
    pub custom_versions: Vec<([u8; 16], i32)>,
    pub names: Vec<String>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    /// `FSoftObjectPath`s referenced by index from property data (UE 5.1+).
    pub soft_object_paths: Vec<String>,
    /// Where the package trailer (editor bulk data payloads) starts, if there is one.
    pub payload_toc_offset: Option<i64>,
    pub bulk_data_start: i64,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub class_name: String,
    pub outer_index: i32,
    pub object_name: String,
}

#[derive(Debug, Clone)]
pub struct Export {
    pub class_index: i32,
    pub outer_index: i32,
    pub name: String,
    pub is_asset: bool,
    pub serial_offset: i64,
    pub serial_size: i64,
}

impl Package {
    /// Whether property tags in this package use UE 5.4+'s complete type names.
    pub(crate) fn complete_type_names(&self) -> bool {
        self.ue5 >= ue5::PROPERTY_TAG_COMPLETE_TYPE_NAME
    }
    pub(crate) fn custom_version(&self, guid: [u8; 16]) -> Option<i32> {
        self.custom_versions.iter().find(|(g, _)| *g == guid).map(|(_, v)| *v)
    }
    /// The main asset export (flagged as the asset, or the first top-level object).
    pub fn main_export(&self) -> Option<&Export> {
        self.exports.iter().find(|e| e.is_asset).or_else(|| self.exports.iter().find(|e| e.outer_index == 0))
    }
    /// Path of an object by package index: `/Game/Pack/T_Rock.T_Rock` for imports, `.Name` for exports.
    pub fn object_path(&self, index: i32) -> Option<String> {
        match index.cmp(&0) {
            std::cmp::Ordering::Less => {
                let mut chain = Vec::new();
                let mut i = usize::try_from(-index - 1).ok()?;
                for _ in 0..16 {
                    let imp = self.imports.get(i)?;
                    chain.push(imp.object_name.clone());
                    if imp.outer_index >= 0 {
                        break;
                    }
                    i = usize::try_from(-imp.outer_index - 1).ok()?;
                }
                chain.reverse();
                let mut it = chain.into_iter();
                let root = it.next()?;
                let rest: Vec<String> = it.collect();
                Some(if rest.is_empty() { root } else { format!("{root}.{}", rest.join(":")) })
            }
            std::cmp::Ordering::Greater => {
                self.exports.get(usize::try_from(index - 1).ok()?).map(|e| format!(".{}", e.name))
            }
            std::cmp::Ordering::Equal => None,
        }
    }
}

/// A source file found embedded in a package.
#[derive(Debug, Clone)]
pub struct Embedded {
    pub ext: &'static str,
    pub bytes: Vec<u8>,
}

/// `FName` as stored in tables: an index into the name map plus an instance number.
fn fname(r: &mut Reader<'_>, names: &[String]) -> Result<String> {
    let index = r.i32()?;
    let number = r.i32()?;
    let base = usize::try_from(index).ok().and_then(|i| names.get(i)).context("name index out of range")?;
    Ok(if number > 0 { format!("{base}_{}", number - 1) } else { base.clone() })
}

/// Read the header of a `.uasset`/`.umap` file.
pub fn read(path: &Path) -> Result<Package> {
    let mut f = std::fs::File::open(path)?;
    // Read enough for the summary, then the whole header once its size is known.
    let mut head = Vec::new();
    (&mut f).take(64 << 10).read_to_end(&mut head)?;
    let total = header_size(&head)?;
    if total > head.len() {
        ensure!(total <= MAX_HEADER, "package header is implausibly large ({total} bytes)");
        let more = total - head.len();
        (&mut f).take(more as u64).read_to_end(&mut head)?;
    }
    parse(&head)
}

/// `TotalHeaderSize` from the start of the summary, without parsing the rest.
fn header_size(data: &[u8]) -> Result<usize> {
    let mut r = Reader::new(data);
    let (_, legacy, ue4v, ue5v) = versions(&mut r)?;
    if ue5v >= ue5::PACKAGE_SAVED_HASH {
        r.skip(20)?;
        return Ok(usize::try_from(r.i32()?)?);
    }
    custom_versions(&mut r, legacy)?;
    let _ = ue4v;
    Ok(usize::try_from(r.i32()?)?)
}

/// Tag, legacy file version, UE4 and UE5 object versions (licensee version skipped).
fn versions(r: &mut Reader<'_>) -> Result<(u32, i32, i32, i32)> {
    let tag = r.u32()?;
    if tag != PACKAGE_TAG {
        if tag == PACKAGE_TAG.swap_bytes() {
            bail!("big-endian packages aren't supported");
        }
        bail!("not an Unreal package");
    }
    let legacy = r.i32()?;
    ensure!((-9..0).contains(&legacy), "unsupported package format {legacy}");
    if legacy != -4 {
        r.i32()?; // legacy UE3 version
    }
    let ue4v = r.i32()?;
    let ue5v = if legacy <= -8 { r.i32()? } else { 0 };
    r.i32()?; // licensee version
    if ue4v == 0 && ue5v == 0 {
        bail!("unversioned package (cooked without version info); can't read it without knowing the engine");
    }
    Ok((tag, legacy, ue4v, ue5v))
}

fn custom_versions(r: &mut Reader<'_>, legacy: i32) -> Result<Vec<([u8; 16], i32)>> {
    let mut out = Vec::new();
    if legacy > -2 {
        return Ok(out);
    }
    let n = r.count(4096)?;
    for _ in 0..n {
        match legacy {
            -2 => r.skip(8)?, // enum tag + version
            -5..=-3 => {
                let g = r.guid()?;
                out.push((g, r.i32()?));
                r.fstring()?;
            }
            _ => {
                let g = r.guid()?;
                out.push((g, r.i32()?));
            }
        }
    }
    Ok(out)
}

#[expect(clippy::too_many_lines, reason = "one linear walk over the summary layout")]
pub fn parse(data: &[u8]) -> Result<Package> {
    let mut r = Reader::new(data);
    let (_, legacy, ue4v, ue5v) = versions(&mut r)?;
    let total_header;
    let customs;
    if ue5v >= ue5::PACKAGE_SAVED_HASH {
        r.skip(20)?;
        total_header = r.i32()?;
        customs = custom_versions(&mut r, legacy)?;
    } else {
        customs = custom_versions(&mut r, legacy)?;
        total_header = r.i32()?;
    }
    r.fstring()?; // folder name
    let flags = r.u32()?;
    let cooked = flags & PKG_FILTER_EDITOR_ONLY != 0;
    let name_count = r.count(4_000_000)?;
    let name_offset = r.i32()?;
    let (mut soft_count, mut soft_offset) = (0, 0);
    if ue5v >= ue5::ADD_SOFTOBJECTPATH_LIST {
        soft_count = r.count(4_000_000)?;
        soft_offset = r.i32()?;
    }
    if !cooked && ue4v >= ue4::ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID {
        r.fstring()?;
    }
    if ue4v >= ue4::SERIALIZE_TEXT_IN_PACKAGES {
        r.i32()?;
        r.i32()?;
    }
    let export_count = r.count(4_000_000)?;
    let export_offset = r.i32()?;
    let import_count = r.count(4_000_000)?;
    let import_offset = r.i32()?;
    if ue5v >= ue5::VERSE_CELLS {
        for _ in 0..4 {
            r.i32()?;
        }
    }
    if ue5v >= ue5::METADATA_SERIALIZATION_OFFSET {
        r.i32()?;
    }
    r.i32()?; // depends offset
    if ue4v >= ue4::ADD_STRING_ASSET_REFERENCES_MAP {
        r.i32()?;
        r.i32()?;
    }
    if ue4v >= ue4::ADDED_SEARCHABLE_NAMES {
        r.i32()?;
    }
    let thumbnail_offset = r.i32()?;
    if ue5v >= ue5::IMPORT_TYPE_HIERARCHIES {
        r.i32()?;
        r.i32()?;
    }
    if ue5v < ue5::PACKAGE_SAVED_HASH {
        r.guid()?;
    }
    if !cooked && ue4v >= ue4::ADDED_PACKAGE_OWNER {
        r.guid()?;
        if ue4v < ue4::NON_OUTER_PACKAGE_IMPORT {
            r.guid()?;
        }
    }
    let generations = r.count(100_000)?;
    let per_generation = if ue4v >= ue4::REMOVE_NET_INDEX { 8 } else { 12 };
    r.skip(generations as u64 * per_generation)?;
    let engine = if ue4v >= ue4::ENGINE_VERSION_OBJECT {
        let (major, minor, patch) = (r.u16()?, r.u16()?, r.u16()?);
        r.u32()?; // changelist
        r.fstring()?; // branch
        if major > 0 { format!("{major}.{minor}.{patch}") } else { String::new() }
    } else {
        r.i32()?;
        String::new()
    };
    if ue4v >= ue4::PACKAGE_SUMMARY_HAS_COMPATIBLE_ENGINE_VERSION {
        r.skip(10)?;
        r.fstring()?;
    }
    r.u32()?; // compression flags
    let chunks = r.count(1_000_000)?;
    ensure!(chunks == 0, "package-level compression isn't supported");
    r.u32()?; // package source
    let extra_packages = r.count(100_000)?;
    for _ in 0..extra_packages {
        r.fstring()?;
    }
    if legacy > -7 {
        r.i32()?; // texture allocations
    }
    let registry_offset = r.i32()?;
    let bulk_data_start = r.i64()?;
    // The rest of the summary only matters for finding the package trailer.
    let tail = (|| -> Result<Option<i64>> {
        if ue4v >= ue4::WORLD_LEVEL_INFO {
            r.i32()?;
        }
        if ue4v >= ue4::CHANGED_CHUNKID_TO_BE_AN_ARRAY_OF_CHUNKIDS {
            let n = r.count(100_000)?;
            r.skip(n as u64 * 4)?;
        } else if ue4v >= ue4::ADDED_CHUNKID_TO_ASSETDATA_AND_UPACKAGE {
            r.i32()?;
        }
        if ue4v >= ue4::PRELOAD_DEPENDENCIES_IN_COOKED_EXPORTS {
            r.i32()?;
            r.i32()?;
        }
        if ue5v >= ue5::NAMES_REFERENCED_FROM_EXPORT_DATA {
            r.i32()?;
        }
        if ue5v >= ue5::PAYLOAD_TOC {
            let off = r.i64()?;
            if ue5v >= ue5::DATA_RESOURCES {
                r.i32()?;
            }
            return Ok((off > 0).then_some(off));
        }
        Ok(None)
    })();
    let payload_toc_offset = tail.ok().flatten();

    ensure!(total_header > 0 && (total_header as usize) <= data.len(), "header size {total_header} doesn't fit");
    let mut pkg = Package {
        engine,
        cooked,
        header_size: u64::try_from(total_header)?,
        ue4: ue4v,
        ue5: ue5v,
        flags,
        custom_versions: customs,
        payload_toc_offset,
        bulk_data_start,
        ..Package::default()
    };

    // Name map.
    r.seek(i64::from(name_offset))?;
    let mut names = Vec::with_capacity(name_count.min(1 << 16));
    for _ in 0..name_count {
        names.push(r.fstring()?);
        if ue4v >= ue4::NAME_HASHES_SERIALIZED {
            r.skip(4)?;
        }
    }

    // Imports: classes the exports are instances of, and the packages this one depends on.
    r.seek(i64::from(import_offset))?;
    let mut imports = Vec::with_capacity(import_count.min(1 << 16));
    for _ in 0..import_count {
        fname(&mut r, &names)?; // class package
        let class_name = fname(&mut r, &names)?;
        let outer_index = r.i32()?;
        let object_name = fname(&mut r, &names)?;
        if !cooked && ue4v >= ue4::NON_OUTER_PACKAGE_IMPORT {
            fname(&mut r, &names)?;
        }
        if ue5v >= ue5::OPTIONAL_RESOURCES {
            r.i32()?;
        }
        imports.push(Import { class_name, outer_index, object_name });
    }
    let mut deps: Vec<String> = imports
        .iter()
        .filter(|i| i.class_name == "Package" && i.object_name.starts_with('/'))
        .map(|i| i.object_name.clone())
        .filter(|p| !p.starts_with("/Script/") && !p.starts_with("/Engine/"))
        .collect();
    deps.sort();
    deps.dedup();
    pkg.dependencies = deps;

    // Exports: the objects in this package; the one flagged as the asset tells us what this is.
    r.seek(i64::from(export_offset))?;
    let mut exports = Vec::with_capacity(export_count.min(1 << 16));
    for _ in 0..export_count {
        let class_index = r.i32()?;
        r.i32()?; // super
        if ue4v >= ue4::TEMPLATE_INDEX_IN_COOKED_EXPORTS {
            r.i32()?;
        }
        let outer_index = r.i32()?;
        let name = fname(&mut r, &names)?;
        r.u32()?; // object flags
        let (serial_size, serial_offset) = if ue4v >= ue4::BIT64_EXPORTMAP_SERIALSIZES {
            (r.i64()?, r.i64()?)
        } else {
            (i64::from(r.i32()?), i64::from(r.i32()?))
        };
        r.skip(12)?; // forced export, not for client, not for server
        if ue5v < ue5::REMOVE_OBJECT_EXPORT_PACKAGE_GUID {
            r.guid()?;
        }
        if ue5v >= ue5::TRACK_OBJECT_EXPORT_IS_INHERITED {
            r.i32()?;
        }
        r.u32()?; // package flags
        if ue4v >= ue4::LOAD_FOR_EDITOR_GAME {
            r.i32()?;
        }
        let is_asset = ue4v >= ue4::COOKED_ASSETS_IN_EDITOR_SUPPORT && r.i32()? != 0;
        if ue5v >= ue5::OPTIONAL_RESOURCES {
            r.i32()?;
        }
        if ue4v >= ue4::PRELOAD_DEPENDENCIES_IN_COOKED_EXPORTS {
            r.skip(20)?;
        }
        if flags & PKG_UNVERSIONED_PROPERTIES == 0 && ue5v >= ue5::SCRIPT_SERIALIZATION_OFFSET {
            r.skip(16)?;
        }
        exports.push(Export { class_index, outer_index, name, is_asset, serial_offset, serial_size });
    }
    let class_of = |e: &Export| -> Option<String> {
        match e.class_index {
            i if i < 0 => imports.get(usize::try_from(-i - 1).ok()?).map(|imp| imp.object_name.clone()),
            i if i > 0 => exports.get(usize::try_from(i - 1).ok()?).map(|ex| ex.name.clone()),
            _ => None,
        }
    };
    if let Some(main) = exports.iter().find(|e| e.is_asset).or_else(|| exports.iter().find(|e| e.outer_index == 0)) {
        pkg.class = class_of(main);
        pkg.asset_name = Some(main.name.clone());
    }

    // Soft object paths referenced by index from property data.
    if soft_count > 0 && soft_offset > 0 {
        r.seek(i64::from(soft_offset))?;
        for _ in 0..soft_count {
            let path = if ue5v >= ue5::FSOFTOBJECTPATH_REMOVE_ASSET_PATH_FNAMES {
                let package = fname(&mut r, &names)?;
                let asset = fname(&mut r, &names)?;
                let sub = r.fstring()?;
                let mut p = if asset == "None" { package } else { format!("{package}.{asset}") };
                if !sub.is_empty() {
                    p = format!("{p}:{sub}");
                }
                p
            } else {
                let path = fname(&mut r, &names)?;
                let sub = r.fstring()?;
                if sub.is_empty() { path } else { format!("{path}:{sub}") }
            };
            pkg.soft_object_paths.push(path);
        }
    }

    // Thumbnail table: class, object path, and where the image data sits.
    if !cooked && thumbnail_offset > 0 && (thumbnail_offset as usize) < data.len() {
        r.seek(i64::from(thumbnail_offset))?;
        let n = r.count(10_000)?;
        let mut first = None;
        for _ in 0..n {
            let class = r.fstring()?;
            r.fstring()?; // object path
            let offset = r.i32()?;
            first.get_or_insert((class, offset));
        }
        if let Some((class, offset)) = first {
            if pkg.class.is_none() {
                pkg.class = Some(class);
            }
            r.seek(i64::from(offset))?;
            r.i32()?; // width
            r.i32()?; // height (negative: JPEG)
            let len = r.count(64 << 20)?;
            if len > 0 {
                let start = r.pos();
                let bytes = data.get(start..start + len).context("thumbnail runs past the header")?;
                pkg.thumbnail = Some(bytes.to_vec());
            }
        }
    }

    // Asset registry tags: what the Content Browser shows in tooltips.
    if registry_offset > 0 && (registry_offset as usize) < data.len() {
        r.seek(i64::from(registry_offset))?;
        if !cooked && ue4v >= ue4::ASSETREGISTRY_DEPENDENCYFLAGS {
            r.i64()?; // dependency data offset
        }
        if let Ok(n) = r.count(10_000) {
            for _ in 0..n {
                let Ok(path) = r.fstring() else { break };
                let Ok(class) = r.fstring() else { break };
                let Ok(tag_count) = r.count(10_000) else { break };
                let mut tags = Vec::with_capacity(tag_count);
                for _ in 0..tag_count {
                    let (Ok(k), Ok(v)) = (r.fstring(), r.fstring()) else { break };
                    tags.push((k, v));
                }
                // The main asset's entry is the one named like the asset.
                let is_main = pkg.asset_name.as_deref().is_some_and(|n| path.ends_with(n)) || pkg.tags.is_empty();
                if is_main {
                    pkg.tags = tags;
                    if pkg.class.is_none() && !class.is_empty() {
                        pkg.class = Some(class.rsplit('.').next().unwrap_or(&class).to_owned());
                    }
                }
            }
        }
    }
    pkg.names = names;
    pkg.imports = imports;
    pkg.exports = exports;
    Ok(pkg)
}

/// What kind of Hoard asset an Unreal class is.
pub fn kind_of(class: &str) -> AssetKind {
    match class {
        c if c.starts_with("Texture") || c.ends_with("Texture") || c == "Texture2DArray" => AssetKind::Texture,
        "StaticMesh" | "SkeletalMesh" | "GeometryCollection" => AssetKind::Model,
        "AnimSequence"
        | "AnimMontage"
        | "BlendSpace"
        | "BlendSpace1D"
        | "AimOffsetBlendSpace"
        | "PoseAsset"
        | "AnimComposite" => AssetKind::Animation,
        "SoundWave" | "SoundCue" | "MetaSoundSource" | "SoundAttenuation" | "SoundClass" | "SoundMix" => {
            AssetKind::Audio
        }
        "Material"
        | "MaterialInstanceConstant"
        | "MaterialFunction"
        | "MaterialParameterCollection"
        | "PhysicalMaterial" => AssetKind::Material,
        "Font" | "FontFace" => AssetKind::Font,
        _ => AssetKind::Data,
    }
}

/// Plain-word tags for an Unreal class, e.g. `ue:static-mesh`.
pub fn class_tag(class: &str) -> String {
    let mut out = String::from("ue:");
    for (i, ch) in class.chars().enumerate() {
        if ch.is_uppercase() && i > 0 && !out.ends_with(':') {
            let prev = class.chars().nth(i - 1).unwrap_or(' ');
            if prev.is_lowercase() {
                out.push('-');
            }
        }
        out.extend(ch.to_lowercase());
    }
    out
}

/// Find the original source file inside a package: the WAV a `SoundWave` was imported from, or a texture source
/// stored as PNG. Only looks past the header, where bulk data lives (the thumbnail is in the header).
pub fn carve_source(data: &[u8], class: &str, header_size: usize) -> Option<Embedded> {
    let body = data.get(header_size.min(data.len())..)?;
    match class {
        "SoundWave" => find_wav(body).map(|bytes| Embedded { ext: "wav", bytes: bytes.to_vec() }),
        c if c.starts_with("Texture") => {
            // The largest PNG is the source; smaller ones can be mip previews.
            let mut best: Option<&[u8]> = None;
            let mut from = 0;
            while let Some(png) = find_png(&body[from..]) {
                let start = png.as_ptr() as usize - body.as_ptr() as usize;
                from = start + png.len();
                if best.is_none_or(|b| png.len() > b.len()) {
                    best = Some(png);
                }
            }
            best.filter(|b| b.len() > 1024).map(|b| Embedded { ext: "png", bytes: b.to_vec() })
        }
        _ => None,
    }
}

/// A complete RIFF/WAVE file inside `data`, sized by its RIFF header.
fn find_wav(data: &[u8]) -> Option<&[u8]> {
    let mut from = 0;
    while let Some(i) = memfind(&data[from..], b"RIFF") {
        let start = from + i;
        let header = data.get(start..start + 12)?;
        if &header[8..12] == b"WAVE" {
            let size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize + 8;
            if let Some(wav) = data.get(start..start + size)
                && memfind(&wav[12..wav.len().min(4096)], b"fmt ").is_some()
            {
                return Some(wav);
            }
        }
        from = start + 4;
    }
    None
}

/// A complete PNG inside `data`: signature, then chunks up to and including IEND.
fn find_png(data: &[u8]) -> Option<&[u8]> {
    const SIG: &[u8] = b"\x89PNG\r\n\x1a\n";
    let mut from = 0;
    while let Some(i) = memfind(&data[from..], SIG) {
        let start = from + i;
        let mut pos = start + SIG.len();
        while let Some(head) = data.get(pos..pos + 8) {
            let len = u32::from_be_bytes([head[0], head[1], head[2], head[3]]) as usize;
            let end = pos + 12 + len;
            if end > data.len() || !head[4..8].iter().all(u8::is_ascii_alphabetic) {
                break;
            }
            if &head[4..8] == b"IEND" {
                return Some(&data[start..end]);
            }
            pos = end;
        }
        from = start + SIG.len();
    }
    None
}

fn memfind(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

/// `/Game/Pack/Textures/T_Rock` → `Pack/Textures/T_Rock`: the part of a package path that matches files under
/// a project's (or plugin's) `Content/` folder.
pub fn content_relative(package: &str) -> Option<&str> {
    let rest = package.strip_prefix('/')?;
    let (_mount, path) = rest.split_once('/')?;
    (!path.is_empty()).then_some(path)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A minimal UE 5.3-style editor package with `body` as its export data.
    pub(crate) fn synthetic_package(class: &str, asset: &str, deps: &[&str], body: &[u8]) -> Vec<u8> {
        let mut p = crate::testpkg::Synth::new(class, asset, 1009);
        p.deps = deps.iter().map(|d| (*d).to_owned()).collect();
        p.export = body.to_vec();
        p.build()
    }

    #[test]
    fn reads_synthetic_editor_package() {
        let wav = {
            let mut w = b"RIFF".to_vec();
            w.extend_from_slice(&(36u32 + 8).to_le_bytes());
            w.extend_from_slice(b"WAVEfmt ");
            w.extend_from_slice(&16u32.to_le_bytes());
            w.extend_from_slice(&[1, 0, 1, 0, 0x44, 0xac, 0, 0, 0x88, 0x58, 1, 0, 2, 0, 16, 0]);
            w.extend_from_slice(b"data");
            w.extend_from_slice(&8u32.to_le_bytes());
            w.extend_from_slice(&[0u8; 8]);
            w
        };
        let mut body = b"bulk data prefix ".to_vec();
        body.extend_from_slice(&wav);
        body.extend_from_slice(b" trailing");
        let bytes = synthetic_package("SoundWave", "S_Whoosh", &["/Game/Pack/Audio/SC_Whoosh"], &body);
        let pkg = parse(&bytes).unwrap();
        assert_eq!(pkg.engine, "5.3.2");
        assert_eq!(pkg.class.as_deref(), Some("SoundWave"));
        assert_eq!(pkg.asset_name.as_deref(), Some("S_Whoosh"));
        assert_eq!(pkg.dependencies, ["/Game/Pack/Audio/SC_Whoosh"]);
        assert!(pkg.thumbnail.as_deref().unwrap().starts_with(b"\x89PNG"));
        assert_eq!(pkg.tags[0], ("Dimensions".to_owned(), "512x256".to_owned()));
        let src = carve_source(&bytes, "SoundWave", pkg.header_size as usize).unwrap();
        assert_eq!((src.ext, src.bytes.as_slice()), ("wav", wav.as_slice()));
        assert_eq!(kind_of("SoundWave"), AssetKind::Audio);
    }

    #[test]
    fn names_and_paths() {
        assert_eq!(class_tag("StaticMesh"), "ue:static-mesh");
        assert_eq!(class_tag("MaterialInstanceConstant"), "ue:material-instance-constant");
        assert_eq!(class_tag("Texture2D"), "ue:texture2d");
        assert_eq!(content_relative("/Game/Pack/T_Rock"), Some("Pack/T_Rock"));
        assert_eq!(content_relative("/MyPlugin/Meshes/SM_A"), Some("Meshes/SM_A"));
        assert_eq!(kind_of("Texture2D"), AssetKind::Texture);
        assert_eq!(kind_of("StaticMesh"), AssetKind::Model);
    }

    /// Point `HOARD_UASSET_DIR` at a folder of real packages (`UAssetAPI`'s test assets work) to check the parser
    /// against many engine versions. Skipped otherwise.
    #[test]
    fn reads_real_packages_if_available() {
        let Some(dir) = std::env::var_os("HOARD_UASSET_DIR") else { return };
        let mut files = Vec::new();
        let mut stack = vec![std::path::PathBuf::from(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(d).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "uasset" || x == "umap") {
                    files.push(p);
                }
            }
        }
        let (mut ok, mut failed) = (0, Vec::new());
        for f in &files {
            match read(f) {
                Ok(p) => {
                    ok += 1;
                    eprintln!(
                        "ok   {} | {} | {} | deps {} | thumb {} {:02x?} | tags {} {:?}",
                        f.display(),
                        p.engine,
                        p.class.as_deref().unwrap_or("?"),
                        p.dependencies.len(),
                        p.thumbnail.as_ref().map_or(0, Vec::len),
                        p.thumbnail.as_ref().map(|t| t[..4.min(t.len())].to_vec()).unwrap_or_default(),
                        p.tags.len(),
                        p.tags
                            .iter()
                            .take(6)
                            .map(|(k, v)| format!("{k}={}", v.chars().take(40).collect::<String>()))
                            .collect::<Vec<_>>()
                    );
                }
                Err(e) => failed.push(format!("{}: {e:#}", f.display())),
            }
        }
        for f in &failed {
            eprintln!("FAIL {f}");
        }
        eprintln!("{ok} read, {} failed", failed.len());
    }
}
