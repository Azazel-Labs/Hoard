<script setup lang="ts">
import { onBeforeUnmount, onMounted, reactive, ref } from 'vue'
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

/** Fill the form from a match; Save fetches the page's description. */
function useMatch(m: StoreMatch) {
  draft.store = m.store
  draft.store_url = m.url
  draft.store_text = null
  if (m.author) draft.seller = m.author
  draft.store_license = m.license ?? null
  lookup.open = false
}

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
    const { readme: _readme, store_auto: _auto, ...ctx } = draft
    Object.assign(draft, await api.setPackContext(props.pack.id, ctx))
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
            <button class="btn" type="button" @click="useMatch(m)">{{ t('packs-lookup-use') }}</button>
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
.find-again { align-self: flex-start; }
.again { padding: 0; border: 0; background: none; color: var(--accent); font: inherit; cursor: pointer; text-decoration: underline; }
.again:disabled { color: var(--muted); cursor: default; }
.auto {
  display: flex; flex-wrap: wrap; align-items: center; gap: 0.3rem 0.7rem; margin: 0; padding: 0.5rem 0.7rem;
  border: 1px solid var(--line); border-radius: var(--r-tile); font-size: var(--step--1);
}
.auto span::before { content: '✓ '; color: var(--ok, var(--accent)); }
</style>
