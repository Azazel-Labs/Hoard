<script setup lang="ts">
// Experimental: features still being worked on, each off until turned on here, in this browser.
import { t } from '../i18n'
import { EXPERIMENTS, setTrying, trying } from '../experiments'
</script>

<template>
  <div class="body">
    <section>
      <p class="what">{{ t('experiments-what') }}</p>
      <p v-if="!EXPERIMENTS.length" class="hint">{{ t('experiments-none') }}</p>
      <div v-for="x in EXPERIMENTS" :key="x.id" class="switches experiment">
        <label class="check">
          <input type="checkbox" :checked="trying(x.id)" @change="setTrying(x.id, ($event.target as HTMLInputElement).checked)" />
          {{ x.label }}
        </label>
        <p class="what">{{ x.what }}</p>
      </div>
    </section>
  </div>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.body { display: grid; gap: 1rem; }
.experiment { padding-top: 0.8rem; border-top: 1px solid var(--line); }
/* Under its name, past the switch. */
.experiment .what { margin: 0 0 0 2.75rem; }
</style>
