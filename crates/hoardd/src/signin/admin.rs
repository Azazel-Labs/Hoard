//! Settings → Authentication's routes: the settings read and saved (secrets kept, never shown back), an LDAP server
//! tried with them, and an OAuth provider's addresses discovered.

use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use axum::extract::State;
use axum::http::{HeaderMap, header};
use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};
use super::oauth::callback_url;
use super::{AuthConfig, Decision, Ldap, decide, http_url};
use crate::AppState;
use crate::catalog::{Access, Operation};
use crate::db;
use crate::directory;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;

/// Settings → Sign-in.
pub(crate) fn operations() -> Vec<Operation> {
    use Access::Admin;
    vec![
        Admin.get("/api/admin/auth", get_config).named(
            "get_sign_in_settings",
            "How people may sign in: passwords, a directory (LDAP), and sign-in providers (OpenID Connect), with \
             secrets left out.",
        ),
        Admin
            .put("/api/admin/auth", put_config)
            .named("change_sign_in_settings", "Change how people may sign in.")
            .for_people("who may sign in, and how, is for a person to decide"),
        Admin.post("/api/admin/auth/ldap/test", test_ldap).named(
            "test_ldap",
            "Try each directory (LDAP) server: reach it, start TLS, and sign in as the account that looks people up; \
             with a name and password, sign in as them too and say what Hoard would make of it.",
        ),
        Admin.post("/api/admin/auth/oauth/discover", discover).named(
            "discover_sign_in_provider",
            "Fill in a sign-in provider's addresses from its OpenID Connect discovery document.",
        ),
    ]
}

async fn get_config(State(state): State<AppState>, headers: HeaderMap) -> Json<Value> {
    let cfg = state.settings.get().auth.clone();
    Json(json!({ "config": cfg.shown(), "callback_url": callback_url(&state, &headers) }))
}

/// Whether a saved reader password may go to `next`: it's the same servers, reached the same way, as the same account.
fn same_target(next: &Ldap, saved: &Ldap) -> bool {
    let servers = |l: &Ldap| {
        l.servers.iter().map(|s| s.trim().to_ascii_lowercase()).filter(|s| !s.is_empty()).collect::<Vec<_>>()
    };
    servers(next) == servers(saved)
        && next.tls == saved.tls
        && next.skip_verify == saved.skip_verify
        && next.ca_cert.trim() == saved.ca_cert.trim()
        && next.bind_dn.trim() == saved.bind_dn.trim()
}

fn give_password_again() -> String {
    tr!("error-ldap-password-again")
}

/// A secret sent empty (as the Settings page shows them) keeps the saved one; `clear_<field>` clears it. Only for
/// where it was given: pointing the servers or token address somewhere else needs it typed again, so whoever can
/// change the settings can't have it sent to a server of theirs.
fn keep_secrets(next: &mut AuthConfig, saved: &AuthConfig, body: &Value) -> Result<(), String> {
    let cleared = |part: &str, field: &str| {
        body.get(part).and_then(|p| p.get(format!("clear_{field}"))).and_then(Value::as_bool).unwrap_or(false)
    };
    for (part, n, s) in [("ldap", &mut next.ldap, &saved.ldap), ("ad", &mut next.ad, &saved.ad)] {
        if !n.bind_password.is_set() && !cleared(part, "bind_password") && s.bind_password.is_set() {
            if same_target(n, s) {
                n.bind_password = s.bind_password.clone();
            } else if !n.anonymous {
                return Err(give_password_again());
            }
        }
    }
    if !next.oauth.client_secret.is_set() && !cleared("oauth", "client_secret") && saved.oauth.client_secret.is_set() {
        if next.oauth.token_url.trim() == saved.oauth.token_url.trim() {
            next.oauth.client_secret = saved.oauth.client_secret.clone();
        } else {
            return Err(tr!("error-oauth-secret-again"));
        }
    }
    Ok(())
}

/// The page's fields, without the ones it adds for itself (`*_set`, `clear_*`).
fn parse_config(body: &Value) -> ApiResult<AuthConfig> {
    let mut body = body.clone();
    for part in ["ldap", "ad", "oauth"] {
        if let Some(o) = body.get_mut(part).and_then(Value::as_object_mut) {
            o.retain(|k, _| !k.ends_with("_set") && !k.starts_with("clear_"));
        }
    }
    serde_json::from_value(body).map_err(|e| AppError::bad_request(tr!("error-auth-settings", reason = e.to_string())))
}

async fn put_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> ApiResult<Json<Value>> {
    let mut next = parse_config(&body)?;
    let saved = state.settings.get().auth.clone();
    keep_secrets(&mut next, &saved, &body).map_err(AppError::bad_request)?;
    next.check().map_err(AppError::bad_request)?;
    let patch = json!({ "auth": next });
    let st = Arc::clone(&state);
    db::call(&state.pool, move |c| match st.settings.update(&st.cfg, c, &patch) {
        Ok(_) => Ok(Ok(())),
        Err(Ok(msg)) => Ok(Err(msg)),
        Err(Err(e)) => Err(e),
    })
    .await?
    .map_err(AppError::bad_request)?;
    tracing::info!(method = ?next.method, "authentication settings changed");
    Ok(get_config(State(state), headers).await)
}

#[derive(Deserialize, schemars::JsonSchema)]
struct LdapTest {
    /// `ldap` or `ad`: which saved settings fill in a password left empty.
    kind: String,
    config: Value,
    username: Option<String>,
    password: Option<String>,
}

/// Try each server: reach it, start TLS, and sign in as the account that looks people up. With a name and password,
/// also sign in as them and say what Hoard would make of it.
async fn test_ldap(State(state): State<AppState>, Json(body): Json<LdapTest>) -> ApiResult<Json<Value>> {
    let mut l: Ldap = {
        let mut c = body.config.clone();
        let clear = c.get("clear_bind_password").and_then(Value::as_bool).unwrap_or(false);
        if let Some(o) = c.as_object_mut() {
            o.retain(|k, _| !k.ends_with("_set") && !k.starts_with("clear_"));
        }
        let mut l: Ldap = serde_json::from_value(c)
            .map_err(|e| AppError::bad_request(tr!("error-ldap-settings", reason = e.to_string())))?;
        let auth = state.settings.get().auth.clone();
        let saved = if body.kind == "ad" { auth.ad } else { auth.ldap };
        if !l.bind_password.is_set() && !clear && saved.bind_password.is_set() {
            if same_target(&l, &saved) {
                l.bind_password = saved.bind_password;
            } else if !l.anonymous {
                return Err(AppError::bad_request(give_password_again()));
            }
        }
        l
    };
    l.servers.retain(|s| !s.trim().is_empty());
    if l.servers.is_empty() {
        return Err(AppError::bad_request(tr!("error-ldap-no-server")));
    }
    let mut servers = Vec::new();
    for s in &l.servers {
        servers.push(match directory::reach(&l, s).await {
            Ok(()) => json!({ "server": s.trim(), "ok": true }),
            Err(e) => json!({ "server": s.trim(), "ok": false, "error": format!("{e:#}") }),
        });
    }
    let user = match (body.username.as_deref().map(str::trim), body.password.as_deref()) {
        (Some(name), Some(pw)) if !name.is_empty() => {
            let roles = state.settings.get().auth.roles.clone();
            Some(match directory::directory_sign_in(&l, name, pw).await {
                Ok(Some(u)) => json!({
                    "ok": true,
                    "name": u.name,
                    "dn": u.dn,
                    "groups": u.groups,
                    "role": match decide(&roles, &u.groups) {
                        Decision::Role(r) => Some(r.as_str()),
                        Decision::Default => Some("default"),
                        Decision::Refused => None,
                    },
                }),
                Ok(None) => {
                    json!({ "ok": false, "error": tr!("error-ldap-wrong-password", name = name) })
                }
                Err(e) => json!({ "ok": false, "error": format!("{e:#}") }),
            })
        }
        _ => None,
    };
    Ok(Json(json!({ "servers": servers, "user": user })))
}

#[derive(Deserialize, schemars::JsonSchema)]
struct Discover {
    issuer: String,
}

/// Fill in a provider's addresses from its OpenID Connect discovery document.
async fn discover(State(state): State<AppState>, Json(body): Json<Discover>) -> ApiResult<Json<Value>> {
    let issuer = body.issuer.trim().trim_end_matches('/');
    if !http_url(issuer) {
        return Err(AppError::bad_request(tr!("error-oauth-issuer")));
    }
    let url = if issuer.contains("/.well-known/") {
        issuer.to_owned()
    } else {
        format!("{issuer}/.well-known/openid-configuration")
    };
    let res = state
        .http
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .timeout(Duration::from_secs(20))
        .send()
        .await
        .map_err(|e| {
            AppError::bad_request(tr!("error-oauth-discovery-unreachable", url = url.as_str(), reason = e.to_string()))
        })?;
    if !res.status().is_success() {
        return Err(AppError::bad_request(tr!(
            "error-oauth-discovery-status",
            url = url.as_str(),
            status = res.status().to_string()
        )));
    }
    let doc: Value =
        res.json().await.map_err(|_| AppError::bad_request(tr!("error-oauth-discovery-bad", url = url.as_str())))?;
    let field = |k: &str| doc.get(k).and_then(Value::as_str).unwrap_or_default().to_owned();
    if field("authorization_endpoint").is_empty() || field("token_endpoint").is_empty() {
        return Err(AppError::bad_request(tr!("error-oauth-discovery-incomplete", url = url.as_str())));
    }
    Ok(Json(json!({
        "authorization_url": field("authorization_endpoint"),
        "token_url": field("token_endpoint"),
        "userinfo_url": field("userinfo_endpoint"),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signin::Secret;

    #[test]
    fn secrets_stay_hidden() {
        let mut cfg = AuthConfig::default();
        cfg.ldap.bind_password = Secret("hunter22".into());
        let shown = cfg.shown();
        assert_eq!(shown["ldap"]["bind_password"], "");
        assert_eq!(shown["ldap"]["bind_password_set"], true);
        assert_eq!(shown["oauth"]["client_secret_set"], false);
        assert!(!format!("{cfg:?}").contains("hunter22"));
        // Sent back as shown, the saved password stays; clearing it is asked for.
        let mut next = parse_config(&shown).unwrap();
        keep_secrets(&mut next, &cfg, &shown).unwrap();
        assert_eq!(next.ldap.bind_password.0, "hunter22");
        let mut asked = shown.clone();
        asked["ldap"]["clear_bind_password"] = json!(true);
        let mut next = parse_config(&asked).unwrap();
        keep_secrets(&mut next, &cfg, &asked).unwrap();
        assert!(!next.ldap.bind_password.is_set());
        // Pointed at another server, the saved password isn't sent there: it's asked for again.
        let mut elsewhere = shown.clone();
        elsewhere["ldap"]["servers"] = json!(["ldap.attacker.example"]);
        let mut next = parse_config(&elsewhere).unwrap();
        assert!(keep_secrets(&mut next, &cfg, &elsewhere).unwrap_err().contains("again"));
        let mut cfg = AuthConfig::default();
        cfg.oauth.token_url = "https://id.example/token".into();
        cfg.oauth.client_secret = Secret("shh".into());
        let mut moved = cfg.shown();
        moved["oauth"]["token_url"] = json!("https://attacker.example/token");
        let mut next = parse_config(&moved).unwrap();
        assert!(keep_secrets(&mut next, &cfg, &moved).unwrap_err().contains("again"));
    }
}
