<script setup lang="ts">
// "?": every keyboard shortcut on one card.
import { onMounted, ref } from 'vue'

const emit = defineEmits<{ close: [] }>()
const dlg = ref<HTMLDialogElement | null>(null)
const mod = /Mac|iPhone|iPad/.test(navigator.platform) ? '⌘' : 'Ctrl'

const SECTIONS: { title: string; keys: [string[], string][] }[] = [
  {
    title: 'Anywhere',
    keys: [
      [[mod, 'K'], 'Command palette: search, jump, run'],
      [['/'], 'Search'],
      [['?'], 'This list'],
      [['Esc'], 'Close, clear the selection, or close the details'],
    ],
  },
  {
    title: 'Library',
    keys: [
      [['←', '↑', '↓', '→'], 'Move between tiles (or J and K)'],
      [['Enter'], 'Open the details'],
      [['Space'], 'Play or stop a sound'],
      [['X'], 'Select or deselect'],
      [['Shift', 'click'], 'Select a range'],
      [[mod, 'A'], 'Select everything shown'],
    ],
  },
  {
    title: 'Sound player',
    keys: [
      [['Space'], 'Play or pause'],
      [['←', '→'], 'Skip back or ahead'],
      [['Home', 'End'], 'Start or end'],
    ],
  },
  {
    title: 'Image inspector',
    keys: [
      [['+', '−'], 'Zoom in or out'],
      [['0'], 'Fit to the window'],
      [['1'], 'Actual pixels'],
      [['T'], 'Tile 3×3 to check the seams'],
      [['K'], 'Next background'],
      [['R', 'G', 'B', 'A'], 'Show one channel'],
    ],
  },
]

onMounted(() => dlg.value?.showModal())
</script>

<template>
  <dialog ref="dlg" class="keys" aria-labelledby="keys-title" @close="emit('close')" @click.self="dlg?.close()">
    <header>
      <h2 id="keys-title">Keyboard shortcuts</h2>
      <button type="button" class="btn ghost x" aria-label="Close" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>
    <div class="cols">
      <section v-for="s in SECTIONS" :key="s.title">
        <h3>{{ s.title }}</h3>
        <dl>
          <template v-for="[combo, what] in s.keys" :key="what">
            <dt><kbd v-for="k in combo" :key="k">{{ k }}</kbd></dt>
            <dd>{{ what }}</dd>
          </template>
        </dl>
      </section>
    </div>
  </dialog>
</template>

<style scoped>
.keys {
  width: min(46rem, calc(100vw - 2rem)); padding: 1.1rem 1.3rem 1.3rem; border: 1px solid var(--line); border-radius: 10px;
  background: var(--panel); color: var(--text); box-shadow: var(--elev-3);
}
.keys[open] { animation: keys-in var(--dur-3) var(--ease-pop); }
.keys::backdrop { background: var(--scrim); animation: fade var(--dur-2) var(--ease-out); }
@keyframes keys-in { from { opacity: 0; scale: 0.96; } }
@keyframes fade { from { opacity: 0; } }
header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.6rem; }
h2 { margin: 0; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.x { padding: 0.35em 0.45em; }
.cols { display: grid; grid-template-columns: repeat(auto-fit, minmax(19rem, 1fr)); gap: 1.1rem 2rem; }
h3 { margin: 0 0 0.45rem; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
dl { display: grid; grid-template-columns: max-content 1fr; gap: 0.4rem 0.9rem; margin: 0; font-size: var(--step--1); align-items: center; }
dt { display: flex; gap: 0.2rem; }
dd { margin: 0; color: var(--muted); }
kbd {
  min-width: 1.6em; padding: 0.05em 0.4em; border: 1px solid var(--line); border-bottom-width: 2px; border-radius: 4px;
  background: var(--raised); font: inherit; font-size: 0.75rem; text-align: center; font-variation-settings: 'MONO' 1;
}
</style>
