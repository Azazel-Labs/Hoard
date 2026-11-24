//! "More like this" for sounds: a small acoustic descriptor per audio file, so a footstep finds other footsteps and a
//! laser other lasers, whatever they're called.
//!
//! A waveform thumbnail says little about how something sounds, so this is worked out from the file itself: its first
//! [`MAX_SECS`] seconds, mixed to mono, with the silence at either end trimmed and the peak brought to full scale (so
//! how loud it was recorded doesn't matter). Stored per file (`sounds`, keyed by its hash) and searched brute force
//! from memory, like `looks`.
//!
//! - **Timbre**: the average spectrum over 32 mel bands, in decibels, less its own mean (its shape, not its level).
//! - **Envelope**: how loud it is at 16 points from its start to its end (a hit's attack and decay, a riser's swell).
//! - **Character**: ten numbers, each scaled to 0..=1: length, brightness, how much the brightness moves, noisiness,
//!   zero crossings, onsets per second after the first, how much the spectrum changes, how much of it is sound rather
//!   than near-silence, loudness range, and how much of it is bass.
//!
//! The same listening gives the measured features kept in the asset's meta (see [`FEATURES`]), shown in its details
//! and made into tags: how it starts and ends, how bright and how noisy it is, whether its pitch rises or falls, and
//! whether it loops cleanly. These are what a description of a sound ("a deep whoosh that swells") is matched with.

mod likeness;
mod listening;

use std::collections::{HashMap, HashSet};
use std::sync::{PoisonError, RwLock};

use anyhow::Result;
use rusqlite::params;
use serde_json::{Map, Value};

use crate::{AppState, db};
use likeness::{DIMS, similarity};
pub(crate) use likeness::{print_secs, same_print};
pub(crate) use listening::{FEATURES, Fft, analyze, decode_mono, listen, measure};
use listening::{Hearing, MAX_BYTES};

/// Bump when the features change; rows from another version are worked out again by the next sounds job.
pub(crate) const VERSION: i64 = 3;
/// The assets this covers.
pub(crate) const KINDS_SQL: &str = "a.kind = 'audio'";

const SIMILAR_MAX: usize = 1000;
const SIMILAR_FLOOR: f32 = 0.6;
/// How alike two files' descriptors must be to compare their prints for a near copy.
const NEAR_FLOOR: f32 = 0.9;
/// Every sound's descriptor, in memory, loaded from the `sounds` table on first use and kept up to date by the sounds
/// job.
#[derive(Debug, Default)]
pub(crate) struct Index(RwLock<Option<HashMap<String, Box<[i8]>>>>);

impl Index {
    /// An empty index that's loaded, for tests.
    #[cfg(test)]
    pub(crate) fn loaded() -> Self {
        Self(RwLock::new(Some(HashMap::new())))
    }

    /// Add a file's descriptor, for tests.
    #[cfg(test)]
    pub(crate) fn insert(&self, hash: &str, descriptor: Vec<i8>) {
        self.write(|m| {
            m.insert(hash.to_owned(), descriptor.into_boxed_slice());
        });
    }

    fn read<R>(&self, f: impl FnOnce(&HashMap<String, Box<[i8]>>) -> R) -> Option<R> {
        self.0.read().unwrap_or_else(PoisonError::into_inner).as_ref().map(f)
    }

    fn write(&self, f: impl FnOnce(&mut HashMap<String, Box<[i8]>>)) {
        if let Some(m) = self.0.write().unwrap_or_else(PoisonError::into_inner).as_mut() {
            f(m);
        }
    }

    /// Load the index if it isn't yet.
    pub(crate) async fn ensure(&self, state: &AppState) -> Result<()> {
        if self.read(|_| ()).is_some() {
            return Ok(());
        }
        let loaded = db::call(&state.pool, |c| {
            let mut stmt = c.prepare("SELECT hash, descriptor FROM sounds WHERE version = ?1")?;
            let rows = stmt
                .query_map([VERSION], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?)))?
                .filter_map(|row| {
                    let (hash, d) = row.ok()?;
                    (d.len() == DIMS).then(|| (hash, d.into_iter().map(|b| b as i8).collect()))
                })
                .collect::<HashMap<_, _>>();
            Ok(rows)
        })
        .await?;
        let mut guard = self.0.write().unwrap_or_else(PoisonError::into_inner);
        if guard.is_none() {
            tracing::debug!(sounds = loaded.len(), "sounds index loaded");
            *guard = Some(loaded);
        }
        Ok(())
    }

    /// Files that could be the same sound as `hash`'s, by their descriptors; their prints decide (see [`same_print`]).
    pub(crate) fn near(&self, hash: &str) -> Vec<String> {
        self.read(|m| {
            let Some(target) = m.get(hash) else { return Vec::new() };
            m.iter()
                .filter(|(h, d)| h.as_str() != hash && similarity(target, d) >= NEAR_FLOOR)
                .map(|(h, _)| h.clone())
                .collect()
        })
        .unwrap_or_default()
    }

    /// Whether two files could be the same sound (see [`Index::near`]).
    pub(crate) fn close(&self, a: &str, b: &str) -> bool {
        a == b
            || self
                .read(|m| m.get(a).zip(m.get(b)).is_some_and(|(x, y)| similarity(x, y) >= NEAR_FLOOR))
                .unwrap_or(false)
    }

    /// Files that sound like `hash`'s, best first.
    pub(crate) fn similar(&self, hash: &str) -> Vec<(String, f32)> {
        self.read(|m| {
            let Some(target) = m.get(hash) else { return Vec::new() };
            let hits = m
                .iter()
                .filter(|(h, _)| h.as_str() != hash)
                .map(|(h, d)| (h, similarity(target, d)))
                .filter(|(_, s)| *s >= SIMILAR_FLOOR)
                .collect();
            crate::embed::best(hits, SIMILAR_MAX)
        })
        .unwrap_or_default()
    }
}

const MISSING_SQL: &str =
    "NOT EXISTS (SELECT 1 FROM sounds s WHERE s.hash = a.blob_hash AND s.version = ?1 AND s.print IS NOT NULL)";
const ORPHAN_SQL: &str = "NOT EXISTS (SELECT 1 FROM assets a WHERE a.blob_hash = s.hash AND a.kind = 'audio')";

/// Queue a sounds job when some sound has no descriptor yet (or one no longer belongs to any sound).
pub(crate) async fn schedule(state: &AppState) {
    let queued = db::call(&state.pool, |c| {
        let waiting: bool = c.query_row(
            "SELECT EXISTS (SELECT 1 FROM jobs WHERE kind = ?1 AND status = 'queued')",
            [crate::jobs::SOUNDS],
            |r| r.get(0),
        )?;
        if waiting {
            return Ok(false);
        }
        let work: bool = c.query_row(
            &format!(
                "SELECT EXISTS (SELECT 1 FROM assets a WHERE {KINDS_SQL} AND {MISSING_SQL})
                     OR EXISTS (SELECT 1 FROM sounds s WHERE {ORPHAN_SQL})"
            ),
            [VERSION],
            |r| r.get(0),
        )?;
        if work {
            db::enqueue(c, crate::jobs::SOUNDS, None, &serde_json::json!({}))?;
        }
        Ok(work)
    })
    .await;
    match queued {
        Ok(true) => state.wake.notify_one(),
        Ok(false) => {}
        Err(e) => tracing::warn!(error = ?e, "could not queue the sounds job"),
    }
}

/// Files listened to per round (read, analyzed, written).
const BATCH: usize = 32;

/// Work out the descriptor of every sound that has none, and forget those of files that are no longer sounds.
pub(crate) async fn run_job(state: &AppState) -> Result<()> {
    state.sounds.ensure(state).await?;
    let gone: Vec<String> = db::call(&state.pool, |c| {
        let gone = c
            .prepare(&format!("SELECT hash FROM sounds s WHERE {ORPHAN_SQL}"))?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        c.execute(&format!("DELETE FROM sounds AS s WHERE {ORPHAN_SQL}"), [])?;
        Ok(gone)
    })
    .await?;
    state.sounds.write(|m| {
        for h in &gone {
            m.remove(h);
        }
    });

    // Files that couldn't be read this time (storage unreachable): left for a later run.
    let mut unreadable = HashSet::new();
    let mut done = 0usize;
    loop {
        let skip = unreadable.clone();
        let todo: Vec<(String, String)> = db::call(&state.pool, move |c| {
            let rows = c
                .prepare(&format!(
                    "SELECT a.blob_hash, MIN(a.ext) FROM assets a WHERE {KINDS_SQL} AND {MISSING_SQL} GROUP BY a.blob_hash"
                ))?
                .query_map([VERSION], |r| Ok((r.get(0)?, r.get(1)?)))?
                .collect::<rusqlite::Result<Vec<(String, String)>>>()?;
            Ok(rows.into_iter().filter(|(h, _)| !skip.contains(h)).collect())
        })
        .await?;
        if todo.is_empty() {
            return Ok(());
        }
        let total = done + todo.len();
        for chunk in todo.chunks(BATCH) {
            crate::activity::job_progress(crate::jobs::SOUNDS, None, done, total, "sounds");
            let mut files = Vec::with_capacity(chunk.len());
            for (hash, ext) in chunk {
                match state.blobs.head(hash, MAX_BYTES).await {
                    Ok(bytes) => files.push((hash.clone(), ext.clone(), bytes::Bytes::from(bytes))),
                    Err(e) => {
                        tracing::debug!(hash, error = %e, "sound unreadable");
                        unreadable.insert(hash.clone());
                    }
                }
            }
            let computed = crate::heavy::run("sounds", move || analyze_all(&files)).await?;
            save(state, &computed).await?;
            done += chunk.len();
            state.sounds.write(|m| {
                for (hash, heard) in computed {
                    if let Some(h) = heard {
                        m.insert(hash, h.descriptor.into_boxed_slice());
                    }
                }
            });
        }
        crate::activity::job_progress(crate::jobs::SOUNDS, None, done, total, "sounds");
    }
}

/// Keep the descriptors, and put the measured features in the meta of every sound with that file (unless they're
/// there already, as for a file analyzed since they were added), with the tags they bring.
async fn save(state: &AppState, computed: &[Heard]) -> Result<()> {
    let rows = computed.to_vec();
    db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        {
            let mut stmt =
                tx.prepare("INSERT OR REPLACE INTO sounds (hash, version, descriptor, print) VALUES (?1, ?2, ?3, ?4)")?;
            for (hash, heard) in &rows {
                // Not a sound this can decode: an empty row, so it isn't tried again.
                let d: Vec<u8> = heard.as_ref().map_or_default(|h| h.descriptor.iter().map(|&v| v as u8).collect());
                let print: &[u8] = heard.as_ref().map_or(&[], |h| &h.print);
                stmt.execute(params![hash, VERSION, d, print])?;
                if let Some(h) = heard {
                    put_features(&tx, hash, &h.features)?;
                }
            }
        }
        tx.commit()?;
        Ok(())
    })
    .await
}

/// Put a file's measured features in the meta of every sound with it, taking out those not measured, and give them the
/// tags the features bring in place of those the old ones brought. Sounds that have them already are left alone.
fn put_features(c: &rusqlite::Connection, hash: &str, features: &Map<String, Value>) -> Result<()> {
    let mut patch: Map<String, Value> = FEATURES.iter().map(|k| ((*k).to_owned(), Value::Null)).collect();
    patch.extend(features.clone());
    let changed = c
        .prepare_cached(&format!(
            "UPDATE assets AS a SET meta = json_patch(meta, ?2)
              WHERE a.blob_hash = ?1 AND {KINDS_SQL} AND json_patch(meta, ?2) IS NOT json(meta)
              RETURNING id, meta"
        ))?
        .query_map(params![hash, Value::Object(patch).to_string()], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let measured = crate::tagger::MEASURED_SOUND_TAGS;
    let prefixes = crate::tagger::MUSIC_TAG_PREFIXES;
    let forget = format!(
        "DELETE FROM asset_tags WHERE asset_id = ? AND source != 'human' AND (tag IN ({}) OR {})",
        measured.map(|_| "?").join(", "),
        prefixes.map(|_| "substr(tag, 1, length(?)) = ?").join(" OR ")
    );
    for (id, meta) in changed {
        let by_prefix = prefixes.iter().flat_map(|p| [*p, *p]);
        c.execute(&forget, rusqlite::params_from_iter(std::iter::once(id.as_str()).chain(measured).chain(by_prefix)))?;
        let looped: bool =
            c.query_row("SELECT EXISTS (SELECT 1 FROM asset_tags WHERE asset_id = ?1 AND tag = 'loop')", [&id], |r| {
                r.get(0)
            })?;
        let meta: Map<String, Value> = serde_json::from_str(&meta).unwrap_or_default();
        for (tag, source, confidence) in crate::tagger::measured_sound_tags(&meta, looped) {
            db::upsert_tag(c, &id, &tag, source, confidence)?;
        }
        db::reindex_asset(c, &id)?;
    }
    Ok(())
}

/// A file's hash, and what listening to it gave when it could be listened to.
type Heard = (String, Option<Hearing>);

/// Listen on a few threads (half the cores: this is background work).
fn analyze_all(files: &[(String, String, bytes::Bytes)]) -> Vec<Heard> {
    let threads = std::thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1));
    let per = files.len().div_ceil(threads).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = files
            .chunks(per)
            .map(|chunk| {
                scope.spawn(move || {
                    crate::heavy::lower_priority();
                    chunk.iter().map(|(h, ext, b)| (h.clone(), analyze(b.clone(), Some(ext)))).collect::<Vec<_>>()
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    })
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use serde_json::json;

    use super::*;
    use crate::sounds::likeness::describe;
    use crate::sounds::listening::hear;

    const RATE: u32 = 44_100;

    /// A repeatable noise source.
    struct Noise(u32);
    impl Noise {
        fn next(&mut self) -> f32 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 17;
            self.0 ^= self.0 << 5;
            (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0
        }
    }

    /// A footstep-ish thud: a short burst of low-passed noise that dies away.
    fn thud(seed: u32, secs: f32, cutoff: f32) -> Vec<f32> {
        let mut n = Noise(seed);
        let a = (-2.0 * PI * cutoff / RATE as f32).exp();
        let mut y = 0.0;
        (0..(secs * RATE as f32) as usize)
            .map(|i| {
                y = a * y + (1.0 - a) * n.next();
                y * (-(i as f32) / (0.04 * RATE as f32)).exp() * 4.0
            })
            .collect()
    }

    /// A laser: a falling tone.
    fn zap(from: f32, to: f32, secs: f32) -> Vec<f32> {
        let len = (secs * RATE as f32) as usize;
        let mut phase = 0.0f32;
        (0..len)
            .map(|i| {
                let t = i as f32 / len as f32;
                phase += 2.0 * PI * (from + (to - from) * t) / RATE as f32;
                phase.sin() * (1.0 - t)
            })
            .collect()
    }

    /// Steady rain: white noise for a few seconds.
    fn rain(seed: u32, secs: f32) -> Vec<f32> {
        let mut n = Noise(seed);
        (0..(secs * RATE as f32) as usize).map(|_| n.next() * 0.5).collect()
    }

    fn d(x: &[f32]) -> Vec<i8> {
        describe(x, RATE).expect("a descriptor")
    }

    #[test]
    fn like_sounds_like_like() {
        let step_a = d(&thud(1, 0.3, 300.0));
        let step_b = d(&thud(7, 0.35, 350.0));
        let zap_a = d(&zap(2000.0, 300.0, 0.4));
        let zap_b = d(&zap(1800.0, 250.0, 0.5));
        let rain_a = d(&rain(3, 4.0));
        let rain_b = d(&rain(9, 5.0));

        let s = similarity;
        assert!(s(&step_a, &step_b) > s(&step_a, &zap_a), "{} vs {}", s(&step_a, &step_b), s(&step_a, &zap_a));
        assert!(s(&step_a, &step_b) > s(&step_a, &rain_a));
        assert!(s(&zap_a, &zap_b) > s(&zap_a, &step_a));
        assert!(s(&zap_a, &zap_b) > s(&zap_a, &rain_a));
        assert!(s(&rain_a, &rain_b) > s(&rain_a, &step_a));
        for (a, b) in [(&step_a, &step_b), (&zap_a, &zap_b), (&rain_a, &rain_b)] {
            assert!(s(a, b) >= SIMILAR_FLOOR, "a variation gets in: {}", s(a, b));
        }
        assert!(s(&step_a, &rain_a) < SIMILAR_FLOOR, "a thud isn't rain: {}", s(&step_a, &rain_a));
        assert!(s(&zap_a, &rain_a) < SIMILAR_FLOOR, "a laser isn't rain: {}", s(&zap_a, &rain_a));
    }

    #[test]
    fn what_a_low_rate_cant_hold_isnt_alike() {
        // At half the rate, neither file has anything above 11 kHz: that's not something they share.
        let half = |x: Vec<f32>| -> Vec<i8> {
            let x: Vec<f32> = x.iter().step_by(2).copied().collect();
            describe(&x, RATE / 2).expect("a descriptor")
        };
        let (step, rain) = (half(thud(1, 0.3, 300.0)), half(rain(3, 4.0)));
        assert!(similarity(&step, &rain) < SIMILAR_FLOOR, "a thud isn't rain: {}", similarity(&step, &rain));
        // And the same sound at either rate is still alike.
        let full = d(&thud(7, 0.35, 350.0));
        assert!(similarity(&step, &full) >= SIMILAR_FLOOR, "{}", similarity(&step, &full));
    }

    #[test]
    fn loudness_and_silence_around_it_dont_matter() {
        let step = thud(1, 0.3, 300.0);
        let mut padded = vec![0.0; RATE as usize / 2];
        padded.extend(step.iter().map(|s| s * 0.1));
        padded.extend(vec![0.0; RATE as usize]);
        assert!(similarity(&d(&step), &d(&padded)) > 0.97, "{}", similarity(&d(&step), &d(&padded)));
    }

    #[test]
    fn prints_tell_a_copy_from_another_take() {
        let p = |x: &[f32], rate: u32| hear(x, rate, true).expect("a print").print;
        let half = |x: &[f32]| -> Vec<f32> { x.chunks(2).map(|c| c.iter().sum::<f32>() / c.len() as f32).collect() };
        let eight_bit = |x: &[f32]| -> Vec<f32> { x.iter().map(|s| (s * 127.0).round() / 127.0).collect() };
        let padded = |x: &[f32]| -> Vec<f32> {
            let mut out = vec![0.0; RATE as usize / 3];
            out.extend(x.iter().map(|s| s * 0.25));
            out.extend(vec![0.0; RATE as usize / 5]);
            out
        };
        for (what, sound) in [("thud", thud(1, 0.4, 300.0)), ("zap", zap(2000.0, 300.0, 0.6)), ("rain", rain(3, 4.0))] {
            let original = p(&sound, RATE);
            for (how, copy, rate) in [
                ("half the rate", half(&sound), RATE / 2),
                ("8-bit", eight_bit(&sound), RATE),
                ("quieter, with silence around it", padded(&sound), RATE),
            ] {
                assert!(same_print(&original, &p(&copy, rate)), "{what} at {how}");
            }
        }

        let step = p(&thud(1, 0.4, 300.0), RATE);
        let laser = p(&zap(2000.0, 300.0, 0.6), RATE);
        // Another take: another length, or the same length but brighter.
        assert!(!same_print(&step, &p(&thud(7, 0.45, 300.0), RATE)), "a longer step");
        assert!(!same_print(&step, &p(&thud(1, 0.4, 900.0), RATE)), "a brighter step");
        assert!(!same_print(&laser, &p(&zap(1500.0, 300.0, 0.6), RATE)), "a lower laser");
        assert!(!same_print(&laser, &p(&zap(300.0, 2000.0, 0.6), RATE)), "a laser turned around");
        assert!(!same_print(&step, &laser));
    }

    #[test]
    fn silence_and_the_shortest_sounds() {
        assert!(describe(&vec![0.0; 1000], RATE).is_none());
        assert_eq!(describe(&[0.5, -0.5, 0.25], RATE).map(|d| d.len()), Some(DIMS));
    }

    #[test]
    fn decodes_a_wav_cut_short() {
        let samples: Vec<i32> = thud(4, 0.5, 400.0).iter().map(|s| (s.clamp(-1.0, 1.0) * 32_767.0) as i32).collect();
        let wav = crate::audio::write_wav(&crate::audio::Audio { samples, channels: 1, rate: RATE, bits: 16 });
        let whole = analyze(bytes::Bytes::from(wav.clone()), Some("wav")).expect("the whole file");
        let cut = analyze(bytes::Bytes::from(wav[..wav.len() * 2 / 3].to_vec()), Some("wav")).expect("its start");
        assert_eq!(whole.descriptor.len(), DIMS);
        assert_eq!(cut.descriptor.len(), DIMS);
        assert!(analyze(bytes::Bytes::from_static(b"not a sound at all"), Some("wav")).is_none());
    }

    /// A riser: noise that gets louder and brighter for two seconds.
    fn riser(seed: u32) -> Vec<f32> {
        let mut n = Noise(seed);
        let len = 2 * RATE as usize;
        let mut y = 0.0;
        (0..len)
            .map(|i| {
                let t = i as f32 / len as f32;
                let a = (-2.0 * PI * (200.0 * 32f32.powf(t)) / RATE as f32).exp();
                y = a * y + (1.0 - a) * n.next();
                y * t * t * 4.0
            })
            .collect()
    }

    /// A steady tone, `secs` long.
    fn tone(hz: f32, len: usize) -> Vec<f32> {
        (0..len).map(|i| (2.0 * PI * hz * i as f32 / RATE as f32).sin() * 0.5).collect()
    }

    fn f(x: &[f32]) -> Map<String, Value> {
        hear(x, RATE, true).expect("features").features
    }

    #[test]
    fn measured_features() {
        let num = |m: &Map<String, Value>, k: &str| m[k].as_f64().unwrap();
        let tags = |m: &Map<String, Value>| {
            crate::tagger::measured_sound_tags(m, false).into_iter().map(|t| t.0).collect::<Vec<_>>()
        };

        // A thud: a hit that's gone in a fifth of a second, neither bright nor dark, and doesn't loop.
        let thud = f(&thud(1, 0.3, 300.0));
        assert_eq!(thud["envelope"], "hit");
        assert!(num(&thud, "attack_ms") <= 10.0 && (100.0..300.0).contains(&num(&thud, "tail_ms")), "{thud:?}");
        assert!(num(&thud, "sweep_octaves").abs() < 1.0 && thud.get("loops_cleanly").is_none(), "{thud:?}");
        assert_eq!(tags(&thud), ["envelope:hit"]);

        // A laser: a hit, a pure tone, falling.
        let zap = f(&zap(2000.0, 300.0, 0.4));
        assert!(num(&zap, "sweep_octaves") < -1.0 && num(&zap, "noisiness") < 0.05, "{zap:?}");
        assert_eq!(tags(&zap), ["envelope:hit", "tone:tonal", "pitch:falling"]);

        // Rain: steady, bright, noise, and a file of it loops (white noise has no seam to hear).
        let rain = f(&rain(3, 4.0));
        assert!(num(&rain, "brightness_hz") > 8000.0 && num(&rain, "noisiness") > 0.6, "{rain:?}");
        assert_eq!(tags(&rain), ["envelope:sustained", "tone:bright", "tone:noisy", "loop"]);

        // A riser: it swells, louder and brighter to its end.
        let riser = f(&riser(5));
        assert!(num(&riser, "attack_ms") > 1000.0 && num(&riser, "tail_ms") < 50.0, "{riser:?}");
        assert_eq!(tags(&riser), ["envelope:swell", "tone:bright", "tone:noisy", "pitch:rising"]);

        // A tone of whole cycles loops; cut off mid-cycle it clicks, so it doesn't; nor does the start of a file.
        let whole = tone(441.0, RATE as usize);
        assert_eq!(f(&whole)["loops_cleanly"], true);
        assert_eq!(num(&f(&whole), "brightness_hz"), 440.0);
        assert!(f(&tone(441.0, RATE as usize + 25)).get("loops_cleanly").is_none());
        assert!(hear(&whole, RATE, false).unwrap().features.get("loops_cleanly").is_none());
    }

    #[test]
    fn a_word_finds_the_sound_it_imitates() {
        let measured = |name: &'static str, x: Vec<f32>| {
            let mut m = f(&x);
            m.insert("duration_secs".into(), json!(x.len() as f32 / RATE as f32));
            (name, crate::imitation::Measured::from_meta(&m).unwrap())
        };
        let sounds = [
            measured("thud", thud(1, 0.3, 300.0)),
            measured("zap", zap(2000.0, 300.0, 0.4)),
            measured("rain", rain(3, 4.0)),
            measured("riser", riser(5)),
        ];
        let best = |word: &str| {
            let shape = crate::imitation::read(word).unwrap();
            let mut scored: Vec<(&str, f32)> =
                sounds.iter().map(|(n, m)| (*n, crate::imitation::score(&shape, m))).collect();
            scored.sort_by(|a, b| b.1.total_cmp(&a.1));
            scored
        };
        for (word, sound) in [
            ("pew", "zap"),
            ("peeoww", "zap"),
            ("doom", "thud"),
            ("bmp", "thud"),
            ("shhhhhhh", "rain"),
            ("sssssss", "rain"),
            ("whoooeeee", "riser"),
        ] {
            let scored = best(word);
            assert_eq!(scored[0].0, sound, "{word}: {scored:?}");
        }
    }

    #[test]
    fn features_measured_later_reach_the_sounds_and_their_tags() {
        let c = db::test_library();
        c.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO assets (id, pack_id, blob_hash, path, name, ext, kind, meta, created_at, updated_at) VALUES
               ('a', 'p', 'h', 'laser.wav', 'laser', 'wav', 'audio', '{\"duration_secs\":0.4,\"envelope\":\"swell\"}', 'x', 'x'),
               ('b', 'p', 'h', 'copy/laser.wav', 'laser', 'wav', 'audio', '{}', 'x', 'x'),
               ('c', 'p', 'other', 'rain.wav', 'rain', 'wav', 'audio', '{}', 'x', 'x');
             INSERT INTO asset_tags (asset_id, tag, source, confidence) VALUES
               ('a', 'envelope:swell', 'meta', 1.0), ('a', 'laser', 'rule', 0.8), ('b', 'tone:tonal', 'human', 1.0);",
        )
        .unwrap();
        let laser = f(&zap(2000.0, 300.0, 0.4));
        put_features(&c, "h", &laser).unwrap();
        let meta = |id: &str| -> Map<String, Value> {
            serde_json::from_str(
                &c.query_row("SELECT meta FROM assets WHERE id = ?1", [id], |r| r.get::<_, String>(0)).unwrap(),
            )
            .unwrap()
        };
        let tags = |id: &str| -> Vec<(String, String)> {
            c.prepare("SELECT tag, source FROM asset_tags WHERE asset_id = ?1 ORDER BY tag")
                .unwrap()
                .query_map([id], |r| Ok((r.get(0)?, r.get(1)?)))
                .unwrap()
                .collect::<rusqlite::Result<_>>()
                .unwrap()
        };
        let pair = |t: &str, s: &str| (t.to_owned(), s.to_owned());
        // Both copies of the file get the features, the old envelope's tag goes, and what else they had stays.
        assert_eq!(meta("a")["envelope"], "hit");
        assert_eq!(meta("a")["duration_secs"], 0.4);
        assert_eq!(meta("b")["sweep_octaves"], laser["sweep_octaves"]);
        assert!(meta("c").is_empty());
        assert_eq!(
            tags("a"),
            [
                pair("envelope:hit", "meta"),
                pair("laser", "rule"),
                pair("pitch:falling", "meta"),
                pair("tone:tonal", "meta")
            ]
        );
        assert_eq!(
            tags("b"),
            [pair("envelope:hit", "meta"), pair("pitch:falling", "meta"), pair("tone:tonal", "human")]
        );
        let words: String = c.query_row("SELECT tags FROM assets_fts WHERE asset_id = 'a'", [], |r| r.get(0)).unwrap();
        assert!(words.contains("pitch falling"), "{words}");

        // Measured again the same, nothing changes; a person's removal of a tag isn't undone either.
        c.execute("DELETE FROM asset_tags WHERE asset_id = 'a' AND tag = 'pitch:falling'", []).unwrap();
        put_features(&c, "h", &laser).unwrap();
        assert!(!tags("a").contains(&pair("pitch:falling", "meta")));
    }

    #[test]
    fn the_fft_finds_a_tone() {
        let n = 1024;
        let fft = Fft::new(n);
        let mut re: Vec<f32> = (0..n).map(|i| (2.0 * PI * 10.0 * i as f32 / n as f32).cos()).collect();
        let mut im = vec![0.0; n];
        fft.run(&mut re, &mut im);
        let mag: Vec<f32> = re.iter().zip(&im).map(|(r, i)| (r * r + i * i).sqrt()).collect();
        let top = mag[..n / 2].iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).map(|(k, _)| k);
        assert_eq!(top, Some(10));
        assert!((mag[10] - n as f32 / 2.0).abs() < 1.0);
    }
}
