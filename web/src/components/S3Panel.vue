<script setup lang="ts">
// Settings → Library locations → S3 storage: the service `s3://bucket` locations are in, and the keys for it.
import { computed, onMounted, reactive, ref } from 'vue'
import { ApiError, api, type S3Setting, type SettingsPatch } from '../api'
import { t } from '../i18n'
import { showError, showOk } from '../ui'

const emit = defineEmits<{ unauthorized: []; changed: [] }>()

const s3 = ref<S3Setting | null>(null)
const form = reactive({ endpoint: '', region: '', access_key: '', secret_key: '', headers: '', removeSecret: false, removeHeaders: false })
const busy = ref(false)

function take(next: S3Setting) {
  s3.value = next
  Object.assign(form, {
    endpoint: next.endpoint,
    region: next.region,
    access_key: next.access_key,
    secret_key: '',
    headers: '',
    removeSecret: false,
    removeHeaders: false,
  })
}

onMounted(async () => {
  try {
    take((await api.settings()).s3)
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
  }
})

const keptSecret = computed(() => !!s3.value?.has_secret_key && !form.removeSecret)
const keptHeaders = computed(() => !!s3.value?.has_headers && !form.removeHeaders)
const dirty = computed(() => {
  const now = s3.value
  if (!now) return false
  return (
    form.endpoint.trim() !== now.endpoint ||
    form.region.trim() !== now.region ||
    form.access_key.trim() !== now.access_key ||
    form.secret_key !== '' ||
    form.headers.trim() !== '' ||
    form.removeSecret ||
    form.removeHeaders
  )
})

async function save() {
  busy.value = true
  try {
    const patch: SettingsPatch = {
      s3_endpoint: form.endpoint.trim(),
      s3_region: form.region.trim(),
      s3_access_key: form.access_key.trim(),
    }
    if (form.secret_key) patch.s3_secret_key = form.secret_key.trim()
    else if (form.removeSecret) patch.s3_secret_key = ''
    if (form.headers.trim()) patch.s3_headers = form.headers.trim()
    else if (form.removeHeaders) patch.s3_headers = ''
    take((await api.saveSettings(patch)).s3)
    showOk(t('locations-s3-saved'))
    emit('changed')
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError(t('locations-failed', { what: t('locations-s3-save-failed'), error: (e as Error).message }))
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <section v-if="s3" data-testid="s3-storage">
    <div class="sh"><h3>{{ t('locations-s3-title') }}</h3></div>
    <p class="hint">{{ t('locations-s3-hint') }}</p>
    <p v-if="!s3.built" class="hint">{{ t('error-not-built', { feature: 'S3' }) }}</p>
    <form v-else class="grid" @submit.prevent="save">
      <label class="lbl" for="s3-endpoint">{{ t('locations-s3-endpoint') }}</label>
      <input id="s3-endpoint" v-model="form.endpoint" class="field mono" type="url" spellcheck="false" :placeholder="t('locations-s3-endpoint-placeholder')" :disabled="busy" />

      <label class="lbl" for="s3-region">{{ t('locations-s3-region') }}</label>
      <input id="s3-region" v-model="form.region" class="field mono short" type="text" spellcheck="false" placeholder="us-east-1" :disabled="busy" />

      <label class="lbl" for="s3-access">{{ t('locations-s3-access-key') }}</label>
      <input id="s3-access" v-model="form.access_key" class="field mono" type="text" spellcheck="false" autocomplete="off" :disabled="busy" />

      <label class="lbl" for="s3-secret">{{ t('locations-s3-secret-key') }}</label>
      <div class="key">
        <input
          id="s3-secret"
          v-model="form.secret_key"
          class="field mono"
          type="password"
          autocomplete="off"
          :placeholder="keptSecret ? t('locations-s3-kept') : ''"
          :disabled="busy"
          @input="form.removeSecret = false"
        />
        <button v-if="keptSecret && !form.secret_key" type="button" class="link" @click="form.removeSecret = true">{{ t('locations-remove') }}</button>
      </div>

      <label class="lbl" for="s3-headers">{{ t('locations-s3-headers') }}</label>
      <div class="key">
        <textarea
          id="s3-headers"
          v-model="form.headers"
          class="field mono"
          rows="2"
          spellcheck="false"
          autocomplete="off"
          :placeholder="keptHeaders ? t('locations-s3-kept') : t('locations-s3-headers-placeholder')"
          :disabled="busy"
          @input="form.removeHeaders = false"
        />
        <button v-if="keptHeaders && !form.headers" type="button" class="link" @click="form.removeHeaders = true">{{ t('locations-remove') }}</button>
      </div>

      <span />
      <p class="hint flat">{{ t('locations-s3-secrets') }}</p>

      <span />
      <div>
        <button class="btn primary" type="submit" :disabled="busy || !dirty">{{ t('locations-save') }}</button>
      </div>
    </form>
  </section>
</template>

<style scoped>
.sh { display: flex; align-items: center; gap: 0.4rem; min-height: 1.75rem; margin-bottom: 0.3rem; }
.hint { margin: 0 0 0.6rem; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.hint.flat { margin: 0; }
.grid { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.6rem 0.9rem; align-items: baseline; font-size: var(--step--1); }
.lbl { color: var(--muted); }
.grid > .field, .key .field { width: 100%; max-width: 32rem; min-width: 0; }
.grid > .field.short { max-width: 12rem; }
.key { display: flex; gap: 0.6rem; align-items: baseline; }
.key .field { flex: 1; }
textarea.field { resize: vertical; font: inherit; }
.link { background: none; border: 0; padding: 0; color: var(--muted); text-decoration: underline; cursor: pointer; font: inherit; }
@media (max-width: 30rem) {
  .grid { grid-template-columns: minmax(0, 1fr); gap: 0.35rem; }
}
</style>
