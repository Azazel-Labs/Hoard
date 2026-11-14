// Resumable bulk uploads (see crates/hoardd/src/uploads.rs for the protocol).
//
// - Every file is registered up front; big files then go up in parts, small ones in batches.
// - Several requests run at once, parts of one big file included, so a single zip doesn't crawl along one connection
//   waiting out a round trip per piece. Each retries with backoff, and the server lists the parts it has, so flaky
//   wifi costs at most one part.
// - The server recognizes a re-sent file list, so if the tab is closed, dropping the same folder again continues
//   where it stopped, and packs that already finished aren't imported twice.
import { ApiError, authHeaders, signInNeeded, type Pack, type UploadFile } from './api'
import { t } from './i18n'

const SMALL = 4 * 1024 * 1024
const BATCH_BYTES = 32 * 1024 * 1024
const BATCH_FILES = 400
const PARALLEL = 6
const MAX_ATTEMPTS = 8

export interface Progress {
  sent: number
  total: number
  filesDone: number
  filesTotal: number
  resumedBytes: number
}

interface FileStatus { path: string; size: number; received: number; parts?: number[] }
interface Session { pack: Pack; files: FileStatus[]; total_bytes: number; received_bytes: number; part_bytes: number }

function headers(extra: Record<string, string> = {}): Record<string, string> {
  return { ...authHeaders(), ...extra }
}

async function json<T>(res: Response): Promise<T> {
  if (!res.ok) {
    let msg = `${res.status} ${res.statusText}`
    try {
      msg = (await res.json()).error ?? msg
    } catch {
      // keep status line
    }
    if (res.status === 401) signInNeeded()
    throw new ApiError(res.status, msg)
  }
  return res.json() as Promise<T>
}

/** Thrown when the server already has a pack with exactly these files. */
export class AlreadyImported extends Error {
  pack: Pack
  constructor(pack: Pack, message: string) {
    super(message)
    this.pack = pack
  }
}

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms))

/** XHR so we get upload progress events (fetch still can't report them everywhere). */
function send(
  method: string,
  url: string,
  body: Blob | FormData,
  onProgress: (loaded: number) => void,
  signal: AbortSignal,
): Promise<{ status: number; body: any }> {
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.open(method, url)
    for (const [k, v] of Object.entries(headers())) xhr.setRequestHeader(k, v)
    xhr.upload.onprogress = (e) => onProgress(e.loaded)
    xhr.onload = () => {
      let parsed: any = null
      try {
        parsed = JSON.parse(xhr.responseText)
      } catch {
        // non-JSON
      }
      resolve({ status: xhr.status, body: parsed })
    }
    xhr.onerror = () => reject(new Error(t('upload-network-error')))
    xhr.onabort = () => reject(new DOMException('aborted', 'AbortError'))
    signal.addEventListener('abort', () => xhr.abort(), { once: true })
    xhr.send(body)
  })
}

async function withRetry<T>(what: string, signal: AbortSignal, fn: () => Promise<T>): Promise<T> {
  let lastError: unknown
  for (let attempt = 0; attempt < MAX_ATTEMPTS; attempt++) {
    if (signal.aborted) throw new DOMException('aborted', 'AbortError')
    try {
      return await fn()
    } catch (e) {
      if ((e as Error).name === 'AbortError') throw e
      // Client errors other than conflicts won't fix themselves.
      if (e instanceof ApiError && e.status >= 400 && e.status < 500 && e.status !== 409 && e.status !== 408 && e.status !== 429) throw e
      lastError = e
      await sleep(Math.min(30_000, 500 * 2 ** attempt))
    }
  }
  throw new Error(t('upload-gave-up', { what, error: (lastError as Error)?.message ?? t('upload-failed-plain'), attempts: MAX_ATTEMPTS }))
}

export async function uploadPackResumable(
  files: UploadFile[],
  name: string | undefined,
  onProgress: (p: Progress) => void,
  signal: AbortSignal,
  options: { force?: boolean; onSession?: (id: string) => void } = {},
): Promise<Pack> {
  const force = options.force ?? false
  const onSession = options.onSession ?? (() => undefined)
  // The server fingerprints the file list: the same list resumes its open session (from any browser), and a list
  // that's already imported is refused unless forced.
  const res = await fetch('/api/uploads', {
    method: 'POST',
    headers: headers({ 'Content-Type': 'application/json' }),
    body: JSON.stringify({ name, force, files: files.map((f) => ({ path: f.path, size: f.file.size })) }),
    signal,
  })
  if (res.status === 409) {
    const body = await res.json().catch(() => ({}))
    if (body.pack) throw new AlreadyImported(body.pack, body.error ?? t('upload-already-imported'))
  }
  const session = await json<Session & { resumed: boolean }>(res)
  onSession(session.pack.id)
  const id = session.pack.id
  const base = `/api/uploads/${id}`

  // Match local files to the server's view (paths are normalized server-side, same rules).
  const byPath = new Map(files.map((f) => [f.path.replace(/\\/g, '/').replace(/^\/+/, ''), f.file]))
  const work = session.files
    .map((s) => ({ ...s, file: byPath.get(s.path) }))
    .filter((w): w is FileStatus & { file: File } => !!w.file)

  const total = session.total_bytes
  const acked = new Map<string, number>(work.map((w) => [w.path, w.received]))
  // Bytes on their way, by request (a batch's files, or `path#part`).
  const inflight = new Map<string, number>()
  const resumedBytes = session.received_bytes
  const report = () => {
    let sent = 0
    let filesDone = 0
    for (const w of work) {
      const a = acked.get(w.path) ?? 0
      sent += a
      if (a >= w.size) filesDone++
    }
    for (const n of inflight.values()) sent += n
    onProgress({ sent: Math.min(total, sent), total, filesDone, filesTotal: work.length, resumedBytes })
  }
  report()

  // Big files' parts first, in order, so a 4 GB file doesn't start last; then batches of small whole files.
  type Task = () => Promise<void>
  const tasks: Task[] = []
  const pendingSmall = work.filter((w) => w.received < w.size && w.size <= SMALL && w.received === 0)
  const big = work.filter((w) => w.received < w.size && !pendingSmall.includes(w))
  const partBytes = session.part_bytes
  for (const w of big) {
    const done = new Set(w.parts ?? [])
    for (let part = 0; part * partBytes < w.size; part++) {
      if (done.has(part)) continue
      const start = part * partBytes
      const end = Math.min(w.size, start + partBytes)
      const key = `${w.path}#${part}`
      tasks.push(() =>
        withRetry(w.path, signal, async () => {
          const url = `${base}/chunk?path=${encodeURIComponent(w.path)}&offset=${start}`
          const res = await send('PUT', url, w.file.slice(start, end), (loaded) => {
            inflight.set(key, loaded)
            report()
          }, signal).finally(() => inflight.delete(key))
          if (res.status === 401) signInNeeded()
          if (res.status >= 300) throw new ApiError(res.status, res.body?.error ?? t('upload-part-failed', { status: res.status }))
          // Parts finish in any order: keep the largest count the server has given.
          acked.set(w.path, Math.max(acked.get(w.path) ?? 0, res.body.received as number))
          report()
        }),
      )
    }
  }
  // Zero-byte files never need bytes sent but must be "received": send them in batches too.
  for (let i = 0; i < pendingSmall.length; ) {
    const group: typeof pendingSmall = []
    let bytes = 0
    while (i < pendingSmall.length && group.length < BATCH_FILES && (group.length === 0 || bytes + pendingSmall[i].size <= BATCH_BYTES)) {
      bytes += pendingSmall[i].size
      group.push(pendingSmall[i++])
    }
    tasks.push(() =>
      withRetry(t('upload-batch', { count: group.length }), signal, async () => {
        const form = new FormData()
        for (const w of group) {
          form.append('relpath', w.path)
          form.append('file', w.file, w.path)
        }
        const key = `batch:${group[0].path}`
        const res = await send('POST', `${base}/batch`, form, (loaded) => {
          inflight.set(key, Math.min(bytes, loaded))
          report()
        }, signal).finally(() => inflight.delete(key))
        if (res.status === 401) signInNeeded()
        if (res.status >= 300) throw new ApiError(res.status, res.body?.error ?? t('upload-batch-failed', { status: res.status }))
        for (const w of group) acked.set(w.path, w.size)
        report()
      }),
    )
  }
  // Run with a small pool.
  let next = 0
  const workers = Array.from({ length: Math.min(PARALLEL, tasks.length) }, async () => {
    while (next < tasks.length) {
      const task = tasks[next++]
      await task()
    }
  })
  await Promise.all(workers)

  const pack = await withRetry(t('upload-finish'), signal, async () =>
    json<Pack>(await fetch(`${base}/complete`, { method: 'POST', headers: headers(), signal })),
  )
  return pack
}

/** Abandon a session (server deletes the staged bytes). */
export async function abandonUpload(id: string): Promise<void> {
  await fetch(`/api/uploads/${id}`, { method: 'DELETE', headers: headers() }).catch(() => undefined)
}

const ARCHIVE = /\.(zip|tar|tgz|tar\.gz)$/i

/**
 * Bulk import: split a folder's contents into one group per subfolder, plus one per archive at the top level.
 * Loose non-archive files at the top level become one more group named after the folder.
 */
export function splitPerSubfolder(rootName: string, files: UploadFile[]): { name: string; files: UploadFile[] }[] {
  const groups = new Map<string, UploadFile[]>()
  const loose: UploadFile[] = []
  for (const f of files) {
    // Paths look like "Root/Child/…"; strip the root.
    const rel = f.path.split('/').slice(1)
    if (rel.length >= 2) {
      const key = rel[0]
      if (!groups.has(key)) groups.set(key, [])
      groups.get(key)!.push({ file: f.file, path: rel.join('/') })
    } else if (rel.length === 1 && ARCHIVE.test(rel[0])) {
      groups.set(rel[0], [{ file: f.file, path: rel[0] }])
    } else if (rel.length === 1) {
      loose.push({ file: f.file, path: `${rootName}/${rel[0]}` })
    }
  }
  const out = [...groups.entries()].map(([name, fs]) => ({ name, files: fs }))
  if (loose.length) out.push({ name: rootName, files: loose })
  return out.sort((a, b) => a.name.localeCompare(b.name))
}

/** Does this dropped group look like a folder of packs (2+ subfolders/archives at its top level)? */
export function looksLikeFolderOfPacks(files: UploadFile[]): number {
  const tops = new Set<string>()
  for (const f of files) {
    const rel = f.path.split('/').slice(1)
    if (rel.length >= 2) tops.add(rel[0])
    else if (rel.length === 1 && ARCHIVE.test(rel[0])) tops.add(rel[0])
  }
  return tops.size >= 2 ? tops.size : 0
}
