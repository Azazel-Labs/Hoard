<script setup lang="ts">
// Plays a Rive file in the side panel: its default artboard first, then whichever artboard's state machine or
// animation is picked. Pointing and clicking reach its state machines, as they would in an app. For a signed-in
// account, once it's up, a moment of it becomes the asset's thumbnail when the asset has none from a browser yet.
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import type { Rive } from '@rive-app/canvas'
import type { Asset, ThumbUpload } from '../api'
import { wantsBrowserThumb } from '../browserThumbs'
import { t } from '../i18n'
import { drawRiveThumb, mountRive, riveBytes, riveContents, type RiveContents, type RivePick } from '../rive'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ thumb: [ThumbUpload] }>()

const canvas = ref<HTMLCanvasElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const playing = ref(!matchMedia('(prefers-reduced-motion: reduce)').matches)
const contents = ref<RiveContents>([])
/** The picked artboard and what plays on it, as `artboard/s|a/index` into `contents`; empty for the file's default. */
const picked = ref('')
/** The artboard the file opens on. */
const opening = ref('')
let rive: Rive | null = null
let bytes: ArrayBuffer | null = null
/** Bumped on every load, so a slow one that finishes after the asset changed is dropped. */
let gen = 0

const choices = computed(() =>
  contents.value.map((b, i) => ({
    artboard: b.name,
    items: [
      ...b.stateMachines.map((name, j) => ({ value: `${i}/s/${j}`, name })),
      ...b.animations.map((name, j) => ({ value: `${i}/a/${j}`, name })),
    ],
  })),
)

function pickOf(value: string): RivePick {
  const [i, what, j] = value.split('/')
  const b = contents.value[Number(i)]
  if (!value || !b) return {}
  return what === 's'
    ? { artboard: b.name, stateMachine: b.stateMachines[Number(j)] }
    : { artboard: b.name, animation: b.animations[Number(j)] }
}

async function load(fresh: boolean) {
  teardown()
  const my = ++gen
  status.value = 'loading'
  const a = props.asset
  try {
    if (fresh) {
      bytes = null
      picked.value = ''
      contents.value = []
      bytes = await riveBytes(a)
    }
    if (my !== gen || !canvas.value || !bytes) return
    const r = await mountRive(canvas.value, bytes, pickOf(picked.value), { autoplay: playing.value, interactive: true })
    if (my !== gen) {
      r.cleanup()
      return
    }
    rive = r
    r.resizeDrawingSurfaceToCanvas()
    if (fresh) {
      contents.value = riveContents(r)
      opening.value = r.activeArtboard
    }
    status.value = 'ready'
    if (fresh && wantsBrowserThumb(a)) void offerThumb(a, bytes, my)
  } catch {
    if (my === gen) status.value = 'error'
  }
}

async function offerThumb(a: Asset, b: ArrayBuffer, my: number) {
  try {
    const r = await drawRiveThumb(a, b)
    if (r?.replaced && my === gen) emit('thumb', r)
  } catch {
    // The asset keeps the thumbnail it has; nothing the person asked for failed.
  }
}

function teardown() {
  rive?.cleanup()
  rive = null
}

function togglePlay() {
  playing.value = !playing.value
  if (!rive) return
  if (playing.value) rive.play()
  else rive.pause()
}

// Unlike Lottie's, Rive's drawing surface follows the canvas when the panel is resized.
const resize = new ResizeObserver(() => rive?.resizeDrawingSurfaceToCanvas())
watch(canvas, (c, old) => {
  if (old) resize.unobserve(old)
  if (c) resize.observe(c)
})
watch(() => props.asset.id, () => load(true))
watch(picked, (v, old) => v !== old && status.value !== 'loading' && load(false))
watch(canvas, (c) => c && load(true), { once: true })
onBeforeUnmount(() => {
  gen++
  resize.disconnect()
  teardown()
})
</script>

<template>
  <div class="rive">
    <div class="stage checker">
      <canvas ref="canvas" class="canvas" role="img" :aria-label="asset.name" />
      <p v-if="status === 'loading'" class="note">{{ t('sprite-loading') }}</p>
      <p v-else-if="status === 'error'" class="note">{{ t('sprite-failed') }}</p>
    </div>
    <div class="controls">
      <button type="button" class="btn play" :aria-label="playing ? t('sprite-pause') : t('sprite-play')" :disabled="status !== 'ready'" @click="togglePlay">
        <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3.5" y="2.5" width="3" height="11" rx="1" /><rect x="9.5" y="2.5" width="3" height="11" rx="1" /></svg>
        <svg v-else viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><path d="M4.5 2.8v10.4a.8.8 0 0 0 1.2.7l8.3-5.2a.8.8 0 0 0 0-1.4L5.7 2.1a.8.8 0 0 0-1.2.7Z" /></svg>
      </button>
      <select v-if="choices.some((c) => c.items.length)" v-model="picked" class="pick" :aria-label="t('sprite-animation')" :disabled="status === 'loading'">
        <option value="">{{ opening || asset.name }}</option>
        <optgroup v-for="c in choices" :key="c.artboard" :label="c.artboard">
          <option v-for="i in c.items" :key="i.value" :value="i.value">{{ i.name }}</option>
        </optgroup>
      </select>
    </div>
  </div>
</template>

<style scoped>
.rive { display: grid; gap: 0.5rem; }
.stage {
  position: relative; display: grid; place-items: center; aspect-ratio: 4 / 3;
  border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--kind);
}
.canvas { position: absolute; inset: 0; display: block; width: 100%; height: 100%; }
.note { position: relative; margin: 0; color: var(--muted); font-size: var(--step--1); }
.controls { display: flex; flex-wrap: wrap; align-items: center; gap: 0.4rem; }
.play { display: grid; place-items: center; width: 2rem; height: 2rem; padding: 0; border-radius: 50%; }
.pick { min-width: 0; max-width: 100%; flex: 1 1 10rem; }
</style>
