//! The addresses Hoard's own certificate covers: those it finds (this computer's name and LAN addresses, the public
//! address in Settings) and those the admin adds, cleaned up.

use std::net::IpAddr;
use std::path::Path;

use anyhow::Result;
use axum::http::{HeaderMap, header};

use super::Saved;
use crate::i18n::tr;

/// The addresses Hoard's own certificate covers: the ones found, and the admin's.
pub(super) fn wanted_names(saved: &Saved, public_url: Option<&str>) -> Vec<String> {
    let mut names = found_names(public_url);
    names.extend(saved.names.iter().cloned());
    sort_names(&mut names);
    names
}

/// This computer's name and LAN addresses, `localhost`, and the public address in Settings. In a container the
/// computer's name is the container's (no use to anyone), so it's left out: the address you type into the browser
/// is added in Settings instead.
pub(super) fn found_names(public_url: Option<&str>) -> Vec<String> {
    let mut out = vec!["localhost".to_owned(), "127.0.0.1".to_owned(), "::1".to_owned()];
    if !Path::new("/.dockerenv").exists()
        && let Some(host) = gethostname::gethostname().to_str().and_then(|h| clean_name(h).ok())
    {
        if !host.contains('.') {
            out.push(format!("{host}.local"));
        }
        out.push(host);
    }
    for iface in if_addrs::get_if_addrs().unwrap_or_default() {
        if let IpAddr::V4(v4) = iface.ip()
            && !v4.is_loopback()
            && !v4.is_link_local()
            && !v4.is_unspecified()
        {
            out.push(v4.to_string());
        }
    }
    if let Some(host) = public_url.and_then(|u| clean_name(u).ok()) {
        out.push(host);
    }
    sort_names(&mut out);
    out
}

/// Names first, then addresses; no repeats.
pub(super) fn sort_names(names: &mut Vec<String>) {
    names.sort_by(|a, b| (a.parse::<IpAddr>().is_ok(), a).cmp(&(b.parse::<IpAddr>().is_ok(), b)));
    names.dedup();
}

/// An address as typed (`https://NAS.local:4627/`, `[::1]`, `192.168.1.5`) as a certificate holds it.
pub(super) fn clean_name(raw: &str) -> Result<String, String> {
    let bad = || tr!("error-https-bad-name", name = raw.trim());
    let mut s = raw.trim();
    if let Some((_, rest)) = s.split_once("://") {
        s = rest;
    }
    s = s.split(['/', '?', '#']).next().unwrap_or("");
    if let Some(rest) = s.strip_prefix('[') {
        s = rest.split(']').next().unwrap_or("");
    } else if s.matches(':').count() == 1 {
        s = s.split(':').next().unwrap_or("");
    }
    if let Ok(ip) = s.parse::<IpAddr>() {
        return Ok(ip.to_string());
    }
    let s = s.trim_end_matches('.').to_ascii_lowercase();
    let label_ok = |l: &str| {
        (1..=63).contains(&l.len())
            && l.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            && !l.starts_with('-')
            && !l.ends_with('-')
    };
    let mut labels = s.split('.');
    let first = labels.next().unwrap_or("");
    let rest: Vec<&str> = labels.collect();
    let ok =
        s.len() <= 253 && (label_ok(first) || (first == "*" && rest.len() >= 2)) && rest.iter().all(|l| label_ok(l));
    if ok { Ok(s) } else { Err(bad()) }
}

/// The host (without the port) a request was made to.
pub(super) fn request_host(headers: &HeaderMap) -> Option<String> {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok())?;
    clean_name(host).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_are_cleaned_up() {
        assert_eq!(clean_name("https://NAS.local:4627/x").unwrap(), "nas.local");
        assert_eq!(clean_name("192.168.0.22:4627").unwrap(), "192.168.0.22");
        assert_eq!(clean_name("[::1]:4627").unwrap(), "::1");
        assert_eq!(clean_name("fe80::1").unwrap(), "fe80::1");
        assert_eq!(clean_name("*.example.com").unwrap(), "*.example.com");
        assert_eq!(clean_name("hoard.tail1234.ts.net.").unwrap(), "hoard.tail1234.ts.net");
        assert!(clean_name("not a name").is_err());
        assert!(clean_name("*.com").is_err());
        assert!(clean_name("-bad.example").is_err());
    }
}
