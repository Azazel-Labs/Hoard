# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

## Performance in a big library

Measured against a generated library of 300,000 assets in 200 packs (`cargo test -p hoardd --lib big_library --
--ignored --nocapture`, which prints a timing for each step). Quick already: the pack list (30 ms), a pack's card
(under a millisecond), tag suggestions, Settings → Issues, sorting by name or size (a few milliseconds), and the
library's numbers, report, near copies and store files no format reads, which are kept, saved for the next start
and worked out again in the background. Still slow, worst
first:

- **The first letter typed in search: 1.6 s**, its next page 0.36 s. Most of it is ranking every match (85,000 for
  one letter) and counting them for the total. A query's ranked ids could be kept until the library changes, so the
  next page and the total come from them instead of searching again, and one or two letters could rank a capped
  number of matches.
- **The sidebar's tag counts while searching: about 0.8 s.** They no longer hold the tiles up. With no words or
  filters they are the same as the tags in use, which are already kept, so that case can reuse them; with a query
  they could be kept with its ranked ids.
- **Browsing a big pack: 0.63 s** for a pack of 120,000, with its counts beside the grid. Counts by kind are
  already kept per pack (`pack_kinds`), so the facets with no other filter can come from there.

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

## Animations as their own thing

Hoard keeps the skeleton and clips of every rigged model (`rigs`: bone names without their `mixamorig:` or
`Armature|` prefixes, and each clip's name, length and the bones it moves), from glTF, from FBX through polysmelt and
from the GLB Blender makes of a .blend. Within a pack, a file of clips with no mesh, or a copy of a character saved
for one clip, becomes an animation linked to the characters it fits nearby, and across the library characters list
"Animations that fit" and animations "Characters it fits". An animation plays on any character it fits, picked
under Plays on, and its thumbnail and tile show it halfway through the clip, with its length; a character's viewer
plays any animation that fits it. An animation downloads with the character it plays on, or as its skeleton and
clips alone, and a character with its animations. A clip whose root travels plays held in the frame over a ground
grid that scrolls the other way.

Each take in a file of many is an animation of its own, beside the file as `file@Take.glb`.

## Spine binary skeletons

Spine skeletons exported as JSON play on their atlas in Hoard's own player. Still to read:

- **Spine binary skeletons** (`.skel`): the same rig in Spine's binary export, read into the same player.

