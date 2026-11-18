//! Answering the search, inspect, credits and tagging tools: an asset summed up with its links, and previews
//! attached as pictures.

use base64::Engine as _;
use hoard_core::SearchQuery;
use rusqlite::OptionalExtension as _;
use serde_json::{Value, json};

use super::ToolError;
use crate::{AppState, db};

pub(super) fn str_arg(args: &Value, key: &str) -> Option<String> {
    args.get(key).and_then(Value::as_str).map(str::to_owned).filter(|s| !s.is_empty())
}

pub(super) fn str_list(args: &Value, key: &str) -> Vec<String> {
    args.get(key).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_owned).collect()
}

fn asset_urls(base: &str, a: &hoard_core::Asset) -> Value {
    let mut urls = json!({
        "file": format!("{base}/api/assets/{}/file?download=1", a.id),
        "page": format!("{base}/#asset={}", a.id),
    });
    if a.has_thumb {
        urls["thumbnail"] = json!(format!("{base}/api/assets/{}/thumb", a.id));
    }
    if !a.links.is_empty() {
        urls["bundle_zip"] = json!(format!("{base}/api/assets/{}/bundle.zip", a.id));
    }
    if a.derivatives.iter().any(|d| d == "glb") {
        urls["converted_glb"] = json!(format!("{base}/api/assets/{}/derivative/glb", a.id));
    }
    // The original WAV/PNG recovered from inside an Unreal package (search hits only carry the meta flag).
    if let Some(ext) = a.meta.get("source_ext").and_then(|v| v.as_str()) {
        urls[format!("recovered_{ext}")] = json!(format!("{base}/api/assets/{}/derivative/source?download=1", a.id));
    }
    urls
}

pub(super) fn summarize(base: &str, a: &hoard_core::Asset) -> Value {
    json!({
        "id": a.id,
        "name": a.name,
        "kind": a.kind,
        "category": a.category,
        // A person's description wins, then the AI's, then what the file says about itself.
        "description": a.description.as_ref().or(a.ai_description.as_ref()).or(a.source_description.as_ref()),
        "pack": a.pack_name,
        "path": a.path,
        "size_bytes": a.size,
        "tags": a.tags.iter().map(|t| t.tag.as_str()).filter(|t| !t.starts_with("format:") && !t.starts_with("kind:")).collect::<Vec<_>>(),
        "meta": a.meta,
        "urls": asset_urls(base, a),
    })
}

/// Internal key a tool result uses to ask for image content blocks; stripped before the result is sent.
pub(super) const PREVIEWS_KEY: &str = "__previews";
/// Internal key for a contact sheet (base64 PNG) sent as an image content block.
pub(super) const SHEET_KEY: &str = "__sheet";
const MAX_PREVIEWS: usize = 8;

/// Inline thumbnails as MCP image content, each preceded by a caption naming the asset.
pub(super) async fn preview_blocks(state: &AppState, wanted: Vec<Value>) -> Vec<Value> {
    let ids: Vec<String> =
        wanted.iter().filter_map(|w| w.get("id").and_then(Value::as_str).map(str::to_owned)).collect();
    let Ok(hashes) = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare("SELECT thumb_hash FROM assets WHERE id = ?1")?;
        ids.iter()
            .map(|id| Ok(stmt.query_row([id], |r| r.get::<_, Option<String>>(0)).optional()?.flatten()))
            .collect::<anyhow::Result<Vec<_>>>()
    })
    .await
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (w, hash) in wanted.iter().zip(hashes) {
        let Some(hash) = hash else { continue };
        let Ok(bytes) = state.blobs.read_all(&hash).await else { continue };
        let (id, name) = (w["id"].as_str().unwrap_or_default(), w["name"].as_str().unwrap_or_default());
        out.push(json!({ "type": "text", "text": format!("Preview of {name} ({id}):") }));
        out.push(json!({
            "type": "image",
            "data": base64::engine::general_purpose::STANDARD.encode(&bytes),
            "mimeType": crate::analyze::thumb_mime(&bytes),
        }));
    }
    out
}

/// `check_files`: what the files a project ships came from, their credits and what needs a look.
pub(super) async fn check_call(state: &AppState, args: &Value) -> Result<Value, ToolError> {
    let hashes = str_list(args, "hashes");
    if hashes.is_empty() {
        return Err(ToolError::Failed("give `hashes`: the BLAKE3 hashes of the files".into()));
    }
    if hashes.len() > crate::check::MAX_HASHES {
        return Err(ToolError::Failed(format!("at most {} hashes at a time", crate::check::MAX_HASHES)));
    }
    let result = db::call(&state.pool, move |c| crate::check::check(c, &hashes))
        .await
        .map_err(|e| ToolError::Failed(format!("{e:#}")))?;
    if args.get("markdown").and_then(Value::as_bool) == Some(true) {
        Ok(json!({ "markdown": crate::check::markdown(&result) }))
    } else {
        Ok(json!(result))
    }
}

/// `get_credits`: credits for any mix of packs, collections, asset ids and a search.
pub(super) async fn credits_call(state: &AppState, args: &Value) -> Result<Value, ToolError> {
    let tags: Vec<String> = str_list(args, "tags").iter().filter_map(|t| hoard_core::normalize_tag(t)).collect();
    let kind = str_arg(args, "kind");
    let query = str_arg(args, "query");
    let search = (query.is_some() || kind.is_some() || !tags.is_empty()).then(|| SearchQuery {
        q: query,
        kind,
        tags,
        ..Default::default()
    });
    let req = hoard_core::CreditsRequest {
        packs: str_list(args, "packs"),
        collections: str_list(args, "collections"),
        ids: str_list(args, "ids"),
        selections: search
            .map(|q| hoard_core::ManifestRequest { query: Some(q), include_deps: true, ..Default::default() })
            .into_iter()
            .collect(),
    };
    let credits =
        crate::credits::for_request(state, req).await.map_err(|e| ToolError::Failed(format!("{:#}", e.err)))?;
    if args.get("markdown").and_then(Value::as_bool) == Some(true) {
        Ok(json!({ "markdown": crate::credits::markdown(&credits) }))
    } else {
        Ok(json!(credits))
    }
}

/// [`tagging_tools`]' calls.
pub(super) async fn tagging_call(state: &AppState, name: &str, args: &Value) -> Result<Value, ToolError> {
    match name {
        "next_to_tag" => {
            let skip = args.get("skip").and_then(Value::as_u64).unwrap_or(0);
            let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(16);
            let next = crate::llm::next_untagged(
                state,
                str_arg(args, "pack"),
                usize::try_from(skip).unwrap_or(usize::MAX),
                usize::try_from(limit).unwrap_or(16),
            )
            .await?;
            let Some((mut out, sheet)) = next else {
                return Ok(json!({ "files": [], "next": "Every asset has been described. Nothing left to tag." }));
            };
            if let Some(png) = sheet {
                out[SHEET_KEY] = json!(base64::engine::general_purpose::STANDARD.encode(png));
            }
            Ok(out)
        }
        "save_tags" => {
            let assets: Vec<crate::llm::Described> =
                serde_json::from_value(args.get("assets").cloned().unwrap_or_default())
                    .map_err(|e| ToolError::Failed(format!("`assets`: {e}")))?;
            let pack: Option<crate::llm::PackDescribed> = match args.get("pack") {
                None | Some(Value::Null) => None,
                Some(p) => {
                    Some(serde_json::from_value(p.clone()).map_err(|e| ToolError::Failed(format!("`pack`: {e}")))?)
                }
            };
            let (saved, unknown) = crate::llm::save_described(state, pack, assets).await?;
            let mut out = json!({ "saved_assets": saved });
            if !unknown.is_empty() {
                out["unknown_ids"] = json!(unknown);
            }
            Ok(out)
        }
        _ => Err(ToolError::Unknown),
    }
}

/// `search_assets`, `find_similar` and `find_by_color`: one search with different required arguments.
pub(super) async fn search_call(state: &AppState, name: &str, args: &Value, base: &str) -> Result<Value, ToolError> {
    let similar = (name == "find_similar")
        .then(|| str_arg(args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into())))
        .transpose()?;
    let color = (name == "find_by_color")
        .then(|| match str_arg(args, "color") {
            Some(c) if crate::looks::parse_hex(&c).is_some() => Ok(c),
            _ => Err(ToolError::Failed("`color` must be a hex color like '#3a7d2c'".into())),
        })
        .transpose()?;
    let q = SearchQuery {
        q: str_arg(args, "query"),
        kind: str_arg(args, "kind"),
        tags: str_list(args, "tags").iter().filter_map(|t| hoard_core::normalize_tag(t)).collect(),
        pack: str_arg(args, "pack_id"),
        collection: str_arg(args, "collection"),
        limit: Some(args.get("limit").and_then(Value::as_u64).unwrap_or(20).clamp(1, 100) as u32),
        similar,
        color,
        ..Default::default()
    };
    let page = crate::looks::search(state, q, false).await?;
    let mut out = json!({
        "total": page.total,
        "returned": page.items.len(),
        "assets": page.items.iter().map(|a| summarize(base, a)).collect::<Vec<_>>(),
    });
    if args.get("include_previews").and_then(Value::as_bool) == Some(true) {
        let ids: Vec<Value> = page
            .items
            .iter()
            .filter(|a| a.has_thumb)
            .take(MAX_PREVIEWS)
            .map(|a| json!({ "id": a.id, "name": a.name }))
            .collect();
        out[PREVIEWS_KEY] = Value::Array(ids);
    }
    Ok(out)
}
