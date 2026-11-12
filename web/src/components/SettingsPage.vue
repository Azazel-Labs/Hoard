<script setup lang="ts">
// The Settings page: a list of pages down the side, one open at a time (its id is in the address, #settings/<id>).
// Appearance is this browser's (light and dark themes, saved here); Library is the hoard in numbers; the rest are the
// server's: Blender and ffmpeg (downloaded if need be), the AI service that tags packs, what happens to Unreal
// packages, and the server's switches and limits. Those are saved on the server and in effect at once. Anything not
// at its default has a turn-back arrow beside it that resets it.
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import {
  ApiError,
  api,
  type ChatGptStatus,
  type LocalModel,
  type LocalModels,
  type ModelInfo,
  type Settings,
  type SettingsPatch,
  type Stats,
  type ThumbQuality,
  type ImageSearchSetting,
  type ToolSetting,
  type UnrealConvert,
  humanBytes,
} from '../api'
import { prefs } from '../prefs'
import { formatMiB } from '../sizes'
import { DARK_THEMES, LIGHT_THEMES, previewScheme, themeFor, usedScheme } from '../theme'
import { confirmDialog, showError, showOk } from '../ui'
import LibraryStats from './LibraryStats.vue'
import LocationsPanel from './LocationsPanel.vue'
import MaintenancePanel from './MaintenancePanel.vue'
import IssuesPanel from './IssuesPanel.vue'
import AccountPanel from './AccountPanel.vue'
import HttpsPanel from './HttpsPanel.vue'
import TagsPanel from './TagsPanel.vue'
import ResetButton from './ResetButton.vue'
import SizeInput from './SizeInput.vue'

const props = defineProps<{ section: string; stats: Stats | null }>()
const emit = defineEmits<{
  unauthorized: []
  'update:section': [string]
  /** From the Library page: show these in the library. */
  tag: [string]
  pack: [{ id: string; name: string }]
  open: [string]
  archive: []
  /** Library locations changed: the stats need a refresh. */
  changed: []
}>()
const s = ref<Settings | null>(null)

const PAGES = [
  { id: 'appearance', label: 'Appearance', blurb: 'The themes this browser wears when it’s light and when it’s dark. Saved in this browser only.' },
  { id: 'library', label: 'Library', blurb: 'What’s in the hoard: how big it is, what it’s made of, how it grew and how well it’s tagged. Archived things aren’t counted.' },
  { id: 'tags', label: 'Tags', blurb: 'Every tag in the library. Merge ones that mean the same thing, or rename one; the taggers use the new name from then on.' },
  { id: 'locations', label: 'Library locations', blurb: 'The folders on the server that hold the library: ones Hoard keeps files in, and folders of packs it indexes where they are.' },
  { id: 'tools', label: 'Tools', blurb: 'Outside programs Hoard uses to convert and read files.' },
  { id: 'ai', label: 'AI tagging', blurb: 'An AI model names, describes and tags new packs, on top of the built-in rules. Its tags are marked as suggested.' },
  { id: 'search', label: 'Image search', blurb: 'Search pictures by what they show, not only by their names, and find things that are alike in kind, not only in colour.' },
  { id: 'unreal', label: 'Unreal packages', blurb: 'What to do with .uasset files when a pack is added.' },
  { id: 'account', label: 'Account', blurb: 'Your password, the other people who can sign in, and access tokens for scripts and other programs.' },
  { id: 'https', label: 'HTTPS', blurb: 'Serve the portal over https://, with a certificate Hoard makes or one you have. Some browser features only work over HTTPS.' },
  { id: 'server', label: 'Server', blurb: 'The server’s switches and limits, saved on the server.' },
  { id: 'issues', label: 'Issues', blurb: 'What went wrong opening or processing files: archives that wouldn’t unpack, files that couldn’t be read, jobs that failed. Try them again once the cause is fixed, or dismiss them.' },
  { id: 'maintenance', label: 'Maintenance', blurb: 'Back up, check for damage and tidy up. One of each runs at a time, and you can leave this page while they do.' },
] as const
const current = computed(() => (PAGES.some((p) => p.id === props.section) ? props.section : 'appearance'))
const page = computed(() => PAGES.find((p) => p.id === current.value)!)
const paneEl = ref<HTMLElement | null>(null)
function go(id: string) {
  emit('update:section', id)
  paneEl.value?.scrollTo({ top: 0 })
}
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
type ProviderId = 'off' | 'local' | 'chatgpt' | 'openrouter' | 'openai' | 'anthropic' | 'gemini' | 'ollama' | 'custom'
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
    id: 'chatgpt',
    label: 'ChatGPT plan',
    url: '',
    hint: 'Uses your ChatGPT Go, Plus, Pro or Business plan instead of an API key.',
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
/** The service the saved settings use. */
const providerIn = (l: Settings['llm']): ProviderId => (l.chatgpt ? 'chatgpt' : providerOf(l.url))
const providerLabel = (l: Settings['llm']) => PROVIDERS.find((p) => p.id === providerIn(l))?.label ?? l.url

const llm = reactive({ provider: 'off' as ProviderId, url: '', model: '', key: '', removeKey: false, vision: false, batch: 1 })
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
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError((e as Error).message)
    return
  }
  if (!gpt.value.signed_in || gpt.value.problem) {
    models.value = []
    modelsState.value = { kind: 'idle', text: '' }
    return
  }
  modelsState.value = { kind: 'loading', text: 'Connecting…' }
  try {
    const res = await api.chatgptModels()
    if (ask !== modelsAsk) return
    models.value = res.models
    const n = res.models.length
    modelsState.value = { kind: 'ok', text: n ? `Connected · ${n} model${n === 1 ? '' : 's'}` : 'Connected; type the model’s name' }
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
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
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
  showOk(st.email ? `Signed in to ChatGPT as ${st.email}` : 'Signed in to ChatGPT')
  void loadChatgpt()
}
async function finishChatgpt() {
  const url = gptPaste.value.trim()
  if (!url) return
  gptBusy.value = true
  try {
    signedIn(await api.chatgptFinish(url))
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError((e as Error).message)
  } finally {
    gptBusy.value = false
  }
}
async function signOutChatgpt() {
  const ok = await confirmDialog({
    title: 'Sign out of ChatGPT?',
    body: s.value?.llm.chatgpt
      ? 'AI tagging uses your ChatGPT plan, so it stops until you sign in again or pick another service.'
      : 'Hoard forgets the sign-in and tells OpenAI to end it.',
    confirm: 'Sign out',
  })
  if (!ok) return
  gptBusy.value = true
  try {
    const st = await api.chatgptSignOut()
    gpt.value = st
    models.value = []
    modelsState.value = { kind: 'idle', text: '' }
    if (st.note) showError(st.note)
    else showOk('Signed out of ChatGPT')
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
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

// Server options: each saves on its own when it's entered (Enter, or leaving the field); switches at once.
const opts = reactive({ workers: 1, max_upload_mb: 1, max_expand_mb: 1, conversion_cache_mb: 1, public_url: '', soundfont: '' })
type OptKey = keyof typeof opts
const OPT_KEYS = Object.keys(opts) as OptKey[]
const OPT_LABEL: Record<OptKey, string> = {
  workers: 'Background workers',
  max_upload_mb: 'Largest upload',
  max_expand_mb: 'Largest pack',
  conversion_cache_mb: 'Converted downloads',
  public_url: 'Public address',
  soundfont: 'MIDI SoundFont',
}
type AnyOpt = keyof Settings['options']
/** Not at the server's default, so it gets a reset arrow. */
const changed = (k: AnyOpt) => !!s.value && s.value.options[k].value !== s.value.options[k].default
const onOff = (v: boolean) => (v ? 'on' : 'off')
/** Whether model thumbnails render on a GPU, in software, or not at all. */
const gpuNote = computed(() => {
  const r = props.stats?.renderer ?? ''
  if (r.startsWith('gpu: ')) return { ok: true, text: `On the GPU: ${r.slice(5)}` }
  if (r.startsWith('cpu: ')) return { ok: false, text: `No GPU found, so in software (${r.slice(5)})` }
  if (r.startsWith('unavailable')) return { ok: false, text: 'No GPU or software renderer found, so models get no thumbnails' }
  return null
})
const ffmpegNote = computed(() => {
  const f = props.stats?.ffmpeg ?? ''
  if (f === 'downloading' || f === 'looking') return 'Fetching it for audio conversions'
  if (f.startsWith('unavailable')) return `${f[0].toUpperCase()}${f.slice(1)}; MP3 and Opus use the built-in encoders`
  return ''
})
function saveOpt(k: OptKey) {
  const cur = s.value
  if (!cur) return
  const was = cur.options[k].value
  let v = opts[k]
  if (typeof v === 'string') v = v.trim()
  else if (!Number.isInteger(v) || v < 1) {
    // Empty or nonsense: put back what's saved.
    ;(opts as Record<OptKey, unknown>)[k] = was
    return
  }
  if (v === was) return
  const shown =
    k === 'max_upload_mb' || k === 'max_expand_mb' || k === 'conversion_cache_mb'
      ? formatMiB(v as number)
      : typeof v === 'number'
        ? v.toLocaleString()
        : v || 'the default'
  void save({ [k]: v } as SettingsPatch, `${OPT_LABEL[k]}: ${shown}`)
}
const resetOpt = (k: AnyOpt) => save({ [k]: null } as SettingsPatch, 'Back to the default')

/** A tool set to something other than its default (off / automatic / a different program). */
function toolChanged(t: ToolSetting): boolean {
  const a = modeOf(t.value)
  return t.saved && (a !== modeOf(t.default) || (a === 'path' && t.value.trim() !== t.default.trim()))
}

function take(next: Settings) {
  s.value = next
  for (const k of ['blender', 'ffmpeg'] as const) {
    chosen[k] = modeOf(next[k].value)
    paths[k] = chosen[k] === 'path' ? next[k].value : ''
  }
  Object.assign(llm, {
    provider: providerIn(next.llm),
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
    // A smaller cache limit trims in the background; show what's left once it has.
    if ('conversion_cache_mb' in patch) setTimeout(loadUsage, 1500)
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError((e as Error).message)
    if (s.value) take(s.value) // put the controls back as they were
  } finally {
    busy.value = false
  }
}

const toggleClip = (on: boolean) => save({ clip: on }, on ? 'Image search: on' : 'Image search: off')

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
    void save({ llm_url: '', llm_chatgpt: false }, 'AI tagging is off')
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
  void save(patch, `AI tagging: ${provider.value.label}`)
}
const resetLlm = () =>
  save(
    { llm_url: null, llm_model: null, llm_api_key: null, llm_vision: null, llm_chatgpt: null, llm_batch: null },
    'AI tagging: back to the default',
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
  if (!d?.url) return 'off'
  return `${PROVIDERS.find((p) => p.id === providerOf(d.url))?.label ?? d.url} · ${d.model}`
})

// While a tool is being looked for or downloaded, keep its status line current.
let timer: number | undefined
function schedulePoll() {
  window.clearTimeout(timer)
  const v = s.value
  const busyTool = v && [v.blender, v.ffmpeg, v.clip].some((t) => t.state === 'looking' || t.state === 'downloading')
  const learning = v?.clip.state === 'ready' && v.clip.done < v.clip.total
  if (busyTool || learning) {
    timer = window.setTimeout(async () => {
      try {
        const next = await api.settings()
        // Don't clobber a path or AI setting someone is typing.
        if (s.value) s.value = { ...s.value, blender: next.blender, ffmpeg: next.ffmpeg, clip: next.clip }
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

function clipStatus(c: ImageSearchSetting): string {
  switch (c.state) {
    case 'off': return 'Off'
    case 'looking': return 'Looking for the model…'
    case 'downloading': return 'Downloading the model… progress is in the activity indicator'
    case 'ready':
      if (!c.total) return 'Ready. Pictures are learned as packs are added.'
      if (c.done >= c.total) return `Ready: knows all ${c.total.toLocaleString()} pictures`
      return `Ready: learning pictures, ${c.done.toLocaleString()} of ${c.total.toLocaleString()} so far`
    case 'unavailable': return `Couldn’t get the model: ${c.detail}. Trying again every hour.`
  }
}

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

const THUMBS: { value: ThumbQuality; label: string; hint: string }[] = [
  { value: 'small', label: 'Smaller', hint: 'JPEG, quality 70. Fine in the grid; a little soft up close.' },
  { value: 'balanced', label: 'Balanced', hint: 'JPEG, quality 82. Hard to tell from the original at thumbnail size.' },
  { value: 'sharp', label: 'Sharper', hint: 'JPEG, quality 92. About twice the size of Balanced.' },
  { value: 'lossless', label: 'Lossless', hint: 'PNG, exactly as made. Several times larger.' },
]
// How much the server keeps around for previews and conversions, fetched fresh when the Server page opens.
const usage = ref<Stats | null>(null)
async function loadUsage() {
  try {
    usage.value = await api.stats()
  } catch {
    usage.value = null
  }
}
watch(current, (c) => { if (c === 'server') void loadUsage() }, { immediate: true })

const UNREAL: { value: UnrealConvert; label: string; hint: string }[] = [
  { value: 'off', label: 'Leave as is', hint: 'Convert from a pack’s page when you want to.' },
  { value: 'keep', label: 'Convert, keep originals', hint: 'Textures, sounds and materials come out in common formats; the .uasset stays too.' },
  { value: 'replace', label: 'Convert, drop originals', hint: 'Saves space once the content is out. Meshes still wait for the Unreal worker.' },
]

const LOOKS = [
  { scheme: 'light' as const, label: 'Light', list: LIGHT_THEMES },
  { scheme: 'dark' as const, label: 'Dark', list: DARK_THEMES },
]
/**
 * Pick a theme and wear it at once, even when it's for the scheme not in use (see `previewScheme`). Runs on click,
 * not change: clicking the theme that's already picked (back to light while previewing dark) has to work too.
 */
function setLook(scheme: 'light' | 'dark', id: string) {
  if (scheme === 'light') prefs.lightTheme = id
  else prefs.darkTheme = id
  previewScheme.value = scheme === usedScheme() ? null : scheme
}
const endPreview = () => (previewScheme.value = null)
watch(current, (c) => c !== 'appearance' && endPreview())

onMounted(async () => {
  await load()
  if (s.value?.llm.enabled || llm.provider === 'local') void connect()
})
onBeforeUnmount(() => {
  endPreview()
  window.clearTimeout(timer)
  window.clearTimeout(localTimer)
  window.clearTimeout(gptTimer)
  localAsk++
})
</script>

<template>
  <div class="settings-page">
    <nav class="snav" aria-label="Settings">
      <h2>Settings</h2>
      <ul>
        <li v-for="p in PAGES" :key="p.id">
          <button type="button" :aria-current="current === p.id ? 'page' : undefined" @click="go(p.id)">
            <span>{{ p.label }}</span>
            <span v-if="p.id === 'ai' && s" class="pip" :data-on="s.llm.enabled" aria-hidden="true" />
          </button>
        </li>
      </ul>
    </nav>

    <div ref="paneEl" class="pane" :aria-busy="busy">
      <div class="inner">
        <header class="phead">
          <h2>{{ page.label }}</h2>
          <p class="what">{{ page.blurb }}</p>
        </header>

        <template v-if="current === 'appearance'">
          <section class="look" aria-label="Themes">
            <div v-for="g in LOOKS" :key="g.scheme" class="themes">
              <span class="tlabel">{{ g.label }}</span>
              <div class="swatches" role="radiogroup" :aria-label="`${g.label} theme`">
                <label
                  v-for="t in g.list"
                  :key="t.id"
                  class="sw"
                  :class="[g.scheme, { on: themeFor(g.scheme).id === t.id }]"
                  v-bind="{ [`data-${g.scheme}`]: t.id }"
                  :title="t.blurb"
                >
                  <input type="radio" :name="`theme-${g.scheme}`" :checked="themeFor(g.scheme).id === t.id" @click="setLook(g.scheme, t.id)" />
                  <span class="mini" aria-hidden="true">
                    <span class="bar" /><span class="ln" /><span class="ln short" /><span class="dot" />
                  </span>
                  <span class="nm">{{ t.name }}</span>
                  <small>{{ t.blurb }}</small>
                </label>
              </div>
            </div>
            <p v-if="previewScheme" class="hint preview" role="status">
              Showing the {{ previewScheme }} theme so you can see it. You're using {{ usedScheme() }}{{ prefs.theme === 'system' ? ' (following the system)' : '' }},
              and it comes back when you leave this page.
              <button type="button" class="btn ghost" @click="endPreview">Back to {{ usedScheme() }}</button>
            </p>
            <p v-else class="hint">Switch between light and dark (or follow the system) from the top bar.</p>
          </section>
        </template>

        <LibraryStats
          v-else-if="current === 'library'"
          :stats="stats"
          @tag="(t) => emit('tag', t)"
          @pack="(p) => emit('pack', p)"
          @open="(id) => emit('open', id)"
          @archive="emit('archive')"
        />

        <LocationsPanel v-else-if="current === 'locations'" @unauthorized="emit('unauthorized')" @changed="emit('changed')" />
        <MaintenancePanel v-else-if="current === 'maintenance'" @unauthorized="emit('unauthorized')" />
        <IssuesPanel
          v-else-if="current === 'issues'"
          @unauthorized="emit('unauthorized')"
          @pack="(p) => emit('pack', p)"
          @open="(id) => emit('open', id)"
          @section="go"
        />
        <AccountPanel v-else-if="current === 'account'" />
        <HttpsPanel v-else-if="current === 'https'" />
        <TagsPanel v-else-if="current === 'tags'" @tag="(t) => emit('tag', t)" @changed="emit('changed')" />

        <p v-else-if="!s" class="loading">Loading…</p>

        <div v-else class="body">
          <template v-if="current === 'tools'">
            <section v-for="t in TOOLS" :key="t.key">
              <div class="sh">
                <h3>{{ t.title }}</h3>
                <ResetButton
                  v-if="toolChanged(s[t.key])"
                  :to="defaultText(s[t.key].default)"
                  :disabled="busy"
                  @reset="save({ [t.key]: null }, `${t.title}: back to the default`)"
                />
              </div>
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
            </section>
          </template>

          <section v-else-if="current === 'search'">
            <div class="sh">
              <h3>Search by what pictures show</h3>
              <ResetButton v-if="s.clip.saved" :to="onOff(s.clip.default)" :disabled="busy" @reset="save({ clip: null }, 'Image search: back to the default')" />
            </div>
            <p class="what">
              Type what you’re after (“mossy rock”, “sci-fi door”, “wooden barrel”) and pictures that show it turn up beside
              names that have the words. “More like this” finds things of the same kind, not only the same colours.
            </p>
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
                  Turn on image search
                </label>
              </div>
            </div>
            <p class="hint">
              Uses OpenAI’s CLIP model, run on this server’s processor: nothing leaves it. Turning it on downloads the model
              ({{ Math.round(s.clip.download_bytes / 1e6) }} MB) from Hugging Face, then learns every picture in the background
              at low priority. It takes 300 to 600 MB of memory while it works or someone searches. Turning it off deletes
              the model and keeps what it learned, for next time.
            </p>
            <p class="status" :data-state="s.clip.state" role="status" data-testid="clip-status">
              <span class="dot" aria-hidden="true" />
              <span class="text">{{ clipStatus(s.clip) }}</span>
            </p>
            <progress v-if="s.clip.state === 'ready' && s.clip.total && s.clip.done < s.clip.total" class="learn" :value="s.clip.done" :max="s.clip.total" />
          </section>

          <section v-else-if="current === 'ai'">
            <div class="sh">
              <h3>Service</h3>
              <ResetButton v-if="llmChanged" :to="llmDefaultText" :disabled="busy" @reset="resetLlm" />
            </div>
            <form class="llm" @submit.prevent="saveLlm">
              <label for="llm-provider">Service</label>
              <select id="llm-provider" class="field" :value="llm.provider" :disabled="busy" @change="pickProvider(($event.target as HTMLSelectElement).value as ProviderId)">
                <option v-for="p in PROVIDERS" :key="p.id" :value="p.id">{{ p.label }}</option>
              </select>
              <span />
              <p class="hint inline">{{ provider.hint }}</p>
              <template v-if="llm.provider === 'chatgpt'">
                <span class="label">Account</span>
                <div class="gpt">
                  <p v-if="!gpt" class="status" data-state="looking" role="status">
                    <span class="dot" aria-hidden="true" />
                    <span class="text">Checking…</span>
                  </p>
                  <template v-else>
                    <p v-if="gpt.signed_in || gpt.problem" class="status" :data-state="gpt.problem ? 'unavailable' : 'ready'" role="status">
                      <span class="dot" aria-hidden="true" />
                      <span class="text">{{ gpt.problem ?? (gpt.email ? `Signed in as ${gpt.email}` : 'Signed in') }}</span>
                    </p>
                    <div class="row">
                      <button v-if="!gpt.signed_in || gpt.problem" type="button" class="btn primary" :disabled="gptBusy" @click="startChatgpt">
                        Continue with ChatGPT
                      </button>
                      <button v-if="gpt.signed_in" type="button" class="link" :disabled="gptBusy" @click="signOutChatgpt">Sign out</button>
                      <a class="link" :href="gpt.manage_url" target="_blank" rel="noopener noreferrer">Manage usage</a>
                    </div>
                    <template v-if="gptUrl && (!gpt.signed_in || gpt.problem)">
                      <p class="hint inline">
                        Finish signing in in the window that opened (<a :href="gptUrl" target="_blank" rel="noopener noreferrer">open it again</a>).
                        If it ends on a page that won’t load, at an address starting with <code>http://127.0.0.1</code>, copy that
                        address and paste it here:
                      </p>
                      <div class="key">
                        <input
                          v-model="gptPaste"
                          class="field mono"
                          type="url"
                          spellcheck="false"
                          aria-label="The address the sign-in ended on"
                          placeholder="http://127.0.0.1:…/auth/callback?code=…"
                          @keydown.enter.prevent="finishChatgpt"
                        />
                        <button type="button" class="btn" :disabled="!gptPaste.trim() || gptBusy" @click="finishChatgpt">Finish</button>
                      </div>
                    </template>
                  </template>
                  <p class="hint inline">
                    Tagging requests count toward your ChatGPT plan’s usage. Manage usage shows what Hoard has used and lets you
                    limit it. To disconnect Hoard from ChatGPT itself, go to Settings → Security and login → Sign in with ChatGPT
                    on chatgpt.com.
                  </p>
                </div>
              </template>
              <template v-if="llm.provider !== 'off' && (llm.provider !== 'chatgpt' || (gpt?.signed_in && !gpt.problem))">
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
                    <button
                      type="button"
                      class="link"
                      :disabled="(llm.provider !== 'chatgpt' && (!trimUrl(llm.url) || missingKey)) || modelsState.kind === 'loading'"
                      @click="connect"
                    >
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
                  <span class="text">{{ s.llm.enabled ? `On: ${providerLabel(s.llm)} · ${s.llm.model}` : 'Off' }}</span>
                </span>
              </div>
            </form>
          </section>

          <section v-else-if="current === 'unreal'">
            <div class="sh">
              <h3>When a pack is added</h3>
              <ResetButton
                v-if="s.unreal_convert.value !== s.unreal_convert.default"
                :to="UNREAL.find((u) => u.value === s!.unreal_convert.default)?.label ?? s.unreal_convert.default"
                :disabled="busy"
                @reset="save({ unreal_convert: null }, 'Unreal packages: back to the default')"
              />
            </div>
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
          </section>

          <template v-else-if="current === 'server'">
            <section v-if="stats" class="about">
              <div class="sh"><h3>This server</h3></div>
              <dl>
                <template v-if="stats.version"><dt>Version</dt><dd>Hoard {{ stats.version }}</dd></template>
                <dt>Files</dt><dd class="mono">{{ stats.storage }}</dd>
                <template v-if="gpuNote"><dt>3D thumbnails</dt><dd :class="{ ok: gpuNote.ok }" :title="stats.renderer">
                  {{ gpuNote.text }}
                  <small v-if="stats.renderer_hint" class="hint" data-testid="gpu-hint">{{ stats.renderer_hint }}</small>
                </dd></template>
                <template v-if="ffmpegNote"><dt>ffmpeg</dt><dd>{{ ffmpegNote }}</dd></template>
              </dl>
            </section>
            <section>
              <div class="sh"><h3>Features</h3></div>
              <div class="switches">
                <div class="opt-row">
                  <label class="check">
                    <input type="checkbox" :checked="s.options.render.value" :disabled="busy" @change="save({ render: ($event.target as HTMLInputElement).checked })" />
                    Render thumbnails for 3D models
                  </label>
                  <ResetButton v-if="changed('render')" :to="onOff(s.options.render.default)" :disabled="busy" @reset="resetOpt('render')" />
                </div>
                <div class="opt-row">
                  <label class="check">
                    <input
                      type="checkbox"
                      :checked="s.options.update_check.value"
                      :disabled="busy"
                      @change="save({ update_check: ($event.target as HTMLInputElement).checked })"
                    />
                    Check GitHub every few hours for a new release
                  </label>
                  <ResetButton v-if="changed('update_check')" :to="onOff(s.options.update_check.default)" :disabled="busy" @reset="resetOpt('update_check')" />
                </div>
                <div class="opt-row">
                  <label class="check">
                    <input
                      type="checkbox"
                      :checked="s.options.store_lookup.value"
                      :disabled="busy"
                      @change="save({ store_lookup: ($event.target as HTMLInputElement).checked })"
                    />
                    Look up store pages automatically
                  </label>
                  <ResetButton v-if="changed('store_lookup')" :to="onOff(s.options.store_lookup.default)" :disabled="busy" @reset="resetOpt('store_lookup')" />
                </div>
              </div>
            </section>

            <section>
              <div class="sh">
                <h3>Thumbnails</h3>
                <ResetButton
                  v-if="changed('thumb_quality')"
                  :to="THUMBS.find((t) => t.value === s!.options.thumb_quality.default)?.label ?? s.options.thumb_quality.default"
                  :disabled="busy"
                  @reset="resetOpt('thumb_quality')"
                />
              </div>
              <p class="what">
                How new thumbnails and model views are stored. Ones with transparency, and pixel art, are always PNG.
                To shrink the ones you already have, run Shrink thumbnails in Maintenance.
                <template v-if="usage?.preview_bytes !== undefined">Previews take {{ humanBytes(usage.preview_bytes) }} now.</template>
              </p>
              <div class="choices" role="radiogroup" aria-label="Thumbnails">
                <label v-for="t in THUMBS" :key="t.value" :class="{ on: s.options.thumb_quality.value === t.value }">
                  <input
                    type="radio"
                    name="thumbs"
                    :checked="s.options.thumb_quality.value === t.value"
                    :disabled="busy"
                    @change="save({ thumb_quality: t.value }, `Thumbnails: ${t.label}`)"
                  />
                  <span><strong>{{ t.label }}</strong><small>{{ t.hint }}</small></span>
                </label>
              </div>
            </section>

            <section>
              <div class="sh"><h3>Limits and work</h3></div>
              <p class="what">Each one saves when you leave the field or press Enter. Sizes take units: 100 GB, 512 MB, 1.5 TB; a plain number is GB.</p>
              <div class="llm opts">
                <label for="opt-workers">Background workers</label>
                <div class="key">
                  <input
                    id="opt-workers"
                    v-model.number="opts.workers"
                    class="field num"
                    type="number"
                    min="1"
                    :max="s.max_workers"
                    :disabled="busy"
                    @change="saveOpt('workers')"
                    @keydown.enter.prevent="saveOpt('workers')"
                  />
                  <span class="unit">jobs at once (ingest, tagging, conversions)</span>
                  <ResetButton v-if="changed('workers')" :to="String(s.options.workers.default)" :disabled="busy" @reset="resetOpt('workers')" />
                </div>
                <label for="opt-upload">Largest upload</label>
                <div class="key">
                  <SizeInput id="opt-upload" v-model="opts.max_upload_mb" class="num" :disabled="busy" @update:model-value="saveOpt('max_upload_mb')" />
                  <span class="unit">per file</span>
                  <ResetButton v-if="changed('max_upload_mb')" :to="formatMiB(s.options.max_upload_mb.default)" :disabled="busy" @reset="resetOpt('max_upload_mb')" />
                </div>
                <label for="opt-expand">Largest pack</label>
                <div class="key">
                  <SizeInput id="opt-expand" v-model="opts.max_expand_mb" class="num" :disabled="busy" @update:model-value="saveOpt('max_expand_mb')" />
                  <span class="unit">once unpacked</span>
                  <ResetButton v-if="changed('max_expand_mb')" :to="formatMiB(s.options.max_expand_mb.default)" :disabled="busy" @reset="resetOpt('max_expand_mb')" />
                </div>
                <label for="opt-conv">Converted downloads</label>
                <div class="key">
                  <SizeInput id="opt-conv" v-model="opts.conversion_cache_mb" class="num" :disabled="busy" @update:model-value="saveOpt('conversion_cache_mb')" />
                  <span class="unit">
                    kept at most{{ usage?.conversion_cache_bytes !== undefined ? `, ${humanBytes(usage.conversion_cache_bytes)} now` : '' }}; the least recently used go first
                  </span>
                  <ResetButton v-if="changed('conversion_cache_mb')" :to="formatMiB(s.options.conversion_cache_mb.default)" :disabled="busy" @reset="resetOpt('conversion_cache_mb')" />
                </div>
                <label for="opt-public">Public address</label>
                <div class="key">
                  <input
                    id="opt-public"
                    v-model="opts.public_url"
                    class="field mono"
                    type="url"
                    spellcheck="false"
                    placeholder="From the browser’s address (e.g. https://hoard.example.com)"
                    :disabled="busy"
                    @change="saveOpt('public_url')"
                    @keydown.enter.prevent="saveOpt('public_url')"
                  />
                  <ResetButton v-if="changed('public_url')" :to="s.options.public_url.default || 'from the browser’s address'" :disabled="busy" @reset="resetOpt('public_url')" />
                </div>
                <label for="opt-sf">MIDI SoundFont</label>
                <div class="key">
                  <input
                    id="opt-sf"
                    v-model="opts.soundfont"
                    class="field mono"
                    type="text"
                    spellcheck="false"
                    placeholder="Found on the server, or built-in"
                    :disabled="busy"
                    @change="saveOpt('soundfont')"
                    @keydown.enter.prevent="saveOpt('soundfont')"
                  />
                  <ResetButton v-if="changed('soundfont')" :to="s.options.soundfont.default || 'found on the server, or built-in'" :disabled="busy" @reset="resetOpt('soundfont')" />
                </div>
              </div>
            </section>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* A page of its own: the list of pages down the side, the open one beside it (scrolling on its own). */
.settings-page { display: grid; grid-template-columns: 220px minmax(0, 1fr); min-height: 0; }
.snav { overflow-y: auto; padding: 1.1rem 0.75rem 2rem; border-right: 1px solid var(--line); }
.snav h2 { margin: 0 0 0.7rem 0.6rem; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.snav ul { display: grid; gap: 0.1rem; margin: 0; padding: 0; list-style: none; }
.snav button {
  display: flex; align-items: center; justify-content: space-between; width: 100%; padding: 0.45rem 0.6rem;
  border: 0; border-radius: var(--r-chip); background: none; color: var(--muted); font: inherit; font-size: var(--step--1);
  text-align: left; cursor: pointer; transition: background-color var(--dur-1), color var(--dur-1);
}
.snav button:hover { color: var(--text); background: color-mix(in srgb, var(--raised) 60%, transparent); }
.snav button[aria-current='page'] { color: var(--text); background: var(--raised); font-weight: 600; }
.pip { width: 0.45rem; height: 0.45rem; border-radius: 50%; background: var(--faint); }
.pip[data-on='true'] { background: var(--ok); }
.pane { overflow-y: auto; scrollbar-gutter: stable; }
.pane[aria-busy='true'] { cursor: progress; }
.inner { max-width: 48rem; padding: 1.4rem 2rem 5rem; }
.phead { margin-bottom: 1.1rem; }
.phead h2 { margin: 0 0 0.25rem; font-size: var(--step-2); font-variation-settings: 'CASL' 0.5; }
.phead .what { margin: 0; max-width: 40rem; }
.loading { color: var(--muted); font-size: var(--step--1); }
@media (max-width: 47.99rem) {
  .settings-page { grid-template-columns: minmax(0, 1fr); grid-template-rows: auto minmax(0, 1fr); }
  .snav { display: flex; align-items: center; gap: 0.75rem; padding: 0.5rem 0.75rem; overflow-x: auto; border-right: 0; border-bottom: 1px solid var(--line); }
  .snav h2 { display: none; }
  .snav ul { display: flex; gap: 0.25rem; }
  .snav button { white-space: nowrap; min-height: 40px; gap: 0.4rem; }
  .inner { padding: 1rem 1rem 5rem; }
}

section { padding: 1rem 0 1.1rem; border-top: 1px solid var(--line); }
section:first-child { border-top: 0; padding-top: 0.2rem; }
.sh { display: flex; align-items: center; gap: 0.4rem; min-height: 1.75rem; margin-bottom: 0.3rem; }
h3 { margin: 0; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.04em; text-transform: uppercase; color: var(--faint); }
.hint.preview .btn { margin-left: 0.4rem; }
.what, .hint { margin: 0 0 0.55rem; font-size: var(--step--1); color: var(--muted); line-height: 1.45; }
.hint { margin-top: 0.45rem; }
.about dl { display: grid; grid-template-columns: 8rem minmax(0, 1fr); gap: 0.35rem 1rem; margin: 0.3rem 0 0; font-size: var(--step--1); }
.about dt { color: var(--muted); }
.about dd { margin: 0; overflow-wrap: anywhere; }
.about dd.ok { color: var(--ok); }
.about dd .hint { display: block; margin-top: 0.25rem; color: var(--muted); line-height: 1.45; }
.mono { font-family: var(--mono, ui-monospace, monospace); font-size: 0.95em; }
.opt-row { display: flex; align-items: center; gap: 0.4rem; min-height: 1.75rem; }

.modes { display: inline-flex; flex-wrap: wrap; border: 1px solid var(--line); border-radius: var(--r-chip); overflow: hidden; }
.modes label {
  display: inline-flex; align-items: center; padding: 0.4em 0.85em; font-size: var(--step--1); cursor: pointer;
  border-left: 1px solid var(--line); transition: background-color var(--dur-1), color var(--dur-1);
}
.modes label:first-child { border-left: 0; }
.modes label:has(:checked) { background: var(--text); color: var(--bg); font-weight: 600; }
.modes label:has(:focus-visible) { outline: 2px solid var(--accent); outline-offset: -2px; }
.modes input { position: absolute; opacity: 0; pointer-events: none; }

.path { display: flex; gap: 0.5rem; margin-top: 0.55rem; }
.path .field { flex: 1; min-width: 0; }

.status { display: flex; align-items: baseline; gap: 0.45rem; margin: 0.55rem 0 0; font-size: var(--step--1); color: var(--muted); }
.status .text { overflow-wrap: anywhere; }
.status .dot { flex: none; width: 0.5rem; height: 0.5rem; border-radius: 50%; background: var(--faint); translate: 0 -0.05rem; }
.status[data-state='ready'] .dot { background: var(--ok); }
.status[data-state='unavailable'] { color: var(--danger); }
.status[data-state='unavailable'] .dot { background: var(--danger); }
.status[data-state='looking'] .dot, .status[data-state='downloading'] .dot { background: var(--accent); animation: pulse 1.2s var(--ease-in-out) infinite; }
@keyframes pulse { 50% { opacity: 0.3; } }
@media (prefers-reduced-motion: reduce) { .status .dot { animation: none !important; } }

.llm { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.5rem 0.8rem; align-items: center; font-size: var(--step--1); }
.llm > label, .llm > .label { color: var(--muted); }
.gpt { display: grid; gap: 0.5rem; justify-items: start; }
.gpt .key { width: 100%; }
.llm .key { display: flex; gap: 0.6rem; align-items: center; }
.llm .key .field { flex: 1; min-width: 0; max-width: 32rem; }
.llm .check { display: inline-flex; gap: 0.45rem; align-items: center; color: var(--text); }
.llm .row { display: flex; gap: 0.9rem; align-items: center; }
.llm .row .status { margin: 0; }
.llm .status, .llm .hint.inline { margin: 0; }
/* Numbers and sizes are short: the unit sits right after them instead of across the row. */
.llm .key .num { width: 7.5rem; flex: none; }
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
.mlist progress { width: 100%; height: 0.35rem; accent-color: var(--accent); }
progress.learn { display: block; width: 100%; max-width: 28rem; height: 0.35rem; accent-color: var(--accent); }
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

/* Theme swatches: each label carries data-light / data-dark, so styles.css gives it that theme's --l-* / --d-*. */
.themes { display: grid; grid-template-columns: 3rem minmax(0, 1fr); gap: 0.8rem; align-items: start; margin-bottom: 1.4rem; }
.tlabel { font-size: var(--step--1); color: var(--muted); padding-top: 1.3rem; }
.swatches { display: grid; grid-template-columns: repeat(auto-fill, minmax(6.5rem, 1fr)); gap: 0.6rem; }
.sw small { font-size: 0.68rem; color: var(--faint); font-weight: 400; line-height: 1.3; }
.sw.light { --s-bg: var(--l-bg); --s-panel: var(--l-panel); --s-line: var(--l-line); --s-text: var(--l-text); --s-muted: var(--l-muted); --s-accent: var(--l-accent); }
.sw.dark { --s-bg: var(--d-bg); --s-panel: var(--d-panel); --s-line: var(--d-line); --s-text: var(--d-text); --s-muted: var(--d-muted); --s-accent: var(--d-accent); }
.sw { display: grid; gap: 0.25rem; cursor: pointer; font-size: 0.75rem; color: var(--muted); }
.sw input { position: absolute; opacity: 0; pointer-events: none; }
.mini {
  position: relative; display: block; height: 3.6rem; border-radius: 6px; overflow: hidden;
  background: var(--s-bg); border: 1px solid var(--line);
  transition: border-color var(--dur-1), box-shadow var(--dur-2) var(--ease-out), scale var(--dur-2) var(--ease-pop);
}
.mini .bar { position: absolute; inset: 0 0 auto; height: 0.6rem; background: var(--s-panel); border-bottom: 1px solid var(--s-line); }
.mini .ln { position: absolute; left: 0.4rem; top: 1rem; width: 55%; height: 0.25rem; border-radius: 1px; background: var(--s-text); }
.mini .ln.short { top: 1.55rem; width: 35%; background: var(--s-muted); }
.mini .dot { position: absolute; right: 0.4rem; bottom: 0.4rem; width: 0.55rem; height: 0.55rem; border-radius: 50%; background: var(--s-accent); }
.sw:hover .mini { border-color: var(--muted); }
.sw:active .mini { scale: 0.95; }
.sw.on { color: var(--text); font-weight: 600; }
.sw.on .mini { border-color: var(--text); box-shadow: 0 0 0 1px var(--text); }
.sw:has(:focus-visible) .mini { outline: 2px solid var(--accent); outline-offset: 2px; }
@media (max-width: 30rem) { .themes { grid-template-columns: minmax(0, 1fr); gap: 0.3rem; } .tlabel { padding-top: 0; } }

.link {
  padding: 0; border: 0; background: none; color: var(--accent); cursor: pointer; font-size: inherit; text-decoration: underline;
  text-underline-offset: 0.15em;
}
.link:disabled { opacity: 0.5; cursor: default; }
</style>
