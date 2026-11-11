<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import {
  api, collectionZipUrl, humanBytes, packZipUrl, thumbUrl, type Collection, type Pack, type PackContext, type Stats,
  type UnrealPackStatus,
} from '../api'

const props = defineProps<{ packs: Pack[]; stats: Stats | null; collections: Collection[] }>()

// Close an open "More" menu when clicking anywhere else.
function closeMenus(e: MouseEvent) {
  for (const d of document.querySelectorAll<HTMLDetailsElement>('.packs details.menu[open]')) {
    if (!d.contains(e.target as Node)) d.removeAttribute('open')
  }
}
onMounted(() => document.addEventListener('click', closeMenus))
onBeforeUnmount(() => document.removeEventListener('click', closeMenus))

const query = ref('')
/** Audio conversions use ffmpeg; say so while it's being fetched, or why it can't be. */
const ffmpegNote = computed(() => {
  const f = props.stats?.ffmpeg ?? ''
  if (f === 'downloading' || f === 'looking') return 'Fetching ffmpeg for audio conversions.'
  if (f.startsWith('unavailable')) return `ffmpeg ${f}; MP3 and Opus use the built-in encoders.`
  return ''
})
const shown = computed(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) return props.packs
  return props.packs.filter((p) =>
    [p.name, p.store, p.license, p.description, ...p.tags].some((s) => s?.toLowerCase().includes(q)),
  )
})
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
async function removeCollection(c: Collection) {
  if (!confirm(`Delete the collection "${c.name}"? The ${c.asset_count} assets in it stay in the library.`)) return
  try {
    await api.deleteCollection(c.id)
    emit('changed')
  } catch (e) {
    emit('error', `Couldn't delete: ${(e as Error).message}`)
  }
}

const statusText: Record<Pack['status'], string> = {
  uploading: 'Uploading (files received so far)',
  ingesting: 'Unpacking and tagging',
  analyzing: 'LLM is tagging',
  ready: 'Ready',
  failed: 'Failed',
}

function summary(s: Stats): string {
  const plural = (n: number, w: string) => `${n.toLocaleString()} ${w}${n === 1 ? '' : 's'}`
  const saved = s.logical_bytes - s.stored_bytes
  // Thumbnails also count as stored bytes, so only mention savings when dedup clearly won.
  const dedup = saved > 0 ? `, ${humanBytes(saved)} saved by deduplication` : ''
  const dupes = s.duplicate_assets ? ` ${plural(s.duplicate_assets, 'file')} appear in more than one pack.` : ''
  return `${plural(s.packs, 'pack')} and ${plural(s.assets, 'asset')} in ${humanBytes(s.stored_bytes)} of storage${dedup}.${dupes}`
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
  } catch (e) {
    emit('error', `Couldn't load the pack's store details: ${(e as Error).message}`)
  }
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
  if (unrealOpts.discard_originals
    && !confirm(`Discard the original .uasset files in "${p.name}" once converted? Unreal-only data in them is lost for good.`)) return
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

async function remove(p: Pack) {
  if (!confirm(`Delete "${p.name}" and its ${p.asset_count} assets from the library? Files shared with other packs stay.`)) return
  try {
    await api.deletePack(p.id)
    emit('changed')
  } catch (e) {
    emit('error', `Couldn't delete: ${(e as Error).message}`)
  }
}
</script>

<template>
  <div class="packs">
    <p v-if="stats" class="summary">
      {{ summary(stats) }} Files live in <span class="mono">{{ stats.storage }}</span>.
      <span v-if="stats.version">Hoard {{ stats.version }}.</span>
      <span v-if="ffmpegNote">{{ ffmpegNote }}</span>
      <strong v-if="stats.blobs_bad" class="bad">{{ stats.blobs_bad }} files failed their last integrity check.</strong>
    </p>

    <section v-if="collections.length" class="colsec" aria-label="Collections">
      <h2>Collections</h2>
      <ul class="cols">
        <li v-for="c in collections" :key="c.id">
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

    <p v-if="packs.length === 0" class="empty">No packs yet. Drop a zip or a folder anywhere on this page to add one.</p>
    <div v-else-if="packs.length > 5" class="find">
      <label class="visually-hidden" for="pack-q">Find a pack</label>
      <input id="pack-q" v-model="query" class="field" type="search" placeholder="Find a pack by name, store, license or tag" />
      <span v-if="query" class="muted">{{ shown.length }} of {{ packs.length }}</span>
    </div>

    <ul>
      <li v-for="p in shown" :key="p.id" :class="`st-${p.status}`">
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
        <div class="head">
          <button type="button" class="name" @click="$emit('browse', p)">{{ p.name }}</button>
          <span class="status">{{ statusText[p.status] }}</span>
        </div>
        <div v-if="p.status === 'uploading' || p.status === 'ingesting' || p.status === 'analyzing'" class="bar" role="progressbar" :aria-valuenow="progress(p)" aria-valuemin="0" aria-valuemax="100">
          <span :style="{ width: `${progress(p)}%` }" />
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
              <button type="button" :disabled="p.status !== 'ready' && p.status !== 'failed'" @click="retag(p, false)">
                Re-run offline tagger
              </button>
              <button v-if="stats?.llm_enabled" type="button" :disabled="p.status !== 'ready'" @click="retag(p, true)">
                Re-run LLM tagger
              </button>
              <button v-if="p.unreal_packages || p.unreal_converted" type="button" @click="openUnreal(p)">
                Convert Unreal content…
              </button>
              <button type="button" class="danger" @click="remove(p)">Delete pack…</button>
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
  border-radius: var(--r-tile); box-shadow: 0 10px 30px var(--shadow);
}
.menu-items button {
  border: 0; background: none; text-align: left; padding: 0.45rem 0.6rem; border-radius: var(--r-chip);
  cursor: pointer; font-size: var(--step--1);
}
.menu-items button:hover:not(:disabled) { background: var(--panel); }
.menu-items button:disabled { opacity: 0.5; cursor: default; }
.menu-items .danger { color: var(--danger); }
.find { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem; }
.find .field { max-width: 26rem; }
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
.st-ready .status { color: var(--ok); }
.st-failed .status { color: var(--danger); }
.bar { height: 4px; background: var(--raised); border-radius: 2px; overflow: hidden; }
.bar span { display: block; height: 100%; background: var(--k-texture); transition: width 0.4s; }
.desc { margin: 0; color: var(--muted); max-width: 70ch; }
.error { margin: 0; color: var(--danger); font-size: var(--step--1); }
.facts { display: flex; flex-wrap: wrap; gap: 0.25rem 1.25rem; font-size: var(--step--1); color: var(--muted); }
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
.ustats { flex-direction: row; flex-wrap: wrap; gap: 0.3rem 1.25rem; font-size: var(--step--1); color: var(--muted); }
.ustats li, .reasons li { padding: 0; border: 0; display: block; }
.ustats strong { color: var(--text); font-variant-numeric: tabular-nums; }
.reasons { gap: 0.2rem; margin-top: 0.4rem; font-size: var(--step--1); color: var(--muted); }
.context .check { flex-direction: row; align-items: flex-start; gap: 0.55rem; color: var(--text); cursor: pointer; }
.check input { margin-top: 0.2rem; }
.check small { display: block; color: var(--muted); }
</style>
