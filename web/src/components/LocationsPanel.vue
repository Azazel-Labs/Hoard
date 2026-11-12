<script setup lang="ts">
// Settings → Library locations: where the library's files live, and which kinds go where.
import { computed, onMounted, reactive, ref } from 'vue'
import { api, ApiError, KIND_LABEL, type AssetKind, type LibraryLocation, type LocationList } from '../api'
import { confirmDialog, showError, showOk } from '../ui'
import LocationForm from './LocationForm.vue'

const emit = defineEmits<{ unauthorized: []; changed: [] }>()

const data = ref<LocationList | null>(null)
const adding = ref(false)
const editing = ref<number | null>(null)
const draft = reactive({ name: '', kinds: [] as AssetKind[], catch_all: false })
const busy = ref(false)

const managed = computed(() => data.value?.locations.filter((l) => l.mode === 'managed') ?? [])
const watched = computed(() => data.value?.locations.filter((l) => l.mode === 'watched') ?? [])

function failed(e: unknown, what: string) {
  if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  showError(`${what}: ${(e as Error).message}`)
}

async function load() {
  try {
    data.value = await api.locations()
  } catch (e) {
    failed(e, "Couldn't load the library locations")
  }
}
onMounted(load)

function takes(l: LibraryLocation): string {
  const kinds = l.kinds.map((k) => KIND_LABEL[k] ?? k)
  const others = managed.value.some((m) => m.id !== l.id && m.kinds.length)
  if (l.catch_all) return kinds.length ? `${kinds.join(', ')}, and everything else` : others ? 'Everything the others don’t take' : 'Everything'
  return kinds.length ? kinds.join(', ') : 'Nothing yet'
}

function edit(l: LibraryLocation) {
  editing.value = l.id
  Object.assign(draft, { name: l.name, kinds: [...l.kinds], catch_all: l.catch_all })
}
function toggleKind(k: AssetKind) {
  draft.kinds = draft.kinds.includes(k) ? draft.kinds.filter((x) => x !== k) : [...draft.kinds, k]
}

async function save(l: LibraryLocation) {
  busy.value = true
  try {
    const patch = l.mode === 'managed' ? { name: draft.name.trim() || l.name, kinds: draft.kinds, catch_all: draft.catch_all } : { name: draft.name.trim() || l.name }
    await api.patchLocation(l.id, patch)
    editing.value = null
    await load()
  } catch (e) {
    failed(e, "Couldn't save the location")
  } finally {
    busy.value = false
  }
}

async function remove(l: LibraryLocation) {
  const ok = await confirmDialog(
    l.mode === 'watched'
      ? {
          title: `Stop watching ${l.name}?`,
          body: l.packs
            ? `Its ${l.packs} pack${l.packs === 1 ? '' : 's'} leave the hoard, with their tags and notes. The files in the folder aren't touched.`
            : "The files in the folder aren't touched.",
          confirm: 'Stop watching',
          danger: true,
        }
      : {
          title: `Remove ${l.name}?`,
          body: 'Only an empty location can be removed. New files go to the other locations from now on.',
          confirm: 'Remove',
          danger: true,
        },
  )
  if (!ok) return
  busy.value = true
  try {
    await api.deleteLocation(l.id)
    showOk(`Removed ${l.name}`)
    await load()
    emit('changed')
  } catch (e) {
    failed(e, `Couldn't remove ${l.name}`)
  } finally {
    busy.value = false
  }
}

function scannedAgo(iso?: string | null): string {
  if (!iso) return 'not scanned yet'
  const mins = Math.floor((Date.now() - new Date(iso).getTime()) / 60_000)
  if (mins < 1) return 'scanned just now'
  if (mins < 60) return `scanned ${mins} min ago`
  const hours = Math.floor(mins / 60)
  if (hours < 24) return `scanned ${hours} h ago`
  return `scanned ${new Date(iso).toLocaleDateString(undefined, { month: 'short', day: 'numeric' })}`
}

const scanning = ref<number | null>(null)
async function scanNow(l: LibraryLocation) {
  scanning.value = l.id
  try {
    await api.scanLocation(l.id)
    // A scan takes a moment; new folders then ingest in the background and show up in the library as they finish.
    await new Promise((r) => setTimeout(r, 1500))
    await load()
    emit('changed')
  } catch (e) {
    failed(e, `Couldn't scan ${l.name}`)
  } finally {
    scanning.value = null
  }
}

async function created(l: LibraryLocation) {
  adding.value = false
  showOk(l.mode === 'watched' ? `Watching ${l.name}` : `Added ${l.name}`)
  await load()
  emit('changed')
}
</script>

<template>
  <div class="locs" :aria-busy="busy">
    <p v-if="!data" class="loading">Loading…</p>
    <template v-else>
      <section v-for="group in [{ id: 'managed', title: 'Libraries Hoard keeps', list: managed }, { id: 'watched', title: 'Watched folders', list: watched }]" :key="group.id">
        <template v-if="group.list.length || group.id === 'managed'">
          <div class="sh"><h3>{{ group.title }}</h3></div>
          <p v-if="group.id === 'managed'" class="hint">
            Uploads and imports are copied in here. Each location takes the kinds it ticks; the catch-all takes the rest.
          </p>
          <p v-else class="hint">Folders of packs indexed where they are. Hoard never changes the files in them, and looks for new and removed folders every 15 minutes.</p>
          <p v-if="!group.list.length" class="hint empty">None yet, so there's nowhere to put an upload. Add one below.</p>
          <ul class="list">
            <li v-for="l in group.list" :key="l.id" :class="{ open: editing === l.id }">
              <div class="top">
                <span class="dot" :data-on="l.online" :title="l.online ? 'Reachable' : 'Can’t be reached right now'" />
                <div class="what">
                  <strong>{{ l.name }} <span v-if="l.catch_all" class="tag">catch-all</span></strong>
                  <small class="mono">{{ l.path }}</small>
                  <small>
                    <template v-if="l.mode === 'managed'">{{ takes(l) }}</template>
                    <template v-if="l.mode === 'managed'"> · </template>{{ l.packs }} pack{{ l.packs === 1 ? '' : 's' }}
                    <template v-if="l.mode === 'watched'"> · {{ scannedAgo(l.scanned_at) }}</template>
                    <template v-if="!l.online"> · <span class="off">can’t be reached</span></template>
                  </small>
                </div>
                <div v-if="editing !== l.id" class="acts">
                  <button v-if="l.mode === 'watched'" type="button" class="btn ghost" :disabled="busy || scanning === l.id" @click="scanNow(l)">
                    {{ scanning === l.id ? 'Scanning…' : 'Scan now' }}
                  </button>
                  <button type="button" class="btn ghost" :disabled="busy" @click="edit(l)">Edit</button>
                  <button type="button" class="btn ghost danger" :disabled="busy" @click="remove(l)">Remove</button>
                </div>
              </div>
              <form v-if="editing === l.id" class="edit" @submit.prevent="save(l)">
                <label class="lbl" :for="`ln-${l.id}`">Name</label>
                <input :id="`ln-${l.id}`" v-model="draft.name" class="field" type="text" maxlength="80" />
                <template v-if="l.mode === 'managed'">
                  <span class="lbl">Files that go here</span>
                  <div>
                    <div class="kinds" role="group" aria-label="Asset kinds">
                      <label v-for="k in data.kinds" :key="k" class="kind" :class="{ on: draft.kinds.includes(k) }">
                        <input type="checkbox" :checked="draft.kinds.includes(k)" @change="toggleKind(k)" />
                        {{ KIND_LABEL[k] }}
                      </label>
                    </div>
                    <label class="check"><input v-model="draft.catch_all" type="checkbox" /> Also everything no other location takes</label>
                    <p class="hint">Applies to files added from now on; what's already here stays put.</p>
                  </div>
                </template>
                <span />
                <div class="eacts">
                  <button type="submit" class="btn primary" :disabled="busy">Save</button>
                  <button type="button" class="btn ghost" @click="editing = null">Cancel</button>
                </div>
              </form>
            </li>
          </ul>
        </template>
      </section>

      <section>
        <div class="sh"><h3>Add a location</h3></div>
        <LocationForm
          v-if="adding"
          :suggested="managed.length ? '' : data.suggested"
          :kinds="data.kinds"
          :first="!data.locations.length"
          cancellable
          @created="created"
          @cancel="adding = false"
          @unauthorized="emit('unauthorized')"
        />
        <template v-else>
          <p class="hint">Another disk for some kinds of files, or a folder of packs you already have.</p>
          <button type="button" class="btn" @click="adding = true">Add a location…</button>
        </template>
      </section>
    </template>
  </div>
</template>

<style scoped>
.locs[aria-busy='true'] { cursor: progress; }
.loading { color: var(--muted); font-size: var(--step--1); }
section { padding: 1rem 0 1.1rem; border-top: 1px solid var(--line); }
section:first-child { border-top: 0; padding-top: 0.2rem; }
section:empty { display: none; }
.sh { display: flex; align-items: center; gap: 0.4rem; min-height: 1.75rem; margin-bottom: 0.3rem; }
h3 { margin: 0; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
.hint { margin: 0 0 0.6rem; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.hint.empty { color: var(--text); }
.list { display: grid; gap: 0.4rem; margin: 0 0 0.4rem; padding: 0; list-style: none; }
.list li { padding: 0.6rem 0.75rem; border: 1px solid var(--line); border-radius: var(--r-chip); font-size: var(--step--1); }
.list li.open { border-color: var(--muted); background: var(--raised); }
.top { display: flex; gap: 0.7rem; align-items: center; }
.dot { flex: none; align-self: flex-start; margin-top: 0.4rem; width: 0.5rem; height: 0.5rem; border-radius: 50%; background: var(--danger); }
.dot[data-on='true'] { background: var(--ok); }
.what { flex: 1; min-width: 0; display: grid; gap: 0.15rem; }
.what small { color: var(--muted); font-size: inherit; overflow-wrap: anywhere; }
.tag { margin-left: 0.3rem; padding: 0.05em 0.45em; border: 1px solid var(--line); border-radius: 999px; font-size: 0.7rem; font-weight: 500; color: var(--muted); }
.off { color: var(--danger); }
.acts { display: flex; gap: 0.3rem; flex: none; }
.edit { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.6rem 0.9rem; align-items: baseline; margin-top: 0.8rem; padding-top: 0.8rem; border-top: 1px solid var(--line); }
.edit > .field { max-width: 22rem; }
.edit .hint { margin: 0.4rem 0 0; }
.lbl { color: var(--muted); }
.eacts { display: flex; gap: 0.5rem; }
.kinds { display: flex; flex-wrap: wrap; gap: 0.35rem; margin-bottom: 0.5rem; }
.kind {
  display: inline-flex; align-items: center; gap: 0.35rem; padding: 0.25em 0.65em; border: 1px solid var(--line); border-radius: 999px;
  cursor: pointer; user-select: none; transition: border-color var(--dur-1), background-color var(--dur-1), color var(--dur-1);
}
.kind input { position: absolute; opacity: 0; pointer-events: none; }
.kind:hover { border-color: var(--muted); }
.kind.on { background: var(--text); color: var(--bg); border-color: var(--text); font-weight: 600; }
.kind:has(:focus-visible) { outline: 2px solid var(--accent); outline-offset: 2px; }
.check { display: inline-flex; gap: 0.45rem; align-items: center; }
@media (max-width: 30rem) {
  .top { flex-wrap: wrap; }
  .acts { width: 100%; justify-content: flex-end; }
  .edit { grid-template-columns: minmax(0, 1fr); gap: 0.35rem; }
}
</style>
