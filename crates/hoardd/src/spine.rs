//! Spine skeletons (`.json`): 2D characters made of images and meshes hung on bones, with skins and animations.
//!
//! The server reads their facts (bones, slots, skins, animations, size, frame rate) and links each skeleton to the
//! libGDX texture atlas it's drawn from (`spineboy.atlas`, whose page images it names). The browser plays them with
//! Hoard's own player, written from Spine's published JSON format (see `web/src/spine.ts`), and draws their thumbnails.

use serde_json::{Map, Value, json};

/// Names listed in the facts.
const MAX_NAMES: usize = 64;

/// The animations a skeleton starts with, when it has one by these names; otherwise the first.
pub(crate) const STARTERS: &[&str] = &["idle", "walk", "run", "animation"];

/// What the asset's meta keeps of a skeleton, or `None` when the document isn't one: `{"skeleton": {"spine": "4.2.18",
/// "width", "height", "fps"}, "bones": […], "slots": […], "skins": […] (or {…} before 3.8), "animations": {…}}`.
pub(crate) fn facts(doc: &Value) -> Option<Map<String, Value>> {
    let skeleton = doc.get("skeleton")?.as_object()?;
    let bones = doc.get("bones")?.as_array().filter(|b| !b.is_empty())?;
    let version = skeleton.get("spine").and_then(Value::as_str).unwrap_or_default();
    let count = |k: &str| doc.get(k).and_then(Value::as_array).map_or(0, Vec::len);
    // Skins are a list of `{name, attachments}` since 3.8, a map of name → attachments before.
    let skins: Vec<(String, &Value)> = match doc.get("skins") {
        Some(Value::Array(a)) => a
            .iter()
            .map(|s| {
                (
                    s.get("name").and_then(Value::as_str).unwrap_or_default().to_owned(),
                    s.get("attachments").unwrap_or(&Value::Null),
                )
            })
            .collect(),
        Some(Value::Object(o)) => o.iter().map(|(k, v)| (k.clone(), v)).collect(),
        _ => Vec::new(),
    };
    let meshes = skins
        .iter()
        .filter_map(|(_, a)| a.as_object())
        .flat_map(|slots| slots.values().filter_map(Value::as_object))
        .flat_map(|atts| atts.values())
        .filter(|a| {
            matches!(
                a.get("type").and_then(Value::as_str),
                Some("mesh" | "linkedmesh" | "skinnedmesh" | "weightedmesh")
            )
        })
        .count();
    let fps = skeleton.get("fps").and_then(Value::as_f64).filter(|f| *f > 0.0).unwrap_or(30.0);
    let mut m = Map::new();
    m.insert("spine".into(), json!(version));
    m.insert("bones".into(), json!(bones.len()));
    m.insert("slots".into(), json!(count("slots")));
    if skins.len() > 1 {
        m.insert("skins".into(), json!(skins.iter().take(MAX_NAMES).map(|(n, _)| n).collect::<Vec<_>>()));
    }
    if meshes > 0 {
        m.insert("meshes".into(), json!(meshes));
    }
    m.insert("fps".into(), json!((fps * 100.0).round() / 100.0));
    if let Some(animations) = doc.get("animations").and_then(Value::as_object).filter(|a| !a.is_empty()) {
        let names: Vec<&String> = animations.keys().collect();
        m.insert("animations".into(), json!(names.iter().take(MAX_NAMES).collect::<Vec<_>>()));
        let first = STARTERS.iter().find_map(|s| animations.get(*s)).unwrap_or(&animations[names[0]]);
        let secs = last_time(first, 0);
        if secs > 0.0 {
            m.insert("duration_secs".into(), json!((secs * 1000.0).round() / 1000.0));
        }
    }
    let size = |k: &str| skeleton.get(k).and_then(Value::as_f64).filter(|v| *v > 0.0);
    if let (Some(w), Some(h)) = (size("width"), size("height")) {
        m.insert("width".into(), json!(w.round() as u64));
        m.insert("height".into(), json!(h.round() as u64));
    }
    Some(m)
}

/// The latest key in an animation: the biggest `time` anywhere in it.
fn last_time(v: &Value, depth: usize) -> f64 {
    if depth > 12 {
        return 0.0;
    }
    match v {
        Value::Object(o) => o
            .iter()
            .map(|(k, v)| if k == "time" { v.as_f64().unwrap_or(0.0) } else { last_time(v, depth + 1) })
            .fold(0.0, f64::max),
        Value::Array(a) => a.iter().map(|v| last_time(v, depth + 1)).fold(0.0, f64::max),
        _ => 0.0,
    }
}

/// The atlases a skeleton may be drawn from, by Spine's naming: `spineboy-pro.json` → `spineboy-pro.atlas` or
/// `spineboy.atlas` (the examples' `-pro` and `-ess` exports share one).
pub(crate) fn atlas_files(file_name: &str) -> Vec<String> {
    let Some(stem) = file_name
        .len()
        .checked_sub(5)
        .and_then(|n| file_name.get(..n))
        .filter(|_| file_name.to_lowercase().ends_with(".json"))
    else {
        return Vec::new();
    };
    let mut stems = vec![stem];
    for edition in ["-pro", "-ess"] {
        if let Some(s) =
            stem.len().checked_sub(4).and_then(|n| stem.get(..n)).filter(|_| stem.to_lowercase().ends_with(edition))
        {
            stems.push(s);
        }
    }
    stems.iter().flat_map(|s| [format!("{s}.atlas"), format!("{s}.atlas.txt")]).collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn skeleton() -> Value {
        json!({
            "skeleton": {"hash": "x", "spine": "4.2.18", "x": -40, "y": 0, "width": 80.4, "height": 120, "fps": 24},
            "bones": [{"name": "root"}, {"name": "arm", "parent": "root", "length": 30, "rotation": 45}],
            "slots": [{"name": "body", "bone": "root", "attachment": "body"}, {"name": "arm", "bone": "arm", "attachment": "arm"}],
            "skins": [
                {"name": "default", "attachments": {
                    "body": {"body": {"width": 80, "height": 120, "y": 60}},
                    "arm": {"arm": {"type": "mesh", "uvs": [0, 0, 1, 0, 1, 1], "triangles": [0, 1, 2],
                        "vertices": [0, 0, 30, 0, 30, 10], "hull": 3}}
                }},
                {"name": "red", "attachments": {}}
            ],
            "animations": {
                "wave": {"bones": {"arm": {"rotate": [{"value": 0}, {"time": 0.5, "value": 90}, {"time": 1.5, "value": 0}]}}},
                "idle": {"slots": {"body": {"rgba": [{"time": 0, "color": "ffffffff"}, {"time": 2, "color": "ffffff80"}]}}}
            }
        })
    }

    #[test]
    fn facts_of_a_skeleton() {
        let m = facts(&skeleton()).unwrap();
        assert_eq!(m["spine"], "4.2.18");
        assert_eq!(m["bones"], 2);
        assert_eq!(m["slots"], 2);
        assert_eq!(m["skins"], json!(["default", "red"]));
        assert_eq!(m["meshes"], 1);
        assert_eq!(m["fps"], 24.0);
        assert_eq!(m["animations"], json!(["idle", "wave"]));
        // It starts with "idle".
        assert_eq!(m["duration_secs"], 2.0);
        assert_eq!((m["width"].as_u64(), m["height"].as_u64()), (Some(80), Some(120)));
        // Before 3.8, skins were a map; one skin isn't listed.
        let mut old = skeleton();
        old["skins"] = json!({"default": {"body": {"body": {}}}});
        assert!(!facts(&old).unwrap().contains_key("skins"));
        // A DragonBones skeleton, or some other JSON, isn't a Spine one.
        assert!(facts(&json!({"armature": [{"bone": []}], "version": "5.5"})).is_none());
        assert!(facts(&json!({"skeleton": "x"})).is_none());
    }

    #[test]
    fn the_atlas_beside_it() {
        assert_eq!(
            atlas_files("spineboy-pro.json"),
            ["spineboy-pro.atlas", "spineboy-pro.atlas.txt", "spineboy.atlas", "spineboy.atlas.txt"]
        );
        assert_eq!(atlas_files("hero.JSON"), ["hero.atlas", "hero.atlas.txt"]);
        assert_eq!(atlas_files("hero.png"), Vec::<String>::new());
    }
}
