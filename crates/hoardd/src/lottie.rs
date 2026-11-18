//! Lottie animations (Bodymovin's JSON, and dotLottie's `.lottie` zip of them): vector animations, common for UI.
//!
//! The server reads their facts (size, frame rate, length, layers) and serves each as the JSON a browser plays: a
//! `.json` as it is (its images, if it names any, are found beside it in the pack by the browser), and a `.lottie`'s
//! first animation out of the zip with its images inlined. The browser draws their thumbnails (see
//! `previews::put_thumb`).

use std::io::Read as _;

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::Response;
use base64::Engine as _;
use rusqlite::OptionalExtension as _;
use serde_json::{Map, Value, json};

use crate::AppState;
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;

/// Bigger files aren't read.
pub(crate) const MAX_FILE: u64 = 32 << 20;

/// What the asset's meta keeps of a Lottie animation, or `None` when the document isn't one: `{"v": "5.7.4", "fr": 30,
/// "ip": 0, "op": 90, "w": 512, "h": 512, "layers": [...]}`.
pub(crate) fn facts(doc: &Value) -> Option<Map<String, Value>> {
    let num = |k: &str| doc.get(k).and_then(Value::as_f64).filter(|n| n.is_finite());
    doc.get("v")?.as_str()?;
    let layers = doc.get("layers")?.as_array()?;
    let (fr, ip, op, w, h) = (num("fr")?, num("ip")?, num("op")?, num("w")?, num("h")?);
    if fr <= 0.0 || op <= ip || w <= 0.0 || h <= 0.0 {
        return None;
    }
    let frames = op - ip;
    let mut m = Map::new();
    m.insert("lottie".into(), json!(doc.get("v")));
    m.insert("width".into(), json!(w.round() as u64));
    m.insert("height".into(), json!(h.round() as u64));
    m.insert("fps".into(), json!((fr * 100.0).round() / 100.0));
    m.insert("frames".into(), json!(frames.round() as u64));
    m.insert("duration_secs".into(), json!((frames / fr * 1000.0).round() / 1000.0));
    m.insert("layers".into(), json!(layers.len()));
    if let Some(name) = doc.get("nm").and_then(Value::as_str).map(str::trim).filter(|n| !n.is_empty()) {
        m.insert("lottie_name".into(), json!(name));
    }
    Some(m)
}

/// The images an animation names, inlined as data URLs: `find(folder, file)` gives each one's bytes. Images already
/// inlined, or not found, are left as they are.
pub(crate) fn inline(doc: &mut Value, mut find: impl FnMut(&str, &str) -> Option<Vec<u8>>) {
    let Some(assets) = doc.get_mut("assets").and_then(Value::as_array_mut) else { return };
    for a in assets {
        let Some(file) = a.get("p").and_then(Value::as_str) else { continue };
        if file.starts_with("data:") || a.get("e").and_then(Value::as_u64) == Some(1) {
            continue;
        }
        let folder = a.get("u").and_then(Value::as_str).unwrap_or_default();
        let Some(bytes) = find(folder, file) else { continue };
        let mime = mime_guess::from_path(file).first_or_octet_stream();
        let url = format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes));
        a["u"] = json!("");
        a["p"] = json!(url);
        a["e"] = json!(1);
    }
}

/// The first animation in a dotLottie file, its images inlined. Version 1 keeps animations in `animations/` and
/// images in `images/`; version 2 in `a/` and `i/`.
pub(crate) fn from_dotlottie(bytes: &[u8]) -> anyhow::Result<Value> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let mut read = |name: &str| -> Option<Vec<u8>> {
        let mut f = zip.by_name(name).ok()?;
        if f.size() > MAX_FILE {
            return None;
        }
        let mut out = Vec::new();
        f.read_to_end(&mut out).ok()?;
        Some(out)
    };
    let manifest: Value =
        serde_json::from_slice(&read("manifest.json").ok_or_else(|| anyhow::anyhow!("no manifest"))?)?;
    let first = manifest
        .get("animations")
        .and_then(Value::as_array)
        .and_then(|a| a.first())
        .and_then(|a| a.get("id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("the manifest names no animation"))?
        .to_owned();
    let text = read(&format!("a/{first}.json"))
        .or_else(|| read(&format!("animations/{first}.json")))
        .ok_or_else(|| anyhow::anyhow!("animation {first} isn't in the file"))?;
    let mut doc: Value = serde_json::from_slice(&text)?;
    inline(&mut doc, |folder, file| {
        let file = file.rsplit('/').next().unwrap_or(file);
        let folder = folder.trim_start_matches(['/', '.']).trim_end_matches('/');
        [format!("{folder}/{file}"), format!("i/{file}"), format!("images/{file}")].iter().find_map(|n| read(n))
    });
    Ok(doc)
}

/// `GET /api/assets/{id}/lottie.json`: the animation to play. A `.lottie`'s is taken out of the zip on first request
/// and kept as the `lottie` derivative.
pub(crate) async fn asset_lottie(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let id2 = id.clone();
    let (hash, ext, is_lottie, made) = db::call(&state.pool, move |c| {
        Ok(c.query_row(
            "SELECT a.blob_hash, lower(a.ext), json_extract(a.meta, '$.lottie') IS NOT NULL,
                    (SELECT blob_hash FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'lottie')
               FROM assets a WHERE a.id = ?1",
            [&id2],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, bool>(2)?,
                    r.get::<_, Option<String>>(3)?,
                ))
            },
        )
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    let not_found = || AppError::not_found(tr!("error-derivative-not-found", kind = "lottie", id = id.clone()));
    if !is_lottie {
        return Err(not_found());
    }
    let hash = match (ext.as_str(), made) {
        ("lottie", Some(made)) => made,
        ("lottie", None) => {
            let bytes = state.blobs.read_all(&hash).await?;
            let doc = crate::heavy::run("lottie", move || from_dotlottie(&bytes)).await?.map_err(|e| {
                tracing::debug!(asset = %id, error = %format!("{e:#}"), "no animation in the dotLottie file");
                not_found()
            })?;
            crate::previews::save_derivative(&state, id.clone(), "lottie", serde_json::to_vec(&doc)?).await?
        }
        _ => hash,
    };
    crate::api::serve_blob(&state, &hash, &headers, "application/json", None).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    fn anim() -> Value {
        json!({"v": "5.7.4", "nm": "Spinner", "fr": 30, "ip": 0, "op": 45, "w": 200, "h": 100,
               "assets": [{"id": "img_0", "w": 1, "h": 1, "u": "images/", "p": "img_0.png", "e": 0}],
               "layers": [{"ty": 4}, {"ty": 2, "refId": "img_0"}]})
    }

    #[test]
    fn facts_of_an_animation() {
        let m = facts(&anim()).unwrap();
        assert_eq!(m["width"], 200);
        assert_eq!(m["frames"], 45);
        assert_eq!(m["duration_secs"], 1.5);
        assert_eq!(m["layers"], 2);
        assert_eq!(m["lottie_name"], "Spinner");
        assert!(facts(&json!({"frames": {}, "meta": {}})).is_none());
        assert!(facts(&json!({"v": "5", "fr": 30, "ip": 0, "op": 0, "w": 1, "h": 1, "layers": []})).is_none());
    }

    #[test]
    fn a_dotlottie_animation_with_its_images() {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut z = zip::ZipWriter::new(&mut buf);
            let o = zip::write::SimpleFileOptions::default();
            z.start_file("manifest.json", o).unwrap();
            z.write_all(br#"{"version":"1","animations":[{"id":"spin"}]}"#).unwrap();
            z.start_file("animations/spin.json", o).unwrap();
            z.write_all(anim().to_string().as_bytes()).unwrap();
            z.start_file("images/img_0.png", o).unwrap();
            z.write_all(b"png!").unwrap();
            z.finish().unwrap();
        }
        let doc = from_dotlottie(buf.get_ref()).unwrap();
        assert_eq!(doc["assets"][0]["p"], "data:image/png;base64,cG5nIQ==");
        assert_eq!(doc["assets"][0]["e"], 1);
        assert!(facts(&doc).is_some());
    }
}
