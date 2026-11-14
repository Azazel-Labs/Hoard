<script setup lang="ts">
// A JSON file as a tree whose values can be clicked, for the format wizard: clicking one picks its path. Values a
// field already has are marked with the field's name.
import { computed } from 'vue'
import { pathKey, type JsonPath } from '../storeFormats'

defineOptions({ name: 'JsonPicker' })
const props = defineProps<{ value: unknown; path?: JsonPath; marks: Record<string, string> }>()
const emit = defineEmits<{ pick: [JsonPath] }>()

/** Keys or items shown at each level. */
const LIMIT = 60
const here = computed<JsonPath>(() => props.path ?? [])
const entries = computed<[string | number, unknown][]>(() => {
  const v = props.value
  if (Array.isArray(v)) return v.slice(0, LIMIT).map((x, i) => [i, x])
  if (v && typeof v === 'object') return Object.entries(v as Record<string, unknown>).slice(0, LIMIT)
  return []
})
const more = computed(() => {
  const v = props.value
  const n = Array.isArray(v) ? v.length : v && typeof v === 'object' ? Object.keys(v).length : 0
  return Math.max(0, n - LIMIT)
})
const scalar = (v: unknown) => v === null || typeof v !== 'object'
const show = (v: unknown) => {
  const s = typeof v === 'string' ? `"${v}"` : String(v)
  return s.length > 160 ? `${s.slice(0, 157)}…` : s
}
const at = (k: string | number) => [...here.value, k]
const mark = (k: string | number) => props.marks[pathKey(at(k))]
</script>

<template>
  <ul class="jp">
    <li v-for="[k, v] in entries" :key="k">
      <template v-if="scalar(v)">
        <span class="k" :class="{ idx: typeof k === 'number' }">{{ k }}</span>
        <button
          type="button"
          class="v"
          :class="[v === null ? 'null' : typeof v, { on: mark(k) }]"
          :disabled="v === null || v === ''"
          :title="typeof v === 'string' && v.length > 160 ? v : undefined"
          @click="emit('pick', at(k))"
        >{{ show(v) }}</button>
        <span v-if="mark(k)" class="mark">{{ mark(k) }}</span>
      </template>
      <details v-else open>
        <summary>
          <span class="k" :class="{ idx: typeof k === 'number' }">{{ k }}</span>
          <span class="t">{{ Array.isArray(v) ? `[${(v as unknown[]).length}]` : '{…}' }}</span>
        </summary>
        <JsonPicker :value="v" :path="at(k)" :marks="marks" @pick="(p) => emit('pick', p)" />
      </details>
    </li>
    <li v-if="more" class="more">… {{ more }} more</li>
  </ul>
</template>

<style scoped>
.jp { margin: 0; padding: 0 0 0 1rem; list-style: none; }
.jp li { margin: 0.1rem 0; display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.35rem; }
.jp li > details { flex: 1 1 100%; }
summary { cursor: pointer; display: flex; gap: 0.35rem; align-items: baseline; }
.k { color: var(--muted); }
.k::after { content: ':'; }
.k.idx { font-variant-numeric: tabular-nums; }
.t { color: var(--muted); font-size: 0.9em; }
.v {
  font: inherit; text-align: left; padding: 0 0.3rem; border: 1px solid transparent; border-radius: 4px;
  background: none; color: var(--text); cursor: pointer; overflow-wrap: anywhere; min-width: 0;
}
.v.string { color: var(--accent); }
.v:hover:not(:disabled), .v:focus-visible { border-color: var(--accent); background: color-mix(in srgb, var(--accent) 10%, transparent); }
.v:disabled { cursor: default; color: var(--muted); }
.v.on { border-color: var(--accent); background: color-mix(in srgb, var(--accent) 16%, transparent); }
.mark {
  font-family: var(--font-ui, inherit); font-size: 0.8em; padding: 0 0.4rem; border-radius: 999px;
  background: var(--accent); color: var(--on-accent, #fff);
}
.more { color: var(--muted); font-style: italic; }
</style>
