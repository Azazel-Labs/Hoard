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

type HandleItem = DataTransferItem & { getAsFileSystemHandle?: () => Promise<FileSystemHandle | null> }

/** Collect files from a drop, one group per top-level item, so dropping three zips makes three packs. */
export async function filesFromDrop(dt: DataTransfer): Promise<Group[]> {
  // Entries and handles must be asked for synchronously, before any await, or the browser empties the DataTransfer.
  const items = [...dt.items].filter((i) => i.kind === 'file')
  const entries = items.map((i) => i.webkitGetAsEntry?.() ?? null)
  const handles = items.map((i) => (i as HandleItem).getAsFileSystemHandle?.().catch(() => null) ?? Promise.resolve(null))
  if (!entries.some(Boolean)) {
    return [...dt.files].map((f) => ({ name: f.name, files: [{ file: f, path: f.name }] }))
  }
  const groups: Group[] = []
  for (const [k, entry] of entries.entries()) {
    if (!entry) continue
    const files: UploadFile[] = []
    await walk(entry, '', files)
    const handle = (await handles[k]) ?? undefined
    if (files.length) groups.push({ name: entry.name, files, handle })
  }
  return groups
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

/** Files picked with a file input: each is a pack of its own, or one folder when the input picks folders. */
export function groupsFromPicked(picked: File[]): Group[] {
  const files = picked.map((f) => ({ file: f, path: f.webkitRelativePath || f.name }))
  if (files.some((f) => f.path.includes('/'))) return [{ name: files[0].path.split('/')[0], files }]
  return files.map((f) => ({ name: f.path, files: [f] }))
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
