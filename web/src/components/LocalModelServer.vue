<script setup lang="ts">
// The local model server (llama.cpp in router mode) in the AI settings: what it has downloaded, the models Hoard
// suggests, and downloading, deleting or using one there. Using one saves it as the tagging model straight away.
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { ApiError, api, type LocalModel, type LocalModels, type SettingsPatch } from '../api'
import { formatNumber, t } from '../i18n'
import { confirmDialog, showError } from '../ui'
import { trimUrl, useSettings } from './settingsContext'

const props = defineProps<{ url: string }>()
/** Whether the server answered, or null before it has (or without an address): when it hasn't, the panel asks for a
 *  model by name. */
const emit = defineEmits<{ reachable: [boolean | null] }>()
const { s, busy, save, unauthorized } = useSettings()

const local = ref<LocalModels | null>(null)
const localState = ref<'idle' | 'loading' | 'ok'>('idle')
/** The model being downloaded or deleted from here, while the server works on it. */
const localBusy = ref('')
const otherModel = ref('')
let localAsk = 0
let localTimer: number | undefined
async function load(action: 'list' | 'download' | 'delete' = 'list', model?: string): Promise<void> {
  window.clearTimeout(localTimer)
  const url = trimUrl(props.url)
  const ask = ++localAsk
  if (!url) {
    local.value = null
    localState.value = 'idle'
    return
  }
  if (action === 'list' && !local.value) localState.value = 'loading'
  if (model) localBusy.value = model
  try {
    const res = await api.llmLocal(url, action, model)
    if (ask !== localAsk) return
    local.value = res
    if (action === 'download') otherModel.value = ''
  } catch (e) {
    if (ask !== localAsk) return
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
    if (action !== 'list') {
      localBusy.value = ''
      return load()
    }
  }
  localState.value = 'ok'
  localBusy.value = ''
  // Keep progress moving while something downloads or starts up.
  if (local.value?.models.some((m) => ['downloading', 'loading'].includes(m.status))) {
    localTimer = window.setTimeout(() => void load(), 1500)
  }
}
const localIds = computed(() => new Set(local.value?.models.map((m) => m.id) ?? []))
const notDownloaded = computed(() => local.value?.suggested.filter((m) => !localIds.value.has(m.id)) ?? [])
const nameOf = (id: string) => local.value?.suggested.find((m) => m.id === id)?.name ?? id
/** The model AI tagging uses right now, if it's on this server. */
const inUse = (m: LocalModel) => !!s.value && s.value.llm.model === m.id && trimUrl(s.value.llm.url) === trimUrl(props.url)
const gb = (n: number) => {
  const digits = n >= 1e10 ? 0 : 1
  return `${formatNumber(n / 1e9, { minimumFractionDigits: digits, maximumFractionDigits: digits })} GB`
}
function localStatus(m: LocalModel): string {
  if (m.status === 'downloading') {
    const [done, total] = m.progress ?? [0, 0]
    return total
      ? t('settings-local-downloading-progress', { percent: Math.floor((done / total) * 100), done: gb(done), total: gb(total) })
      : t('settings-local-downloading')
  }
  if (m.status === 'incomplete') return t('settings-local-incomplete')
  if (m.failed) return t('settings-local-failed')
  if (inUse(m)) return m.status === 'loaded' ? t('settings-local-in-use-running') : t('settings-local-in-use')
  if (m.status === 'loading') return t('settings-local-starting')
  if (m.status === 'loaded') return t('settings-local-running')
  return m.vision ? t('settings-local-ready-images') : t('settings-local-ready-text')
}
const ready = (m: LocalModel) => !['downloading', 'incomplete'].includes(m.status)
function useLocal(m: LocalModel) {
  const patch: SettingsPatch = { llm_url: trimUrl(props.url), llm_model: m.id, llm_vision: m.vision, llm_api_key: '' }
  // Small models tag better a few assets at a time; keep a batch size someone chose.
  if (!s.value?.llm.batch.saved) patch.llm_batch = 12
  void save(patch, t('settings-ai-saved', { service: nameOf(m.id) }))
}
async function deleteLocal(m: LocalModel) {
  const ok = await confirmDialog({
    title: t('settings-local-delete-title', { name: nameOf(m.id) }),
    body: inUse(m)
      ? t('settings-local-delete-in-use')
      : m.status === 'downloading'
        ? t('settings-local-delete-downloading')
        : t('settings-local-delete-body'),
    confirm: t('settings-local-delete'),
    danger: true,
  })
  if (ok) void load('delete', m.id)
}
function downloadOther() {
  const id = otherModel.value.trim()
  if (id) void load('download', id)
}
watch(() => local.value?.reachable ?? null, (r) => emit('reachable', r), { immediate: true })

onMounted(() => void load())
onBeforeUnmount(() => {
  window.clearTimeout(localTimer)
  localAsk++
})
defineExpose({ load })
</script>

<template>
  <template v-if="!local?.reachable">
    <span />
    <p v-if="localState !== 'ok'" class="status" data-state="looking" role="status">
      <span class="dot" aria-hidden="true" />
      <span class="text">{{ t('settings-ai-connecting') }}</span>
    </p>
    <div v-else-if="local" class="local-off">
      <p class="status" data-state="unavailable" role="status">
        <span class="dot" aria-hidden="true" />
        <span class="text">{{ local.error }}</span>
      </p>
      <p class="hint inline">
        {{ t('settings-local-start-before') }}<code>docker compose up -d llm</code>{{ t('settings-local-start-or') }}<code>cargo xtask llm</code>{{ t('settings-local-start-developing') }}<button type="button" class="link" @click="load()">{{ t('settings-local-start-retry') }}</button>{{ t('settings-local-start-after') }}
      </p>
    </div>
  </template>
  <div v-else class="local">
    <ul v-if="local.models.length" class="mlist" :aria-label="t('settings-local-list')">
      <li v-for="m in local.models" :key="m.id" :class="{ on: inUse(m) }">
        <span class="what">
          <strong>{{ nameOf(m.id) }}</strong>
          <small class="mono">{{ m.id }}</small>
          <span
            class="status"
            :data-state="m.status === 'downloading' ? 'downloading' : m.status === 'loading' ? 'looking' : m.failed || m.status === 'incomplete' ? 'unavailable' : inUse(m) ? 'ready' : 'off'"
          >
            <span class="dot" aria-hidden="true" />
            <span class="text">{{ localStatus(m) }}</span>
          </span>
          <progress v-if="m.status === 'downloading' && m.progress?.[1]" :value="m.progress[0]" :max="m.progress[1]" />
        </span>
        <span class="acts">
          <button v-if="m.status === 'incomplete'" type="button" class="btn" :disabled="!!localBusy" @click="load('download', m.id)">{{ t('settings-local-resume') }}</button>
          <button v-else-if="ready(m) && !inUse(m)" type="button" class="btn primary" :disabled="busy || !!localBusy" @click="useLocal(m)">{{ t('settings-use') }}</button>
          <button type="button" class="btn ghost" :disabled="!!localBusy" @click="deleteLocal(m)">
            {{ localBusy === m.id ? t('settings-local-deleting') : m.status === 'downloading' ? t('settings-local-cancel') : t('settings-local-delete') }}
          </button>
        </span>
      </li>
    </ul>
    <p v-else class="hint inline">{{ t('settings-local-none') }}</p>
    <ul v-if="notDownloaded.length" class="mlist" :aria-label="t('settings-local-catalog')">
      <li v-for="m in notDownloaded" :key="m.id">
        <span class="what">
          <strong>{{ m.name }}</strong>
          <small>{{ t('settings-local-size', { size: m.size_gb, license: m.license }) }}</small>
          <small>{{ m.note }}</small>
        </span>
        <span class="acts">
          <button type="button" class="btn" :disabled="!!localBusy" @click="load('download', m.id)">
            {{ localBusy === m.id ? t('settings-local-starting') : t('settings-local-download') }}
          </button>
        </span>
      </li>
    </ul>
    <div class="key">
      <input
        v-model="otherModel"
        class="field mono"
        type="text"
        spellcheck="false"
        :aria-label="t('settings-local-other')"
        :placeholder="t('settings-local-other-placeholder')"
        @keydown.enter.prevent="downloadOther"
      />
      <button type="button" class="btn" :disabled="!otherModel.trim() || !!localBusy" @click="downloadOther">{{ t('settings-local-download') }}</button>
    </div>
  </div>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.local-off { display: grid; gap: 0.35rem; }
.local { grid-column: 1 / -1; display: grid; gap: 0.6rem; }
.mlist { display: grid; gap: 0.4rem; margin: 0; padding: 0; list-style: none; }
.mlist li {
  display: flex; gap: 0.8rem; align-items: center; padding: 0.6rem 0.75rem; border: 1px solid var(--line); border-radius: 8px;
}
.mlist li.on { border-color: var(--card-on-line); background: var(--card-on); }
.mlist .what { flex: 1; min-width: 0; display: grid; gap: 0.15rem; }
.mlist small { color: var(--muted); font-size: inherit; overflow-wrap: anywhere; }
.mlist .status { margin: 0.1rem 0 0; }
.mlist progress { width: 100%; height: 0.35rem; accent-color: var(--accent); }
.mlist .acts { display: flex; gap: 0.4rem; flex: none; }
@media (max-width: 30rem) { .mlist li { flex-direction: column; align-items: stretch; } }
</style>
