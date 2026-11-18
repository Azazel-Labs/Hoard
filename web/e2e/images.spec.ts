import { assetsOf, uploadPack, waitReady } from './support/api'
import { png } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

const W = 8
const H = 8
/** Each pixel's color: a gradient, so every format's channel order shows. */
const color = (x: number, y: number): [number, number, number, number] => [x * 32, y * 32, 128, 255]
const pixels = (order: (c: [number, number, number, number]) => number[]) => {
  const out: number[] = []
  for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) out.push(...order(color(x, y)))
  return Buffer.from(out)
}
const u16 = (v: number) => Buffer.from(new Uint16Array([v]).buffer)
const u32 = (...v: number[]) => Buffer.from(new Uint32Array(v).buffer)

function icon(type: 1 | 2): Buffer {
  const image = png(W, H, color)
  return Buffer.concat([u16(0), u16(type), u16(1), Buffer.from([W, H, 0, 0]), u16(1), u16(32), u32(image.length, 22), image])
}

function qoi(): Buffer {
  const head = Buffer.from('qoif')
  const size = Buffer.alloc(8)
  size.writeUInt32BE(W, 0)
  size.writeUInt32BE(H, 4)
  // Every pixel spelled out (QOI_OP_RGBA).
  const body = pixels((c) => [0xff, ...c])
  return Buffer.concat([head, size, Buffer.from([4, 0]), body, Buffer.from([0, 0, 0, 0, 0, 0, 0, 1])])
}

function pcx(): Buffer {
  const head = Buffer.alloc(128)
  head.set([0x0a, 5, 1, 8])
  head.writeUInt16LE(W - 1, 8)
  head.writeUInt16LE(H - 1, 10)
  head[65] = 3
  head.writeUInt16LE(W, 66)
  // Three planes a row; every value under 0xC0 stands for itself.
  const rows: number[] = []
  for (let y = 0; y < H; y++) for (let p = 0; p < 3; p++) for (let x = 0; x < W; x++) rows.push(Math.min(color(x, y)[p], 0xbf))
  return Buffer.concat([head, Buffer.from(rows)])
}

function ktx(): Buffer {
  const magic = Buffer.from([0xab, 0x4b, 0x54, 0x58, 0x20, 0x31, 0x31, 0xbb, 0x0d, 0x0a, 0x1a, 0x0a])
  const data = pixels((c) => c)
  return Buffer.concat([magic, u32(0x04030201, 0x1401, 1, 0x1908, 0x8058, 0x1908, W, H, 0, 0, 1, 1, 0, data.length), data])
}

function pvr(): Buffer {
  const format = Buffer.from([0x62, 0x67, 0x72, 0x61, 8, 8, 8, 8]) // "bgra", eight bits each
  return Buffer.concat([u32(0x03525650, 0), format, u32(0, 0, H, W, 1, 1, 1, 1, 0), pixels(([r, g, b, a]) => [b, g, r, a])])
}

function vtf(): Buffer {
  const head = Buffer.alloc(80)
  head.write('VTF\0')
  head.writeUInt32LE(7, 4)
  head.writeUInt32LE(2, 8)
  head.writeUInt32LE(80, 12)
  head.writeUInt16LE(W, 16)
  head.writeUInt16LE(H, 18)
  head.writeUInt16LE(1, 24)
  head.writeUInt32LE(12, 52) // BGRA8888
  head[56] = 1
  head.writeUInt32LE(0xffffffff, 57) // no low-res thumbnail
  head.writeUInt16LE(1, 63)
  return Buffer.concat([head, pixels(([r, g, b, a]) => [b, g, r, a])])
}

// Formats Hoard reads itself or with the image library's smaller decoders, each a picture with a thumbnail.
test('icons, cursors, QOI, PNM, PCX and older GPU textures are pictures with thumbnails', async ({ page, api }) => {
  const name = `E2E Images ${unique()}`
  const files = [
    { relpath: 'icon.ico', data: icon(1) },
    { relpath: 'pointer.cur', data: icon(2) },
    { relpath: 'quite_ok.qoi', data: qoi() },
    { relpath: 'portable.ppm', data: Buffer.concat([Buffer.from(`P6\n${W} ${H}\n255\n`), pixels(([r, g, b]) => [r, g, b])]) },
    { relpath: 'paintbrush.pcx', data: pcx() },
    { relpath: 'khronos.ktx', data: ktx() },
    { relpath: 'powervr.pvr', data: pvr() },
    { relpath: 'valve.vtf', data: vtf() },
  ]
  const pack = await uploadPack(api, name, files)
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    expect(assets.map((a) => a.path).sort()).toEqual(files.map((f) => f.relpath).sort())
    for (const a of assets) {
      expect(a.kind, a.path).toBe('texture')
      const detail = (await (await api.get(`/api/assets/${a.id}`)).json()) as { has_thumb?: boolean; meta: Record<string, unknown> }
      expect(detail.has_thumb, a.path).toBe(true)
      expect([detail.meta.width, detail.meta.height], a.path).toEqual([W, H])
    }

    await openPack(page, pack.id, name, '&sort=name')
    const vtfAsset = assets.find((a) => a.path === 'valve.vtf')!
    await tile(page, vtfAsset.name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await expect(panel.locator('img').first()).toBeVisible()
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
