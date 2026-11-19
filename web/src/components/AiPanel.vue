<script setup lang="ts">
// AI tagging: the service that tags packs (a hosted one, a ChatGPT plan, or the local model server), its model,
// and whether it sees thumbnails.
import { computed, onMounted, reactive, ref } from 'vue'
import { ApiError, api, type ChatGptStatus, type ModelInfo, type Settings, type SettingsPatch } from '../api'
import { t } from '../i18n'
import ChatGptAccount from './ChatGptAccount.vue'
import LocalModelServer from './LocalModelServer.vue'
import ResetButton from './ResetButton.vue'
import { trimUrl, useSettings } from './settingsContext'

const { s, busy, save, onTake, unauthorized } = useSettings()

// AI services: picking one fills in its address, so all that's left is a key (for hosted ones) and a model.
type ProviderId = 'off' | 'local' | 'chatgpt' | 'openrouter' | 'openai' | 'anthropic' | 'gemini' | 'ollama' | 'custom'
interface Provider {
  id: ProviderId
  readonly label: string
  url: string
  /** Needs an API key. */
  key?: boolean
  /** Where to get one. */
  keyUrl?: string
  /** The address can be edited (a self-hosted service). */
  editUrl?: boolean
  readonly hint: string
}
// Labels that aren't product names, and hints, are getters, so they're in the language of the moment.
const PROVIDERS: Provider[] = [
  { id: 'off', get label() { return t('settings-ai-off') }, url: '', get hint() { return t('settings-ai-off-hint') } },
  {
    id: 'local',
    get label() { return t('settings-ai-local') },
    // Filled in from the server (llm.local_url).
    url: '',
    editUrl: true,
    get hint() { return t('settings-ai-local-hint') },
  },
  {
    id: 'chatgpt',
    get label() { return t('settings-ai-chatgpt') },
    url: '',
    get hint() { return t('settings-ai-chatgpt-hint') },
  },
  {
    id: 'openrouter',
    label: 'OpenRouter',
    url: 'https://openrouter.ai/api/v1',
    key: true,
    keyUrl: 'https://openrouter.ai/keys',
    get hint() { return t('settings-ai-openrouter-hint') },
  },
  {
    id: 'openai',
    label: 'OpenAI',
    url: 'https://api.openai.com/v1',
    key: true,
    keyUrl: 'https://platform.openai.com/api-keys',
    get hint() { return t('settings-ai-openai-hint') },
  },
  {
    id: 'anthropic',
    label: 'Anthropic',
    url: 'https://api.anthropic.com/v1',
    key: true,
    keyUrl: 'https://console.anthropic.com/settings/keys',
    get hint() { return t('settings-ai-anthropic-hint') },
  },
  {
    id: 'gemini',
    label: 'Google Gemini',
    url: 'https://generativelanguage.googleapis.com/v1beta/openai',
    key: true,
    keyUrl: 'https://aistudio.google.com/apikey',
    get hint() { return t('settings-ai-gemini-hint') },
  },
  {
    id: 'ollama',
    label: 'Ollama',
    url: 'http://ollama:11434/v1',
    editUrl: true,
    get hint() { return t('settings-ai-ollama-hint') },
  },
  {
    id: 'custom',
    get label() { return t('settings-ai-custom') },
    url: '',
    editUrl: true,
    get hint() { return t('settings-ai-custom-hint') },
  },
]
function providerOf(url: string): ProviderId {
  const u = trimUrl(url)
  if (!u) return 'off'
  const exact = PROVIDERS.find((p) => p.url && trimUrl(p.url) === u)
  if (exact) return exact.id
  if (s.value && u === trimUrl(s.value.llm.local_url)) return 'local'
  if (/:11434(\/|$)/.test(u)) return 'ollama'
  // The compose service, or `cargo xtask llm` on its usual port.
  if (/^https?:\/\/(llm(:\d+)?|(localhost|127\.0\.0\.1):8081)(\/|$)/.test(u)) return 'local'
  return 'custom'
}
/** The service the saved settings use. */
const providerIn = (l: Settings['llm']): ProviderId => (l.chatgpt ? 'chatgpt' : providerOf(l.url))
const providerLabel = (l: Settings['llm']) => PROVIDERS.find((p) => p.id === providerIn(l))?.label ?? l.url

const llm = reactive({ provider: 'off' as ProviderId, url: '', model: '', key: '', removeKey: false, vision: false, batch: 1 })
onTake((next) =>
  Object.assign(llm, {
    provider: providerIn(next.llm),
    url: next.llm.url,
    model: next.llm.model,
    key: '',
    removeKey: false,
    vision: next.llm.vision,
    batch: next.llm.batch.value,
  }),
)
const provider = computed(() => PROVIDERS.find((p) => p.id === llm.provider) ?? PROVIDERS[0])
/** The saved key belongs to the saved address; a different address starts without it. */
const urlChanged = computed(() => !!s.value && trimUrl(llm.url) !== trimUrl(s.value.llm.url))
const keptKey = computed(() => !!s.value?.llm.has_key && !urlChanged.value && !llm.removeKey)
const missingKey = computed(() => !!provider.value.key && !llm.key && !keptKey.value)
const llmDirty = computed(() => {
  const l = s.value?.llm
  if (!l) return false
  if (llm.provider === 'off') return l.url !== '' || l.chatgpt
  if ((llm.provider === 'chatgpt') !== l.chatgpt) return true
  return (
    urlChanged.value || llm.model !== l.model || llm.key !== '' || llm.removeKey || llm.vision !== l.vision ||
    llm.batch !== l.batch.value
  )
})
const llmReady = computed(
  () =>
    llm.provider === 'off' ||
    ((llm.provider === 'chatgpt' ? !!gpt.value?.signed_in : !!trimUrl(llm.url) && !missingKey.value) &&
      !!llm.model.trim() &&
      llm.batch >= 1),
)

const models = ref<ModelInfo[]>([])
const modelsState = ref<{ kind: 'idle' | 'loading' | 'ok' | 'error'; text: string }>({ kind: 'idle', text: '' })
let modelsAsk = 0
async function loadModels() {
  const url = trimUrl(llm.url)
  const ask = ++modelsAsk
  if (!url || missingKey.value) {
    models.value = []
    modelsState.value = { kind: 'idle', text: '' }
    return
  }
  modelsState.value = { kind: 'loading', text: t('settings-ai-connecting') }
  try {
    // Nothing typed: the server uses the saved key, if this is the address in use.
    const key = llm.key ? llm.key.trim() : llm.removeKey || urlChanged.value ? '' : undefined
    const res = await api.llmModels(url, key)
    if (ask !== modelsAsk) return
    models.value = res.models
    const n = res.models.length
    modelsState.value = {
      kind: 'ok',
      text: n ? t('settings-ai-connected', { count: n }) : t('settings-ai-connected-none'),
    }
    if (!llm.model && n === 1) chooseModel(res.models[0].id)
  } catch (e) {
    if (ask !== modelsAsk) return
    if (e instanceof ApiError && e.status === 401) unauthorized()
    models.value = []
    modelsState.value = { kind: 'error', text: (e as Error).message }
  }
}
/** Picking a listed model also sets "send thumbnails" when the service says whether it takes images. */
function modelPicked() {
  const known = models.value.find((m) => m.id === llm.model.trim())?.vision
  if (known !== undefined) llm.vision = known
}
function chooseModel(id: string) {
  llm.model = id
  modelPicked()
}
function pickProvider(id: ProviderId) {
  const p = PROVIDERS.find((x) => x.id === id)
  const cur = s.value?.llm
  if (!p || !cur) return
  llm.provider = id
  llm.key = ''
  llm.removeKey = false
  models.value = []
  modelsState.value = { kind: 'idle', text: '' }
  localReachable.value = null
  gpt.value = null
  if (id !== 'off' && providerIn(cur) === id) {
    // Back to the one in use: put its settings back.
    Object.assign(llm, { url: cur.url, model: cur.model, vision: cur.vision })
  } else if (id === 'local') {
    Object.assign(llm, { url: cur.local_url, model: '' })
  } else {
    Object.assign(llm, { url: p.url, model: '' })
  }
  // The ChatGPT account and the local model server look for themselves as they appear.
  if (id !== 'chatgpt' && id !== 'local') void loadModels()
}
/** Look at the chosen service: the local model server's own list, the ChatGPT account, or a service's models. */
function connect() {
  if (llm.provider === 'chatgpt') return loadChatgptModels()
  return llm.provider === 'local' ? localServer.value?.load() : loadModels()
}

// A ChatGPT plan: the server signs in to the person's ChatGPT account, and its models are listed once it has.
const gpt = ref<ChatGptStatus | null>(null)
function chatgptStatus(st: ChatGptStatus) {
  gpt.value = st
  if (st.signed_in && !st.problem) return void loadChatgptModels()
  modelsAsk++
  models.value = []
  modelsState.value = { kind: 'idle', text: '' }
}
async function loadChatgptModels() {
  const ask = ++modelsAsk
  modelsState.value = { kind: 'loading', text: t('settings-ai-connecting') }
  try {
    const res = await api.chatgptModels()
    if (ask !== modelsAsk) return
    models.value = res.models
    const n = res.models.length
    modelsState.value = { kind: 'ok', text: n ? t('settings-ai-connected', { count: n }) : t('settings-chatgpt-connected-none') }
    // ChatGPT lists its recommended model first.
    if (!llm.model && n) chooseModel(res.models[0].id)
  } catch (e) {
    if (ask !== modelsAsk) return
    if (e instanceof ApiError && e.status === 401) unauthorized()
    models.value = []
    modelsState.value = { kind: 'error', text: (e as Error).message }
  }
}

// The local model server lists, downloads and deletes its own models; without it, a model is named by hand.
const localServer = ref<InstanceType<typeof LocalModelServer> | null>(null)
const localReachable = ref<boolean | null>(null)

const modelLabel = (m: ModelInfo) => [m.name, m.vision ? t('settings-ai-takes-images') : ''].filter(Boolean).join(' · ')

function saveLlm() {
  if (llm.provider === 'off') {
    void save({ llm_url: '', llm_chatgpt: false }, t('settings-ai-saved-off'))
    return
  }
  const chatgpt = llm.provider === 'chatgpt'
  const patch: SettingsPatch = {
    llm_chatgpt: chatgpt,
    llm_url: chatgpt ? '' : trimUrl(llm.url),
    llm_model: llm.model.trim(),
    llm_vision: llm.vision,
  }
  if (llm.batch !== s.value?.llm.batch.value) patch.llm_batch = llm.batch
  if (chatgpt) patch.llm_api_key = ''
  else if (llm.key) patch.llm_api_key = llm.key.trim()
  // A key is only ever sent to the address it was saved with.
  else if (llm.removeKey || urlChanged.value) patch.llm_api_key = ''
  void save(patch, t('settings-ai-saved', { service: provider.value.label }))
}
const resetLlm = () =>
  save(
    { llm_url: null, llm_model: null, llm_api_key: null, llm_vision: null, llm_chatgpt: null, llm_batch: null },
    t('settings-ai-reset'),
  )
const llmChanged = computed(() => {
  const l = s.value?.llm
  if (!l?.saved) return false
  const d = l.default
  return (
    l.chatgpt || trimUrl(l.url) !== trimUrl(d.url) || l.model !== d.model || l.vision !== d.vision || l.has_key !== d.has_key ||
    l.batch.value !== l.batch.default
  )
})
const llmDefaultText = computed(() => {
  const d = s.value?.llm.default
  if (!d?.url) return t('settings-default-off')
  return `${PROVIDERS.find((p) => p.id === providerOf(d.url))?.label ?? d.url} · ${d.model}`
})

onMounted(() => {
  if (s.value.llm.enabled && llm.provider !== 'chatgpt' && llm.provider !== 'local') void loadModels()
})
</script>

<template>
  <section>
    <div class="sh">
      <h3>{{ t('settings-ai-service') }}</h3>
      <ResetButton v-if="llmChanged" :to="llmDefaultText" :disabled="busy" @reset="resetLlm" />
    </div>
    <form class="llm" @submit.prevent="saveLlm">
      <label for="llm-provider">{{ t('settings-ai-service') }}</label>
      <select id="llm-provider" class="field" :value="llm.provider" :disabled="busy" @change="pickProvider(($event.target as HTMLSelectElement).value as ProviderId)">
        <option v-for="p in PROVIDERS" :key="p.id" :value="p.id">{{ p.label }}</option>
      </select>
      <span />
      <p class="hint inline">{{ provider.hint }}</p>
      <ChatGptAccount v-if="llm.provider === 'chatgpt'" @status="chatgptStatus" />
      <template v-if="llm.provider !== 'off' && (llm.provider !== 'chatgpt' || (gpt?.signed_in && !gpt.problem))">
        <template v-if="provider.editUrl">
          <label for="llm-url">{{ t('settings-ai-url') }}</label>
          <input
            id="llm-url"
            v-model="llm.url"
            class="field mono"
            type="url"
            spellcheck="false"
            :placeholder="llm.provider === 'local' ? s.llm.local_url : 'http://localhost:1234/v1'"
            @change="connect"
          />
        </template>
        <template v-if="provider.key || (provider.editUrl && llm.provider !== 'local')">
          <label for="llm-key">{{ t('settings-ai-key') }}</label>
          <div class="key">
            <input
              id="llm-key"
              v-model="llm.key"
              class="field mono"
              type="password"
              autocomplete="off"
              :placeholder="keptKey ? t('settings-ai-key-saved') : provider.key ? t('settings-ai-key-paste') : t('settings-ai-key-optional')"
              @input="llm.removeKey = false"
              @change="loadModels"
            />
            <button v-if="keptKey && !llm.key" type="button" class="link" @click="llm.removeKey = true; loadModels()">{{ t('settings-ai-key-remove') }}</button>
            <a v-if="provider.keyUrl" class="link" :href="provider.keyUrl" target="_blank" rel="noopener noreferrer">{{ t('settings-ai-get-key') }}</a>
          </div>
        </template>
        <LocalModelServer
          v-if="llm.provider === 'local'"
          ref="localServer"
          :url="llm.url"
          @reachable="(r) => (localReachable = r)"
        />
        <template v-if="llm.provider !== 'local' || localReachable === false">
          <label for="llm-model">{{ t('settings-ai-model') }}</label>
          <div class="key">
            <input
              id="llm-model"
              v-model="llm.model"
              class="field mono"
              type="text"
              spellcheck="false"
              list="llm-models"
              :placeholder="models.length ? t('settings-ai-model-pick') : t('settings-ai-model-name')"
              @change="modelPicked"
            />
            <button
              type="button"
              class="link"
              :disabled="(llm.provider !== 'chatgpt' && (!trimUrl(llm.url) || missingKey)) || modelsState.kind === 'loading'"
              @click="connect"
            >
              {{ modelsState.kind === 'ok' ? t('settings-ai-refresh') : t('settings-ai-connect') }}
            </button>
          </div>
          <datalist id="llm-models">
            <option v-for="m in models" :key="m.id" :value="m.id">{{ modelLabel(m) }}</option>
          </datalist>
          <template v-if="modelsState.kind !== 'idle' || missingKey">
            <span />
            <p class="status" :data-state="missingKey ? 'off' : { loading: 'looking', ok: 'ready', error: 'unavailable', idle: 'off' }[modelsState.kind]" role="status">
              <span class="dot" aria-hidden="true" />
              <span class="text">{{ missingKey ? t('settings-ai-need-key') : modelsState.text }}</span>
            </p>
          </template>
        </template>
        <span />
        <label class="check"><input v-model="llm.vision" type="checkbox" /> {{ t('settings-ai-vision') }}</label>
        <label for="llm-batch">{{ t('settings-ai-batch') }}</label>
        <div class="key">
          <input id="llm-batch" v-model.number="llm.batch" class="field num" type="number" min="1" max="200" />
          <span class="unit">{{ t('settings-ai-batch-unit') }}</span>
        </div>
      </template>
      <span />
      <div class="row">
        <button class="btn primary" type="submit" :disabled="busy || !llmDirty || !llmReady">{{ t('settings-save') }}</button>
        <span class="status" :data-state="s.llm.enabled ? 'ready' : 'off'">
          <span class="dot" aria-hidden="true" />
          <span class="text">{{ s.llm.enabled ? t('settings-ai-on', { service: providerLabel(s.llm), model: s.llm.model }) : t('settings-status-off') }}</span>
        </span>
      </div>
    </form>
  </section>
</template>

<style scoped src="./settingsForm.css"></style>
