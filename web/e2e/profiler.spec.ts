import { expect, test } from './support/test'

// The profiler at the top of Settings → Performance: processor use, what's running, and where requests' time goes.
test.describe('profiler', () => {
  test('shows processor use and the requests just served, by route', async ({ page }) => {
    await page.goto('/#settings/performance')
    const panel = page.locator('section').filter({ has: page.getByRole('heading', { name: 'Profiler', level: 3 }) })
    await expect(panel.getByRole('img', { name: /^Hoard is using .+ of the processors/ })).toBeVisible({
      timeout: 10_000,
    })
    await expect(panel.getByText('Whole machine')).toBeVisible()

    // A few requests of a route the page doesn't ask for itself show up in the table within a poll or two.
    await page.evaluate(async () => {
      for (let i = 0; i < 3; i++) await fetch('/api/tags?limit=1')
    })
    const row = panel.getByRole('row').filter({ has: page.getByRole('rowheader', { name: 'GET /api/tags' }) })
    await expect(row).toBeVisible({ timeout: 10_000 })
    await expect(row.getByRole('cell').first()).toHaveText(/^[3-9]$|^\d{2,}$/)
    await expect(row.getByRole('img', { name: /^Database \d+%, Disk \d+%, Sending \d+%, Other \d+%$/ })).toBeVisible()
    await expect(panel.getByRole('img', { name: /\d+ requests · Database \d+% · Disk/ })).toBeVisible()

    // Hovering the processor chart reads out that second.
    const cpu = panel.getByRole('img', { name: /^Hoard is using/ })
    const box = (await cpu.boundingBox())!
    await page.mouse.move(box.x + box.width - 12, box.y + box.height / 2)
    await expect(panel.getByText('Requests in progress')).toBeVisible()
  })
})
