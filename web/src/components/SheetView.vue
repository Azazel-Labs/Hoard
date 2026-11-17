<script setup lang="ts">
// A sprite sheet: its animations playing and its sprites listed, each drawn upright on its whole canvas from the
// pack's pages, as the sheet Hoard keeps for it says.
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { api, thumbUrl, type Asset, type SheetData, type SheetSprite } from '../api'
import { t } from '../i18n'

const props = defineProps<{ asset: Asset; pixelated: boolean }>()

/** Sprites drawn in the list at once; finding one by name reaches the rest. */
const MOST_SHOWN = 120
/** A sprite in the list is drawn at most this many pixels a side. */
const TILE = 64

const sheet = ref<SheetData | null>(null)
const failed = ref(false)
/** Each page's image, once it's loaded. */
let pages: (HTMLImageElement | null)[] = []
const anim = ref(0)
/** The sprite picked from the list, shown instead of the animation. */
const picked = ref(-1)
const filter = ref('')
const playing = ref(!matchMedia('(prefers-reduced-motion: reduce)').matches)
const stage = ref<HTMLCanvasElement | null>(null)
const list = ref<HTMLElement | null>(null)
let frame = 0
let timer = 0
let loading = 0

const animations = computed(() => sheet.value?.animations ?? [])
/** What the stage shows: a picked sprite, an animation, or (with neither) the thumbnail. */
const showing = computed(() => picked.value >= 0 || animations.value.length > 0)

const counts = computed(() => {
  const s = sheet.value
  if (!s) return ''
  const parts = [t('sheet-sprites', { count: s.sprites.length }), t('sheet-pages', { count: s.pages.length })]
  if (animations.value.length) parts.push(t('sheet-animations', { count: animations.value.length }))
  return parts.join(' · ')
})

const matches = computed(() => {
  const all = (sheet.value?.sprites ?? []).map((s, i) => ({ ...s, i }))
  const q = filter.value.trim().toLowerCase()
  return q ? all.filter((s) => s.name.toLowerCase().includes(q)) : all
})
const shown = computed(() => matches.value.slice(0, MOST_SHOWN))

async function load() {
  const id = ++loading
  stop()
  sheet.value = null
  failed.value = false
  pages = []
  anim.value = 0
  picked.value = -1
  filter.value = ''
  frame = 0
  try {
    const s = await api.sheet(props.asset.id)
    if (id !== loading) return
    sheet.value = s
    pages = s.pages.map(() => null)
    s.pages.forEach((p, i) => {
      if (!p.asset) return
      const img = new Image()
      img.decoding = 'async'
      img.onload = () => {
        if (id !== loading) return
        pages[i] = img
        redraw()
      }
      img.src = `/api/assets/${p.asset}/file`
    })
    if (playing.value) start()
  } catch {
    if (id === loading) failed.value = true
  }
}

/** Draw a sprite upright on its whole canvas, scaled by `k`, with that canvas's corner at `ox`, `oy`. */
function paint(g: CanvasRenderingContext2D, s: SheetSprite, ox: number, oy: number, k: number) {
  const img = pages[s.page]
  if (!img) return
  const [x, y, w, h] = s.rect
  const turn = s.turn ?? 0
  const [uw, uh] = turn % 180 === 90 ? [h, w] : [w, h]
  const [ax, ay] = s.at ?? [0, 0]
  g.save()
  g.translate(ox, oy)
  g.scale(k, k)
  g.translate(ax + uw / 2, ay + uh / 2)
  // Its pixels are stored turned clockwise: turn them back.
  g.rotate((-turn * Math.PI) / 180)
  g.drawImage(img, x, y, w, h, -w / 2, -h / 2, w, h)
  g.restore()
}

function sized(c: HTMLCanvasElement, w: number, h: number) {
  const [cw, ch] = [Math.max(1, Math.round(w)), Math.max(1, Math.round(h))]
  if (c.width !== cw || c.height !== ch) [c.width, c.height] = [cw, ch]
  const g = c.getContext('2d')
  if (!g) return null
  g.clearRect(0, 0, cw, ch)
  g.imageSmoothingEnabled = !props.pixelated
  return g
}

/** The sprites the stage steps through: the picked one, or the animation's frames. */
function frames(): { sprite: number; ms: number }[] {
  if (picked.value >= 0) return [{ sprite: picked.value, ms: 0 }]
  return animations.value[anim.value]?.frames ?? []
}

function drawStage() {
  const c = stage.value
  const s = sheet.value
  const list = frames()
  if (!c || !s || !list.length) return
  // As big as the biggest frame, so the animation doesn't jump about; each frame centered on it.
  const sprites = list.map((f) => s.sprites[f.sprite]).filter((x): x is SheetSprite => !!x)
  const w = Math.max(1, ...sprites.map((x) => x.size[0]))
  const h = Math.max(1, ...sprites.map((x) => x.size[1]))
  const g = sized(c, w, h)
  const sprite = s.sprites[list[frame % list.length]!.sprite]
  if (g && sprite) paint(g, sprite, (w - sprite.size[0]) / 2, (h - sprite.size[1]) / 2, 1)
}

function drawList() {
  const s = sheet.value
  if (!s || !list.value) return
  for (const c of list.value.querySelectorAll<HTMLCanvasElement>('canvas[data-i]')) {
    const sprite = s.sprites[Number(c.dataset.i)]
    if (!sprite) continue
    const k = Math.min(1, TILE / Math.max(sprite.size[0], sprite.size[1], 1))
    const g = sized(c, sprite.size[0] * k, sprite.size[1] * k)
    if (g) paint(g, sprite, 0, 0, k)
  }
}

function redraw() {
  drawStage()
  drawList()
}

function tick() {
  const list = frames()
  const a = animations.value[anim.value]
  if (picked.value >= 0 || list.length < 2) return stop()
  if (frame + 1 >= list.length && a && !a.loop) {
    playing.value = false
    return stop()
  }
  frame = (frame + 1) % list.length
  drawStage()
  timer = window.setTimeout(tick, Math.max(16, list[frame]?.ms || 100))
}

function start() {
  stop()
  const list = frames()
  if (picked.value < 0 && list.length > 1) timer = window.setTimeout(tick, Math.max(16, list[frame % list.length]?.ms || 100))
}

function stop() {
  window.clearTimeout(timer)
  timer = 0
}

function togglePlay() {
  playing.value = !playing.value
  if (!playing.value) return stop()
  const a = animations.value[anim.value]
  if (a && !a.loop && frame + 1 >= a.frames.length) frame = 0
  picked.value = -1
  start()
}

function pick(i: number) {
  picked.value = picked.value === i ? -1 : i
  frame = 0
  if (picked.value >= 0) stop()
  else if (playing.value) start()
  void nextTick(drawStage)
}

watch(anim, () => {
  picked.value = -1
  frame = 0
  drawStage()
  if (playing.value) start()
})
watch(shown, () => void nextTick(drawList))
watch(() => props.pixelated, redraw)
watch(() => props.asset.id, load, { immediate: true })
watch(stage, (c) => c && drawStage())
onBeforeUnmount(() => {
  loading++
  stop()
})
</script>

<template>
  <div class="sheet">
    <div class="stage checker">
      <canvas
        v-show="showing"
        ref="stage"
        :class="{ pixelated }"
        role="img"
        :aria-label="picked >= 0 ? sheet?.sprites[picked]?.name : animations[anim]?.name"
      />
      <img v-if="!showing && asset.has_thumb" :src="thumbUrl(asset)" alt="" :class="{ pixelated }" />
    </div>
    <div class="controls">
      <template v-if="animations.length">
        <button type="button" class="btn play" :aria-label="playing ? t('sprite-pause') : t('sprite-play')" @click="togglePlay">
          <svg v-if="playing && picked < 0" viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><rect x="3.5" y="2.5" width="3" height="11" rx="1" /><rect x="9.5" y="2.5" width="3" height="11" rx="1" /></svg>
          <svg v-else viewBox="0 0 16 16" width="14" height="14" fill="currentColor" aria-hidden="true"><path d="M4.5 2.8v10.4a.8.8 0 0 0 1.2.7l8.3-5.2a.8.8 0 0 0 0-1.4L5.7 2.1a.8.8 0 0 0-1.2.7Z" /></svg>
        </button>
        <select v-model.number="anim" class="field" :aria-label="t('sprite-animation')">
          <option v-for="(a, i) in animations" :key="i" :value="i">{{ a.name }}</option>
        </select>
      </template>
      <span class="count" data-testid="sheet-counts">{{ counts }}</span>
    </div>
    <p v-if="failed" class="note" role="alert">{{ t('sheet-failed') }}</p>
    <p v-else-if="!sheet" class="note">{{ t('sheet-loading') }}</p>
    <section v-else class="sprites">
      <div class="head">
        <h3>{{ t('asset-sprites') }}</h3>
        <input v-model="filter" type="search" class="field" :placeholder="t('sheet-find')" :aria-label="t('sheet-find')" />
      </div>
      <ul ref="list" class="list">
        <li v-for="s in shown" :key="s.i">
          <button
            type="button"
            :class="{ on: picked === s.i }"
            :aria-pressed="picked === s.i"
            :title="`${s.name} · ${s.size[0]}×${s.size[1]}`"
            @click="pick(s.i)"
          >
            <canvas :data-i="s.i" :class="{ pixelated }" />
            <span class="name">{{ s.name }}</span>
          </button>
        </li>
      </ul>
      <p v-if="matches.length > shown.length" class="note">
        {{ t('sheet-showing', { shown: shown.length, total: matches.length }) }}
      </p>
      <p v-else-if="!matches.length" class="note">{{ t('sheet-no-match') }}</p>
    </section>
  </div>
</template>

<style scoped>
.sheet { display: grid; gap: 0.5rem; }
.stage {
  position: relative; aspect-ratio: 4 / 3;
  border-radius: var(--r-tile); border: 1px solid var(--line); overflow: hidden;
  box-shadow: inset 0 3px 0 var(--kind);
}
.stage canvas, .stage img { position: absolute; inset: 0; width: 100%; height: 100%; object-fit: contain; }
.pixelated { image-rendering: pixelated; }
.note { margin: 0; color: var(--muted); font-size: var(--step--1); }
.controls { display: flex; flex-wrap: wrap; align-items: center; gap: 0.4rem; }
.play { display: grid; place-items: center; width: 2rem; height: 2rem; padding: 0; border-radius: 50%; }
.count { font-size: var(--step--1); color: var(--muted); margin-left: auto; }
.head { display: flex; align-items: center; gap: 0.5rem; }
.head h3 { margin: 0; font-size: var(--step--1); font-weight: 600; color: var(--muted); }
.head input { flex: 1; min-width: 0; }
.list {
  list-style: none; margin: 0.4rem 0 0; padding: 0;
  display: grid; grid-template-columns: repeat(auto-fill, minmax(5.5rem, 1fr)); gap: 0.35rem;
}
.list button {
  display: grid; justify-items: center; align-content: start; gap: 0.25rem; width: 100%;
  padding: 0.35rem; border: 1px solid var(--line); border-radius: var(--r-tile);
  background: var(--panel); color: inherit; cursor: pointer;
}
.list button:hover { border-color: var(--kind); }
.list button.on { border-color: var(--accent); box-shadow: 0 0 0 1px var(--accent); }
.list canvas { max-width: 64px; max-height: 64px; }
.name {
  max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  font-size: 0.72rem; color: var(--muted);
}
</style>
