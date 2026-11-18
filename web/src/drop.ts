// Files dropped on the page, gathered with their paths inside the folders they came in.

/** One file to upload with its path inside the pack. */
export interface UploadFile { file: File; path: string }

/** Collect files from a drop, one group per top-level item, so dropping three zips makes three packs. */
export async function filesFromDrop(dt: DataTransfer): Promise<{ name: string; files: UploadFile[] }[]> {
  // Entries must be grabbed synchronously, before any await, or the browser empties the DataTransfer.
  const entries = [...dt.items]
    .filter((i) => i.kind === 'file')
    .map((i) => i.webkitGetAsEntry?.())
    .filter((e): e is FileSystemEntry => !!e)
  if (entries.length === 0) {
    return [...dt.files].map((f) => ({ name: f.name, files: [{ file: f, path: f.name }] }))
  }
  const groups: { name: string; files: UploadFile[] }[] = []
  for (const entry of entries) {
    const files: UploadFile[] = []
    await walk(entry, '', files)
    if (files.length) groups.push({ name: entry.name, files })
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
