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

Still open: whether the root travels or a clip plays in place, and whether each clip in a many-take file should be
an entry of its own in search.

## Sprite sheets and 2D animation: more formats

Atlases from TexturePacker (and Aseprite), Starling, libGDX/Spine, Cocos2d and Godot become sprite sheets, and so do
tile sheets that come with only a note of their grid (Kenney's `Tilesheet.txt`). Still to read, the most common first:

- [x] **Tiled tilesets** (`.tsx`, `.tsj`, or a tileset saved as `.json`): the image, tile size, spacing and margin
  are given exactly, so each becomes a sheet of its tiles, empty ones left out, its tiles' animations kept. Tilesets
  embedded in a `.tmx` map stay part of the map.
- [x] **Unity sprite slicing**: Asset Store packs keep each sprite's rectangle in the image's `.png.meta`. That
  `.meta` is kept in the pack and becomes the sheet, its image the page.
- [x] **DragonBones texture atlases** (`_tex.json`) as sheets.
- [ ] **DragonBones skeletons** (`_ske.json`) played on their atlas.
- [x] **Lottie / Bodymovin** (`.json`, `.lottie`): vector animations, common for UI. Played in the details panel
  with lottie-web, the thumbnail drawn by the browser as the 3D viewer's is.
- [x] **Rive** (`.riv`), played with its free web runtime, served with the app; the thumbnail drawn by the browser.
- [ ] **Spine skeletons** (`.json`, `.skel`): the atlas is read already. Playing the rig needs Spine's runtime, whose
  license only covers people who hold a Spine license, which is the open question.
- [ ] Rarer: Spriter `.scml`, Live2D `.moc3`, GameMaker `.yy`, Defold atlases.

## Images: more formats

Hoard previews PNG, JPEG, GIF, BMP, TGA, WebP, TIFF, HDR, EXR, PSD/PSB, Krita, OpenRaster, DDS, KTX and KTX2,
PowerVR, VTF, Basis, ICO/CUR, QOI, PNM, PCX, SVG, Illustrator and Aseprite. Some formats are filed as images but get no thumbnail, and some aren't recognized at
all. The most common first:

- [ ] **Source files with a preview inside**: Clip Studio (`.clip`, a SQLite database with a canvas preview),
  Procreate (`.procreate`, a zip with a thumbnail), Affinity Photo and Designer (`.afphoto`, `.afdesign`) and EPS
  (a TIFF preview in its header, when it has one). All of these are recognized now, but none gets a picture.
- [ ] **GIMP** (`.xcf`): recognized, but nothing is drawn. The format is documented, and Hoard would composite
  the layers itself, as it does for Krita.
- [ ] **AVIF, HEIC and JPEG XL**: newer formats, rare in asset packs so far. JPEG XL has a pure-Rust decoder.
  AVIF and HEIC would need dav1d and libheif, or could go through ffmpeg once it's downloaded.
- [x] **Small ones**: ICO/CUR, QOI and PNM (`.ppm`, `.pgm`, `.pbm`) through the image library, PCX read by Hoard,
  and KTX 1, PowerVR and Valve's VTF textures decoded like DDS.
- [ ] Camera raw files (`.dng`, `.cr2`, `.nef`) by their embedded JPEG. These are reference photos rather than
  game art.

## Sounds: more formats

Hoard decodes WAV, FLAC, MP3, OGG Vorbis, AIFF and ADPCM itself, AAC/M4A, Opus, WMA, CAF, WavPack, APE and more through
ffmpeg, and plays MOD, XM, S3M, IT and MIDI. What's missing is mostly containers rather than codecs:

- [ ] **SFZ instruments and SF2 SoundFonts**: a set of samples and how they map to notes, common in music packs. One
  asset, its samples kept with it, its preview a few notes played on it.
- [ ] **Opus without ffmpeg**: a built-in decoder, so Opus plays before ffmpeg is downloaded, or without it.
- [ ] More tracker and chiptune formats (`.mptm`, `.umx`, `.nsf`, `.vgm`).
- [ ] FMOD and Wwise banks (`.bank`, `.fsb`, `.wem`): found in game files more than in asset packs.

## Search sounds by what they sound like

So that "find me a sound that goes WHOOOM", typed or asked through MCP, finds deep whooshes. In layers, each useful
on its own:

- [x] **Measured features**: each sound's duration, envelope (a sharp attack or a swell, how long the tail is),
  brightness, how noisy or tonal it is, whether its pitch rises or falls, its loudness and whether it loops cleanly.
  Kept with the sound, shown in its details, filterable in search and the API.
- [x] **Onomatopoeia**: a word read as the shape of a sound and scored against those features. A plosive start
  (b, p, k, t) is a sharp attack; long or repeated vowels are length; "oo"/"o" low and dark, "ee"/"i" high and
  bright; "sh", "s", "wh", "f" are noise and air; a trailing "m" or "n" a hum that fades; capitals are loud.
  "WHOOOM": airy start, long low middle, fading hum, loud.
- [x] **CLAP embeddings**: sounds and text in one space, as CLIP does for images, so "glassy UI blip" or "distant
  thunder roll" find sounds whatever they're called. Off until an admin turns it on, like CLIP: LAION's HTSAT
  unfused model, 614 MB, Apache-2.0.
- [x] **MCP**: `search_sounds` — the calling AI passes the raw word as `imitation` and its own reading of it ("a
  deep whoosh, about a second, rising") as `shape`, which overrides that part of Hoard's reading — returning each
  sound's measured features, the same in words and a link to listen, plus `describe_sound(id)` and `sounds_like(id)`.
- [x] **Search by imitation**: make the noise into the microphone, and sounds with that envelope and pitch shape are
  found.

## Ideas

- Embeddings for "find more like this" and near-duplicates (SigLIP 2 for images, or a multimodal embedding API).
- BPM and key detection for music (Essentia).
- Previews for Substance source files (`.sbs`, `.spp`).
- An Unreal-friendly export layout.
