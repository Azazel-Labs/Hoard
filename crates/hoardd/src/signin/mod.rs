//! Settings → Authentication: how people sign in to the portal besides Hoard's own accounts.
//!
//! - **Internal**: names and passwords kept by Hoard (the `auth` module). These always work, whatever is chosen here, so an
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

mod accounts;
mod admin;
mod ldap;
mod oauth;

use std::fmt;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::auth::Role;
use crate::i18n::tr;

pub(crate) use accounts::{Source, account_for};
pub(crate) use admin::admin_router;
pub(crate) use ldap::{recheck_loop, roots_from_pem, server_url};
pub(crate) use oauth::router;

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
            if cfg!(not(feature = "ldap")) {
                return Err(tr!("error-not-built", feature = "LDAP"));
            }
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

/// What the sign-in page offers. An OAuth button with no text of its own is "" and the portal words it in the reader's
/// language.
pub(crate) fn sign_in_options(cfg: &AuthConfig) -> Value {
    let button = cfg.oauth.button.trim();
    json!({
        "directory": match cfg.method {
            Method::Ldap => Some("LDAP"),
            Method::Ad => Some("Active Directory"),
            Method::Internal | Method::Oauth => None,
        },
        "oauth": (cfg.method == Method::Oauth).then_some(button),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signin::ldap::recheck_verdict;

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
        let cfg = AuthConfig { method: Method::Oauth, ..AuthConfig::default() };
        assert!(cfg.check().unwrap_err().contains("client ID"));
        let mut cfg = AuthConfig { method: Method::Ldap, ..AuthConfig::default() };
        if cfg!(not(feature = "ldap")) {
            assert!(cfg.check().unwrap_err().contains("LDAP"));
            return;
        }
        assert!(cfg.check().unwrap_err().contains("server"));
        cfg.ldap.servers = vec!["dir:389".into()];
        assert!(cfg.check().unwrap_err().contains("base DN"));
        cfg.ldap.base_dn = "dc=example,dc=com".into();
        assert!(cfg.check().unwrap_err().contains("looks people up"));
        cfg.ldap.anonymous = true;
        assert_eq!(cfg.check(), Ok(()));
        cfg.ldap.ca_cert = "not a cert".into();
        assert!(cfg.check().unwrap_err().contains("PEM"));
    }
}
