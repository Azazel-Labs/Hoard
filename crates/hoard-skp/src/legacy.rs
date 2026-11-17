//! Legacy SketchUp files (SketchUp 3 to 2020): an MFC `CArchive` object stream. A port of openskp's `legacy.cpp`
//! read path, keeping only what drawing the model needs; scenes, dimensions and text are still parsed, because
//! the stream has no record lengths and skipping them by size would lose alignment.

use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{Context, Result, anyhow, bail, ensure};

use crate::model::{self, Model};
use crate::reader::{Reader, byte_at, bytes_at, f64_at, marker_at, u16_at, u32_at};

/// SketchUp 3 predates several fields every later file carries.
const FIRST_V4: u32 = 4;
/// The texture image became an object of its own (with a slot) in SketchUp 6.
const FIRST_DIB_OBJECT: u32 = 6;
/// SketchUp 3 and 4 instances carry no name.
const FIRST_NAMED_INSTANCE: u32 = 5;
/// How deep objects may nest inside each other. Real files nest a few levels per component level; the cap keeps a
/// hostile file from exhausting the stack.
const MAX_DEPTH: usize = 160;
/// How deep attribute arrays may nest, for the same reason.
const MAX_ATTR_DEPTH: usize = 16;
/// Widest zero padding seen between SketchUp 2020's filler string and the count after it.
const MAX_V20_FILLER_PAD: usize = 29;
/// Where the throwaway slot-base probes number from, far above any real slot.
const PROBE_BASE: u64 = 1 << 20;

#[derive(Debug, Default, Clone, Copy)]
struct Draw {
    mat: u64,
    hidden: bool,
    layer: u64,
}

#[derive(Debug, Default)]
struct TexBlock {
    dib: Option<u64>,
    inline: Option<(usize, usize)>,
    tile: [f64; 2],
    rgb: [u8; 3],
    opacity: f64,
}

#[derive(Debug)]
struct MatRec {
    name: String,
    rgba: [u8; 4],
    opacity: f64,
    tex: Option<TexBlock>,
}

#[derive(Debug)]
struct FaceRec {
    attrs: Option<u64>,
    draw: Draw,
    normal: [f64; 3],
    loops: Vec<Rc<Value>>,
}

#[derive(Debug)]
struct InstRec {
    draw: Draw,
    def: u64,
    xf: [f64; 13],
    name: String,
}

/// What a read object decodes to. Records that only matter for alignment are `Other`.
#[derive(Debug, Default)]
enum Value {
    #[default]
    Other,
    Vertex([f64; 3]),
    Edge {
        v1: u64,
        v2: u64,
    },
    EdgeUse {
        edge: u64,
        sense: bool,
    },
    Loop(Vec<Rc<Value>>),
    Face(Box<FaceRec>),
    Attrs(Vec<Obj>),
    Layer {
        hidden: bool,
    },
    Material(Box<MatRec>),
    Dib {
        start: usize,
        len: usize,
    },
    Ftc([f64; 9]),
    Definition {
        name: String,
        ents: Vec<Obj>,
    },
    Instance(Box<InstRec>),
}

/// One object reference as read: its slot, class name (empty for a null reference) and value.
#[derive(Debug, Clone)]
struct Obj {
    slot: u64,
    name: Rc<str>,
    v: Option<Rc<Value>>,
}

#[derive(Debug)]
struct Entry {
    cls: bool,
    name: Rc<str>,
    v: Option<Rc<Value>>,
}

/// True when the bytes at `p` are a class reference to `slot`, in either the short or the escaped form.
fn is_class_ref(d: &[u8], p: usize, slot: u64) -> bool {
    if slot <= 0x7fff {
        return u16_at(d, p).is_some_and(|t| u64::from(t) == (0x8000 | slot));
    }
    u16_at(d, p) == Some(0x7fff) && u32_at(d, p + 2).is_some_and(|b| u64::from(b) == (0x8000_0000 | slot))
}

/// Whether the text whose font ends at `pos` is anchored to an entity, which adds a connection block.
fn anchored_text_connection(d: &[u8], pos: usize) -> bool {
    u32_at(d, pos + 20) == Some(4) && u32_at(d, pos + 16).is_some_and(|t| (2..=15).contains(&t))
}

/// The first empty string record within 12 bytes of `count_pos`: SketchUp 2020's filler.
fn v20_filler_marker(d: &[u8], count_pos: usize) -> Option<usize> {
    let at = (count_pos..count_pos + 12).take_while(|i| i + 4 <= d.len()).find(|&i| marker_at(d, i))?;
    (byte_at(d, at + 3) == Some(0)).then_some(at)
}

/// SketchUp 2020's layout of a model with no definitions and no root entities: the filler, 13 zero bytes, then a
/// zero root count. Returns where that count sits.
fn find_empty_root_after_v20_filler(d: &[u8], count_pos: usize) -> Option<usize> {
    let marker = v20_filler_marker(d, count_pos)?;
    let root_count_at = marker + 4 + 13;
    let tail = d.get(marker + 3..root_count_at + 4)?;
    tail.iter().all(|&b| b == 0).then_some(root_count_at)
}

struct Archive<'a> {
    r: Reader<'a>,
    ver: u32,
    pid: bool,
    next: u64,
    base: u64,
    current_loop: u64,
    slots: HashMap<u64, Entry>,
    class_slot: HashMap<String, u64>,
    /// Store-map indices the writer burned without writing bytes (annotation connection points), as
    /// (file band start, width). References past a band run ahead of the walker's numbering.
    burns: Vec<(u64, u64)>,
    cum_delta: u64,
    annot_watermark: Option<u64>,
    burn_stack: Vec<u32>,
    inline_stack: Vec<Vec<u64>>,
    cline_tail: Option<usize>,
    depth: usize,
    empty: Rc<str>,
}

impl std::fmt::Debug for Archive<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Archive").field("p", &self.r.p).field("ver", &self.ver).finish_non_exhaustive()
    }
}

impl<'a> Archive<'a> {
    fn new(d: &'a [u8], ver: u32) -> Self {
        Self {
            r: Reader::new(d),
            ver,
            pid: ver >= 17,
            next: 0,
            base: 0,
            current_loop: 0,
            slots: HashMap::new(),
            class_slot: HashMap::new(),
            burns: Vec::new(),
            cum_delta: 0,
            annot_watermark: None,
            burn_stack: Vec::new(),
            inline_stack: Vec::new(),
            cline_tail: None,
            depth: 0,
            empty: Rc::from(""),
        }
    }

    fn d(&self) -> &'a [u8] {
        self.r.d
    }

    fn null(&self) -> Obj {
        Obj { slot: 0, name: Rc::clone(&self.empty), v: None }
    }

    fn alloc(&mut self, e: Entry) -> u64 {
        let s = self.next;
        self.next += 1;
        self.slots.insert(s, e);
        s
    }

    fn object(&mut self, expect: Option<&str>) -> Result<Obj> {
        let tag = self.r.u16()?;
        match tag {
            0 => Ok(self.null()),
            0x7fff => {
                let big = self.r.u32()?;
                if big & 0x8000_0000 != 0 {
                    self.new_class_ref(u64::from(big & 0x7fff_ffff), expect)
                } else {
                    self.back(u64::from(big))
                }
            }
            0xffff => {
                let _schema = self.r.u16()?;
                let n = self.r.u16()?;
                ensure!(n <= 40, "implausible legacy class name");
                let name: Rc<str> = Rc::from(String::from_utf8_lossy(self.r.raw(usize::from(n))?).as_ref());
                let cs = self.alloc(Entry { cls: true, name: Rc::clone(&name), v: None });
                self.class_slot.insert(name.to_string(), cs);
                self.new_obj(name)
            }
            t if t & 0x8000 != 0 => self.new_class_ref(u64::from(t & 0x7fff), expect),
            t => self.back(u64::from(t)),
        }
    }

    fn new_class_ref(&mut self, s: u64, expect: Option<&str>) -> Result<Obj> {
        if let std::collections::hash_map::Entry::Vacant(slot) = self.slots.entry(s) {
            let name = expect.context("unknown legacy class slot")?;
            slot.insert(Entry { cls: true, name: Rc::from(name), v: None });
            self.class_slot.insert(name.to_owned(), s);
        }
        let entry = self.slots.get(&s).context("unknown legacy class slot")?;
        ensure!(entry.cls, "class ref points to an object");
        let name = Rc::clone(&entry.name);
        self.new_obj(name)
    }

    /// Maps a file store-map index to the walker's numbering through the burn bands; `None` inside a band.
    fn translate_ref(&self, slot: u64) -> Option<u64> {
        let mut offset = 0;
        for &(start, width) in &self.burns {
            if slot < start {
                break;
            }
            if slot < start + width {
                return None;
            }
            offset += width;
        }
        Some(slot - offset)
    }

    fn back(&self, mut s: u64) -> Result<Obj> {
        if self.burns.first().is_some_and(|b| s >= b.0) {
            match self.translate_ref(s) {
                Some(w) => s = w,
                None => return Ok(Obj { slot: s, name: Rc::from("reserved"), v: None }),
            }
        }
        match self.slots.get(&s) {
            None if s < self.base => Ok(Obj { slot: s, name: Rc::from("premodel"), v: None }),
            None => bail!("legacy backref to unknown slot"),
            Some(e) if e.cls => bail!("legacy backref to class"),
            Some(e) => Ok(Obj { slot: s, name: Rc::clone(&e.name), v: e.v.clone() }),
        }
    }

    fn register_burn(&mut self, delta: u64) -> Result<()> {
        let watermark = self.annot_watermark.take().context("burn without an annotation")?;
        self.burns.push((watermark + self.cum_delta, delta));
        self.cum_delta += delta;
        if let Some(credit) = self.burn_stack.last_mut() {
            *credit += 1;
        }
        Ok(())
    }

    /// A reference to an entity that may point forward, past the walk (annotations are written before what they
    /// label). `allow_inline` lets it be a whole new object.
    fn entity_ref(&mut self, allow_inline: bool) -> Result<Option<u64>> {
        let start = self.r.p;
        let tag = self.r.u16()?;
        if tag == 0 {
            return Ok(None);
        }
        if tag == 0x7fff {
            let big = self.r.u32()?;
            if big & 0x8000_0000 == 0 {
                return Ok(Some(u64::from(big)));
            }
        } else if tag != 0xffff && tag & 0x8000 == 0 {
            return Ok(Some(u64::from(tag)));
        }
        ensure!(allow_inline, "entity ref is a new object");
        self.r.p = start;
        Ok(Some(self.object(None)?.slot))
    }

    /// Null, a plain back-reference or the escaped form; a new object is an error here.
    fn plain_entity_ref(&mut self) -> Result<Option<u64>> {
        let tag = self.r.u16()?;
        if tag == 0 {
            return Ok(None);
        }
        if tag == 0x7fff {
            let big = self.r.u32()?;
            ensure!(big & 0x8000_0000 == 0, "entity ref is a new object");
            return Ok(Some(u64::from(big)));
        }
        ensure!(tag != 0xffff && tag & 0x8000 == 0, "entity ref is a new object");
        Ok(Some(u64::from(tag)))
    }

    /// The instance paths after an annotation's connection ref: a ref, then two counted ref lists. MFC writes an
    /// object in full the first time anything points at it, so an entry can be a whole new object.
    fn connection_paths(&mut self) -> Result<()> {
        self.plain_entity_ref()?;
        let mut firsts = Vec::new();
        for _ in 0..2 {
            let count = self.r.u32()?;
            ensure!(count <= 1000, "implausible dimension path length");
            for i in 0..count {
                let tag = self.r.peek_u16()?;
                let is_new = if tag == 0x7fff {
                    u32_at(self.d(), self.r.p + 2).context("archive ends early")? & 0x8000_0000 != 0
                } else {
                    tag == 0xffff || tag & 0x8000 != 0
                };
                if is_new {
                    let slot = self.object(None)?.slot;
                    if i == 0 {
                        firsts.push(slot);
                    }
                } else {
                    self.plain_entity_ref()?;
                }
            }
        }
        if let Some(stack) = self.inline_stack.last_mut() {
            stack.extend(firsts);
        }
        Ok(())
    }

    /// The walker slot of the back-reference at the cursor, without moving it.
    fn peek_backref(&self) -> Option<u64> {
        let d = self.d();
        let mut slot = u64::from(u16_at(d, self.r.p)?);
        if slot == 0x7fff {
            slot = u64::from(u32_at(d, self.r.p + 2)?);
            if slot & 0x8000_0000 != 0 {
                return None;
            }
        } else if slot == 0 || slot & 0x8000 != 0 {
            return None;
        }
        if self.burns.first().is_some_and(|b| slot >= b.0) {
            return self.translate_ref(slot);
        }
        Some(slot)
    }

    /// Whether the u16 at `at` starts an object read in an unambiguous form.
    fn strict_next_tag(&self, at: usize, allow_null: bool) -> bool {
        match u16_at(self.d(), at) {
            Some(0) => allow_null,
            Some(0x7fff | 0xffff) => true,
            Some(t) if t & 0x8000 != 0 => self.slots.get(&u64::from(t & 0x7fff)).is_some_and(|e| e.cls),
            None | Some(_) => false,
        }
    }

    /// The tag at `at` can legally start an object read, back-references within the allocated range included.
    fn plausible_list_tag(&self, at: usize) -> bool {
        match u16_at(self.d(), at) {
            None => false,
            Some(0 | 0x7fff | 0xffff) => true,
            Some(t) if t & 0x8000 != 0 => self.slots.get(&u64::from(t & 0x7fff)).is_some_and(|e| e.cls),
            Some(t) => u64::from(t) < self.next,
        }
    }

    /// SketchUp 2020 writes an undocumented filler (an empty string then zero padding) ahead of some counts, which
    /// leaves the reader early. Finds the real count behind it and moves the cursor past it.
    fn retry_v20(&mut self, count_pos: usize, limit: u32) -> Option<u32> {
        let d = self.d();
        let marker = v20_filler_marker(d, count_pos)?;
        for pad in (1..=MAX_V20_FILLER_PAD).step_by(4) {
            let at = marker + 4 + pad;
            let count = u32_at(d, at)?;
            if count > 0 && count <= limit && self.plausible_list_tag(at + 4) {
                self.r.p = at + 4;
                return Some(count);
            }
        }
        None
    }

    /// The entity preamble: an attribute-container reference, then from SketchUp 2017 a persistent id. Returns
    /// the container's slot and the class the reference named.
    fn preamble(&mut self) -> Result<(Option<u64>, Rc<str>)> {
        let attrs = self.object(Some("CAttributeContainer"))?;
        if self.pid {
            let mask = self.r.u8()?;
            self.r.skip(mask.count_ones() as usize)?;
        }
        let slot = (!attrs.name.is_empty()).then_some(attrs.slot);
        Ok((slot, attrs.name))
    }

    /// The preamble for records that carry none in SketchUp 3: vertex, loop, edge use, material, layer.
    fn early_preamble(&mut self) -> Result<Option<u64>> {
        if self.ver < FIRST_V4 {
            return Ok(None);
        }
        Ok(self.preamble()?.0)
    }

    fn draw(&mut self) -> Result<Draw> {
        let v3 = self.ver < FIRST_V4;
        let b = self.r.raw(if v3 {
            5
        } else if self.ver < 5 {
            7
        } else {
            8
        })?;
        let mut out = Draw::default();
        if !v3 {
            out.mat = u64::from(u16::from_le_bytes([b[0], b[1]]));
            out.hidden = b[2] != 0;
        }
        // The layer is normally a u16 id, but can be the layer object itself: inline on first use, an escaped
        // back-reference after.
        let tag = self.r.peek_u16()?;
        let lay_cs = self.class_slot.get("CLayer").copied();
        if lay_cs.is_some_and(|cs| u64::from(tag) == (0x8000 | cs)) {
            self.object(Some("CLayer"))?;
        } else if tag == 0x7fff {
            self.r.u16()?;
            ensure!(self.r.u32()? & 0x8000_0000 == 0, "drawbase layer: unexpected class");
        } else {
            out.layer = u64::from(self.r.u16()?);
        }
        Ok(out)
    }

    /// The textured-material payload shared by a textured `CMaterial` and a colour-by-layer `CLayer`.
    fn texture_block(&mut self) -> Result<TexBlock> {
        let mut t = TexBlock::default();
        if self.ver < FIRST_DIB_OBJECT {
            self.r.u32()?;
            let z = self.r.u32()? as usize;
            ensure!(z <= self.d().len(), "implausible dib length");
            t.inline = Some((self.r.p, z));
            self.r.skip(z)?;
        } else {
            // The caller read the flag as a u16; its high byte opens the texture's own preamble.
            self.r.p = self.r.p.checked_sub(1).context("archive offset underflows")?;
            self.preamble()?;
            let dib = self.object(Some("CDib"))?;
            t.dib = (!dib.name.is_empty()).then_some(dib.slot);
        }
        let begin = self.r.p;
        let limit = self.d().len().min(begin + 28);
        let gap =
            (begin..limit.saturating_sub(2)).find(|&m| marker_at(self.d(), m)).map_or(limit - begin, |m| m - begin);
        match gap {
            20 => {
                self.r.u32()?;
            }
            16 => {}
            _ => bail!("texture size block misaligned"),
        }
        t.tile = self.r.f64s::<2>()?;
        self.r.utf16()?;
        let c = self.r.raw(9)?;
        t.rgb = [c[5], c[6], c[7]];
        self.r.utf16()?;
        self.r.skip(8)?;
        t.opacity = self.r.f64()?;
        if self.r.u8()? == 0 {
            t.opacity = 0.0;
        }
        Ok(t)
    }

    /// A `CLayer` record at `at`: its class declaration, a reference to its known class, or (for the throwaway
    /// probe) any short class reference.
    fn clayer_record_at(&self, at: usize, unmatched_class_ref: bool) -> bool {
        let d = self.d();
        let Some(tag) = u16_at(d, at) else { return false };
        if tag == 0xffff && u16_at(d, at + 4) == Some(6) && bytes_at(d, at + 6, b"CLayer") {
            return true;
        }
        if self.class_slot.get("CLayer").is_some_and(|&cs| is_class_ref(d, at, cs)) {
            return true;
        }
        unmatched_class_ref && tag & 0x8000 != 0 && tag != 0xffff
    }

    /// Some v18 files follow a colour layer with 12 zero bytes and a u32 before the next `CLayer`.
    fn skip_clayer_colour_ext(&mut self) {
        let p = self.r.p;
        let Some(pad) = self.d().get(p..p + 18) else { return };
        if pad[..12].iter().all(|&b| b == 0) && self.clayer_record_at(p + 16, true) {
            self.r.p += 16;
        }
    }

    /// v18's Layer0 after a custom tag carries a parent reference; consume it only when it and the word after it
    /// both point at layers.
    fn skip_clayer_parent_ref(&mut self, me: u64) {
        let d = self.d();
        let (Some(a), Some(b)) = (u16_at(d, self.r.p), u16_at(d, self.r.p + 2)) else { return };
        let plain = |t: u16| t != 0 && t & 0x8000 == 0 && t != 0x7fff;
        if !plain(a) || !plain(b) || u64::from(a) == me {
            return;
        }
        let is_layer = |t: u16| self.slots.get(&u64::from(t)).is_some_and(|e| !e.cls && &*e.name == "CLayer");
        if is_layer(a) && is_layer(b) {
            self.r.p += 2;
        }
    }

    /// Layer records past the declared count (and null separators), up to the definition-list anchor.
    fn collect_trailing_layers(&mut self, out: &mut Vec<Obj>, unmatched_class_ref: bool) -> Result<()> {
        while let Some(tag) = u16_at(self.d(), self.r.p) {
            if tag == 0 {
                self.r.p += 2;
                continue;
            }
            if !self.clayer_record_at(self.r.p, unmatched_class_ref) {
                break;
            }
            let q = self.object(Some("CLayer"))?;
            if q.v.is_some() {
                out.push(q);
            }
        }
        Ok(())
    }

    fn new_obj(&mut self, name: Rc<str>) -> Result<Obj> {
        let slot = self.alloc(Entry { cls: false, name: Rc::clone(&name), v: None });
        ensure!(self.depth < MAX_DEPTH, "objects nest too deeply");
        self.depth += 1;
        let read = self.read(&name, slot);
        self.depth -= 1;
        let v = Rc::new(read?);
        if let Some(e) = self.slots.get_mut(&slot) {
            e.v = Some(Rc::clone(&v));
        }
        if matches!(&*name, "CDimensionLinear" | "CText") {
            self.annot_watermark = Some(self.next);
        }
        Ok(Obj { slot, name, v: Some(v) })
    }

    fn read(&mut self, name: &str, me: u64) -> Result<Value> {
        match name {
            "CVertex" => {
                self.early_preamble()?;
                Ok(Value::Vertex(self.r.f64s::<3>()?))
            }
            "CEdge" => {
                self.preamble()?;
                self.draw()?;
                let v1 = self.object(Some("CVertex"))?.slot;
                let v2 = self.object(Some("CVertex"))?.slot;
                self.object(None)?; // the curve the edge belongs to
                Ok(Value::Edge { v1, v2 })
            }
            "CCurve" => {
                self.preamble()?;
                self.r.skip(5)?;
                Ok(Value::Other)
            }
            "CArcCurve" => {
                self.preamble()?;
                self.r.skip(5 + 14 * 8)?;
                Ok(Value::Other)
            }
            "CEdgeUse" => self.read_edge_use(),
            "CLoop" => self.read_loop(me),
            "CFace" => self.read_face(),
            "CAttributeContainer" => {
                self.preamble()?;
                let mut ents = Vec::new();
                while u16_at(self.d(), self.r.p).is_some_and(|t| t != 0) {
                    ents.push(self.object(Some("CAttributeNamed"))?);
                }
                self.r.u16()?;
                Ok(Value::Attrs(ents))
            }
            "CAttributeNamed" => self.read_attribute_named(),
            "CLayer" => self.read_layer(me),
            "CMaterial" => self.read_material(),
            "CDib" => {
                self.r.u32()?;
                let len = self.r.u32()? as usize;
                let start = self.r.p;
                self.r.skip(len)?;
                Ok(Value::Dib { start, len })
            }
            "CFaceTextureCoords" => self.read_texture_coords(),
            "CCamera" => {
                self.r.skip(137)?;
                self.r.u16()?;
                self.r.utf16()?;
                self.r.skip(if self.ver >= 6 { 33 } else { 10 })?;
                Ok(Value::Other)
            }
            "CThumbnail" => {
                self.preamble()?;
                if self.ver < FIRST_V4 {
                    self.r.skip(129)?;
                } else {
                    self.object(Some("CCamera"))?;
                    if self.ver >= 6 {
                        self.object(Some("CDib"))?;
                    }
                }
                Ok(Value::Other)
            }
            "CImage" => self.read_image(),
            "CRelationship" => {
                self.preamble()?;
                self.entity_ref(false)?;
                self.entity_ref(false)?;
                Ok(Value::Other)
            }
            "CConstructionLine" => self.read_construction_line(),
            "CConstructionPoint" => {
                self.preamble()?;
                self.draw()?;
                self.r.skip(6 * 8 + 1)?;
                Ok(Value::Other)
            }
            "CSectionPlane" => self.read_section_plane(),
            "CSkFont" => {
                self.object(Some("CAttributeContainer"))?;
                if self.pid {
                    let mask = self.r.u8()?;
                    self.r.skip(mask.count_ones() as usize)?;
                }
                self.r.utf16()?;
                self.r.skip(15)?;
                Ok(Value::Other)
            }
            "CDimensionLinear" => self.read_dimension(),
            "CText" => self.read_text(),
            "CComponentDefinition" => self.read_definition(),
            "CComponentInstance" | "CGroup" => self.read_instance(),
            other => bail!("no legacy reader for {other}"),
        }
    }

    fn read_edge_use(&mut self) -> Result<Value> {
        self.early_preamble()?;
        let edge = self.object(Some("CEdge"))?.slot;
        let sense = self.r.u8()? != 0;
        // The parent-loop back-reference keeps the walk honest. After annotations it can sit ahead of the walker's
        // numbering by the indices the writer burned, which is a correction, not a misparse.
        let p0 = self.r.p;
        let tag = self.r.u16()?;
        let parent = if tag == 0x7fff {
            let big = self.r.u32()?;
            ensure!(big & 0x8000_0000 == 0, "edge-use parent is a new object");
            Some(u64::from(big))
        } else {
            ensure!(tag != 0xffff && tag & 0x8000 == 0, "edge-use parent is a new object");
            (tag != 0).then_some(u64::from(tag))
        };
        let expected = self.current_loop + self.cum_delta;
        if parent != Some(expected) {
            let delta = parent.and_then(|p| p.checked_sub(expected)).unwrap_or(0);
            if delta > 0 && delta <= 4096 && self.annot_watermark.is_some() {
                self.register_burn(delta)?;
            } else {
                self.r.p = p0;
                bail!("edge-use parent mismatch");
            }
        }
        Ok(Value::EdgeUse { edge, sense })
    }

    fn read_loop(&mut self, me: u64) -> Result<Value> {
        let old = self.current_loop;
        self.current_loop = me;
        let uses = self.read_loop_body();
        self.current_loop = old;
        Ok(Value::Loop(uses?))
    }

    fn read_loop_body(&mut self) -> Result<Vec<Rc<Value>>> {
        self.early_preamble()?;
        self.r.skip(2)?;
        let mut uses = Vec::new();
        while u16_at(self.d(), self.r.p).is_some_and(|t| t != 0) {
            if let Some(v) = self.object(Some("CEdgeUse"))?.v {
                uses.push(v);
            }
        }
        self.r.u16()?;
        Ok(uses)
    }

    fn read_face(&mut self) -> Result<Value> {
        let (mut attrs, lead_cls) = self.preamble()?;
        let mut draw = self.draw()?;
        if self.ver < FIRST_V4 && &*lead_cls == "CMaterial" {
            // SketchUp 3 opens a face with its front material, not an attribute container.
            if let Some(m) = attrs.take() {
                draw.mat = m;
            }
        }
        let plane = self.r.f64s::<4>()?;
        let count = self.r.u32()?;
        ensure!(count <= 10_000, "implausible loop count");
        let mut loops = Vec::new();
        for _ in 0..count {
            if let Some(v) = self.object(Some("CLoop"))?.v {
                loops.push(v);
            }
        }
        self.r.u16()?; // back material
        Ok(Value::Face(Box::new(FaceRec { attrs, draw, normal: [plane[0], plane[1], plane[2]], loops })))
    }

    fn read_attribute_named(&mut self) -> Result<Value> {
        self.preamble()?;
        self.r.skip(4)?;
        self.r.utf16()?;
        loop {
            if self.r.utf16()?.is_empty() {
                break;
            }
            let t = self.r.u8()?;
            self.skip_typed(t, 0)?;
        }
        // SketchUp 6 ends the record at the empty key; 7 added a u32.
        if self.ver >= 7 {
            self.r.u32()?;
        }
        Ok(Value::Other)
    }

    fn skip_typed(&mut self, t: u8, depth: usize) -> Result<()> {
        match t {
            0 => {}
            7 => self.r.skip(1)?,
            4 | 9 => self.r.skip(4)?,
            6 | 12 => self.r.skip(8)?,
            10 => {
                self.r.utf16()?;
            }
            17 | 18 => self.r.skip(24)?,
            11 => {
                ensure!(depth < MAX_ATTR_DEPTH, "attribute arrays nest too deeply");
                let n = self.r.u32()?;
                ensure!(n <= 100_000, "attr array too large");
                for _ in 0..n {
                    let t = self.r.u8()?;
                    self.skip_typed(t, depth + 1)?;
                }
            }
            _ => bail!("unknown legacy attribute type {t}"),
        }
        Ok(())
    }

    fn read_layer(&mut self, me: u64) -> Result<Value> {
        self.early_preamble()?;
        self.r.utf16()?;
        let mut hidden = None;
        let mut mid = 0;
        while mid < 8 && !self.r.marker() {
            let b = self.r.u8()?;
            hidden.get_or_insert(b);
            mid += 1;
        }
        self.r.utf16()?;
        let flags = self.r.u16()?;
        if flags & 0x00ff != 0 {
            // colour-by-layer with a textured material
            self.texture_block()?;
            self.r.skip(4)?;
        } else {
            self.r.skip(4)?;
            self.r.utf16()?;
            self.r.skip(if self.ver < FIRST_V4 { 17 } else { 21 })?;
            self.skip_clayer_colour_ext();
            self.skip_clayer_parent_ref(me);
        }
        Ok(Value::Layer { hidden: hidden.is_some_and(|h| h != 0) })
    }

    fn read_material(&mut self) -> Result<Value> {
        self.early_preamble()?;
        let name = self.r.utf16()?;
        let flag = self.r.u16()?;
        let rec = if flag == 0 {
            let c = self.r.raw(4)?;
            let rgba = [c[0], c[1], c[2], c[3]];
            self.r.utf16()?;
            self.r.skip(8)?;
            let mut opacity = self.r.f64()?;
            if self.r.u8()? == 0 {
                opacity = 0.0;
            }
            MatRec { name, rgba, opacity, tex: None }
        } else {
            let t = self.texture_block()?;
            MatRec { name, rgba: [t.rgb[0], t.rgb[1], t.rgb[2], 255], opacity: t.opacity, tex: Some(t) }
        };
        Ok(Value::Material(Box::new(rec)))
    }

    fn read_texture_coords(&mut self) -> Result<Value> {
        self.preamble()?;
        self.r.u32()?;
        let a = self.r.f64s::<24>()?;
        let mut uvf = [0.0; 9];
        uvf.copy_from_slice(&a[..9]);
        for _ in 0..2 {
            let z = self.r.u32()? as usize;
            self.r.skip(z.checked_mul(32).context("texture point count overflows")?)?;
        }
        self.r.skip(8)?; // front and back flags
        Ok(Value::Ftc(uvf))
    }

    fn read_image(&mut self) -> Result<Value> {
        self.preamble()?;
        let draw = self.draw()?;
        let def = self.object(None)?.slot;
        let placement = self.r.f64s::<12>()?;
        self.r.f64()?;
        self.r.utf16()?;
        self.r.skip(16)?;
        let mut xf = [0.0; 13];
        xf[..12].copy_from_slice(&placement);
        xf[12] = 1.0;
        Ok(Value::Instance(Box::new(InstRec { draw, def, xf, name: String::new() })))
    }

    fn read_construction_line(&mut self) -> Result<Value> {
        self.preamble()?;
        self.draw()?;
        self.r.skip(8 * 8)?;
        // The tail length varies by the build that wrote the file; calibrate on the first guide line by finding
        // the length that lands on a legitimate next tag, then keep it.
        let tail = if let Some(t) = self.cline_tail {
            t
        } else {
            let preferred = if self.ver == 17 { 7 } else { 4 };
            let order = std::iter::once(preferred).chain([0, 4, 7].into_iter().filter(|&c| c != preferred));
            let p = self.r.p;
            let strict = order.clone().find(|&c| self.strict_next_tag(p + c, false));
            let t = strict.or_else(|| [4, 0, 7].into_iter().find(|&c| self.strict_next_tag(p + c, true))).unwrap_or(4);
            self.cline_tail = Some(t);
            t
        };
        self.r.skip(tail)?;
        Ok(Value::Other)
    }

    fn read_section_plane(&mut self) -> Result<Value> {
        self.preamble()?;
        self.draw()?;
        let first = f64_at(self.d(), self.r.p).context("archive ends early")?;
        if first.abs() > 1.0001 {
            self.object(None)?;
        }
        self.r.skip(4 * 8)?;
        if self.r.marker() {
            self.r.utf16()?;
            self.r.utf16()?;
        }
        Ok(Value::Other)
    }

    fn read_dimension(&mut self) -> Result<Value> {
        self.preamble()?;
        self.draw()?;
        self.r.utf16()?;
        if self.ver >= 6 {
            self.object(Some("CSkFont"))?;
            self.r.skip(5)?;
        }
        for _ in 0..2 {
            self.r.skip(32)?;
            self.entity_ref(true)?;
            self.connection_paths()?;
        }
        self.r.skip(if self.ver >= 6 { 72 } else { 68 })?;
        Ok(Value::Other)
    }

    fn read_text(&mut self) -> Result<Value> {
        self.preamble()?;
        self.draw()?;
        self.object(Some("CSkFont"))?;
        if anchored_text_connection(self.d(), self.r.p) {
            self.r.skip(16 + 32)?;
            self.entity_ref(false)?;
            self.connection_paths()?;
        }
        let d = self.d();
        let start = self.r.p;
        let limit = d.len().min(start + 512);
        let found = (start..limit.saturating_sub(13))
            .find(|&q| {
                d.get(q..q + 14).is_some_and(|w| {
                    w[..4] == [1, 0, 0, 0]
                        && w[6] <= 8
                        && w[7..10] == [0, 0, 0]
                        && w[10] <= 1
                        && w[11..14] == [0xff, 0xfe, 0xff]
                })
            })
            .context("text delimiter not found")?;
        self.r.p = found + 11;
        self.r.utf16()?;
        self.r.skip(5)?;
        // escaped leader-attachment refs
        while bytes_at(d, self.r.p, &[0xff, 0x7f]) {
            match u32_at(d, self.r.p + 2) {
                Some(v) if v & 0x8000_0000 == 0 => self.r.p += 6,
                _ => break,
            }
        }
        Ok(Value::Other)
    }

    fn read_definition(&mut self) -> Result<Value> {
        self.preamble()?;
        let base_len = match self.ver {
            17.. => 22,
            5.. => 20,
            FIRST_V4.. => 19,
            _ => 13,
        };
        self.r.skip(base_len)?;
        let nl = self.r.u32()?;
        ensure!(nl <= 10_000, "implausible definition layer count");
        // The count is real layers; SketchUp 2020 puts null separators between them.
        let mut got = 0;
        while got < nl {
            if u16_at(self.d(), self.r.p) == Some(0) {
                self.r.p += 2;
                continue;
            }
            self.object(Some("CLayer"))?;
            got += 1;
        }
        if self.ver == 19 && nl > 0 && u16_at(self.d(), self.r.p) == Some(0) {
            self.r.p += 2;
        }
        if self.r.u16()? == 0x7fff {
            self.r.u32()?;
        }
        let count = self.definition_entity_count()?;
        let ents = self.entity_list(count, false)?;
        let mut nr = self.r.u32()?;
        if nr > 100_000
            && let Some(c) = self.retry_v20(self.r.p - 4, 100_000)
        {
            nr = c;
        }
        ensure!(nr <= 100_000, "definition list misaligned");
        for _ in 0..nr {
            self.object(Some("CRelationship"))?;
        }
        self.r.u16()?;
        let name = self.definition_tail()?;
        Ok(Value::Definition { name, ents })
    }

    fn definition_entity_count(&mut self) -> Result<u32> {
        let mut count = None;
        if self.ver >= 20 {
            count = self.retry_v20(self.r.p, 5_000_000);
        }
        let mut count = if let Some(c) = count {
            c
        } else {
            self.r.u32()?;
            self.r.u32()?
        };
        // A zero count is as much a sign of the filler as a huge one.
        if (count > 5_000_000 || count == 0)
            && let Some(c) = self.retry_v20(self.r.p - 4, 5_000_000)
        {
            count = c;
        }
        ensure!(count <= 5_000_000, "implausible definition entity count");
        Ok(count)
    }

    /// GUID, name, description, path and load time, then a variable gap up to the thumbnail. Returns the name.
    fn definition_tail(&mut self) -> Result<String> {
        // Some 2020 files carry two extra bytes ahead of the GUID; anchor on the name's string marker after it.
        if !marker_at(self.d(), self.r.p + 16)
            && let Some(skip) = (1..=4).find(|s| marker_at(self.d(), self.r.p + s + 16))
        {
            self.r.p += skip;
        }
        self.r.skip(16)?;
        let name = self.r.utf16()?;
        self.r.utf16()?;
        self.r.utf16()?;
        self.r.u32()?;
        let d = self.d();
        let thumb_cs = self.class_slot.get("CThumbnail").copied();
        let tpos = (0..96)
            .map(|off| self.r.p + off)
            .take_while(|p| p + 26 <= d.len())
            .find(|&p| {
                (bytes_at(d, p, &[0xff, 0xff]) && bytes_at(d, p + 4, &[10, 0]) && bytes_at(d, p + 6, b"CThumbnail"))
                    || thumb_cs.is_some_and(|cs| is_class_ref(d, p, cs))
            })
            .context("definition thumbnail not found")?;
        self.r.p = tpos;
        self.object(Some("CThumbnail"))?;
        Ok(name)
    }

    fn read_instance(&mut self) -> Result<Value> {
        self.preamble()?;
        let draw = self.draw()?;
        let def = self.object(Some("CComponentDefinition"))?.slot;
        let xf = self.r.f64s::<13>()?;
        let name = if self.ver >= FIRST_NAMED_INSTANCE || self.r.marker() { self.r.utf16()? } else { String::new() };
        if self.ver >= 14 {
            self.r.skip(16)?;
        }
        Ok(Value::Instance(Box::new(InstRec { draw, def, xf, name })))
    }

    fn entity_list(&mut self, n: u32, root: bool) -> Result<Vec<Obj>> {
        self.burn_stack.push(0);
        self.inline_stack.push(Vec::new());
        let depth = self.inline_stack.len() - 1;
        let out = self.entity_list_inner(n as usize, root, depth);
        self.burn_stack.pop();
        self.inline_stack.pop();
        out
    }

    fn entity_list_inner(&mut self, n: usize, root: bool, depth: usize) -> Result<Vec<Obj>> {
        let mut v: Vec<Obj> = Vec::new();
        while v.len() < n {
            let save = self.r.p;
            let pending = self.inline_stack.get(depth).context("entity list stack")?;
            if !pending.is_empty() && v.len() + pending.len() >= n {
                // everything left is an instance an annotation already wrote in full
                if !self.peek_backref().is_some_and(|s| pending.contains(&s)) {
                    break;
                }
            }
            let has_burn_credit = !root && self.burn_stack.last().is_some_and(|&c| c > 0);
            if has_burn_credit && u32_at(self.d(), save) == Some(0) && marker_at(self.d(), save + 22) {
                // the definition tail: the list's real end when the declared count includes phantoms
                break;
            }
            match self.object(None) {
                Ok(item) => {
                    if let Some(listed) = self.inline_stack.get_mut(depth) {
                        listed.retain(|&s| s != item.slot);
                    }
                    v.push(item);
                }
                Err(_) if root || has_burn_credit => {
                    // an over-declared count runs into the document tail
                    self.r.p = save;
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        // instances an annotation wrote in full that the list never listed again are still its entities
        let leftover = self.inline_stack.get(depth).cloned().unwrap_or_default();
        for slot in leftover {
            if v.len() >= n {
                break;
            }
            let e = self.slots.get(&slot).context("inline instance slot")?;
            v.push(Obj { slot, name: Rc::clone(&e.name), v: e.v.clone() });
        }
        Ok(v)
    }
}

struct Walk<'a> {
    ar: Archive<'a>,
    root: Vec<Obj>,
    materials: Vec<Obj>,
}

/// With two or more materials, the second material's class reference names `CMaterial`'s true slot, which fixes
/// the slot numbering for the whole file.
fn bootstrap_two_materials(data: &[u8], ver: u32, start: usize) -> Result<u64> {
    let mut boot = Archive::new(data, ver);
    boot.next = PROBE_BASE;
    boot.base = PROBE_BASE;
    boot.r.p = start;
    boot.object(Some("CMaterial"))?;
    let tag = boot.r.peek_u16()?;
    ensure!(tag != 0xffff && tag & 0x8000 != 0, "cannot bootstrap the slot base");
    Ok(u64::from(tag & 0x7fff))
}

/// Slot-base candidates for files with fewer than two materials: the reference after the layer list points back
/// at a layer, absolutely, so each layer read under a throwaway base gives one candidate.
fn probe_layer_anchor_bases(data: &[u8], ver: u32, start: usize, mat_count: u32) -> Result<Vec<u64>> {
    let mut boot = Archive::new(data, ver);
    boot.next = PROBE_BASE;
    boot.base = PROBE_BASE;
    boot.r.p = start;
    for _ in 0..mat_count {
        boot.object(Some("CMaterial"))?;
    }
    boot.r.u32()?;
    if ver >= 17 {
        boot.r.u8()?;
    }
    let mut layers = Vec::new();
    if ver >= FIRST_V4 {
        let count = boot.r.u32()?;
        ensure!((1..=100_000).contains(&count), "implausible layer count in base probe");
        for _ in 0..count {
            layers.push(boot.object(Some("CLayer"))?);
        }
    }
    boot.collect_trailing_layers(&mut layers, true)?;
    let anchor = boot.object(None)?;
    ensure!(&*anchor.name == "premodel", "base probe: anchor resolved to {}", anchor.name);
    Ok(layers
        .iter()
        .filter_map(|l| anchor.slot.checked_add(PROBE_BASE)?.checked_sub(l.slot))
        .filter(|&c| c > 0 && c < PROBE_BASE)
        .collect())
}

/// True when a `CComponentDefinition` record starts at `at`.
fn definition_starts_at(ar: &Archive<'_>, at: usize) -> bool {
    let d = ar.d();
    if bytes_at(d, at, &[0xff, 0xff]) && bytes_at(d, at + 6, b"CComponentDefinition") {
        return true;
    }
    ar.class_slot.get("CComponentDefinition").is_some_and(|&cs| is_class_ref(d, at, cs))
}

fn walk_model(
    data: &[u8],
    ver: u32,
    start: usize,
    mat_count: u32,
    base: u64,
) -> Result<Walk<'_>, (anyhow::Error, usize)> {
    let mut ar = Archive::new(data, ver);
    ar.next = base;
    ar.base = base;
    ar.r.p = start;
    match walk_model_on(&mut ar, mat_count) {
        Ok((root, materials)) => Ok(Walk { ar, root, materials }),
        Err(e) => Err((e, ar.r.p)),
    }
}

fn walk_model_on(ar: &mut Archive<'_>, mat_count: u32) -> Result<(Vec<Obj>, Vec<Obj>)> {
    let ver = ar.ver;
    let mut materials = Vec::new();
    for _ in 0..mat_count {
        materials.push(ar.object(Some("CMaterial"))?);
    }
    ar.r.u32()?;
    if ver >= 17 {
        ar.r.u8()?;
    }
    // SketchUp 3 writes no layer count; its layers run until the definition-list anchor.
    let lc = if ver < FIRST_V4 { usize::MAX } else { ar.r.u32()? as usize };
    ensure!(lc <= 100_000 || ver < FIRST_V4, "invalid layer count");
    let mut layers = Vec::new();
    while layers.len() < lc {
        let Some(tag) = u16_at(ar.d(), ar.r.p) else { break };
        if tag == 0 {
            ar.r.p += 2;
            continue;
        }
        if tag != 0xffff && tag & 0x8000 == 0 {
            break;
        }
        let q = ar.object(Some("CLayer"))?;
        if q.v.is_some() {
            layers.push(q);
        }
    }
    ar.collect_trailing_layers(&mut layers, false)?;
    let anchor = ar.object(None)?;
    ensure!(&*anchor.name == "CLayer", "definition anchor is not a layer");
    read_definitions(ar)?;
    let mut root_count = ar.r.u32()?;
    if root_count > 5_000_000
        && let Some(c) = ar.retry_v20(ar.r.p - 4, 5_000_000)
    {
        root_count = c;
    }
    ensure!(root_count <= 5_000_000, "implausible root entity count");
    let root = ar.entity_list(root_count, true)?;
    Ok((root, materials))
}

fn read_definitions(ar: &mut Archive<'_>) -> Result<()> {
    let mut dc = ar.r.u32()?;
    if dc > 1_000_000 {
        if let Some(c) = ar.retry_v20(ar.r.p - 4, 1_000_000) {
            dc = c;
            if !definition_starts_at(ar, ar.r.p) {
                // no definitions: the count behind the filler was the root count, so put it back
                dc = 0;
                ar.r.p -= 4;
            }
        } else if let Some(at) = find_empty_root_after_v20_filler(ar.d(), ar.r.p - 4) {
            dc = 0;
            ar.r.p = at;
        }
    }
    ensure!(dc <= 1_000_000, "invalid definition count");
    for _ in 0..dc {
        ar.object(Some("CComponentDefinition"))?;
    }
    while definition_starts_at(ar, ar.r.p) {
        ar.object(None)?;
    }
    Ok(())
}

/// Tries each slot base; every wrong one fails almost at once, so when none works the error that got furthest
/// into the file is the useful one.
fn walk_with_bases<'a>(data: &'a [u8], ver: u32, start: usize, mat_count: u32, bases: &[u64]) -> Result<Walk<'a>> {
    let mut best: Option<(anyhow::Error, usize)> = None;
    for &base in bases {
        match walk_model(data, ver, start, mat_count, base) {
            Ok(w) => return Ok(w),
            Err((e, at)) => {
                if best.as_ref().is_none_or(|b| at > b.1) {
                    best = Some((e, at));
                }
            }
        }
    }
    Err(best.map_or_else(|| anyhow!("no viable slot base"), |(e, at)| e.context(format!("stopped at offset {at}"))))
}

/// Finds where the model starts: the material count right before the first `CMaterial` class record, or for a
/// file with no materials the layer list before the first `CLayer` class record.
fn find_start(data: &[u8], ver: u32) -> Result<(usize, u32)> {
    let class_record = |name: &[u8]| {
        (0..data.len()).find(|&i| {
            bytes_at(data, i, &[0xff, 0xff])
                && u16_at(data, i + 4) == Some(name.len() as u16)
                && bytes_at(data, i + 6, name)
        })
    };
    if let Some(mh) = class_record(b"CMaterial").filter(|&mh| mh >= 4) {
        let mc = u32_at(data, mh - 4).context("archive ends early")?;
        ensure!(mc <= 100_000, "implausible material count");
        return Ok((mh, mc));
    }
    let lh = class_record(b"CLayer").context("no CMaterial or CLayer class record found")?;
    let start = lh.checked_sub(if ver >= 17 { 9 } else { 8 }).context("layer record too early")?;
    Ok((start, 0))
}

pub(crate) fn parse(data: &[u8], ver: u32) -> Result<Model> {
    let (start, mc) = find_start(data, ver)?;
    let bases = if mc >= 2 {
        vec![bootstrap_two_materials(data, ver, start)?]
    } else {
        probe_layer_anchor_bases(data, ver, start, mc)?
    };
    let walk = walk_with_bases(data, ver, start, mc, &bases)?;
    Ok(to_model(data, &walk))
}

fn to_model(data: &[u8], walk: &Walk<'_>) -> Model {
    let slots = &walk.ar.slots;
    let mut model = Model::default();
    for m in &walk.materials {
        if let Some(Value::Material(rec)) = m.v.as_deref() {
            model.materials.insert(m.slot, material(data, slots, rec));
        }
    }
    for (&slot, e) in slots {
        match e.v.as_deref() {
            Some(Value::Layer { hidden: true }) => model.hidden_layers.push(slot),
            Some(Value::Definition { name, ents }) => {
                let mut def = fill(ents, slots);
                def.name.clone_from(name);
                model.definitions.insert(slot, def);
            }
            _ => {}
        }
    }
    model.root = fill(&walk.root, slots);
    model
}

fn material(data: &[u8], slots: &HashMap<u64, Entry>, rec: &MatRec) -> model::Material {
    let texture = rec.tex.as_ref().and_then(|t| {
        let (start, len) = match t.inline {
            Some(range) => range,
            None => match slots.get(&t.dib?)?.v.as_deref()? {
                Value::Dib { start, len } => (*start, *len),
                _ => return None,
            },
        };
        let bytes = data.get(start..start.checked_add(len)?)?;
        crate::glb::image_mime(bytes)?;
        Some(model::Texture { bytes: bytes.to_vec(), tile: t.tile })
    });
    model::Material {
        name: rec.name.clone(),
        rgb: [rec.rgba[0], rec.rgba[1], rec.rgba[2]],
        alpha: legacy_alpha(rec),
        texture,
    }
}

/// The colour's own alpha times the material's opacity. The opacity field only counts when the flag after it
/// says transparency is in use; otherwise the reader has already zeroed it.
fn legacy_alpha(rec: &MatRec) -> f64 {
    let transparency = (1.0 - rec.opacity).clamp(0.0, 1.0);
    f64::from(rec.rgba[3]) / 255.0 * transparency
}

/// Turns one entity list into faces and instances.
fn fill(ents: &[Obj], slots: &HashMap<u64, Entry>) -> model::Definition {
    let mut def = model::Definition::default();
    let nonzero = |id: u64| (id != 0).then_some(id);
    for obj in ents {
        match obj.v.as_deref() {
            Some(Value::Face(f)) => {
                let loops: Vec<Vec<[f64; 3]>> = f.loops.iter().filter_map(|l| loop_points(l, slots)).collect();
                if loops.is_empty() {
                    continue;
                }
                def.faces.push(model::Face {
                    loops,
                    normal: f.normal,
                    material: nonzero(f.draw.mat),
                    uv_matrix: f.attrs.and_then(|a| texture_matrix(a, slots)),
                    hidden: f.draw.hidden,
                    layer: nonzero(f.draw.layer),
                });
            }
            Some(Value::Instance(i)) => def.instances.push(model::Instance {
                definition: i.def,
                name: i.name.clone(),
                matrix: i.xf,
                material: nonzero(i.draw.mat),
                hidden: i.draw.hidden,
                layer: nonzero(i.draw.layer),
            }),
            _ => {}
        }
    }
    def
}

/// The positioned-texture matrix kept among a face's attributes, if any.
fn texture_matrix(attrs: u64, slots: &HashMap<u64, Entry>) -> Option<[f64; 9]> {
    let Some(Value::Attrs(ents)) = slots.get(&attrs)?.v.as_deref() else { return None };
    ents.iter().find_map(|e| match e.v.as_deref() {
        Some(Value::Ftc(m)) => Some(*m),
        _ => None,
    })
}

/// A loop's corner points: each edge use starts at its edge's first vertex, or its second when reversed.
fn loop_points(l: &Value, slots: &HashMap<u64, Entry>) -> Option<Vec<[f64; 3]>> {
    let Value::Loop(uses) = l else { return None };
    let mut ids: Vec<u64> = Vec::new();
    for u in uses {
        let Value::EdgeUse { edge, sense } = **u else { continue };
        let Some(Value::Edge { v1, v2 }) = slots.get(&edge).and_then(|e| e.v.as_deref()) else { continue };
        let id = if sense { *v2 } else { *v1 };
        if ids.last() != Some(&id) {
            ids.push(id);
        }
    }
    if ids.len() > 1 && ids.first() == ids.last() {
        ids.pop();
    }
    let points: Option<Vec<[f64; 3]>> = ids
        .iter()
        .map(|id| match slots.get(id)?.v.as_deref()? {
            Value::Vertex(p) => Some(*p),
            _ => None,
        })
        .collect();
    points.filter(|p| p.len() >= 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class_decl(name: &str) -> Vec<u8> {
        let mut out = vec![0xff, 0xff, 1, 0];
        out.extend((name.len() as u16).to_le_bytes());
        out.extend(name.as_bytes());
        out
    }

    #[test]
    fn reads_class_declarations_and_back_references() {
        // a SketchUp 3 vertex has no preamble: the declaration, three doubles, then a back-reference to it
        let mut d = class_decl("CVertex");
        for v in [1.0f64, 2.0, 3.0] {
            d.extend(v.to_le_bytes());
        }
        d.extend(11u16.to_le_bytes());
        let mut ar = Archive::new(&d, 3);
        ar.next = 10;
        ar.base = 10;
        let v = ar.object(None).unwrap();
        assert_eq!(v.slot, 11);
        assert!(
            matches!(v.v.as_deref(), Some(Value::Vertex(p)) if p.map(f64::to_bits) == [1.0, 2.0, 3.0].map(f64::to_bits))
        );
        let again = ar.object(None).unwrap();
        assert_eq!(again.slot, 11);
        assert_eq!(&*again.name, "CVertex");
    }

    #[test]
    fn rejects_unknown_classes_and_slots() {
        let d = class_decl("CMystery");
        assert!(Archive::new(&d, 15).object(None).is_err());
        let d = 0x8005u16.to_le_bytes();
        assert!(Archive::new(&d, 15).object(None).is_err());
        let d = 5u16.to_le_bytes();
        assert!(Archive::new(&d, 15).object(None).is_err());
        let d = [0xff, 0xff, 1, 0, 200, 0];
        assert!(Archive::new(&d, 15).object(None).is_err());
    }

    #[test]
    fn deep_nesting_is_an_error_not_a_stack_overflow() {
        // a loop whose first edge use opens another loop, and so on: each level nests a new object
        let mut d = class_decl("CLoop");
        for _ in 0..400 {
            d.extend([0, 0]);
            d.extend(0x8000u16.to_le_bytes());
        }
        let mut ar = Archive::new(&d, 3);
        assert!(ar.object(None).is_err());
    }

    #[test]
    fn finds_counts_behind_the_v20_filler() {
        let mut d = vec![0; 4];
        d.extend([0xff, 0xfe, 0xff, 0]);
        d.extend([0; 9]);
        d.extend(7u32.to_le_bytes());
        d.extend([0, 0]);
        let mut ar = Archive::new(&d, 20);
        assert_eq!(ar.retry_v20(0, 100), Some(7));
        assert_eq!(ar.r.p, 21);
        // a non-empty string is real data, not filler
        d[7] = 1;
        assert_eq!(Archive::new(&d, 20).retry_v20(0, 100), None);
    }

    #[test]
    fn finds_an_empty_root_behind_the_filler() {
        let mut d = vec![0xff, 0xfe, 0xff, 0];
        d.extend([0; 17]);
        assert_eq!(find_empty_root_after_v20_filler(&d, 0), Some(17));
        d[10] = 1;
        assert_eq!(find_empty_root_after_v20_filler(&d, 0), None);
    }

    #[test]
    fn burn_bands_translate_references() {
        let mut ar = Archive::new(&[], 17);
        ar.burns = vec![(10, 2), (20, 3)];
        assert_eq!(ar.translate_ref(5), Some(5));
        assert_eq!(ar.translate_ref(11), None);
        assert_eq!(ar.translate_ref(12), Some(10));
        assert_eq!(ar.translate_ref(30), Some(25));
    }
}
