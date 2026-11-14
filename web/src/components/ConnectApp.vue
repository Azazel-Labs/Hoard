<script setup lang="ts">
// `/oauth/authorize`: an app (Claude, another MCP client) wants to sign in to this Hoard. The server sends the browser
// here with the app's request in the query string; allowing hands the browser back to the app with a one-time code.
import { onMounted, ref } from 'vue'
import { api } from '../api'
import { auth, siteName } from '../auth'

const params = Object.fromEntries(new URLSearchParams(location.search))
const info = ref<{ client: string; redirect_host: string } | null>(null)
const error = ref('')
const busy = ref(false)

onMounted(async () => {
  try {
    info.value = await api.oauthRequest(location.search)
  } catch (e) {
    error.value = (e as Error).message
  }
})

async function answer(allow: boolean) {
  busy.value = true
  try {
    const { redirect } = await api.oauthApprove(params, allow)
    location.assign(redirect)
  } catch (e) {
    error.value = (e as Error).message
    busy.value = false
  }
}
</script>

<template>
  <div class="connect">
    <div class="card" role="dialog" aria-labelledby="connect-title">
      <p class="brand">{{ siteName() }}</p>
      <template v-if="info">
        <h1 id="connect-title">Connect {{ info.client }}?</h1>
        <p class="lede">
          <strong>{{ info.client }}</strong> will be able to search, download, describe and tag everything in this Hoard,
          acting as <strong>{{ auth.status?.user?.name }}</strong>. It can’t manage accounts or access tokens.
        </p>
        <p class="lede">
          You’ll go back to <span class="mono">{{ info.redirect_host }}</span>. Disconnect it any time in Settings → Account.
        </p>
        <div class="actions">
          <button type="button" class="btn primary" :disabled="busy" @click="answer(true)">Allow</button>
          <button type="button" class="btn" :disabled="busy" @click="answer(false)">Cancel</button>
        </div>
      </template>
      <template v-else-if="error">
        <h1 id="connect-title">Can’t connect this app</h1>
        <p class="err" role="alert">{{ error }}</p>
        <a class="btn" href="/">Go to the library</a>
      </template>
      <p v-else id="connect-title" class="lede">Checking…</p>
      <p v-if="info && error" class="err" role="alert">{{ error }}</p>
    </div>
  </div>
</template>

<style scoped>
.connect { position: fixed; inset: 0; display: grid; place-items: start center; overflow-y: auto; padding: clamp(1rem, 12vh, 7rem) 1rem 2rem; background: var(--bg); }
.card { width: min(28rem, 100%); display: grid; gap: 0.5rem; justify-items: start; padding: 1.6rem 1.8rem; border: 1px solid var(--line); border-radius: 10px; background: var(--panel); box-shadow: var(--elev-2); }
.brand { margin: 0 0 0.4rem; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.08em; text-transform: uppercase; color: var(--faint); }
h1 { margin: 0; font-size: var(--step-2); line-height: 1.2; overflow-wrap: anywhere; }
.lede { margin: 0; color: var(--muted); font-size: var(--step--1); line-height: 1.5; }
.lede strong, .lede .mono { color: var(--text); overflow-wrap: anywhere; }
.actions { display: flex; gap: 0.5rem; margin-top: 0.8rem; }
.err { margin: 0; color: var(--danger); font-size: var(--step--1); line-height: 1.5; }
@media (max-width: 30rem) { .card { padding: 1.1rem 1rem 1.2rem; } }
</style>
