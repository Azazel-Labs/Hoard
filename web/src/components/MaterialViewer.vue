<script setup lang="ts">
// Full-screen material viewer, the counterpart of the model viewer: the material on a sphere, cube or plane, how
// often it repeats, how far its height map pushes, each of its maps switched on and off, the environment light and
// the background.
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Asset, MapRole, MaterialInfo } from '../api'
import { formatNumber, t } from '../i18n'
import { ROLE_LABEL, ROLE_NAME, UNUSED_ROLES } from '../materials'
import { MaterialStage, SHAPES, type Shape } from '../materialStage'
import { BACKGROUNDS, type Background } from '../modelStage'

const props = defineProps<{ asset: Asset; material: MaterialInfo }>()
const emit = defineEmits<{ close: [] }>()

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
function shapeLabel(s: Shape): string {
  switch (s) {
    case 'sphere': return t('material-shape-sphere')
    case 'cube': return t('material-shape-cube')
    case 'plane': return t('material-shape-plane')
  }
}
const REPEATS = [1, 2, 4, 8]
/** How far each shape's map runs, in the viewer's units, which are metres at real size: the sphere is 2 m across
 * (its map runs from pole to pole, and twice as far around), the cube's side 1.5 m and the plane 2.4 m. */
const EXTENT: Record<Shape, number> = { sphere: Math.PI, cube: 1.5, plane: 2.4 }

/** `2` → "2 m", `0.5` → "50 cm". */
const length = (m: number) => (m >= 1 ? `${formatNumber(+m.toFixed(2))} m` : `${formatNumber(+(m * 100).toFixed(1))} cm`)

const dlg = ref<HTMLDialogElement | null>(null)
const host = ref<HTMLDivElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const error = ref('')
const shape = ref<Shape>('sphere')
const real = computed(() => props.material.physical_size ?? null)
/** Times the map repeats, or `real` for its real size (when the store gives one). */
const repeat = ref<number | 'real'>(real.value ? 'real' : 1)
const displacement = ref(Math.min(0.2, props.material.displacement ?? 0.03))
const repeatTimes = computed(() =>
  repeat.value === 'real' && real.value ? EXTENT[shape.value] / real.value[0] : repeat.value === 'real' ? 1 : repeat.value,
)
const realLabel = computed(() => (real.value ? real.value.map(length).join(' × ').replace(/ m × /, ' × ') : ''))
const environment = ref(true)
const spin = ref(false)
const bg = ref<Background>('checker')
const active = ref<MapRole[]>([])
const off = ref<Set<MapRole>>(new Set())
let stage: MaterialStage | null = null

/** Each kind of map once, in the material's order, with whether the preview uses it. */
const roles = computed(() => {
  const seen = new Set<MapRole>()
  const out: { role: MapRole; used: boolean }[] = []
  for (const m of props.material.maps) {
    if (seen.has(m.role)) continue
    seen.add(m.role)
    out.push({ role: m.role, used: !UNUSED_ROLES.has(m.role) })
  }
  return out
})
const hasHeight = computed(() => props.material.maps.some((m) => m.role === 'height'))
const sizes = computed(() => {
  const px = Math.max(0, ...props.material.maps.map((m) => Math.max(m.width ?? 0, m.height ?? 0)))
  return px ? `${px} px` : ''
})

onMounted(async () => {
  dlg.value?.showModal()
  if (!host.value) return
  const s = new MaterialStage(host.value)
  stage = s
  s.onMaps = (r) => (active.value = r)
  s.setRepeat(repeatTimes.value)
  s.setDisplacement(displacement.value)
  try {
    await s.load(props.material, props.asset.pack_id, 2048)
    status.value = 'ready'
  } catch (e) {
    if ((e as Error).message === 'closed') return
    status.value = 'error'
    error.value = (e as Error).message
  }
})
onBeforeUnmount(() => stage?.dispose())

watch(shape, (v) => stage?.setShape(v))
watch(repeatTimes, (v) => stage?.setRepeat(v))
watch(displacement, (v) => stage?.setDisplacement(v))
watch(environment, (on) => {
  stage?.setEnvironment(on)
  stage?.setBackground(bg.value)
})
watch(spin, (on) => stage?.setAutoRotate(on))
watch(bg, (b) => stage?.setBackground(b))

function toggle(role: MapRole) {
  const next = new Set(off.value)
  if (next.has(role)) next.delete(role)
  else next.add(role)
  off.value = next
  stage?.setRole(role, !next.has(role))
}

const cycleBg = () => (bg.value = BACKGROUNDS[(BACKGROUNDS.indexOf(bg.value) + 1) % BACKGROUNDS.length])
const repeatChoices = computed<(number | 'real')[]>(() => (real.value ? ['real', ...REPEATS] : REPEATS))
const cycleRepeat = () => {
  const all = repeatChoices.value
  repeat.value = all[(all.indexOf(repeat.value) + 1) % all.length]
}

function onKey(e: KeyboardEvent) {
  // The page's own shortcuts (Esc closes the detail panel, arrows move between tiles) stay out of the viewer.
  e.stopPropagation()
  if (e.ctrlKey || e.metaKey || e.altKey) return
  if ((e.target as HTMLElement).tagName === 'INPUT' && e.key !== 'Escape') return
  const act: Record<string, () => void> = {
    r: () => stage?.reset(),
    a: () => (spin.value = !spin.value),
    e: () => (environment.value = !environment.value),
    k: cycleBg,
    t: cycleRepeat,
    '1': () => (shape.value = 'sphere'),
    '2': () => (shape.value = 'cube'),
    '3': () => (shape.value = 'plane'),
  }
  const f = act[e.key.toLowerCase()]
  if (!f || status.value !== 'ready') return
  e.preventDefault()
  f()
}
</script>

<template>
  <dialog ref="dlg" class="viewer" :aria-label="t('material-dialog', { name: asset.name })" @close="emit('close')" @keydown="onKey">
    <header class="tools">
      <div class="title">
        <strong>{{ asset.name }}</strong>
        <span class="mono dim">
          {{ t('material-maps', { count: material.maps.length }) }}<template v-if="sizes"> · {{ t('material-up-to', { size: sizes }) }}</template><template v-if="real"> · {{ t('material-repeat-size', { size: realLabel }) }}</template>
        </span>
      </div>
      <div class="group" role="group" :aria-label="t('material-group-shape')">
        <button
          v-for="(s, i) in SHAPES"
          :key="s"
          type="button"
          :title="t('material-shape-title', { shape: shapeLabel(s), key: i + 1 })"
          :aria-pressed="shape === s"
          :disabled="status !== 'ready'"
          @click="shape = s"
        >
          <svg v-if="s === 'sphere'" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><circle cx="8" cy="8" r="5.8" fill="none" stroke="currentColor" stroke-width="1.5" /><ellipse cx="8" cy="8" rx="5.8" ry="2.2" fill="none" stroke="currentColor" stroke-width="1" opacity=".6" /></svg>
          <svg v-else-if="s === 'cube'" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M8 1.8 13.6 5v6L8 14.2 2.4 11V5zM2.4 5 8 8.2 13.6 5M8 8.2v6" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" /></svg>
          <svg v-else viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M1.8 11 6 4.5h8.2L10 11z" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" /></svg>
          <span class="wide">{{ shapeLabel(s) }}</span>
        </button>
      </div>
      <div class="group" role="group" :aria-label="t('model-group-camera')">
        <button type="button" :title="t('model-reset-title')" :disabled="status !== 'ready'" @click="stage?.reset()">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M3 8a5 5 0 1 0 1.6-3.7M3 2.5v2.8h2.8" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">{{ t('model-reset') }}</span>
        </button>
        <button type="button" :title="t('model-turn-title')" :aria-pressed="spin" :disabled="status !== 'ready'" @click="spin = !spin">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><ellipse cx="8" cy="8" rx="6" ry="2.6" fill="none" stroke="currentColor" stroke-width="1.5" /><path d="m11.5 3.8 2 1.6-2.4.9" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">{{ t('model-turn') }}</span>
        </button>
        <button type="button" :title="t('model-light-title')" :aria-pressed="environment" :disabled="status !== 'ready'" @click="environment = !environment">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><circle cx="8" cy="8" r="2.8" fill="currentColor" /><path d="M8 1.5v1.8M8 12.7v1.8M1.5 8h1.8M12.7 8h1.8M3.4 3.4l1.3 1.3M11.3 11.3l1.3 1.3M3.4 12.6l1.3-1.3M11.3 4.7l1.3-1.3" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" /></svg>
          <span class="wide">{{ t('model-light') }}</span>
        </button>
      </div>
      <div class="group" role="group" :aria-label="t('material-group-tiling')">
        <button
          v-for="n in repeatChoices"
          :key="n"
          type="button"
          class="num"
          :title="n === 'real' ? t('material-real-title', { size: realLabel }) : t('material-repeat-title', { n })"
          :aria-pressed="repeat === n"
          :disabled="status !== 'ready'"
          @click="repeat = n"
        >
          {{ n === 'real' ? t('material-real') : t('material-repeat', { n }) }}
        </button>
      </div>
      <label v-if="hasHeight" class="group slider" :title="t('material-height-title')">
        <span class="wide">{{ t('material-height') }}</span>
        <input v-model.number="displacement" type="range" min="0" max="0.2" step="0.005" :disabled="status !== 'ready'" :aria-label="t('material-height-strength')" />
      </label>
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
      <button type="button" class="close" :aria-label="t('model-close-title')" :title="t('model-close-title')" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <div class="stage" :class="[`bg-${bg}`, { checker: bg === 'checker' }]">
      <div ref="host" class="canvas" @dblclick="stage?.reset()" />
      <p v-if="status === 'loading'" class="msg">{{ t('material-loading') }}</p>
      <p v-else-if="status === 'error'" class="msg err">{{ t('material-show-failed', { error }) }}</p>
      <div v-if="status === 'ready'" class="maps" role="group" :aria-label="t('material-group-maps')">
        <button
          v-for="r in roles"
          :key="r.role"
          type="button"
          :aria-pressed="r.used && !off.has(r.role)"
          :disabled="!r.used"
          :title="!r.used ? t('material-map-unused') : off.has(r.role) ? t('material-map-show', { map: ROLE_NAME[r.role] }) : t('material-map-hide', { map: ROLE_NAME[r.role] })"
          @click="toggle(r.role)"
        >
          <span class="dot" :class="{ on: active.includes(r.role) && !off.has(r.role) }" aria-hidden="true" />
          {{ ROLE_LABEL[r.role] }}
        </button>
      </div>
    </div>
    <footer class="keys" aria-hidden="true">
      {{ t('material-keys-mouse') }} · <kbd>1</kbd><kbd>2</kbd><kbd>3</kbd> {{ t('material-key-shape') }} ·
      <kbd>T</kbd> {{ t('material-key-tiling') }} · <kbd>R</kbd> {{ t('model-key-reset') }} · <kbd>A</kbd> {{ t('material-key-turn') }} ·
      <kbd>E</kbd> {{ t('model-key-light') }} · <kbd>K</kbd> {{ t('model-key-background') }}
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
.num { font-variant-numeric: tabular-nums; }
.slider { gap: 0.4rem; padding: 0 0.5rem; height: calc(1.75rem + 6px); color: var(--muted); }
.slider input { width: 6.5rem; accent-color: var(--k-material); }
.maps {
  position: absolute; left: 0.75rem; bottom: 0.75rem; display: flex; flex-wrap: wrap; gap: 0.3rem;
  max-width: calc(100% - 1.5rem);
}
.maps button {
  display: inline-flex; align-items: center; gap: 0.35rem; padding: 0.25rem 0.6rem; border: 1px solid var(--line);
  border-radius: var(--r-chip); background: var(--panel); color: var(--muted); font-size: 0.75rem; cursor: pointer;
  box-shadow: var(--elev-1); transition: color var(--dur-1), background-color var(--dur-1), scale var(--dur-1) var(--ease-out);
}
.maps button:hover:not(:disabled) { color: var(--text); background: var(--raised); }
.maps button:active:not(:disabled) { scale: 0.95; }
.maps button[aria-pressed='true'] { color: var(--text); }
.maps button[aria-pressed='false']:not(:disabled) { text-decoration: line-through; }
.maps button:disabled { opacity: 0.55; cursor: default; }
.dot { width: 0.5rem; height: 0.5rem; border-radius: 50%; border: 1px solid var(--faint); }
.dot.on { background: var(--k-material); border-color: var(--k-material); }
</style>
