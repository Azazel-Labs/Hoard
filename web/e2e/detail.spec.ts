import { crate } from './support/files'
import { assetsOf, uploadPack, waitReady } from './support/api'
import { expect, openPack, test, tile, unique } from './support/test'

test.describe('Asset details', () => {
  test('the name is edited where it is: same place and size, Enter saves, Escape gives up', async ({ page, api }) => {
    const name = `E2E Rename ${unique()}`
    const { obj, mtl, texture } = crate()
    const pack = await uploadPack(api, name, [
      { relpath: 'crate.obj', data: obj },
      { relpath: 'crate.mtl', data: mtl },
      { relpath: 'crate_diffuse.png', data: texture },
    ])
    await waitReady(api, [pack.id])
    await openPack(page, pack.id, name, '&sort=name')
    await tile(page, 'Crate').first().click()

    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const heading = panel.getByRole('heading', { level: 2 })
    await expect(heading).toHaveText('Crate')
    const before = await heading.boundingBox()

    await heading.click()
    await expect(heading).toHaveAttribute('contenteditable', 'plaintext-only')
    await expect(heading).toBeFocused()
    expect(await heading.boundingBox()).toEqual(before)

    // Escape gives up without closing the panel.
    await page.keyboard.press('End')
    await page.keyboard.type(' nope')
    await page.keyboard.press('Escape')
    await expect(heading).toHaveText('Crate')
    await expect(heading).not.toHaveAttribute('contenteditable')
    await expect(panel).toBeVisible()

    await heading.click()
    await page.keyboard.press('End')
    await page.keyboard.type(' (wood)')
    await page.keyboard.press('Enter')
    await expect(heading).toHaveText('Crate (wood)')
    await expect(heading).not.toHaveAttribute('contenteditable')
    await expect
      .poll(async () => (await assetsOf(api, pack.id)).some((a) => a.name === 'Crate (wood)'))
      .toBe(true)
  })
})
