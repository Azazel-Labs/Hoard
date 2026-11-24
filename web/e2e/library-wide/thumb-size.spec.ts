import { assetsOf, HOARD_HEADERS, uploadPack, waitReady } from '../support/api'
import { png } from '../support/files'
import { expect, test, unique } from '../support/test'

// The thumbnail size is the whole library's: changing it draws thumbnails again everywhere, so this runs after the
// rest (the "library-wide" project), not beside tests waiting for the server to be idle.

/** A WebP's width, from its header: lossy, lossless or extended. */
function webpWidth(b: Buffer): number {
  const chunk = b.subarray(12, 16).toString('latin1')
  if (chunk === 'VP8X') return 1 + b.readUIntLE(24, 3)
  if (chunk === 'VP8L') return 1 + (b.readUInt16LE(21) & 0x3fff)
  return b.readUInt16LE(26) & 0x3fff
}

test('thumbnails are WebP, and picking a smaller size makes the ones there are again in the background', async ({ page, api }) => {
  test.setTimeout(180_000)
  // Smooth color with detail, like a render.
  const photo = png(512, 512, (x, y) => {
    const ripple = Math.sin(x / 11) * Math.cos(y / 15) * 40
    const c = (v: number) => Math.max(0, Math.min(255, Math.round(v * 200 + ripple)))
    return [c(x / 512), c(y / 512), c(1 - (x * y) / 262_144), 255]
  })
  const pack = await uploadPack(api, `E2E thumbs ${unique()}`, [{ relpath: 'render.png', data: photo }])
  try {
    await waitReady(api, [pack.id])
    const [asset] = await assetsOf(api, pack.id)
    const thumb = async () => {
      const res = await api.get(`/api/assets/${asset.id}/thumb`)
      return { type: res.headers()['content-type'], width: webpWidth(await res.body()) }
    }
    expect(await thumb()).toEqual({ type: 'image/webp', width: 256 })
    // The engine plugins ask for JSON and read PNG and JPEG only, so they're sent PNG.
    const plugin = await api.get(`/api/assets/${asset.id}/thumb`, { headers: { Accept: 'application/json' } })
    expect(plugin.headers()['content-type']).toBe('image/png')
    expect((await plugin.body()).readUInt32BE(16)).toBe(256)

    await page.goto('/#settings/performance')
    // Each quality says the quality it's made at; there's no format to pick, and no Lossless.
    const choice = (name: string) =>
      page.getByRole('radiogroup', { name: 'Thumbnails' }).locator('label').filter({ has: page.getByText(name, { exact: true }) })
    await expect(choice('Balanced')).toContainText('Quality 75. Hard to tell')
    await expect(choice('Sharper')).toContainText('Quality 88.')
    await expect(page.getByRole('radiogroup', { name: 'Thumbnails' }).locator('label')).toHaveCount(3)
    const size = page.getByRole('radiogroup', { name: 'Size' })
    await expect(size.getByRole('radio', { name: '256 px' })).toBeChecked()
    await size.getByText('128 px', { exact: true }).click()
    await expect.poll(async () => (await (await api.get('/api/admin/settings')).json()).options.thumb_size.value).toBe(128)
    await expect.poll(thumb, { timeout: 60_000 }).toEqual({ type: 'image/webp', width: 128 })
  } finally {
    await api.put('/api/admin/settings', { headers: HOARD_HEADERS, data: { thumb_size: null } })
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
  await page.reload()
  await expect(page.getByRole('radiogroup', { name: 'Size' }).getByRole('radio', { name: '256 px' })).toBeChecked()
})
