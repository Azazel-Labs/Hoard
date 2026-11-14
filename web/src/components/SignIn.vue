<script setup lang="ts">
// Signing in, or making the server's first account. Full-page when there's nothing to show without an account; a
// dialog over the library when someone browsing as a guest needs to sign in to change something.
import { computed, nextTick, onMounted, ref } from 'vue'
import { api, ApiError } from '../api'
import { auth, siteName } from '../auth'

const props = defineProps<{ mode: 'setup' | 'login'; overlay?: boolean }>()
const emit = defineEmits<{ done: []; close: [] }>()

const username = ref('')
const password = ref('')
const again = ref('')
const busy = ref(false)
/** Signing in with a provider came back with a reason it didn't work: say it once, and take it out of the address. */
const turnedAway = new URLSearchParams(location.search).get('signin_error')
if (turnedAway !== null) {
  const u = new URL(location.href)
  u.searchParams.delete('signin_error')
  history.replaceState(history.state, '', u)
}
const error = ref(turnedAway ?? '')
const nameEl = ref<HTMLInputElement | null>(null)

const setup = computed(() => props.mode === 'setup')
/** Other ways to sign in, from Settings → Authentication. */
const others = computed(() => (setup.value ? null : (auth.status?.sign_in ?? null)))
// Password managers (1Password, Bitwarden, browsers) tell a sign-up form from a sign-in one by its field names, ids
// and button words as much as by `autocomplete`, so the account-making form uses sign-up words throughout: that's
// what makes them offer a new strong password instead of filling a saved one.
const f = computed(() => (setup.value ? 'signup' : 'signin'))
const mismatch = computed(() => setup.value && again.value !== '' && again.value !== password.value)
const ready = computed(
  () => username.value.trim() !== '' && password.value !== '' && (!setup.value || (password.value.length >= 8 && again.value === password.value)),
)

onMounted(() => nextTick(() => nameEl.value?.focus()))

async function submit() {
  if (!ready.value || busy.value) return
  busy.value = true
  error.value = ''
  try {
    if (setup.value) await api.setup(username.value.trim(), password.value)
    else await api.login(username.value.trim(), password.value)
    emit('done')
  } catch (e) {
    error.value = e instanceof ApiError || e instanceof Error ? e.message : String(e)
    password.value = setup.value ? password.value : ''
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <div class="signin" :class="{ overlay }" role="dialog" aria-modal="true" :aria-labelledby="`${f}-title`" @keydown.esc="overlay && emit('close')">
    <form :id="`${f}-form`" :name="f" class="card" method="post" :action="setup ? '/api/auth/setup' : '/api/auth/login'" @submit.prevent="submit">
      <p class="brand">{{ siteName() }}</p>
      <h1 :id="`${f}-title`">{{ setup ? 'Create your account' : 'Sign in' }}</h1>
      <p class="lede">
        <template v-if="setup">
          This server doesn’t have an account yet. The one you make here can do everything; add accounts for other people from Settings → Account
          later.
        </template>
        <template v-else-if="overlay">Browsing is open to everyone here, but changes need an account.</template>
      </p>

      <template v-if="others?.oauth">
        <a class="btn primary oauth" href="/api/auth/oauth/start">Sign in with {{ others.oauth }}</a>
        <p class="or"><span>or with a name and password</span></p>
      </template>

      <label :for="`${f}-username`">{{ setup ? 'Username' : 'Name' }}</label>
      <input
        :id="`${f}-username`"
        ref="nameEl"
        name="username"
        v-model="username"
        class="field"
        autocomplete="username"
        autocapitalize="none"
        spellcheck="false"
        maxlength="64"
        required
      />
      <label :for="setup ? 'new-password' : 'current-password'">{{ setup ? 'New password' : 'Password' }}</label>
      <input
        :id="setup ? 'new-password' : 'current-password'"
        :name="setup ? 'new-password' : 'password'"
        v-model="password"
        class="field"
        type="password"
        :autocomplete="setup ? 'new-password' : 'current-password'"
        :minlength="setup ? 8 : undefined"
        required
        :aria-describedby="setup ? 'signup-rule' : undefined"
      />
      <template v-if="setup">
        <small id="signup-rule" class="rule">At least 8 characters.</small>
        <label for="confirm-password">Confirm password</label>
        <input
          id="confirm-password"
          name="confirm-password"
          v-model="again"
          class="field"
          type="password"
          autocomplete="new-password"
          required
          :aria-invalid="mismatch"
          :aria-describedby="mismatch ? 'signup-mismatch' : undefined"
        />
        <small v-if="mismatch" id="signup-mismatch" class="rule bad">The two passwords don’t match.</small>
      </template>

      <p v-if="error" class="err" role="alert">{{ error }}</p>

      <div class="actions">
        <button type="submit" class="btn primary" :disabled="!ready || busy">
          {{ busy ? (setup ? 'Creating…' : 'Signing in…') : setup ? 'Create account' : 'Sign in' }}
        </button>
        <button v-if="overlay" type="button" class="btn ghost" @click="emit('close')">Keep browsing</button>
      </div>

      <p v-if="others?.directory" class="hint">Use your {{ others.directory }} name and password.</p>
      <p v-if="!setup" class="hint">
        Forgot a Hoard password? On the server, run <span class="mono">hoardd --reset-password &lt;name&gt;</span> (in Docker,
        <span class="mono">docker compose exec hoard hoardd --reset-password &lt;name&gt;</span>) for a new one.
      </p>
    </form>
  </div>
</template>

<style scoped>
.signin {
  position: fixed; inset: 0; z-index: 60; display: grid; place-items: start center; overflow-y: auto; padding: clamp(1rem, 12vh, 7rem) 1rem 2rem;
  background: var(--bg);
}
.signin.overlay { background: color-mix(in srgb, var(--bg) 70%, transparent); backdrop-filter: blur(3px); }
.card {
  width: min(26rem, 100%); display: grid; gap: 0.35rem; padding: 1.6rem 1.8rem 1.6rem; border: 1px solid var(--line); border-radius: 10px;
  background: var(--panel); box-shadow: var(--elev-2); animation: rise var(--dur-3) var(--ease-out);
}
@keyframes rise { from { opacity: 0; translate: 0 8px; } }
@media (prefers-reduced-motion: reduce) { .card { animation: none; } }
.brand { margin: 0 0 0.6rem; font-size: 0.7rem; font-weight: 650; letter-spacing: 0.08em; text-transform: uppercase; color: var(--faint); }
h1 { margin: 0; font-size: var(--step-2); font-variation-settings: 'CASL' 0.5; line-height: 1.2; }
.lede { margin: 0.2rem 0 0.8rem; color: var(--muted); font-size: var(--step--1); line-height: 1.5; }
.lede:empty { display: none; }
label { margin-top: 0.45rem; font-size: var(--step--1); font-weight: 550; }
.field { width: 100%; }
.rule { color: var(--muted); font-size: var(--step--1); }
.rule.bad, .err { color: var(--danger); }
.err { margin: 0.5rem 0 0; font-size: var(--step--1); }
.actions { display: flex; gap: 0.5rem; margin-top: 1rem; }
.hint { margin: 1rem 0 0; color: var(--muted); font-size: var(--step--1); line-height: 1.5; }
.oauth { justify-content: center; text-align: center; text-decoration: none; padding-block: 0.55rem; }
.or { display: flex; align-items: center; gap: 0.6rem; margin: 0.9rem 0 0.1rem; color: var(--faint); font-size: var(--step--1); }
.or::before, .or::after { content: ''; flex: 1; border-top: 1px solid var(--line); }
.hint .mono { overflow-wrap: anywhere; color: var(--text); }
@media (max-width: 30rem) { .card { padding: 1.1rem 1rem 1.2rem; } }
</style>
