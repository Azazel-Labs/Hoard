import { type Asset, assetsOf, uploadPack, waitReady } from './support/api'
import { triangleGltf } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

const OBJ = Buffer.from('o triangle\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n')

// A model a pack has in several formats, each in a folder named after it as Kenney's kits do, is one asset: the
// format Hoard reads best is shown, and the others are listed with it and downloadable from "Download as".
test('an asset in several formats is shown once, its other formats beside it', async ({ page, api }) => {
  const name = `E2E Formats ${unique()}`
  const pack = await uploadPack(api, name, [
    { relpath: 'Models/GLTF format/wedge.gltf', data: triangleGltf() },
    { relpath: 'Models/OBJ format/wedge.obj', data: OBJ },
    { relpath: 'Models/OBJ format/ramp.obj', data: OBJ },
  ])
  try {
    await waitReady(api, [pack.id])
    const assets: Asset[] = await assetsOf(api, pack.id)
    const at = (path: string) => assets.find((a) => a.path === path)!
    const glb = at('GLTF format/wedge.glb')
    const obj = at('OBJ format/wedge.obj')
    const links = async (id: string) =>
      ((await (await api.get(`/api/assets/${id}`)).json()) as { links: { id: string; rel: string }[] }).links
    await expect
      .poll(async () => (await links(glb.id)).filter((l) => l.rel === 'format').map((l) => l.id), { timeout: 60_000 })
      .toEqual([obj.id])

    const shown = await api.get('/api/assets', { params: { pack: pack.id, limit: 500, facets: false, hide_supporting: true } })
    const body = (await shown.json()) as { items: Asset[]; supporting_hidden?: number }
    expect(body.items.map((a) => a.path).sort()).toEqual(['GLTF format/wedge.glb', 'OBJ format/ramp.obj'])
    expect(body.supporting_hidden).toBe(1)

    await openPack(page, pack.id, name, '&sort=name')
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await tile(page, glb.name).click()
    const others = panel.locator('section', { has: page.getByRole('heading', { name: 'Other formats', exact: true }) })
    await expect(others.getByRole('button')).toHaveCount(1)
    await expect(others.getByRole('button')).toContainText('OBJ')

    // The pack's own OBJ downloads as it is, rather than converted.
    await panel.getByLabel('Download as').selectOption(`file:${obj.id}`)
    await expect(panel.locator('.asrow a.btn')).toHaveAttribute('href', `/api/assets/${obj.id}/file?download=1`)

    await others.getByRole('button').click()
    await expect(panel.getByRole('definition').filter({ hasText: 'wedge.obj' })).toHaveText('OBJ format/wedge.obj')
    const shownAs = panel.locator('section', { has: page.getByRole('heading', { name: 'Shown in the library as', exact: true }) })
    await expect(shownAs.getByRole('button')).toContainText('GLB')
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
