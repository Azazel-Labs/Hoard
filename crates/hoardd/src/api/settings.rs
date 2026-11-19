//! The Settings page's reads and writes, and the AI models it can list or fetch.

use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{AppState, db};
use axum::Json;
use axum::extract::State;
use serde::Deserialize;
use serde_json::{Value, json};

/// A tool's line in the settings panel. `describe` is its `tools::Slot::describe` text.
fn tool_setting(value: &str, flag: &str, saved: bool, describe: &str, can_download: bool) -> Value {
    let (state, detail) = match describe {
        "off" | "looking" | "downloading" => (describe, ""),
        s => s.strip_prefix("unavailable: ").map_or(("ready", s), |why| ("unavailable", why)),
    };
    json!({
        "value": value,
        "default": flag,
        "saved": saved,
        "state": state,
        "detail": detail,
        "can_download": can_download,
    })
}

/// Image search's or sound search's switch, where its model is (off, downloading, ready…), and how much of the
/// library it has learned.
fn model_json(value: bool, saved: bool, describe: &str, download_bytes: u64, (done, total): (u64, u64)) -> Value {
    let (status, detail) = match describe {
        s @ ("off" | "looking" | "downloading" | "ready") => (s, ""),
        s => s.strip_prefix("unavailable: ").map_or(("ready", s), |why| ("unavailable", why)),
    };
    json!({
        "value": value,
        "default": false,
        "saved": saved,
        "state": status,
        "detail": detail,
        "download_bytes": download_bytes,
        "done": done,
        "total": total,
    })
}

async fn settings_json(state: &AppState) -> Value {
    let mut out = settings_json_sync(state);
    let (saved, now) = (state.settings.saved(), state.settings.get());
    let pictures = db::call(&state.pool, |c| crate::clip::progress(c)).await.unwrap_or((0, 0));
    let sounds = db::call(&state.pool, |c| crate::clap::progress(c)).await.unwrap_or((0, 0));
    let image_search = &crate::clip::describe();
    out["clip"] = model_json(now.clip, saved.clip.is_some(), image_search, crate::clip::DOWNLOAD_BYTES, pictures);
    // Searches use what the model before learned until every picture is relearned.
    out["clip"]["relearning"] = json!(crate::clip::relearning());
    // A hosted service in place of the model here; its key is never sent back, only whether there is one.
    let hosted = now.clip_hosted.as_ref();
    out["clip"]["hosted"] = json!({
        "provider": hosted.map_or("", |h| h.provider.id()),
        "model": hosted.map_or("", |h| h.model.as_str()),
        "url": hosted.and_then(|h| h.url.as_deref()).unwrap_or(""),
        "has_key": saved.clip_hosted_key.as_deref().is_some_and(|k| !k.trim().is_empty()),
        "providers": crate::hosted::Provider::ALL
            .iter()
            .map(|p| json!({ "id": p.id(), "name": p.name(), "model": p.default_model(), "url": p.base() }))
            .collect::<Vec<_>>(),
    });
    let sound_search = &crate::clap::describe();
    out["clap"] = model_json(now.clap, saved.clap.is_some(), sound_search, crate::clap::DOWNLOAD_BYTES, sounds);
    out
}

fn settings_json_sync(state: &AppState) -> Value {
    let cfg = &state.cfg;
    let saved = state.settings.saved();
    let now = state.settings.get();
    json!({
        "blender": tool_setting(
            &now.blender, &cfg.blender, saved.blender.is_some(), &crate::blender::describe(), crate::blender::can_download() && crate::tools::editor_downloads(cfg),
        ),
        "ffmpeg": tool_setting(
            &now.ffmpeg, &cfg.ffmpeg, saved.ffmpeg.is_some(), &crate::ffmpeg::describe(), crate::ffmpeg::can_download(),
        ),
        "unreal": tool_setting(
            &now.unreal, &cfg.unreal, saved.unreal.is_some(), &crate::unreal_editor::describe(), crate::unreal_editor::can_download() && crate::tools::editor_downloads(cfg),
        ),
        "unreal_image": option(&now.unreal_image, &cfg.unreal_image, saved.unreal_image.is_some()),
        "unreal_convert": {
            "value": now.unreal_convert,
            "default": cfg.unreal_convert,
            "saved": saved.unreal_convert.is_some(),
        },
        // The key is never sent back, only whether there is one.
        "llm": {
            "url": now.llm_url.as_deref().unwrap_or(""),
            "model": now.llm_model.as_deref().unwrap_or(""),
            "has_key": now.llm_api_key.is_some(),
            "vision": now.llm_vision,
            "chatgpt": now.llm_chatgpt,
            "batch": option(now.llm_batch, cfg.llm_batch, saved.llm_batch.is_some()),
            "enabled": now.llm_enabled(),
            // Where the local model server listens.
            "local_url": cfg.local_llm_url,
            "saved": saved.llm_url.is_some() || saved.llm_model.is_some() || saved.llm_api_key.is_some()
                || saved.llm_vision.is_some()
                || saved.llm_chatgpt.is_some(),
            "default": {
                "url": cfg.llm_url.as_deref().unwrap_or(""),
                "model": cfg.llm_model.as_deref().unwrap_or(""),
                "has_key": cfg.llm_api_key.is_some(),
                "vision": cfg.llm_vision,
            },
        },
        // What's called when an update is out and Hoard is idle; the header itself is never sent back.
        "update_hook": {
            "url": now.update_hook_url.as_deref().unwrap_or(""),
            "has_auth": now.update_hook_auth.is_some(),
        },
        // How `s3://` locations are reached. The secret key and the headers are never sent back.
        "s3": {
            "endpoint": now.s3.endpoint.as_deref().unwrap_or(""),
            "region": now.s3.region,
            "access_key": now.s3.access_key.as_deref().unwrap_or(""),
            "has_secret_key": now.s3.secret_key.is_some(),
            "has_headers": !now.s3.headers.is_empty(),
            "saved": saved.s3_endpoint.is_some() || saved.s3_region.is_some() || saved.s3_access_key.is_some()
                || saved.s3_secret_key.is_some()
                || saved.s3_headers.is_some(),
            "built": cfg!(feature = "s3"),
        },
        // Server options that used to need a restart. `default` is what the flags (or env) give.
        "options": {
            "workers": option(now.workers, cfg.workers, saved.workers.is_some()),
            "max_upload_mb": option(now.max_upload_mb, cfg.max_upload_mb, saved.max_upload_mb.is_some()),
            "max_expand_mb": option(now.max_expand_mb, cfg.max_expand_mb, saved.max_expand_mb.is_some()),
            "render": option(now.render, cfg.render, saved.render.is_some()),
            "update_check": option(now.update_check, cfg.update_check, saved.update_check.is_some()),
            "guest_browsing": option(now.guest_browsing, false, saved.guest_browsing.is_some()),
            "store_lookup": option(now.store_lookup, true, saved.store_lookup.is_some()),
            "motion_thumbs": option(now.motion_thumbs, false, saved.motion_thumbs.is_some()),
            "thumb_quality": option(
                now.thumb_quality,
                crate::settings::ThumbQuality::default(),
                saved.thumb_quality.is_some(),
            ),
            "thumb_avif": option(now.thumb_avif, false, saved.thumb_avif.is_some()),
            "conversion_cache_mb": option(
                now.conversion_cache_mb,
                crate::settings::DEFAULT_CONVERSION_CACHE_MB,
                saved.conversion_cache_mb.is_some(),
            ),
            "name": option(&now.name, crate::settings::DEFAULT_NAME, saved.name.is_some()),
            "public_url": option(
                now.public_url.as_deref().unwrap_or(""),
                cfg.public_url.as_deref().unwrap_or(""),
                saved.public_url.is_some(),
            ),
            "soundfont": option(
                now.soundfont.as_deref().map_or_default(|p| p.display().to_string()),
                cfg.soundfont.as_deref().map_or_default(|p| p.display().to_string()),
                saved.soundfont.is_some(),
            ),
        },
        "max_workers": crate::settings::MAX_WORKERS,
        // How texture maps are named in what's sent; the suffixes are every role's, saved or not.
        "texture_names": {
            "on": now.texture_names.on,
            "pattern": now.texture_names.pattern,
            "maps": crate::naming::suffixes(&now.texture_names),
            "endings": now.texture_names.endings,
            "default": crate::naming::defaults(),
            "saved": saved.texture_names.is_some(),
        },
        // Files converted in place, and the formats each target can be made from.
        "conversion_rules": {
            "rules": now.conversion_rules.rules,
            "keep_days": now.conversion_rules.keep_days,
            "default_keep_days": crate::reformat::DEFAULT_KEEP_DAYS,
            "targets": crate::reformat::TARGETS.iter()
                .map(|t| json!({ "to": t, "from": crate::reformat::sources(t) }))
                .collect::<Vec<_>>(),
        },
    })
}

#[derive(Deserialize)]
pub(super) struct ModelsBody {
    url: String,
    /// Absent: the saved key, when `url` is the address in use. Empty: no key.
    api_key: Option<String>,
}

/// The models an AI service offers, for the Settings dialog. Doubles as a connection test.
pub(super) async fn llm_models(State(state): State<AppState>, Json(body): Json<ModelsBody>) -> ApiResult<Json<Value>> {
    let url = body.url.trim();
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(AppError::bad_request(tr!("error-ai-address")));
    }
    let now = state.settings.get();
    let key = match body.api_key {
        Some(k) => Some(k.trim().to_owned()).filter(|k| !k.is_empty()),
        None if now.llm_url.as_deref().map(|u| u.trim_end_matches('/')) == Some(url.trim_end_matches('/')) => {
            now.llm_api_key.clone()
        }
        None => None,
    };
    let models = crate::llm::list_models(&state.http, url, key.as_deref())
        .await
        .map_err(|e| AppError::bad_request(format!("{e:#}")))?;
    Ok(Json(json!({ "models": models })))
}

#[derive(Deserialize)]
pub(super) struct LocalBody {
    url: String,
    /// `list` (the default), `download` or `delete`.
    #[serde(default)]
    action: Option<String>,
    #[serde(default)]
    model: Option<String>,
}

/// The local model server (llama.cpp in router mode): what it has, plus downloading and deleting models. Always
/// answers with the list, and with `reachable: false` and the reason when there's no such server at `url`.
pub(super) async fn llm_local(State(state): State<AppState>, Json(body): Json<LocalBody>) -> ApiResult<Json<Value>> {
    use crate::local_llm;
    let url = body.url.trim();
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(AppError::bad_request(tr!("error-ai-address")));
    }
    let model = body.model.as_deref().map(str::trim).filter(|m| !m.is_empty());
    match (body.action.as_deref().unwrap_or("list"), model) {
        ("list", _) => {}
        ("download", Some(m)) => {
            local_llm::download(&state.http, url, m).await.map_err(|e| AppError::bad_request(format!("{e:#}")))?;
        }
        ("delete", Some(m)) => {
            local_llm::remove(&state.http, url, m).await.map_err(|e| AppError::bad_request(format!("{e:#}")))?;
        }
        _ => return Err(AppError::bad_request(tr!("error-local-llm-action"))),
    }
    Ok(Json(match local_llm::list(&state.http, url).await {
        Ok(models) => json!({ "reachable": true, "models": models, "suggested": local_llm::SUGGESTED }),
        // Just the outermost reason ("no model server answers at …"), not the socket error under it.
        Err(e) => {
            json!({ "reachable": false, "error": e.to_string(), "models": [], "suggested": local_llm::SUGGESTED })
        }
    }))
}

fn option(value: impl serde::Serialize, default: impl serde::Serialize, saved: bool) -> Value {
    json!({ "value": value, "default": default, "saved": saved })
}

pub(super) async fn get_settings(State(state): State<AppState>) -> Json<Value> {
    Json(settings_json(&state).await)
}

/// Change settings: a JSON object of the ones to change, `null` to go back to the server's flag. Whatever changed
/// takes effect at once (a tool switched on starts looking for, or downloading, its program).
pub(super) async fn put_settings(State(state): State<AppState>, Json(patch): Json<Value>) -> ApiResult<Json<Value>> {
    let st = std::sync::Arc::clone(&state);
    let (before, after) = db::call(&state.pool, move |c| match st.settings.update(&st.cfg, c, &patch) {
        Ok(v) => Ok(Ok(v)),
        Err(Ok(msg)) => Ok(Err(msg)),
        Err(Err(e)) => Err(e),
    })
    .await?
    .map_err(AppError::bad_request)?;
    if before.blender != after.blender {
        crate::convert::start_blender(&state, true);
    }
    if before.unreal != after.unreal || before.unreal_image != after.unreal_image {
        crate::unreal_editor::start(
            &after.unreal,
            &after.unreal_image,
            state.cfg.tools_dir(),
            crate::tools::editor_downloads(&state.cfg),
        );
    }
    if before.ffmpeg != after.ffmpeg {
        crate::ffmpeg::start(&after.ffmpeg, state.cfg.tools_dir());
    }
    crate::analyze::set_render_enabled(after.render, after.llm_vision && after.llm_enabled());
    crate::analyze::set_thumb_quality(after.thumb_quality);
    if after.conversion_cache_mb < before.conversion_cache_mb {
        let st = std::sync::Arc::clone(&state);
        tokio::spawn(async move { crate::formats::log_trim(&st).await });
    }
    crate::updates::set_enabled(after.update_check);
    if before.clip != after.clip || before.clip_hosted != after.clip_hosted {
        crate::clip::start(&state, after.clip, after.clip_hosted.clone());
        // Off, or a hosted service does the model's part: the model here goes.
        if !after.clip || after.clip_hosted.is_some() {
            crate::clip::remove_files(&state).await;
        }
    }
    if before.clap != after.clap {
        crate::clap::start(&state, after.clap);
        if !after.clap {
            crate::clap::remove_files(&state).await;
        }
    }
    if (after.motion_thumbs && !before.motion_thumbs) || (after.render && !before.render) {
        crate::motion::kick(&state);
    } else if before.motion_thumbs && !after.motion_thumbs {
        let st = std::sync::Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = crate::motion::remove_all(&st).await {
                tracing::warn!(error = %format!("{e:#}"), "couldn't remove motion thumbnails");
            }
        });
    }
    if after.store_lookup && !before.store_lookup {
        crate::store_lookup::start(&state).await;
    }
    crate::music::configure(after.soundfont.clone());
    if after.s3 != before.s3
        && let Err(e) = state.blobs.set_s3(after.s3.clone()).await
    {
        tracing::warn!(error = %format!("{e:#}"), "couldn't open the library locations again");
    }
    if after.workers > before.workers {
        state.wake.notify_waiters();
    }
    tracing::info!(
        blender = %after.blender,
        ffmpeg = %after.ffmpeg,
        unreal = ?after.unreal_convert,
        llm = after.llm_label(),
        "settings changed"
    );
    Ok(Json(settings_json(&state).await))
}
