<script setup lang="ts">
// "Add to collection" button with a small menu: existing collections, or a new one by name.
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import type { Collection } from '../api'
import { t } from '../i18n'

const props = defineProps<{
  collections: Collection[]
  /** Collections the asset(s) are already in, shown checked. */
  current?: string[]
  label?: string
  busy?: boolean
  /** Open upward (for the selection bar at the bottom of the screen). */
  up?: boolean
}>()
const emit = defineEmits<{ add: [string]; remove: [string]; create: [string] }>()

const open = ref(false)
const draft = ref('')
const root = ref<HTMLElement | null>(null)
const input = ref<HTMLInputElement | null>(null)
const query = computed(() => draft.value.trim().toLowerCase())
const shown = computed(() =>
  query.value ? props.collections.filter((c) => c.name.toLowerCase().includes(query.value)) : props.collections,
)
const exact = computed(() => props.collections.some((c) => c.name.toLowerCase() === query.value))

function toggle() {
  open.value = !open.value
  if (open.value) nextTick(() => input.value?.focus())
}
function pick(c: Collection) {
  if (props.current?.includes(c.id)) emit('remove', c.id)
  else emit('add', c.id)
  open.value = false
}
function create() {
  const name = draft.value.trim()
  if (!name) return
  const existing = props.collections.find((c) => c.name.toLowerCase() === name.toLowerCase())
  if (existing) emit('add', existing.id)
  else emit('create', name)
  draft.value = ''
  open.value = false
}

function onDoc(e: MouseEvent) {
  if (root.value && !root.value.contains(e.target as Node)) open.value = false
}
function onEsc(e: KeyboardEvent) {
  if (e.key === 'Escape' && open.value) {
    e.stopPropagation()
    open.value = false
  }
}
watch(open, (o) => {
  if (o) {
    document.addEventListener('click', onDoc)
    document.addEventListener('keydown', onEsc, true)
  } else {
    document.removeEventListener('click', onDoc)
    document.removeEventListener('keydown', onEsc, true)
  }
})
onBeforeUnmount(() => {
  document.removeEventListener('click', onDoc)
  document.removeEventListener('keydown', onEsc, true)
})
</script>

<template>
  <div ref="root" class="picker">
    <button type="button" class="btn" :aria-expanded="open" :disabled="busy" @click="toggle">
      {{ label ?? t('collections-add') }}
    </button>
    <Transition name="pop">
    <div v-if="open" class="menu" :class="{ up }" role="dialog" :aria-label="t('collections-menu')">
      <form class="new" @submit.prevent="create">
        <label class="visually-hidden" for="collection-name">{{ t('collections-find-label') }}</label>
        <input
          id="collection-name"
          ref="input"
          v-model="draft"
          class="field"
          :placeholder="t('collections-find-placeholder')"
          autocomplete="off"
        />
        <button v-if="query && !exact" class="btn primary" type="submit">{{ t('collections-create') }}</button>
      </form>
      <ul v-if="shown.length">
        <li v-for="c in shown" :key="c.id">
          <button type="button" role="menuitemcheckbox" :aria-checked="current?.includes(c.id) ?? false" @click="pick(c)">
            <span class="tick" aria-hidden="true">{{ current?.includes(c.id) ? '✓' : '' }}</span>
            <span class="cname">{{ c.name }}</span>
            <span class="n mono">{{ c.asset_count }}</span>
          </button>
        </li>
      </ul>
      <p v-else-if="!query" class="empty">{{ t('collections-empty') }}</p>
    </div>
    </Transition>
  </div>
</template>

<style scoped>
.picker { position: relative; display: inline-block; }
.menu {
  position: absolute; left: 0; top: calc(100% + 4px); z-index: 40; width: 17rem; padding: 0.4rem;
  background: var(--raised); border: 1px solid var(--line); border-radius: var(--r-tile);
  box-shadow: var(--elev-3); transform-origin: top left;
}
.menu.up { top: auto; bottom: calc(100% + 6px); transform-origin: bottom left; }
.new { display: flex; gap: 0.35rem; }
.new .field { padding: 0.35em 0.55em; font-size: var(--step--1); }
ul { list-style: none; margin: 0.35rem 0 0; padding: 0; max-height: 15rem; overflow-y: auto; }
li button {
  display: grid; grid-template-columns: 1rem 1fr auto; gap: 0.4rem; align-items: center; width: 100%;
  padding: 0.4rem 0.45rem; border: 0; border-radius: var(--r-chip); background: none; cursor: pointer;
  text-align: left; font-size: var(--step--1);
}
li button:hover { background: var(--panel); }
li button { transition: background-color var(--dur-1); }
li button[aria-checked='true'] .tick { display: inline-block; animation: tick-pop var(--dur-3) var(--ease-pop); }
@keyframes tick-pop { from { scale: 0; } }
.tick { color: var(--ok); font-weight: 700; }
.cname { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.n { color: var(--faint); font-size: 0.75rem; }
.empty { margin: 0.5rem 0.25rem 0.2rem; font-size: 0.78rem; color: var(--faint); }
</style>
