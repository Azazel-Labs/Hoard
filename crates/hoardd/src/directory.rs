//! Signing in against LDAP or Active Directory (the `ldap` feature): finding someone's entry, checking their password
//! by binding as them, and reading their groups. What's asked for and when is in `signin.rs`.

use std::sync::{Arc, LazyLock};
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow, bail};
use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry, ldap_escape};

use crate::i18n::tr;
use crate::signin::{Ldap, Tls, roots_from_pem, server_url};

/// How long to wait for a directory server to answer at all, and for each request after that.
const LDAP_CONNECT: Duration = Duration::from_secs(8);
const LDAP_OP: Duration = Duration::from_secs(15);

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
            .with_custom_certificate_verifier(Arc::new(crate::signature_only::SignatureOnly(provider)))
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
/// Reach `server`, start TLS, and sign in as the account that looks people up.
pub(crate) async fn reach(l: &Ldap, server: &str) -> Result<()> {
    let mut ldap = connect_to(l, server).await?;
    let bound = bind_service(&mut ldap, l).await;
    let _ = ldap.unbind().await;
    bound
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signin::Secret;

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
}
