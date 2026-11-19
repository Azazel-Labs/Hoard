<script setup lang="ts">
import { t } from '../i18n'

// What covers the window while files are dragged over it: a well with marching ants to drop them in.
defineProps<{
  show: boolean
  /** How many items are being dragged, when the browser says. */
  count: number
}>()
</script>

<template>
  <Transition name="drop">
    <div v-if="show" class="drop" aria-hidden="true">
      <div class="well">
        <svg class="ants" aria-hidden="true"><rect x="1" y="1" rx="12" /></svg>
        <svg class="arrow" viewBox="0 0 24 24" width="34" height="34" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 15V4M7.5 8.5 12 4l4.5 4.5M4 15v3.5A1.5 1.5 0 0 0 5.5 20h13a1.5 1.5 0 0 0 1.5-1.5V15" />
        </svg>
        <strong>{{ count > 1 ? t('app-drop-items', { count }) : t('app-drop') }}</strong>
        <span>{{ t('app-drop-hint') }}</span>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.drop {
  position: fixed; inset: 0; z-index: 20; display: grid; place-items: center;
  background: color-mix(in srgb, var(--bg) 80%, transparent); backdrop-filter: blur(4px); pointer-events: none;
}
.well {
  position: relative; display: flex; flex-direction: column; gap: 0.4rem; align-items: center; text-align: center;
  max-width: 34rem; padding: 2.5rem 3.5rem; margin: 1rem;
}
/* Marching ants: an SVG outline so the dashes follow the rounded corners. */
.ants { position: absolute; inset: 0; width: 100%; height: 100%; overflow: visible; }
.ants rect {
  width: calc(100% - 2px); height: calc(100% - 2px);
  fill: color-mix(in srgb, var(--k-texture) 7%, transparent); stroke: var(--k-texture); stroke-width: 2;
  stroke-dasharray: 10 8; animation: ants 0.8s linear infinite;
}
@keyframes ants { to { stroke-dashoffset: -18; } }
.arrow { position: relative; color: var(--k-texture); animation: bob 1.4s var(--ease-in-out) infinite; }
@keyframes bob { 50% { translate: 0 -6px; } }
.drop strong { position: relative; font-size: var(--step-2); font-variation-settings: 'CASL' 0.6; }
.drop span { position: relative; color: var(--muted); }
.drop-enter-active { transition: opacity var(--dur-2) var(--ease-out); }
.drop-enter-active .well { transition: scale var(--dur-3) var(--ease-pop); }
.drop-leave-active { transition: opacity var(--dur-1) var(--ease-out); }
.drop-enter-from, .drop-leave-to { opacity: 0; }
.drop-enter-from .well { scale: 0.92; }
</style>
