use super::*;
use crate::links::extract::{FOLDER_ATLAS, atlas_refs, attr_refs};

#[test]
fn guesses_come_from_the_models_part_of_the_pack() {
    assert!(guessable("Kit/Models/Barrel.fbx", "Kit/Textures/Barrel_Albedo.png"));
    assert!(guessable("Kit/Models/Barrel.fbx", "Kit/Models/barrel.png"));
    assert!(guessable("Models/Barrel.fbx", "Elsewhere/Barrel.png"));
    assert!(!guessable("Kit/Models/Barrel.fbx", "Kit/Textures/Barrels.png"));
    assert!(!guessable("Archive/Space Kit/Models/stairs.fbx", "2D assets/Isometric Town/PNG/stairs_stone_01_0.png"));
    assert!(!guessable("Archive/Space Kit/Models/stairs.fbx", "Archive/Space Kit Extra/stairs.png"));
}

#[test]
fn joins() {
    assert_eq!(join_rel("Models/crate.gltf", "textures/a%20b.png").as_deref(), Some("Models/textures/a b.png"));
    assert_eq!(join_rel("Models/sub/x.obj", "../t.png").as_deref(), Some("Models/t.png"));
    assert_eq!(join_rel("x.obj", "../../t.png"), None);
    assert_eq!(join_rel("Models/x.fbx", "C:\\art\\Rock.png").as_deref(), Some("Models/C:/art/Rock.png"));
}

#[test]
fn reads_tile_font_and_atlas_references() {
    let tmx = r#"<map><tileset firstgid="1" source="terrain.tsx"/><imagelayer><image source="bg/sky.png"/></imagelayer></map>"#;
    let refs: Vec<String> = attr_refs(tmx, "source=").into_iter().map(|r| r.target).collect();
    assert_eq!(refs, ["terrain.tsx", "bg/sky.png"]);
    let fnt = "info face=\"Pixel\" size=16\npage id=0 file=\"pixel_0.png\"\n";
    assert_eq!(attr_refs(fnt, "file=")[0].target, "pixel_0.png");
    assert_eq!(attr_refs(fnt, "file=")[0].rel, LinkRel::Texture);
    let atlas = "\nhero.png\nsize: 512,512\nformat: RGBA8888\nidle\n  xy: 0, 0\n";
    assert_eq!(atlas_refs(atlas)[0].target, "hero.png");
    let tres = r#"[ext_resource type="Texture2D" uid="uid://abc" path="res://art/rock.png" id="1"]"#;
    assert_eq!(attr_refs(tres, "path=")[0].target, "res://art/rock.png");
}

#[test]
fn a_spine_skeleton_names_its_atlas() {
    let dir = tempfile::tempdir().unwrap();
    let json = dir.path().join("hero-pro.json");
    std::fs::write(&json, crate::spine::tests::skeleton().to_string()).unwrap();
    let refs: Vec<String> = extract_refs(&json, "json").into_iter().map(|r| r.target).collect();
    assert_eq!(refs, [FOLDER_ATLAS, "hero-pro.atlas", "hero-pro.atlas.txt", "hero.atlas", "hero.atlas.txt"]);
}

#[test]
fn a_glb_names_the_files_beside_it() {
    let mut json = serde_json::to_vec(&serde_json::json!({
        "asset": { "version": "2.0" },
        "buffers": [{ "byteLength": 4 }],
        "images": [{ "uri": "textures/wood.png" }, { "uri": "data:image/png;base64,AAAA" }],
    }))
    .unwrap();
    json.resize(json.len().next_multiple_of(4), b' ');
    let mut glb = b"glTF".to_vec();
    glb.extend(2u32.to_le_bytes());
    glb.extend((12 + 8 + json.len() as u32).to_le_bytes());
    glb.extend((json.len() as u32).to_le_bytes());
    glb.extend(b"JSON");
    glb.extend(json);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("crate.glb");
    std::fs::write(&path, glb).unwrap();
    let refs: Vec<(String, LinkRel)> = extract_refs(&path, "glb").into_iter().map(|r| (r.target, r.rel)).collect();
    assert_eq!(refs, [("textures/wood.png".to_owned(), LinkRel::Texture)]);
}

#[test]
fn reads_unity_guid_references() {
    let dir = tempfile::tempdir().unwrap();
    let mat = dir.path().join("M.mat");
    std::fs::write(
        &mat,
        "%YAML 1.1\nMaterial:\n  m_Shader: {fileID: 46, guid: 0000000000000000f000000000000000, type: 0}\n  \
             m_Texture: {fileID: 2800000, guid: 7E1A2E907C7A4339BE186C6B102FDE95, type: 3}\n",
    )
    .unwrap();
    let refs = extract_refs(&mat, "mat");
    assert_eq!(refs.len(), 1, "built-in shader GUIDs are skipped");
    assert_eq!(refs[0].target, "unity-guid:7e1a2e907c7a4339be186c6b102fde95");
}

#[test]
fn resolves_unreal_package_paths() {
    let assets = vec![
        PackAsset {
            id: "a".into(),
            path: "Forest/Content/Forest/Meshes/SM_Rock.uasset".into(),
            kind: AssetKind::Model,
            origin: None,
        },
        PackAsset {
            id: "b".into(),
            path: "Forest/Content/Forest/Textures/T_Rock_D.uasset".into(),
            kind: AssetKind::Texture,
            origin: None,
        },
    ];
    let rel = hoard_unreal::content_relative("/Game/Forest/Textures/T_Rock_D").unwrap();
    assert_eq!(resolve_unreal(&assets, rel), Some(1));
    // After conversion the texture is a PNG that remembers its package.
    let mut converted = assets;
    converted[1].path = "Forest/Content/Forest/Textures/T_Rock_D.png".into();
    converted[1].origin = Some("Forest/Content/Forest/Textures/T_Rock_D.uasset".into());
    assert_eq!(resolve_unreal(&converted, rel), Some(1));
    // Uploaded without the Content/Forest prefix.
    converted[1].origin = Some("Textures/T_Rock_D.uasset".into());
    assert_eq!(resolve_unreal(&converted, rel), Some(1));
    assert_eq!(resolve_unreal(&converted, "Forest/Textures/T_Missing"), None);
    assert!(is_package_path("/Game/Forest/Textures/T_Rock_D"));
    assert!(!is_package_path("/textures/rock.png"));
}

#[test]
fn resolves_by_name_fallback() {
    let assets = vec![
        PackAsset { id: "a".into(), path: "Models/barrel.fbx".into(), kind: AssetKind::Model, origin: None },
        PackAsset {
            id: "b".into(),
            path: "Models/Textures/Barrel_Albedo.png".into(),
            kind: AssetKind::Texture,
            origin: None,
        },
        PackAsset { id: "c".into(), path: "Other/Barrel_Albedo.png".into(), kind: AssetKind::Texture, origin: None },
    ];
    let idx = PathIndex::build(&assets);
    assert_eq!(idx.resolve(&assets, "Models/barrel.fbx", "C:\\Users\\art\\barrel_albedo.PNG"), Some(1));
    // The source it was painted in isn't in the pack, but its export is.
    assert_eq!(idx.resolve(&assets, "Models/barrel.fbx", "D:/paint/Barrel_Albedo.psd"), Some(1));
    assert_eq!(idx.resolve(&assets, "Models/barrel.fbx", "D:/paint/Barrel_Albedo.fbx"), None);
}
