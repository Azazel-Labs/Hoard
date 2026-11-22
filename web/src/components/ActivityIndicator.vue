<script setup lang="ts">
// What the server is doing in the background (ingests, tagging, conversions, zips, the ffmpeg download): a header
// button with a count, and a list on hover or click. Polls quickly while anything runs or the list is open. An admin
// can stop what can be stopped: take a job off the queue, or let background work and Maintenance go partway through.
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { api, type ActivityItem } from '../api'
import { may } from '../auth'
import { t } from '../i18n'
import { showError } from '../ui'

const items = ref<ActivityItem[]>([])
/** AI tagging waiting or running, all of it, for stopping in one go after a big import. */
const tagging = ref(0)
const admin = computed(() => may('admin'))
/** What's been asked to stop, until it's gone from the list. */
const stopping = ref(new Set<string>())
const open = ref(false)
const pinned = ref(false)
const failed = ref(false)
const root = ref<HTMLElement | null>(null)

/** Bumped whenever the count goes up, to give the badge a little hop. */
const bump = ref(0)
watch(() => items.value.length, (n, was) => { if (n > was) bump.value++ })

const running = computed(() => items.value.filter((i) => i.state === 'running'))
const queued = computed(() => items.value.filter((i) => i.state === 'queued'))
const label = computed(() => {
  if (!items.value.length) return t('activity-label-idle')
  return queued.value.length
    ? t('activity-label-queued', { running: running.value.length, queued: queued.value.length })
    : t('activity-label-running', { running: running.value.length })
})

let timer: ReturnType<typeof setTimeout> | undefined
async function refresh() {
  clearTimeout(timer)
  try {
    const got = await api.activity()
    items.value = got.items
    tagging.value = got.ai_tagging ?? 0
    const live = new Set(got.items.map((i) => i.stop))
    stopping.value = new Set([...stopping.value].filter((s) => live.has(s)))
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

async function stop(i: ActivityItem) {
  if (!i.stop) return
  stopping.value = new Set(stopping.value).add(i.stop)
  try {
    await api.stopActivity(i.stop)
  } catch (e) {
    showError((e as Error).message)
  }
  await refresh()
}
const stoppingAll = ref(false)
async function stopTagging() {
  stoppingAll.value = true
  try {
    await api.stopJobs('llm_tag')
  } catch (e) {
    showError((e as Error).message)
  } finally {
    stoppingAll.value = false
  }
  await refresh()
}

function since(iso: string) {
  const s = Math.max(0, Math.round((Date.now() - Date.parse(iso)) / 1000))
  if (s < 60) return t('activity-seconds', { s })
  if (s < 3600) return t('activity-minutes', { min: Math.floor(s / 60) })
  return t('activity-hours', { h: Math.floor(s / 3600), min: Math.floor((s % 3600) / 60) })
}
const pct = (p: number) => `${Math.round(p * 100)}%`
const pctText = (p: number) => t('activity-percent', { pct: Math.round(p * 100) })
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
      <Transition name="badge"><span v-if="items.length" :key="bump" class="count">{{ items.length }}</span></Transition>
    </button>

    <Transition name="pop">
    <section v-if="open" id="activity-list" class="panel" :aria-label="t('activity-title')">
      <header>
        <h2>{{ t('activity-title') }}</h2>
        <span v-if="failed" class="stale">{{ t('activity-unreachable') }}</span>
      </header>
      <p v-if="!items.length" class="empty">
        {{ t('activity-empty') }}
      </p>
      <ul v-else>
        <li v-for="i in items" :key="i.id" :class="i.state">
          <div class="line">
            <span class="what">{{ i.label }}</span>
            <span class="when mono">{{ i.state === 'queued' ? t('activity-queued') : i.progress !== undefined ? pctText(i.progress) : since(i.since) }}</span>
            <button
              v-if="admin && i.stop"
              type="button"
              class="stop"
              :aria-label="t('activity-stop', { what: i.label })"
              :title="t('activity-stop', { what: i.label })"
              :disabled="stopping.has(i.stop)"
              @click.stop="stop(i)"
            >
              <svg viewBox="0 0 12 12" width="10" height="10" aria-hidden="true" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><path d="M2.5 2.5l7 7M9.5 2.5l-7 7" /></svg>
            </button>
          </div>
          <div v-if="i.state === 'running'" class="bar" :class="{ unknown: i.progress === undefined }">
            <span :style="i.progress !== undefined ? { width: pct(i.progress) } : undefined" />
          </div>
          <div v-if="i.detail" class="detail">{{ i.detail }}</div>
        </li>
      </ul>
      <footer v-if="admin && tagging > 1">
        <button type="button" class="btn" :disabled="stoppingAll" @click.stop="stopTagging">
          {{ t('activity-stop-tagging', { count: tagging }) }}
        </button>
      </footer>
    </section>
    </Transition>
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
.trigger.busy { color: var(--accent); }
.trigger.busy svg path { stroke-dasharray: 34; animation: trace 1.6s linear infinite; }
@keyframes trace { from { stroke-dashoffset: 68; } to { stroke-dashoffset: 0; } }
.count {
  position: absolute; top: -0.45rem; right: -0.45rem; min-width: 1.1rem; height: 1.1rem; padding: 0 0.25rem;
  border-radius: 999px; background: var(--accent); color: var(--bg);
  font-size: 0.6875rem; font-weight: 700; line-height: 1.1rem; text-align: center; font-variant-numeric: tabular-nums;
}
.badge-enter-active { animation: hop var(--dur-3) var(--ease-pop); }
.badge-leave-active { display: none; }
@keyframes hop { from { scale: 0.4; } 60% { scale: 1.25; } }

.panel {
  transform-origin: top right;
  position: absolute; right: 0; top: calc(100% + 0.5rem); z-index: 30; width: min(24rem, calc(100vw - 1.5rem));
  max-height: min(28rem, 70vh); overflow: auto;
  border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--panel);
  box-shadow: var(--elev-3); font-size: var(--step--1);
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
.stop {
  flex: none; display: grid; place-items: center; width: 1.3rem; height: 1.3rem; margin: -0.2rem -0.35rem -0.2rem 0;
  padding: 0; border: 0; border-radius: var(--r-chip); background: transparent; color: var(--faint); cursor: pointer;
}
.stop:hover:not(:disabled), .stop:focus-visible { color: var(--danger); background: var(--raised); }
.stop:disabled { opacity: 0.4; cursor: default; }
footer { padding: 0.55rem 0.9rem 0.7rem; border-top: 1px solid var(--line); }
.bar { position: relative; height: 4px; margin-top: 0.4rem; border-radius: 2px; background: var(--raised); overflow: hidden; }
.bar span { position: absolute; inset: 0 auto 0 0; border-radius: 2px; background: var(--accent); transition: width 0.4s ease; }
.bar.unknown span { width: 30%; animation: sweep 1.4s ease-in-out infinite; }
@keyframes sweep { from { left: -30%; } to { left: 100%; } }

@media (prefers-reduced-motion: reduce) {
  .trigger.busy svg path, .bar.unknown span { animation: none; }
  .bar.unknown span { width: 100%; opacity: 0.35; }
  .bar span { transition: none; }
}
</style>
