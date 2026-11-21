import { expect, test } from './support/test'

test.describe('finding a setting', () => {
  test('shows the sections with every word, editable where they are, and says when none has them', async ({ page }) => {
    // Appearance opens first, with the server's name at its top.
    await page.goto('/#settings')
    await expect(page.getByRole('heading', { name: 'Appearance', level: 2 })).toBeVisible()
    await expect(page.getByLabel('Name', { exact: true })).toBeEditable()

    const find = page.getByTestId('settings-find')
    const found = page.getByTestId('settings-found')
    await find.fill('public ADDRESS')
    await expect(found.getByLabel('Public address')).toBeEditable()
    await expect(found.locator('[data-page="connection"]')).toBeVisible()
    await expect(found.locator('[data-page="appearance"]')).toBeHidden()
    await expect(found.locator('[data-page="maintenance"]')).toBeHidden()
    // The list doesn't say any one page is open while looking.
    await expect(page.locator('nav [aria-current="page"]')).toHaveCount(0)

    // A page's name opens it.
    await found.getByRole('button', { name: 'Connection' }).click()
    await expect(page.getByRole('heading', { name: 'Connection', level: 2 })).toBeVisible()
    await expect(find).toHaveValue('')
    await expect(page).toHaveURL(/#settings\/connection$/)

    // Pages without settings to change are found by their name, and what they're for, as a way to them.
    await find.fill('dragon')
    await expect(found.locator('[data-page="dragon"]')).toContainText('every asset in the library is a piece of its treasure')
    await expect(found.getByText(/No setting matches/)).toHaveCount(0)
    await found.getByRole('button', { name: 'Dragon' }).click()
    await expect(page.getByRole('heading', { name: 'Dragon', level: 2 })).toBeVisible()
    await find.fill('treasure')
    await expect(found.getByRole('button', { name: 'Dragon' })).toBeVisible()

    await find.fill('zzqx nothing')
    await expect(found.getByText('No setting matches “zzqx nothing”.')).toBeVisible()
    await find.press('Escape')
    await expect(found).toHaveCount(0)
    await expect(page.getByRole('heading', { name: 'Dragon', level: 2 })).toBeVisible()
  })
})
