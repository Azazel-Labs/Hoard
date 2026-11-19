# Hoard

[![CI](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Azazel-Labs/Hoard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Azazel-Labs/Hoard)](https://github.com/Azazel-Labs/Hoard/releases/latest)
[![Rust 1.100+](https://img.shields.io/badge/rust-1.100%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-blue)](LICENSE)

<!-- languages:start -->
English · [Afrikaans](docs/i18n/README.af.md) · [Català](docs/i18n/README.ca.md) · [Čeština](docs/i18n/README.cs.md) · [Dansk](docs/i18n/README.da.md) · [Deutsch](docs/i18n/README.de.md) · [Ελληνικά](docs/i18n/README.el.md) · [Español (Latinoamérica)](docs/i18n/README.es-419.md) · [Español (España)](docs/i18n/README.es-ES.md) · [Suomi](docs/i18n/README.fi.md) · [Français](docs/i18n/README.fr.md) · [Magyar](docs/i18n/README.hu.md) · [Bahasa Indonesia](docs/i18n/README.id.md) · [Italiano](docs/i18n/README.it.md) · [日本語](docs/i18n/README.ja.md) · [한국어](docs/i18n/README.ko.md) · [Norsk bokmål](docs/i18n/README.nb.md) · [Nederlands](docs/i18n/README.nl.md) · [Polski](docs/i18n/README.pl.md) · [Português (Brasil)](docs/i18n/README.pt-BR.md) · [Português (Portugal)](docs/i18n/README.pt-PT.md) · [Română](docs/i18n/README.ro.md) · [Русский](docs/i18n/README.ru.md) · [Српски](docs/i18n/README.sr-Cyrl.md) · [Svenska](docs/i18n/README.sv.md) · [ไทย](docs/i18n/README.th.md) · [Türkçe](docs/i18n/README.tr.md) · [Українська](docs/i18n/README.uk.md) · [Tiếng Việt](docs/i18n/README.vi.md) · [简体中文](docs/i18n/README.zh-Hans.md) · [繁體中文](docs/i18n/README.zh-Hant.md)
<!-- languages:end -->

<h3 align="center">Every asset pack you own, searchable, previewable and one click from your game.</h3>

Hoard is a self-hosted library for game assets. Drop in a zip, a Unity package, a Fab download or a folder of ten
thousand files, and Hoard gets to work. Models turn in 3D with their textures, loose texture maps become materials,
sounds draw their waveforms, sprite sheets play their animations, and everything gets tagged, so typing
`footsteps gravel` or `rigged low-poly` finds it. Then it lands in Godot, Unity or Unreal with everything it needs.

No subscription, no upload to anyone's cloud, no per-seat pricing: one Docker image on your own server, or an app on
your own computer, and it's yours. Think of it as an asset store with no checkout.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/img/hero-dark.webp">
  <img alt="The Hoard library: 3D models in a grid, with a treasure chest open in the 3D preview beside its tags and downloads" src="docs/img/hero-light.webp">
</picture>

### See everything before you use it

Models open in 3D with their textures, materials and animations. FBX, glTF, OBJ, USD, SketchUp, 3MF, STL and
voxels are read by Hoard itself, with no 3D app installed; Blender files come in through headless Blender, and
Unreal packages show what the editor saved about them, without Unreal. Thumbnails are drawn on the server, framed
the same way every time, so a whole pack can be skimmed at a glance.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/img/page-dark.webp">
  <img alt="A chandelier shown full page: a large 3D view beside its facts, tags, linked textures and downloads" src="docs/img/page-light.webp">
</picture>

### Loose texture maps become materials

`Rock_BaseColor.png`, `Rock_Normal.png` and `Rock_Roughness.png` become one Rock material on a lit sphere. Hoard
reads the naming of Megascans, Poly Haven, ambientCG, Substance and Unreal, packed maps included, and Unity `.mat`
files too. The full-screen viewer shows a material on a sphere, cube or plane, at its real-world scale, with each
map switched on and off.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/img/material-dark.webp">
  <img alt="A brick material in the full-screen viewer, on a sphere, with switches for its color, normal, roughness and height maps" src="docs/img/material-light.webp">
</picture>

### Sprite sheets that play

Atlases from TexturePacker, Starling, libGDX, Cocos2d, Godot and DragonBones, Tiled tilesets, Unity images cut into sprites, and tile sheets that come with only a
note of their grid, become one sprite sheet that plays its animations and lists its sprites. Download it repacked for whichever engine you're using: ten formats, each told only what it
can hold.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/img/sheet-dark.webp">
  <img alt="A character sprite sheet shown full page, one of its animations playing beside its tags and download formats" src="docs/img/sheet-light.webp">
</picture>

### Sounds you can see

Every sound gets a waveform colored by how bright it sounds, so a sharp hit and a long ambience look different
before you play them. Sound library metadata is read (UCS categories, Soundminer and BaseHead fields, loop points,
tempo, root note), MIDI, tracker music, chiptunes and game audio banks (FMOD, Wwise) are rendered so they play like any
other sound, and instruments (SFZ, SF2) play a few notes.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/img/sound-dark.webp">
  <img alt="A grid of sound effects drawn as colored waveforms, with one open showing its player, tags and takes" src="docs/img/sound-light.webp">
</picture>

### Search that knows what things are

Hoard tags everything itself, offline: `tileable`, `res:2k`, `poly:low`, `rigged`, `loop`, `bpm:120`,
`license:cc0`, and a category from a game-asset tree. Search by words, by color or by what a picture shows, and
ask any texture or sound for "More like this". Point it at any OpenAI-compatible model, local or hosted, and it
writes names and descriptions too.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/img/search-dark.webp">
  <img alt="A search for wood: a wooden table, bark, planks and a wood surface together, with filters by kind and tag beside them" src="docs/img/search-light.webp">
</picture>

### Straight into your engine

- **Engine plugins** for Godot 4, Unity, Unreal Engine 5 and Lumina: browse the library inside the editor and press
  Add to project. The model comes with its textures, in the right place, imported.
- **`hoard sync`** pulls exactly the files a project names into it, pinned in a lockfile, so every teammate and
  build machine gets the same bytes.
- **Download as** GLB, PNG, WAV, MP3 or a sprite sheet format, converted on the fly, with texture names in your
  studio's convention.
- **AI agents** search, inspect and fetch assets over MCP.

### Stored once, kept safe

Every file is stored once by its BLAKE3 hash, so the same texture in five bundles takes the space of one, and
corruption can't go unnoticed. Keep it on local disk or any S3 bucket. Folders you point Hoard at are read, never
changed. Accounts, read-only tokens, single sign-on, HTTPS and HTTP/3 are built in.

### Or just on your computer

Not running a server? The desktop app for Windows, macOS and Linux is the same library in a window of its own.
Install it, pick a folder, and it works offline: no accounts, passwords or server settings, nothing the rest of the
network can reach, and the keys you type in kept in the system's keychain. New versions download in the background
and install when it quits.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/img/desktop-dark.webp">
  <img alt="The Hoard desktop app in a Windows window: the library's models in a grid, with a lantern open in the 3D preview beside its tags and downloads" src="docs/img/desktop-light.webp">
</picture>

<sub>The demo library is CC0 art: models from [Poly Haven](https://polyhaven.com) (glTF and USD), surfaces from
[ambientCG](https://ambientcg.com), and sounds, sprite sheets and fonts from [Kenney](https://kenney.nl).
`node web/scripts/readme-images.mts` makes these pictures and the social card.</sub>

## Install the desktop app

Download it from the [latest release](https://github.com/Azazel-Labs/Hoard/releases/latest): the `-setup.exe` for
Windows, the `.dmg` for macOS on Apple silicon, the `.AppImage` or `.deb` for Linux. On Windows it installs for you
alone, without asking for an administrator. The installers aren't signed by Apple or Microsoft yet, so Windows asks
before running the installer (More info → Run anyway) and macOS before opening the app the first time (System Settings
→ Privacy & Security → Open Anyway).

The first time it opens, it asks where the library goes. Everything else is in Settings, as on a server.

After that it keeps itself up to date, with nothing to click: a new release downloads in the background and installs
when you quit, so the next time you open Hoard it's the new version. A `.deb` is updated by installing the next one.

## Install with Docker

Hoard ships as one image for amd64 and arm64: `ghcr.io/azazel-labs/hoard`. Save this as `compose.yml` (it's
[deploy/compose.yml](deploy/compose.yml)):

```yaml
services:
  hoard:
    container_name: hoard
    image: ghcr.io/azazel-labs/hoard:latest
    restart: unless-stopped
    volumes:
      - hoard-config:/config      # database, Settings, backups: small, back it up
      - hoard-library:/library    # your files: big, put it on the disk with room
      - hoard-cache:/cache        # thumbnails, uploads in progress, Blender/ffmpeg: fast disk, can be rebuilt
    ports:
      - "4627:4627"

volumes:
  hoard-config:
  hoard-library:
  hoard-cache:
```

Each volume can be a folder on whichever disk suits it instead: `/mnt/big/hoard:/library` puts your files on the big
disk, while the database stays small and easy to back up. Folders you mount need no chown: Hoard takes over
`/config` and `/cache` and runs as whoever owns `/library` (set `PUID` and `PGID` to pick a user yourself).

Then start it:

```sh
docker compose up -d        # open http://<host>:4627
```

The first visit asks you to make an account, then opens a short setup that asks where your library lives. It suggests `/library`, the volume above; pick
it and drop a pack onto the portal. More locations come later, from **Settings → Library locations**:

- **A library Hoard keeps** gets copies of what you upload or import. Each one ticks the kinds of files it takes
  (audio on one disk, models on another), and one is the catch-all for the rest. Changing what a location takes
  applies to new files; **Move stored files to match** moves what's already stored. **Remove** moves a location's
  files to the others first, by kind, and removes it once it's empty. Each file is checked against its hash before
  the old copy is deleted. A location whose disk is gone can be removed anyway, and its files then show as missing.
- **A watched folder** is a folder of packs you already have. Each folder inside it becomes a pack, indexed where it
  sits. Hoard never changes, moves or deletes anything there. Mount it read-only if you like. Hoard looks through it
  at startup and every 15 minutes after (or when you press **Scan now**), picking up new folders, files added to or
  removed from a pack, files edited where they sit (a size or modified time that changed, then contents that hash
  differently: each is read again, keeping its tags and notes, and its thumbnail and previews are made again), and
  packs whose folder is gone. Most rescans only check each pack's folders for a changed modified time, so a big
  library on a slow share isn't read file by file every time; a file edited where it sits is noticed at startup, on
  **Scan now** or by the once-a-day look at every file. A folder that's suddenly empty or unreachable is taken for a
  disk that isn't mounted, and its packs are kept. A pack whose folder is missing is kept for three days, with its
  tags and notes, in case the folder comes back. A folder renamed keeps its pack, found by the files it holds. Anything you delete for good from a watched pack stays deleted when
  the folder is rescanned, even though its files are still there.

A location is any folder the server can see, so in Docker, mount it into the container first.

**Updating.** The header shows a badge when a newer release exists. Run `docker compose pull && docker compose up -d`
to take it; Hoard upgrades its database on start, saving a copy in `/config/backups` first. Change the image tag to
`:1.2` to get patch releases only. To update automatically without restarting Hoard mid-upload, give
**Settings → Maintenance → Updates** an address to call: Hoard calls it once a release is out and nothing is going on
(Watchtower's HTTP API, a Portainer webhook, a script; see [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md#updating)).

**GPU (optional).** Hoard renders 3D model thumbnails on the GPU when it can see one, and in software when it can't.
The packs page shows which (`GPU: ...` or `No GPU found`). To pass the GPU through, add the overlay for your
hardware:

```sh
# NVIDIA: needs the NVIDIA driver and the NVIDIA Container Toolkit on the host
docker compose -f compose.yml -f compose.gpu-nvidia.yml up -d

# Intel or AMD, including integrated graphics: needs /dev/dri on the host
echo "RENDER_GID=$(getent group render | cut -d: -f3)" >> .env
docker compose -f compose.yml -f compose.gpu-dri.yml up -d
```

Get both overlays from [deploy/](deploy/). They also put the local model server (AI tagging) on the GPU. Files on a NAS, S3 storage, remote access through Cloudflare
or Tailscale, and backups are covered in [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md).

## Build from source

```sh
# Server (stable Rust via rust-toolchain.toml, MSRV 1.100; rustup installs it automatically)
cargo build --release
(cd web && npm ci && npm run build)
./target/release/hoardd                      # http://127.0.0.1:4627

# Try it with a generated sample pack (textures, PBR set, sprites, audio, glTF/OBJ/GLB models, CC0 license)
python3 tools/make_sample_pack.py /tmp/sample       # needs Pillow + numpy; ffmpeg optional
./target/release/hoard --token <token> upload /tmp/sample/forest-starter-kit.zip   # make the token in Settings → Account
```

Or drop the zip onto the portal.

### Development

Open the folder in VS Code and accept the recommended extensions (rust-analyzer, CodeLLDB, Vue, TOML, YAML).
Ctrl+Shift+B starts the dev servers; the Run and Debug panel has configurations for `hoardd`, `hoardd` with a local
model, unit tests, the CLI, `xtask`, the portal in the browser, and a "full stack" compound. Everything is also a
`cargo xtask` command, so the terminal and the editor do the same thing:

```sh
cargo xtask dev              # hoardd + Vite with hot reload on http://127.0.0.1:5173; Ctrl-C stops both
cargo xtask dev --llm        # ...plus a local model server for AI tagging (pick a model in Settings)
cargo xtask ci               # fmt, clippy -D warnings, tests, web build: what CI runs
cargo xtask sample --upload  # generate the sample pack and upload it
cargo xtask web              # build the portal into web/dist
```

Releases are automatic and change no files. Each push to `main` that passes CI is tagged with the next patch version
(`#minor` or `#major` in a commit message bumps that instead; `#skip-release` only refreshes the `:edge` image), and
the tag builds the binaries, the container image and the GitHub release. Builds take their version from the tag.

### Tests

`cargo test --workspace` runs the Rust tests. The portal has an end-to-end suite in [web/e2e/](web/e2e/) (Playwright,
Chromium): it starts its own `hoardd` on a free port with temporary data and library folders, makes the account and a
location, uploads packs it generates in code (PNGs, an OBJ with its material and texture, a glTF, JSON, text, a zip
and a 300-image pack), and drives the real portal: sign-in, the grid's infinite scroll and keyboard, the side panel,
the full-screen viewers, the packs finder, tag merging, collections, access tokens and copying, and phone-width and
dark-mode passes. It needs no network.

```sh
cargo build -p hoardd
cd web && npm ci && npm run build
npm run e2e:install          # once: Playwright's Chromium
npm run e2e                  # or: npx playwright test e2e/viewers.spec.ts --headed
```

It uses the newest `hoardd` under `target/` (debug, then release); `HOARD_BIN` points it at another, and
`HOARD_WEB_DIR` at another portal build. Failures keep a trace, a screenshot and the server's log in
`web/test-results/` (`npx playwright show-trace <trace.zip>`).

`e2e/engines.spec.ts` runs the engine plugins against that server: each adds a model to a scratch project through
the plugin's own code, the files, `hoard.toml`, `hoard.lock` and what the server was told are checked, and then the
real `hoard sync --verify` must find everything in place, downloading nothing and writing the same lock. The Unity
package's code runs on .NET whenever `dotnet` is installed (so in CI); Godot's runs headless when `GODOT_BIN` names a
Godot 4 executable; Unreal's runs as a commandlet (`-run=HoardAdd`) when `UNREAL_EDITOR` names `UnrealEditor-Cmd`,
which also checks the model was imported. The Unreal plugin is built with that engine's UAT the first time, and again
whenever its sources change. Lumina's core (everything but drawing and importing) is built into a small program with
the C++ compiler (`CXX`, or `c++`; `clang++` on Windows) against libcurl or WinHTTP, so it runs in CI too. All use the hoard CLI under `target/` (`cargo build -p hoard`), or `HOARD_CLI`.

### Local models for the tagger

The AI tagger works with any OpenAI-compatible endpoint. The simplest is a local model server: every compose file in
[deploy/](deploy/) has an `llm` service (llama.cpp's server in router mode), and `cargo xtask llm` runs the same
thing on your machine. It starts with no model. In the portal, **Settings → AI tagging → Local model** lists these
vision models (they look at the contact sheets), all of which run on a laptop CPU, and downloads, switches and deletes
them with a click:

| Model | Hugging Face reference | Download | License | Notes |
|---|---|---|---|---|
| Qwen3.5 2B | `unsloth/Qwen3.5-2B-GGUF:Q4_K_M` | ~1.9 GB | Apache-2.0 | Good at the answer format; about 3 GB of RAM |
| Qwen3.5 4B | `unsloth/Qwen3.5-4B-GGUF:Q4_K_M` | ~3.4 GB | Apache-2.0 | Better names and descriptions; wants 6 GB of RAM |
| Qwen3-VL 2B | `Qwen/Qwen3-VL-2B-Instruct-GGUF:Q4_K_M` | ~1.9 GB | Apache-2.0 | Qwen's own GGUF build of the previous generation |
| Gemma 4 E2B | `unsloth/gemma-4-E2B-it-GGUF:Q4_K_M` | ~3.5 GB | Apache-2.0 | Google's edge model; strong captions |
| LFM2.5-VL 1.6B | `LiquidAI/LFM2.5-VL-1.6B-GGUF:Q4_0` | ~1.3 GB | LFM Open License 1.0 | Smallest and fastest; check the license for commercial use |

Any other GGUF model on Hugging Face can be downloaded the same way by its `owner/name:quant` reference; the vision
projector comes along with it. Thinking models run with reasoning off, since tagging wants a quick JSON answer.
`cargo xtask llm` needs llama.cpp (`curl -LsSf https://llama.app/install.sh | sh`, `brew install llama.cpp` or
`winget install llama.cpp`) and keeps models in `models/`. A hosted model (OpenAI, OpenRouter, …), your own Ollama,
or a ChatGPT plan through "Sign in with ChatGPT" is picked in the same place. `deploy/compose.yml` has ready-to-uncomment Ollama and vLLM services, and
[docs/DEPLOYMENT.md](docs/DEPLOYMENT.md#ai-tagging) compares them.

### Bulk imports

Imports of many gigabytes work from the browser and the CLI. "Import folder" in the portal (or dropping a folder)
uploads every file in it; a folder that looks like a folder *of* packs can be split into one pack per subfolder.
Uploads go through resumable sessions: the file list is registered first, small files travel in batches and big
ones in 32 MB chunks, and a dropped connection or closed tab picks up where it stopped (drop the same folder again).
The server fingerprints each import (paths and sizes), so re-running a bulk import skips packs it already has
("Import again anyway" or `hoard upload --force` overrides that).

```sh
hoard upload ~/Downloads/Humble-Bundle-2026/ --each    # one pack per subfolder or archive, resumable
```

Model thumbnails need a Vulkan/Metal/DX12/GL adapter. On a headless Linux box without a GPU, install Mesa's
software rasterizer (`apt install mesa-vulkan-drivers`). Without any adapter, everything else still works.

## What happens to a pack

1. **Unpack.** Archives are expanded server-side (zip, tar, tar.gz, and Unity's `.unitypackage`; nested ones too,
   3 levels deep), with `..`/absolute-path rejection and limits on expanded size and file count. A `.unitypackage`
   is rebuilt into its original `Assets/…` tree. `__MACOSX` and `.DS_Store` are dropped; Unity `.meta` files are read
   for their GUIDs and a model's material slots (so materials, prefabs and models link to theirs) and then dropped,
   along with Godot `.import` files. Wrapper folders are collapsed and the innermost folder's name becomes the pack
   name. A `.gltf` becomes a `.glb` with its `.bin` packed in when nothing is lost (no other glTF reads that `.bin`),
   meshopt compression and all; its textures stay files beside it. Watched folders are never changed.
2. **Store.** Each file's BLAKE3 hash is its name in blob storage, so a file shared by ten packs is stored once,
   corruption is detectable, and blobs never change (cache them forever, back them up with any sync tool).
3. **Analyze.**
   - Images: dimensions, alpha, a thumbnail, dominant color, seamless tiling on each axis (edge-continuity test),
     pixel-art detection (few colors, small, upscaled crisply), tangent-space normal map detection. Images over
     2048 px, and any format browsers can't draw (TGA, DDS, EXR, PSD…), also get a light preview of at most 2048 px
     (JPEG, or lossless WebP when there's transparency) for the image and 3D viewers.
   - AI provenance: an image that says it was made or edited with generative AI is tagged `ai-generated` or
     `ai-edited`, with what made it where the file names it. Read from C2PA Content Credentials (OpenAI, Adobe
     Firefly, Google and others sign their images), the IPTC digital source type in XMP (Google, Midjourney, Adobe,
     Meta), and the settings Stable Diffusion front ends keep in PNGs (AUTOMATIC1111, Forge, ComfyUI, InvokeAI,
     NovelAI, Fooocus). It's what the file declares: metadata can be stripped, and invisible watermarks such as
     Google's SynthID can only be read by their maker's detector.
   - GPU textures, decoded in pure Rust: DDS (BC1-7, raw RGBA/BGRA, float, legacy DXT and bitmask formats) and KTX2
     (BCn, ETC2/EAC, ASTC, raw, with zstd or zlib supercompression), with their GPU format and mip count, and so
     are KTX 1, PowerVR (`.pvr`, with PVRTC and ETC1 too) and Valve's VTF. Basis Universal (KTX2 with BasisLZ/ETC1S
     or UASTC, and `.basis` files) is transcoded in pure Rust too. Aseprite files show their first frame with frame,
     layer and animation-tag counts. Icons and cursors (ICO, CUR), QOI, PNM (`.ppm`, `.pgm`, `.pbm`) and PCX are
     pictures too, and so is JPEG XL, decoded in pure Rust. AVIF and HEIC (iPhone photos) are read by ffmpeg once
     it's ready.
   - Vector art (SVG, Illustrator, Affinity Designer, EPS) is a kind of its own, filtered apart from pictures made of
     pixels, and goes wherever a library location takes textures. SVGs render with their text (system fonts, DejaVu
     in the Docker image). Illustrator files (`.ai`) render their first artboard from the PDF inside them, or show
     the preview they carry when they have none (Illustrator 8 and earlier, or saved without PDF content).
     Lottie animations (Bodymovin's `.json`, and dotLottie's `.lottie`) are vector art too, with their size, frame
     rate, length and layer count, and play in their details, images and all; the browser draws their thumbnails.
     So are Rive files (`.riv`), with their first artboard's name and size; they play in their details with Rive's
     web runtime, any artboard's state machine or animation picked from a list, and answer pointing and clicking as
     they would in an app. Images or fonts a file leaves on Rive's servers aren't fetched.
   - Pictures that move: an animated GIF, PNG or WebP says how many frames it has and how long it plays (`frames`,
     `duration_ms`), and plays in its details (up to 16 MB) and on its tile while the pointer is over it (up to
     4 MB). An Aseprite file of more than one frame plays in its details too, at each frame's own time and through
     any of its tags (forward, reverse or ping-pong), from a sheet of its frames made on first request. A sprite
     sheet tagged `spritesheet` or `frames` plays when asked, cut into the frame size its name gives (`coin_16x16.png`)
     or where the see-through gutters between its frames are, skipping empty cells; the columns, rows and speed can be
     changed. With reduced motion asked for, they wait to be played.
   - DragonBones skeletons (`_ske.json`) are animations, with their armature, bones, slots, meshes, animations, frame
     rate and size. Each is linked to its atlas (`_tex.json` and its image), so they're downloaded together, and plays
     in its details on that atlas, meshes, IK and the armatures nested in it included: any armature's animation is
     picked from a list, and paused it steps frame by frame. The browser draws its thumbnail.
   - Spine skeletons (`.json`, from Spine 3.8 on) are animations too, with their Spine version, bones, slots, skins,
     meshes, animations, frame rate and size. Each is linked to its libGDX atlas (`hero.atlas`, or the one atlas in its
     folder), so they're downloaded together, and plays in its details on that atlas with Hoard's own player, written
     from Spine's published format: meshes, weights, IK and transform constraints, clipping, deform keys, draw order
     and blend modes included, each skin picked from the list with its animations. The browser draws its thumbnail.
   - Sprite sheets (Starling/Sparrow XML, TexturePacker JSON including multipack, libGDX `.atlas`, Cocos2d `.plist`,
     DragonBones texture atlases (`_tex.json`),
     Godot `.tres` `SpriteFrames` and `TileSet` resources, and Tiled tilesets, `.tsx` or JSON, their empty tiles left
     out and their tiles' animations kept, and the `.meta` of a Unity image cut into sprites, which the pack keeps
     when other `.meta` files are left out): the atlas file becomes a sprite sheet,
     kept as a Hoard sheet (`.hoardsheet`, JSON) that names its pages, where each sprite is on them, how it's turned
     and trimmed, and its animations (the file's own, or numbered names such as `walk_0`, `walk_1`). The pages stay
     textures in the pack, hidden with the sheet, and the sprites aren't assets of their own. Loose images in the pack
     with a sprite's name and picture (Kenney's `PNG/` folders beside each sheet) are linked from the sheet and hidden
     with it too, listed in its details under "Also as separate files". A tile sheet that comes with only a note of
     its grid (Kenney's `Tilesheet.txt` or `Instructions.txt`: "Tile size 16px × 16px, space between tiles 1px") is a
     sprite sheet too: the note becomes the sheet, every image beside or below it on that grid is a page (with the
     space between tiles, packed without it, or twice the size), its cells are its sprites (`tile_0000`, empty cells
     left out), and the kit's loose tiles are kept with it by name or, when they're named otherwise, by picture. A
     note that doesn't count its tiles only takes images in a folder of sheets or named as one. A sheet's details
     show its animations playing
     and its sprites, which a name narrows down, each drawn upright on its whole canvas. It downloads as a Hoard
     sheet, TexturePacker JSON (hash or array), Starling XML, libGDX `.atlas`, Cocos2d `.plist`, Godot
     `SpriteFrames` or `TileSet`, a grid image or a PNG per sprite, each a zip with its images. A format gets what it
     can say: sprites stay turned only in a format that turns them the same way (TexturePacker, Starling and Cocos2d
     clockwise, libGDX counter-clockwise, a Hoard sheet any right angle), and are otherwise packed again upright;
     trims stay only where the format keeps them, and animations only where it has them.
   - Fonts (TTF, OTF, TTC, WOFF, WOFF2): family and style (used as the name), designer, license, weight, italic, monospace,
     variable axes, glyph count and the writing systems covered, with an "Aa" specimen as the thumbnail. Symbol
     fonts show their first glyphs. Tagged `weight:bold`, `script:cyrillic`, `monospace`, `variable-font`, `icons`.
   - Audio: duration, sample rate, channels, embedded title/artist/genre, peak and RMS level (and clipping), plus a
     waveform thumbnail so a sharp hit and a sustained loop look different in the grid: mirrored bars with a solid
     core at the loudness and a halo to the peak, colored by how bright each moment sounds (azure bass, violet and
     orchid mids, orange hiss and cymbals). Formats Hoard doesn't decode
     itself (WMA, CAF, AAC/M4A, WavPack, APE, …) are decoded through ffmpeg (see below) for the same treatment.
   - Music: tracker modules (MOD, XM, S3M, IT, OpenMPT's MPTM, and the module inside an Unreal UMX package), MIDI and
     chiptunes (NES NSF and NSFe, VGM and VGZ) are rendered to audio, so they get a waveform, a duration and an MP3
     preview that plays in the grid like any other sound. Modules list their title, channels, patterns and
     instruments; MIDI lists tempo, key, tracks and General MIDI instruments. MIDI plays through a SoundFont
     (the one picked in Settings, or a GM SoundFont found in the usual system places), falling back to a simple built-in synth.
   - Chiptunes: an NSF runs the game's own music code on an emulated NES, cartridge sound chips included, and plays
     its first song (for two and a half minutes when the file doesn't say how long). A VGM replays the writes a game
     made to its sound chips: Yamaha's FM chips (Mega Drive, arcade boards, MSX, PC-88/98, AdLib and Sound Blaster
     cards), the SN76489 and the AY-3-8910. Other chips are listed but stay silent. Both list the title, game,
     composer and the chips used.
   - Game audio banks: FMOD sound banks (`.fsb`, and the FSB inside an FMOD Studio `.bank`), Wwise sounds (`.wem`) and
     Wwise sound banks (`.bnk`) play their sounds one after another, with a short gap between them, for the waveform
     and preview, and list the codec, how many sounds there are and, for FMOD, their names. PCM, Vorbis, MP3 and
     FMOD's ADPCM play from FMOD banks; PCM, Vorbis and Wwise's two ADPCM codecs from Wwise. Console codecs (XMA,
     ATRAC9, DSP and the like), Wwise's Opus and encrypted banks are named but don't play, and a Wwise bank that only
     holds events (its sounds streamed from `.wem` files) says so.
   - Instruments: an SFZ (a text file mapping samples to keys) or an SF2 SoundFont plays a rising arpeggio and a chord
     (a beat, for a drum kit), for its waveform and preview, and lists the keys it covers, its regions or presets and
     its velocity layers. An SFZ's samples are linked to it, so they're downloaded with it and stay out of the grid.
   - Metadata inside WAVs, as sound libraries ship it: BWF `bext` descriptions, `LIST/INFO` titles and keywords, iXML
     from Soundminer/BaseHead with Universal Category System (UCS) category, subcategory and CatID, `smpl` loop
     points, ACID tempo, root note and one-shot flag, cue markers, and RF64 for files over 4 GB. Embedded titles
     become the asset's name, and embedded descriptions show as "From the file".
   - Models: glTF/GLB mesh, material, animation and skin counts, vertex/triangle counts; OBJ counts; FBX version
     plus skin and animation detection; a wgpu-rendered thumbnail for glTF/GLB/OBJ/FBX, and for STL, PLY, 3DS and
     MagicaVoxel `.vox`, which Hoard reads itself (voxels keep their palette colors, and STL is stood up from the Z up
     of 3D printing). Thumbnails are drawn the way the 3D viewer shows the model: textures from the file, an OBJ's
     MTL, or found in the pack by name; tone mapping; cut-outs and see-through surfaces, glass included (glTF's
     transmission, where the metal parts of the same material stay solid); framed tightly from the same angle every
     time. glTF and GLB compressed with meshopt
     (`EXT_meshopt_compression`, `KHR_meshopt_compression`) are decoded for the thumbnail and the 3D viewer. The 3D
     viewer is handed every model as a plain GLB Hoard makes of it: the whole scene (hierarchy, skins, morph targets,
     animation, materials) with any compression decoded, and its textures as JPEG, or PNG where they're see-through,
     at most 2048 px a side, so the browser reads nothing but GLB. A model only Blender reads shows from Blender's GLB
     of it. Texture files a model names that its pack lacks are named under the viewer. FBX
     geometry, node transforms, materials and embedded or nearby textures are read natively (axis and unit settings
     become glTF's Y-up meters), from binary FBX and from the text form (ASCII FBX 7) that Asset Forge and some exporters write.
     3DS meshes, materials, smoothing groups and
     textures are read natively too, since Blender stopped shipping its 3DS importer in 4.2.
   - 3D-print files (`.3mf`): meshes, components and build plates, read natively, with their colors: the 3MF
     materials extension's base materials and color groups, and from Bambu Studio, OrcaSlicer and PrusaSlicer
     projects the filament colors, which filament each object and part prints in, and filaments painted onto the
     model. Facts: triangles, objects, colors and the application that wrote it. The 3D viewer shows the GLB Hoard
     makes from it, so the colors match the thumbnail.
   - USD (`.usd`, `.usda`, `.usdc`, `.usdz`): meshes, instances, transforms, units and up axis, read natively, with
     their UsdPreviewSurface materials and textures (from beside the file or inside the `.usdz`), or their display
     colors. Guides, proxies and hidden meshes are left out; a file of materials alone has no thumbnail. The 3D viewer
     shows the GLB Hoard makes from it.
   - SketchUp (`.skp`), from SketchUp 3 through today's: faces, groups and components, layers and tags (hidden ones
     left out), colors and textures, read natively by polysmelt's port of the MIT-licensed
     [openskp](https://github.com/iamahsanmehmood/openskp) reader, since SketchUp's SDK has no Linux build. Edges without a face, guides, text and dimensions aren't drawn. The thumbnail, the 3D viewer and
     "download as GLB" all use the GLB Hoard makes from it, in meters with Y up.
   - Unreal packages (`.uasset`, `.umap`), read without Unreal: see [Unreal packs](#unreal-packs).
   - Source files:
     - `.blend`: Blender version, compression, and the preview Blender embeds in the file (read directly, gzip
       or zstd, legacy and 5.0 headers, no Blender needed). A preview that is a picture of the Blender window isn't
       used; with Blender switched on, the thumbnail is drawn from the converted model instead.
     - `.psd`/`.psb`: size, bit depth, color mode, layer count, and Photoshop's embedded preview.
     - `.kra`/`.ora`: the flattened image and layer count.
     - `.xcf` (GIMP): the layers composited by Hoard, each with its offset, opacity, mask and visibility, and the
       layer count.
     - `.exr`/`.hdr`: tone-mapped previews.
     - Clip Studio, Procreate, Affinity Photo and Designer, and EPS: the picture of itself the file keeps inside
       (Clip Studio's canvas preview, Procreate's thumbnail, the PNG among Affinity's data, the TIFF preview an EPS
       names in its header).
     - Camera raw photos (DNG, Canon, Nikon, Sony, Fujifilm, Pentax, Olympus, Panasonic, Samsung): the largest JPEG
       the camera kept inside, turned the way the camera was held, with the camera's name.
     - Recognized by kind, without previews yet: Substance, ZBrush, Maya/Max, Houdini, C4D, DAW projects, and video.
   - Text: code, shaders, scripts, JSON and other data, configs, markup and prose are drawn as a silhouette of their
     lines, each a bar from the left edge as indented and as long as the line, colored by what it's made of
     (keywords, strings, numbers, comments, punctuation, keys and tags), so a file reads like a speckled gene
     sequence: its shape shows its indentation and density, its colors what kind of text it is. PDFs are shown by
     their first page. Files added before Hoard drew these get theirs the next time it starts.
   - One asset in several formats: a model, sound or font the pack has more than once under the same name, beside
     itself (`house.skp` and `house.obj`) or in folders named after each format (Kenney's `Models/FBX format/` and
     `Models/OBJ format/`), is shown once, in the format Hoard reads best or that loses least (GLB, then glTF, FBX,
     OBJ…; FLAC and WAV before OGG and MP3; OTF before TTF and WOFF). The others are hidden with it, listed in its
     details under "Other formats", and downloadable as the pack's own files from its "Download as".
4. **Tag (offline).** A deterministic tagger tokenizes paths (`FootstepGrass_01` → `footstep grass`), applies a
   game-asset vocabulary (audio-only words such as "run" only apply to audio), adds measured facts (`res:2k`, `pot`,
   `tileable`, `duration:short`, `poly:low`, `rigged`), groups PBR maps into sets (`Rock_Cliff_Normal.png` →
   `map:normal`, `set:rock-cliff`), detects the pack license from LICENSE/README files (`license:cc0`, `cc-by`, …),
   and picks a category from a fixed tree (`sfx/footsteps`, `textures/pbr`, `models/props`, …). Sounds get a UCS
   category (`ucs:footsteps`, `ucs:swooshes/swish`) from embedded metadata or confident rules, plus `loop`,
   `one-shot`, `bpm:120`, `root:a3`, `level:quiet`/`level:loud` and `clipped` where measured, and tags for how they
   sound: `envelope:hit`, `envelope:swell` or `envelope:sustained`, `tone:bright`/`tone:dark`,
   `tone:noisy`/`tone:tonal` and `pitch:rising`/`pitch:falling`. A sound's genre
   and artist are tags too (`genre:ambient`, `artist:kenney`), and clicking either in the details shows everything
   with it. Words in brackets in a folder or file name are tags: everything in `Audio (Female)/` is tagged `female`
   (a copy number like `(1)`, a year or a whole sentence isn't).
   - **Variants.** Numbered takes (`whoosh_01..12`, `Hit (3)`, `rock_v2`) in one folder form a family. The detail
     panel lists the other takes, and tag or category edits can apply to all of them at once.
   - **Duplicates.** Byte-identical files across packs (the same CC0 texture in five bundles) are stored once and
     listed under "Identical copies".
   - **Near copies.** The same picture at another size or in another format (`rock_4k.png`, `rock_2k.jpg`), the same
     sound at another sample rate or in another format, and the same model in another format (an FBX and a GLB with
     the same triangles and animations, whose renders look alike) are listed under "Near copies". The best of them is
     marked: the biggest, then lossless, then the most complete (a model with its textures and clips), so it's clear
     which to download. Settings → Near copies lists every set in the library, the most space taken by the copies
     that aren't the best first, so a bundle bought twice shows up. It only points them out: nothing is deleted, and
     watched folders are never touched. A recolored texture or another take of a sound isn't a copy; it can be a
     variant.
   - **Megascans files.** The `<id>.json` that comes with each Megascans asset names the pack (unless someone typed
     its name), adds its tags (`wood`, `furniture`, `color:brown`, …) to the pack and the asset, and gives the pack a
     short description: what it is, where it was scanned, its size, its maps and its levels of detail ("LOD0 to LOD5,
     24,000 to 380 triangles"). Its list of maps says what each map is, which beats the file name (a normal map per
     LOD is still one material), its triangle counts fill in a model's when the file doesn't give one, and a
     surface's real size and height go with its material (see the "Real" repeat below). The file goes with the
     whole pack's download, not with a model's or a material's. More → "Read the pack's files again" reads them again,
     for example after the description was deleted.
   - **Sketchfab files.** The `license.txt` in a Sketchfab download names the model, its author, its license and its
     page, which becomes the pack's store page without a search. Each store's file is described in TOML, not code, so
     reading another store's takes one file and a sample ([docs/store-formats.md](docs/store-formats.md)). PRs for
     more stores are welcome.
   - **Teaching it a store's file.** Settings → Store files lists files that look like a store's but aren't read. Its
     wizard shows one beside a form: pick a field, click its value in the file, and it writes the format, showing what
     it reads there and in the library's other files like it. "Use it here" saves it and reads those packs again;
     "Share it" downloads a zip of the format and the file as its sample, laid out like this repository, to send in.
   - **Pack context.** README text is kept with the pack, and a store link inside it (Fab, itch.io, Humble Bundle,
     Unity Asset Store, Kenney, …) fills in where the pack came from. "Store details" on the packs page takes the
     store page URL, license tier and listing text. For a pack without a store link it first searches Fab, itch.io,
     Kenney and OpenGameArt and lists likely matches; "Use" fills in the form. Plain names ("Medieval Village" has
     dozens of listings) get help from the pack's files: the README's store links, seller ("Created by", "©") and
     product name ("Thank you for purchasing …"), product folders (`Content/<Product>`, `Assets/<Publisher>/<Product>`),
     the engine, and the mesh and texture counts, which are checked against Fab's technical details. A Megascans
     asset's `<id>.json` gives its exact listing title, which beats a file name like `…_Old_05_xeokah0_Raw`. Pieces of
     the name are searched too, and each match says which clues it fits. Whenever a pack has
     a store link and no text, from either place, Hoard fetches the page's title and description; the asset panel
     shows it as "From the <store> page".
   - **Store lookup.** New packs are looked up in the background, one every few seconds, newest first. When the pack
     names its listing itself (its README or Sketchfab file links to it, the listing has exactly the title in its
     Megascans file, or the listing has exactly the pack's name and is sold by the seller its README or folders name),
     that listing becomes the pack's store page, with its seller and license. Store details says it
     was found automatically, and "Not this one" takes it off. Anything less certain is kept, so Store details lists the likely
     matches at once ("Search again" searches afresh). Turn it off in Settings → Adding packs → "Look up store pages
     automatically".
5. **Link.** glTF buffers/images, OBJ → MTL → textures, FBX texture paths (absolute artist paths are matched by file
   name, the one nearest the model when several share it), DAE images, `.blend` external textures and linked
   libraries (`//` paths resolved relative to the file), animation clips, and name-based guesses for models that
   reference nothing (from the model's own part of the pack: its folder's parent and below, beside or
   above the model or in a folder named for textures, so `Models/` and `Textures/` side by side but never a 2D
   sprite of the same name, and only painted on a model whose materials have no colors of their own). A file of
   clips with no mesh, or a copy of a character saved for one clip (`run.blend` holding "Run"), becomes an animation
   linked to the characters it fits, those whose bones it moves in its part of the pack, where a Unity package's
   copy of a character gets the copies of the clips beside it; so does `Model@Clip.fbx`. Across the library, a
   character's details list the "Animations that fit" it and an animation's the "Characters it fits": those with its
   skeleton (at least ten of the bones it moves, named the same once `mixamorig:` or `Armature|` is off, and not just
   Blender's `Bone.001`), so a Mixamo clip fits every Mixamo character; its own pack's come first. An animation
   plays in the 3D viewer on a character it fits, Plays on picking any other of them. Its thumbnail is drawn halfway
   through the clip, on that character when it has no mesh of its own, and its tile shows its length. A character's
   viewer plays its own clips and, under "Animations that fit", any of those, each fetched when picked. An animation's zip can take its character along ("Include the character":
   one from another pack goes in a folder named for it), and it downloads as "Clip only, no mesh (GLB)", its skeleton
   and clips without the mesh it was saved on. A clip whose root travels (a walk, a run) plays with the character held
   in the frame over a ground grid that scrolls the other way, so it looks different from one made to play in place,
   which gets no grid. With Settings →
   Performance → Thumbnails → "Moving thumbnails for animations" on (it's off: they take space and time to make), each
   animation also gets a three-second looping WebP of it playing on its character, which its tile plays while the
   pointer is over it; turning it off deletes them. Game-engine
   and tool files link too: Unity materials,
   prefabs, scenes and controllers through their GUIDs; Tiled maps → tilesets → images;
   LDtk projects; BMFont `.fnt` → page images; sprite sheets → pages; Godot
   `.tres`/`.tscn` resources; and Unreal package imports.
6. **Tag (LLM, optional).** Any OpenAI-compatible endpoint (OpenAI, OpenRouter, Ollama, llama.cpp, vLLM, LM Studio),
   or a ChatGPT plan through "Sign in with ChatGPT", gets the pack context, paths, metadata and rule tags, and returns names, a description of one to three
   sentences (for music: mood, instruments, tempo and where it fits), extra tags, UCS subcategories and
   categories. Each variant family is sent once and the answer applies to every take. With
   thumbnails switched on in Settings, each batch also gets one numbered contact sheet of its thumbnails (one image instead of
   sixteen), and models appear as a 2x2 sheet rendered from four directions. It only adds; it never removes rule tags, never overrides anything a
   person edited, and can't flatten a specific category into a vaguer one.
   - **Descriptions.** Each kind is kept apart: a person's own (written in the asset panel or over MCP), the AI's,
     the file's embedded one and the pack's store page. Search covers all of them, and re-running the AI updates
     its description without touching yours.
7. **Convert (optional).** With Blender switched on (in Settings), the model formats Hoard can't read itself (.blend,
   Collada, Alembic and X3D) are exported to GLB by headless Blender, with their linked textures laid out beside them. The GLB is stored as a derivative that feeds
   thumbnails, the browser preview and downloads. A file Blender can't convert is listed in Issues with the Blender
   version and the step that failed (opening, importing or exporting) in Blender's own words.

Tags carry their source (`meta`, `rule`, `ai`, `human`). Human tags always win, and re-running the offline
tagger after improving the vocabulary (`POST /api/packs/{id}/retag`) replaces only `rule`/`meta` tags.

A tag is kept lower case with dashes between words, as `prefix:name` or just `name`, and either half can be in any
script: `楽器:ドラム`, `संगीत:ताल-वाद्य` and `звук:барабан` are all tags, kept with the marks written on their letters.
An accent typed as its own character is the same tag as one typed with its letter. A tag is at most 64 characters.

**Tidying tags.** Settings → Tags lists every tag with its count. Pick tags that mean the same thing and merge
them, or pick one to rename it (a rename that leaves off the prefix keeps it, so renaming `set:rock-000` to
"granite" gives `set:granite`). The old names are remembered: when a tagger comes up with one later, the asset gets
the new name instead. "Allow the old name again" undoes that.

**Renaming packs.** More → Rename on the packs page. Filters, links and collections refer to a pack by its id and
rescans match it by its folder, so only the name you see changes.

**Turning a pack's models.** OBJ, STL, PLY, 3DS and Collada files don't say which way is forward, and some don't say
which way is up, so a pack's models can come in facing away or lying down. In the 3D viewer, Facing → Turn a quarter
spins the model about its up axis and Tip a quarter tips its top toward its front; Use for this pack turns every
model in the pack that way. Their thumbnails are drawn again (ones a browser drew too) and the 3D viewer shows them
turned. Downloads stay as they were made.

**Finding packs.** The packs page searches names, stores, licenses, descriptions and tags (every word has to
match), filters by what a pack holds, its status and its store, and sorts by date, name, size or asset count.

### Materials from texture maps

Texture maps that share a name in one folder become a material: `Rock_BaseColor.png`, `Rock_Normal.png` and
`Rock_Roughness.png` are one "Rock" tile in the grid, with a lit sphere as its thumbnail. Hoard reads the naming of
Megascans, Poly Haven, ambientCG, Substance and Unreal (`T_Rock_N`), and knows base color, normal (OpenGL or
DirectX), roughness, gloss, metalness, ambient occlusion, height, bump, cavity, specular, opacity, emissive and packed
maps (ORM, ARM, MRA, RMA). The same set in several sizes (`Rock_2K_…`, `Rock_4K_…`, or `2K/` and `4K/` folders) is
one material. It takes a color map plus one more, or two other kinds of map.

A Unity material (`.mat`, text or binary) becomes one too: its textures are found through the GUIDs their `.meta` files
give them, and its color, metalness, smoothness, glow, cutout and tiling are kept as the material's look, which the
sphere thumbnail and the 3D viewers draw it with. A `.mat` with only a color (Kenney's kits) is a swatch of that
color. The `.mat` itself is hidden behind the material, and a set of maps found by name that it uses gives way to it.
An FBX, OBJ or 3DS model's material slots wear the Unity materials named after them (`Paint`, or `Crate_Paint` for
`Crate.fbx`; the nearest one when several share a name), or the ones its `.meta` puts in them, each slot drawn in its
own in the thumbnail and the 3D viewer.

An OBJ's `.mtl` colors with no maps (as SketchUp exports them) become swatches too, worn by the OBJs whose `.mtl` names
them. The same name and color in several `.mtl` files of a folder is one swatch. The `.mtl` itself shows its materials
side by side as its thumbnail.

- **In the grid**, a material searches, filters, tags and collects like any asset. Its maps are hidden; "Show" next
  to "supporting files hidden" brings them back.
- **In the detail panel**, the material turns in a live preview, with its maps listed by kind and size. Click one to
  open it. Double-click the tile (or the preview) for the full-screen viewer: sphere, cube or plane, how often it
  repeats, how far the height map pushes, each map switched off and on, the environment light and the background.
  A scanned surface whose real size is known ("2 × 2 m" in its Megascans file) starts at "Real": the sphere is 2 m
  across and the surface repeats as often as it would on something that size, and its height map pushes as far as
  the scan was deep.
- **Models** that use a material's maps, by naming them or by sharing its name (`Rock_A.fbx` next to the `Rock_…`
  maps), show the material in their panel with its maps under it by kind. On a model that names no textures, the 3D
  viewers put its maps on by kind.
  A material only one model wears (or only that model's LODs, `Rock_LOD0` … `Rock_LOD3`) is part of it: it leaves the
  grid like its maps and opens from the model's panel. A material several models share stays a tile of its own.
- **Maps are named** after their material and kind: `rock_moss_xeokah0_raw_8K_AO.jpg` shows as "Rock Moss Ambient
  occlusion", with the size only when the set comes in more than one. A name typed in the panel stays.
- **Downloading** a material gives a zip of all its maps. A model that uses one brings the maps along. Maps go out
  named by a convention, `RockMoss_AO.jpg` by default, in downloads, zips, GLB downloads, `hoard sync` and to AI
  assistants; Settings → Downloads switches to short game-engine names (`T_RockMoss_AO.jpg`), takes a pattern of your
  own with your own suffix for each kind of map and each glTF material slot, or turns it off. Endings of your own
  rename the textures that aren't maps: `_diff` → `_BC` sends `rock_diff.png` as `rock_BC.png`. The files in the
  library are never renamed, and a texture that a model or MTL in the same download names by file keeps that name so
  the model still finds it.

New packs are looked through as they come in, and packs already in the library once after an upgrade. A rescan or
"Re-run offline tagger" (packs page, More) looks again.

**Model thumbnails from the browser.** The 3D viewer sometimes finds textures the server's renderer can't. When you're
signed in and open a model, once it's shown with its textures, the viewer's picture (same angle and framing as the
server's) becomes the asset's thumbnail. A picture with every texture is never replaced by one with some missing,
and neither is replaced by the server's, including when the pack is re-analyzed. For a whole pack, More → Make
thumbnails in this browser opens each model out of sight, one at a time, with progress and Cancel. Lottie
animations, Rive files and DragonBones and Spine skeletons, which the server doesn't draw, get their thumbnails the same way.

**Big textures.** The image viewer opens a big texture as its light preview and shows both sizes (pixels and bytes).
Zooming in past the preview's pixels, Full resolution or F loads the file itself. The 3D viewers load light previews
of the textures they put on a model themselves (a material's maps, ones found by name); the full-screen viewer's
Full-res switch (H) loads those files as they are. The detail panel
shows the size of each linked file and of each download.

## Getting assets into an engine

**Bundles.** Every asset with links has a "Download with N linked files (.zip)" button, built on the fly, that keeps
the original relative layout. Unzip it and `crate.gltf` still finds `crate.bin` and `textures/crate_albedo.png`.
Whole packs download as one zip too, and so does any selection: Ctrl/Cmd-click, Shift-click or press X on tiles
(long-press on a phone). The panel beside the grid then lists what's picked, how many and how big, with a button to
take each out. "Download zip" gets them all with their dependencies, one folder per pack. "Download GLB zip" gets
every model as a GLB standing Y up and facing forward (turned the way its pack's models face), its textures as
separate files in one `textures` folder the models share, each texture once, and everything else as it is. Zips are
streamed as they're written, so even a multi-gigabyte pack starts downloading at once (a GLB zip converts its
models as it goes).

**Download as.** The detail panel's "Download as" converts on request (the first download makes and caches it, so
later ones are instant). Cached conversions are kept up to a limit (Settings → Performance → Converted downloads, 10 GB
unless you change it); past that, the least recently downloaded go first, never one handed out in the last hour. Images, models, fonts, WAV/FLAC and music rendering are Hoard's own code; MP3, OGG Vorbis
and Opus use ffmpeg's reference encoders (LAME, libvorbis, libopus), as do sources only ffmpeg reads:

| From | To |
|---|---|
| PNG, JPEG, WebP, TGA, TIFF, BMP, GIF, EXR, HDR, PSD/PSB (full composite), Krita, OpenRaster, GIMP, DDS, KTX, KTX2, PowerVR, VTF, ICO, QOI, PNM, PCX, JPEG XL, AVIF and HEIC (with ffmpeg), SVG, Illustrator, Aseprite | PNG, JPEG, WebP (lossless), TGA, TIFF, plus EXR for HDR sources; optionally scaled down to 4096…64 px, including in the original format |
| WAV, FLAC, OGG Vorbis, Opus, MP3, AIFF, and with ffmpeg also AAC/M4A, WMA, CAF, WavPack, APE, MKA | WAV (16 or 24-bit, as the source), FLAC, MP3 (VBR ~190 kbps), OGG Vorbis (q5), Opus (128 kbps) |
| MOD, XM, S3M, IT, MPTM, UMX, MIDI, NSF, VGM, FSB, FMOD and Wwise banks, WEM | the same audio formats, rendered |
| glTF, GLB | GLB + textures (zipped) |
| GLB | glTF + .bin (zipped) |
| OBJ (with MTL and textures), FBX (binary and ASCII), 3DS, STL, PLY, MagicaVoxel, 3MF (with its colors), USD, SketchUp | GLB + textures (zipped) |
| TTF/OTF | WOFF2, WOFF |
| WOFF, WOFF2 | TTF/OTF and each other |
| .blend, Collada, Alembic, X3D, once Blender converted them (Settings → Blender) | GLB + textures (zipped) |

A GLB download keeps its textures out of the GLB: the zip holds `<name>.glb` and a `textures` folder it points at.
A texture Hoard knows as a map is named by the convention in Settings → Downloads, and any other keeps the name of the
file it came from, with your own endings swapped. One that was only ever inside a model takes the name it was given
there, or else its material's and the slot it fills, by the same convention (`Wood_BaseColor.png`, `Rock_ORM.png`,
`Paint_ClearcoatNormal.png`, or `Wood_BC.png` with your own suffixes). A model without textures comes as a plain `.glb`.
Models are turned the way their pack's models face (see Facing in the 3D viewer).

`GET /api/assets/{id}/as/{format}[?max=1024]`; the asset detail lists the formats on offer. 16-bit images stay
16-bit; HDR going into an 8-bit format is tone-mapped rather than clipped.

Conversions never hold up the server. They run detached, at most two at a time: a request waits up to `?wait=`
seconds (default 20) and then gets `202` with `{"state": "converting", "progress": 0.4}` and `Retry-After`, while
the conversion carries on even if the client leaves. Requests for the same conversion share it.
`GET /api/assets/{id}/as/{format}/status` starts or checks one without downloading (the portal shows its progress
on the button).

**ffmpeg** is fetched by the server itself: set to Automatic in Settings (the default) Hoard uses a system ffmpeg that
has LAME, libvorbis and libopus, or else downloads a static build in the background into `<data dir>/tools/ffmpeg`
(Martin Riedl's builds for Linux and macOS, BtbN's for Windows), checks it against the publisher's SHA-256, and uses
it from then on. Pick a specific binary there, or turn it off. Until it's ready, or without it
(offline), MP3 and Opus use built-in pure-Rust encoders, and OGG and ffmpeg-only sources aren't offered. ffmpeg only
ever reads uploads with their format forced (no playlists or concat scripts), local files only, under a time
limit. The packs page shows its state.

**Activity.** The pulse icon in the portal's header counts what's running in the background; hover or click it for
the list (ingests with file counts, tagging, conversions, zips being streamed, the ffmpeg download), with progress
and the queued jobs behind them. `GET /api/activity` returns the same.

The portal has light, dark and "match system" themes (the switch beside it), remembered per browser.

**Responsiveness.** Analysis, conversion, rendering, zipping, ffmpeg and Blender run on their own threads at a
lower CPU priority than request handling (Linux `nice` 10, inherited by child processes), so the portal and API stay
quick while a pack ingests on every core.

**Unity.** Bundles of assets that came from a `.unitypackage` (or a Unity project folder) include a minimal `.meta`
for each file with its original GUID, so materials and prefabs still find their textures when you drop the folder
into a Unity project.

**Browsing.** Sounds play straight from the grid (the play button, or Space), with the played part shaded over the
waveform. Arrow keys move through results, Enter opens one, and with the detail panel open the arrows step through
assets for quick triage. Searches and filters live in the URL, so a search can be bookmarked or shared. Files that
only exist to support a model (glTF buffers, `.mtl` libraries) are hidden from the grid by default; they still come
along in downloads.

**`hoard sync`.** A package manager for art. In your game project:

```sh
hoard init        # writes hoard.toml
hoard sync        # downloads, verifies BLAKE3, writes hoard.lock; run again and it downloads nothing
```

```toml
server = "http://nas.local:4627"
project = "My Game"             # what the server calls this project; the folder's name if not set
out_dir = "assets/hoard"        # Godot: a folder under res://, Unity: Assets/Hoard, Bevy: assets/hoard
layout = "category"             # category | pack | flat
sidecars = true                 # <file>.hoard.json with tags, category, provenance for import scripts
prune = true                    # delete files a previous sync added that nothing selects anymore

[[select]]
query = "footstep"
kind = "audio"
dest = "sfx/footsteps"

[[select]]
tags = ["pbr"]
kind = "texture"

[[select]]
ids = ["0192f0c4-…"]            # pin one model; its textures come along (with_deps = true by default)
with_animations = true
```

Models with dependencies land in their own folder (`models/props/crate/crate.gltf`, `…/crate/textures/…`), so
relative references survive any layout. Commit `hoard.toml` and `hoard.lock`, and teammates get identical,
hash-verified files.

**Where assets are used.** Each sync tells the server what its `hoard.lock` now holds, under the project's name (a
token that can only look is enough). An asset's details list the projects that have it, with the path each put it at
and when it last synced; **Projects** in the filter sidebar shows just one project's assets; and archiving or deleting
something a project uses names the projects and asks first, since their next sync would take it out. A project
renamed or abandoned can be forgotten from the sidebar.

**Manifest API.** Engine plugins can call `POST /api/manifest` with ids and/or a search, plus
`include_deps`, and get a flat list of hashes, sizes, tags, suggested paths and immutable `/api/blobs/{hash}` URLs.

### Engine plugins

A browser for the hoard inside the editor: search, thumbnails, and **Add to project**, which downloads the picked
assets with what they need (a model's textures and buffers), places them exactly where `hoard sync` would, pins them
in `hoard.toml` as a `[[select]] ids = […]` (making the file if there's none), merges `hoard.lock` and tells the server
the project uses them. A later `hoard sync` finds everything already there and downloads nothing; teammates without
the plugin get the same files from it. A token that can only look is enough. The server's address and the token are
kept in your editor's own settings, never in the project (the address falls back to `hoard.toml`'s `server`). Files
are checked by size as they arrive; `hoard sync --verify` checks their BLAKE3 hashes.

Going the other way, the Godot, Unity and Unreal plugins send a project's own assets to Hoard as a new pack, and only what another tool can
open goes:

- **Originals:** the file an asset was imported from, if it's still there. That includes the files a model names: an
  OBJ's material libraries and the pictures they use, and a glTF's buffers and images.
- **Copies of things made in the engine:** a mesh as glTF, FBX or OBJ, and a texture as PNG.
- **Left out:** scenes, materials, prefabs, scripts and Blueprints only the engine reads (what they use still goes),
  and anything Hoard put in the project.

Sending needs a token that can add to the library (an editor's or an admin's). When it's done, the plugin links to the
new pack.

**Godot 4.** Copy [integrations/godot/addons/hoard/](integrations/godot/addons/hoard/) into your project's
`addons/`, then Project → Project Settings → Plugins → enable Hoard. The dock sits top right; open Settings in it for
the server and token. Assets go under `out_dir` (`assets/hoard` unless `hoard.toml` says otherwise), and the editor
rescans when they land. Double-click a result to add it, or pick several and press Add to project. To send, right-click
files or folders in the FileSystem dock → Send to Hoard… (or select them and press Send… in the dock). It lists what
will go before sending. Meshes and textures made in Godot go as `.glb` and `.png`.

**Unity 2021.3 or later.** Window → Package Manager → + → Add package from git URL:
`https://github.com/Azazel-Labs/Hoard.git?path=integrations/unity/com.hoard.browser` (or Add package from disk, and pick
[integrations/unity/com.hoard.browser/package.json](integrations/unity/com.hoard.browser/package.json) in a checkout).
Then Window → Hoard; Settings in its toolbar takes the server and token. Assets go under `Assets/Hoard` unless
`hoard.toml` says otherwise, and the editor imports them when they land. Click, Ctrl/Cmd-click and Shift-click pick
results; double-click adds one. To send, right-click assets or folders in the Project window → Send to Hoard… (or
select them and press Send… in the Hoard window). It lists what will go before sending. Meshes and textures made in
Unity go as `.obj` and `.png`.

**Unreal Engine 5** (built and tested with 5.8). Copy [integrations/unreal/Hoard/](integrations/unreal/Hoard/) into
your project's `Plugins/` and open the project; Unreal offers to build it (a C++ toolchain is needed, as for any code
plugin), or build it once with `RunUAT BuildPlugin -Plugin=…/Hoard.uplugin -Package=…` and copy the result. Then
Window → Hoard; the gear takes the server and token, kept in the editor's own settings. Files go under `Hoard/` beside
`Content/`, not in it (unless `hoard.toml` says otherwise), so Unreal never imports them twice; what was just
downloaded is then imported into `/Game/Hoard`, in the same folders: models bring their materials and textures, and
pictures, sounds and fonts come in on their own. Click, Ctrl-click and Shift-click pick results; double-click adds one. To send,
right-click assets or folders in the Content Browser → Send to Hoard… (or select them and press Send… in the Hoard
tab). It lists what will go before sending. Meshes, textures and sounds made in Unreal go as `.fbx`, `.png` and `.wav`.

**Lumina.** Copy [integrations/lumina/Hoard/](integrations/lumina/Hoard/) into your project's `Plugins/` and build the
project as usual; the plugin is C++ like the engine, and uses WinHTTP on Windows and libcurl on Linux. Then open the
Hoard drawer from the editor's tools; the gear takes the server and token, kept in your own settings
(`Hoard/lumina.json` under `%APPDATA%`, or `~/.config`), never in the project. Files go under `Hoard/` beside the
`.lproject` (unless `hoard.toml` says otherwise), and what was just downloaded is imported into `/Game/Content/Hoard`,
in the same folders: models bring their materials and textures, and pictures, sounds and fonts come in on their own.
Adding again updates those assets in place, so what uses them still does. Click, Ctrl-click and Shift-click pick
results; double-click adds one. It only adds: Lumina projects aren't sent to Hoard.

## Unreal packs

Fab and Unreal Marketplace packs often arrive as Unreal content (`Content/<Pack>/…/*.uasset`) rather than FBX, PNG
and WAV. Hoard reads editor packages from UE 4.x through 5.7 directly, without Unreal installed:

- **What it is:** the asset's class (`StaticMesh`, `Texture2D`, `SoundWave`, `MaterialInstanceConstant`, …) sets
  its kind, so a mesh files under models and a sound under audio. Tagged `unreal`, `ue:static-mesh`,
  `engine:ue5.3`.
- **Preview:** the thumbnail the editor saved for its Content Browser (PNG in UE4, JPEG in UE5).
- **Facts:** the asset registry tags the editor writes (texture dimensions, triangle and vertex counts, durations,
  material domains, …), shown in the detail panel and used for the usual tags (`poly:low`, `res:2k`).
- **Links:** each package's imports (`/Game/Pack/Materials/MI_Rock`) are resolved to the pack's own files, so a
  mesh links to its material instance and that to its textures, and a bundle download pulls them all.
- **Names:** naming-convention prefixes are dropped (`SM_TreeStump` → "Tree stump", `T_Rock_N` → "Rock normal").
- **Source files, where Unreal kept them:** a `SoundWave` stores the WAV it was imported from, and UE4-era
  textures keep their source as PNG. Hoard pulls those out: the sound gets a real waveform and plays in the grid,
  the texture gets analyzed like any PNG, and "Download .wav/.png" (or "Recovered WAV/PNG instead of .uasset" on a
  selection zip) gives you the original file.

### Converting to common formats

Hoard can take the content out of the packages and keep it as ordinary files, so a Fab pack ends up as PNGs, WAVs
and FBX like any other pack. It runs after every ingest (Settings → Adding packs → Unreal packages, default keep the originals) or on demand from
the pack page (More → Convert Unreal content) and `POST /api/packs/{id}/unreal/convert`.

| Package | Becomes | Done by |
|---|---|---|
| `Texture2D` imported from PNG/JPEG | that PNG/JPEG, byte for byte | Hoard |
| `Texture2D` imported uncompressed (TGA, PSD, EXR…) | PNG (8/16-bit) or EXR (HDR) when the pixels are stored plain or LZ4; FBX/PNG/EXR export when Oodle | Hoard, or Unreal Editor |
| `SoundWave` | the imported WAV, byte for byte | Hoard |
| `Material`, `MaterialInstanceConstant` | `material.json`: parent, texture/scalar/vector parameters, PBR slot guesses | Hoard |
| `DataTable` | JSON rows | Hoard |
| `StaticMesh`, `SkeletalMesh`, animations | FBX | Unreal Editor, via `hoard unreal-worker` |
| Blueprints, levels, … | stays a package (nothing to convert it to) | – |

Hoard's reader (`crates/hoard-unreal`) is its own safe Rust, no engine libraries: package summary, tagged
properties (UE 4.x, 5.0-5.3 and 5.4+ layouts), UE4 bulk data and the UE5 package trailer with uncompressed and LZ4
payloads. UE5 compresses raw source data with Oodle by default, which only Unreal decodes, so textures imported
from uncompressed formats usually go to the editor worker. Sources that were compressed when imported (PNG, JPEG)
don't shrink, so Unreal stores them as they are, and Hoard converts those itself.

The converted file takes over the asset: same id, tags, links, collections and family. `GET /api/assets/{id}`
gains an `origin` (original path, class, engine, how it was converted), the Unreal tags stay, and other packages'
references keep resolving to it. With `keep` the original `.uasset` stays downloadable
(`GET /api/assets/{id}/original`); with `replace`, or "Discard the original .uasset files" in the portal, it's
deleted once its content is out. Unreal-only data (material graphs, Blueprint logic) goes with it.

**Meshes and animations** need Unreal itself. Run the worker on any machine with Unreal Editor installed:

```sh
hoard --server https://hoard.example.com unreal-worker --editor "C:/Program Files/Epic Games/UE_5.4/Engine/Binaries/Win64/UnrealEditor-Cmd.exe"
```

It polls `GET /api/unreal/pending`, lays each pack out as a throwaway project with every package at its
`/Game/...` path (inferring the mount when an upload lost the `Content/` folder), runs the bundled export script
headless, uploads the FBX files (`POST /api/assets/{id}/replace`) and has the server re-analyze the pack.
`--once` works through the queue and exits, `--keep-originals` keeps the packages, and `--dry-run` shows the
layout without starting the editor. `cargo xtask unreal-export <pack>/Content --editor …` still exports a local
Content folder directly. Cooked packages from shipped games are recognized but carry no editor data to convert.

Check the license before moving content between engines: Fab's Standard License allows use in other engines and
tools, but some Epic-made content is licensed for Unreal Engine only.

## Conversion rules

Conversion rules make a library all alike: every OBJ, FBX, 3DS, USD, STL, PLY, SketchUp, glTF, Collada or Blender
file converted to GLB, say, and turned upright (Y up, facing +Z) on the way. Set them in Settings → Adding packs → Conversion rules
(admins only, none by default). Each rule names the formats it converts and what to; a format can be in one rule.

A rule converts files in packs Hoard keeps. Files in watched folders are never changed, and neither are files
something else finds by name: a scene or model that names them, a Unity project's `.meta` file, or a GLB of the same
name beside them. Each converted file takes the original's place, with its id, name (`tree.obj` becomes
`tree.glb`), tags, collections, links and family. Its textures aren't copied into it: the GLB points at the image
files beside it in the pack, so they're stored once. An OBJ's `.mtl` and a glTF's `.bin` stay as they were.

Rules apply to packs added from then on, after analysis. When saving rules that match files already in the
library, Settings says how many files in how many packs they'd convert, lists them, says what's left alone and why,
and converts them only on "Save and convert". "Convert files already in the library" asks the same at any time.
Formats only Blender reads (`.blend`, `.dae`, `.abc`, `.x3d`) wait for Blender's conversion (Settings → Tools).

The original is kept for 30 days (1 to 365 in Settings). Until then a converted asset's page shows where it came
from, offers the original for download, and can put it back, after which rules leave that file alone. A file a rule
can't convert is listed under Settings → Issues and stays as it was.

## Collections

A collection is a hand-picked set of assets from any packs, like everything chosen for one level. Select tiles
and use "Add to collection" (typing a new name creates one), or add from an asset's detail panel. Collections sit
at the top of the filter sidebar and on the packs page, download as one zip with their dependencies, and work in
`hoard.toml`:

```toml
[[select]]
collection = "Forest level"
dest = "levels/forest"
```

## Favorites and saved searches

Everyone signed in has their own: star a tile (or press F on it, or star its details), and Favorites, under
**Yours** at the top of the filter sidebar, shows just those, with every other filter still working. "Save search"
beside the active filters keeps what you're looking at under a name, there and in the command palette (Ctrl K);
saving the same name again replaces it. An account that can only look has them too, and an access token or a
connected app uses its maker's.

To send someone an asset, the share button beside the star in its details copies a link that opens it (on a phone or
tablet it brings up the share sheet instead). Whoever opens it signs in to Hoard first.

## The archive

Archiving a pack or an asset (from its menu, its details, or a selection) takes it out of search, the counts and the
tag list without deleting anything: the **Archive** (the button in the header, or `#archive`) lists it, searchable, until it's restored
or deleted for good. If a project's last `hoard sync` had any of it, archiving and deleting say which projects
and ask first. Deleting for good frees its space at the next clean-up, and a watched folder's pack deleted that
way isn't read in again (the folder itself is never touched). Archiving and restoring need an account that can change
the library; deleting for good, an admin.

## Search by color, by what pictures show, by how things sound, and More like this

Hoard looks at the thumbnail of every texture, model, material, animation, video and sprite, and works out its main
colors and what its surface looks like. A background job does this for new packs and, once, for everything already
in the library. Activity shows it as "Working out colors and looks".

- **Color.** The Color swatches in the filter sidebar (or the picker beside them, for any color) show assets with
  much of that color, the ones it covers most first. A slightly different shade still counts, because colors are
  compared the way eyes see them (in OKLab). Color works together with words, kind, tags, pack and collection. An
  asset's detail panel shows its colors as a strip sized by how much of the picture each covers. Click one to search
  the whole library for it.
- **More like this.** The button in the detail panel, or the overlapping squares on a tile's corner, shows assets that
  look like this one, across every pack, most alike first. You might get other rocks for a rock texture, the same
  texture at another size, or renders of similar models. The results are an ordinary search, so you can scroll, select
  and narrow them by kind, tags or words. "Newest" in the sort menu sets the ranking aside.

What counts as alike is the colors (a color histogram), the grain and edges of the surface (edge directions,
contrast and local binary patterns, measured in a way that ignores which way a texture is turned), and a perceptual
hash that catches near-copies. It is all computed on the server from the thumbnail, needs no model download and works
offline. On its own, nothing in it knows what a picture shows. Two brown, rough textures can count as alike even when
one is bark and the other is rock. Fonts, documents and shaders take no part.

A sound's **More like this** listens instead of looking: it brings up other sounds that sound like it, whatever their
files are called, so one footstep finds the other footsteps and one laser the other lasers. A background job,
"Listening to sounds", works through every WAV, Ogg, FLAC, MP3 and AIFF (the first 30 seconds of each) and keeps up
with new packs. It compares the spectrum's shape, how the sound rises and dies away, and its character: how long,
bright, noisy, busy and bass-heavy it is and how much it changes. How loud a file is, the silence around it and the
rate it was recorded at don't count. Like the pictures, it all runs on the server and needs no model.

The same listening measures each sound, and its details show what it found: its envelope (a hit, a swell or a
sustained sound, with how fast it starts and how long its tail rings), its tone (how bright and how noisy it is),
whether its pitch rises or falls and by how many octaves, and whether it loops cleanly. These are kept in the sound's
metadata in the API and become the tags above, so `pitch:falling tone:tonal` finds falling zaps.

With **Kind** set to Audio, a word in the search field also finds sounds by the noise they make, written the way you'd
spell it: `WHOOOM`, `pew`, `krssh`. The word is read as the shape of a sound. A start like b, p, k or t is a sharp attack, and wh or h a soft one.
Drawn-out letters make it longer. "oo" and "u" are low and dark, "ee" and "i" high and bright, and vowels that move
from one to the other make the pitch rise or fall. s, sh and f are hiss. An m or n at the end rings on, and a k, t or p
cuts it off. Capitals are loud. Sounds whose measurements fit that shape best come first, so `pew` finds falling
lasers and `shhhhhh` long hiss, after the sounds whose names, tags or sound match the word. It's a reading of English
spelling, not a model, and works with a pack, a folder or other filters set. In the API it's `imitation=<word>` for
the shape alone.

The microphone button in the search field takes the noise itself: make it, up to six seconds, and press it again. The
browser asks for the microphone first, and only gives it to a page on HTTPS or this computer. Hoard measures the
recording the way it measures every sound, without the room's noise either side, and finds the sounds shaped like
it. The recording isn't kept. In the API, `POST /api/imitation` with a WAV file answers with the `imitation` value
to search with.

### Tempo and key of music

Music and loops get a tempo and a key, worked out by the same listening. The tempo comes from where notes and beats
start, and the beat that fits them best, between 50 and 220 BPM with 70 to 180 preferred. Half or double the real
tempo is the usual mistake, so the other reading is kept too: the details say "about 70 BPM (or 140), as heard", and
a tempo search finds it either way. A loop whose length is a whole number of bars at that tempo gets it exactly, and
its length in bars (`bars:8`, in 4/4). The key comes from how much of each of the 12 notes is heard, matched against
major and minor key profiles, with its place on the Camelot wheel beside it, as DJs write keys: "A minor (8A)". A tempo
or key the file states (a WAV's ACID chunk, an MP3's or FLAC's BPM and key fields, a MIDI file's tempo and key
signature) wins over what's heard. MIDI files get their key from their notes, and tracker music from its rendering.
Sounds that die away at once and are shorter than four seconds are skipped, so a footstep doesn't get a key, and so is
anything under two seconds; a key needs eight seconds unless there's a beat. The first 30 seconds are what's listened
to, so a track whose tempo or key changes partway gets the one it starts with, or the one there most of that time.

They become tags, `bpm:120` and `key:a-minor`, and the sidebar's **Music** section searches by them: a tempo range,
and a key, which also finds the keys that mix with it (its relative major or minor, and the keys a fifth either side)
unless that's unticked. In the API they're `bpm_min`, `bpm_max`, `key` (written any way: `a-minor`, `Am`, `A minor`,
`8A`) and `compatible_keys=true`; over MCP, the same arguments to `search_sounds`.

### Image search

Settings → Image and sound search adds a model that knows what pictures show (Google's SigLIP 2, base size, 32-pixel
patches). It's off until an admin turns it on, because the model is a 1.5 GB download (from Hugging Face, pinned to
one revision and checked against its SHA-256, into the cache folder's `models`). It runs on the server's processor, and nothing leaves
the server. A background job, "Learning what pictures show", then works through every thumbnail at low priority, at
roughly 5–10 pictures a second, and keeps up with new packs. Settings shows how far it has got.

- **Words.** The main search box finds pictures that show what you typed ("mossy rock", "brick wall", "ceiling
  lamp") beside the assets whose names, tags or descriptions have the words. A name match still ranks above a weak
  picture match. Words with nothing to show return only the name matches.
- **More like this** weighs what the pictures show above their colors and grain, so a barrel brings up barrels
  rather than anything brown.

The model takes 350–750 MB of memory while it works: the picture half during the job, the words half from the first
search until 15 minutes after the last. Turning image search off deletes the model and keeps what it learned, so
turning it back on only downloads the model again.

A library that learned its pictures with OpenAI's CLIP, the model Hoard used before, keeps searching with what CLIP
learned while SigLIP 2 learns every picture again in the background. Then CLIP's embeddings and model files are
deleted.

A server too small to run the model can send the work to a hosted service instead: under "Runs on", pick "A hosted
service", then Voyage AI, Cohere or Jina AI, and paste its API key. Nothing is downloaded or run on the server then,
but **pictures leave it**: a thumbnail of every picture in the library, and the words of every image search, go to the
service, which charges for them and keeps them as its terms say. Settings says so before anything is saved. The
service learns every picture a batch at a time; if it says no (a wrong key, an outage), the job tries again every
hour. Another service or model learns every picture again, and what each one learned is kept, so switching back is
instant. The key stays on the server and is never sent to the browser.

### Sound search

Settings → Image and sound search has a second model, for sounds (LAION's CLAP, HTSAT unfused). Like image search,
it's off until an admin turns it on: the model is a 614 MB download (Apache-2.0, from Hugging Face, pinned and
checked the same way), and it runs on the server's processor. A background job, "Learning what sounds sound like",
listens to the first ten seconds of every sound at low priority, about one a second, and keeps up with new packs.

- **Words.** The main search box finds sounds that sound like what you typed ("an explosion", "a laser gun",
  "distant thunder roll", "glassy UI blip"), beside pictures that show it and assets named for it. A name match
  still ranks first. Words with nothing to hear return only the name and picture matches, though a sound that comes
  close now and then gets in.
- **Agents.** `search_sounds` over MCP with only a `query` searches the same way.

The model takes up to about 1 GB of memory while it works: the listening half during the job, the words half from
the first search until 15 minutes after the last. Turning sound search off deletes the model and keeps what it
learned.

## Credits

Hoard writes a `CREDITS.md` for whatever you ship: one row per pack with its seller, store page, license and
whether the license asks for credit, then the credit lines to paste into your game's credits and any terms worth
knowing ("Non-commercial use only."). The license comes from the pack's own license file, then the license on its
store page, then the store itself (a Fab pack without one is under Fab's Standard License). Each pack says
**Required**, **Not required**, **Keep the license notice** (MIT, GPL, OFL) or **Not known**. Nothing is guessed:
when Hoard doesn't know a license, the file says how many packs to check by hand.

- **Packs page:** More → Download credits on a pack, and Credits on a collection.
- **A selection:** select tiles, then Credits beside "Download zip". It covers the packs of everything selected
  and what it needs.
- **Zips:** every pack, collection and selection zip has `CREDITS.md` at its root.
- **Your game project:** `hoard credits` covers what `hoard.toml` selects (`--pack` and `--collection` name others
  instead, `--json` for JSON, `--out CREDITS.md` to write a file).
- **Agents:** the `get_credits` MCP tool.

### Ship check

Hoard doesn't track your projects. Whatever builds them asks instead: send the BLAKE3 hashes of the files you ship
(`b3sum`, or the `blake3` library) to `POST /api/check` (`{"hashes": [...]}`, up to 20,000; `?format=md` for
Markdown) or the `check_files` MCP tool. A hash matches a file as Hoard keeps it, as it was uploaded before Hoard
converted it, or a format Hoard made from it. The answer lists each file Hoard knows with its asset, path and pack,
the credits for those packs, the hashes Hoard doesn't know, and which files need a look before shipping:

| Warning | Means |
|---|---|
| `license_unknown` | Hoard doesn't know the pack's license, or can't read the one it has |
| `non_commercial` | CC BY-NC: can't be sold |
| `share_alike` | CC BY-SA or GPL: what's made from it shares the license |
| `keep_notice` | MIT, OFL, GPL: ship the license notice with it |
| `fab_personal` | bought under Fab's Personal tier, which has a revenue cap |
| `ai_generated`, `ai_edited` | the image says it was made or edited with generative AI; stores such as Steam ask you to disclose it |

A file is clear when any pack it's in covers it: the same bytes in a CC0 pack and an unknown one are fine.

## MCP for agents

`POST /mcp` is a stateless Streamable HTTP MCP endpoint. It speaks both the 2026-07-28 protocol (per-request `_meta`,
`server/discover`) and the older `initialize` handshake (2025-11-25 back to 2025-03-26), so current and older clients
both connect.

| Tool | What it does |
|---|---|
| `search_assets` | full-text + kind/tag/pack filters, with thumbnail and download URLs; `include_previews` returns the first 8 thumbnails as images |
| `find_similar` | assets that look like a given one (colors, surface, near-copies), across every pack; same result shape as `search_assets` |
| `find_by_color` | assets with much of a hex color in them, most of it first |
| `search_sounds` | sounds ranked by how well what was measured of them fits a word imitating them (`imitation`: "WHOOOM", "pew") and the agent's own reading of it (`shape`: seconds, a sharp or soft attack, ringing on or cutting off, dark or bright, tonal or noisy, pitch falling or rising, loud), with how Hoard read the word; or by tempo (`bpm_min`, `bpm_max`) and key (`key`, `compatible_keys`); each sound comes with its measurements, the same in words and a link to listen |
| `describe_sound` | what one sound sounds like, as measured, in numbers and words |
| `sounds_like` | the sounds that sound like a given one, described the same way |
| `get_asset` | full detail: metadata, tags with sources, links, what uses it, variants, identical copies, near copies (the best marked), the thumbnail's colors |
| `get_bundle` | download URLs and save-as paths for an asset and everything it needs, plus a zip URL |
| `list_packs`, `list_tags`, `list_collections` | browse the library, its tag vocabulary and collections (`search_assets` takes `collection` too) |
| `get_credits` | who made the packs behind some packs, collections, assets or a search, their licenses and the credit lines they ask for (`markdown` for a ready `CREDITS.md`) |
| `check_files` | given the BLAKE3 hashes of the files a project ships: which came from Hoard, their credits, and which need a look (unknown or limiting license, AI-made images); see [Ship check](#ship-check) |
| `tag_asset`, `update_asset` | edits, recorded as human |
| `next_to_tag` | the next files nothing has described yet (up to 16, one pack at a time), with facts, the pack's store page and README, and a numbered contact sheet image |
| `save_tags` | names, categories, descriptions and tags for a batch (and the pack), recorded as AI-made exactly like Hoard's own AI tagger: a person's edits always win |

The `tag_pack` prompt (in Claude Code: `/mcp__hoard__tag_pack <pack name>`) tells the agent to loop `next_to_tag` →
look → `save_tags` until the pack is done, with the same naming, category and tag rules Hoard's built-in tagger uses.

**Connecting Claude.** Settings → Account → *Claude and other AI apps* has the address and the steps. Apps sign in to
Hoard over OAuth the first time (Hoard opens in your browser and asks you to allow it), and Settings → Account lists
them and disconnects them.

- **Claude Code:** `claude mcp add --transport http --scope user hoard https://your-host/mcp`, then `/mcp` to sign in.
- **claude.ai, Claude Desktop and Cowork:** Settings → Connectors → Add custom connector, paste
  `https://your-host/mcp`, then Connect. These connect from Anthropic's servers, so Hoard has to be reachable from the
  internet over HTTPS with a certificate browsers trust (Settings → Connection with your own certificate, or a reverse
  proxy, Tailscale Funnel or a Cloudflare tunnel), and its Public address (Settings → Connection) set to that URL.

**Tagging with your Claude subscription.** Hoard can't sign in to Claude for you (Anthropic doesn't allow other apps to
use a Claude login), but Claude can come to Hoard: connect it and run `/mcp__hoard__tag_pack Medieval Village` (or ask
"tag the untagged packs in Hoard"). It looks at each contact sheet and saves its descriptions, on your plan.

Other MCP clients can sign in the same way (OAuth 2.1 with PKCE and dynamic client registration; the 401 from `/mcp`
points at `/.well-known/oauth-protected-resource/mcp`), or send an access token from Settings → Account as
`Authorization: Bearer <token>`. `/mcp` always needs one or the other, even with guest browsing on: asking is what makes
apps offer to sign in. Browser-originated requests from other origins are rejected (DNS-rebinding guard). Set the public
address in Settings → Connection so returned links are absolute.

## Storage, and splitting it from the web server

Each library location Hoard keeps is a folder, or an S3-compatible bucket: give the location's folder as
`s3://<bucket>` and enter the service's address and keys in Settings → Library locations → S3 storage (the `HOARD_S3_*`
variables are the defaults until something is saved there). The bucket can be Garage, RustFS, SeaweedFS or VersityGW on
a NAS, or R2/B2/AWS. Downloads are streamed through the server with HTTP Range support (audio and video seek), and
immutable caching headers let the browser keep them (and a CDN too, when guest browsing is on: otherwise they're marked
private, so a shared cache never hands them out). Extra headers go on every storage request, e.g. Cloudflare Access
service-token headers when the NAS's S3 endpoint sits behind Access. A folder that Dropbox, Google Drive, OneDrive or
iCloud Drive keeps in sync is a folder like any other.

The keys, headers and other secrets typed into Settings are kept with the server's settings. The desktop app seals them
with a key it keeps in the system's keychain (Windows Credential Manager, the macOS keychain, the Secret Service on
Linux), so a copy of the library folder doesn't carry them readable.

[docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) covers the setups. The recommended one is everything on the NAS, reachable
from anywhere only through Cloudflare One private routing (or Tailscale), with no public hostname
(`deploy/compose.private.yml`). It also covers a public hostname via Cloudflare Tunnel, web on a VPS with blobs on the
NAS, and a Cloudflare-native plan.

## HTTPS, HTTP/2 and HTTP/3

A library page loads hundreds of thumbnails, so connection reuse matters. Over plain HTTP, `hoardd` speaks HTTP/1.1
and cleartext HTTP/2 (h2c), but browsers only use HTTP/2 over HTTPS, where the whole grid shares one multiplexed
connection. Browsers also keep some things for HTTPS pages: 3D with WebGPU, the modern clipboard, installing Hoard as
an app, and Claude connectors. When the portal is opened over plain `http://` from another device, a dismissible line
under the header says so and links to the setup.

**Settings → Connection** turns it on. HTTPS is served on the same port as `http://` (4627 unless `--listen` says
otherwise): Hoard tells the two apart by the first byte a client sends. So there's no extra port to open, and a change
applies at once, without a restart. Choose one of:

- **Hoard's own certificate.** Hoard makes a certificate authority once (kept in the data folder, under `tls/`) and
  issues the server's certificate from it, for the addresses it finds (this computer's name and LAN addresses, the
  public address from Settings → Connection) and any you add. The address you're using is added for you, which matters in
  Docker, where Hoard can't see the host's addresses. Download `hoard-ca.crt` from the same page and install it once on
  each device (the page has the steps for Windows, macOS, iPhone and iPad, Android, Linux and Firefox); its browsers
  then trust Hoard, now and after every renewal. The server certificate is renewed and re-issued by itself. Give the
  `hoard` CLI the same file with `--ca-cert hoard-ca.crt`.
- **My certificate.** Paste or upload a PEM certificate chain and its private key. Hoard checks that the key belongs to
  the certificate and that it's in date, and shows the names it covers and when it runs out. On a tailnet with HTTPS
  turned on, `tailscale cert <machine>.<tailnet>.ts.net` gives you one browsers trust (for 90 days, so upload the new
  one before then); for a domain you own, use any ACME client (certbot, acme.sh, lego) and upload `fullchain.pem` and
  `privkey.pem`.
- **Off.** Plain `http://` only. The right choice behind a reverse proxy (Caddy, Traefik, nginx, Cloudflare Tunnel,
  Tailscale Serve or Funnel) that has the certificate: let it send plain `http://` to Hoard. Hoard reads
  `X-Forwarded-Proto` from it, so sign-in cookies and MCP links still say `https`.

Hoard tries a new certificate in a test connection before switching to it, and keeps what it had if that fails.

**Plain `http://` once HTTPS is on.** It keeps working on the same port. Browsers on other devices that load a page
over `http://` are sent to `https://` (a temporary redirect, and Hoard never sends HSTS, so turning HTTPS off again
just works); a switch on the page turns that off. Everything else is served as before: the `hoard` CLI, the Unreal
worker, scripts, MCP apps, anything going through a reverse proxy, and `http://localhost` on the server itself. If a
certificate ever locks you out, open `http://localhost:4627` on the server, or run `hoardd --https-off` (in Docker:
`docker compose exec hoard hoardd --https-off`) and restart Hoard.

**HTTP/3** (a switch on the same page) adds QUIC on the same port number over UDP, advertised with `Alt-Svc`, so
browsers move to it after the first page load. Open UDP 4627 as well as TCP (the compose files map both). Browsers only
use HTTP/3 with a certificate they trust.

Responses are compressed with Brotli/zstd/gzip where it helps (not for images, audio, models or zips), the built
portal's hashed assets are cached as immutable, and blob downloads carry ETags and Range support.

## Configuration

Where the server keeps things, how it listens and how it's protected are set when it starts, with a flag or env var
(`hoardd --help` lists them). Everything else (the server's name, AI tagging, Blender, ffmpeg, Unreal conversion,
thumbnails, the update check, workers, limits, the public address, HTTPS, sign-in, the SoundFont) is changed in
**Settings** in the portal header (by an admin), takes effect at once, and is kept in the database, so it survives restarts and backups.

**Settings → General** names the server: the name is shown in the top bar, the browser tab and on the sign-in page,
for telling several hoards apart.

Set before starting (these need a restart):

| Env | Default | |
|---|---|---|
| `HOARD_LISTEN` | `127.0.0.1:4627` | bind address |
| `HOARD_DATA_DIR` | `./hoard-data` | the database (with Settings), its backups, the HTTPS certificates |
| `HOARD_FILES_DIR` | – | the folder the first-run setup suggests (`/library` in the image) |
| `HOARD_CACHE_DIR` | `<data dir>` | thumbnails and previews, uploads in progress, scratch space, downloaded Blender/ffmpeg |
| `HOARD_WEB_DIR` | `./web/dist` | built portal |
| `HOARD_LOCAL_LLM_URL` | `http://llm:8080/v1` | where the local model server listens (Settings → AI tagging → Local model) |
| `HOARD_S3_ENDPOINT` / `_BUCKET` / `_REGION` / `_ACCESS_KEY` / `_SECRET_KEY` / `_HEADERS` | – | the S3 service for `s3://` library locations |
| `HOARD_SETUP_CODE` | – | a code whoever makes the first account has to give; without it, the first visitor makes it |

## Keeping it safe

**Settings → Maintenance** has a button for each job that looks after the library, with progress while it runs and a history of past runs:
back up the database, check files for damage, re-analyze every pack (thumbnails, previews and rule tags made afresh;
your edits stay, the AI isn't asked again), shrink thumbnails, clear cached "Download as" conversions and leftovers,
and delete unused files. One of each runs at a time. Backup, scrub and gc can also be run from cron (below) and show up in the
history the same way.

**Settings → Issues** lists what went wrong opening or processing files, most serious first: packs that failed to
ingest, damaged or missing files, watched folders that can't be read or have vanished (an unmounted drive), jobs that
gave up, archives that wouldn't unpack, files that couldn't be opened or made sense of, models the thumbnail renderer
failed on, failed conversions and Unreal packages, material detection and store files that couldn't be read. Each
links to its pack or file, and says how often it happened and when. Fix the cause, then try it again from there
(scan the folder again, re-analyze the pack, retry the job), one at a time or a whole kind at once, or dismiss it.
An issue that's fixed by the next scan or analysis goes away on its own. `GET /api/admin/issues` has the same list.

**Profiler.** The top of Settings → Performance shows what Hoard has been doing over the last ten minutes, updated
every two seconds while you watch: how much of the processors Hoard and the whole machine are using, and its memory;
what it's working on (each ingest, thumbnail run, analysis or other job as a lane across the same clock); and the time
spent serving requests, ten seconds to a bar, split into the database, reading from disk, sending to the browser and
everything else. Below that each route has its count, average, slowest 5%, where its time went and how much it sent.
It's kept in memory only and starts over when Hoard restarts. `GET /api/admin/profile?after=` has the same samples.

**Thumbnails.** Settings → Performance → Thumbnails picks how thumbnails and model views are stored: Smaller, Balanced
(the default) or Sharper JPEG, or Lossless PNG. Anything with transparency (model renders, waveforms, font
specimens, cut-out sprites) and pixel art always stays PNG, and so does a picture whose PNG is about as small as the
JPEG would be. For photo-like textures Balanced is roughly an eighth of the PNG. Changing it affects new thumbnails;
Maintenance → Shrink thumbnails re-stores the existing PNG ones.

- Content addressing: every byte is verifiable. `POST /api/admin/scrub` re-hashes all blobs (local or S3) and flags
  corrupt or missing ones; the portal shows the count. It answers 202 and runs in the background.
- Write-once blobs (read-only on disk), fsync before rename on local storage.
- `POST /api/admin/backup` writes a consistent SQLite snapshot (`VACUUM INTO`) to `backups/` next to the database
  and answers `{"summary": "Saved <path> (<size>)"}`. Back up that file plus the blob store; blobs never change, so
  incremental sync is trivial.
- `POST /api/admin/gc` deletes blobs no asset, thumbnail, derivative or kept original references (after deleting
  packs).
- Uploads: path traversal rejected, archive expansion capped, symlinks in tarballs ignored. A file opened on its own
  is served sandboxed (no scripts, nothing loaded from elsewhere) unless it's a picture, sound, video or PDF, so an SVG
  or HTML page in a pack can't act as whoever opened it.
- Store pages and store searches are only fetched from the internet, never this machine or the local network, however
  the address is written or wherever it redirects.

## Accounts and access tokens

The first visit to a new server makes its first account, an admin. If a new server is reachable by people you don't
know, set `HOARD_SETUP_CODE` in its environment before starting it, and that account can only be made by someone who
gives the code. Add accounts for other people, change your password, and make access tokens in **Settings → Account**. Each
account has a role:

| Role | Can |
|---|---|
| Can look | search, preview and download; sync a project; make tokens that only look |
| Can change | also add packs, rename, describe, tag, collect, archive and restore (new accounts get this) |
| Admin | also Settings, library locations, maintenance, accounts and everyone's tokens, and deleting for good |

An admin changes another account's role in its row; nobody changes their own, so there's always an admin. The portal
only shows what an account may do: one that can look has no Add files button or tag editing, and only the Appearance,
Library and Account pages of Settings.

- **Signing in** gives the browser a session cookie (HttpOnly, SameSite=Lax) that lasts until you sign out or don't
  visit for 30 days, or for a set time from signing in if Settings → Authentication says so. Repeated wrong passwords
  for a name are slowed down. Removing an account signs it out everywhere and revokes its tokens and connected apps.
- **Access tokens** are for everything that isn't a browser: the `hoard` CLI (`--token` or `HOARD_TOKEN`), scripts and
  cron, MCP clients and the Unreal worker. They're sent as `Authorization: Bearer <token>`, shown once when made, and
  can be revoked; the list shows when each was last used. A token has a role too, up to its maker's (and lowered with
  it if the maker's is), and never manages accounts or tokens. Each account sees and revokes its own; admins see all.
- **Connected apps** (Claude, other MCP clients) sign in over OAuth: you allow each one in the browser, it acts as
  you with your role (one that can look isn't offered the tools that change things), and Settings → Account
  disconnects it.
- **Guest browsing** (off by default) lets anyone who can reach the server search, preview and download without
  signing in. Changing anything still needs an account or a token.
- **Forgot your password?** On the server, `hoardd --reset-password <name>` prints a new one and signs that account out
  everywhere. In Docker: `docker compose exec hoard hoardd --reset-password <name>`.

### Signing in with a directory or OAuth

**Settings → Authentication** chooses how people sign in besides Hoard's own accounts, which always work (so a
directory that's down can't lock out the admin):

- **LDAP**: the name and password typed on the sign-in page are checked against a directory: OpenLDAP (or FreeIPA,
  LLDAP and the like) or a custom setup. List one or more servers (the next is tried when one can't be reached), choose
  StartTLS, LDAPS or none, and give a certificate authority if the directory's certificate isn't one the system trusts.
  Hoard finds the person's entry as a reader account (or anonymously) under *People are under*, matching the name
  attribute and an optional filter, then signs in as them with their password. Their groups come from `memberOf` and,
  if *Groups are under* is set, a search there (`{dn}` and `{user}` stand for the person).
- **Microsoft Active Directory**: the same, with Active Directory's defaults (`sAMAccountName`, nested groups).
- **OAuth**: a *Sign in with…* button on the sign-in page goes to the provider and back, with PKCE. Presets fill in
  Google, Microsoft Entra ID, GitHub and GitLab; for anything else (Keycloak, Authentik, Authelia, Okta…) give the issuer
  and *Discover* reads its OpenID configuration. Give the provider the redirect address shown there (it's built from
  the public address in Settings → Connection). The name comes from a claim (`preferred_username` unless you say), and
  so do groups.

*Test connectivity* reaches each LDAP server, signs in as the reader account and, given a name and password, as that
person, and shows their groups and the role they'd get, without saving anything.

People from a directory or provider get a Hoard account the first time they sign in (unless that's turned off: then an
admin adds them in Settings → Account, choosing how they sign in), with the default role. Under *Roles from groups*,
list the groups whose members are admins, can change or can look: the highest that matches wins and is checked at
every sign-in, and with groups listed under *Can look*, people in none of the listed groups are turned away. Their
passwords are the directory's or provider's, so Hoard has none to change or reset for them, and a Hoard account of the
same name that signs in another way is never taken over. An OAuth account belongs to who the provider says signed in
(its issuer and `sub`), not the name: it follows them when they're renamed there, and someone else given their old name
is turned away.

Directory and OAuth accounts, with their sessions, tokens and connected apps, only work while that's how people sign
in. An OAuth sign-in lasts a day unless Settings → Authentication sets a time. People from a directory are asked about
again every 15 minutes: someone it no longer has (or, in Active Directory, has disabled), or who's left every group
under *Can look*, is signed out everywhere, and everyone else's role follows their groups. A saved reader password or
client secret is only sent where it was given: change the servers, how they're reached or the token address, and it's
asked for again.

From cron, with a token made for it:

```sh
curl -fsS -X POST -H "Authorization: Bearer $HOARD_TOKEN" http://localhost:4627/api/admin/backup
```

## API overview

Every request needs a signed-in session or `Authorization: Bearer <token>`, except reads while guest browsing is on.
Requests that change something are marked 🔒: guests and accounts that can only look can't make them. Those under
`/api/admin/` (except your own tokens and connected apps), deleting a pack and emptying the archive need an admin. A
browser session's changes must also carry an `X-Hoard` header, which pages on other sites can't add.

**Assets**

| Method | Path | |
|---|---|---|
| GET | `/api/assets` | search: `q`, `kind`, `tags=a,b`, `pack`, `collection`, `ext`, `similar=<asset id>`, `color=<hex>`, `imitation=<word>` (sounds shaped like it), `favorites=true` (whoever's asking), `project` (name or id), `sort`, `limit`, `offset`, `hide_supporting`; results plus kind/tag facets |
| GET | `/api/assets/{id}` | one asset, with tags, metadata and dependencies |
| GET | `/api/duplicates?offset=&limit=` | the library's sets of near copies, the best of each first, the most space taken by the rest first, and how much they take in all |
| PATCH 🔒 | `/api/assets/{id}` | rename, recategorize, describe, tag; `apply_to_family` for a whole variant family |
| GET | `/api/assets/{id}/file` | the file (`?download=1` to save it) |
| GET | `/api/assets/{id}/thumb` | thumbnail (`?v=` its `thumb_rev`, for a URL that's cached for good) |
| PUT 🔒 | `/api/assets/{id}/thumb` | a model's thumbnail drawn by a browser (PNG/JPEG/WebP body, `?source=browser` or `browser-partial`); kept only if it's no worse than the one there |
| GET | `/api/assets/{id}/lite` | a texture's light preview (at most 2048 px), made on first request if it has none |
| GET | `/api/assets/{id}/lottie.json` | a Lottie animation to play: a `.json` as it is, a `.lottie`'s first animation with its images inlined |
| GET | `/api/assets/{id}/frames.png` | an Aseprite file's frames on one sheet, as many columns as rows, each its `width`×`height`, to play with its `frame_ms` |
| GET | `/api/assets/{id}/original` | the file as it came in, when it was converted on ingest |
| GET | `/api/assets/{id}/derivative/{kind}` | `glb`, `views`, or `source` (a recovered WAV/PNG) |
| GET | `/api/assets/{id}/as/{format}` | download converted (`?max=` size, `?wait=` seconds); `/status` to follow a slow one |
| GET | `/api/assets/{id}/bundle.zip` | the asset and everything it needs (`?animations=1`) |
| POST 🔒 | `/api/assets/{id}/replace?ext=` | swap in a new file (body is the file) |
| POST 🔒 | `/api/assets/{id}/unconvert` | put back the original a conversion rule replaced |
| GET | `/api/bundle.zip?ids=a,b,c` | a selection as one zip (`&deps=0`, `&sources=1`, `&glb=1` for models as GLBs) |
| POST | `/api/manifest` | a flat file list (hash, path, URL) for `ids` or a search, with `include_deps`; what engine tools pull from |
| POST | `/api/credits` | credits for `packs` (ids or names), `collections`, asset `ids` and `selections` (manifest requests); `?format=md` for `CREDITS.md` |
| GET | `/api/blobs/{hash}` | raw content by BLAKE3 hash |
| GET | `/api/tags?prefix=` | tag autocomplete (`anywhere=true` matches inside names, for Settings → Tags) |
| POST 🔒 | `/api/tags/merge` | `{from: [...], into}`: retag those assets and remember the old names |
| GET, DELETE 🔒 | `/api/tags/aliases` | merged-away names; `DELETE ?tag=` allows one again |

**Packs**

| Method | Path | |
|---|---|---|
| GET | `/api/packs` | all packs |
| POST 🔒 | `/api/packs` | upload a pack in one request (multipart) |
| GET | `/api/packs/{id}` | one pack and its status |
| PATCH 🔒 | `/api/packs/{id}` | rename it (`{name}`), turn its models (`{facing: {turn, tilt}}`, quarter turns in degrees) |
| DELETE 🔒 | `/api/packs/{id}` | delete it (blobs go at the next `gc`); admins only |
| POST 🔒 | `/api/packs/{id}/retag` | run the tagger again |
| POST 🔒 | `/api/packs/{id}/read-files` | read the pack's store files again (name, tags, description) |
| GET, PUT 🔒 | `/api/packs/{id}/context` | store link, seller, license, README and notes the tagger reads |
| GET 🔒 | `/api/packs/{id}/store-matches` | likely store pages (`q` to search for something else, `fresh` to search again) |
| POST 🔒 | `/api/packs/{id}/store/undo` | take off a store page that was found automatically |
| GET | `/api/packs/{id}/files/{path}` | a pack file by its original path (the 3D viewer uses this) |
| GET | `/api/packs/{id}/bundle.zip` | the whole pack |
| GET | `/api/packs/{id}/credits` | seller, license and credit line as JSON (`?format=md` for `CREDITS.md`) |

**Archive**

| Method | Path | |
|---|---|---|
| GET | `/api/packs?archived=true`, `/api/assets?archived=true` | what's in the archive |
| POST 🔒 | `/api/archive` | `{packs?, assets?}`: archive them |
| POST 🔒 | `/api/archive/restore` | `{packs?, assets?}`: put them back |
| POST 🔒 | `/api/archive/purge` | `{packs?, assets?, all?}`: delete archived ones for good; admins only |

**Resumable uploads** (what the portal and `hoard push` use)

| Method | Path | |
|---|---|---|
| POST 🔒 | `/api/uploads` | start: the list of files and sizes |
| PUT 🔒 | `/api/uploads/{id}/chunk?path=&offset=` | one part of a large file (`part_bytes` each, any order, several at once) |
| POST 🔒 | `/api/uploads/{id}/batch` | many small files in one request |
| GET, DELETE 🔒 | `/api/uploads/{id}` | what's arrived (to resume), or abandon it |
| POST 🔒 | `/api/uploads/{id}/complete` | all there: ingest it |

**Collections**

| Method | Path | |
|---|---|---|
| GET, POST 🔒 | `/api/collections` | list, or create |
| GET, PATCH 🔒, DELETE 🔒 | `/api/collections/{id}` | one collection |
| POST 🔒 | `/api/collections/{id}/assets` | `add` / `remove` assets |
| GET | `/api/collections/{id}/bundle.zip` | all of it as a zip (`?sources=1`) |
| GET | `/api/collections/{id}/credits` | credits for its packs (`?format=md`) |

**Projects** (what each `hoard sync` last had)

| Method | Path | |
|---|---|---|
| GET | `/api/projects` | every project, with how many assets it has and when and by whom it last synced |
| PUT | `/api/projects/{name}` | `{assets: [{id, path}]}`: what this project has now, replacing the last report; looking is enough |
| DELETE 🔒 | `/api/projects/{name}` | forget it |
| POST | `/api/usage` | `{packs?, assets?, archived?}`: the projects using any of these, with how many and a few of their paths |

**Favorites and saved searches** (each person's own, so a guest has none; looking is enough)

| Method | Path | |
|---|---|---|
| GET, POST | `/api/me/favorites` | your favorite asset ids, or change them: `{add?: [ids], remove?: [ids]}` |
| GET, POST | `/api/me/searches` | your saved searches, or save one: `{name, query}` (the portal's query string, like `q=rock&kind=texture`) |
| DELETE | `/api/me/searches/{id}` | forget one |

**Unreal**

| Method | Path | |
|---|---|---|
| GET | `/api/packs/{id}/unreal` | what's converted and what's waiting |
| POST 🔒 | `/api/packs/{id}/unreal/convert` | convert (`discard_originals`, `drop_unsupported`) |
| POST 🔒 | `/api/packs/{id}/unreal/finish` | after a worker's replacements: retag, queue previews, free discarded originals |
| GET | `/api/unreal/pending?limit=` | Unreal packages still waiting for the Unreal worker |

**Server**

| Method | Path | |
|---|---|---|
| GET | `/api/health` | `{"ok": true}` |
| GET | `/api/stats` | counts, storage, renderer, a newer release if there is one |
| GET | `/api/activity` | background work in progress |
| GET | `/api/admin/jobs` | the job queue |
| GET 🔒 | `/api/admin/profile?after=` | the profiler: processor and memory use each second, what's running, and each request's time in the database, on disk and sending, for the last ten minutes or since `after` |
| GET 🔒, PUT 🔒 | `/api/admin/settings` | Settings: read, or change some (`null` goes back to the flag) |
| POST 🔒 | `/api/admin/conversions/preview` | what conversion rules (`{"rules": …}`) would convert in the library, and what they'd leave alone |
| POST 🔒 | `/api/admin/conversions/run` | convert files already in the library by the saved rules |
| GET 🔒 | `/api/admin/update` | this version, the newer release if one is out, whether Hoard is idle and what's keeping it busy, the last call to the update address |
| POST 🔒 | `/api/admin/update/hook` | call the update address now |
| GET, POST 🔒 | `/api/admin/store-formats` | the store file formats, and files that look like a store's that none reads; `{toml, replaces?}` saves one in the data folder and reads the packs with such a file again |
| POST 🔒 | `/api/admin/store-formats/try` | `{toml, example?}`: what a format reads in a file and in the library's files with a name it takes, without saving |
| POST 🔒 | `/api/admin/store-formats/share` | `{toml, example}`: a zip of the format, its sample and what the sample reads as, laid out like the repository |
| DELETE 🔒 | `/api/admin/store-formats/{file}` | remove one of the data folder's formats |
| GET 🔒, PUT 🔒 | `/api/admin/https` | Settings → Connection: what's served, or change it: `{mode?: "off"\|"own"\|"custom", names?, http3?, redirect?, cert?, key?}` (tried before it's switched to; 400 with the reason if it doesn't work) |
| POST 🔒 | `/api/admin/https/check` | `{cert, key}`: what a certificate covers and until when, and whether the key matches, without saving |
| GET | `/api/https/ca.crt` | Hoard's certificate authority, to install on devices |
| GET 🔒, POST 🔒 | `/api/admin/locations` | library locations (with whether each is reachable and its pack count), or add one: `{mode, path, kinds, catch_all, name?}` |
| PATCH 🔒, DELETE 🔒 | `/api/admin/locations/{id}` | rename one, change its kinds or make it the catch-all; or remove it. A managed one must be empty (409 otherwise), and one that can't be reached is refused with 503 unless `?force=true` |
| POST 🔒 | `/api/admin/locations/{id}/empty` | move a managed location's files to the other locations by kind, then remove it; returns 202 and runs as a job |
| POST 🔒 | `/api/admin/locations/rebalance` | move every stored file to the location its kind goes to now; returns 202 and runs as a job |
| POST 🔒 | `/api/admin/locations/{id}/scan` | look through a watched folder now; returns 202 and the scan runs as a job |
| GET 🔒 | `/api/admin/fs?path=` | the folders in a folder on the server, for picking a location |
| POST 🔒 | `/api/admin/llm/models` | `{url, api_key?}`: the models an AI service offers (also a connection test) |
| POST 🔒 | `/api/admin/scrub` | re-hash every blob |
| POST 🔒 | `/api/admin/gc` | delete blobs nothing uses |
| POST 🔒 | `/api/admin/backup` | snapshot the database |
| GET 🔒 | `/api/admin/maintenance` | maintenance tasks, what's running, past runs |
| POST 🔒 | `/api/admin/maintenance/{task}` | start `backup`, `scrub`, `reanalyze`, `shrink_thumbs`, `clear_cache` or `gc` |
| POST | `/mcp` | MCP over streamable HTTP (needs a sign-in: OAuth or an access token) |
| GET | `/.well-known/oauth-protected-resource/mcp`, `/.well-known/oauth-authorization-server` | OAuth discovery for MCP apps |
| POST | `/oauth/register`, `/oauth/token` | OAuth dynamic client registration; codes and refresh tokens for access tokens |
| GET 🔒 | `/api/admin/connections` | apps signed in over OAuth: your own, or everyone's for an admin (`DELETE /api/admin/connections/{id}` disconnects one) |

**Accounts**

| Method | Path | |
|---|---|---|
| GET | `/api/auth/status` | whether there's an account yet, and who's signed in with what role |
| POST | `/api/auth/setup`, `/api/auth/login`, `/api/auth/logout` | the first account; signing in and out |
| POST 🔒 | `/api/auth/password` | change your password |
| GET 🔒, POST 🔒 | `/api/admin/tokens` | your access tokens (an admin sees all), or make one: `{name, role?}` (it defaults to yours) |
| DELETE 🔒 | `/api/admin/tokens/{id}` | revoke one |
| GET 🔒, POST 🔒 | `/api/admin/users` | accounts, or add one: `{username, password, role?, source?}` (role `view`, `edit`, the default, or `admin`; source `internal`, the default, or `ldap` or `oauth` with no password); admins only |
| GET 🔒, PUT 🔒 | `/api/admin/auth` | Settings → Authentication, and the OAuth redirect address; secrets are never sent back, and left empty they're kept; admins only |
| POST 🔒 | `/api/admin/auth/ldap/test`, `/api/admin/auth/oauth/discover` | test an LDAP setup (`{kind, config, username?, password?}`); read a provider's addresses from its issuer (`{issuer}`); admins only |
| GET | `/api/auth/oauth/start`, `/api/auth/oauth/callback` | signing in with the OAuth provider |
| PATCH 🔒, DELETE 🔒 | `/api/admin/users/{id}` | change another account's role (`{role}`), or remove it; admins only |

## Repository layout

```
crates/hoard-core     shared wire types (serde), kind detection, tag normalization
crates/hoard-render   headless wgpu renderer: glTF/GLB/OBJ in, thumbnail out (`cargo run -p hoard-render --example render`)
crates/hoardd         the server; formats/ describes the files stores ship beside their art (Megascans, Sketchfab)
crates/hoard          the CLI
crates/hoard-unreal   Unreal package reader and converters (textures, sounds, materials, data tables)
xtask/                `cargo xtask`: dev servers, checks, local model downloads and serving, Unreal export
integrations/godot    the Godot 4 editor plugin (addons/hoard) and its headless test script
integrations/unity    the Unity editor package (com.hoard.browser) and a .NET harness that runs its code without Unity
integrations/unreal   the Unreal Engine 5 editor plugin (Hoard), with a HoardAdd commandlet for headless tests
integrations/lumina   the Lumina editor plugin (Hoard), and a program that runs its core without the engine for tests
web/                  Vue 3 + TypeScript portal (Vite); three.js viewer is lazy-loaded
web/scripts/          readme-images.mts: the README's screenshots and the social card, in docs/img/
.vscode/              tasks, debug configurations, settings and recommended extensions
deploy/               Dockerfile companions: compose topologies, Garage config, .env.example
tools/                sample pack generator (now with a small Unreal content folder), Unreal Editor export script
```

Rust 2024 edition, stable toolchain (MSRV 1.100), clippy `pedantic` plus panic/footgun lints for the whole workspace
(`[workspace.lints]`), rustfmt at 120 columns. CI runs fmt, clippy `-D warnings`, tests and the web build.

## Status

Built and exercised end to end in development: ingest of the sample pack; offline tagging, PBR sets and license
detection; search and facets; Range proxying; the S3 backend (against a local S3 emulator); scrub catching a
deliberately corrupted blob; the LLM tagger (against a mock OpenAI-compatible server); MCP in both protocol eras;
wgpu thumbnails on lavapipe; link discovery and bundle zips; `hoard sync` including pruning and bundles; the
portal including the three.js viewer on its WebGL2 fallback; and source-format previews and links, checked against
spec-built .blend (raw and zstd), PSD, Krita and EXR files. These were generated from the formats' specs, not saved
by Blender, Photoshop or Krita. USD reading was checked against Poly Haven's USDC models (with their textures,
and packed as a `.usdz`) and ambientCG's material-only USDC. .blend reading, including Blender 5.0's new header,
was also checked against the 18 .blend files that ship with Blender 5.0.1 (saved by Blender 4.3 to 5.0, all zstd).

Written but not yet run against the real thing: files saved by real Photoshop/Krita (please throw some at it),
Blender conversion, Garage/RustFS specifically, Cloudflare Tunnel/Access, and LLM vision mode.

Also exercised: resumable browser and CLI imports of 1.1 GB with an interrupted, resumed run and duplicate refusal;
HTTPS with HTTP/2 and HTTP/3 in Chrome (page, thumbnails and uploads over h3); WAV library metadata, variant
families, duplicate detection, pack context with store-page fetching, contact sheets and multi-view renders against
a mock LLM; MCP inline previews; browser-drawn model thumbnails, light previews and the full-resolution swap in
headless Chromium; and the Docker image, its compose files and the viewer's WebGPU path on real hardware, deployed
and in daily use.

What's planned next is in [docs/roadmap.md](docs/roadmap.md).

## Facts and figures

| | |
|---|---|
| File types recognized | 158 extensions in 13 kinds, from PNG and WAV to `.uasset` and `.skp` |
| Read natively, no other app | glTF/GLB, FBX (binary and ASCII), OBJ, STL, PLY, 3DS, MagicaVoxel, 3MF, USD/USDA/USDC/USDZ, SketchUp 3 to today, Unreal packages from UE 4.x to 5.7, PSD/PSB, Krita, OpenRaster, Aseprite, DDS, KTX, KTX2, PowerVR, VTF, Basis, ICO, QOI, PNM, PCX, EXR, HDR, SVG, Illustrator, Lottie, Rive, DragonBones, Spine (JSON), TTF/OTF/WOFF2, WAV/BWF/RF64, OGG, Opus, FLAC, MP3, AIFF, MIDI, MOD/XM/S3M/IT/MPTM/UMX, NSF, VGM, FMOD banks (FSB, .bank), Wwise WEM and BNK, SFZ, SF2 |
| Archives unpacked | zip, tar, tar.gz and `.unitypackage`, nested up to 3 deep |
| Download as | 6 image formats, 5 audio formats, GLB or glTF for models, TTF, WOFF or WOFF2 for fonts, 10 sprite sheet formats |
| Engine plugins | Godot 4, Unity 2021.3+, Unreal Engine 5 |
| Ways in | web portal, `hoard` CLI, `hoard sync` with a lockfile, REST API, MCP for AI agents |
| Languages | 31, each with all 2,571 of the portal's and server's messages |
| Runs on | one Docker image for amd64 and arm64; binaries for Linux (x86-64 and ARM), macOS (Apple silicon) and Windows |
| Built with | Rust (axum, wgpu) and Vue 3; SQLite with full-text search; BLAKE3 content-addressed storage on disk or S3 |
| Tested by | 350+ Rust tests and 90 end-to-end browser tests, on every push |
| License | MIT-0: no attribution needed |

## Translations

Hoard's portal and server messages come in 31 languages, picked in Settings → Appearance (the browser's language by
default). The first translations were machine-assisted; corrections from native speakers are welcome.
[docs/translations.md](docs/translations.md) explains how.

## License

[MIT No Attribution](LICENSE) (MIT-0): use, copy, modify and redistribute Hoard for any purpose, with no need to
keep the copyright notice. The asset packs you store in Hoard keep their own licenses.
