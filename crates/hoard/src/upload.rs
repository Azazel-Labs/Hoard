//! `hoard upload`: resumable bulk uploads over the server's upload-session API (see `hoardd/src/uploads.rs`).
//!
//! Every file is registered first; small files then go up in batches and big ones in 32 MB chunks, four requests at
//! a time, each retried with backoff. The server recognizes a re-sent file list, so re-running the same command after
//! an interruption resumes the unfinished pack and skips the ones that already made it.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use futures_util::{StreamExt, stream};
use hoard_core::Pack;
use serde::Deserialize;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::{Client, human_bytes};

const SMALL: u64 = 4 << 20;
const BATCH_BYTES: u64 = 32 << 20;
const BATCH_FILES: usize = 400;
const PARALLEL: usize = 6;
const ATTEMPTS: u32 = 8;

#[derive(Debug, Clone)]
struct LocalFile {
    /// Path inside the pack, `/`-separated, starting with the pack's folder name.
    rel: String,
    abs: PathBuf,
    size: u64,
}

#[derive(Debug)]
struct Group {
    name: Option<String>,
    label: String,
    files: Vec<LocalFile>,
}

#[derive(Debug, Deserialize)]
struct FileStatus {
    path: String,
    size: u64,
    received: u64,
    /// The parts of a big file that have arrived.
    #[serde(default)]
    parts: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct Session {
    pack: Pack,
    files: Vec<FileStatus>,
    total_bytes: u64,
    received_bytes: u64,
    part_bytes: u64,
    #[serde(default)]
    resumed: bool,
}

fn is_archive(p: &Path) -> bool {
    let n = p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
    [".zip", ".tar", ".tgz", ".tar.gz"].iter().any(|e| n.ends_with(e))
}

/// All files under `root`, with paths relative to `root`'s parent (so they start with `root`'s own name).
fn collect(root: &Path) -> Result<Vec<LocalFile>> {
    let base = root.parent().unwrap_or(Path::new(""));
    let mut out = Vec::new();
    for entry in walkdir::WalkDir::new(root).follow_links(false) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry.path().strip_prefix(base).unwrap_or(entry.path());
        let rel = rel.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/");
        out.push(LocalFile { rel, abs: entry.path().to_path_buf(), size: entry.metadata()?.len() });
    }
    Ok(out)
}

fn file_label(p: &Path) -> String {
    p.file_name().map_or_else(|| p.display().to_string(), |n| n.to_string_lossy().into_owned())
}

fn plan(paths: &[PathBuf], name: Option<String>, each: bool) -> Result<Vec<Group>> {
    if !each {
        let mut files = Vec::new();
        for p in paths {
            files.extend(collect(p)?);
        }
        let label = name.clone().unwrap_or_else(|| paths.first().map(|p| file_label(p)).unwrap_or_default());
        return Ok(vec![Group { name, label, files }]);
    }
    let mut groups = Vec::new();
    for root in paths {
        if !root.is_dir() {
            groups.push(Group { name: None, label: file_label(root), files: collect(root)? });
            continue;
        }
        let mut loose = Vec::new();
        let mut children: Vec<PathBuf> = std::fs::read_dir(root)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
        children.sort();
        for child in children {
            let label = file_label(&child);
            if label.starts_with('.') {
                continue;
            }
            if child.is_dir() {
                groups.push(Group { name: Some(label.clone()), label, files: collect(&child)? });
            } else if is_archive(&child) {
                groups.push(Group { name: None, label, files: collect(&child)? });
            } else {
                loose.extend(collect(&child)?.into_iter().map(|mut f| {
                    f.rel = format!("{}/{}", file_label(root), f.rel);
                    f
                }));
            }
        }
        if !loose.is_empty() {
            let label = file_label(root);
            groups.push(Group { name: Some(label.clone()), label, files: loose });
        }
    }
    Ok(groups)
}

async fn with_retry<T, F, Fut>(what: &str, mut f: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut last = None;
    for attempt in 0..ATTEMPTS {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                last = Some(e);
                tokio::time::sleep(Duration::from_millis((500u64 << attempt).min(30_000))).await;
            }
        }
    }
    Err(last.unwrap_or_else(|| anyhow::anyhow!("failed")).context(format!("{what}: gave up after {ATTEMPTS} attempts")))
}

pub(crate) async fn run(
    client: &Client,
    paths: &[PathBuf],
    name: Option<String>,
    source: Option<String>,
    each: bool,
    force: bool,
) -> Result<()> {
    let groups = plan(paths, name, each)?;
    let groups: Vec<Group> = groups.into_iter().filter(|g| !g.files.is_empty()).collect();
    if groups.is_empty() {
        bail!("nothing to upload");
    }
    let total: u64 = groups.iter().flat_map(|g| &g.files).map(|f| f.size).sum();
    let count: usize = groups.iter().map(|g| g.files.len()).sum();
    println!("{} pack(s), {count} files, {}", groups.len(), human_bytes(total));
    for (i, g) in groups.iter().enumerate() {
        println!("[{}/{}] {}", i + 1, groups.len(), g.label);
        match upload_group(client, g, source.clone(), force).await? {
            Outcome::Queued(pack) => println!("      queued for ingest as \"{}\" ({})", pack.name, pack.id),
            Outcome::AlreadyImported(name) => {
                println!("      already imported as \"{name}\", skipping (use --force to import again)");
            }
        }
    }
    println!("watch progress with `hoard packs`");
    Ok(())
}

enum Outcome {
    Queued(Box<Pack>),
    AlreadyImported(String),
}

/// Register the file list. The server resumes a matching open session, or refuses a list it already imported.
async fn open_session(
    client: &Client,
    g: &Group,
    source: Option<String>,
    force: bool,
) -> Result<Result<Session, String>> {
    let body = json!({
        "name": g.name,
        "source": source,
        "force": force,
        "files": g.files.iter().map(|f| json!({ "path": f.rel, "size": f.size })).collect::<Vec<_>>(),
    });
    let resp = client.auth(client.http.post(client.url("/api/uploads")).json(&body)).send().await?;
    if resp.status() == reqwest::StatusCode::CONFLICT {
        let v: serde_json::Value = resp.json().await.unwrap_or_default();
        let name = v.pointer("/pack/name").and_then(|n| n.as_str()).unwrap_or("an existing pack").to_owned();
        return Ok(Err(name));
    }
    let status = resp.status();
    if !status.is_success() {
        bail!("server returned {status}: {}", resp.text().await.unwrap_or_default());
    }
    let s: Session = resp.json().await?;
    if s.resumed {
        println!(
            "      resuming: {} of {} already on the server",
            human_bytes(s.received_bytes),
            human_bytes(s.total_bytes)
        );
    }
    Ok(Ok(s))
}

/// What still needs sending: batches of small untouched files, and each missing part of the big ones (parts go up at
/// the same time, so one big file isn't stuck on one connection).
enum Task {
    Batch(Vec<(LocalFile, u64)>),
    Part { file: LocalFile, offset: u64, len: u64 },
}

fn plan_tasks(g: &Group, session: &Session) -> Result<Vec<Task>> {
    let by_rel: std::collections::HashMap<&str, &LocalFile> = g.files.iter().map(|f| (f.rel.as_str(), f)).collect();
    let mut small = Vec::new();
    let mut tasks = Vec::new();
    for st in &session.files {
        let Some(local) = by_rel.get(st.path.as_str()).copied().cloned().or_else(|| {
            // The server normalizes paths; fall back to matching by normalized form.
            g.files.iter().find(|f| f.rel.trim_start_matches('/') == st.path).cloned()
        }) else {
            bail!("server expects {} but it isn't in the local file list", st.path);
        };
        if st.received >= st.size {
            continue;
        }
        if st.size <= SMALL && st.received == 0 {
            small.push((local, st.size));
        } else {
            let done: std::collections::HashSet<u64> = st.parts.iter().copied().collect();
            for part in (0..st.size.div_ceil(session.part_bytes)).filter(|part| !done.contains(part)) {
                let offset = part * session.part_bytes;
                let len = session.part_bytes.min(st.size - offset);
                tasks.push(Task::Part { file: local.clone(), offset, len });
            }
        }
    }
    let mut batch = Vec::new();
    let mut batch_bytes = 0;
    for (f, size) in small {
        if !batch.is_empty() && (batch.len() >= BATCH_FILES || batch_bytes + size > BATCH_BYTES) {
            tasks.push(Task::Batch(std::mem::take(&mut batch)));
            batch_bytes = 0;
        }
        batch_bytes += size;
        batch.push((f, size));
    }
    if !batch.is_empty() {
        tasks.push(Task::Batch(batch));
    }
    Ok(tasks)
}

async fn upload_group(client: &Client, g: &Group, source: Option<String>, force: bool) -> Result<Outcome> {
    let session = match open_session(client, g, source, force).await? {
        Ok(s) => s,
        Err(existing) => return Ok(Outcome::AlreadyImported(existing)),
    };
    let id = session.pack.id.clone();
    let sent = Arc::new(AtomicU64::new(session.received_bytes));
    let total = session.total_bytes;
    let tasks = plan_tasks(g, &session)?;

    let started = Instant::now();
    let start_sent = sent.load(Ordering::Relaxed);
    let ticker = {
        let sent = Arc::clone(&sent);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(2)).await;
                let now = sent.load(Ordering::Relaxed);
                let rate = (now - start_sent) as f64 / started.elapsed().as_secs_f64().max(0.001);
                #[expect(clippy::print_stdout, reason = "progress output")]
                {
                    println!("      {} / {}  {}/s", human_bytes(now), human_bytes(total), human_bytes(rate as u64));
                }
            }
        })
    };

    let id_ref = &id;
    let sent_ref = &sent;
    let results: Vec<Result<()>> = stream::iter(tasks)
        .map(|task| async move {
            match task {
                Task::Batch(files) => {
                    with_retry("batch upload", || async {
                        let mut form = reqwest::multipart::Form::new();
                        for (f, _) in &files {
                            let bytes = tokio::fs::read(&f.abs).await?;
                            form = form
                                .text("relpath", f.rel.clone())
                                .part("file", reqwest::multipart::Part::bytes(bytes).file_name(f.rel.clone()));
                        }
                        let url = client.url(&format!("/api/uploads/{id_ref}/batch"));
                        let _: serde_json::Value = client.json(client.http.post(url).multipart(form)).await?;
                        Ok(())
                    })
                    .await?;
                    sent_ref.fetch_add(files.iter().map(|(_, s)| s).sum(), Ordering::Relaxed);
                    Ok(())
                }
                Task::Part { file: f, offset, len } => {
                    with_retry(&f.rel, || async {
                        let mut file = tokio::fs::File::open(&f.abs).await?;
                        file.seek(std::io::SeekFrom::Start(offset)).await?;
                        let mut buf = vec![0u8; usize::try_from(len)?];
                        file.read_exact(&mut buf).await?;
                        let url = client.url(&format!("/api/uploads/{id_ref}/chunk"));
                        let req = client
                            .http
                            .put(url)
                            .query(&[("path", f.rel.as_str()), ("offset", &offset.to_string())])
                            .body(buf);
                        let resp = client.auth(req).send().await?;
                        let status = resp.status();
                        if status.is_success() {
                            return Ok(());
                        }
                        let body: serde_json::Value = resp.json().await.unwrap_or_default();
                        bail!("{status}: {}", body.get("error").and_then(|e| e.as_str()).unwrap_or("upload failed"))
                    })
                    .await?;
                    sent_ref.fetch_add(len, Ordering::Relaxed);
                    Ok(())
                }
            }
        })
        .buffer_unordered(PARALLEL)
        .collect()
        .await;
    ticker.abort();
    for r in results {
        r?;
    }
    let pack: Pack = with_retry("finish upload", || async {
        client.json(client.http.post(client.url(&format!("/api/uploads/{id}/complete")))).await
    })
    .await
    .context("files are uploaded but finishing failed; re-run the same command to retry")?;
    Ok(Outcome::Queued(Box::new(pack)))
}
