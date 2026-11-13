import { readFileSync } from 'node:fs'
import { MEGASCANS, ZIPPED } from './support/seed'
import { checker, withText } from './support/files'
import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
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

  // An image that says Stable Diffusion made it is tagged so, and a ship check by hash finds it, its pack's unknown
  // license, and a file from a CC0 pack that's clear.
  test('a ship check by file hash says what to credit and which files need a look', async ({ api, seed }) => {
    const made = withText(checker(32, [200, 60, 60], [60, 60, 200]), 'parameters', 'a red tile\nSteps: 20, Sampler: Euler a, Seed: 1')
    const pack = await uploadPack(api, 'E2E Ship Check', [
      { relpath: 'Tiles/Generated.png', data: made },
      { relpath: 'Tiles/Painted.png', data: checker(32, [20, 160, 20], [240, 240, 240]) },
    ])
    await waitReady(api, [pack.id])
    type Full = Asset & { hash: string; path: string; meta: { ai?: { made: string; by?: string } } }
    const full = async (id: string) => (await (await api.get(`/api/assets/${id}`)).json()) as Full
    const mine = await Promise.all((await assetsOf(api, pack.id)).map((a) => full(a.id)))
    const generated = mine.find((a) => a.path.endsWith('Generated.png'))!
    const painted = mine.find((a) => a.path.endsWith('Painted.png'))!
    expect(generated.meta.ai).toEqual({ made: 'generated', by: 'Stable Diffusion', how: 'generator settings' })
    expect(generated.tags.map((t) => t.tag)).toContain('ai-generated')
    expect(painted.meta.ai).toBeUndefined()
    const cc0 = await full((await assetsOf(api, seed.packs[ZIPPED]))[0].id)

    const unknown = 'ab'.repeat(32)
    const res = await api.post('/api/check', { data: { hashes: [generated.hash.toUpperCase(), painted.hash, cc0.hash, unknown] } })
    expect(res.ok()).toBe(true)
    type Check = {
      ok: boolean
      summary: { checked: number; found: number; need_a_look: number; warnings: Record<string, number> }
      files: { hash: string; ok: boolean; assets: { id: string; pack: string; warnings: { code: string }[] }[] }[]
      not_found: string[]
      credits: { packs: { pack: string }[] }
    }
    const check = (await res.json()) as Check
    expect(check.ok).toBe(false)
    expect(check.not_found).toEqual([unknown])
    expect(check.summary).toMatchObject({ checked: 4, found: 3, need_a_look: 2, warnings: { license_unknown: 2, ai_generated: 1 } })
    const file = (hash: string) => check.files.find((f) => f.hash === hash)!
    expect(file(generated.hash).assets[0].warnings.map((w) => w.code).sort()).toEqual(['ai_generated', 'license_unknown'])
    expect(file(cc0.hash).ok).toBe(true)
    expect(check.credits.packs.map((p) => p.pack)).toEqual(expect.arrayContaining(['E2E Ship Check', ZIPPED]))

    const md = await (await api.post('/api/check?format=md', { data: { hashes: [generated.hash] } })).text()
    expect(md).toContain('## Ship check')
    expect(md).toContain('Stable Diffusion')
    await api.delete(`/api/packs/${pack.id}`)
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
