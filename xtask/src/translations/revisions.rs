//! The text of each message as a review records it.
use super::catalog::{self, PatternInfo, Result};
use fluent_syntax::ast;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

fn dependencies_of(id: &str, dependencies: &BTreeMap<String, BTreeSet<String>>, seen: &mut BTreeSet<String>) {
    for dependency in &dependencies[id] {
        if seen.insert(dependency.clone()) {
            dependencies_of(dependency, dependencies, seen);
        }
    }
}

/// Each message value and attribute in canonical form, with the messages and terms it uses (theirs too), so changing
/// one string changes only the revisions that show it. Comments, order and formatting don't count.
pub(crate) fn revisions(text: &str) -> Result<BTreeMap<String, String>> {
    let text = text.replace("\r\n", "\n");
    catalog::parse(&text)?; // Duplicates, references to nothing and loops are refused first.
    let resource =
        fluent_syntax::parser::parse(text.as_str()).map_err(|(_, errors)| format!("Invalid Fluent: {errors:?}"))?;
    let mut values = BTreeMap::new();
    let mut dependencies = BTreeMap::new();
    for entry in &resource.body {
        let (id, value, attributes) = match entry {
            ast::Entry::Message(message) => (message.id.name.to_owned(), message.value.as_ref(), &message.attributes),
            ast::Entry::Term(term) => (format!("-{}", term.id.name), Some(&term.value), &term.attributes),
            _ => continue,
        };
        let patterns = value
            .map(|value| (id.clone(), value))
            .into_iter()
            .chain(attributes.iter().map(|attribute| (format!("{id}.{}", attribute.id.name), &attribute.value)));
        for (key, value) in patterns {
            let mut info = PatternInfo::default();
            catalog::pattern(value, &mut info);
            info.references.extend(info.terms);
            dependencies.insert(key.clone(), info.references);
            let single = ast::Resource {
                body: vec![ast::Entry::Message(ast::Message {
                    id: ast::Identifier { name: "value" },
                    value: Some(value.clone()),
                    attributes: Vec::new(),
                    comment: None,
                })],
            };
            let serialized = fluent_syntax::serializer::serialize(&single);
            values.insert(key, serialized.trim_start_matches("value =").trim().to_owned());
        }
    }
    let mut revisions = BTreeMap::new();
    for (id, value) in values.iter().filter(|(id, _)| !id.starts_with('-')) {
        let mut references = BTreeSet::new();
        dependencies_of(id, &dependencies, &mut references);
        let mut revision = value.clone();
        for reference in references {
            write!(revision, "\n\n# reference {reference}\n{}", values[&reference])?;
        }
        revisions.insert(id.clone(), revision);
    }
    Ok(revisions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revisions_track_attributes_variants_and_transitive_references_independently() {
        let source = "-brand = Hoard\na = { -brand }\n    .label = Open\nb = { a }\nc = Unchanged\ncount = { $n ->\n    [one] One file\n   *[other] Many files\n}\n";
        let before = revisions(source).expect("source");
        let changed = |after: &BTreeMap<String, String>| -> Vec<String> {
            before.keys().filter(|id| before[*id] != after[*id]).cloned().collect()
        };
        assert_eq!(changed(&revisions(&source.replace("Open", "Launch")).expect("attribute edit")), ["a.label"]);
        assert_eq!(changed(&revisions(&source.replace("Hoard", "New brand")).expect("term edit")), ["a", "b"]);
        assert_eq!(changed(&revisions(&source.replace("Many files", "Several files")).expect("variant")), ["count"]);
    }

    #[test]
    fn revisions_ignore_comments_order_and_formatting_but_preserve_literal_whitespace() {
        let before = revisions("a = Hello { $name }\nb = { \" \" }\n").expect("source");
        let after = revisions("# Comment\r\nb = {\" \"}\r\n\r\na = Hello {$name}\r\n").expect("formatting");
        assert_eq!(before, after);
        let after = revisions("a = Hello { $name }\nb = { \"  \" }\n").expect("literal edit");
        assert_ne!(before["b"], after["b"]);
        assert!(revisions("a = { b }\nb = { a }").is_err());
    }
}
