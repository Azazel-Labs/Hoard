import { MEGASCANS } from './support/seed'
import { checker, MEGASCANS_FOLDER, quadGltf } from './support/files'
import type { Page } from '@playwright/test'
import { assetsOf, signedIn, uploadPack, waitIdle, waitReady } from './support/api'
import { doubleClick, expect, openPack, test } from './support/test'

// The Megascans pack's four maps make one material, named after the listing like its model. Only the model wears it,
// so it's part of the model: one tile, and the material opens from the model.
test.describe('materials', () => {
  /** Opens the pack, the model, then the material it wears; returns the side panel. */
  async function openMaterial(page: Page, packId: string) {
    await openPack(page, packId, MEGASCANS, '&sort=name')
    await page.locator('.grid button.hit[aria-label*=" tris"]').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const worn = panel.locator('section', { has: page.getByRole('heading', { name: 'Material', exact: true }) })
    await worn.getByRole('button', { name: new RegExp(MEGASCANS) }).click()
    await expect(panel.locator('section', { has: page.getByRole('heading', { name: 'Maps' }) })).toBeVisible()
    return panel
  }

  test('a model and the one material it wears show as one tile, the maps hidden behind it', async ({ page, seed }) => {
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    const grid = page.locator('.grid button.hit')
    await expect(grid).toHaveCount(1)
    await expect(page.locator(`.grid button.hit[aria-label^="${MEGASCANS},"][aria-label*=" tris"]`)).toHaveCount(1)
    // The material, its maps (and the store's JSON) are there when asked for.
    await expect(page.getByText(/6 supporting files hidden/)).toBeVisible()
  })

  test('the material opens with a live preview, its maps by role, and a full-screen viewer', async ({ page, seed }) => {
    // A WebGL material on a busy runner: give it room.
    test.slow()
    const panel = await openMaterial(page, seed.packs[MEGASCANS])
    await expect(panel.getByRole('heading', { name: MEGASCANS })).toBeVisible()
    const maps = panel.locator('section', { has: page.getByRole('heading', { name: 'Maps' }) })
    for (const role of ['Colour', 'Normal', 'Roughness', 'Ambient occlusion']) {
      await expect(maps.getByRole('button', { name: new RegExp(`^${role} `) })).toBeVisible()
    }
    await expect(panel.getByRole('group', { name: 'Shape' })).toBeVisible()

    await panel.getByRole('button', { name: 'Open the full-screen material viewer' }).click()
    const viewer = page.getByRole('dialog', { name: `View the material ${MEGASCANS}` })
    await expect(viewer).toBeVisible()
    const toggles = viewer.getByRole('group', { name: 'Maps' })
    await expect(toggles).toBeVisible({ timeout: 20_000 })
    await expect(viewer.getByText(/Couldn't show this material/)).toHaveCount(0)
    const normal = toggles.getByRole('button', { name: /Normal/ })
    await expect(normal).toHaveAttribute('aria-pressed', 'true')
    await normal.click()
    await expect(normal).toHaveAttribute('aria-pressed', 'false')
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
  })

  test('double-clicking the material tile opens its viewer', async ({ page, seed }) => {
    // The side panel's live preview and the viewer both start WebGL, slow on a runner without a GPU.
    test.slow()
    // Shown as itself once supporting files are.
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await page.locator('.supporting').getByRole('button', { name: 'Show' }).click()
    await doubleClick(page, page.locator('.grid button.hit[aria-label*=" maps"]'))
    await expect(page.getByRole('dialog', { name: `View the material ${MEGASCANS}` })).toBeVisible({ timeout: 45_000 })
  })

  test('the model shows the material it wears', async ({ page, seed }) => {
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await page.locator('.grid button.hit[aria-label*=" tris"]').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const worn = panel.locator('section', { has: page.getByRole('heading', { name: 'Material', exact: true }) })
    await expect(worn.getByRole('button', { name: new RegExp(MEGASCANS) })).toBeVisible()
    // Its own file name isn't what it's called any more.
    await expect(panel.getByRole('heading', { name: MEGASCANS })).toBeVisible()
    await expect(panel.getByText(MEGASCANS_FOLDER.replaceAll('_', ' '), { exact: true })).toHaveCount(0)
  })

  test("the server's thumbnail of the model has the material's colour map on", async ({ seed, baseURL }) => {
    const api = await signedIn(baseURL!)
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    test.skip(!/^(gpu|cpu):/.test(stats.renderer ?? ''), `no model renderer here (${stats.renderer})`)
    const model = (await assetsOf(api, seed.packs[MEGASCANS])).find((a) => a.kind === 'model')!
    const full = (await (await api.get(`/api/assets/${model.id}`)).json()) as { meta: { thumb_dressed?: string } }
    expect(full.meta.thumb_dressed).toBeTruthy()
  })

  test("a glTF model's server thumbnail has the colour map of the material beside it on", async ({ baseURL }) => {
    const api = await signedIn(baseURL!)
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    test.skip(!/^(gpu|cpu):/.test(stats.renderer ?? ''), `no model renderer here (${stats.renderer})`)
    const maps: [string, [number, number, number]][] = [
      ['BaseColor', [150, 40, 30]],
      ['Normal', [128, 128, 255]],
      ['Roughness', [200, 200, 200]],
    ]
    const pack = await uploadPack(api, 'E2E Dressed glTF', [
      { relpath: 'Signpost/Signpost.gltf', data: quadGltf() },
      ...maps.map(([m, c]) => ({ relpath: `Signpost/Signpost_${m}.png`, data: checker(64, c, c, 8) })),
    ])
    await waitReady(api, [pack.id])
    await waitIdle(api)
    const model = (await assetsOf(api, pack.id)).find((a) => a.kind === 'model')!
    const full = (await (await api.get(`/api/assets/${model.id}`)).json()) as { meta: { thumb_dressed?: string } }
    expect(full.meta.thumb_dressed).toBeTruthy()
    await api.delete(`/api/packs/${pack.id}`)
  })
})
