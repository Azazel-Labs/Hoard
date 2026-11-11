<script setup lang="ts">
import { computed } from 'vue'
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
const playing = computed(() => isCurrent.value && player.playing)

// Touch screens have no Ctrl-click: a long press (which fires contextmenu) selects instead.
function onLongPress(e: MouseEvent) {
  if (!matchMedia('(hover: none)').matches) return
  e.preventDefault()
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
          :class="{ pixelated, glyphs: asset.kind === 'font' }"
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
.tile { position: relative; min-width: 0; }
.hit {
  display: flex; flex-direction: column; gap: 0.15rem; width: 100%;
  padding: 0; border: 0; background: none; text-align: left; cursor: pointer;
  border-radius: var(--r-tile);
}
.hit:focus-visible { outline: none; }
.hit:focus-visible .art { outline: 2px solid var(--k-font); outline-offset: 2px; }
.art {
  position: relative; display: grid; place-items: center;
  aspect-ratio: 1; width: 100%; border-radius: var(--r-tile); overflow: hidden;
  border: 1px solid var(--line);
  box-shadow: inset 0 3px 0 var(--kind);
}
.hit:hover .art { border-color: var(--muted); }
.selected .art { border-color: var(--kind); outline: 2px solid var(--kind); outline-offset: 1px; }
.checked .art { border-color: var(--text); box-shadow: inset 0 3px 0 var(--kind), inset 0 0 0 2px var(--text); }
img { width: 100%; height: 100%; object-fit: contain; }
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
.overlay { position: absolute; inset: 0 0 auto 0; aspect-ratio: 1; pointer-events: none; }
.play, .check {
  position: absolute; pointer-events: auto; display: grid; place-items: center;
  border: 1px solid var(--line); background: var(--panel); color: var(--text); cursor: pointer;
}
.play {
  right: 0.45rem; bottom: 0.45rem; width: 2rem; height: 2rem; border-radius: 50%;
  opacity: 0; transition: opacity 0.12s;
}
.check {
  left: 0.45rem; top: 0.55rem; width: 1.35rem; height: 1.35rem; border-radius: 4px;
  color: transparent; opacity: 0; transition: opacity 0.12s;
}
.tile:hover .play, .tile:focus-within .play, .playing .play { opacity: 1; }
.tile:hover .check, .selecting .check { opacity: 1; }
.checked .check { opacity: 1; background: var(--text); border-color: var(--text); color: var(--bg); }
.play:hover, .check:hover { border-color: var(--muted); }
.playing .play { background: var(--kind); border-color: var(--kind); color: var(--bg); }

/* Touch screens have no hover: keep the play button visible. */
@media (hover: none) {
  .play { opacity: 1; }
}
</style>
