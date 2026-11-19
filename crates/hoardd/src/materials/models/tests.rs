use super::*;
use crate::materials::models::linking::{link_unity_slots, object_key};

#[test]
fn lods_are_one_object() {
    assert_eq!(object_key("Rocks/Rock_LOD0.fbx"), object_key("Rocks/Rock_LOD3.fbx"));
    assert_ne!(object_key("Rocks/Rock_LOD0.fbx"), object_key("Rocks/Cliff_LOD0.fbx"));
    assert_ne!(object_key("a/Rock.fbx"), object_key("b/Rock.fbx"));
    // Only a trailing LOD number goes.
    assert_ne!(object_key("Rock_LOD0_Base.fbx"), object_key("Rock_LOD0_Top.fbx"));
}

#[test]
fn a_material_one_object_wears_is_part_of_it() {
    let c = Connection::open_in_memory().unwrap();
    c.execute_batch(
        r#"CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, path TEXT, ext TEXT, kind TEXT, meta TEXT);
               CREATE TABLE asset_links (from_id TEXT, to_id TEXT, rel TEXT);
               CREATE TABLE material_maps (material_id TEXT, texture_id TEXT);
               INSERT INTO assets VALUES
                 ('rockmat', 'p', 'Rock.material', 'material', 'material', '{"material_maps":1}'),
                 ('sharedmat', 'p', 'Moss.material', 'material', 'material', '{"material_maps":1}'),
                 ('lonemat', 'p', 'Sand.material', 'material', 'material', '{"material_maps":1,"owned_by":"gone"}'),
                 ('rock_tex', 'p', 'Rock_BaseColor.png', 'png', 'texture', '{}'),
                 ('lod1', 'p', 'Rock_LOD1.fbx', 'fbx', 'model', '{}'),
                 ('lod0', 'p', 'Rock_LOD0.fbx', 'fbx', 'model', '{}'),
                 ('cliff', 'p', 'Cliff.fbx', 'fbx', 'model', '{}');
               INSERT INTO material_maps VALUES ('rockmat', 'rock_tex');
               INSERT INTO asset_links VALUES
                 ('lod0', 'rock_tex', 'texture'), ('lod1', 'rockmat', 'material'),
                 ('lod0', 'sharedmat', 'material'), ('cliff', 'sharedmat', 'material');"#,
    )
    .unwrap();
    mark_owned(&c, "p").unwrap();
    let owner = |id: &str| -> Option<String> {
        c.query_row("SELECT json_extract(meta, '$.owned_by') FROM assets WHERE id = ?1", [id], |r| r.get(0)).unwrap()
    };
    assert_eq!(owner("rockmat").as_deref(), Some("lod0"), "the rock's LODs are one object; LOD0 owns it");
    assert_eq!(owner("sharedmat"), None, "two objects share it");
    assert_eq!(owner("lonemat"), None, "nothing uses it any more");
}

#[test]
fn unity_materials_go_in_the_slots_they_are_named_for() {
    let c = Connection::open_in_memory().unwrap();
    c.execute_batch(
        r#"CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, path TEXT, ext TEXT, kind TEXT, meta TEXT);
               CREATE TABLE asset_links (from_id TEXT, to_id TEXT, rel TEXT, PRIMARY KEY (from_id, to_id, rel));
               CREATE TABLE unity_guids (pack_id TEXT, guid TEXT, path TEXT);
               CREATE TABLE unity_remaps (pack_id TEXT, path TEXT, slot TEXT, guid TEXT);
               INSERT INTO assets VALUES
                 ('paint', 'p', 'Kit/Materials/Paint.material', 'material', 'material', '{"material_maps":1}'),
                 ('paint.mat', 'p', 'Kit/Materials/Paint.mat', 'mat', 'other', '{}'),
                 ('far', 'p', 'Other/Paint.material', 'material', 'material', '{"material_maps":1}'),
                 ('far.mat', 'p', 'Other/Paint.mat', 'mat', 'other', '{}'),
                 ('brick', 'p', 'Kit/Materials/Wall_Brick.material', 'material', 'material', '{"material_maps":1}'),
                 ('brick.mat', 'p', 'Kit/Materials/Wall_Brick.mat', 'mat', 'other', '{}'),
                 ('wood', 'p', 'Kit/Materials/Barrel-Wood.material', 'material', 'material', '{"material_maps":0}'),
                 ('wood.mat', 'p', 'Kit/Materials/Barrel-Wood.mat', 'mat', 'other', '{}'),
                 ('crate', 'p', 'Kit/Models/Crate.fbx', 'fbx', 'model',
                  '{"material_names":["Paint","lambert2","Steel"],"material_slots":{"Old":"gone"}}'),
                 ('barrel', 'p', 'Kit/Models/Barrel.fbx', 'fbx', 'model', '{"material_names":["Wood"]}'),
                 ('plain', 'p', 'Kit/Models/Plain.fbx', 'fbx', 'model', '{"material_names":[]}');
               INSERT INTO asset_links VALUES
                 ('paint', 'paint.mat', 'sidecar'), ('far', 'far.mat', 'sidecar'), ('brick', 'brick.mat', 'sidecar'),
                 ('wood', 'wood.mat', 'sidecar');
               INSERT INTO unity_guids VALUES ('p', 'gw', 'Kit/Materials/Wall_Brick.mat');
               INSERT INTO unity_remaps VALUES ('p', 'Kit/Models/Crate.fbx', 'lambert2', 'gw');
               ALTER TABLE assets ADD COLUMN thumb_hash TEXT;"#,
    )
    .unwrap();
    let slotted = link_unity_slots(&c, "p").unwrap();
    assert_eq!(slotted, HashSet::from(["crate".to_owned(), "barrel".to_owned()]));
    let slots = |id: &str| -> Option<String> {
        c.query_row("SELECT json_extract(meta, '$.material_slots') FROM assets WHERE id = ?1", [id], |r| r.get(0))
            .unwrap()
    };
    assert_eq!(
        slots("crate").as_deref(),
        Some(r#"{"Paint":"paint","lambert2":"brick"}"#),
        "the nearest Paint, the .meta's material for lambert2, and nothing for Steel"
    );
    assert_eq!(slots("barrel").as_deref(), Some(r#"{"Wood":"wood"}"#), "named after the model and the slot");
    assert_eq!(slots("plain"), None);
    let worn: Vec<String> = c
        .prepare("SELECT from_id || '>' || to_id FROM asset_links WHERE rel = 'material' ORDER BY 1")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    assert_eq!(worn, ["barrel>wood", "crate>brick", "crate>paint"]);
}

#[test]
fn a_kit_doesnt_wear_another_kits_material_of_the_same_name() {
    let c = Connection::open_in_memory().unwrap();
    c.execute_batch(
        r#"CREATE TABLE assets (id TEXT PRIMARY KEY, pack_id TEXT, path TEXT, ext TEXT, kind TEXT, meta TEXT);
               CREATE TABLE asset_links (from_id TEXT, to_id TEXT, rel TEXT, PRIMARY KEY (from_id, to_id, rel));
               CREATE TABLE unity_guids (pack_id TEXT, guid TEXT, path TEXT);
               CREATE TABLE unity_remaps (pack_id TEXT, path TEXT, slot TEXT, guid TEXT);
               INSERT INTO assets VALUES
                 ('heroes', 'p', 'Heroes/Unity/Assets/Materials/skin.material', 'material', 'material',
                  '{"material_maps":1}'),
                 ('heroes.mat', 'p', 'Heroes/Unity/Assets/Materials/skin.mat', 'mat', 'other', '{}'),
                 ('zombies', 'p', 'Zombies/Unity/Assets/Materials/skin.material', 'material', 'material',
                  '{"material_maps":1}'),
                 ('zombies.mat', 'p', 'Zombies/Unity/Assets/Materials/skin.mat', 'mat', 'other', '{}'),
                 ('zombie', 'p', 'Zombies/Unity/Assets/Models/zombie.fbx', 'fbx', 'model',
                  '{"material_names":["skin"]}'),
                 ('loose', 'p', 'Zombies/Models/zombie.fbx', 'fbx', 'model', '{"material_names":["skin"]}'),
                 ('bundle', 'p', 'Bundle/Models/character.fbx', 'fbx', 'model', '{"material_names":["skin"]}');
               INSERT INTO asset_links VALUES
                 ('heroes', 'heroes.mat', 'sidecar'), ('zombies', 'zombies.mat', 'sidecar');
               ALTER TABLE assets ADD COLUMN thumb_hash TEXT;
               UPDATE assets SET thumb_hash = 'h', meta = json_set(meta, '$.thumb_dressed', 'h',
                   '$.material_slots',
                   json(CASE id WHEN 'loose' THEN '{"skin":"zombies"}' ELSE '{"skin":"heroes"}' END))
                WHERE id IN ('zombie', 'loose', 'bundle');"#,
    )
    .unwrap();
    link_unity_slots(&c, "p").unwrap();
    let slots = |id: &str| -> Option<String> {
        c.query_row("SELECT json_extract(meta, '$.material_slots') FROM assets WHERE id = ?1", [id], |r| r.get(0))
            .unwrap()
    };
    assert_eq!(slots("zombie").as_deref(), Some(r#"{"skin":"zombies"}"#), "its own project's, not the first");
    assert_eq!(slots("loose").as_deref(), Some(r#"{"skin":"zombies"}"#), "within its part of the pack");
    assert_eq!(slots("bundle"), None, "no kit of its own has a skin");
    let redress: Vec<String> = c
        .prepare(
            "SELECT id FROM assets
                  WHERE json_extract(meta, '$.redress') AND meta ->> 'thumb_dressed' IS NULL ORDER BY id",
        )
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    assert_eq!(redress, ["bundle", "zombie"], "drawn again: their materials changed, the loose one's didn't");
}
