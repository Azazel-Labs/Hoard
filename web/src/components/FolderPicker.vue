<script setup lang="ts">
// A folder on the server: typed in, or picked by walking the server's folders.
import { ref } from 'vue'
import { api, ApiError, type FolderListing } from '../api'

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
  if (open.value) show(model.value.trim())
}

function choose(path: string) {
  model.value = path
  show(path)
}

</script>

<template>
  <div class="picker">
    <div class="row">
      <input :id="props.id" v-model="model" class="field mono" type="text" spellcheck="false" autocomplete="off" :placeholder="props.placeholder" />
      <button type="button" class="btn" :aria-expanded="open" @click="toggle">{{ open ? 'Close' : 'Browse…' }}</button>
    </div>
    <div v-if="open" class="browser" :aria-busy="loading">
      <div class="where">
        <button type="button" class="btn ghost" :disabled="listing?.parent == null || loading" title="Up one folder" @click="listing?.parent != null && choose(listing.parent)">
          <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 19V5M5 12l7-7 7 7" /></svg>
          Up
        </button>
        <span class="mono cur">{{ listing?.path || 'This computer' }}</span>
      </div>
      <p v-if="error" class="err">{{ error }}</p>
      <ul v-else-if="listing" class="dirs">
        <li v-if="!listing.dirs.length" class="none">No folders in here.</li>
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
.row .field { flex: 1; min-width: 0; }
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
