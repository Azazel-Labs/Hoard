// Typed client for the hoardd API: its requests and the addresses of its files. The types it speaks are in
// types/, and come from here too.
import { language } from './i18n'
import type {
  ArchiveCount, ArchiveItems, Asset, AssetKind, BulkEdit, Collection, ConversionPreview, ConversionRules,
  ConversionStatus, CopyGroups, CreditsRequest, Facet, Facing, FolderListing, LibraryStats, Pack, PackContext,
  Project, ProjectUse, SavedSearch, SearchParams, SearchResult, SheetData, Stats, StoreMatches, ThumbSource,
  ThumbUpload, UnrealPackStatus, UsageItems,
} from './types/library'
import type {
  AccessToken, Account, AccountSource, ActivityItem, AuthConfig, AuthSettings, AuthStatus, CertCheck, ChatGptStatus,
  Connection, ExampleFile, FormatTrial, HttpsChange, HttpsStatus, IssuePick, Issues, LdapConfig, LdapTestResult,
  LibraryLocation, LocalModels, LocationList, Maintenance, ModelInfo, NewLocation, OAuthConfig, Profile, Role,
  Settings, SettingsPatch, StoreFormats, UnreadFiles, UpdateHookCall, UpdateStatus,
} from './types/server'

export type * from './types/library'
export type * from './types/server'

/** Facings set in this tab, by pack: a model opened again shows the new one before its record catches up. */
export const packFacings = new Map<string, Facing>()

/** The server omits empty link/derivative lists; give the UI arrays to work with. */
function normalize(a: Asset): Asset {
  return { ...a, links: a.links ?? [], linked_from: a.linked_from ?? [], derivatives: a.derivatives ?? [] }
}

/**
 * Sent with every request. The session cookie does the signing in; this header tells the server the request came
 * from the portal and not from another site's page riding on the cookie.
 */
export function authHeaders(): Record<string, string> {
  // The server's messages (errors, mostly) come back in the portal's language.
  return { 'X-Hoard': '1', 'Accept-Language': language() }
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

/** The error a failed response says it is: the server's message, or else the status line. */
async function failure(res: Response): Promise<ApiError> {
  let msg = `${res.status} ${res.statusText}`
  try {
    const body = await res.json()
    if (body?.error) msg = body.error
  } catch {
    // Non-JSON error body; keep the status line.
  }
  return new ApiError(res.status, msg)
}

async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const res = await fetch(path, { ...init, headers: { ...authHeaders(), ...(init.headers ?? {}) } })
  if (!res.ok) {
    if (res.status === 401 && !path.startsWith('/api/auth/')) signInNeeded()
    throw await failure(res)
  }
  if (res.status === 204 || res.status === 202) return undefined as T
  return res.json() as Promise<T>
}

function post<T>(path: string, body: unknown): Promise<T> {
  return request<T>(path, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) })
}

export const api = {
  /** A thumbnail this browser drew (a model in its 3D viewer, a Lottie animation); kept unless the one there came from a better source. */
  putThumb: (id: string, png: Blob, source: ThumbSource) =>
    request<ThumbUpload>(`/api/assets/${id}/thumb?source=${source}`, {
      method: 'PUT',
      headers: { 'Content-Type': png.type || 'image/png' },
      body: png,
    }),
  /** A noise made into the microphone (WAV), read as the shape of a sound: what `imitation` takes to find sounds like it. */
  imitation: (wav: Blob) =>
    request<{ imitation: string; seconds: number }>('/api/imitation', {
      method: 'POST',
      headers: { 'Content-Type': 'audio/wav' },
      body: wav,
    }),
  /** A Lottie animation's JSON, a .lottie's with its images inlined. */
  lottie: (id: string) => request<import('./lottie').LottieDoc>(`/api/assets/${id}/lottie.json`),
  stats: () => request<Stats>('/api/stats'),
  sheet: (id: string) => request<SheetData>(`/api/assets/${id}/derivative/sheet`),
  libraryStats: () => request<LibraryStats>('/api/stats/library'),
  duplicates: (offset = 0, limit = 50) => request<CopyGroups>(`/api/duplicates?offset=${offset}&limit=${limit}`),
  packs: (archived = false) => request<Pack[]>(`/api/packs${archived ? '?archived=true' : ''}`),
  /** Only the packs being uploaded, ingested or analyzed: cheap to ask after while they are. */
  busyPacks: () => request<Pack[]>('/api/packs?busy=true'),
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
  /** Turn every model in a pack; their thumbnails are drawn again. */
  setPackFacing: async (id: string, facing: Facing) => {
    const pack = await request<Pack>(`/api/packs/${id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ facing }),
    })
    packFacings.set(id, pack.facing ?? facing)
    return pack
  },
  retagPack: (id: string, body: { rules?: boolean; llm?: boolean }) =>
    request<void>(`/api/packs/${id}/retag`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    }),
  readPackFiles: (id: string) => request<void>(`/api/packs/${id}/read-files`, { method: 'POST' }),
  /** `signal` stops it when the results are no longer wanted, and the server stops working them out. */
  search(p: SearchParams, signal?: AbortSignal): Promise<SearchResult> {
    const qs = new URLSearchParams()
    if (p.q) qs.set('q', p.q)
    if (p.kind) qs.set('kind', p.kind)
    if (p.tags?.length) qs.set('tags', p.tags.join(','))
    if (p.pack) qs.set('pack', p.pack)
    if (p.pack && p.folder !== undefined) qs.set('folder', p.folder)
    if (p.sort) qs.set('sort', p.sort)
    qs.set('limit', String(p.limit ?? 60))
    if (p.offset) qs.set('offset', String(p.offset))
    if (p.facets === false) qs.set('facets', 'false')
    if (p.hide_supporting) qs.set('hide_supporting', 'true')
    if (p.collection) qs.set('collection', p.collection)
    if (p.project) qs.set('project', p.project)
    if (p.archived) qs.set('archived', 'true')
    if (p.similar) qs.set('similar', p.similar)
    if (p.color) qs.set('color', p.color)
    if (p.imitation) qs.set('imitation', p.imitation)
    if (p.bpm_min !== undefined) qs.set('bpm_min', String(p.bpm_min))
    if (p.bpm_max !== undefined) qs.set('bpm_max', String(p.bpm_max))
    if (p.key) qs.set('key', p.key)
    if (p.compatible_keys) qs.set('compatible_keys', 'true')
    if (p.favorites) qs.set('favorites', 'true')
    return request<SearchResult>(`/api/assets?${qs}`, { signal }).then((r) => ({ ...r, items: r.items.map(normalize) }))
  },
  asset: (id: string) => request<Asset>(`/api/assets/${id}`).then(normalize),
  /** Where a web page in a pack opens sandboxed, with its pictures, styles and scripts loading from beside it. */
  assetSite: (id: string) => request<{ url: string; expires: number }>(`/api/assets/${id}/site`),
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
  profile: (after: number) => request<Profile>(`/api/admin/profile?after=${after}`),
  updateStatus: () => request<UpdateStatus>('/api/admin/update'),
  /** Call the update address now, to see that it's wired up. */
  callUpdateHook: () => request<UpdateHookCall>('/api/admin/update/hook', { method: 'POST' }),
  previewConversions: (rules: ConversionRules) =>
    request<ConversionPreview>('/api/admin/conversions/preview', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ rules }),
    }),
  /** Convert the files already in the library that the saved rules match; returns how many packs were queued. */
  runConversions: () => request<{ packs: number }>('/api/admin/conversions/run', { method: 'POST' }),
  /** Put back the file a conversion rule replaced. */
  unconvert: (id: string) => request<{ pack_id: string }>(`/api/assets/${id}/unconvert`, { method: 'POST' }),
  /** A SoundFont from this computer, kept on the server and made the one MIDI plays with. */
  uploadSoundFont: (file: File) =>
    request<Settings>(`/api/admin/soundfont?name=${encodeURIComponent(file.name)}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/octet-stream' },
      body: file,
    }),
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
  setup: (username: string, password: string, code = '') =>
    post<{ user: { name: string } }>('/api/auth/setup', { username, password, code }),
  login: (username: string, password: string) =>
    post<{ user: { name: string } }>('/api/auth/login', { username, password }),
  logout: () => request<void>('/api/auth/logout', { method: 'POST' }),
  changePassword: (current: string, next: string) =>
    post<void>('/api/auth/password', { current, new: next }),
  accounts: () => request<{ users: Account[] }>('/api/admin/users'),
  addAccount: (username: string, password: string, role: Role, source: AccountSource = 'internal') =>
    post<void>('/api/admin/users', { username, password, role, source }),
  authSettings: () => request<AuthSettings>('/api/admin/auth'),
  saveAuthSettings: (config: AuthConfig) =>
    request<AuthSettings>('/api/admin/auth', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(config),
    }),
  testLdap: (kind: 'ldap' | 'ad', config: LdapConfig, username?: string, password?: string) =>
    post<LdapTestResult>('/api/admin/auth/ldap/test', { kind, config, username, password }),
  discoverOAuth: (issuer: string) =>
    post<Pick<OAuthConfig, 'authorization_url' | 'token_url' | 'userinfo_url'>>('/api/admin/auth/oauth/discover', { issuer }),
  setRole: (id: number, role: Role) =>
    request<void>(`/api/admin/users/${id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ role }),
    }),
  removeAccount: (id: number) => request<void>(`/api/admin/users/${id}`, { method: 'DELETE' }),
  tokens: () => request<{ tokens: AccessToken[] }>('/api/admin/tokens'),
  makeToken: (name: string, role: Role) =>
    post<{ id: number; name: string; token: string }>('/api/admin/tokens', { name, role }),
  revokeToken: (id: number) => request<void>(`/api/admin/tokens/${id}`, { method: 'DELETE' }),
  projects: () => request<{ projects: Project[] }>('/api/projects'),
  usage: (items: UsageItems) => post<{ projects: ProjectUse[] }>('/api/usage', items),
  forgetProject: (name: string) => request<void>(`/api/projects/${encodeURIComponent(name)}`, { method: 'DELETE' }),
  favorites: () => request<{ assets: string[] }>('/api/me/favorites'),
  setFavorites: (change: { add?: string[]; remove?: string[] }) => post<{ assets: string[] }>('/api/me/favorites', change),
  savedSearches: () => request<{ searches: SavedSearch[] }>('/api/me/searches'),
  saveSearch: (name: string, query: string) => post<{ searches: SavedSearch[] }>('/api/me/searches', { name, query }),
  deleteSearch: (id: number) => request<void>(`/api/me/searches/${id}`, { method: 'DELETE' }),
  /** How many assets of each kind this account has collected into its hoard, or null before it has collected any. */
  hoard: () => request<{ seen: Record<string, number> | null }>('/api/me/hoard'),
  setHoard: (seen: Record<string, number>) =>
    request<{ seen: Record<string, number> }>('/api/me/hoard', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ seen }),
    }),
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
  storeFormats: () => request<StoreFormats>('/api/admin/store-formats'),
  /** As last worked out, or once worked out again when `fresh`. */
  unreadFiles: (fresh = false) => request<UnreadFiles>(`/api/admin/store-formats/unread${fresh ? '?fresh=true' : ''}`),
  tryStoreFormat: (toml: string, example?: ExampleFile | null) =>
    post<FormatTrial>('/api/admin/store-formats/try', { toml, example: example ?? null }),
  saveStoreFormat: (toml: string, replaces?: string | null) =>
    post<{ file: string; packs: number }>('/api/admin/store-formats', { toml, replaces: replaces ?? null }),
  removeStoreFormat: (file: string) => request<void>(`/api/admin/store-formats/${encodeURIComponent(file)}`, { method: 'DELETE' }),
  /** Start or look in on a "download as" conversion; answers when it moves on, or after `wait` seconds. */
  asStatus: (a: { id: string }, format: string, max?: number, wait = 5) =>
    request<ConversionStatus>(`/api/assets/${a.id}/as/${format}/status?wait=${wait}${max ? `&max=${max}` : ''}`),
}

export const fileUrl = (a: { id: string }, download = false) => `/api/assets/${a.id}/file${download ? '?download=1' : ''}`
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
/** An animation's looping movie (an animated WebP), when the server has made one. */
export const motionUrl = (a: { id: string; has_motion?: boolean; thumb_rev?: string | null }) =>
  a.has_motion ? `/api/assets/${a.id}/derivative/motion${a.thumb_rev ? `?v=${a.thumb_rev}` : ''}` : null
/** An image at most 2048 px on its longer side (the image itself when it's that small and browsers read it). */
export const liteUrl = (a: { id: string }) => `/api/assets/${a.id}/lite`
/** An asset and what it needs, zipped; with its animations, or (for an animation) with a character by id. */
export function bundleUrl(a: Asset, animations = false, character = '') {
  const q = new URLSearchParams()
  if (animations) q.set('animations', '1')
  if (character) q.set('character', character)
  return `/api/assets/${a.id}/bundle.zip${q.size ? `?${q}` : ''}`
}
export const packZipUrl = (packId: string) => `/api/packs/${packId}/bundle.zip`
export const collectionZipUrl = (id: string, sources = false) =>
  `/api/collections/${id}/bundle.zip${sources ? '?sources=1' : ''}`
/** Several assets in one zip, each with what it needs (one folder per pack when they come from several). With
 *  `glb`, every model is a GLB standing Y up and facing forward, the textures beside them in one shared folder. */
export const selectionZipUrl = (ids: string[], deps = true, sources = false, glb = false) =>
  `/api/bundle.zip?ids=${ids.join(',')}${deps ? '' : '&deps=0'}${sources ? '&sources=1' : ''}${glb ? '&glb=1' : ''}`
/** CREDITS.md for a pack, or for the packs a collection's assets come from. */
export const packCreditsUrl = (packId: string) => `/api/packs/${packId}/credits?format=md`
export const collectionCreditsUrl = (id: string) => `/api/collections/${id}/credits?format=md`

/** Download CREDITS.md for a request (a POST, so it's fetched and then saved). */
export function downloadCredits(body: CreditsRequest): Promise<void> {
  return postDownload('/api/credits?format=md', body, 'CREDITS.md')
}

/** Download a zip of a store file format, its sample and what the sample reads as, to send in for everyone. */
export function downloadFormatShare(toml: string, example: ExampleFile, file: string): Promise<void> {
  return postDownload('/api/admin/store-formats/share', { toml, example }, file)
}

/** POST, then save what comes back as a file. */
async function postDownload(path: string, body: unknown, file: string): Promise<void> {
  const res = await fetch(path, {
    method: 'POST',
    headers: { ...authHeaders(), 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw await failure(res)
  const url = URL.createObjectURL(await res.blob())
  const a = document.createElement('a')
  a.href = url
  a.download = file
  a.click()
  setTimeout(() => URL.revokeObjectURL(url), 10_000)
}

/** `rel` from the folder `dir` (both pack paths), with `.` and `..` worked out. */
export function joinPath(dir: string, rel: string): string {
  const out = dir ? dir.split('/') : []
  for (const part of rel.split('/')) {
    if (part === '..') out.pop()
    else if (part && part !== '.') out.push(part)
  }
  return out.join('/')
}

/** A pack file by its original path; relative references inside model files resolve against this URL. */
export const packFileUrl = (packId: string, path: string) =>
  `/api/packs/${packId}/files/${path.split('/').map(encodeURIComponent).join('/')}`

/** Asset kinds the browser viewer can open (plus anything the server converted to .glb). */
export const VIEWABLE_EXTS = new Set(['glb', 'gltf', 'obj', 'fbx', 'stl', 'ply', 'dae', '3ds', '3mf', 'usd', 'usda', 'usdc', 'usdz', 'skp'])
