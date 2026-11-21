<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import { api, type Pack, type PackContext, type StoreMatch, type StoreMatches } from '../api'
import { formatDate, t } from '../i18n'

// A pack's store details: which store page it came from (found by searching the stores, or typed in), the seller,
// the license, and the page's text, which the tagger reads.
const props = defineProps<{
  pack: Pack
  /** Whether an AI tagger is set up, so saving can tag the pack again with what's written here. */
  llm: boolean
}>()
const emit = defineEmits<{ close: []; changed: []; error: [string] }>()

const saving = ref(false)
const draft = reactive<PackContext>({
  store: null,
  store_url: null,
  store_text: null,
  license_tier: null,
  readme: null,
  seller: null,
  store_license: null,
  store_auto: null,
})

// Store lookup: the pack's name searched in the stores, best matches first.
const lookup = reactive({ q: '', loading: false, open: false, error: '', result: null as StoreMatches | null })
let lookupAsked = 0
let closed = false
onBeforeUnmount(() => (closed = true))

onMounted(async () => {
  try {
    Object.assign(draft, await api.packContext(props.pack.id))
    Object.assign(lookup, { q: '', result: null, error: '', open: !draft.store_url })
    // No store page yet: look for one straight away.
    if (!draft.store_url) find()
  } catch (e) {
    emit('error', t('packs-context-load-failed', { error: (e as Error).message }))
    emit('close')
  }
})

async function find(q?: string, fresh = false) {
  const n = ++lookupAsked
  lookup.loading = true
  lookup.open = true
  lookup.error = ''
  try {
    const r = await api.storeMatches(props.pack.id, q?.trim() || undefined, fresh)
    if (n !== lookupAsked || closed) return
    lookup.result = r
    if (!lookup.q) lookup.q = r.query
  } catch (e) {
    if (n === lookupAsked) lookup.error = (e as Error).message
  } finally {
    if (n === lookupAsked) lookup.loading = false
  }
}

// A match is looked over field by field before it fills the form: what's there now beside what the store says,
// each to take or leave. Those that differ start ticked; the description is read from the page on Save.
type Key = 'store' | 'store_url' | 'seller' | 'store_license' | 'store_text'
interface Row { key: Key; label: string; now: string; store: string; same: boolean }
const review = ref<{ m: StoreMatch; take: Record<Key, boolean> } | null>(null)
const rows = computed<Row[]>(() => {
  const r = review.value
  if (!r) return []
  const text = draft.store_text?.trim() ?? ''
  const all: Row[] = [
    { key: 'store', label: t('packs-store'), now: draft.store ?? '', store: r.m.store, same: false },
    { key: 'store_url', label: t('packs-store-page'), now: draft.store_url ?? '', store: r.m.url, same: false },
    { key: 'seller', label: t('packs-seller'), now: draft.seller ?? '', store: r.m.author ?? '', same: false },
    { key: 'store_license', label: t('packs-license-label'), now: draft.store_license ?? '', store: r.m.license ?? '', same: false },
    {
      key: 'store_text',
      label: t('packs-store-text'),
      now: text.length > 90 ? text.slice(0, 88).trimEnd() + '…' : text,
      store: r.m.summary || t('packs-review-read-page'),
      same: false,
    },
  ]
  return all
    .filter((x) => x.store)
    .map((x) => ({ ...x, same: x.key !== 'store_text' && x.now.trim().toLowerCase() === x.store.trim().toLowerCase() }))
})
function startReview(m: StoreMatch) {
  review.value = { m, take: { store: true, store_url: true, seller: true, store_license: true, store_text: true } }
  const take = review.value.take
  // What's already the same, or missing from the store, has nothing to take.
  for (const k of Object.keys(take) as Key[]) take[k] = rows.value.some((x) => x.key === k && !x.same)
  // A description of some other page has to go with it; one of the same page can stay.
  take.store_text = !draft.store_text || (draft.store_url ?? '') !== m.url
}
const taking = computed(() => rows.value.filter((x) => !x.same && review.value?.take[x.key]).length)
/** Fill the form with the fields ticked; Save fetches the page's description. */
function applyReview() {
  const r = review.value
  if (!r) return
  if (r.take.store) draft.store = r.m.store
  if (r.take.store_url) draft.store_url = r.m.url
  if (r.take.seller && r.m.author) draft.seller = r.m.author
  if (r.take.store_license && r.m.license) draft.store_license = r.m.license
  if (r.take.store_text) draft.store_text = null
  review.value = null
  lookup.open = false
}

// The pack's license is every asset's, unless one has its own: Save can make those go by the pack's too.
const allToPack = ref(false)

/** The store page found automatically isn't this pack's: take it off and show the other suggestions. */
async function notThisOne() {
  try {
    Object.assign(draft, await api.undoStorePage(props.pack.id))
    emit('changed')
    find()
  } catch (e) {
    emit('error', t('packs-undo-store-failed', { error: (e as Error).message }))
  }
}

function day(at: string) {
  return formatDate(at, { day: 'numeric', month: 'short', year: 'numeric' })
}

async function save(thenRetag: boolean) {
  saving.value = true
  try {
    const { readme: _readme, store_auto: _auto, own_licenses: _own, ...ctx } = draft
    Object.assign(draft, await api.setPackContext(props.pack.id, ctx))
    if (allToPack.value && draft.own_licenses) {
      // An asset's own license cleared goes by the pack's (each change is in its history, to revert).
      await api.bulkAssets({ query: { pack: props.pack.id }, license: '' })
    }
    if (thenRetag) await api.retagPack(props.pack.id, { rules: false, llm: true })
    emit('changed')
    emit('close')
  } catch (e) {
    emit('error', t('packs-context-save-failed', { error: (e as Error).message }))
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <form class="context" @submit.prevent="save(false)">
    <p class="hint">
      {{ t('packs-context-hint-before') }} <span class="mono">spell35.wav</span>{{ t('packs-context-hint-after') }}
    </p>
    <p v-if="draft.store_auto" class="auto">
      <span>{{ t('packs-store-auto') }}</span>
      <button class="btn ghost" type="button" @click="notThisOne()">{{ t('packs-store-not-this') }}</button>
    </p>
    <section v-if="lookup.open" class="lookup" :aria-busy="lookup.loading">
      <div class="find">
        <input
          v-model="lookup.q"
          class="field"
          type="search"
          :aria-label="t('packs-lookup-label')"
          :placeholder="t('packs-lookup-placeholder')"
          @keydown.enter.prevent="find(lookup.q)"
        />
        <button class="btn" type="button" :disabled="lookup.loading" @click="find(lookup.q)">
          {{ lookup.loading ? t('packs-lookup-searching') : t('packs-lookup-search') }}
        </button>
        <button class="btn ghost" type="button" @click="lookup.open = false">{{ t('packs-lookup-hide') }}</button>
      </div>
      <p v-if="lookup.error" class="err">{{ lookup.error }}</p>
      <template v-else-if="lookup.result">
        <p v-if="lookup.result.clues.length || lookup.result.also.length" class="hint">
          <template v-if="lookup.result.clues.length">{{ t('packs-lookup-clues', { clues: lookup.result.clues.join(' · ') }) }} </template>
          <template v-if="lookup.result.also.length">{{ t('packs-lookup-also', { queries: lookup.result.also.map((q) => t('packs-lookup-quoted', { query: q })).join(', ') }) }}</template>
        </p>
        <p v-if="!lookup.result.matches.length" class="hint">
          {{ t('packs-lookup-nothing') }}
        </p>
        <ul v-else class="matches">
          <li v-for="m in lookup.result.matches" :key="m.url">
            <img v-if="m.thumb" :src="m.thumb" alt="" loading="lazy" referrerpolicy="no-referrer" />
            <span v-else class="noimg" aria-hidden="true" />
            <div class="what">
              <a :href="m.url" target="_blank" rel="noopener noreferrer" class="title">{{ m.title }}</a>
              <span class="meta">{{ [m.store, m.author, m.price].filter(Boolean).join(' · ') }}</span>
              <span v-if="m.summary" class="meta clamp">{{ m.summary }}</span>
              <span v-for="w in m.why ?? []" :key="w" class="why">{{ w }}</span>
            </div>
            <button v-if="review?.m.url !== m.url" class="btn" type="button" @click="startReview(m)">{{ t('packs-lookup-use') }}</button>
            <div v-else class="review">
              <table>
                <thead>
                  <tr><th /><th>{{ t('packs-review-now') }}</th><th>{{ t('packs-review-store') }}</th></tr>
                </thead>
                <tbody>
                  <tr v-for="x in rows" :key="x.key" :class="{ same: x.same, off: !x.same && !review.take[x.key] }">
                    <th scope="row">
                      <label>
                        <input v-model="review.take[x.key]" type="checkbox" :disabled="x.same" />
                        {{ x.label }}
                      </label>
                    </th>
                    <td :class="{ none: !x.now }">{{ x.now || '—' }}</td>
                    <td>{{ x.same ? t('packs-review-same') : x.store }}</td>
                  </tr>
                </tbody>
              </table>
              <div class="actions">
                <button class="btn primary" type="button" :disabled="!taking" @click="applyReview()">{{ t('packs-review-take', { count: taking }) }}</button>
                <button class="btn ghost" type="button" @click="review = null">{{ t('packs-cancel') }}</button>
              </div>
            </div>
          </li>
        </ul>
        <p class="hint">
          <template v-if="lookup.result.searched_at">
            {{ t('packs-lookup-searched', { date: day(lookup.result.searched_at) }) }}
            <button class="again" type="button" :disabled="lookup.loading" @click="find(undefined, true)">{{ t('packs-lookup-again') }}</button>{{ t('packs-lookup-again-after') }}
          </template>
          <template v-if="lookup.result.failed.length">{{ t('packs-lookup-unreachable', { stores: lookup.result.failed.join(', ') }) }} </template>
          {{ t('packs-lookup-also-look') }}
          <template v-for="(l, i) in lookup.result.links" :key="l.store">
            <a :href="l.url" target="_blank" rel="noopener noreferrer">{{ l.store }}</a>{{ i < lookup.result.links.length - 1 ? t('packs-lookup-links-and') : '' }}
          </template>
          {{ t('packs-lookup-browser-only') }}
        </p>
      </template>
    </section>
    <button v-else class="btn ghost find-again" type="button" @click="lookup.result ? (lookup.open = true) : find()">
      {{ t('packs-lookup-open') }}
    </button>
    <div class="row">
      <label>{{ t('packs-store') }} <input v-model="draft.store" class="field" :placeholder="t('packs-store-placeholder')" /></label>
      <label>{{ t('packs-seller') }} <input v-model="draft.seller" class="field" :placeholder="t('packs-seller-placeholder')" /></label>
    </div>
    <div class="row">
      <label>{{ t('packs-license-label') }} <input v-model="draft.store_license" class="field" :placeholder="t('packs-license-placeholder')" /></label>
      <label>{{ t('packs-license-tier-label') }} <input v-model="draft.license_tier" class="field" :placeholder="t('packs-license-tier-placeholder')" /></label>
    </div>
    <label v-if="draft.own_licenses" class="fanout">
      <input v-model="allToPack" type="checkbox" />
      {{ t('packs-own-licenses', { count: draft.own_licenses }) }}
    </label>
    <label>{{ t('packs-store-page') }}
      <input v-model="draft.store_url" class="field" type="url" placeholder="https://…" />
    </label>
    <label>{{ t('packs-store-text') }}
      <textarea v-model="draft.store_text" class="field" rows="5" :placeholder="t('packs-store-text-placeholder')" />
    </label>
    <p v-if="draft.store_url && !draft.store_text" class="hint">{{ t('packs-store-text-fetch') }}</p>
    <details v-if="draft.readme">
      <summary>{{ t('packs-readme') }}</summary>
      <pre>{{ draft.readme }}</pre>
    </details>
    <div class="actions">
      <button class="btn primary" type="submit" :disabled="saving">{{ t('packs-save') }}</button>
      <button v-if="llm" class="btn" type="button" :disabled="saving || pack.status !== 'ready'" @click="save(true)">
        {{ t('packs-save-retag') }}
      </button>
      <button class="btn ghost" type="button" @click="emit('close')">{{ t('packs-cancel') }}</button>
    </div>
  </form>
</template>

<style scoped>
.context {
  display: flex; flex-direction: column; gap: 0.7rem; margin-top: 0.3rem; padding: 0.9rem 1rem;
  border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--raised);
}
.context label { display: flex; flex-direction: column; gap: 0.3rem; font-size: var(--step--1); color: var(--muted); }
.hint { margin: 0; font-size: var(--step--1); color: var(--muted); max-width: 70ch; }
.actions { display: flex; flex-wrap: wrap; gap: 0.4rem; }
ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
li { padding: 1rem 0; border-top: 1px solid var(--line); display: flex; gap: 1rem; align-items: flex-start; }
.context .row { display: grid; grid-template-columns: 1fr 1fr; gap: 0.7rem; }
.context textarea { resize: vertical; font: inherit; }
.context pre { white-space: pre-wrap; font-size: 0.8rem; max-height: 14rem; overflow: auto; margin: 0.4rem 0 0; color: var(--muted); }
.context summary { cursor: pointer; font-size: var(--step--1); color: var(--muted); }
@media (max-width: 560px) { .context .row { grid-template-columns: 1fr; } }
.lookup { display: grid; gap: 0.5rem; padding: 0.7rem; border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--bg); }
.lookup[aria-busy='true'] .matches { opacity: 0.6; }
.find { display: flex; gap: 0.5rem; flex-wrap: wrap; }
.find .field { flex: 1 1 14rem; min-width: 0; }
.lookup .err { margin: 0; color: var(--danger); font-size: var(--step--1); }
.lookup .hint { margin: 0; }
.matches { gap: 0; max-height: 24rem; overflow-y: auto; }
.matches li { padding: 0.5rem 0; gap: 0.7rem; align-items: center; }
.matches li:first-child { border-top: 0; }
.matches img, .matches .noimg {
  flex: none; width: 64px; height: 40px; object-fit: cover; border-radius: var(--r-chip); background: var(--raised);
}
.matches .what { display: flex; flex-direction: column; gap: 0.1rem; flex: 1; min-width: 0; }
.matches .title { color: var(--text); font-size: var(--step--1); font-weight: 600; overflow-wrap: anywhere; }
.matches .meta { color: var(--muted); font-size: 0.75rem; overflow-wrap: anywhere; }
.matches .why { color: var(--ok, var(--accent)); font-size: 0.75rem; }
.matches .why::before { content: '✓ '; }
.matches .clamp { display: -webkit-box; -webkit-line-clamp: 1; line-clamp: 1; -webkit-box-orient: vertical; overflow: hidden; }
.matches li:has(.review) { flex-wrap: wrap; }
.review { flex-basis: 100%; display: grid; gap: 0.5rem; padding: 0.6rem 0.7rem; border-radius: var(--r-tile); background: var(--raised); }
.review table { width: 100%; border-collapse: collapse; font-size: var(--step--1); table-layout: fixed; }
.review th, .review td { padding: 0.25rem 0.5rem 0.25rem 0; text-align: start; vertical-align: top; overflow-wrap: anywhere; }
.review thead th { color: var(--muted); font-weight: 500; font-size: 0.75rem; }
.review thead th:first-child { width: 34%; }
.review tbody th { font-weight: 500; }
.review tbody th label { flex-direction: row; align-items: center; gap: 0.45rem; color: var(--text); font-size: inherit; cursor: pointer; }
.review td { color: var(--text); }
.review td.none, .review .same td, .review .same th label { color: var(--faint); }
.review .off td:last-child { color: var(--muted); text-decoration: line-through; text-decoration-color: var(--faint); }
.review tbody tr:not(.same):not(.off) td:last-child { color: var(--accent); }
.fanout { flex-direction: row !important; align-items: center; gap: 0.45rem !important; color: var(--text) !important; cursor: pointer; }
.find-again { align-self: flex-start; }
.again { padding: 0; border: 0; background: none; color: var(--accent); font: inherit; cursor: pointer; text-decoration: underline; }
.again:disabled { color: var(--muted); cursor: default; }
.auto {
  display: flex; flex-wrap: wrap; align-items: center; gap: 0.3rem 0.7rem; margin: 0; padding: 0.5rem 0.7rem;
  border: 1px solid var(--line); border-radius: var(--r-tile); font-size: var(--step--1);
}
.auto span::before { content: '✓ '; color: var(--ok, var(--accent)); }
</style>
