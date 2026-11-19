use super::*;
use crate::unitymat::binary::{COMMON_STRINGS, MATERIAL_CLASS};

const STANDARD: &str = "%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!21 &2100000
Material:
  serializedVersion: 6
  m_Name: Bricks
  m_Shader: {fileID: 46, guid: 0000000000000000f000000000000000, type: 0}
  m_ShaderKeywords: _EMISSION _NORMALMAP
  m_SavedProperties:
    serializedVersion: 3
    m_TexEnvs:
    - _BumpMap:
        m_Texture: {fileID: 2800000, guid: 1111111111111111aaaaaaaaaaaaaaaa, type: 3}
        m_Scale: {x: 1, y: 1}
        m_Offset: {x: 0, y: 0}
    - _DetailAlbedoMap:
        m_Texture: {fileID: 0}
        m_Scale: {x: 1, y: 1}
        m_Offset: {x: 0, y: 0}
    - _MainTex:
        m_Texture: {fileID: 2800000, guid: 2222222222222222BBBBBBBBBBBBBBBB, type: 3}
        m_Scale: {x: 4, y: 2}
        m_Offset: {x: 0.5, y: 0}
    m_Floats:
    - _BumpScale: 0.5
    - _Cutoff: 0.3
    - _Glossiness: 0.25
    - _Metallic: 1
    - _Mode: 1
    m_Colors:
    - _Color: {r: 1, g: 0.5, b: 0, a: 1}
    - _EmissionColor: {r: 1, g: 1, b: 1, a: 1}
--- !u!114 &1
MonoBehaviour:
  m_Name:
";

#[test]
fn a_text_material_gives_its_maps_and_settings() {
    let m = parse(STANDARD.as_bytes()).unwrap();
    assert_eq!(m.name, "Bricks");
    assert_eq!(
        m.maps(),
        vec![(Role::BaseColor, "2222222222222222bbbbbbbbbbbbbbbb"), (Role::Normal, "1111111111111111aaaaaaaaaaaaaaaa")]
    );
    let look = m.look(None);
    let [red, green, blue, alpha] = look.base_color.unwrap();
    assert!((red - 1.0).abs() < 1e-6 && (green - 0.214).abs() < 1e-3);
    assert!(blue.abs() < 1e-6 && (alpha - 1.0).abs() < 1e-6);
    assert_eq!(look.metallic, Some(1.0));
    assert_eq!(look.roughness, Some(0.75));
    assert_eq!(look.emissive, Some([1.0, 1.0, 1.0]));
    assert_eq!(look.alpha_mode.as_deref(), Some("mask"));
    assert_eq!(look.alpha_cutoff, Some(0.3));
    assert_eq!(look.normal_scale, Some(0.5));
    assert_eq!(look.uv_scale, Some([4.0, 2.0]));
    assert_eq!(look.uv_offset, Some([0.5, 0.0]));
}

#[test]
fn emission_needs_its_keyword_and_smoothness_can_live_in_the_metallic_map() {
    let text = STANDARD
            .replace("_EMISSION _NORMALMAP", "_NORMALMAP")
            .replace("    - _MainTex:", "    - _MetallicGlossMap:\n        m_Texture: {fileID: 2800000, guid: 3333333333333333cccccccccccccccc, type: 3}\n    - _MainTex:")
            .replace("    - _Metallic: 1\n", "    - _Metallic: 1\n    - _GlossMapScale: 0.5\n");
    let m = parse(text.as_bytes()).unwrap();
    assert_eq!(m.smoothness_in(), Some(SmoothnessIn { role: Role::Metalness, scale: 0.5 }));
    let look = m.look(Some(0.8));
    assert_eq!(look.emissive, None);
    // The metallic map says how metal it is.
    assert_eq!(look.metallic, None);
    assert!((look.roughness.unwrap() - 0.6).abs() < 1e-6);
}

/// A binary material as Unity 5.6 saves it: a type tree for `Material` with one texture, one float and one
/// color, the texture in a file listed by GUID.
fn binary_material() -> Vec<u8> {
    let mut strings = Vec::new();
    let mut s = |text: &str| {
        let at = strings.len() as u32;
        strings.extend_from_slice(text.as_bytes());
        strings.push(0);
        at
    };
    let common = |name: &str| {
        let mut at = 0;
        for part in COMMON_STRINGS.split('\0') {
            if part == name {
                return at | 0x8000_0000;
            }
            at += part.len() as u32 + 1;
        }
        panic!("{name} isn't common")
    };
    // (level, array, type, name, aligned)
    let tree: Vec<(u8, bool, u32, u32, bool)> = vec![
        (0, false, s("Material"), s("Base"), false),
        (1, false, common("string"), common("m_Name"), false),
        (2, true, common("Array"), common("Array"), true),
        (3, false, common("SInt32"), common("size"), false),
        (3, false, common("char"), common("data"), false),
        (1, false, s("UnityPropertySheet"), s("m_SavedProperties"), false),
        (2, false, common("map"), s("m_TexEnvs"), false),
        (3, true, common("Array"), common("Array"), true),
        (4, false, common("int"), common("size"), false),
        (4, false, common("pair"), common("data"), false),
        (5, false, common("FastPropertyName"), common("first"), false),
        (6, false, common("string"), s("name"), false),
        (7, true, common("Array"), common("Array"), true),
        (8, false, common("SInt32"), common("size"), false),
        (8, false, common("char"), common("data"), false),
        (5, false, s("UnityTexEnv"), common("second"), false),
        (6, false, common("PPtr<Texture>"), s("m_Texture"), false),
        (7, false, common("int"), s("m_FileID"), false),
        (7, false, common("SInt64"), s("m_PathID"), false),
        (6, false, common("Vector2f"), s("m_Scale"), false),
        (7, false, common("float"), s("x"), false),
        (7, false, common("float"), s("y"), false),
        (6, false, common("Vector2f"), s("m_Offset"), false),
        (7, false, common("float"), s("x"), false),
        (7, false, common("float"), s("y"), false),
        (2, false, common("map"), s("m_Floats"), false),
        (3, true, common("Array"), common("Array"), true),
        (4, false, common("int"), common("size"), false),
        (4, false, common("pair"), common("data"), false),
        (5, false, common("string"), common("first"), false),
        (6, true, common("Array"), common("Array"), true),
        (7, false, common("SInt32"), common("size"), false),
        (7, false, common("char"), common("data"), false),
        (5, false, common("float"), common("second"), false),
        (2, false, common("map"), s("m_Colors"), false),
        (3, true, common("Array"), common("Array"), true),
        (4, false, common("int"), common("size"), false),
        (4, false, common("pair"), common("data"), false),
        (5, false, common("string"), common("first"), false),
        (6, true, common("Array"), common("Array"), true),
        (7, false, common("SInt32"), common("size"), false),
        (7, false, common("char"), common("data"), false),
        (5, false, common("ColorRGBA"), common("second"), false),
        (6, false, common("float"), s("r"), false),
        (6, false, common("float"), s("g"), false),
        (6, false, common("float"), s("b"), false),
        (6, false, common("float"), s("a"), false),
    ];
    let string = |out: &mut Vec<u8>, text: &str| {
        out.extend((text.len() as i32).to_le_bytes());
        out.extend(text.as_bytes());
        while !out.len().is_multiple_of(4) {
            out.push(0);
        }
    };
    let mut data = Vec::new();
    string(&mut data, "Rust");
    data.extend(1i32.to_le_bytes());
    string(&mut data, "_MainTex");
    data.extend(2i32.to_le_bytes());
    data.extend(2_800_000i64.to_le_bytes());
    for v in [2.0f32, 3.0, 0.0, 0.0] {
        data.extend(v.to_le_bytes());
    }
    data.extend(1i32.to_le_bytes());
    string(&mut data, "_Glossiness");
    data.extend(0.4f32.to_le_bytes());
    data.extend(1i32.to_le_bytes());
    string(&mut data, "_Color");
    for v in [1.0f32, 1.0, 1.0, 0.5] {
        data.extend(v.to_le_bytes());
    }

    let mut meta = Vec::new();
    meta.extend(b"5.6.5p2\0");
    meta.extend(5i32.to_le_bytes());
    meta.push(1);
    meta.extend(1i32.to_le_bytes());
    meta.extend(MATERIAL_CLASS.to_le_bytes());
    meta.push(0);
    meta.extend((-1i16).to_le_bytes());
    meta.extend([0; 16]);
    meta.extend((tree.len() as i32).to_le_bytes());
    meta.extend((strings.len() as i32).to_le_bytes());
    for (level, array, ty, name, aligned) in &tree {
        meta.extend(1u16.to_le_bytes());
        meta.push(*level);
        meta.push(u8::from(*array));
        meta.extend(ty.to_le_bytes());
        meta.extend(name.to_le_bytes());
        meta.extend((-1i32).to_le_bytes());
        meta.extend(0i32.to_le_bytes());
        meta.extend((if *aligned { 0x4000i32 } else { 0 }).to_le_bytes());
    }
    meta.extend(&strings);
    meta.extend(1i32.to_le_bytes());
    while (20 + meta.len()) % 4 != 0 {
        meta.push(0);
    }
    meta.extend(2_100_000i64.to_le_bytes());
    meta.extend(0u32.to_le_bytes());
    meta.extend((data.len() as u32).to_le_bytes());
    meta.extend(0i32.to_le_bytes());
    meta.extend(0i32.to_le_bytes());
    meta.extend(2i32.to_le_bytes());
    for guid in [[0u8; 16], [0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 1, 2, 3, 4, 5, 6, 7, 8]] {
        meta.push(0);
        meta.extend(guid);
        meta.extend(0i32.to_le_bytes());
        meta.extend(b"x\0");
    }
    let data_offset = (20 + meta.len()).next_multiple_of(16);
    let mut file = Vec::new();
    file.extend((meta.len() as u32).to_be_bytes());
    file.extend(((data_offset + data.len()) as u32).to_be_bytes());
    file.extend(17u32.to_be_bytes());
    file.extend((data_offset as u32).to_be_bytes());
    file.extend([0, 0, 0, 0]);
    file.extend(&meta);
    file.resize(data_offset, 0);
    file.extend(&data);
    file
}

#[test]
fn a_binary_material_is_read_through_its_type_tree() {
    let m = parse(&binary_material()).unwrap();
    assert_eq!(m.name, "Rust");
    assert_eq!(m.maps(), vec![(Role::BaseColor, "0123456789abcdef1020304050607080")]);
    assert_eq!(m.textures["_MainTex"].1, [2.0, 3.0]);
    let look = m.look(None);
    assert!((look.roughness.unwrap() - 0.6).abs() < 1e-6);
    assert_eq!(look.base_color, Some([1.0, 1.0, 1.0, 0.5]));
}

#[test]
fn a_cut_short_binary_file_is_an_error_not_a_panic() {
    let file = binary_material();
    for n in [0, 10, 40, 200, file.len() - 3] {
        let _ = parse(&file[..n]);
    }
}
