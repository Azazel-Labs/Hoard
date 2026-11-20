//! The last few hundred lines the server logged, kept in memory for Settings → About's "Report a problem". Each line
//! is scrubbed as it's kept: access tokens, passwords, keys, signed-in cookies, email addresses and the user's name in
//! home folder paths never reach the buffer, so the report can be pasted into a public issue as it is.

use std::collections::VecDeque;
use std::fmt::Write as _;
use std::sync::{LazyLock, Mutex};

use regex::Regex;
use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

/// How many lines are kept, and how long one may be.
const KEEP: usize = 300;
const LINE_MAX: usize = 1000;

static LINES: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// The layer that keeps them; add it to the subscriber next to the one that prints.
pub fn layer() -> RecentLog {
    RecentLog
}

#[derive(Debug)]
pub struct RecentLog;

impl<S: Subscriber> Layer<S> for RecentLog {
    fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
        let meta = event.metadata();
        let mut fields = Fields::default();
        event.record(&mut fields);
        let now = time::OffsetDateTime::now_utc();
        let (h, m, s) = now.time().as_hms();
        let said = scrub(&format!("{}{}", fields.message, fields.rest));
        let mut line = format!("{h:02}:{m:02}:{s:02} {:>5} {}: {said}", meta.level(), meta.target());
        if line.len() > LINE_MAX {
            let mut end = LINE_MAX;
            while !line.is_char_boundary(end) {
                end -= 1;
            }
            line.truncate(end);
            line.push('…');
        }
        let Ok(mut lines) = LINES.lock() else { return };
        if lines.len() == KEEP {
            lines.pop_front();
        }
        lines.push_back(line);
    }
}

#[derive(Default)]
struct Fields {
    message: String,
    rest: String,
}

impl Visit for Fields {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message.push_str(value);
        } else {
            let _ = write!(self.rest, " {}={value:?}", field.name());
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            let _ = write!(self.message, "{value:?}");
        } else {
            let _ = write!(self.rest, " {}={value:?}", field.name());
        }
    }
}

/// The lines kept, oldest first.
pub(crate) fn lines() -> Vec<String> {
    LINES.lock().map_or_else(|_| Vec::new(), |l| l.iter().cloned().collect())
}

/// What could identify someone or let them in, replaced by `…`.
pub(crate) fn scrub(line: &str) -> String {
    static RULES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
        [
            // Hoard's own access tokens, and anything sent as a bearer or basic credential.
            (r"\bhoard_[0-9A-Za-z]{8,}", "hoard_…"),
            (r"(?i)\b(bearer|basic)\s+[^\s,;]+", "$1 …"),
            // `password=…`, `?token=…`, `"secret_key": "…"`, `Authorization: …` and the like.
            (
                r#"(?i)\b([a-z0-9_-]*(?:password|passwd|secret|token|api[_-]?key|access[_-]?key|authorization|cookie|session|credential|signature)[a-z0-9_-]*)(\s*=\s*"?)((?:bearer\s+|basic\s+)?)[^\s"&,;}]+"#,
                "$1$2$3…",
            ),
            (
                r#"(?i)"([a-z0-9_-]*(?:password|passwd|secret|token|api[_-]?key|access[_-]?key|authorization|cookie|session|credential|signature)[a-z0-9_-]*)("\s*:\s*"?)((?:bearer\s+|basic\s+)?)[^\s"&,;}]+"#,
                "\"$1$2$3…",
            ),
            (
                r"(?i)\b(authorization|proxy-authorization|cookie|set-cookie|x-api-key|password)(:\s*)((?:bearer\s+|basic\s+)?)[^\s,;]+",
                "$1$2$3…",
            ),
            // Keys that look like keys whatever they're called.
            (r"\b(AKIA|ASIA)[0-9A-Z]{16}\b", "…"),
            (r"\bsk-[0-9A-Za-z_-]{16,}", "sk-…"),
            (r"\bgh[pousr]_[0-9A-Za-z]{20,}", "gh…"),
            (r"\beyJ[0-9A-Za-z_-]{10,}\.[0-9A-Za-z_-]{10,}\.[0-9A-Za-z_-]*", "…"),
            // A user and password in an address.
            (r"(?i)\b([a-z][a-z0-9+.-]*://)[^/\s@]+@", "$1…@"),
            (r"[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)+", "…@…"),
            // Whose computer it is.
            (r"(?i)\b([a-z]:[\\/]+users[\\/]+)[^\\/\s]+", "$1…"),
            (r"(/home/|/Users/)[^/\s]+", "$1…"),
        ]
        .into_iter()
        .map(|(re, to)| (Regex::new(re).expect("the scrubbing rules are valid"), to))
        .collect()
    });
    let mut out = std::borrow::Cow::Borrowed(line);
    for (re, to) in RULES.iter() {
        if let std::borrow::Cow::Owned(s) = re.replace_all(&out, *to) {
            out = std::borrow::Cow::Owned(s);
        }
    }
    out.into_owned()
}

#[cfg(test)]
mod tests {
    use super::scrub;

    #[test]
    fn scrubs_what_lets_someone_in_or_says_who_they_are() {
        for (line, kept, gone) in [
            ("token hoard_0123456789abcdef0123456789abcdef used", "hoard_…", "0123456789abcdef"),
            ("Authorization: Bearer abc.def.ghi", "Bearer …", "abc.def"),
            ("connecting with password=hunter2&user=me", "password=…", "hunter2"),
            (r#"{"s3_secret_key": "wJalrXUtnFEMI", "region": "auto"}"#, r#""s3_secret_key": "…"#, "wJalrXUtn"),
            ("key AKIAIOSFODNN7EXAMPLE rejected", "key …", "AKIAIOSFODNN7EXAMPLE"),
            ("fetching https://me:pa55@nas.local/x", "https://…@nas.local", "pa55"),
            ("signed in as someone@example.com", "…@…", "someone@"),
            (r"reading C:\Users\Someone\Assets\a.png", r"C:\Users\…\Assets", "Someone"),
            ("reading /home/someone/assets", "/home/…/assets", "someone"),
            ("reading /Users/someone/assets", "/Users/…/assets", "someone"),
            ("cookie hoard_session=abcdef123", "hoard_session=…", "abcdef123"),
            ("Cookie: hoard_session=abcdef123", "Cookie: …", "abcdef123"),
            ("update hook sent with x-api-key: k3y", "x-api-key: …", "k3y"),
        ] {
            let s = scrub(line);
            assert!(s.contains(kept), "{line:?} → {s:?}, wanted {kept:?}");
            assert!(!s.contains(gone), "{line:?} → {s:?} still has {gone:?}");
        }
    }

    #[test]
    fn leaves_ordinary_lines_alone() {
        for line in [
            "scanned 1204 files in Sprites (3.2 s)",
            "blob 3f2a9c0d1e4b5a6978c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2 is ok",
            "listening on http://0.0.0.0:4627",
            "converted s3://bucket/packs/x.fbx to glb",
            "first account made user=e2e",
            "credits for pack author=\"Kenney\" license=\"CC0\"",
        ] {
            assert_eq!(scrub(line), line);
        }
    }
}
