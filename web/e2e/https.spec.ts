import { expect, test } from './support/test'

// Only looks: turning HTTPS on would change how the shared server answers every other test.
test.describe('Connection settings', () => {
  test('has the public address, shows that HTTPS is off and the ways to turn it on', async ({ page }) => {
    await page.goto('/#settings/connection')
    await expect(page.getByRole('heading', { name: 'Connection', level: 2 })).toBeVisible()
    await expect(page.getByLabel('Public address')).toBeVisible()
    await expect(page.getByText('HTTPS is off: pages are served over plain http://.')).toBeVisible()
    const modes = page.getByRole('radiogroup', { name: 'HTTPS' })
    await expect(modes.getByRole('radio', { name: /^Off/ })).toBeChecked()
    await expect(modes.getByRole('radio', { name: /Hoard’s own certificate/ })).toBeVisible()
    await expect(page.getByRole('heading', { name: 'If you get locked out' })).toBeVisible()
  })

  test('no plain-http notice on this computer, where browsers treat http as secure', async ({ page }) => {
    await page.goto('/')
    await expect(page.getByLabel('Search assets')).toBeVisible()
    await expect(page.getByRole('note').filter({ hasText: 'This is plain http://' })).toHaveCount(0)
  })
})
