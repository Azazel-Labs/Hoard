# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

## Maintainability: what's left

The largest files have been split into folder modules. The rest are big but each about one thing; splitting them
waits until they're next changed: `db/schema.rs` (1,620 lines, mostly the migrations, in order), `store_formats.rs`,
`unitymat.rs`, `sidecar.rs`, `sources.rs`, `links.rs` and `materials/models.rs` (750 to 900 lines each), and on the
web side `App.vue` (1,750), `AssetDetail.vue`, `PacksView.vue`, `skeleton2d.ts` and `AiPanel.vue` (700 to 870).

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

