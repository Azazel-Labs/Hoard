//! Rive files (`.riv`): interactive vector animations, played in the browser with Rive's web runtime, which also
//! draws their thumbnails (see `previews::put_thumb`).
//!
//! A `.riv` is a header (`RIVE`, the format's major and minor version, a file id, and the backing types of the
//! properties an older runtime might not know) and then objects, each a type key and properties until a zero key.
//! Only the runtime knows every property's type, so the server reads up to the first artboard: its name and size.

use serde_json::{Map, Value, json};

/// How much of the file is looked at: the first artboard comes right after the header.
pub(crate) const HEAD: usize = 64 << 10;

const BACKBOARD: u64 = 23;
const ARTBOARD: u64 = 1;

/// How a property's value is written.
#[derive(Clone, Copy, PartialEq)]
enum Field {
    /// A variable-length unsigned integer, or a bool.
    Uint,
    /// A length, then that many bytes.
    String,
    /// A 32-bit float.
    Double,
    /// A 32-bit color.
    Color,
}

/// The types of the properties a backboard or an artboard has, whatever version wrote them.
fn known(key: u64) -> Option<Field> {
    match key {
        4 => Some(Field::String),
        // Width, height, x, y and origin; translation, rotation, scale and opacity.
        7..=18 => Some(Field::Double),
        // Parent, clip, default state machine, and other ids and flags.
        5 | 196 | 236 => Some(Field::Uint),
        _ => None,
    }
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn uint(&mut self) -> Option<u64> {
        let mut out = 0u64;
        for shift in (0..64).step_by(7) {
            let c = *self.b.get(self.at)?;
            self.at += 1;
            out |= u64::from(c & 0x7f) << shift;
            if c & 0x80 == 0 {
                return Some(out);
            }
        }
        None
    }
    fn bytes(&mut self, n: usize) -> Option<&[u8]> {
        let s = self.b.get(self.at..self.at.checked_add(n)?)?;
        self.at += n;
        Some(s)
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.bytes(4)?.try_into().ok()?))
    }
    /// One property's value, or `None` at the end of the data.
    fn value(&mut self, field: Field) -> Option<Value> {
        Some(match field {
            Field::Uint => json!(self.uint()?),
            Field::String => {
                let n = usize::try_from(self.uint()?).ok()?;
                json!(String::from_utf8_lossy(self.bytes(n)?))
            }
            Field::Double => json!(f32::from_bits(self.u32()?)),
            Field::Color => json!(self.u32()?),
        })
    }
}

/// What the asset's meta keeps of a Rive file, or `None` when it isn't one: the format version (`rive`), and the first
/// artboard's name and size when they can be read.
pub(crate) fn facts(head: &[u8]) -> Option<Map<String, Value>> {
    let rest = head.strip_prefix(b"RIVE")?;
    let mut r = Reader { b: rest, at: 0 };
    let (major, minor, _file) = (r.uint()?, r.uint()?, r.uint()?);
    let mut m = Map::new();
    m.insert("rive".into(), json!(format!("{major}.{minor}")));
    // The table of contents: property keys, then their types, two bits each in the low byte of a 32-bit word.
    let mut keys = Vec::new();
    loop {
        match r.uint()? {
            0 => break,
            k => keys.push(k),
        }
    }
    let mut toc = std::collections::HashMap::new();
    for chunk in keys.chunks(4) {
        let word = r.u32()?;
        for (i, k) in chunk.iter().enumerate() {
            let field = match (word >> (i * 2)) & 3 {
                0 => Field::Uint,
                1 => Field::String,
                2 => Field::Double,
                _ => Field::Color,
            };
            toc.insert(*k, field);
        }
    }
    let field = |k: u64| known(k).or_else(|| toc.get(&k).copied());
    // An object's properties, up to its end or to one of unknown type (which ends the reading); and whether the end
    // was reached.
    let props = |r: &mut Reader| -> (Vec<(u64, Value)>, bool) {
        let mut out = Vec::new();
        while let Some(k) = r.uint() {
            if k == 0 {
                return (out, true);
            }
            let Some(v) = field(k).and_then(|f| r.value(f)) else { break };
            out.push((k, v));
        }
        (out, false)
    };
    // The backboard, then the first artboard.
    let mut ty = r.uint();
    if ty == Some(BACKBOARD) {
        if !props(&mut r).1 {
            return Some(m);
        }
        ty = r.uint();
    }
    if ty != Some(ARTBOARD) {
        return Some(m);
    }
    for (k, v) in props(&mut r).0 {
        match k {
            4 => {
                if let Some(name) = v.as_str().map(str::trim).filter(|n| !n.is_empty()) {
                    m.insert("rive_artboard".into(), json!(name));
                }
            }
            7 | 8 => {
                if let Some(n) = v.as_f64().filter(|n| n.is_finite() && *n >= 1.0) {
                    m.insert(if k == 7 { "width" } else { "height" }.into(), json!(n.round() as u64));
                }
            }
            _ => {}
        }
    }
    Some(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f32s(n: f32) -> [u8; 4] {
        n.to_le_bytes()
    }

    #[test]
    fn the_first_artboard_of_a_rive_file() {
        let mut b = b"RIVE".to_vec();
        // Version 7.0, file 2639, one property in the table of contents (160, a uint), then its type word.
        b.extend([7, 0, 0xcf, 0x14, 0xa0, 0x01, 0, 0, 0, 0, 0]);
        // The backboard, empty; the artboard "Truck", 1920 by 1080, at -2023, 125, with property 160 too.
        b.extend([23, 0, 1, 4, 5]);
        b.extend(b"Truck");
        b.push(7);
        b.extend(f32s(1920.0));
        b.push(8);
        b.extend(f32s(1080.0));
        b.push(160);
        b.push(3);
        b.push(9);
        b.extend(f32s(-2023.0));
        b.push(0);
        let m = facts(&b).unwrap();
        assert_eq!(m["rive"], "7.0");
        assert_eq!(m["rive_artboard"], "Truck");
        assert_eq!((m["width"].as_u64(), m["height"].as_u64()), (Some(1920), Some(1080)));

        // An unknown property after the name: the name is kept, the size isn't known.
        let mut b = b"RIVE".to_vec();
        b.extend([7, 0, 1, 0, 23, 0, 1, 4, 3]);
        b.extend(b"Box");
        b.extend([99, 1, 7]);
        let m = facts(&b).unwrap();
        assert_eq!(m["rive_artboard"], "Box");
        assert!(m.get("width").is_none());

        assert!(facts(b"PK\x03\x04").is_none());
    }
}
