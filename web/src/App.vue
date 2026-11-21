<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, shallowRef, watch } from 'vue'
import {
  api, downloadCredits, selectionZipUrl, type Asset, type AssetKind, type BulkEdit, type Collection, type Facet, type Pack, type Stats,
} from './api'
import { filesFromDrop, groupsFromPicked, type Group } from './drop'
import { KIND_LABEL, KIND_NAME } from './kinds'
import { canPreview, stop as stopPreview, toggle as togglePreview } from './player'
import { prefs, toggleIn } from './prefs'
import { askToSignIn, auth, isDesktop, may, mayEdit, signOut, siteName } from './auth'
import { mine, setFavorite, toggleFavorite } from './mine'
import { loadProjects, projects } from './projects'
import { teach } from './storeFormats'
import { DARK_THEMES, LIGHT_THEMES, SCHEMES, setTheme, themeFor } from './theme'
import ActivityIndicator from './components/ActivityIndicator.vue'
import { looksLikeFolderOfPacks, splitPerSubfolder } from './uploader'
import UploadQueue from './components/UploadQueue.vue'
import AssetDetail from './components/AssetDetail.vue'
import SelectionBag from './components/SelectionBag.vue'
import AssetTile from './components/AssetTile.vue'
import SelectionBar from './components/SelectionBar.vue'
import DropOverlay from './components/DropOverlay.vue'
import PacksView from './components/PacksView.vue'
import ActiveFilters from './components/ActiveFilters.vue'
import FilterSidebar from './components/FilterSidebar.vue'
import PackPanel from './components/PackPanel.vue'
import MicButton from './components/MicButton.vue'
import FolderBar from './components/FolderBar.vue'
import NothingHere from './components/NothingHere.vue'
import ThemeSwitch from './components/ThemeSwitch.vue'
import UiLayer from './components/UiLayer.vue'
import CommandPalette from './components/CommandPalette.vue'
import InsecureNotice from './components/InsecureNotice.vue'
import { archiveWithUndo, countText } from './archive'
import { showError, showOk, swipeToClose, withTransition, type Command } from './ui'
import { useVirtualGrid } from './virtualGrid'
import { usePanelFit } from './panelFit'
import { lazy } from './lazy'
import { formatNumber, t } from './i18n'
import { clearFilters, filterPlace, filterQuery, filters, filterSources, hasFilters, heardNoise, NO_MUSIC, readFilters, type Named } from './filters'

// Opened now and then, so they load when first shown.
const SettingsPage = lazy(() => import('./components/SettingsPage.vue'), () => t('app-lazy-settings'))
const ArchiveView = lazy(() => import('./components/ArchiveView.vue'), () => t('app-lazy-archive'))
const ShortcutsDialog = lazy(() => import('./components/ShortcutsDialog.vue'), () => t('app-lazy-shortcuts'), 'overlay')
const SetupWizard = lazy(() => import('./components/SetupWizard.vue'), () => t('app-lazy-setup'), 'overlay')

type View = 'library' | 'packs' | 'settings' | 'archive'
const PAGE = 100

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

const collections = ref<Collection[]>([])
/** The results by place, one per match: empty until its page arrives, so the grid is as tall as every match from
 *  the start and fills in wherever it's scrolled to. */
const slots = shallowRef<(Asset | undefined)[]>([])
/** The results that have arrived, in order. */
const items = computed(() => slots.value.filter((a): a is Asset => !!a))
const total = ref(0)
const supportingHidden = ref(0)
/** The folders in the one being browsed, with how many matches each holds. */
const folders = ref<Facet[]>([])
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
const sideBar = ref<InstanceType<typeof FilterSidebar> | null>(null)
const sideEl = computed(() => (sideBar.value?.$el as HTMLElement | undefined) ?? null)
let unswipeSide = () => {}

const queue = ref<InstanceType<typeof UploadQueue> | null>(null)
const folderInput = ref<HTMLInputElement | null>(null)
/** A dropped/picked folder that contains several packs: ask how to import it. */
const pendingSplit = ref<{ group: Group; parts: number } | null>(null)
const dragDepth = ref(0)
/** How many files or folders are being dragged over the page (0 when the browser won't say). */
const dragCount = ref(0)

// ------------------------------------------------------------------ data

let searchSeq = 0
/** Stops the requests of the search on screen when another takes its place, so the server doesn't go on with them. */
let searchAbort = new AbortController()
/** Pages of the current results that have arrived, and those on their way. */
let pagesIn = new Set<number>()
let pagesOnTheWay = new Map<number, Promise<void>>()
const fetchingPages = ref(0)

function searchPage(page: number, sidebar = false) {
  return api.search({
      q: filters.q.trim() || undefined,
      kind: filters.kind || undefined,
      tags: filters.tags,
      pack: filters.pack?.id,
      folder: filters.pack ? (filters.folder ?? undefined) : undefined,
      collection: filters.collection?.id,
      sort: filters.sort || undefined,
      similar: filters.similar?.id,
      color: filters.color || undefined,
      imitation: filters.imitation || undefined,
      bpm_min: filters.bpmMin === '' ? undefined : filters.bpmMin,
      bpm_max: filters.bpmMax === '' ? undefined : filters.bpmMax,
      key: filters.key || undefined,
      compatible_keys: (filters.key && filters.mixKeys) || undefined,
      favorites: filters.favorites || undefined,
      project: filters.project || undefined,
      reviewed: filters.reviewed ? filters.reviewed === 'yes' : undefined,
      // The sidebar's tags only need the count, not the tiles again.
      limit: sidebar ? 1 : PAGE,
      offset: page * PAGE,
      hide_supporting: !prefs.showSupporting,
      facets: sidebar,
    }, searchAbort.signal)
}

/** Search from the top. `keep` (the same search again, after something changed) leaves the tiles showing in place
 *  while their pages are fetched again, instead of turning them back into placeholders. */
async function runSearch(opts: { keep?: boolean } = {}) {
  const seq = ++searchSeq
  searchAbort.abort()
  searchAbort = new AbortController()
  pagesIn = new Set()
  pagesOnTheWay = new Map()
  fetchingPages.value = 0
  loading.value = true
  try {
    const res = await searchPage(0)
    if (seq !== searchSeq) return
    const next = new Array<Asset | undefined>(res.total)
    for (let i = 0; i < res.total; i++) next[i] = opts.keep ? slots.value[i] : undefined
    res.items.forEach((a, i) => (next[i] = a))
    slots.value = next
    // Different results start from the top, not wherever the last ones were scrolled to.
    if (!opts.keep && mainEl.value?.scrollTop) mainEl.value.scrollTo({ top: 0 })
    pagesIn.add(0)
    total.value = res.total
    supportingHidden.value = res.supporting_hidden ?? 0
    folders.value = res.folders ?? []
    kindFacets.value = res.kinds
    // Counting the tags goes through everything found once more: they follow the tiles instead of holding them up.
    searchPage(0, true)
      .then((sidebar) => seq === searchSeq && (tagFacets.value = sidebar.tags))
      .catch(() => {})
    // A new search's first tiles rise in, once: scrolled back to later, they just appear.
    rising.value = opts.keep ? 0 : res.items.length
    setTimeout(() => seq === searchSeq && (rising.value = 0), 1000)
  } catch (e) {
    if (seq !== searchSeq) return
    showError(t('app-search-failed', { error: (e as Error).message }), { label: t('app-retry'), run: () => runSearch(opts) })
  } finally {
    if (seq === searchSeq) {
      loading.value = false
      await nextTick()
      fillWindow()
    }
  }
}

/** Fetch one page of the current results into its place. */
function loadPage(page: number): Promise<void> {
  if (pagesIn.has(page)) return Promise.resolve()
  const coming = pagesOnTheWay.get(page)
  if (coming) return coming
  const seq = searchSeq
  const onTheWay = pagesOnTheWay
  fetchingPages.value++
  const run = searchPage(page)
    .then((res) => {
      if (seq !== searchSeq) return
      const next = slots.value.slice()
      // Matches came or went since the first page: go by the count as it is now.
      next.length = res.total
      res.items.forEach((a, i) => {
        if (page * PAGE + i < next.length) next[page * PAGE + i] = a
      })
      slots.value = next
      total.value = res.total
      pagesIn.add(page)
    })
    .catch((e: Error) => {
      if (seq === searchSeq) showError(t('app-search-failed', { error: e.message }), { label: t('app-retry'), run: fillWindow })
    })
    .finally(() => {
      onTheWay.delete(page)
      if (seq === searchSeq) fetchingPages.value--
    })
  onTheWay.set(page, run)
  return run
}

/** Fetch the pages under the rows in view, and a screenful either side so scrolling on finds them there. */
function fillWindow() {
  if (loading.value || !slots.value.length) return
  const ahead = win.end.value - win.start.value
  const from = Math.max(0, win.start.value - ahead)
  const to = Math.min(slots.value.length, win.end.value + ahead)
  // Nearest first: the rows in view, then the ones the scrolling is heading for.
  const mid = Math.floor((win.start.value + win.end.value) / 2 / PAGE)
  const pages = []
  for (let p = Math.floor(from / PAGE); p * PAGE < to; p++) pages.push(p)
  pages.sort((a, b) => Math.abs(a - mid) - Math.abs(b - mid))
  for (const p of pages) void loadPage(p)
}

let debounce: ReturnType<typeof setTimeout> | undefined
/** The search field's hint: the noise being searched by, or, searching sounds, that a noise can be written there too. */
const searchPlaceholder = computed(() =>
  heardNoise.value
    ? t('app-imitation-heard')
    : filters.kind === 'audio'
      ? `${t('app-search-placeholder')} · ${t('app-imitation-placeholder')}`
      : t('app-search-placeholder'),
)
watch(() => filters.q, () => {
  // Words typed take the place of a noise made into the microphone.
  if (filters.q.trim() && heardNoise.value) filters.imitation = ''
  clearTimeout(debounce)
  debounce = setTimeout(() => runSearch(), 200)
})
watch(() => [
  filters.kind, filters.tags.join(','), filters.pack?.id, filters.folder, filters.collection?.id, filters.sort, filters.similar?.id,
  filters.color, filters.imitation, filters.favorites, filters.project, prefs.showSupporting,
  filters.bpmMin, filters.bpmMax, filters.key, filters.mixKeys,
], () => {
  checked.value = []
  runSearch()
})

/** Packs, the library's numbers, collections and projects, after a change. Says whether a pack had just finished. */
async function refreshPacks(): Promise<boolean> {
  try {
    const [p, s, c] = await Promise.all([api.packs(), api.stats(), api.collections(), loadProjects()])
    const wasBusy = packs.value.some(busy)
    packs.value = p
    stats.value = s
    collections.value = c
    caughtUp = Date.now()
    // A pack just finished: its assets are now searchable.
    if (wasBusy && !p.some(busy)) {
      runSearch({ keep: true })
      return true
    }
  } catch (e) {
    showError(t('app-unreachable', { error: (e as Error).message }), { label: t('app-retry'), run: refreshPacks })
  }
  return false
}
const busy = (p: Pack) => p.status === 'ingesting' || p.status === 'analyzing' || p.status === 'uploading'

// While packs import, the portal asks after just those every couple of seconds (more slowly from a tab in the
// background), and only once the last answer is in, so a server busy importing hundreds of packs is never asked
// again before it has answered. Everything else (the library's numbers, collections, the grid as the new assets get
// their pictures and tags) follows when a pack finishes, at most every ten seconds, and at once when the last does.
const POLL = 2000
const POLL_HIDDEN = 15_000
const CATCH_UP = 10_000
let poll: ReturnType<typeof setTimeout> | undefined
let caughtUp = 0
let behind = false
function schedulePoll() {
  clearTimeout(poll)
  poll = setTimeout(async () => {
    await pollPacks().catch(() => {})
    schedulePoll()
  }, document.hidden ? POLL_HIDDEN : POLL)
}
async function pollPacks() {
  const before = packs.value.filter(busy)
  if (!before.length) return
  const now = await api.busyPacks()
  const fresh = new Map(now.map((p) => [p.id, p]))
  const known = new Set(packs.value.map((p) => p.id))
  packs.value = [...now.filter((p) => !known.has(p.id)), ...packs.value.map((p) => fresh.get(p.id) ?? p)]
  if (before.some((p) => !fresh.has(p.id)) || now.some((p) => !known.has(p.id))) behind = true
  const analyzing = now.some((p) => p.status === 'analyzing')
  if (!now.length || ((behind || analyzing) && Date.now() - caughtUp >= CATCH_UP)) {
    behind = false
    // The last pack finishing searches again by itself.
    if (!(await refreshPacks())) runSearch({ keep: true })
  }
}


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
  Object.assign(filters, {
    pack: { id: p.id, name: p.name }, folder: prefs.packFolders ? '' : null, collection: null, favorites: false, project: '',
    kind: '', tags: [], q: '', similar: null, color: '', imitation: '', ...NO_MUSIC,
  })
  // A pack picked is the pack to see: the asset open makes way for what the pack is.
  opening = ''
  selected.value = null
  packPanelHidden.value = ''
  setView('library')
}
/** Going to a pack's files: an asset of another pack, or one shown full-page over the grid, is put away. */
function leaveDetailFor(packId: string) {
  if (!selected.value || (selected.value.pack_id === packId && !prefs.detailPage)) return
  opening = ''
  selected.value = null
}
/** A folder of a pack, as it was on disk: what's in it, and the folders in that. */
function openFolder(pack: Named, folder: string) {
  Object.assign(filters, {
    pack: { id: pack.id, name: pack.name }, folder, collection: null, favorites: false, project: '', kind: '', tags: [], q: '',
    similar: null, color: '', imitation: '', ...NO_MUSIC,
  })
  prefs.packFolders = true
  leaveDetailFor(pack.id)
  setView('library')
}
/** The asset's folder `path` in its pack, leaving the full-page view of it for the grid. */
function openFolderOf(a: Asset, path: string) {
  if (prefs.detailPage) closeDetail()
  openFolder({ id: a.pack_id, name: a.pack_name }, path)
}
function browseCollection(c: Collection | Named) {
  const same = filters.collection?.id === c.id
  Object.assign(filters, {
    collection: same ? null : { id: c.id, name: c.name }, pack: null, kind: '', tags: [], q: '', similar: null, color: '', imitation: '', ...NO_MUSIC,
    favorites: false, project: '',
  })
  setView('library')
  showFilters.value = false
}
function browseProject(name: string) {
  const same = filters.project === name
  Object.assign(filters, { project: same ? '' : name, favorites: false, collection: null, pack: null, kind: '', tags: [], q: '', similar: null, color: '', imitation: '', ...NO_MUSIC })
  setView('library')
  showFilters.value = false
}
function browseFavorites() {
  const same = filters.favorites
  Object.assign(filters, { favorites: !same, project: '', collection: null, pack: null, kind: '', tags: [], q: '', similar: null, color: '', imitation: '', ...NO_MUSIC })
  setView('library')
  showFilters.value = false
}

// ------------------------------------------------------------------ saved searches

function applySearch(s: { query: string }) {
  readFilters(s.query)
  setView('library')
  showFilters.value = false
}
/** Every selected asset is a favorite already, so the button takes them out. */
const allStarred = computed(() => checked.value.length > 0 && checked.value.every((id) => mine.favorites.has(id)))
const mainEl = ref<HTMLElement | null>(null)
/** How many of a new search's first tiles rise in. */
const rising = ref(0)
const win = useVirtualGrid({ scroller: mainEl, grid: gridEl, count: () => slots.value.length })
// Wherever the grid is scrolled, its pages are fetched. Dragging the scrollbar across thousands of results asks for
// the places it passes only once it slows down.
let fillTimer: ReturnType<typeof setTimeout> | undefined
watch([win.start, win.end], () => {
  clearTimeout(fillTimer)
  fillTimer = setTimeout(fillWindow, pagesOnTheWay.size ? 60 : 0)
})

const appEl = ref<HTMLElement | null>(null)
const { isNarrow, detailDrag, fit, startResize, resizeBy, appStyle } = usePanelFit({
  app: appEl,
  side: sideEl,
  main: mainEl,
  grid: gridEl,
  panel: () => detailScroller(),
})
const windowed = computed(() => {
  const out: { a: Asset | undefined; i: number }[] = []
  for (let i = win.start.value; i < win.end.value; i++) out.push({ a: slots.value[i], i })
  return out
})
/** The logo: back to the whole library, nothing filtered or open, at the top. */
function goHome() {
  clearFilters()
  selected.value = null
  setView('library')
  mainEl.value?.scrollTo({ top: 0, behavior: 'smooth' })
}
/** "More like this": everything that looks like an asset, across every pack, most alike first. */
function findSimilar(a: Asset) {
  Object.assign(filters, {
    similar: { id: a.id, name: a.name }, color: '', imitation: '', ...NO_MUSIC, q: '', kind: '', tags: [], pack: null, collection: null, favorites: false, project: '', sort: '',
  })
  setView('library')
  mainEl.value?.scrollTo({ top: 0 })
}
/** A color from an asset's palette: everything with much of it, across the library. */
function findColor(hex: string) {
  Object.assign(filters, { color: hex, similar: null, q: '', kind: '', tags: [], pack: null, collection: null, favorites: false, project: '', sort: '' })
  setView('library')
  mainEl.value?.scrollTo({ top: 0 })
}
const ranking = computed(() => !!(filters.imitation || filters.similar || filters.color || filters.q.trim()))
const rankLabel = computed(() =>
  filters.imitation ? t('app-rank-imitation') : filters.similar ? t('app-rank-alike') : filters.color ? t('app-rank-color') : t('app-rank-match'),
)
/** Which way each order goes unless turned round: newest, A to Z, biggest first. */
const SORT_UP: Record<string, boolean> = { created: false, updated: false, name: true, size: false }
/**
 * What the library is in order by and which way round, from `filters.sort`: `created`, `updated`, `name` or `size`,
 * with `:asc` or `:desc` after it when turned round (`new` and `old` from older links are `created` either way). Without
 * one it's best match first while there's something to match, and newest first otherwise.
 */
const sortOrder = computed(() => {
  const s = filters.sort === 'new' ? 'created' : filters.sort === 'old' ? 'created:asc' : filters.sort
  const [by, dir] = s.split(':')
  if (!Object.hasOwn(SORT_UP, by)) return ranking.value ? null : { by: 'created', up: false }
  return { by, up: dir ? dir === 'asc' : SORT_UP[by] }
})
function setSort(by: string, up = SORT_UP[by]) {
  filters.sort = !by ? '' : up === SORT_UP[by] ? by : `${by}:${up ? 'asc' : 'desc'}`
}
const sortBy = computed({ get: () => sortOrder.value?.by ?? '', set: (by: string) => setSort(by) })
const sortWay = computed(() => {
  const o = sortOrder.value
  if (!o) return ''
  if (o.by === 'name') return o.up ? t('app-sort-a-to-z') : t('app-sort-z-to-a')
  if (o.by === 'size') return o.up ? t('app-sort-smallest-first') : t('app-sort-largest-first')
  return o.up ? t('app-sort-oldest-first') : t('app-sort-newest-first')
})

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
    showOk(t('app-collection-added', { count: ids.length, name: c.name }), {
      label: t('app-undo'),
      run: () => removeFromCollection(ids, id, true),
    })
    if (selected.value && ids.includes(selected.value.id)) openAsset(selected.value.id)
  } catch (e) {
    showError(t('app-collection-add-failed', { error: (e as Error).message }))
  }
}

async function removeFromCollection(ids: string[], id: string, quiet = false) {
  try {
    const c = await api.collectionMembers(id, { remove: ids })
    await refreshCollections()
    if (!quiet) showOk(t('app-collection-removed', { count: ids.length, name: c.name }), { label: t('app-undo'), run: () => addToCollection(ids, { id }) })
    if (filters.collection?.id === id) {
      checked.value = []
      runSearch({ keep: true })
    }
    if (selected.value && ids.includes(selected.value.id)) openAsset(selected.value.id)
  } catch (e) {
    showError(t('app-collection-remove-failed', { error: (e as Error).message }))
  }
}

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
    if (edit.accept_all_tags || edit.accept_tags?.length) {
      const keep = new Set(edit.accept_tags ?? [])
      tags = tags.map((t) => (keep.has(t.tag) || (edit.accept_all_tags && (t.source === 'rule' || t.source === 'ai')) ? { ...t, source: 'human' as const, confidence: 1 } : t))
    }
    const category = edit.category === undefined ? a.category : edit.category.trim() || null
    const reviewed_at = edit.reviewed === undefined ? a.reviewed_at : edit.reviewed ? (a.reviewed_at ?? new Date().toISOString()) : null
    return { ...a, tags, category, reviewed_at }
  }
  slots.value = slots.value.map((a) => a && patch(a))
  if (selected.value && set.has(selected.value.id)) selected.value = patch(selected.value)
}
/** CREDITS.md for the selected assets' packs (and the packs of what they need, as the zip has them). */
async function creditsForChecked() {
  try {
    await downloadCredits({ selections: [{ ids: [...checked.value], include_deps: true }] })
  } catch (e) {
    showError(t('app-credits-failed', { error: (e as Error).message }))
  }
}
/** Archive what's selected: gone from the grid at once, with Undo on the toast. */
async function archiveChecked() {
  const ids = [...checked.value]
  if (!ids.length) return
  const ok = await archiveWithUndo(t('app-archived-assets', { count: ids.length }), { assets: ids }, onArchiveChanged)
  if (!ok) return
  checked.value = []
  selectMode.value = false
  if (selected.value && ids.includes(selected.value.id)) selected.value = null
}
/** Archive the open asset and close it. */
async function archiveSelected() {
  const a = selected.value
  if (!a) return
  const ok = await archiveWithUndo(t('app-archived-one', { name: a.name }), { assets: [a.id] }, onArchiveChanged)
  if (ok && selected.value?.id === a.id) closeDetail()
}
/** Something went into or came out of the archive: the grid, counts and packs change. */
function onArchiveChanged() {
  runSearch({ keep: true })
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
const glbSelectionUrl = computed(() => selectionZipUrl(checked.value, false, false, true))

/** The selected assets as last seen, so the bag keeps them when the grid shows something else. */
const checkedSeen = shallowRef(new Map<string, Asset>())
watch([checked, items], () => {
  const seen = new Map<string, Asset>()
  const loaded = new Map(items.value.map((a) => [a.id, a]))
  for (const id of checked.value) {
    const a = loaded.get(id) ?? checkedSeen.value.get(id)
    if (a) seen.set(id, a)
  }
  checkedSeen.value = seen
})
const bagAssets = computed(() => [...checkedSeen.value.values()])
/** With assets picked, the panel shows them all (the bag) instead of one; phones keep the bar's buttons. */
const bagShown = computed(() => checked.value.length > 0 && view.value === 'library' && !isNarrow.value)
/** The pack the grid is showing, as the packs list has it. */
const browsedPack = computed(() => (filters.pack ? (packs.value.find((p) => p.id === filters.pack!.id) ?? null) : null))
/** The pack whose panel was put away: browsing another brings the panel back. */
const packPanelHidden = ref('')
/** Browsing a pack with nothing open or picked, the panel tells about the pack. Phones keep the grid to itself. */
const packShown = computed(
  () => view.value === 'library' && !!browsedPack.value && !selected.value && !bagShown.value && !isNarrow.value &&
    packPanelHidden.value !== browsedPack.value.id,
)
/** Whether a panel is beside the grid: the open asset's, the bag, or the pack's. */
const sidePanel = computed(
  () => view.value === 'library' && (bagShown.value || (!!selected.value && !prefs.detailPage) || packShown.value),
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
  const { name, files, handle } = p.group
  // Each part keeps the folder it came from, so a reload can read it again and pick its part back out.
  queue.value?.enqueue(split ? splitPerSubfolder(name, files).map((g) => ({ ...g, handle, root: name })) : [p.group])
}

// Something dragged from the page itself (a picture, a tile) isn't files to add, though a browser offers a picture as one.
let draggingOwn = false
document.addEventListener('dragstart', () => (draggingOwn = true))
document.addEventListener('dragend', () => (draggingOwn = false))
function onDragEnter(e: DragEvent) {
  if (draggingOwn || !editor.value || !e.dataTransfer?.types.includes('Files')) return
  if (!dragDepth.value) dragCount.value = e.dataTransfer.items?.length ?? 0
  dragDepth.value++
}
function onDragLeave() {
  dragDepth.value = Math.max(0, dragDepth.value - 1)
}
async function onDrop(e: DragEvent) {
  dragDepth.value = 0
  const own = draggingOwn
  draggingOwn = false
  if (!e.dataTransfer || own) return
  const groups = await filesFromDrop(e.dataTransfer)
  if (groups.length) importGroups(groups)
}
function onPick(e: Event) {
  const input = e.target as HTMLInputElement
  const files = [...(input.files ?? [])]
  input.value = ''
  importGroups(groupsFromPicked(files))
}
/** Folder picker: webkitRelativePath is "Folder/sub/file.png". */
function onPickFolder(e: Event) {
  const input = e.target as HTMLInputElement
  const files = [...(input.files ?? [])]
  input.value = ''
  if (files.length) importGroups(groupsFromPicked(files))
}
function onCreated() {
  refreshPacks()
}

// ------------------------------------------------------------------ misc

/** The asset the detail panel is opening or showing; cleared on close so a late fetch can't reopen it. */
let opening = ''
/** The asset whose full record the detail panel has (not just the search hit it opened from). */
const fullId = ref('')
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
      fullId.value = id
      if (selected.value) selected.value = full
      else showDetail(full)
    }
  } catch (e) {
    showError(t('app-open-failed', { error: (e as Error).message }))
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
/** Where an asset is in the results, or -1. */
const placeOf = (id: string | undefined) => (id ? slots.value.findIndex((a) => a?.id === id) : -1)

async function step(delta: number) {
  const next = placeOf(selected.value?.id) + delta
  if (next < 0 || next >= slots.value.length) return
  if (!slots.value[next]) await loadPage(Math.floor(next / PAGE))
  const target = slots.value[next]
  if (!target) return
  openAsset(target)
  focusTile(target.id)
}
const position = computed(() => {
  const i = placeOf(selected.value?.id)
  return i < 0 ? null : { index: i, total: total.value, hasPrev: i > 0, hasNext: i < total.value - 1 }
})

function onUpdated(a: Asset) {
  if (selected.value?.id === a.id) selected.value = a
  slots.value = slots.value.map((x) => (x?.id === a.id ? a : x))
}

// ------------------------------------------------------------------ keyboard

function focusTile(id: string) {
  const i = placeOf(id)
  if (i < 0) return
  // Its row may be scrolled out of the page: bring it in first, then focus it once it's rendered.
  win.reveal(i)
  nextTick(() => gridEl.value?.querySelector<HTMLButtonElement>(`.hit[data-id="${id}"]`)?.focus({ preventScroll: true }))
}

/** Move focus through the grid like a file browser; with the detail open, it follows. */
async function moveFocus(key: string) {
  const count = slots.value.length
  if (!count) return
  const current = placeOf((document.activeElement as HTMLElement | null)?.closest<HTMLElement>('.grid .hit')?.dataset.id)
  const from = current >= 0 ? current : Math.max(0, placeOf(selected.value?.id))
  const perRow = win.columns.value
  const delta = { ArrowRight: 1, ArrowLeft: -1, ArrowDown: perRow, ArrowUp: -perRow, j: 1, k: -1 }[key] ?? 0
  const next = current < 0 ? from : Math.min(count - 1, Math.max(0, from + delta))
  if (!slots.value[next]) await loadPage(Math.floor(next / PAGE))
  const a = slots.value[next]
  if (!a) return
  focusTile(a.id)
  if (selected.value && selected.value.id !== a.id) openAsset(a)
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
  if (e.key === 'f' && focused && member.value) {
    toggleFavorite(focused.id)
  }
  if (e.key === 'r' && focused && mayEdit()) {
    void toggleReviewed(focused)
  }
  if (e.key === 't' && focused && mayEdit()) {
    e.preventDefault()
    void focusTagBox(focused)
  }
}

/** Put the cursor in an asset's tag box (the `t` key), opening it first when it isn't. */
async function focusTagBox(a: Asset) {
  if (selected.value?.id !== a.id) void openAsset(a)
  // The panel may still be sliding in or loading: wait for the box, up to a second.
  for (let i = 0; i < 60; i++) {
    const box = selected.value?.id === a.id ? document.querySelector<HTMLInputElement>('.detail-pane #new-tag') : null
    if (box) {
      box.focus()
      return
    }
    await new Promise(requestAnimationFrame)
  }
}

/** Mark one asset gone over, or not (the `r` key), with Undo. */
async function toggleReviewed(a: Asset) {
  const to = !a.reviewed_at
  const set = async (reviewed: boolean) => {
    patchLoaded([a.id], { reviewed })
    try {
      const fresh = await api.patchAsset(a.id, { reviewed })
      if (selected.value?.id === fresh.id) selected.value = fresh
      return true
    } catch (e) {
      patchLoaded([a.id], { reviewed: !reviewed })
      showError(t('asset-save-error', { error: (e as Error).message }))
      return false
    }
  }
  if (await set(to)) {
    showOk(t(to ? 'asset-reviewed-on-named' : 'asset-reviewed-off-named', { name: a.name }), { label: t('asset-undo'), run: () => void set(!to) })
  }
}

// ------------------------------------------------------------------ URL

// The filters live in the query string (filters.ts); the hash holds the view or an open asset
// (#packs, #archive, #settings/<page>, #asset=<id>; MCP links use the last).
/** What a history entry is: going somewhere new adds an entry; refining where you are rewrites it. */
function urlState() {
  return {
    view: view.value,
    section: view.value === 'settings' ? settingsSection.value : '',
    asset: selected.value?.id ?? '',
    place: filterPlace(),
  }
}
let lastState = urlState()
/** Set while Back/Forward is being followed, so following it doesn't write history of its own. */
let restoring = false
function writeUrl() {
  if (restoring) return
  const qs = filterQuery()
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
watch([view, settingsSection, () => selected.value?.id, ...filterSources], writeUrl)

watch(() => filters.pack?.id, (id) => {
  if (!id) filters.folder = null
})

watch(view, (v) => {
  if (v !== 'library') stopPreview()
  showFilters.value = false
})

onMounted(() => {
  readFilters(location.search)
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
  clearTimeout(poll)
  clearTimeout(fillTimer)
  window.removeEventListener('keydown', onKey)
  window.removeEventListener('popstate', onHistory)
  document.removeEventListener('scroll', onAnyScroll, { capture: true })
})
/** Back/Forward or a typed address: put the page back the way the address says. */
async function onHistory() {
  restoring = true
  try {
    readFilters(location.search)
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
  // The group names match the ones CommandPalette shows before anything is typed.
  const goTo = t('palette-group-go-to')
  const actions = t('palette-group-actions')
  const themeGroup = t('palette-group-theme')
  const current = t('palette-hint-current')
  const out: Command[] = [
    { id: 'go:library', group: goTo, label: t('palette-library'), keywords: t('palette-library-keywords'), run: () => setView('library') },
    ...(member.value
      ? [{ id: 'go:favorites', group: goTo, label: t('palette-favorites'), hint: mine.favorites.size ? formatNumber(mine.favorites.size) : undefined, keywords: t('palette-favorites-keywords'), run: browseFavorites }]
      : []),
    { id: 'go:packs', group: goTo, label: t('palette-packs'), hint: busyPacks.value.length ? t('app-packs-working', { count: busyPacks.value.length }) : undefined, run: () => setView('packs') },
    { id: 'go:archive', group: goTo, label: t('palette-archive'), hint: archivedCount.value ? formatNumber(archivedCount.value) : undefined, keywords: t('palette-archive-keywords'), run: () => setView('archive') },
    { id: 'go:stats', group: goTo, label: t('palette-stats'), keywords: t('palette-stats-keywords'), run: () => openSettings('library') },
    { id: 'go:copies', group: goTo, label: t('palette-copies'), keywords: t('palette-copies-keywords'), run: () => openSettings('copies') },
    { id: 'go:search', group: goTo, label: t('palette-search'), hint: '/', run: () => { setView('library'); void nextTick(() => searchInput.value?.focus()) } },
  ]
  if (editor.value) {
    out.push({ id: 'act:add', group: actions, label: t('palette-add-files'), keywords: t('palette-add-files-keywords'), run: () => fileInput.value?.click() })
    out.push({ id: 'act:folder', group: actions, label: t('palette-import-folder'), keywords: t('palette-import-folder-keywords'), run: () => folderInput.value?.click() })
  }
  if (view.value === 'library' && items.value.length) {
    out.push({ id: 'act:all', group: actions, label: t('palette-select-all', { count: items.value.length }), hint: `${mod}A`, keywords: t('palette-select-all-keywords'), run: () => (checked.value = items.value.map((a) => a.id)) })
  }
  if (view.value === 'library' && !selecting.value) out.push({ id: 'act:select', group: actions, label: t('palette-select-several'), hint: 'X', keywords: t('palette-select-several-keywords'), run: () => (selectMode.value = true) })
  if (checked.value.length && editor.value) out.push({ id: 'act:archive', group: actions, label: t('palette-archive-selected', { count: checked.value.length }), keywords: t('palette-archive-selected-keywords'), run: archiveChecked })
  if (checked.value.length) out.push({ id: 'act:none', group: actions, label: t('palette-clear-selection'), hint: 'Esc', run: () => (checked.value = []) })
  if (hasFilters.value) out.push({ id: 'act:clear', group: actions, label: t('palette-clear-filters'), keywords: t('palette-clear-filters-keywords'), run: clearFilters })
  out.push({
    id: 'act:supporting', group: actions, label: prefs.showSupporting ? t('palette-hide-supporting') : t('palette-show-supporting'),
    keywords: t('palette-supporting-keywords'), run: () => (prefs.showSupporting = !prefs.showSupporting),
  })
  if (auth.status?.signed_in) {
    out.push({ id: 'act:account', group: actions, label: t('palette-account'), keywords: t('palette-account-keywords'), run: () => openSettings('account') })
    if (!isDesktop()) out.push({ id: 'act:signout', group: actions, label: t('palette-sign-out', { name: auth.status.user?.name ?? '' }), keywords: t('palette-sign-out-keywords'), run: signOut })
  } else {
    out.push({ id: 'act:signin', group: actions, label: t('palette-sign-in'), keywords: t('palette-sign-in-keywords'), run: askToSignIn })
  }
  if (member.value) out.push({ id: 'act:settings', group: actions, label: t('palette-settings'), keywords: t('palette-settings-keywords'), run: () => openSettings() })
  out.push({ id: 'act:keys', group: actions, label: t('palette-shortcuts'), hint: '?', keywords: t('palette-shortcuts-keywords'), run: () => (shortcutsOpen.value = true) })
  for (const th of SCHEMES) {
    out.push({ id: `theme:${th.value}`, group: themeGroup, label: th.label(), hint: prefs.theme === th.value ? current : undefined, keywords: t('palette-theme-keywords'), run: () => setTheme(th.value) })
  }
  for (const th of LIGHT_THEMES) {
    out.push({ id: `light:${th.id}`, group: themeGroup, label: t('palette-light-theme', { name: th.name }), hint: themeFor('light').id === th.id ? current : undefined, keywords: `${t('palette-theme-variant-keywords')} ${th.blurb}`, run: () => (prefs.lightTheme = th.id) })
  }
  for (const th of DARK_THEMES) {
    out.push({ id: `dark:${th.id}`, group: themeGroup, label: t('palette-dark-theme', { name: th.name }), hint: themeFor('dark').id === th.id ? current : undefined, keywords: `${t('palette-theme-variant-keywords')} ${th.blurb}`, run: () => (prefs.darkTheme = th.id) })
  }
  const kinds = t('palette-group-kinds')
  for (const k of kindFacets.value) {
    const label = KIND_LABEL[k.value as AssetKind] ?? k.value
    const kind = KIND_NAME[k.value as AssetKind] ?? k.value
    out.push({
      id: `kind:${k.value}`, group: kinds, label: filters.kind === k.value ? t('palette-kind-all', { kind }) : t('palette-kind-only', { kind }),
      hint: formatNumber(k.count), keywords: label, run: () => { setView('library'); setKind(k.value) },
    })
  }
  const packsGroup = t('palette-group-packs')
  const collectionsGroup = t('palette-group-collections')
  const projectsGroup = t('palette-group-projects')
  const savedGroup = t('palette-group-saved')
  for (const p of packs.value) out.push({ id: `pack:${p.id}`, group: packsGroup, label: p.name, run: () => browsePack(p) })
  for (const c of collections.value) out.push({ id: `col:${c.id}`, group: collectionsGroup, label: c.name, run: () => browseCollection(c) })
  for (const p of projects.value) out.push({ id: `project:${p.id}`, group: projectsGroup, label: p.name, hint: formatNumber(p.assets), run: () => browseProject(p.name) })
  for (const sv of mine.searches) out.push({ id: `saved:${sv.id}`, group: savedGroup, label: sv.name, run: () => applySearch(sv) })
  return out
})
</script>

<template>
  <div
    ref="appEl"
    class="app"
    :class="{
      'with-detail': sidePanel,
      'page-detail': selected && view === 'library' && prefs.detailPage && !bagShown,
      'filters-open': showFilters,
      fitted: !!fit,
    }"
    :style="appStyle"
    @dragenter.prevent="onDragEnter"
    @dragover.prevent
    @dragleave="onDragLeave"
    @drop.prevent="onDrop"
  >
    <header class="top" data-tauri-drag-region="deep">
      <a href="#" class="brand" :title="stats?.version ? t('app-home-version', { version: stats.version }) : t('app-home')" @click.prevent="goHome">
        <svg viewBox="0 0 32 32" width="24" height="24" aria-hidden="true">
          <rect x="3" y="3" width="12" height="12" rx="2.5" fill="var(--k-texture)" />
          <rect x="17" y="3" width="12" height="12" rx="2.5" fill="var(--k-audio)" />
          <rect x="3" y="17" width="12" height="12" rx="2.5" fill="var(--k-model)" />
          <rect x="17" y="17" width="12" height="12" rx="2.5" fill="var(--k-font)" />
        </svg>
        <span>{{ siteName() }}</span>
      </a>

      <div class="search">
        <label for="q" class="visually-hidden">{{ t('app-search-label') }}</label>
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
          :placeholder="searchPlaceholder"
          :title="filters.kind === 'audio' ? t('app-imitation-title') : undefined"
          autocomplete="off"
          enterkeyhint="search"
          @focus="view = 'library'"
          @keydown.down.prevent="moveFocus('ArrowRight')"
        />
        <kbd class="slash" aria-hidden="true">/</kbd>
        <MicButton />
      </div>

      <nav class="views" :aria-label="t('app-views')">
        <button type="button" :aria-current="view === 'library' ? 'page' : undefined" @click="setView('library')">{{ t('app-library') }}</button>
        <button type="button" :aria-current="view === 'packs' ? 'page' : undefined" @click="setView('packs')">
          {{ t('app-packs') }}<span v-if="busyPacks.length" class="pulse" :title="t('app-packs-working', { count: busyPacks.length })" />
        </button>
      </nav>

      <div class="actions">
        <a
          v-if="stats?.update"
          class="btn ghost update"
          :href="stats.update.url"
          target="_blank"
          rel="noopener"
          :title="t('app-update-title', { version: stats.update.version, current: stats.version })"
        >
          <span class="dot" aria-hidden="true" /><span class="wide-only">{{ t('app-update') }} </span>{{ stats.update.version }}
        </a>
        <ActivityIndicator />
        <ThemeSwitch />
        <button v-if="auth.status && !auth.status.signed_in" type="button" class="btn ghost" @click="askToSignIn">{{ t('app-sign-in') }}</button>
        <button
          v-if="archivedCount || view === 'archive'"
          type="button"
          class="btn ghost archive-btn"
          :aria-pressed="view === 'archive'"
          :title="t('app-archive-title', { contents: countText(stats?.archived_packs ?? 0, stats?.archived_assets ?? 0) })"
          :aria-label="t('app-archive')"
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
          :title="t('app-settings')"
          :aria-label="t('app-settings')"
          :aria-pressed="view === 'settings'"
          @click="view === 'settings' ? setView('library') : openSettings()"
        >
          <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
            <path d="M2.5 4.5h6M11.5 4.5h2M2.5 11.5h2M7.5 11.5h6" />
            <circle cx="10" cy="4.5" r="1.6" />
            <circle cx="6" cy="11.5" r="1.6" />
          </svg>
        </button>
        <button type="button" class="btn ghost cmdk" :title="t('app-palette-title', { keys: `${mod}K` })" @click="paletteOpen = true">
          <kbd>{{ mod.replace('+', '') }}</kbd><kbd>K</kbd>
        </button>
        <template v-if="editor">
          <button type="button" class="btn import-folder" @click="folderInput?.click()">{{ t('app-import-folder') }}</button>
          <button type="button" class="btn primary" @click="fileInput?.click()">
            <span class="wide-only">{{ t('app-add-files') }}</span><span class="narrow-only" aria-hidden="true">{{ t('app-add-files-short') }}</span>
          </button>
        </template>
        <input ref="fileInput" type="file" multiple hidden @change="onPick" />
        <input ref="folderInput" type="file" webkitdirectory multiple hidden @change="onPickFolder" />
      </div>
    </header>

    <div v-if="pendingSplit" class="tokenbar splitbar" role="dialog" :aria-label="t('app-split-label')">
      <span>
        {{ t('app-split-before') }}<strong>{{ pendingSplit.group.name }}</strong>{{ t('app-split-after', { count: pendingSplit.parts }) }}
      </span>
      <button type="button" class="btn primary" @click="resolveSplit(true)">{{ t('app-split-each', { count: pendingSplit.parts }) }}</button>
      <button type="button" class="btn" @click="resolveSplit(false)">{{ t('app-split-one') }}</button>
      <button type="button" class="btn ghost" @click="pendingSplit = null">{{ t('app-cancel') }}</button>
    </div>
    <InsecureNotice v-else :member="member" @settings="openSettings('connection')" />


    <template v-if="view === 'library'">
      <div v-if="showFilters" class="scrim" aria-hidden="true" @click="showFilters = false" />
      <FilterSidebar
        ref="sideBar"
        :collections="collections"
        :kind-facets="kindFacets"
        :tag-facets="tagFacets"
        :loading="loading"
        @close="showFilters = false"
        @favorites="browseFavorites"
        @project="browseProject"
        @collection="browseCollection"
        @search="applySearch"
        @kind="setKind"
        @tag="toggleTag"
      />

      <main ref="mainEl" class="main">
        <div class="bar">
          <button
            type="button"
            class="btn filters-toggle"
            aria-controls="filters"
            :aria-expanded="showFilters"
            @click="showFilters = !showFilters"
          >
            {{ t('app-filters') }}<span v-if="filters.tags.length + (filters.kind ? 1 : 0)" class="badge">{{ filters.tags.length + (filters.kind ? 1 : 0) }}</span>
          </button>
          <ActiveFilters :first-load="firstLoad" :total="total" />
          <button
            v-if="browsedPack && packPanelHidden === browsedPack.id && !selected && !bagShown && !isNarrow"
            type="button"
            class="btn ghost"
            @click="packPanelHidden = ''"
          >{{ t('pack-panel-label') }}</button>
          <button
            v-if="items.length"
            type="button"
            class="btn ghost select-btn"
            :aria-pressed="selecting"
            :title="t('app-select-title')"
            @click="selecting ? ((checked = []), (selectMode = false)) : (selectMode = true)"
          >
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="2" width="12" height="12" rx="2.5" />
              <path d="m5.2 8.2 2 2 3.8-4.2" />
            </svg>
            <span>{{ selecting ? t('app-done') : t('app-select') }}</span>
          </button>
          <label class="size" :title="t('app-tile-size')">
            <span class="visually-hidden">{{ t('app-tile-size') }}</span>
            <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><rect x="2" y="2" width="5" height="5" rx="1" fill="currentColor" /><rect x="9" y="2" width="5" height="5" rx="1" fill="currentColor" /><rect x="2" y="9" width="5" height="5" rx="1" fill="currentColor" /><rect x="9" y="9" width="5" height="5" rx="1" fill="currentColor" /></svg>
            <input v-model.number="prefs.tile" type="range" min="112" max="280" step="4" />
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><rect x="2" y="2" width="12" height="12" rx="1.5" fill="currentColor" /></svg>
          </label>
          <div class="sort">
            <label>
              <span class="visually-hidden">{{ t('app-sort') }}</span>
              <select v-model="sortBy" class="field">
                <option v-if="ranking" value="">{{ rankLabel }}</option>
                <option value="created">{{ t('app-sort-created') }}</option>
                <option value="updated">{{ t('app-sort-updated') }}</option>
                <option value="name">{{ t('app-sort-name') }}</option>
                <option value="size">{{ t('app-sort-size') }}</option>
              </select>
            </label>
            <button
              v-if="sortOrder"
              type="button"
              class="btn ghost sort-way"
              :title="sortWay"
              :aria-label="sortWay"
              @click="setSort(sortOrder.by, !sortOrder.up)"
            >
              <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path v-if="sortOrder.up" d="M5 13V3M2.5 5.5 5 3l2.5 2.5M9.5 4h4M9.5 8h3M9.5 12h1.5" />
                <path v-else d="M5 3v10M2.5 10.5 5 13l2.5-2.5M9.5 4h1.5M9.5 8h3M9.5 12h4" />
              </svg>
            </button>
          </div>
        </div>

        <p v-if="supportingHidden > 0 || prefs.showSupporting" class="supporting">
          <template v-if="!prefs.showSupporting">
            <span :title="t('app-supporting-title')">
              {{ t('app-supporting-hidden', { count: supportingHidden }) }}
            </span>
            <button type="button" class="linkbtn" @click="prefs.showSupporting = true">{{ t('app-supporting-show') }}</button>
          </template>
          <template v-else>
            {{ t('app-supporting-shown') }}
            <button type="button" class="linkbtn" @click="prefs.showSupporting = false">{{ t('app-supporting-hide') }}</button>
          </template>
        </p>

        <FolderBar :folders="folders" />

        <NothingHere
          v-if="!loading && ((stats && stats.assets === 0) || (items.length === 0 && !folders.length))"
          :welcome="!!stats && stats.assets === 0"
          @folder="folderInput?.click()"
          @files="fileInput?.click()"
        />

        <div ref="gridEl" class="grid" :class="{ selecting: checked.length > 0 }" :style="win.padding.value">
          <template v-if="firstLoad">
            <div v-for="n in 18" :key="`s${n}`" class="skeleton" :style="{ '--i': n }" aria-hidden="true"><span /><i /><b /></div>
          </template>
          <template v-for="{ a, i } in windowed" :key="a ? `${a.id}@${i}` : i">
            <AssetTile
              v-if="a"
              :style="i < rising ? { '--i': i } : { animation: 'none' }"
              :asset="a"
            :selected="selected?.id === a.id"
            :checked="checkedSet.has(a.id)"
            :selecting="selecting"
              @open="onTileOpen(a, $event)"
              @check="toggleCheck(a, $event)"
              @similar="findSimilar(a)"
            />
            <!-- Stands in until its page arrives. -->
            <div v-else class="skeleton slot" aria-hidden="true"><span /><i /><b /></div>
          </template>
        </div>
        <p class="visually-hidden" aria-live="polite">{{ fetchingPages ? t('app-loading-more') : '' }}</p>
        <p v-if="items.length && !checked.length" class="hint">
          {{ t('app-keys-hint') }}
        </p>
      </main>

      <div
        v-if="sidePanel && fit"
        class="detail-resize"
        :class="{ dragging: detailDrag !== null }"
        role="separator"
        aria-orientation="vertical"
        :aria-label="t('app-resize-label')"
        :aria-valuenow="fit.detail"
        tabindex="0"
        :title="t('app-resize-title')"
        @pointerdown="startResize"
        @dblclick="prefs.detailWidth = null"
        @keydown="resizeBy"
      />
      <SelectionBag
        v-if="bagShown"
        v-model:prefer-sources="preferSources"
        class="detail-pane"
        :assets="bagAssets"
        :count="checked.length"
        :zip-url="selectionUrl"
        :glb-url="glbSelectionUrl"
        :with-source="checkedWithSource"
        @remove="(id) => (checked = checked.filter((x) => x !== id))"
        @clear="checked = []; selectMode = false"
        @credits="creditsForChecked"
      />
      <AssetDetail
        v-else-if="selected"
        class="detail-pane"
        :asset="selected"
        :position="position"
        :collections="collections"
        :page="prefs.detailPage"
        :full="selected.id === fullId"
        @layout="prefs.detailPage = !prefs.detailPage"
        @close="closeDetail"
        @collect="(id, create) => addToCollection([selected!.id], create ? { create } : { id })"
        @uncollect="(id) => removeFromCollection([selected!.id], id)"
        @browse-collection="browseCollection"
        @browse-project="browseProject"
        @browse-pack="browsePack"
        @updated="onUpdated"
        @tag="toggleTag"
        @error="showError"
        @open="followAsset"
        @step="step"
        @archive="archiveSelected"
        @similar="findSimilar(selected!)"
        @color="findColor"
        @teach="teachFile(selected!)"
        @folder="(path) => openFolderOf(selected!, path)"
      />
      <PackPanel
        v-else-if="packShown && browsedPack"
        class="detail-pane"
        :pack="browsedPack"
        :kind="filters.kind"
        :tags="filters.tags"
        @close="packPanelHidden = browsedPack.id"
        @kind="setKind"
        @tag="toggleTag"
        @packs="setView('packs')"
      />

      <Transition name="rise">
      <SelectionBar
        v-if="selecting"
        v-model:prefer-sources="preferSources"
        :class="{ beside: sidePanel }"
        :checked="checked"
        :items="items"
        :bag-shown="bagShown"
        :zip-url="selectionUrl"
        :with-source="checkedWithSource"
        :editor="editor"
        :member="member"
        :collections="collections"
        :all-starred="allStarred"
        @patched="patchLoaded"
        @credits="creditsForChecked"
        @collect="(target) => addToCollection(checked, target)"
        @uncollect="(id) => removeFromCollection(checked, id)"
        @favorite="(on) => setFavorite(checked, on)"
        @all="checked = items.map((a) => a.id)"
        @archive="archiveChecked"
        @done="checked = []; selectMode = false"
      />
      </Transition>
    </template>

    <SettingsPage
      v-else-if="view === 'settings'"
      v-model:section="settingsSection"
      class="settings-pane"
      :stats="stats"
      @unauthorized="askToSignIn"
      @tag="(t) => { Object.assign(filters, { tags: [], kind: '', pack: null, collection: null, favorites: false, project: '', q: '' }); toggleTag(t) }"
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

    <DropOverlay :show="dragDepth > 0" :count="dragCount" />

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
:global(html.framed .top) { padding-right: calc(3 * 46px + 0.5rem); }
:global(html.framed-mac .top) { padding-left: 84px; }
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
.search .field { background: var(--bg); padding: 0.55em 4.2em 0.55em 2.1em; }
.search-icon { position: absolute; left: 0.7em; top: 50%; translate: 0 -50%; color: var(--faint); pointer-events: none; }
.slash {
  position: absolute; right: 2.4em; top: 50%; translate: 0 -50%; pointer-events: none;
  padding: 0 0.4em; border: 1px solid var(--line); border-radius: 3px; color: var(--faint); font: inherit; font-size: 0.75rem;
}
.search:focus-within .slash { display: none; }
.views { display: flex; gap: 0.15rem; }
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
.update .dot { flex: none; width: 0.5em; height: 0.5em; border-radius: 50%; background: currentColor; }
.narrow-only { display: none; }

.tokenbar { grid-area: token; display: flex; gap: 0.75rem; align-items: center; padding: 0.6rem 1rem; background: var(--raised); font-size: var(--step--1); }
.tokenbar .field { max-width: 320px; }

.side { grid-area: side; overflow-y: auto; padding: 1rem 0.9rem 2rem; border-right: 1px solid var(--line); }

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
.size { display: flex; align-items: center; gap: 0.4rem; color: var(--faint); }
.size input { width: 84px; accent-color: var(--muted); }
.sort { display: flex; align-items: center; gap: 0.25rem; }
.sort .field { width: auto; padding: 0.3em 0.5em; font-size: var(--step--1); }
.sort-way { padding: 0.4em; color: var(--muted); }
.filters-toggle { display: none; }
.badge {
  min-width: 1.2rem; padding: 0 0.3rem; border-radius: 999px; background: var(--text); color: var(--bg);
  font-size: 0.7rem; font-weight: 700; text-align: center;
}
.supporting { margin: -0.25rem 0 0.9rem; font-size: 0.78rem; color: var(--faint); }
.supporting .linkbtn { font-size: inherit; color: var(--muted); }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(var(--tile), 1fr)); gap: 1.1rem 0.9rem; }
.hint { margin: 2.5rem 0 0; text-align: center; font-size: 0.75rem; color: var(--faint); }

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
/* A result not fetched yet, met while scrolling: as tall as a tile, so nothing moves when it arrives, and flat, so a
   screenful of them costs nothing to paint mid-scroll (the shimmer's fixed background repaints on every frame). */
.skeleton.slot { height: var(--row-h); animation: none; }
.skeleton.slot > * { background: var(--panel); animation: none; }

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

.rise-enter-active { transition: opacity var(--dur-2) var(--ease-out), transform var(--dur-3) var(--ease-pop); }
.rise-leave-active { transition: opacity var(--dur-2) var(--ease-out), transform var(--dur-2) var(--ease-out); }
.rise-enter-from, .rise-leave-to { opacity: 0; transform: translateY(18px) scale(0.97); }

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
  /* The search's own row runs under the window's buttons, to the edge. */
  :global(html.framed .top .search) { margin-right: calc(-3 * 46px); }
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
}
/* Fingers need bigger targets than pointers. */
@media (pointer: coarse) {
  .views button { min-height: 44px; min-width: 44px; }
}
@media (max-width: 480px) {
  .wide-only { display: none; }
  .narrow-only { display: inline; }
  .brand span { display: none; }
}
</style>
