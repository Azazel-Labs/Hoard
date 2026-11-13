//! 3MF, the 3D-printing format: a zip of XML models. Read here: meshes, components (in the same model or another
//! part of the zip) and the build's placed items; the colours of the 3MF materials extension (base materials and
//! colour groups); and what slicers keep beside them: the filaments' colours, which filament each object and part
//! prints in (Bambu Studio, `OrcaSlicer`, `PrusaSlicer`), and the filaments painted onto triangles.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use anyhow::{Context as _, Result, ensure};
use quick_xml::events::{BytesStart, Event};
use serde_json::{Map, Value, json};
use zip::ZipArchive;

use crate::meshes::{MAX_TRIANGLES, Mesh};

/// A 3MF transform: the first three columns of a 4x4 matrix, row by row (points are row vectors).
type Mat = [f32; 12];
const IDENTITY: Mat = [1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0.];
/// The most read from one part of the zip, unpacked: more is a zip bomb, not a model.
const MAX_PART: u64 = 2 << 30;
/// Slicer settings files are small; anything bigger isn't one.
const MAX_CONFIG: u64 = 64 << 20;
/// Filament colours for a slicer project that assigns filaments without saying what colour they are.
const FALLBACK: [[u8; 3]; 6] = [
    [0xB8, 0xBC, 0xC4],
    [0xD9, 0x48, 0x3B],
    [0x3B, 0x7D, 0xD8],
    [0x4C, 0xAF, 0x50],
    [0xF2, 0xB7, 0x05],
    [0x8E, 0x5B, 0xC4],
];

#[derive(Default)]
struct Tri {
    v: [u32; 3],
    pid: Option<u32>,
    p1: Option<u32>,
    /// The filament painted on it (0: none, the part's own).
    paint: u8,
}

struct Component {
    path: Option<String>,
    id: u32,
    transform: Mat,
}

#[derive(Default)]
struct Object {
    pid: Option<u32>,
    pindex: Option<u32>,
    verts: Vec<[f32; 3]>,
    tris: Vec<Tri>,
    components: Vec<Component>,
}

/// One model file of the zip.
#[derive(Default)]
struct Part {
    objects: HashMap<u32, Object>,
    /// Base materials and colour groups by resource id: each entry's colour (sRGB).
    colors: HashMap<u32, Vec<Option<[u8; 3]>>>,
    build: Vec<(u32, Mat)>,
    application: Option<String>,
}

/// What a slicer project says about filaments: their colours, and which one each object, part (Bambu Studio,
/// `OrcaSlicer`) or range of triangles (`PrusaSlicer`'s volumes) prints in. Filaments count from 1.
#[derive(Default)]
struct Slicer {
    filaments: Vec<[u8; 3]>,
    objects: HashMap<u32, usize>,
    parts: HashMap<(u32, u32), usize>,
    volumes: HashMap<u32, Vec<(u32, u32, usize)>>,
}

impl Slicer {
    fn filament(&self, n: usize) -> [u8; 3] {
        let i = n.saturating_sub(1);
        self.filaments.get(i).copied().unwrap_or(FALLBACK[i % FALLBACK.len()])
    }
}

/// Load a 3MF: one mesh with a material per colour, and the facts worth keeping.
pub(crate) fn load(path: &Path) -> Result<(Option<Mesh>, Map<String, Value>)> {
    let mut zip = ZipArchive::new(File::open(path)?).context("not a 3MF file (it isn't a zip)")?;
    let names: HashMap<String, String> =
        zip.file_names().filter_map(Result::ok).map(|n| (norm(&n), n.into_owned())).collect();
    let root = text(&mut zip, &names, "_rels/.rels", MAX_CONFIG)
        .and_then(|rels| root_target(&rels))
        .map_or_else(|| "3d/3dmodel.model".to_owned(), |t| norm(&t));

    // The root model, then every model file its components point into.
    let mut parts: HashMap<String, Part> = HashMap::new();
    let mut todo = vec![root.clone()];
    let mut triangles = 0;
    while let Some(key) = todo.pop() {
        if parts.contains_key(&key) {
            continue;
        }
        ensure!(parts.len() < 1024, "too many model files in this 3MF");
        let Some(real) = names.get(&key) else {
            ensure!(key != root, "no 3D model in this 3MF");
            continue;
        };
        let reader = zip.by_name(real)?.take(MAX_PART);
        let part = parse_part(reader, &mut triangles).with_context(|| format!("reading {real}"))?;
        for o in part.objects.values() {
            todo.extend(o.components.iter().filter_map(|c| c.path.as_deref()).map(norm));
        }
        parts.insert(key, part);
    }
    let slicer = read_slicer(&mut zip, &names);
    let main = &parts[&root];
    ensure!(!main.build.is_empty(), "this 3MF builds nothing");

    let mut out = Emit { parts: &parts, slicer: &slicer, tris: Vec::new(), colors: Vec::new() };
    for (id, m) in &main.build {
        out.emit(&root, *id, m, *id, slicer.objects.get(id).copied(), 0)?;
    }
    ensure!(!out.tris.is_empty(), "this 3MF has no triangles");

    let mut facts = Map::new();
    facts.insert("triangles".into(), json!(out.tris.len()));
    facts.insert("objects".into(), json!(main.build.len()));
    let distinct: HashSet<[u8; 3]> = out.colors.iter().flatten().copied().collect();
    if distinct.len() > 1 {
        facts.insert("colors".into(), json!(distinct.len()));
    }
    if let Some(app) = main.application.as_ref().filter(|a| !a.is_empty()) {
        facts.insert("application".into(), json!(app));
    }
    let mesh = if out.colors.iter().all(Option::is_none) {
        crate::meshes::gray_soup(&out.tris)
    } else {
        crate::meshes::colored_soup(&out.tris, &out.colors)
    };
    Ok((Some(mesh), facts))
}

/// The build's objects as placed triangles, each with its colour.
struct Emit<'a> {
    parts: &'a HashMap<String, Part>,
    slicer: &'a Slicer,
    tris: Vec<[[f32; 3]; 3]>,
    colors: Vec<Option<[u8; 3]>>,
}

#[expect(clippy::many_single_char_names, reason = "geometry: a/b/c, m, o, t")]
impl Emit<'_> {
    /// Object `id` of model `key` placed by `m`, inside the build item for object `item` (the one slicers' settings
    /// are kept against), printing in `filament` unless its parts or triangles say otherwise.
    fn emit(&mut self, key: &str, id: u32, m: &Mat, item: u32, filament: Option<usize>, depth: u32) -> Result<()> {
        ensure!(depth < 16, "this 3MF's components nest too deep");
        let Some(part) = self.parts.get(key) else { return Ok(()) };
        let Some(o) = part.objects.get(&id) else { return Ok(()) };
        for c in &o.components {
            let sub = c.path.as_deref().map_or_else(|| key.to_owned(), norm);
            let f = self.slicer.parts.get(&(item, c.id)).copied().or(filament);
            self.emit(&sub, c.id, &mul(&c.transform, m), item, f, depth + 1)?;
        }
        let volumes = if depth == 0 { self.slicer.volumes.get(&id) } else { None };
        let mirrored = det(m) < 0.0;
        for (i, t) in o.tris.iter().enumerate() {
            ensure!(self.tris.len() < MAX_TRIANGLES, "too many triangles to preview");
            let [Some(a), Some(b), Some(c)] = t.v.map(|k| o.verts.get(k as usize)) else { continue };
            // 3MF is Z-up (the build plate is XY), glTF Y-up.
            let up = |p: [f32; 3]| {
                let [x, y, z] = apply(m, p);
                [x, z, -y]
            };
            let (a, b, c) = (up(*a), up(*b), up(*c));
            self.tris.push(if mirrored { [a, c, b] } else { [a, b, c] });
            let i = u32::try_from(i).unwrap_or(u32::MAX);
            let volume = volumes.and_then(|vs| vs.iter().find(|v| (v.0..=v.1).contains(&i)).map(|v| v.2));
            let filament = if t.paint > 0 { Some(usize::from(t.paint)) } else { volume.or(filament) };
            let own = t
                .pid
                .or(o.pid)
                .and_then(|g| part.colors.get(&g))
                .and_then(|g| g.get(t.p1.or(o.pindex).unwrap_or(0) as usize).copied().flatten());
            self.colors.push(filament.map(|n| self.slicer.filament(n)).or(own));
        }
        Ok(())
    }
}

fn apply(m: &Mat, p: [f32; 3]) -> [f32; 3] {
    [
        p[0] * m[0] + p[1] * m[3] + p[2] * m[6] + m[9],
        p[0] * m[1] + p[1] * m[4] + p[2] * m[7] + m[10],
        p[0] * m[2] + p[1] * m[5] + p[2] * m[8] + m[11],
    ]
}

/// `a`, then `b`.
fn mul(a: &Mat, b: &Mat) -> Mat {
    let mut c = [0.0; 12];
    for r in 0..4 {
        for col in 0..3 {
            c[r * 3 + col] =
                (0..3).map(|k| a[r * 3 + k] * b[k * 3 + col]).sum::<f32>() + if r == 3 { b[9 + col] } else { 0.0 };
        }
    }
    c
}

fn det(m: &Mat) -> f32 {
    m[0] * (m[4] * m[8] - m[5] * m[7]) - m[1] * (m[3] * m[8] - m[5] * m[6]) + m[2] * (m[3] * m[7] - m[4] * m[6])
}

/// A zip entry name as looked up: no leading slash, lower case (3MF part names aren't case-sensitive).
fn norm(name: &str) -> String {
    name.trim_start_matches('/').to_ascii_lowercase()
}

fn text(zip: &mut ZipArchive<File>, names: &HashMap<String, String>, name: &str, limit: u64) -> Option<String> {
    let mut s = String::new();
    zip.by_name(names.get(&norm(name))?).ok()?.take(limit).read_to_string(&mut s).ok()?;
    Some(s)
}

fn each_attr(e: &BytesStart<'_>, mut f: impl FnMut(&[u8], &[u8])) {
    for a in e.attributes().with_checks(false).flatten() {
        f(a.key.local_name().as_ref(), &a.value);
    }
}

fn attr(e: &BytesStart<'_>, name: &[u8]) -> Option<String> {
    let mut out = None;
    each_attr(e, |k, v| {
        if k == name {
            out = Some(String::from_utf8_lossy(v).into_owned());
        }
    });
    out
}

fn num<T: std::str::FromStr>(v: &[u8]) -> Option<T> {
    std::str::from_utf8(v).ok()?.trim().parse().ok()
}

fn transform(v: &[u8]) -> Mat {
    let n: Vec<f32> = std::str::from_utf8(v).unwrap_or("").split_whitespace().filter_map(|s| s.parse().ok()).collect();
    n.try_into().unwrap_or(IDENTITY)
}

/// `#RRGGBB` or `#RRGGBBAA`.
fn hex_color(s: &str) -> Option<[u8; 3]> {
    let h = s.trim().strip_prefix('#')?;
    if h.len() != 6 && h.len() != 8 {
        return None;
    }
    let byte = |i: usize| u8::from_str_radix(h.get(i..i + 2)?, 16).ok();
    Some([byte(0)?, byte(2)?, byte(4)?])
}

/// The root model's name, from the package's relationships.
fn root_target(rels: &str) -> Option<String> {
    let mut reader = quick_xml::Reader::from_str(rels);
    loop {
        match reader.read_event() {
            Ok(Event::Start(e) | Event::Empty(e)) if e.local_name().as_ref() == b"Relationship" => {
                if attr(&e, b"Type").is_some_and(|t| t.ends_with("/3dmodel")) {
                    return attr(&e, b"Target");
                }
            }
            Ok(Event::Eof) | Err(_) => return None,
            _ => {}
        }
    }
}

fn parse_part(r: impl Read, triangles: &mut usize) -> Result<Part> {
    let mut reader = quick_xml::Reader::from_reader(BufReader::new(r));
    let mut buf = Vec::new();
    let mut part = Part::default();
    let mut object: Option<(u32, Object)> = None;
    let mut group: Option<u32> = None;
    let mut in_build = false;
    let mut app_name = false;
    loop {
        let (e, empty) = match reader.read_event_into(&mut buf)? {
            Event::Start(e) => (e, false),
            Event::Empty(e) => (e, true),
            Event::Text(t) => {
                if app_name {
                    part.application = Some(String::from_utf8_lossy(&t).trim().to_owned());
                    app_name = false;
                }
                buf.clear();
                continue;
            }
            Event::End(e) => {
                match e.local_name().as_ref() {
                    b"object" => {
                        if let Some((id, o)) = object.take() {
                            part.objects.insert(id, o);
                        }
                    }
                    b"basematerials" | b"colorgroup" => group = None,
                    b"build" => in_build = false,
                    b"metadata" => app_name = false,
                    _ => {}
                }
                buf.clear();
                continue;
            }
            Event::Eof => break,
            _ => {
                buf.clear();
                continue;
            }
        };
        match e.local_name().as_ref() {
            b"object" => {
                let mut o = Object::default();
                let mut id = None;
                each_attr(&e, |k, v| match k {
                    b"id" => id = num(v),
                    b"pid" => o.pid = num(v),
                    b"pindex" => o.pindex = num(v),
                    _ => {}
                });
                if let Some(id) = id {
                    if empty {
                        part.objects.insert(id, o);
                    } else {
                        object = Some((id, o));
                    }
                }
            }
            b"vertex" => {
                if let Some((_, o)) = &mut object {
                    let mut p = [0.0f32; 3];
                    each_attr(&e, |k, v| match k {
                        b"x" => p[0] = num(v).unwrap_or(0.0),
                        b"y" => p[1] = num(v).unwrap_or(0.0),
                        b"z" => p[2] = num(v).unwrap_or(0.0),
                        _ => {}
                    });
                    o.verts.push(p);
                }
            }
            b"triangle" => {
                if let Some((_, o)) = &mut object {
                    let mut t = Tri { v: [u32::MAX; 3], ..Tri::default() };
                    each_attr(&e, |k, v| match k {
                        b"v1" => t.v[0] = num(v).unwrap_or(u32::MAX),
                        b"v2" => t.v[1] = num(v).unwrap_or(u32::MAX),
                        b"v3" => t.v[2] = num(v).unwrap_or(u32::MAX),
                        b"pid" => t.pid = num(v),
                        b"p1" => t.p1 = num(v),
                        // Bambu Studio and OrcaSlicer; PrusaSlicer (`slic3rpe:`).
                        b"paint_color" | b"mmu_segmentation" => t.paint = paint_state(v),
                        _ => {}
                    });
                    *triangles += 1;
                    ensure!(*triangles <= MAX_TRIANGLES, "too many triangles to preview");
                    o.tris.push(t);
                }
            }
            b"component" => {
                if let Some((_, o)) = &mut object {
                    let mut c = Component { path: None, id: u32::MAX, transform: IDENTITY };
                    each_attr(&e, |k, v| match k {
                        b"objectid" => c.id = num(v).unwrap_or(u32::MAX),
                        b"transform" => c.transform = transform(v),
                        b"path" => c.path = Some(String::from_utf8_lossy(v).into_owned()),
                        _ => {}
                    });
                    o.components.push(c);
                }
            }
            b"basematerials" | b"colorgroup" => {
                group = attr(&e, b"id").and_then(|v| v.parse().ok()).filter(|_| !empty);
                if let Some(g) = group {
                    part.colors.entry(g).or_default();
                }
            }
            name @ (b"base" | b"color") => {
                if let Some(g) = group {
                    let key: &[u8] = if name == b"base" { b"displaycolor" } else { b"color" };
                    let c = attr(&e, key).and_then(|v| hex_color(&v));
                    part.colors.entry(g).or_default().push(c);
                }
            }
            b"build" => in_build = !empty,
            b"item" if in_build => {
                let (mut id, mut m) = (None, IDENTITY);
                each_attr(&e, |k, v| match k {
                    b"objectid" => id = num(v),
                    b"transform" => m = transform(v),
                    _ => {}
                });
                if let Some(id) = id {
                    part.build.push((id, m));
                }
            }
            b"metadata" => {
                app_name = !empty && attr(&e, b"name").is_some_and(|n| n.eq_ignore_ascii_case("Application"));
            }
            _ => {}
        }
        buf.clear();
    }
    if let Some((id, o)) = object {
        part.objects.insert(id, o);
    }
    Ok(part)
}

/// The filament most of a painted triangle prints in (0: the part's own). Slicers store the painting as a tree of
/// subdivisions in hex, its nibbles read from the end: the low two bits of a node say how many sides it splits (none:
/// a leaf; else into that many plus one), the high two a leaf's filament (3: the next nibble plus 3).
fn paint_state(code: &[u8]) -> u8 {
    fn walk(n: &[u8], pos: &mut usize, w: f32, area: &mut [f32; 19], depth: u32) -> Option<()> {
        let code = *n.get(*pos)?;
        *pos += 1;
        let sides = code & 0b11;
        if sides == 0 {
            let state = if code >> 2 == 3 {
                *pos += 1;
                n.get(*pos - 1)? + 3
            } else {
                code >> 2
            };
            area[usize::from(state)] += w;
        } else {
            if depth > 24 {
                return None;
            }
            let children = sides + 1;
            for _ in 0..children {
                walk(n, pos, w / f32::from(children), area, depth + 1)?;
            }
        }
        Some(())
    }
    let nibbles: Vec<u8> =
        code.iter().rev().filter_map(|&c| char::from(c).to_digit(16)).filter_map(|d| u8::try_from(d).ok()).collect();
    let mut area = [0.0f32; 19];
    walk(&nibbles, &mut 0, 1.0, &mut area, 0);
    let mut best = 0;
    for (i, a) in area.iter().enumerate() {
        if *a > area[best] {
            best = i;
        }
    }
    u8::try_from(best).unwrap_or(0)
}

/// Filament colours and assignments from a slicer project's settings files, when it has them.
fn read_slicer(zip: &mut ZipArchive<File>, names: &HashMap<String, String>) -> Slicer {
    let mut s = Slicer::default();
    // Bambu Studio and OrcaSlicer: JSON.
    if let Some(cfg) = text(zip, names, "Metadata/project_settings.config", MAX_CONFIG)
        && let Ok(v) = serde_json::from_str::<Value>(&cfg)
    {
        s.filaments = v
            .get("filament_colour")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|c| c.as_str().and_then(hex_color).unwrap_or(FALLBACK[0]))
            .collect();
    }
    // PrusaSlicer: `; key = value` lines; an extruder's own colour wins over its filament's.
    if s.filaments.is_empty()
        && let Some(cfg) = text(zip, names, "Metadata/Slic3r_PE.config", MAX_CONFIG)
    {
        let list = |key: &str| -> Vec<Option<[u8; 3]>> {
            cfg.lines()
                .find_map(|l| l.strip_prefix("; ")?.strip_prefix(key)?.trim_start().strip_prefix('='))
                .map(|v| v.trim().trim_matches('"').split(';').map(hex_color).collect())
                .unwrap_or_default()
        };
        let (extruder, filament) = (list("extruder_colour"), list("filament_colour"));
        s.filaments = (0..extruder.len().max(filament.len()))
            .map(|i| {
                extruder.get(i).copied().flatten().or(filament.get(i).copied().flatten()).unwrap_or(FALLBACK[i % 6])
            })
            .collect();
    }
    for name in ["Metadata/model_settings.config", "Metadata/Slic3r_PE_model.config"] {
        if let Some(xml) = text(zip, names, name, MAX_CONFIG) {
            read_assignments(&xml, &mut s);
        }
    }
    s
}

/// `<object id>` with `<metadata key="extruder" value>`, inside it Bambu's `<part id>` and Prusa's
/// `<volume firstid lastid>`, each of which may name its own.
fn read_assignments(xml: &str, s: &mut Slicer) {
    let mut reader = quick_xml::Reader::from_str(xml);
    let (mut object, mut part, mut volume) = (None::<u32>, None::<u32>, None::<(u32, u32)>);
    loop {
        let (e, empty) = match reader.read_event() {
            Ok(Event::Start(e)) => (e, false),
            Ok(Event::Empty(e)) => (e, true),
            Ok(Event::End(e)) => {
                match e.local_name().as_ref() {
                    b"object" => object = None,
                    b"part" => part = None,
                    b"volume" => volume = None,
                    _ => {}
                }
                continue;
            }
            Ok(Event::Eof) | Err(_) => return,
            _ => continue,
        };
        let id = |k: &[u8]| attr(&e, k).and_then(|v| v.parse::<u32>().ok());
        match e.local_name().as_ref() {
            b"object" if !empty => object = id(b"id"),
            b"part" if !empty => part = id(b"id"),
            b"volume" if !empty => volume = id(b"firstid").zip(id(b"lastid")),
            b"metadata" if attr(&e, b"key").as_deref() == Some("extruder") => {
                let (Some(o), Some(n)) = (object, attr(&e, b"value").and_then(|v| v.parse::<usize>().ok())) else {
                    continue;
                };
                if n == 0 {
                    continue;
                }
                if let Some(p) = part {
                    s.parts.insert((o, p), n);
                } else if let Some((a, b)) = volume {
                    s.volumes.entry(o).or_default().push((a, b, n));
                } else {
                    s.objects.insert(o, n);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    fn zip_of(files: &[(&str, &str)]) -> tempfile::NamedTempFile {
        let f = tempfile::Builder::new().suffix(".3mf").tempfile().unwrap();
        let mut zip = zip::ZipWriter::new(f.reopen().unwrap());
        for (name, body) in files {
            zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            zip.write_all(body.as_bytes()).unwrap();
        }
        zip.finish().unwrap();
        f
    }

    const RELS: &str = r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Target="/3D/3dmodel.model" Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/></Relationships>"#;

    const QUAD: &str = r#"<vertices><vertex x="0" y="0" z="0"/><vertex x="10" y="0" z="0"/><vertex x="10" y="10" z="0"/><vertex x="0" y="10" z="0"/></vertices>"#;

    fn colors(mesh: &Mesh) -> Vec<[u8; 3]> {
        let to = |c: f32| (c.powf(1.0 / 2.2) * 255.0).round() as u8;
        mesh.materials.iter().map(|m| [to(m.color[0]), to(m.color[1]), to(m.color[2])]).collect()
    }

    #[test]
    fn paint_states() {
        assert_eq!(paint_state(b"4"), 1);
        assert_eq!(paint_state(b"8"), 2);
        assert_eq!(paint_state(b"0C"), 3);
        assert_eq!(paint_state(b"1C"), 4);
        // Split on three sides into four: filament 1 twice, 2 once, unpainted once.
        assert_eq!(paint_state(b"08443"), 1);
        assert_eq!(paint_state(b""), 0);
    }

    #[test]
    fn base_material_colours_and_build_transforms() {
        let model = format!(
            r##"<?xml version="1.0"?><model unit="millimeter" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02">
            <metadata name="Application">Test Exporter</metadata>
            <resources><basematerials id="1"><base name="Red" displaycolor="#FF0000"/><base name="Blue" displaycolor="#0000FFFF"/></basematerials>
            <object id="2" type="model" pid="1" pindex="0"><mesh>{QUAD}<triangles><triangle v1="0" v2="1" v3="2"/><triangle v1="0" v2="2" v3="3" pid="1" p1="1" p2="1" p3="1"/></triangles></mesh></object></resources>
            <build><item objectid="2" transform="1 0 0 0 1 0 0 0 1 100 0 0"/><item objectid="2"/></build></model>"##
        );
        let f = zip_of(&[("_rels/.rels", RELS), ("3D/3dmodel.model", &model)]);
        let (mesh, facts) = load(f.path()).unwrap();
        let mesh = mesh.unwrap();
        assert_eq!(facts["triangles"], 4);
        assert_eq!(facts["objects"], 2);
        assert_eq!(facts["colors"], 2);
        assert_eq!(facts["application"], "Test Exporter");
        assert_eq!(colors(&mesh), [[255, 0, 0], [0, 0, 255]]);
        // The first item moved 100 along x.
        assert!(mesh.prims[0].positions.iter().any(|p| p[0] >= 100.0));
    }

    #[test]
    fn slicer_filaments_parts_and_paint() {
        // Bambu Studio's layout: the root object's parts live in another model file.
        let root = r#"<?xml version="1.0"?><model unit="millimeter" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02" xmlns:p="http://schemas.microsoft.com/3dmanufacturing/production/2015/06">
            <resources><object id="5" type="model"><components>
              <component p:path="/3D/Objects/object_1.model" objectid="1"/>
              <component p:path="/3D/Objects/object_1.model" objectid="2" transform="1 0 0 0 1 0 0 0 1 0 0 20"/>
            </components></object></resources><build><item objectid="5"/></build></model>"#;
        let parts = format!(
            r#"<?xml version="1.0"?><model unit="millimeter" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02"><resources>
            <object id="1" type="model"><mesh>{QUAD}<triangles><triangle v1="0" v2="1" v3="2"/><triangle v1="0" v2="2" v3="3" paint_color="0C"/></triangles></mesh></object>
            <object id="2" type="model"><mesh>{QUAD}<triangles><triangle v1="0" v2="1" v3="2"/></triangles></mesh></object>
            </resources></model>"#
        );
        let settings = r#"<?xml version="1.0"?><config><object id="5"><metadata key="extruder" value="1"/>
            <part id="1" subtype="normal_part"><metadata key="extruder" value="1"/></part>
            <part id="2" subtype="normal_part"><metadata key="extruder" value="2"/></part></object></config>"#;
        let project = r##"{"filament_colour": ["#FFFFFF", "#FF0000", "#00FF00"]}"##;
        let f = zip_of(&[
            ("_rels/.rels", RELS),
            ("3D/3dmodel.model", root),
            ("3D/Objects/object_1.model", &parts),
            ("Metadata/model_settings.config", settings),
            ("Metadata/project_settings.config", project),
        ]);
        let (mesh, facts) = load(f.path()).unwrap();
        assert_eq!(facts["triangles"], 3);
        // Part 1 in white with one triangle painted green; part 2 in red.
        assert_eq!(colors(&mesh.unwrap()), [[255, 255, 255], [0, 255, 0], [255, 0, 0]]);
    }

    #[test]
    fn prusa_volumes() {
        let model = format!(
            r#"<?xml version="1.0"?><model unit="millimeter" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02"><resources>
            <object id="1" type="model"><mesh>{QUAD}<triangles><triangle v1="0" v2="1" v3="2"/><triangle v1="0" v2="2" v3="3"/></triangles></mesh></object>
            </resources><build><item objectid="1"/></build></model>"#
        );
        let cfg = r#"<?xml version="1.0"?><config><object id="1" instances_count="1"><metadata type="object" key="extruder" value="1"/>
            <volume firstid="0" lastid="0"><metadata type="volume" key="extruder" value="2"/></volume>
            <volume firstid="1" lastid="1"></volume></object></config>"#;
        let settings = "; extruder_colour = \"#0000FF;\"\n; filament_colour = \"#FFFFFF;#FFFF00\"\n";
        let f = zip_of(&[
            ("3D/3dmodel.model", &model),
            ("Metadata/Slic3r_PE_model.config", cfg),
            ("Metadata/Slic3r_PE.config", settings),
        ]);
        let (mesh, _) = load(f.path()).unwrap();
        // Volume 0 in extruder 2 (no colour of its own: its filament's yellow); volume 1 in the object's extruder 1.
        assert_eq!(colors(&mesh.unwrap()), [[255, 255, 0], [0, 0, 255]]);
    }
}
