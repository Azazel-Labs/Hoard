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

    // Requests are recorded by route, with their time split up. (Other tests running alongside fill the table's top
    // routes, so these are checked in what the panel reads rather than looked for in the table.)
    await page.evaluate(async () => {
      for (let i = 0; i < 3; i++) await fetch('/api/tags?limit=1')
    })
    type Served = { route: string; method: string; status: number; total: number }
    let served: Served[] = []
    await expect
      .poll(async () => {
        served = await page.evaluate(async () => {
          const profile = await (await fetch('/api/admin/profile?after=0')).json()
          return profile.served.filter((r: { route: string }) => r.route === '/api/tags')
        })
        return served.length
      })
      .toBeGreaterThanOrEqual(3)
    expect(served[0]).toMatchObject({ method: 'GET', status: 200 })
    for (const r of served) expect(r.total).toBeGreaterThan(0)

    const row = panel.getByRole('row').filter({ has: page.getByRole('rowheader', { name: /^(GET|POST|PUT|DELETE) / }) })
    await expect(row.first()).toBeVisible({ timeout: 10_000 })
    const split = /^Database \d+%, Disk \d+%, Sending \d+%, Other \d+%$/
    await expect(row.first().getByRole('img', { name: split })).toBeVisible()
    await expect(panel.getByRole('img', { name: /\d+ requests · Database \d+% · Disk/ })).toBeVisible()

    // Hovering the processor chart reads out that second.
    const cpu = panel.getByRole('img', { name: /^Hoard is using/ })
    const box = (await cpu.boundingBox())!
    await page.mouse.move(box.x + box.width - 12, box.y + box.height / 2)
    await expect(panel.getByText('Requests in progress')).toBeVisible()
  })
})
