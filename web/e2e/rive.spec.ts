import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { expect, openPack, test, tile, unique } from './support/test'

/** Rive's variable-length unsigned integer. */
function varuint(n: number): number[] {
  const out: number[] = []
  do {
    let b = n & 0x7f
    n >>>= 7
    if (n) b |= 0x80
    out.push(b)
  } while (n)
  return out
}
const f32 = (n: number) => [...new Uint8Array(new Float32Array([n]).buffer)]
const u32 = (n: number) => [...new Uint8Array(new Uint32Array([n]).buffer)]
const str = (s: string) => [...varuint(Buffer.byteLength(s)), ...Buffer.from(s)]

/** One object: its type, then its properties (each a key and its value's bytes), then a zero. */
const object = (type: number, ...props: [number, number[]][]) => [...varuint(type), ...props.flatMap(([k, v]) => [...varuint(k), ...v]), 0]

/** A Rive file: the artboard "Box", 100 by 100, with an 80 by 80 red square in the middle, and an animation "Idle". */
function riv(): Buffer {
  return Buffer.from([
    ...Buffer.from('RIVE'),
    // Version 7.0, file 1, and nothing in the table of contents.
    7, 0, 1, 0,
    ...object(23), // The backboard.
    ...object(1, [4, str('Box')], [7, f32(100)], [8, f32(100)]), // 0: the artboard.
    ...object(3, [5, varuint(0)], [13, f32(50)], [14, f32(50)]), // 1: a shape in it, at its middle.
    ...object(7, [5, varuint(1)], [20, f32(80)], [21, f32(80)]), // 2: a rectangle, the shape's path.
    ...object(20, [5, varuint(1)]), // 3: the shape's fill,
    ...object(18, [5, varuint(3)], [37, u32(0xffff0000)]), // 4: which is solid red.
    ...object(31, [55, str('Idle')], [56, varuint(60)], [57, varuint(60)]), // The animation: a second at 60 fps.
  ])
}

test('a Rive file plays in the details panel and gets a thumbnail drawn by the browser', async ({ page, api }) => {
  test.setTimeout(120_000)
  const name = `E2E Rive ${unique()}`
  const pack = await uploadPack(api, name, [{ relpath: 'box.riv', data: riv() }])
  try {
    await waitReady(api, [pack.id])
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path === 'box.riv')?.kind, { timeout: 60_000 })
      .toBe('vector')
    const box = assets.find((a) => a.path === 'box.riv')!
    const detail = async () =>
      (await (await api.get(`/api/assets/${box.id}`)).json()) as { thumb_source: string | null; meta: Record<string, unknown> }
    expect((await detail()).meta).toMatchObject({ rive: '7.0', rive_artboard: 'Box', width: 100, height: 100 })

    // Opened, it plays, and lists what it can play; and the browser draws its thumbnail, which the server can't.
    await openPack(page, pack.id, name)
    await tile(page, box.name).click()
    const player = page.locator('.rive')
    await expect(player.locator('canvas')).toBeVisible()
    await expect(player.locator('select option', { hasText: 'Idle' })).toHaveCount(1)
    // The square is drawn red, in the middle of the stage.
    const middle = () =>
      player.locator('canvas').evaluate((c: HTMLCanvasElement) => [...c.getContext('2d')!.getImageData(c.width / 2, c.height / 2, 1, 1).data])
    await expect.poll(middle).toEqual([255, 0, 0, 255])
    await expect.poll(async () => (await detail()).thumb_source, { timeout: 30_000 }).toBe('browser')
    const thumb = await api.get(`/api/assets/${box.id}/thumb`)
    expect(thumb.status()).toBe(200)

    // Picking the animation plays that instead.
    await player.locator('select').selectOption({ label: 'Idle' })
    await expect(player.locator('select')).toBeEnabled()
    await expect.poll(middle).toEqual([255, 0, 0, 255])
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
