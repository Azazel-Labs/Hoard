import { expect, test } from './support/test'

// Only looks: turning image or sound search on downloads a model of 600 MB or so, which a test run mustn't.
test.describe('Image and sound search settings', () => {
  test('are off until turned on, and say what turning them on costs', async ({ page }) => {
    await page.goto('/#settings/search')
    await expect(page.getByRole('heading', { name: 'Image and sound search', level: 2 })).toBeVisible()
    await expect(page.getByLabel('Turn on image search')).not.toBeChecked()
    await expect(page.getByTestId('clip-status')).toHaveText('Off')
    await expect(page.getByLabel('Turn on sound search')).not.toBeChecked()
    await expect(page.getByTestId('clap-status')).toHaveText('Off')
    await expect(page.getByText(/downloads the model \(\d+ MB\)/)).toHaveCount(2)
  })
})
