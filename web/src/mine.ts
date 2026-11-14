// What's the signed-in person's own: the assets they've starred and the searches they've saved. Loaded once they're
// signed in (guests have neither) and kept here so every tile and panel shows the same stars.
import { reactive, watch } from 'vue'
import { api, type SavedSearch } from './api'
import { auth } from './auth'
import { showError } from './ui'

export const mine = reactive({
  favourites: new Set<string>(),
  searches: [] as SavedSearch[],
})

/** Starring is for people with an account (and the tokens they make), not guests. */
export function canStar(): boolean {
  return !!auth.status?.signed_in
}

export async function loadMine(): Promise<void> {
  if (!canStar()) {
    mine.favourites = new Set()
    mine.searches = []
    return
  }
  try {
    const [f, s] = await Promise.all([api.favourites(), api.savedSearches()])
    mine.favourites = new Set(f.assets)
    mine.searches = s.searches
  } catch {
    // Not worth an error of its own: the stars just don't show until the next sign-in or reload.
  }
}

watch(() => auth.status?.user?.name ?? null, loadMine, { immediate: true })

/** Star (or with `on` false, unstar) these assets. */
export async function setFavourite(ids: string[], on: boolean): Promise<void> {
  if (!ids.length) return
  // Shown at once; put back if the server says no.
  const was = new Set(mine.favourites)
  const next = new Set(was)
  for (const id of ids) {
    if (on) next.add(id)
    else next.delete(id)
  }
  mine.favourites = next
  try {
    mine.favourites = new Set((await api.setFavourites(on ? { add: ids } : { remove: ids })).assets)
  } catch (e) {
    mine.favourites = was
    showError(`Couldn't ${on ? 'add to' : 'take out of'} your favourites: ${(e as Error).message}`)
  }
}

export function toggleFavourite(id: string): Promise<void> {
  return setFavourite([id], !mine.favourites.has(id))
}

export async function saveSearch(name: string, query: string): Promise<boolean> {
  try {
    mine.searches = (await api.saveSearch(name, query)).searches
    return true
  } catch (e) {
    showError(`Couldn't save the search: ${(e as Error).message}`)
    return false
  }
}

export async function forgetSearch(s: SavedSearch): Promise<void> {
  try {
    await api.deleteSearch(s.id)
    mine.searches = mine.searches.filter((x) => x.id !== s.id)
  } catch (e) {
    showError(`Couldn't remove ${s.name}: ${(e as Error).message}`)
  }
}
