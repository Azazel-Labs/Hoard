// Typed client for the hoardd API. Types mirror crates/hoard-core.

export type AssetKind =
  | 'texture' | 'audio' | 'model' | 'animation' | 'font' | 'shader'
  | 'material' | 'video' | 'document' | 'data' | 'other'

export type TagSource = 'meta' | 'rule' | 'ai' | 'human'

export interface Tag { tag: string; source: TagSource; confidence: number }

export type LinkRel = 'texture' | 'buffer' | 'material' | 'animation' | 'library' | 'guess' | 'sidecar' | 'variant'

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
  meta: Record<string, unknown>
  tags: Tag[]
  created_at: string
  updated_at: string
  /** When it went to the archive (its own, or its pack's). */
  archived_at?: string | null
  /** Detail view only. */
  links: AssetLink[]
  linked_from: AssetLink[]
  derivatives: string[]
  /** Numbered takes of the same thing (whoosh_01..12); `members` lists the others. */
  family?: { key: string; size: number; members: AssetLink[] } | null
  /** Byte-identical files elsewhere in the library. */
  duplicates?: Duplicate[]
  /** Collections this asset is in. */
  collections?: CollectionRef[]
  /** What this file was converted from (an Unreal package). */
  origin?: AssetOrigin | null
  /** For an Unreal package not converted yet: why. */
  unreal?: UnrealStatus | null
  /** Other formats the server can convert this asset to on download. */
  formats?: FormatOption[]
  /** Dominant colours of the thumbnail, most of the picture first. */
  palette?: Swatch[]
  /** For a material made from texture maps: its maps. */
  material?: MaterialInfo | null
  /** Materials made from texture maps that this texture is one of, or this model uses. */
  materials?: MaterialInfo[]
}

/** A material made from texture maps that share a name (Rock_BaseColor.png, Rock_Normal.png, …). */
export interface MaterialInfo {
  id: string
  name: string
  path: string
  has_thumb: boolean
  /** By role (colour first), each role's largest first. */
  maps: MaterialMap[]
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
  /** `native` (Hoard's reader) or `editor` (Unreal Editor export). */
  method: string
  notes: string | null
  converted_at: string
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

/** A hand-picked set of assets across packs ("Forest level props"). */
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
}

export interface Facet { value: string; count: number }

export interface SearchResult {
  items: Asset[]
  total: number
  kinds: Facet[]
  tags: Facet[]
  /** Matches left out because `hide_supporting` was on (glTF buffers, .mtl files). */
  supporting_hidden: number
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
  /** No library location yet: the first-run setup should open. */
  setup_needed?: boolean
  /** Space cached "Download as" conversions take, and the most they may. */
  conversion_cache_bytes?: number
  conversion_cache_limit?: number
  /** Space thumbnails, model views and audio previews take. */
  preview_bytes?: number
}

/** Where a library keeps files: managed (Hoard copies files in) or watched (indexed where they sit). */
export type LocationMode = 'managed' | 'watched'

export interface LibraryLocation {
  id: number
  name: string
  mode: LocationMode
  /** A folder on the server, or `s3://bucket`. */
  path: string
  /** The asset kinds new files of which go here (managed locations). */
  kinds: AssetKind[]
  /** Takes every kind no other location ticks. */
  catch_all: boolean
  scanned_at?: string | null
  created_at: string
  /** Whether the folder can be reached right now. */
  online: boolean
  packs: number
}

export interface LocationList {
  locations: LibraryLocation[]
  /** Where the setup suggests the first location goes. */
  suggested: string
  kinds: AssetKind[]
}

export interface NewLocation {
  name?: string
  mode: LocationMode
  path: string
  kinds: AssetKind[]
  catch_all: boolean
}

/** `GET /api/auth/status`: who this browser is signed in as, and what the server wants first. */
export interface AuthStatus {
  /** No account yet: the portal asks for the first one. */
  setup_needed: boolean
  signed_in: boolean
  user: { name: string; kind: 'user' | 'token' } | null
  /** People who aren't signed in may look around. */
  guest_browsing: boolean
}

export interface Account {
  id: number
  username: string
  created_at: string
  you: boolean
}

/** An app (Claude, another MCP client) someone let sign in over OAuth. */
export interface Connection {
  id: number
  app: string
  user: string
  created_at: string
  last_used_at: string | null
}

export interface AccessToken {
  id: number
  name: string
  created_at: string
  last_used_at: string | null
  created_by: string | null
}

/** A finished (or interrupted) run of a maintenance task. */
export interface MaintenanceRun {
  id: number
  task: string
  started_at: string
  finished_at: string | null
  ok: boolean | null
  summary: string | null
}

export interface MaintenanceTask {
  id: string
  label: string
  blurb: string
  running: { run: number; started_at: string; progress?: number; detail?: string } | null
  last: MaintenanceRun | null
}

/** Settings → Maintenance (`GET /api/admin/maintenance`). */
export interface Maintenance {
  tasks: MaintenanceTask[]
  history: MaintenanceRun[]
}

/** Something that went wrong opening or processing the library, in Settings → Issues. */
export interface Issue {
  /** Opaque: pass it back to retry or dismiss. */
  id: string
  title: string
  message: string
  pack_id: string | null
  pack_name: string | null
  asset_id: string | null
  path: string | null
  location_id: number | null
  location_name: string | null
  /** How many times it's happened. */
  count: number
  at: string | null
  retry: boolean
}

/** One kind of issue. `retry` is the retry button's label, when it has one; at most 200 `items` of `total`. */
export interface IssueGroup {
  kind: string
  title: string
  blurb: string
  retry: string | null
  dismiss: boolean
  total: number
  items: Issue[]
}

/** `GET /api/admin/issues`: only groups that have issues, most serious first. */
export interface Issues {
  total: number
  groups: IssueGroup[]
}

/** Which issues an action is for: some, or a whole group. */
export type IssuePick = { ids: string[] } | { kind: string }

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
}

/** An outside program Hoard can use, as the Settings page shows it. */
export interface ToolSetting {
  /** `off`, `auto` or a path. */
  value: string
  /** The built-in default, used when nothing is saved. */
  default: string
  saved: boolean
  state: 'off' | 'looking' | 'downloading' | 'ready' | 'unavailable'
  /** The version when ready, why when unavailable. */
  detail: string
  can_download: boolean
}

export type UnrealConvert = 'off' | 'keep' | 'replace'

/** A server option: what's in effect, what the flags give, and whether the portal changed it. */
export interface OptionSetting<T> {
  value: T
  default: T
  saved: boolean
}

/** A model an AI service offers (`POST /api/admin/llm/models`). */
export interface ModelInfo {
  id: string
  name?: string
  /** Whether it takes images, when the service says. */
  vision?: boolean
}

/** A certificate, as Settings → HTTPS shows it. Times are Unix seconds. */
export interface CertInfo {
  names: string[]
  subject: string
  issuer: string
  not_before: number
  not_after: number
  self_signed: boolean
  fingerprint: string
}

export type HttpsMode = 'off' | 'own' | 'custom'

/** Settings → HTTPS (`GET /api/admin/https`). */
export interface HttpsStatus {
  available: boolean
  mode: HttpsMode
  /** Addresses added for Hoard's own certificate. */
  names: string[]
  http3: boolean
  redirect: boolean
  /** HTTPS is being served now. */
  on: boolean
  /** Why it isn't, though it's chosen. */
  error: string | null
  http3_running: boolean
  http3_error: string | null
  /** Addresses Hoard found by itself. */
  found: string[]
  covers_this_address: boolean | null
  /** The certificate being served. */
  cert: CertInfo | null
  /** The uploaded certificate, if there is one. */
  custom: CertInfo | null
  /** Hoard's own certificate authority, once made. */
  ca: CertInfo | null
  port: number
  this_address: string | null
}

export interface HttpsChange {
  mode?: HttpsMode
  names?: string[]
  http3?: boolean
  redirect?: boolean
  cert?: string
  key?: string
}

export interface CertCheck {
  cert: CertInfo
  problem: string | null
  covers_this_address: boolean | null
  this_address: string | null
}

/** The ChatGPT account the server tags through, if it's signed in (`GET /api/admin/chatgpt`). */
export interface ChatGptStatus {
  signed_in: boolean
  email?: string | null
  /** Why tagging can't use the account right now. */
  problem?: string | null
  /** A sign-in was started and hasn't finished. */
  waiting: boolean
  /** Where to see or limit what Hoard uses of the plan. */
  manage_url: string
  /** After signing out: anything the person should still do. */
  note?: string | null
}

/** A model the local model server has, or is downloading (`POST /api/admin/llm/local`). */
export interface LocalModel {
  /** Its Hugging Face reference, which is also the name to use it by. */
  id: string
  /** `downloading`, `incomplete`, `unloaded` (ready), `loading`, `loaded`, `sleeping`. */
  status: string
  /** Bytes done and expected, while downloading. */
  progress?: [number, number]
  vision: boolean
  failed: boolean
}

/** A model Hoard suggests for the local model server. */
export interface SuggestedModel {
  id: string
  name: string
  size_gb: number
  license: string
  note: string
}

export interface LocalModels {
  reachable: boolean
  error?: string
  models: LocalModel[]
  suggested: SuggestedModel[]
}

export interface Settings {
  blender: ToolSetting
  ffmpeg: ToolSetting
  unreal_convert: { value: UnrealConvert; default: UnrealConvert; saved: boolean }
  llm: {
    url: string
    model: string
    has_key: boolean
    vision: boolean
    /** Tagging goes through the ChatGPT account signed in on the server. */
    chatgpt: boolean
    batch: OptionSetting<number>
    enabled: boolean
    saved: boolean
    default: { url: string; model: string; has_key: boolean; vision: boolean }
    /** Where the compose files' local model server listens. */
    local_url: string
  }
  options: {
    workers: OptionSetting<number>
    max_upload_mb: OptionSetting<number>
    max_expand_mb: OptionSetting<number>
    render: OptionSetting<boolean>
    update_check: OptionSetting<boolean>
    guest_browsing: OptionSetting<boolean>
    /** Search the stores for packs' pages in the background. */
    store_lookup: OptionSetting<boolean>
    thumb_quality: OptionSetting<ThumbQuality>
    /** Most space cached "Download as" conversions may take, in MiB. */
    conversion_cache_mb: OptionSetting<number>
    /** Empty: from the request's Host header. */
    public_url: OptionSetting<string>
    /** Empty: look in the usual places. */
    soundfont: OptionSetting<string>
  }
  max_workers: number
  clip: ImageSearchSetting
}

/** Image search (CLIP): its switch, its model, and how many pictures it has learned. */
export interface ImageSearchSetting {
  value: boolean
  default: boolean
  saved: boolean
  state: 'off' | 'looking' | 'downloading' | 'ready' | 'unavailable'
  /** Why, when unavailable. */
  detail: string
  download_bytes: number
  /** Pictures with an embedding, of all that could have one. */
  done: number
  total: number
}

/** How thumbnails are stored: JPEG at three qualities, or PNG. Transparent ones and pixel art are always PNG. */
export type ThumbQuality = 'small' | 'balanced' | 'sharp' | 'lossless'

/** Settings to change; `null` goes back to the server's default. */
export interface SettingsPatch {
  blender?: string | null
  ffmpeg?: string | null
  unreal_convert?: UnrealConvert | null
  llm_url?: string | null
  llm_model?: string | null
  llm_api_key?: string | null
  llm_vision?: boolean | null
  llm_chatgpt?: boolean | null
  llm_batch?: number | null
  workers?: number | null
  max_upload_mb?: number | null
  max_expand_mb?: number | null
  render?: boolean | null
  update_check?: boolean | null
  guest_browsing?: boolean | null
  store_lookup?: boolean | null
  thumb_quality?: ThumbQuality | null
  conversion_cache_mb?: number | null
  public_url?: string | null
  soundfont?: string | null
  clip?: boolean | null
}

export interface SearchParams {
  q?: string
  kind?: string
  tags?: string[]
  pack?: string
  sort?: string
  limit?: number
  offset?: number
  hide_supporting?: boolean
  collection?: string
  /** Search the archive instead of the library. */
  archived?: boolean
  /** "More like this": an asset id. */
  similar?: string
  /** `#rrggbb`: assets with much of this colour. */
  color?: string
}

/** Packs and assets to archive, restore or delete for good. */
export interface ArchiveItems { packs?: string[]; assets?: string[] }
export interface ArchiveCount { packs: number; assets: number }

/** One edit to many assets at once. */
export interface BulkEdit { ids: string[]; add_tags?: string[]; remove_tags?: string[]; category?: string }

/** The server omits empty link/derivative lists; give the UI arrays to work with. */
function normalize(a: Asset): Asset {
  return { ...a, links: a.links ?? [], linked_from: a.linked_from ?? [], derivatives: a.derivatives ?? [] }
}

/**
 * Sent with every request. The session cookie does the signing in; this header tells the server the request came
 * from the portal and not from another site's page riding on the cookie.
 */
export function authHeaders(): Record<string, string> {
  return { 'X-Hoard': '1' }
}

/** Called when the server says to sign in (set by the auth store). */
let onSignInNeeded: () => void = () => {}
export function whenSignInNeeded(f: () => void) {
  onSignInNeeded = f
}
export function signInNeeded() {
  onSignInNeeded()
}

export class ApiError extends Error {
  status: number
  constructor(status: number, message: string) {
    super(message)
    this.status = status
  }
}

async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const res = await fetch(path, { ...init, headers: { ...authHeaders(), ...(init.headers ?? {}) } })
  if (!res.ok) {
    let msg = `${res.status} ${res.statusText}`
    try {
      const body = await res.json()
      if (body?.error) msg = body.error
    } catch {
      // Non-JSON error body; keep the status line.
    }
    if (res.status === 401 && !path.startsWith('/api/auth/')) signInNeeded()
    throw new ApiError(res.status, msg)
  }
  if (res.status === 204 || res.status === 202) return undefined as T
  return res.json() as Promise<T>
}

function post<T>(path: string, body: unknown): Promise<T> {
  return request<T>(path, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) })
}

export interface ThumbUpload { replaced: boolean; thumb_source: ThumbSource | null; thumb_rev?: string }

export const api = {
  /** A model's thumbnail as this browser's 3D viewer drew it; kept unless the one there came from a better source. */
  putThumb: (id: string, png: Blob, source: ThumbSource) =>
    request<ThumbUpload>(`/api/assets/${id}/thumb?source=${source}`, {
      method: 'PUT',
      headers: { 'Content-Type': png.type || 'image/png' },
      body: png,
    }),
  stats: () => request<Stats>('/api/stats'),
  libraryStats: () => request<LibraryStats>('/api/stats/library'),
  packs: (archived = false) => request<Pack[]>(`/api/packs${archived ? '?archived=true' : ''}`),
  archive: (items: ArchiveItems) => post<ArchiveCount>('/api/archive', items),
  restore: (items: ArchiveItems) => post<ArchiveCount>('/api/archive/restore', items),
  /** Delete for good: these, or with `all` everything in the archive. Only archived things are deleted. */
  purge: (items: ArchiveItems & { all?: boolean }) => post<ArchiveCount>('/api/archive/purge', items),
  bulkAssets: (edit: BulkEdit) => post<{ updated: number }>('/api/assets/bulk', edit),
  deletePack: (id: string) => request<void>(`/api/packs/${id}`, { method: 'DELETE', keepalive: true }),
  renamePack: (id: string, name: string) =>
    request<Pack>(`/api/packs/${id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name }),
    }),
  retagPack: (id: string, body: { rules?: boolean; llm?: boolean }) =>
    request<void>(`/api/packs/${id}/retag`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    }),
  readPackFiles: (id: string) => request<void>(`/api/packs/${id}/read-files`, { method: 'POST' }),
  search(p: SearchParams): Promise<SearchResult> {
    const qs = new URLSearchParams()
    if (p.q) qs.set('q', p.q)
    if (p.kind) qs.set('kind', p.kind)
    if (p.tags?.length) qs.set('tags', p.tags.join(','))
    if (p.pack) qs.set('pack', p.pack)
    if (p.sort) qs.set('sort', p.sort)
    qs.set('limit', String(p.limit ?? 60))
    if (p.offset) qs.set('offset', String(p.offset))
    if (p.hide_supporting) qs.set('hide_supporting', 'true')
    if (p.collection) qs.set('collection', p.collection)
    if (p.archived) qs.set('archived', 'true')
    if (p.similar) qs.set('similar', p.similar)
    if (p.color) qs.set('color', p.color)
    return request<SearchResult>(`/api/assets?${qs}`).then((r) => ({ ...r, items: r.items.map(normalize) }))
  },
  asset: (id: string) => request<Asset>(`/api/assets/${id}`).then(normalize),
  patchAsset: (id: string, patch: {
    name?: string; category?: string; description?: string; add_tags?: string[]; remove_tags?: string[]
    apply_to_family?: boolean
  }) =>
    request<Asset>(`/api/assets/${id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(patch),
    }).then(normalize),
  packContext: (id: string) => request<PackContext>(`/api/packs/${id}/context`),
  /** Likely store pages for a pack, searched by `q` or else by its name (what the last lookup found, unless `fresh`). */
  storeMatches: (id: string, q?: string, fresh = false) =>
    request<StoreMatches>(
      `/api/packs/${id}/store-matches${q ? `?q=${encodeURIComponent(q)}` : fresh ? '?fresh=true' : ''}`,
    ),
  /** Take off a store page that was found automatically. */
  undoStorePage: (id: string) => request<PackContext>(`/api/packs/${id}/store/undo`, { method: 'POST' }),
  setPackContext: (id: string, ctx: Omit<PackContext, 'readme' | 'store_auto'>) =>
    request<PackContext>(`/api/packs/${id}/context`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(ctx),
    }),
  collections: () => request<Collection[]>('/api/collections'),
  createCollection: (name: string, description?: string) =>
    request<Collection>('/api/collections', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, description }),
    }),
  updateCollection: (id: string, patch: { name?: string; description?: string }) =>
    request<Collection>(`/api/collections/${id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(patch),
    }),
  deleteCollection: (id: string) => request<void>(`/api/collections/${id}`, { method: 'DELETE', keepalive: true }),
  collectionMembers: (id: string, change: { add?: string[]; remove?: string[] }) =>
    request<Collection>(`/api/collections/${id}/assets`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(change),
    }),
  unrealStatus: (packId: string) => request<UnrealPackStatus>(`/api/packs/${packId}/unreal`),
  unrealConvert: (packId: string, opts: { discard_originals: boolean; drop_unsupported: boolean }) =>
    request<void>(`/api/packs/${packId}/unreal/convert`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(opts),
    }),
  tags: (prefix: string) => request<Facet[]>(`/api/tags?prefix=${encodeURIComponent(prefix)}&limit=12`),
  /** Every tag in use containing `q`, most used first. */
  allTags: (q: string, limit = 2000) =>
    request<Facet[]>(`/api/tags?prefix=${encodeURIComponent(q)}&anywhere=true&limit=${limit}`),
  /** Fold tags into one; a single tag into a new name renames it. */
  mergeTags: (from: string[], into: string) => post<{ into: string; assets: number }>('/api/tags/merge', { from, into }),
  tagAliases: () => request<{ tag: string; into: string }[]>('/api/tags/aliases'),
  dropTagAlias: (tag: string) => request<void>(`/api/tags/aliases?tag=${encodeURIComponent(tag)}`, { method: 'DELETE' }),
  activity: () => request<{ items: ActivityItem[] }>('/api/activity'),
  settings: () => request<Settings>('/api/admin/settings'),
  saveSettings: (patch: SettingsPatch) =>
    request<Settings>('/api/admin/settings', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(patch),
    }),
  /** The models an AI service offers; without `api_key`, the saved key is used if `url` is the one in use. */
  llmModels: (url: string, api_key?: string) =>
    request<{ models: ModelInfo[] }>('/api/admin/llm/models', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url, api_key }),
    }),
  /** The local model server's models; `download` and `delete` act on `model` first. */
  llmLocal: (url: string, action: 'list' | 'download' | 'delete' = 'list', model?: string) =>
    request<LocalModels>('/api/admin/llm/local', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url, action, model }),
    }),
  chatgpt: () => request<ChatGptStatus>('/api/admin/chatgpt'),
  /** The address to open to sign in; `port` is the one the browser reaches Hoard on. */
  chatgptStart: (port: number) =>
    request<{ url: string }>('/api/admin/chatgpt/start', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ port }),
    }),
  /** Finish signing in from the address the browser ended up on. */
  chatgptFinish: (url: string) =>
    request<ChatGptStatus>('/api/admin/chatgpt/finish', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url }),
    }),
  chatgptSignOut: () => request<ChatGptStatus>('/api/admin/chatgpt/sign-out', { method: 'POST' }),
  chatgptModels: () => request<{ models: ModelInfo[] }>('/api/admin/chatgpt/models'),
  https: () => request<HttpsStatus>('/api/admin/https'),
  /** Tried before it's switched to: a setup that doesn't work is refused, and what's running stays. */
  saveHttps: (change: HttpsChange) =>
    request<HttpsStatus>('/api/admin/https', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(change),
    }),
  checkCert: (cert: string, key: string) => post<CertCheck>('/api/admin/https/check', { cert, key }),
  authStatus: () => request<AuthStatus>('/api/auth/status'),
  setup: (username: string, password: string) =>
    post<{ user: { name: string } }>('/api/auth/setup', { username, password }),
  login: (username: string, password: string) =>
    post<{ user: { name: string } }>('/api/auth/login', { username, password }),
  logout: () => request<void>('/api/auth/logout', { method: 'POST' }),
  changePassword: (current: string, next: string) =>
    post<void>('/api/auth/password', { current, new: next }),
  accounts: () => request<{ users: Account[] }>('/api/admin/users'),
  addAccount: (username: string, password: string) =>
    post<void>('/api/admin/users', { username, password }),
  removeAccount: (id: number) => request<void>(`/api/admin/users/${id}`, { method: 'DELETE' }),
  tokens: () => request<{ tokens: AccessToken[] }>('/api/admin/tokens'),
  makeToken: (name: string) =>
    post<{ id: number; name: string; token: string }>('/api/admin/tokens', { name }),
  revokeToken: (id: number) => request<void>(`/api/admin/tokens/${id}`, { method: 'DELETE' }),
  connections: () => request<{ connections: Connection[] }>('/api/admin/connections'),
  disconnect: (id: number) => request<void>(`/api/admin/connections/${id}`, { method: 'DELETE' }),
  oauthRequest: (search: string) => request<{ client: string; redirect_host: string }>(`/api/oauth/request${search}`),
  oauthApprove: (params: Record<string, string>, allow: boolean) =>
    post<{ redirect: string }>('/api/oauth/approve', { ...params, allow }),
  maintenance: () => request<Maintenance>('/api/admin/maintenance'),
  issues: () => request<Issues>('/api/admin/issues'),
  retryIssues: (pick: IssuePick) => post<{ started: number }>('/api/admin/issues/retry', pick),
  dismissIssues: (pick: IssuePick) => post<{ dismissed: number }>('/api/admin/issues/dismiss', pick),
  runMaintenance: (task: string) =>
    request<{ run: number }>(`/api/admin/maintenance/${task}`, { method: 'POST' }),
  locations: () => request<LocationList>('/api/admin/locations'),
  addLocation: (loc: NewLocation) =>
    request<LibraryLocation>('/api/admin/locations', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(loc),
    }),
  patchLocation: (id: number, patch: { name?: string; kinds?: AssetKind[]; catch_all?: boolean }) =>
    request<LibraryLocation>(`/api/admin/locations/${id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(patch),
    }),
  /** `force` removes a managed location that can't be reached; files still on it go missing. */
  deleteLocation: (id: number, force = false) =>
    request<void>(`/api/admin/locations/${id}${force ? '?force=true' : ''}`, { method: 'DELETE' }),
  scanLocation: (id: number) => request<void>(`/api/admin/locations/${id}/scan`, { method: 'POST' }),
  emptyLocation: (id: number) => request<void>(`/api/admin/locations/${id}/empty`, { method: 'POST' }),
  rebalance: () => request<void>('/api/admin/locations/rebalance', { method: 'POST' }),
  /** The folders in `path` on the server; an empty path lists the drives or `/`. */
  browse: (path = '') => request<FolderListing>(`/api/admin/fs?path=${encodeURIComponent(path)}`),
  /** Start or look in on a "download as" conversion; answers when it moves on, or after `wait` seconds. */
  asStatus: (a: { id: string }, format: string, max?: number, wait = 5) =>
    request<ConversionStatus>(`/api/assets/${a.id}/as/${format}/status?wait=${wait}${max ? `&max=${max}` : ''}`),
}

/** Background work the server is doing (see `GET /api/activity`). */
export interface ActivityItem {
  id: number
  kind: string
  label: string
  detail?: string
  /** 0..1, when the work can tell. */
  progress?: number
  state: 'running' | 'queued'
  since: string
  pack_id?: string
}

/** Where a "download as" conversion stands. */
export interface ConversionStatus {
  state: 'ready' | 'queued' | 'converting' | 'failed'
  /** 0..1, while converting, when the conversion can tell. */
  progress?: number
  error?: string
}

export const fileUrl = (a: Asset, download = false) => `/api/assets/${a.id}/file${download ? '?download=1' : ''}`
/** The asset converted to another format (made on first request, then cached), optionally scaled down. */
export const asUrl = (a: { id: string }, format: string, max?: number) =>
  `/api/assets/${a.id}/as/${format}${max ? `?max=${max}` : ''}`
/** Image formats the server re-encodes as themselves when scaling down, keyed by extension. */
export const RESIZABLE_AS_ITSELF: Record<string, string> = {
  png: 'png', jpg: 'jpg', jpeg: 'jpg', webp: 'webp', tga: 'tga', tif: 'tiff', tiff: 'tiff', exr: 'exr',
}
/** The original file of a converted asset (the .uasset), when it was kept. */
export const originalUrl = (a: { id: string }) => `/api/assets/${a.id}/original`
export const thumbUrl = (a: { id: string; thumb_rev?: string | null }) =>
  `/api/assets/${a.id}/thumb${a.thumb_rev ? `?v=${a.thumb_rev}` : ''}`
/** An image at most 2048 px on its longer side (the image itself when it's that small and browsers read it). */
export const liteUrl = (a: { id: string }) => `/api/assets/${a.id}/lite`
export const bundleUrl = (a: Asset, animations = false) => `/api/assets/${a.id}/bundle.zip${animations ? '?animations=1' : ''}`
export const packZipUrl = (packId: string) => `/api/packs/${packId}/bundle.zip`
export const collectionZipUrl = (id: string, sources = false) =>
  `/api/collections/${id}/bundle.zip${sources ? '?sources=1' : ''}`
/** Several assets in one zip, each with what it needs (one folder per pack when they come from several). */
export const selectionZipUrl = (ids: string[], deps = true, sources = false) =>
  `/api/bundle.zip?ids=${ids.join(',')}${deps ? '' : '&deps=0'}${sources ? '&sources=1' : ''}`
/** CREDITS.md for a pack, or for the packs a collection's assets come from. */
export const packCreditsUrl = (packId: string) => `/api/packs/${packId}/credits?format=md`
export const collectionCreditsUrl = (id: string) => `/api/collections/${id}/credits?format=md`

/** What a project owes each pack, by any mix of packs, collections, assets and searches. */
export interface CreditsRequest {
  packs?: string[]
  collections?: string[]
  ids?: string[]
  selections?: { ids?: string[]; query?: Record<string, unknown>; include_deps?: boolean }[]
}

/** Download CREDITS.md for a request (a POST, so it's fetched and then saved). */
export async function downloadCredits(body: CreditsRequest): Promise<void> {
  const res = await fetch('/api/credits?format=md', {
    method: 'POST',
    headers: { ...authHeaders(), 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) {
    let msg = `${res.status} ${res.statusText}`
    try {
      msg = (await res.json())?.error ?? msg
    } catch {
      // Non-JSON error body; keep the status line.
    }
    throw new ApiError(res.status, msg)
  }
  const url = URL.createObjectURL(await res.blob())
  const a = document.createElement('a')
  a.href = url
  a.download = 'CREDITS.md'
  a.click()
  setTimeout(() => URL.revokeObjectURL(url), 10_000)
}

/** A pack file by its original path; relative references inside model files resolve against this URL. */
export const packFileUrl = (packId: string, path: string) =>
  `/api/packs/${packId}/files/${path.split('/').map(encodeURIComponent).join('/')}`

/** Asset kinds the browser viewer can open (plus anything the server converted to .glb). */
export const VIEWABLE_EXTS = new Set(['glb', 'gltf', 'obj', 'fbx', 'stl', 'ply', 'dae'])

/** One file to upload with its path inside the pack. */
export interface UploadFile { file: File; path: string }

/** Collect files from a drop, one group per top-level item, so dropping three zips makes three packs. */
export async function filesFromDrop(dt: DataTransfer): Promise<{ name: string; files: UploadFile[] }[]> {
  // Entries must be grabbed synchronously, before any await, or the browser empties the DataTransfer.
  const entries = [...dt.items]
    .filter((i) => i.kind === 'file')
    .map((i) => i.webkitGetAsEntry?.())
    .filter((e): e is FileSystemEntry => !!e)
  if (entries.length === 0) {
    return [...dt.files].map((f) => ({ name: f.name, files: [{ file: f, path: f.name }] }))
  }
  const groups: { name: string; files: UploadFile[] }[] = []
  for (const entry of entries) {
    const files: UploadFile[] = []
    await walk(entry, '', files)
    if (files.length) groups.push({ name: entry.name, files })
  }
  return groups
}

async function walk(entry: FileSystemEntry, prefix: string, out: UploadFile[]): Promise<void> {
  if (entry.isFile) {
    const file = await new Promise<File>((res, rej) => (entry as FileSystemFileEntry).file(res, rej))
    out.push({ file, path: prefix + entry.name })
  } else if (entry.isDirectory) {
    const reader = (entry as FileSystemDirectoryEntry).createReader()
    for (;;) {
      const batch = await new Promise<FileSystemEntry[]>((res, rej) => reader.readEntries(res, rej))
      if (batch.length === 0) break
      for (const child of batch) await walk(child, `${prefix}${entry.name}/`, out)
    }
  }
}

export function humanBytes(b: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let v = b
  let i = 0
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024
    i++
  }
  return `${v < 10 && i > 0 ? v.toFixed(1) : Math.round(v)} ${units[i]}`
}

export function formatDuration(secs: number): string {
  if (secs < 10) return `${secs.toFixed(2)} s`
  const m = Math.floor(secs / 60)
  const s = Math.round(secs % 60)
  return m ? `${m}:${String(s).padStart(2, '0')}` : `${s} s`
}

export const KIND_LABEL: Record<AssetKind, string> = {
  texture: 'Textures',
  audio: 'Audio',
  model: 'Models',
  animation: 'Animations',
  font: 'Fonts',
  shader: 'Shaders',
  material: 'Materials',
  video: 'Video',
  document: 'Documents',
  data: 'Data',
  other: 'Other',
}

/** Kinds whose thumbnails are pictures of the thing itself, so "looks like" means something (mirrors the server). */
const LOOK_KINDS = new Set<AssetKind>(['texture', 'model', 'material', 'animation', 'video', 'other'])
/** Whether "More like this" can find anything for an asset. */
export const looksAlike = (a: Asset) => a.has_thumb && LOOK_KINDS.has(a.kind)
