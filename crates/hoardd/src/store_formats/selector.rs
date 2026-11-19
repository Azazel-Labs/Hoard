//! Selectors: where a fact is in a file, as a path into JSON or a regular expression over text.

use super::*;

/// Selectors tried in order.
#[derive(Debug, Default)]
pub(crate) struct Sels(pub(super) Vec<Sel>);

impl Sels {
    /// The first value any of them finds.
    pub(crate) fn first(&self, at: Node<'_>) -> Option<String> {
        self.0.iter().find_map(|s| s.strings(at).into_iter().next())
    }

    /// Every value they find, each once.
    pub(crate) fn all(&self, at: Node<'_>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for v in self.0.iter().flat_map(|s| s.strings(at)) {
            if !out.contains(&v) {
                out.push(v);
            }
        }
        out
    }

    /// Whether any of them finds a value that says yes.
    pub(crate) fn holds(&self, at: Node<'_>) -> bool {
        self.0.iter().any(|s| s.holds(at))
    }
}

/// One selector: a path into JSON, or a regular expression over text.
#[derive(Debug)]
pub(crate) enum Sel {
    Path { steps: Vec<Step>, test: Option<Test> },
    Pattern(Regex),
}

/// ` ~ glob` keeps the values that match; ` !~ glob` the ones that don't.
#[derive(Debug)]
pub(crate) struct Test {
    not: bool,
    glob: String,
}

#[derive(Debug)]
pub(crate) struct Step {
    key: Option<Key>,
    filters: Vec<Filter>,
}

#[derive(Debug)]
enum Key {
    Exact(String),
    /// Keys matching a lowercase glob, but not the ones matching `except`.
    Glob {
        glob: String,
        except: Vec<String>,
    },
}

#[derive(Debug)]
enum Filter {
    /// `[]`: an array's items.
    Each,
    /// `[key=a|b]`: an array's items whose `key` is one of these (ignoring case).
    Where { key: String, any_of: Vec<String> },
    /// `[array]`, `[string]`, `[number]`, `[bool]`, `[object]`.
    Type(fn(&Value) -> bool),
}

impl Sel {
    pub(super) fn parse(s: &str, syntax: Syntax) -> Result<Self> {
        let s = s.trim();
        if s.is_empty() {
            bail!(tr!("error-format-empty-selector"));
        }
        if syntax == Syntax::Text {
            return Ok(Self::Pattern(Regex::new(&format!("(?m){s}"))?));
        }
        let (path, test) = if let Some((p, g)) = s.split_once(" !~ ") {
            (p, Some(Test { not: true, glob: g.trim().to_lowercase() }))
        } else if let Some((p, g)) = s.split_once(" ~ ") {
            (p, Some(Test { not: false, glob: g.trim().to_lowercase() }))
        } else {
            (s, None)
        };
        let steps = split_steps(path.trim())?.into_iter().map(parse_step).collect::<Result<Vec<_>>>()?;
        Ok(Self::Path { steps, test })
    }

    /// The places it points at.
    pub(crate) fn nodes<'a>(&self, at: Node<'a>) -> Vec<Node<'a>> {
        match (self, at) {
            (Self::Path { steps, .. }, Node::Json(v)) => {
                let mut cur = vec![v];
                for s in steps {
                    cur = s.apply(cur);
                }
                cur.into_iter().map(Node::Json).collect()
            }
            (Self::Pattern(re), Node::Text(t)) => re
                .captures_iter(t)
                .filter_map(|c| c.get(1).or_else(|| c.get(0)))
                .map(|m| Node::Text(m.as_str()))
                .collect(),
            _ => Vec::new(),
        }
    }

    /// The values it finds, as text: arrays are flattened, objects left out, blanks dropped.
    pub(super) fn strings(&self, at: Node<'_>) -> Vec<String> {
        fn flat(v: &Value, out: &mut Vec<String>) {
            match v {
                Value::String(s) => out.push(s.trim().to_owned()),
                Value::Number(n) => out.push(n.to_string()),
                Value::Bool(b) => out.push(b.to_string()),
                Value::Array(a) => a.iter().for_each(|x| flat(x, out)),
                _ => {}
            }
        }
        let mut out = Vec::new();
        for n in self.nodes(at) {
            match n {
                Node::Json(v) => flat(v, &mut out),
                Node::Text(t) => out.push(t.trim().to_owned()),
            }
        }
        out.retain(|s| !s.is_empty());
        if let Self::Path { test: Some(t), .. } = self {
            out.retain(|s| glob(&t.glob, &s.to_lowercase()) != t.not);
        }
        out
    }

    /// Whether it finds anything at all.
    pub(super) fn finds(&self, at: Node<'_>) -> bool {
        match self {
            Self::Path { test: Some(_), .. } => !self.strings(at).is_empty(),
            _ => !self.nodes(at).is_empty(),
        }
    }

    /// Whether it finds a value that says yes: `true`, a number but 0, words but "false", "no" or "0", or (with a
    /// test) any value that passes it.
    pub(super) fn holds(&self, at: Node<'_>) -> bool {
        if let Self::Path { test: Some(_), .. } = self {
            return !self.strings(at).is_empty();
        }
        self.nodes(at).into_iter().any(|n| match n {
            Node::Json(Value::Bool(b)) => *b,
            Node::Json(Value::Number(n)) => n.as_f64().is_some_and(|f| f != 0.0),
            Node::Json(Value::String(s)) => {
                let s = s.trim().to_ascii_lowercase();
                !s.is_empty() && !matches!(s.as_str(), "false" | "no" | "0")
            }
            Node::Json(Value::Array(a)) => !a.is_empty(),
            Node::Json(Value::Object(_)) | Node::Text(_) => true,
            Node::Json(Value::Null) => false,
        })
    }
}

impl Step {
    pub(super) fn apply<'a>(&self, cur: Vec<&'a Value>) -> Vec<&'a Value> {
        let mut out = match &self.key {
            None => cur,
            Some(k) => {
                let mut out = Vec::new();
                for v in cur {
                    k.find(v, &mut out);
                }
                out
            }
        };
        for f in &self.filters {
            out = f.apply(out);
        }
        out
    }
}

impl Key {
    /// The values under this key; in an array, under it in each item.
    pub(super) fn find<'a>(&self, v: &'a Value, out: &mut Vec<&'a Value>) {
        match v {
            Value::Object(o) => match self {
                Self::Exact(name) => {
                    let found =
                        o.get(name).or_else(|| o.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|e| e.1));
                    out.extend(found.filter(|x| !x.is_null()));
                }
                Self::Glob { glob: g, except } => {
                    for (k, x) in o {
                        let k = k.to_lowercase();
                        if glob(g, &k) && !except.iter().any(|e| glob(e, &k)) && !x.is_null() {
                            out.push(x);
                        }
                    }
                }
            },
            Value::Array(a) => a.iter().for_each(|x| self.find(x, out)),
            _ => {}
        }
    }
}

impl Filter {
    pub(super) fn apply<'a>(&self, cur: Vec<&'a Value>) -> Vec<&'a Value> {
        let items = || {
            cur.iter().flat_map(|v| match v {
                Value::Array(a) => a.iter().filter(|x| !x.is_null()).collect::<Vec<_>>(),
                v => vec![*v],
            })
        };
        match self {
            Self::Each => items().collect(),
            Self::Where { key, any_of } => items()
                .filter(|v| {
                    let found = v.as_object().and_then(|o| {
                        o.get(key).or_else(|| o.iter().find(|(k, _)| k.eq_ignore_ascii_case(key)).map(|e| e.1))
                    });
                    let text = match found {
                        Some(Value::String(s)) => s.trim().to_lowercase(),
                        Some(Value::Number(n)) => n.to_string(),
                        Some(Value::Bool(b)) => b.to_string(),
                        _ => return false,
                    };
                    any_of.contains(&text)
                })
                .collect(),
            Self::Type(is) => cur.into_iter().filter(|v| is(v)).collect(),
        }
    }
}

/// A path's steps: split on dots outside brackets and parentheses.
fn split_steps(path: &str) -> Result<Vec<&str>> {
    let mut out = Vec::new();
    let (mut depth, mut start) = (0i32, 0);
    for (i, c) in path.char_indices() {
        match c {
            '[' | '(' => depth += 1,
            ']' | ')' => depth -= 1,
            '.' if depth == 0 => {
                out.push(&path[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        if depth < 0 {
            bail!(tr!("error-format-unopened-bracket"));
        }
    }
    if depth != 0 {
        bail!(tr!("error-format-unclosed-bracket"));
    }
    out.push(&path[start..]);
    if out.iter().any(|s| s.trim().is_empty()) {
        bail!(tr!("error-format-empty-step-dots"));
    }
    Ok(out)
}

/// `key`, `glob*!(except,*others*)`, then filters: `[]`, `[key=a|b]`, `[array]`.
fn parse_step(part: &str) -> Result<Step> {
    let part = part.trim();
    let key_end = part.find(['[', '!']).unwrap_or(part.len());
    let key = part[..key_end].trim();
    let mut rest = &part[key_end..];
    let mut except = Vec::new();
    if let Some(r) = rest.strip_prefix("!(") {
        let close = r.find(')').with_context(|| tr!("error-format-unclosed-except"))?;
        except = r[..close].split(',').map(|e| e.trim().to_lowercase()).filter(|e| !e.is_empty()).collect();
        rest = &r[close + 1..];
    }
    let mut filters = Vec::new();
    while let Some(r) = rest.strip_prefix('[') {
        let close = r.find(']').with_context(|| tr!("error-format-unclosed-filter"))?;
        filters.push(parse_filter(r[..close].trim())?);
        rest = &r[close + 1..];
    }
    if !rest.is_empty() {
        bail!(tr!("error-format-after-filters", rest = format!("{rest:?}"), step = format!("{part:?}")));
    }
    let key = if key.is_empty() {
        if !except.is_empty() {
            bail!(tr!("error-format-except-needs-glob"));
        }
        None
    } else if key.contains(['*', '?']) {
        Some(Key::Glob { glob: key.to_lowercase(), except })
    } else {
        if !except.is_empty() {
            bail!(tr!("error-format-except-not-glob", key = format!("{key:?}")));
        }
        Some(Key::Exact(key.to_owned()))
    };
    if key.is_none() && filters.is_empty() {
        bail!(tr!("error-format-empty-step"));
    }
    Ok(Step { key, filters })
}

fn parse_filter(f: &str) -> Result<Filter> {
    Ok(match f {
        "" => Filter::Each,
        "array" => Filter::Type(Value::is_array),
        "string" => Filter::Type(Value::is_string),
        "number" => Filter::Type(Value::is_number),
        "bool" => Filter::Type(Value::is_boolean),
        "object" => Filter::Type(Value::is_object),
        f => {
            let (key, values) = f.split_once('=').with_context(|| tr!("error-format-not-a-filter", filter = f))?;
            let any_of: Vec<String> =
                values.split('|').map(|v| v.trim().to_lowercase()).filter(|v| !v.is_empty()).collect();
            if key.trim().is_empty() || any_of.is_empty() {
                bail!(tr!("error-format-filter-needs-value", filter = f));
            }
            Filter::Where { key: key.trim().to_owned(), any_of }
        }
    })
}

/// Does `text` match `pattern`, where `*` is any run of characters and `?` any one? Both are lowercase.
pub(super) fn glob(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some((pi, ti));
            pi += 1;
        } else if let Some((sp, st)) = star {
            pi = sp + 1;
            ti = st + 1;
            star = Some((sp, st + 1));
        } else {
            return false;
        }
    }
    p[pi..].iter().all(|&c| c == '*')
}
