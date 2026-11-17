//! SketchUp 2021 and later: a ZIP holding `model.dat`, a tree of tagged records (two-byte tag, u32 length,
//! payload), with materials as XML beside it. A port of openskp's `core.cpp`, `tlv.cpp` and `geometry.cpp` read
//! path, keeping only geometry, instances, materials and tags.

use std::collections::HashMap;

use anyhow::{Context, Result, ensure};

use crate::model::{self, Id, Model};
use crate::reader::{f64_at, u32_at};
use crate::zip::Zip;

/// How deep records nest in real files is a handful of levels; the cap keeps hostile ones off the stack.
const MAX_DEPTH: usize = 64;

/// Tags whose payload is more records rather than a value. Everything else is a leaf.
const CONTAINERS: [u16; 41] = [
    0xF401, 0xF701, 0xD430, 0xD530, 0xC832, 0x7C15, 0x8813, 0x8913, 0x8A13, 0x8B13, 0x8C13, 0x8D13, 0x4C1D, 0x6419,
    0xF901, 0x7017, 0x7117, 0xD007, 0xC409, 0x9411, 0x9511, 0x0F01, 0x384A, 0xB80B, 0x9713, 0x2C4C, 0xAC0D, 0xAE0D,
    0xF601, 0xF801, 0x983A, 0x993A, 0x8C3C, 0x8D3C, 0x9013, 0x401F, 0x9113, 0x9213, 0x6942, 0x6C42, 0x6842,
];

/// Tags read as the bytes appear in the file, so `0xF401` is `F4 01`.
fn tag_at(d: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*d.get(at)?, *d.get(at + 1)?]))
}

#[derive(Debug)]
struct Node<'a> {
    tag: u16,
    payload: &'a [u8],
    children: Vec<Node<'a>>,
}

/// The records filling `d`, stopping quietly at one that claims more bytes than remain (real containers can end
/// in bytes that are not records).
fn flat(d: &[u8]) -> Vec<(u16, &[u8])> {
    let mut out = Vec::new();
    let mut p = 0;
    while let (Some(tag), Some(size)) = (tag_at(d, p), u32_at(d, p + 2)) {
        let Some(payload) = d.get(p + 6..(p + 6).saturating_add(size as usize)) else { break };
        out.push((tag, payload));
        p += 6 + payload.len();
    }
    out
}

fn tree(d: &[u8], depth: usize) -> Result<Vec<Node<'_>>> {
    let mut out = Vec::new();
    for (tag, payload) in flat(d) {
        let children = if !payload.is_empty() && CONTAINERS.contains(&tag) {
            ensure!(depth < MAX_DEPTH, "records nest deeper than {MAX_DEPTH} levels");
            tree(payload, depth + 1)?
        } else {
            Vec::new()
        };
        out.push(Node { tag, payload, children });
    }
    Ok(out)
}

/// The first record with `tag` anywhere below `nodes`, depth first.
fn find<'n, 'a>(nodes: &'n [Node<'a>], tag: u16) -> Option<&'n Node<'a>> {
    nodes.iter().find_map(|n| if n.tag == tag { Some(n) } else { find(&n.children, tag) })
}

fn find_all<'n, 'a>(nodes: &'n [Node<'a>], tag: u16, out: &mut Vec<&'n Node<'a>>) {
    for n in nodes {
        if n.tag == tag {
            out.push(n);
        }
        find_all(&n.children, tag, out);
    }
}

/// A little-endian integer of one to eight bytes.
fn varint(p: &[u8]) -> Option<u64> {
    (!p.is_empty() && p.len() <= 8).then(|| p.iter().rev().fold(0, |v, &b| (v << 8) | u64::from(b)))
}

/// An id that may be wrapped in a `DE05` record.
fn wrapped_id(p: &[u8]) -> Option<&[u8]> {
    if p.len() >= 6 && p[..2] == [0xde, 0x05] {
        let z = u32_at(p, 2)? as usize;
        return p.get(6..6usize.checked_add(z)?);
    }
    None
}

fn dc_id(p: &[u8]) -> Option<u64> {
    varint(wrapped_id(p).unwrap_or(p))
}

fn entity_id(n: &Node<'_>) -> Option<Id> {
    for c in &n.children {
        if c.tag == 0xDE05 && !c.payload.is_empty() {
            return varint(c.payload);
        }
        if c.tag == 0xDC05
            && let Some(inner) = wrapped_id(c.payload)
        {
            return varint(inner);
        }
    }
    n.children.iter().find_map(entity_id)
}

fn text(p: &[u8]) -> String {
    String::from_utf8_lossy(p).into_owned()
}

/// The front face's positioned-texture matrix, nested five records deep in the face's `DC05` payload.
fn uv_matrix(p: &[u8]) -> Option<[f64; 9]> {
    let mut cur = p;
    for tag in [0xDD05, 0xB136, 0xB236, 0x1027, 0x1127, 0x1327, 0x1527] {
        cur = flat(cur).into_iter().find(|r| r.0 == tag)?.1;
    }
    if cur.len() != 72 {
        return None;
    }
    let mut m = [0.0; 9];
    for (i, v) in m.iter_mut().enumerate() {
        *v = f64_at(cur, i * 8)?;
    }
    Some(m)
}

/// Draw flags, material and tag from a `D007` drawing-element record.
#[derive(Default)]
struct Drawing {
    material: Option<Id>,
    layer: Option<Id>,
    hidden: bool,
    uv: Option<[f64; 9]>,
}

fn drawing(n: &Node<'_>) -> Drawing {
    let mut out = Drawing::default();
    for d in n.children.iter().filter(|c| c.tag == 0xD007) {
        for x in &d.children {
            match x.tag {
                0xD107 => out.material = varint(x.payload),
                0xD207 => out.layer = varint(x.payload),
                0xD307 => out.hidden = x.payload.first().is_some_and(|f| f & 1 != 0),
                0xDC05 => out.uv = uv_matrix(x.payload),
                _ => {}
            }
        }
    }
    out
}

struct RawFace {
    normal: [f64; 3],
    loops: Vec<Vec<(Id, bool)>>,
    drawing: Drawing,
}

#[derive(Default)]
struct Builder {
    vertices: HashMap<Id, [f64; 3]>,
    edges: HashMap<Id, (Option<Id>, Option<Id>)>,
    faces: Vec<RawFace>,
    instances: Vec<model::Instance>,
}

fn collect_geometry(nodes: &[Node<'_>], b: &mut Builder) {
    for e in nodes {
        match e.tag {
            0xC409 => {
                if let (Some(id), Some(p)) = (entity_id(e), find(&e.children, 0xC509))
                    && let (Some(x), Some(y), Some(z)) =
                        (f64_at(p.payload, 0), f64_at(p.payload, 8), f64_at(p.payload, 16))
                {
                    b.vertices.insert(id, [x, y, z]);
                }
            }
            0xB80B => {
                if let Some(id) = entity_id(e) {
                    let end = |tag| find(&e.children, tag).and_then(|n| varint(n.payload));
                    b.edges.insert(id, (end(0xB90B), end(0xBA0B)));
                }
            }
            0xAC0D => {
                if entity_id(e).is_some() {
                    b.faces.push(face(e));
                }
            }
            0x6419 => {
                if let Some(inst) = instance(e) {
                    b.instances.push(inst);
                }
            }
            _ => collect_geometry(&e.children, b),
        }
    }
}

fn face(e: &Node<'_>) -> RawFace {
    let normal = find(&e.children, 0xAD0D)
        .and_then(|n| Some([f64_at(n.payload, 0)?, f64_at(n.payload, 8)?, f64_at(n.payload, 16)?]))
        .unwrap_or([0.0; 3]);
    let mut loops = Vec::new();
    if let Some(l) = find(&e.children, 0xAE0D) {
        let mut found = Vec::new();
        find_all(&l.children, 0x9411, &mut found);
        for ln in found {
            let mut uses = Vec::new();
            find_all(&ln.children, 0xA00F, &mut uses);
            let co: Vec<(Id, bool)> = uses
                .iter()
                .filter_map(|c| {
                    let fields = flat(c.payload);
                    let edge = fields.iter().find(|f| f.0 == 0xA10F).and_then(|f| varint(f.1))?;
                    let reversed = fields.iter().find(|f| f.0 == 0xA20F).and_then(|f| varint(f.1))? != 0;
                    Some((edge, reversed))
                })
                .collect();
            if !co.is_empty() {
                loops.push(co);
            }
        }
    }
    RawFace { normal, loops, drawing: drawing(e) }
}

fn instance(e: &Node<'_>) -> Option<model::Instance> {
    let definition = find(&e.children, 0x6719).and_then(|r| varint(r.payload))?;
    let name = find(&e.children, 0x6519).map_or_default(|n| text(n.payload));
    let mut matrix = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    if let Some(m) = find(&e.children, 0x6619).filter(|m| m.payload.len() >= 104) {
        for (i, v) in matrix.iter_mut().enumerate() {
            *v = f64_at(m.payload, i * 8)?;
        }
    }
    let d = drawing(e);
    Some(model::Instance { definition, name, matrix, material: d.material, hidden: d.hidden, layer: d.layer })
}

fn to_definition(b: Builder, name: String) -> model::Definition {
    let faces = b
        .faces
        .into_iter()
        .filter_map(|f| {
            let loops: Vec<Vec<[f64; 3]>> = f
                .loops
                .iter()
                .filter_map(|l| {
                    let mut ids: Vec<Id> = Vec::new();
                    for &(edge, reversed) in l {
                        let Some(&(v1, v2)) = b.edges.get(&edge) else { continue };
                        if let Some(id) = if reversed { v2 } else { v1 }
                            && ids.last() != Some(&id)
                        {
                            ids.push(id);
                        }
                    }
                    if ids.len() > 1 && ids.first() == ids.last() {
                        ids.pop();
                    }
                    let points: Option<Vec<[f64; 3]>> = ids.iter().map(|id| b.vertices.get(id).copied()).collect();
                    points.filter(|p| p.len() >= 3)
                })
                .collect();
            (!loops.is_empty()).then_some(model::Face {
                loops,
                normal: f.normal,
                material: f.drawing.material,
                uv_matrix: f.drawing.uv,
                hidden: f.drawing.hidden,
                layer: f.drawing.layer,
            })
        })
        .collect();
    model::Definition { name, faces, instances: b.instances }
}

#[derive(Default)]
struct Collected {
    layers_hidden: Vec<Id>,
    material_names: HashMap<Id, String>,
    definitions: HashMap<Id, model::Definition>,
    root: Option<model::Definition>,
}

fn collect(nodes: &[Node<'_>], out: &mut Collected) {
    for e in nodes {
        match e.tag {
            0x993A => {
                for c in e.children.iter().filter(|c| c.tag == 0x8C3C) {
                    let field = |tag| c.children.iter().find(|x| x.tag == tag);
                    if let Some(id) = field(0xDC05).and_then(|d| dc_id(d.payload))
                        && field(0x8E3C).and_then(|h| h.payload.first()) == Some(&1)
                    {
                        out.layers_hidden.push(id);
                    }
                }
                continue;
            }
            0xC832 => {
                if let (Some(id), Some(name)) =
                    (find(&e.children, 0xDC05).and_then(|d| dc_id(d.payload)), find(&e.children, 0xCC32))
                {
                    out.material_names.insert(id, text(name.payload));
                }
            }
            0x7C15 => {
                let name = e.children.iter().find(|c| c.tag == 0x7E15).map_or_default(|c| text(c.payload));
                let mut b = Builder::default();
                collect_geometry(&e.children, &mut b);
                if let Some(id) = entity_id(e) {
                    out.definitions.insert(id, to_definition(b, name));
                }
            }
            0xF601 => {
                let mut b = Builder::default();
                collect_geometry(&e.children, &mut b);
                out.root = Some(to_definition(b, String::new()));
            }
            _ => {}
        }
        collect(&e.children, out);
    }
}

/// The top-level records of `model.dat`, with the definitions container opened up so each definition is its own
/// record.
fn top_level(d: &[u8]) -> Result<Vec<&[u8]>> {
    let mut p = 0;
    while let Some(size) = u32_at(d, p + 2) {
        ensure!(size as usize <= d.len() - p - 6, "model.dat is truncated");
        p += 6 + size as usize;
    }
    let mut records: Vec<(u16, &[u8], &[u8])> = Vec::new();
    let mut at = 0;
    for (tag, payload) in flat(d) {
        records.push((tag, payload, &d[at..at + 6 + payload.len()]));
        at += 6 + payload.len();
    }
    if let [(0xF401, payload, _)] = records[..] {
        records.clear();
        let mut at = 0;
        for (tag, inner) in flat(payload) {
            records.push((tag, inner, &payload[at..at + 6 + inner.len()]));
            at += 6 + inner.len();
        }
    }
    let mut out = Vec::new();
    for (tag, payload, whole) in records {
        if tag == 0xF901 {
            let defs = (|| {
                let mut level = payload;
                for t in [0x7017, 0x7117] {
                    level = flat(level).into_iter().find(|r| r.0 == t)?.1;
                }
                let mut at = 0;
                let mut defs = Vec::new();
                for (tag, inner) in flat(level) {
                    if tag == 0x7C15 {
                        defs.push(&level[at..at + 6 + inner.len()]);
                    }
                    at += 6 + inner.len();
                }
                Some(defs)
            })();
            if let Some(defs) = defs.filter(|d| !d.is_empty()) {
                out.extend(defs);
                continue;
            }
        }
        out.push(whole);
    }
    Ok(out)
}

pub(crate) fn parse(data: &[u8]) -> Result<Model> {
    let off = (0..data.len().min(4096))
        .find(|&i| data.get(i..i + 4) == Some(b"PK\x03\x04"))
        .context("no ZIP container found")?;
    let zip = Zip::open(&data[off..])?;
    let by_name = materials(&zip);
    let dat = zip.get("model.dat").context("model.dat not found in the ZIP container")??;
    let mut c = Collected::default();
    for record in top_level(&dat)? {
        collect(&tree(record, 0)?, &mut c);
    }
    let mut model = Model { hidden_layers: c.layers_hidden, definitions: c.definitions, ..Model::default() };
    model.root = c.root.context("model.dat has no model record")?;
    for (id, name) in c.material_names {
        if let Some(m) = by_name.get(&name) {
            model.materials.insert(id, m.clone());
        }
    }
    Ok(model)
}

/// Every `materials/<folder>/material.xml`, keyed both by material name and by folder, which is what the
/// model's material records name.
fn materials(zip: &Zip<'_>) -> HashMap<String, model::Material> {
    let mut out = HashMap::new();
    let names: Vec<String> = zip.names().map(str::to_owned).collect();
    for path in names.iter().filter(|n| n.starts_with("materials/") && n.ends_with("material.xml")) {
        let Some(Ok(bytes)) = zip.get(path) else { continue };
        let xml = String::from_utf8_lossy(&bytes);
        let Some(attrs) = element_attrs(&xml, "material") else { continue };
        let get = |k: &str| attr(attrs, k);
        let channel = |k: &str| get(k).and_then(|v| v.trim().parse::<u8>().ok()).unwrap_or(128);
        let alpha = if get("useTrans").as_deref() == Some("1") {
            (1.0 - get("trans").and_then(|t| t.trim().parse::<f64>().ok()).unwrap_or(0.0)).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let folder = path["materials/".len()..].split('/').next().unwrap_or_default().to_owned();
        let texture = element_attrs(&xml, "texture").and_then(|t| {
            let file = attr(t, "textureFilename").unwrap_or_default();
            let scale = |k: &str| attr(t, k).and_then(|v| v.trim().parse::<f64>().ok()).unwrap_or(0.0);
            let bytes = texture_bytes(zip, &names, path, &folder, &file)?;
            Some(model::Texture { bytes, tile: [scale("xScale"), scale("yScale")] })
        });
        let name = get("name").filter(|n| !n.is_empty()).unwrap_or_else(|| "unknown".to_owned());
        let m = model::Material {
            name: name.clone(),
            rgb: [channel("colorRed"), channel("colorGreen"), channel("colorBlue")],
            alpha,
            texture,
        };
        out.insert(folder, m.clone());
        out.insert(name, m);
    }
    out
}

/// The image a material uses: the file it names in its folder, or failing that the folder's only other file.
fn texture_bytes(zip: &Zip<'_>, names: &[String], xml_path: &str, folder: &str, file: &str) -> Option<Vec<u8>> {
    let base = file.rsplit(['/', '\\']).next().unwrap_or(file);
    let prefix = format!("materials/{folder}/");
    let candidates = [format!("{prefix}{file}"), format!("{prefix}{base}")];
    let others = names.iter().filter(|n| {
        let is_xml = std::path::Path::new(n).extension().is_some_and(|e| e.eq_ignore_ascii_case("xml"));
        n.starts_with(&prefix) && n.as_str() != xml_path && !is_xml
    });
    candidates
        .into_iter()
        .chain(others.cloned())
        .find_map(|n| zip.get(&n)?.ok().filter(|b| crate::glb::image_mime(b).is_some()))
}

/// The attribute text of the first `<name ...>` element, allowing a namespace prefix.
fn element_attrs<'x>(xml: &'x str, name: &str) -> Option<&'x str> {
    let mut rest = xml;
    while let Some(open) = rest.find('<') {
        rest = &rest[open + 1..];
        let end = rest.find(|c: char| c.is_whitespace() || c == '>' || c == '/').unwrap_or(rest.len());
        let tag = &rest[..end];
        let local = tag.rsplit(':').next().unwrap_or(tag);
        if local.eq_ignore_ascii_case(name) {
            let close = rest.find('>').unwrap_or(rest.len());
            return Some(&rest[end..close]);
        }
    }
    None
}

/// One `key="value"` attribute, with XML entities decoded.
fn attr(attrs: &str, key: &str) -> Option<String> {
    let mut rest = attrs;
    loop {
        rest = rest.trim_start();
        let eq = rest.find('=')?;
        let k = rest[..eq].trim();
        let after = rest[eq + 1..].trim_start();
        let quote = after.chars().next().filter(|c| *c == '"' || *c == '\'')?;
        let body = &after[1..];
        let close = body.find(quote)?;
        if k.eq_ignore_ascii_case(key) {
            return Some(decode_entities(&body[..close]));
        }
        rest = &body[close + 1..];
    }
}

fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        let Some(semi) = rest.find(';').filter(|&i| i <= 10) else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let name = &rest[1..semi];
        let decoded = match name {
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "apos" => Some('\''),
            "quot" => Some('"'),
            _ => name
                .strip_prefix("#x")
                .or_else(|| name.strip_prefix("#X"))
                .map(|h| u32::from_str_radix(h, 16))
                .or_else(|| name.strip_prefix('#').map(str::parse))
                .and_then(Result::ok)
                .and_then(char::from_u32),
        };
        if let Some(c) = decoded {
            out.push(c);
            rest = &rest[semi + 1..];
        } else {
            out.push('&');
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(tag: u16, payload: &[u8]) -> Vec<u8> {
        let mut out = tag.to_be_bytes().to_vec();
        out.extend((payload.len() as u32).to_le_bytes());
        out.extend(payload);
        out
    }

    #[test]
    fn walks_records_and_stops_at_overlong_ones() {
        let mut d = rec(0xD007, &rec(0xD107, &[5]));
        d.extend(rec(0x1234, &[1, 2]));
        d.extend([0xAA, 0xBB, 0xff, 0xff, 0xff, 0x7f]);
        let t = tree(&d, 0).unwrap();
        assert_eq!(t.len(), 2);
        assert_eq!(find(&t, 0xD107).map(|n| n.payload), Some(&[5][..]));
    }

    #[test]
    fn deep_nesting_is_an_error() {
        let mut d = rec(0x1234, &[]);
        for _ in 0..200 {
            d = rec(0xD007, &d);
        }
        assert!(tree(&d, 0).is_err());
    }

    #[test]
    fn reads_ids() {
        assert_eq!(varint(&[0x34, 0x12]), Some(0x1234));
        assert_eq!(varint(&[]), None);
        assert_eq!(varint(&[0; 9]), None);
        let mut wrapped = vec![0xde, 0x05];
        wrapped.extend(2u32.to_le_bytes());
        wrapped.extend([7, 1]);
        assert_eq!(dc_id(&wrapped), Some(0x107));
        assert_eq!(dc_id(&[9]), Some(9));
    }

    #[test]
    fn reads_material_xml_attributes() {
        let xml = r#"<?xml version="1.0"?><mat:material name="Brick &amp; Mortar" colorRed='200' useTrans="1"
            trans="0.25"><texture textureFilename="brick.jpg" xScale="12"/></mat:material>"#;
        let a = element_attrs(xml, "material").unwrap();
        assert_eq!(attr(a, "name").as_deref(), Some("Brick & Mortar"));
        assert_eq!(attr(a, "colorRed").as_deref(), Some("200"));
        assert_eq!(attr(a, "colorBlue"), None);
        let t = element_attrs(xml, "texture").unwrap();
        assert_eq!(attr(t, "xScale").as_deref(), Some("12"));
        assert_eq!(decode_entities("&lt;auto&gt; &#65;&#x42; &bogus; &"), "<auto> AB &bogus; &");
    }

    #[test]
    fn truncated_model_data_is_an_error() {
        let d = rec(0xF601, &[1, 2, 3]);
        assert!(top_level(&d[..d.len() - 1]).is_err());
        assert_eq!(top_level(&d).unwrap().len(), 1);
    }

    fn id(tag: u16, v: u8) -> Vec<u8> {
        rec(tag, &[v])
    }

    fn vertex(n: u8, p: [f64; 3]) -> Vec<u8> {
        let xyz: Vec<u8> = p.iter().flat_map(|v| v.to_le_bytes()).collect();
        [id(0xDE05, n), rec(0xC509, &xyz)].concat()
    }

    /// A model holding one triangle painted with a textured material, as SketchUp 2021 lays it out.
    fn triangle_file() -> Vec<u8> {
        let mut geometry = Vec::new();
        for (n, p) in [(1, [0.0, 0.0, 0.0]), (2, [10.0, 0.0, 0.0]), (3, [0.0, 10.0, 0.0])] {
            geometry.extend(rec(0xC409, &vertex(n, p)));
        }
        for (n, a, b) in [(11, 1, 2), (12, 2, 3), (13, 3, 1)] {
            geometry.extend(rec(0xB80B, &[id(0xDE05, n), id(0xB90B, a), id(0xBA0B, b)].concat()));
        }
        let co_edges: Vec<u8> = (11..=13).flat_map(|e| rec(0xA00F, &[id(0xA10F, e), id(0xA20F, 0)].concat())).collect();
        let normal: Vec<u8> = [0.0f64, 0.0, 1.0].iter().flat_map(|v| v.to_le_bytes()).collect();
        let face = [
            id(0xDE05, 20),
            rec(0xAD0D, &normal),
            rec(0xAE0D, &rec(0x9411, &co_edges)),
            rec(0xD007, &[id(0xD107, 5), id(0xD207, 7)].concat()),
        ]
        .concat();
        geometry.extend(rec(0xAC0D, &face));
        let material = rec(0xC832, &[rec(0xDC05, &[5]), rec(0xCC32, b"Brick")].concat());
        let layer = rec(0x993A, &rec(0x8C3C, &[rec(0xDC05, &[7]), rec(0x8E3C, &[1])].concat()));
        let dat = rec(0xF401, &[material, layer, rec(0xF601, &geometry)].concat());
        let xml = br#"<material name="Brick" colorRed="200" colorGreen="10" colorBlue="20" useTrans="1" trans="0.5">
            <texture textureFilename="C:\textures\brick.png" xScale="2" yScale="3"/></material>"#;
        let png = b"\x89PNG\r\n\x1a\n0000IHDR";
        let zip = crate::zip::tests::archive_of(
            &[("model.dat", &dat), ("materials/Brick/material.xml", xml), ("materials/Brick/brick.png", png)],
            true,
        );
        [b"header bytes before the container".to_vec(), zip].concat()
    }

    #[test]
    fn reads_a_painted_face_on_a_hidden_tag() {
        let m = parse(&triangle_file()).unwrap();
        assert_eq!(m.root.faces.len(), 1);
        let f = &m.root.faces[0];
        assert_eq!(f.loops, [vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [0.0, 10.0, 0.0]]]);
        assert_eq!((f.material, f.layer), (Some(5), Some(7)));
        assert_eq!(m.hidden_layers, [7]);
        let mat = &m.materials[&5];
        assert_eq!((mat.name.as_str(), mat.rgb, mat.alpha), ("Brick", [200, 10, 20], 0.5));
        let tex = mat.texture.as_ref().unwrap();
        assert_eq!((tex.bytes.len(), tex.tile), (16, [2.0, 3.0]));
    }

    #[test]
    fn damaged_files_fail_cleanly() {
        let data = triangle_file();
        for len in 0..data.len() {
            let _ = parse(&data[..len]);
        }
        let mut data = triangle_file();
        for i in (0..data.len()).step_by(5) {
            data[i] ^= 0x5a;
            let _ = parse(&data);
            data[i] ^= 0x5a;
        }
    }
}
