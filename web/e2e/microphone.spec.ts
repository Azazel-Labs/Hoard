import { mkdtempSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { uploadPack, waitReady } from './support/api'
import { pew, sound } from './support/files'
import { expect, openPack, test, unique } from './support/test'

// Chromium's pretend microphone plays this file, once, as what it hears.
const heard = join(mkdtempSync(join(tmpdir(), 'hoard-mic-')), 'pew.wav')
writeFileSync(heard, pew())
test.use({
  launchOptions: {
    args: ['--use-fake-ui-for-media-stream', '--use-fake-device-for-media-stream', `--use-file-for-fake-audio-capture=${heard}%noloop`],
  },
  permissions: ['microphone'],
})

test('a noise made into the microphone finds the sounds shaped like it', async ({ page, api }) => {
  const name = `E2E Microphone ${unique()}`
  // Named so the words can't help.
  const pack = await uploadPack(api, name, [
    { relpath: 'take_1.wav', data: sound('thud', 1) },
    { relpath: 'take_2.wav', data: sound('zap', 1) },
    { relpath: 'take_3.wav', data: sound('rain', 1) },
  ])
  try {
    await waitReady(api, [pack.id])

    // The recording is read as a shape, which searches like a word.
    const r = await api.post('/api/imitation', { data: pew(), headers: { 'Content-Type': 'audio/wav' } })
    expect(r.ok()).toBe(true)
    const shape = (await r.json()) as { imitation: string; seconds: number }
    expect(shape.imitation).toMatch(/^heard:/)
    expect(shape.seconds).toBeGreaterThan(0.2)
    expect(shape.seconds).toBeLessThan(1)
    const found = await api.get(`/api/assets?imitation=${encodeURIComponent(shape.imitation)}&pack=${pack.id}`)
    expect(((await found.json()) as { items: { path: string }[] }).items[0]?.path).toBe('take_2.wav')

    // Only the room: nothing to go on.
    const silent = await api.post('/api/imitation', { data: sound('rain', 1).subarray(0, 44), headers: { 'Content-Type': 'audio/wav' } })
    expect(silent.status()).toBe(400)

    // From the sidebar: record, stop, and the sounds shaped like it come first.
    await openPack(page, pack.id, name, '&sort=name')
    const mic = page.getByRole('button', { name: /^Make the noise into the microphone/ })
    await mic.click()
    const stop = page.getByRole('button', { name: 'Stop recording' })
    await expect(stop).toBeVisible()
    await page.waitForTimeout(2500)
    await stop.click()
    await expect(page).toHaveURL(/[?&]imitation=heard%3A/)
    await expect(page.locator('.active')).toContainText('sounds-like:Your noise')
    await expect(page.getByRole('searchbox', { name: 'Sounds like' })).toHaveAttribute('placeholder', 'Your noise')
    await expect(page.locator('.grid button.hit').first()).toHaveAttribute('aria-label', /^Take 2,/)
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
