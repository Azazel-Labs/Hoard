import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'

import { type Pack, assetsOf, waitReady } from './support/api'
import { expect, test, unique } from './support/test'

// A watched folder's files are indexed where they sit. One edited there is read again at the next scan: its contents,
// its size and its picture are the new ones, and the asset (with its tags) is the same.
test('a file edited in a watched folder is read again at the next scan', async ({ api }) => {
  const root = mkdtempSync(join(tmpdir(), 'hoard-e2e-watched-'))
  const folder = `E2E Watched ${unique()}`
  mkdirSync(join(root, folder))
  const notes = join(root, folder, 'notes.txt')
  writeFileSync(notes, 'First draft.\n')
  const created = await api.post('/api/admin/locations', {
    data: { name: folder, mode: 'watched', path: root, kinds: [], catch_all: false },
  })
  expect(created.ok(), await created.text()).toBe(true)
  const loc = (await created.json()) as { id: number }
  try {
    let pack: Pack | undefined
    await expect
      .poll(
        async () => {
          pack = ((await (await api.get('/api/packs')).json()) as Pack[]).find((p) => p.name === folder)
          return pack?.id
        },
        { timeout: 60_000 },
      )
      .toBeTruthy()
    await waitReady(api, [pack!.id])
    const [asset] = await assetsOf(api, pack!.id)
    expect(asset.path).toBe('notes.txt')
    const detail = async () =>
      (await (await api.get(`/api/assets/${asset.id}`)).json()) as { size: number; has_thumb?: boolean }
    expect(await detail()).toMatchObject({ size: 13, has_thumb: true })

    const edited = 'Second draft, longer:\n  with an indented line\n  and another.\n'
    writeFileSync(notes, edited)
    expect((await api.post(`/api/admin/locations/${loc.id}/scan`)).ok()).toBe(true)
    await expect.poll(async () => (await detail()).size, { timeout: 60_000 }).toBe(Buffer.byteLength(edited))
    await expect.poll(async () => (await detail()).has_thumb, { timeout: 60_000 }).toBe(true)
    expect(await (await api.get(`/api/assets/${asset.id}/file`)).text()).toBe(edited)
    expect((await assetsOf(api, pack!.id)).map((a) => a.id)).toEqual([asset.id])
  } finally {
    await api.delete(`/api/admin/locations/${loc.id}`)
    rmSync(root, { recursive: true, force: true })
  }
})
