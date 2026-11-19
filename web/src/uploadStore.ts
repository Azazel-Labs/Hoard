// Imports under way, remembered in IndexedDB so a reload shows them again and carries them on where they stopped.
// Every call swallows failures: without IndexedDB (a private window, a locked-down browser) a reload forgets them.

/** What's kept of an import between page loads. */
export interface Saved {
  key: string
  name: string
  /** signature() of its files, to know them when they're picked or dropped again. */
  sig: string
  /** Whether it came as a folder (picked again with the folder picker) rather than a file. */
  folder: boolean
  /** The folder of packs it was split from, if it was. */
  root?: string
  /** The dropped file or folder, where the browser hands one out, to read the files again without asking. */
  handle?: FileSystemHandle
  force?: boolean
  sessionId?: string
  /** The hold registering it gave on its session, to cancel it with (see `abandonUpload`). */
  claim?: string
  sent: number
  total: number
  filesTotal: number
}

const DB = 'hoard-uploads'
const STORE = 'jobs'
let opened: Promise<IDBDatabase | null> | null = null

function db(): Promise<IDBDatabase | null> {
  opened ??= new Promise((resolve) => {
    try {
      const req = indexedDB.open(DB, 1)
      req.onupgradeneeded = () => req.result.createObjectStore(STORE, { keyPath: 'key' })
      req.onsuccess = () => resolve(req.result)
      req.onerror = () => resolve(null)
      req.onblocked = () => resolve(null)
    } catch {
      resolve(null)
    }
  })
  return opened
}

/** Run one request against the store; undefined when it couldn't. */
async function run<T>(mode: IDBTransactionMode, ask: (s: IDBObjectStore) => IDBRequest<T>): Promise<T | undefined> {
  const d = await db()
  if (!d) return undefined
  return new Promise((resolve) => {
    try {
      const req = ask(d.transaction(STORE, mode).objectStore(STORE))
      req.onsuccess = () => resolve(req.result)
      req.onerror = () => resolve(undefined)
    } catch {
      resolve(undefined)
    }
  })
}

export function newKey(): string {
  return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`
}

export async function loadSaved(): Promise<Saved[]> {
  return (await run('readonly', (s) => s.getAll() as IDBRequest<Saved[]>)) ?? []
}

/** Remember an import; false when nothing could be kept. A handle the browser won't store is left out. */
export async function save(job: Saved): Promise<boolean> {
  if ((await run('readwrite', (s) => s.put(job))) !== undefined) return true
  if (!job.handle) return false
  return (await run('readwrite', (s) => s.put({ ...job, handle: undefined }))) !== undefined
}

export async function forget(key: string): Promise<void> {
  await run('readwrite', (s) => s.delete(key))
}
