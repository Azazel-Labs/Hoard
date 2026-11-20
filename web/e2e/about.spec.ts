import { expect, test } from './support/test'

test('About says what Hoard is and which version, and lists every library it is built from with its license', async ({ page }) => {
  await page.goto('/#settings/about')
  await expect(page.getByRole('heading', { name: 'About', level: 2 })).toBeVisible()
  await expect(page.getByTestId('about-version')).toHaveText(/^\d+\.\d+\.\d+/)
  await expect(page.getByTestId('about-edition')).toHaveText('Server')
  await expect(page.getByTestId('about-steam')).toHaveCount(0)
  await expect(page.getByTestId('about-discord')).toHaveAttribute('href', 'https://discord.gg/zf9ncS4EW')
  await expect(page.getByRole('link', { name: 'Source code on GitHub' })).toHaveAttribute('href', 'https://github.com/Azazel-Labs/Hoard')

  await page.getByText('MIT-0', { exact: true }).click()
  await expect(page.getByText(/MIT No Attribution/).first()).toBeVisible()

  const built = page.getByTestId('about-built-with')
  await built.getByTestId('about-filter').fill('serde_json')
  const row = built.locator('details', { hasText: 'serde_json' }).first()
  await expect(row).toBeVisible()
  await expect(built.getByText(/^1 of \d+$/)).toBeVisible()
  await row.locator('summary').click()
  await expect(row.locator('pre').first()).toContainText(/Permission is hereby granted|Apache License/)

  await built.getByTestId('about-filter').fill('three')
  await expect(built.locator('details', { hasText: /^three/ }).first()).toBeVisible()
})
