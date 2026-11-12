<script setup lang="ts">
// Settings → Tags: every tag in the library, to tidy up. Pick duplicates and merge them into one, or rename a tag
// (merge it into a new name). The old names are remembered, so the taggers use the new one from then on.
import { computed, onMounted, ref, watch } from 'vue'
import { api, type Facet } from '../api'
import { confirmDialog, showError, showOk } from '../ui'
import TagChip from './TagChip.vue'

const emit = defineEmits<{ tag: [string]; changed: [] }>()

const q = ref('')
const tags = ref<Facet[] | null>(null)
const aliases = ref<{ tag: string; into: string }[]>([])
const picked = ref<string[]>([])
const into = ref('')
const busy = ref(false)
/** Rows shown; more come in as the list scrolls. */
const limit = ref(200)

async function load() {
  try {
    const [t, a] = await Promise.all([api.allTags(q.value.trim().toLowerCase()), api.tagAliases()])
    tags.value = t
    aliases.value = a
  } catch (e) {
    showError(`Couldn't load the tags: ${(e as Error).message}`)
  }
}
onMounted(load)
let debounce: ReturnType<typeof setTimeout> | undefined
watch(q, () => {
  clearTimeout(debounce)
  limit.value = 200
  debounce = setTimeout(load, 200)
})

const count = computed(() => new Map((tags.value ?? []).map((t) => [t.value, t.count])))
function toggle(tag: string) {
  picked.value = picked.value.includes(tag) ? picked.value.filter((t) => t !== tag) : [...picked.value, tag]
}
// The name to merge into starts as the most used of those picked; renaming one starts from its own name.
watch(picked, (now, was) => {
  if (!now.length) into.value = ''
  else if (!was.length || !now.includes(into.value) || now.length === 1) {
    into.value = [...now].sort((a, b) => (count.value.get(b) ?? 0) - (count.value.get(a) ?? 0))[0]
  }
})
const renaming = computed(() => picked.value.length === 1)
/** What the tags picked have in common before the colon (`set:`, `material:`), if anything. */
const prefix = computed(() => {
  const p = picked.value.map((t) => (t.includes(':') ? t.slice(0, t.indexOf(':') + 1) : ''))
  return p[0] && p.every((x) => x === p[0]) ? p[0] : ''
})
// The name as the server will keep it: lower case, words joined by dashes. A rename that leaves off the prefix keeps it,
// so renaming a set gives a set.
const target = computed(() => {
  const t = into.value.trim().toLowerCase().replace(/[^\p{L}\p{N}/.+:]+/gu, '-').replace(/^-+|-+$/g, '')
  return t && !t.includes(':') && prefix.value ? prefix.value + t : t
})
const ready = computed(() => !!target.value && picked.value.some((t) => t !== target.value))
const affected = computed(() => picked.value.reduce((n, t) => n + (count.value.get(t) ?? 0), 0))

async function merge() {
  if (!ready.value) return
  const from = picked.value.filter((t) => t !== target.value)
  const ok = await confirmDialog({
    title: renaming.value ? `Rename ${from[0]}?` : `Merge ${from.length + (picked.value.includes(target.value) ? 1 : 0)} tags?`,
    body: renaming.value
      ? `Its ${affected.value.toLocaleString()} assets get ${target.value} instead, and the taggers use the new name from now on.`
      : `${from.join(', ')} become ${target.value} on up to ${affected.value.toLocaleString()} assets. The taggers use ${target.value} from now on.`,
    confirm: renaming.value ? 'Rename' : 'Merge',
  })
  if (!ok) return
  busy.value = true
  try {
    const r = await api.mergeTags(from, target.value)
    showOk(`${r.assets.toLocaleString()} ${r.assets === 1 ? 'asset' : 'assets'} now tagged ${r.into}`)
    picked.value = []
    await load()
    emit('changed')
  } catch (e) {
    showError(`Couldn't merge: ${(e as Error).message}`)
  } finally {
    busy.value = false
  }
}

async function forget(a: { tag: string; into: string }) {
  try {
    await api.dropTagAlias(a.tag)
    aliases.value = aliases.value.filter((x) => x.tag !== a.tag)
  } catch (e) {
    showError(`Couldn't change that: ${(e as Error).message}`)
  }
}

const moreEl = ref<HTMLElement | null>(null)
let observer: IntersectionObserver | undefined
watch(moreEl, (el) => {
  observer?.disconnect()
  if (!el) return
  observer = new IntersectionObserver((e) => {
    if (e.some((x) => x.isIntersecting)) limit.value += 200
  }, { rootMargin: '800px 0px' })
  observer.observe(el)
})
</script>

<template>
  <div class="tags">
    <section>
      <div class="find">
        <label class="visually-hidden" for="tags-q">Find tags</label>
        <input id="tags-q" v-model="q" class="field" type="search" placeholder="Find tags, e.g. rock or set:" />
        <span v-if="tags" class="muted">{{ tags.length.toLocaleString() }}{{ tags.length >= 2000 ? '+' : '' }} {{ tags.length === 1 ? 'tag' : 'tags' }}</span>
      </div>
      <p class="what">Pick tags that mean the same thing and merge them, or pick one to rename it.</p>

      <form v-if="picked.length" class="merge" @submit.prevent="merge">
        <div class="picked">
          <TagChip v-for="t in picked" :key="t" :tag="t" removable @remove="toggle(t)" />
        </div>
        <label class="into">
          <span>{{ renaming ? 'New name' : 'Merge into' }}</span>
          <input v-model="into" class="field mono" list="tags-picked" autocomplete="off" spellcheck="false" />
          <datalist id="tags-picked"><option v-for="t in picked" :key="t" :value="t" /></datalist>
        </label>
        <p v-if="target && target !== into.trim()" class="what">Saved as <span class="mono">{{ target }}</span></p>
        <div class="row">
          <button class="btn primary" type="submit" :disabled="!ready || busy">{{ renaming ? 'Rename' : 'Merge' }}</button>
          <button class="btn ghost" type="button" @click="picked = []">Cancel</button>
        </div>
      </form>

      <p v-if="!tags" class="what">Loading…</p>
      <p v-else-if="!tags.length" class="what none">No tags{{ q ? ` containing “${q.trim()}”` : '' }}.</p>
      <ul v-else class="list">
        <li v-for="t in tags.slice(0, limit)" :key="t.value" :class="{ on: picked.includes(t.value) }">
          <label class="pick">
            <input type="checkbox" :checked="picked.includes(t.value)" @change="toggle(t.value)" />
            <span class="mono">{{ t.value }}</span>
          </label>
          <span class="n">{{ t.count.toLocaleString() }}</span>
          <button type="button" class="btn ghost small" :title="`Browse assets tagged ${t.value}`" @click="emit('tag', t.value)">Browse</button>
        </li>
      </ul>
      <div v-if="tags && tags.length > limit" ref="moreEl" class="more" aria-hidden="true" />
    </section>

    <section v-if="aliases.length">
      <div class="sh"><h3>Merged tags</h3></div>
      <p class="what">When the taggers come up with one of these old names, the asset gets the tag it was merged into.</p>
      <ul class="list">
        <li v-for="a in aliases" :key="a.tag">
          <span class="mono grow">{{ a.tag }} → {{ a.into }}</span>
          <button type="button" class="btn ghost small" @click="forget(a)">Allow the old name again</button>
        </li>
      </ul>
    </section>
  </div>
</template>

<style scoped>
.tags { display: grid; gap: 1.8rem; font-size: var(--step--1); }
section { display: grid; gap: 0.6rem; }
.sh h3 { margin: 0; }
.what { margin: 0; color: var(--muted); line-height: 1.5; }
.none { font-style: italic; }
.find { display: flex; gap: 0.75rem; align-items: center; }
.find .field { flex: 1; min-width: 0; max-width: 26rem; }
.muted { color: var(--muted); white-space: nowrap; }
.merge {
  position: sticky; top: 0; z-index: 2; display: grid; gap: 0.6rem; padding: 0.8rem 0.9rem;
  border: 1px solid var(--accent); border-radius: var(--r-chip); background: var(--raised); box-shadow: var(--elev-2);
}
.picked { display: flex; flex-wrap: wrap; gap: 0.35rem; }
.into { display: flex; gap: 0.6rem; align-items: center; }
.into span { color: var(--muted); white-space: nowrap; }
.into .field { flex: 1; min-width: 0; max-width: 22rem; }
.row { display: flex; gap: 0.5rem; }
.list { margin: 0; padding: 0; list-style: none; border-top: 1px solid var(--line); }
.list li { display: flex; gap: 0.8rem; align-items: center; padding: 0.3rem 0; border-bottom: 1px solid var(--line); }
.list li.on { background: color-mix(in srgb, var(--accent) 8%, transparent); }
.pick { flex: 1; min-width: 0; display: flex; gap: 0.55rem; align-items: center; cursor: pointer; }
.pick .mono { overflow-wrap: anywhere; }
.n { color: var(--muted); font-variant-numeric: tabular-nums; }
.grow { flex: 1; min-width: 0; overflow-wrap: anywhere; }
.small { padding-block: 0.1rem; font-size: 0.85em; }
.more { height: 1px; }
@media (max-width: 560px) { .into { flex-direction: column; align-items: stretch; } }
</style>
