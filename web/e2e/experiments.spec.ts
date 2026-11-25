import { expect, test } from './support/test'

test.describe('experimental features', () => {
  test('are off until turned on, and stay as they were left', async ({ page }) => {
    await page.goto('/#settings/experimental')
    await expect(page.getByRole('heading', { name: 'Experimental', level: 2 })).toBeVisible()
    const far = page.getByLabel('Far copies in the mesh explorer')
    await expect(far).not.toBeChecked()

    await far.check()
    await page.reload()
    await expect(page.getByLabel('Far copies in the mesh explorer')).toBeChecked()

    await page.getByLabel('Far copies in the mesh explorer').uncheck()
    await page.reload()
    await expect(page.getByLabel('Far copies in the mesh explorer')).not.toBeChecked()
  })
})
