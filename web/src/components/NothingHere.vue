<script setup lang="ts">
// The grid with nothing in it: an empty library's welcome (with the two ways to fill it), or a search that found
// nothing (saying why, as far as it can, with a way back to everything).
import { computed } from 'vue'
import { clearFilters, filters, hasFilters, heardNoise } from '../filters'
import { t } from '../i18n'

/** The library is empty, rather than the search finding nothing in it. */
defineProps<{ welcome: boolean }>()
const emit = defineEmits<{ folder: []; files: [] }>()

/** Said when a search finds nothing. */
const noneText = computed(() => {
  const query = filters.q.trim()
  const narrowed = hasFilters.value && !!(filters.tags.length || filters.kind || filters.pack)
  if (heardNoise.value) return t('app-none-heard')
  if (filters.imitation) return t('app-none-imitation', { word: filters.imitation })
  if (filters.q) return narrowed ? t('app-none-query-filters', { query }) : t('app-none-query', { query })
  return narrowed ? t('app-none-filters') : t('app-none')
})
</script>

<template>
  <div v-if="welcome" class="welcome">
    <svg class="art crates" viewBox="0 0 140 112" width="150" height="120" aria-hidden="true">
      <g class="c c1"><rect x="30" y="14" width="26" height="26" rx="5" fill="var(--k-texture)" /></g>
      <g class="c c2"><rect x="64" y="2" width="26" height="26" rx="5" fill="var(--k-audio)" /></g>
      <g class="c c3"><rect x="50" y="42" width="26" height="26" rx="5" fill="var(--k-model)" /></g>
      <g class="c c4"><rect x="86" y="32" width="26" height="26" rx="5" fill="var(--k-font)" /></g>
      <path d="M16 64h108l-10 40a6 6 0 0 1-5.8 4.5H31.8A6 6 0 0 1 26 104Z" fill="var(--panel)" stroke="var(--line)" stroke-width="2" stroke-linejoin="round" />
      <path d="M14 64h112" stroke="var(--muted)" stroke-width="3" stroke-linecap="round" />
      <path d="M56 86h28" stroke="var(--line)" stroke-width="3" stroke-linecap="round" />
    </svg>
    <h1>{{ t('app-welcome-title') }}</h1>
    <p>
      {{ t('app-welcome-before') }}<span class="mono">{{ t('app-welcome-example-1') }}</span>{{ t('app-welcome-or') }}<span class="mono">{{ t('app-welcome-example-2') }}</span>{{ t('app-welcome-after') }}
    </p>
    <div class="welcome-actions">
      <button type="button" class="btn primary" @click="emit('folder')">{{ t('app-welcome-import-folder') }}</button>
      <button type="button" class="btn" @click="emit('files')">{{ t('app-welcome-choose-files') }}</button>
    </div>
  </div>
  <div v-else class="none">
    <svg class="art" viewBox="0 0 124 92" width="124" height="92" aria-hidden="true">
      <rect x="10" y="20" width="66" height="54" rx="7" fill="none" stroke="var(--line)" stroke-width="2" stroke-dasharray="6 5" />
      <rect x="21" y="32" width="15" height="15" rx="3" fill="var(--k-texture)" opacity="0.3" />
      <rect x="42" y="32" width="15" height="15" rx="3" fill="var(--k-audio)" opacity="0.3" />
      <rect x="21" y="53" width="15" height="10" rx="3" fill="var(--k-model)" opacity="0.3" />
      <g class="glass">
        <circle cx="84" cy="44" r="17" fill="var(--bg)" stroke="var(--muted)" stroke-width="3" />
        <path d="M78 38a8 8 0 0 1 9-3" stroke="var(--line)" stroke-width="2.5" stroke-linecap="round" fill="none" />
        <path d="m96.5 56.5 13 13" stroke="var(--muted)" stroke-width="5.5" stroke-linecap="round" />
      </g>
    </svg>
    <p v-if="filters.similar && !filters.q && !filters.kind && !filters.tags.length && !filters.color">
      {{ t('app-none-similar', { name: filters.similar.name }) }}
    </p>
    <p v-else>{{ noneText }}</p>
    <button v-if="hasFilters" type="button" class="btn" @click="clearFilters">{{ t('app-clear-search-filters') }}</button>
  </div>
</template>

<style scoped>
.none { color: var(--muted); padding: 3rem 0; display: flex; flex-direction: column; align-items: center; text-align: center; }
.art { display: block; margin-bottom: 1rem; overflow: visible; }
.welcome .art { margin-bottom: 1.25rem; }
.glass { animation: hunt 3.2s var(--ease-in-out) infinite; }
@keyframes hunt { 25% { translate: -14px -6px; } 50% { translate: -26px 6px; } 75% { translate: -8px 8px; } }
.crates .c { animation: bob 2.6s var(--ease-in-out) infinite; }
.crates .c2 { animation-delay: -0.6s; }
.crates .c3 { animation-delay: -1.3s; }
.crates .c4 { animation-delay: -1.9s; }
.crates .c1 rect { rotate: -8deg; transform-box: fill-box; transform-origin: center; }
.crates .c4 rect { rotate: 10deg; transform-box: fill-box; transform-origin: center; }
.none p { margin: 0 0 0.9rem; }
.welcome { max-width: 60ch; padding: 3rem 0; }
.welcome h1 { margin: 0 0 0.75rem; font-size: var(--step-2); line-height: 1.2; font-variation-settings: 'CASL' 0.6; font-weight: 700; }
.welcome p { color: var(--muted); margin: 0 0 1.25rem; }
.welcome-actions { display: flex; gap: 0.5rem; }
@keyframes bob { 50% { translate: 0 -6px; } }
</style>
