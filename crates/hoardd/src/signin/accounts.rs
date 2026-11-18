//! Accounts for people from outside: the Hoard account a directory or OAuth sign-in gets the first time, and its role
//! from their groups at each sign-in after.

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension as _, params};

use super::{AuthConfig, Decision, decide};
use crate::auth::Role;
use crate::db;
use crate::i18n::tr;

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

#[cfg(test)]
mod tests {
    use super::*;

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
