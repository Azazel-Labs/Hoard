<script setup lang="ts">
// The library's Filters column (a drawer on phones): your favorites and saved searches, projects, collections, color,
// kinds, tags and, searching sounds, music. Choosing a place to browse (a saved search, project, collection or your
// favorites) is App's to do, since it also changes the view; the rest just sets the filters.
import { computed } from 'vue'
import { api, type AssetKind, type Collection, type Facet, type Project, type SavedSearch } from '../api'
import { KIND_LABEL } from '../kinds'
import { auth, mayEdit } from '../auth'
import { filters, savedNow } from '../filters'
import { formatNumber, t } from '../i18n'
import { forgetSearch, mine } from '../mine'
import { prefs, toggleIn } from '../prefs'
import { loadProjects, projects } from '../projects'
import { confirmDialog, daysAgo, showError } from '../ui'
import MusicFilters from './MusicFilters.vue'

const props = defineProps<{ collections: Collection[]; kindFacets: Facet[]; tagFacets: Facet[]; loading: boolean }>()
const emit = defineEmits<{
  close: []
  favorites: []
  project: [name: string]
  collection: [c: Collection]
  search: [s: SavedSearch]
  kind: [kind: string]
  tag: [tag: string]
}>()

/** Values shown per filter group before "more". */
const FACET_PREVIEW = 8
/** Colors to search by in one click; the picker beside them does any other. */
const COLOR_PRESETS: { hex: string; name: () => string }[] = [
  { hex: '#b8322a', name: () => t('app-color-red') }, { hex: '#e07b28', name: () => t('app-color-orange') },
  { hex: '#e8c234', name: () => t('app-color-yellow') }, { hex: '#4f8a2e', name: () => t('app-color-green') },
  { hex: '#2a9d8f', name: () => t('app-color-teal') }, { hex: '#2f6fd0', name: () => t('app-color-blue') },
  { hex: '#7b4bb3', name: () => t('app-color-purple') }, { hex: '#d9599a', name: () => t('app-color-pink') },
  { hex: '#7a5230', name: () => t('app-color-brown') }, { hex: '#d6c09a', name: () => t('app-color-sand') },
  { hex: '#8a8a8a', name: () => t('app-color-gray') }, { hex: '#4a5560', name: () => t('app-color-slate') },
  { hex: '#1c1c1c', name: () => t('app-color-black') }, { hex: '#f2f2f0', name: () => t('app-color-white') },
]

/** Signed in, rather than browsing as a guest. */
const member = computed(() => !!auth.status?.signed_in)
/** May change the library. */
const editor = computed(() => mayEdit())

async function forgetProject(p: Project) {
  const ok = await confirmDialog({
    title: t('app-forget-project-title', { name: p.name }),
    body: t('app-forget-project-body', { count: p.assets }),
    confirm: t('app-forget-project-confirm'),
  })
  if (!ok) return
  try {
    await api.forgetProject(p.name)
    if (filters.project === p.name) filters.project = ''
    await loadProjects()
  } catch (e) {
    showError(t('app-forget-project-failed', { name: p.name, error: (e as Error).message }))
  }
}

/** A color from the sidebar: added to the filters already set, or taken off when it's the one set. */
function pickColor(hex: string) {
  filters.color = filters.color === hex ? '' : hex
  if (filters.color) filters.imitation = ''
}

/** The heading of each tag group, by tag namespace (a function, so it follows a change of language). */
const NS_LABEL = (): Record<string, string> => ({
  '': t('app-ns-subject'), style: t('app-ns-style'), theme: t('app-ns-theme'), material: t('app-ns-material'),
  map: t('app-ns-map'), color: t('app-ns-color'), mood: t('app-ns-mood'), genre: t('app-ns-genre'),
  license: t('app-ns-license'), res: t('app-ns-res'), duration: t('app-ns-duration'), poly: t('app-ns-poly'),
  format: t('app-ns-format'), weight: t('app-ns-weight'), tech: t('app-ns-tech'), blender: t('app-ns-blender'),
  depth: t('app-ns-depth'), ucs: t('app-ns-ucs'), bpm: t('app-ns-bpm'), root: t('app-ns-root'), level: t('app-ns-level'),
  ue: t('app-ns-ue'), engine: t('app-ns-engine'), extracted: t('app-ns-extracted'), script: t('app-ns-script'),
})
// Plain tags that are measured facts rather than subjects; they get their own group so "Subject" stays useful.
const TECH = new Set(['pot', 'square', 'alpha', 'mono', 'stereo', 'surround', 'tileable', 'frames', 'rigged', 'animated', 'loop', 'one-shot', 'clipped'])
const NS_ORDER = ['', 'ue', 'ucs', 'style', 'theme', 'material', 'map', 'mood', 'genre', 'color', 'tech', 'duration', 'bpm', 'root', 'level', 'res', 'depth', 'poly', 'blender', 'license', 'format']

const tagGroups = computed(() => {
  const groups = new Map<string, Facet[]>()
  for (const f of props.tagFacets) {
    const i = f.value.indexOf(':')
    const ns = i > 0 ? f.value.slice(0, i) : TECH.has(f.value) ? 'tech' : ''
    if (ns === 'kind' || ns === 'set') continue
    if (!groups.has(ns)) groups.set(ns, [])
    groups.get(ns)!.push(f)
  }
  const rank = (ns: string) => (NS_ORDER.indexOf(ns) === -1 ? 50 : NS_ORDER.indexOf(ns))
  const labels = NS_LABEL()
  return [...groups.entries()]
    .sort(([a], [b]) => rank(a) - rank(b) || a.localeCompare(b))
    .map(([ns, facets]) => {
      // Active values always stay visible, even in a collapsed or shortened group.
      const active = facets.filter((f) => filters.tags.includes(f.value))
      const open = !prefs.collapsed.includes(ns)
      const long = prefs.expanded.includes(ns) ? facets : facets.slice(0, FACET_PREVIEW)
      const shown = open ? [...new Set([...long, ...active])] : active
      return { ns, label: labels[ns] ?? ns, open, shown, hidden: open ? facets.length - shown.length : 0, all: facets.length }
    })
})
/** Tempo and key: only while searching sounds (Kind is Audio, or every result is a sound), or while one is set. */
const showMusic = computed(
  () =>
    filters.kind === 'audio' ||
    (props.kindFacets.length === 1 && props.kindFacets[0].value === 'audio') ||
    filters.bpmMin !== '' || filters.bpmMax !== '' || !!filters.key,
)
const facetLabel = (ns: string, value: string) => (ns && ns !== 'tech' ? value.slice(ns.length + 1) : value)
</script>

<template>
  <aside id="filters" class="side" :aria-label="t('app-filters')">
    <div class="side-head">
      <h2 class="side-title">{{ t('app-filters') }}</h2>
      <button type="button" class="btn ghost" @click="emit('close')">{{ t('app-done') }}</button>
    </div>
    <section v-if="member" class="collections yours">
      <h3>{{ t('app-yours') }}</h3>
      <ul class="kinds">
        <li>
          <button type="button" :aria-pressed="filters.favorites" @click="emit('favorites')">
            <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true"><path d="M8 1.9l1.8 3.7 4.1.6-3 2.9.7 4.1L8 11.3l-3.6 1.9.7-4.1-3-2.9 4.1-.6z" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" /></svg>
            <span class="cname">{{ t('app-favorites') }}</span>
            <span class="n mono">{{ mine.favorites.size }}</span>
          </button>
        </li>
        <li v-if="editor">
          <!-- Goes with whatever else is chosen: what's left to go over in this pack, say. -->
          <button type="button" :aria-pressed="filters.reviewed === 'no'" :title="t('app-needs-review-title')" @click="filters.reviewed = filters.reviewed === 'no' ? '' : 'no'">
            <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="8" cy="8" r="6" stroke-dasharray="2.4 2" /><path d="m5.4 8.2 1.8 1.8 3.4-3.7" /></svg>
            <span class="cname">{{ t('app-needs-review') }}</span>
          </button>
        </li>
        <li v-for="s in mine.searches" :key="s.id" class="saved">
          <button type="button" :aria-pressed="savedNow?.id === s.id" :title="s.query" @click="emit('search', s)">
            <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><circle cx="7" cy="7" r="4.5" /><path d="m10.5 10.5 3 3" /></svg>
            <span class="cname">{{ s.name }}</span>
          </button>
          <button type="button" class="forget" :aria-label="t('app-forget-search-label', { name: s.name })" :title="t('app-forget-search')" @click="forgetSearch(s)">
            <svg viewBox="0 0 16 16" width="10" height="10" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
          </button>
        </li>
      </ul>
    </section>
    <section v-if="projects.length" class="collections projects">
      <h3 :title="t('app-projects-title')">{{ t('app-projects') }}</h3>
      <ul class="kinds">
        <li v-for="p in projects" :key="p.id" class="saved">
          <button type="button" :aria-pressed="filters.project === p.name" :title="p.synced_by ? t('app-project-synced-by', { when: daysAgo(p.synced_at), who: p.synced_by }) : t('app-project-synced', { when: daysAgo(p.synced_at) })" @click="emit('project', p.name)">
            <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"><path d="M2 4.5h4.5l1.5 1.5H14v6.5H2z" /></svg>
            <span class="cname">{{ p.name }}</span>
            <span class="n mono">{{ formatNumber(p.assets) }}</span>
          </button>
          <button v-if="editor" type="button" class="forget" :aria-label="t('app-forget-project-label', { name: p.name })" :title="t('app-forget-project')" @click="forgetProject(p)">
            <svg viewBox="0 0 16 16" width="10" height="10" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
          </button>
        </li>
      </ul>
    </section>
    <section v-if="collections.length" class="collections">
      <h3>{{ t('app-collections') }}</h3>
      <ul class="kinds">
        <li v-for="c in collections" :key="c.id">
          <button type="button" :aria-pressed="filters.collection?.id === c.id" @click="emit('collection', c)">
            <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true"><path d="M3 2.5h10v11.5l-5-3-5 3z" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" /></svg>
            <span class="cname">{{ c.name }}</span>
            <span class="n mono">{{ c.asset_count }}</span>
          </button>
        </li>
      </ul>
    </section>
    <section class="colors">
      <h3>{{ t('app-color') }}</h3>
      <div class="swatches">
        <button
          v-for="c in COLOR_PRESETS"
          :key="c.hex"
          type="button"
          class="sw"
          :style="{ background: c.hex }"
          :aria-pressed="filters.color === c.hex"
          :aria-label="t('app-color-swatch', { color: c.name(), lower: c.name().toLowerCase() })"
          :title="c.name()"
          @click="pickColor(c.hex)"
        />
        <label class="sw pick" :class="{ on: filters.color && !COLOR_PRESETS.some((c) => c.hex === filters.color) }" :title="t('app-color-any')">
          <span class="visually-hidden">{{ t('app-color-pick') }}</span>
          <input type="color" :value="filters.color || '#808080'" @change="filters.color = ($event.target as HTMLInputElement).value" />
        </label>
      </div>
    </section>
    <p v-if="!kindFacets.length && !loading" class="side-empty">{{ t('app-no-filters') }}</p>
    <section v-if="kindFacets.length">
      <h3>{{ t('app-kind') }}</h3>
      <ul class="kinds">
        <li v-for="k in kindFacets" :key="k.value">
          <button type="button" :aria-pressed="filters.kind === k.value" :style="{ '--kind': `var(--k-${k.value})` }" @click="emit('kind', k.value)">
            <span class="dot" aria-hidden="true" />
            <span>{{ KIND_LABEL[k.value as AssetKind] ?? k.value }}</span>
            <span class="n mono">{{ k.count }}</span>
          </button>
        </li>
      </ul>
    </section>
    <MusicFilters v-if="showMusic && !tagGroups.some((g) => g.ns === '')" />
    <template v-for="g in tagGroups" :key="g.ns">
      <section>
        <h3>
          <button
            type="button"
            class="group-head"
            :aria-expanded="g.open"
            @click="prefs.collapsed = toggleIn(prefs.collapsed, g.ns)"
          >
            <svg viewBox="0 0 10 10" width="9" height="9" aria-hidden="true"><path d="M3 2l4 3-4 3" fill="none" stroke="currentColor" stroke-width="1.5" /></svg>
            {{ g.label }}
            <span v-if="!g.open" class="n mono">{{ g.all }}</span>
          </button>
        </h3>
        <div v-if="g.shown.length" class="facet-tags">
          <button
            v-for="f in g.shown"
            :key="f.value"
            type="button"
            class="facet"
            :class="{ long: facetLabel(g.ns, f.value).length > 30 }"
            :title="facetLabel(g.ns, f.value).length > 30 ? facetLabel(g.ns, f.value) : undefined"
            :aria-pressed="filters.tags.includes(f.value)"
            @click="emit('tag', f.value)"
          >
            <span class="fl">{{ facetLabel(g.ns, f.value) }}</span><span class="n mono">{{ f.count }}</span>
          </button>
          <button v-if="g.hidden > 0" type="button" class="facet more-facets" @click="prefs.expanded = toggleIn(prefs.expanded, g.ns)">
            {{ t('app-facets-more', { count: g.hidden }) }}
          </button>
          <button
            v-else-if="prefs.expanded.includes(g.ns) && g.all > FACET_PREVIEW"
            type="button"
            class="facet more-facets"
            @click="prefs.expanded = toggleIn(prefs.expanded, g.ns)"
          >
            {{ t('app-facets-fewer') }}
          </button>
        </div>
      </section>
      <MusicFilters v-if="showMusic && g.ns === ''" />
    </template>
  </aside>
</template>

<style scoped>
.side-head { display: none; }
.side-empty { margin: 0; font-size: var(--step--1); color: var(--faint); }
.collections svg { flex: none; color: var(--muted); }
.cname { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.side section + section { margin-top: 1rem; }
.side h3 { margin: 0 0 0.4rem; font-size: 0.75rem; font-weight: 650; color: var(--faint); }
.swatches { display: grid; grid-template-columns: repeat(auto-fill, minmax(1.45rem, 1fr)); gap: 0.3rem; }
.sw {
  position: relative; aspect-ratio: 1; border: 0; padding: 0; border-radius: 50%; cursor: pointer;
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--text) 18%, transparent);
  transition: scale var(--dur-2) var(--ease-pop), box-shadow var(--dur-1);
}
.sw:hover { scale: 1.12; }
.sw[aria-pressed='true'], .sw.on { box-shadow: 0 0 0 2px var(--bg), 0 0 0 3.5px var(--text); }
.sw.pick {
  display: block; overflow: hidden;
  background: conic-gradient(from 90deg, #e04040, #e0c040, #40c060, #40b0e0, #6050e0, #d040b0, #e04040);
}
.sw.pick input { position: absolute; inset: 0; width: 100%; height: 100%; opacity: 0; cursor: pointer; border: 0; padding: 0; }
.group-head {
  display: flex; align-items: center; gap: 0.35rem; width: 100%; padding: 0; border: 0; background: none; cursor: pointer;
  font: inherit; color: inherit; text-align: left;
}
.group-head:hover { color: var(--muted); }
.group-head svg { transition: rotate 0.12s; }
.group-head[aria-expanded='true'] svg { rotate: 90deg; }
.kinds { list-style: none; margin: 0; padding: 0; }
.kinds button {
  display: flex; width: 100%; align-items: center; gap: 0.55rem; padding: 0.3rem 0.45rem;
  border: 0; border-radius: var(--r-chip); background: none; cursor: pointer; font-size: var(--step--1); text-align: left;
}
.kinds button:hover { background: var(--raised); }
.kinds button[aria-pressed='true'] { background: var(--raised); box-shadow: inset 3px 0 0 var(--kind); }
.dot { width: 10px; height: 10px; border-radius: 2px; background: var(--kind); flex: none; }
.n { margin-left: auto; color: var(--faint); font-size: 0.75rem; padding-left: 0.4rem; }
.collections li.saved { display: flex; align-items: center; }
.collections .forget {
  flex: none; width: auto; padding: 0.3rem 0.4rem; color: var(--faint); opacity: 0;
}
.collections li.saved:hover .forget, .collections .forget:focus-visible { opacity: 1; }
.collections .forget:hover { color: var(--text); }
@media (hover: none) { .collections .forget { opacity: 1; } }
.facet-tags { display: flex; flex-wrap: wrap; gap: 0.3rem; }
.facet {
  display: inline-flex; align-items: baseline; padding: 0.1em 0.45em; border: 1px solid var(--line); border-radius: var(--r-chip);
  background: none; cursor: pointer; font-size: 0.78rem;
}
.facet:hover { border-color: var(--muted); }
/* Long tags stop at about 30 characters and fade, so the sidebar never scrolls sideways. */
.facet { max-width: 100%; min-width: 0; }
.facet .fl { min-width: 0; max-width: 30ch; overflow: hidden; white-space: nowrap; }
.facet.long .fl { mask-image: linear-gradient(to right, #000 calc(100% - 2.5em), transparent); }
.facet .n { flex: none; }
.facet[aria-pressed='true'] { background: var(--text); color: var(--bg); border-color: var(--text); }
.facet[aria-pressed='true'] .n { color: var(--bg); opacity: 0.6; }
.more-facets { border-style: dashed; color: var(--muted); }

@media (max-width: 900px) {
  .side-head { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.75rem; }
  .side-title { margin: 0; font-size: var(--step-1); color: var(--text); font-weight: 650; }
}
</style>
