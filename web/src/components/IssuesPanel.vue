<script setup lang="ts">
// Settings → Issues: what went wrong opening or processing the library's files, grouped by kind, with a way to try
// again or put it out of sight.
import { onMounted, onUnmounted, reactive, ref } from 'vue'
import { api, ApiError, type Issue, type IssueGroup, type IssuePick, type Issues } from '../api'
import { confirmDialog, showError, showOk } from '../ui'

const emit = defineEmits<{
  unauthorized: []
  pack: [{ id: string; name: string }]
  open: [string]
  section: [string]
}>()

const data = ref<Issues | null>(null)
/** Groups showing every item, not only the first few. */
const expanded = reactive(new Set<string>())
/** What's being done right now: an issue's id, or `all:<kind>`. */
const busy = ref<string | null>(null)
const FEW = 5
let timer: ReturnType<typeof setTimeout> | undefined
let gone = false

function failed(e: unknown, what: string) {
  if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  showError(`${what}: ${(e as Error).message}`)
}

async function load() {
  clearTimeout(timer)
  try {
    data.value = await api.issues()
  } catch (e) {
    failed(e, "Couldn't load Issues")
  }
  // Retries take a while to land; keep the list current while the page is open.
  if (!gone) timer = setTimeout(load, 10_000)
}
onMounted(load)
onUnmounted(() => {
  gone = true
  clearTimeout(timer)
})

function shown(g: IssueGroup): Issue[] {
  return expanded.has(g.kind) ? g.items : g.items.slice(0, FEW)
}

async function act(what: 'retry' | 'dismiss', pick: IssuePick, key: string) {
  busy.value = key
  try {
    if (what === 'retry') {
      const { started } = await api.retryIssues(pick)
      showOk(started === 1 ? 'Started again.' : `Started ${started} things again.`)
    } else {
      await api.dismissIssues(pick)
    }
  } catch (e) {
    failed(e, what === 'retry' ? "Couldn't retry" : "Couldn't dismiss")
  } finally {
    busy.value = null
  }
  await load()
}

async function dismissAll(g: IssueGroup) {
  const ok = await confirmDialog({
    title: `Dismiss all ${g.total}?`,
    body: `“${g.title}” is cleared. Anything that goes wrong again is listed again.`,
    confirm: 'Dismiss all',
  })
  if (ok) await act('dismiss', { kind: g.kind }, `all:${g.kind}`)
}

const fmt = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'short' })
function when(at: string | null): string {
  if (!at) return ''
  const secs = (Date.now() - Date.parse(at)) / 1000
  if (secs < 60) return 'just now'
  if (secs < 3600) return `${Math.round(secs / 60)} min ago`
  if (secs < 86400) return `${Math.round(secs / 3600)} h ago`
  return fmt.format(new Date(at))
}

/** Where the issue is, when the title doesn't already say. */
function where(i: Issue): string | null {
  if (i.pack_name && i.pack_name !== i.title) return i.pack_name
  if (i.location_name && i.location_name !== i.title) return i.location_name
  return null
}

function openTitle(i: Issue) {
  if (i.asset_id) emit('open', i.asset_id)
  else if (i.pack_id) emit('pack', { id: i.pack_id, name: i.pack_name ?? i.title })
  else if (i.location_id != null) emit('section', 'locations')
}
function openWhere(i: Issue) {
  if (i.pack_id && i.pack_name !== i.title) emit('pack', { id: i.pack_id, name: i.pack_name ?? '' })
  else emit('section', 'locations')
}
</script>

<template>
  <div class="issues" :aria-busy="!data">
    <p v-if="!data" class="loading">Loading…</p>
    <p v-else-if="!data.groups.length" class="none" data-testid="issues-none">
      <span class="dot" aria-hidden="true" />Nothing has gone wrong.
    </p>
    <template v-else>
      <section v-for="g in data.groups" :key="g.kind" class="group" :data-testid="`issues-${g.kind}`">
        <header>
          <div class="what">
            <h3>{{ g.title }} <span class="count">{{ g.total }}</span></h3>
            <small>{{ g.blurb }}</small>
          </div>
          <div class="all">
            <button
              v-if="g.retry && g.items.some((i) => i.retry) && g.total > 1"
              type="button"
              class="btn"
              :disabled="busy !== null"
              @click="act('retry', { kind: g.kind }, `all:${g.kind}`)"
            >
              {{ g.retry }} all
            </button>
            <button v-if="g.dismiss && g.total > 1" type="button" class="btn ghost" :disabled="busy !== null" @click="dismissAll(g)">
              Dismiss all
            </button>
          </div>
        </header>
        <ul>
          <li v-for="i in shown(g)" :key="i.id">
            <div class="item">
              <button
                v-if="i.asset_id || i.pack_id || i.location_id != null"
                type="button"
                class="link title"
                :title="i.asset_id ? 'Open it' : i.pack_id ? 'Show the pack' : 'Library locations'"
                @click="openTitle(i)"
              >
                {{ i.title }}
              </button>
              <strong v-else class="title">{{ i.title }}</strong>
              <small class="meta">
                <template v-if="where(i)">
                  in <button type="button" class="link" @click="openWhere(i)">{{ where(i) }}</button> ·
                </template>
                {{ when(i.at) }}<template v-if="i.count > 1"> · {{ i.count }} times</template>
              </small>
              <p v-if="i.message" class="msg">{{ i.message }}</p>
            </div>
            <div class="acts">
              <button
                v-if="g.retry && i.retry"
                type="button"
                class="btn"
                :disabled="busy !== null"
                @click="act('retry', { ids: [i.id] }, i.id)"
              >
                {{ busy === i.id ? 'Starting…' : g.retry }}
              </button>
              <button
                v-if="g.dismiss"
                type="button"
                class="btn ghost"
                :disabled="busy !== null"
                :aria-label="`Dismiss ${i.title}`"
                @click="act('dismiss', { ids: [i.id] }, i.id)"
              >
                Dismiss
              </button>
            </div>
          </li>
        </ul>
        <p v-if="g.items.length > FEW || g.total > g.items.length" class="more">
          <button v-if="g.items.length > FEW" type="button" class="link" @click="expanded.has(g.kind) ? expanded.delete(g.kind) : expanded.add(g.kind)">
            {{ expanded.has(g.kind) ? 'Show fewer' : `Show all ${g.items.length}` }}
          </button>
          <span v-if="g.total > g.items.length && (expanded.has(g.kind) || g.items.length <= FEW)">
            {{ g.total - g.items.length }} more aren’t listed.
          </span>
        </p>
      </section>
    </template>
  </div>
</template>

<style scoped>
.issues[aria-busy='true'] { cursor: progress; }
.loading { color: var(--muted); font-size: var(--step--1); }
.none { display: flex; gap: 0.5rem; align-items: baseline; font-size: var(--step--1); margin: 0; }
.dot { flex: none; width: 0.5rem; height: 0.5rem; border-radius: 50%; background: var(--ok); }
.group { margin: 0 0 1.4rem; }
header { display: flex; gap: 0.9rem; align-items: flex-start; margin-bottom: 0.5rem; }
.what { flex: 1; min-width: 0; display: grid; gap: 0.2rem; }
.what h3 { margin: 0; display: flex; gap: 0.5rem; align-items: baseline; }
.what small { color: var(--muted); font-size: var(--step--1); line-height: 1.45; }
.count {
  font-size: var(--step--1); font-weight: 500; font-variant-numeric: tabular-nums;
  padding: 0 0.45rem; border-radius: 999px; background: color-mix(in srgb, var(--danger) 14%, transparent); color: var(--danger);
}
.all { flex: none; display: flex; gap: 0.4rem; }
ul { display: grid; gap: 0.4rem; margin: 0; padding: 0; list-style: none; }
li {
  display: flex; gap: 0.9rem; align-items: flex-start;
  padding: 0.6rem 0.85rem; border: 1px solid var(--line); border-radius: var(--r-chip); font-size: var(--step--1);
}
.item { flex: 1; min-width: 0; display: grid; gap: 0.15rem; }
.title { font-weight: 500; overflow-wrap: anywhere; text-align: left; justify-self: start; }
.meta { color: var(--muted); font-size: inherit; }
.msg {
  margin: 0.25rem 0 0; color: var(--muted); font-family: var(--mono, ui-monospace, monospace); font-size: 0.92em;
  line-height: 1.45; overflow-wrap: anywhere; white-space: pre-wrap; max-height: 7.5em; overflow: auto;
}
.acts { flex: none; display: flex; gap: 0.4rem; }
.link {
  padding: 0; border: 0; background: none; color: inherit; font: inherit; cursor: pointer;
  text-decoration: underline; text-decoration-color: color-mix(in srgb, currentColor 35%, transparent); text-underline-offset: 2px;
}
.link:hover { text-decoration-color: currentColor; }
.more { margin: 0.5rem 0 0; display: flex; gap: 0.8rem; font-size: var(--step--1); color: var(--muted); }
@media (max-width: 560px) {
  header, li { flex-direction: column; }
}
</style>
