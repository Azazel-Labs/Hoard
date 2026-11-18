# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

## Written noises outside sound searches

Searching sounds (Kind is Audio), a word in the search field also finds the sounds shaped like it ("whooom", "pew"),
and the microphone is in the search field. Still open: whether a search not limited to sounds should notice a written
noise by itself (a word that matches nothing and spells like one) and offer the sounds shaped like it.

## Maintainability: what's left

The largest files have been split into folder modules. The rest are big but each about one thing; splitting them
waits until they're next changed: `db/schema.rs` (1,620 lines, mostly the migrations, in order), `store_formats.rs`,
`unitymat.rs`, `sidecar.rs`, `sources.rs`, `links.rs` and `materials/models.rs` (750 to 900 lines each), and on the
web side `App.vue` (1,750), `AssetDetail.vue`, `PacksView.vue`, `skeleton2d.ts` and `AiPanel.vue` (700 to 870).

## Store details filled in without asking

"Store details" already searches Fab, itch.io, Kenney and OpenGameArt by the pack's name and lists likely matches to
pick from. Still to come:

- Filling them in when a pack is added, once a match is sure enough: an exact title from the seller named in the
  README, say. How sure is sure enough is the open question.
- The Unity Asset Store and Humble Bundle only get search links: one searches in the page's script, the other turns
  away non-browser clients.

## AI tagging with a Claude subscription

A ChatGPT plan can already be used for tagging ("Sign in with ChatGPT", see
[DEPLOYMENT.md](DEPLOYMENT.md#with-a-chatgpt-plan)). Still to find out there is how far a library's worth of
tagging gets against the plan's weekly cap. Claude is harder (checked 2026-11-12):

- **Claude: only by running Claude Code itself.** Anthropic doesn't allow third-party apps to offer claude.ai login,
  collect or store the session tokens, or send requests with Pro or Max credentials, so Hoard can't do the OAuth
  itself. Running the unmodified `claude` binary, signed into by the user through
  Anthropic's own flow, is allowed. Hoard could install it on demand like Blender and run `claude -p` with the
  thumbnails. Plan limits assume ordinary individual use, so tagging a large library this way may run into them.
  Policy: <https://code.claude.com/docs/en/legal-and-compliance>

## Animations as their own thing

Hoard keeps the skeleton and clips of every rigged model (`rigs`: bone names without their `mixamorig:` or
`Armature|` prefixes, and each clip's name, length and the bones it moves), from glTF, from FBX through polysmelt and
from the GLB Blender makes of a .blend. Within a pack, a file of clips with no mesh, or a copy of a character saved
for one clip, becomes an animation linked to the characters it fits nearby. What's left:

- **Link across packs by skeleton.** An animation fits any character whose skeleton has the bones it moves (same
  names once the prefixes are off), so a Mixamo clip fits every Mixamo character and an Unreal Mannequin clip every
  Mannequin. Characters list "Animations that fit"; animations list "Characters it fits". The same pack comes first.
- **Show them as animations.** An animation's details play it in a loop on its character, with a picker to put it
  on any other character it fits; three.js plays a clip on any skinned mesh whose bones have the same names. Its
  thumbnail is drawn mid-clip, not in the rest pose, so a grid of them isn't a grid of the same pose, and its tile
  shows the length. A character's details list its animations, each played in the viewer when picked.
- **Download them either way.** An animation comes with or without its character's mesh, and a character with or
  without its animations, as `?animations=1` does now.

A clip whose root travels (a walk, a run, a jump forward) is shown traveling, without leaving the frame: the camera
follows the character, and a ground grid beneath it scrolls the other way, so root motion and a clip made to play in
place look different at a glance. A clip that stays put gets no grid.

Still open: whether each clip in a many-take file should be an entry of its own in search.

## Spine binary skeletons

Spine skeletons exported as JSON play on their atlas in Hoard's own player. Still to read:

- **Spine binary skeletons** (`.skel`): the same rig in Spine's binary export, read into the same player.

