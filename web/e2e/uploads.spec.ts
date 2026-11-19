import { createHash } from 'node:crypto'
import { assetsOf, waitReady, type Pack } from './support/api'
import { expect, test, unique } from './support/test'

interface Session {
  pack: Pack
  part_bytes: number
  files: { path: string; size: number; received: number; parts?: number[] }[]
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
})

test.describe('the import queue', () => {
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
