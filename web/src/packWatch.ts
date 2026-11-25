import { computed, onBeforeUnmount, ref } from 'vue'
import { api, type Collection, type Pack, type Stats } from './api'
import { t } from './i18n'
import { loadProjects } from './projects'
import { showError } from './ui'

/** Whether a pack is still being brought in. */
export const busy = (p: Pack) => p.status === 'ingesting' || p.status === 'analyzing' || p.status === 'uploading'

// While packs import, the portal asks after just those every couple of seconds (more slowly from a tab in the
// background), and only once the last answer is in, so a server busy importing hundreds of packs is never asked
// again before it has answered. Everything else (the library's numbers, collections, the grid as the new assets get
// their pictures and tags) follows when a pack finishes, at most every ten seconds, and at once when the last does.
const POLL = 2000
const POLL_HIDDEN = 15_000
const CATCH_UP = 10_000

/** The library's packs, numbers and collections, kept up to date while packs import. `searchAgain` runs when what a
 *  search finds may have changed: a pack finished, or imported assets got their pictures and tags. */
export function usePackWatch(searchAgain: () => void) {
  const packs = ref<Pack[]>([])
  const stats = ref<Stats | null>(null)
  const collections = ref<Collection[]>([])
  const busyPacks = computed(() => packs.value.filter(busy))
  let poll: ReturnType<typeof setTimeout> | undefined
  let caughtUp = 0
  let behind = false

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
        searchAgain()
        return true
      }
    } catch (e) {
      showError(t('app-unreachable', { error: (e as Error).message }), { label: t('app-retry'), run: refreshPacks })
    }
    return false
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
      if (!(await refreshPacks())) searchAgain()
    }
  }

  /** Start asking after importing packs; it stops when the component using this goes. */
  function startPolling() {
    clearTimeout(poll)
    poll = setTimeout(async () => {
      await pollPacks().catch(() => {})
      startPolling()
    }, document.hidden ? POLL_HIDDEN : POLL)
  }
  onBeforeUnmount(() => clearTimeout(poll))

  return { packs, stats, collections, busyPacks, refreshPacks, startPolling }
}
