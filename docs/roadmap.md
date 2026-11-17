# Roadmap

What's planned or being weighed, roughly in order. Nothing here is promised; each item says what's settled and what
isn't yet.

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

Animation packs often come as one file per animation, each a whole character: `Knight_Walk.blend`, `Knight_Run.blend`,
`Knight_Idle.blend`, all the same rig and mesh with one action each. Hoard shows them as that many copies of the same
model. Only `Knight@Run.fbx` naming and Unity/Unreal references link a clip to its model today.

The plan:

- **Read the rig and clips of every rigged model.** glTF has them, polysmelt reads FBX skins and takes into the same
  form, and a .blend's are in the GLB Blender makes of it. For each file, keep:
  - a fingerprint of the skeleton: its bone names, with prefixes like `mixamorig:` and `Armature|` taken off, and
    which is whose parent
  - a fingerprint of the mesh: vertex and triangle counts and bounds
  - each clip's name, length and the bones it moves, and whether the root travels or it plays in place
- **Decide what each file is, pack by pack.** Files with the same skeleton and mesh are one character. The one with
  no clip, or else the one whose name is the others' common part (`Knight`), stays a model. The rest become
  animations of it, linked as they are now. A file with a skeleton and clips but no mesh is already an animation.
  A file with one mesh and many takes stays one model with its clips listed.
- **Link across packs by skeleton.** An animation fits any character whose skeleton has the bones it moves (same
  names once the prefixes are off), so a Mixamo clip fits every Mixamo character and an Unreal Mannequin clip every
  Mannequin. Characters list "Animations that fit"; animations list "Characters it fits". The same pack comes first.
- **Show them as animations.** An animation's details play it in a loop on its character, with a picker to put it
  on any other character it fits; three.js plays a clip on any skinned mesh whose bones have the same names. Its
  thumbnail is drawn mid-clip, not in the rest pose, so a grid of them isn't a grid of the same pose, and its tile
  shows the length. A character's details list its animations, each played in the viewer when picked.
- **Download them either way.** An animation comes with or without its character's mesh, and a character with or
  without its animations, as `?animations=1` does now.

Still open: how sure a mesh match has to be (the same character can be saved with different modifiers), and whether
each clip in a many-take file should be an entry of its own in search.

## Ideas

- Embeddings for "find more like this" and near-duplicates (CLAP for audio, SigLIP 2 for images, or a multimodal
  embedding API).
- BPM and key detection for music (Essentia).
- Previews for more source formats (GIMP, Substance).
- An Unreal-friendly export layout.
