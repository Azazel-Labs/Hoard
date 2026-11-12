//! Optional LLM tagging pass over a pack, via any OpenAI-compatible `/chat/completions` endpoint:
//! `OpenAI`, `OpenRouter`, Ollama, llama.cpp server, vLLM, LM Studio, … Point it at Ollama for a fully
//! offline setup, or at a hosted model for better results.
//!
//! The model sees the pack's context (store page text, README), paths, metadata and the rule tags (plus a numbered
//! contact sheet of thumbnails when vision is on) and returns nicer names, a category, a short description and
//! extra tags. Its tags are stored with `source = ai` and lower confidence; it never removes or overrides rule or
//! human tags. Variant families (`whoosh_01..12`) are sent once and the answer is shared by every member.

use std::fmt::Write as _;

use anyhow::{Context, Result, bail};
use base64::Engine as _;
use hoard_core::{PackStatus, TagSource};
use rusqlite::params;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::db;
use crate::tagger::CATEGORIES;

const LLM_CONFIDENCE: f32 = 0.7;
/// Tiles per contact sheet (4x4 at 192 px is a 784 px image).
const MAX_IMAGES_PER_BATCH: usize = 16;
const SHEET_CELL: u32 = 192;
const SHEET_COLS: u32 = 4;
/// Pack context included per batch, in characters.
const CONTEXT_CHARS: usize = 2500;

#[derive(Debug)]
struct Row {
    id: String,
    path: String,
    kind: String,
    meta: Value,
    tags: Vec<String>,
    thumb: Option<String>,
    /// Multi-view sheet of a model, preferred over the thumbnail for vision.
    views: Option<String>,
    human_edited: bool,
    family: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct Reply {
    #[serde(default)]
    pack: Option<PackReply>,
    #[serde(default)]
    assets: Vec<AssetReply>,
}

#[derive(Debug, Deserialize)]
struct PackReply {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AssetReply {
    i: usize,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

fn system_prompt() -> String {
    format!(
        "You catalogue game-development assets (textures, sprites, audio, music, 3D models, fonts, shaders) for a \
         searchable asset library. For each numbered file you get its path inside the pack, its kind, measured \
         metadata and tags an offline tagger already assigned. Reply with JSON only:\n\
         {{\"pack\": {{\"title\": str, \"description\": str, \"tags\": [str]}} | null,\n \
         \"assets\": [{{\"i\": int, \"name\": str, \"category\": str, \"description\": str, \"tags\": [str]}}]}}\n\
         Rules:\n\
         - name: short human title, e.g. \"Footstep on gravel 3\". No file extensions.\n\
         - category: exactly one of: {}.\n\
         - description: 1-3 sentences: what it is, what it looks or sounds like, and how a game might use it. \
           Music: mood, instruments, tempo and where it fits (menu, combat, exploration, ambience). Add detail \
           only where the path, metadata, context or image supports it; one plain sentence is fine otherwise.\n\
         - tags: 3-8 lowercase kebab-case tags people would search for. Prefer these namespaces when they fit: \
           style:, theme:, mood:, genre:, material:, subject words (e.g. footsteps, sword, tree). Don't repeat \
           format/resolution facts; those are known.\n\
         - Sound effects: add one ucs:<category>/<subcategory> tag from the Universal Category System, lowercase \
           kebab-case, e.g. ucs:footsteps/gravel, ucs:magic/elemental, ucs:user-interface/click. Skip if unsure.\n\
         - A file marked \"+N variants\" stands for a family of near-identical takes; describe the family.\n\
         - Pack context (store page, README) explains what the files are for; use it, but trust the file over it.\n\
         - Only describe what the path, metadata, context or image supports. If unsure, keep it generic.\n\
         - Images: a contact sheet; each tile's white number is its file number. Model tiles show four views.\n\
         - Include `pack` only when asked; otherwise null.",
        CATEGORIES.join(", ")
    )
}

fn summarize_meta(meta: &Value) -> String {
    let keys = [
        "width",
        "height",
        "alpha",
        "tileable",
        "duration_secs",
        "channels",
        "sample_rate",
        "title",
        "artist",
        "genre",
        "triangles",
        "skins",
        "animations",
        "has_animation",
        "has_deformers",
        "color",
        "description",
        "keywords",
        "ucs_category",
        "ucs_subcategory",
        "library",
        "tempo_bpm",
        "one_shot",
        "loop_start",
        "ue_class",
        "ue_engine",
    ];
    let mut out = String::new();
    for k in keys {
        if let Some(v) = meta.get(k).filter(|v| !v.is_null()) {
            let v = v.to_string();
            let v: String = v.trim_matches('"').chars().take(160).collect();
            let _ = write!(out, "{k}={v} ");
        }
    }
    out.trim_end().to_owned()
}

fn load_rows(conn: &rusqlite::Connection, pack_id: &str) -> Result<Vec<Row>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.path, a.kind, a.meta, a.thumb_hash, a.human_edited,
                COALESCE((SELECT group_concat(tag, ' ') FROM asset_tags t
                          WHERE t.asset_id = a.id AND t.source != 'meta'), ''),
                (SELECT family_key FROM asset_family f WHERE f.asset_id = a.id),
                (SELECT blob_hash FROM derivatives d WHERE d.asset_id = a.id AND d.kind = 'views')
         FROM assets a WHERE a.pack_id = ?1 ORDER BY a.path",
    )?;
    let rows = stmt
        .query_map([pack_id], |r| {
            let meta: String = r.get(3)?;
            let tags: String = r.get(6)?;
            Ok(Row {
                id: r.get(0)?,
                path: r.get(1)?,
                kind: r.get(2)?,
                meta: serde_json::from_str(&meta).unwrap_or_default(),
                thumb: r.get(4)?,
                human_edited: r.get::<_, i64>(5)? != 0,
                tags: tags.split_whitespace().map(str::to_owned).collect(),
                family: r.get(7)?,
                views: r.get(8)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

/// One model an AI service offers, for the Settings dialog's model list.
#[derive(Debug, serde::Serialize)]
pub(crate) struct ModelInfo {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    /// Whether it takes images, when the service says (`OpenRouter` does).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vision: Option<bool>,
}

/// Ask an OpenAI-compatible service which models it has (`GET {base}/models`). Also how Settings tests a connection.
pub(crate) async fn list_models(http: &reqwest::Client, base: &str, key: Option<&str>) -> Result<Vec<ModelInfo>> {
    let base = base.trim().trim_end_matches('/');
    let mut req = http.get(format!("{base}/models")).timeout(std::time::Duration::from_secs(20));
    if let Some(key) = key {
        req = req.bearer_auth(key);
        // Anthropic's own model list wants its key header; its chat endpoint takes the bearer.
        if base.contains("api.anthropic.com") {
            req = req.header("x-api-key", key).header("anthropic-version", "2023-06-01");
        }
    }
    let resp = req.send().await.with_context(|| format!("couldn't reach {base}"))?;
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        bail!("{base} refused the API key ({status})");
    }
    if !status.is_success() {
        bail!("{base} answered {status}: {}", text.chars().take(300).collect::<String>());
    }
    let v: Value = serde_json::from_str(&text).with_context(|| format!("{base}/models didn't answer with JSON"))?;
    Ok(parse_models(&v))
}

fn parse_models(v: &Value) -> Vec<ModelInfo> {
    // OpenAI and most others: {"data": [...]}; Ollama's native list: {"models": [...]}.
    let list = v["data"].as_array().or_else(|| v["models"].as_array()).cloned().unwrap_or_default();
    let mut out: Vec<ModelInfo> = list
        .iter()
        .filter_map(|m| {
            let id = m["id"].as_str().or_else(|| m["name"].as_str())?;
            // Gemini lists "models/gemini-…" but takes the bare name.
            let id = id.strip_prefix("models/").unwrap_or(id).to_owned();
            let name =
                m["display_name"].as_str().or_else(|| m["name"].as_str()).filter(|n| *n != id).map(str::to_owned);
            let vision = m["architecture"]["input_modalities"]
                .as_array()
                .map(|mods| mods.iter().any(|x| x.as_str() == Some("image")));
            Some(ModelInfo { id, name, vision })
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out.dedup_by(|a, b| a.id == b.id);
    out
}

async fn chat(state: &AppState, messages: Value) -> Result<Reply> {
    let cfg = state.settings.get();
    let base = cfg.llm_url.as_deref().context("LLM not configured")?.trim_end_matches('/');
    let url = format!("{base}/chat/completions");
    let mut body = json!({
        "model": cfg.llm_model.as_deref().context("no LLM model is set")?,
        "messages": messages,
        "temperature": 0.2,
        "response_format": { "type": "json_object" },
    });
    for attempt in 0..2 {
        let mut req = state.http.post(&url).json(&body);
        if let Some(key) = &cfg.llm_api_key {
            req = req.bearer_auth(key);
        }
        let resp = req.send().await.context("LLM request failed")?;
        let status = resp.status();
        let text = resp.text().await?;
        if status == reqwest::StatusCode::BAD_REQUEST && attempt == 0 && text.contains("response_format") {
            // Some servers don't support JSON mode; the prompt already demands JSON.
            if let Some(o) = body.as_object_mut() {
                o.remove("response_format");
            }
            continue;
        }
        if !status.is_success() {
            bail!("LLM returned {status}: {}", text.chars().take(500).collect::<String>());
        }
        let v: Value = serde_json::from_str(&text).context("LLM response is not JSON")?;
        let content = v["choices"][0]["message"]["content"].as_str().context("LLM response has no message content")?;
        return parse_reply(content);
    }
    bail!("LLM request failed")
}

/// Models sometimes wrap JSON in prose or code fences; take the outermost object.
fn parse_reply(content: &str) -> Result<Reply> {
    let start = content.find('{').context("no JSON object in LLM reply")?;
    let end = content.rfind('}').context("no JSON object in LLM reply")?;
    Ok(serde_json::from_str(&content[start..=end])?)
}

/// One entry the model sees: a file, or the representative of a variant family.
struct Entry<'a> {
    row: &'a Row,
    /// Every asset the answer applies to (the representative first), with its `human_edited` flag.
    targets: Vec<(String, bool)>,
}

/// Collapse variant families to their first member.
fn entries(rows: &[Row]) -> Vec<Entry<'_>> {
    let mut out: Vec<Entry<'_>> = Vec::new();
    let mut family_at: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for r in rows {
        if let Some(f) = r.family.as_deref() {
            if let Some(&i) = family_at.get(f) {
                out[i].targets.push((r.id.clone(), r.human_edited));
                continue;
            }
            family_at.insert(f, out.len());
        }
        out.push(Entry { row: r, targets: vec![(r.id.clone(), r.human_edited)] });
    }
    out
}

fn context_block(ctx: &hoard_core::PackContext) -> String {
    let mut out = String::new();
    if let Some(s) = &ctx.store {
        let _ = writeln!(
            out,
            "Store: {s}{}",
            ctx.license_tier.as_ref().map(|t| format!(" ({t} license)")).unwrap_or_default()
        );
    }
    let mut left = CONTEXT_CHARS;
    for (label, text) in [("Store page", &ctx.store_text), ("README", &ctx.readme)] {
        if let Some(t) = text.as_deref().filter(|_| left > 200) {
            let t: String = t.chars().take(left).collect();
            left = left.saturating_sub(t.chars().count());
            let _ = writeln!(out, "{label}:\n{}", t.trim());
        }
    }
    out
}

/// A numbered contact sheet of the batch's thumbnails (multi-view sheets for models), as a PNG data URL.
async fn batch_sheet(state: &AppState, batch: &[Entry<'_>]) -> Option<String> {
    let mut tiles = Vec::new();
    for (i, e) in batch.iter().enumerate() {
        let Some(hash) = e.row.views.as_ref().or(e.row.thumb.as_ref()) else { continue };
        let Ok(bytes) = state.blobs.read_all(hash).await else { continue };
        let Ok(img) = image::load_from_memory(&bytes) else { continue };
        tiles.push((i, img.to_rgba8()));
        if tiles.len() == MAX_IMAGES_PER_BATCH {
            break;
        }
    }
    if tiles.is_empty() {
        return None;
    }
    let sheet = crate::heavy::run("contact-sheet", move || {
        crate::analyze::png_bytes(crate::sheet::contact_sheet(&tiles, SHEET_CELL, SHEET_COLS))
    })
    .await
    .ok()?
    .ok()?;
    Some(format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(&sheet)))
}

pub(crate) async fn tag_pack(state: &AppState, pack_id: &str) -> Result<()> {
    crate::context::fill_store_text(state, pack_id).await;
    let pid = pack_id.to_owned();
    let (pack, ctx, rows) = db::call(&state.pool, move |c| {
        let pack = db::get_pack(c, &pid)?.context("pack not found")?;
        Ok((pack, db::pack_context(c, &pid)?, load_rows(c, &pid)?))
    })
    .await?;
    let pack_ctx = context_block(&ctx);
    let entries = entries(&rows);

    let batch_size = state.settings.get().llm_batch;
    let mut done = 0;
    for (batch_no, batch) in entries.chunks(batch_size).enumerate() {
        crate::activity::job_progress(
            crate::jobs::LLM_TAG,
            Some(pack_id),
            batch_no * batch_size,
            entries.len(),
            "assets",
        );
        let ask_pack = batch_no == 0;
        let mut listing = format!(
            "Pack: {}\nLicense: {}\n{pack_ctx}{}\nFiles:\n",
            pack.name,
            pack.license.as_deref().unwrap_or("unknown"),
            if ask_pack { "Also fill in `pack`." } else { "Set `pack` to null." }
        );
        for (i, e) in batch.iter().enumerate() {
            let r = e.row;
            let variants = match e.targets.len() {
                1 => String::new(),
                n => format!(" (+{} variants)", n - 1),
            };
            let _ = writeln!(
                listing,
                "{i} | {}{variants} | {} | {} | tags: {}",
                r.path,
                r.kind,
                summarize_meta(&r.meta),
                r.tags.join(" ")
            );
        }

        let mut content = vec![json!({ "type": "text", "text": listing })];
        if state.settings.get().llm_vision
            && let Some(url) = batch_sheet(state, batch).await
        {
            content.push(json!({ "type": "image_url", "image_url": { "url": url } }));
        }
        let messages = json!([
            { "role": "system", "content": system_prompt() },
            { "role": "user", "content": content },
        ]);

        let reply = chat(state, messages).await?;
        let targets: Vec<Vec<(String, bool)>> = batch.iter().map(|e| e.targets.clone()).collect();
        done += targets.iter().map(Vec::len).sum::<usize>() as i64;
        let pid = pack_id.to_owned();
        db::call(&state.pool, move |c| {
            apply_reply(c, &pid, &targets, reply, ask_pack)?;
            c.execute("UPDATE packs SET files_done = ?2 WHERE id = ?1", params![pid, done])?;
            Ok(())
        })
        .await?;
    }

    let pid = pack_id.to_owned();
    db::call(&state.pool, move |c| db::set_pack_status(c, &pid, PackStatus::Ready, None)).await
}

fn apply_reply(
    c: &mut db::Conn,
    pack_id: &str,
    targets: &[Vec<(String, bool)>],
    reply: Reply,
    ask_pack: bool,
) -> Result<()> {
    let now = db::now();
    let tx = c.transaction()?;
    if ask_pack && let Some(p) = reply.pack {
        let tags: Vec<String> = p.tags.iter().filter_map(|t| hoard_core::normalize_tag(t)).collect();
        tx.execute(
            "UPDATE packs SET description = COALESCE(?2, description), tags = ?3 WHERE id = ?1",
            params![pack_id, p.description.or(p.title.map(|t| format!("{t}."))), serde_json::to_string(&tags)?],
        )?;
    }
    for a in reply.assets {
        let Some(members) = targets.get(a.i) else { continue };
        let category = a.category.filter(|c| CATEGORIES.contains(&c.as_str()));
        let description =
            a.description.map(|d| d.trim().chars().take(800).collect::<String>()).filter(|d| !d.is_empty());
        let name = a.name.map(|n| n.chars().take(120).collect::<String>());
        let tags: Vec<String> = a.tags.iter().take(12).filter_map(|t| hoard_core::normalize_tag(t)).collect();
        for (n, (id, human_edited)) in members.iter().enumerate() {
            if !human_edited {
                tx.execute(
                    // Category: only take the LLM's answer if it's at least as specific as what we have,
                    // so "sfx/footsteps" from the filename doesn't get flattened to "sfx".
                    // Names go to the representative only: variants keep their own (numbered) names.
                    "UPDATE assets SET name = COALESCE(?2, name),
                            category = CASE
                                WHEN ?3 IS NULL THEN category
                                WHEN category IS NULL OR instr(category, '/') = 0 OR instr(?3, '/') > 0 THEN ?3
                                ELSE category END,
                            updated_at = ?4
                     WHERE id = ?1",
                    params![id, name.as_ref().filter(|_| n == 0), category, now],
                )?;
            }
            // The AI's description has its own field, so it stays current even after a person edits the asset.
            if description.is_some() {
                tx.execute(
                    "UPDATE assets SET ai_description = ?2, updated_at = ?3 WHERE id = ?1",
                    params![id, description, now],
                )?;
            }
            for t in &tags {
                db::upsert_tag(&tx, id, t, TagSource::Ai, LLM_CONFIDENCE)?;
            }
            db::reindex_asset(&tx, id)?;
        }
    }
    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_model_lists_from_the_usual_services() {
        let openrouter = json!({ "data": [
            { "id": "z/text", "name": "Z Text", "architecture": { "input_modalities": ["text"] } },
            { "id": "a/vision", "name": "A Vision", "architecture": { "input_modalities": ["text", "image"] } },
        ]});
        let m = parse_models(&openrouter);
        assert_eq!(m.iter().map(|m| m.id.as_str()).collect::<Vec<_>>(), ["a/vision", "z/text"]);
        assert_eq!(m[0].vision, Some(true));
        assert_eq!(m[1].vision, Some(false));
        assert_eq!(m[0].name.as_deref(), Some("A Vision"));

        let gemini = json!({ "data": [{ "id": "models/gemini-2.5-flash", "object": "model" }] });
        assert_eq!(parse_models(&gemini)[0].id, "gemini-2.5-flash");

        let anthropic = json!({ "data": [{ "id": "claude-x", "display_name": "Claude X" }] });
        assert_eq!(parse_models(&anthropic)[0].name.as_deref(), Some("Claude X"));

        let ollama = json!({ "models": [{ "name": "qwen3:8b" }] });
        let m = parse_models(&ollama);
        assert_eq!((m[0].id.as_str(), m[0].name.as_deref(), m[0].vision), ("qwen3:8b", None, None));
    }

    #[test]
    fn parses_fenced_json() {
        let r = parse_reply("Sure!\n```json\n{\"pack\": null, \"assets\": [{\"i\": 0, \"tags\": [\"Grass\"]}]}\n```")
            .unwrap();
        assert_eq!(r.assets.len(), 1);
        assert_eq!(r.assets[0].tags, ["Grass"]);
    }
}
