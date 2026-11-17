//! Built without the `ldap` feature (the desktop app): Settings → Authentication refuses LDAP and Active Directory
//! (see `AuthConfig::check`), so these are only reached by settings saved by a build that had it.

use anyhow::{Result, bail};

use crate::i18n::tr;
use crate::signin::Ldap;

/// Someone the directory vouched for.
#[derive(Debug)]
pub(crate) struct DirectoryUser {
    pub(crate) name: String,
    pub(crate) dn: String,
    pub(crate) groups: Vec<String>,
}

#[expect(clippy::unused_async, reason = "the same signature as with the `ldap` feature")]
pub(crate) async fn directory_sign_in(_l: &Ldap, _name: &str, _password: &str) -> Result<Option<DirectoryUser>> {
    bail!(tr!("error-not-built", feature = "LDAP"))
}

#[expect(clippy::unused_async, reason = "the same signature as with the `ldap` feature")]
pub(crate) async fn directory_lookup(_l: &Ldap, _names: &[String]) -> Result<Vec<Option<Vec<String>>>> {
    bail!(tr!("error-not-built", feature = "LDAP"))
}

#[expect(clippy::unused_async, reason = "the same signature as with the `ldap` feature")]
pub(crate) async fn reach(_l: &Ldap, _server: &str) -> Result<()> {
    bail!(tr!("error-not-built", feature = "LDAP"))
}
