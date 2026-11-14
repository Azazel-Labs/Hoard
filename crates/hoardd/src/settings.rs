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
use crate::i18n::tr;

/// Most background workers the Settings dialog can ask for .
pub(crate) const MAX_WORKERS: usize = 16;

/// How much space converted downloads (a texture as PNG, a model as FBX) may take before the least recently used go.
pub(crate) const DEFAULT_CONVERSION_CACHE_MB: u64 = 10 << 10;

/// How thumbnails and model views are stored. Pictures with transparency, and pixel art, are always PNG.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ThumbQuality {
    /// JPEG at 70: about a fifth of a PNG.
    Small,
    /// JPEG at 82.
    #[default]
    Balanced,
    /// JPEG at 92.
    Sharp,
    /// PNG.
    Lossless,
}

impl ThumbQuality {
    pub(crate) fn jpeg_quality(self) -> Option<u8> {
        match self {
            Self::Small => Some(70),
            Self::Balanced => Some(82),
            Self::Sharp => Some(92),
            Self::Lossless => None,
        }
    }
}

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
    /// Empty turns the LLM tagger off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_model: Option<String>,
    /// Empty means "no key".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_vision: Option<bool>,
    /// Tag through the ChatGPT account signed in from Settings instead of `llm_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_chatgpt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) llm_batch: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) render: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) update_check: Option<bool>,
    /// Called (POST) when a newer release is out and Hoard is idle, for whatever updates the container. Empty: none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) update_hook_url: Option<String>,
    /// The `Authorization` header sent with it (e.g. `Bearer <token>`). Empty: none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) update_hook_auth: Option<String>,
    /// Empty means "find one on the system".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) soundfont: Option<String>,
    /// What this server calls itself in the portal (top bar, tab, sign-in page): telling several hoards apart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    /// Empty means "from the Host header".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) public_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_upload_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_expand_mb: Option<u64>,
    /// Retired: packs have no file limit any more. Still read, so a settings file that has it loads.
    #[serde(skip_serializing, rename = "max_files", deserialize_with = "ignore")]
    pub(crate) retired_max_files: (),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) workers: Option<usize>,
    /// Let people who aren't signed in look around (search, preview, download). Changes still need an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) guest_browsing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_quality: Option<ThumbQuality>,
    /// Search the stores for new packs' pages in the background, attaching the ones that are certain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) store_lookup: Option<bool>,
    /// Image search (CLIP): search pictures by what they show. Off until turned on, as the model is a big download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) conversion_cache_mb: Option<u64>,
    /// How texture maps are named in downloads, bundles and manifests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) texture_names: Option<crate::naming::TextureNames>,
    /// Settings → Authentication: signing in through a directory or OAuth, and how long sessions last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) auth: Option<crate::signin::AuthConfig>,
}

/// Reads a retired setting and throws it away.
fn ignore<'de, D: serde::Deserializer<'de>>(d: D) -> Result<(), D::Error> {
    serde::de::IgnoredAny::deserialize(d).map(drop)
}

/// The settings in effect: saved ones over the flags.
#[derive(Clone, Debug)]
#[expect(clippy::struct_excessive_bools, reason = "each bool is an independent setting")]
pub(crate) struct Effective {
    pub(crate) blender: String,
    pub(crate) ffmpeg: String,
    pub(crate) unreal_convert: UnrealConvert,
    pub(crate) llm_url: Option<String>,
    pub(crate) llm_model: Option<String>,
    pub(crate) llm_api_key: Option<String>,
    pub(crate) llm_vision: bool,
    pub(crate) llm_chatgpt: bool,
    pub(crate) llm_batch: usize,
    pub(crate) render: bool,
    pub(crate) update_check: bool,
    pub(crate) update_hook_url: Option<String>,
    pub(crate) update_hook_auth: Option<String>,
    pub(crate) soundfont: Option<PathBuf>,
    /// [`DEFAULT_NAME`] unless one was given.
    pub(crate) name: String,
    pub(crate) public_url: Option<String>,
    pub(crate) max_upload_mb: u64,
    pub(crate) max_expand_mb: u64,
    pub(crate) workers: usize,
    pub(crate) guest_browsing: bool,
    pub(crate) store_lookup: bool,
    pub(crate) clip: bool,
    pub(crate) thumb_quality: ThumbQuality,
    pub(crate) conversion_cache_mb: u64,
    pub(crate) texture_names: crate::naming::TextureNames,
    pub(crate) auth: crate::signin::AuthConfig,
}

impl Effective {
    pub(crate) fn llm_enabled(&self) -> bool {
        self.llm_model.is_some() && (self.llm_chatgpt || self.llm_url.is_some())
    }

    /// Where tagging requests go, for the log.
    pub(crate) fn llm_label(&self) -> &str {
        match (self.llm_enabled(), self.llm_chatgpt) {
            (false, _) => "off",
            (true, true) => "ChatGPT plan",
            (true, false) => self.llm_url.as_deref().unwrap_or("off"),
        }
    }

    pub(crate) fn max_upload_bytes(&self) -> u64 {
        self.max_upload_mb.saturating_mul(1 << 20)
    }

    pub(crate) fn conversion_cache_bytes(&self) -> u64 {
        self.conversion_cache_mb.saturating_mul(1 << 20)
    }
}

/// What the server is called until it's given a name.
pub(crate) const DEFAULT_NAME: &str = "Hoard";
const MAX_NAME: usize = 40;

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
            llm_chatgpt: self.llm_chatgpt.unwrap_or(false),
            llm_batch: self.llm_batch.unwrap_or(cfg.llm_batch).max(1),
            render: self.render.unwrap_or(cfg.render),
            update_check: self.update_check.unwrap_or(cfg.update_check),
            update_hook_url: self.update_hook_url.as_deref().and_then(non_empty),
            update_hook_auth: self.update_hook_auth.as_deref().and_then(non_empty),
            soundfont: match &self.soundfont {
                Some(s) => non_empty(s).map(PathBuf::from),
                None => cfg.soundfont.clone(),
            },
            name: self.name.as_deref().and_then(non_empty).unwrap_or_else(|| DEFAULT_NAME.to_owned()),
            public_url: pick(&self.public_url, &cfg.public_url),
            max_upload_mb: self.max_upload_mb.unwrap_or(cfg.max_upload_mb),
            max_expand_mb: self.max_expand_mb.unwrap_or(cfg.max_expand_mb),
            workers: self.workers.unwrap_or(cfg.workers).clamp(1, MAX_WORKERS),
            guest_browsing: self.guest_browsing.unwrap_or(false),
            store_lookup: self.store_lookup.unwrap_or(true),
            clip: self.clip.unwrap_or(false),
            thumb_quality: self.thumb_quality.unwrap_or_default(),
            conversion_cache_mb: self.conversion_cache_mb.unwrap_or(DEFAULT_CONVERSION_CACHE_MB),
            texture_names: self.texture_names.clone().unwrap_or_default(),
            auth: self.auth.clone().unwrap_or_default(),
        }
    }

    /// Problems a person can fix, in their words.
    fn check(&self, effective: &Effective) -> Result<(), String> {
        effective.auth.check()?;
        if effective.name.chars().count() > MAX_NAME || effective.name.chars().any(char::is_control) {
            return Err(tr!("error-server-name", max = MAX_NAME));
        }
        for (name, v) in [("Blender", &self.blender), ("ffmpeg", &self.ffmpeg)] {
            if v.as_deref().is_some_and(|s| s.trim().is_empty()) {
                return Err(tr!("error-tool-setting", tool = name));
            }
        }
        if let Some(url) = self.llm_url.as_deref().map(str::trim)
            && !url.is_empty()
            && !is_http(url)
        {
            return Err(tr!("error-ai-address"));
        }
        if (effective.llm_chatgpt || effective.llm_url.is_some()) && effective.llm_model.is_none() {
            return Err(tr!("error-ai-model"));
        }
        if let Some(url) = self.public_url.as_deref().map(str::trim)
            && !url.is_empty()
            && !is_http(url)
        {
            return Err(tr!("error-public-address"));
        }
        if let Some(url) = &effective.update_hook_url
            && !is_http(url)
        {
            return Err(tr!("error-update-hook-address"));
        }
        if effective.update_hook_auth.as_deref().is_some_and(|a| reqwest::header::HeaderValue::from_str(a).is_err()) {
            return Err(tr!("error-update-hook-auth"));
        }
        if let Some(sf) = self.soundfont.as_deref().map(str::trim)
            && !sf.is_empty()
            && !std::path::Path::new(sf).is_file()
        {
            return Err(tr!("error-no-soundfont", path = sf));
        }
        if self.workers.is_some_and(|n| !(1..=MAX_WORKERS).contains(&n)) {
            return Err(tr!("error-workers", max = MAX_WORKERS));
        }
        if self.llm_batch.is_some_and(|n| !(1..=200).contains(&n)) {
            return Err(tr!("error-ai-batch"));
        }
        if self.conversion_cache_mb.is_some_and(|n| n < 100) {
            return Err(tr!("error-conversion-cache"));
        }
        if self.max_upload_mb == Some(0) || self.max_expand_mb == Some(0) {
            return Err(tr!("error-limits-zero"));
        }
        if let Some(names) = &self.texture_names {
            names.check()?;
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
        let patch = patch.as_object().ok_or_else(|| Ok(tr!("error-settings-object")))?;
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
        let next: Saved =
            serde_json::from_value(merged).map_err(|e| Ok(tr!("error-bad-setting", reason = e.to_string())))?;
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
    fn saved_settings_win_and_clearing_goes_back_to_the_default() {
        let mut cfg = cfg(&[]);
        cfg.llm_url = Some("http://env/v1".into());
        cfg.llm_model = Some("envmodel".into());
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
        let mut cfg = cfg(&[]);
        cfg.public_url = Some("https://env.example".into());
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
        assert!(msg(json!({ "max_expand_mb": 0 })).contains("zero"));
        assert!(msg(json!({ "public_url": "hoard.example" })).contains("http"));
        assert!(msg(json!({ "soundfont": "/no/such/font.sf2" })).contains("SoundFont"));
        assert!(msg(json!({ "llm_batch": 0 })).contains("AI request"));
        assert!(msg(json!({ "name": "x".repeat(41) })).contains("name"));
        assert!(
            msg(json!({ "name": "two
lines" }))
            .contains("name")
        );
        assert_eq!(store.get().name, "Hoard");
        let (_, named) = store.update(&cfg, &c, &json!({ "name": "  Studio art  " })).unwrap();
        assert_eq!(named.name, "Studio art");
        let (_, blank) = store.update(&cfg, &c, &json!({ "name": " " })).unwrap();
        assert_eq!(blank.name, "Hoard", "an empty name goes back to Hoard");
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
