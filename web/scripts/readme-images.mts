// Makes the README's screenshots and the GitHub social preview card (docs/img/).
//
//   cargo build --release -p hoardd && (cd web && npm run build && node scripts/readme-images.mts)
//
// It fetches a demo library of CC0 art (models from Poly Haven, as glTF and USD, surfaces from ambientCG, sounds,
// sprite sheets and fonts from Kenney by way of OpenGameArt) into target/readme-assets (once; later runs reuse it),
// starts a throwaway hoardd on its own data folder, uploads the art as packs, waits for thumbnails, then screenshots
// the portal in dark and light and lays the card out around the dark hero. The desktop shot is the portal as the
// desktop app shows it, drawn in a Windows 11 window.
import { execFileSync, spawn, type ChildProcess } from 'node:child_process'
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs'
import { createServer } from 'node:net'
import { tmpdir } from 'node:os'
import { dirname, join, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { chromium, request, type APIRequestContext, type Browser, type Page } from '@playwright/test'

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const repo = resolve(web, '..')
const cache = join(repo, 'target', 'readme-assets')
const out = join(repo, 'docs', 'img')
const USER = 'demo'
const PASSWORD = 'demo-password-123'
const HEADERS = { 'X-Hoard': '1' }

/** Poly Haven models by pack; each comes as its 1k glTF with its textures. */
const MODELS: Record<string, string[]> = {
  'Poly Haven Props': [
    'treasure_chest', 'Lantern_01', 'Barrel_01', 'Camera_01', 'Television_01', 'wine_bottles_01',
    'rubber_duck_toy', 'Ukulele_01', 'chess_set', 'tea_set_01', 'food_apple_01',
  ],
  'Poly Haven Furniture': ['sofa_02', 'wooden_table_02', 'modern_arm_chair_01', 'GothicCabinet_01', 'Chandelier_03'],
  'Poly Haven Statues and Plants': [
    'marble_bust_01', 'horse_statue_01', 'concrete_cat_statue', 'potted_plant_04', 'rock_moss_set_02',
  ],
}
/** Poly Haven models that come as their 1k USDC instead. */
const USD_MODELS = ['Drill_01', 'boombox', 'brass_vase_01', 'Rockingchair_01', 'vintage_video_camera', 'ornate_mirror_01']
/** ambientCG surfaces, as their 1K JPG zips. */
const SURFACES = [
  'Bricks113A', 'Planks040', 'PavingStones152', 'Fabric086', 'Bark016', 'Wood096', 'Ground111', 'Concrete049',
  'Asphalt033', 'Tiles141', 'Grass005', 'Metal063', 'Rock064', 'Gravel043', 'Clay005',
]
/** Kenney's CC0 packs as OpenGameArt keeps them: the pack's name, its zip, and which of its files to add. */
const KENNEY: { pack: string; zip: string; keep: RegExp }[] = [
  { pack: 'Kenney Toon Characters', zip: 'kenney_toonCharacters1.zip', keep: /\/Tilesheet\/[^/]+HD\.(png|xml)$/ },
  { pack: 'Kenney Interface Sounds', zip: 'kenney_interfaceSounds.zip', keep: /(\.ogg|License\.txt)$/ },
  { pack: 'Kenney RPG Audio', zip: 'RPGsounds_Kenney.zip', keep: /\.ogg$/ },
  { pack: 'Kenney Fonts', zip: 'ken_fonts.zip', keep: /\.(ttf|txt)$/ },
]
/** The asset the hero opens. */
const HERO = 'treasure_chest'

async function download(url: string, to: string): Promise<void> {
  if (existsSync(to)) return
  const res = await fetch(url)
  if (!res.ok) throw new Error(`${url}: ${res.status}`)
  mkdirSync(dirname(to), { recursive: true })
  writeFileSync(to, Buffer.from(await res.arrayBuffer()))
}

type PolyFile = { url: string; include?: Record<string, { url: string }> }

/** A Poly Haven model at 1k, in `format` (`gltf` or `usd`), with the textures it uses. */
async function fetchModel(id: string, format: 'gltf' | 'usd', ext: string): Promise<void> {
  const dir = join(cache, `polyhaven-${format}`, id)
  if (existsSync(join(dir, `${id}_1k.${ext}`))) return
  const files = (await (await fetch(`https://api.polyhaven.com/files/${id}`)).json()) as Record<string, Record<string, Record<string, PolyFile>>>
  const f = files[format]['1k'][format]
  for (const [path, inc] of Object.entries(f.include ?? {})) await download(inc.url, join(dir, path))
  await download(f.url, join(dir, `${id}_1k.${ext}`))
  console.log(`fetched ${id}.${ext}`)
}

/** Unpack a zip: Windows' own bsdtar reads zips (Git's GNU tar doesn't); elsewhere, unzip. */
function unzip(zip: string, dir: string): void {
  mkdirSync(dir, { recursive: true })
  if (process.platform === 'win32') {
    execFileSync(join(process.env.SystemRoot ?? 'C:\Windows', 'System32', 'tar.exe'), ['-xf', zip, '-C', dir])
  } else {
    execFileSync('unzip', ['-oq', zip, '-d', dir])
  }
}

async function fetchArt(): Promise<void> {
  for (const id of Object.values(MODELS).flat()) await fetchModel(id, 'gltf', 'gltf')
  for (const id of USD_MODELS) await fetchModel(id, 'usd', 'usdc')
  for (const id of SURFACES) {
    const zip = join(cache, 'ambientcg', `${id}_1K-JPG.zip`)
    await download(`https://ambientcg.com/get?file=${id}_1K-JPG.zip`, zip)
    const dir = join(cache, 'ambientcg', `${id}_1K-JPG`)
    if (!existsSync(dir) || !readdirSync(dir).length) unzip(zip, dir)
  }
  for (const k of KENNEY) {
    const zip = join(cache, 'kenney', k.zip)
    await download(`https://opengameart.org/sites/default/files/${k.zip}`, zip)
    const dir = zip.replace(/\.zip$/, '')
    if (!existsSync(dir) || !readdirSync(dir).length) unzip(zip, dir)
  }
}

/** An ambientCG set's texture maps, without the Blender, USD, MaterialX and Godot files that come with them. */
const surfaceMaps = (dir: string) => files(dir).filter((f) => /_(Color|NormalGL|Roughness|AmbientOcclusion|Displacement|Metalness|Opacity)\.jpg$/.test(f))

function files(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory() ? files(join(dir, e.name)) : [join(dir, e.name)],
  )
}

async function upload(api: APIRequestContext, name: string, paths: string[], root: string): Promise<string> {
  const form = new FormData()
  form.append('name', name)
  for (const p of paths) {
    const rel = relative(root, p).replaceAll('\\', '/')
    form.append('relpath', rel)
    form.append('file', new Blob([new Uint8Array(readFileSync(p))]), rel.split('/').pop())
  }
  const res = await api.post('/api/packs', { multipart: form, timeout: 600_000 })
  if (!res.ok()) throw new Error(`upload of ${name}: ${res.status()} ${await res.text()}`)
  return ((await res.json()) as { id: string }).id
}

async function settle(api: APIRequestContext, ids: string[]): Promise<void> {
  const deadline = Date.now() + 15 * 60_000
  for (;;) {
    const packs = (await (await api.get('/api/packs')).json()) as { id: string; name: string; status: string; error?: string }[]
    const mine = packs.filter((p) => ids.includes(p.id))
    const failed = mine.find((p) => p.status === 'failed')
    if (failed) throw new Error(`${failed.name} failed: ${failed.error}`)
    const jobs = (await (await api.get('/api/admin/jobs')).json()) as { status: string }[]
    if (mine.every((p) => p.status === 'ready') && !jobs.some((j) => j.status === 'queued' || j.status === 'running')) return
    if (Date.now() > deadline) throw new Error('the library took too long to settle')
    await new Promise((r) => setTimeout(r, 1000))
  }
}

function freePort(): Promise<number> {
  return new Promise((ok, fail) => {
    const srv = createServer()
    srv.on('error', fail)
    srv.listen(0, '127.0.0.1', () => {
      const a = srv.address()
      srv.close(() => ok(typeof a === 'object' && a ? a.port : 0))
    })
  })
}

function hoardd(): string {
  if (process.env.HOARD_BIN) return resolve(process.env.HOARD_BIN)
  const exe = process.platform === 'win32' ? 'hoardd.exe' : 'hoardd'
  const found = ['target/release', 'target/debug', 'target/agent/release', 'target/agent/debug']
    .map((d) => join(repo, d, exe))
    .filter(existsSync)
    .sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs)
  if (!found.length) throw new Error('build hoardd first, or set HOARD_BIN')
  return found[0]
}

async function startServer(root: string): Promise<{ url: string; child: ChildProcess }> {
  const port = await freePort()
  const url = `http://127.0.0.1:${port}`
  mkdirSync(join(root, 'data'))
  mkdirSync(join(root, 'library'))
  const child = spawn(hoardd(), ['--listen', `127.0.0.1:${port}`, '--data-dir', join(root, 'data'), '--web-dir', join(web, 'dist')], {
    stdio: 'ignore',
  })
  for (let i = 0; i < 600; i++) {
    if (child.exitCode !== null) throw new Error(`hoardd exited (${child.exitCode})`)
    try {
      if ((await fetch(`${url}/api/auth/status`)).ok) return { url, child }
    } catch {
      // not up yet
    }
    await new Promise((r) => setTimeout(r, 100))
  }
  throw new Error('hoardd did not start')
}

/** The demo assets each shot opens, by kind and a word of their name. */
type Picks = Record<'hero' | 'page' | 'material' | 'sheet' | 'sound' | 'desktop', string>

async function find(api: APIRequestContext, q: string, kind: string): Promise<string> {
  const found = (await (await api.get('/api/assets', { params: { q, kind, limit: 5 } })).json()) as { items: { id: string }[] }
  if (!found.items.length) throw new Error(`no ${kind} called ${q}`)
  return found.items[0].id
}

async function fillLibrary(url: string, root: string): Promise<{ api: APIRequestContext; picks: Picks }> {
  const anon = await request.newContext({ baseURL: url, extraHTTPHeaders: HEADERS })
  await anon.post('/api/auth/setup', { data: { username: USER, password: PASSWORD } })
  await anon.dispose()
  const api = await request.newContext({ baseURL: url, extraHTTPHeaders: HEADERS })
  await api.post('/api/auth/login', { data: { username: USER, password: PASSWORD } })
  await api.post('/api/admin/locations', {
    data: { name: 'Library', mode: 'managed', path: join(root, 'library'), catch_all: true },
  })
  await api.put('/api/admin/settings', { data: { store_lookup: false, update_check: false } })
  const ids: string[] = []
  for (const [name, models] of Object.entries(MODELS)) {
    const dir = join(cache, 'polyhaven-gltf')
    ids.push(await upload(api, name, models.flatMap((m) => files(join(dir, m))), dir))
  }
  const usd = join(cache, 'polyhaven-usd')
  ids.push(await upload(api, 'Poly Haven USD', USD_MODELS.flatMap((m) => files(join(usd, m))), usd))
  const surfaces = join(cache, 'ambientcg')
  ids.push(await upload(api, 'ambientCG Surfaces', SURFACES.flatMap((s) => surfaceMaps(join(surfaces, `${s}_1K-JPG`))), surfaces))
  for (const k of KENNEY) {
    const dir = join(cache, 'kenney', k.zip.replace(/\.zip$/, ''))
    ids.push(await upload(api, k.pack, files(dir).filter((f) => k.keep.test(f.replaceAll('\\', '/'))), dir))
  }
  await settle(api, ids)
  const picks: Picks = {
    hero: await find(api, HERO.replaceAll('_', ' '), 'model'),
    page: await find(api, 'chandelier', 'model'),
    material: await find(api, 'Bricks113A', 'material'),
    sheet: await find(api, 'femaleAdventurer', 'sheet'),
    sound: await find(api, 'book flip 1', 'audio'),
    desktop: await find(api, 'lantern', 'model'),
  }
  return { api, picks }
}

/** Re-encode a PNG as WebP in the browser (Chromium encodes WebP; Node doesn't without a dependency). */
async function webp(browser: Browser, png: Buffer, quality = 0.9): Promise<Buffer> {
  const page = await browser.newPage()
  const data = await page.evaluate(
    async ([src, q]) => {
      const img = new Image()
      img.src = src as string
      await img.decode()
      const c = document.createElement('canvas')
      c.width = img.naturalWidth
      c.height = img.naturalHeight
      c.getContext('2d')!.drawImage(img, 0, 0)
      return c.toDataURL('image/webp', q as number)
    },
    [`data:image/png;base64,${png.toString('base64')}`, quality],
  )
  await page.close()
  return Buffer.from(data.split(',')[1], 'base64')
}

/** A screenshot: where it opens, what it does there before the picture is taken, and whether it's the desktop app. */
type Shot = { name: string; path: (p: Picks) => string; act?: (page: Page) => Promise<void>; desktop?: boolean }

const panel = (page: Page) => page.getByRole('complementary', { name: 'Asset details' })

// By name, so models and surfaces mix (newest would be the surfaces, whose materials are made last).
const SHOTS: Shot[] = [
  // The library with the treasure chest open beside it.
  { name: 'hero', path: (p) => `/?sort=name&kind=model#asset=${p.hero}` },
  // A model over the whole page: the 3D viewer big, its facts and downloads beside it.
  {
    name: 'page',
    path: (p) => `/?sort=name#asset=${p.page}`,
    act: async (page) => {
      await panel(page).getByRole('button', { name: 'Show full page' }).click()
    },
  },
  // A surface in the full-screen material viewer.
  {
    name: 'material',
    path: (p) => `/?sort=name&kind=material#asset=${p.material}`,
    act: async (page) => {
      await panel(page).getByRole('button', { name: 'Open the full-screen material viewer' }).click()
      await page.getByRole('dialog').getByRole('group', { name: 'Maps' }).waitFor({ timeout: 60_000 })
    },
  },
  // A sprite sheet over the whole page, its animation playing beside its sprites.
  {
    name: 'sheet',
    path: (p) => `/?sort=name&kind=sheet#asset=${p.sheet}`,
    act: async (page) => {
      await panel(page).getByRole('button', { name: 'Show full page' }).click()
    },
  },
  // Sounds: a grid of waveforms and one sound open.
  { name: 'sound', path: (p) => `/?sort=name&kind=audio#asset=${p.sound}` },
  // A search across kinds: models, materials and textures that are wood.
  { name: 'search', path: () => `/?q=wood` },
  // The desktop app's window: the library on this computer, a lantern open.
  { name: 'desktop', path: (p) => `/?sort=name#asset=${p.desktop}`, desktop: true },
]

async function shoot(browser: Browser, api: APIRequestContext, url: string, picks: Picks, shot: Shot, scheme: 'dark' | 'light'): Promise<Buffer> {
  const ctx = await browser.newContext({
    baseURL: url,
    storageState: await api.storageState(),
    viewport: { width: 1440, height: 860 },
    deviceScaleFactor: 2,
    colorScheme: scheme,
    reducedMotion: 'no-preference',
  })
  const page = await ctx.newPage()
  // The desktop app's portal is this one told it's the desktop app: no accounts, guests or server settings.
  if (shot.desktop) {
    await page.route('**/api/auth/status', async (route) => {
      const res = await route.fetch()
      await route.fulfill({ response: res, json: { ...(await res.json()), desktop: true } })
    })
  }
  await page.goto(shot.path(picks))
  if (shot.path(picks).includes('#asset=')) await panel(page).waitFor()
  await page.waitForLoadState('networkidle')
  await shot.act?.(page)
  // Thumbnails decoded, and the 3D previews given time to load their textures and draw.
  await page.waitForFunction(() => [...document.images].every((i) => i.complete))
  await page.waitForTimeout(4000)
  const png = await page.screenshot()
  await ctx.close()
  return shot.desktop ? framed(browser, png, scheme) : png
}

/** Where the window sits in the desktop shot, around the 1440 × 860 portal and its 32 px title bar. */
const MARGIN = 56
const TITLE_BAR = 32

/** The portal in a Windows 11 window titled Hoard, its title bar following the color scheme, on a plain backdrop. */
async function framed(browser: Browser, shot: Buffer, scheme: 'dark' | 'light'): Promise<Buffer> {
  const icon = readFileSync(join(web, 'public/favicon.svg'), 'utf8')
  const dark = scheme === 'dark'
  const glyph = (d: string) => `<span><svg viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1">${d}</svg></span>`
  const page = await browser.newPage({ viewport: { width: 1440 + 2 * MARGIN, height: 860 + TITLE_BAR + 2 * MARGIN }, deviceScaleFactor: 2 })
  await page.setContent(`<!doctype html><html><head><style>
  * { box-sizing: border-box; margin: 0; }
  body { height: 100vh; overflow: hidden; display: grid; place-items: center;
         background: ${dark ? 'radial-gradient(120% 140% at 0% 0%, #2b3550 0%, #171c28 55%, #10141c 100%)' : 'radial-gradient(120% 140% at 0% 0%, #eef3fa 0%, #d9e2ef 55%, #c7d3e4 100%)'}; }
  .win { width: 1440px; border-radius: 8px; overflow: hidden; background: ${dark ? '#202020' : '#f3f3f3'};
         box-shadow: 0 0 0 1px ${dark ? '#ffffff26' : '#0000001f'}, 0 24px 64px ${dark ? '#000a' : '#1c27404d'}, 0 4px 14px ${dark ? '#0006' : '#1c274026'}; }
  .bar { height: ${TITLE_BAR}px; display: flex; align-items: center; color: ${dark ? '#ffffff' : '#000000'};
         font: 12px 'Segoe UI Variable Text', 'Segoe UI', system-ui, sans-serif; }
  .bar > svg { width: 16px; height: 16px; margin: 0 8px 0 12px; }
  .caps { margin-left: auto; display: flex; height: 100%; }
  .caps span { width: 46px; display: grid; place-items: center; }
  .caps svg { width: 10px; height: 10px; }
  .shot { display: block; width: 1440px; height: 860px; }
</style></head><body><div class="win">
  <div class="bar">${icon}<span>Hoard</span><div class="caps">
    ${glyph('<path d="M0 5.5h10"/>')}${glyph('<rect x="0.5" y="0.5" width="9" height="9" rx="1.5"/>')}${glyph('<path d="M0.5 0.5l9 9M9.5 0.5l-9 9"/>')}
  </div></div>
  <img class="shot" src="data:image/png;base64,${shot.toString('base64')}">
</div></body></html>`)
  await page.waitForFunction(() => [...document.images].every((i) => i.complete))
  const png = await page.screenshot()
  await page.close()
  return png
}

function card(shot: Buffer): string {
  const font = readFileSync(join(web, 'node_modules/@fontsource-variable/recursive/files/recursive-latin-wght-normal.woff2'))
  const icon = readFileSync(join(web, 'public/favicon.svg'), 'utf8')
  return `<!doctype html><html><head><style>
  @font-face { font-family: Recursive; src: url(data:font/woff2;base64,${font.toString('base64')}) format('woff2'); font-weight: 300 1000; }
  * { box-sizing: border-box; margin: 0; }
  body { width: 1280px; height: 640px; overflow: hidden; font-family: Recursive, sans-serif; color: #e8eaf0;
         background: radial-gradient(120% 140% at 0% 0%, #273047 0%, #161b26 55%, #10141c 100%); position: relative; }
  .text { position: absolute; left: 72px; top: 0; bottom: 0; width: 470px; display: flex; flex-direction: column; justify-content: center; gap: 28px; }
  .brand { display: flex; align-items: center; gap: 22px; }
  .brand svg { width: 84px; height: 84px; }
  h1 { font-size: 92px; font-weight: 750; letter-spacing: -0.02em; line-height: 1; }
  p { font-size: 33px; line-height: 1.28; font-weight: 420; color: #c3c8d4; }
  ul { list-style: none; padding: 0; display: flex; flex-wrap: wrap; gap: 10px; }
  li { font-size: 19px; font-weight: 560; padding: 7px 14px; border-radius: 999px; background: #ffffff12; color: #d6dae3; border: 1px solid #ffffff1a; }
  /* The results and the open asset, without the filters or the top bar: the window is 1440 wide, drawn at 0.6. */
  .shot { position: absolute; left: 580px; top: 80px; width: 740px; height: 480px; border-radius: 14px; overflow: hidden;
          box-shadow: 0 30px 80px #0009, 0 0 0 1px #ffffff1f; }
  .shot img { display: block; width: 864px; margin: -36px 0 0 -150px; }
</style></head><body>
  <div class="text">
    <div class="brand">${icon}<h1>Hoard</h1></div>
    <p>A self-hosted asset library for game development.</p>
    <ul><li>3D preview</li><li>Materials</li><li>Sprite sheets</li><li>Sounds</li><li>Auto-tagging</li><li>Engine plugins</li><li>MCP</li></ul>
  </div>
  <div class="shot"><img src="data:image/png;base64,${shot.toString('base64')}"></div>
</body></html>`
}

async function main(): Promise<void> {
  if (!existsSync(join(web, 'dist/index.html'))) throw new Error('run `npm run build` in web first')
  await fetchArt()
  const root = mkdtempSync(join(tmpdir(), 'hoard-readme-'))
  const { url, child } = await startServer(root)
  const browser = await chromium.launch({ args: ['--use-angle=default', '--ignore-gpu-blocklist'] })
  try {
    const { api, picks } = await fillLibrary(url, root)
    mkdirSync(out, { recursive: true })
    const made: string[] = []
    let hero: Buffer | undefined
    // Only some of them when named: `node scripts/readme-images.mts sheet sound`.
    const only = process.argv.slice(2)
    for (const shot of SHOTS.filter((s) => !only.length || only.includes(s.name))) {
      for (const scheme of ['dark', 'light'] as const) {
        const png = await shoot(browser, api, url, picks, shot, scheme)
        writeFileSync(join(cache, `last-${shot.name}-${scheme}.png`), png)
        writeFileSync(join(out, `${shot.name}-${scheme}.webp`), await webp(browser, png))
        made.push(`${shot.name}-${scheme}.webp`)
        if (shot.name === 'hero' && scheme === 'dark') hero = png
      }
    }
    if (hero) {
      const page = await browser.newPage({ viewport: { width: 1280, height: 640 } })
      await page.setContent(card(hero))
      await page.evaluate(() => document.fonts.ready)
      writeFileSync(join(out, 'social.png'), await page.screenshot())
      await page.close()
      made.push('social.png')
    }
    await api.dispose()
    for (const f of made) console.log(`${f}: ${Math.round(statSync(join(out, f)).size / 1024)} KB`)
  } finally {
    await browser.close()
    child.kill()
    await new Promise((r) => setTimeout(r, 500))
    rmSync(root, { recursive: true, force: true })
  }
}

await main()
