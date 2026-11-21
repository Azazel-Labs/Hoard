// What's in the library, as the server sends it: assets and what they link to, packs, collections, projects,
// copies and searches. Types mirror crates/hoard-core.

export type AssetKind =
  | 'texture' | 'vector' | 'sheet' | 'audio' | 'model' | 'animation' | 'font' | 'shader'
  | 'material' | 'video' | 'document' | 'data' | 'other'

export type TagSource = 'meta' | 'rule' | 'ai' | 'human'

export interface Tag { tag: string; source: TagSource; confidence: number }

export type LinkRel = 'texture' | 'buffer' | 'material' | 'animation' | 'library' | 'guess' | 'sidecar' | 'variant' | 'sprite' | 'format' | 'sample'

export interface AssetLink {
  id: string
  name: string
  path: string
  kind: AssetKind
  rel: LinkRel
  has_thumb: boolean
  /** File size in bytes. */
  size?: number
}

export interface Asset {
  id: string
  pack_id: string
  pack_name: string
  name: string
  path: string
  ext: string
  kind: AssetKind
  category: string | null
  /** A person's own description. */
  description: string | null
  /** What the file says about itself (BWF, RIFF INFO, iXML). */
  source_description: string | null
  /** Written by the AI tagger. */
  ai_description: string | null
  hash: string
  size: number
  has_thumb: boolean
  /** Who made the thumbnail of a model: null for the server, `browser` for a browser's 3D viewer with every
   *  texture, `browser-partial` for one with some missing. A better one is never replaced by a worse one. */
  thumb_source?: ThumbSource | null
  /** Changes whenever the thumbnail does; `thumbUrl` puts it in the URL so a new thumbnail isn't hidden behind the
   *  browser's cached copy of the old one. */
  thumb_rev?: string | null
  /** An animation has a short looping movie of it playing (`motionUrl`), made while the Moving thumbnails setting
   *  is on. */
  has_motion?: boolean
  /** An animation's length in seconds: its longest clip that isn't a pose. */
  clip_secs?: number
  meta: Record<string, unknown>
  tags: Tag[]
  created_at: string
  updated_at: string
  /** When it went to the archive (its own, or its pack's). */
  archived_at?: string | null
  /** When a person said its details are right. Rules and the AI leave a reviewed asset's tags alone. */
  reviewed_at?: string | null
  /** 1 to 5. */
  rating?: number | null
  /** Its own license, when it isn't its pack's (`pack_license`). */
  license?: string | null
  /** Where it came from. */
  source_url?: string | null
  /** Detail view only: its pack's license. */
  pack_license?: string | null
  /** Detail view only: the library's own fields, by name. */
  fields?: Record<string, string>
  /** Detail view only: tags a rule or the AI gave it that a person took off, which stay off. */
  rejected_tags?: string[]
  /** Detail view only. */
  links: AssetLink[]
  linked_from: AssetLink[]
  derivatives: string[]
  /** Numbered takes of the same thing (whoosh_01..12); `members` lists the others. */
  family?: { key: string; size: number; members: AssetLink[] } | null
  /** Byte-identical files elsewhere in the library. */
  duplicates?: Duplicate[]
  /** The same picture, sound or model elsewhere at another size, rate or format; the best of them marked. */
  near_copies?: NearCopy[]
  /** A character's animations across the library that fit its skeleton, or the characters an animation fits; the
   *  same pack first, at most 200. */
  fits?: Fit[]
  /** How many fit in all, when that's more than `fits` lists. */
  fits_total?: number | null
  /** Collections this asset is in. */
  collections?: CollectionRef[]
  /** What this file was converted from (an Unreal package). */
  origin?: AssetOrigin | null
  /** For an Unreal package not converted yet: why. */
  unreal?: UnrealStatus | null
  /** Other formats the server can convert this asset to on download. */
  formats?: FormatOption[]
  /** Dominant colors of the thumbnail, most of the picture first. */
  palette?: Swatch[]
  /** For a material made from texture maps: its maps. */
  material?: MaterialInfo | null
  /** Materials made from texture maps that this texture is one of, or this model uses. */
  materials?: MaterialInfo[]
  /** Detail view of a model only: how its pack's models are turned, when they are. */
  facing?: Facing
}

/** A material made from texture maps that share a name (Rock_BaseColor.png, Rock_Normal.png, …). */
export interface MaterialInfo {
  id: string
  name: string
  path: string
  has_thumb: boolean
  /** By role (color first), each role's largest first. */
  maps: MaterialMap[]
  /** How much ground one repeat of a tiling material covers, [width, height] in meters, when the store says. */
  physical_size?: [number, number]
  /** How far its height map reaches, in meters, when the store says. */
  displacement?: number
  /** The settings its maps are drawn with, when it was read from an engine's material (a Unity .mat). */
  look?: MaterialLook
  /** On a model's materials: the model's material slots it goes in (a Unity .mat in an FBX's slot). */
  slots?: string[]
}

/** An engine material's settings: colors are linear, factors 0..1. */
export interface MaterialLook {
  base_color?: [number, number, number, number]
  metallic?: number
  roughness?: number
  emissive?: [number, number, number]
  alpha_mode?: 'mask' | 'blend'
  alpha_cutoff?: number
  normal_scale?: number
  occlusion_strength?: number
  uv_scale?: [number, number]
  uv_offset?: [number, number]
}

export type MapRole =
  | 'base_color' | 'normal' | 'normal_dx' | 'roughness' | 'gloss' | 'metalness' | 'ao' | 'orm' | 'mra' | 'rma'
  | 'height' | 'bump' | 'cavity' | 'specular' | 'opacity' | 'emissive' | 'translucency'

/** One texture of a material and what it's for. */
export interface MaterialMap {
  /** The texture's asset id. */
  id: string
  name: string
  path: string
  ext: string
  role: MapRole
  /** "2K", "512": a set that comes in several sizes has the same role more than once. */
  resolution: string | null
  width: number | null
  height: number | null
  has_thumb: boolean
  size: number
}

export interface Swatch { hex: string; share: number }

export type ThumbSource = 'browser' | 'browser-partial'

export interface FormatOption { format: string; label: string; resizable: boolean }

export interface AssetOrigin {
  path: string
  ext: string
  original_kept: boolean
  class: string | null
  engine: string | null
  dependencies: string[]
  /** `native` (Hoard's reader), `editor` (Unreal Editor export) or `rule` (a conversion rule). */
  method: string
  notes: string | null
  converted_at: string
  /** A conversion rule's original: when it's deleted, until which the file can be put back. */
  kept_until?: string
}

/** Files of the formats in `from` are converted to `to` (Settings → Adding packs → Conversion rules). */
export interface ConversionRule {
  on: boolean
  from: string[]
  to: string
  /** Turned as they're written to stand Y up and face +Z, the way their pack's models are shown. */
  upright: boolean
}
export interface ConversionRules { rules: ConversionRule[]; keep_days: number }

/** What saving some conversion rules would do to files already in the library. */
export interface ConversionPreview {
  files: number
  packs: number
  /** Files each rule would convert, by the rule's index. */
  by_rule: number[]
  /** Of `files`, those in packs still being added, converted once they're done. */
  busy: number
  /** Files a rule matches but leaves as they are, by why. */
  skipped: Partial<Record<'watched' | 'unity' | 'used' | 'beside' | 'blender', number>>
  /** The first files, by pack and path, and what each becomes. */
  sample: { id: string; pack_id: string; pack: string; path: string; to: string }[]
}

export interface UnrealStatus { status: 'needs_editor' | 'unsupported' | 'failed'; class: string | null; reason: string }

export interface UnrealPackStatus {
  packages: number
  converted: number
  originals_kept: number
  needs_editor: number
  unsupported: number
  failed: number
  pending: number
  reasons: [string, number][]
}

export interface Duplicate { id: string; name: string; pack_id: string; pack_name: string; path: string }

/** One of a set of near copies: the same picture, sound or model at another size, rate or format. */
export interface NearCopy {
  id: string
  name: string
  pack_id: string
  pack_name: string
  path: string
  ext: string
  size: number
  width?: number
  height?: number
  sample_rate?: number
  bit_depth?: number
  /** The one to keep: biggest, lossless, most complete. */
  best: boolean
}

/** An animation that plays on a character, or a character an animation plays on. */
export interface Fit {
  id: string
  name: string
  path: string
  pack_id: string
  pack_name: string
  kind: AssetKind
  has_thumb: boolean
  thumb_rev?: string | null
  /** An animation's length in seconds. */
  secs?: number | null
}

/** A set of near copies, the best first; `wasted` is what the rest take. */
export interface CopyGroup { wasted: number; copies: NearCopy[] }

/**
 * A page of the library's sets of near copies (`GET /api/duplicates`), the most wasted first. `generation` says which
 * working-out of the sets it's from; `refreshing`, that the library has changed since and they're being worked out again.
 */
export interface CopyGroups { groups: CopyGroup[]; total: number; wasted: number; generation: number; refreshing: boolean }

/** A hand-picked set of assets across packs ("Forest level props"). */
/** A project that `hoard sync`s from here (`GET /api/projects`). */
export interface Project {
  id: number
  name: string
  synced_at: string
  /** Who reported its last sync. */
  synced_by?: string | null
  /** How many assets it has. */
  assets: number
}

/** A project using some of the assets asked about (`POST /api/usage`). */
export interface ProjectUse {
  id: number
  name: string
  synced_at: string
  synced_by?: string | null
  /** How many of them it has. */
  assets: number
  /** A few of them, and where the project keeps them. */
  examples: { asset_id: string; name: string; path: string }[]
}

/** What to ask `/api/usage` about: assets, whole packs, or everything in the archive. */
export interface UsageItems { packs?: string[]; assets?: string[]; archived?: boolean }

export interface Collection {
  id: string
  name: string
  description: string | null
  asset_count: number
  total_bytes: number
  preview: string[]
  /** Kind of each preview asset; font thumbnails are tinted to the theme. */
  preview_kinds?: string[]
  created_at: string
  updated_at: string
}
export interface CollectionRef { id: string; name: string }

/** What the seller says about a pack; fed to the LLM tagger with every file. */
/** A store page that may be where a pack came from (`GET /api/packs/{id}/store-matches`). */
export interface StoreMatch {
  store: string
  title: string
  url: string
  author?: string
  summary?: string
  thumb?: string
  price?: string
  /** The license the store sells it under ("Fab Standard License", "CC BY 4.0", "CC0"). */
  license?: string
  /** 0 to 1: how well it matches the name and the clues in the pack's files. */
  score: number
  /** Why it ranks where it does, beyond its title ("Linked in the pack's README"). */
  why?: string[]
}

export interface StoreMatches {
  /** When the stores were searched (a lookup in the background keeps its results). */
  searched_at: string
  query: string
  /** The other searches made, from pieces of the name and the pack's files. */
  also: string[]
  /** What the pack's files gave away ("Seller: Marco Maria Rossi (README)"). */
  clues: string[]
  matches: StoreMatch[]
  /** Stores that can only be searched in the browser. */
  links: { store: string; url: string }[]
  /** Stores whose search failed this time. */
  failed: string[]
}

export interface PackContext {
  store: string | null
  store_url: string | null
  store_text: string | null
  license_tier: string | null
  /** README text captured at import (read-only). */
  readme: string | null
  /** Who made or sells it, for credits. */
  seller: string | null
  /** The license the store sells it under, for credits. */
  store_license: string | null
  /** When the store page was found and attached automatically (read-only). */
  store_auto: string | null
}

export type PackStatus = 'uploading' | 'ingesting' | 'analyzing' | 'ready' | 'failed'

export interface Pack {
  id: string
  name: string
  status: PackStatus
  source: string | null
  license: string | null
  description: string | null
  tags: string[]
  asset_count: number
  total_bytes: number
  error: string | null
  files_done: number
  files_total: number
  jobs_pending: number
  created_at: string
  /** When any of its assets last changed, or `created_at` while it has none. */
  updated_at: string
  store: string | null
  store_url: string | null
  license_tier: string | null
  /** Up to four asset ids with thumbnails, for the cover mosaic. */
  preview: string[]
  /** Kind of each preview asset; font thumbnails are tinted to the theme. */
  preview_kinds?: string[]
  /** Unreal packages still in package form, and assets converted from them. */
  unreal_packages?: number
  unreal_converted?: number
  /** When it went to the archive. */
  archived_at?: string | null
  /** Assets of each kind in it. */
  kinds?: Partial<Record<AssetKind, number>>
  /** How its models are turned to face forward and stand up. */
  facing?: Facing
}

/** How a pack's models are turned, in quarter turns: `turn` counterclockwise seen from above, then `tilt`, the top
 *  tipped toward the viewer (applied first). Thumbnails and the 3D viewer show them turned; downloads are as made. */
export interface Facing {
  turn: number
  tilt: number
}

export interface Facet { value: string; count: number }

/** A tag offered while typing one; `alias` is the old name typed, when the tag is what it was merged into. */
export interface TagSuggestion { tag: string; count: number; alias?: string }
/** What `/api/tags/suggest` offers for what's typed into a tag box. */
export interface TagSuggestions {
  /** The best match first, grouped by how well they match and then by how much they're used. */
  matches: TagSuggestion[]
  /** What's typed as it would be saved; null for nothing that makes a tag. */
  normalized: string | null
  /** The tag it would land on is in use already. */
  exists: boolean
  /** When it'd be a new tag: tags in use it's probably meant to be. */
  near: TagSuggestion[]
}

/** What a tag's page shows (`/api/tags/info`). */
export interface TagInfo {
  tag: string
  /** How many assets have it. */
  count: number
  description: string | null
  /** The asset whose picture stands for the tag. */
  image_id: string | null
  /** Old names merged into it. */
  aliases: string[]
  /** When the tag asked for is an old name: the tag it was merged into. */
  merged_into?: string
}

export interface SearchResult {
  items: Asset[]
  total: number
  kinds: Facet[]
  tags: Facet[]
  /** Matches left out because `hide_supporting` was on (glTF buffers, .mtl files). */
  supporting_hidden: number
  /** Browsing a folder of a pack: the folders in it, each with how many matches are under it. */
  folders?: Facet[]
}

export interface Stats {
  assets: number
  packs: number
  blobs: number
  blobs_bad: number
  stored_bytes: number
  logical_bytes: number
  jobs_pending: number
  llm_enabled: boolean
  storage: string
  kinds: Facet[]
  /** What's in the archive: whole packs, and assets archived on their own. */
  archived_packs?: number
  archived_assets?: number
  duplicate_assets: number
  version: string
  /** ffmpeg: its version line, `downloading`, `looking`, `off` or `unavailable: …`. */
  ffmpeg?: string
  /** Blender, for model conversion: its version, `downloading`, `looking`, `off` or `unavailable: …`. */
  blender?: string
  /** A newer Hoard release, when the server's daily check found one. */
  update?: { version: string; url: string }
  /** Model thumbnails: `gpu: <adapter> (<api>)`, `cpu: …` (software), `starting`, `off` or `unavailable: …`. */
  renderer?: string
  /** When thumbnails render in software although a GPU seems to be there: what the container is missing. */
  renderer_hint?: string
  /** No library location yet: the first-run setup should open. */
  setup_needed?: boolean
  /** Space cached "Download as" conversions take, and the most they may. */
  conversion_cache_bytes?: number
  conversion_cache_limit?: number
  /** Space thumbnails, model views and audio previews take. */
  preview_bytes?: number
  /** The editors this build fetches when a conversion needs one: `blender`, `unreal`. */
  editor_downloads?: string[]
}

/** An app (Claude, another MCP client) someone let sign in over OAuth. */
/** A search someone kept: the portal's query string for it (`q=rock&kind=texture`). */
export interface SavedSearch {
  id: number
  name: string
  query: string
  created_at: string
}

/** A folder on the server and the folders in it (`GET /api/admin/fs`). */
export interface FolderListing {
  path: string
  parent?: string | null
  dirs: { name: string; path: string }[]
}

/** The library in numbers (`GET /api/stats/library`); archived things aren't counted. */
export interface LibraryStats {
  kinds: { value: string; count: number; bytes: number }[]
  extensions: { value: string; count: number; bytes: number }[]
  /** Newest first: `YYYY-MM`. */
  months: { month: string; count: number; bytes: number }[]
  /** Tag rows by who added them; `tags` is how many different tags. */
  tag_sources: { value: TagSource; count: number; tags: number }[]
  top_tags: Facet[]
  packs: { id: string; name: string; count: number; bytes: number }[]
  largest: { id: string; name: string; kind: AssetKind; pack_name: string; bytes: number }[]
  distinct_tags: number
  untagged: number
  described: number
  human_edited: number
  /** When the report was made: it's kept, and made again in the background once the library changes. */
  made_at?: string
  /** A newer report is being made; asking again shortly gets it. */
  refreshing?: boolean
}

export interface SearchParams {
  q?: string
  kind?: string
  tags?: string[]
  pack?: string
  /** With `pack`: only what's directly in this folder of it (`''` for its top), or with `q`, anything under it. */
  folder?: string
  sort?: string
  limit?: number
  offset?: number
  /** false leaves out the kind and tag counts, for a later page. */
  facets?: boolean
  hide_supporting?: boolean
  collection?: string
  /** Only what this project (name) has. */
  project?: string
  /** Search the archive instead of the library. */
  archived?: boolean
  /** "More like this": an asset id. */
  similar?: string
  /** `#rrggbb`: assets with much of this color. */
  color?: string
  /** A word imitating a sound (`whooom`): sounds shaped like it, most alike first. */
  imitation?: string
  /** Music at this tempo (BPM) or faster, or slower. */
  bpm_min?: number
  bpm_max?: number
  /** Music in this key (`a-minor`), and with `compatible_keys` the keys that mix with it. */
  key?: string
  compatible_keys?: boolean
  /** Only the favorites of whoever's signed in. */
  favorites?: boolean
  /** Only what's been reviewed (true), or only what hasn't (false). */
  reviewed?: boolean
}

/** Packs and assets to archive, restore or delete for good. */
export interface ArchiveItems { packs?: string[]; assets?: string[] }
export interface ArchiveCount { packs: number; assets: number }

/** A change to an asset: each field left out stays as it is, and an empty text clears what it's for. */
export interface AssetPatch {
  name?: string
  category?: string
  description?: string
  license?: string
  source_url?: string
  /** 1 to 5; 0 clears it. */
  rating?: number
  reviewed?: boolean
  /** By name; an empty value removes one. */
  fields?: Record<string, string>
  add_tags?: string[]
  /** A rule's or the AI's tag taken off stays off, until `unreject_tags` puts it back. */
  remove_tags?: string[]
  /** A rule's or the AI's tags to keep as a person's. */
  accept_tags?: string[]
  /** Keep all of its rule and AI tags as a person's. */
  accept_all_tags?: boolean
  unreject_tags?: string[]
  /** The family's other variants too (all but the name, rating, fields and whether it's reviewed). */
  apply_to_family?: boolean
}

/** One edit to many assets at once: those listed, or everything a search finds. */
export interface BulkEdit extends Omit<AssetPatch, 'name' | 'apply_to_family'> {
  ids?: string[]
  query?: SearchParams
}

/** One change in an asset's history. `field` is `name`, `category`, `description`, `license`, `source_url`,
 *  `rating`, `reviewed`, `tag:<tag>` (old and new are its source then: `ai:0.83` with its confidence, or null where
 *  it didn't have it) or `field:<name>`. `source` is how: `human`, `app`, `mcp`, `rule` or `ai`. */
export interface Change {
  id: number
  field: string
  old: string | null
  new: string | null
  who: string | null
  source: string
  at: string
}

/** A sprite sheet as Hoard keeps it (see crates/hoardd/src/spritesheet/). */
export interface SheetData {
  hoard_sheet: number
  /** The format it was read from: `texturepacker`, `libgdx`, `godot-tileset`, … */
  from: string
  pages: { image: string; size: [number, number]; asset?: string }[]
  sprites: SheetSprite[]
  animations?: { name: string; frames: { sprite: number; ms: number }[]; loop: boolean }[]
  tile?: [number, number]
}

export interface SheetSprite {
  name: string
  page: number
  /** Where its pixels are on the page, as stored there (turned, if they are). */
  rect: [number, number, number, number]
  /** How far its stored pixels are turned clockwise from upright. */
  turn?: number
  /** Its size, upright, before its transparent margin was trimmed off. */
  size: [number, number]
  /** Where its pixels, upright, sit in that size. */
  at?: [number, number]
}

/** `palette` is the new thumbnail's, when it replaced the old one. */
export interface ThumbUpload { replaced: boolean; thumb_source: ThumbSource | null; thumb_rev?: string; palette?: Swatch[] }

/** Where a "download as" conversion stands. */
export interface ConversionStatus {
  state: 'ready' | 'queued' | 'converting' | 'failed'
  /** 0..1, while converting, when the conversion can tell. */
  progress?: number
  error?: string
}

/** What a project owes each pack, by any mix of packs, collections, assets and searches. */
export interface CreditsRequest {
  packs?: string[]
  collections?: string[]
  ids?: string[]
  selections?: { ids?: string[]; query?: Record<string, unknown>; include_deps?: boolean }[]
}
