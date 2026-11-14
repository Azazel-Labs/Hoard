//! The server's messages in the language each request asks for (its `Accept-Language`), from the Fluent catalogs in
//! `locales/`, which the build checks and embeds. A message a catalog lacks falls back to English. Clients that don't
//! say (the CLI, the engine plugins) get English.
use std::{cell::Cell, collections::HashMap, sync::LazyLock};

use axum::{extract::Request, http::header::ACCEPT_LANGUAGE, middleware::Next, response::Response};
use fluent_bundle::{FluentArgs, FluentResource, FluentValue, concurrent::FluentBundle};

// `CATALOGS`, each locale with its catalog's text, and `LANGUAGES`, their codes.
include!(concat!(env!("OUT_DIR"), "/catalogs.rs"));

static BUNDLES: LazyLock<HashMap<&'static str, FluentBundle<FluentResource>>> = LazyLock::new(|| {
    CATALOGS
        .iter()
        .map(|&(code, text)| {
            let mut bundle =
                FluentBundle::new_concurrent(vec![code.parse().expect("the build checked each locale's name")]);
            // Every catalog is left-to-right; bidi isolation marks would only show up as stray characters.
            bundle.set_use_isolating(false);
            let resource = FluentResource::try_new(text.to_owned()).expect("the build checked each catalog");
            bundle.add_resource(resource).expect("the build checked each catalog");
            (code, bundle)
        })
        .collect()
});

tokio::task_local! {
    static LANGUAGE: &'static str;
}

thread_local! {
    /// The language of the request a blocking task (a database call, say) is working for.
    static BLOCKING: Cell<Option<&'static str>> = const { Cell::new(None) };
}

/// The language the current request asked for, or English.
pub(crate) fn current() -> &'static str {
    LANGUAGE.try_with(|language| *language).ok().or_else(|| BLOCKING.with(Cell::get)).unwrap_or("en")
}

/// Runs blocking work in `language`: the task-local doesn't follow it onto another thread.
pub(crate) fn blocking<T>(language: &'static str, f: impl FnOnce() -> T) -> T {
    let before = BLOCKING.with(|cell| cell.replace(Some(language)));
    let result = f();
    BLOCKING.with(|cell| cell.set(before));
    result
}

/// A message in the current language, with its arguments. Use [`tr!`], which `cargo xtask translations check` reads to
/// make sure every message it names is in the catalogs with the arguments it's given.
pub(crate) fn text(id: &str, args: &[(&str, FluentValue<'_>)]) -> String {
    let mut fluent = FluentArgs::new();
    for (name, value) in args {
        fluent.set(*name, value.clone());
    }
    for code in [current(), "en"] {
        let Some(bundle) = BUNDLES.get(code) else { continue };
        if let Some(pattern) = bundle.get_message(id).and_then(|message| message.value()) {
            let mut errors = Vec::new();
            return bundle.format_pattern(pattern, (!args.is_empty()).then_some(&fluent), &mut errors).into_owned();
        }
    }
    id.to_owned()
}

/// A value a message can show: text, or a number (which plural and number formatting see as one).
pub(crate) trait Arg {
    fn fluent(&self) -> FluentValue<'_>;
}
impl Arg for str {
    fn fluent(&self) -> FluentValue<'_> {
        FluentValue::from(self)
    }
}
impl Arg for String {
    fn fluent(&self) -> FluentValue<'_> {
        FluentValue::from(self.as_str())
    }
}
impl<T: Arg + ?Sized> Arg for &T {
    fn fluent(&self) -> FluentValue<'_> {
        (**self).fluent()
    }
}
macro_rules! number_arg {
    ($($t:ty),*) => {$(
        impl Arg for $t {
            fn fluent(&self) -> FluentValue<'_> {
                FluentValue::from(*self)
            }
        }
    )*};
}
number_arg!(i32, i64, u32, u64, usize, f64);

/// `tr!("message-id", name = value, …)`: the message in the language the request asked for. Values are borrowed.
macro_rules! tr {
    ($id:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::i18n::text($id, &[$((stringify!($name), $crate::i18n::Arg::fluent(&$value))),*])
    };
}
pub(crate) use tr;

/// Middleware: the rest of the request runs in the language its `Accept-Language` prefers among the catalogs.
pub(crate) async fn negotiate(req: Request, next: Next) -> Response {
    let language = req
        .headers()
        .get(ACCEPT_LANGUAGE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| preferred_catalog(&accepted(value), LANGUAGES))
        .unwrap_or("en");
    LANGUAGE.scope(language, next.run(req)).await
}

/// An `Accept-Language` header's languages, most wanted first (`*` and refused ones left out).
fn accepted(header: &str) -> Vec<String> {
    let mut languages: Vec<(String, f32)> = header
        .split(',')
        .filter_map(|item| {
            let mut parts = item.split(';');
            let tag = parts.next()?.trim();
            let q = parts
                .find_map(|param| param.trim().strip_prefix("q=").map(|q| q.trim().parse::<f32>().unwrap_or(0.0)))
                .unwrap_or(1.0);
            (!tag.is_empty() && tag != "*" && q > 0.0).then(|| (tag.to_owned(), q))
        })
        .collect();
    languages.sort_by(|a, b| b.1.total_cmp(&a.1));
    languages.into_iter().map(|(tag, _)| tag).collect()
}

/// CLDR parents, checked after an exact match and before truncation. Explicit scripts (zh-Hans-CN) keep priority
/// through the normal truncation.
/// <https://github.com/unicode-org/cldr/blob/main/common/supplemental/supplementalData.xml>
const PARENTS: &[(&str, &[&str])] = &[
    // Region-only locales for Chinese.
    ("zh-Hans", &["zh-cn", "zh-sg"]),
    ("zh-Hant", &["zh-tw", "zh-hk", "zh-mo"]),
    // parentLocales: Latin American Spanish, before the generic `es` fallback.
    (
        "es-419",
        &[
            "es-ar", "es-bo", "es-br", "es-bz", "es-cl", "es-co", "es-cr", "es-cu", "es-do", "es-ec", "es-gt", "es-hn",
            "es-jp", "es-mx", "es-ni", "es-pa", "es-pe", "es-pr", "es-py", "es-sv", "es-us", "es-uy", "es-ve",
        ],
    ),
    // parentLocales: European Portuguese.
    ("pt-PT", &["pt-ao", "pt-ch", "pt-cv", "pt-fr", "pt-gq", "pt-gw", "pt-lu", "pt-mo", "pt-mz", "pt-st", "pt-tl"]),
    // The macrolanguage `no` means Bokmål in practice.
    ("nb", &["no"]),
    // Generic Spanish and Portuguese go to the catalog most people asking for them read.
    ("es-ES", &["es"]),
    ("pt-BR", &["pt"]),
];
/// Serbian locales whose likely script is Cyrillic, unless Latin is asked for.
const SERBIAN_CYRILLIC: &[&str] = &["sr", "sr-rs", "sr-ba", "sr-xk"];

/// Matches preferences in order, letting a regional locale use its generic catalog (fr-CA → fr), with CLDR's explicit
/// parents above. Never guesses between sibling regional or script variants.
fn preferred_catalog<'a>(preferences: &[String], available: &[&'a str]) -> Option<&'a str> {
    let find = |code: &str| available.iter().copied().find(|c| c.eq_ignore_ascii_case(code));
    for preference in preferences {
        let code = preference.to_ascii_lowercase().replace('_', "-");
        // CLDR likely subtags: Serbian is Cyrillic except in Montenegro or when Latin is asked for.
        let serbian_latin = code.contains("latn") || code.split('-').nth(1) == Some("me");
        let mut code = code.as_str();
        loop {
            let parent = PARENTS
                .iter()
                .find(|(_, children)| children.contains(&code))
                .map(|(parent, _)| *parent)
                .or_else(|| (!serbian_latin && SERBIAN_CYRILLIC.contains(&code)).then_some("sr-Cyrl"));
            if let Some(catalog) = find(code).or_else(|| parent.and_then(find)) {
                return Some(catalog);
            }
            let Some(end) = code.rfind('-') else { break };
            code = &code[..end];
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pick<'a>(header: &str, available: &[&'a str]) -> Option<&'a str> {
        preferred_catalog(&accepted(header), available)
    }

    #[test]
    fn accept_language_is_read_by_weight_then_order() {
        assert_eq!(accepted("fr-CA, en;q=0.5, de;q=0.8, *;q=0.1, it;q=0"), ["fr-CA", "de", "en"]);
        assert_eq!(accepted(""), Vec::<String>::new());
        let available = ["en", "fr", "de"];
        assert_eq!(pick("ja, de;q=0.9, en;q=0.8", &available), Some("de"));
        assert_eq!(pick("en;q=0.5, fr-CA", &available), Some("fr"));
        assert_eq!(pick("ja", &available), None);
    }

    #[test]
    fn regional_and_script_preferences_find_their_catalogs() {
        let available = ["en", "es-419", "es-ES", "pt-BR", "pt-PT", "zh-Hans", "zh-Hant", "sr-Cyrl", "nb"];
        for (header, expected) in [
            ("es-MX", "es-419"),
            ("es", "es-ES"),
            ("es-ES", "es-ES"),
            ("pt", "pt-BR"),
            ("pt-AO", "pt-PT"),
            ("zh-TW", "zh-Hant"),
            ("zh-CN", "zh-Hans"),
            ("zh-Hant-HK", "zh-Hant"),
            ("sr-RS", "sr-Cyrl"),
            ("no", "nb"),
            ("nb-NO", "nb"),
        ] {
            assert_eq!(pick(header, &available), Some(expected), "{header}");
        }
        assert_eq!(pick("sr-Latn-RS", &available), None);
        assert_eq!(pick("zh", &available), None);
    }

    #[test]
    fn every_catalog_is_embedded_and_english_is_the_fallback() {
        assert!(LANGUAGES.contains(&"en"));
        assert_eq!(LANGUAGES.len(), CATALOGS.len());
        assert_eq!(current(), "en");
        assert_eq!(text("no-such-message", &[]), "no-such-message");
    }
}
