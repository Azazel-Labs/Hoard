//! Tagged properties: the `UPROPERTY` values an editor package stores for each object, the same data the Details
//! panel shows. Read into a JSON value tree, e.g. a texture's `Source` struct (size, pixel format, compression) or
//! a material instance's parameter overrides.
//!
//! Every tag records its value's size, so values this reader doesn't understand (or misreads) are skipped
//! exactly and can't desynchronize the stream. Covers the UE4 tag layout, UE 5.0-5.3 additions (property
//! extensions) and UE 5.4+ complete type names.

use anyhow::{Context, Result, bail, ensure};
use serde_json::{Map, Value, json};

use crate::package::{Export, Package, ue4, ue5};
use crate::reader::Reader;

/// How deep structs may nest before we give up (real data stays far below this).
const MAX_DEPTH: usize = 24;
const MAX_ELEMENTS: usize = 4_000_000;

/// The properties of one export, and where they ended in the file.
#[derive(Debug, Clone, Default)]
pub struct Properties {
    pub values: Map<String, Value>,
    /// File offset just past the terminating `None`; native serialization (bulk data, …) follows.
    pub end: usize,
}

/// A property type: `ArrayProperty(StructProperty(MaterialParameterInfo))` and so on.
#[derive(Debug, Clone, Default)]
struct Ty {
    name: String,
    params: Vec<Ty>,
}

impl Ty {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), params: Vec::new() }
    }
    fn with(name: impl Into<String>, params: Vec<Ty>) -> Self {
        Self { name: name.into(), params }
    }
    fn param(&self, i: usize) -> Option<&Ty> {
        self.params.get(i)
    }
}

struct Tag {
    name: String,
    ty: Ty,
    size: usize,
    array_index: i32,
    bool_value: bool,
}

struct Ctx<'a> {
    pkg: &'a Package,
}

/// Read the tagged properties at the start of an export.
pub fn read_export(data: &[u8], pkg: &Package, export: &Export) -> Result<Properties> {
    ensure!(!pkg.cooked || pkg.flags & 0x2000 == 0, "cooked packages store unversioned properties");
    let start = usize::try_from(export.serial_offset).context("negative export offset")?;
    let end = start.checked_add(usize::try_from(export.serial_size)?).context("export size overflow")?;
    ensure!(end <= data.len(), "export runs past the end of the file");
    let mut r = Reader::at(&data[..end], start)?;
    if pkg.ue5 >= ue5::PROPERTY_TAG_EXTENSION_AND_OVERRIDABLE_SERIALIZATION {
        // EClassSerializationControlExtension; with OverridableSerializationInformation, the operation follows.
        if r.u8()? & 2 != 0 {
            r.u8()?;
        }
    }
    let ctx = Ctx { pkg };
    let values = read_tags(&mut r, &ctx, end, 0)?;
    Ok(Properties { values, end: r.pos() })
}

/// `DataTable` rows: a count, then (row name, tagged struct) pairs, all within `export`.
pub(crate) fn read_rows(r: &mut Reader<'_>, pkg: &Package, export: &Export) -> Result<Map<String, Value>> {
    let limit = usize::try_from(export.serial_offset + export.serial_size)?;
    let ctx = Ctx { pkg };
    let n = r.count(MAX_ELEMENTS)?;
    let mut rows = Map::new();
    for _ in 0..n {
        let name = fname(r, &ctx)?;
        let row = read_tags(r, &ctx, limit, 1)?;
        rows.insert(name, Value::Object(row));
    }
    Ok(rows)
}

fn fname(r: &mut Reader<'_>, ctx: &Ctx<'_>) -> Result<String> {
    let index = r.i32()?;
    let number = r.i32()?;
    let base = usize::try_from(index).ok().and_then(|i| ctx.pkg.names.get(i)).context("name index out of range")?;
    Ok(if number > 0 { format!("{base}_{}", number - 1) } else { base.clone() })
}

/// Read tags until `None`.
fn read_tags(r: &mut Reader<'_>, ctx: &Ctx<'_>, limit: usize, depth: usize) -> Result<Map<String, Value>> {
    ensure!(depth < MAX_DEPTH, "properties nest too deeply");
    let mut out = Map::new();
    loop {
        let Some(tag) = read_tag(r, ctx)? else { break };
        let start = r.pos();
        let end = start.checked_add(tag.size).context("tag size overflow")?;
        ensure!(end <= limit, "property {} runs past its container", tag.name);
        let value = if tag.ty.name == "BoolProperty" {
            Value::Bool(tag.bool_value)
        } else {
            let mut sub = r.clone();
            read_value(&mut sub, ctx, &tag.ty, tag.size, end, depth, true)
                .unwrap_or_else(|_| json!({ "unread": tag.ty.name, "bytes": tag.size }))
        };
        r.seek(i64::try_from(end)?)?;
        let key = if tag.array_index > 0 { format!("{}[{}]", tag.name, tag.array_index) } else { tag.name };
        out.insert(key, value);
    }
    Ok(out)
}

fn read_tag(r: &mut Reader<'_>, ctx: &Ctx<'_>) -> Result<Option<Tag>> {
    let pkg = ctx.pkg;
    let name = fname(r, ctx)?;
    if name == "None" {
        return Ok(None);
    }
    if pkg.complete_type_names() {
        // UE 5.4+: a pre-order list of (name, inner count) nodes, then size and flags.
        let ty = read_type_name(r, ctx)?;
        let size = usize::try_from(r.i32()?).context("negative property size")?;
        let flags = r.u8()?;
        let array_index = if flags & 0x01 != 0 { r.i32()? } else { 0 };
        if flags & 0x02 != 0 {
            r.guid()?;
        }
        if flags & 0x04 != 0 {
            extensions(r)?;
        }
        return Ok(Some(Tag { name, ty, size, array_index, bool_value: flags & 0x10 != 0 }));
    }
    let type_name = fname(r, ctx)?;
    let size = usize::try_from(r.i32()?).context("negative property size")?;
    let array_index = r.i32()?;
    let mut bool_value = false;
    let ty = match type_name.as_str() {
        "StructProperty" => {
            let s = fname(r, ctx)?;
            if pkg.ue4 >= ue4::STRUCT_GUID_IN_PROPERTY_TAG {
                r.guid()?;
            }
            Ty::with(type_name, vec![Ty::new(s)])
        }
        "BoolProperty" => {
            bool_value = r.u8()? != 0;
            Ty::new(type_name)
        }
        "ByteProperty" | "EnumProperty" => {
            let e = fname(r, ctx)?;
            Ty::with(type_name, vec![Ty::new(e)])
        }
        "ArrayProperty" | "SetProperty" | "OptionalProperty" => {
            let inner = fname(r, ctx)?;
            Ty::with(type_name, vec![Ty::new(inner)])
        }
        "MapProperty" => {
            let k = fname(r, ctx)?;
            let v = fname(r, ctx)?;
            Ty::with(type_name, vec![Ty::new(k), Ty::new(v)])
        }
        _ => Ty::new(type_name),
    };
    if pkg.ue4 >= ue4::PROPERTY_GUID_IN_PROPERTY_TAG && r.u8()? != 0 {
        r.guid()?;
    }
    if pkg.ue5 >= ue5::PROPERTY_TAG_EXTENSION_AND_OVERRIDABLE_SERIALIZATION {
        extensions(r)?;
    }
    Ok(Some(Tag { name, ty, size, array_index, bool_value }))
}

/// Property tag extensions (UE 5.2+): overridable-serialization info.
fn extensions(r: &mut Reader<'_>) -> Result<()> {
    let ext = r.u8()?;
    if ext & 0x02 != 0 {
        r.u8()?; // overridden property operation
        r.i32()?; // experimental overridable logic flag
    }
    Ok(())
}

fn read_type_name(r: &mut Reader<'_>, ctx: &Ctx<'_>) -> Result<Ty> {
    fn build(flat: &[(String, usize)], i: &mut usize) -> Ty {
        let (name, n) = flat[*i].clone();
        *i += 1;
        let params = (0..n).map(|_| build(flat, i)).collect();
        Ty { name, params }
    }
    let mut flat = Vec::new();
    let mut remaining = 1usize;
    while remaining > 0 {
        let name = fname(r, ctx)?;
        let inner = usize::try_from(r.i32()?).context("negative type parameter count")?;
        ensure!(inner < 64 && flat.len() < 256, "implausible property type");
        flat.push((name, inner));
        remaining = remaining - 1 + inner;
    }
    let mut i = 0;
    Ok(build(&flat, &mut i))
}

/// Read one value of type `ty` occupying (up to) `size` bytes and not running past `limit`.
fn read_value(
    r: &mut Reader<'_>,
    ctx: &Ctx<'_>,
    ty: &Ty,
    size: usize,
    limit: usize,
    depth: usize,
    top_level: bool,
) -> Result<Value> {
    let pkg = ctx.pkg;
    Ok(match ty.name.as_str() {
        "BoolProperty" => Value::Bool(r.u8()? != 0),
        "Int8Property" => json!(r.u8()?.cast_signed()),
        "Int16Property" => json!(r.i16()?),
        "IntProperty" => json!(r.i32()?),
        "Int64Property" => json!(r.i64()?),
        "UInt16Property" => json!(r.u16()?),
        "UInt32Property" => json!(r.u32()?),
        "UInt64Property" => json!(r.u64()?),
        "FloatProperty" => json!(f64::from(r.f32()?)),
        "DoubleProperty" => json!(r.f64()?),
        "ByteProperty" => {
            // An enum stored as a name, or a raw byte.
            let enum_name = ty.param(0).map_or("None", |t| t.name.as_str());
            if (top_level && size == 8) || (!top_level && enum_name != "None" && size >= 8) {
                Value::String(fname(r, ctx)?)
            } else {
                json!(r.u8()?)
            }
        }
        "EnumProperty" | "NameProperty" => Value::String(fname(r, ctx)?),
        "StrProperty" => Value::String(r.fstring()?),
        "TextProperty" => read_text(r)?,
        "ObjectProperty" | "ClassProperty" | "WeakObjectProperty" | "InterfaceProperty" => {
            let index = r.i32()?;
            pkg.object_path(index).map_or(Value::Null, Value::String)
        }
        "LazyObjectProperty" => Value::String(hex(&r.guid()?)),
        "SoftObjectProperty" | "SoftClassProperty" => read_soft_path(r, ctx)?,
        "StructProperty" => {
            let name = ty.param(0).map_or("", |t| t.name.as_str());
            read_struct(r, ctx, name, limit, depth)?
        }
        "ArrayProperty" | "SetProperty" => {
            if ty.name == "SetProperty" {
                let removed = r.count(MAX_ELEMENTS)?;
                ensure!(removed == 0, "sets with removed keys aren't supported");
            }
            let count = r.count(MAX_ELEMENTS)?;
            let inner = ty.param(0).cloned().unwrap_or_default();
            let mut items = Vec::with_capacity(count.min(4096));
            if inner.name == "StructProperty" {
                let mut struct_name = inner.param(0).map(|t| t.name.clone()).unwrap_or_default();
                if !pkg.complete_type_names() && pkg.ue4 >= ue4::INNER_ARRAY_TAG_INFO && ty.name == "ArrayProperty" {
                    // A full inner tag describing the element struct.
                    fname(r, ctx)?; // name
                    fname(r, ctx)?; // type
                    r.i32()?; // size
                    r.i32()?; // array index
                    struct_name = fname(r, ctx)?;
                    if pkg.ue4 >= ue4::STRUCT_GUID_IN_PROPERTY_TAG {
                        r.guid()?;
                    }
                    if pkg.ue4 >= ue4::PROPERTY_GUID_IN_PROPERTY_TAG && r.u8()? != 0 {
                        r.guid()?;
                    }
                    if pkg.ue5 >= ue5::PROPERTY_TAG_EXTENSION_AND_OVERRIDABLE_SERIALIZATION {
                        extensions(r)?;
                    }
                }
                for _ in 0..count {
                    items.push(read_struct(r, ctx, &struct_name, limit, depth + 1)?);
                }
            } else {
                // Element size, for byte-vs-name arrays where the tag doesn't say which.
                let elem = size.saturating_sub(4).checked_div(count).unwrap_or(0);
                for _ in 0..count {
                    items.push(read_value(r, ctx, &inner, elem, limit, depth + 1, false)?);
                }
            }
            ensure!(r.pos() <= limit, "array runs past its property");
            Value::Array(items)
        }
        "MapProperty" => {
            let key = ty.param(0).cloned().unwrap_or_default();
            let val = ty.param(1).cloned().unwrap_or_default();
            let removed = r.count(MAX_ELEMENTS)?;
            for _ in 0..removed {
                read_value(r, ctx, &key, 0, limit, depth + 1, false)?;
            }
            let count = r.count(MAX_ELEMENTS)?;
            let mut pairs = Vec::with_capacity(count.min(4096));
            for _ in 0..count {
                let k = read_value(r, ctx, &key, 0, limit, depth + 1, false)?;
                let v = read_value(r, ctx, &val, 0, limit, depth + 1, false)?;
                pairs.push(json!([k, v]));
            }
            Value::Array(pairs)
        }
        other => bail!("{other} isn't read"),
    })
}

fn read_soft_path(r: &mut Reader<'_>, ctx: &Ctx<'_>) -> Result<Value> {
    let pkg = ctx.pkg;
    if !pkg.soft_object_paths.is_empty() {
        let i = usize::try_from(r.i32()?).context("negative soft path index")?;
        return Ok(pkg.soft_object_paths.get(i).cloned().map_or(Value::Null, Value::String));
    }
    let path = if pkg.ue5 >= ue5::FSOFTOBJECTPATH_REMOVE_ASSET_PATH_FNAMES {
        let package = fname(r, ctx)?;
        let asset = fname(r, ctx)?;
        if asset == "None" { package } else { format!("{package}.{asset}") }
    } else {
        fname(r, ctx)?
    };
    let sub = r.fstring()?;
    Ok(Value::String(if sub.is_empty() { path } else { format!("{path}:{sub}") }))
}

/// `FText`: only the common "base" (namespace, key, source string) and invariant forms are decoded.
fn read_text(r: &mut Reader<'_>) -> Result<Value> {
    r.u32()?; // flags
    let history = r.u8()?.cast_signed();
    match history {
        -1 => {
            let has_invariant = r.i32()? != 0;
            Ok(if has_invariant { Value::String(r.fstring()?) } else { Value::String(String::new()) })
        }
        0 => {
            r.fstring()?; // namespace
            r.fstring()?; // key
            Ok(Value::String(r.fstring()?))
        }
        _ => bail!("text history {history} isn't read"),
    }
}

/// Structs with their own binary layout instead of tagged properties.
fn read_struct(r: &mut Reader<'_>, ctx: &Ctx<'_>, name: &str, limit: usize, depth: usize) -> Result<Value> {
    let lwc = ctx.pkg.ue5 >= ue5::LARGE_WORLD_COORDINATES;
    let real = |r: &mut Reader<'_>| -> Result<f64> { if lwc { r.f64() } else { r.f32().map(f64::from) } };
    Ok(match name {
        "Vector" | "Rotator" => {
            let (x, y, z) = (real(r)?, real(r)?, real(r)?);
            if name == "Vector" {
                json!({ "x": x, "y": y, "z": z })
            } else {
                json!({ "pitch": x, "yaw": y, "roll": z })
            }
        }
        "Vector3f" => json!({ "x": r.f32()?, "y": r.f32()?, "z": r.f32()? }),
        "Vector2D" => json!({ "x": real(r)?, "y": real(r)? }),
        "Vector2f" => json!({ "x": r.f32()?, "y": r.f32()? }),
        "Vector4" | "Quat" | "Plane" => json!([real(r)?, real(r)?, real(r)?, real(r)?]),
        "Vector4f" => json!([r.f32()?, r.f32()?, r.f32()?, r.f32()?]),
        "LinearColor" => json!({ "r": r.f32()?, "g": r.f32()?, "b": r.f32()?, "a": r.f32()? }),
        "Color" => {
            let (b, g, red, a) = (r.u8()?, r.u8()?, r.u8()?, r.u8()?);
            json!({ "r": red, "g": g, "b": b, "a": a })
        }
        "IntPoint" => json!([r.i32()?, r.i32()?]),
        "IntVector" => json!([r.i32()?, r.i32()?, r.i32()?]),
        "Guid" => Value::String(hex(&r.guid()?)),
        "DateTime" | "Timespan" => json!(r.i64()?),
        "FrameNumber" => json!(r.i32()?),
        "SoftObjectPath" | "SoftClassPath" => read_soft_path(r, ctx)?,
        "Box" => {
            let min = [real(r)?, real(r)?, real(r)?];
            let max = [real(r)?, real(r)?, real(r)?];
            r.u8()?;
            json!({ "min": min, "max": max })
        }
        "Box2D" => {
            let v = [real(r)?, real(r)?, real(r)?, real(r)?];
            r.u8()?;
            json!(v)
        }
        _ => Value::Object(read_tags(r, ctx, limit, depth + 1)?),
    })
}

fn hex(b: &[u8]) -> String {
    use std::fmt::Write as _;
    b.iter().fold(String::with_capacity(b.len() * 2), |mut s, x| {
        let _ = write!(s, "{x:02x}");
        s
    })
}

/// Look up a nested value by a dotted path (`Source.SizeX`).
pub fn get<'a>(values: &'a Map<String, Value>, path: &str) -> Option<&'a Value> {
    let mut parts = path.split('.');
    let mut cur = values.get(parts.next()?)?;
    for p in parts {
        cur = cur.get(p)?;
    }
    Some(cur)
}
