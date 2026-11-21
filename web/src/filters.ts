// What the library is narrowed to: the search box, the sidebar's choices, and "more like this". They live in the
// query string (bookmarkable searches), and a saved search keeps them the same way.
import { computed, reactive, ref } from 'vue'
import { t } from './i18n'
import { findKey } from './keys'
import { mine } from './mine'

export type Named = { id: string; name: string }

export const filters = reactive({
  q: '', kind: '', tags: [] as string[], pack: null as Named | null, collection: null as Named | null, sort: '',
  /** With a pack: the folder of it being browsed (`''` its top), as if it were still on disk; null for everything. */
  folder: null as string | null,
  /** Only your favorites. */
  favorites: false,
  /** Only what this project (its name) has. */
  project: '',
  /** Only what's been gone over (`yes`) or not yet (`no`); `''` for both. Goes with any other filter. */
  reviewed: '' as '' | 'yes' | 'no',
  /** Assets that look like this one, most alike first. */
  similar: null as Named | null,
  /** Assets with much of this color (hex), most of it first. */
  color: '',
  /** Sounds shaped like this word imitating them (`whooom`), most alike first. */
  imitation: '',
  /** Music at this tempo (BPM) or faster, or slower; `''` for any. */
  bpmMin: '' as number | '',
  bpmMax: '' as number | '',
  /** Music in this key (`a-minor`), and with `mixKeys` the keys that mix with it. */
  key: '',
  mixKeys: true,
})

/** The music filters, cleared. */
export const NO_MUSIC = { bpmMin: '', bpmMax: '', key: '' } as const

/** Anything narrowing the library at all. */
export const hasFilters = computed(
  () => !!(filters.q || filters.kind || filters.tags.length || filters.pack || filters.collection || filters.similar || filters.color || filters.imitation || filters.favorites || filters.project || filters.reviewed ||
    filters.bpmMin !== '' || filters.bpmMax !== '' || filters.key),
)
/** Back to the whole library (the order chosen stays). */
export function clearFilters() {
  Object.assign(filters, { q: '', kind: '', tags: [], pack: null, folder: null, collection: null, favorites: false, project: '', reviewed: '', similar: null, color: '', imitation: '', ...NO_MUSIC })
}

/** How a noise made into the microphone travels as `imitation` (its shape, as the server measured it). */
export const HEARD = 'heard:'
export const heardNoise = computed(() => filters.imitation.startsWith(HEARD))

/** Each filter, for watching them all; `q` first, since typing in the search box waits for a pause. */
export const filterSources = [
  () => filters.q,
  () => filters.kind,
  () => filters.tags.join(','),
  () => filters.sort,
  () => filters.pack?.id,
  () => filters.folder,
  () => filters.collection?.id,
  () => filters.similar?.id,
  () => filters.color,
  () => filters.imitation,
  () => filters.favorites,
  () => filters.project,
  () => filters.reviewed,
  () => filters.bpmMin,
  () => filters.bpmMax,
  () => filters.key,
  () => filters.mixKeys,
]

/** Where the filters say the library is, without the words typed: going somewhere new is a new history entry. */
export function filterPlace(): string {
  return [
    filters.kind, filters.tags.join(','), filters.sort, filters.pack?.id, filters.folder, filters.collection?.id,
    filters.similar?.id, filters.color, filters.imitation, filters.favorites, filters.project, filters.reviewed,
    filters.bpmMin, filters.bpmMax, filters.key, filters.mixKeys,
  ].join('|')
}

/** Set the filters from a query string (the address's, or a saved search's). */
export function readFilters(search: string) {
  const u = new URLSearchParams(search)
  filters.q = u.get('q') ?? ''
  filters.kind = u.get('kind') ?? ''
  filters.tags = (u.get('tags') ?? '').split(',').filter(Boolean)
  filters.sort = u.get('sort') ?? ''
  const pack = u.get('pack')
  filters.pack = pack ? { id: pack, name: u.get('pack_name') ?? t('app-unnamed-pack') } : null
  filters.folder = pack ? u.get('folder') : null
  const col = u.get('collection')
  filters.collection = col ? { id: col, name: u.get('collection_name') ?? t('app-unnamed-collection') } : null
  const like = u.get('similar')
  filters.similar = like ? { id: like, name: u.get('similar_name') ?? t('app-unnamed-asset') } : null
  const color = u.get('color') ?? ''
  filters.color = /^#[0-9a-f]{6}$/i.test(color) ? color.toLowerCase() : ''
  filters.imitation = u.get('imitation') ?? ''
  const bpm = (k: string) => (u.get(k) && Number.isFinite(Number(u.get(k))) ? Number(u.get(k)) : '')
  filters.bpmMin = bpm('bpm_min')
  filters.bpmMax = bpm('bpm_max')
  filters.key = findKey(u.get('key') ?? '')?.value ?? ''
  filters.mixKeys = u.get('mix') !== '0'
  filters.favorites = u.get('fav') === '1'
  filters.project = u.get('project') ?? ''
  const rev = u.get('rev')
  filters.reviewed = rev === '1' ? 'yes' : rev === '0' ? 'no' : ''
}

/** The filters as a query string: what goes in the address, and what a saved search keeps. */
export function filterQuery(): string {
  const u = new URLSearchParams()
  if (filters.q.trim()) u.set('q', filters.q.trim())
  if (filters.kind) u.set('kind', filters.kind)
  if (filters.tags.length) u.set('tags', filters.tags.join(','))
  if (filters.sort) u.set('sort', filters.sort)
  if (filters.pack) {
    u.set('pack', filters.pack.id)
    u.set('pack_name', filters.pack.name)
    if (filters.folder !== null) u.set('folder', filters.folder)
  }
  if (filters.collection) {
    u.set('collection', filters.collection.id)
    u.set('collection_name', filters.collection.name)
  }
  if (filters.similar) {
    u.set('similar', filters.similar.id)
    u.set('similar_name', filters.similar.name)
  }
  if (filters.color) u.set('color', filters.color)
  if (filters.imitation) u.set('imitation', filters.imitation)
  if (filters.bpmMin !== '') u.set('bpm_min', String(filters.bpmMin))
  if (filters.bpmMax !== '') u.set('bpm_max', String(filters.bpmMax))
  if (filters.key) {
    u.set('key', filters.key)
    if (!filters.mixKeys) u.set('mix', '0')
  }
  if (filters.favorites) u.set('fav', '1')
  if (filters.project) u.set('project', filters.project)
  if (filters.reviewed) u.set('rev', filters.reviewed === 'yes' ? '1' : '0')
  return u.toString()
}

/** The saved search the filters were last set from (or saved as): what "Update" keeps the filters as. */
export const fromSaved = ref<number | null>(null)
export const savedBasis = computed(() => mine.searches.find((s) => s.id === fromSaved.value) ?? null)

/** The saved search you're looking at, if it's one. */
export const savedNow = computed(() => {
  const q = filterQuery()
  return q ? (mine.searches.find((s) => s.query === q) ?? null) : null
})
