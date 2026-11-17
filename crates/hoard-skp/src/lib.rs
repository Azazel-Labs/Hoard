//! Reads SketchUp `.skp` files and converts them to binary glTF, so Hoard can preview SketchUp models without the
//! SketchUp SDK.
//!
//! Ported from openskp (<https://github.com/iamahsanmehmood/openskp>, MIT, see `LICENSE-openskp`): its C++ read
//! path, face triangulation and glTF building, without its writer or other exporters.
//!
//! - Files from SketchUp 3 to 2020 are an MFC object stream ([`legacy`]); 2021 and later are a ZIP holding
//!   tagged records ([`vff`]).
//! - Component instances become glTF nodes with their transforms; each definition's faces become one mesh per
//!   inherited material, shared by every placement.
//! - Hidden entities and entities on hidden tags (layers) are left out.

mod glb;
mod legacy;
mod model;
mod reader;
mod triangulate;
mod vff;
mod zip;

use anyhow::{Result, bail};

/// Every SketchUp file opens with a string record (`FF FE FF`) holding the 14 characters "SketchUp Model".
const MAGIC: [u8; 4] = [0xff, 0xfe, 0xff, 0x0e];
/// The last major version written as an MFC object stream; 2021 (21) moved to the ZIP container.
const LAST_LEGACY: u32 = 20;

/// Whether `bytes` look like a SketchUp file, from its header alone.
pub fn is_skp(bytes: &[u8]) -> bool {
    bytes.starts_with(&MAGIC)
}

/// Converts a SketchUp file to a GLB file, in metres with Y up.
///
/// # Errors
/// When the bytes are not a SketchUp file, come from a version this reader doesn't know, are damaged, or hold no
/// visible faces.
pub fn to_glb(bytes: &[u8]) -> Result<Vec<u8>> {
    if !is_skp(bytes) {
        bail!("not a SketchUp file");
    }
    let Some(major) = major_version(bytes) else { bail!("SketchUp file has no version in its header") };
    let model = if major <= LAST_LEGACY {
        if major < 3 {
            bail!("SketchUp {major} files are not supported (SketchUp 3 is the oldest this reader knows)");
        }
        legacy::parse(bytes, major)
            .map_err(|e| e.context(format!("reading a SketchUp {} file", version_name(major))))?
    } else {
        vff::parse(bytes).map_err(|e| e.context(format!("reading a SketchUp {} file", version_name(major))))?
    };
    glb::write(&model)
}

/// SketchUp's marketing name for a major version: 3 to 8 kept the number, 13 to 2020 became years.
fn version_name(major: u32) -> String {
    if major >= 13 { (2000 + major).to_string() } else { major.to_string() }
}

/// The major version from the header's `{major.minor.build}` string, which follows "SketchUp Model" in UTF-16.
fn major_version(bytes: &[u8]) -> Option<u32> {
    let ascii: Vec<u8> = bytes.iter().take(96).copied().filter(|&b| b != 0).collect();
    let open = ascii.iter().position(|&b| b == b'{')?;
    let digits: Vec<u8> = ascii[open + 1..].iter().copied().take_while(u8::is_ascii_digit).collect();
    if digits.is_empty() || ascii.get(open + 1 + digits.len()) != Some(&b'.') {
        return None;
    }
    std::str::from_utf8(&digits).ok()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(version: &str) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend("SketchUp Model".encode_utf16().flat_map(u16::to_le_bytes));
        out.extend([0xff, 0xfe, 0xff, 12]);
        out.extend(version.encode_utf16().flat_map(u16::to_le_bytes));
        out
    }

    #[test]
    fn reads_the_header_version() {
        assert!(is_skp(&header("{15.0.9350}")));
        assert!(!is_skp(b"PK\x03\x04"));
        assert_eq!(major_version(&header("{15.0.9350}")), Some(15));
        assert_eq!(major_version(&header("{23.1.340}")), Some(23));
        assert_eq!(major_version(&header("{x}")), None);
    }

    #[test]
    fn rejects_what_it_cannot_read() {
        assert!(to_glb(b"hello").is_err());
        assert!(to_glb(&header("{2.0.1}")).is_err());
        assert!(to_glb(&header("{15.0.9350}")).is_err());
        assert!(to_glb(&header("{23.0.1}")).is_err());
        assert!(to_glb(&MAGIC).is_err());
    }

    #[test]
    fn truncated_files_fail_cleanly() {
        let data = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/legacy_v5_textured.skp"));
        let Ok(data) = data else { return };
        for len in (0..data.len()).step_by(7) {
            let _ = to_glb(&data[..len]);
        }
    }
}
