<script setup lang="ts">
// A glinting gold pip by something new in Hoard (see news.ts). Given an `id`, it marks that thing; given `ids` (a
// page's new things), it marks the way there, with how many when there's more than one. Under the pointer (or
// focused, or clicked) it says what's new, with Got it to stop pointing it out. It doesn't go by itself: what's new
// counts as seen once it's been used, dismissed here, gone to from the tray, or all marked seen.
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { formatNumber, t } from '../i18n'
import { isNew, markSeen, newThing } from '../news'

const props = defineProps<{ id?: string; ids?: string[] }>()

const shown = computed(() => (props.id ? [props.id] : (props.ids ?? [])).filter(isNew))
const count = computed(() => shown.value.length)
const things = computed(() => shown.value.map(newThing).filter((n) => !!n))
const el = ref<HTMLElement | null>(null)
const card = ref<HTMLElement | null>(null)
const open = ref(false)
const at = ref({ left: 0, top: 0 })
const cardId = `newpip-${Math.random().toString(36).slice(2, 9)}`

/** Pauses before opening under a passing pointer and before closing, so the pointer can cross to the card. */
const OPEN_AFTER = 180
const CLOSE_AFTER = 260
let timer: ReturnType<typeof setTimeout> | undefined

function place() {
  const r = el.value?.getBoundingClientRect()
  if (!r) return
  const w = card.value?.offsetWidth ?? 288
  const h = card.value?.offsetHeight ?? 120
  const gap = 8
  const left = Math.min(Math.max(8, r.left + r.width / 2 - 24), window.innerWidth - w - 8)
  const below = r.bottom + gap
  const top = below + h <= window.innerHeight - 8 ? below : Math.max(8, r.top - gap - h)
  at.value = { left, top }
}
async function show() {
  clearTimeout(timer)
  if (open.value || !count.value) return
  open.value = true
  place()
  await nextTick()
  place()
}
function hide() {
  clearTimeout(timer)
  open.value = false
}
function soon(then: () => void, ms: number) {
  clearTimeout(timer)
  timer = setTimeout(then, ms)
}
function toggle() {
  if (open.value) hide()
  else void show()
}
function keep() {
  clearTimeout(timer)
}
function gotIt() {
  for (const id of shown.value) markSeen(id)
  hide()
}
function onKey(e: KeyboardEvent) {
  if (e.key !== 'Escape' || !open.value) return
  e.stopPropagation()
  e.preventDefault()
  const inCard = card.value?.contains(document.activeElement)
  hide()
  if (inCard && props.id) el.value?.focus()
}
function onScroll() {
  if (open.value) place()
}
function listen(on: boolean) {
  if (on) {
    window.addEventListener('scroll', onScroll, true)
    window.addEventListener('resize', onScroll)
    document.addEventListener('keydown', onKey, true)
  } else {
    window.removeEventListener('scroll', onScroll, true)
    window.removeEventListener('resize', onScroll)
    document.removeEventListener('keydown', onKey, true)
  }
}
watch(open, listen)
watch(count, (n) => n || hide())
onBeforeUnmount(() => {
  hide()
  listen(false)
})
</script>

<template>
  <Transition name="newpip">
    <!-- A thing's own pip is a button of its own. A page's sits in the way there, itself a button, so it's only drawn
         and shows its card under the pointer. -->
    <button
      v-if="count && id"
      ref="el"
      type="button"
      class="newpip"
      :data-new="id"
      :aria-label="t('news-pip', { count })"
      :aria-expanded="open"
      :aria-controls="open ? cardId : undefined"
      @pointerenter="soon(show, OPEN_AFTER)"
      @pointerleave="soon(hide, CLOSE_AFTER)"
      @click.stop.prevent="toggle"
    />
    <span
      v-else-if="count"
      ref="el"
      class="newpip"
      :class="{ many: count > 1 }"
      role="img"
      :aria-label="t('news-pip', { count })"
      :data-count="count > 1 ? formatNumber(count) : undefined"
      @pointerenter="soon(show, OPEN_AFTER)"
      @pointerleave="soon(hide, CLOSE_AFTER)"
    />
  </Transition>
  <Teleport to="body">
    <Transition name="newcard">
      <div
        v-if="open && things.length"
        :id="cardId"
        ref="card"
        class="newcard"
        role="dialog"
        :aria-label="t('news-title')"
        :style="{ left: `${at.left}px`, top: `${at.top}px` }"
        @pointerenter="keep"
        @pointerleave="soon(hide, CLOSE_AFTER)"
        @click.stop
      >
        <span class="kicker">{{ t('news-title') }}</span>
        <div v-for="n in things" :key="n.id" class="thing">
          <strong>{{ n.title() }}</strong>
          <p>{{ n.body() }}</p>
        </div>
        <button type="button" class="btn ghost got" @click="gotIt">{{ t('news-got-it') }}</button>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.newpip {
  --pip: #f2c55c;
  display: inline-grid;
  place-items: center;
  flex: none;
  width: 10px;
  height: 10px;
  margin-inline: 0.35rem;
  padding: 0;
  border: 0;
  border-radius: 999px;
  background: var(--pip);
  /* A thin darker edge keeps the gold visible on the lightest themes. */
  outline: 1px solid rgb(150 105 10 / 0.35);
  vertical-align: middle;
  animation: glint 2.2s ease-in-out infinite;
}
button.newpip {
  cursor: help;
  /* A bigger target than the dot itself. */
  position: relative;
}
button.newpip::before {
  content: '';
  position: absolute;
  inset: -7px;
  border-radius: 999px;
}
button.newpip:focus-visible {
  outline: 2px solid var(--accent, #f2c55c);
  outline-offset: 2px;
}
/* Drawn rather than written, so the count isn't part of the text of what it marks (a button's name stays its own). */
.newpip.many::after {
  content: attr(data-count);
}
.newpip.many {
  width: auto;
  min-width: 1.15rem;
  height: 1.15rem;
  padding: 0 0.3rem;
  font-size: 0.68rem;
  font-weight: 700;
  line-height: 1;
  color: #3b2a05;
  font-variant-numeric: tabular-nums;
}
@keyframes glint {
  0%, 100% { box-shadow: 0 0 0 0 rgb(242 197 92 / 0.55); }
  60% { box-shadow: 0 0 0 7px rgb(242 197 92 / 0); }
}
@media (prefers-reduced-motion: reduce) {
  .newpip { animation: none; }
}
.newpip-leave-active { transition: opacity var(--dur-2, 0.3s), scale var(--dur-2, 0.3s); }
.newpip-leave-to { opacity: 0; scale: 0.4; }

.newcard {
  position: fixed;
  z-index: 1000;
  display: grid;
  gap: 0.55rem;
  width: min(18rem, calc(100vw - 16px));
  padding: 0.75rem 0.85rem 0.7rem;
  border: 1px solid var(--line);
  border-top: 2px solid #f2c55c;
  border-radius: 8px;
  background: var(--panel);
  color: var(--text);
  box-shadow: var(--elev-3);
  font-size: var(--step--1);
  font-weight: 400;
  text-align: start;
  text-transform: none;
  letter-spacing: normal;
  white-space: normal;
}
.kicker {
  color: color-mix(in srgb, #f2c55c 65%, var(--text));
  font-size: 0.68rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.thing { display: grid; gap: 0.15rem; }
.thing + .thing { padding-top: 0.5rem; border-top: 1px solid var(--line); }
.thing strong { font-weight: 650; }
.thing p { margin: 0; color: var(--muted); line-height: 1.4; }
.got { justify-self: end; font-size: var(--step--1); }
.newcard-enter-active, .newcard-leave-active { transition: opacity var(--dur-1, 0.12s), translate var(--dur-1, 0.12s); }
.newcard-enter-from, .newcard-leave-to { opacity: 0; translate: 0 -3px; }
@media (prefers-reduced-motion: reduce) {
  .newcard-enter-active, .newcard-leave-active { transition: none; }
}
</style>
