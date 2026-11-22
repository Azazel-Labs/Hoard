<script setup lang="ts">
// The Settings page: a list of pages down the side, one open at a time (its id is in the address, #settings/<id>).
// Appearance opens first: (for an admin) the server's name, then this browser's light and dark themes, saved here.
// Dragon is everyone's dragon hoard; Library is the hoard in numbers; the rest are the server's: Blender and ffmpeg
// (downloaded if need be), the AI service that tags packs, what happens to Unreal packages, and the server's switches
// and limits. Those are saved on the server and in effect at once. Anything not at its default has a turn-back arrow
// beside it that resets it.
// Each page is a panel of its own (SettingsPageBody); they share the settings, saving, and the options that save on
// their own (settingsContext). The box above the list finds settings: the pages with settings are shown one after
// another, with only their sections that have every word typed.
import { computed, nextTick, onBeforeUnmount, onMounted, provide, reactive, ref, watch } from 'vue'
import { ApiError, api, type Role, type Settings, type SettingsPatch, type Stats } from '../api'
import { isDesktop, may, refreshAuth } from '../auth'
import { formatNumber, t } from '../i18n'
import { formatMiB } from '../sizes'
import { showError, showOk } from '../ui'
import SettingsGlyph from './SettingsGlyph.vue'
import SettingsPageBody from './SettingsPageBody.vue'
import { type AnyOpt, type OptKey, type Opts, SETTINGS } from './settingsContext'

const props = defineProps<{ section: string; stats: Stats | null }>()
const emit = defineEmits<{
  unauthorized: []
  'update:section': [string]
  /** From the Library page: show these in the library. */
  tag: [string]
  pack: [{ id: string; name: string }]
  open: [string]
  archive: []
  /** Library locations changed: the stats need a refresh. */
  changed: []
}>()
const s = ref<Settings | null>(null)

// Labels and blurbs are getters, so they're in the language of the moment. `sep` starts a group in the list: what's
// in the library, how things get into it, who gets in, and how the server is doing.
const ALL_PAGES: { id: string; readonly label: string; readonly blurb: string; need?: Role; sep?: boolean; server?: boolean }[] = [
  { id: 'appearance', get label() { return t('settings-page-appearance') }, get blurb() { return t('settings-page-appearance-blurb') } },
  { id: 'dragon', get label() { return t('settings-page-dragon') }, get blurb() { return t('settings-page-dragon-blurb') } },
  { id: 'library', get label() { return t('settings-page-library') }, get blurb() { return t('settings-page-library-blurb') } },
  { id: 'copies', get label() { return t('settings-page-copies') }, get blurb() { return t('settings-page-copies-blurb') } },
  { id: 'tags', need: 'edit', get label() { return t('settings-page-tags') }, get blurb() { return t('settings-page-tags-blurb') } },
  { id: 'store-files', need: 'admin', get label() { return t('settings-page-store-files') }, get blurb() { return t('settings-page-store-files-blurb') } },
  { id: 'locations', sep: true, need: 'admin', get label() { return t('settings-page-locations') }, get blurb() { return t('settings-page-locations-blurb') } },
  { id: 'tools', need: 'admin', get label() { return t('settings-page-tools') }, get blurb() { return t('settings-page-tools-blurb') } },
  { id: 'ai', need: 'admin', get label() { return t('settings-page-ai') }, get blurb() { return t('settings-page-ai-blurb') } },
  { id: 'search', need: 'admin', get label() { return t('settings-page-search-tab') }, get blurb() { return t('settings-page-search-blurb') } },
  { id: 'adding', need: 'admin', get label() { return t('settings-page-adding') }, get blurb() { return t('settings-page-adding-blurb') } },
  { id: 'downloads', need: 'admin', get label() { return t('settings-page-downloads') }, get blurb() { return t('settings-page-downloads-blurb') } },
  { id: 'account', sep: true, get label() { return t('settings-page-account') }, get blurb() { return t('settings-page-account-blurb') } },
  { id: 'auth', need: 'admin', server: true, get label() { return t('settings-page-auth') }, get blurb() { return t('settings-page-auth-blurb') } },
  { id: 'connection', need: 'admin', server: true, get label() { return t('settings-page-connection') }, get blurb() { return t('settings-page-connection-blurb') } },
  { id: 'performance', sep: true, need: 'admin', get label() { return t('settings-page-performance') }, get blurb() { return t('settings-page-performance-blurb') } },
  { id: 'issues', need: 'admin', get label() { return t('settings-page-issues') }, get blurb() { return t('settings-page-issues-blurb') } },
  { id: 'maintenance', need: 'admin', get label() { return t('settings-page-maintenance') }, get blurb() { return t('settings-page-maintenance-blurb') } },
  { id: 'about', sep: true, get label() { return t('settings-page-about') }, get blurb() { return t('settings-page-about-blurb') } },
]
/** The pages this account may see: an admin sees them all (but the desktop app has no server to set up). */
const PAGES = computed(() => ALL_PAGES.filter((p) => (!p.need || may(p.need)) && !(p.server && isDesktop())))
const current = computed(() => (PAGES.value.some((p) => p.id === props.section) ? props.section : 'appearance'))
const page = computed(() => PAGES.value.find((p) => p.id === current.value)!)
const paneEl = ref<HTMLElement | null>(null)
function go(id: string) {
  query.value = ''
  emit('update:section', id)
  paneEl.value?.scrollTo({ top: 0 })
}

// Finding a setting.
const query = ref('')
const searching = computed(() => query.value.trim() !== '')
/** The pages with settings to find; the others show the library or how it's doing. */
const SEARCHED = ['appearance', 'locations', 'tools', 'ai', 'search', 'adding', 'downloads', 'account', 'auth', 'connection', 'performance', 'maintenance']
const searched = computed(() => PAGES.value.filter((p) => SEARCHED.includes(p.id)))
const resultsEl = ref<HTMLElement | null>(null)
/** Some section has every word. */
const found = ref(true)
/** Lowercase without accents, so "resume" finds "résumé". */
const fold = (text: string) => text.toLocaleLowerCase().normalize('NFD').replace(/\p{M}/gu, '')
const queryWords = computed(() => fold(query.value).split(/\s+/).filter(Boolean))
/** The other pages (the dragon, the library, how it's doing) are found by their name and what they're for, and
 *  shown as a way to them: what's on them is the library, not settings, and too much to show here. */
const pagesFound = computed(() =>
  PAGES.value.filter((p) => {
    if (SEARCHED.includes(p.id)) return false
    const text = fold(`${p.label} ${p.blurb}`)
    return queryWords.value.every((w) => text.includes(w))
  }),
)
/** Show the sections with every word typed (or on a page whose name has them), and the pages with any. */
function filter() {
  const root = resultsEl.value
  if (!root) return
  const words = queryWords.value
  let any = false
  for (const pageEl of root.querySelectorAll<HTMLElement>('[data-found]')) {
    let shown = 0
    for (const section of pageEl.querySelectorAll<HTMLElement>('section')) {
      if (section.parentElement?.closest('section')) continue
      const text = fold(`${pageEl.dataset.label} ${pageEl.dataset.blurb} ${section.textContent}`)
      const hit = words.every((w) => text.includes(w))
      // Only when it changes: the watcher below sees class changes too.
      if (section.classList.contains('unfound') === hit) section.classList.toggle('unfound', !hit)
      if (hit) shown++
    }
    if (pageEl.hidden !== !shown) pageEl.hidden = !shown
    if (shown) any = true
  }
  found.value = any
}
// The pages fill in as what they show arrives, so they're looked through again whenever they change.
let frame = 0
const refilter = () => {
  cancelAnimationFrame(frame)
  frame = requestAnimationFrame(filter)
}
const watcher = new MutationObserver(refilter)
watch(resultsEl, (el) => {
  watcher.disconnect()
  if (el) watcher.observe(el, { childList: true, subtree: true, characterData: true, attributes: true, attributeFilter: ['class'] })
})
watch(query, () => void nextTick(refilter))
const busy = ref(false)

// Server options: each saves on its own when it's entered (Enter, or leaving the field); switches at once.
const opts = reactive<Opts>({ workers: 1, max_upload_mb: 1, max_expand_mb: 1, conversion_cache_mb: 1, backup_keep: 1, backup_copy_to: '', name: '', public_url: '', soundfont: '' })
const OPT_KEYS = Object.keys(opts) as OptKey[]
/** An option's name, for the note that it's saved. */
const optLabel = (k: OptKey): string =>
  ({
    workers: t('settings-workers'),
    max_upload_mb: t('settings-max-upload'),
    max_expand_mb: t('settings-max-expand'),
    conversion_cache_mb: t('settings-conversion-cache'),
    backup_keep: t('settings-backup-keep'),
    backup_copy_to: t('settings-backup-copy-to'),
    name: t('settings-name'),
    public_url: t('settings-public-url'),
    soundfont: t('settings-soundfont'),
  })[k]
const changed = (k: AnyOpt) => !!s.value && s.value.options[k].value !== s.value.options[k].default
function saveOpt(k: OptKey) {
  const cur = s.value
  if (!cur) return
  const was = cur.options[k].value
  let v = opts[k]
  if (typeof v === 'string') v = v.trim()
  else if (!Number.isInteger(v) || v < 1) {
    // Empty or nonsense: put back what's saved.
    ;(opts as Record<OptKey, unknown>)[k] = was
    return
  }
  if (v === was) return
  const shown =
    k === 'max_upload_mb' || k === 'max_expand_mb' || k === 'conversion_cache_mb'
      ? formatMiB(v as number)
      : typeof v === 'number'
        ? formatNumber(v)
        : v || t('settings-the-default')
  void save({ [k]: v } as SettingsPatch, t('settings-option-saved', { name: optLabel(k), value: shown }))
}
const resetOpt = (k: AnyOpt) => save({ [k]: null } as SettingsPatch, t('settings-back-to-default'))

/** The pages' own forms, put back to what's saved whenever the settings come back. */
const takers = new Set<(next: Settings) => void>()
function take(next: Settings) {
  s.value = next
  Object.assign(
    opts,
    Object.fromEntries(OPT_KEYS.map((k) => [k, next.options[k].value])),
  )
  for (const fn of takers) fn(next)
  schedulePoll()
}

async function load() {
  try {
    take(await api.settings())
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) {
      emit('unauthorized')
      return
    }
    showError(t('settings-load-failed', { error: (e as Error).message }))
  }
}

async function save(patch: SettingsPatch, done?: string) {
  busy.value = true
  try {
    take(await api.saveSettings(patch))
    if (done) showOk(done)
    // The top bar, the tab and the sign-in page show the name.
    if ('name' in patch) void refreshAuth()
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError((e as Error).message)
    if (s.value) take(s.value) // put the controls back as they were
  } finally {
    busy.value = false
  }
}

// While a tool is being looked for or downloaded, keep its status line current.
let timer: number | undefined
function schedulePoll() {
  window.clearTimeout(timer)
  const v = s.value
  const busyTool = v && [v.blender, v.unreal, v.ffmpeg, v.clip, v.clap].some((t) => t.state === 'looking' || t.state === 'downloading')
  const learning = !!v && [v.clip, v.clap].some((m) => m.state === 'ready' && m.done < m.total)
  if (busyTool || learning) {
    timer = window.setTimeout(async () => {
      try {
        const next = await api.settings()
        // Don't clobber a path or AI setting someone is typing.
        if (s.value) s.value = { ...s.value, blender: next.blender, unreal: next.unreal, ffmpeg: next.ffmpeg, clip: next.clip, clap: next.clap }
      } catch {
        // Try again on the next tick.
      }
      schedulePoll()
    }, 1500)
  }
}

provide(SETTINGS, {
  s: computed(() => s.value!),
  ready: computed(() => !!s.value),
  busy,
  save,
  take,
  onTake(fn) {
    takers.add(fn)
    if (s.value) fn(s.value)
    onBeforeUnmount(() => takers.delete(fn))
  },
  opts,
  saveOpt,
  resetOpt,
  changed,
  unauthorized: () => emit('unauthorized'),
})

onMounted(() => {
  if (may('admin')) void load()
})
onBeforeUnmount(() => {
  window.clearTimeout(timer)
  watcher.disconnect()
  cancelAnimationFrame(frame)
})
</script>

<template>
  <div class="settings-page">
    <nav class="snav" :aria-label="t('settings-title')">
      <h2>{{ t('settings-title') }}</h2>
      <input
        v-model="query"
        class="field find"
        type="search"
        :placeholder="t('settings-find')"
        :aria-label="t('settings-find')"
        data-testid="settings-find"
        @keydown.esc="query = ''"
      />
      <ul>
        <li v-for="p in PAGES" :key="p.id" :class="{ sep: p.sep }">
          <button type="button" :aria-current="!searching && current === p.id ? 'page' : undefined" @click="go(p.id)">
            <SettingsGlyph class="ico" :page="p.id" />
            <span class="lbl">{{ p.label }}</span>
            <span v-if="p.id === 'ai' && s" class="pip" :data-on="s.llm.enabled" aria-hidden="true" />
          </button>
        </li>
      </ul>
    </nav>

    <div ref="paneEl" class="pane" :aria-busy="busy">
      <div v-if="searching" ref="resultsEl" class="inner" data-testid="settings-found">
        <div v-for="p in pagesFound" :key="p.id" class="found" :data-page="p.id">
          <button type="button" class="found-h" @click="go(p.id)">
            <SettingsGlyph :page="p.id" />
            {{ p.label }}
          </button>
          <p class="what">{{ p.blurb }}</p>
        </div>
        <p v-if="!found && !pagesFound.length" class="loading">{{ t('settings-find-none', { query: query.trim() }) }}</p>
        <div v-for="p in searched" :key="p.id" class="found" data-found :data-label="p.label" :data-blurb="p.blurb" :data-page="p.id">
          <button type="button" class="found-h" @click="go(p.id)">
            <SettingsGlyph :page="p.id" />
            {{ p.label }}
          </button>
          <SettingsPageBody
            :id="p.id"
            :stats="stats"
            @unauthorized="emit('unauthorized')"
            @changed="emit('changed')"
            @section="go"
          />
        </div>
      </div>
      <div v-else class="inner" :class="{ wide: current === 'store-files', roomy: current === 'performance' }">
        <header class="phead">
          <span class="pglyph" aria-hidden="true"><SettingsGlyph :page="page.id" :size="22" /></span>
          <div class="ptext">
            <h2>{{ page.label }}</h2>
            <p class="what">{{ page.blurb }}</p>
          </div>
        </header>
        <SettingsPageBody
          :id="current"
          :stats="stats"
          @unauthorized="emit('unauthorized')"
          @tag="(t) => emit('tag', t)"
          @pack="(p) => emit('pack', p)"
          @open="(id) => emit('open', id)"
          @archive="emit('archive')"
          @changed="emit('changed')"
          @section="go"
        />
      </div>
    </div>
  </div>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
/* A page of its own: the list of pages down the side, the open one beside it (scrolling on its own). */
.settings-page { display: grid; grid-template-columns: 220px minmax(0, 1fr); min-height: 0; }
.snav {
  overflow-y: auto; padding: 1.1rem 0.75rem 2rem; border-right: 1px solid var(--line);
  background: color-mix(in srgb, var(--panel) 60%, transparent);
}
.snav h2 { margin: 0 0 0.8rem 0.6rem; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.snav ul { display: grid; gap: 0.1rem; margin: 0; padding: 0; list-style: none; }
/* The groups (what's in the library, how it gets in, who gets in, how the server is doing) have a rule between. */
.snav li.sep:not(:first-child) { margin-top: 0.45rem; padding-top: 0.45rem; border-top: 1px solid var(--line); }
.snav button {
  position: relative; display: flex; align-items: center; gap: 0.6rem; width: 100%; padding: 0.42rem 0.6rem;
  border: 0; border-radius: 6px; background: none; color: var(--muted); font: inherit; font-size: var(--step--1);
  text-align: left; cursor: pointer; transition: background-color var(--dur-1), color var(--dur-1), box-shadow var(--dur-2);
}
.snav .ico { flex: none; color: var(--faint); transition: color var(--dur-1); }
.snav .lbl { flex: 1; min-width: 0; }
.snav button:hover { color: var(--text); background: color-mix(in srgb, var(--raised) 60%, transparent); }
.snav button:hover .ico { color: var(--muted); }
.snav button[aria-current='page'] { color: var(--text); background: var(--raised); font-weight: 600; box-shadow: var(--elev-1); }
.snav button[aria-current='page'] .ico { color: var(--accent); }
/* The open page is marked with the accent at the list's edge. */
.snav button[aria-current='page']::before {
  content: ''; position: absolute; left: -0.75rem; top: 0.35rem; bottom: 0.35rem; width: 3px; border-radius: 0 3px 3px 0;
  background: var(--accent);
}
.pip { flex: none; width: 0.45rem; height: 0.45rem; border-radius: 50%; background: var(--faint); }
.pip[data-on='true'] { background: var(--ok); box-shadow: 0 0 0 3px color-mix(in srgb, var(--ok) 22%, transparent); }
.pane { overflow-y: auto; scrollbar-gutter: stable; }
.pane[aria-busy='true'] { cursor: progress; }
.inner { max-width: 50rem; padding: 1.6rem 2rem 5rem; }
/* The format wizard shows a file beside its form. */
.inner.wide { max-width: 80rem; }
/* Performance's charts and the table of routes have more to show than a page of settings; its text keeps its width. */
.inner.roomy { max-width: 72rem; }
/* The page's heading: its icon on an accent tile, the anchor the eye lands on first. */
.phead { display: flex; align-items: flex-start; gap: 0.95rem; margin-bottom: 1.5rem; }
.pglyph {
  flex: none; display: grid; place-items: center; width: 2.9rem; height: 2.9rem; border-radius: 12px; color: var(--accent);
  border: 1px solid color-mix(in srgb, var(--accent) 30%, var(--line));
  background: linear-gradient(145deg, color-mix(in srgb, var(--accent) 26%, var(--raised)), color-mix(in srgb, var(--accent) 7%, var(--raised)));
  box-shadow: var(--elev-1);
}
.ptext { min-width: 0; }
.phead h2 { margin: 0 0 0.15rem; font-size: var(--step-2); line-height: 1.2; font-variation-settings: 'CASL' 0.5; }
.phead .what { margin: 0; max-width: 40rem; }
.loading { color: var(--muted); font-size: var(--step--1); }
/* The box that finds settings, above the list. */
.find { width: 100%; margin: 0 0 0.8rem; }
/* What's found: each page's name, then its sections that match. */
.found { display: grid; gap: 1rem; margin-bottom: 1.8rem; }
.found[hidden] { display: none; }
.found-h {
  display: flex; align-items: center; gap: 0.5rem; justify-self: start; padding: 0; border: 0; background: none;
  color: var(--muted); font: inherit; font-size: var(--step--1); font-weight: 600; cursor: pointer;
}
.found-h:hover { color: var(--accent); }
@media (max-width: 47.99rem) {
  .settings-page { grid-template-columns: minmax(0, 1fr); grid-template-rows: auto minmax(0, 1fr); }
  .snav { display: flex; align-items: center; gap: 0.75rem; padding: 0.5rem 0.75rem; overflow-x: auto; border-right: 0; border-bottom: 1px solid var(--line); }
  .snav h2 { display: none; }
  .find { flex: none; width: 11rem; margin: 0; }
  .snav ul { display: flex; gap: 0.25rem; }
  .snav li.sep:not(:first-child) { margin: 0 0 0 0.3rem; padding: 0 0 0 0.55rem; border-top: 0; border-left: 1px solid var(--line); }
  .snav button { white-space: nowrap; min-height: 40px; gap: 0.45rem; }
  .snav button[aria-current='page']::before { inset: auto 0.6rem 0; width: auto; height: 2px; border-radius: 2px 2px 0 0; }
  .inner { padding: 1.1rem 1rem 5rem; }
  .phead { align-items: flex-start; gap: 0.75rem; margin-bottom: 1.1rem; }
  .pglyph { width: 2.5rem; height: 2.5rem; border-radius: 10px; }
}

</style>

<style>
/* The Settings look, shared with the panels its pages show (they only ever appear in here): each section is a card
   on the page, titled a step below the page's heading and a step above its labels. */
.settings-page {
  --card: light-dark(var(--raised), var(--panel));
  /* A chosen option or the row in use, inside a card: tinted with the accent rather than lifted. */
  --card-on: color-mix(in srgb, var(--accent) 8%, var(--card));
  --card-on-line: color-mix(in srgb, var(--accent) 50%, var(--line));
  /* Something set into a card: an example, a fingerprint, a track. */
  --well: color-mix(in srgb, var(--bg) 65%, var(--card));
}
.settings-page .inner section {
  min-width: 0; padding: 1.1rem 1.25rem 1.2rem; border: 1px solid var(--line); border-radius: 10px; background: var(--card);
  box-shadow: var(--elev-1);
}
.settings-page .inner h3 {
  margin: 0; font-size: var(--step-0); font-weight: 650; letter-spacing: -0.005em; text-transform: none; color: var(--text);
}
@media (max-width: 30rem) {
  .settings-page .inner section { padding: 0.95rem 0.9rem 1rem; }
}
/* A section without the words being looked for. */
.settings-page .inner section.unfound { display: none !important; }
/* Options that save the moment they're flipped look like switches: still checkboxes underneath. */
.settings-page .switches input[type='checkbox'],
.settings-page input[type='checkbox'].switch {
  appearance: none; position: relative; flex: none; width: 2.15rem; height: 1.25rem; margin: 0; border-radius: 999px;
  background: color-mix(in srgb, var(--muted) 40%, var(--card)); cursor: pointer;
  transition: background-color var(--dur-2) var(--ease-out);
}
.settings-page .switches input[type='checkbox']::before,
.settings-page input[type='checkbox'].switch::before {
  content: ''; position: absolute; top: 0.16rem; left: 0.16rem; width: 0.93rem; height: 0.93rem; border-radius: 50%;
  background: light-dark(#fff, var(--text)); box-shadow: 0 1px 2px rgb(0 0 0 / 0.25); transition: translate var(--dur-2) var(--ease-pop);
}
.settings-page .switches input[type='checkbox']:checked,
.settings-page input[type='checkbox'].switch:checked { background: var(--accent); }
.settings-page .switches input[type='checkbox']:checked::before,
.settings-page input[type='checkbox'].switch:checked::before { translate: 0.9rem 0; background: light-dark(#fff, var(--bg)); }
.settings-page .switches input[type='checkbox']:disabled,
.settings-page input[type='checkbox'].switch:disabled { opacity: 0.5; cursor: default; }
</style>
