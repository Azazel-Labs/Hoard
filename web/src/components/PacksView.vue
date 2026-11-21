<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import {
  api, collectionCreditsUrl, collectionZipUrl, packCreditsUrl, packZipUrl, thumbUrl, type AssetKind, type Collection, type Pack, type Stats, type UnrealPackStatus,
} from '../api'
import { humanBytes } from '../format'
import { KIND_LABEL } from '../kinds'
import { confirmDialog, undoable } from '../ui'
import { archiveWithUndo } from '../archive'
import { mayEdit } from '../auth'
import { canDrawThumbs, cancelThumbRun, makePackThumbs, thumbRun } from '../browserThumbs'
import { formatDate, t } from '../i18n'
import PackContextForm from './PackContextForm.vue'

const props = defineProps<{ packs: Pack[]; stats: Stats | null; collections: Collection[] }>()

/** May change packs and collections (a guest is asked to sign in when they try). */
const editor = computed(() => mayEdit())

// Close an open "More" menu when clicking anywhere else.
function closeMenus(e: MouseEvent) {
  for (const d of document.querySelectorAll<HTMLDetailsElement>('.packs details.menu[open]')) {
    if (!d.contains(e.target as Node)) d.removeAttribute('open')
  }
}
// Esc closes an open menu and hands focus back to its button.
function escMenus(e: KeyboardEvent) {
  if (e.key !== 'Escape') return
  for (const d of document.querySelectorAll<HTMLDetailsElement>('.packs details.menu[open]')) {
    d.removeAttribute('open')
    d.querySelector('summary')?.focus()
  }
}
onMounted(() => {
  document.addEventListener('click', closeMenus)
  document.addEventListener('keydown', escMenus)
})
onBeforeUnmount(() => {
  document.removeEventListener('click', closeMenus)
  document.removeEventListener('keydown', escMenus)
})

// Deleted packs and collections vanish at once and only go from the server once their Undo toast runs out.
const hidden = reactive(new Set<string>())
/** Packs that just finished: their row flashes once so the moment doesn't go unnoticed. */
const justReady = reactive(new Set<string>())
const lastStatus = new Map<string, Pack['status']>()
watch(() => props.packs, (list) => {
  for (const p of list) {
    const was = lastStatus.get(p.id)
    if (was && was !== 'ready' && p.status === 'ready') {
      justReady.add(p.id)
      setTimeout(() => justReady.delete(p.id), 1600)
    }
    lastStatus.set(p.id, p.status)
  }
}, { immediate: true })
const visiblePacks = computed(() => props.packs.filter((p) => !hidden.has(p.id)))
const visibleCollections = computed(() => props.collections.filter((c) => !hidden.has(c.id)))

// ---- finding packs: words, what's in them, where they're at, where they came from, and an order.
const query = ref('')
const kind = ref<AssetKind | ''>('')
const state = ref<'' | 'working' | 'failed' | 'untagged' | 'unlicensed'>('')
/** A store's name, `none` for packs with no store page, or empty for any. */
const store = ref('')
type PackSort = 'created' | 'updated' | 'name' | 'size' | 'assets'
/** Which way round each order usually goes: names A to Z, everything else most first. */
const SORT_UP: Record<PackSort, boolean> = { created: false, updated: false, name: true, size: false, assets: false }
const sort = ref<PackSort>('created')
const sortUp = ref(false)
// Another order starts its usual way round.
watch(sort, (by) => (sortUp.value = SORT_UP[by]))
const sortWay = computed(() => {
  const up = sortUp.value
  switch (sort.value) {
    case 'name':
      return up ? t('app-sort-a-to-z') : t('app-sort-z-to-a')
    case 'size':
      return up ? t('packs-sort-smallest') : t('packs-sort-size')
    case 'assets':
      return up ? t('packs-sort-fewest') : t('packs-sort-assets')
    default:
      return up ? t('packs-sort-old') : t('packs-sort-new')
  }
})
const working = (p: Pack) => p.status === 'uploading' || p.status === 'ingesting' || p.status === 'analyzing'

/** Kinds some pack has, most common first, for the kind chips. */
const kindsPresent = computed(() => {
  const n = new Map<AssetKind, number>()
  for (const p of visiblePacks.value) {
    for (const [k, c] of Object.entries(p.kinds ?? {})) if (c) n.set(k as AssetKind, (n.get(k as AssetKind) ?? 0) + 1)
  }
  return [...n.entries()].sort((a, b) => b[1] - a[1]).map(([k]) => k)
})
const stores = computed(() => [...new Set(visiblePacks.value.map((p) => p.store).filter((s): s is string => !!s))].sort())
const filtering = computed(() => !!(query.value.trim() || kind.value || state.value || store.value))
function clearFinding() {
  query.value = ''
  kind.value = ''
  state.value = ''
  store.value = ''
}

const shown = computed(() => {
  // Every word has to turn up somewhere: the name, store, license, description or tags.
  const words = query.value.trim().toLowerCase().split(/\s+/).filter(Boolean)
  const list = visiblePacks.value.filter((p) => {
    if (kind.value && !p.kinds?.[kind.value]) return false
    if (state.value === 'working' && !working(p)) return false
    if (state.value === 'failed' && p.status !== 'failed') return false
    if (state.value === 'untagged' && (p.description || p.tags.length)) return false
    if (state.value === 'unlicensed' && (p.license?.trim() || p.license_tier)) return false
    if (store.value === 'none' ? !!p.store_url : store.value && p.store !== store.value) return false
    if (!words.length) return true
    const hay = [p.name, p.store, p.license, p.license_tier, p.description, ...p.tags].filter(Boolean).join(' ').toLowerCase()
    return words.every((w) => hay.includes(w))
  })
  // Each ascending; turned round for the other way.
  const by: Record<PackSort, (a: Pack, b: Pack) => number> = {
    created: (a, b) => a.created_at.localeCompare(b.created_at),
    updated: (a, b) => (a.updated_at || a.created_at).localeCompare(b.updated_at || b.created_at),
    name: (a, b) => a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' }),
    size: (a, b) => a.total_bytes - b.total_bytes,
    assets: (a, b) => a.asset_count - b.asset_count,
  }
  const cmp = by[sort.value]
  return list.sort(sortUp.value ? cmp : (a, b) => cmp(b, a))
})

// Rows go into the page a batch at a time as the list scrolls, so a library of thousands of packs opens at once.
const BATCH = 40
const limit = ref(BATCH)
watch([query, kind, state, store, sort, sortUp], () => (limit.value = BATCH))
const rendered = computed(() => shown.value.slice(0, limit.value))
const moreEl = ref<HTMLElement | null>(null)
let moreObserver: IntersectionObserver | undefined
watch(moreEl, (el) => {
  moreObserver?.disconnect()
  if (!el) return
  moreObserver = new IntersectionObserver((entries) => {
    if (entries.some((e) => e.isIntersecting)) limit.value += BATCH
  }, { rootMargin: '1200px 0px' })
  moreObserver.observe(el)
})
onBeforeUnmount(() => moreObserver?.disconnect())

/** `v-focus`: take the keyboard when shown (the rename box). */
const vFocus = { mounted: (el: HTMLInputElement) => el.select() }

// Renaming a pack: searches, links and collections refer to it by id, so nothing else changes.
const renamingPack = ref<string | null>(null)
const packName = ref('')
function startPackRename(p: Pack) {
  renamingPack.value = p.id
  packName.value = p.name
}
async function savePackRename(p: Pack) {
  const name = packName.value.trim()
  renamingPack.value = null
  if (!name || name === p.name) return
  try {
    await api.renamePack(p.id, name)
    emit('changed')
  } catch (e) {
    emit('error', t('packs-rename-failed', { error: (e as Error).message }))
  }
}
const emit = defineEmits<{ browse: [Pack]; browseCollection: [Collection]; changed: []; error: [string] }>()

// Collections: rename inline, delete with a confirmation.
const renaming = ref<string | null>(null)
const nameDraft = ref('')
function startRename(c: Collection) {
  renaming.value = c.id
  nameDraft.value = c.name
}
async function saveRename(c: Collection) {
  const name = nameDraft.value.trim()
  renaming.value = null
  if (!name || name === c.name) return
  try {
    await api.updateCollection(c.id, { name })
    emit('changed')
  } catch (e) {
    emit('error', t('packs-rename-failed', { error: (e as Error).message }))
  }
}
function removeCollection(c: Collection) {
  undoable(t('packs-collection-deleted', { name: c.name }), {
    apply: () => hidden.add(c.id),
    revert: () => hidden.delete(c.id),
    commit: async () => {
      try {
        await api.deleteCollection(c.id)
        emit('changed')
      } catch (e) {
        hidden.delete(c.id)
        emit('error', t('packs-delete-failed', { error: (e as Error).message }))
      }
    },
  })
}

function statusText(status: Pack['status']): string {
  switch (status) {
    case 'uploading': return t('packs-status-uploading')
    case 'ingesting': return t('packs-status-ingesting')
    case 'analyzing': return t('packs-status-analyzing')
    case 'ready': return t('packs-status-ready')
    case 'failed': return t('packs-status-failed')
  }
  return status
}

function progress(p: Pack): number {
  return p.files_total ? Math.min(100, Math.round((p.files_done / p.files_total) * 100)) : 0
}

async function retag(p: Pack, llm: boolean) {
  try {
    await api.retagPack(p.id, llm ? { rules: false, llm: true } : { rules: true, llm: false })
    emit('changed')
  } catch (e) {
    emit('error', t('packs-retag-failed', { error: (e as Error).message }))
  }
}

async function readFiles(p: Pack) {
  try {
    await api.readPackFiles(p.id)
    emit('changed')
  } catch (e) {
    emit('error', t('packs-read-files-failed', { error: (e as Error).message }))
  }
}

// The store details form: one pack open at a time.
const editing = ref<string | null>(null)
function editContext(p: Pack) {
  editing.value = editing.value === p.id ? null : p.id
}

// Unreal conversion panel: one pack open at a time, like the store details.
const unrealOpen = ref<string | null>(null)
const unrealStatus = ref<UnrealPackStatus | null>(null)
const unrealOpts = reactive({ discard_originals: false, drop_unsupported: false })
const unrealBusy = ref(false)

async function openUnreal(p: Pack) {
  if (unrealOpen.value === p.id) {
    unrealOpen.value = null
    return
  }
  try {
    unrealStatus.value = await api.unrealStatus(p.id)
    unrealOpts.discard_originals = false
    unrealOpts.drop_unsupported = false
    unrealOpen.value = p.id
    editing.value = null
  } catch (e) {
    emit('error', t('packs-unreal-load-failed', { error: (e as Error).message }))
  }
}

async function convertUnreal(p: Pack) {
  if (unrealOpts.discard_originals && !(await confirmDialog({
    title: t('packs-unreal-discard-title'),
    body: t('packs-unreal-discard-body', { name: p.name }),
    confirm: t('packs-unreal-discard-confirm'),
    danger: true,
  }))) return
  unrealBusy.value = true
  try {
    await api.unrealConvert(p.id, { ...unrealOpts })
    unrealOpen.value = null
    emit('changed')
  } catch (e) {
    emit('error', t('packs-unreal-convert-failed', { error: (e as Error).message }))
  } finally {
    unrealBusy.value = false
  }
}

function unrealFact(p: Pack): string {
  const left = p.unreal_packages ?? 0
  const done = p.unreal_converted ?? 0
  if (!left) return t('packs-unreal-fact-converted', { done })
  if (!done) return t('packs-unreal-fact-packages', { left })
  return t('packs-unreal-fact-both', { done, left })
}

/** What a finished "Make thumbnails in this browser" run did. */
function thumbSummary(): string {
  const lead = thumbRun.done < thumbRun.total
    ? t('packs-thumbs-canceled', { done: thumbRun.done, total: thumbRun.total })
    : t('packs-thumbs-drew', { count: thumbRun.total })
  return thumbRun.failed
    ? t('packs-thumbs-result-failed', { lead, improved: thumbRun.improved, failed: thumbRun.failed })
    : t('packs-thumbs-result', { lead, improved: thumbRun.improved })
}

/** To the archive (Undo on the toast brings it back); it's deleted for good from there. */
async function archive(p: Pack) {
  hidden.add(p.id)
  const ok = await archiveWithUndo(t('packs-archived', { name: p.name, count: p.asset_count }), { packs: [p.id] }, () => {
    hidden.delete(p.id)
    emit('changed')
  })
  if (!ok) hidden.delete(p.id)
}
</script>

<template>
  <div class="packs">
    <p v-if="stats?.blobs_bad" class="summary">
      <strong class="bad">{{ t('packs-blobs-bad', { count: stats.blobs_bad }) }}</strong>
    </p>

    <section v-if="visibleCollections.length" class="colsec" :aria-label="t('packs-collections')">
      <h2>{{ t('packs-collections') }}</h2>
      <ul class="cols">
        <li v-for="c in visibleCollections" :key="c.id">
          <button type="button" class="cover checker" :aria-label="t('packs-browse-named', { name: c.name })" tabindex="-1" @click="$emit('browseCollection', c)">
            <img
              v-for="(id, i) in c.preview"
              :key="id"
              :src="thumbUrl({ id })"
              :class="{ glyphs: c.preview_kinds?.[i] === 'font' }"
              alt=""
              loading="lazy"
            />
            <span v-if="!c.preview.length" class="cover-empty">{{ c.name.slice(0, 1) }}</span>
          </button>
          <div class="body">
            <form v-if="renaming === c.id" class="rename" @submit.prevent="saveRename(c)">
              <label class="visually-hidden" :for="`rename-${c.id}`">{{ t('packs-collection-name') }}</label>
              <input :id="`rename-${c.id}`" v-model="nameDraft" class="field" @keydown.esc="renaming = null" />
              <button class="btn primary" type="submit">{{ t('packs-save') }}</button>
              <button class="btn ghost" type="button" @click="renaming = null">{{ t('packs-cancel') }}</button>
            </form>
            <button v-else type="button" class="name" @click="$emit('browseCollection', c)">{{ c.name }}</button>
            <p v-if="c.description" class="desc">{{ c.description }}</p>
            <div class="facts">
              <span>{{ t('packs-assets', { count: c.asset_count }) }}</span>
              <span>{{ humanBytes(c.total_bytes) }}</span>
              <span>{{ t('packs-updated', { date: formatDate(c.updated_at) }) }}</span>
            </div>
            <div class="actions">
              <button class="btn" type="button" @click="$emit('browseCollection', c)">{{ t('packs-browse') }}</button>
              <a v-if="c.asset_count" class="btn ghost" :href="collectionZipUrl(c.id)">{{ t('packs-collection-download') }}</a>
              <a v-if="c.asset_count" class="btn ghost" :href="collectionCreditsUrl(c.id)" download="CREDITS.md" :title="t('packs-collection-credits-title')">{{ t('packs-collection-credits') }}</a>
              <template v-if="editor">
                <button class="btn ghost" type="button" @click="startRename(c)">{{ t('packs-rename') }}</button>
                <button class="btn ghost danger" type="button" @click="removeCollection(c)">{{ t('packs-delete') }}</button>
              </template>
            </div>
          </div>
        </li>
      </ul>
      <h2>{{ t('packs-packs') }}</h2>
    </section>

    <p v-if="visiblePacks.length === 0" class="empty">{{ t('packs-empty') }}</p>
    <div v-else class="finder" role="search">
      <div class="find">
        <label class="visually-hidden" for="pack-q">{{ t('packs-find') }}</label>
        <input id="pack-q" v-model="query" class="field" type="search" :placeholder="t('packs-find-placeholder')" />
        <label class="visually-hidden" for="pack-sort">{{ t('packs-sort') }}</label>
        <select id="pack-sort" v-model="sort" class="field narrow">
          <option value="created">{{ t('app-sort-created') }}</option>
          <option value="updated">{{ t('app-sort-updated') }}</option>
          <option value="name">{{ t('app-sort-name') }}</option>
          <option value="size">{{ t('app-sort-size') }}</option>
          <option value="assets">{{ t('packs-sort-count') }}</option>
        </select>
        <button type="button" class="btn ghost sort-way" :title="sortWay" :aria-label="sortWay" @click="sortUp = !sortUp">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path v-if="sortUp" d="M5 13V3M2.5 5.5 5 3l2.5 2.5M9.5 4h4M9.5 8h3M9.5 12h1.5" />
            <path v-else d="M5 3v10M2.5 10.5 5 13l2.5-2.5M9.5 4h1.5M9.5 8h3M9.5 12h4" />
          </svg>
        </button>
      </div>
      <div class="filters">
        <div v-if="kindsPresent.length > 1" class="chips" role="group" :aria-label="t('packs-with-kind')">
          <button
            v-for="k in kindsPresent"
            :key="k"
            type="button"
            class="chip"
            :style="{ '--kind': `var(--k-${k})` }"
            :aria-pressed="kind === k"
            @click="kind = kind === k ? '' : k"
          >{{ KIND_LABEL[k] }}</button>
        </div>
        <label class="visually-hidden" for="pack-state">{{ t('packs-state') }}</label>
        <select id="pack-state" v-model="state" class="field narrow">
          <option value="">{{ t('packs-state-any') }}</option>
          <option value="working">{{ t('packs-state-working') }}</option>
          <option value="failed">{{ t('packs-state-failed') }}</option>
          <option value="untagged">{{ t('packs-state-untagged') }}</option>
          <option value="unlicensed">{{ t('packs-state-unlicensed') }}</option>
        </select>
        <label class="visually-hidden" for="pack-store">{{ t('packs-store') }}</label>
        <select id="pack-store" v-model="store" class="field narrow">
          <option value="">{{ t('packs-store-any') }}</option>
          <option v-for="s in stores" :key="s" :value="s">{{ t('packs-store-from', { store: s }) }}</option>
          <option value="none">{{ t('packs-store-none') }}</option>
        </select>
        <span v-if="filtering" class="muted">{{ t('packs-shown-of', { shown: shown.length, total: visiblePacks.length }) }}</span>
        <button v-if="filtering" type="button" class="btn ghost" @click="clearFinding">{{ t('packs-clear') }}</button>
      </div>
    </div>
    <p v-if="visiblePacks.length && !shown.length" class="empty">
      {{ t('packs-no-match') }} <button type="button" class="linkish" @click="clearFinding">{{ t('packs-clear-all') }}</button>
    </p>

    <ul>
      <li v-for="p in rendered" :key="p.id" :class="[`st-${p.status}`, { flash: justReady.has(p.id) }]">
        <button type="button" class="cover checker" :aria-label="t('packs-browse-named', { name: p.name })" tabindex="-1" @click="$emit('browse', p)">
          <img
              v-for="(id, i) in p.preview"
              :key="id"
              :src="thumbUrl({ id })"
              :class="{ glyphs: p.preview_kinds?.[i] === 'font' }"
              alt=""
              loading="lazy"
            />
          <span v-if="!p.preview.length" class="cover-empty">{{ p.name.slice(0, 1) }}</span>
        </button>
        <div class="body">
        <form v-if="renamingPack === p.id" class="rename" @submit.prevent="savePackRename(p)">
          <label class="visually-hidden" :for="`prename-${p.id}`">{{ t('packs-pack-name') }}</label>
          <input :id="`prename-${p.id}`" v-model="packName" v-focus class="field" maxlength="200" @keydown.esc="renamingPack = null" />
          <button class="btn primary" type="submit">{{ t('packs-save') }}</button>
          <button class="btn ghost" type="button" @click="renamingPack = null">{{ t('packs-cancel') }}</button>
        </form>
        <div v-else class="head">
          <button type="button" class="name" @click="$emit('browse', p)">{{ p.name }}</button>
          <span class="status">{{ statusText(p.status) }}</span>
        </div>
        <div
          v-if="p.status === 'uploading' || p.status === 'ingesting' || p.status === 'analyzing'"
          class="bar"
          :class="{ indeterminate: !p.files_total }"
          role="progressbar"
          :aria-valuenow="p.files_total ? progress(p) : undefined"
          aria-valuemin="0"
          aria-valuemax="100"
        >
          <span :style="{ width: `${progress(p)}%` }" />
        </div>
        <div v-if="thumbRun.packId === p.id" class="thumbrun" role="status">
          <template v-if="thumbRun.running">
            <span>
              {{ thumbRun.total ? t('packs-thumbs-drawing-count', { done: thumbRun.done, total: thumbRun.total }) : t('packs-thumbs-drawing') }}
              <span v-if="thumbRun.current" class="dim">{{ thumbRun.current }}</span>
            </span>
            <button class="btn ghost" type="button" @click="cancelThumbRun">{{ t('packs-cancel') }}</button>
            <div class="bar" :class="{ indeterminate: !thumbRun.total }" role="progressbar" aria-valuemin="0" aria-valuemax="100"
              :aria-valuenow="thumbRun.total ? Math.round((thumbRun.done / thumbRun.total) * 100) : undefined">
              <span :style="{ width: `${thumbRun.total ? (thumbRun.done / thumbRun.total) * 100 : 0}%` }" />
            </div>
          </template>
          <template v-else>
            <span v-if="thumbRun.error" class="bad">{{ t('packs-thumbs-stopped', { error: thumbRun.error }) }}</span>
            <span v-else-if="!thumbRun.total">{{ t('packs-thumbs-none') }}</span>
            <span v-else>{{ thumbSummary() }}</span>
            <button class="btn ghost" type="button" @click="thumbRun.packId = ''">{{ t('packs-dismiss') }}</button>
          </template>
        </div>
        <p v-if="p.description" class="desc">{{ p.description }}</p>
        <p v-if="p.error" class="error">{{ p.error }}</p>
        <div class="facts">
          <span>{{ t('packs-assets', { count: p.asset_count }) }}</span>
          <span>{{ humanBytes(p.total_bytes) }}</span>
          <span>{{ p.license_tier ? t('packs-license-tier', { license: p.license ?? t('packs-license-not-found'), tier: p.license_tier }) : t('packs-license', { license: p.license ?? t('packs-license-not-found') }) }}</span>
          <span v-if="p.store">
            {{ t('packs-from-before') }} <a v-if="p.store_url" :href="p.store_url" target="_blank" rel="noopener noreferrer">{{ p.store }}</a><template v-else>{{ p.store }}</template>{{ t('packs-from-after') }}
          </span>
          <span v-if="p.unreal_packages || p.unreal_converted">{{ unrealFact(p) }}</span>
          <span>{{ t('packs-added', { date: formatDate(p.created_at) }) }}</span>
        </div>
        <div class="actions">
          <button class="btn" type="button" @click="$emit('browse', p)">{{ t('packs-browse') }}</button>
          <a class="btn ghost" :href="packZipUrl(p.id)">{{ t('packs-download') }}</a>
          <button v-if="editor" class="btn ghost" type="button" :aria-expanded="editing === p.id" @click="editContext(p)">{{ t('packs-store-details') }}</button>
          <details class="menu">
            <summary class="btn ghost" :aria-label="t('packs-more-named', { name: p.name })">{{ t('packs-more') }}</summary>
            <div class="menu-items" @click="($event.currentTarget as HTMLElement).closest('details')?.removeAttribute('open')">
              <template v-if="editor">
              <button type="button" @click="startPackRename(p)">{{ t('packs-rename-more') }}</button>
              <button type="button" :disabled="p.status !== 'ready' && p.status !== 'failed'" @click="retag(p, false)">
                {{ t('packs-retag-rules') }}
              </button>
              <button v-if="stats?.llm_enabled" type="button" :disabled="p.status !== 'ready'" @click="retag(p, true)">
                {{ t('packs-retag-llm') }}
              </button>
              <button
                v-if="canDrawThumbs() && (!p.kinds || p.kinds.model || p.kinds.animation)"
                type="button"
                :disabled="thumbRun.running || p.status !== 'ready'"
                :title="t('packs-make-thumbs-title')"
                @click="makePackThumbs(p.id, p.name)"
              >
                {{ t('packs-make-thumbs') }}
              </button>
              <button type="button" :disabled="p.status !== 'ready'" @click="readFiles(p)">
                {{ t('packs-read-files') }}
              </button>
              </template>
              <a :href="packCreditsUrl(p.id)" download="CREDITS.md" :title="t('packs-credits-title')">{{ t('packs-credits') }}</a>
              <button v-if="editor && (p.unreal_packages || p.unreal_converted)" type="button" @click="openUnreal(p)">
                {{ t('packs-unreal-open') }}
              </button>
              <button v-if="editor" type="button" class="danger" @click="archive(p)">{{ t('packs-archive') }}</button>
            </div>
          </details>
        </div>
        <form v-if="unrealOpen === p.id && unrealStatus" class="context unreal" @submit.prevent="convertUnreal(p)">
          <p class="hint">{{ t('packs-unreal-intro') }}</p>
          <ul class="ustats">
            <li><strong>{{ unrealStatus.converted }}</strong> {{ unrealStatus.originals_kept ? t('packs-unreal-converted-kept', { count: unrealStatus.converted, kept: unrealStatus.originals_kept }) : t('packs-unreal-converted', { count: unrealStatus.converted }) }}</li>
            <li v-if="unrealStatus.pending"><strong>{{ unrealStatus.pending }}</strong> {{ t('packs-unreal-pending', { count: unrealStatus.pending }) }}</li>
            <li v-if="unrealStatus.needs_editor"><strong>{{ unrealStatus.needs_editor }}</strong> {{ t('packs-unreal-needs-editor', { count: unrealStatus.needs_editor }) }}</li>
            <li v-if="unrealStatus.unsupported"><strong>{{ unrealStatus.unsupported }}</strong> {{ t('packs-unreal-unsupported', { count: unrealStatus.unsupported }) }}</li>
            <li v-if="unrealStatus.failed" class="bad"><strong>{{ unrealStatus.failed }}</strong> {{ t('packs-unreal-failed', { count: unrealStatus.failed }) }}</li>
          </ul>
          <details v-if="unrealStatus.reasons.length">
            <summary>{{ t('packs-unreal-reasons') }}</summary>
            <ul class="reasons">
              <li v-for="[why, n] in unrealStatus.reasons" :key="why"><span class="mono">{{ n }}×</span> {{ why }}</li>
            </ul>
          </details>
          <p v-if="unrealStatus.needs_editor" class="hint">
            {{ t('packs-unreal-editor-before') }}
            <span class="mono">{{ t('packs-unreal-editor-command') }}</span>{{ t('packs-unreal-editor-after') }}
          </p>
          <label class="check">
            <input v-model="unrealOpts.discard_originals" type="checkbox" />
            <span>{{ t('packs-unreal-discard') }}<small>{{ t('packs-unreal-discard-hint') }}</small></span>
          </label>
          <label class="check">
            <input v-model="unrealOpts.drop_unsupported" type="checkbox" />
            <span>{{ t('packs-unreal-drop') }}<small>{{ t('packs-unreal-drop-hint') }}</small></span>
          </label>
          <div class="actions">
            <button class="btn primary" type="submit" :disabled="unrealBusy">{{ t('packs-unreal-convert') }}</button>
            <button class="btn ghost" type="button" @click="unrealOpen = null">{{ t('packs-cancel') }}</button>
          </div>
        </form>
        <PackContextForm
          v-if="editing === p.id"
          :pack="p"
          :llm="!!stats?.llm_enabled"
          @close="editing = null"
          @changed="emit('changed')"
          @error="(m) => emit('error', m)"
        />
        </div>
      </li>
    </ul>
    <div v-if="rendered.length < shown.length" ref="moreEl" class="loading-more" aria-hidden="true" />
  </div>
</template>

<style scoped>
.packs { padding: 1.25rem 1.5rem 3rem; max-width: 900px; }
.summary { margin: 0 0 1.25rem; color: var(--muted); font-size: var(--step--1); max-width: 75ch; }
.bad { color: var(--danger); }
.empty { color: var(--muted); }
ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
li { padding: 1rem 0; border-top: 1px solid var(--line); display: flex; gap: 1rem; align-items: flex-start; }
.body { display: flex; flex-direction: column; gap: 0.45rem; flex: 1; min-width: 0; }
.cover {
  flex: none; width: 96px; height: 96px; padding: 0; border: 1px solid var(--line); border-radius: var(--r-tile);
  overflow: hidden; cursor: pointer; display: grid; grid-template-columns: 1fr 1fr; grid-auto-rows: 1fr; gap: 1px;
  background-color: var(--line);
}
.cover:hover { border-color: var(--muted); }
.cover img { width: 100%; height: 100%; object-fit: cover; background: var(--check-a); }
.cover img:only-child { grid-column: 1 / -1; grid-row: 1 / -1; object-fit: contain; }
.cover img:first-child:nth-last-child(2), .cover img:first-child:nth-last-child(2) ~ img { grid-row: 1 / -1; }
.cover img:first-child:nth-last-child(3) { grid-row: 1 / -1; }
.cover-empty {
  grid-column: 1 / -1; grid-row: 1 / -1; display: grid; place-items: center; background: var(--panel);
  font-size: 2rem; font-weight: 700; color: var(--faint); font-variation-settings: 'CASL' 1;
}
.colsec h2 { margin: 0.5rem 0 0.25rem; font-size: var(--step-0); font-weight: 650; color: var(--muted); }
.colsec h2 + .cols { margin-bottom: 1.5rem; }
.cols li { padding: 0.75rem 0; }
.cols .cover { width: 72px; height: 72px; }
.rename { display: flex; gap: 0.4rem; }
.rename .field { max-width: 20rem; }
.menu { position: relative; }
.menu summary { list-style: none; }
.menu summary::-webkit-details-marker { display: none; }
.menu-items {
  position: absolute; top: calc(100% + 4px); left: 0; z-index: 5; min-width: 13rem; padding: 0.3rem;
  display: flex; flex-direction: column; background: var(--raised); border: 1px solid var(--line);
  border-radius: var(--r-tile); box-shadow: var(--elev-3); transform-origin: top left;
}
.menu[open] .menu-items { animation: menu-in var(--dur-3) var(--ease-pop); }
@keyframes menu-in { from { opacity: 0; scale: 0.95; translate: 0 -4px; } }
.menu-items button, .menu-items a {
  border: 0; background: none; text-align: left; padding: 0.45rem 0.6rem; border-radius: var(--r-chip);
  cursor: pointer; font-size: var(--step--1);
}
.menu-items a { color: inherit; text-decoration: none; }
.menu-items button, .menu-items a { transition: background-color var(--dur-1); }
.menu-items button:hover:not(:disabled), .menu-items a:hover { background: var(--panel); }
.menu-items button:disabled { opacity: 0.5; cursor: default; }
.menu-items .danger { color: var(--danger); }
.finder { display: grid; gap: 0.6rem; margin-bottom: 1rem; }
.finder .find { margin: 0; flex-wrap: nowrap; }
.finder .find .field { flex: 1 1 auto; max-width: 30rem; }
.finder .find .narrow, .filters .narrow { flex: none; width: auto; }
.finder .sort-way { flex: none; padding: 0.4em; color: var(--muted); }
.filters { display: flex; flex-wrap: wrap; align-items: center; gap: 0.5rem; }
.chips { display: flex; flex-wrap: wrap; gap: 0.35rem; }
.chip {
  padding: 0.2rem 0.65rem; border: 1px solid var(--line); border-radius: 999px; background: none; cursor: pointer;
  font-size: var(--step--1); color: var(--muted);
}
.chip::before {
  content: ''; display: inline-block; width: 0.5rem; height: 0.5rem; margin-right: 0.4rem; border-radius: 50%;
  background: var(--kind); vertical-align: 0.05em;
}
.chip:hover { border-color: var(--muted); color: var(--text); }
.chip[aria-pressed='true'] { border-color: var(--kind); color: var(--text); background: color-mix(in srgb, var(--kind) 14%, transparent); }
.linkish { border: 0; background: none; padding: 0; color: var(--accent); cursor: pointer; text-decoration: underline; font: inherit; }
.loading-more { height: 1px; }
.muted { color: var(--muted); font-size: var(--step--1); white-space: nowrap; }
@media (max-width: 560px) {
  .packs { padding: 1rem 0.75rem 3rem; }
  .cover { width: 64px; height: 64px; }
  li { gap: 0.75rem; }
}
.head { display: flex; align-items: baseline; gap: 0.75rem; }
.name {
  border: 0; background: none; padding: 0; cursor: pointer; text-align: left;
  font-size: var(--step-1); font-weight: 650; font-variation-settings: 'CASL' 0.4;
}
.name:hover { text-decoration: underline; }
.status { font-size: var(--step--1); color: var(--muted); }
.st-ready .status { color: var(--ok); display: inline-block; }
.st-failed .status { color: var(--danger); }
.bar { height: 4px; background: var(--raised); border-radius: 2px; overflow: hidden; }
.bar span {
  position: relative; display: block; height: 100%; background: var(--k-texture); border-radius: inherit; overflow: hidden;
  transition: width 0.5s var(--ease-out);
}
/* A sheen sweeps along the filled part so a slow import still looks alive. */
.bar span::after {
  content: ''; position: absolute; inset: 0; translate: -100% 0;
  background: linear-gradient(90deg, transparent, color-mix(in srgb, white 45%, transparent), transparent);
  animation: sheen 1.6s var(--ease-in-out) infinite;
}
@keyframes sheen { to { translate: 100% 0; } }
/* No file count yet: a segment slides back and forth instead of a bar stuck at zero. */
.bar.indeterminate span { width: 35% !important; animation: indeterminate 1.3s var(--ease-in-out) infinite alternate; }
.bar.indeterminate span::after { display: none; }
@keyframes indeterminate { from { translate: -10% 0; } to { translate: 195% 0; } }
li.flash { animation: ready-flash 1.6s var(--ease-out); }
li.flash .status { animation: ready-pop var(--dur-3) var(--ease-pop); }
@keyframes ready-flash { from { background: color-mix(in srgb, var(--ok) 16%, transparent); } }
@keyframes ready-pop { from { scale: 0.6; opacity: 0; } }
.desc { margin: 0; color: var(--muted); max-width: 70ch; }
.error { margin: 0; color: var(--danger); font-size: var(--step--1); }
.facts { display: flex; flex-wrap: wrap; gap: 0.25rem 1.25rem; font-size: var(--step--1); color: var(--muted); }
.thumbrun {
  display: flex; flex-wrap: wrap; align-items: center; gap: 0.35rem 0.75rem; padding: 0.45rem 0.6rem;
  border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--raised); font-size: var(--step--1);
}
.thumbrun > span { flex: 1; min-width: 0; }
.thumbrun .dim { display: block; color: var(--faint); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.thumbrun .bar { flex-basis: 100%; }
.thumbrun .bad { color: var(--danger); }
.actions { display: flex; flex-wrap: wrap; gap: 0.4rem; }
.facts a { color: inherit; }
.context {
  display: flex; flex-direction: column; gap: 0.7rem; margin-top: 0.3rem; padding: 0.9rem 1rem;
  border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--raised);
}
.context label { display: flex; flex-direction: column; gap: 0.3rem; font-size: var(--step--1); color: var(--muted); }
.hint { margin: 0; font-size: var(--step--1); color: var(--muted); max-width: 70ch; }
.find { display: flex; gap: 0.5rem; flex-wrap: wrap; }
.find .field { flex: 1 1 14rem; min-width: 0; }
.ustats { flex-direction: row; flex-wrap: wrap; gap: 0.3rem 1.25rem; font-size: var(--step--1); color: var(--muted); }
.ustats li, .reasons li { padding: 0; border: 0; display: block; }
.ustats strong { color: var(--text); font-variant-numeric: tabular-nums; }
.reasons { gap: 0.2rem; margin-top: 0.4rem; font-size: var(--step--1); color: var(--muted); }
.context .check { flex-direction: row; align-items: flex-start; gap: 0.55rem; color: var(--text); cursor: pointer; }
.check input { margin-top: 0.2rem; }
.check small { display: block; color: var(--muted); }
</style>
