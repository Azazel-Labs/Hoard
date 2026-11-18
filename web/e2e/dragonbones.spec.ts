import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { png } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

/**
 * A DragonBones skeleton: a red square (an image) over a bigger blue one (a mesh), both on the root bone, which turns a
 * quarter turn over 24 frames.
 */
const skeleton = {
  frameRate: 24,
  name: 'hero',
  version: '5.5',
  compatibleVersion: '5.5',
  armature: [
    {
      type: 'Armature',
      frameRate: 24,
      name: 'hero',
      aabb: { x: -40, y: -40, width: 80, height: 80 },
      bone: [{ name: 'root' }],
      slot: [
        { name: 'back', parent: 'root' },
        { name: 'body', parent: 'root' },
      ],
      skin: [
        {
          slot: [
            {
              name: 'back',
              display: [
                {
                  type: 'mesh',
                  name: 'back',
                  width: 80,
                  height: 80,
                  vertices: [-40, -40, 40, -40, 40, 40, -40, 40],
                  uvs: [0, 0, 1, 0, 1, 1, 0, 1],
                  triangles: [0, 1, 2, 0, 2, 3],
                },
              ],
            },
            { name: 'body', display: [{ name: 'body' }] },
          ],
        },
      ],
      animation: [
        { name: 'still', duration: 0 },
        {
          name: 'turn',
          duration: 24,
          bone: [
            {
              name: 'root',
              rotateFrame: [
                { duration: 24, tweenEasing: 0, rotate: 0 },
                { duration: 0, rotate: 90 },
              ],
            },
          ],
        },
      ],
      defaultActions: [{ gotoAndPlay: 'turn' }],
    },
  ],
}

const atlas = {
  name: 'hero',
  imagePath: 'hero_tex.png',
  width: 80,
  height: 40,
  SubTexture: [
    { name: 'body', x: 0, y: 0, width: 40, height: 40 },
    { name: 'back', x: 40, y: 0, width: 40, height: 40 },
  ],
}

test('a DragonBones skeleton plays on its atlas in the details panel and gets a thumbnail drawn by the browser', async ({ page, api }) => {
  test.setTimeout(120_000)
  const name = `E2E DragonBones ${unique()}`
  const sheet = png(80, 40, (x) => (x < 40 ? [255, 0, 0, 255] : [0, 0, 255, 255]))
  const pack = await uploadPack(api, name, [
    { relpath: 'hero_ske.json', data: Buffer.from(JSON.stringify(skeleton)) },
    { relpath: 'hero_tex.json', data: Buffer.from(JSON.stringify(atlas)) },
    { relpath: 'hero_tex.png', data: sheet },
  ])
  try {
    await waitReady(api, [pack.id])
    let assets: Asset[] = []
    await expect
      .poll(async () => (assets = await assetsOf(api, pack.id)).find((a) => a.path === 'hero_ske.json')?.kind, { timeout: 60_000 })
      .toBe('animation')
    const hero = assets.find((a) => a.path === 'hero_ske.json')!
    const detail = async () =>
      (await (await api.get(`/api/assets/${hero.id}`)).json()) as {
        thumb_source: string | null
        meta: Record<string, unknown>
        links: { path: string; rel: string }[]
        tags: { tag: string }[]
      }
    const d = await detail()
    expect(d.meta).toMatchObject({ dragonbones: '5.5', armature: 'hero', bones: 1, slots: 2, meshes: 1, fps: 24, animations: ['still', 'turn'], duration_secs: 1 })
    // It's drawn from the atlas beside it, which goes with it when it's downloaded.
    expect(d.links.map((l) => [l.path, l.rel]).sort()).toEqual([
      ['hero_tex.json', 'library'],
      ['hero_tex.png', 'texture'],
    ])

    // Opened, it plays; and the browser draws its thumbnail, which the server can't.
    await openPack(page, pack.id, name)
    await tile(page, 'Hero ske').click()
    const player = page.locator('.dragonbones')
    await expect(player.locator('canvas')).toBeVisible()
    await expect(player.getByTestId('frame-count')).toHaveText(/^\d+ \/ 24$/)
    await expect(player.getByRole('combobox')).toHaveValue('hero\u0000turn')
    // Red in the middle (the image), blue further out (the mesh, which the red square never reaches as it turns).
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
    // Paused and stepped, frame by frame.
    await player.getByRole('button', { name: /pause|play/i }).nth(0).click()
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
