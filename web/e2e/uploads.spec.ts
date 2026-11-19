import { createHash } from 'node:crypto'
import { assetsOf, waitReady, type Pack } from './support/api'
import { zip } from './support/files'
import { expect, test, unique } from './support/test'

interface Session {
  pack: Pack
  part_bytes: number
  files: { path: string; size: number; received: number; parts?: number[] }[]
  resumed?: boolean
  claim?: string
}

test.describe('resumable uploads', () => {
  test('a big file goes up in parts at once and in any order, and arrives whole', async ({ api }) => {
    // A session is picked up again by its list of files, so each attempt names its file afresh.
    const name = `e2e parts ${unique()}`
    const file = `notes-${unique()}.txt`
    const opened = await api.post('/api/uploads', { data: { name, files: [{ path: file, size: 1 }] } })
    expect(opened.status()).toBe(201)
    const partBytes = ((await opened.json()) as Session).part_bytes
    await api.delete(`/api/uploads/${((await opened.json()) as Session).pack.id}`)

    // Two and a half parts of text whose every line says where it is, so a part in the wrong place shows.
    const size = Math.floor(partBytes * 2.5)
    const text = Buffer.alloc(size)
    for (let at = 0; at < size; at += 16) text.write(`${at.toString(16).padStart(15, '0')}\n`.slice(0, size - at), at)
    const session = (await (
      await api.post('/api/uploads', { data: { name, files: [{ path: file, size }] } })
    ).json()) as Session
    const base = `/api/uploads/${session.pack.id}`
    const part = (n: number) =>
      api.put(`${base}/chunk`, {
        params: { path: file, offset: n * partBytes },
        data: text.subarray(n * partBytes, Math.min(size, (n + 1) * partBytes)),
      })

    // The last part first, then the other two at the same time.
    expect((await part(2)).status()).toBe(200)
    const status = (await (await api.get(base)).json()) as Session
    expect(status.files[0].parts).toEqual([2])
    expect(status.files[0].received).toBe(size - 2 * partBytes)
    const [first, second] = await Promise.all([part(0), part(1)])
    expect([first.status(), second.status()]).toEqual([200, 200])

    // A part sent again counts once; an offset that isn't where a part starts is refused.
    expect(await (await part(1)).json()).toEqual({ received: size, size })
    const wrong = await api.put(`${base}/chunk`, { params: { path: file, offset: 5 }, data: Buffer.from('x') })
    expect(wrong.status()).toBe(400)

    expect((await api.post(`${base}/complete`)).status()).toBe(202)
    await waitReady(api, [session.pack.id])
    const [asset] = await assetsOf(api, session.pack.id)
    const back = await (await api.get(`/api/assets/${asset.id}/file`)).body()
    expect(back.length).toBe(size)
    expect(createHash('sha256').update(back).digest('hex')).toBe(createHash('sha256').update(text).digest('hex'))
  })

  test('cancelling an upload the same files were dropped again for leaves it to the new one', async ({ api }) => {
    // A failed upload, retried by dropping the same files again: the server picks the same session back up.
    const stem = `retried-${unique()}`
    const files = [`${stem}/a.txt`, `${stem}/b.txt`].map((path) => ({ path, data: Buffer.from(`${path}
`) }))
    const list = { name: stem, files: files.map((f) => ({ path: f.path, size: f.data.length })) }
    const first = (await (await api.post('/api/uploads', { data: list })).json()) as Session
    const base = `/api/uploads/${first.pack.id}`
    expect((await api.put(`${base}/chunk`, { params: { path: files[0].path, offset: 0 }, data: files[0].data })).status()).toBe(200)
    const again = await api.post('/api/uploads', { data: list })
    expect(again.status()).toBe(200)
    const second = (await again.json()) as Session
    expect(second.resumed).toBe(true)
    expect(second.pack.id).toBe(first.pack.id)
    expect(second.claim).toBeTruthy()
    expect(second.claim).not.toBe(first.claim)

    // The first one's cancelled, as a person tidying up the failed row would: the second carries on regardless.
    expect((await api.delete(base, { params: { claim: first.claim! } })).status()).toBe(204)
    const status = (await (await api.get(base)).json()) as Session
    expect(status.files.map((f) => f.received)).toEqual([files[0].data.length, 0])
    expect((await api.put(`${base}/chunk`, { params: { path: files[1].path, offset: 0 }, data: files[1].data })).status()).toBe(200)
    expect((await api.post(`${base}/complete`)).status()).toBe(202)
    await waitReady(api, [first.pack.id])
    expect(await assetsOf(api, first.pack.id)).toHaveLength(2)
    await api.delete(`/api/packs/${first.pack.id}`)

    // The latest registration cancels it, and so does a client that doesn't say which it is.
    for (const claim of [true, false]) {
      const files = [{ path: `${stem}-${claim}.txt`, size: 3 }]
      await api.post('/api/uploads', { data: { files } })
      const latest = (await (await api.post('/api/uploads', { data: { files } })).json()) as Session
      const at = `/api/uploads/${latest.pack.id}`
      expect((await api.delete(at, claim ? { params: { claim: latest.claim! } } : {})).status()).toBe(204)
      expect((await api.get(at)).status()).toBe(404)
    }
  })
})

test.describe('the import queue', () => {
  test('an archive picked in pieces is one pack, unpacked from all of them', async ({ page, api }) => {
    test.setTimeout(120_000)
    const stem = `pieces-${unique()}`
    const whole = zip(
      ['notes', 'more'].map((n) => ({ path: `${stem}/${n}.txt`, data: Buffer.from(`${n} for ${stem}\n`.repeat(40)) })),
    )
    const half = Math.ceil(whole.length / 2)
    const pieces = [whole.subarray(0, half), whole.subarray(half)].map((buffer, i) => ({
      name: `${stem}.zip.00${i + 1}`,
      mimeType: 'application/octet-stream',
      buffer,
    }))
    await page.goto('/')
    await page.locator('input[type=file]:not([webkitdirectory])').first().setInputFiles(pieces)
    const queue = page.getByRole('region', { name: 'Imports' })
    await expect(queue.getByText(`${stem}.zip`, { exact: true })).toBeVisible()

    let mine: Pack[] = []
    await expect
      .poll(
        async () => {
          mine = ((await (await api.get('/api/packs')).json()) as Pack[]).filter((p) => p.name.includes(stem))
          return mine.length
        },
        { timeout: 60_000 },
      )
      .toBe(1)
    await waitReady(api, [mine[0].id])
    const paths = (await assetsOf(api, mine[0].id)).map((a) => a.path).sort()
    expect(paths).toEqual(['more.txt', 'notes.txt'])
    await api.delete(`/api/packs/${mine[0].id}`)
  })

  test('files dragged over the window show where to drop them, counted, until they leave', async ({ page }) => {
    await page.goto('/')
    const files = await page.evaluateHandle(() => {
      const dt = new DataTransfer()
      dt.items.add(new File(['a'], 'a.png', { type: 'image/png' }))
      dt.items.add(new File(['b'], 'b.png', { type: 'image/png' }))
      return dt
    })
    await page.locator('.app').dispatchEvent('dragenter', { dataTransfer: files })
    await expect(page.getByText('Drop 2 items to add them')).toBeVisible()
    await page.locator('.app').dispatchEvent('dragleave', { dataTransfer: files })
    await expect(page.getByText('Drop 2 items to add them')).toHaveCount(0)
  })

  test('a failed import dropped again carries on in its own row', async ({ page, api }) => {
    test.setTimeout(120_000)
    const stem = `failed-${unique()}`
    const file = { name: `${stem}.txt`, mimeType: 'text/plain', buffer: Buffer.from(`notes for ${stem}
`) }
    // The first try at its files is turned away, for good as far as the queue knows.
    let refused = false
    await page.route('**/api/uploads/*/batch', (route) => {
      if (refused) return route.continue()
      refused = true
      return route.fulfill({ status: 400, contentType: 'application/json', body: JSON.stringify({ error: 'held back' }) })
    })
    await page.goto('/')
    const input = page.locator('input[type=file]:not([webkitdirectory])').first()
    await input.setInputFiles(file)
    const queue = page.getByRole('region', { name: 'Imports' })
    await expect(queue.getByText('held back')).toBeVisible()

    await input.setInputFiles(file)
    await expect(queue.getByText('Uploaded, now unpacking')).toBeVisible({ timeout: 30_000 })
    await expect(queue.getByText(file.name)).toHaveCount(1)
    const mine = ((await (await api.get('/api/packs')).json()) as Pack[]).filter((p) => p.name.includes(stem))
    expect(mine).toHaveLength(1)
    await api.delete(`/api/packs/${mine[0].id}`)
  })

  test('folds down to a pill, and an import a reload cut short carries on when its file is picked again', async ({ page, api }) => {
    test.setTimeout(120_000)
    const stem = `resume-${unique()}`
    const file = { name: `${stem}.txt`, mimeType: 'text/plain', buffer: Buffer.from(`notes for ${stem}\n`) }
    // Hold the upload's files back, so the import is still going when the page reloads.
    await page.route('**/api/uploads/*/batch', () => undefined)
    await page.goto('/')
    await page.locator('input[type=file]:not([webkitdirectory])').first().setInputFiles(file)
    const queue = page.getByRole('region', { name: 'Imports' })
    await expect(queue.getByText(file.name)).toBeVisible()

    await queue.getByRole('button', { name: 'Minimize' }).click()
    await expect(queue).toBeHidden()
    const pill = page.getByRole('button', { name: /Importing 1 · \d+%.*Show imports/ })
    await expect(pill).toBeVisible()
    await pill.click()
    await expect(queue.getByText(file.name)).toBeVisible()

    // The reload doesn't warn: the browser remembers the import, and shows it again afterwards.
    let warned = false
    page.on('dialog', (d) => {
      warned = true
      void d.accept()
    })
    await page.reload()
    expect(warned).toBe(false)
    await page.unroute('**/api/uploads/*/batch')
    await expect(queue.getByText('Interrupted')).toBeVisible()
    await expect(queue.getByText('Pick the same file again')).toBeVisible()

    // A different file doesn't carry it on; the same one does, and lands as one pack.
    const chooser = page.waitForEvent('filechooser')
    await queue.getByRole('button', { name: 'Resume' }).click()
    await (await chooser).setFiles({ ...file, buffer: Buffer.from('other') })
    await expect(queue.getByText('aren’t the same files')).toBeVisible()
    await page.locator('input[type=file]:not([webkitdirectory])').first().setInputFiles(file)
    await expect(queue.getByText('Uploaded, now unpacking')).toBeVisible({ timeout: 30_000 })
    const packs = (await (await api.get('/api/packs')).json()) as { id: string; name: string }[]
    const mine = packs.filter((p) => p.name.includes(stem))
    expect(mine).toHaveLength(1)
    await api.delete(`/api/packs/${mine[0].id}`)
  })
})
