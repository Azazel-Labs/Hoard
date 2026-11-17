import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { png } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

const RED: [number, number, number, number] = [230, 60, 60, 255]
const BLUE: [number, number, number, number] = [70, 110, 240, 255]

test('a sprite atlas becomes a sprite sheet that shows its sprites, plays their animation and downloads in other formats', async ({
  page,
  api,
}) => {
  test.setTimeout(120_000)
  const name = `E2E Atlas ${unique()}`
  // Two 16 px sprites side by side, numbered, so they're an animation.
  const sheet = png(32, 16, (x) => (x < 16 ? RED : BLUE))
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<TextureAtlas imagePath="sheet.png">
  <SubTexture name="walk_0" x="0" y="0" width="16" height="16"/>
  <SubTexture name="walk_1" x="16" y="0" width="16" height="16"/>
</TextureAtlas>`
  const pack = await uploadPack(api, name, [
    { relpath: 'sheet.png', data: sheet },
    { relpath: 'atlas.xml', data: Buffer.from(xml) },
  ])
  try {
    await waitReady(api, [pack.id])
    // The atlas is read after the pack is: wait for it to become a sheet.
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path === 'atlas.xml')?.kind, { timeout: 60_000 })
      .toBe('sheet')
    expect(assets.map((a) => a.path).sort()).toEqual(['atlas.xml', 'sheet.png'])
    const at = (path: string) => assets.find((a) => a.path === path)!
    const detail = async (id: string) =>
      (await (await api.get(`/api/assets/${id}`)).json()) as {
        meta: Record<string, unknown>
        derivatives: string[]
        formats: { format: string }[]
        links: { id: string; rel: string }[]
      }

    const atlas = await detail(at('atlas.xml').id)
    expect(atlas.meta.sheet).toMatchObject({ format: 'starling', sprites: 2, pages: 1, animations: 1 })
    expect(atlas.derivatives).toContain('sheet')
    expect(atlas.formats.map((f) => f.format)).toEqual(expect.arrayContaining(['sheet-hoard', 'sheet-tp-hash', 'sheet-godot-frames']))
    expect(atlas.links).toContainEqual(expect.objectContaining({ id: at('sheet.png').id, rel: 'texture' }))
    expect((await detail(at('sheet.png').id)).meta.atlas).toMatchObject({ format: 'starling', sprites: 2 })

    const kept = await (await api.get(`/api/assets/${at('atlas.xml').id}/derivative/sheet`)).json()
    expect(kept.sprites.map((s: { name: string }) => s.name)).toEqual(['walk_0', 'walk_1'])
    expect(kept.pages[0]).toMatchObject({ image: 'sheet.png', size: [32, 16], asset: at('sheet.png').id })
    expect(kept.animations).toEqual([{ name: 'walk', loop: true, frames: [expect.objectContaining({ sprite: 0 }), expect.objectContaining({ sprite: 1 })] }])

    // A download in another format is a zip of its files.
    const zip = await api.get(`/api/assets/${at('atlas.xml').id}/as/sheet-tp-hash?wait=60`)
    expect(zip.status()).toBe(200)
    const body = await zip.body()
    expect(body.subarray(0, 2).toString()).toBe('PK')
    expect(body.includes('.json')).toBe(true)
    expect(body.includes('.png')).toBe(true)

    await openPack(page, pack.id, name, '&sort=name')
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await tile(page, at('atlas.xml').name).click()
    await expect(panel.getByTestId('sheet-counts')).toHaveText('2 sprites · 1 page · 1 animation')
    const sprites = panel.locator('section', { has: page.getByRole('heading', { name: 'Sprites', exact: true }) })
    await expect(sprites.getByRole('button')).toHaveCount(2)
    await sprites.getByRole('searchbox', { name: 'Find a sprite' }).fill('_1')
    await expect(sprites.getByRole('button')).toHaveCount(1)
    await sprites.getByRole('button', { name: 'walk_1' }).click()
    await expect(sprites.getByRole('button', { name: 'walk_1' })).toHaveAttribute('aria-pressed', 'true')
    await expect(panel.getByRole('img', { name: 'walk_1' })).toBeVisible()

    // Each format says what it can hold.
    await panel.getByLabel('Download as').selectOption('sheet-godot-frames')
    await expect(panel.getByText('Sprites never turned, their trim kept as margins')).toBeVisible()
    await panel.getByLabel('Download as').selectOption('sheet-libgdx')
    await expect(panel.getByText('Sprites turned counter-clockwise and trimmed.')).toBeVisible()
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
