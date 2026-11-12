<script setup lang="ts">
// The side panel's 3D preview: orbit and zoom in place, pick a clip, and the expand button (or a double-click)
// opens the full-screen viewer. The scene itself is ModelStage, shared with that viewer; it brings three.js, so it
// loads the first time a model is shown, behind the same "Loading" as the model.
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Asset } from '../api'
import type { ModelStage, StageInfo } from '../modelStage'

const props = defineProps<{ asset: Asset; paused?: boolean }>()
const emit = defineEmits<{ expand: [] }>()

const host = ref<HTMLDivElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const error = ref('')
const info = ref<StageInfo | null>(null)
const missing = ref<string[]>([])
const activeClip = ref('')
let stage: ModelStage | null = null

async function setup() {
  teardown()
  status.value = 'loading'
  info.value = null
  missing.value = []
  activeClip.value = ''
  if (!host.value) return
  const id = props.asset.id
  let ModelStage: typeof import('../modelStage').ModelStage
  try {
    ;({ ModelStage } = await import('../modelStage'))
  } catch (e) {
    status.value = 'error'
    error.value = `the 3D viewer didn't load (${(e as Error).message}); reloading the page may help`
    return
  }
  if (!host.value || props.asset.id !== id || stage) return
  const s = new ModelStage(host.value)
  stage = s
  s.onMissing = (names) => (missing.value = names)
  try {
    const i = await s.load(props.asset)
    if (stage !== s) return
    info.value = i
    missing.value = i.missing
    activeClip.value = i.clips[0] ?? ''
    s.setPaused(!!props.paused)
    status.value = 'ready'
  } catch (e) {
    if (stage !== s) return
    status.value = 'error'
    error.value = (e as Error).message
  }
}

function teardown() {
  stage?.dispose()
  stage = null
}

watch(activeClip, (name) => stage?.playClip(name))
watch(() => props.paused, (p) => stage?.setPaused(!!p))
watch(() => props.asset.id, setup)
onMounted(setup)
onBeforeUnmount(teardown)
</script>

<template>
  <div class="mv">
    <div ref="host" class="canvas checker" @dblclick="emit('expand')" />
    <p v-if="status === 'loading'" class="overlay">Loading model…</p>
    <p v-else-if="status === 'error'" class="overlay err">Couldn't preview this model: {{ error }}</p>
    <button type="button" class="expand" title="Full screen" aria-label="Open the full-screen 3D viewer" @click="emit('expand')">
      <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
    </button>
    <div v-if="status === 'ready' && info" class="hud">
      <span class="mono">{{ info.triangles.toLocaleString() }} triangles</span>
      <label v-if="info.clips.length">
        <span class="visually-hidden">Animation</span>
        <select v-model="activeClip" class="field">
          <option v-for="c in info.clips" :key="c" :value="c">{{ c }}</option>
        </select>
      </label>
      <span class="hint">Drag to orbit · scroll to zoom · double-click for full screen</span>
      <p v-if="missing.length" class="missing" :title="missing.join('\n')">
        {{ missing.length === 1 ? `Texture not found: ${missing[0]}` : `${missing.length} textures not found` }}
      </p>
      <p v-else-if="info.guessed" class="missing">Textures matched by name; the file names none.</p>
    </div>
  </div>
</template>

<style scoped>
.mv { position: relative; }
.canvas {
  aspect-ratio: 4 / 3; border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--k-model); cursor: grab; touch-action: none;
}
.canvas:active { cursor: grabbing; }
.canvas :deep(canvas) { display: block; width: 100%; height: 100%; }
.overlay { position: absolute; inset: 0; display: grid; place-items: center; margin: 0; color: var(--muted); font-size: var(--step--1); padding: 1rem; text-align: center; pointer-events: none; }
.err { color: var(--danger); }
.expand {
  position: absolute; top: 0.5rem; right: 0.5rem; display: grid; place-items: center; width: 1.9rem; height: 1.9rem;
  border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--panel); color: var(--muted); cursor: pointer;
  box-shadow: var(--elev-1); transition: color var(--dur-1), background-color var(--dur-1), scale var(--dur-1) var(--ease-out);
}
.expand:hover { color: var(--text); background: var(--raised); }
.expand:active { scale: 0.92; }
.hud { display: flex; flex-wrap: wrap; gap: 0.5rem 0.9rem; align-items: center; margin-top: 0.45rem; font-size: 0.78rem; color: var(--muted); }
.hud .field { width: auto; padding: 0.2em 0.4em; font-size: 0.78rem; }
.hint { margin-left: auto; color: var(--faint); }
.missing { flex-basis: 100%; margin: 0; color: var(--faint); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
