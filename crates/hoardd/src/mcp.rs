//! MCP (Model Context Protocol) endpoint at `/mcp`, so agents can search, inspect and pull assets.
//!
//! Dual-era Streamable HTTP server, stateless, JSON responses only (no SSE):
//! - modern clients (2026-07-28) send the protocol version in `params._meta` on every request and may call
//!   `server/discover`;
//! - legacy clients (2025-11-25 and earlier) open with `initialize`, which we answer without a session.
//!
//! Read tools are open like the REST API; write tools need the same bearer token as the REST writes.

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use base64::Engine as _;
use hoard_core::{SearchQuery, TagSource};
use rusqlite::{OptionalExtension as _, params};
use serde_json::{Value, json};

use crate::{AppState, db};

const SUPPORTED: &[&str] = &["2026-07-28", "2025-11-25", "2025-06-18", "2025-03-26"];
const META_VERSION: &str = "io.modelcontextprotocol/protocolVersion";

const INSTRUCTIONS: &str = "Hoard is a library of game assets (textures, sprites, audio, music, 3D models, fonts). \
Use search_assets to find things by words, kind and tags (tags look like `sfx`, `material:stone`, `style:pixel-art`, \
`map:normal`, `license:cc0`). Use get_asset for full details, including linked textures/animations. To actually \
fetch files, use get_bundle: it returns direct download URLs for an asset and every file it depends on, plus a zip URL.";

pub(crate) fn router(_state: &AppState) -> Router<AppState> {
    Router::new().route("/mcp", post(handle).get(|| async { StatusCode::METHOD_NOT_ALLOWED }))
}

fn rpc_result(id: &Value, result: &Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn rpc_error(id: &Value, code: i64, message: &str, data: Option<Value>) -> Value {
    let mut err = json!({ "code": code, "message": message });
    if let Some(d) = data {
        err["data"] = d;
    }
    json!({ "jsonrpc": "2.0", "id": id, "error": err })
}

/// Origin check against DNS rebinding: a browser-originated request must come from this server's own host.
fn origin_ok(headers: &HeaderMap, public_url: Option<&str>) -> bool {
    let Some(origin) = headers.get(header::ORIGIN).and_then(|v| v.to_str().ok()) else { return true };
    let origin_host = origin.split("://").nth(1).unwrap_or(origin);
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok()).unwrap_or("");
    origin_host == host
        || public_url.is_some_and(|u| u.split("://").nth(1).unwrap_or(u).trim_end_matches('/') == origin_host)
}

fn base_url(state: &AppState, headers: &HeaderMap) -> String {
    if let Some(u) = &state.cfg.public_url {
        return u.trim_end_matches('/').to_owned();
    }
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok()).unwrap_or("localhost");
    let proto = headers.get("x-forwarded-proto").and_then(|v| v.to_str().ok()).unwrap_or("http");
    format!("{proto}://{host}")
}

fn authorized(state: &AppState, headers: &HeaderMap) -> bool {
    let Some(expected) = &state.cfg.token else { return true };
    let given = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");
    given.len() == expected.len() && given.bytes().zip(expected.bytes()).fold(0u8, |acc, (a, b)| acc | (a ^ b)) == 0
}

async fn handle(State(state): State<AppState>, headers: HeaderMap, body: String) -> Response {
    if !origin_ok(&headers, state.cfg.public_url.as_deref()) {
        return (StatusCode::FORBIDDEN, "origin not allowed").into_response();
    }
    let msg: Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(e) => return Json(rpc_error(&Value::Null, -32700, &format!("parse error: {e}"), None)).into_response(),
    };
    let Some(method) = msg.get("method").and_then(Value::as_str) else {
        return Json(rpc_error(&Value::Null, -32600, "expected a single JSON-RPC request object", None))
            .into_response();
    };
    let Some(id) = msg.get("id").cloned() else {
        // Notifications (e.g. legacy `notifications/initialized`) need no reply.
        return StatusCode::ACCEPTED.into_response();
    };
    let params = msg.get("params").cloned().unwrap_or(Value::Null);

    // Version: modern requests carry it in _meta (and the header); legacy ones in the header after initialize.
    let meta_version = params.get("_meta").and_then(|m| m.get(META_VERSION)).and_then(Value::as_str);
    let header_version = headers.get("mcp-protocol-version").and_then(|v| v.to_str().ok());
    let modern = meta_version.is_some();
    if let Some(v) = meta_version.or(header_version).filter(|_| method != "initialize")
        && !SUPPORTED.contains(&v)
    {
        let data = json!({ "supported": SUPPORTED, "requested": v });
        return Json(rpc_error(&id, -32022, "Unsupported protocol version", Some(data))).into_response();
    }

    let complete = |mut v: Value| {
        if modern && let Some(o) = v.as_object_mut() {
            o.insert("resultType".into(), json!("complete"));
        }
        v
    };
    let server_info = json!({ "name": "hoard", "title": "Hoard asset library", "version": env!("CARGO_PKG_VERSION") });
    let reply = match method {
        "initialize" => {
            let requested = params.get("protocolVersion").and_then(Value::as_str).unwrap_or("2025-11-25");
            let version = if SUPPORTED[1..].contains(&requested) { requested } else { "2025-11-25" };
            rpc_result(
                &id,
                &json!({
                    "protocolVersion": version,
                    "capabilities": { "tools": { "listChanged": false } },
                    "serverInfo": server_info,
                    "instructions": INSTRUCTIONS,
                }),
            )
        }
        "server/discover" => rpc_result(
            &id,
            &complete(json!({
                "supportedVersions": SUPPORTED,
                "capabilities": { "tools": {} },
                "_meta": { "io.modelcontextprotocol/serverInfo": server_info },
                "instructions": INSTRUCTIONS,
            })),
        ),
        "ping" => rpc_result(&id, &complete(json!({}))),
        "tools/list" => rpc_result(&id, &complete(json!({ "tools": tool_list() }))),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or_default().to_owned();
            let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
            let base = base_url(&state, &headers);
            let auth = authorized(&state, &headers);
            match call_tool(&state, &name, args, &base, auth).await {
                Ok(mut value) => {
                    let previews = value.as_object_mut().and_then(|o| o.remove(PREVIEWS_KEY));
                    let mut content = vec![
                        json!({ "type": "text", "text": serde_json::to_string_pretty(&value).unwrap_or_default() }),
                    ];
                    if let Some(Value::Array(p)) = previews {
                        content.extend(preview_blocks(&state, p).await);
                    }
                    rpc_result(
                        &id,
                        &complete(json!({ "content": content, "structuredContent": value, "isError": false })),
                    )
                }
                Err(ToolError::Unknown) => rpc_error(&id, -32602, &format!("unknown tool: {name}"), None),
                // Tool failures are results the model can read and react to, not protocol errors.
                Err(ToolError::Failed(msg)) => {
                    rpc_result(&id, &complete(json!({ "content": [{ "type": "text", "text": msg }], "isError": true })))
                }
            }
        }
        _ => rpc_error(&id, -32601, &format!("method not found: {method}"), None),
    };
    Json(reply).into_response()
}

fn tool_list() -> Value {
    let read = json!({ "readOnlyHint": true, "openWorldHint": false });
    let write =
        json!({ "readOnlyHint": false, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false });
    json!([
        {
            "name": "search_assets",
            "title": "Search assets",
            "description": "Full-text search over asset names, paths, categories, descriptions and tags, with filters. Returns up to `limit` matches with thumbnails and download URLs.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Words to search for, e.g. 'grass footsteps'." },
                    "kind": { "type": "string", "enum": ["texture", "audio", "model", "animation", "font", "shader", "material", "video", "document", "data", "other"] },
                    "tags": { "type": "array", "items": { "type": "string" }, "description": "All must match, e.g. ['sfx', 'material:stone']." },
                    "pack_id": { "type": "string" },
                    "collection": { "type": "string", "description": "Only assets in this collection (id or name)." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
                    "include_previews": { "type": "boolean", "default": false, "description": "Also return the first 8 thumbnails as images, so you can see what you found." }
                }
            },
            "annotations": read
        },
        {
            "name": "get_asset",
            "title": "Get asset details",
            "description": "Everything known about one asset: metadata (size, duration, poly count…), tags with their source, linked dependencies (textures, buffers, materials, animation clips), what uses it, and URLs.",
            "inputSchema": { "type": "object", "properties": { "id": { "type": "string" } }, "required": ["id"] },
            "annotations": read
        },
        {
            "name": "get_bundle",
            "title": "Get download links for an asset and its dependencies",
            "description": "Direct download URLs for an asset plus every file it needs, each with the relative path to save it at so references keep working, and a single zip URL.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "include_animations": { "type": "boolean", "default": false }
                },
                "required": ["id"]
            },
            "annotations": read
        },
        {
            "name": "list_packs",
            "title": "List packs",
            "description": "All uploaded packs with status, license, asset count and size.",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": read
        },
        {
            "name": "list_collections",
            "title": "List collections",
            "description": "Hand-picked sets of assets (e.g. everything chosen for one level), with asset counts and a zip URL each. Pass a collection's id or name to search_assets as `collection` to see its contents.",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": read
        },
        {
            "name": "list_tags",
            "title": "List tags",
            "description": "Most-used tags, optionally filtered by prefix (e.g. 'style:' or 'material:'). Useful to learn the vocabulary before searching.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "prefix": { "type": "string", "default": "" },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 200, "default": 50 }
                }
            },
            "annotations": read
        },
        {
            "name": "tag_asset",
            "title": "Add or remove tags",
            "description": "Add and/or remove tags on an asset. Added tags are recorded as human-sourced and are never overwritten by the automatic taggers. Needs the server token if one is configured.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "add": { "type": "array", "items": { "type": "string" } },
                    "remove": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["id"]
            },
            "annotations": write
        },
        {
            "name": "update_asset",
            "title": "Rename or recategorize an asset",
            "description": "Set an asset's display name, category or description. Edited fields are locked against the LLM tagger. Needs the server token if one is configured.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "name": { "type": "string" },
                    "category": { "type": "string" },
                    "description": { "type": "string" }
                },
                "required": ["id"]
            },
            "annotations": write
        }
    ])
}

#[derive(Debug)]
enum ToolError {
    Unknown,
    Failed(String),
}

impl<E: std::fmt::Display> From<E> for ToolError {
    fn from(e: E) -> Self {
        ToolError::Failed(e.to_string())
    }
}

fn str_arg(args: &Value, key: &str) -> Option<String> {
    args.get(key).and_then(Value::as_str).map(str::to_owned).filter(|s| !s.is_empty())
}

fn str_list(args: &Value, key: &str) -> Vec<String> {
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

fn summarize(base: &str, a: &hoard_core::Asset) -> Value {
    json!({
        "id": a.id,
        "name": a.name,
        "kind": a.kind,
        "category": a.category,
        "description": a.description,
        "pack": a.pack_name,
        "path": a.path,
        "size_bytes": a.size,
        "tags": a.tags.iter().map(|t| t.tag.as_str()).filter(|t| !t.starts_with("format:") && !t.starts_with("kind:")).collect::<Vec<_>>(),
        "meta": a.meta,
        "urls": asset_urls(base, a),
    })
}

/// Internal key a tool result uses to ask for image content blocks; stripped before the result is sent.
const PREVIEWS_KEY: &str = "__previews";
const MAX_PREVIEWS: usize = 8;

/// Inline thumbnails as MCP image content, each preceded by a caption naming the asset.
async fn preview_blocks(state: &AppState, wanted: Vec<Value>) -> Vec<Value> {
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
            "mimeType": "image/png",
        }));
    }
    out
}

async fn call_tool(
    state: &AppState,
    name: &str,
    args: Value,
    base: &str,
    authorized: bool,
) -> Result<Value, ToolError> {
    let base = base.to_owned();
    match name {
        "search_assets" => {
            let q = SearchQuery {
                q: str_arg(&args, "query"),
                kind: str_arg(&args, "kind"),
                tags: str_list(&args, "tags").iter().filter_map(|t| hoard_core::normalize_tag(t)).collect(),
                pack: str_arg(&args, "pack_id"),
                collection: str_arg(&args, "collection"),
                limit: Some(args.get("limit").and_then(Value::as_u64).unwrap_or(20).clamp(1, 100) as u32),
                ..Default::default()
            };
            let page = db::call(&state.pool, move |c| db::search(c, &q, false)).await?;
            let mut out = json!({
                "total": page.total,
                "returned": page.items.len(),
                "assets": page.items.iter().map(|a| summarize(&base, a)).collect::<Vec<_>>(),
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
        "get_asset" => {
            let id = str_arg(&args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into()))?;
            let asset = db::call(&state.pool, move |c| db::get_asset(c, &id)).await?;
            let a = asset.ok_or_else(|| ToolError::Failed("no asset with that id".into()))?;
            let mut v = summarize(&base, &a);
            v["tag_sources"] =
                json!(a.tags.iter().map(|t| json!({ "tag": t.tag, "source": t.source })).collect::<Vec<_>>());
            v["hash_blake3"] = json!(a.hash);
            v["links"] = json!(a.links);
            v["linked_from"] = json!(a.linked_from);
            if let Some(f) = &a.family {
                v["variants"] = json!(f.members);
            }
            if !a.duplicates.is_empty() {
                v["identical_copies"] = json!(a.duplicates);
            }
            Ok(v)
        }
        "get_bundle" => {
            let id = str_arg(&args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into()))?;
            let anims = args.get("include_animations").and_then(Value::as_bool).unwrap_or(false);
            let id2 = id.clone();
            let files = db::call(&state.pool, move |c| {
                if db::asset_file_info(c, &id2)?.is_none() {
                    return Ok(None);
                }
                let ids = db::dependency_closure(c, &id2, anims)?;
                Ok(Some(db::bundle_files(c, &ids)?))
            })
            .await?
            .ok_or_else(|| ToolError::Failed("no asset with that id".into()))?;
            let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
            let common = db::common_dir(&paths);
            Ok(json!({
                "zip_url": format!("{base}/api/assets/{id}/bundle.zip{}", if anims { "?animations=1" } else { "" }),
                "files": files.iter().map(|f| json!({
                    "id": f.id,
                    "save_as": db::strip_dir(&f.path, &common),
                    "size_bytes": f.size,
                    "blake3": f.hash,
                    "url": format!("{base}/api/blobs/{}", f.hash),
                })).collect::<Vec<_>>(),
            }))
        }
        "list_packs" => {
            let packs = db::call(&state.pool, |c| db::list_packs(c)).await?;
            Ok(json!({ "packs": packs }))
        }
        "list_collections" => {
            let cols = db::call(&state.pool, |c| db::list_collections(c)).await?;
            let cols: Vec<Value> = cols
                .into_iter()
                .map(|c| {
                    json!({
                        "id": c.id,
                        "name": c.name,
                        "description": c.description,
                        "asset_count": c.asset_count,
                        "total_bytes": c.total_bytes,
                        "zip": format!("{base}/api/collections/{}/bundle.zip", c.id),
                    })
                })
                .collect();
            Ok(json!({ "collections": cols }))
        }
        "list_tags" => {
            let prefix = str_arg(&args, "prefix").unwrap_or_default().to_lowercase();
            let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(50).clamp(1, 200) as u32;
            let tags = db::call(&state.pool, move |c| db::tag_counts(c, &prefix, limit)).await?;
            Ok(json!({ "tags": tags }))
        }
        "tag_asset" | "update_asset" => {
            if !authorized {
                return Err(ToolError::Failed(
                    "This server requires a token for changes: configure the MCP client to send `Authorization: Bearer <HOARD_TOKEN>`.".into(),
                ));
            }
            let id = str_arg(&args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into()))?;
            let is_tag = name == "tag_asset";
            let (add, remove) = (str_list(&args, "add"), str_list(&args, "remove"));
            let (n, cat, desc) = (str_arg(&args, "name"), str_arg(&args, "category"), str_arg(&args, "description"));
            let asset = db::call(&state.pool, move |c| {
                let tx = c.transaction()?;
                if !tx.query_row("SELECT EXISTS (SELECT 1 FROM assets WHERE id = ?1)", [&id], |r| r.get::<_, bool>(0))? {
                    return Ok(None);
                }
                if is_tag {
                    for t in add.iter().filter_map(|t| hoard_core::normalize_tag(t)) {
                        db::upsert_tag(&tx, &id, &t, TagSource::Human, 1.0)?;
                    }
                    for t in &remove {
                        tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag = ?2", params![id, t])?;
                    }
                } else {
                    tx.execute(
                        "UPDATE assets SET name = COALESCE(?2, name), category = COALESCE(?3, category),
                                description = COALESCE(?4, description), human_edited = 1, updated_at = ?5 WHERE id = ?1",
                        params![id, n, cat, desc, db::now()],
                    )?;
                }
                db::reindex_asset(&tx, &id)?;
                tx.commit()?;
                db::get_asset(c, &id)
            })
            .await?
            .ok_or_else(|| ToolError::Failed("no asset with that id".into()))?;
            Ok(summarize(&base, &asset))
        }
        _ => Err(ToolError::Unknown),
    }
}
