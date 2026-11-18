<script setup lang="ts">
// The Filters column's Music section: a tempo range and a key (with the keys that mix with it). Shown while searching
// sounds.
import { filters } from '../filters'
import { t } from '../i18n'
import { KEYS, keyName } from '../keys'

/** A tempo typed in: a number, or nothing. */
function bpmInput(e: Event): number | '' {
  const v = (e.target as HTMLInputElement).valueAsNumber
  return Number.isFinite(v) && v > 0 ? Math.round(v) : ''
}
</script>

<template>
  <section class="music">
    <h3>{{ t('app-music') }}</h3>
    <div class="tempo">
      <label for="bpm-min">{{ t('app-tempo') }}</label>
      <input id="bpm-min" class="field" type="number" min="20" max="400" inputmode="numeric" :value="filters.bpmMin" :placeholder="t('app-tempo-min')" :title="t('app-tempo-min-title')" @change="filters.bpmMin = bpmInput($event)" />
      <span aria-hidden="true">–</span>
      <input id="bpm-max" class="field" type="number" min="20" max="400" inputmode="numeric" :value="filters.bpmMax" :placeholder="t('app-tempo-max')" :aria-label="t('app-tempo-max-title')" :title="t('app-tempo-max-title')" @change="filters.bpmMax = bpmInput($event)" />
    </div>
    <label class="visually-hidden" for="music-key">{{ t('app-key') }}</label>
    <select id="music-key" v-model="filters.key" class="field">
      <option value="">{{ t('app-key-any') }}</option>
      <optgroup :label="t('app-key-major')">
        <option v-for="k in KEYS.filter((k) => !k.minor)" :key="k.value" :value="k.value">{{ keyName(k) }} · {{ k.camelot }}</option>
      </optgroup>
      <optgroup :label="t('app-key-minor')">
        <option v-for="k in KEYS.filter((k) => k.minor)" :key="k.value" :value="k.value">{{ keyName(k) }} · {{ k.camelot }}</option>
      </optgroup>
    </select>
    <label v-if="filters.key" class="check" :title="t('app-key-mix-title')">
      <input v-model="filters.mixKeys" type="checkbox" />
      {{ t('app-key-mix') }}
    </label>
  </section>
</template>

<style scoped>
.music { margin-top: 1rem; }
.music h3 { margin: 0 0 0.4rem; font-size: 0.75rem; font-weight: 650; color: var(--faint); }
.music .tempo { display: flex; gap: 0.3rem; align-items: center; margin-bottom: 0.35rem; }
.music .tempo label { flex: none; font-size: 0.85em; color: var(--muted); }
.music .tempo .field { flex: 1; min-width: 0; width: 0; }
.music select { width: 100%; }
.music .check { display: flex; gap: 0.35rem; align-items: center; margin-top: 0.35rem; font-size: 0.85em; color: var(--muted); }
</style>
