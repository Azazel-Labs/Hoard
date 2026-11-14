import { devices, type APIRequestContext, type Page } from '@playwright/test'
import { BIG, TEXTURES } from './support/seed'
import { expect, openPack, test, tile } from './support/test'

/** The main pages, each with something to wait for so the page has really drawn. */
const PAGES: { name: string; path: string; ready: (page: Page) => ReturnType<Page['getByRole']> }[] = [
  { name: 'library', path: '/', ready: (p) => p.locator('.grid button.hit').first() },
  { name: 'packs', path: '/#packs', ready: (p) => p.getByLabel('Find a pack') },
  { name: 'settings: appearance', path: '/#settings/appearance', ready: (p) => p.getByRole('heading', { name: 'Appearance', level: 2 }) },
  { name: 'settings: library', path: '/#settings/library', ready: (p) => p.getByRole('heading', { name: 'Library', level: 2 }) },
  { name: 'settings: tags', path: '/#settings/tags', ready: (p) => p.getByLabel('Find tags') },
  { name: 'settings: account', path: '/#settings/account', ready: (p) => p.getByLabel('What will use it') },
  { name: 'settings: adding packs', path: '/#settings/adding', ready: (p) => p.getByRole('radiogroup', { name: 'Unreal packages' }) },
  { name: 'settings: performance', path: '/#settings/performance', ready: (p) => p.getByLabel('Background workers') },
  { name: 'settings: tools', path: '/#settings/tools', ready: (p) => p.getByLabel('SoundFont file') },
  { name: 'settings: maintenance', path: '/#settings/maintenance', ready: (p) => p.getByRole('heading', { name: 'Updates' }) },
]

/** The page's background (the body's, or the root's behind a see-through body) as luminance, 0 black to 1 white. */
function bodyLuminance(page: Page): Promise<number> {
  return page.evaluate(() => {
    const probe = document.createElement('canvas').getContext('2d', { willReadFrequently: true })!
    const read = (el: Element) => {
      probe.clearRect(0, 0, 1, 1)
      probe.fillStyle = getComputedStyle(el).backgroundColor
      probe.fillRect(0, 0, 1, 1)
      return probe.getImageData(0, 0, 1, 1).data
    }
    let px = read(document.body)
    if (px[3] === 0) px = read(document.documentElement)
    if (px[3] === 0) return -1
    const [r, g, b] = px
    const lin = (c: number) => ((c /= 255) <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4)
    return 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
  })
}

/** The errors left once thumbnails that failed because their asset is gone are set aside: other tests delete packs
 * and read them again on the same server, and a page listing one of their assets can ask for its thumbnail just after. */
async function realErrors(api: APIRequestContext, errors: string[]): Promise<string[]> {
  const kept: string[] = []
  for (const e of errors) {
    const id = /\/api\/assets\/([^/?)]+)\/thumb/.exec(e)?.[1]
    if (id && (await api.get(`/api/assets/${id}`)).status() === 404) continue
    kept.push(e)
  }
  return kept
}

async function visitAll(
  page: Page,
  api: APIRequestContext,
  consoleErrors: string[],
  check: (name: string) => Promise<void>,
): Promise<void> {
  for (const p of PAGES) {
    await test.step(p.name, async () => {
      await page.goto(p.path)
      await expect(p.ready(page)).toBeVisible()
      await check(p.name)
    })
  }
  expect(await realErrors(api, consoleErrors), 'console errors').toEqual([])
}

test.describe('phone width (390px)', () => {
  test.use({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true, userAgent: devices['iPhone 13'].userAgent })

  test('main pages fit the screen without sideways scrolling or errors', async ({ page, api, consoleErrors }) => {
    await visitAll(page, api, consoleErrors, async (name) => {
      const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)
      expect(overflow, `${name} scrolls sideways`).toBeLessThanOrEqual(0)
    })
  })

  test('filters open as a sheet and a tile opens its details', async ({ page, seed, consoleErrors }) => {
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    const toggle = page.getByRole('button', { name: 'Filters' })
    await toggle.click()
    await expect(toggle).toHaveAttribute('aria-expanded', 'true')
    await expect(page.getByRole('complementary', { name: 'Filters' }).getByRole('heading', { name: 'Kind' })).toBeVisible()
    await page.getByRole('complementary', { name: 'Filters' }).getByRole('button', { name: 'Done' }).click()
    await expect(toggle).toHaveAttribute('aria-expanded', 'false')

    await tile(page, 'Brick wall').tap()
    await expect(page.getByRole('complementary', { name: 'Asset details' }).getByRole('heading', { name: 'Brick wall' })).toBeVisible()
    expect(consoleErrors).toEqual([])
  })
})

test.describe('dark mode', () => {
  test.use({ colorScheme: 'dark' })

  test('main pages follow the system dark scheme without errors', async ({ page, api, consoleErrors }) => {
    await visitAll(page, api, consoleErrors, async (name) => {
      await expect(page.locator('html')).toHaveAttribute('data-scheme', 'dark')
      const lum = await bodyLuminance(page)
      expect(lum, `${name} background`).toBeGreaterThanOrEqual(0)
      expect(lum, `${name} background`).toBeLessThan(0.2)
    })
  })

  test('the theme buttons switch between light, dark and the system', async ({ page }) => {
    await page.goto('/#packs')
    const theme = page.getByRole('group', { name: 'Theme' })
    const html = page.locator('html')
    await expect(html).toHaveAttribute('data-scheme', 'dark')
    await theme.getByRole('button', { name: 'Light theme' }).click()
    await expect(html).toHaveAttribute('data-scheme', 'light')
    await expect.poll(() => bodyLuminance(page)).toBeGreaterThan(0.5)
    await theme.getByRole('button', { name: 'Match system theme' }).click()
    await expect(html).toHaveAttribute('data-scheme', 'dark')
    await expect(theme.getByRole('button', { name: 'Match system theme' })).toHaveAttribute('aria-pressed', 'true')
  })
})

test.describe('detail panel', () => {
  for (const width of [1280, 1600, 1920]) {
    test(`opening it at ${width}px leaves the tiles their size; the grid drops a column instead`, async ({ page, seed }) => {
      await page.setViewportSize({ width, height: 900 })
      await openPack(page, seed.packs[BIG], BIG)
      const tiles = page.locator('.grid button.hit')
      await expect(tiles.first()).toBeVisible()
      const before = (await tiles.first().boundingBox())!.width
      await tiles.first().click()
      await expect(page.getByRole('complementary', { name: 'Asset details' })).toBeVisible()
      // The panel slides in; wait for the layout to settle before measuring.
      await expect.poll(async () => Math.abs((await tiles.first().boundingBox())!.width - before)).toBeLessThan(0.5)
      // Nothing scrolls sideways.
      expect(await page.locator('.main').evaluate((m) => m.scrollWidth - m.clientWidth)).toBeLessThanOrEqual(0)
    })
  }
})
