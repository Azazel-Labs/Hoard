<script setup lang="ts">
// Settings: turn Blender and ffmpeg on or off (downloading them if need be), pick the AI service that tags packs,
// choose what happens to Unreal packages, and the server's limits. Saved on the server and in effect at once; a
// saved setting wins over its HOARD_* variable.
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import {
  ApiError,
  api,
  type LocalModel,
  type LocalModels,
  type ModelInfo,
  type Settings,
  type SettingsPatch,
  type ToolSetting,
  type UnrealConvert,
} from '../api'
import { confirmDialog, showError, showOk } from '../ui'

const emit = defineEmits<{ close: []; unauthorized: [] }>()
const dlg = ref<HTMLDialogElement | null>(null)
const s = ref<Settings | null>(null)
const busy = ref(false)

type ToolKey = 'blender' | 'ffmpeg'
type Mode = 'off' | 'auto' | 'path'
const modeOf = (v: string): Mode => {
  const t = v.trim().toLowerCase()
  if (['off', 'false', '0', 'none', 'no'].includes(t)) return 'off'
  if (t === '' || t === 'auto') return 'auto'
  return 'path'
}
/** The path being typed for each tool, before it's applied. */
const paths = reactive<Record<ToolKey, string>>({ blender: '', ffmpeg: '' })
const chosen = reactive<Record<ToolKey, Mode>>({ blender: 'off', ffmpeg: 'auto' })

// AI services: picking one fills in its address, so all that's left is a key (for hosted ones) and a model.
type ProviderId = 'off' | 'local' | 'openrouter' | 'openai' | 'anthropic' | 'gemini' | 'ollama' | 'custom'
interface Provider {
  id: ProviderId
  label: string
  url: string
  /** Needs an API key. */
  key?: boolean
  /** Where to get one. */
  keyUrl?: string
  /** The address can be edited (a self-hosted service). */
  editUrl?: boolean
  hint: string
}
const PROVIDERS: Provider[] = [
  { id: 'off', label: 'Off', url: '', hint: 'Packs are tagged with the built-in rules only.' },
  {
    id: 'local',
    label: 'Local model',
    // Filled in from the server (llm.local_url).
    url: '',
    editUrl: true,
    hint: 'The llm service in Hoard’s compose file (or cargo xtask llm). Download a model below; it runs on this machine and nothing leaves it.',
  },
  {
    id: 'openrouter',
    label: 'OpenRouter',
    url: 'https://openrouter.ai/api/v1',
    key: true,
    keyUrl: 'https://openrouter.ai/keys',
    hint: 'One key for hundreds of hosted models, some of them free. Models that take images are marked.',
  },
  {
    id: 'openai',
    label: 'OpenAI',
    url: 'https://api.openai.com/v1',
    key: true,
    keyUrl: 'https://platform.openai.com/api-keys',
    hint: 'Hosted by OpenAI.',
  },
  {
    id: 'anthropic',
    label: 'Anthropic',
    url: 'https://api.anthropic.com/v1',
    key: true,
    keyUrl: 'https://console.anthropic.com/settings/keys',
    hint: 'Hosted by Anthropic, through its OpenAI-compatible endpoint.',
  },
  {
    id: 'gemini',
    label: 'Google Gemini',
    url: 'https://generativelanguage.googleapis.com/v1beta/openai',
    key: true,
    keyUrl: 'https://aistudio.google.com/apikey',
    hint: 'Hosted by Google, through its OpenAI-compatible endpoint.',
  },
  {
    id: 'ollama',
    label: 'Ollama',
    url: 'http://ollama:11434/v1',
    editUrl: true,
    hint: 'An Ollama server you run. Change the address if it isn’t a container called ollama.',
  },
  {
    id: 'custom',
    label: 'Other (OpenAI-compatible)',
    url: '',
    editUrl: true,
    hint: 'Anything that speaks the OpenAI chat API: LM Studio, vLLM, llama.cpp, LiteLLM, …',
  },
]
const trimUrl = (u: string) => u.trim().replace(/\/+$/, '')
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

const llm = reactive({ provider: 'off' as ProviderId, url: '', model: '', key: '', removeKey: false, vision: false, batch: 1 })
const provider = computed(() => PROVIDERS.find((p) => p.id === llm.provider) ?? PROVIDERS[0])
/** The saved key belongs to the saved address; a different address starts without it. */
const urlChanged = computed(() => !!s.value && trimUrl(llm.url) !== trimUrl(s.value.llm.url))
const keptKey = computed(() => !!s.value?.llm.has_key && !urlChanged.value && !llm.removeKey)
const missingKey = computed(() => !!provider.value.key && !llm.key && !keptKey.value)
const llmDirty = computed(() => {
  const l = s.value?.llm
  if (!l) return false
  if (llm.provider === 'off') return l.url !== ''
  return (
    urlChanged.value || llm.model !== l.model || llm.key !== '' || llm.removeKey || llm.vision !== l.vision ||
    llm.batch !== l.batch.value
  )
})
const llmReady = computed(
  () => llm.provider === 'off' || (!!trimUrl(llm.url) && !!llm.model.trim() && !missingKey.value && llm.batch >= 1),
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
  modelsState.value = { kind: 'loading', text: 'Connecting…' }
  try {
    // Nothing typed: the server uses the saved key, if this is the address in use.
    const key = llm.key ? llm.key.trim() : llm.removeKey || urlChanged.value ? '' : undefined
    const res = await api.llmModels(url, key)
    if (ask !== modelsAsk) return
    models.value = res.models
    const n = res.models.length
    modelsState.value = {
      kind: 'ok',
      text: n ? `Connected · ${n} model${n === 1 ? '' : 's'}` : 'Connected, but it lists no models; type the model’s name',
    }
    if (!llm.model && n === 1) chooseModel(res.models[0].id)
  } catch (e) {
    if (ask !== modelsAsk) return
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
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
  if (id !== 'off' && providerOf(cur.url) === id) {
    // Back to the one in use: put its settings back.
    Object.assign(llm, { url: cur.url, model: cur.model, vision: cur.vision })
  } else if (id === 'local') {
    // The server's own default when that's a local model server (`cargo xtask dev --llm`), else the compose service.
    const url = providerOf(cur.default.url) === 'local' ? cur.default.url : cur.local_url
    Object.assign(llm, { url, model: '' })
  } else {
    Object.assign(llm, { url: p.url, model: '' })
  }
  void connect()
}
/** Look at the chosen service: the local model server's own list, or any other service's models. */
function connect() {
  return llm.provider === 'local' ? loadLocal() : loadModels()
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
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
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
const gb = (n: number) => `${(n / 1e9).toFixed(n >= 1e10 ? 0 : 1)} GB`
function localStatus(m: LocalModel): string {
  if (m.status === 'downloading') {
    const [done, total] = m.progress ?? [0, 0]
    return total ? `Downloading ${Math.floor((done / total) * 100)}% · ${gb(done)} of ${gb(total)}` : 'Downloading…'
  }
  if (m.status === 'incomplete') return 'Download stopped'
  if (m.failed) return 'Failed to start last time'
  if (inUse(m)) return m.status === 'loaded' ? 'In use · running' : 'In use'
  if (m.status === 'loading') return 'Starting…'
  if (m.status === 'loaded') return 'Running'
  return m.vision ? 'Ready · takes images' : 'Ready · text only'
}
const ready = (m: LocalModel) => !['downloading', 'incomplete'].includes(m.status)
function useLocal(m: LocalModel) {
  const patch: SettingsPatch = { llm_url: trimUrl(llm.url), llm_model: m.id, llm_vision: m.vision, llm_api_key: '' }
  // Small models tag better a few assets at a time; keep a batch size someone chose.
  if (!s.value?.llm.batch.saved) patch.llm_batch = 12
  void save(patch, `AI tagging: ${nameOf(m.id)}`)
}
async function deleteLocal(m: LocalModel) {
  const ok = await confirmDialog({
    title: `Delete ${nameOf(m.id)}?`,
    body: inUse(m)
      ? 'AI tagging uses this model, so it stops until you pick another. Its files are deleted from the model server.'
      : m.status === 'downloading'
        ? 'The download stops and what came down so far is deleted.'
        : 'Its files are deleted from the model server. You can download it again later.',
    confirm: 'Delete',
    danger: true,
  })
  if (ok) void loadLocal('delete', m.id)
}
function downloadOther() {
  const id = otherModel.value.trim()
  if (id) void loadLocal('download', id)
}
const modelLabel = (m: ModelInfo) => [m.name, m.vision ? 'takes images' : ''].filter(Boolean).join(' · ')

// Server options: numbers and addresses wait for Save; the switches apply at once.
const opts = reactive({ workers: 1, max_upload_mb: 1, max_expand_mb: 1, max_files: 1, public_url: '', soundfont: '' })
type OptKey = keyof typeof opts
const OPT_KEYS = Object.keys(opts) as OptKey[]
const optsDirty = computed(() => !!s.value && OPT_KEYS.some((k) => opts[k] !== s.value!.options[k].value))
const optsSaved = computed(() => !!s.value && Object.values(s.value.options).some((o) => o.saved))
function saveOpts() {
  const cur = s.value
  if (!cur) return
  const patch: Record<string, string | number> = {}
  for (const k of OPT_KEYS) {
    const v = opts[k]
    if (v !== cur.options[k].value) patch[k] = typeof v === 'string' ? v.trim() : v
  }
  void save(patch as SettingsPatch, 'Server options saved')
}
function resetOpts() {
  const patch: Record<string, null> = {}
  for (const k of Object.keys(s.value?.options ?? {})) patch[k] = null
  void save(patch as SettingsPatch, 'Server options are back to the defaults')
}
const mib = (n: number) => (n >= 1024 && n % 1024 === 0 ? `${n / 1024} GiB` : `${n} MiB`)
const defaultsText = computed(() => {
  const o = s.value?.options
  if (!o) return ''
  return [
    `${o.workers.default} worker${o.workers.default === 1 ? '' : 's'}`,
    `uploads up to ${mib(o.max_upload_mb.default)}`,
    `packs up to ${mib(o.max_expand_mb.default)} and ${o.max_files.default.toLocaleString()} files`,
  ].join(', ')
})

function take(next: Settings) {
  s.value = next
  for (const k of ['blender', 'ffmpeg'] as const) {
    chosen[k] = modeOf(next[k].value)
    paths[k] = chosen[k] === 'path' ? next[k].value : ''
  }
  Object.assign(llm, {
    provider: providerOf(next.llm.url),
    url: next.llm.url,
    model: next.llm.model,
    key: '',
    removeKey: false,
    vision: next.llm.vision,
    batch: next.llm.batch.value,
  })
  Object.assign(
    opts,
    Object.fromEntries(OPT_KEYS.map((k) => [k, next.options[k].value])),
  )
  schedulePoll()
}

async function load() {
  try {
    take(await api.settings())
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) {
      emit('unauthorized')
      dlg.value?.close()
      return
    }
    showError(`Couldn't load the settings: ${(e as Error).message}`)
  }
}

async function save(patch: SettingsPatch, done?: string) {
  busy.value = true
  try {
    take(await api.saveSettings(patch))
    if (done) showOk(done)
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError((e as Error).message)
    if (s.value) take(s.value) // put the controls back as they were
  } finally {
    busy.value = false
  }
}

function pick(k: ToolKey, m: Mode) {
  chosen[k] = m
  if (m === 'off') void save({ [k]: 'off' })
  else if (m === 'auto') void save({ [k]: 'auto' })
  // A path waits for "Use".
}
function usePath(k: ToolKey) {
  const p = paths[k].trim()
  if (p) void save({ [k]: p })
}

function saveLlm() {
  if (llm.provider === 'off') {
    void save({ llm_url: '' }, 'AI tagging is off')
    return
  }
  const patch: SettingsPatch = { llm_url: trimUrl(llm.url), llm_model: llm.model.trim(), llm_vision: llm.vision }
  if (llm.batch !== s.value?.llm.batch.value) patch.llm_batch = llm.batch
  if (llm.key) patch.llm_api_key = llm.key.trim()
  // A key is only ever sent to the address it was saved with.
  else if (llm.removeKey || urlChanged.value) patch.llm_api_key = ''
  void save(patch, `AI tagging: ${provider.value.label}`)
}
const resetLlm = () => save({ llm_url: null, llm_model: null, llm_api_key: null, llm_vision: null, llm_batch: null })

// While a tool is being looked for or downloaded, keep its status line current.
let timer: number | undefined
function schedulePoll() {
  window.clearTimeout(timer)
  const v = s.value
  if (v && [v.blender, v.ffmpeg].some((t) => t.state === 'looking' || t.state === 'downloading')) {
    timer = window.setTimeout(async () => {
      try {
        const next = await api.settings()
        // Don't clobber a path or AI setting someone is typing.
        if (s.value) s.value = { ...s.value, blender: next.blender, ffmpeg: next.ffmpeg }
      } catch {
        // Try again on the next tick.
      }
      schedulePoll()
    }, 1500)
  }
}

const TOOLS: { key: ToolKey; title: string; what: string; download: string }[] = [
  {
    key: 'blender',
    title: 'Blender',
    what: 'Converts .blend, .fbx, .dae, .usd, .ply, .stl and .3ds models to glTF in the background, so they get thumbnails and a 3D preview. Packs added while it was off are converted once it’s ready.',
    download: 'Uses an installed Blender, or downloads Blender 4.5 LTS (about 400 MB) from a blender.org mirror.',
  },
  {
    key: 'ffmpeg',
    title: 'ffmpeg',
    what: 'Encodes MP3, OGG and Opus downloads, and reads audio Hoard can’t decode itself (AAC, M4A, WMA, …).',
    download: 'Uses a system ffmpeg with the right encoders, or downloads a static build (about 80 MB).',
  },
]

function statusText(t: ToolSetting): string {
  switch (t.state) {
    case 'off': return 'Off'
    case 'looking': return 'Looking…'
    case 'downloading': return 'Downloading… progress is in the activity indicator'
    case 'ready': return `Ready: ${t.detail}`
    case 'unavailable': return t.detail
  }
}
const defaultText = (v: string) => (modeOf(v) === 'off' ? 'off' : modeOf(v) === 'auto' ? 'automatic' : v)

const UNREAL: { value: UnrealConvert; label: string; hint: string }[] = [
  { value: 'off', label: 'Leave as is', hint: 'Convert from a pack’s page when you want to.' },
  { value: 'keep', label: 'Convert, keep originals', hint: 'Textures, sounds and materials come out in common formats; the .uasset stays too.' },
  { value: 'replace', label: 'Convert, drop originals', hint: 'Saves space once the content is out. Meshes still wait for the Unreal worker.' },
]

onMounted(async () => {
  dlg.value?.showModal()
  await load()
  if (s.value?.llm.enabled || llm.provider === 'local') void connect()
})
onBeforeUnmount(() => {
  window.clearTimeout(timer)
  window.clearTimeout(localTimer)
  localAsk++
})
</script>

<template>
  <dialog ref="dlg" class="settings" aria-labelledby="settings-title" @close="emit('close')" @click.self="dlg?.close()">
    <header>
      <h2 id="settings-title">Settings</h2>
      <button type="button" class="btn ghost x" aria-label="Close" autofocus @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <p v-if="!s" class="loading">Loading…</p>
    <div v-else class="body" :aria-busy="busy">
      <section v-for="t in TOOLS" :key="t.key">
        <h3>{{ t.title }}</h3>
        <p class="what">{{ t.what }}</p>
        <div class="modes" role="radiogroup" :aria-label="t.title">
          <label>
            <input type="radio" :name="t.key" :checked="chosen[t.key] === 'off'" :disabled="busy" @change="pick(t.key, 'off')" />
            Off
          </label>
          <label>
            <input type="radio" :name="t.key" :checked="chosen[t.key] === 'auto'" :disabled="busy" @change="pick(t.key, 'auto')" />
            Automatic
          </label>
          <label>
            <input type="radio" :name="t.key" :checked="chosen[t.key] === 'path'" :disabled="busy" @change="pick(t.key, 'path')" />
            This program
          </label>
        </div>
        <p v-if="chosen[t.key] === 'auto'" class="hint">
          {{ s[t.key].can_download ? t.download : `Uses an installed ${t.title}. There’s no build to download for this machine, so install it or give its path.` }}
        </p>
        <form v-if="chosen[t.key] === 'path'" class="path" @submit.prevent="usePath(t.key)">
          <input
            v-model="paths[t.key]"
            class="field mono"
            type="text"
            spellcheck="false"
            :aria-label="`${t.title} program path`"
            :placeholder="t.key === 'blender' ? '/opt/blender/blender' : '/usr/bin/ffmpeg'"
          />
          <button class="btn" type="submit" :disabled="busy || !paths[t.key].trim() || paths[t.key].trim() === s[t.key].value">Use</button>
        </form>
        <p class="status" :data-state="s[t.key].state" role="status">
          <span class="dot" aria-hidden="true" />
          <span class="text">{{ statusText(s[t.key]) }}</span>
        </p>
        <p v-if="s[t.key].saved" class="default">
          Server default: {{ defaultText(s[t.key].default) }} ·
          <button type="button" class="link" :disabled="busy" @click="save({ [t.key]: null })">Use it</button>
        </p>
      </section>

      <section>
        <h3>AI tagging</h3>
        <p class="what">An AI model names, describes and tags new packs, on top of the built-in rules. Its tags are marked as suggested.</p>
        <form class="llm" @submit.prevent="saveLlm">
          <label for="llm-provider">Service</label>
          <select id="llm-provider" class="field" :value="llm.provider" :disabled="busy" @change="pickProvider(($event.target as HTMLSelectElement).value as ProviderId)">
            <option v-for="p in PROVIDERS" :key="p.id" :value="p.id">{{ p.label }}</option>
          </select>
          <span />
          <p class="hint inline">{{ provider.hint }}</p>
          <template v-if="llm.provider !== 'off'">
            <template v-if="provider.editUrl">
              <label for="llm-url">Address</label>
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
              <label for="llm-key">API key</label>
              <div class="key">
                <input
                  id="llm-key"
                  v-model="llm.key"
                  class="field mono"
                  type="password"
                  autocomplete="off"
                  :placeholder="keptKey ? 'Saved (type to replace)' : provider.key ? 'Paste your key' : 'None (optional)'"
                  @input="llm.removeKey = false"
                  @change="loadModels"
                />
                <button v-if="keptKey && !llm.key" type="button" class="link" @click="llm.removeKey = true; loadModels()">Remove</button>
                <a v-if="provider.keyUrl" class="link" :href="provider.keyUrl" target="_blank" rel="noopener noreferrer">Get a key</a>
              </div>
            </template>
            <template v-if="llm.provider === 'local' && !local?.reachable">
              <span />
              <p v-if="localState !== 'ok'" class="status" data-state="looking" role="status">
                <span class="dot" aria-hidden="true" />
                <span class="text">Connecting…</span>
              </p>
              <div v-else-if="local" class="local-off">
                <p class="status" data-state="unavailable" role="status">
                  <span class="dot" aria-hidden="true" />
                  <span class="text">{{ local.error }}</span>
                </p>
                <p class="hint inline">
                  Start it with <code>docker compose up -d llm</code> (or <code>cargo xtask llm</code> when developing), then
                  <button type="button" class="link" @click="loadLocal()">try again</button>. A server that runs just one model
                  works too: give its model’s name below.
                </p>
              </div>
            </template>
            <div v-if="llm.provider === 'local' && local?.reachable" class="local">
              <ul v-if="local.models.length" class="mlist" aria-label="On the model server">
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
                    <button v-if="m.status === 'incomplete'" type="button" class="btn" :disabled="!!localBusy" @click="loadLocal('download', m.id)">Resume</button>
                    <button v-else-if="ready(m) && !inUse(m)" type="button" class="btn primary" :disabled="busy || !!localBusy" @click="useLocal(m)">Use</button>
                    <button type="button" class="btn ghost" :disabled="!!localBusy" @click="deleteLocal(m)">
                      {{ localBusy === m.id ? 'Deleting…' : m.status === 'downloading' ? 'Cancel' : 'Delete' }}
                    </button>
                  </span>
                </li>
              </ul>
              <p v-else class="hint inline">Nothing downloaded yet. Pick one below; it’s kept on the server, and you can switch any time.</p>
              <ul v-if="notDownloaded.length" class="mlist" aria-label="Models to download">
                <li v-for="m in notDownloaded" :key="m.id">
                  <span class="what">
                    <strong>{{ m.name }}</strong>
                    <small>{{ m.size_gb }} GB · {{ m.license }}</small>
                    <small>{{ m.note }}</small>
                  </span>
                  <span class="acts">
                    <button type="button" class="btn" :disabled="!!localBusy" @click="loadLocal('download', m.id)">
                      {{ localBusy === m.id ? 'Starting…' : 'Download' }}
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
                  aria-label="Another model from Hugging Face"
                  placeholder="Another from Hugging Face: owner/name-GGUF:Q4_K_M"
                  @keydown.enter.prevent="downloadOther"
                />
                <button type="button" class="btn" :disabled="!otherModel.trim() || !!localBusy" @click="downloadOther">Download</button>
              </div>
            </div>
            <template v-if="llm.provider !== 'local' || (local && !local.reachable)">
            <label for="llm-model">Model</label>
            <div class="key">
              <input
                id="llm-model"
                v-model="llm.model"
                class="field mono"
                type="text"
                spellcheck="false"
                list="llm-models"
                :placeholder="models.length ? 'Pick or type a model' : 'Model name'"
                @change="modelPicked"
              />
              <button type="button" class="link" :disabled="!trimUrl(llm.url) || missingKey || modelsState.kind === 'loading'" @click="loadModels">
                {{ modelsState.kind === 'ok' ? 'Refresh' : 'Connect' }}
              </button>
            </div>
            <datalist id="llm-models">
              <option v-for="m in models" :key="m.id" :value="m.id">{{ modelLabel(m) }}</option>
            </datalist>
            <template v-if="modelsState.kind !== 'idle' || missingKey">
              <span />
              <p class="status" :data-state="missingKey ? 'off' : { loading: 'looking', ok: 'ready', error: 'unavailable', idle: 'off' }[modelsState.kind]" role="status">
                <span class="dot" aria-hidden="true" />
                <span class="text">{{ missingKey ? 'Add the API key to see its models' : modelsState.text }}</span>
              </p>
            </template>
            </template>
            <span />
            <label class="check"><input v-model="llm.vision" type="checkbox" /> Send thumbnails too (needs a model that takes images)</label>
            <label for="llm-batch">Per request</label>
            <div class="key">
              <input id="llm-batch" v-model.number="llm.batch" class="field num" type="number" min="1" max="200" />
              <span class="unit">assets (fewer for small local models, more for big hosted ones)</span>
            </div>
          </template>
          <span />
          <div class="row">
            <button class="btn primary" type="submit" :disabled="busy || !llmDirty || !llmReady">Save</button>
            <span class="status" :data-state="s.llm.enabled ? 'ready' : 'off'">
              <span class="dot" aria-hidden="true" />
              <span class="text">{{ s.llm.enabled ? `On: ${PROVIDERS.find((p) => p.id === providerOf(s!.llm.url))?.label} · ${s.llm.model}` : 'Off' }}</span>
            </span>
          </div>
        </form>
        <p v-if="s.llm.saved" class="default">
          Server default: {{ s.llm.default.url ? `${s.llm.default.url} · ${s.llm.default.model}` : 'off' }} ·
          <button type="button" class="link" :disabled="busy" @click="resetLlm">Use it</button>
        </p>
      </section>

      <section>
        <h3>Unreal packages</h3>
        <p class="what">What to do with .uasset files when a pack is added.</p>
        <div class="choices" role="radiogroup" aria-label="Unreal packages">
          <label v-for="u in UNREAL" :key="u.value" :class="{ on: s.unreal_convert.value === u.value }">
            <input
              type="radio"
              name="unreal"
              :checked="s.unreal_convert.value === u.value"
              :disabled="busy"
              @change="save({ unreal_convert: u.value })"
            />
            <span><strong>{{ u.label }}</strong><small>{{ u.hint }}</small></span>
          </label>
        </div>
        <p v-if="s.unreal_convert.saved" class="default">
          Server default: {{ UNREAL.find((u) => u.value === s!.unreal_convert.default)?.label.toLowerCase() }} ·
          <button type="button" class="link" :disabled="busy" @click="save({ unreal_convert: null })">Use it</button>
        </p>
      </section>

      <section>
        <h3>Server</h3>
        <div class="switches">
          <label class="check">
            <input type="checkbox" :checked="s.options.render.value" :disabled="busy" @change="save({ render: ($event.target as HTMLInputElement).checked })" />
            Render thumbnails for 3D models
          </label>
          <label class="check">
            <input
              type="checkbox"
              :checked="s.options.update_check.value"
              :disabled="busy"
              @change="save({ update_check: ($event.target as HTMLInputElement).checked })"
            />
            Check GitHub once a day for a new release
          </label>
        </div>
        <form class="llm opts" @submit.prevent="saveOpts">
          <label for="opt-workers">Background workers</label>
          <div class="key">
            <input id="opt-workers" v-model.number="opts.workers" class="field num" type="number" min="1" :max="s.max_workers" />
            <span class="unit">jobs at once (ingest, tagging, conversions)</span>
          </div>
          <label for="opt-upload">Largest upload</label>
          <div class="key">
            <input id="opt-upload" v-model.number="opts.max_upload_mb" class="field num" type="number" min="1" />
            <span class="unit">MiB</span>
          </div>
          <label for="opt-expand">Largest pack</label>
          <div class="key">
            <input id="opt-expand" v-model.number="opts.max_expand_mb" class="field num" type="number" min="1" />
            <span class="unit">MiB once unpacked</span>
          </div>
          <label for="opt-files">Most files</label>
          <div class="key">
            <input id="opt-files" v-model.number="opts.max_files" class="field num" type="number" min="1" />
            <span class="unit">in one pack</span>
          </div>
          <label for="opt-public">Public address</label>
          <input
            id="opt-public"
            v-model="opts.public_url"
            class="field mono"
            type="url"
            spellcheck="false"
            placeholder="From the browser’s address (e.g. https://hoard.example.com)"
          />
          <label for="opt-sf">MIDI SoundFont</label>
          <input id="opt-sf" v-model="opts.soundfont" class="field mono" type="text" spellcheck="false" placeholder="Found on the server, or built-in" />
          <span />
          <div class="row">
            <button class="btn primary" type="submit" :disabled="busy || !optsDirty">Save</button>
          </div>
        </form>
        <p class="default">
          Server defaults: {{ defaultsText }}<template v-if="optsSaved">
            · <button type="button" class="link" :disabled="busy" @click="resetOpts">Use them</button></template>
        </p>
      </section>
    </div>
  </dialog>
</template>

<style scoped>
.settings {
  width: min(40rem, calc(100vw - 2rem)); max-height: calc(100dvh - 2rem); padding: 1.1rem 1.3rem 1.3rem;
  border: 1px solid var(--line); border-radius: 10px; background: var(--panel); color: var(--text); box-shadow: var(--elev-3);
}
.settings[open] { animation: settings-in var(--dur-3) var(--ease-pop); }
.settings::backdrop { background: var(--scrim); animation: fade var(--dur-2) var(--ease-out); }
@keyframes settings-in { from { opacity: 0; scale: 0.96; } }
@keyframes fade { from { opacity: 0; } }
header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.4rem; }
h2 { margin: 0; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.x { padding: 0.35em 0.45em; }
.loading { color: var(--muted); font-size: var(--step--1); }
.body[aria-busy='true'] { cursor: progress; }

section { padding: 0.9rem 0; border-top: 1px solid var(--line); }
section:first-child { border-top: 0; }
h3 { margin: 0 0 0.3rem; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
.what, .hint, .default { margin: 0 0 0.55rem; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.hint { margin-top: 0.45rem; }
.default { margin: 0.4rem 0 0; color: var(--faint); }

.modes { display: inline-flex; flex-wrap: wrap; border: 1px solid var(--line); border-radius: var(--r-chip); overflow: hidden; }
.modes label {
  display: inline-flex; align-items: center; padding: 0.4em 0.85em; font-size: var(--step--1); cursor: pointer;
  border-left: 1px solid var(--line); transition: background-color var(--dur-1), color var(--dur-1);
}
.modes label:first-child { border-left: 0; }
.modes label:has(:checked) { background: var(--text); color: var(--bg); font-weight: 600; }
.modes label:has(:focus-visible) { outline: 2px solid var(--k-font); outline-offset: -2px; }
.modes input { position: absolute; opacity: 0; pointer-events: none; }

.path { display: flex; gap: 0.5rem; margin-top: 0.55rem; }
.path .field { flex: 1; min-width: 0; }

.status { display: flex; align-items: baseline; gap: 0.45rem; margin: 0.55rem 0 0; font-size: var(--step--1); color: var(--muted); }
.status .text { overflow-wrap: anywhere; }
.status .dot { flex: none; width: 0.5rem; height: 0.5rem; border-radius: 50%; background: var(--faint); translate: 0 -0.05rem; }
.status[data-state='ready'] .dot { background: var(--ok); }
.status[data-state='unavailable'] { color: var(--danger); }
.status[data-state='unavailable'] .dot { background: var(--danger); }
.status[data-state='looking'] .dot, .status[data-state='downloading'] .dot { background: var(--k-font); animation: pulse 1.2s var(--ease-in-out) infinite; }
@keyframes pulse { 50% { opacity: 0.3; } }
@media (prefers-reduced-motion: reduce) { .status .dot { animation: none !important; } }

.llm { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.5rem 0.8rem; align-items: center; font-size: var(--step--1); }
.llm > label { color: var(--muted); }
.llm .key { display: flex; gap: 0.6rem; align-items: center; }
.llm .key .field { flex: 1; min-width: 0; }
.llm .check { display: inline-flex; gap: 0.45rem; align-items: center; color: var(--text); }
.llm .row { display: flex; gap: 0.9rem; align-items: center; }
.llm .row .status { margin: 0; }
.llm .status, .llm .hint.inline { margin: 0; }
.llm .num { width: 6.5rem; flex: none; }
.llm .unit { color: var(--muted); }
.llm select.field { justify-self: start; min-width: 14rem; max-width: 100%; }
.llm code { font-size: 0.95em; }
.local-off { display: grid; gap: 0.35rem; }
.local { grid-column: 1 / -1; display: grid; gap: 0.6rem; }
.mlist { display: grid; gap: 0.4rem; margin: 0; padding: 0; list-style: none; }
.mlist li {
  display: flex; gap: 0.8rem; align-items: center; padding: 0.55rem 0.7rem; border: 1px solid var(--line); border-radius: var(--r-chip);
}
.mlist li.on { border-color: var(--muted); background: var(--raised); }
.mlist .what { flex: 1; min-width: 0; display: grid; gap: 0.15rem; }
.mlist small { color: var(--muted); font-size: inherit; overflow-wrap: anywhere; }
.mlist .status { margin: 0.1rem 0 0; }
.mlist progress { width: 100%; height: 0.35rem; accent-color: var(--k-font); }
.mlist .acts { display: flex; gap: 0.4rem; flex: none; }
@media (max-width: 30rem) { .mlist li { flex-direction: column; align-items: stretch; } }
.switches { display: grid; gap: 0.35rem; margin-bottom: 0.8rem; font-size: var(--step--1); }
.switches .check { display: inline-flex; gap: 0.45rem; align-items: center; }
@media (max-width: 30rem) { .llm { grid-template-columns: minmax(0, 1fr); } .llm > span:empty { display: none; } }

.choices { display: grid; gap: 0.4rem; }
.choices label {
  display: flex; gap: 0.6rem; align-items: flex-start; padding: 0.55rem 0.7rem; border: 1px solid var(--line);
  border-radius: var(--r-chip); cursor: pointer; font-size: var(--step--1); transition: border-color var(--dur-1), background-color var(--dur-1);
}
.choices label.on { border-color: var(--muted); background: var(--raised); }
.choices input { margin: 0.2rem 0 0; }
.choices span { display: grid; gap: 0.1rem; }
.choices small { color: var(--muted); font-size: inherit; }

.link {
  padding: 0; border: 0; background: none; color: var(--k-font); cursor: pointer; font-size: inherit; text-decoration: underline;
  text-underline-offset: 0.15em;
}
.link:disabled { opacity: 0.5; cursor: default; }
</style>
