import { addTags, assetsOf, uploadPack, waitReady } from './support/api'
import { apng, aseprite, png } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

const COLOURS: [number, number, number, number][] = [
  [230, 60, 60, 255],
  [60, 200, 90, 255],
  [70, 110, 240, 255],
  [240, 200, 40, 255],
  [200, 80, 220, 255],
]

test('pictures that move play: Aseprite frames by their times and tags, sprite sheets cut into a grid, animated PNGs', async ({
  page,
  api,
}) => {
  const name = `E2E Sprites ${unique()}`
  const walk = aseprite(8, [100, 150, 100, 100], (f) => COLOURS[f], [
    ['walk', 0, 3, 0],
    ['blink', 1, 2, 1],
  ])
  // Three frames across and two down, 16 px each, with see-through gutters; the last cell is empty.
  const coins = png(48, 32, (x, y) => {
    const [c, r] = [Math.floor(x / 16), Math.floor(y / 16)]
    const inside = x % 16 >= 3 && x % 16 < 13 && y % 16 >= 3 && y % 16 < 13
    return inside && r * 3 + c < 5 ? COLOURS[r * 3 + c] : [0, 0, 0, 0]
  })
  const blink = apng(8, 8, 3, 100, (f) => COLOURS[f])
  const pack = await uploadPack(api, name, [
    { relpath: 'walk.aseprite', data: walk },
    { relpath: 'coins.png', data: coins },
    { relpath: 'blink.png', data: blink },
  ])
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const at = (path: string) => assets.find((a) => a.path === path)!
    const meta = async (id: string) => ((await (await api.get(`/api/assets/${id}`)).json()) as { meta: Record<string, unknown> }).meta

    const ase = await meta(at('walk.aseprite').id)
    expect(ase.frames).toBe(4)
    expect(ase.frame_ms).toEqual([100, 150, 100, 100])
    expect(ase.duration_ms).toBe(450)
    expect(ase.animations).toEqual([
      { name: 'walk', from: 0, to: 3, direction: 'forward' },
      { name: 'blink', from: 1, to: 2, direction: 'reverse' },
    ])
    const sheet = await api.get(`/api/assets/${at('walk.aseprite').id}/frames.png`)
    expect(sheet.status()).toBe(200)
    expect(sheet.headers()['content-type']).toBe('image/png')
    expect((await api.get(`/api/assets/${at('coins.png').id}/frames.png`)).status()).toBe(404)

    const moving = await meta(at('blink.png').id)
    expect(moving.frames).toBe(3)
    expect(moving.duration_ms).toBe(300)
    expect((await meta(at('coins.png').id)).frames).toBeUndefined()
    await addTags(api, at('coins.png').id, ['spritesheet'])

    await openPack(page, pack.id, name, '&sort=name')
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const count = panel.getByTestId('frame-count')

      // An Aseprite animation plays as soon as it's shown, through all its frames or one of its tags.
    await tile(page, at('walk.aseprite').name).click()
    await expect(panel.getByRole('checkbox', { name: 'Play as animation' })).toBeChecked()
    // The suite asks for reduced motion, so it waits to be played.
    await expect(count).toHaveText('1 / 4')
    await panel.getByRole('button', { name: 'Play animation' }).click()
    await expect(count).not.toHaveText('1 / 4')
    await panel.getByRole('button', { name: 'Pause animation' }).click()
    const before = await count.textContent()
    await panel.getByRole('button', { name: 'Next frame' }).click()
    await expect(count).not.toHaveText(before!)
    await panel.getByRole('combobox', { name: 'Animation' }).selectOption('blink')
    await expect(count).toHaveText('1 / 2')
    await panel.getByRole('checkbox', { name: 'Play as animation' }).uncheck()
    await expect(panel.locator('button.zoomable img')).toBeVisible()

    // A sprite sheet plays when asked, cut where its gutters are, skipping the empty cell.
    await tile(page, at('coins.png').name).click()
    const play = panel.getByRole('checkbox', { name: 'Play as animation' })
    await expect(play).not.toBeChecked()
    await play.check()
    await expect(count).toHaveText(/ \/ 5$/)
    await expect(panel.getByRole('spinbutton', { name: 'Columns' })).toHaveValue('3')
    await expect(panel.getByRole('spinbutton', { name: 'Rows' })).toHaveValue('2')
    // Cut differently, every cell is a frame.
    await panel.getByRole('spinbutton', { name: 'Rows' }).fill('1')
    await panel.getByRole('spinbutton', { name: 'Rows' }).press('Enter')
    await expect(count).toHaveText(/ \/ 3$/)

    // An animated PNG is shown as itself, so the browser plays it.
    await tile(page, at('blink.png').name).click()
    await expect(panel.getByRole('checkbox', { name: 'Play as animation' })).toHaveCount(0)
    await expect(panel.locator('button.zoomable img')).toHaveAttribute('src', new RegExp(`/api/assets/${at('blink.png').id}/file$`))
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
