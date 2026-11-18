//! DragonBones skeletons (`…_ske.json`): 2D characters made of images (and meshes) hung on bones, with animations.
//!
//! The server reads their facts (armatures, animations, size, frame rate) and links each skeleton to the texture atlas
//! it's drawn from (`…_tex.json` and its image, which are a sheet of their own). The browser plays them (see
//! `web/src/dragonbones.ts`) and draws their thumbnails.

use serde_json::{Map, Value, json};

/// Names listed in the facts.
const MAX_NAMES: usize = 64;

/// What the asset's meta keeps of a skeleton, or `None` when the document isn't one: `{"version": "5.5",
/// "frameRate": 24, "name": …, "armature": [{"name", "aabb", "bone", "slot", "skin", "animation", …}]}`.
pub(crate) fn facts(doc: &Value) -> Option<Map<String, Value>> {
    let armatures = doc.get("armature")?.as_array().filter(|a| !a.is_empty())?;
    let version = doc.get("version").and_then(Value::as_str)?;
    let main = &armatures[0];
    main.get("bone").or_else(|| main.get("slot"))?.as_array()?;
    let list = |v: &Value, k: &str| -> Vec<Value> { v.get(k).and_then(Value::as_array).cloned().unwrap_or_default() };
    let name = |v: &Value| v.get("name").and_then(Value::as_str).unwrap_or_default().to_owned();
    let fps = main
        .get("frameRate")
        .or_else(|| doc.get("frameRate"))
        .and_then(Value::as_f64)
        .filter(|f| *f > 0.0)
        .unwrap_or(24.0);
    let animations = list(main, "animation");
    let mut m = Map::new();
    m.insert("dragonbones".into(), json!(version));
    m.insert("armature".into(), json!(name(main)));
    if armatures.len() > 1 {
        m.insert("armatures".into(), json!(armatures.iter().take(MAX_NAMES).map(name).collect::<Vec<_>>()));
    }
    m.insert("bones".into(), json!(list(main, "bone").len()));
    m.insert("slots".into(), json!(list(main, "slot").len()));
    let meshes = list(main, "skin")
        .iter()
        .flat_map(|s| list(s, "slot"))
        .flat_map(|s| list(&s, "display"))
        .filter(|d| d.get("type").and_then(Value::as_str) == Some("mesh"))
        .count();
    if meshes > 0 {
        m.insert("meshes".into(), json!(meshes));
    }
    m.insert("fps".into(), json!((fps * 100.0).round() / 100.0));
    if !animations.is_empty() {
        m.insert("animations".into(), json!(animations.iter().take(MAX_NAMES).map(name).collect::<Vec<_>>()));
        // The animation it starts with: its default action, or the first.
        let first = main
            .get("defaultActions")
            .and_then(Value::as_array)
            .and_then(|a| a.iter().find_map(|a| a.get("gotoAndPlay").and_then(Value::as_str)))
            .and_then(|n| animations.iter().find(|a| name(a) == n))
            .unwrap_or(&animations[0]);
        if let Some(frames) = first.get("duration").and_then(Value::as_f64).filter(|d| *d > 0.0) {
            m.insert("duration_secs".into(), json!((frames / fps * 1000.0).round() / 1000.0));
        }
    }
    if let Some(aabb) = main.get("aabb") {
        let size = |k: &str| aabb.get(k).and_then(Value::as_f64).filter(|v| *v > 0.0);
        if let (Some(w), Some(h)) = (size("width"), size("height")) {
            m.insert("width".into(), json!(w.round() as u64));
            m.insert("height".into(), json!(h.round() as u64));
        }
    }
    Some(m)
}

/// The atlas a skeleton is drawn from, by DragonBones' naming: `hero_ske.json` → `hero_tex.json` and `hero_tex.png`.
pub(crate) fn atlas_files(file_name: &str) -> Option<[String; 2]> {
    let lower = file_name.to_lowercase();
    let stem = &file_name[..lower.strip_suffix("_ske.json")?.len()];
    Some([format!("{stem}_tex.json"), format!("{stem}_tex.png")])
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn skeleton() -> Value {
        json!({"frameRate": 24, "name": "hero", "version": "5.5", "compatibleVersion": "5.5", "armature": [{
            "type": "Armature", "frameRate": 30, "name": "hero",
            "aabb": {"x": -50, "y": -100, "width": 100.4, "height": 120},
            "bone": [{"name": "root"}, {"name": "arm", "parent": "root", "transform": {"x": 10, "skX": 30, "skY": 30}}],
            "slot": [{"name": "body", "parent": "root"}, {"name": "arm", "parent": "arm"}],
            "skin": [{"slot": [
                {"name": "body", "display": [{"name": "body", "transform": {"y": -50}}]},
                {"name": "arm", "display": [{"type": "mesh", "name": "arm", "width": 20, "height": 40,
                    "vertices": [-10, -20, 10, -20, 10, 20, -10, 20], "uvs": [0, 0, 1, 0, 1, 1, 0, 1],
                    "triangles": [0, 1, 2, 0, 2, 3]}]}
            ]}],
            "animation": [
                {"name": "idle", "duration": 0},
                {"name": "wave", "duration": 45, "bone": [{"name": "arm", "rotateFrame": [
                    {"duration": 15, "tweenEasing": 0, "rotate": 0},
                    {"duration": 30, "tweenEasing": 0, "rotate": 90},
                    {"duration": 0, "rotate": 0}]}]}
            ],
            "defaultActions": [{"gotoAndPlay": "wave"}]
        }, {"type": "Armature", "name": "sword", "bone": [{"name": "root"}]}]})
    }

    #[test]
    fn facts_of_a_skeleton() {
        let m = facts(&skeleton()).unwrap();
        assert_eq!(m["dragonbones"], "5.5");
        assert_eq!(m["armature"], "hero");
        assert_eq!(m["armatures"], json!(["hero", "sword"]));
        assert_eq!(m["bones"], 2);
        assert_eq!(m["slots"], 2);
        assert_eq!(m["meshes"], 1);
        assert_eq!(m["fps"], 30.0);
        assert_eq!(m["animations"], json!(["idle", "wave"]));
        assert_eq!(m["duration_secs"], 1.5);
        assert_eq!((m["width"].as_u64(), m["height"].as_u64()), (Some(100), Some(120)));
        // A texture atlas, or a Lottie animation, isn't a skeleton.
        assert!(facts(&json!({"imagePath": "hero_tex.png", "SubTexture": []})).is_none());
        assert!(facts(&json!({"v": "5.7", "layers": []})).is_none());
    }

    #[test]
    fn the_atlas_beside_it() {
        assert_eq!(atlas_files("Hero_SKE.json"), Some(["Hero_tex.json".into(), "Hero_tex.png".into()]));
        assert_eq!(atlas_files("hero.json"), None);
    }
}
