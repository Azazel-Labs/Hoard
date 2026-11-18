//! Signing in and out: what the portal asks before anyone has, making the first account, a name and password (or a
//! directory's) traded for a session cookie, and signing out.

use anyhow::anyhow;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use rusqlite::{OptionalExtension as _, params};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{
    COOKIE, CSRF_HEADER, DUMMY_HASH, Role, SECURE_COOKIE, Who, check_new, cookie_value, credential, digest,
    hash_password, lookup, new_session, note_attempt, over_https, password_matches, session_cookie, set_cookie,
    wait_for,
};
use crate::AppState;
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::signin::Source;

/// Open to everyone: what the portal needs to show the sign-in or first-account screen.
pub(crate) fn router() -> Router<AppState> {
    let posts = Router::new()
        .route("/api/auth/setup", post(setup))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route_layer(axum::middleware::from_fn(from_portal));
    Router::new().route("/api/auth/status", get(status)).merge(posts)
}

/// Signing in, making the first account and signing out only come from the portal: another site can't make a browser
/// claim a fresh server, or sign it in as someone else.
async fn from_portal(req: Request, next: Next) -> Response {
    if req.headers().contains_key(CSRF_HEADER) {
        next.run(req).await
    } else {
        AppError::forbidden(tr!("error-not-from-portal")).into_response()
    }
}

async fn status(State(state): State<AppState>, headers: HeaderMap) -> ApiResult<Json<Value>> {
    let cred = credential(&headers);
    let settings = state.settings.get();
    let gate = settings.auth.gate();
    let (accounts, who) = db::call(&state.pool, move |c| {
        let n: i64 = c.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))?;
        let who = match cred {
            Some(cred) => lookup(c, &cred, gate)?,
            None => None,
        };
        let source: Option<String> = match &who {
            Some(Who::User { id, .. }) => {
                c.query_row("SELECT source FROM users WHERE id = ?1", [id], |r| r.get(0)).optional()?
            }
            _ => None,
        };
        Ok((n, who.map(|w| (w, source))))
    })
    .await?;
    let user = who.as_ref().map(|(w, source)| {
        let mut j = w.json();
        if let (Some(source), Some(o)) = (source, j.as_object_mut()) {
            o.insert("source".into(), json!(source));
        }
        j
    });
    Ok(Json(json!({
        "setup_needed": accounts == 0,
        "setup_code": accounts == 0 && setup_code(&state).is_some(),
        "signed_in": who.is_some(),
        "user": user,
        "guest_browsing": settings.guest_browsing,
        "name": settings.name,
        "sign_in": crate::signin::sign_in_options(&settings.auth),
        "desktop": state.cfg.desktop,
    })))
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct SetupBody {
    username: String,
    password: String,
    #[serde(default)]
    code: String,
}

fn signed_in(headers: &HeaderMap, token: &str, name: &str, role: Role) -> ApiResult<Response> {
    let set = session_cookie(headers, token)?;
    let user = json!({ "name": name, "kind": "user", "role": role.as_str() });
    Ok(([(header::SET_COOKIE, set)], Json(json!({ "user": user }))).into_response())
}

/// `HOARD_SETUP_CODE`, when it's set: then only someone who knows it can make the first account.
fn setup_code(state: &AppState) -> Option<&str> {
    state.cfg.setup_code.as_deref().map(str::trim).filter(|s| !s.is_empty())
}

/// The first account: whoever comes first, or with `HOARD_SETUP_CODE` when that's set. Refused once there is one.
async fn setup(State(state): State<AppState>, headers: HeaderMap, Json(body): Json<SetupBody>) -> ApiResult<Response> {
    let name = check_new(&body.username, &body.password)?;
    if let Some(expected) = setup_code(&state) {
        const TRIES: &str = "\u{1}setup";
        if let Some(wait) = wait_for(TRIES) {
            return Err(AppError::too_many(tr!("error-too-many-codes", seconds = wait.as_secs().max(1))));
        }
        // Compared as digests, so how long it takes says nothing about how much of the code was right.
        let right = digest(body.code.trim()) == digest(expected);
        note_attempt(TRIES, right);
        if !right {
            return Err(AppError { status: StatusCode::FORBIDDEN, err: anyhow!(tr!("error-wrong-setup-code")) });
        }
    }
    let made = name.clone();
    let token = db::call(&state.pool, move |c| {
        let hash = hash_password(&body.password)?;
        let added = c.execute(
            "INSERT INTO users (username, password_hash, role, created_at)
             SELECT ?1, ?2, 'admin', ?3 WHERE NOT EXISTS (SELECT 1 FROM users)",
            params![made, hash, db::now()],
        )?;
        if added == 0 {
            return Ok(None);
        }
        new_session(c, c.last_insert_rowid()).map(Some)
    })
    .await?
    .ok_or_else(|| AppError::conflict(tr!("error-already-set-up")))?;
    tracing::info!(user = %name, "first account made");
    signed_in(&headers, &token, &name, Role::Admin)
}

async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<Credentials>,
) -> ApiResult<Response> {
    let name = body.username.trim().to_owned();
    if let Some(wait) = wait_for(&name) {
        return Err(AppError::too_many(tr!("error-too-many-tries", seconds = wait.as_secs().max(1))));
    }
    let auth = state.settings.get().auth.clone();
    let directory = auth.directory().is_some();
    let (lookup_name, password) = (name.clone(), body.password.clone());
    // Hoard's own accounts always sign in with their own password, whatever Settings → Authentication says, so a
    // directory that's down or set up wrong can't lock an admin out. Anyone else goes to the directory, if there is one.
    let found = db::call(&state.pool, move |c| {
        let row: Option<(i64, String, String, String, String)> = c
            .query_row(
                "SELECT id, username, password_hash, role, source FROM users WHERE username = ?1",
                [&lookup_name],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .optional()?;
        if directory && row.as_ref().is_none_or(|r| r.4 == "ldap") {
            return Ok(Err(()));
        }
        let internal = row.filter(|r| r.4 == "internal");
        let hash = internal.as_ref().map_or(DUMMY_HASH.as_str(), |r| r.2.as_str());
        if !password_matches(&password, hash) {
            return Ok(Ok(None));
        }
        let Some((id, username, _, role, _)) = internal else { return Ok(Ok(None)) };
        Ok(Ok(Some((new_session(c, id)?, username, Role::stored(&role)))))
    })
    .await?;
    let found = match found {
        Ok(found) => found,
        Err(()) => directory_login(&state, &auth, &name, &body.password).await?,
    };
    note_attempt(&name, found.is_some());
    let Some((token, username, role)) = found else {
        tracing::info!(user = %name, "failed sign-in");
        return Err(AppError { status: StatusCode::UNAUTHORIZED, err: anyhow!(tr!("error-wrong-password")) });
    };
    signed_in(&headers, &token, &username, role)
}

/// Sign in through LDAP or Active Directory. `None`: not someone the directory knows with that password.
async fn directory_login(
    state: &AppState,
    auth: &crate::signin::AuthConfig,
    name: &str,
    password: &str,
) -> ApiResult<Option<(String, String, Role)>> {
    let Some(dir) = auth.directory() else { return Ok(None) };
    let person = crate::directory::directory_sign_in(dir, name, password).await.map_err(|e| {
        tracing::warn!(user = %name, "the directory couldn't check a sign-in: {e:#}");
        AppError::unavailable(tr!("error-directory-unavailable", reason = format!("{e:#}")))
    })?;
    let Some(person) = person else { return Ok(None) };
    let auth = auth.clone();
    let made = db::call(&state.pool, move |c| {
        Ok(match crate::signin::account_for(c, &auth, Source::Ldap, &person.name, None, &person.groups)? {
            Ok((id, username, role)) => Ok((new_session(c, id)?, username, role)),
            Err(msg) => Err(msg),
        })
    })
    .await?;
    match made {
        Ok(signed_in) => Ok(Some(signed_in)),
        Err(msg) => Err(AppError::forbidden(msg)),
    }
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> ApiResult<Response> {
    // Over HTTPS the browser may hold both cookies (from before and after HTTPS was turned on): end both.
    let secure = over_https(&headers);
    let tokens: Vec<String> =
        [secure.then(|| cookie_value(&headers, SECURE_COOKIE)).flatten(), cookie_value(&headers, COOKIE)]
            .into_iter()
            .flatten()
            .collect();
    db::call(&state.pool, move |c| {
        for token in &tokens {
            c.execute("DELETE FROM sessions WHERE token_hash = ?1", [digest(token)])?;
        }
        Ok(())
    })
    .await?;
    let mut res = StatusCode::NO_CONTENT.into_response();
    res.headers_mut().append(header::SET_COOKIE, set_cookie(false, "", 0)?);
    if secure {
        res.headers_mut().append(header::SET_COOKIE, set_cookie(true, "", 0)?);
    }
    Ok(res)
}
