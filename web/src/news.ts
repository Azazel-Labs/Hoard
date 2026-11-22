// What's new in Hoard: a short list of things recent releases added, each pointed out with a glinting gold pip where
// it lives until it's been seen (on screen for a moment, or used), and gathered in the top bar's "N new" tray.
// What's been seen is kept with the account (so a new browser doesn't point it all out again), and in this browser
// for guests. Only what this account may use is pointed out: an admin's settings never glint for anyone else.
import { computed, reactive, ref, watch } from 'vue'
import { api, type Role } from './api'
import { auth, may } from './auth'
import { t } from './i18n'
import { toast } from './ui'

/** Where a new thing lives: a page of Settings, or the library (showing one kind of asset). */
export type NewPlace = { settings: string } | { library: { kind?: string } }

export interface NewThing {
  /** Kept in each account's record of what it's seen: short, lowercase, never reused. */
  id: string
  /** When the release that brought it came out (YYYY-MM-DD). It's new for NEW_FOR_DAYS after. */
  date: string
  /** Who can use it: it isn't pointed out to anyone else. */
  need?: Role
  where: NewPlace
  title: () => string
  body: () => string
  /** Where it is, as the tray shows it ("Settings › Appearance"). */
  crumb: () => string[]
}

const settingsCrumb = (page: () => string) => () => [t('settings-title'), page()]

/** Newest first. Take things off the end once they're well past NEW_FOR_DAYS. */
export const NEWS: NewThing[] = [
  {
    id: 'themes-20', date: '2026-11-22', where: { settings: 'appearance' },
    title: () => t('news-themes-title'), body: () => t('news-themes-body'),
    crumb: settingsCrumb(() => t('settings-page-appearance')),
  },
  {
    id: 'tiles-move', date: '2026-11-22', where: { settings: 'appearance' },
    title: () => t('news-tiles-move-title'), body: () => t('news-tiles-move-body'),
    crumb: settingsCrumb(() => t('settings-page-appearance')),
  },
  {
    id: 'motion-trails', date: '2026-11-22', where: { library: { kind: 'animation' } },
    title: () => t('news-motion-trails-title'), body: () => t('news-motion-trails-body'),
    crumb: () => [t('app-library')],
  },
  {
    id: 'text-size', date: '2026-11-21', where: { settings: 'appearance' },
    title: () => t('news-text-size-title'), body: () => t('news-text-size-body'),
    crumb: settingsCrumb(() => t('settings-page-appearance')),
  },
  {
    id: 'tag-tree', date: '2026-11-21', need: 'edit', where: { settings: 'tags' },
    title: () => t('news-tag-tree-title'), body: () => t('news-tag-tree-body'),
    crumb: settingsCrumb(() => t('settings-page-tags')),
  },
  {
    id: 'bulk-edit', date: '2026-11-21', need: 'edit', where: { library: {} },
    title: () => t('news-bulk-edit-title'), body: () => t('news-bulk-edit-body'),
    crumb: () => [t('app-library')],
  },
  {
    id: 'backups', date: '2026-11-20', need: 'admin', where: { settings: 'maintenance' },
    title: () => t('news-backups-title'), body: () => t('news-backups-body'),
    crumb: settingsCrumb(() => t('settings-page-maintenance')),
  },
]

/** How long after its release a thing counts as new, and how long the tray keeps what's been seen. */
const NEW_FOR_DAYS = 60
const SEEN_FOR_DAYS = 30
const DAY = 24 * 60 * 60 * 1000

interface NewsRecord {
  /** Each new thing seen, with when (ISO). */
  seen: Record<string, string>
  /** Asked not to have new things pointed out. */
  off: boolean
}

const KEY = 'hoard.news'

function local(): NewsRecord | null {
  try {
    const raw = localStorage.getItem(KEY)
    return raw ? (JSON.parse(raw) as NewsRecord) : null
  } catch {
    return null
  }
}

export const news = reactive<NewsRecord & { loaded: boolean }>({ seen: {}, off: false, loaded: false })
/** The tray is open. */
export const newsTray = ref(false)

function take(r: NewsRecord | null) {
  news.seen = { ...(r?.seen ?? {}) }
  news.off = !!r?.off
}

let account = false
async function load() {
  account = !!auth.status?.signed_in
  news.loaded = false
  if (!account) take(local())
  else {
    try {
      // An account with nothing kept yet starts from what this browser saw.
      take((await api.news()).news ?? local())
    } catch {
      take(local())
    }
  }
  news.loaded = true
}
watch(() => (auth.status ? auth.status.user?.name ?? '' : null), (who) => who !== null && void load(), { immediate: true })

let timer: ReturnType<typeof setTimeout> | undefined
function save() {
  // Only what's still on the list is worth keeping.
  const ids = new Set(NEWS.map((n) => n.id))
  const r: NewsRecord = { seen: Object.fromEntries(Object.entries(news.seen).filter(([id]) => ids.has(id))), off: news.off }
  try {
    localStorage.setItem(KEY, JSON.stringify(r))
  } catch {
    // Storage unavailable: the account still keeps it.
  }
  if (!account) return
  clearTimeout(timer)
  timer = setTimeout(() => void api.setNews(r).catch(() => {}), 500)
}

/** A new thing this account may use, still within its time. */
function mine(n: NewThing, now: number): boolean {
  if (n.need ? !may(n.need) : 'settings' in n.where && !auth.status?.signed_in) return false
  return now - Date.parse(n.date) < NEW_FOR_DAYS * DAY
}

const now = ref(Date.now())
setInterval(() => (now.value = Date.now()), 60 * 60 * 1000)

/** New things still to see, newest first. */
export const unseen = computed(() => (news.loaded ? NEWS.filter((n) => mine(n, now.value) && !news.seen[n.id]) : []))
/** New things seen lately, most recently seen first. */
export const seenLately = computed(() =>
  NEWS.filter((n) => mine(n, now.value) && news.seen[n.id] && now.value - Date.parse(news.seen[n.id]) < SEEN_FOR_DAYS * DAY).sort(
    (a, b) => news.seen[b.id].localeCompare(news.seen[a.id]),
  ),
)
const unseenIds = computed(() => new Set(unseen.value.map((n) => n.id)))

/** Whether to point out this new thing (or new things at all, without an id). */
export function isNew(id: string): boolean {
  return !news.off && unseenIds.value.has(id)
}
/** The new things on a page of Settings still to see. */
export function newOnPage(page: string): string[] {
  return unseen.value.filter((n) => 'settings' in n.where && n.where.settings === page).map((n) => n.id)
}

export function markSeen(id: string) {
  if (!unseenIds.value.has(id)) return
  news.seen = { ...news.seen, [id]: new Date().toISOString() }
  save()
}
/** Mark everything new as seen; answers with what was, for Undo. */
export function markAllSeen(): string[] {
  const ids = unseen.value.map((n) => n.id)
  if (!ids.length) return ids
  const when = new Date().toISOString()
  news.seen = { ...news.seen, ...Object.fromEntries(ids.map((id) => [id, when])) }
  save()
  return ids
}
/** Mark everything new as seen, with Undo on the note that says so. */
export function markAllSeenWithUndo() {
  const ids = markAllSeen()
  if (ids.length) toast(t('news-marked-seen', { count: ids.length }), { action: { label: t('ui-undo'), run: () => unsee(ids) } })
}
/** Point these out again (Undo). */
export function unsee(ids: string[]) {
  const seen = { ...news.seen }
  for (const id of ids) delete seen[id]
  news.seen = seen
  save()
}
export function setPointingOut(on: boolean) {
  news.off = !on
  save()
}
