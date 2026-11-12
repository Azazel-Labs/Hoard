# Dead ends

Ideas that were tried or seriously considered and then dropped, with the reason why. Check here before reviving one;
if the reason no longer holds (a crate appears, a project is revived, terms change), say so in the entry and go
ahead. Add new entries at the top of their section, dated.

Each entry: **what** was tried, **why** it didn't work out, and **instead**, what Hoard does now.

## Deployment and updates

### Port 7878 as the default (2026-11)

- **Why not:** it's Radarr's port, so it clashed on many of the home servers Hoard is meant for.
- **Instead:** 4627.

### The original Watchtower for automatic updates (2026-10)

- **Why not:** the containrrr/watchtower project was archived in December 2025 and reportedly breaks on Docker
  Engine 29+.
- **Instead:** the maintained fork, `nicholas-fedor/watchtower`, pinned to a real tag and limited to Hoard's own
  containers.

### One Docker volume for everything (2026-10)

- **Why not:** the database, the bulk files and the cache need different things. The database wants a fast local
  disk and backups, the files want the big disk, and the cache can be thrown away. One volume put them all in one
  place.
- **Instead:** three volumes: `/config` (database and settings), `/library` (files) and `/cache` (thumbnails,
  staging).

### Asking people to set PUID/PGID (2026-11)

- **Why not:** a root-owned `/cache` failed with "create /cache/staging: Permission denied" unless the user knew
  to set them, and most people mounting a local folder shouldn't have to.
- **Instead:** the entrypoint runs Hoard as whoever owns `/library` and takes over `/cache` itself. PUID/PGID still
  override that, for shares that refuse `chown`.

### `${HOARD_*}` variables in the compose files (2026-11)

- **Why not:** Portainer and other stack UIs show the file as-is, so people saw variables instead of values.
- **Instead:** literal defaults in every compose file. Everything else is set in the app.

### Build-time flags and environment variables for optional features (2026-10)

- **What:** a `WITH_BLENDER`-style build flag for Blender, and environment variables for the AI model and
  service.
- **Why not:** features need to be switched on and off without rebuilding the image or recreating the
  container.
- **Instead:** Settings in the app. Blender is downloaded on demand when it's set to Automatic, and the AI service
  and model are picked there.

## Releases

### A CHANGELOG.md (2026-10)

- **Why not:** a file to keep in step by hand, saying what the commit history already says.
- **Instead:** releases are tags only. Their notes are the commit subjects since the previous tag
  (`scripts/release-notes.sh`).

### Deleting and recreating release tags to move them (2026-10)

- **Why not:** a GitHub release whose tag is deleted can turn back into a draft. Separately, VS Code's
  background auto-fetch quietly reset local tags to the remote ones, so a later `git push` had "nothing to send".
- **Instead:** force-push the tags in place (`git push -f origin v1.0.0`), by commit hash if they might have been
  reset locally.

## Server internals

### Fixing "database is locked" at startup with `busy_timeout` (2026-11)

- **Why not:** two separate problems. `PRAGMA journal_mode = WAL` ran on each new connection before
  `busy_timeout` was set, so connections opening together failed at once. Also, SQLite skips the busy handler on a
  lock-upgrade deadlock, so no timeout helps there.
- **Instead:** WAL is switched on once, on a single connection, before the pool opens. The mode persists in the
  file, so the per-connection pragma did nothing useful anyway.

### Lowering the server's thread priority on Windows (2026-11)

- **What:** lower the priority of the heavy analysis threads, as is already done on Linux, so the UI stays
  responsive during big imports.
- **Why not:** it needs a direct Win32 call, and the workspace forbids `unsafe_code`. No safe wrapper was already
  in the dependency tree, and the server stayed responsive without it.
- **Instead:** only the Linux (Docker) build lowers priority.

### Blender 5 for model conversion (2026-10)

- **Why not:** Blender 5 removed Collada (.dae) import, which a lot of older asset packs use.
- **Instead:** Blender 4.5 LTS, downloaded and checksum-verified on demand.

## AI tagging

### Logging in with a Claude or ChatGPT subscription (2026-11)

- **Why not:** Anthropic doesn't allow third-party apps to offer claude.ai login or use subscription limits
  without its approval, and OpenAI's ChatGPT plans are similar.
- **Instead:** API keys, OpenRouter and local models. Driving a CLI the user has signed into themselves
  (`claude -p`, `codex exec`) is still open; see [roadmap.md](roadmap.md). The terms question has to be answered
  before that's built.
