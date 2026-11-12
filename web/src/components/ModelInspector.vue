<script setup lang="ts">
// Full-screen 3D viewer, the model counterpart of the image inspector: orbit, frame and reset, and the switches that
// answer "what is this model made of" (wireframe, textures off, lighting without the environment, a floor grid),
// against different backgrounds, with its clips.
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Asset } from '../api'
// Types only: the scene (and three.js with it) loads once the dialog is open, so it shows straight away.
import type { Background, ModelStage, StageInfo } from '../modelStage'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ close: [] }>()

const BG_LABEL: Record<Background, string> = {
  checker: 'Checker', dark: 'Dark', grey: 'Grey', light: 'Light', env: 'Environment',
}
const BACKGROUNDS = Object.keys(BG_LABEL) as Background[]

const dlg = ref<HTMLDialogElement | null>(null)
const host = ref<HTMLDivElement | null>(null)
const status = ref<'viewer' | 'loading' | 'ready' | 'error'>('viewer')
const error = ref('')
const info = ref<StageInfo | null>(null)
const missing = ref<string[]>([])
const wireframe = ref(false)
const textures = ref(true)
const environment = ref(true)
const grid = ref(false)
const spin = ref(false)
const bg = ref<Background>('checker')
const clip = ref('')
const playing = ref(true)
let stage: ModelStage | null = null

onMounted(async () => {
  dlg.value?.showModal()
  let ModelStage: typeof import('../modelStage').ModelStage
  try {
    ;({ ModelStage } = await import('../modelStage'))
  } catch (e) {
    status.value = 'error'
    error.value = `the 3D viewer didn't load (${(e as Error).message}); reloading the page may help`
    return
  }
  if (!host.value || closed) return
  status.value = 'loading'
  const s = new ModelStage(host.value)
  stage = s
  s.onMissing = (names) => (missing.value = names)
  try {
    const i = await s.load(props.asset)
    info.value = i
    missing.value = i.missing
    clip.value = i.clips[0] ?? ''
    status.value = 'ready'
  } catch (e) {
    if ((e as Error).message === 'closed') return
    status.value = 'error'
    error.value = (e as Error).message
  }
})
let closed = false
onBeforeUnmount(() => {
  closed = true
  stage?.dispose()
})

watch(wireframe, (on) => stage?.setWireframe(on))
watch(textures, (on) => stage?.setTextures(on))
watch(environment, (on) => {
  stage?.setEnvironment(on)
  stage?.setBackground(bg.value)
})
watch(grid, (on) => stage?.setGrid(on))
watch(spin, (on) => stage?.setAutoRotate(on))
watch(bg, (b) => stage?.setBackground(b))
watch(clip, (name) => stage?.playClip(name))
watch(playing, (on) => stage?.setPlaying(on))

const cycleBg = () => (bg.value = BACKGROUNDS[(BACKGROUNDS.indexOf(bg.value) + 1) % BACKGROUNDS.length])

function onKey(e: KeyboardEvent) {
  // The page's own shortcuts (Esc closes the detail panel, arrows move between tiles) stay out of the viewer.
  e.stopPropagation()
  if (e.ctrlKey || e.metaKey || e.altKey) return
  // Typing in the clip picker isn't a shortcut.
  if ((e.target as HTMLElement).tagName === 'SELECT') return
  const act: Record<string, () => void> = {
    f: () => stage?.frame(),
    r: () => stage?.reset(),
    w: () => (wireframe.value = !wireframe.value),
    t: () => (textures.value = !textures.value),
    e: () => (environment.value = !environment.value),
    g: () => (grid.value = !grid.value),
    a: () => (spin.value = !spin.value),
    k: cycleBg,
    ' ': () => (playing.value = !playing.value),
  }
  const f = act[e.key.toLowerCase()]
  if (!f || status.value !== 'ready') return
  // Space on a focused button would press it as well.
  e.preventDefault()
  f()
}
</script>

<template>
  <dialog ref="dlg" class="viewer" :aria-label="`View ${asset.name} in 3D`" @close="emit('close')" @keydown="onKey">
    <header class="tools">
      <div class="title">
        <strong>{{ asset.name }}</strong>
        <span v-if="info" class="mono dim">
          {{ info.triangles.toLocaleString() }} tris · {{ info.meshes }} {{ info.meshes === 1 ? 'mesh' : 'meshes' }} ·
          {{ info.materials }} {{ info.materials === 1 ? 'material' : 'materials' }}
        </span>
      </div>
      <div class="group" role="group" aria-label="Camera">
        <button type="button" title="Frame the model (F)" :disabled="status !== 'ready'" @click="stage?.frame()">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /><circle cx="8" cy="8" r="2" fill="currentColor" /></svg>
          <span class="wide">Frame</span>
        </button>
        <button type="button" title="Back to the starting view (R)" :disabled="status !== 'ready'" @click="stage?.reset()">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M3 8a5 5 0 1 0 1.6-3.7M3 2.5v2.8h2.8" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">Reset</span>
        </button>
        <button type="button" title="Turn slowly (A)" :aria-pressed="spin" :disabled="status !== 'ready'" @click="spin = !spin">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><ellipse cx="8" cy="8" rx="6" ry="2.6" fill="none" stroke="currentColor" stroke-width="1.5" /><path d="m11.5 3.8 2 1.6-2.4.9" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">Turn</span>
        </button>
      </div>
      <div class="group" role="group" aria-label="Shading">
        <button type="button" title="Wireframe (W)" :aria-pressed="wireframe" :disabled="status !== 'ready'" @click="wireframe = !wireframe">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M8 1.8 13.6 5v6L8 14.2 2.4 11V5zM8 1.8v12.4M2.4 5l11.2 6M13.6 5 2.4 11" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" /></svg>
          <span class="wide">Wire</span>
        </button>
        <button type="button" title="Textures (T)" :aria-pressed="textures" :disabled="status !== 'ready'" @click="textures = !textures">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><rect x="2" y="2" width="12" height="12" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.5" /><path d="M2 8h12M8 2v12" stroke="currentColor" stroke-width="1.2" /><rect x="2.8" y="2.8" width="5.2" height="5.2" fill="currentColor" opacity=".55" /><rect x="8" y="8" width="5.2" height="5.2" fill="currentColor" opacity=".55" /></svg>
          <span class="wide">Textures</span>
        </button>
        <button type="button" title="Environment light (E)" :aria-pressed="environment" :disabled="status !== 'ready'" @click="environment = !environment">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><circle cx="8" cy="8" r="2.8" fill="currentColor" /><path d="M8 1.5v1.8M8 12.7v1.8M1.5 8h1.8M12.7 8h1.8M3.4 3.4l1.3 1.3M11.3 11.3l1.3 1.3M3.4 12.6l1.3-1.3M11.3 4.7l1.3-1.3" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" /></svg>
          <span class="wide">Light</span>
        </button>
        <button type="button" title="Floor grid (G)" :aria-pressed="grid" :disabled="status !== 'ready'" @click="grid = !grid">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M1.5 12.5h13M3 9.5h10M4.5 6.5h7M5 12.5l1.6-6M11 12.5l-1.6-6M8 12.5v-6" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" /></svg>
          <span class="wide">Grid</span>
        </button>
      </div>
      <div class="group" role="group" aria-label="Background">
        <button
          v-for="b in BACKGROUNDS"
          :key="b"
          type="button"
          class="swatch"
          :class="b"
          :title="`${BG_LABEL[b]} background (K cycles)`"
          :aria-label="`${BG_LABEL[b]} background`"
          :aria-pressed="bg === b"
          @click="bg = b"
        />
      </div>
      <div v-if="info?.clips.length" class="group" role="group" aria-label="Animation">
        <button
          type="button"
          :title="playing ? 'Pause (Space)' : 'Play (Space)'"
          :aria-label="playing ? 'Pause' : 'Play'"
          @click="playing = !playing"
        >
          <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="currentColor"><rect x="4" y="3" width="3" height="10" rx=".8" /><rect x="9" y="3" width="3" height="10" rx=".8" /></svg>
          <svg v-else viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="currentColor"><path d="M4.5 2.8v10.4L13 8z" /></svg>
        </button>
        <label>
          <span class="visually-hidden">Animation clip</span>
          <select v-model="clip">
            <option v-for="c in info.clips" :key="c" :value="c">{{ c }}</option>
          </select>
        </label>
      </div>
      <button type="button" class="close" aria-label="Close (Esc)" title="Close (Esc)" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <div class="stage" :class="[`bg-${bg}`, { checker: bg === 'checker' }]">
      <div ref="host" class="canvas" @dblclick="stage?.frame()" />
      <p v-if="status === 'viewer'" class="msg">Loading the 3D viewer…</p>
      <p v-else-if="status === 'loading'" class="msg">Loading model…</p>
      <p v-else-if="status === 'error'" class="msg err">Couldn't show this model: {{ error }}</p>
      <p v-if="missing.length" class="note" :title="missing.join('\n')">
        {{ missing.length === 1 ? `Texture not found: ${missing[0]}` : `${missing.length} textures not found: ${missing.slice(0, 3).join(', ')}${missing.length > 3 ? '…' : ''}` }}
      </p>
      <p v-else-if="info?.guessed" class="note">Textures matched by name; the file names none.</p>
    </div>
    <footer class="keys" aria-hidden="true">
      Drag to orbit · right-drag to pan · scroll to zoom · double-click to frame · <kbd>F</kbd> frame · <kbd>R</kbd> reset ·
      <kbd>W</kbd> wireframe · <kbd>T</kbd> textures · <kbd>E</kbd> light · <kbd>G</kbd> grid · <kbd>K</kbd> background<template v-if="info?.clips.length"> · <kbd>Space</kbd> play</template>
    </footer>
  </dialog>
</template>

<style scoped src="./viewerChrome.css"></style>
<style scoped>
.stage.bg-dark { background: #1b1c20; }
.stage.bg-grey { background: #808080; }
.stage.bg-light { background: #e8e9ed; }
.stage.bg-env { background: #6e6e72; }
.swatch.dark::after { background: #1b1c20; }
.swatch.grey::after { background: #808080; }
.swatch.light::after { background: #e8e9ed; }
.swatch.env::after { background: radial-gradient(circle at 35% 30%, #fff, #9a9aa0 55%, #4a4a50); }
.canvas { position: absolute; inset: 0; cursor: grab; touch-action: none; }
.canvas:active { cursor: grabbing; }
.canvas :deep(canvas) { display: block; width: 100%; height: 100%; }
.msg { pointer-events: none; }
.err { color: var(--danger); }
.note {
  position: absolute; left: 50%; bottom: 0.75rem; translate: -50% 0; max-width: calc(100% - 2rem); margin: 0;
  padding: 0.3rem 0.7rem; border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--panel);
  color: var(--muted); font-size: 0.75rem; box-shadow: var(--elev-1);
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
</style>
