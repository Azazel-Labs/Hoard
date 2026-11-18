import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { png, zip } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

const still = <T>(k: T) => ({ a: 0, k })

/** A second-long Lottie animation: a red square, and an image from `images/dot.png` beside it. */
function animation(name: string) {
  const transform = { o: still(100), r: still(0), p: still([0, 0, 0]), a: still([0, 0, 0]), s: still([100, 100, 100]) }
  return {
    v: '5.7.4',
    nm: name,
    fr: 30,
    ip: 0,
    op: 30,
    w: 100,
    h: 100,
    assets: [{ id: 'img_0', w: 8, h: 8, u: 'images/', p: 'dot.png', e: 0 }],
    layers: [
      { ty: 2, ind: 1, refId: 'img_0', ip: 0, op: 30, st: 0, ks: transform },
      {
        ty: 4,
        ind: 2,
        ip: 0,
        op: 30,
        st: 0,
        ks: transform,
        shapes: [
          {
            ty: 'gr',
            it: [
              { ty: 'rc', p: still([50, 50]), s: still([80, 80]), r: still(0) },
              { ty: 'fl', c: still([1, 0, 0, 1]), o: still(100) },
              { ty: 'tr', p: still([0, 0]), a: still([0, 0]), s: still([100, 100]), r: still(0), o: still(100) },
            ],
          },
        ],
      },
    ],
  }
}

test('a Lottie animation plays in the details panel and gets a thumbnail drawn by the browser', async ({ page, api }) => {
  test.setTimeout(120_000)
  const name = `E2E Lottie ${unique()}`
  const dot = png(8, 8, () => [40, 90, 220, 255])
  const dotlottie = zip([
    { path: 'manifest.json', data: Buffer.from(JSON.stringify({ version: '1', animations: [{ id: 'spin' }] })) },
    { path: 'animations/spin.json', data: Buffer.from(JSON.stringify(animation('Spin'))) },
    { path: 'images/dot.png', data: dot },
  ])
  const pack = await uploadPack(api, name, [
    { relpath: 'ui/loader.json', data: Buffer.from(JSON.stringify(animation('Loader'))) },
    { relpath: 'ui/images/dot.png', data: dot },
    { relpath: 'spin.lottie', data: dotlottie },
  ])
  try {
    await waitReady(api, [pack.id])
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path === 'ui/loader.json')?.kind, { timeout: 60_000 })
      .toBe('vector')
    const at = (path: string) => assets.find((a) => a.path === path)!
    const detail = async (id: string) =>
      (await (await api.get(`/api/assets/${id}`)).json()) as { kind: string; thumb_source: string | null; meta: Record<string, unknown> }
    expect((await detail(at('ui/loader.json').id)).meta).toMatchObject({
      lottie: '5.7.4',
      lottie_name: 'Loader',
      width: 100,
      height: 100,
      fps: 30,
      frames: 30,
      duration_secs: 1,
      layers: 2,
    })
    expect(at('spin.lottie').kind).toBe('vector')
    expect((await detail(at('spin.lottie').id)).meta).toMatchObject({ lottie: '5.7.4', lottie_name: 'Spin', frames: 30 })

    // A .lottie is played as its animation, its images inlined; a .json as it is.
    const spin = await (await api.get(`/api/assets/${at('spin.lottie').id}/lottie.json`)).json()
    expect(spin.assets[0].p).toMatch(/^data:image\/png;base64,/)
    const loader = await (await api.get(`/api/assets/${at('ui/loader.json').id}/lottie.json`)).json()
    expect(loader.assets[0]).toMatchObject({ u: 'images/', p: 'dot.png' })
    expect((await api.get(`/api/assets/${at('ui/images/dot.png').id}/lottie.json`)).status()).toBe(404)

    // Opened, it plays; and the browser draws its thumbnail, which the server can't.
    await openPack(page, pack.id, name)
    await tile(page, 'Loader').click()
    const player = page.locator('.lottie')
    await expect(player.locator('canvas')).toBeVisible()
    await expect(player.getByTestId('frame-count')).toHaveText(/^\d+ \/ 30$/)
    // The square is drawn red, in the middle of the stage.
    const middle = () =>
      player.locator('canvas').evaluate((c: HTMLCanvasElement) => [...c.getContext('2d')!.getImageData(c.width / 2, c.height / 2, 1, 1).data])
    await expect.poll(middle).toEqual([255, 0, 0, 255])
    await expect
      .poll(async () => (await detail(at('ui/loader.json').id)).thumb_source, { timeout: 30_000 })
      .toBe('browser')
    const thumb = await api.get(`/api/assets/${at('ui/loader.json').id}/thumb`)
    expect(thumb.status()).toBe(200)
    expect(thumb.headers()['content-type']).toMatch(/^image\//)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
