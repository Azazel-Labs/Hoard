//! Ordering what the stores found: by the words a listing shares with the pack's name, and by how well it fits
//! what the pack's files say.

use std::collections::HashMap;

use super::clues::{Clues, same_title};
use super::{MAX_MATCHES, PER_STORE, StoreMatch, Why, same_page};

/// Words in pack names that say where a pack is from or what it's for, not what it is.
const NOISE: &[&str] = &[
    "unity",
    "unreal",
    "ue",
    "ue4",
    "ue5",
    "engine",
    "godot",
    "fab",
    "itch",
    "itchio",
    "humble",
    "kenney",
    "opengameart",
    "zip",
    "rar",
    "7z",
    "final",
    "copy",
];

/// The words of a pack name worth searching for: separators become spaces, `CamelCase` is split, and version
/// numbers and engine or store names are dropped.
pub(crate) fn search_terms(name: &str) -> String {
    let mut spaced = String::with_capacity(name.len() + 8);
    let mut prev: Option<char> = None;
    for c in name.chars() {
        if !c.is_alphanumeric() && c != '\'' {
            spaced.push(' ');
        } else {
            if c.is_uppercase() && prev.is_some_and(char::is_lowercase) {
                spaced.push(' ');
            }
            spaced.push(c);
        }
        prev = Some(c);
    }
    spaced
        .split_whitespace()
        .filter(|w| {
            let lower = w.to_lowercase();
            let version =
                lower.strip_prefix('v').is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
            !version && !lower.chars().all(|c| c.is_ascii_digit()) && !NOISE.contains(&lower.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn words(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for w in s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
        let mut w = w.to_lowercase();
        // "trees" finds "tree".
        if w.len() > 3 && w.ends_with('s') && !w.ends_with("ss") {
            w.pop();
        }
        if !out.contains(&w) {
            out.push(w);
        }
    }
    out
}

/// How well a title matches the query, 0 to 1: mostly how many of the query's words it has, a little how few
/// other words.
fn score(query: &[String], title: &str) -> f32 {
    let title = words(title);
    if query.is_empty() || title.is_empty() {
        return 0.0;
    }
    #[expect(clippy::cast_precision_loss, reason = "word counts are tiny")]
    let (hits, q, t) =
        (query.iter().filter(|w| title.contains(w)).count() as f32, query.len() as f32, title.len() as f32);
    0.7 * hits / q + 0.3 * hits / t
}

/// Letters and digits only, lower case, for comparing names written differently (`Synty Studios`, `SyntyStudios`).
pub(super) fn squash(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

/// The number a listing's technical details give for something ("Number of Meshes: 496", "173 textures").
fn spec_count(specs: &str, stem: &str) -> Option<u64> {
    let lower = specs.to_lowercase();
    let number = |s: &str| -> Option<u64> {
        let digits: String = s
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit() || *c == ',')
            .filter(char::is_ascii_digit)
            .collect();
        digits.parse().ok()
    };
    lower.match_indices(stem).find_map(|(i, _)| {
        // "Meshes: 496": a number just after; "496 static meshes": a number just before.
        let after: String = lower[i..].chars().skip_while(|c| c.is_alphabetic()).take(14).collect();
        let after_ok = after.trim_start().starts_with(|c: char| c == ':' || c == '-' || c == '(' || c.is_ascii_digit());
        if after_ok && let Some(n) = number(&after) {
            return Some(n);
        }
        let before: String = lower[..i].chars().rev().take(18).collect::<Vec<_>>().into_iter().rev().collect();
        let tail = before.split(['.', ';', ':', '\n']).next_back().unwrap_or("");
        let n: String = tail.split_whitespace().rfind(|w| w.chars().any(|c| c.is_ascii_digit()))?.to_owned();
        number(&n)
    })
}

/// Close enough to be the same pack: within 15%.
#[expect(clippy::cast_precision_loss, reason = "asset counts are far below 2^52")]
fn about(a: u64, b: u64) -> bool {
    a > 0 && b > 0 && (a.min(b) as f64) / (a.max(b) as f64) >= 0.85
}

/// Score each match against the name and the clues and keep the likely ones, best first.
pub(super) fn rank(query: &str, clues: &Clues, matches: Vec<StoreMatch>) -> Vec<StoreMatch> {
    let primary = words(query);
    let others: Vec<Vec<String>> = clues.names.iter().map(|n| words(n)).collect();
    let mut kept: Vec<StoreMatch> = Vec::new();
    for mut m in matches {
        if let Some(k) = kept.iter_mut().find(|k| same_page(&k.url, &m.url)) {
            // The same listing from two searches: keep one, with every reason.
            for w in m.why.drain(..) {
                if !k.why.contains(&w) {
                    k.why.push(w);
                }
            }
            continue;
        }
        let by_name = others.iter().map(|o| 0.9 * score(o, &m.title)).fold(score(&primary, &m.title), f32::max);
        let mut s = by_name;
        if m.why.iter().any(|w| matches!(w, Why::Linked(_))) {
            s = 1.0;
        }
        if let Some((title, from)) = &clues.title
            && same_title(title, &m.title)
        {
            s = 1.0;
            m.why.push(Why::SameTitle(from.clone()));
        }
        if let Some(author) = m.author.as_deref().map(squash).filter(|a| a.len() >= 3)
            && let Some((seller, from)) = clues.sellers.iter().find(|(s, _)| {
                let s = squash(s);
                s.len() >= 3 && (author.contains(&s) || s.contains(&author))
            })
        {
            s += 0.35;
            let author = m.author.clone().unwrap_or_else(|| seller.clone());
            m.why.push(Why::By { author, place: from.clone() });
        }
        if let Some(specs) = m.specs.as_deref() {
            if clues.meshes >= 5 && spec_count(specs, "mesh").is_some_and(|n| about(n, clues.meshes)) {
                s += 0.2;
                m.why.push(Why::Meshes(clues.meshes));
            }
            if clues.textures >= 5 && spec_count(specs, "texture").is_some_and(|n| about(n, clues.textures)) {
                s += 0.15;
                m.why.push(Why::Textures(clues.textures));
            }
        }
        // A listing for another engine is less likely, though the same art is often sold for several.
        if let Some(engine) = clues.engine
            && !m.formats.is_empty()
            && !m.formats.iter().any(|f| f == engine)
        {
            s -= 0.15;
        }
        if by_name <= 0.0 && m.why.is_empty() {
            continue;
        }
        // Knowing the exact title, a listing with another one (the same scan's "Old 04") is a near miss at best.
        if let Some((title, _)) = &clues.title
            && !same_title(title, &m.title)
            && !m.why.iter().any(|w| matches!(w, Why::Linked(_)))
        {
            s = s.min(0.85);
        }
        m.score = (s.clamp(0.0, 1.0) * 100.0).round() / 100.0;
        kept.push(m);
    }
    kept.sort_by(|a, b| b.score.total_cmp(&a.score).then(b.why.len().cmp(&a.why.len())));
    let mut per: HashMap<&'static str, usize> = HashMap::new();
    kept.retain(|m| {
        let n = per.entry(m.store).or_default();
        *n += 1;
        *n <= PER_STORE || !m.why.is_empty()
    });
    kept.truncate(MAX_MATCHES);
    kept
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::sidecar;
    use crate::store_search::clues::clues;
    use crate::store_search::tests::{fab_match, matches};
    use crate::store_search::{CC_BY_4, FAB_STANDARD, Place, found};

    #[test]
    fn cleans_pack_names_into_search_terms() {
        assert_eq!(search_terms("MedievalVillage_v1_3"), "Medieval Village");
        assert_eq!(search_terms("POLYGON - Fantasy Kingdom (Unity 2020)"), "POLYGON Fantasy Kingdom");
        assert_eq!(search_terms("kenney_platformer-pack-redux"), "platformer pack redux");
        assert_eq!(search_terms("UE5 Stylized Trees V2"), "Stylized Trees");
    }

    #[test]
    fn megascans_files_name_the_listing() {
        let paths: Vec<String> = [
            "Historic_Miscellaneous_Stand_Wood_Old_05_xeokah0_Raw.fbx",
            "Historic_Miscellaneous_Stand_Wood_Old_05_xeokah0_Raw_8K_BaseColor.jpg",
            "xeokah0.json",
        ]
        .map(String::from)
        .to_vec();
        let mut c = clues("historic_miscellaneous_stand_wood_old_05_xeokah0_raw", None, &paths, &BTreeMap::new());
        c.add_sidecars(&[sidecar::read("xeokah0.json", sidecar::tests::STAND).unwrap()]);
        assert_eq!(
            c.title,
            Some(("Historic Miscellaneous Stand Wood Old 05".to_owned(), Place::File("Megascans file".to_owned())))
        );
        assert_eq!(c.sellers[0].0, "Quixel Megascans");
        assert!(c.describe()[0].starts_with("Title: Historic Miscellaneous Stand Wood Old 05"));

        let m = |title: &str, author: &str| {
            let mut m = found("Fab", title, format!("https://www.fab.com/listings/{}", squash(title))).unwrap();
            m.author = Some(author.to_owned());
            m
        };
        let ranked = rank(
            "Historic Miscellaneous Stand Wood Old 05",
            &c,
            vec![
                m("Historic Miscellaneous Door Wood Old 01", "Quixel Megascans"),
                m("Old Styled Wooden Info Stand", "Incg5764"),
                m("Historic Miscellaneous Stand Wood Old 04", "Quixel Megascans"),
                m("Historic Miscellaneous Stand Wood Old 05", "Quixel Megascans"),
            ],
        );
        assert_eq!(ranked[0].title, "Historic Miscellaneous Stand Wood Old 05");
        assert!((ranked[0].score - 1.0).abs() < f32::EPSILON);
        assert!(ranked[0].why.iter().any(|w| w.to_string() == "Same title as the pack's Megascans file"));
        assert!(ranked[1].score < 1.0, "a different number isn't the same listing");
        assert!(same_title("stand_wood_old_05", "Stand Wood Old 05") && !same_title("Old 05", "Old 04"));
    }

    #[test]
    fn only_a_listing_the_pack_names_is_certain() {
        let mut c = Clues::default();
        c.add_sidecars(&[sidecar::read("xeokah0.json", sidecar::tests::STAND).unwrap()]);
        let stand = "Historic Miscellaneous Stand Wood Old 05";
        let found = matches(rank(
            stand,
            &c,
            vec![
                fab_match(stand, "Quixel Megascans", &["Personal", "Professional"]),
                fab_match("Historic Miscellaneous Stand Wood Old 04", "Quixel Megascans", &["Personal"]),
            ],
        ));
        let sure = found.certain().unwrap();
        assert_eq!(
            (sure.url(), sure.license()),
            ("https://www.fab.com/listings/historicmiscellaneousstandwoodold05", Some(FAB_STANDARD))
        );

        // The same words aren't enough: plain names have many listings.
        let plain = matches(rank("medieval village", &Clues::default(), vec![fab_match("Medieval Village", "A", &[])]));
        assert!((plain.matches[0].score - 1.0).abs() < f32::EPSILON && plain.certain().is_none());

        // Two pages the README links to: which one is it?
        let mut two = matches(vec![fab_match("One", "A", &[]), fab_match("Two", "B", &["CC-BY"])]);
        for m in &mut two.matches {
            m.score = 1.0;
            m.why.push(Why::Linked(Place::Readme));
        }
        assert!(two.certain().is_none());
        assert_eq!(two.matches[1].license(), Some(CC_BY_4));
        two.matches.truncate(1);
        assert_eq!(two.certain().map(StoreMatch::url), Some("https://www.fab.com/listings/one"));
    }

    #[test]
    fn ranks_by_shared_words() {
        let m = |store, title: &str| found(store, title, format!("https://example.com/{}", squash(title))).unwrap();
        let ranked = rank(
            "medieval village",
            &Clues::default(),
            vec![
                m("Fab", "Space Station"),
                m("Fab", "Medieval Village Megakit Ultimate Edition"),
                m("itch.io", "Medieval Villages"),
            ],
        );
        let titles: Vec<&str> = ranked.iter().map(|m| m.title.as_str()).collect();
        assert_eq!(titles, ["Medieval Villages", "Medieval Village Megakit Ultimate Edition"]);
        assert!((ranked[0].score - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn reads_counts_from_technical_details() {
        let specs = "Number of Meshes: 496 Static meshes in total\nTexture: 173\nCollision: Yes";
        assert_eq!(spec_count(specs, "mesh"), Some(496));
        assert_eq!(spec_count(specs, "texture"), Some(173));
        assert_eq!(spec_count("Includes 1,204 textures and 80 materials", "texture"), Some(1204));
        assert_eq!(spec_count("Nanite: Yes", "mesh"), None);
        assert!(about(496, 480) && !about(496, 300));
    }

    #[test]
    fn clues_pick_the_listing_among_same_named_ones() {
        let listing = |title: &str, author: &str, url: &str, specs: &str| {
            let mut m = found("Fab", title, format!("https://www.fab.com/listings/{url}")).unwrap();
            m.author = Some(author.to_owned());
            m.formats = vec!["unreal-engine".to_owned()];
            m.specs = Some(specs.to_owned());
            m
        };
        let c = Clues {
            sellers: vec![("Marco Maria Rossi".into(), Place::Readme)],
            engine: Some("unreal-engine"),
            meshes: 490,
            ..Clues::default()
        };
        let ranked = rank(
            "Medieval Village",
            &c,
            vec![
                listing("Medieval Village", "Anatolii Umanets", "a", "Meshes: 40"),
                listing("Medieval Village", "Marco Maria Rossi", "b", "Number of Meshes: 496"),
                listing("Medieval Village", "Anatolii Umanets", "a", ""),
            ],
        );
        assert_eq!(ranked.len(), 2, "the same listing found twice is kept once");
        assert_eq!(ranked[0].url, "https://www.fab.com/listings/b");
        assert_eq!(ranked[0].why.len(), 2);
        assert!(ranked[0].why[0].to_string().contains("Marco Maria Rossi"));
    }
}
