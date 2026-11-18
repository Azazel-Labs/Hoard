//! The tools an agent is offered, with what each takes and returns: searching and inspecting, the sound tools, and
//! tagging for those who may change the library.

use serde_json::{Value, json};

/// The tools that change the library, which accounts that can only look don't get.
pub(super) const CHANGES: [&str; 3] = ["tag_asset", "update_asset", "save_tags"];

pub(super) fn tool_list(may_change: bool) -> Value {
    let (read, write) = (read_hints(), write_hints());
    let tools = json!([
        {
            "name": "get_asset",
            "title": "Get asset details",
            "description": "Everything known about one asset: metadata (size, duration, poly count…), tags with their source, linked dependencies (textures, buffers, materials, animation clips), what uses it, identical copies elsewhere, near copies (the same picture, sound or model at another size, rate or format, the best of them marked), and URLs.",
            "inputSchema": { "type": "object", "properties": { "id": { "type": "string" } }, "required": ["id"] },
            "annotations": read
        },
        {
            "name": "get_bundle",
            "title": "Get download links for an asset and its dependencies",
            "description": "Direct download URLs for an asset plus every file it needs, each with the relative path to save it at so references keep working, and a single zip URL.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "include_animations": { "type": "boolean", "default": false }
                },
                "required": ["id"]
            },
            "annotations": read
        },
        {
            "name": "list_packs",
            "title": "List packs",
            "description": "All uploaded packs with status, license, asset count and size.",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": read
        },
        {
            "name": "list_collections",
            "title": "List collections",
            "description": "Hand-picked sets of assets (e.g. everything chosen for one level), with asset counts and a zip URL each. Pass a collection's id or name to search_assets as `collection` to see its contents.",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": read
        },
        {
            "name": "get_credits",
            "title": "Get credits",
            "description": "Who made the packs a project uses, where each is sold, its license and whether the license asks for attribution (yes, no, or 'terms': keep the license notice with the files; 'unknown' where the license isn't known). Give any mix of packs, collections, asset ids and a search; everything is put together. `markdown` returns a ready CREDITS.md.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "packs": { "type": "array", "items": { "type": "string" }, "description": "Pack ids or names." },
                    "collections": { "type": "array", "items": { "type": "string" }, "description": "Collection ids or names." },
                    "ids": { "type": "array", "items": { "type": "string" }, "description": "Asset ids." },
                    "query": { "type": "string", "description": "Also credit the packs of what this search finds." },
                    "kind": { "type": "string", "description": "With `query`: only this kind." },
                    "tags": { "type": "array", "items": { "type": "string" }, "description": "With `query`: all must match." },
                    "markdown": { "type": "boolean", "default": false, "description": "Return CREDITS.md text instead of JSON." }
                }
            },
            "annotations": read
        },
        {
            "name": "check_files",
            "title": "Check files before shipping",
            "description": "Given the BLAKE3 hashes of the files a project ships (b3sum), says which came from Hoard (the asset, its path and pack; a hash matches a file as Hoard keeps it, as it was uploaded before conversion, or a format Hoard made from it), the credits they need, and which files need a look first, with warnings: license_unknown, non_commercial, share_alike, keep_notice (ship the license notice), fab_personal (Fab's Personal tier), ai_generated / ai_edited (the image says so in its C2PA Content Credentials, IPTC metadata or generator settings). A file is ok when one pack it's in covers it. Hashes Hoard doesn't know are listed as not_found. `markdown` returns CREDITS.md with a ship-check section.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hashes": { "type": "array", "items": { "type": "string" }, "description": "BLAKE3 hashes, 64 hex digits each." },
                    "markdown": { "type": "boolean", "default": false, "description": "Return CREDITS.md text with the ship check instead of JSON." }
                },
                "required": ["hashes"]
            },
            "annotations": read
        },
        {
            "name": "list_tags",
            "title": "List tags",
            "description": "Most-used tags, optionally filtered by prefix (e.g. 'style:' or 'material:'). Useful to learn the vocabulary before searching.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "prefix": { "type": "string", "default": "" },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 200, "default": 50 }
                }
            },
            "annotations": read
        },
        {
            "name": "tag_asset",
            "title": "Add or remove tags",
            "description": "Add and/or remove tags on an asset. Added tags are recorded as human-sourced and are never overwritten by the automatic taggers.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "add": { "type": "array", "items": { "type": "string" } },
                    "remove": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["id"]
            },
            "annotations": write
        },
        {
            "name": "update_asset",
            "title": "Rename or recategorize an asset",
            "description": "Set an asset's display name, category or description (its own field, next to the AI's and the file's). Edited fields are locked against the AI tagger.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "name": { "type": "string" },
                    "category": { "type": "string" },
                    "description": { "type": "string" }
                },
                "required": ["id"]
            },
            "annotations": write
        }
    ]);
    let mut all = search_tools();
    if let Some(list) = all.as_array_mut() {
        for more in [tools, tagging_tools()] {
            if let Value::Array(more) = more {
                list.extend(more);
            }
        }
        if !may_change {
            list.retain(|t| !t["name"].as_str().is_some_and(|n| CHANGES.contains(&n)));
        }
    }
    all
}

fn search_tools() -> Value {
    let read = read_hints();
    json!([
        {
            "name": "search_assets",
            "title": "Search assets",
            "description": "Full-text search over asset names, paths, categories, descriptions and tags, with filters. When image search is on (Settings), pictures that show the words match too, so 'mossy rock' finds mossy rocks whatever they're called. Returns up to `limit` matches with thumbnails and download URLs.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Words to search for, e.g. 'grass footsteps'." },
                    "kind": { "type": "string", "enum": ["texture", "vector", "sheet", "audio", "model", "animation", "font", "shader", "material", "video", "document", "data", "other"] },
                    "tags": { "type": "array", "items": { "type": "string" }, "description": "All must match, e.g. ['sfx', 'material:stone']." },
                    "pack_id": { "type": "string" },
                    "collection": { "type": "string", "description": "Only assets in this collection (id or name)." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
                    "include_previews": { "type": "boolean", "default": false, "description": "Also return the first 8 thumbnails as images, so you can see what you found." }
                }
            },
            "annotations": read
        },
        {
            "name": "find_similar",
            "title": "Find assets that look or sound alike",
            "description": "Assets whose thumbnail looks like this asset's (colors, surface grain, edges, layout, and, when image search is on, what it shows), across every pack, most alike first: other rocks for a rock texture, near-copies at other sizes, renders of similar models. For a sound, the sounds that sound like it (its spectrum, how it rises and dies away, how bright, noisy and busy it is), whatever they're called. Same result shape as search_assets.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "The asset to match." },
                    "kind": { "type": "string", "enum": ["texture", "vector", "model", "material", "animation", "video", "audio", "other"] },
                    "query": { "type": "string", "description": "Optional words the matches must also have." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
                    "include_previews": { "type": "boolean", "default": false, "description": "Also return the first 8 thumbnails as images." }
                },
                "required": ["id"]
            },
            "annotations": read
        },
        {
            "name": "find_by_color",
            "title": "Find assets by color",
            "description": "Assets with much of a color in their thumbnail, those covered most by it first (colors are compared perceptually, so a slightly different shade still matches). Same result shape as search_assets.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "color": { "type": "string", "description": "Hex color, e.g. '#3a7d2c'." },
                    "kind": { "type": "string", "enum": ["texture", "vector", "model", "material", "animation", "video", "other"] },
                    "query": { "type": "string", "description": "Optional words the matches must also have." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
                    "include_previews": { "type": "boolean", "default": false, "description": "Also return the first 8 thumbnails as images." }
                },
                "required": ["color"]
            },
            "annotations": read
        },
        {
            "name": "search_sounds",
            "title": "Find sounds by what they sound like",
            "description": "Sounds ranked by how well what Hoard measured of them (how long, how sharply they start, whether they ring on or cut off, how bright and noisy, whether the pitch rises or falls, how loud) fits a word imitating them ('WHOOOM', 'pew', 'krssh'), a shape you describe, or both. Hoard reads the word itself and returns its reading as `reading`; any part of `shape` you give replaces that part of the reading, so give both when you know more than the spelling says (imitation 'whooom' with shape {seconds: 1.5, pitch: 'rising'} for a swelling whoosh). `query` and `tags` are words the sound's name, path or tags must have ('sci-fi', 'impact'): keep them few, or leave them out to search by sound alone. `bpm_min`, `bpm_max` and `key` find music and loops by tempo and key. Each sound comes with what was measured, the same in words, and a link to listen to it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "imitation": { "type": "string", "description": "The word for the noise, spelled as said, e.g. 'WHOOOM' (capitals mean loud), 'pew', 'shhhhh'." },
                    "shape": {
                        "type": "object",
                        "description": "Your own reading of the sound; every part is optional.",
                        "properties": {
                            "seconds": { "type": "number", "minimum": 0.01, "description": "About how long." },
                            "attack": { "type": "string", "enum": ["sharp", "soft"], "description": "Starts at once (a hit, a click) or fades in." },
                            "ending": { "type": "string", "enum": ["rings", "cuts_off"], "description": "Rings on and dies away, or stops short." },
                            "brightness": { "type": "string", "enum": ["dark", "mid", "bright"], "description": "Deep and dull, or high and crisp." },
                            "texture": { "type": "string", "enum": ["tonal", "mixed", "noisy"], "description": "A clear pitch (a beep, a hum) or hiss and air (wind, a whoosh, static)." },
                            "pitch": { "type": "string", "enum": ["falling", "steady", "rising"] },
                            "loud": { "type": "boolean" }
                        }
                    },
                    "query": { "type": "string", "description": "Optional words the sounds must also have. Given alone, with sound search on (an admin setting), it's also a description of the sound ('distant thunder roll', 'glassy UI blip'), matching sounds that sound like it whatever they're called." },
                    "tags": { "type": "array", "items": { "type": "string" }, "description": "All must match, e.g. ['sfx']." },
                    "bpm_min": { "type": "number", "description": "Music and loops at this tempo or faster, as the file states it or as Hoard heard it (half or double counts too)." },
                    "bpm_max": { "type": "number", "description": "Music and loops at this tempo or slower." },
                    "key": { "type": "string", "description": "Music in this key: 'A minor', 'Am', 'F# major', 'E♭ major' or Camelot '8A'." },
                    "compatible_keys": { "type": "boolean", "default": false, "description": "With `key`: also the keys that mix with it (its relative major or minor, and a fifth either side)." },
                    "pack_id": { "type": "string" },
                    "collection": { "type": "string", "description": "Only sounds in this collection (id or name)." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 }
                }
            },
            "annotations": read
        },
        {
            "name": "describe_sound",
            "title": "Describe a sound",
            "description": "What one sound sounds like, as measured: how long, how it starts and ends, how bright and noisy, where its pitch goes, how loud, whether it loops, in numbers and in words, with its tags and a link to listen to it.",
            "inputSchema": { "type": "object", "properties": { "id": { "type": "string" } }, "required": ["id"] },
            "annotations": read
        },
        {
            "name": "sounds_like",
            "title": "Find sounds like one",
            "description": "The sounds that sound most like this one (its spectrum, how it rises and dies away, how bright, noisy and busy it is), whatever they're called and whichever pack they're in, each described like search_sounds' results.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "The sound to match." },
                    "query": { "type": "string", "description": "Optional words the matches must also have." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 }
                },
                "required": ["id"]
            },
            "annotations": read
        }
    ])
}

fn read_hints() -> Value {
    json!({ "readOnlyHint": true, "openWorldHint": false })
}

fn write_hints() -> Value {
    json!({ "readOnlyHint": false, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false })
}

/// The tools for describing assets nothing has described yet (see the `tag_pack` prompt).
fn tagging_tools() -> Value {
    let (read, write) = (read_hints(), write_hints());
    json!([
        {
            "name": "next_to_tag",
            "title": "Next assets to describe",
            "description": "The next files nothing has described yet (no AI or human tags), up to 16 at a time, from one pack: path, kind, measured facts, the offline tagger's tags, the pack's context (store page, README), and a contact sheet image whose white numbers match each file's `n`. A file with `variants` stands for a family of near-identical takes. Describe them and call save_tags; saved files drop out, so call again for the next batch until `files` is empty.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "pack": { "type": "string", "description": "Pack id or name. Leave out to take the newest pack that has undescribed files." },
                    "skip": { "type": "integer", "minimum": 0, "default": 0, "description": "Skip this many, e.g. files you decided to leave alone." },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 16, "default": 16 }
                }
            },
            "annotations": read
        },
        {
            "name": "save_tags",
            "title": "Save descriptions and tags",
            "description": "Store names, categories, descriptions and tags for assets (usually a batch from next_to_tag), recorded as AI-made the same way Hoard's own AI tagger records them: never overriding what a person typed, and an answer for a family's representative covers its variants. Optionally the pack's description and tags.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "assets": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "id": { "type": "string" },
                                "name": { "type": "string", "description": "Short human title, no extension." },
                                "category": { "type": "string", "description": "One of Hoard's categories (see the tag_pack prompt)." },
                                "description": { "type": "string", "description": "1-3 sentences." },
                                "tags": { "type": "array", "items": { "type": "string" }, "description": "3-8 lowercase kebab-case tags." }
                            },
                            "required": ["id"]
                        }
                    },
                    "pack": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "string" },
                            "description": { "type": "string" },
                            "tags": { "type": "array", "items": { "type": "string" } }
                        },
                        "required": ["id"]
                    }
                },
                "required": ["assets"]
            },
            "annotations": write
        }
    ])
}
