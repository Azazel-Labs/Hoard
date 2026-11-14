// The engine plugins against a real server, when the engine is installed: GODOT_BIN for Godot 4 (its console build on
// Windows). Skipped otherwise, as in CI.
import { spawnSync } from 'node:child_process'
import { cpSync, existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { assetsOf, uploadPack, waitReady } from './support/api'
import { crate } from './support/files'
import { expect, test, unique } from './support/test'

const repo = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..')

test('the Godot plugin adds an asset with what it needs, pins it in hoard.toml and says the project uses it', async ({ api, baseURL }) => {
  const godot = process.env.GODOT_BIN
  test.skip(!godot, 'GODOT_BIN is not set')
  test.setTimeout(120_000)
  // Letters only: names are split where letters meet digits, and the search should find it as one word.
  const tag = unique().replace(/\d/g, (d) => 'abcdefghij'[Number(d)])
  const c = crate()
  const pack = await uploadPack(api, `E2E Godot ${tag}`, [
    { relpath: `godot_${tag}/crate_${tag}.obj`, data: Buffer.from(c.obj.toString().replace('crate.mtl', `crate_${tag}.mtl`)) },
    { relpath: `godot_${tag}/crate_${tag}.mtl`, data: c.mtl },
    { relpath: `godot_${tag}/crate_diffuse.png`, data: c.texture },
  ])
  const made = (await (await api.post('/api/admin/tokens', { data: { name: `godot ${tag}`, role: 'view' } })).json()) as {
    id: number
    token: string
  }
  const dir = mkdtempSync(join(tmpdir(), 'hoard-godot-'))
  const name = `Godot Test ${tag}`
  try {
    await waitReady(api, [pack.id])
    const obj = (await assetsOf(api, pack.id)).find((a) => a.path.endsWith('.obj'))!
    writeFileSync(join(dir, 'project.godot'), `config_version=5\n\n[application]\nconfig/name="${name}"\n`)
    writeFileSync(join(dir, 'hoard.toml'), `project = "${name}"\nout_dir = "assets/hoard"\n`)
    cpSync(join(repo, 'integrations/godot/addons'), join(dir, 'addons'), { recursive: true })
    cpSync(join(repo, 'integrations/godot/test'), join(dir, 'test'), { recursive: true })
    const run = () => {
      const out = spawnSync(godot!, ['--headless', '--path', dir, '-s', 'res://test/add_assets.gd'], {
        env: { ...process.env, HOARD_URL: baseURL, HOARD_TOKEN: made.token, HOARD_QUERY: `crate ${tag}`, HOARD_KIND: 'model', HOARD_PROJECT: dir },
        encoding: 'utf8',
        timeout: 90_000,
      })
      const line = `${out.stdout}`.split('\n').find((l) => l.startsWith('RESULT '))
      expect(line, `no result from Godot:\n${out.stdout}\n${out.stderr}`).toBeTruthy()
      return JSON.parse(line!.slice(7)) as { ok: boolean; error?: string; files: number; fetched: number; reported: boolean; matched: number; thumbnails: number }
    }

    const first = run()
    expect(first.error).toBeUndefined()
    expect(first).toMatchObject({ ok: true, matched: 1, thumbnails: 1, files: 3, fetched: 3, reported: true })
    // Where hoard sync puts a model with what it needs: its category, then a folder named after it.
    const folder = join(dir, 'assets/hoard', ...(await (await api.get(`/api/assets/${obj.id}`)).json()).category.split('/'), `crate_${tag}`)
    for (const f of [`crate_${tag}.obj`, `crate_${tag}.mtl`, 'crate_diffuse.png']) expect(existsSync(join(folder, f)), f).toBe(true)
    const toml = readFileSync(join(dir, 'hoard.toml'), 'utf8')
    expect(toml).toContain(`[[select]]\nids = ["${obj.id}"]`)
    const lock = JSON.parse(readFileSync(join(dir, 'hoard.lock'), 'utf8')) as { version: number; files: Record<string, { id: string }> }
    expect(lock.version).toBe(1)
    expect(Object.keys(lock.files)).toHaveLength(3)
    const projects = ((await (await api.get('/api/projects')).json()) as { projects: { name: string; assets: number }[] }).projects
    expect(projects.find((p) => p.name === name)?.assets).toBe(3)

    // Again: nothing to download, and pinned once.
    const second = run()
    expect(second).toMatchObject({ ok: true, files: 3, fetched: 0 })
    expect(readFileSync(join(dir, 'hoard.toml'), 'utf8').split(obj.id).length).toBe(2)
  } finally {
    rmSync(dir, { recursive: true, force: true })
    await api.delete(`/api/projects/${encodeURIComponent(name)}`)
    await api.delete(`/api/admin/tokens/${made.id}`)
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
