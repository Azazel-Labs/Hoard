import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { png } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

const RED: [number, number, number, number] = [230, 60, 60, 255]
const BLUE: [number, number, number, number] = [70, 110, 240, 255]

test('a sprite atlas holds its sprites as assets of their own, and a sprite the pack has loose is that file', async ({
  page,
  api,
}) => {
  const name = `E2E Atlas ${unique()}`
  // Two 16 px sprites side by side; the blue one is in the pack as a file of its own too, under another name.
  const sheet = png(32, 16, (x) => (x < 16 ? RED : BLUE))
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<TextureAtlas imagePath="sheet.png">
  <SubTexture name="red" x="0" y="0" width="16" height="16"/>
  <SubTexture name="gem" x="16" y="0" width="16" height="16"/>
</TextureAtlas>`
  const pack = await uploadPack(api, name, [
    { relpath: 'sheet.png', data: sheet },
    { relpath: 'atlas.xml', data: Buffer.from(xml) },
    { relpath: 'loose/blue.png', data: png(16, 16, () => BLUE) },
  ])
  try {
    await waitReady(api, [pack.id])
    // The atlas is read after the pack is: wait for its sprite to turn up.
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).map((a) => a.path).sort(), { timeout: 60_000 })
      .toEqual(['atlas.xml', 'atlas/red.png', 'loose/blue.png', 'sheet.png'])
    const at = (path: string) => assets.find((a) => a.path === path)!
    const detail = async (id: string) =>
      (await (await api.get(`/api/assets/${id}`)).json()) as {
        meta: Record<string, unknown>
        tags: { tag: string }[]
        links: { id: string; rel: string }[]
        linked_from: { id: string; rel: string }[]
      }

    const red = await detail(at('atlas/red.png').id)
    expect(red.meta.sprite).toMatchObject({ rect: [0, 0, 16, 16], rotated: false })
    expect(red.meta.width).toBe(16)
    expect(red.tags.map((t) => t.tag)).toContain('sprite')
    const page_ = await detail(at('sheet.png').id)
    expect(page_.meta.atlas).toMatchObject({ format: 'starling', sprites: 2 })
    expect(page_.links.filter((l) => l.rel === 'sprite').map((l) => l.id).sort()).toEqual(
      [at('atlas/red.png').id, at('loose/blue.png').id].sort(),
    )
    expect((await detail(at('atlas.xml').id)).links).toContainEqual(expect.objectContaining({ id: at('sheet.png').id, rel: 'texture' }))

    await openPack(page, pack.id, name, '&sort=name')
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const group = (title: string) => panel.locator('section', { has: page.getByRole('heading', { name: title, exact: true }) })
    await tile(page, at('sheet.png').name).click()
    await expect(group('Sprites').getByRole('button')).toHaveCount(2)
    // An atlas page isn't a grid of frames to play.
    await expect(panel.getByRole('checkbox', { name: 'Play as animation' })).toHaveCount(0)
    await group('Sprites').getByRole('button', { name: new RegExp(at('atlas/red.png').name) }).click()
    await expect(group('In atlas').getByRole('button', { name: new RegExp(at('sheet.png').name) })).toBeVisible()
    await expect(group('Used by')).toHaveCount(0)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
