<script setup lang="ts">
/**
 * A pack's pictures of itself (its previews, screenshots and marketing shots), one at a time. Small, on a pack's row,
 * it goes from one to the next while pointed at; big, in the pack's panel, it goes on by itself (unless motion is
 * turned down, or it's pointed at) and has buttons to step through them.
 */
import { onBeforeUnmount, ref, watch, watchEffect } from 'vue'
import { liteUrl, thumbUrl } from '../api'
import { t } from '../i18n'

const props = withDefaults(defineProps<{ ids: string[]; big?: boolean }>(), { big: false })

/** How long each picture shows: quickly while a small one is pointed at, slowly when a big one goes by itself. */
const HOVER_MS = 1100
const AUTO_MS = 5000

const at = ref(0)
/** The pictures shown so far and the one after the current, so the rest load only if they're reached. */
const loaded = ref(new Set<number>([0, 1]))
const pointedAt = ref(false)
const reduced = window.matchMedia?.('(prefers-reduced-motion: reduce)').matches ?? false

function go(i: number) {
  const n = props.ids.length
  if (!n) return
  at.value = ((i % n) + n) % n
  loaded.value.add(at.value)
  loaded.value.add((at.value + 1) % n)
}

watch(
  () => props.ids.join(),
  () => {
    at.value = 0
    loaded.value = new Set([0, 1])
  },
)

let timer: ReturnType<typeof setInterval> | undefined
watchEffect(() => {
  clearInterval(timer)
  timer = undefined
  const going = props.ids.length > 1 && (props.big ? !pointedAt.value && !reduced : pointedAt.value)
  if (going) timer = setInterval(() => go(at.value + 1), props.big ? AUTO_MS : HOVER_MS)
})
onBeforeUnmount(() => clearInterval(timer))

function enter() {
  pointedAt.value = true
  // A small one shows its next picture straight away, so pointing at it shows there's more.
  if (!props.big && props.ids.length > 1) go(at.value + 1)
}
function leave() {
  pointedAt.value = false
  if (!props.big) go(0)
}
</script>

<template>
  <div class="showcase" :class="{ big }" @pointerenter="enter" @pointerleave="leave" @focusin="pointedAt = true" @focusout="pointedAt = false">
    <template v-for="(id, i) in ids" :key="id">
      <img
        v-if="loaded.has(i)"
        :src="big ? liteUrl({ id }) : thumbUrl({ id })"
        :style="big ? { backgroundImage: `url(${thumbUrl({ id })})` } : undefined"
        :class="{ on: i === at }"
        alt=""
        :loading="i === 0 ? 'lazy' : undefined"
      />
    </template>
    <template v-if="ids.length > 1">
      <template v-if="big">
        <button type="button" class="step prev" :aria-label="t('pack-cover-previous')" :title="t('pack-cover-previous')" @click="go(at - 1)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M10 3 5 8l5 5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
        </button>
        <button type="button" class="step next" :aria-label="t('pack-cover-next')" :title="t('pack-cover-next')" @click="go(at + 1)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m6 3 5 5-5 5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
        </button>
        <div class="dots">
          <button
            v-for="(id, i) in ids"
            :key="id"
            type="button"
            :class="{ on: i === at }"
            :aria-label="t('pack-cover-picture', { n: i + 1, count: ids.length })"
            :aria-current="i === at ? 'true' : undefined"
            @click="go(i)"
          />
        </div>
      </template>
      <div v-else class="dots" aria-hidden="true">
        <span v-for="(id, i) in ids" :key="id" :class="{ on: i === at }" />
      </div>
    </template>
  </div>
</template>

<style scoped>
.showcase { position: relative; width: 100%; height: 100%; overflow: hidden; background: var(--check-a); }
.showcase img {
  position: absolute; inset: 0; width: 100%; height: 100%; object-fit: cover;
  background-size: cover; background-position: center;
  opacity: 0; transition: opacity 0.45s ease;
}
.showcase img.on { opacity: 1; }
.dots {
  position: absolute; left: 0; right: 0; bottom: 4px; display: flex; justify-content: center; gap: 3px;
  pointer-events: none;
}
.dots > * {
  width: 5px; height: 5px; padding: 0; border: 0; border-radius: 50%;
  background: rgb(255 255 255 / 0.55); box-shadow: 0 0 0 1px rgb(0 0 0 / 0.25);
}
.dots > .on { background: #fff; }
.big .dots { bottom: 8px; gap: 6px; pointer-events: auto; }
.big .dots > button { width: 8px; height: 8px; cursor: pointer; }
.step {
  position: absolute; top: 50%; translate: 0 -50%; display: grid; place-items: center;
  width: 28px; height: 28px; padding: 0; border: 0; border-radius: 50%; cursor: pointer;
  color: #fff; background: rgb(0 0 0 / 0.45); opacity: 0; transition: opacity 0.15s ease;
}
.prev { left: 6px; }
.next { right: 6px; }
.showcase:hover .step, .step:focus-visible { opacity: 1; }
@media (hover: none) { .step { opacity: 1; } }
@media (prefers-reduced-motion: reduce) { .showcase img, .step { transition: none; } }
</style>
