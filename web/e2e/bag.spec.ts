import { MODELS } from './support/seed'
import { zipNames } from './support/files'
import { assetsOf, signedIn, tagsOf } from './support/api'
import { expect, openPack, test, unique } from './support/test'

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

  test('the bar tags what is picked, and Undo takes the tag back off', async ({ page, seed, baseURL }) => {
    await openPack(page, seed.packs[MODELS], MODELS)
    for (const name of [/^crate,/i, /^triangle,/i]) {
      await page.getByRole('button', { name }).first().click({ modifiers: ['ControlOrMeta'] })
    }
    const tag = `bulk-${unique()}`
    const bar = page.getByRole('region', { name: 'Selected assets' })
    await bar.getByRole('textbox', { name: /tag to add/i }).fill(tag)
    await bar.getByRole('button', { name: 'Add', exact: true }).click()

    const api = await signedIn(baseURL!)
    const picked = (await assetsOf(api, seed.packs[MODELS])).filter((a) => a.kind === 'model' && /^(crate|triangle)$/i.test(a.name))
    expect(picked).toHaveLength(2)
    for (const a of picked) expect(await tagsOf(api, a.id)).toContain(tag)

    await page.getByRole('button', { name: 'Undo' }).click()
    for (const a of picked) await expect.poll(() => tagsOf(api, a.id)).not.toContain(tag)
  })
})
