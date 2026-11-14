// The engine plugins against a real server: each adds a model with what it needs to a scratch project, then the real
// `hoard sync --verify` must find every file already in place. Godot runs when GODOT_BIN names a Godot 4 executable (its
// console build on Windows); Unity's package code runs on .NET when `dotnet` is installed, as it is on CI's runners;
// Unreal runs when UNREAL_EDITOR names UnrealEditor-Cmd, building the plugin with that engine's UAT the first time.
// All need the hoard CLI built (`cargo build -p hoard`, or HOARD_CLI).
import { createHash } from 'node:crypto'
import { spawnSync, type SpawnSyncReturns } from 'node:child_process'
import { cpSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { basename, dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import type { APIRequestContext } from '@playwright/test'
import { assetsOf, uploadPack, waitReady } from './support/api'
import { checker, crate, sound } from './support/files'
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
  /** Unreal: the assets it imported. */
  imported?: string[]
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
  /** Anything more to check after the first add, with the files in place. */
  after?(dir: string, first: Outcome, folder: string): void
  /** Sending the project's own assets to Hoard, after it has some from Hoard. */
  exporter?: Exporter
}

interface Planted {
  /** What's picked to send, as the engine names it. */
  pick: string[]
  /** The paths the new pack should have, and nothing else. */
  expect: string[]
  /** How many picked or used things are left out because Hoard gave them, or only this engine opens them. */
  fromHoard: number
  engineOnly: number
}

interface Sent {
  ok: boolean
  error?: string | null
  pack?: string
  link?: string
  sent?: string[]
  planned: string[]
  from_hoard: number
  engine_only: number
}

interface Exporter {
  plant(dir: string, tag: string): Planted
  run(dir: string, env: Record<string, string>): Sent
}

/** A project's own model, not from Hoard: an OBJ whose material's picture is in a folder below it. */
function ownCrate(tag: string): { obj: Buffer; mtl: Buffer; texture: Buffer; png: string } {
  const c = crate()
  const png = `tex/wood_${tag}.png`
  return {
    obj: Buffer.from(c.obj.toString().replace('mtllib crate.mtl', `mtllib mine_${tag}.mtl`)),
    mtl: Buffer.from(c.mtl.toString().replace('map_Kd crate_diffuse.png', `map_Kd ${png}`)),
    texture: checker(32, [40, 120, 200], [20, 60, 100], 4),
    png,
  }
}

function withoutSharedTop(paths: string[]): string[] {
  let out = paths
  for (;;) {
    const top = out[0]?.split('/')[0]
    if (!top || !out.every((p) => p.startsWith(`${top}/`))) return out
    out = out.map((p) => p.slice(top.length + 1))
  }
}

function sentOutcome(out: SpawnSyncReturns<string>, engine: string): Sent {
  return outcome(out, engine) as unknown as Sent
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
  const cleanup: (() => Promise<unknown>)[] = []
  const gone = async (id: string) => {
    await api.post('/api/archive', { data: { packs: [id] } })
    await api.post('/api/archive/purge', { data: { packs: [id] } })
  }
  try {
    await waitReady(api, [pack.id])
    const obj = (await assetsOf(api, pack.id)).find((a) => a.path.endsWith('.obj'))!
    // A model's thumbnail is drawn in the background after its pack is ready, when there's a renderer to draw it
    // (CI runners have none); the plugin should find it drawn.
    const stats = (await (await api.get('/api/stats')).json()) as { renderer?: string }
    const thumbnails = /^(gpu|cpu):/.test(stats.renderer ?? '') ? 1 : 0
    if (thumbnails)
      await expect
        .poll(async () => ((await (await api.get(`/api/assets/${obj.id}`)).json()) as { has_thumb?: boolean }).has_thumb, { timeout: 120_000 })
        .toBe(true)
    plugin.prepare(dir, name)
    const env = { HOARD_URL: baseURL, HOARD_TOKEN: made.token, HOARD_QUERY: `crate ${tag}`, HOARD_KIND: 'model', HOARD_PROJECT: dir }

    const first = plugin.run(dir, env)
    expect(first.error ?? undefined).toBeUndefined()
    expect(first).toMatchObject({ ok: true, matched: 1, thumbnails, files: 3, fetched: 3, reported: true })
    // Where hoard sync puts a model with what it needs: its category, then a folder named after it.
    const category = ((await (await api.get(`/api/assets/${obj.id}`)).json()) as { category: string }).category
    const folder = join(dir, plugin.outDir, ...category.split('/'), `crate_${tag}`)
    for (const f of [`crate_${tag}.obj`, `crate_${tag}.mtl`, 'crate_diffuse.png']) expect(existsSync(join(folder, f)), f).toBe(true)
    plugin.after?.(dir, first, folder)
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

    if (plugin.exporter) {
      // The project's own assets go to Hoard as a new pack: originals, or copies other engines open; nothing from Hoard.
      const planted = plugin.exporter.plant(dir, tag)
      const pick = { ...env, HOARD_EXPORT: planted.pick.join('\n'), HOARD_PACK_NAME: `Sent from ${plugin.engine} ${tag}` }
      const refused = plugin.exporter.run(dir, pick)
      expect(refused.ok).toBe(false)
      expect(refused.error).toContain('can only look')
      const editor = (await (await api.post('/api/admin/tokens', { data: { name: `${plugin.engine} sends ${tag}`, role: 'edit' } })).json()) as {
        id: number
        token: string
      }
      cleanup.push(() => api.delete(`/api/admin/tokens/${editor.id}`))
      const sent = plugin.exporter.run(dir, { ...pick, HOARD_TOKEN: editor.token })
      expect(sent.error ?? undefined).toBeUndefined()
      if (sent.pack) cleanup.push(() => gone(sent.pack!))
      expect(sent).toMatchObject({ ok: true, from_hoard: planted.fromHoard, engine_only: planted.engineOnly })
      expect([...sent.sent!].sort()).toEqual([...planted.expect].sort())
      expect(sent.link).toContain(`pack=${sent.pack}`)
      await waitReady(api, [sent.pack!])
      // The server drops a top folder every file shares.
      expect((await assetsOf(api, sent.pack!)).map((a) => a.path).sort()).toEqual(withoutSharedTop(planted.expect).sort())
    }
  } finally {
    for (const undo of cleanup.reverse()) await undo()
    rmSync(dir, { recursive: true, force: true })
    await api.delete(`/api/projects/${encodeURIComponent(name)}`)
    await api.delete(`/api/admin/tokens/${made.id}`)
    await gone(pack.id)
  }
}

test('the Godot plugin adds an asset with what it needs, where hoard sync would, says the project uses it, and sends the project’s own', async ({
  api,
  baseURL,
}) => {
  const godot = process.env.GODOT_BIN
  test.skip(!godot, 'GODOT_BIN is not set')
  test.setTimeout(200_000)
  const godotRun = (script: string) => (dir: string, env: Record<string, string>) =>
    spawnSync(godot!, ['--headless', '--path', dir, '-s', `res://test/${script}`], { env: { ...process.env, ...env }, encoding: 'utf8', timeout: 90_000 })
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
    run: (dir, env) => outcome(godotRun('add_assets.gd')(dir, env), 'Godot'),
    exporter: {
      plant(dir, tag) {
        // A scene using the project's own OBJ and a script, a mesh made in Godot, and a sound.
        const own = ownCrate(tag)
        const mine = join(dir, 'mine')
        mkdirSync(join(mine, 'tex'), { recursive: true })
        writeFileSync(join(mine, `mine_${tag}.obj`), own.obj)
        writeFileSync(join(mine, `mine_${tag}.mtl`), own.mtl)
        writeFileSync(join(mine, own.png), own.texture)
        writeFileSync(join(mine, 'beep.wav'), sound('zap', 3))
        writeFileSync(join(mine, 'spin.gd'), 'extends Node3D\n')
        writeFileSync(join(mine, 'box.tres'), '[gd_resource type="BoxMesh" format=3]\n\n[resource]\nsize = Vector3(1, 2, 1)\n')
        writeFileSync(
          join(mine, 'level.tscn'),
          `[gd_scene load_steps=3 format=3]\n\n[ext_resource type="PackedScene" path="res://mine/mine_${tag}.obj" id="1"]\n` +
            `[ext_resource type="Script" path="res://mine/spin.gd" id="2"]\n\n[node name="Level" type="Node3D"]\nscript = ExtResource("2")\n\n` +
            `[node name="Crate" parent="." instance=ExtResource("1")]\n`,
        )
        return {
          pick: ['res://mine/level.tscn', 'res://mine/box.tres', 'res://mine/beep.wav', 'res://assets/hoard'],
          expect: [`mine/mine_${tag}.obj`, `mine/mine_${tag}.mtl`, `mine/${own.png}`, 'mine/box.glb', 'mine/beep.wav'],
          fromHoard: 1,
          engineOnly: 2,
        }
      },
      run: (dir, env) => sentOutcome(godotRun('export_assets.gd')(dir, env), 'Godot'),
    },
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

/** Every file under a folder, relative to it, in a stable order. */
function filesUnder(root: string, at = ''): string[] {
  return readdirSync(join(root, at), { withFileTypes: true })
    .sort((a, b) => a.name.localeCompare(b.name))
    .flatMap((e) => (e.isDirectory() ? filesUnder(root, join(at, e.name)) : [join(at, e.name)]))
}

/** The plugin built for this engine from the sources as they are now: UAT's BuildPlugin, once per change. */
function unrealPlugin(editor: string): string {
  const source = join(repo, 'integrations/unreal/Hoard')
  const hash = createHash('sha256').update(editor)
  for (const f of filesUnder(source)) hash.update(f).update(readFileSync(join(source, f)))
  const out = join(tmpdir(), `hoard-unreal-plugin-${hash.digest('hex').slice(0, 12)}`)
  if (existsSync(join(out, 'Hoard.uplugin'))) return out
  const uat = resolve(dirname(editor), '..', '..', 'Build', 'BatchFiles', process.platform === 'win32' ? 'RunUAT.bat' : 'RunUAT.sh')
  const args = ['BuildPlugin', `-Plugin=${join(source, 'Hoard.uplugin')}`, `-Package=${out}`, '-Rocket']
  // A .bat runs through cmd, which wants the whole line quoted as it will read it.
  const built =
    process.platform === 'win32'
      ? spawnSync('cmd.exe', ['/d', '/s', '/c', `"${[uat, ...args].map((a) => `"${a}"`).join(' ')}"`], {
          encoding: 'utf8',
          timeout: 1_200_000,
          windowsVerbatimArguments: true,
        })
      : spawnSync(uat, args, { encoding: 'utf8', timeout: 1_200_000 })
  expect(built.status, `${built.stdout?.slice(-4000)}
${built.stderr}`).toBe(0)
  return out
}

test('the Unreal plugin adds an asset with what it needs, where hoard sync would, and imports it', async ({ api, baseURL }) => {
  const editor = process.env.UNREAL_EDITOR
  test.skip(!editor, 'UNREAL_EDITOR is not set')
  test.setTimeout(1_800_000)
  const plugin = unrealPlugin(editor!)
  const uproject = 'HoardTest.uproject'
  await roundTrip(api, baseURL!, {
    engine: 'Unreal',
    outDir: 'Hoard',
    project: (dir) => basename(dir),
    prepare(dir) {
      writeFileSync(join(dir, uproject), JSON.stringify({ FileVersion: 3, Plugins: [{ Name: 'Hoard', Enabled: true }] }, null, 2))
      cpSync(plugin, join(dir, 'Plugins', 'Hoard'), { recursive: true })
    },
    run: (dir, env) =>
      outcome(
        spawnSync(editor!, [join(dir, uproject), '-run=HoardAdd', '-unattended', '-nullrhi', '-nosplash', '-nosound', '-nop4'], {
          env: { ...process.env, ...env },
          encoding: 'utf8',
          timeout: 900_000,
          maxBuffer: 256 * 1024 * 1024,
        }),
        'Unreal',
      ),
    after(dir, first, folder) {
      // The model came in as Unreal assets, under /Game/Hoard in the folders it has under Hoard/.
      expect(first.imported?.length, JSON.stringify(first.imported)).toBeGreaterThan(0)
      const content = join(dir, 'Content', 'Hoard', ...folder.slice(join(dir, 'Hoard').length + 1).split(/[\\/]/))
      expect(existsSync(content) && filesUnder(content).some((f) => f.endsWith('.uasset')), `nothing imported into ${content}`).toBe(true)
    },
  })
})
