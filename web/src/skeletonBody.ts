// The character a skeleton's animations play on everywhere: their thumbnails, the viewer, a zip with the character.
// Hoard picks one for each skeleton; an editor can pick another from the viewer's "Plays on", and every animation
// drawn on the old one is drawn again.
import { computed } from 'vue'
import { api } from './api'
import { may } from './auth'
import { t } from './i18n'
import type { StageInfo } from './modelStage'
import { toast } from './ui'

/** Whether this account may choose the character a skeleton's animations play on. */
export const mayChoose = computed(() => may('edit'))

/** The name of the character the viewer is playing the animation on. */
export function wearerName(info: StageInfo): string {
  return info.wearers.find((w) => w.id === info.on)?.name ?? ''
}

/** Make the character the viewer is playing on the one its skeleton's animations play on everywhere. */
export async function playEverywhere(info: StageInfo) {
  const name = wearerName(info)
  try {
    await api.chooseSkeletonBody(info.on)
    info.playsOn = info.on
    toast(t('model-plays-everywhere-done', { name }))
  } catch (e) {
    toast(t('model-plays-everywhere-failed', { error: (e as Error).message }), { kind: 'error' })
  }
}
