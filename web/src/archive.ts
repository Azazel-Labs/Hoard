// The archive is Hoard's recycling bin: archived packs and assets leave search, the counts and the tag list, and
// wait there until someone restores them or deletes them for good from the Archive view.
import { api, type ArchiveItems } from './api'
import { usageWarning } from './projects'
import { confirmDialog, showError, toast } from './ui'

/**
 * Move things to the archive now, with an Undo on the toast that brings them back. When projects use any of them it
 * asks first, since their next sync would take them out. False if it failed or was called off.
 */
export async function archiveWithUndo(text: string, items: ArchiveItems, changed: () => void): Promise<boolean> {
  const uses = await usageWarning(items)
  if (uses && !(await confirmDialog({ title: uses.title, body: uses.body, confirm: 'Archive anyway' }))) return false
  try {
    await api.archive(items)
  } catch (e) {
    showError(`Couldn't archive: ${(e as Error).message}`)
    return false
  }
  changed()
  toast(text, {
    kind: 'info',
    duration: 7000,
    action: {
      label: 'Undo',
      run: async () => {
        try {
          await api.restore(items)
          changed()
        } catch (e) {
          showError(`Couldn't restore: ${(e as Error).message}`)
        }
      },
    },
  })
  return true
}

/** "3 packs and 12 assets", "1 asset". */
export function countText(packs: number, assets: number): string {
  const part = (n: number, one: string) => (n ? `${n.toLocaleString()} ${one}${n === 1 ? '' : 's'}` : '')
  return [part(packs, 'pack'), part(assets, 'asset')].filter(Boolean).join(' and ') || 'nothing'
}
