import { ref } from 'vue'
import type { Asset } from './api'
import { t } from './i18n'
import { copyText, showOk } from './ui'

/** The tags last copied with `c`, which `v` pastes; the clipboard is only read when nothing's been copied here. */
export const copiedTags = ref<string[]>([])

/** An asset's own tags: not the ones every file of its kind or format has. */
function ownTags(a: Asset): string[] {
  return a.tags.map((x) => x.tag).filter((tag) => !tag.startsWith('kind:') && !tag.startsWith('format:'))
}

/** Copy an asset's tags (the `c` key), for pasting here or anywhere else as a list. */
export async function copyTags(a: Asset) {
  const tags = ownTags(a)
  if (!tags.length) {
    showOk(t('app-tags-copy-none', { name: a.name }))
    return
  }
  copiedTags.value = tags
  await copyText(tags.join(', '))
  showOk(t('app-tags-copied', { count: tags.length }))
}

/** Tags from text copied elsewhere: a short list, not a sentence or a link. */
async function clipboardTags(): Promise<string[]> {
  try {
    const text = await navigator.clipboard.readText()
    const tags = text.split(/[,;\n]/).map((x) => x.trim()).filter(Boolean)
    const ok = tags.length <= 40 && tags.every((tag) => tag.length <= 48 && !tag.includes('://'))
    return ok ? [...new Set(tags)] : []
  } catch {
    return []
  }
}

/** What `v` pastes: the tags copied here, or failing that, a list of them copied elsewhere. */
export async function tagsToPaste(): Promise<string[]> {
  return copiedTags.value.length ? copiedTags.value : await clipboardTags()
}
