import { assetsOf, uploadPack, waitReady } from './support/api'
import { sound } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

test('a sound’s More like this finds what sounds like it, whatever it’s called', async ({ page, api }) => {
  const name = `E2E Sounds ${unique()}`
  // Named so the words can't help: the thuds and the lasers are told apart by how they sound.
  const files = [
    ...[1, 2, 3].map((s) => ({ relpath: `thud/take_${s}.wav`, data: sound('thud', s) })),
    ...[1, 2].map((s) => ({ relpath: `zap/take_${s + 3}.wav`, data: sound('zap', s) })),
    ...[1, 2].map((s) => ({ relpath: `rain/take_${s + 5}.wav`, data: sound('rain', s) })),
  ]
  const pack = await uploadPack(api, name, files)
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const at = (path: string) => assets.find((a) => a.path === path)!
    const thud = at('thud/take_1.wav')

    // The sounds are listened to after they're in, in the background.
    let found: string[] = []
    await expect
      .poll(
        async () => {
          const r = (await (await api.get(`/api/assets?similar=${thud.id}&limit=20`)).json()) as { items: { path: string }[] }
          found = r.items.map((a) => a.path)
          return found.length
        },
        { timeout: 60_000 },
      )
      .toBeGreaterThan(0)
    expect(found.slice(0, 2).sort()).toEqual(['thud/take_2.wav', 'thud/take_3.wav'])
    expect(found).not.toContain('rain/take_6.wav')
    expect(found).not.toContain('thud/take_1.wav')

    await openPack(page, pack.id, name, '&sort=name')
    await tile(page, thud.name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await panel.getByRole('button', { name: 'More like this' }).click()
    await expect(page).toHaveURL(/[?&]similar=/)
    const first = page.locator('.grid button.hit')
    await expect(first.first()).toBeVisible()
    for (const n of [0, 1]) {
      await expect(first.nth(n)).toHaveAttribute('aria-label', new RegExp(`^(${at('thud/take_2.wav').name}|${at('thud/take_3.wav').name}),`))
    }
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
