<script setup lang="ts">
// Full-screen text viewer: JSON as a foldable tree (or re-indented, or as written), everything else with line
// numbers and light colouring. Big files show their first megabyte until asked for the rest.
import { computed, onMounted, provide, ref } from 'vue'
import { fileUrl, humanBytes, type Asset } from '../api'
import { LANG_LABEL, fetchText, highlightLines, langOf, reflowJson, type TextFile } from '../text'
import { copyText, showError } from '../ui'
import JsonTree from './JsonTree.vue'

const props = defineProps<{ asset: Asset }>()
const emit = defineEmits<{ close: [] }>()

/** What opens at first; "Show all" fetches the rest. Past this the page gets slow to draw as text. */
const CAP = 1024 * 1024
/** JSON that follows the spec (no comments), which can be parsed and re-indented. */
const STRICT_JSON = new Set(['json', 'ldtk', 'gltf'])

type Mode = 'tree' | 'pretty' | 'raw'

const dlg = ref<HTMLDialogElement | null>(null)
const file = ref<TextFile | null>(null)
const status = ref<'loading' | 'ready' | 'error'>('loading')
const error = ref('')
const ext = computed(() => props.asset.ext.toLowerCase())
const lang = computed(() => langOf(ext.value))
// Code and scripts say which language (GLSL, CS, PY) rather than the generic name of their highlighter.
const label = computed(() => (['c', 'hash', 'plain'].includes(lang.value) && ext.value ? ext.value.toUpperCase() : LANG_LABEL[lang.value]))
const strict = computed(() => STRICT_JSON.has(ext.value))
const mode = ref<Mode>('raw')
// Prose reads best wrapped; code and data keep their lines.
const wrap = ref(lang.value === 'plain' || lang.value === 'markdown')
const copied = ref(false)
const fold = ref({ gen: 0, open: true })
provide('jsonFold', fold)

const parsed = computed<{ ok: true; value: unknown } | { ok: false; error: string } | null>(() => {
  if (!file.value || lang.value !== 'json' || file.value.truncated) return null
  try {
    return { ok: true, value: JSON.parse(file.value.text) }
  } catch (e) {
    return { ok: false, error: (e as Error).message }
  }
})

async function load(all = false) {
  status.value = 'loading'
  try {
    file.value = await fetchText(props.asset, all ? undefined : CAP)
    if (mode.value === 'raw' && lang.value === 'json') mode.value = parsed.value?.ok ? 'tree' : strict.value ? 'pretty' : 'raw'
    status.value = 'ready'
  } catch (e) {
    status.value = 'error'
    error.value = (e as Error).message
  }
}

const shownText = computed(() => {
  const t = file.value?.text ?? ''
  return mode.value === 'pretty' ? reflowJson(t) : t
})
const lineCount = computed(() => (shownText.value.match(/\n/g)?.length ?? 0) + 1)
const html = computed(() =>
  highlightLines(shownText.value, lang.value, ext.value)
    .map((l) => `<span class="l">${l}</span>`)
    .join(''),
)

async function copy() {
  if (!(await copyText(shownText.value))) {
    showError('Couldn’t copy: the browser blocked it')
    return
  }
  copied.value = true
  setTimeout(() => (copied.value = false), 1400)
}

function setFold(open: boolean) {
  fold.value = { gen: fold.value.gen + 1, open }
}

function onKey(e: KeyboardEvent) {
  // The page's own shortcuts (Esc closes the detail panel, arrows move between tiles) stay out of the viewer.
  e.stopPropagation()
  if (e.ctrlKey || e.metaKey || e.altKey || status.value !== 'ready') return
  const act: Record<string, () => void> = {
    w: () => (wrap.value = !wrap.value),
    c: copy,
  }
  if (lang.value === 'json') {
    if (parsed.value?.ok) act.t = () => (mode.value = 'tree')
    if (strict.value) act.p = () => (mode.value = 'pretty')
    act.r = () => (mode.value = 'raw')
  }
  const f = act[e.key.toLowerCase()]
  if (!f) return
  e.preventDefault()
  f()
}

onMounted(() => {
  dlg.value?.showModal()
  load()
})
</script>

<template>
  <dialog ref="dlg" class="viewer" :aria-label="`Read ${asset.name}`" @close="emit('close')" @keydown="onKey">
    <header class="tools">
      <div class="title">
        <strong>{{ asset.name }}</strong>
        <span class="mono dim">
          {{ label }} · {{ humanBytes(asset.size) }}<template v-if="file && mode !== 'tree'"> · {{ lineCount.toLocaleString() }} {{ lineCount === 1 ? 'line' : 'lines' }}</template>
        </span>
      </div>
      <div v-if="lang === 'json' && file && !file.binary" class="group" role="group" aria-label="Show as">
        <button type="button" title="Foldable tree (T)" :aria-pressed="mode === 'tree'" :disabled="!parsed?.ok" @click="mode = 'tree'">Tree</button>
        <button v-if="strict" type="button" title="Re-indented (P)" :aria-pressed="mode === 'pretty'" @click="mode = 'pretty'">Pretty</button>
        <button type="button" title="As written (R)" :aria-pressed="mode === 'raw'" @click="mode = 'raw'">Raw</button>
      </div>
      <div v-if="mode === 'tree' && parsed?.ok" class="group" role="group" aria-label="Fold">
        <button type="button" title="Unfold everything" @click="setFold(true)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m4.5 6 3.5 3.5L11.5 6" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">Expand all</span>
        </button>
        <button type="button" title="Fold everything" @click="setFold(false)">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M6 4.5 9.5 8 6 11.5" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">Collapse all</span>
        </button>
      </div>
      <div class="group" role="group" aria-label="Text">
        <button v-if="mode !== 'tree'" type="button" title="Wrap long lines (W)" :aria-pressed="wrap" :disabled="!file || file.binary" @click="wrap = !wrap">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M2 4h12M2 8h9.5a2.25 2.25 0 0 1 0 4.5H8m1.6-1.8L7.8 12.5l1.8 1.8M2 12.5h3" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">Wrap</span>
        </button>
        <button type="button" title="Copy the text (C)" :disabled="!file || file.binary" @click="copy">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><rect x="5" y="5" width="8.5" height="8.5" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.5" /><path d="M10.5 3.2V3a1 1 0 0 0-1-1H3.5a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h.3" fill="none" stroke="currentColor" stroke-width="1.5" /></svg>
          <span class="wide" aria-live="polite">{{ copied ? 'Copied' : 'Copy' }}</span>
        </button>
        <a class="btn" :href="fileUrl(asset, true)" download title="Download the file">
          <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M8 2.5v8m-3.2-3.2L8 10.5l3.2-3.2M3 13.5h10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" /></svg>
          <span class="wide">Download</span>
        </a>
      </div>
      <button type="button" class="close" aria-label="Close (Esc)" title="Close (Esc)" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <p v-if="file?.truncated && status === 'ready'" class="banner">
      Showing the first {{ humanBytes(CAP) }} of {{ humanBytes(file.total) }}{{ lang === 'json' ? ' (the tree needs the whole file)' : '' }}.
      <button type="button" class="linkish" @click="load(true)">Show all</button>
    </p>
    <p v-else-if="parsed && !parsed.ok && status === 'ready'" class="banner">This JSON doesn't parse: {{ parsed.error }}</p>

    <div class="stage scroll" tabindex="0" :aria-label="`${asset.name} contents`">
      <p v-if="status === 'loading'" class="msg">Loading…</p>
      <p v-else-if="status === 'error'" class="msg err">Couldn't load this file: {{ error }}</p>
      <p v-else-if="file?.binary" class="msg">This file is binary, not text, so there's nothing to read here. Download it to open it in its own tool.</p>
      <div v-else-if="mode === 'tree' && parsed?.ok" class="tree">
        <JsonTree :value="parsed.value" />
      </div>
      <!-- eslint-disable-next-line vue/no-v-html -- text.ts escapes every character of the file before adding spans. -->
      <pre v-else class="code-lines" :class="{ wrap }" :style="{ '--gutter': `${String(lineCount).length}ch` }" v-html="html" />
    </div>
    <footer class="keys" aria-hidden="true">
      <template v-if="lang === 'json'"><kbd>T</kbd> tree · <template v-if="strict"><kbd>P</kbd> pretty · </template><kbd>R</kbd> raw · </template>
      <kbd>W</kbd> wrap · <kbd>C</kbd> copy · <kbd>Ctrl</kbd>+<kbd>F</kbd> find · <kbd>Esc</kbd> close
    </footer>
  </dialog>
</template>

<style scoped src="./viewerChrome.css"></style>
<style scoped>
.scroll { overflow: auto; padding: 0.75rem 1rem 2rem; background: var(--panel); outline-offset: -2px; }
.tree { font-variation-settings: 'MONO' 1, 'CASL' 0; font-size: 0.8rem; line-height: 1.55; }
.code-lines { width: max-content; min-width: 100%; }
.code-lines.wrap { width: auto; }
.err { color: var(--danger); }
.banner {
  margin: 0; padding: 0.4rem 0.75rem; border-bottom: 1px solid var(--line); background: var(--raised);
  color: var(--muted); font-size: var(--step--1);
}
.linkish { padding: 0; border: 0; background: none; color: var(--accent); font: inherit; cursor: pointer; text-decoration: underline; }
</style>
