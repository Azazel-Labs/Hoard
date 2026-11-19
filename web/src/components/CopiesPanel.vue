<script setup lang="ts">
// Settings → Near copies: every set of files in the library that are the same picture, sound or model at another size,
// rate or format, the most space taken by the ones that aren't the best first. It only lists them; nothing is deleted.
// The server works the sets out in the background and answers with the last ones at once; when the library has changed
// since, it says it's working them out again, and the list here is swapped for the new ones when they're ready.
import { onMounted, onUnmounted, ref } from 'vue'
import { api, ApiError, type CopyGroup } from '../api'
import { humanBytes } from '../format'
import { copyFacts } from '../copies'
import { formatNumber, t } from '../i18n'
import { showError } from '../ui'

const emit = defineEmits<{ open: [string]; unauthorized: [] }>()

const PAGE = 50
/** How often to ask whether the sets being worked out again are ready. */
const POLL_MS = 3000
const groups = ref<CopyGroup[] | null>(null)
const total = ref(0)
const wasted = ref(0)
const loading = ref(false)
const refreshing = ref(false)
let generation = 0
let poll: ReturnType<typeof setTimeout> | undefined
let gone = false

function failed(e: unknown) {
  if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  else showError(t('copies-load-failed', { error: (e as Error).message }))
  groups.value ??= []
}

/** Takes in a page, starting over when it's from a newer working-out than the ones shown. */
async function take(offset: number) {
  let page = await api.duplicates(offset, PAGE)
  if (offset > 0 && page.generation !== generation) page = await api.duplicates(0, offset + PAGE)
  const fresh = offset === 0 || page.generation !== generation
  groups.value = fresh ? page.groups : [...(groups.value ?? []), ...page.groups]
  total.value = page.total
  wasted.value = page.wasted
  generation = page.generation
  refreshing.value = page.refreshing
  clearTimeout(poll)
  if (page.refreshing && !gone) poll = setTimeout(check, POLL_MS)
}

async function more() {
  if (loading.value) return
  loading.value = true
  try {
    await take(groups.value?.length ?? 0)
  } catch (e) {
    failed(e)
  } finally {
    loading.value = false
  }
}

/** While the sets are worked out again: once they're ready, the list shown so far is swapped for the new ones. */
async function check() {
  if (loading.value) {
    poll = setTimeout(check, POLL_MS)
    return
  }
  try {
    const page = await api.duplicates(0, 1)
    if (gone) return
    if (page.generation !== generation) {
      loading.value = true
      try {
        const shown = Math.max(groups.value?.length ?? 0, PAGE)
        const all = await api.duplicates(0, shown)
        groups.value = all.groups
        total.value = all.total
        wasted.value = all.wasted
        generation = all.generation
        refreshing.value = all.refreshing
      } finally {
        loading.value = false
      }
    } else refreshing.value = page.refreshing
    if (refreshing.value && !gone) poll = setTimeout(check, POLL_MS)
  } catch (e) {
    refreshing.value = false
    failed(e)
  }
}

onMounted(more)
onUnmounted(() => {
  gone = true
  clearTimeout(poll)
})
</script>

<template>
  <div class="copies" :aria-busy="!groups">
    <p v-if="!groups" class="loading"><span class="spinner" aria-hidden="true" />{{ t('copies-loading') }}</p>
    <template v-else-if="!groups.length">
      <p v-if="refreshing" class="loading" role="status" data-testid="copies-refreshing">
        <span class="spinner" aria-hidden="true" />{{ t('copies-refreshing') }}
      </p>
      <p v-else class="none" data-testid="copies-none">{{ t('copies-none') }}</p>
    </template>
    <template v-else>
      <p class="sum" data-testid="copies-sum">{{ t('copies-sum', { count: total, size: humanBytes(wasted) }) }}</p>
      <p v-if="refreshing" class="loading" role="status" data-testid="copies-refreshing">
        <span class="spinner" aria-hidden="true" />{{ t('copies-refreshing') }}
      </p>
      <section v-for="(g, i) in groups" :key="g.copies[0].id" class="group" data-testid="copies-group">
        <header>
          <h3>{{ t('copies-set', { n: formatNumber(i + 1) }) }}</h3>
          <small>{{ t('copies-set-wasted', { size: humanBytes(g.wasted) }) }}</small>
        </header>
        <ul>
          <li v-for="c in g.copies" :key="c.id">
            <button type="button" class="link title" :title="c.path" @click="emit('open', c.id)">{{ c.name }}</button>
            <span v-if="c.best" class="best">{{ t('asset-near-best') }}</span>
            <small class="meta">{{ c.pack_name }} · {{ copyFacts(c) }}</small>
          </li>
        </ul>
      </section>
      <p v-if="groups.length < total" class="more">
        <button type="button" class="btn" :disabled="loading" @click="more">
          {{ t('copies-more', { count: total - groups.length }) }}
        </button>
      </p>
    </template>
  </div>
</template>

<style scoped>
.copies[aria-busy='true'] { cursor: progress; }
.loading, .none, .sum { color: var(--muted); font-size: var(--step--1); margin: 0; }
.copies { display: grid; gap: 1rem; }
.loading { display: flex; gap: 0.5rem; align-items: center; }
.spinner {
  flex: none; width: 14px; height: 14px; border-radius: 50%;
  border: 2px solid var(--line); border-top-color: var(--accent); animation: spin 0.7s linear infinite;
}
@keyframes spin { to { rotate: 1turn; } }
@media (prefers-reduced-motion: reduce) { .spinner { animation-duration: 2.4s; } }
header { display: flex; gap: 0.6rem; align-items: baseline; flex-wrap: wrap; margin-bottom: 0.5rem; }
header h3 { margin: 0; }
header small { color: var(--muted); font-size: var(--step--1); }
ul { display: grid; gap: 0.4rem; margin: 0; padding: 0; list-style: none; }
li {
  display: flex; gap: 0.3rem 0.6rem; align-items: baseline; flex-wrap: wrap;
  padding: 0.55rem 0.85rem; border: 1px solid var(--line); border-radius: 8px; font-size: var(--step--1);
}
.title { font-weight: 500; overflow-wrap: anywhere; text-align: left; }
.meta { color: var(--muted); font-size: inherit; flex-basis: 100%; overflow-wrap: anywhere; }
.best {
  font-size: 0.85em; font-weight: 500; padding: 0 0.45rem; border-radius: 999px;
  background: color-mix(in srgb, var(--ok) 16%, transparent); color: var(--ok);
}
.link {
  padding: 0; border: 0; background: none; color: inherit; font: inherit; cursor: pointer;
  text-decoration: underline; text-decoration-color: color-mix(in srgb, currentColor 35%, transparent); text-underline-offset: 2px;
}
.link:hover { text-decoration-color: currentColor; }
.more { margin: 0; }
</style>
