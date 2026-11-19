<script setup lang="ts">
// Settings → General, for everyone: the library as a dragon's hoard. Every asset is a piece of treasure, its kind
// picks which, and each account keeps a record of how many of each kind it has collected (`/api/me/hoard`). What the
// library holds beyond that record is loot still to collect: Collect pours it onto the pile and records it. The
// library is counted again now and then, so what's added while the page is open turns up as more to collect.
// A guest has no record, so their hoard is the whole library, and new loot pours straight in.
// The hoard is drawn by `dragonHoard` (three.js), which is loaded only here. It's seen through an arcade screen unless
// this browser turned that off, and it can fill the screen.
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { ApiError, api, type AssetKind, type Stats } from '../api'
import { askToSignIn, auth } from '../auth'
import type { DragonHoardHandle, HoardCounts, TreasureType } from '../dragonHoard'
import { humanBytes } from '../format'
import { formatNumber, t } from '../i18n'
import { KIND_LABEL } from '../kinds'
import { prefs } from '../prefs'
import { showError } from '../ui'

const emit = defineEmits<{ unauthorized: [] }>()

/** The treasure each kind of asset is. */
const TREASURE: Record<AssetKind, TreasureType> = {
  texture: 'gold',
  material: 'gold',
  other: 'gold',
  model: 'gems',
  audio: 'weapons',
  animation: 'bones',
  font: 'tablets',
  vector: 'paintings',
  video: 'paintings',
  sheet: 'potions',
  document: 'books',
  data: 'books',
  shader: 'books',
}
const ORDER: TreasureType[] = ['gold', 'gems', 'weapons', 'bones', 'tablets', 'paintings', 'potions', 'books']
const TREASURE_LABEL: Record<TreasureType, string> = {
  get gold() { return t('hoard-gold') },
  get gems() { return t('hoard-gems') },
  get books() { return t('hoard-books') },
  get weapons() { return t('hoard-weapons') },
  get bones() { return t('hoard-bones') },
  get tablets() { return t('hoard-tablets') },
  get paintings() { return t('hoard-paintings') },
  get potions() { return t('hoard-potions') },
}
/** How often the library is counted again while the page is open. */
const RECOUNT_MS = 20_000

type ByKind = Record<string, number>

const stats = ref<Stats | null>(null)
/** What this account has collected, by kind; null before it has collected anything. */
const seen = ref<ByKind | null>(null)
const loaded = ref(false)
const signedIn = computed(() => !!auth.status?.signed_in)
const unavailable = ref('')
const busy = ref(false)

/** What the library holds now, by kind. */
const have = computed<ByKind>(() => Object.fromEntries((stats.value?.kinds ?? []).map((f) => [f.value, f.count])))
/** What's on the pile, by kind: what's been collected (never more than the library holds), or for a guest, everything. */
const shown = computed<ByKind>(() => {
  if (!signedIn.value) return have.value
  const s = seen.value ?? {}
  return Object.fromEntries(Object.entries(have.value).map(([k, n]) => [k, Math.min(n, s[k] ?? 0)]))
})
const fresh = (k: string) => Math.max(0, (have.value[k] ?? 0) - (shown.value[k] ?? 0))
const newCount = computed(() => Object.keys(have.value).reduce((sum, k) => sum + fresh(k), 0))

function treasure(byKind: ByKind): HoardCounts {
  const out: HoardCounts = {}
  for (const [k, n] of Object.entries(byKind)) {
    const tt = TREASURE[k as AssetKind] ?? 'gold'
    out[tt] = (out[tt] ?? 0) + n
  }
  return out
}

/** Each kind of treasure the library has: how much is on the pile, how much is still to collect, and what it stands for. */
const legend = computed(() =>
  ORDER.map((tt) => {
    const kinds = Object.keys(have.value).filter((k) => (TREASURE[k as AssetKind] ?? 'gold') === tt && have.value[k] > 0)
    return {
      type: tt,
      pile: kinds.reduce((sum, k) => sum + (shown.value[k] ?? 0), 0),
      fresh: kinds.reduce((sum, k) => sum + fresh(k), 0),
      kinds: kinds.map((k) => KIND_LABEL[k as AssetKind] ?? k).join(', '),
    }
  }).filter((r) => r.kinds),
)

const cave = ref<HTMLElement | null>(null)
const stage = ref<HTMLElement | null>(null)
let hoard: DragonHoardHandle | null = null
let closed = false

async function mount() {
  if (!stage.value) return
  try {
    const { mountDragonHoard } = await import('../dragonHoard')
    if (closed || !stage.value) return
    hoard = mountDragonHoard(stage.value, {
      counts: treasure(shown.value),
      label: (s) => t('hoard-label', { count: s.total }),
      roarOnClick: true,
      retro: prefs.hoardArcade,
    })
    await hoard.ready
  } catch (e) {
    hoard?.dispose()
    hoard = null
    unavailable.value = (e as Error).message
  }
}

// The pile follows what's shown: it grows when loot is collected (or, for a guest, added), and shrinks at once when
// the library does.
watch(shown, (next) => hoard?.setCounts(treasure(next)))
watch(() => prefs.hoardArcade, (on) => hoard?.setRetro(on))

/** Whether this browser lets the cave fill the screen (iPhones don't), and whether it does now. */
const canFill = !!document.fullscreenEnabled
const filling = ref(false)
function onFullscreen() {
  filling.value = !!cave.value && document.fullscreenElement === cave.value
}
async function toggleFill() {
  try {
    if (document.fullscreenElement) await document.exitFullscreen()
    else await cave.value?.requestFullscreen()
  } catch {
    // Refused (no user gesture, or a policy): the cave stays where it is.
  }
}

async function count() {
  try {
    stats.value = await api.stats()
  } catch {
    // Counted again on the next round.
  }
}

async function loadSeen() {
  if (!signedIn.value) return
  try {
    seen.value = (await api.hoard()).seen
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  }
}

async function collect() {
  const next = { ...have.value }
  busy.value = true
  seen.value = next
  try {
    seen.value = (await api.setHoard(next)).seen
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError(t('hoard-collect-failed', { error: (e as Error).message }))
  } finally {
    busy.value = false
  }
}

let timer: number | undefined
function recount() {
  timer = window.setTimeout(async () => {
    if (document.visibilityState === 'visible') await count()
    if (!closed) recount()
  }, RECOUNT_MS)
}

onMounted(async () => {
  await Promise.all([count(), loadSeen()])
  loaded.value = true
  if (closed) return
  await mount()
  recount()
})
document.addEventListener('fullscreenchange', onFullscreen)
onBeforeUnmount(() => {
  closed = true
  document.removeEventListener('fullscreenchange', onFullscreen)
  if (filling.value) void document.exitFullscreen().catch(() => {})
  window.clearTimeout(timer)
  hoard?.dispose()
  hoard = null
})
</script>

<template>
  <section class="hoard" aria-labelledby="hoard-title">
    <div class="sh"><h3 id="hoard-title">{{ t('hoard-title') }}</h3></div>
    <p class="what">{{ t('hoard-what') }}</p>

    <div ref="cave" class="cave">
      <div ref="stage" class="stage" />
      <div class="tools">
        <button
          type="button"
          class="tool"
          :aria-pressed="prefs.hoardArcade"
          :title="t('hoard-arcade')"
          :aria-label="t('hoard-arcade')"
          @click="prefs.hoardArcade = !prefs.hoardArcade"
        >
          <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <rect x="1.75" y="5" width="12.5" height="9" rx="2" /><path d="M5.5 1.75 8 5l2.5-3.25M11.5 8.25v2.5" />
          </svg>
        </button>
        <button
          v-if="canFill"
          type="button"
          class="tool"
          :title="filling ? t('hoard-leave-full-screen') : t('hoard-full-screen')"
          :aria-label="filling ? t('hoard-leave-full-screen') : t('hoard-full-screen')"
          @click="toggleFill"
        >
          <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path v-if="filling" d="M6 2v4H2M10 2v4h4M6 14v-4H2M10 14v-4h4" />
            <path v-else d="M2 6V2h4M14 6V2h-4M2 10v4h4M14 10v4h-4" />
          </svg>
        </button>
      </div>
      <p v-if="unavailable" class="off">{{ t('hoard-unavailable', { error: unavailable }) }}</p>
      <div v-if="loaded" class="loot">
        <template v-if="signedIn">
          <span class="count" aria-live="polite">
            {{ newCount ? t('hoard-new', { count: newCount }) : t('hoard-none-new') }}
          </span>
          <button v-if="newCount" type="button" class="btn primary" :disabled="busy" @click="collect">{{ t('hoard-collect') }}</button>
        </template>
        <button v-else type="button" class="btn" @click="askToSignIn">{{ t('hoard-sign-in') }}</button>
      </div>
    </div>

    <dl class="tiles">
      <div><dt>{{ t('stats-assets') }}</dt><dd>{{ formatNumber(stats?.assets ?? 0) }}</dd></div>
      <div><dt>{{ t('stats-packs') }}</dt><dd>{{ formatNumber(stats?.packs ?? 0) }}</dd></div>
      <div><dt>{{ t('stats-on-disk') }}</dt><dd>{{ humanBytes(stats?.stored_bytes ?? 0) }}</dd></div>
    </dl>

    <ul class="legend" :aria-label="t('hoard-title')">
      <li v-for="r in legend" :key="r.type" :data-treasure="r.type">
        <span class="swatch" aria-hidden="true" />
        <span class="name">{{ TREASURE_LABEL[r.type] }}</span>
        <span class="val">{{ formatNumber(r.pile) }}</span>
        <span class="fresh">{{ r.fresh ? formatNumber(r.fresh, { signDisplay: 'always' }) : '' }}</span>
        <span class="kinds">{{ r.kinds }}</span>
      </li>
    </ul>
  </section>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.hoard { display: grid; gap: 0.9rem; }
.hoard > .what { margin: 0; }

/* The hoard is always in its cave, dark whatever the theme. */
.cave {
  position: relative; height: clamp(16rem, 52vh, 30rem); border-radius: 12px; overflow: hidden;
  border: 1px solid var(--line); box-shadow: var(--elev-1);
  background: radial-gradient(ellipse at 50% 70%, #3a2a1c 0%, #1b1410 55%, #0d0a08 100%);
}
.cave:fullscreen { border: 0; border-radius: 0; }
.stage { position: absolute; inset: 0; }
/* Small buttons in the corner: the arcade screen on or off, and full screen. */
.tools { position: absolute; top: 0.6rem; right: 0.6rem; display: flex; gap: 0.35rem; }
.tool {
  display: grid; place-items: center; width: 2rem; height: 2rem; padding: 0; border: 1px solid #ffffff1f; border-radius: 8px;
  background: color-mix(in srgb, #0d0a08 65%, transparent); color: #cbbca6; cursor: pointer; backdrop-filter: blur(4px);
}
.tool:hover { color: #f3e7d3; border-color: #ffffff40; }
.tool:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
.tool[aria-pressed='true'] { color: #ffd27a; border-color: #ffd27a59; }
.off { position: absolute; inset: auto 1rem 50% 1rem; margin: 0; text-align: center; color: #e9dcc8; font-size: var(--step--1); }
.loot {
  position: absolute; left: 0.75rem; bottom: 0.75rem; display: flex; align-items: center; gap: 0.6rem; flex-wrap: wrap;
  max-width: calc(100% - 1.5rem); padding: 0.45rem 0.6rem 0.45rem 0.8rem; border-radius: 10px;
  background: color-mix(in srgb, #0d0a08 72%, transparent); color: #f3e7d3; font-size: var(--step--1);
  backdrop-filter: blur(4px);
}
.loot .count { font-weight: 600; font-variant-numeric: tabular-nums; }

.tiles { display: grid; grid-template-columns: repeat(auto-fill, minmax(9.5rem, 1fr)); gap: 0.6rem; margin: 0; }
.tiles > div {
  padding: 0.75rem 0.9rem; border: 1px solid var(--line); border-radius: 10px; box-shadow: var(--elev-1);
  background: linear-gradient(160deg, color-mix(in srgb, var(--accent) 7%, var(--card, var(--panel))), var(--card, var(--panel)) 70%);
}
.tiles dt { font-size: var(--step--1); color: var(--muted); }
.tiles dd { margin: 0.2rem 0 0; font-size: var(--step-2); font-weight: 700; font-variant-numeric: tabular-nums; letter-spacing: -0.01em; }

/* One row per kind of treasure: its name, how much is on the pile, how much is still to collect, what it stands for. */
.legend { display: grid; gap: 0.15rem; margin: 0; padding: 0; list-style: none; font-size: var(--step--1); }
.legend li {
  display: grid; grid-template-columns: 0.7rem minmax(6rem, 9rem) 4.5rem 3.5rem minmax(0, 1fr); gap: 0.6rem; align-items: center;
  padding: 0.25rem 0.4rem;
}
.swatch { width: 0.7rem; height: 0.7rem; border-radius: 3px; background: var(--sw); }
.val, .fresh { text-align: end; font-variant-numeric: tabular-nums; }
.fresh { color: var(--accent); font-weight: 600; }
.kinds { color: var(--muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
[data-treasure='gold'] { --sw: #e0b13a; }
[data-treasure='gems'] { --sw: #3fa7d6; }
[data-treasure='weapons'] { --sw: #9aa3ad; }
[data-treasure='bones'] { --sw: #e6dcc6; }
[data-treasure='tablets'] { --sw: #7f8a6e; }
[data-treasure='paintings'] { --sw: #b0563c; }
[data-treasure='potions'] { --sw: #9b5fc0; }
[data-treasure='books'] { --sw: #7a4b2a; }
@media (max-width: 30rem) {
  .legend li { grid-template-columns: 0.7rem minmax(0, 1fr) 4rem 3rem; }
  .kinds { grid-column: 2 / -1; }
}
</style>
