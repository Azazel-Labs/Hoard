import { assetsOf, uploadPack, waitReady } from './support/api'
import { png, sound } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

/** A smooth, blotchy picture that looks the same at any size. */
function blotches(size: number): Buffer {
  return png(size, size, (x, y) => {
    const [u, v] = [x / size, y / size]
    const at = (k: number) => Math.sin(u * 4 + v * k * 2 + 1) * Math.cos(v * (2 + k) - u) * 0.5 + 0.5
    return [Math.round(at(1) * 255), Math.round(at(2.3) * 200), Math.round(at(3.7) * 120), 255]
  })
}

test('near copies are listed on an asset and across the library, the best marked', async ({ page, api }) => {
  test.setTimeout(180_000)
  const name = `E2E Copies ${unique()}`
  const pack = await uploadPack(api, name, [
    { relpath: 'art/mural.png', data: blotches(256) },
    { relpath: 'small/mural-small.png', data: blotches(128) },
    { relpath: 'sfx/laser.wav', data: sound('zap', 2, 22_050) },
    { relpath: 'low/laser-low.wav', data: sound('zap', 2, 11_025) },
  ])
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const at = (path: string) => assets.find((a) => a.path === path)!
    const [big, small, laser, low] = ['art/mural.png', 'small/mural-small.png', 'sfx/laser.wav', 'low/laser-low.wav'].map(at)

    // Pictures and sounds are looked at and listened to in the background, after the pack is ready, behind what
    // else is being processed.
    const near = async (id: string) =>
      (((await (await api.get(`/api/assets/${id}`)).json()) as { near_copies?: { id: string; best: boolean }[] }).near_copies ?? []).map(
        (n) => `${n.id}${n.best ? ' (best)' : ''}`,
      )
    await expect.poll(() => near(small.id), { timeout: 120_000 }).toEqual([`${big.id} (best)`])
    expect(await near(big.id)).toEqual([small.id])
    await expect.poll(() => near(low.id), { timeout: 120_000 }).toEqual([`${laser.id} (best)`])

    await openPack(page, pack.id, name, '&sort=name')
    await tile(page, small.name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const section = panel.getByTestId('near-copies')
    await expect(section.getByRole('heading', { name: 'Near copies' })).toBeVisible()
    await expect(section).toContainText('The best copy is marked.')
    await expect(section.getByRole('button', { name: /PNG · .* · 256 × 256/ })).toContainText('best')
    // The best one says so of itself.
    await section.getByRole('button', { name: /PNG/ }).click()
    await expect(panel.getByRole('heading', { name: big.name })).toBeVisible()
    await expect(section).toContainText('This one is the best copy.')
    await expect(section.getByRole('button', { name: /PNG · .* · 128 × 128/ })).not.toContainText('best')
    if (process.env.HOARD_SHOTS) await section.evaluate((el) => el.scrollIntoView({ block: 'start' }))
    if (process.env.HOARD_SHOTS) await section.screenshot({ path: `${process.env.HOARD_SHOTS}/near-copies.png` })

    // Every set in the library, on its own Settings page; it lists them, it doesn't delete anything.
    await page.goto('/#settings/copies')
    await expect(page.getByRole('heading', { name: 'Near copies', level: 2 })).toBeVisible()
    await expect(page.getByTestId('copies-sum')).toContainText('The copies that aren’t the best take')
    const set = page.getByTestId('copies-group').filter({ hasText: low.name })
    await expect(set.getByRole('listitem')).toHaveCount(2)
    await expect(set.getByRole('listitem').first()).toContainText(laser.name)
    await expect(set.getByRole('listitem').first()).toContainText('best')
    await expect(set.getByRole('listitem').first()).toContainText('22.05 kHz')
    await expect(set.getByRole('button', { name: /delete|archive/i })).toHaveCount(0)
    if (process.env.HOARD_SHOTS) await page.screenshot({ path: `${process.env.HOARD_SHOTS}/copies-page.png` })
    await set.getByRole('button', { name: low.name }).click()
    await expect(page.getByRole('complementary', { name: 'Asset details' }).getByRole('heading', { name: low.name })).toBeVisible()
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
