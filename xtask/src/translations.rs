//! `cargo xtask translations`: checks the Fluent catalogs in `locales/` against each other and against the code that
//! uses them (`tr!` in hoardd, `t()` in the portal), formats them, and records reviews (see docs/translations.md).
use std::{
    collections::{BTreeMap, BTreeSet, btree_map::Entry},
    fs,
    path::Path,
};
use syn::{parse::Parser, visit::Visit};
#[path = "../../localization/catalog.rs"]
mod catalog;
mod review;
mod revisions;

use catalog::Result;

/// Each message the code asks for, with the arguments it passes and where it first asks.
#[derive(Debug, Default)]
struct Calls {
    messages: BTreeMap<String, (BTreeSet<String>, String)>,
    errors: Vec<String>,
}

impl Calls {
    fn add(&mut self, id: String, arguments: BTreeSet<String>, place: String) {
        match self.messages.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert((arguments, place));
            }
            Entry::Occupied(entry) if entry.get().0 != arguments => {
                let (first, at) = entry.get();
                self.errors.push(format!(
                    "{place}: {:?} is given {arguments:?}, but {first:?} at {at}; pass the same arguments everywhere",
                    entry.key()
                ));
            }
            Entry::Occupied(_) => {}
        }
    }
}

/// `tr!` calls in Rust.
struct Rust<'a> {
    calls: &'a mut Calls,
    file: String,
}

impl<'ast> Visit<'ast> for Rust<'_> {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if node.path.segments.last().is_some_and(|part| part.ident == "tr") {
            self.message(node.tokens.clone());
        } else {
            self.nested(node.tokens.clone());
        }
    }
}

impl Rust<'_> {
    fn message(&mut self, tokens: proc_macro2::TokenStream) {
        // A `tr!` among the arguments (`tr!("a", what = tr!("b"))`) is a message too.
        self.nested(tokens.clone().into_iter().skip(1).collect());
        let parser = |input: syn::parse::ParseStream<'_>| {
            let id: syn::LitStr = input.parse()?;
            let mut arguments = BTreeSet::new();
            while !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
                if input.is_empty() {
                    break;
                }
                let name: syn::Ident = input.parse()?;
                input.parse::<syn::Token![=]>()?;
                input.parse::<syn::Expr>()?;
                if !arguments.insert(name.to_string()) {
                    return Err(syn::Error::new(name.span(), "duplicate argument"));
                }
            }
            Ok((id, arguments))
        };
        match parser.parse2(tokens) {
            Ok((id, arguments)) => {
                let place = format!("{}:{}", self.file, id.span().start().line);
                self.calls.add(id.value(), arguments, place);
            }
            Err(error) => self.calls.errors.push(format!("{}:{}: {error}", self.file, error.span().start().line)),
        }
    }

    /// syn doesn't look inside other macros (`format!`, `json!`…), so `tr!` is found in their tokens by hand.
    fn nested(&mut self, tokens: proc_macro2::TokenStream) {
        use proc_macro2::TokenTree;
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        let mut i = 0;
        while i < tokens.len() {
            if matches!(&tokens[i], TokenTree::Ident(name) if name == "tr")
                && matches!(tokens.get(i + 1), Some(TokenTree::Punct(p)) if p.as_char() == '!')
                && let Some(TokenTree::Group(group)) = tokens.get(i + 2)
            {
                self.message(group.stream());
                i += 3;
                continue;
            }
            if let TokenTree::Group(group) = &tokens[i] {
                self.nested(group.stream());
            }
            i += 1;
        }
    }
}

fn rust_calls(directory: &Path, root: &Path, calls: &mut Calls) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            rust_calls(&path, root, calls)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let file = syn::parse_file(&fs::read_to_string(&path)?)?;
            let name = path.strip_prefix(root).unwrap_or(&path).display().to_string().replace('\\', "/");
            Rust { calls, file: name }.visit_file(&file);
        }
    }
    Ok(())
}

fn web_calls(directory: &Path, root: &Path, calls: &mut Calls) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            web_calls(&path, root, calls)?;
        } else if path.extension().is_some_and(|ext| ext == "vue" || ext == "ts") {
            let name = path.strip_prefix(root).unwrap_or(&path).display().to_string().replace('\\', "/");
            // Where `t` itself lives: its examples aren't messages.
            if name != "web/src/i18n.ts" {
                scan_web(&fs::read_to_string(&path)?, &name, calls);
            }
        }
    }
    Ok(())
}

/// The portal's `t('id')` and `t('id', { name: value, other })` calls: the ID a literal, the arguments an object
/// literal, so both can be checked against the catalog.
fn scan_web(text: &str, file: &str, calls: &mut Calls) {
    let b = text.as_bytes();
    let line = |at: usize| text[..at].bytes().filter(|&c| c == b'\n').count() + 1;
    let mut from = 0;
    while let Some(found) = text[from..].find("t(") {
        let at = from + found;
        from = at + 2;
        if at > 0 && (b[at - 1].is_ascii_alphanumeric() || matches!(b[at - 1], b'_' | b'$' | b'.')) {
            continue;
        }
        let place = format!("{file}:{}", line(at));
        // The search goes on inside the call, for a `t()` among its arguments.
        match web_call(b, at + 2) {
            Ok((id, arguments)) => {
                calls.add(id, arguments, place);
            }
            Err(error) => calls.errors.push(format!("{place}: {error}")),
        }
    }
}

fn skip_space(b: &[u8], mut i: usize) -> usize {
    while i < b.len() && b[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

/// Past the string literal starting at `i` (its quote), with its text; `${…}` in a template literal is skipped over.
fn string(b: &[u8], i: usize) -> std::result::Result<(usize, String, bool), String> {
    let quote = b[i];
    let mut j = i + 1;
    let mut text = Vec::new();
    let mut dynamic = false;
    while j < b.len() {
        match b[j] {
            b'\\' => {
                text.extend_from_slice(&b[j..(j + 2).min(b.len())]);
                j += 2;
            }
            b'$' if quote == b'`' && b.get(j + 1) == Some(&b'{') => {
                dynamic = true;
                j = balanced(b, j + 1)?;
            }
            c if c == quote => return Ok((j + 1, String::from_utf8_lossy(&text).into_owned(), dynamic)),
            c => {
                text.push(c);
                j += 1;
            }
        }
    }
    Err("unterminated string".into())
}

/// Past the bracket group opening at `i`.
fn balanced(b: &[u8], i: usize) -> std::result::Result<usize, String> {
    let mut depth = 0;
    let mut j = i;
    while j < b.len() {
        match b[j] {
            b'\'' | b'"' | b'`' => {
                j = string(b, j)?.0;
                continue;
            }
            b'{' | b'[' | b'(' => depth += 1,
            b'}' | b']' | b')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(j + 1);
                }
            }
            _ => {}
        }
        j += 1;
    }
    Err("unbalanced brackets".into())
}

/// The object literal's keys at `i` (its `{`), and where it ends.
fn object_keys(b: &[u8], i: usize) -> std::result::Result<(BTreeSet<String>, usize), String> {
    let end = balanced(b, i)?;
    let mut keys = BTreeSet::new();
    let mut j = i + 1;
    loop {
        j = skip_space(b, j);
        if j >= end - 1 {
            return Ok((keys, end));
        }
        let key = if matches!(b[j], b'\'' | b'"') {
            let (next, text, _) = string(b, j)?;
            j = next;
            text
        } else {
            let start = j;
            while j < end && (b[j].is_ascii_alphanumeric() || matches!(b[j], b'_' | b'$')) {
                j += 1;
            }
            if start == j {
                return Err("pass t() its arguments as `{ name: value }`, without spreads or computed names".into());
            }
            String::from_utf8_lossy(&b[start..j]).into_owned()
        };
        keys.insert(key);
        j = skip_space(b, j);
        if b[j] == b':' {
            // The value runs to the next comma at this level.
            j += 1;
            while j < end - 1 && b[j] != b',' {
                j = match b[j] {
                    b'\'' | b'"' | b'`' => string(b, j)?.0,
                    b'{' | b'[' | b'(' => balanced(b, j)?,
                    _ => j + 1,
                };
            }
        }
        if b[j] == b',' {
            j += 1;
        }
    }
}

/// The call whose arguments start at `i`: its message ID and argument names, and where it ends.
fn web_call(b: &[u8], i: usize) -> std::result::Result<(String, BTreeSet<String>), String> {
    let mut j = skip_space(b, i);
    if !matches!(b.get(j), Some(b'\'' | b'"' | b'`')) {
        return Err("t() needs the message ID written out, so it can be checked".into());
    }
    let (next, id, dynamic) = string(b, j)?;
    if dynamic {
        return Err(format!("t() needs the message ID written out, not built (`{id}`)"));
    }
    j = skip_space(b, next);
    let mut arguments = BTreeSet::new();
    if b.get(j) == Some(&b',') {
        j = skip_space(b, j + 1);
        if b.get(j) == Some(&b'{') {
            let (keys, end) = object_keys(b, j)?;
            arguments = keys;
            j = skip_space(b, end);
            if b.get(j) == Some(&b',') {
                j = skip_space(b, j + 1);
            }
        }
    }
    if b.get(j) != Some(&b')') {
        return Err(format!("t({id:?}…) takes the ID and an object literal of arguments, nothing else"));
    }
    Ok((id, arguments))
}

#[derive(Clone, Debug, clap::Subcommand)]
pub(crate) enum Action {
    /// Check the catalogs, their reviews, formatting, and every message the code uses
    Check,
    /// Record reviews of the strings named (or the README) for one locale
    Review {
        /// One locale, for example fr or zh-Hant
        locale: String,
        /// Message IDs, attributes included, such as button.label
        ids: Vec<String>,
        /// Review every string and the README in this locale
        #[arg(long, conflicts_with_all = ["ids", "readme"])]
        all: bool,
        /// Review only the README, accepting no changed strings
        #[arg(long, conflicts_with = "ids")]
        readme: bool,
    },
    /// Format the catalogs in place
    Fmt {
        /// Report catalogs that aren't formatted instead of rewriting them
        #[arg(long)]
        check: bool,
    },
}

pub(crate) fn run(root: &Path, action: Action) -> Result<()> {
    let directory = root.join("locales");
    match action {
        Action::Fmt { check } => format_catalogs(&directory, !check),
        Action::Review { locale, ids, all, readme } => review::run(root, &locale, &ids, all, readme),
        Action::Check => check(root, &directory),
    }
}

fn check(root: &Path, directory: &Path) -> Result<()> {
    let mut calls = Calls::default();
    rust_calls(&root.join("crates/hoardd/src"), root, &mut calls)?;
    web_calls(&root.join("web/src"), root, &mut calls)?;
    if !calls.errors.is_empty() {
        return Err(calls.errors.join("\n").into());
    }
    let catalogs = catalog::load(directory)?;
    let mut stale = Vec::new();
    for locale in catalogs.keys().filter(|locale| *locale != "en") {
        stale.extend(review::check(root, locale)?);
    }
    if !stale.is_empty() {
        return Err(format!(
            "{}\nReview the listed IDs with cargo xtask translations review <locale> <id>...",
            stale.join("\n\n")
        )
        .into());
    }
    let source = catalogs.get("en").ok_or("missing English catalog")?;
    let mut errors = Vec::new();
    for (id, (arguments, place)) in &calls.messages {
        match source.get(id) {
            None => errors.push(format!("{place}: {id:?} isn't in locales/en/{}", catalog::FILE)),
            Some(variables) if variables != arguments => {
                errors.push(format!("{place}: {id:?} is given {arguments:?}, but the English uses {variables:?}"));
            }
            Some(_) => {}
        }
    }
    for id in source.keys().filter(|id| !calls.messages.contains_key(*id)) {
        errors.push(format!("{id:?} is in locales/en/{} but nothing uses it", catalog::FILE));
    }
    if !errors.is_empty() {
        return Err(errors.join("\n").into());
    }
    format_catalogs(directory, false)?;
    #[expect(clippy::print_stdout, reason = "a CLI command's summary")]
    {
        println!("Checked {} messages in {} catalogs.", calls.messages.len(), catalogs.len());
    }
    Ok(())
}

fn formatted(source: &str) -> Result<String> {
    // Windows checkouts are read as they are; what's written is the same on every system.
    let source = source.replace("\r\n", "\n");
    let resource = fluent_syntax::parser::parse(source.as_str())
        .map_err(|(_, errors)| format!("Invalid Fluent; refusing to format: {errors:?}"))?;
    Ok(fluent_syntax::serializer::serialize(&resource))
}

fn format_catalogs(directory: &Path, write: bool) -> Result<()> {
    let mut changes = Vec::new();
    for (_, path) in catalog::files(directory)? {
        let source = fs::read_to_string(&path)?;
        let output = formatted(&source).map_err(|error| format!("{}: {error}", path.display()))?;
        if source.replace("\r\n", "\n") != output {
            changes.push((path, output));
        }
    }
    if !write && !changes.is_empty() {
        let paths = changes.iter().map(|(path, _)| path.display().to_string()).collect::<Vec<_>>().join("\n");
        return Err(format!("Fluent formatting differs:\n{paths}\nRun cargo xtask translations fmt").into());
    }
    // Every file is parsed before any is written, so a broken one can't lose another's text.
    for (path, output) in &changes {
        fs::write(path, output)?;
    }
    if write {
        #[expect(clippy::print_stdout, reason = "a CLI command's summary")]
        {
            println!("Formatted {} Fluent catalogs.", changes.len());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_preserves_grammar_comments_and_literal_whitespace() {
        let source = "# Translator context\ncount = { $n ->\n [one] One\n *[other] { $n } files\n    }\n\nline = { \"\\u000A  \" }{ $path }\n";
        let output = formatted(source).expect("valid Fluent");
        assert_eq!(fluent_syntax::parser::parse(source), fluent_syntax::parser::parse(output.as_str()));
        assert_eq!(formatted(&output).expect("formatted Fluent"), output);
        assert_eq!(formatted(&source.replace('\n', "\r\n")).expect("Windows newlines"), output);
        assert!(formatted("broken = { ").is_err());
    }

    #[test]
    fn finds_rust_calls_inside_other_macros_and_checks_their_arguments() {
        let mut calls = Calls::default();
        let mut rust = Rust { calls: &mut calls, file: "a.rs".into() };
        rust.visit_file(&syn::parse_file(r#"fn f() { vec![tr!("name"), tr!("workers", count = 2)]; }"#).expect("Rust"));
        rust.visit_file(&syn::parse_file(r#"fn f() { tr!("workers", other = 2); }"#).expect("Rust"));
        rust.visit_file(&syn::parse_file(r#"fn f() { tr!("wrapped", inner = tr!("inner")); }"#).expect("Rust"));
        assert_eq!(calls.messages.len(), 4);
        assert_eq!(calls.errors.len(), 1);
        assert!(calls.errors[0].contains("\"workers\""), "{}", calls.errors[0]);
    }

    #[test]
    fn finds_portal_calls_with_their_arguments() {
        let mut calls = Calls::default();
        scan_web(
            "<p :title=\"t('tip', { n: items.length, name })\">{{ t(\"hello\") }}</p>\n\
             const x = t(`plain`, { who: `a ${b}`, what: f(1, { x: t('inner') }), }); set(1); a.t('no'); format('no')",
            "x.vue",
            &mut calls,
        );
        assert!(calls.errors.is_empty(), "{:?}", calls.errors);
        let names = |id: &str| calls.messages[id].0.iter().cloned().collect::<Vec<_>>();
        assert_eq!(names("tip"), ["n", "name"]);
        assert_eq!(names("hello"), Vec::<String>::new());
        assert_eq!(names("plain"), ["what", "who"]);
        assert_eq!(names("inner"), Vec::<String>::new());
        assert_eq!(calls.messages.len(), 4);
        let mut calls = Calls::default();
        scan_web("t(id); t(`a-${k}`); t('x', args); t('y', { ...rest })", "x.ts", &mut calls);
        assert_eq!(calls.errors.len(), 4, "{:?}", calls.errors);
    }
}
