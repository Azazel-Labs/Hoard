import { checker } from './support/files'
import { uploadPack, waitReady } from './support/api'
import { expect, test, unique } from './support/test'

test.describe('teaching Hoard a store’s file', () => {
  test.setTimeout(120_000)
  test('the wizard writes a format from clicks in the file, shares it and reads the library with it', async ({ page, api }) => {
    const u = unique()
    const file = `wiz-${u}-info.json`
    const store = `Wiz Store ${u}`
    const info = { store: 'Example', item: { title: `Old crate ${u}`, by: 'Someone', tags: ['wood', 'crate'] } }
    // Named as an upload without a name is, so the title the format reads replaces it.
    const pack = await uploadPack(api, `Pack ${new Date().toISOString().slice(0, 10)}`, [
      { relpath: file, data: Buffer.from(JSON.stringify(info, null, 2)) },
      { relpath: 'crate.png', data: checker(32, [120, 80, 40], [200, 160, 90]) },
    ])
    await waitReady(api, [pack.id])

    try {
      await page.goto('/#settings/store-files')
      await page.locator(`[data-file="${file}"]`).getByRole('button', { name: 'Teach Hoard this file' }).click()

      const wizard = page.getByRole('region', { name: 'Format wizard' })
      await wizard.getByPlaceholder('The store’s name, like Kenney').fill(store)
      // The title is armed first.
      await wizard.getByRole('button', { name: `"Old crate ${u}"` }).click()
      await wizard.locator('.fields li', { hasText: 'Seller' }).getByRole('button', { name: 'Pick' }).click()
      await wizard.getByRole('button', { name: '"Someone"' }).click()
      await wizard.locator('.fields li', { hasText: 'Tags' }).getByRole('button', { name: 'Pick' }).click()
      await wizard.getByRole('button', { name: '"wood"' }).click()

      await expect(wizard.getByLabel('Title selector')).toHaveValue('item.title')
      await expect(wizard.getByLabel('Tags selector')).toHaveValue('item.tags')
      await expect(wizard.locator('[data-read="title"]')).toHaveText(`Old crate ${u}`)
      await expect(wizard.locator('[data-read="seller"]')).toHaveText('Someone')
      await expect(wizard.locator('[data-read="tags"]')).toContainText('wood')

      await wizard.getByRole('button', { name: 'Share it' }).click()
      const [download] = await Promise.all([
        page.waitForEvent('download'),
        wizard.getByRole('button', { name: /^Download / }).click(),
      ])
      expect(download.suggestedFilename()).toBe(`hoard-format-wiz-store-${u}.zip`)

      await wizard.getByRole('button', { name: 'Use it here' }).click()
      await expect(wizard).toBeHidden()
      await expect(page.locator(`[data-format="${store}"]`)).toContainText('Yours')

      // The packs with the file are read again: the title names the pack, which had a name nobody chose.
      await expect
        .poll(async () => ((await (await api.get(`/api/packs/${pack.id}`)).json()) as { name: string }).name, { timeout: 60_000 })
        .toBe(`Old crate ${u}`)
      const read = (await (await api.get(`/api/packs/${pack.id}`)).json()) as { description: string; tags: string[] }
      expect(read.description).toContain('Someone')
      expect(read.tags).toEqual(expect.arrayContaining(['wood', 'crate']))
    } finally {
      await api.delete(`/api/admin/store-formats/wiz-store-${u}.toml`)
    }
  })
})
