//! Settings changed from the portal while the server runs (the Settings dialog), on top of the flags and `HOARD_*`
//! variables it started with. A setting saved here wins over its flag; clearing it goes back to the flag.
//!
//! They live in the database (`settings`, one JSON row), so they survive restarts and backups.

use std::sync::{Arc, Mutex, PoisonError, RwLock};

use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::{Config, UnrealConvert};

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
}

impl Effective {
    pub(crate) fn llm_enabled(&self) -> bool {
        self.llm_url.is_some() && self.llm_model.is_some()
    }
}

fn non_empty(s: &str) -> Option<String> {
    let s = s.trim();
    (!s.is_empty()).then(|| s.to_owned())
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
            && !(url.starts_with("http://") || url.starts_with("https://"))
        {
            return Err("The LLM address should start with http:// or https://".into());
        }
        if effective.llm_url.is_some() && effective.llm_model.is_none() {
            return Err("Name the model the LLM endpoint should use".into());
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
    fn settings_from_a_newer_build_dont_stop_this_one() {
        let cfg = cfg(&[]);
        let c = db();
        c.execute("INSERT INTO settings VALUES ('portal', ?1)", [r#"{"blender":"auto","from_the_future":true}"#])
            .unwrap();
        assert_eq!(Store::load(&cfg, &c).unwrap().get().blender, "auto");
    }
}
