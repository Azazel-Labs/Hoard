import { HOARD_HEADERS } from './support/api'
import { expect, test } from './support/test'

test('an admin can stop background work: one job, all AI tagging, or a Maintenance task partway', async ({ page, api }) => {
  // A job that isn't waiting or running can't be stopped.
  const gone = await api.post('/api/admin/jobs/999999999/stop', { headers: HOARD_HEADERS })
  expect(gone.status()).toBe(404)
  expect((await gone.json()).error).toContain("isn't waiting or running")

  // Stopping all AI tagging says how many it stopped (none, with no AI set up here).
  const all = await api.post('/api/admin/jobs/stop', { headers: HOARD_HEADERS, data: { kind: 'llm_tag' } })
  expect(all.status()).toBe(200)
  expect((await all.json()).stopped).toBeGreaterThanOrEqual(0)

  // The activity list says how much AI tagging there is, for the Stop all button.
  expect((await (await api.get('/api/activity')).json()).ai_tagging).toBeGreaterThanOrEqual(0)

  // A task that isn't running can't be stopped, and one that can't stop partway says so.
  const task = async (id: string) => (await (await api.get('/api/admin/maintenance')).json()).tasks.find((t: { id: string }) => t.id === id)
  const idle = await api.post('/api/admin/maintenance/scrub/stop', { headers: HOARD_HEADERS })
  expect(idle.status()).toBe(404)
  expect((await idle.json()).error).toContain("isn't running")

  // Start a scrub and stop it straight away. It stops between files, or finishes first if the library is quick to check.
  expect((await api.post('/api/admin/maintenance/scrub', { headers: HOARD_HEADERS })).ok()).toBe(true)
  const stop = await api.post('/api/admin/maintenance/scrub/stop', { headers: HOARD_HEADERS })
  expect([202, 404]).toContain(stop.status())
  await expect.poll(async () => (await task('scrub')).running, { timeout: 120_000 }).toBeNull()
  const { last } = await task('scrub')
  expect(last.ok).toBe(true)
  expect(last.summary).toMatch(stop.status() === 202 ? /^(Stopped after checking \d+ of \d+ files?|Checked \d+ files?):/ : /^Checked \d+ files?:/)

  // On the page, the Scrub button is back to Run once it's done.
  await page.goto('/#settings/maintenance')
  const row = page.locator('li', { has: page.getByText('Check files for damage') }).first()
  await expect(row.getByRole('button', { name: 'Run' })).toBeEnabled()
})

test('the activity list has a stop button on what can be stopped, and stops all AI tagging in one go', async ({ page }) => {
  const now = new Date().toISOString()
  const asked: string[] = []
  let items = [
    { id: 1, kind: 'llm_tag', label: 'AI tagging · Forest Pack', state: 'running', since: now, stop: '/api/admin/jobs/7/stop' },
    { id: 2, kind: 'llm_tag', label: 'AI tagging · Desert Pack', state: 'queued', since: now, stop: '/api/admin/jobs/8/stop' },
    { id: 3, kind: 'ingest', label: 'Importing · Ruins Pack', state: 'queued', since: now },
  ]
  await page.route('**/api/activity', (route) => route.fulfill({ json: { items, ai_tagging: items.filter((i) => i.kind === 'llm_tag').length } }))
  await page.route('**/api/admin/jobs/**', async (route) => {
    const url = new URL(route.request().url()).pathname
    asked.push(url)
    if (url === '/api/admin/jobs/stop') {
      expect(route.request().postDataJSON()).toEqual({ kind: 'llm_tag' })
      items = items.filter((i) => i.kind !== 'llm_tag')
      return route.fulfill({ json: { stopped: 1 } })
    }
    items = items.filter((i) => i.stop !== url)
    return route.fulfill({ status: 204 })
  })
  await page.goto('/')
  await page.getByRole('button', { name: /^Background activity/ }).click()
  const list = page.getByRole('region', { name: 'Background activity' })

  // Imports can't be stopped from here (delete the pack instead), so they have no button.
  await expect(list.getByRole('button', { name: 'Stop “Importing · Ruins Pack”' })).toHaveCount(0)
  await list.getByRole('button', { name: 'Stop “AI tagging · Desert Pack”' }).click()
  await expect(list.getByText('AI tagging · Desert Pack')).toHaveCount(0)
  expect(asked).toEqual(['/api/admin/jobs/8/stop'])

  // With only one left there's no Stop all; put another back and there is.
  await expect(list.getByRole('button', { name: /^Stop all AI tagging/ })).toHaveCount(0)
  items.push({ id: 4, kind: 'llm_tag', label: 'AI tagging · Cave Pack', state: 'queued', since: now, stop: '/api/admin/jobs/9/stop' })
  const all = list.getByRole('button', { name: 'Stop all AI tagging (2)' })
  await expect(all).toBeVisible({ timeout: 10_000 })
  await all.click()
  await expect(list.getByText(/AI tagging ·/)).toHaveCount(0)
  await expect(list.getByText('Importing · Ruins Pack')).toBeVisible()
})
