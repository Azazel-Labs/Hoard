import { TEXTURES } from './support/seed'
import { assetCount, expect, openPack, test, tile, unique } from './support/test'

interface Collection {
  id: string
  name: string
  asset_count: number
}

test.describe('collections', () => {
  test('make one from the side panel, add to it, browse it, take one out', async ({ page, api, seed }) => {
    const name = `E2E Coll ${unique()}`
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    const panel = page.getByRole('complementary', { name: 'Asset details' })

    await tile(page, 'Brick wall').click()
    await panel.getByRole('button', { name: 'Add to collection' }).click()
    const picker = page.getByRole('dialog', { name: 'Collections' })
    await picker.getByLabel('Find or create a collection').fill(name)
    await picker.getByRole('button', { name: 'Create' }).click()
    await expect(panel.getByRole('button', { name, exact: true })).toBeVisible()
    await expect(panel.getByRole('button', { name: 'Add to another' })).toBeVisible()

    await tile(page, 'Grass ground').click()
    await expect(panel.getByRole('heading', { name: 'Grass ground' })).toBeVisible()
    await panel.getByRole('button', { name: 'Add to collection' }).click()
    await picker.getByLabel('Find or create a collection').fill(name)
    await picker.getByRole('menuitemcheckbox', { name: new RegExp(name) }).click()
    await expect(panel.getByRole('button', { name, exact: true })).toBeVisible()

    // The sidebar lists it with its count; picking it browses it.
    const side = page.getByRole('complementary', { name: 'Filters' })
    const entry = side.getByRole('button', { name: new RegExp(`^${name}\\s*2$`) })
    await expect(entry).toBeVisible()
    await entry.click()
    await expect(entry).toHaveAttribute('aria-pressed', 'true')
    await expect(page.getByText(`collection:${name}`)).toBeVisible()
    await expect(assetCount(page)).toHaveText('2 assets')
    await expect(page.locator('.grid button.hit')).toHaveCount(2)

    await tile(page, 'Brick wall').click()
    await panel.getByRole('button', { name: `Remove from ${name}` }).click()
    await expect(panel.getByRole('button', { name, exact: true })).toHaveCount(0)
    await expect
      .poll(async () => ((await (await api.get('/api/collections')).json()) as Collection[]).find((c) => c.name === name)?.asset_count)
      .toBe(1)
  })

  test('rename and delete (with undo) from the packs page', async ({ page, api }) => {
    const name = `E2E Coll ${unique()}`
    const made = await api.post('/api/collections', { data: { name } })
    expect(made.ok()).toBeTruthy()
    const { id } = (await made.json()) as Collection

    await page.goto('/#packs')
    const section = page.getByRole('region', { name: 'Collections' })
    const row = section.locator('li').filter({ has: page.getByRole('button', { name, exact: true }) })
    await expect(row).toBeVisible()
    await row.getByRole('button', { name: 'Rename' }).click()
    const renamed = `${name} renamed`
    await section.getByLabel('Collection name').fill(renamed)
    await section.getByRole('button', { name: 'Save' }).click()
    const renamedRow = section.locator('li').filter({ has: page.getByRole('button', { name: renamed, exact: true }) })
    await expect(renamedRow).toBeVisible()
    await expect
      .poll(async () => ((await (await api.get(`/api/collections/${id}`)).json()) as { name: string }).name)
      .toBe(renamed)

    // Delete hides it at once and offers Undo.
    await renamedRow.getByRole('button', { name: 'Delete' }).click()
    const toast = page.getByRole('status').filter({ hasText: `Deleted the collection ${renamed}` })
    await expect(toast).toBeVisible()
    await expect(renamedRow).toHaveCount(0)
    await toast.getByRole('button', { name: 'Undo' }).click()
    await expect(renamedRow).toBeVisible()
    await expect(toast).toHaveCount(0)

    // Deleting again and dismissing the toast deletes it for real.
    await renamedRow.getByRole('button', { name: 'Delete' }).click()
    await toast.getByRole('button', { name: 'Dismiss' }).click()
    await expect.poll(async () => (await api.get(`/api/collections/${id}`)).status()).toBe(404)
  })
})
