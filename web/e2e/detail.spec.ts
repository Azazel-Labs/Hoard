import { crate } from './support/files'
import { MODELS } from './support/seed'
import { addTags, assetsOf, uploadPack, waitReady } from './support/api'
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

    // A very long tag fades out instead of pushing the panel sideways.
    // (Tags are up to 64 characters.)
    const long = 'note:an-extremely-long-tag-someone-pasted-in-by-mistake-oops'
    const id = (await assetsOf(api, pack.id)).find((a) => a.name === 'Crate (wood)')!.id
    await addTags(api, id, [long])
    await page.reload()
    const chip = panel.locator('.chip.long-ns, .chip.long').first()
    await expect(chip).toHaveAttribute('title', new RegExp(`^${long}`))
    expect((await chip.boundingBox())!.width).toBeLessThan(340)
    expect(await panel.evaluate((el) => el.scrollWidth - el.clientWidth)).toBeLessThanOrEqual(0)
  })

  test('a link in the panel opens the asset at the top; Back returns to the one before, scrolled where it was', async ({ page, seed }) => {
    await page.setViewportSize({ width: 1400, height: 640 })
    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await tile(page, 'Crate').first().click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const heading = panel.getByRole('heading', { level: 2 })
    await expect(heading).toHaveText('Crate')
    const crateUrl = page.url()
    // The crate needs its .mtl, a supporting file (also named Crate).
    const link = panel.getByRole('button', { name: /material file/ })
    await link.scrollIntoViewIfNeeded()
    const down = await panel.evaluate((el) => el.scrollTop)
    expect(down).toBeGreaterThan(50)

    await link.click()
    await expect(page).not.toHaveURL(crateUrl)
    await expect(panel.getByText('crate.mtl').first()).toBeVisible()
    await expect.poll(() => panel.evaluate((el) => el.scrollTop)).toBe(0)

    await page.goBack()
    await expect(page).toHaveURL(crateUrl)
    await expect.poll(() => panel.evaluate((el) => el.scrollTop)).toBeGreaterThan(down - 2)
    await page.goForward()
    await expect(page).not.toHaveURL(crateUrl)
    await expect.poll(() => panel.evaluate((el) => el.scrollTop)).toBe(0)
    await page.goBack()
    await expect(page).toHaveURL(crateUrl)
    await expect(heading).toHaveText('Crate')
    // And Back once more closes the panel, as opening it was the step before.
    await page.goBack()
    await expect(panel).toHaveCount(0)
  })
})
