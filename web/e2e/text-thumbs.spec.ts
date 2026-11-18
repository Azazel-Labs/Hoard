import { assetsOf, uploadPack, waitReady } from './support/api'
import { expect, openPack, test, tile, unique } from './support/test'

/** A one-page PDF with a filled square on it, its cross-reference offsets worked out. */
function pdf(): Buffer {
  const content = '0.2 0.4 0.8 rg 20 20 160 160 re f'
  const objects = [
    '<< /Type /Catalog /Pages 2 0 R >>',
    '<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
    '<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Contents 4 0 R >>',
    `<< /Length ${content.length} >>\nstream\n${content}\nendstream`,
  ]
  let out = '%PDF-1.4\n'
  const offsets: number[] = []
  objects.forEach((body, i) => {
    offsets.push(out.length)
    out += `${i + 1} 0 obj\n${body}\nendobj\n`
  })
  const xref = out.length
  out += `xref\n0 ${objects.length + 1}\n0000000000 65535 f \n`
  out += offsets.map((o) => `${String(o).padStart(10, '0')} 00000 n \n`).join('')
  out += `trailer\n<< /Size ${objects.length + 1} /Root 1 0 R >>\nstartxref\n${xref}\n%%EOF\n`
  return Buffer.from(out, 'latin1')
}

const SCRIPT = `// Moves the player.
fn step(pos: f32, speed: f32) -> f32 {
    let next = pos + speed * 0.5;
    if next > 10.0 {
        return 10.0;
    }
    next
}
`

// Text has no picture of its own, so Hoard draws one: its lines as bars from the left, colored by what they're made
// of. A PDF is shown by its first page; a file that's blank gets none.
test('scripts, data and PDFs get a thumbnail drawn from what they hold', async ({ page, api }) => {
  const name = `E2E Text thumbs ${unique()}`
  const files = [
    { relpath: 'player.rs', data: Buffer.from(SCRIPT) },
    { relpath: 'level.json', data: Buffer.from('{\n  "name": "cave",\n  "size": [32, 18],\n  "dark": true\n}\n') },
    { relpath: 'notes.txt', data: Buffer.from('Made for the jam.\nUse freely.\n') },
    { relpath: 'manual.pdf', data: pdf() },
    { relpath: 'blank.txt', data: Buffer.from('   \n\n') },
  ]
  const pack = await uploadPack(api, name, files)
  try {
    await waitReady(api, [pack.id])
    const assets = await assetsOf(api, pack.id)
    const thumbs: Record<string, boolean> = {}
    for (const a of assets) {
      const detail = (await (await api.get(`/api/assets/${a.id}`)).json()) as { has_thumb?: boolean }
      thumbs[a.path] = detail.has_thumb === true
    }
    expect(thumbs).toEqual({ 'player.rs': true, 'level.json': true, 'notes.txt': true, 'manual.pdf': true, 'blank.txt': false })

    const script = assets.find((a) => a.path === 'player.rs')!
    const thumb = await api.get(`/api/assets/${script.id}/thumb`)
    expect(thumb.ok()).toBe(true)
    expect(thumb.headers()['content-type']).toMatch(/^image\//)

    await openPack(page, pack.id, name, '&sort=name')
    await expect(tile(page, script.name).locator('img')).toBeVisible()
  } finally {
    await api.post('/api/archive', { data: { packs: [pack.id] } })
    await api.post('/api/archive/purge', { data: { packs: [pack.id] } })
  }
})
