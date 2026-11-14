// The `test` every spec uses: the server global setup started as the base URL, the seeded ids, a signed-in API
// context, and a page that fails the test on an uncaught error.
import { test as base, expect, type APIRequestContext, type Locator, type Page } from '@playwright/test'
import { signedIn } from './api'
import type { Seed } from './paths'

interface Fixtures {
  seed: Seed
  api: APIRequestContext
  /** Errors the page logged or threw, for tests that assert there were none. */
  consoleErrors: string[]
}

export const test = base.extend<Fixtures>({
  baseURL: async ({}, use) => {
    const url = process.env.HOARD_E2E_URL
    if (!url) throw new Error('HOARD_E2E_URL is unset: global setup did not run')
    await use(url)
  },
  seed: async ({}, use) => {
    await use(JSON.parse(process.env.HOARD_E2E_SEED ?? '{}') as Seed)
  },
  api: async ({ baseURL }, use) => {
    const api = await signedIn(baseURL!)
    await use(api)
    await api.dispose()
  },
  consoleErrors: async ({}, use) => {
    await use([])
  },
  page: async ({ page, consoleErrors }, use) => {
    page.on('pageerror', (e) => consoleErrors.push(`pageerror: ${e.message}`))
    page.on('console', (m) => {
      // A failed load's message doesn't say what failed; its location does.
      const url = m.location().url
      if (m.type() === 'error') consoleErrors.push(`console: ${m.text()}${url ? ` (${url})` : ''}`)
    })
    await use(page)
    const thrown = consoleErrors.filter((e) => e.startsWith('pageerror:'))
    expect(thrown, 'uncaught errors in the page').toEqual([])
  },
})

export { expect }

/** A unique suffix per test attempt, so retries and parallel workers never collide on names. */
export function unique(): string {
  return `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 6)}`
}

/** The library grid's tile for an asset name. */
export function tile(page: Page, name: string): Locator {
  return page.locator(`.grid button.hit[aria-label^="${name},"]`)
}

/** A real double-click at the middle of an element: the app tells double-clicks apart by position and timing. */
export async function doubleClick(page: Page, target: Locator): Promise<void> {
  await target.scrollIntoViewIfNeeded()
  const box = await target.boundingBox()
  if (!box) throw new Error('no box to double-click')
  await page.mouse.dblclick(box.x + box.width / 2, box.y + box.height / 2)
}

/** Open the library filtered to one pack, as browsing a pack does. */
export async function openPack(page: Page, packId: string, packName: string, extra = ''): Promise<void> {
  const qs = new URLSearchParams({ pack: packId, pack_name: packName })
  await page.goto(`/?${qs}${extra}`)
}

/** The "N assets" count above the grid (the visible copy, not the screen-reader one). */
export function assetCount(page: Page): Locator {
  return page.locator('.bar .count > [aria-hidden="true"]')
}
