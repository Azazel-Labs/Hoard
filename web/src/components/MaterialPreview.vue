<script setup lang="ts">
// The side panel's material preview: the material on a sphere (or a cube or plane) to orbit in place, and the
// expand button (or a double-click) opens the full-screen viewer. The scene itself is MaterialStage, shared with it.
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Asset, MaterialInfo } from '../api'
import { t } from '../i18n'
import { MaterialStage, SHAPES, type Shape } from '../materialStage'

const props = defineProps<{ asset: Asset; material: MaterialInfo; paused?: boolean }>()
const emit = defineEmits<{ expand: [] }>()

function shapeLabel(s: Shape): string {
  switch (s) {
    case 'sphere': return t('material-shape-sphere')
    case 'cube': return t('material-shape-cube')
    case 'plane': return t('material-shape-plane')
  }
}

const host = ref<HTMLDivElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const error = ref('')
const shape = ref<Shape>('sphere')
let stage: MaterialStage | null = null

async function setup() {
  teardown()
  status.value = 'loading'
  shape.value = 'sphere'
  if (!host.value) return
  const s = new MaterialStage(host.value)
  stage = s
  try {
    await s.load(props.material, props.asset.pack_id, 1024)
    if (stage !== s) return
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

watch(shape, (v) => stage?.setShape(v))
watch(() => props.paused, (p) => stage?.setPaused(!!p))
watch(() => props.asset.id, setup)
onMounted(setup)
onBeforeUnmount(teardown)
</script>

<template>
  <div class="mp">
    <div ref="host" class="canvas checker" @dblclick="emit('expand')" />
    <p v-if="status === 'loading'" class="overlay">{{ t('material-loading') }}</p>
    <p v-else-if="status === 'error'" class="overlay err">{{ t('material-preview-failed', { error }) }}</p>
    <button type="button" class="expand" :title="t('model-full-screen')" :aria-label="t('material-open-full-screen')" @click="emit('expand')">
      <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
    </button>
    <div v-if="status === 'ready'" class="hud">
      <div class="shapes" role="group" :aria-label="t('material-group-shape')">
        <button v-for="s in SHAPES" :key="s" type="button" :aria-pressed="shape === s" @click="shape = s">{{ shapeLabel(s) }}</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mp { position: relative; }
.canvas {
  aspect-ratio: 4 / 3; border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--k-material); cursor: grab; touch-action: none;
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
.shapes { display: inline-flex; gap: 2px; padding: 2px; border: 1px solid var(--line); border-radius: var(--r-chip); }
.shapes button {
  padding: 0.1rem 0.5rem; border: 0; border-radius: 3px; background: none; color: var(--muted); font-size: 0.75rem; cursor: pointer;
  transition: color var(--dur-1), background-color var(--dur-1);
}
.shapes button:hover { color: var(--text); background: var(--raised); }
.shapes button[aria-pressed='true'] { color: var(--text); background: var(--raised); box-shadow: inset 0 0 0 1px var(--line); }
</style>
