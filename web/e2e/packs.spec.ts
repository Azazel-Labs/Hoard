import type { Page } from '@playwright/test'
import { uploadPack, waitReady, type Pack } from './support/api'
import { checker } from './support/files'
import { BIG, MODELS, STORE, TEXTURES, ZIPPED } from './support/seed'
import { expect, test, unique } from './support/test'

const SEEDED = [BIG, MODELS, TEXTURES, ZIPPED]

/** The names of the pack rows on the page, top to bottom. */
function packNames(page: Page) {
  return page.locator('.packs li .head button.name')
}

async function seededOrder(page: Page): Promise<string[]> {
  return (await packNames(page).allTextContents()).map((s) => s.trim()).filter((n) => SEEDED.includes(n))
}

test.describe('packs page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/#packs')
    await expect(page.getByLabel('Find a pack')).toBeVisible()
    await expect(packNames(page).filter({ hasText: ZIPPED })).toBeVisible()
  })

  test('finds packs by words across name, store and license', async ({ page, api, seed }) => {
    const find = page.getByLabel('Find a pack')
    await find.fill('e2e zipped')
    await expect(packNames(page)).toHaveText([ZIPPED])
    await expect(page.getByText(/^1 of \d+$/)).toBeVisible()

    // The store name counts too, and every word has to match.
    await find.fill(STORE.toLowerCase())
    await expect(packNames(page)).toHaveText([ZIPPED])
    await find.fill(`${STORE} textures`)
    await expect(page.getByText('No packs match.')).toBeVisible()
    await page.getByRole('button', { name: 'Clear the search and filters' }).click()
    await expect(find).toHaveValue('')
    await expect(packNames(page).filter({ hasText: TEXTURES })).toBeVisible()

    // The license found in the zip is searchable.
    const zipped = ((await (await api.get(`/api/packs/${seed.packs[ZIPPED]}`)).json()) as { license: string | null }).license
    expect(zipped, 'license read from LICENSE.txt').toBeTruthy()
    await find.fill(zipped!.split(/\s+/)[0])
    await expect(packNames(page).filter({ hasText: ZIPPED })).toBeVisible()
    await expect(packNames(page).filter({ hasText: BIG })).toHaveCount(0)
  })

  test('kind chips, status and store filters narrow the list; Clear resets them', async ({ page }) => {
    const kinds = page.getByRole('group', { name: 'Packs with' })
    await kinds.getByRole('button', { name: 'Models' }).click()
    await expect(kinds.getByRole('button', { name: 'Models' })).toHaveAttribute('aria-pressed', 'true')
    await expect(packNames(page).filter({ hasText: MODELS })).toBeVisible()
    await expect(packNames(page).filter({ hasText: TEXTURES })).toHaveCount(0)
    await expect(packNames(page).filter({ hasText: BIG })).toHaveCount(0)
    await kinds.getByRole('button', { name: 'Models' }).click()
    await expect(packNames(page).filter({ hasText: TEXTURES })).toBeVisible()

    await page.getByLabel('Store').selectOption({ label: `From ${STORE}` })
    await expect(packNames(page)).toHaveText([ZIPPED])
    await page.getByLabel('Store').selectOption({ label: 'No store page' })
    await expect(packNames(page).filter({ hasText: TEXTURES })).toBeVisible()
    await page.getByLabel('Store').selectOption({ label: 'Any store' })

    await page.getByLabel('Status').selectOption({ label: 'Failed' })
    await expect(page.getByText('No packs match.')).toBeVisible()
    await page.getByRole('button', { name: 'Clear', exact: true }).click()
    await expect(page.getByLabel('Status')).toHaveValue('')
    await expect(page.getByText('No packs match.')).toHaveCount(0)
    await expect(packNames(page).filter({ hasText: BIG })).toBeVisible()
  })

  test('sorts by name and by asset count', async ({ page }) => {
    const order = page.getByLabel('Order')
    await order.selectOption({ label: 'By name' })
    await expect.poll(() => seededOrder(page)).toEqual([...SEEDED].sort((a, b) => a.localeCompare(b)))
    await order.selectOption({ label: 'Most assets first' })
    await expect(packNames(page).first()).toHaveText(BIG)
    await order.selectOption({ label: 'Oldest first' })
    // Seeded in this order, before any test made a pack.
    await expect.poll(() => seededOrder(page)).toEqual([TEXTURES, MODELS, ZIPPED, BIG])
  })

  test('renames a pack from its More menu', async ({ page, api }) => {
    const name = `E2E Rename ${unique()}`
    const pack: Pack = await uploadPack(api, name, [{ relpath: 'thing.png', data: checker(8, [1, 2, 3], [4, 5, 6], 2) }])
    await waitReady(api, [pack.id])
    await page.reload()
    await page.getByLabel('Find a pack').fill(name)
    await expect(packNames(page)).toHaveText([name])

    await page.getByLabel(`More actions for ${name}`).click()
    await page.getByRole('button', { name: 'Rename…' }).click()
    const input = page.getByLabel('Pack name')
    await expect(input).toBeFocused()
    const renamed = `${name} renamed`
    await input.fill(renamed)
    await page.getByRole('button', { name: 'Save' }).click()
    await page.getByLabel('Find a pack').fill(renamed)
    await expect(packNames(page)).toHaveText([renamed])
    const saved = (await (await api.get(`/api/packs/${pack.id}`)).json()) as Pack
    expect(saved.name).toBe(renamed)

    // Escape gives up a rename without saving it.
    await page.getByLabel(`More actions for ${renamed}`).click()
    await page.getByRole('button', { name: 'Rename…' }).click()
    await page.getByLabel('Pack name').fill('Not this')
    await page.getByLabel('Pack name').press('Escape')
    await expect(packNames(page)).toHaveText([renamed])
  })
})
