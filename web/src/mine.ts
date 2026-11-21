// What's the signed-in person's own: the assets they've starred and the searches they've saved. Loaded once they're
// signed in (guests have neither) and kept here so every tile and panel shows the same stars.
import { reactive, watch } from 'vue'
import { api, type SavedSearch } from './api'
import { auth } from './auth'
import { showError } from './ui'
import { t } from './i18n'

export const mine = reactive({
  favorites: new Set<string>(),
  searches: [] as SavedSearch[],
  /** The saved searches are in (for an account). */
  loaded: false,
})

/** Starring is for people with an account (and the tokens they make), not guests. */
export function canStar(): boolean {
  return !!auth.status?.signed_in
}

export async function loadMine(): Promise<void> {
  if (!canStar()) {
    mine.favorites = new Set()
    mine.searches = []
    return
  }
  try {
    const [f, s] = await Promise.all([api.favorites(), api.savedSearches()])
    mine.favorites = new Set(f.assets)
    mine.searches = s.searches
    mine.loaded = true
  } catch {
    // Not worth an error of its own: the stars just don't show until the next sign-in or reload.
  }
}

watch(() => auth.status?.user?.name ?? null, loadMine, { immediate: true })

/** Star (or with `on` false, unstar) these assets. */
export async function setFavorite(ids: string[], on: boolean): Promise<void> {
  if (!ids.length) return
  // Shown at once; put back if the server says no.
  const was = new Set(mine.favorites)
  const next = new Set(was)
  for (const id of ids) {
    if (on) next.add(id)
    else next.delete(id)
  }
  mine.favorites = next
  try {
    mine.favorites = new Set((await api.setFavorites(on ? { add: ids } : { remove: ids })).assets)
  } catch (e) {
    mine.favorites = was
    showError(on ? t('projects-favorite-add-failed', { error: (e as Error).message }) : t('projects-favorite-remove-failed', { error: (e as Error).message }))
  }
}

export function toggleFavorite(id: string): Promise<void> {
  return setFavorite([id], !mine.favorites.has(id))
}

export async function saveSearch(name: string, query: string): Promise<boolean> {
  try {
    mine.searches = (await api.saveSearch(name, query)).searches
    return true
  } catch (e) {
    showError(t('projects-save-search-failed', { error: (e as Error).message }))
    return false
  }
}

/** Rename a saved search, change what it searches, or make it (or stop it being) what the library opens to. */
export async function changeSearch(s: SavedSearch, change: { name?: string; query?: string; default?: boolean }): Promise<boolean> {
  try {
    mine.searches = (await api.changeSearch(s.id, change)).searches
    return true
  } catch (e) {
    showError(t('projects-change-search-failed', { name: s.name, error: (e as Error).message }))
    return false
  }
}

export async function forgetSearch(s: SavedSearch): Promise<void> {
  try {
    await api.deleteSearch(s.id)
    mine.searches = mine.searches.filter((x) => x.id !== s.id)
  } catch (e) {
    showError(t('projects-forget-search-failed', { name: s.name, error: (e as Error).message }))
  }
}
