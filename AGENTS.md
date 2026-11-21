# Hoard project guidance

## Visual output needs a human look before it's done

When a change makes Hoard produce a picture it hasn't produced before (a new thumbnail source, a new viewer, an
animation player, a colour conversion, a converted texture), passing tests and good-looking numbers do not mean
it's right. Before calling the work finished:

- Build a before/after page (an artifact or a contact sheet) showing real files from each affected path. Include
  known answers: assets whose correct look is certain from their name or what they are, such as a red icon, a flat
  normal map (lavender), a sky (blue) or a lookup table whose ramps run a known way.
- Show it to the user and wait for their verdict. Say what wasn't checked, such as a user's own files that weren't
  available.
- Look at the images yourself as well. Don't rely on a score that compares one output with another, because both
  can be wrong in the same way.

Why: both of these passed their tests and still looked wrong to the first person who saw them.

- **Animation playback.** Root motion, rotation keys and the followed node were "fixed" several times over before
  anyone watched the result play.
- **Unreal thumbnails came out blue.** Unreal stores thumbnail and PNG texture-source pixels with red and blue
  swapped (BGRA memory compressed as RGBA). Hoard showed them as stored, so red assets turned blue. Every check
  compared Hoard's output with other Hoard output, so they all agreed.

## Never lose where a file came from

Any file Hoard makes in place of a user's file (a conversion, a rule, an editor export) must record, for good, what
it came from and what made it, even after the original is discarded and even if nothing shows the record:

- The original's hash goes in `asset_origin.orig_hash` and is never cleared. `blob_hash` is only the original
  while it's kept.
- What made it goes in `asset_origin.converter`: the code, its version or revision, and which way it went
  (`hoard-unreal r2: PNG source, red and blue put right`). A second conversion is added after `; then `, not
  written over the first. Free-text `notes` are not a record.
- When what a converter writes changes, raise its revision (`hoard_unreal::convert::REVISION`; library versions
  named in `reformat.rs`, which a test checks against Cargo.lock), so files the old code made can be found and
  made again.

Why: when the Unreal converter turned out to have swapped red and blue in PNG textures, nothing said which files it
had made or which way. Where the original was discarded even its hash was gone. Which PNGs to fix had to be guessed
from notes and from how the bytes were compressed. See "What a converted file came from" in
[docs/roadmap.md](docs/roadmap.md) for what's still missing.
