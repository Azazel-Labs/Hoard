//! Connecting GitHub for Epic's Unreal Editor images, by GitHub's device flow: Hoard asks GitHub for a short code, the
//! person enters it at github.com/login/device, and Hoard is handed a token that can only read packages. It's kept as
//! the GitHub sign-in in Settings, the same as one typed in by hand. No password reaches Hoard, and the server needn't
//! be reachable from GitHub, so it works on a NAS too.
//!
//! The browser asks [`poll`] every few seconds while the code is showing; each ask asks GitHub at most once, never
//! sooner than GitHub said to.
//!
//! GitHub's tokens from signing in run out after hours, so [`keep_fresh`] swaps each for a new one before it does,
//! with the refresh token GitHub handed over with it (a device-flow sign-in needs no secret for that either).

use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::extract::State;
use axum::http::header;
use serde_json::{Value, json};

use super::settings::put_settings;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::{AppState, db};

/// Connecting GitHub, from Settings → Tools.
pub(super) fn operations() -> Vec<crate::catalog::Operation> {
    use crate::catalog::Access::Admin;
    const SIGN_IN: &str = "signing in to GitHub means entering a code at GitHub in a browser";
    vec![
        Admin
            .post("/api/admin/unreal/github/start", start)
            .named("connect_github", "Start connecting GitHub: a code to enter at GitHub, and where.")
            .for_people(SIGN_IN),
        Admin
            .post("/api/admin/unreal/github/poll", poll)
            .named(
                "check_github_connection",
                "Whether the code from `connect_github` has been entered yet: `waiting`, `expired`, `denied`, or `done`.",
            )
            .for_people(SIGN_IN),
        Admin.post("/api/admin/unreal/github/cancel", cancel).named(
            "cancel_github_connection",
            "Stop waiting for the GitHub code from `connect_github` (its dialog was closed).",
        ),
    ]
}

const DEVICE_CODE: &str = "https://github.com/login/device/code";
const TOKEN: &str = "https://github.com/login/oauth/access_token";
const USER: &str = "https://api.github.com/user";
/// All Hoard asks for: reading Epic's images from ghcr.io.
const SCOPE: &str = "read:packages";

struct Pending {
    device_code: String,
    interval: Duration,
    /// When GitHub may next be asked.
    next: Instant,
    /// When the code stops working.
    until: Instant,
}

/// The sign-in waiting for its code to be entered: one at a time (starting another drops the last).
static PENDING: Mutex<Option<Pending>> = Mutex::new(None);

fn pending() -> std::sync::MutexGuard<'static, Option<Pending>> {
    PENDING.lock().unwrap_or_else(PoisonError::into_inner)
}

fn form(pairs: &[(&str, &str)]) -> String {
    let mut url = reqwest::Url::parse("http://hoard/").expect("valid");
    url.query_pairs_mut().extend_pairs(pairs);
    url.query().unwrap_or_default().to_owned()
}

async fn ask_github(state: &AppState, url: &str, body: String) -> ApiResult<Value> {
    let res = state
        .http
        .post(url)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(header::USER_AGENT, hoard_core::user_agent())
        .timeout(Duration::from_secs(30))
        .body(body)
        .send()
        .await
        .map_err(|_| AppError::unavailable(tr!("error-github-unreachable")))?;
    Ok(res.json().await.unwrap_or(Value::Null))
}

/// Start connecting: a code to enter at GitHub, and where.
pub(super) async fn start(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let client_id = state.cfg.github_client_id.trim();
    if client_id.is_empty() {
        return Err(AppError::bad_request(tr!("error-github-no-app")));
    }
    let got = ask_github(&state, DEVICE_CODE, form(&[("client_id", client_id), ("scope", SCOPE)])).await?;
    let text = |k: &str| got.get(k).and_then(Value::as_str).map(str::to_owned);
    let (Some(device_code), Some(code), Some(url)) = (text("device_code"), text("user_code"), text("verification_uri"))
    else {
        let why = text("error_description").or_else(|| text("error")).unwrap_or_default();
        tracing::warn!(reason = why, "GitHub didn't start a device sign-in");
        return Err(AppError::unavailable(tr!("error-github-refused", reason = why)));
    };
    let interval = Duration::from_secs(got.get("interval").and_then(Value::as_u64).unwrap_or(5).max(1));
    let expires = Duration::from_secs(got.get("expires_in").and_then(Value::as_u64).unwrap_or(900));
    let now = Instant::now();
    *pending() = Some(Pending { device_code, interval, next: now + interval, until: now + expires });
    Ok(Json(json!({ "code": code, "url": url, "interval": interval.as_secs(), "expires_in": expires.as_secs() })))
}

/// A token GitHub handed over, and, when it runs out, what gets a new one and in how many seconds.
#[derive(Debug, PartialEq, Eq)]
struct Tokens {
    access: String,
    refresh: Option<String>,
    expires_in: Option<u64>,
}

impl Tokens {
    /// As saved settings: the GitHub sign-in, with what keeps it fresh (or nothing, for a token that lasts).
    fn saved(&self) -> Value {
        let expires = self.refresh.as_ref().and(self.expires_in).map(|s| now() + s);
        json!({ "ghcr_token": self.access, "ghcr_refresh": self.refresh, "ghcr_expires": expires })
    }
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// What GitHub's answer to a token request means.
#[derive(Debug, PartialEq, Eq)]
enum Answer {
    Token(Tokens),
    Waiting,
    SlowDown(Option<u64>),
    Expired,
    Denied,
    Failed(String),
}

fn answer(got: &Value) -> Answer {
    let text = |k: &str| got.get(k).and_then(Value::as_str).filter(|t| !t.is_empty()).map(str::to_owned);
    if let Some(access) = text("access_token") {
        let expires_in = got.get("expires_in").and_then(Value::as_u64);
        return Answer::Token(Tokens { access, refresh: text("refresh_token"), expires_in });
    }
    match got.get("error").and_then(Value::as_str).unwrap_or_default() {
        "authorization_pending" => Answer::Waiting,
        "slow_down" => Answer::SlowDown(got.get("interval").and_then(Value::as_u64)),
        "expired_token" | "token_expired" => Answer::Expired,
        "access_denied" => Answer::Denied,
        other => Answer::Failed(
            got.get("error_description").and_then(Value::as_str).map_or_else(|| other.to_owned(), str::to_owned),
        ),
    }
}

/// Whether the code has been entered yet: `waiting`, `expired`, `denied`, or `done` with the settings as they now
/// are (the token saved as the GitHub sign-in, and the editor download tried again with it).
pub(super) async fn poll(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let device_code = {
        let mut p = pending();
        let Some(waiting) = p.as_mut() else { return Ok(Json(json!({ "state": "expired" }))) };
        let now = Instant::now();
        if now >= waiting.until {
            *p = None;
            return Ok(Json(json!({ "state": "expired" })));
        }
        if now < waiting.next {
            return Ok(Json(json!({ "state": "waiting" })));
        }
        waiting.next = now + waiting.interval;
        waiting.device_code.clone()
    };
    let client_id = state.cfg.github_client_id.trim();
    let got = ask_github(
        &state,
        TOKEN,
        form(&[
            ("client_id", client_id),
            ("device_code", &device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ]),
    )
    .await?;
    let still = |p: &Option<Pending>| p.as_ref().is_some_and(|w| w.device_code == device_code);
    let token = match answer(&got) {
        Answer::Token(token) => token,
        Answer::Waiting => return Ok(Json(json!({ "state": "waiting" }))),
        Answer::SlowDown(interval) => {
            let mut p = pending();
            if still(&p)
                && let Some(w) = p.as_mut()
            {
                w.interval = interval.map_or(w.interval + Duration::from_secs(5), Duration::from_secs);
                w.next = Instant::now() + w.interval;
            }
            return Ok(Json(json!({ "state": "waiting" })));
        }
        end @ (Answer::Expired | Answer::Denied | Answer::Failed(_)) => {
            let mut p = pending();
            if still(&p) {
                *p = None;
            }
            return match end {
                Answer::Expired => Ok(Json(json!({ "state": "expired" }))),
                Answer::Denied => Ok(Json(json!({ "state": "denied" }))),
                Answer::Failed(why) => Err(AppError::unavailable(tr!("error-github-refused", reason = why))),
                _ => unreachable!(),
            };
        }
    };
    {
        let mut p = pending();
        if !still(&p) {
            // Cancelled or started again meanwhile: this token isn't wanted.
            return Ok(Json(json!({ "state": "expired" })));
        }
        *p = None;
    }
    let user = who(&state, &token.access).await?;
    tracing::info!(user, lasts = token.refresh.is_none(), "GitHub connected for Unreal Editor downloads");
    let mut saved = token.saved();
    saved["ghcr_user"] = json!(user);
    let Json(settings) = put_settings(State(state), Json(saved)).await?;
    Ok(Json(json!({ "state": "done", "user": user, "settings": settings })))
}

/// The GitHub user name a token is of.
async fn who(state: &AppState, token: &str) -> ApiResult<String> {
    let res = state
        .http
        .get(USER)
        .bearer_auth(token)
        .header(header::ACCEPT, "application/vnd.github+json")
        .header(header::USER_AGENT, hoard_core::user_agent())
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|_| AppError::unavailable(tr!("error-github-unreachable")))?;
    let got: Value = res.json().await.unwrap_or(Value::Null);
    got.get("login")
        .and_then(Value::as_str)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| AppError::unavailable(tr!("error-github-refused", reason = "no user name")))
}

/// Renews the GitHub sign-in before it runs out, for as long as Hoard runs. Starts Unreal Editor's download too, once
/// a sign-in that ran out while Hoard was off has been renewed (or couldn't be).
pub(crate) async fn keep_fresh(state: AppState) {
    renew(&state).await;
    let now = state.settings.get();
    crate::unreal_editor::start(
        &now.unreal,
        &now.unreal_image,
        now.ghcr.clone(),
        state.cfg.tools_dir(),
        crate::tools::editor_downloads(&state.cfg),
    );
    loop {
        tokio::time::sleep(Duration::from_mins(20)).await;
        renew(&state).await;
    }
}

/// How long before a token runs out it's renewed.
const EARLY: u64 = 60 * 60;

/// A new token for the saved one if it runs out within the hour. Saved without starting the editor download again: it's
/// the same GitHub account.
async fn renew(state: &AppState) {
    let saved = state.settings.saved();
    let (Some(refresh), Some(expires)) = (saved.ghcr_refresh.filter(|r| !r.is_empty()), saved.ghcr_expires) else {
        return;
    };
    let client_id = state.cfg.github_client_id.trim();
    if expires > now() + EARLY || client_id.is_empty() {
        return;
    }
    let Ok(got) = ask_github(
        state,
        TOKEN,
        form(&[("client_id", client_id), ("grant_type", "refresh_token"), ("refresh_token", &refresh)]),
    )
    .await
    else {
        return tracing::warn!("Couldn't reach GitHub to renew its sign-in; trying again later");
    };
    let Answer::Token(token) = answer(&got) else {
        let why = got.get("error_description").or_else(|| got.get("error")).and_then(Value::as_str).unwrap_or_default();
        return tracing::warn!(reason = why, "GitHub wouldn't renew its sign-in; connect GitHub again in Settings");
    };
    let st = std::sync::Arc::clone(state);
    let saved = db::call(&state.pool, move |c| {
        // Replaced or disconnected meanwhile: leave that be.
        if st.settings.saved().ghcr_refresh.as_deref() != Some(refresh.as_str()) {
            return Ok(false);
        }
        Ok(st.settings.update(&st.cfg, c, &token.saved()).is_ok())
    })
    .await;
    match saved {
        Ok(true) => tracing::info!("GitHub sign-in renewed"),
        Ok(false) => {}
        Err(e) => tracing::warn!(error = %e, "Couldn't save the renewed GitHub sign-in"),
    }
}

/// Stop waiting for the code (the dialog was closed).
pub(super) async fn cancel() -> Json<Value> {
    *pending() = None;
    Json(json!({ "state": "expired" }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_answers_mean_what_github_says_they_do() {
        assert_eq!(
            answer(&json!({ "access_token": "gho_x", "scope": "read:packages" })),
            Answer::Token(Tokens { access: "gho_x".into(), refresh: None, expires_in: None })
        );
        assert_eq!(
            answer(&json!({ "access_token": "ghu_x", "expires_in": 28800, "refresh_token": "ghr_x" })),
            Answer::Token(Tokens { access: "ghu_x".into(), refresh: Some("ghr_x".into()), expires_in: Some(28800) })
        );
        assert_eq!(answer(&json!({ "error": "authorization_pending" })), Answer::Waiting);
        assert_eq!(answer(&json!({ "error": "slow_down", "interval": 10 })), Answer::SlowDown(Some(10)));
        assert_eq!(answer(&json!({ "error": "expired_token" })), Answer::Expired);
        assert_eq!(answer(&json!({ "error": "access_denied" })), Answer::Denied);
        assert_eq!(
            answer(&json!({ "error": "incorrect_client_credentials", "error_description": "The client_id is wrong." })),
            Answer::Failed("The client_id is wrong.".into())
        );
        assert_eq!(answer(&json!({ "access_token": "" })), Answer::Failed(String::new()));
    }

    #[test]
    fn a_token_that_runs_out_is_saved_with_what_renews_it() {
        let lasts = Tokens { access: "gho_x".into(), refresh: None, expires_in: None }.saved();
        assert_eq!(lasts, json!({ "ghcr_token": "gho_x", "ghcr_refresh": null, "ghcr_expires": null }));
        let runs_out =
            Tokens { access: "ghu_x".into(), refresh: Some("ghr_x".into()), expires_in: Some(28800) }.saved();
        assert_eq!(runs_out["ghcr_refresh"], "ghr_x");
        let expires = runs_out["ghcr_expires"].as_u64().unwrap();
        assert!(expires > now() + 28000 && expires <= now() + 28800);
    }

    #[test]
    fn the_form_is_url_encoded() {
        assert_eq!(form(&[("scope", "read:packages"), ("a", "b c")]), "scope=read%3Apackages&a=b+c");
    }
}
