<script setup lang="ts">
// The side panel's 3D preview: orbit and zoom in place, pick a clip, and the expand button (or a double-click)
// opens the full-screen viewer. The scene itself is ModelStage, shared with that viewer; it brings three.js, so it
// loads the first time a model is shown, behind the same "Loading" as the model. For a signed-in account, once the
// model and its textures are up, its picture becomes the asset's thumbnail when that improves on the one it has.
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Asset, ThumbUpload } from '../api'
import { drawThumb, wantsBrowserThumb } from '../browserThumbs'
import { t } from '../i18n'
import type { ModelStage, StageInfo } from '../modelStage'
import { mayChoose, playEverywhere, wearerName } from '../skeletonBody'

const props = defineProps<{ asset: Asset; paused?: boolean }>()
const emit = defineEmits<{ expand: []; thumb: [ThumbUpload] }>()

const host = ref<HTMLDivElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const error = ref('')
const info = ref<StageInfo | null>(null)
const missing = ref<string[]>([])
const activeClip = ref('')
/** For an animation: the character it plays on, when the person picked one of several. */
const on = ref('')
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
    error.value = t('model-viewer-failed', { error: (e as Error).message })
    return
  }
  if (!host.value || props.asset.id !== id || stage) return
  const s = new ModelStage(host.value)
  stage = s
  s.onMissing = (names) => (missing.value = names)
  try {
    const i = await s.load(props.asset, { on: on.value })
    if (stage !== s) return
    info.value = i
    missing.value = i.missing
    activeClip.value = i.clips[0] ?? ''
    s.setPaused(!!props.paused)
    status.value = 'ready'
    if (!on.value && wantsBrowserThumb(props.asset)) void offerThumb(s, props.asset)
  } catch (e) {
    if (stage !== s) return
    status.value = 'error'
    error.value = (e as Error).message
  }
}

async function offerThumb(s: ModelStage, a: Asset) {
  try {
    const r = await drawThumb(s, a)
    if (r?.replaced && stage === s) emit('thumb', r)
  } catch {
    // The server's thumbnail stays; nothing the person asked for failed.
  }
}

function teardown() {
  stage?.dispose()
  stage = null
}

watch(activeClip, (name) => void stage?.playClip(name))
watch(() => props.paused, (p) => stage?.setPaused(!!p))
function playOn(id: string) {
  on.value = id
  void setup()
}

watch(
  () => props.asset.id,
  () => {
    on.value = ''
    void setup()
  },
)
onMounted(setup)
onBeforeUnmount(teardown)
</script>

<template>
  <div class="mv">
    <div ref="host" class="canvas checker" @dblclick="emit('expand')" />
    <p v-if="status === 'loading'" class="overlay">{{ t('model-loading') }}</p>
    <p v-else-if="status === 'error'" class="overlay err">{{ t('model-preview-failed', { error }) }}</p>
    <button type="button" class="expand" :title="t('model-full-screen')" :aria-label="t('model-open-full-screen')" @click="emit('expand')">
      <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
    </button>
    <!-- Always there, so the panel doesn't move when the model's details arrive. -->
    <div class="hud">
      <template v-if="status === 'ready' && info">
      <span class="mono">{{ t('model-triangles', { count: info.triangles }) }}</span>
      <label v-if="info.clips.length || info.fitting.length">
        <span class="visually-hidden">{{ t('model-animation') }}</span>
        <select v-model="activeClip" class="field">
          <option v-if="!info.clips.length" value="">{{ t('model-rest-pose') }}</option>
          <option v-for="c in info.clips" :key="c" :value="c">{{ c }}</option>
          <optgroup v-if="info.fitting.length" :label="t('asset-fit-animations', { count: info.fitting.length })">
            <option v-for="f in info.fitting" :key="f.clip" :value="f.clip">{{ f.name }}</option>
          </optgroup>
        </select>
      </label>
      <label v-if="info.wearers.length > 1 || info.wearers[0]?.away">
        <span class="visually-hidden">{{ t('model-plays-on') }}</span>
        <select class="field" :value="info.on" @change="playOn(($event.target as HTMLSelectElement).value)">
          <option v-for="w in info.wearers" :key="w.id" :value="w.id">{{ w.name }}</option>
        </select>
      </label>
        <button
          v-if="mayChoose && info.playsOn && info.on !== info.playsOn"
          type="button"
          class="btn ghost everywhere"
          :title="t('model-plays-everywhere-title', { name: wearerName(info) })"
          @click="void playEverywhere(info)"
        >{{ t('model-plays-everywhere') }}</button>
      <p v-if="missing.length" class="missing" :title="missing.join('\n')">
        {{ missing.length === 1 ? t('model-missing-one', { name: missing[0] }) : t('model-missing-count', { count: missing.length }) }}
      </p>
      <p v-else-if="info.guessed" class="missing">{{ t('model-guessed') }}</p>
      </template>
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
.hud {
  display: flex; flex-wrap: wrap; gap: 0.5rem 0.9rem; align-items: center; min-height: 1.6rem; margin-top: 0.45rem;
  font-size: 0.78rem; color: var(--muted);
}
.hud .field { width: auto; padding: 0.2em 0.4em; font-size: 0.78rem; }
.hud .everywhere { padding: 0.2em 0.55em; font-size: 0.78rem; }
.missing { flex-basis: 100%; margin: 0; color: var(--faint); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
