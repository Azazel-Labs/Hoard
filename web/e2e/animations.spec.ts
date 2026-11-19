import { assetsOf, uploadPack, waitReady } from './support/api'
import { expect, openPack, test, tile, unique } from './support/test'

const BONES = ['Hips', 'Spine', 'Spine1', 'Spine2', 'Neck', 'Head', 'LeftArm', 'LeftForeArm', 'LeftHand', 'RightArm', 'RightForeArm', 'RightHand']

/** A glTF skeleton with these bone names in a chain: with a triangle skinned to it (a character), or with a clip
 *  that turns every bone for `secs` (an animation of its own, no mesh). */
function rigged(prefix: string, opts: { mesh?: boolean; clip?: { name: string; secs: number } }): Buffer {
  const floats = [
    ...[0, 0, 0, 1, 0, 0, 0, 1, 0], // a triangle
    ...[0, opts.clip?.secs ?? 0], // two keyframe times
    ...[0, 0, 0, 1, 0, 0.38, 0, 0.92], // two turns
  ]
  const joints = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
  const weights = [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]
  const bin = Buffer.concat([Buffer.from(new Float32Array(floats).buffer), Buffer.from(new Uint8Array(joints).buffer), Buffer.from(new Float32Array(weights).buffer)])
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
    ],
    accessors: [
      { bufferView: 0, componentType: 5126, count: 3, type: 'VEC3', min: [0, 0, 0], max: [1, 1, 0] },
      { bufferView: 1, componentType: 5126, count: 2, type: 'SCALAR', min: [0], max: [opts.clip?.secs ?? 0] },
      { bufferView: 2, componentType: 5126, count: 2, type: 'VEC4' },
      { bufferView: 3, componentType: 5121, count: 3, type: 'VEC4' },
      { bufferView: 4, componentType: 5126, count: 3, type: 'VEC4' },
    ],
    ...(opts.mesh
      ? {
          meshes: [{ primitives: [{ attributes: { POSITION: 0, JOINTS_0: 3, WEIGHTS_0: 4 } }] }],
          skins: [{ joints: BONES.map((_, i) => i) }],
        }
      : {}),
    ...(opts.clip
      ? {
          animations: [
            {
              name: opts.clip.name,
              samplers: [{ input: 1, output: 2 }],
              channels: BONES.map((_, i) => ({ sampler: 0, target: { node: i, path: 'rotation' } })),
            },
          ],
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
  const b = await uploadPack(api, mocap, [{ relpath: 'Clips/run.gltf', data: rigged('Armature|', { clip: { name: 'Run', secs: 0.8 } }) }])
  try {
    await waitReady(api, [a.id, b.id])
    const [hero] = await assetsOf(api, a.id)
    const [run] = await assetsOf(api, b.id)
    expect([hero.kind, run.kind]).toEqual(['model', 'animation'])
    type Detail = { fits?: { id: string; pack_name: string; secs?: number }[] }
    const fits = async (id: string) => ((await (await api.get(`/api/assets/${id}`)).json()) as Detail).fits ?? []
    expect(await fits(hero.id)).toEqual([expect.objectContaining({ id: run.id, pack_name: mocap, secs: 0.8 })])
    expect(await fits(run.id)).toEqual([expect.objectContaining({ id: hero.id, pack_name: heroes })])

    await openPack(page, a.id, heroes)
    await tile(page, hero.name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const section = panel.getByTestId('fits')
    await expect(section.getByRole('heading', { name: 'Animations that fit (1)' })).toBeVisible()
    await expect(section.getByRole('button', { name: new RegExp(`${run.name}.*${mocap}`) })).toContainText('0.80 s')
    await section.getByRole('button', { name: new RegExp(run.name) }).click()
    await expect(panel.getByRole('heading', { name: run.name })).toBeVisible()
    await expect(section.getByRole('heading', { name: 'Characters it fits (1)' })).toBeVisible()
    await expect(section.getByRole('button', { name: new RegExp(`${hero.name}.*${heroes}`) })).toBeVisible()
    // Its own pack has no character for it, so it plays on the one from the other pack, which says so.
    const playsOn = panel.getByRole('combobox', { name: 'Plays on' })
    await expect(playsOn).toBeVisible({ timeout: 30_000 })
    await expect(playsOn.locator('option:checked')).toHaveText(`${hero.name} · ${heroes}`)
  } finally {
    await api.post('/api/archive', { data: { packs: [a.id, b.id] } })
    await api.post('/api/archive/purge', { data: { packs: [a.id, b.id] } })
  }
})
