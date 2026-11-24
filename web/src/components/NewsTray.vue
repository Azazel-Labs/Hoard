<script setup lang="ts">
// The top bar's "N new" tray: what's new in Hoard still to see, each with the way to it, what's been seen lately, and
// Mark all seen (with Undo on its note) or turning the pips off altogether.
import { onMounted, ref } from 'vue'
import { formatNumber, t } from '../i18n'
import { markAllSeenWithUndo, news, type NewThing, seenLately, setPointingOut, unseen } from '../news'

const emit = defineEmits<{ close: []; go: [NewThing] }>()
const dlg = ref<HTMLDialogElement | null>(null)

function go(n: NewThing) {
  dlg.value?.close()
  emit('go', n)
}

onMounted(() => dlg.value?.showModal())
</script>

<template>
  <dialog ref="dlg" class="tray" aria-labelledby="news-title" @close="emit('close')" @click.self="dlg?.close()">
    <header>
      <h2 id="news-title">{{ t('news-title') }}</h2>
      <span v-if="unseen.length" class="count">{{ t('news-to-find', { count: unseen.length }) }}</span>
      <button type="button" class="btn ghost x" :aria-label="t('news-close')" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <div class="body">
      <ul v-if="unseen.length" class="rows">
        <li v-for="n in unseen" :key="n.id">
          <span class="mark gold" aria-hidden="true" />
          <div class="what">
            <strong>{{ n.title() }}</strong>
            <p>{{ n.body() }}</p>
            <small>{{ n.crumb().join(' › ') }}</small>
          </div>
          <button type="button" class="btn ghost there" @click="go(n)">{{ t('news-take-me') }}</button>
        </li>
      </ul>
      <p v-else class="none">{{ t('news-none') }}</p>

      <template v-if="seenLately.length">
        <h3>{{ t('news-seen-lately') }}</h3>
        <ul class="rows seen">
          <li v-for="n in seenLately" :key="n.id">
            <span class="mark ring" aria-hidden="true" />
            <div class="what">
              <strong>{{ n.title() }}</strong>
              <small>{{ n.crumb().join(' › ') }}</small>
            </div>
            <button type="button" class="btn ghost there" @click="go(n)">{{ t('news-look-again') }}</button>
          </li>
        </ul>
      </template>
    </div>

    <footer>
      <button type="button" class="linkish" @click="setPointingOut(news.off)">
        {{ news.off ? t('news-point-out') : t('news-stop-pointing') }}
      </button>
      <button type="button" class="btn primary" :disabled="!unseen.length" @click="markAllSeenWithUndo">
        {{ t('news-mark-all') }}<span v-if="unseen.length" class="n">{{ formatNumber(unseen.length) }}</span>
      </button>
    </footer>
  </dialog>
</template>

<style scoped>
.tray {
  width: min(34rem, calc(100vw - 2rem)); max-height: min(40rem, calc(100vh - 3rem)); padding: 1.1rem 1.3rem 1rem;
  border: 1px solid var(--line); border-radius: 10px; background: var(--panel); color: var(--text); box-shadow: var(--elev-3);
  /* One scrolling part, the lists, between a header and footer that stay put: not the dialog's own scroll as well. */
  overflow: hidden;
}
.tray[open] { display: flex; flex-direction: column; animation: tray-in var(--dur-3) var(--ease-pop); }
.tray::backdrop { background: var(--scrim); animation: fade var(--dur-2) var(--ease-out); }
@keyframes tray-in { from { opacity: 0; scale: 0.96; } }
@keyframes fade { from { opacity: 0; } }
@media (prefers-reduced-motion: reduce) { .tray[open], .tray::backdrop { animation: none; } }
header { flex: none; display: flex; align-items: center; gap: 0.6rem; margin-bottom: 0.7rem; }
h2 { margin: 0; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
/* Gold leaning toward the text color: brighter on dark themes, deeper on light ones, readable on both. */
.count { color: color-mix(in srgb, #f2c55c 65%, var(--text)); font-size: var(--step--1); font-weight: 600; }
.x { margin-left: auto; padding: 0.35em 0.45em; }
.body { flex: 1 1 auto; min-height: 0; overflow-y: auto; overscroll-behavior: contain; margin: 0 -0.4rem; padding: 0 0.4rem; }
.rows { list-style: none; margin: 0; padding: 0; }
.rows li { display: grid; grid-template-columns: auto minmax(0, 1fr) auto; gap: 0.75rem; align-items: start; padding: 0.6rem 0.2rem; }
.rows li + li { border-top: 1px solid var(--line); }
.what { display: grid; gap: 0.15rem; }
.what strong { font-weight: 650; }
.what p { margin: 0; color: var(--muted); font-size: var(--step--1); line-height: 1.4; }
.what small { color: var(--faint); font-size: 0.72rem; }
.mark { width: 10px; height: 10px; margin-top: 0.38rem; border-radius: 50%; }
.mark.gold { background: #f2c55c; outline: 1px solid rgb(150 105 10 / 0.35); animation: glint 2.2s ease-in-out infinite; }
.mark.ring { border: 1.5px solid var(--faint); }
@keyframes glint {
  0%, 100% { box-shadow: 0 0 0 0 rgb(242 197 92 / 0.55); }
  60% { box-shadow: 0 0 0 7px rgb(242 197 92 / 0); }
}
@media (prefers-reduced-motion: reduce) { .mark.gold { animation: none; } }
.there { white-space: nowrap; font-size: var(--step--1); }
.there::after { content: ' →'; }
.seen .what strong { font-weight: 500; color: var(--muted); }
.none { margin: 0.4rem 0 0.8rem; color: var(--muted); }
h3 { margin: 1rem 0 0.2rem; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
footer { flex: none; display: flex; align-items: center; justify-content: space-between; gap: 1rem; margin-top: 1rem; padding-top: 0.8rem; border-top: 1px solid var(--line); }
.linkish { border: 0; background: none; padding: 0; color: var(--muted); font: inherit; font-size: var(--step--1); text-decoration: underline; text-underline-offset: 2px; cursor: pointer; }
.linkish:hover { color: var(--text); }
.n { margin-left: 0.45rem; padding: 0 0.4rem; border-radius: 999px; background: rgb(0 0 0 / 0.18); font-size: 0.75em; font-variant-numeric: tabular-nums; }
</style>
