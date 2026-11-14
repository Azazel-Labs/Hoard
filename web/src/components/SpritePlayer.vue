<script setup lang="ts">
// Plays an Aseprite file from the server's sheet of its frames, at each frame's own time and through any of its
// tagged animations, or a sprite sheet cut into a grid (guessed, and changeable) at a steady rate.
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import { liteUrl, type Asset } from '../api'
import { animatedAseprite, animations, filledCells, framesUrl, guessGrid, sequence, type Grid } from '../sprites'

const props = defineProps<{ asset: Asset; pixelated: boolean }>()

const canvas = ref<HTMLCanvasElement | null>(null)
const aseprite = computed(() => animatedAseprite(props.asset))
const meta = computed(() => props.asset.meta)
const num = (v: unknown, d: number) => (typeof v === 'number' && Number.isFinite(v) && v > 0 ? v : d)

const image = ref<HTMLImageElement | null>(null)
const failed = ref(false)
const grid = ref<Grid>({ cols: 1, rows: 1 })
/** Which cells hold a frame, in reading order. */
const cells = ref<number[]>([0])
/** The sheet as drawn small, to find its frames by. */
let sample: { px: Uint8ClampedArray; w: number; h: number } | null = null

const tags = computed(() => (aseprite.value ? animations(props.asset) : []))
const tag = ref('')
const fps = ref(10)
const frame = ref(0)
const playing = ref(!matchMedia('(prefers-reduced-motion: reduce)').matches)

/** The frames to play, as cell indices, in order. */
const order = computed(() => {
  if (!aseprite.value) return cells.value
  const n = num(meta.value.frames, 1)
  const t = tags.value.find((x) => x.name === tag.value)
  return t ? sequence(Math.max(0, t.from), Math.min(n - 1, t.to), t.direction) : sequence(0, n - 1, 'forward')
})
const frameMs = (cell: number) => {
  if (!aseprite.value) return 1000 / Math.min(60, Math.max(1, fps.value || 10))
  const list = meta.value.frame_ms
  return num(Array.isArray(list) ? list[cell] : undefined, 100)
}

function load() {
  stop()
  failed.value = false
  image.value = null
  sample = null
  frame.value = 0
  tag.value = ''
  const img = new Image()
  img.decoding = 'async'
  img.onload = () => {
    if (img.src !== new URL(src.value, location.href).href) return
    image.value = img
    if (aseprite.value) {
      const n = num(meta.value.frames, 1)
      const cols = Math.ceil(Math.sqrt(n))
      grid.value = { cols, rows: Math.ceil(n / cols) }
    } else {
      sample = sampleOf(img)
      grid.value = guessGrid(sample?.px ?? null, sample?.w ?? 0, sample?.h ?? 0, props.asset.name, {
        w: num(meta.value.width, img.naturalWidth),
        h: num(meta.value.height, img.naturalHeight),
      })
    }
    findCells()
    draw()
    if (playing.value) start()
  }
  img.onerror = () => (failed.value = true)
  img.src = src.value
}
const src = computed(() => (aseprite.value ? framesUrl(props.asset) : liteUrl(props.asset)))

/** The picture at most 512 px across, as pixels. */
function sampleOf(img: HTMLImageElement) {
  const s = Math.min(1, 512 / Math.max(img.naturalWidth, img.naturalHeight))
  const [w, h] = [Math.max(1, Math.round(img.naturalWidth * s)), Math.max(1, Math.round(img.naturalHeight * s))]
  const c = document.createElement('canvas')
  ;[c.width, c.height] = [w, h]
  const g = c.getContext('2d', { willReadFrequently: true })
  if (!g) return null
  g.imageSmoothingEnabled = false
  g.drawImage(img, 0, 0, w, h)
  return { px: g.getImageData(0, 0, w, h).data, w, h }
}

function findCells() {
  const { cols, rows } = grid.value
  if (aseprite.value) return
  const found = sample ? filledCells(sample.px, sample.w, sample.h, grid.value) : []
  cells.value = found.length ? found : Array.from({ length: cols * rows }, (_, i) => i)
  frame.value = Math.min(frame.value, cells.value.length - 1)
}

/** The size of one frame on the loaded image. */
const cell = computed(() => {
  const img = image.value
  if (!img) return { w: 0, h: 0 }
  if (aseprite.value) return { w: num(meta.value.width, 1), h: num(meta.value.height, 1) }
  return { w: img.naturalWidth / grid.value.cols, h: img.naturalHeight / grid.value.rows }
})

function draw() {
  const c = canvas.value
  const img = image.value
  if (!c || !img) return
  const { w, h } = cell.value
  const at = order.value[frame.value % Math.max(1, order.value.length)] ?? 0
  const { cols } = grid.value
  const [cw, ch] = [Math.max(1, Math.round(w)), Math.max(1, Math.round(h))]
  if (c.width !== cw || c.height !== ch) [c.width, c.height] = [cw, ch]
  const g = c.getContext('2d')
  if (!g) return
  g.imageSmoothingEnabled = !props.pixelated
  g.clearRect(0, 0, cw, ch)
  g.drawImage(img, (at % cols) * w, Math.floor(at / cols) * h, w, h, 0, 0, cw, ch)
}

let raf = 0
let last = 0
let owed = 0
function tick(now: number) {
  if (!last) last = now
  owed += now - last
  last = now
  const n = order.value.length
  let moved = false
  // Catch up whole frames, but never spin through more than one lap after a long pause (a hidden tab).
  for (let i = 0; i < n && owed >= frameMs(order.value[frame.value] ?? 0); i++) {
    owed -= frameMs(order.value[frame.value] ?? 0)
    frame.value = (frame.value + 1) % n
    moved = true
  }
  if (owed > 1000) owed = 0
  if (moved) draw()
  raf = requestAnimationFrame(tick)
}
function start() {
  stop()
  if (order.value.length < 2) return
  last = 0
  owed = 0
  raf = requestAnimationFrame(tick)
}
function stop() {
  cancelAnimationFrame(raf)
  raf = 0
}
function togglePlay() {
  playing.value = !playing.value
  if (playing.value) start()
  else stop()
}
function step(by: number) {
  playing.value = false
  stop()
  const n = Math.max(1, order.value.length)
  frame.value = (frame.value + by + n) % n
  draw()
}

function setGrid(key: keyof Grid, e: Event) {
  const v = Math.round(Number((e.target as HTMLInputElement).value))
  if (!(v >= 1 && v <= 256)) return
  grid.value = { ...grid.value, [key]: v }
  findCells()
  draw()
  if (playing.value) start()
}

watch(() => props.asset.id, load, { immediate: true })
watch(tag, () => {
  frame.value = 0
  draw()
  if (playing.value) start()
})
watch(() => props.pixelated, draw)
watch(canvas, draw)
onBeforeUnmount(stop)
</script>

<template>
  <div class="sprite">
    <div class="stage checker">
      <canvas v-show="image" ref="canvas" :class="{ pixelated }" role="img" :aria-label="`${asset.name}, frame ${frame + 1} of ${order.length}`" />
      <p v-if="failed" class="note">This one can't be played.</p>
      <p v-else-if="!image" class="note">Loading frames…</p>
    </div>
    <div class="controls">
      <button type="button" class="btn ghost icon" aria-label="Previous frame" title="Previous frame" @click="step(-1)">
        <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3" y="3" width="2" height="10" rx="0.6" /><path d="M13 3.6v8.8a.6.6 0 0 1-.92.5L6.2 8.5a.6.6 0 0 1 0-1l5.88-4.4a.6.6 0 0 1 .92.5Z" /></svg>
      </button>
      <button type="button" class="btn play" :aria-label="playing ? 'Pause animation' : 'Play animation'" @click="togglePlay">
        <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3.5" y="2.5" width="3" height="11" rx="1" /><rect x="9.5" y="2.5" width="3" height="11" rx="1" /></svg>
        <svg v-else viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><path d="M4.5 2.8v10.4a.8.8 0 0 0 1.2.7l8.3-5.2a.8.8 0 0 0 0-1.4L5.7 2.1a.8.8 0 0 0-1.2.7Z" /></svg>
      </button>
      <button type="button" class="btn ghost icon" aria-label="Next frame" title="Next frame" @click="step(1)">
        <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="11" y="3" width="2" height="10" rx="0.6" /><path d="M3 3.6v8.8a.6.6 0 0 0 .92.5L9.8 8.5a.6.6 0 0 0 0-1L3.92 3.1a.6.6 0 0 0-.92.5Z" /></svg>
      </button>
      <span class="mono count" data-testid="frame-count">{{ frame + 1 }} / {{ order.length }}</span>
      <span v-if="tags.length" class="field">
        <select v-model="tag" aria-label="Animation">
          <option value="">All frames</option>
          <option v-for="t in tags" :key="t.name" :value="t.name">{{ t.name }}</option>
        </select>
      </span>
      <template v-if="!aseprite">
        <span class="field grid" title="How the sheet is cut into frames">
          <input :value="grid.cols" type="number" min="1" max="256" aria-label="Columns" @change="setGrid('cols', $event)" />
          <span aria-hidden="true">×</span>
          <input :value="grid.rows" type="number" min="1" max="256" aria-label="Rows" @change="setGrid('rows', $event)" />
        </span>
        <label class="field">
          <input v-model.number="fps" type="number" min="1" max="60" aria-label="Frames a second" />
          <span>fps</span>
        </label>
      </template>
    </div>
  </div>
</template>

<style scoped>
.sprite { display: grid; gap: 0.5rem; }
.stage {
  display: grid; place-items: center; aspect-ratio: 4 / 3;
  border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--kind);
}
canvas { width: 100%; height: 100%; object-fit: contain; }
canvas.pixelated { image-rendering: pixelated; }
.note { margin: 0; color: var(--muted); font-size: var(--step--1); }
.controls { display: flex; flex-wrap: wrap; align-items: center; gap: 0.4rem; }
.icon { padding: 0.35em 0.45em; }
.play { display: grid; place-items: center; width: 2rem; height: 2rem; padding: 0; border-radius: 50%; }
.count { font-size: var(--step--1); color: var(--muted); min-width: 4.5ch; }
.field { display: inline-flex; align-items: center; gap: 0.3rem; font-size: var(--step--1); color: var(--muted); }
.field input { width: 4.2em; }
.grid { margin-left: auto; }
</style>
