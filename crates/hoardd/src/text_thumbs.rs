//! Thumbnails of text: code, data and prose drawn as a silhouette of their lines. Each line is a bar from the left
//! edge, indented as the line is and as long as it is, one cell per character, colored by what the character is part
//! of (a keyword, a string, a number, a comment, punctuation, a key or a tag) with the spaces between left dark. A file
//! reads like a speckled gene sequence: the shape shows its indentation and density, the colors what kind of text it is.

use std::io::Read as _;
use std::path::Path;

use image::{Rgba, RgbaImage};

use crate::analyze::thumb_size;
use crate::{AppState, db};

/// How much of a file is read: more than the lines a thumbnail has room for.
const HEAD: u64 = 64 << 10;
/// The empty border around the lines, in pixels.
const PAD: u32 = 8;
/// Spaces a tab stands for.
const TAB: usize = 4;

/// What a character is part of, which picks its color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ink {
    Space,
    Text,
    Keyword,
    Str,
    Number,
    Comment,
    Punct,
    /// A key, a heading, a tag, a type or a function's name.
    Name,
    /// A markup attribute.
    Attr,
}

impl Ink {
    fn color(self) -> Rgba<u8> {
        Rgba(match self {
            Ink::Space => GROUND,
            Ink::Text => [0xb8, 0xbe, 0xc9, 0xff],
            Ink::Keyword => [0xc6, 0x78, 0xdd, 0xff],
            Ink::Str => [0x98, 0xc3, 0x79, 0xff],
            Ink::Number => [0xd1, 0x9a, 0x66, 0xff],
            Ink::Comment => [0x5c, 0x63, 0x70, 0xff],
            Ink::Punct => [0x7a, 0x82, 0x90, 0xff],
            Ink::Name => [0x61, 0xaf, 0xef, 0xff],
            Ink::Attr => [0xe5, 0xc0, 0x7b, 0xff],
        })
    }
}

const GROUND: [u8; 4] = [0x1d, 0x1f, 0x24, 0xff];

/// How a file's text is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lang {
    /// C-like code: `//` and `/* */` comments.
    Code,
    /// Code with `#` comments (Python, Godot scripts, shell).
    HashCode,
    /// Lua: `--` comments.
    Lua,
    Json,
    /// XML and HTML.
    Markup,
    /// INI, TOML and Godot's scenes: `[sections]`, `key = value`, `;` or `#` comments.
    Ini,
    /// YAML, Unity's scenes and prefabs among them.
    Yaml,
    Csv,
    Markdown,
    /// Plain text.
    Prose,
}

/// The extensions drawn, by how they're read.
const EXTS: &[(Lang, &[&str])] = &[
    (
        Lang::Code,
        &[
            "c", "h", "cc", "cpp", "cxx", "hpp", "hh", "cs", "java", "js", "mjs", "ts", "rs", "go", "swift", "kt",
            "glsl", "hlsl", "wgsl", "shader", "vert", "frag", "comp", "gdshader", "usf", "ush", "metal", "cginc",
            "compute", "fx",
        ],
    ),
    (Lang::HashCode, &["py", "gd", "sh", "rb", "ps1"]),
    (Lang::Lua, &["lua"]),
    (Lang::Json, &["json", "ldtk", "shadergraph", "shadersubgraph"]),
    (Lang::Markup, &["xml", "html", "htm", "tmx", "tsx", "plist"]),
    (Lang::Ini, &["ini", "toml", "cfg", "tscn", "godot", "url", "inf", "desktop"]),
    (
        Lang::Yaml,
        &["yaml", "yml", "prefab", "unity", "asset", "controller", "overridecontroller", "physicmaterial", "mask"],
    ),
    (Lang::Csv, &["csv", "tsv"]),
    (Lang::Markdown, &["md", "markdown"]),
    (Lang::Prose, &["txt", "nfo", "log"]),
];

/// How a file with this extension is read, if it's text Hoard draws.
fn lang(ext: &str) -> Option<Lang> {
    EXTS.iter().find(|(_, exts)| exts.contains(&ext)).map(|(lang, _)| *lang)
}

/// How much of a file with an unfamiliar extension is read to tell whether it's text.
pub(crate) const SNIFF: u64 = 8 << 10;

/// Whether the start of a file is clearly text: UTF-8 with no NULs and next to no control characters. A file cut off
/// mid-character still counts.
pub(crate) fn looks_like_text(head: &[u8]) -> bool {
    let head = head.strip_prefix(b"\xef\xbb\xbf").unwrap_or(head);
    if head.is_empty() || head.contains(&0) {
        return false;
    }
    let text = match std::str::from_utf8(head) {
        Ok(t) => t,
        // Only a character cut off at the end of what was read is forgiven.
        Err(e) if e.error_len().is_none() => std::str::from_utf8(&head[..e.valid_up_to()]).unwrap_or_default(),
        Err(_) => return false,
    };
    let (mut chars, mut odd) = (0usize, 0usize);
    for c in text.chars() {
        chars += 1;
        if c.is_control() && !matches!(c, '\n' | '\r' | '\t' | '\u{c}') {
            odd += 1;
        }
    }
    odd * 100 <= chars && text.chars().any(|c| !c.is_whitespace())
}

/// Whether a file with this extension is text drawn as a silhouette.
pub(crate) fn handles(ext: &str) -> bool {
    lang(ext).is_some()
}

/// Whether an asset's thumbnail is the silhouette of its lines rather than a picture of it (which an AI tagging by
/// thumbnails learns nothing from). `kind` as the database has it.
pub(crate) fn is_silhouette(path: &str, kind: &str) -> bool {
    let ext = Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or_default().to_ascii_lowercase();
    matches!(kind, "document" | "data" | "shader" | "other") && handles(&ext)
}

/// The file drawn `size` pixels square, or `None` when it isn't text after all (a binary Unity asset) or is empty.
pub(crate) fn thumbnail(path: &Path, ext: &str, size: u32) -> anyhow::Result<Option<RgbaImage>> {
    let Some(lang) = lang(ext) else { return Ok(None) };
    thumbnail_as(path, lang, size)
}

/// A file with an unfamiliar extension that turned out to be text, drawn as plain prose.
pub(crate) fn thumbnail_prose(path: &Path, size: u32) -> anyhow::Result<Option<RgbaImage>> {
    thumbnail_as(path, Lang::Prose, size)
}

fn thumbnail_as(path: &Path, lang: Lang, size: u32) -> anyhow::Result<Option<RgbaImage>> {
    let mut head = Vec::new();
    std::fs::File::open(path)?.take(HEAD).read_to_end(&mut head)?;
    Ok(draw(&head, lang, size))
}

async fn draw_one(state: &AppState, id: &str, ext: &str, hash: &str, size: i64) -> anyhow::Result<bool> {
    let pdf = ext == "pdf";
    let picture = if pdf {
        if size.unsigned_abs() > crate::analyze::MAX_IMAGE_DECODE_BYTES {
            return Ok(false);
        }
        let data = state.blobs.read_all(hash).await?.to_vec();
        tokio::task::spawn_blocking(move || crate::textures::pdf_thumbnail_of(data, thumb_size())).await??
    } else {
        let head = state.blobs.head(hash, HEAD).await?;
        let lang = lang(ext);
        tokio::task::spawn_blocking(move || lang.and_then(|lang| draw(&head, lang, thumb_size()))).await?
    };
    let Some(img) = picture else { return Ok(false) };
    let blob = state.blobs.put_cache_bytes(crate::analyze::thumb_bytes(&img, !pdf)?).await?;
    let id = id.to_owned();
    db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
            rusqlite::params![blob.hash, blob.size as i64, db::now()],
        )?;
        c.execute(
            "UPDATE assets SET thumb_hash = ?2 WHERE id = ?1 AND thumb_hash IS NULL",
            rusqlite::params![id, blob.hash],
        )?;
        Ok(())
    })
    .await?;
    Ok(true)
}

/// The key in `settings` saying libraries from before these thumbnails have had theirs drawn.
const DRAWN_KEY: &str = "text_thumbs_drawn";

/// Once, at startup: draw the thumbnails of text and PDFs that were added before Hoard drew them. Reads only the head
/// of a text file, so it's light even on a big library.
pub(crate) async fn draw_missing(state: &AppState) -> anyhow::Result<()> {
    use rusqlite::OptionalExtension as _;
    let todo: Option<Vec<(String, String, String, i64)>> = db::call(&state.pool, |c| {
        if c.query_row("SELECT 1 FROM settings WHERE key = ?1", [DRAWN_KEY], |_| Ok(())).optional()?.is_some() {
            return Ok(None);
        }
        let rows = c
            .prepare(
                "SELECT a.id, a.ext, a.blob_hash, b.size FROM assets a JOIN blobs b ON b.hash = a.blob_hash
                 WHERE a.thumb_hash IS NULL AND a.kind IN ('document', 'data', 'shader', 'other')",
            )?
            .query_map([], |r| Ok((r.get(0)?, r.get::<_, String>(1)?, r.get(2)?, r.get(3)?)))?
            .filter(|r| r.as_ref().map_or(true, |(_, ext, _, _)| handles(ext) || ext == "pdf"))
            .collect::<rusqlite::Result<_>>()?;
        Ok(Some(rows))
    })
    .await?;
    let Some(todo) = todo else { return Ok(()) };
    let mut drawn = 0;
    for (id, ext, hash, size) in todo {
        // A file that can't be read now (a watched folder's drive gone) is left without one.
        match draw_one(state, &id, &ext, &hash, size).await {
            Ok(true) => drawn += 1,
            Ok(false) => {}
            Err(e) => tracing::debug!(asset = %id, error = %format!("{e:#}"), "no text thumbnail"),
        }
    }
    db::call(&state.pool, |c| {
        c.execute("INSERT INTO settings (key, value) VALUES (?1, '1') ON CONFLICT DO NOTHING", [DRAWN_KEY])?;
        Ok(())
    })
    .await?;
    if drawn > 0 {
        tracing::info!(drawn, "drew the thumbnails of text and PDFs added before Hoard drew them");
    }
    Ok(())
}

/// Settings key marking that files of no known kind have been looked at for plain text.
const SNIFFED_KEY: &str = "text_sniffed";

/// Once: files added as of no kind Hoard knew (an Internet shortcut, a README with no extension) that are plainly text
/// become documents, read and drawn as text, as they are when added now.
pub(crate) async fn sniff_others(state: &AppState) -> anyhow::Result<()> {
    use rusqlite::OptionalExtension as _;
    let todo: Option<Vec<(String, String)>> = db::call(&state.pool, |c| {
        if c.query_row("SELECT 1 FROM settings WHERE key = ?1", [SNIFFED_KEY], |_| Ok(())).optional()?.is_some() {
            return Ok(None);
        }
        let rows = c
            .prepare("SELECT id, blob_hash FROM assets WHERE kind = 'other'")?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        Ok(Some(rows))
    })
    .await?;
    let Some(todo) = todo else { return Ok(()) };
    let category = crate::tagger::categorize(hoard_core::AssetKind::Document, &crate::tagger::TagSet::default());
    let mut found = 0;
    for (id, hash) in todo {
        // A file that can't be read now (a watched folder's drive gone) stays as it is.
        let Ok(head) = state.blobs.head(&hash, HEAD).await else { continue };
        if !looks_like_text(&head[..head.len().min(SNIFF as usize)]) {
            continue;
        }
        let picture = tokio::task::spawn_blocking(move || draw(&head, Lang::Prose, thumb_size())).await?;
        let thumb = match picture {
            Some(img) => Some(state.blobs.put_cache_bytes(crate::analyze::thumb_bytes(&img, true)?).await?),
            None => None,
        };
        let category = category.clone();
        db::call(&state.pool, move |c| {
            let tx = c.transaction()?;
            if let Some(b) = &thumb {
                tx.execute(
                    "INSERT INTO blobs (hash, size, created_at) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING",
                    rusqlite::params![b.hash, b.size as i64, db::now()],
                )?;
            }
            let n = tx.execute(
                "UPDATE assets SET kind = 'document', meta = json_set(meta, '$.text', json('true')),
                     category = CASE WHEN human_edited THEN category ELSE ?2 END,
                     thumb_hash = COALESCE(thumb_hash, ?3)
                   WHERE id = ?1 AND kind = 'other'",
                rusqlite::params![id, category, thumb.as_ref().map(|b| b.hash.clone())],
            )?;
            if n > 0 {
                tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1 AND tag = 'kind:other'", [&id])?;
                db::upsert_tag(&tx, &id, "kind:document", hoard_core::TagSource::Meta, 1.0)?;
                db::reindex_asset(&tx, &id)?;
            }
            tx.commit()?;
            Ok(())
        })
        .await?;
        found += 1;
    }
    db::call(&state.pool, |c| {
        c.execute("INSERT INTO settings (key, value) VALUES (?1, '1') ON CONFLICT DO NOTHING", [SNIFFED_KEY])?;
        Ok(())
    })
    .await?;
    if found > 0 {
        tracing::info!(found, "files of no known kind turned out to be text");
    }
    Ok(())
}

fn draw(bytes: &[u8], lang: Lang, size: u32) -> Option<RgbaImage> {
    if bytes[..bytes.len().min(8192)].contains(&0) {
        return None;
    }
    let text = String::from_utf8_lossy(bytes);
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    let inner = size.saturating_sub(2 * PAD);
    let mut lines: Vec<&str> = text.lines().collect();
    // A file cut short ends mid-line; that line is left out.
    if bytes.len() as u64 >= HEAD && lines.len() > 1 {
        lines.pop();
    }
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    if lines.is_empty() || inner < 8 {
        return None;
    }
    // Short files get taller lines, up to 6 px; long ones 2 px, as many as fit.
    let pitch = (inner / lines.len() as u32).clamp(2, 6);
    lines.truncate((inner / pitch) as usize);
    let mut lexer = Lexer::new(lang);
    let inks: Vec<Vec<Ink>> = lines.iter().map(|l| lexer.line(&expand_tabs(l))).collect();
    if inks.iter().all(|l| l.iter().all(|&i| i == Ink::Space)) {
        return None;
    }
    // Columns for the longest line, within reason: a long line is cut at the right edge.
    let cols = inks.iter().map(Vec::len).max().unwrap_or(0).clamp(48, 160);
    let cell = inner as f32 / cols as f32;
    let bar = if pitch > 2 { pitch - 1 } else { 1 };
    let mut img = RgbaImage::from_pixel(size, size, Rgba(GROUND));
    for (row, line) in inks.iter().enumerate() {
        let y0 = PAD + row as u32 * pitch;
        for x in 0..inner {
            let col = ((x as f32 + 0.5) / cell) as usize;
            let Some(&ink) = line.get(col) else { break };
            if ink == Ink::Space {
                continue;
            }
            for y in y0..y0 + bar {
                img.put_pixel(PAD + x, y, ink.color());
            }
        }
    }
    Some(img)
}

fn expand_tabs(line: &str) -> Vec<char> {
    let mut out = Vec::with_capacity(line.len());
    for c in line.chars() {
        if c == '\t' {
            out.resize(out.len() + TAB - out.len() % TAB, ' ');
        } else if c.is_control() {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

/// Words that color as keywords, across the languages drawn: a thumbnail needn't know which language each is from.
#[rustfmt::skip]
const KEYWORDS: &[&str] = &[
    "and", "as", "async", "await", "bool", "break", "case", "catch", "char", "class", "const", "continue", "def",
    "default", "delete", "do", "double", "elif", "else", "elseif", "end", "enum", "export", "extends", "extern",
    "false", "False", "final", "finally", "float", "float2", "float3", "float4", "fn", "for", "foreach", "from",
    "func", "function", "half", "half2", "half3", "half4", "if", "impl", "implements", "import", "in", "include",
    "inout", "int", "interface", "is", "lambda", "layout", "let", "local", "mat3", "mat4", "match", "mut", "namespace",
    "new", "nil", "None", "not", "null", "or", "out", "override", "package", "pass", "private", "protected", "pub",
    "public", "raise", "readonly", "repeat", "return", "sampler2D", "self", "signal", "static", "struct", "super",
    "switch", "then", "this", "throw", "trait", "true", "True", "try", "type", "uniform", "until", "use", "using",
    "var", "varying", "vec2", "vec3", "vec4", "virtual", "void", "where", "while", "with", "yield",
];

/// Reads lines into inks, carrying what spans lines (a block comment, a fenced code block, a markup tag).
struct Lexer {
    lang: Lang,
    /// Inside a block comment, until this closes it.
    comment: Option<&'static str>,
    /// Markdown: inside a fenced code block. Markup: inside a tag.
    inside: bool,
}

impl Lexer {
    fn new(lang: Lang) -> Self {
        Self { lang, comment: None, inside: false }
    }

    fn line(&mut self, c: &[char]) -> Vec<Ink> {
        let mut out = vec![Ink::Space; c.len()];
        match self.lang {
            Lang::Markdown => self.markdown(c, &mut out),
            Lang::Markup => self.markup(c, &mut out),
            _ => self.code(c, &mut out),
        }
        for (i, ch) in c.iter().enumerate() {
            if ch.is_whitespace() {
                out[i] = Ink::Space;
            }
        }
        out
    }

    fn code(&mut self, c: &[char], out: &mut [Ink]) {
        let lang = self.lang;
        let starts = |i: usize, s: &str| s.chars().enumerate().all(|(k, x)| c.get(i + k) == Some(&x));
        let first = c.iter().position(|ch| !ch.is_whitespace()).unwrap_or(c.len());
        if lang == Lang::Ini && c.get(first) == Some(&'[') {
            out[first..].fill(Ink::Name);
            return;
        }
        let mut i = 0;
        while i < c.len() {
            if let Some(close) = self.comment {
                let end = (i..c.len()).find(|&k| starts(k, close)).map(|k| k + close.len());
                let to = end.unwrap_or(c.len());
                out[i..to].fill(Ink::Comment);
                if end.is_some() {
                    self.comment = None;
                }
                i = to;
                continue;
            }
            let ch = c[i];
            let line_comment = match lang {
                Lang::Code | Lang::Json => starts(i, "//"),
                Lang::HashCode => ch == '#',
                Lang::Lua => starts(i, "--"),
                Lang::Ini => i == first && (ch == ';' || ch == '#'),
                Lang::Yaml => ch == '#' && (i == 0 || c[i - 1].is_whitespace()),
                _ => false,
            };
            if line_comment {
                out[i..].fill(Ink::Comment);
                return;
            }
            if matches!(lang, Lang::Code | Lang::Json) && starts(i, "/*") {
                self.comment = Some("*/");
                out[i..i + 2].fill(Ink::Comment);
                i += 2;
                continue;
            }
            if lang == Lang::Lua && starts(i, "--[[") {
                self.comment = Some("]]");
                continue;
            }
            let quotes = match lang {
                Lang::Prose | Lang::Csv => ch == '"',
                Lang::Code => matches!(ch, '"' | '\'' | '`'),
                _ => matches!(ch, '"' | '\''),
            };
            if quotes {
                let mut k = i + 1;
                while k < c.len() && c[k] != ch {
                    k += if c[k] == '\\' { 2 } else { 1 };
                }
                let to = (k + 1).min(c.len());
                // A string that a colon follows is a key.
                let key = matches!(lang, Lang::Json | Lang::Yaml) && next_is(c, to, ':');
                out[i..to].fill(if key { Ink::Name } else { Ink::Str });
                i = to;
                continue;
            }
            let number_start = ch.is_ascii_digit()
                || ((ch == '-' || ch == '.')
                    && c.get(i + 1).is_some_and(char::is_ascii_digit)
                    && (i == 0 || !is_word(c[i - 1])));
            if number_start {
                let mut k = i + 1;
                while k < c.len() && (is_word(c[k]) || c[k] == '.') {
                    k += 1;
                }
                out[i..k].fill(Ink::Number);
                i = k;
                continue;
            }
            if is_word(ch) {
                let mut k = i + 1;
                while k < c.len() && (is_word(c[k]) || (lang == Lang::Yaml && c[k] == '-')) {
                    k += 1;
                }
                let word: String = c[i..k].iter().collect();
                let ink = match lang {
                    Lang::Yaml | Lang::Ini if next_is(c, k, ':') || next_is(c, k, '=') => Ink::Name,
                    Lang::Prose | Lang::Csv => {
                        if word.len() > 1 && word.chars().all(|x| !x.is_lowercase()) {
                            Ink::Name
                        } else {
                            Ink::Text
                        }
                    }
                    _ if KEYWORDS.contains(&word.as_str()) => Ink::Keyword,
                    Lang::Yaml | Lang::Ini | Lang::Json => Ink::Text,
                    _ if next_is(c, k, '(') || word.starts_with(|x: char| x.is_uppercase()) => Ink::Name,
                    _ => Ink::Text,
                };
                out[i..k].fill(ink);
                i = k;
                continue;
            }
            out[i] = Ink::Punct;
            i += 1;
        }
    }

    fn markup(&mut self, c: &[char], out: &mut [Ink]) {
        let starts = |i: usize, s: &str| s.chars().enumerate().all(|(k, x)| c.get(i + k) == Some(&x));
        let mut i = 0;
        while i < c.len() {
            if self.comment.is_some() {
                let end = (i..c.len()).find(|&k| starts(k, "-->")).map(|k| k + 3);
                let to = end.unwrap_or(c.len());
                out[i..to].fill(Ink::Comment);
                if end.is_some() {
                    self.comment = None;
                }
                i = to;
                continue;
            }
            if starts(i, "<!--") {
                self.comment = Some("-->");
                continue;
            }
            let ch = c[i];
            if !self.inside {
                if ch == '<' {
                    self.inside = true;
                    out[i] = Ink::Punct;
                    i += 1;
                    // The tag's name, after `</`, `<?` or `<!`.
                    while i < c.len() && matches!(c[i], '/' | '?' | '!') {
                        out[i] = Ink::Punct;
                        i += 1;
                    }
                    while i < c.len() && (is_word(c[i]) || matches!(c[i], ':' | '-' | '.')) {
                        out[i] = Ink::Name;
                        i += 1;
                    }
                    continue;
                }
                out[i] = if ch.is_ascii_digit() { Ink::Number } else { Ink::Text };
                i += 1;
                continue;
            }
            match ch {
                '>' => {
                    self.inside = false;
                    out[i] = Ink::Punct;
                    i += 1;
                }
                '"' | '\'' => {
                    let k = (i + 1..c.len()).find(|&k| c[k] == ch).map_or(c.len(), |k| k + 1);
                    out[i..k].fill(Ink::Str);
                    i = k;
                }
                _ if is_word(ch) => {
                    while i < c.len() && (is_word(c[i]) || matches!(c[i], ':' | '-' | '.')) {
                        out[i] = Ink::Attr;
                        i += 1;
                    }
                }
                _ => {
                    out[i] = Ink::Punct;
                    i += 1;
                }
            }
        }
    }

    fn markdown(&mut self, c: &[char], out: &mut [Ink]) {
        let first = c.iter().position(|ch| !ch.is_whitespace()).unwrap_or(c.len());
        let rest = &c[first..];
        if rest.starts_with(&['`', '`', '`']) || rest.starts_with(&['~', '~', '~']) {
            self.inside = !self.inside;
            out[first..].fill(Ink::Punct);
            return;
        }
        if self.inside {
            out.fill(Ink::Str);
            return;
        }
        match rest.first() {
            Some('#') => return out[first..].fill(Ink::Name),
            Some('>') => return out[first..].fill(Ink::Comment),
            _ => {}
        }
        let mut i = first;
        // A list's marker.
        if matches!(rest, ['-' | '*' | '+', ' ', ..]) {
            out[i] = Ink::Punct;
            i += 1;
        }
        while i < c.len() {
            let ch = c[i];
            match ch {
                '`' => {
                    let k = (i + 1..c.len()).find(|&k| c[k] == '`').map_or(c.len(), |k| k + 1);
                    out[i..k].fill(Ink::Str);
                    i = k;
                }
                '[' => {
                    let k = (i + 1..c.len()).find(|&k| c[k] == ']').map_or(c.len(), |k| k + 1);
                    out[i..k].fill(Ink::Keyword);
                    i = k;
                    if c.get(i) == Some(&'(') {
                        let k = (i..c.len()).find(|&k| c[k] == ')').map_or(c.len(), |k| k + 1);
                        out[i..k].fill(Ink::Punct);
                        i = k;
                    }
                }
                '*' | '_' | '|' => {
                    out[i] = Ink::Punct;
                    i += 1;
                }
                _ => {
                    out[i] = if ch.is_ascii_digit() { Ink::Number } else { Ink::Text };
                    i += 1;
                }
            }
        }
    }
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Whether the next character that isn't a space, from `i` on, is `want`.
fn next_is(c: &[char], i: usize, want: char) -> bool {
    c[i.min(c.len())..].iter().find(|ch| **ch != ' ') == Some(&want)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_told_from_binary() {
        assert!(looks_like_text(b"[InternetShortcut]\r\nURL=https://kenney.nl/\r\n"));
        assert!(looks_like_text("\u{feff}Caf\u{e9} cr\u{e8}me\n".as_bytes()));
        // Cut off mid-character by the read.
        assert!(looks_like_text(&"na\u{ef}ve".as_bytes()[..3]));
        assert!(!looks_like_text(b"PK\x03\x04\x14\x00\x00\x00"));
        assert!(!looks_like_text(b"\x89PNG\r\n\x1a\n"));
        assert!(!looks_like_text(b"\x01\x02\x03\x04 header \x05\x06\x07"));
        assert!(!looks_like_text(b"  \n\t "));
        assert!(!looks_like_text(b""));
    }

    fn inks(lang: Lang, text: &str) -> Vec<Vec<Ink>> {
        let mut lexer = Lexer::new(lang);
        text.lines().map(|l| lexer.line(&expand_tabs(l))).collect()
    }

    /// One letter per character: k keyword, s string, n number, c comment, p punctuation, N name, a attribute, t
    /// text, and a space for a space.
    fn letters(line: &[Ink]) -> String {
        line.iter()
            .map(|i| match i {
                Ink::Space => ' ',
                Ink::Text => 't',
                Ink::Keyword => 'k',
                Ink::Str => 's',
                Ink::Number => 'n',
                Ink::Comment => 'c',
                Ink::Punct => 'p',
                Ink::Name => 'N',
                Ink::Attr => 'a',
            })
            .collect()
    }

    #[test]
    fn code_colors_keywords_strings_numbers_and_comments() {
        let l = inks(Lang::Code, "let x = \"a b\"; // done\n\tfoo(42);");
        assert_eq!(letters(&l[0]), "kkk t p ss ssp cc cccc");
        assert_eq!(letters(&l[1]), "    NNNpnnpp");
    }

    #[test]
    fn a_block_comment_runs_on_across_lines() {
        let l = inks(Lang::Code, "a /* one\ntwo */ b");
        assert_eq!(letters(&l[0]), "t cc ccc");
        assert_eq!(letters(&l[1]), "ccc cc t");
    }

    #[test]
    fn json_keys_are_names_and_values_strings() {
        let l = inks(Lang::Json, r#"{"id": "x", "n": -1.5, "ok": true}"#);
        assert_eq!(letters(&l[0]), "pNNNNp sssp NNNp nnnnp NNNNp kkkkp");
    }

    #[test]
    fn markup_colors_tags_attributes_and_text() {
        let l = inks(Lang::Markup, "<map width=\"8\">hi</map> <!-- note -->");
        assert_eq!(letters(&l[0]), "pNNN aaaaapssspttppNNNp cccc cccc ccc");
    }

    #[test]
    fn yaml_keys_markdown_headings_and_prose_capitals() {
        assert_eq!(letters(&inks(Lang::Yaml, "  m_Name: Hero # x")[0]), "  NNNNNNp tttt c c");
        assert_eq!(letters(&inks(Lang::Markdown, "# Title\n- see `code`")[1]), "p ttt ssssss");
        assert_eq!(letters(&inks(Lang::Prose, "READ me 2")[0]), "NNNN tt n");
    }

    #[test]
    fn a_file_is_drawn_from_the_top_left_as_long_as_its_lines() {
        let img = draw(b"fn main() {\n    let x = 1;\n}\n", Lang::Code, 256).unwrap();
        let ground = Rgba(GROUND);
        // The first line's first letters are inked; the right of the picture and its foot aren't.
        assert_ne!(*img.get_pixel(PAD + 1, PAD), ground);
        assert_eq!(*img.get_pixel(250, PAD), ground);
        assert_eq!(*img.get_pixel(PAD + 1, 200), ground);
        // The second line is indented: its start is dark.
        assert_eq!(*img.get_pixel(PAD + 1, PAD + 6), ground);
    }

    /// Drawn scripts are of the kind "other", whose thumbnails are otherwise pictures: their silhouettes stay out of
    /// searches by color and by look.
    #[test]
    fn drawn_files_of_kind_other_stay_out_of_the_looks_index() {
        for ext in EXTS.iter().flat_map(|(_, exts)| exts.iter()) {
            if hoard_core::AssetKind::from_ext(ext) == hoard_core::AssetKind::Other {
                assert!(crate::looks::KINDS_SQL.contains(&format!("'{ext}'")), "{ext} isn't left out of the looks");
            }
        }
    }

    #[test]
    fn binary_and_empty_files_get_no_picture() {
        assert!(draw(b"\x00\x01binary", Lang::Yaml, 256).is_none());
        assert!(draw(b"  \n\n \t\n", Lang::Prose, 256).is_none());
        assert!(draw(b"", Lang::Prose, 256).is_none());
    }
}
