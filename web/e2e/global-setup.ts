// Starts one hoardd for the whole run, on a free port with its own data and library folders, makes the account and a
// library location, seeds the packs, and saves a signed-in browser state for the tests.
import { spawn } from 'node:child_process'
import { existsSync, mkdirSync, mkdtempSync, openSync, statSync, writeFileSync } from 'node:fs'
import { createServer } from 'node:net'
import { tmpdir } from 'node:os'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { request, type FullConfig } from '@playwright/test'
import { assetsOf, HOARD_HEADERS, PASSWORD, signedIn, uploadPack, USERNAME, waitIdle, waitReady } from './support/api'
import { seedPacks, STORE, ZIPPED } from './support/seed'
import { AUTH_FILE, RUN_FILE, type RunInfo, type Seed } from './support/paths'

const here = dirname(fileURLToPath(import.meta.url))
const webDir = resolve(here, '..')
const repo = resolve(webDir, '..')

/** `HOARD_BIN`, or the newest hoardd built in the usual places. */
function findBinary(): string {
  const fromEnv = process.env.HOARD_BIN
  if (fromEnv) {
    if (!existsSync(fromEnv)) throw new Error(`HOARD_BIN points at ${fromEnv}, which doesn't exist`)
    return resolve(fromEnv)
  }
  const exe = process.platform === 'win32' ? 'hoardd.exe' : 'hoardd'
  const found = ['target/debug', 'target/agent/debug', 'target/release']
    .map((d) => join(repo, d, exe))
    .filter((p) => existsSync(p))
    .sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs)
  if (!found.length) throw new Error('No hoardd binary found: run `cargo build -p hoardd` or set HOARD_BIN')
  return found[0]
}

function freePort(): Promise<number> {
  return new Promise((ok, fail) => {
    const srv = createServer()
    srv.unref()
    srv.on('error', fail)
    srv.listen(0, '127.0.0.1', () => {
      const addr = srv.address()
      const port = typeof addr === 'object' && addr ? addr.port : 0
      srv.close(() => ok(port))
    })
  })
}

async function waitUp(url: string, exited: () => string | null, timeoutMs = 60_000): Promise<void> {
  const deadline = Date.now() + timeoutMs
  for (;;) {
    const why = exited()
    if (why) throw new Error(`hoardd exited before it was ready (${why}); see its log`)
    try {
      const res = await fetch(`${url}/api/auth/status`)
      if (res.ok) return
    } catch {
      // not listening yet
    }
    if (Date.now() > deadline) throw new Error(`hoardd didn't answer at ${url} in time`)
    await new Promise((r) => setTimeout(r, 100))
  }
}

export default async function globalSetup(_config: FullConfig): Promise<void> {
  const webDist = resolve(process.env.HOARD_WEB_DIR ?? join(webDir, 'dist'))
  if (!existsSync(join(webDist, 'index.html'))) throw new Error(`No web build at ${webDist}: run \`npm run build\` first`)
  const bin = findBinary()
  const root = mkdtempSync(join(tmpdir(), 'hoard-e2e-'))
  const dataDir = join(root, 'data')
  const libraryDir = join(root, 'library')
  mkdirSync(dataDir)
  mkdirSync(libraryDir)
  const port = await freePort()
  const url = `http://127.0.0.1:${port}`

  const logDir = join(webDir, 'test-results')
  mkdirSync(logDir, { recursive: true })
  const logFile = join(logDir, 'hoardd.log')
  const log = openSync(logFile, 'w')
  const child = spawn(bin, ['--listen', `127.0.0.1:${port}`, '--data-dir', dataDir, '--web-dir', webDist], {
    stdio: ['ignore', log, log],
    env: { ...process.env, RUST_LOG: process.env.RUST_LOG ?? 'info' },
  })
  let exit: string | null = null
  child.on('exit', (code, signal) => (exit = `code ${code}, signal ${signal}`))
  if (!child.pid) throw new Error(`couldn't start ${bin}`)

  const run: RunInfo = { pid: child.pid, root, url, logFile }
  mkdirSync(dirname(RUN_FILE), { recursive: true })
  writeFileSync(RUN_FILE, JSON.stringify(run))

  await waitUp(url, () => exit)

  const anon = await request.newContext({ baseURL: url, extraHTTPHeaders: HOARD_HEADERS })
  const setup = await anon.post('/api/auth/setup', { data: { username: USERNAME, password: PASSWORD } })
  if (!setup.ok()) throw new Error(`account setup failed: ${setup.status()} ${await setup.text()}`)
  await anon.dispose()

  const api = await signedIn(url)
  const loc = await api.post('/api/admin/locations', {
    data: { name: 'E2E library', mode: 'managed', path: libraryDir, catch_all: true },
  })
  if (!loc.ok()) throw new Error(`making the library location failed: ${loc.status()} ${await loc.text()}`)

  // No looking packs up on real stores from a test run.
  const settings = await api.put('/api/admin/settings', { data: { store_lookup: false } })
  if (!settings.ok()) throw new Error(`turning off store lookups failed: ${settings.status()} ${await settings.text()}`)

  const uploaded = []
  for (const p of seedPacks()) uploaded.push(await uploadPack(api, p.name, p.files))
  await waitReady(
    api,
    uploaded.map((p) => p.id),
  )
  // One pack with store details (no store page link, so nothing is fetched), for the packs page's store filter.
  const zipped = uploaded.find((p) => p.name === ZIPPED)!
  const ctx = await api.put(`/api/packs/${zipped.id}/context`, {
    data: { store: STORE, license_tier: 'Personal', store_text: 'Shiny coins and gems for a platformer.' },
  })
  if (!ctx.ok()) throw new Error(`saving store details failed: ${ctx.status()} ${await ctx.text()}`)
  const packs = await waitReady(
    api,
    uploaded.map((p) => p.id),
  )

  await waitIdle(api)

  const seed: Seed = { packs: {}, assets: {} }
  for (const p of packs) {
    seed.packs[p.name] = p.id
    for (const a of await assetsOf(api, p.id)) seed.assets[`${p.name}/${a.name}`] = a.id
  }
  process.env.HOARD_E2E_URL = url
  process.env.HOARD_E2E_SEED = JSON.stringify(seed)

  mkdirSync(dirname(AUTH_FILE), { recursive: true })
  await api.storageState({ path: AUTH_FILE })
  await api.dispose()
}
