import { TEXTURES } from './support/seed'
import { assetCount, expect, openPack, test, tile } from './support/test'

// Every thumbnail's colours and look are indexed in the background after a pack is ready, so these allow for it.
test.describe('search by look', () => {
  test('a colour from the sidebar puts the assets with most of it first', async ({ page, seed }) => {
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    await expect(assetCount(page)).toHaveText('4 assets')
    await page.getByRole('button', { name: /^Blue: assets with much blue/ }).click()
    await expect(page.getByRole('button', { name: /^Blue: / })).toHaveAttribute('aria-pressed', 'true')
    await expect(page).toHaveURL(/[?&]color=%232f6fd0/)
    await expect(page.getByRole('button', { name: 'Remove tag colour:#2f6fd0' })).toBeVisible()
    await expect(page.locator('.grid button.hit').first()).toHaveAttribute('aria-label', /^Water surface,/, { timeout: 30_000 })
    await expect(tile(page, 'Brick wall')).toHaveCount(0)
    // Pressing it again takes the colour off.
    await page.getByRole('button', { name: /^Blue: / }).click()
    await expect(assetCount(page)).toHaveText('4 assets')
  })

  test("an asset's swatches and More like this search the whole library", async ({ page, seed }) => {
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    await tile(page, 'Grass ground').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const swatches = panel.getByRole('group', { name: 'Colours in the picture' })
    await expect(swatches.getByRole('button').first()).toBeVisible({ timeout: 30_000 })

    await swatches.getByRole('button').first().click()
    // The pack filter goes: a colour searches everything.
    await expect(page.getByText(`pack:${TEXTURES}`)).toHaveCount(0)
    await expect(page.getByRole('button', { name: /^Remove tag colour:#[0-9a-f]{6}$/ })).toBeVisible()
    await expect(tile(page, 'Grass ground')).toBeVisible()

    await tile(page, 'Grass ground').click()
    await panel.getByRole('button', { name: 'More like this' }).click()
    await expect(page.getByRole('button', { name: 'Remove tag like:Grass ground' })).toBeVisible()
    await expect(page).toHaveURL(/[?&]similar=/)
    await expect(page.locator('.grid button.hit').first()).toBeVisible()
    await expect(tile(page, 'Grass ground')).toHaveCount(0)
  })
})
