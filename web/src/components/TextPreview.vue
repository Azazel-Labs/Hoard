<script setup lang="ts">
// The side panel's peek at a text file: its first lines, coloured, and a click opens the full-screen viewer.
// Only the first few kilobytes are fetched, however big the file.
import { computed, ref, watch } from 'vue'
import type { Asset } from '../api'
import { fetchText, highlightLines, langOf, reflowJson, type TextFile } from '../text'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ expand: [] }>()

const LINES = 18
const file = ref<TextFile | null>(null)
const failed = ref(false)

watch(
  () => props.asset.id,
  async (id) => {
    file.value = null
    failed.value = false
    try {
      const f = await fetchText(props.asset, 16 * 1024)
      if (id === props.asset.id) file.value = f
    } catch {
      if (id === props.asset.id) failed.value = true
    }
  },
  { immediate: true },
)

const peek = computed(() => {
  const f = file.value
  if (!f || f.binary) return { html: '', more: false }
  const ext = props.asset.ext.toLowerCase()
  // Minified JSON is one enormous line; re-indented, the preview shows its shape.
  const text = ext === 'json' || ext === 'ldtk' ? reflowJson(f.text) : f.text
  const all = text.split('\n')
  const lines = highlightLines(all.slice(0, LINES).join('\n'), langOf(ext), ext)
  return { html: lines.map((l) => `<span class="l">${l}</span>`).join(''), more: all.length > LINES || f.truncated }
})
const html = computed(() => peek.value.html)
const more = computed(() => peek.value.more)
</script>

<template>
  <button v-if="!file?.binary && !failed" type="button" class="peek" :class="{ more }" :aria-label="`Read ${asset.name} full screen`" @click="emit('expand')">
    <p v-if="!file" class="wait">Loading…</p>
    <!-- eslint-disable-next-line vue/no-v-html -- text.ts escapes every character of the file before adding spans. -->
    <pre v-else class="code-lines" style="--gutter: 2ch" v-html="html" />
    <span class="badge">
      <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
      Read
    </span>
  </button>
</template>

<style scoped>
.peek {
  position: relative; display: block; width: 100%; max-height: 17.5rem; overflow: hidden; padding: 0.6rem 0.7rem;
  border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--raised); text-align: left;
  cursor: zoom-in; box-shadow: inset 0 3px 0 var(--k-data); color: var(--text); font: inherit;
  transition: border-color var(--dur-1);
}
.peek:hover { border-color: color-mix(in srgb, var(--muted) 60%, var(--line)); }
.peek.more::after {
  content: ''; position: absolute; inset: auto 0 0; height: 3.5rem; pointer-events: none;
  background: linear-gradient(transparent, var(--raised));
}
.code-lines { font-size: 0.72rem; overflow: hidden; }
.wait { margin: 0; color: var(--muted); font-size: var(--step--1); }
.badge {
  position: absolute; z-index: 1; top: 0.45rem; right: 0.45rem; display: inline-flex; align-items: center; gap: 0.3rem;
  padding: 0.15rem 0.5rem; border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--panel);
  color: var(--muted); font-size: 0.72rem; box-shadow: var(--elev-1); opacity: 0; transition: opacity var(--dur-2);
}
.peek:hover .badge, .peek:focus-visible .badge { opacity: 1; }
@media (hover: none) { .badge { opacity: 1; } }
</style>
