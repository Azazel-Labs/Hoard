import { assetsOf, uploadPack, waitReady } from './support/api'
import { chords, sound } from './support/files'
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
          const r = (await (await api.get(`/api/assets?similar=${thud.id}&pack=${pack.id}&limit=20`)).json()) as { items: { path: string }[] }
          found = r.items.map((a) => a.path)
          // Until every sound has been listened to, the ones still waiting are missing.
          return found.slice(0, 2).sort()
        },
        { timeout: 60_000 },
      )
      .toEqual(['thud/take_2.wav', 'thud/take_3.wav'])
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

test('music’s tempo and key are heard, shown in its details and searched by in the sidebar', async ({ page, api }) => {
  const name = `E2E Music ${unique()}`
  const pack = await uploadPack(api, name, [
    { relpath: 'loop.wav', data: chords(120, 8) },
    { relpath: 'rain.wav', data: sound('rain', 1) },
  ])
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const loop = assets.find((a) => a.path === 'loop.wav')!
    const detail = async () =>
      (await (await api.get(`/api/assets/${loop.id}`)).json()) as { meta: Record<string, unknown>; tags: { tag: string }[] }
    await expect.poll(async () => (await detail()).meta.measured_key, { timeout: 60_000 }).toBe('A minor')
    const d = await detail()
    expect(d.meta).toMatchObject({ measured_bpm: 120, measured_bpm_alt: 60 })
    expect(d.tags.map((t) => t.tag)).toEqual(expect.arrayContaining(['bpm:120', 'key:a-minor']))

    // By tempo, counted either way, and by key, with the keys that mix with it or without.
    const found = async (q: string) =>
      ((await (await api.get(`/api/assets?pack=${pack.id}&${q}`)).json()) as { items: { path: string }[] }).items.map((a) => a.path)
    expect(await found('bpm_min=115&bpm_max=125')).toEqual(['loop.wav'])
    expect(await found('bpm_max=65')).toEqual(['loop.wav'])
    expect(await found('bpm_min=130')).toEqual([])
    expect(await found('key=8A')).toEqual(['loop.wav'])
    expect(await found('key=C%20major&compatible_keys=true')).toEqual(['loop.wav'])
    expect(await found('key=C%20major')).toEqual([])

    await openPack(page, pack.id, name)
    await tile(page, loop.name).click()
    const panel = page.getByRole('complementary', { name: 'Asset details' })
    const row = (label: string) => panel.locator('dt', { hasText: new RegExp(`^${label}$`) }).locator('xpath=following-sibling::dd[1]')
    await expect(row('Tempo')).toHaveText('about 120 BPM (or 60), as heard')
    await expect(row('Key')).toHaveText('A minor (8A), as heard')

    // The sidebar's Music section.
    const loopTile = tile(page, loop.name)
    const rainTile = tile(page, assets.find((a) => a.path === 'rain.wav')!.name)
    const from = page.getByRole('spinbutton', { name: 'BPM' })
    await from.fill('100')
    await from.press('Enter')
    await expect(rainTile).toBeHidden()
    await expect(loopTile).toBeVisible()
    await from.fill('130')
    await from.press('Enter')
    await expect(loopTile).toBeHidden()
    await from.fill('')
    await from.press('Enter')
    await expect(rainTile).toBeVisible()
    await page.getByRole('combobox', { name: 'Key' }).selectOption('e-minor')
    await expect(rainTile).toBeHidden()
    await expect(loopTile).toBeVisible()
    await page.getByRole('checkbox', { name: 'And keys that mix with it' }).uncheck()
    await expect(loopTile).toBeHidden()
    await expect(page).toHaveURL(/[?&]key=e-minor&mix=0/)
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

    // Searching sounds, a word in the search field is also a written noise.
    await openPack(page, pack.id, name, '&kind=audio')
    const box = page.getByRole('searchbox', { name: 'Search assets' })
    await expect(box).toHaveAttribute('placeholder', /pew/)
    await box.fill('pew')
    await expect(page).toHaveURL(/[?&]q=pew/)
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

test('an SFZ instrument plays a few notes on its samples, which go with it', async ({ api }) => {
  const name = `E2E Instrument ${unique()}`
  const sfz = '<control> default_path=Samples\\\n<region> sample=Hit.wav pitch_keycenter=60 lokey=c3 hikey=c6\n'
  const pack = await uploadPack(api, name, [
    { relpath: 'kalimba.sfz', data: Buffer.from(sfz) },
    { relpath: 'Samples/Hit.wav', data: sound('zap', 1) },
  ])
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const at = (path: string) => assets.find((a) => a.path === path)!
    const detail = (await (await api.get(`/api/assets/${at('kalimba.sfz').id}`)).json()) as {
      kind: string
      has_thumb: boolean
      meta: Record<string, unknown>
      links: { path: string; rel: string }[]
    }
    expect(detail.kind).toBe('audio')
    expect(detail.has_thumb).toBe(true)
    expect(detail.meta).toMatchObject({ preview: 'mp3', key_range: 'C3–C6', regions: 1, samples: 1 })
    expect(detail.links).toEqual([expect.objectContaining({ path: 'Samples/Hit.wav', rel: 'sample' })])
    // The sample is part of the instrument, so it stays out of the grid with the other supporting files.
    const shown = await api.get('/api/assets', { params: { pack: pack.id, limit: 500, facets: false, hide_supporting: true } })
    expect(((await shown.json()) as { items: { path: string }[] }).items.map((a) => a.path)).toEqual(['kalimba.sfz'])
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

test('an Opus file gets a waveform and converts to WAV, ffmpeg or not', async ({ api }) => {
  const name = `E2E Opus ${unique()}`
  const source = await uploadPack(api, `${name} source`, [{ relpath: 'laser.wav', data: sound('zap', 1) }])
  let pack: { id: string } | undefined
  try {
    await waitReady(api, [source.id])
    const [wav] = await assetsOf(api, source.id)
    const opus = await api.get(`/api/assets/${wav.id}/as/opus?wait=60`)
    expect(opus.status()).toBe(200)
    const bytes = await opus.body()
    expect(bytes.subarray(0, 4).toString()).toBe('OggS')

    pack = await uploadPack(api, name, [{ relpath: 'laser.opus', data: bytes }])
    await waitReady(api, [pack.id])
    const [asset] = await assetsOf(api, pack.id)
    const detail = (await (await api.get(`/api/assets/${asset.id}`)).json()) as { kind: string; has_thumb: boolean; meta: Record<string, unknown> }
    expect(detail.kind).toBe('audio')
    expect(detail.has_thumb).toBe(true)
    expect(detail.meta.peak_dbfs).toEqual(expect.any(Number))
    const back = await api.get(`/api/assets/${asset.id}/as/wav?wait=60`)
    expect(back.status()).toBe(200)
    expect((await back.body()).subarray(0, 4).toString()).toBe('RIFF')
  } finally {
    const packs = [source.id, ...(pack ? [pack.id] : [])]
    await api.post('/api/archive', { data: { packs } })
    await api.post('/api/archive/purge', { data: { packs } })
  }
})

/** A VGM file logging a tune on the SN76489 of Sega's consoles: an A, then an E, half a second each. */
function vgm(): Buffer {
  const header = Buffer.alloc(0x40)
  header.write('Vgm ', 0)
  header.writeUInt32LE(0x150, 0x08)
  header.writeUInt32LE(3_579_545, 0x0c)
  header.writeUInt32LE(2 * 30 * 735, 0x18)
  header.writeUInt32LE(0x40 - 0x34, 0x34)
  const tone = (period: number) => [0x50, 0x80 | (period & 0x0f), 0x50, period >> 4, 0x50, 0x90]
  const halfSecond = Array<number>(30).fill(0x62)
  const body = Buffer.from([...tone(254), ...halfSecond, ...tone(170), ...halfSecond, 0x50, 0x9f, 0x66])
  const file = Buffer.concat([header, body])
  file.writeUInt32LE(file.length - 4, 0x04)
  return file
}

test('a VGM chiptune plays through its emulated sound chip and converts to WAV', async ({ api }) => {
  const pack = await uploadPack(api, `E2E Chiptune ${unique()}`, [{ relpath: 'title-theme.vgm', data: vgm() }])
  try {
    await waitReady(api, [pack.id])
    const [asset] = await assetsOf(api, pack.id)
    const detail = (await (await api.get(`/api/assets/${asset.id}`)).json()) as {
      kind: string
      has_thumb: boolean
      meta: Record<string, unknown>
      tags: { tag: string }[]
    }
    expect(detail.kind).toBe('audio')
    expect(detail.has_thumb).toBe(true)
    expect(detail.meta).toMatchObject({ chips: ['SN76489'], preview: 'mp3' })
    expect(detail.meta.duration_secs).toBeCloseTo(1, 1)
    expect(detail.tags.map((t) => t.tag)).toContain('chiptune')
    const wav = await api.get(`/api/assets/${asset.id}/as/wav?wait=60`)
    expect(wav.status()).toBe(200)
    expect((await wav.body()).subarray(0, 4).toString()).toBe('RIFF')
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})

/** An FMOD sound bank (FSB5) holding two named quarter-second tones, 16-bit mono PCM at 44.1 kHz. */
function fsb(): Buffer {
  const tone = (hz: number) => {
    const pcm = Buffer.alloc(2 * 11_025)
    for (let i = 0; i < 11_025; i++) pcm.writeInt16LE(Math.round(Math.sin((2 * Math.PI * hz * i) / 44_100) * 12_000), 2 * i)
    return pcm
  }
  const headers = Buffer.alloc(16)
  const bodies: Buffer[] = []
  let offset = 0
  ;[tone(440), tone(660)].forEach((pcm, i) => {
    // Frames from bit 34, the offset (in 32-byte units) from bit 7, rate index 8 (44.1 kHz) from bit 1.
    headers.writeBigUInt64LE((BigInt(pcm.length / 2) << 34n) | (BigInt(offset / 32) << 7n) | (8n << 1n), 8 * i)
    const padded = Buffer.alloc(Math.ceil(pcm.length / 32) * 32)
    pcm.copy(padded)
    bodies.push(padded)
    offset += padded.length
  })
  const names = Buffer.alloc(32)
  names.writeUInt32LE(8, 0)
  names.writeUInt32LE(13, 4)
  names.write('beep\0boop\0', 8)
  const header = Buffer.alloc(0x3c)
  header.write('FSB5', 0)
  ;[1, 2, headers.length, names.length, offset, 2].forEach((v, i) => header.writeUInt32LE(v, 4 + 4 * i))
  return Buffer.concat([header, headers, names, ...bodies])
}

test('an FMOD sound bank plays its sounds in turn, lists their names and converts to WAV', async ({ api }) => {
  const pack = await uploadPack(api, `E2E Sound Bank ${unique()}`, [{ relpath: 'weapons.fsb', data: fsb() }])
  try {
    await waitReady(api, [pack.id])
    const [asset] = await assetsOf(api, pack.id)
    const detail = (await (await api.get(`/api/assets/${asset.id}`)).json()) as {
      kind: string
      has_thumb: boolean
      meta: Record<string, unknown>
      tags: { tag: string }[]
    }
    expect(detail.kind).toBe('audio')
    expect(detail.has_thumb).toBe(true)
    expect(detail.meta).toMatchObject({
      container: 'FMOD sound bank',
      codec: 'PCM',
      sounds: 2,
      sound_names: ['beep', 'boop'],
      preview: 'mp3',
    })
    expect(detail.meta.duration_secs).toBeCloseTo(0.8, 1)
    expect(detail.tags.map((t) => t.tag)).toContain('fmod')
    const wav = await api.get(`/api/assets/${asset.id}/as/wav?wait=60`)
    expect(wav.status()).toBe(200)
    expect((await wav.body()).subarray(0, 4).toString()).toBe('RIFF')
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
