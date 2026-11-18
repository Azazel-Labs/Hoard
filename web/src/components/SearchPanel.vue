<script setup lang="ts">
// Image and sound search: each off until turned on, and image search on this server or a hosted service.
import { computed, reactive } from 'vue'
import type { HostedSearch, SearchModelSetting, SettingsPatch } from '../api'
import { t } from '../i18n'
import ResetButton from './ResetButton.vue'
import { onOff, trimUrl, useSettings } from './settingsContext'

const { s, busy, save, onTake } = useSettings()

const toggleClip = (on: boolean) => save({ clip: on }, on ? t('settings-clip-saved-on') : t('settings-clip-saved-off'))
const toggleClap = (on: boolean) => save({ clap: on }, on ? t('settings-clap-saved-on') : t('settings-clap-saved-off'))

/** Image search through a hosted service, as typed; `on` is the "a hosted service" choice, saved or not. */
const hosted = reactive({ on: false, provider: '', model: '', url: '', key: '' })
const hostedSaved = computed(() => s.value?.clip.hosted)
const hostedProvider = computed(() => hostedSaved.value?.providers.find((p) => p.id === hosted.provider))
/** The saved key is only ever sent to the service and address it was saved for. */
const hostedKeptKey = computed(() => {
  const h = hostedSaved.value
  return !!h?.has_key && h.provider === hosted.provider && trimUrl(h.url) === trimUrl(hosted.url)
})
const hostedDirty = computed(() => {
  const h = hostedSaved.value
  return (
    !h?.provider || h.provider !== hosted.provider || h.model !== (hosted.model.trim() || hostedProvider.value?.model) ||
    trimUrl(h.url) !== trimUrl(hosted.url) || !!hosted.key.trim()
  )
})
function takeHosted(h: HostedSearch | undefined) {
  Object.assign(hosted, { on: !!h?.provider, provider: h?.provider || h?.providers[0]?.id || '', model: h?.model ?? '', url: h?.url ?? '', key: '' })
}
onTake((next) => takeHosted(next.clip.hosted))
function pickHostedProvider(id: string) {
  // A model left at the last service's own follows to the new service's.
  if (hosted.model.trim() === hostedProvider.value?.model) hosted.model = ''
  hosted.provider = id
}
function runHere() {
  hosted.on = false
  if (hostedSaved.value?.provider) {
    void save({ clip_hosted: '', clip_hosted_model: '', clip_hosted_key: '', clip_hosted_url: '' }, t('settings-clip-saved-here'))
  }
}
function saveHosted() {
  const p = hostedProvider.value
  if (!p) return
  const patch: SettingsPatch = { clip_hosted: p.id, clip_hosted_model: hosted.model.trim(), clip_hosted_url: trimUrl(hosted.url) }
  if (hosted.key.trim()) patch.clip_hosted_key = hosted.key.trim()
  void save(patch, t('settings-clip-saved-hosted', { service: p.name }))
}

function modelStatus(c: SearchModelSetting, which: 'clip' | 'clap'): string {
  switch (c.state) {
    case 'off': return t('settings-status-off')
    case 'looking': return t('settings-clip-looking')
    case 'downloading': return t('settings-clip-downloading')
    case 'ready':
      if (!c.total) return which === 'clip' ? t('settings-clip-ready-empty') : t('settings-clap-ready-empty')
      if (c.done >= c.total) {
        const count = c.total
        return which === 'clip' ? t('settings-clip-ready-all', { count }) : t('settings-clap-ready-all', { count })
      }
      const { done, total } = c
      return which === 'clip' ? t('settings-clip-learning', { done, total }) : t('settings-clap-learning', { done, total })
    case 'unavailable':
      if (which === 'clip' && c.hosted?.provider) {
        const service = c.hosted.providers.find((p) => p.id === c.hosted?.provider)?.name ?? c.hosted.provider
        return t('settings-clip-hosted-unavailable', { service, detail: c.detail })
      }
      return t('settings-clip-unavailable', { detail: c.detail })
  }
}
</script>

<template>
  <section>
    <div class="sh">
      <h3>{{ t('settings-clip-heading') }}</h3>
      <ResetButton v-if="s.clip.saved" :to="onOff(s.clip.default)" :disabled="busy" @reset="save({ clip: null }, t('settings-clip-reset'))" />
    </div>
    <p class="what">{{ t('settings-clip-what') }}</p>
    <div class="switches">
      <div class="opt-row">
        <label class="check">
          <input
            type="checkbox"
            data-testid="clip-toggle"
            :checked="s.clip.value"
            :disabled="busy"
            @change="toggleClip(($event.target as HTMLInputElement).checked)"
          />
          {{ t('settings-clip-toggle') }}
        </label>
      </div>
    </div>
    <form v-if="s.clip.hosted" class="llm where" @submit.prevent="saveHosted">
      <span id="clip-where" class="label">{{ t('settings-clip-where') }}</span>
      <div class="choice" role="radiogroup" aria-labelledby="clip-where">
        <label class="check">
          <input type="radio" name="clip-where" :checked="!hosted.on" :disabled="busy" @change="runHere" />
          {{ t('settings-clip-here') }}
        </label>
        <label class="check">
          <input type="radio" name="clip-where" data-testid="clip-hosted" :checked="hosted.on" :disabled="busy" @change="hosted.on = true" />
          {{ t('settings-clip-hosted') }}
        </label>
      </div>
      <template v-if="hosted.on">
        <span />
        <p class="leaves" role="note" data-testid="clip-hosted-warning">
          {{ t('settings-clip-hosted-warning', { service: hostedProvider?.name ?? '' }) }}
        </p>
        <label for="clip-provider">{{ t('settings-ai-service') }}</label>
        <select id="clip-provider" class="field" :value="hosted.provider" :disabled="busy" @change="pickHostedProvider(($event.target as HTMLSelectElement).value)">
          <option v-for="p in s.clip.hosted.providers" :key="p.id" :value="p.id">{{ p.name }}</option>
        </select>
        <label for="clip-model">{{ t('settings-ai-model') }}</label>
        <input id="clip-model" v-model="hosted.model" class="field mono" type="text" spellcheck="false" autocomplete="off" :placeholder="hostedProvider?.model" />
        <label for="clip-key">{{ t('settings-ai-key') }}</label>
        <div class="key">
          <input
            id="clip-key"
            v-model="hosted.key"
            class="field mono"
            type="password"
            autocomplete="off"
            :placeholder="hostedKeptKey ? t('settings-ai-key-saved') : t('settings-ai-key-paste')"
          />
        </div>
        <label for="clip-url">{{ t('settings-ai-url') }}</label>
        <input id="clip-url" v-model="hosted.url" class="field mono" type="url" spellcheck="false" :placeholder="hostedProvider?.url" />
        <span />
        <div class="row">
          <button class="btn primary" type="submit" :disabled="busy || !hostedDirty || !(hosted.key.trim() || hostedKeptKey)">{{ t('settings-save') }}</button>
        </div>
      </template>
    </form>
    <p v-if="s.clip.hosted?.provider" class="hint">{{ t('settings-clip-hosted-hint') }}</p>
    <p v-else class="hint">{{ t('settings-clip-hint', { size: Math.round(s.clip.download_bytes / 1e6) }) }}</p>
    <p class="status" :data-state="s.clip.state" role="status" data-testid="clip-status">
      <span class="dot" aria-hidden="true" />
      <span class="text">{{ modelStatus(s.clip, 'clip') }}</span>
    </p>
    <progress v-if="s.clip.state === 'ready' && s.clip.total && s.clip.done < s.clip.total" class="learn" :value="s.clip.done" :max="s.clip.total" />
    <p v-if="s.clip.relearning" class="hint" data-testid="clip-relearning">{{ t('settings-clip-relearning') }}</p>
    <div class="sh second">
      <h3>{{ t('settings-clap-heading') }}</h3>
      <ResetButton v-if="s.clap.saved" :to="onOff(s.clap.default)" :disabled="busy" @reset="save({ clap: null }, t('settings-clap-reset'))" />
    </div>
    <p class="what">{{ t('settings-clap-what') }}</p>
    <div class="switches">
      <div class="opt-row">
        <label class="check">
          <input
            type="checkbox"
            data-testid="clap-toggle"
            :checked="s.clap.value"
            :disabled="busy"
            @change="toggleClap(($event.target as HTMLInputElement).checked)"
          />
          {{ t('settings-clap-toggle') }}
        </label>
      </div>
    </div>
    <p class="hint">{{ t('settings-clap-hint', { size: Math.round(s.clap.download_bytes / 1e6) }) }}</p>
    <p class="status" :data-state="s.clap.state" role="status" data-testid="clap-status">
      <span class="dot" aria-hidden="true" />
      <span class="text">{{ modelStatus(s.clap, 'clap') }}</span>
    </p>
    <progress v-if="s.clap.state === 'ready' && s.clap.total && s.clap.done < s.clap.total" class="learn" :value="s.clap.done" :max="s.clap.total" />
  </section>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
/* Where image search runs: under its switch, the hosted service's fields only once one is chosen. */
.where { margin-top: 0.9rem; }
.where .choice { display: flex; flex-wrap: wrap; gap: 0.4rem 1.2rem; }
.where .field:not(select) { max-width: 32rem; }
.leaves {
  margin: 0; padding: 0.5rem 0.75rem; border-radius: 8px; color: var(--text); line-height: 1.45;
  border: 1px solid color-mix(in srgb, var(--danger) 35%, transparent); background: color-mix(in srgb, var(--danger) 8%, transparent);
}
progress.learn { display: block; width: 100%; max-width: 28rem; height: 0.35rem; accent-color: var(--accent); }
</style>
