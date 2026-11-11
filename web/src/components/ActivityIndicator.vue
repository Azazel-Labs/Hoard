<script setup lang="ts">
// What the server is doing in the background (ingests, tagging, conversions, zips, the ffmpeg download): a header
// button with a count, and a list on hover or click. Polls quickly while anything runs or the list is open.
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { api, type ActivityItem } from '../api'

const items = ref<ActivityItem[]>([])
const open = ref(false)
const pinned = ref(false)
const failed = ref(false)
const root = ref<HTMLElement | null>(null)

const running = computed(() => items.value.filter((i) => i.state === 'running'))
const queued = computed(() => items.value.filter((i) => i.state === 'queued'))
const label = computed(() => {
  if (!items.value.length) return 'Background activity: idle'
  const q = queued.value.length ? `, ${queued.value.length} queued` : ''
  return `Background activity: ${running.value.length} running${q}`
})

let timer: ReturnType<typeof setTimeout> | undefined
async function refresh() {
  clearTimeout(timer)
  try {
    items.value = (await api.activity()).items
    failed.value = false
  } catch {
    failed.value = true
  }
  const busy = items.value.length > 0 || open.value
  timer = setTimeout(refresh, document.hidden ? 15000 : busy ? 1500 : 8000)
}

let hoverTimer: ReturnType<typeof setTimeout> | undefined
function hover(on: boolean) {
  clearTimeout(hoverTimer)
  if (pinned.value) return
  hoverTimer = setTimeout(() => {
    if (on && !open.value) {
      open.value = true
      refresh()
    } else if (!on) {
      open.value = false
    }
  }, on ? 120 : 250)
}
function toggle() {
  pinned.value = !pinned.value || !open.value
  open.value = pinned.value
  if (open.value) refresh()
}
function close() {
  open.value = false
  pinned.value = false
}
function onDocClick(e: MouseEvent) {
  if (open.value && root.value && !root.value.contains(e.target as Node)) close()
}
function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape' && open.value) close()
}
const onVisible = () => !document.hidden && refresh()

onMounted(() => {
  refresh()
  document.addEventListener('click', onDocClick)
  document.addEventListener('keydown', onKey)
  document.addEventListener('visibilitychange', onVisible)
})
onBeforeUnmount(() => {
  clearTimeout(timer)
  clearTimeout(hoverTimer)
  document.removeEventListener('click', onDocClick)
  document.removeEventListener('keydown', onKey)
  document.removeEventListener('visibilitychange', onVisible)
})

function since(iso: string) {
  const s = Math.max(0, Math.round((Date.now() - Date.parse(iso)) / 1000))
  if (s < 60) return `${s} s`
  if (s < 3600) return `${Math.floor(s / 60)} min`
  return `${Math.floor(s / 3600)} h ${Math.floor((s % 3600) / 60)} min`
}
const pct = (p: number) => `${Math.round(p * 100)}%`
</script>

<template>
  <div ref="root" class="activity" @mouseenter="hover(true)" @mouseleave="hover(false)">
    <button
      type="button"
      class="trigger"
      :class="{ busy: running.length > 0 }"
      :aria-label="label"
      :aria-expanded="open"
      aria-controls="activity-list"
      @click="toggle"
    >
      <svg viewBox="0 0 20 16" width="18" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
        <path d="M1 8h3.5l2-5.5 3.5 11 2.5-8 1.8 2.5H19" />
      </svg>
      <span v-if="items.length" class="count">{{ items.length }}</span>
    </button>

    <section v-if="open" id="activity-list" class="panel" aria-label="Background activity">
      <header>
        <h2>Background activity</h2>
        <span v-if="failed" class="stale">Can't reach the server</span>
      </header>
      <p v-if="!items.length" class="empty">
        Nothing running. Imports, tagging, conversions and zip downloads show up here while they work.
      </p>
      <ul v-else>
        <li v-for="i in items" :key="i.id" :class="i.state">
          <div class="line">
            <span class="what">{{ i.label }}</span>
            <span class="when mono">{{ i.state === 'queued' ? 'queued' : i.progress !== undefined ? pct(i.progress) : since(i.since) }}</span>
          </div>
          <div v-if="i.state === 'running'" class="bar" :class="{ unknown: i.progress === undefined }">
            <span :style="i.progress !== undefined ? { width: pct(i.progress) } : undefined" />
          </div>
          <div v-if="i.detail" class="detail">{{ i.detail }}</div>
        </li>
      </ul>
    </section>
  </div>
</template>

<style scoped>
.activity { position: relative; }
.trigger {
  position: relative; display: grid; place-items: center; width: 2.1rem; height: 2rem; padding: 0;
  border: 1px solid var(--line); border-radius: var(--r-chip); background: transparent; color: var(--faint);
  cursor: pointer;
}
.trigger:hover, .trigger[aria-expanded='true'] { color: var(--text); border-color: var(--muted); }
.trigger.busy { color: var(--k-font); }
.trigger.busy svg path { stroke-dasharray: 34; animation: trace 1.6s linear infinite; }
@keyframes trace { from { stroke-dashoffset: 68; } to { stroke-dashoffset: 0; } }
.count {
  position: absolute; top: -0.45rem; right: -0.45rem; min-width: 1.1rem; height: 1.1rem; padding: 0 0.25rem;
  border-radius: 999px; background: var(--k-font); color: var(--bg);
  font-size: 0.6875rem; font-weight: 700; line-height: 1.1rem; text-align: center; font-variant-numeric: tabular-nums;
}

.panel {
  position: absolute; right: 0; top: calc(100% + 0.5rem); z-index: 30; width: min(24rem, calc(100vw - 1.5rem));
  max-height: min(28rem, 70vh); overflow: auto;
  border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--panel);
  box-shadow: 0 12px 32px var(--shadow); font-size: var(--step--1);
}
.panel header {
  display: flex; align-items: baseline; justify-content: space-between; gap: 1rem;
  padding: 0.7rem 0.9rem 0.5rem; border-bottom: 1px solid var(--line);
}
.panel h2 { margin: 0; font-size: var(--step-0); font-weight: 650; }
.stale { color: var(--danger); }
.empty { margin: 0; padding: 0.9rem; color: var(--muted); }
ul { list-style: none; margin: 0; padding: 0.3rem 0; }
li { padding: 0.55rem 0.9rem; }
li + li { border-top: 1px solid color-mix(in srgb, var(--line) 60%, transparent); }
li.queued .what { color: var(--muted); }
.line { display: flex; align-items: baseline; gap: 0.75rem; }
.what { flex: 1; min-width: 0; overflow-wrap: anywhere; }
.when { color: var(--muted); white-space: nowrap; font-variant-numeric: tabular-nums; }
.detail { margin-top: 0.2rem; color: var(--faint); }
.bar { position: relative; height: 4px; margin-top: 0.4rem; border-radius: 2px; background: var(--raised); overflow: hidden; }
.bar span { position: absolute; inset: 0 auto 0 0; border-radius: 2px; background: var(--k-font); transition: width 0.4s ease; }
.bar.unknown span { width: 30%; animation: sweep 1.4s ease-in-out infinite; }
@keyframes sweep { from { left: -30%; } to { left: 100%; } }

@media (prefers-reduced-motion: reduce) {
  .trigger.busy svg path, .bar.unknown span { animation: none; }
  .bar.unknown span { width: 100%; opacity: 0.35; }
  .bar span { transition: none; }
}
</style>
