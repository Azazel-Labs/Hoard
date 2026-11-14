import { request } from '@playwright/test'
import { assetsOf } from './support/api'
import { MODELS } from './support/seed'
import { expect, openPack, test, tile, unique } from './support/test'

test.describe('where assets are used', () => {
  test('a sync reports what its project has, which search and usage answer from', async ({ api, baseURL, seed }) => {
    const assets = (await assetsOf(api, seed.packs[MODELS])).slice(0, 2)
    const name = `Game ${unique()}`
    const path = `/api/projects/${encodeURIComponent(name)}`
    try {
      // A token that can only look is enough to report.
      const { token } = (await (await api.post('/api/admin/tokens', { data: { name: `sync-${unique()}`, role: 'view' } })).json()) as { token: string }
      const bearer = await request.newContext({ baseURL, extraHTTPHeaders: { Authorization: `Bearer ${token}` } })
      const files = [...assets.map((a, i) => ({ id: a.id, path: `props/${i}-${a.name}` })), { id: 'gone', path: 'old.png' }]
      const res = await bearer.put(path, { data: { assets: files } })
      expect(res.status()).toBe(200)
      expect(await res.json()).toMatchObject({ assets: 2, unknown: 1 })
      await bearer.dispose()

      const guest = await request.newContext({ baseURL, storageState: { cookies: [], origins: [] } })
      expect((await guest.put(path, { data: { assets: [] } })).status()).toBe(401)
      await guest.dispose()

      const listed = ((await (await api.get('/api/projects')).json()) as { projects: { name: string; assets: number; synced_by: string }[] }).projects
      expect(listed.find((p) => p.name === name)).toMatchObject({ assets: 2, synced_by: expect.stringContaining('sync-') })

      const found = (await (await api.get(`/api/assets?project=${encodeURIComponent(name)}&limit=50`)).json()) as { items: { id: string }[] }
      expect(found.items.map((a) => a.id).sort()).toEqual(assets.map((a) => a.id).sort())

      const usage = (await (await api.post('/api/usage', { data: { packs: [seed.packs[MODELS]] } })).json()) as { projects: { name: string; assets: number; examples: { path: string }[] }[] }
      const mine = usage.projects.find((p) => p.name === name)!
      expect(mine.assets).toBe(2)
      expect(mine.examples.map((e) => e.path)).toEqual(['props/0-' + assets[0]!.name, 'props/1-' + assets[1]!.name])

      // Each report replaces the last.
      await api.put(path, { data: { assets: [{ id: assets[1]!.id, path: 'x' }] } })
      const again = (await (await api.post('/api/usage', { data: { assets: [assets[0]!.id] } })).json()) as { projects: { name: string }[] }
      expect(again.projects.map((p) => p.name)).not.toContain(name)
    } finally {
      expect((await api.delete(path)).status()).toBe(204)
    }
  })

  test('an asset’s details list its projects, which filter the library, and archiving it asks first', async ({ page, api, seed }) => {
    // crate.obj and crate.mtl are both called Crate; the project has both.
    const crates = (await assetsOf(api, seed.packs[MODELS])).filter((a) => a.name === 'Crate')
    const name = `Level ${unique()}`
    await api.put(`/api/projects/${encodeURIComponent(name)}`, { data: { assets: crates.map((c) => ({ id: c.id, path: 'props/crate.obj' })) } })
    try {
      await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
      await tile(page, 'Crate').first().click()
      const detail = page.getByRole('complementary', { name: 'Asset details' })
      const used = detail.locator('section.used')
      await expect(used).toContainText(name)
      await expect(used).toContainText('props/crate.obj')

      // Archiving asks, and Cancel leaves it be.
      await detail.getByRole('button', { name: 'Archive', exact: true }).click()
      const dialog = page.getByRole('dialog')
      await expect(dialog).toContainText(`${name} uses this`)
      await expect(dialog).toContainText('props/crate.obj')
      await dialog.getByRole('button', { name: 'Cancel' }).click()
      await expect(detail.getByRole('button', { name: 'Archive', exact: true })).toBeVisible()
      for (const c of crates) expect(((await (await api.get(`/api/assets/${c.id}`)).json()) as { archived_at?: string | null }).archived_at ?? null).toBeNull()

      await used.getByRole('button', { name }).click()
      await expect(page).toHaveURL(new RegExp(`project=${encodeURIComponent(name).replace(/%20/g, '\\+')}`))
      // One tile: crate.mtl, the other Crate, is a supporting file the grid leaves out.
      await expect(page.locator('.grid button.hit')).toHaveCount(1)
      await expect(tile(page, 'Crate').first()).toBeVisible()
      const projects = page.getByRole('complementary', { name: 'Filters' }).locator('.projects')
      await expect(projects.getByRole('button', { name: new RegExp(`^${name}`) })).toHaveAttribute('aria-pressed', 'true')

      // Forgetting it takes it out of the filters.
      await projects.getByRole('button', { name: `Forget the project ${name}` }).click()
      await page.getByRole('dialog').getByRole('button', { name: 'Forget it' }).click()
      await expect(projects.getByRole('button', { name: new RegExp(`^${name}`) })).toHaveCount(0)
    } finally {
      await api.delete(`/api/projects/${encodeURIComponent(name)}`)
    }
  })
})
