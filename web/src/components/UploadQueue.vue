<script setup lang="ts">
// The import queue: packs upload one after another (each with several parallel requests inside), with live
// speed/ETA, cancel and retry. Each import is remembered in the browser, so a reload shows it again as interrupted
// and carries it on: straight from the dropped folder when the browser kept hold of it, or once the same files are
// picked or dropped again (the server resumes by the list of files). It folds down to a small pill out of the way.
import { computed, markRaw, onBeforeUnmount, onMounted, ref } from 'vue'
import { ApiError, type Pack } from '../api'
import { filesFromHandle, groupsFromPicked, signature, type Group, type UploadFile } from '../drop'
import { humanBytes } from '../format'
import { prefs } from '../prefs'
import { abandonUpload, AlreadyImported, splitPerSubfolder, uploadPackResumable } from '../uploader'
import { forget, loadSaved, newKey, save, type Saved } from '../uploadStore'
import { formatNumber, t } from '../i18n'

type State = 'queued' | 'uploading' | 'done' | 'failed' | 'skipped' | 'interrupted'
interface Job {
  id: number
  key: string
  name: string
  sig: string
  folder: boolean
  root?: string
  handle?: FileSystemHandle
  /** Set once the handle couldn't be read, so Resume asks for the files instead. */
  handleFailed?: boolean
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
  /** Whether the browser is remembering it, so a reload loses nothing. */
  saved?: boolean
}

type PermissionHandle = FileSystemHandle & {
  queryPermission?: (o: { mode: 'read' }) => Promise<PermissionState>
  requestPermission?: (o: { mode: 'read' }) => Promise<PermissionState>
}

const emit = defineEmits<{ created: [Pack]; unauthorized: [] }>()
const jobs = ref<Job[]>([])
let seq = 0

const active = computed(() => jobs.value.filter((j) => j.state === 'queued' || j.state === 'uploading'))
const interrupted = computed(() => jobs.value.filter((j) => j.state === 'interrupted'))
const stopped = computed(() => jobs.value.filter((j) => j.state === 'interrupted' || j.state === 'failed'))
const totals = computed(() => {
  const list = active.value
  const sent = list.reduce((n, j) => n + j.sent, 0)
  const total = list.reduce((n, j) => n + j.total, 0)
  return {
    count: list.length,
    sent,
    total,
    percent: total ? Math.floor((sent / total) * 100) : 0,
    rate: list.find((j) => j.state === 'uploading')?.rate ?? 0,
  }
})

function job(id: number) {
  return jobs.value.find((j) => j.id === id)
}

function remember(j: Job) {
  const record: Saved = {
    key: j.key,
    name: j.name,
    sig: j.sig,
    folder: j.folder,
    root: j.root,
    handle: j.handle,
    force: j.force,
    sessionId: j.sessionId,
    sent: j.sent,
    total: j.total,
    filesTotal: j.filesTotal,
  }
  const id = j.id
  void save(record).then((ok) => {
    const live = job(id)
    if (live) live.saved = ok
  })
}

function enqueue(groups: Group[]) {
  for (const g of groups) {
    if (!g.files.length) continue
    const sig = signature(g.files)
    // The same files as an import a reload interrupted carry that one on.
    const old = jobs.value.find((j) => j.state === 'interrupted' && j.sig === sig)
    if (old) {
      if (g.handle) {
        old.handle = markRaw(g.handle)
        old.handleFailed = false
        remember(old)
      }
      start(old, g.files)
      continue
    }
    jobs.value.push({
      id: ++seq,
      key: newKey(),
      name: g.name,
      sig,
      folder: g.files.some((f) => f.path.includes('/')),
      root: g.root,
      handle: g.handle && markRaw(g.handle),
      files: markRaw(g.files),
      state: 'queued',
      sent: 0,
      total: g.files.reduce((n, f) => n + f.file.size, 0),
      filesDone: 0,
      filesTotal: g.files.length,
      resumed: 0,
      rate: 0,
    })
    remember(jobs.value[jobs.value.length - 1])
  }
  pump()
}
defineExpose({ enqueue })

/** Queue an interrupted import again with its files back in hand. */
function start(j: Job, files: UploadFile[]) {
  j.files = markRaw(files)
  j.state = 'queued'
  j.error = undefined
  pump()
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
  let lastSaved = lastT
  try {
    // Folders are named after themselves; for a lone file or archive the server picks a name from its contents.
    const pack = await uploadPackResumable(
      next.files,
      next.folder ? next.name : undefined,
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
        if (now - lastSaved > 5000) {
          lastSaved = now
          remember(j)
        }
      },
      next.controller.signal,
      {
        force: next.force,
        onSession: (sid) => {
          const j = job(id)
          if (!j) return
          j.sessionId = sid
          remember(j)
        },
      },
    )
    const j = job(id)
    if (j) {
      j.state = 'done'
      void forget(j.key)
      setTimeout(() => (jobs.value = jobs.value.filter((x) => x.id !== id)), 5000)
    }
    emit('created', pack)
  } catch (e) {
    const j = job(id)
    if (j && e instanceof AlreadyImported) {
      j.state = 'skipped'
      j.existing = e.pack.name
      void forget(j.key)
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
  void forget(j.key)
  if (j.state !== 'queued' && j.state !== 'skipped' && j.sessionId) await abandonUpload(j.sessionId)
  pump()
}

function retry(j: Job) {
  j.state = 'queued'
  pump()
}

function importAgain(j: Job) {
  j.force = true
  remember(j)
  retry(j)
}

// ------------------------------------------------------------------ carrying on after a reload

/** Read an interrupted import's files again from the folder or file it was dropped as. */
async function resumeFromHandle(j: Job, ask: boolean): Promise<boolean> {
  const handle = j.handle as PermissionHandle | undefined
  if (!handle) return false
  try {
    let permission = (await handle.queryPermission?.({ mode: 'read' })) ?? 'granted'
    if (permission !== 'granted' && ask) permission = (await handle.requestPermission?.({ mode: 'read' })) ?? 'denied'
    if (permission !== 'granted') {
      if (ask) j.error = t('upload-permission-denied')
      return false
    }
    const files = await filesFromHandle(handle)
    const groups = j.root ? splitPerSubfolder(j.root, files) : [{ name: j.name, files }]
    const same = groups.find((g) => signature(g.files) === j.sig)
    if (!same) {
      j.error = t('upload-not-same')
      return false
    }
    start(j, same.files)
    return true
  } catch (e) {
    if (ask) j.error = (e as Error).message
    return false
  }
}

const folderPick = ref<HTMLInputElement | null>(null)
const filePick = ref<HTMLInputElement | null>(null)
/** The import whose Resume opened the picker, to say so there if the files picked aren't its own. */
let pickingFor = 0

async function resume(j: Job) {
  if (j.handle && !j.handleFailed) {
    if (await resumeFromHandle(j, true)) return
    j.handleFailed = true
  }
  pickingFor = j.id
  ;(j.folder ? folderPick : filePick).value?.click()
}

async function resumeAll() {
  for (const j of [...interrupted.value]) {
    if (j.handle && !j.handleFailed && !(await resumeFromHandle(j, true))) j.handleFailed = true
  }
  const left = interrupted.value[0]
  if (left) {
    pickingFor = left.id
    ;(left.folder ? folderPick : filePick).value?.click()
  }
}

/** Files picked to carry imports on: every interrupted import they make up starts again. */
function onRepick(e: Event) {
  const input = e.target as HTMLInputElement
  const picked = [...(input.files ?? [])]
  input.value = ''
  if (!picked.length) return
  const groups = groupsFromPicked(picked)
  const root = groups.length === 1 && groups[0].files[0]?.path.includes('/') ? groups[0] : null
  if (root) groups.push(...splitPerSubfolder(root.name, root.files))
  let found = 0
  for (const g of groups) {
    const sig = signature(g.files)
    const j = interrupted.value.find((x) => x.sig === sig)
    if (j) {
      start(j, g.files)
      found++
    }
  }
  const asked = job(pickingFor) ?? (interrupted.value.length === 1 ? interrupted.value[0] : undefined)
  if (!found && asked) asked.error = t('upload-not-same')
}

onMounted(async () => {
  const saved = await loadSaved()
  for (const s of saved) {
    if (jobs.value.some((j) => j.key === s.key)) continue
    jobs.value.push({
      id: ++seq,
      key: s.key,
      name: s.name,
      sig: s.sig,
      folder: s.folder,
      root: s.root,
      handle: s.handle && markRaw(s.handle),
      files: [],
      state: 'interrupted',
      sent: s.sent,
      total: s.total,
      filesDone: 0,
      filesTotal: s.filesTotal,
      resumed: 0,
      rate: 0,
      sessionId: s.sessionId,
      force: s.force,
      saved: true,
    })
  }
  // Where the browser still lets us read the dropped folders, carry on without asking.
  for (const j of [...interrupted.value]) if (j.handle) await resumeFromHandle(j, false)
})

function eta(j: Job): string {
  if (!j.rate || j.state !== 'uploading') return ''
  const s = (j.total - j.sent) / j.rate
  if (!Number.isFinite(s)) return ''
  if (s < 60) return t('upload-eta-seconds', { s: Math.ceil(s) })
  if (s < 3600) return t('upload-eta-minutes', { min: Math.ceil(s / 60) })
  return t('upload-eta-hours', { h: formatNumber(s / 3600, { minimumFractionDigits: 1, maximumFractionDigits: 1 }) })
}

/** Only an import the browser couldn't remember would be lost by leaving, so only then is there a warning. */
function warnOnClose(e: BeforeUnloadEvent) {
  if (active.value.some((j) => !j.saved)) {
    e.preventDefault()
  }
}
onMounted(() => window.addEventListener('beforeunload', warnOnClose))
onBeforeUnmount(() => window.removeEventListener('beforeunload', warnOnClose))

const pillText = computed(() => {
  if (totals.value.count) return t('upload-pill', { count: totals.value.count, percent: totals.value.percent })
  if (stopped.value.length) return t('upload-pill-stopped', { count: stopped.value.length })
  return t('upload-queue')
})
</script>

<template>
  <input ref="folderPick" type="file" webkitdirectory multiple hidden @change="onRepick" />
  <input ref="filePick" type="file" multiple hidden @change="onRepick" />
  <Transition name="q" mode="out-in">
  <button
    v-if="jobs.length && prefs.uploadsCollapsed"
    key="pill"
    type="button"
    class="pill"
    :class="{ attention: !totals.count && stopped.length }"
    :title="t('upload-expand')"
    :aria-label="`${pillText}. ${t('upload-expand')}`"
    @click="prefs.uploadsCollapsed = false"
  >
    <span class="row">
      <span>{{ pillText }}</span>
      <span v-if="totals.rate" class="mono meta">{{ t('upload-rate', { size: humanBytes(totals.rate) }) }}</span>
      <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="m4 10 4-4 4 4" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
    </span>
    <span v-if="totals.count" class="bar"><span :style="{ width: `${totals.percent}%` }" /></span>
  </button>
  <section v-else-if="jobs.length" key="queue" class="queue" :aria-label="t('upload-queue')" aria-live="polite">
    <header>
      <strong>{{ totals.count ? t('upload-importing', { count: totals.count }) : t('upload-queue') }}</strong>
      <span v-if="totals.count > 1" class="mono">{{ t('upload-bytes-of', { sent: humanBytes(totals.sent), total: humanBytes(totals.total) }) }}</span>
      <span v-if="totals.count > 1 && totals.rate" class="mono">{{ t('upload-rate', { size: humanBytes(totals.rate) }) }}</span>
      <span class="grow" />
      <button v-if="interrupted.length > 1" type="button" class="btn" @click="resumeAll">{{ t('upload-resume-all') }}</button>
      <button type="button" class="btn ghost icon" :title="t('upload-minimize')" :aria-label="t('upload-minimize')" @click="prefs.uploadsCollapsed = true">
        <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="m4 6 4 4 4-4" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" /></svg>
      </button>
    </header>
    <TransitionGroup tag="ol" name="job">
      <li v-for="j in jobs" :key="j.id" :class="j.state">
        <div class="row">
          <span class="name" :title="j.name">{{ j.name }}</span>
          <span v-if="j.state === 'queued'" class="meta">{{ t('upload-waiting') }}</span>
          <span v-else-if="j.state === 'uploading'" class="meta mono">
            {{ humanBytes(j.sent) }} / {{ humanBytes(j.total) }}
          </span>
          <span v-else-if="j.state === 'done'" class="meta ok donemsg">
            <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <circle class="ring" cx="8" cy="8" r="6.5" /><path class="tick" d="m5 8.2 2 2 4-4.4" />
            </svg>
            {{ t('upload-unpacking') }}
          </span>
          <span v-else-if="j.state === 'skipped'" class="meta">{{ t('upload-already') }}</span>
          <span v-else-if="j.state === 'interrupted'" class="meta">{{ t('upload-interrupted') }}</span>
          <span v-else class="meta err">{{ t('upload-stopped') }}</span>
        </div>
        <div v-if="j.state === 'uploading'" class="bar"><span :style="{ width: `${j.total ? (j.sent / j.total) * 100 : 0}%` }" /></div>
        <div v-if="j.state === 'interrupted'" class="bar still"><span :style="{ width: `${j.total ? (j.sent / j.total) * 100 : 0}%` }" /></div>
        <div v-if="j.state === 'uploading'" class="sub">
          <span>{{ t('upload-files-of', { done: j.filesDone, total: j.filesTotal }) }}</span>
          <span v-if="j.rate" class="mono">{{ t('upload-rate', { size: humanBytes(j.rate) }) }}</span>
          <span>{{ eta(j) }}</span>
          <span v-if="j.resumed" class="ok">{{ t('upload-resumed', { size: humanBytes(j.resumed) }) }}</span>
        </div>
        <div v-if="j.state === 'interrupted'" class="sub">
          <span class="mono">{{ t('upload-bytes-of', { sent: humanBytes(j.sent), total: humanBytes(j.total) }) }}</span>
        </div>
        <p v-if="j.state === 'interrupted' && (!j.handle || j.handleFailed)" class="msg hint">
          {{ j.folder ? t('upload-resume-hint-folder') : t('upload-resume-hint-file') }}
        </p>
        <p v-if="j.state === 'skipped'" class="msg">{{ t('upload-skipped', { name: j.existing ?? '' }) }}</p>
        <p v-if="j.error" class="err msg">{{ j.state === 'failed' ? t('upload-error', { error: j.error }) : j.error }}</p>
        <div v-if="j.state !== 'done'" class="actions">
          <button v-if="j.state === 'interrupted'" type="button" class="btn" @click="resume(j)">{{ t('upload-resume') }}</button>
          <button v-if="j.state === 'failed'" type="button" class="btn" @click="retry(j)">{{ t('upload-retry') }}</button>
          <button v-if="j.state === 'skipped'" type="button" class="btn" @click="importAgain(j)">{{ t('upload-import-again') }}</button>
          <button type="button" class="btn ghost" @click="cancel(j)">
            {{ j.state === 'queued' || j.state === 'skipped' ? t('upload-dismiss') : t('upload-cancel') }}
          </button>
        </div>
      </li>
    </TransitionGroup>
  </section>
  </Transition>
</template>

<style scoped>
.queue {
  position: fixed; right: 1rem; bottom: 1rem; z-index: 30; width: min(400px, calc(100vw - 2rem));
  max-height: 60vh; overflow-y: auto; overflow-x: hidden;
  background: var(--raised); border: 1px solid var(--line); border-radius: var(--r-tile);
  font-size: var(--step--1); box-shadow: var(--elev-3);
}
/* Folded down: a small pill in the corner with how far the imports have got. */
.pill {
  position: fixed; right: 1rem; bottom: 1rem; z-index: 30; display: flex; flex-direction: column; gap: 0.3rem;
  min-width: 12rem; max-width: calc(100vw - 2rem); padding: 0.45rem 0.75rem; cursor: pointer;
  background: var(--raised); color: var(--text); border: 1px solid var(--line); border-radius: 999px;
  font: inherit; font-size: var(--step--1); box-shadow: var(--elev-2); text-align: left;
}
.pill:hover { border-color: var(--muted); }
.pill.attention { border-color: var(--danger); }
.pill .row { align-items: center; }
.pill svg { flex: none; color: var(--muted); }
.q-enter-active { transition: opacity var(--dur-2) var(--ease-out), translate var(--dur-3) var(--ease-pop); }
.q-leave-active { transition: opacity var(--dur-2) var(--ease-out), translate var(--dur-2) var(--ease-out); }
.q-enter-from, .q-leave-to { opacity: 0; translate: 0 16px; }
/* Finished jobs slide away and the rest close up behind them. */
.job-enter-active { transition: opacity var(--dur-2) var(--ease-out), translate var(--dur-3) var(--ease-out); }
.job-leave-active { transition: opacity var(--dur-2) var(--ease-out), translate var(--dur-2) var(--ease-out); position: absolute; width: 100%; }
.job-enter-from { opacity: 0; translate: 0 -8px; }
.job-leave-to { opacity: 0; translate: 40px 0; }
.job-move { transition: translate var(--dur-3) var(--ease-out); }
ol { position: relative; }
.donemsg { display: inline-flex; align-items: center; gap: 0.35rem; }
.donemsg .ring { stroke-dasharray: 41; stroke-dashoffset: 41; animation: draw 0.45s var(--ease-out) forwards; }
.donemsg .tick { stroke-dasharray: 10; stroke-dashoffset: 10; animation: draw 0.3s 0.35s var(--ease-out) forwards; }
@keyframes draw { to { stroke-dashoffset: 0; } }
li.done { animation: done-glow 1.2s var(--ease-out); }
@keyframes done-glow { from { background: color-mix(in srgb, var(--ok) 12%, transparent); } }
header {
  position: sticky; top: 0; z-index: 1; display: flex; gap: 0.75rem; align-items: center;
  padding: 0.4rem 0.4rem 0.4rem 0.8rem; border-bottom: 1px solid var(--line); background: var(--raised);
}
header .mono { color: var(--muted); }
header .grow { flex: 1; }
header .btn:not(.icon) { padding: 0.25em 0.7em; }
ol { list-style: none; margin: 0; padding: 0; }
li { padding: 0.6rem 0.8rem; display: flex; flex-direction: column; gap: 0.3rem; }
li + li { border-top: 1px solid var(--line); }
.row { display: flex; justify-content: space-between; gap: 0.75rem; }
.name { font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.meta { color: var(--muted); white-space: nowrap; }
.bar { height: 3px; background: var(--line); border-radius: 2px; overflow: hidden; }
.bar span { position: relative; display: block; height: 100%; background: var(--k-texture); overflow: hidden; transition: width 0.3s var(--ease-out); }
.bar span::after {
  content: ''; position: absolute; inset: 0; translate: -100% 0;
  background: linear-gradient(90deg, transparent, color-mix(in srgb, white 45%, transparent), transparent);
  animation: sheen 1.6s var(--ease-in-out) infinite;
}
.bar.still span { background: var(--muted); }
.bar.still span::after { display: none; }
@keyframes sheen { to { translate: 100% 0; } }
.sub { display: flex; flex-wrap: wrap; gap: 0.2rem 0.9rem; color: var(--muted); font-size: 0.75rem; }
.ok { color: var(--ok); }
.err { color: var(--danger); }
li.skipped .msg, .msg.hint { color: var(--muted); }
.msg { margin: 0; font-size: 0.78rem; }
.actions { display: flex; gap: 0.4rem; }
.actions .btn { padding: 0.25em 0.7em; }
</style>
