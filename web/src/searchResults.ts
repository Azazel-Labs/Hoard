// The library's search results: every match has a place from the first page on, and the pages under the rows in view
// are fetched as the grid scrolls. A new search stops the requests of the one before it.
import { computed, nextTick, onBeforeUnmount, ref, shallowRef } from 'vue'
import { api, type Asset, type Facet, type SearchParams } from './api'
import { filters } from './filters'
import { t } from './i18n'
import { prefs } from './prefs'
import { showError } from './ui'

/** Results fetched at a time. */
export const PAGE = 100

/** What the filters search for, without which page of it. */
export function queryParams(): SearchParams {
  return {
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
    rating_min: filters.ratingMin || undefined,
    hide_supporting: !prefs.showSupporting,
  }
}

export function useSearchResults(opts: {
  /** The places in view in the grid, as of now. */
  inView: () => { start: number; end: number }
  /** A search begins; `keep` when it's the same one again, after something changed. */
  onStart?: (keep: boolean) => void
  /** A different search's first page is in. */
  onNewResults?: () => void
}) {
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
  /** The first page of a search is on its way. */
  const loading = ref(false)
  /** Pages after the first on their way. */
  const fetchingPages = ref(0)
  /** How many of a new search's first tiles rise in. */
  const rising = ref(0)

  let searchSeq = 0
  /** Stops the requests of the search on screen when another takes its place, so the server doesn't go on with them. */
  let searchAbort = new AbortController()
  /** Pages of the current results that have arrived, and those on their way. */
  let pagesIn = new Set<number>()
  let pagesOnTheWay = new Map<number, Promise<void>>()

  function searchPage(page: number, sidebar = false) {
    return api.search({
      ...queryParams(),
      // The sidebar's tags only need the count, not the tiles again.
      limit: sidebar ? 1 : PAGE,
      offset: page * PAGE,
      facets: sidebar,
    }, searchAbort.signal)
  }

  /** Search from the top. `keep` (the same search again, after something changed) leaves the tiles showing in place
   *  while their pages are fetched again, instead of turning them back into placeholders. */
  async function runSearch(o: { keep?: boolean } = {}) {
    const keep = !!o.keep
    const seq = ++searchSeq
    searchAbort.abort()
    searchAbort = new AbortController()
    pagesIn = new Set()
    pagesOnTheWay = new Map()
    fetchingPages.value = 0
    loading.value = true
    opts.onStart?.(keep)
    try {
      const res = await searchPage(0)
      if (seq !== searchSeq) return
      const next = new Array<Asset | undefined>(res.total)
      for (let i = 0; i < res.total; i++) next[i] = keep ? slots.value[i] : undefined
      res.items.forEach((a, i) => (next[i] = a))
      slots.value = next
      if (!keep) opts.onNewResults?.()
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
      rising.value = keep ? 0 : res.items.length
      setTimeout(() => seq === searchSeq && (rising.value = 0), 1000)
    } catch (e) {
      if (seq !== searchSeq) return
      showError(t('app-search-failed', { error: (e as Error).message }), { label: t('app-retry'), run: () => runSearch(o) })
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

  /** The result at a place, fetching its page first if it hasn't arrived. */
  async function resultAt(i: number): Promise<Asset | undefined> {
    if (!slots.value[i]) await loadPage(Math.floor(i / PAGE))
    return slots.value[i]
  }

  /** Fetch the pages under the rows in view, and a screenful either side so scrolling on finds them there. */
  function fillWindow() {
    if (loading.value || !slots.value.length) return
    const { start, end } = opts.inView()
    const ahead = end - start
    const from = Math.max(0, start - ahead)
    const to = Math.min(slots.value.length, end + ahead)
    // Nearest first: the rows in view, then the ones the scrolling is heading for.
    const mid = Math.floor((start + end) / 2 / PAGE)
    const pages = []
    for (let p = Math.floor(from / PAGE); p * PAGE < to; p++) pages.push(p)
    pages.sort((a, b) => Math.abs(a - mid) - Math.abs(b - mid))
    for (const p of pages) void loadPage(p)
  }

  let fillTimer: ReturnType<typeof setTimeout> | undefined
  /** The rows in view moved. Dragging the scrollbar across thousands of results asks for the places it passes only
   *  once it slows down. */
  function scrolled() {
    clearTimeout(fillTimer)
    fillTimer = setTimeout(fillWindow, pagesOnTheWay.size ? 60 : 0)
  }

  onBeforeUnmount(() => {
    clearTimeout(fillTimer)
    searchAbort.abort()
  })

  return {
    slots, items, total, supportingHidden, folders, kindFacets, tagFacets, loading, fetchingPages, rising,
    runSearch, resultAt, scrolled,
  }
}
