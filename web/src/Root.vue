<script setup lang="ts">
// Asks the server who's signed in before anything else: the first account, the sign-in screen, or the library.
import { onMounted } from 'vue'
import App from './App.vue'
import SignIn from './components/SignIn.vue'
import { auth, canBrowse, refreshAuth } from './auth'

onMounted(refreshAuth)

async function signedIn() {
  auth.prompt = false
  await refreshAuth()
}
</script>

<template>
  <div v-if="!auth.status && auth.error" class="unreachable" role="alert">
    <p>Couldn’t reach the Hoard server: {{ auth.error }}</p>
    <button type="button" class="btn" @click="refreshAuth">Try again</button>
  </div>
  <SignIn v-else-if="auth.status?.setup_needed" mode="setup" @done="signedIn" />
  <template v-else-if="auth.status">
    <App v-if="canBrowse()" />
    <SignIn v-if="!canBrowse()" mode="login" @done="signedIn" />
    <SignIn v-else-if="auth.prompt && !auth.status.signed_in" mode="login" overlay @done="signedIn" @close="auth.prompt = false" />
  </template>
</template>

<style scoped>
.unreachable { display: grid; gap: 0.6rem; justify-items: start; max-width: 32rem; margin: 15vh auto; padding: 0 1rem; color: var(--danger); }
.unreachable p { margin: 0; }
</style>
