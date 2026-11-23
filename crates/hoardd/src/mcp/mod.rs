//! MCP (Model Context Protocol) endpoint at `/mcp`, so agents can search, inspect and pull assets.
//!
//! Dual-era Streamable HTTP server, stateless, JSON responses only (no SSE):
//! - modern clients (2026-07-28) send the protocol version in `params._meta` on every request and may call
//!   `server/discover`;
//! - legacy clients (2025-11-25 and earlier) open with `initialize`, which we answer without a session.
//!
//! Every request needs a sign-in: an app that signed in over OAuth (`oauth.rs`), or an access token from Settings →
//! Account. Always asking, even when guests may browse the portal, is what makes MCP apps offer to sign in.

mod calls;
pub(crate) mod live;
pub(crate) mod operations;
mod sound_tools;
mod tools;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{Value, json};

use crate::{AppState, db};
use calls::{
    PREVIEWS_KEY, SHEET_KEY, check_call, credits_call, preview_blocks, search_call, str_arg, str_list, summarize,
    tagging_call,
};
use sound_tools::sounds_call;
use tools::{CHANGES, tool_list};

const SUPPORTED: &[&str] = &["2026-07-28", "2025-11-25", "2025-06-18", "2025-03-26"];
const META_VERSION: &str = "io.modelcontextprotocol/protocolVersion";

const INSTRUCTIONS: &str = "Hoard is a library of game assets (textures, sprites, audio, music, 3D models, fonts). \
Use search_assets to find things by words, kind and tags (tags look like `sfx`, `material:stone`, `style:pixel-art`, \
`map:normal`, `license:cc0`). Use get_asset for full details, including linked textures/animations and the thumbnail's colors. find_similar finds \
assets that look like one you have (textures, models, sprites, across every pack), or sound like it for a sound, and find_by_color finds assets with \
much of a color in them. To find a sound by what it sounds like, use search_sounds: pass the word someone used for the \
noise (\"WHOOOM\", \"pew\", \"krssh\") as `imitation` and, when you can tell more than the spelling says, your own reading \
of it as `shape`; describe_sound says what a sound sounds like and sounds_like finds others like it. To actually \
fetch files, use get_bundle: it returns direct download URLs for an asset and every file it depends on, plus a zip URL. \
To describe and tag assets nothing has described yet, loop next_to_tag → save_tags (the tag_pack prompt explains how). Everything else the Hoard web app does is here too, with this sign-in's permissions: reorganizing tags (parents, merges, aliases), renaming and retagging packs, collections, the archive, projects, storage locations, settings and maintenance. find_operations finds the operation for what you want to do, describe_operation says what it takes, and read_operation (to look) or run_operation (to change) calls it. Say what you're about to change before a change that touches many things or deletes.";

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
    live::record(&who, method, &params);

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
                match operations::call(&state, who.role(), &name, &args, &headers, &base).await {
                    Some(result) => result,
                    None => call_tool(&state, &who, &name, args, &base).await,
                }
            };
            match result {
                Ok(mut value) => {
                    // Structured content is always an object.
                    if !value.is_object() {
                        value = json!({ "result": value });
                    }
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
         `skip`. When done, tell me how many you described and anything that looked wrong or mislabeled.\n\n\
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

async fn call_tool(
    state: &AppState,
    who: &crate::auth::Who,
    name: &str,
    args: Value,
    base: &str,
) -> Result<Value, ToolError> {
    if matches!(name, "search_assets" | "find_similar" | "find_by_color") {
        return search_call(state, name, &args, base).await;
    }
    if matches!(name, "next_to_tag" | "save_tags") {
        return tagging_call(state, name, &args).await;
    }
    if matches!(name, "search_sounds" | "describe_sound" | "sounds_like") {
        return sounds_call(state, name, &args, base).await;
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
            // A help, not the point: an asset still comes back if they can't be looked for.
            let near = crate::nearcopies::for_asset(state, a.id.clone()).await.unwrap_or_default();
            if !near.is_empty() {
                v["near_copies"] = json!(near);
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
            let packs = db::call(&state.pool, |c| db::list_packs(c, db::Packs::Library)).await?;
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
            let (tags, tree) =
                db::call(&state.pool, move |c| Ok((db::tag_counts(c, &prefix, false, limit)?, db::tag_tree(c)?)))
                    .await?;
            // Each with the broader tags it goes under, when it goes under any: searching one finds what's under it.
            let tags: Vec<Value> = tags
                .into_iter()
                .map(|t| {
                    let under: Vec<&str> =
                        tree.iter().filter(|(tag, _)| *tag == t.value).map(|(_, p)| p.as_str()).collect();
                    let mut v = json!({ "value": t.value, "count": t.count });
                    if !under.is_empty() {
                        v["under"] = json!(under);
                    }
                    v
                })
                .collect();
            Ok(json!({ "tags": tags }))
        }
        "tag_asset" | "update_asset" => {
            let id = str_arg(&args, "id").ok_or_else(|| ToolError::Failed("`id` is required".into()))?;
            let is_tag = name == "tag_asset";
            let (add, remove) = (str_list(&args, "add"), str_list(&args, "remove"));
            let patch = if is_tag {
                hoard_core::AssetPatch { add_tags: add, remove_tags: remove, ..Default::default() }
            } else {
                hoard_core::AssetPatch {
                    name: str_arg(&args, "name"),
                    category: str_arg(&args, "category"),
                    description: str_arg(&args, "description"),
                    ..Default::default()
                }
            };
            let actor = db::Actor::mcp(who);
            let asset = db::call(&state.pool, move |c| {
                let tx = c.transaction()?;
                if !db::apply(&tx, &id, &patch, &actor)? {
                    return Ok(None);
                }
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
