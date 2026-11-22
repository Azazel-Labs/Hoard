<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import {
  api, bundleUrl, fileUrl, liteUrl, thumbUrl, type Asset, type Collection, type CollectionRef, type ProjectUse,
} from '../api'
import { humanBytes } from '../format'
import { looksAlike } from '../kinds'
import CollectionPicker from './CollectionPicker.vue'
import { playableUrl } from '../player'
import AssetLinks from './AssetLinks.vue'
import AssetDescriptions from './AssetDescriptions.vue'
import AssetFields from './AssetFields.vue'
import TagInput from './TagInput.vue'
import AssetHistory from './AssetHistory.vue'
import AssetOrigin from './AssetOrigin.vue'
import AudioPreview from './AudioPreview.vue'
import DownloadAs from './DownloadAs.vue'
import KindGlyph from './KindGlyph.vue'
import TagChip from './TagChip.vue'
import { copyText, daysAgo, showOk, swipeToClose } from '../ui'
import { may, mayEdit } from '../auth'
import { canStar, mine, toggleFavorite } from '../mine'
import { MAX_STORE_FILE, TEXT_EXTS } from '../storeFormats'
import { lazy } from '../lazy'
import { t } from '../i18n'
import { isSkeletonAsset, isLottieAsset, isModelAsset, isRiveAsset, isTextAsset, takeViewRequest, viewRequest, viewerFor, type ViewerKind } from '../viewers'
import { isMaterialAsset } from '../materials'
import { withThumb } from '../browserThumbs'
import { animsOf, needsOf } from '../assetLinks'
import { fileFacts } from '../fileFacts'
import { animatedAseprite, animatedSrc, PLAY_IN_PANEL, spriteSheet } from '../sprites'

// three.js is big; only load it when someone opens a model.
const ModelViewer = lazy(() => import('./ModelViewer.vue'), () => t('asset-lazy-3d-viewer'), 'model-slot')
const ModelInspector = lazy(() => import('./ModelInspector.vue'), () => t('asset-lazy-3d-viewer'), 'overlay')
const TextPreview = lazy(() => import('./TextPreview.vue'), () => t('asset-lazy-text'))
const TextViewer = lazy(() => import('./TextViewer.vue'), () => t('asset-lazy-text-viewer'), 'overlay')
const ImageViewer = lazy(() => import('./ImageViewer.vue'), () => t('asset-lazy-image-viewer'), 'overlay')
const MaterialPreview = lazy(() => import('./MaterialPreview.vue'), () => t('asset-lazy-material-preview'), 'model-slot')
const MaterialViewer = lazy(() => import('./MaterialViewer.vue'), () => t('asset-lazy-material-viewer'), 'overlay')
const SpritePlayer = lazy(() => import('./SpritePlayer.vue'), () => t('asset-lazy-sprite-player'))
const SheetView = lazy(() => import('./SheetView.vue'), () => t('asset-lazy-sheet-view'))
const LottieViewer = lazy(() => import('./LottieViewer.vue'), () => t('asset-lazy-sprite-player'))
const SkeletonViewer = lazy(() => import('./SkeletonViewer.vue'), () => t('asset-lazy-sprite-player'))
const RiveViewer = lazy(() => import('./RiveViewer.vue'), () => t('asset-lazy-sprite-player'))

const props = defineProps<{
  asset: Asset
  /** Where this asset sits in the current results, for previous/next. */
  position: { index: number; total: number; hasPrev: boolean; hasNext: boolean } | null
  collections: Collection[]
  /** Shown over the whole results area: the preview large on the left, the details beside it. */
  page?: boolean
  /** False while `asset` is still the search hit it was opened from, before its full record (links, copies,
   *  formats) arrives. */
  full?: boolean
}>()
const emit = defineEmits<{
  close: []; updated: [Asset]; tag: [string, boolean?]; error: [string]; open: [string]; step: [number]
  /** Move it to the archive (out of search until it's restored or deleted for good). */
  archive: []
  /** Add to a collection by id, or (with a name) create one first. */
  collect: [id: string, create?: string]
  uncollect: [id: string]
  browseCollection: [CollectionRef]
  /** Show what this project (its name) has. */
  browseProject: [string]
  browsePack: [{ id: string; name: string }]
  /** The pack's name alt-clicked: search without it. */
  leavePack: [string]
  /** Find assets that look like this one. */
  similar: []
  /** Find assets with much of this color (hex). */
  color: [string]
  /** Switch between the side panel and the full page. */
  layout: []
  /** Open the format wizard on this file (Settings → Store files). */
  teach: []
  /** Browse this folder of its pack (`a/b`), as it was on disk. */
  folder: [string]
}>()
/** The folders the file sits in, from the pack's top, each with the path that opens it; then the file's name. */
const pathParts = computed(() => {
  const parts = props.asset.path.split('/')
  const file = parts.pop() ?? ''
  return { folders: parts.map((name, i) => ({ name, path: parts.slice(0, i + 1).join('/') })), file }
})
const palette = computed(() => props.asset.palette ?? [])
/** A text file that may be a store's, which a signed-in user can teach Hoard to read. */
/** May change it (a guest is asked to sign in when they try). */
const editor = computed(() => mayEdit())
const starred = computed(() => mine.favorites.has(props.asset.id))
const teachable = computed(
  () => may('admin') && TEXT_EXTS.includes(props.asset.ext.toLowerCase()) && props.asset.size <= MAX_STORE_FILE,
)
const alikeable = computed(() => looksAlike(props.asset))
const inCollections = computed(() => props.asset.collections ?? [])

/** The projects whose last hoard sync had this asset. */
const usedIn = ref<ProjectUse[]>([])
/** Whether `usedIn` has been asked for this asset yet (answered or not). */
const usageKnown = ref(false)
watch(
  () => props.asset.id,
  async (id) => {
    usedIn.value = []
    usageKnown.value = false
    try {
      const uses = (await api.usage({ assets: [id] })).projects
      if (props.asset.id === id) usedIn.value = uses
    } catch {
      // Not knowing where it's used isn't worth an error.
    }
    if (props.asset.id === id) usageKnown.value = true
  },
  { immediate: true },
)
/** Everything below the collections waits for the full record and the projects using it, then shows at once, so
 *  sections arriving one by one don't push each other (and what's being read) down the panel. */
const settled = computed(() => props.full !== false && usageKnown.value)


const withAnimations = ref(false)
const isModel = computed(() => isModelAsset(props.asset))
const isText = computed(() => isTextAsset(props.asset))
const needs = computed(() => needsOf(props.asset))
// A material made from texture maps lists its maps by role instead of as "Needs"; a texture or model that belongs
// to one names it.
const isMaterial = computed(() => isMaterialAsset(props.asset))
const material = computed(() => props.asset.material ?? null)
const partOf = computed(() => props.asset.materials ?? [])
const mapsBytes = computed(() => material.value?.maps.reduce((n, m) => n + m.size, 0) ?? 0)
const anims = computed(() => animsOf(props.asset))
/** For an animation: the character it plays on (its skeleton's, else one its pack ships it for, else the first that
 *  fits), which its zip can take along with what that needs. */
const character = computed(() => {
  if (props.asset.kind !== 'animation') return null
  const plays = props.asset.plays_on
  if (plays) return { id: plays.id, name: plays.pack_id === props.asset.pack_id ? plays.name : `${plays.name} · ${plays.pack_name}` }
  const own = props.asset.linked_from.find((l) => l.rel === 'animation' && l.kind === 'model')
  if (own) return { id: own.id, name: own.name }
  const fit = props.asset.fits?.find((f) => f.kind === 'model')
  if (!fit) return null
  return { id: fit.id, name: fit.pack_id === props.asset.pack_id ? fit.name : `${fit.name} · ${fit.pack_name}` }
})
const withCharacter = ref(false)
/** The character's full record, for what it needs: fetched once it's asked for. */
const characterFull = ref<Asset | null>(null)
watch([withCharacter, () => character.value?.id], async ([on, id]) => {
  characterFull.value = null
  if (!on || !id) return
  const a = await api.asset(id).catch(() => null)
  if (withCharacter.value && character.value?.id === id) characterFull.value = a
})
/** The character's files the zip adds: itself and what it needs. */
const characterFiles = computed(() =>
  withCharacter.value && characterFull.value ? [characterFull.value, ...needsOf(characterFull.value)] : [],
)
// A material made from maps comes as its maps, not as a file of its own.
const linkedCount = computed(() => {
  const files = new Set<string>()
  for (const l of needs.value) {
    const m = partOf.value.find((x) => x.id === l.id)
    if (m) m.maps.forEach((x) => files.add(x.id))
    else files.add(l.id)
  }
  for (const l of characterFiles.value) files.add(l.id)
  return files.size + (withAnimations.value ? anims.value.length : 0)
})
/** What the zip holds before compression: this file and each linked one once (sizes the server didn't give count 0). A
 * material counts as its maps. */
const bundleBytes = computed(() => {
  const seen = new Map<string, number>()
  for (const l of [...needs.value, ...(withAnimations.value ? anims.value : []), ...characterFiles.value]) seen.set(l.id, l.size ?? 0)
  for (const m of partOf.value) {
    if (!seen.delete(m.id)) continue
    for (const x of m.maps) seen.set(x.id, x.size)
  }
  return props.asset.size + [...seen.values()].reduce((s, n) => s + n, 0)
})

// Variant families: tag edits can fan out to every take.
const family = computed(() => props.asset.family ?? null)
const applyToFamily = ref(true)
const showViews = ref(false)
const hasViews = computed(() => props.asset.derivatives.includes('views'))

const editingName = ref(false)
const nameEl = ref<HTMLElement | null>(null)
const copied = ref('')
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

const isImage = computed(() => (props.asset.kind === 'texture' || props.asset.kind === 'vector') && props.asset.has_thumb)
// The panel's picture: the thumbnail stretched to the size the picture will have, then the light preview once it's in.
const sharpSrc = ref('')
const sharpening = ref(false)
watch(
  () => (isImage.value && !animatedSrc(props.asset, PLAY_IN_PANEL) ? thumbUrl(props.asset) : ''),
  (key) => {
    sharpSrc.value = ''
    sharpening.value = !!key
    if (!key) return
    const url = liteUrl(props.asset)
    const img = new Image()
    img.src = url
    const current = () => key === thumbUrl(props.asset)
    img.decode().then(
      () => {
        if (!current()) return
        sharpSrc.value = url
        sharpening.value = false
      },
      () => current() && (sharpening.value = false),
    )
  },
  { immediate: true },
)
/** Keeps the stretched thumbnail within the picture's own pixels, so the swap doesn't change its size. */
const thumbBox = computed(() => {
  const dim = (k: string) => {
    const v = props.asset.meta?.[k]
    return typeof v === 'number' && v > 0 ? `${v}px` : 'none'
  }
  return { maxWidth: dim('width'), maxHeight: dim('height') }
})
const isSheet = computed(() => props.asset.kind === 'sheet' && props.asset.derivatives.includes('sheet'))
// Aseprite animations play from the start; a sprite sheet does when asked.
const spritePlayable = computed(() => isImage.value && (animatedAseprite(props.asset) || spriteSheet(props.asset)))
const animate = ref(false)
watch(() => props.asset.id, () => (animate.value = animatedAseprite(props.asset)), { immediate: true })
const isAudio = computed(() => playableUrl(props.asset) !== null)

const pixelated = computed(() => props.asset.tags.some((t) => t.tag === 'style:pixel-art'))

const visibleTags = computed(() =>
  props.asset.tags.filter((t) => !t.tag.startsWith('kind:') && !t.tag.startsWith('format:')),
)

const metaRows = computed(() => fileFacts(props.asset))

const tomlSnippet = computed(
  () => `[[select]]\nids = ["${props.asset.id}"]  # ${props.asset.name}\ndest = "${props.asset.category ?? props.asset.kind}"`,
)

/** What the copy buttons put on the clipboard, for the "Copied …" note. */
type Copyable = 'hash' | 'toml' | 'id' | 'link'
function copiedNote(what: string): string {
  switch (what) {
    case 'link': return t('asset-copied-link')
    case 'hash': return t('asset-copied-hash')
    case 'toml': return t('asset-copied-toml')
    case 'id': return t('asset-copied-id')
    default: return ''
  }
}

/**
 * Shares a link that opens this asset: the share sheet on a phone or tablet, the clipboard elsewhere (where a share
 * sheet is more in the way than copying). Whoever follows it signs in to Hoard first.
 */
async function share() {
  const url = `${location.origin}${location.pathname}#asset=${props.asset.id}`
  if (navigator.share && matchMedia('(pointer: coarse)').matches) {
    try {
      await navigator.share({ title: props.asset.name, url })
      return
    } catch (e) {
      if ((e as DOMException).name === 'AbortError') return
    }
  }
  await copy(url, 'link')
}

async function copy(text: string, what: Copyable) {
  if (await copyText(text)) {
    copied.value = what
    setTimeout(() => (copied.value = ''), 1500)
  } else {
    emit('error', t('asset-copy-blocked'))
  }
}

async function patch(body: Parameters<typeof api.patchAsset>[1]): Promise<boolean> {
  // What the variants share goes to all of them (the name, rating, fields and Reviewed are each one's own).
  const shared = body.add_tags || body.remove_tags || body.accept_tags || body.accept_all_tags || body.unreject_tags
    || body.category !== undefined || body.description !== undefined || body.license !== undefined || body.source_url !== undefined
  if (family.value && applyToFamily.value && shared) {
    body = { ...body, apply_to_family: true }
  }
  // Tag edits show at once and are rolled back if the server says no.
  const before = props.asset
  const optimistic = !!(body.add_tags || body.remove_tags || body.accept_tags || body.accept_all_tags || body.reviewed !== undefined)
  if (optimistic) {
    const drop = new Set(body.remove_tags ?? [])
    const keep = new Set(body.accept_tags ?? [])
    const tags = before.tags
      .filter((t) => !drop.has(t.tag))
      .map((t) => (keep.has(t.tag) || (body.accept_all_tags && guess(t.source)) ? { ...t, source: 'human' as const, confidence: 1 } : t))
    for (const tag of body.add_tags ?? []) {
      if (!tags.some((t) => t.tag === tag)) tags.push({ tag, source: 'human', confidence: 1 })
    }
    const reviewed_at = body.reviewed === undefined ? before.reviewed_at : body.reviewed ? new Date().toISOString() : null
    emit('updated', { ...before, tags, reviewed_at })
  }
  try {
    emit('updated', await api.patchAsset(props.asset.id, body))
    return true
  } catch (e) {
    if (optimistic) emit('updated', before)
    emit('error', t('asset-save-error', { error: (e as Error).message }))
    return false
  }
}

/** A tag a rule or the AI put on, rather than a person or the file itself. */
function guess(source: string): boolean {
  return source === 'rule' || source === 'ai'
}
const guesses = computed(() => visibleTags.value.filter((t) => guess(t.source)))
const rejected = computed(() => props.asset.rejected_tags ?? [])

async function removeTag(tag: string) {
  const was = props.asset.tags.find((t) => t.tag === tag)
  if (!(await patch({ remove_tags: [tag] }))) return
  // A guess turned down is remembered: undoing it puts the guess back as it was rather than making it a person's tag.
  const undo = was && guess(was.source) ? { unreject_tags: [tag] } : { add_tags: [tag] }
  showOk(t('asset-tag-removed', { tag }), { label: t('asset-undo'), run: () => void patch(undo) })
}
async function acceptTags(tags: string[] | 'all') {
  const n = tags === 'all' ? guesses.value.length : tags.length
  if (!(await patch(tags === 'all' ? { accept_all_tags: true } : { accept_tags: tags }))) return
  if (n > 1) showOk(t('asset-tags-accepted', { count: n }))
}
async function restoreTag(tag: string) {
  if (await patch({ unreject_tags: [tag] })) showOk(t('asset-tag-restored', { tag }))
}

const reviewed = computed(() => !!props.asset.reviewed_at)
async function toggleReviewed() {
  const to = !reviewed.value
  if (await patch({ reviewed: to })) {
    showOk(to ? t('asset-reviewed-on') : t('asset-reviewed-off'), { label: t('asset-undo'), run: () => void patch({ reviewed: !to }) })
  }
}

// Renaming edits the heading itself, so nothing moves: the text just becomes editable where it is.
function startRename(ev?: MouseEvent) {
  if (editingName.value || !editor.value) return
  editingName.value = true
  void nextTick(() => {
    const el = nameEl.value
    const sel = getSelection()
    if (!el || !sel) return
    el.focus()
    // A click puts the caret where it landed; from the keyboard, the whole name is selected to type over.
    const at = ev ? caretAt(ev.clientX, ev.clientY) : null
    const range = document.createRange()
    if (at && el.contains(at.node)) range.setStart(at.node, at.offset)
    else range.selectNodeContents(el)
    sel.removeAllRanges()
    sel.addRange(range)
  })
}
function caretAt(x: number, y: number): { node: Node; offset: number } | null {
  if (document.caretPositionFromPoint) {
    const p = document.caretPositionFromPoint(x, y)
    return p && { node: p.offsetNode, offset: p.offset }
  }
  const r = document.caretRangeFromPoint?.(x, y)
  return r ? { node: r.startContainer, offset: r.startOffset } : null
}
async function endRename(save: boolean) {
  const el = nameEl.value
  if (!editingName.value || !el) return
  editingName.value = false
  getSelection()?.removeAllRanges()
  const { id, name: was } = props.asset
  const name = (el.textContent ?? '').replace(/\s+/g, ' ').trim()
  const keep = save && name !== '' && name !== was
  el.textContent = keep ? name : was
  if (keep && !(await patch({ name })) && props.asset.id === id) el.textContent = props.asset.name
}
function onNameKey(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault()
    if (editingName.value) void endRename(true)
    else startRename()
  } else if (e.key === 'Escape' && editingName.value) {
    // Gives up the rename; the panel stays open.
    e.preventDefault()
    e.stopPropagation()
    void endRename(false)
  }
}

async function addTags(tags: string[]) {
  if (!(await patch({ add_tags: tags })) || tags.length < 2) return
  showOk(t('asset-tags-added', { count: tags.length }), { label: t('asset-undo'), run: () => void patch({ remove_tags: tags }) })
}
/** Backspace in the empty tag box takes the last tag off (with Undo). */
function removeLastTag() {
  const last = visibleTags.value.at(-1)
  if (last) void removeTag(last.tag)
}

// The tags the most alike assets share that this one lacks, to add with a click.
const fromSimilar = ref<string[]>([])
let similarAsk: AbortController | null = null
watch(
  () => [props.asset.id, editor.value && alikeable.value] as const,
  ([id, want]) => {
    similarAsk?.abort()
    fromSimilar.value = []
    if (!want) return
    const ask = (similarAsk = new AbortController())
    api
      .search({ similar: id, limit: 12, facets: false }, ask.signal)
      .then((r) => {
        const near = r.items.filter((a) => a.id !== id)
        const counts = new Map<string, number>()
        for (const a of near) {
          for (const x of a.tags) {
            if (!x.tag.startsWith('kind:') && !x.tag.startsWith('format:')) counts.set(x.tag, (counts.get(x.tag) ?? 0) + 1)
          }
        }
        // Shared by a quarter of them at least, so one odd neighbour doesn't suggest anything.
        const least = Math.max(2, Math.ceil(near.length / 4))
        fromSimilar.value = [...counts].filter(([, n]) => n >= least).sort((x, y) => y[1] - x[1]).map(([tag]) => tag)
      })
      .catch(() => {})
  },
  { immediate: true },
)
onBeforeUnmount(() => similarAsk?.abort())
const similarTags = computed(() => {
  const skip = new Set([...props.asset.tags.map((x) => x.tag), ...rejected.value])
  return fromSimilar.value.filter((tag) => !skip.has(tag)).slice(0, 8)
})

// On a phone the panel covers the screen: swipe it away to the right.
const root = ref<HTMLElement | null>(null)
let unswipe = () => {}
onMounted(() => {
  if (root.value) unswipe = swipeToClose(root.value, 1, () => emit('close'), true)
})
onBeforeUnmount(() => unswipe())
</script>

<template>
  <aside ref="root" class="detail" :class="{ page }" :style="{ '--kind': `var(--k-${asset.kind})` }" :aria-label="t('asset-details')">
    <header>
      <div class="topbar">
        <p class="where">
          <span>{{ asset.category ?? asset.kind }}</span>
          <span class="sep" aria-hidden="true">{{ t('asset-in-pack') }}</span>
          <button
            type="button"
            class="pack linkish"
            :title="t('app-alt-leave-out')"
            @click="$event.altKey ? $emit('leavePack', asset.pack_name) : $emit('browsePack', { id: asset.pack_id, name: asset.pack_name })"
          >{{ asset.pack_name }}</button>
        </p>
        <div class="nav">
          <!-- Stepping through results by button is for the full page; the side panel steps with the arrow keys. -->
          <template v-if="page">
            <button class="btn ghost icon" type="button" :aria-label="t('asset-previous')" :disabled="!position?.hasPrev" @click="$emit('step', -1)">
              <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M10 3 5 8l5 5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
            </button>
            <span v-if="position" class="pos mono">{{ position.index + 1 }}/{{ position.total }}</span>
            <button class="btn ghost icon" type="button" :aria-label="t('asset-next')" :disabled="!position?.hasNext" @click="$emit('step', 1)">
              <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m6 3 5 5-5 5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
            </button>
          </template>
          <button
            v-if="editor"
            class="btn ghost icon reviewed"
            :class="{ on: reviewed }"
            type="button"
            :aria-pressed="reviewed"
            :aria-label="t('asset-reviewed')"
            :title="reviewed ? t('asset-reviewed-on-title') : t('asset-reviewed-off-title')"
            @click="toggleReviewed"
          >
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
              <circle cx="8" cy="8" r="6.2" :fill="reviewed ? 'currentColor' : 'none'" stroke="currentColor" stroke-width="1.4" />
              <path d="m5.2 8.2 2 2 3.7-4" fill="none" :stroke="reviewed ? 'var(--panel)' : 'currentColor'" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
          <button
            v-if="canStar()"
            class="btn ghost icon star"
            :class="{ on: starred }"
            type="button"
            :aria-pressed="starred"
            :aria-label="t('asset-favorite')"
            :title="starred ? t('asset-favorite-on-title') : t('asset-favorite-off-title')"
            @click="toggleFavorite(asset.id)"
          >
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M8 1.9l1.8 3.7 4.1.6-3 2.9.7 4.1L8 11.3l-3.6 1.9.7-4.1-3-2.9 4.1-.6z" :fill="starred ? 'currentColor' : 'none'" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" /></svg>
          </button>
          <span class="copywrap">
            <button
              class="btn ghost icon share"
              type="button"
              :aria-label="t('asset-share')"
              :title="t('asset-share-title')"
              @click="share"
            >
              <svg
                viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"
                fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"
              >
                <path d="M8 10V1.75M5 4.5 8 1.5l3 3" />
                <path d="M5.5 7H4a1 1 0 0 0-1 1v5.25a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V8a1 1 0 0 0-1-1h-1.5" />
              </svg>
            </button>
            <span v-if="copied === 'link'" class="copytip below" aria-hidden="true">{{ copiedNote('link') }}</span>
          </span>
          <button
            class="btn ghost icon layout"
            type="button"
            :aria-label="page ? t('asset-layout-panel') : t('asset-layout-page')"
            :title="page ? t('asset-layout-panel') : t('asset-layout-page-title')"
            @click="$emit('layout')"
          >
            <svg v-if="page" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="1.75" y="2.75" width="12.5" height="10.5" rx="1.5" /><path d="M9.5 2.75v10.5" /></svg>
            <svg v-else viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="1.75" y="2.75" width="12.5" height="10.5" rx="1.5" /><path d="M5 9.5 7.5 7 10 9.5" transform="rotate(45 7.5 8)" /></svg>
          </button>
          <button class="btn ghost icon close" type="button" :aria-label="t('asset-close')" @click="$emit('close')">
            <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
          </button>
        </div>
      </div>
      <h2
        ref="nameEl"
        class="name"
        :class="{ editing: editingName }"
        tabindex="0"
        spellcheck="false"
        :contenteditable="editingName ? 'plaintext-only' : undefined"
        :title="editingName || !editor ? undefined : t('asset-rename-title')"
        :aria-label="editingName ? t('asset-name') : undefined"
        @click="startRename($event)"
        @keydown="onNameKey"
        @blur="endRename(true)"
        v-text="asset.name"
      />
    </header>

    <!-- Two columns on the full page (the preview stays in view while the details scroll); in the side panel these
         wrappers step aside and everything is one column. -->
    <div class="stagecol">
    <div v-if="isModel && showViews" class="preview checker">
      <img :src="`/api/assets/${asset.id}/derivative/views`" :alt="t('asset-views-alt')" />
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
    <SheetView v-else-if="isSheet" :asset="asset" :pixelated="pixelated" />
    <SpritePlayer v-else-if="spritePlayable && animate" :asset="asset" :pixelated="pixelated" />
    <LottieViewer v-else-if="isLottieAsset(asset)" :asset="asset" @thumb="(r) => emit('updated', withThumb(asset, r))" />
    <SkeletonViewer v-else-if="isSkeletonAsset(asset)" :asset="asset" @thumb="(r) => emit('updated', withThumb(asset, r))" />
    <RiveViewer v-else-if="isRiveAsset(asset)" :asset="asset" @thumb="(r) => emit('updated', withThumb(asset, r))" />
    <button
      v-else-if="isImage"
      type="button"
      class="preview checker zoomable"
      :aria-label="t('asset-inspect-aria', { name: asset.name })"
      :title="t('asset-inspect-title')"
      @click="inspecting = 'image'"
    >
      <img v-if="animatedSrc(asset, PLAY_IN_PANEL)" :src="animatedSrc(asset, PLAY_IN_PANEL)!" alt="" :class="{ pixelated }" />
      <img v-else-if="sharpSrc" :src="sharpSrc" alt="" :class="{ pixelated }" />
      <img v-else :src="thumbUrl(asset)" alt="" class="stretched" :class="{ pixelated }" :style="pixelated ? undefined : thumbBox" />
      <span v-if="sharpening" class="spinner" aria-hidden="true" />
      <span class="zoombadge" aria-hidden="true">
        <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round">
          <circle cx="7" cy="7" r="4.5" /><path d="m10.5 10.5 3.5 3.5M5 7h4M7 5v4" />
        </svg>
      </span>
    </button>
    <AudioPreview v-else-if="isAudio" :asset="asset" @error="(m) => emit('error', m)" />
    <TextPreview v-else-if="isText" :asset="asset" @expand="inspecting = 'text'" />
    <div v-else class="preview checker">
      <img v-if="asset.has_thumb" :src="thumbUrl(asset)" alt="" :class="{ pixelated, glyphs: asset.kind === 'font' }" />
      <KindGlyph v-else :kind="asset.kind" :size="72" />
    </div>
    <ImageViewer v-if="inspecting === 'image'" :asset="asset" :pixelated="pixelated" @close="inspecting = ''" />
    <ModelInspector v-else-if="inspecting === 'model'" :asset="asset" @close="inspecting = ''" />
    <TextViewer v-else-if="inspecting === 'text'" :asset="asset" @close="inspecting = ''" />
    <MaterialViewer v-else-if="inspecting === 'material' && material" :asset="asset" :material="material" @close="inspecting = ''" />
    <label v-if="spritePlayable" class="animcheck">
      <input v-model="animate" type="checkbox" /> {{ t('asset-play-animation') }}
    </label>
    <label v-if="isModel && hasViews" class="animcheck">
      <input v-model="showViews" type="checkbox" /> {{ t('asset-show-views') }}
    </label>

    <div v-if="alikeable || palette.length" class="looks">
      <div v-if="palette.length" class="palette" role="group" :aria-label="t('asset-palette')">
        <button
          v-for="c in palette"
          :key="c.hex"
          type="button"
          class="swatch"
          :style="{ background: c.hex, '--g': c.share }"
          :title="t('asset-swatch-title', { hex: c.hex, percent: Math.round(c.share * 100) })"
          :aria-label="t('asset-swatch-aria', { hex: c.hex, percent: Math.round(c.share * 100) })"
          @click="emit('color', c.hex)"
        />
      </div>
      <button v-if="alikeable" type="button" class="btn alike" @click="emit('similar')">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round">
          <rect x="1.8" y="5.5" width="7.5" height="7.5" rx="1.6" />
          <rect x="6.7" y="2.5" width="7.5" height="7.5" rx="1.6" fill="var(--panel)" />
        </svg>
        {{ t('asset-more-like-this') }}
      </button>
    </div>
    </div>

    <div class="infocol">
    <AssetDescriptions
      :asset="asset"
      :editor="editor"
      :family-size="family && applyToFamily ? family.size : null"
      :save="(description) => patch({ description })"
    />

    <AssetFields :asset="asset" :editor="editor" :save="patch" />

    <section>
      <div class="taghead">
        <h3>{{ t('asset-tags') }}</h3>
        <button
          v-if="editor && guesses.length > 1"
          type="button"
          class="linkish small acceptall"
          :title="t('asset-tags-accept-all-title')"
          @click="acceptTags('all')"
        >{{ t('asset-tags-accept-all', { count: guesses.length }) }}</button>
      </div>
      <TransitionGroup tag="div" name="chip" class="tags">
        <TagChip
          v-for="vt in visibleTags"
          :key="vt.tag"
          :tag="vt.tag"
          :source="vt.source"
          :removable="editor"
          :acceptable="editor && guess(vt.source)"
          @click="$emit('tag', vt.tag, $event.altKey)"
          @remove="removeTag(vt.tag)"
          @accept="acceptTags([vt.tag])"
        />
      </TransitionGroup>
      <p v-if="editor && rejected.length" class="rejected">
        <span class="lbl">{{ t('asset-tags-rejected') }}</span>
        <button
          v-for="tag in rejected"
          :key="tag"
          type="button"
          class="linkish rej"
          :title="t('asset-tag-restore-title', { tag })"
          @click="restoreTag(tag)"
        >{{ tag }}</button>
      </p>
      <div v-if="editor" class="addtag">
        <label class="visually-hidden" for="new-tag">{{ t('asset-tag-add-label') }}</label>
        <TagInput
          id="new-tag"
          :have="asset.tags.map((x) => x.tag)"
          :placeholder="t('asset-tag-add-placeholder')"
          @add="addTags"
          @remove-last="removeLastTag"
        />
      </div>
      <p v-if="editor && similarTags.length" class="rejected suggest">
        <span class="lbl">{{ t('asset-tags-from-similar') }}</span>
        <button
          v-for="tag in similarTags"
          :key="tag"
          type="button"
          class="linkish sug"
          :title="t('asset-tag-suggest-title', { tag })"
          @click="addTags([tag])"
        ><span aria-hidden="true">+</span>{{ tag }}</button>
      </p>
      <label v-if="family && editor" class="animcheck familycheck">
        <input v-model="applyToFamily" type="checkbox" /> {{ t('asset-apply-family', { count: family.size }) }}
      </label>
    </section>

    <section v-if="editor || inCollections.length" class="cols">
      <h3>{{ t('asset-collections') }}</h3>
      <div class="tags">
        <span v-for="c in inCollections" :key="c.id" class="chip src-human colchip">
          <button type="button" class="linkish" @click="$emit('browseCollection', c)">{{ c.name }}</button>
          <button v-if="editor" class="x" type="button" :aria-label="t('asset-collection-remove', { name: c.name })" @click="$emit('uncollect', c.id)">×</button>
        </span>
        <CollectionPicker
          v-if="editor"
          :collections="collections"
          :current="inCollections.map((c) => c.id)"
          :label="inCollections.length ? t('asset-collection-add-another') : t('asset-collection-add')"
          @add="(id) => $emit('collect', id)"
          @remove="(id) => $emit('uncollect', id)"
          @create="(name) => $emit('collect', '', name)"
        />
      </div>
    </section>

    <template v-if="settled">
    <section v-if="usedIn.length" class="used">
      <h3>{{ t('asset-used-in') }}</h3>
      <ul>
        <li v-for="u in usedIn" :key="u.id">
          <button type="button" class="linkish" :title="t('asset-used-in-title', { name: u.name })" @click="$emit('browseProject', u.name)">{{ u.name }}</button>
          <span class="mono where" :title="u.examples[0]?.path">{{ u.examples[0]?.path }}</span>
          <span class="when">{{ t('asset-used-in-synced', { when: daysAgo(u.synced_at) }) }}</span>
        </li>
      </ul>
    </section>

    <AssetLinks :asset="asset" @open="(id) => emit('open', id)" />

    <AssetOrigin :asset="asset" @updated="(a) => emit('updated', a)" @error="(m) => emit('error', m)" />

    <section v-if="metaRows.length">
      <h3>{{ t('asset-file') }}</h3>
      <dl>
        <template v-for="[k, v, tag] in metaRows" :key="k">
          <dt>{{ k }}</dt>
          <dd v-if="tag">
            <button type="button" class="linkish" :title="t('asset-meta-everything-with', { tag })" @click="$emit('tag', tag)">{{ v }}</button>
          </dd>
          <dd v-else>{{ v }}</dd>
        </template>
        <dt>{{ t('asset-bytes') }}</dt>
        <dd>{{ humanBytes(material ? mapsBytes : asset.size) }}</dd>
      </dl>
    </section>

    <section>
      <h3>{{ t('asset-source') }}</h3>
      <dl>
        <dt>{{ t('asset-path') }}</dt>
        <dd class="mono path">
          <template v-for="f in pathParts.folders" :key="f.path">
            <button type="button" class="linkish" :title="t('asset-path-open-folder', { folder: f.name })" @click="$emit('folder', f.path)">{{ f.name }}</button><span class="sep">/</span>
          </template>{{ pathParts.file }}
        </dd>
        <dt>BLAKE3</dt>
        <dd class="mono hash">
          <span class="copywrap">
            <button type="button" class="linkish" @click="copy(asset.hash, 'hash')">{{ asset.hash.slice(0, 16) }}…</button>
            <span v-if="copied === 'hash'" class="copytip" aria-hidden="true">{{ copiedNote('hash') }}</span>
          </span>
        </dd>
      </dl>
    </section>

    <AssetHistory :asset="asset" :editor="editor" @updated="(a) => emit('updated', a)" />

    <footer>
      <div class="get">
        <a v-if="isMaterial && needs.length" class="btn primary whole" :href="bundleUrl(asset)">
          <span>{{ t('asset-download-maps', { count: needs.length }) }}</span>
          <span class="bytes">.zip · {{ humanBytes(bundleBytes) }}</span>
        </a>
        <template v-else-if="needs.length || anims.length || character">
          <a class="btn primary whole" :href="bundleUrl(asset, withAnimations, withCharacter ? character?.id : '')">
            <span>{{ t('asset-download-linked', { count: linkedCount }) }}</span>
            <span class="bytes">.zip · {{ humanBytes(bundleBytes) }}</span>
          </a>
          <a class="btn" :href="fileUrl(asset, true)" :title="t('asset-download-only-title')">
            <span>{{ t('asset-download-only') }}</span>
            <span class="bytes">.{{ asset.ext }} · {{ humanBytes(asset.size) }}</span>
          </a>
        </template>
        <a v-else class="btn primary whole" :href="fileUrl(asset, true)">
          <span>{{ t('asset-download') }}</span>
          <span class="bytes">.{{ asset.ext }} · {{ humanBytes(asset.size) }}</span>
        </a>
      </div>
      <label v-if="anims.length && !(isMaterial && needs.length)" class="animcheck">
        <input v-model="withAnimations" type="checkbox" /> {{ t('asset-include-animations') }}
      </label>
      <label v-if="character" class="animcheck">
        <input v-model="withCharacter" type="checkbox" /> {{ t('asset-include-character', { name: character.name }) }}
      </label>
      <DownloadAs :asset="asset" />
      <div class="tools">
        <span class="copywrap">
          <button class="btn ghost" type="button" @click="copy(tomlSnippet, 'toml')">{{ t('asset-copy-toml') }}</button>
          <span v-if="copied === 'toml'" class="copytip" aria-hidden="true">{{ copiedNote('toml') }}</span>
        </span>
        <span class="copywrap">
          <button class="btn ghost" type="button" @click="copy(asset.id, 'id')">{{ t('asset-copy-id') }}</button>
          <span v-if="copied === 'id'" class="copytip" aria-hidden="true">{{ copiedNote('id') }}</span>
        </span>
        <button v-if="teachable" class="btn ghost" type="button" :title="t('asset-teach-title')" @click="emit('teach')">{{ t('asset-teach') }}</button>
        <button v-if="!asset.archived_at && editor" class="btn ghost danger archive" type="button" :title="t('asset-archive-title')" @click="emit('archive')">{{ t('asset-archive') }}</button>
      </div>
    </footer>
    </template>
    <span class="visually-hidden" role="status">{{ copied ? copiedNote(copied) : '' }}</span>
    </div>
  </aside>
</template>

<style scoped>
.used ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 0.3rem; }
.used li { display: flex; align-items: baseline; gap: 0.5rem; min-width: 0; font-size: 0.85rem; }
.used .where { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; color: var(--muted); font-size: 0.75rem; }
.used .when { flex: none; color: var(--faint); font-size: 0.75rem; }
.star.on { color: var(--star); }
.reviewed.on { color: var(--ok); }
.taghead { display: flex; align-items: baseline; justify-content: space-between; gap: 0.5rem; }
.acceptall { color: var(--muted); font-size: 0.75rem; }
.acceptall:hover { color: var(--ok); }
.rejected { display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.15rem 0.6rem; margin: 0.55rem 0 0; font-size: 0.75rem; color: var(--faint); }
.rejected .lbl { margin-right: 0.1rem; }
.rejected .rej { color: var(--faint); text-decoration: line-through; text-decoration-color: color-mix(in srgb, var(--faint) 60%, transparent); }
.rejected .rej:hover { color: var(--text); text-decoration: none; }
.suggest .sug { color: var(--muted); }
.suggest .sug span { margin-right: 0.1em; color: var(--faint); }
.suggest .sug:hover { color: var(--accent); }
.detail {
  display: flex; flex-direction: column; gap: 1rem;
  padding: 1.1rem 1.2rem 1.4rem;
  background: var(--panel); border-left: 1px solid var(--line);
  overflow-y: auto; height: 100%;
}
/* The panel scrolls; its sections must keep their size instead of squashing the preview. */
.detail > *, .stagecol > *, .infocol > * { flex-shrink: 0; }
.stagecol, .infocol { display: contents; }
/* The full page: the preview as big as the screen allows, held in view on the left; the details scroll beside it. */
.detail.page {
  display: grid; grid-template-columns: minmax(0, 1.7fr) minmax(340px, 1fr); grid-template-rows: auto 1fr;
  gap: 1.1rem clamp(1.5rem, 3vw, 3rem); align-items: start; padding: 1.1rem clamp(1.25rem, 3vw, 2.5rem) 0; border-left: 0;
}
.page > header { grid-column: 1 / -1; }
.page .stagecol { display: flex; flex-direction: column; gap: 1rem; position: sticky; top: 0; padding-bottom: 1.5rem; }
.page .infocol { display: flex; flex-direction: column; gap: 1rem; align-self: stretch; }
.page .name { font-size: 1.85rem; line-height: 1.15; }
.page .stagecol :is(.preview, :deep(.canvas), :deep(.peek)) { max-height: calc(100dvh - 14rem); }
.page .stagecol .preview:not(.audio) { min-height: min(24rem, calc(100dvh - 14rem)); }
.page .stagecol :deep(.peek) { max-height: calc(100dvh - 14rem); }
.page footer { bottom: 0; margin: auto 0 0; padding-inline: 0; padding-bottom: 1rem; }
@media (max-width: 900px) {
  .detail.page { display: flex; flex-direction: column; padding: 1.1rem 1.2rem 1.4rem; }
  .page .stagecol, .page .infocol { display: contents; }
  .page .name { font-size: 1.3rem; }
  .page footer { bottom: -1.4rem; margin: auto -1.2rem -1.4rem; padding: 0.85rem 1.2rem 0.7rem; }
  .layout { display: none; }
}
/* Where it lives and the stepping buttons share a row, so the name below gets the panel's whole width. */
.topbar { display: flex; gap: 0.75rem; align-items: center; justify-content: space-between; margin: -0.35rem 0 0.35rem; }
/* The name is edited in place: the same element turns editable, so the text keeps its size and line breaks. */
.name {
  margin: 0; font-size: 1.3rem; font-weight: 650; line-height: 1.22; letter-spacing: -0.01em;
  font-variation-settings: 'CASL' 0.4; text-wrap: balance; overflow-wrap: anywhere;
  border-radius: 4px; outline: 1px solid transparent; outline-offset: 3px; cursor: text;
  transition: outline-color var(--dur-1), background-color var(--dur-1);
}
.name:hover { outline-color: var(--line); }
.name:focus-visible { outline: 2px solid var(--accent); }
.name.editing {
  outline: 2px solid var(--accent); background: color-mix(in srgb, var(--accent) 6%, transparent);
  box-shadow: 0 0 0 5px color-mix(in srgb, var(--accent) 6%, transparent); text-wrap: pretty;
}
.nav { display: flex; align-items: center; gap: 0.1rem; flex: none; margin-right: -0.45rem; }
.icon { padding: 0.35em 0.45em; }
.pos { font-size: 0.72rem; color: var(--faint); padding: 0 0.15rem; }
.close { margin-left: 0.3rem; }
.where { margin: 0; min-width: 0; color: var(--muted); font-size: var(--step--1); display: flex; gap: 0.4em; white-space: nowrap; }
.where .pack { overflow: hidden; text-overflow: ellipsis; }
.sep { color: var(--faint); }
.preview {
  display: grid; place-items: center; aspect-ratio: 4 / 3;
  border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--kind);
}
.preview img { max-width: 100%; max-height: 100%; object-fit: contain; }
.preview img.pixelated { image-rendering: pixelated; width: 100%; height: 100%; }
/* Images open in the inspector. */
.preview img.stretched { width: 100%; height: 100%; }
.preview .spinner {
  position: absolute; top: 0.6rem; right: 0.6rem; width: 14px; height: 14px; border-radius: 50%;
  border: 2px solid color-mix(in srgb, var(--panel) 70%, transparent); border-top-color: var(--accent);
  animation: spin 0.7s linear infinite;
}
@keyframes spin { to { rotate: 1turn; } }
.zoomable { position: relative; width: 100%; padding: 0; cursor: zoom-in; color: inherit; }
.zoomable img { transition: scale var(--dur-3) var(--ease-out); }
.zoomable:hover img { scale: 1.03; }
.zoombadge {
  position: absolute; right: 0.5rem; bottom: 0.5rem; display: grid; place-items: center; width: 1.9rem; height: 1.9rem;
  border-radius: 50%; background: color-mix(in srgb, var(--panel) 85%, transparent); box-shadow: var(--elev-2);
  opacity: 0; scale: 0.8; transition: opacity var(--dur-2) var(--ease-out), scale var(--dur-2) var(--ease-pop);
}
.zoomable:hover .zoombadge, .zoomable:focus-visible .zoombadge { opacity: 1; scale: 1; }
/* Sounds: the waveform is the scrubber. Dim bars underneath, full-color ones revealed up to the playhead. */
.preview.audio {
  display: flex; flex-direction: column; aspect-ratio: auto; place-items: stretch; gap: 0.4rem; padding: 0.75rem 0.75rem 0.5rem;
  background: var(--bg);
}
.tags { position: relative; display: flex; flex-wrap: wrap; gap: 0.35rem; }
.tags :deep(.chip) { cursor: pointer; }
.chip-enter-active { transition: opacity var(--dur-2) var(--ease-out), scale var(--dur-3) var(--ease-pop); }
.chip-leave-active { transition: opacity var(--dur-1) var(--ease-out), scale var(--dur-1) var(--ease-out); position: absolute; }
.chip-enter-from, .chip-leave-to { opacity: 0; scale: 0.6; }
.chip-move { transition: translate var(--dur-2) var(--ease-out); }
.addtag { display: flex; gap: 0.4rem; margin-top: 0.6rem; }
.path { word-break: break-all; }
.path .sep { color: var(--faint); padding: 0 0.1em; }
.btn .bytes { font-weight: 400; opacity: 0.75; font-variant-numeric: tabular-nums; }
/* The download actions stay in reach while the panel scrolls. */
footer {
  position: sticky; bottom: -1.4rem; z-index: 1; display: flex; flex-direction: column; gap: 0.6rem;
  margin: auto -1.2rem -1.4rem; padding: 0.85rem 1.2rem 0.7rem; background: var(--panel); border-top: 1px solid var(--line);
}
/* The download first and widest, the file alone beside it, converting under them, and the odds and ends last. */
.get { display: flex; flex-direction: column; gap: 0.4rem; }
.get .btn { justify-content: space-between; gap: 0.8rem; }
.get .whole { padding-block: 0.65em; }
.tools {
  display: flex; flex-wrap: wrap; align-items: center; gap: 0.15rem; margin: 0 -0.55rem; padding-top: 0.35rem;
  border-top: 1px dashed color-mix(in srgb, var(--line) 80%, transparent);
}
.tools .btn { padding: 0.35em 0.55em; color: var(--muted); border-color: transparent; }
.tools .btn:hover:not(:disabled) { color: var(--text); }
.tools .btn.danger { color: var(--danger); }
.tools .archive { margin-left: auto; }
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
/* "Copied" pops up above the button that was pressed, over whatever is there, so nothing moves. */
.copywrap { position: relative; display: inline-flex; }
.copytip {
  position: absolute; bottom: calc(100% + 6px); left: 50%; transform: translateX(-50%); z-index: 5;
  padding: 0.2em 0.55em; border-radius: 6px; white-space: nowrap; pointer-events: none;
  font-size: 0.75rem; color: var(--text); background: var(--panel); border: 1px solid var(--line);
  box-shadow: 0 4px 12px rgb(0 0 0 / 0.25); animation: copytip 1.5s ease forwards;
}
@keyframes copytip {
  0% { opacity: 0; transform: translate(-50%, 4px); }
  12%, 75% { opacity: 1; transform: translate(-50%, 0); }
  100% { opacity: 0; transform: translate(-50%, 0); }
}
@media (prefers-reduced-motion: reduce) { .copytip { animation: none; } }
/* At the top of the panel there's no room above, so the link's note drops below its button. */
.copytip.below { top: calc(100% + 6px); bottom: auto; }
.familycheck { margin-top: 0.5rem; }
.cols .tags { align-items: center; }
.colchip { gap: 0.35em; }
.colchip .x { border: 0; background: none; cursor: pointer; color: var(--faint); padding: 0; font: inherit; }
.colchip .x:hover { color: var(--text); }
.cols :deep(.picker .btn) { padding: 0.2em 0.6em; }
</style>
<style scoped src="./detailSection.css"></style>
