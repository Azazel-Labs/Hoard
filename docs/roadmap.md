# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

## Sounds in the one search field

The sidebar's "Sounds like" box (a written noise, or the microphone) and its music filters (tempo and key) sit above
Kind and Subject, taking room on every search, though they only matter for sounds. Settled:

- **Music filters only for sounds.** Tempo and key move below Kind and the tag groups, and only show while the search
  is limited to sounds (Kind is Audio), or while one of them is set, so it can be cleared.
- **The microphone in the search field.** The search field gets the microphone button; a recorded noise shows as
  "Your noise" in the field, and searching anything else drops it.
- **Written noises in the search field too.** With the search limited to sounds, the words in the search field also
  find sounds shaped like them ("whooom", "pew"), beside the sounds whose names, tags or sound match the words, and the
  sidebar box goes. Outside sounds the words search as they do now.

Still open: whether a search not limited to sounds should notice a written noise by itself (a word that matches
nothing and spells like one) and offer the sounds shaped like it.

## Text and document thumbnails

Text, JSON and PDF files show a generic icon. Planned:

- **Text and code as a silhouette.** Each line is drawn as a bar from the left edge, indented as the line is and as
  long as it is, colored by what's in it: keywords, strings, numbers, comments and punctuation each a color of their
  own, on a dark ground. A file reads like a speckled gene sequence: its shape shows its indentation and density, its
  colors what kind of text it is. JSON gets the same, colored by keys, strings, numbers and brackets.
- **PDFs by their first page**, drawn the way an Illustrator file's is.

## Maintainability: what's left

The largest files have been split into folder modules. The rest are big but each about one thing; splitting them
waits until they're next changed: `db/schema.rs` (1,620 lines, mostly the migrations, in order), `store_formats.rs`,
`unitymat.rs`, `sidecar.rs`, `sources.rs`, `links.rs` and `materials/models.rs` (750 to 900 lines each), and on the
web side `App.vue` (1,750), `AssetDetail.vue`, `PacksView.vue`, `skeleton2d.ts` and `AiPanel.vue` (700 to 870).

## Library locations: what's left

The first-run setup and library locations are in. A server can keep files in several managed locations, routed by
asset kind, and move stored files between them. It can also index watched folders in place. Still to come:

- Watched files edited in place: a rescan picks up new and removed folders and files, but a file changed where it sits
  keeps its old hash and preview until the pack is re-analyzed.
- Renamed watched folders: a renamed folder is a new pack, and the old one (with its tags and notes) goes three days
  later. Matching the two by their files would carry the curation over.
- Cheaper rescans: every 15 minutes, each watched pack's whole tree is walked. A big library on a slow share could
  check folder modification times first.

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

