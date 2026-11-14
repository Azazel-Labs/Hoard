// The archive is Hoard's recycling bin: archived packs and assets leave search, the counts and the tag list, and
// wait there until someone restores them or deletes them for good from the Archive view.
import { api, type ArchiveItems } from './api'
import { usageWarning } from './projects'
import { confirmDialog, showError, toast } from './ui'
import { t } from './i18n'

/**
 * Move things to the archive now, with an Undo on the toast that brings them back. When projects use any of them it
 * asks first, since their next sync would take them out. False if it failed or was called off.
 */
export async function archiveWithUndo(text: string, items: ArchiveItems, changed: () => void): Promise<boolean> {
  const uses = await usageWarning(items)
  if (uses && !(await confirmDialog({ title: uses.title, body: uses.body, confirm: t('archive-anyway') }))) return false
  try {
    await api.archive(items)
  } catch (e) {
    showError(t('archive-failed', { error: (e as Error).message }))
    return false
  }
  changed()
  toast(text, {
    kind: 'info',
    duration: 7000,
    action: {
      label: t('archive-undo'),
      run: async () => {
        try {
          await api.restore(items)
          changed()
        } catch (e) {
          showError(t('archive-restore-failed', { error: (e as Error).message }))
        }
      },
    },
  })
  return true
}

/** "3 packs and 12 assets", "1 asset". */
export function countText(packs: number, assets: number): string {
  if (packs && assets) return t('archive-count-packs-and-assets', { packs, assets })
  if (packs) return t('archive-count-packs', { packs })
  if (assets) return t('archive-count-assets', { assets })
  return t('archive-count-nothing')
}
