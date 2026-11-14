//! MCP (Model Context Protocol) endpoint at `/mcp`, so agents can search, inspect and pull assets.
//!
//! Dual-era Streamable HTTP server, stateless, JSON responses only (no SSE):
//! - modern clients (2026-07-28) send the protocol version in `params._meta` on every request and may call
//!   `server/discover`;
//! - legacy clients (2025-11-25 and earlier) open with `initialize`, which we answer without a session.
//!
//! Every request needs a sign-in: an app that signed in over OAuth (`oauth.rs`), or an access token from Settings →
//! Account. Always asking, even when guests may browse the portal, is what makes MCP apps offer to sign in.

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
`map:normal`, `license:cc0`). Use get_asset for full details, including linked textures/animations and the thumbnail's colours. find_similar finds \
assets that look like one you have (textures, models, sprites, across every pack), or sound like it for a sound, and find_by_color finds assets with \
much of a colour in them. To actually \
fetch files, use get_bundle: it returns direct download URLs for an asset and every file it depends on, plus a zip URL. \
To describe and tag assets nothing has described yet, loop next_to_tag → save_tags (the tag_pack prompt explains how).";

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

pub(crate) fn base_url(state: &AppState, headers: &HeaderMap) -> String {
    if let Some(u) = &state.settings.get().public_url {
        return u.trim_end_matches('/').to_owned();
    }
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok()).unwrap_or("localhost");
    let proto = headers.get("x-forwarded-proto").and_then(|v| v.to_str().ok()).unwrap_or("http");
    format!("{proto}://{host}")
}

async fn handle(State(state): State<AppState>, headers: HeaderMap, body: String) -> Response {
    if !origin_ok(&headers, state.settings.get().public_url.as_deref()) {
        return (StatusCode::FORBIDDEN, "origin not allowed").into_response();
    }
    let Some(who) = crate::auth::authenticate(&state, &http::Method::POST, &headers).await.ok().flatten() else {
        let challenge = crate::oauth::challenge(&state, &headers, headers.contains_key(header::AUTHORIZATION));
        return (
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, challenge)],
            "sign in to Hoard: connect over OAuth, or send an access token from Settings → Account as \
             `Authorization: Bearer <token>`",
        )
            .into_response();
    };
    let may_change = who.role() >= crate::auth::Role::Edit;
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
    let server_info = json!({ "name": "hoard", "title": "Hoard asset library", "version": hoard_core::VERSION });
    let reply = match method {
        "initialize" => {
            let requested = params.get("protocolVersion").and_then(Value::as_str).unwrap_or("2025-11-25");
            let version = if SUPPORTED[1..].contains(&requested) { requested } else { "2025-11-25" };
            rpc_result(
                &id,
                &json!({
                    "protocolVersion": version,
                    "capabilities": { "tools": { "listChanged": false }, "prompts": { "listChanged": false } },
                    "serverInfo": server_info,
                    "instructions": INSTRUCTIONS,
                }),
            )
        }
        "server/discover" => rpc_result(
            &id,
            &complete(json!({
                "supportedVersions": SUPPORTED,
                "capabilities": { "tools": {}, "prompts": {} },
                "_meta": { "io.modelcontextprotocol/serverInfo": server_info },
                "instructions": INSTRUCTIONS,
            })),
        ),
        "ping" => rpc_result(&id, &complete(json!({}))),
        "tools/list" => rpc_result(&id, &complete(json!({ "tools": tool_list(may_change) }))),
        "prompts/list" => rpc_result(&id, &complete(json!({ "prompts": prompt_list() }))),
        "prompts/get" => match get_prompt(&params) {
            Some(p) => rpc_result(&id, &complete(p)),
            None => rpc_error(&id, -32602, "unknown prompt", None),
        },
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or_default().to_owned();
            let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
            let base = base_url(&state, &headers);
            let result = if !may_change && CHANGES.contains(&name.as_str()) {
                Err(ToolError::Failed("this account can look but not change anything".into()))
            } else {
                call_tool(&state, &name, args, &base).await
            };
            match result {
                Ok(mut value) => {
                    let previews = value.as_object_mut().and_then(|o| o.remove(PREVIEWS_KEY));
                    let sheet = value.as_object_mut().and_then(|o| o.remove(SHEET_KEY));
                    let mut content = vec![
                        json!({ "type": "text", "text": serde_json::to_string_pretty(&value).unwrap_or_default() }),
                    ];
                    if let Some(Value::Array(p)) = previews {
                        content.extend(preview_blocks(&state, p).await);
                    }
                    if let Some(Value::String(png)) = sheet {
                        content.push(json!({ "type": "image", "data": png, "mimeType": "image/png" }));
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

/// The tools that change the library, which accounts that can only look don't get.
const CHANGES: [&str; 3] = ["tag_asset", "update_asset", "save_tags"];

fn tool_list(may_change: bool) -> Value {
    let (read, write) = (read_hints(), write_hints());
    let tools = json!([
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
            "name": "get_credits",
            "title": "Get credits",
            "description": "Who made the packs a project uses, where each is sold, its license and whether the license asks for attribution (yes, no, or 'terms': keep the license notice with the files; 'unknown' where the license isn't known). Give any mix of packs, collections, asset ids and a search; everything is put together. `markdown` returns a ready CREDITS.md.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "packs": { "type": "array", "items": { "type": "string" }, "description": "Pack ids or names." },
                    "collections": { "type": "array", "items": { "type": "string" }, "description": "Collection ids or names." },
                    "ids": { "type": "array", "items": { "type": "string" }, "description": "Asset ids." },
                    "query": { "type": "string", "description": "Also credit the packs of what this search finds." },
                    "kind": { "type": "string", "description": "With `query`: only this kind." },
                    "tags": { "type": "array", "items": { "type": "string" }, "description": "With `query`: all must match." },
                    "markdown": { "type": "boolean", "default": false, "description": "Return CREDITS.md text instead of JSON." }
                }
            },
            "annotations": read
        },
        {
            "name": "check_files",
            "title": "Check files before shipping",
            "description": "Given the BLAKE3 hashes of the files a project ships (b3sum), says which came from Hoard (the asset, its path and pack; a hash matches a file as Hoard keeps it, as it was uploaded before conversion, or a format Hoard made from it), the credits they need, and which files need a look first, with warnings: license_unknown, non_commercial, share_alike, keep_notice (ship the license notice), fab_personal (Fab's Personal tier), ai_generated / ai_edited (the image says so in its C2PA Content Credentials, IPTC metadata or generator settings). A file is ok when one pack it's in covers it. Hashes Hoard doesn't know are listed as not_found. `markdown` returns CREDITS.md with a ship-check section.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hashes": { "type": "array", "items": { "type": "string" }, "description": "BLAKE3 hashes, 64 hex digits each." },
                    "markdown": { "type": "boolean", "default": false, "description": "Return CREDITS.md text with the ship check instead of JSON." }
                },
                "required": ["hashes"]
            },
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
            "description": "Add and/or remove tags on an asset. Added tags are recorded as human-sourced and are never overwritten by the automatic taggers.",
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
            "description": "Set an asset's display name, category or description (its own field, next to the AI's and the file's). Edited fields are locked against the AI tagger.",
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
    ]);
    let mut all = search_tools();
    if let Some(list) = all.as_array_mut() {
        for more in [tools, tagging_tools()] {
            if let Value::Array(more) = more {
                list.extend(more);
            }
        }
        if !may_change {
            list.retain(|t| !t["name"].as_str().is_some_and(|n| CHANGES.contains(&n)));
        }
    }
    all
}

fn search_tools() -> Value {
    let read = read_hints();
    json!([
        {
            "name": "search_assets",
            "title": "Search assets",
            "description": "Full-text search over asset names, paths, categories, descriptions and tags, with filters. When image search is on (Settings), pictures that show the words match too, so 'mossy rock' finds mossy rocks whatever they're called. Returns up to `limit` matches with thumbnails and download URLs.",
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
            "name": "find_similar",
            "title": "Find assets that look or sound alike",
            "description": "Assets whose thumbnail looks like this asset's (colours, surface grain, edges, layout, and, when image search is on, what it shows), across every pack, most alike first: other rocks for a rock texture, near-copies at other sizes, renders of similar models. For a sound, the sounds that sound like it (its spectrum, how it rises and dies away, how bright, noisy and busy it is), whatever they're called. Same result shape as search_assets.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "The asset to match." },
                    "kind": { "type": "string", "enum": ["texture", "model", "material", "animation", "video", "audio", "other"] },
                    "query": { "type": "string", "description": "Optional words the matches must also have." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
                    "include_previews": { "type": "boolean", "default": false, "description": "Also return the first 8 thumbnails as images." }
                },
                "required": ["id"]
            },
            "annotations": read
        },
        {
            "name": "find_by_color",
            "title": "Find assets by colour",
            "description": "Assets with much of a colour in their thumbnail, those covered most by it first (colours are compared perceptually, so a slightly different shade still matches). Same result shape as search_assets.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "color": { "type": "string", "description": "Hex colour, e.g. '#3a7d2c'." },
                    "kind": { "type": "string", "enum": ["texture", "model", "material", "animation", "video", "other"] },
                    "query": { "type": "string", "description": "Optional words the matches must also have." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
                    "include_previews": { "type": "boolean", "default": false, "description": "Also return the first 8 thumbnails as images." }
                },
                "required": ["color"]
            },
            "annotations": read
        }
    ])
}

fn read_hints() -> Value {
    json!({ "readOnlyHint": true, "openWorldHint": false })
}

fn write_hints() -> Value {
    json!({ "readOnlyHint": false, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false })
}

/// The tools for describing assets nothing has described yet (see the `tag_pack` prompt).
fn tagging_tools() -> Value {
    let (read, write) = (read_hints(), write_hints());
    json!([
        {
            "name": "next_to_tag",
            "title": "Next assets to describe",
            "description": "The next files nothing has described yet (no AI or human tags), up to 16 at a time, from one pack: path, kind, measured facts, the offline tagger's tags, the pack's context (store page, README), and a contact sheet image whose white numbers match each file's `n`. A file with `variants` stands for a family of near-identical takes. Describe them and call save_tags; saved files drop out, so call again for the next batch until `files` is empty.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "pack": { "type": "string", "description": "Pack id or name. Leave out to take the newest pack that has undescribed files." },
                    "skip": { "type": "integer", "minimum": 0, "default": 0, "description": "Skip this many, e.g. files you decided to leave alone." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 16, "default": 16 }
                }
            },
            "annotations": read
        },
        {
            "name": "save_tags",
            "title": "Save descriptions and tags",
            "description": "Store names, categories, descriptions and tags for assets (usually a batch from next_to_tag), recorded as AI-made the same way Hoard's own AI tagger records them: never overriding what a person typed, and an answer for a family's representative covers its variants. Optionally the pack's description and tags.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "assets": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "id": { "type": "string" },
                                "name": { "type": "string", "description": "Short human title, no extension." },
                                "category": { "type": "string", "description": "One of Hoard's categories (see the tag_pack prompt)." },
                                "description": { "type": "string", "description": "1-3 sentences." },
                                "tags": { "type": "array", "items": { "type": "string" }, "description": "3-8 lowercase kebab-case tags." }
                            },
                            "required": ["id"]
                        }
                    },
                    "pack": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "string" },
                            "description": { "type": "string" },
                            "tags": { "type": "array", "items": { "type": "string" } }
                        },
                        "required": ["id"]
                    }
                },
                "required": ["assets"]
            },
            "annotations": write
        }
    ])
}

/// The prompts this server offers: ready-made instructions a client can show as a command.
fn prompt_list() -> Value {
    json!([{
        "name": "tag_pack",
        "title": "Describe and tag a pack",
        "description": "Go through the files in a pack that nothing has described yet, look at each one and save a name, category, description and tags.",
        "arguments": [{ "name": "pack", "description": "Pack name or id (leave empty for the newest pack with undescribed files)", "required": false }]
    }])
}

fn get_prompt(params: &Value) -> Option<Value> {
    if params.get("name").and_then(Value::as_str) != Some("tag_pack") {
        return None;
    }
    let pack = params.get("arguments").and_then(|a| a.get("pack")).and_then(Value::as_str).map(str::trim);
    let (which, call) = match pack.filter(|p| !p.is_empty()) {
        Some(p) => (format!("the pack \"{p}\""), format!("next_to_tag with pack \"{p}\"")),
        None => ("the newest pack with undescribed files".to_owned(), "next_to_tag".to_owned()),
    };
    let text = format!(
        "Describe and tag the files in Hoard's {which} that nothing has described yet.\n\n\
         Work in batches. Call {call}. It returns up to 16 files with their paths, measured facts, the offline \
         tagger's tags and the pack's context, plus a contact sheet: each tile's white number is a file's `n`. Look \
         at the sheet and the facts, then call save_tags with a name, category, description and tags for every file \
         in the batch, by `id`. If the pack has no description yet, include `pack` (id, description, tags) in the \
         first save_tags. Saved files drop out, so call next_to_tag again (with the pack id it returned) until \
         `files` is empty. If you can't say anything useful about a file, still save its tags, or skip it with \
         `skip`. When done, tell me how many you described and anything that looked wrong or mislabelled.\n\n\
         Rules:\n{}",
        crate::llm::tagging_rules()
    );
    Some(json!({
        "description": format!("Describe and tag {which}"),
        "messages": [{ "role": "user", "content": { "type": "text", "text": text } }],
    }))
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
const PREVIEWS_KEY: &str = "__previews";
/// Internal key for a contact sheet (base64 PNG) sent as an image content block.
const SHEET_KEY: &str = "__sheet";
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
            "mimeType": crate::analyze::thumb_mime(&bytes),
        }));
    }
    out
}

/// `check_files`: what the files a project ships came from, their credits and what needs a look.
async fn check_call(state: &AppState, args: &Value) -> Result<Value, ToolError> {
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
async fn credits_call(state: &AppState, args: &Value) -> Result<Value, ToolError> {
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
async fn tagging_call(state: &AppState, name: &str, args: &Value) -> Result<Value, ToolError> {
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
async fn search_call(state: &AppState, name: &str, args: &Value, base: &str) -> Result<Value, ToolError> {
    let similar = (name == "find_similar")
        .then(|| str_arg(args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into())))
        .transpose()?;
    let color = (name == "find_by_color")
        .then(|| match str_arg(args, "color") {
            Some(c) if crate::looks::parse_hex(&c).is_some() => Ok(c),
            _ => Err(ToolError::Failed("`color` must be a hex colour like '#3a7d2c'".into())),
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

async fn call_tool(state: &AppState, name: &str, args: Value, base: &str) -> Result<Value, ToolError> {
    if matches!(name, "search_assets" | "find_similar" | "find_by_color") {
        return search_call(state, name, &args, base).await;
    }
    if matches!(name, "next_to_tag" | "save_tags") {
        return tagging_call(state, name, &args).await;
    }
    let base = base.to_owned();
    match name {
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
            if !a.palette.is_empty() {
                v["colors"] = json!(a.palette);
            }
            Ok(v)
        }
        "get_bundle" => {
            let id = str_arg(&args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into()))?;
            let anims = args.get("include_animations").and_then(Value::as_bool).unwrap_or(false);
            let id2 = id.clone();
            let names = state.settings.get();
            let files = db::call(&state.pool, move |c| {
                if db::asset_file_info(c, &id2)?.is_none() {
                    return Ok(None);
                }
                let ids = db::dependency_closure(c, &id2, anims)?;
                let mut files = db::bundle_files(c, &ids)?;
                crate::naming::apply(c, &names.texture_names, &mut files)?;
                Ok(Some(files))
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
            let packs = db::call(&state.pool, |c| db::list_packs(c, false)).await?;
            Ok(json!({ "packs": packs }))
        }
        "get_credits" => credits_call(state, &args).await,
        "check_files" => check_call(state, &args).await,
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
            let tags = db::call(&state.pool, move |c| db::tag_counts(c, &prefix, false, limit)).await?;
            Ok(json!({ "tags": tags }))
        }
        "tag_asset" | "update_asset" => {
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
