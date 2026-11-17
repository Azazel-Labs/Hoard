//! Godot's text resources (`.tres`, `.tscn`): sections (`[ext_resource …]`, `[sub_resource …]`, `[resource]`) of
//! `key = value` lines, values written in Godot's variant syntax (`Rect2(0, 0, 16, 16)`, `SubResource("id")`,
//! `[{ "name": &"walk" }]`), Godot 3's and 4's alike.

use std::collections::HashMap;

/// A value as Godot writes it.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Val {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Arr(Vec<Val>),
    Dict(Vec<(Val, Val)>),
    /// `Rect2(…)`, `Vector2i(…)`, `ExtResource(…)`, `SubResource(…)`, `Color(…)`…
    Call(String, Vec<Val>),
}

impl Val {
    pub(crate) fn num(&self) -> Option<f64> {
        match self {
            Val::Num(n) => Some(*n),
            Val::Bool(b) => Some(f64::from(u8::from(*b))),
            _ => None,
        }
    }

    pub(crate) fn str(&self) -> Option<&str> {
        match self {
            Val::Str(s) => Some(s),
            _ => None,
        }
    }

    /// A dictionary's value under a string key.
    pub(crate) fn get(&self, key: &str) -> Option<&Val> {
        match self {
            Val::Dict(items) => items.iter().find(|(k, _)| k.str() == Some(key)).map(|(_, v)| v),
            _ => None,
        }
    }

    /// The numbers of a constructor (`Rect2(1, 2, 3, 4)` → [1, 2, 3, 4]), or of a number on its own.
    pub(crate) fn nums(&self) -> Vec<f64> {
        match self {
            Val::Call(_, args) => args.iter().filter_map(Val::num).collect(),
            Val::Num(n) => vec![*n],
            _ => Vec::new(),
        }
    }

    /// The id a `SubResource(…)` or `ExtResource(…)` names (a number in Godot 3, a string in 4), as text.
    pub(crate) fn reference(&self, kind: &str) -> Option<String> {
        match self {
            Val::Call(name, args) if name == kind => match args.first()? {
                Val::Str(s) => Some(s.clone()),
                Val::Num(n) => Some(format!("{n}")),
                _ => None,
            },
            _ => None,
        }
    }
}

/// A `[section …]` and the properties under it, in order.
#[derive(Debug, Clone, Default)]
pub(crate) struct Section {
    pub(crate) name: String,
    pub(crate) attrs: HashMap<String, Val>,
    pub(crate) props: Vec<(String, Val)>,
}

impl Section {
    pub(crate) fn attr_str(&self, key: &str) -> Option<String> {
        match self.attrs.get(key)? {
            Val::Str(s) => Some(s.clone()),
            Val::Num(n) => Some(format!("{n}")),
            _ => None,
        }
    }

    pub(crate) fn prop(&self, key: &str) -> Option<&Val> {
        self.props.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

/// A whole resource file: its header (`gd_resource` or `gd_scene`) first, then every other section.
#[derive(Debug, Clone, Default)]
pub(crate) struct Resource {
    pub(crate) sections: Vec<Section>,
}

impl Resource {
    /// The resource's type, from its `[gd_resource type="…"]` header.
    pub(crate) fn kind(&self) -> Option<String> {
        self.sections.first().filter(|s| s.name == "gd_resource").and_then(|s| s.attr_str("type"))
    }

    /// `ExtResource` ids → their paths (`res://…`, or relative to the file).
    pub(crate) fn ext_paths(&self) -> HashMap<String, String> {
        self.sections
            .iter()
            .filter(|s| s.name == "ext_resource")
            .filter_map(|s| Some((s.attr_str("id")?, s.attr_str("path")?)))
            .collect()
    }

    pub(crate) fn sub(&self, id: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.name == "sub_resource" && s.attr_str("id").as_deref() == Some(id))
    }

    /// The `[resource]` section: the resource's own properties.
    pub(crate) fn main(&self) -> Option<&Section> {
        self.sections.iter().find(|s| s.name == "resource")
    }
}

/// Read a text resource. `None` when it isn't one.
pub(crate) fn parse(text: &str) -> Option<Resource> {
    let mut p = Parser { s: text.trim_start_matches('\u{feff}').as_bytes(), i: 0 };
    let mut res = Resource::default();
    loop {
        p.skip_space_and_comments();
        let Some(c) = p.peek() else { break };
        if c == b'[' {
            res.sections.push(p.header()?);
            continue;
        }
        let key = p.key()?;
        let value = p.value()?;
        res.sections.last_mut()?.props.push((key, value));
    }
    res.sections.first().filter(|s| s.name.starts_with("gd_"))?;
    Some(res)
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

/// Deeper than this isn't a resource anyone wrote.
const MAX_DEPTH: usize = 64;

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn skip_space(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_whitespace()) {
            self.i += 1;
        }
    }

    fn skip_space_and_comments(&mut self) {
        loop {
            self.skip_space();
            if self.peek() == Some(b';') {
                while self.peek().is_some_and(|c| c != b'\n') {
                    self.i += 1;
                }
            } else {
                return;
            }
        }
    }

    fn eat(&mut self, c: u8) -> bool {
        self.skip_space();
        if self.peek() == Some(c) {
            self.i += 1;
            true
        } else {
            false
        }
    }

    fn word(&mut self) -> String {
        let start = self.i;
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_') {
            self.i += 1;
        }
        String::from_utf8_lossy(&self.s[start..self.i]).into_owned()
    }

    /// `[name key=value key=value]`.
    fn header(&mut self) -> Option<Section> {
        self.i += 1;
        self.skip_space();
        let name = self.word();
        if name.is_empty() {
            return None;
        }
        let mut attrs = HashMap::new();
        loop {
            self.skip_space();
            if self.peek()? == b']' {
                self.i += 1;
                break;
            }
            let key = self.word();
            if key.is_empty() || !self.eat(b'=') {
                return None;
            }
            attrs.insert(key, self.value_at(0)?);
        }
        Some(Section { name, attrs, props: Vec::new() })
    }

    /// A property's name, up to its `=`: `region`, `sources/0`, `0:0/0`, `0/autotile/tile_size`, `"quoted"`.
    fn key(&mut self) -> Option<String> {
        self.skip_space();
        if self.peek() == Some(b'"') {
            let k = self.string()?;
            return self.eat(b'=').then_some(k);
        }
        let start = self.i;
        while self.peek().is_some_and(|c| c != b'=' && c != b'\n') {
            self.i += 1;
        }
        if self.peek() != Some(b'=') {
            return None;
        }
        let key = String::from_utf8_lossy(&self.s[start..self.i]).trim().to_owned();
        self.i += 1;
        (!key.is_empty()).then_some(key)
    }

    fn value(&mut self) -> Option<Val> {
        self.value_at(0)
    }

    fn value_at(&mut self, depth: usize) -> Option<Val> {
        if depth > MAX_DEPTH {
            return None;
        }
        self.skip_space();
        match self.peek()? {
            b'"' => self.string().map(Val::Str),
            // A StringName: &"walk". A NodePath: ^"path".
            b'&' | b'^' => {
                self.i += 1;
                self.string().map(Val::Str)
            }
            b'[' => {
                self.i += 1;
                let mut items = Vec::new();
                loop {
                    if self.eat(b']') {
                        break;
                    }
                    items.push(self.value_at(depth + 1)?);
                    if !self.eat(b',') && !self.eat(b']') {
                        return None;
                    } else if self.s.get(self.i - 1) == Some(&b']') {
                        break;
                    }
                }
                Some(Val::Arr(items))
            }
            b'{' => {
                self.i += 1;
                let mut items = Vec::new();
                loop {
                    if self.eat(b'}') {
                        break;
                    }
                    let k = self.value_at(depth + 1)?;
                    if !self.eat(b':') {
                        return None;
                    }
                    items.push((k, self.value_at(depth + 1)?));
                    if !self.eat(b',') && !self.eat(b'}') {
                        return None;
                    } else if self.s.get(self.i - 1) == Some(&b'}') {
                        break;
                    }
                }
                Some(Val::Dict(items))
            }
            c if c == b'-' || c == b'+' || c == b'.' || c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == b'_' => {
                let w = self.word();
                match w.as_str() {
                    "true" => return Some(Val::Bool(true)),
                    "false" => return Some(Val::Bool(false)),
                    "null" | "nil" => return Some(Val::Null),
                    "inf" => return Some(Val::Num(f64::INFINITY)),
                    "nan" => return Some(Val::Num(f64::NAN)),
                    _ => {}
                }
                // A typed array: Array[Type]([…]).
                if self.peek() == Some(b'[') {
                    let mut depth_b = 0;
                    while let Some(c) = self.peek() {
                        self.i += 1;
                        match c {
                            b'[' => depth_b += 1,
                            b']' => {
                                depth_b -= 1;
                                if depth_b == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if !self.eat(b'(') {
                    return Some(Val::Str(w));
                }
                let mut args = Vec::new();
                loop {
                    if self.eat(b')') {
                        break;
                    }
                    args.push(self.value_at(depth + 1)?);
                    if !self.eat(b',') && !self.eat(b')') {
                        return None;
                    } else if self.s.get(self.i - 1) == Some(&b')') {
                        break;
                    }
                }
                // A typed array's contents stand for it.
                if w == "Array" && args.len() == 1 && matches!(args[0], Val::Arr(_)) {
                    return args.pop();
                }
                Some(Val::Call(w, args))
            }
            _ => None,
        }
    }

    fn number(&mut self) -> Option<Val> {
        let start = self.i;
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'+' | b'.')) {
            self.i += 1;
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).ok()?;
        text.parse::<f64>().ok().map(Val::Num)
    }

    fn string(&mut self) -> Option<String> {
        if self.peek() != Some(b'"') {
            return None;
        }
        self.i += 1;
        let mut out = Vec::new();
        loop {
            let c = self.peek()?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = self.peek()?;
                    self.i += 1;
                    out.push(match e {
                        b'n' => b'\n',
                        b't' => b'\t',
                        b'r' => b'\r',
                        other => other,
                    });
                }
                other => out.push(other),
            }
        }
        Some(String::from_utf8_lossy(&out).into_owned())
    }
}

/// A string as Godot writes one, quoted and escaped.
pub(crate) fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn godot_4_sprite_frames() {
        let text = r#"[gd_resource type="SpriteFrames" load_steps=4 format=3 uid="uid://b1"]

[ext_resource type="Texture2D" uid="uid://c2" path="res://art/hero.png" id="1_abc"]

[sub_resource type="AtlasTexture" id="AtlasTexture_x1"]
atlas = ExtResource("1_abc")
region = Rect2(0, 0, 16, 24)

[resource]
animations = [{
"frames": [{
"duration": 1.0,
"texture": SubResource("AtlasTexture_x1")
}],
"loop": true,
"name": &"idle",
"speed": 5.0
}]
"#;
        let r = parse(text).expect("a resource");
        assert_eq!(r.kind().as_deref(), Some("SpriteFrames"));
        assert_eq!(r.ext_paths().get("1_abc").map(String::as_str), Some("res://art/hero.png"));
        let sub = r.sub("AtlasTexture_x1").expect("the atlas texture");
        assert_eq!(sub.prop("atlas").and_then(|v| v.reference("ExtResource")).as_deref(), Some("1_abc"));
        assert_eq!(sub.prop("region").map(Val::nums), Some(vec![0.0, 0.0, 16.0, 24.0]));
        let anims = r.main().and_then(|m| m.prop("animations")).expect("animations");
        let Val::Arr(list) = anims else { panic!("an array") };
        assert_eq!(list[0].get("name").and_then(Val::str), Some("idle"));
        assert_eq!(list[0].get("speed").and_then(Val::num), Some(5.0));
        let Some(Val::Arr(frames)) = list[0].get("frames") else { panic!("frames") };
        assert_eq!(
            frames[0].get("texture").and_then(|t| t.reference("SubResource")).as_deref(),
            Some("AtlasTexture_x1")
        );
    }

    #[test]
    fn godot_3_and_odd_values() {
        let text = "[gd_resource type=\"TileSet\" load_steps=2 format=2]\n\n[ext_resource path=\"res://tiles.png\" \
                    type=\"Texture\" id=1]\n\n[resource]\n0/name = \"grass \\\"tall\\\"\"\n0/texture = ExtResource( 1 \
                    )\n0/region = Rect2( 0, 0, 64, 32 )\n0/shapes = [  ]\n0/z_index = -1\n0/modulate = Color( 1, 1, \
                    1, 1 )\n0/occluder_offset = Vector2( 0, 0 )\n; a comment\n0/flags = Array[int]([1, 2])\n";
        let r = parse(text).expect("a resource");
        assert_eq!(r.ext_paths().get("1").map(String::as_str), Some("res://tiles.png"));
        let m = r.main().expect("resource");
        assert_eq!(m.prop("0/name").and_then(Val::str), Some("grass \"tall\""));
        assert_eq!(m.prop("0/texture").and_then(|v| v.reference("ExtResource")).as_deref(), Some("1"));
        assert_eq!(m.prop("0/region").map(Val::nums), Some(vec![0.0, 0.0, 64.0, 32.0]));
        assert_eq!(m.prop("0/z_index").and_then(Val::num), Some(-1.0));
        assert_eq!(m.prop("0/flags"), Some(&Val::Arr(vec![Val::Num(1.0), Val::Num(2.0)])));
        assert!(parse("just text = 1").is_none());
        assert!(parse("[gd_resource type=\"X\"]\nbroken = [1, 2").is_none());
    }
}
