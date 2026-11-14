<script setup lang="ts">
import { siteName } from '../auth'
import { t } from '../i18n'
// The first visit to a new server: pick where the library lives before anything else.
import { onMounted, ref } from 'vue'
import { api, type LibraryLocation, type LocationList } from '../api'
import LocationForm from './LocationForm.vue'

const emit = defineEmits<{ done: [] }>()

const data = ref<LocationList | null>(null)
const error = ref('')
const made = ref<LibraryLocation | null>(null)

async function load() {
  error.value = ''
  try {
    data.value = await api.locations()
    // Someone finished the setup in another tab.
    if (data.value.locations.length) emit('done')
  } catch (e) {
    error.value = (e as Error).message
  }
}
onMounted(load)
</script>

<template>
  <div class="setup" role="dialog" aria-modal="true" aria-labelledby="setup-title">
    <div class="card">
      <p class="brand">{{ siteName() }}</p>

      <template v-if="!made">
        <h1 id="setup-title">{{ t('setup-title') }}</h1>
        <p class="lede">
          {{ t('setup-lede') }}
        </p>

        <div v-if="error" class="err" role="alert">
          <p>{{ t('setup-unreachable', { error }) }}</p>
          <button type="button" class="btn" @click="load">{{ t('setup-try-again') }}</button>
        </div>
        <p v-else-if="!data" class="loading">{{ t('setup-loading') }}</p>
        <LocationForm
          v-else
          :suggested="data.suggested"
          :kinds="data.kinds"
          first
          :submit-label="t('setup-start')"
          @created="(l) => (made = l)"
        />
      </template>

      <template v-else>
        <h1 id="setup-title">{{ t('setup-ready') }}</h1>
        <p class="lede">
          <template v-if="made.mode === 'managed'">
            {{ t('setup-managed-before') }}<span class="mono">{{ made.path }}</span>{{ t('setup-managed-after') }}
          </template>
          <template v-else>
            {{ t('setup-watched-before') }}<span class="mono">{{ made.path }}</span>{{ t('setup-watched-after') }}
          </template>
        </p>
        <button type="button" class="btn primary" autofocus @click="emit('done')">{{ t('setup-open') }}</button>
      </template>
    </div>
  </div>
</template>

<style scoped>
.setup {
  position: fixed; inset: 0; z-index: 50; display: grid; place-items: start center; overflow-y: auto; padding: clamp(1rem, 8vh, 5rem) 1rem 2rem;
  background: var(--bg);
}
.card {
  width: min(40rem, 100%); padding: 1.6rem 1.8rem 1.8rem; border: 1px solid var(--line); border-radius: 10px; background: var(--panel);
  box-shadow: var(--elev-2); animation: rise var(--dur-3) var(--ease-out);
}
@keyframes rise { from { opacity: 0; translate: 0 8px; } }
@media (prefers-reduced-motion: reduce) { .card { animation: none; } }
.brand { margin: 0 0 1rem; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.08em; text-transform: uppercase; color: var(--faint); }
h1 { margin: 0 0 0.4rem; font-size: var(--step-2); font-variation-settings: 'CASL' 0.5; line-height: 1.2; }
.lede { margin: 0 0 1.3rem; color: var(--muted); font-size: var(--step-0); line-height: 1.5; }
.lede .mono { color: var(--text); overflow-wrap: anywhere; }
.err { display: grid; gap: 0.5rem; justify-items: start; color: var(--danger); font-size: var(--step--1); }
.err p { margin: 0; }
.loading { color: var(--muted); font-size: var(--step--1); }
@media (max-width: 30rem) { .card { padding: 1.1rem 1rem 1.2rem; } }
</style>
