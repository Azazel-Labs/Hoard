import { assetsOf, uploadPack, waitReady } from './support/api'
import { sound } from './support/files'
import { expect, openPack, test, tile, unique } from './support/test'

test('a sound’s More like this finds what sounds like it, whatever it’s called', async ({ page, api }) => {
  const name = `E2E Sounds ${unique()}`
  // Named so the words can't help: the thuds and the lasers are told apart by how they sound.
  const files = [
    ...[1, 2, 3].map((s) => ({ relpath: `thud/take_${s}.wav`, data: sound('thud', s) })),
    ...[1, 2].map((s) => ({ relpath: `zap/take_${s + 3}.wav`, data: sound('zap', s) })),
    ...[1, 2].map((s) => ({ relpath: `rain/take_${s + 5}.wav`, data: sound('rain', s) })),
  ]
  const pack = await uploadPack(api, name, files)
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const at = (path: string) => assets.find((a) => a.path === path)!
    const thud = at('thud/take_1.wav')

    // The sounds are listened to after they're in, in the background.
    let found: string[] = []
    await expect
      .poll(
        async () => {
          const r = (await (await api.get(`/api/assets?similar=${thud.id}&limit=20`)).json()) as { items: { path: string }[] }
          found = r.items.map((a) => a.path)
          return found.length
        },
        { timeout: 60_000 },
      )
      .toBeGreaterThan(0)
    expect(found.slice(0, 2).sort()).toEqual(['thud/take_2.wav', 'thud/take_3.wav'])
    expect(found).not.toContain('rain/take_6.wav')
    expect(found).not.toContain('thud/take_1.wav')

    await openPack(page, pack.id, name, '&sort=name')
    await tile(page, thud.name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    await panel.getByRole('button', { name: 'More like this' }).click()
    await expect(page).toHaveURL(/[?&]similar=/)
    const first = page.locator('.grid button.hit')
    await expect(first.first()).toBeVisible()
    for (const n of [0, 1]) {
      await expect(first.nth(n)).toHaveAttribute('aria-label', new RegExp(`^(${at('thud/take_2.wav').name}|${at('thud/take_3.wav').name}),`))
    }
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

test('a sound’s envelope, tone and pitch are measured, shown in its details and made into tags', async ({ page, api }) => {
  const name = `E2E Sound features ${unique()}`
  const pack = await uploadPack(api, name, [
    { relpath: 'laser.wav', data: sound('zap', 1) },
    { relpath: 'rain.wav', data: sound('rain', 1) },
  ])
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const at = (path: string) => assets.find((a) => a.path === path)!
    const detail = async (path: string) =>
      (await (await api.get(`/api/assets/${at(path).id}`)).json()) as { meta: Record<string, unknown>; tags: { tag: string }[] }

    // A laser: a sharp hit, a pure tone, falling in pitch.
    const laser = await detail('laser.wav')
    expect(laser.meta).toMatchObject({ envelope: 'hit' })
    expect(laser.meta.sweep_octaves as number).toBeLessThan(-1)
    expect(laser.tags.map((t) => t.tag)).toEqual(expect.arrayContaining(['envelope:hit', 'tone:tonal', 'pitch:falling']))
    // Rain: steady noise, which loops.
    const rain = await detail('rain.wav')
    expect(rain.meta).toMatchObject({ envelope: 'sustained', loops_cleanly: true })
    expect(rain.tags.map((t) => t.tag)).toEqual(expect.arrayContaining(['tone:noisy', 'loop']))

    // Searching for a tag finds what has it.
    const found = (await (await api.get(`/api/assets?pack=${pack.id}&q=${encodeURIComponent('pitch:falling')}`)).json()) as {
      items: { path: string }[]
    }
    expect(found.items.map((a) => a.path)).toEqual(['laser.wav'])

    await openPack(page, pack.id, name)
    await tile(page, at('laser.wav').name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const row = (label: string) => panel.locator('dt', { hasText: new RegExp(`^${label}$`) }).locator('xpath=following-sibling::dd[1]')
    await expect(row('Envelope')).toHaveText(/^hit, attack \d+ ms, tail \d+ ms$/)
    await expect(row('Tone')).toHaveText(/^brightness \d\.\d kHz, noise \d+%$/)
    await expect(row('Pitch')).toHaveText(/^falls \d\.\d octaves$/)
    await tile(page, at('rain.wav').name).click()
    await expect(row('Playback')).toHaveText('Loops cleanly')
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

test('a word imitating a sound finds the sounds shaped like it', async ({ page, api }) => {
  const name = `E2E Sound imitation ${unique()}`
  // Named so the words can't help.
  const pack = await uploadPack(api, name, [
    { relpath: 'take_1.wav', data: sound('thud', 1) },
    { relpath: 'take_2.wav', data: sound('zap', 1) },
    { relpath: 'take_3.wav', data: sound('rain', 1) },
  ])
  try {
    await waitReady(api, [pack.id])
    const first = async (word: string) => {
      const r = await api.get(`/api/assets?imitation=${encodeURIComponent(word)}&pack=${pack.id}`)
      return ((await r.json()) as { items: { path: string }[] }).items[0]?.path
    }
    expect(await first('pew')).toBe('take_2.wav')
    expect(await first('doom')).toBe('take_1.wav')
    expect(await first('shhhhhhh')).toBe('take_3.wav')

    await openPack(page, pack.id, name, '&sort=name')
    const box = page.getByRole('searchbox', { name: 'Sounds like' })
    await box.fill('pew')
    await box.press('Enter')
    await expect(page).toHaveURL(/[?&]imitation=pew/)
    await expect(page.locator('.active')).toContainText('sounds-like:pew')
    await expect(page.locator('.sort select option').first()).toHaveText('Sounds most like it')
    await expect(page.locator('.grid button.hit').first()).toHaveAttribute('aria-label', /^Take 2,/)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

test('an AI finds a sound over MCP by the word for it or by its own reading of it', async ({ api }) => {
  const name = `E2E Sound tools ${unique()}`
  const pack = await uploadPack(api, name, [
    { relpath: 'take_1.wav', data: sound('thud', 1) },
    { relpath: 'take_2.wav', data: sound('zap', 1) },
    { relpath: 'take_3.wav', data: sound('rain', 1) },
  ])
  type Sound = { id: string; path: string; sounds: string; measured: Record<string, unknown>; urls: { listen: string } }
  type Result = { isError: boolean; content: { text: string }[]; structuredContent: { sounds: Sound[]; reading?: Record<string, unknown> } & Sound }
  const call = async (tool: string, args: object) => {
    const r = await api.post('/mcp', { data: { jsonrpc: '2.0', id: 1, method: 'tools/call', params: { name: tool, arguments: args } } })
    return ((await r.json()) as { result: Result }).result
  }
  try {
    await waitReady(api, [pack.id])
    const list = await api.post('/mcp', { data: { jsonrpc: '2.0', id: 1, method: 'tools/list' } })
    const tools = ((await list.json()) as { result: { tools: { name: string }[] } }).result.tools.map((t) => t.name)
    expect(tools).toEqual(expect.arrayContaining(['search_sounds', 'describe_sound', 'sounds_like']))

    // The word, read by Hoard: a sharp start and a falling vowel is the laser.
    const pew = (await call('search_sounds', { imitation: 'pew', pack_id: pack.id })).structuredContent
    expect(pew.reading).toMatchObject({ attack: 'sharp', pitch: 'falling' })
    const zap = pew.sounds[0]
    expect(zap.path).toBe('take_2.wav')
    expect(zap.sounds).toContain('pitch falls')
    expect(typeof zap.measured.brightness_hz).toBe('number')
    expect(zap.urls.listen).toMatch(/\/api\/assets\/[^/]+\/file$/)

    // Or the caller's own reading, with no word at all.
    const long = (await call('search_sounds', { shape: { texture: 'noisy', seconds: 3 }, pack_id: pack.id })).structuredContent
    expect(long.sounds[0].path).toBe('take_3.wav')
    const words = await call('search_sounds', { shape: { pitch: 'up' } })
    expect(words.isError).toBe(true)
    expect(words.content[0].text).toContain('falling, steady, rising')

    const described = (await call('describe_sound', { id: zap.id })).structuredContent
    expect(described.path).toBe('take_2.wav')
    expect(described.sounds).toContain('starts sharply')

    const alike = (await call('sounds_like', { id: zap.id })).structuredContent
    expect(alike.sounds.map((s) => s.id)).not.toContain(zap.id)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
