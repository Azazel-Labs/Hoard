<script setup lang="ts">
import { computed, defineAsyncComponent, nextTick, ref, watch } from 'vue'
import {
  api, asUrl, bundleUrl, fileUrl, formatDuration, humanBytes, originalUrl, RESIZABLE_AS_ITSELF, thumbUrl,
  VIEWABLE_EXTS,
  type Asset, type AssetLink, type Collection, type CollectionRef, type ConversionStatus, type Facet, type LinkRel,
} from '../api'
import CollectionPicker from './CollectionPicker.vue'
import { playableUrl, player, sourceExt, sourceUrl, stop as stopPreview } from '../player'
import KindGlyph from './KindGlyph.vue'
import TagChip from './TagChip.vue'
import { showOk } from '../ui'

// three.js is big; only load it when someone opens a model.
const ModelViewer = defineAsyncComponent(() => import('./ModelViewer.vue'))

const props = defineProps<{
  asset: Asset
  /** Where this asset sits in the current results, for previous/next. */
  position: { index: number; total: number; hasPrev: boolean; hasNext: boolean } | null
  collections: Collection[]
}>()
const emit = defineEmits<{
  close: []; updated: [Asset]; tag: [string]; error: [string]; open: [string]; step: [number]
  /** Add to a collection by id, or (with a name) create one first. */
  collect: [id: string, create?: string]
  uncollect: [id: string]
  browseCollection: [CollectionRef]
}>()
const inCollections = computed(() => props.asset.collections ?? [])

// The grid's quick preview and this panel's player shouldn't talk over each other.
const audioEl = ref<HTMLAudioElement | null>(null)
watch(() => player.playing, (on) => {
  if (on) audioEl.value?.pause()
})

const withAnimations = ref(false)
const isModel = computed(
  () => (props.asset.kind === 'model' || props.asset.kind === 'animation') &&
    (VIEWABLE_EXTS.has(props.asset.ext) || props.asset.derivatives.includes('glb')),
)
const REL_LABEL: Record<LinkRel, string> = {
  texture: 'texture', buffer: 'geometry data', material: 'material file', animation: 'animation',
  library: 'linked file', guess: 'probably goes with', variant: 'variant',
}
const needs = computed(() => props.asset.links.filter((l) => l.rel !== 'animation'))
const anims = computed(() => props.asset.links.filter((l) => l.rel === 'animation'))
const usedBy = computed(() => props.asset.linked_from)
const linkGroups = computed(() =>
  [
    { title: 'Needs', items: needs.value },
    { title: 'Animations', items: anims.value },
    { title: 'Used by', items: usedBy.value },
  ].filter((g) => g.items.length),
)
const linkLabel = (l: AssetLink) => REL_LABEL[l.rel]
const linkedCount = computed(() => needs.value.length + (withAnimations.value ? anims.value.length : 0))

// Variant families: tag edits can fan out to every take.
const family = computed(() => props.asset.family ?? null)
const applyToFamily = ref(true)
const duplicates = computed(() => props.asset.duplicates ?? [])
const showViews = ref(false)
const hasViews = computed(() => props.asset.derivatives.includes('views'))

const editingName = ref(false)
const nameDraft = ref('')
const newTag = ref('')
const suggestions = ref<Facet[]>([])
const copied = ref('')
const canvas = ref<HTMLCanvasElement | null>(null)
const waveState = ref<'idle' | 'loading' | 'done' | 'skipped'>('idle')

const isImage = computed(() => props.asset.kind === 'texture' && props.asset.has_thumb)
const audioUrl = computed(() => playableUrl(props.asset))
const isAudio = computed(() => audioUrl.value !== null)
const recovered = computed(() => sourceExt(props.asset))
const ueTags = computed(() => {
  const t = props.asset.meta.ue_tags
  return t && typeof t === 'object' ? Object.entries(t as Record<string, string>) : []
})
// "Download as": another format, or the same one scaled down. '' = the original format.
const asFormats = computed(() => props.asset.formats ?? [])
const sameFormat = computed(() => RESIZABLE_AS_ITSELF[props.asset.ext.toLowerCase()] ?? null)
const asFormat = ref('')
const asMax = ref(0)
watch(() => props.asset.id, () => {
  asFormat.value = sameFormat.value ? '' : (asFormats.value[0]?.format ?? '')
  asMax.value = 0
}, { immediate: true })
// The format list arrives with the full asset detail, after the tile's summary: pick a default once it's there.
watch(asFormats, (list) => {
  const valid = asFormat.value === '' ? sameFormat.value !== null : list.some((f) => f.format === asFormat.value)
  if (!valid) asFormat.value = sameFormat.value ? '' : (list[0]?.format ?? '')
})
const asResizable = computed(() =>
  asFormat.value === '' ? sameFormat.value !== null : asFormats.value.some((f) => f.format === asFormat.value && f.resizable),
)
/** Powers of two below the image's longest side, largest first. */
const sizeChoices = computed(() => {
  const w = Number(props.asset.meta.width ?? 0)
  const h = Number(props.asset.meta.height ?? 0)
  const out: number[] = []
  for (let m = 8192; m >= 64; m /= 2) if (m < Math.max(w, h)) out.push(m)
  return out.slice(0, 6)
})
const asHref = computed(() => {
  const max = asResizable.value ? asMax.value : 0
  if (asFormat.value === '') return sameFormat.value && max ? asUrl(props.asset, sameFormat.value, max) : null
  return asUrl(props.asset, asFormat.value, max || undefined)
})
// Conversions run on the server in the background: poll until it's made, then download. Cached ones are instant.
const asState = ref<ConversionStatus | null>(null)
const asError = ref('')
let asRun = 0
watch([() => props.asset.id, asFormat, asMax], () => {
  asRun++
  asState.value = null
  asError.value = ''
})
async function downloadAs(ev: MouseEvent) {
  const href = asHref.value
  if (!href || ev.ctrlKey || ev.metaKey || ev.shiftKey || ev.button !== 0) return
  ev.preventDefault()
  if (asState.value) return
  const run = ++asRun
  const format = asFormat.value || sameFormat.value || ''
  const max = asResizable.value && asMax.value ? asMax.value : undefined
  asError.value = ''
  asState.value = { state: 'queued' }
  try {
    for (;;) {
      const asked = Date.now()
      const st = await api.asStatus(props.asset, format, max)
      if (run !== asRun) return
      if (st.state === 'failed') throw new Error(st.error ?? 'conversion failed')
      if (st.state === 'ready') break
      asState.value = st
      // Progress comes back as it moves; a few updates a second is plenty.
      await new Promise((r) => setTimeout(r, Math.max(0, 300 - (Date.now() - asked))))
    }
    const a = document.createElement('a')
    a.href = href
    a.download = ''
    document.body.appendChild(a)
    a.click()
    a.remove()
  } catch (e) {
    if (run === asRun) asError.value = e instanceof Error ? e.message : String(e)
  } finally {
    if (run === asRun) asState.value = null
  }
}
const asLabel = computed(() => {
  const st = asState.value
  if (st?.state === 'converting') return st.progress !== undefined ? `Converting… ${Math.round(st.progress * 100)}%` : 'Converting…'
  if (st) return 'Queued…'
  const f = asFormat.value || sameFormat.value || ''
  return `Download .${f === 'gltf' ? 'gltf.zip' : f}`
})
const showAs = computed(() => asFormats.value.length > 0 || (sameFormat.value !== null && sizeChoices.value.length > 0))

const origin = computed(() => props.asset.origin ?? null)
const ueStatus = computed(() => props.asset.unreal ?? null)
const originFile = computed(() => origin.value?.path.split('/').pop() ?? '')
const pixelated = computed(() => props.asset.tags.some((t) => t.tag === 'style:pixel-art'))

const visibleTags = computed(() =>
  props.asset.tags.filter((t) => !t.tag.startsWith('kind:') && !t.tag.startsWith('format:')),
)

const metaRows = computed(() => {
  const m = props.asset.meta
  const rows: [string, string][] = []
  const num = (k: string) => (typeof m[k] === 'number' ? (m[k] as number) : undefined)
  if (num('width') !== undefined) rows.push(['Size', `${m.width} × ${m.height} px`])
  if (num('duration_secs') !== undefined) rows.push(['Length', formatDuration(num('duration_secs')!)])
  if (num('sample_rate') !== undefined) rows.push(['Sample rate', `${(num('sample_rate')! / 1000).toFixed(1)} kHz`])
  if (num('channels') !== undefined) rows.push(['Channels', num('channels') === 1 ? 'Mono' : num('channels') === 2 ? 'Stereo' : String(m.channels)])
  if (num('bit_depth') !== undefined) rows.push(['Bit depth', `${m.bit_depth}-bit`])
  if (num('peak_dbfs') !== undefined) {
    rows.push(['Level', `peak ${num('peak_dbfs')!.toFixed(1)} dBFS, RMS ${num('rms_dbfs')!.toFixed(1)} dBFS${m.clipped ? ', clips' : ''}`])
  }
  if (num('loop_start') !== undefined && num('sample_rate')) {
    const sr = num('sample_rate')!
    rows.push(['Loop', `${formatDuration(num('loop_start')! / sr)} to ${formatDuration(num('loop_end')! / sr)}`])
  } else if (m.one_shot === true) rows.push(['Playback', 'One-shot'])
  if (num('tempo_bpm') !== undefined) rows.push(['Tempo', `${m.tempo_bpm} BPM`])
  if (typeof m.root_note === 'string') rows.push(['Root note', m.root_note as string])
  if (num('cue_points')) rows.push(['Cue markers', String(m.cue_points)])
  if (typeof m.ucs_category === 'string') {
    rows.push(['UCS', [m.ucs_category, m.ucs_subcategory].filter(Boolean).join(' / ') + (m.ucs_catid ? ` (${m.ucs_catid})` : '')])
  }
  if (typeof m.library === 'string') rows.push(['Library', m.library as string])
  if (Array.isArray(m.keywords) && m.keywords.length) rows.push(['Keywords', (m.keywords as string[]).join(', ')])
  for (const k of ['title', 'artist', 'album', 'genre', 'generator']) {
    if (typeof m[k] === 'string') rows.push([k[0].toUpperCase() + k.slice(1), m[k] as string])
  }
  if (typeof m.blender_version === 'string') rows.push(['Saved with', `Blender ${m.blender_version}`])
  if (typeof m.color_mode === 'string') rows.push(['Color mode', (m.color_mode as string).toUpperCase()])
  if (num('layers') !== undefined) rows.push(['Layers', String(m.layers)])
  if (m.hdr === true) rows.push(['Range', 'HDR (preview is tone-mapped)'])
  if (m.large_document === true) rows.push(['Format', 'Large document (PSB)'])
  if (typeof m.compression === 'string') rows.push(['Compression', m.compression as string])
  if (typeof m.note === 'string') rows.push(['Note', m.note as string])
  if (num('triangles') !== undefined) rows.push(['Triangles', num('triangles')!.toLocaleString()])
  if (num('vertices') !== undefined) rows.push(['Vertices', num('vertices')!.toLocaleString()])
  for (const k of ['meshes', 'materials', 'animations', 'skins']) {
    if (num(k)) rows.push([k[0].toUpperCase() + k.slice(1), String(m[k])])
  }
  if (typeof m.ue_class === 'string') {
    rows.push(['Unreal asset', `${m.ue_class}${typeof m.ue_engine === 'string' ? `, saved with UE ${m.ue_engine}` : ''}`])
    if (m.ue_cooked === true) rows.push(['Package', 'Cooked (game build): previews and sources are stripped'])
  }
  if (m.tileable === true) rows.push(['Tiling', 'Seamless on both axes'])
  if (typeof m.analysis_error === 'string') rows.push(['Note', `Couldn't fully read this file: ${m.analysis_error}`])
  return rows
})

const tomlSnippet = computed(
  () => `[[select]]\nids = ["${props.asset.id}"]  # ${props.asset.name}\ndest = "${props.asset.category ?? props.asset.kind}"`,
)

async function copy(text: string, what: string) {
  try {
    await navigator.clipboard.writeText(text)
    copied.value = what
    setTimeout(() => (copied.value = ''), 1500)
  } catch {
    emit('error', 'Copy failed: the browser blocked clipboard access')
  }
}

async function patch(body: Parameters<typeof api.patchAsset>[1]): Promise<boolean> {
  if (family.value && applyToFamily.value && (body.add_tags || body.remove_tags || body.category || body.description)) {
    body = { ...body, apply_to_family: true }
  }
  // Tag edits show at once and are rolled back if the server says no.
  const before = props.asset
  const optimistic = !!(body.add_tags || body.remove_tags)
  if (optimistic) {
    const drop = new Set(body.remove_tags ?? [])
    const tags = before.tags.filter((t) => !drop.has(t.tag))
    for (const tag of body.add_tags ?? []) {
      if (!tags.some((t) => t.tag === tag)) tags.push({ tag, source: 'human', confidence: 1 })
    }
    emit('updated', { ...before, tags })
  }
  try {
    emit('updated', await api.patchAsset(props.asset.id, body))
    return true
  } catch (e) {
    if (optimistic) emit('updated', before)
    emit('error', `Couldn't save: ${(e as Error).message}`)
    return false
  }
}

async function removeTag(tag: string) {
  if (await patch({ remove_tags: [tag] })) showOk(`Removed ${tag}`, { label: 'Undo', run: () => void patch({ add_tags: [tag] }) })
}

function startRename() {
  nameDraft.value = props.asset.name
  editingName.value = true
  nextTick(() => document.getElementById('asset-name-input')?.focus())
}
async function saveName() {
  editingName.value = false
  const name = nameDraft.value.trim()
  if (name && name !== props.asset.name) await patch({ name })
}

async function addTag() {
  const t = newTag.value.trim()
  if (!t) return
  newTag.value = ''
  suggestions.value = []
  await patch({ add_tags: [t] })
}

let suggestTimer: ReturnType<typeof setTimeout> | undefined
watch(newTag, (v) => {
  clearTimeout(suggestTimer)
  if (v.trim().length < 2) {
    suggestions.value = []
    return
  }
  suggestTimer = setTimeout(async () => {
    try {
      suggestions.value = await api.tags(v.trim().toLowerCase())
    } catch {
      suggestions.value = []
    }
  }, 150)
})

/** Draw a min/max waveform from the decoded audio. Skipped for big files to keep the tab responsive. */
async function drawWaveform() {
  waveState.value = 'idle'
  if (!isAudio.value) return
  if (props.asset.size > 25 * 1024 * 1024) {
    waveState.value = 'skipped'
    return
  }
  waveState.value = 'loading'
  const id = props.asset.id
  try {
    const buf = await (await fetch(audioUrl.value!)).arrayBuffer()
    const ctx = new OfflineAudioContext(1, 1, 44100)
    const audio = await ctx.decodeAudioData(buf)
    if (id !== props.asset.id) return
    await nextTick()
    const c = canvas.value
    if (!c) return
    const w = (c.width = c.clientWidth * devicePixelRatio)
    const h = (c.height = c.clientHeight * devicePixelRatio)
    const g = c.getContext('2d')!
    const data = audio.getChannelData(0)
    const step = Math.max(1, Math.floor(data.length / w))
    g.fillStyle = getComputedStyle(c).getPropertyValue('--k-audio')
    for (let x = 0; x < w; x++) {
      let min = 1
      let max = -1
      for (let i = x * step; i < (x + 1) * step && i < data.length; i++) {
        const v = data[i]
        if (v < min) min = v
        if (v > max) max = v
      }
      g.fillRect(x, ((1 - max) / 2) * h, 1, Math.max(1, ((max - min) / 2) * h))
    }
    waveState.value = 'done'
  } catch {
    waveState.value = 'skipped'
  }
}

watch(() => props.asset.id, drawWaveform, { immediate: true })
</script>

<template>
  <aside class="detail" :style="{ '--kind': `var(--k-${asset.kind})` }" aria-label="Asset details">
    <header>
      <div class="titlebar">
        <input
          v-if="editingName"
          id="asset-name-input"
          v-model="nameDraft"
          class="field name-input"
          @keydown.enter="saveName"
          @keydown.esc="editingName = false"
          @blur="saveName"
        />
        <h2 v-else :title="'Click to rename'" @click="startRename">{{ asset.name }}</h2>
        <div class="nav">
          <button class="btn ghost icon" type="button" aria-label="Previous asset" :disabled="!position?.hasPrev" @click="$emit('step', -1)">
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M10 3 5 8l5 5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
          </button>
          <span v-if="position" class="pos mono">{{ position.index + 1 }}/{{ position.total }}</span>
          <button class="btn ghost icon" type="button" aria-label="Next asset" :disabled="!position?.hasNext" @click="$emit('step', 1)">
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m6 3 5 5-5 5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
          </button>
          <button class="btn ghost icon close" type="button" aria-label="Close details" @click="$emit('close')">
            <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
          </button>
        </div>
      </div>
      <p class="where">
        <span>{{ asset.category ?? asset.kind }}</span>
        <span class="sep" aria-hidden="true">in</span>
        <span>{{ asset.pack_name }}</span>
      </p>
    </header>

    <div v-if="isModel && showViews" class="preview checker">
      <img :src="`/api/assets/${asset.id}/derivative/views`" alt="The model from four directions" />
    </div>
    <ModelViewer v-else-if="isModel" :asset="asset" />
    <div v-else class="preview checker">
      <img v-if="isImage || (asset.has_thumb && !isAudio)" :src="thumbUrl(asset)" alt="" :class="{ pixelated, glyphs: asset.kind === 'font' }" />
      <canvas v-else-if="isAudio && waveState !== 'skipped'" ref="canvas" class="wave" />
      <KindGlyph v-else :kind="asset.kind" :size="72" />
    </div>
    <label v-if="isModel && hasViews" class="animcheck">
      <input v-model="showViews" type="checkbox" /> Show the four-view sheet the tagger saw
    </label>
    <audio v-if="isAudio" ref="audioEl" :src="audioUrl ?? undefined" controls preload="none" class="player" @play="stopPreview" />

    <p v-if="asset.description" class="desc">{{ asset.description }}</p>

    <section>
      <h3>Tags</h3>
      <TransitionGroup tag="div" name="chip" class="tags">
        <TagChip
          v-for="t in visibleTags"
          :key="t.tag"
          :tag="t.tag"
          :source="t.source"
          removable
          @click="$emit('tag', t.tag)"
          @remove="removeTag(t.tag)"
        />
      </TransitionGroup>
      <form class="addtag" @submit.prevent="addTag">
        <label class="visually-hidden" for="new-tag">Add a tag</label>
        <input id="new-tag" v-model="newTag" class="field" list="tag-suggestions" placeholder="Add a tag, e.g. mood:tense" autocomplete="off" />
        <datalist id="tag-suggestions">
          <option v-for="s in suggestions" :key="s.value" :value="s.value">{{ s.count }}</option>
        </datalist>
        <button class="btn" type="submit" :disabled="!newTag.trim()">Add</button>
      </form>
      <label v-if="family" class="animcheck familycheck">
        <input v-model="applyToFamily" type="checkbox" /> Apply tag and category changes to all {{ family.size }} variants
      </label>
      <p class="legend">
        Solid tags were added by a person, outlined ones by the offline tagger, dashed ones by the LLM. Click a tag to filter by it.
      </p>
    </section>

    <section class="cols">
      <h3>Collections</h3>
      <div class="tags">
        <span v-for="c in inCollections" :key="c.id" class="chip src-human colchip">
          <button type="button" class="linkish" @click="$emit('browseCollection', c)">{{ c.name }}</button>
          <button class="x" type="button" :aria-label="`Remove from ${c.name}`" @click="$emit('uncollect', c.id)">×</button>
        </span>
        <CollectionPicker
          :collections="collections"
          :current="inCollections.map((c) => c.id)"
          :label="inCollections.length ? 'Add to another' : 'Add to collection'"
          @add="(id) => $emit('collect', id)"
          @remove="(id) => $emit('uncollect', id)"
          @create="(name) => $emit('collect', '', name)"
        />
      </div>
    </section>

    <section v-for="g in linkGroups" :key="g.title">
      <h3>{{ g.title }}</h3>
      <ul class="links">
        <li v-for="l in g.items" :key="l.id + l.rel">
          <button type="button" class="link" :style="{ '--kind': `var(--k-${l.kind})` }" @click="$emit('open', l.id)">
            <span class="lthumb checker">
              <img v-if="l.has_thumb" :src="thumbUrl(l)" :class="{ glyphs: l.kind === 'font' }" alt="" loading="lazy" />
              <KindGlyph v-else :kind="l.kind" :size="22" />
            </span>
            <span class="lname">{{ l.name }}</span>
            <span class="lrel">{{ linkLabel(l) }}</span>
          </button>
        </li>
      </ul>
    </section>

    <section v-if="family">
      <h3>Variants ({{ family.size }} takes)</h3>
      <ul class="links">
        <li v-for="l in family.members" :key="l.id">
          <button type="button" class="link" :style="{ '--kind': `var(--k-${l.kind})` }" @click="$emit('open', l.id)">
            <span class="lthumb checker">
              <img v-if="l.has_thumb" :src="thumbUrl(l)" :class="{ glyphs: l.kind === 'font' }" alt="" loading="lazy" />
              <KindGlyph v-else :kind="l.kind" :size="22" />
            </span>
            <span class="lname">{{ l.name }}</span>
            <span class="lrel">{{ l.path.split('/').pop() }}</span>
          </button>
        </li>
      </ul>
    </section>

    <section v-if="duplicates.length">
      <h3>Identical copies</h3>
      <p class="legend dupnote">The same bytes are stored once; these packs ship this file too.</p>
      <ul class="links">
        <li v-for="d in duplicates" :key="d.id">
          <button type="button" class="link dup" @click="$emit('open', d.id)">
            <span class="lname">{{ d.pack_name }}</span>
            <span class="lrel">{{ d.path }}</span>
          </button>
        </li>
      </ul>
    </section>

    <section v-if="origin || ueStatus || recovered || ueTags.length" class="unreal">
      <h3>{{ origin ? 'Converted from Unreal' : 'From the Unreal package' }}</h3>
      <template v-if="origin">
        <p class="origin-note">
          This file was <span class="mono">{{ originFile }}</span><template v-if="origin.class">, a {{ origin.class }}</template><template v-if="origin.engine"> saved by UE {{ origin.engine }}</template>.
          {{ origin.method === 'editor' ? 'Unreal Editor exported it' : 'Hoard read it out of the package' }}
          on {{ new Date(origin.converted_at).toLocaleDateString() }}<template v-if="origin.notes"> ({{ origin.notes }})</template>.
        </p>
        <p class="legend">
          <a v-if="origin.original_kept" class="btn" :href="originalUrl(asset)">Download the original .{{ origin.ext }}</a>
          <template v-else>The original package was discarded after conversion.</template>
        </p>
      </template>
      <p v-else-if="ueStatus?.status === 'needs_editor'" class="legend">
        Waiting for Unreal Editor: {{ ueStatus.reason }}. On a machine with Unreal installed, run
        <span class="mono">hoard unreal-worker --editor &lt;path to UnrealEditor-Cmd&gt;</span> and it comes back as FBX.
      </p>
      <p v-else-if="ueStatus?.status === 'unsupported'" class="legend">
        Kept as a package: {{ ueStatus.reason }}.
      </p>
      <p v-else-if="ueStatus?.status === 'failed'" class="legend bad">
        Conversion failed: {{ ueStatus.reason }}
      </p>
      <p v-if="recovered && !origin" class="recovered">
        The original .{{ recovered }} was stored inside this package, and Hoard pulled it out.
        <a class="btn" :href="sourceUrl(asset, true)">Download .{{ recovered }}</a>
      </p>
      <p v-else-if="!origin && !ueStatus && (asset.kind === 'model' || asset.kind === 'animation')" class="legend">
        Meshes and animations are stored in Unreal's own format. Convert the pack (pack page, More), then run
        <span class="mono">hoard unreal-worker</span> next to an Unreal Editor install to get FBX.
      </p>
      <details v-if="ueTags.length">
        <summary>{{ ueTags.length }} asset registry {{ ueTags.length === 1 ? 'tag' : 'tags' }}</summary>
        <dl>
          <template v-for="[k, v] in ueTags" :key="k">
            <dt>{{ k }}</dt>
            <dd class="mono">{{ v }}</dd>
          </template>
        </dl>
      </details>
    </section>

    <section v-if="metaRows.length">
      <h3>File</h3>
      <dl>
        <template v-for="[k, v] in metaRows" :key="k">
          <dt>{{ k }}</dt>
          <dd>{{ v }}</dd>
        </template>
        <dt>Bytes</dt>
        <dd>{{ humanBytes(asset.size) }}</dd>
      </dl>
    </section>

    <section>
      <h3>Source</h3>
      <dl>
        <dt>Path</dt>
        <dd class="mono path">{{ asset.path }}</dd>
        <dt>BLAKE3</dt>
        <dd class="mono hash">
          <button type="button" class="linkish" @click="copy(asset.hash, 'hash')">{{ asset.hash.slice(0, 16) }}…</button>
        </dd>
      </dl>
    </section>

    <footer>
      <template v-if="needs.length || anims.length">
        <a class="btn primary" :href="bundleUrl(asset, withAnimations)">Download with {{ linkedCount }} linked {{ linkedCount === 1 ? 'file' : 'files' }} (.zip)</a>
        <label v-if="anims.length" class="animcheck"><input v-model="withAnimations" type="checkbox" /> Include animations</label>
        <a class="btn" :href="fileUrl(asset, true)">This file only</a>
      </template>
      <a v-else class="btn primary" :href="fileUrl(asset, true)">Download</a>
      <div v-if="showAs" class="asrow">
        <label for="as-format">Download as</label>
        <select id="as-format" v-model="asFormat" class="field">
          <option v-if="sameFormat" value="">{{ asset.ext.toUpperCase() }} (original format)</option>
          <option v-for="f in asFormats" :key="f.format" :value="f.format">{{ f.label }}</option>
        </select>
        <select v-if="asResizable && sizeChoices.length" v-model.number="asMax" class="field" aria-label="Size">
          <option :value="0">Full size</option>
          <option v-for="m in sizeChoices" :key="m" :value="m">Up to {{ m }} px</option>
        </select>
        <a v-if="asHref" class="btn" :href="asHref" :aria-busy="asState ? 'true' : 'false'" @click="downloadAs">{{ asLabel }}</a>
        <span v-else class="ashint">Pick a size or another format</span>
        <span v-if="asError" class="aserror" role="alert">{{ asError }}</span>
      </div>
      <button class="btn" type="button" @click="copy(tomlSnippet, 'toml')">Copy hoard.toml entry</button>
      <button class="btn ghost" type="button" @click="copy(asset.id, 'id')">Copy ID</button>
      <span class="copied" role="status">{{ copied ? `Copied ${copied}` : '' }}</span>
    </footer>
  </aside>
</template>

<style scoped>
.detail {
  display: flex; flex-direction: column; gap: 1rem;
  padding: 1.1rem 1.2rem 1.4rem;
  background: var(--panel); border-left: 1px solid var(--line);
  overflow-y: auto; height: 100%;
}
/* The panel scrolls; its sections must keep their size instead of squashing the preview. */
.detail > * { flex-shrink: 0; }
.titlebar { display: flex; gap: 0.5rem; align-items: flex-start; }
h2 {
  flex: 1; margin: 0; font-size: var(--step-1); font-weight: 650; line-height: 1.25; cursor: text;
  font-variation-settings: 'CASL' 0.4;
}
.name-input { flex: 1; font-size: var(--step-1); font-weight: 650; }
.nav { display: flex; align-items: center; gap: 0.1rem; flex: none; }
.icon { padding: 0.35em 0.45em; }
.pos { font-size: 0.72rem; color: var(--faint); padding: 0 0.15rem; }
.close { margin-left: 0.3rem; }
.where { margin: 0.25rem 0 0; color: var(--muted); font-size: var(--step--1); display: flex; gap: 0.4em; flex-wrap: wrap; }
.sep { color: var(--faint); }
.preview {
  display: grid; place-items: center; aspect-ratio: 4 / 3;
  border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--kind);
}
.preview img { max-width: 100%; max-height: 100%; object-fit: contain; }
.preview img.pixelated { image-rendering: pixelated; width: 100%; height: 100%; }
.wave { width: 100%; height: 60%; }
.player { width: 100%; height: 36px; }
.desc { margin: 0; color: var(--muted); max-width: 60ch; }
h3 { margin: 0 0 0.5rem; font-size: var(--step--1); font-weight: 650; color: var(--muted); }
.tags { position: relative; display: flex; flex-wrap: wrap; gap: 0.35rem; }
.tags :deep(.chip) { cursor: pointer; }
.chip-enter-active { transition: opacity var(--dur-2) var(--ease-out), scale var(--dur-3) var(--ease-pop); }
.chip-leave-active { transition: opacity var(--dur-1) var(--ease-out), scale var(--dur-1) var(--ease-out); position: absolute; }
.chip-enter-from, .chip-leave-to { opacity: 0; scale: 0.6; }
.chip-move { transition: translate var(--dur-2) var(--ease-out); }
.addtag { display: flex; gap: 0.4rem; margin-top: 0.6rem; }
.legend { margin: 0.5rem 0 0; font-size: 0.75rem; color: var(--faint); max-width: 52ch; }
.asrow { flex-basis: 100%; display: flex; flex-wrap: wrap; align-items: center; gap: 0.4rem; font-size: var(--step--1); color: var(--muted); }
.asrow select { width: auto; min-width: 0; max-width: 16rem; padding-block: 0.3rem; }
.ashint { color: var(--faint); }
.aserror { flex-basis: 100%; color: var(--danger); }
.asrow .btn[aria-busy='true'] { cursor: progress; font-variant-numeric: tabular-nums; }
.origin-note { margin: 0; font-size: var(--step--1); color: var(--muted); max-width: 52ch; }
.unreal .bad { color: var(--danger); }
dl { display: grid; grid-template-columns: max-content 1fr; gap: 0.3rem 1rem; margin: 0; font-size: var(--step--1); }
dt { color: var(--muted); }
dd { margin: 0; min-width: 0; }
.path { word-break: break-all; }
.linkish { border: 0; background: none; padding: 0; cursor: pointer; color: var(--text); text-decoration: underline dotted var(--faint); font: inherit; }
/* The download actions stay in reach while the panel scrolls. */
footer {
  position: sticky; bottom: -1.4rem; z-index: 1; display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center;
  margin: auto -1.2rem -1.4rem; padding: 0.75rem 1.2rem 1rem; background: var(--panel); border-top: 1px solid var(--line);
}
.animcheck { display: inline-flex; gap: 0.35rem; align-items: center; font-size: var(--step--1); color: var(--muted); }
.links { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 0.3rem; }
.link {
  display: grid; grid-template-columns: 32px 1fr auto; gap: 0.6rem; align-items: center; width: 100%;
  padding: 0.25rem; border: 1px solid transparent; border-radius: var(--r-chip); background: none; cursor: pointer; text-align: left;
}
.link:hover { border-color: var(--line); background: var(--raised); }
.lthumb {
  width: 32px; height: 32px; border-radius: 3px; overflow: hidden; display: grid; place-items: center;
  box-shadow: inset 0 2px 0 var(--kind);
}
.lthumb img { width: 100%; height: 100%; object-fit: contain; }
.lname { font-size: var(--step--1); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.lrel { font-size: 0.75rem; color: var(--faint); }
.copied { font-size: 0.75rem; color: var(--ok); }
.familycheck { margin-top: 0.5rem; }
.cols .tags { align-items: center; }
.colchip { gap: 0.35em; }
.colchip .x { border: 0; background: none; cursor: pointer; color: var(--faint); padding: 0; font: inherit; }
.colchip .x:hover { color: var(--text); }
.cols :deep(.picker .btn) { padding: 0.2em 0.6em; }
.recovered { margin: 0 0 0.6rem; display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; font-size: var(--step--1); color: var(--muted); }
.unreal summary { cursor: pointer; font-size: var(--step--1); color: var(--muted); margin-bottom: 0.4rem; }
.unreal dd { word-break: break-word; }
.dupnote { margin: 0 0 0.4rem; }
.link.dup { grid-template-columns: 1fr auto; }
.link.dup .lrel { max-width: 22ch; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; }
</style>
