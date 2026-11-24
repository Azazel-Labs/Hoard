//! Accounts and access tokens: changing one's own password, an admin's list of accounts (made, given a role, removed),
//! and the tokens scripts and agents sign in with.

use super::{
    Role, Who, check_name, check_new, check_password, digest, hash_password, owner_filter, password_matches,
    random_bytes, session_token, sign_out_everywhere,
};
use crate::AppState;
use crate::catalog::{Access, Operation};
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use crate::signin::Source;
use crate::{db, tools};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json};
use rusqlite::{OptionalExtension, params};
use serde::Deserialize;
use serde_json::{Value, json};

/// Everyone's own password and tokens (an admin sees and can revoke everyone's tokens), and an admin's accounts.
pub(crate) fn operations() -> Vec<Operation> {
    use Access::{Admin, SignedIn};
    const ACCESS: &str = "who may sign in, and what they and their tokens may do, is for a person to decide";
    vec![
        SignedIn
            .post("/api/auth/password", change_password)
            .named("change_password", "Change your own password. Signs out your other sessions.")
            .for_people("a password is for its owner to type"),
        SignedIn.get("/api/admin/tokens", list_tokens).named(
            "list_tokens",
            "Your access tokens (an admin's: everyone's): name, what each may do, and when each was last used.",
        ),
        SignedIn
            .post("/api/admin/tokens", create_token)
            .named(
                "create_token",
                "A new access token. The token itself is in this answer and nowhere else: only its hash is kept.",
            )
            .for_people(ACCESS),
        SignedIn
            .delete("/api/admin/tokens/{id}", delete_token)
            .named("revoke_token", "Revoke an access token: whatever uses it is refused from its next request."),
        SignedIn
            .patch("/api/admin/tokens/{id}", patch_token)
            .named(
                "change_token",
                "Let an access token do more or less, from its next request. Never more than you can do.",
            )
            .for_people(ACCESS),
        Admin
            .get("/api/admin/users", list_users)
            .named("list_accounts", "Every account: name, role, where it signs in from, and when it last did."),
        Admin
            .post("/api/admin/users", create_user)
            .named("create_account", "Make an account with a name, a password and a role.")
            .for_people(ACCESS),
        Admin
            .delete("/api/admin/users/{id}", delete_user)
            .named("delete_account", "Delete an account, its sessions and its tokens. Not your own.")
            .for_people(ACCESS),
        Admin
            .patch("/api/admin/users/{id}", patch_user)
            .named("change_account", "Give an account another role. Not your own.")
            .for_people(ACCESS),
    ]
}

#[derive(Deserialize, schemars::JsonSchema)]
struct PasswordChange {
    current: String,
    new: String,
}

/// Change your own password. Signs out your other sessions.
async fn change_password(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    headers: HeaderMap,
    Json(body): Json<PasswordChange>,
) -> ApiResult<StatusCode> {
    let id = who.person()?;
    check_password(&body.new)?;
    let keep = session_token(&headers).map_or_default(|t| digest(&t));
    let changed = db::call(&state.pool, move |c| {
        let (hash, source): (String, String) =
            c.query_row("SELECT password_hash, source FROM users WHERE id = ?1", [id], |r| Ok((r.get(0)?, r.get(1)?)))?;
        if source != "internal" {
            return Ok(None);
        }
        if !password_matches(&body.current, &hash) {
            return Ok(Some(false));
        }
        c.execute("UPDATE users SET password_hash = ?2 WHERE id = ?1", params![id, hash_password(&body.new)?])?;
        c.execute("DELETE FROM sessions WHERE user_id = ?1 AND token_hash != ?2", params![id, keep])?;
        Ok(Some(true))
    })
    .await?;
    match changed {
        None => Err(AppError::bad_request(tr!("error-password-elsewhere"))),
        Some(false) => Err(AppError::bad_request(tr!("error-current-password"))),
        Some(true) => Ok(StatusCode::NO_CONTENT),
    }
}

async fn list_users(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let me = who.person()?;
    let users = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare("SELECT id, username, created_at, role, source FROM users ORDER BY username")?;
        let rows = stmt
            .query_map([], |r| {
                let id: i64 = r.get(0)?;
                Ok(json!({
                    "id": id,
                    "username": r.get::<_, String>(1)?,
                    "created_at": r.get::<_, String>(2)?,
                    "role": Role::stored(&r.get::<_, String>(3)?).as_str(),
                    "source": r.get::<_, String>(4)?,
                    "you": id == me,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!({ "users": users })))
}

#[derive(Deserialize, schemars::JsonSchema)]
struct NewAccount {
    username: String,
    /// Not for accounts that sign in through a directory or OAuth.
    #[serde(default)]
    password: String,
    /// `internal` (the default), `ldap` or `oauth`: who checks the account's sign-in.
    source: Option<String>,
    /// `view`, `edit` (the default) or `admin`.
    role: Option<String>,
}

async fn create_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(body): Json<NewAccount>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    who.person()?;
    let source = match body.source.as_deref() {
        None => Source::Internal,
        Some(s) => Source::parse(s).ok_or_else(|| AppError::bad_request(tr!("error-account-source")))?,
    };
    let internal = source == Source::Internal;
    let name = if internal { check_new(&body.username, &body.password)? } else { check_name(&body.username)? };
    let role = Role::requested(body.role.as_deref(), Role::Edit)?;
    let username = name.clone();
    let added = db::call(&state.pool, move |c| {
        let hash = if internal { hash_password(&body.password)? } else { String::new() };
        let n = c.execute(
            "INSERT INTO users (username, password_hash, role, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT DO NOTHING",
            params![username, hash, role.as_str(), source.as_str(), db::now()],
        )?;
        Ok((n == 1).then(|| c.last_insert_rowid()))
    })
    .await?;
    let Some(id) = added else {
        return Err(AppError::conflict(tr!("error-account-exists")));
    };
    let made = json!({ "id": id, "username": name, "role": role.as_str(), "source": source.as_str() });
    Ok((StatusCode::CREATED, Json(made)))
}

#[derive(Deserialize, schemars::JsonSchema)]
struct AccountChange {
    role: String,
}

/// Give an account another role. Not your own: there's always an admin, because the one doing this is one.
async fn patch_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
    Json(body): Json<AccountChange>,
) -> ApiResult<StatusCode> {
    if who.person()? == id {
        return Err(AppError::bad_request(tr!("error-own-role")));
    }
    let role = Role::requested(Some(&body.role), Role::View)?;
    let n = db::call(&state.pool, move |c| {
        Ok(c.execute("UPDATE users SET role = ?2 WHERE id = ?1", params![id, role.as_str()])?)
    })
    .await?;
    if n == 0 {
        return Err(AppError::not_found(tr!("error-account-not-found")));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_user(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    if who.person()? == id {
        return Err(AppError::bad_request(tr!("error-remove-self")));
    }
    // Their tokens and connected apps go with them.
    let n = db::call(&state.pool, move |c| {
        sign_out_everywhere(c, id)?;
        Ok(c.execute("DELETE FROM users WHERE id = ?1", [id])?)
    })
    .await?;
    if n == 0 {
        return Err(AppError::not_found(tr!("error-account-not-found")));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_tokens(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let only = owner_filter(&who)?;
    let tokens = db::call(&state.pool, move |c| {
        let mut stmt = c.prepare(
            "SELECT t.id, t.name, t.created_at, t.last_used_at, u.username, t.role, u.role
               FROM access_tokens t LEFT JOIN users u ON u.id = t.created_by
              WHERE ?1 IS NULL OR t.created_by = ?1 ORDER BY t.created_at DESC",
        )?;
        let rows = stmt
            .query_map([only], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "created_at": r.get::<_, String>(2)?,
                    "last_used_at": r.get::<_, Option<String>>(3)?,
                    "created_by": r.get::<_, Option<String>>(4)?,
                    "role": Role::stored(&r.get::<_, String>(5)?).as_str(),
                    // The most it can be given, and the most it can do whatever it was given.
                    "maker_role": Role::stored(&r.get::<_, Option<String>>(6)?.unwrap_or_default()).as_str(),
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    Ok(Json(json!({ "tokens": tokens })))
}

#[derive(Deserialize, schemars::JsonSchema)]
struct NewToken {
    name: String,
    /// Up to the maker's own role, which is the default.
    role: Option<String>,
}

/// A new access token. The token itself is in this response and nowhere else: only its hash is kept.
async fn create_token(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(body): Json<NewToken>,
) -> ApiResult<Json<Value>> {
    let me = who.person()?;
    let name = body.name.trim().to_owned();
    if name.is_empty() || name.chars().count() > 80 {
        return Err(AppError::bad_request(tr!("error-token-name")));
    }
    let role = Role::requested(body.role.as_deref(), who.role())?;
    if role > who.role() {
        return Err(AppError::forbidden(tr!("error-token-role")));
    }
    let token = format!("hoard_{}", tools::hex(&random_bytes::<32>()?));
    let (hash, label) = (digest(&token), name.clone());
    let id = db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO access_tokens (name, token_hash, created_by, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![label, hash, me, role.as_str(), db::now()],
        )?;
        Ok(c.last_insert_rowid())
    })
    .await?;
    Ok(Json(json!({ "id": id, "name": name, "token": token, "role": role.as_str() })))
}

#[derive(Deserialize, schemars::JsonSchema)]
struct TokenChange {
    role: String,
}

/// Let a token do more or less. The token itself isn't shown again or changed, so whatever uses it carries on, with
/// the new role from its next request. Never more than the one changing it can do, nor the account that made it.
async fn patch_token(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
    Json(body): Json<TokenChange>,
) -> ApiResult<StatusCode> {
    let only = owner_filter(&who)?;
    let role = Role::requested(Some(&body.role), Role::View)?;
    if role > who.role() {
        return Err(AppError::forbidden(tr!("error-token-role")));
    }
    let changed = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let maker: Option<String> = tx
            .query_row(
                "SELECT u.role FROM access_tokens t JOIN users u ON u.id = t.created_by
                  WHERE t.id = ?1 AND (?2 IS NULL OR t.created_by = ?2)",
                params![id, only],
                |r| r.get(0),
            )
            .optional()?;
        let Some(maker) = maker else { return Ok(None) };
        if role > Role::stored(&maker) {
            return Ok(Some(false));
        }
        tx.execute("UPDATE access_tokens SET role = ?2 WHERE id = ?1", params![id, role.as_str()])?;
        tx.commit()?;
        Ok(Some(true))
    })
    .await?;
    match changed {
        None => Err(AppError::not_found(tr!("error-token-not-found"))),
        Some(false) => Err(AppError::forbidden(tr!("error-token-role"))),
        Some(true) => Ok(StatusCode::NO_CONTENT),
    }
}

async fn delete_token(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    let only = owner_filter(&who)?;
    let n = db::call(&state.pool, move |c| {
        Ok(c.execute("DELETE FROM access_tokens WHERE id = ?1 AND (?2 IS NULL OR created_by = ?2)", params![id, only])?)
    })
    .await?;
    if n == 0 {
        return Err(AppError::not_found(tr!("error-token-not-found")));
    }
    Ok(StatusCode::NO_CONTENT)
}
