import { readFileSync } from 'node:fs'
import { MEGASCANS, ZIPPED } from './support/seed'
import { expect, test } from './support/test'

function packRow(page: import('@playwright/test').Page, name: string) {
  return page.locator('.packs li', { has: page.locator('.head button.name', { hasText: name }) })
}

test.describe("what a pack's own files say", () => {
  test('a Megascans download is named, described and tagged from its JSON', async ({ api, seed }) => {
    const pack = (await (await api.get(`/api/packs/${seed.packs[MEGASCANS]}`)).json()) as {
      name: string
      description: string | null
      tags: string[]
    }
    expect(pack.name).toBe(MEGASCANS)
    expect(pack.description).toContain('Quixel Megascans')
    expect(pack.description).toContain('Denmark')
    expect(pack.tags).toEqual(expect.arrayContaining(['megascans', 'furniture']))
  })

  test('Download credits gives a CREDITS.md naming the seller and license', async ({ page }) => {
    await page.goto('/#packs')
    await page.getByLabel('Find a pack').fill(MEGASCANS)
    await page.getByLabel(`More actions for ${MEGASCANS}`).click()
    const [download] = await Promise.all([
      page.waitForEvent('download'),
      packRow(page, MEGASCANS).getByRole('link', { name: 'Download credits' }).click(),
    ])
    expect(download.suggestedFilename()).toBe('CREDITS.md')
    const md = readFileSync((await download.path())!, 'utf8')
    expect(md).toContain('# Credits')
    expect(md).toContain(`| ${MEGASCANS} | Quixel Megascans |`)
  })

  test('credits for a pack with a license file say what it asks for', async ({ api, seed }) => {
    const res = await api.get(`/api/packs/${seed.packs[ZIPPED]}/credits?format=md`)
    expect(res.ok()).toBe(true)
    const md = await res.text()
    expect(md).toContain(ZIPPED)
    expect(md).toMatch(/CC0/)
  })

  test("Read the pack's files again asks the server to", async ({ page }) => {
    await page.goto('/#packs')
    await page.getByLabel('Find a pack').fill(MEGASCANS)
    await page.getByLabel(`More actions for ${MEGASCANS}`).click()
    const [res] = await Promise.all([
      page.waitForResponse((r) => r.url().endsWith('/read-files') && r.request().method() === 'POST'),
      packRow(page, MEGASCANS).getByRole('button', { name: "Read the pack's files again" }).click(),
    ])
    expect(res.status()).toBe(202)
    await expect(page.getByRole('alert')).toHaveCount(0)
  })
})
