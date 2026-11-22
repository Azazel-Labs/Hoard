import { TEXTURES } from './support/seed'
import { expect, openPack, test, tile } from './support/test'

// Settings → Appearance: text size, spacing, and the parts of the tiles and the top bar that can be turned off.
test.describe('appearance', () => {
  test('text size and spacing go on the whole page and stay', async ({ page }) => {
    await page.goto('/#settings/appearance')
    const html = page.locator('html')
    await expect(html).toHaveAttribute('data-text', 'normal')
    await page.getByRole('radiogroup', { name: 'Text size' }).getByText('Large').click()
    await expect(html).toHaveAttribute('data-text', 'large')
    expect(await html.evaluate((el) => getComputedStyle(el).fontSize)).toBe('18px')
    await page.getByRole('radiogroup', { name: 'Spacing' }).getByText('Compact').click()
    await expect(html).toHaveAttribute('data-density', 'compact')
    await page.reload()
    await expect(html).toHaveAttribute('data-text', 'large')
    await expect(html).toHaveAttribute('data-density', 'compact')
  })

  test('tile names and top bar buttons can be turned off', async ({ page, seed }) => {
    await page.goto('/#settings/appearance')
    await page.getByRole('checkbox', { name: 'Names' }).uncheck()
    await page.getByRole('checkbox', { name: /^Command palette button/ }).uncheck()
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    // The name is still the tile's label, and its tooltip.
    await expect(tile(page, 'Brick wall')).toHaveAttribute('title', 'Brick wall')
    await expect(page.locator('.grid .tile .name')).toHaveCount(0)
    await expect(page.locator('.grid .tile .sub').first()).toBeVisible()
    await expect(page.locator('.top .cmdk')).toHaveCount(0)
    // Its shortcut still opens the palette.
    await page.keyboard.press('ControlOrMeta+K')
    await expect(page.getByRole('dialog', { name: 'Command palette' })).toBeVisible()
  })
})
