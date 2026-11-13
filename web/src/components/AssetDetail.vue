<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import {
  api, asUrl, bundleUrl, fileUrl, formatDuration, humanBytes, looksAlike, originalUrl, RESIZABLE_AS_ITSELF, thumbUrl,
  type Asset, type AssetLink, type Collection, type CollectionRef, type ConversionStatus, type Facet, type LinkRel,
  type MaterialInfo,
} from '../api'
import CollectionPicker from './CollectionPicker.vue'
import { playableUrl, player, sourceExt, sourceUrl, stop as stopPreview } from '../player'
import KindGlyph from './KindGlyph.vue'
import TagChip from './TagChip.vue'
import { copyText, showOk, swipeToClose } from '../ui'
import { lazy } from '../lazy'
import { isModelAsset, isTextAsset, takeViewRequest, viewRequest, viewerFor, type ViewerKind } from '../viewers'
import { isMaterialAsset, resolutions, ROLE_LABEL, sizeLabel } from '../materials'
import { withThumb } from '../browserThumbs'

// three.js is big; only load it when someone opens a model.
const ModelViewer = lazy(() => import('./ModelViewer.vue'), 'the 3D viewer', 'model-slot')
const ModelInspector = lazy(() => import('./ModelInspector.vue'), 'the 3D viewer', 'overlay')
const TextPreview = lazy(() => import('./TextPreview.vue'), 'the text')
const TextViewer = lazy(() => import('./TextViewer.vue'), 'the text viewer', 'overlay')
const ImageViewer = lazy(() => import('./ImageViewer.vue'), 'the image viewer', 'overlay')
const MaterialPreview = lazy(() => import('./MaterialPreview.vue'), 'the material preview', 'model-slot')
const MaterialViewer = lazy(() => import('./MaterialViewer.vue'), 'the material viewer', 'overlay')

const props = defineProps<{
  asset: Asset
  /** Where this asset sits in the current results, for previous/next. */
  position: { index: number; total: number; hasPrev: boolean; hasNext: boolean } | null
  collections: Collection[]
}>()
const emit = defineEmits<{
  close: []; updated: [Asset]; tag: [string]; error: [string]; open: [string]; step: [number]
  /** Move it to the archive (out of search until it's restored or deleted for good). */
  archive: []
  /** Add to a collection by id, or (with a name) create one first. */
  collect: [id: string, create?: string]
  uncollect: [id: string]
  browseCollection: [CollectionRef]
  /** Find assets that look like this one. */
  similar: []
  /** Find assets with much of this colour (hex). */
  color: [string]
}>()
const palette = computed(() => props.asset.palette ?? [])
const alikeable = computed(() => looksAlike(props.asset))
const inCollections = computed(() => props.asset.collections ?? [])

// The grid's quick preview and this panel's player shouldn't talk over each other.
const audioEl = ref<HTMLAudioElement | null>(null)
watch(() => player.playing, (on) => {
  if (on) audioEl.value?.pause()
})

const withAnimations = ref(false)
const isModel = computed(() => isModelAsset(props.asset))
const isText = computed(() => isTextAsset(props.asset))
const REL_LABEL: Record<LinkRel, string> = {
  texture: 'texture', buffer: 'geometry data', material: 'material file', animation: 'animation',
  library: 'linked file', guess: 'probably goes with', sidecar: 'store details', variant: 'variant',
}
const needs = computed(() => props.asset.links.filter((l) => l.rel !== 'animation'))
// A material made from texture maps lists its maps by role instead of as "Needs"; a texture or model that belongs
// to one names it.
const isMaterial = computed(() => isMaterialAsset(props.asset))
const material = computed(() => props.asset.material ?? null)
const partOf = computed(() => props.asset.materials ?? [])
const roleOfHere = (m: MaterialInfo) => {
  const own = m.maps.find((x) => x.id === props.asset.id)
  return own ? `${ROLE_LABEL[own.role].toLowerCase()} map` : `${m.maps.length} maps`
}
const mapsBytes = computed(() => material.value?.maps.reduce((n, m) => n + m.size, 0) ?? 0)
const anims = computed(() => props.asset.links.filter((l) => l.rel === 'animation'))
// The materials it belongs to have a section of their own.
const usedBy = computed(() => props.asset.linked_from.filter((l) => !partOf.value.some((m) => m.id === l.id)))
const linkGroups = computed(() =>
  [
    { title: 'Needs', items: isMaterial.value ? [] : needs.value.filter((l) => !partOf.value.some((m) => m.id === l.id)) },
    { title: 'Animations', items: anims.value },
    { title: 'Used by', items: usedBy.value },
  ].filter((g) => g.items.length),
)
const linkLabel = (l: AssetLink) => REL_LABEL[l.rel]
/** Long lists show their first few: the panel is about one asset, and a 40-texture "Needs" pushed the rest of it
 *  out of reach. A list only a couple longer than that shows whole, rather than hiding two behind a button. */
const LIST_PEEK = 6
const unfolded = ref(new Set<string>())
watch(() => props.asset.id, () => (unfolded.value = new Set()))
function peek<T>(key: string, items: T[]): T[] {
  return unfolded.value.has(key) || items.length <= LIST_PEEK + 2 ? items : items.slice(0, LIST_PEEK)
}
const unfold = (key: string) => (unfolded.value = new Set(unfolded.value).add(key))
// A material made from maps comes as its maps, not as a file of its own.
const linkedCount = computed(() => {
  const viaMaterial = partOf.value.filter((m) => needs.value.some((l) => l.id === m.id))
  const files = needs.value.length - viaMaterial.length + viaMaterial.reduce((n, m) => n + m.maps.length, 0)
  return files + (withAnimations.value ? anims.value.length : 0)
})
/** What the zip holds before compression: this file and each linked one once (sizes the server didn't give count 0). A
 * material counts as its maps. */
const bundleBytes = computed(() => {
  const seen = new Map<string, number>()
  for (const l of [...needs.value, ...(withAnimations.value ? anims.value : [])]) seen.set(l.id, l.size ?? 0)
  for (const m of partOf.value) {
    if (!seen.delete(m.id)) continue
    for (const x of m.maps) seen.set(x.id, x.size)
  }
  return props.asset.size + [...seen.values()].reduce((s, n) => s + n, 0)
})

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
const canvasTop = ref<HTMLCanvasElement | null>(null)
const waveState = ref<'idle' | 'loading' | 'done' | 'skipped'>('idle')
/** The full-screen viewer that's open, if any. */
const inspecting = ref<ViewerKind | ''>('')
watch(() => props.asset.id, () => (inspecting.value = ''))
// A double-clicked tile asks for its viewer. The first click may still be opening the asset, and the grid's summary
// can lack what decides the viewer (a converted model's derivatives), so wait until this asset can say.
watch(
  [() => viewRequest.id, () => props.asset],
  () => {
    const kind = viewerFor(props.asset)
    if (kind && takeViewRequest(props.asset.id)) inspecting.value = kind
  },
  { immediate: true },
)

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
  const ai = m.ai as { made?: string; by?: string; how?: string } | undefined
  if (ai?.made) {
    const made = ai.made === 'edited' ? 'Edited with generative AI' : 'Made with generative AI'
    rows.push(['AI', `${made}${ai.by ? ` (${ai.by})` : ''}, says its ${ai.how ?? 'metadata'}`])
  } else if (m.content_credentials === true) rows.push(['Provenance', 'Has C2PA Content Credentials'])
  if (typeof m.compression === 'string') rows.push(['Compression', m.compression as string])
  if (typeof m.note === 'string') rows.push(['Note', m.note as string])
  if (material.value) {
    rows.push(['Maps', String(material.value.maps.length)])
    const sizes = resolutions(material.value)
    if (sizes.length) rows.push([sizes.length === 1 ? 'Size' : 'Sizes', sizes.map(sizeLabel).join(', ')])
  }
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
  if (await copyText(text)) {
    copied.value = what
    setTimeout(() => (copied.value = ''), 1500)
  } else {
    emit('error', 'Couldn’t copy: the browser blocked it')
  }
}

async function patch(body: Parameters<typeof api.patchAsset>[1]): Promise<boolean> {
  if (family.value && applyToFamily.value && (body.add_tags || body.remove_tags || body.category || body.description !== undefined)) {
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

// Descriptions: a person's own (editable here), the AI's, the file's embedded text and the pack's store page.
const editingDesc = ref(false)
const descDraft = ref('')
const storeBlurb = ref<{ pack: string; store: string | null; url: string | null; text: string } | null>(null)
const storeOpen = ref(false)
watch(() => props.asset.id, () => (editingDesc.value = false))
watch(
  () => props.asset.pack_id,
  async (pack) => {
    storeOpen.value = false
    if (storeBlurb.value?.pack === pack) return
    storeBlurb.value = null
    try {
      const ctx = await api.packContext(pack)
      if (props.asset.pack_id === pack && ctx.store_text) {
        storeBlurb.value = { pack, store: ctx.store, url: ctx.store_url, text: ctx.store_text }
      }
    } catch {
      // No context, no blurb.
    }
  },
  { immediate: true },
)
function editDescription() {
  descDraft.value = props.asset.description ?? ''
  editingDesc.value = true
}
async function saveDescription() {
  if (await patch({ description: descDraft.value.trim() })) editingDesc.value = false
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
    if (!duration.value) duration.value = audio.duration
    await nextTick()
    const base = canvas.value
    const top = canvasTop.value
    if (!base || !top) return
    // Bars two device pixels wide with a one-pixel gap: reads as a waveform rather than a smear.
    const w = (base.width = top.width = Math.round(base.clientWidth * devicePixelRatio))
    const h = (base.height = top.height = Math.round(base.clientHeight * devicePixelRatio))
    const data = audio.getChannelData(0)
    const bar = Math.max(2, Math.round(2 * devicePixelRatio))
    const gap = Math.max(1, Math.round(devicePixelRatio))
    const bars = Math.floor(w / (bar + gap))
    const step = Math.max(1, Math.floor(data.length / bars))
    // Canvas can't resolve light-dark() or variables; the computed `color` comes back as plain rgb().
    const color = getComputedStyle(top).color
    const paint = (g: CanvasRenderingContext2D) => {
      g.fillStyle = color
      for (let b = 0; b < bars; b++) {
        let peak = 0
        for (let i = b * step; i < (b + 1) * step && i < data.length; i++) peak = Math.max(peak, Math.abs(data[i]))
        const bh = Math.max(gap, peak * h * 0.95)
        g.fillRect(b * (bar + gap), (h - bh) / 2, bar, bh)
      }
    }
    paint(base.getContext('2d')!)
    paint(top.getContext('2d')!)
    waveState.value = 'done'
  } catch {
    waveState.value = 'skipped'
  }
}

watch(() => props.asset.id, drawWaveform, { immediate: true })

// The player: a hidden <audio> driven by a play button and the waveform, which doubles as the scrubber.
const playing = ref(false)
const current = ref(0)
const duration = ref(0)
/** Where the pointer is over the waveform, 0..1, or null. */
const hoverAt = ref<number | null>(null)
let scrubbing = false
let frame = 0
watch(() => props.asset.id, () => {
  playing.value = false
  current.value = 0
  const d = props.asset.meta.duration_secs
  duration.value = typeof d === 'number' ? d : 0
}, { immediate: true })
const progress = computed(() => (duration.value ? Math.min(1, current.value / duration.value) : 0))

/** m:ss, with tenths for short sounds where a second is a lot. */
function clock(secs: number) {
  if (!Number.isFinite(secs) || secs < 0) secs = 0
  const m = Math.floor(secs / 60)
  const s = secs - m * 60
  return duration.value < 10 ? `${m}:${s.toFixed(1).padStart(4, '0')}` : `${m}:${String(Math.floor(s)).padStart(2, '0')}`
}

// timeupdate only fires a few times a second; follow the playhead every frame while it moves.
function follow() {
  cancelAnimationFrame(frame)
  const tick = () => {
    const el = audioEl.value
    if (!el || el.paused) return
    if (!scrubbing) current.value = el.currentTime
    frame = requestAnimationFrame(tick)
  }
  frame = requestAnimationFrame(tick)
}
function onPlay() {
  playing.value = true
  stopPreview()
  follow()
}
function onPause() {
  playing.value = false
  cancelAnimationFrame(frame)
  if (audioEl.value) current.value = audioEl.value.currentTime
}
function onMeta() {
  const d = audioEl.value?.duration
  if (d && Number.isFinite(d)) duration.value = d
}
function togglePlay() {
  const el = audioEl.value
  if (!el) return
  if (el.paused) {
    if (el.ended || progress.value >= 1) el.currentTime = 0
    void el.play().catch((e) => emit('error', `Couldn't play this file: ${(e as Error).message}`))
  } else el.pause()
}
function seekTo(f: number) {
  const el = audioEl.value
  if (!el || !duration.value) return
  const t = Math.min(1, Math.max(0, f)) * duration.value
  el.currentTime = t
  current.value = t
}
const fractionAt = (e: PointerEvent) => {
  const r = (e.currentTarget as HTMLElement).getBoundingClientRect()
  return Math.min(1, Math.max(0, (e.clientX - r.left) / r.width))
}
function scrubDown(e: PointerEvent) {
  if (e.button !== 0) return
  ;(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId)
  scrubbing = true
  seekTo(fractionAt(e))
}
function scrubMove(e: PointerEvent) {
  hoverAt.value = fractionAt(e)
  if (scrubbing) seekTo(hoverAt.value)
}
function scrubUp() {
  if (!scrubbing) return
  scrubbing = false
  if (audioEl.value?.paused) togglePlay()
}
function scrubKey(e: KeyboardEvent) {
  const nudge = Math.max(1, duration.value / 20)
  const map: Record<string, () => void> = {
    ArrowLeft: () => seekTo((current.value - nudge) / duration.value),
    ArrowDown: () => seekTo((current.value - nudge) / duration.value),
    ArrowRight: () => seekTo((current.value + nudge) / duration.value),
    ArrowUp: () => seekTo((current.value + nudge) / duration.value),
    Home: () => seekTo(0),
    End: () => seekTo(1),
    ' ': togglePlay,
    Enter: togglePlay,
  }
  const f = map[e.key]
  if (!f) return
  e.preventDefault()
  e.stopPropagation()
  f()
}
onBeforeUnmount(() => cancelAnimationFrame(frame))

// On a phone the panel covers the screen: swipe it away to the right.
const root = ref<HTMLElement | null>(null)
let unswipe = () => {}
onMounted(() => {
  if (root.value) unswipe = swipeToClose(root.value, 1, () => emit('close'), true)
})
onBeforeUnmount(() => unswipe())
</script>

<template>
  <aside ref="root" class="detail" :style="{ '--kind': `var(--k-${asset.kind})` }" aria-label="Asset details">
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
    <ModelViewer
      v-else-if="isModel"
      :asset="asset"
      :paused="inspecting === 'model'"
      @expand="inspecting = 'model'"
      @thumb="(r) => emit('updated', withThumb(asset, r))"
    />
    <MaterialPreview
      v-else-if="isMaterial && material"
      :asset="asset"
      :material="material"
      :paused="inspecting === 'material'"
      @expand="inspecting = 'material'"
    />
    <button
      v-else-if="isImage"
      type="button"
      class="preview checker zoomable"
      :aria-label="`Inspect ${asset.name} full screen`"
      title="Inspect: zoom, tile, channels"
      @click="inspecting = 'image'"
    >
      <img :src="thumbUrl(asset)" alt="" :class="{ pixelated }" />
      <span class="zoombadge" aria-hidden="true">
        <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round">
          <circle cx="7" cy="7" r="4.5" /><path d="m10.5 10.5 3.5 3.5M5 7h4M7 5v4" />
        </svg>
      </span>
    </button>
    <div v-else-if="isAudio" class="preview audio">
      <div
        class="scrub"
        :class="{ flat: waveState !== 'done' }"
        role="slider"
        tabindex="0"
        aria-label="Playback position"
        :aria-valuemin="0"
        :aria-valuemax="Math.round(duration)"
        :aria-valuenow="Math.round(current)"
        :aria-valuetext="`${clock(current)} of ${clock(duration)}`"
        :style="{ '--p': progress }"
        @pointerdown="scrubDown"
        @pointermove="scrubMove"
        @pointerup="scrubUp"
        @pointercancel="scrubUp"
        @pointerleave="hoverAt = null"
        @keydown="scrubKey"
      >
        <canvas ref="canvas" class="wave" />
        <canvas ref="canvasTop" class="wave top" />
        <span class="head" aria-hidden="true" />
        <span v-if="hoverAt !== null && duration" class="hover" :style="{ '--h': hoverAt }" aria-hidden="true">
          <span class="mono">{{ clock(hoverAt * duration) }}</span>
        </span>
      </div>
      <div class="transport">
        <button type="button" class="play" :aria-label="playing ? 'Pause' : 'Play'" @click="togglePlay">
          <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3.5" y="2.5" width="3" height="11" rx="1" /><rect x="9.5" y="2.5" width="3" height="11" rx="1" /></svg>
          <svg v-else viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><path d="M4.5 2.8v10.4a.8.8 0 0 0 1.2.7l8.3-5.2a.8.8 0 0 0 0-1.4L5.7 2.1a.8.8 0 0 0-1.2.7Z" /></svg>
        </button>
        <span class="mono time">{{ clock(current) }} <span class="of">/ {{ clock(duration) }}</span></span>
        <span v-if="waveState === 'loading'" class="wavenote">Drawing waveform…</span>
        <span v-else-if="waveState === 'skipped'" class="wavenote">No waveform for files this big</span>
      </div>
      <audio
        ref="audioEl"
        :src="audioUrl ?? undefined"
        preload="none"
        @play="onPlay"
        @pause="onPause"
        @ended="onPause"
        @loadedmetadata="onMeta"
        @durationchange="onMeta"
      />
    </div>
    <TextPreview v-else-if="isText" :asset="asset" @expand="inspecting = 'text'" />
    <div v-else class="preview checker">
      <img v-if="asset.has_thumb" :src="thumbUrl(asset)" alt="" :class="{ pixelated, glyphs: asset.kind === 'font' }" />
      <KindGlyph v-else :kind="asset.kind" :size="72" />
    </div>
    <ImageViewer v-if="inspecting === 'image'" :asset="asset" :pixelated="pixelated" @close="inspecting = ''" />
    <ModelInspector v-else-if="inspecting === 'model'" :asset="asset" @close="inspecting = ''" />
    <TextViewer v-else-if="inspecting === 'text'" :asset="asset" @close="inspecting = ''" />
    <MaterialViewer v-else-if="inspecting === 'material' && material" :asset="asset" :material="material" @close="inspecting = ''" />
    <label v-if="isModel && hasViews" class="animcheck">
      <input v-model="showViews" type="checkbox" /> Show the four-view sheet the tagger saw
    </label>

    <div v-if="alikeable || palette.length" class="looks">
      <div v-if="palette.length" class="palette" role="group" aria-label="Colours in the picture">
        <button
          v-for="c in palette"
          :key="c.hex"
          type="button"
          class="swatch"
          :style="{ background: c.hex, '--g': c.share }"
          :title="`${c.hex}, ${Math.round(c.share * 100)}% of the picture. Find assets with this colour`"
          :aria-label="`Find assets with ${c.hex}, ${Math.round(c.share * 100)}% of this picture`"
          @click="emit('color', c.hex)"
        />
      </div>
      <button v-if="alikeable" type="button" class="btn alike" @click="emit('similar')">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round">
          <rect x="1.8" y="5.5" width="7.5" height="7.5" rx="1.6" />
          <rect x="6.7" y="2.5" width="7.5" height="7.5" rx="1.6" fill="var(--panel)" />
        </svg>
        More like this
      </button>
    </div>

    <section class="descs">
      <form v-if="editingDesc" class="descedit" @submit.prevent="saveDescription">
        <label class="visually-hidden" for="asset-desc">Your description</label>
        <textarea id="asset-desc" v-model="descDraft" class="field" rows="3" maxlength="2000"
          placeholder="What it is, where you'd use it…" />
        <div class="descbtns">
          <button class="btn primary" type="submit">Save</button>
          <button class="btn" type="button" @click="editingDesc = false">Cancel</button>
          <span v-if="family && applyToFamily" class="hint">Saves to all {{ family.size }} variants</span>
        </div>
      </form>
      <template v-else>
        <p v-if="asset.description" class="desc mine">{{ asset.description }}</p>
        <button type="button" class="linkish small" @click="editDescription">
          {{ asset.description ? 'Edit description' : 'Write a description' }}
        </button>
      </template>
      <div v-if="asset.ai_description" class="descsrc">
        <span class="srclabel">Written by AI</span>
        <p class="desc">{{ asset.ai_description }}</p>
      </div>
      <div v-if="asset.source_description" class="descsrc">
        <span class="srclabel">From the file</span>
        <p class="desc">{{ asset.source_description }}</p>
      </div>
      <div v-if="storeBlurb" class="descsrc">
        <span class="srclabel">
          From the
          <a v-if="storeBlurb.url" :href="storeBlurb.url" target="_blank" rel="noopener noreferrer">{{ storeBlurb.store ?? 'store' }} page</a>
          <template v-else>{{ storeBlurb.store ?? 'store' }} page</template>
          (the whole pack)
        </span>
        <p class="desc store" :class="{ open: storeOpen }">{{ storeBlurb.text }}</p>
        <button v-if="storeBlurb.text.length > 240" type="button" class="linkish small" @click="storeOpen = !storeOpen">
          {{ storeOpen ? 'Less' : 'More' }}
        </button>
      </div>
    </section>

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
        <input v-model="applyToFamily" type="checkbox" /> Apply tag, category and description changes to all {{ family.size }} variants
      </label>
      <p class="legend">
        Solid tags were added by a person, outlined ones by the offline tagger, dashed ones by AI. Click a tag to filter by it.
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

    <section v-if="material">
      <h3>Maps</h3>
      <ul class="links">
        <li v-for="m in peek('maps', material.maps)" :key="m.id">
          <button type="button" class="link" style="--kind: var(--k-texture)" :title="m.path" @click="$emit('open', m.id)">
            <span class="lthumb checker">
              <img v-if="m.has_thumb" :src="thumbUrl(m)" alt="" loading="lazy" />
              <KindGlyph v-else kind="texture" :size="22" />
            </span>
            <span class="lname">{{ ROLE_LABEL[m.role] }}</span>
            <span class="lrel mono">{{ m.resolution ? sizeLabel(m.resolution) : (m.width ? `${m.width}×${m.height}` : m.ext) }}</span>
          </button>
        </li>
      </ul>
      <button v-if="peek('maps', material.maps).length < material.maps.length" type="button" class="linkish small showmore" @click="unfold('maps')">
        Show {{ material.maps.length - LIST_PEEK }} more
      </button>
    </section>

    <section v-if="partOf.length">
      <h3>{{ partOf.length === 1 ? 'Material' : 'Materials' }}</h3>
      <ul class="links">
        <li v-for="m in partOf" :key="m.id">
          <button type="button" class="link" style="--kind: var(--k-material)" @click="$emit('open', m.id)">
            <span class="lthumb checker">
              <img v-if="m.has_thumb" :src="thumbUrl(m)" alt="" loading="lazy" />
              <KindGlyph v-else kind="material" :size="22" />
            </span>
            <span class="lname">{{ m.name }}</span>
            <span class="lrel">{{ roleOfHere(m) }}</span>
          </button>
        </li>
      </ul>
    </section>

    <section v-for="g in linkGroups" :key="g.title">
      <h3>{{ g.title }}</h3>
      <ul class="links">
        <li v-for="l in peek(g.title, g.items)" :key="l.id + l.rel">
          <button type="button" class="link" :style="{ '--kind': `var(--k-${l.kind})` }" @click="$emit('open', l.id)">
            <span class="lthumb checker">
              <img v-if="l.has_thumb" :src="thumbUrl(l)" :class="{ glyphs: l.kind === 'font' }" alt="" loading="lazy" />
              <KindGlyph v-else :kind="l.kind" :size="22" />
            </span>
            <span class="lname">{{ l.name }}</span>
            <span class="lrel">{{ linkLabel(l) }}<template v-if="l.size != null"> · {{ humanBytes(l.size) }}</template></span>
          </button>
        </li>
      </ul>
      <button v-if="peek(g.title, g.items).length < g.items.length" type="button" class="linkish small showmore" @click="unfold(g.title)">
        Show {{ g.items.length - LIST_PEEK }} more
      </button>
    </section>

    <section v-if="family">
      <h3>Variants ({{ family.size }} takes)</h3>
      <ul class="links">
        <li v-for="l in peek('variants', family.members)" :key="l.id">
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
      <button v-if="peek('variants', family.members).length < family.members.length" type="button" class="linkish small showmore" @click="unfold('variants')">
        Show {{ family.members.length - LIST_PEEK }} more
      </button>
    </section>

    <section v-if="duplicates.length">
      <h3>Identical copies</h3>
      <p class="legend dupnote">The same bytes are stored once; these packs ship this file too.</p>
      <ul class="links">
        <li v-for="d in peek('copies', duplicates)" :key="d.id">
          <button type="button" class="link dup" @click="$emit('open', d.id)">
            <span class="lname">{{ d.pack_name }}</span>
            <span class="lrel">{{ d.path }}</span>
          </button>
        </li>
      </ul>
      <button v-if="peek('copies', duplicates).length < duplicates.length" type="button" class="linkish small showmore" @click="unfold('copies')">
        Show {{ duplicates.length - LIST_PEEK }} more
      </button>
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
        <dd>{{ humanBytes(material ? mapsBytes : asset.size) }}</dd>
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
      <template v-if="isMaterial && needs.length">
        <a class="btn primary" :href="bundleUrl(asset)">Download all {{ needs.length }} {{ needs.length === 1 ? 'map' : 'maps' }} (.zip)
          <span class="bytes">{{ humanBytes(bundleBytes) }}</span>
        </a>
      </template>
      <template v-else-if="needs.length || anims.length">
        <a class="btn primary" :href="bundleUrl(asset, withAnimations)">
          Download with {{ linkedCount }} linked {{ linkedCount === 1 ? 'file' : 'files' }} (.zip)
          <span class="bytes">{{ humanBytes(bundleBytes) }}</span>
        </a>
        <label v-if="anims.length" class="animcheck"><input v-model="withAnimations" type="checkbox" /> Include animations</label>
        <a class="btn" :href="fileUrl(asset, true)">This file only <span class="bytes">{{ humanBytes(asset.size) }}</span></a>
      </template>
      <a v-else class="btn primary" :href="fileUrl(asset, true)">Download <span class="bytes">{{ humanBytes(asset.size) }}</span></a>
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
      <button v-if="!asset.archived_at" class="btn danger archive" type="button" title="Out of search until it's restored or deleted for good, from the Archive" @click="emit('archive')">Archive</button>
    </footer>
  </aside>
</template>

<style scoped>
.archive { margin-left: auto; }
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
/* Images open in the inspector. */
.zoomable { position: relative; width: 100%; padding: 0; cursor: zoom-in; color: inherit; }
.zoomable img { transition: scale var(--dur-3) var(--ease-out); }
.zoomable:hover img { scale: 1.03; }
.zoombadge {
  position: absolute; right: 0.5rem; bottom: 0.5rem; display: grid; place-items: center; width: 1.9rem; height: 1.9rem;
  border-radius: 50%; background: color-mix(in srgb, var(--panel) 85%, transparent); box-shadow: var(--elev-2);
  opacity: 0; scale: 0.8; transition: opacity var(--dur-2) var(--ease-out), scale var(--dur-2) var(--ease-pop);
}
.zoomable:hover .zoombadge, .zoomable:focus-visible .zoombadge { opacity: 1; scale: 1; }
/* Sounds: the waveform is the scrubber. Dim bars underneath, full-colour ones revealed up to the playhead. */
.preview.audio {
  display: flex; flex-direction: column; aspect-ratio: auto; place-items: stretch; gap: 0.4rem; padding: 0.75rem 0.75rem 0.5rem;
  background: var(--bg);
}
.scrub { position: relative; height: 8.5rem; cursor: pointer; touch-action: none; border-radius: 3px; --p: 0; }
.scrub:focus-visible { outline-offset: 4px; }
.wave { position: absolute; inset: 0; width: 100%; height: 100%; color: var(--k-audio); opacity: 0.32; }
.wave.top { opacity: 1; clip-path: inset(0 calc((1 - var(--p)) * 100%) 0 0); }
.scrub.flat::before, .scrub.flat::after {
  content: ''; position: absolute; left: 0; top: calc(50% - 2px); height: 4px; border-radius: 2px; background: var(--k-audio);
}
.scrub.flat::before { right: 0; opacity: 0.25; }
.scrub.flat::after { width: calc(var(--p) * 100%); }
.head {
  position: absolute; top: -4px; bottom: -4px; left: calc(var(--p) * 100%); width: 2px; margin-left: -1px;
  background: var(--text); border-radius: 1px; pointer-events: none;
}
.hover {
  position: absolute; top: 0; bottom: 0; left: calc(var(--h) * 100%); width: 1px;
  background: color-mix(in srgb, var(--text) 45%, transparent); pointer-events: none;
}
.hover span {
  position: absolute; bottom: calc(100% + 2px); left: 50%; translate: -50% 0; padding: 0 0.3rem; border-radius: 3px;
  background: var(--raised); box-shadow: var(--elev-1); font-size: 0.7rem; white-space: nowrap;
}
.transport { display: flex; align-items: center; gap: 0.6rem; }
.play {
  display: grid; place-items: center; width: 2.25rem; height: 2.25rem; border-radius: 50%; cursor: pointer;
  border: 0; background: var(--k-audio); color: var(--bg); box-shadow: var(--elev-1);
  transition: scale var(--dur-2) var(--ease-pop), box-shadow var(--dur-2) var(--ease-out);
}
.play:hover { scale: 1.06; box-shadow: var(--elev-2); }
.play:active { scale: 0.92; transition-duration: 60ms; }
.time { font-size: 0.78rem; font-variant-numeric: tabular-nums; }
.time .of { color: var(--faint); }
.wavenote { margin-left: auto; font-size: 0.72rem; color: var(--faint); }
.desc { margin: 0; color: var(--muted); max-width: 60ch; white-space: pre-line; }
.desc.mine { color: var(--text); }
.descs { display: grid; gap: 0.6rem; justify-items: start; }
.descsrc { display: grid; gap: 0.15rem; }
.srclabel { font-size: 0.72rem; color: var(--faint); }
.desc.store { display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 4; line-clamp: 4; overflow: hidden; }
.desc.store.open { display: block; }
.descedit { display: grid; gap: 0.4rem; width: 100%; max-width: 60ch; }
.descedit textarea { resize: vertical; font: inherit; }
.descbtns { display: flex; gap: 0.4rem; align-items: center; }
.hint { font-size: 0.75rem; color: var(--faint); }
.linkish.small { font-size: 0.8rem; }
.showmore { margin-top: 0.45rem; color: var(--muted); }
.showmore:hover { color: var(--text); }
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
.btn .bytes { font-weight: 400; opacity: 0.75; font-variant-numeric: tabular-nums; }
/* The download actions stay in reach while the panel scrolls. */
footer {
  position: sticky; bottom: -1.4rem; z-index: 1; display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center;
  margin: auto -1.2rem -1.4rem; padding: 0.75rem 1.2rem 1rem; background: var(--panel); border-top: 1px solid var(--line);
}
.looks { display: flex; gap: 0.6rem; align-items: center; }
.palette {
  flex: 1; display: flex; height: 1.9rem; min-width: 0; border-radius: 6px; overflow: hidden;
  box-shadow: inset 0 0 0 1px var(--line);
}
.swatch {
  flex: var(--g) 1 0; min-width: 0.6rem; border: 0; padding: 0; cursor: pointer;
  transition: flex-grow var(--dur-2) var(--ease-out), box-shadow var(--dur-1);
}
.swatch:hover, .swatch:focus-visible { flex-grow: calc(var(--g) + 0.12); box-shadow: inset 0 0 0 2px var(--panel); outline: none; }
.alike { white-space: nowrap; flex: none; display: inline-flex; gap: 0.4rem; align-items: center; }
.looks:not(:has(.palette)) .alike { margin-right: auto; }
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
