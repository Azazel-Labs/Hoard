import { request } from '@playwright/test'
import { assetsOf } from './support/api'
import { MODELS } from './support/seed'
import { expect, openPack, test, tile, unique } from './support/test'

test.describe('favourites and saved searches', () => {
  test('are each person’s own, and a token sees its maker’s', async ({ api, baseURL, seed }) => {
    const [asset] = await assetsOf(api, seed.packs[MODELS])
    try {
      const starred = (await (await api.post('/api/me/favourites', { data: { add: [asset.id, 'no-such-asset'] } })).json()) as { assets: string[] }
      expect(starred.assets).toContain(asset.id)
      expect(starred.assets).not.toContain('no-such-asset')
      const found = (await (await api.get('/api/assets?favourites=true&limit=500')).json()) as { items: { id: string }[] }
      expect(found.items.map((a) => a.id)).toContain(asset.id)

      const { token } = (await (await api.post('/api/admin/tokens', { data: { name: `fav-${unique()}`, role: 'view' } })).json()) as { token: string }
      const bearer = await request.newContext({ baseURL, extraHTTPHeaders: { Authorization: `Bearer ${token}` } })
      expect(((await (await bearer.get('/api/me/favourites')).json()) as { assets: string[] }).assets).toContain(asset.id)
      await bearer.dispose()

      // A guest has none to ask for.
      const guest = await request.newContext({ baseURL, storageState: { cookies: [], origins: [] } })
      expect((await guest.get('/api/assets?favourites=true')).status()).toBe(401)
      await guest.dispose()

      const name = `Saved ${unique()}`
      const saved = (await (await api.post('/api/me/searches', { data: { name, query: '?q=crate&kind=model' } })).json()) as { searches: { id: number; name: string; query: string }[] }
      const mine = saved.searches.find((s) => s.name === name)!
      expect(mine.query).toBe('q=crate&kind=model')
      // The same name again replaces it.
      await api.post('/api/me/searches', { data: { name, query: 'q=barrel' } })
      const again = ((await (await api.get('/api/me/searches')).json()) as { searches: { name: string; query: string }[] }).searches.filter((s) => s.name === name)
      expect(again).toEqual([expect.objectContaining({ query: 'q=barrel' })])
      expect((await api.delete(`/api/me/searches/${mine.id}`)).status()).toBe(204)
    } finally {
      await api.post('/api/me/favourites', { data: { remove: [asset.id] } })
    }
  })

  test('a star on a tile, Favourites in the filters, and a search kept under Yours', async ({ page, api, seed }) => {
    const crates = (await assetsOf(api, seed.packs[MODELS])).filter((a) => a.name === 'Crate').map((a) => a.id)
    const name = `Crates ${unique()}`
    try {
      await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
      const crateTile = tile(page, 'Crate').first()
      await crateTile.hover()
      const star = page.getByRole('button', { name: 'Favourite Crate' }).first()
      await star.click()
      await expect(star).toHaveAttribute('aria-pressed', 'true')
      await expect
        .poll(async () => ((await (await api.get('/api/me/favourites')).json()) as { assets: string[] }).assets.some((id) => crates.includes(id)))
        .toBe(true)

      const yours = page.getByRole('complementary', { name: 'Filters' }).locator('.yours')
      await yours.getByRole('button', { name: /^Favourites/ }).click()
      await expect(page).toHaveURL(/fav=1/)
      await expect(page.locator('.grid button.hit')).toHaveCount(1)
      await expect(crateTile).toBeVisible()

      // Keep a search, leave it, and come back to it.
      await page.getByLabel('Search assets').fill('crate')
      await expect(page).toHaveURL(/q=crate/)
      await page.getByRole('button', { name: 'Save search' }).click()
      await page.getByLabel('Name for this search').fill(name)
      await page.getByRole('button', { name: 'Save', exact: true }).click()
      const saved = yours.getByRole('button', { name, exact: true })
      await expect(saved).toHaveAttribute('aria-pressed', 'true')
      await expect(page.getByRole('button', { name: 'Save search' })).toHaveCount(0)
      const toast = page.getByRole('status').filter({ hasText: `Saved “${name}”` })
      await toast.getByRole('button', { name: 'Dismiss' }).click()

      await page.getByRole('button', { name: 'Clear all' }).click()
      await expect(saved).toHaveAttribute('aria-pressed', 'false')
      await saved.click()
      await expect(page.getByLabel('Search assets')).toHaveValue('crate')
      await expect(page).toHaveURL(/fav=1/)
      await expect(page.locator('.grid button.hit')).toHaveCount(1)

      // F on the open asset takes it out again.
      await crateTile.click()
      const detailStar = page.getByRole('complementary', { name: 'Asset details' }).getByRole('button', { name: 'Favourite' })
      await expect(detailStar).toHaveAttribute('aria-pressed', 'true')
      await page.keyboard.press('f')
      await expect(detailStar).toHaveAttribute('aria-pressed', 'false')

      await yours.getByRole('button', { name: `Forget the saved search ${name}` }).click()
      await expect(saved).toHaveCount(0)
    } finally {
      await api.post('/api/me/favourites', { data: { remove: crates } })
      const left = ((await (await api.get('/api/me/searches')).json()) as { searches: { id: number; name: string }[] }).searches
      for (const s of left.filter((s) => s.name === name)) await api.delete(`/api/me/searches/${s.id}`)
    }
  })
})
