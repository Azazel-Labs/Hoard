//! `cargo xtask spelling`: Hoard is written in American English, so British spellings (`colour`, `grey`, `licence`,
//! `cancelled`, `organise`…) fail CI, in prose and in identifiers alike (`unreal_recolour`, `favouritesAsset`).
//!
//! It reads every tracked text file in English: the code and its comments, the docs, the English catalog and the
//! engine plugins (not the other languages' catalogs or READMEs, where `licence` is French). A line that has to
//! keep a British word, because it reads text written that way (a texture named `*_colour`, a `LICENCE` file) or
//! names something it doesn't own (Unreal's `EHttpFailureReason::Cancelled`), says `spelling: allow`. `--message`
//! checks a commit message instead, as `.githooks/commit-msg` does.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

/// Words that are British wherever they appear in a word, with their American spelling.
const INSIDE: &[(&str, &str)] = &[
    ("colour", "color"),
    ("favour", "favor"),
    ("behaviour", "behavior"),
    ("neighbour", "neighbor"),
    ("flavour", "flavor"),
    ("honour", "honor"),
    ("humour", "humor"),
    ("harbour", "harbor"),
    ("labour", "labor"),
    ("rumour", "rumor"),
    ("armour", "armor"),
    ("vapour", "vapor"),
    ("savour", "savor"),
    ("endeavour", "endeavor"),
    ("catalogue", "catalog"),
    ("artefact", "artifact"),
    ("aluminium", "aluminum"),
    ("greyscale", "grayscale"),
    ("centimetre", "centimeter"),
    ("millimetre", "millimeter"),
    ("kilometre", "kilometer"),
];

/// Words that are British only as a whole word (`grey` is, `greyhound` isn't), with their American spelling.
const WHOLE: &[(&str, &str)] = &[
    ("grey", "gray"),
    ("greys", "grays"),
    ("greyed", "grayed"),
    ("greying", "graying"),
    ("greyish", "grayish"),
    ("centre", "center"),
    ("centres", "centers"),
    ("centred", "centered"),
    ("centring", "centering"),
    ("metre", "meter"),
    ("metres", "meters"),
    ("licence", "license"),
    ("licences", "licenses"),
    ("licenced", "licensed"),
    ("cancelled", "canceled"),
    ("cancelling", "canceling"),
    ("travelled", "traveled"),
    ("travelling", "traveling"),
    ("traveller", "traveler"),
    ("travellers", "travelers"),
    ("labelled", "labeled"),
    ("labelling", "labeling"),
    ("modelled", "modeled"),
    ("modelling", "modeling"),
    ("signalled", "signaled"),
    ("signalling", "signaling"),
    ("levelled", "leveled"),
    ("levelling", "leveling"),
    ("fuelled", "fueled"),
    ("fuelling", "fueling"),
    ("totalled", "totaled"),
    ("analyse", "analyze"),
    ("analysed", "analyzed"),
    ("analysing", "analyzing"),
    ("whilst", "while"),
    ("amongst", "among"),
    ("programme", "program"),
    ("programmes", "programs"),
    ("defence", "defense"),
    ("offence", "offense"),
    ("learnt", "learned"),
    ("spelt", "spelled"),
    ("tyre", "tire"),
    ("tyres", "tires"),
    ("storey", "story"),
    ("storeys", "stories"),
    ("cheque", "check"),
    ("mould", "mold"),
    ("fulfil", "fulfill"),
    ("enrol", "enroll"),
    ("sceptical", "skeptical"),
];

/// Stems spelled with `-ise` in British English and `-ize` in American: `organise`, `normalisation`, `recognised`.
const ISE: &[&str] = &[
    "apolog",
    "author",
    "capital",
    "categor",
    "central",
    "character",
    "custom",
    "emphas",
    "final",
    "general",
    "harmon",
    "initial",
    "internation",
    "legal",
    "linear",
    "local",
    "material",
    "maxim",
    "memor",
    "minim",
    "modern",
    "monet",
    "neutral",
    "normal",
    "optim",
    "organ",
    "parameter",
    "personal",
    "priorit",
    "quant",
    "random",
    "rational",
    "real",
    "recogn",
    "sanit",
    "serial",
    "special",
    "stabil",
    "standard",
    "summar",
    "synchron",
    "token",
    "util",
    "vector",
    "visual",
];

/// What `-ise` stems take: the British endings, and the American one for each.
const ISE_ENDINGS: &[(&str, &str)] = &[
    ("ise", "ize"),
    ("ised", "ized"),
    ("ises", "izes"),
    ("ising", "izing"),
    ("iser", "izer"),
    ("isers", "izers"),
    ("isation", "ization"),
    ("isations", "izations"),
];

/// Files that keep a British word they can't change: the schema's past steps, which are never edited, and the tests
/// that run them; and Unity's `MonoBehaviour`, in its type tree's strings.
const KEPT: &[(&str, &str)] = &[
    ("crates/hoardd/src/db/schema/migrations.rs", "favourites"),
    ("crates/hoardd/src/db/schema/migrations.rs", "recolour"),
    ("crates/hoardd/src/db/schema/tests.rs", "favourites"),
    ("crates/hoardd/src/db/schema/tests.rs", "recolour"),
    ("crates/hoardd/src/unitymat/binary.rs", "behaviour"),
    ("crates/hoardd/src/unitymat/tests.rs", "behaviour"),
];

/// The kinds of file read, by extension.
const TEXT: &[&str] = &[
    "rs", "ts", "mts", "js", "vue", "css", "html", "md", "toml", "yml", "yaml", "ftl", "cs", "cpp", "h", "hpp", "py",
    "sh", "ps1", "uplugin", "json",
];

/// Paths not in English, or not Hoard's words: the other languages' catalogs, review records and READMEs, and lock
/// files and third-party license texts.
fn foreign(path: &str) -> bool {
    (path.starts_with("locales/") && !path.starts_with("locales/en/"))
        || path.starts_with("localization/")
        || path.starts_with("docs/i18n/")
        || path.starts_with("hoard-data/")
        || path == "xtask/src/spelling.rs"
        || path.ends_with("package-lock.json")
        || path.ends_with("package.json")
        || path == "web/public/licenses.json"
}

/// The American spelling of `word` (lowercase), if it's British.
fn american(word: &str) -> Option<String> {
    if let Some((_, us)) = WHOLE.iter().find(|(uk, _)| *uk == word) {
        return Some((*us).to_owned());
    }
    if let Some((uk, us)) = INSIDE.iter().find(|(uk, _)| word.contains(uk)) {
        return Some(word.replace(uk, us));
    }
    for prefix in ["", "re", "de", "un", "non"] {
        let Some(rest) = word.strip_prefix(prefix) else { continue };
        for stem in ISE {
            let Some(ending) = rest.strip_prefix(stem) else { continue };
            if let Some((_, us)) = ISE_ENDINGS.iter().find(|(uk, _)| *uk == ending) {
                return Some(format!("{prefix}{stem}{us}"));
            }
        }
    }
    None
}

/// The words in a line, lowercase, with identifiers split into theirs: `favouritesAsset` and `unreal_recolour` are
/// `favourites`, `asset`, `unreal` and `recolour`.
fn words(line: &str) -> impl Iterator<Item = String> + '_ {
    line.split(|c: char| !c.is_ascii_alphabetic()).filter(|w| !w.is_empty()).flat_map(|w| {
        let mut parts = Vec::new();
        let mut start = 0;
        let b = w.as_bytes();
        for i in 1..b.len() {
            if b[i - 1].is_ascii_lowercase() && b[i].is_ascii_uppercase() {
                parts.push(&w[start..i]);
                start = i;
            }
        }
        parts.push(&w[start..]);
        parts.into_iter().map(str::to_ascii_lowercase)
    })
}

/// Every British spelling in `text`, as `(line number, word, American spelling)`, leaving out lines that say
/// `spelling: allow` and the words `kept`.
fn british(text: &str, kept: &[&str]) -> Vec<(usize, String, String)> {
    let mut found = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if line.contains("spelling: allow") {
            continue;
        }
        for w in words(line) {
            if kept.contains(&w.as_str()) {
                continue;
            }
            if let Some(us) = american(&w) {
                found.push((n + 1, w, us));
            }
        }
    }
    found
}

pub(crate) fn run(root: &Path, message: Option<&Path>) -> Result<()> {
    let mut problems = Vec::new();
    if let Some(path) = message {
        let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let text = text.lines().filter(|l| !l.starts_with('#')).collect::<Vec<_>>().join("\n");
        for (n, w, us) in british(&text, &[]) {
            problems.push(format!("commit message:{n}: {w} → {us}"));
        }
    } else {
        let out = Command::new("git").arg("ls-files").current_dir(root).output().context("running git ls-files")?;
        for path in String::from_utf8(out.stdout)?.lines() {
            let ext = path.rsplit_once('.').map_or("", |(_, e)| e);
            if !TEXT.contains(&ext) || foreign(path) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(root.join(path)) else { continue };
            let kept: Vec<&str> = KEPT.iter().filter(|(p, _)| *p == path).map(|(_, w)| *w).collect();
            for (n, w, us) in british(&text, &kept) {
                problems.push(format!("{path}:{n}: {w} → {us}"));
            }
        }
    }
    if problems.is_empty() {
        return Ok(());
    }
    for p in &problems {
        println!("{p}");
    }
    bail!(
        "{} British spelling(s): Hoard is written in American English (a line that must keep one says `spelling: allow`)",
        problems.len()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn british_spellings_are_found_in_prose_and_identifiers() {
        let found = |s: &str| british(s, &[]).into_iter().map(|(_, w, us)| format!("{w}>{us}")).collect::<Vec<_>>();
        assert_eq!(found("The colours were greyed out."), ["colours>colors", "greyed>grayed"]);
        assert_eq!(found("fn unreal_recolour(favouritesAsset: u8)"), ["recolour>recolor", "favourites>favorites"]);
        assert_eq!(
            found("We normalise, then deserialisation, unrecognised"),
            ["normalise>normalize", "deserialisation>deserialization", "unrecognised>unrecognized"]
        );
        assert_eq!(found("LICENCE and Cancelled"), ["licence>license", "cancelled>canceled"]);
        let none: [String; 0] = [];
        assert_eq!(
            found("a greyhound, an analysis, its analyses, a promise, the exercise, expertise, the center"),
            none
        );
        assert_eq!(found("const C: &str = \"colour\"; // spelling: allow"), none);
        assert_eq!(british("CREATE TABLE favourites", &["favourites"]), []);
    }
}
