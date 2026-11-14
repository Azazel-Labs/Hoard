# Store file formats

Some stores ship a small file beside the art that says what it is: Megascans' `<id>.json`, Sketchfab's
`license.txt`. Hoard reads these to name the pack, tag it, describe it, and find its store page. Each store's file is
described by a TOML file in [`crates/hoardd/formats/`](../crates/hoardd/formats/). No Rust is needed. Adding a store
means adding a TOML file and a sample, which makes a small PR.

## Adding a store

1. Write `crates/hoardd/formats/<store>.toml` (see below), and add it to `BUILT_IN` in
   [`store_formats.rs`](../crates/hoardd/src/store_formats.rs), in name order. A test fails if a file is missing from
   that list.
2. Put a real (or realistic, made-up) file in `crates/hoardd/formats/samples/<store>/`. Beside it, put
   `<file>.expected.json` with the values you expect it to read. Only the keys you list are compared. Start with
   `{"title": ""}`: the failing test prints everything that was read, and you keep the parts that are right. The
   sample folder's name must match the format's `name`.
3. If the file gives the store page's address, add the store's host to `STORES` in
   [`context.rs`](../crates/hoardd/src/context.rs) if it isn't there. A page on a known store becomes the pack's
   store page, with no search needed.
4. `cargo test -p hoardd store_formats`.

To try a format without rebuilding, drop the TOML file into `<data dir>/formats/` and restart the server. A file there
with the same `name` as a built-in format replaces it. The log says which files were read, and why any wasn't.
Packs already in the library are read again with More → "Read the pack's files again".

## The file

```toml
# What the file is, where it comes from, and an example of what's in it.
name = "Megascans"          # the store or library; the description says "3 Megascans assets"
called = "Megascans file"   # optional; the store search's reasons say "Same title as the pack's Megascans file"
seller = "Quixel Megascans" # optional; when the file doesn't name one
files = ["*.json"]          # file names it reads, case-insensitive (* and ?)
syntax = "json"             # or "text"
require = ["semanticTags", ["id", "meta"]]  # how to tell the file is the store's (below)
tags = ["megascans"]        # optional; added to everything it describes
origin_words = "scanned in" # optional; "… from Quixel Megascans, scanned in Norway."
id_from_file_name = true    # optional; the file's name (`xeokah0.json`) is the asset's id when the file gives none

[fields]
title = "semanticTags.name"
# …
```

A file is read as the first format whose `files` match its name, whose `require` holds, and in which `title` finds
something. Other files are skipped. Keep `require` specific: `*.json` matches every JSON file in every pack.

`require` lists the ways to recognise the file. Each way is one selector, or a list of selectors that must all find
something. `["semanticTags", ["id", "meta"]]` means: it has `semanticTags`, or it has both `id` and `meta`.

### Fields

Every field is optional except `title`. A field takes one selector or a list of them. A field with one value takes
the first that finds something, so put the best place first. A field with many values takes all of them.

| Field | Values | Used for |
|---|---|---|
| `title` | one | the pack's name and the store search's exact title |
| `id` | one | the asset's id; it must be letters, digits, `_` or `-` |
| `type` | one | the kind of asset ("3D asset", "surface"); also a tag |
| `words` | many | tags; comma-separated values are split |
| `colors` | many | `color:` tags |
| `origin` | many | where it was made or scanned, for the description |
| `seller` | one | who made or sells it; beats the format's `seller` |
| `license` | one | the license as the file writes it ("CC-BY-4.0") |
| `url` | one | the store page; it must start with `http://` or `https://` |
| `size_min`, `size_max`, `size` | one | its size in words ("1 m to 2 m"), from a range or else `size` |
| `texel_density` | one | for the description |
| `tiles` | yes/no | it's a surface that repeats: anything found that isn't `false`, `no`, `0` or empty |
| `physical_size` | one | a surface's real size per repeat ("2x2 m"; without a unit, numbers over 20 are decimetres) |
| `displacement` | one | a surface's height-map depth ("2 cm") |

Three fields are groups, and each can appear more than once. `each` finds the items, and the other selectors are read
inside each item:

```toml
[[fields.dims]]      # measurements: "length 1.2 m"
each = "meta[key=length|width|height|depth]"
name = "key"
value = "value"

[[fields.maps]]      # texture maps; the kind ("albedo", "normal") beats guessing from the file name
each = "maps[]"
kind = "type"
file = "uri"

[[fields.lods]]      # levels of detail; numbered by their file names (`_LOD2`), or else in order
each = "meshes[type=lod]"
tris = "tris"
files = "uris.uri"
```

## Selectors in JSON

A selector is a path of keys separated by dots. Values come back as text: numbers and booleans are written out,
arrays are flattened, and empty strings are dropped.

| Selector | Finds |
|---|---|
| `name` | the key `name`. If no key matches exactly, the case is ignored (`Name`) |
| `semanticTags.name` | a key inside a key |
| `maps.uri` | when `maps` is an array, `uri` in each of its items |
| `maps[]` | each item of `maps` (for a group's `each`) |
| `meta[key=scanArea].value` | in the array `meta`, the items whose `key` is `scanArea` (case ignored), then their `value` |
| `meta[key=width\|height]` | items whose `key` is either |
| `semanticTags.*size*` | keys matching a pattern: `*` is any text, `?` is one character |
| `semanticTags.*!(name,color)` | any key except these (patterns too) |
| `semanticTags.*[array]` | only values of a type: `array`, `string`, `number`, `bool`, `object` |
| `type ~ *surface*` | only values matching the pattern (case ignored) |
| `averageColor !~ #*` | only values not matching it |

## Selectors in text

With `syntax = "text"`, each selector is a regular expression ([Rust's syntax](https://docs.rs/regex)). It works
line by line: `^` and `$` match at each line's start and end, and Windows line endings are made plain. The first
group is the value; with no group, the whole match is. Use single quotes in TOML so backslashes stay as written:

```toml
title = '^\* title:[ \t]*(.+?)[ \t]*$'
```

A selector that doesn't compile, a misspelt field, or a key Hoard doesn't know stops the file from loading, and the
error names the field.
