<script setup lang="ts">
// Browsing a pack as the folders it was on disk: the switch between that and everything in it at once, where you are
// (each step back up a click away), and the folders in this one.
import { computed } from 'vue'
import type { Facet } from '../api'
import { filters } from '../filters'
import { t } from '../i18n'
import { prefs } from '../prefs'

/** The folders in the one being browsed, with how many matches each holds. */
defineProps<{ folders: Facet[] }>()

/** Browse the pack folder by folder, or everything in it at once. */
function toggleFolders() {
  prefs.packFolders = filters.folder === null
  filters.folder = prefs.packFolders ? '' : null
}
/** The folder being browsed, step by step from the pack's top: each with the path to open it. */
const crumbs = computed(() => {
  const parts = (filters.folder ?? '').split('/').filter(Boolean)
  return parts.map((name, i) => ({ name, path: parts.slice(0, i + 1).join('/') }))
})
function intoFolder(name: string) {
  filters.folder = filters.folder ? `${filters.folder}/${name}` : name
}
</script>

<template>
  <nav v-if="filters.pack" class="folderbar" :aria-label="t('app-folders-label')">
    <button
      type="button"
      class="btn ghost folders-toggle"
      :aria-pressed="filters.folder !== null"
      :title="t('app-folders-title')"
      @click="toggleFolders"
    >
      <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round">
        <path d="M1.75 4.25a1 1 0 0 1 1-1h3.4l1.5 1.6h5.6a1 1 0 0 1 1 1v6.9a1 1 0 0 1-1 1H2.75a1 1 0 0 1-1-1Z" />
      </svg>
      <span>{{ t('app-folders') }}</span>
    </button>
    <ol v-if="filters.folder !== null" class="crumbs">
      <li>
        <button type="button" class="crumb" :aria-current="filters.folder ? undefined : 'location'" @click="filters.folder = ''">
          {{ filters.pack.name }}
        </button>
      </li>
      <li v-for="c in crumbs" :key="c.path">
        <button type="button" class="crumb" :aria-current="c.path === filters.folder ? 'location' : undefined" @click="filters.folder = c.path">
          {{ c.name }}
        </button>
      </li>
    </ol>
  </nav>
  <ul v-if="filters.pack && filters.folder !== null && folders.length" class="subfolders">
    <li v-for="f in folders" :key="f.value">
      <button type="button" class="subfolder" @click="intoFolder(f.value)">
        <svg viewBox="0 0 16 16" width="18" height="18" aria-hidden="true">
          <path d="M1.75 4.25a1 1 0 0 1 1-1h3.4l1.5 1.6h5.6a1 1 0 0 1 1 1v6.9a1 1 0 0 1-1 1H2.75a1 1 0 0 1-1-1Z" />
        </svg>
        <span class="name">{{ f.value }}</span>
        <span class="fcount">{{ t('app-asset-count', { count: f.count }) }}</span>
      </button>
    </li>
  </ul>
</template>

<style scoped>
.folderbar { display: flex; align-items: center; gap: 0.6rem; margin: -0.25rem 0 0.9rem; min-width: 0; }
/* Compact's bar over the grid is shorter, so nothing tucks up under it. */
:root[data-density='compact'] .folderbar { margin: 0 0 0.6rem; }
.folders-toggle { display: inline-flex; align-items: center; gap: 0.35rem; font-size: var(--step--1); padding: 0.25em 0.6em; }
.folders-toggle[aria-pressed='true'] { background: var(--panel); border-color: var(--line); }
.crumbs { display: flex; flex-wrap: wrap; align-items: center; margin: 0; padding: 0; list-style: none; min-width: 0; font-size: var(--step--1); }
.crumbs li { display: flex; align-items: center; min-width: 0; }
.crumbs li + li::before { content: '/'; color: var(--faint); padding: 0 0.15rem; }
.crumb {
  border: 0; background: none; padding: 0.15em 0.3em; border-radius: 4px; cursor: pointer; color: var(--muted);
  overflow-wrap: anywhere; text-align: start;
}
.crumb:hover { color: var(--text); background: var(--panel); }
.crumb[aria-current] { color: var(--text); font-weight: 600; }
.subfolders {
  display: grid; grid-template-columns: repeat(auto-fill, minmax(calc(var(--tile) * 1.4), 1fr)); gap: 0.5rem;
  margin: 0 0 1.1rem; padding: 0; list-style: none;
}
.subfolder {
  display: grid; grid-template-columns: auto 1fr; grid-template-rows: auto auto; column-gap: 0.55rem; align-items: center;
  width: 100%; padding: 0.55rem 0.7rem; border: 1px solid var(--line); border-radius: 8px; background: var(--panel);
  color: var(--text); cursor: pointer; text-align: start;
}
.subfolder:hover { border-color: var(--muted); }
.subfolder svg { grid-row: span 2; fill: color-mix(in srgb, var(--muted) 35%, transparent); stroke: var(--muted); stroke-width: 1.2; stroke-linejoin: round; }
.subfolder .name { font-size: var(--step--1); font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.subfolder .fcount { font-size: 0.72rem; color: var(--faint); font-variant-numeric: tabular-nums; }
</style>
