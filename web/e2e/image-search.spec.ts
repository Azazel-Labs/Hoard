import { expect, test } from './support/test'

// Only looks: turning image or sound search on downloads a model of 0.6–1.5 GB, which a test run mustn't.
test.describe('Search settings', () => {
  test('are off until turned on, and say what turning them on costs', async ({ page }) => {
    await page.goto('/#settings/search')
    await expect(page.getByRole('heading', { name: 'Search', level: 2 })).toBeVisible()
    await expect(page.getByLabel('Turn on image search')).not.toBeChecked()
    await expect(page.getByTestId('clip-status')).toHaveText('Off')
    await expect(page.getByLabel('Turn on sound search')).not.toBeChecked()
    await expect(page.getByTestId('clap-status')).toHaveText('Off')
    await expect(page.getByText(/downloads the model \([\d,]+ MB\)/)).toHaveCount(2)
  })

  // Only the form: nothing is saved, so no service is ever called.
  test('say plainly that pictures leave the server before a hosted service is chosen', async ({ page }) => {
    await page.goto('/#settings/search')
    const where = page.getByRole('radiogroup', { name: 'Runs on' })
    await expect(where.getByRole('radio', { name: 'This server: nothing leaves it' })).toBeChecked()
    await expect(page.getByTestId('clip-hosted-warning')).toHaveCount(0)
    await page.getByTestId('clip-hosted').check()
    await expect(page.getByTestId('clip-hosted-warning')).toContainText('Pictures leave this server')
    await expect(page.getByTestId('clip-hosted-warning')).toContainText('Voyage AI')
    const save = page.getByRole('button', { name: 'Save' }).last()
    await expect(save).toBeDisabled()
    await page.locator('#clip-provider').selectOption({ label: 'Cohere' })
    await expect(page.getByTestId('clip-hosted-warning')).toContainText('Cohere')
    await page.locator('#clip-key').fill('not-a-real-key')
    await expect(save).toBeEnabled()
    await where.getByRole('radio', { name: 'This server: nothing leaves it' }).check()
    await expect(page.getByTestId('clip-hosted-warning')).toHaveCount(0)
  })
})
