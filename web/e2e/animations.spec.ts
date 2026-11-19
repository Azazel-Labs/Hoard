import { assetsOf, uploadPack, waitReady } from './support/api'
import { expect, openPack, test, tile, unique } from './support/test'

const BONES = ['Hips', 'Spine', 'Spine1', 'Spine2', 'Neck', 'Head', 'LeftArm', 'LeftForeArm', 'LeftHand', 'RightArm', 'RightForeArm', 'RightHand']

/** A glTF skeleton with these bone names in a chain: with a triangle skinned to it (a character), or with a clip
 *  that turns every bone for `secs` (an animation of its own, no mesh), carrying the hips `travel` forward, once
 *  for each of `takes` when the file has more than one. */
function rigged(prefix: string, opts: { mesh?: boolean; clip?: { name: string; secs: number; travel?: number }; takes?: string[] }): Buffer {
  const floats = [
    ...[0, 0, 0, 1, 0, 0, 0, 1, 0], // a triangle
    ...[0, opts.clip?.secs ?? 0], // two keyframe times
    ...[0, 0, 0, 1, 0, 0.38, 0, 0.92], // two turns
    ...[0, 0, 0, 0, 0, opts.clip?.travel ?? 0], // two places for the hips
  ]
  const joints = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
  const weights = [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]
  const bin = Buffer.concat([
    Buffer.from(new Float32Array(floats.slice(0, 19)).buffer),
    Buffer.from(new Uint8Array(joints).buffer),
    Buffer.from(new Float32Array(weights).buffer),
    Buffer.from(new Float32Array(floats.slice(19)).buffer),
  ])
  const bones = BONES.map((b, i) => ({ name: `${prefix}${b}`, ...(i + 1 < BONES.length ? { children: [i + 1] } : {}) }))
  const nodes: object[] = [...bones]
  if (opts.mesh) nodes.push({ name: 'Body', mesh: 0, skin: 0 })
  const doc = {
    asset: { version: '2.0' },
    scene: 0,
    scenes: [{ nodes: opts.mesh ? [0, BONES.length] : [0] }],
    nodes,
    buffers: [{ byteLength: bin.length, uri: `data:application/octet-stream;base64,${bin.toString('base64')}` }],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: 36 },
      { buffer: 0, byteOffset: 36, byteLength: 8 },
      { buffer: 0, byteOffset: 44, byteLength: 32 },
      { buffer: 0, byteOffset: 76, byteLength: 12 },
      { buffer: 0, byteOffset: 88, byteLength: 48 },
      { buffer: 0, byteOffset: 136, byteLength: 24 },
    ],
    accessors: [
      { bufferView: 0, componentType: 5126, count: 3, type: 'VEC3', min: [0, 0, 0], max: [1, 1, 0] },
      { bufferView: 1, componentType: 5126, count: 2, type: 'SCALAR', min: [0], max: [opts.clip?.secs ?? 0] },
      { bufferView: 2, componentType: 5126, count: 2, type: 'VEC4' },
      { bufferView: 3, componentType: 5121, count: 3, type: 'VEC4' },
      { bufferView: 4, componentType: 5126, count: 3, type: 'VEC4' },
      { bufferView: 5, componentType: 5126, count: 2, type: 'VEC3' },
    ],
    ...(opts.mesh
      ? {
          meshes: [{ primitives: [{ attributes: { POSITION: 0, JOINTS_0: 3, WEIGHTS_0: 4 } }] }],
          skins: [{ joints: BONES.map((_, i) => i) }],
        }
      : {}),
    ...(opts.clip
      ? {
          animations: (opts.takes ?? [opts.clip.name]).map((name) => ({
            name,
            samplers: [
              { input: 1, output: 2 },
              { input: 1, output: 5 },
            ],
            channels: [
              ...BONES.map((_, i) => ({ sampler: 0, target: { node: i, path: 'rotation' } })),
              ...(opts.clip?.travel ? [{ sampler: 1, target: { node: 0, path: 'translation' } }] : []),
            ],
          })),
        }
      : {}),
  }
  return Buffer.from(JSON.stringify(doc))
}

// A clip in one pack plays on a character in another when its skeleton has the bones the clip moves, named the same
// once Mixamo's or Blender's prefixes are off.
test('an animation fits the characters with its skeleton in other packs, and each lists the other', async ({ page, api }) => {
  test.setTimeout(180_000)
  const [heroes, mocap] = [`E2E Heroes ${unique()}`, `E2E Mocap ${unique()}`]
  const a = await uploadPack(api, heroes, [{ relpath: 'hero.gltf', data: rigged('mixamorig:', { mesh: true }) }])
  const b = await uploadPack(api, mocap, [{ relpath: 'Clips/run.gltf', data: rigged('Armature|', { clip: { name: 'Run', secs: 0.8, travel: 3 } }) }])
  try {
    await waitReady(api, [a.id, b.id])
    const [hero] = await assetsOf(api, a.id)
    const [run] = await assetsOf(api, b.id)
    expect([hero.kind, run.kind]).toEqual(['model', 'animation'])
    type Detail = { fits?: { id: string; pack_name: string; secs?: number }[] }
    const fits = async (id: string) => ((await (await api.get(`/api/assets/${id}`)).json()) as Detail).fits ?? []
    expect(await fits(hero.id)).toEqual([expect.objectContaining({ id: run.id, pack_name: mocap, secs: 0.8 })])
    expect(await fits(run.id)).toEqual([expect.objectContaining({ id: hero.id, pack_name: heroes })])
    // With a renderer, the clip's thumbnail is drawn mid-clip on that character, as it has no mesh of its own.
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    if (/^(gpu|cpu):/.test(stats.renderer ?? ''))
      await expect
        .poll(async () => ((await (await api.get(`/api/assets/${run.id}`)).json()) as { derivatives?: string[] }).derivatives, { timeout: 60_000 })
        .toContain('still')

    // Its tile shows its length.
    await openPack(page, b.id, mocap)
    await expect(tile(page, run.name)).toContainText('0.80 s')

    await openPack(page, a.id, heroes)
    await tile(page, hero.name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    // The character's viewer offers the clip from the other pack, fetched only when picked.
    const clips = panel.getByRole('combobox', { name: 'Animation', exact: true })
    await expect(clips).toBeVisible({ timeout: 30_000 })
    await expect(clips.locator('option:checked')).toHaveText('Rest pose')
    const fetched = page.waitForRequest((r) => r.url().includes(`/api/assets/${run.id}/as/view`))
    await clips.selectOption({ label: `${run.name} · ${mocap}` })
    await fetched
    // Its bones are named for Blender, the character's for Mixamo, and it runs forward: the character is held in the
    // frame over a scrolling grid; at rest it isn't.
    await expect(panel.locator('[data-travels]')).toHaveCount(1)
    await clips.selectOption({ label: 'Rest pose' })
    await expect(panel.locator('[data-travels]')).toHaveCount(0)
    const section = panel.getByTestId('fits')
    await expect(section.getByRole('heading', { name: 'Animations that fit (1)' })).toBeVisible()
    await expect(section.getByRole('button', { name: new RegExp(`${run.name}.*${mocap}`) })).toContainText('0.80 s')
    await section.getByRole('button', { name: new RegExp(run.name) }).click()
    await expect(panel.getByRole('heading', { name: run.name })).toBeVisible()
    await expect(section.getByRole('heading', { name: 'Characters it fits (1)' })).toBeVisible()
    await expect(section.getByRole('button', { name: new RegExp(`${hero.name}.*${heroes}`) })).toBeVisible()
    // Its zip can take that character along, in a folder named for its pack, and it downloads as a clip on its own.
    await panel.getByRole('checkbox', { name: `Include the character ${hero.name} · ${heroes}` }).check()
    await expect(panel.locator('a[href*="bundle.zip"]')).toHaveAttribute('href', new RegExp(`character=${hero.id}`))
    await expect(panel.locator('a[href*="bundle.zip"]')).toContainText('1 linked file')
    const zip = await api.get(`/api/assets/${run.id}/bundle.zip?character=${hero.id}`)
    expect((await zip.body()).toString('latin1')).toContain(`${heroes}/hero.glb`)
    await expect(panel.locator('#as-format option', { hasText: 'Clip only, no mesh (GLB)' })).toHaveCount(1)
    const clip = await api.get(`/api/assets/${run.id}/as/clip?wait=60`)
    expect(clip.status()).toBe(200)
    expect((await clip.body()).subarray(0, 4).toString()).toBe('glTF')
    // Its own pack has no character for it, so it plays on the one from the other pack, which says so.
    const playsOn = panel.getByRole('combobox', { name: 'Plays on' })
    await expect(playsOn).toBeVisible({ timeout: 30_000 })
    await expect(playsOn.locator('option:checked')).toHaveText(`${hero.name} · ${heroes}`)
  } finally {
    await api.post('/api/archive', { data: { packs: [a.id, b.id] } })
    await api.post('/api/archive/purge', { data: { packs: [a.id, b.id] } })
  }
})

// A file of many takes has each as an animation of its own, named for it, found by it and fitting what the file
// fits; the file of clips itself is shown by them.
test('each take in a file of many is an animation of its own', async ({ api }) => {
  test.setTimeout(120_000)
  const name = `E2E Takes ${unique()}`
  // Bones of their own, so these fit only each other and not the other tests' characters running alongside.
  const bones = `Takes${unique()}_`
  const pack = await uploadPack(api, name, [
    { relpath: 'hero.gltf', data: rigged(bones, { mesh: true }) },
    { relpath: 'Clips/moves.gltf', data: rigged(bones, { clip: { name: 'Moves', secs: 0.8 }, takes: ['Walk', 'Sprint'] }) },
  ])
  try {
    await waitReady(api, [pack.id])
    const all = await assetsOf(api, pack.id)
    const byPath = (p: string) => all.find((a) => a.path === p)
    const [walk, sprint, file] = [byPath('Clips/moves@Walk.glb'), byPath('Clips/moves@Sprint.glb'), all.find((a) => a.path.startsWith('Clips/moves.'))]
    expect([walk?.kind, walk?.name, sprint?.kind, sprint?.name]).toEqual(['animation', 'Walk', 'animation', 'Sprint'])
    expect(file).toBeTruthy()
    const hero = all.find((a) => a.path.startsWith('hero.'))!
    // The grid shows the takes, not the file they're in.
    const shown = await api.get('/api/assets', { params: { pack: pack.id, limit: 500, facets: false, hide_supporting: true } })
    const paths = ((await shown.json()) as { items: { path: string }[] }).items.map((a) => a.path)
    expect(paths).toEqual(expect.arrayContaining(['Clips/moves@Walk.glb', 'Clips/moves@Sprint.glb', hero.path]))
    expect(paths).not.toContain(file!.path)
    // Searching for one finds it.
    const found = await api.get('/api/assets', { params: { pack: pack.id, q: 'sprint', limit: 10, facets: false, hide_supporting: true } })
    expect(((await found.json()) as { items: { id: string }[] }).items.map((a) => a.id)).toEqual([sprint!.id])
    // Each holds its one clip, and plays on the pack's character; the character has them as its clips already.
    type Detail = { fits?: { id: string }[] }
    const fits = async (id: string) => ((await (await api.get(`/api/assets/${id}`)).json()) as Detail).fits ?? []
    expect((await fits(sprint!.id)).map((f) => f.id)).toEqual([hero.id])
    expect((await fits(hero.id)).map((f) => f.id)).not.toContain(file!.id)
    const clip = await api.get(`/api/assets/${sprint!.id}/as/clip?wait=60`)
    expect(clip.status()).toBe(200)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
