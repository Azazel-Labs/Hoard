<script setup lang="ts">
// The live mesh explorer: every model the search found, laid out in rows on an endless floor, streamed in as you fly
// near and let go when far away. Fly like a game editor (right-drag to look, WASD to move), pick a model to see what
// it is, frame it, or open it in the detail panel. The 3D world is explorerStage; three.js loads with this dialog.
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Asset } from '../api'
import type { SearchParams } from '../types/library'
import { formatNumber, t } from '../i18n'
// Types only: the world (and three.js with it) loads once the dialog is open, so it shows straight away.
import type { ExplorerSelection, ExplorerStage, ExplorerStats } from '../explorerStage'

const props = defineProps<{ query: SearchParams }>()
const emit = defineEmits<{ close: []; open: [asset: Asset] }>()

const dlg = ref<HTMLDialogElement | null>(null)
const host = ref<HTMLDivElement | null>(null)
const labels = ref<HTMLDivElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const error = ref('')
const stats = ref<ExplorerStats | null>(null)
const picked = ref<ExplorerSelection | null>(null)
const trueSize = ref(false)
const animate = ref(true)
const orbit = ref(false)
let stage: ExplorerStage | null = null

const mb = (bytes: number) => Math.round(bytes / 2 ** 20)
/** A length to two significant digits: 0.31, 1.8, 24. */
const meters = (n: number) => formatNumber(n, { maximumSignificantDigits: 2 })

onMounted(async () => {
  dlg.value?.showModal()
  try {
    const { ExplorerStage } = await import('../explorerStage')
    if (!host.value || !labels.value) return
    // The world is always dark, so it takes the theme's accent for dark backgrounds, resolved to a plain color.
    const probe = document.createElement('span')
    probe.style.color = 'var(--d-accent, var(--accent))'
    dlg.value?.appendChild(probe)
    const accent = getComputedStyle(probe).color
    probe.remove()
    const s = new ExplorerStage(host.value, labels.value, props.query, {
      accent,
      onStats: (v) => (stats.value = v),
      onSelect: (v) => (picked.value = v),
      onOpen: (a) => emit('open', a),
      onError: (e) => {
        status.value = 'error'
        error.value = e.message
      },
    })
    stage = s
    await s.start()
    if (stage === s && status.value !== 'error') status.value = 'ready'
  } catch (e) {
    if ((e as Error).message === 'closed') return
    status.value = 'error'
    error.value = (e as Error).message
  }
})

onBeforeUnmount(() => {
  stage?.dispose()
  stage = null
})

watch(trueSize, (on) => stage?.setTrueSize(on))
watch(animate, (on) => stage?.setAnimate(on))
watch(orbit, (on) => stage?.setOrbit(on))

function onKey(e: KeyboardEvent) {
  // The page's own shortcuts stay out of the explorer.
  e.stopPropagation()
  if (e.ctrlKey || e.metaKey) return
  // Flying: held keys, by where they are on the keyboard, so WASD works whatever the layout.
  if (stage?.keyDown(e.code)) {
    e.preventDefault()
    return
  }
  if (e.altKey) return
  const act: Record<string, () => void> = {
    f: () => stage?.frameSelected(),
    // Enter on a button presses it; on the world, it opens what's picked.
    enter: () => stage?.openSelected(),
    t: () => (trueSize.value = !trueSize.value),
    o: () => (orbit.value = !orbit.value),
    ' ': () => (animate.value = !animate.value),
    '+': () => stage?.nudgeSpeed(1),
    '=': () => stage?.nudgeSpeed(1),
    '-': () => stage?.nudgeSpeed(-1),
  }
  const f = act[e.key.toLowerCase()]
  if (!f || (e.key === 'Enter' && (e.target as HTMLElement).tagName === 'BUTTON')) return
  // Space or Enter on a focused button would press it as well.
  e.preventDefault()
  f()
}

const onKeyUp = (e: KeyboardEvent) => stage?.keyUp(e.code)
const onBlur = () => stage?.releaseKeys()
onMounted(() => window.addEventListener('blur', onBlur))
onBeforeUnmount(() => window.removeEventListener('blur', onBlur))
</script>

<template>
  <dialog ref="dlg" class="viewer" :aria-label="t('explorer-dialog')" @close="emit('close')" @keydown="onKey" @keyup="onKeyUp">
    <header class="tools">
      <div class="title">
        <strong>{{ t('explorer-title') }}</strong>
        <span v-if="query.q" class="dim">“{{ query.q }}”</span>
      </div>
      <div class="group" role="group" :aria-label="t('explorer-group-view')">
        <button type="button" :title="t('explorer-true-size-title')" :aria-pressed="trueSize" @click="trueSize = !trueSize">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M1.5 13.5h13M3 13.5V11M6 13.5v-1.5M9 13.5V11M12 13.5v-1.5" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" /><rect x="3" y="2.5" width="4" height="6" rx="0.8" fill="none" stroke="currentColor" stroke-width="1.4" /><rect x="9.5" y="5.5" width="3" height="3" rx="0.6" fill="none" stroke="currentColor" stroke-width="1.4" /></svg>
          <span class="wide">{{ t('explorer-true-size') }}</span>
        </button>
        <button type="button" :title="t('explorer-animate-title')" :aria-pressed="animate" @click="animate = !animate">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M5 3.5v9l7-4.5z" fill="currentColor" /></svg>
          <span class="wide">{{ t('explorer-animate') }}</span>
        </button>
        <button type="button" :title="t('explorer-orbit-title')" :aria-pressed="orbit" @click="orbit = !orbit">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><ellipse cx="8" cy="8" rx="6" ry="2.6" fill="none" stroke="currentColor" stroke-width="1.5" /><circle cx="8" cy="8" r="1.6" fill="currentColor" /></svg>
          <span class="wide">{{ t('explorer-orbit') }}</span>
        </button>
      </div>
      <button type="button" class="close" :aria-label="t('model-close-title')" :title="t('model-close-title')" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <div class="stage">
      <!-- Focused when used, so its keys don't press the toolbar's buttons. -->
      <div ref="host" class="canvas" tabindex="-1" @pointerdown="host?.focus({ preventScroll: true })" />
      <div ref="labels" class="labels" aria-hidden="true" />
      <p v-if="status === 'loading'" class="msg">{{ t('model-loading-viewer') }}</p>
      <p v-else-if="status === 'error'" class="msg err">{{ t('model-show-failed', { error }) }}</p>
      <p v-else-if="stats && !stats.total" class="msg">{{ t('explorer-empty') }}</p>

      <div v-if="stats && stats.total" class="hud mono" role="status">
        <span>{{ t('explorer-hud-loaded', { loaded: stats.loaded, total: stats.total }) }}</span>
        <span v-if="stats.inFlight">{{ t('explorer-hud-loading', { count: stats.inFlight }) }}</span>
        <span>{{ t('model-triangles', { count: stats.triangles }) }}</span>
        <span v-if="stats.far" :title="t('explorer-hud-far-title')">{{ t('explorer-hud-far', { count: stats.far, ms: stats.farMs }) }}</span>
        <span>{{ t('explorer-hud-memory', { mb: mb(stats.bytes) }) }}</span>
        <span>{{ t('explorer-hud-speed', { speed: formatNumber(stats.speed, { maximumFractionDigits: 1 }) }) }}</span>
        <span>{{ t('explorer-hud-fps', { fps: stats.fps }) }}</span>
      </div>

      <aside v-if="picked" class="card">
        <strong class="name" :title="picked.asset.name">{{ picked.asset.name }}</strong>
        <span class="dim">{{ picked.asset.pack_name }}</span>
        <span class="facts mono">
          <span class="fmt">{{ picked.asset.ext.toUpperCase() }}</span>
          <span v-if="picked.triangles !== null">{{ t('model-triangles', { count: Math.round(picked.triangles) }) }}</span>
          <span v-if="picked.size">{{ t('explorer-size', { x: meters(picked.size[0]), y: meters(picked.size[1]), z: meters(picked.size[2]) }) }}</span>
        </span>
        <span v-if="picked.far" class="dim">{{ t('explorer-far', { triangles: picked.far.triangles_out, total: picked.far.triangles_in, ms: picked.far.ms }) }}</span>
        <span class="actions">
          <button v-if="picked.canLoadFull" type="button" class="btn ghost" :title="t('explorer-full-title')" @click="stage?.loadFull()">{{ t('explorer-full') }}</button>
          <span v-else-if="picked.loadingFull" class="dim">{{ t('explorer-full-loading') }}</span>
          <button type="button" class="btn ghost" :title="t('model-frame-title')" @click="stage?.frameSelected()">{{ t('model-frame') }}</button>
          <button type="button" class="btn" :title="t('explorer-open-title')" @click="emit('open', picked.asset)">{{ t('explorer-open') }}</button>
        </span>
      </aside>
    </div>

    <footer class="keys" aria-hidden="true">
      {{ t('explorer-keys-mouse') }} · <kbd>W</kbd><kbd>A</kbd><kbd>S</kbd><kbd>D</kbd> {{ t('explorer-key-fly') }} ·
      <kbd>Q</kbd><kbd>E</kbd> {{ t('explorer-key-down-up') }} · <kbd>Shift</kbd> {{ t('explorer-key-faster') }} ·
      <kbd>F</kbd> {{ t('model-key-frame') }} · <kbd>Enter</kbd> {{ t('explorer-key-open') }} · <kbd>T</kbd> {{ t('explorer-key-true-size') }} ·
      <kbd>O</kbd> {{ t('explorer-key-orbit') }} · <kbd>Space</kbd> {{ t('explorer-key-animate') }} · <kbd>Esc</kbd> {{ t('explorer-key-close') }}
    </footer>
  </dialog>
</template>

<style scoped src="./viewerChrome.css"></style>
<style scoped>
.stage { background: #1a1c21; }
.canvas { position: absolute; inset: 0; cursor: crosshair; touch-action: none; outline: none; }
.canvas :deep(canvas) { display: block; width: 100%; height: 100%; }
.msg { pointer-events: none; color: #b8bcc6; }
.err { color: var(--danger); }
.labels { position: absolute; inset: 0; overflow: hidden; pointer-events: none; }
.labels :deep(.label) {
  position: absolute; left: 0; top: 0; max-width: 11rem; padding: 1px 6px; border-radius: 4px;
  background: rgb(14 15 19 / 0.72); color: #e6e8ee; font-size: 11px; line-height: 1.45; white-space: nowrap;
  overflow: hidden; text-overflow: ellipsis; will-change: transform;
}
.labels :deep(.label.on) { background: var(--d-accent, var(--accent)); color: #111318; font-weight: 600; z-index: 1; }
.hud {
  position: absolute; left: 0.75rem; top: 0.75rem; display: flex; flex-wrap: wrap; gap: 0.25rem 0.75rem;
  max-width: calc(100% - 1.5rem); padding: 0.35rem 0.6rem; border-radius: var(--r-chip);
  background: rgb(14 15 19 / 0.72); color: #c9cdd6; font-size: 0.72rem; pointer-events: none;
}
.card {
  position: absolute; right: 0.75rem; bottom: 0.75rem; display: flex; flex-direction: column; gap: 0.3rem;
  width: min(18rem, calc(100% - 1.5rem)); padding: 0.7rem 0.8rem; border: 1px solid var(--line);
  border-radius: var(--r-chip); background: var(--panel); box-shadow: var(--elev-1); font-size: var(--step--1);
}
.card .name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.card .dim { overflow: hidden; text-overflow: ellipsis; }
.facts { display: flex; flex-wrap: wrap; gap: 0.2rem 0.6rem; color: var(--muted); font-size: 0.75rem; }
.fmt { padding: 0 0.3rem; border: 1px solid var(--line); border-radius: 3px; }
.actions { display: flex; gap: 0.4rem; justify-content: flex-end; margin-top: 0.2rem; }
</style>
