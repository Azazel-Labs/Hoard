// Talking to the server under test straight over HTTP: signing in, seeding packs, tagging assets.
import { request, type APIRequestContext } from '@playwright/test'

export const USERNAME = 'e2e'
export const PASSWORD = 'e2e-password-123'
/** Every mutating call needs this header; the server refuses anything else as not from the portal. */
export const HOARD_HEADERS = { 'X-Hoard': '1' }

export interface Pack {
  id: string
  name: string
  status: 'uploading' | 'ingesting' | 'analyzing' | 'ready' | 'failed'
  asset_count: number
  error: string | null
}

export interface Asset {
  id: string
  name: string
  kind: string
  pack_id: string
  path: string
  tags: { tag: string; source: string; confidence: number }[]
}

export interface UploadFile {
  relpath: string
  data: Buffer
  mime?: string
}

function mimeFor(path: string): string {
  const ext = path.split('.').pop()?.toLowerCase()
  return (
    { png: 'image/png', json: 'application/json', gltf: 'model/gltf+json', txt: 'text/plain', zip: 'application/zip' }[
      ext ?? ''
    ] ?? 'application/octet-stream'
  )
}

/** A request context signed in as the e2e account. */
export async function signedIn(baseURL: string): Promise<APIRequestContext> {
  const ctx = await request.newContext({ baseURL, extraHTTPHeaders: HOARD_HEADERS })
  const res = await ctx.post('/api/auth/login', { data: { username: USERNAME, password: PASSWORD } })
  if (!res.ok()) throw new Error(`sign-in failed: ${res.status()} ${await res.text()}`)
  return ctx
}

/** Upload a pack; returns it as the server first records it (not yet ready). */
export async function uploadPack(api: APIRequestContext, name: string, files: UploadFile[]): Promise<Pack> {
  const form = new FormData()
  form.append('name', name)
  for (const f of files) {
    form.append('relpath', f.relpath)
    form.append('file', new Blob([new Uint8Array(f.data)], { type: f.mime ?? mimeFor(f.relpath) }), f.relpath.split('/').pop())
  }
  const res = await api.post('/api/packs', { multipart: form })
  if (!res.ok()) throw new Error(`upload of ${name} failed: ${res.status()} ${await res.text()}`)
  return (await res.json()) as Pack
}

/** Wait until every pack named is ready, failing on one that failed or a timeout. */
export async function waitReady(api: APIRequestContext, ids: string[], timeoutMs = 120_000): Promise<Pack[]> {
  const deadline = Date.now() + timeoutMs
  for (;;) {
    const packs = (await (await api.get('/api/packs')).json()) as Pack[]
    const mine = ids.map((id) => packs.find((p) => p.id === id))
    const failed = mine.find((p) => p?.status === 'failed')
    if (failed) throw new Error(`pack ${failed.name} failed: ${failed.error}`)
    if (mine.every((p) => p?.status === 'ready')) return mine as Pack[]
    if (Date.now() > deadline) {
      throw new Error(`packs not ready in time: ${mine.map((p) => `${p?.name}=${p?.status}`).join(', ')}`)
    }
    await new Promise((r) => setTimeout(r, 250))
  }
}

/** Waits (up to a point) for background work after the uploads to settle, so tests don't race it for the CPU. */
export async function waitIdle(api: APIRequestContext, timeoutMs = 90_000): Promise<void> {
  const deadline = Date.now() + timeoutMs
  while (Date.now() < deadline) {
    const jobs = (await (await api.get('/api/admin/jobs')).json()) as { status: string }[]
    if (!jobs.some((j) => j.status === 'queued' || j.status === 'running')) return
    await new Promise((r) => setTimeout(r, 250))
  }
}

export async function assetsOf(api: APIRequestContext, packId: string): Promise<Asset[]> {
  const res = await api.get('/api/assets', { params: { pack: packId, limit: 500, facets: false } })
  if (!res.ok()) throw new Error(`asset search failed: ${res.status()}`)
  return ((await res.json()) as { items: Asset[] }).items
}

export async function addTags(api: APIRequestContext, assetId: string, tags: string[]): Promise<void> {
  const res = await api.patch(`/api/assets/${assetId}`, { data: { add_tags: tags } })
  if (!res.ok()) throw new Error(`tagging ${assetId} failed: ${res.status()} ${await res.text()}`)
}

/** The tag names an asset has now. */
export async function tagsOf(api: APIRequestContext, assetId: string): Promise<string[]> {
  const res = await api.get(`/api/assets/${assetId}`)
  if (!res.ok()) throw new Error(`reading ${assetId} failed: ${res.status()}`)
  return ((await res.json()) as Asset).tags.map((t) => t.tag)
}
