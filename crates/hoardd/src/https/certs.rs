//! Certificates: Hoard's own authority and the server certificate it issues (renewed, and re-issued when the
//! addresses change), an uploaded chain and key checked before it's used, and what a certificate says.

use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, anyhow, bail};
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use rustls::ServerConfig;
use rustls_pki_types::pem::PemObject;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use sha2::Digest as _;
use time::OffsetDateTime;

use super::names::{sort_names, wanted_names};
use super::{Built, CertInfo, ISSUE_DAYS, Mode, RENEW_DAYS, Saved};
use crate::i18n::tr;

/// The certificate `saved` asks for: Hoard's own (made or renewed as needed), the uploaded one (`upload`, or the
/// one kept from before), or none.
pub(super) fn build(
    dir: &Path,
    saved: &Saved,
    public_url: Option<&str>,
    upload: Option<&Upload>,
) -> Result<Option<Built>> {
    match saved.mode {
        Mode::Off => Ok(None),
        Mode::Own => {
            let names = wanted_names(saved, public_url);
            if own_needs_issue(dir, &names) {
                let (_, issuer) = load_or_make_ca(dir)?;
                issue(dir, &names, &issuer)?;
            }
            let built = read_pair(&dir.join("server.crt"), &dir.join("server.key"))?;
            Ok(Some(built))
        }
        Mode::Custom => {
            let built = if let Some(u) = upload {
                u.parse()?
            } else {
                let (c, k) = (dir.join("custom.crt"), dir.join("custom.key"));
                if !c.is_file() || !k.is_file() {
                    bail!(tr!("error-https-no-certificate"));
                }
                read_pair(&c, &k)?
            };
            check_dates(&built.info)?;
            Ok(Some(built))
        }
    }
}

pub(super) fn read_pair(cert: &Path, key: &Path) -> Result<Built> {
    let upload = Upload {
        cert: std::fs::read_to_string(cert).with_context(|| format!("read {}", cert.display()))?,
        key: std::fs::read_to_string(key).with_context(|| format!("read {}", key.display()))?,
    };
    upload.parse()
}

/// A certificate chain and key as pasted or uploaded (in either box, or both in one).
#[derive(Debug)]
pub(super) struct Upload {
    pub(super) cert: String,
    pub(super) key: String,
}

impl Upload {
    /// The PEM blocks: certificates (in order) and the private key.
    fn split(&self) -> Result<(String, String)> {
        let blocks = pem_blocks(&format!("{}\n{}", self.cert, self.key));
        let certs: Vec<&str> =
            blocks.iter().filter(|(label, _)| label == "CERTIFICATE").map(|(_, b)| b.as_str()).collect();
        let keys: Vec<&str> =
            blocks.iter().filter(|(label, _)| label.ends_with("PRIVATE KEY")).map(|(_, b)| b.as_str()).collect();
        if certs.is_empty() {
            bail!(tr!("error-https-no-cert-found"));
        }
        match keys.len() {
            0 => bail!(tr!("error-https-no-key-found")),
            1 => {}
            _ => bail!(tr!("error-https-several-keys")),
        }
        if blocks.iter().any(|(label, _)| label == "ENCRYPTED PRIVATE KEY") {
            bail!(tr!("error-https-key-password"));
        }
        Ok((certs.join(""), keys.concat()))
    }

    pub(super) fn parse(&self) -> Result<Built> {
        let (certs, key) = self.split()?;
        let chain = CertificateDer::pem_slice_iter(certs.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| anyhow!(tr!("error-https-cert-not-pem")))?;
        let key = PrivateKeyDer::from_pem_slice(key.as_bytes()).map_err(|_| anyhow!(tr!("error-https-key-not-pem")))?;
        let info = cert_info(&chain[0])?;
        server_config(&chain, &key, &[b"h2"]).map_err(|e| match e.downcast_ref::<rustls::Error>() {
            Some(rustls::Error::InconsistentKeys(_)) => anyhow!(tr!("error-https-key-mismatch")),
            _ => e.context(tr!("error-https-unusable")),
        })?;
        Ok(Built { chain, key, info })
    }

    /// The certificate and key as they're kept (`custom.crt`, `custom.key`).
    pub(super) fn store(&self, dir: &Path) -> Result<()> {
        let (certs, key) = self.split()?;
        std::fs::create_dir_all(dir)?;
        write_private(&dir.join("custom.key"), &key)?;
        std::fs::write(dir.join("custom.crt"), certs)?;
        Ok(())
    }
}

/// `-----BEGIN <label>-----` … `-----END <label>-----` blocks, as (label, text with a newline at the end).
fn pem_blocks(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut open: Option<(String, String)> = None;
    for line in text.lines().map(str::trim) {
        if let Some(label) = line.strip_prefix("-----BEGIN ").and_then(|l| l.strip_suffix("-----")) {
            open = Some((label.to_owned(), format!("{line}\n")));
        } else if let Some((label, mut body)) = open.take() {
            body.push_str(line);
            body.push('\n');
            if line.strip_prefix("-----END ").and_then(|l| l.strip_suffix("-----")) == Some(label.as_str()) {
                out.push((label, body));
            } else {
                open = Some((label, body));
            }
        }
    }
    out
}

pub(super) fn check_dates(info: &CertInfo) -> Result<()> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    if info.not_after < now {
        bail!(tr!("error-https-expired", date = day(info.not_after)));
    }
    if info.not_before > now {
        bail!(tr!("error-https-not-yet-valid", date = day(info.not_before)));
    }
    Ok(())
}

fn day(unix: i64) -> String {
    OffsetDateTime::from_unix_timestamp(unix)
        .ok()
        .and_then(|t| t.format(time::macros::format_description!("[year]-[month]-[day]")).ok())
        .unwrap_or_else(|| unix.to_string())
}

pub(crate) fn server_config(
    chain: &[CertificateDer<'static>],
    key: &PrivateKeyDer<'static>,
    alpn: &[&[u8]],
) -> Result<ServerConfig> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut config = ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(chain.to_vec(), key.clone_key())?;
    config.alpn_protocols = alpn.iter().map(|p| p.to_vec()).collect();
    Ok(config)
}

fn cert_info(der: &[u8]) -> Result<CertInfo> {
    use x509_parser::extensions::GeneralName;
    let (_, x) = x509_parser::parse_x509_certificate(der).map_err(|_| anyhow!(tr!("error-https-unreadable")))?;
    let cn = |n: &x509_parser::x509::X509Name<'_>| {
        n.iter_common_name().next().and_then(|a| a.as_str().ok()).map_or_else(|| n.to_string(), str::to_owned)
    };
    let mut names = Vec::new();
    if let Ok(Some(san)) = x.subject_alternative_name() {
        for g in &san.value.general_names {
            match g {
                GeneralName::DNSName(d) => names.push((*d).to_owned()),
                GeneralName::IPAddress(b) => {
                    if let Ok(v4) = <[u8; 4]>::try_from(*b) {
                        names.push(IpAddr::from(v4).to_string());
                    } else if let Ok(v6) = <[u8; 16]>::try_from(*b) {
                        names.push(IpAddr::from(v6).to_string());
                    }
                }
                _ => {}
            }
        }
    }
    let fingerprint = sha2::Sha256::digest(der).iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(":");
    Ok(CertInfo {
        names,
        subject: cn(x.subject()),
        issuer: cn(x.issuer()),
        not_before: x.validity().not_before.timestamp(),
        not_after: x.validity().not_after.timestamp(),
        self_signed: x.subject().as_raw() == x.issuer().as_raw(),
        fingerprint,
    })
}

/// Whether a certificate for `names` covers `host` (`*.example.com` covers one label).
pub(super) fn covers(names: &[String], host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    names.iter().any(|n| {
        let n = n.to_ascii_lowercase();
        n == host
            || n.strip_prefix("*.")
                .is_some_and(|base| host.split_once('.').is_some_and(|(label, rest)| !label.is_empty() && rest == base))
    })
}

/// The CA (made on first use) as PEM, and as an issuer.
fn load_or_make_ca(dir: &Path) -> Result<(String, Issuer<'static, KeyPair>)> {
    let (c, k) = (dir.join("ca.crt"), dir.join("ca.key"));
    if !c.is_file() || !k.is_file() {
        std::fs::create_dir_all(dir)?;
        let key = KeyPair::generate()?;
        let tag = crate::tools::hex(&crate::auth::random_bytes::<3>()?);
        let mut p = CertificateParams::default();
        p.distinguished_name = DistinguishedName::new();
        p.distinguished_name.push(DnType::CommonName, format!("Hoard local authority {tag}"));
        p.distinguished_name.push(DnType::OrganizationName, "Hoard");
        p.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        p.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign, KeyUsagePurpose::DigitalSignature];
        let now = OffsetDateTime::now_utc();
        p.not_before = now - time::Duration::days(1);
        p.not_after = now + time::Duration::days(3650);
        let cert = p.self_signed(&key)?;
        write_private(&k, &key.serialize_pem())?;
        std::fs::write(&c, cert.pem())?;
        tracing::info!(cert = %c.display(), "made Hoard's certificate authority");
    }
    let pem = std::fs::read_to_string(&c).with_context(|| format!("read {}", c.display()))?;
    let key = KeyPair::from_pem(&std::fs::read_to_string(&k).with_context(|| format!("read {}", k.display()))?)
        .context("Hoard's certificate authority key")?;
    let issuer = Issuer::from_ca_cert_pem(&pem, key).context("Hoard's certificate authority")?;
    Ok((pem, issuer))
}

/// The CA certificate, if there is one.
pub(super) fn ca_info(dir: &Path) -> Option<CertInfo> {
    let pem = std::fs::read_to_string(dir.join("ca.crt")).ok()?;
    let der = CertificateDer::from_pem_slice(pem.as_bytes()).ok()?;
    cert_info(&der).ok()
}

/// Whether the server certificate has to be (re)issued: missing, not for exactly `names`, not from the CA there is
/// now, or running out.
pub(super) fn own_needs_issue(dir: &Path, names: &[String]) -> bool {
    let Some(ca) = ca_info(dir) else { return true };
    let Ok(pem) = std::fs::read_to_string(dir.join("server.crt")) else { return true };
    let Some(cert) = CertificateDer::from_pem_slice(pem.as_bytes()).ok().and_then(|d| cert_info(&d).ok()) else {
        return true;
    };
    let mut have = cert.names.clone();
    sort_names(&mut have);
    let left = cert.not_after - OffsetDateTime::now_utc().unix_timestamp();
    have != names || cert.issuer != ca.subject || left < RENEW_DAYS * 86_400 || !dir.join("server.key").is_file()
}

fn issue(dir: &Path, names: &[String], issuer: &Issuer<'_, KeyPair>) -> Result<()> {
    let key = KeyPair::generate()?;
    let mut p = CertificateParams::new(names.to_vec()).context("certificate addresses")?;
    p.distinguished_name = DistinguishedName::new();
    p.distinguished_name.push(DnType::CommonName, "Hoard");
    p.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    p.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    p.use_authority_key_identifier_extension = true;
    let now = OffsetDateTime::now_utc();
    p.not_before = now - time::Duration::days(1);
    p.not_after = now + time::Duration::days(ISSUE_DAYS);
    let cert = p.signed_by(&key, issuer)?;
    write_private(&dir.join("server.key"), &key.serialize_pem())?;
    std::fs::write(dir.join("server.crt"), cert.pem())?;
    tracing::info!(names = ?names, "issued Hoard's HTTPS certificate");
    Ok(())
}

/// A private key, readable only by this user (on Unix).
fn write_private(path: &Path, text: &str) -> Result<()> {
    use std::io::Write as _;
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut opts, 0o600);
    let mut f = opts.open(path).with_context(|| format!("write {}", path.display()))?;
    f.write_all(text.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::https::try_handshake;

    #[test]
    fn wildcards_cover_one_label() {
        let names = vec!["*.example.com".to_owned(), "192.168.0.22".to_owned()];
        assert!(covers(&names, "hoard.example.com"));
        assert!(!covers(&names, "a.b.example.com"));
        assert!(!covers(&names, "example.com"));
        assert!(covers(&names, "192.168.0.22"));
    }

    #[tokio::test]
    async fn own_certificate_is_issued_from_the_authority_and_reissued_when_addresses_change() {
        let dir = tempfile::tempdir().unwrap();
        let saved = Saved { mode: Mode::Own, names: vec!["nas.example".into()], ..Saved::default() };
        let built = build(dir.path(), &saved, None, None).unwrap().unwrap();
        assert!(built.info.names.contains(&"nas.example".to_owned()));
        assert!(built.info.names.contains(&"127.0.0.1".to_owned()));
        let ca = ca_info(dir.path()).unwrap();
        assert_eq!(built.info.issuer, ca.subject);
        assert!(!built.info.self_signed);
        try_handshake(Arc::new(server_config(&built.chain, &built.key, &[b"h2"]).unwrap())).await.unwrap();

        // Same addresses: kept. A new one: re-issued from the same authority.
        let names = wanted_names(&saved, None);
        assert!(!own_needs_issue(dir.path(), &names));
        let more = Saved { names: vec!["nas.example".into(), "hoard.example".into()], ..saved };
        let again = build(dir.path(), &more, Some("https://public.example"), None).unwrap().unwrap();
        assert!(again.info.names.contains(&"hoard.example".to_owned()));
        assert!(again.info.names.contains(&"public.example".to_owned()));
        assert_ne!(again.info.fingerprint, built.info.fingerprint);
        assert_eq!(ca_info(dir.path()).unwrap().fingerprint, ca.fingerprint, "the authority stays");
    }

    fn self_signed(names: &[&str], days: (i64, i64)) -> (String, String) {
        let key = KeyPair::generate().unwrap();
        let mut p = CertificateParams::new(names.iter().map(|n| (*n).to_owned()).collect::<Vec<_>>()).unwrap();
        let now = OffsetDateTime::now_utc();
        p.not_before = now + time::Duration::days(days.0);
        p.not_after = now + time::Duration::days(days.1);
        (p.self_signed(&key).unwrap().pem(), key.serialize_pem())
    }

    #[test]
    fn uploaded_certificates_are_checked() {
        let (cert, key) = self_signed(&["hoard.example"], (-1, 90));
        let ok = Upload { cert: cert.clone(), key: key.clone() }.parse().unwrap();
        assert_eq!(ok.info.names, ["hoard.example"]);
        assert!(check_dates(&ok.info).is_ok());

        // Both in one box works too.
        assert!(Upload { cert: format!("{cert}{key}"), key: String::new() }.parse().is_ok());

        let (_, other_key) = self_signed(&["x.example"], (-1, 90));
        let msg = |u: Upload| format!("{:#}", u.parse().err().unwrap());
        assert!(msg(Upload { cert: cert.clone(), key: other_key }).contains("doesn't belong"));
        assert!(msg(Upload { cert: cert.clone(), key: String::new() }).contains("No private key"));
        assert!(msg(Upload { cert: String::new(), key: key.clone() }).contains("No certificate"));

        let (old, old_key) = self_signed(&["hoard.example"], (-90, -1));
        let expired = Upload { cert: old, key: old_key }.parse().unwrap();
        assert!(check_dates(&expired.info).unwrap_err().to_string().contains("expired"));
        let dir = tempfile::tempdir().unwrap();
        let saved = Saved { mode: Mode::Custom, ..Saved::default() };
        let (old, old_key) = self_signed(&["hoard.example"], (-90, -1));
        let err = build(dir.path(), &saved, None, Some(&Upload { cert: old, key: old_key })).err().unwrap();
        assert!(err.to_string().contains("expired"));
        assert!(build(dir.path(), &saved, None, None).err().unwrap().to_string().contains("Paste or upload"));
    }
}
