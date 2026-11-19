<script setup lang="ts">
// The ChatGPT account AI tagging can use: whether the server is signed in to it, and signing in (in another window,
// or by pasting the address that window ends on) or out. The AI panel lists the plan's models once it's signed in.
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { ApiError, api, type ChatGptStatus } from '../api'
import { t } from '../i18n'
import { confirmDialog, showError, showOk } from '../ui'
import { useSettings } from './settingsContext'

const emit = defineEmits<{ status: [ChatGptStatus] }>()
const { s, unauthorized } = useSettings()

const gpt = ref<ChatGptStatus | null>(null)
const gptBusy = ref(false)
/** The sign-in address, once "Continue with ChatGPT" was pressed (in case the window didn't open). */
const gptUrl = ref('')
const gptPaste = ref('')
let gptTimer: number | undefined
let gptUntil = 0
let closed = false
async function load() {
  try {
    const st = await api.chatgpt()
    if (!closed) set(st)
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
  }
}
function set(st: ChatGptStatus) {
  gpt.value = st
  emit('status', st)
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
      set(st)
    } catch {
      // Try again on the next tick.
    }
    pollChatgpt()
  }, 2000)
}
function signedIn(st: ChatGptStatus) {
  window.clearTimeout(gptTimer)
  set(st)
  gptUrl.value = ''
  gptPaste.value = ''
  showOk(st.email ? t('settings-chatgpt-signed-in-as', { email: st.email }) : t('settings-chatgpt-signed-in'))
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
    set(st)
    if (st.note) showError(st.note)
    else showOk(t('settings-chatgpt-signed-out'))
  } catch (e) {
    if (e instanceof ApiError && e.status === 401) unauthorized()
    showError((e as Error).message)
  } finally {
    gptBusy.value = false
  }
}

onMounted(() => void load())
onBeforeUnmount(() => {
  closed = true
  window.clearTimeout(gptTimer)
})
</script>

<template>
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

<style scoped src="./settingsForm.css"></style>

<style scoped>
.gpt { display: grid; gap: 0.5rem; justify-items: start; }
.gpt .key { width: 100%; }
</style>
