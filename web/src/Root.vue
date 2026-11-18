<script setup lang="ts">
// Asks the server who's signed in before anything else: the first account, the sign-in screen, or the library.
import { onMounted } from 'vue'
import App from './App.vue'
import SignIn from './components/SignIn.vue'
import WindowControls from './components/WindowControls.vue'
import { auth, canBrowse, isDesktop, refreshAuth } from './auth'
import { lazy } from './lazy'
import { t } from './i18n'

const ConnectApp = lazy(() => import('./components/ConnectApp.vue'), () => t('root-lazy-connect'))

/** Inside the desktop app's window, which has no frame of the system's (any other browser on its server has one). */
const framed = '__TAURI_INTERNALS__' in window

/** An app signing in over OAuth (see `oauth.rs`): sign in if need be, then allow or cancel. */
const connecting = location.pathname === '/oauth/authorize'

onMounted(refreshAuth)

async function signedIn() {
  auth.prompt = false
  await refreshAuth()
}
</script>

<template>
  <WindowControls v-if="framed" />
  <div v-if="!auth.status && auth.error" class="unreachable" role="alert">
    <p>{{ t('root-unreachable', { error: auth.error }) }}</p>
    <button type="button" class="btn" @click="refreshAuth">{{ t('root-try-again') }}</button>
  </div>
  <p v-else-if="isDesktop() && !auth.status?.signed_in" class="reopen" role="alert">{{ t('root-desktop-reopen') }}</p>
  <SignIn v-else-if="auth.status?.setup_needed" mode="setup" @done="signedIn" />
  <template v-else-if="auth.status && connecting">
    <ConnectApp v-if="auth.status.signed_in" />
    <SignIn v-else mode="login" @done="signedIn" />
  </template>
  <template v-else-if="auth.status">
    <App v-if="canBrowse()" />
    <SignIn v-if="!canBrowse()" mode="login" @done="signedIn" />
    <SignIn v-else-if="auth.prompt && !auth.status.signed_in" mode="login" overlay @done="signedIn" @close="auth.prompt = false" />
  </template>
</template>

<style scoped>
.unreachable { display: grid; gap: 0.6rem; justify-items: start; max-width: 32rem; margin: 15vh auto; padding: 0 1rem; color: var(--danger); }
.unreachable p { margin: 0; }
.reopen { max-width: 32rem; margin: 15vh auto; padding: 0 1rem; color: var(--muted); }
</style>
