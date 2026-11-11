//! Unreal Engine editor packages (`.uasset`, `.umap`), read and converted without Unreal.
//!
//! - [`package`]: the package header: summary, names, imports, exports, thumbnail, asset registry tags.
//! - [`props`]: tagged properties of an export (what the Details panel shows), as a value tree.
//! - [`bulk`]: bulk data: UE5 package trailers and compressed buffers, UE4 byte bulk data.
//! - [`convert`]: turning assets into common files: sounds to WAV, textures to PNG/EXR, materials to JSON.
//!
//! Everything here is plain, safe Rust with no engine libraries. UE5 stores raw source data (uncompressed texture
//! pixels, mesh descriptions) Oodle-compressed by default, which only Unreal can decode; those packages are
//! reported as needing Unreal Editor. Sources that were already compressed when imported (PNG, JPEG, and the WAV
//! inside a sound) don't shrink, so Unreal stores them as-is and they convert here.

pub mod bulk;
pub mod convert;
pub mod package;
pub mod props;
mod reader;
#[cfg(test)]
mod testpkg;

pub use package::{
    Embedded, Export, Import, MAX_CARVE, Package, carve_source, class_tag, content_relative, kind_of, parse, read,
};
