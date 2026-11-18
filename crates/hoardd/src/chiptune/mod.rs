//! Chiptunes and the tracker formats beyond the four `xmrs` reads by name: NES music (NSF, NSFe), logs of the writes
//! a game made to its sound chips (VGM, VGZ), and Unreal's music packages (UMX), which wrap a tracker module.
//! OpenMPT's own modules (MPTM) are IT files underneath, and play like one.

mod nsf;
mod sn76489;
mod vgm;

use anyhow::{Result, bail};

use crate::music::Rendered;

/// Formats played here.
pub(crate) const EXTS: &[&str] = &["nsf", "nsfe", "vgm", "vgz", "umx"];

pub(crate) fn render(data: &[u8], ext: &str, max_secs: f32) -> Result<Rendered> {
    match ext {
        "nsf" | "nsfe" => nsf::render(data, max_secs),
        "vgm" | "vgz" => vgm::render(data, max_secs),
        "umx" => {
            let mut r = crate::music::module(unreal_module(data)?, max_secs)?;
            r.facts.insert("container".into(), "Unreal package".into());
            Ok(r)
        }
        _ => bail!("no chiptune player for .{ext}"),
    }
}

/// The tracker module inside an Unreal package: the first place one of the formats' signatures sits where it should,
/// and from which a module loads.
fn unreal_module(data: &[u8]) -> Result<&[u8]> {
    if !data.starts_with(&[0xC1, 0x83, 0x2A, 0x9E]) {
        bail!("not an Unreal package");
    }
    // Each signature, and how far into a module it is.
    let signatures: [(&[u8], usize); 4] = [(b"IMPM", 0), (b"Extended Module: ", 0), (b"SCRM", 44), (b"M.K.", 1080)];
    for (sig, offset) in signatures {
        let mut from = 0;
        while let Some(i) = data[from..].windows(sig.len()).position(|w| w == sig) {
            let at = from + i;
            from = at + 1;
            let Some(start) = at.checked_sub(offset) else { continue };
            if xmrs::prelude::Module::load(&data[start..]).is_ok() {
                return Ok(&data[start..]);
            }
        }
    }
    bail!("no tracker module in the Unreal package")
}

#[cfg(test)]
pub(crate) mod tests {
    pub(crate) use super::nsf::tests::tiny_nsf;
    pub(crate) use super::vgm::tests::tiny_vgm;

    /// An Unreal package holding a tracker module among other data.
    pub(crate) fn tiny_umx() -> Vec<u8> {
        let mut umx = vec![0xC1, 0x83, 0x2A, 0x9E];
        umx.extend([0x45, 0, 0, 0]);
        umx.extend(b"names and imports M.K. too early to be a module".repeat(4));
        umx.extend(crate::music::tests::tiny_mod());
        umx.extend(b"trailing export table");
        umx
    }

    #[test]
    fn finds_the_module_in_an_unreal_package() {
        let r = super::render(&tiny_umx(), "umx", 30.0).unwrap();
        assert_eq!(r.facts["title"], "hoard test");
        assert_eq!(r.facts["container"], "Unreal package");
        assert!(super::render(b"\xC1\x83\x2A\x9Enothing", "umx", 1.0).is_err());
        assert!(super::render(b"not unreal", "umx", 1.0).is_err());
    }
}
