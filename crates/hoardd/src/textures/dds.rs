//! DDS textures (`.dds`): the header, the DX10 extension and the pixel layout either names, read as mip 0 of
//! the first layer or face.

use std::path::Path;

use anyhow::{Result, bail};
use serde_json::{Map, json};

use super::{Decoded, Layout, decode_layout};

pub(super) fn dds(path: &Path) -> Result<Decoded> {
    use ddsfile::{D3DFormat as D, DxgiFormat as X};
    let dds = ddsfile::Dds::read(std::io::BufReader::new(std::fs::File::open(path)?))
        .map_err(|e| anyhow::anyhow!("not a readable DDS file: {e}"))?;
    let (w, h) = (dds.get_width(), dds.get_height());
    let (layout, name) = if let Some(f) = dds.get_dxgi_format() {
        let l = match f {
            X::BC1_UNorm | X::BC1_UNorm_sRGB | X::BC1_Typeless => Layout::Bc1,
            X::BC2_UNorm | X::BC2_UNorm_sRGB | X::BC2_Typeless => Layout::Bc2,
            X::BC3_UNorm | X::BC3_UNorm_sRGB | X::BC3_Typeless => Layout::Bc3,
            X::BC4_UNorm | X::BC4_SNorm | X::BC4_Typeless => Layout::Bc4,
            X::BC5_UNorm | X::BC5_SNorm | X::BC5_Typeless => Layout::Bc5,
            X::BC6H_UF16 | X::BC6H_Typeless => Layout::Bc6 { signed: false },
            X::BC6H_SF16 => Layout::Bc6 { signed: true },
            X::BC7_UNorm | X::BC7_UNorm_sRGB | X::BC7_Typeless => Layout::Bc7,
            X::R8G8B8A8_UNorm | X::R8G8B8A8_UNorm_sRGB => Layout::Rgba8,
            X::B8G8R8A8_UNorm | X::B8G8R8A8_UNorm_sRGB => Layout::Bgra8,
            X::B8G8R8X8_UNorm | X::B8G8R8X8_UNorm_sRGB => Layout::Bgrx8,
            X::R8_UNorm => Layout::R8,
            X::R8G8_UNorm => Layout::Rg8,
            X::R16G16B16A16_Float => Layout::Rgba16F,
            X::R32G32B32A32_Float => Layout::Rgba32F,
            other => bail!("DDS format {other:?} isn't supported yet"),
        };
        (l, format!("{f:?}"))
    } else if let Some(f) = dds.get_d3d_format() {
        let l = match f {
            D::DXT1 => Layout::Bc1,
            D::DXT2 | D::DXT3 => Layout::Bc2,
            D::DXT4 | D::DXT5 => Layout::Bc3,
            D::A8R8G8B8 => Layout::Bgra8,
            D::X8R8G8B8 => Layout::Bgrx8,
            D::A8B8G8R8 | D::X8B8G8R8 => Layout::Rgba8,
            D::R8G8B8 => Layout::Bgr8,
            D::L8 => Layout::R8,
            D::A8L8 => Layout::La8,
            other => bail!("DDS format {other:?} isn't supported yet"),
        };
        (l, format!("{f:?}"))
    } else {
        bail!("DDS file with an unknown pixel format")
    };
    let data = dds.get_data(0).map_err(|e| anyhow::anyhow!("DDS data: {e}"))?;
    let image = decode_layout(data, w, h, layout)?;
    let mut facts = Map::new();
    facts.insert("gpu_format".into(), json!(name));
    facts.insert("mip_levels".into(), json!(dds.get_num_mipmap_levels().max(1)));
    let layers = dds.get_num_array_layers();
    if layers > 1 {
        facts.insert("array_layers".into(), json!(layers));
    }
    if dds.get_depth() > 1 {
        facts.insert("depth".into(), json!(dds.get_depth()));
    }
    Ok(Decoded { image, facts })
}

#[cfg(test)]
mod tests {

    use crate::textures::decode;
    use crate::textures::tests::{BC1_BLOCK, write};

    #[test]
    fn dds_bc1_and_raw() {
        let dir = tempfile::tempdir().unwrap();
        let mut dds = ddsfile::Dds::new_dxgi(ddsfile::NewDxgiParams {
            height: 4,
            width: 4,
            depth: None,
            format: ddsfile::DxgiFormat::BC1_UNorm,
            mipmap_levels: None,
            array_layers: None,
            caps2: None,
            is_cubemap: false,
            resource_dimension: ddsfile::D3D10ResourceDimension::Texture2D,
            alpha_mode: ddsfile::AlphaMode::Unknown,
        })
        .unwrap();
        dds.get_mut_data(0).unwrap().copy_from_slice(&BC1_BLOCK);
        let mut bytes = Vec::new();
        dds.write(&mut bytes).unwrap();
        let d = decode(&write(&dir, "t.dds", &bytes), "dds").unwrap();
        let img = d.image.to_rgba8();
        assert_eq!(img.get_pixel(0, 0).0, [0, 0, 255, 255], "first row is color1 (blue)");
        assert_eq!(img.get_pixel(2, 3).0, [255, 0, 0, 255], "the rest is color0 (red)");
        assert_eq!(d.facts["gpu_format"], "BC1_UNorm");

        let mut raw = ddsfile::Dds::new_dxgi(ddsfile::NewDxgiParams {
            height: 1,
            width: 2,
            depth: None,
            format: ddsfile::DxgiFormat::B8G8R8A8_UNorm,
            mipmap_levels: None,
            array_layers: None,
            caps2: None,
            is_cubemap: false,
            resource_dimension: ddsfile::D3D10ResourceDimension::Texture2D,
            alpha_mode: ddsfile::AlphaMode::Unknown,
        })
        .unwrap();
        raw.get_mut_data(0).unwrap().copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut bytes = Vec::new();
        raw.write(&mut bytes).unwrap();
        let img = decode(&write(&dir, "r.dds", &bytes), "dds").unwrap().image.to_rgba8();
        assert_eq!(img.get_pixel(1, 0).0, [7, 6, 5, 8]);
    }
}
