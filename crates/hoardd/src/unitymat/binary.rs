//! The binary serialized file older Unity projects saved materials as, read through the type tree it carries.

use anyhow::{Context, Result, bail, ensure};
use serde_json::Value;

use super::{UnityMat, is_guid};

/// Hex digits, for GUIDs.
const HEX: &[u8; 16] = b"0123456789abcdef";

/// Unity's class id for a material.
pub(super) const MATERIAL_CLASS: i32 = 21;

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
pub(super) const COMMON_STRINGS: &str = "AABB\0AnimationClip\0AnimationCurve\0AnimationState\0Array\0Base\0BitField\0bitset\0\
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

pub(super) fn parse_binary(bytes: &[u8]) -> Result<UnityMat> {
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
