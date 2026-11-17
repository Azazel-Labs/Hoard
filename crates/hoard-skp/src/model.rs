//! The format-neutral model both readers produce and the glTF writer consumes. Lengths are in inches, Z up, as
//! SketchUp stores them.

use std::collections::HashMap;

/// Material and layer ids are whatever the file uses to point at them (a slot number in legacy files, a record id
/// in VFF files).
pub(crate) type Id = u64;

#[derive(Debug, Default)]
pub(crate) struct Model {
    pub(crate) materials: HashMap<Id, Material>,
    pub(crate) hidden_layers: Vec<Id>,
    pub(crate) definitions: HashMap<Id, Definition>,
    pub(crate) root: Definition,
}

#[derive(Debug, Clone)]
pub(crate) struct Material {
    pub(crate) name: String,
    /// sRGB colour, 0-255.
    pub(crate) rgb: [u8; 3],
    /// 0 is fully transparent, 1 opaque.
    pub(crate) alpha: f64,
    pub(crate) texture: Option<Texture>,
}

#[derive(Debug, Clone)]
pub(crate) struct Texture {
    /// The PNG or JPEG file exactly as SketchUp stored it.
    pub(crate) bytes: Vec<u8>,
    /// How many inches one copy of the image covers across and up.
    pub(crate) tile: [f64; 2],
}

#[derive(Debug, Default)]
pub(crate) struct Definition {
    pub(crate) name: String,
    pub(crate) faces: Vec<Face>,
    pub(crate) instances: Vec<Instance>,
}

#[derive(Debug, Clone)]
pub(crate) struct Face {
    /// The outer loop first, then any holes.
    pub(crate) loops: Vec<Vec<[f64; 3]>>,
    pub(crate) normal: [f64; 3],
    pub(crate) material: Option<Id>,
    /// SketchUp's row-major texture-to-face-plane matrix when the texture was positioned by hand.
    pub(crate) uv_matrix: Option<[f64; 9]>,
    pub(crate) hidden: bool,
    pub(crate) layer: Option<Id>,
}

#[derive(Debug, Clone)]
pub(crate) struct Instance {
    pub(crate) definition: Id,
    pub(crate) name: String,
    /// SketchUp's 13-value transform: a row-major 3x3, the translation, then a uniform scale divisor.
    pub(crate) matrix: [f64; 13],
    pub(crate) material: Option<Id>,
    pub(crate) hidden: bool,
    pub(crate) layer: Option<Id>,
}
