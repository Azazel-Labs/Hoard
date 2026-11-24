//! Everything else the HTTP API does, as four tools: `find_operations` and `describe_operation` to find an operation
//! of the catalog ([`crate::catalog`]) and what it takes, `read_operation` and `run_operation` to call it. A call goes
//! through the same router as the portal's, with the same sign-in, so an assistant may do just what its account or
//! token may, and nothing about an operation needs saying twice.

use std::sync::{Arc, Mutex, PoisonError};

use axum::Router;
use axum::body::Body;
use http::{HeaderMap, HeaderValue, Request, StatusCode, header};
use serde_json::{Value, json};
use tower::ServiceExt as _;

use super::ToolError;
use crate::AppState;
use crate::auth::Role;
use crate::catalog::{self, Body as Takes, CATALOG, Described};
use crate::store_search::percent_encode;

tokio::task_local! {
    /// Set while an operation runs for an assistant, so that what it changes is recorded as done over MCP.
    pub(crate) static OVER_MCP: ();
}

/// Whether what's running now was asked for over MCP.
pub(crate) fn over_mcp() -> bool {
    OVER_MCP.try_with(|()| ()).is_ok()
}

/// The most of an answer handed back, in bytes of JSON: the longest lists in it are cut short to fit.
const MOST_ANSWER: usize = 256 * 1024;

/// The most of an answer read before giving up on it.
const MOST_READ: usize = 16 << 20;

/// The tools, for `tools/list`. `run_operation` is only for accounts that can change something.
pub(super) fn tools(may_change: bool) -> Vec<Value> {
    let input = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string", "description": "The operation, as find_operations names it." },
            "path": { "type": "object", "description": "Its path's parameters, by name (`{\"id\": \"…\"}`)." },
            "query": { "type": "object", "description": "Its query parameters, by name." },
            "body": { "description": "Its JSON body, as describe_operation says." }
        },
        "required": ["name"]
    });
    let mut tools = vec![
        json!({
            "name": "find_operations",
            "title": "Find what else Hoard can do",
            "description": "Everything the Hoard web app does is an operation of its HTTP API, and every one this \
                sign-in may call is here, beyond the tools above: organizing tags (parents, merges, aliases), packs, \
                collections, the archive, projects, storage locations, settings, maintenance and more. Give `words` \
                (\"tag parents\", \"rename pack\", \"locations\") for the best matches, or nothing for the groups there \
                are (then `group` lists one). Then describe_operation says what one takes, and read_operation (for a \
                GET) or run_operation (for a change) calls it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "words": { "type": "string", "description": "What you want to do, in a few words." },
                    "group": { "type": "string", "description": "Every operation of this group, as listed with no words." }
                }
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "describe_operation",
            "title": "Say what an operation takes",
            "description": "What an operation found with find_operations does and takes: its path parameters, its \
                query parameters and its body, as JSON Schema.",
            "inputSchema": {
                "type": "object",
                "properties": { "name": { "type": "string" } },
                "required": ["name"]
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "read_operation",
            "title": "Call an operation that only looks",
            "description": "Call an operation whose method is GET (see describe_operation), and get its answer. A \
                file comes back as the address to fetch it from.",
            "inputSchema": input,
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
    ];
    if may_change {
        tools.push(json!({
            "name": "run_operation",
            "title": "Call an operation that changes something",
            "description": "Call an operation that changes something (POST, PUT, PATCH or DELETE; see \
                describe_operation), as the account or token this sign-in is, and get its answer. What it changes \
                in an asset's history is recorded as done over MCP. Some can't be undone: say what you're about to \
                do first when it deletes.",
            "inputSchema": input,
            "annotations": { "readOnlyHint": false, "destructiveHint": true, "idempotentHint": false, "openWorldHint": false }
        }));
    }
    tools
}

/// Calls one of the tools, or `None` if `name` isn't one of them.
pub(super) async fn call(
    state: &AppState,
    role: Role,
    name: &str,
    args: &Value,
    headers: &HeaderMap,
    base: &str,
) -> Option<Result<Value, ToolError>> {
    Some(match name {
        "find_operations" => Ok(find(role, args)),
        "describe_operation" => describe(role, args),
        "read_operation" => run(state, role, args, headers, base, false).await,
        "run_operation" => run(state, role, args, headers, base, true).await,
        _ => return None,
    })
}

/// What an assistant is offered: what this sign-in may call, less what's for people to do themselves.
fn offered(role: Role) -> impl Iterator<Item = &'static Described> {
    CATALOG.iter().filter(move |d| d.refused(role).is_none())
}

/// The group an operation is listed under: the thing its path is about.
fn group(path: &str) -> &str {
    let mut parts = path.trim_start_matches("/api/").split('/');
    let first = parts.next().unwrap_or_default();
    let first = if first == "admin" { parts.next().unwrap_or(first) } else { first };
    first.split('.').next().unwrap_or(first)
}

/// A word to match, folded so `packs` finds `pack` and `tagging` finds `tag`.
fn stem(word: &str) -> String {
    let word = word.to_lowercase();
    for end in ["ing", "es", "s"] {
        if word.len() > end.len() + 2
            && let Some(stem) = word.strip_suffix(end)
        {
            return stem.to_owned();
        }
    }
    word
}

fn listed(d: &Described) -> Value {
    json!({ "name": d.name, "method": d.method, "path": d.path, "summary": d.summary })
}

fn find(role: Role, args: &Value) -> Value {
    let words: Vec<String> = args["words"]
        .as_str()
        .unwrap_or_default()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 1)
        .map(stem)
        .collect();
    if let Some(g) = args["group"].as_str().filter(|g| !g.is_empty() && words.is_empty()) {
        let ops: Vec<Value> = offered(role).filter(|d| group(d.path) == g).map(listed).collect();
        return json!({ "group": g, "operations": ops });
    }
    if words.is_empty() {
        let mut groups: Vec<(&str, usize)> = Vec::new();
        for d in offered(role) {
            match groups.iter_mut().find(|(g, _)| *g == group(d.path)) {
                Some((_, n)) => *n += 1,
                None => groups.push((group(d.path), 1)),
            }
        }
        let groups: Vec<Value> = groups.into_iter().map(|(g, n)| json!({ "group": g, "operations": n })).collect();
        return json!({
            "groups": groups,
            "next": "find_operations with `words` (what you want to do), or `group` for every operation of one",
        });
    }
    let mut found: Vec<(usize, &Described)> = offered(role)
        .filter_map(|d| {
            let (name, path, summary) = (d.name.replace('_', " "), d.path.to_lowercase(), d.summary.to_lowercase());
            let score: usize = words
                .iter()
                .map(|w| {
                    let in_name = name.split(' ').any(|n| stem(n) == *w || n.starts_with(w.as_str()));
                    4 * usize::from(in_name)
                        + 2 * usize::from(path.contains(w.as_str()))
                        + usize::from(summary.contains(w.as_str()))
                })
                .sum();
            (score > 0).then_some((score, d))
        })
        .collect();
    found.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
    let ops: Vec<Value> = found.into_iter().take(12).map(|(_, d)| listed(d)).collect();
    json!({ "operations": ops, "next": "describe_operation says what one takes" })
}

fn lookup(role: Role, args: &Value) -> Result<&'static Described, ToolError> {
    let name = args["name"].as_str().ok_or_else(|| ToolError::Failed("`name` is required".into()))?;
    let d = catalog::find(name).ok_or_else(|| {
        ToolError::Failed(format!("there's no operation called `{name}`: find_operations lists them"))
    })?;
    d.refused(role).map_or(Ok(d), |why| Err(ToolError::Failed(why)))
}

fn describe(role: Role, args: &Value) -> Result<Value, ToolError> {
    let d = lookup(role, args)?;
    let mut v = serde_json::to_value(d)?;
    v["call_with"] = json!(if d.changes() { "run_operation" } else { "read_operation" });
    Ok(v)
}

/// The router operations run through, made once per server.
fn router(state: &AppState) -> Router {
    static MADE: Mutex<Option<(usize, Router)>> = Mutex::new(None);
    // The router keeps `state` alive, so while it's kept no other server's can be at the same address.
    let key = Arc::as_ptr(state) as usize;
    let mut made = MADE.lock().unwrap_or_else(PoisonError::into_inner);
    match &*made {
        Some((k, r)) if *k == key => r.clone(),
        _ => {
            let r = crate::api::router(state).with_state(Arc::clone(state));
            *made = Some((key, r.clone()));
            r
        }
    }
}

/// The address an operation is called at, its path's parameters and query filled in.
fn address(d: &Described, args: &Value) -> Result<String, ToolError> {
    let mut path = String::new();
    for part in d.path.split('/').skip(1) {
        path.push('/');
        let Some(name) = part.strip_prefix('{').and_then(|p| p.strip_suffix('}')) else {
            path.push_str(part);
            continue;
        };
        let (name, rest) = name.strip_prefix('*').map_or((name, false), |n| (n, true));
        let value = args["path"]
            .get(name)
            .or_else(|| args.get(name))
            .ok_or_else(|| ToolError::Failed(format!("`path.{name}` is required for `{}` ({})", d.name, d.path)))?;
        let value = scalar(value).ok_or_else(|| ToolError::Failed(format!("`path.{name}` must be a string")))?;
        if rest {
            path.push_str(&value.split('/').map(percent_encode).collect::<Vec<_>>().join("/"));
        } else {
            path.push_str(&percent_encode(&value));
        }
    }
    let mut query = Vec::new();
    for (k, v) in args["query"].as_object().into_iter().flatten() {
        let v = match v {
            Value::Null => continue,
            Value::Array(items) => items.iter().filter_map(scalar).collect::<Vec<_>>().join(","),
            v => scalar(v).ok_or_else(|| ToolError::Failed(format!("`query.{k}` must be a string, number or list")))?,
        };
        query.push(format!("{}={}", percent_encode(k), percent_encode(&v)));
    }
    if !query.is_empty() {
        path.push('?');
        path.push_str(&query.join("&"));
    }
    Ok(path)
}

fn scalar(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

async fn run(
    state: &AppState,
    role: Role,
    args: &Value,
    headers: &HeaderMap,
    base: &str,
    changes: bool,
) -> Result<Value, ToolError> {
    let d = lookup(role, args)?;
    match (d.changes(), changes) {
        (true, false) => return Err(ToolError::Failed(format!("`{}` changes something: use run_operation", d.name))),
        (false, true) => return Err(ToolError::Failed(format!("`{}` only looks: use read_operation", d.name))),
        _ => {}
    }
    let uri = address(d, args)?;
    let (body, content_type) = match &d.body {
        Some(Takes::Json { required, .. }) => match args.get("body").filter(|b| !b.is_null()) {
            Some(b) => (Body::from(serde_json::to_vec(b)?), Some("application/json")),
            None if *required => {
                return Err(ToolError::Failed(format!("`{}` takes a `body`: describe_operation says what", d.name)));
            }
            None => (Body::empty(), None),
        },
        // Refused by `lookup` (see `Described::refused`).
        Some(Takes::File | Takes::Form) => unreachable!("files aren't sent over MCP"),
        None => (Body::empty(), None),
    };
    let mut req = Request::builder().method(d.method).uri(&uri).body(body)?;
    // The same sign-in as the MCP request, and the same host (for the addresses answers give).
    for (k, v) in headers {
        let skip = matches!(
            *k,
            header::CONTENT_TYPE
                | header::CONTENT_LENGTH
                | header::ACCEPT
                | header::ACCEPT_ENCODING
                | header::TRANSFER_ENCODING
        ) || k.as_str().starts_with("mcp-");
        if !skip {
            req.headers_mut().append(k, v.clone());
        }
    }
    if let Some(t) = content_type {
        req.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static(t));
    }
    req.headers_mut().insert(header::ACCEPT, HeaderValue::from_static("application/json"));

    let resp = OVER_MCP.scope((), router(state).oneshot(req)).await.unwrap_or_else(|never| match never {});
    answer(resp, base, &uri).await
}

async fn answer(resp: http::Response<Body>, base: &str, uri: &str) -> Result<Value, ToolError> {
    let status = resp.status();
    let kind = resp.headers().get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or_default().to_owned();
    let readable = kind.is_empty() || kind.contains("json") || kind.starts_with("text/");
    if status.is_success() && !readable {
        // A file: where to get it, rather than its bytes.
        return Ok(json!({ "status": status.as_u16(), "content_type": kind, "url": format!("{base}{uri}") }));
    }
    let bytes = axum::body::to_bytes(resp.into_body(), MOST_READ).await?;
    let value: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).trim().to_owned()));
    if !status.is_success() {
        let why = value.get("error").and_then(Value::as_str).map_or_else(|| value.to_string(), str::to_owned);
        return Err(ToolError::Failed(format!("HTTP {}: {why}", status.as_u16())));
    }
    let mut value = match value {
        _ if bytes.is_empty() => json!({ "done": true }),
        Value::Object(_) => value,
        other => json!({ "result": other }),
    };
    if status != StatusCode::OK
        && let Some(o) = value.as_object_mut()
    {
        o.insert("_status".into(), json!(status.as_u16()));
    }
    shorten(&mut value);
    Ok(value)
}

/// Cuts the longest lists in an answer short until it's at most [`MOST_ANSWER`], saying so.
fn shorten(value: &mut Value) {
    let size = |v: &Value| serde_json::to_vec(v).map_or(0, |b| b.len());
    let mut cut = Vec::new();
    while size(value) > MOST_ANSWER {
        let Some((key, longest)) = longest_list(value) else { break };
        if longest.len() <= 1 {
            break;
        }
        let keep = longest.len() / 2;
        let was = longest.len();
        longest.truncate(keep);
        cut.push(format!("`{key}` cut to {keep} of {was}"));
    }
    if !cut.is_empty()
        && let Some(o) = value.as_object_mut()
    {
        o.insert("_shortened".into(), json!(format!("{}: ask for less (a filter, or a limit)", cut.join(", "))));
    }
}

/// The longest list in an answer, at its top or one level down, and what it's called.
fn longest_list(value: &mut Value) -> Option<(String, &mut Vec<Value>)> {
    let weight = |l: &Vec<Value>| serde_json::to_vec(l).map_or(0, |b| b.len());
    let Value::Object(o) = value else { return None };
    o.iter_mut()
        .flat_map(|(k, v)| -> Vec<(String, &mut Vec<Value>)> {
            match v {
                Value::Array(a) => vec![(k.clone(), a)],
                Value::Object(inner) => inner
                    .iter_mut()
                    .filter_map(|(k2, v2)| match v2 {
                        Value::Array(a) => Some((format!("{k}.{k2}"), a)),
                        _ => None,
                    })
                    .collect(),
                _ => Vec::new(),
            }
        })
        .max_by_key(|(_, list)| weight(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations_are_found_by_what_they_do() {
        let found = find(Role::Edit, &json!({ "words": "tag parents" }));
        assert_eq!(found["operations"][0]["name"], "set_tag_parents", "{found:#}");
        let found = find(Role::Edit, &json!({ "words": "rename a pack" }));
        assert!(found["operations"].as_array().unwrap().iter().any(|o| o["name"] == "change_pack"), "{found:#}");
        let groups = find(Role::Edit, &json!({}));
        assert!(groups["groups"].as_array().unwrap().iter().any(|g| g["group"] == "tags"), "{groups:#}");
        let tags = find(Role::Edit, &json!({ "group": "tags" }));
        assert!(tags["operations"].as_array().unwrap().iter().any(|o| o["name"] == "merge_tags"), "{tags:#}");
    }

    #[test]
    fn what_a_sign_in_cant_call_isnt_offered() {
        let names = |role| offered(role).map(|d| d.name).collect::<Vec<_>>();
        assert!(names(Role::View).contains(&"get_tag_tree"));
        assert!(!names(Role::View).contains(&"set_tag_parents"));
        assert!(names(Role::Edit).contains(&"set_tag_parents"));
        assert!(!names(Role::Edit).contains(&"change_settings"));
        assert!(names(Role::Admin).contains(&"change_settings"));
        // What's for a person to do themselves isn't, whoever asks.
        assert!(!names(Role::Admin).contains(&"create_token"));
        assert!(lookup(Role::Admin, &json!({ "name": "create_token" })).is_err());
        assert!(lookup(Role::View, &json!({ "name": "merge_tags" })).is_err());
    }

    #[test]
    fn files_arent_sent_over_mcp() {
        let names = offered(Role::Admin).map(|d| d.name).collect::<Vec<_>>();
        for name in ["send_upload_chunk", "send_upload_batch", "upload_pack", "set_asset_thumbnail"] {
            assert!(!names.contains(&name), "{name}");
            let Err(ToolError::Failed(why)) = lookup(Role::Admin, &json!({ "name": name })) else { panic!("{name}") };
            assert!(why.contains("create_upload"), "{why}");
        }
        // What's needed to send them another way still is.
        for name in ["create_upload", "get_upload", "complete_upload"] {
            assert!(names.contains(&name), "{name}");
        }
    }

    #[test]
    fn an_address_has_its_parameters_filled_in() {
        let file = catalog::find("get_pack_file").unwrap();
        let args = json!({ "path": { "id": "p 1", "path": "Textures/wood grain.png" }, "query": { "x": [1, 2] } });
        assert_eq!(address(file, &args).unwrap(), "/api/packs/p%201/files/Textures/wood%20grain.png?x=1%2C2");
        let asset = catalog::find("get_asset").unwrap();
        assert_eq!(address(asset, &json!({ "id": "abc" })).unwrap(), "/api/assets/abc");
        assert!(address(asset, &json!({})).is_err());
    }

    #[test]
    fn a_long_answer_is_cut_short() {
        let mut v = json!({ "items": (0..100_000).collect::<Vec<_>>(), "total": 100_000 });
        shorten(&mut v);
        assert!(serde_json::to_vec(&v).unwrap().len() <= MOST_ANSWER);
        assert_eq!(v["total"], 100_000);
        assert!(v["_shortened"].as_str().unwrap().contains("items"));
    }
}
