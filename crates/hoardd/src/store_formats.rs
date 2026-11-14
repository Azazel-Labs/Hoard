//! The files stores ship beside their art ([`crate::sidecar`]) are described in TOML, not code. Reading another
//! store's file takes a new file in `crates/hoardd/formats/`: which files it reads, how to tell they're the store's,
//! and where each fact is in them (see `docs/store-formats.md`). The built-in formats are compiled in. More can go in
//! `<data dir>/formats/`, which is read at startup and again whenever Settings → Store files changes it.
//!
//! A JSON format points at values with paths (`semanticTags.name`, `meta[key=scanArea].value`). A text format
//! points at them with regular expressions, whose first group is the value.

use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, OnceLock, PoisonError, RwLock};

use anyhow::{Context as _, Result, bail};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

use crate::i18n::tr;

/// The formats Hoard ships with, by file name (a test checks every file in `formats/` is here).
const BUILT_IN: &[(&str, &str)] = &[
    ("megascans.toml", include_str!("../formats/megascans.toml")),
    ("sketchfab.toml", include_str!("../formats/sketchfab.toml")),
];

static FORMATS: LazyLock<RwLock<Arc<Vec<Format>>>> = LazyLock::new(|| RwLock::new(Arc::new(built_in())));
/// `<data dir>/formats`, once the server has started.
static DIR: OnceLock<PathBuf> = OnceLock::new();

/// Every format, the data folder's first.
pub(crate) fn all() -> Arc<Vec<Format>> {
    Arc::clone(&FORMATS.read().unwrap_or_else(PoisonError::into_inner))
}

/// The folder of the data folder's formats, once the server has started.
pub(crate) fn dir() -> Option<&'static Path> {
    DIR.get().map(PathBuf::as_path)
}

/// A format's name as a file name: lowercase, words joined by dashes ("Quixel Bridge" → `quixel-bridge`). The
/// samples of a built-in format are in `formats/samples/<slug>/`.
pub(crate) fn slug(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_owned()
}

fn built_in() -> Vec<Format> {
    BUILT_IN
        .iter()
        .filter_map(|(file, text)| match Format::parse(text) {
            Ok(mut f) => {
                (*file).clone_into(&mut f.file);
                Some(f)
            }
            Err(e) => {
                tracing::error!(file, error = %format!("{e:#}"), "a built-in store file format doesn't read");
                None
            }
        })
        .collect()
}

/// Read the formats in `dir` (`<data dir>/formats`) as well as the built-in ones. One named like a built-in one
/// takes its place. Called at startup; a file that doesn't read is reported and left out.
pub(crate) fn load(dir: &Path) {
    DIR.get_or_init(|| dir.to_owned());
    reload();
}

/// Read the data folder's formats again, after one was added, changed or removed.
pub(crate) fn reload() {
    let mut formats = built_in();
    let Some(dir) = dir() else {
        *FORMATS.write().unwrap_or_else(PoisonError::into_inner) = Arc::new(formats);
        return;
    };
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("toml")))
        .collect();
    paths.sort();
    for p in paths.iter().rev() {
        match std::fs::read_to_string(p).map_err(anyhow::Error::from).and_then(|t| Format::parse(&t)) {
            Ok(mut f) => {
                tracing::info!(file = %p.display(), format = %f.name, "store file format read");
                f.file = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                f.custom = true;
                formats.retain(|b| !b.name.eq_ignore_ascii_case(&f.name));
                formats.insert(0, f);
            }
            Err(e) => tracing::warn!(file = %p.display(), error = %format!("{e:#}"), "store file format not read"),
        }
    }
    *FORMATS.write().unwrap_or_else(PoisonError::into_inner) = Arc::new(formats);
}

/// A store's file, as its TOML describes it.
#[derive(Debug)]
pub(crate) struct Format {
    /// The store or library ("Megascans").
    pub(crate) name: String,
    /// The file in words ("Megascans file"), for the store search's reasons.
    pub(crate) called: String,
    /// Who sells what it describes, when the file doesn't say.
    pub(crate) seller: Option<String>,
    /// Tags everything it describes gets ("megascans").
    pub(crate) tags: Vec<String>,
    /// How the description says where it's from ("scanned in").
    pub(crate) origin_words: String,
    /// The file's name is the asset's id (`xeokah0.json`) when the file doesn't give one.
    pub(crate) id_from_file_name: bool,
    pub(crate) fields: Fields,
    /// Its file's name (`megascans.toml`).
    pub(crate) file: String,
    /// It's in the data folder, not built in.
    pub(crate) custom: bool,
    /// The TOML it was read from.
    pub(crate) text: String,
    /// Lowercase globs of the file names it reads.
    files: Vec<String>,
    syntax: Syntax,
    /// Ways to tell the file is the store's: in one of these, every selector finds something.
    require: Vec<Vec<Sel>>,
}

/// Where each fact is in the file. Every one is optional.
#[derive(Debug, Default)]
pub(crate) struct Fields {
    pub(crate) title: Sels,
    pub(crate) id: Sels,
    pub(crate) kind: Sels,
    pub(crate) words: Sels,
    pub(crate) colors: Sels,
    pub(crate) origin: Sels,
    pub(crate) texel_density: Sels,
    pub(crate) size_min: Sels,
    pub(crate) size_max: Sels,
    pub(crate) size: Sels,
    pub(crate) tiles: Sels,
    pub(crate) physical_size: Sels,
    pub(crate) displacement: Sels,
    pub(crate) seller: Sels,
    pub(crate) license: Sels,
    pub(crate) url: Sels,
    pub(crate) dims: Vec<Dims>,
    pub(crate) maps: Vec<Maps>,
    pub(crate) lods: Vec<Lods>,
}

/// Measurements: for each value `each` finds, a name and a value inside it.
#[derive(Debug)]
pub(crate) struct Dims {
    pub(crate) each: Sel,
    pub(crate) name: Sels,
    pub(crate) value: Sels,
}

/// Texture maps: for each value `each` finds, what the map is for and its files.
#[derive(Debug)]
pub(crate) struct Maps {
    pub(crate) each: Sel,
    pub(crate) kind: Sels,
    pub(crate) file: Sels,
}

/// Levels of detail: for each value `each` finds, its triangle count and its mesh files.
#[derive(Debug)]
pub(crate) struct Lods {
    pub(crate) each: Sel,
    pub(crate) tris: Sels,
    pub(crate) files: Sels,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Syntax {
    #[default]
    Json,
    Text,
}

/// A file, parsed.
#[derive(Debug)]
pub(crate) enum Doc {
    Json(Value),
    Text(String),
}

impl Doc {
    pub(crate) fn root(&self) -> Node<'_> {
        match self {
            Self::Json(v) => Node::Json(v),
            Self::Text(t) => Node::Text(t),
        }
    }
}

/// A place in a file: a JSON value, or a piece of a text file.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Node<'a> {
    Json(&'a Value),
    Text(&'a str),
}

impl Format {
    /// Whether it reads a file of this name.
    pub(crate) fn wants(&self, path: &str) -> bool {
        let name = path.rsplit(['/', '\\']).next().unwrap_or(path).to_lowercase();
        self.files.iter().any(|g| glob(g, &name))
    }

    /// The file parsed, if it's this store's.
    pub(crate) fn open(&self, text: &str) -> Option<Doc> {
        let doc = match self.syntax {
            Syntax::Json => Doc::Json(serde_json::from_str(text).ok()?),
            Syntax::Text => Doc::Text(text.replace("\r\n", "\n")),
        };
        let root = doc.root();
        (self.require.is_empty() || self.require.iter().any(|all| all.iter().all(|s| s.finds(root)))).then_some(doc)
    }

    /// The file names it reads, as SQL `LIKE` patterns for lowercase paths (escaped with `\`).
    pub(crate) fn like_patterns(&self) -> Vec<String> {
        let mut out = Vec::new();
        for g in &self.files {
            let mut p = String::new();
            for c in g.chars() {
                match c {
                    '*' => p.push('%'),
                    '?' => p.push('_'),
                    '%' | '_' | '\\' => {
                        p.push('\\');
                        p.push(c);
                    }
                    c => p.push(c),
                }
            }
            if !p.starts_with('%') {
                out.push(format!("%/{p}"));
            }
            out.push(p);
        }
        out
    }

    pub(crate) fn parse(text: &str) -> Result<Self> {
        let spec: Spec = toml::from_str(text)?;
        let name = spec.name.trim().to_owned();
        if name.is_empty() {
            bail!(tr!("error-format-no-name"));
        }
        if spec.files.is_empty() {
            bail!(tr!("error-format-no-files"));
        }
        let syntax = spec.syntax;
        let sel = |s: &str, at: &str| Sel::parse(s, syntax).with_context(|| format!("{at}: {s:?}"));
        let sels = |v: &OneOrMany, at: &str| -> Result<Sels> {
            Ok(Sels(v.list().into_iter().map(|s| sel(s, at)).collect::<Result<_>>()?))
        };
        let f = &spec.fields;
        let fields = Fields {
            title: sels(&f.title, "fields.title")?,
            id: sels(&f.id, "fields.id")?,
            kind: sels(&f.kind, "fields.type")?,
            words: sels(&f.words, "fields.words")?,
            colors: sels(&f.colors, "fields.colors")?,
            origin: sels(&f.origin, "fields.origin")?,
            texel_density: sels(&f.texel_density, "fields.texel_density")?,
            size_min: sels(&f.size_min, "fields.size_min")?,
            size_max: sels(&f.size_max, "fields.size_max")?,
            size: sels(&f.size, "fields.size")?,
            tiles: sels(&f.tiles, "fields.tiles")?,
            physical_size: sels(&f.physical_size, "fields.physical_size")?,
            displacement: sels(&f.displacement, "fields.displacement")?,
            seller: sels(&f.seller, "fields.seller")?,
            license: sels(&f.license, "fields.license")?,
            url: sels(&f.url, "fields.url")?,
            dims: f
                .dims
                .iter()
                .map(|d| {
                    Ok(Dims {
                        each: sel(&d.each, "fields.dims.each")?,
                        name: sels(&d.name, "fields.dims.name")?,
                        value: sels(&d.value, "fields.dims.value")?,
                    })
                })
                .collect::<Result<_>>()?,
            maps: f
                .maps
                .iter()
                .map(|m| {
                    Ok(Maps {
                        each: sel(&m.each, "fields.maps.each")?,
                        kind: sels(&m.kind, "fields.maps.kind")?,
                        file: sels(&m.file, "fields.maps.file")?,
                    })
                })
                .collect::<Result<_>>()?,
            lods: f
                .lods
                .iter()
                .map(|l| {
                    Ok(Lods {
                        each: sel(&l.each, "fields.lods.each")?,
                        tris: sels(&l.tris, "fields.lods.tris")?,
                        files: sels(&l.files, "fields.lods.files")?,
                    })
                })
                .collect::<Result<_>>()?,
        };
        Ok(Self {
            called: spec.called.unwrap_or_else(|| format!("{name} file")),
            name,
            seller: spec.seller.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()),
            tags: spec.tags,
            origin_words: spec.origin_words.unwrap_or_else(|| "from".to_owned()),
            id_from_file_name: spec.id_from_file_name,
            fields,
            file: String::new(),
            custom: false,
            text: text.to_owned(),
            files: spec.files.iter().map(|g| g.trim().to_lowercase()).collect(),
            syntax,
            require: spec.require.iter().map(|r| sels(r, "require").map(|s| s.0)).collect::<Result<_>>()?,
        })
    }
}

/// Selectors tried in order.
#[derive(Debug, Default)]
pub(crate) struct Sels(Vec<Sel>);

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
    fn parse(s: &str, syntax: Syntax) -> Result<Self> {
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
    fn strings(&self, at: Node<'_>) -> Vec<String> {
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
    fn finds(&self, at: Node<'_>) -> bool {
        match self {
            Self::Path { test: Some(_), .. } => !self.strings(at).is_empty(),
            _ => !self.nodes(at).is_empty(),
        }
    }

    /// Whether it finds a value that says yes: `true`, a number but 0, words but "false", "no" or "0", or (with a
    /// test) any value that passes it.
    fn holds(&self, at: Node<'_>) -> bool {
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
    fn apply<'a>(&self, cur: Vec<&'a Value>) -> Vec<&'a Value> {
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
    fn find<'a>(&self, v: &'a Value, out: &mut Vec<&'a Value>) {
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
    fn apply<'a>(&self, cur: Vec<&'a Value>) -> Vec<&'a Value> {
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
fn glob(pattern: &str, text: &str) -> bool {
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

/// A format file as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Spec {
    name: String,
    called: Option<String>,
    seller: Option<String>,
    files: Vec<String>,
    #[serde(default)]
    syntax: Syntax,
    #[serde(default)]
    require: Vec<OneOrMany>,
    #[serde(default)]
    tags: Vec<String>,
    origin_words: Option<String>,
    #[serde(default)]
    id_from_file_name: bool,
    #[serde(default)]
    fields: FieldSpec,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct FieldSpec {
    title: OneOrMany,
    id: OneOrMany,
    #[serde(rename = "type")]
    kind: OneOrMany,
    words: OneOrMany,
    colors: OneOrMany,
    origin: OneOrMany,
    texel_density: OneOrMany,
    size_min: OneOrMany,
    size_max: OneOrMany,
    size: OneOrMany,
    tiles: OneOrMany,
    physical_size: OneOrMany,
    displacement: OneOrMany,
    seller: OneOrMany,
    license: OneOrMany,
    url: OneOrMany,
    dims: Vec<DimsSpec>,
    maps: Vec<MapsSpec>,
    lods: Vec<LodsSpec>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DimsSpec {
    each: String,
    name: OneOrMany,
    value: OneOrMany,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MapsSpec {
    each: String,
    kind: OneOrMany,
    file: OneOrMany,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LodsSpec {
    each: String,
    tris: OneOrMany,
    files: OneOrMany,
}

/// A selector, or a list of them.
#[derive(Deserialize)]
#[serde(untagged)]
enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

impl Default for OneOrMany {
    fn default() -> Self {
        Self::Many(Vec::new())
    }
}

impl OneOrMany {
    fn list(&self) -> Vec<&str> {
        match self {
            Self::One(s) => vec![s.as_str()],
            Self::Many(v) => v.iter().map(String::as_str).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json(sel: &str, doc: &str) -> Vec<String> {
        let v: Value = serde_json::from_str(doc).unwrap();
        Sel::parse(sel, Syntax::Json).unwrap().strings(Node::Json(&v))
    }

    #[test]
    fn paths_find_values() {
        let doc = r##"{"a": {"name": "N", "minSize": "1 m", "Tags": ["x", "y"], "flag": true},
                      "meta": [{"key": "scanArea", "value": "2x2 m"}, {"key": "Height", "value": "2cm"}],
                      "list": [{"uri": "a.jpg"}, {"uri": "b.jpg"}], "color": "#fff", "cats": ["surface_x", "soil"]}"##;
        assert_eq!(json("a.name", doc), ["N"]);
        assert_eq!(json("a.tags", doc), ["x", "y"], "keys ignore case when no key matches exactly");
        assert_eq!(json("a.*size*", doc), ["1 m"]);
        assert_eq!(json("a.*!(name,*size*)[array]", doc), ["x", "y"]);
        assert_eq!(json("meta[key=scanarea].value", doc), ["2x2 m"]);
        assert_eq!(json("meta[key=width|height].value", doc), ["2cm"]);
        assert_eq!(json("list.uri", doc), ["a.jpg", "b.jpg"], "a key steps into each item of an array");
        assert_eq!(json("list[].uri", doc), ["a.jpg", "b.jpg"]);
        assert_eq!(json("color !~ #*", doc), Vec::<String>::new());
        assert_eq!(json("cats ~ surface*", doc), ["surface_x"]);
        assert_eq!(json("nothing.here", doc), Vec::<String>::new());

        let v: Value = serde_json::from_str(doc).unwrap();
        let holds = |s: &str| Sel::parse(s, Syntax::Json).unwrap().holds(Node::Json(&v));
        assert!(holds("a.flag") && holds("cats ~ surface*") && !holds("cats ~ rock*") && !holds("a.missing"));
    }

    #[test]
    fn bad_selectors_say_why() {
        for bad in ["a..b", "a[key=]", "a[wat]", "a[", "name!(x)", "a.b)"] {
            assert!(Sel::parse(bad, Syntax::Json).is_err(), "{bad}");
        }
        assert!(Sel::parse("(unclosed", Syntax::Text).is_err());
        let e = Format::parse("name = \"X\"\nfiles = [\"*.json\"]\n[fields]\ntitel = \"name\"").unwrap_err();
        assert!(format!("{e:#}").contains("titel"), "{e:#}");
    }

    #[test]
    fn text_patterns_take_their_first_group() {
        let doc = Doc::Text("* title:\tRound thing\n* author:\tSomeone (https://example.com/someone)\n".to_owned());
        let p = |s: &str| Sel::parse(s, Syntax::Text).unwrap().strings(doc.root());
        assert_eq!(p(r"^\* title:[ \t]*(.+?)[ \t]*$"), ["Round thing"]);
        assert_eq!(p(r"^\* author:[ \t]*(.+?)(?:[ \t]+\(https?://[^)]*\))?[ \t]*$"), ["Someone"]);
        assert_eq!(p(r"^\* source:[ \t]*(\S+)"), Vec::<String>::new());
    }

    #[test]
    fn globs() {
        assert!(glob("*.json", "a.json") && glob("license.txt", "license.txt") && glob("*min*size*", "minsize"));
        assert!(!glob("*.json", "a.json5") && !glob("a?c", "ac") && glob("a*b*c", "aXbYbc"));
        let f = Format::parse("name = \"X\"\nfiles = [\"*.json\", \"license.txt\"]").unwrap();
        assert!(f.wants("A/B.JSON") && f.wants("License.txt") && !f.wants("a/licenses.txt"));
        assert_eq!(f.like_patterns(), ["%.json", "%/license.txt", "license.txt"]);
    }

    /// Every file in `formats/` is compiled in and reads.
    #[test]
    fn every_format_is_built_in() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("formats");
        let mut files: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| Path::new(n).extension().is_some_and(|e| e == "toml"))
            .collect();
        files.sort();
        let built: Vec<&str> = BUILT_IN.iter().map(|(f, _)| *f).collect();
        assert_eq!(files, built, "add new formats to BUILT_IN in store_formats.rs, in name order");
        for (file, text) in BUILT_IN {
            if let Err(e) = Format::parse(text) {
                panic!("formats/{file}: {e:#}");
            }
        }
    }

    #[test]
    fn slugs() {
        assert_eq!(slug("Quixel Bridge"), "quixel-bridge");
        assert_eq!(slug("  Itch.io (assets)!"), "itch-io-assets");
        assert_eq!(slug("Megascans"), "megascans");
        assert_eq!(slug("ÆØ"), "");
    }

    /// Each sample in `formats/samples/<slug of the format's name>/` reads as that format, and gives what its `.expected.json` says
    /// (only the keys it lists are compared).
    #[test]
    fn samples_read_as_expected() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("formats/samples");
        let mut n = 0;
        for format_dir in std::fs::read_dir(&dir).unwrap().flatten() {
            let format = format_dir.file_name().to_string_lossy().into_owned();
            for file in std::fs::read_dir(format_dir.path()).unwrap().flatten() {
                let name = file.file_name().to_string_lossy().into_owned();
                if name.ends_with(".expected.json") {
                    continue;
                }
                let text = std::fs::read_to_string(file.path()).unwrap();
                let expected_path = file.path().with_file_name(format!("{name}.expected.json"));
                let expected: Value = serde_json::from_str(
                    &std::fs::read_to_string(&expected_path)
                        .unwrap_or_else(|_| panic!("{} has no {name}.expected.json beside it", file.path().display())),
                )
                .unwrap();
                let got = crate::sidecar::read(&name, &text)
                    .unwrap_or_else(|| panic!("samples/{format}/{name} isn't read as any format"));
                let got = serde_json::to_value(&got).unwrap();
                let pretty = serde_json::to_string_pretty(&got).unwrap();
                assert!(got["format"].as_str().is_some_and(|f| slug(f) == format), "{pretty}");
                for (k, v) in expected.as_object().unwrap() {
                    assert_eq!(&got[k], v, "samples/{format}/{name}: {k}\nread as:\n{pretty}");
                }
                n += 1;
            }
        }
        assert!(n >= 2);
    }
}
