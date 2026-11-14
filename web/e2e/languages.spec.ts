import { readFileSync } from 'node:fs'
import { FluentBundle, FluentResource, type FluentVariable } from '@fluent/bundle'
import { HOARD_HEADERS } from './support/api'
import { expect, test } from './support/test'

// What a French catalog says, so the tests follow its wording rather than repeat it.
const french = new FluentBundle('fr', { useIsolating: false })
french.addResource(new FluentResource(readFileSync(new URL('../../locales/fr/hoard.ftl', import.meta.url), 'utf8')))
function fr(id: string, args?: Record<string, FluentVariable>): string {
  const message = french.getMessage(id)
  if (!message?.value) throw new Error(`the French catalog has no ${id}`)
  return french.formatPattern(message.value, args)
}

test.describe('languages', () => {
  test('Settings → Appearance switches the portal to another language and back', async ({ page }) => {
    await page.goto('/#settings/appearance')
    await expect(page.getByRole('heading', { name: 'Appearance', level: 2 })).toBeVisible()
    await page.locator('#language').selectOption('fr')
    await expect(page.getByRole('heading', { name: fr('settings-page-appearance'), level: 2 })).toBeVisible()
    await expect(page.locator('html')).toHaveAttribute('lang', 'fr')

    // It's remembered in this browser.
    await page.reload()
    await expect(page.getByRole('heading', { name: fr('settings-page-appearance'), level: 2 })).toBeVisible()

    // The browser's language (English here) again.
    await page.locator('#language').selectOption('auto')
    await expect(page.getByRole('heading', { name: 'Appearance', level: 2 })).toBeVisible()
    await expect(page.locator('html')).toHaveAttribute('lang', 'en')
  })

  test("the server answers in the language a request asks for, and in English when it doesn't ask", async ({ api }) => {
    const missing = 'no-such-asset'
    const asked = await api.get(`/api/assets/${missing}`, { headers: { ...HOARD_HEADERS, 'Accept-Language': 'fr-CA, en;q=0.5' } })
    expect(asked.status()).toBe(404)
    expect(await asked.json()).toEqual({ error: fr('error-asset-not-found', { id: missing }) })

    const plain = await api.get(`/api/assets/${missing}`)
    expect(await plain.json()).toEqual({ error: `asset ${missing} not found` })

    // A language there's no catalog for gets English.
    const other = await api.get(`/api/assets/${missing}`, { headers: { ...HOARD_HEADERS, 'Accept-Language': 'tlh' } })
    expect(await other.json()).toEqual({ error: `asset ${missing} not found` })
  })
})
