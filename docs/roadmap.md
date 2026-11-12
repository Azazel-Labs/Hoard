# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

Ideas that were tried and dropped are in [deadends.md](deadends.md); check there before picking one back up.

## Library locations: what's left

The first-run setup and library locations are in. A server can keep files in several managed locations, routed by
asset kind, and move stored files between them. It can also index watched folders in place. Still to come:

- Watched files edited in place: a rescan picks up new and removed folders and files, but a file changed where it sits
  keeps its old hash and preview until the pack is re-analyzed.
- Renamed watched folders: a renamed folder is a new pack, and the old one (with its tags and notes) goes three days
  later. Matching the two by their files would carry the curation over.
- Cheaper rescans: every 15 minutes, each watched pack's whole tree is walked. A big library on a slow share could
  check folder modification times first.

## Store details looked up automatically

When a pack is added, search a few well-known stores by the pack's name (Fab, the Unity Asset Store, itch.io,
Humble Bundle, Kenney, OpenGameArt and so on) and prefill the pack's store details: store, store page, license tier
and description. It builds on the store-page fetching that already reads a store page once you've given its link.
The open question is how sure a match has to be before it's filled in without asking; a list of likely matches to
pick from is the safe start.

## Settings → Maintenance

One place for the jobs that look after a library: back up now, scrub (re-hash every file), re-run analysis across
all packs, rebuild thumbnails, clear the cache, and remove files nothing refers to any more. Most of these exist as
API calls already; this gives them buttons, progress and a history.

## AI tagging with a ChatGPT or Claude subscription

People without an API key would like to use the subscription they already pay for. The two providers differ
(checked 2026-11-12):

- **ChatGPT: supported, and the better first step.** Since September 2026 OpenAI offers "Sign in with ChatGPT" with
  plan usage, self-serve for open-source and locally hosted apps, which Hoard is. It's OAuth with PKCE; requests go
  to the Responses API and draw on the user's plan, with a weekly cap per app that the user sets in ChatGPT. Open
  questions:
  - The sign-in redirects to a loopback address, which is the browser's machine, not the server on a NAS. The
    usual fix is letting the user paste the redirected URL back into Hoard.
  - Whether image input is allowed. Tagging sends thumbnails and contact sheets.
  - The preview limits and how far a library's worth of tagging gets against the weekly cap.
  - Docs: <https://developers.openai.com/siwc/token-sharing-open-source>
- **Claude: only by running Claude Code itself.** Anthropic doesn't allow third-party apps to offer claude.ai login,
  collect or store the session tokens, or send requests with Pro or Max credentials, so Hoard can't do the OAuth
  itself (see [deadends.md](deadends.md)). Running the unmodified `claude` binary, signed into by the user through
  Anthropic's own flow, is allowed. Hoard could install it on demand like Blender and run `claude -p` with the
  thumbnails. Plan limits assume ordinary individual use, so tagging a large library this way may run into them.
  Policy: <https://code.claude.com/docs/en/legal-and-compliance>

## Imports without the staging copy

An upload is unpacked into a staging folder, then each file is hashed and copied again into the library. For packs
of many small files the cost is creating files, not reading them: the Kenney all-in-one (92,000 files in a 529 MB
zip) took nine minutes just to unpack on Windows with Defender scanning each new file, where a plain single-threaded
extract manages about 150 files a second on the same machine. Reading entries straight out of the archive into the
library would create each file once instead of twice. Nested archives, and analysis that needs neighbouring files
(a model next to its textures), are what make it more than a small change.

## Lower priority for heavy work on Windows

Unpacking, analysis and thumbnail rendering run at a lower priority on Linux so the server and the machine stay
responsive. On Windows they run at normal priority, because setting thread priority there needs a system call and
the code base forbids `unsafe`. A small, well-kept crate (such as `thread-priority`) would do it.

## Ideas

- Embeddings for "find more like this" and near-duplicates (CLAP for audio, SigLIP 2 for images, or a multimodal
  embedding API).
- BPM and key detection for music (Essentia).
- Previews for more source formats (GIMP, Substance).
- An Unreal-friendly export layout.
