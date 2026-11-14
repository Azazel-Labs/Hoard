//! Settings → Authentication: how people sign in to the portal besides Hoard's own accounts.
//!
//! - **Internal**: names and passwords kept by Hoard (`auth.rs`). These always work, whatever is chosen here, so an
//!   admin can't be locked out by a directory that's down or set up wrong.
//! - **LDAP** (a custom server or `OpenLDAP`) and **Microsoft Active Directory**: the name and password typed on the
//!   sign-in page are checked by the directory. Hoard finds the person's entry (as a service account, or anonymously),
//!   binds as them with the password, and reads the groups they're in. Several servers can be listed: the next is tried
//!   when one can't be reached. `StartTLS` or LDAPS, with the system's certificates or one given here.
//! - **OAuth** (OpenID Connect, or plain OAuth 2 with a user-info address): the sign-in page offers a button that goes
//!   to the provider and back (authorization code with PKCE).
//!
//! People who sign in these ways get a Hoard account the first time (unless automatic provisioning is off, when an admin
//! adds them first), marked as signing in through the directory or OAuth, with no password of its own. Their role comes
//! from groups when any are listed under Roles (the directory's groups, or the provider's groups claim), checked at each
//! sign-in; otherwise it's the default role, which an admin can change in Settings → Account.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, LazyLock, Mutex, PoisonError};
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, anyhow, bail};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine as _;
use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry, ldap_escape};
use rusqlite::{Connection, OptionalExtension as _, params};
use rustls_pki_types::CertificateDer;
use rustls_pki_types::pem::PemObject as _;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::Digest as _;

use crate::AppState;
use crate::auth::Role;
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;

/// How long to wait for a directory server to answer at all, and for each request after that.
const LDAP_CONNECT: Duration = Duration::from_secs(8);
const LDAP_OP: Duration = Duration::from_secs(15);
/// How long someone has to finish signing in at the OAuth provider.
const OAUTH_WAIT: Duration = Duration::from_secs(600);
const OAUTH_COOKIE: &str = "hoard_oauth";
const CALLBACK: &str = "/api/auth/oauth/callback";

// ---------------------------------------------------------------------------------------------
// Settings

/// A password or client secret: kept, never shown back or logged.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct Secret(pub(crate) String);

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.0.is_empty() { "\"\"" } else { "\"…\"" })
    }
}

impl Secret {
    fn is_set(&self) -> bool {
        !self.0.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Method {
    #[default]
    Internal,
    Ldap,
    /// Microsoft Active Directory: LDAP with its own defaults.
    Ad,
    Oauth,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[expect(clippy::enum_variant_names, reason = "StartTLS is its name")]
pub(crate) enum Tls {
    /// Plain LDAP. Passwords cross the network as they are.
    None,
    /// Plain LDAP upgraded to TLS before anything is sent.
    #[default]
    StartTls,
    /// TLS from the start (`ldaps://`, port 636).
    Ldaps,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Ldap {
    /// `custom` or `openldap`, for the Settings page's defaults. (Active Directory has its own [`AuthConfig::ad`].)
    pub(crate) server_type: String,
    /// `host`, `host:port` or `ldap[s]://host[:port]`, tried in order: the next when one can't be reached.
    pub(crate) servers: Vec<String>,
    pub(crate) tls: Tls,
    /// Accept any certificate. Only for trying things out: anyone in between could read the passwords.
    pub(crate) skip_verify: bool,
    /// The certificate authority (PEM) that signed the servers' certificates, when it isn't one the system trusts.
    pub(crate) ca_cert: String,
    /// Look people up without signing in to the directory first.
    pub(crate) anonymous: bool,
    /// The account that looks people up, e.g. `cn=hoard,ou=services,dc=example,dc=com` (or `hoard@example.com` in AD).
    pub(crate) bind_dn: String,
    pub(crate) bind_password: Secret,
    /// Where people's entries are, e.g. `ou=people,dc=example,dc=com`.
    pub(crate) base_dn: String,
    /// The attribute that holds the name people type: `uid`, or `sAMAccountName` in AD.
    pub(crate) username_attribute: String,
    /// Narrows the search, e.g. `(objectClass=inetOrgPerson)`. Empty: anything with that name.
    pub(crate) user_filter: String,
    /// Where groups are. Empty: only the groups the entry's `memberOf` names.
    pub(crate) group_base_dn: String,
    /// Which groups a person is in: `{dn}` is their entry, `{user}` the name they signed in with.
    pub(crate) group_filter: String,
}

impl Default for Ldap {
    fn default() -> Self {
        Self {
            server_type: "custom".into(),
            servers: Vec::new(),
            tls: Tls::default(),
            skip_verify: false,
            ca_cert: String::new(),
            anonymous: false,
            bind_dn: String::new(),
            bind_password: Secret::default(),
            base_dn: String::new(),
            username_attribute: "uid".into(),
            user_filter: String::new(),
            group_base_dn: String::new(),
            group_filter: "(|(member={dn})(uniqueMember={dn})(memberUid={user}))".into(),
        }
    }
}

impl Ldap {
    fn active_directory() -> Self {
        Self {
            server_type: "ad".into(),
            username_attribute: "sAMAccountName".into(),
            user_filter: "(objectClass=user)".into(),
            // Nested groups too (LDAP_MATCHING_RULE_IN_CHAIN).
            group_filter: "(member:1.2.840.113556.1.4.1941:={dn})".into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct OAuth {
    /// Which preset the Settings page filled in (`custom`, `google`, `microsoft`, `github`, …). Only for the page.
    pub(crate) provider: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: Secret,
    pub(crate) authorization_url: String,
    pub(crate) token_url: String,
    /// Where to ask who signed in. Empty: the ID token's claims (OpenID Connect).
    pub(crate) userinfo_url: String,
    /// Space-separated.
    pub(crate) scopes: String,
    /// The claim that becomes the Hoard account's name (a dotted path reaches inside objects).
    pub(crate) username_claim: String,
    /// The claim that lists the person's groups, for Roles.
    pub(crate) groups_claim: String,
    /// The sign-in button: "Sign in with …".
    pub(crate) button: String,
}

impl Default for OAuth {
    fn default() -> Self {
        Self {
            provider: "custom".into(),
            client_id: String::new(),
            client_secret: Secret::default(),
            authorization_url: String::new(),
            token_url: String::new(),
            userinfo_url: String::new(),
            scopes: "openid profile email".into(),
            username_claim: "preferred_username".into(),
            groups_claim: "groups".into(),
            button: String::new(),
        }
    }
}

/// Groups (by name or DN, any case) whose members get each role. The highest that matches wins.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Roles {
    pub(crate) admin: Vec<String>,
    pub(crate) edit: Vec<String>,
    /// When there are any, only members of a listed group may sign in.
    pub(crate) view: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct AuthConfig {
    pub(crate) method: Method,
    /// Sign people out this long after they signed in. None: only after 30 days unused.
    pub(crate) session_hours: Option<u32>,
    /// Make a Hoard account the first time someone signs in through the directory or OAuth.
    pub(crate) provision: bool,
    /// The role of new accounts when no Roles groups match (and there are none for viewers).
    pub(crate) default_role: String,
    pub(crate) roles: Roles,
    pub(crate) ldap: Ldap,
    pub(crate) ad: Ldap,
    pub(crate) oauth: OAuth,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            method: Method::Internal,
            session_hours: None,
            provision: true,
            default_role: "view".into(),
            roles: Roles::default(),
            ldap: Ldap::default(),
            ad: Ldap::active_directory(),
            oauth: OAuth::default(),
        }
    }
}

/// How long an OAuth sign-in lasts when Settings doesn't say: the provider isn't asked again until it ends, so someone
/// it has since turned away isn't kept in for a month.
const OAUTH_SESSION_HOURS: u32 = 24;

/// Which accounts may be used under the current settings, and how long their sessions last.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Gate {
    session_hours: Option<u32>,
    method: Method,
}

impl Gate {
    /// Hoard's own accounts always; directory and OAuth accounts only while that's how people sign in.
    pub(crate) fn lets_in(self, source: &str) -> bool {
        match Source::parse(source) {
            Some(Source::Internal) => true,
            Some(Source::Ldap) => matches!(self.method, Method::Ldap | Method::Ad),
            Some(Source::Oauth) => self.method == Method::Oauth,
            None => false,
        }
    }

    pub(crate) fn session_hours(self, source: &str) -> Option<u32> {
        if Source::parse(source) == Some(Source::Oauth) {
            Some(self.session_hours.unwrap_or(OAUTH_SESSION_HOURS))
        } else {
            self.session_hours
        }
    }
}

fn http_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

impl AuthConfig {
    pub(crate) fn gate(&self) -> Gate {
        Gate { session_hours: self.session_hours, method: self.method }
    }

    /// The directory in use, if signing in goes through one.
    pub(crate) fn directory(&self) -> Option<&Ldap> {
        match self.method {
            Method::Ldap => Some(&self.ldap),
            Method::Ad => Some(&self.ad),
            Method::Internal | Method::Oauth => None,
        }
    }

    fn default_role(&self) -> Role {
        Role::parse(&self.default_role).unwrap_or(Role::View)
    }

    /// Problems a person can fix, in their words.
    pub(crate) fn check(&self) -> Result<(), String> {
        if Role::parse(&self.default_role).is_none() {
            return Err(tr!("error-auth-default-role"));
        }
        if self.session_hours.is_some_and(|h| !(1..=8760).contains(&h)) {
            return Err(tr!("error-auth-session-hours"));
        }
        if let Some(l) = self.directory() {
            if l.servers.iter().all(|s| s.trim().is_empty()) {
                return Err(tr!("error-ldap-no-server"));
            }
            for s in &l.servers {
                server_url(s, l.tls)?;
            }
            if l.base_dn.trim().is_empty() {
                return Err(tr!("error-ldap-base-dn"));
            }
            if l.username_attribute.trim().is_empty() {
                return Err(tr!("error-ldap-username-attribute"));
            }
            if !l.anonymous && l.bind_dn.trim().is_empty() {
                return Err(tr!("error-ldap-bind-dn"));
            }
            if !l.ca_cert.trim().is_empty() && roots_from_pem(&l.ca_cert).is_err() {
                return Err(tr!("error-ldap-ca-cert"));
            }
        }
        if self.method == Method::Oauth {
            let o = &self.oauth;
            if o.client_id.trim().is_empty() {
                return Err(tr!("error-oauth-client-id"));
            }
            if !http_url(o.authorization_url.trim()) || !http_url(o.token_url.trim()) {
                return Err(tr!("error-oauth-addresses"));
            }
            if !o.userinfo_url.trim().is_empty() && !http_url(o.userinfo_url.trim()) {
                return Err(tr!("error-oauth-userinfo-address"));
            }
            if o.username_claim.trim().is_empty() {
                return Err(tr!("error-oauth-username-claim"));
            }
        }
        Ok(())
    }

    /// For the Settings page: secrets left out, with whether they're set.
    fn shown(&self) -> Value {
        let mut v = serde_json::to_value(self).unwrap_or_default();
        for (part, field) in [("ldap", "bind_password"), ("ad", "bind_password"), ("oauth", "client_secret")] {
            if let Some(o) = v.get_mut(part).and_then(Value::as_object_mut) {
                let set = o.get(field).and_then(Value::as_str).is_some_and(|s| !s.is_empty());
                o.insert(field.into(), Value::String(String::new()));
                o.insert(format!("{field}_set"), Value::Bool(set));
            }
        }
        v
    }
}

/// What the groups someone is in say about their role.
#[derive(Debug, PartialEq, Eq)]
enum Decision {
    /// No groups are listed under Roles: new accounts get the default role, and an admin sets it after.
    Default,
    Role(Role),
    /// Viewer groups are listed and they're in none of the listed groups.
    Refused,
}

fn decide(roles: &Roles, groups: &[String]) -> Decision {
    if roles.admin.is_empty() && roles.edit.is_empty() && roles.view.is_empty() {
        return Decision::Default;
    }
    let member = |list: &[String]| list.iter().any(|g| groups.iter().any(|m| m.trim().eq_ignore_ascii_case(g.trim())));
    if member(&roles.admin) {
        Decision::Role(Role::Admin)
    } else if member(&roles.edit) {
        Decision::Role(Role::Edit)
    } else if member(&roles.view) || roles.view.is_empty() {
        Decision::Role(Role::View)
    } else {
        Decision::Refused
    }
}

// ---------------------------------------------------------------------------------------------
// Accounts for people from outside

/// Where an account's sign-in is checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Source {
    Internal,
    Ldap,
    Oauth,
}

impl Source {
    pub(crate) fn parse(s: &str) -> Option<Self> {
        match s {
            "internal" => Some(Self::Internal),
            "ldap" => Some(Self::Ldap),
            "oauth" => Some(Self::Oauth),
            _ => None,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Ldap => "ldap",
            Self::Oauth => "oauth",
        }
    }
}

/// An account's id, name, role, where it signs in and who it is at the OAuth provider.
type UserRow = (i64, String, String, String, Option<String>);

/// The account `name` signs in to from `source`, made if need be and its role brought up to date. `subject` is who
/// the OAuth provider says they are (its issuer and `sub`), which a name can't stand in for: the account follows it
/// when they're renamed there, and someone else given their old name doesn't get their account. `Err(msg)`: why they
/// can't, in their words.
pub(crate) fn account_for(
    c: &Connection,
    cfg: &AuthConfig,
    source: Source,
    name: &str,
    subject: Option<&str>,
    groups: &[String],
) -> Result<Result<(i64, String, Role), String>> {
    let decision = decide(&cfg.roles, groups);
    if decision == Decision::Refused {
        return Ok(Err(tr!("error-signin-no-group", name = name)));
    }
    let pick = |column: &str, key: &str| -> rusqlite::Result<Option<UserRow>> {
        c.query_row(&format!("SELECT id, username, role, source, subject FROM users WHERE {column} = ?1"), [key], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
        })
        .optional()
    };
    let found = match subject {
        Some(s) => match pick("subject", s)? {
            Some(row) => Some(row),
            None => pick("username", name)?,
        },
        None => pick("username", name)?,
    };
    if let Some((id, username, role, from, stored)) = found {
        if Source::parse(&from) != Some(source) {
            return Ok(Err(tr!("error-signin-other-source", name = username.as_str())));
        }
        if let Some(s) = subject {
            match stored.as_deref() {
                Some(t) if t != s => {
                    return Ok(Err(tr!("error-signin-other-subject", name = username.as_str())));
                }
                Some(_) => {}
                None => {
                    c.execute("UPDATE users SET subject = ?2 WHERE id = ?1", params![id, s])?;
                }
            }
        }
        let role = match decision {
            Decision::Role(r) => {
                c.execute("UPDATE users SET role = ?2 WHERE id = ?1", params![id, r.as_str()])?;
                r
            }
            _ => Role::stored(&role),
        };
        return Ok(Ok((id, username, role)));
    }
    if !cfg.provision {
        return Ok(Err(tr!("error-signin-no-account", name = name)));
    }
    if name.is_empty() || name.chars().count() > 64 || name.chars().any(char::is_control) {
        return Ok(Err(tr!("error-signin-bad-name")));
    }
    let role = match decision {
        Decision::Role(r) => r,
        _ => cfg.default_role(),
    };
    c.execute(
        "INSERT INTO users (username, password_hash, role, source, subject, created_at) VALUES (?1, '', ?2, ?3, ?4, ?5)",
        params![name, role.as_str(), source.as_str(), subject, db::now()],
    )?;
    tracing::info!(user = %name, source = source.as_str(), role = role.as_str(), "account made at first sign-in");
    Ok(Ok((c.last_insert_rowid(), name.to_owned(), role)))
}

// ---------------------------------------------------------------------------------------------
// LDAP and Active Directory

/// `host`, `host:port` or a URL, as a URL.
fn server_url(s: &str, tls: Tls) -> Result<String, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err(tr!("error-ldap-empty-server"));
    }
    let url = if s.contains("://") {
        s.to_owned()
    } else {
        format!("{}://{s}", if tls == Tls::Ldaps { "ldaps" } else { "ldap" })
    };
    if !(url.starts_with("ldap://") || url.starts_with("ldaps://")) {
        return Err(tr!("error-ldap-server-form", server = s));
    }
    if url.starts_with("ldaps://") && tls == Tls::StartTls {
        return Err(tr!("error-ldap-not-starttls", server = s));
    }
    Ok(url)
}

fn roots_from_pem(pem: &str) -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in CertificateDer::pem_slice_iter(pem.as_bytes()) {
        roots.add(cert?)?;
    }
    if roots.is_empty() {
        bail!(tr!("error-ldap-no-certificate"));
    }
    Ok(roots)
}

/// The system's certificate authorities, read once.
static SYSTEM_ROOTS: LazyLock<Arc<rustls::RootCertStore>> = LazyLock::new(|| {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
        let _ = roots.add(cert);
    }
    Arc::new(roots)
});

fn tls_config(l: &Ldap) -> Result<Arc<rustls::ClientConfig>> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let builder =
        rustls::ClientConfig::builder_with_provider(Arc::clone(&provider)).with_safe_default_protocol_versions()?;
    let config = if l.skip_verify {
        builder
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(crate::https::SignatureOnly(provider)))
            .with_no_client_auth()
    } else if l.ca_cert.trim().is_empty() {
        builder.with_root_certificates(Arc::clone(&SYSTEM_ROOTS)).with_no_client_auth()
    } else {
        builder.with_root_certificates(roots_from_pem(&l.ca_cert)?).with_no_client_auth()
    };
    Ok(Arc::new(config))
}

/// A connection to the first server that answers. Says what went wrong with each when none does.
async fn connect(l: &Ldap) -> Result<(ldap3::Ldap, String)> {
    let mut why = Vec::new();
    for s in l.servers.iter().filter(|s| !s.trim().is_empty()) {
        match connect_to(l, s).await {
            Ok(ldap) => return Ok((ldap, s.trim().to_owned())),
            Err(e) => why.push(format!("{}: {e:#}", s.trim())),
        }
    }
    bail!(tr!("error-ldap-unreachable", reasons = why.join("; ")))
}

async fn connect_to(l: &Ldap, server: &str) -> Result<ldap3::Ldap> {
    let url = server_url(server, l.tls).map_err(|e| anyhow!(e))?;
    let settings = LdapConnSettings::new()
        .set_conn_timeout(LDAP_CONNECT)
        .set_starttls(l.tls == Tls::StartTls)
        .set_no_tls_verify(l.skip_verify)
        .set_config(tls_config(l)?);
    let (conn, ldap) = LdapConnAsync::with_settings(settings, &url).await?;
    ldap3::drive!(conn);
    Ok(ldap)
}

/// Sign in as the service account, or anonymously.
async fn bind_service(ldap: &mut ldap3::Ldap, l: &Ldap) -> Result<()> {
    let (dn, pw) = if l.anonymous { ("", "") } else { (l.bind_dn.trim(), l.bind_password.0.as_str()) };
    ldap.with_timeout(LDAP_OP).simple_bind(dn, pw).await?.success().map_err(|e| {
        if l.anonymous {
            anyhow!(tr!("error-ldap-no-anonymous", reason = e.to_string()))
        } else {
            anyhow!(tr!("error-ldap-bind-failed", reason = e.to_string()))
        }
    })?;
    Ok(())
}

/// Someone the directory vouched for.
#[derive(Debug)]
pub(crate) struct DirectoryUser {
    /// As the directory spells it.
    pub(crate) name: String,
    pub(crate) dn: String,
    /// Their groups' DNs and names.
    pub(crate) groups: Vec<String>,
}

fn attr<'a>(e: &'a SearchEntry, name: &str) -> Option<&'a Vec<String>> {
    e.attrs.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v)
}

/// `cn=Artists,ou=groups,…` → `Artists`.
fn first_rdn_value(dn: &str) -> Option<&str> {
    let first = dn.split(',').next()?;
    first.split_once('=').map(|(_, v)| v.trim())
}

/// Check `name` and `password` against the directory. `Ok(None)`: no such person, or the wrong password.
pub(crate) async fn directory_sign_in(l: &Ldap, name: &str, password: &str) -> Result<Option<DirectoryUser>> {
    // An empty password would be an "unauthenticated bind", which many servers let through.
    if name.trim().is_empty() || password.is_empty() {
        return Ok(None);
    }
    let (mut ldap, _) = connect(l).await?;
    let found = find_and_bind(&mut ldap, l, name.trim(), password).await;
    let _ = ldap.unbind().await;
    found
}

/// `name`'s entry, looked up as the service account (signed in already). `active_only`: in Active Directory, not if
/// the account is turned off (signing in checks that itself).
async fn find_entry(ldap: &mut ldap3::Ldap, l: &Ldap, name: &str, active_only: bool) -> Result<Option<SearchEntry>> {
    let user_attr = l.username_attribute.trim();
    let enabled =
        if active_only && l.server_type == "ad" { "(!(userAccountControl:1.2.840.113556.1.4.803:=2))" } else { "" };
    let filter = format!("(&({user_attr}={}){}{enabled})", ldap_escape(name), l.user_filter.trim());
    let (entries, _) = ldap
        .with_timeout(LDAP_OP)
        .search(l.base_dn.trim(), Scope::Subtree, &filter, vec![user_attr, "memberOf"])
        .await?
        .success()
        .with_context(|| tr!("error-ldap-search", name = name, base = l.base_dn.trim()))?;
    let mut entries: Vec<SearchEntry> = entries.into_iter().map(SearchEntry::construct).collect();
    if entries.len() > 1 {
        bail!(tr!("error-ldap-several", count = entries.len(), name = name));
    }
    Ok(entries.pop())
}

async fn find_and_bind(ldap: &mut ldap3::Ldap, l: &Ldap, name: &str, password: &str) -> Result<Option<DirectoryUser>> {
    bind_service(ldap, l).await?;
    let Some(entry) = find_entry(ldap, l, name, false).await? else { return Ok(None) };
    let bound = ldap.with_timeout(LDAP_OP).simple_bind(&entry.dn, password).await?;
    match bound.rc {
        0 => {}
        // invalidCredentials
        49 => return Ok(None),
        _ => bail!(tr!("error-ldap-password-check", name = name, reason = bound.to_string())),
    }
    let groups = groups_of(ldap, l, &entry, name).await?;
    let user_attr = l.username_attribute.trim();
    let spelled = attr(&entry, user_attr).and_then(|v| v.iter().find(|v| v.eq_ignore_ascii_case(name))).cloned();
    Ok(Some(DirectoryUser { name: spelled.unwrap_or_else(|| name.to_owned()), dn: entry.dn, groups }))
}

/// The groups `entry` (found for `name`) is in, by DN and name.
async fn groups_of(ldap: &mut ldap3::Ldap, l: &Ldap, entry: &SearchEntry, name: &str) -> Result<Vec<String>> {
    let mut groups = Vec::new();
    for dn in attr(entry, "memberOf").into_iter().flatten() {
        groups.push(dn.clone());
        groups.extend(first_rdn_value(dn).map(str::to_owned));
    }
    if !l.group_base_dn.trim().is_empty() && !l.group_filter.trim().is_empty() {
        // Group lists usually need the service account (or anonymous) to read, not the person.
        bind_service(ldap, l).await?;
        let filter =
            l.group_filter.trim().replace("{dn}", &ldap_escape(&entry.dn)).replace("{user}", &ldap_escape(name));
        let (found, _) = ldap
            .with_timeout(LDAP_OP)
            .search(l.group_base_dn.trim(), Scope::Subtree, &filter, vec!["cn"])
            .await?
            .success()
            .with_context(|| tr!("error-ldap-group-search", name = name, base = l.group_base_dn.trim()))?;
        for g in found.into_iter().map(SearchEntry::construct) {
            groups.extend(attr(&g, "cn").into_iter().flatten().cloned());
            groups.push(g.dn);
        }
    }
    groups.sort_unstable_by_key(|g| g.to_lowercase());
    groups.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    Ok(groups)
}

/// How often the directory is asked again about the people signed in through it.
const RECHECK: Duration = Duration::from_mins(15);

/// Each name's groups, or `None` for someone the directory no longer has (or, in Active Directory, has turned off).
pub(crate) async fn directory_lookup(l: &Ldap, names: &[String]) -> Result<Vec<Option<Vec<String>>>> {
    let (mut ldap, _) = connect(l).await?;
    let found = async {
        let mut out = Vec::with_capacity(names.len());
        for name in names {
            bind_service(&mut ldap, l).await?;
            out.push(match find_entry(&mut ldap, l, name, true).await? {
                Some(entry) => Some(groups_of(&mut ldap, l, &entry, name).await?),
                None => None,
            });
        }
        Ok(out)
    }
    .await;
    let _ = ldap.unbind().await;
    found
}

/// What the directory's answer about someone means now: their role (`None`: leave it, no groups are listed), or why
/// they're signed out.
fn recheck_verdict(roles: &Roles, groups: Option<&[String]>) -> Result<Option<Role>, &'static str> {
    let Some(groups) = groups else { return Err("no longer in the directory") };
    match decide(roles, groups) {
        Decision::Refused => Err("no longer in a group that may use Hoard"),
        Decision::Role(r) => Ok(Some(r)),
        Decision::Default => Ok(None),
    }
}

/// Ask the directory about everyone signed in through it (with a session, token or connected app): someone it no
/// longer has, or no longer lets use Hoard, is signed out everywhere, and everyone else's role follows their groups.
/// A password checked at sign-in would otherwise keep working for a month after they left.
pub(crate) async fn recheck_directory(state: &AppState) -> Result<()> {
    let cfg = state.settings.get().auth.clone();
    let Some(l) = cfg.directory().cloned() else { return Ok(()) };
    let people: Vec<(i64, String)> = db::call(&state.pool, |c| {
        let mut stmt = c.prepare(
            "SELECT id, username FROM users u WHERE source = 'ldap' AND (
               EXISTS (SELECT 1 FROM sessions s WHERE s.user_id = u.id)
               OR EXISTS (SELECT 1 FROM access_tokens t WHERE t.created_by = u.id)
               OR EXISTS (SELECT 1 FROM oauth_grants g WHERE g.user_id = u.id))",
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    })
    .await?;
    if people.is_empty() {
        return Ok(());
    }
    let names: Vec<String> = people.iter().map(|(_, n)| n.clone()).collect();
    let found = directory_lookup(&l, &names).await?;
    let roles = cfg.roles;
    db::call(&state.pool, move |c| {
        for ((id, name), groups) in people.iter().zip(&found) {
            match recheck_verdict(&roles, groups.as_deref()) {
                Ok(Some(role)) => {
                    c.execute("UPDATE users SET role = ?2 WHERE id = ?1 AND role <> ?2", params![id, role.as_str()])?;
                }
                Ok(None) => {}
                Err(why) => {
                    crate::auth::sign_out_everywhere(c, *id)?;
                    tracing::info!(user = %name, "signed out everywhere: {why}");
                }
            }
        }
        Ok(())
    })
    .await
}

/// [`recheck_directory`] every [`RECHECK`].
pub(crate) async fn recheck_loop(state: AppState) {
    let mut every = tokio::time::interval(RECHECK);
    every.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    every.tick().await;
    loop {
        every.tick().await;
        if let Err(e) = recheck_directory(&state).await {
            tracing::warn!("couldn't check directory accounts against the directory: {e:#}");
        }
    }
}

// ---------------------------------------------------------------------------------------------
// OAuth

struct Pending {
    verifier: String,
    redirect: String,
    since: Instant,
}

/// Sign-ins started and not yet back from the provider, by `state`.
static PENDING: LazyLock<Mutex<HashMap<String, Pending>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn random() -> Result<String> {
    Ok(b64(&crate::auth::random_bytes::<32>()?))
}

/// Where the provider sends people back to. Give the provider this address.
fn callback_url(state: &AppState, headers: &HeaderMap) -> String {
    format!("{}{CALLBACK}", crate::mcp::base_url(state, headers))
}

fn state_cookie(headers: &HeaderMap, value: &str, max_age: u64) -> Result<HeaderValue> {
    let secure = if crate::auth::over_https(headers) { "; Secure" } else { "" };
    Ok(HeaderValue::from_str(&format!(
        "{OAUTH_COOKIE}={value}; Path=/api/auth/oauth; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}"
    ))?)
}

/// Back to the portal, showing why signing in didn't work.
fn back_with_error(headers: &HeaderMap, msg: &str) -> Response {
    tracing::info!(reason = msg, "OAuth sign-in failed");
    let mut url = reqwest::Url::parse("http://hoard/").expect("valid");
    url.query_pairs_mut().append_pair("signin_error", msg);
    let location = format!("/?{}", url.query().unwrap_or_default());
    let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, location)]).into_response();
    if let Ok(clear) = state_cookie(headers, "", 0) {
        res.headers_mut().append(header::SET_COOKIE, clear);
    }
    res
}

/// Off to the provider's sign-in page.
async fn oauth_start(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let cfg = state.settings.get().auth.clone();
    if cfg.method != Method::Oauth {
        return back_with_error(&headers, &tr!("error-oauth-off"));
    }
    match start_url(&state, &headers, &cfg.oauth) {
        Ok((location, st)) => {
            let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, location)]).into_response();
            if let Ok(c) = state_cookie(&headers, &st, OAUTH_WAIT.as_secs()) {
                res.headers_mut().append(header::SET_COOKIE, c);
            }
            res
        }
        Err(e) => back_with_error(&headers, &format!("{e:#}")),
    }
}

fn start_url(state: &AppState, headers: &HeaderMap, o: &OAuth) -> Result<(String, String)> {
    let st = random()?;
    let verifier = random()?;
    let challenge = b64(&sha2::Sha256::digest(verifier.as_bytes()));
    let redirect = callback_url(state, headers);
    let mut url = reqwest::Url::parse(o.authorization_url.trim())
        .with_context(|| tr!("error-oauth-bad-authorization-address"))?;
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", o.client_id.trim())
        .append_pair("redirect_uri", &redirect)
        .append_pair("scope", o.scopes.trim())
        .append_pair("state", &st)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256");
    let mut pending = PENDING.lock().unwrap_or_else(PoisonError::into_inner);
    pending.retain(|_, p| p.since.elapsed() < OAUTH_WAIT);
    pending.insert(st.clone(), Pending { verifier, redirect, since: Instant::now() });
    Ok((url.into(), st))
}

#[derive(Deserialize)]
struct Callback {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

/// Back from the provider: trade the code for tokens, find out who it is, and sign them in.
async fn oauth_callback(State(state): State<AppState>, headers: HeaderMap, Query(q): Query<Callback>) -> Response {
    if let Some(err) = q.error {
        let detail = q.error_description.unwrap_or(err);
        return back_with_error(&headers, &tr!("error-oauth-refused", reason = detail));
    }
    let (Some(code), Some(st)) = (q.code, q.state) else {
        return back_with_error(&headers, &tr!("error-oauth-no-code"));
    };
    let same_browser = crate::auth::cookie_value(&headers, OAUTH_COOKIE).is_some_and(|c| c == st);
    let pending = PENDING.lock().unwrap_or_else(PoisonError::into_inner).remove(&st);
    let Some(pending) = pending.filter(|p| same_browser && p.since.elapsed() < OAUTH_WAIT) else {
        return back_with_error(&headers, &tr!("error-oauth-expired"));
    };
    let cfg = state.settings.get().auth.clone();
    if cfg.method != Method::Oauth {
        return back_with_error(&headers, &tr!("error-oauth-off"));
    }
    let claims = match who_signed_in(&state.http, &cfg.oauth, &code, &pending).await {
        Ok(c) => c,
        Err(e) => return back_with_error(&headers, &format!("{e:#}")),
    };
    let Some(name) = claim_string(&claims, cfg.oauth.username_claim.trim()) else {
        return back_with_error(&headers, &tr!("error-oauth-no-claim", claim = cfg.oauth.username_claim.trim()));
    };
    let Some(sub) = claim_string(&claims, "sub").or_else(|| claim_string(&claims, "id")) else {
        return back_with_error(&headers, &tr!("error-oauth-no-sub"));
    };
    let issuer = claim_string(&claims, "iss").unwrap_or_else(|| cfg.oauth.token_url.trim().to_owned());
    let subject = format!("{issuer} {sub}");
    let groups = claim_strings(&claims, cfg.oauth.groups_claim.trim());
    let made = db::call(&state.pool, move |c| {
        Ok(match account_for(c, &cfg, Source::Oauth, &name, Some(&subject), &groups)? {
            Ok((id, username, _)) => Ok((crate::auth::new_session(c, id)?, username)),
            Err(msg) => Err(msg),
        })
    })
    .await;
    match made {
        Ok(Ok((token, username))) => {
            tracing::info!(user = %username, "signed in with OAuth");
            let mut res = (StatusCode::SEE_OTHER, [(header::LOCATION, "/")]).into_response();
            if let Ok(c) = crate::auth::session_cookie(&headers, &token) {
                res.headers_mut().append(header::SET_COOKIE, c);
            }
            if let Ok(c) = state_cookie(&headers, "", 0) {
                res.headers_mut().append(header::SET_COOKIE, c);
            }
            res
        }
        Ok(Err(msg)) => back_with_error(&headers, &msg),
        Err(e) => back_with_error(&headers, &format!("{e:#}")),
    }
}

/// The claims about who signed in: the user-info answer over the ID token's. (The ID token came straight from the
/// token endpoint over TLS, which OpenID Connect accepts in place of checking its signature.)
async fn who_signed_in(http: &reqwest::Client, o: &OAuth, code: &str, pending: &Pending) -> Result<Value> {
    let mut form = reqwest::Url::parse("http://hoard/").expect("valid");
    form.query_pairs_mut()
        .append_pair("grant_type", "authorization_code")
        .append_pair("code", code)
        .append_pair("redirect_uri", &pending.redirect)
        .append_pair("client_id", o.client_id.trim())
        .append_pair("client_secret", &o.client_secret.0)
        .append_pair("code_verifier", &pending.verifier);
    let res = http
        .post(o.token_url.trim())
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .timeout(Duration::from_secs(30))
        .body(form.query().unwrap_or_default().to_owned())
        .send()
        .await
        .with_context(|| tr!("error-oauth-token-unreachable"))?;
    let status = res.status();
    let body: Value = res.json().await.unwrap_or(Value::Null);
    if !status.is_success() || body.get("error").is_some() {
        let why = body
            .get("error_description")
            .or_else(|| body.get("error"))
            .and_then(Value::as_str)
            .map_or_else(|| status.to_string(), str::to_owned);
        bail!(tr!("error-oauth-token-refused", reason = why));
    }
    let mut claims = body.get("id_token").and_then(Value::as_str).and_then(jwt_claims).unwrap_or_else(|| json!({}));
    if !o.userinfo_url.trim().is_empty() {
        let token =
            body.get("access_token").and_then(Value::as_str).with_context(|| tr!("error-oauth-no-access-token"))?;
        let res = http
            .get(o.userinfo_url.trim())
            .bearer_auth(token)
            .header(header::ACCEPT, "application/json")
            .header(header::USER_AGENT, hoard_core::user_agent())
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .with_context(|| tr!("error-oauth-userinfo-unreachable"))?;
        if !res.status().is_success() {
            bail!(tr!("error-oauth-userinfo-refused", status = res.status().to_string()));
        }
        let info: Value = res.json().await.with_context(|| tr!("error-oauth-userinfo-not-json"))?;
        if let (Some(all), Some(more)) = (claims.as_object_mut(), info.as_object()) {
            all.extend(more.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
    }
    Ok(claims)
}

/// A JWT's claims, unchecked.
fn jwt_claims(jwt: &str) -> Option<Value> {
    let payload = jwt.split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload.trim_end_matches('=')).ok()?;
    serde_json::from_slice(&bytes).ok().filter(Value::is_object)
}

fn claim<'a>(claims: &'a Value, path: &str) -> Option<&'a Value> {
    if path.is_empty() {
        return None;
    }
    claims.get(path).or_else(|| path.split('.').try_fold(claims, |v, k| v.get(k)))
}

fn claim_string(claims: &Value, path: &str) -> Option<String> {
    match claim(claims, path)? {
        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_owned()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn claim_strings(claims: &Value, path: &str) -> Vec<String> {
    match claim(claims, path) {
        Some(Value::Array(a)) => a.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect(),
        Some(Value::String(s)) => s.split([',', ' ']).filter(|s| !s.is_empty()).map(str::to_owned).collect(),
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------------------------
// Routes

/// Open to everyone: the trip to the OAuth provider and back.
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/api/auth/oauth/start", get(oauth_start)).route(CALLBACK, get(oauth_callback))
}

/// Behind `require_admin`.
pub(crate) fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/auth", get(get_config).put(put_config))
        .route("/api/admin/auth/ldap/test", post(test_ldap))
        .route("/api/admin/auth/oauth/discover", post(discover))
}

/// What the sign-in page offers.
pub(crate) fn sign_in_options(cfg: &AuthConfig) -> Value {
    let button = cfg.oauth.button.trim();
    json!({
        "directory": match cfg.method {
            Method::Ldap => Some("LDAP"),
            Method::Ad => Some("Active Directory"),
            Method::Internal | Method::Oauth => None,
        },
        "oauth": (cfg.method == Method::Oauth).then(|| if button.is_empty() { "your account".to_owned() } else { button.to_owned() }),
    })
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

#[derive(Deserialize)]
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
        let tried = async {
            let mut ldap = connect_to(&l, s).await?;
            let bound = bind_service(&mut ldap, &l).await;
            let _ = ldap.unbind().await;
            bound
        }
        .await;
        servers.push(match tried {
            Ok(()) => json!({ "server": s.trim(), "ok": true }),
            Err(e) => json!({ "server": s.trim(), "ok": false, "error": format!("{e:#}") }),
        });
    }
    let user = match (body.username.as_deref().map(str::trim), body.password.as_deref()) {
        (Some(name), Some(pw)) if !name.is_empty() => {
            let roles = state.settings.get().auth.roles.clone();
            Some(match directory_sign_in(&l, name, pw).await {
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

#[derive(Deserialize)]
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

    #[test]
    fn roles_from_groups() {
        let none = Roles::default();
        assert_eq!(decide(&none, &["anything".into()]), Decision::Default);
        let roles =
            Roles { admin: vec!["Admins".into()], edit: vec!["cn=artists,ou=groups,dc=x".into()], view: vec![] };
        assert_eq!(decide(&roles, &["admins".into()]), Decision::Role(Role::Admin));
        assert_eq!(decide(&roles, &["CN=Artists,ou=groups,dc=x".into()]), Decision::Role(Role::Edit));
        assert_eq!(decide(&roles, &[]), Decision::Role(Role::View));
        let only = Roles { view: vec!["hoard".into()], ..Roles::default() };
        assert_eq!(decide(&only, &["other".into()]), Decision::Refused);
        assert_eq!(decide(&only, &["Hoard".into()]), Decision::Role(Role::View));
    }

    #[test]
    fn server_addresses() {
        assert_eq!(server_url("dir.example.com", Tls::StartTls).unwrap(), "ldap://dir.example.com");
        assert_eq!(server_url("dir:636", Tls::Ldaps).unwrap(), "ldaps://dir:636");
        assert_eq!(server_url("ldap://dir:389", Tls::None).unwrap(), "ldap://dir:389");
        assert!(server_url("ldaps://dir", Tls::StartTls).is_err());
        assert!(server_url("http://dir", Tls::None).is_err());
    }

    #[test]
    fn claims() {
        let c = json!({ "preferred_username": "ada", "id": 42, "realm_access": { "roles": ["admins", "x"] }, "groups": "a, b" });
        assert_eq!(claim_string(&c, "preferred_username").as_deref(), Some("ada"));
        assert_eq!(claim_string(&c, "id").as_deref(), Some("42"));
        assert_eq!(claim_strings(&c, "realm_access.roles"), ["admins", "x"]);
        assert_eq!(claim_strings(&c, "groups"), ["a", "b"]);
        let jwt = format!("x.{}.y", b64(br#"{"sub":"1","email":"a@b.c"}"#));
        assert_eq!(jwt_claims(&jwt).unwrap()["email"], "a@b.c");
    }

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

    #[test]
    fn signing_in_follows_the_settings() {
        let oauth = AuthConfig { method: Method::Oauth, ..AuthConfig::default() }.gate();
        assert!(oauth.lets_in("internal") && oauth.lets_in("oauth") && !oauth.lets_in("ldap"));
        assert_eq!(oauth.session_hours("oauth"), Some(OAUTH_SESSION_HOURS));
        assert_eq!(oauth.session_hours("internal"), None);
        let ad = AuthConfig { method: Method::Ad, session_hours: Some(8), ..AuthConfig::default() }.gate();
        assert!(ad.lets_in("ldap") && !ad.lets_in("oauth"));
        assert_eq!(ad.session_hours("oauth"), Some(8));
        let only = Roles { view: vec!["hoard".into()], ..Roles::default() };
        assert_eq!(recheck_verdict(&only, None), Err("no longer in the directory"));
        assert!(recheck_verdict(&only, Some(&["other".into()])).is_err());
        assert_eq!(recheck_verdict(&only, Some(&["Hoard".into()])), Ok(Some(Role::View)));
        assert_eq!(recheck_verdict(&Roles::default(), Some(&[])), Ok(None));
    }

    #[test]
    fn checks() {
        let mut cfg = AuthConfig { method: Method::Ldap, ..AuthConfig::default() };
        assert!(cfg.check().unwrap_err().contains("server"));
        cfg.ldap.servers = vec!["dir:389".into()];
        assert!(cfg.check().unwrap_err().contains("base DN"));
        cfg.ldap.base_dn = "dc=example,dc=com".into();
        assert!(cfg.check().unwrap_err().contains("looks people up"));
        cfg.ldap.anonymous = true;
        assert_eq!(cfg.check(), Ok(()));
        cfg.ldap.ca_cert = "not a cert".into();
        assert!(cfg.check().unwrap_err().contains("PEM"));
        let cfg = AuthConfig { method: Method::Oauth, ..AuthConfig::default() };
        assert!(cfg.check().unwrap_err().contains("client ID"));
    }

    /// A directory in a test: `ou=people,dc=hoard,dc=test` with ada (in Artists) and bob, who look people up as
    /// `cn=svc` with `svcpass`.
    mod fake_directory {
        use futures_util::{SinkExt as _, StreamExt as _};
        use ldap3_proto::LdapCodec;
        use ldap3_proto::proto::{LdapFilter, LdapPartialAttribute, LdapResultCode, LdapSearchResultEntry};
        use ldap3_proto::simple::{ServerOps, SimpleBindRequest};
        use tokio_util::codec::{FramedRead, FramedWrite};

        pub(super) const PEOPLE: &str = "ou=people,dc=hoard,dc=test";
        const USERS: &[(&str, &str, &[&str])] =
            &[("ada", "lovelace1", &["cn=Artists,ou=groups,dc=hoard,dc=test"]), ("bob", "builder22", &[])];

        fn dn(user: &str) -> String {
            format!("uid={user},{PEOPLE}")
        }

        fn bind(b: &SimpleBindRequest) -> bool {
            (b.dn == "cn=svc" && b.pw == "svcpass")
                || (b.dn.is_empty() && b.pw.is_empty())
                || USERS.iter().any(|(u, pw, _)| b.dn == dn(u) && b.pw == *pw)
        }

        fn wanted(f: &LdapFilter) -> Option<String> {
            match f {
                LdapFilter::And(all) => all.iter().find_map(wanted),
                LdapFilter::Equality(a, v) if a.eq_ignore_ascii_case("uid") => Some(v.clone()),
                _ => None,
            }
        }

        /// Serve one connection; `Some(port)` once listening.
        pub(super) async fn start() -> u16 {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let port = listener.local_addr().unwrap().port();
            tokio::spawn(async move {
                while let Ok((socket, _)) = listener.accept().await {
                    tokio::spawn(async move {
                        let (r, w) = tokio::io::split(socket);
                        let mut reqs = FramedRead::new(r, LdapCodec::default());
                        let mut resp = FramedWrite::new(w, LdapCodec::default());
                        let mut service = false;
                        while let Some(Ok(msg)) = reqs.next().await {
                            let out = match ServerOps::try_from(msg) {
                                Ok(ServerOps::SimpleBind(b)) => {
                                    let ok = bind(&b);
                                    service = ok && (b.dn == "cn=svc" || b.dn.is_empty());
                                    vec![if ok { b.gen_success() } else { b.gen_invalid_cred() }]
                                }
                                Ok(ServerOps::Search(s)) if !service => {
                                    vec![s.gen_error(LdapResultCode::InsufficentAccessRights, "no".into())]
                                }
                                Ok(ServerOps::Search(s)) => {
                                    let mut out = Vec::new();
                                    if let Some(name) = wanted(&s.filter)
                                        && let Some((u, _, groups)) =
                                            USERS.iter().find(|(u, ..)| u.eq_ignore_ascii_case(&name))
                                        && s.base == PEOPLE
                                    {
                                        out.push(s.gen_result_entry(LdapSearchResultEntry {
                                            dn: dn(u),
                                            attributes: vec![
                                                LdapPartialAttribute {
                                                    atype: "uid".into(),
                                                    vals: vec![u.as_bytes().to_vec()],
                                                },
                                                LdapPartialAttribute {
                                                    atype: "memberOf".into(),
                                                    vals: groups.iter().map(|g| g.as_bytes().to_vec()).collect(),
                                                },
                                            ],
                                        }));
                                    }
                                    out.push(s.gen_success());
                                    out
                                }
                                Ok(ServerOps::Unbind(_)) | Err(()) => return,
                                Ok(_) => continue,
                            };
                            for m in out {
                                if resp.send(m).await.is_err() {
                                    return;
                                }
                            }
                        }
                    });
                }
            });
            port
        }
    }

    fn plain_ldap(port: u16) -> Ldap {
        Ldap {
            servers: vec![format!("127.0.0.1:{port}")],
            tls: Tls::None,
            bind_dn: "cn=svc".into(),
            bind_password: Secret("svcpass".into()),
            base_dn: fake_directory::PEOPLE.into(),
            ..Ldap::default()
        }
    }

    #[tokio::test]
    async fn signs_in_against_a_directory() {
        let port = fake_directory::start().await;
        let l = plain_ldap(port);
        let ada = directory_sign_in(&l, "ADA", "lovelace1").await.unwrap().expect("ada signs in");
        assert_eq!(ada.name, "ada");
        assert_eq!(ada.dn, format!("uid=ada,{}", fake_directory::PEOPLE));
        assert!(ada.groups.iter().any(|g| g == "Artists"), "{:?}", ada.groups);
        assert!(directory_sign_in(&l, "ada", "wrong").await.unwrap().is_none());
        assert!(directory_sign_in(&l, "ada", "").await.unwrap().is_none());
        assert!(directory_sign_in(&l, "nobody", "lovelace1").await.unwrap().is_none());

        // A server that isn't there is passed over for the next.
        let dead = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let fallback = Ldap { servers: vec![format!("127.0.0.1:{dead}"), format!("127.0.0.1:{port}")], ..l.clone() };
        assert!(directory_sign_in(&fallback, "bob", "builder22").await.unwrap().is_some());

        // A wrong service password is a problem with the settings, not with the person.
        let bad = Ldap { bind_password: Secret("nope".into()), ..l.clone() };
        let err = directory_sign_in(&bad, "bob", "builder22").await.unwrap_err();
        assert!(format!("{err:#}").contains("looks people up"), "{err:#}");
        let only_dead = Ldap { servers: vec![format!("127.0.0.1:{dead}")], ..l.clone() };
        assert!(format!("{:#}", directory_sign_in(&only_dead, "bob", "x").await.unwrap_err()).contains("reached"));

        // Asked again later without a password: who's still there, and in which groups.
        let found = directory_lookup(&l, &["ada".into(), "carol".into()]).await.unwrap();
        assert!(found[0].as_ref().is_some_and(|g| g.iter().any(|g| g == "Artists")), "{found:?}");
        assert_eq!(found[1], None);
    }

    #[test]
    fn accounts_for_outsiders() {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE COLLATE NOCASE,
               password_hash TEXT NOT NULL, created_at TEXT NOT NULL, role TEXT NOT NULL DEFAULT 'admin',
               source TEXT NOT NULL DEFAULT 'internal', subject TEXT UNIQUE);
             INSERT INTO users (username, password_hash, created_at) VALUES ('root', 'x', '');",
        )
        .unwrap();
        let mut cfg = AuthConfig::default();
        let (_, name, role) = account_for(&c, &cfg, Source::Ldap, "ada", None, &[]).unwrap().unwrap();
        assert_eq!((name.as_str(), role), ("ada", Role::View));
        // Same person again: the same account, and the role an admin gave it stays when no groups are listed.
        c.execute("UPDATE users SET role = 'edit' WHERE username = 'ada'", []).unwrap();
        assert_eq!(account_for(&c, &cfg, Source::Ldap, "ADA", None, &[]).unwrap().unwrap().2, Role::Edit);
        // Groups listed: they decide, each time.
        cfg.roles.admin = vec!["Admins".into()];
        assert_eq!(
            account_for(&c, &cfg, Source::Ldap, "ada", None, &["admins".into()]).unwrap().unwrap().2,
            Role::Admin
        );
        assert_eq!(account_for(&c, &cfg, Source::Ldap, "ada", None, &[]).unwrap().unwrap().2, Role::View);
        // Not someone else's account, and not a new one when provisioning is off.
        assert!(account_for(&c, &cfg, Source::Oauth, "root", None, &[]).unwrap().unwrap_err().contains("another way"));
        assert!(account_for(&c, &cfg, Source::Oauth, "ada", None, &[]).unwrap().is_err());
        // An OAuth account follows the person at the provider, not their name there.
        let (carol, ..) = account_for(&c, &cfg, Source::Oauth, "carol", Some("https://id 1"), &[]).unwrap().unwrap();
        let (again, name, _) =
            account_for(&c, &cfg, Source::Oauth, "caroline", Some("https://id 1"), &[]).unwrap().unwrap();
        assert_eq!((again, name.as_str()), (carol, "carol"));
        let other = account_for(&c, &cfg, Source::Oauth, "carol", Some("https://id 2"), &[]).unwrap().unwrap_err();
        assert!(other.contains("someone else"), "{other}");
        cfg.provision = false;
        assert!(account_for(&c, &cfg, Source::Oauth, "eve", None, &[]).unwrap().unwrap_err().contains("admin can add"));
    }
}
