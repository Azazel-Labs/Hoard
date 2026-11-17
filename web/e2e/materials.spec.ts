import { createHash } from 'node:crypto'
import { MEGASCANS } from './support/seed'
import { checker, crate, MEGASCANS_FOLDER, quadGltf, zipNames } from './support/files'
import type { Page } from '@playwright/test'
import { type Asset, assetsOf, signedIn, uploadPack, waitIdle, waitReady } from './support/api'
import { doubleClick, expect, openPack, test } from './support/test'

// The Megascans pack's four maps make one material, named after the listing like its model. Only the model wears it,
// so it's part of the model: one tile, and the material opens from the model.
test.describe('materials', () => {
  /** Opens the pack, the model, then the material it wears; returns the side panel. */
  async function openMaterial(page: Page, packId: string) {
    await openPack(page, packId, MEGASCANS, '&sort=name')
    await page.locator('.grid button.hit[aria-label*=" tris"]').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const worn = panel.locator('section', { has: page.getByRole('heading', { name: 'Material', exact: true }) })
    await worn.getByRole('button', { name: new RegExp(MEGASCANS) }).click()
    await expect(panel.locator('section', { has: page.getByRole('heading', { name: 'Maps' }) })).toBeVisible()
    return panel
  }

  test('a model and the one material it wears show as one tile, the maps hidden behind it', async ({ page, seed }) => {
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    const grid = page.locator('.grid button.hit')
    await expect(grid).toHaveCount(1)
    await expect(page.locator(`.grid button.hit[aria-label^="${MEGASCANS},"][aria-label*=" tris"]`)).toHaveCount(1)
    // The material, its maps (and the store's JSON) are there when asked for.
    await expect(page.getByText(/6 supporting files hidden/)).toBeVisible()
  })

  test('the material opens with a live preview, its maps by role, and a full-screen viewer', async ({ page, seed }) => {
    // A WebGL material on a busy runner: give it room.
    test.slow()
    const panel = await openMaterial(page, seed.packs[MEGASCANS])
    await expect(panel.getByRole('heading', { name: MEGASCANS })).toBeVisible()
    const maps = panel.locator('section', { has: page.getByRole('heading', { name: 'Maps' }) })
    for (const role of ['Color', 'Normal', 'Roughness', 'Ambient occlusion']) {
      await expect(maps.getByRole('button', { name: new RegExp(`^${role} `) })).toBeVisible()
    }
    await expect(panel.getByRole('group', { name: 'Shape' })).toBeVisible()

    await panel.getByRole('button', { name: 'Open the full-screen material viewer' }).click()
    const viewer = page.getByRole('dialog', { name: `View the material ${MEGASCANS}` })
    await expect(viewer).toBeVisible()
    const toggles = viewer.getByRole('group', { name: 'Maps' })
    await expect(toggles).toBeVisible({ timeout: 20_000 })
    await expect(viewer.getByText(/Couldn't show this material/)).toHaveCount(0)
    const normal = toggles.getByRole('button', { name: /Normal/ })
    await expect(normal).toHaveAttribute('aria-pressed', 'true')
    await normal.click()
    await expect(normal).toHaveAttribute('aria-pressed', 'false')
    await page.keyboard.press('Escape')
    await expect(viewer).toBeHidden()
  })

  test('double-clicking the material tile opens its viewer', async ({ page, seed }) => {
    // The side panel's live preview and the viewer both start WebGL, slow on a runner without a GPU.
    test.slow()
    // Shown as itself once supporting files are.
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await page.locator('.supporting').getByRole('button', { name: 'Show' }).click()
    await doubleClick(page, page.locator('.grid button.hit[aria-label*=" maps"]'))
    await expect(page.getByRole('dialog', { name: `View the material ${MEGASCANS}` })).toBeVisible({ timeout: 45_000 })
  })

  test('the model shows the material it wears', async ({ page, seed }) => {
    await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
    await page.locator('.grid button.hit[aria-label*=" tris"]').click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const worn = panel.locator('section', { has: page.getByRole('heading', { name: 'Material', exact: true }) })
    await expect(worn.getByRole('button', { name: new RegExp(MEGASCANS) })).toBeVisible()
    // Its own file name isn't what it's called any more.
    await expect(panel.getByRole('heading', { name: MEGASCANS })).toBeVisible()
    await expect(panel.getByText(MEGASCANS_FOLDER.replaceAll('_', ' '), { exact: true })).toHaveCount(0)
  })

  // The naming setting changes every download's names: what they're called by default is checked apart from it.
  test.describe('download names', () => {
    test.describe.configure({ mode: 'serial' })

    test('maps are called by their material and role, and go out named that way', async ({ page, seed, baseURL }) => {
      const api = await signedIn(baseURL!)
      const assets = await assetsOf(api, seed.packs[MEGASCANS])
      const model = assets.find((a) => a.kind === 'model')!
      const ao = assets.find((a) => a.path.endsWith('_AO.png'))!
      expect(ao.name).toBe(`${MEGASCANS} Ambient occlusion`)
      const stem = 'HistoricMiscellaneousStandWoodOld05'

      // The model's zip: its maps renamed in their folder, the model as it was.
      const names = zipNames(await (await api.get(`/api/assets/${model.id}/bundle.zip`)).body())
      expect(names).toEqual(
        expect.arrayContaining([
          expect.stringMatching(new RegExp(`(^|/)${MEGASCANS_FOLDER}\\.obj$`)),
          ...['BaseColor', 'Normal', 'Roughness', 'AO'].map((m) => expect.stringMatching(new RegExp(`(^|/)${stem}_${m}\\.png$`))),
        ]),
      )
      expect(names.filter((n) => n.includes('_2K_'))).toEqual([])
      // The store's JSON goes with the whole pack, not with the model.
      expect(names.filter((n) => n.endsWith('.json'))).toEqual([])
      // One map on its own, and in the manifest the hoard tool reads.
      const one = await api.get(`/api/assets/${ao.id}/file?download=1`)
      expect(one.headers()['content-disposition']).toContain(`"${stem}_AO.png"`)
      const manifest = (await (await api.post('/api/manifest', { data: { ids: [model.id], include_deps: true } })).json()) as {
        entries: { id: string; bundle_path?: string; file_name?: string }[]
      }
      const entry = manifest.entries.find((e) => e.id === ao.id)!
      expect(entry.file_name).toBe(`${stem}_AO.png`)
      expect(entry.bundle_path).toMatch(new RegExp(`(^|/)${stem}_AO\\.png$`))

      // The model's page lists the maps under the material by what they're for, and not again under Needs.
      await openPack(page, seed.packs[MEGASCANS], MEGASCANS, '&sort=name')
      await page.locator('.grid button.hit[aria-label*=" tris"]').click()
      const panel = page.getByRole('complementary', { name: 'Asset details' })
      const worn = panel.getByRole('list', { name: `${MEGASCANS}: maps` })
      for (const role of ['Color', 'Normal', 'Roughness', 'Ambient occlusion']) {
        await expect(worn.getByRole('button', { name: new RegExp(`^${role} `) })).toBeVisible()
      }
      const needs = panel.locator('section, div').filter({ has: page.getByRole('heading', { name: 'Needs', exact: true }) }).last()
      await expect(needs.getByRole('button', { name: /Xeokah 0/ })).toBeVisible()
      await expect(needs.getByRole('button', { name: new RegExp(MEGASCANS) })).toHaveCount(0)
    })

    test('Settings choose how maps are named in downloads', async ({ page, seed, baseURL }) => {
      const api = await signedIn(baseURL!)
      const ao = (await assetsOf(api, seed.packs[MEGASCANS])).find((a) => a.path.endsWith('_AO.png'))!
      const fileName = async () => (await api.get(`/api/assets/${ao.id}/file?download=1`)).headers()['content-disposition']
      await page.goto('/#settings/downloads')
      const section = page.getByTestId('texture-names')
      const example = page.getByTestId('texture-example')
      await expect(example).toContainText('StandWoodOld05_BaseColor.jpg')
      try {
        await section.getByText('Short', { exact: true }).click()
        await expect(example).toContainText('T_StandWoodOld05_BC.jpg')
        await expect(example).toContainText('T_RockMoss_BC_4K.png')
        await expect.poll(fileName).toContain('T_HistoricMiscellaneousStandWoodOld05_AO.png')

        // A pattern that can't work is refused before it's saved.
        const pattern = section.getByLabel('Pattern')
        await pattern.fill('{name}')
        await pattern.press('Enter')
        await expect(section.getByRole('alert')).toContainText('{map}')
        await pattern.fill('{map}-{name}')
        await pattern.press('Enter')
        await expect(example).toContainText('BC-StandWoodOld05.jpg')
        await expect.poll(fileName).toContain('AO-HistoricMiscellaneousStandWoodOld05.png')

        await section.getByRole('checkbox', { name: 'Name texture maps by their material' }).uncheck()
        await expect.poll(fileName).toContain(`${MEGASCANS_FOLDER}_2K_AO.png`)
      } finally {
        await api.put('/api/admin/settings', { data: { texture_names: null } })
      }
      await expect.poll(fileName).toContain('HistoricMiscellaneousStandWoodOld05_AO.png')
    })

    // A crate of its own, with a texture named so no other test's ending is this one.
    test("an ending of your own renames a texture that isn't a map, alone and in a GLB zip", async ({
      page,
      baseURL,
    }) => {
      test.setTimeout(90_000)
      const api = await signedIn(baseURL!)
      const c = crate()
      const pack = await uploadPack(api, 'E2E Endings', [
        { relpath: 'crate.obj', data: c.obj },
        { relpath: 'crate.mtl', data: Buffer.from(c.mtl.toString().replace('crate_diffuse', 'crate_paint')) },
        { relpath: 'crate_paint.png', data: c.texture },
      ])
      await waitReady(api, [pack.id])
      const assets = await assetsOf(api, pack.id)
      const id = (path: string) => assets.find((a) => a.path === path)!.id
      const texture = `/api/assets/${id('crate_paint.png')}/file?download=1`
      const fileName = async () => (await api.get(texture)).headers()['content-disposition']
      const glbZip = async () => {
        const res = await api.get(`/api/bundle.zip?ids=${id('crate.obj')}&glb=1`, { timeout: 60_000 })
        expect(res.ok(), `${res.status()}`).toBe(true)
        return zipNames(await res.body()).filter((n) => n.includes('/textures/'))
      }
      expect(await glbZip()).toEqual(['hoard-glb/textures/crate_paint.png'])

      await page.goto('/#settings/downloads')
      const section = page.getByTestId('texture-names')
      const endings = page.getByTestId('texture-endings')
      try {
        await endings.getByRole('button', { name: 'Add an ending' }).click()
        await endings.getByLabel('Ending', { exact: true }).fill('_paint')
        await endings.getByLabel('Swap for').fill('_P')
        await endings.getByLabel('Swap for').press('Enter')
        await expect.poll(fileName).toContain('crate_P.png')
        // The GLB made before is made again with the new name.
        expect(await glbZip()).toEqual(['hoard-glb/textures/crate_P.png'])

        // The same ending twice is refused before it's saved.
        await endings.getByRole('button', { name: 'Add an ending' }).click()
        await endings.getByLabel('Ending', { exact: true }).last().fill('_PAINT')
        await endings.getByLabel('Ending', { exact: true }).last().press('Enter')
        await expect(section.getByRole('alert')).toContainText('_PAINT is in your endings twice')
        await endings.getByRole('button', { name: 'Remove _PAINT', exact: true }).click()
        await endings.getByRole('button', { name: 'Remove _paint', exact: true }).click()
        await expect.poll(fileName).toContain('crate_paint.png')
      } finally {
        await api.put('/api/admin/settings', { data: { texture_names: null } })
        await api.delete(`/api/packs/${pack.id}`)
      }
    })
  })

  test("the server's thumbnail of the model has the material's color map on", async ({ seed, baseURL }) => {
    const api = await signedIn(baseURL!)
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    test.skip(!/^(gpu|cpu):/.test(stats.renderer ?? ''), `no model renderer here (${stats.renderer})`)
    const model = (await assetsOf(api, seed.packs[MEGASCANS])).find((a) => a.kind === 'model')!
    const full = (await (await api.get(`/api/assets/${model.id}`)).json()) as { meta: { thumb_dressed?: string } }
    expect(full.meta.thumb_dressed).toBeTruthy()
  })

  // Uploaded, it's packed into one GLB (its .bin inside, its maps still beside it), and dressed like any model.
  test("a glTF model's server thumbnail has the color map of the material beside it on", async ({ baseURL }) => {
    const api = await signedIn(baseURL!)
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    test.skip(!/^(gpu|cpu):/.test(stats.renderer ?? ''), `no model renderer here (${stats.renderer})`)
    const maps: [string, [number, number, number]][] = [
      ['BaseColor', [150, 40, 30]],
      ['Normal', [128, 128, 255]],
      ['Roughness', [200, 200, 200]],
    ]
    const sign = quadGltf('Signpost.bin')
    const pack = await uploadPack(api, 'E2E Dressed glTF', [
      { relpath: 'Signpost/Signpost.gltf', data: sign.gltf },
      { relpath: 'Signpost/Signpost.bin', data: sign.bin },
      ...maps.map(([m, c]) => ({ relpath: `Signpost/Signpost_${m}.png`, data: checker(64, c, c, 8) })),
    ])
    await waitReady(api, [pack.id])
    await waitIdle(api)
    const assets = (await assetsOf(api, pack.id)) as (Asset & { path: string })[]
    expect(assets.map((a) => a.path).filter((p) => /\.(gltf|bin|glb)$/.test(p))).toEqual(['Signpost.glb'])
    expect(assets.filter((a) => a.kind === 'texture')).toHaveLength(3)
    const model = assets.find((a) => a.kind === 'model')!
    const full = (await (await api.get(`/api/assets/${model.id}`)).json()) as { meta: { thumb_dressed?: string } }
    expect(full.meta.thumb_dressed).toBeTruthy()
    await api.delete(`/api/packs/${pack.id}`)
  })

  // A Unity material is read into one: its texture found through the GUID the texture's .meta gives it, its color,
  // smoothness and tiling kept as its look, and the .mat itself hidden behind it.
  test('a Unity .mat becomes a material with its texture and color, the .mat behind it', async ({ baseURL }) => {
    const api = await signedIn(baseURL!)
    const guid = createHash('md5').update('e2e-unity-planks').digest('hex')
    const mat =
      '%YAML 1.1\n%TAG !u! tag:unity3d.com,2011:\n--- !u!21 &2100000\nMaterial:\n  m_Name: Planks\n' +
      '  m_SavedProperties:\n' +
      `    m_TexEnvs:\n    - _MainTex:\n        m_Texture: {fileID: 2800000, guid: ${guid}, type: 3}\n` +
      '        m_Scale: {x: 2, y: 2}\n        m_Offset: {x: 0, y: 0}\n' +
      '    m_Floats:\n    - _Glossiness: 0.25\n    - _Metallic: 0\n' +
      '    m_Colors:\n    - _Color: {r: 1, g: 0.5, b: 0, a: 1}\n'
    const pack = await uploadPack(api, 'E2E Unity Material', [
      { relpath: 'Mats/Planks.mat', data: Buffer.from(mat) },
      { relpath: 'Textures/wood_grain.png', data: checker(64, [180, 140, 90], [120, 90, 60], 8) },
      {
        relpath: 'Textures/wood_grain.png.meta',
        data: Buffer.from(`fileFormatVersion: 2\nguid: ${guid}\nTextureImporter:\n  userData: \n`),
      },
    ])
    await waitReady(api, [pack.id])
    await waitIdle(api)
    const assets = (await assetsOf(api, pack.id)) as (Asset & { path: string })[]
    const material = assets.find((a) => a.path === 'Mats/Planks.material')
    expect(material, assets.map((a) => a.path).join(', ')).toBeTruthy()
    const { material: info } = (await (await api.get(`/api/assets/${material!.id}`)).json()) as {
      material: {
        name: string
        maps: { role: string; path: string }[]
        look?: { base_color?: number[]; roughness?: number; uv_scale?: number[] }
      }
    }
    expect(info.name).toBe('Planks')
    expect(info.maps.map((m) => [m.role, m.path])).toEqual([['base_color', 'Textures/wood_grain.png']])
    expect(info.look?.base_color?.[0]).toBeCloseTo(1)
    expect(info.look?.base_color?.[2]).toBeCloseTo(0)
    expect(info.look?.roughness).toBeCloseTo(0.75)
    expect(info.look?.uv_scale).toEqual([2, 2])
    // With supporting files left out, the .mat is behind the material.
    const shown = await api.get('/api/assets', { params: { pack: pack.id, hide_supporting: true, facets: false } })
    const paths = ((await shown.json()) as { items: (Asset & { path: string })[] }).items.map((a) => a.path)
    expect(paths).toContain('Mats/Planks.material')
    expect(paths).not.toContain('Mats/Planks.mat')
    await api.delete(`/api/packs/${pack.id}`)
  })

  // An OBJ's .mtl colors with no maps (as SketchUp exports them) become swatches the OBJ wears: one worn by two models
  // stays in the grid, one only the road wears is part of it, and the .mtl is drawn as its colors.
  test("a .mtl's colors become swatch materials its OBJs wear", async ({ baseURL }) => {
    const api = await signedIn(baseURL!)
    const quad = (mtl: string, use: string) =>
      Buffer.from(`mtllib ${mtl}\nv 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nusemtl ${use}\nf 1 2 3 4\n`)
    const pack = await uploadPack(api, 'E2E MTL Swatches', [
      { relpath: 'road.obj', data: quad('road.mtl', 'Stone') },
      {
        relpath: 'road.mtl',
        data: Buffer.from(
          '# Exported from SketchUp\nnewmtl Asphalt\nKa 0 0 0\nKd 0.411765 0.411765 0.411765\n\n' +
            'newmtl Stone\nKd 0.847059 0.827451 0.749020\n',
        ),
      },
      { relpath: 'car.obj', data: quad('car.mtl', 'Paint') },
      {
        relpath: 'car.mtl',
        data: Buffer.from('newmtl Asphalt\nKd 0.411765 0.411765 0.411765\nnewmtl Paint\nKd 1 0 0\n'),
      },
    ])
    await waitReady(api, [pack.id])
    await waitIdle(api)
    const assets = (await assetsOf(api, pack.id)) as (Asset & { path: string; has_thumb: boolean })[]
    const byPath = (p: string) => assets.find((a) => a.path === p)
    const swatches = assets.filter((a) => a.path.endsWith('.material')).map((a) => a.path)
    expect(swatches.sort()).toEqual(['Asphalt.material', 'Paint.material', 'Stone.material'])
    const { material: info } = (await (await api.get(`/api/assets/${byPath('Asphalt.material')!.id}`)).json()) as {
      material: { maps: unknown[]; look?: { base_color?: number[] } }
    }
    expect(info.maps).toEqual([])
    expect(info.look?.base_color?.[0]).toBeCloseTo(0.141, 2)
    const road = (await (await api.get(`/api/assets/${byPath('road.obj')!.id}`)).json()) as {
      links: { rel: string; path: string }[]
    }
    expect(road.links.filter((l) => l.rel === 'material').map((l) => l.path).sort()).toEqual([
      'Asphalt.material',
      'Stone.material',
      'road.mtl',
    ])
    expect(byPath('road.mtl')!.has_thumb).toBe(true)
    const shown = await api.get('/api/assets', { params: { pack: pack.id, hide_supporting: true, facets: false } })
    const paths = ((await shown.json()) as { items: (Asset & { path: string })[] }).items.map((a) => a.path).sort()
    expect(paths).toEqual(['Asphalt.material', 'car.obj', 'road.obj'])
    await api.delete(`/api/packs/${pack.id}`)
  })

  // Unity puts a .mat in each of an FBX's material slots: the one named after the slot, or the one the FBX's .meta
  // remaps the slot to. The model wears each in its slot.
  test("an FBX's material slots wear the Unity materials named for them", async ({ baseURL }) => {
    const api = await signedIn(baseURL!)
    const guid = (s: string) => createHash('md5').update(`e2e-unity-slots-${s}`).digest('hex')
    const meta = (g: string, rest = '') => Buffer.from(`fileFormatVersion: 2\nguid: ${g}\n${rest}`)
    const mat = (name: string, extra: string) =>
      Buffer.from(
        `%YAML 1.1\n%TAG !u! tag:unity3d.com,2011:\n--- !u!21 &2100000\nMaterial:\n  m_Name: ${name}\n` +
          `  m_SavedProperties:\n${extra}`,
      )
    // A cube with two materials: Paint on two faces, lambert2 on the rest.
    const fbx = [
      '; FBX 7.5.0 project file',
      'FBXHeaderExtension:  {',
      '\tFBXVersion: 7400',
      '}',
      'Objects:  {',
      '\tGeometry: 1, "Geometry::Cube", "Mesh" {',
      '\t\tVertices: *24 {',
      '\t\t\ta: 0,0,0,100,0,0,0,100,0,100,100,0,0,0,100,100,0,100,0,100,100,100,100,100',
      '\t\t}',
      '\t\tPolygonVertexIndex: *24 {',
      '\t\t\ta: 0,2,3,-2,4,5,7,-7,0,1,5,-5,2,6,7,-4,0,4,6,-3,1,3,7,-6',
      '\t\t}',
      '\t\tLayerElementMaterial: 0 {',
      '\t\t\tMappingInformationType: "ByPolygon"',
      '\t\t\tReferenceInformationType: "IndexToDirect"',
      '\t\t\tMaterials: *6 {',
      '\t\t\t\ta: 0,0,1,1,1,1',
      '\t\t\t}',
      '\t\t}',
      '\t\tLayer: 0 {',
      '\t\t\tLayerElement:  {',
      '\t\t\t\tType: "LayerElementMaterial"',
      '\t\t\t\tTypedIndex: 0',
      '\t\t\t}',
      '\t\t}',
      '\t}',
      '\tModel: 10, "Model::Cube", "Mesh" {',
      '\t}',
      '\tMaterial: 20, "Material::Paint", "" {',
      '\t\tShadingModel: "lambert"',
      '\t}',
      '\tMaterial: 21, "Material::lambert2", "" {',
      '\t\tShadingModel: "lambert"',
      '\t}',
      '}',
      'Connections:  {',
      '\tC: "OO", 10, 0',
      '\tC: "OO", 1, 10',
      '\tC: "OO", 20, 10',
      '\tC: "OO", 21, 10',
      '}',
      '',
    ].join('\n')
    const remap =
      'ModelImporter:\n  externalObjects:\n  - first:\n      type: UnityEngine:Material\n' +
      '      assembly: UnityEngine.CoreModule\n      name: lambert2\n' +
      `    second: {fileID: 2100000, guid: ${guid('brick')}, type: 2}\n  materials:\n    materialImportMode: 1\n`
    const pack = await uploadPack(api, 'E2E Unity Slots', [
      { relpath: 'Models/Crate.fbx', data: Buffer.from(fbx) },
      { relpath: 'Models/Crate.fbx.meta', data: meta(guid('crate'), remap) },
      { relpath: 'Materials/Paint.mat', data: mat('Paint', '    m_Colors:\n    - _Color: {r: 1, g: 0, b: 0, a: 1}\n') },
      { relpath: 'Materials/Paint.mat.meta', data: meta(guid('paint')) },
      {
        relpath: 'Materials/Wall_Brick.mat',
        data: mat(
          'Wall_Brick',
          `    m_TexEnvs:\n    - _MainTex:\n        m_Texture: {fileID: 2800000, guid: ${guid('tex')}, type: 3}\n`,
        ),
      },
      { relpath: 'Materials/Wall_Brick.mat.meta', data: meta(guid('brick')) },
      { relpath: 'Textures/brick.png', data: checker(64, [170, 60, 40], [120, 40, 30], 8) },
      { relpath: 'Textures/brick.png.meta', data: meta(guid('tex'), 'TextureImporter:\n') },
    ])
    await waitReady(api, [pack.id])
    await waitIdle(api)
    const assets = (await assetsOf(api, pack.id)) as (Asset & { path: string })[]
    const crate = assets.find((a) => a.path === 'Models/Crate.fbx')!
    const full = (await (await api.get(`/api/assets/${crate.id}`)).json()) as {
      materials: { path: string; slots?: string[] }[]
    }
    expect(full.materials.map((m) => [m.path, m.slots]).sort()).toEqual([
      ['Materials/Paint.material', ['Paint']],
      ['Materials/Wall_Brick.material', ['lambert2']],
    ])
    await api.delete(`/api/packs/${pack.id}`)
  })
})
