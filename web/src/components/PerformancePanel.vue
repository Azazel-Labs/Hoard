<script setup lang="ts">
// Performance: the profiler, rendering model thumbnails, their quality, and the limits on work, uploads and
// the conversion cache.
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { api, type Stats, type ThumbQuality } from '../api'
import { humanBytes } from '../format'
import { formatNumber, t } from '../i18n'
import { formatMiB } from '../sizes'
import ProfilerPanel from './ProfilerPanel.vue'
import ResetButton from './ResetButton.vue'
import SizeInput from './SizeInput.vue'
import { onOff, useSettings } from './settingsContext'

const props = defineProps<{ stats: Stats | null }>()
const { s, busy, save, opts, saveOpt, resetOpt, changed } = useSettings()

/** Whether model thumbnails render on a GPU, in software, or not at all. */
const gpuNote = computed(() => {
  const r = props.stats?.renderer ?? ''
  if (r.startsWith('gpu: ')) return { state: 'ready', text: t('settings-gpu-on', { renderer: r.slice(5) }) }
  if (r.startsWith('cpu: ')) return { state: 'off', text: t('settings-gpu-software', { renderer: r.slice(5) }) }
  if (r.startsWith('unavailable')) return { state: 'unavailable', text: t('settings-gpu-none') }
  return null
})

/** The frame rates moving thumbnails can be made at (the server's `MOTION_FPS`). */
const MOTION_FPS = [12, 24, 30]

// Labels and hints are getters, so they're in the language of the moment. `webp` is the lossy quality thumbnails are
// made at (`ThumbQuality::webp_quality` in crates/hoardd/src/settings.rs); one is exact instead where that's smaller.
const THUMBS: { value: ThumbQuality; webp: number; readonly label: string; readonly hint: string }[] = [
  { value: 'small', webp: 60, get label() { return t('settings-thumbs-small') }, get hint() { return t('settings-thumbs-small-hint') } },
  { value: 'balanced', webp: 75, get label() { return t('settings-thumbs-balanced') }, get hint() { return t('settings-thumbs-balanced-hint') } },
  { value: 'sharp', webp: 88, get label() { return t('settings-thumbs-sharp') }, get hint() { return t('settings-thumbs-sharp-hint') } },
]
/** How many pixels across thumbnails can be made (the server's `THUMB_SIZES`). */
const THUMB_SIZES = [128, 256]
// How much the server keeps around for previews and conversions, fetched when the Performance page opens. The server
// answers with the sizes it last worked out and works out new ones behind that when the library has changed (after
// thumbnails are made again, say), so it's asked again until they're current. Only a few times: a busy library is never quite.
const usage = ref<Stats | null>(null)
let usageTimer: ReturnType<typeof setTimeout> | undefined
async function loadUsage(again = 6) {
  clearTimeout(usageTimer)
  try {
    usage.value = await api.stats()
    if (usage.value.sizes_refreshing && again > 0) usageTimer = setTimeout(() => loadUsage(again - 1), 2500)
  } catch {
    usage.value = null
  }
}
onMounted(() => loadUsage())
onUnmounted(() => clearTimeout(usageTimer))
// A smaller cache limit trims in the background; show what's left once it has.
watch(
  () => s.value.options.conversion_cache_mb.value,
  () => setTimeout(() => loadUsage(), 1500),
)
</script>

<template>
  <ProfilerPanel />
  <section>
    <div class="sh"><h3>{{ t('settings-render') }}</h3></div>
    <p v-if="gpuNote" class="status" :data-state="gpuNote.state" role="status" :title="stats?.renderer">
      <span class="dot" aria-hidden="true" />
      <span class="text">{{ gpuNote.text }}</span>
    </p>
    <p v-if="stats?.renderer_hint" class="hint" data-testid="gpu-hint">{{ stats.renderer_hint }}</p>
    <div class="switches">
      <div class="opt-row">
        <label class="check">
          <input type="checkbox" :checked="s.options.render.value" :disabled="busy" @change="save({ render: ($event.target as HTMLInputElement).checked })" />
          {{ t('settings-render-toggle') }}
        </label>
        <ResetButton v-if="changed('render')" :to="onOff(s.options.render.default)" :disabled="busy" @reset="resetOpt('render')" />
      </div>
    </div>
  </section>

  <section>
    <div class="sh">
      <h3>{{ t('settings-thumbs') }}</h3>
      <ResetButton
        v-if="changed('thumb_quality')"
        :to="THUMBS.find((t) => t.value === s!.options.thumb_quality.default)?.label ?? s.options.thumb_quality.default"
        :disabled="busy"
        @reset="resetOpt('thumb_quality')"
      />
    </div>
    <p class="what">
      {{ t('settings-thumbs-what') }}
      <span v-if="usage?.preview_bytes !== undefined" class="size">{{ t('settings-previews-size', { size: humanBytes(usage.preview_bytes) }) }}</span>
    </p>
    <div class="opt-row format">
      <span id="thumbs-size">{{ t('settings-thumbs-size') }}</span>
      <div class="modes" role="radiogroup" aria-labelledby="thumbs-size">
        <label v-for="px in THUMB_SIZES" :key="px">
          <input
            type="radio"
            name="thumbs-size"
            :checked="s.options.thumb_size.value === px"
            :disabled="busy"
            @change="save({ thumb_size: px })"
          />{{ t('settings-thumbs-size-option', { px }) }}
        </label>
      </div>
      <ResetButton
        v-if="changed('thumb_size')"
        :to="t('settings-thumbs-size-option', { px: s.options.thumb_size.default })"
        :disabled="busy"
        @reset="resetOpt('thumb_size')"
      />
    </div>
    <div class="choices" role="radiogroup" :aria-label="t('settings-thumbs')">
      <label v-for="q in THUMBS" :key="q.value" :class="{ on: s.options.thumb_quality.value === q.value }">
        <input
          type="radio"
          name="thumbs"
          :checked="s.options.thumb_quality.value === q.value"
          :disabled="busy"
          @change="save({ thumb_quality: q.value }, t('settings-thumbs-saved', { quality: q.label }))"
        />
        <span><strong>{{ q.label }}</strong><small>{{ t('settings-thumbs-quality', { quality: q.webp }) }} {{ q.hint }}</small></span>
      </label>
    </div>
    <div class="switches motion">
      <div class="opt-row">
        <label class="check">
          <input
            type="checkbox"
            :checked="s.options.motion_thumbs.value"
            :disabled="busy"
            @change="save({ motion_thumbs: ($event.target as HTMLInputElement).checked })"
          />
          {{ t('settings-motion-thumbs') }}
        </label>
        <ResetButton v-if="changed('motion_thumbs')" :to="onOff(s.options.motion_thumbs.default)" :disabled="busy" @reset="resetOpt('motion_thumbs')" />
      </div>
    </div>
    <p class="what">{{ t('settings-motion-thumbs-what') }}</p>
    <template v-if="s.options.motion_thumbs.value">
      <div class="opt-row format">
        <span id="motion-fps">{{ t('settings-motion-fps') }}</span>
        <div class="modes" role="radiogroup" aria-labelledby="motion-fps">
          <label v-for="f in MOTION_FPS" :key="f">
            <input
              type="radio"
              name="motion-fps"
              :checked="s.options.motion_fps.value === f"
              :disabled="busy"
              @change="save({ motion_fps: f })"
            />{{ t('settings-motion-fps-option', { fps: f }) }}
          </label>
        </div>
        <ResetButton
          v-if="changed('motion_fps')"
          :to="t('settings-motion-fps-option', { fps: s.options.motion_fps.default })"
          :disabled="busy"
          @reset="resetOpt('motion_fps')"
        />
      </div>
      <p class="what">{{ t('settings-motion-fps-what') }}</p>
    </template>
  </section>

  <section>
    <div class="sh"><h3>{{ t('settings-limits') }}</h3></div>
    <p class="what">{{ t('settings-limits-what') }}</p>
    <div class="llm opts">
      <label for="opt-workers">{{ t('settings-workers') }}</label>
      <div class="key">
        <input
          id="opt-workers"
          v-model.number="opts.workers"
          class="field num"
          type="number"
          min="1"
          :max="s.max_workers"
          :disabled="busy"
          @change="saveOpt('workers')"
          @keydown.enter.prevent="saveOpt('workers')"
        />
        <span class="unit">{{ t('settings-workers-unit') }}</span>
        <ResetButton v-if="changed('workers')" :to="formatNumber(s.options.workers.default)" :disabled="busy" @reset="resetOpt('workers')" />
      </div>
      <label for="opt-upload">{{ t('settings-max-upload') }}</label>
      <div class="key">
        <SizeInput id="opt-upload" v-model="opts.max_upload_mb" class="num" :disabled="busy" @update:model-value="saveOpt('max_upload_mb')" />
        <span class="unit">{{ t('settings-max-upload-unit') }}</span>
        <ResetButton v-if="changed('max_upload_mb')" :to="formatMiB(s.options.max_upload_mb.default)" :disabled="busy" @reset="resetOpt('max_upload_mb')" />
      </div>
      <label for="opt-expand">{{ t('settings-max-expand') }}</label>
      <div class="key">
        <SizeInput id="opt-expand" v-model="opts.max_expand_mb" class="num" :disabled="busy" @update:model-value="saveOpt('max_expand_mb')" />
        <span class="unit">{{ t('settings-max-expand-unit') }}</span>
        <ResetButton v-if="changed('max_expand_mb')" :to="formatMiB(s.options.max_expand_mb.default)" :disabled="busy" @reset="resetOpt('max_expand_mb')" />
      </div>
      <label for="opt-conv">{{ t('settings-conversion-cache') }}</label>
      <div class="key">
        <SizeInput id="opt-conv" v-model="opts.conversion_cache_mb" class="num" :disabled="busy" @update:model-value="saveOpt('conversion_cache_mb')" />
        <span class="unit">
          {{ usage?.conversion_cache_bytes !== undefined ? t('settings-conversion-cache-unit-now', { size: humanBytes(usage.conversion_cache_bytes) }) : t('settings-conversion-cache-unit') }}
        </span>
        <ResetButton v-if="changed('conversion_cache_mb')" :to="formatMiB(s.options.conversion_cache_mb.default)" :disabled="busy" @reset="resetOpt('conversion_cache_mb')" />
      </div>
    </div>
  </section>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.switches.motion { margin: 0.9rem 0 0.2rem; }
.format { margin-bottom: 0.5rem; font-size: var(--step--1); color: var(--muted); }
/* How much space previews take, set apart from the text so it catches the eye. */
.size {
  display: inline-block; margin-left: 0.3em; padding: 0.05em 0.45em;
  border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--raised);
  color: var(--text); font-weight: 500; font-variant-numeric: tabular-nums; white-space: nowrap;
}
</style>
