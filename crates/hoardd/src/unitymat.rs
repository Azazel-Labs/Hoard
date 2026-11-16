//! Unity materials (`.mat`), read into what a Hoard material needs: the textures each property points at (by the
//! GUID their `.meta` gives them), and the colors and numbers they're drawn with. Both of Unity's ways of saving
//! one are read: text (YAML, the default since Unity 2019) and the binary serialized file older projects saved,
//! through the type tree it carries. Only the material's own object is read; nothing is run.

use std::collections::BTreeMap;

use anyhow::{Context, Result, bail, ensure};
use hoard_core::MaterialLook;
use serde_json::Value;

use crate::materials::Role;

/// A Unity material's saved properties.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct UnityMat {
    pub(crate) name: String,
    /// Shader keywords switched on (`_EMISSION`, `_NORMALMAP`).
    pub(crate) keywords: Vec<String>,
    /// Texture properties that point at a texture: (GUID of the texture's file, tiling, offset).
    pub(crate) textures: BTreeMap<String, (String, [f32; 2], [f32; 2])>,
    pub(crate) floats: BTreeMap<String, f32>,
    /// As saved: gamma-space RGBA.
    pub(crate) colors: BTreeMap<String, [f32; 4]>,
}

/// Where the material's smoothness comes from, when a map holds it: Unity keeps it in the alpha of the metallic
/// map, or of the color map, scaled by `_GlossMapScale`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SmoothnessIn {
    pub(crate) role: Role,
    pub(crate) scale: f32,
}

/// The texture properties of Unity's shaders (built-in Standard and legacy, URP Lit, HDRP Lit) as Hoard's roles,
/// the first of each role that's set winning.
const TEXTURES: &[(&str, Role)] = &[
    ("_BaseMap", Role::BaseColor),
    ("_BaseColorMap", Role::BaseColor),
    ("_MainTex", Role::BaseColor),
    ("_BumpMap", Role::Normal),
    ("_NormalMap", Role::Normal),
    ("_MetallicGlossMap", Role::Metalness),
    ("_SpecGlossMap", Role::Specular),
    ("_OcclusionMap", Role::Ao),
    ("_EmissionMap", Role::Emissive),
    ("_EmissiveColorMap", Role::Emissive),
    ("_ParallaxMap", Role::Height),
    ("_HeightMap", Role::Height),
];

/// Read a `.mat` file, text or binary.
pub(crate) fn parse(bytes: &[u8]) -> Result<UnityMat> {
    if bytes.starts_with(b"%YAML") { parse_text(&String::from_utf8_lossy(bytes)) } else { parse_binary(bytes) }
}

impl UnityMat {
    /// The textures by role, as GUIDs of their files.
    pub(crate) fn maps(&self) -> Vec<(Role, &str)> {
        let mut out: Vec<(Role, &str)> = Vec::new();
        for (prop, role) in TEXTURES {
            if let Some((guid, ..)) = self.textures.get(*prop)
                && !out.iter().any(|(r, _)| r == role)
            {
                out.push((*role, guid));
            }
        }
        out
    }

    fn float(&self, names: &[&str]) -> Option<f32> {
        names.iter().find_map(|n| self.floats.get(*n).copied()).filter(|v| v.is_finite())
    }

    fn color(&self, names: &[&str]) -> Option<[f32; 4]> {
        names.iter().find_map(|n| self.colors.get(*n).copied()).filter(|c| c.iter().all(|v| v.is_finite()))
    }

    /// A setting that's a whole number (a mode, a switch), as one.
    #[expect(clippy::cast_possible_truncation, reason = "modes and switches: small whole numbers")]
    fn whole(&self, names: &[&str]) -> Option<i32> {
        self.float(names).map(|v| v.round() as i32)
    }

    fn has(&self, role: Role) -> bool {
        self.maps().iter().any(|(r, _)| *r == role)
    }

    /// Where its smoothness is kept, when a map holds it rather than `_Glossiness`.
    pub(crate) fn smoothness_in(&self) -> Option<SmoothnessIn> {
        let scale = self.float(&["_GlossMapScale"]).unwrap_or(1.0).clamp(0.0, 1.0);
        if self.whole(&["_SmoothnessTextureChannel"]) == Some(1) && self.has(Role::BaseColor) {
            Some(SmoothnessIn { role: Role::BaseColor, scale })
        } else if self.has(Role::Metalness) && !self.floats.contains_key("_Smoothness") {
            Some(SmoothnessIn { role: Role::Metalness, scale })
        } else {
            None
        }
    }

    /// How the material draws its maps. `smoothness` is what the map named by [`Self::smoothness_in`] holds on
    /// average (its alpha), when there is one.
    pub(crate) fn look(&self, smoothness: Option<f32>) -> MaterialLook {
        let mut look = MaterialLook::default();
        if let Some([r, g, b, a]) = self.color(&["_BaseColor", "_Color"]) {
            look.base_color = Some([linear(r), linear(g), linear(b), a.clamp(0.0, 1.0)]);
        }
        if !self.has(Role::Metalness) {
            look.metallic = self.float(&["_Metallic"]).map(|m| m.clamp(0.0, 1.0));
        }
        let smooth = match self.smoothness_in() {
            Some(s) => smoothness.map(|v| v * s.scale),
            None => self.float(&["_Smoothness", "_Glossiness"]),
        };
        look.roughness = smooth.map(|s| 1.0 - s.clamp(0.0, 1.0));
        let glows = self.keywords.iter().any(|k| k == "_EMISSION") || self.colors.contains_key("_EmissiveColor");
        if glows && let Some([r, g, b, _]) = self.color(&["_EmissiveColor", "_EmissionColor"]) {
            let c = [linear(r), linear(g), linear(b)];
            if c.iter().any(|v| *v > 0.0) || self.has(Role::Emissive) {
                look.emissive = Some(c);
            }
        }
        let cutoff = self.float(&["_Cutoff", "_AlphaCutoff"]).map(|c| c.clamp(0.0, 1.0));
        // Standard's rendering mode (cutout, fade, transparent), or URP's and HDRP's surface type and alpha clip.
        let mode = match self.whole(&["_Mode"]) {
            Some(1) => Some("mask"),
            Some(2 | 3) => Some("blend"),
            _ if self.whole(&["_Surface", "_SurfaceType"]) == Some(1) => Some("blend"),
            _ if self.whole(&["_AlphaClip", "_AlphaCutoffEnable"]) == Some(1) => Some("mask"),
            _ => None,
        };
        look.alpha_mode = mode.map(str::to_owned);
        look.alpha_cutoff = cutoff.filter(|_| mode == Some("mask"));
        look.normal_scale =
            self.float(&["_BumpScale", "_NormalScale"]).filter(|s| not_one(*s) && self.has(Role::Normal));
        look.occlusion_strength = self.float(&["_OcclusionStrength"]).filter(|s| not_one(*s) && self.has(Role::Ao));
        let base = ["_BaseMap", "_BaseColorMap", "_MainTex"].iter().find_map(|p| self.textures.get(*p));
        if let Some((_, scale, offset)) = base {
            let fine = |v: &[f32; 2]| v.iter().all(|n| n.is_finite());
            look.uv_scale =
                Some(*scale).filter(|s| s.iter().any(|n| not_one(*n)) && fine(s) && s.iter().all(|n| n.abs() > 1e-6));
            look.uv_offset = Some(*offset).filter(|o| o.iter().any(|n| n.abs() > 1e-6) && fine(o));
        }
        look
    }
}

/// Whether a setting is other than 1, its usual value.
fn not_one(v: f32) -> bool {
    (v - 1.0).abs() > 1e-6
}

/// A gamma-space color channel as linear light (Unity saves colors as picked, in gamma space; HDR ones go past 1).
fn linear(c: f32) -> f32 {
    let c = c.max(0.0);
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

// ---------------------------------------------------------------------------------------------
// Text

/// `{fileID: 2800000, guid: 0123…, type: 3}` as its keys and values.
fn flow_map(s: &str) -> BTreeMap<&str, &str> {
    let inner = s.trim().trim_start_matches('{').trim_end_matches('}');
    inner.split(',').filter_map(|kv| kv.split_once(':')).map(|(k, v)| (k.trim(), v.trim())).collect()
}

fn num(map: &BTreeMap<&str, &str>, key: &str, or: f32) -> f32 {
    map.get(key).and_then(|v| v.parse().ok()).unwrap_or(or)
}

#[derive(PartialEq)]
enum Section {
    None,
    Keywords,
    Textures,
    Floats,
    Colors,
}

/// A texture property being read: its name, and its texture's GUID, tiling and offset so far.
type TexEntry = (String, Option<String>, [f32; 2], [f32; 2]);

fn finish(mat: &mut UnityMat, tex: &mut Option<TexEntry>) {
    if let Some((prop, Some(guid), scale, offset)) = tex.take() {
        mat.textures.insert(prop, (guid, scale, offset));
    }
}

fn parse_text(text: &str) -> Result<UnityMat> {
    let mut mat = UnityMat::default();
    // The material's own document: from `Material:` to the next `--- `.
    let start = text.lines().position(|l| l.trim_end() == "Material:").context("no material in this file")?;
    let mut section = Section::None;
    let mut indent = 0;
    let mut tex: Option<TexEntry> = None;
    for line in text.lines().skip(start + 1) {
        if line.starts_with("--- ") {
            break;
        }
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let here = line.len() - trimmed.len();
        if section != Section::None && here <= indent && !trimmed.starts_with("- ") {
            finish(&mut mat, &mut tex);
            section = Section::None;
        }
        if section == Section::None {
            let (key, value) = trimmed.split_once(':').unwrap_or((trimmed, ""));
            let value = value.trim();
            indent = here;
            match key {
                "m_Name" if here <= 2 => value.clone_into(&mut mat.name),
                "m_ShaderKeywords" => mat.keywords.extend(value.split_whitespace().map(str::to_owned)),
                "m_ValidKeywords" if value.is_empty() => section = Section::Keywords,
                "m_TexEnvs" if value.is_empty() => section = Section::Textures,
                "m_Floats" if value.is_empty() => section = Section::Floats,
                "m_Colors" if value.is_empty() => section = Section::Colors,
                _ => {}
            }
            continue;
        }
        match section {
            Section::Keywords => {
                if let Some(k) = trimmed.strip_prefix("- ") {
                    mat.keywords.push(k.trim().to_owned());
                }
            }
            Section::Textures => {
                if let Some(entry) = trimmed.strip_prefix("- ") {
                    finish(&mut mat, &mut tex);
                    let prop = entry.trim().trim_end_matches(':');
                    tex = Some((prop.to_owned(), None, [1.0, 1.0], [0.0, 0.0]));
                } else if let Some((_, guid, scale, offset)) = tex.as_mut() {
                    let (key, value) = trimmed.split_once(':').unwrap_or((trimmed, ""));
                    let map = flow_map(value);
                    match key {
                        "m_Texture" => {
                            let file = map.get("fileID").copied().unwrap_or("0");
                            *guid =
                                map.get("guid").filter(|g| file != "0" && is_guid(g)).map(|g| g.to_ascii_lowercase());
                        }
                        "m_Scale" => *scale = [num(&map, "x", 1.0), num(&map, "y", 1.0)],
                        "m_Offset" => *offset = [num(&map, "x", 0.0), num(&map, "y", 0.0)],
                        _ => {}
                    }
                }
            }
            Section::Floats => {
                if let Some((k, v)) = trimmed.strip_prefix("- ").and_then(|e| e.split_once(':'))
                    && let Ok(v) = v.trim().parse::<f32>()
                {
                    mat.floats.insert(k.trim().to_owned(), v);
                }
            }
            Section::Colors => {
                if let Some((k, v)) = trimmed.strip_prefix("- ").and_then(|e| e.split_once(':')) {
                    let map = flow_map(v);
                    let c = [num(&map, "r", 0.0), num(&map, "g", 0.0), num(&map, "b", 0.0), num(&map, "a", 1.0)];
                    mat.colors.insert(k.trim().to_owned(), c);
                }
            }
            Section::None => {}
        }
    }
    finish(&mut mat, &mut tex);
    Ok(mat)
}

fn is_guid(g: &str) -> bool {
    g.len() == 32 && g.bytes().all(|b| b.is_ascii_hexdigit()) && !g.bytes().all(|b| b == b'0')
}

// ---------------------------------------------------------------------------------------------
// Binary

/// Hex digits, for GUIDs.
const HEX: &[u8; 16] = b"0123456789abcdef";

/// Unity's class id for a material.
const MATERIAL_CLASS: i32 = 21;

struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
    big: bool,
}

impl<'a> Cursor<'a> {
    fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.b.len()).context("the file ends early")?;
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut a: [u8; N] = self.bytes(N)?.try_into()?;
        if self.big {
            a.reverse();
        }
        Ok(a)
    }

    fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes(1)?[0])
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.array()?))
    }
    fn i16(&mut self) -> Result<i16> {
        Ok(i16::from_le_bytes(self.array()?))
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.array()?))
    }
    fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.array()?))
    }
    fn i64(&mut self) -> Result<i64> {
        Ok(i64::from_le_bytes(self.array()?))
    }
    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.array()?))
    }
    fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.array()?))
    }
    fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.array()?))
    }

    fn cstr(&mut self) -> Result<&'a str> {
        let rest = &self.b[self.at..];
        let n = rest.iter().position(|&c| c == 0).context("unterminated string")?;
        let s = std::str::from_utf8(&rest[..n]).unwrap_or("");
        self.at += n + 1;
        Ok(s)
    }

    fn align(&mut self) {
        self.at = (self.at + 3) & !3;
    }

    /// A count of things at least `each` bytes long, checked against what's left.
    fn count(&mut self, each: usize) -> Result<usize> {
        let n = usize::try_from(self.i32()?).context("a negative count")?;
        ensure!(n.saturating_mul(each.max(1)) <= self.b.len().saturating_sub(self.at), "a count past the file's end");
        Ok(n)
    }
}

/// One field of a type tree.
#[derive(Debug, Clone)]
struct Node {
    level: u8,
    array: bool,
    ty: String,
    name: String,
    align: bool,
}

/// Unity's strings shared by every type tree, named by their offset in this buffer (with the top bit set).
const COMMON_STRINGS: &str = "AABB\0AnimationClip\0AnimationCurve\0AnimationState\0Array\0Base\0BitField\0bitset\0\
bool\0char\0ColorRGBA\0Component\0data\0deque\0double\0dynamic_array\0FastPropertyName\0first\0float\0Font\0\
GameObject\0Generic Mono\0GradientNEW\0GUID\0GUIStyle\0int\0list\0long long\0map\0Matrix4x4f\0MdFour\0MonoBehaviour\0\
MonoScript\0m_ByteSize\0m_Curve\0m_EditorClassIdentifier\0m_EditorHideFlags\0m_Enabled\0m_ExtensionPtr\0\
m_GameObject\0m_Index\0m_IsArray\0m_IsStatic\0m_MetaFlag\0m_Name\0m_ObjectHideFlags\0m_PrefabInternal\0\
m_PrefabParentObject\0m_Script\0m_StaticEditorFlags\0m_Type\0m_Version\0Object\0pair\0PPtr<Component>\0\
PPtr<GameObject>\0PPtr<Material>\0PPtr<MonoBehaviour>\0PPtr<MonoScript>\0PPtr<Object>\0PPtr<Prefab>\0PPtr<Sprite>\0\
PPtr<TextAsset>\0PPtr<Texture>\0PPtr<Texture2D>\0PPtr<Transform>\0Prefab\0Quaternionf\0Rectf\0RectInt\0RectOffset\0\
second\0set\0short\0size\0SInt16\0SInt32\0SInt64\0SInt8\0staticvector\0string\0TextAsset\0TextMesh\0Texture\0\
Texture2D\0Transform\0TypelessData\0UInt16\0UInt32\0UInt64\0UInt8\0unsigned int\0unsigned long long\0\
unsigned short\0vector\0Vector2f\0Vector3f\0Vector4f\0m_ScriptingClassIdentifier\0Gradient\0Type*\0int2_storage\0\
int3_storage\0BoundsInt\0m_CorrespondingSourceObject\0m_PrefabInstance\0m_PrefabAsset\0FileSize\0Hash128\0";

fn tree_string(local: &[u8], offset: u32) -> String {
    let (buf, at) = if offset & 0x8000_0000 == 0 {
        (local, offset as usize)
    } else {
        (COMMON_STRINGS.as_bytes(), (offset & 0x7fff_ffff) as usize)
    };
    let rest = buf.get(at..).unwrap_or_default();
    let end = rest.iter().position(|&c| c == 0).unwrap_or(rest.len());
    String::from_utf8_lossy(&rest[..end]).into_owned()
}

fn parse_binary(bytes: &[u8]) -> Result<UnityMat> {
    let mut c = Cursor { b: bytes, at: 0, big: true };
    let _metadata_size = c.u32()?;
    let _file_size = c.u32()?;
    let version = c.u32()?;
    let mut data_offset = u64::from(c.u32()?);
    ensure!((14..=22).contains(&version), "not a Unity material this reads (serialized file version {version})");
    let big = c.u8()? != 0;
    c.bytes(3)?;
    if version >= 22 {
        c.u32()?;
        c.i64()?;
        data_offset = c.u64()?;
        c.i64()?;
    }
    c.big = big;
    let _unity_version = c.cstr()?;
    c.i32()?;
    let tree = c.u8()? != 0;
    ensure!(tree, "this material was saved without its type tree");
    let node_size = if version >= 19 { 32 } else { 24 };
    let n_types = c.count(20)?;
    let mut types: Vec<(i32, Vec<Node>)> = Vec::with_capacity(n_types);
    for _ in 0..n_types {
        let class = c.i32()?;
        if version >= 16 {
            c.u8()?;
        }
        if version >= 17 {
            c.i16()?;
        }
        if (version < 16 && class < 0) || (version >= 16 && class == 114) {
            c.bytes(16)?;
        }
        c.bytes(16)?;
        let n = c.count(node_size)?;
        let strings_len = usize::try_from(c.i32()?).context("a negative length")?;
        let mut raw = Vec::with_capacity(n);
        for _ in 0..n {
            let _version = c.u16()?;
            let level = c.u8()?;
            let flags = c.u8()?;
            let ty = c.u32()?;
            let name = c.u32()?;
            let _size = c.i32()?;
            let _index = c.i32()?;
            let meta = c.i32()?;
            if version >= 19 {
                c.u64()?;
            }
            raw.push((level, flags, ty, name, meta));
        }
        let strings = c.bytes(strings_len)?;
        let nodes = raw
            .into_iter()
            .map(|(level, flags, ty, name, meta)| Node {
                level,
                array: flags & 1 != 0,
                ty: tree_string(strings, ty),
                name: tree_string(strings, name),
                align: meta & 0x4000 != 0,
            })
            .collect();
        if version >= 21 {
            let deps = c.count(4)?;
            c.bytes(deps * 4)?;
        }
        types.push((class, nodes));
    }
    let n_objects = c.count(20)?;
    let mut objects = Vec::with_capacity(n_objects);
    for _ in 0..n_objects {
        c.align();
        c.i64()?;
        let start = if version >= 22 { c.u64()? } else { u64::from(c.u32()?) };
        let _size = c.u32()?;
        let ty = c.i32()?;
        let index = if version < 16 {
            c.u16()?;
            c.i16()?;
            if version == 15 {
                c.u8()?;
            }
            types.iter().position(|(class, _)| *class == ty)
        } else {
            usize::try_from(ty).ok()
        };
        objects.push((start, index.context("an object of no listed type")?));
    }
    let n_scripts = c.count(12)?;
    for _ in 0..n_scripts {
        c.align();
        c.i32()?;
        c.i64()?;
    }
    let n_externals = c.count(21)?;
    let mut externals = Vec::with_capacity(n_externals);
    for _ in 0..n_externals {
        c.cstr()?;
        let guid = c.bytes(16)?;
        c.i32()?;
        c.cstr()?;
        // Unity writes a GUID's hex digits low nibble first.
        externals.push(
            guid.iter().flat_map(|b| [HEX[usize::from(b & 15)], HEX[usize::from(b >> 4)]]).map(char::from).collect(),
        );
    }
    let (start, index) = objects
        .iter()
        .find(|(_, i)| types.get(*i).is_some_and(|(class, _)| *class == MATERIAL_CLASS))
        .context("no material in this file")?;
    let nodes = &types[*index].1;
    ensure!(!nodes.is_empty(), "the material's type tree is empty");
    let at = usize::try_from(data_offset.checked_add(*start).context("a bad offset")?)?;
    let mut data = Cursor { b: bytes, at, big };
    let value = read_node(&mut data, nodes, 0)?;
    Ok(from_value(&value, &externals))
}

/// The fields directly under node `i`.
fn children(nodes: &[Node], i: usize) -> (Vec<usize>, usize) {
    let mut kids = Vec::new();
    let mut j = i + 1;
    while j < nodes.len() && nodes[j].level > nodes[i].level {
        if nodes[j].level == nodes[i].level + 1 {
            kids.push(j);
        }
        j += 1;
    }
    (kids, j)
}

/// Read one field as JSON: numbers, strings, objects by field name, arrays, and maps as `[key, value]` pairs.
fn read_node(c: &mut Cursor<'_>, nodes: &[Node], i: usize) -> Result<Value> {
    let n = &nodes[i];
    let (kids, _) = children(nodes, i);
    let value = match n.ty.as_str() {
        "string" => {
            let len = c.count(1)?;
            let s = String::from_utf8_lossy(c.bytes(len)?).into_owned();
            let array_aligned = kids.first().is_some_and(|&k| nodes[k].align);
            if array_aligned || n.align {
                c.align();
            }
            return Ok(Value::String(s));
        }
        _ if kids.is_empty() => match n.ty.as_str() {
            "bool" => Value::Bool(c.u8()? != 0),
            "SInt8" => Value::from(c.u8()?.cast_signed()),
            "UInt8" | "char" => Value::from(c.u8()?),
            "SInt16" | "short" => Value::from(c.i16()?),
            "UInt16" | "unsigned short" => Value::from(c.u16()?),
            "SInt32" | "int" => Value::from(c.i32()?),
            "UInt32" | "unsigned int" | "Type*" => Value::from(c.u32()?),
            "SInt64" | "long long" => Value::from(c.i64()?),
            "UInt64" | "unsigned long long" | "FileSize" => Value::from(c.u64()?),
            "float" => Value::from(f64::from(c.f32()?)),
            "double" => Value::from(c.f64()?),
            other => bail!("a field of unknown type {other}"),
        },
        _ if nodes[kids[0]].array => {
            let array = kids[0];
            let (inner, _) = children(nodes, array);
            let &element = inner.get(1).context("an array without elements")?;
            let len = c.count(1)?;
            let mut items = Vec::with_capacity(len.min(4096));
            for _ in 0..len {
                items.push(read_node(c, nodes, element)?);
            }
            if nodes[array].align {
                c.align();
            }
            if n.ty == "map" {
                let pairs = items
                    .into_iter()
                    .map(|mut p| {
                        let key = match p["first"].take() {
                            // Unity 5's FastPropertyName.
                            Value::Object(mut o) => o.remove("name").unwrap_or_default(),
                            k => k,
                        };
                        Value::Array(vec![key, p["second"].take()])
                    })
                    .collect();
                Value::Array(pairs)
            } else {
                Value::Array(items)
            }
        }
        _ => {
            let mut map = serde_json::Map::new();
            for k in kids {
                map.insert(nodes[k].name.clone(), read_node(c, nodes, k)?);
            }
            Value::Object(map)
        }
    };
    if n.align {
        c.align();
    }
    Ok(value)
}

/// The material from its fields, read as JSON.
fn from_value(v: &Value, externals: &[String]) -> UnityMat {
    let f = |v: &Value| v.as_f64().map(|n| n as f32);
    let mut mat = UnityMat { name: v["m_Name"].as_str().unwrap_or_default().to_owned(), ..UnityMat::default() };
    if let Some(k) = v["m_ShaderKeywords"].as_str() {
        mat.keywords.extend(k.split_whitespace().map(str::to_owned));
    }
    if let Some(k) = v["m_ValidKeywords"].as_array() {
        mat.keywords.extend(k.iter().filter_map(Value::as_str).map(str::to_owned));
    }
    let props = &v["m_SavedProperties"];
    let pairs = |key: &str| props[key].as_array().cloned().unwrap_or_default();
    for p in pairs("m_TexEnvs") {
        let (Some(name), env) = (p[0].as_str(), &p[1]) else { continue };
        let file = env["m_Texture"]["m_FileID"].as_i64().unwrap_or(0);
        let Some(guid) = usize::try_from(file - 1).ok().and_then(|i| externals.get(i)).filter(|g| is_guid(g)) else {
            continue;
        };
        let xy = |v: &Value, or: f32| [f(&v["x"]).unwrap_or(or), f(&v["y"]).unwrap_or(or)];
        mat.textures.insert(name.to_owned(), (guid.clone(), xy(&env["m_Scale"], 1.0), xy(&env["m_Offset"], 0.0)));
    }
    for p in pairs("m_Floats") {
        if let (Some(name), Some(n)) = (p[0].as_str(), f(&p[1])) {
            mat.floats.insert(name.to_owned(), n);
        }
    }
    for p in pairs("m_Colors") {
        if let Some(name) = p[0].as_str() {
            let c = &p[1];
            let ch = |k: &str, or: f32| f(&c[k]).unwrap_or(or);
            mat.colors.insert(name.to_owned(), [ch("r", 0.0), ch("g", 0.0), ch("b", 0.0), ch("a", 1.0)]);
        }
    }
    mat
}

#[cfg(test)]
mod tests {
    use super::*;

    const STANDARD: &str = "%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!21 &2100000
Material:
  serializedVersion: 6
  m_Name: Bricks
  m_Shader: {fileID: 46, guid: 0000000000000000f000000000000000, type: 0}
  m_ShaderKeywords: _EMISSION _NORMALMAP
  m_SavedProperties:
    serializedVersion: 3
    m_TexEnvs:
    - _BumpMap:
        m_Texture: {fileID: 2800000, guid: 1111111111111111aaaaaaaaaaaaaaaa, type: 3}
        m_Scale: {x: 1, y: 1}
        m_Offset: {x: 0, y: 0}
    - _DetailAlbedoMap:
        m_Texture: {fileID: 0}
        m_Scale: {x: 1, y: 1}
        m_Offset: {x: 0, y: 0}
    - _MainTex:
        m_Texture: {fileID: 2800000, guid: 2222222222222222BBBBBBBBBBBBBBBB, type: 3}
        m_Scale: {x: 4, y: 2}
        m_Offset: {x: 0.5, y: 0}
    m_Floats:
    - _BumpScale: 0.5
    - _Cutoff: 0.3
    - _Glossiness: 0.25
    - _Metallic: 1
    - _Mode: 1
    m_Colors:
    - _Color: {r: 1, g: 0.5, b: 0, a: 1}
    - _EmissionColor: {r: 1, g: 1, b: 1, a: 1}
--- !u!114 &1
MonoBehaviour:
  m_Name:
";

    #[test]
    fn a_text_material_gives_its_maps_and_settings() {
        let m = parse(STANDARD.as_bytes()).unwrap();
        assert_eq!(m.name, "Bricks");
        assert_eq!(
            m.maps(),
            vec![
                (Role::BaseColor, "2222222222222222bbbbbbbbbbbbbbbb"),
                (Role::Normal, "1111111111111111aaaaaaaaaaaaaaaa")
            ]
        );
        let look = m.look(None);
        let [red, green, blue, alpha] = look.base_color.unwrap();
        assert!((red - 1.0).abs() < 1e-6 && (green - 0.214).abs() < 1e-3);
        assert!(blue.abs() < 1e-6 && (alpha - 1.0).abs() < 1e-6);
        assert_eq!(look.metallic, Some(1.0));
        assert_eq!(look.roughness, Some(0.75));
        assert_eq!(look.emissive, Some([1.0, 1.0, 1.0]));
        assert_eq!(look.alpha_mode.as_deref(), Some("mask"));
        assert_eq!(look.alpha_cutoff, Some(0.3));
        assert_eq!(look.normal_scale, Some(0.5));
        assert_eq!(look.uv_scale, Some([4.0, 2.0]));
        assert_eq!(look.uv_offset, Some([0.5, 0.0]));
    }

    #[test]
    fn emission_needs_its_keyword_and_smoothness_can_live_in_the_metallic_map() {
        let text = STANDARD
            .replace("_EMISSION _NORMALMAP", "_NORMALMAP")
            .replace("    - _MainTex:", "    - _MetallicGlossMap:\n        m_Texture: {fileID: 2800000, guid: 3333333333333333cccccccccccccccc, type: 3}\n    - _MainTex:")
            .replace("    - _Metallic: 1\n", "    - _Metallic: 1\n    - _GlossMapScale: 0.5\n");
        let m = parse(text.as_bytes()).unwrap();
        assert_eq!(m.smoothness_in(), Some(SmoothnessIn { role: Role::Metalness, scale: 0.5 }));
        let look = m.look(Some(0.8));
        assert_eq!(look.emissive, None);
        // The metallic map says how metal it is.
        assert_eq!(look.metallic, None);
        assert!((look.roughness.unwrap() - 0.6).abs() < 1e-6);
    }

    /// A binary material as Unity 5.6 saves it: a type tree for `Material` with one texture, one float and one
    /// color, the texture in a file listed by GUID.
    fn binary_material() -> Vec<u8> {
        let mut strings = Vec::new();
        let mut s = |text: &str| {
            let at = strings.len() as u32;
            strings.extend_from_slice(text.as_bytes());
            strings.push(0);
            at
        };
        let common = |name: &str| {
            let mut at = 0;
            for part in COMMON_STRINGS.split('\0') {
                if part == name {
                    return at | 0x8000_0000;
                }
                at += part.len() as u32 + 1;
            }
            panic!("{name} isn't common")
        };
        // (level, array, type, name, aligned)
        let tree: Vec<(u8, bool, u32, u32, bool)> = vec![
            (0, false, s("Material"), s("Base"), false),
            (1, false, common("string"), common("m_Name"), false),
            (2, true, common("Array"), common("Array"), true),
            (3, false, common("SInt32"), common("size"), false),
            (3, false, common("char"), common("data"), false),
            (1, false, s("UnityPropertySheet"), s("m_SavedProperties"), false),
            (2, false, common("map"), s("m_TexEnvs"), false),
            (3, true, common("Array"), common("Array"), true),
            (4, false, common("int"), common("size"), false),
            (4, false, common("pair"), common("data"), false),
            (5, false, common("FastPropertyName"), common("first"), false),
            (6, false, common("string"), s("name"), false),
            (7, true, common("Array"), common("Array"), true),
            (8, false, common("SInt32"), common("size"), false),
            (8, false, common("char"), common("data"), false),
            (5, false, s("UnityTexEnv"), common("second"), false),
            (6, false, common("PPtr<Texture>"), s("m_Texture"), false),
            (7, false, common("int"), s("m_FileID"), false),
            (7, false, common("SInt64"), s("m_PathID"), false),
            (6, false, common("Vector2f"), s("m_Scale"), false),
            (7, false, common("float"), s("x"), false),
            (7, false, common("float"), s("y"), false),
            (6, false, common("Vector2f"), s("m_Offset"), false),
            (7, false, common("float"), s("x"), false),
            (7, false, common("float"), s("y"), false),
            (2, false, common("map"), s("m_Floats"), false),
            (3, true, common("Array"), common("Array"), true),
            (4, false, common("int"), common("size"), false),
            (4, false, common("pair"), common("data"), false),
            (5, false, common("string"), common("first"), false),
            (6, true, common("Array"), common("Array"), true),
            (7, false, common("SInt32"), common("size"), false),
            (7, false, common("char"), common("data"), false),
            (5, false, common("float"), common("second"), false),
            (2, false, common("map"), s("m_Colors"), false),
            (3, true, common("Array"), common("Array"), true),
            (4, false, common("int"), common("size"), false),
            (4, false, common("pair"), common("data"), false),
            (5, false, common("string"), common("first"), false),
            (6, true, common("Array"), common("Array"), true),
            (7, false, common("SInt32"), common("size"), false),
            (7, false, common("char"), common("data"), false),
            (5, false, common("ColorRGBA"), common("second"), false),
            (6, false, common("float"), s("r"), false),
            (6, false, common("float"), s("g"), false),
            (6, false, common("float"), s("b"), false),
            (6, false, common("float"), s("a"), false),
        ];
        let string = |out: &mut Vec<u8>, text: &str| {
            out.extend((text.len() as i32).to_le_bytes());
            out.extend(text.as_bytes());
            while !out.len().is_multiple_of(4) {
                out.push(0);
            }
        };
        let mut data = Vec::new();
        string(&mut data, "Rust");
        data.extend(1i32.to_le_bytes());
        string(&mut data, "_MainTex");
        data.extend(2i32.to_le_bytes());
        data.extend(2_800_000i64.to_le_bytes());
        for v in [2.0f32, 3.0, 0.0, 0.0] {
            data.extend(v.to_le_bytes());
        }
        data.extend(1i32.to_le_bytes());
        string(&mut data, "_Glossiness");
        data.extend(0.4f32.to_le_bytes());
        data.extend(1i32.to_le_bytes());
        string(&mut data, "_Color");
        for v in [1.0f32, 1.0, 1.0, 0.5] {
            data.extend(v.to_le_bytes());
        }

        let mut meta = Vec::new();
        meta.extend(b"5.6.5p2\0");
        meta.extend(5i32.to_le_bytes());
        meta.push(1);
        meta.extend(1i32.to_le_bytes());
        meta.extend(MATERIAL_CLASS.to_le_bytes());
        meta.push(0);
        meta.extend((-1i16).to_le_bytes());
        meta.extend([0; 16]);
        meta.extend((tree.len() as i32).to_le_bytes());
        meta.extend((strings.len() as i32).to_le_bytes());
        for (level, array, ty, name, aligned) in &tree {
            meta.extend(1u16.to_le_bytes());
            meta.push(*level);
            meta.push(u8::from(*array));
            meta.extend(ty.to_le_bytes());
            meta.extend(name.to_le_bytes());
            meta.extend((-1i32).to_le_bytes());
            meta.extend(0i32.to_le_bytes());
            meta.extend((if *aligned { 0x4000i32 } else { 0 }).to_le_bytes());
        }
        meta.extend(&strings);
        meta.extend(1i32.to_le_bytes());
        while (20 + meta.len()) % 4 != 0 {
            meta.push(0);
        }
        meta.extend(2_100_000i64.to_le_bytes());
        meta.extend(0u32.to_le_bytes());
        meta.extend((data.len() as u32).to_le_bytes());
        meta.extend(0i32.to_le_bytes());
        meta.extend(0i32.to_le_bytes());
        meta.extend(2i32.to_le_bytes());
        for guid in [[0u8; 16], [0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 1, 2, 3, 4, 5, 6, 7, 8]] {
            meta.push(0);
            meta.extend(guid);
            meta.extend(0i32.to_le_bytes());
            meta.extend(b"x\0");
        }
        let data_offset = (20 + meta.len()).next_multiple_of(16);
        let mut file = Vec::new();
        file.extend((meta.len() as u32).to_be_bytes());
        file.extend(((data_offset + data.len()) as u32).to_be_bytes());
        file.extend(17u32.to_be_bytes());
        file.extend((data_offset as u32).to_be_bytes());
        file.extend([0, 0, 0, 0]);
        file.extend(&meta);
        file.resize(data_offset, 0);
        file.extend(&data);
        file
    }

    #[test]
    fn a_binary_material_is_read_through_its_type_tree() {
        let m = parse(&binary_material()).unwrap();
        assert_eq!(m.name, "Rust");
        assert_eq!(m.maps(), vec![(Role::BaseColor, "0123456789abcdef1020304050607080")]);
        assert_eq!(m.textures["_MainTex"].1, [2.0, 3.0]);
        let look = m.look(None);
        assert!((look.roughness.unwrap() - 0.6).abs() < 1e-6);
        assert_eq!(look.base_color, Some([1.0, 1.0, 1.0, 0.5]));
    }

    #[test]
    fn a_cut_short_binary_file_is_an_error_not_a_panic() {
        let file = binary_material();
        for n in [0, 10, 40, 200, file.len() - 3] {
            let _ = parse(&file[..n]);
        }
    }
}
