<script setup lang="ts">
import { computed } from 'vue'
import { packCreditsUrl, packZipUrl, thumbUrl, type AssetKind, type Pack } from '../api'
import { humanBytes } from '../format'
import { KIND_LABEL } from '../kinds'
import { formatDate, formatNumber, t } from '../i18n'

// The pack being browsed, in the panel's place while no asset is open: its cover, what it is and where it's from,
// what's in it (each kind narrows the grid), and its downloads.
const props = defineProps<{
  pack: Pack
  /** The kind the grid is narrowed to, if any. */
  kind: string
  /** The tags the grid is narrowed to. */
  tags: string[]
}>()
const emit = defineEmits<{ close: []; kind: [AssetKind]; tag: [string]; packs: [] }>()

const kinds = computed(() =>
  (Object.entries(props.pack.kinds ?? {}) as [AssetKind, number][]).filter(([, n]) => n > 0).sort((a, b) => b[1] - a[1]),
)
const license = computed(() => {
  const p = props.pack
  const name = p.license ?? t('packs-license-not-found')
  return p.license_tier ? t('packs-license-tier', { license: name, tier: p.license_tier }) : t('packs-license', { license: name })
})
</script>

<template>
  <aside class="pack-panel" :aria-label="t('pack-panel-label')">
    <header class="top">
      <h2>{{ pack.name }}</h2>
      <button class="btn ghost icon" type="button" :aria-label="t('pack-panel-hide')" :title="t('pack-panel-hide')" @click="emit('close')">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <div v-if="pack.preview.length" class="cover checker">
      <img
        v-for="(id, i) in pack.preview"
        :key="id"
        :src="thumbUrl({ id })"
        :class="{ glyphs: pack.preview_kinds?.[i] === 'font' }"
        alt=""
      />
    </div>

    <p class="facts">
      <span>{{ t('packs-assets', { count: pack.asset_count }) }}</span>
      <span class="mono">{{ humanBytes(pack.total_bytes) }}</span>
      <span>{{ t('packs-added', { date: formatDate(pack.created_at) }) }}</span>
    </p>
    <p class="facts">
      <span>{{ license }}</span>
      <span v-if="pack.store">
        {{ t('packs-from-before') }} <a v-if="pack.store_url" :href="pack.store_url" target="_blank" rel="noopener noreferrer">{{ pack.store }}</a><template v-else>{{ pack.store }}</template>{{ t('packs-from-after') }}
      </span>
    </p>
    <p v-if="pack.error" class="error">{{ pack.error }}</p>
    <p v-if="pack.description" class="desc">{{ pack.description }}</p>

    <section v-if="kinds.length > 1" class="kinds">
      <h3>{{ t('pack-panel-kinds') }}</h3>
      <div class="chips">
        <button
          v-for="[k, n] in kinds"
          :key="k"
          type="button"
          class="chip kind"
          :style="{ '--kind': `var(--k-${k})` }"
          :aria-pressed="kind === k"
          @click="emit('kind', k)"
        >
          {{ KIND_LABEL[k] ?? k }} <span class="mono">{{ formatNumber(n) }}</span>
        </button>
      </div>
    </section>
    <div v-if="pack.tags.length" class="chips">
      <button
        v-for="tag in pack.tags"
        :key="tag"
        type="button"
        class="chip"
        :aria-pressed="tags.includes(tag)"
        @click="emit('tag', tag)"
      >{{ tag }}</button>
    </div>

    <div class="row">
      <a class="btn" :href="packZipUrl(pack.id)">{{ t('packs-download') }}</a>
      <a class="btn ghost" :href="packCreditsUrl(pack.id)" download="CREDITS.md" :title="t('packs-credits-title')">{{ t('packs-credits') }}</a>
    </div>
    <button type="button" class="linkbtn all" @click="emit('packs')">{{ t('pack-panel-all') }}</button>
  </aside>
</template>

<style scoped>
.pack-panel {
  display: flex; flex-direction: column; gap: 0.85rem;
  padding: 1.1rem 1.2rem 1.4rem;
  background: var(--panel); border-left: 1px solid var(--line);
  overflow-y: auto; height: 100%;
}
.pack-panel > * { flex-shrink: 0; }
.top { display: flex; align-items: flex-start; justify-content: space-between; gap: 0.75rem; }
h2 { margin: 0; font-size: 1.3rem; font-weight: 650; letter-spacing: -0.01em; overflow-wrap: anywhere; }
.top .icon { padding: 0.35em 0.45em; margin-right: -0.45rem; }
.cover {
  display: grid; grid-template-columns: 1fr 1fr; grid-template-rows: 1fr 1fr; gap: 2px;
  aspect-ratio: 16 / 10; overflow: hidden; border: 1px solid var(--line); border-radius: var(--r-tile);
}
.cover img { width: 100%; height: 100%; object-fit: cover; background: var(--check-a); min-height: 0; }
.cover img:only-child { grid-column: 1 / -1; grid-row: 1 / -1; object-fit: contain; }
.cover img:first-child:nth-last-child(2), .cover img:first-child:nth-last-child(2) ~ img { grid-row: 1 / -1; }
.cover img:first-child:nth-last-child(3) { grid-row: 1 / -1; }
.facts { margin: 0; display: flex; flex-wrap: wrap; gap: 0.3rem 0.9rem; color: var(--muted); font-size: var(--step--1); }
.desc { margin: 0; white-space: pre-line; line-height: 1.5; font-size: 0.9rem; overflow-wrap: anywhere; }
.error { margin: 0; color: var(--danger); font-size: var(--step--1); }
h3 { margin: 0 0 0.4rem; font-size: 0.8rem; font-weight: 600; color: var(--muted); text-transform: uppercase; letter-spacing: 0.04em; }
.chips { display: flex; flex-wrap: wrap; gap: 0.35rem; }
.chips .chip { cursor: pointer; color: var(--text); }
.chips .chip:hover { border-color: var(--muted); }
.chips .chip[aria-pressed='true'] { background: var(--raised); border-color: var(--text); }
.chip.kind::before { content: ''; width: 0.5em; height: 0.5em; border-radius: 2px; background: var(--kind); align-self: center; }
.chip .mono { color: var(--muted); }
.row { display: flex; gap: 0.4rem; flex-wrap: wrap; }
.all { align-self: flex-start; padding-inline: 0; }
</style>
