<script setup lang="ts">
// Settings → Library locations: where the library's files live, and which kinds go where.
import { computed, onMounted, reactive, ref } from 'vue'
import { api, ApiError, KIND_LABEL, type AssetKind, type LibraryLocation, type LocationList } from '../api'
import { confirmDialog, showError, showOk } from '../ui'
import { formatDate, t } from '../i18n'
import KindChips from './KindChips.vue'
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
  showError(t('locations-failed', { what, error: (e as Error).message }))
}

async function load() {
  try {
    data.value = await api.locations()
  } catch (e) {
    failed(e, t('locations-load-failed'))
  }
}
onMounted(load)

function takes(l: LibraryLocation): string {
  const kinds = l.kinds.map((k) => KIND_LABEL[k] ?? k)
  const others = managed.value.some((m) => m.id !== l.id && m.kinds.length)
  if (l.catch_all) return kinds.length ? t('locations-takes-and-rest', { kinds: kinds.join(', ') }) : others ? t('locations-takes-rest') : t('locations-takes-everything')
  return kinds.length ? kinds.join(', ') : t('locations-takes-nothing')
}

function edit(l: LibraryLocation) {
  editing.value = l.id
  Object.assign(draft, { name: l.name, kinds: [...l.kinds], catch_all: l.catch_all })
}

async function save(l: LibraryLocation) {
  busy.value = true
  try {
    const patch = l.mode === 'managed' ? { name: draft.name.trim() || l.name, kinds: draft.kinds, catch_all: draft.catch_all } : { name: draft.name.trim() || l.name }
    await api.patchLocation(l.id, patch)
    editing.value = null
    await load()
  } catch (e) {
    failed(e, t('locations-save-failed'))
  } finally {
    busy.value = false
  }
}

async function remove(l: LibraryLocation) {
  const watchedLoc = l.mode === 'watched'
  const others = managed.value.filter((m) => m.id !== l.id).length
  const ok = await confirmDialog(
    watchedLoc
      ? {
          title: t('locations-unwatch-title', { name: l.name }),
          body: l.packs
            ? t('locations-unwatch-body', { count: l.packs })
            : t('locations-unwatch-body-empty'),
          confirm: t('locations-unwatch'),
          danger: true,
        }
      : {
          title: t('locations-remove-title', { name: l.name }),
          body: others
            ? t('locations-remove-body')
            : t('locations-remove-body-only'),
          confirm: t('locations-remove'),
          danger: true,
        },
  )
  if (!ok) return
  busy.value = true
  try {
    try {
      await api.deleteLocation(l.id)
      showOk(t('locations-removed', { name: l.name }))
    } catch (e) {
      if (!(e instanceof ApiError) || watchedLoc) throw e
      if (e.status === 409 && others) {
        // It still holds files: move them out, and it's removed when that's done.
        await api.emptyLocation(l.id)
        showOk(t('locations-emptying', { name: l.name }))
      } else if (e.status === 503) {
        // Its disk can't be reached, so its files can't be moved.
        const anyway = await confirmDialog({
          title: t('locations-offline-title', { name: l.name }),
          body: t('locations-offline-body'),
          confirm: t('locations-remove-anyway'),
          danger: true,
        })
        if (!anyway) return
        await api.deleteLocation(l.id, true)
        showOk(t('locations-removed', { name: l.name }))
      } else {
        throw e
      }
    }
    await load()
    emit('changed')
  } catch (e) {
    failed(e, t('locations-remove-failed', { name: l.name }))
  } finally {
    busy.value = false
  }
}

async function rebalance() {
  busy.value = true
  try {
    await api.rebalance()
    showOk(t('locations-rebalancing'))
  } catch (e) {
    failed(e, t('locations-rebalance-failed'))
  } finally {
    busy.value = false
  }
}

function scannedAgo(iso?: string | null): string {
  if (!iso) return t('locations-scanned-never')
  const mins = Math.floor((Date.now() - new Date(iso).getTime()) / 60_000)
  if (mins < 1) return t('locations-scanned-now')
  if (mins < 60) return t('locations-scanned-minutes', { mins })
  const hours = Math.floor(mins / 60)
  if (hours < 24) return t('locations-scanned-hours', { hours })
  return t('locations-scanned-date', { date: formatDate(iso, { month: 'short', day: 'numeric' }) })
}

const scanning = ref<number | null>(null)
async function scanNow(l: LibraryLocation) {
  scanning.value = l.id
  try {
    await api.scanLocation(l.id)
    // Wait for the scan to finish (up to half a minute); new folders then ingest in the background and show up in
    // the library as they're done.
    const before = l.scanned_at
    for (let i = 0; i < 30; i++) {
      await new Promise((r) => setTimeout(r, 1000))
      await load()
      if (data.value?.locations.find((x) => x.id === l.id)?.scanned_at !== before) break
    }
    emit('changed')
  } catch (e) {
    failed(e, t('locations-scan-failed', { name: l.name }))
  } finally {
    scanning.value = null
  }
}

async function created(l: LibraryLocation) {
  adding.value = false
  showOk(l.mode === 'watched' ? t('locations-watching', { name: l.name }) : t('locations-added', { name: l.name }))
  await load()
  emit('changed')
}
</script>

<template>
  <div class="locs" :aria-busy="busy">
    <p v-if="!data" class="loading">{{ t('locations-loading') }}</p>
    <template v-else>
      <section v-for="group in [{ id: 'managed', title: t('locations-managed-title'), list: managed }, { id: 'watched', title: t('locations-watched-title'), list: watched }]" :key="group.id">
        <template v-if="group.list.length || group.id === 'managed'">
          <div class="sh"><h3>{{ group.title }}</h3></div>
          <p v-if="group.id === 'managed'" class="hint">
            {{ t('locations-managed-hint') }}
          </p>
          <p v-else class="hint">{{ t('locations-watched-hint') }}</p>
          <p v-if="!group.list.length" class="hint empty">{{ t('locations-managed-empty') }}</p>
          <ul class="list">
            <li v-for="l in group.list" :key="l.id" :class="{ open: editing === l.id }">
              <div class="top">
                <span class="dot" :data-on="l.online" :title="l.online ? t('locations-reachable') : t('locations-unreachable-now')" />
                <div class="what">
                  <strong>{{ l.name }} <span v-if="l.catch_all" class="tag">{{ t('locations-catch-all-tag') }}</span></strong>
                  <small class="mono">{{ l.path }}</small>
                  <small>
                    <template v-if="l.mode === 'managed'">{{ takes(l) }}</template>
                    <template v-else>{{ t('locations-packs', { count: l.packs }) }} · {{ scannedAgo(l.scanned_at) }}</template>
                    <template v-if="!l.online"> · <span class="off">{{ t('locations-unreachable') }}</span></template>
                  </small>
                </div>
                <div v-if="editing !== l.id" class="acts">
                  <button v-if="l.mode === 'watched'" type="button" class="btn ghost" :disabled="busy || scanning === l.id" @click="scanNow(l)">
                    {{ scanning === l.id ? t('locations-scanning') : t('locations-scan') }}
                  </button>
                  <button type="button" class="btn ghost" :disabled="busy" @click="edit(l)">{{ t('locations-edit') }}</button>
                  <button type="button" class="btn ghost danger" :disabled="busy" @click="remove(l)">{{ t('locations-remove') }}</button>
                </div>
              </div>
              <form v-if="editing === l.id" class="edit" @submit.prevent="save(l)">
                <label class="lbl" :for="`ln-${l.id}`">{{ t('locations-name') }}</label>
                <input :id="`ln-${l.id}`" v-model="draft.name" class="field" type="text" maxlength="80" />
                <template v-if="l.mode === 'managed'">
                  <span class="lbl">{{ t('locations-files') }}</span>
                  <div>
                    <KindChips v-model="draft.kinds" :kinds="data.kinds" />
                    <label class="check" :title="l.catch_all ? t('locations-catch-all-locked') : undefined">
                      <input v-model="draft.catch_all" type="checkbox" :disabled="l.catch_all" /> {{ t('locations-catch-all') }}
                    </label>
                    <p class="hint">{{ t('locations-edit-hint-before') }} <em>{{ t('locations-rebalance') }}</em> {{ t('locations-edit-hint-after') }}</p>
                  </div>
                </template>
                <span />
                <div class="eacts">
                  <button type="submit" class="btn primary" :disabled="busy">{{ t('locations-save') }}</button>
                  <button type="button" class="btn ghost" @click="editing = null">{{ t('locations-cancel') }}</button>
                </div>
              </form>
            </li>
          </ul>
          <div v-if="group.id === 'managed' && managed.length > 1" class="rebal">
            <button type="button" class="btn" :disabled="busy" @click="rebalance">{{ t('locations-rebalance') }}</button>
            <small>{{ t('locations-rebalance-hint') }}</small>
          </div>
        </template>
      </section>

      <section>
        <div class="sh"><h3>{{ t('locations-add-title') }}</h3></div>
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
          <p class="hint">{{ t('locations-add-hint') }}</p>
          <button type="button" class="btn" @click="adding = true">{{ t('locations-add-more') }}</button>
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
.check { display: inline-flex; gap: 0.45rem; align-items: center; }
.rebal { display: flex; gap: 0.7rem; align-items: center; margin-top: 0.6rem; }
.rebal small { color: var(--muted); font-size: var(--step--1); line-height: 1.4; }
@media (max-width: 30rem) {
  .top { flex-wrap: wrap; }
  .acts { width: 100%; justify-content: flex-end; }
  .edit { grid-template-columns: minmax(0, 1fr); gap: 0.35rem; }
}
</style>
