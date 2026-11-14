<script setup lang="ts">
import type { UsageItems } from '../api'
import { usageWarning } from '../projects'
// The archive: Hoard's recycling bin. Archived packs and assets are out of search, the counts and the tag list, and
// wait here until they're restored or deleted for good (which is when their files are freed).
import { computed, onMounted, ref } from 'vue'
import { api, humanBytes, KIND_LABEL, thumbUrl, type ArchiveItems, type Asset, type Pack } from '../api'
import { countText } from '../archive'
import { mayTry } from '../auth'
import { confirmDialog, daysAgo, showError, showOk } from '../ui'
import KindGlyph from './KindGlyph.vue'

const emit = defineEmits<{ changed: []; error: [string] }>()
const PAGE = 100

const packs = ref<Pack[]>([])
const assets = ref<Asset[]>([])
const assetTotal = ref(0)
const loading = ref(true)
const busy = ref(false)

async function load(more = false) {
  try {
    const [p, a] = await Promise.all([
      more ? Promise.resolve(packs.value) : api.packs(true),
      api.search({ archived: true, limit: PAGE, offset: more ? assets.value.length : 0, sort: '' }),
    ])
    packs.value = p
    assets.value = more ? [...assets.value, ...a.items] : a.items
    assetTotal.value = a.total
  } catch (e) {
    emit('error', `Couldn't load the archive: ${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}
onMounted(() => load())

const empty = computed(() => !loading.value && !packs.value.length && !assetTotal.value)
const packBytes = computed(() => packs.value.reduce((n, p) => n + p.total_bytes, 0))

const when = daysAgo
/** The usual warning, after which projects use what's going, if any do. */
async function warned(body: string, items: UsageItems): Promise<string> {
  const uses = await usageWarning(items)
  return uses ? `${uses.body} ${body}` : body
}

async function run(what: 'restore' | 'purge', items: ArchiveItems & { all?: boolean }) {
  busy.value = true
  try {
    const n = what === 'restore' ? await api.restore(items) : await api.purge(items)
    const text = countText(n.packs, n.assets)
    showOk(what === 'restore' ? `Restored ${text}` : `Deleted ${text} for good`)
    emit('changed')
    await load()
  } catch (e) {
    showError(`Couldn't ${what === 'restore' ? 'restore' : 'delete'}: ${(e as Error).message}`)
  } finally {
    busy.value = false
  }
}

/** Restoring changes the library; deleting for good is for admins. */
const restorer = computed(() => mayTry('edit'))
const purger = computed(() => mayTry('admin'))
const restorePack = (p: Pack) => run('restore', { packs: [p.id] })
const restoreAsset = (a: Asset) => run('restore', { assets: [a.id] })
async function purgePack(p: Pack) {
  const ok = await confirmDialog({
    title: `Delete ${p.name} for good?`,
    body: await warned(
      `Its ${p.asset_count.toLocaleString()} assets and ${humanBytes(p.total_bytes)} of files go too, except files another pack shares. This can't be undone.`,
      { packs: [p.id] },
    ),
    confirm: 'Delete forever',
    danger: true,
  })
  if (ok) await run('purge', { packs: [p.id] })
}
async function purgeAsset(a: Asset) {
  const ok = await confirmDialog({
    title: `Delete ${a.name} for good?`,
    body: await warned("Its file is freed unless another asset shares it. This can't be undone.", { assets: [a.id] }),
    confirm: 'Delete forever',
    danger: true,
  })
  if (ok) await run('purge', { assets: [a.id] })
}
async function emptyArchive() {
  const ok = await confirmDialog({
    title: 'Empty the archive?',
    body: await warned(
      `${countText(packs.value.length, assetTotal.value)} will be deleted for good, and their files freed. This can't be undone.`,
      { archived: true },
    ),
    confirm: 'Empty archive',
    danger: true,
  })
  if (ok) await run('purge', { all: true })
}
</script>

<template>
  <div class="archive">
    <div class="inner">
      <header class="head">
        <div>
          <h2>Archive</h2>
          <p class="what">
            Archived packs and assets are out of search, the counts and the tag list. Restore them, or delete them for
            good to free their space.
          </p>
        </div>
        <button v-if="!empty && !loading && purger" type="button" class="btn danger" :disabled="busy" @click="emptyArchive">Empty archive</button>
      </header>

      <p v-if="loading" class="muted">Loading…</p>

      <div v-else-if="empty" class="empty">
        <svg viewBox="0 0 24 24" width="40" height="40" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="4" width="18" height="4.5" rx="1" />
          <path d="M5 8.5V19a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V8.5M10 12.5h4" />
        </svg>
        <strong>The archive is empty</strong>
        <span>Archive a pack or an asset and it waits here until you restore it or delete it for good.</span>
      </div>

      <template v-else>
        <section v-if="packs.length" aria-labelledby="arch-packs">
          <h3 id="arch-packs">Packs <span class="n">{{ packs.length.toLocaleString() }} · {{ humanBytes(packBytes) }}</span></h3>
          <ul class="rows">
            <li v-for="p in packs" :key="p.id" class="row">
              <div class="cover" aria-hidden="true">
                <img v-if="p.preview[0]" :src="thumbUrl({ id: p.preview[0] })" alt="" loading="lazy" />
              </div>
              <div class="info">
                <strong :title="p.name">{{ p.name }}</strong>
                <span>{{ p.asset_count.toLocaleString() }} {{ p.asset_count === 1 ? 'asset' : 'assets' }} · {{ humanBytes(p.total_bytes) }} · archived {{ when(p.archived_at) }}</span>
              </div>
              <div class="acts">
                <button v-if="restorer" type="button" class="btn" :disabled="busy" @click="restorePack(p)">Restore</button>
                <button v-if="purger" type="button" class="btn danger" :disabled="busy" @click="purgePack(p)">Delete forever</button>
              </div>
            </li>
          </ul>
        </section>

        <section v-if="assetTotal" aria-labelledby="arch-assets">
          <h3 id="arch-assets">Assets <span class="n">{{ assetTotal.toLocaleString() }}</span></h3>
          <ul class="rows">
            <li v-for="a in assets" :key="a.id" class="row">
              <div class="cover" aria-hidden="true">
                <img v-if="a.has_thumb" :src="thumbUrl(a)" alt="" loading="lazy" />
                <KindGlyph v-else :kind="a.kind" />
              </div>
              <div class="info">
                <strong :title="a.path">{{ a.name }}</strong>
                <span>{{ KIND_LABEL[a.kind] ?? a.kind }} · {{ humanBytes(a.size) }} · from {{ a.pack_name }} · archived {{ when(a.archived_at) }}</span>
              </div>
              <div class="acts">
                <button v-if="restorer" type="button" class="btn" :disabled="busy" @click="restoreAsset(a)">Restore</button>
                <button v-if="purger" type="button" class="btn danger" :disabled="busy" @click="purgeAsset(a)">Delete forever</button>
              </div>
            </li>
          </ul>
          <div v-if="assets.length < assetTotal" class="more">
            <button type="button" class="btn ghost" @click="load(true)">Show more ({{ (assetTotal - assets.length).toLocaleString() }} left)</button>
          </div>
        </section>
      </template>
    </div>
  </div>
</template>

<style scoped>
.archive { overflow-y: auto; scrollbar-gutter: stable; }
.inner { max-width: 60rem; margin: 0 auto; padding: 1.4rem 2rem 5rem; }
.head { display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; margin-bottom: 1.2rem; }
h2 { margin: 0 0 0.25rem; font-size: var(--step-2); font-variation-settings: 'CASL' 0.5; }
.what { margin: 0; max-width: 40rem; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.muted { color: var(--muted); }
section { padding: 1rem 0; border-top: 1px solid var(--line); }
h3 { margin: 0 0 0.6rem; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
.n { margin-left: 0.4rem; font-weight: 500; letter-spacing: 0; text-transform: none; font-variant-numeric: tabular-nums; }
.rows { display: grid; gap: 0.25rem; margin: 0; padding: 0; list-style: none; }
.row {
  display: grid; grid-template-columns: 2.75rem minmax(0, 1fr) auto; align-items: center; gap: 0.8rem;
  padding: 0.45rem 0.6rem; border-radius: var(--r-chip); transition: background-color var(--dur-1);
}
.row:hover { background: color-mix(in srgb, var(--raised) 55%, transparent); }
.cover {
  display: grid; place-items: center; width: 2.75rem; height: 2.75rem; overflow: hidden;
  border-radius: 6px; background: var(--raised); color: var(--muted);
}
.cover img { width: 100%; height: 100%; object-fit: cover; filter: grayscale(0.6); opacity: 0.85; }
.info { display: grid; gap: 0.1rem; min-width: 0; }
.info strong { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-weight: 600; }
.info span { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: var(--step--1); color: var(--muted); }
.acts { display: flex; gap: 0.35rem; }
.more { display: flex; justify-content: center; padding-top: 0.8rem; }
.empty {
  display: grid; justify-items: center; gap: 0.4rem; padding: 4rem 1rem; text-align: center; color: var(--muted);
}
.empty svg { color: var(--faint); margin-bottom: 0.4rem; }
.empty strong { color: var(--text); font-size: var(--step-1); }
.empty span { max-width: 28rem; font-size: var(--step--1); }
@media (max-width: 47.99rem) {
  .inner { padding: 1rem 1rem 5rem; }
  .head { flex-direction: column; }
  .row { grid-template-columns: 2.75rem minmax(0, 1fr); }
  .acts { grid-column: 1 / -1; justify-content: flex-end; }
}
</style>
