import { assetsOf, HOARD_HEADERS, uploadPack, waitReady } from './support/api'
import { png, zip } from './support/files'
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

/** GIMP: one uncompressed RGB layer of the gradient, laid out by hand (offsets in the comments). */
function xcf(): Buffer {
  const be = (...v: number[]) => Buffer.concat(v.map((n) => { const b = Buffer.alloc(4); b.writeUInt32BE(n); return b }))
  return Buffer.concat([
    Buffer.from('gimp xcf v003\0'), // 0
    be(W, H, 0), // 14: size, RGB
    be(17, 1), Buffer.from([0]), be(0, 0), // 26: no compression, end of properties
    be(55, 0, 0), // 43: the layer, end of layers, end of channels
    be(W, H, 0, 2), Buffer.from('L\0'), be(0, 0), // 55: the layer, named "L", no properties
    be(89, 0), // 81: its pixels, no mask
    be(W, H, 3, 109, 0), // 89: three bytes a pixel, one level
    be(W, H, 125, 0), // 109: one tile
    pixels(([r, g, b]) => [r, g, b]), // 125
  ])
}

/** A little-endian TIFF of `entries` (tag, type 3 short or 4 long, values), each value list over one placed after. */
function tiff(entries: [number, 3 | 4, number[]][], data: Buffer): { file: Buffer; dataAt: number } {
  const ifd = 8
  let extra = ifd + 2 + entries.length * 12 + 4
  const tail: Buffer[] = []
  const rows = entries.map(([tag, type, values]) => {
    const e = Buffer.alloc(12)
    e.writeUInt16LE(tag, 0)
    e.writeUInt16LE(type, 2)
    e.writeUInt32LE(values.length, 4)
    const packed = Buffer.concat(values.map((v) => (type === 3 ? u16(v) : u32(v))))
    if (packed.length <= 4) packed.copy(e, 8)
    else {
      e.writeUInt32LE(extra, 8)
      tail.push(packed)
      extra += packed.length
    }
    return e
  })
  const head = Buffer.concat([Buffer.from('II*\0'), u32(ifd), u16(entries.length), ...rows, u32(0), ...tail])
  return { file: Buffer.concat([head, data]), dataAt: head.length }
}

/** An uncompressed RGB TIFF of the gradient. */
function rgbTiff(): Buffer {
  const data = pixels(([r, g, b]) => [r, g, b])
  // The strip offset is known only once the header is laid out, which doesn't depend on its value.
  const entries = (at: number): [number, 3 | 4, number[]][] => [
    [256, 3, [W]],
    [257, 3, [H]],
    [258, 3, [8, 8, 8]],
    [259, 3, [1]],
    [262, 3, [2]],
    [273, 4, [at]],
    [277, 3, [3]],
    [278, 3, [H]],
    [279, 4, [data.length]],
  ]
  return tiff(entries(tiff(entries(0), data).dataAt), data).file
}

/** EPS with a binary header pointing at a TIFF preview after the PostScript. */
function eps(): Buffer {
  const ps = Buffer.from('%!PS-Adobe-3.0 EPSF-3.0\n%%BoundingBox: 0 0 8 8\n')
  const preview = rgbTiff()
  return Buffer.concat([Buffer.from([0xc5, 0xd0, 0xd3, 0xc6]), u32(30, ps.length, 0, 0, 30 + ps.length, preview.length), u16(0xffff), ps, preview])
}

/** A DNG-like raw: IFD0 naming the camera and pointing at a JPEG thumbnail. */
function dng(jpeg: Buffer): Buffer {
  // Header (8), then four entries (2 + 4 * 12 + 4 = 54), then the make at 62, the model at 70 and the JPEG at 78.
  const entry = (tag: number, type: number, n: number, value: number) => Buffer.concat([u16(tag), u16(type), u32(n, value)])
  return Buffer.concat([
    Buffer.from('II*\0'),
    u32(8),
    u16(4),
    entry(0x010f, 2, 6, 62),
    entry(0x0110, 2, 7, 70),
    entry(0x0201, 4, 1, 78),
    entry(0x0202, 4, 1, jpeg.length),
    u32(0),
    Buffer.from('Canon\0\0\0EOS R5\0\0'),
    jpeg,
  ])
}

// Formats Hoard reads itself or with the image library's smaller decoders, each a picture with a thumbnail.
test('icons, cursors, QOI, PNM, PCX, GIMP and older GPU textures are pictures with thumbnails', async ({ page, api }) => {
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
    { relpath: 'layers.xcf', data: xcf() },
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

// Files whose own contents Hoard can't draw, shown by the picture kept inside them.
test('Procreate, Affinity, EPS and camera raw files show the picture inside', async ({ page, api }) => {
  // A JPEG from the browser, for the raw file to carry.
  await page.goto('/')
  const jpeg = Buffer.from(
    await page.evaluate(([w, h]) => {
      const c = document.createElement('canvas')
      c.width = w
      c.height = h
      const g = c.getContext('2d')!
      g.fillStyle = '#4080c0'
      g.fillRect(0, 0, w, h)
      return c.toDataURL('image/jpeg').split(',')[1]
    }, [W, H]),
    'base64',
  )
  const name = `E2E Previews ${unique()}`
  const picture = png(W, H, color)
  const files = [
    { relpath: 'painting.procreate', data: zip([{ path: 'Document.archive', data: Buffer.from('bplist00') }, { path: 'QuickLook/Thumbnail.png', data: picture }]) },
    { relpath: 'photo.afphoto', data: Buffer.concat([Buffer.from('\0\xffKInAf'), Buffer.alloc(64, 7), picture, Buffer.alloc(32, 9)]) },
    { relpath: 'logo.eps', data: eps() },
    { relpath: 'shot.dng', data: dng(jpeg) },
  ]
  const pack = await uploadPack(api, name, files)
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    expect(assets.map((a) => a.path).sort()).toEqual(files.map((f) => f.relpath).sort())
    for (const a of assets) {
      expect(a.kind, a.path).toBe(a.path === 'logo.eps' ? 'vector' : 'texture')
      const detail = (await (await api.get(`/api/assets/${a.id}`)).json()) as { has_thumb?: boolean; meta: Record<string, unknown> }
      expect(detail.has_thumb, a.path).toBe(true)
      expect([detail.meta.width, detail.meta.height], a.path).toEqual([W, H])
      expect(detail.meta.embedded_preview, a.path).toBe(true)
      if (a.path === 'shot.dng') expect(detail.meta.camera).toBe('Canon EOS R5')
    }
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

test('with AVIF on, a browser that reads it gets thumbnails as AVIF, made once the stored one has been sent', async ({ page, api }) => {
  test.setTimeout(120_000)
  // Smooth color with detail, like a render: its thumbnail is a JPEG that AVIF shrinks.
  const photo = png(256, 256, (x, y) => {
    const ripple = Math.sin(x / 11) * Math.cos(y / 15) * 40
    const c = (v: number) => Math.max(0, Math.min(255, Math.round(v * 200 + ripple)))
    return [c(x / 256), c(y / 256), c(1 - (x * y) / 65_536), 255]
  })
  const pack = await uploadPack(api, `E2E AVIF ${unique()}`, [{ relpath: 'render.png', data: photo }])
  await waitReady(api, [pack.id])
  const [render] = await assetsOf(api, pack.id)
  const thumb = (accept: string) => api.get(`/api/assets/${render.id}/thumb`, { headers: { Accept: accept } })

  await page.goto('/#settings/performance')
  const avif = page.getByRole('checkbox', { name: 'Send thumbnails as AVIF' })
  // Each choice says what browsers get: the stored JPEG, then the AVIF made at the same sharpness.
  const choice = (name: string) =>
    page.getByRole('radiogroup', { name: 'Thumbnails' }).locator('label').filter({ has: page.getByText(name, { exact: true }) })
  await expect(avif).not.toBeChecked()
  await expect(choice('Balanced')).toContainText('JPEG, quality 82. Hard to tell')
  await avif.check()
  await expect.poll(async () => (await (await api.get('/api/admin/settings')).json()).options.thumb_avif.value).toBe(true)
  await expect(choice('Balanced')).toContainText('AVIF, quality 68. Hard to tell')
  await expect(choice('Sharper')).toContainText('AVIF, quality 80.')
  await expect(choice('Lossless')).toHaveText(/^Lossless\s*PNG, exactly as made\. Several times larger, and never sent as AVIF\.$/)
  try {
    // The stored picture goes out while the AVIF is made, not to be kept by the browser.
    const first = await thumb('image/avif,image/webp,*/*')
    expect(first.headers()['content-type']).toBe('image/jpeg')
    expect(first.headers()['cache-control']).toContain('no-cache')
    expect(first.headers()['vary']).toBe('Accept')
    let made = first
    await expect
      .poll(async () => (made = await thumb('image/avif,image/webp,*/*')).headers()['content-type'], { timeout: 60_000 })
      .toBe('image/avif')
    expect(made.headers()['vary']).toBe('Accept')
    const body = await made.body()
    expect(body.subarray(4, 12).toString('latin1')).toBe('ftypavif')
    expect(body.length).toBeLessThan((await first.body()).length)

    // One that doesn't read AVIF still gets the JPEG.
    expect((await thumb('image/webp,*/*')).headers()['content-type']).toBe('image/jpeg')
  } finally {
    await api.put('/api/admin/settings', { headers: HOARD_HEADERS, data: { thumb_avif: null } })
  }
  await page.reload()
  await expect(page.getByRole('checkbox', { name: 'Send thumbnails as AVIF' })).not.toBeChecked()
})
