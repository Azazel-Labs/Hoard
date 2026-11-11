# Changelog

All notable changes to Hoard are recorded here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and versions follow [Semantic Versioning](https://semver.org/). While the version is 0.x, minor releases may change
the API, database schema or config; the release notes call it out when they do.

## [Unreleased]

## [0.8.0] - 2026-11-11

### Added
- ffmpeg, fetched by the server: `HOARD_FFMPEG=auto` (default) uses a system ffmpeg with LAME, libvorbis and libopus,
  or downloads a static build into `<data dir>/tools` in the background and checks its published SHA-256. A path
  or `off` also work. The packs page and `GET /api/stats` (`ffmpeg`) show its state.
- MP3, OGG Vorbis and Opus downloads use ffmpeg's reference encoders when it's ready.
- Audio Hoard doesn't decode itself (AAC/M4A, Opus, WMA, CAF, WavPack, APE, MKA, W64, AU) gets waveforms, levels and
  downloads through ffmpeg. These extensions are now classified as audio.
- `GET /api/assets/{id}/as/{format}/status` starts or checks a conversion without downloading it.
- Light, dark and "match system" themes for the portal, chosen in the header and remembered per browser. Font
  specimens are tinted to the theme (white glyphs on dark, dark glyphs on light), in tiles, details and covers.
- A background activity indicator in the portal header: hover or click for what the server is working on (ingests
  with file counts, tagging, Blender and Unreal conversion, scrubs, "download as" conversions, zips being streamed,
  the ffmpeg download) with progress, and the queued jobs behind them. `GET /api/activity`.
- Pack covers include font specimens; packs and collections list `preview_kinds` beside `preview`.

### Changed
- Waveform thumbnails are mirrored rounded bars: a solid core at each moment's loudness (RMS) and a lighter halo to
  its peak, gently compressed so quiet sounds show a shape, and colored by how bright the sound is (azure lows,
  violet and orchid mids, orange highs). Re-tag a pack to redraw its existing waveforms.
- Conversions run detached from the request: `/as/{format}` waits up to `?wait=` seconds (default 20), then answers
  `202` with progress and `Retry-After`. A conversion carries on if the client leaves, and requests for the same one
  share it. "Download as" in the portal shows the progress.
- Bundle zips stream as they're written instead of being built first, so downloads start at once (no
  `Content-Length`).
- Heavy work (ingest analysis, conversions, thumbnails, contact sheets, Unreal conversion, Blender and ffmpeg) runs
  at a lower CPU priority than request handling, on Linux.

### Removed
- The bundled libvorbis encoder (`vorbis_rs`): OGG Vorbis downloads need ffmpeg. The pure-Rust MP3 and Opus encoders
  stay as the fallback.

### Fixed
- "Download as" could start with no format selected until the detail finished loading.

## [0.7.0] - 2026-11-11

### Added
- Download as MP3, OGG Vorbis or Opus, from any audio or music file. The MP3 and Opus encoders are pure Rust; Vorbis
  uses the BSD-licensed libvorbis, compiled in.
- Tracker modules (MOD, XM, S3M, IT) and MIDI are rendered to audio: waveform thumbnail, duration, module and MIDI
  facts (tempo, tracks, General MIDI instruments), an MP3 preview that plays in the grid, `music`/`tracker-module`/
  `midi` tags, and downloads as WAV, FLAC, MP3, OGG or Opus. MIDI uses a SoundFont (`HOARD_SOUNDFONT` or a system GM
  SoundFont), with a built-in synth as a fallback.
- WOFF2 fonts are read natively (details, specimen, tags), and fonts convert between TTF/OTF, WOFF and WOFF2.
- Basis Universal textures (KTX2 with BasisLZ/ETC1S or UASTC, and `.basis` files) are transcoded in pure Rust.
- Text inside SVGs is drawn, using system fonts.
- OBJ (with MTL colors, base color and normal textures) and binary FBX (geometry, node transforms, axis and unit
  settings, materials, embedded and nearby textures) convert to GLB natively, without Blender. FBX gets a rendered
  thumbnail.

### Changed
- The Docker image includes the DejaVu fonts (for SVG text) and the small TimGM6mb General MIDI SoundFont.

## [0.6.0] - 2026-11-11

This release migrates the database to schema 8 on first start (new tables only).

### Added
- Download as another format: images (including PSD composites, Krita, DDS, KTX2, SVG and Aseprite) to PNG, JPEG,
  WebP, TGA, TIFF or EXR, optionally scaled down; audio to WAV or FLAC; glTF↔GLB; STL, PLY and MagicaVoxel to GLB;
  WOFF to TTF/OTF; Blender-converted models to GLB. Converted on first request and cached.
  `GET /api/assets/{id}/as/{format}[?max=]`; assets list their `formats` in the detail view; "Download as" in the
  portal.
- Unity packages: `.unitypackage` files unpack into their original `Assets/` tree. GUIDs from `.meta` files link
  materials, prefabs, scenes and controllers to what they use, and bundles recreate the `.meta` files so the GUIDs
  survive a round trip into Unity.
- GPU textures decoded in pure Rust: DDS (BC1-7, raw and float formats, legacy DXT/bitmask headers) and KTX2 (BCn,
  ETC2/EAC, ASTC, raw; zstd/zlib supercompression), with GPU format and mip count. SVG and Aseprite thumbnails.
- Fonts: family, style, designer, license, weight, italic, monospace, variable axes, glyph count and writing systems,
  an "Aa" specimen thumbnail, and font tags (`weight:*`, `script:*`, `monospace`, `variable-font`, `icons`). WOFF 1
  is unwrapped natively.
- STL, PLY and MagicaVoxel `.vox` models read natively, with rendered thumbnails.
- Links for Tiled (`.tmx`/`.tsx`/`.tx`), LDtk, BMFont `.fnt`, Spine/libGDX `.atlas`, TexturePacker JSON and Godot
  `.tres`/`.tscn`.
- PSD/PSB full-size composite decoding (RLE or raw, 8/16/32-bit), used for downloads.

### Changed
- Unity `.meta` files are read for GUIDs before being left out of a pack (still not stored as assets).

## [0.5.1] - 2026-11-11

### Removed
- The optional Oodle library (`HOARD_OODLE_LIB`) and the `oodle` dependency: `hoard-unreal` links no engine code.
  Textures whose raw pixels UE5 stored Oodle-compressed go to `hoard unreal-worker`; PNG/JPEG-sourced textures and
  WAVs still convert natively. `hoard_unreal::convert::convert` no longer takes an options argument.

## [0.5.0] - 2026-11-11

This release migrates the database to schema 7 on first start (new tables only). Unreal packs are now converted
after ingest by default; set `HOARD_UNREAL_CONVERT=off` to keep the old behaviour.

### Added
- `hoard-unreal`, a standalone crate with Hoard's own Unreal package reader: tagged properties for UE 4.x, 5.0-5.3
  and 5.4+ (checked against every export of real UE 4.27 and 5.7 editor files), UE4 bulk data, and the UE5 package
  trailer with uncompressed and LZ4 payloads (Oodle through the optional runtime library).
- Unreal conversion to common formats: `Texture2D` to the imported PNG/JPEG or the source pixels as PNG (8/16-bit)
  or EXR (HDR formats), `SoundWave` to its WAV, materials and material instances to `material.json` (parent,
  parameters, PBR slot guesses), `DataTable` to JSON. The converted file takes over the asset (id, tags, links,
  collections) and records its origin; the original is kept or discarded.
  `HOARD_UNREAL_CONVERT=off|keep|replace` (default `keep`), `HOARD_OODLE_LIB`.
- `hoard unreal-worker --editor <UnrealEditor-Cmd>`: converts the meshes and animations the server can't, by
  rebuilding each pack as a temporary project, exporting headless and uploading the FBX files.
- API: `POST /api/packs/{id}/unreal/convert`, `GET /api/packs/{id}/unreal`, `POST /api/packs/{id}/unreal/finish`,
  `GET /api/unreal/pending`, `POST /api/assets/{id}/replace`, `GET /api/assets/{id}/original`; assets carry
  `origin` and `unreal` in the detail view, packs carry `unreal_packages` and `unreal_converted`.
- Portal: "Convert Unreal content" on the pack page (status, reasons, discard and drop options), and the asset
  detail shows where a converted file came from, with a download for the kept original.
- Converted materials are analyzed as materials and link to their parent and textures.

### Fixed
- Links between Unreal packages resolve when the upload dropped the `Content/<Mount>/` folders.
- Array-of-struct properties in UE 5.2-5.3 packages (inner tag extensions byte).

## [0.4.0] - 2026-11-11

This release migrates the database to schema 6 on first start (new tables only).

### Added
- Unreal packages (`.uasset`, `.umap`) read without Unreal, UE 4.x through 5.7: class-based kind (mesh, texture,
  sound, material, animation…), engine version, the editor's Content Browser thumbnail, asset registry tags
  (dimensions, triangle counts, durations…), links from package imports resolved against the pack's `Content/`
  folders, `unreal`/`ue:<class>`/`engine:ueX.Y` tags, and display names without naming prefixes.
- Source files recovered from inside Unreal packages: a `SoundWave`'s original WAV and PNG texture sources,
  analyzed like the real files (waveforms, playback in the grid, tiling and color tags) and stored as a `source`
  derivative. `GET /api/assets/{id}/derivative/source[?download=1]` serves them by their real type and name;
  `sources=1` on selection and collection zips swaps them in for the `.uasset`.
- `cargo xtask unreal-export` and `tools/unreal_export.py`: export a pack's meshes and animations to FBX, textures
  to PNG/EXR and sounds to WAV with Unreal Editor in headless mode, ready to import.
- Collections: hand-picked sets of assets across packs. Create and fill from the selection bar or the detail
  panel; browse from the sidebar; rename, delete and download as a zip from the packs page.
  `GET|POST /api/collections`, `GET|PATCH|DELETE /api/collections/{id}`, `POST /api/collections/{id}/assets`,
  `/api/collections/{id}/bundle.zip`; `collection` search filter; `collection = "…"` in `hoard.toml` selections;
  MCP `list_collections` and a `collection` argument on `search_assets`.
- The sample pack includes a small Unreal content folder (a sound, a texture, a material instance and a mesh).
- MCP asset URLs include recovered sources; the LLM tagger sees the Unreal class and engine version.

### Changed
- "Supporting files" hidden from the grid are now glTF buffers and `.mtl` libraries only; Unreal material
  instances stay visible.

## [0.3.0] - 2026-11-11

### Added
- `cargo xtask`: `dev` (hoardd + Vite, optionally with a local model wired into the LLM tagger), `ci`, `web`,
  `sample`, `models list|fetch|verify|rm|path` and `llm serve|env`.
- Small local vision models for the tagger, downloaded from Hugging Face with resume and SHA-256 checks:
  Qwen3.5 2B (default) and 4B, Qwen3-VL 2B, Gemma 4 E2B, LFM2.5-VL 1.6B. Served with llama.cpp (`llama` or
  `llama-server`), reasoning off for thinking models.
- Optional `llm` service in the private compose file (llama.cpp container serving a downloaded model).
- VS Code workspace: recommended extensions, settings (clippy on save, 120-column rulers), tasks for every xtask
  command, debug configurations for hoardd (with or without a local model), tests, the CLI, xtask and the portal.
  `.editorconfig` for other editors.
- Portal: sound previews from the grid (play button or Space) with playback progress over the waveform; keyboard
  navigation (arrows, Enter, Escape, `/`, X) and previous/next in the detail panel; multi-select with Ctrl/Cmd,
  Shift ranges, X or long-press, with a selection bar to download everything as one zip or tag it all at once;
  tile size slider; collapsible filter groups with "more"; searches and filters in the URL; infinite scroll;
  loading placeholders; success messages; pack cover mosaics, a pack finder and a "More" menu on the packs page.
- `GET /api/bundle.zip?ids=…` zips a selection with its dependencies (a folder per pack when they span several).
- `hide_supporting` search parameter (default in the portal) leaves out glTF buffers and material libraries, with
  a count of what was hidden; packs carry `preview` asset ids for covers.

### Changed
- Phone layout: search gets its own row, Library/Packs stay reachable, filters slide in as a drawer, the detail
  panel's download actions stay pinned at the bottom.
- Display names keep abbreviations in capitals ("UI click", "SFX hit", "PBR rock").

## [0.2.0] - 2026-11-11

This release migrates the database to schema 5 on first start (new tables only; back up `hoard.db` first anyway).

### Added
- Resumable bulk imports of many gigabytes from the browser and the CLI: upload sessions register the file list,
  send small files in batches and large ones in 32 MB chunks, and resume after a dropped connection or closed tab.
  "Import folder" in the portal, with an optional split of a folder of packs into one pack per subfolder; an upload
  queue with speed and time left; `hoard upload --each`.
- Duplicate-import protection: the server fingerprints each import's file list and skips packs it already has
  ("Import again anyway" / `hoard upload --force` override).
- HTTPS with HTTP/2 (`HOARD_TLS_CERT`/`HOARD_TLS_KEY`, or `HOARD_TLS_SELF_SIGNED` with `HOARD_TLS_NAMES`), cleartext
  HTTP/2 (h2c) without TLS, and experimental HTTP/3 over QUIC (`HOARD_HTTP3`) advertised with `Alt-Svc`. Responses
  compressed with Brotli, zstd or gzip where it helps; immutable caching for the portal's hashed files.
- `hoard --ca-cert` (`HOARD_CA_CERT`) to trust a server's self-signed certificate.
- Metadata embedded in WAV files: BWF `bext`, `LIST/INFO`, iXML (Soundminer/BaseHead, including UCS category,
  subcategory and CatID), `smpl` loop points, ACID tempo/root note/one-shot, cue markers, RF64. Embedded titles and
  descriptions become the asset's name and description.
- Audio peak and RMS level and clipping detection, with `level:quiet`, `level:loud` and `clipped` tags.
- Universal Category System tags for sounds (`ucs:footsteps`, `ucs:swooshes/swish`) from embedded metadata, rules
  and the LLM; `loop`, `one-shot`, `bpm:N` and `root:<note>` tags.
- Variant families: numbered takes (`whoosh_01..12`, `Hit (3)`, `rock_v2`) are grouped; the detail panel lists them
  and edits can apply to the whole family (`apply_to_family` on `PATCH /api/assets/{id}`).
- Exact duplicates across packs listed under "Identical copies", and counted on the packs page.
- Pack context: README text captured at import, store links in it recognized (Fab, itch.io, Humble Bundle, Unity
  Asset Store, Kenney and others), and a "Store details" editor (store, URL, license tier, listing text) with
  best-effort fetching of the store page's title and description. `GET|PUT /api/packs/{id}/context`.
- LLM tagger: sees the pack context, sends each variant family once and applies the answer to every take, asks for
  UCS subcategories, and in vision mode sends one numbered contact sheet per batch instead of separate images.
- Multi-view model renders (front, back, side, top in one 2x2 image) stored as a `views` derivative when vision
  tagging is on, viewable from the detail panel.
- MCP: `search_assets` with `include_previews` returns the first 8 thumbnails as image content; `get_asset` includes
  variants and identical copies.
- Release tooling: `scripts/release.sh`, a release workflow that publishes binaries and a multi-arch container
  image to GHCR, and [RELEASING.md](RELEASING.md). The version shows in the portal and `GET /api/stats`.

### Changed
- The private-access compose file serves HTTPS with a self-signed certificate by default and maps UDP for HTTP/3.
- "whoosh"/"swoosh" sounds get their own `whoosh` tag instead of counting as melee.
- The sample pack generator adds library-style WAVs (with embedded metadata), a variant family and a README.

## [0.1.0] - 2026-11-11

First tagged version.

### Added
- `hoardd` server: content-addressed blob storage (BLAKE3) on local disk or any S3-compatible bucket, SQLite + FTS5
  metadata, durable job queue, REST API, Range-capable download proxy with immutable caching.
- Pack ingest with archive unpacking (zip/tar/tar.gz, nested), zip-slip and zip-bomb guards, junk-file filtering,
  pack naming from wrapper folders.
- Analysis: image facts, thumbnails, tiling, dominant color, pixel-art and normal-map detection; audio facts and
  waveform thumbnails; glTF/GLB/OBJ/FBX facts; headless wgpu model thumbnails (`hoard-render`).
- Source formats: `.blend` embedded previews (gzip/zstd), PSD/PSB embedded previews and facts, Krita/OpenRaster
  composites, tone-mapped EXR/HDR previews.
- Offline rule tagger with PBR set grouping, license detection and categories; optional LLM tagger for any
  OpenAI-compatible endpoint; optional Blender conversion to GLB derivatives.
- Dependency links (glTF, OBJ/MTL, FBX, DAE, `.blend` textures and libraries, `Model@Clip` animations) and on-the-fly
  zip bundles; whole-pack zips.
- MCP endpoint (`/mcp`), dual-era: 2026-07-28 stateless and `initialize`-based clients.
- `hoard` CLI: search, upload, packs, `init`, and `sync` with `hoard.toml`/`hoard.lock`, BLAKE3 verification,
  pruning and dependency bundles.
- Vue 3 portal: drag-and-drop packs, search with facets, detail panel with audio waveform and three.js model viewer,
  tag editing, linked assets, bundle downloads, packs view.
- Deployment: Dockerfile, compose files for private Cloudflare One access, public tunnel, and VPS + NAS over Tailscale;
  Garage config; CI for fmt, clippy, tests and the web build.

[Unreleased]: https://github.com/OWNER/hoard/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/OWNER/hoard/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/OWNER/hoard/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/OWNER/hoard/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/OWNER/hoard/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/OWNER/hoard/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/OWNER/hoard/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/OWNER/hoard/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/OWNER/hoard/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/OWNER/hoard/releases/tag/v0.1.0
