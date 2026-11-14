<script setup lang="ts">
// One value of a JSON document as a tree: objects and arrays fold, and a long one shows its children a page at a
// time, so a 50 MB level file opens as fast as a 5 KB one (only what's unfolded is drawn).
import { computed, inject, ref, watch, type Ref } from 'vue'
import { t } from '../i18n'

const props = withDefaults(defineProps<{ value: unknown; k?: string | number; depth?: number }>(), { depth: 0 })

const PAGE = 200
/** Bumped by the viewer's Expand all / Collapse all; `open` says which. */
const fold = inject<Ref<{ gen: number; open: boolean }>>('jsonFold', ref({ gen: 0, open: true }))

const isArray = computed(() => Array.isArray(props.value))
const isObject = computed(() => props.value !== null && typeof props.value === 'object')
const entries = computed<[string | number, unknown][]>(() => {
  if (!isObject.value) return []
  return isArray.value ? (props.value as unknown[]).map((v, i) => [i, v]) : Object.entries(props.value as object)
})
// The top levels start open; deeper ones too when they're small enough to take in at once.
const open = ref(props.depth < 2 || (props.depth < 4 && entries.value.length <= 4))
const limit = ref(PAGE)
const shown = computed(() => entries.value.slice(0, limit.value))
watch(
  () => fold.value.gen,
  () => (open.value = fold.value.open || props.depth === 0),
)

const scalar = computed(() => {
  const v = props.value
  if (v === null) return { cls: 'n', text: 'null' }
  if (typeof v === 'string') return { cls: 's', text: JSON.stringify(v) }
  return { cls: 'n', text: String(v) }
})
const summary = computed(() => {
  const n = entries.value.length
  return isArray.value ? t('json-items', { count: n }) : t('json-keys', { count: n })
})
</script>

<template>
  <div class="jn">
    <div class="row">
      <button
        v-if="isObject && entries.length"
        type="button"
        class="twist"
        :aria-expanded="open"
        :aria-label="open ? t('json-fold') : t('json-unfold')"
        @click="open = !open"
      >
        <svg viewBox="0 0 10 10" width="9" height="9" aria-hidden="true"><path d="M3 1.8 7 5 3 8.2z" fill="currentColor" /></svg>
      </button>
      <span v-else class="twist-space" />
      <!-- One flex item, so the row's gap doesn't land between a key and its colon. -->
      <span v-if="k !== undefined" class="key">
        <span v-if="typeof k === 'number'" class="idx">{{ k }}</span>
        <span v-else class="tk-key">{{ JSON.stringify(k) }}</span><span class="tk-p">:</span>
      </span>
      <template v-if="isObject">
        <span class="tk-p">{{ isArray ? '[' : '{' }}</span>
        <template v-if="!entries.length"><span class="tk-p">{{ isArray ? ']' : '}' }}</span></template>
        <template v-else-if="!open">
          <button type="button" class="ellipsis" :aria-label="t('json-unfold-summary', { summary })" @click="open = true">…</button>
          <span class="tk-p">{{ isArray ? ']' : '}' }}</span>
          <span class="count">{{ summary }}</span>
        </template>
      </template>
      <span v-else :class="`tk-${scalar.cls}`" class="scalar">{{ scalar.text }}</span>
    </div>
    <template v-if="isObject && entries.length && open">
      <div class="kids">
        <JsonTree v-for="[ck, cv] in shown" :key="ck" :value="cv" :k="ck" :depth="depth + 1" />
        <button v-if="entries.length > limit" type="button" class="more" @click="limit += PAGE">
          {{ t('json-show-more', { count: Math.min(PAGE, entries.length - limit) }) }}
          <span class="count">{{ t('json-left', { count: entries.length - limit }) }}</span>
        </button>
      </div>
      <div class="row close"><span class="twist-space" /><span class="tk-p">{{ isArray ? ']' : '}' }}</span></div>
    </template>
  </div>
</template>

<style scoped>
.row { display: flex; align-items: baseline; gap: 0.15rem; min-height: 1.55em; }
.scalar { overflow-wrap: anywhere; white-space: pre-wrap; }
.twist, .twist-space { flex: none; width: 1.1rem; height: 1.1rem; }
.twist {
  display: inline-grid; place-items: center; align-self: center; padding: 0; border: 0; border-radius: 3px;
  background: none; color: var(--faint); cursor: pointer; transition: rotate var(--dur-1) var(--ease-out), color var(--dur-1);
}
.twist:hover { color: var(--text); background: var(--raised); }
.twist[aria-expanded='true'] { rotate: 90deg; }
.idx { color: var(--faint); font-variant-numeric: tabular-nums; }
.key { margin-right: 0.35ch; white-space: nowrap; }
.ellipsis {
  padding: 0 0.35em; border: 1px solid var(--line); border-radius: 3px; background: var(--raised);
  color: var(--muted); font: inherit; line-height: 1.2; cursor: pointer;
}
.ellipsis:hover { color: var(--text); }
.count { margin-left: 0.6ch; color: var(--faint); font-size: 0.92em; }
.kids { margin-left: 0.5rem; padding-left: 0.6rem; border-left: 1px solid var(--line); }
.more {
  margin: 0.15rem 0 0.15rem 1.1rem; padding: 0.1rem 0.5rem; border: 1px dashed var(--line); border-radius: var(--r-chip);
  background: none; color: var(--accent); font: inherit; cursor: pointer;
}
.more:hover { background: var(--raised); }
</style>
