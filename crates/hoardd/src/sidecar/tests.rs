use super::*;

/// Shaped like a Megascans download's `<id>.json`.
pub(crate) const STAND: &str = r#"{
        "id": "xeokah0",
        "semanticTags": {
            "name": "Historic Miscellaneous Stand Wood Old 05",
            "name_formats": { "title": "Historic Miscellaneous Stand Wood Old 05", "slug": "historic_miscellaneous_stand_wood_old_05" },
            "contains": ["table"],
            "color": ["brown"],
            "country": "Denmark",
            "region": "Europe",
            "asset_type": "3D asset",
            "3d_mesh": true,
            "minSize": "1 m",
            "maxSize": "2 m",
            "theme": ["historic", "medieval"]
        },
        "meta": [
            { "name": "Texel Density", "key": "texelDensity", "value": "4096 px/m" },
            { "name": "Length", "key": "length", "value": "118 cm" },
            { "name": "Width", "key": "width", "value": "54 cm" },
            { "name": "Height", "key": "height", "value": "81 cm" },
            { "name": "Tris", "key": "tris", "value": 12000 }
        ],
        "tags": ["wood", "old", "stand", "furniture", "rustic"]
    }"#;

#[test]
fn reads_a_megascans_file() {
    let s = read("xeokah0.json", STAND).unwrap();
    assert_eq!(s.title, "Historic Miscellaneous Stand Wood Old 05");
    assert_eq!(s.id.as_deref(), Some("xeokah0"));
    assert_eq!(s.asset_type.as_deref(), Some("3D asset"));
    assert_eq!(s.origin.as_deref(), Some("Denmark, Europe"));
    assert_eq!(s.texel_density.as_deref(), Some("4096 px/m"));
    assert_eq!(s.size.as_deref(), Some("1 m to 2 m"));
    for t in ["megascans", "wood", "table", "historic", "3d-asset", "color:brown"] {
        assert!(s.tags.contains(&t.to_owned()), "{t} in {:?}", s.tags);
    }
    assert!(s.all_tags().contains(&"material:wood".to_owned()));
    let d = s.describe(&["base color", "normal", "roughness"]);
    assert_eq!(
        d,
        "A 3D asset from Quixel Megascans, scanned in Denmark, Europe. Length 118 cm, width 54 cm, height 81 cm; \
             texel density 4096 px/m. Maps: base color, normal, roughness."
    );
    assert!(read("x.json", r#"{"name": "a package.json"}"#).is_none());
    assert!(read("x.json", "not json").is_none());
}

#[test]
fn reads_the_older_shape() {
    let s = read(
        "Rock/abc123.json",
        r#"{"id":"abc123","name":"Mossy Rock","meta":[{"key":"scanArea","value":"2x2 m"}],"categories":["3d","rock"],
               "tags":"mossy, nordic","environment":{"region":"Iceland"}}"#,
    )
    .unwrap();
    assert_eq!(s.title, "Mossy Rock");
    assert_eq!(s.size.as_deref(), Some("2x2 m"));
    assert_eq!(s.origin.as_deref(), Some("Iceland"));
    assert!(s.tags.contains(&"mossy".to_owned()) && s.tags.contains(&"rock".to_owned()));
    assert!(s.describe(&[]).starts_with("An asset from Quixel Megascans, scanned in Iceland. Size 2x2 m."));
}

/// A surface in the shape of Quixel's asset spec: maps under `components`, sizes in decimetres.
const SURFACE_SPEC: &str = r#"{
        "id": "sbkpa", "name": "Mossy Ground", "categories": ["surface_scans", "soil"],
        "meta": [
            { "key": "scanArea", "value": "2x2 m" }, { "key": "tileable", "value": true },
            { "key": "height", "value": "2cm" }
        ],
        "components": [
            { "type": "albedo", "colorSpace": "sRGB", "uris": [{ "physicalSize": "20x20", "resolutions": [
                { "resolution": "2048x2048", "formats": [
                    { "mimeType": "image/jpeg", "uri": "sbkpa_2K_Albedo.jpg" },
                    { "mimeType": "image/x-exr", "uri": "sbkpa_2K_Albedo.exr" }
                ] }
            ] }] },
            { "type": "displacement", "uris": [{ "resolutions": [{ "formats": [{ "uri": "sbkpa_2K_Displacement.exr" }] }] }] }
        ]
    }"#;

#[test]
fn reads_a_surfaces_maps_and_size() {
    let s = read("Mossy/sbkpa.json", SURFACE_SPEC).unwrap();
    assert_eq!(s.physical_size, Some((2.0, 2.0)));
    assert!((s.displacement.unwrap() - 0.02).abs() < 1e-6);
    assert_eq!(s.maps.len(), 3);
    assert_eq!(s.listed_map("Mossy/sbkpa_2K_Albedo.exr").map(|m| m.kind.as_str()), Some("albedo"));
    assert_eq!(s.listed_map("Mossy/sbkpa_2K_displacement.EXR").map(|m| m.kind.as_str()), Some("displacement"));
    assert_eq!(s.listed_map("Other/sbkpa_2K_Albedo.jpg"), None, "only files beside the JSON");

    // Today's `maps` list; a size without a scan area, in meters.
    let today = r#"{"id": "x1", "semanticTags": {"name": "Tiles", "asset_type": "surface"},
            "maps": [{"type": "normal", "uri": "x1_4K_Normal.jpg", "physicalSize": "0.91x0.91", "colorSpace": "Linear"}]}"#;
    let s = read("x1.json", today).unwrap();
    assert_eq!(s.physical_size, Some((0.91, 0.91)));
    assert_eq!(s.listed_map("x1_4K_Normal.jpg").map(|m| m.kind.as_str()), Some("normal"));

    // A 3D asset's measurements aren't a repeat.
    let s = read("xeokah0.json", STAND).unwrap();
    assert_eq!((s.physical_size, s.displacement), (None, None));
    assert_eq!(area_metres("200 x 100 cm"), Some((2.0, 1.0)));
    assert_eq!(area_metres("40x40"), Some((4.0, 4.0)));
    assert_eq!(area_metres("2x2 furlongs"), None);
}

#[test]
fn reads_levels_of_detail() {
    let json = r#"{"id": "pdEuS", "name": "Branch", "meta": [],
            "meshes": [
                {"type": "zbrush", "uris": [{"uri": "Branch_pdEuS.ztl"}]},
                {"type": "lod", "tris": 380, "uris": [{"uri": "Branch_pdEuS_LOD5.fbx"}]},
                {"type": "lod", "tris": 24000, "uris": [{"mimeType": "application/x-fbx", "uri": "Branch_pdEuS_LOD0.fbx"},
                                                       {"uri": "Branch_pdEuS_LOD0.obj"}]}
            ]}"#;
    let s = read("pdEuS.json", json).unwrap();
    assert_eq!(s.lods.iter().map(|l| (l.number, l.tris)).collect::<Vec<_>>(), [(0, 24000), (5, 380)]);
    assert_eq!(s.lod_tris("Branch/Branch_pdEuS_LOD0.OBJ"), Some(24000));
    assert_eq!(s.lod_tris("Branch/Branch_pdEuS_LOD0.glb"), Some(24000), "the same mesh in another format");
    assert_eq!(s.lod_tris("Branch/Branch_pdEuS_LOD3.fbx"), None);
    assert!(s.describe(&[]).ends_with("LOD0 to LOD5, 24,000 to 380 triangles."), "{}", s.describe(&[]));
    assert_eq!(thousands(999), "999");
    assert_eq!(thousands(1_234_567), "1,234,567");
}

#[test]
fn finds_the_lod_in_a_file_name() {
    assert_eq!(lod_of("Stand_xeokah0/Stand_xeokah0_LOD2.fbx"), Some(2));
    assert_eq!(lod_of("a/stand_lod0.obj"), Some(0));
    assert_eq!(lod_of("a/Stand_High.fbx"), None);
    assert_eq!(lod_of("a/lodge.obj"), None);
}

#[test]
fn knows_a_name_nobody_typed() {
    let s = read("xeokah0.json", STAND).unwrap();
    assert!(name_is_automatic("historic_miscellaneous_stand_wood_old_05_xeokah0_raw", &s, None));
    assert!(name_is_automatic("Historic_Miscellaneous_Stand_Wood_Old_05_Raw", &s, None));
    assert!(name_is_automatic("Pack 2026-11-12", &s, None));
    assert!(name_is_automatic("My Download", &s, Some("my_download")));
    assert!(!name_is_automatic("Market stall table", &s, Some("xeokah0")));
    assert!(!name_is_automatic("Historic stand", &s, None));
}
