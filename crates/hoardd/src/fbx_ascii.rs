//! ASCII FBX (7.x), the text form some tools write (Asset Forge, which made many of Kenney's older kits, and others
//! on request), rewritten as binary FBX so it's read like any other: by `fbx` for thumbnails and GLB copies, and
//! by Blender, whose importer takes only binary. Values get the types binary FBX gives them, since Blender checks:
//! object ids and connections are 64-bit, a property's values follow the type it declares, and an array takes the
//! type its node always has.

use anyhow::{Context, Result, bail, ensure};
use base64::Engine as _;
use fbxcel::writer::v7400::binary::{FbxFooter, Writer};

const BINARY_MAGIC: &[u8] = b"Kaydara FBX Binary";

/// Whether `data` is an ASCII FBX file.
pub(crate) fn is_ascii(data: &[u8]) -> bool {
    !data.starts_with(BINARY_MAGIC) && {
        let head = &data[..data.len().min(64 << 10)];
        head.windows(19).any(|w| w == b"FBXHeaderExtension:")
    }
}

/// Rewrite an ASCII FBX file as binary FBX 7.4.
pub(crate) fn to_binary(data: &[u8]) -> Result<Vec<u8>> {
    let text = std::str::from_utf8(data)
        .map_or_else(|_| String::from_utf8_lossy(data).into_owned(), str::to_owned)
        .trim_start_matches('\u{feff}')
        .to_owned();
    if let Some(v) = text.lines().next().and_then(|l| l.strip_prefix("; FBX ")).and_then(|r| r.split('.').next())
        && let Ok(major) = v.trim().parse::<u32>()
    {
        ensure!(major >= 7, "this is FBX {major}, older than FBX 7, which neither Hoard nor Blender can read");
    }
    let mut lex = Lexer { src: &text, i: 0, line: 1, peeked: None };
    let mut w = Writer::new(std::io::Cursor::new(Vec::new()), fbxcel::low::FbxVersion::V7_4)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    while let Some(tok) = lex.next()? {
        let Tok::Name(name) = tok else { bail!("line {}: expected a node name", lex.line) };
        node(&mut lex, &mut w, name, "")?;
    }
    let out = w.finalize_and_flush(&FbxFooter::default()).map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(out.into_inner())
}

#[derive(Debug)]
enum Tok<'a> {
    /// `Name:`
    Name(&'a str),
    Str(&'a str),
    Num(&'a str),
    /// A bare word: `T`, `Y`, `N`, `F`, …
    Word(&'a str),
    Comma,
    Open,
    Close,
    Star,
}

struct Lexer<'a> {
    src: &'a str,
    i: usize,
    line: usize,
    peeked: Option<Tok<'a>>,
}

impl<'a> Lexer<'a> {
    fn peek(&mut self) -> Result<Option<&Tok<'a>>> {
        if self.peeked.is_none() {
            self.peeked = self.lex()?;
        }
        Ok(self.peeked.as_ref())
    }

    fn next(&mut self) -> Result<Option<Tok<'a>>> {
        match self.peeked.take() {
            Some(t) => Ok(Some(t)),
            None => self.lex(),
        }
    }

    fn lex(&mut self) -> Result<Option<Tok<'a>>> {
        let b = self.src.as_bytes();
        loop {
            match b.get(self.i) {
                None => return Ok(None),
                Some(b'\n') => {
                    self.line += 1;
                    self.i += 1;
                }
                Some(c) if c.is_ascii_whitespace() => self.i += 1,
                Some(b';') => {
                    while self.i < b.len() && b[self.i] != b'\n' {
                        self.i += 1;
                    }
                }
                Some(_) => break,
            }
        }
        let start = self.i;
        let c = b[start];
        self.i += 1;
        Ok(Some(match c {
            b'{' => Tok::Open,
            b'}' => Tok::Close,
            b',' => Tok::Comma,
            b'*' => Tok::Star,
            b'"' => {
                let end = memchr(b'"', &b[self.i..]).with_context(|| format!("line {}: unclosed string", self.line))?;
                let s = &self.src[self.i..self.i + end];
                self.line += s.matches('\n').count();
                self.i += end + 1;
                Tok::Str(s)
            }
            b'0'..=b'9' | b'-' | b'+' | b'.' => {
                while self.i < b.len()
                    && !matches!(b[self.i], b',' | b'{' | b'}' | b';')
                    && !b[self.i].is_ascii_whitespace()
                {
                    self.i += 1;
                }
                Tok::Num(&self.src[start..self.i])
            }
            c if c.is_ascii_alphabetic() || c == b'_' => {
                while self.i < b.len() && (b[self.i].is_ascii_alphanumeric() || matches!(b[self.i], b'_' | b'|' | b'-'))
                {
                    self.i += 1;
                }
                let word = &self.src[start..self.i];
                let mut j = self.i;
                while j < b.len() && matches!(b[j], b' ' | b'\t') {
                    j += 1;
                }
                if b.get(j) == Some(&b':') {
                    self.i = j + 1;
                    Tok::Name(word)
                } else {
                    Tok::Word(word)
                }
            }
            _ => bail!("line {}: unexpected {:?}", self.line, char::from(c)),
        }))
    }
}

fn memchr(needle: u8, hay: &[u8]) -> Option<usize> {
    hay.iter().position(|&c| c == needle)
}

enum Val<'a> {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Arr(Vec<f64>),
    /// Kept for `Content`, whose strings are base64 to join and decode.
    Raw(&'a str),
}

fn number(s: &str) -> Val<'static> {
    if !s.contains(['.', 'e', 'E', '#'])
        && let Ok(i) = s.parse::<i64>()
    {
        return Val::Int(i);
    }
    // `1.#INF` and the like, from old Windows exporters, have no use here.
    Val::Float(s.parse().unwrap_or(0.0))
}

/// `Geometry::Rock` → binary FBX's `Rock\0\x01Geometry`.
fn object_name(s: &str) -> String {
    let s = s.replace("&quot;", "\"");
    if let Some((class, name)) = s.split_once("::")
        && !class.is_empty()
        && class.bytes().all(|c| c.is_ascii_alphanumeric())
    {
        return format!("{name}\0\x01{class}");
    }
    s
}

/// One node: its values, then its children if it has a block. `parent` decides how ids are typed.
fn node<'a>(lex: &mut Lexer<'a>, w: &mut Writer<std::io::Cursor<Vec<u8>>>, name: &'a str, parent: &str) -> Result<()> {
    let mut vals: Vec<Val<'a>> = Vec::new();
    let mut block = false;
    loop {
        match lex.peek()? {
            None | Some(Tok::Name(_) | Tok::Close) => break,
            Some(Tok::Open) => {
                lex.next()?;
                block = true;
                break;
            }
            Some(_) => {}
        }
        match lex.next()?.context("unreachable")? {
            Tok::Comma => {}
            Tok::Num(s) => vals.push(number(s)),
            Tok::Str(s) if name == "Content" => vals.push(Val::Raw(s)),
            Tok::Str(s) => vals.push(Val::Str(object_name(s))),
            Tok::Word(s) => vals.push(match s {
                "T" | "Y" => Val::Bool(true),
                "F" | "N" => Val::Bool(false),
                _ => Val::Str(s.to_owned()),
            }),
            Tok::Star => {
                // `*N { a: v,v,… }`
                let n = match lex.next()? {
                    Some(Tok::Num(n)) => n.parse::<usize>().unwrap_or(0),
                    _ => bail!("line {}: expected an array length", lex.line),
                };
                ensure!(matches!(lex.next()?, Some(Tok::Open)), "line {}: expected an array", lex.line);
                ensure!(matches!(lex.next()?, Some(Tok::Name("a"))), "line {}: expected an array's values", lex.line);
                let mut arr = Vec::with_capacity(n.min(1 << 24));
                loop {
                    match lex.next()? {
                        Some(Tok::Num(s)) => arr.push(match number(s) {
                            Val::Int(i) => i as f64,
                            Val::Float(f) => f,
                            _ => 0.0,
                        }),
                        Some(Tok::Comma) => {}
                        Some(Tok::Close) => break,
                        _ => bail!("line {}: unexpected value in an array", lex.line),
                    }
                }
                vals.push(Val::Arr(arr));
            }
            Tok::Name(_) | Tok::Open | Tok::Close => unreachable!(),
        }
    }
    write_node(w, name, parent, vals)?;
    if block {
        loop {
            match lex.next()? {
                Some(Tok::Close) => break,
                Some(Tok::Name(child)) => node(lex, w, child, name)?,
                Some(t) => bail!("line {}: expected a node, found {t:?}", lex.line),
                None => bail!("the file ends inside {name}"),
            }
        }
    }
    w.close_node().map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}

/// How a number is stored in binary FBX.
#[derive(Clone, Copy, PartialEq)]
enum Ty {
    I32,
    I64,
    F32,
    F64,
    /// By its value: a 32-bit integer if it fits, otherwise 64-bit; a double if it has a point.
    Auto,
}

/// A `P` property's values, by the type it declares.
fn property_type(declared: &str) -> Ty {
    match declared {
        "int"
        | "Integer"
        | "enum"
        | "bool"
        | "Bool"
        | "Visibility Inheritance"
        | "Short"
        | "UShort"
        | "UInteger"
        | "ULong" => Ty::I32,
        "KTime" | "ULongLong" | "LongLong" => Ty::I64,
        "double" | "Number" | "Vector3D" | "Vector" | "Vector4D" | "ColorRGB" | "Color" | "ColorAndAlpha"
        | "ColorRGBA" | "Lcl Translation" | "Lcl Rotation" | "Lcl Scaling" | "Visibility" | "FieldOfView"
        | "FieldOfViewX" | "FieldOfViewY" | "Float" | "Real" | "Distance" | "Opacity" => Ty::F64,
        _ => Ty::Auto,
    }
}

/// An array node's element type, as binary FBX always stores it. Anything else is doubles.
fn array_type(name: &str) -> Ty {
    match name {
        "PolygonVertexIndex" | "Edges" | "UVIndex" | "NormalsIndex" | "NormalIndex" | "BinormalsIndex"
        | "BinormalIndex" | "TangentsIndex" | "TangentIndex" | "ColorIndex" | "Materials" | "Smoothing" | "Indexes"
        | "KeyAttrFlags" | "KeyAttrRefCount" | "TextureId" | "PolygonGroup" | "VertexCreaseIndex" => Ty::I32,
        "KeyTime" => Ty::I64,
        "KeyValueFloat" | "KeyAttrDataFloat" => Ty::F32,
        _ => Ty::F64,
    }
}

fn write_node(w: &mut Writer<std::io::Cursor<Vec<u8>>>, name: &str, parent: &str, vals: Vec<Val<'_>>) -> Result<()> {
    let err = |e: fbxcel::writer::v7400::binary::Error| anyhow::anyhow!("{e}");
    let declared = if name == "P" {
        match vals.get(1) {
            Some(Val::Str(t)) => property_type(t),
            _ => Ty::Auto,
        }
    } else {
        Ty::Auto
    };
    let mut a = w.new_node(name).map_err(err)?;
    if name == "Content" {
        let joined: String = vals.iter().filter_map(|v| if let Val::Raw(s) = v { Some(*s) } else { None }).collect();
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(joined.split_whitespace().collect::<String>())
            .unwrap_or_default();
        return a.append_binary_direct(&bytes).map_err(err);
    }
    for (i, v) in vals.into_iter().enumerate() {
        let ty = if name == "P" && i >= 4 {
            declared
        } else if name == "C" || (parent == "Objects" && i == 0) {
            Ty::I64
        } else {
            Ty::Auto
        };
        match v {
            Val::Int(n) => match ty {
                Ty::I32 => a.append_i32(n as i32),
                Ty::I64 => a.append_i64(n),
                Ty::F32 => a.append_f32(n as f32),
                Ty::F64 => a.append_f64(n as f64),
                Ty::Auto => match i32::try_from(n) {
                    Ok(n) => a.append_i32(n),
                    Err(_) => a.append_i64(n),
                },
            },
            Val::Float(f) => match ty {
                Ty::I32 => a.append_i32(f as i32),
                Ty::I64 => a.append_i64(f as i64),
                Ty::F32 => a.append_f32(f as f32),
                Ty::F64 | Ty::Auto => a.append_f64(f),
            },
            Val::Str(s) => a.append_string_direct(&s),
            Val::Raw(s) => a.append_string_direct(s),
            Val::Bool(b) => a.append_bool(b),
            Val::Arr(xs) => match array_type(name) {
                Ty::I32 => a.append_arr_i32_from_iter(None, xs.into_iter().map(|x| x as i32)),
                Ty::I64 => a.append_arr_i64_from_iter(None, xs.into_iter().map(|x| x as i64)),
                Ty::F32 => a.append_arr_f32_from_iter(None, xs.into_iter().map(|x| x as f32)),
                Ty::F64 | Ty::Auto => a.append_arr_f64_from_iter(None, xs),
            },
        }
        .map_err(err)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fbxcel::low::v7400::AttributeValue as A;

    const QUAD: &str = r#"; FBX 7.3.0 project file
; ----------------------------------------------------
FBXHeaderExtension:  {
	FBXHeaderVersion: 1003
	FBXVersion: 7300
	Creator: "Made using Asset Forge (www.assetforge.io)"
	SceneInfo: "SceneInfo::GlobalInfo", "UserData" {
		Type: "UserData"
	}
}
GlobalSettings:  {
	Version: 1000
	Properties70:  {
		P: "UpAxis", "int", "Integer", "",1
		P: "UnitScaleFactor", "double", "Number", "",100
		P: "TimeSpanStop", "KTime", "Time", "",479181389250
	}
}
Objects:  {
	Geometry: 2295830373648, "Geometry::Quad", "Mesh" {
		Vertices: *12 {
			a: 0,0,0,100,0,0,
100,100,0,0,100,0
		}
		PolygonVertexIndex: *4 {
			a: 0,1,2,-4
		}
		LayerElementNormal: 0 {
			Version: 101
			MappingInformationType: "ByPolygonVertex"
			Normals: *0 {
				a:
			}
		}
	}
	Model: 20, "Model::Floor", "Mesh" {
		Properties70:  {
			P: "Lcl Translation", "Lcl Translation", "", "A",0,0,1.5e2
		}
		Shading: T
		Culling: "CullingOff"
	}
	Video: 50, "Video::Tex", "Clip" {
		Content: , "aGVs",
		  "bG8="
	}
}
Connections:  {
	;Geometry::Quad, Model::Floor
	C: "OO",2295830373648,20
	C: "OO",20,0
}
"#;

    #[test]
    fn ascii_becomes_binary_with_binary_types() {
        assert!(is_ascii(QUAD.as_bytes()));
        let bin = to_binary(QUAD.as_bytes()).unwrap();
        assert!(!is_ascii(&bin));
        let fbxcel::tree::any::AnyTree::V7400(_, tree, _) =
            fbxcel::tree::any::AnyTree::from_seekable_reader(std::io::Cursor::new(bin)).unwrap()
        else {
            panic!("not 7.4")
        };
        let root = tree.root();
        let info = root.first_child_by_name("FBXHeaderExtension").unwrap().first_child_by_name("SceneInfo").unwrap();
        assert_eq!(info.attributes()[0], A::String("GlobalInfo\0\x01SceneInfo".into()));
        let p70 = root.first_child_by_name("GlobalSettings").unwrap().first_child_by_name("Properties70").unwrap();
        let props: Vec<_> = p70.children_by_name("P").map(|p| p.attributes()[4].clone()).collect();
        assert_eq!(props, [A::I32(1), A::F64(100.0), A::I64(479_181_389_250)]);

        let objects = root.first_child_by_name("Objects").unwrap();
        let geom = objects.first_child_by_name("Geometry").unwrap();
        assert_eq!(geom.attributes()[0], A::I64(2_295_830_373_648));
        assert_eq!(geom.attributes()[1], A::String("Quad\0\x01Geometry".into()));
        let verts = geom.first_child_by_name("Vertices").unwrap();
        assert_eq!(verts.attributes()[0], A::ArrF64(vec![0., 0., 0., 100., 0., 0., 100., 100., 0., 0., 100., 0.]));
        let idx = geom.first_child_by_name("PolygonVertexIndex").unwrap();
        assert_eq!(idx.attributes()[0], A::ArrI32(vec![0, 1, 2, -4]));
        let normals = geom.first_child_by_name("LayerElementNormal").unwrap();
        assert_eq!(normals.attributes()[0], A::I32(0));
        assert_eq!(normals.first_child_by_name("Normals").unwrap().attributes()[0], A::ArrF64(vec![]));
        let model = objects.first_child_by_name("Model").unwrap();
        let t = model.first_child_by_name("Properties70").unwrap().first_child_by_name("P").unwrap();
        assert_eq!(&t.attributes()[4..], [A::F64(0.0), A::F64(0.0), A::F64(150.0)]);
        assert_eq!(model.first_child_by_name("Shading").unwrap().attributes()[0], A::Bool(true));
        let content = objects.first_child_by_name("Video").unwrap().first_child_by_name("Content").unwrap();
        assert_eq!(content.attributes()[0], A::Binary(b"hello".to_vec()));
        let c: Vec<_> =
            root.first_child_by_name("Connections").unwrap().children().map(|c| c.attributes().to_vec()).collect();
        assert_eq!(c[0], [A::String("OO".into()), A::I64(2_295_830_373_648), A::I64(20)]);
    }

    #[test]
    fn fbx_6_is_refused_by_name() {
        let err = to_binary(b"; FBX 6.1.0 project file\nFBXHeaderExtension:  {\n}\n").unwrap_err();
        assert!(format!("{err}").contains("FBX 6"), "{err}");
    }
}
