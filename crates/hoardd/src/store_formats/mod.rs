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
use serde::Deserialize;
use serde_json::Value;

use crate::i18n::tr;

mod selector;

use selector::{Sel, Sels, glob};

/// The formats Hoard ships with, by file name (a test checks every file in `formats/` is here).
const BUILT_IN: &[(&str, &str)] = &[
    ("kenney.toml", include_str!("../../formats/kenney.toml")),
    ("megascans.toml", include_str!("../../formats/megascans.toml")),
    ("sketchfab.toml", include_str!("../../formats/sketchfab.toml")),
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
                f.file = p.file_name().map_or_default(|n| n.to_string_lossy().into_owned());
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
    /// The file speaks for its whole pack (a kit of many models), not one asset.
    pub(crate) whole_pack: bool,
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
            whole_pack: spec.describes == Describes::Pack,
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
    describes: Describes,
    #[serde(default)]
    fields: FieldSpec,
}

/// What a file speaks for: the asset beside it (whose model takes its title), or the whole pack.
#[derive(Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Describes {
    #[default]
    Asset,
    Pack,
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
mod tests;
