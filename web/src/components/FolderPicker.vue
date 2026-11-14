<script setup lang="ts">
// A folder on the server: typed in, with the folders that match offered as you type (like Plex), or picked by
// walking the server's folders.
import { computed, nextTick, onBeforeUnmount, ref, useId } from 'vue'
import { api, ApiError, type FolderListing } from '../api'
import { t } from '../i18n'

const model = defineModel<string>({ required: true })
const props = defineProps<{ id?: string; placeholder?: string }>()
const emit = defineEmits<{ unauthorized: [] }>()

const open = ref(false)
const listing = ref<FolderListing | null>(null)
const loading = ref(false)
const error = ref('')

async function show(path: string) {
  loading.value = true
  error.value = ''
  try {
    listing.value = await api.browse(path)
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    // A path that isn't there yet (a managed folder Hoard will create): show its parent instead.
    if (path && listing.value === null) {
      const up = path.replace(/[\\/]+$/, '').replace(/[\\/][^\\/]*$/, '')
      if (up && up !== path) return show(up)
    }
    error.value = (e as Error).message
  } finally {
    loading.value = false
  }
}

function toggle() {
  open.value = !open.value
  suggestOpen.value = false
  if (open.value) show(model.value.trim())
}

function choose(path: string) {
  model.value = path
  show(path)
}

// Suggestions: the folders in the typed path's parent whose names start with what follows the last separator.
const listId = useId()
const suggestOpen = ref(false)
const active = ref(-1)
const suggestions = ref<{ name: string; path: string }[]>([])
const cache = new Map<string, Promise<FolderListing | null>>()
let timer: ReturnType<typeof setTimeout> | undefined
let asked = 0

function split(text: string): [parent: string, prefix: string] {
  const i = Math.max(text.lastIndexOf('/'), text.lastIndexOf('\\'))
  return i < 0 ? ['', text] : [text.slice(0, i + 1), text.slice(i + 1)]
}

function listingOf(parent: string): Promise<FolderListing | null> {
  let p = cache.get(parent)
  if (!p) {
    p = api.browse(parent).catch((e) => {
      if (e instanceof ApiError && e.status === 401) emit('unauthorized')
      cache.delete(parent)
      return null
    })
    cache.set(parent, p)
  }
  return p
}

async function suggest() {
  const text = model.value
  // A relative path can't be listed; leading `/` or a drive letter can.
  if (text && !/^([\\/]|[A-Za-z]:)/.test(text)) {
    suggestOpen.value = false
    return
  }
  const [parent, prefix] = split(text)
  const n = ++asked
  const l = await listingOf(parent)
  if (n !== asked) return
  const want = prefix.toLowerCase()
  suggestions.value = (l?.dirs ?? []).filter((d) => d.name.toLowerCase().startsWith(want)).slice(0, 50)
  active.value = suggestions.value.length && want ? 0 : -1
  suggestOpen.value = suggestions.value.length > 0
}

function onInput() {
  clearTimeout(timer)
  timer = setTimeout(suggest, 120)
}

function onFocus() {
  if (!open.value) suggest()
}

function onBlur() {
  // Late enough for a click on a suggestion to land first.
  setTimeout(() => (suggestOpen.value = false), 150)
}

async function pick(d: { path: string }) {
  const sep = d.path.includes('\\') ? '\\' : '/'
  model.value = d.path.endsWith(sep) ? d.path : d.path + sep
  if (open.value) show(model.value)
  await nextTick()
  suggest()
}

function onKey(e: KeyboardEvent) {
  if (!suggestOpen.value || !suggestions.value.length) {
    if (e.key === 'ArrowDown') suggest()
    return
  }
  const n = suggestions.value.length
  if (e.key === 'ArrowDown') {
    active.value = (active.value + 1) % n
  } else if (e.key === 'ArrowUp') {
    active.value = active.value <= 0 ? n - 1 : active.value - 1
  } else if ((e.key === 'Enter' || e.key === 'Tab') && active.value >= 0) {
    pick(suggestions.value[active.value])
  } else if (e.key === 'Escape') {
    // Closes the list, not the dialog the picker is in.
    e.stopPropagation()
    suggestOpen.value = false
  } else {
    return
  }
  e.preventDefault()
  if (e.key.startsWith('Arrow')) document.getElementById(`${listId}-${active.value}`)?.scrollIntoView({ block: 'nearest' })
}

const activeId = computed(() => (suggestOpen.value && active.value >= 0 ? `${listId}-${active.value}` : undefined))
onBeforeUnmount(() => clearTimeout(timer))
</script>

<template>
  <div class="picker">
    <div class="row">
      <div class="combo">
        <input
          :id="props.id"
          v-model="model"
          class="field mono"
          type="text"
          spellcheck="false"
          autocomplete="off"
          role="combobox"
          aria-autocomplete="list"
          :aria-expanded="suggestOpen"
          :aria-controls="listId"
          :aria-activedescendant="activeId"
          :placeholder="props.placeholder"
          @input="onInput"
          @focus="onFocus"
          @blur="onBlur"
          @keydown="onKey"
        />
        <ul v-show="suggestOpen" :id="listId" class="suggest" role="listbox" :aria-label="t('folders-list')">
          <li
            v-for="(d, i) in suggestions"
            :id="`${listId}-${i}`"
            :key="d.path"
            role="option"
            :aria-selected="i === active"
            @mousedown.prevent="pick(d)"
            @mousemove="active = i"
          >
            <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round" aria-hidden="true"><path d="M3 7.5A1.5 1.5 0 0 1 4.5 6H9l2 2h8.5A1.5 1.5 0 0 1 21 9.5v8a1.5 1.5 0 0 1-1.5 1.5h-15A1.5 1.5 0 0 1 3 17.5z" /></svg>
            <span class="mono">{{ d.name }}</span>
          </li>
        </ul>
      </div>
      <button type="button" class="btn" :aria-expanded="open" @click="toggle">{{ open ? t('folders-close') : t('folders-browse') }}</button>
    </div>
    <div v-if="open" class="browser" :aria-busy="loading">
      <div class="where">
        <button type="button" class="btn ghost" :disabled="listing?.parent == null || loading" :title="t('folders-up-title')" @click="listing?.parent != null && choose(listing.parent)">
          <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 19V5M5 12l7-7 7 7" /></svg>
          {{ t('folders-up') }}
        </button>
        <span class="mono cur">{{ listing?.path || t('folders-this-computer') }}</span>
      </div>
      <p v-if="error" class="err">{{ error }}</p>
      <ul v-else-if="listing" class="dirs">
        <li v-if="!listing.dirs.length" class="none">{{ t('folders-none') }}</li>
        <li v-for="d in listing.dirs" :key="d.path">
          <button type="button" @click="choose(d.path)">
            <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round" aria-hidden="true"><path d="M3 7.5A1.5 1.5 0 0 1 4.5 6H9l2 2h8.5A1.5 1.5 0 0 1 21 9.5v8a1.5 1.5 0 0 1-1.5 1.5h-15A1.5 1.5 0 0 1 3 17.5z" /></svg>
            <span>{{ d.name }}</span>
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.picker { display: grid; gap: 0.4rem; min-width: 0; }
.row { display: flex; gap: 0.5rem; }
.combo { position: relative; flex: 1; min-width: 0; }
.combo .field { width: 100%; }
.suggest {
  position: absolute; z-index: 20; top: calc(100% + 2px); left: 0; right: 0; max-height: 15rem; overflow-y: auto;
  margin: 0; padding: 0.25rem; list-style: none; border: 1px solid var(--line); border-radius: var(--r-chip);
  background: var(--bg); box-shadow: 0 6px 20px rgb(0 0 0 / 0.18);
}
.suggest li {
  display: flex; align-items: center; gap: 0.5rem; padding: 0.3rem 0.5rem; border-radius: var(--r-chip);
  font-size: var(--step--1); cursor: pointer;
}
.suggest li[aria-selected='true'] { background: var(--raised); }
.suggest svg { flex: none; color: var(--muted); }
.suggest span { overflow-wrap: anywhere; }
.browser { border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--bg); overflow: hidden; }
.browser[aria-busy='true'] { cursor: progress; }
.where { display: flex; align-items: center; gap: 0.5rem; padding: 0.3rem 0.4rem; border-bottom: 1px solid var(--line); background: var(--raised); }
.where .btn { padding: 0.3em 0.6em; }
.cur { overflow-wrap: anywhere; font-size: var(--step--1); color: var(--muted); }
.dirs { max-height: 15rem; overflow-y: auto; margin: 0; padding: 0.25rem; list-style: none; }
.dirs button {
  display: flex; align-items: center; gap: 0.5rem; width: 100%; padding: 0.35rem 0.5rem; border: 0; border-radius: var(--r-chip);
  background: none; color: var(--text); font-size: var(--step--1); text-align: left; cursor: pointer;
}
.dirs button:hover { background: var(--raised); }
.dirs svg { flex: none; color: var(--muted); }
.dirs span { overflow-wrap: anywhere; }
.none, .err { margin: 0; padding: 0.6rem 0.7rem; font-size: var(--step--1); color: var(--muted); }
.err { color: var(--danger); }
</style>
