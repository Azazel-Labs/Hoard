//! Search sounds by what they sound like, in words, with CLAP (LAION's `clap-htsat-unfused`): every sound gets a
//! 512-number embedding, and so does what's typed in the search box, so "glassy UI blip" or "distant thunder roll"
//! finds sounds whatever their files are called. It is to sounds what [`crate::clip`] is to pictures.
//!
//! It's off until an admin turns it on in Settings, because the model is a 614 MB download. Turned on, the model is
//! fetched (pinned to one revision, checked against its SHA-256) into `<cache>/models`, and a background job listens
//! to every sound at low priority: up to its first ten seconds, silence at the ends left out, at 48 kHz, as a log-mel
//! spectrogram. The text half is loaded on the first search and dropped after a while without one; the audio half
//! only lives while the job runs.
//!
//! Embeddings are stored per sound file (`sound_embeddings`, keyed by its hash) as 512 signed bytes, scaled per
//! vector, and searched brute force from memory.

mod htsat;
mod roberta;
mod spectrogram;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use crate::embed::{Entry, Model, Spec, Table, Text, in_pool, quantize};
use crate::tools::{Slot, Status};
use crate::{AppState, db};
use htsat::AudioModel;
use roberta::TextModel;
pub(crate) use spectrogram::prepare;

/// Stored with each embedding; rows from another model are redone.
pub(crate) const MODEL: &str = "clap-htsat-unfused";

/// Where the files come from: one revision of the model (the safetensors conversion of LAION's weights), pinned.
const REPO: &str = "https://huggingface.co/laion/clap-htsat-unfused/resolve/79b58ed25fc00386262a2bea4b19fd21dc4310a0";
const WEIGHTS: &str = "model.safetensors";
const TOKENIZER: &str = "tokenizer.json";
/// Name, size and SHA-256 of each file.
const FILES: [(&str, u64, &str); 2] = [
    (TOKENIZER, 2_108_746, "77ef92283d67f0d97e1454909a964afcbfa2019f0fb9f18f8e88d5c25c3ba729"),
    (WEIGHTS, 614_431_440, "abba8294f0c63d00eb3f8d998dacd16177cc5860cad3a11618e7f361a75587fe"),
];
/// The whole download, for the Settings page.
pub(crate) const DOWNLOAD_BYTES: u64 = FILES[0].1 + FILES[1].1;

const DIMS: usize = 512;

static CLAP: Model = Model { name: MODEL, repo: REPO, files: &FILES, what: "sound search", activity: "clap" };

static SLOT: Slot<PathBuf> = Slot::new();

pub(crate) fn is_ready() -> bool {
    SLOT.ready().is_some()
}

/// `off`, `looking`, `downloading`, `ready` or `unavailable: <why>`.
pub(crate) fn describe() -> String {
    SLOT.describe(|_| "ready".into())
}

/// Turn sound search on (find the model, downloading it if need be) or off.
pub(crate) fn start(state: &AppState, enabled: bool) {
    TEXT.clear();
    if !enabled {
        SLOT.restart(Status::Off);
        return;
    }
    CLAP.fetch(state, &SLOT, |st| async move {
        TEXT.unload_when_idle();
        schedule(&st).await;
    });
}

pub(crate) async fn remove_files(state: &AppState) {
    CLAP.remove_files(state).await;
}

static TEXT: Text<TextModel> = Text::new("sound search", TextModel::load, TextModel::embed);

/// Sound search's embeddings: per sound file. Words score far apart from sounds: on a library of game sounds (300 of
/// Kenney's), a true match stands 0.25–0.6 above the median sound for the same words ("an explosion" +0.54, "a laser
/// gun" +0.45, "footsteps" +0.25), and words with nothing to find (a cat, birds, someone talking) peak around 0.2.
pub(crate) static SPEC: Spec = Spec {
    load: "SELECT hash, vector FROM sound_embeddings WHERE model = ?1",
    model: MODEL,
    dims: DIMS,
    what: "sound search",
    // A sound's "more like this" listens instead (see `sounds`).
    unrelated: 0.5,
    same: 0.95,
    relative: false,
    lift: 0.24,
    margin: 0.12,
    sure: 0.33,
    weak: 0.24,
    strong: 0.45,
};

/// Sound files that sound like what was typed, best first, scored about 0..=0.5; empty when sound search is off or
/// has nothing yet.
pub(crate) async fn search_text(state: &AppState, text: &str) -> Result<Vec<(String, f32)>> {
    let Some(dir) = SLOT.ready() else { return Ok(Vec::new()) };
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    state.clap.ensure(state).await?;
    if state.clap.len() == 0 {
        return Ok(Vec::new());
    }
    let q = TEXT.embed(dir, text.trim().to_lowercase()).await?;
    Ok(state.clap.matching(&q))
}

/// Embeddings are per sound file.
static TABLE: Table = Table {
    name: "sound_embeddings",
    key: "hash",
    asset_key: "blob_hash",
    kinds: crate::sounds::KINDS_SQL,
    orphan: "NOT EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = e.hash AND a.kind = 'audio')",
    job: crate::jobs::HEAR,
};

/// How many sound files have an embedding, and how many could, for the Settings page.
pub(crate) fn progress(conn: &rusqlite::Connection) -> Result<(u64, u64)> {
    TABLE.progress(conn, MODEL)
}

/// Queue a job when sound search is on and some sound has no embedding yet.
pub(crate) async fn schedule(state: &AppState) {
    if is_ready()
        && let Err(e) = TABLE.queue(state, MODEL.to_owned()).await
    {
        tracing::warn!(error = ?e, "could not queue the sound search job");
    }
}

/// Sounds read and embedded per round.
const BATCH: usize = 8;
/// As much of a file as is read: plenty for its first ten seconds.
const MAX_BYTES: u64 = 12 << 20;

/// Embed every sound that has no embedding, and forget the ones whose files are gone.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    let Some(dir) = SLOT.ready() else { return Ok(()) };
    state.clap.ensure(state).await?;
    let gone = TABLE.forget_orphans(state).await?;
    state.clap.write(|m| {
        for h in &gone {
            m.remove(h);
        }
    });

    let mut model: Option<Arc<AudioModel>> = None;
    let mut unreadable = std::collections::HashSet::new();
    let mut done = 0usize;
    loop {
        let skip = unreadable.clone();
        let todo: Vec<(String, String)> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT a.blob_hash, MIN(a.ext) FROM assets a WHERE {} AND {} GROUP BY a.blob_hash",
                    TABLE.wanted(),
                    TABLE.missing()
                ))?
                .query_map([MODEL], |r| Ok((r.get(0)?, r.get(1)?)))?
                .collect::<rusqlite::Result<Vec<(String, String)>>>()?;
            Ok(rows.into_iter().filter(|(h, _)| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return Ok(());
        }
        let model = if let Some(m) = &model {
            Arc::clone(m)
        } else {
            let d = dir.clone();
            let m = Arc::new(crate::heavy::run("clap-load", move || AudioModel::load(&d)).await??);
            Arc::clone(model.insert(m))
        };
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            // Switched off partway through: stop, keeping what's done.
            if !is_ready() {
                return Ok(());
            }
            crate::activity::job_progress(crate::jobs::HEAR, None, done, total, "sounds");
            let mut files = Vec::with_capacity(chunk.len());
            for (hash, ext) in chunk {
                match state.blobs.head(hash, MAX_BYTES).await {
                    Ok(bytes) => files.push((hash.clone(), ext.clone(), bytes::Bytes::from(bytes))),
                    Err(e) => {
                        tracing::debug!(hash, error = %e, "sound unreadable for sound search");
                        unreadable.insert(hash.clone());
                    }
                }
            }
            let m = Arc::clone(&model);
            let computed = crate::heavy::run("clap", move || in_pool(|| embed_all(&m, &files))).await??;
            TABLE.save(state, MODEL, &computed).await?;
            done += chunk.len();
            state.clap.write(|m| {
                for (hash, v) in computed {
                    if let Some(e) = v.and_then(Entry::new) {
                        m.insert(hash, e);
                    }
                }
            });
        }
        crate::activity::job_progress(crate::jobs::HEAR, None, done, total, "sounds");
    }
}

fn embed_all(model: &AudioModel, files: &[(String, String, bytes::Bytes)]) -> Result<Vec<(String, Option<Vec<i8>>)>> {
    use rayon::prelude::*;
    let prepared: Vec<(String, Option<Vec<f32>>)> = files
        .par_iter()
        .map(|(h, ext, b)| {
            let clip =
                crate::sounds::decode_mono(b.clone(), Some(ext)).and_then(|(mono, rate, _)| prepare(&mono, rate));
            (h.clone(), clip)
        })
        .collect();
    let inputs: Vec<Vec<f32>> = prepared.iter().filter_map(|(_, p)| p.clone()).collect();
    let mut embedded = model.embed(&inputs)?.into_iter();
    Ok(prepared.into_iter().map(|(h, p)| (h, p.and_then(|_| embedded.next()).map(|v| quantize(&v)))).collect())
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::clap::roberta::tokens;
    use crate::clap::spectrogram::{RATE, SAMPLES, fit};

    /// The real model, when it's been downloaded: `HOARD_CLAP_DIR=<folder with the two files> cargo test -- --ignored`.
    /// With `HOARD_CLAP_WAVS` (a folder of WAV files), it prints each one's embedding's first numbers and its cosine
    /// with a few descriptions, to compare with a reference.
    #[test]
    #[ignore = "needs the 614 MB model"]
    fn real_model_matches_words_to_sounds() {
        let dir = PathBuf::from(std::env::var("HOARD_CLAP_DIR").expect("HOARD_CLAP_DIR"));
        let text = TextModel::load(&dir).unwrap();
        eprintln!("tokens: {:?}", tokens(&text.tokenizer, "a dog barking").unwrap());
        let audio = AudioModel::load(&dir).unwrap();
        let words = ["a dog barking", "rain falling", "a low thud", "a laser zap", "a pure tone", "white noise"];
        let texts: Vec<Vec<f32>> = words.iter().map(|w| text.embed(w).unwrap()).collect();
        for (w, t) in words.iter().zip(&texts) {
            eprintln!("text {w:?}: {:?}", &t[..6]);
        }
        let cos = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
        let tone: Vec<f32> =
            (0..SAMPLES).map(|i| (0.5 * (2.0 * PI * 440.0 * i as f64 / 48_000.0).sin()) as f32).collect();
        let mut seed = 1u32;
        let noise: Vec<f32> = (0..SAMPLES)
            .map(|_| {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (seed >> 8) as f32 / (1 << 24) as f32 - 0.5
            })
            .collect();
        let started = std::time::Instant::now();
        let got = audio.embed(&[tone, noise]).unwrap();
        eprintln!("2 clips in {:?}", started.elapsed());
        for (name, v) in ["tone", "noise"].iter().zip(&got) {
            let sims: Vec<String> = words.iter().zip(&texts).map(|(w, t)| format!("{w}: {:.3}", cos(v, t))).collect();
            eprintln!("{name}: {:?} | {}", &v[..6], sims.join(", "));
        }
        assert!(cos(&got[0], &texts[4]) > cos(&got[0], &texts[5]), "a tone is more a tone than noise");
        assert!(cos(&got[1], &texts[5]) > cos(&got[1], &texts[4]), "noise is more noise than a tone");
        if let Ok(wavs) = std::env::var("HOARD_CLAP_WAVS") {
            let mut paths: Vec<_> = std::fs::read_dir(wavs).unwrap().map(|e| e.unwrap().path()).collect();
            paths.sort();
            for p in paths.iter().filter(|p| p.extension().is_some_and(|e| e == "wav")) {
                let bytes = bytes::Bytes::from(std::fs::read(p).unwrap());
                let (mut mono, rate, _) = crate::sounds::decode_mono(bytes, Some("wav")).unwrap();
                // As given (at 48 kHz, so nothing is resampled), so a reference fed the same samples should agree.
                assert_eq!(rate, RATE, "{}", p.display());
                let v = &audio.embed(&[fit(&mut mono)]).unwrap()[0];
                let sims: Vec<String> =
                    words.iter().zip(&texts).map(|(w, t)| format!("{w}: {:.3}", cos(v, t))).collect();
                eprintln!("{}: {:?} | {}", p.display(), &v[..6], sims.join(", "));
            }
        }
    }

    /// Calibration on real sounds: `HOARD_CLAP_DIR=<model> HOARD_CLAP_SOUNDS=<folder with names.tsv> cargo test --
    /// --ignored calibrate --nocapture` prints, for each query, the median score, the best and the best few by name.
    #[test]
    #[ignore = "needs the model and a folder of sounds"]
    fn calibrate() {
        let dir = PathBuf::from(std::env::var("HOARD_CLAP_DIR").expect("HOARD_CLAP_DIR"));
        let folder = PathBuf::from(std::env::var("HOARD_CLAP_SOUNDS").expect("HOARD_CLAP_SOUNDS"));
        let names: Vec<(String, String)> = std::fs::read_to_string(folder.join("names.tsv"))
            .unwrap()
            .lines()
            .filter_map(|l| l.split_once('\t').map(|(f, n)| (f.to_owned(), n.to_owned())))
            .collect();
        // Embedding the sounds is the slow part: kept in a cache file between runs.
        let cache = std::env::temp_dir().join("hoard-clap-calibrate.tsv");
        let mut raw: Vec<(String, Vec<i8>)> = std::fs::read_to_string(&cache).map_or_default(|c| {
            c.lines()
                .filter_map(|l| l.split_once('\t'))
                .map(|(n, v)| (n.to_owned(), v.split(',').filter_map(|x| x.parse().ok()).collect()))
                .collect()
        });
        if raw.is_empty() {
            let audio = AudioModel::load(&dir).unwrap();
            let started = std::time::Instant::now();
            for chunk in names.chunks(BATCH) {
                let files: Vec<(String, String, bytes::Bytes)> = chunk
                    .iter()
                    .map(|(f, n)| {
                        let ext = f.rsplit('.').next().unwrap().to_owned();
                        (n.clone(), ext, bytes::Bytes::from(std::fs::read(folder.join(f)).unwrap()))
                    })
                    .collect();
                raw.extend(embed_all(&audio, &files).unwrap().into_iter().filter_map(|(n, v)| Some((n, v?))));
            }
            eprintln!("{} sounds in {:?}", raw.len(), started.elapsed());
            let lines: Vec<String> = raw
                .iter()
                .map(|(n, v)| format!("{n}\t{}", v.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")))
                .collect();
            std::fs::write(&cache, lines.join("\n")).unwrap();
        }
        let sounds: Vec<(String, Entry)> = raw.into_iter().filter_map(|(n, v)| Some((n, Entry::new(v)?))).collect();
        let text = TextModel::load(&dir).unwrap();
        let queries = std::env::var("HOARD_CLAP_QUERIES").unwrap_or_else(|_| {
            "a dog barking|a car engine|thunder|an explosion|footsteps|picking up a coin|a laser gun|a button click|\
             a man saying fight|a door creaking|music|glass breaking|a whoosh|a bell ringing|a piano|a robot voice|\
             a woman speaking|a sword hit|a punch|cards being shuffled|dice rolling|water splashing|a monster growl|\
             a cat meowing|birds singing|a jump sound effect|game over|an engine humming|a spaceship|a beep"
                .into()
        });
        for q in queries.split('|') {
            let t = text.embed(q).unwrap();
            let mut scored: Vec<(f32, &str)> = sounds.iter().map(|(n, v)| (v.cosine_f32(&t), n.as_str())).collect();
            scored.sort_by(|a, b| b.0.total_cmp(&a.0));
            let median = scored[scored.len() / 2].0;
            let top: Vec<String> = scored
                .iter()
                .take(5)
                .map(|(s, n)| format!("{:+.3} {}", s - median, n.rsplit('/').take(2).collect::<Vec<_>>().join("<")))
                .collect();
            eprintln!("{q:?}: median {median:.3} best {:.3}\n    {}", scored[0].0, top.join("\n    "));
        }
    }
}
