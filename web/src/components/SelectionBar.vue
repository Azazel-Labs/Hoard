<script setup lang="ts">
import { ref } from 'vue'
import { api, type Asset, type BulkEdit, type Collection } from '../api'
import { filters } from '../filters'
import { t } from '../i18n'
import { showError, showOk } from '../ui'
import CollectionPicker from './CollectionPicker.vue'

// The bar along the bottom while assets are picked: download them, tag them, file them, star or archive them.
const props = defineProps<{
  /** The ids picked. */
  checked: string[]
  /** The assets loaded in the grid. */
  items: Asset[]
  /** Whether the bag beside the grid has the download buttons, so the bar leaves them out. */
  bagShown: boolean
  /** The zip of what's picked. */
  zipUrl: string
  /** How many have a recovered WAV/PNG that can go in instead of the .uasset. */
  withSource: number
  editor: boolean
  member: boolean
  collections: Collection[]
  /** Whether everything picked is a favorite already. */
  allStarred: boolean
}>()
const preferSources = defineModel<boolean>('preferSources', { required: true })
const emit = defineEmits<{
  /** A bulk edit went through: show it on the loaded tiles. */
  patched: [ids: string[], edit: Omit<BulkEdit, 'ids'>]
  credits: []
  collect: [target: { id?: string; create?: string }]
  uncollect: [id: string]
  favorite: [on: boolean]
  all: []
  archive: []
  done: []
}>()

const bulkTag = ref('')
const bulkCategory = ref('')
const bulkBusy = ref(false)

/** One edit to many assets in one go. False if it failed. */
async function bulkEdit(ids: string[], edit: Omit<BulkEdit, 'ids'>): Promise<boolean> {
  if (!ids.length) return true
  bulkBusy.value = true
  try {
    await api.bulkAssets({ ids, ...edit })
  } catch (e) {
    showError(t('app-bulk-failed', { count: ids.length, error: (e as Error).message }))
    return false
  } finally {
    bulkBusy.value = false
  }
  emit('patched', ids, edit)
  return true
}
async function bulkTagEdit(remove: boolean) {
  const tag = bulkTag.value.trim().toLowerCase()
  const ids = [...props.checked]
  if (!tag || !ids.length) return
  // Undo only touches the assets this changed.
  const has = (id: string) => !!props.items.find((a) => a.id === id)?.tags.some((t) => t.tag === tag)
  const touched = ids.filter((id) => has(id) === remove)
  if (!(await bulkEdit(ids, remove ? { remove_tags: [tag] } : { add_tags: [tag] }))) return
  bulkTag.value = ''
  const undo = remove ? { add_tags: [tag] } : { remove_tags: [tag] }
  showOk(remove ? t('app-bulk-untagged', { tag, count: touched.length }) : t('app-bulk-tagged', { tag, count: ids.length }), {
    label: t('app-undo'),
    run: () => void bulkEdit(touched, undo),
  })
}
async function bulkSetCategory() {
  const category = bulkCategory.value.trim()
  const ids = [...props.checked]
  if (!ids.length) return
  // Remember each one's old category so Undo can put them back.
  const before = new Map<string, string[]>()
  for (const id of ids) {
    const c = props.items.find((a) => a.id === id)?.category ?? ''
    before.set(c, [...(before.get(c) ?? []), id])
  }
  if (!(await bulkEdit(ids, { category }))) return
  bulkCategory.value = ''
  showOk(category ? t('app-bulk-category-set', { category, count: ids.length }) : t('app-bulk-category-cleared', { count: ids.length }), {
    label: t('app-undo'),
    run: () => {
      for (const [c, group] of before) void bulkEdit(group, { category: c })
    },
  })
}
</script>

<template>
  <div class="selbar" role="region" :aria-label="t('app-selected-label')">
    <strong>{{ checked.length ? t('app-selected', { count: checked.length }) : t('app-pick-assets') }}</strong>
    <a v-if="checked.length && !bagShown" class="btn primary" :href="zipUrl" download>{{ t('app-download-zip') }}</a>
    <button v-if="checked.length && !bagShown" type="button" class="btn" :title="t('app-credits-title')" @click="emit('credits')">{{ t('app-credits') }}</button>
    <label v-if="withSource && !bagShown" class="srccheck" :title="t('app-prefer-sources-title')">
      <input v-model="preferSources" type="checkbox" /> {{ t('app-prefer-sources', { count: withSource }) }}
    </label>
    <template v-if="editor">
      <form class="bulktag" @submit.prevent="bulkTagEdit(false)">
        <label class="visually-hidden" for="bulk-tag">{{ t('app-bulk-tag-label') }}</label>
        <input id="bulk-tag" v-model="bulkTag" class="field" :placeholder="t('app-bulk-tag-placeholder')" autocomplete="off" :disabled="bulkBusy || !checked.length" />
        <button class="btn" type="submit" :disabled="!bulkTag.trim() || bulkBusy" :aria-busy="bulkBusy" :title="t('app-bulk-tag-add-title')">{{ t('app-bulk-tag-add') }}</button>
        <button class="btn" type="button" :disabled="!bulkTag.trim() || bulkBusy" :title="t('app-bulk-tag-remove-title')" @click="bulkTagEdit(true)">{{ t('app-bulk-tag-remove') }}</button>
      </form>
      <form class="bulktag" @submit.prevent="bulkSetCategory">
        <label class="visually-hidden" for="bulk-cat">{{ t('app-bulk-category-label') }}</label>
        <input id="bulk-cat" v-model="bulkCategory" class="field cat" :placeholder="t('app-bulk-category-placeholder')" autocomplete="off" :disabled="bulkBusy || !checked.length" />
        <button class="btn" type="submit" :disabled="bulkBusy || !checked.length" :title="bulkCategory.trim() ? t('app-bulk-category-set-title') : t('app-bulk-category-clear-title')">
          {{ bulkCategory.trim() ? t('app-bulk-category-set-button') : t('app-bulk-category-clear-button') }}
        </button>
      </form>
      <CollectionPicker
        v-if="checked.length"
        :collections="collections"
        up
        @add="(id) => emit('collect', { id })"
        @create="(name) => emit('collect', { create: name })"
      />
      <button v-if="filters.collection && checked.length" type="button" class="btn" @click="emit('uncollect', filters.collection.id)">
        {{ t('app-remove-from-collection', { name: filters.collection.name }) }}
      </button>
    </template>
    <button v-if="member && checked.length" type="button" class="btn" @click="emit('favorite', !allStarred)">
      {{ allStarred ? t('app-unfavorite') : t('app-favorite') }}
    </button>
    <button type="button" class="linkbtn" @click="emit('all')">{{ t('app-select-all', { count: items.length }) }}</button>
    <button v-if="editor" type="button" class="btn danger" :disabled="!checked.length || bulkBusy" :title="t('app-archive-selected-title')" @click="emit('archive')">{{ t('app-archive') }}</button>
    <button type="button" class="btn ghost" @click="emit('done')">{{ t('app-done') }}</button>
  </div>
</template>

<style scoped>
.selbar {
  position: fixed; left: 50%; bottom: 1rem; translate: -50% 0; z-index: 25;
  display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; max-width: calc(100vw - 2rem);
  padding: 0.6rem 0.75rem 0.6rem 1rem; border: 1px solid var(--muted); border-radius: var(--r-tile);
  background: var(--raised); box-shadow: 0 10px 30px var(--shadow); font-size: var(--step--1);
}
/* With a panel open, the bar centers over the grid so it doesn't cover the panel. */
@media (min-width: 901px) {
  .selbar.beside {
    left: calc(50% - var(--detail-w, 420px) / 2); max-width: calc(100vw - var(--detail-w, 420px) - 2rem);
  }
}
.bulktag { display: flex; gap: 0.35rem; }
.srccheck { display: inline-flex; gap: 0.35rem; align-items: center; color: var(--muted); }
.bulktag .field { width: 11rem; padding: 0.35em 0.6em; }
@media (max-width: 900px) {
  .selbar { left: 0.5rem; right: 0.5rem; bottom: 0.5rem; translate: none; max-width: none; gap: 0.5rem; justify-content: space-between; }
  .bulktag { order: 3; flex-basis: 100%; }
  .bulktag .field { flex: 1; width: auto; }
}
</style>
