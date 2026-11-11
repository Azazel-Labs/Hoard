//! Little-endian reader over a byte slice with the Unreal primitives (`FString`, counts, GUIDs) and bounds checks
//! on every read, so malformed files fail with an error instead of a panic or a huge allocation.

use anyhow::{Context, Result, ensure};

#[derive(Debug, Clone)]
pub(crate) struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    pub(crate) fn at(data: &'a [u8], pos: usize) -> Result<Self> {
        ensure!(pos <= data.len(), "offset {pos} is past the end ({} bytes)", data.len());
        Ok(Self { data, pos })
    }
    pub(crate) fn pos(&self) -> usize {
        self.pos
    }
    pub(crate) fn seek(&mut self, pos: i64) -> Result<()> {
        let pos = usize::try_from(pos).context("negative offset")?;
        ensure!(pos <= self.data.len(), "offset {pos} is past the end ({} bytes)", self.data.len());
        self.pos = pos;
        Ok(())
    }
    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(n).context("length overflow")?;
        let s = self.data.get(self.pos..end).context("data ends early")?;
        self.pos = end;
        Ok(s)
    }
    pub(crate) fn skip(&mut self, n: u64) -> Result<()> {
        self.take(usize::try_from(n)?).map(|_| ())
    }
    fn bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let s = self.take(N)?;
        let mut b = [0u8; N];
        b.copy_from_slice(s);
        Ok(b)
    }
    pub(crate) fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes::<1>()?[0])
    }
    pub(crate) fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn i16(&mut self) -> Result<i16> {
        Ok(i16::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn i64(&mut self) -> Result<i64> {
        Ok(i64::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.bytes()?))
    }
    pub(crate) fn u32_be(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.bytes()?))
    }
    pub(crate) fn u64_be(&mut self) -> Result<u64> {
        Ok(u64::from_be_bytes(self.bytes()?))
    }
    /// A count that has to be plausible before anything is allocated for it.
    pub(crate) fn count(&mut self, max: usize) -> Result<usize> {
        let n = self.i32()?;
        let n = usize::try_from(n).with_context(|| format!("negative count {n}"))?;
        ensure!(n <= max, "implausible count {n}");
        Ok(n)
    }
    /// `FString`: length including the terminator; negative means UTF-16.
    pub(crate) fn fstring(&mut self) -> Result<String> {
        let len = self.i32()?;
        if len == 0 {
            return Ok(String::new());
        }
        let n = usize::try_from(len.unsigned_abs())?;
        ensure!(n <= 1 << 20, "implausible string length {n}");
        if len > 0 {
            let s = self.take(n)?;
            let s = s.strip_suffix(&[0]).unwrap_or(s);
            Ok(String::from_utf8_lossy(s).into_owned())
        } else {
            let raw = self.take(n * 2)?;
            let mut units: Vec<u16> = raw.as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
            if units.last() == Some(&0) {
                units.pop();
            }
            Ok(String::from_utf16_lossy(&units))
        }
    }
    pub(crate) fn guid(&mut self) -> Result<[u8; 16]> {
        self.bytes()
    }
}
