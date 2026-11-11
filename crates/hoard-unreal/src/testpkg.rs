//! Test-only package writer: lays out editor packages field by field, the way the parser reads them, so converters
//! can be tested without redistributing real Unreal content. Supports UE 4.27 (`ue5: 0`), 5.3 (`1009`) and the
//! 5.4+ property tag layout (`1012`).

#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::too_many_lines,
    reason = "small test fixtures, laid out in one linear walk"
)]

use crate::package::PACKAGE_TAG;

/// A name table that grows as names are used.
#[derive(Debug, Default, Clone)]
pub(crate) struct Names(pub Vec<String>);

impl Names {
    pub(crate) fn idx(&mut self, s: &str) -> i32 {
        if let Some(i) = self.0.iter().position(|x| x == s) {
            return i as i32;
        }
        self.0.push(s.to_owned());
        (self.0.len() - 1) as i32
    }
}

fn fstr(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&((s.len() + 1) as i32).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
    out.push(0);
}

/// Writes tagged properties in the tag layout of the given versions.
pub(crate) struct Props<'a> {
    names: &'a mut Names,
    ue5: i32,
    pub(crate) out: Vec<u8>,
}

impl<'a> Props<'a> {
    pub(crate) fn new(names: &'a mut Names, ue5: i32) -> Self {
        Self { names, ue5, out: Vec::new() }
    }

    pub(crate) fn fname(&mut self, s: &str) {
        let i = self.names.idx(s);
        self.out.extend_from_slice(&i.to_le_bytes());
        self.out.extend_from_slice(&0i32.to_le_bytes());
    }

    /// One tag. `ty` is the type tree in pre-order: `[("ArrayProperty", 1), ("StructProperty", 1), ("X", 0)]`.
    fn tag(&mut self, name: &str, ty: &[(&str, i32)], value: &[u8], bool_value: bool) {
        self.fname(name);
        if self.ue5 >= 1012 {
            for (n, c) in ty {
                self.fname(n);
                self.out.extend_from_slice(&c.to_le_bytes());
            }
            self.out.extend_from_slice(&(value.len() as i32).to_le_bytes());
            self.out.push(if bool_value { 0x10 } else { 0 });
        } else {
            let at = |i: usize| ty.get(i).map_or("None", |t| t.0);
            self.fname(ty[0].0);
            self.out.extend_from_slice(&(value.len() as i32).to_le_bytes());
            self.out.extend_from_slice(&0i32.to_le_bytes()); // array index
            match ty[0].0 {
                "StructProperty" => {
                    self.fname(at(1));
                    self.out.extend_from_slice(&[0; 16]);
                }
                "BoolProperty" => self.out.push(u8::from(bool_value)),
                "ByteProperty" | "EnumProperty" | "ArrayProperty" | "SetProperty" => self.fname(at(1)),
                "MapProperty" => {
                    self.fname(at(1));
                    self.fname(at(2));
                }
                _ => {}
            }
            self.out.push(0); // no property guid
            if self.ue5 >= 1011 {
                self.out.push(0); // no extensions
            }
        }
        self.out.extend_from_slice(value);
    }

    pub(crate) fn int(&mut self, name: &str, v: i32) -> &mut Self {
        self.tag(name, &[("IntProperty", 0)], &v.to_le_bytes(), false);
        self
    }

    pub(crate) fn float(&mut self, name: &str, v: f32) -> &mut Self {
        self.tag(name, &[("FloatProperty", 0)], &v.to_le_bytes(), false);
        self
    }

    pub(crate) fn boolean(&mut self, name: &str, v: bool) -> &mut Self {
        self.tag(name, &[("BoolProperty", 0)], &[], v);
        self
    }

    /// An enum value stored as a byte property with an enum name.
    pub(crate) fn enumeration(&mut self, name: &str, enum_name: &str, value: &str) -> &mut Self {
        let i = self.names.idx(value);
        let mut v = i.to_le_bytes().to_vec();
        v.extend_from_slice(&0i32.to_le_bytes());
        self.tag(name, &[("ByteProperty", 1), (enum_name, 0)], &v, false);
        self
    }

    pub(crate) fn name(&mut self, name: &str, value: &str) -> &mut Self {
        let i = self.names.idx(value);
        let mut v = i.to_le_bytes().to_vec();
        v.extend_from_slice(&0i32.to_le_bytes());
        self.tag(name, &[("NameProperty", 0)], &v, false);
        self
    }

    /// An object reference by package index (negative: import).
    pub(crate) fn object(&mut self, name: &str, index: i32) -> &mut Self {
        self.tag(name, &[("ObjectProperty", 0)], &index.to_le_bytes(), false);
        self
    }

    /// A struct of tagged properties.
    pub(crate) fn structure(&mut self, name: &str, struct_name: &str, f: impl FnOnce(&mut Props<'_>)) -> &mut Self {
        let mut sub = Props::new(self.names, self.ue5);
        f(&mut sub);
        sub.end();
        let v = sub.out;
        self.tag(name, &[("StructProperty", 1), (struct_name, 0)], &v, false);
        self
    }

    /// An array of tagged structs (with the pre-5.4 inner tag).
    pub(crate) fn struct_array(
        &mut self,
        name: &str,
        struct_name: &str,
        items: &[&dyn Fn(&mut Props<'_>)],
    ) -> &mut Self {
        let mut body = Vec::new();
        for f in items {
            let mut sub = Props::new(self.names, self.ue5);
            f(&mut sub);
            sub.end();
            body.extend(sub.out);
        }
        let mut v = (items.len() as i32).to_le_bytes().to_vec();
        if self.ue5 < 1012 {
            let mut inner = Props::new(self.names, self.ue5);
            inner.fname(name);
            inner.fname("StructProperty");
            inner.out.extend_from_slice(&(body.len() as i32).to_le_bytes());
            inner.out.extend_from_slice(&0i32.to_le_bytes());
            inner.fname(struct_name);
            inner.out.extend_from_slice(&[0; 16]);
            inner.out.push(0);
            if self.ue5 >= 1011 {
                inner.out.push(0);
            }
            v.extend(inner.out);
        }
        v.extend(body);
        self.tag(name, &[("ArrayProperty", 1), ("StructProperty", 1), (struct_name, 0)], &v, false);
        self
    }

    pub(crate) fn end(&mut self) {
        self.fname("None");
    }
}

/// The package to write.
#[derive(Debug, Clone)]
pub(crate) struct Synth {
    pub class: String,
    pub asset: String,
    /// Package imports, besides `/Script/Engine` and the class.
    pub deps: Vec<String>,
    /// Extra imports: `(class, object, outer)`, the outer being one of `deps` or empty for `/Script/Engine`
    /// (e.g. `("Texture2D", "T_Rock", "/Game/Rocks/T_Rock")`).
    pub objects: Vec<(String, String, String)>,
    pub ue5: i32,
    pub custom_versions: Vec<([u8; 16], i32)>,
    pub names: Names,
    /// The main export's serialized data.
    pub export: Vec<u8>,
    /// A package trailer, appended after the export.
    pub trailer: Option<Vec<u8>>,
}

impl Synth {
    pub(crate) fn new(class: &str, asset: &str, ue5: i32) -> Self {
        let mut names = Names::default();
        for n in ["/Script/Engine", "Package", class, asset, "Class", "None"] {
            names.idx(n);
        }
        Self {
            class: class.into(),
            asset: asset.into(),
            deps: Vec::new(),
            objects: Vec::new(),
            ue5,
            custom_versions: Vec::new(),
            names,
            export: Vec::new(),
            trailer: None,
        }
    }

    /// Import index (negative) of an extra object added with [`Synth::objects`].
    pub(crate) fn object_index(&self, object: &str) -> i32 {
        let i = self.objects.iter().position(|(_, o, _)| o == object).expect("object import");
        -(3 + self.deps.len() as i32 + i as i32)
    }

    /// Start of export data, for building the export: properties then native data.
    pub(crate) fn props(&mut self) -> Props<'_> {
        let mut p = Props::new(&mut self.names, self.ue5);
        if self.ue5 >= 1011 {
            p.out.push(0); // serialization control extension
        }
        p
    }

    pub(crate) fn build(&self) -> Vec<u8> {
        let mut names = self.names.clone();
        for d in &self.deps {
            names.idx(d);
        }
        for (c, o, _) in &self.objects {
            names.idx(c);
            names.idx(o);
        }
        let ue5 = self.ue5;
        let i32b = |v: i32| v.to_le_bytes();
        let (class, asset) = (self.class.as_str(), self.asset.as_str());

        let mut s = Vec::new();
        s.extend_from_slice(&PACKAGE_TAG.to_le_bytes());
        s.extend_from_slice(&i32b(if ue5 > 0 { -8 } else { -7 })); // legacy
        s.extend_from_slice(&i32b(864)); // legacy UE3
        s.extend_from_slice(&i32b(522)); // UE4
        if ue5 > 0 {
            s.extend_from_slice(&i32b(ue5));
        }
        s.extend_from_slice(&i32b(0)); // licensee
        s.extend_from_slice(&i32b(self.custom_versions.len() as i32));
        for (g, v) in &self.custom_versions {
            s.extend_from_slice(g);
            s.extend_from_slice(&i32b(*v));
        }
        let total_at = s.len();
        s.extend_from_slice(&i32b(0)); // total header size
        fstr(&mut s, "None");
        s.extend_from_slice(&0u32.to_le_bytes()); // flags (editor package)
        s.extend_from_slice(&i32b(names.0.len() as i32));
        let name_off_at = s.len();
        s.extend_from_slice(&i32b(0));
        if ue5 >= 1008 {
            s.extend_from_slice(&i32b(0)); // soft object paths count
            s.extend_from_slice(&i32b(0)); // offset
        }
        fstr(&mut s, ""); // localization id
        s.extend_from_slice(&i32b(0)); // gatherable text
        s.extend_from_slice(&i32b(0));
        s.extend_from_slice(&i32b(1)); // exports
        let export_off_at = s.len();
        s.extend_from_slice(&i32b(0));
        s.extend_from_slice(&i32b((2 + self.deps.len() + self.objects.len()) as i32)); // imports
        let import_off_at = s.len();
        s.extend_from_slice(&i32b(0));
        s.extend_from_slice(&i32b(0)); // depends
        s.extend_from_slice(&i32b(0)); // soft package refs
        s.extend_from_slice(&i32b(0));
        s.extend_from_slice(&i32b(0)); // searchable names
        let thumb_off_at = s.len();
        s.extend_from_slice(&i32b(0));
        s.extend_from_slice(&[0u8; 16]); // guid
        s.extend_from_slice(&[0u8; 16]); // persistent guid
        s.extend_from_slice(&i32b(1)); // generations
        s.extend_from_slice(&i32b(1));
        s.extend_from_slice(&i32b(names.0.len() as i32));
        let (minor, label) = match ue5 {
            0 => (27, "++UE4+Release-4.27"),
            1012.. => (4, "++UE5+Release-5.4"),
            _ => (3, "++UE5+Release-5.3"),
        };
        for _ in 0..2 {
            s.extend_from_slice(&(if ue5 > 0 { 5u16 } else { 4 }).to_le_bytes());
            s.extend_from_slice(&(minor as u16).to_le_bytes());
            s.extend_from_slice(&2u16.to_le_bytes());
            s.extend_from_slice(&0u32.to_le_bytes());
            fstr(&mut s, label);
        }
        s.extend_from_slice(&0u32.to_le_bytes()); // compression
        s.extend_from_slice(&i32b(0)); // chunks
        s.extend_from_slice(&0u32.to_le_bytes()); // source
        s.extend_from_slice(&i32b(0)); // additional packages
        let registry_off_at = s.len();
        s.extend_from_slice(&i32b(0));
        s.extend_from_slice(&0i64.to_le_bytes()); // bulk data start
        s.extend_from_slice(&i32b(0)); // world tile info
        s.extend_from_slice(&i32b(0)); // chunk ids
        s.extend_from_slice(&i32b(0)); // preload dependency count
        s.extend_from_slice(&i32b(0)); // offset
        let mut toc_at = None;
        if ue5 > 0 {
            s.extend_from_slice(&i32b(0)); // names referenced from export data
            toc_at = Some(s.len());
            s.extend_from_slice(&(-1i64).to_le_bytes()); // payload toc offset
            s.extend_from_slice(&i32b(-1)); // data resource offset
        }

        let patch = |s: &mut Vec<u8>, at: usize, v: usize| s[at..at + 4].copy_from_slice(&(v as i32).to_le_bytes());
        let fname = |s: &mut Vec<u8>, i: i32| {
            s.extend_from_slice(&i.to_le_bytes());
            s.extend_from_slice(&0i32.to_le_bytes());
        };

        let at = s.len();
        patch(&mut s, name_off_at, at);
        for n in &names.0 {
            fstr(&mut s, n);
            s.extend_from_slice(&[0u8; 4]);
        }
        let at = s.len();
        patch(&mut s, import_off_at, at);
        let mut imports: Vec<(&str, i32, &str)> = vec![("Package", 0, "/Script/Engine"), ("Class", -1, class)];
        imports.extend(self.deps.iter().map(|d| ("Package", 0, d.as_str())));
        imports.extend(self.objects.iter().map(|(c, o, outer)| {
            let outer = self.deps.iter().position(|d| d == outer).map_or(-1, |i| -(3 + i as i32));
            (c.as_str(), outer, o.as_str())
        }));
        for (cls, outer, obj) in imports {
            fname(&mut s, names.idx("/Script/Engine"));
            fname(&mut s, names.idx(cls));
            s.extend_from_slice(&i32b(outer));
            fname(&mut s, names.idx(obj));
            fname(&mut s, 0); // package name (UE4 >= NON_OUTER_PACKAGE_IMPORT)
            if ue5 >= 1003 {
                s.extend_from_slice(&i32b(0)); // import optional
            }
        }
        let at = s.len();
        patch(&mut s, export_off_at, at);
        s.extend_from_slice(&i32b(-2)); // class: import 2
        s.extend_from_slice(&i32b(0)); // super
        s.extend_from_slice(&i32b(0)); // template
        s.extend_from_slice(&i32b(0)); // outer
        fname(&mut s, names.idx(asset));
        s.extend_from_slice(&0u32.to_le_bytes()); // flags
        let serial_at = s.len();
        s.extend_from_slice(&(self.export.len() as i64).to_le_bytes()); // serial size
        s.extend_from_slice(&0i64.to_le_bytes()); // serial offset, patched
        s.extend_from_slice(&[0u8; 12]);
        if ue5 < 1005 {
            s.extend_from_slice(&[0u8; 16]); // package guid
        }
        if ue5 >= 1006 {
            s.extend_from_slice(&i32b(0)); // is inherited instance
        }
        s.extend_from_slice(&0u32.to_le_bytes()); // package flags
        s.extend_from_slice(&i32b(0)); // not always loaded
        s.extend_from_slice(&i32b(1)); // is asset
        if ue5 >= 1003 {
            s.extend_from_slice(&i32b(0)); // generate public hash
        }
        s.extend_from_slice(&[0u8; 20]); // preload deps
        if ue5 >= 1010 {
            s.extend_from_slice(&[0u8; 16]); // script serialization offsets
        }

        let at = s.len();
        patch(&mut s, thumb_off_at, at);
        s.extend_from_slice(&i32b(1));
        fstr(&mut s, class);
        fstr(&mut s, asset);
        let data_at_pos = s.len();
        s.extend_from_slice(&i32b(0));
        let at = s.len();
        patch(&mut s, data_at_pos, at);
        let thumb = b"\x89PNG\r\n\x1a\nthumb";
        s.extend_from_slice(&i32b(4));
        s.extend_from_slice(&i32b(4));
        s.extend_from_slice(&i32b(thumb.len() as i32));
        s.extend_from_slice(thumb);

        let at = s.len();
        patch(&mut s, registry_off_at, at);
        s.extend_from_slice(&0i64.to_le_bytes()); // dependency data offset
        s.extend_from_slice(&i32b(1));
        fstr(&mut s, asset);
        fstr(&mut s, &format!("/Script/Engine.{class}"));
        s.extend_from_slice(&i32b(2));
        fstr(&mut s, "Dimensions");
        fstr(&mut s, "512x256");
        fstr(&mut s, "Triangles");
        fstr(&mut s, "1234");

        let header = s.len();
        patch(&mut s, total_at, header);
        s[serial_at + 8..serial_at + 16].copy_from_slice(&(header as i64).to_le_bytes());
        s.extend_from_slice(&self.export);
        if let (Some(trailer), Some(at)) = (&self.trailer, toc_at) {
            let off = s.len() as i64;
            s[at..at + 8].copy_from_slice(&off.to_le_bytes());
            s.extend_from_slice(trailer);
        }
        s
    }
}

/// An uncompressed (method 0) `FCompressedBuffer`.
pub(crate) fn compressed_buffer_raw(payload: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&0xb775_6362u32.to_be_bytes());
    b.extend_from_slice(&0u32.to_be_bytes()); // crc
    b.extend_from_slice(&[0, 0, 0, 0]); // method none, compressor, level, block exponent
    b.extend_from_slice(&1u32.to_be_bytes());
    b.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    b.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    b.extend_from_slice(&[0; 32]);
    b.extend_from_slice(payload);
    b
}

/// An LZ4 (method 4) `FCompressedBuffer` with the given block size exponent.
pub(crate) fn compressed_buffer_lz4(payload: &[u8], block_exp: u8) -> Vec<u8> {
    let block = 1usize << block_exp;
    let blocks: Vec<Vec<u8>> = payload
        .chunks(block)
        .map(|c| {
            let z = lz4_flex::block::compress(c);
            if z.len() >= c.len() { c.to_vec() } else { z }
        })
        .collect();
    let total: usize = blocks.iter().map(Vec::len).sum();
    let mut b = Vec::new();
    b.extend_from_slice(&0xb775_6362u32.to_be_bytes());
    b.extend_from_slice(&0u32.to_be_bytes());
    b.extend_from_slice(&[4, 0, 0, block_exp]);
    b.extend_from_slice(&(blocks.len() as u32).to_be_bytes());
    b.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    b.extend_from_slice(&(total as u64).to_be_bytes());
    b.extend_from_slice(&[0; 32]);
    for blk in &blocks {
        b.extend_from_slice(&(blk.len() as u32).to_be_bytes());
    }
    for blk in blocks {
        b.extend(blk);
    }
    b
}

/// A version-2 package trailer holding local payloads, keyed by id.
pub(crate) fn trailer(payloads: &[([u8; 20], Vec<u8>, u64)]) -> Vec<u8> {
    let entry_len = 20 + 8 + 8 + 8 + 2 + 2 + 1;
    let header_len = 8 + 4 + 4 + 8 + 4 + payloads.len() * entry_len;
    let payloads_len: usize = payloads.iter().map(|p| p.1.len()).sum();
    let mut t = Vec::new();
    t.extend_from_slice(&0xD1C4_3B2E_80A5_F697u64.to_le_bytes());
    t.extend_from_slice(&2i32.to_le_bytes());
    t.extend_from_slice(&(header_len as u32).to_le_bytes());
    t.extend_from_slice(&(payloads_len as u64).to_le_bytes());
    t.extend_from_slice(&(payloads.len() as i32).to_le_bytes());
    let mut offset = 0u64;
    for (id, buf, raw) in payloads {
        t.extend_from_slice(id);
        t.extend_from_slice(&offset.to_le_bytes());
        t.extend_from_slice(&(buf.len() as u64).to_le_bytes());
        t.extend_from_slice(&raw.to_le_bytes());
        t.extend_from_slice(&[0; 4]); // flags, filter reason
        t.push(0); // local
        offset += buf.len() as u64;
    }
    for (_, buf, _) in payloads {
        t.extend_from_slice(buf);
    }
    t.extend_from_slice(&0x29BF_CA04_5138_DE76u64.to_le_bytes());
    t
}
