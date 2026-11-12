<script setup lang="ts">
// The little turn-back arrow beside a setting that isn't at its default. Its tooltip says what the default is.
defineProps<{ to: string; disabled?: boolean }>()
defineEmits<{ reset: [] }>()
</script>

<template>
  <button
    type="button"
    class="reset"
    :title="`Reset to the default: ${to}`"
    :aria-label="`Reset to the default, ${to}`"
    :disabled="disabled"
    @click="$emit('reset')"
  >
    <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
      <path d="M3 8a5 5 0 1 0 1.6-3.7" />
      <path d="M3.2 2.2v3h3" />
    </svg>
  </button>
</template>

<style scoped>
.reset {
  display: inline-grid; place-items: center; flex: none; width: 1.75rem; height: 1.75rem; padding: 0;
  border: 1px solid transparent; border-radius: var(--r-chip); background: none; color: var(--muted); cursor: pointer;
  transition: color var(--dur-1), background-color var(--dur-1);
}
.reset:hover:not(:disabled) { color: var(--text); background: var(--raised); }
.reset:hover:not(:disabled) svg { rotate: -40deg; }
.reset svg { transition: rotate var(--dur-3) var(--ease-pop); }
.reset:active:not(:disabled) { scale: 0.92; }
.reset:disabled { opacity: 0.4; cursor: default; }
</style>
