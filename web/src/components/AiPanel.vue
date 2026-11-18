<script setup lang="ts">
// AI tagging: the service that tags packs (a hosted one, a ChatGPT plan, or the local model server), its model,
// and whether it sees thumbnails.
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import { ApiError, api, type ChatGptStatus, type LocalModel, type LocalModels, type ModelInfo, type Settings, type SettingsPatch } from '../api'
import { formatNumber, t } from '../i18n'
import { confirmDialog, showError, showOk } from '../ui'
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
  local.value = null
  if (id !== 'off' && providerIn(cur) === id) {
    // Back to the one in use: put its settings back.
    Object.assign(llm, { url: cur.url, model: cur.model, vision: cur.vision })
  } else if (id === 'local') {
    Object.assign(llm, { url: cur.local_url, model: '' })
  } else {
    Object.assign(llm, { url: p.url, model: '' })
  }
  void connect()
}
/** Look at the chosen service: the local model server's own list, the ChatGPT account, or a service's models. */
function connect() {
  if (llm.provider === 'chatgpt') return loadChatgpt()
  return llm.provider === 'local' ? loadLocal() : loadModels()
}

// A ChatGPT plan: the server signs in to the person's ChatGPT account and tags with what the plan includes.
const gpt = ref<ChatGptStatus | null>(null)
const gptBusy = ref(false)
/** The sign-in address, once "Continue with ChatGPT" was pressed (in case the window didn't open). */
const gptUrl = ref('')
const gptPaste = ref('')
let gptTimer: number | undefined
let gptUntil = 0
async function loadChatgpt() {
  const ask = ++modelsAsk
  try {
    const st = await api.chatgpt()
    if (ask !== modelsAsk) return
    gpt.value = st
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
    return
  }
  if (!gpt.value.signed_in || gpt.value.problem) {
    models.value = []
    modelsState.value = { kind: 'idle', text: '' }
    return
  }
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
    models.value = []
    modelsState.value = { kind: 'error', text: (e as Error).message }
  }
}
async function startChatgpt() {
  // Open the window while the click still counts; it's sent on once the server has the address ready.
  const win = window.open('', '_blank')
  gptBusy.value = true
  try {
    // The port this browser reaches Hoard on: on the server's own machine, ChatGPT sends it straight back here.
    const port = Number(location.port) || (location.protocol === 'https:' ? 443 : 80)
    const { url } = await api.chatgptStart(port)
    gptUrl.value = url
    gptPaste.value = ''
    if (win) {
      win.opener = null
      win.location.href = url
    }
    gptUntil = Date.now() + 15 * 60_000
    pollChatgpt()
  } catch (e) {
    win?.close()
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
  } finally {
    gptBusy.value = false
  }
}
/** While a sign-in is under way, notice when it finishes in the other window. */
function pollChatgpt() {
  window.clearTimeout(gptTimer)
  if (Date.now() > gptUntil) return
  gptTimer = window.setTimeout(async () => {
    try {
      const st = await api.chatgpt()
      if (st.signed_in && !st.problem && (!gpt.value?.signed_in || gpt.value.problem)) return signedIn(st)
      gpt.value = st
    } catch {
      // Try again on the next tick.
    }
    pollChatgpt()
  }, 2000)
}
function signedIn(st: ChatGptStatus) {
  window.clearTimeout(gptTimer)
  gpt.value = st
  gptUrl.value = ''
  gptPaste.value = ''
  showOk(st.email ? t('settings-chatgpt-signed-in-as', { email: st.email }) : t('settings-chatgpt-signed-in'))
  void loadChatgpt()
}
async function finishChatgpt() {
  const url = gptPaste.value.trim()
  if (!url) return
  gptBusy.value = true
  try {
    signedIn(await api.chatgptFinish(url))
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
  } finally {
    gptBusy.value = false
  }
}
async function signOutChatgpt() {
  const ok = await confirmDialog({
    title: t('settings-chatgpt-sign-out-title'),
    body: s.value?.llm.chatgpt ? t('settings-chatgpt-sign-out-in-use') : t('settings-chatgpt-sign-out-body'),
    confirm: t('settings-chatgpt-sign-out'),
  })
  if (!ok) return
  gptBusy.value = true
  try {
    const st = await api.chatgptSignOut()
    gpt.value = st
    models.value = []
    modelsState.value = { kind: 'idle', text: '' }
    if (st.note) showError(st.note)
    else showOk(t('settings-chatgpt-signed-out'))
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
  } finally {
    gptBusy.value = false
  }
}

// The local model server (llama.cpp in router mode): downloads models from Hugging Face and lists what it has.
const local = ref<LocalModels | null>(null)
const localState = ref<'idle' | 'loading' | 'ok'>('idle')
/** The model being downloaded or deleted from here, while the server works on it. */
const localBusy = ref('')
const otherModel = ref('')
let localAsk = 0
let localTimer: number | undefined
async function loadLocal(action: 'list' | 'download' | 'delete' = 'list', model?: string): Promise<void> {
  window.clearTimeout(localTimer)
  const url = trimUrl(llm.url)
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
      return loadLocal()
    }
  }
  localState.value = 'ok'
  localBusy.value = ''
  // Keep progress moving while something downloads or starts up.
  if (local.value?.models.some((m) => ['downloading', 'loading'].includes(m.status))) {
    localTimer = window.setTimeout(() => void loadLocal(), 1500)
  }
}
const localIds = computed(() => new Set(local.value?.models.map((m) => m.id) ?? []))
const notDownloaded = computed(() => local.value?.suggested.filter((m) => !localIds.value.has(m.id)) ?? [])
const nameOf = (id: string) => local.value?.suggested.find((m) => m.id === id)?.name ?? id
/** The model AI tagging uses right now, if it's on this server. */
const inUse = (m: LocalModel) => !!s.value && s.value.llm.model === m.id && trimUrl(s.value.llm.url) === trimUrl(llm.url)
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
  const patch: SettingsPatch = { llm_url: trimUrl(llm.url), llm_model: m.id, llm_vision: m.vision, llm_api_key: '' }
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
  if (ok) void loadLocal('delete', m.id)
}
function downloadOther() {
  const id = otherModel.value.trim()
  if (id) void loadLocal('download', id)
}
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
  if (s.value.llm.enabled || llm.provider === 'local') void connect()
})
onBeforeUnmount(() => {
  window.clearTimeout(localTimer)
  window.clearTimeout(gptTimer)
  localAsk++
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
      <template v-if="llm.provider === 'chatgpt'">
        <span class="label">{{ t('settings-chatgpt-account') }}</span>
        <div class="gpt">
          <p v-if="!gpt" class="status" data-state="looking" role="status">
            <span class="dot" aria-hidden="true" />
            <span class="text">{{ t('settings-chatgpt-checking') }}</span>
          </p>
          <template v-else>
            <p v-if="gpt.signed_in || gpt.problem" class="status" :data-state="gpt.problem ? 'unavailable' : 'ready'" role="status">
              <span class="dot" aria-hidden="true" />
              <span class="text">{{ gpt.problem ?? (gpt.email ? t('settings-chatgpt-status-as', { email: gpt.email }) : t('settings-chatgpt-status')) }}</span>
            </p>
            <div class="row">
              <button v-if="!gpt.signed_in || gpt.problem" type="button" class="btn primary" :disabled="gptBusy" @click="startChatgpt">
                {{ t('settings-chatgpt-continue') }}
              </button>
              <button v-if="gpt.signed_in" type="button" class="link" :disabled="gptBusy" @click="signOutChatgpt">{{ t('settings-chatgpt-sign-out') }}</button>
              <a class="link" :href="gpt.manage_url" target="_blank" rel="noopener noreferrer">{{ t('settings-chatgpt-manage') }}</a>
            </div>
            <template v-if="gptUrl && (!gpt.signed_in || gpt.problem)">
              <p class="hint inline">
                {{ t('settings-chatgpt-finish-before') }}<a :href="gptUrl" target="_blank" rel="noopener noreferrer">{{ t('settings-chatgpt-finish-link') }}</a>{{ t('settings-chatgpt-finish-middle') }}<code>http://127.0.0.1</code>{{ t('settings-chatgpt-finish-after') }}
              </p>
              <div class="key">
                <input
                  v-model="gptPaste"
                  class="field mono"
                  type="url"
                  spellcheck="false"
                  :aria-label="t('settings-chatgpt-paste')"
                  placeholder="http://127.0.0.1:…/auth/callback?code=…"
                  @keydown.enter.prevent="finishChatgpt"
                />
                <button type="button" class="btn" :disabled="!gptPaste.trim() || gptBusy" @click="finishChatgpt">{{ t('settings-chatgpt-finish') }}</button>
              </div>
            </template>
          </template>
          <p class="hint inline">{{ t('settings-chatgpt-usage') }}</p>
        </div>
      </template>
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
        <template v-if="llm.provider === 'local' && !local?.reachable">
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
              {{ t('settings-local-start-before') }}<code>docker compose up -d llm</code>{{ t('settings-local-start-or') }}<code>cargo xtask llm</code>{{ t('settings-local-start-developing') }}<button type="button" class="link" @click="loadLocal()">{{ t('settings-local-start-retry') }}</button>{{ t('settings-local-start-after') }}
            </p>
          </div>
        </template>
        <div v-if="llm.provider === 'local' && local?.reachable" class="local">
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
                <button v-if="m.status === 'incomplete'" type="button" class="btn" :disabled="!!localBusy" @click="loadLocal('download', m.id)">{{ t('settings-local-resume') }}</button>
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
                <button type="button" class="btn" :disabled="!!localBusy" @click="loadLocal('download', m.id)">
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
        <template v-if="llm.provider !== 'local' || (local && !local.reachable)">
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

<style scoped>
.gpt { display: grid; gap: 0.5rem; justify-items: start; }
.gpt .key { width: 100%; }
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
