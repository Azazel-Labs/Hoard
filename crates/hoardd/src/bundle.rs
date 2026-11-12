//! On-the-fly zip bundles: an asset plus everything it links to, laid out with their original relative paths
//! so a .gltf still finds its .bin and textures after unzipping.

use std::io::Write;

use bytes::Bytes;
use futures_util::StreamExt;

use anyhow::Result;
use axum::body::Body;
use axum::http::{StatusCode, header};
use axum::response::Response;
use zip::CompressionMethod;
use zip::write::SimpleFileOptions;

use crate::AppState;
use crate::db::{self, BundleFile};

/// Formats that are already compressed; deflating them again only burns CPU.
const STORED_EXTS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "ogg", "mp3", "opus", "m4a", "flac", "mp4", "webm", "zip", "ktx2", "basis", "dds",
    "glb",
];

/// Stream `files` as a zip under a top folder called `root`, paths relative to the files' common folder, with
/// `credits` as `CREDITS.md` beside them.
pub(crate) fn zip_files(
    state: &AppState,
    root: &str,
    files: &[BundleFile],
    credits: Option<String>,
) -> Result<Response> {
    let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
    let common = db::common_dir(&paths);
    let entries: Vec<Entry> = files
        .iter()
        .map(|f| Entry {
            name: db::strip_dir(&f.path, &common).to_owned(),
            hash: f.hash.clone(),
            size: f.size,
            unity_guid: f.unity_guid.clone(),
        })
        .collect();
    zip_response(state, root, entries, credits)
}

/// One file in a zip.
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) hash: String,
    pub(crate) size: u64,
    /// Write `<name>.meta` with this Unity GUID next to it.
    pub(crate) unity_guid: Option<String>,
}

/// How much zip output to gather before handing it to the connection.
const CHUNK: usize = 256 * 1024;

/// Sends what the zip writer produces to the response body. A closed connection stops the zipping.
struct ChannelWriter {
    tx: tokio::sync::mpsc::Sender<std::io::Result<Bytes>>,
    buf: Vec<u8>,
}

impl ChannelWriter {
    fn send(&mut self) -> std::io::Result<()> {
        if self.buf.is_empty() {
            return Ok(());
        }
        let chunk = Bytes::from(std::mem::replace(&mut self.buf, Vec::with_capacity(CHUNK)));
        self.tx.blocking_send(Ok(chunk)).map_err(|_| std::io::Error::from(std::io::ErrorKind::BrokenPipe))
    }
}

impl Write for ChannelWriter {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(data);
        if self.buf.len() >= CHUNK {
            self.send()?;
        }
        Ok(data.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.send()
    }
}

/// Stream entries as a zip under a top folder called `root`. The download starts at once; the zip is written on a
/// low-priority thread as the client reads it, so a large pack never waits to be zipped first. Already-compressed
/// formats are stored, the rest deflated. `credits` goes in as `CREDITS.md` at the top, or `CREDITS (Hoard).md` where
/// the files have a `CREDITS.md` of their own.
pub(crate) fn zip_response(
    state: &AppState,
    root: &str,
    entries: Vec<Entry>,
    credits: Option<String>,
) -> Result<Response> {
    let root = root.replace(['"', '\\', '/'], "_");
    let extra: Vec<(String, Vec<u8>)> = credits
        .map(|text| {
            let taken = entries.iter().any(|e| e.name.eq_ignore_ascii_case(CREDITS));
            let name = if taken { "CREDITS (Hoard).md" } else { CREDITS };
            (format!("{root}/{name}"), text.into_bytes())
        })
        .into_iter()
        .collect();
    let entries: Vec<Entry> = entries.into_iter().map(|e| Entry { name: format!("{root}/{}", e.name), ..e }).collect();
    let blobs = state.blobs.clone();
    let rt = tokio::runtime::Handle::current();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<std::io::Result<Bytes>>(8);
    let errors = tx.clone();
    let activity = crate::activity::start("zip", format!("Zipping {root}"), None);
    crate::heavy::spawn("zip", move || {
        let out = ChannelWriter { tx, buf: Vec::with_capacity(CHUNK) };
        let result = write_zip(&rt, &blobs, extra, entries, out, &activity);
        if let Err(e) = result {
            // Fail the response rather than end it cleanly, so the client knows the zip is incomplete.
            if !e.chain().any(|c| {
                c.downcast_ref::<std::io::Error>().is_some_and(|io| io.kind() == std::io::ErrorKind::BrokenPipe)
            }) {
                tracing::warn!(error = %format!("{e:#}"), "zip stream failed");
            }
            errors.blocking_send(Err(std::io::Error::other(format!("{e:#}")))).ok();
        }
    })?;
    let body = Body::from_stream(futures_util::stream::poll_fn(move |cx| rx.poll_recv(cx)));
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{root}.zip\""))
        .body(body)?)
}

/// The name of the credits file in a zip.
const CREDITS: &str = "CREDITS.md";

/// Write `extra` (small files made on the spot), then the blobs `entries` name.
fn write_zip(
    rt: &tokio::runtime::Handle,
    blobs: &crate::blobs::BlobStore,
    extra: Vec<(String, Vec<u8>)>,
    entries: Vec<Entry>,
    out: ChannelWriter,
    activity: &crate::activity::Handle,
) -> Result<()> {
    let total: u64 = entries.iter().map(|e| e.size).sum::<u64>().max(1);
    let (count, mut written) = (entries.len(), 0u64);
    activity.detail(format!("{count} files"));
    let mut zip = zip::ZipWriter::new_stream(out);
    let mut seen = std::collections::HashSet::new();
    for (name, data) in extra {
        let opts = SimpleFileOptions::default().last_modified_time(zip::DateTime::default_for_write());
        zip.start_file(name.as_str(), opts)?;
        zip.write_all(&data)?;
        seen.insert(name);
    }
    for Entry { name, hash, size, unity_guid } in entries {
        if !seen.insert(name.clone()) {
            continue;
        }
        if let Some(guid) = unity_guid {
            // A minimal .meta: Unity keeps the GUID (so materials still find their textures) and fills in the
            // importer settings itself.
            let opts = SimpleFileOptions::default().last_modified_time(zip::DateTime::default_for_write());
            zip.start_file(format!("{name}.meta"), opts)?;
            zip.write_all(format!("fileFormatVersion: 2\nguid: {guid}\n").as_bytes())?;
        }
        let ext = name.rsplit_once('.').map(|(_, e)| e.to_lowercase()).unwrap_or_default();
        let method =
            if STORED_EXTS.contains(&ext.as_str()) { CompressionMethod::Stored } else { CompressionMethod::Deflated };
        let opts = SimpleFileOptions::default()
            .compression_method(method)
            .large_file(size >= u64::from(u32::MAX))
            .last_modified_time(zip::DateTime::default_for_write());
        zip.start_file(name, opts)?;
        // Read in the runtime, write outside it: the writer waits on the connection.
        let Some(mut read) = rt.block_on(blobs.read(&hash, None))? else {
            anyhow::bail!("blob {hash} missing from storage")
        };
        while let Some(chunk) = rt.block_on(read.stream.next()) {
            let chunk = chunk?;
            written += chunk.len() as u64;
            zip.write_all(&chunk)?;
            activity.progress(written as f32 / total as f32);
        }
    }
    zip.finish()?.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn streamed_zip_unzips() {
        let dir = tempfile::tempdir().unwrap();
        let (cfg, pool) = crate::blobs::tests::setup(dir.path());
        crate::blobs::tests::add(&pool, dir.path(), "library", &[], true);
        let blobs = crate::blobs::BlobStore::open(&cfg, pool).unwrap();
        let text =
            blobs.put_bytes(b"hello hello hello hello".repeat(1000), hoard_core::AssetKind::Document).await.unwrap();
        let png = blobs.put_bytes(vec![7u8; 300_000], hoard_core::AssetKind::Texture).await.unwrap();
        let entries = vec![
            Entry { name: "root/a.txt".into(), hash: text.hash.clone(), size: 23_000, unity_guid: Some("abc".into()) },
            Entry { name: "root/img/b.png".into(), hash: png.hash.clone(), size: 300_000, unity_guid: None },
        ];
        let (tx, mut rx) = tokio::sync::mpsc::channel(8);
        let rt = tokio::runtime::Handle::current();
        let writer = std::thread::spawn(move || {
            let shown = crate::activity::start("zip", "Zipping test", None);
            let extra = vec![("root/CREDITS.md".to_owned(), b"# Credits\n".to_vec())];
            write_zip(&rt, &blobs, extra, entries, ChannelWriter { tx, buf: Vec::new() }, &shown).unwrap();
        });
        let mut bytes = Vec::new();
        while let Some(chunk) = rx.recv().await {
            bytes.extend_from_slice(&chunk.unwrap());
        }
        writer.join().unwrap();
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let names: Vec<String> = zip.file_names().map(|n| n.unwrap().into_owned()).collect();
        assert_eq!(names.len(), 4, "{names:?}");
        let mut s = String::new();
        std::io::Read::read_to_string(&mut zip.by_name("root/a.txt.meta").unwrap(), &mut s).unwrap();
        assert!(s.contains("guid: abc"));
        let mut b = Vec::new();
        std::io::Read::read_to_end(&mut zip.by_name("root/img/b.png").unwrap(), &mut b).unwrap();
        assert_eq!(b.len(), 300_000);
        s.clear();
        std::io::Read::read_to_string(&mut zip.by_name("root/CREDITS.md").unwrap(), &mut s).unwrap();
        assert_eq!(s, "# Credits\n");
    }
}
