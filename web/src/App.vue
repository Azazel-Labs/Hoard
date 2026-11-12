<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import {
  api, filesFromDrop, getToken, KIND_LABEL, selectionZipUrl, setToken,
  type Asset, type AssetKind, type Collection, type Facet, type Pack, type Stats, type UploadFile,
} from './api'
import { canPreview, stop as stopPreview, toggle as togglePreview } from './player'
import { prefs, toggleIn } from './prefs'
import ActivityIndicator from './components/ActivityIndicator.vue'
import { looksLikeFolderOfPacks, splitPerSubfolder } from './uploader'
import UploadQueue from './components/UploadQueue.vue'
import AssetDetail from './components/AssetDetail.vue'
import AssetTile from './components/AssetTile.vue'
import CollectionPicker from './components/CollectionPicker.vue'
import PacksView from './components/PacksView.vue'
import TagChip from './components/TagChip.vue'

type View = 'library' | 'packs'
const PAGE = 60
const THEMES = [
  { value: 'system', label: 'Match system theme' },
  { value: 'light', label: 'Light theme' },
  { value: 'dark', label: 'Dark theme' },
] as const
/** Values shown per filter group before "more". */
const FACET_PREVIEW = 8

const view = ref<View>(location.hash === '#packs' ? 'packs' : 'library')
type Named = { id: string; name: string }
const filters = reactive({
  q: '', kind: '', tags: [] as string[], pack: null as Named | null, collection: null as Named | null, sort: '',
})
const collections = ref<Collection[]>([])
const items = ref<Asset[]>([])
const total = ref(0)
const supportingHidden = ref(0)
const kindFacets = ref<Facet[]>([])
const tagFacets = ref<Facet[]>([])
const loading = ref(false)
const selected = ref<Asset | null>(null)
const packs = ref<Pack[]>([])
const stats = ref<Stats | null>(null)
const toast = ref<{ text: string; kind: 'error' | 'ok' } | null>(null)
const showFilters = ref(false)
const tokenDraft = ref(getToken())
const showToken = ref(false)
const searchInput = ref<HTMLInputElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)
const gridEl = ref<HTMLElement | null>(null)
const sentinel = ref<HTMLElement | null>(null)

type Group = { name: string; files: UploadFile[] }
const queue = ref<InstanceType<typeof UploadQueue> | null>(null)
const folderInput = ref<HTMLInputElement | null>(null)
/** A dropped/picked folder that contains several packs: ask how to import it. */
const pendingSplit = ref<{ group: Group; parts: number } | null>(null)
const dragDepth = ref(0)

// ------------------------------------------------------------------ data

let searchSeq = 0
async function runSearch(append = false) {
  const seq = ++searchSeq
  loading.value = true
  try {
    const res = await api.search({
      q: filters.q.trim() || undefined,
      kind: filters.kind || undefined,
      tags: filters.tags,
      pack: filters.pack?.id,
      collection: filters.collection?.id,
      sort: filters.sort || undefined,
      limit: PAGE,
      offset: append ? items.value.length : 0,
      hide_supporting: !prefs.showSupporting,
    })
    if (seq !== searchSeq) return
    items.value = append ? [...items.value, ...res.items] : res.items
    total.value = res.total
    supportingHidden.value = res.supporting_hidden ?? 0
    if (!append) {
      kindFacets.value = res.kinds
      tagFacets.value = res.tags
    }
  } catch (e) {
    showError(`Search failed: ${(e as Error).message}`)
  } finally {
    if (seq === searchSeq) loading.value = false
  }
}

let debounce: ReturnType<typeof setTimeout> | undefined
watch(() => filters.q, () => {
  clearTimeout(debounce)
  debounce = setTimeout(() => runSearch(), 200)
})
watch(() => [filters.kind, filters.tags.join(','), filters.pack?.id, filters.collection?.id, filters.sort, prefs.showSupporting], () => {
  checked.value = []
  runSearch()
})

async function refreshPacks() {
  try {
    const [p, s, c] = await Promise.all([api.packs(), api.stats(), api.collections()])
    const wasBusy = packs.value.some(busy)
    packs.value = p
    stats.value = s
    collections.value = c
    // A pack just finished: its assets are now searchable.
    if (wasBusy && !p.some(busy)) runSearch()
  } catch (e) {
    showError(`Couldn't reach the server: ${(e as Error).message}`)
  }
}
const busy = (p: Pack) => p.status === 'ingesting' || p.status === 'analyzing' || p.status === 'uploading'

let poll: ReturnType<typeof setInterval> | undefined
function schedulePoll() {
  clearInterval(poll)
  poll = setInterval(() => {
    if (packs.value.some(busy)) {
      refreshPacks()
      if (packs.value.some((p) => p.status === 'analyzing')) runSearch()
    }
  }, 1500)
}

// Load the next page when the end of the grid scrolls into view.
let observer: IntersectionObserver | undefined
watch(sentinel, (el) => {
  observer?.disconnect()
  if (!el) return
  observer = new IntersectionObserver((entries) => {
    if (entries.some((e) => e.isIntersecting) && !loading.value && items.value.length < total.value) runSearch(true)
  }, { rootMargin: '600px' })
  observer.observe(el)
})

// ------------------------------------------------------------------ filters

function toggleTag(tag: string) {
  filters.tags = filters.tags.includes(tag) ? filters.tags.filter((t) => t !== tag) : [...filters.tags, tag]
  view.value = 'library'
}
function setKind(kind: string) {
  filters.kind = filters.kind === kind ? '' : kind
}
function browsePack(p: Pack) {
  Object.assign(filters, { pack: { id: p.id, name: p.name }, collection: null, kind: '', tags: [], q: '' })
  view.value = 'library'
}
function browseCollection(c: Collection | Named) {
  const same = filters.collection?.id === c.id
  Object.assign(filters, { collection: same ? null : { id: c.id, name: c.name }, pack: null, kind: '', tags: [], q: '' })
  view.value = 'library'
  showFilters.value = false
}
function clearFilters() {
  Object.assign(filters, { q: '', kind: '', tags: [], pack: null, collection: null })
}
const hasFilters = computed(() => !!(filters.q || filters.kind || filters.tags.length || filters.pack || filters.collection))

// ------------------------------------------------------------------ collections

async function refreshCollections() {
  try {
    collections.value = await api.collections()
  } catch {
    // The packs poll will retry; collections aren't worth an error toast on their own.
  }
}

/** Add assets to a collection (creating it first when given a new name). */
async function addToCollection(ids: string[], target: { id?: string; create?: string }) {
  if (!ids.length) return
  try {
    const id = target.id ?? (await api.createCollection(target.create!)).id
    const c = await api.collectionMembers(id, { add: ids })
    await refreshCollections()
    showOk(`Added ${ids.length} ${ids.length === 1 ? 'asset' : 'assets'} to ${c.name}`)
    if (selected.value && ids.includes(selected.value.id)) openAsset(selected.value.id)
  } catch (e) {
    showError(`Couldn't add to the collection: ${(e as Error).message}`)
  }
}

async function removeFromCollection(ids: string[], id: string) {
  try {
    const c = await api.collectionMembers(id, { remove: ids })
    await refreshCollections()
    showOk(`Removed ${ids.length} from ${c.name}`)
    if (filters.collection?.id === id) {
      checked.value = []
      runSearch()
    }
    if (selected.value && ids.includes(selected.value.id)) openAsset(selected.value.id)
  } catch (e) {
    showError(`Couldn't remove from the collection: ${(e as Error).message}`)
  }
}

const NS_LABEL: Record<string, string> = {
  '': 'Subject', style: 'Style', theme: 'Theme', material: 'Material', map: 'PBR map', color: 'Color',
  mood: 'Mood', genre: 'Genre', license: 'License', res: 'Resolution', duration: 'Length', poly: 'Poly count',
  format: 'Format', weight: 'Weight', tech: 'Technical', blender: 'Blender version', depth: 'Bit depth',
  ucs: 'Sound category (UCS)', bpm: 'Tempo', root: 'Root note', level: 'Loudness',
  ue: 'Unreal class', engine: 'Engine', extracted: 'Recovered source', script: 'Writing system',
}
// Plain tags that are measured facts rather than subjects; they get their own group so "Subject" stays useful.
const TECH = new Set(['pot', 'square', 'alpha', 'mono', 'stereo', 'surround', 'tileable', 'frames', 'rigged', 'animated', 'loop', 'one-shot', 'clipped'])
const NS_ORDER = ['', 'ue', 'ucs', 'style', 'theme', 'material', 'map', 'mood', 'genre', 'color', 'tech', 'duration', 'bpm', 'root', 'level', 'res', 'depth', 'poly', 'blender', 'license', 'format']

const tagGroups = computed(() => {
  const groups = new Map<string, Facet[]>()
  for (const f of tagFacets.value) {
    const i = f.value.indexOf(':')
    const ns = i > 0 ? f.value.slice(0, i) : TECH.has(f.value) ? 'tech' : ''
    if (ns === 'kind' || ns === 'set') continue
    if (!groups.has(ns)) groups.set(ns, [])
    groups.get(ns)!.push(f)
  }
  const rank = (ns: string) => (NS_ORDER.indexOf(ns) === -1 ? 50 : NS_ORDER.indexOf(ns))
  return [...groups.entries()]
    .sort(([a], [b]) => rank(a) - rank(b) || a.localeCompare(b))
    .map(([ns, facets]) => {
      // Active values always stay visible, even in a collapsed or shortened group.
      const active = facets.filter((f) => filters.tags.includes(f.value))
      const open = !prefs.collapsed.includes(ns)
      const long = prefs.expanded.includes(ns) ? facets : facets.slice(0, FACET_PREVIEW)
      const shown = open ? [...new Set([...long, ...active])] : active
      return { ns, label: NS_LABEL[ns] ?? ns, open, shown, hidden: open ? facets.length - shown.length : 0, all: facets.length }
    })
})
const facetLabel = (ns: string, value: string) => (ns && ns !== 'tech' ? value.slice(ns.length + 1) : value)

// ------------------------------------------------------------------ selection (multi-select)

const checked = ref<string[]>([])
const checkedSet = computed(() => new Set(checked.value))
let anchor: string | null = null

function toggleCheck(a: Asset, e?: MouseEvent | KeyboardEvent) {
  if (e?.shiftKey && anchor) {
    const ids = items.value.map((x) => x.id)
    const [i, j] = [ids.indexOf(anchor), ids.indexOf(a.id)]
    if (i >= 0 && j >= 0) {
      const range = ids.slice(Math.min(i, j), Math.max(i, j) + 1)
      checked.value = [...new Set([...checked.value, ...range])]
      return
    }
  }
  checked.value = toggleIn(checked.value, a.id)
  anchor = a.id
}

/** Plain click opens; Ctrl/Cmd-click or Shift-click selects; while selecting, any click selects. */
function onTileOpen(a: Asset, e: MouseEvent | KeyboardEvent) {
  if (e.metaKey || e.ctrlKey || e.shiftKey || checked.value.length) {
    toggleCheck(a, e)
    return
  }
  openAsset(a)
}

const bulkTag = ref('')
const bulkBusy = ref(false)
async function applyBulkTag() {
  const tag = bulkTag.value.trim()
  if (!tag || !checked.value.length) return
  bulkBusy.value = true
  let failed = 0
  for (const id of checked.value) {
    try {
      const updated = await api.patchAsset(id, { add_tags: [tag] })
      onUpdated(updated)
    } catch {
      failed++
    }
  }
  bulkBusy.value = false
  bulkTag.value = ''
  if (failed) showError(`Couldn't tag ${failed} of ${checked.value.length} assets. Is the token set?`)
  else showOk(`Tagged ${checked.value.length} ${checked.value.length === 1 ? 'asset' : 'assets'} with ${tag}`)
}
// Unreal assets with a recovered WAV/PNG can go into the zip as that file instead of the .uasset.
const preferSources = ref(true)
const checkedWithSource = computed(
  () => items.value.filter((a) => checkedSet.value.has(a.id) && typeof a.meta.source_ext === 'string').length,
)
const selectionUrl = computed(() =>
  selectionZipUrl(checked.value, true, preferSources.value && checkedWithSource.value > 0),
)

// ------------------------------------------------------------------ uploads

/** Queue groups for import; a lone folder full of packs gets a choice first. */
function importGroups(groups: Group[]) {
  if (groups.length === 1) {
    const parts = looksLikeFolderOfPacks(groups[0].files)
    if (parts) {
      pendingSplit.value = { group: groups[0], parts }
      return
    }
  }
  queue.value?.enqueue(groups)
}

function resolveSplit(split: boolean) {
  const p = pendingSplit.value
  pendingSplit.value = null
  if (!p) return
  queue.value?.enqueue(split ? splitPerSubfolder(p.group.name, p.group.files) : [p.group])
}

function onDragEnter(e: DragEvent) {
  if (e.dataTransfer?.types.includes('Files')) dragDepth.value++
}
function onDragLeave() {
  dragDepth.value = Math.max(0, dragDepth.value - 1)
}
async function onDrop(e: DragEvent) {
  dragDepth.value = 0
  if (!e.dataTransfer) return
  const groups = await filesFromDrop(e.dataTransfer)
  if (groups.length) importGroups(groups)
}
function onPick(e: Event) {
  const input = e.target as HTMLInputElement
  const files = [...(input.files ?? [])]
  input.value = ''
  importGroups(files.map((f) => ({ name: f.name, files: [{ file: f, path: f.name }] })))
}
/** Folder picker: webkitRelativePath is "Folder/sub/file.png". */
function onPickFolder(e: Event) {
  const input = e.target as HTMLInputElement
  const files = [...(input.files ?? [])].map((f) => ({ file: f, path: f.webkitRelativePath || f.name }))
  input.value = ''
  if (!files.length) return
  importGroups([{ name: files[0].path.split('/')[0], files }])
}
function onCreated() {
  refreshPacks()
}

// ------------------------------------------------------------------ misc

let toastTimer: ReturnType<typeof setTimeout> | undefined
function showToast(text: string, kind: 'error' | 'ok') {
  toast.value = { text, kind }
  clearTimeout(toastTimer)
  toastTimer = setTimeout(() => (toast.value = null), kind === 'error' ? 6000 : 2500)
}
const showError = (msg: string) => showToast(msg, 'error')
const showOk = (msg: string) => showToast(msg, 'ok')

/** Open an asset in the detail panel, upgrading the search hit to the full record (links, derivatives). */
async function openAsset(idOrAsset: string | Asset) {
  const id = typeof idOrAsset === 'string' ? idOrAsset : idOrAsset.id
  if (typeof idOrAsset !== 'string') selected.value = idOrAsset
  view.value = 'library'
  try {
    const full = await api.asset(id)
    if (typeof idOrAsset === 'string' || selected.value?.id === id) selected.value = full
  } catch (e) {
    showError(`Couldn't open that asset: ${(e as Error).message}`)
  }
}

/** Previous/next asset in the grid (detail panel arrows, keyboard). */
async function step(delta: number) {
  const i = items.value.findIndex((a) => a.id === selected.value?.id)
  const next = i + delta
  if (next >= items.value.length && items.value.length < total.value) await runSearch(true)
  const target = items.value[next]
  if (!target) return
  openAsset(target)
  focusTile(target.id)
}
const position = computed(() => {
  const i = items.value.findIndex((a) => a.id === selected.value?.id)
  return i < 0 ? null : { index: i, total: total.value, hasPrev: i > 0, hasNext: i < total.value - 1 }
})

function onUpdated(a: Asset) {
  if (selected.value?.id === a.id) selected.value = a
  items.value = items.value.map((x) => (x.id === a.id ? a : x))
}

function saveToken() {
  setToken(tokenDraft.value.trim())
  showToken.value = false
}

// ------------------------------------------------------------------ keyboard

const tileButtons = () => [...(gridEl.value?.querySelectorAll<HTMLButtonElement>('.hit') ?? [])]

function focusTile(id: string) {
  nextTick(() => {
    const el = gridEl.value?.querySelector<HTMLButtonElement>(`.hit[data-id="${id}"]`)
    el?.focus({ preventScroll: true })
    el?.scrollIntoView({ block: 'nearest' })
  })
}

/** Move focus through the grid like a file browser; with the detail open, it follows. */
function moveFocus(key: string) {
  const tiles = tileButtons()
  if (!tiles.length) return
  const current = tiles.indexOf(document.activeElement as HTMLButtonElement)
  const from = current >= 0 ? current : Math.max(0, items.value.findIndex((a) => a.id === selected.value?.id))
  const top = tiles[0].getBoundingClientRect().top
  // The first tile that starts a new row tells how many fit across (-1: everything fits on one row).
  const rowBreak = tiles.findIndex((t) => Math.abs(t.getBoundingClientRect().top - top) > 4)
  const perRow = rowBreak > 0 ? rowBreak : tiles.length
  const delta = { ArrowRight: 1, ArrowLeft: -1, ArrowDown: perRow, ArrowUp: -perRow, j: 1, k: -1 }[key] ?? 0
  const next = current < 0 ? from : Math.min(tiles.length - 1, Math.max(0, from + delta))
  const id = tiles[next].dataset.id
  if (!id) return
  focusTile(id)
  if (selected.value && selected.value.id !== id) {
    const a = items.value.find((x) => x.id === id)
    if (a) openAsset(a)
  }
  if (next >= tiles.length - perRow && items.value.length < total.value && !loading.value) runSearch(true)
}

function onKey(e: KeyboardEvent) {
  const target = e.target instanceof HTMLElement ? e.target : null
  const typing = target?.closest('input, textarea, select, [contenteditable]')
  if (e.key === 'Escape') {
    if (showFilters.value) showFilters.value = false
    else if (checked.value.length) checked.value = []
    else if (selected.value) selected.value = null
    return
  }
  if (typing || view.value !== 'library' || e.altKey) return
  if (e.key === '/') {
    e.preventDefault()
    searchInput.value?.focus()
    return
  }
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'a') {
    e.preventDefault()
    checked.value = items.value.map((a) => a.id)
    return
  }
  if (e.metaKey || e.ctrlKey) return
  if (['ArrowRight', 'ArrowLeft', 'ArrowDown', 'ArrowUp', 'j', 'k'].includes(e.key)) {
    // Arrow keys still scroll the detail panel when it has focus.
    if (target?.closest('.detail-pane') && e.key.startsWith('Arrow')) return
    e.preventDefault()
    moveFocus(e.key)
    return
  }
  const focused = items.value.find((a) => a.id === target?.dataset.id) ?? selected.value
  if (e.key === ' ' && focused) {
    // Space previews a sound instead of "clicking" the tile.
    if (canPreview(focused)) {
      e.preventDefault()
      togglePreview(focused)
    }
    return
  }
  if (e.key === 'x' && focused) {
    toggleCheck(focused)
  }
}

// ------------------------------------------------------------------ URL

// Filters live in the query string (bookmarkable searches); the hash holds the view or an open asset
// (#packs, #asset=<id>; MCP links use the latter).
function readUrl() {
  const u = new URLSearchParams(location.search)
  filters.q = u.get('q') ?? ''
  filters.kind = u.get('kind') ?? ''
  filters.tags = (u.get('tags') ?? '').split(',').filter(Boolean)
  filters.sort = u.get('sort') ?? ''
  const pack = u.get('pack')
  filters.pack = pack ? { id: pack, name: u.get('pack_name') ?? 'pack' } : null
  const col = u.get('collection')
  filters.collection = col ? { id: col, name: u.get('collection_name') ?? 'collection' } : null
}
function writeUrl() {
  const u = new URLSearchParams()
  if (filters.q.trim()) u.set('q', filters.q.trim())
  if (filters.kind) u.set('kind', filters.kind)
  if (filters.tags.length) u.set('tags', filters.tags.join(','))
  if (filters.sort) u.set('sort', filters.sort)
  if (filters.pack) {
    u.set('pack', filters.pack.id)
    u.set('pack_name', filters.pack.name)
  }
  if (filters.collection) {
    u.set('collection', filters.collection.id)
    u.set('collection_name', filters.collection.name)
  }
  const qs = u.toString()
  const hash = view.value === 'packs' ? '#packs' : selected.value ? `#asset=${selected.value.id}` : ''
  history.replaceState(null, '', `${location.pathname}${qs ? `?${qs}` : ''}${hash}`)
}
watch([view, () => selected.value?.id, () => filters.q, () => filters.kind, () => filters.tags.join(','), () => filters.sort, () => filters.pack?.id, () => filters.collection?.id], writeUrl)

watch(view, (v) => {
  if (v !== 'library') stopPreview()
  showFilters.value = false
})

onMounted(() => {
  readUrl()
  const deep = location.hash.match(/^#asset=([\w-]+)/)
  if (deep) openAsset(deep[1])
  runSearch()
  refreshPacks()
  schedulePoll()
  window.addEventListener('keydown', onKey)
})
onBeforeUnmount(() => {
  clearInterval(poll)
  observer?.disconnect()
  window.removeEventListener('keydown', onKey)
})

const busyPacks = computed(() => packs.value.filter(busy))
const firstLoad = computed(() => loading.value && !items.value.length)
</script>

<template>
  <div
    class="app"
    :class="{ 'with-detail': selected && view === 'library', 'filters-open': showFilters }"
    :style="{ '--tile': `${prefs.tile}px` }"
    @dragenter.prevent="onDragEnter"
    @dragover.prevent
    @dragleave="onDragLeave"
    @drop.prevent="onDrop"
  >
    <header class="top">
      <div class="brand" :title="stats?.version ? `Hoard ${stats.version}` : undefined">
        <svg viewBox="0 0 32 32" width="24" height="24" aria-hidden="true">
          <rect x="3" y="3" width="12" height="12" rx="2.5" fill="var(--k-texture)" />
          <rect x="17" y="3" width="12" height="12" rx="2.5" fill="var(--k-audio)" />
          <rect x="3" y="17" width="12" height="12" rx="2.5" fill="var(--k-model)" />
          <rect x="17" y="17" width="12" height="12" rx="2.5" fill="var(--k-font)" />
        </svg>
        <span>Hoard</span>
      </div>

      <div class="search">
        <label for="q" class="visually-hidden">Search assets</label>
        <svg class="search-icon" viewBox="0 0 16 16" width="15" height="15" aria-hidden="true">
          <circle cx="7" cy="7" r="4.5" fill="none" stroke="currentColor" stroke-width="1.6" />
          <path d="m10.5 10.5 3.5 3.5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
        </svg>
        <input
          id="q"
          ref="searchInput"
          v-model="filters.q"
          class="field"
          type="search"
          placeholder="Search names, paths, tags"
          autocomplete="off"
          enterkeyhint="search"
          @focus="view = 'library'"
          @keydown.down.prevent="moveFocus('ArrowRight')"
        />
        <kbd class="slash" aria-hidden="true">/</kbd>
      </div>

      <nav class="views" aria-label="Views">
        <button type="button" :aria-current="view === 'library' ? 'page' : undefined" @click="view = 'library'">Library</button>
        <button type="button" :aria-current="view === 'packs' ? 'page' : undefined" @click="view = 'packs'">
          Packs<span v-if="busyPacks.length" class="pulse" :title="`${busyPacks.length} working`" />
        </button>
      </nav>

      <div class="actions">
        <a
          v-if="stats?.update"
          class="btn ghost update"
          :href="stats.update.url"
          target="_blank"
          rel="noopener"
          :title="`Hoard ${stats.update.version} is out (this server runs ${stats.version}). Pull the new image or binary and restart; the database upgrades itself.`"
        >
          <span class="dot" aria-hidden="true" /><span class="wide-only">Update </span>{{ stats.update.version }}
        </a>
        <ActivityIndicator />
        <div class="theme" role="group" aria-label="Theme">
          <button
            v-for="t in THEMES"
            :key="t.value"
            type="button"
            :aria-pressed="prefs.theme === t.value"
            :title="t.label"
            @click="prefs.theme = t.value"
          >
            <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
              <template v-if="t.value === 'light'">
                <circle cx="8" cy="8" r="3" />
                <path d="M8 1.5v1.6M8 12.9v1.6M1.5 8h1.6M12.9 8h1.6M3.4 3.4l1.1 1.1M11.5 11.5l1.1 1.1M3.4 12.6l1.1-1.1M11.5 4.5l1.1-1.1" />
              </template>
              <template v-else-if="t.value === 'dark'">
                <path d="M13.2 10.1A5.6 5.6 0 0 1 5.9 2.8a5.6 5.6 0 1 0 7.3 7.3Z" />
              </template>
              <template v-else>
                <rect x="1.8" y="2.5" width="12.4" height="8.5" rx="1.2" />
                <path d="M5.5 14h5M8 11v3" />
              </template>
            </svg>
            <span class="visually-hidden">{{ t.label }}</span>
          </button>
        </div>
        <button v-if="stats?.auth_required" type="button" class="btn ghost" @click="showToken = !showToken">
          {{ getToken() ? 'Token set' : 'Set token' }}
        </button>
        <button type="button" class="btn import-folder" @click="folderInput?.click()">Import folder</button>
        <button type="button" class="btn primary" @click="fileInput?.click()">
          <span class="wide-only">Add files</span><span class="narrow-only" aria-hidden="true">Add</span>
        </button>
        <input ref="fileInput" type="file" multiple hidden @change="onPick" />
        <input ref="folderInput" type="file" webkitdirectory multiple hidden @change="onPickFolder" />
      </div>
    </header>

    <div v-if="pendingSplit" class="tokenbar splitbar" role="dialog" aria-label="How to import this folder">
      <span>
        <strong>{{ pendingSplit.group.name }}</strong> contains {{ pendingSplit.parts }} folders or archives.
      </span>
      <button type="button" class="btn primary" @click="resolveSplit(true)">Import each as its own pack ({{ pendingSplit.parts }})</button>
      <button type="button" class="btn" @click="resolveSplit(false)">Import as one pack</button>
      <button type="button" class="btn ghost" @click="pendingSplit = null">Cancel</button>
    </div>

    <form v-if="showToken" class="tokenbar" @submit.prevent="saveToken">
      <label for="token">This server needs a token to add or change assets.</label>
      <input id="token" v-model="tokenDraft" class="field mono" type="password" autocomplete="off" placeholder="HOARD_TOKEN" />
      <button class="btn primary" type="submit">Save token</button>
    </form>

    <template v-if="view === 'library'">
      <div v-if="showFilters" class="scrim" aria-hidden="true" @click="showFilters = false" />
      <aside id="filters" class="side" aria-label="Filters">
        <div class="side-head">
          <h2 class="side-title">Filters</h2>
          <button type="button" class="btn ghost" @click="showFilters = false">Done</button>
        </div>
        <section v-if="collections.length" class="collections">
          <h3>Collections</h3>
          <ul class="kinds">
            <li v-for="c in collections" :key="c.id">
              <button type="button" :aria-pressed="filters.collection?.id === c.id" @click="browseCollection(c)">
                <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true"><path d="M3 2.5h10v11.5l-5-3-5 3z" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" /></svg>
                <span class="cname">{{ c.name }}</span>
                <span class="n mono">{{ c.asset_count }}</span>
              </button>
            </li>
          </ul>
        </section>
        <p v-if="!kindFacets.length && !loading" class="side-empty">No filters for this search.</p>
        <section v-if="kindFacets.length">
          <h3>Kind</h3>
          <ul class="kinds">
            <li v-for="k in kindFacets" :key="k.value">
              <button type="button" :aria-pressed="filters.kind === k.value" :style="{ '--kind': `var(--k-${k.value})` }" @click="setKind(k.value)">
                <span class="dot" aria-hidden="true" />
                <span>{{ KIND_LABEL[k.value as AssetKind] ?? k.value }}</span>
                <span class="n mono">{{ k.count }}</span>
              </button>
            </li>
          </ul>
        </section>
        <section v-for="g in tagGroups" :key="g.ns">
          <h3>
            <button
              type="button"
              class="group-head"
              :aria-expanded="g.open"
              @click="prefs.collapsed = toggleIn(prefs.collapsed, g.ns)"
            >
              <svg viewBox="0 0 10 10" width="9" height="9" aria-hidden="true"><path d="M3 2l4 3-4 3" fill="none" stroke="currentColor" stroke-width="1.5" /></svg>
              {{ g.label }}
              <span v-if="!g.open" class="n mono">{{ g.all }}</span>
            </button>
          </h3>
          <div v-if="g.shown.length" class="facet-tags">
            <button
              v-for="f in g.shown"
              :key="f.value"
              type="button"
              class="facet"
              :aria-pressed="filters.tags.includes(f.value)"
              @click="toggleTag(f.value)"
            >
              {{ facetLabel(g.ns, f.value) }}<span class="n mono">{{ f.count }}</span>
            </button>
            <button v-if="g.hidden > 0" type="button" class="facet more-facets" @click="prefs.expanded = toggleIn(prefs.expanded, g.ns)">
              {{ g.hidden }} more
            </button>
            <button
              v-else-if="prefs.expanded.includes(g.ns) && g.all > FACET_PREVIEW"
              type="button"
              class="facet more-facets"
              @click="prefs.expanded = toggleIn(prefs.expanded, g.ns)"
            >
              fewer
            </button>
          </div>
        </section>
      </aside>

      <main class="main">
        <div class="bar">
          <button
            type="button"
            class="btn filters-toggle"
            aria-controls="filters"
            :aria-expanded="showFilters"
            @click="showFilters = !showFilters"
          >
            Filters<span v-if="filters.tags.length + (filters.kind ? 1 : 0)" class="badge">{{ filters.tags.length + (filters.kind ? 1 : 0) }}</span>
          </button>
          <div class="active">
            <span class="count" aria-live="polite">
              {{ firstLoad ? 'Searching…' : `${total.toLocaleString()} ${total === 1 ? 'asset' : 'assets'}` }}
            </span>
            <TagChip v-if="filters.pack" :tag="`pack:${filters.pack.name}`" removable @remove="filters.pack = null" />
            <TagChip v-if="filters.collection" :tag="`collection:${filters.collection.name}`" removable @remove="filters.collection = null" />
            <TagChip v-if="filters.kind" :tag="`kind:${filters.kind}`" removable @remove="filters.kind = ''" />
            <TagChip v-for="t in filters.tags" :key="t" :tag="t" removable @remove="toggleTag(t)" />
            <button v-if="hasFilters" type="button" class="linkbtn" @click="clearFilters">Clear all</button>
          </div>
          <label class="size" title="Tile size">
            <span class="visually-hidden">Tile size</span>
            <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><rect x="2" y="2" width="5" height="5" rx="1" fill="currentColor" /><rect x="9" y="2" width="5" height="5" rx="1" fill="currentColor" /><rect x="2" y="9" width="5" height="5" rx="1" fill="currentColor" /><rect x="9" y="9" width="5" height="5" rx="1" fill="currentColor" /></svg>
            <input v-model.number="prefs.tile" type="range" min="112" max="280" step="4" />
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><rect x="2" y="2" width="12" height="12" rx="1.5" fill="currentColor" /></svg>
          </label>
          <label class="sort">
            <span class="visually-hidden">Sort</span>
            <select v-model="filters.sort" class="field">
              <option value="">{{ filters.q ? 'Best match' : 'Newest' }}</option>
              <option value="name">Name</option>
              <option value="size">Largest</option>
              <option value="old">Oldest</option>
            </select>
          </label>
        </div>

        <p v-if="supportingHidden > 0 || prefs.showSupporting" class="supporting">
          <template v-if="!prefs.showSupporting">
            <span :title="'glTF buffers and material libraries. They come along when you download the model that uses them.'">
              {{ supportingHidden }} supporting {{ supportingHidden === 1 ? 'file' : 'files' }} hidden
            </span>
            <button type="button" class="linkbtn" @click="prefs.showSupporting = true">Show</button>
          </template>
          <template v-else>
            Including supporting files
            <button type="button" class="linkbtn" @click="prefs.showSupporting = false">Hide</button>
          </template>
        </p>

        <div v-if="!loading && stats && stats.assets === 0" class="welcome">
          <h1>Drop an asset pack anywhere on this page</h1>
          <p>
            Zips, tarballs, folders or loose files all work. Hoard unpacks them, stores every file once by its hash,
            reads sizes, lengths and poly counts, and tags everything so you can search for things like
            <span class="mono">grass footsteps</span> or <span class="mono">pbr stone</span>.
          </p>
          <div class="welcome-actions">
            <button type="button" class="btn primary" @click="folderInput?.click()">Import a folder</button>
            <button type="button" class="btn" @click="fileInput?.click()">Choose files</button>
          </div>
        </div>
        <div v-else-if="!loading && items.length === 0" class="none">
          <p>Nothing matches{{ filters.q ? ` “${filters.q.trim()}”` : '' }}{{ hasFilters && (filters.tags.length || filters.kind || filters.pack) ? ' with these filters' : '' }}.</p>
          <button v-if="hasFilters" type="button" class="btn" @click="clearFilters">Clear search and filters</button>
        </div>

        <div ref="gridEl" class="grid" :class="{ selecting: checked.length > 0 }">
          <template v-if="firstLoad">
            <div v-for="n in 18" :key="`s${n}`" class="skeleton" aria-hidden="true"><span /><i /><b /></div>
          </template>
          <AssetTile
            v-for="a in items"
            :key="a.id"
            :asset="a"
            :selected="selected?.id === a.id"
            :checked="checkedSet.has(a.id)"
            :selecting="checked.length > 0"
            @open="onTileOpen(a, $event)"
            @check="toggleCheck(a, $event)"
          />
        </div>
        <div v-if="items.length < total" ref="sentinel" class="more">
          <button type="button" class="btn ghost" :disabled="loading" @click="runSearch(true)">
            {{ loading ? 'Loading…' : `Show more (${(total - items.length).toLocaleString()} left)` }}
          </button>
        </div>
        <p v-if="items.length && !checked.length" class="hint">
          Arrow keys move, Enter opens, Space plays a sound, X or Ctrl-click selects several (long-press on touch).
        </p>
      </main>

      <AssetDetail
        v-if="selected"
        class="detail-pane"
        :asset="selected"
        :position="position"
        :collections="collections"
        @close="selected = null"
        @collect="(id, create) => addToCollection([selected!.id], create ? { create } : { id })"
        @uncollect="(id) => removeFromCollection([selected!.id], id)"
        @browse-collection="browseCollection"
        @updated="onUpdated"
        @tag="toggleTag"
        @error="showError"
        @open="openAsset"
        @step="step"
      />

      <div v-if="checked.length" class="selbar" role="region" aria-label="Selected assets">
        <strong>{{ checked.length }} selected</strong>
        <a class="btn primary" :href="selectionUrl" download>Download zip</a>
        <label v-if="checkedWithSource" class="srccheck" title="Unreal SoundWaves and textures whose original file Hoard recovered">
          <input v-model="preferSources" type="checkbox" /> Recovered WAV/PNG instead of .uasset ({{ checkedWithSource }})
        </label>
        <form class="bulktag" @submit.prevent="applyBulkTag">
          <label class="visually-hidden" for="bulk-tag">Tag to add to the selected assets</label>
          <input id="bulk-tag" v-model="bulkTag" class="field" placeholder="Add a tag to all" autocomplete="off" :disabled="bulkBusy" />
          <button class="btn" type="submit" :disabled="!bulkTag.trim() || bulkBusy">{{ bulkBusy ? 'Tagging…' : 'Tag' }}</button>
        </form>
        <CollectionPicker
          :collections="collections"
          up
          @add="(id) => addToCollection(checked, { id })"
          @create="(name) => addToCollection(checked, { create: name })"
        />
        <button v-if="filters.collection" type="button" class="btn" @click="removeFromCollection(checked, filters.collection.id)">
          Remove from {{ filters.collection.name }}
        </button>
        <button type="button" class="linkbtn" @click="checked = items.map((a) => a.id)">Select all {{ items.length }}</button>
        <button type="button" class="btn ghost" @click="checked = []">Clear</button>
      </div>
    </template>

    <PacksView
      v-else
      class="packs-pane"
      :packs="packs"
      :stats="stats"
      :collections="collections"
      @browse="browsePack"
      @browse-collection="browseCollection"
      @changed="refreshPacks"
      @error="showError"
    />

    <div v-if="dragDepth > 0" class="drop" aria-hidden="true">
      <div>
        <strong>Drop to add to the hoard</strong>
        <span>Each top-level file or folder becomes its own pack. Gigabytes are fine; big files resume if the connection drops.</span>
      </div>
    </div>

    <UploadQueue ref="queue" @created="onCreated" @unauthorized="showToken = true" />
    <div v-if="toast" class="toast" :class="toast.kind" role="status"><span>{{ toast.text }}</span></div>
  </div>
</template>

<style scoped>
.app {
  display: grid;
  grid-template-columns: 240px minmax(0, 1fr);
  grid-template-rows: auto auto 1fr;
  grid-template-areas: 'top top' 'token token' 'side main';
  height: 100vh;
  height: 100dvh;
}
.app.with-detail { grid-template-columns: 240px minmax(0, 1fr) minmax(340px, 420px); grid-template-areas: 'top top top' 'token token token' 'side main detail'; }
.packs-pane { grid-row: 3; grid-column: 1 / -1; overflow-y: auto; }

.top {
  grid-area: top;
  display: flex; align-items: center; gap: 1rem;
  padding: 0.6rem 1rem; border-bottom: 1px solid var(--line); background: var(--panel);
}
.brand { display: flex; align-items: center; gap: 0.5rem; font-weight: 750; font-size: var(--step-1); font-variation-settings: 'CASL' 1; }
.search { position: relative; flex: 1; max-width: 640px; }
.search .field { background: var(--bg); padding: 0.55em 2.2em 0.55em 2.1em; }
.search-icon { position: absolute; left: 0.7em; top: 50%; translate: 0 -50%; color: var(--faint); pointer-events: none; }
.slash {
  position: absolute; right: 0.55em; top: 50%; translate: 0 -50%; pointer-events: none;
  padding: 0 0.4em; border: 1px solid var(--line); border-radius: 3px; color: var(--faint); font: inherit; font-size: 0.75rem;
}
.search:focus-within .slash { display: none; }
.views { display: flex; gap: 0.15rem; }
.theme { display: inline-flex; padding: 2px; gap: 1px; border: 1px solid var(--line); border-radius: var(--r-chip); }
.theme button {
  display: grid; place-items: center; width: 1.8rem; height: 1.6rem; padding: 0;
  border: 0; border-radius: 3px; background: transparent; color: var(--faint); cursor: pointer;
}
.theme button:hover { color: var(--text); }
.theme button[aria-pressed='true'] { background: var(--raised); color: var(--text); }
.views button {
  position: relative; border: 0; background: none; padding: 0.4em 0.75em; cursor: pointer; border-radius: var(--r-chip);
  color: var(--muted); font-size: var(--step--1); font-weight: 550;
}
.views button:hover { color: var(--text); }
.views button[aria-current='page'] { color: var(--text); background: var(--raised); }
.pulse { position: absolute; top: 0.35em; right: 0.3em; width: 6px; height: 6px; border-radius: 50%; background: var(--k-model); animation: blink 1.2s infinite; }
@keyframes blink { 50% { opacity: 0.25; } }
.actions { display: flex; gap: 0.5rem; margin-left: auto; }
.update { color: var(--ok); text-decoration: none; white-space: nowrap; }
.update .dot { width: 0.5em; height: 0.5em; border-radius: 50%; background: currentColor; }
.narrow-only { display: none; }

.tokenbar { grid-area: token; display: flex; gap: 0.75rem; align-items: center; padding: 0.6rem 1rem; background: var(--raised); font-size: var(--step--1); }
.tokenbar .field { max-width: 320px; }

.side { grid-area: side; overflow-y: auto; padding: 1rem 0.9rem 2rem; border-right: 1px solid var(--line); }
.side-head { display: none; }
.side-empty { margin: 0; font-size: var(--step--1); color: var(--faint); }
.collections svg { flex: none; color: var(--muted); }
.cname { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.side section + section { margin-top: 1rem; }
.side h3 { margin: 0 0 0.4rem; font-size: 0.75rem; font-weight: 650; color: var(--faint); }
.group-head {
  display: flex; align-items: center; gap: 0.35rem; width: 100%; padding: 0; border: 0; background: none; cursor: pointer;
  font: inherit; color: inherit; text-align: left;
}
.group-head:hover { color: var(--muted); }
.group-head svg { transition: rotate 0.12s; }
.group-head[aria-expanded='true'] svg { rotate: 90deg; }
.kinds { list-style: none; margin: 0; padding: 0; }
.kinds button {
  display: flex; width: 100%; align-items: center; gap: 0.55rem; padding: 0.3rem 0.45rem;
  border: 0; border-radius: var(--r-chip); background: none; cursor: pointer; font-size: var(--step--1); text-align: left;
}
.kinds button:hover { background: var(--raised); }
.kinds button[aria-pressed='true'] { background: var(--raised); box-shadow: inset 3px 0 0 var(--kind); }
.dot { width: 10px; height: 10px; border-radius: 2px; background: var(--kind); flex: none; }
.n { margin-left: auto; color: var(--faint); font-size: 0.75rem; padding-left: 0.4rem; }
.facet-tags { display: flex; flex-wrap: wrap; gap: 0.3rem; }
.facet {
  display: inline-flex; align-items: baseline; padding: 0.1em 0.45em; border: 1px solid var(--line); border-radius: var(--r-chip);
  background: none; cursor: pointer; font-size: 0.78rem;
}
.facet:hover { border-color: var(--muted); }
.facet[aria-pressed='true'] { background: var(--text); color: var(--bg); border-color: var(--text); }
.facet[aria-pressed='true'] .n { color: var(--bg); opacity: 0.6; }
.more-facets { border-style: dashed; color: var(--muted); }

.main { grid-area: main; overflow-y: auto; padding: 0 1.25rem 4rem; }
.app:has(.selbar) .main { padding-bottom: 7rem; }
.bar {
  position: sticky; top: 0; z-index: 3; display: flex; align-items: center; gap: 0.9rem;
  padding: 0.75rem 0; background: var(--bg);
}
.active { display: flex; flex-wrap: wrap; gap: 0.35rem; align-items: center; flex: 1; min-width: 0; }
.count { font-size: var(--step--1); color: var(--muted); margin-right: 0.35rem; }
.linkbtn {
  border: 0; background: none; padding: 0.1em 0.2em; cursor: pointer; font-size: var(--step--1);
  color: var(--text); text-decoration: underline; text-decoration-color: var(--faint); text-underline-offset: 3px;
}
.linkbtn:hover { text-decoration-color: var(--text); }
.size { display: flex; align-items: center; gap: 0.4rem; color: var(--faint); }
.size input { width: 84px; accent-color: var(--muted); }
.sort .field { width: auto; padding: 0.3em 0.5em; font-size: var(--step--1); }
.filters-toggle { display: none; }
.badge {
  min-width: 1.2rem; padding: 0 0.3rem; border-radius: 999px; background: var(--text); color: var(--bg);
  font-size: 0.7rem; font-weight: 700; text-align: center;
}
.supporting { margin: -0.25rem 0 0.9rem; font-size: 0.78rem; color: var(--faint); }
.supporting .linkbtn { font-size: inherit; color: var(--muted); }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(var(--tile), 1fr)); gap: 1.1rem 0.9rem; }
.more { display: flex; justify-content: center; margin-top: 1.5rem; }
.none { color: var(--muted); padding: 2rem 0; }
.none p { margin: 0 0 0.9rem; }
.hint { margin: 2.5rem 0 0; text-align: center; font-size: 0.75rem; color: var(--faint); }
.welcome { max-width: 60ch; padding: 3rem 0; }
.welcome h1 { margin: 0 0 0.75rem; font-size: var(--step-2); line-height: 1.2; font-variation-settings: 'CASL' 0.6; font-weight: 700; }
.welcome p { color: var(--muted); margin: 0 0 1.25rem; }
.welcome-actions { display: flex; gap: 0.5rem; }

/* Placeholder tiles while the first page loads. */
.skeleton { display: flex; flex-direction: column; gap: 0.4rem; }
.skeleton span { aspect-ratio: 1; border-radius: var(--r-tile); background: var(--panel); border: 1px solid var(--line); }
.skeleton i, .skeleton b { height: 0.7rem; border-radius: 3px; background: var(--panel); }
.skeleton i { width: 70%; margin-top: 0.25rem; }
.skeleton b { width: 45%; }
.skeleton > * { animation: shimmer 1.4s ease-in-out infinite; }
@keyframes shimmer { 50% { opacity: 0.45; } }

.detail-pane { grid-area: detail; min-height: 0; }

.selbar {
  position: fixed; left: 50%; bottom: 1rem; translate: -50% 0; z-index: 25;
  display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; max-width: calc(100vw - 2rem);
  padding: 0.6rem 0.75rem 0.6rem 1rem; border: 1px solid var(--muted); border-radius: var(--r-tile);
  background: var(--raised); box-shadow: 0 10px 30px var(--shadow); font-size: var(--step--1);
}
.bulktag { display: flex; gap: 0.35rem; }
.srccheck { display: inline-flex; gap: 0.35rem; align-items: center; color: var(--muted); }
.bulktag .field { width: 11rem; padding: 0.35em 0.6em; }

.drop {
  position: fixed; inset: 0; z-index: 20; display: grid; place-items: center;
  background: color-mix(in srgb, var(--bg) 85%, transparent); pointer-events: none;
}
.drop div {
  display: flex; flex-direction: column; gap: 0.4rem; align-items: center; text-align: center;
  padding: 3rem 4rem; margin: 1rem; border: 2px dashed var(--k-texture); border-radius: 12px;
}
.drop strong { font-size: var(--step-2); font-variation-settings: 'CASL' 0.6; }
.drop span { color: var(--muted); }

.toast {
  position: fixed; left: 50%; top: 4.2rem; translate: -50% 0; z-index: 31; max-width: min(560px, calc(100vw - 2rem));
  padding: 0.6rem 0.9rem; background: var(--raised); border: 1px solid var(--line); border-radius: var(--r-tile);
  font-size: var(--step--1); box-shadow: 0 8px 24px var(--shadow);
}
.toast.error { border-color: var(--danger); color: var(--danger); }
.toast.ok { border-color: var(--ok); color: var(--ok); }
.splitbar { flex-wrap: wrap; }
.scrim { display: none; }

@media (max-width: 1100px) {
  .app.with-detail { grid-template-columns: 220px minmax(0, 1fr) minmax(320px, 380px); }
  .size { display: none; }
}

/* Narrow screens: one column. Filters slide in from the left, the detail covers the grid. */
@media (max-width: 900px) {
  .app, .app.with-detail { grid-template-columns: minmax(0, 1fr); grid-template-areas: 'top' 'token' 'main'; }
  .top { flex-wrap: wrap; gap: 0.6rem; padding: 0.55rem 0.75rem; }
  .search { order: 3; flex-basis: 100%; max-width: none; }
  .slash { display: none; }
  .brand span { display: inline; }
  .views { margin-left: auto; }
  .actions { margin-left: 0; }
  .import-folder { display: none; }
  .side {
    position: fixed; inset: 0 auto 0 0; z-index: 12; width: min(320px, 86vw);
    background: var(--panel); transform: translateX(-102%); transition: transform 0.18s ease-out;
    border-right: 1px solid var(--line); padding-top: 0.75rem;
  }
  .app.filters-open .side { transform: none; box-shadow: 10px 0 30px var(--shadow); }
  .side-head { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.75rem; }
  .side-title { margin: 0; font-size: var(--step-1); color: var(--text); font-weight: 650; }
  .scrim { display: block; position: fixed; inset: 0; z-index: 11; background: var(--scrim); }
  .filters-toggle { display: inline-flex; }
  .main { padding: 0 0.75rem 6rem; }
  .bar { flex-wrap: wrap; gap: 0.5rem; }
  .active { order: 3; flex-basis: 100%; }
  .sort { margin-left: auto; }
  .grid { grid-template-columns: repeat(auto-fill, minmax(min(var(--tile), 42vw), 1fr)); gap: 0.9rem 0.7rem; }
  .detail-pane { position: fixed; inset: 0; z-index: 15; }
  .hint { display: none; }
  .selbar { left: 0.5rem; right: 0.5rem; bottom: 0.5rem; translate: none; max-width: none; gap: 0.5rem; justify-content: space-between; }
  .bulktag { order: 3; flex-basis: 100%; }
  .bulktag .field { flex: 1; width: auto; }
}
@media (max-width: 480px) {
  .wide-only { display: none; }
  .narrow-only { display: inline; }
  .brand span { display: none; }
}
</style>
