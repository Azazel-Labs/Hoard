<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { formatDuration, thumbUrl, type Asset } from '../api'
import { canPreview, player, toggle } from '../player'
import KindGlyph from './KindGlyph.vue'

const props = defineProps<{ asset: Asset; selected: boolean; checked: boolean; selecting: boolean }>()
const emit = defineEmits<{ open: [MouseEvent | KeyboardEvent]; check: [MouseEvent] }>()

const detail = computed(() => {
  const m = props.asset.meta
  if (typeof m.width === 'number' && typeof m.height === 'number') return `${m.width}×${m.height}`
  if (typeof m.duration_secs === 'number') return formatDuration(m.duration_secs)
  if (typeof m.triangles === 'number') return `${m.triangles.toLocaleString()} tris`
  return ''
})
const pixelated = computed(() => props.asset.tags.some((t) => t.tag === 'style:pixel-art'))
const playable = computed(() => canPreview(props.asset))
const isCurrent = computed(() => player.id === props.asset.id)
// Thumbnails fade in once decoded instead of popping in line by line.
const loaded = ref(false)
watch(() => thumbUrl(props.asset), () => (loaded.value = false))
const playing = computed(() => isCurrent.value && player.playing)

// Touch screens have no Ctrl-click: a long press (which fires contextmenu) selects instead.
function onLongPress(e: MouseEvent) {
  if (!matchMedia('(hover: none)').matches) return
  e.preventDefault()
  navigator.vibrate?.(12)
  emit('check', e)
}
</script>

<template>
  <div
    class="tile"
    :class="{ selected, checked, selecting, playing }"
    :style="{ '--kind': `var(--k-${asset.kind})`, '--progress': isCurrent ? player.progress : 0 }"
  >
    <button
      type="button"
      class="hit"
      :data-id="asset.id"
      :aria-pressed="selected"
      :aria-label="`${asset.name}, ${asset.category ?? asset.kind}${detail ? `, ${detail}` : ''}`"
      @click="emit('open', $event)"
      @contextmenu="onLongPress"
    >
      <span class="art checker">
        <img
          v-if="asset.has_thumb"
          :src="thumbUrl(asset)"
          alt=""
          loading="lazy"
          decoding="async"
          :class="{ pixelated, glyphs: asset.kind === 'font', loaded }"
          @load="loaded = true"
        />
        <KindGlyph v-else :kind="asset.kind" :size="44" />
        <span v-if="isCurrent" class="playhead" aria-hidden="true" />
        <span class="ext mono">.{{ asset.ext || '?' }}</span>
      </span>
      <span class="name">{{ asset.name }}</span>
      <span class="sub">
        <span>{{ asset.category ?? asset.kind }}</span>
        <span v-if="detail" class="mono">{{ detail }}</span>
      </span>
    </button>
    <span class="overlay">
      <button
        v-if="playable"
        type="button"
        class="play"
        :aria-label="playing ? `Pause ${asset.name}` : `Play ${asset.name}`"
        tabindex="-1"
        @click.stop="toggle(asset)"
      >
        <svg v-if="playing" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
          <rect x="3.5" y="3" width="3" height="10" rx="0.5" fill="currentColor" />
          <rect x="9.5" y="3" width="3" height="10" rx="0.5" fill="currentColor" />
        </svg>
        <svg v-else viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
          <path d="M5 3.2v9.6a.5.5 0 0 0 .77.42l7.4-4.8a.5.5 0 0 0 0-.84l-7.4-4.8A.5.5 0 0 0 5 3.2Z" fill="currentColor" />
        </svg>
      </button>
      <button
        type="button"
        class="check"
        role="checkbox"
        :aria-checked="checked"
        :aria-label="`Select ${asset.name}`"
        tabindex="-1"
        @click.stop="emit('check', $event)"
      >
        <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
          <path d="M3.5 8.5 6.5 11.5 12.5 4.5" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
    </span>
  </div>
</template>

<style scoped>
.tile {
  position: relative; min-width: 0;
  /* Tiles rise in, staggered by their place in the page (App.vue sets --i). */
  animation: tile-in var(--dur-3) var(--ease-out) backwards;
  animation-delay: calc(min(var(--i, 0), 24) * 16ms);
}
@keyframes tile-in { from { opacity: 0; translate: 0 10px; scale: 0.97; } }
.hit {
  display: flex; flex-direction: column; gap: 0.15rem; width: 100%;
  padding: 0; border: 0; background: none; text-align: left; cursor: pointer;
  border-radius: var(--r-tile);
}
.hit:focus-visible { outline: none; }
.hit:focus-visible .art { outline: 2px solid var(--accent); outline-offset: 2px; }
.art {
  --ring: 0 0 #0000;
  --lift: 0 0 #0000;
  position: relative; display: grid; place-items: center;
  aspect-ratio: 1; width: 100%; border-radius: var(--r-tile); overflow: hidden;
  border: 1px solid var(--line);
  box-shadow: inset 0 3px 0 var(--kind), var(--ring), var(--lift);
  transition:
    border-color var(--dur-1), box-shadow var(--dur-2) var(--ease-out),
    translate var(--dur-2) var(--ease-out), scale var(--dur-2) var(--ease-pop);
}
/* Hover lifts the tile off the shelf; pressing pushes it back down. */
.tile:hover .art { border-color: var(--muted); translate: 0 -3px; --lift: var(--elev-2); }
.hit:active .art { translate: 0 0; scale: 0.97; --lift: 0 0 #0000; transition-duration: 70ms; }
.selected .art { border-color: var(--kind); outline: 2px solid var(--kind); outline-offset: 1px; }
.checked .art { border-color: var(--text); --ring: inset 0 0 0 2px var(--text); scale: 0.94; }
.checked:hover .art { scale: 0.94; }
img {
  width: 100%; height: 100%; object-fit: contain;
  opacity: 0; scale: 1.03; transition: opacity var(--dur-3) var(--ease-out), scale 0.45s var(--ease-out);
}
img.loaded { opacity: 1; scale: 1; }
.tile:hover img.loaded { scale: 1.05; }
img.pixelated { image-rendering: pixelated; }

/* Played portion of the waveform, like a scrubbed region in an audio editor. */
.playhead {
  position: absolute; inset: 0 auto 0 0; width: calc(var(--progress) * 100%);
  background: color-mix(in srgb, var(--kind) 22%, transparent);
  border-right: 2px solid var(--kind);
  pointer-events: none;
}
.ext {
  position: absolute; left: 0.4rem; bottom: 0.4rem;
  padding: 0 0.35rem; border-radius: 3px;
  background: var(--kind); color: var(--bg); font-weight: 650; font-size: 0.72rem; line-height: 1.5;
}
.name {
  margin-top: 0.35rem; font-size: var(--step--1); font-weight: 550;
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
}
.sub {
  display: flex; justify-content: space-between; gap: 0.5rem;
  font-size: 0.75rem; color: var(--muted);
  overflow: hidden; white-space: nowrap;
}
.sub > span:first-child { overflow: hidden; text-overflow: ellipsis; }

/* Overlay controls sit on the artwork's corners. */
.overlay {
  position: absolute; inset: 0 0 auto 0; aspect-ratio: 1; pointer-events: none;
  transition: translate var(--dur-2) var(--ease-out);
}
.tile:hover .overlay { translate: 0 -3px; }
.play, .check {
  position: absolute; pointer-events: auto; display: grid; place-items: center;
  border: 1px solid var(--line); background: var(--panel); color: var(--text); cursor: pointer;
  box-shadow: var(--elev-1);
  transition:
    opacity var(--dur-1), translate var(--dur-2) var(--ease-out), scale var(--dur-2) var(--ease-pop),
    background-color var(--dur-1), border-color var(--dur-1), color var(--dur-1);
}
.play {
  right: 0.45rem; bottom: 0.45rem; width: 2rem; height: 2rem; border-radius: 50%;
  opacity: 0; translate: 0 6px;
}
.check {
  left: 0.45rem; top: 0.55rem; width: 1.35rem; height: 1.35rem; border-radius: 4px;
  color: transparent; opacity: 0; scale: 0.8;
}
.tile:hover .play, .tile:focus-within .play, .playing .play { opacity: 1; translate: 0 0; }
.tile:hover .check, .selecting .check { opacity: 1; scale: 1; }
.checked .check { opacity: 1; scale: 1; background: var(--text); border-color: var(--text); color: var(--bg); animation: pop var(--dur-3) var(--ease-pop); }
/* The tick draws itself in. */
.check path { stroke-dasharray: 14; stroke-dashoffset: 14; transition: stroke-dashoffset var(--dur-2) var(--ease-out) 60ms; }
.checked .check path { stroke-dashoffset: 0; }
@keyframes pop { from { scale: 0.5; } }
.play:hover, .check:hover { border-color: var(--muted); scale: 1.1; }
.play:active, .check:active { scale: 0.92; transition-duration: 60ms; }
.playing .play { background: var(--kind); border-color: var(--kind); color: var(--bg); }

/* Touch screens have no hover: keep the play button visible. */
@media (hover: none) {
  .play { opacity: 1; }
}
</style>
