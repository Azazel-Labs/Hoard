<script setup lang="ts">
// Plays a DragonBones or Spine skeleton in the side panel: any of its armatures (or skins) and animations, frame by frame when paused. For a
// signed-in account, once it's up, a pose of it becomes the asset's thumbnail when the asset has none from a browser yet.
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import type { Asset, ThumbUpload } from '../api'
import { wantsBrowserThumb } from '../browserThumbs'
import { choices, drawSkeletonThumb, mountSkeleton, skeletonOf, type Player, type Skeleton } from '../skeleton2d'
import { t } from '../i18n'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ thumb: [ThumbUpload] }>()

const canvas = ref<HTMLCanvasElement | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const playing = ref(!matchMedia('(prefers-reduced-motion: reduce)').matches)
const frame = ref(0)
const total = ref(0)
const options = ref<ReturnType<typeof choices>>([])
/** `armature\u0000animation`. */
const picked = ref('')
let player: Player | null = null
/** Bumped on every load, so a slow one that finishes after the asset changed is dropped. */
let gen = 0

const many = computed(() => options.value.length > 1)

async function load() {
  teardown()
  const my = ++gen
  status.value = 'loading'
  frame.value = 0
  total.value = 0
  options.value = []
  const a = props.asset
  try {
    const skeleton = await skeletonOf(a)
    if (my !== gen || !canvas.value) return
    player = mountSkeleton(canvas.value, skeleton, playing.value)
    player.onFrame = (f) => (frame.value = f)
    options.value = choices(skeleton)
    const first = skeleton.armatures[0]
    picked.value = `${first.name}\u0000${first.first}`
    total.value = player.total
    status.value = 'ready'
    if (wantsBrowserThumb(a)) void offerThumb(a, skeleton, my)
  } catch {
    if (my === gen) status.value = 'error'
  }
}

async function offerThumb(a: Asset, skeleton: Skeleton, my: number) {
  try {
    const r = await drawSkeletonThumb(a, skeleton)
    if (r?.replaced && my === gen) emit('thumb', r)
  } catch {
    // The asset keeps the thumbnail it has; nothing the person asked for failed.
  }
}

function teardown() {
  player?.destroy()
  player = null
}

function togglePlay() {
  playing.value = !playing.value
  player?.setPlaying(playing.value)
}
function step(by: number) {
  if (!player) return
  playing.value = false
  player.setPlaying(false)
  player.goTo(player.frame + by)
}

watch(picked, (v) => {
  if (!player || !v) return
  const [armature, animation] = v.split('\u0000')
  player.show(armature, animation)
  total.value = player.total
})
watch(() => props.asset.id, load)
watch(canvas, (c) => c && load(), { once: true })
onBeforeUnmount(() => {
  gen++
  teardown()
})
</script>

<template>
  <div class="skeleton2d">
    <div class="stage checker">
      <canvas ref="canvas" class="canvas" role="img" :aria-label="t('sprite-frame-aria', { name: asset.name, frame: frame + 1, count: total || 1 })" />
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
      <select v-if="options.some((o) => o.animations.length)" v-model="picked" class="pick" :aria-label="t('sprite-animation')" :disabled="status !== 'ready'">
        <template v-if="many">
          <optgroup v-for="o in options" :key="o.armature" :label="o.armature">
            <option v-for="n in o.animations" :key="n" :value="`${o.armature}\u0000${n}`">{{ n }}</option>
          </optgroup>
        </template>
        <template v-else>
          <option v-for="n in options[0].animations" :key="n" :value="`${options[0].armature}\u0000${n}`">{{ n }}</option>
        </template>
      </select>
    </div>
  </div>
</template>

<style scoped>
.skeleton2d { display: grid; gap: 0.5rem; }
.stage {
  position: relative; display: grid; place-items: center; aspect-ratio: 4 / 3;
  border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--kind);
}
.canvas { position: absolute; inset: 0; display: block; width: 100%; height: 100%; }
.note { position: relative; margin: 0; color: var(--muted); font-size: var(--step--1); }
.controls { display: flex; flex-wrap: wrap; align-items: center; gap: 0.4rem; }
.icon { padding: 0.35em 0.45em; }
.play { display: grid; place-items: center; width: 2rem; height: 2rem; padding: 0; border-radius: 50%; }
.count { font-size: var(--step--1); color: var(--muted); min-width: 4.5ch; }
.pick { min-width: 0; max-width: 100%; flex: 1 1 10rem; }
</style>
