//! Bounds-checked little-endian reads, so malformed files fail with an error rather than a panic.

use anyhow::{Context, Result, bail};

/// The three bytes that open every length-prefixed UTF-16 string in a SketchUp file.
pub(crate) const STRING_MARKER: [u8; 3] = [0xff, 0xfe, 0xff];

/// A cursor over a byte slice. Every read checks the bounds first.
#[derive(Debug, Clone)]
pub(crate) struct Reader<'a> {
    pub(crate) d: &'a [u8],
    pub(crate) p: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(d: &'a [u8]) -> Self {
        Self { d, p: 0 }
    }

    /// The next `n` bytes, moving past them.
    pub(crate) fn raw(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.p.checked_add(n).context("archive offset overflows")?;
        let bytes = self.d.get(self.p..end).context("archive ends early")?;
        self.p = end;
        Ok(bytes)
    }

    pub(crate) fn skip(&mut self, n: usize) -> Result<()> {
        self.raw(n).map(|_| ())
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut out = [0; N];
        out.copy_from_slice(self.raw(N)?);
        Ok(out)
    }

    pub(crate) fn u8(&mut self) -> Result<u8> {
        Ok(self.array::<1>()?[0])
    }

    pub(crate) fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.array()?))
    }

    pub(crate) fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    pub(crate) fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.array()?))
    }

    pub(crate) fn f64s<const N: usize>(&mut self) -> Result<[f64; N]> {
        let mut out = [0.0; N];
        for v in &mut out {
            *v = self.f64()?;
        }
        Ok(out)
    }

    /// The u16 at the cursor without moving it.
    pub(crate) fn peek_u16(&self) -> Result<u16> {
        u16_at(self.d, self.p).context("archive ends early")
    }

    /// Whether a string record starts at the cursor.
    pub(crate) fn marker(&self) -> bool {
        marker_at(self.d, self.p)
    }

    /// A SketchUp string record: the marker, a length that widens through u8, u16 and u32, then UTF-16LE.
    pub(crate) fn utf16(&mut self) -> Result<String> {
        if !self.marker() {
            bail!("expected a string record at offset {}", self.p);
        }
        self.p += 3;
        let mut len = u32::from(self.u8()?);
        if len == 0xff {
            len = u32::from(self.u16()?);
            if len == 0xffff {
                len = self.u32()?;
            }
        }
        let bytes = self.raw((len as usize).checked_mul(2).context("string length overflows")?)?;
        Ok(decode_utf16le(bytes))
    }
}

/// UTF-16LE to a Rust string, with unpaired surrogates replaced.
pub(crate) fn decode_utf16le(bytes: &[u8]) -> String {
    let units = bytes.as_chunks::<2>().0.iter().map(|&c| u16::from_le_bytes(c));
    char::decode_utf16(units).map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER)).collect()
}

pub(crate) fn u16_at(d: &[u8], at: usize) -> Option<u16> {
    let b = d.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

pub(crate) fn u32_at(d: &[u8], at: usize) -> Option<u32> {
    let b = d.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

pub(crate) fn f64_at(d: &[u8], at: usize) -> Option<f64> {
    let b = d.get(at..at.checked_add(8)?)?;
    let mut a = [0; 8];
    a.copy_from_slice(b);
    Some(f64::from_le_bytes(a))
}

pub(crate) fn marker_at(d: &[u8], at: usize) -> bool {
    at.checked_add(3).and_then(|end| d.get(at..end)) == Some(&STRING_MARKER[..])
}

/// Whether `needle` sits at `at`.
pub(crate) fn bytes_at(d: &[u8], at: usize, needle: &[u8]) -> bool {
    at.checked_add(needle.len()).and_then(|end| d.get(at..end)) == Some(needle)
}

/// The byte at `at`, or `None` past the end.
pub(crate) fn byte_at(d: &[u8], at: usize) -> Option<u8> {
    d.get(at).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_little_endian_values() {
        let d = [1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xf0, 0x3f];
        let mut r = Reader::new(&d);
        assert_eq!(r.u16().unwrap(), 1);
        assert_eq!(r.u32().unwrap(), 2);
        assert!((r.f64().unwrap() - 1.0).abs() < 1e-12);
        assert!(r.u8().is_err());
    }

    #[test]
    fn reads_string_records() {
        let mut d = vec![0xff, 0xfe, 0xff, 2, b'h', 0, b'i', 0];
        let mut r = Reader::new(&d);
        assert_eq!(r.utf16().unwrap(), "hi");
        // the u16 length form
        d = vec![0xff, 0xfe, 0xff, 0xff, 1, 0, b'x', 0];
        assert_eq!(Reader::new(&d).utf16().unwrap(), "x");
        // a length past the end fails instead of panicking
        d = vec![0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        assert!(Reader::new(&d).utf16().is_err());
        assert!(Reader::new(&[1, 2, 3]).utf16().is_err());
    }

    #[test]
    fn replaces_unpaired_surrogates() {
        assert_eq!(decode_utf16le(&[0x00, 0xd8, b'a', 0]), "\u{fffd}a");
        assert_eq!(decode_utf16le(&[0x3d, 0xd8, 0x00, 0xde]), "\u{1f600}");
    }

    #[test]
    fn peeks_are_bounded() {
        assert_eq!(u16_at(&[1], 0), None);
        assert_eq!(u32_at(&[1, 2, 3, 4], usize::MAX), None);
        assert!(!marker_at(&[0xff, 0xfe], 0));
        assert!(bytes_at(b"xCLayer", 1, b"CLayer"));
    }
}
