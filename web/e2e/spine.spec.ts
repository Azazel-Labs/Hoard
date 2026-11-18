import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { png } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

/**
 * A Spine skeleton: a red square (a region) over a bigger blue one (a mesh), both on the root bone, which turns a quarter
 * turn over a second at 24 frames a second.
 */
const skeleton = {
  skeleton: { hash: 'e2e', spine: '4.2.18', x: -40, y: -40, width: 80, height: 80, fps: 24 },
  bones: [{ name: 'root' }],
  slots: [
    { name: 'back', bone: 'root', attachment: 'back' },
    { name: 'body', bone: 'root', attachment: 'body' },
  ],
  skins: [
    {
      name: 'default',
      attachments: {
        back: {
          back: {
            type: 'mesh',
            uvs: [0, 1, 1, 1, 1, 0, 0, 0],
            triangles: [0, 1, 2, 0, 2, 3],
            vertices: [-40, -40, 40, -40, 40, 40, -40, 40],
            hull: 4,
            width: 80,
            height: 80,
          },
        },
        body: { body: { width: 40, height: 40 } },
      },
    },
  ],
  animations: {
    idle: { bones: { root: { rotate: [{ value: 0 }, { time: 1, value: 90 }] } } },
    still: {},
  },
}

const atlas = ['hero.png', 'size: 80,40', 'filter: Linear,Linear', 'body', '  bounds: 0,0,40,40', 'back', '  bounds: 40,0,40,40', ''].join('\n')

test('a Spine skeleton plays on its atlas in the details panel and gets a thumbnail drawn by the browser', async ({ page, api }) => {
  test.setTimeout(120_000)
  const name = `E2E Spine ${unique()}`
  const sheet = png(80, 40, (x) => (x < 40 ? [255, 0, 0, 255] : [0, 0, 255, 255]))
  const pack = await uploadPack(api, name, [
    { relpath: 'hero.json', data: Buffer.from(JSON.stringify(skeleton)) },
    { relpath: 'hero.atlas', data: Buffer.from(atlas) },
    { relpath: 'hero.png', data: sheet },
  ])
  try {
    await waitReady(api, [pack.id])
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path === 'hero.json')?.kind, { timeout: 60_000 })
      .toBe('animation')
    const hero = assets.find((a) => a.path === 'hero.json')!
    const detail = async () =>
      (await (await api.get(`/api/assets/${hero.id}`)).json()) as {
        thumb_source: string | null
        meta: Record<string, unknown>
        links: { path: string; rel: string }[]
        tags: { tag: string }[]
      }
    const d = await detail()
    expect(d.meta).toMatchObject({ spine: '4.2.18', bones: 1, slots: 2, meshes: 1, fps: 24, animations: ['idle', 'still'], duration_secs: 1, width: 80, height: 80 })
    expect(d.tags.map((t) => t.tag)).toContain('spine')
    // It's drawn from the atlas beside it, which goes with it when it's downloaded.
    expect(d.links.map((l) => [l.path, l.rel])).toContainEqual(['hero.atlas', 'library'])

    // Opened, it plays; and the browser draws its thumbnail, which the server can't.
    await openPack(page, pack.id, name)
    await tile(page, 'Hero').filter({ hasText: '.json' }).click()
    const player = page.locator('.skeleton2d')
    await expect(player.locator('canvas')).toBeVisible()
    await expect(player.getByTestId('frame-count')).toHaveText(/^\d+ \/ 24$/)
    await expect(player.getByRole('combobox')).toHaveValue('default\u0000idle')
    // Red in the middle (the region), blue further out (the mesh, which the red square never reaches as it turns).
    const pixel = (fx: number) =>
      player.locator('canvas').evaluate(
        (c: HTMLCanvasElement, fx) => {
          const x = Math.round(c.width / 2 + fx * Math.min(c.width, c.height))
          return [...c.getContext('2d')!.getImageData(x, Math.round(c.height / 2), 1, 1).data]
        },
        fx,
      )
    await expect.poll(() => pixel(0)).toEqual([255, 0, 0, 255])
    await expect.poll(() => pixel(0.27)).toEqual([0, 0, 255, 255])
    // Stepped, frame by frame.
    await player.getByRole('button', { name: /next/i }).click()
    await expect(player.getByTestId('frame-count')).toHaveText(/^\d+ \/ 24$/)
    // The other animation.
    await player.getByRole('combobox').selectOption({ label: 'still' })
    await expect(player.getByTestId('frame-count')).toHaveText('1 / 1')
    await expect.poll(() => pixel(0)).toEqual([255, 0, 0, 255])

    await expect.poll(async () => (await detail()).thumb_source, { timeout: 30_000 }).toBe('browser')
    const thumb = await api.get(`/api/assets/${hero.id}/thumb`)
    expect(thumb.status()).toBe(200)
    expect(thumb.headers()['content-type']).toMatch(/^image\//)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
