import { existsSync } from 'node:fs'
import { signedIn } from './support/api'
import { expect, test } from './support/test'

/** A SoundFont of one looping sine, as small as the format allows (the server's own tests build the same one). */
function tinySf2(): Buffer {
  const u16 = (...v: number[]) => Buffer.concat(v.map((x) => Buffer.from([x & 0xff, x >> 8])))
  const u32 = (x: number) => {
    const b = Buffer.alloc(4)
    b.writeUInt32LE(x)
    return b
  }
  const chunk = (id: string, body: Buffer) =>
    Buffer.concat([Buffer.from(id, 'ascii'), u32(body.length), body, body.length % 2 ? Buffer.alloc(1) : Buffer.alloc(0)])
  const list = (kind: string, chunks: Buffer[]) => chunk('LIST', Buffer.concat([Buffer.from(kind, 'ascii'), ...chunks]))
  const name = (n: string) => {
    const b = Buffer.alloc(20)
    b.write(n, 'ascii')
    return b
  }
  const len = 10_000
  const smpl = Buffer.alloc(len * 2 + 92)
  for (let i = 0; i < len; i++) smpl.writeInt16LE(Math.trunc(Math.sin((i / 100) * Math.PI * 2) * 20_000), i * 2)
  const phdr = Buffer.concat([name('Lead'), u16(0, 0, 0), Buffer.alloc(12), name('EOP'), u16(0, 0, 1), Buffer.alloc(12)])
  const inst = Buffer.concat([name('Sine'), u16(0), name('EOI'), u16(1)])
  const shdr = Buffer.concat([
    name('sine'),
    ...[0, len, 100, 9_900, 44_100].map(u32),
    Buffer.from([69, 0]),
    u16(0, 1),
    name('EOS'),
    Buffer.alloc(26),
  ])
  const info = list('INFO', [chunk('ifil', u16(2, 1)), chunk('INAM', Buffer.from('Tiny Font\0', 'ascii'))])
  const sdta = list('sdta', [chunk('smpl', smpl)])
  const pdta = list('pdta', [
    chunk('phdr', phdr),
    chunk('pbag', u16(0, 0, 1, 0)),
    chunk('pmod', Buffer.alloc(10)),
    chunk('pgen', u16(41, 0, 0, 0)),
    chunk('inst', inst),
    chunk('ibag', u16(0, 0, 4, 0)),
    chunk('imod', Buffer.alloc(10)),
    chunk('igen', u16(43, 0x30 | (0x54 << 8), 54, 1, 58, 69, 53, 0, 0, 0)),
    chunk('shdr', shdr),
  ])
  return chunk('RIFF', Buffer.concat([Buffer.from('sfbk', 'ascii'), info, sdta, pdta]))
}

test.describe('the MIDI SoundFont', () => {
  test.afterAll(async () => {
    const api = await signedIn(process.env.HOARD_E2E_URL!)
    await api.put('/api/admin/settings', { data: { soundfont: null } })
    await api.dispose()
  })

  test('one sent from this computer is kept on the server and played with, and goes once it isn’t', async ({ page }) => {
    await page.goto('/#settings/tools')
    const section = page.locator('section', { has: page.getByRole('heading', { name: 'MIDI SoundFont' }) })
    const file = page.getByTestId('soundfont-file')
    const field = section.getByLabel('SoundFont file')

    // Not a SoundFont: refused, and the setting stays as it was.
    await file.setInputFiles({ name: 'notes.sf2', mimeType: 'application/octet-stream', buffer: Buffer.from('RIFF\0\0\0\0junk') })
    await expect(page.getByText(/^notes\.sf2 isn’t a SoundFont MIDI can play with/)).toBeVisible()
    await expect(field).toHaveValue('')

    await file.setInputFiles({ name: 'Tiny Font.sf2', mimeType: 'application/octet-stream', buffer: tinySf2() })
    await expect(page.getByText('MIDI plays with Tiny Font.sf2 now')).toBeVisible()
    await expect(field).toHaveValue(/soundfonts[\\/]Tiny Font\.sf2$/)
    const kept = await field.inputValue()
    expect(existsSync(kept)).toBe(true)

    // Back to the default: the file sent goes.
    await section.getByRole('button', { name: /^Reset to the default/ }).click()
    await expect(field).toHaveValue('')
    await expect.poll(() => existsSync(kept)).toBe(false)
  })
})
