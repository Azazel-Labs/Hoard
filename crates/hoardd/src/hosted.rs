//! Image search through a hosted multimodal embedding service (Voyage AI, Cohere or Jina AI) instead of the model on
//! this server, chosen in Settings. Every thumbnail, and every search's words, are sent to the provider, which sends
//! back their embeddings; nothing is downloaded and nothing runs here but the index (see `clip`).

use std::sync::{Mutex, PoisonError};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use base64::Engine;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Provider {
    Voyage,
    Cohere,
    Jina,
}

impl Provider {
    pub(crate) const ALL: [Self; 3] = [Self::Voyage, Self::Cohere, Self::Jina];

    pub(crate) fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.id() == s)
    }

    pub(crate) const fn id(self) -> &'static str {
        match self {
            Self::Voyage => "voyage",
            Self::Cohere => "cohere",
            Self::Jina => "jina",
        }
    }

    /// What the provider calls itself.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Voyage => "Voyage AI",
            Self::Cohere => "Cohere",
            Self::Jina => "Jina AI",
        }
    }

    /// The model used when none is given.
    pub(crate) const fn default_model(self) -> &'static str {
        match self {
            Self::Voyage => "voyage-multimodal-3.5",
            Self::Cohere => "embed-v4.0",
            Self::Jina => "jina-clip-v2",
        }
    }

    pub(crate) const fn base(self) -> &'static str {
        match self {
            Self::Voyage => "https://api.voyageai.com/v1",
            Self::Cohere => "https://api.cohere.com/v2",
            Self::Jina => "https://api.jina.ai/v1",
        }
    }

    const fn path(self) -> &'static str {
        match self {
            Self::Voyage => "/multimodalembeddings",
            Self::Cohere => "/embed",
            Self::Jina => "/embeddings",
        }
    }
}

/// Where image search runs when it's hosted.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Hosted {
    pub(crate) provider: Provider,
    pub(crate) model: String,
    pub(crate) key: String,
    /// In place of the provider's own address (`…/v1`), for a gateway in between.
    pub(crate) url: Option<String>,
}

/// Without the key, so it never reaches a log.
impl std::fmt::Debug for Hosted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hosted")
            .field("provider", &self.provider)
            .field("model", &self.model)
            .field("url", &self.url)
            .finish_non_exhaustive()
    }
}

impl Hosted {
    /// What its embeddings are stored as, so another provider or model never mixes with them.
    pub(crate) fn id(&self) -> String {
        format!("{}:{}", self.provider.id(), self.model)
    }

    fn endpoint(&self) -> String {
        let base = self.url.as_deref().unwrap_or(self.provider.base()).trim_end_matches('/');
        format!("{base}{}", self.provider.path())
    }
}

/// The picture formats every provider takes, by their first bytes.
fn mime(bytes: &[u8]) -> Option<&'static str> {
    match bytes {
        [0x89, b'P', b'N', b'G', ..] => Some("image/png"),
        [0xFF, 0xD8, 0xFF, ..] => Some("image/jpeg"),
        [b'R', b'I', b'F', b'F', _, _, _, _, b'W', b'E', b'B', b'P', ..] => Some("image/webp"),
        [b'G', b'I', b'F', b'8', ..] => Some("image/gif"),
        _ => None,
    }
}

fn data_uri(mime: &str, bytes: &[u8]) -> String {
    format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))
}

/// What's sent in one request: pictures to be found, or the words of a search.
enum Batch<'a> {
    Pictures(&'a [String]),
    Words(&'a str),
}

fn body(h: &Hosted, batch: &Batch) -> Value {
    match (h.provider, batch) {
        (Provider::Voyage, Batch::Pictures(uris)) => json!({
            "model": h.model,
            "input_type": "document",
            "inputs": uris.iter().map(|u| json!({ "content": [{ "type": "image_base64", "image_base64": u }] })).collect::<Vec<_>>(),
        }),
        (Provider::Voyage, Batch::Words(text)) => json!({
            "model": h.model,
            "input_type": "query",
            "inputs": [{ "content": [{ "type": "text", "text": text }] }],
        }),
        (Provider::Cohere, Batch::Pictures(uris)) => json!({
            "model": h.model,
            "input_type": "search_document",
            "embedding_types": ["float"],
            "images": uris,
        }),
        (Provider::Cohere, Batch::Words(text)) => json!({
            "model": h.model,
            "input_type": "search_query",
            "embedding_types": ["float"],
            "texts": [text],
        }),
        (Provider::Jina, Batch::Pictures(uris)) => json!({
            "model": h.model,
            "normalized": true,
            "input": uris.iter().map(|u| json!({ "image": u })).collect::<Vec<_>>(),
        }),
        (Provider::Jina, Batch::Words(text)) => json!({
            "model": h.model,
            "normalized": true,
            "input": [{ "text": text }],
        }),
    }
}

#[derive(Deserialize)]
struct Indexed {
    embedding: Vec<f32>,
    #[serde(default)]
    index: Option<usize>,
}

/// The embeddings in a provider's answer, in the order they were asked for.
fn parse(provider: Provider, v: Value, expected: usize) -> Result<Vec<Vec<f32>>> {
    let out = match provider {
        Provider::Voyage | Provider::Jina => {
            #[derive(Deserialize)]
            struct Answer {
                data: Vec<Indexed>,
            }
            let mut data = serde_json::from_value::<Answer>(v).context("unexpected answer")?.data;
            data.sort_by_key(|d| d.index);
            data.into_iter().map(|d| d.embedding).collect::<Vec<_>>()
        }
        Provider::Cohere => {
            #[derive(Deserialize)]
            struct Floats {
                float: Vec<Vec<f32>>,
            }
            #[derive(Deserialize)]
            struct Answer {
                embeddings: Floats,
            }
            serde_json::from_value::<Answer>(v).context("unexpected answer")?.embeddings.float
        }
    };
    if out.len() != expected || out.iter().any(Vec::is_empty) {
        bail!("asked for {expected} embeddings and got {}", out.len());
    }
    Ok(out)
}

/// A refusal that's about what was sent (one picture it couldn't read, say), not the key or the service.
#[derive(Debug)]
struct Refused(String);

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Refused {}

/// Tries before giving up on a busy service.
const TRIES: u32 = 6;

async fn request(http: &reqwest::Client, h: &Hosted, batch: &Batch<'_>, expected: usize) -> Result<Vec<Vec<f32>>> {
    let body = body(h, batch);
    let mut wait = Duration::from_secs(2);
    for attempt in 1..=TRIES {
        let sent =
            http.post(h.endpoint()).bearer_auth(&h.key).timeout(Duration::from_secs(120)).json(&body).send().await;
        let resp = match sent {
            Ok(r) => r,
            Err(e) if attempt < TRIES => {
                tracing::debug!(error = %e, provider = h.provider.id(), "image search service unreachable; retrying");
                tokio::time::sleep(wait).await;
                wait = (wait * 2).min(Duration::from_mins(1));
                continue;
            }
            Err(e) => return Err(e).with_context(|| format!("couldn't reach {}", h.provider.name())),
        };
        let status = resp.status();
        if status.is_success() {
            let v: Value = resp.json().await.with_context(|| format!("{}'s answer", h.provider.name()))?;
            return parse(h.provider, v, expected).with_context(|| format!("{}'s answer", h.provider.name()));
        }
        let retry_after = resp
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok()?.parse::<u64>().ok())
            .map(Duration::from_secs);
        let text = resp.text().await.unwrap_or_default();
        let detail: String = text.trim().chars().take(300).collect();
        if (status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()) && attempt < TRIES {
            tracing::debug!(%status, provider = h.provider.id(), "image search service busy; retrying");
            tokio::time::sleep(retry_after.unwrap_or(wait).min(Duration::from_mins(2))).await;
            wait = (wait * 2).min(Duration::from_mins(1));
            continue;
        }
        let msg = format!("{} answered {status}: {detail}", h.provider.name());
        if status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY {
            return Err(Refused(msg).into());
        }
        bail!(msg);
    }
    unreachable!("the last try returns")
}

/// A hosted image search: its settings, the last thing that went wrong, and recent searches.
#[derive(Debug)]
pub(crate) struct Client {
    pub(crate) hosted: Hosted,
    /// Why the last request failed, until one works.
    failed: Mutex<Option<String>>,
    recent: Mutex<std::collections::HashMap<String, std::sync::Arc<Vec<f32>>>>,
}

impl Client {
    pub(crate) fn new(hosted: Hosted) -> Self {
        Self { hosted, failed: Mutex::new(None), recent: Mutex::default() }
    }

    pub(crate) fn failed(&self) -> Option<String> {
        self.failed.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    fn note<T>(&self, r: &Result<T>) {
        *self.failed.lock().unwrap_or_else(PoisonError::into_inner) = r.as_ref().err().map(|e| format!("{e:#}"));
    }

    /// The embedding of a search's words.
    pub(crate) async fn words(&self, http: &reqwest::Client, text: &str) -> Result<std::sync::Arc<Vec<f32>>> {
        if let Some(v) = self.recent.lock().unwrap_or_else(PoisonError::into_inner).get(text) {
            return Ok(std::sync::Arc::clone(v));
        }
        let r = request(http, &self.hosted, &Batch::Words(text), 1).await;
        self.note(&r);
        let v = std::sync::Arc::new(crate::clip::normalized(r?.swap_remove(0)));
        let mut recent = self.recent.lock().unwrap_or_else(PoisonError::into_inner);
        if recent.len() >= 64 {
            recent.clear();
        }
        recent.insert(text.to_owned(), std::sync::Arc::clone(&v));
        Ok(v)
    }

    /// The embeddings of thumbnails, `None` for one that isn't a picture the provider takes or that it refused.
    pub(crate) async fn pictures(&self, http: &reqwest::Client, pictures: &[&[u8]]) -> Result<Vec<Option<Vec<f32>>>> {
        let r = self.pictures_inner(http, pictures).await;
        self.note(&r);
        r
    }

    async fn pictures_inner(&self, http: &reqwest::Client, pictures: &[&[u8]]) -> Result<Vec<Option<Vec<f32>>>> {
        let uris: Vec<Option<String>> = pictures.iter().map(|b| mime(b).map(|m| data_uri(m, b))).collect();
        let sent: Vec<String> = uris.iter().flatten().cloned().collect();
        if sent.is_empty() {
            return Ok(vec![None; pictures.len()]);
        }
        let embedded: Vec<Option<Vec<f32>>> =
            match request(http, &self.hosted, &Batch::Pictures(&sent), sent.len()).await {
                Ok(vs) => vs.into_iter().map(Some).collect(),
                // One picture it won't take spoils the batch: ask for each alone, and leave out the ones it refuses.
                Err(e) if e.is::<Refused>() && sent.len() > 1 => {
                    let mut out = Vec::with_capacity(sent.len());
                    for uri in &sent {
                        match request(http, &self.hosted, &Batch::Pictures(std::slice::from_ref(uri)), 1).await {
                            Ok(mut v) => out.push(Some(v.swap_remove(0))),
                            Err(e) if e.is::<Refused>() => {
                                tracing::debug!(error = %e, "the image search service refused a thumbnail");
                                out.push(None);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    out
                }
                Err(e) if e.is::<Refused>() => vec![None],
                Err(e) => return Err(e),
            };
        let mut embedded = embedded.into_iter();
        Ok(uris.iter().map(|u| u.as_ref().and_then(|_| embedded.next().flatten())).collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use axum::Json;
    use axum::http::HeaderMap;
    use axum::response::IntoResponse;

    use super::*;

    const PNG: &[u8] = b"\x89PNG\r\n\x1a\nrest";
    const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0rest";
    const BAD: &[u8] = b"\x89PNG\r\n\x1a\nBAD";

    fn hosted(provider: Provider, url: &str) -> Hosted {
        Hosted { provider, model: provider.default_model().into(), key: "k".into(), url: Some(url.into()) }
    }

    /// A stand-in for the providers: answers in each one's shape, refuses a batch holding [`BAD`], and is busy for
    /// the first request.
    async fn serve() -> (String, Arc<Mutex<Vec<Value>>>) {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let calls = Arc::new(AtomicUsize::new(0));
        let handler = {
            let seen = Arc::clone(&seen);
            move |headers: HeaderMap, Json(body): Json<Value>| {
                let (seen, calls) = (Arc::clone(&seen), Arc::clone(&calls));
                async move {
                    assert_eq!(headers.get("authorization").unwrap(), "Bearer k");
                    if calls.fetch_add(1, Ordering::Relaxed) == 0 {
                        return (StatusCode::TOO_MANY_REQUESTS, [("retry-after", "0")], "slow down").into_response();
                    }
                    seen.lock().unwrap().push(body.clone());
                    let bad = base64::engine::general_purpose::STANDARD.encode(BAD);
                    if body.to_string().contains(&bad) {
                        return (StatusCode::BAD_REQUEST, "can't read an image").into_response();
                    }
                    // How many inputs, and a vector for each whose first number says which.
                    let n = ["inputs", "input", "images", "texts"]
                        .iter()
                        .find_map(|k| body[k].as_array().map(Vec::len))
                        .unwrap();
                    let vs: Vec<Vec<f32>> = (0..n).map(|i| vec![i as f32 + 1.0, 1.0]).collect();
                    if body.get("embedding_types").is_some() {
                        Json(json!({ "embeddings": { "float": vs } })).into_response()
                    } else {
                        // Out of order, as the index says where each goes.
                        let data: Vec<Value> =
                            vs.iter().enumerate().rev().map(|(i, v)| json!({ "index": i, "embedding": v })).collect();
                        Json(json!({ "data": data })).into_response()
                    }
                }
            }
        };
        let app = axum::Router::new()
            .route("/v1/multimodalembeddings", axum::routing::post(handler.clone()))
            .route("/v1/embed", axum::routing::post(handler.clone()))
            .route("/v1/embeddings", axum::routing::post(handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        (format!("http://{addr}/v1"), seen)
    }

    #[test]
    fn pictures_are_told_by_their_first_bytes() {
        assert_eq!(mime(PNG), Some("image/png"));
        assert_eq!(mime(JPEG), Some("image/jpeg"));
        assert_eq!(mime(b"RIFF\0\0\0\0WEBPVP8 "), Some("image/webp"));
        assert_eq!(mime(b"GIF89a"), Some("image/gif"));
        assert_eq!(mime(b"glTF"), None);
        assert_eq!(Provider::parse("cohere"), Some(Provider::Cohere));
        assert_eq!(Provider::parse(""), None);
        let h = hosted(Provider::Voyage, "https://gw.example/v1/");
        assert_eq!(h.endpoint(), "https://gw.example/v1/multimodalembeddings");
        assert_eq!(h.id(), "voyage:voyage-multimodal-3.5");
    }

    #[tokio::test]
    async fn each_provider_is_asked_in_its_own_shape() {
        let http = reqwest::Client::new();
        for provider in Provider::ALL {
            let (url, seen) = serve().await;
            let client = Client::new(hosted(provider, &url));
            let out = client.pictures(&http, &[PNG, b"not a picture", JPEG]).await.unwrap();
            assert_eq!(out[0].as_deref(), Some(&[1.0, 1.0][..]), "{provider:?}");
            assert!(out[1].is_none(), "what isn't a picture isn't sent");
            assert_eq!(out[2].as_deref(), Some(&[2.0, 1.0][..]), "{provider:?}: in the order asked");
            let q = client.words(&http, "a mossy rock").await.unwrap();
            assert!((q.iter().map(|x| x * x).sum::<f32>() - 1.0).abs() < 1e-5, "normalized");
            assert!(client.failed().is_none());

            let seen = seen.lock().unwrap();
            let (pics, words) = (seen[0].to_string(), seen[1].to_string());
            assert!(pics.contains("data:image/png;base64,") && pics.contains("data:image/jpeg;base64,"), "{pics}");
            assert!(words.contains("a mossy rock"));
            match provider {
                Provider::Voyage => {
                    assert_eq!(seen[0]["input_type"], "document");
                    assert_eq!(seen[1]["input_type"], "query");
                    assert_eq!(seen[0]["inputs"][0]["content"][0]["type"], "image_base64");
                }
                Provider::Cohere => {
                    assert_eq!(seen[0]["input_type"], "search_document");
                    assert_eq!(seen[1]["input_type"], "search_query");
                    assert_eq!(seen[0]["images"].as_array().unwrap().len(), 2);
                }
                Provider::Jina => {
                    assert!(seen[0]["input"][0]["image"].is_string());
                    assert_eq!(seen[1]["input"][0]["text"], "a mossy rock");
                }
            }
        }
    }

    #[tokio::test]
    async fn a_picture_the_service_refuses_is_left_out() {
        let http = reqwest::Client::new();
        let (url, seen) = serve().await;
        let client = Client::new(hosted(Provider::Voyage, &url));
        let out = client.pictures(&http, &[PNG, BAD, JPEG]).await.unwrap();
        assert!(out[0].is_some() && out[1].is_none() && out[2].is_some(), "{out:?}");
        assert_eq!(seen.lock().unwrap().len(), 4, "the batch, then each alone");
    }

    #[tokio::test]
    async fn a_wrong_key_is_said() {
        let app = axum::Router::new()
            .route("/v1/embeddings", axum::routing::post(|| async { (StatusCode::UNAUTHORIZED, "invalid api key") }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = Client::new(hosted(Provider::Jina, &format!("http://{addr}/v1")));
        assert!(client.pictures(&reqwest::Client::new(), &[PNG]).await.is_err());
        let why = client.failed().unwrap();
        assert!(why.contains("Jina AI") && why.contains("401") && why.contains("invalid api key"), "{why}");
    }
}
