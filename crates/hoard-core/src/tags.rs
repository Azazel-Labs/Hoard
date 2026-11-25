//! Tags: where one came from, and the one spelling each is kept in.

use serde::{Deserialize, Serialize};

/// Where a tag came from. Human tags always win and are never overwritten by machines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TagSource {
    /// Derived from file metadata (dimensions, duration, format). Facts.
    Meta,
    /// Offline heuristic tagger (filename vocab, image analysis, audio heuristics).
    Rule,
    /// An AI model (the local model server, or any OpenAI-compatible endpoint).
    /// Stored as `llm` before v1.0.1; that still reads as this.
    #[serde(alias = "llm")]
    Ai,
    /// A person typed it.
    Human,
}

impl TagSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagSource::Meta => "meta",
            TagSource::Rule => "rule",
            TagSource::Ai => "ai",
            TagSource::Human => "human",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "meta" => TagSource::Meta,
            "rule" => TagSource::Rule,
            "ai" | "llm" => TagSource::Ai,
            _ => TagSource::Human,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Tag {
    pub tag: String,
    pub source: TagSource,
    pub confidence: f32,
}

/// Normalize a tag: lowercase, trimmed, spaces/underscores -> '-', optional `namespace:value`. Any script works: letters
/// and digits are kept with the marks written on them (Hindi's vowel signs, Thai's tone marks) and the joiners some scripts
/// spell with, in composed form so an accent typed on its own matches one typed with its letter. At most 64 characters.
pub fn normalize_tag(raw: &str) -> Option<String> {
    use icu_normalizer::ComposingNormalizerBorrowed;
    use icu_properties::{
        CodePointMapData,
        props::{GeneralCategory, GeneralCategoryGroup},
    };
    let raw = ComposingNormalizerBorrowed::new_nfc().normalize(&raw.trim().to_lowercase()).into_owned();
    if raw.is_empty() {
        return None;
    }
    let category = CodePointMapData::<GeneralCategory>::new();
    let norm_part = |s: &str| -> String {
        let mut out = String::with_capacity(s.len());
        for c in s.chars() {
            let after_text = out.chars().next_back().is_some_and(|p| p != '-');
            if c.is_alphanumeric() || c == '/' || c == '.' || c == '+' {
                out.push(c);
            } else if c == '\u{200c}' || c == '\u{200d}' || GeneralCategoryGroup::Mark.contains(category.get(c)) {
                // A mark or joiner belongs to the letter before it, and means nothing without one.
                if after_text {
                    out.push(c);
                }
            } else if after_text {
                out.push('-');
            }
        }
        out.trim_end_matches(['-', '\u{200c}', '\u{200d}']).to_owned()
    };
    let tag = match raw.split_once(':') {
        Some((ns, v)) => {
            let (ns, v) = (norm_part(ns), norm_part(v));
            if ns.is_empty() || v.is_empty() {
                return None;
            }
            format!("{ns}:{v}")
        }
        None => norm_part(&raw),
    };
    if tag.is_empty() || tag.chars().count() > 64 { None } else { Some(tag) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tags_normalize() {
        assert_eq!(normalize_tag("  Pixel Art "), Some("pixel-art".into()));
        assert_eq!(normalize_tag("Genre: Lo Fi"), Some("genre:lo-fi".into()));
        assert_eq!(normalize_tag("sfx/explosion"), Some("sfx/explosion".into()));
        assert_eq!(normalize_tag(":"), None);
        // Native text in both halves, whatever the script.
        assert_eq!(normalize_tag("楽器:ドラム"), Some("楽器:ドラム".into()));
        assert_eq!(normalize_tag("Инструмент: Барабан"), Some("инструмент:барабан".into()));
        assert_eq!(normalize_tag("संगीत:क्षेत्र"), Some("संगीत:क्षेत्र".into()));
        assert_eq!(normalize_tag("เสียง:ไม้กลอง"), Some("เสียง:ไม้กลอง".into()));
        assert_eq!(normalize_tag("מוזיקה:כְּלִי"), Some("מוזיקה:כְּלִי".into()));
        assert_eq!(normalize_tag("می\u{200c}خواهم"), Some("می\u{200c}خواهم".into()));
        // An accent typed apart from its letter is the same tag as one typed with it.
        assert_eq!(normalize_tag("Cafe\u{301}"), Some("café".into()));
        assert_eq!(normalize_tag("Café"), Some("café".into()));
        // The limit is characters, not bytes.
        assert!(normalize_tag(&"楽器".repeat(32)).is_some());
        assert!(normalize_tag(&"楽器".repeat(33)).is_none());
    }
}
