<script setup lang="ts">
// Adding a library location starts here: a folder, or a bucket in S3 storage.
import { onMounted, ref } from 'vue'
import { t } from '../i18n'

export type LocationKind = 'folder' | 's3'

const emit = defineEmits<{ pick: [LocationKind]; close: [] }>()
const dlg = ref<HTMLDialogElement | null>(null)
let picked: LocationKind | null = null

onMounted(() => dlg.value?.showModal())

function pick(kind: LocationKind) {
  picked = kind
  dlg.value?.close()
}
function closed() {
  if (picked) emit('pick', picked)
  else emit('close')
}
</script>

<template>
  <dialog ref="dlg" class="kinds" aria-labelledby="kinds-title" data-testid="location-kinds" @close="closed" @click.self="dlg?.close()">
    <header>
      <h2 id="kinds-title">{{ t('locations-add-title') }}</h2>
      <button type="button" class="btn ghost x" :aria-label="t('locations-cancel')" @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>
    <p class="q">{{ t('locations-kind-question') }}</p>
    <div class="cards">
      <button type="button" class="card" autofocus @click="pick('folder')">
        <!-- A folder, with the disk it's on and the cloud that keeps it in sync. -->
        <svg class="art folder" viewBox="0 0 160 96" aria-hidden="true">
          <rect x="18" y="66" width="74" height="16" rx="4" class="disk" />
          <circle cx="82" cy="74" r="2.5" class="led" />
          <path d="M30 24h20l7 7h33a5 5 0 0 1 5 5v26a5 5 0 0 1-5 5H30a5 5 0 0 1-5-5V29a5 5 0 0 1 5-5z" class="back" />
          <path d="M27 38h66a4 4 0 0 1 4 4l-3 22a5 5 0 0 1-5 4H31a5 5 0 0 1-5-4l-3-22a4 4 0 0 1 4-4z" class="front" />
          <path d="M112 50a12 12 0 0 1 3-23 15 15 0 0 1 28 4 10 10 0 0 1-2 19z" class="cloud" />
          <path d="M120 42a9 9 0 0 1 15-4m2 6a9 9 0 0 1-15 4" class="sync" />
          <path d="M135 34v4h-4M122 52v-4h4" class="sync" />
        </svg>
        <strong>{{ t('locations-kind-folder') }}</strong>
        <small>{{ t('locations-kind-folder-hint') }}</small>
      </button>
      <button type="button" class="card" @click="pick('s3')">
        <!-- A bucket in a cloud, files dropping into it. -->
        <svg class="art bucket" viewBox="0 0 160 96" aria-hidden="true">
          <path d="M50 50a16 16 0 0 1 4-31 22 22 0 0 1 41-4 18 18 0 0 1 14 35z" class="cloud" />
          <rect x="66" y="27" width="12" height="15" rx="2" class="file a" />
          <rect x="84" y="21" width="12" height="15" rx="2" class="file b" />
          <ellipse cx="80" cy="50" rx="30" ry="7" class="rim" />
          <path d="M50 50l6 32a5 5 0 0 0 5 4h38a5 5 0 0 0 5-4l6-32a30 7 0 0 1-60 0z" class="pail" />
          <path d="M53 62a27 6 0 0 0 54 0" class="band" />
        </svg>
        <strong>{{ t('locations-s3-title') }}</strong>
        <small>{{ t('locations-kind-s3-hint') }}</small>
      </button>
    </div>
  </dialog>
</template>

<style scoped>
.kinds {
  width: min(40rem, calc(100vw - 2rem)); padding: 1.1rem 1.3rem 1.3rem; border: 1px solid var(--line); border-radius: 10px;
  background: var(--panel); color: var(--text); box-shadow: var(--elev-3);
}
.kinds[open] { animation: kinds-in var(--dur-3) var(--ease-pop); }
.kinds::backdrop { background: var(--scrim); animation: fade var(--dur-2) var(--ease-out); }
@keyframes kinds-in { from { opacity: 0; scale: 0.96; } }
@keyframes fade { from { opacity: 0; } }
header { display: flex; align-items: center; justify-content: space-between; }
h2 { margin: 0; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.x { padding: 0.35em 0.45em; }
.q { margin: 0.2rem 0 0.9rem; color: var(--muted); font-size: var(--step--1); }
.cards { display: grid; grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr)); gap: 0.8rem; }
.card {
  display: grid; gap: 0.35rem; align-content: start; justify-items: start; padding: 0.8rem 0.9rem 1rem; text-align: start;
  border: 1px solid var(--line); border-radius: var(--r-tile); background: var(--bg); color: inherit; font: inherit; cursor: pointer;
  transition: border-color var(--dur-1), background-color var(--dur-1), translate var(--dur-1) var(--ease-out);
}
.card:hover, .card:focus-visible { border-color: var(--muted); background: var(--raised); translate: 0 -1px; }
.card strong { font-size: var(--step-0); }
.card small { color: var(--muted); font-size: var(--step--1); line-height: 1.45; }
.art { width: 100%; height: auto; margin-bottom: 0.4rem; border-radius: calc(var(--r-tile) - 3px); background: var(--raised); }
.card:hover .art { background: var(--panel); }
.art .disk { fill: color-mix(in srgb, var(--muted) 35%, var(--panel)); }
.art .led { fill: var(--ok); }
.art .back { fill: color-mix(in srgb, var(--k-sheet) 70%, var(--panel)); }
.art .front { fill: var(--k-sheet); }
.art .cloud { fill: color-mix(in srgb, var(--k-vector) 35%, var(--panel)); }
.art .sync { fill: none; stroke: var(--k-vector); stroke-width: 2.4; stroke-linecap: round; stroke-linejoin: round; }
.art .file { fill: var(--panel); stroke: var(--muted); stroke-width: 1.5; }
.art .file.a { fill: var(--k-texture); stroke: none; }
.art .file.b { fill: var(--k-audio); stroke: none; }
.art .rim { fill: color-mix(in srgb, var(--k-shader) 55%, var(--bg)); }
.art .pail { fill: var(--k-shader); }
.art .band { fill: none; stroke: color-mix(in srgb, var(--k-shader) 60%, var(--bg)); stroke-width: 2.4; }
.card:not(:hover) .art { filter: saturate(0.75); }
</style>
