<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import {
  api, collectionZipUrl, humanBytes, KIND_LABEL, packZipUrl, thumbUrl, type AssetKind, type Collection, type Pack,
  type PackContext, type Stats, type StoreMatch, type StoreMatches, type UnrealPackStatus,
} from '../api'
import { confirmDialog, undoable } from '../ui'
import { archiveWithUndo } from '../archive'
import { canDrawThumbs, cancelThumbRun, makePackThumbs, thumbRun } from '../browserThumbs'

const props = defineProps<{ packs: Pack[]; stats: Stats | null; collections: Collection[] }>()

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
const state = ref<'' | 'working' | 'failed' | 'untagged'>('')
/** A store's name, `none` for packs with no store page, or empty for any. */
const store = ref('')
const sort = ref<'new' | 'old' | 'name' | 'size' | 'assets'>('new')
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
    if (store.value === 'none' ? !!p.store_url : store.value && p.store !== store.value) return false
    if (!words.length) return true
    const hay = [p.name, p.store, p.license, p.license_tier, p.description, ...p.tags].filter(Boolean).join(' ').toLowerCase()
    return words.every((w) => hay.includes(w))
  })
  const by: Record<typeof sort.value, (a: Pack, b: Pack) => number> = {
    new: (a, b) => b.created_at.localeCompare(a.created_at),
    old: (a, b) => a.created_at.localeCompare(b.created_at),
    name: (a, b) => a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' }),
    size: (a, b) => b.total_bytes - a.total_bytes,
    assets: (a, b) => b.asset_count - a.asset_count,
  }
  return list.sort(by[sort.value])
})

// Rows go into the page a batch at a time as the list scrolls, so a library of thousands of packs opens at once.
const BATCH = 40
const limit = ref(BATCH)
watch([query, kind, state, store, sort], () => (limit.value = BATCH))
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
    emit('error', `Couldn't rename: ${(e as Error).message}`)
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
    emit('error', `Couldn't rename: ${(e as Error).message}`)
  }
}
function removeCollection(c: Collection) {
  undoable(`Deleted the collection ${c.name}`, {
    apply: () => hidden.add(c.id),
    revert: () => hidden.delete(c.id),
    commit: async () => {
      try {
        await api.deleteCollection(c.id)
        emit('changed')
      } catch (e) {
        hidden.delete(c.id)
        emit('error', `Couldn't delete: ${(e as Error).message}`)
      }
    },
  })
}

const statusText: Record<Pack['status'], string> = {
  uploading: 'Uploading (files received so far)',
  ingesting: 'Unpacking and tagging',
  analyzing: 'LLM is tagging',
  ready: 'Ready',
  failed: 'Failed',
}

function progress(p: Pack): number {
  return p.files_total ? Math.min(100, Math.round((p.files_done / p.files_total) * 100)) : 0
}

async function retag(p: Pack, llm: boolean) {
  try {
    await api.retagPack(p.id, llm ? { rules: false, llm: true } : { rules: true, llm: false })
    emit('changed')
  } catch (e) {
    emit('error', `Couldn't start retagging: ${(e as Error).message}`)
  }
}

// Pack context editor: one pack open at a time.
const editing = ref<string | null>(null)
const saving = ref(false)
const draft = reactive<PackContext>({ store: null, store_url: null, store_text: null, license_tier: null, readme: null })

async function editContext(p: Pack) {
  if (editing.value === p.id) {
    editing.value = null
    return
  }
  try {
    Object.assign(draft, await api.packContext(p.id))
    editing.value = p.id
    Object.assign(lookup, { q: '', result: null, error: '', open: !draft.store_url })
    // No store page yet: look for one straight away.
    if (!draft.store_url) findInStores(p)
  } catch (e) {
    emit('error', `Couldn't load the pack's store details: ${(e as Error).message}`)
  }
}

// Store lookup: the pack's name searched in the stores, best matches first.
const lookup = reactive({ q: '', loading: false, open: false, error: '', result: null as StoreMatches | null })
let lookupAsked = 0

async function findInStores(p: Pack, q?: string) {
  const n = ++lookupAsked
  lookup.loading = true
  lookup.open = true
  lookup.error = ''
  try {
    const r = await api.storeMatches(p.id, q?.trim() || undefined)
    if (n !== lookupAsked || editing.value !== p.id) return
    lookup.result = r
    if (!lookup.q) lookup.q = r.query
  } catch (e) {
    if (n === lookupAsked) lookup.error = (e as Error).message
  } finally {
    if (n === lookupAsked) lookup.loading = false
  }
}

/** Fill the form from a match; Save fetches the page's description. */
function useMatch(m: StoreMatch) {
  draft.store = m.store
  draft.store_url = m.url
  draft.store_text = null
  lookup.open = false
}

async function saveContext(p: Pack, thenRetag: boolean) {
  saving.value = true
  try {
    const { readme: _readme, ...ctx } = draft
    Object.assign(draft, await api.setPackContext(p.id, ctx))
    if (thenRetag) await api.retagPack(p.id, { rules: false, llm: true })
    editing.value = null
    emit('changed')
  } catch (e) {
    emit('error', `Couldn't save store details: ${(e as Error).message}`)
  } finally {
    saving.value = false
  }
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
    emit('error', `Couldn't load the Unreal conversion status: ${(e as Error).message}`)
  }
}

async function convertUnreal(p: Pack) {
  if (unrealOpts.discard_originals && !(await confirmDialog({
    title: 'Discard the originals?',
    body: `The original .uasset files in ${p.name} go once converted. Unreal-only data in them, such as material graphs, is lost for good.`,
    confirm: 'Convert and discard',
    danger: true,
  }))) return
  unrealBusy.value = true
  try {
    await api.unrealConvert(p.id, { ...unrealOpts })
    unrealOpen.value = null
    emit('changed')
  } catch (e) {
    emit('error', `Couldn't start the conversion: ${(e as Error).message}`)
  } finally {
    unrealBusy.value = false
  }
}

function unrealFact(p: Pack): string {
  const left = p.unreal_packages ?? 0
  const done = p.unreal_converted ?? 0
  if (!left) return `${done.toLocaleString()} converted from Unreal`
  if (!done) return `${left.toLocaleString()} Unreal ${left === 1 ? 'package' : 'packages'}`
  return `${done.toLocaleString()} converted from Unreal, ${left.toLocaleString()} still packages`
}

/** To the archive (Undo on the toast brings it back); it's deleted for good from there. */
async function archive(p: Pack) {
  hidden.add(p.id)
  const ok = await archiveWithUndo(`Archived ${p.name} and its ${p.asset_count.toLocaleString()} assets`, { packs: [p.id] }, () => {
    hidden.delete(p.id)
    emit('changed')
  })
  if (!ok) hidden.delete(p.id)
}
</script>

<template>
  <div class="packs">
    <p v-if="stats?.blobs_bad" class="summary">
      <strong class="bad">{{ stats.blobs_bad }} files failed their last integrity check.</strong>
    </p>

    <section v-if="visibleCollections.length" class="colsec" aria-label="Collections">
      <h2>Collections</h2>
      <ul class="cols">
        <li v-for="c in visibleCollections" :key="c.id">
          <button type="button" class="cover checker" :aria-label="`Browse ${c.name}`" tabindex="-1" @click="$emit('browseCollection', c)">
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
              <label class="visually-hidden" :for="`rename-${c.id}`">Collection name</label>
              <input :id="`rename-${c.id}`" v-model="nameDraft" class="field" @keydown.esc="renaming = null" />
              <button class="btn primary" type="submit">Save</button>
              <button class="btn ghost" type="button" @click="renaming = null">Cancel</button>
            </form>
            <button v-else type="button" class="name" @click="$emit('browseCollection', c)">{{ c.name }}</button>
            <p v-if="c.description" class="desc">{{ c.description }}</p>
            <div class="facts">
              <span>{{ c.asset_count.toLocaleString() }} {{ c.asset_count === 1 ? 'asset' : 'assets' }}</span>
              <span>{{ humanBytes(c.total_bytes) }}</span>
              <span>Updated {{ new Date(c.updated_at).toLocaleDateString() }}</span>
            </div>
            <div class="actions">
              <button class="btn" type="button" @click="$emit('browseCollection', c)">Browse</button>
              <a v-if="c.asset_count" class="btn ghost" :href="collectionZipUrl(c.id)">Download (.zip)</a>
              <button class="btn ghost" type="button" @click="startRename(c)">Rename</button>
              <button class="btn ghost danger" type="button" @click="removeCollection(c)">Delete</button>
            </div>
          </div>
        </li>
      </ul>
      <h2>Packs</h2>
    </section>

    <p v-if="visiblePacks.length === 0" class="empty">No packs yet. Drop a zip or a folder anywhere on this page to add one.</p>
    <div v-else class="finder" role="search">
      <div class="find">
        <label class="visually-hidden" for="pack-q">Find a pack</label>
        <input id="pack-q" v-model="query" class="field" type="search" placeholder="Find a pack by name, store, license or tag" />
        <label class="visually-hidden" for="pack-sort">Order</label>
        <select id="pack-sort" v-model="sort" class="field narrow">
          <option value="new">Newest first</option>
          <option value="old">Oldest first</option>
          <option value="name">By name</option>
          <option value="size">Biggest first</option>
          <option value="assets">Most assets first</option>
        </select>
      </div>
      <div class="filters">
        <div v-if="kindsPresent.length > 1" class="chips" role="group" aria-label="Packs with">
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
        <label class="visually-hidden" for="pack-state">Status</label>
        <select id="pack-state" v-model="state" class="field narrow">
          <option value="">Any status</option>
          <option value="working">Working on</option>
          <option value="failed">Failed</option>
          <option value="untagged">Not described yet</option>
        </select>
        <label class="visually-hidden" for="pack-store">Store</label>
        <select id="pack-store" v-model="store" class="field narrow">
          <option value="">Any store</option>
          <option v-for="s in stores" :key="s" :value="s">From {{ s }}</option>
          <option value="none">No store page</option>
        </select>
        <span v-if="filtering" class="muted">{{ shown.length.toLocaleString() }} of {{ visiblePacks.length.toLocaleString() }}</span>
        <button v-if="filtering" type="button" class="btn ghost" @click="clearFinding">Clear</button>
      </div>
    </div>
    <p v-if="visiblePacks.length && !shown.length" class="empty">
      No packs match. <button type="button" class="linkish" @click="clearFinding">Clear the search and filters</button>
    </p>

    <ul>
      <li v-for="p in rendered" :key="p.id" :class="[`st-${p.status}`, { flash: justReady.has(p.id) }]">
        <button type="button" class="cover checker" :aria-label="`Browse ${p.name}`" tabindex="-1" @click="$emit('browse', p)">
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
          <label class="visually-hidden" :for="`prename-${p.id}`">Pack name</label>
          <input :id="`prename-${p.id}`" v-model="packName" v-focus class="field" maxlength="200" @keydown.esc="renamingPack = null" />
          <button class="btn primary" type="submit">Save</button>
          <button class="btn ghost" type="button" @click="renamingPack = null">Cancel</button>
        </form>
        <div v-else class="head">
          <button type="button" class="name" @click="$emit('browse', p)">{{ p.name }}</button>
          <span class="status">{{ statusText[p.status] }}</span>
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
              Drawing thumbnails in this browser<template v-if="thumbRun.total">: {{ thumbRun.done }} of {{ thumbRun.total }}</template>…
              <span v-if="thumbRun.current" class="dim">{{ thumbRun.current }}</span>
            </span>
            <button class="btn ghost" type="button" @click="cancelThumbRun">Cancel</button>
            <div class="bar" :class="{ indeterminate: !thumbRun.total }" role="progressbar" aria-valuemin="0" aria-valuemax="100"
              :aria-valuenow="thumbRun.total ? Math.round((thumbRun.done / thumbRun.total) * 100) : undefined">
              <span :style="{ width: `${thumbRun.total ? (thumbRun.done / thumbRun.total) * 100 : 0}%` }" />
            </div>
          </template>
          <template v-else>
            <span v-if="thumbRun.error" class="bad">Stopped: {{ thumbRun.error }}</span>
            <span v-else-if="!thumbRun.total">Every model here already has a thumbnail drawn with all its textures.</span>
            <span v-else>
              {{ thumbRun.done < thumbRun.total ? `Cancelled after ${thumbRun.done} of ${thumbRun.total}` : `Drew ${thumbRun.total} ${thumbRun.total === 1 ? 'model' : 'models'}` }}:
              {{ thumbRun.improved }} {{ thumbRun.improved === 1 ? 'thumbnail' : 'thumbnails' }} replaced<template v-if="thumbRun.failed">, {{ thumbRun.failed }} couldn't be drawn</template>.
            </span>
            <button class="btn ghost" type="button" @click="thumbRun.packId = ''">Dismiss</button>
          </template>
        </div>
        <p v-if="p.description" class="desc">{{ p.description }}</p>
        <p v-if="p.error" class="error">{{ p.error }}</p>
        <div class="facts">
          <span>{{ p.asset_count.toLocaleString() }} assets</span>
          <span>{{ humanBytes(p.total_bytes) }}</span>
          <span>License: {{ p.license ?? 'not found' }}{{ p.license_tier ? ` (${p.license_tier})` : '' }}</span>
          <span v-if="p.store">
            From <a v-if="p.store_url" :href="p.store_url" target="_blank" rel="noopener noreferrer">{{ p.store }}</a><template v-else>{{ p.store }}</template>
          </span>
          <span v-if="p.unreal_packages || p.unreal_converted">{{ unrealFact(p) }}</span>
          <span>Added {{ new Date(p.created_at).toLocaleDateString() }}</span>
        </div>
        <div class="actions">
          <button class="btn" type="button" @click="$emit('browse', p)">Browse</button>
          <a class="btn ghost" :href="packZipUrl(p.id)">Download pack (.zip)</a>
          <button class="btn ghost" type="button" :aria-expanded="editing === p.id" @click="editContext(p)">Store details</button>
          <details class="menu">
            <summary class="btn ghost" :aria-label="`More actions for ${p.name}`">More</summary>
            <div class="menu-items" @click="($event.currentTarget as HTMLElement).closest('details')?.removeAttribute('open')">
              <button type="button" @click="startPackRename(p)">Rename…</button>
              <button type="button" :disabled="p.status !== 'ready' && p.status !== 'failed'" @click="retag(p, false)">
                Re-run offline tagger
              </button>
              <button v-if="stats?.llm_enabled" type="button" :disabled="p.status !== 'ready'" @click="retag(p, true)">
                Re-run LLM tagger
              </button>
              <button
                v-if="canDrawThumbs() && (!p.kinds || p.kinds.model || p.kinds.animation)"
                type="button"
                :disabled="thumbRun.running || p.status !== 'ready'"
                title="Open each model in a 3D viewer out of sight, one at a time, and use its picture as the thumbnail where that's better than the server's"
                @click="makePackThumbs(p.id, p.name)"
              >
                Make thumbnails in this browser
              </button>
              <button v-if="p.unreal_packages || p.unreal_converted" type="button" @click="openUnreal(p)">
                Convert Unreal content…
              </button>
              <button type="button" class="danger" @click="archive(p)">Archive pack</button>
            </div>
          </details>
        </div>
        <form v-if="unrealOpen === p.id && unrealStatus" class="context unreal" @submit.prevent="convertUnreal(p)">
          <p class="hint">
            Hoard reads Unreal packages itself and stores what's inside as ordinary files: textures as PNG or EXR,
            sounds as WAV, materials as JSON. The converted file takes the package's place, keeping its tags, links
            and collections.
          </p>
          <ul class="ustats">
            <li><strong>{{ unrealStatus.converted }}</strong> converted<template v-if="unrealStatus.originals_kept">, {{ unrealStatus.originals_kept }} with the original kept</template></li>
            <li v-if="unrealStatus.pending"><strong>{{ unrealStatus.pending }}</strong> not looked at yet</li>
            <li v-if="unrealStatus.needs_editor"><strong>{{ unrealStatus.needs_editor }}</strong> waiting for Unreal Editor</li>
            <li v-if="unrealStatus.unsupported"><strong>{{ unrealStatus.unsupported }}</strong> with no common format</li>
            <li v-if="unrealStatus.failed" class="bad"><strong>{{ unrealStatus.failed }}</strong> failed</li>
          </ul>
          <details v-if="unrealStatus.reasons.length">
            <summary>Why packages are left</summary>
            <ul class="reasons">
              <li v-for="[why, n] in unrealStatus.reasons" :key="why"><span class="mono">{{ n }}×</span> {{ why }}</li>
            </ul>
          </details>
          <p v-if="unrealStatus.needs_editor" class="hint">
            Meshes and animations need Unreal Editor. On a machine with Unreal installed, run
            <span class="mono">hoard unreal-worker --editor &lt;path to UnrealEditor-Cmd&gt;</span>; it exports
            them as FBX and uploads the results here.
          </p>
          <label class="check">
            <input v-model="unrealOpts.discard_originals" type="checkbox" />
            <span>Discard the original .uasset files once converted<small>Frees the space. Unreal-only data, such as material graphs, is lost.</small></span>
          </label>
          <label class="check">
            <input v-model="unrealOpts.drop_unsupported" type="checkbox" />
            <span>Delete packages with no common format<small>Blueprints, levels and other engine-only assets.</small></span>
          </label>
          <div class="actions">
            <button class="btn primary" type="submit" :disabled="unrealBusy">Convert</button>
            <button class="btn ghost" type="button" @click="unrealOpen = null">Cancel</button>
          </div>
        </form>
        <form v-if="editing === p.id" class="context" @submit.prevent="saveContext(p, false)">
          <p class="hint">
            Where the pack came from and what the seller says about it. The LLM tagger reads this with every file,
            so a store blurb like "120 ice and fire spells" helps it name <span class="mono">spell35.wav</span>.
          </p>
          <section v-if="lookup.open" class="lookup" :aria-busy="lookup.loading">
            <div class="find">
              <input
                v-model="lookup.q"
                class="field"
                type="search"
                aria-label="Search the stores for"
                placeholder="The pack's name in the store"
                @keydown.enter.prevent="findInStores(p, lookup.q)"
              />
              <button class="btn" type="button" :disabled="lookup.loading" @click="findInStores(p, lookup.q)">
                {{ lookup.loading ? 'Searching…' : 'Search stores' }}
              </button>
              <button class="btn ghost" type="button" @click="lookup.open = false">Hide</button>
            </div>
            <p v-if="lookup.error" class="err">{{ lookup.error }}</p>
            <template v-else-if="lookup.result">
              <p v-if="lookup.result.clues.length || lookup.result.also.length" class="hint">
                <template v-if="lookup.result.clues.length">From the pack’s files: {{ lookup.result.clues.join(' · ') }}. </template>
                <template v-if="lookup.result.also.length">Also searched {{ lookup.result.also.map((q) => `“${q}”`).join(', ') }}.</template>
              </p>
              <p v-if="!lookup.result.matches.length" class="hint">
                Nothing on Fab, itch.io, Kenney or OpenGameArt looks like it. Try fewer or different words.
              </p>
              <ul v-else class="matches">
                <li v-for="m in lookup.result.matches" :key="m.url">
                  <img v-if="m.thumb" :src="m.thumb" alt="" loading="lazy" referrerpolicy="no-referrer" />
                  <span v-else class="noimg" aria-hidden="true" />
                  <div class="what">
                    <a :href="m.url" target="_blank" rel="noopener noreferrer" class="title">{{ m.title }}</a>
                    <span class="meta">{{ [m.store, m.author, m.price].filter(Boolean).join(' · ') }}</span>
                    <span v-if="m.summary" class="meta clamp">{{ m.summary }}</span>
                    <span v-for="w in m.why ?? []" :key="w" class="why">{{ w }}</span>
                  </div>
                  <button class="btn" type="button" @click="useMatch(m)">Use</button>
                </li>
              </ul>
              <p class="hint">
                <template v-if="lookup.result.failed.length">Couldn't reach {{ lookup.result.failed.join(', ') }} this time. </template>
                Also look on
                <template v-for="(l, i) in lookup.result.links" :key="l.store">
                  <a :href="l.url" target="_blank" rel="noopener noreferrer">{{ l.store }}</a>{{ i < lookup.result.links.length - 1 ? ' and ' : '' }}
                </template>
                (they can only be searched in your browser); paste the page's address below.
              </p>
            </template>
          </section>
          <button v-else class="btn ghost find-again" type="button" @click="lookup.result ? (lookup.open = true) : findInStores(p)">
            Find in stores…
          </button>
          <div class="row">
            <label>Store <input v-model="draft.store" class="field" placeholder="Fab, itch.io, Humble Bundle…" /></label>
            <label>License tier <input v-model="draft.license_tier" class="field" placeholder="Personal, Professional…" /></label>
          </div>
          <label>Store page
            <input v-model="draft.store_url" class="field" type="url" placeholder="https://…" />
          </label>
          <label>Description from the store
            <textarea v-model="draft.store_text" class="field" rows="5" placeholder="Paste the listing text, or leave empty to fetch it from the store page." />
          </label>
          <p v-if="draft.store_url && !draft.store_text" class="hint">Saving fetches the description from the store page.</p>
          <details v-if="draft.readme">
            <summary>README found in the pack</summary>
            <pre>{{ draft.readme }}</pre>
          </details>
          <div class="actions">
            <button class="btn primary" type="submit" :disabled="saving">Save</button>
            <button v-if="stats?.llm_enabled" class="btn" type="button" :disabled="saving || p.status !== 'ready'" @click="saveContext(p, true)">
              Save and re-run LLM tagger
            </button>
            <button class="btn ghost" type="button" @click="editing = null">Cancel</button>
          </div>
        </form>
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
.menu-items button {
  border: 0; background: none; text-align: left; padding: 0.45rem 0.6rem; border-radius: var(--r-chip);
  cursor: pointer; font-size: var(--step--1);
}
.menu-items button { transition: background-color var(--dur-1); }
.menu-items button:hover:not(:disabled) { background: var(--panel); }
.menu-items button:disabled { opacity: 0.5; cursor: default; }
.menu-items .danger { color: var(--danger); }
.finder { display: grid; gap: 0.6rem; margin-bottom: 1rem; }
.finder .find { margin: 0; flex-wrap: nowrap; }
.finder .find .field { flex: 1 1 auto; max-width: 30rem; }
.finder .find .narrow, .filters .narrow { flex: none; width: auto; }
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
.context .row { display: grid; grid-template-columns: 1fr 1fr; gap: 0.7rem; }
.context textarea { resize: vertical; font: inherit; }
.hint { margin: 0; font-size: var(--step--1); color: var(--muted); max-width: 70ch; }
.context pre { white-space: pre-wrap; font-size: 0.8rem; max-height: 14rem; overflow: auto; margin: 0.4rem 0 0; color: var(--muted); }
.context summary { cursor: pointer; font-size: var(--step--1); color: var(--muted); }
@media (max-width: 560px) { .context .row { grid-template-columns: 1fr; } }
.lookup { display: grid; gap: 0.5rem; padding: 0.7rem; border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--bg); }
.lookup[aria-busy='true'] .matches { opacity: 0.6; }
.find { display: flex; gap: 0.5rem; flex-wrap: wrap; }
.find .field { flex: 1 1 14rem; min-width: 0; }
.lookup .err { margin: 0; color: var(--danger); font-size: var(--step--1); }
.lookup .hint { margin: 0; }
.matches { gap: 0; max-height: 24rem; overflow-y: auto; }
.matches li { padding: 0.5rem 0; gap: 0.7rem; align-items: center; }
.matches li:first-child { border-top: 0; }
.matches img, .matches .noimg {
  flex: none; width: 64px; height: 40px; object-fit: cover; border-radius: var(--r-chip); background: var(--raised);
}
.matches .what { display: flex; flex-direction: column; gap: 0.1rem; flex: 1; min-width: 0; }
.matches .title { color: var(--text); font-size: var(--step--1); font-weight: 600; overflow-wrap: anywhere; }
.matches .meta { color: var(--muted); font-size: 0.75rem; overflow-wrap: anywhere; }
.matches .why { color: var(--ok, var(--accent)); font-size: 0.75rem; }
.matches .why::before { content: '✓ '; }
.matches .clamp { display: -webkit-box; -webkit-line-clamp: 1; line-clamp: 1; -webkit-box-orient: vertical; overflow: hidden; }
.find-again { align-self: flex-start; }
.ustats { flex-direction: row; flex-wrap: wrap; gap: 0.3rem 1.25rem; font-size: var(--step--1); color: var(--muted); }
.ustats li, .reasons li { padding: 0; border: 0; display: block; }
.ustats strong { color: var(--text); font-variant-numeric: tabular-nums; }
.reasons { gap: 0.2rem; margin-top: 0.4rem; font-size: var(--step--1); color: var(--muted); }
.context .check { flex-direction: row; align-items: flex-start; gap: 0.55rem; color: var(--text); cursor: pointer; }
.check input { margin-top: 0.2rem; }
.check small { display: block; color: var(--muted); }
</style>
