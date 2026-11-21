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
        <!-- An open folder of assets (a picture, a model, a sound), with the badge of a folder kept in sync. -->
        <svg class="art folder" viewBox="0 0 160 96" aria-hidden="true">
          <ellipse cx="80" cy="85" rx="42" ry="3" class="shadow" />
          <path d="M51 26h16a4 4 0 0 1 3.1 1.5l4.4 5.5H109a4 4 0 0 1 4 4v40.5a4 4 0 0 1-4 4H51a4 4 0 0 1-4-4V30a4 4 0 0 1 4-4z" class="back" />
          <g transform="rotate(-10 64 40)">
            <rect x="53.5" y="25" width="21" height="27" rx="2" class="paper" />
            <rect x="56.5" y="28" width="15" height="12" rx="1" class="pic" />
            <path d="M56.5 40l5-5.5 3 3 2.5-2.5 4.5 5z" class="paper" />
            <circle cx="67.5" cy="31.5" r="1.6" class="paper" />
          </g>
          <g>
            <rect x="69.5" y="18" width="21" height="27" rx="2" class="paper" />
            <path d="M80 22.5l6 3.4v6.8L80 36l-6-3.3v-6.8z" class="cube" />
            <path d="M74 25.9l6 3.4 6-3.4M80 29.3V36" class="cube-edge" />
          </g>
          <g transform="rotate(9 96 38)">
            <rect x="85.5" y="24" width="21" height="27" rx="2" class="paper" />
            <path d="M89.5 33v5M92.5 30v11M95.5 32v7M98.5 28.5v14M101.5 31.5v8" class="wave" />
          </g>
          <path d="M45 46h70a3 3 0 0 1 3 3.4l-3.7 28.4a4 4 0 0 1-4 3.7H49.7a4 4 0 0 1-4-3.7L42 49.4a3 3 0 0 1 3-3.4z" class="front" />
          <rect x="70" y="56" width="20" height="7" rx="1.5" class="label" />
          <circle cx="116" cy="74" r="11" class="badge" />
          <path d="M110.6 72.6a5.6 5.6 0 0 1 9.6-2.8M121.4 75.4a5.6 5.6 0 0 1-9.6 2.8" class="sync" />
          <path d="M120.6 66.9v3.3h-3.3M111.4 81.1v-3.3h3.3" class="sync" />
        </svg>
        <strong>{{ t('locations-kind-folder') }}</strong>
        <small>{{ t('locations-kind-folder-hint') }}</small>
      </button>
      <button type="button" class="card" @click="pick('s3')">
        <!-- A bucket up in the clouds, files dropping into it. -->
        <svg class="art bucket" viewBox="0 0 160 96" aria-hidden="true">
          <path d="M101 38h25a8 8 0 0 0 .6-16 12 12 0 0 0-22.8-3.6A9.5 9.5 0 0 0 101 38z" class="cloud" />
          <path d="M28 30h17a5.5 5.5 0 0 0 0-11 7.5 7.5 0 0 0-14-1.5A6.3 6.3 0 0 0 28 30z" class="cloud far" />
          <ellipse cx="80" cy="85" rx="28" ry="3" class="shadow" />
          <ellipse cx="80" cy="46" rx="27" ry="6.5" class="inside" />
          <g transform="rotate(-14 72 38)">
            <rect x="64" y="26" width="17" height="22" rx="2" class="paper" />
            <rect x="66.5" y="28.5" width="12" height="10" rx="1" class="pic" />
            <path d="M66.5 38.5l4-4.5 2.5 2.5 2-2 3.5 4z" class="paper" />
          </g>
          <g transform="rotate(11 90 30)">
            <rect x="82" y="17" width="17" height="22" rx="2" class="paper" />
            <path d="M85.5 25v4M88 23v8M90.5 24.5v5M93 21.5v11M95.5 24v6" class="wave" />
          </g>
          <path d="M80.5 8v5M74 11.5l2 3.5M87 11.5l-2 3.5" class="drop" />
          <path d="M53 46a27 6.5 0 0 0 54 0l-6 34a21 5 0 0 1-42 0z" class="pail" />
          <path d="M55.1 58a25 6 0 0 0 49.8 0M57.2 70a23 5.5 0 0 0 45.6 0" class="band" />
          <path d="M53 46a27 6.5 0 0 0 54 0" class="rim" />
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
/* The art's own background, which the sync badge's ring matches to stand clear of the folder. On light themes it's
   a grey a step darker than the card, not white, for the white sheets of paper to show on. */
.art {
  --art-bg: light-dark(var(--l-check-a), var(--d-raised)); --paper: light-dark(#ffffff, #e4e4e8);
  width: 100%; height: auto; margin-bottom: 0.4rem; border-radius: calc(var(--r-tile) - 3px); background: var(--art-bg);
}
.card:hover .art { --art-bg: light-dark(var(--l-check-a), var(--d-panel)); }
.art .shadow { fill: color-mix(in srgb, var(--text) 9%, transparent); }
.art .paper { fill: var(--paper); }
.art rect.paper { stroke: light-dark(rgb(28 38 56 / 0.14), transparent); stroke-width: 0.8; }
.art .pic { fill: var(--k-texture); }
.art .cube { fill: var(--k-model); }
.art .cube-edge { fill: none; stroke: var(--paper); stroke-width: 1.1; stroke-linejoin: round; }
.art .wave { fill: none; stroke: var(--k-audio); stroke-width: 1.8; stroke-linecap: round; }
.art .back { fill: color-mix(in srgb, var(--k-sheet) 62%, var(--art-bg)); }
.art .front { fill: var(--k-sheet); }
.art .label { fill: var(--paper); opacity: 0.55; }
.art .badge { fill: var(--k-vector); stroke: var(--art-bg); stroke-width: 3; }
.art .sync { fill: none; stroke: var(--paper); stroke-width: 1.7; stroke-linecap: round; stroke-linejoin: round; }
.art .cloud { fill: color-mix(in srgb, var(--k-vector) 30%, var(--art-bg)); }
.art .cloud.far { fill: color-mix(in srgb, var(--k-vector) 16%, var(--art-bg)); }
.art .drop { fill: none; stroke: var(--muted); stroke-width: 1.4; stroke-linecap: round; opacity: 0.6; }
.art .inside { fill: color-mix(in srgb, var(--k-shader) 40%, var(--bg)); }
.art .pail { fill: var(--k-shader); }
.art .band { fill: none; stroke: color-mix(in srgb, var(--k-shader) 70%, var(--bg)); stroke-width: 2; }
.art .rim { fill: none; stroke: color-mix(in srgb, var(--k-shader) 55%, var(--paper)); stroke-width: 2.4; stroke-linecap: round; }
.card:not(:hover) .art { filter: saturate(0.8); }
</style>
