<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import {
  api, downloadCredits, filesFromDrop, KIND_LABEL, selectionZipUrl,
  type Asset, type AssetKind, type BulkEdit, type Collection, type Facet, type Pack, type Stats, type UploadFile,
} from './api'
import { canPreview, stop as stopPreview, toggle as togglePreview } from './player'
import { prefs, toggleIn } from './prefs'
import { askToSignIn, auth, may, mayEdit, signOut } from './auth'
import { teach } from './storeFormats'
import { DARK_THEMES, LIGHT_THEMES, themeFor } from './theme'
import ActivityIndicator from './components/ActivityIndicator.vue'
import { looksLikeFolderOfPacks, splitPerSubfolder } from './uploader'
import UploadQueue from './components/UploadQueue.vue'
import AssetDetail from './components/AssetDetail.vue'
import AssetTile from './components/AssetTile.vue'
import CollectionPicker from './components/CollectionPicker.vue'
import PacksView from './components/PacksView.vue'
import TagChip from './components/TagChip.vue'
import UiLayer from './components/UiLayer.vue'
import CommandPalette from './components/CommandPalette.vue'
import InsecureNotice from './components/InsecureNotice.vue'
import { archiveWithUndo, countText } from './archive'
import { showError, showOk, swipeToClose, useTween, withTransition, type Command } from './ui'
import { useVirtualGrid } from './virtualGrid'
import { lazy } from './lazy'

// Opened now and then, so they load when first shown.
const SettingsPage = lazy(() => import('./components/SettingsPage.vue'), 'settings')
const ArchiveView = lazy(() => import('./components/ArchiveView.vue'), 'the archive')
const ShortcutsDialog = lazy(() => import('./components/ShortcutsDialog.vue'), 'the shortcuts', 'overlay')
const SetupWizard = lazy(() => import('./components/SetupWizard.vue'), 'setup', 'overlay')

type View = 'library' | 'packs' | 'settings' | 'archive'
const PAGE = 60
const THEMES = [
  { value: 'system', label: 'Match system theme' },
  { value: 'light', label: 'Light theme' },
  { value: 'dark', label: 'Dark theme' },
] as const
/** Values shown per filter group before "more". */
const FACET_PREVIEW = 8

/** The view in the address: #packs, #archive, #settings/<page>; anything else is the library. */
function viewFromHash(): { view: View; section: string } {
  const h = location.hash
  if (h === '#packs') return { view: 'packs', section: '' }
  if (h === '#archive') return { view: 'archive', section: '' }
  const m = h.match(/^#settings(?:\/([\w-]+))?$/)
  if (m) return { view: 'settings', section: m[1] ?? 'appearance' }
  return { view: 'library', section: '' }
}
const initial = viewFromHash()
const view = ref<View>(initial.view)
/** The open page of Settings. */
const settingsSection = ref(initial.section || 'appearance')
type Named = { id: string; name: string }
const filters = reactive({
  q: '', kind: '', tags: [] as string[], pack: null as Named | null, collection: null as Named | null, sort: '',
  /** Assets that look like this one, most alike first. */
  similar: null as Named | null,
  /** Assets with much of this colour (hex), most of it first. */
  color: '',
})
/** Colours to search by in one click; the picker beside them does any other. */
const COLOR_PRESETS: { hex: string; name: string }[] = [
  { hex: '#b8322a', name: 'Red' }, { hex: '#e07b28', name: 'Orange' }, { hex: '#e8c234', name: 'Yellow' },
  { hex: '#4f8a2e', name: 'Green' }, { hex: '#2a9d8f', name: 'Teal' }, { hex: '#2f6fd0', name: 'Blue' },
  { hex: '#7b4bb3', name: 'Purple' }, { hex: '#d9599a', name: 'Pink' }, { hex: '#7a5230', name: 'Brown' },
  { hex: '#d6c09a', name: 'Sand' }, { hex: '#8a8a8a', name: 'Grey' }, { hex: '#4a5560', name: 'Slate' },
  { hex: '#1c1c1c', name: 'Black' }, { hex: '#f2f2f0', name: 'White' },
]

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
const showFilters = ref(false)
// A new server with nowhere to keep files opens the setup first; it stays up until it's finished.
const setupOpen = ref(false)
watch(() => stats.value?.setup_needed, (needed) => { if (needed) setupOpen.value = true })
/** Signed in, rather than browsing as a guest: Settings and the setup are only for accounts. */
const member = computed(() => !!auth.status?.signed_in)
/** May change the library (a guest is asked to sign in when they try). */
const editor = computed(() => mayEdit())
const admin = computed(() => may('admin'))
watch([member, view], ([m, v]) => { if (!m && v === 'settings') view.value = 'library' }, { immediate: true })
function setupDone() {
  setupOpen.value = false
  refreshPacks()
}
const searchInput = ref<HTMLInputElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)
const gridEl = ref<HTMLElement | null>(null)
/** The filters column, a drawer on phones that can be swiped shut. */
const sideEl = ref<HTMLElement | null>(null)
let unswipeSide = () => {}
const sentinel = ref<HTMLElement | null>(null)

type Group = { name: string; files: UploadFile[] }
const queue = ref<InstanceType<typeof UploadQueue> | null>(null)
const folderInput = ref<HTMLInputElement | null>(null)
/** A dropped/picked folder that contains several packs: ask how to import it. */
const pendingSplit = ref<{ group: Group; parts: number } | null>(null)
const dragDepth = ref(0)
/** How many files or folders are being dragged over the page (0 when the browser won't say). */
const dragCount = ref(0)

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
      similar: filters.similar?.id,
      color: filters.color || undefined,
      limit: PAGE,
      offset: append ? items.value.length : 0,
      hide_supporting: !prefs.showSupporting,
    })
    if (seq !== searchSeq) return
    // Only a new page's tiles rise in; tiles scrolled back into view just appear.
    fresh.value = append ? items.value.length : 0
    items.value = append ? [...items.value, ...res.items] : res.items
    total.value = res.total
    supportingHidden.value = res.supporting_hidden ?? 0
    if (!append) {
      kindFacets.value = res.kinds
      tagFacets.value = res.tags
    }
  } catch (e) {
    showError(`Search failed: ${(e as Error).message}`, { label: 'Retry', run: () => runSearch(append) })
  } finally {
    if (seq === searchSeq) {
      loading.value = false
      // Observing again reports where the end is now: still in range (a short page, a tall window) loads another.
      await nextTick()
      if (sentinel.value && observer) {
        observer.unobserve(sentinel.value)
        observer.observe(sentinel.value)
      }
    }
  }
}

let debounce: ReturnType<typeof setTimeout> | undefined
watch(() => filters.q, () => {
  clearTimeout(debounce)
  debounce = setTimeout(() => runSearch(), 200)
})
watch(() => [
  filters.kind, filters.tags.join(','), filters.pack?.id, filters.collection?.id, filters.sort, filters.similar?.id,
  filters.color, prefs.showSupporting,
], () => {
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
    showError(`Couldn't reach the server: ${(e as Error).message}`, { label: 'Retry', run: refreshPacks })
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
  }, { root: mainEl.value, rootMargin: '1500px 0px' })
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
/** Switch views (library, packs, archive, settings) with a short cross-fade. */
function setView(v: View) {
  if (view.value === v) return
  void withTransition(() => (view.value = v), 'view')
}
function browsePack(p: Named) {
  Object.assign(filters, { pack: { id: p.id, name: p.name }, collection: null, kind: '', tags: [], q: '', similar: null, color: '' })
  setView('library')
}
function browseCollection(c: Collection | Named) {
  const same = filters.collection?.id === c.id
  Object.assign(filters, {
    collection: same ? null : { id: c.id, name: c.name }, pack: null, kind: '', tags: [], q: '', similar: null, color: '',
  })
  setView('library')
  showFilters.value = false
}
const mainEl = ref<HTMLElement | null>(null)
/** Index of the first tile of the newest page, the ones that animate in. */
const fresh = ref(0)
const win = useVirtualGrid({ scroller: mainEl, grid: gridEl, count: () => items.value.length })

// Opening the detail panel leaves the tiles the size they were: the grid keeps its column width and drops a column,
// and the panel takes what's left over (so it's a little wider or narrower depending on the window), instead of the
// remaining columns stretching to fill the space.
const appEl = ref<HTMLElement | null>(null)
/** The room the grid and panel share, what the grid's padding and scrollbar take of it, and the column gap. */
const room = ref({ width: 0, side: 0, chrome: 0, gap: 0 })
const DETAIL_MIN = 340
const DETAIL_MAX = 640
const narrow = window.matchMedia('(max-width: 900px)')
function measureRoom() {
  const app = appEl.value
  const side = sideEl.value
  const main = mainEl.value
  if (!app || !side || !main || narrow.matches) {
    room.value = { width: 0, side: 0, chrome: 0, gap: 0 }
    return
  }
  const ms = getComputedStyle(main)
  const chrome = parseFloat(ms.paddingLeft) + parseFloat(ms.paddingRight) + (main.offsetWidth - main.clientWidth)
  const gap = gridEl.value ? parseFloat(getComputedStyle(gridEl.value).columnGap) || 0 : room.value.gap
  room.value = { width: app.clientWidth, side: side.offsetWidth, chrome, gap }
}
const roomObserver = new ResizeObserver(measureRoom)
watch([appEl, sideEl], ([a, s]) => {
  roomObserver.disconnect()
  if (a) roomObserver.observe(a)
  if (s) roomObserver.observe(s)
})
watch(gridEl, () => nextTick(measureRoom))
narrow.addEventListener('change', measureRoom)
onBeforeUnmount(() => {
  roomObserver.disconnect()
  narrow.removeEventListener('change', measureRoom)
})
/** The panel's width while its edge is being dragged. */
const detailDrag = ref<number | null>(null)
/** With the panel open: the columns to show, their width, and the panel's width. The tiles keep their size; the
 *  grid drops columns to make room. */
const fit = computed(() => {
  const { width, side, chrome, gap } = room.value
  if (!width || !gap) return null
  const space = width - side - chrome
  // The columns the grid has without the panel, as auto-fill makes them.
  const n = Math.max(1, Math.floor((space + gap) / (prefs.tile + gap)))
  const col = (space - gap * (n - 1)) / n
  const used = (cols: number) => cols * col + (cols - 1) * gap
  const want = detailDrag.value ?? prefs.detailWidth
  if (want === null) {
    // Left alone, the panel takes what's over once enough columns go for it to fit.
    const cols = Math.floor((space - DETAIL_MIN + gap) / (col + gap))
    if (cols < 1) return null
    return { col, cols, detail: Math.min(DETAIL_MAX, Math.floor(space - used(cols))) }
  }
  // Dragged: as wide as asked, leaving the grid at least one column.
  const detail = Math.min(Math.max(want, DETAIL_MIN), space - col)
  if (detail < DETAIL_MIN) return null
  let cols = Math.max(1, Math.floor((space - detail + gap) / (col + gap)))
  // Mid-drag the edge follows the pointer; let go, and it settles on the nearest column so no gap is left over.
  if (detailDrag.value !== null) return { col, cols, detail }
  if (space - used(cols + 1) >= DETAIL_MIN && space - used(cols + 1) > detail - (col + gap) / 2) cols++
  return { col, cols, detail: Math.floor(space - used(cols)) }
})

/** Dragging the panel's left edge sets its width; double-clicking it goes back to the width that fits. */
function startResize(e: PointerEvent) {
  const panel = detailScroller()
  if (!panel || e.button !== 0) return
  e.preventDefault()
  const handle = e.currentTarget as HTMLElement
  handle.setPointerCapture(e.pointerId)
  const x0 = e.clientX
  const w0 = panel.offsetWidth
  detailDrag.value = w0
  document.documentElement.classList.add('resizing')
  const move = (m: PointerEvent) => (detailDrag.value = w0 + (x0 - m.clientX))
  const up = () => {
    handle.removeEventListener('pointermove', move)
    handle.removeEventListener('pointerup', up)
    handle.removeEventListener('pointercancel', up)
    document.documentElement.classList.remove('resizing')
    if (detailDrag.value !== null && Math.abs(detailDrag.value - w0) > 2) prefs.detailWidth = Math.round(fit.value?.detail ?? detailDrag.value)
    detailDrag.value = null
  }
  handle.addEventListener('pointermove', move)
  handle.addEventListener('pointerup', up)
  handle.addEventListener('pointercancel', up)
}
function resizeBy(e: KeyboardEvent) {
  const delta = e.key === 'ArrowLeft' ? 1 : e.key === 'ArrowRight' ? -1 : 0
  if (e.key === 'Home') prefs.detailWidth = null
  if (!delta) return
  e.preventDefault()
  const f = fit.value
  if (!f) return
  // A column at a time, so each press shows a change.
  prefs.detailWidth = Math.round(f.detail + delta * (f.col + room.value.gap))
}
const appStyle = computed(() => {
  const s: Record<string, string> = { '--tile': `${prefs.tile}px` }
  const f = fit.value
  if (f) {
    s['--side-w'] = `${room.value.side}px`
    s['--detail-w'] = `${f.detail}px`
    s['--fit-cols'] = String(f.cols)
    s['--fit-col'] = `${f.col}px`
  }
  return s
})
const windowed = computed(() =>
  items.value.slice(win.start.value, win.end.value).map((a, n) => ({ a, i: win.start.value + n })),
)
/** The logo: back to the whole library, nothing filtered or open, at the top. */
function goHome() {
  clearFilters()
  selected.value = null
  setView('library')
  mainEl.value?.scrollTo({ top: 0, behavior: 'smooth' })
}
function clearFilters() {
  Object.assign(filters, { q: '', kind: '', tags: [], pack: null, collection: null, similar: null, color: '' })
}
const hasFilters = computed(
  () => !!(filters.q || filters.kind || filters.tags.length || filters.pack || filters.collection || filters.similar || filters.color),
)
/** "More like this": everything that looks like an asset, across every pack, most alike first. */
function findSimilar(a: Asset) {
  Object.assign(filters, {
    similar: { id: a.id, name: a.name }, color: '', q: '', kind: '', tags: [], pack: null, collection: null, sort: '',
  })
  setView('library')
  mainEl.value?.scrollTo({ top: 0 })
}
/** A colour from an asset's palette: everything with much of it, across the library. */
function findColor(hex: string) {
  Object.assign(filters, { color: hex, similar: null, q: '', kind: '', tags: [], pack: null, collection: null, sort: '' })
  setView('library')
  mainEl.value?.scrollTo({ top: 0 })
}
/** A colour from the sidebar: added to the filters already set, or taken off when it's the one set. */
function pickColor(hex: string) {
  filters.color = filters.color === hex ? '' : hex
}
const rankLabel = computed(() =>
  filters.similar ? 'Most alike' : filters.color ? 'Most of the colour' : filters.q ? 'Best match' : 'Newest',
)

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
    showOk(`Added ${ids.length} ${ids.length === 1 ? 'asset' : 'assets'} to ${c.name}`, {
      label: 'Undo',
      run: () => removeFromCollection(ids, id, true),
    })
    if (selected.value && ids.includes(selected.value.id)) openAsset(selected.value.id)
  } catch (e) {
    showError(`Couldn't add to the collection: ${(e as Error).message}`)
  }
}

async function removeFromCollection(ids: string[], id: string, quiet = false) {
  try {
    const c = await api.collectionMembers(id, { remove: ids })
    await refreshCollections()
    if (!quiet) showOk(`Removed ${ids.length} from ${c.name}`, { label: 'Undo', run: () => addToCollection(ids, { id }) })
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
  if (e.metaKey || e.ctrlKey || e.shiftKey || selecting.value) {
    toggleCheck(a, e)
    return
  }
  openAsset(a)
}

const bulkTag = ref('')
const bulkCategory = ref('')
const bulkBusy = ref(false)
/** Selecting without anything picked yet (the Select button); tiles then check instead of opening. */
const selectMode = ref(false)
const selecting = computed(() => selectMode.value || checked.value.length > 0)
watch(view, () => (selectMode.value = false))

/** Show a bulk edit on the tiles that are loaded (and the open asset) without searching again. */
function patchLoaded(ids: string[], edit: Omit<BulkEdit, 'ids'>) {
  const set = new Set(ids)
  const patch = (a: Asset): Asset => {
    if (!set.has(a.id)) return a
    let tags = a.tags
    if (edit.remove_tags?.length) tags = tags.filter((t) => !edit.remove_tags!.includes(t.tag))
    for (const tag of edit.add_tags ?? []) {
      if (!tags.some((t) => t.tag === tag)) tags = [...tags, { tag, source: 'human', confidence: 1 }]
    }
    const category = edit.category === undefined ? a.category : edit.category.trim() || null
    return { ...a, tags, category }
  }
  items.value = items.value.map(patch)
  if (selected.value && set.has(selected.value.id)) selected.value = patch(selected.value)
}
/** One edit to many assets in one go. False if it failed. */
async function bulkEdit(ids: string[], edit: Omit<BulkEdit, 'ids'>): Promise<boolean> {
  if (!ids.length) return true
  bulkBusy.value = true
  try {
    await api.bulkAssets({ ids, ...edit })
  } catch (e) {
    showError(`Couldn't change ${ids.length === 1 ? 'that asset' : `those ${ids.length} assets`}: ${(e as Error).message}`)
    return false
  } finally {
    bulkBusy.value = false
  }
  patchLoaded(ids, edit)
  return true
}
const plural = (n: number) => `${n.toLocaleString()} ${n === 1 ? 'asset' : 'assets'}`
async function bulkTagEdit(remove: boolean) {
  const tag = bulkTag.value.trim().toLowerCase()
  const ids = [...checked.value]
  if (!tag || !ids.length) return
  // Undo only touches the assets this changed.
  const has = (id: string) => !!items.value.find((a) => a.id === id)?.tags.some((t) => t.tag === tag)
  const touched = ids.filter((id) => has(id) === remove)
  if (!(await bulkEdit(ids, remove ? { remove_tags: [tag] } : { add_tags: [tag] }))) return
  bulkTag.value = ''
  const undo = remove ? { add_tags: [tag] } : { remove_tags: [tag] }
  showOk(remove ? `Took ${tag} off ${plural(touched.length)}` : `Tagged ${plural(ids.length)} with ${tag}`, {
    label: 'Undo',
    run: () => void bulkEdit(touched, undo),
  })
}
async function bulkSetCategory() {
  const category = bulkCategory.value.trim()
  const ids = [...checked.value]
  if (!ids.length) return
  // Remember each one's old category so Undo can put them back.
  const before = new Map<string, string[]>()
  for (const id of ids) {
    const c = items.value.find((a) => a.id === id)?.category ?? ''
    before.set(c, [...(before.get(c) ?? []), id])
  }
  if (!(await bulkEdit(ids, { category }))) return
  bulkCategory.value = ''
  showOk(category ? `Set the category of ${plural(ids.length)} to ${category}` : `Cleared the category of ${plural(ids.length)}`, {
    label: 'Undo',
    run: () => {
      for (const [c, group] of before) void bulkEdit(group, { category: c })
    },
  })
}
/** Archive what's selected: gone from the grid at once, with Undo on the toast. */
/** CREDITS.md for the selected assets' packs (and the packs of what they need, as the zip has them). */
async function creditsForChecked() {
  try {
    await downloadCredits({ selections: [{ ids: [...checked.value], include_deps: true }] })
  } catch (e) {
    showError(`Couldn't make the credits: ${(e as Error).message}`)
  }
}
async function archiveChecked() {
  const ids = [...checked.value]
  if (!ids.length) return
  const ok = await archiveWithUndo(`Archived ${plural(ids.length)}`, { assets: ids }, onArchiveChanged)
  if (!ok) return
  checked.value = []
  selectMode.value = false
  if (selected.value && ids.includes(selected.value.id)) selected.value = null
}
/** Archive the open asset and close it. */
async function archiveSelected() {
  const a = selected.value
  if (!a) return
  const ok = await archiveWithUndo(`Archived ${a.name}`, { assets: [a.id] }, onArchiveChanged)
  if (ok && selected.value?.id === a.id) closeDetail()
}
/** Something went into or came out of the archive: the grid, counts and packs change. */
function onArchiveChanged() {
  runSearch()
  refreshPacks()
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
  if (!editor.value) return
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
  if (!editor.value || !e.dataTransfer?.types.includes('Files')) return
  if (!dragDepth.value) dragCount.value = e.dataTransfer.items?.length ?? 0
  dragDepth.value++
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

/** The asset the detail panel is opening or showing; cleared on close so a late fetch can't reopen it. */
let opening = ''
/** Show an asset in the detail panel. The panel slides in when it wasn't open; moving between assets just swaps. */
function showDetail(a: Asset) {
  if (selected.value || view.value !== 'library') selected.value = a
  else void withTransition(() => (selected.value = a), 'detail')
}
/** Open an asset in the detail panel, upgrading the search hit to the full record (links, derivatives). */
async function openAsset(idOrAsset: string | Asset) {
  const id = typeof idOrAsset === 'string' ? idOrAsset : idOrAsset.id
  opening = id
  if (typeof idOrAsset !== 'string') showDetail(idOrAsset)
  view.value = 'library'
  try {
    const full = await api.asset(id)
    if (opening === id) {
      if (selected.value) selected.value = full
      else showDetail(full)
    }
  } catch (e) {
    showError(`Couldn't open that asset: ${(e as Error).message}`)
  }
}
/** Close the detail panel and put focus back on the tile it was showing, so the keyboard picks up where it was. */
function closeDetail() {
  const id = selected.value?.id
  opening = ''
  if (!id) return
  const active = document.activeElement
  const refocus = !active || active === document.body || !!active.closest('.detail-pane')
  void withTransition(() => (selected.value = null), 'detail')
  if (refocus) focusTile(id)
}

/** Set when a link in the panel opens another asset: that's somewhere new, so it gets its own history entry. */
let following = ''
/** Open an asset from a link in the detail panel (what it uses, what uses it, duplicates). Back returns to the one
 *  before, scrolled where it was; the new one starts at the top. */
function followAsset(id: string) {
  if (id === selected.value?.id) return
  following = id
  scrollOnOpen = 0
  void openAsset(id)
}

// The panel's scroll position, per history entry, so Back and Forward put it where it was.
const scrolls = new Map<string, number>()
let entries = 0
const entryKey = () => `${Date.now().toString(36)}-${++entries}`
const currentKey = (): string => (history.state as { key?: string } | null)?.key ?? ''
const detailScroller = () => document.querySelector<HTMLElement>('.detail-pane')
function onAnyScroll(e: Event) {
  const el = e.target
  if (el instanceof HTMLElement && el.classList.contains('detail-pane') && currentKey()) scrolls.set(currentKey(), el.scrollTop)
}
/** Where the panel should scroll once the asset being opened shows: the top for a followed link, where it was for
 *  Back and Forward. */
let scrollOnOpen: number | null = null
watch(
  () => selected.value?.id,
  async () => {
    const top = scrollOnOpen
    scrollOnOpen = null
    if (top === null) return
    await nextTick()
    scrollPanel(top)
  },
)
/** Parts of the panel load after it opens (previews, links), so a deep position may not exist yet: keep trying for a
 *  moment, unless someone scrolls first. */
function scrollPanel(top: number) {
  let frames = 0
  const tryIt = () => {
    const el = detailScroller()
    if (!el) return
    el.scrollTop = top
    if (Math.abs(el.scrollTop - top) > 1 && ++frames < 40) requestAnimationFrame(tryIt)
  }
  tryIt()
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

// ------------------------------------------------------------------ keyboard

function focusTile(id: string) {
  const i = items.value.findIndex((a) => a.id === id)
  if (i < 0) return
  // Its row may be scrolled out of the page: bring it in first, then focus it once it's rendered.
  win.reveal(i)
  nextTick(() => gridEl.value?.querySelector<HTMLButtonElement>(`.hit[data-id="${id}"]`)?.focus({ preventScroll: true }))
}

/** Move focus through the grid like a file browser; with the detail open, it follows. */
function moveFocus(key: string) {
  const count = items.value.length
  if (!count) return
  const focusedId = (document.activeElement as HTMLElement | null)?.closest<HTMLElement>('.grid .hit')?.dataset.id
  const current = focusedId ? items.value.findIndex((a) => a.id === focusedId) : -1
  const from = current >= 0 ? current : Math.max(0, items.value.findIndex((a) => a.id === selected.value?.id))
  const perRow = win.columns.value
  const delta = { ArrowRight: 1, ArrowLeft: -1, ArrowDown: perRow, ArrowUp: -perRow, j: 1, k: -1 }[key] ?? 0
  const next = current < 0 ? from : Math.min(count - 1, Math.max(0, from + delta))
  const id = items.value[next].id
  focusTile(id)
  if (selected.value && selected.value.id !== id) openAsset(items.value[next])
  if (next >= count - perRow && count < total.value && !loading.value) runSearch(true)
}

function onKey(e: KeyboardEvent) {
  const target = e.target instanceof HTMLElement ? e.target : null
  const typing = target?.closest('input, textarea, select, [contenteditable]')
  // The palette opens from anywhere, even mid-typing, and the same keys close it.
  if ((e.metaKey || e.ctrlKey) && !e.altKey && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    if (paletteOpen.value) document.querySelector<HTMLDialogElement>('dialog.palette')?.close()
    else paletteOpen.value = true
    return
  }
  // A modal (palette, shortcuts, confirm, inspector) has the keyboard to itself.
  if (target?.closest('dialog[open]')) return
  if (e.key === 'Escape') {
    if (showFilters.value) showFilters.value = false
    else if (selecting.value) {
      checked.value = []
      selectMode.value = false
    }
    else if (selected.value) closeDetail()
    return
  }
  if (typing || e.altKey) return
  if (e.key === '?') {
    e.preventDefault()
    shortcutsOpen.value = true
    return
  }
  if (view.value !== 'library') return
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
    // On the full page the grid is out of sight: left and right step through it, up and down scroll the page.
    if (prefs.detailPage && selected.value) {
      if (e.key === 'ArrowUp' || e.key === 'ArrowDown') return
      e.preventDefault()
      step(e.key === 'ArrowLeft' || e.key === 'k' ? -1 : 1)
      return
    }
    // Arrow keys still scroll the detail panel when it has focus.
    if (target?.closest('.detail-pane') && e.key.startsWith('Arrow')) return
    e.preventDefault()
    moveFocus(e.key)
    return
  }
  const focused = items.value.find((a) => a.id === target?.dataset.id) ?? selected.value
  if (e.key === ' ' && focused && !target?.closest('.detail-pane')) {
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
// (#packs, #archive, #settings/<page>, #asset=<id>; MCP links use the last).
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
  const like = u.get('similar')
  filters.similar = like ? { id: like, name: u.get('similar_name') ?? 'asset' } : null
  const color = u.get('color') ?? ''
  filters.color = /^#[0-9a-f]{6}$/i.test(color) ? color.toLowerCase() : ''
}
/** What a history entry is: going somewhere new adds an entry; refining where you are rewrites it. */
function urlState() {
  return {
    view: view.value,
    section: view.value === 'settings' ? settingsSection.value : '',
    asset: selected.value?.id ?? '',
    place: [
      filters.kind, filters.tags.join(','), filters.sort, filters.pack?.id, filters.collection?.id, filters.similar?.id,
      filters.color,
    ].join('|'),
  }
}
let lastState = urlState()
/** Set while Back/Forward is being followed, so following it doesn't write history of its own. */
let restoring = false
function writeUrl() {
  if (restoring) return
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
  if (filters.similar) {
    u.set('similar', filters.similar.id)
    u.set('similar_name', filters.similar.name)
  }
  if (filters.color) u.set('color', filters.color)
  const qs = u.toString()
  const hash =
    view.value === 'packs' ? '#packs'
    : view.value === 'archive' ? '#archive'
    : view.value === 'settings' ? `#settings/${settingsSection.value}`
    : selected.value ? `#asset=${selected.value.id}` : ''
  const url = `${location.pathname}${qs ? `?${qs}` : ''}${hash}`
  const was = lastState
  const now = (lastState = urlState())
  if (url === location.pathname + location.search + location.hash) return
  // A new view, Settings page, filter, or opening or closing an asset is somewhere Back should return from.
  // Typing a search, or stepping from one open asset to the next, only rewrites where you are.
  // So is following a link in the panel to another asset.
  const followed = following !== '' && now.asset === following
  if (followed) following = ''
  const moved =
    followed || now.view !== was.view || now.section !== was.section || now.place !== was.place || !now.asset !== !was.asset
  if (moved) history.pushState({ key: entryKey() }, '', url)
  else history.replaceState(history.state, '', url)
}
watch(
  [
    view,
    settingsSection,
    () => selected.value?.id,
    () => filters.q,
    () => filters.kind,
    () => filters.tags.join(','),
    () => filters.sort,
    () => filters.pack?.id,
    () => filters.collection?.id,
    () => filters.similar?.id,
    () => filters.color,
  ],
  writeUrl,
)

watch(view, (v) => {
  if (v !== 'library') stopPreview()
  showFilters.value = false
})

onMounted(() => {
  readUrl()
  const deep = location.hash.match(/^#asset=([\w-]+)/)
  if (deep) openAsset(deep[1])
  lastState = urlState()
  if (!currentKey()) history.replaceState({ ...(history.state as object | null), key: entryKey() }, '')
  document.addEventListener('scroll', onAnyScroll, { capture: true, passive: true })
  runSearch()
  refreshPacks()
  schedulePoll()
  window.addEventListener('keydown', onKey)
  window.addEventListener('popstate', onHistory)
  if (sideEl.value) unswipeSide = swipeToClose(sideEl.value, -1, () => (showFilters.value = false))
})
onBeforeUnmount(() => {
  unswipeSide()
  clearInterval(poll)
  observer?.disconnect()
  window.removeEventListener('keydown', onKey)
  window.removeEventListener('popstate', onHistory)
  document.removeEventListener('scroll', onAnyScroll, { capture: true })
})
/** Back/Forward or a typed address: put the page back the way the address says. */
async function onHistory() {
  restoring = true
  try {
    readUrl()
    const next = viewFromHash()
    if (next.section) settingsSection.value = next.section
    const deep = location.hash.match(/^#asset=([\w-]+)/)
    if (deep) {
      const top = scrolls.get(currentKey()) ?? 0
      if (selected.value?.id !== deep[1]) {
        scrollOnOpen = top
        following = ''
        await openAsset(deep[1])
      } else scrollPanel(top)
    } else {
      if (selected.value) closeDetail()
      setView(next.view)
    }
    // Let the watchers see the restored state before history is written again.
    await new Promise((r) => setTimeout(r))
  } finally {
    restoring = false
    lastState = urlState()
  }
}

const busyPacks = computed(() => packs.value.filter(busy))
const shownTotal = useTween(() => total.value)

/** Change the theme with the new one spreading out in a circle from the button that was pressed. */
function setTheme(value: typeof prefs.theme, e?: MouseEvent) {
  if (prefs.theme === value) return
  // From the palette there's no button: spread from the theme switcher's current button instead.
  const from = (e?.currentTarget as HTMLElement | undefined) ?? document.querySelector('.theme [aria-pressed="true"]')
  const r = from?.getBoundingClientRect() ?? new DOMRect(innerWidth / 2, innerHeight / 2, 0, 0)
  const x = r.left + r.width / 2
  const y = r.top + r.height / 2
  const reach = Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y))
  void withTransition(() => (prefs.theme = value), 'theme', () => {
    document.documentElement.animate(
      { clipPath: [`circle(0 at ${x}px ${y}px)`, `circle(${reach}px at ${x}px ${y}px)`] },
      { duration: 520, easing: 'cubic-bezier(0.65, 0, 0.35, 1)', pseudoElement: '::view-transition-new(root)' },
    )
  })
}
const firstLoad = computed(() => loading.value && !items.value.length)

// ------------------------------------------------------------------ command palette

const paletteOpen = ref(false)
const shortcutsOpen = ref(false)
const archivedCount = computed(() => (stats.value?.archived_packs ?? 0) + (stats.value?.archived_assets ?? 0))
function openSettings(section?: string) {
  if (!member.value) return askToSignIn()
  if (section) settingsSection.value = section
  setView('settings')
}
/** Open the format wizard on a file, from its details. */
function teachFile(a: Asset) {
  teach.value = { asset_id: a.id, pack_id: a.pack_id, pack_name: a.pack_name, path: a.path }
  openSettings('store-files')
}
const mod = /Mac|iPhone|iPad/.test(navigator.platform) ? '⌘' : 'Ctrl+'
const commands = computed<Command[]>(() => {
  const out: Command[] = [
    { id: 'go:library', group: 'Go to', label: 'Library', keywords: 'assets grid home', run: () => setView('library') },
    { id: 'go:packs', group: 'Go to', label: 'Packs', hint: busyPacks.value.length ? `${busyPacks.value.length} working` : undefined, run: () => setView('packs') },
    { id: 'go:archive', group: 'Go to', label: 'Archive', hint: archivedCount.value ? archivedCount.value.toLocaleString() : undefined, keywords: 'trash recycle bin deleted restore', run: () => setView('archive') },
    { id: 'go:stats', group: 'Go to', label: 'Library stats', keywords: 'statistics numbers size disk space count', run: () => openSettings('library') },
    { id: 'go:search', group: 'Go to', label: 'Search', hint: '/', run: () => { setView('library'); void nextTick(() => searchInput.value?.focus()) } },
  ]
  if (editor.value) {
    out.push({ id: 'act:add', group: 'Actions', label: 'Add files…', keywords: 'upload import', run: () => fileInput.value?.click() })
    out.push({ id: 'act:folder', group: 'Actions', label: 'Import a folder…', keywords: 'upload pack', run: () => folderInput.value?.click() })
  }
  if (view.value === 'library' && items.value.length) {
    out.push({ id: 'act:all', group: 'Actions', label: `Select all ${items.value.length} shown`, hint: `${mod}A`, keywords: 'check', run: () => (checked.value = items.value.map((a) => a.id)) })
  }
  if (view.value === 'library' && !selecting.value) out.push({ id: 'act:select', group: 'Actions', label: 'Select several…', hint: 'X', keywords: 'bulk multiple tag archive', run: () => (selectMode.value = true) })
  if (checked.value.length && editor.value) out.push({ id: 'act:archive', group: 'Actions', label: `Archive the ${checked.value.length} selected`, keywords: 'delete remove trash', run: archiveChecked })
  if (checked.value.length) out.push({ id: 'act:none', group: 'Actions', label: 'Clear the selection', hint: 'Esc', run: () => (checked.value = []) })
  if (hasFilters.value) out.push({ id: 'act:clear', group: 'Actions', label: 'Clear filters', keywords: 'reset', run: clearFilters })
  out.push({
    id: 'act:supporting', group: 'Actions', label: prefs.showSupporting ? 'Hide supporting files' : 'Show supporting files',
    keywords: 'normal maps roughness masks', run: () => (prefs.showSupporting = !prefs.showSupporting),
  })
  if (auth.status?.signed_in) {
    out.push({ id: 'act:account', group: 'Actions', label: 'Account and access tokens', keywords: 'password login user token api', run: () => openSettings('account') })
    out.push({ id: 'act:signout', group: 'Actions', label: `Sign out (${auth.status.user?.name ?? ''})`, keywords: 'log out logout', run: signOut })
  } else {
    out.push({ id: 'act:signin', group: 'Actions', label: 'Sign in', keywords: 'log in login account password', run: askToSignIn })
  }
  if (member.value) out.push({ id: 'act:settings', group: 'Actions', label: 'Settings', keywords: 'blender ffmpeg llm ai tagging unreal admin tools preferences', run: () => openSettings() })
  out.push({ id: 'act:keys', group: 'Actions', label: 'Keyboard shortcuts', hint: '?', keywords: 'help keys', run: () => (shortcutsOpen.value = true) })
  for (const t of THEMES) {
    out.push({ id: `theme:${t.value}`, group: 'Theme', label: t.label, hint: prefs.theme === t.value ? 'current' : undefined, keywords: 'theme appearance mode', run: () => setTheme(t.value) })
  }
  for (const t of LIGHT_THEMES) {
    out.push({ id: `light:${t.id}`, group: 'Theme', label: `Light theme: ${t.name}`, hint: themeFor('light').id === t.id ? 'current' : undefined, keywords: `appearance color ${t.blurb}`, run: () => (prefs.lightTheme = t.id) })
  }
  for (const t of DARK_THEMES) {
    out.push({ id: `dark:${t.id}`, group: 'Theme', label: `Dark theme: ${t.name}`, hint: themeFor('dark').id === t.id ? 'current' : undefined, keywords: `appearance color ${t.blurb}`, run: () => (prefs.darkTheme = t.id) })
  }
  for (const k of kindFacets.value) {
    const label = KIND_LABEL[k.value as AssetKind] ?? k.value
    out.push({
      id: `kind:${k.value}`, group: 'Kinds', label: filters.kind === k.value ? `All kinds (stop showing only ${label.toLowerCase()})` : `Only ${label.toLowerCase()}`,
      hint: k.count.toLocaleString(), keywords: label, run: () => { setView('library'); setKind(k.value) },
    })
  }
  for (const p of packs.value) out.push({ id: `pack:${p.id}`, group: 'Packs', label: p.name, run: () => browsePack(p) })
  for (const c of collections.value) out.push({ id: `col:${c.id}`, group: 'Collections', label: c.name, run: () => browseCollection(c) })
  return out
})
</script>

<template>
  <div
    ref="appEl"
    class="app"
    :class="{
      'with-detail': selected && view === 'library' && !prefs.detailPage,
      'page-detail': selected && view === 'library' && prefs.detailPage,
      'filters-open': showFilters,
      fitted: !!fit,
    }"
    :style="appStyle"
    @dragenter.prevent="onDragEnter"
    @dragover.prevent
    @dragleave="onDragLeave"
    @drop.prevent="onDrop"
  >
    <header class="top">
      <a href="#" class="brand" :title="stats?.version ? `Hoard ${stats.version} — back to the library` : 'Back to the library'" @click.prevent="goHome">
        <svg viewBox="0 0 32 32" width="24" height="24" aria-hidden="true">
          <rect x="3" y="3" width="12" height="12" rx="2.5" fill="var(--k-texture)" />
          <rect x="17" y="3" width="12" height="12" rx="2.5" fill="var(--k-audio)" />
          <rect x="3" y="17" width="12" height="12" rx="2.5" fill="var(--k-model)" />
          <rect x="17" y="17" width="12" height="12" rx="2.5" fill="var(--k-font)" />
        </svg>
        <span>Hoard</span>
      </a>

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
        <button type="button" :aria-current="view === 'library' ? 'page' : undefined" @click="setView('library')">Library</button>
        <button type="button" :aria-current="view === 'packs' ? 'page' : undefined" @click="setView('packs')">
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
            @click="setTheme(t.value, $event)"
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
        <button v-if="auth.status && !auth.status.signed_in" type="button" class="btn ghost" @click="askToSignIn">Sign in</button>
        <button
          v-if="archivedCount || view === 'archive'"
          type="button"
          class="btn ghost archive-btn"
          :aria-pressed="view === 'archive'"
          :title="`Archive: ${countText(stats?.archived_packs ?? 0, stats?.archived_assets ?? 0)}`"
          aria-label="Archive"
          @click="setView(view === 'archive' ? 'library' : 'archive')"
        >
          <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <rect x="1.8" y="2.5" width="12.4" height="3.3" rx="0.8" />
            <path d="M3 5.8v6.9a.8.8 0 0 0 .8.8h8.4a.8.8 0 0 0 .8-.8V5.8M6.5 8.6h3" />
          </svg>
          <span v-if="archivedCount" class="acount">{{ archivedCount > 999 ? '999+' : archivedCount }}</span>
        </button>
        <button
          v-if="member"
          type="button"
          class="btn ghost settings-btn"
          title="Settings"
          aria-label="Settings"
          :aria-pressed="view === 'settings'"
          @click="view === 'settings' ? setView('library') : openSettings()"
        >
          <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
            <path d="M2.5 4.5h6M11.5 4.5h2M2.5 11.5h2M7.5 11.5h6" />
            <circle cx="10" cy="4.5" r="1.6" />
            <circle cx="6" cy="11.5" r="1.6" />
          </svg>
        </button>
        <button type="button" class="btn ghost cmdk" :title="`Command palette (${mod}K)`" @click="paletteOpen = true">
          <kbd>{{ mod.replace('+', '') }}</kbd><kbd>K</kbd>
        </button>
        <template v-if="editor">
          <button type="button" class="btn import-folder" @click="folderInput?.click()">Import folder</button>
          <button type="button" class="btn primary" @click="fileInput?.click()">
            <span class="wide-only">Add files</span><span class="narrow-only" aria-hidden="true">Add</span>
          </button>
        </template>
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
    <InsecureNotice v-else :member="member" @settings="openSettings('connection')" />


    <template v-if="view === 'library'">
      <div v-if="showFilters" class="scrim" aria-hidden="true" @click="showFilters = false" />
      <aside id="filters" ref="sideEl" class="side" aria-label="Filters">
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
        <section class="colors">
          <h3>Colour</h3>
          <div class="swatches">
            <button
              v-for="c in COLOR_PRESETS"
              :key="c.hex"
              type="button"
              class="sw"
              :style="{ background: c.hex }"
              :aria-pressed="filters.color === c.hex"
              :aria-label="`${c.name}: assets with much ${c.name.toLowerCase()} in them`"
              :title="c.name"
              @click="pickColor(c.hex)"
            />
            <label class="sw pick" :class="{ on: filters.color && !COLOR_PRESETS.some((c) => c.hex === filters.color) }" title="Any colour">
              <span class="visually-hidden">Pick any colour</span>
              <input type="color" :value="filters.color || '#808080'" @change="filters.color = ($event.target as HTMLInputElement).value" />
            </label>
          </div>
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
              :class="{ long: facetLabel(g.ns, f.value).length > 30 }"
              :title="facetLabel(g.ns, f.value).length > 30 ? facetLabel(g.ns, f.value) : undefined"
              :aria-pressed="filters.tags.includes(f.value)"
              @click="toggleTag(f.value)"
            >
              <span class="fl">{{ facetLabel(g.ns, f.value) }}</span><span class="n mono">{{ f.count }}</span>
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

      <main ref="mainEl" class="main">
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
            <span class="count">
              <span aria-hidden="true">{{ firstLoad ? 'Searching…' : `${shownTotal.toLocaleString()} ${total === 1 ? 'asset' : 'assets'}` }}</span>
              <span class="visually-hidden" aria-live="polite">{{ firstLoad ? 'Searching' : `${total.toLocaleString()} ${total === 1 ? 'asset' : 'assets'}` }}</span>
            </span>
            <TagChip v-if="filters.pack" :tag="`pack:${filters.pack.name}`" removable @remove="filters.pack = null" />
            <TagChip v-if="filters.collection" :tag="`collection:${filters.collection.name}`" removable @remove="filters.collection = null" />
            <TagChip v-if="filters.similar" :tag="`like:${filters.similar.name}`" removable @remove="filters.similar = null" />
            <TagChip v-if="filters.color" :tag="`colour:${filters.color}`" :swatch="filters.color" removable @remove="filters.color = ''" />
            <TagChip v-if="filters.kind" :tag="`kind:${filters.kind}`" removable @remove="filters.kind = ''" />
            <TagChip v-for="t in filters.tags" :key="t" :tag="t" removable @remove="toggleTag(t)" />
            <button v-if="hasFilters" type="button" class="linkbtn" @click="clearFilters">Clear all</button>
          </div>
          <button
            v-if="items.length"
            type="button"
            class="btn ghost select-btn"
            :aria-pressed="selecting"
            title="Pick several assets to tag, categorize, collect, download or archive together (X, or Ctrl-click)"
            @click="selecting ? ((checked = []), (selectMode = false)) : (selectMode = true)"
          >
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="2" width="12" height="12" rx="2.5" />
              <path d="m5.2 8.2 2 2 3.8-4.2" />
            </svg>
            <span>{{ selecting ? 'Done' : 'Select' }}</span>
          </button>
          <label class="size" title="Tile size">
            <span class="visually-hidden">Tile size</span>
            <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><rect x="2" y="2" width="5" height="5" rx="1" fill="currentColor" /><rect x="9" y="2" width="5" height="5" rx="1" fill="currentColor" /><rect x="2" y="9" width="5" height="5" rx="1" fill="currentColor" /><rect x="9" y="9" width="5" height="5" rx="1" fill="currentColor" /></svg>
            <input v-model.number="prefs.tile" type="range" min="112" max="280" step="4" />
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><rect x="2" y="2" width="12" height="12" rx="1.5" fill="currentColor" /></svg>
          </label>
          <label class="sort">
            <span class="visually-hidden">Sort</span>
            <select v-model="filters.sort" class="field">
              <option value="">{{ rankLabel }}</option>
              <option v-if="filters.similar || filters.color" value="new">Newest</option>
              <option value="name">Name</option>
              <option value="size">Largest</option>
              <option value="old">Oldest</option>
            </select>
          </label>
        </div>

        <p v-if="supportingHidden > 0 || prefs.showSupporting" class="supporting">
          <template v-if="!prefs.showSupporting">
            <span :title="'glTF buffers, material libraries, the maps of materials, materials only one model wears and the store files describing models. They come along when you download what uses them.'">
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
          <svg class="art crates" viewBox="0 0 140 112" width="150" height="120" aria-hidden="true">
            <g class="c c1"><rect x="30" y="14" width="26" height="26" rx="5" fill="var(--k-texture)" /></g>
            <g class="c c2"><rect x="64" y="2" width="26" height="26" rx="5" fill="var(--k-audio)" /></g>
            <g class="c c3"><rect x="50" y="42" width="26" height="26" rx="5" fill="var(--k-model)" /></g>
            <g class="c c4"><rect x="86" y="32" width="26" height="26" rx="5" fill="var(--k-font)" /></g>
            <path d="M16 64h108l-10 40a6 6 0 0 1-5.8 4.5H31.8A6 6 0 0 1 26 104Z" fill="var(--panel)" stroke="var(--line)" stroke-width="2" stroke-linejoin="round" />
            <path d="M14 64h112" stroke="var(--muted)" stroke-width="3" stroke-linecap="round" />
            <path d="M56 86h28" stroke="var(--line)" stroke-width="3" stroke-linecap="round" />
          </svg>
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
          <svg class="art" viewBox="0 0 124 92" width="124" height="92" aria-hidden="true">
            <rect x="10" y="20" width="66" height="54" rx="7" fill="none" stroke="var(--line)" stroke-width="2" stroke-dasharray="6 5" />
            <rect x="21" y="32" width="15" height="15" rx="3" fill="var(--k-texture)" opacity="0.3" />
            <rect x="42" y="32" width="15" height="15" rx="3" fill="var(--k-audio)" opacity="0.3" />
            <rect x="21" y="53" width="15" height="10" rx="3" fill="var(--k-model)" opacity="0.3" />
            <g class="glass">
              <circle cx="84" cy="44" r="17" fill="var(--bg)" stroke="var(--muted)" stroke-width="3" />
              <path d="M78 38a8 8 0 0 1 9-3" stroke="var(--line)" stroke-width="2.5" stroke-linecap="round" fill="none" />
              <path d="m96.5 56.5 13 13" stroke="var(--muted)" stroke-width="5.5" stroke-linecap="round" />
            </g>
          </svg>
          <p v-if="filters.similar && !filters.q && !filters.kind && !filters.tags.length && !filters.color">
            Nothing else looks much like {{ filters.similar.name }} yet.
          </p>
          <p v-else>Nothing matches{{ filters.q ? ` “${filters.q.trim()}”` : '' }}{{ hasFilters && (filters.tags.length || filters.kind || filters.pack) ? ' with these filters' : '' }}.</p>
          <button v-if="hasFilters" type="button" class="btn" @click="clearFilters">Clear search and filters</button>
        </div>

        <div ref="gridEl" class="grid" :class="{ selecting: checked.length > 0 }" :style="win.padding.value">
          <template v-if="firstLoad">
            <div v-for="n in 18" :key="`s${n}`" class="skeleton" :style="{ '--i': n }" aria-hidden="true"><span /><i /><b /></div>
          </template>
          <AssetTile
            v-for="{ a, i } in windowed"
            :key="a.id"
            :style="i >= fresh ? { '--i': i - fresh } : { animation: 'none' }"
            :asset="a"
            :selected="selected?.id === a.id"
            :checked="checkedSet.has(a.id)"
            :selecting="selecting"
            @open="onTileOpen(a, $event)"
            @check="toggleCheck(a, $event)"
            @similar="findSimilar(a)"
          />
        </div>
        <!-- The next page loads well before the end comes into view; these stand in while it does. -->
        <div v-if="items.length < total" ref="sentinel" class="grid more" aria-hidden="true">
          <div v-for="n in win.columns.value * 2" :key="`m${n}`" class="skeleton" :style="{ '--i': n }"><span /><i /><b /></div>
        </div>
        <p v-if="items.length < total" class="visually-hidden" aria-live="polite">{{ loading ? 'Loading more' : '' }}</p>
        <p v-if="items.length && !checked.length" class="hint">
          Arrow keys move, Enter opens, Space plays a sound, X or Ctrl-click selects several (long-press on touch).
        </p>
      </main>

      <div
        v-if="selected && view === 'library' && fit && !prefs.detailPage"
        class="detail-resize"
        :class="{ dragging: detailDrag !== null }"
        role="separator"
        aria-orientation="vertical"
        aria-label="Resize the details panel"
        :aria-valuenow="fit.detail"
        tabindex="0"
        title="Drag to resize · double-click for the width that fits"
        @pointerdown="startResize"
        @dblclick="prefs.detailWidth = null"
        @keydown="resizeBy"
      />
      <AssetDetail
        v-if="selected"
        class="detail-pane"
        :asset="selected"
        :position="position"
        :collections="collections"
        :page="prefs.detailPage"
        @layout="prefs.detailPage = !prefs.detailPage"
        @close="closeDetail"
        @collect="(id, create) => addToCollection([selected!.id], create ? { create } : { id })"
        @uncollect="(id) => removeFromCollection([selected!.id], id)"
        @browse-collection="browseCollection"
        @updated="onUpdated"
        @tag="toggleTag"
        @error="showError"
        @open="followAsset"
        @step="step"
        @archive="archiveSelected"
        @similar="findSimilar(selected!)"
        @color="findColor"
        @teach="teachFile(selected!)"
      />

      <Transition name="rise">
      <div v-if="selecting" class="selbar" role="region" aria-label="Selected assets">
        <strong>{{ checked.length ? `${checked.length.toLocaleString()} selected` : 'Pick assets' }}</strong>
        <a v-if="checked.length" class="btn primary" :href="selectionUrl" download>Download zip</a>
        <button v-if="checked.length" type="button" class="btn" title="CREDITS.md for the packs these come from: who made them, their licenses and the credit lines they ask for" @click="creditsForChecked">Credits</button>
        <label v-if="checkedWithSource" class="srccheck" title="Unreal SoundWaves and textures whose original file Hoard recovered">
          <input v-model="preferSources" type="checkbox" /> Recovered WAV/PNG instead of .uasset ({{ checkedWithSource }})
        </label>
        <template v-if="editor">
        <form class="bulktag" @submit.prevent="bulkTagEdit(false)">
          <label class="visually-hidden" for="bulk-tag">A tag to add to, or take off, the selected assets</label>
          <input id="bulk-tag" v-model="bulkTag" class="field" placeholder="Tag" autocomplete="off" :disabled="bulkBusy || !checked.length" />
          <button class="btn" type="submit" :disabled="!bulkTag.trim() || bulkBusy" :aria-busy="bulkBusy" title="Add this tag to every selected asset">Add</button>
          <button class="btn" type="button" :disabled="!bulkTag.trim() || bulkBusy" title="Take this tag off every selected asset" @click="bulkTagEdit(true)">Remove</button>
        </form>
        <form class="bulktag" @submit.prevent="bulkSetCategory">
          <label class="visually-hidden" for="bulk-cat">Category for the selected assets (empty clears it)</label>
          <input id="bulk-cat" v-model="bulkCategory" class="field cat" placeholder="Category" autocomplete="off" :disabled="bulkBusy || !checked.length" />
          <button class="btn" type="submit" :disabled="bulkBusy || !checked.length" :title="bulkCategory.trim() ? 'Set the category of every selected asset' : 'Clear the category of every selected asset'">
            {{ bulkCategory.trim() ? 'Set' : 'Clear' }}
          </button>
        </form>
        <CollectionPicker
          v-if="checked.length"
          :collections="collections"
          up
          @add="(id) => addToCollection(checked, { id })"
          @create="(name) => addToCollection(checked, { create: name })"
        />
        <button v-if="filters.collection && checked.length" type="button" class="btn" @click="removeFromCollection(checked, filters.collection.id)">
          Remove from {{ filters.collection.name }}
        </button>
        </template>
        <button type="button" class="linkbtn" @click="checked = items.map((a) => a.id)">Select all {{ items.length }}</button>
        <button v-if="editor" type="button" class="btn danger" :disabled="!checked.length || bulkBusy" title="Out of search until they're restored or deleted for good, from the Archive" @click="archiveChecked">Archive</button>
        <button type="button" class="btn ghost" @click="checked = []; selectMode = false">Done</button>
      </div>
      </Transition>
    </template>

    <SettingsPage
      v-else-if="view === 'settings'"
      v-model:section="settingsSection"
      class="settings-pane"
      :stats="stats"
      @unauthorized="askToSignIn"
      @tag="(t) => { Object.assign(filters, { tags: [], kind: '', pack: null, collection: null, q: '' }); toggleTag(t) }"
      @pack="browsePack"
      @open="openAsset"
      @archive="setView('archive')"
      @changed="refreshPacks"
    />

    <ArchiveView
      v-else-if="view === 'archive'"
      class="archive-pane"
      @changed="onArchiveChanged"
      @error="showError"
    />

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

    <Transition name="drop">
    <div v-if="dragDepth > 0" class="drop" aria-hidden="true">
      <div class="well">
        <svg class="ants" aria-hidden="true"><rect x="1" y="1" rx="12" /></svg>
        <svg class="arrow" viewBox="0 0 24 24" width="34" height="34" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 15V4M7.5 8.5 12 4l4.5 4.5M4 15v3.5A1.5 1.5 0 0 0 5.5 20h13a1.5 1.5 0 0 0 1.5-1.5V15" />
        </svg>
        <strong>{{ dragCount > 1 ? `Drop ${dragCount} items to add them` : 'Drop to add to the hoard' }}</strong>
        <span>Each top-level file or folder becomes its own pack. Gigabytes are fine; big files resume if the connection drops.</span>
      </div>
    </div>
    </Transition>

    <UploadQueue ref="queue" @created="onCreated" @unauthorized="askToSignIn" />
    <CommandPalette v-if="paletteOpen" :commands="commands" @close="paletteOpen = false" @open="openAsset" />
    <ShortcutsDialog v-if="shortcutsOpen" @close="shortcutsOpen = false" />
    <SetupWizard v-if="setupOpen && admin" @done="setupDone" />
    <UiLayer />
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
.app.with-detail.fitted { grid-template-columns: var(--side-w) minmax(0, 1fr) var(--detail-w); }
.app.with-detail.fitted .grid { grid-template-columns: repeat(var(--fit-cols), var(--fit-col)); }
.packs-pane, .archive-pane { grid-row: 3; grid-column: 1 / -1; overflow-y: auto; }
.settings-pane { grid-row: 3; grid-column: 1 / -1; }

.top {
  grid-area: top;
  display: flex; align-items: center; gap: 1rem;
  padding: 0.6rem 1rem; border-bottom: 1px solid var(--line); background: var(--panel);
}
.brand {
  display: flex; align-items: center; gap: 0.5rem; font-weight: 750; font-size: var(--step-1); font-variation-settings: 'CASL' 1;
  color: inherit; text-decoration: none; border-radius: 6px;
}
.brand:focus-visible { outline: 2px solid var(--accent); outline-offset: 3px; }
/* Hover the logo and the bins jostle; the wordmark stays put. */
.brand rect { transform-box: fill-box; transform-origin: center; transition: rotate var(--dur-3) var(--ease-pop), scale var(--dur-3) var(--ease-pop), translate var(--dur-3) var(--ease-pop); }
.brand:hover rect:nth-of-type(1) { rotate: -10deg; translate: -1px -1px; }
.brand:hover rect:nth-of-type(2) { rotate: 8deg; scale: 1.08; transition-delay: 30ms; }
.brand:hover rect:nth-of-type(3) { rotate: 6deg; translate: -1px 1px; transition-delay: 60ms; }
.brand:hover rect:nth-of-type(4) { rotate: -12deg; scale: 0.9; transition-delay: 90ms; }
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
.settings-btn { padding-inline: 0.55em; color: var(--muted); }
.cmdk { gap: 0.2rem; padding-inline: 0.55em; color: var(--muted); }
.cmdk kbd {
  min-width: 1.5em; padding: 0 0.3em; border: 1px solid var(--line); border-bottom-width: 2px; border-radius: 4px;
  font: inherit; font-size: 0.72rem; line-height: 1.5; text-align: center;
}
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
.colors + .side-empty { margin-top: 1rem; }
.swatches { display: grid; grid-template-columns: repeat(auto-fill, minmax(1.45rem, 1fr)); gap: 0.3rem; }
.sw {
  position: relative; aspect-ratio: 1; border: 0; padding: 0; border-radius: 50%; cursor: pointer;
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--text) 18%, transparent);
  transition: scale var(--dur-2) var(--ease-pop), box-shadow var(--dur-1);
}
.sw:hover { scale: 1.12; }
.sw[aria-pressed='true'], .sw.on { box-shadow: 0 0 0 2px var(--bg), 0 0 0 3.5px var(--text); }
.sw.pick {
  display: block; overflow: hidden;
  background: conic-gradient(from 90deg, #e04040, #e0c040, #40c060, #40b0e0, #6050e0, #d040b0, #e04040);
}
.sw.pick input { position: absolute; inset: 0; width: 100%; height: 100%; opacity: 0; cursor: pointer; border: 0; padding: 0; }
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
/* Long tags stop at about 30 characters and fade, so the sidebar never scrolls sideways. */
.facet { max-width: 100%; min-width: 0; }
.facet .fl { min-width: 0; max-width: 30ch; overflow: hidden; white-space: nowrap; }
.facet.long .fl { mask-image: linear-gradient(to right, #000 calc(100% - 2.5em), transparent); }
.facet .n { flex: none; }
.facet[aria-pressed='true'] { background: var(--text); color: var(--bg); border-color: var(--text); }
.facet[aria-pressed='true'] .n { color: var(--bg); opacity: 0.6; }
.more-facets { border-style: dashed; color: var(--muted); }

/* Positioned, so screen-reader-only text far down the grid stays inside it rather than making the page itself
   scroll; and scrolling past the end stops here, never scrolling the whole app out of view. */
.main {
  grid-area: main; position: relative; overflow-y: auto; overscroll-behavior: contain; padding: 0 1.25rem 4rem;
  scrollbar-gutter: stable;
}
.app:has(.selbar) .main { padding-bottom: 7rem; }
.bar {
  position: sticky; top: 0; z-index: 3; display: flex; align-items: center; gap: 0.9rem;
  margin-inline: -1.25rem; padding: 0.75rem 1.25rem; background: var(--bg);
}
/* Once the grid scrolls under it, the bar frosts over and casts a soft edge (scroll-driven, no JS). */
@supports (animation-timeline: scroll()) {
  .main { scroll-timeline: --main block; }
  .bar { animation: bar-lift linear both; animation-timeline: --main; animation-range: 0 40px; }
}
@keyframes bar-lift {
  to {
    background: color-mix(in srgb, var(--bg) 78%, transparent);
    backdrop-filter: blur(12px) saturate(1.3);
    box-shadow: 0 1px 0 var(--line), 0 10px 18px -14px var(--shadow);
  }
}
.active { display: flex; flex-wrap: wrap; gap: 0.35rem; align-items: center; flex: 1; min-width: 0; }
.count { font-size: var(--step--1); color: var(--muted); margin-right: 0.35rem; font-variant-numeric: tabular-nums; }
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
.grid.more { margin-top: 1.1rem; }
.none { color: var(--muted); padding: 3rem 0; display: flex; flex-direction: column; align-items: center; text-align: center; }
.art { display: block; margin-bottom: 1rem; overflow: visible; }
.welcome .art { margin-bottom: 1.25rem; }
.glass { animation: hunt 3.2s var(--ease-in-out) infinite; }
@keyframes hunt { 25% { translate: -14px -6px; } 50% { translate: -26px 6px; } 75% { translate: -8px 8px; } }
.crates .c { animation: bob 2.6s var(--ease-in-out) infinite; }
.crates .c2 { animation-delay: -0.6s; }
.crates .c3 { animation-delay: -1.3s; }
.crates .c4 { animation-delay: -1.9s; }
.crates .c1 rect { rotate: -8deg; transform-box: fill-box; transform-origin: center; }
.crates .c4 rect { rotate: 10deg; transform-box: fill-box; transform-origin: center; }
.none p { margin: 0 0 0.9rem; }
.hint { margin: 2.5rem 0 0; text-align: center; font-size: 0.75rem; color: var(--faint); }
.welcome { max-width: 60ch; padding: 3rem 0; }
.welcome h1 { margin: 0 0 0.75rem; font-size: var(--step-2); line-height: 1.2; font-variation-settings: 'CASL' 0.6; font-weight: 700; }
.welcome p { color: var(--muted); margin: 0 0 1.25rem; }
.welcome-actions { display: flex; gap: 0.5rem; }

/* Placeholder tiles while the first page loads. */
.skeleton {
  display: flex; flex-direction: column; gap: 0.4rem;
  animation: rise-in var(--dur-3) var(--ease-out) backwards; animation-delay: calc(var(--i, 0) * 20ms);
}
.skeleton span { aspect-ratio: 1; border-radius: var(--r-tile); border: 1px solid var(--line); }
.skeleton i, .skeleton b { height: 0.7rem; border-radius: 3px; }
.skeleton i { width: 70%; margin-top: 0.25rem; }
.skeleton b { width: 45%; }
/* One band of light sweeps across every placeholder together: the gradient is pinned to the viewport. */
.skeleton > * {
  background: var(--panel) linear-gradient(100deg, transparent 35%, color-mix(in srgb, var(--raised) 90%, var(--text) 10%) 50%, transparent 65%) fixed;
  background-size: 200vw 100%;
  animation: shimmer 1.6s linear infinite;
}
@keyframes shimmer { from { background-position: 100vw 0; } to { background-position: -100vw 0; } }
@keyframes rise-in { from { opacity: 0; translate: 0 10px; } }

.detail-pane { grid-area: detail; min-height: 0; }
/* The full-page detail covers the filters and the grid, which stay as they were underneath for when it closes. */
.app.page-detail .detail-pane { grid-row: 3; grid-column: 1 / -1; z-index: 5; }
.app.page-detail :is(.side, .main) { visibility: hidden; }
/* The panel's left edge: a strip to grab, drawn as a line on hover or while dragging. */
.detail-resize {
  grid-area: detail; justify-self: start; z-index: 3; width: 10px; margin-left: -5px; cursor: col-resize; touch-action: none;
  position: relative; outline: none;
}
.detail-resize::after {
  content: ''; position: absolute; inset: 0 4px; border-radius: 1px; background: transparent;
  transition: background-color var(--dur-1);
}
.detail-resize:hover::after, .detail-resize:focus-visible::after, .detail-resize.dragging::after { background: var(--accent); }
:global(.resizing), :global(.resizing *) { cursor: col-resize !important; user-select: none !important; }

.selbar {
  position: fixed; left: 50%; bottom: 1rem; translate: -50% 0; z-index: 25;
  display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; max-width: calc(100vw - 2rem);
  padding: 0.6rem 0.75rem 0.6rem 1rem; border: 1px solid var(--muted); border-radius: var(--r-tile);
  background: var(--raised); box-shadow: 0 10px 30px var(--shadow); font-size: var(--step--1);
}
.rise-enter-active { transition: opacity var(--dur-2) var(--ease-out), transform var(--dur-3) var(--ease-pop); }
.rise-leave-active { transition: opacity var(--dur-2) var(--ease-out), transform var(--dur-2) var(--ease-out); }
.rise-enter-from, .rise-leave-to { opacity: 0; transform: translateY(18px) scale(0.97); }
.bulktag { display: flex; gap: 0.35rem; }
.srccheck { display: inline-flex; gap: 0.35rem; align-items: center; color: var(--muted); }
.bulktag .field { width: 11rem; padding: 0.35em 0.6em; }

.drop {
  position: fixed; inset: 0; z-index: 20; display: grid; place-items: center;
  background: color-mix(in srgb, var(--bg) 80%, transparent); backdrop-filter: blur(4px); pointer-events: none;
}
.well {
  position: relative; display: flex; flex-direction: column; gap: 0.4rem; align-items: center; text-align: center;
  max-width: 34rem; padding: 2.5rem 3.5rem; margin: 1rem;
}
/* Marching ants: an SVG outline so the dashes follow the rounded corners. */
.ants { position: absolute; inset: 0; width: 100%; height: 100%; overflow: visible; }
.ants rect {
  width: calc(100% - 2px); height: calc(100% - 2px);
  fill: color-mix(in srgb, var(--k-texture) 7%, transparent); stroke: var(--k-texture); stroke-width: 2;
  stroke-dasharray: 10 8; animation: ants 0.8s linear infinite;
}
@keyframes ants { to { stroke-dashoffset: -18; } }
.arrow { position: relative; color: var(--k-texture); animation: bob 1.4s var(--ease-in-out) infinite; }
@keyframes bob { 50% { translate: 0 -6px; } }
.drop strong { position: relative; font-size: var(--step-2); font-variation-settings: 'CASL' 0.6; }
.drop span { position: relative; color: var(--muted); }
.drop-enter-active { transition: opacity var(--dur-2) var(--ease-out); }
.drop-enter-active .well { transition: scale var(--dur-3) var(--ease-pop); }
.drop-leave-active { transition: opacity var(--dur-1) var(--ease-out); }
.drop-enter-from, .drop-leave-to { opacity: 0; }
.drop-enter-from .well { scale: 0.92; }

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
  .import-folder, .cmdk { display: none; }
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
  .bar { margin-inline: -0.75rem; padding-inline: 0.75rem; }
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
/* Fingers need bigger targets than pointers. */
@media (pointer: coarse) {
  .views button, .theme button { min-height: 44px; min-width: 44px; }
  .linkbtn { padding-block: 0.6em; }
}
@media (max-width: 480px) {
  .wide-only { display: none; }
  .narrow-only { display: inline; }
  .brand span { display: none; }
}
</style>
