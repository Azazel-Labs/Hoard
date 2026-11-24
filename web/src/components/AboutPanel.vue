<script setup lang="ts">
// Settings → About: what Hoard is, which version and edition this is, where it lives (GitHub, and the Steam store for
// the Steam build), its license, and everyone else's work it's made with: the programs and models it downloads when
// they're needed, and every crate, npm package and piece of art it's built from with their licenses (`cargo xtask
// licenses` writes those to licenses.json, loaded only when this page is opened).
import { computed, onMounted, reactive, ref } from 'vue'
import type { Stats } from '../api'
import { isDesktop, may, steamApp } from '../auth'
import { formatNumber, t } from '../i18n'
import ReportDialog from './ReportDialog.vue'

const props = defineProps<{ stats: Stats | null }>()

interface Package { name: string; version: string; license: string | null; url: string | null; texts: number[] }
interface Licenses { hoard: string; crates: Package[]; npm: Package[]; art?: Package[]; texts: string[] }

const REPO = 'https://github.com/Azazel-Labs/Hoard'
const DISCORD = 'https://discord.gg/zf9ncS4EW'
const steam = steamApp()
const reporting = ref(false)
const edition = computed(() =>
  steam ? t('about-edition-steam') : isDesktop() ? t('about-edition-desktop') : t('about-edition-server'),
)

/** What Hoard fetches the first time it needs it, rather than shipping. The editors are fetched only by builds that do
 * (the Docker image); everywhere else they're found where they're installed, so they aren't listed. */
const DOWNLOADED = (): { name: string; license: string; url: string; what: string; editor?: string }[] => [
  {
    name: 'Blender',
    license: 'GPL-2.0-or-later',
    url: 'https://www.blender.org',
    what: t('about-blender'),
    editor: 'blender',
  },
  {
    name: 'Unreal Editor',
    license: 'Epic Unreal Engine EULA',
    url: 'https://www.unrealengine.com/eula/unreal',
    what: t('settings-unreal-editor-what'),
    editor: 'unreal',
  },
  { name: 'FFmpeg', license: 'GPL-3.0-or-later', url: 'https://ffmpeg.org', what: t('about-ffmpeg') },
  {
    name: 'SigLIP 2',
    license: 'Apache-2.0',
    url: 'https://huggingface.co/google/siglip2-base-patch32-256',
    what: t('about-siglip'),
  },
  { name: 'CLAP', license: 'Apache-2.0', url: 'https://huggingface.co/laion/clap-htsat-unfused', what: t('about-clap') },
]
const downloaded = computed(() =>
  DOWNLOADED().filter((d) => !d.editor || props.stats?.editor_downloads?.includes(d.editor)),
)

const licenses = ref<Licenses | null>(null)
const failed = ref(false)
const filter = ref('')
/** The packages whose license texts are open: only those are put on the page. */
const open = reactive(new Set<string>())

const all = computed(() => {
  const l = licenses.value
  if (!l) return []
  return [
    ...l.crates.map((p) => ({ ...p, key: `crate:${p.name}@${p.version}`, from: 'crates.io' })),
    ...l.npm.map((p) => ({ ...p, key: `npm:${p.name}@${p.version}`, from: 'npm' })),
    // Art copied in (an icon from a set), from wherever it's published.
    ...(l.art ?? []).map((p) => ({ ...p, key: `art:${p.name}@${p.version}`, from: p.url ? new URL(p.url).host : '' })),
  ]
})
const shown = computed(() => {
  const q = filter.value.trim().toLowerCase()
  return q ? all.value.filter((p) => p.name.toLowerCase().includes(q) || p.license?.toLowerCase().includes(q)) : all.value
})

function toggled(key: string, e: Event) {
  if ((e.target as HTMLDetailsElement).open) open.add(key)
  else open.delete(key)
}

onMounted(async () => {
  try {
    const res = await fetch('licenses.json')
    if (!res.ok) throw new Error(res.statusText)
    licenses.value = await res.json()
  } catch {
    failed.value = true
  }
})
</script>

<template>
  <div class="about">
    <section data-testid="about-hoard">
      <div class="sh"><h3>Hoard</h3></div>
      <p class="what">{{ t('about-what') }}</p>
      <dl class="facts">
        <template v-if="stats?.version">
          <dt>{{ t('about-version') }}</dt>
          <dd data-testid="about-version">{{ stats.version }}</dd>
        </template>
        <dt>{{ t('about-edition') }}</dt>
        <dd data-testid="about-edition">{{ edition }}</dd>
        <dt>{{ t('about-made-by') }}</dt>
        <dd>Azazel Labs LLC</dd>
      </dl>
      <ul class="links">
        <li><a :href="REPO" target="_blank" rel="noopener">{{ t('about-source') }}</a></li>
        <li v-if="may('admin')">
          <button type="button" class="linkish" data-testid="about-report" @click="reporting = true">
            {{ t('about-report') }}
          </button>
        </li>
        <li v-else><a :href="`${REPO}/issues`" target="_blank" rel="noopener">{{ t('about-issues') }}</a></li>
        <li><a :href="DISCORD" target="_blank" rel="noopener" data-testid="about-discord">{{ t('about-discord') }}</a></li>
        <li v-if="steam">
          <a :href="`https://store.steampowered.com/app/${steam}`" target="_blank" rel="noopener" data-testid="about-steam">
            {{ t('about-steam') }}
          </a>
        </li>
      </ul>
    </section>

    <section>
      <div class="sh"><h3>{{ t('about-license') }}</h3></div>
      <p class="what">{{ t('about-license-what') }}</p>
      <details v-if="licenses" class="pkg">
        <summary>MIT-0</summary>
        <pre>{{ licenses.hoard }}</pre>
      </details>
    </section>

    <section>
      <div class="sh"><h3>{{ t('about-downloaded') }}</h3></div>
      <p class="what">{{ t('about-downloaded-what') }}</p>
      <ul class="plain" data-testid="about-downloaded">
        <li v-for="d in downloaded" :key="d.name">
          <a :href="d.url" target="_blank" rel="noopener">{{ d.name }}</a> <small>{{ d.license }}</small>
          <span class="muted">{{ d.what }}</span>
        </li>
      </ul>
      <p class="what">{{ t('about-models') }}</p>
    </section>

    <section data-testid="about-built-with">
      <div class="sh"><h3>{{ t('about-built-with') }}</h3></div>
      <p class="what">
        {{ t('about-built-with-what') }}
        <template v-if="steam">{{ t('about-steamworks') }}</template>
      </p>
      <p v-if="failed" class="what">{{ t('about-licenses-failed') }}</p>
      <p v-else-if="!licenses" class="what">{{ t('about-licenses-loading') }}</p>
      <template v-else>
        <input
          v-model="filter"
          class="field"
          type="search"
          :placeholder="t('about-filter')"
          :aria-label="t('about-filter')"
          data-testid="about-filter"
        />
        <p class="what">{{ t('about-count', { shown: formatNumber(shown.length), total: formatNumber(all.length) }) }}</p>
        <ul class="plain pkgs">
          <li v-for="p in shown" :key="p.key">
            <details class="pkg" @toggle="toggled(p.key, $event)">
              <summary>
                <span class="name">{{ p.name }}</span> <small>{{ p.version }}</small>
                <small class="lic">{{ p.license ?? t('about-no-license') }}</small>
              </summary>
              <template v-if="open.has(p.key)">
                <p class="where">
                  {{ p.from }}<template v-if="p.url"> · <a :href="p.url" target="_blank" rel="noopener">{{ p.url }}</a></template>
                </p>
                <pre v-for="i in p.texts" :key="i">{{ licenses.texts[i] }}</pre>
              </template>
            </details>
          </li>
        </ul>
      </template>
    </section>
    <ReportDialog v-if="reporting" :repo="REPO" @close="reporting = false" />
  </div>
</template>

<style scoped>
/* Its sections are cards like the rest of Settings' (that look is SettingsPage's). */
.about { display: grid; gap: 1rem; font-size: var(--step--1); }
section { display: grid; gap: 0.6rem; min-width: 0; }
.sh h3 { margin: 0; }
.what { margin: 0; color: var(--muted); line-height: 1.5; max-width: 40rem; }
.facts { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.3rem 1rem; margin: 0; }
.facts dt { color: var(--muted); }
.facts dd { margin: 0; overflow-wrap: anywhere; }
.links { display: flex; flex-wrap: wrap; gap: 0.4rem 1.2rem; margin: 0; padding: 0; list-style: none; }
.linkish { padding: 0; border: 0; background: none; color: LinkText; font: inherit; text-decoration: underline; cursor: pointer; }
.plain { display: grid; gap: 0.35rem; margin: 0; padding: 0; list-style: none; }
.plain small, .muted { color: var(--muted); }
.muted { display: block; }
.pkgs { gap: 0; border-top: 1px solid var(--line); }
.pkgs li { border-bottom: 1px solid var(--line); }
.pkg summary { padding: 0.35rem 0; cursor: pointer; overflow-wrap: anywhere; }
.pkg .name { font-weight: 550; }
.pkg .lic { margin-left: 0.5rem; }
.where { margin: 0 0 0.4rem; color: var(--muted); overflow-wrap: anywhere; }
pre {
  max-height: 20rem; margin: 0 0 0.6rem; padding: 0.6rem 0.75rem; overflow: auto; border-radius: 6px; background: var(--well);
  font-size: var(--step--2); white-space: pre-wrap; overflow-wrap: anywhere;
}
input.field { max-width: 24rem; }
</style>
