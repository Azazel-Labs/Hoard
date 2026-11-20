<script setup lang="ts">
// The side panel's peek at a text file, and a click opens the full-screen viewer. Markdown shows as a document,
// prose wraps, code and data show their first lines colored. Only the first few kilobytes are fetched, however
// big the file. A web page shows as the page, small, sandboxed as in the full-screen viewer.
import { computed, nextTick, ref, watch } from 'vue'
import { api, type Asset } from '../api'
import { t } from '../i18n'
import { renderMarkdown } from '../markdown'
import { fetchText, highlightLines, langOf, reflowJson, type TextFile } from '../text'
import { isPageAsset } from '../viewers'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ expand: [] }>()

const LINES = 18
const file = ref<TextFile | null>(null)
const failed = ref(false)
const page = computed(() => isPageAsset(props.asset))
const siteUrl = ref('')

watch(
  () => props.asset.id,
  async (id) => {
    file.value = null
    failed.value = false
    siteUrl.value = ''
    try {
      if (page.value) {
        const r = await api.assetSite(id)
        if (id === props.asset.id) siteUrl.value = r.url
        return
      }
      const f = await fetchText(props.asset, 16 * 1024)
      if (id === props.asset.id) file.value = f
    } catch {
      if (id === props.asset.id) failed.value = true
    }
  },
  { immediate: true },
)

const ext = computed(() => props.asset.ext.toLowerCase())
const isMarkdown = computed(() => langOf(ext.value) === 'markdown')
/** Plain prose wraps and needs no line numbers; .nfo files are ASCII art and keep their lines. */
const isProse = computed(() => langOf(ext.value) === 'plain' && ext.value !== 'nfo')
const doc = computed(() => (isMarkdown.value && file.value && !file.value.binary ? renderMarkdown(file.value.text) : ''))
const box = ref<HTMLElement | null>(null)
const overflows = ref(false)
watch([doc, file], async () => {
  await nextTick()
  const el = box.value
  overflows.value = !!el && el.scrollHeight > el.clientHeight + 1
})

const peek = computed(() => {
  const f = file.value
  if (!f || f.binary || isMarkdown.value) return { html: '', more: false }
  const ext = props.asset.ext.toLowerCase()
  // Minified JSON is one enormous line; re-indented, the preview shows its shape.
  const text = ext === 'json' || ext === 'ldtk' ? reflowJson(f.text) : f.text
  const all = text.split('\n')
  const lines = highlightLines(all.slice(0, LINES).join('\n'), langOf(ext), ext)
  return { html: lines.map((l) => `<span class="l">${l}</span>`).join(''), more: all.length > LINES || f.truncated }
})
const html = computed(() => peek.value.html)
const more = computed(() => peek.value.more || overflows.value)
</script>

<template>
  <div v-if="page && !failed" class="peek site">
    <!-- Someone else's page: it runs without an origin, so it can't reach Hoard, and loads only the pack's files. -->
    <iframe v-if="siteUrl" :src="siteUrl" :title="t('text-page-aria', { name: asset.name })" sandbox="allow-scripts" referrerpolicy="no-referrer" tabindex="-1" loading="lazy" />
    <p v-else class="wait">{{ t('text-loading') }}</p>
    <button type="button" class="open" :aria-label="t('text-preview-aria', { name: asset.name })" @click="emit('expand')">
      <span class="badge">
        <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
        {{ t('text-preview-read') }}
      </span>
    </button>
  </div>
  <button v-else-if="!page && !file?.binary && !failed" ref="box" type="button" class="peek" :class="{ more, doc: isMarkdown }" :aria-label="t('text-preview-aria', { name: asset.name })" @click="emit('expand')">
    <p v-if="!file" class="wait">{{ t('text-loading') }}</p>
    <!-- eslint-disable-next-line vue/no-v-html -- markdown.ts renders with raw HTML off, so the file can't add markup. -->
    <div v-else-if="isMarkdown" class="md-body" v-html="doc" />
    <!-- eslint-disable-next-line vue/no-v-html -- text.ts escapes every character of the file before adding spans. -->
    <pre v-else class="code-lines" :class="{ prose: isProse, wrap: isProse }" style="--gutter: 2ch" v-html="html" />
    <span class="badge">
      <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="M2.5 6V2.5H6M10 2.5h3.5V6M13.5 10v3.5H10M6 13.5H2.5V10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
      {{ t('text-preview-read') }}
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
/* Prose: wrapped, no line numbers, a size meant for reading. */
.code-lines.prose { font-size: 0.8rem; line-height: 1.6; }
.code-lines.prose :deep(.l) { padding-left: 0; }
.code-lines.prose :deep(.l::before) { content: none; }
.peek.doc { padding: 0.8rem 0.95rem; }
.md-body { font-size: 0.85rem; }
/* It's a preview: a click anywhere opens the reader, links included. */
.md-body :deep(a) { pointer-events: none; }
/* The page at twice the panel's width, shrunk to fit, so it lays out as on a small screen rather than a sliver. */
.peek.site { height: 15rem; padding: 0; overflow: hidden; background: #fff; }
.peek.site iframe {
  width: 200%; height: 200%; border: 0; transform: scale(0.5); transform-origin: 0 0; pointer-events: none;
  background: #fff; color-scheme: light;
}
.peek.site .wait { padding: 0.6rem 0.7rem; }
.open { position: absolute; inset: 0; padding: 0; border: 0; background: none; cursor: zoom-in; }
.open:hover .badge, .open:focus-visible .badge { opacity: 1; }
.wait { margin: 0; color: var(--muted); font-size: var(--step--1); }
.badge {
  position: absolute; z-index: 1; top: 0.45rem; right: 0.45rem; display: inline-flex; align-items: center; gap: 0.3rem;
  padding: 0.15rem 0.5rem; border: 1px solid var(--line); border-radius: var(--r-chip); background: var(--panel);
  color: var(--muted); font-size: 0.72rem; box-shadow: var(--elev-1); opacity: 0; transition: opacity var(--dur-2);
}
.peek:hover .badge, .peek:focus-visible .badge { opacity: 1; }
@media (hover: none) { .badge { opacity: 1; } }
</style>
