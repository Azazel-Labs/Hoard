//! The local model server: llama.cpp's `llama-server` in router mode (the `llm` service in the compose files, or
//! `cargo xtask llm`). It starts with no model; Settings asks it to download one from Hugging Face, lists what it
//! has, and deletes what you don't want, all through its own API. Models are named by their Hugging Face reference
//! (`unsloth/Qwen3.5-2B-GGUF:Q4_K_M`), and the vision projector comes along with the model.

use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::{Value, json};

/// Models Hoard is tuned for: all take images, run on a CPU, and fit in a few gigabytes.
#[derive(Debug, Serialize)]
pub(crate) struct Suggested {
    /// The Hugging Face reference the server downloads and names the model by.
    pub(crate) id: &'static str,
    pub(crate) name: &'static str,
    /// Model plus projector, roughly.
    pub(crate) size_gb: f64,
    pub(crate) license: &'static str,
    pub(crate) note: &'static str,
}

pub(crate) const SUGGESTED: &[Suggested] = &[
    Suggested {
        id: "unsloth/Qwen3.5-2B-GGUF:Q4_K_M",
        name: "Qwen3.5 2B",
        size_gb: 1.9,
        license: "Apache-2.0",
        note: "The default. Good at following the answer format; about 3 GB of RAM.",
    },
    Suggested {
        id: "unsloth/Qwen3.5-4B-GGUF:Q4_K_M",
        name: "Qwen3.5 4B",
        size_gb: 3.4,
        license: "Apache-2.0",
        note: "Noticeably better names and descriptions; wants 6 GB of RAM.",
    },
    Suggested {
        id: "Qwen/Qwen3-VL-2B-Instruct-GGUF:Q4_K_M",
        name: "Qwen3-VL 2B",
        size_gb: 1.9,
        license: "Apache-2.0",
        note: "Qwen's own build of the previous generation.",
    },
    Suggested {
        id: "unsloth/gemma-4-E2B-it-GGUF:Q4_K_M",
        name: "Gemma 4 E2B",
        size_gb: 3.5,
        license: "Apache-2.0",
        note: "Google's edge model; strong captions, larger download for its speed.",
    },
    Suggested {
        id: "LiquidAI/LFM2.5-VL-1.6B-GGUF:Q4_0",
        name: "LFM2.5-VL 1.6B",
        size_gb: 1.3,
        license: "LFM Open License 1.0",
        note: "Smallest and fastest; fine for tags, weaker at descriptions. Check the license for commercial use.",
    },
];

/// A model the server has, or is downloading.
#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct LocalModel {
    pub(crate) id: String,
    /// `downloading`, `incomplete` (a download that stopped), `unloaded` (ready), `loading`, `loaded`, `sleeping`.
    pub(crate) status: String,
    /// Bytes downloaded and expected, while downloading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) progress: Option<(u64, u64)>,
    pub(crate) vision: bool,
    /// The last attempt to load it failed.
    pub(crate) failed: bool,
}

/// The router's own address: the OpenAI-compatible one without `/v1`.
fn router(url: &str) -> String {
    let url = url.trim().trim_end_matches('/');
    url.strip_suffix("/v1").unwrap_or(url).to_owned()
}

async fn answer(resp: reqwest::Response, what: &str) -> Result<Value> {
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    let v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    if !status.is_success() {
        let msg = v["error"]["message"].as_str().map_or_else(|| text.chars().take(300).collect(), str::to_owned);
        bail!("{what}: {msg}");
    }
    Ok(v)
}

/// What the server has. Fails if nothing answers, or if it isn't a llama.cpp router (which can't download models).
pub(crate) async fn list(http: &reqwest::Client, url: &str) -> Result<Vec<LocalModel>> {
    let base = router(url);
    let resp = http
        .get(format!("{base}/models"))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .with_context(|| format!("no model server answers at {base}"))?;
    let v = answer(resp, "the model server").await?;
    let mut models = parse_list(&v)
        .with_context(|| format!("{base} isn't a llama.cpp server in router mode, so it can't download models"))?;
    let downloading = models.iter().any(|m| m.status == "downloading");
    if downloading {
        // The model list doesn't say how far a download has got; the router's event stream does.
        follow(http.clone(), base.clone());
        let w = WATCH.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for m in models.iter_mut().filter(|m| m.status == "downloading" && m.progress.is_none()) {
            m.progress = w.progress.get(&(base.clone(), m.id.clone())).copied();
        }
    }
    Ok(models)
}

/// Download progress per (router, model), from the routers' event streams, and the routers being followed.
#[derive(Default)]
struct Watch {
    following: HashSet<String>,
    progress: HashMap<(String, String), (u64, u64)>,
}
static WATCH: LazyLock<Mutex<Watch>> = LazyLock::new(Mutex::default);

/// Follow a router's event stream (`/models/sse`) in the background, noting download progress, until nothing is
/// downloading any more.
fn follow(http: reqwest::Client, base: String) {
    {
        let mut w = WATCH.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !w.following.insert(base.clone()) {
            return;
        }
    }
    tokio::spawn(async move {
        if let Err(e) = read_events(&http, &base).await {
            tracing::debug!(router = %base, "stopped following downloads: {e:#}");
        }
        let mut w = WATCH.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        w.following.remove(&base);
        w.progress.retain(|(b, _), _| *b != base);
    });
}

async fn read_events(http: &reqwest::Client, base: &str) -> Result<()> {
    let mut resp = http
        .get(format!("{base}/models/sse"))
        // Longer than the client's usual limit; a big download takes a while. Listing again picks up from here.
        .timeout(Duration::from_hours(4))
        .send()
        .await?
        .error_for_status()?;
    let mut buf = Vec::new();
    loop {
        match tokio::time::timeout(Duration::from_secs(20), resp.chunk()).await {
            Ok(Ok(Some(bytes))) => buf.extend_from_slice(&bytes),
            Ok(Ok(None)) => return Ok(()),
            Ok(Err(e)) => return Err(e.into()),
            // Quiet for a while: stop once nothing is downloading.
            Err(_) => {
                let v = answer(http.get(format!("{base}/models")).send().await?, "the model server").await?;
                if !parse_list(&v).unwrap_or_default().iter().any(|m| m.status == "downloading") {
                    return Ok(());
                }
                continue;
            }
        }
        while let Some(end) = buf.iter().position(|&b| b == b'\n') {
            let line: Vec<u8> = buf.drain(..=end).collect();
            let Some(data) = line.strip_prefix(b"data:") else { continue };
            let Ok(ev) = serde_json::from_slice::<Value>(data) else { continue };
            if note_event(base, &ev) == Some(true) {
                return Ok(());
            }
        }
    }
}

/// Record one event from the stream. `Some(true)` when no download is left.
fn note_event(base: &str, ev: &Value) -> Option<bool> {
    let model = ev["model"].as_str()?.to_owned();
    let mut w = WATCH.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    match ev["event"].as_str()? {
        "download_progress" => {
            let p = sum_files(&ev["data"]["progress"])?;
            w.progress.insert((base.to_owned(), model), p);
            Some(false)
        }
        "model_status" if ev["data"]["status"].as_str() != Some("downloading") => {
            w.progress.remove(&(base.to_owned(), model));
            Some(!w.progress.keys().any(|(b, _)| b == base))
        }
        _ => None,
    }
}

/// Bytes done and expected across the files of one download.
fn sum_files(progress: &Value) -> Option<(u64, u64)> {
    let files = progress.as_object()?;
    Some(
        files
            .values()
            .fold((0, 0), |(d, t), f| (d + f["done"].as_u64().unwrap_or(0), t + f["total"].as_u64().unwrap_or(0))),
    )
}

fn parse_list(v: &Value) -> Option<Vec<LocalModel>> {
    let data = v["data"].as_array()?;
    // A router lists every model with a status; a single-model server or another service doesn't.
    if data.iter().any(|m| m["status"]["value"].as_str().is_none()) {
        return None;
    }
    let mut out: Vec<LocalModel> = data
        .iter()
        .filter_map(|m| {
            let st = &m["status"];
            let progress = sum_files(&st["progress"]);
            let vision = m["architecture"]["input_modalities"]
                .as_array()
                .is_some_and(|mods| mods.iter().any(|x| x.as_str() == Some("image")))
                || m["sidecars"].as_array().is_some_and(|s| s.iter().any(|x| x.as_str() == Some("mmproj")));
            Some(LocalModel {
                id: m["id"].as_str()?.to_owned(),
                status: st["value"].as_str()?.to_owned(),
                progress,
                vision,
                failed: st["failed"].as_bool().unwrap_or(false),
            })
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Some(out)
}

/// Start downloading a model (and its vision projector) from Hugging Face. Returns at once; `list` shows progress.
/// Posting a stopped download again resumes it.
pub(crate) async fn download(http: &reqwest::Client, url: &str, model: &str) -> Result<()> {
    let base = router(url);
    let resp = http
        .post(format!("{base}/models"))
        .json(&json!({ "model": model }))
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .with_context(|| format!("no model server answers at {base}"))?;
    answer(resp, &format!("couldn't download {model}")).await.map(drop)
}

/// Stop a model (or its download) and delete its files.
pub(crate) async fn remove(http: &reqwest::Client, url: &str, model: &str) -> Result<()> {
    let base = router(url);
    // Unloading something that isn't running fails harmlessly; a running model can't be deleted.
    let _ = http
        .post(format!("{base}/models/unload"))
        .json(&json!({ "model": model }))
        .timeout(Duration::from_secs(30))
        .send()
        .await;
    for _ in 0..20 {
        let running = list(http, url)
            .await?
            .into_iter()
            .any(|m| m.id == model && matches!(m.status.as_str(), "loading" | "loaded" | "sleeping" | "downloading"));
        if !running {
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    let resp = http
        .delete(format!("{base}/models"))
        .query(&[("model", model)])
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .with_context(|| format!("no model server answers at {base}"))?;
    answer(resp, &format!("couldn't delete {model}")).await.map(drop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_routers_model_list() {
        let v = json!({ "data": [
            { "id": "b/B-GGUF:Q4_K_M", "status": { "value": "downloading", "progress": {
                "https://x/model.gguf": { "done": 100, "total": 1000 },
                "https://x/mmproj.gguf": { "done": 50, "total": 200 } } },
              "architecture": { "input_modalities": ["text"] }, "sidecars": ["mmproj"] },
            { "id": "a/A-GGUF:Q8_0", "status": { "value": "unloaded", "failed": true, "exit_code": 1 },
              "architecture": { "input_modalities": ["text", "image"] } },
        ] });
        let list = parse_list(&v).unwrap();
        assert_eq!(list[0].id, "a/A-GGUF:Q8_0");
        assert!(list[0].vision && list[0].failed && list[0].progress.is_none());
        assert_eq!(list[1].status, "downloading");
        assert_eq!(list[1].progress, Some((150, 1200)));
        assert!(list[1].vision, "the projector is on its way");
        // OpenAI-style lists have no status: not a router.
        assert!(parse_list(&json!({ "data": [{ "id": "gpt-x", "object": "model" }] })).is_none());
        assert_eq!(router("http://llm:8080/v1/"), "http://llm:8080");
    }

    #[test]
    fn notes_download_progress_from_the_event_stream() {
        let base = "http://test-router";
        let progress =
            |files: Value| json!({ "model": "m", "event": "download_progress", "data": { "progress": files } });
        assert_eq!(
            note_event(base, &json!({ "model": "m", "event": "model_status", "data": { "status": "downloading" } })),
            None
        );
        let ev = progress(
            json!({ "https://x/a.gguf": { "done": 10, "total": 100 }, "https://x/b.gguf": { "done": 0, "total": 0 } }),
        );
        assert_eq!(note_event(base, &ev), Some(false));
        let key = (base.to_owned(), "m".to_owned());
        assert_eq!(WATCH.lock().unwrap().progress.get(&key), Some(&(10, 100)));
        // Done: forgotten, and nothing is left to follow.
        assert_eq!(
            note_event(base, &json!({ "model": "m", "event": "model_status", "data": { "status": "unloaded" } })),
            Some(true)
        );
        assert!(!WATCH.lock().unwrap().progress.contains_key(&key));
    }
}
