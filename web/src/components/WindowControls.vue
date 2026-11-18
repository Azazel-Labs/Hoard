<script setup lang="ts">
// The desktop app's window has no frame of the system's: the page is the title bar. Its empty space (and the
// header's) moves the window, a double-click maximizes it, and these buttons stand in for the system's at the top
// right. On a Mac the system's own buttons stay at the top left, over the page, and this draws nothing.
import { onBeforeUnmount, onMounted, ref } from 'vue'
import type { Window } from '@tauri-apps/api/window'
import { t } from '../i18n'

const mac = navigator.userAgent.includes('Mac')
const maximized = ref(false)
/** The height of the header's first row, which the buttons fill; the default where there's no header. */
const height = ref(40)
let win: Window | undefined
let observer: ResizeObserver | undefined

async function sync() {
  if (win) maximized.value = await win.isMaximized()
}

/** The brand sits in the middle of the header's first row, so the row is as tall as the brand's top plus bottom. */
function measure() {
  const brand = document.querySelector('header.top .brand')
  if (!brand) return
  const r = brand.getBoundingClientRect()
  if (r.height) height.value = Math.round(r.top + r.bottom)
}

onMounted(async () => {
  const root = document.documentElement
  root.classList.add(mac ? 'framed-mac' : 'framed')
  for (const el of [root, document.body, document.getElementById('app')]) el?.setAttribute('data-tauri-drag-region', '')
  if (mac) return
  observer = new ResizeObserver(measure)
  const watch = () => {
    const top = document.querySelector('header.top')
    if (top) observer?.observe(top)
    measure()
  }
  new MutationObserver(watch).observe(document.getElementById('app') ?? document.body, { childList: true })
  watch()
  win = (await import('@tauri-apps/api/window')).getCurrentWindow()
  addEventListener('resize', sync)
  await sync()
})

onBeforeUnmount(() => {
  removeEventListener('resize', sync)
  observer?.disconnect()
})
</script>

<template>
  <div v-if="!mac" class="controls" :style="{ height: `${height}px` }">
    <button type="button" :title="t('window-minimize')" :aria-label="t('window-minimize')" @click="win?.minimize()">
      <svg viewBox="0 0 10 10" aria-hidden="true"><path d="M0 5.5h10" /></svg>
    </button>
    <button
      type="button"
      :title="maximized ? t('window-restore') : t('window-maximize')"
      :aria-label="maximized ? t('window-restore') : t('window-maximize')"
      @click="win?.toggleMaximize()"
    >
      <svg v-if="maximized" viewBox="0 0 10 10" aria-hidden="true"><path d="M2.5 2.5V.5h7v7h-2M.5 2.5h7v7h-7z" /></svg>
      <svg v-else viewBox="0 0 10 10" aria-hidden="true"><path d="M.5.5h9v9h-9z" /></svg>
    </button>
    <button type="button" class="close" :title="t('window-close')" :aria-label="t('window-close')" @click="win?.close()">
      <svg viewBox="0 0 10 10" aria-hidden="true"><path d="M0 0l10 10M10 0L0 10" /></svg>
    </button>
  </div>
</template>

<style scoped>
.controls { position: fixed; top: 0; right: 0; z-index: 100; display: flex; }
button {
  display: grid; place-items: center; width: 46px; height: 100%; padding: 0; border: 0; border-radius: 0;
  background: transparent; color: var(--text); cursor: default;
}
button:hover { background: color-mix(in srgb, var(--text) 10%, transparent); }
button:active { background: color-mix(in srgb, var(--text) 18%, transparent); }
button.close:hover { background: #c42b1c; color: #fff; }
button.close:active { background: color-mix(in srgb, #c42b1c 80%, transparent); color: #fff; }
button:focus-visible { outline: 2px solid var(--accent); outline-offset: -2px; }
svg { width: 10px; height: 10px; fill: none; stroke: currentColor; stroke-width: 1; shape-rendering: crispEdges; }
</style>
