//! LDAP and Active Directory: a server's address as a URL, a certificate of its own, and checking now and then that
//! people signed in through the directory are still in it and in the same groups.

use std::time::Duration;

use anyhow::{Result, bail};
use rusqlite::params;
use rustls_pki_types::CertificateDer;
use rustls_pki_types::pem::PemObject as _;

use super::{Decision, Roles, Tls, decide};
use crate::AppState;
use crate::auth::Role;
use crate::db;
use crate::directory;
use crate::i18n::tr;

/// `host`, `host:port` or a URL, as a URL.
pub(crate) fn server_url(s: &str, tls: Tls) -> Result<String, String> {
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

pub(crate) fn roots_from_pem(pem: &str) -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in CertificateDer::pem_slice_iter(pem.as_bytes()) {
        roots.add(cert?)?;
    }
    if roots.is_empty() {
        bail!(tr!("error-ldap-no-certificate"));
    }
    Ok(roots)
}

/// How often the directory is asked again about the people signed in through it.
const RECHECK: Duration = Duration::from_mins(15);

/// What the directory's answer about someone means now: their role (`None`: leave it, no groups are listed), or why
/// they're signed out.
pub(super) fn recheck_verdict(roles: &Roles, groups: Option<&[String]>) -> Result<Option<Role>, &'static str> {
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
    let found = directory::directory_lookup(&l, &names).await?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_addresses() {
        assert_eq!(server_url("dir.example.com", Tls::StartTls).unwrap(), "ldap://dir.example.com");
        assert_eq!(server_url("dir:636", Tls::Ldaps).unwrap(), "ldaps://dir:636");
        assert_eq!(server_url("ldap://dir:389", Tls::None).unwrap(), "ldap://dir:389");
        assert!(server_url("ldaps://dir", Tls::StartTls).is_err());
        assert!(server_url("http://dir", Tls::None).is_err());
    }
}
