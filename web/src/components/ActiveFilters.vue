<script setup lang="ts">
// What the library is narrowed to, above the grid: how many match, a chip for each filter (removing it takes it
// off), Clear all, and saving the search under a name.
import { computed, nextTick, ref } from 'vue'
import { auth } from '../auth'
import { clearFilters, filterQuery, filters, hasFilters, heardNoise, savedNow } from '../filters'
import { t } from '../i18n'
import { findKey, keyName } from '../keys'
import { saveSearch } from '../mine'
import { showOk, useTween } from '../ui'
import TagChip from './TagChip.vue'

const props = defineProps<{
  /** The first page of a new search is on its way and nothing is showing yet. */
  firstLoad: boolean
  total: number
}>()
const shownTotal = useTween(() => props.total)
/** Signed in, rather than browsing as a guest: only accounts save searches. */
const member = computed(() => !!auth.status?.signed_in)

/** The tempo range being searched, in words ("90–120", "90+", "≤120"); `''` for any. */
const tempoLabel = computed(() => {
  const [lo, hi] = [filters.bpmMin, filters.bpmMax]
  if (lo !== '' && hi !== '') return `${lo}–${hi}`
  return lo !== '' ? `${lo}+` : hi !== '' ? `≤${hi}` : ''
})
const chosenKey = computed(() => (filters.key ? findKey(filters.key) : undefined))

const naming = ref(false)
const saveName = ref('')
const saveNameEl = ref<HTMLInputElement | null>(null)
async function startNaming() {
  saveName.value = [
    filters.q.trim(), filters.kind, ...filters.tags, filters.pack?.name, filters.collection?.name,
    filters.similar && t('app-save-name-like', { name: filters.similar.name }), filters.color,
    heardNoise.value ? t('app-imitation-heard') : filters.imitation, filters.favorites && t('app-save-name-favorites'), filters.project,
    tempoLabel.value && `${tempoLabel.value} BPM`, chosenKey.value && keyName(chosenKey.value),
  ].filter(Boolean).join(' · ').slice(0, 80)
  naming.value = true
  await nextTick()
  saveNameEl.value?.select()
}
async function saveCurrent() {
  const name = saveName.value.trim()
  if (!name) return
  if (await saveSearch(name, filterQuery())) {
    naming.value = false
    showOk(t('app-search-saved', { name }))
  }
}
</script>

<template>
  <div class="active">
    <span class="count">
      <span aria-hidden="true">{{ firstLoad ? t('app-searching') : t('app-asset-count', { count: shownTotal }) }}</span>
      <span class="visually-hidden" aria-live="polite">{{ firstLoad ? t('app-searching-live') : t('app-asset-count', { count: total }) }}</span>
    </span>
    <TagChip v-if="filters.pack" :tag="`pack:${filters.pack.name}`" removable @remove="filters.pack = null" />
    <TagChip v-if="filters.collection" :tag="`collection:${filters.collection.name}`" removable @remove="filters.collection = null" />
    <TagChip v-if="filters.favorites" tag="your:favorites" removable @remove="filters.favorites = false" />
    <TagChip v-if="filters.project" :tag="`project:${filters.project}`" removable @remove="filters.project = ''" />
    <TagChip v-if="filters.similar" :tag="`like:${filters.similar.name}`" removable @remove="filters.similar = null" />
    <TagChip v-if="filters.color" :tag="`color:${filters.color}`" :swatch="filters.color" removable @remove="filters.color = ''" />
    <TagChip v-if="filters.imitation" :tag="`sounds-like:${heardNoise ? t('app-imitation-heard') : filters.imitation}`" removable @remove="filters.imitation = ''" />
    <TagChip v-if="tempoLabel" :tag="`bpm:${tempoLabel}`" removable @remove="Object.assign(filters, { bpmMin: '', bpmMax: '' })" />
    <TagChip v-if="chosenKey" :tag="`key:${keyName(chosenKey)}${filters.mixKeys ? ' ±' : ''}`" removable @remove="filters.key = ''" />
    <TagChip v-if="filters.kind" :tag="`kind:${filters.kind}`" removable @remove="filters.kind = ''" />
    <TagChip v-for="t in filters.tags" :key="t" :tag="t" removable @remove="filters.tags = filters.tags.filter((x) => x !== t)" />
    <button v-if="hasFilters" type="button" class="linkbtn" @click="clearFilters">{{ t('app-clear-all') }}</button>
    <template v-if="member && hasFilters && !savedNow">
      <form v-if="naming" class="savesearch" @submit.prevent="saveCurrent">
        <label class="visually-hidden" for="save-name">{{ t('app-save-name-label') }}</label>
        <input id="save-name" ref="saveNameEl" v-model="saveName" class="field" maxlength="80" :placeholder="t('app-save-name-placeholder')" autocomplete="off" @keydown.esc.stop="naming = false" />
        <button class="btn" type="submit" :disabled="!saveName.trim()">{{ t('app-save') }}</button>
        <button class="btn ghost" type="button" @click="naming = false">{{ t('app-cancel') }}</button>
      </form>
      <button v-else type="button" class="linkbtn" :title="t('app-save-search-title')" @click="startNaming">{{ t('app-save-search') }}</button>
    </template>
  </div>
</template>

<style scoped>
.count { font-size: var(--step--1); color: var(--muted); margin-right: 0.35rem; font-variant-numeric: tabular-nums; }
.savesearch { display: inline-flex; gap: 0.3rem; align-items: center; }
.savesearch .field { width: 12rem; padding-block: 0.2rem; }
</style>
