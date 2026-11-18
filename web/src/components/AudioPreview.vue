<script setup lang="ts">
// A sound in the detail panel: its waveform, which is also the scrubber, and a play button.
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import type { Asset } from '../api'
import { t } from '../i18n'
import { playableUrl, player, stop as stopPreview } from '../player'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ error: [message: string] }>()
const audioUrl = computed(() => playableUrl(props.asset))

// The grid's quick preview and this panel's player shouldn't talk over each other.
const audioEl = ref<HTMLAudioElement | null>(null)
watch(() => player.playing, (on) => {
  if (on) audioEl.value?.pause()
})
const canvas = ref<HTMLCanvasElement | null>(null)
const canvasTop = ref<HTMLCanvasElement | null>(null)
const waveState = ref<'idle' | 'loading' | 'done' | 'skipped'>('idle')
/** Draw a min/max waveform from the decoded audio. Skipped for big files to keep the tab responsive. */
async function drawWaveform() {
  waveState.value = 'idle'
  if (!audioUrl.value) return
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
    void el.play().catch((e) => emit('error', t('asset-play-error', { error: (e as Error).message })))
  } else el.pause()
}
function seekTo(f: number) {
  const el = audioEl.value
  if (!el || !duration.value) return
  const at = Math.min(1, Math.max(0, f)) * duration.value
  el.currentTime = at
  current.value = at
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
</script>

<template>
  <div class="preview audio">
    <div
      class="scrub"
      :class="{ flat: waveState !== 'done' }"
      role="slider"
      tabindex="0"
      :aria-label="t('asset-playback-position')"
      :aria-valuemin="0"
      :aria-valuemax="Math.round(duration)"
      :aria-valuenow="Math.round(current)"
      :aria-valuetext="t('asset-playback-value', { current: clock(current), duration: clock(duration) })"
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
      <button type="button" class="play" :aria-label="playing ? t('asset-pause') : t('asset-play')" @click="togglePlay">
        <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3.5" y="2.5" width="3" height="11" rx="1" /><rect x="9.5" y="2.5" width="3" height="11" rx="1" /></svg>
        <svg v-else viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><path d="M4.5 2.8v10.4a.8.8 0 0 0 1.2.7l8.3-5.2a.8.8 0 0 0 0-1.4L5.7 2.1a.8.8 0 0 0-1.2.7Z" /></svg>
      </button>
      <span class="mono time">{{ clock(current) }} <span class="of">/ {{ clock(duration) }}</span></span>
      <span v-if="waveState === 'loading'" class="wavenote">{{ t('asset-waveform-drawing') }}</span>
      <span v-else-if="waveState === 'skipped'" class="wavenote">{{ t('asset-waveform-skipped') }}</span>
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
</template>

<style scoped>
/* The waveform is the scrubber: dim bars underneath, full-color ones revealed up to the playhead. */
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
</style>
