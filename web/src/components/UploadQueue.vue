<script setup lang="ts">
// The import queue: packs upload one after another (each with several parallel requests inside), with live
// speed/ETA, cancel and retry. Closing the tab mid-import warns first; re-dropping the same files resumes.
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { ApiError, humanBytes, type Pack, type UploadFile } from '../api'
import { abandonUpload, AlreadyImported, uploadPackResumable } from '../uploader'

type State = 'queued' | 'uploading' | 'done' | 'failed' | 'skipped'
interface Job {
  id: number
  name: string
  files: UploadFile[]
  state: State
  sent: number
  total: number
  filesDone: number
  filesTotal: number
  resumed: number
  rate: number
  error?: string
  controller?: AbortController
  sessionId?: string
  force?: boolean
  existing?: string
}

const emit = defineEmits<{ created: [Pack]; unauthorized: [] }>()
const jobs = ref<Job[]>([])
let seq = 0

const active = computed(() => jobs.value.filter((j) => j.state === 'queued' || j.state === 'uploading'))
const totals = computed(() => {
  const list = active.value
  return {
    count: list.length,
    sent: list.reduce((n, j) => n + j.sent, 0),
    total: list.reduce((n, j) => n + j.total, 0),
    rate: list.find((j) => j.state === 'uploading')?.rate ?? 0,
  }
})

function enqueue(groups: { name: string; files: UploadFile[] }[]) {
  for (const g of groups) {
    if (!g.files.length) continue
    jobs.value.push({
      id: ++seq,
      name: g.name,
      files: g.files,
      state: 'queued',
      sent: 0,
      total: g.files.reduce((n, f) => n + f.file.size, 0),
      filesDone: 0,
      filesTotal: g.files.length,
      resumed: 0,
      rate: 0,
    })
  }
  pump()
}
defineExpose({ enqueue })

function job(id: number) {
  return jobs.value.find((j) => j.id === id)
}

async function pump() {
  if (jobs.value.some((j) => j.state === 'uploading')) return
  const next = jobs.value.find((j) => j.state === 'queued')
  if (!next) return
  const id = next.id
  next.state = 'uploading'
  next.error = undefined
  next.controller = new AbortController()
  // Speed: exponentially smoothed bytes/second.
  let lastT = performance.now()
  let lastSent = 0
  try {
    // Folders are named after themselves; for a lone file or archive the server picks a name from its contents.
    const isFolder = next.files.some((f) => f.path.includes('/'))
    const pack = await uploadPackResumable(
      next.files,
      isFolder ? next.name : undefined,
      (p) => {
        const j = job(id)
        if (!j) return
        const now = performance.now()
        const dt = (now - lastT) / 1000
        if (dt > 0.5) {
          const inst = (p.sent - lastSent) / dt
          j.rate = j.rate ? j.rate * 0.7 + inst * 0.3 : inst
          lastT = now
          lastSent = p.sent
        }
        Object.assign(j, { sent: p.sent, total: p.total, filesDone: p.filesDone, filesTotal: p.filesTotal, resumed: p.resumedBytes })
        if (!lastSent) lastSent = p.resumedBytes
      },
      next.controller.signal,
      { force: next.force, onSession: (sid) => { const j = job(id); if (j) j.sessionId = sid } },
    )
    const j = job(id)
    if (j) {
      j.state = 'done'
      setTimeout(() => (jobs.value = jobs.value.filter((x) => x.id !== id)), 5000)
    }
    emit('created', pack)
  } catch (e) {
    const j = job(id)
    if (j && e instanceof AlreadyImported) {
      j.state = 'skipped'
      j.existing = e.pack.name
    } else if (j && (e as Error).name !== 'AbortError') {
      j.state = 'failed'
      j.error = (e as Error).message
      if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    }
  } finally {
    pump()
  }
}

async function cancel(j: Job) {
  if (j.state === 'uploading') j.controller?.abort()
  jobs.value = jobs.value.filter((x) => x.id !== j.id)
  if (j.state !== 'queued' && j.state !== 'skipped' && j.sessionId) await abandonUpload(j.sessionId)
  pump()
}

function retry(j: Job) {
  j.state = 'queued'
  pump()
}

function importAgain(j: Job) {
  j.force = true
  retry(j)
}

function eta(j: Job): string {
  if (!j.rate || j.state !== 'uploading') return ''
  const s = (j.total - j.sent) / j.rate
  if (!Number.isFinite(s)) return ''
  if (s < 60) return `${Math.ceil(s)} s left`
  if (s < 3600) return `${Math.ceil(s / 60)} min left`
  return `${(s / 3600).toFixed(1)} h left`
}

function warnOnClose(e: BeforeUnloadEvent) {
  if (active.value.length) {
    e.preventDefault()
  }
}
onMounted(() => window.addEventListener('beforeunload', warnOnClose))
onBeforeUnmount(() => window.removeEventListener('beforeunload', warnOnClose))
</script>

<template>
  <section v-if="jobs.length" class="queue" aria-label="Imports" aria-live="polite">
    <header v-if="totals.count > 1">
      <strong>Importing {{ totals.count }} packs</strong>
      <span class="mono">{{ humanBytes(totals.sent) }} of {{ humanBytes(totals.total) }}</span>
      <span v-if="totals.rate" class="mono">{{ humanBytes(totals.rate) }}/s</span>
    </header>
    <ol>
      <li v-for="j in jobs" :key="j.id" :class="j.state">
        <div class="row">
          <span class="name" :title="j.name">{{ j.name }}</span>
          <span v-if="j.state === 'queued'" class="meta">Waiting</span>
          <span v-else-if="j.state === 'uploading'" class="meta mono">
            {{ humanBytes(j.sent) }} / {{ humanBytes(j.total) }}
          </span>
          <span v-else-if="j.state === 'done'" class="meta ok">Uploaded, now unpacking</span>
          <span v-else-if="j.state === 'skipped'" class="meta">Already in the library</span>
          <span v-else class="meta err">Stopped</span>
        </div>
        <div v-if="j.state === 'uploading'" class="bar"><span :style="{ width: `${j.total ? (j.sent / j.total) * 100 : 0}%` }" /></div>
        <div v-if="j.state === 'uploading'" class="sub">
          <span>{{ j.filesDone.toLocaleString() }} of {{ j.filesTotal.toLocaleString() }} files</span>
          <span v-if="j.rate" class="mono">{{ humanBytes(j.rate) }}/s</span>
          <span>{{ eta(j) }}</span>
          <span v-if="j.resumed" class="ok">resumed, {{ humanBytes(j.resumed) }} were already there</span>
        </div>
        <p v-if="j.state === 'skipped'" class="msg">These exact files were already imported as “{{ j.existing }}”, so they were skipped.</p>
        <p v-if="j.error" class="err msg">{{ j.error }}. Your progress is kept: retry, or drop the same files again later.</p>
        <div v-if="j.state !== 'done'" class="actions">
          <button v-if="j.state === 'failed'" type="button" class="btn" @click="retry(j)">Retry</button>
          <button v-if="j.state === 'skipped'" type="button" class="btn" @click="importAgain(j)">Import again anyway</button>
          <button type="button" class="btn ghost" @click="cancel(j)">
            {{ j.state === 'queued' || j.state === 'skipped' ? 'Dismiss' : 'Cancel' }}
          </button>
        </div>
      </li>
    </ol>
  </section>
</template>

<style scoped>
.queue {
  position: fixed; right: 1rem; bottom: 1rem; z-index: 30; width: min(400px, calc(100vw - 2rem));
  max-height: 60vh; overflow-y: auto;
  background: var(--raised); border: 1px solid var(--line); border-radius: var(--r-tile);
  font-size: var(--step--1);
}
header { display: flex; gap: 0.75rem; align-items: baseline; padding: 0.6rem 0.8rem; border-bottom: 1px solid var(--line); }
header .mono { color: var(--muted); }
ol { list-style: none; margin: 0; padding: 0; }
li { padding: 0.6rem 0.8rem; display: flex; flex-direction: column; gap: 0.3rem; }
li + li { border-top: 1px solid var(--line); }
.row { display: flex; justify-content: space-between; gap: 0.75rem; }
.name { font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.meta { color: var(--muted); white-space: nowrap; }
.bar { height: 3px; background: var(--line); border-radius: 2px; overflow: hidden; }
.bar span { display: block; height: 100%; background: var(--k-texture); transition: width 0.3s; }
.sub { display: flex; flex-wrap: wrap; gap: 0.2rem 0.9rem; color: var(--muted); font-size: 0.75rem; }
.ok { color: var(--ok); }
.err { color: var(--danger); }
li.skipped .msg { color: var(--muted); }
.msg { margin: 0; font-size: 0.78rem; }
.actions { display: flex; gap: 0.4rem; }
.actions .btn { padding: 0.25em 0.7em; }
</style>
