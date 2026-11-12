<script setup lang="ts">
// Settings → Maintenance: run the jobs that look after the library and see how past runs went.
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { api, ApiError, type Maintenance, type MaintenanceTask } from '../api'
import { confirmDialog, showError } from '../ui'

const emit = defineEmits<{ unauthorized: [] }>()

const data = ref<Maintenance | null>(null)
const starting = ref<string | null>(null)
let timer: ReturnType<typeof setTimeout> | undefined
let gone = false

const anyRunning = computed(() => data.value?.tasks.some((t) => t.running) ?? false)
const labels = computed(() => Object.fromEntries((data.value?.tasks ?? []).map((t) => [t.id, t.label])))

function failed(e: unknown, what: string) {
  if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  showError(`${what}: ${(e as Error).message}`)
}

async function load() {
  clearTimeout(timer)
  try {
    data.value = await api.maintenance()
  } catch (e) {
    failed(e, "Couldn't load Maintenance")
  }
  // Follow running tasks closely; otherwise just keep the history current.
  if (!gone) timer = setTimeout(load, anyRunning.value ? 1500 : 5000)
}
onMounted(load)
onUnmounted(() => {
  gone = true
  clearTimeout(timer)
})

/** Tasks that change or delete things ask first. */
const ASK: Record<string, { title: string; body: string; confirm: string; danger?: boolean }> = {
  reanalyze: {
    title: 'Re-analyze every pack?',
    body: 'Every file is read again, which can take a long while on a big library; packs show as working until it’s their turn. Your own names, tags and notes stay.',
    confirm: 'Re-analyze',
  },
  gc: {
    title: 'Delete unused files?',
    body: 'Stored files that nothing in the library points at are deleted for good. Packs in the archive keep theirs.',
    confirm: 'Delete them',
    danger: true,
  },
}

async function run(t: MaintenanceTask) {
  const ask = ASK[t.id]
  if (ask && !(await confirmDialog(ask))) return
  starting.value = t.id
  try {
    await api.runMaintenance(t.id)
  } catch (e) {
    failed(e, `Couldn't start “${t.label}”`)
  } finally {
    starting.value = null
  }
  await load()
}

const fmt = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'short' })
function when(at: string): string {
  const secs = (Date.now() - Date.parse(at)) / 1000
  if (secs < 60) return 'just now'
  if (secs < 3600) return `${Math.round(secs / 60)} min ago`
  if (secs < 86400) return `${Math.round(secs / 3600)} h ago`
  return fmt.format(new Date(at))
}
function took(from: string, to: string | null): string {
  if (!to) return ''
  const s = Math.max(0, Math.round((Date.parse(to) - Date.parse(from)) / 1000))
  return s < 60 ? `${s} s` : s < 3600 ? `${Math.round(s / 60)} min` : `${(s / 3600).toFixed(1)} h`
}
</script>

<template>
  <div class="maint" :aria-busy="!data">
    <p v-if="!data" class="loading">Loading…</p>
    <template v-else>
      <ul class="tasks">
        <li v-for="t in data.tasks" :key="t.id" :class="{ on: t.running }">
          <div class="top">
            <div class="what">
              <strong>{{ t.label }}</strong>
              <small>{{ t.blurb }}</small>
            </div>
            <button
              type="button"
              class="btn"
              :disabled="!!t.running || starting === t.id"
              @click="run(t)"
            >
              {{ t.running ? 'Running…' : starting === t.id ? 'Starting…' : 'Run' }}
            </button>
          </div>
          <div v-if="t.running" class="live" role="status">
            <div class="bar" :class="{ unknown: t.running.progress == null }">
              <span :style="t.running.progress != null ? { width: `${Math.round(t.running.progress * 100)}%` } : undefined" />
            </div>
            <small>{{ t.running.detail ?? 'Starting' }} · started {{ when(t.running.started_at) }}</small>
          </div>
          <p v-else-if="t.last" class="last" :class="{ bad: !t.last.ok }">
            <span class="dot" aria-hidden="true" />
            <span>
              <span class="sr">{{ t.last.ok ? 'Finished' : 'Failed' }}: </span>{{ t.last.summary }}
              <small>· {{ when(t.last.finished_at ?? t.last.started_at) }}</small>
            </span>
          </p>
          <p v-else class="last never">Never run</p>
        </li>
      </ul>

      <section v-if="data.history.length" class="history">
        <h3>Past runs</h3>
        <table>
          <thead>
            <tr><th scope="col">Task</th><th scope="col">Finished</th><th scope="col">Took</th><th scope="col">Result</th></tr>
          </thead>
          <tbody>
            <tr v-for="r in data.history" :key="r.id" :class="{ bad: !r.ok }">
              <td>{{ labels[r.task] ?? r.task }}</td>
              <td class="num">{{ r.finished_at ? when(r.finished_at) : '' }}</td>
              <td class="num">{{ took(r.started_at, r.finished_at) }}</td>
              <td><span class="sr">{{ r.ok ? 'Finished' : 'Failed' }}: </span>{{ r.summary }}</td>
            </tr>
          </tbody>
        </table>
      </section>
      <p class="hint">
        Backups and the damage check can also be run from a script:
        <span class="mono">POST /api/admin/backup</span>, <span class="mono">/api/admin/scrub</span> and
        <span class="mono">/api/admin/gc</span>. Those runs show up here too.
      </p>
    </template>
  </div>
</template>

<style scoped>
.maint[aria-busy='true'] { cursor: progress; }
.loading { color: var(--muted); font-size: var(--step--1); }
.tasks { display: grid; gap: 0.5rem; margin: 0 0 1.4rem; padding: 0; list-style: none; }
.tasks li { padding: 0.7rem 0.85rem; border: 1px solid var(--line); border-radius: var(--r-chip); font-size: var(--step--1); }
.tasks li.on { border-color: var(--muted); background: var(--raised); }
.top { display: flex; gap: 0.9rem; align-items: flex-start; }
.top .btn { flex: none; min-width: 6rem; }
.what { flex: 1; min-width: 0; display: grid; gap: 0.2rem; }
.what small { color: var(--muted); font-size: inherit; line-height: 1.45; }
.live { margin-top: 0.6rem; display: grid; gap: 0.3rem; }
.live small, .last small { color: var(--muted); font-size: inherit; }
.bar { position: relative; height: 4px; border-radius: 2px; background: var(--line); overflow: hidden; }
.bar span { position: absolute; inset: 0 auto 0 0; border-radius: 2px; background: var(--accent); transition: width 0.4s ease; }
.bar.unknown span { width: 30%; animation: sweep 1.4s ease-in-out infinite; }
@keyframes sweep { from { left: -30%; } to { left: 100%; } }
.last { display: flex; gap: 0.5rem; align-items: baseline; margin: 0.55rem 0 0; overflow-wrap: anywhere; }
.last.never { color: var(--muted); }
.dot { flex: none; width: 0.5rem; height: 0.5rem; border-radius: 50%; background: var(--ok); }
.bad .dot { background: var(--danger); }
.history h3 { margin: 0 0 0.4rem; }
table { width: 100%; border-collapse: collapse; font-size: var(--step--1); }
th { text-align: left; font-weight: 500; color: var(--muted); padding: 0.3rem 0.6rem 0.3rem 0; border-bottom: 1px solid var(--line); }
td { padding: 0.4rem 0.6rem 0.4rem 0; border-bottom: 1px solid color-mix(in srgb, var(--line) 60%, transparent); vertical-align: top; overflow-wrap: anywhere; }
td.num { white-space: nowrap; color: var(--muted); font-variant-numeric: tabular-nums; }
tr.bad td:last-child { color: var(--danger); }
.hint { margin: 1rem 0 0; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
@media (max-width: 560px) {
  th:nth-child(3), td:nth-child(3) { display: none; }
  .top { flex-direction: column; }
}
@media (prefers-reduced-motion: reduce) {
  .bar.unknown span { animation: none; width: 100%; opacity: 0.35; }
  .bar span { transition: none; }
}
</style>
