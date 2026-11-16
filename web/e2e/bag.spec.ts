import { MODELS } from './support/seed'
import { zipNames } from './support/files'
import { signedIn } from './support/api'
import { expect, openPack, test } from './support/test'

// Picking assets swaps the details panel for the bag: what's picked, how big, and ways to download it all.
test.describe('selection bag', () => {
  test('lists what is picked and downloads the models as GLBs with their textures beside them', async ({
    page,
    seed,
    baseURL,
  }) => {
    test.setTimeout(90_000)
    await openPack(page, seed.packs[MODELS], MODELS)
    for (const name of [/^crate,/i, /^triangle,/i, /^notes,/i]) {
      await page.getByRole('button', { name }).first().click({ modifiers: ['ControlOrMeta'] })
    }
    const bag = page.getByRole('complementary', { name: 'Selected assets' })
    await expect(bag.getByRole('heading', { name: '3 assets' })).toBeVisible()
    await expect(bag.getByText('Models', { exact: false }).first()).toBeVisible()
    // The bar keeps the edits; the downloads moved to the bag.
    const bar = page.getByRole('region', { name: 'Selected assets' })
    await expect(bar.getByRole('link', { name: 'Download zip' })).toHaveCount(0)
    await expect(bag.getByRole('link', { name: 'Download zip' })).toBeVisible()

    // Taking one out.
    await bag.getByRole('button', { name: /^Take notes out of the selection$/i }).click()
    await expect(bag.getByRole('heading', { name: '2 assets' })).toBeVisible()

    // Every model a GLB, the crate's texture once in the shared folder, named as it was.
    const href = await bag.getByRole('link', { name: 'Download GLB zip' }).getAttribute('href')
    expect(href).toContain('glb=1')
    const api = await signedIn(baseURL!)
    const res = await api.get(href!, { timeout: 60_000 })
    expect(res.ok(), `${res.status()} ${res.ok() ? '' : await res.text()}`).toBe(true)
    const names = zipNames(await res.body())
    expect(names.sort()).toEqual([
      'hoard-glb/CREDITS.md',
      'hoard-glb/crate.glb',
      'hoard-glb/textures/crate_diffuse.png',
      'hoard-glb/triangle.glb',
    ])

    // Clearing the selection brings the grid back without a panel.
    await bag.getByRole('button', { name: 'Clear the selection' }).click()
    await expect(bag).toHaveCount(0)
  })
})
