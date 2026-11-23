<script setup lang="ts">
// Settings → Tags: every tag in the library, to tidy up. Pick duplicates and merge them into one, or rename a tag
// (merge it into a new name). The old names are remembered, so the taggers use the new one from then on.
import { computed, onMounted, ref, watch } from 'vue'
import { api, type Facet } from '../api'
import { setTagParents } from '../tagTree'
import { formatNumber, t } from '../i18n'
import { markSeen } from '../news'
import { prefs } from '../prefs'
import { confirmDialog, showError, showOk } from '../ui'
import NewPip from './NewPip.vue'
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
    const [all, a, tree] = await Promise.all([api.allTags(q.value.trim().toLowerCase()), api.tagAliases(), api.tagTree()])
    tags.value = all
    aliases.value = a
    edges.value = tree
    if (!q.value.trim()) everyCount.value = new Map(all.map((x) => [x.value, x.count]))
  } catch (e) {
    showError(t('tags-load-failed', { error: (e as Error).message }))
  }
}
onMounted(load)
let debounce: ReturnType<typeof setTimeout> | undefined
watch(q, () => {
  clearTimeout(debounce)
  limit.value = 200
  debounce = setTimeout(load, 200)
})

/** The list in the order picked: as the server sends it (most used first), or A to Z. */
const sorted = computed(() =>
  tags.value && prefs.tagSort === 'name' ? [...tags.value].sort((a, b) => a.value.localeCompare(b.value)) : (tags.value ?? []),
)
const count = computed(() => new Map((tags.value ?? []).map((t) => [t.value, t.count])))

// Tags put under broader ones (on their tag pages), drawn as a tree: each broader tag with what's under it.
const edges = ref<{ tag: string; parent: string }[]>([])
/** Every tag's count, from before anything was typed into the find box: the tree stays as it is meanwhile. */
const everyCount = ref(new Map<string, number>())
const tree = computed(() => {
  const under = new Map<string, string[]>()
  const isChild = new Set<string>()
  for (const e of edges.value) {
    under.set(e.parent, [...(under.get(e.parent) ?? []), e.tag])
    isChild.add(e.tag)
  }
  const rows: { tag: string; depth: number; key: string; parent?: string }[] = []
  // A tag under two broader ones shows under each; `path` stops a loop going on forever.
  const walk = (tag: string, depth: number, path: string[]) => {
    rows.push({ tag, depth, key: [...path, tag].join('>'), parent: path.at(-1) })
    if (path.includes(tag)) return
    for (const c of (under.get(tag) ?? []).sort()) walk(c, depth + 1, [...path, tag])
  }
  for (const root of [...under.keys()].filter((p) => !isChild.has(p)).sort()) walk(root, 0, [])
  return rows
})
/** The broader tags a tag goes under now. */
const parentsOf = (tag: string) => edges.value.filter((e) => e.tag === tag).map((e) => e.parent)
// Putting one tag under another right here, as its tag page does.
const child = ref('')
const parent = ref('')
const nesting = ref(false)
const nestReady = computed(() => {
  const [c, p] = [child.value.trim(), parent.value.trim()]
  return !!c && !!p && c.toLowerCase() !== p.toLowerCase()
})
async function setParents(tag: string, parents: string[], note: string) {
  const was = parentsOf(tag)
  try {
    await setTagParents(tag, parents)
    markSeen('tag-tree')
    await load()
    showOk(note, { label: t('asset-undo'), run: () => void setTagParents(tag, was).then(load).then(() => emit('changed')) })
    emit('changed')
  } catch (e) {
    showError(t('tag-page-save-failed', { error: (e as Error).message }))
  }
}
async function nest() {
  if (!nestReady.value || nesting.value) return
  const [c, p] = [child.value.trim().toLowerCase(), parent.value.trim().toLowerCase()]
  nesting.value = true
  const have = parentsOf(c)
  if (!have.includes(p)) await setParents(c, [...have, p], t('tag-page-parent-saved', { tag: c, parent: p }))
  child.value = ''
  parent.value = ''
  nesting.value = false
}
function unnest(tag: string, from: string) {
  void setParents(tag, parentsOf(tag).filter((p) => p !== from), t('tag-page-parent-removed', { tag, parent: from }))
}
/** Every tag, offered as each box is typed into. */
const known = computed(() => [...everyCount.value.keys()].sort())

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
  // Each side of the colon on its own: letters and digits in any script, with their marks and joiners.
  const part = (s: string) => s.replace(/[^\p{L}\p{M}\p{N}‌‍/.+]+/gu, '-').replace(/^-+|-+$/g, '')
  const raw = into.value.trim().toLowerCase().normalize('NFC')
  const colon = raw.indexOf(':')
  if (colon < 0) {
    const t = part(raw)
    return t && prefix.value ? prefix.value + t : t
  }
  const [ns, v] = [part(raw.slice(0, colon)), part(raw.slice(colon + 1))]
  return ns && v ? `${ns}:${v}` : ''
})
const ready = computed(() => !!target.value && picked.value.some((t) => t !== target.value))
const affected = computed(() => picked.value.reduce((n, t) => n + (count.value.get(t) ?? 0), 0))

async function merge() {
  if (!ready.value) return
  const from = picked.value.filter((t) => t !== target.value)
  const ok = await confirmDialog({
    title: renaming.value
      ? t('tags-rename-title', { tag: from[0] })
      : t('tags-merge-title', { count: from.length + (picked.value.includes(target.value) ? 1 : 0) }),
    body: renaming.value
      ? t('tags-rename-body', { count: affected.value, into: target.value })
      : t('tags-merge-body', { tags: from.join(', '), into: target.value, count: affected.value }),
    confirm: renaming.value ? t('tags-rename') : t('tags-merge'),
  })
  if (!ok) return
  busy.value = true
  try {
    const r = await api.mergeTags(from, target.value)
    showOk(t('tags-merged', { count: r.assets, into: r.into }))
    picked.value = []
    await load()
    emit('changed')
  } catch (e) {
    showError(t('tags-merge-failed', { error: (e as Error).message }))
  } finally {
    busy.value = false
  }
}

async function forget(a: { tag: string; into: string }) {
  try {
    await api.dropTagAlias(a.tag)
    aliases.value = aliases.value.filter((x) => x.tag !== a.tag)
  } catch (e) {
    showError(t('tags-forget-failed', { error: (e as Error).message }))
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
      <div class="sh"><h3>{{ t('tags-tree-heading') }}<NewPip id="tag-tree" /></h3></div>
      <p class="what">{{ t('tags-tree-what') }}</p>
      <form class="nest" @submit.prevent="nest">
        <label class="visually-hidden" for="tags-nest-child">{{ t('tags-tree-tag-label') }}</label>
        <input id="tags-nest-child" v-model="child" class="field mono" list="tags-known" autocomplete="off" spellcheck="false" :placeholder="t('tags-tree-tag-placeholder')" />
        <span class="under">{{ t('tags-tree-goes-under') }}</span>
        <label class="visually-hidden" for="tags-nest-parent">{{ t('tags-tree-parent-label') }}</label>
        <input id="tags-nest-parent" v-model="parent" class="field mono" list="tags-known" autocomplete="off" spellcheck="false" :placeholder="t('tag-page-parent-placeholder')" />
        <button class="btn" type="submit" :disabled="!nestReady || nesting">{{ t('tags-tree-put') }}</button>
        <datalist id="tags-known"><option v-for="tag in known" :key="tag" :value="tag" /></datalist>
      </form>
      <p v-if="!tree.length" class="what none">{{ t('tags-tree-none') }}</p>
      <ul v-else class="list tree">
        <li v-for="row in tree" :key="row.key" :style="{ paddingInlineStart: `${row.depth * 1.25}rem` }">
          <span v-if="row.depth" class="branch" aria-hidden="true">└</span>
          <button type="button" class="linkbtn mono grow" :title="t('tags-browse-title', { tag: row.tag })" @click="emit('tag', row.tag)">{{ row.tag }}</button>
          <span class="n">{{ formatNumber(everyCount.get(row.tag) ?? 0) }}</span>
          <button
            v-if="row.parent"
            type="button"
            class="btn ghost small unnest"
            :title="t('tag-page-parent-remove', { tag: row.tag, parent: row.parent })"
            :aria-label="t('tag-page-parent-remove', { tag: row.tag, parent: row.parent })"
            @click="unnest(row.tag, row.parent)"
          >
            <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
          </button>
          <span v-else class="unnest-room" aria-hidden="true" />
        </li>
      </ul>
    </section>

    <section>
      <div class="find">
        <label class="visually-hidden" for="tags-q">{{ t('tags-find') }}</label>
        <input id="tags-q" v-model="q" class="field" type="search" :placeholder="t('tags-find-placeholder')" />
        <span v-if="tags" class="muted">{{ tags.length >= 2000 ? t('tags-count-more', { count: tags.length }) : t('tags-count', { count: tags.length }) }}</span>
        <div class="order" role="radiogroup" :aria-label="t('tags-sort')">
          <label v-for="o in (['count', 'name'] as const)" :key="o">
            <input v-model="prefs.tagSort" type="radio" name="tags-sort" :value="o" />{{ o === 'count' ? t('tags-sort-count') : t('tags-sort-name') }}
          </label>
        </div>
      </div>
      <p class="what">{{ t('tags-what') }}</p>

      <form v-if="picked.length" class="merge" @submit.prevent="merge">
        <div class="picked">
          <TagChip v-for="tag in picked" :key="tag" :tag="tag" removable @remove="toggle(tag)" />
        </div>
        <label class="into">
          <span>{{ renaming ? t('tags-new-name') : t('tags-merge-into') }}</span>
          <input v-model="into" class="field mono" list="tags-picked" autocomplete="off" spellcheck="false" />
          <datalist id="tags-picked"><option v-for="tag in picked" :key="tag" :value="tag" /></datalist>
        </label>
        <p v-if="target && target !== into.trim()" class="what">{{ t('tags-saved-as-before') }} <span class="mono">{{ target }}</span>{{ t('tags-saved-as-after') }}</p>
        <div class="row">
          <button class="btn primary" type="submit" :disabled="!ready || busy">{{ renaming ? t('tags-rename') : t('tags-merge') }}</button>
          <button class="btn ghost" type="button" @click="picked = []">{{ t('tags-cancel') }}</button>
        </div>
      </form>

      <p v-if="!tags" class="what">{{ t('tags-loading') }}</p>
      <p v-else-if="!tags.length" class="what none">{{ q ? t('tags-none-containing', { text: q.trim() }) : t('tags-none') }}</p>
      <ul v-else class="list">
        <li v-for="tag in sorted.slice(0, limit)" :key="tag.value" :class="{ on: picked.includes(tag.value) }">
          <label class="pick">
            <input type="checkbox" :checked="picked.includes(tag.value)" @change="toggle(tag.value)" />
            <span class="mono">{{ tag.value }}</span>
          </label>
          <span class="n">{{ formatNumber(tag.count) }}</span>
          <button type="button" class="btn ghost small" :title="t('tags-browse-title', { tag: tag.value })" @click="emit('tag', tag.value)">{{ t('tags-browse') }}</button>
        </li>
      </ul>
      <div v-if="tags && tags.length > limit" ref="moreEl" class="more" aria-hidden="true" />
    </section>

    <section v-if="aliases.length">
      <div class="sh"><h3>{{ t('tags-merged-heading') }}</h3></div>
      <p class="what">{{ t('tags-merged-what') }}</p>
      <ul class="list">
        <li v-for="a in aliases" :key="a.tag">
          <span class="mono grow">{{ a.tag }} → {{ a.into }}</span>
          <button type="button" class="btn ghost small" @click="forget(a)">{{ t('tags-allow-old') }}</button>
        </li>
      </ul>
    </section>
  </div>
</template>

<style scoped>
/* Its sections are cards like the rest of Settings' (that look is SettingsPage's). */
.tags { display: grid; gap: 1rem; font-size: var(--step--1); }
section { display: grid; gap: 0.6rem; }
.sh h3 { margin: 0; }
.what { margin: 0; color: var(--muted); line-height: 1.5; }
.none { font-style: italic; }
.find { display: flex; gap: 0.75rem; align-items: center; }
.find .field { flex: 1; min-width: 0; max-width: 26rem; }
.muted { color: var(--muted); white-space: nowrap; }
/* Most used or A to Z: two segments, the radios inside them. */
.order { display: inline-flex; gap: 2px; margin-left: auto; padding: 2px; border: 1px solid var(--line); border-radius: 8px; background: var(--well); }
.order label {
  position: relative; padding: 0.25em 0.75em; border-radius: 6px; color: var(--muted); cursor: pointer; white-space: nowrap;
  transition: background-color var(--dur-1), color var(--dur-1);
}
.order label:hover { color: var(--text); }
.order label:has(:checked) { background: var(--raised); color: var(--text); font-weight: 600; box-shadow: var(--elev-1), 0 0 0 1px var(--line); }
.order label:has(:focus-visible) { outline: 2px solid var(--accent); outline-offset: -2px; }
.order input { position: absolute; opacity: 0; pointer-events: none; }
.merge {
  position: sticky; top: 0; z-index: 2; display: grid; gap: 0.6rem; padding: 0.8rem 0.9rem;
  border: 1px solid var(--card-on-line); border-radius: 8px; background: var(--raised); box-shadow: var(--elev-2);
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
.tree li { gap: 0.4rem; }
.tree .linkbtn { padding-inline: 0; text-align: start; justify-content: flex-start; }
.branch { color: var(--faint); }
.nest { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; }
.nest .field { flex: 1 1 10rem; min-width: 0; max-width: 16rem; }
.under { color: var(--muted); white-space: nowrap; }
.unnest, .unnest-room { flex: none; width: 1.9rem; display: inline-grid; place-items: center; }
.unnest { color: var(--muted); }
.unnest:hover { color: var(--text); }
@media (max-width: 560px) { .into { flex-direction: column; align-items: stretch; } }
</style>
