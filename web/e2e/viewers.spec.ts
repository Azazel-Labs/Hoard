import { LARGE, LARGE_PX, MODELS, TEXTURES } from './support/seed'
import { paintedCube3mf, partsGltf } from './support/files'
import { assetsOf, signedIn, uploadPack, waitIdle, waitReady } from './support/api'
import { doubleClick, expect, openPack, test, tile, unique } from './support/test'

// Double-clicking a tile opens its full-screen viewer; the app spots the second click by where and when it lands, so
// these use real mouse double-clicks.
test.describe('full-screen viewers', () => {
  test('a texture opens the image viewer; Escape closes it, then the panel', async ({ page, seed }) => {
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    await doubleClick(page, tile(page, 'Brick wall'))
    const viewer = page.getByRole('dialog', { name: 'Inspect Brick wall' })
    await expect(viewer).toBeVisible()
    await expect(viewer.getByRole('group', { name: 'Zoom' })).toBeVisible()
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await expect(panel.getByRole('heading', { name: 'Brick wall' })).toBeVisible()
    await page.keyboard.press('Escape')
    await expect(panel).toHaveCount(0)
  })

  test('an OBJ model opens the 3D viewer and renders', async ({ page, seed }) => {
    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await doubleClick(page, tile(page, 'Crate'))
    const viewer = page.getByRole('dialog', { name: 'View Crate in 3D' })
    await expect(viewer).toBeVisible()
    await expect(viewer.getByText(/12 tris · 1 mesh/)).toBeVisible({ timeout: 20_000 })
    await expect(viewer.getByText(/Couldn't show this model/)).toHaveCount(0)
    await expect(viewer.locator('canvas')).toBeVisible()
    await viewer.getByRole('button', { name: 'Close (Esc)' }).click()
    await expect(viewer).toBeHidden()
  })

  // The server reads it, filaments and painting included, and the viewer shows the GLB it makes.
  test('a painted 3MF print shows its filament colours', async ({ page, baseURL }) => {
    const api = await signedIn(baseURL!)
    const pack = await uploadPack(api, 'E2E Painted 3MF', [{ relpath: 'Painted_Cube.3mf', data: paintedCube3mf() }])
    await waitReady(api, [pack.id])
    await waitIdle(api)
    const [model] = await assetsOf(api, pack.id)
    expect(model.kind).toBe('model')
    const full = (await (await api.get(`/api/assets/${model.id}`)).json()) as { meta: Record<string, unknown> }
    expect(full.meta).toMatchObject({ triangles: 12, colors: 2, application: 'BambuStudio-01.10.00.00' })

    await openPack(page, pack.id, 'E2E Painted 3MF')
    await doubleClick(page, tile(page, 'Painted cube'))
    const viewer = page.getByRole('dialog', { name: 'View Painted cube in 3D' })
    await expect(viewer.getByText(/12 tris · 2 meshes/)).toBeVisible({ timeout: 30_000 })
    await expect(viewer.getByText(/Couldn't show this model/)).toHaveCount(0)
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
    await api.delete(`/api/packs/${pack.id}`)
  })

  // Parts that share a material are drawn as one mesh, so a model of thousands turns smoothly; they still count, and
  // still show apart in the geometry view.
  test('a model of many parts counts each part', async ({ page, baseURL }) => {
    const api = await signedIn(baseURL!)
    const name = `E2E Parts ${unique()}`
    const pack = await uploadPack(api, name, [{ relpath: 'Parts.gltf', data: partsGltf(4) }])
    await waitReady(api, [pack.id])
    await openPack(page, pack.id, name)
    await doubleClick(page, tile(page, 'Parts'))
    const viewer = page.getByRole('dialog', { name: 'View Parts in 3D' })
    await expect(viewer.getByText(/8 tris · 4 meshes · 1 material/)).toBeVisible({ timeout: 20_000 })
    await page.keyboard.press('0')
    const menu = viewer.getByRole('navigation', { name: 'View' })
    await expect(menu.getByRole('button', { name: 'Geometries', exact: true })).toHaveAttribute('aria-pressed', 'true')
    await page.keyboard.press('i')
    const info = viewer.getByRole('region', { name: 'Scene information' })
    const row = (label: string) => info.locator('dl > div').filter({ has: page.getByText(label, { exact: true }) }).locator('dd')
    await expect(row('Geometries')).toHaveText('4')
    await expect(row('Triangles')).toHaveText('8')
    await expect(viewer.getByText(/Couldn't show this model/)).toHaveCount(0)
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
    await api.delete(`/api/packs/${pack.id}`)
  })

  test('the 3D viewer shows a model other ways, and its scene information', async ({ page, seed }) => {
    test.slow()
    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await doubleClick(page, tile(page, 'Crate'))
    const viewer = page.getByRole('dialog', { name: 'View Crate in 3D' })
    await expect(viewer.getByText(/12 tris · 1 mesh/)).toBeVisible({ timeout: 20_000 })
    const menu = viewer.getByRole('navigation', { name: 'View' })
    const mode = (name: string) => menu.getByRole('button', { name, exact: true })
    await expect(mode('Final render')).toHaveAttribute('aria-pressed', 'true')
    // The crate has a colour map and nothing else, so only that map is offered.
    await expect(mode('Base color')).toBeVisible()
    await expect(mode('Roughness')).toHaveCount(0)
    await expect(mode('Normal')).toHaveCount(0)

    await mode('UV checker').click()
    await expect(mode('UV checker')).toHaveAttribute('aria-pressed', 'true')
    await expect(mode('Final render')).toHaveAttribute('aria-pressed', 'false')
    // The textures and light switches belong to the model's own look.
    await expect(viewer.getByRole('button', { name: 'Textures', exact: true })).toBeDisabled()
    await expect(viewer.getByRole('button', { name: 'Light', exact: true })).toBeDisabled()

    await page.keyboard.press('3')
    await expect(mode('Base color')).toHaveAttribute('aria-pressed', 'true')
    await page.keyboard.press('w')
    await expect(mode('Wireframe')).toHaveAttribute('aria-pressed', 'true')

    await page.keyboard.press('i')
    const info = viewer.getByRole('region', { name: 'Scene information' })
    const row = (name: string) => info.locator('dl > div').filter({ has: page.getByText(name, { exact: true }) }).locator('dd')
    await expect(row('Geometries')).toHaveText('1')
    await expect(row('Materials')).toHaveText('1')
    await expect(row('Textures')).toHaveText('1')
    await expect(row('Triangles')).toHaveText('12')
    await expect(row('Vertices')).toHaveText('36')
    // OBJ says nothing of its units.
    await expect(row('Height')).toHaveText('2 units')
    await expect(row('Width')).toHaveText('2 units')
    await info.getByRole('button', { name: 'Close scene information' }).click()
    await expect(info).toHaveCount(0)

    await page.keyboard.press('1')
    await expect(mode('Final render')).toHaveAttribute('aria-pressed', 'true')
    await expect(viewer.getByRole('button', { name: 'Textures', exact: true })).toBeEnabled()
    await expect(viewer.getByText(/Couldn't show this model/)).toHaveCount(0)
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
  })

  test('a glTF model opens the 3D viewer too', async ({ page, seed }) => {
    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await doubleClick(page, tile(page, 'Triangle'))
    const viewer = page.getByRole('dialog', { name: 'View Triangle in 3D' })
    await expect(viewer.getByText(/1 tris · 1 mesh/)).toBeVisible({ timeout: 20_000 })
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
  })

  test('JSON opens as a folding tree, and pretty-printed', async ({ page, seed }) => {
    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await doubleClick(page, tile(page, 'Level'))
    const viewer = page.getByRole('dialog', { name: 'Read Level' })
    await expect(viewer).toBeVisible()
    const contents = viewer.getByLabel('Level contents')
    await expect(viewer.getByRole('button', { name: 'Tree' })).toHaveAttribute('aria-pressed', 'true')
    await expect(contents.getByText('"enemies"')).toBeVisible()
    await expect(contents.getByText('"slime"')).toBeVisible()

    await viewer.getByRole('button', { name: 'Collapse all' }).click()
    await expect(contents.getByText('"slime"')).toBeHidden()
    await expect(contents.getByRole('button', { name: 'Unfold', exact: true }).first()).toHaveAttribute('aria-expanded', 'false')
    await viewer.getByRole('button', { name: 'Expand all' }).click()
    await expect(contents.getByText('"slime"')).toBeVisible()

    await viewer.getByRole('button', { name: 'Pretty' }).click()
    await expect(viewer.getByRole('button', { name: 'Pretty' })).toHaveAttribute('aria-pressed', 'true')
    await expect(contents).toContainText('"name": "Forest clearing"')
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
  })

  test('plain text opens with its lines', async ({ page, seed }) => {
    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await doubleClick(page, tile(page, 'Notes'))
    const viewer = page.getByRole('dialog', { name: 'Read Notes' })
    await expect(viewer.getByLabel('Notes contents')).toContainText('Notes for the e2e pack')
    await expect(viewer.getByRole('group', { name: 'Show as' })).toHaveCount(0)
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
  })

  test('markdown reads as a document, in the panel and full screen, and shows its source on request', async ({ page, api }) => {
    const name = `E2E Readme ${unique()}`
    const md = [
      '# Forest Kit',
      '',
      'Trees for **stylised** games. See [the store](https://example.com/forest).',
      '',
      '<img src=x onerror="window.pwned=1">',
      '',
      '| File | Tris |',
      '|------|------|',
      '| tree | 640 |',
    ].join('\n')
    const pack = await uploadPack(api, name, [{ relpath: 'README.md', data: Buffer.from(md) }])
    await waitReady(api, [pack.id])
    await openPack(page, pack.id, name, '&sort=name')
    await page.locator('.grid button.hit').first().click()

    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await expect(panel.getByRole('heading', { name: 'Forest Kit' })).toBeVisible()
    await panel.getByRole('button', { name: /^Read .* full screen$/ }).click()
    const viewer = page.getByRole('dialog', { name: /^Read / })
    const body = viewer.getByLabel(/contents$/)
    await expect(body.getByRole('heading', { name: 'Forest Kit' })).toBeVisible()
    await expect(body.getByRole('cell', { name: '640' })).toBeVisible()
    const link = body.getByRole('link', { name: 'the store' })
    await expect(link).toHaveAttribute('target', '_blank')
    await expect(link).toHaveAttribute('rel', 'noopener noreferrer')
    // HTML in the file is shown as text, never run.
    await expect(body).toContainText('<img src=x onerror=')
    await expect(body.locator('img')).toHaveCount(0)
    expect(await page.evaluate(() => (window as { pwned?: number }).pwned)).toBeUndefined()

    await page.keyboard.press('r')
    await expect(viewer.getByRole('button', { name: 'Source' })).toHaveAttribute('aria-pressed', 'true')
    await expect(body).toContainText('# Forest Kit')
    await expect(body.getByRole('heading')).toHaveCount(0)
    await page.keyboard.press('d')
    await expect(body.getByRole('heading', { name: 'Forest Kit' })).toBeVisible()
  })
})

test.describe('light previews and lazy viewers', () => {
  test('a big picture opens as a light preview, and Full resolution loads the file', async ({ page, seed }) => {
    await openPack(page, seed.packs[LARGE], LARGE)
    await doubleClick(page, tile(page, 'Big mural'))
    const viewer = page.getByRole('dialog', { name: 'Inspect Big mural' })
    await expect(viewer.getByText(/preview 2048×2048/)).toBeVisible({ timeout: 20_000 })
    await expect(viewer.getByText(new RegExp(`${LARGE_PX}×${LARGE_PX}`)).first()).toBeVisible()
    const full = page.waitForRequest((r) => /\/api\/assets\/[\w-]+\/file/.test(r.url()))
    await viewer.getByRole('button', { name: 'Full resolution' }).click()
    await full
    await expect(viewer.getByRole('group', { name: 'Resolution' })).toBeHidden({ timeout: 20_000 })
  })

  test('3D code loads only when a model is opened', async ({ page, seed }) => {
    const scripts: string[] = []
    page.on('request', (r) => {
      if (r.resourceType() === 'script') scripts.push(new URL(r.url()).pathname)
    })
    const threeish = () => scripts.filter((s) => /\/(materialMaps|modelStage|ModelInspector)-/.test(s))
    await openPack(page, seed.packs[TEXTURES], TEXTURES, '&sort=name')
    await tile(page, 'Brick wall').click()
    await expect(page.getByRole('complementary', { name: 'Asset details' })).toBeVisible()
    expect(threeish(), '3D code loaded for the library and a texture').toEqual([])

    await openPack(page, seed.packs[MODELS], MODELS, '&sort=name')
    await doubleClick(page, tile(page, 'Crate'))
    await expect(page.getByRole('dialog', { name: 'View Crate in 3D' })).toBeVisible()
    await expect.poll(() => threeish().length).toBeGreaterThan(0)
  })
})
