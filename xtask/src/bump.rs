//! `cargo xtask bump`: Hoard's version lives in one place, `[workspace.package] version` in Cargo.toml. Once a commit
//! that changes it is pushed to main and passes CI, `.github/workflows/auto-release.yml` tags that commit and releases
//! it: binaries, desktop installers, the container image and the GitHub release.
//!
//! ```text
//! cargo xtask bump patch        # 1.18.19 -> 1.18.20 (1.19.0-rc.1 -> 1.19.0)
//! cargo xtask bump minor        # 1.18.19 -> 1.19.0
//! cargo xtask bump major        # 1.18.19 -> 2.0.0
//! cargo xtask bump 2.0.0-rc.1   # exactly this
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};
use toml_edit::DocumentMut;

pub(crate) fn run(root: &Path, to: &str) -> Result<()> {
    let path = root.join("Cargo.toml");
    let text = std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let mut doc: DocumentMut = text.parse().context("parse Cargo.toml")?;
    let item = &mut doc["workspace"]["package"]["version"];
    let current = item.as_str().context("Cargo.toml has no [workspace.package] version")?;
    let current = Version::parse(current)?;
    let next = match to {
        "patch" => current.next_patch(),
        "minor" => Version { major: current.major, minor: current.minor + 1, patch: 0, pre: None },
        "major" => Version { major: current.major + 1, minor: 0, patch: 0, pre: None },
        v => Version::parse(v.strip_prefix('v').unwrap_or(v))?,
    };
    if next <= current {
        bail!("{next} isn't newer than Cargo.toml's {current}");
    }
    let tag = format!("v{next}");
    let tagged = Command::new("git").current_dir(root).args(["tag", "--list", &tag]).output().context("run git tag")?;
    if !String::from_utf8_lossy(&tagged.stdout).trim().is_empty() {
        bail!("{tag} is already a release");
    }

    // Keep the value's spacing; only the version changes.
    let decor = item.as_value().map(|v| v.decor().clone());
    *item = toml_edit::value(next.to_string());
    if let (Some(decor), Some(v)) = (decor, item.as_value_mut()) {
        *v.decor_mut() = decor;
    }
    std::fs::write(&path, doc.to_string()).with_context(|| format!("write {}", path.display()))?;

    // CI builds with --locked, so Cargo.lock's entries for Hoard's own crates have to move with it.
    crate::run(crate::cargo().current_dir(root).args(["update", "--workspace"]), "cargo update --workspace")?;

    println!(
        "\nHoard is {next} in Cargo.toml and Cargo.lock (was {current}). Commit both, e.g. \"Release {next}\", and push to\n\
         main: once CI passes, that commit is tagged {tag} and released."
    );
    Ok(())
}

/// A semver version: `major.minor.patch`, optionally `-pre.release`. Build metadata isn't used.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    pre: Option<String>,
}

impl Version {
    fn parse(s: &str) -> Result<Self> {
        let bad = || anyhow::anyhow!("'{s}' isn't a version like 1.2.3 or 1.2.3-rc.1 (or patch, minor or major)");
        let (core, pre) = match s.split_once('-') {
            Some((core, pre)) => (core, Some(pre)),
            None => (s, None),
        };
        if let Some(pre) = pre
            && (pre.is_empty()
                || pre.split('.').any(|id| id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')))
        {
            return Err(bad());
        }
        let mut parts = core.split('.').map(|p| {
            if p.is_empty() || !p.bytes().all(|b| b.is_ascii_digit()) || (p.len() > 1 && p.starts_with('0')) {
                return Err(bad());
            }
            p.parse::<u64>().map_err(|_| bad())
        });
        let (Some(major), Some(minor), Some(patch), None) = (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(bad());
        };
        Ok(Self { major: major?, minor: minor?, patch: patch?, pre: pre.map(str::to_owned) })
    }

    /// The next patch release, or this one's release when it's a pre-release.
    fn next_patch(&self) -> Self {
        match self.pre {
            Some(_) => Self { pre: None, ..self.clone() },
            None => Self { patch: self.patch + 1, ..self.clone() },
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{pre}")?;
        }
        Ok(())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Semver precedence: a pre-release comes before its release, and pre-release identifiers compare numerically when
/// both are numbers, numbers before words, and a shorter list first when one starts the other.
impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch)).then_with(|| {
            match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => {
                    let key = |id: &str| match id.parse::<u64>() {
                        Ok(n) => (0, n, String::new()),
                        Err(_) => (1, 0, id.to_owned()),
                    };
                    a.split('.').map(key).cmp(b.split('.').map(key))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn parses_and_prints() {
        assert_eq!(v("1.18.19").to_string(), "1.18.19");
        assert_eq!(v("2.0.0-rc.1").to_string(), "2.0.0-rc.1");
        for bad in ["1.2", "1.2.3.4", "01.2.3", "1.2.x", "1.2.3-", "1.2.3-rc..1", "", "v1.2.3"] {
            assert!(Version::parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn orders_like_semver() {
        let order = [
            "1.0.0-alpha",
            "1.0.0-alpha.1",
            "1.0.0-alpha.beta",
            "1.0.0-beta.2",
            "1.0.0-beta.11",
            "1.0.0-rc.1",
            "1.0.0",
            "1.0.1",
            "1.10.0",
        ];
        for pair in order.windows(2) {
            assert!(v(pair[0]) < v(pair[1]), "{} < {}", pair[0], pair[1]);
        }
    }

    #[test]
    fn next_patch_finishes_a_pre_release() {
        assert_eq!(v("1.18.19").next_patch(), v("1.18.20"));
        assert_eq!(v("1.19.0-rc.2").next_patch(), v("1.19.0"));
    }
}
