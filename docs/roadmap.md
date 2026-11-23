# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

## Performance in a big library

Measured against a generated library of 300,000 assets in 200 packs (`cargo test -p hoardd --lib big_library --
--release --ignored --nocapture`, which prints a timing for each step; last run 2026-11-22). Quick already: the pack
list (56 ms), a pack's card (under a millisecond), tag suggestions, Settings → Issues, sorting by name or size and a
search's next pages (a millisecond or two, now that a search keeps what it found in order), and the library's
numbers, report, near copies and store files no format reads, which are kept, saved for the next start and worked
out again in the background. Still slow, worst first:

- **The first letter typed in search: 1.6 s.** Its next page is quick now, but the first still ranks every match
  (279,000 for one letter) and counts them for the total. One or two letters could rank a capped number of matches.
- **The sidebar's tag counts: about 0.75 s browsing everything and 1 s for the first letter typed.** They don't
  hold the tiles up, but browsing everything takes 1.5 s with them and under a millisecond without. With no words
  or filters they are the same as the tags in use, which are already kept, so that case can reuse them; with words
  they come from what the search kept, but counting the tags of a quarter of a million matches still takes a
  second.
- **Browsing a big pack: 0.7 s** for a pack of 120,000, with its counts beside the grid; its top folder 0.65 s.
  Counts by kind are already kept per pack (`pack_kinds`), so the counts with no other filter can come from there.
- **Words and pictures together: 0.6 s**, 0.4 s without the counts: what looks like a picture is ranked for every
  match of the words.

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

