//! The sound tools: a sound described in words (bright or dark, rising or falling, its tempo and key), and sounds
//! found by a shape given that way.

use hoard_core::SearchQuery;
use serde_json::{Value, json};

use super::ToolError;
use super::calls::{str_arg, str_list};
use crate::{AppState, db, imitation};

/// What `shape`'s words mean, as `imitation` measures them.
const BRIGHTNESS: [(&str, f32); 3] = [("dark", 500.0), ("mid", 1500.0), ("bright", 4500.0)];
const TEXTURE: [(&str, f32); 3] = [("tonal", 0.12), ("mixed", 0.3), ("noisy", 0.6)];
const PITCH: [(&str, f32); 3] = [("falling", -1.5), ("steady", 0.0), ("rising", 1.5)];

/// The word in `table` nearest to `x` (by octaves with `log`).
fn nearest(table: &[(&'static str, f32)], x: f32, log: bool) -> &'static str {
    let at = |v: f32| if log { v.max(1.0).log2() } else { v };
    table.iter().min_by(|a, b| (at(a.1) - at(x)).abs().total_cmp(&(at(b.1) - at(x)).abs())).map_or("", |t| t.0)
}

/// `shape`'s parts, laid over what the word said.
fn apply_shape(shape: &mut imitation::Shape, given: &Value) -> Result<(), ToolError> {
    let word = |key: &str, table: &[(&str, f32)]| -> Result<Option<f32>, ToolError> {
        let Some(v) = given.get(key).filter(|v| !v.is_null()) else { return Ok(None) };
        let v = v.as_str().unwrap_or_default();
        table.iter().find(|t| t.0 == v).map(|t| Some(t.1)).ok_or_else(|| {
            let words: Vec<&str> = table.iter().map(|t| t.0).collect();
            ToolError::Failed(format!("`shape.{key}` is one of {}", words.join(", ")))
        })
    };
    if let Some(secs) = given.get("seconds").and_then(Value::as_f64) {
        shape.secs = Some((secs as f32).clamp(0.01, 600.0));
    }
    if let Some(hz) = word("brightness", &BRIGHTNESS)? {
        shape.brightness_hz = Some(hz);
    }
    if let Some(n) = word("texture", &TEXTURE)? {
        shape.noisiness = Some(n);
    }
    if let Some(o) = word("pitch", &PITCH)? {
        shape.sweep_octaves = Some(o);
    }
    if let Some(sharp) = word("attack", &[("sharp", 1.0), ("soft", 0.0)])? {
        shape.sharp = Some(sharp > 0.5);
    }
    if let Some(rings) = word("ending", &[("rings", 1.0), ("cuts_off", 0.0)])? {
        shape.rings = Some(rings > 0.5);
    }
    if let Some(loud) = given.get("loud").and_then(Value::as_bool) {
        shape.loud = loud;
    }
    Ok(())
}

/// A shape in `shape`'s own words.
fn shape_json(shape: &imitation::Shape) -> Value {
    let mut out = serde_json::Map::new();
    if let Some(secs) = shape.secs {
        out.insert("seconds".into(), json!((secs * 100.0).round() / 100.0));
    }
    if let Some(sharp) = shape.sharp {
        out.insert("attack".into(), json!(if sharp { "sharp" } else { "soft" }));
    }
    if let Some(rings) = shape.rings {
        out.insert("ending".into(), json!(if rings { "rings" } else { "cuts_off" }));
    }
    if let Some(hz) = shape.brightness_hz {
        out.insert("brightness".into(), json!(nearest(&BRIGHTNESS, hz, true)));
    }
    if let Some(n) = shape.noisiness {
        out.insert("texture".into(), json!(nearest(&TEXTURE, n, false)));
    }
    if let Some(o) = shape.sweep_octaves {
        out.insert("pitch".into(), json!(nearest(&PITCH, o, false)));
    }
    out.insert("loud".into(), json!(shape.loud));
    Value::Object(out)
}

/// What a sound's measured features say, in words; nothing if it hasn't been listened to.
/// A piece of music's tempo and key, in words: "120 BPM (or 60), 4 bars, in A minor (8A)".
fn music_words(m: &Value) -> Vec<String> {
    let num = |k: &str| m.get(k).and_then(Value::as_f64);
    let mut words = Vec::new();
    if let Some(bpm) = num("tempo_bpm") {
        words.push(format!("{bpm:.0} BPM"));
    } else if let Some(bpm) = num("measured_bpm") {
        let alt = num("measured_bpm_alt").map_or_else(String::new, |a| format!(" (or {a:.0})"));
        words.push(format!("about {bpm:.0} BPM{alt}"));
    }
    let key = m.get("key").or_else(|| m.get("measured_key")).and_then(Value::as_str);
    if let Some(k) = key.and_then(crate::rhythm::Key::parse) {
        words.push(format!("in {} ({})", k.name(), k.camelot()));
    }
    words
}

fn sound_words(m: &Value) -> Option<String> {
    let num = |k: &str| m.get(k).and_then(Value::as_f64);
    let music = music_words(m);
    let Some(brightness) = num("brightness_hz") else {
        return (!music.is_empty()).then(|| music.join(", "));
    };
    let mut words: Vec<String> = Vec::new();
    if let Some(secs) = num("duration_secs") {
        words.push(if secs >= 10.0 { format!("{secs:.0} s long") } else { format!("{secs:.2} s long") });
    }
    match m.get("envelope").and_then(Value::as_str) {
        Some("hit") => words.push("a hit".into()),
        Some("swell") => words.push("swells".into()),
        Some("sustained") => words.push("sustained".into()),
        _ => {}
    }
    match num("attack_ms") {
        Some(a) if a <= 15.0 => words.push("starts sharply".into()),
        Some(a) if a >= 60.0 => words.push("fades in".into()),
        _ => {}
    }
    match num("tail_ms") {
        Some(t) if t >= 150.0 => words.push("rings on".into()),
        Some(t) if t <= 80.0 => words.push("cuts off".into()),
        _ => {}
    }
    words.push(
        match brightness {
            b if b >= 4000.0 => "bright",
            b if b <= 800.0 => "dark",
            _ => "neither bright nor dark",
        }
        .into(),
    );
    match num("noisiness") {
        Some(n) if n >= 0.45 => words.push("noisy (hiss, air)".into()),
        Some(n) if n <= 0.15 => words.push("tonal".into()),
        Some(_) => words.push("part tone, part noise".into()),
        None => {}
    }
    match num("sweep_octaves") {
        Some(o) if o >= 1.0 => words.push(format!("pitch rises about {o:.1} octaves")),
        Some(o) if o <= -1.0 => words.push(format!("pitch falls about {:.1} octaves", -o)),
        Some(o) if o.abs() >= 0.3 => words.push(format!("pitch drifts {}", if o > 0.0 { "up" } else { "down" })),
        Some(_) => words.push("steady pitch".into()),
        None => {}
    }
    match num("rms_dbfs") {
        Some(r) if r >= -18.0 => words.push("loud".into()),
        Some(r) if r <= -35.0 => words.push("quiet".into()),
        _ => {}
    }
    if m.get("loops_cleanly").and_then(Value::as_bool) == Some(true) {
        words.push("loops cleanly".into());
    }
    words.extend(music);
    Some(words.join(", "))
}

/// One sound, for the sound tools: what was measured, in numbers and in words, and where to listen.
fn sound_summary(base: &str, a: &hoard_core::Asset) -> Value {
    let keys = ["duration_secs", "sample_rate", "channels", "rms_dbfs", "peak_dbfs", "tempo_bpm", "key"];
    let measured: serde_json::Map<String, Value> = keys
        .iter()
        .chain(crate::sounds::FEATURES.iter())
        .filter_map(|k| a.meta.get(*k).map(|v| ((*k).to_owned(), v.clone())))
        .collect();
    json!({
        "id": a.id,
        "name": a.name,
        "description": a.description.as_ref().or(a.ai_description.as_ref()).or(a.source_description.as_ref()),
        "pack": a.pack_name,
        "path": a.path,
        "tags": a.tags.iter().map(|t| t.tag.as_str()).filter(|t| !t.starts_with("format:") && !t.starts_with("kind:")).collect::<Vec<_>>(),
        "sounds": sound_words(&a.meta).unwrap_or_else(|| "not listened to yet".into()),
        "measured": measured,
        "urls": {
            "listen": format!("{base}/api/assets/{}/file", a.id),
            "download": format!("{base}/api/assets/{}/file?download=1", a.id),
            "page": format!("{base}/#asset={}", a.id),
        },
    })
}

/// `search_sounds`, `describe_sound` and `sounds_like`.
pub(super) async fn sounds_call(state: &AppState, name: &str, args: &Value, base: &str) -> Result<Value, ToolError> {
    let limit = Some(args.get("limit").and_then(Value::as_u64).unwrap_or(20).clamp(1, 100) as u32);
    let id = || str_arg(args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into()));
    let sounds = |page: &db::SearchPage| page.items.iter().map(|a| sound_summary(base, a)).collect::<Vec<_>>();
    match name {
        "search_sounds" => {
            let mut shape = str_arg(args, "imitation").as_deref().and_then(imitation::read).unwrap_or_default();
            if let Some(given) = args.get("shape").filter(|v| !v.is_null()) {
                apply_shape(&mut shape, given)?;
            }
            let q = SearchQuery {
                q: str_arg(args, "query"),
                kind: Some("audio".into()),
                tags: str_list(args, "tags").iter().filter_map(|t| hoard_core::normalize_tag(t)).collect(),
                pack: str_arg(args, "pack_id"),
                collection: str_arg(args, "collection"),
                bpm_min: args.get("bpm_min").and_then(Value::as_f64).map(|b| b as f32),
                bpm_max: args.get("bpm_max").and_then(Value::as_f64).map(|b| b as f32),
                key: str_arg(args, "key"),
                compatible_keys: args.get("compatible_keys").and_then(Value::as_bool).unwrap_or(false),
                limit,
                ..Default::default()
            };
            let music = q.bpm_min.is_some() || q.bpm_max.is_some() || q.key.is_some();
            if shape.is_empty() && q.q.is_none() && q.tags.is_empty() && !music {
                return Err(ToolError::Failed(
                    "give `imitation` (a word with letters), `shape`, `query`, a tempo or a key".into(),
                ));
            }
            let rank = if !shape.is_empty() {
                let scores = imitation::rank_shape(state, shape.clone()).await?;
                Some(db::LookRank { scores, exclude: None, blend: false, keys: db::Keys::Files })
            } else if let Some(text) = q.q.as_deref() {
                // Words alone: with sound search on, sounds that sound like them, beside names that have them.
                let scores = crate::clap::search_text(state, text).await.unwrap_or_default();
                (!scores.is_empty()).then_some(db::LookRank {
                    scores,
                    exclude: None,
                    blend: true,
                    keys: db::Keys::Files,
                })
            } else {
                None
            };
            let page = db::call(&state.pool, move |c| db::search_ranked(c, &q, false, rank.as_ref())).await?;
            let mut out = json!({ "total": page.total, "returned": page.items.len(), "sounds": sounds(&page) });
            if !shape.is_empty() {
                out["reading"] = shape_json(&shape);
            }
            Ok(out)
        }
        "sounds_like" => {
            let q = SearchQuery {
                q: str_arg(args, "query"),
                kind: Some("audio".into()),
                similar: Some(id()?),
                limit,
                ..Default::default()
            };
            let page = crate::looks::search(state, q, false).await?;
            Ok(json!({ "total": page.total, "returned": page.items.len(), "sounds": sounds(&page) }))
        }
        "describe_sound" => {
            let id = id()?;
            let asset = db::call(&state.pool, move |c| db::get_asset(c, &id)).await?;
            let a = asset.ok_or_else(|| ToolError::Failed("no asset with that id".into()))?;
            if a.kind != hoard_core::AssetKind::Audio {
                return Err(ToolError::Failed(format!("that isn't a sound but {}", a.kind.as_str())));
            }
            Ok(sound_summary(base, &a))
        }
        _ => Err(ToolError::Unknown),
    }
}
