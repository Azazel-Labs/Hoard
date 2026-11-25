import { PASSWORD } from './support/api'
import { expect, test } from './support/test'

test('About says what Hoard is and which version, and lists every library it is built from with its license', async ({ page }) => {
  await page.goto('/#settings/about')
  await expect(page.getByRole('heading', { name: 'About', level: 2 })).toBeVisible()
  await expect(page.getByTestId('about-version')).toHaveText(/^\d+\.\d+\.\d+/)
  await expect(page.getByTestId('about-edition')).toHaveText('Server')
  await expect(page.getByTestId('about-steam')).toHaveCount(0)
  await expect(page.getByTestId('about-discord')).toHaveAttribute('href', 'https://discord.gg/zf9ncS4EW')
  await expect(page.getByRole('link', { name: 'Source code on GitHub' })).toHaveAttribute('href', 'https://github.com/Azazel-Labs/Hoard')

  await page.locator('summary', { hasText: /^MIT-0$/ }).click()
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

// Blender and the Unreal Editor are listed only by a build that fetches them (the Docker image); everywhere else
// they're found where they're installed.
test('Downloaded when needed lists the editors only when this build fetches them', async ({ page }) => {
  await page.goto('/#settings/about')
  const list = page.getByTestId('about-downloaded')
  await expect(list.getByRole('link', { name: 'FFmpeg' })).toBeVisible()
  await expect(list.getByRole('link', { name: 'Blender' })).toHaveCount(0)
  await expect(list.getByRole('link', { name: 'Unreal Editor' })).toHaveCount(0)

  await page.route('**/api/stats', async (route) => {
    const res = await route.fetch()
    await route.fulfill({ response: res, json: { ...(await res.json()), editor_downloads: ['blender', 'unreal'] } })
  })
  await page.reload()
  await expect(list.getByRole('link', { name: 'Blender' })).toBeVisible()
  await expect(list.getByRole('link', { name: 'Unreal Editor' })).toHaveAttribute('href', 'https://www.unrealengine.com/eula/unreal')
  await expect(list).toContainText('Epic Unreal Engine EULA')
})

// Report a problem gathers what an issue needs (the build, where it runs, what's on, the server's last log lines),
// with nothing private in it; Copy takes all of it, and the GitHub button opens a new issue filled in with it.
test('Report a problem gathers the build and the recent log, and opens a new issue on GitHub filled in with them', async ({ page }) => {
  await page.goto('/#settings/about')
  await page.getByTestId('about-report').click()
  const dialog = page.getByRole('dialog', { name: 'Report a problem' })
  await expect(dialog.getByText('Nothing private is in it', { exact: false })).toBeVisible()
  const version = (await page.getByTestId('about-version').textContent())!.trim()
  await expect(dialog.locator('dd').first()).toHaveText(version)
  await expect(dialog.getByText(/^server on (windows|linux|macos)/)).toBeVisible()

  await dialog.getByText('What the server logged lately').click()
  await expect(dialog.getByTestId('report-log')).toContainText(`Hoard ${version}`)
  await expect(dialog.getByTestId('report-log')).not.toContainText(PASSWORD)

  const href = (await dialog.getByTestId('report-github').getAttribute('href'))!
  expect(href.startsWith('https://github.com/Azazel-Labs/Hoard/issues/new?')).toBe(true)
  const q = new URL(href).searchParams
  expect(q.get('template')).toBe('bug.yml')
  expect(q.get('version')).toBe(version)
  expect(q.get('environment')).toMatch(/^server on /)
  expect(q.get('report')).toContain(`- **Version:** ${version}`)
  expect(q.get('report')).toContain('<summary>Recent log</summary>')
  expect(href.length).toBeLessThanOrEqual(7500)

  await dialog.getByRole('button', { name: 'Copy' }).click()
  await expect(page.getByText('Copied the report.')).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(dialog).toBeHidden()
})
