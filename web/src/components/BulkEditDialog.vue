<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { api, type Asset, type AssetPatch, type Facet, type SearchParams } from '../api'
import { t } from '../i18n'
import TagInput from './TagInput.vue'

// Edit many assets at once. Each tag they carry is a checkbox: ticked on all, unticked on none, and in between when
// only some have it, which leaves it as it is until it's clicked. The fields show the value they share, or say that
// they differ; only what's changed here is saved. With `query` it's everything a search matches, loaded or not.
export interface UndoGroup { ids: string[]; edit: AssetPatch }
type Edit = Omit<AssetPatch, 'name' | 'apply_to_family'>
const props = defineProps<{
  /** The assets picked (with `query`, nothing: the tags come from the search's counts instead). */
  assets: Asset[]
  /** Everything this search finds, rather than `assets`. */
  query?: SearchParams | null
  /** How many are being edited. */
  count: number
  /** Saves the edit; with what puts each asset back, for Undo. False if it failed, which keeps the dialog open. */
  apply: (edit: Edit, undo: UndoGroup[]) => Promise<boolean>
}>()
const emit = defineEmits<{ close: [] }>()

const dlg = ref<HTMLDialogElement | null>(null)
const tagBox = ref<InstanceType<typeof TagInput> | null>(null)
onMounted(() => {
  dlg.value?.showModal()
  tagBox.value?.focus()
})
const busy = ref(false)

// ------------------------------------------------------------------ tags

type State = 'on' | 'off' | 'some'
interface Row { tag: string; have: number; state: State }
const rows = ref<Row[]>([])
const tagsLoading = ref(false)

function startRows(counts: Facet[]) {
  rows.value = counts.map((f) => ({ tag: f.value, have: f.count, state: start(f.count) }))
}
function start(have: number): State {
  return have >= props.count ? 'on' : have > 0 ? 'some' : 'off'
}
onMounted(async () => {
  if (!props.query) {
    const counts = new Map<string, number>()
    for (const a of props.assets) for (const x of a.tags) counts.set(x.tag, (counts.get(x.tag) ?? 0) + 1)
    startRows([...counts].map(([value, count]) => ({ value, count })).sort((a, b) => b.count - a.count || a.value.localeCompare(b.value)))
    return
  }
  tagsLoading.value = true
  try {
    startRows((await api.search({ ...props.query, limit: 1, offset: 0, facets: true })).tags)
  } catch {
    rows.value = []
  } finally {
    tagsLoading.value = false
  }
})
/** A box clicked shows the state it's in now, which the browser's own toggle doesn't know about. */
function show(e: Event, state: State) {
  const box = e.target as HTMLInputElement
  box.checked = state === 'on'
  box.indeterminate = state === 'some'
}
/** Ticked, unticked, and (for one only some had) back to as it was. */
function cycle(r: Row) {
  const mixed = r.have > 0 && r.have < props.count
  r.state = r.state === 'some' ? 'on' : r.state === 'on' ? 'off' : mixed ? 'some' : 'on'
}
function addTags(tags: string[]) {
  for (const tag of tags) {
    const r = rows.value.find((x) => x.tag === tag)
    if (r) r.state = 'on'
    else rows.value.unshift({ tag, have: 0, state: 'on' })
  }
}
/** Every tag off, to tick back only the ones to keep: the same tags on all of them. */
function untickAll() {
  for (const r of rows.value) r.state = 'off'
}
const changed = (r: Row) => r.state !== start(r.have)
const addTagsOut = computed(() => rows.value.filter((r) => r.state === 'on' && r.have < props.count).map((r) => r.tag))
const removeTagsOut = computed(() => rows.value.filter((r) => r.state === 'off' && r.have > 0).map((r) => r.tag))
function parts(tag: string) {
  const i = tag.indexOf(':')
  return i > 0 ? { ns: tag.slice(0, i + 1), value: tag.slice(i + 1) } : { ns: '', value: tag }
}

// ------------------------------------------------------------------ fields

type TextKey = 'category' | 'license' | 'source_url'
const textKeys: TextKey[] = ['category', 'license', 'source_url']
const labels: Record<TextKey, () => string> = {
  category: () => t('asset-field-category'),
  license: () => t('asset-field-license'),
  source_url: () => t('asset-field-source'),
}
/** The value they all share, or null when they differ (or, for a search, aren't known). */
function shared<T>(get: (a: Asset) => T): T | null {
  if (props.query || !props.assets.length) return null
  const first = get(props.assets[0])
  return props.assets.every((a) => get(a) === first) ? first : null
}
const was = {
  category: shared((a) => a.category ?? ''),
  license: shared((a) => a.license ?? ''),
  source_url: shared((a) => a.source_url ?? ''),
  rating: shared((a) => a.rating ?? 0),
  reviewed: shared((a) => !!a.reviewed_at),
}
const text = reactive<Record<TextKey, string>>({ category: was.category ?? '', license: was.license ?? '', source_url: was.source_url ?? '' })
const touched = reactive<Record<TextKey | 'rating' | 'reviewed', boolean>>({ category: false, license: false, source_url: false, rating: false, reviewed: false })
const rating = ref(was.rating ?? 0)
const hoverRating = ref(0)
const reviewed = ref(was.reviewed ?? false)

function placeholder(k: TextKey): string {
  if (touched[k]) return was[k] === '' ? '' : t('bulk-edit-cleared')
  return was[k] === null ? (props.query ? t('bulk-edit-keep') : t('bulk-edit-mixed')) : ''
}
function reset(k: TextKey | 'rating' | 'reviewed') {
  touched[k] = false
  if (k === 'rating') rating.value = was.rating ?? 0
  else if (k === 'reviewed') reviewed.value = was.reviewed ?? false
  else text[k] = was[k] ?? ''
}
function rate(n: number) {
  rating.value = touched.rating && rating.value === n ? 0 : n
  touched.rating = rating.value !== was.rating
}
function toggleReviewed() {
  reviewed.value = touched.reviewed ? !reviewed.value : !(was.reviewed ?? false)
  touched.reviewed = reviewed.value !== was.reviewed
}
const categories = ref<Facet[]>([])
let categoriesAsked = false
function askCategories() {
  if (categoriesAsked) return
  categoriesAsked = true
  api.categories().then((c) => (categories.value = c), () => (categoriesAsked = false))
}

// ------------------------------------------------------------------ saving

const edit = computed<Edit>(() => {
  const e: Edit = {}
  if (addTagsOut.value.length) e.add_tags = addTagsOut.value
  if (removeTagsOut.value.length) e.remove_tags = removeTagsOut.value
  for (const k of textKeys) if (touched[k] && text[k].trim() !== (was[k] ?? null)) e[k] = text[k].trim()
  if (touched.rating) e.rating = rating.value
  if (touched.reviewed) e.reviewed = reviewed.value
  return e
})
const changes = computed(() => {
  const e = edit.value
  return (e.add_tags?.length ?? 0) + (e.remove_tags?.length ?? 0) + Object.keys(e).filter((k) => !k.endsWith('_tags')).length
})

/** What puts each asset back as it was, those needing the same grouped together. */
function undoFor(e: Edit): UndoGroup[] {
  const groups = new Map<string, UndoGroup>()
  for (const a of props.assets) {
    const back: AssetPatch = {}
    const had = new Map(a.tags.map((x) => [x.tag, x.source]))
    const added = (e.add_tags ?? []).filter((tag) => !had.has(tag))
    if (added.length) back.remove_tags = added
    const removed = (e.remove_tags ?? []).filter((tag) => had.has(tag))
    // A rule's or the AI's guess taken off is remembered as unwanted: putting it back forgets that.
    const guesses = removed.filter((tag) => had.get(tag) === 'rule' || had.get(tag) === 'ai')
    const own = removed.filter((tag) => !guesses.includes(tag))
    if (own.length) back.add_tags = own
    if (guesses.length) back.unreject_tags = guesses
    for (const k of textKeys) if (e[k] !== undefined && e[k] !== (a[k] ?? '')) back[k] = a[k] ?? ''
    if (e.rating !== undefined && e.rating !== (a.rating ?? 0)) back.rating = a.rating ?? 0
    if (e.reviewed !== undefined && e.reviewed !== !!a.reviewed_at) back.reviewed = !!a.reviewed_at
    if (!Object.keys(back).length) continue
    const key = JSON.stringify(back)
    const g = groups.get(key)
    if (g) g.ids.push(a.id)
    else groups.set(key, { ids: [a.id], edit: back })
  }
  return [...groups.values()]
}

async function save() {
  if (!changes.value || busy.value) return
  busy.value = true
  const e = edit.value
  try {
    if (await props.apply(e, props.query ? [] : undoFor(e))) dlg.value?.close()
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <dialog ref="dlg" class="bulkedit" aria-labelledby="bulk-title" @close="emit('close')" @click.self="dlg?.close()">
    <form method="dialog" @submit.prevent="save">
      <header>
        <h2 id="bulk-title">{{ query ? t('bulk-edit-title-all', { count }) : t('bulk-edit-title', { count }) }}</h2>
        <button type="button" class="btn ghost x" :aria-label="t('bulk-edit-close')" @click="dlg?.close()">
          <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
        </button>
      </header>

      <section class="tags">
        <div class="taghead">
          <h3>{{ t('bulk-edit-tags') }}</h3>
          <button v-if="rows.length && !query" type="button" class="linkbtn" :title="t('bulk-edit-untick-title')" @click="untickAll">{{ t('bulk-edit-untick') }}</button>
        </div>
        <label class="visually-hidden" for="bulk-new-tag">{{ t('bulk-edit-tag-add') }}</label>
        <TagInput id="bulk-new-tag" ref="tagBox" :have="rows.filter((r) => r.state === 'on').map((r) => r.tag)" :placeholder="t('bulk-edit-tag-add')" @add="addTags" />
        <p v-if="query && rows.length" class="note">{{ t('bulk-edit-tags-most-used') }}</p>
        <ul class="taglist" :aria-busy="tagsLoading">
          <li v-for="r in rows" :key="r.tag" :class="{ changed: changed(r) }">
            <label>
              <input
                type="checkbox"
                :checked="r.state === 'on'"
                :indeterminate="r.state === 'some'"
                :aria-checked="r.state === 'some' ? 'mixed' : r.state === 'on'"
                @change="cycle(r); show($event, r.state)"
              />
              <span class="name"><span class="ns">{{ parts(r.tag).ns }}</span>{{ parts(r.tag).value }}</span>
              <span class="have">{{ r.have ? t('bulk-edit-tag-count', { have: r.have, count }) : t('tag-suggest-new') }}</span>
            </label>
          </li>
        </ul>
      </section>

      <dl class="fields">
        <template v-for="k in textKeys" :key="k">
          <dt><label :for="`bulk-${k}`">{{ labels[k]() }}</label></dt>
          <dd :class="{ changed: touched[k] }">
            <input
              :id="`bulk-${k}`"
              v-model="text[k]"
              class="field"
              autocomplete="off"
              :type="k === 'source_url' ? 'url' : 'text'"
              :list="k === 'category' ? 'bulk-category-choices' : undefined"
              :placeholder="placeholder(k)"
              @focus="k === 'category' && askCategories()"
              @input="touched[k] = true"
            />
            <button v-if="touched[k]" type="button" class="btn ghost back" :title="t('bulk-edit-reset')" :aria-label="t('bulk-edit-reset')" @click="reset(k)">
              <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="M3.5 6.5h6a3 3 0 0 1 0 6H7M3.5 6.5 6 4M3.5 6.5 6 9" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
            </button>
          </dd>
        </template>
        <datalist id="bulk-category-choices">
          <option v-for="c in categories" :key="c.value" :value="c.value" />
        </datalist>

        <dt>{{ t('asset-field-rating') }}</dt>
        <dd :class="{ changed: touched.rating }">
          <span class="stars" role="group" :aria-label="t('asset-field-rating')" @mouseleave="hoverRating = 0">
            <button
              v-for="n in 5"
              :key="n"
              type="button"
              class="star"
              :class="{ on: n <= (hoverRating || rating) }"
              :aria-pressed="touched.rating ? n === rating : n === was.rating"
              :aria-label="t('asset-rate', { count: n })"
              @mouseenter="hoverRating = n"
              @click="rate(n)"
            >
              <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true"><path d="M8 1.9l1.8 3.7 4.1.6-3 2.9.7 4.1L8 11.3l-3.6 1.9.7-4.1-3-2.9 4.1-.6z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" /></svg>
            </button>
          </span>
          <span v-if="!touched.rating && was.rating === null" class="mixed-note">{{ query ? t('bulk-edit-keep') : t('bulk-edit-mixed') }}</span>
          <button v-if="touched.rating" type="button" class="btn ghost back" :title="t('bulk-edit-reset')" :aria-label="t('bulk-edit-reset')" @click="reset('rating')">
            <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="M3.5 6.5h6a3 3 0 0 1 0 6H7M3.5 6.5 6 4M3.5 6.5 6 9" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
          </button>
        </dd>

        <dt>{{ t('asset-reviewed') }}</dt>
        <dd :class="{ changed: touched.reviewed }">
          <label class="check">
            <input
              type="checkbox"
              :checked="touched.reviewed ? reviewed : was.reviewed === true"
              :indeterminate="!touched.reviewed && was.reviewed === null"
              :aria-checked="!touched.reviewed && was.reviewed === null ? 'mixed' : touched.reviewed ? reviewed : was.reviewed === true"
              @change="toggleReviewed(); show($event, touched.reviewed ? (reviewed ? 'on' : 'off') : was.reviewed === null ? 'some' : was.reviewed ? 'on' : 'off')"
            />
            <span v-if="!touched.reviewed && was.reviewed === null" class="mixed-note">{{ query ? t('bulk-edit-keep') : t('bulk-edit-mixed') }}</span>
          </label>
        </dd>
      </dl>

      <p v-if="query" class="note">{{ t('bulk-edit-all-note') }}</p>
      <footer>
        <button type="button" class="btn ghost" @click="dlg?.close()">{{ t('app-cancel') }}</button>
        <button type="submit" class="btn primary" :disabled="!changes || busy" :aria-busy="busy">
          {{ changes ? t('bulk-edit-save', { count: changes }) : t('bulk-edit-nothing') }}
        </button>
      </footer>
    </form>
  </dialog>
</template>

<style scoped>
.bulkedit {
  width: min(34rem, calc(100vw - 2rem)); max-height: calc(100vh - 2rem); padding: 1.1rem 1.3rem 1.1rem; border: 1px solid var(--line);
  border-radius: 10px; background: var(--panel); color: var(--text); box-shadow: var(--elev-3);
}
.bulkedit[open] { animation: bulk-in var(--dur-3) var(--ease-pop); }
.bulkedit::backdrop { background: var(--scrim); animation: fade var(--dur-2) var(--ease-out); }
@keyframes bulk-in { from { opacity: 0; scale: 0.96; } }
@keyframes fade { from { opacity: 0; } }
header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.8rem; }
h2 { margin: 0; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.x { padding: 0.35em 0.45em; }
h3 { margin: 0; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
.taghead { display: flex; align-items: baseline; justify-content: space-between; margin-bottom: 0.45rem; }
.taghead .linkbtn { font-size: var(--step--1); }
.tags { display: flex; flex-direction: column; }
.taglist {
  margin: 0.5rem 0 0; padding: 0.2rem; list-style: none; max-height: 15rem; overflow-y: auto;
  border: 1px solid var(--line); border-radius: var(--r-chip); font-size: var(--step--1);
}
.taglist:empty { display: none; }
.taglist li label {
  display: flex; align-items: center; gap: 0.55rem; padding: 0.28rem 0.45rem; border-radius: calc(var(--r-chip) - 2px); cursor: pointer;
}
.taglist li label:hover { background: var(--raised); }
.name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.ns { color: var(--faint); }
.have { margin-left: auto; color: var(--faint); font-size: 0.9em; white-space: nowrap; font-variation-settings: 'MONO' 1; }
/* What this will change wears the accent, so it's plain what Save does. */
.changed .name, dd.changed .field { color: var(--accent); }
dd.changed .field { border-color: var(--accent); }
.fields { display: grid; grid-template-columns: max-content 1fr; gap: 0.5rem 0.9rem; margin: 1.1rem 0 0; align-items: center; font-size: var(--step--1); }
dt { color: var(--muted); }
dd { display: flex; align-items: center; gap: 0.35rem; margin: 0; min-height: 2rem; }
dd .field { flex: 1; min-width: 0; padding: 0.35em 0.6em; }
.back { padding: 0.3em 0.4em; color: var(--muted); }
.stars { display: inline-flex; margin-left: -0.3rem; }
.star { border: 0; background: none; padding: 0.2rem 0.15rem; color: var(--faint); cursor: pointer; line-height: 0; }
.star svg { fill: none; transition: fill var(--dur-1), color var(--dur-1); }
.star.on { color: var(--star); }
.star.on svg { fill: currentColor; }
.mixed-note { color: var(--faint); font-style: italic; }
.check { display: inline-flex; align-items: center; gap: 0.5rem; cursor: pointer; }
.note { margin: 0.5rem 0 0; color: var(--faint); font-size: var(--step--1); }
footer { display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1.2rem; }
</style>
