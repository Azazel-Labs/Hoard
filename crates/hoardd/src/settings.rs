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

    /// The AVIF quality thumbnails are sent at when Settings asks for AVIF (about as sharp as the JPEG, and smaller);
    /// none for Lossless, which keeps them exact.
    pub(crate) fn avif_quality(self) -> Option<u8> {
        match self {
            Self::Small => Some(55),
            Self::Balanced => Some(68),
            Self::Sharp => Some(80),
            Self::Lossless => None,
        }
    }
}

/// How often the database backs itself up (Settings → Maintenance).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum BackupEvery {
    #[default]
    Off,
    Daily,
    Weekly,
}

impl BackupEvery {
    /// How long after the last scheduled backup the next is due; none when they're off.
    pub(crate) fn interval(self) -> Option<std::time::Duration> {
        const DAY: u64 = 24 * 60 * 60;
        match self {
            Self::Off => None,
            Self::Daily => Some(std::time::Duration::from_secs(DAY)),
            Self::Weekly => Some(std::time::Duration::from_secs(7 * DAY)),
        }
    }
}

/// How many scheduled backups are kept unless Settings says otherwise.
pub(crate) const DEFAULT_BACKUP_KEEP: u32 = 7;
/// Most scheduled backups Settings can ask to keep.
pub(crate) const MAX_BACKUP_KEEP: u32 = 365;

/// What's been saved from the portal. `None` means "use the flag".
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Saved {
    /// `off`, `auto` or a path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) blender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unreal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unreal_image: Option<String>,
    /// The GitHub account Epic's private Unreal Editor images are downloaded as, and its token (with
    /// `read:packages`). Empty: none (`GHCR_USER` / `GHCR_TOKEN_FILE`, or crane's own sign-in, if set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ghcr_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ghcr_token: Option<String>,
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
    /// Send browsers that read AVIF their thumbnails as AVIF, made from the stored ones and kept with the conversions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb_avif: Option<bool>,
    /// Search the stores for new packs' pages in the background, attaching the ones that are certain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) store_lookup: Option<bool>,
    /// Image search (SigLIP 2): search pictures by what they show. Off until turned on, as the model is a big download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip: Option<bool>,
    /// Image search through a hosted service (`voyage`, `cohere` or `jina`) instead of the model here, which sends
    /// every thumbnail to it. Empty: here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip_hosted: Option<String>,
    /// Empty: the provider's usual model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip_hosted_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip_hosted_key: Option<String>,
    /// In place of the provider's own address, for a gateway in between. Empty: the provider's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip_hosted_url: Option<String>,
    /// Sound search (CLAP): search sounds by what they sound like, in words. Off until turned on, like image search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clap: Option<bool>,
    /// Animations' tiles play them when pointed at (`motion`): off until turned on, as each takes room and rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) motion_thumbs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) conversion_cache_mb: Option<u64>,
    /// Back the database up on a schedule, as `hoard-auto-<time>.db` in the backups folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) backup_every: Option<BackupEvery>,
    /// How many scheduled backups are kept: older ones go. Backups made by hand are never removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) backup_keep: Option<u32>,
    /// A folder each scheduled backup is also copied to (another disk, a synced folder), kept to the same number.
    /// Empty: none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) backup_copy_to: Option<String>,
    /// How texture maps are named in downloads, bundles and manifests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) texture_names: Option<crate::naming::TextureNames>,
    /// Settings → Conversion: files converted in place to the format a rule names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) conversion_rules: Option<crate::reformat::Rules>,
    /// Settings → Authentication: signing in through a directory or OAuth, and how long sessions last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) auth: Option<crate::signin::AuthConfig>,
    /// Settings → Locations → S3: the service `s3://bucket` locations are in. An empty address means AWS itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) s3_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) s3_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) s3_access_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) s3_secret_key: Option<String>,
    /// Extra headers on every request, one `Name=value` a line (Cloudflare Access's, say). Empty: none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) s3_headers: Option<String>,
}

/// The saved settings that are secrets: sealed in the desktop app (see [`crate::secrets`]).
const SECRETS: &[&str] = &[
    "llm_api_key",
    "clip_hosted_key",
    "update_hook_auth",
    "s3_access_key",
    "s3_secret_key",
    "s3_headers",
    "ghcr_token",
];

/// Reads a retired setting and throws it away.
fn ignore<'de, D: serde::Deserializer<'de>>(d: D) -> Result<(), D::Error> {
    serde::de::IgnoredAny::deserialize(d).map(drop)
}

/// The settings in effect: saved ones over the flags.
#[derive(Clone, Debug)]
#[expect(clippy::struct_excessive_bools, reason = "each bool is an independent setting")]
pub(crate) struct Effective {
    pub(crate) blender: String,
    pub(crate) unreal: String,
    pub(crate) unreal_image: String,
    /// Who Unreal Editor is downloaded as, when both a user and a token are saved.
    pub(crate) ghcr: Option<crate::unreal_editor::Ghcr>,
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
    /// Where image search runs when it isn't here.
    pub(crate) clip_hosted: Option<crate::hosted::Hosted>,
    pub(crate) clap: bool,
    pub(crate) motion_thumbs: bool,
    pub(crate) thumb_quality: ThumbQuality,
    pub(crate) thumb_avif: bool,
    pub(crate) conversion_cache_mb: u64,
    pub(crate) backup_every: BackupEvery,
    pub(crate) backup_keep: u32,
    pub(crate) backup_copy_to: Option<PathBuf>,
    pub(crate) texture_names: crate::naming::TextureNames,
    pub(crate) conversion_rules: crate::reformat::Rules,
    pub(crate) auth: crate::signin::AuthConfig,
    pub(crate) s3: crate::blobs::S3,
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
            unreal: self.unreal.clone().unwrap_or_else(|| cfg.unreal.clone()),
            unreal_image: self.unreal_image.clone().unwrap_or_else(|| cfg.unreal_image.clone()),
            ghcr: match (self.ghcr_user.as_deref().and_then(non_empty), self.ghcr_token.as_deref().and_then(non_empty))
            {
                (Some(user), Some(token)) => Some(crate::unreal_editor::Ghcr { user, token }),
                _ => None,
            },
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
            clip_hosted: self.clip_hosted.as_deref().and_then(crate::hosted::Provider::parse).map(|provider| {
                crate::hosted::Hosted {
                    provider,
                    model: self
                        .clip_hosted_model
                        .as_deref()
                        .and_then(non_empty)
                        .unwrap_or_else(|| provider.default_model().to_owned()),
                    key: self.clip_hosted_key.as_deref().and_then(non_empty).unwrap_or_default(),
                    url: self
                        .clip_hosted_url
                        .as_deref()
                        .and_then(non_empty)
                        .map(|u| u.trim_end_matches('/').to_owned()),
                }
            }),
            clap: self.clap.unwrap_or(false),
            motion_thumbs: self.motion_thumbs.unwrap_or(false),
            thumb_quality: self.thumb_quality.unwrap_or_default(),
            thumb_avif: self.thumb_avif.unwrap_or(false),
            conversion_cache_mb: self.conversion_cache_mb.unwrap_or(DEFAULT_CONVERSION_CACHE_MB),
            backup_every: self.backup_every.unwrap_or_default(),
            backup_keep: self.backup_keep.unwrap_or(DEFAULT_BACKUP_KEEP).clamp(1, MAX_BACKUP_KEEP),
            backup_copy_to: self.backup_copy_to.as_deref().and_then(non_empty).map(PathBuf::from),
            texture_names: self.texture_names.clone().unwrap_or_default(),
            conversion_rules: self.conversion_rules.clone().unwrap_or_default(),
            auth: self.auth.clone().unwrap_or_default(),
            s3: crate::blobs::S3 {
                endpoint: pick(&self.s3_endpoint, &cfg.s3_endpoint).map(|e| e.trim_end_matches('/').to_owned()),
                region: self.s3_region.as_deref().and_then(non_empty).unwrap_or_else(|| cfg.s3_region.clone()),
                access_key: pick(&self.s3_access_key, &cfg.s3_access_key),
                secret_key: pick(&self.s3_secret_key, &cfg.s3_secret_key),
                headers: match &self.s3_headers {
                    Some(h) => h.lines().filter_map(non_empty).collect(),
                    None => cfg.s3_headers.clone(),
                },
            },
        }
    }

    /// Problems a person can fix, in their words.
    fn check(&self, effective: &Effective) -> Result<(), String> {
        effective.auth.check()?;
        if crate::unreal_editor::image_ref(&effective.unreal_image).is_err() {
            return Err(tr!("error-unreal-image"));
        }
        if effective.name.chars().count() > MAX_NAME || effective.name.chars().any(char::is_control) {
            return Err(tr!("error-server-name", max = MAX_NAME));
        }
        for (name, v) in [("Blender", &self.blender), ("ffmpeg", &self.ffmpeg), ("Unreal Editor", &self.unreal)] {
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
        if let Some(p) = self.clip_hosted.as_deref().map(str::trim)
            && !p.is_empty()
            && crate::hosted::Provider::parse(p).is_none()
        {
            return Err(tr!("error-bad-setting", reason = format!("clip_hosted: {p}")));
        }
        if let Some(h) = &effective.clip_hosted {
            if h.key.is_empty() {
                return Err(tr!("error-hosted-key", provider = h.provider.name()));
            }
            if h.url.as_deref().is_some_and(|u| !is_http(u)) {
                return Err(tr!("error-hosted-address"));
            }
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
        if self.backup_keep.is_some_and(|n| !(1..=MAX_BACKUP_KEEP).contains(&n)) {
            return Err(tr!("error-backup-keep", max = MAX_BACKUP_KEEP));
        }
        if let Some(dir) = &effective.backup_copy_to
            && !(dir.is_absolute() && dir.is_dir())
        {
            return Err(tr!("error-backup-copy-to", path = dir.display().to_string()));
        }
        if self.max_upload_mb == Some(0) || self.max_expand_mb == Some(0) {
            return Err(tr!("error-limits-zero"));
        }
        if let Some(names) = &self.texture_names {
            names.check()?;
        }
        effective.conversion_rules.check()?;
        if let Some(url) = self.s3_endpoint.as_deref().map(str::trim)
            && !url.is_empty()
            && !is_http(url)
        {
            return Err(tr!("error-s3-address"));
        }
        effective.s3.header_map().map_err(|e| e.to_string())?;
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
            Some(t) => serde_json::from_str::<Value>(&t)
                .ok()
                .and_then(|mut v| {
                    crate::secrets::open(&mut v, SECRETS);
                    lenient(&v)
                })
                .unwrap_or_default(),
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
        let mut text = serde_json::to_value(&next).map_err(|e| Err(e.into()))?;
        crate::secrets::seal(&mut text, SECRETS).map_err(|e| Ok(e.to_string()))?;
        let text = text.to_string();
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
    fn hosted_image_search_needs_a_known_provider_and_a_key() {
        let cfg = cfg(&[]);
        let c = db();
        let store = Store::load(&cfg, &c).unwrap();
        let msg = |v: Value| match store.update(&cfg, &c, &v) {
            Err(Ok(m)) => m,
            other => panic!("expected a refusal, got {other:?}"),
        };
        assert!(msg(json!({ "clip_hosted": "voyage" })).contains("Voyage AI"), "no key");
        assert!(msg(json!({ "clip_hosted": "acme", "clip_hosted_key": "k" })).contains("acme"));
        assert!(
            msg(json!({ "clip_hosted": "jina", "clip_hosted_key": "k", "clip_hosted_url": "ftp://x" }))
                .contains("http")
        );
        store
            .update(&cfg, &c, &json!({ "clip_hosted": "cohere", "clip_hosted_key": " k ", "clip_hosted_model": "" }))
            .unwrap();
        let h = store.get().clip_hosted.clone().unwrap();
        assert_eq!(
            (h.provider, h.model.as_str(), h.key.as_str()),
            (crate::hosted::Provider::Cohere, "embed-v4.0", "k")
        );
        assert!(!format!("{:?}", store.get()).contains("\"k\""), "the key stays out of logs");
        assert!(SECRETS.contains(&"clip_hosted_key"));
        store.update(&cfg, &c, &json!({ "clip_hosted": "" })).unwrap();
        assert!(store.get().clip_hosted.is_none(), "empty: the model here");
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
    fn scheduled_backups_keep_some_and_copy_to_a_folder_that_exists() {
        let cfg = cfg(&[]);
        let c = db();
        let store = Store::load(&cfg, &c).unwrap();
        assert_eq!((store.get().backup_every, store.get().backup_keep), (BackupEvery::Off, DEFAULT_BACKUP_KEEP));
        let msg = |v: Value| match store.update(&cfg, &c, &v) {
            Err(Ok(m)) => m,
            other => panic!("expected a refusal, got {other:?}"),
        };
        assert!(msg(json!({ "backup_keep": 0 })).contains("365"));
        assert!(msg(json!({ "backup_copy_to": "relative/folder" })).contains("relative/folder"));
        let dir = tempfile::tempdir().unwrap();
        assert!(msg(json!({ "backup_copy_to": dir.path().join("missing") })).contains("missing"));
        let (_, after) = store
            .update(&cfg, &c, &json!({ "backup_every": "weekly", "backup_keep": 3, "backup_copy_to": dir.path() }))
            .unwrap();
        assert_eq!(after.backup_every.interval(), Some(std::time::Duration::from_hours(7 * 24)));
        assert_eq!((after.backup_keep, after.backup_copy_to.as_deref()), (3, Some(dir.path())));
        let (_, after) = store.update(&cfg, &c, &json!({ "backup_copy_to": " " })).unwrap();
        assert_eq!(after.backup_copy_to, None, "empty: no copy");
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
