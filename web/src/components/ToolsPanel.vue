<script setup lang="ts">
// Tools: Blender and ffmpeg (off, found or downloaded, or a program of your own), and the SoundFont MIDI plays with.
import { reactive, ref } from 'vue'
import { ApiError, type ToolSetting, api } from '../api'
import { t } from '../i18n'
import { showError, showOk } from '../ui'
import ResetButton from './ResetButton.vue'
import { useSettings } from './settingsContext'

const { s, busy, save, take, onTake, opts, saveOpt, resetOpt, changed, unauthorized } = useSettings()

/** A SoundFont chosen on this computer, sent to the server, which keeps it and plays MIDI with it. */
const fontInput = ref<HTMLInputElement | null>(null)
async function sendFont() {
  const file = fontInput.value?.files?.[0]
  if (fontInput.value) fontInput.value.value = ''
  if (!file) return
  busy.value = true
  try {
    take(await api.uploadSoundFont(file))
    showOk(t('settings-soundfont-sent', { name: file.name }))
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
  } finally {
    busy.value = false
  }
}

type ToolKey = 'blender' | 'ffmpeg' | 'unreal'
type Mode = 'off' | 'auto' | 'path'
const modeOf = (v: string): Mode => {
  const t = v.trim().toLowerCase()
  if (['off', 'false', '0', 'none', 'no'].includes(t)) return 'off'
  if (t === '' || t === 'auto') return 'auto'
  return 'path'
}
/** The path being typed for each tool, before it's applied. */
const paths = reactive<Record<ToolKey, string>>({ blender: '', ffmpeg: '', unreal: '' })
const chosen = reactive<Record<ToolKey, Mode>>({ blender: 'off', ffmpeg: 'auto', unreal: 'off' })

/** A tool set to something other than its default (off / automatic / a different program). */
function toolChanged(tool: ToolSetting): boolean {
  const a = modeOf(tool.value)
  return tool.saved && (a !== modeOf(tool.default) || (a === 'path' && tool.value.trim() !== tool.default.trim()))
}

const unrealImage = ref('5.8')
/** The GitHub account Unreal Editor is downloaded as; the token is typed afresh, never shown. */
const github = reactive({ user: '', token: '' })
onTake((next) => {
  unrealImage.value = next.unreal_image.value
  github.user = next.unreal_github?.user ?? ''
  github.token = ''
  for (const k of ['blender', 'ffmpeg', 'unreal'] as const) {
    chosen[k] = k !== 'ffmpeg' && !next[k].can_download && modeOf(next[k].value) === 'auto' ? 'path' : modeOf(next[k].value)
    paths[k] = modeOf(next[k].value) === 'path' ? next[k].value : ''
  }
})

function pick(k: ToolKey, m: Mode) {
  chosen[k] = m
  if (m === 'off') void save({ [k]: 'off' })
  else if (m === 'auto') void save({ [k]: 'auto', ...(k === 'unreal' ? { unreal_image: unrealImage.value.trim() } : {}) })
  // A path waits for "Use".
}
function saveGithub() {
  const user = github.user.trim()
  const token = github.token.trim()
  void save({ ghcr_user: user || null, ...(token ? { ghcr_token: token } : {}) }, t('settings-unreal-github-saved'))
}
function forgetGithub() {
  void save({ ghcr_user: null, ghcr_token: null }, t('settings-unreal-github-forgotten'))
}

function usePath(k: ToolKey) {
  const p = paths[k].trim()
  if (p) void save({ [k]: p })
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
    key: 'unreal',
    title: 'Unreal Editor',
    get what() { return t('settings-unreal-editor-what') },
    get download() { return t('settings-unreal-download') },
  },
  {
    key: 'ffmpeg',
    title: 'ffmpeg',
    get what() { return t('settings-ffmpeg-what') },
    get download() { return t('settings-ffmpeg-download') },
  },
]

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
</script>

<template>
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
    <p class="what">{{ tool.key === 'unreal' && !s.unreal.can_download ? t('settings-editor-path', { tool: tool.title }) : tool.what }}</p>
    <p v-if="tool.key === 'blender' && !s.blender.can_download" class="hint">{{ t('settings-editor-path', { tool: tool.title }) }}</p>
    <form v-if="tool.key === 'unreal' && s.unreal.can_download" class="path" @submit.prevent="save({ unreal_image: unrealImage.trim() })">
      <label for="unreal-image">{{ t('settings-unreal-image') }}</label>
      <input id="unreal-image" v-model="unrealImage" class="field mono" type="text" spellcheck="false" placeholder="5.8" :disabled="busy" />
      <button class="btn" type="submit" :disabled="busy || !unrealImage.trim() || unrealImage.trim() === s.unreal_image.value">{{ t('settings-use') }}</button>
      <ResetButton v-if="s.unreal_image.saved" :to="s.unreal_image.default" :disabled="busy" @reset="save({ unreal_image: null })" />
    </form>
    <details v-if="tool.key === 'unreal' && s.unreal.can_download" class="github" :open="chosen.unreal === 'auto' && !s.unreal_github?.has_token && !s.unreal_github?.from_server">
      <summary>{{ t('settings-unreal-github') }}</summary>
      <p class="hint">{{ t('settings-unreal-github-why') }}</p>
      <ol class="steps">
        <li>
          {{ t('settings-unreal-github-step-link') }}
          <a href="https://www.unrealengine.com/en-US/ue-on-github" target="_blank" rel="noopener noreferrer">{{ t('settings-unreal-github-step-link-go') }}</a>
        </li>
        <li>{{ t('settings-unreal-github-step-invite') }}</li>
        <li>
          {{ t('settings-unreal-github-step-token') }}
          <a href="https://github.com/settings/tokens/new?scopes=read:packages&amp;description=Hoard%20(Unreal%20Editor)" target="_blank" rel="noopener noreferrer">{{ t('settings-unreal-github-step-token-go') }}</a>
        </li>
        <li>{{ t('settings-unreal-github-step-enter') }}</li>
      </ol>
      <form class="creds" data-testid="unreal-github" @submit.prevent="saveGithub">
        <label for="ghcr-user">{{ t('settings-unreal-github-user') }}</label>
        <input id="ghcr-user" v-model="github.user" class="field mono" type="text" autocomplete="username" spellcheck="false" :disabled="busy" />
        <label for="ghcr-token">{{ t('settings-unreal-github-token') }}</label>
        <input
          id="ghcr-token"
          v-model="github.token"
          class="field mono"
          type="password"
          autocomplete="new-password"
          spellcheck="false"
          :placeholder="s.unreal_github?.has_token ? t('settings-unreal-github-token-kept') : 'ghp_…'"
          :disabled="busy"
        />
        <div class="buttons">
          <button class="btn" type="submit" :disabled="busy || !github.user.trim() || (!github.token.trim() && !s.unreal_github?.has_token)">{{ t('settings-unreal-github-save') }}</button>
          <button v-if="s.unreal_github?.has_token || s.unreal_github?.user" class="btn ghost" type="button" :disabled="busy" @click="forgetGithub">{{ t('settings-unreal-github-forget') }}</button>
        </div>
      </form>
      <p v-if="s.unreal_github?.from_server && !s.unreal_github?.has_token" class="hint">{{ t('settings-unreal-github-from-server') }}</p>
    </details>
    <div class="modes" role="radiogroup" :aria-label="tool.title">
      <label>
        <input type="radio" :name="tool.key" :checked="chosen[tool.key] === 'off'" :disabled="busy" @change="pick(tool.key, 'off')" />
        {{ t('settings-mode-off') }}
      </label>
      <label v-if="tool.key === 'ffmpeg' || s[tool.key].can_download">
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
        :placeholder="tool.key === 'unreal' ? '/opt/UnrealEngine/Engine/Binaries/Linux/UnrealEditor' : tool.key === 'blender' ? '/opt/blender/blender' : '/usr/bin/ffmpeg'"
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
        <button class="btn" type="button" :disabled="busy" data-testid="soundfont-send" @click="fontInput?.click()">
          {{ t('settings-soundfont-send') }}
        </button>
        <input ref="fontInput" type="file" accept=".sf2" hidden data-testid="soundfont-file" @change="sendFont" />
      </div>
    </div>
  </section>
</template>

<style scoped src="./settingsForm.css"></style>

<style scoped>
.path { display: flex; gap: 0.5rem; margin-top: 0.7rem; }
.path .field { flex: 1; min-width: 0; }
.github { margin-top: 0.7rem; }
.github summary { cursor: pointer; font-weight: 600; }
.github .steps { margin: 0.5rem 0; padding-left: 1.3rem; display: grid; gap: 0.3rem; }
.github .creds { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.45rem 0.6rem; align-items: center; margin-top: 0.5rem; }
.github .creds .buttons { grid-column: 1 / -1; display: flex; gap: 0.5rem; justify-content: flex-end; }
@media (max-width: 520px) {
  .github .creds { grid-template-columns: minmax(0, 1fr); }
}
</style>
