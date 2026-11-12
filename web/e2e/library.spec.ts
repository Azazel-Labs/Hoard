import { BIG, BIG_COUNT, TEXTURES } from './support/seed'
import { assetCount, expect, openPack, test, tile } from './support/test'

test.describe('library grid', () => {
  test('loads the library and shows tiles with their counts', async ({ page, seed }) => {
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    await expect(assetCount(page)).toHaveText('4 assets')
    await expect(page.locator('.grid button.hit')).toHaveCount(4)
    await expect(tile(page, 'Brick wall')).toHaveAttribute('aria-label', /^Brick wall, textures.*, 32×32$/)
    // The pack filter shows as a removable chip.
    await expect(page.getByText(`pack:${TEXTURES}`)).toBeVisible()
  })

  test('infinite scroll fetches more pages and keeps only nearby rows in the DOM', async ({ page, seed }) => {
    // Hundreds of thumbnails on a busy machine: give it room.
    test.slow()
    const offsets: number[] = []
    page.on('request', (r) => {
      const u = new URL(r.url())
      if (u.pathname === '/api/assets' && u.searchParams.get('pack') === seed.packs[BIG]) {
        offsets.push(Number(u.searchParams.get('offset') ?? 0))
      }
    })
    await openPack(page, seed.packs[BIG], BIG, '&sort=name')
    await expect(assetCount(page)).toHaveText(`${BIG_COUNT} assets`)
    await expect(tile(page, 'Img 000')).toBeVisible()
    const firstPage = await page.locator('.grid button.hit').count()
    expect(firstPage).toBeLessThan(BIG_COUNT)

    // Keep scrolling until the last image is in; each step lets the sentinel ask for the next page.
    await tile(page, 'Img 000').hover()
    await expect
      .poll(
        async () => {
          await page.mouse.wheel(0, 2500)
          return tile(page, 'Img 299').count()
        },
        { timeout: 60_000, intervals: [100] },
      )
      .toBe(1)

    expect(new Set(offsets.filter((o) => o > 0)).size, 'pages fetched past the first').toBeGreaterThanOrEqual(3)
    // The grid is virtual: the first rows are gone and far fewer than all 300 tiles exist.
    await expect(tile(page, 'Img 000')).toHaveCount(0)
    expect(await page.locator('.grid button.hit').count()).toBeLessThan(BIG_COUNT / 2)
  })

  test('arrow keys and j/k move through tiles, Enter opens, Escape closes', async ({ page, seed }) => {
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    const first = tile(page, 'Brick wall')
    await first.focus()
    await page.keyboard.press('ArrowRight')
    await expect(tile(page, 'Grass ground')).toBeFocused()
    await page.keyboard.press('j')
    await expect(tile(page, 'Stone tile')).toBeFocused()
    await page.keyboard.press('k')
    await expect(tile(page, 'Grass ground')).toBeFocused()
    await page.keyboard.press('ArrowLeft')
    await expect(first).toBeFocused()

    await page.keyboard.press('Enter')
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await expect(panel.getByRole('heading', { name: 'Brick wall' })).toBeVisible()
    // With the panel open, moving on shows the next asset in it.
    await first.focus()
    await page.keyboard.press('ArrowRight')
    await expect(panel.getByRole('heading', { name: 'Grass ground' })).toBeVisible()

    await page.keyboard.press('Escape')
    await expect(panel).toHaveCount(0)
    await expect(tile(page, 'Grass ground')).toBeFocused()

    // "/" jumps to the search box.
    await page.keyboard.press('/')
    await expect(page.getByLabel('Search assets')).toBeFocused()
  })

  test('selecting a tile opens the side panel with its details', async ({ page, seed }) => {
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    await tile(page, 'Stone tile').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await expect(panel.getByRole('heading', { name: 'Stone tile' })).toBeVisible()
    await expect(page).toHaveURL(new RegExp(`#asset=${seed.assets[`${TEXTURES}/Stone tile`]}`))
    await expect(panel.getByLabel('Add a tag')).toBeVisible()
    await panel.getByRole('button', { name: 'Next asset' }).click()
    await expect(panel.getByRole('heading', { name: 'Water surface' })).toBeVisible()
    await panel.getByRole('button', { name: 'Previous asset' }).click()
    await expect(panel.getByRole('heading', { name: 'Stone tile' })).toBeVisible()
    await panel.getByRole('button', { name: 'Close details' }).click()
    await expect(panel).toHaveCount(0)
  })

  test('search finds assets by words', async ({ page }) => {
    await page.goto('/')
    const search = page.getByLabel('Search assets')
    await search.fill('water surface')
    await expect(tile(page, 'Water surface')).toBeVisible()
    await expect(tile(page, 'Brick wall')).toHaveCount(0)
    await expect(page).toHaveURL(/[?&]q=water/)
  })
})
