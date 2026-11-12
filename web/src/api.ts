// Typed client for the hoardd API. Types mirror crates/hoard-core.

export type AssetKind =
  | 'texture' | 'audio' | 'model' | 'animation' | 'font' | 'shader'
  | 'material' | 'video' | 'document' | 'data' | 'other'

export type TagSource = 'meta' | 'rule' | 'ai' | 'human'

export interface Tag { tag: string; source: TagSource; confidence: number }

export type LinkRel = 'texture' | 'buffer' | 'material' | 'animation' | 'library' | 'guess' | 'variant'

export interface AssetLink {
  id: string
  name: string
  path: string
  kind: AssetKind
  rel: LinkRel
  has_thumb: boolean
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
}

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
export interface PackContext {
  store: string | null
  store_url: string | null
  store_text: string | null
  license_tier: string | null
  /** README text captured at import (read-only). */
  readme: string | null
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
  auth_required: boolean
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
    /** Empty: from the request's Host header. */
    public_url: OptionSetting<string>
    /** Empty: look in the usual places. */
    soundfont: OptionSetting<string>
  }
  max_workers: number
}

/** Settings to change; `null` goes back to the server's default. */
export interface SettingsPatch {
  blender?: string | null
  ffmpeg?: string | null
  unreal_convert?: UnrealConvert | null
  llm_url?: string | null
  llm_model?: string | null
  llm_api_key?: string | null
  llm_vision?: boolean | null
  llm_batch?: number | null
  workers?: number | null
  max_upload_mb?: number | null
  max_expand_mb?: number | null
  render?: boolean | null
  update_check?: boolean | null
  public_url?: string | null
  soundfont?: string | null
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

const TOKEN_KEY = 'hoard.token'

export function getToken(): string {
  try {
    return localStorage.getItem(TOKEN_KEY) ?? ''
  } catch {
    return ''
  }
}

export function setToken(token: string): void {
  try {
    if (token) localStorage.setItem(TOKEN_KEY, token)
    else localStorage.removeItem(TOKEN_KEY)
  } catch {
    // Storage can be unavailable (private mode, sandboxed frames); the token then lasts for this tab only.
  }
}

function authHeaders(): Record<string, string> {
  const t = getToken()
  return t ? { Authorization: `Bearer ${t}` } : {}
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
    throw new ApiError(res.status, msg)
  }
  if (res.status === 204 || res.status === 202) return undefined as T
  return res.json() as Promise<T>
}

function post<T>(path: string, body: unknown): Promise<T> {
  return request<T>(path, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) })
}

export const api = {
  stats: () => request<Stats>('/api/stats'),
  libraryStats: () => request<LibraryStats>('/api/stats/library'),
  packs: (archived = false) => request<Pack[]>(`/api/packs${archived ? '?archived=true' : ''}`),
  archive: (items: ArchiveItems) => post<ArchiveCount>('/api/archive', items),
  restore: (items: ArchiveItems) => post<ArchiveCount>('/api/archive/restore', items),
  /** Delete for good: these, or with `all` everything in the archive. Only archived things are deleted. */
  purge: (items: ArchiveItems & { all?: boolean }) => post<ArchiveCount>('/api/archive/purge', items),
  bulkAssets: (edit: BulkEdit) => post<{ updated: number }>('/api/assets/bulk', edit),
  deletePack: (id: string) => request<void>(`/api/packs/${id}`, { method: 'DELETE', keepalive: true }),
  retagPack: (id: string, body: { rules?: boolean; llm?: boolean }) =>
    request<void>(`/api/packs/${id}/retag`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    }),
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
  setPackContext: (id: string, ctx: Omit<PackContext, 'readme'>) =>
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
export const thumbUrl = (a: { id: string }) => `/api/assets/${a.id}/thumb`
export const bundleUrl = (a: Asset, animations = false) => `/api/assets/${a.id}/bundle.zip${animations ? '?animations=1' : ''}`
export const packZipUrl = (packId: string) => `/api/packs/${packId}/bundle.zip`
export const collectionZipUrl = (id: string, sources = false) =>
  `/api/collections/${id}/bundle.zip${sources ? '?sources=1' : ''}`
/** Several assets in one zip, each with what it needs (one folder per pack when they come from several). */
export const selectionZipUrl = (ids: string[], deps = true, sources = false) =>
  `/api/bundle.zip?ids=${ids.join(',')}${deps ? '' : '&deps=0'}${sources ? '&sources=1' : ''}`
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
