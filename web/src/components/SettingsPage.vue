<script setup lang="ts">
// The Settings page: a list of pages down the side, one open at a time (its id is in the address, #settings/<id>).
// General opens first: everyone's dragon hoard, and (for an admin) the server's name. Appearance is this browser's
// (light and dark themes, saved here); Library is the hoard in numbers; the rest are the server's: Blender and ffmpeg
// (downloaded if need be), the AI service that tags packs, what happens to Unreal packages, and the server's switches
// and limits. Those are saved on the server and in effect at once. Anything not at its default has a turn-back arrow
// beside it that resets it.
// Each page is a panel of its own; they share the settings, saving, and the options that save on their own
// (settingsContext).
import { computed, onBeforeUnmount, onMounted, provide, reactive, ref } from 'vue'
import { ApiError, api, type Role, type Settings, type SettingsPatch, type Stats } from '../api'
import { isDesktop, may, refreshAuth } from '../auth'
import { formatNumber, t } from '../i18n'
import { formatMiB } from '../sizes'
import { showError, showOk } from '../ui'
import AccountPanel from './AccountPanel.vue'
import AddingPanel from './AddingPanel.vue'
import AiPanel from './AiPanel.vue'
import AppearancePanel from './AppearancePanel.vue'
import AuthPanel from './AuthPanel.vue'
import CopiesPanel from './CopiesPanel.vue'
import HoardPanel from './HoardPanel.vue'
import HttpsPanel from './HttpsPanel.vue'
import IssuesPanel from './IssuesPanel.vue'
import LibraryStats from './LibraryStats.vue'
import LocationsPanel from './LocationsPanel.vue'
import MaintenancePanel from './MaintenancePanel.vue'
import PerformancePanel from './PerformancePanel.vue'
import ResetButton from './ResetButton.vue'
import SearchPanel from './SearchPanel.vue'
import SettingsGlyph from './SettingsGlyph.vue'
import StoreFormatsPanel from './StoreFormatsPanel.vue'
import TagsPanel from './TagsPanel.vue'
import TextureNamesPanel from './TextureNamesPanel.vue'
import ToolsPanel from './ToolsPanel.vue'
import UpdatesPanel from './UpdatesPanel.vue'
import { type AnyOpt, type OptKey, type Opts, SETTINGS, onOff } from './settingsContext'

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
  { id: 'general', get label() { return t('settings-page-general') }, get blurb() { return t('settings-page-general-blurb') } },
  { id: 'appearance', get label() { return t('settings-page-appearance') }, get blurb() { return t('settings-page-appearance-blurb') } },
  { id: 'library', get label() { return t('settings-page-library') }, get blurb() { return t('settings-page-library-blurb') } },
  { id: 'copies', get label() { return t('settings-page-copies') }, get blurb() { return t('settings-page-copies-blurb') } },
  { id: 'tags', need: 'edit', get label() { return t('settings-page-tags') }, get blurb() { return t('settings-page-tags-blurb') } },
  { id: 'store-files', need: 'admin', get label() { return t('settings-page-store-files') }, get blurb() { return t('settings-page-store-files-blurb') } },
  { id: 'locations', sep: true, need: 'admin', get label() { return t('settings-page-locations') }, get blurb() { return t('settings-page-locations-blurb') } },
  { id: 'tools', need: 'admin', get label() { return t('settings-page-tools') }, get blurb() { return t('settings-page-tools-blurb') } },
  { id: 'ai', need: 'admin', get label() { return t('settings-page-ai') }, get blurb() { return t('settings-page-ai-blurb') } },
  { id: 'search', need: 'admin', get label() { return t('settings-page-search') }, get blurb() { return t('settings-page-search-blurb') } },
  { id: 'adding', need: 'admin', get label() { return t('settings-page-adding') }, get blurb() { return t('settings-page-adding-blurb') } },
  { id: 'downloads', need: 'admin', get label() { return t('settings-page-downloads') }, get blurb() { return t('settings-page-downloads-blurb') } },
  { id: 'account', sep: true, get label() { return t('settings-page-account') }, get blurb() { return t('settings-page-account-blurb') } },
  { id: 'auth', need: 'admin', server: true, get label() { return t('settings-page-auth') }, get blurb() { return t('settings-page-auth-blurb') } },
  { id: 'connection', need: 'admin', server: true, get label() { return t('settings-page-connection') }, get blurb() { return t('settings-page-connection-blurb') } },
  { id: 'performance', sep: true, need: 'admin', get label() { return t('settings-page-performance') }, get blurb() { return t('settings-page-performance-blurb') } },
  { id: 'issues', need: 'admin', get label() { return t('settings-page-issues') }, get blurb() { return t('settings-page-issues-blurb') } },
  { id: 'maintenance', need: 'admin', get label() { return t('settings-page-maintenance') }, get blurb() { return t('settings-page-maintenance-blurb') } },
]
/** The pages this account may see: an admin sees them all (but the desktop app has no server to set up). */
const PAGES = computed(() => ALL_PAGES.filter((p) => (!p.need || may(p.need)) && !(p.server && isDesktop())))
const current = computed(() => (PAGES.value.some((p) => p.id === props.section) ? props.section : 'general'))
const page = computed(() => PAGES.value.find((p) => p.id === current.value)!)
const paneEl = ref<HTMLElement | null>(null)
function go(id: string) {
  emit('update:section', id)
  paneEl.value?.scrollTo({ top: 0 })
}
const busy = ref(false)

// Server options: each saves on its own when it's entered (Enter, or leaving the field); switches at once.
const opts = reactive<Opts>({ workers: 1, max_upload_mb: 1, max_expand_mb: 1, conversion_cache_mb: 1, name: '', public_url: '', soundfont: '' })
const OPT_KEYS = Object.keys(opts) as OptKey[]
/** An option's name, for the note that it's saved. */
const optLabel = (k: OptKey): string =>
  ({
    workers: t('settings-workers'),
    max_upload_mb: t('settings-max-upload'),
    max_expand_mb: t('settings-max-expand'),
    conversion_cache_mb: t('settings-conversion-cache'),
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
  const busyTool = v && [v.blender, v.ffmpeg, v.clip, v.clap].some((t) => t.state === 'looking' || t.state === 'downloading')
  const learning = !!v && [v.clip, v.clap].some((m) => m.state === 'ready' && m.done < m.total)
  if (busyTool || learning) {
    timer = window.setTimeout(async () => {
      try {
        const next = await api.settings()
        // Don't clobber a path or AI setting someone is typing.
        if (s.value) s.value = { ...s.value, blender: next.blender, ffmpeg: next.ffmpeg, clip: next.clip, clap: next.clap }
      } catch {
        // Try again on the next tick.
      }
      schedulePoll()
    }, 1500)
  }
}

provide(SETTINGS, {
  s: computed(() => s.value!),
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
onBeforeUnmount(() => window.clearTimeout(timer))
</script>

<template>
  <div class="settings-page">
    <nav class="snav" :aria-label="t('settings-title')">
      <h2>{{ t('settings-title') }}</h2>
      <ul>
        <li v-for="p in PAGES" :key="p.id" :class="{ sep: p.sep }">
          <button type="button" :aria-current="current === p.id ? 'page' : undefined" @click="go(p.id)">
            <SettingsGlyph class="ico" :page="p.id" />
            <span class="lbl">{{ p.label }}</span>
            <span v-if="p.id === 'ai' && s" class="pip" :data-on="s.llm.enabled" aria-hidden="true" />
          </button>
        </li>
      </ul>
    </nav>

    <div ref="paneEl" class="pane" :aria-busy="busy">
      <div class="inner" :class="{ wide: current === 'store-files' }">
        <header class="phead">
          <span class="pglyph" aria-hidden="true"><SettingsGlyph :page="page.id" :size="22" /></span>
          <div class="ptext">
            <h2>{{ page.label }}</h2>
            <p class="what">{{ page.blurb }}</p>
          </div>
        </header>

        <AppearancePanel v-if="current === 'appearance'" />

        <LibraryStats
          v-else-if="current === 'library'"
          :stats="stats"
          @tag="(t) => emit('tag', t)"
          @pack="(p) => emit('pack', p)"
          @open="(id) => emit('open', id)"
          @archive="emit('archive')"
        />

        <LocationsPanel v-else-if="current === 'locations'" @unauthorized="emit('unauthorized')" @changed="emit('changed')" />
        <IssuesPanel
          v-else-if="current === 'issues'"
          @unauthorized="emit('unauthorized')"
          @pack="(p) => emit('pack', p)"
          @open="(id) => emit('open', id)"
          @section="go"
        />
        <AccountPanel v-else-if="current === 'account'" />
        <AuthPanel v-else-if="current === 'auth'" />
        <CopiesPanel v-else-if="current === 'copies'" @open="(id) => emit('open', id)" @unauthorized="emit('unauthorized')" />
        <TagsPanel v-else-if="current === 'tags'" @tag="(t) => emit('tag', t)" @changed="emit('changed')" />
        <StoreFormatsPanel v-else-if="current === 'store-files'" />
        <template v-else-if="current === 'general'">
          <div v-if="s" class="body">
            <section>
              <div class="sh"><h3>{{ t('settings-name') }}</h3></div>
              <p class="what">{{ t('settings-name-what') }}</p>
              <div class="llm opts">
                <label for="opt-name">{{ t('settings-name') }}</label>
                <div class="key">
                  <input
                    id="opt-name"
                    v-model="opts.name"
                    class="field"
                    maxlength="40"
                    :placeholder="s.options.name.default"
                    :disabled="busy"
                    @change="saveOpt('name')"
                    @keydown.enter.prevent="saveOpt('name')"
                  />
                  <ResetButton v-if="changed('name')" :to="s.options.name.default" :disabled="busy" @reset="resetOpt('name')" />
                </div>
              </div>
            </section>
          </div>
          <HoardPanel @unauthorized="emit('unauthorized')" />
        </template>

        <p v-else-if="!s" class="loading">{{ t('settings-loading') }}</p>

        <div v-else class="body">
          <ToolsPanel v-if="current === 'tools'" />
          <SearchPanel v-else-if="current === 'search'" />
          <AiPanel v-else-if="current === 'ai'" />
          <AddingPanel v-else-if="current === 'adding'" />
          <TextureNamesPanel v-else-if="current === 'downloads'" />

          <template v-else-if="current === 'connection'">
            <section>
              <div class="sh"><h3>{{ t('settings-address') }}</h3></div>
              <p class="what">{{ t('settings-address-what') }}</p>
              <div class="llm opts">
                <label for="opt-public">{{ t('settings-public-url') }}</label>
                <div class="key">
                  <input
                    id="opt-public"
                    v-model="opts.public_url"
                    class="field mono"
                    type="url"
                    spellcheck="false"
                    :placeholder="t('settings-public-url-placeholder')"
                    :disabled="busy"
                    @change="saveOpt('public_url')"
                    @keydown.enter.prevent="saveOpt('public_url')"
                  />
                  <ResetButton v-if="changed('public_url')" :to="s.options.public_url.default || t('settings-public-url-default')" :disabled="busy" @reset="resetOpt('public_url')" />
                </div>
              </div>
            </section>
            <HttpsPanel />
          </template>

          <PerformancePanel v-else-if="current === 'performance'" :stats="stats" />

          <template v-else-if="current === 'maintenance'">
            <section>
              <div class="sh"><h3>{{ t('settings-updates') }}</h3></div>
              <p v-if="stats?.version" class="what">{{ t('settings-version', { version: stats.version }) }}</p>
              <p v-if="isDesktop()" class="what">{{ t('settings-desktop-updates') }}</p>
              <div v-else class="switches">
                <div class="opt-row">
                  <label class="check">
                    <input
                      type="checkbox"
                      :checked="s.options.update_check.value"
                      :disabled="busy"
                      @change="save({ update_check: ($event.target as HTMLInputElement).checked })"
                    />
                    {{ t('settings-update-check') }}
                  </label>
                  <ResetButton v-if="changed('update_check')" :to="onOff(s.options.update_check.default)" :disabled="busy" @reset="resetOpt('update_check')" />
                </div>
              </div>
              <UpdatesPanel v-if="!isDesktop()" :settings="s" @saved="take" @unauthorized="emit('unauthorized')" />
            </section>
            <MaintenancePanel @unauthorized="emit('unauthorized')" />
          </template>
        </div>
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
/* The page's heading: its icon on an accent tile, the anchor the eye lands on first. */
.phead { display: flex; align-items: center; gap: 0.95rem; margin-bottom: 1.5rem; }
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
@media (max-width: 47.99rem) {
  .settings-page { grid-template-columns: minmax(0, 1fr); grid-template-rows: auto minmax(0, 1fr); }
  .snav { display: flex; align-items: center; gap: 0.75rem; padding: 0.5rem 0.75rem; overflow-x: auto; border-right: 0; border-bottom: 1px solid var(--line); }
  .snav h2 { display: none; }
  .snav ul { display: flex; gap: 0.25rem; }
  .snav li.sep:not(:first-child) { margin: 0 0 0 0.3rem; padding: 0 0 0 0.55rem; border-top: 0; border-left: 1px solid var(--line); }
  .snav button { white-space: nowrap; min-height: 40px; gap: 0.45rem; }
  .snav button[aria-current='page']::before { inset: auto 0.6rem 0; width: auto; height: 2px; border-radius: 2px 2px 0 0; }
  .inner { padding: 1.1rem 1rem 5rem; }
  .phead { align-items: flex-start; gap: 0.75rem; margin-bottom: 1.1rem; }
  .pglyph { width: 2.5rem; height: 2.5rem; border-radius: 10px; }
}

.body { display: grid; gap: 1rem; }
/* General: the server's name, then the hoard (HoardPanel's root) the same gap below it. */
.body + .hoard { margin-top: 1rem; }
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
