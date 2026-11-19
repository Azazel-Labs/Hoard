// Files dropped on the page, gathered with their paths inside the folders they came in.

/** One file to upload with its path inside the pack. */
export interface UploadFile { file: File; path: string }

/** The files for one pack, and where they came from when the browser can read them again after a reload. */
export interface Group {
  name: string
  files: UploadFile[]
  /** The dropped file or folder itself (Chromium, on https or localhost), kept so a reload can carry on. */
  handle?: FileSystemHandle
  /** Set when this group is one part of a folder of packs that was split up; names that folder. */
  root?: string
}

/** The kinds of archive the server unpacks. */
const ARCHIVE = /\.(zip|7z|rar|tar|tgz|tar\.gz|unitypackage)$/i

/**
 * The archive a file is, or is a part of, named as if it were whole: `Forest.part2.rar` and `Forest.r00` are parts
 * of `Forest.rar`, `Rocks.z01` of `Rocks.zip`, `Big.7z.002` of `Big.7z`. Null for a file that isn't one.
 */
export function archiveOf(name: string): string | null {
  const part = /^(.+)\.part\d+\.rar$/i.exec(name) ?? /^(.+)\.[rs]\d{2}$/i.exec(name)
  if (part) return `${part[1]}.rar`
  const disk = /^(.+)\.z\d{2}$/i.exec(name)
  if (disk) return `${disk[1]}.zip`
  const piece = /^(.+)\.\d{3,}$/.exec(name)
  if (piece) return ARCHIVE.test(piece[1]) ? piece[1] : null
  return ARCHIVE.test(name) ? name : null
}

/** Groups of one file that are parts of the same archive, made one group: the archive is one pack, not one a part. */
export function joinArchiveParts(groups: Group[]): Group[] {
  const out: Group[] = []
  const sets = new Map<string, Group>()
  for (const g of groups) {
    const archive = g.files.length === 1 && !g.files[0].path.includes('/') ? archiveOf(g.files[0].path) : null
    if (!archive || archive === g.files[0].path) {
      out.push(g)
      continue
    }
    const key = archive.toLowerCase()
    const set = sets.get(key)
    if (set) {
      set.files.push(...g.files)
      continue
    }
    // A set can't be read again from one part's handle after a reload.
    const joined = { name: archive, files: [...g.files] }
    sets.set(key, joined)
    out.push(joined)
  }
  // The whole archive's own file (Rocks.zip after Rocks.z01) joins its parts too.
  return out.filter((g) => {
    const set = g.files.length === 1 ? sets.get(g.files[0].path.toLowerCase()) : undefined
    if (set && set !== g) set.files.push(...g.files)
    return !set || set === g
  })
}

type HandleItem = DataTransferItem & { getAsFileSystemHandle?: () => Promise<FileSystemHandle | null> }

/**
 * Collect files from a drop, one group per top-level item, so dropping three zips makes three packs (and the parts
 * of one archive make one).
 */
export async function filesFromDrop(dt: DataTransfer): Promise<Group[]> {
  // Entries and handles must be asked for synchronously, before any await, or the browser empties the DataTransfer.
  const items = [...dt.items].filter((i) => i.kind === 'file')
  const entries = items.map((i) => i.webkitGetAsEntry?.() ?? null)
  const handles = items.map((i) => (i as HandleItem).getAsFileSystemHandle?.().catch(() => null) ?? Promise.resolve(null))
  if (!entries.some(Boolean)) {
    return joinArchiveParts([...dt.files].map((f) => ({ name: f.name, files: [{ file: f, path: f.name }] })))
  }
  const groups: Group[] = []
  for (const [k, entry] of entries.entries()) {
    if (!entry) continue
    const files: UploadFile[] = []
    await walk(entry, '', files)
    const handle = (await handles[k]) ?? undefined
    if (files.length) groups.push({ name: entry.name, files, handle })
  }
  return joinArchiveParts(groups)
}

async function walk(entry: FileSystemEntry, prefix: string, out: UploadFile[]): Promise<void> {
  if (entry.isFile) {
    const file = await new Promise<File>((res, rej) => (entry as FileSystemFileEntry).file(res, rej))
    out.push({ file, path: prefix + entry.name })
  } else if (entry.isDirectory) {
    const reader = (entry as FileSystemDirectoryEntry).createReader()
    for (;;) {
      const batch = await new Promise<FileSystemEntry[]>((res, rej) => reader.readEntries(res, rej))
      if (batch.length === 0) break
      for (const child of batch) await walk(child, `${prefix}${entry.name}/`, out)
    }
  }
}

type DirHandle = FileSystemDirectoryHandle & { values(): AsyncIterable<FileSystemHandle> }

/** Read a dropped file or folder again from its handle, with the same paths the drop gave it. */
export async function filesFromHandle(handle: FileSystemHandle): Promise<UploadFile[]> {
  const out: UploadFile[] = []
  await walkHandle(handle, '', out)
  return out
}

async function walkHandle(handle: FileSystemHandle, prefix: string, out: UploadFile[]): Promise<void> {
  if (handle.kind === 'file') {
    out.push({ file: await (handle as FileSystemFileHandle).getFile(), path: prefix + handle.name })
    return
  }
  for await (const child of (handle as DirHandle).values()) await walkHandle(child, `${prefix}${handle.name}/`, out)
}

/**
 * Files picked with a file input: each is a pack of its own (the parts of an archive one between them), or one
 * folder when the input picks folders.
 */
export function groupsFromPicked(picked: File[]): Group[] {
  const files = picked.map((f) => ({ file: f, path: f.webkitRelativePath || f.name }))
  if (files.some((f) => f.path.includes('/'))) return [{ name: files[0].path.split('/')[0], files }]
  return joinArchiveParts(files.map((f) => ({ name: f.path, files: [f] })))
}

/** A short fingerprint of a list of files (their paths and sizes, in any order): the same files give the same one. */
export function signature(files: UploadFile[]): string {
  const text = files
    .map((f) => `${f.path}\t${f.file.size}`)
    .sort()
    .join('\n')
  // cyrb53: two 32-bit FNV-like lanes mixed together.
  let h1 = 0xdeadbeef
  let h2 = 0x41c6ce57
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i)
    h1 = Math.imul(h1 ^ c, 2654435761)
    h2 = Math.imul(h2 ^ c, 1597334677)
  }
  h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909)
  h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909)
  return `${files.length}:${(h2 >>> 0).toString(36)}${(h1 >>> 0).toString(36)}`
}
