// Resumable bulk uploads (see crates/hoardd/src/uploads.rs for the protocol).
//
// - Every file is registered up front; big files then go up in parts, small ones in batches.
// - Several requests run at once, parts of one big file included, so a single zip doesn't crawl along one connection
//   waiting out a round trip per piece. Each retries with backoff, and the server lists the parts it has, so flaky
//   wifi costs at most one part.
// - The server recognizes a re-sent file list, so if the tab is closed, dropping the same folder again continues
//   where it stopped, and packs that already finished aren't imported twice.
import { ApiError, authHeaders, signInNeeded, type Pack } from './api'
import { archiveOf, type UploadFile } from './drop'
import { t } from './i18n'

const SMALL = 4 * 1024 * 1024
const BATCH_BYTES = 32 * 1024 * 1024
const BATCH_FILES = 400
const PARALLEL = 6
const MAX_ATTEMPTS = 8
/**
 * A request that sends nothing and hears nothing for this long is given up on and sent again. A connection can go
 * quiet without failing (a proxy or the network dropping it, say), and nothing else would notice: the upload just
 * stopped until it was canceled.
 */
const STALL_MS = 120_000

/** A request that failed, being sent again. */
export interface Retrying {
  what: string
  error: string
  /** Which attempt is next, of `attempts`. */
  attempt: number
  attempts: number
}

export interface Progress {
  sent: number
  total: number
  filesDone: number
  filesTotal: number
  resumedBytes: number
  /** The latest of the requests being sent again, while any are. */
  retrying?: Retrying
}

interface FileStatus { path: string; size: number; received: number; parts?: number[] }
interface Session {
  pack: Pack
  files: FileStatus[]
  total_bytes: number
  received_bytes: number
  part_bytes: number
  /** This registration's hold on the session, to cancel it with. */
  claim?: string
  /** The pack the library has already that the upload goes into (new files added, changed ones replaced). */
  merge_into?: Pack
}

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

const stalled = () => new Error(t('upload-no-reply', { seconds: STALL_MS / 1000 }))

/**
 * XHR so we get upload progress events (fetch still can't report them everywhere). Given up on once it has sent and
 * heard nothing for [STALL_MS]; `retry` says why the last attempt failed, for the server's log.
 */
function send(
  method: string,
  url: string,
  body: Blob | FormData,
  onProgress: (loaded: number) => void,
  signal: AbortSignal,
  retry?: string,
): Promise<{ status: number; body: any }> {
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.open(method, url)
    for (const [k, v] of Object.entries(headers())) xhr.setRequestHeader(k, v)
    if (retry) xhr.setRequestHeader('X-Hoard-Retry', encodeURIComponent(retry))
    let heard = Date.now()
    let quiet = false
    const watch = setInterval(() => {
      if (Date.now() - heard < STALL_MS) return
      quiet = true
      xhr.abort()
    }, 5_000)
    const stop = () => xhr.abort()
    const settle = () => {
      clearInterval(watch)
      signal.removeEventListener('abort', stop)
    }
    xhr.upload.onprogress = (e) => {
      heard = Date.now()
      onProgress(e.loaded)
    }
    // All sent: the wait for the answer starts now.
    xhr.upload.onload = () => (heard = Date.now())
    xhr.onprogress = () => (heard = Date.now())
    xhr.onload = () => {
      settle()
      let parsed: any = null
      try {
        parsed = JSON.parse(xhr.responseText)
      } catch {
        // non-JSON
      }
      resolve({ status: xhr.status, body: parsed })
    }
    xhr.onerror = () => {
      settle()
      reject(new Error(t('upload-network-error')))
    }
    xhr.onabort = () => {
      settle()
      reject(quiet ? stalled() : new DOMException('aborted', 'AbortError'))
    }
    signal.addEventListener('abort', stop, { once: true })
    xhr.send(body)
  })
}

/** `fetch`, given up on when no answer comes within [STALL_MS]. */
async function fetchWithin(url: string, init: RequestInit, signal: AbortSignal): Promise<Response> {
  const controller = new AbortController()
  const stop = () => controller.abort()
  signal.addEventListener('abort', stop, { once: true })
  let quiet = false
  const watch = setTimeout(() => {
    quiet = true
    controller.abort()
  }, STALL_MS)
  try {
    return await fetch(url, { ...init, signal: controller.signal })
  } catch (e) {
    throw quiet && !signal.aborted ? stalled() : e
  } finally {
    clearTimeout(watch)
    signal.removeEventListener('abort', stop)
  }
}

/**
 * Run `fn` until it works, waiting longer after each failure. `fn` is told why the last attempt failed, and
 * `onRetry` of each retry, then `null` once it worked: a failure that fixes itself is otherwise never seen.
 */
async function withRetry<T>(
  what: string,
  signal: AbortSignal,
  fn: (retry?: string) => Promise<T>,
  onRetry: (r: Retrying | null) => void = () => undefined,
): Promise<T> {
  let lastError: unknown
  let retry: string | undefined
  for (let attempt = 0; attempt < MAX_ATTEMPTS; attempt++) {
    if (signal.aborted) throw new DOMException('aborted', 'AbortError')
    try {
      const out = await fn(retry)
      if (retry) onRetry(null)
      return out
    } catch (e) {
      if ((e as Error).name === 'AbortError') throw e
      // Client errors other than conflicts won't fix themselves.
      if (e instanceof ApiError && e.status >= 400 && e.status < 500 && e.status !== 409 && e.status !== 408 && e.status !== 429) throw e
      lastError = e
      if (attempt + 1 < MAX_ATTEMPTS) {
        const error = (e as Error)?.message ?? t('upload-failed-plain')
        retry = `${attempt + 2}/${MAX_ATTEMPTS}: ${error}`
        console.warn(`upload: ${what}: ${error}; trying again (${attempt + 2} of ${MAX_ATTEMPTS})`)
        onRetry({ what, error, attempt: attempt + 2, attempts: MAX_ATTEMPTS })
        await sleep(Math.min(30_000, 500 * 2 ** attempt))
      }
    }
  }
  throw new Error(t('upload-gave-up', { what, error: (lastError as Error)?.message ?? t('upload-failed-plain'), attempts: MAX_ATTEMPTS }))
}

export async function uploadPackResumable(
  files: UploadFile[],
  name: string | undefined,
  onProgress: (p: Progress) => void,
  signal: AbortSignal,
  options: {
    force?: boolean
    /** Make a pack of its own even when the library has this pack already. */
    copy?: boolean
    onSession?: (id: string, claim?: string, into?: Pack) => void
  } = {},
): Promise<Pack> {
  const force = options.force ?? false
  const copy = options.copy ?? false
  const onSession = options.onSession ?? (() => undefined)
  // The server fingerprints the file list: the same list resumes its open session (from any browser), and a list
  // that's already imported is refused unless forced. A pack the library has already (by name, or these very files
  // when forced) is added to rather than copied, unless asked for a copy.
  const res = await fetchWithin(
    '/api/uploads',
    {
      method: 'POST',
      headers: headers({ 'Content-Type': 'application/json' }),
      body: JSON.stringify({ name, force, copy, files: files.map((f) => ({ path: f.path, size: f.file.size })) }),
    },
    signal,
  )
  if (res.status === 409) {
    const body = await res.json().catch(() => ({}))
    if (body.pack) throw new AlreadyImported(body.pack, body.error ?? t('upload-already-imported'))
  }
  const session = await json<Session & { resumed: boolean }>(res)
  onSession(session.pack.id, session.claim, session.merge_into)
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
  // Requests being sent again, by task, the latest last.
  const retries = new Map<string, Retrying>()
  const retried = (key: string) => (r: Retrying | null) => {
    retries.delete(key)
    if (r) retries.set(key, r)
    report()
  }
  const report = () => {
    let sent = 0
    let filesDone = 0
    for (const w of work) {
      const a = acked.get(w.path) ?? 0
      sent += a
      if (a >= w.size) filesDone++
    }
    for (const n of inflight.values()) sent += n
    const retrying = [...retries.values()].pop()
    onProgress({ sent: Math.min(total, sent), total, filesDone, filesTotal: work.length, resumedBytes, retrying })
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
        withRetry(w.path, signal, async (retry) => {
          const url = `${base}/chunk?path=${encodeURIComponent(w.path)}&offset=${start}`
          const res = await send('PUT', url, w.file.slice(start, end), (loaded) => {
            inflight.set(key, loaded)
            report()
          }, signal, retry).finally(() => inflight.delete(key))
          if (res.status === 401) signInNeeded()
          if (res.status >= 300) throw new ApiError(res.status, res.body?.error ?? t('upload-part-failed', { status: res.status }))
          // Parts finish in any order: keep the largest count the server has given.
          acked.set(w.path, Math.max(acked.get(w.path) ?? 0, res.body.received as number))
          report()
        }, retried(key)),
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
    const key = `batch:${group[0].path}`
    tasks.push(() =>
      withRetry(t('upload-batch', { count: group.length }), signal, async (retry) => {
        const form = new FormData()
        for (const w of group) {
          form.append('relpath', w.path)
          form.append('file', w.file, w.path)
        }
        const res = await send('POST', `${base}/batch`, form, (loaded) => {
          inflight.set(key, Math.min(bytes, loaded))
          report()
        }, signal, retry).finally(() => inflight.delete(key))
        if (res.status === 401) signInNeeded()
        if (res.status >= 300) throw new ApiError(res.status, res.body?.error ?? t('upload-batch-failed', { status: res.status }))
        for (const w of group) acked.set(w.path, w.size)
        report()
      }, retried(key)),
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

  const pack = await withRetry(t('upload-finish'), signal, async (retry) => {
    const res = await fetchWithin(`${base}/complete`, { method: 'POST', headers: headers(retry ? { 'X-Hoard-Retry': encodeURIComponent(retry) } : {}) }, signal)
    // An attempt that went unanswered may have finished anyway: then the upload is done, and there's nothing to finish.
    if (retry && res.status >= 400 && res.status < 500 && res.status !== 409) {
      const done = await finishedAlready(id, session.merge_into, signal)
      if (done) return done
    }
    return json<Pack>(res)
  }, retried('complete'))
  return pack
}

/** The pack an upload went into, if it's finished (no longer being uploaded). */
async function finishedAlready(id: string, into: Pack | undefined, signal: AbortSignal): Promise<Pack | undefined> {
  for (const url of [`/api/uploads/${id}`, into && `/api/packs/${into.id}`]) {
    if (!url) continue
    const res = await fetchWithin(url, { headers: headers() }, signal).catch(() => undefined)
    if (!res?.ok) continue
    const body = await res.json().catch(() => undefined)
    const pack: Pack | undefined = body?.pack ?? body
    if (pack?.status && pack.status !== 'uploading') return pack
  }
  return undefined
}

/**
 * Abandon a session (server deletes the staged bytes). With the claim registering it gave, a session the same files
 * were registered for again since (dropped again to retry) is left to that later upload.
 */
export async function abandonUpload(id: string, claim?: string): Promise<void> {
  const query = claim ? `?claim=${encodeURIComponent(claim)}` : ''
  await fetch(`/api/uploads/${id}${query}`, { method: 'DELETE', headers: headers() }).catch(() => undefined)
}

/**
 * Bulk import: split a folder's contents into one group per subfolder, plus one per archive at the top level (its
 * parts together, when it's in parts).
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
    } else if (rel.length === 1 && archiveOf(rel[0])) {
      const key = archiveOf(rel[0])!
      if (!groups.has(key)) groups.set(key, [])
      groups.get(key)!.push({ file: f.file, path: rel[0] })
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
    else if (rel.length === 1 && archiveOf(rel[0])) tops.add(archiveOf(rel[0])!.toLowerCase())
  }
  return tops.size >= 2 ? tops.size : 0
}
