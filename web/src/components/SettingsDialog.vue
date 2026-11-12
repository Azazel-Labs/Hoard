<script setup lang="ts">
// Settings: turn Blender and ffmpeg on or off (downloading them if need be), point the AI tagger at an endpoint,
// and choose what happens to Unreal packages. Saved on the server; a saved setting wins over its HOARD_* variable.
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import { ApiError, api, type Settings, type SettingsPatch, type ToolSetting, type UnrealConvert } from '../api'
import { showError, showOk } from '../ui'

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

const llm = reactive({ url: '', model: '', key: '', removeKey: false, vision: false })
const llmDirty = computed(() => {
  const l = s.value?.llm
  return !!l && (llm.url !== l.url || llm.model !== l.model || llm.key !== '' || llm.removeKey || llm.vision !== l.vision)
})

function take(next: Settings) {
  s.value = next
  for (const k of ['blender', 'ffmpeg'] as const) {
    chosen[k] = modeOf(next[k].value)
    paths[k] = chosen[k] === 'path' ? next[k].value : ''
  }
  Object.assign(llm, { url: next.llm.url, model: next.llm.model, key: '', removeKey: false, vision: next.llm.vision })
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
  const patch: SettingsPatch = { llm_url: llm.url.trim(), llm_model: llm.model.trim(), llm_vision: llm.vision }
  if (llm.key) patch.llm_api_key = llm.key
  else if (llm.removeKey) patch.llm_api_key = ''
  void save(patch, patch.llm_url ? 'AI tagging settings saved' : 'AI tagging is off')
}
const resetLlm = () => save({ llm_url: null, llm_model: null, llm_api_key: null, llm_vision: null })

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

onMounted(() => {
  dlg.value?.showModal()
  void load()
})
onBeforeUnmount(() => window.clearTimeout(timer))
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
        <p class="what">
          Any OpenAI-compatible endpoint (OpenAI, Ollama, LM Studio, vLLM, …) adds tags and names to new packs. Leave the address
          empty to tag with the built-in rules only.
        </p>
        <form class="llm" @submit.prevent="saveLlm">
          <label for="llm-url">Address</label>
          <input id="llm-url" v-model="llm.url" class="field mono" type="url" spellcheck="false" placeholder="http://localhost:11434/v1" />
          <label for="llm-model">Model</label>
          <input id="llm-model" v-model="llm.model" class="field mono" type="text" spellcheck="false" placeholder="qwen2.5vl:7b" />
          <label for="llm-key">API key</label>
          <div class="key">
            <input
              id="llm-key"
              v-model="llm.key"
              class="field mono"
              type="password"
              autocomplete="off"
              :placeholder="s.llm.has_key && !llm.removeKey ? 'Saved (type to replace)' : 'None'"
              @input="llm.removeKey = false"
            />
            <button v-if="s.llm.has_key && !llm.key && !llm.removeKey" type="button" class="link" @click="llm.removeKey = true">Remove</button>
          </div>
          <span />
          <label class="check"><input v-model="llm.vision" type="checkbox" /> Send thumbnails too (needs a vision model)</label>
          <span />
          <div class="row">
            <button class="btn primary" type="submit" :disabled="busy || !llmDirty">Save</button>
            <span class="status" :data-state="s.llm.enabled ? 'ready' : 'off'">
              <span class="dot" aria-hidden="true" /><span class="text">{{ s.llm.enabled ? 'On' : 'Off' }}</span>
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
