<script setup lang="ts">
// The search field's microphone: a noise made into it finds the sounds shaped like it, most alike first. The noise
// replaces the words searched for, and typing drops it again (see App).
import { shallowRef } from 'vue'
import { api, ApiError } from '../api'
import { filters } from '../filters'
import { t } from '../i18n'
import { canRecord, record, type Recording } from '../record'
import { showError } from '../ui'

/** The noise being made into the microphone, while it's recorded. */
const listening = shallowRef<Recording | null>(null)
const canListen = canRecord()

/** Records a noise (or stops recording it), then searches by it. */
async function listen() {
  if (listening.value) {
    listening.value.stop()
    return
  }
  try {
    const recording = await record()
    listening.value = recording
    const wav = await recording.done.finally(() => (listening.value = null))
    const heard = await api.imitation(wav)
    Object.assign(filters, { imitation: heard.imitation, q: '', similar: null, color: '', sort: '' })
  } catch (e) {
    showError(e instanceof ApiError ? e.message : t('app-imitation-mic-failed', { error: (e as Error).message }))
  }
}
</script>

<template>
  <button
    v-if="canListen"
    type="button"
    class="mic"
    :class="{ on: listening }"
    :aria-pressed="!!listening"
    :title="listening ? t('app-imitation-stop') : t('app-imitation-record')"
    @click="listen"
  >
    <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <rect v-if="listening" x="7" y="7" width="10" height="10" rx="1.5" />
      <template v-else>
        <rect x="9" y="3" width="6" height="11" rx="3" />
        <path d="M5 11a7 7 0 0 0 14 0M12 18v3" />
      </template>
    </svg>
    <span class="visually-hidden">{{ listening ? t('app-imitation-stop') : t('app-imitation-record') }}</span>
  </button>
</template>

<style scoped>
.mic {
  position: absolute; right: 0.3em; top: 50%; translate: 0 -50%;
  border: 0; background: none; padding: 0.3rem; line-height: 0; cursor: pointer; border-radius: var(--r-chip); color: var(--faint);
}
.mic:hover { color: var(--text); background: var(--raised); }
.mic.on { color: var(--danger); animation: mic-pulse 1.2s ease-in-out infinite; }
@keyframes mic-pulse { 50% { opacity: 0.45; } }
@media (prefers-reduced-motion: reduce) { .mic.on { animation: none; } }
</style>
