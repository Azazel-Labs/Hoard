import type { Page } from '@playwright/test'
import { uploadPack, waitReady, type Pack } from './support/api'
import { checker } from './support/files'
import { BIG, MODELS, STORE, TEXTURES, ZIPPED } from './support/seed'
import { expect, test, tile, unique } from './support/test'

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

    await page.getByLabel('Store', { exact: true }).selectOption({ label: `From ${STORE}` })
    await expect(packNames(page)).toHaveText([ZIPPED])
    await page.getByLabel('Store', { exact: true }).selectOption({ label: 'No store page' })
    await expect(packNames(page).filter({ hasText: TEXTURES })).toBeVisible()
    await page.getByLabel('Store', { exact: true }).selectOption({ label: 'Any store' })

    // Packs no license was found or noted for: the zip's LICENSE.txt gave it one.
    await page.getByLabel('Status', { exact: true }).selectOption({ label: 'No license noted' })
    await expect(packNames(page).filter({ hasText: TEXTURES })).toBeVisible()
    await expect(packNames(page).filter({ hasText: ZIPPED })).toHaveCount(0)

    await page.getByLabel('Status', { exact: true }).selectOption({ label: 'Failed' })
    await expect(page.getByText('No packs match.')).toBeVisible()
    await page.getByRole('button', { name: 'Clear', exact: true }).click()
    await expect(page.getByLabel('Status', { exact: true })).toHaveValue('')
    await expect(page.getByText('No packs match.')).toHaveCount(0)
    await expect(packNames(page).filter({ hasText: BIG })).toBeVisible()
  })

  test('a pack browsed shows what it is beside its assets; a kind narrows them, and it hides until asked back', async ({ page }) => {
    await packNames(page).filter({ hasText: MODELS }).click()
    const about = page.getByRole('complementary', { name: 'About this pack' })
    await expect(about.getByRole('heading', { name: MODELS })).toBeVisible()
    await expect(tile(page, 'Crate')).toBeVisible()
    await expect(tile(page, 'Notes')).toBeVisible()

    const kinds = about.getByRole('button', { name: /^Models/ })
    await kinds.click()
    await expect(kinds).toHaveAttribute('aria-pressed', 'true')
    await expect(tile(page, 'Notes')).toHaveCount(0)
    await expect(tile(page, 'Crate')).toBeVisible()

    await about.getByRole('button', { name: 'Hide the pack’s details' }).click()
    await expect(about).toHaveCount(0)
    await page.getByRole('button', { name: 'About this pack' }).click()
    await expect(page.getByRole('complementary', { name: 'About this pack' })).toBeVisible()

    // An asset opened takes the panel's place; All packs goes back to the list.
    await tile(page, 'Crate').click()
    await expect(page.getByRole('complementary', { name: 'About this pack' })).toHaveCount(0)
    // Its pack's name takes the asset's place with the pack again.
    await page.getByRole('complementary', { name: 'Asset details' }).getByRole('button', { name: MODELS, exact: true }).click()
    await expect(page.getByRole('complementary', { name: 'About this pack' })).toBeVisible()
    await tile(page, 'Crate').click()
    await expect(page.getByRole('complementary', { name: 'About this pack' })).toHaveCount(0)
    // Picking the pack from Packs with its asset open shows the pack, too.
    await page.locator('header.top').getByRole('button', { name: /^Packs/ }).click()
    await packNames(page).filter({ hasText: MODELS }).click()
    await expect(page.getByRole('complementary', { name: 'About this pack' })).toBeVisible()
    await tile(page, 'Crate').click()
    await page.keyboard.press('Escape')
    await page.getByRole('complementary', { name: 'About this pack' }).getByRole('button', { name: 'All packs' }).click()
    await expect(page.getByLabel('Find a pack')).toBeVisible()
  })

  test('sorts by what is chosen, turned round with the button beside it', async ({ page }) => {
    const order = page.getByLabel('Order')
    const way = page.locator('.finder .sort-way')
    const byName = [...SEEDED].sort((a, b) => a.localeCompare(b))
    await order.selectOption({ label: 'Name' })
    await expect(way).toHaveAttribute('aria-label', 'A to Z')
    await expect.poll(() => seededOrder(page)).toEqual(byName)
    await way.click()
    await expect(way).toHaveAttribute('aria-label', 'Z to A')
    await expect.poll(() => seededOrder(page)).toEqual([...byName].reverse())
    // Another order starts its usual way round.
    await order.selectOption({ label: 'Asset count' })
    await expect(way).toHaveAttribute('aria-label', 'Most assets first')
    await expect(packNames(page).first()).toHaveText(BIG)
    await order.selectOption({ label: 'Created' })
    await expect(way).toHaveAttribute('aria-label', 'Newest first')
    await way.click()
    await expect(way).toHaveAttribute('aria-label', 'Oldest first')
    // Seeded in this order, before any test made a pack.
    await expect.poll(() => seededOrder(page)).toEqual([TEXTURES, MODELS, ZIPPED, BIG])
  })

  test('store details: a match found in the stores fills the form, and Save keeps it', async ({ page, api }) => {
    const name = `E2E Store ${unique()}`
    const pack: Pack = await uploadPack(api, name, [{ relpath: 'thing.png', data: checker(8, [1, 2, 3], [4, 5, 6], 2) }])
    await waitReady(api, [pack.id])
    // The stores themselves aren't asked: the lookup answers with one match.
    await page.route(`**/api/packs/${pack.id}/store-matches*`, (route) =>
      route.fulfill({
        json: {
          searched_at: new Date().toISOString(),
          query: name,
          also: [],
          clues: [],
          matches: [{ store: 'Example Store', title: `${name} on sale`, url: 'https://store.example/thing', author: 'Some Seller', license: 'CC0', score: 0.9 }],
          links: [],
          failed: [],
        },
      }),
    )
    await page.reload()
    await page.getByLabel('Find a pack').fill(name)
    await expect(packNames(page)).toHaveText([name])

    await page.getByRole('button', { name: 'Store details' }).click()
    await expect(page.getByLabel('Search the stores for')).toHaveValue(name)
    await page.getByRole('button', { name: 'Use', exact: true }).click()
    await expect(page.getByLabel('Seller')).toHaveValue('Some Seller')
    await page.getByRole('button', { name: 'Save', exact: true }).click()
    await expect(page.getByLabel('Seller')).toHaveCount(0)

    const ctx = (await (await api.get(`/api/packs/${pack.id}/context`)).json()) as { store: string; store_url: string; seller: string }
    expect(ctx).toMatchObject({ store: 'Example Store', store_url: 'https://store.example/thing', seller: 'Some Seller' })
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
