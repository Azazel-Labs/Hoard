//! The Fluent catalogs in `locales/`, read and checked: their syntax, that each translation has every English message
//! and no others, and that none uses a variable the English doesn't pass. Shared by hoardd's build script, which embeds
//! them, and `cargo xtask translations`.
use fluent_syntax::ast::{self, Expression, InlineExpression, PatternElement};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::{Path, PathBuf},
};

/// Each message (and `message.attribute`), with the variables it uses, its references' included.
pub(crate) type Messages = BTreeMap<String, BTreeSet<String>>;
pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Every locale's catalog has this name, in a folder named for the locale.
pub(crate) const FILE: &str = "hoard.ftl";

/// What a pattern uses.
#[derive(Debug, Default)]
pub(crate) struct PatternInfo {
    pub(crate) variables: BTreeSet<String>,
    pub(crate) references: BTreeSet<String>,
    pub(crate) terms: BTreeSet<String>,
}

pub(crate) fn pattern(value: &ast::Pattern<&str>, info: &mut PatternInfo) {
    for element in &value.elements {
        if let PatternElement::Placeable { expression: value } = element {
            expression(value, info);
        }
    }
}

fn expression(value: &Expression<&str>, info: &mut PatternInfo) {
    match value {
        Expression::Inline(value) => inline(value, info),
        Expression::Select { selector, variants } => {
            inline(selector, info);
            for variant in variants {
                pattern(&variant.value, info);
            }
        }
    }
}

fn arguments(args: &ast::CallArguments<&str>, info: &mut PatternInfo) {
    for value in &args.positional {
        inline(value, info);
    }
    for value in &args.named {
        inline(&value.value, info);
    }
}

fn inline(value: &InlineExpression<&str>, info: &mut PatternInfo) {
    match value {
        InlineExpression::VariableReference { id } => {
            info.variables.insert(id.name.into());
        }
        InlineExpression::MessageReference { id, attribute } => {
            info.references
                .insert(attribute.as_ref().map_or_else(|| id.name.into(), |attr| format!("{}.{}", id.name, attr.name)));
        }
        InlineExpression::TermReference { id, attribute, arguments: args } => {
            info.terms.insert(
                attribute
                    .as_ref()
                    .map_or_else(|| format!("-{}", id.name), |attr| format!("-{}.{}", id.name, attr.name)),
            );
            if let Some(args) = args {
                arguments(args, info);
            }
        }
        InlineExpression::FunctionReference { arguments: args, .. } => arguments(args, info),
        InlineExpression::Placeable { expression: value } => expression(value, info),
        InlineExpression::StringLiteral { .. } | InlineExpression::NumberLiteral { .. } => {}
    }
}

fn references_of(
    id: &str,
    entries: &BTreeMap<String, PatternInfo>,
    stack: &mut BTreeSet<String>,
) -> Result<BTreeSet<String>> {
    if !stack.insert(id.into()) {
        return Err(format!("Cyclic Fluent reference: {id}").into());
    }
    let info = entries.get(id).ok_or_else(|| format!("Missing Fluent reference: {id}"))?;
    let mut vars = info.variables.clone();
    for reference in &info.references {
        vars.extend(references_of(reference, entries, stack)?);
    }
    // A term's variables are its own; only what the caller passes it reaches the message.
    for term in &info.terms {
        references_of(term, entries, stack)?;
    }
    stack.remove(id);
    Ok(vars)
}

/// The messages in a catalog, refusing bad syntax, a duplicate, a reference to nothing and a reference loop.
pub(crate) fn parse(text: &str) -> Result<Messages> {
    let resource = fluent_syntax::parser::parse(text).map_err(|(_, errors)| format!("Invalid Fluent: {errors:?}"))?;
    let mut entries = BTreeMap::new();
    let mut ids = BTreeSet::new();
    for entry in &resource.body {
        let (id, value, attributes) = match entry {
            ast::Entry::Message(message) => (message.id.name.to_owned(), message.value.as_ref(), &message.attributes),
            ast::Entry::Term(term) => (format!("-{}", term.id.name), Some(&term.value), &term.attributes),
            _ => continue,
        };
        if !ids.insert(id.clone()) {
            return Err(format!("Duplicate Fluent ID: {id}").into());
        }
        if let Some(value) = value {
            let mut info = PatternInfo::default();
            pattern(value, &mut info);
            entries.insert(id.clone(), info);
        }
        for attribute in attributes {
            let key = format!("{id}.{}", attribute.id.name);
            let mut info = PatternInfo::default();
            pattern(&attribute.value, &mut info);
            if entries.insert(key.clone(), info).is_some() {
                return Err(format!("Duplicate attribute: {key}").into());
            }
        }
    }
    let mut messages = BTreeMap::new();
    for id in entries.keys() {
        let variables = references_of(id, &entries, &mut BTreeSet::new())?;
        if !id.starts_with('-') {
            messages.insert(id.clone(), variables);
        }
    }
    Ok(messages)
}

/// Every locale folder's catalog, sorted by locale.
pub(crate) fn files(directory: &Path) -> Result<Vec<(String, PathBuf)>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }
        let locale = path.file_name().and_then(|s| s.to_str()).ok_or("invalid locale directory")?;
        locale.parse::<unic_langid::LanguageIdentifier>()?;
        files.push((locale.to_owned(), path.join(FILE)));
    }
    files.sort();
    Ok(files)
}

/// Every catalog, each translation checked against the English.
pub(crate) fn load(directory: &Path) -> Result<BTreeMap<String, Messages>> {
    let source = parse(&fs::read_to_string(directory.join("en").join(FILE))?)?;
    let mut catalogs = BTreeMap::new();
    for (locale, path) in files(directory)? {
        if locale == "en" {
            continue;
        }
        let messages = parse(&fs::read_to_string(path)?).map_err(|e| format!("{locale}: {e}"))?;
        for id in source.keys() {
            if !messages.contains_key(id) {
                return Err(format!("{locale}: missing translation for {id}").into());
            }
        }
        for (id, variables) in &messages {
            let expected = source.get(id).ok_or_else(|| format!("{locale}: unknown message {id}"))?;
            // A language may leave out a variable it doesn't need, but can't ask for one nobody passes.
            if !variables.is_subset(expected) {
                return Err(format!("{locale}: unexpected variables for {id}: {variables:?}").into());
            }
        }
        catalogs.insert(locale, messages);
    }
    catalogs.insert("en".into(), source);
    Ok(catalogs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_every_source_message_and_attribute() {
        let root = std::env::temp_dir().join(format!("hoard-catalog-test-{}", std::process::id()));
        fs::create_dir_all(root.join("en")).expect("source directory");
        fs::create_dir_all(root.join("fr")).expect("translation directory");
        fs::write(root.join("en").join(FILE), "hello = Hello\nbutton =\n    .label = Save\n").expect("source");
        fs::write(root.join("fr").join(FILE), "hello = Bonjour\n").expect("incomplete translation");
        assert!(load(&root).expect_err("missing attribute").to_string().contains("button.label"));
        fs::write(root.join("fr").join(FILE), "hello = Bonjour\nbutton =\n    .label = Enregistrer\n")
            .expect("complete translation");
        assert_eq!(load(&root).expect("complete catalogs").len(), 2);
        fs::write(root.join("fr").join(FILE), "hello = Bonjour { $who }\nbutton =\n    .label = Enregistrer\n")
            .expect("invented variable");
        assert!(load(&root).expect_err("invented variable").to_string().contains("unexpected variables"));
        fs::remove_dir_all(root).expect("remove test fixtures");
    }

    #[test]
    fn rejects_malformed_duplicate_and_unresolved_messages() {
        for bad in ["broken =", "a = One\na = Two", "a = { absent }", "a = { b }\nb = { a }"] {
            assert!(parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn accepts_variants_attributes_and_scoped_terms() {
        let messages = parse("-user = { $gender ->\n [female] Elle\n *[other] Il\n}\nresult = { -user(gender: \"female\") }\ncount = { $count ->\n [one] One\n *[other] { $count } files\n}\ncopy = { count }\nbutton =\n .label = Cancel").expect("valid Fluent");
        assert!(messages["result"].is_empty());
        assert_eq!(messages["copy"], BTreeSet::from(["count".into()]));
        assert!(messages.contains_key("button.label"));
    }
}
