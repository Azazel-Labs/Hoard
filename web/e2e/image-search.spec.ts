import { expect, test } from './support/test'

// Only looks: turning image search on downloads a 605 MB model, which a test run mustn't.
test.describe('Image search settings', () => {
  test('is off until turned on, and says what turning it on costs', async ({ page }) => {
    await page.goto('/#settings/search')
    await expect(page.getByRole('heading', { name: 'Image search', level: 2 })).toBeVisible()
    await expect(page.getByLabel('Turn on image search')).not.toBeChecked()
    await expect(page.getByTestId('clip-status')).toHaveText('Off')
    await expect(page.getByText(/downloads the model \(\d+ MB\)/)).toBeVisible()
  })
})
