<script setup lang="ts">
// Ctrl/Cmd-K: jump anywhere, run anything, or find an asset by name without leaving the keyboard.
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { api, thumbUrl, type Asset } from '../api'
import KindGlyph from './KindGlyph.vue'
import type { Command } from '../ui'

const props = defineProps<{ commands: Command[] }>()
const emit = defineEmits<{ close: []; open: [Asset] }>()

/** Groups shown before anything is typed; the long lists (packs, collections) wait for a query. */
const IDLE_GROUPS = new Set(['Go to', 'Actions', 'Theme'])
const PER_GROUP = 6

interface Row {
  key: string
  group: string
  label: string
  segs: { text: string; hit: boolean }[]
  hint?: string
  asset?: Asset
  run: () => void
}

const dlg = ref<HTMLDialogElement | null>(null)
const input = ref<HTMLInputElement | null>(null)
const list = ref<HTMLElement | null>(null)
const query = ref('')
const active = ref(0)
const assets = ref<Asset[]>([])
const searching = ref(false)

/** Case-insensitive match: a run of the query scores best (earlier and at a word start better still), then the
 *  query's letters in order with gaps between them. */
function match(text: string, q: string): { score: number; hits: number[] } | null {
  const t = text.toLowerCase()
  const at = t.indexOf(q)
  if (at >= 0) {
    const wordStart = at === 0 || /[\s\-_/.(]/.test(t[at - 1])
    return { score: (wordStart ? 0 : 2) + at / 100, hits: Array.from({ length: q.length }, (_, k) => at + k) }
  }
  const hits: number[] = []
  let i = 0
  for (const ch of q) {
    if (ch === ' ') continue
    i = t.indexOf(ch, i)
    if (i < 0) return null
    hits.push(i++)
  }
  return { score: 10 + (hits[hits.length - 1] - hits[0]) / 10, hits }
}

function segments(label: string, hits: number[]) {
  const set = new Set(hits)
  const out: Row['segs'] = []
  for (let k = 0; k < label.length; k++) {
    const hit = set.has(k)
    const last = out[out.length - 1]
    if (last && last.hit === hit) last.text += label[k]
    else out.push({ text: label[k], hit })
  }
  return out
}

const rows = computed<Row[]>(() => {
  const q = query.value.trim().toLowerCase()
  const order: string[] = []
  const byGroup = new Map<string, { score: number; row: Row }[]>()
  for (const c of props.commands) {
    if (!q && !IDLE_GROUPS.has(c.group)) continue
    let score = 0
    let hits: number[] = []
    if (q) {
      const m = match(c.label, q) ?? (c.keywords ? match(c.keywords, q) : null)
      if (!m) continue
      score = m.score
      // A keyword match has no letters to highlight in the label.
      hits = match(c.label, q) ? m.hits : []
      if (!hits.length) score += 5
    }
    if (!byGroup.has(c.group)) {
      byGroup.set(c.group, [])
      order.push(c.group)
    }
    byGroup.get(c.group)!.push({
      score,
      row: { key: c.id, group: c.group, label: c.label, segs: segments(c.label, hits), hint: c.hint, run: c.run },
    })
  }
  // Groups whose best match is strongest come first; the idle list keeps its natural order.
  const groups = order.map((g) => {
    const items = byGroup.get(g)!
    if (q) items.sort((a, b) => a.score - b.score)
    return { g, best: q ? items[0].score : 0, items: q ? items.slice(0, PER_GROUP) : items }
  })
  if (q) groups.sort((a, b) => a.best - b.best)
  const out = groups.flatMap((x) => x.items.map((i) => i.row))
  for (const a of assets.value) {
    out.push({
      key: `asset:${a.id}`, group: 'Assets', label: a.name, segs: segments(a.name, match(a.name, q)?.hits ?? []),
      hint: a.pack_name, asset: a, run: () => emit('open', a),
    })
  }
  return out
})

const sections = computed(() => {
  const out: { group: string; rows: (Row & { i: number })[] }[] = []
  rows.value.forEach((r, i) => {
    if (out[out.length - 1]?.group !== r.group) out.push({ group: r.group, rows: [] })
    out[out.length - 1].rows.push({ ...r, i })
  })
  return out
})

// Live asset search, debounced, ignoring answers that come back after a newer query.
let timer: ReturnType<typeof setTimeout> | undefined
let seq = 0
watch(query, (v) => {
  active.value = 0
  clearTimeout(timer)
  const q = v.trim()
  if (q.length < 2) {
    assets.value = []
    searching.value = false
    return
  }
  searching.value = true
  const mine = ++seq
  timer = setTimeout(async () => {
    try {
      const r = await api.search({ q, limit: 6 })
      if (mine === seq) assets.value = r.items
    } catch {
      if (mine === seq) assets.value = []
    } finally {
      if (mine === seq) searching.value = false
    }
  }, 140)
})
watch(rows, (r) => {
  if (active.value >= r.length) active.value = Math.max(0, r.length - 1)
})

function move(d: number) {
  const n = rows.value.length
  if (!n) return
  active.value = (active.value + d + n) % n
  void nextTick(() => list.value?.querySelector('[aria-selected="true"]')?.scrollIntoView({ block: 'nearest' }))
}
function run(row: Row | undefined) {
  if (!row) return
  dlg.value?.close()
  row.run()
}
function onKey(e: KeyboardEvent) {
  if (e.key === 'ArrowDown' || (e.ctrlKey && e.key === 'n')) move(1)
  else if (e.key === 'ArrowUp' || (e.ctrlKey && e.key === 'p')) move(-1)
  else if (e.key === 'Enter') run(rows.value[active.value])
  else if (e.key === 'PageDown') move(5)
  else if (e.key === 'PageUp') move(-5)
  else return
  e.preventDefault()
}

onMounted(() => {
  dlg.value?.showModal()
  input.value?.focus()
})
</script>

<template>
  <dialog ref="dlg" class="palette" aria-label="Command palette" @close="emit('close')" @click.self="dlg?.close()">
    <div class="box">
      <div class="inputrow">
        <svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
          <circle cx="7" cy="7" r="4.5" fill="none" stroke="currentColor" stroke-width="1.6" />
          <path d="m10.5 10.5 3.5 3.5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
        </svg>
        <input
          ref="input"
          v-model="query"
          type="text"
          role="combobox"
          aria-expanded="true"
          aria-controls="palette-list"
          aria-autocomplete="list"
          :aria-activedescendant="rows[active] ? `pal-${active}` : undefined"
          placeholder="Search assets, go to a pack, run a command…"
          autocomplete="off"
          spellcheck="false"
          @keydown="onKey"
        />
        <span v-if="searching" class="spinner" aria-hidden="true" />
        <kbd>Esc</kbd>
      </div>
      <div id="palette-list" ref="list" class="list" role="listbox" aria-label="Results">
        <div v-for="s in sections" :key="s.group" role="group" :aria-label="s.group">
          <div class="group" aria-hidden="true">{{ s.group }}</div>
          <div
            v-for="r in s.rows"
            :id="`pal-${r.i}`"
            :key="r.key"
            class="row"
            role="option"
            :aria-selected="r.i === active"
            @pointermove="active = r.i"
            @click="run(r)"
          >
            <span v-if="r.asset" class="thumb checker" :style="{ '--kind': `var(--k-${r.asset.kind})` }">
              <img v-if="r.asset.has_thumb" :src="thumbUrl(r.asset)" :class="{ glyphs: r.asset.kind === 'font' }" alt="" />
              <KindGlyph v-else :kind="r.asset.kind" :size="16" />
            </span>
            <span class="label"><template v-for="(g, k) in r.segs" :key="k"><mark v-if="g.hit">{{ g.text }}</mark><template v-else>{{ g.text }}</template></template></span>
            <span v-if="r.hint" class="hint">{{ r.hint }}</span>
            <svg class="enter" viewBox="0 0 16 16" width="12" height="12" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
              <path d="M13 3v5.5a1.5 1.5 0 0 1-1.5 1.5H3M6 7 3 10l3 3" />
            </svg>
          </div>
        </div>
        <p v-if="!rows.length" class="empty">{{ searching ? 'Searching…' : `Nothing matches “${query.trim()}”` }}</p>
      </div>
      <footer class="foot" aria-hidden="true">
        <span><kbd>↑</kbd><kbd>↓</kbd> move</span>
        <span><kbd>Enter</kbd> run</span>
        <span><kbd>?</kbd> all shortcuts</span>
      </footer>
    </div>
  </dialog>
</template>

<style scoped>
.palette {
  width: min(40rem, calc(100vw - 2rem)); max-height: none; margin: 12vh auto auto; padding: 0;
  border: 1px solid var(--line); border-radius: 10px; background: var(--panel); color: var(--text);
  box-shadow: var(--elev-3), 0 0 0 1px color-mix(in srgb, var(--text) 4%, transparent);
  overflow: hidden;
}
.palette[open] { animation: pal-in var(--dur-3) var(--ease-pop); }
.palette::backdrop { background: var(--scrim); backdrop-filter: blur(2px); animation: fade-in var(--dur-2) var(--ease-out); }
@keyframes pal-in { from { opacity: 0; scale: 0.96; translate: 0 -10px; } }
@keyframes fade-in { from { opacity: 0; } }
.box { display: flex; flex-direction: column; max-height: min(32rem, 76vh); }
.inputrow {
  display: flex; align-items: center; gap: 0.6rem; padding: 0.8rem 0.95rem; border-bottom: 1px solid var(--line); color: var(--muted);
}
.inputrow input {
  flex: 1; min-width: 0; border: 0; outline: none; background: none; font-size: var(--step-1); color: var(--text);
}
.inputrow input::placeholder { color: var(--faint); }
.spinner {
  width: 14px; height: 14px; border-radius: 50%; border: 2px solid var(--line); border-top-color: var(--k-font);
  animation: spin 0.7s linear infinite;
}
@keyframes spin { to { rotate: 1turn; } }
.list { overflow-y: auto; padding: 0.35rem; scroll-padding: 2rem 0 0.35rem; overscroll-behavior: contain; }
.group {
  position: sticky; top: -0.35rem; z-index: 1; padding: 0.5rem 0.6rem 0.25rem; background: var(--panel);
  font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint);
}
.row {
  display: flex; align-items: center; gap: 0.6rem; min-height: 2.25rem; padding: 0.3rem 0.6rem; border-radius: 6px;
  cursor: pointer; font-size: var(--step-0); transition: background-color var(--dur-1);
}
.row[aria-selected='true'] { background: var(--raised); box-shadow: inset 2px 0 0 var(--k-font), var(--elev-1); }
.label { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
mark { background: none; color: inherit; font-weight: 700; text-decoration: underline 2px color-mix(in srgb, var(--k-font) 60%, transparent); text-underline-offset: 3px; }
.hint { max-width: 40%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: var(--step--1); color: var(--faint); }
.enter { flex: none; color: var(--muted); opacity: 0; translate: -4px 0; transition: opacity var(--dur-1), translate var(--dur-2) var(--ease-out); }
.row[aria-selected='true'] .enter { opacity: 1; translate: 0 0; }
.thumb {
  flex: none; display: grid; place-items: center; width: 26px; height: 26px; border-radius: 4px; overflow: hidden;
  box-shadow: inset 0 2px 0 var(--kind);
}
.thumb img { width: 100%; height: 100%; object-fit: contain; }
.empty { margin: 0; padding: 1.5rem 1rem; text-align: center; color: var(--muted); font-size: var(--step--1); }
.foot {
  display: flex; gap: 1rem; padding: 0.45rem 0.95rem; border-top: 1px solid var(--line);
  font-size: 0.72rem; color: var(--faint);
}
kbd {
  display: inline-block; min-width: 1.4em; padding: 0 0.35em; margin-right: 0.2em; border: 1px solid var(--line);
  border-bottom-width: 2px; border-radius: 4px; font: inherit; font-size: 0.72rem; text-align: center; color: var(--muted);
}
@media (max-width: 600px) {
  .palette { margin-top: 1rem; }
  .foot { display: none; }
}
</style>
