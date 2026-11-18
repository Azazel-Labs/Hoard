<script setup lang="ts">
import { computed } from 'vue'
import { thumbUrl, type Asset, type AssetKind } from '../api'
import { humanBytes } from '../format'
import { KIND_LABEL } from '../kinds'
import KindGlyph from './KindGlyph.vue'
import { formatNumber, t } from '../i18n'

// What's selected, in the panel's place: how many, how big, and ways to download them all.
const props = defineProps<{
  /** The selected assets Hoard has seen, in the order they were picked. */
  assets: Asset[]
  /** How many are selected (a few may not have been seen). */
  count: number
  /** The zip of the files as they are. */
  zipUrl: string
  /** The zip of the models as GLBs. */
  glbUrl: string
  /** How many have a recovered WAV/PNG that can go in instead of the .uasset. */
  withSource: number
}>()
const preferSources = defineModel<boolean>('preferSources', { required: true })
const emit = defineEmits<{ remove: [string]; clear: []; credits: [] }>()

/** The list stops here; the rest are counted. */
const SHOWN = 200

const size = computed(() => props.assets.reduce((n, a) => n + a.size, 0))
const kinds = computed(() => {
  const by = new Map<AssetKind, number>()
  for (const a of props.assets) by.set(a.kind, (by.get(a.kind) ?? 0) + 1)
  return [...by].sort((a, b) => b[1] - a[1])
})
const models = computed(() => props.assets.filter((a) => a.kind === 'model').length)
const shown = computed(() => props.assets.slice(-SHOWN).reverse())
</script>

<template>
  <aside class="bag" :aria-label="t('app-selected-label')">
    <header class="top">
      <h2>{{ t('app-asset-count', { count }) }}</h2>
      <button class="btn ghost icon" type="button" :aria-label="t('bag-clear')" :title="t('bag-clear')" @click="emit('clear')">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>
    <p class="sum">
      <span class="mono">{{ humanBytes(size) }}</span>
      <span v-for="[k, n] in kinds" :key="k" class="kind" :style="{ '--kind': `var(--k-${k})` }">
        <span class="mono">{{ formatNumber(n) }}</span> {{ KIND_LABEL[k] ?? k }}
      </span>
    </p>

    <section v-if="models" class="way">
      <h3>{{ t('bag-glb-heading') }}</h3>
      <p>{{ t('bag-glb-text') }}</p>
      <a class="btn primary" :href="glbUrl" download>{{ t('bag-glb-download') }}</a>
    </section>
    <section class="way">
      <h3>{{ t('bag-plain-heading') }}</h3>
      <p>{{ t('bag-plain-text') }}</p>
      <label v-if="withSource" class="src" :title="t('app-prefer-sources-title')">
        <input v-model="preferSources" type="checkbox" /> {{ t('app-prefer-sources', { count: withSource }) }}
      </label>
      <div class="row">
        <a class="btn" :class="{ primary: !models }" :href="zipUrl" download>{{ t('app-download-zip') }}</a>
        <button type="button" class="btn ghost" :title="t('app-credits-title')" @click="emit('credits')">{{ t('app-credits') }}</button>
      </div>
    </section>

    <ul class="items">
      <li v-for="a in shown" :key="a.id" :style="{ '--kind': `var(--k-${a.kind})` }">
        <span class="thumb">
          <img v-if="a.has_thumb" :src="thumbUrl(a)" alt="" loading="lazy" />
          <KindGlyph v-else :kind="a.kind" :size="20" />
        </span>
        <span class="name" :title="a.name">{{ a.name }}</span>
        <span class="meta mono">{{ humanBytes(a.size) }}</span>
        <button type="button" class="btn ghost icon" :aria-label="t('bag-remove', { name: a.name })" @click="emit('remove', a.id)">
          <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
        </button>
      </li>
    </ul>
    <p v-if="assets.length > SHOWN" class="more">{{ t('bag-more', { count: assets.length - SHOWN }) }}</p>
  </aside>
</template>

<style scoped>
.bag {
  display: flex; flex-direction: column; gap: 1rem;
  padding: 1.1rem 1.2rem 1.4rem;
  background: var(--panel); border-left: 1px solid var(--line);
  overflow-y: auto; height: 100%;
}
.bag > * { flex-shrink: 0; }
.top { display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; }
h2 { margin: 0; font-size: 1.3rem; font-weight: 650; letter-spacing: -0.01em; }
.top .icon { padding: 0.35em 0.45em; margin-right: -0.45rem; }
.sum { margin: -0.5rem 0 0; display: flex; flex-wrap: wrap; gap: 0.35rem 0.8rem; color: var(--muted); font-size: var(--step--1); }
.kind { display: inline-flex; align-items: baseline; gap: 0.3em; }
.kind::before { content: ''; width: 0.5em; height: 0.5em; border-radius: 2px; background: var(--kind); align-self: center; }
.way {
  display: flex; flex-direction: column; align-items: flex-start; gap: 0.5rem;
  padding: 0.85rem 0.95rem; border: 1px solid var(--line); border-radius: var(--r-tile);
}
.way h3 { margin: 0; font-size: 0.95rem; font-weight: 650; }
.way p { margin: 0; color: var(--muted); font-size: var(--step--1); line-height: 1.45; }
.way .btn { margin-top: 0.2rem; }
.row { display: flex; gap: 0.4rem; flex-wrap: wrap; }
.src { display: flex; gap: 0.4em; align-items: center; font-size: var(--step--1); color: var(--muted); }
.items { list-style: none; margin: 0; padding: 0; display: grid; gap: 0.15rem; }
.items li { display: flex; align-items: center; gap: 0.6rem; min-width: 0; padding: 0.2rem 0; }
.thumb {
  flex: none; width: 2.2rem; height: 2.2rem; display: grid; place-items: center; overflow: hidden;
  border-radius: 5px; border: 1px solid var(--line); box-shadow: inset 0 2px 0 var(--kind); color: var(--kind);
}
.thumb img { width: 100%; height: 100%; object-fit: contain; }
.name { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 0.88rem; }
.meta { flex: none; color: var(--faint); font-size: 0.72rem; }
.items .icon { padding: 0.35em 0.45em; color: var(--faint); }
.items .icon:hover { color: var(--text); }
.more { margin: 0; color: var(--faint); font-size: var(--step--1); }
</style>
