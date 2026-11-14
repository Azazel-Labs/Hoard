<script setup lang="ts">
// The library in numbers, on the Settings page: totals up top, then where the space goes (kinds, file types, packs,
// the biggest files), how it grew month by month, and how well it's tagged. Archived things aren't counted.
// Bars are one series each and every one carries its name and value as text, so colour is never the only label.
import { computed, onMounted, ref } from 'vue'
import { api, humanBytes, KIND_LABEL, type AssetKind, type LibraryStats, type Stats, type TagSource } from '../api'
import { formatDate, formatNumber, t } from '../i18n'
import KindGlyph from './KindGlyph.vue'

const props = defineProps<{ stats: Stats | null }>()
const emit = defineEmits<{ tag: [string]; pack: [{ id: string; name: string }]; open: [string]; archive: [] }>()

const lib = ref<LibraryStats | null>(null)
const failed = ref('')
onMounted(async () => {
  try {
    lib.value = await api.libraryStats()
  } catch (e) {
    failed.value = (e as Error).message
  }
})

const n = (v: number) => formatNumber(v)
const pct = (part: number, whole: number) => formatNumber(whole ? part / whole : 0, { style: 'percent', maximumFractionDigits: 0 })
/** Bar length as a share of the biggest in its list; a sliver stays visible. */
const share = (v: number, max: number) => `${max ? Math.max(1.5, (v / max) * 100) : 0}%`

const saved = computed(() => (props.stats ? Math.max(0, props.stats.logical_bytes - props.stats.stored_bytes) : 0))
const archived = computed(() => (props.stats?.archived_packs ?? 0) + (props.stats?.archived_assets ?? 0))

const maxOf = <T,>(rows: T[] | undefined, f: (r: T) => number) => Math.max(0, ...(rows ?? []).map(f))
const kindMax = computed(() => maxOf(lib.value?.kinds, (k) => k.bytes))
const extMax = computed(() => maxOf(lib.value?.extensions, (k) => k.bytes))
const packMax = computed(() => maxOf(lib.value?.packs, (k) => k.bytes))
const tagMax = computed(() => maxOf(lib.value?.top_tags, (k) => k.count))

/** The last twelve months, oldest first, with empty months filled in so the gaps show. */
const months = computed(() => {
  const have = new Map((lib.value?.months ?? []).map((m) => [m.month, m]))
  const out: { month: string; label: string; count: number; bytes: number }[] = []
  const d = new Date()
  d.setDate(1)
  for (let i = 0; i < 12; i++) {
    const key = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}`
    const m = have.get(key)
    out.unshift({ month: key, label: formatDate(d, { month: 'short' }), count: m?.count ?? 0, bytes: m?.bytes ?? 0 })
    d.setMonth(d.getMonth() - 1)
  }
  return out
})
const monthMax = computed(() => maxOf(months.value, (m) => m.count))
const monthName = (key: string) => formatDate(new Date(`${key}-01T00:00:00`), { month: 'long', year: 'numeric' })

/** Where tags came from, in words. A function, so it's in the language of the moment. */
const sourceLabel = (s: TagSource): string | undefined =>
  ({
    human: t('stats-source-human'),
    ai: t('stats-source-ai'),
    rule: t('stats-source-rule'),
    meta: t('stats-source-meta'),
  })[s]
const assets = computed(() => props.stats?.assets ?? 0)
</script>

<template>
  <div class="stats">
    <dl class="tiles">
      <div><dt>{{ t('stats-assets') }}</dt><dd>{{ n(stats?.assets ?? 0) }}</dd></div>
      <div><dt>{{ t('stats-packs') }}</dt><dd>{{ n(stats?.packs ?? 0) }}</dd></div>
      <div><dt>{{ t('stats-on-disk') }}</dt><dd>{{ humanBytes(stats?.stored_bytes ?? 0) }}</dd></div>
      <div :title="t('stats-duplicates-title', { count: stats?.duplicate_assets ?? 0 })">
        <dt>{{ t('stats-saved-dedup') }}</dt><dd>{{ humanBytes(saved) }}</dd>
      </div>
      <div><dt>{{ t('stats-distinct-tags') }}</dt><dd>{{ lib ? n(lib.distinct_tags) : '…' }}</dd></div>
      <div>
        <dt>{{ t('stats-archive') }}</dt>
        <dd>
          <button v-if="archived" type="button" class="linkish" @click="emit('archive')">{{ n(archived) }}</button>
          <template v-else>0</template>
        </dd>
      </div>
    </dl>

    <p v-if="failed" class="note">{{ t('stats-load-failed', { error: failed }) }}</p>
    <p v-else-if="!lib" class="note">{{ t('stats-counting') }}</p>
    <template v-else-if="assets">
      <section aria-labelledby="st-kinds">
        <h4 id="st-kinds">{{ t('stats-by-kind') }}</h4>
        <ul class="bars">
          <li v-for="k in lib.kinds" :key="k.value" :title="t('stats-files-size', { name: KIND_LABEL[k.value as AssetKind] ?? k.value, count: k.count, size: humanBytes(k.bytes) })">
            <span class="name"><KindGlyph :kind="k.value as AssetKind" :size="14" />{{ KIND_LABEL[k.value as AssetKind] ?? k.value }}</span>
            <span class="track"><span class="bar" :style="{ width: share(k.bytes, kindMax), background: `var(--k-${k.value})` }" /></span>
            <span class="val">{{ humanBytes(k.bytes) }}</span>
            <span class="sub">{{ n(k.count) }}</span>
          </li>
        </ul>
      </section>

      <section aria-labelledby="st-months">
        <h4 id="st-months">{{ t('stats-months') }}</h4>
        <div class="cols" role="list">
          <div
            v-for="m in months"
            :key="m.month"
            class="col"
            role="listitem"
            :title="t('stats-month', { month: monthName(m.month), count: m.count, size: humanBytes(m.bytes) })"
            :aria-label="t('stats-month', { month: monthName(m.month), count: m.count, size: humanBytes(m.bytes) })"
          >
            <span class="ctrack"><span class="cbar" :style="{ height: m.count ? share(m.count, monthMax) : '0' }" /></span>
            <span class="clabel" aria-hidden="true">{{ m.label }}</span>
          </div>
        </div>
      </section>

      <section aria-labelledby="st-tags">
        <h4 id="st-tags">{{ t('stats-tagging') }}</h4>
        <dl class="facts">
          <div><dt>{{ t('stats-tagged') }}</dt><dd>{{ pct(assets - lib.untagged, assets) }}</dd><dd class="sub">{{ t('stats-untagged', { count: lib.untagged }) }}</dd></div>
          <div><dt>{{ t('stats-described') }}</dt><dd>{{ pct(lib.described, assets) }}</dd><dd class="sub">{{ t('stats-described-assets', { count: lib.described }) }}</dd></div>
          <div><dt>{{ t('stats-human-edited') }}</dt><dd>{{ n(lib.human_edited) }}</dd><dd class="sub">{{ t('stats-human-edited-unit', { count: lib.human_edited }) }}</dd></div>
        </dl>
        <ul class="sources">
          <li v-for="src in lib.tag_sources" :key="src.value">
            <span>{{ sourceLabel(src.value) ?? src.value }}</span>
            <span class="val">{{ n(src.count) }}</span>
            <span class="sub">{{ t('stats-source-different', { count: src.tags }) }}</span>
          </li>
        </ul>
        <h5>{{ t('stats-most-used') }}</h5>
        <ul class="bars tags">
          <li v-for="tag in lib.top_tags" :key="tag.value">
            <button type="button" class="name tagname" :title="t('stats-show-tag', { tag: tag.value })" @click="emit('tag', tag.value)">{{ tag.value }}</button>
            <span class="track"><span class="bar accent" :style="{ width: share(tag.count, tagMax) }" /></span>
            <span class="val">{{ n(tag.count) }}</span>
          </li>
        </ul>
      </section>

      <section aria-labelledby="st-exts">
        <h4 id="st-exts">{{ t('stats-file-types') }}</h4>
        <ul class="bars">
          <li v-for="e in lib.extensions" :key="e.value" :title="t('stats-files-size', { name: `.${e.value}`, count: e.count, size: humanBytes(e.bytes) })">
            <span class="name mono">.{{ e.value || '—' }}</span>
            <span class="track"><span class="bar accent" :style="{ width: share(e.bytes, extMax) }" /></span>
            <span class="val">{{ humanBytes(e.bytes) }}</span>
            <span class="sub">{{ n(e.count) }}</span>
          </li>
        </ul>
      </section>

      <section aria-labelledby="st-packs">
        <h4 id="st-packs">{{ t('stats-biggest-packs') }}</h4>
        <ul class="bars">
          <li v-for="p in lib.packs" :key="p.id">
            <button type="button" class="name" :title="t('stats-browse-pack', { name: p.name })" @click="emit('pack', p)">{{ p.name }}</button>
            <span class="track"><span class="bar accent" :style="{ width: share(p.bytes, packMax) }" /></span>
            <span class="val">{{ humanBytes(p.bytes) }}</span>
            <span class="sub">{{ n(p.count) }}</span>
          </li>
        </ul>
      </section>

      <section aria-labelledby="st-largest">
        <h4 id="st-largest">{{ t('stats-largest-files') }}</h4>
        <ul class="files">
          <li v-for="f in lib.largest" :key="f.id">
            <KindGlyph :kind="f.kind" :size="14" />
            <button type="button" class="name" :title="t('stats-open-file', { name: f.name })" @click="emit('open', f.id)">{{ f.name }}</button>
            <span class="sub">{{ f.pack_name }}</span>
            <span class="val">{{ humanBytes(f.bytes) }}</span>
          </li>
        </ul>
      </section>
    </template>
    <p v-else class="note">{{ t('stats-empty') }}</p>
  </div>
</template>

<style scoped>
.stats { display: grid; gap: 1.6rem; }
.note { margin: 0; color: var(--muted); font-size: var(--step--1); }
h4 { margin: 0 0 0.6rem; font-size: var(--step-0); font-weight: 650; }
h5 { margin: 1rem 0 0.5rem; font-size: var(--step--1); font-weight: 600; color: var(--muted); }
section { min-width: 0; }

.tiles { display: grid; grid-template-columns: repeat(auto-fill, minmax(9.5rem, 1fr)); gap: 0.6rem; margin: 0; }
.tiles > div { padding: 0.75rem 0.9rem; border: 1px solid var(--line); border-radius: 8px; background: var(--panel); }
.tiles dt { font-size: var(--step--1); color: var(--muted); }
.tiles dd { margin: 0.2rem 0 0; font-size: var(--step-2); font-weight: 700; font-variant-numeric: tabular-nums; letter-spacing: -0.01em; }

/* Horizontal bars: name, the bar on a faint track, the value, and a count. */
.bars { display: grid; gap: 0.15rem; margin: 0; padding: 0; list-style: none; font-size: var(--step--1); }
.bars li {
  display: grid; grid-template-columns: minmax(6rem, 11rem) minmax(0, 1fr) 4.8rem 3.6rem; gap: 0.75rem; align-items: center;
  padding: 0.3rem 0.4rem; border-radius: 5px;
}
.bars.tags li { grid-template-columns: minmax(6rem, 11rem) minmax(0, 1fr) 3.6rem; }
.bars li:hover { background: var(--raised); }
.name { display: flex; align-items: center; gap: 0.45rem; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
button.name {
  display: block; padding: 0; border: 0; background: none; color: var(--text); font: inherit; text-align: left; cursor: pointer;
}
button.name:hover { text-decoration: underline; text-underline-offset: 0.15em; }
.track { display: block; height: 0.6rem; }
.bar { display: block; height: 100%; border-radius: 0 4px 4px 0; transition: width var(--dur-3) var(--ease-out); }
.bar.accent { background: var(--accent); }
.val { text-align: right; font-variant-numeric: tabular-nums; }
.sub { color: var(--muted); text-align: right; font-variant-numeric: tabular-nums; }

/* Month columns: one series, with each month's numbers on hover. */
.cols { display: grid; grid-template-columns: repeat(12, minmax(0, 1fr)); gap: 0.35rem; height: 8rem; }
.col { display: grid; grid-template-rows: minmax(0, 1fr) auto; gap: 0.3rem; min-width: 0; }
.ctrack { display: flex; align-items: flex-end; border-bottom: 1px solid var(--line); }
.cbar { display: block; width: 100%; max-width: 2.2rem; margin: 0 auto; border-radius: 4px 4px 0 0; background: var(--accent); transition: height var(--dur-3) var(--ease-out); }
.col:hover .cbar { filter: brightness(1.15); }
.clabel { font-size: 0.7rem; color: var(--muted); text-align: center; overflow: hidden; white-space: nowrap; }

.facts { display: grid; grid-template-columns: repeat(auto-fill, minmax(9rem, 1fr)); gap: 0.6rem; margin: 0 0 0.8rem; }
.facts > div { padding: 0.6rem 0.8rem; border: 1px solid var(--line); border-radius: 8px; }
.facts dt { font-size: var(--step--1); color: var(--muted); }
.facts dd { margin: 0.15rem 0 0; font-size: var(--step-1); font-weight: 650; font-variant-numeric: tabular-nums; }
.facts dd.sub { margin: 0; font-size: var(--step--1); font-weight: 400; text-align: left; }
.sources { display: grid; gap: 0.15rem; margin: 0; padding: 0; list-style: none; font-size: var(--step--1); }
.sources li { display: grid; grid-template-columns: minmax(0, 1fr) 5rem 7rem; gap: 0.75rem; padding: 0.25rem 0.4rem; }

.files { display: grid; gap: 0.15rem; margin: 0; padding: 0; list-style: none; font-size: var(--step--1); }
.files li { display: grid; grid-template-columns: auto minmax(0, 1fr) minmax(0, 12rem) 4.8rem; gap: 0.6rem; align-items: center; padding: 0.3rem 0.4rem; border-radius: 5px; }
.files li:hover { background: var(--raised); }
.files .sub { text-align: left; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

.linkish { padding: 0; border: 0; background: none; color: var(--accent); font: inherit; cursor: pointer; text-decoration: underline; text-underline-offset: 0.12em; }

@media (max-width: 34rem) {
  .bars li { grid-template-columns: minmax(5rem, 8rem) minmax(0, 1fr) 4.4rem; }
  .bars li .sub { display: none; }
  .files li { grid-template-columns: auto minmax(0, 1fr) 4.4rem; }
  .files .sub { display: none; }
  .sources li { grid-template-columns: minmax(0, 1fr) 4rem; }
  .sources .sub { display: none; }
}
</style>
