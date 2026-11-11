//! Bulk data: the large payloads (texture source pixels, imported audio, mesh descriptions) that live outside an
//! object's tagged properties.
//!
//! - UE5 editor packages store them in a *package trailer* at the end of the file: a lookup table of payloads,
//!   each an `FCompressedBuffer` (64-byte header, then one raw block or a table of compressed blocks).
//!   Objects point at payloads with an `FEditorBulkData` record carrying the payload's content id.
//! - UE4 packages use `FByteBulkData`: a small header, then the payload inline or at an offset in the file.

use anyhow::{Context, Result, bail, ensure};

use crate::package::Package;
use crate::reader::Reader;

const TRAILER_HEADER_TAG: u64 = 0xD1C4_3B2E_80A5_F697;
const TRAILER_FOOTER_TAG: u64 = 0x29BF_CA04_5138_DE76;
const COMPRESSED_BUFFER_MAGIC: u32 = 0xb775_6362;
const MAX_PAYLOAD: u64 = 4 << 30;

/// Where a trailer payload lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    /// In this file.
    Local,
    /// In another package's trailer (a workspace-domain file we don't have).
    Referenced,
    /// In a virtualization backend (a shared cache we don't have).
    Virtualized,
}

#[derive(Debug, Clone)]
pub struct TrailerEntry {
    /// `FIoHash` of the payload: what `FEditorBulkData` refers to.
    pub id: [u8; 20],
    /// Absolute offset in the file (for local payloads).
    pub offset: i64,
    pub compressed_size: u64,
    pub raw_size: u64,
    pub access: Access,
}

/// The payload table at the end of a UE5 editor package.
pub fn read_trailer(data: &[u8], pkg: &Package) -> Result<Vec<TrailerEntry>> {
    let start = pkg.payload_toc_offset.context("package has no trailer")?;
    let mut r = Reader::at(data, usize::try_from(start)?)?;
    ensure!(r.u64()? == TRAILER_HEADER_TAG, "bad package trailer tag");
    let version = r.i32()?;
    let header_len = u64::from(r.u32()?);
    let payloads_len = r.u64()?;
    ensure!(
        u64::try_from(start)? + header_len + payloads_len <= data.len() as u64,
        "package trailer runs past the end of the file"
    );
    let legacy_access = if version < 1 { Some(r.u8()?) } else { None };
    let n = r.count(1 << 20)?;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let mut id = [0u8; 20];
        id.copy_from_slice(r.take(20)?);
        let offset = r.i64()?;
        let compressed_size = r.u64()?;
        let raw_size = r.u64()?;
        if version >= 2 {
            r.u16()?; // payload flags
            r.u16()?; // filter reason
        }
        let mode = if version >= 1 {
            r.u8()?
        } else if offset == -1 {
            2
        } else {
            legacy_access.unwrap_or(0)
        };
        let access = match mode {
            0 => Access::Local,
            1 => Access::Referenced,
            _ => Access::Virtualized,
        };
        // Local offsets are relative to the end of the trailer header.
        let offset = if access == Access::Local { start + i64::try_from(header_len)? + offset } else { offset };
        out.push(TrailerEntry { id, offset, compressed_size, raw_size, access });
    }
    // The footer confirms we read the right thing.
    let mut f = Reader::at(data, usize::try_from(start)? + usize::try_from(header_len + payloads_len)?)?;
    ensure!(f.u64()? == TRAILER_FOOTER_TAG, "bad package trailer footer");
    Ok(out)
}

/// Why a payload couldn't be decoded here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Undecodable {
    /// Oodle-compressed (Unreal's default for raw source data); only Unreal can decode it.
    Oodle,
    /// Stored outside this file (virtualized, or in a workspace-domain package).
    NotInFile,
    Unknown(String),
}

/// Decode an `FCompressedBuffer` at `offset`.
pub fn decode_compressed_buffer(data: &[u8], offset: usize) -> Result<Vec<u8>> {
    let mut r = Reader::at(data, offset)?;
    ensure!(r.u32_be()? == COMPRESSED_BUFFER_MAGIC, "not a compressed buffer");
    r.u32_be()?; // crc32
    let method = r.u8()?;
    r.u8()?; // compressor
    r.u8()?; // level
    let block_exp = r.u8()?;
    let block_count = usize::try_from(r.u32_be()?)?;
    let raw_size = r.u64_be()?;
    let compressed_size = r.u64_be()?;
    r.skip(32)?; // raw hash (BLAKE3-160, padded)
    ensure!(raw_size <= MAX_PAYLOAD && compressed_size <= MAX_PAYLOAD, "implausible payload size");
    let raw_size = usize::try_from(raw_size)?;
    match method {
        0 => Ok(r.take(raw_size)?.to_vec()),
        3 | 4 => {
            ensure!(block_count <= 1 << 20 && block_exp < 40, "implausible block layout");
            let block_size = 1usize << block_exp;
            let mut sizes = Vec::with_capacity(block_count);
            for _ in 0..block_count {
                sizes.push(usize::try_from(r.u32_be()?)?);
            }
            let mut out = Vec::with_capacity(raw_size);
            for (i, &csize) in sizes.iter().enumerate() {
                let block = r.take(csize)?;
                let want = if i + 1 == block_count {
                    raw_size.checked_sub(block_size * i).context("blocks exceed the payload size")?
                } else {
                    block_size
                };
                if csize == want {
                    // Blocks that didn't shrink are stored raw.
                    out.extend_from_slice(block);
                    continue;
                }
                // Oodle (3) blocks that did shrink need Oodle itself.
                ensure!(method == 4, UndecodableError(Undecodable::Oodle));
                out.extend(lz4_flex::block::decompress(block, want).context("LZ4 block")?);
            }
            ensure!(out.len() == raw_size, "payload decoded to {} bytes, expected {raw_size}", out.len());
            Ok(out)
        }
        m => bail!("unknown compressed buffer method {m}"),
    }
}

/// Error wrapper so callers can tell "needs Unreal" apart from corruption.
#[derive(Debug)]
pub struct UndecodableError(pub Undecodable);

impl std::fmt::Display for UndecodableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Undecodable::Oodle => write!(f, "payload is Oodle-compressed, which only Unreal can decode"),
            Undecodable::NotInFile => write!(f, "payload isn't stored in this file (virtualized or referenced)"),
            Undecodable::Unknown(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for UndecodableError {}

/// Decode a trailer payload.
pub fn decode_entry(data: &[u8], entry: &TrailerEntry) -> Result<Vec<u8>> {
    if entry.access != Access::Local || entry.offset < 0 {
        bail!(UndecodableError(Undecodable::NotInFile));
    }
    decode_compressed_buffer(data, usize::try_from(entry.offset)?)
}

/// An `FEditorBulkData` record (UE5): which payload an object uses.
#[derive(Debug, Clone)]
pub struct EditorBulkData {
    pub flags: u32,
    pub payload_id: [u8; 20],
    pub payload_size: i64,
    /// For payloads not in the trailer: absolute offset of the compressed buffer.
    pub offset: Option<i64>,
}

pub(crate) const EBD_STORED_IN_TRAILER: u32 = 1 << 9;
const EBD_IS_VIRTUALIZED: u32 = 1 << 0;
const EBD_REFERENCES_LEGACY_FILE: u32 = 1 << 2;

pub(crate) fn read_editor_bulk_data(r: &mut Reader<'_>) -> Result<EditorBulkData> {
    let flags = r.u32()?;
    r.guid()?; // bulk data id
    let mut payload_id = [0u8; 20];
    payload_id.copy_from_slice(r.take(20)?);
    let payload_size = r.i64()?;
    let offset = if flags & EBD_STORED_IN_TRAILER == 0 { Some(r.i64()?) } else { None };
    ensure!(payload_size >= 0 && payload_size <= MAX_PAYLOAD as i64, "implausible editor bulk data size");
    Ok(EditorBulkData { flags, payload_id, payload_size, offset })
}

impl EditorBulkData {
    /// The payload bytes, from the trailer or an inline compressed buffer.
    pub fn load(&self, data: &[u8], trailer: &[TrailerEntry]) -> Result<Vec<u8>> {
        if self.payload_size == 0 {
            return Ok(Vec::new());
        }
        if self.flags & (EBD_IS_VIRTUALIZED | EBD_REFERENCES_LEGACY_FILE) != 0
            && self.flags & EBD_STORED_IN_TRAILER == 0
        {
            bail!(UndecodableError(Undecodable::NotInFile));
        }
        if let Some(off) = self.offset {
            ensure!(off >= 0, "payload has no offset");
            return decode_compressed_buffer(data, usize::try_from(off)?);
        }
        let entry = trailer.iter().find(|e| e.id == self.payload_id).context("payload missing from the trailer")?;
        decode_entry(data, entry)
    }
}

// UE4 `FByteBulkData` flags.
const BULKDATA_PAYLOAD_AT_END_OF_FILE: u32 = 0x0001;
const BULKDATA_SERIALIZE_COMPRESSED_ZLIB: u32 = 0x0002;
const BULKDATA_UNUSED: u32 = 0x0020;
const BULKDATA_PAYLOAD_IN_SEPARATE_FILE: u32 = 0x0100;
const BULKDATA_SIZE_64BIT: u32 = 0x2000;
const BULKDATA_NO_OFFSET_FIXUP: u32 = 0x0001_0000;

/// A UE4 `FByteBulkData` at the reader's position: header, then the payload inline or elsewhere in the file.
pub(crate) fn read_byte_bulk_data<'a>(r: &mut Reader<'a>, data: &'a [u8], pkg: &Package) -> Result<&'a [u8]> {
    let flags = r.u32()?;
    let (count, size) = if flags & BULKDATA_SIZE_64BIT != 0 {
        (r.i64()?, r.i64()?)
    } else {
        (i64::from(r.i32()?), i64::from(r.i32()?))
    };
    let mut offset = r.i64()?;
    ensure!(flags & BULKDATA_UNUSED == 0 && count >= 0 && size >= 0, "bulk data is unused");
    ensure!(flags & BULKDATA_PAYLOAD_IN_SEPARATE_FILE == 0, "bulk data is in a separate .ubulk file");
    ensure!(flags & BULKDATA_SERIALIZE_COMPRESSED_ZLIB == 0, "zlib-compressed bulk data isn't supported yet");
    let size = usize::try_from(size)?;
    if flags & BULKDATA_PAYLOAD_AT_END_OF_FILE == 0 {
        return r.take(size);
    }
    if flags & BULKDATA_NO_OFFSET_FIXUP == 0 {
        offset += pkg.bulk_data_start;
    }
    let start = usize::try_from(offset)?;
    data.get(start..start + size).context("bulk data runs past the end of the file")
}
