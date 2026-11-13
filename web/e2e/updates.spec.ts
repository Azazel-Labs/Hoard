import { createServer, type IncomingHttpHeaders } from 'node:http'
import type { AddressInfo } from 'node:net'
import { HOARD_HEADERS } from './support/api'
import { expect, test } from './support/test'

interface Received { method?: string; headers: IncomingHttpHeaders; body: string }

test.describe('Updates', () => {
  test('the update address is saved, called with its header and the release, and the header is never shown', async ({ page, api }) => {
    // Stands in for Watchtower or a webhook listener.
    const received: Received[] = []
    const server = createServer((req, res) => {
      let body = ''
      req.on('data', (c: Buffer) => (body += c.toString()))
      req.on('end', () => {
        received.push({ method: req.method, headers: req.headers, body })
        res.writeHead(202).end()
      })
    })
    await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve))
    const url = `http://127.0.0.1:${(server.address() as AddressInfo).port}/v1/update?container=hoard&async=true`

    try {
      await page.goto('/#settings/maintenance')
      await expect(page.getByRole('heading', { name: 'Updates' })).toBeVisible()
      await page.getByLabel('Address to call').fill(url)
      await page.getByLabel('Authorization').fill('Bearer e2e-token')
      await page.getByRole('button', { name: 'Save', exact: true }).click()
      await expect(page.getByText('Hoard will call it when an update is ready')).toBeVisible()
      await expect(page.getByLabel('Authorization')).toHaveAttribute('placeholder', 'Saved (type to replace)')

      await page.getByRole('button', { name: 'Call it now' }).click()
      await expect(page.getByText('It answered 202 Accepted')).toBeVisible()
      expect(received).toHaveLength(1)
      expect(received[0].method).toBe('POST')
      expect(received[0].headers.authorization).toBe('Bearer e2e-token')
      const body = JSON.parse(received[0].body) as { event: string; version: string; running: string }
      expect(body.event).toBe('update')
      expect(body.running).toBeTruthy()
      await expect(page.getByRole('status').filter({ hasText: /^Called .* 202 Accepted$/ })).toBeVisible()

      // Settings carry the address but never the header; the status says what would hold an update back.
      const settings = await (await api.get('/api/admin/settings')).json()
      expect(settings.update_hook).toEqual({ url, has_auth: true })
      expect(JSON.stringify(settings)).not.toContain('e2e-token')
      const status = await (await api.get('/api/admin/update')).json()
      expect(status.hook.set).toBe(true)
      expect(typeof status.idle).toBe('boolean')
      expect(status.busy).toEqual(expect.objectContaining({ queued: expect.any(Number), uploads: expect.any(Number) }))

      // Not an address: refused.
      const bad = await api.put('/api/admin/settings', { headers: HOARD_HEADERS, data: { update_hook_url: 'watchtower:8080' } })
      expect(bad.status()).toBe(400)
    } finally {
      await api.put('/api/admin/settings', { headers: HOARD_HEADERS, data: { update_hook_url: null, update_hook_auth: null } })
      server.close()
    }
  })
})
