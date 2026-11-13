import { MEGASCANS } from './support/seed'
import { checker, MEGASCANS_FOLDER, quadGltf, zipNames } from './support/files'
import type { Page } from '@playwright/test'
import { type Asset, assetsOf, signedIn, uploadPack, waitIdle, waitReady } from './support/api'
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

  test('maps are called by their material and role, and go out named that way', async ({ page, seed, baseURL }) => {
    const api = await signedIn(baseURL!)
    const assets = await assetsOf(api, seed.packs[MEGASCANS])
    const model = assets.find((a) => a.kind === 'model')!
    const ao = assets.find((a) => a.path.endsWith('_AO.png'))!
    expect(ao.name).toBe(`${MEGASCANS} Ambient occlusion`)
    const stem = 'HistoricMiscellaneousStandWoodOld05'

    // The model's zip: its maps renamed in their folder, the model as it was.
    const names = zipNames(await (await api.get(`/api/assets/${model.id}/bundle.zip`)).body())
    expect(names).toEqual(
      expect.arrayContaining([
        expect.stringMatching(new RegExp(`(^|/)${MEGASCANS_FOLDER}\\.obj$`)),
        ...['BaseColor', 'Normal', 'Roughness', 'AO'].map((m) => expect.stringMatching(new RegExp(`(^|/)${stem}_${m}\\.png$`))),
      ]),
    )
    expect(names.filter((n) => n.includes('_2K_'))).toEqual([])
    // One map on its own, and in the manifest the hoard tool reads.
    const one = await api.get(`/api/assets/${ao.id}/file?download=1`)
    expect(one.headers()['content-disposition']).toContain(`"${stem}_AO.png"`)
    const manifest = (await (await api.post('/api/manifest', { data: { ids: [model.id], include_deps: true } })).json()) as {
      entries: { id: string; bundle_path?: string; file_name?: string }[]
    }
    const entry = manifest.entries.find((e) => e.id === ao.id)!
    expect(entry.file_name).toBe(`${stem}_AO.png`)
    expect(entry.bundle_path).toMatch(new RegExp(`(^|/)${stem}_AO\\.png$`))

    // The model's page lists the maps under the material by what they're for, and not again under Needs.
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await page.locator('.grid button.hit[aria-label*=" tris"]').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const worn = panel.getByRole('list', { name: `${MEGASCANS}: maps` })
    for (const role of ['Colour', 'Normal', 'Roughness', 'Ambient occlusion']) {
      await expect(worn.getByRole('button', { name: new RegExp(`^${role} `) })).toBeVisible()
    }
    const needs = panel.locator('section, div').filter({ has: page.getByRole('heading', { name: 'Needs', exact: true }) }).last()
    await expect(needs.getByRole('button', { name: /Xeokah 0/ })).toBeVisible()
    await expect(needs.getByRole('button', { name: new RegExp(MEGASCANS) })).toHaveCount(0)
  })

  test('Settings choose how maps are named in downloads', async ({ page, seed, baseURL }) => {
    const api = await signedIn(baseURL!)
    const ao = (await assetsOf(api, seed.packs[MEGASCANS])).find((a) => a.path.endsWith('_AO.png'))!
    const fileName = async () => (await api.get(`/api/assets/${ao.id}/file?download=1`)).headers()['content-disposition']
    await page.goto('/#settings/downloads')
    const section = page.getByTestId('texture-names')
    const example = page.getByTestId('texture-example')
    await expect(example).toContainText('StandWoodOld05_BaseColor.jpg')
    try {
      await section.getByText('Short', { exact: true }).click()
      await expect(example).toContainText('T_StandWoodOld05_BC.jpg')
      await expect(example).toContainText('T_RockMoss_BC_4K.png')
      await expect.poll(fileName).toContain('T_HistoricMiscellaneousStandWoodOld05_AO.png')

      // A pattern that can't work is refused before it's saved.
      const pattern = section.getByLabel('Pattern')
      await pattern.fill('{name}')
      await pattern.press('Enter')
      await expect(section.getByRole('alert')).toContainText('{map}')
      await pattern.fill('{map}-{name}')
      await pattern.press('Enter')
      await expect(example).toContainText('BC-StandWoodOld05.jpg')
      await expect.poll(fileName).toContain('AO-HistoricMiscellaneousStandWoodOld05.png')

      await section.getByRole('checkbox', { name: 'Name texture maps by their material' }).uncheck()
      await expect.poll(fileName).toContain(`${MEGASCANS_FOLDER}_2K_AO.png`)
    } finally {
      await api.put('/api/admin/settings', { data: { texture_names: null } })
    }
    await expect.poll(fileName).toContain('HistoricMiscellaneousStandWoodOld05_AO.png')
  })

  test("the server's thumbnail of the model has the material's colour map on", async ({ seed, baseURL }) => {
    const api = await signedIn(baseURL!)
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    test.skip(!/^(gpu|cpu):/.test(stats.renderer ?? ''), `no model renderer here (${stats.renderer})`)
    const model = (await assetsOf(api, seed.packs[MEGASCANS])).find((a) => a.kind === 'model')!
    const full = (await (await api.get(`/api/assets/${model.id}`)).json()) as { meta: { thumb_dressed?: string } }
    expect(full.meta.thumb_dressed).toBeTruthy()
  })

  // Uploaded, it's packed into one GLB (its .bin inside, its maps still beside it), and dressed like any model.
  test("a glTF model's server thumbnail has the colour map of the material beside it on", async ({ baseURL }) => {
    const api = await signedIn(baseURL!)
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    test.skip(!/^(gpu|cpu):/.test(stats.renderer ?? ''), `no model renderer here (${stats.renderer})`)
    const maps: [string, [number, number, number]][] = [
      ['BaseColor', [150, 40, 30]],
      ['Normal', [128, 128, 255]],
      ['Roughness', [200, 200, 200]],
    ]
    const sign = quadGltf('Signpost.bin')
    const pack = await uploadPack(api, 'E2E Dressed glTF', [
      { relpath: 'Signpost/Signpost.gltf', data: sign.gltf },
      { relpath: 'Signpost/Signpost.bin', data: sign.bin },
      ...maps.map(([m, c]) => ({ relpath: `Signpost/Signpost_${m}.png`, data: checker(64, c, c, 8) })),
    ])
    await waitReady(api, [pack.id])
    await waitIdle(api)
    const assets = (await assetsOf(api, pack.id)) as (Asset & { path: string })[]
    expect(assets.map((a) => a.path).filter((p) => /\.(gltf|bin|glb)$/.test(p))).toEqual(['Signpost.glb'])
    expect(assets.filter((a) => a.kind === 'texture')).toHaveLength(3)
    const model = assets.find((a) => a.kind === 'model')!
    const full = (await (await api.get(`/api/assets/${model.id}`)).json()) as { meta: { thumb_dressed?: string } }
    expect(full.meta.thumb_dressed).toBeTruthy()
    await api.delete(`/api/packs/${pack.id}`)
  })
})
