<script setup lang="ts">
// Full-screen texture inspector: zoom around the cursor, drag or pinch to pan, see it at 1:1, tiled 3×3 to check
// the seams, against different backgrounds, or one channel at a time.
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import { fileUrl, thumbUrl, type Asset } from '../api'

const props = defineProps<{ asset: Asset; pixelated?: boolean }>()
const emit = defineEmits<{ close: [] }>()

/** Formats every browser draws itself; anything else (TGA, DDS, EXR, PSD…) shows the server's preview. */
const NATIVE = new Set(['png', 'jpg', 'jpeg', 'gif', 'webp', 'avif', 'bmp', 'svg', 'ico'])
const native = computed(() => NATIVE.has(props.asset.ext.toLowerCase()))
const src = computed(() => (native.value ? fileUrl(props.asset) : thumbUrl(props.asset)))

const BACKGROUNDS = ['checker', 'black', 'grey', 'white'] as const
const CHANNELS = [
  { key: 'r', label: 'R', title: 'Red channel' },
  { key: 'g', label: 'G', title: 'Green channel' },
  { key: 'b', label: 'B', title: 'Blue channel' },
  { key: 'a', label: 'A', title: 'Alpha channel' },
] as const
type Channel = (typeof CHANNELS)[number]['key'] | ''

const dlg = ref<HTMLDialogElement | null>(null)
const stage = ref<HTMLElement | null>(null)
const size = reactive({ w: 0, h: 0 })
const failed = ref(false)
const view = reactive({ x: 0, y: 0, s: 1 })
const tiled = ref(false)
const bg = ref<(typeof BACKGROUNDS)[number]>('checker')
const channel = ref<Channel>('')
/** Set for button and key zooms so they glide; wheel, drag and pinch follow the hand directly. */
const gliding = ref(false)

const tiles = computed(() => (tiled.value ? 3 : 1))
const fitScale = computed(() => {
  const el = stage.value
  if (!el || !size.w) return 1
  return Math.min(el.clientWidth / (size.w * tiles.value), el.clientHeight / (size.h * tiles.value)) * 0.92
})
const pct = computed(() => `${Math.round(view.s * 100)}%`)
const crisp = computed(() => props.pixelated || view.s >= 2)

function load() {
  const img = new Image()
  img.onload = () => {
    size.w = img.naturalWidth || 512
    size.h = img.naturalHeight || 512
    fit(false)
  }
  img.onerror = () => (failed.value = true)
  img.src = src.value
}

function place(s: number, cx?: number, cy?: number, glide = true) {
  const el = stage.value
  if (!el) return
  s = Math.min(64, Math.max(0.02, s))
  const px = cx ?? el.clientWidth / 2
  const py = cy ?? el.clientHeight / 2
  view.x = px - ((px - view.x) * s) / view.s
  view.y = py - ((py - view.y) * s) / view.s
  view.s = s
  gliding.value = glide
}
function center(s: number, glide = true) {
  const el = stage.value
  if (!el) return
  view.s = s
  view.x = (el.clientWidth - size.w * tiles.value * s) / 2
  view.y = (el.clientHeight - size.h * tiles.value * s) / 2
  gliding.value = glide
}
const fit = (glide = true) => center(fitScale.value, glide)
const actual = () => center(1)
function toggleTile() {
  tiled.value = !tiled.value
  fit()
}

function onWheel(e: WheelEvent) {
  e.preventDefault()
  const r = stage.value!.getBoundingClientRect()
  place(view.s * Math.exp(-e.deltaY * (e.ctrlKey ? 0.01 : 0.0015)), e.clientX - r.left, e.clientY - r.top, false)
}

// Pointers: one drags, two pinch.
const pointers = new Map<number, { x: number; y: number }>()
let pinch = 0
function onDown(e: PointerEvent) {
  stage.value?.setPointerCapture(e.pointerId)
  pointers.set(e.pointerId, { x: e.clientX, y: e.clientY })
  if (pointers.size === 2) pinch = spread()
  gliding.value = false
}
function spread() {
  const [a, b] = [...pointers.values()]
  return Math.hypot(a.x - b.x, a.y - b.y)
}
function onMove(e: PointerEvent) {
  const p = pointers.get(e.pointerId)
  if (!p) return
  const dx = e.clientX - p.x
  const dy = e.clientY - p.y
  p.x = e.clientX
  p.y = e.clientY
  if (pointers.size === 1) {
    view.x += dx
    view.y += dy
  } else if (pointers.size === 2) {
    const d = spread()
    const [a, b] = [...pointers.values()]
    const r = stage.value!.getBoundingClientRect()
    place(view.s * (d / (pinch || d)), (a.x + b.x) / 2 - r.left, (a.y + b.y) / 2 - r.top, false)
    pinch = d
  }
}
function onUp(e: PointerEvent) {
  pointers.delete(e.pointerId)
  pinch = pointers.size === 2 ? spread() : 0
}
function onDouble(e: MouseEvent) {
  const r = stage.value!.getBoundingClientRect()
  if (Math.abs(view.s - fitScale.value) < 0.01) place(Math.max(1, fitScale.value * 2.5), e.clientX - r.left, e.clientY - r.top)
  else fit()
}

function onKey(e: KeyboardEvent) {
  // The page's own shortcuts (Esc closes the detail panel, arrows move between tiles) stay out of the inspector.
  e.stopPropagation()
  if (e.ctrlKey || e.metaKey || e.altKey) return
  const k = e.key.toLowerCase()
  const act: Record<string, () => void> = {
    '+': () => place(view.s * 1.5), '=': () => place(view.s * 1.5), '-': () => place(view.s / 1.5),
    '0': () => fit(), '1': () => actual(), t: toggleTile,
    k: () => (bg.value = BACKGROUNDS[(BACKGROUNDS.indexOf(bg.value) + 1) % BACKGROUNDS.length]),
    r: () => setChannel('r'), g: () => setChannel('g'), b: () => setChannel('b'), a: () => setChannel('a'),
    arrowleft: () => (view.x += 60), arrowright: () => (view.x -= 60), arrowup: () => (view.y += 60), arrowdown: () => (view.y -= 60),
  }
  const f = act[k]
  if (!f) return
  e.preventDefault()
  e.stopPropagation()
  if (k.startsWith('arrow')) gliding.value = true
  f()
}
const setChannel = (c: Channel) => (channel.value = channel.value === c ? '' : c)

function onResize() {
  if (Math.abs(view.s - fitScale.value) < 0.05) fit(false)
}

onMounted(() => {
  dlg.value?.showModal()
  load()
  window.addEventListener('resize', onResize)
})
onBeforeUnmount(() => window.removeEventListener('resize', onResize))

const contentStyle = computed(() => ({
  width: `${size.w * tiles.value}px`,
  height: `${size.h * tiles.value}px`,
  transform: `translate(${view.x}px, ${view.y}px) scale(${view.s})`,
  backgroundImage: `url("${src.value}")`,
  backgroundSize: `${size.w}px ${size.h}px`,
  filter: channel.value ? `url(#hoard-ch-${channel.value})` : undefined,
}))
</script>

<template>
  <dialog ref="dlg" class="viewer" :aria-label="`Inspect ${asset.name}`" @close="emit('close')" @keydown="onKey">
    <svg width="0" height="0" aria-hidden="true" class="defs">
      <!-- Each channel as greyscale, fully opaque: the alpha one turns transparency into black. -->
      <filter id="hoard-ch-r" color-interpolation-filters="sRGB"><feColorMatrix values="1 0 0 0 0  1 0 0 0 0  1 0 0 0 0  0 0 0 0 1" /></filter>
      <filter id="hoard-ch-g" color-interpolation-filters="sRGB"><feColorMatrix values="0 1 0 0 0  0 1 0 0 0  0 1 0 0 0  0 0 0 0 1" /></filter>
      <filter id="hoard-ch-b" color-interpolation-filters="sRGB"><feColorMatrix values="0 0 1 0 0  0 0 1 0 0  0 0 1 0 0  0 0 0 0 1" /></filter>
      <filter id="hoard-ch-a" color-interpolation-filters="sRGB"><feColorMatrix values="0 0 0 1 0  0 0 0 1 0  0 0 0 1 0  0 0 0 0 1" /></filter>
    </svg>

    <header class="tools">
      <div class="title">
        <strong>{{ asset.name }}</strong>
        <span v-if="size.w" class="mono dim">{{ size.w }}×{{ size.h }}{{ native ? '' : ' preview' }}</span>
      </div>
      <div class="group" role="group" aria-label="Zoom">
        <button type="button" title="Zoom out (−)" aria-label="Zoom out" @click="place(view.s / 1.5)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M3.5 8h9" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
        </button>
        <span class="mono zoom" aria-live="polite">{{ pct }}</span>
        <button type="button" title="Zoom in (+)" aria-label="Zoom in" @click="place(view.s * 1.5)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M3.5 8h9M8 3.5v9" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
        </button>
        <button type="button" title="Fit (0)" @click="fit()">Fit</button>
        <button type="button" title="Actual pixels (1)" @click="actual()">1:1</button>
      </div>
      <div class="group" role="group" aria-label="View">
        <button type="button" title="Tile 3×3 to check the seams (T)" :aria-pressed="tiled" @click="toggleTile">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="currentColor"><rect x="1.5" y="1.5" width="3.6" height="3.6" rx=".6" /><rect x="6.2" y="1.5" width="3.6" height="3.6" rx=".6" /><rect x="10.9" y="1.5" width="3.6" height="3.6" rx=".6" /><rect x="1.5" y="6.2" width="3.6" height="3.6" rx=".6" /><rect x="6.2" y="6.2" width="3.6" height="3.6" rx=".6" /><rect x="10.9" y="6.2" width="3.6" height="3.6" rx=".6" /><rect x="1.5" y="10.9" width="3.6" height="3.6" rx=".6" /><rect x="6.2" y="10.9" width="3.6" height="3.6" rx=".6" /><rect x="10.9" y="10.9" width="3.6" height="3.6" rx=".6" /></svg>
          <span class="wide">Tile</span>
        </button>
        <button
          v-for="b in BACKGROUNDS"
          :key="b"
          type="button"
          class="swatch"
          :class="b"
          :title="`${b[0].toUpperCase()}${b.slice(1)} background (K cycles)`"
          :aria-label="`${b} background`"
          :aria-pressed="bg === b"
          @click="bg = b"
        />
      </div>
      <div class="group" role="group" aria-label="Channels">
        <button
          v-for="c in CHANNELS"
          :key="c.key"
          type="button"
          class="ch"
          :class="`ch-${c.key}`"
          :title="`${c.title} (${c.label})`"
          :aria-pressed="channel === c.key"
          @click="setChannel(c.key)"
        >
          {{ c.label }}
        </button>
      </div>
      <button type="button" class="close" aria-label="Close (Esc)" title="Close (Esc)" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <div
      ref="stage"
      class="stage"
      :class="[`bg-${bg}`, { checker: bg === 'checker' }]"
      @wheel="onWheel"
      @pointerdown="onDown"
      @pointermove="onMove"
      @pointerup="onUp"
      @pointercancel="onUp"
      @dblclick="onDouble"
    >
      <p v-if="failed" class="msg">This image couldn't be loaded.</p>
      <div
        v-else-if="size.w"
        class="content"
        :class="{ glide: gliding, crisp, tiled }"
        :style="contentStyle"
        role="img"
        :aria-label="asset.name"
      />
      <p v-else class="msg">Loading…</p>
    </div>
    <footer class="keys" aria-hidden="true">
      Scroll or pinch to zoom · drag to pan · double-click to zoom in or fit · <kbd>T</kbd> tile · <kbd>K</kbd> background · <kbd>R</kbd><kbd>G</kbd><kbd>B</kbd><kbd>A</kbd> channels
    </footer>
  </dialog>
</template>

<style scoped>
.viewer {
  width: 100vw; height: 100dvh; max-width: none; max-height: none; margin: 0; padding: 0; border: 0;
  background: var(--bg); color: var(--text); display: flex; flex-direction: column;
}
.viewer:not([open]) { display: none; }
.viewer[open] { animation: viewer-in var(--dur-3) var(--ease-out); }
.viewer::backdrop { background: var(--bg); }
@keyframes viewer-in { from { opacity: 0; scale: 0.98; } }
.defs { position: absolute; }
.tools {
  display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; padding: 0.55rem 0.75rem;
  border-bottom: 1px solid var(--line); background: var(--panel); font-size: var(--step--1);
}
.title { display: flex; align-items: baseline; gap: 0.6rem; min-width: 0; flex: 1; }
.title strong { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.dim { color: var(--muted); font-size: 0.75rem; white-space: nowrap; }
.group { display: inline-flex; align-items: center; gap: 2px; padding: 2px; border: 1px solid var(--line); border-radius: var(--r-chip); }
.group button, .close {
  display: inline-flex; align-items: center; gap: 0.3rem; height: 1.75rem; min-width: 1.75rem; padding: 0 0.45rem;
  border: 0; border-radius: 3px; background: none; color: var(--muted); cursor: pointer; font-size: var(--step--1);
  justify-content: center; transition: background-color var(--dur-1), color var(--dur-1), scale var(--dur-1) var(--ease-out);
}
.group button:hover, .close:hover { color: var(--text); background: var(--raised); }
.group button:active, .close:active { scale: 0.92; }
.group button[aria-pressed='true'] { color: var(--text); background: var(--raised); box-shadow: inset 0 0 0 1px var(--line); }
.zoom { min-width: 3.4rem; text-align: center; color: var(--text); font-size: 0.75rem; font-variant-numeric: tabular-nums; }
.swatch { width: 1.75rem; padding: 0; }
.swatch::after { content: ''; width: 0.9rem; height: 0.9rem; border-radius: 50%; border: 1px solid var(--line); }
.swatch.checker::after { background: repeating-conic-gradient(#888 0 25%, #ccc 0 50%) 0 0 / 0.45rem 0.45rem; }
.swatch.black::after { background: #000; }
.swatch.grey::after { background: #808080; }
.swatch.white::after { background: #fff; }
.swatch[aria-pressed='true']::after { outline: 2px solid var(--text); outline-offset: 1px; }
.ch { font-weight: 700; font-variation-settings: 'MONO' 1; }
.ch-r[aria-pressed='true'] { color: #ff5a52; }
.ch-g[aria-pressed='true'] { color: #3ccf6e; }
.ch-b[aria-pressed='true'] { color: #4c8dff; }
.close { border: 1px solid var(--line); border-radius: var(--r-chip); }
.stage { position: relative; flex: 1; overflow: hidden; cursor: grab; touch-action: none; user-select: none; }
.stage:active { cursor: grabbing; }
.stage.bg-black { background: #000; }
.stage.bg-grey { background: #808080; }
.stage.bg-white { background: #fff; }
.content {
  position: absolute; left: 0; top: 0; transform-origin: 0 0; background-repeat: no-repeat; will-change: transform;
}
.content.tiled { background-repeat: repeat; }
.content.crisp { image-rendering: pixelated; }
.content.glide { transition: transform var(--dur-3) var(--ease-out); }
.msg { position: absolute; inset: 0; display: grid; place-items: center; margin: 0; color: var(--muted); }
.keys {
  padding: 0.4rem 0.75rem; border-top: 1px solid var(--line); background: var(--panel);
  color: var(--faint); font-size: 0.72rem; text-align: center;
}
kbd { padding: 0 0.3em; border: 1px solid var(--line); border-radius: 3px; font: inherit; margin: 0 1px; }
@media (max-width: 700px) {
  .wide, .keys { display: none; }
  .title { flex-basis: 100%; }
}
</style>
