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
    const name = `e2e parts ${unique()}`
    const opened = await api.post('/api/uploads', { data: { name, files: [{ path: 'notes.txt', size: 1 }] } })
    expect(opened.status()).toBe(201)
    const partBytes = ((await opened.json()) as Session).part_bytes
    await api.delete(`/api/uploads/${((await opened.json()) as Session).pack.id}`)

    // Two and a half parts of text whose every line says where it is, so a part in the wrong place shows.
    const size = Math.floor(partBytes * 2.5)
    const text = Buffer.alloc(size)
    for (let at = 0; at < size; at += 16) text.write(`${at.toString(16).padStart(15, '0')}\n`.slice(0, size - at), at)
    const session = (await (
      await api.post('/api/uploads', { data: { name, files: [{ path: 'notes.txt', size }] } })
    ).json()) as Session
    const base = `/api/uploads/${session.pack.id}`
    const part = (n: number) =>
      api.put(`${base}/chunk`, {
        params: { path: 'notes.txt', offset: n * partBytes },
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
    const wrong = await api.put(`${base}/chunk`, { params: { path: 'notes.txt', offset: 5 }, data: Buffer.from('x') })
    expect(wrong.status()).toBe(400)

    expect((await api.post(`${base}/complete`)).status()).toBe(202)
    await waitReady(api, [session.pack.id])
    const [asset] = await assetsOf(api, session.pack.id)
    const back = await (await api.get(`/api/assets/${asset.id}/file`)).body()
    expect(back.length).toBe(size)
    expect(createHash('sha256').update(back).digest('hex')).toBe(createHash('sha256').update(text).digest('hex'))
  })
})
