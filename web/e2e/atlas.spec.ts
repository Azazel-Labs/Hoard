import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { hue, png } from './support/files'
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

// A pack that has a sheet's sprites as loose images too (as Kenney's do) keeps them with the sheet: the sheet links
// each one whose picture is its sprite's, and they and its page leave the grid. One with a sprite's name but another
// picture, or the same sprite drawn at twice the size, stays.
test('loose images that are a sheet’s sprites are kept with the sheet', async ({ page, api }) => {
  test.setTimeout(120_000)
  const name = `E2E Loose sprites ${unique()}`
  const GREEN: [number, number, number, number] = [60, 200, 90, 255]
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<TextureAtlas imagePath="sheet.png">
  <SubTexture name="button_red.png" x="0" y="0" width="16" height="16"/>
  <SubTexture name="button_blue.png" x="16" y="0" width="16" height="16"/>
</TextureAtlas>`
  const pack = await uploadPack(api, name, [
    { relpath: 'Spritesheet/sheet.png', data: png(32, 16, (x) => (x < 16 ? RED : BLUE)) },
    { relpath: 'Spritesheet/atlas.xml', data: Buffer.from(xml) },
    // Saved again: a few levels off.
    { relpath: 'PNG/Default/button_red.png', data: png(16, 16, () => [236, 55, 64, 255]) },
    { relpath: 'PNG/Double/button_red.png', data: png(32, 32, () => RED) },
    { relpath: 'PNG/Default/button_blue.png', data: png(16, 16, () => GREEN) },
  ])
  try {
    await waitReady(api, [pack.id])
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path.endsWith('atlas.xml'))?.kind, { timeout: 60_000 })
      .toBe('sheet')
    const at = (path: string) => assets.find((a) => a.path === path)!
    const sheet = at('Spritesheet/atlas.xml')
    const links = async (id: string) =>
      ((await (await api.get(`/api/assets/${id}`)).json()) as { links: { id: string; rel: string }[] }).links
    await expect.poll(async () => (await links(sheet.id)).filter((l) => l.rel === 'sprite').map((l) => l.id)).toEqual([
      at('PNG/Default/button_red.png').id,
    ])

    const shown = await api.get('/api/assets', { params: { pack: pack.id, limit: 500, facets: false, hide_supporting: true } })
    const body = (await shown.json()) as { items: Asset[]; supporting_hidden?: number }
    expect(body.items.map((a) => a.path).sort()).toEqual(['PNG/Default/button_blue.png', 'PNG/Double/button_red.png', 'Spritesheet/atlas.xml'])
    expect(body.supporting_hidden).toBe(2)

    // The sheet lists its loose copy, and the copy names the sheet it's in.
    await openPack(page, pack.id, name, '&sort=name')
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await tile(page, sheet.name).click()
    const loose = panel.locator('section', { has: page.getByRole('heading', { name: 'Also as separate files', exact: true }) })
    await expect(loose.getByRole('button')).toHaveCount(1)
    await loose.getByRole('button').click()
    await expect(panel.getByText('PNG/Default/button_red.png')).toBeVisible()
    const inSheet = panel.locator('section', { has: page.getByRole('heading', { name: 'In sprite sheet', exact: true }) })
    await expect(inSheet.getByRole('button', { name: new RegExp(sheet.name) })).toBeVisible()
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

// A tile sheet that comes with only a note of its grid, as Kenney's do, is a sprite sheet of its tiles: every image on
// the grid (with the space between tiles, or packed without it) is a page, empty cells are left out, and the kit's
// loose tiles are kept with it, by name (`tile_0001.png`) or by picture. One with a tile's name but another picture stays.
test('a tile sheet described by a note of its grid becomes a sprite sheet of its tiles', async ({ page, api }) => {
  test.setTimeout(120_000)
  const name = `E2E Grid sheet ${unique()}`
  // Three by two 8 px tiles, each its own color, the last cell empty.
  const color = (cell: number): [number, number, number, number] => (cell < 5 ? [...hue(cell, 5), 255] : [0, 0, 0, 0])
  const note = [
    'Tilesheet information:',
    '',
    'Tile size                 •  8px × 8px',
    'Space between tiles       •  1px × 1px',
    '---',
    'Total tiles (horizontal)  •  3 tiles',
    'Total tiles (vertical)    •  2 tiles',
  ].join('\n')
  const spaced = png(26, 17, (x, y) => (x % 9 === 8 || y % 9 === 8 ? [0, 0, 0, 0] : color(Math.floor(y / 9) * 3 + Math.floor(x / 9))))
  const packed = png(24, 16, (x, y) => color(Math.floor(y / 8) * 3 + Math.floor(x / 8)))
  const solid = (cell: number) => png(8, 8, () => color(cell))
  const pack = await uploadPack(api, name, [
    { relpath: 'Kit/Tilesheet.txt', data: Buffer.from(note) },
    { relpath: 'Kit/Tilemap/tilemap.png', data: spaced },
    { relpath: 'Kit/Tilemap/tilemap_packed.png', data: packed },
    { relpath: 'Kit/Tiles/tile_0000.png', data: solid(0) },
    { relpath: 'Kit/Tiles/tile_0001.png', data: solid(1) },
    { relpath: 'Kit/Tiles/tile_0003.png', data: png(8, 8, (x, y) => ((x + y) % 2 ? [0, 0, 0, 255] : [255, 255, 255, 255])) },
    { relpath: 'Kit/PNG/flag.png', data: solid(4) },
  ])
  try {
    await waitReady(api, [pack.id])
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path.endsWith('Tilesheet.txt'))?.kind, { timeout: 60_000 })
      .toBe('sheet')
    const at = (path: string) => assets.find((a) => a.path.endsWith(path))!
    const sheet = at('Tilesheet.txt')
    const detail = (await (await api.get(`/api/assets/${sheet.id}`)).json()) as {
      meta: Record<string, unknown>
      links: { id: string; rel: string }[]
    }
    expect(detail.meta.sheet).toMatchObject({ format: 'grid', sprites: 10, pages: 2 })
    expect(detail.links.filter((l) => l.rel === 'texture').map((l) => l.id).sort()).toEqual(
      [at('Tilemap/tilemap.png').id, at('Tilemap/tilemap_packed.png').id].sort(),
    )
    expect(detail.links.filter((l) => l.rel === 'sprite').map((l) => l.id).sort()).toEqual(
      [at('Tiles/tile_0000.png').id, at('Tiles/tile_0001.png').id, at('PNG/flag.png').id].sort(),
    )
    const kept = await (await api.get(`/api/assets/${sheet.id}/derivative/sheet`)).json()
    expect(kept.sprites.map((s: { name: string }) => s.name).slice(0, 5)).toEqual([
      'tilemap/tile_0000',
      'tilemap/tile_0001',
      'tilemap/tile_0002',
      'tilemap/tile_0003',
      'tilemap/tile_0004',
    ])
    expect(kept.sprites[4]).toMatchObject({ rect: [9, 9, 8, 8] })

    const shown = await api.get('/api/assets', { params: { pack: pack.id, limit: 500, facets: false, hide_supporting: true } })
    const body = (await shown.json()) as { items: Asset[]; supporting_hidden?: number }
    expect(body.items.map((a) => a.path.replace(/^Kit\//, '')).sort()).toEqual(['Tiles/tile_0003.png', 'Tilesheet.txt'])
    expect(body.supporting_hidden).toBe(5)

    await openPack(page, pack.id, name, '&sort=name')
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await tile(page, sheet.name).click()
    const sprites = panel.locator('section', { has: page.getByRole('heading', { name: 'Sprites', exact: true }) })
    await expect(sprites.getByRole('button')).toHaveCount(10)
    const loose = panel.locator('section', { has: page.getByRole('heading', { name: 'Also as separate files', exact: true }) })
    await expect(loose.getByRole('button')).toHaveCount(3)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

// A Tiled tileset is a sprite sheet of its tiles: its image is the page, its empty tiles are left out and its tiles'
// animations play.
test('a Tiled tileset becomes a sprite sheet of its tiles', async ({ api }) => {
  test.setTimeout(120_000)
  const name = `E2E Tiled ${unique()}`
  // Two by two 8 px tiles, 2 px apart round a 1 px margin; the last tile empty.
  const color = (cell: number): [number, number, number, number] => (cell < 3 ? [...hue(cell, 3), 255] : [0, 0, 0, 0])
  const image = png(19, 19, (x, y) => {
    const [cx, cy] = [Math.floor((x - 1) / 10), Math.floor((y - 1) / 10)]
    const inside = x >= 1 && y >= 1 && (x - 1) % 10 < 8 && (y - 1) % 10 < 8
    return inside ? color(cy * 2 + cx) : [0, 0, 0, 0]
  })
  const tsx = `<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" name="kit" tilewidth="8" tileheight="8" spacing="2" margin="1" tilecount="4" columns="2">
 <image source="../Tilemap/tiles.png" width="19" height="19"/>
 <tile id="0" type="flicker"><animation><frame tileid="0" duration="150"/><frame tileid="1" duration="150"/></animation></tile>
</tileset>`
  const pack = await uploadPack(api, name, [
    { relpath: 'Kit/Tiled/kit.tsx', data: Buffer.from(tsx) },
    { relpath: 'Kit/Tilemap/tiles.png', data: image },
  ])
  try {
    await waitReady(api, [pack.id])
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path.endsWith('kit.tsx'))?.kind, { timeout: 60_000 })
      .toBe('sheet')
    const sheet = assets.find((a) => a.path.endsWith('kit.tsx'))!
    const detail = (await (await api.get(`/api/assets/${sheet.id}`)).json()) as { meta: Record<string, unknown> }
    expect(detail.meta.sheet).toMatchObject({ format: 'tiled', sprites: 3, pages: 1, animations: 1 })
    const kept = await (await api.get(`/api/assets/${sheet.id}/derivative/sheet`)).json()
    expect(kept.sprites.map((s: { name: string; rect: number[] }) => [s.name, s.rect])).toEqual([
      ['tile_0000', [1, 1, 8, 8]],
      ['tile_0001', [11, 1, 8, 8]],
      ['tile_0002', [1, 11, 8, 8]],
    ])
    expect(kept.animations).toEqual([expect.objectContaining({ name: 'flicker', frames: [expect.objectContaining({ sprite: 0 }), expect.objectContaining({ sprite: 1 })] })])
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
