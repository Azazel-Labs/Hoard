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
  type Role,
  type Settings,
  type SettingsPatch,
  type Stats,
  type ThumbQuality,
  type ImageSearchSetting,
  type ToolSetting,
  type UnrealConvert,
  humanBytes,
} from '../api'
import { may, refreshAuth } from '../auth'
import { formatNumber, LANGUAGES, languageName, preferredCatalog, t } from '../i18n'
import { prefs } from '../prefs'
import { formatMiB } from '../sizes'
import { ROLE_LABEL } from '../materials'
import { ROLES, SHORT, problem as namesProblem, stem as mapStem, type TextureNames } from '../texnames'
import { DARK_THEMES, LIGHT_THEMES, previewScheme, themeFor, usedScheme } from '../theme'
import { confirmDialog, showError, showOk } from '../ui'
import LibraryStats from './LibraryStats.vue'
import LocationsPanel from './LocationsPanel.vue'
import MaintenancePanel from './MaintenancePanel.vue'
import UpdatesPanel from './UpdatesPanel.vue'
import IssuesPanel from './IssuesPanel.vue'
import AccountPanel from './AccountPanel.vue'
import AuthPanel from './AuthPanel.vue'
import HttpsPanel from './HttpsPanel.vue'
import TagsPanel from './TagsPanel.vue'
import StoreFormatsPanel from './StoreFormatsPanel.vue'
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

const browserLanguages = navigator.languages
// Labels and blurbs are getters, so they're in the language of the moment.
const ALL_PAGES: { id: string; readonly label: string; readonly blurb: string; need?: Role }[] = [
  { id: 'general', need: 'admin', get label() { return t('settings-page-general') }, get blurb() { return t('settings-page-general-blurb') } },
  { id: 'appearance', get label() { return t('settings-page-appearance') }, get blurb() { return t('settings-page-appearance-blurb') } },
  { id: 'library', get label() { return t('settings-page-library') }, get blurb() { return t('settings-page-library-blurb') } },
  { id: 'tags', need: 'edit', get label() { return t('settings-page-tags') }, get blurb() { return t('settings-page-tags-blurb') } },
  { id: 'store-files', need: 'admin', get label() { return t('settings-page-store-files') }, get blurb() { return t('settings-page-store-files-blurb') } },
  { id: 'locations', need: 'admin', get label() { return t('settings-page-locations') }, get blurb() { return t('settings-page-locations-blurb') } },
  { id: 'tools', need: 'admin', get label() { return t('settings-page-tools') }, get blurb() { return t('settings-page-tools-blurb') } },
  { id: 'ai', need: 'admin', get label() { return t('settings-page-ai') }, get blurb() { return t('settings-page-ai-blurb') } },
  { id: 'search', need: 'admin', get label() { return t('settings-page-search') }, get blurb() { return t('settings-page-search-blurb') } },
  { id: 'adding', need: 'admin', get label() { return t('settings-page-adding') }, get blurb() { return t('settings-page-adding-blurb') } },
  { id: 'downloads', need: 'admin', get label() { return t('settings-page-downloads') }, get blurb() { return t('settings-page-downloads-blurb') } },
  { id: 'account', get label() { return t('settings-page-account') }, get blurb() { return t('settings-page-account-blurb') } },
  { id: 'auth', need: 'admin', get label() { return t('settings-page-auth') }, get blurb() { return t('settings-page-auth-blurb') } },
  { id: 'connection', need: 'admin', get label() { return t('settings-page-connection') }, get blurb() { return t('settings-page-connection-blurb') } },
  { id: 'performance', need: 'admin', get label() { return t('settings-page-performance') }, get blurb() { return t('settings-page-performance-blurb') } },
  { id: 'issues', need: 'admin', get label() { return t('settings-page-issues') }, get blurb() { return t('settings-page-issues-blurb') } },
  { id: 'maintenance', need: 'admin', get label() { return t('settings-page-maintenance') }, get blurb() { return t('settings-page-maintenance-blurb') } },
]
/** The pages this account may see: an admin sees them all. */
const PAGES = computed(() => ALL_PAGES.filter((p) => !p.need || may(p.need)))
const current = computed(() => (PAGES.value.some((p) => p.id === props.section) ? props.section : 'appearance'))
const page = computed(() => PAGES.value.find((p) => p.id === current.value)!)
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
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
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

// Server options: each saves on its own when it's entered (Enter, or leaving the field); switches at once.
const opts = reactive({ workers: 1, max_upload_mb: 1, max_expand_mb: 1, conversion_cache_mb: 1, name: '', public_url: '', soundfont: '' })
type OptKey = keyof typeof opts
const OPT_KEYS = Object.keys(opts) as OptKey[]
/** An option's name, for the note that it's saved. */
const optLabel = (k: OptKey): string =>
  ({
    workers: t('settings-workers'),
    max_upload_mb: t('settings-max-upload'),
    max_expand_mb: t('settings-max-expand'),
    conversion_cache_mb: t('settings-conversion-cache'),
    name: t('settings-name'),
    public_url: t('settings-public-url'),
    soundfont: t('settings-soundfont'),
  })[k]
type AnyOpt = keyof Settings['options']
/** Not at the server's default, so it gets a reset arrow. */
const changed = (k: AnyOpt) => !!s.value && s.value.options[k].value !== s.value.options[k].default
const onOff = (v: boolean) => (v ? t('settings-default-on') : t('settings-default-off'))
/** Whether model thumbnails render on a GPU, in software, or not at all. */
const gpuNote = computed(() => {
  const r = props.stats?.renderer ?? ''
  if (r.startsWith('gpu: ')) return { state: 'ready', text: t('settings-gpu-on', { renderer: r.slice(5) }) }
  if (r.startsWith('cpu: ')) return { state: 'off', text: t('settings-gpu-software', { renderer: r.slice(5) }) }
  if (r.startsWith('unavailable')) return { state: 'unavailable', text: t('settings-gpu-none') }
  return null
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
        ? formatNumber(v)
        : v || t('settings-the-default')
  void save({ [k]: v } as SettingsPatch, t('settings-option-saved', { name: optLabel(k), value: shown }))
}
const resetOpt = (k: AnyOpt) => save({ [k]: null } as SettingsPatch, t('settings-back-to-default'))

// Texture names: edited here, saved when a field is left (or Enter), checked first the way the server checks.
const names = reactive<TextureNames>({ on: true, pattern: '{name}_{map}', maps: {} })
const namesError = computed(() => (names.on ? namesProblem(names) : null))
const sameNames = (a: TextureNames, b: TextureNames) =>
  a.on === b.on && a.pattern === b.pattern && ROLES.every((r) => (a.maps[r] ?? '').trim() === (b.maps[r] ?? '').trim())
/** Which preset the convention is, if it's one. */
const namesPreset = computed(() => {
  const d = s.value?.texture_names.default
  if (d && sameNames({ ...names, on: true }, { ...d, on: true })) return 'descriptive'
  if (sameNames({ ...names, on: true }, { ...SHORT, on: true })) return 'short'
  return 'custom'
})
const namesExample = computed(() =>
  [
    `${mapStem(names, 'Stand Wood Old 05', 'base_color')}.jpg`,
    `${mapStem(names, 'Stand Wood Old 05', 'normal')}.jpg`,
    `${mapStem(names, 'Stand Wood Old 05', 'roughness')}.jpg`,
    `${mapStem(names, 'Rock Moss', 'base_color', '4K')}.png`,
  ],
)
function saveNames() {
  const cur = s.value
  if (!cur || namesError.value) return
  if (sameNames(names, cur.texture_names)) return
  void save({ texture_names: { on: names.on, pattern: names.pattern.trim(), maps: { ...names.maps } } }, t('settings-tn-saved'))
}
function useNames(preset: 'descriptive' | 'short') {
  const d = s.value?.texture_names.default
  if (!d) return
  Object.assign(names, preset === 'short' ? { pattern: SHORT.pattern, maps: { ...SHORT.maps } } : { pattern: d.pattern, maps: { ...d.maps } })
  saveNames()
}

/** A tool set to something other than its default (off / automatic / a different program). */
function toolChanged(tool: ToolSetting): boolean {
  const a = modeOf(tool.value)
  return tool.saved && (a !== modeOf(tool.default) || (a === 'path' && tool.value.trim() !== tool.default.trim()))
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
  Object.assign(names, { on: next.texture_names.on, pattern: next.texture_names.pattern, maps: { ...next.texture_names.maps } })
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
    showError(t('settings-load-failed', { error: (e as Error).message }))
  }
}

async function save(patch: SettingsPatch, done?: string) {
  busy.value = true
  try {
    take(await api.saveSettings(patch))
    if (done) showOk(done)
    // A smaller cache limit trims in the background; show what's left once it has.
    if ('conversion_cache_mb' in patch) setTimeout(loadUsage, 1500)
    // The top bar, the tab and the sign-in page show the name.
    if ('name' in patch) void refreshAuth()
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) emit('unauthorized')
    showError((e as Error).message)
    if (s.value) take(s.value) // put the controls back as they were
  } finally {
    busy.value = false
  }
}

const toggleClip = (on: boolean) => save({ clip: on }, on ? t('settings-clip-saved-on') : t('settings-clip-saved-off'))

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

// What each tool does and how it's found are getters, so they're in the language of the moment.
const TOOLS: { key: ToolKey; title: string; readonly what: string; readonly download: string }[] = [
  {
    key: 'blender',
    title: 'Blender',
    get what() { return t('settings-blender-what') },
    get download() { return t('settings-blender-download') },
  },
  {
    key: 'ffmpeg',
    title: 'ffmpeg',
    get what() { return t('settings-ffmpeg-what') },
    get download() { return t('settings-ffmpeg-download') },
  },
]

function clipStatus(c: ImageSearchSetting): string {
  switch (c.state) {
    case 'off': return t('settings-status-off')
    case 'looking': return t('settings-clip-looking')
    case 'downloading': return t('settings-clip-downloading')
    case 'ready':
      if (!c.total) return t('settings-clip-ready-empty')
      if (c.done >= c.total) return t('settings-clip-ready-all', { count: c.total })
      return t('settings-clip-learning', { done: c.done, total: c.total })
    case 'unavailable': return t('settings-clip-unavailable', { detail: c.detail })
  }
}

function statusText(tool: ToolSetting): string {
  switch (tool.state) {
    case 'off': return t('settings-status-off')
    case 'looking': return t('settings-tool-looking')
    case 'downloading': return t('settings-tool-downloading')
    case 'ready': return t('settings-tool-ready', { detail: tool.detail })
    case 'unavailable': return tool.detail
  }
}
const defaultText = (v: string) => (modeOf(v) === 'off' ? t('settings-default-off') : modeOf(v) === 'auto' ? t('settings-default-auto') : v)

// Labels and hints are getters, so they're in the language of the moment.
const THUMBS: { value: ThumbQuality; readonly label: string; readonly hint: string }[] = [
  { value: 'small', get label() { return t('settings-thumbs-small') }, get hint() { return t('settings-thumbs-small-hint') } },
  { value: 'balanced', get label() { return t('settings-thumbs-balanced') }, get hint() { return t('settings-thumbs-balanced-hint') } },
  { value: 'sharp', get label() { return t('settings-thumbs-sharp') }, get hint() { return t('settings-thumbs-sharp-hint') } },
  { value: 'lossless', get label() { return t('settings-thumbs-lossless') }, get hint() { return t('settings-thumbs-lossless-hint') } },
]
// How much the server keeps around for previews and conversions, fetched fresh when the Performance page opens.
const usage = ref<Stats | null>(null)
async function loadUsage() {
  try {
    usage.value = await api.stats()
  } catch {
    usage.value = null
  }
}
watch(current, (c) => { if (c === 'performance') void loadUsage() }, { immediate: true })

const UNREAL: { value: UnrealConvert; readonly label: string; readonly hint: string }[] = [
  { value: 'off', get label() { return t('settings-unreal-off') }, get hint() { return t('settings-unreal-off-hint') } },
  { value: 'keep', get label() { return t('settings-unreal-keep') }, get hint() { return t('settings-unreal-keep-hint') } },
  { value: 'replace', get label() { return t('settings-unreal-replace') }, get hint() { return t('settings-unreal-replace-hint') } },
]

const LOOKS = [
  { scheme: 'light' as const, get label() { return t('settings-look-light') }, list: LIGHT_THEMES },
  { scheme: 'dark' as const, get label() { return t('settings-look-dark') }, list: DARK_THEMES },
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
  if (!may('admin')) return
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
    <nav class="snav" :aria-label="t('settings-title')">
      <h2>{{ t('settings-title') }}</h2>
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
      <div class="inner" :class="{ wide: current === 'store-files' }">
        <header class="phead">
          <h2>{{ page.label }}</h2>
          <p class="what">{{ page.blurb }}</p>
        </header>

        <template v-if="current === 'appearance'">
          <section class="look" :aria-label="t('settings-themes')">
            <div class="themes">
              <label class="tlabel lang" for="language">{{ t('appearance-language') }}</label>
              <select id="language" v-model="prefs.language" class="field lang">
                <option value="auto">{{ t('appearance-language-auto', { name: languageName(preferredCatalog(browserLanguages, LANGUAGES) ?? 'en') }) }}</option>
                <option v-for="code in LANGUAGES" :key="code" :value="code" :lang="code">{{ languageName(code) }}</option>
              </select>
            </div>
            <div v-for="g in LOOKS" :key="g.scheme" class="themes">
              <span class="tlabel">{{ g.label }}</span>
              <div class="swatches" role="radiogroup" :aria-label="t('settings-look-group', { scheme: g.scheme })">
                <label
                  v-for="theme in g.list"
                  :key="theme.id"
                  class="sw"
                  :class="[g.scheme, { on: themeFor(g.scheme).id === theme.id }]"
                  v-bind="{ [`data-${g.scheme}`]: theme.id }"
                  :title="theme.blurb"
                >
                  <input type="radio" :name="`theme-${g.scheme}`" :checked="themeFor(g.scheme).id === theme.id" @click="setLook(g.scheme, theme.id)" />
                  <span class="mini" aria-hidden="true">
                    <span class="bar" /><span class="ln" /><span class="ln short" /><span class="dot" />
                  </span>
                  <span class="nm">{{ theme.name }}</span>
                  <small>{{ theme.blurb }}</small>
                </label>
              </div>
            </div>
            <p v-if="previewScheme" class="hint preview" role="status">
              {{ t('settings-preview', { preview: previewScheme, used: usedScheme(), system: prefs.theme === 'system' ? 'yes' : 'no' }) }}
              <button type="button" class="btn ghost" @click="endPreview">{{ t('settings-preview-back', { used: usedScheme() }) }}</button>
            </p>
            <p v-else class="hint">{{ t('settings-look-hint') }}</p>
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
        <IssuesPanel
          v-else-if="current === 'issues'"
          @unauthorized="emit('unauthorized')"
          @pack="(p) => emit('pack', p)"
          @open="(id) => emit('open', id)"
          @section="go"
        />
        <AccountPanel v-else-if="current === 'account'" />
        <AuthPanel v-else-if="current === 'auth'" />
        <TagsPanel v-else-if="current === 'tags'" @tag="(t) => emit('tag', t)" @changed="emit('changed')" />
        <StoreFormatsPanel v-else-if="current === 'store-files'" />

        <p v-else-if="!s" class="loading">{{ t('settings-loading') }}</p>

        <div v-else class="body">
          <template v-if="current === 'tools'">
            <section v-for="tool in TOOLS" :key="tool.key">
              <div class="sh">
                <h3>{{ tool.title }}</h3>
                <ResetButton
                  v-if="toolChanged(s[tool.key])"
                  :to="defaultText(s[tool.key].default)"
                  :disabled="busy"
                  @reset="save({ [tool.key]: null }, t('settings-tool-reset', { tool: tool.title }))"
                />
              </div>
              <p class="what">{{ tool.what }}</p>
              <div class="modes" role="radiogroup" :aria-label="tool.title">
                <label>
                  <input type="radio" :name="tool.key" :checked="chosen[tool.key] === 'off'" :disabled="busy" @change="pick(tool.key, 'off')" />
                  {{ t('settings-mode-off') }}
                </label>
                <label>
                  <input type="radio" :name="tool.key" :checked="chosen[tool.key] === 'auto'" :disabled="busy" @change="pick(tool.key, 'auto')" />
                  {{ t('settings-mode-auto') }}
                </label>
                <label>
                  <input type="radio" :name="tool.key" :checked="chosen[tool.key] === 'path'" :disabled="busy" @change="pick(tool.key, 'path')" />
                  {{ t('settings-mode-path') }}
                </label>
              </div>
              <p v-if="chosen[tool.key] === 'auto'" class="hint">
                {{ s[tool.key].can_download ? tool.download : t('settings-tool-no-download', { tool: tool.title }) }}
              </p>
              <form v-if="chosen[tool.key] === 'path'" class="path" @submit.prevent="usePath(tool.key)">
                <input
                  v-model="paths[tool.key]"
                  class="field mono"
                  type="text"
                  spellcheck="false"
                  :aria-label="t('settings-tool-path', { tool: tool.title })"
                  :placeholder="tool.key === 'blender' ? '/opt/blender/blender' : '/usr/bin/ffmpeg'"
                />
                <button class="btn" type="submit" :disabled="busy || !paths[tool.key].trim() || paths[tool.key].trim() === s[tool.key].value">{{ t('settings-use') }}</button>
              </form>
              <p class="status" :data-state="s[tool.key].state" role="status">
                <span class="dot" aria-hidden="true" />
                <span class="text">{{ statusText(s[tool.key]) }}</span>
              </p>
            </section>
            <section>
              <div class="sh"><h3>{{ t('settings-soundfont') }}</h3></div>
              <p class="what">{{ t('settings-soundfont-what') }}</p>
              <div class="llm opts">
                <label for="opt-sf">{{ t('settings-soundfont-file') }}</label>
                <div class="key">
                  <input
                    id="opt-sf"
                    v-model="opts.soundfont"
                    class="field mono"
                    type="text"
                    spellcheck="false"
                    :placeholder="t('settings-soundfont-placeholder')"
                    :disabled="busy"
                    @change="saveOpt('soundfont')"
                    @keydown.enter.prevent="saveOpt('soundfont')"
                  />
                  <ResetButton v-if="changed('soundfont')" :to="s.options.soundfont.default || t('settings-soundfont-default')" :disabled="busy" @reset="resetOpt('soundfont')" />
                </div>
              </div>
            </section>
          </template>

          <section v-else-if="current === 'search'">
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
            <p class="hint">{{ t('settings-clip-hint', { size: Math.round(s.clip.download_bytes / 1e6) }) }}</p>
            <p class="status" :data-state="s.clip.state" role="status" data-testid="clip-status">
              <span class="dot" aria-hidden="true" />
              <span class="text">{{ clipStatus(s.clip) }}</span>
            </p>
            <progress v-if="s.clip.state === 'ready' && s.clip.total && s.clip.done < s.clip.total" class="learn" :value="s.clip.done" :max="s.clip.total" />
          </section>

          <section v-else-if="current === 'ai'">
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

          <template v-else-if="current === 'adding'">
            <section>
              <div class="sh"><h3>{{ t('settings-store-pages') }}</h3></div>
              <p class="what">{{ t('settings-store-pages-what') }}</p>
              <div class="switches">
                <div class="opt-row">
                  <label class="check">
                    <input
                      type="checkbox"
                      :checked="s.options.store_lookup.value"
                      :disabled="busy"
                      @change="save({ store_lookup: ($event.target as HTMLInputElement).checked })"
                    />
                    {{ t('settings-store-lookup') }}
                  </label>
                  <ResetButton v-if="changed('store_lookup')" :to="onOff(s.options.store_lookup.default)" :disabled="busy" @reset="resetOpt('store_lookup')" />
                </div>
              </div>
            </section>
            <section>
              <div class="sh">
                <h3>{{ t('settings-unreal') }}</h3>
              <ResetButton
                v-if="s.unreal_convert.value !== s.unreal_convert.default"
                :to="UNREAL.find((u) => u.value === s!.unreal_convert.default)?.label ?? s.unreal_convert.default"
                :disabled="busy"
                @reset="save({ unreal_convert: null }, t('settings-unreal-reset'))"
              />
              </div>
              <p class="what">{{ t('settings-unreal-what') }}</p>
            <div class="choices" role="radiogroup" :aria-label="t('settings-unreal')">
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
          </template>

          <template v-else-if="current === 'downloads'">
            <section data-testid="texture-names">
              <div class="sh">
                <h3>{{ t('settings-tn') }}</h3>
                <ResetButton
                  v-if="s.texture_names.saved && !sameNames(s.texture_names, s.texture_names.default)"
                  :to="t('settings-the-default')"
                  :disabled="busy"
                  @reset="save({ texture_names: null }, t('settings-tn-reset'))"
                />
              </div>
              <p class="what">{{ t('settings-tn-what') }}</p>
              <div class="switches">
                <div class="opt-row">
                  <label class="check">
                    <input
                      type="checkbox"
                      :checked="names.on"
                      :disabled="busy"
                      @change="names.on = ($event.target as HTMLInputElement).checked; saveNames()"
                    />
                    {{ t('settings-tn-toggle') }}
                  </label>
                </div>
              </div>
              <template v-if="names.on">
                <div class="llm opts">
                  <span class="label">{{ t('settings-tn-style') }}</span>
                  <div class="modes" role="radiogroup" :aria-label="t('settings-tn-style-group')">
                    <label><input type="radio" name="tn-style" :checked="namesPreset === 'descriptive'" :disabled="busy" @change="useNames('descriptive')" />{{ t('settings-tn-descriptive') }}</label>
                    <label><input type="radio" name="tn-style" :checked="namesPreset === 'short'" :disabled="busy" @change="useNames('short')" />{{ t('settings-tn-short') }}</label>
                    <label><input type="radio" name="tn-style" :checked="namesPreset === 'custom'" disabled />{{ t('settings-tn-custom') }}</label>
                  </div>
                  <label for="tn-pattern">{{ t('settings-tn-pattern') }}</label>
                  <div class="key">
                    <input
                      id="tn-pattern"
                      v-model="names.pattern"
                      class="field mono"
                      spellcheck="false"
                      :disabled="busy"
                      @change="saveNames"
                      @keydown.enter.prevent="saveNames"
                    />
                  </div>
                  <span />
                  <p class="hint inline">
                    <code>{name}</code>{{ t('settings-tn-hint-name') }}<code>{map}</code>{{ t('settings-tn-hint-map') }}<code>{size}</code>{{ t('settings-tn-hint-size') }}
                  </p>
                  <span class="label">{{ t('settings-tn-example') }}</span>
                  <p class="tn-example mono" data-testid="texture-example">
                    <span v-for="e in namesExample" :key="e">{{ e }}</span>
                  </p>
                </div>
                <p v-if="namesError" class="status" data-state="unavailable" role="alert">
                  <span class="dot" aria-hidden="true" /><span class="text">{{ namesError }}</span>
                </p>
                <details class="tn-maps">
                  <summary>{{ t('settings-tn-suffixes') }}</summary>
                  <div class="llm opts">
                    <template v-for="r in ROLES" :key="r">
                      <label :for="`tn-${r}`">{{ ROLE_LABEL[r] }}</label>
                      <div class="key">
                        <input
                          :id="`tn-${r}`"
                          v-model="names.maps[r]"
                          class="field mono short"
                          spellcheck="false"
                          :disabled="busy"
                          @change="saveNames"
                          @keydown.enter.prevent="saveNames"
                        />
                      </div>
                    </template>
                  </div>
                </details>
              </template>
            </section>
          </template>

          <template v-else-if="current === 'general'">
            <section>
              <div class="sh"><h3>{{ t('settings-name') }}</h3></div>
              <p class="what">{{ t('settings-name-what') }}</p>
              <div class="llm opts">
                <label for="opt-name">{{ t('settings-name') }}</label>
                <div class="key">
                  <input
                    id="opt-name"
                    v-model="opts.name"
                    class="field"
                    maxlength="40"
                    :placeholder="s.options.name.default"
                    :disabled="busy"
                    @change="saveOpt('name')"
                    @keydown.enter.prevent="saveOpt('name')"
                  />
                  <ResetButton v-if="changed('name')" :to="s.options.name.default" :disabled="busy" @reset="resetOpt('name')" />
                </div>
              </div>
            </section>
          </template>

          <template v-else-if="current === 'connection'">
            <section>
              <div class="sh"><h3>{{ t('settings-address') }}</h3></div>
              <p class="what">{{ t('settings-address-what') }}</p>
              <div class="llm opts">
                <label for="opt-public">{{ t('settings-public-url') }}</label>
                <div class="key">
                  <input
                    id="opt-public"
                    v-model="opts.public_url"
                    class="field mono"
                    type="url"
                    spellcheck="false"
                    :placeholder="t('settings-public-url-placeholder')"
                    :disabled="busy"
                    @change="saveOpt('public_url')"
                    @keydown.enter.prevent="saveOpt('public_url')"
                  />
                  <ResetButton v-if="changed('public_url')" :to="s.options.public_url.default || t('settings-public-url-default')" :disabled="busy" @reset="resetOpt('public_url')" />
                </div>
              </div>
            </section>
            <HttpsPanel />
          </template>

          <template v-else-if="current === 'performance'">
            <section>
              <div class="sh"><h3>{{ t('settings-render') }}</h3></div>
              <p v-if="gpuNote" class="status" :data-state="gpuNote.state" role="status" :title="stats?.renderer">
                <span class="dot" aria-hidden="true" />
                <span class="text">{{ gpuNote.text }}</span>
              </p>
              <p v-if="stats?.renderer_hint" class="hint" data-testid="gpu-hint">{{ stats.renderer_hint }}</p>
              <div class="switches">
                <div class="opt-row">
                  <label class="check">
                    <input type="checkbox" :checked="s.options.render.value" :disabled="busy" @change="save({ render: ($event.target as HTMLInputElement).checked })" />
                    {{ t('settings-render-toggle') }}
                  </label>
                  <ResetButton v-if="changed('render')" :to="onOff(s.options.render.default)" :disabled="busy" @reset="resetOpt('render')" />
                </div>
              </div>
            </section>

            <section>
              <div class="sh">
                <h3>{{ t('settings-thumbs') }}</h3>
                <ResetButton
                  v-if="changed('thumb_quality')"
                  :to="THUMBS.find((t) => t.value === s!.options.thumb_quality.default)?.label ?? s.options.thumb_quality.default"
                  :disabled="busy"
                  @reset="resetOpt('thumb_quality')"
                />
              </div>
              <p class="what">
                {{ t('settings-thumbs-what') }}
                <template v-if="usage?.preview_bytes !== undefined">{{ t('settings-previews-size', { size: humanBytes(usage.preview_bytes) }) }}</template>
              </p>
              <div class="choices" role="radiogroup" :aria-label="t('settings-thumbs')">
                <label v-for="q in THUMBS" :key="q.value" :class="{ on: s.options.thumb_quality.value === q.value }">
                  <input
                    type="radio"
                    name="thumbs"
                    :checked="s.options.thumb_quality.value === q.value"
                    :disabled="busy"
                    @change="save({ thumb_quality: q.value }, t('settings-thumbs-saved', { quality: q.label }))"
                  />
                  <span><strong>{{ q.label }}</strong><small>{{ q.hint }}</small></span>
                </label>
              </div>
            </section>

            <section>
              <div class="sh"><h3>{{ t('settings-limits') }}</h3></div>
              <p class="what">{{ t('settings-limits-what') }}</p>
              <div class="llm opts">
                <label for="opt-workers">{{ t('settings-workers') }}</label>
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
                  <span class="unit">{{ t('settings-workers-unit') }}</span>
                  <ResetButton v-if="changed('workers')" :to="formatNumber(s.options.workers.default)" :disabled="busy" @reset="resetOpt('workers')" />
                </div>
                <label for="opt-upload">{{ t('settings-max-upload') }}</label>
                <div class="key">
                  <SizeInput id="opt-upload" v-model="opts.max_upload_mb" class="num" :disabled="busy" @update:model-value="saveOpt('max_upload_mb')" />
                  <span class="unit">{{ t('settings-max-upload-unit') }}</span>
                  <ResetButton v-if="changed('max_upload_mb')" :to="formatMiB(s.options.max_upload_mb.default)" :disabled="busy" @reset="resetOpt('max_upload_mb')" />
                </div>
                <label for="opt-expand">{{ t('settings-max-expand') }}</label>
                <div class="key">
                  <SizeInput id="opt-expand" v-model="opts.max_expand_mb" class="num" :disabled="busy" @update:model-value="saveOpt('max_expand_mb')" />
                  <span class="unit">{{ t('settings-max-expand-unit') }}</span>
                  <ResetButton v-if="changed('max_expand_mb')" :to="formatMiB(s.options.max_expand_mb.default)" :disabled="busy" @reset="resetOpt('max_expand_mb')" />
                </div>
                <label for="opt-conv">{{ t('settings-conversion-cache') }}</label>
                <div class="key">
                  <SizeInput id="opt-conv" v-model="opts.conversion_cache_mb" class="num" :disabled="busy" @update:model-value="saveOpt('conversion_cache_mb')" />
                  <span class="unit">
                    {{ usage?.conversion_cache_bytes !== undefined ? t('settings-conversion-cache-unit-now', { size: humanBytes(usage.conversion_cache_bytes) }) : t('settings-conversion-cache-unit') }}
                  </span>
                  <ResetButton v-if="changed('conversion_cache_mb')" :to="formatMiB(s.options.conversion_cache_mb.default)" :disabled="busy" @reset="resetOpt('conversion_cache_mb')" />
                </div>
              </div>
            </section>
          </template>

          <template v-else-if="current === 'maintenance'">
            <section>
              <div class="sh"><h3>{{ t('settings-updates') }}</h3></div>
              <p v-if="stats?.version" class="what">{{ t('settings-version', { version: stats.version }) }}</p>
              <div class="switches">
                <div class="opt-row">
                  <label class="check">
                    <input
                      type="checkbox"
                      :checked="s.options.update_check.value"
                      :disabled="busy"
                      @change="save({ update_check: ($event.target as HTMLInputElement).checked })"
                    />
                    {{ t('settings-update-check') }}
                  </label>
                  <ResetButton v-if="changed('update_check')" :to="onOff(s.options.update_check.default)" :disabled="busy" @reset="resetOpt('update_check')" />
                </div>
              </div>
              <UpdatesPanel :settings="s" @saved="take" @unauthorized="emit('unauthorized')" />
            </section>
            <MaintenancePanel @unauthorized="emit('unauthorized')" />
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
/* The format wizard shows a file beside its form. */
.inner.wide { max-width: 80rem; }
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

.tn-example { display: grid; gap: 0.1rem; margin: 0; color: var(--text); overflow-wrap: anywhere; }
.tn-maps { margin-top: 0.8rem; font-size: var(--step--1); }
.tn-maps summary { cursor: pointer; color: var(--muted); margin-bottom: 0.6rem; }
.llm .key .field.short { max-width: 10rem; }

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
.themes { display: grid; grid-template-columns: 6rem minmax(0, 1fr); gap: 0.8rem; align-items: start; margin-bottom: 1.4rem; }
.tlabel { font-size: var(--step--1); color: var(--muted); padding-top: 1.3rem; }
.tlabel.lang { padding-top: 0; align-self: center; }
.field.lang { justify-self: start; min-width: 14rem; }
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
