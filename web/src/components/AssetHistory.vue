<script setup lang="ts">
// An asset's history: each change a person, an app, a rule or the AI made to it, grouped by edit, the latest first.
// It's fetched when opened. Someone who edits can put back what any one change changed (which is itself a change).
import { computed, ref, watch } from 'vue'
import { api, type Asset, type Change } from '../api'
import { formatDate, t } from '../i18n'
import { showError, showOk } from '../ui'

const props = defineProps<{ asset: Asset; editor: boolean }>()
const emit = defineEmits<{ updated: [Asset] }>()

const open = ref(false)
const changes = ref<Change[] | null>(null)
const busy = ref<number | null>(null)

async function load() {
  const id = props.asset.id
  try {
    const got = await api.history(id)
    if (props.asset.id === id) changes.value = got
  } catch (e) {
    showError(t('history-load-failed', { error: (e as Error).message }))
  }
}
function toggle(e: Event) {
  open.value = (e.target as HTMLDetailsElement).open
  if (open.value) void load()
}
// Another asset starts closed; an edit to this one shows up while it's open.
watch(
  () => props.asset.id,
  () => {
    changes.value = null
  },
)
watch(
  () => props.asset.updated_at,
  () => {
    if (open.value) void load()
  },
)

/** One edit: the changes made at the same moment by the same one. */
interface Edit { key: string; who: string; source: string; at: string; changes: Change[] }
const edits = computed<Edit[]>(() => {
  const out: Edit[] = []
  for (const c of changes.value ?? []) {
    const last = out[out.length - 1]
    if (last && last.at === c.at && last.source === c.source && last.who === (c.who ?? '')) last.changes.push(c)
    else out.push({ key: String(c.id), who: c.who ?? '', source: c.source, at: c.at, changes: [c] })
  }
  return out
})

function byWhom(e: Edit): string {
  if (e.source === 'ai') return t('history-by-ai')
  if (e.source === 'rule') return t('history-by-rule')
  if (!e.who) return t('history-by-someone')
  if (e.source === 'app') return t('history-by-app', { who: e.who })
  if (e.source === 'mcp') return t('history-by-assistant', { who: e.who })
  return e.who
}
function when(at: string): string {
  const d = new Date(at.replace(' ', 'T') + (/[zZ+]/.test(at.slice(10)) ? '' : 'Z'))
  return Number.isNaN(d.getTime()) ? at : formatDate(d, { dateStyle: 'medium', timeStyle: 'short' })
}

const FIELDS: Record<string, () => string> = {
  name: () => t('asset-name'),
  category: () => t('asset-field-category'),
  description: () => t('history-field-description'),
  license: () => t('asset-field-license'),
  source_url: () => t('asset-field-source'),
  rating: () => t('asset-field-rating'),
}
const short = (s: string) => (s.length > 60 ? s.slice(0, 57).trimEnd() + '…' : s)
const stars = (n: string | null) => (n ? '★'.repeat(Math.min(5, Number(n) || 0)) : '')
/** A tag's source as history keeps it (`ai`, `ai:0.83`): whether a person put it there. */
const byPerson = (s: string | null) => s === 'human'

/** What a change did, in words. */
function describe(c: Change): string {
  if (c.field.startsWith('tag:')) {
    const tag = c.field.slice(4)
    if (!c.old) return t('history-tag-added', { tag })
    if (!c.new) return t('history-tag-removed', { tag })
    if (byPerson(c.new) && !byPerson(c.old)) return t('history-tag-kept', { tag })
    return t('history-tag-changed', { tag })
  }
  if (c.field === 'reviewed') return c.new ? t('history-reviewed') : t('history-unreviewed')
  const field = c.field.startsWith('field:') ? c.field.slice(6) : (FIELDS[c.field]?.() ?? c.field)
  const show = (v: string | null) => (c.field === 'rating' ? stars(v) : short(v ?? ''))
  if (!c.old) return t('history-set', { field, to: show(c.new) })
  if (!c.new) return t('history-cleared', { field, from: show(c.old) })
  return t('history-changed', { field, from: show(c.old), to: show(c.new) })
}
/** The whole of what was there before and after, when the line had to shorten it. */
function full(c: Change): string | undefined {
  const long = (c.old?.length ?? 0) > 60 || (c.new?.length ?? 0) > 60
  return long ? `${c.old ?? ''}\n→\n${c.new ?? ''}` : undefined
}

async function revert(c: Change) {
  busy.value = c.id
  try {
    const asset = await api.revertChange(props.asset.id, c.id)
    emit('updated', asset)
    showOk(t('history-reverted', { change: describe(c) }))
    await load()
  } catch (e) {
    showError(t('history-revert-failed', { error: (e as Error).message }))
  } finally {
    busy.value = null
  }
}
</script>

<template>
  <section class="history">
    <details :open="open" @toggle="toggle">
      <summary><h3>{{ t('history-heading') }}</h3></summary>
      <p v-if="!changes" class="note">{{ t('history-loading') }}</p>
      <p v-else-if="!changes.length" class="note">{{ t('history-none') }}</p>
      <ol v-else class="edits">
        <li v-for="e in edits" :key="e.key" class="edit">
          <p class="by">
            <span class="who" :class="{ 'ai-voice': e.source === 'ai' }">{{ byWhom(e) }}</span>
            <time :datetime="e.at">{{ when(e.at) }}</time>
          </p>
          <ul>
            <li v-for="c in e.changes" :key="c.id" class="change">
              <span class="what" :class="{ 'ai-voice': e.source === 'ai' }" :title="full(c)">{{ describe(c) }}</span>
              <button
                v-if="editor"
                type="button"
                class="linkish small revert"
                :disabled="busy !== null"
                :title="t('history-revert-title')"
                @click="revert(c)"
              >{{ t('history-revert') }}</button>
            </li>
          </ul>
        </li>
      </ol>
    </details>
  </section>
</template>

<style scoped src="./detailSection.css"></style>
<style scoped>
summary { display: flex; align-items: center; gap: 0.4rem; cursor: pointer; list-style: none; }
summary::-webkit-details-marker { display: none; }
summary::before { content: '›'; color: var(--faint); transition: transform 0.15s; }
details[open] > summary::before { transform: rotate(90deg); }
summary h3 { margin: 0; }
details[open] > summary { margin-bottom: 0.5rem; }
.note { margin: 0; color: var(--faint); font-size: var(--step--1); }
.edits, .edits ul { margin: 0; padding: 0; list-style: none; }
.edits { display: grid; gap: 0.7rem; max-height: 24rem; overflow-y: auto; font-size: var(--step--1); }
.by { display: flex; flex-wrap: wrap; gap: 0.2rem 0.6rem; margin: 0 0 0.2rem; color: var(--muted); }
.who { font-weight: 600; color: var(--text); }
time { color: var(--faint); font-variant-numeric: tabular-nums; }
.change { display: flex; align-items: baseline; gap: 0.6rem; padding: 0.1rem 0 0.1rem 0.7rem; border-left: 2px solid var(--line); }
.what { flex: 1; min-width: 0; overflow-wrap: anywhere; }
.revert { flex: none; color: var(--muted); opacity: 0; }
.change:hover .revert, .revert:focus-visible { opacity: 1; }
@media (hover: none) { .revert { opacity: 1; } }
</style>
