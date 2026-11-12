//! Settings changed from the portal while the server runs (the Settings dialog), on top of the flags and `HOARD_*`
//! variables it started with. A setting saved here wins over its flag; clearing it goes back to the flag.
//!
//! They live in the database (`settings`, one JSON row), so they survive restarts and backups.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::{Config, UnrealConvert};

/// Most background workers the Settings dialog can ask for (`HOARD_WORKERS` can't go past it either).
pub(crate) const MAX_WORKERS: usize = 16;

/// What's been saved from the portal. `None` means "use the flag".
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Saved {
    /// `off`, `auto` or a path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) blender: Option<String>,
    /// `off`, `auto` or a path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ffmpeg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unreal_convert: Option<UnrealConvert>,
    /// Empty turns the LLM tagger off even when `HOARD_LLM_URL` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_model: Option<String>,
    /// Empty means "no key", even when `HOARD_LLM_API_KEY` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_vision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_batch: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) render: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) update_check: Option<bool>,
    /// Empty means "find one on the system".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) soundfont: Option<String>,
    /// Empty means "from the Host header".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) public_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_upload_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_expand_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_files: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) workers: Option<usize>,
}

/// The settings in effect: saved ones over the flags.
#[derive(Clone, Debug)]
pub(crate) struct Effective {
    pub(crate) blender: String,
    pub(crate) ffmpeg: String,
    pub(crate) unreal_convert: UnrealConvert,
    pub(crate) llm_url: Option<String>,
    pub(crate) llm_model: Option<String>,
    pub(crate) llm_api_key: Option<String>,
    pub(crate) llm_vision: bool,
    pub(crate) llm_batch: usize,
    pub(crate) render: bool,
    pub(crate) update_check: bool,
    pub(crate) soundfont: Option<PathBuf>,
    pub(crate) public_url: Option<String>,
    pub(crate) max_upload_mb: u64,
    pub(crate) max_expand_mb: u64,
    pub(crate) max_files: usize,
    pub(crate) workers: usize,
}

impl Effective {
    pub(crate) fn llm_enabled(&self) -> bool {
        self.llm_url.is_some() && self.llm_model.is_some()
    }

    pub(crate) fn max_upload_bytes(&self) -> u64 {
        self.max_upload_mb.saturating_mul(1 << 20)
    }
}

fn non_empty(s: &str) -> Option<String> {
    let s = s.trim();
    (!s.is_empty()).then(|| s.to_owned())
}

fn is_http(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

impl Saved {
    pub(crate) fn over(&self, cfg: &Config) -> Effective {
        let pick = |saved: &Option<String>, flag: &Option<String>| match saved {
            Some(s) => non_empty(s),
            None => flag.clone(),
        };
        Effective {
            blender: self.blender.clone().unwrap_or_else(|| cfg.blender.clone()),
            ffmpeg: self.ffmpeg.clone().unwrap_or_else(|| cfg.ffmpeg.clone()),
            unreal_convert: self.unreal_convert.unwrap_or(cfg.unreal_convert),
            llm_url: pick(&self.llm_url, &cfg.llm_url),
            llm_model: pick(&self.llm_model, &cfg.llm_model),
            llm_api_key: pick(&self.llm_api_key, &cfg.llm_api_key),
            llm_vision: self.llm_vision.unwrap_or(cfg.llm_vision),
            llm_batch: self.llm_batch.unwrap_or(cfg.llm_batch).max(1),
            render: self.render.unwrap_or(cfg.render),
            update_check: self.update_check.unwrap_or(cfg.update_check),
            soundfont: match &self.soundfont {
                Some(s) => non_empty(s).map(PathBuf::from),
                None => cfg.soundfont.clone(),
            },
            public_url: pick(&self.public_url, &cfg.public_url),
            max_upload_mb: self.max_upload_mb.unwrap_or(cfg.max_upload_mb),
            max_expand_mb: self.max_expand_mb.unwrap_or(cfg.max_expand_mb),
            max_files: self.max_files.unwrap_or(cfg.max_files),
            workers: self.workers.unwrap_or(cfg.workers).clamp(1, MAX_WORKERS),
        }
    }

    /// Problems a person can fix, in their words.
    fn check(&self, effective: &Effective) -> Result<(), String> {
        for (name, v) in [("Blender", &self.blender), ("ffmpeg", &self.ffmpeg)] {
            if v.as_deref().is_some_and(|s| s.trim().is_empty()) {
                return Err(format!("{name}: choose off, automatic, or give a path"));
            }
        }
        if let Some(url) = self.llm_url.as_deref().map(str::trim)
            && !url.is_empty()
            && !is_http(url)
        {
            return Err("The AI address should start with http:// or https://".into());
        }
        if effective.llm_url.is_some() && effective.llm_model.is_none() {
            return Err("Choose the model the AI service should use".into());
        }
        if let Some(url) = self.public_url.as_deref().map(str::trim)
            && !url.is_empty()
            && !is_http(url)
        {
            return Err("The public address should start with http:// or https://".into());
        }
        if let Some(sf) = self.soundfont.as_deref().map(str::trim)
            && !sf.is_empty()
            && !std::path::Path::new(sf).is_file()
        {
            return Err(format!("There's no SoundFont at {sf} on the server"));
        }
        if self.workers.is_some_and(|n| !(1..=MAX_WORKERS).contains(&n)) {
            return Err(format!("Background workers: between 1 and {MAX_WORKERS}"));
        }
        if self.llm_batch.is_some_and(|n| !(1..=200).contains(&n)) {
            return Err("Assets per AI request: between 1 and 200".into());
        }
        if self.max_upload_mb == Some(0) || self.max_expand_mb == Some(0) || self.max_files == Some(0) {
            return Err("Limits can't be zero".into());
        }
        Ok(())
    }
}

const KEY: &str = "portal";

/// The saved settings and what's in effect, shared by everything that reads them.
#[derive(Debug)]
pub(crate) struct Store {
    saved: Mutex<Saved>,
    effective: RwLock<Arc<Effective>>,
}

impl Store {
    pub(crate) fn load(cfg: &Config, conn: &Connection) -> Result<Self> {
        let text: Option<String> =
            conn.query_row("SELECT value FROM settings WHERE key = ?1", [KEY], |r| r.get(0)).optional()?;
        let saved = match text {
            // A setting this build doesn't know (saved by a newer one) is dropped rather than refusing to start.
            Some(t) => serde_json::from_str::<Value>(&t).ok().and_then(|v| lenient(&v)).unwrap_or_default(),
            None => Saved::default(),
        };
        let effective = Arc::new(saved.over(cfg));
        Ok(Self { saved: Mutex::new(saved), effective: RwLock::new(effective) })
    }

    pub(crate) fn get(&self) -> Arc<Effective> {
        Arc::clone(&self.effective.read().unwrap_or_else(PoisonError::into_inner))
    }

    pub(crate) fn saved(&self) -> Saved {
        self.saved.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    /// Merge `patch` (a JSON object of settings; `null` clears one) into what's saved, check it, store it, and put it
    /// in effect. Returns what was in effect before and after. `Err(Ok(msg))` is a problem with the request.
    pub(crate) fn update(
        &self,
        cfg: &Config,
        conn: &Connection,
        patch: &Value,
    ) -> Result<(Arc<Effective>, Arc<Effective>), Result<String, anyhow::Error>> {
        let patch = patch.as_object().ok_or_else(|| Ok("expected a JSON object of settings".to_owned()))?;
        let mut saved = self.saved.lock().unwrap_or_else(PoisonError::into_inner);
        let mut merged = serde_json::to_value(&*saved).map_err(|e| Err(e.into()))?;
        let obj = merged.as_object_mut().ok_or_else(|| Err(anyhow::anyhow!("settings aren't an object")))?;
        for (k, v) in patch {
            if v.is_null() {
                obj.remove(k);
            } else {
                obj.insert(k.clone(), v.clone());
            }
        }
        let next: Saved = serde_json::from_value(merged).map_err(|e| Ok(format!("bad setting: {e}")))?;
        let effective = Arc::new(next.over(cfg));
        next.check(&effective).map_err(Ok)?;
        let text = serde_json::to_string(&next).map_err(|e| Err(e.into()))?;
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT (key) DO UPDATE SET value = excluded.value",
            [KEY, text.as_str()],
        )
        .context("saving settings")
        .map_err(Err)?;
        *saved = next;
        let before = std::mem::replace(
            &mut *self.effective.write().unwrap_or_else(PoisonError::into_inner),
            Arc::clone(&effective),
        );
        Ok((before, effective))
    }
}

/// Read saved settings, skipping any this build doesn't know or can't parse.
fn lenient(v: &Value) -> Option<Saved> {
    let obj = v.as_object()?;
    let mut out = serde_json::Map::new();
    for (k, val) in obj {
        let mut one = serde_json::Map::new();
        one.insert(k.clone(), val.clone());
        if serde_json::from_value::<Saved>(Value::Object(one)).is_ok() {
            out.insert(k.clone(), val.clone());
        }
    }
    serde_json::from_value(Value::Object(out)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use serde_json::json;

    fn cfg(args: &[&str]) -> Config {
        Config::try_parse_from([&["hoardd"], args].concat()).unwrap()
    }

    fn db() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch("CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL) WITHOUT ROWID;").unwrap();
        c
    }

    #[test]
    fn saved_settings_win_and_clearing_goes_back_to_the_flag() {
        let cfg = cfg(&["--llm-url", "http://env/v1", "--llm-model", "envmodel"]);
        let c = db();
        let store = Store::load(&cfg, &c).unwrap();
        assert_eq!(store.get().blender, "off");
        assert!(store.get().llm_enabled());

        let (before, after) = store.update(&cfg, &c, &json!({ "blender": "auto", "llm_url": "" })).unwrap();
        assert_eq!(before.blender, "off");
        assert_eq!(after.blender, "auto");
        assert!(!after.llm_enabled(), "an empty saved URL turns the tagger off");

        // Survives a restart.
        let store = Store::load(&cfg, &c).unwrap();
        assert_eq!(store.get().blender, "auto");

        store.update(&cfg, &c, &json!({ "llm_url": null })).unwrap();
        assert_eq!(store.get().llm_url.as_deref(), Some("http://env/v1"));
        assert_eq!(store.saved(), Saved { blender: Some("auto".into()), ..Saved::default() });
    }

    #[test]
    fn refuses_what_wont_work() {
        let cfg = cfg(&[]);
        let c = db();
        let store = Store::load(&cfg, &c).unwrap();
        let msg = |v: Value| match store.update(&cfg, &c, &v) {
            Err(Ok(m)) => m,
            other => panic!("expected a refusal, got {other:?}"),
        };
        assert!(msg(json!({ "llm_url": "http://x/v1" })).contains("model"));
        assert!(msg(json!({ "llm_url": "ftp://x", "llm_model": "m" })).contains("http"));
        assert!(msg(json!({ "unreal_convert": "sometimes" })).contains("bad setting"));
        assert!(msg(json!({ "nope": 1 })).contains("bad setting"));
        assert!(msg(json!({ "blender": " " })).contains("Blender"));
        assert_eq!(store.saved(), Saved::default(), "nothing was saved");
        assert!(
            store
                .update(&cfg, &c, &json!({ "llm_url": "http://x/v1", "llm_model": "m", "unreal_convert": "replace" }))
                .is_ok()
        );
        assert_eq!(store.get().unreal_convert, UnrealConvert::Replace);
    }

    #[test]
    fn server_options_change_without_a_restart() {
        let cfg = cfg(&["--workers", "2", "--public-url", "https://env.example"]);
        let c = db();
        let store = Store::load(&cfg, &c).unwrap();
        assert_eq!(store.get().workers, 2);
        assert_eq!(store.get().public_url.as_deref(), Some("https://env.example"));

        let (_, after) = store
            .update(&cfg, &c, &json!({ "workers": 6, "public_url": "", "max_upload_mb": 10, "render": false }))
            .unwrap();
        assert_eq!(after.workers, 6);
        assert_eq!(after.public_url, None, "an empty address goes back to the Host header");
        assert_eq!(after.max_upload_bytes(), 10 << 20);
        assert!(!after.render);

        let msg = |v: Value| match store.update(&cfg, &c, &v) {
            Err(Ok(m)) => m,
            other => panic!("expected a refusal, got {other:?}"),
        };
        assert!(msg(json!({ "workers": 0 })).contains("workers"));
        assert!(msg(json!({ "workers": 99 })).contains("workers"));
        assert!(msg(json!({ "max_files": 0 })).contains("zero"));
        assert!(msg(json!({ "public_url": "hoard.example" })).contains("http"));
        assert!(msg(json!({ "soundfont": "/no/such/font.sf2" })).contains("SoundFont"));
        assert!(msg(json!({ "llm_batch": 0 })).contains("AI request"));
        assert_eq!(store.get().workers, 6, "a refusal changes nothing");
    }

    #[test]
    fn settings_from_a_newer_build_dont_stop_this_one() {
        let cfg = cfg(&[]);
        let c = db();
        c.execute("INSERT INTO settings VALUES ('portal', ?1)", [r#"{"blender":"auto","from_the_future":true}"#])
            .unwrap();
        assert_eq!(Store::load(&cfg, &c).unwrap().get().blender, "auto");
    }
}
