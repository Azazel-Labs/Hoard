import { mkdtempSync, readdirSync, rmSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { HOARD_HEADERS } from './support/api'
import { expect, test } from './support/test'

test.describe('Scheduled backups', () => {
  test('turning them on backs up at once, copies to the folder given, and shows in the history', async ({ page, api }) => {
    const folder = mkdtempSync(join(tmpdir(), 'hoard-e2e-backups-'))
    try {
      // Not a full path, or not a folder: refused.
      for (const bad of ['relative/folder', join(folder, 'missing')]) {
        const r = await api.put('/api/admin/settings', { headers: HOARD_HEADERS, data: { backup_copy_to: bad } })
        expect(r.status()).toBe(400)
      }
      const keep = await api.put('/api/admin/settings', { headers: HOARD_HEADERS, data: { backup_keep: 0 } })
      expect(keep.status()).toBe(400)

      await page.goto('/#settings/maintenance')
      await expect(page.getByRole('heading', { name: 'Scheduled backups' })).toBeVisible()
      const copyTo = page.getByLabel('Also copy to', { exact: true })
      await expect(copyTo).toHaveAttribute('placeholder', 'No other folder')
      await copyTo.fill(folder)
      await copyTo.press('Enter')
      await expect(page.getByRole('status').filter({ hasText: `Also copy to: ${folder}` })).toBeVisible()

      await page.getByLabel('Back up', { exact: true }).selectOption({ label: 'Every day' })
      // Due at once, as none has been made into this folder yet; it shows in the history once it's done.
      await expect(page.getByRole('cell', { name: new RegExp(`hoard-auto-.*copied to ${folder.replace(/[\\^$.*+?()[\]{}|]/g, '\\$&')}`) }).first()).toBeVisible({
        timeout: 30_000,
      })
      await expect.poll(() => readdirSync(folder)).toEqual([expect.stringMatching(/^hoard-auto-\d{8}-\d{6}\.db$/)])

      const settings = await (await api.get('/api/admin/settings')).json()
      expect(settings.options.backup_every).toEqual(expect.objectContaining({ value: 'daily', default: 'off' }))
      expect(settings.options.backup_copy_to.value).toBe(folder)
    } finally {
      await api.put('/api/admin/settings', {
        headers: HOARD_HEADERS,
        data: { backup_every: null, backup_keep: null, backup_copy_to: null },
      })
      rmSync(folder, { recursive: true, force: true })
    }
  })
})
