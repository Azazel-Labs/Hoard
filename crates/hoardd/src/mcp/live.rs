//! Which apps are using the MCP server right now, for Settings → Account. The server keeps no sessions, so "connected"
//! means "asked something lately": each request is noted here, in memory, by who sent it, and forgotten once that app
//! has been quiet for a while.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use axum::Json;
use axum::extract::{Extension, State};
use serde_json::{Value, json};

use crate::auth::Who;
use crate::error::ApiResult;
use crate::{AppState, db};

/// How long an app counts as using Hoard after its last request.
pub(crate) const QUIET: Duration = Duration::from_mins(10);

#[derive(Clone)]
struct Seen {
    owner: Option<i64>,
    name: String,
    client: Option<String>,
    last: Instant,
    what: String,
    calls: u64,
}

static LIVE: Mutex<Vec<Seen>> = Mutex::new(Vec::new());

/// Note a request from `who`: the method, or for a tool call the tool's name.
pub(super) fn record(who: &Who, method: &str, params: &Value) {
    let (owner, name) = match who {
        Who::User { id, name, .. } => (Some(*id), name.clone()),
        Who::Token { name, owner, .. } => (*owner, name.clone()),
    };
    let what = match method {
        "tools/call" | "prompts/get" => params.get("name").and_then(Value::as_str).unwrap_or(method),
        _ => method,
    };
    let what: String = what.chars().take(64).collect();
    // Older clients name themselves on `initialize`; newer ones may send it in `_meta` on every request.
    let info = params
        .get("clientInfo")
        .or_else(|| params.get("_meta").and_then(|m| m.get("io.modelcontextprotocol/clientInfo")));
    let client = info.and_then(|i| {
        let name = i.get("title").or_else(|| i.get("name")).and_then(Value::as_str)?;
        let version = i.get("version").and_then(Value::as_str).unwrap_or_default();
        Some(format!("{name} {version}").trim().chars().take(80).collect::<String>())
    });
    let now = Instant::now();
    let Ok(mut live) = LIVE.lock() else { return };
    live.retain(|s| now.duration_since(s.last) < QUIET);
    if let Some(s) = live.iter_mut().find(|s| s.owner == owner && s.name == name) {
        s.last = now;
        s.what = what;
        s.calls += 1;
        if client.is_some() {
            s.client = client;
        }
    } else {
        live.push(Seen { owner, name, client, last: now, what, calls: 1 });
    }
}

/// `GET /api/mcp/live`: the apps that asked something in the last [`QUIET`], most recent first. Everyone sees their
/// own; admins see everyone's.
pub(crate) async fn list(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let only = crate::auth::owner_filter(&who)?;
    let now = Instant::now();
    let mut seen: Vec<Seen> = LIVE.lock().map_or_else(
        |_| Vec::new(),
        |live| {
            live.iter()
                .filter(|s| now.duration_since(s.last) < QUIET && (only.is_none() || s.owner == only))
                .cloned()
                .collect()
        },
    );
    seen.sort_by_key(|s| std::cmp::Reverse(s.last));
    let owners: Vec<i64> = seen.iter().filter_map(|s| s.owner).collect();
    let names: HashMap<i64, String> = if owners.is_empty() {
        HashMap::new()
    } else {
        db::read(&state.pool, move |c| {
            let mut stmt = c.prepare("SELECT id, username FROM users")?;
            let mut rows = stmt
                .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))?
                .collect::<rusqlite::Result<HashMap<_, _>>>()?;
            rows.retain(|id, _| owners.contains(id));
            Ok(rows)
        })
        .await?
    };
    let live: Vec<Value> = seen
        .into_iter()
        .map(|s| {
            json!({
                "name": s.name,
                "user": s.owner.and_then(|o| names.get(&o).cloned()),
                "client": s.client,
                "last": s.what,
                "seconds_ago": now.duration_since(s.last).as_secs(),
                "calls": s.calls,
            })
        })
        .collect();
    Ok(Json(json!({ "live": live })))
}
