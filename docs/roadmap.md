# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

## Performance in a big library

Measured against a generated library of 300,000 assets in 200 packs, with names drawn from 26,000 words and a tag
tree over its tags (`cargo test -p hoardd --lib big_library --release -- --ignored --nocapture`, which prints a
timing for each step; last run 2026-11-22). Quick already: browsing everything with its counts and sidebar (9 ms), a
big pack (3 ms), a broad tag picked with everything under it (6 ms), a search by words (16 ms), the first letter typed
(0.12 s, nearly all of it the search index finding 285,000 matches), the pack list (46 ms), a pack's card (under a
millisecond), tag suggestions, Settings → Issues, sorting by name or size and a search's next pages (a millisecond or
two), and the library's numbers, report, near copies and store files no format reads, which are kept, saved for the
next start and worked out again in the background. What a search tells assets apart by (kind, pack, archived,
supporting, newest first) is kept for every asset and searched in memory, and the sidebar's tags are counted from each
tag's assets; both are kept as tags, ratings and thumbnails change, and put right in place as assets are added,
removed, moved or change kind (browsing again during an import takes 10 to 15 ms, not the 0.4 s reading them all
again took). Still slow, worst first:

- **Words and pictures together: 0.3 s**: what looks like a picture is ranked for every match of the words.
- **A broad tag typed: 0.26 s**: its 140,000 matches are ranked by how well they match.
- **A pack's folders: 0.13 to 0.18 s**: every asset in the pack is looked at to find what's in the folder.
- **The first letter typed again after any change to the library: 0.12 s**, as the first time: an asset's tags
  changing, say, sends the search index looking for all its matches again.

## What a converted file came from

A file Hoard converted (an Unreal texture to PNG, a WAV to FLAC) has to say what it came from for good, even once
the original is discarded. Until migration 90 it didn't reliably: the original's hash was dropped with the original,
and what made the file was free-text notes that didn't always say. So when the Unreal converter turned out to have
swapped red and blue, which PNGs it had swapped had to be worked out from clues. Since then, `asset_origin` keeps the
original's hash for good (`orig_hash`) and which converter at which revision made the file and how (`converter`,
e.g. `hoard-unreal r2: PNG source, red and blue put right`). Still missing:

- **Originals discarded before migration 90 are only a path and notes.** Their hash is gone unless the pack is added
  again.
- **The record isn't shown.** The asset page shows the original's path, class, engine and notes, but not its hash or
  what converted it.
- **Deleting a converted asset deletes the record** (`asset_origin` goes with its asset). A small tombstone would
  keep that the pack once had `T_Jar.uasset`, with its hash, and what came of it.
- **Thumbnails and other derivatives don't record what drew them.** A fix to a thumbnail reader (the same red/blue
  swap) has to re-read every pack of that format, because which thumbnails the old reader drew isn't known.

## Store details for the Unity Asset Store and Humble Bundle

Store details searches Fab, itch.io, Kenney and OpenGameArt, and a pack whose files name its listing gets it filled
in without asking. The Unity Asset Store and Humble Bundle only get search links: one searches in the page's script,
the other turns away non-browser clients.

## AI tagging with a Claude subscription

A ChatGPT plan can already be used for tagging ("Sign in with ChatGPT", see
[DEPLOYMENT.md](DEPLOYMENT.md#with-a-chatgpt-plan)). Still to find out there is how far a library's worth of
tagging gets against the plan's weekly cap. Claude is harder:

- **Claude: waiting on a ruling from Anthropic.** Its policy doesn't let third-party apps offer claude.ai login,
  collect or store its tokens, or route requests through Pro or Max plans on their users' behalf, and running Claude
  Code in a product needs its Commercial Terms. A person signing in to the unmodified `claude` binary themselves is
  allowed, but whether Hoard may then run `claude -p` on the thumbnails for them isn't spelled out, so it waits until
  Anthropic says (<https://www.anthropic.com/contact-sales>). Tagging with an Anthropic API key works now.
  Policy: <https://code.claude.com/docs/en/legal-and-compliance> (checked 2026-11-18)

## Spine binary skeletons

Spine skeletons exported as JSON play on their atlas in Hoard's own player. Still to read:

- **Spine binary skeletons** (`.skel`): the same rig in Spine's binary export, read into the same player.

