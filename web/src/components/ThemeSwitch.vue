<script setup lang="ts">
// Light, dark or the system's, in the top bar; picking one spreads the new theme out from the button.
import { t } from '../i18n'
import { prefs } from '../prefs'
import { SCHEMES, setTheme } from '../theme'
</script>

<template>
  <div class="theme" role="group" :aria-label="t('app-theme')">
    <button
      v-for="th in SCHEMES"
      :key="th.value"
      type="button"
      :aria-pressed="prefs.theme === th.value"
      :title="th.label()"
      @click="setTheme(th.value, $event)"
    >
      <svg viewBox="0 0 16 16" width="15" height="15" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
        <template v-if="th.value === 'light'">
          <circle cx="8" cy="8" r="3" />
          <path d="M8 1.5v1.6M8 12.9v1.6M1.5 8h1.6M12.9 8h1.6M3.4 3.4l1.1 1.1M11.5 11.5l1.1 1.1M3.4 12.6l1.1-1.1M11.5 4.5l1.1-1.1" />
        </template>
        <template v-else-if="th.value === 'dark'">
          <path d="M13.2 10.1A5.6 5.6 0 0 1 5.9 2.8a5.6 5.6 0 1 0 7.3 7.3Z" />
        </template>
        <template v-else>
          <rect x="1.8" y="2.5" width="12.4" height="8.5" rx="1.2" />
          <path d="M5.5 14h5M8 11v3" />
        </template>
      </svg>
      <span class="visually-hidden">{{ th.label() }}</span>
    </button>
  </div>
</template>

<style scoped>
.theme { display: inline-flex; padding: 2px; gap: 1px; border: 1px solid var(--line); border-radius: var(--r-chip); }
.theme button {
  display: grid; place-items: center; width: 1.8rem; height: 1.6rem; padding: 0;
  border: 0; border-radius: 3px; background: transparent; color: var(--faint); cursor: pointer;
}
.theme button:hover { color: var(--text); }
.theme button[aria-pressed='true'] { background: var(--raised); color: var(--text); }
/* Fingers need bigger targets than pointers. */
@media (pointer: coarse) {
  .theme button { min-height: 44px; min-width: 44px; }
}
</style>
