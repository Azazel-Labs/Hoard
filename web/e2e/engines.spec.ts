// The engine plugins against a real server: each adds a model with what it needs to a scratch project, then the real
// `hoard sync --verify` must find every file already in place. Godot runs when GODOT_BIN names a Godot 4 executable (its
// console build on Windows); Unity's package code runs on .NET when `dotnet` is installed, as it is on CI's runners.
// Both need the hoard CLI built (`cargo build -p hoard`, or HOARD_CLI).
import { spawnSync, type SpawnSyncReturns } from 'node:child_process'
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { basename, dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import type { APIRequestContext } from '@playwright/test'
import { assetsOf, uploadPack, waitReady } from './support/api'
import { crate } from './support/files'
import { expect, test, unique } from './support/test'

const repo = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..')

interface Outcome {
  ok: boolean
  error?: string | null
  files: number
  fetched: number
  reported: boolean
  matched: number
  thumbnails: number
}

/** HOARD_CLI, the hoard CLI beside HOARD_BIN, or the newest one built in the usual places. */
function hoardCli(): string | undefined {
  if (process.env.HOARD_CLI) return process.env.HOARD_CLI
  const exe = process.platform === 'win32' ? 'hoard.exe' : 'hoard'
  const dirs = [...(process.env.HOARD_BIN ? [dirname(resolve(process.env.HOARD_BIN))] : []), ...['target/debug', 'target/agent/debug', 'target/release'].map((d) => join(repo, d))]
  return dirs
    .map((d) => join(d, exe))
    .filter((p) => existsSync(p))
    .sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs)[0]
}

function outcome(out: SpawnSyncReturns<string>, engine: string): Outcome {
  const line = `${out.stdout}`.split(/\r?\n/).find((l) => l.startsWith('RESULT '))
  expect(line, `no result from ${engine}:\n${out.stdout}\n${out.stderr}`).toBeTruthy()
  return JSON.parse(line!.slice(7)) as Outcome
}

interface Plugin {
  engine: string
  /** Where a project keeps what Hoard adds, as hoard.toml says or the plugin's default. */
  outDir: string
  /** Make the scratch project; `name` is what hoard.toml should call it (when the plugin doesn't make hoard.toml). */
  prepare(dir: string, name: string): void
  /** The project's name on the server, once prepared. */
  project(dir: string, name: string): string
  run(dir: string, env: Record<string, string>): Outcome
}

async function roundTrip(api: APIRequestContext, baseURL: string, plugin: Plugin) {
  const cli = hoardCli()
  expect(cli, 'no hoard CLI: run `cargo build -p hoard` or set HOARD_CLI').toBeTruthy()
  // Letters only: names are split where letters meet digits, and the search should find it as one word.
  const tag = unique().replace(/\d/g, (d) => 'abcdefghij'[Number(d)])
  const c = crate()
  const pack = await uploadPack(api, `E2E ${plugin.engine} ${tag}`, [
    { relpath: `crate_${tag}.obj`, data: Buffer.from(c.obj.toString().replace('crate.mtl', `crate_${tag}.mtl`)) },
    { relpath: `crate_${tag}.mtl`, data: c.mtl },
    { relpath: 'crate_diffuse.png', data: c.texture },
  ])
  const made = (await (await api.post('/api/admin/tokens', { data: { name: `${plugin.engine} ${tag}`, role: 'view' } })).json()) as {
    id: number
    token: string
  }
  const dir = mkdtempSync(join(tmpdir(), `hoard-${plugin.engine.toLowerCase()}-`))
  const name = plugin.project(dir, `${plugin.engine} Test ${tag}`)
  try {
    await waitReady(api, [pack.id])
    const obj = (await assetsOf(api, pack.id)).find((a) => a.path.endsWith('.obj'))!
    plugin.prepare(dir, name)
    const env = { HOARD_URL: baseURL, HOARD_TOKEN: made.token, HOARD_QUERY: `crate ${tag}`, HOARD_KIND: 'model', HOARD_PROJECT: dir }

    const first = plugin.run(dir, env)
    expect(first.error ?? undefined).toBeUndefined()
    expect(first).toMatchObject({ ok: true, matched: 1, thumbnails: 1, files: 3, fetched: 3, reported: true })
    // Where hoard sync puts a model with what it needs: its category, then a folder named after it.
    const category = ((await (await api.get(`/api/assets/${obj.id}`)).json()) as { category: string }).category
    const folder = join(dir, plugin.outDir, ...category.split('/'), `crate_${tag}`)
    for (const f of [`crate_${tag}.obj`, `crate_${tag}.mtl`, 'crate_diffuse.png']) expect(existsSync(join(folder, f)), f).toBe(true)
    const toml = readFileSync(join(dir, 'hoard.toml'), 'utf8')
    expect(toml).toContain(`[[select]]\nids = ["${obj.id}"]`)
    const lockText = readFileSync(join(dir, 'hoard.lock'), 'utf8')
    const lock = JSON.parse(lockText) as { version: number; files: Record<string, { id: string }> }
    expect(lock.version).toBe(1)
    expect(Object.keys(lock.files)).toHaveLength(3)
    const used = async () => ((await (await api.get('/api/projects')).json()) as { projects: { name: string; assets: number }[] }).projects.find((p) => p.name === name)?.assets
    expect(await used()).toBe(3)

    // hoard sync agrees: everything is where it would have put it, with the right hashes, and nothing is taken out.
    const sync = spawnSync(cli!, ['sync', '--verify'], { cwd: dir, env: { ...process.env, HOARD_SERVER: baseURL, HOARD_TOKEN: made.token }, encoding: 'utf8', timeout: 60_000 })
    expect(sync.status, `${sync.stdout}\n${sync.stderr}`).toBe(0)
    expect(sync.stdout).toMatch(/\b0 to download \([^)]*\), 3 unchanged, 0 to remove/)
    // It writes the lock the plugin wrote, to the byte.
    expect(readFileSync(join(dir, 'hoard.lock'), 'utf8')).toBe(lockText)

    // Again: nothing to download, and pinned once.
    const second = plugin.run(dir, env)
    expect(second).toMatchObject({ ok: true, files: 3, fetched: 0 })
    expect(readFileSync(join(dir, 'hoard.toml'), 'utf8').split(obj.id).length).toBe(2)
    expect(await used()).toBe(3)
  } finally {
    rmSync(dir, { recursive: true, force: true })
    await api.delete(`/api/projects/${encodeURIComponent(name)}`)
    await api.delete(`/api/admin/tokens/${made.id}`)
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
}

test('the Godot plugin adds an asset with what it needs, where hoard sync would, and says the project uses it', async ({ api, baseURL }) => {
  const godot = process.env.GODOT_BIN
  test.skip(!godot, 'GODOT_BIN is not set')
  test.setTimeout(150_000)
  await roundTrip(api, baseURL!, {
    engine: 'Godot',
    outDir: 'assets/hoard',
    project: (_dir, name) => name,
    prepare(dir, name) {
      writeFileSync(join(dir, 'project.godot'), `config_version=5\n\n[application]\nconfig/name="${name}"\n`)
      writeFileSync(join(dir, 'hoard.toml'), `project = "${name}"\nout_dir = "assets/hoard"\n`)
      cpSync(join(repo, 'integrations/godot/addons'), join(dir, 'addons'), { recursive: true })
      cpSync(join(repo, 'integrations/godot/test'), join(dir, 'test'), { recursive: true })
    },
    run: (dir, env) =>
      outcome(spawnSync(godot!, ['--headless', '--path', dir, '-s', 'res://test/add_assets.gd'], { env: { ...process.env, ...env }, encoding: 'utf8', timeout: 90_000 }), 'Godot'),
  })
})

const dotnet = { ...process.env, DOTNET_CLI_TELEMETRY_OPTOUT: '1', DOTNET_NOLOGO: '1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE: '1' }

test('the Unity package adds an asset with what it needs, making hoard.toml, where hoard sync would', async ({ api, baseURL }) => {
  test.skip(spawnSync('dotnet', ['--version'], { env: dotnet }).status !== 0, 'dotnet is not installed')
  test.setTimeout(240_000)
  const project = join(repo, 'integrations/unity/test/AddAssets/AddAssets.csproj')
  const build = spawnSync('dotnet', ['build', project, '-nologo', '-v', 'q'], { env: dotnet, encoding: 'utf8', timeout: 180_000 })
  expect(build.status, `${build.stdout}\n${build.stderr}`).toBe(0)
  await roundTrip(api, baseURL!, {
    engine: 'Unity',
    outDir: 'Assets/Hoard',
    // With no hoard.toml the project goes by its folder's name, and the package makes one.
    project: (dir) => basename(dir),
    prepare(dir) {
      mkdirSync(join(dir, 'Assets'))
    },
    run: (_dir, env) =>
      outcome(spawnSync('dotnet', ['run', '--no-build', '--project', project], { env: { ...dotnet, ...env }, encoding: 'utf8', timeout: 90_000 }), 'Unity'),
  })
})
