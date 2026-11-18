<script setup lang="ts">
// Plays a Lottie animation in the side panel, frame by frame when paused. For a signed-in account, once it's up, a
// frame of it becomes the asset's thumbnail when the asset has none from a browser yet.
import { onBeforeUnmount, ref, watch } from 'vue'
import type { AnimationItem } from 'lottie-web'
import type { Asset, ThumbUpload } from '../api'
import { wantsBrowserThumb } from '../browserThumbs'
import { t } from '../i18n'
import { drawLottieThumb, lottieData, mountLottie } from '../lottie'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ thumb: [ThumbUpload] }>()

const host = ref<HTMLDivElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const playing = ref(!matchMedia('(prefers-reduced-motion: reduce)').matches)
const frame = ref(0)
const total = ref(0)
let anim: AnimationItem | null = null
/** Bumped on every load, so a slow one that finishes after the asset changed is dropped. */
let gen = 0

async function load() {
  teardown()
  const my = ++gen
  status.value = 'loading'
  frame.value = 0
  total.value = 0
  const a = props.asset
  try {
    const doc = await lottieData(a)
    if (my !== gen || !host.value) return
    const item = await mountLottie(host.value, doc, playing.value)
    if (my !== gen) {
      item.destroy()
      return
    }
    anim = item
    total.value = Math.max(1, Math.floor(item.totalFrames))
    item.addEventListener('enterFrame', (e) => (frame.value = Math.floor(e.currentTime)))
    if (!playing.value) item.goToAndStop(0, true)
    status.value = 'ready'
    if (wantsBrowserThumb(a)) void offerThumb(a, doc, my)
  } catch {
    if (my === gen) status.value = 'error'
  }
}

async function offerThumb(a: Asset, doc: Parameters<typeof drawLottieThumb>[1], my: number) {
  try {
    const r = await drawLottieThumb(a, doc)
    if (r?.replaced && my === gen) emit('thumb', r)
  } catch {
    // The asset keeps the thumbnail it has; nothing the person asked for failed.
  }
}

function teardown() {
  anim?.destroy()
  anim = null
  host.value?.replaceChildren()
}

function togglePlay() {
  playing.value = !playing.value
  if (!anim) return
  if (playing.value) anim.play()
  else anim.pause()
}
function step(by: number) {
  if (!anim) return
  playing.value = false
  const n = total.value || 1
  frame.value = (frame.value + by + n) % n
  anim.goToAndStop(frame.value, true)
}

// The canvas keeps the size it had when the animation loaded; the stage's shape never changes, so if the panel is
// resized it's only scaled. (lottie-web's own resize loses the canvas's colors until the next frame.)
watch(() => props.asset.id, load)
watch(host, (h) => h && load(), { once: true })
onBeforeUnmount(() => {
  gen++
  teardown()
})
</script>

<template>
  <div class="lottie">
    <div class="stage checker">
      <div ref="host" class="host" role="img" :aria-label="t('sprite-frame-aria', { name: asset.name, frame: frame + 1, count: total || 1 })" />
      <p v-if="status === 'loading'" class="note">{{ t('sprite-loading') }}</p>
      <p v-else-if="status === 'error'" class="note">{{ t('sprite-failed') }}</p>
    </div>
    <div class="controls">
      <button type="button" class="btn ghost icon" :aria-label="t('sprite-previous')" :title="t('sprite-previous')" :disabled="status !== 'ready'" @click="step(-1)">
        <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3" y="3" width="2" height="10" rx="0.6" /><path d="M13 3.6v8.8a.6.6 0 0 1-.92.5L6.2 8.5a.6.6 0 0 1 0-1l5.88-4.4a.6.6 0 0 1 .92.5Z" /></svg>
      </button>
      <button type="button" class="btn play" :aria-label="playing ? t('sprite-pause') : t('sprite-play')" :disabled="status !== 'ready'" @click="togglePlay">
        <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3.5" y="2.5" width="3" height="11" rx="1" /><rect x="9.5" y="2.5" width="3" height="11" rx="1" /></svg>
        <svg v-else viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><path d="M4.5 2.8v10.4a.8.8 0 0 0 1.2.7l8.3-5.2a.8.8 0 0 0 0-1.4L5.7 2.1a.8.8 0 0 0-1.2.7Z" /></svg>
      </button>
      <button type="button" class="btn ghost icon" :aria-label="t('sprite-next')" :title="t('sprite-next')" :disabled="status !== 'ready'" @click="step(1)">
        <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="11" y="3" width="2" height="10" rx="0.6" /><path d="M3 3.6v8.8a.6.6 0 0 0 .92.5L9.8 8.5a.6.6 0 0 0 0-1L3.92 3.1a.6.6 0 0 0-.92.5Z" /></svg>
      </button>
      <span class="mono count" data-testid="frame-count">{{ status === 'ready' ? `${frame + 1} / ${total}` : '' }}</span>
    </div>
  </div>
</template>

<style scoped>
.lottie { display: grid; gap: 0.5rem; }
.stage {
  position: relative; display: grid; place-items: center; aspect-ratio: 4 / 3;
  border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--kind);
}
.host { position: absolute; inset: 0; }
.host :deep(canvas) { display: block; width: 100% !important; height: 100% !important; }
.note { position: relative; margin: 0; color: var(--muted); font-size: var(--step--1); }
.controls { display: flex; flex-wrap: wrap; align-items: center; gap: 0.4rem; }
.icon { padding: 0.35em 0.45em; }
.play { display: grid; place-items: center; width: 2rem; height: 2rem; padding: 0; border-radius: 50%; }
.count { font-size: var(--step--1); color: var(--muted); min-width: 4.5ch; }
</style>
