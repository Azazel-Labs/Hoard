<script setup lang="ts">
// Connect GitHub, for downloading Unreal Editor from Epic's private images: sign in with GitHub by its device flow
// (Hoard shows a code, GitHub asks for it, and Hoard is handed a token that can only read packages), or type in a token.
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
import { ApiError, api, type GitHubCode } from '../api'
import { t } from '../i18n'
import { copyText, showError, showOk } from '../ui'
import { useSettings } from './settingsContext'

const emit = defineEmits<{ close: [] }>()
const { s, busy, save, take, unauthorized } = useSettings()
const dlg = ref<HTMLDialogElement | null>(null)

/** Whether there's a GitHub app to sign in through; without one, only a token. */
const device = computed(() => !!s.value.unreal_github?.device)
const byToken = ref(!device.value)

/** `loading` getting a code; `ready` showing it; `waiting` for it to be entered at GitHub; then how that ended. */
type Step = 'loading' | 'ready' | 'waiting' | 'expired' | 'denied' | 'failed'
const step = ref<Step>('loading')
const code = ref<GitHubCode | null>(null)
const copied = ref(false)
let timer: ReturnType<typeof setTimeout> | undefined

async function getCode() {
  clearTimeout(timer)
  step.value = 'loading'
  code.value = null
  try {
    code.value = await api.githubStart()
    step.value = 'ready'
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
    step.value = 'failed'
  }
}

/** The sign-in button is a link to GitHub, so it opens in a new tab however the browser feels about pop-ups. */
function signIn() {
  if (!code.value) return
  void copy()
  if (step.value !== 'waiting') {
    step.value = 'waiting'
    timer = setTimeout(poll, code.value.interval * 1000)
  }
}

async function poll() {
  try {
    const r = await api.githubPoll()
    if (r.state === 'waiting') {
      timer = setTimeout(poll, (code.value?.interval ?? 5) * 1000)
      return
    }
    if (r.state === 'done') {
      take(r.settings)
      showOk(t('settings-unreal-github-connected', { user: r.user }))
      dlg.value?.close()
      return
    }
    step.value = r.state
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
    step.value = 'failed'
  }
}

async function copy() {
  if (code.value && (await copyText(code.value.code))) {
    copied.value = true
    setTimeout(() => (copied.value = false), 2000)
  }
}

/** A token typed in, as before there was signing in. */
const token = reactive({ user: s.value.unreal_github?.user ?? '', value: '' })
async function saveToken() {
  const user = token.user.trim()
  const value = token.value.trim()
  await save({ ghcr_user: user || null, ...(value ? { ghcr_token: value } : {}) }, t('settings-unreal-github-saved'))
  dlg.value?.close()
}

function toToken() {
  clearTimeout(timer)
  void api.githubCancel().catch(() => {})
  byToken.value = true
}
function toSignIn() {
  byToken.value = false
  void getCode()
}

onMounted(() => {
  dlg.value?.showModal()
  if (!byToken.value) void getCode()
})
onUnmounted(() => {
  clearTimeout(timer)
  if (step.value === 'waiting' || step.value === 'ready') void api.githubCancel().catch(() => {})
})
</script>

<template>
  <dialog ref="dlg" class="connect" aria-labelledby="gh-title" @close="emit('close')">
    <header>
      <div>
        <h2 id="gh-title">{{ t('settings-unreal-github-connect-title') }}</h2>
        <p class="sub">{{ t('settings-unreal-github-connect-why') }}</p>
      </div>
      <button type="button" class="btn ghost x" :aria-label="t('window-close')" @click="dlg?.close()">
        <svg viewBox="0 0 16 16" width="13" height="13" aria-hidden="true"><path d="m4 4 8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
      </button>
    </header>

    <template v-if="!byToken">
      <a
        class="gh"
        :class="{ off: !code }"
        :href="code?.url ?? 'https://github.com/login/device'"
        target="_blank"
        rel="noopener noreferrer"
        :aria-disabled="!code"
        data-testid="github-sign-in"
        @click="code ? signIn() : $event.preventDefault()"
      >
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M9 19c-4.3 1.4-4.3-2.5-6-3m12 5v-3.5c0-1 .1-1.4-.5-2 2.8-.3 5.5-1.4 5.5-6a4.6 4.6 0 0 0-1.3-3.2 4.2 4.2 0 0 0-.1-3.2s-1.1-.3-3.5 1.3a12.3 12.3 0 0 0-6.2 0C6.5 2.8 5.4 3.1 5.4 3.1a4.2 4.2 0 0 0-.1 3.2A4.6 4.6 0 0 0 4 9.5c0 4.6 2.7 5.7 5.5 6-.6.6-.6 1.2-.5 2V21" />
        </svg>
        {{ t('settings-unreal-github-sign-in') }}
      </a>

      <div class="code" aria-live="polite">
        <template v-if="step === 'expired' || step === 'denied' || step === 'failed'">
          <span class="note">{{ step === 'expired' ? t('settings-unreal-github-expired') : step === 'denied' ? t('settings-unreal-github-denied') : t('settings-unreal-github-failed') }}</span>
          <button type="button" class="btn" @click="getCode">{{ t('settings-unreal-github-again') }}</button>
        </template>
        <template v-else>
          <span class="note">{{ t('settings-unreal-github-enter-code') }}</span>
          <div class="row">
            <span class="value" data-testid="github-code">{{ code?.code ?? '····-····' }}</span>
            <button type="button" class="btn ghost" :disabled="!code" @click="copy">{{ copied ? t('text-copied') : t('text-copy') }}</button>
          </div>
          <span v-if="step === 'waiting'" class="wait"><span class="dot" aria-hidden="true" />{{ t('settings-unreal-github-waiting') }}</span>
        </template>
      </div>
    </template>

    <form v-else class="creds" data-testid="unreal-github" @submit.prevent="saveToken">
      <label for="ghcr-user">{{ t('settings-unreal-github-user') }}</label>
      <input id="ghcr-user" v-model="token.user" class="field mono" type="text" autocomplete="username" spellcheck="false" :disabled="busy" />
      <label for="ghcr-token">{{ t('settings-unreal-github-token') }}</label>
      <input
        id="ghcr-token"
        v-model="token.value"
        class="field mono"
        type="password"
        autocomplete="new-password"
        spellcheck="false"
        :placeholder="s.unreal_github?.has_token ? t('settings-unreal-github-token-kept') : 'ghp_…'"
        :disabled="busy"
      />
      <p class="hint">
        {{ t('settings-unreal-github-step-token') }}
        <a href="https://github.com/settings/tokens/new?scopes=read:packages&amp;description=Hoard%20(Unreal%20Editor)" target="_blank" rel="noopener noreferrer">{{ t('settings-unreal-github-step-token-go') }}</a>
      </p>
      <div class="buttons">
        <button class="btn" type="submit" :disabled="busy || !token.user.trim() || (!token.value.trim() && !s.unreal_github?.has_token)">{{ t('settings-unreal-github-save') }}</button>
      </div>
    </form>

    <div class="before">
      <strong>{{ t('settings-unreal-github-before') }}</strong>
      <span>
        {{ t('settings-unreal-github-before-what') }}
        <a href="https://www.unrealengine.com/en-US/ue-on-github" target="_blank" rel="noopener noreferrer">{{ t('settings-unreal-github-step-link-go') }}</a>
      </span>
    </div>

    <footer>
      <button v-if="device && !byToken" type="button" class="link" @click="toToken">{{ t('settings-unreal-github-use-token') }}</button>
      <button v-else-if="device" type="button" class="link" @click="toSignIn">{{ t('settings-unreal-github-use-sign-in') }}</button>
      <span v-else />
      <span v-if="!byToken" class="quiet">{{ t('settings-unreal-github-no-password') }}</span>
    </footer>
  </dialog>
</template>

<style scoped>
.connect {
  width: min(32rem, calc(100vw - 2rem)); padding: 1.5rem; border: 1px solid var(--line); border-radius: 12px;
  background: var(--panel); color: var(--text); box-shadow: var(--elev-3);
}
.connect[open] { display: flex; flex-direction: column; gap: 1.1rem; animation: pop var(--dur-3) var(--ease-pop); }
.connect::backdrop { background: var(--scrim); animation: fade var(--dur-2) var(--ease-out); }
@keyframes pop { from { opacity: 0; scale: 0.96; } }
@keyframes fade { from { opacity: 0; } }
header { display: flex; justify-content: space-between; align-items: flex-start; gap: 1rem; }
h2 { margin: 0 0 0.35rem; font-size: var(--step-1); font-variation-settings: 'CASL' 0.5; }
.sub { margin: 0; font-size: var(--step--1); line-height: 1.5; color: var(--muted); }
.x { flex: none; padding: 0.35em 0.45em; }

/* GitHub's own look for its button: light on dark, dark on light. */
.gh {
  display: flex; align-items: center; justify-content: center; gap: 0.6rem; min-height: 3.4rem; border-radius: 8px;
  background: var(--text); color: var(--panel); font-size: var(--step-0); font-weight: 700; text-decoration: none;
  transition: opacity var(--dur-1) var(--ease-out);
}
.gh:hover { color: var(--panel); opacity: 0.9; }
.gh:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
.gh.off { opacity: 0.55; cursor: progress; }

.code {
  display: flex; flex-direction: column; align-items: center; gap: 0.6rem; padding: 1.1rem;
  border: 1px solid var(--line); border-radius: 8px; background: var(--raised); text-align: center;
}
.note, .wait { font-size: var(--step--1); color: var(--muted); }
.row { display: flex; flex-wrap: wrap; justify-content: center; align-items: center; gap: 0.6rem; }
.value { font-size: clamp(1.5rem, 8vw, 2rem); white-space: nowrap; font-weight: 700; letter-spacing: 0.12em; font-variation-settings: 'MONO' 1; user-select: all; }
.wait { display: flex; align-items: center; gap: 0.5rem; }
.wait .dot { width: 8px; height: 8px; border-radius: 50%; background: var(--accent); animation: breathe 1.4s ease-in-out infinite; }
@keyframes breathe { 50% { opacity: 0.35; } }
@media (prefers-reduced-motion: reduce) { .wait .dot { animation: none; } }

.creds { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.5rem 0.7rem; align-items: center; }
.creds .hint { grid-column: 1 / -1; margin: 0; font-size: var(--step--1); color: var(--muted); }
.creds .buttons { grid-column: 1 / -1; display: flex; justify-content: flex-end; }
@media (max-width: 480px) { .creds { grid-template-columns: minmax(0, 1fr); } }

.before { display: flex; flex-direction: column; gap: 0.4rem; font-size: var(--step--1); color: var(--muted); line-height: 1.5; }
.before strong { color: var(--text); font-weight: 600; }
footer { display: flex; justify-content: space-between; align-items: center; gap: 1rem; font-size: var(--step--1); }
.link { padding: 0; text-align: start; border: 0; background: none; color: var(--accent); font: inherit; cursor: pointer; }
.link:hover { text-decoration: underline; }
.quiet { color: var(--faint); text-align: end; }
.hint a, .before a { color: var(--accent); }
</style>
