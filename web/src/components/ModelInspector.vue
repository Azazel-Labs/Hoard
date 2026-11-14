<script setup lang="ts">
// Full-screen 3D viewer, the model counterpart of the image inspector: orbit, frame and reset, and the switches that
// answer "what is this model made of" (wireframe, textures off, lighting without the environment, a floor grid),
// against different backgrounds, with its clips. A menu on the left shows it other ways: plainly lit, one map at a
// time, as clay, with a UV checker or each mesh in its own colour, and the scene's counts and size. Textures come as
// light previews (at most 2048 px) unless "Full-resolution textures" is on.
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Asset } from '../api'
import { formatNumber, t } from '../i18n'
// Types only: the scene (and three.js with it) loads once the dialog is open, so it shows straight away.
import type { Background, MapMode, ModelStage, SceneStats, StageInfo, ViewMode } from '../modelStage'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ close: [] }>()

const BACKGROUNDS: Background[] = ['checker', 'dark', 'grey', 'light', 'env']
/** A background swatch's name, as read out: "Checker background". */
function bgLabel(b: Background): string {
  switch (b) {
    case 'checker': return t('model-bg-checker')
    case 'dark': return t('model-bg-dark')
    case 'grey': return t('model-bg-grey')
    case 'light': return t('model-bg-light')
    case 'env': return t('model-bg-env')
  }
}

/** The view modes, in menu order, with their keys: 1 to 9, then 0. */
const MODES: { mode: ViewMode; label: () => string; key: string }[] = [
  { mode: 'final', label: () => t('model-mode-final'), key: '1' },
  { mode: 'raw', label: () => t('model-mode-raw'), key: '2' },
  { mode: 'color', label: () => t('model-mode-color'), key: '3' },
  { mode: 'roughness', label: () => t('model-mode-roughness'), key: '4' },
  { mode: 'metalness', label: () => t('model-mode-metalness'), key: '5' },
  { mode: 'normal', label: () => t('model-mode-normal'), key: '6' },
  { mode: 'ao', label: () => t('model-mode-ao'), key: '7' },
  { mode: 'matcap', label: () => t('model-mode-matcap'), key: '8' },
  { mode: 'uv', label: () => t('model-mode-uv'), key: '9' },
  { mode: 'geometry', label: () => t('model-mode-geometry'), key: '0' },
]
const MAP_MODES = new Set<ViewMode>(['color', 'roughness', 'metalness', 'normal', 'ao'])

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
const fullTextures = ref(false)
const mode = ref<ViewMode>('final')
const stats = ref<SceneStats | null>(null)
const showInfo = ref(false)
// Folded away on a narrow screen, where it would cover the model.
const menuOpen = ref(!matchMedia('(max-width: 700px)').matches)
let stage: ModelStage | null = null

/** A map's view is offered only when some material has that map. */
const available = (m: ViewMode) => !MAP_MODES.has(m) || !!stats.value?.maps.includes(m as MapMode)
const menu = computed(() => [
  { id: 'render', label: t('model-menu-render'), modes: MODES.slice(0, 2) },
  { id: 'maps', label: t('model-menu-maps'), modes: MODES.slice(2, 7).filter((m) => available(m.mode)) },
  { id: 'inspect', label: t('model-menu-inspect'), modes: MODES.slice(7) },
].filter((g) => g.modes.length))
const modeLabel = computed(() => MODES.find((m) => m.mode === mode.value)!.label())
// The textures switch means something only with the model's own materials, the light switch only in the final render.
const ownMaterials = computed(() => mode.value === 'final' || mode.value === 'raw')

/** A length as people say it, to three figures: metres for glTF (in cm or mm when small), else plain units. */
function length(n: number, metres: boolean): string {
  if (!metres) return t('model-length-units', { n })
  if (n >= 1 || n === 0) return t('model-length-m', { n })
  if (n >= 0.01) return t('model-length-cm', { n: n * 100 })
  return t('model-length-mm', { n: n * 1000 })
}

/** Load the model into a new stage; switching texture resolution starts over, keeping the switches as they were. */
async function open() {
  let ModelStage: typeof import('../modelStage').ModelStage
  try {
    ;({ ModelStage } = await import('../modelStage'))
  } catch (e) {
    status.value = 'error'
    error.value = t('model-viewer-failed', { error: (e as Error).message })
    return
  }
  if (!host.value || closed) return
  stage?.dispose()
  host.value.replaceChildren()
  const again = !!info.value
  status.value = 'loading'
  const s = new ModelStage(host.value, { fullTextures: fullTextures.value })
  stage = s
  const count = () => {
    if (stage === s) stats.value = s.sceneStats()
  }
  s.onMissing = (names) => {
    missing.value = names
    count()
  }
  s.onTexturesChanged = count
  try {
    const i = await s.load(props.asset)
    if (stage !== s) return
    info.value = i
    missing.value = i.missing
    count()
    void s.texturesSettled().then(count)
    if (!available(mode.value)) mode.value = 'final'
    s.setMode(mode.value)
    if (again) {
      s.setWireframe(wireframe.value)
      s.setTextures(textures.value)
      s.setEnvironment(environment.value)
      s.setGrid(grid.value)
      s.setAutoRotate(spin.value)
      s.setBackground(bg.value)
      s.setPlaying(playing.value)
      if (clip.value) s.playClip(clip.value)
    } else {
      clip.value = i.clips[0] ?? ''
    }
    status.value = 'ready'
  } catch (e) {
    if ((e as Error).message === 'closed' || stage !== s) return
    status.value = 'error'
    error.value = (e as Error).message
  }
}

onMounted(() => {
  dlg.value?.showModal()
  void open()
})
let closed = false
onBeforeUnmount(() => {
  closed = true
  stage?.dispose()
})
watch(fullTextures, () => void open())

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
watch(mode, (m) => stage?.setMode(m))

const foldBtn = ref<HTMLButtonElement | null>(null)
const infoBtn = ref<HTMLButtonElement | null>(null)

/** Closing the panel from its own button would leave focus nowhere, and the viewer's keys with it. */
function closeInfo() {
  showInfo.value = false
  ;(infoBtn.value ?? foldBtn.value)?.focus()
}

function pickMode(m: ViewMode) {
  if (available(m)) mode.value = m
}

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
    t: () => ownMaterials.value && (textures.value = !textures.value),
    e: () => mode.value === 'final' && (environment.value = !environment.value),
    i: () => (showInfo.value = !showInfo.value),
    ...Object.fromEntries(MODES.map((m) => [m.key, () => pickMode(m.mode)])),
    g: () => (grid.value = !grid.value),
    h: () => (fullTextures.value = !fullTextures.value),
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
  <dialog ref="dlg" class="viewer" :aria-label="t('model-dialog', { name: asset.name })" @close="emit('close')" @keydown="onKey">
    <header class="tools">
      <div class="title">
        <strong>{{ asset.name }}</strong>
        <span v-if="info" class="mono dim">
          {{ t('model-title-stats', { triangles: info.triangles, meshes: info.meshes, materials: info.materials }) }}
        </span>
      </div>
      <div class="group" role="group" :aria-label="t('model-group-camera')">
        <button type="button" :title="t('model-frame-title')" :disabled="status !== 'ready'" @click="stage?.frame()">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /><circle cx="8" cy="8" r="2" fill="currentColor" /></svg>
          <span class="wide">{{ t('model-frame') }}</span>
        </button>
        <button type="button" :title="t('model-reset-title')" :disabled="status !== 'ready'" @click="stage?.reset()">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M3 8a5 5 0 1 0 1.6-3.7M3 2.5v2.8h2.8" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">{{ t('model-reset') }}</span>
        </button>
        <button type="button" :title="t('model-turn-title')" :aria-pressed="spin" :disabled="status !== 'ready'" @click="spin = !spin">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><ellipse cx="8" cy="8" rx="6" ry="2.6" fill="none" stroke="currentColor" stroke-width="1.5" /><path d="m11.5 3.8 2 1.6-2.4.9" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">{{ t('model-turn') }}</span>
        </button>
      </div>
      <div class="group" role="group" :aria-label="t('model-group-shading')">
        <button type="button" :title="t('model-textures-title')" :aria-pressed="textures" :disabled="status !== 'ready' || !ownMaterials" @click="textures = !textures">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><rect x="2" y="2" width="12" height="12" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.5" /><path d="M2 8h12M8 2v12" stroke="currentColor" stroke-width="1.2" /><rect x="2.8" y="2.8" width="5.2" height="5.2" fill="currentColor" opacity=".55" /><rect x="8" y="8" width="5.2" height="5.2" fill="currentColor" opacity=".55" /></svg>
          <span class="wide">{{ t('model-textures') }}</span>
        </button>
        <button
          type="button"
          :title="t('model-full-res-title')"
          :aria-pressed="fullTextures"
          :disabled="status !== 'ready'"
          @click="fullTextures = !fullTextures"
        >
          <span class="hd">HD</span>
          <span class="wide">{{ t('model-full-res') }}</span>
        </button>
        <button type="button" :title="t('model-light-title')" :aria-pressed="environment" :disabled="status !== 'ready' || mode !== 'final'" @click="environment = !environment">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><circle cx="8" cy="8" r="2.8" fill="currentColor" /><path d="M8 1.5v1.8M8 12.7v1.8M1.5 8h1.8M12.7 8h1.8M3.4 3.4l1.3 1.3M11.3 11.3l1.3 1.3M3.4 12.6l1.3-1.3M11.3 4.7l1.3-1.3" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" /></svg>
          <span class="wide">{{ t('model-light') }}</span>
        </button>
        <button type="button" :title="t('model-grid-title')" :aria-pressed="grid" :disabled="status !== 'ready'" @click="grid = !grid">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M1.5 12.5h13M3 9.5h10M4.5 6.5h7M5 12.5l1.6-6M11 12.5l-1.6-6M8 12.5v-6" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" /></svg>
          <span class="wide">{{ t('model-grid') }}</span>
        </button>
      </div>
      <div class="group" role="group" :aria-label="t('model-group-background')">
        <button
          v-for="b in BACKGROUNDS"
          :key="b"
          type="button"
          class="swatch"
          :class="b"
          :title="t('model-bg-title', { background: bgLabel(b) })"
          :aria-label="bgLabel(b)"
          :aria-pressed="bg === b"
          @click="bg = b"
        />
      </div>
      <div v-if="info?.clips.length" class="group" role="group" :aria-label="t('model-group-animation')">
        <button
          type="button"
          :title="playing ? t('model-pause-title') : t('model-play-title')"
          :aria-label="playing ? t('model-pause') : t('model-play')"
          @click="playing = !playing"
        >
          <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="currentColor"><rect x="4" y="3" width="3" height="10" rx=".8" /><rect x="9" y="3" width="3" height="10" rx=".8" /></svg>
          <svg v-else viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="currentColor"><path d="M4.5 2.8v10.4L13 8z" /></svg>
        </button>
        <label>
          <span class="visually-hidden">{{ t('model-animation-clip') }}</span>
          <select v-model="clip">
            <option v-for="c in info.clips" :key="c" :value="c">{{ c }}</option>
          </select>
        </label>
      </div>
      <button type="button" class="close" :aria-label="t('model-close-title')" :title="t('model-close-title')" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <div class="stage" :class="[`bg-${bg}`, { checker: bg === 'checker' }]">
      <div ref="host" class="canvas" @dblclick="stage?.frame()" />
      <nav v-if="status === 'ready'" class="modes" :aria-label="t('model-group-view')">
        <button ref="foldBtn" type="button" class="fold" :aria-expanded="menuOpen" :title="menuOpen ? t('model-menu-fold') : t('model-menu-show')" @click="menuOpen = !menuOpen">
          <span><span class="visually-hidden">{{ t('model-view-prefix') }}</span>{{ modeLabel }}</span>
          <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path :d="menuOpen ? 'm4 10 4-4 4 4' : 'm4 6 4 4 4-4'" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
        </button>
        <template v-if="menuOpen">
          <div v-for="g in menu" :key="g.id" class="section" role="group" :aria-label="g.label">
            <span class="label" aria-hidden="true">{{ g.label }}</span>
            <button
              v-for="m in g.modes"
              :key="m.mode"
              type="button"
              :title="t('model-mode-title', { mode: m.label(), key: m.key })"
              :aria-pressed="mode === m.mode"
              @click="mode = m.mode"
            >
              <span>{{ m.label() }}</span><kbd aria-hidden="true">{{ m.key }}</kbd>
            </button>
          </div>
          <div class="section" role="group" :aria-label="t('model-group-show')">
            <button type="button" :title="t('model-wireframe-title')" :aria-pressed="wireframe" @click="wireframe = !wireframe">
              <span>{{ t('model-wireframe') }}</span><kbd aria-hidden="true">W</kbd>
            </button>
            <button ref="infoBtn" type="button" :title="t('model-info-title')" :aria-pressed="showInfo" @click="showInfo = !showInfo">
              <span>{{ t('model-info') }}</span><kbd aria-hidden="true">I</kbd>
            </button>
          </div>
        </template>
      </nav>
      <section v-if="status === 'ready' && showInfo && stats" class="scene-info" aria-labelledby="scene-info-title">
        <header>
          <h2 id="scene-info-title">{{ t('model-info') }}</h2>
          <button type="button" class="x" :aria-label="t('model-info-close')" :title="t('model-info-close-title')" @click="closeInfo">
            <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
          </button>
        </header>
        <dl>
          <div><dt>{{ t('model-info-geometries') }}</dt><dd>{{ formatNumber(stats.meshes) }}</dd></div>
          <div><dt>{{ t('model-info-materials') }}</dt><dd>{{ formatNumber(stats.materials) }}</dd></div>
          <div><dt>{{ t('model-info-textures') }}</dt><dd>{{ formatNumber(stats.textures) }}</dd></div>
          <div><dt>{{ t('model-info-triangles') }}</dt><dd>{{ formatNumber(stats.triangles) }}</dd></div>
          <div><dt>{{ t('model-info-vertices') }}</dt><dd>{{ formatNumber(stats.vertices) }}</dd></div>
        </dl>
        <dl class="size" :title="stats.metres ? t('model-info-metres') : t('model-info-no-units')">
          <div><dt>{{ t('model-info-height') }}</dt><dd>{{ length(stats.size.y, stats.metres) }}</dd></div>
          <div><dt>{{ t('model-info-length') }}</dt><dd>{{ length(stats.size.z, stats.metres) }}</dd></div>
          <div><dt>{{ t('model-info-width') }}</dt><dd>{{ length(stats.size.x, stats.metres) }}</dd></div>
        </dl>
      </section>
      <p v-if="status === 'viewer'" class="msg">{{ t('model-loading-viewer') }}</p>
      <p v-else-if="status === 'loading'" class="msg">{{ t('model-loading') }}</p>
      <p v-else-if="status === 'error'" class="msg err">{{ t('model-show-failed', { error }) }}</p>
      <p v-if="missing.length" class="note" :title="missing.join('\n')">
        {{
          missing.length === 1
            ? t('model-missing-one', { name: missing[0] })
            : missing.length > 3
              ? t('model-missing-more', { count: missing.length, names: missing.slice(0, 3).join(', ') })
              : t('model-missing-some', { count: missing.length, names: missing.slice(0, 3).join(', ') })
        }}
      </p>
      <p v-else-if="info?.guessed" class="note">{{ t('model-guessed') }}</p>
    </div>
    <footer class="keys" aria-hidden="true">
      {{ t('model-keys-mouse') }} · <kbd>F</kbd> {{ t('model-key-frame') }} · <kbd>R</kbd> {{ t('model-key-reset') }} ·
      <kbd>1</kbd>–<kbd>0</kbd> {{ t('model-key-view') }} · <kbd>W</kbd> {{ t('model-key-wireframe') }} · <kbd>I</kbd> {{ t('model-key-info') }} ·
      <kbd>T</kbd> {{ t('model-key-textures') }} · <kbd>H</kbd> {{ t('model-key-full-res') }} · <kbd>E</kbd> {{ t('model-key-light') }} ·
      <kbd>G</kbd> {{ t('model-key-grid') }} · <kbd>K</kbd> {{ t('model-key-background') }}<template v-if="info?.clips.length"> · <kbd>Space</kbd> {{ t('model-key-play') }}</template>
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
.hd { font-size: 0.62rem; font-weight: 700; letter-spacing: 0.02em; border: 1.4px solid currentColor; border-radius: 3px; padding: 0 2px; line-height: 1.25; }
.canvas { position: absolute; inset: 0; cursor: grab; touch-action: none; }
.canvas:active { cursor: grabbing; }
.canvas :deep(canvas) { display: block; width: 100%; height: 100%; }
.msg { pointer-events: none; }
.err { color: var(--danger); }
.modes {
  position: absolute; left: 0.75rem; top: 0.75rem; display: flex; flex-direction: column; width: 11.5rem;
  max-height: calc(100% - 1.5rem); overflow-y: auto; padding: 3px; border: 1px solid var(--line);
  border-radius: var(--r-chip); background: var(--panel); box-shadow: var(--elev-1); font-size: var(--step--1);
}
.modes button {
  display: flex; align-items: center; justify-content: space-between; gap: 0.5rem; width: 100%; min-height: 1.75rem;
  padding: 0 0.5rem; border: 0; border-radius: 3px; background: none; color: var(--muted); font: inherit;
  text-align: left; cursor: pointer; transition: background-color var(--dur-1), color var(--dur-1);
}
.modes button:hover { color: var(--text); background: var(--raised); }
.modes button[aria-pressed='true'] { color: var(--text); background: var(--raised); box-shadow: inset 0 0 0 1px var(--line); }
.modes button[aria-pressed='true'] span::before {
  content: ''; display: inline-block; width: 0.4rem; height: 0.4rem; margin-right: 0.4rem; border-radius: 50%;
  background: var(--k-model); vertical-align: 0.1em;
}
.modes .fold { color: var(--text); font-weight: 600; }
.modes kbd { color: var(--faint); font-size: 0.68rem; line-height: 1.3; }
.section { display: flex; flex-direction: column; gap: 1px; padding-top: 0.3rem; margin-top: 0.3rem; border-top: 1px solid var(--line); }
.label { padding: 0 0.5rem 0.15rem; color: var(--faint); font-size: 0.68rem; letter-spacing: 0.04em; text-transform: uppercase; }
.scene-info {
  position: absolute; right: 0.75rem; top: 0.75rem; width: 13rem; padding: 0.5rem 0.7rem 0.6rem;
  border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--panel); box-shadow: var(--elev-1);
  font-size: var(--step--1);
}
.scene-info header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.35rem; }
.scene-info h2 { margin: 0; font-size: inherit; font-weight: 600; }
.scene-info .x {
  display: inline-grid; place-items: center; width: 1.4rem; height: 1.4rem; padding: 0; border: 0; border-radius: 3px;
  background: none; color: var(--muted); cursor: pointer;
}
.scene-info .x:hover { color: var(--text); background: var(--raised); }
.scene-info dl { margin: 0; display: grid; gap: 0.15rem; }
.scene-info dl div { display: flex; justify-content: space-between; gap: 1rem; }
.scene-info dt { color: var(--muted); }
.scene-info dd { margin: 0; font-variant-numeric: tabular-nums; }
.scene-info .size { margin-top: 0.4rem; padding-top: 0.4rem; border-top: 1px solid var(--line); }
@media (max-width: 700px) {
  .modes { width: 10rem; }
  .scene-info { top: auto; bottom: 0.75rem; }
}
.note {
  position: absolute; left: 50%; bottom: 0.75rem; translate: -50% 0; max-width: calc(100% - 2rem); margin: 0;
  padding: 0.3rem 0.7rem; border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--panel);
  color: var(--muted); font-size: 0.75rem; box-shadow: var(--elev-1);
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
</style>
