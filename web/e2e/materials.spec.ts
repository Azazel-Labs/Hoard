import { MEGASCANS } from './support/seed'
import { MEGASCANS_FOLDER } from './support/files'
import { doubleClick, expect, openPack, test } from './support/test'

// The Megascans pack's four maps make one material, named after the listing like its model.
test.describe('materials', () => {
  test('a texture set shows as one material tile, its maps hidden behind it', async ({ page, seed }) => {
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    const grid = page.locator('.grid button.hit')
    await expect(grid).toHaveCount(2)
    await expect(page.locator(`.grid button.hit[aria-label^="${MEGASCANS},"]`)).toHaveCount(2)
    await expect(page.locator('.grid button.hit[aria-label*=" maps"]')).toHaveCount(1)
    // The maps (and the store's JSON) are there when asked for.
    await expect(page.getByText(/5 supporting files? hidden/)).toBeVisible()
  })

  test('the material opens with a live preview, its maps by role, and a full-screen viewer', async ({ page, seed }) => {
    // A WebGL material on a busy runner: give it room.
    test.slow()
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await page.locator('.grid button.hit[aria-label*=" maps"]').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await expect(panel.getByRole('heading', { name: MEGASCANS })).toBeVisible()
    const maps = panel.locator('section', { has: page.getByRole('heading', { name: 'Maps' }) })
    for (const role of ['Colour', 'Normal', 'Roughness', 'Ambient occlusion']) {
      await expect(maps.getByRole('button', { name: new RegExp(`^${role} `) })).toBeVisible()
    }
    await expect(panel.getByRole('group', { name: 'Shape' })).toBeVisible()

    await panel.getByRole('button', { name: 'Open the full-screen material viewer' }).click()
    const viewer = page.getByRole('dialog', { name: `View the material ${MEGASCANS}` })
    await expect(viewer).toBeVisible()
    const toggles = viewer.getByRole('group', { name: 'Maps' })
    await expect(toggles).toBeVisible({ timeout: 20_000 })
    await expect(viewer.getByText(/Couldn't show this material/)).toHaveCount(0)
    const normal = toggles.getByRole('button', { name: /Normal/ })
    await expect(normal).toHaveAttribute('aria-pressed', 'true')
    await normal.click()
    await expect(normal).toHaveAttribute('aria-pressed', 'false')
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
  })

  test('double-clicking the material tile opens its viewer', async ({ page, seed }) => {
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await doubleClick(page, page.locator('.grid button.hit[aria-label*=" maps"]'))
    await expect(page.getByRole('dialog', { name: `View the material ${MEGASCANS}` })).toBeVisible()
  })

  test('the model shows the material it wears', async ({ page, seed }) => {
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await page.locator('.grid button.hit[aria-label*=" tris"]').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const worn = panel.locator('section', { has: page.getByRole('heading', { name: 'Material', exact: true }) })
    await expect(worn.getByRole('button', { name: new RegExp(MEGASCANS) })).toBeVisible()
    // Its own file name isn't what it's called any more.
    await expect(panel.getByRole('heading', { name: MEGASCANS })).toBeVisible()
    await expect(panel.getByText(MEGASCANS_FOLDER.replaceAll('_', ' '), { exact: true })).toHaveCount(0)
  })
})
