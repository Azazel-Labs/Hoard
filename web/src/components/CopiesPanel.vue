<script setup lang="ts">
// Settings → Near copies: every set of files in the library that are the same picture, sound or model at another size,
// rate or format, the most space taken by the ones that aren't the best first. It only lists them; nothing is deleted.
import { onMounted, ref } from 'vue'
import { api, ApiError, type CopyGroup } from '../api'
import { humanBytes } from '../format'
import { copyFacts } from '../copies'
import { formatNumber, t } from '../i18n'
import { showError } from '../ui'

const emit = defineEmits<{ open: [string]; unauthorized: [] }>()

const PAGE = 50
const groups = ref<CopyGroup[] | null>(null)
const total = ref(0)
const wasted = ref(0)
const loading = ref(false)

async function more() {
  if (loading.value) return
  loading.value = true
  try {
    const page = await api.duplicates(groups.value?.length ?? 0, PAGE)
    groups.value = [...(groups.value ?? []), ...page.groups]
    total.value = page.total
    wasted.value = page.wasted
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    else showError(t('copies-load-failed', { error: (e as Error).message }))
    groups.value ??= []
  } finally {
    loading.value = false
  }
}
onMounted(more)
</script>

<template>
  <div class="copies" :aria-busy="!groups">
    <p v-if="!groups" class="loading">{{ t('copies-loading') }}</p>
    <p v-else-if="!groups.length" class="none" data-testid="copies-none">{{ t('copies-none') }}</p>
    <template v-else>
      <p class="sum" data-testid="copies-sum">{{ t('copies-sum', { count: total, size: humanBytes(wasted) }) }}</p>
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
